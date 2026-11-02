//! Sparse model of the engine's linear memory and CPU state.
//!
//! The port reserves 3 GiB (`0xC000_0000`) for wasm64 linear memory and carries
//! 4 077 passive data segments. Allocating that for every test run would be
//! wasteful, so this model keeps a sparse `BTreeMap<page_index, 64 KiB page>`.
//! Absent pages read as zero and are not hashed; a page becomes present the
//! first time the workload touches it. Because both runs start from the same
//! page set, page presence itself is reproducible and is mixed into the digest.
//!
//! ## Excluded regions
//!
//! Two pages are deliberately excluded from the memory digest:
//!
//! * stack — `0x2000_0000 .. 0x2001_0000`
//! * TLS   — `0x2001_0000 .. 0x2002_0000`
//!
//! The server contract says simulation state never lives there; they hold host
//! bookkeeping (canaries, thread id, per-process salt). [`Vm::boot`] writes a
//! process-specific nonce into both, and the `excluded_regions_*` test proves
//! that the digest ignores that difference while the raw bytes differ.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use engine_runtime::hash::StateHasher;

use crate::harness::futex::FutexTable;

/// WebAssembly page size.
pub const PAGE_SIZE: usize = 65_536;
/// Reserved linear-memory size: 3 GiB.
pub const RESERVED_BYTES: u64 = 3 * 1024 * 1024 * 1024;
/// Number of wasm pages in the reservation (49_152).
pub const RESERVED_PAGES: u32 = (RESERVED_BYTES / PAGE_SIZE as u64) as u32;

/// Base of the initialised data image.
pub const DATA_BASE: u32 = 0x1000_0000;
/// Base of the simulated engine stack page (excluded).
pub const STACK_BASE: u32 = 0x2000_0000;
/// End of the simulated engine stack page (exclusive).
pub const STACK_END: u32 = STACK_BASE + PAGE_SIZE as u32;
/// Base of the simulated TLS page (excluded).
pub const TLS_BASE: u32 = 0x2001_0000;
/// End of the simulated TLS page (exclusive).
pub const TLS_END: u32 = TLS_BASE + PAGE_SIZE as u32;
/// Base of the futex word table.
pub const FUTEX_BASE: u32 = 0x3000_0000;
/// Base of the scratch/output region.
pub const SCRATCH_BASE: u32 = 0x4000_0000;

/// Passive data segments carried by the recovered module.
pub const SEGMENT_COUNT: usize = 4_077;
/// Bytes per modelled passive segment.
pub const SEGMENT_BYTES: usize = 32;

/// Magic at the start of the data image (`"SENG"` little-endian).
pub const IMAGE_MAGIC: u32 = 0x474E_4553;
/// Layout revision; bumping it changes every memory digest by design.
pub const LAYOUT_VERSION: u32 = 1;

/// True when `[start, end)` overlaps a documented excluded region.
pub fn excluded(start: u64, end: u64) -> bool {
    let ranges = [
        (STACK_BASE as u64, STACK_END as u64),
        (TLS_BASE as u64, TLS_END as u64),
    ];
    ranges.iter().any(|&(a, b)| start < b && end > a)
}

/// Per-run salt written only to the excluded pages.
static BOOT_COUNTER: AtomicU64 = AtomicU64::new(0);

/// A value that differs between boots and processes. Used solely for the
/// excluded stack/TLS pages, never for simulation state.
pub fn process_nonce() -> u64 {
    let counter = BOOT_COUNTER.fetch_add(0x9E37_79B9_7F4A_7C15, Ordering::Relaxed);
    let wall = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    let pid = std::process::id() as u64;
    let mut x = counter ^ wall.rotate_left(23) ^ pid.rotate_left(41);
    // splitmix64 finalizer
    x ^= x >> 30;
    x = x.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

/// Digest of one run's machine state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateDigest {
    /// Page-granular hash of every included linear-memory page.
    pub memory: u64,
    /// General-purpose + SIMD registers, pc, frame and tick.
    pub registers: u64,
    /// Wasm globals.
    pub globals: u64,
    /// Futex table, waiter queue and wake log.
    pub futex: u64,
    /// Hash of the four fields above.
    pub combined: u64,
}

/// Sparse linear memory plus the CPU state a tick may mutate.
pub struct Vm {
    pages: BTreeMap<u32, Box<[u8]>>,
    /// 32 wasm64 general-purpose registers.
    pub regs: [u64; 32],
    /// 32 v128 registers, stored as raw lanes.
    pub fregs: [u128; 32],
    /// 16 wasm globals.
    pub globals: [u64; 16],
    /// Program counter / function id of the last dispatched function.
    pub pc: u64,
    /// Engine frame counter (`GetGameFrame`, 1-based inside a tick).
    pub frame: i32,
    /// Executed tick index.
    pub tick: u64,
    /// Modelled futex subsystem.
    pub futex: FutexTable,
}

impl Vm {
    /// Empty image: every page reads as zero until written.
    pub fn new() -> Self {
        Self {
            pages: BTreeMap::new(),
            regs: [0; 32],
            fregs: [0; 32],
            globals: [0; 16],
            pc: 0,
            frame: 0,
            tick: 0,
            futex: FutexTable::new(),
        }
    }

    /// Deterministic boot image, plus a process-specific nonce in the two
    /// excluded pages.
    ///
    /// The 4 077 passive segments are applied at `DATA_BASE + 0x1000`, 32 bytes
    /// each, in index order — the same bytes in every process.
    pub fn boot(volatile_nonce: u64) -> Self {
        let mut vm = Self::new();
        vm.write_u32(DATA_BASE, IMAGE_MAGIC);
        vm.write_u32(DATA_BASE + 4, LAYOUT_VERSION);
        vm.write_u32(DATA_BASE + 8, SEGMENT_COUNT as u32);
        vm.write_u32(DATA_BASE + 12, SEGMENT_BYTES as u32);

        let mut segment = [0u8; SEGMENT_BYTES];
        for i in 0..SEGMENT_COUNT {
            for (j, byte) in segment.iter_mut().enumerate() {
                *byte = ((i.wrapping_mul(31).wrapping_add(j.wrapping_mul(17))) & 0xFF) as u8;
            }
            let addr = DATA_BASE + 0x1000 + (i * SEGMENT_BYTES) as u32;
            vm.write(addr, &segment);
        }

        for (i, g) in vm.globals.iter_mut().enumerate() {
            *g = 0x9E37_79B9_7F4A_7C15u64.wrapping_mul(i as u64 + 1);
        }
        for (i, r) in vm.regs.iter_mut().enumerate() {
            *r = 0x0123_4567_89AB_CDEFu64 ^ (i as u64).wrapping_mul(0x0101_0101_0101_0101);
        }

        // Deterministic initial futex words.
        for i in 0..16u32 {
            vm.write_u32(FUTEX_BASE + i * 4, i % 2);
        }

        // Volatile host bookkeeping — EXCLUDED from the digest.
        let pid = std::process::id() as u64;
        vm.write(STACK_BASE, &volatile_nonce.to_le_bytes());
        vm.write(STACK_BASE + 8, &pid.to_le_bytes());
        vm.write(TLS_BASE, &volatile_nonce.rotate_left(17).to_le_bytes());
        vm.write(TLS_BASE + 16, &pid.rotate_left(9).to_le_bytes());
        vm
    }

    fn page_mut(&mut self, index: u32) -> &mut [u8] {
        self.pages
            .entry(index)
            .or_insert_with(|| vec![0u8; PAGE_SIZE].into_boxed_slice())
    }

    /// Write `bytes` at `addr`, allocating pages as needed.
    pub fn write(&mut self, addr: u32, bytes: &[u8]) {
        assert!(
            addr as u64 + bytes.len() as u64 <= RESERVED_BYTES,
            "write outside reserved memory"
        );
        let mut pos = 0usize;
        let mut at = addr as u64;
        while pos < bytes.len() {
            let page = (at / PAGE_SIZE as u64) as u32;
            let off = (at % PAGE_SIZE as u64) as usize;
            let n = (PAGE_SIZE - off).min(bytes.len() - pos);
            let dst = self.page_mut(page);
            dst[off..off + n].copy_from_slice(&bytes[pos..pos + n]);
            pos += n;
            at += n as u64;
        }
    }

    /// Read `len` bytes at `addr`; absent pages read as zero.
    pub fn read(&self, addr: u32, len: usize) -> Vec<u8> {
        let mut out = vec![0u8; len];
        let mut pos = 0usize;
        let mut at = addr as u64;
        while pos < len {
            let page = (at / PAGE_SIZE as u64) as u32;
            let off = (at % PAGE_SIZE as u64) as usize;
            let n = (PAGE_SIZE - off).min(len - pos);
            if let Some(src) = self.pages.get(&page) {
                out[pos..pos + n].copy_from_slice(&src[off..off + n]);
            }
            pos += n;
            at += n as u64;
        }
        out
    }

    /// Read a little-endian u32.
    pub fn read_u32(&self, addr: u32) -> u32 {
        let b = self.read(addr, 4);
        u32::from_le_bytes([b[0], b[1], b[2], b[3]])
    }

    /// Read a little-endian u64.
    pub fn read_u64(&self, addr: u32) -> u64 {
        let b = self.read(addr, 8);
        u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
    }

    /// Write a little-endian u32.
    pub fn write_u32(&mut self, addr: u32, value: u32) {
        self.write(addr, &value.to_le_bytes());
    }

    /// Write a little-endian u64.
    pub fn write_u64(&mut self, addr: u32, value: u64) {
        self.write(addr, &value.to_le_bytes());
    }

    /// Raw page bytes, or `None` when the page has never been touched.
    pub fn raw_page(&self, index: u32) -> Option<&[u8]> {
        self.pages.get(&index).map(|p| &p[..])
    }

    /// Number of allocated pages (including the two excluded ones).
    pub fn allocated_pages(&self) -> usize {
        self.pages.len()
    }

    /// Hash of `[start, end)` (read through the zero-fill view).
    pub fn hash_range(&self, start: u32, len: usize) -> u64 {
        let mut h = StateHasher::new();
        h.write_u64(len as u64);
        h.write(&self.read(start, len));
        h.finish()
    }

    /// One digest per included page, in page-index order.
    pub fn page_hashes(&self) -> Vec<(u32, u64)> {
        let mut out = Vec::new();
        for (&index, page) in &self.pages {
            let start = index as u64 * PAGE_SIZE as u64;
            if excluded(start, start + PAGE_SIZE as u64) {
                continue;
            }
            let mut h = StateHasher::new();
            h.write_u32(index);
            h.write(page);
            out.push((index, h.finish()));
        }
        out
    }

    /// Page-granular memory digest, excluding stack and TLS.
    pub fn memory_digest(&self) -> u64 {
        let included = self
            .pages
            .keys()
            .filter(|&&index| {
                let start = index as u64 * PAGE_SIZE as u64;
                !excluded(start, start + PAGE_SIZE as u64)
            })
            .count();
        let mut h = StateHasher::new();
        h.write_u32(RESERVED_PAGES);
        h.write_u64(included as u64);
        for (&index, page) in &self.pages {
            let start = index as u64 * PAGE_SIZE as u64;
            if excluded(start, start + PAGE_SIZE as u64) {
                continue;
            }
            h.write_u32(index);
            h.write(page);
        }
        h.finish()
    }

    /// Registers, pc, frame counter and tick index.
    pub fn register_digest(&self) -> u64 {
        let mut h = StateHasher::new();
        for r in self.regs {
            h.write_u64(r);
        }
        for v in self.fregs {
            h.write(&v.to_le_bytes());
        }
        h.write_u64(self.pc);
        h.write_i32(self.frame);
        h.write_u64(self.tick);
        h.finish()
    }

    /// Wasm globals.
    pub fn globals_digest(&self) -> u64 {
        let mut h = StateHasher::new();
        h.write_u64(self.globals.len() as u64);
        for g in self.globals {
            h.write_u64(g);
        }
        h.finish()
    }

    /// Full state digest.
    pub fn digest(&self) -> StateDigest {
        let memory = self.memory_digest();
        let registers = self.register_digest();
        let globals = self.globals_digest();
        let futex = self.futex.digest();
        let mut h = StateHasher::new();
        h.write_u64(memory);
        h.write_u64(registers);
        h.write_u64(globals);
        h.write_u64(futex);
        StateDigest {
            memory,
            registers,
            globals,
            futex,
            combined: h.finish(),
        }
    }
}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}
