//! Linear-memory reservation with lazy commit.
//!
//! The wasm64 guest addresses one contiguous 64-bit linear memory. The host
//! reserves the full window up front (3 GiB by default) but only commits pages
//! that a data segment or a runtime write actually touches. On both supported
//! OSes that is a real reservation, not a 3 GiB heap allocation:
//!
//! * Windows — `VirtualAlloc(MEM_RESERVE)` for the window, `VirtualAlloc(MEM_COMMIT)`
//!   in 64 KiB granules as pages are needed.
//! * Linux — `mmap(PROT_NONE, MAP_NORESERVE)` for the window, then `mprotect`
//!   to `PROT_READ|PROT_WRITE` per page range.
//!
//! Everything else falls back to a zeroed heap buffer (the OS over-commits it),
//! so the crate still builds; the fallback is reported in the status line as
//! `"mem":"heap"`.

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use crate::{log_debug, log_info};

pub const DEFAULT_RESERVE_BYTES: u64 = 3 * 1024 * 1024 * 1024;

#[derive(Debug)]
pub enum MemError {
    Reserve { bytes: u64, os: i32 },
    Commit { offset: u64, len: u64, os: i32 },
    OutOfBounds { offset: u64, len: u64, size: u64 },
}

impl fmt::Display for MemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemError::Reserve { bytes, os } => write!(
                f,
                "could not reserve {} MiB of linear memory (os error {})",
                bytes / (1024 * 1024),
                os
            ),
            MemError::Commit { offset, len, os } => write!(
                f,
                "could not commit {} bytes at offset {:#x} (os error {})",
                len, offset, os
            ),
            MemError::OutOfBounds { offset, len, size } => write!(
                f,
                "write of {} bytes at offset {:#x} exceeds linear memory ({} bytes)",
                len, offset, size
            ),
        }
    }
}

impl std::error::Error for MemError {}

impl From<MemError> for crate::ServerError {
    fn from(e: MemError) -> Self {
        crate::ServerError::Memory(e)
    }
}

/// Reserved linear memory owned by the host.
///
/// The guest pointer space is `base .. base + size`. Access is routed through
/// the methods below so that every write commits its pages first.
pub struct LinearMemory {
    inner: Inner,
    size: u64,
    page: u64,
    /// Sorted, non-overlapping committed ranges in guest offsets.
    committed: Mutex<Vec<(u64, u64)>>,
    committed_bytes: AtomicU64,
}

enum Inner {
    #[cfg(windows)]
    Win { base: *mut u8 },
    #[cfg(all(unix, not(target_os = "macos")))]
    Unix { base: *mut u8 },
    Heap(Vec<u8>),
}

// SAFETY: the reservation is process-global OS memory, not a Rust borrow of any
// stack or heap object. All mutation of guest bytes happens inside `commit` +
// `copy`, with the range list guarded by a mutex; the base pointer is freed
// only in `Drop`, which requires `&mut self`.
unsafe impl Send for LinearMemory {}
unsafe impl Sync for LinearMemory {}

impl LinearMemory {
    /// Reserve `size` bytes of guest address space. Nothing is committed yet.
    pub fn reserve(size: u64) -> Result<LinearMemory, MemError> {
        let page = crate::sys::page_size().max(4096);
        let size = size.max(page);
        let size = (size + page - 1) & !(page - 1);

        #[cfg(windows)]
        {
            let base = unsafe {
                VirtualAlloc(
                    std::ptr::null_mut(),
                    size as usize,
                    MEM_RESERVE,
                    PAGE_NOACCESS,
                )
            };
            if base.is_null() {
                return Err(MemError::Reserve {
                    bytes: size,
                    os: unsafe { GetLastError() } as i32,
                });
            }
            log_info!(
                "mem",
                "reserved {} MiB of linear memory (lazy commit)",
                size / (1024 * 1024)
            );
            return Ok(LinearMemory {
                inner: Inner::Win { base: base as *mut u8 },
                size,
                page,
                committed: Mutex::new(Vec::new()),
                committed_bytes: AtomicU64::new(0),
            });
        }

        #[cfg(all(unix, not(target_os = "macos")))]
        {
            let base = unsafe {
                mmap(
                    std::ptr::null_mut(),
                    size as usize,
                    PROT_NONE,
                    MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE,
                    -1,
                    0,
                )
            };
            // mmap reports failure as MAP_FAILED == (void*)-1
            if base as isize == -1 {
                return Err(MemError::Reserve {
                    bytes: size,
                    os: errno(),
                });
            }
            log_info!(
                "mem",
                "reserved {} MiB of linear memory (MAP_NORESERVE, lazy commit)",
                size / (1024 * 1024)
            );
            return Ok(LinearMemory {
                inner: Inner::Unix { base: base as *mut u8 },
                size,
                page,
                committed: Mutex::new(Vec::new()),
                committed_bytes: AtomicU64::new(0),
            });
        }

        #[cfg(not(any(windows, all(unix, not(target_os = "macos")))))]
        {
            log_info!(
                "mem",
                "reserved {} MiB of linear memory (heap fallback, OS-zeroed)",
                size / (1024 * 1024)
            );
            let mut buf = Vec::new();
            buf.try_reserve_exact(size as usize).map_err(|_| MemError::Reserve {
                bytes: size,
                os: 0,
            })?;
            buf.resize(size as usize, 0);
            Ok(LinearMemory {
                inner: Inner::Heap(buf),
                size,
                page,
                committed: Mutex::new(Vec::new()),
                committed_bytes: AtomicU64::new(0),
            })
        }
    }

    pub fn len(&self) -> u64 {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Backend name for the status line.
    pub fn backend(&self) -> &'static str {
        match self.inner {
            #[cfg(windows)]
            Inner::Win { .. } => "virtualalloc",
            #[cfg(all(unix, not(target_os = "macos")))]
            Inner::Unix { .. } => "mmap-noreserve",
            Inner::Heap(_) => "heap",
        }
    }

    pub fn base(&self) -> *mut u8 {
        match &self.inner {
            #[cfg(windows)]
            Inner::Win { base } => *base,
            #[cfg(all(unix, not(target_os = "macos")))]
            Inner::Unix { base } => *base,
            Inner::Heap(buf) => buf.as_ptr() as *mut u8,
        }
    }

    /// Make `[offset, offset+len)` backed by real memory. Idempotent.
    pub fn commit(&self, offset: u64, len: u64) -> Result<(), MemError> {
        if len == 0 {
            return Ok(());
        }
        let end = offset.checked_add(len).ok_or(MemError::OutOfBounds {
            offset,
            len,
            size: self.size,
        })?;
        if end > self.size {
            return Err(MemError::OutOfBounds {
                offset,
                len,
                size: self.size,
            });
        }

        if matches!(self.inner, Inner::Heap(_)) {
            // Heap fallback is always resident; keep the accounting consistent.
            let mut ranges = self.committed.lock().expect("commit lock");
            record_range(&mut ranges, offset, end);
            drop(ranges);
            self.committed_bytes.store(0, Ordering::Relaxed);
            return Ok(());
        }

        let start = offset & !(self.page - 1);
        let end = (end + self.page - 1) & !(self.page - 1);
        let clamped_end = end.min(self.size);

        let mut ranges = self.committed.lock().expect("commit lock");
        for (s, e) in uncovered(&ranges, start, clamped_end) {
            self.os_commit(s, e - s)?;
            record_range(&mut ranges, s, e);
        }
        let total: u64 = ranges.iter().map(|(s, e)| e - s).sum();
        drop(ranges);
        self.committed_bytes.store(total, Ordering::Relaxed);
        log_debug!("mem", "committed {:#x}..{:#x}", start, clamped_end);
        Ok(())
    }

    #[cfg(windows)]
    fn os_commit(&self, offset: u64, len: u64) -> Result<(), MemError> {
        let addr = unsafe { self.base().add(offset as usize) };
        let len = round_up(len, COMMIT_GRANULARITY);
        let result = unsafe { VirtualAlloc(addr as *mut _, len as usize, MEM_COMMIT, PAGE_READWRITE) };
        if result.is_null() {
            return Err(MemError::Commit {
                offset,
                len,
                os: unsafe { GetLastError() } as i32,
            });
        }
        Ok(())
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    fn os_commit(&self, offset: u64, len: u64) -> Result<(), MemError> {
        let addr = unsafe { self.base().add(offset as usize) };
        let rc = unsafe { mprotect(addr as *mut _, len as usize, PROT_READ | PROT_WRITE) };
        if rc != 0 {
            return Err(MemError::Commit {
                offset,
                len,
                os: errno(),
            });
        }
        Ok(())
    }

    #[cfg(not(any(windows, all(unix, not(target_os = "macos")))))]
    fn os_commit(&self, _offset: u64, _len: u64) -> Result<(), MemError> {
        Ok(())
    }

    /// Copy host bytes into guest memory, committing the target pages first.
    pub fn write(&self, offset: u64, bytes: &[u8]) -> Result<(), MemError> {
        if bytes.is_empty() {
            return Ok(());
        }
        let end = offset.checked_add(bytes.len() as u64).ok_or(MemError::OutOfBounds {
            offset,
            len: bytes.len() as u64,
            size: self.size,
        })?;
        if end > self.size {
            return Err(MemError::OutOfBounds {
                offset,
                len: bytes.len() as u64,
                size: self.size,
            });
        }
        self.commit(offset, bytes.len() as u64)?;
        // SAFETY: the range was bounds-checked against the reservation and
        // committed above, so it is writable for `bytes.len()` bytes.
        unsafe {
            std::ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                self.base().add(offset as usize),
                bytes.len(),
            );
        }
        Ok(())
    }

    /// Copy bytes out of guest memory (used by tests and diagnostics).
    pub fn read(&self, offset: u64, len: usize) -> Result<Vec<u8>, MemError> {
        let end = offset
            .checked_add(len as u64)
            .ok_or(MemError::OutOfBounds { offset, len: len as u64, size: self.size })?;
        if end > self.size {
            return Err(MemError::OutOfBounds { offset, len: len as u64, size: self.size });
        }
        self.commit(offset, len as u64)?;
        let mut out = vec![0u8; len];
        // SAFETY: same reasoning as `write`; the source range is committed.
        unsafe {
            std::ptr::copy_nonoverlapping(self.base().add(offset as usize), out.as_mut_ptr(), len);
        }
        Ok(out)
    }

    pub fn committed_bytes(&self) -> u64 {
        self.committed_bytes.load(Ordering::Relaxed)
    }

    pub fn committed_ranges(&self) -> usize {
        self.committed.lock().map(|r| r.len()).unwrap_or(0)
    }
}

#[cfg(windows)]
const MEM_COMMIT: u32 = 0x0000_1000;
#[cfg(windows)]
const MEM_RESERVE: u32 = 0x0000_2000;
#[cfg(windows)]
const MEM_RELEASE: u32 = 0x0000_8000;
#[cfg(windows)]
const PAGE_NOACCESS: u32 = 0x01;
#[cfg(windows)]
const PAGE_READWRITE: u32 = 0x04;
#[cfg(windows)]
pub const COMMIT_GRANULARITY: u64 = 64 * 1024;

#[cfg(windows)]
extern "system" {
    fn VirtualAlloc(addr: *mut std::ffi::c_void, size: usize, ty: u32, protect: u32) -> *mut std::ffi::c_void;
    fn VirtualFree(addr: *mut std::ffi::c_void, size: usize, ty: u32) -> i32;
    fn GetLastError() -> u32;
}

#[cfg(all(unix, not(target_os = "macos")))]
pub const COMMIT_GRANULARITY: u64 = 4096;

#[cfg(all(unix, not(target_os = "macos")))]
extern "C" {
    fn mmap(addr: *mut std::ffi::c_void, len: usize, prot: i32, flags: i32, fd: i32, off: i64) -> *mut std::ffi::c_void;
    fn munmap(addr: *mut std::ffi::c_void, len: usize) -> i32;
    fn mprotect(addr: *mut std::ffi::c_void, len: usize, prot: i32) -> i32;
    fn errno_location() -> *mut i32;
}

#[cfg(all(unix, not(target_os = "macos")))]
const PROT_NONE: i32 = 0;
#[cfg(all(unix, not(target_os = "macos")))]
const PROT_READ: i32 = 1;
#[cfg(all(unix, not(target_os = "macos")))]
const PROT_WRITE: i32 = 2;
#[cfg(all(unix, not(target_os = "macos")))]
const MAP_PRIVATE: i32 = 0x02;
#[cfg(all(unix, not(target_os = "macos")))]
const MAP_ANONYMOUS: i32 = 0x20;
#[cfg(all(unix, not(target_os = "macos")))]
const MAP_NORESERVE: i32 = 0x4000;

#[cfg(all(unix, not(target_os = "macos")))]
fn errno() -> i32 {
    // SAFETY: errno_location always returns a valid thread-local pointer.
    unsafe { *errno_location() }
}

fn round_up(value: u64, align: u64) -> u64 {
    if align == 0 {
        return value;
    }
    (value + align - 1) & !(align - 1)
}

fn record_range(ranges: &mut Vec<(u64, u64)>, start: u64, end: u64) {
    ranges.push((start, end));
    ranges.sort_unstable();
    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());
    for (s, e) in ranges.drain(..) {
        match merged.last_mut() {
            Some((_, last_end)) if s <= *last_end => {
                if e > *last_end {
                    *last_end = e;
                }
            }
            _ => merged.push((s, e)),
        }
    }
    *ranges = merged;
}

/// Sub-ranges of `[start, end)` not already covered by `ranges`.
fn uncovered(ranges: &[(u64, u64)], start: u64, end: u64) -> Vec<(u64, u64)> {
    let mut gaps = Vec::new();
    let mut cursor = start;
    for &(s, e) in ranges {
        if e <= cursor || s >= end {
            continue;
        }
        if s > cursor {
            gaps.push((cursor, s.min(end)));
        }
        cursor = cursor.max(e);
        if cursor >= end {
            break;
        }
    }
    if cursor < end {
        gaps.push((cursor, end));
    }
    gaps
}

impl Drop for LinearMemory {
    fn drop(&mut self) {
        #[cfg(windows)]
        if let Inner::Win { base } = self.inner {
            // SAFETY: `base` came from VirtualAlloc and is released once here.
            unsafe {
                VirtualFree(base as *mut _, 0, MEM_RELEASE);
            }
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        if let Inner::Unix { base } = self.inner {
            // SAFETY: `base` came from mmap and is unmapped once here.
            unsafe {
                munmap(base as *mut _, self.size as usize);
            }
        }
    }
}

impl fmt::Debug for LinearMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LinearMemory")
            .field("size", &self.size)
            .field("backend", &self.backend())
            .field("committed_bytes", &self.committed_bytes())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserve_is_lazy() {
        let mem = LinearMemory::reserve(64 * 1024 * 1024).unwrap();
        assert_eq!(mem.len(), 64 * 1024 * 1024);
        assert_eq!(mem.committed_bytes(), 0, "reservation must not touch pages");
    }

    #[test]
    fn write_commits_only_touched_pages() {
        let mem = LinearMemory::reserve(8 * 1024 * 1024).unwrap();
        mem.write(4096, b"hello").unwrap();
        assert!(mem.committed_bytes() >= 4096 && mem.committed_bytes() <= 64 * 1024);
        assert_eq!(mem.read(4096, 5).unwrap(), b"hello");
    }

    #[test]
    fn repeated_commits_coalesce() {
        let mem = LinearMemory::reserve(8 * 1024 * 1024).unwrap();
        for i in 0..16u64 {
            mem.write(i * 4096, &[0xAB]).unwrap();
        }
        assert_eq!(mem.committed_ranges(), 1, "adjacent pages should coalesce");
    }

    #[test]
    fn out_of_bounds_is_rejected() {
        let mem = LinearMemory::reserve(4096 * 4).unwrap();
        let err = mem.write(4096 * 4 - 1, b"xx").unwrap_err();
        assert!(matches!(err, MemError::OutOfBounds { .. }), "{err}");
    }

    #[test]
    fn gaps_are_computed_correctly() {
        let ranges = [(0u64, 4096u64), (8192, 12288)];
        assert_eq!(uncovered(&ranges, 0, 12288), vec![(4096, 8192)]);
        assert_eq!(uncovered(&ranges, 0, 2048), Vec::<(u64, u64)>::new());
        assert_eq!(uncovered(&[], 0, 8192), vec![(0, 8192)]);
    }
}
