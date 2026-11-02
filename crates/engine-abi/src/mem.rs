//! Linear-memory pointer helpers.
//!
//! The engine is wasm64: every "pointer" it passes is a byte offset into one
//! linear memory, and it is handed to the ABI as an `i64`. To stay
//! byte-compatible with the wasm import types, the shims take those pointers as
//! `*mut u8` / `*const u8` (same 64-bit width, no truncation) and resolve them
//! here against the memory published with [`crate::state::set_memory`].
//!
//! Nothing in this module allocates, and no accessor ever dereferences a pointer
//! the engine gave us without first range-checking it against the installed
//! memory. A pointer outside the mapping yields `None`, so a corrupt or foreign
//! pointer becomes a documented error return instead of a segfault.
//!
//! ## Aliasing model
//!
//! wasm has no aliasing rules: the same bytes are read and written by whichever
//! function runs next, from any thread. The engine keeps that model after
//! translation, so the slices returned here are shared views into memory the
//! engine may be writing concurrently. They are therefore `unsafe`-contract
//! material in the Rust sense even though the accessors themselves are safe to
//! call: the caller must not hold one across a call back into the engine.

use std::sync::atomic::Ordering;

use crate::state;

/// Longest C string the engine may hand us; guards against a missing NUL.
pub const MAX_CSTR: usize = 8192;

/// Resolve `ptr..ptr+len` to a shared view of linear memory.
///
/// Returns `None` when no memory is installed or the range is out of bounds.
pub fn bytes(ptr: *const u8, len: usize) -> Option<&'static [u8]> {
    let base = state::memory_base();
    if base == 0 && !state::memory_installed() {
        return None;
    }
    let mlen = state::memory_len();
    let off = ptr as usize;
    let end = off.checked_add(len)?;
    if end > mlen {
        return None;
    }
    // SAFETY: `base` was published by `state::set_memory` with at least `mlen`
    // valid bytes, and the range was just range-checked against `mlen`.
    Some(unsafe { std::slice::from_raw_parts((base as *const u8).add(off), len) })
}

/// Resolve `ptr..ptr+len` to an exclusive view of linear memory.
///
/// Returns `None` when no memory is installed or the range is out of bounds.
pub fn bytes_mut(ptr: *mut u8, len: usize) -> Option<&'static mut [u8]> {
    let base = state::memory_base();
    if base == 0 && !state::memory_installed() {
        return None;
    }
    let mlen = state::memory_len();
    let off = ptr as usize;
    let end = off.checked_add(len)?;
    if end > mlen {
        return None;
    }
    // SAFETY: same range check as `bytes`; uniqueness is the caller's problem
    // under the wasm aliasing model documented at the module level.
    Some(unsafe { std::slice::from_raw_parts_mut((base as *mut u8).add(off), len) })
}

/// Read a NUL-terminated UTF-8 string at `ptr`.
///
/// Returns `None` for an out-of-range pointer, a missing terminator within
/// [`MAX_CSTR`] bytes, or invalid UTF-8 (the engine only produces ASCII/UTF-8
/// for the strings it hands to the host).
pub fn cstr(ptr: *const u8) -> Option<&'static str> {
    let base = state::memory_base();
    if base == 0 && !state::memory_installed() {
        return None;
    }
    let mlen = state::memory_len();
    let off = ptr as usize;
    if off >= mlen {
        return None;
    }
    let limit = (mlen - off).min(MAX_CSTR);
    // SAFETY: `off < mlen` and `limit <= mlen - off`, both inside the mapping.
    let raw = unsafe { std::slice::from_raw_parts((base as *const u8).add(off), limit) };
    let end = raw.iter().position(|b| *b == 0)?;
    std::str::from_utf8(&raw[..end]).ok()
}

/// Copy `len` bytes from `ptr` into an owned `Vec` (used where the host API
/// needs an owned buffer; never used for pointers we only probe).
pub fn to_vec(ptr: *const u8, len: usize) -> Option<Vec<u8>> {
    bytes(ptr, len).map(<[u8]>::to_vec)
}

/// Read a little-endian `u32` at `ptr`.
pub fn read_u32(ptr: *const u8) -> Option<u32> {
    let b = bytes(ptr, 4)?;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

/// Read a little-endian `u64` at `ptr`.
pub fn read_u64(ptr: *const u8) -> Option<u64> {
    let b = bytes(ptr, 8)?;
    Some(u64::from_le_bytes([
        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
    ]))
}

/// Read a little-endian `i64` at `ptr`.
pub fn read_i64(ptr: *const u8) -> Option<i64> {
    read_u64(ptr).map(|v| v as i64)
}

/// Read a little-endian `f32` at `ptr`.
pub fn read_f32(ptr: *const u8) -> Option<f32> {
    let b = bytes(ptr, 4)?;
    Some(f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

/// Write `src` to `ptr..ptr+src.len()`; returns `false` when out of range.
pub fn write_bytes(ptr: *mut u8, src: &[u8]) -> bool {
    match bytes_mut(ptr, src.len()) {
        Some(dst) => {
            dst.copy_from_slice(src);
            true
        }
        None => false,
    }
}

/// Write a little-endian `u32`; returns `false` when out of range.
pub fn write_u32(ptr: *mut u8, value: u32) -> bool {
    write_bytes(ptr, &value.to_le_bytes())
}

/// Write a little-endian `u64`; returns `false` when out of range.
pub fn write_u64(ptr: *mut u8, value: u64) -> bool {
    write_bytes(ptr, &value.to_le_bytes())
}

/// Write a little-endian `i32`; returns `false` when out of range.
pub fn write_i32(ptr: *mut u8, value: i32) -> bool {
    write_bytes(ptr, &(value as u32).to_le_bytes())
}

/// Write a little-endian `i64`; returns `false` when out of range.
pub fn write_i64(ptr: *mut u8, value: i64) -> bool {
    write_bytes(ptr, &(value as u64).to_le_bytes())
}

/// Zero `len` bytes at `ptr`; returns `false` when out of range.
pub fn zero(ptr: *mut u8, len: usize) -> bool {
    match bytes_mut(ptr, len) {
        Some(dst) => {
            dst.fill(0);
            true
        }
        None => false,
    }
}

/// Whether `ptr` resolves inside the installed memory (used to tell a valid
/// engine pointer from the null constant).
pub fn is_valid(ptr: *const u8, len: usize) -> bool {
    bytes(ptr, len).is_some()
}

/// Current memory size, straight from the state module.
pub fn capacity() -> usize {
    state::memory_len()
}

/// Ordering re-export so callers can annotate their own atomics consistently.
pub const ORDERING: Ordering = Ordering::Relaxed;