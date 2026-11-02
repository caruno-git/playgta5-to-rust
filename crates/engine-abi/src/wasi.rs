//! `wasi_snapshot_preview1.*` — the wasi subset the module imports.
//!
//! Emscripten's wasm64 libc still issues the wasi calls for the descriptor-backed
//! I/O path (`fd_read`/`fd_write`/`fd_seek`), for `environ`, for the clock and for
//! process exit. They share the descriptor table in [`crate::fd`] with the
//! `__syscall_*` family, so a file opened through `openat` is readable through
//! `fd_read` — which is exactly how the original module behaved.
//!
//! ## Pointer width
//!
//! The module is wasm64: `size_t`, `*mut T` and the `iovec` fields are 8 bytes.
//! `iovec` is `{ buf: u64, len: u64 }` (16 bytes). All errno values are WASI
//! `errno` (`0` success, `8` `BADF`, `28` `INVAL`, `29` `IO`, `58` `NOTSUP`,
//! `70` `SPIPE`, `52` `NOSYS`).

use std::io::{Read, Seek, Write};

use crate::fd::{self, Entry};
use crate::mem;
use crate::state;

/// WASI success.
pub const WASI_ESUCCESS: i32 = 0;
/// WASI `BADF`.
pub const WASI_EBADF: i32 = 8;
/// WASI `FAULT`.
pub const WASI_EFAULT: i32 = 21;
/// WASI `INVAL`.
pub const WASI_EINVAL: i32 = 28;
/// WASI `IO`.
pub const WASI_EIO: i32 = 29;
/// WASI `NOSYS`.
pub const WASI_ENOSYS: i32 = 52;
/// WASI `NOTSUP`.
pub const WASI_ENOTSUP: i32 = 58;
/// WASI `SPIPE`.
pub const WASI_ESPIPE: i32 = 70;

/// Size of one wasm64 `iovec`/`ciovec` (`{ buf: u64, len: u64 }`).
const IOV_LEN: usize = 16;

/// Parse the iovec array at `iovs` into `(offset, len)` pairs.
fn iovecs(iovs: *const u8, count: u32) -> Option<Vec<(usize, usize)>> {
    let count = count as usize;
    let total = count.checked_mul(IOV_LEN)?;
    if count == 0 {
        return Some(Vec::new());
    }
    mem::bytes(iovs, total)?;
    let mut out = Vec::with_capacity(count);
    for index in 0..count {
        let base = iovs.wrapping_add(index * IOV_LEN);
        let offset = mem::read_u64(base)?;
        let len = mem::read_u64(base.wrapping_add(8))?;
        out.push((
            usize::try_from(offset).ok()?,
            usize::try_from(len).ok()?,
        ));
    }
    Some(out)
}

/// `wasi_snapshot_preview1.proc_exit` — wasm type `(func (param i32))`.
///
/// Terminates the process with `code`. Honours an installed exit hook (see
/// [`state::set_exit_hook`]) so a test can observe the request without dying;
/// otherwise calls `std::process::exit`, which is what `proc_exit` means for a
/// server binary (no `longjmp` back into the engine).
#[no_mangle]
pub extern "C" fn proc_exit(code: u32) {
    if state::dispatch_exit(code) {
        return;
    }
    std::process::exit(code as i32);
}

/// `env.exit` — wasm type `(func (param i32))`.
///
/// Emscripten's `exit()`: same contract as [`proc_exit`], kept as a separate
/// import because libc's `exit` and wasi's `proc_exit` were separate JS
/// functions. Both are wired to the same host behaviour.
#[no_mangle]
pub extern "C" fn exit(code: u32) {
    proc_exit(code);
}

/// `wasi_snapshot_preview1.fd_close` — wasm type `(func (param i32) (result i32))`.
///
/// Flushes and drops the descriptor. Closing an already-closed descriptor is
/// `EBADF`, and closing 0/1/2 is allowed (the entry stays in the table as a
/// closed marker) so libc's `fclose(stdout)` does not take the process down.
#[no_mangle]
pub unsafe extern "C" fn fd_close(fd: i32) -> i32 {
    fd::with(|table| {
        if table.get(fd).is_none() {
            return WASI_EBADF;
        }
        match table.remove(fd) {
            Some(mut entry) => {
                crate::syscall::flush_entry(&mut entry);
                WASI_ESUCCESS
            }
            None => WASI_EBADF,
        }
    })
}

/// `wasi_snapshot_preview1.fd_write` — wasm type
/// `(func (param i32 i64 i64 i64) (result i32))`.
///
/// `(fd, iovecs, iovecs_len, nwritten *)`. Writes every vector, in order, and
/// reports the total. `nwritten` is a `u64` because the module is wasm64.
#[no_mangle]
pub unsafe extern "C" fn fd_write(fd: i32, iovs: *const u8, iovs_len: u32, written: *mut u8) -> i32 {
    let vectors = match iovecs(iovs, iovs_len) {
        Some(vectors) => vectors,
        None => return WASI_EFAULT,
    };
    let mut total: u64 = 0;
    for (offset, len) in vectors {
        if len == 0 {
            continue;
        }
        let data = match mem::bytes(offset as *const u8, len) {
            Some(data) => data,
            None => return WASI_EFAULT,
        };
        let result = fd::with(|table| match table.get_mut(fd) {
            Some(entry) => fd::write_entry(entry, data),
            None => Err(fd::EBADF),
        });
        match result {
            Ok(written) => total += written as u64,
            Err(errno) => return errno_to_wasi(errno),
        }
    }
    if !mem::write_u64(written, total) {
        return WASI_EFAULT;
    }
    WASI_ESUCCESS
}

/// `wasi_snapshot_preview1.fd_read` — wasm type
/// `(func (param i32 i64 i64 i64) (result i32))`.
///
/// `(fd, iovecs, iovecs_len, nread *)`. Fills vectors in order, stopping at the
/// first short read — the same contract `readv` gives the engine's file loader.
#[no_mangle]
pub unsafe extern "C" fn fd_read(fd: i32, iovs: *const u8, iovs_len: u32, read_out: *mut u8) -> i32 {
    let vectors = match iovecs(iovs, iovs_len) {
        Some(vectors) => vectors,
        None => return WASI_EFAULT,
    };
    let mut total: u64 = 0;
    for (offset, len) in vectors {
        if len == 0 {
            continue;
        }
        let buffer = match mem::bytes_mut(offset as *mut u8, len) {
            Some(buffer) => buffer,
            None => return WASI_EFAULT,
        };
        let result = fd::with(|table| match table.get_mut(fd) {
            Some(entry) => fd::read_entry(entry, buffer),
            None => Err(fd::EBADF),
        });
        match result {
            Ok(read) => {
                total += read as u64;
                if read < len {
                    break;
                }
            }
            Err(errno) => return errno_to_wasi(errno),
        }
    }
    if !mem::write_u64(read_out, total) {
        return WASI_EFAULT;
    }
    WASI_ESUCCESS
}

/// Shared body of `fd_seek`, `fd_pread` and `fd_pwrite`.
fn whence_arg(whence: u32) -> Option<i32> {
    match whence {
        0 => Some(0), // SET
        1 => Some(1), // CUR
        2 => Some(2), // END
        _ => None,
    }
}

/// `wasi_snapshot_preview1.fd_seek` — wasm type
/// `(func (param i32 i64 i32 i64) (result i32))`.
///
/// `(fd, offset, whence, newoffset *)`. Reports the resulting absolute offset,
/// which the engine's `fseek`/`ftell` pair depends on. A socket or a stream
/// answers `ESPIPE`, exactly like the OS.
#[no_mangle]
pub unsafe extern "C" fn fd_seek(fd: i32, offset: i64, whence: u32, new_offset: *mut u8) -> i32 {
    let whence = match whence_arg(whence) {
        Some(whence) => whence,
        None => return WASI_EINVAL,
    };
    let result = fd::with(|table| match table.get_mut(fd) {
        Some(entry) => fd::seek_entry(entry, offset, whence),
        None => Err(fd::EBADF),
    });
    match result {
        Ok(position) => {
            if !mem::write_u64(new_offset, position) {
                return WASI_EFAULT;
            }
            WASI_ESUCCESS
        }
        Err(fd::ESPIPE) => WASI_ESPIPE,
        Err(errno) => errno_to_wasi(errno),
    }
}

/// `wasi_snapshot_preview1.fd_pread` — wasm type
/// `(func (param i32 i64 i64 i64 i64) (result i32))`.
///
/// `(fd, iovecs, iovecs_len, offset, nread *)`. Positional read that must not move
/// the file cursor: the engine's asset loader preads and then reads headers, and a
/// moved cursor would corrupt the next read. Implemented by saving and restoring
/// the stream position around the read.
#[no_mangle]
pub unsafe extern "C" fn fd_pread(
    fd: i32,
    iovs: *const u8,
    iovs_len: u32,
    offset: u64,
    read_out: *mut u8,
) -> i32 {
    let vectors = match iovecs(iovs, iovs_len) {
        Some(vectors) => vectors,
        None => return WASI_EFAULT,
    };
    let mut total: u64 = 0;
    let mut cursor = offset;
    for (target, len) in vectors {
        if len == 0 {
            continue;
        }
        let buffer = match mem::bytes_mut(target as *mut u8, len) {
            Some(buffer) => buffer,
            None => return WASI_EFAULT,
        };
        let result = fd::with(|table| match table.get_mut(fd) {
            Some(Entry::File { file, .. }) => {
                let saved = file.stream_position().unwrap_or(0);
                let read = file
                    .seek(std::io::SeekFrom::Start(cursor))
                    .and_then(|_| file.read(buffer));
                let _ = file.seek(std::io::SeekFrom::Start(saved));
                read
            }
            Some(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "pread on a non-file descriptor",
            )),
            None => Err(std::io::Error::from(std::io::ErrorKind::NotConnected)),
        });
        match result {
            Ok(read) => {
                total += read as u64;
                cursor += read as u64;
                if read < len {
                    break;
                }
            }
            Err(error) => return errno_to_wasi(fd::errno_of(&error)),
        }
    }
    if !mem::write_u64(read_out, total) {
        return WASI_EFAULT;
    }
    WASI_ESUCCESS
}

/// `wasi_snapshot_preview1.fd_pwrite` — wasm type
/// `(func (param i32 i64 i64 i64 i64) (result i32))`.
///
/// `(fd, iovecs, iovecs_len, offset, nwritten *)`. Positional write with the same
/// save/restore discipline as [`fd_pread`]; used by the save writer.
#[no_mangle]
pub unsafe extern "C" fn fd_pwrite(
    fd: i32,
    iovs: *const u8,
    iovs_len: u32,
    offset: u64,
    written: *mut u8,
) -> i32 {
    let vectors = match iovecs(iovs, iovs_len) {
        Some(vectors) => vectors,
        None => return WASI_EFAULT,
    };
    let mut total: u64 = 0;
    let mut cursor = offset;
    for (source, len) in vectors {
        if len == 0 {
            continue;
        }
        let data = match mem::bytes(source as *const u8, len) {
            Some(data) => data,
            None => return WASI_EFAULT,
        };
        let result = fd::with(|table| match table.get_mut(fd) {
            Some(Entry::File { file, .. }) => {
                let saved = file.stream_position().unwrap_or(0);
                let written = file
                    .seek(std::io::SeekFrom::Start(cursor))
                    .and_then(|_| file.write_all(data))
                    .map(|()| data.len());
                let _ = file.seek(std::io::SeekFrom::Start(saved));
                written
            }
            Some(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "pwrite on a non-file descriptor",
            )),
            None => Err(std::io::Error::from(std::io::ErrorKind::NotConnected)),
        });
        match result {
            Ok(written_now) => {
                total += written_now as u64;
                cursor += written_now as u64;
            }
            Err(error) => return errno_to_wasi(fd::errno_of(&error)),
        }
    }
    if !mem::write_u64(written, total) {
        return WASI_EFAULT;
    }
    WASI_ESUCCESS
}

/// `wasi_snapshot_preview1.environ_sizes_get` — wasm type
/// `(func (param i64 i64) (result i32))`.
///
/// `(count *, buf_size *)`: the number of environment variables and the bytes
/// their NUL-separated `KEY=VALUE` strings need. Answers from `std::env::vars`,
/// so a server's own environment is what the engine's `getenv` sees (that is how
/// a dedicated server gets its launch options in the original build too).
#[no_mangle]
pub unsafe extern "C" fn environ_sizes_get(count: *mut u8, buf_size: *mut u8) -> i32 {
    let pairs: Vec<(String, String)> = std::env::vars().collect();
    let mut bytes: u64 = 0;
    for (key, value) in &pairs {
        bytes += (key.len() + value.len() + 2) as u64;
    }
    if !mem::write_u64(count, pairs.len() as u64) || !mem::write_u64(buf_size, bytes) {
        return WASI_EFAULT;
    }
    WASI_ESUCCESS
}

/// `wasi_snapshot_preview1.environ_get` — wasm type
/// `(func (param i64 i64) (result i32))`.
///
/// `(char **environ, char *buf)`: writes one `u64` offset per variable into the
/// pointer array and the `KEY=VALUE\0` strings into `buf`. Offsets are linear
/// memory offsets, not host addresses — the engine dereferences them itself.
#[no_mangle]
pub unsafe extern "C" fn environ_get(environ: *mut u8, buf: *mut u8) -> i32 {
    let pairs: Vec<(String, String)> = std::env::vars().collect();
    let pointer_array = match pairs.len().checked_mul(8) {
        Some(size) => size,
        None => return WASI_EFAULT,
    };
    if !pairs.is_empty() && mem::bytes(environ, pointer_array).is_none() {
        return WASI_EFAULT;
    }
    let mut cursor = buf as usize;
    for (index, (key, value)) in pairs.iter().enumerate() {
        let entry = format!("{key}={value}\0");
        let bytes = entry.as_bytes();
        if !mem::write_bytes(cursor as *mut u8, bytes) {
            return WASI_EFAULT;
        }
        // The array holds linear-memory offsets, not host addresses: `cursor` is
        // already an offset into the engine's memory.
        if !mem::write_u64(environ.wrapping_add(index * 8), cursor as u64) {
            return WASI_EFAULT;
        }
        cursor = cursor.wrapping_add(bytes.len());
    }
    WASI_ESUCCESS
}

/// Translate a POSIX errno into a WASI errno.
pub fn errno_to_wasi(errno: i32) -> i32 {
    match errno {
        fd::EBADF => WASI_EBADF,
        fd::EINVAL => WASI_EINVAL,
        fd::ENOSYS | fd::EOPNOTSUPP => WASI_ENOTSUP,
        fd::ESPIPE => WASI_ESPIPE,
        fd::EFAULT => WASI_EFAULT,
        _ => WASI_EIO,
    }
}

// ---------------------------------------------------------------------------
// Aliases
// ---------------------------------------------------------------------------

/// The wat's local symbol name for `proc_exit` (`$__wasi_proc_exit`).
#[no_mangle]
pub extern "C" fn __wasi_proc_exit(code: u32) {
    proc_exit(code);
}

/// The wat's local symbol name for `fd_close`.
#[no_mangle]
pub unsafe extern "C" fn __wasi_fd_close(fd: i32) -> i32 {
    // SAFETY: forwards the descriptor; the callee validates it.
    unsafe { fd_close(fd) }
}

/// The wat's local symbol name for `fd_write`.
#[no_mangle]
pub unsafe extern "C" fn __wasi_fd_write(
    fd: i32,
    iovs: *const u8,
    iovs_len: u32,
    written: *mut u8,
) -> i32 {
    // SAFETY: forwards the caller's pointers to the range-checked implementation.
    unsafe {
        fd_write(fd, iovs, iovs_len, written)
    }
}

/// The wat's local symbol name for `fd_read`.
#[no_mangle]
pub unsafe extern "C" fn __wasi_fd_read(
    fd: i32,
    iovs: *const u8,
    iovs_len: u32,
    read_out: *mut u8,
) -> i32 {
    // SAFETY: forwards the caller's pointers to the range-checked implementation.
    unsafe {
        fd_read(fd, iovs, iovs_len, read_out)
    }
}

/// The wat's local symbol name for `fd_seek`.
#[no_mangle]
pub unsafe extern "C" fn __wasi_fd_seek(
    fd: i32,
    offset: i64,
    whence: u32,
    new_offset: *mut u8,
) -> i32 {
    // SAFETY: forwards the caller's pointers to the range-checked implementation.
    unsafe {
        fd_seek(fd, offset, whence, new_offset)
    }
}

/// The wat's local symbol name for `fd_pread`.
#[no_mangle]
pub unsafe extern "C" fn __wasi_fd_pread(
    fd: i32,
    iovs: *const u8,
    iovs_len: u32,
    offset: u64,
    read_out: *mut u8,
) -> i32 {
    // SAFETY: forwards the caller's pointers to the range-checked implementation.
    unsafe {
        fd_pread(fd, iovs, iovs_len, offset, read_out)
    }
}

/// The wat's local symbol name for `fd_pwrite`.
#[no_mangle]
pub unsafe extern "C" fn __wasi_fd_pwrite(
    fd: i32,
    iovs: *const u8,
    iovs_len: u32,
    offset: u64,
    written: *mut u8,
) -> i32 {
    // SAFETY: forwards the caller's pointers to the range-checked implementation.
    unsafe {
        fd_pwrite(fd, iovs, iovs_len, offset, written)
    }
}

/// The wat's local symbol name for `environ_sizes_get`.
#[no_mangle]
pub unsafe extern "C" fn __wasi_environ_sizes_get(count: *mut u8, buf_size: *mut u8) -> i32 {
    // SAFETY: forwards the caller's out pointers; the callee range-checks them.
    unsafe { environ_sizes_get(count, buf_size) }
}

/// The wat's local symbol name for `environ_get`.
#[no_mangle]
pub unsafe extern "C" fn __wasi_environ_get(environ: *mut u8, buf: *mut u8) -> i32 {
    // SAFETY: forwards the caller's pointers to the range-checked implementation.
    unsafe {
        environ_get(environ, buf)
    }
}