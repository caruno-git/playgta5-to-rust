//! Runtime support for the machine-translated wasm64 engine.
//!
//! Scope: the parts of the runtime that the translated corpus calls into but
//! that are *not* expressible as straight-line Rust:
//!
//! * [`Memory`] — linear memory with wasm bounds semantics. Every accessor
//!   returns [`Trap`] instead of panicking; there is no `unwrap` on any path
//!   reachable from generated code.
//! * [`segments`] — the 4 077 passive data segments of the module (7.8 MB in
//!   the shipping build; bytes are extracted from that binary).
//! * [`memory_init`] / [`data_drop`] — the passive-segment instructions. After
//!   `data.drop` a segment reads as length zero, so a later non-empty
//!   `memory.init` from it traps, and a zero-length init still succeeds.
//! * [`memory_copy`] / [`memory_fill`] — bulk memory, both bounds-checked
//!   *before* any byte is touched, with `memmove` overlap semantics.
//! * [`Memory::a_wait32`] / [`Memory::a_notify`] — `memory.atomic.wait32` /
//!   `notify` with the wasm return codes (0 woken, 1 not-equal, 2 timed out).
//! * [`simd`] — 128-bit helpers (`V = u128`, little-endian lane order).
//!
//! Arithmetic that wasm defines as wrapping is spelled `wrapping_*`; the only
//! saturating operations are the `*_sat_*` SIMD ops and the saturating adds in
//! the futex deadlines, which is what their names promise.

#![allow(clippy::cast_possible_truncation)] // lane narrowing is intentional and masked
#![cfg_attr(target_arch = "wasm64", feature(simd_wasm64, stdarch_wasm_atomic_wait, simd_ffi))]

use std::fmt;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, MutexGuard};

mod futex;
#[cfg(target_arch = "wasm64")]
mod guest_ops;
pub mod init_events;
mod numeric;
pub mod segments;
pub mod simd;
mod simd_shuffle;
mod translated;
pub use numeric::*;
pub use translated::Phantom;

use segments::{SEG_COUNT, SEG_LENS};

/// WebAssembly page size: 64 KiB.
pub const PAGE_SIZE: u64 = 65_536;

/// Which memory access tripped the bounds check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    /// `memory.init` destination.
    Init,
    /// `memory.copy` destination.
    CopyDst,
    /// `memory.copy` source.
    CopySrc,
    /// `memory.fill` destination.
    Fill,
    /// A plain load/store helper.
    Load,
    /// A plain load/store helper.
    Store,
    /// An atomic load (futex word read).
    Atomic,
}

/// A wasm trap. Traps are values here, not panics, so the caller decides
/// whether to unwind the instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trap {
    /// An access ran past the end of linear memory.
    OutOfBounds {
        /// Which access.
        access: Access,
        /// First address of the access.
        addr: u64,
        /// Access length in bytes.
        len: u64,
        /// Current memory size in bytes.
        size: u64,
    },
    /// A `memory.init` source range ran past the (possibly dropped) segment.
    SegmentOutOfRange {
        /// Segment index.
        segment: u32,
        /// Requested source offset.
        src: u64,
        /// Requested length.
        len: u64,
        /// Effective segment length (0 after `data.drop`).
        seglen: u64,
    },
    /// A segment index that the module does not declare.
    NoSuchSegment {
        /// Segment index.
        segment: u32,
    },
    /// `memory.grow` past the declared maximum.
    GrowLimit {
        /// Requested total pages.
        want: u64,
        /// Declared maximum.
        max: u64,
    },
    UnalignedAtomic {
        addr: u64,
    },
}

impl fmt::Display for Trap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Trap::OutOfBounds {
                access,
                addr,
                len,
                size,
            } => write!(
                f,
                "out of bounds {:?}: [{:#x}, {:#x}) is past memory size {:#x}",
                access,
                addr,
                addr.saturating_add(*len),
                size
            ),
            Trap::SegmentOutOfRange {
                segment,
                src,
                len,
                seglen,
            } => write!(
                f,
                "memory.init: segment {segment} src {src:#x} len {len:#x} exceeds length {seglen:#x}"
            ),
            Trap::NoSuchSegment { segment } => write!(f, "no such data segment: {segment}"),
            Trap::GrowLimit { want, max } => {
                write!(f, "memory.grow to {want} pages exceeds max {max}")
            }
            Trap::UnalignedAtomic { addr } => write!(f, "unaligned atomic address {addr:#x}"),
        }
    }
}

impl std::error::Error for Trap {}

/// Lock a mutex, tolerating poisoning: a panic in another tick thread must not
/// turn a well-formed futex operation into a second panic.
fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

// ---------------------------------------------------------------------------
// Linear memory
// ---------------------------------------------------------------------------

/// Linear memory of a wasm64 instance, backed by one contiguous allocation.
///
/// Sizes are `u64` (wasm64 addresses), offsets within the buffer are `usize`;
/// the conversion is checked so a 32-bit host narrows nothing silently.
pub struct Memory {
    buf: Vec<u8>,
    max_pages: u64,
}

impl Memory {
    /// Allocate `initial_pages` of zeroed memory, refusing to exceed `max_pages`.
    pub fn new(initial_pages: u64, max_pages: u64) -> Result<Self, Trap> {
        if initial_pages > max_pages {
            return Err(Trap::GrowLimit {
                want: initial_pages,
                max: max_pages,
            });
        }
        let bytes = initial_pages
            .checked_mul(PAGE_SIZE)
            .and_then(|b| usize::try_from(b).ok())
            .ok_or(Trap::GrowLimit {
                want: initial_pages,
                max: max_pages,
            })?;
        let mut buf = Vec::new();
        buf.try_reserve_exact(bytes).map_err(|_| Trap::GrowLimit {
            want: initial_pages,
            max: max_pages,
        })?;
        buf.resize(bytes, 0);
        Ok(Self { buf, max_pages })
    }

    /// `memory.size`, in pages.
    pub fn pages(&self) -> u64 {
        self.buf.len() as u64 / PAGE_SIZE
    }

    /// Current size in bytes.
    pub fn size(&self) -> u64 {
        self.buf.len() as u64
    }

    /// `memory.grow`: returns the previous page count, or a trap past the max.
    pub fn grow(&mut self, delta_pages: u64) -> Result<u64, Trap> {
        let old = self.pages();
        let want = old.checked_add(delta_pages).ok_or(Trap::GrowLimit {
            want: u64::MAX,
            max: self.max_pages,
        })?;
        if want > self.max_pages {
            return Err(Trap::GrowLimit {
                want,
                max: self.max_pages,
            });
        }
        let bytes = want
            .checked_mul(PAGE_SIZE)
            .and_then(|b| usize::try_from(b).ok())
            .ok_or(Trap::GrowLimit {
                want,
                max: self.max_pages,
            })?;
        self.buf
            .try_reserve_exact(bytes - self.buf.len())
            .map_err(|_| Trap::GrowLimit {
                want,
                max: self.max_pages,
            })?;
        self.buf.resize(bytes, 0);
        Ok(old)
    }

    /// Bounds-check `[addr, addr + len)` against the current size.
    fn checked_range(&self, access: Access, addr: u64, len: u64) -> Result<(usize, usize), Trap> {
        let size = self.size();
        let end = addr.checked_add(len).ok_or(Trap::OutOfBounds {
            access,
            addr,
            len,
            size,
        })?;
        if end > size {
            return Err(Trap::OutOfBounds {
                access,
                addr,
                len,
                size,
            });
        }
        let start = usize::try_from(addr).map_err(|_| Trap::OutOfBounds {
            access,
            addr,
            len,
            size,
        })?;
        let stop = usize::try_from(end).map_err(|_| Trap::OutOfBounds {
            access,
            addr,
            len,
            size,
        })?;
        Ok((start, stop))
    }

    /// Immutable view of `[addr, addr + len)`.
    pub fn read(&self, addr: u64, len: u64) -> Result<&[u8], Trap> {
        let (start, stop) = self.checked_range(Access::Load, addr, len)?;
        Ok(&self.buf[start..stop])
    }

    /// Write `bytes` at `addr` (bounds-checked before any byte is written).
    pub fn write(&mut self, addr: u64, bytes: &[u8]) -> Result<(), Trap> {
        let len = bytes.len() as u64;
        let (start, stop) = self.checked_range(Access::Store, addr, len)?;
        self.buf[start..stop].copy_from_slice(bytes);
        Ok(())
    }

    /// Little-endian `u32` load.
    pub fn read_u32(&self, addr: u64) -> Result<u32, Trap> {
        let b = self.read(addr, 4)?;
        let mut a = [0u8; 4];
        a.copy_from_slice(b);
        Ok(u32::from_le_bytes(a))
    }

    /// Little-endian `u32` store.
    pub fn write_u32(&mut self, addr: u64, value: u32) -> Result<(), Trap> {
        self.write(addr, &value.to_le_bytes())
    }

    /// Little-endian `u64` load.
    pub fn read_u64(&self, addr: u64) -> Result<u64, Trap> {
        let b = self.read(addr, 8)?;
        let mut a = [0u8; 8];
        a.copy_from_slice(b);
        Ok(u64::from_le_bytes(a))
    }

    /// Little-endian `u64` store.
    pub fn write_u64(&mut self, addr: u64, value: u64) -> Result<(), Trap> {
        self.write(addr, &value.to_le_bytes())
    }

    /// Raw whole-buffer view (digests, image comparison).
    pub fn as_slice(&self) -> &[u8] {
        &self.buf
    }

    // -- futex -------------------------------------------------------------

    /// `memory.atomic.wait32(addr, expected, timeout_ns)`.
    ///
    /// Returns the wasm result: `0` woken, `1` the word did not hold
    /// `expected` (no wait happened), `2` the timeout elapsed. A negative
    /// timeout waits without a deadline.
    pub fn a_wait32(&self, addr: u64, expected: i32, timeout_ns: i64) -> Result<i32, Trap> {
        let ptr = self.atomic_word(addr)?;
        let word = unsafe { &*ptr };
        Ok(futex::wait(
            ptr as usize,
            || word.load(Ordering::SeqCst) == expected as u32,
            timeout_ns,
        ))
    }

    /// `memory.atomic.notify(addr, count)`: returns how many waiters were
    /// registered on the word (never more than `count`).
    pub fn a_notify(&self, addr: u64, count: i32) -> i32 {
        self.try_notify(addr, count)
            .unwrap_or_else(|e| trap(&e.to_string()))
    }

    pub fn try_notify(&self, addr: u64, count: i32) -> Result<i32, Trap> {
        Ok(futex::notify(self.atomic_word(addr)? as usize, count as u32) as i32)
    }

    fn atomic_word(&self, addr: u64) -> Result<*const AtomicU32, Trap> {
        let (start, _) = self.checked_range(Access::Atomic, addr, 4)?;
        let ptr = unsafe { self.buf.as_ptr().add(start) };
        if addr % 4 != 0 || ptr as usize % core::mem::align_of::<AtomicU32>() != 0 {
            return Err(Trap::UnalignedAtomic { addr });
        }
        Ok(ptr as *const AtomicU32)
    }

    /// Number of threads parked on `addr` (test observability only).
    pub fn waiting(&self, addr: u64) -> u32 {
        self.atomic_word(addr)
            .map(|p| futex::waiting(p as usize))
            .unwrap_or(0)
    }
}

impl Default for Memory {
    fn default() -> Self {
        // 1 page, no growth past 1 page; callers that need more use `new`.
        Self {
            buf: vec![0u8; PAGE_SIZE as usize],
            max_pages: 1,
        }
    }
}

// ---------------------------------------------------------------------------
// Passive data segments
// ---------------------------------------------------------------------------

/// Live state of the module's passive data segments.
///
/// `data.drop` is modelled the way wasm defines it: the segment keeps its slot
/// but its length becomes zero, so `memory.init` from it can still copy zero
/// bytes and traps on anything else.
pub struct SegmentState {
    lens: Vec<u32>,
    dropped: Vec<bool>,
}

impl SegmentState {
    /// All segments live, with the declared lengths.
    pub fn new() -> Self {
        Self {
            lens: SEG_LENS.to_vec(),
            dropped: vec![false; SEG_COUNT],
        }
    }

    /// Declared segment count of the module.
    pub fn len(&self) -> usize {
        self.lens.len()
    }

    /// Always false for a real module.
    pub fn is_empty(&self) -> bool {
        self.lens.is_empty()
    }

    /// Effective length of `idx` (`0` when dropped); `None` if undeclared.
    pub fn len_of(&self, idx: u32) -> Option<u32> {
        let i = usize::try_from(idx).ok()?;
        let len = *self.lens.get(i)?;
        if self.dropped[i] {
            Some(0)
        } else {
            Some(len)
        }
    }

    /// True when `idx` has been dropped.
    pub fn is_dropped(&self, idx: u32) -> bool {
        usize::try_from(idx)
            .ok()
            .and_then(|i| self.dropped.get(i))
            .copied()
            .unwrap_or(false)
    }

    /// `data.drop idx`. Dropping twice is a no-op, as in wasm.
    pub fn drop_segment(&mut self, idx: u32) -> Result<(), Trap> {
        match usize::try_from(idx)
            .ok()
            .and_then(|i| self.dropped.get_mut(i))
        {
            Some(slot) => {
                *slot = true;
                Ok(())
            }
            None => Err(Trap::NoSuchSegment { segment: idx }),
        }
    }

    /// Live bytes of `idx` (empty once dropped).
    pub fn bytes(&self, idx: u32) -> Result<&[u8], Trap> {
        let i = usize::try_from(idx)
            .ok()
            .filter(|i| *i < SEG_COUNT)
            .ok_or(Trap::NoSuchSegment { segment: idx })?;
        if self.dropped[i] {
            return Ok(&[]);
        }
        segments::segment(idx).ok_or(Trap::NoSuchSegment { segment: idx })
    }
}

impl Default for SegmentState {
    fn default() -> Self {
        Self::new()
    }
}

/// `memory.init seg, dst, src, len`.
///
/// Checks the segment range first, then the destination range, and copies only
/// when both pass — the source bytes are never read for a trapping call.
pub fn memory_init(
    mem: &mut Memory,
    segs: &SegmentState,
    segment: u32,
    dst: u64,
    src: u64,
    len: u64,
) -> Result<(), Trap> {
    let seglen = segs
        .len_of(segment)
        .ok_or(Trap::NoSuchSegment { segment })? as u64;
    let src_end = src.checked_add(len).ok_or(Trap::SegmentOutOfRange {
        segment,
        src,
        len,
        seglen,
    })?;
    if src_end > seglen {
        return Err(Trap::SegmentOutOfRange {
            segment,
            src,
            len,
            seglen,
        });
    }
    // Bounds-check the destination before copying anything.
    mem.checked_range(Access::Init, dst, len)?;
    if len == 0 {
        return Ok(());
    }
    let bytes = segs.bytes(segment)?;
    let from = usize::try_from(src).map_err(|_| Trap::SegmentOutOfRange {
        segment,
        src,
        len,
        seglen,
    })?;
    let count = usize::try_from(len).map_err(|_| Trap::SegmentOutOfRange {
        segment,
        src,
        len,
        seglen,
    })?;
    let chunk = bytes
        .get(from..from + count)
        .ok_or(Trap::SegmentOutOfRange {
            segment,
            src,
            len,
            seglen,
        })?;
    mem.write(dst, chunk)
}

/// `data.drop seg`.
pub fn data_drop(segs: &mut SegmentState, segment: u32) -> Result<(), Trap> {
    segs.drop_segment(segment)
}

/// `memory.copy dst, src, len` — `memmove` semantics, both ranges checked
/// before the copy starts.
pub fn memory_copy(mem: &mut Memory, dst: u64, src: u64, len: u64) -> Result<(), Trap> {
    let (d0, d1) = mem.checked_range(Access::CopyDst, dst, len)?;
    let (s0, s1) = mem.checked_range(Access::CopySrc, src, len)?;
    if len != 0 {
        mem.buf.copy_within(s0..s1, d0);
    }
    let _ = (d0, d1);
    Ok(())
}

/// `memory.fill dst, val, len`.
pub fn memory_fill(mem: &mut Memory, dst: u64, val: i32, len: u64) -> Result<(), Trap> {
    let (start, stop) = mem.checked_range(Access::Fill, dst, len)?;
    if len != 0 {
        mem.buf[start..stop].fill(val as u8);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Float helpers used by the generated SIMD code
// ---------------------------------------------------------------------------
//
// `f32x4.min` / `max` / `nearest` are not Rust's `f32::min` / `round`:
// wasm pins NaN propagation, the sign of zero and round-half-to-even, and the
// determinism contract requires identical results on every host.

/// `f32.min` with wasm's NaN and -0.0 rules.
#[inline]
pub fn f32_min(a: f32, b: f32) -> f32 {
    if a.is_nan() || b.is_nan() {
        f32::NAN
    } else if a == b {
        if a.is_sign_negative() {
            a
        } else {
            b
        }
    } else if a < b {
        a
    } else {
        b
    }
}

/// `f32.max` with wasm's NaN and -0.0 rules.
#[inline]
pub fn f32_max(a: f32, b: f32) -> f32 {
    if a.is_nan() || b.is_nan() {
        f32::NAN
    } else if a == b {
        if a.is_sign_negative() {
            b
        } else {
            a
        }
    } else if a > b {
        a
    } else {
        b
    }
}

/// `f64.min`.
#[inline]
pub fn f64_min(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a == b {
        if a.is_sign_negative() {
            a
        } else {
            b
        }
    } else if a < b {
        a
    } else {
        b
    }
}

/// `f64.max`.
#[inline]
pub fn f64_max(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a == b {
        if a.is_sign_negative() {
            b
        } else {
            a
        }
    } else if a > b {
        a
    } else {
        b
    }
}

/// `f32.nearest`: round half to even, preserving the sign of a zero result.
///
/// The sign matters: `nearest(-0.5)` is `-0.0`, not `+0.0`, so a naive
/// `round()` plus adjustment is not enough.
#[inline]
pub fn f32_nearest(a: f32) -> f32 {
    if !a.is_finite() {
        return a;
    }
    let r = a.round();
    let out = if (a - r).abs() == 0.5 && r % 2.0 != 0.0 {
        r - a.signum()
    } else {
        r
    };
    if out == 0.0 {
        0.0f32.copysign(a)
    } else {
        out
    }
}

/// `f64.nearest`: round half to even, preserving the sign of a zero result.
#[inline]
pub fn f64_nearest(a: f64) -> f64 {
    if !a.is_finite() {
        return a;
    }
    let r = a.round();
    let out = if (a - r).abs() == 0.5 && r % 2.0 != 0.0 {
        r - a.signum()
    } else {
        r
    };
    if out == 0.0 {
        0.0f64.copysign(a)
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearest_keeps_sign_of_zero() {
        assert_eq!(f32_nearest(-0.5).to_bits(), 0x8000_0000);
        assert_eq!(f32_nearest(0.5).to_bits(), 0x0000_0000);
        assert_eq!(f64_nearest(-0.5).to_bits(), 0x8000_0000_0000_0000);
    }

    #[test]
    fn traps_render_without_panic() {
        let t = Trap::OutOfBounds {
            access: Access::Init,
            addr: 16,
            len: 32,
            size: 16,
        };
        assert!(t.to_string().contains("out of bounds"));
    }
}
