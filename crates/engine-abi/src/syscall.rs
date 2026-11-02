//! `env.__syscall_*` — the emscripten syscall layer.
//!
//! The engine is C++ over libc: every `fopen`, `stat`, `opendir`, `socket` and
//! `recv` in the game ends up here. In the browser these were implemented in
//! `library_syscall.js` over `MEMFS` and, for sockets, over raw `WebSocket`/TCP
//! shims; on a server they map to real `std::fs` and `std::net` calls.
//!
//! ## Signature convention
//!
//! emscripten declares the syscall imports with an over-wide signature (the JS
//! side read `arguments`), so several of them carry more parameters than the
//! underlying call takes — `__syscall_socket` is `(i32 i32 i32 i32 i32 i32)` but
//! only the first three are `domain`, `type`, `protocol`. Every shim documents
//! which parameters it consumes; the rest are ignored, exactly as the JS import
//! ignored them.
//!
//! ## Return convention
//!
//! Success returns the value the C wrapper expects (a descriptor, a byte count,
//! a pointer-free length); failure returns `-errno` with Linux generic numbering.
//! Nothing panics: a bad pointer is `-EFAULT`, a closed descriptor `-EBADF`.

use std::io::Write;
use std::net::{Shutdown, TcpStream, UdpSocket};
use std::time::{Duration, Instant};

use crate::fd::{self, Entry};
use crate::mem;
use crate::state;

/// A pointer the engine passed that does not resolve inside linear memory.
const EFAULT: i32 = fd::EFAULT;

/// Read a NUL-terminated path from linear memory.
fn path_arg(ptr: *const u8) -> Option<&'static str> {
    mem::cstr(ptr)
}

/// `env.__syscall_fcntl64` — wasm type `(func (param i32 i32 i64) (result i32))`.
///
/// `(fd, cmd, arg)`. The engine only uses the descriptor-flag subset:
/// `F_GETFD`/`F_SETFD`/`F_GETFL`/`F_SETFL` answer `0` (nothing to report, nothing
/// to change — the wasm build had no `O_NONBLOCK` state to keep per descriptor
/// because every pipe was a `MEMFS` file). `F_DUPFD` is not implemented and
/// answers `-EINVAL`, which is what the engine's libc does when it probes for
/// `dup` support in the wasm build.
#[no_mangle]
pub unsafe extern "C" fn __syscall_fcntl64(fd: i32, cmd: i32, _arg: u64) -> i32 {
    const F_GETFD: i32 = 1;
    const F_SETFD: i32 = 2;
    const F_GETFL: i32 = 3;
    const F_SETFL: i32 = 4;
    let known = fd::with(|table| table.get(fd).is_some());
    if !known && fd > 2 {
        return -fd::EBADF;
    }
    match cmd {
        F_GETFD | F_SETFD | F_GETFL | F_SETFL => 0,
        _ => -fd::EINVAL,
    }
}

/// `env.__syscall_openat` — wasm type `(func (param i32 i64 i32 i64) (result i32))`.
///
/// `(dirfd, path, flags, varargs)`: `varargs` carries the mode, which the
/// descriptor-backed store ignores (the host's `OpenOptions` decides). `dirfd` is
/// ignored for the same reason it was in the browser build: the engine's virtual
/// filesystem is a single root, so `openat(AT_FDCWD, …)` and `openat(dir, …)`
/// resolve identically.
#[no_mangle]
pub unsafe extern "C" fn __syscall_openat(_dirfd: i32, path: *const u8, flags: u32, _mode: u64) -> i32 {
    let path = match path_arg(path) {
        Some(path) => path,
        None => return -EFAULT,
    };
    fd::open_path(path, flags as i32)
}

/// `env.__syscall_ioctl` — wasm type `(func (param i32 i32 i64) (result i32))`.
///
/// `(fd, request, arg)`. Nothing in the module needs a working ioctl: libc probes
/// `TCGETS`/`TIOCGWINSZ` to decide whether a stream is a terminal, and answering
/// `-ENOTTY` is the correct answer for a file, a socket and a redirected stream.
/// That makes "not a tty" the documented result rather than a failure.
#[no_mangle]
pub unsafe extern "C" fn __syscall_ioctl(fd: i32, _request: i32, _arg: u64) -> i32 {
    let known = fd::with(|table| table.get(fd).is_some());
    if !known && fd > 2 {
        return -fd::EBADF;
    }
    -fd::ENOTTY
}

/// `env.__syscall_fstat64` — wasm type `(func (param i32 i64) (result i32))`.
///
/// `(fd, struct stat *)`. Files and the standard streams answer from
/// `std::fs::metadata` / a synthetic stream record; sockets report `S_IFSOCK`.
#[no_mangle]
pub unsafe extern "C" fn __syscall_fstat64(fd: i32, stat_ptr: *mut u8) -> i32 {
    let buffer = match mem::bytes_mut(stat_ptr, fd::STAT_LEN) {
        Some(buffer) => buffer,
        None => return -EFAULT,
    };
    fd::with(|table| match table.get(fd) {
        Some(Entry::File { file, .. }) => match file.metadata() {
            Ok(meta) => {
                if fd::encode_stat(&meta, buffer) {
                    0
                } else {
                    -fd::EINVAL
                }
            }
            Err(e) => -fd::errno_of(&e),
        },
        Some(Entry::Stdin) | Some(Entry::Stdout) | Some(Entry::Stderr) => {
            if fd::encode_stat_socket(buffer) {
                0
            } else {
                -fd::EINVAL
            }
        }
        // Sockets and directories get the synthetic record: `S_IFSOCK` with no
        // size is what the engine's libc checks for before reading.
        Some(_) => {
            if fd::encode_stat_socket(buffer) {
                0
            } else {
                -fd::EINVAL
            }
        }
        None => -fd::EBADF,
    })
}

/// Shared body of `stat64`, `lstat64` and `newfstatat`.
unsafe fn stat_path(path: *const u8, stat_ptr: *mut u8, follow: bool) -> i32 {
    let path = match path_arg(path) {
        Some(path) => path,
        None => return -EFAULT,
    };
    let host_path = fd::resolve(path);
    let meta = if follow {
        std::fs::metadata(&host_path)
    } else {
        std::fs::symlink_metadata(&host_path)
    };
    match meta {
        Ok(meta) => match mem::bytes_mut(stat_ptr, fd::STAT_LEN) {
            Some(buffer) => {
                if fd::encode_stat(&meta, buffer) {
                    0
                } else {
                    -fd::EINVAL
                }
            }
            None => -EFAULT,
        },
        Err(e) => -fd::errno_of(&e),
    }
}

/// `env.__syscall_stat64` — wasm type `(func (param i64 i64) (result i32))`.
///
/// `(path, struct stat *)`, following symlinks.
#[no_mangle]
pub unsafe extern "C" fn __syscall_stat64(path: *const u8, stat_ptr: *mut u8) -> i32 {
    // SAFETY: forwards the caller's pointers; `stat_path` range-checks them.
    unsafe { stat_path(path, stat_ptr, true) }
}

/// `env.__syscall_lstat64` — wasm type `(func (param i64 i64) (result i32))`.
///
/// `(path, struct stat *)`, *not* following the final symlink.
#[no_mangle]
pub unsafe extern "C" fn __syscall_lstat64(path: *const u8, stat_ptr: *mut u8) -> i32 {
    // SAFETY: as `__syscall_stat64`, without following the final symlink.
    unsafe { stat_path(path, stat_ptr, false) }
}

/// `env.__syscall_newfstatat` — wasm type
/// `(func (param i32 i64 i64 i32) (result i32))`.
///
/// `(dirfd, path, struct stat *, flags)`. `AT_SYMLINK_NOFOLLOW` (`0x100`) is
/// honoured; `AT_EMPTY_PATH` (`0x1000`) with an empty path reports the descriptor
/// itself, which is how libc implements `fstatat(fd, "", …)`.
#[no_mangle]
pub unsafe extern "C" fn __syscall_newfstatat(
    dirfd: i32,
    path: *const u8,
    stat_ptr: *mut u8,
    flags: i32,
) -> i32 {
    const AT_SYMLINK_NOFOLLOW: i32 = 0x100;
    const AT_EMPTY_PATH: i32 = 0x1000;
    let text = match path_arg(path) {
        Some(text) => text,
        None => return -EFAULT,
    };
    if text.is_empty() && flags & AT_EMPTY_PATH != 0 {
        // SAFETY: `stat_ptr` is the caller's `struct stat *`.
        return unsafe { __syscall_fstat64(dirfd, stat_ptr) };
    }
    // SAFETY: forwards the caller's pointers; `stat_path` range-checks them.
    unsafe { stat_path(path, stat_ptr, flags & AT_SYMLINK_NOFOLLOW == 0) }
}

/// `env.__syscall_ftruncate64` — wasm type `(func (param i32 i64) (result i32))`.
///
/// `(fd, length)`. Resizes the file through `File::set_len`, which is what the
/// engine's save writer relies on before rewriting a blob.
#[no_mangle]
pub unsafe extern "C" fn __syscall_ftruncate64(fd: i32, length: u64) -> i32 {
    fd::with(|table| match table.get(fd) {
        Some(Entry::File { file, .. }) => match file.set_len(length) {
            Ok(()) => 0,
            Err(e) => -fd::errno_of(&e),
        },
        Some(_) => -fd::EINVAL,
        None => -fd::EBADF,
    })
}

/// `env.__syscall_getcwd` — wasm type `(func (param i64 i64) (result i32))`.
///
/// `(buf, size)` → number of bytes written including the NUL, or `-ERANGE`-ish
/// `-EINVAL` when the buffer is too small. The answer is
/// [`state::fs_root`] resolved to an absolute path where possible: that is the
/// engine's working directory now.
#[no_mangle]
pub unsafe extern "C" fn __syscall_getcwd(buf: *mut u8, size: u64) -> i32 {
    let cwd = std::env::current_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| state::fs_root().to_string_lossy().into_owned());
    let bytes = cwd.as_bytes();
    let needed = bytes.len() + 1;
    if (size as usize) < needed {
        return -fd::EINVAL;
    }
    if !mem::write_bytes(buf, bytes) || !mem::write_bytes(buf.wrapping_add(bytes.len()), &[0]) {
        return -EFAULT;
    }
    needed as i32
}

/// `env.__syscall_mkdirat` — wasm type `(func (param i32 i64 i32) (result i32))`.
///
/// `(dirfd, path, mode)`: `create_dir_all`, so a save directory can be created
/// with its parents in one call (the engine does exactly that on first save).
#[no_mangle]
pub unsafe extern "C" fn __syscall_mkdirat(_dirfd: i32, path: *const u8, _mode: u32) -> i32 {
    let path = match path_arg(path) {
        Some(path) => path,
        None => return -EFAULT,
    };
    match std::fs::create_dir_all(fd::resolve(path)) {
        Ok(()) => 0,
        Err(e) => -fd::errno_of(&e),
    }
}

/// `env.__syscall_unlinkat` — wasm type `(func (param i32 i64 i32) (result i32))`.
///
/// `(dirfd, path, flags)`. Removes a file; `AT_REMOVEDIR` (`0x200`) removes an
/// empty directory instead.
#[no_mangle]
pub unsafe extern "C" fn __syscall_unlinkat(_dirfd: i32, path: *const u8, flags: i32) -> i32 {
    const AT_REMOVEDIR: i32 = 0x200;
    let path = match path_arg(path) {
        Some(path) => path,
        None => return -EFAULT,
    };
    let host_path = fd::resolve(path);
    let result = if flags & AT_REMOVEDIR != 0 {
        std::fs::remove_dir(&host_path)
    } else {
        std::fs::remove_file(&host_path)
    };
    match result {
        Ok(()) => 0,
        Err(e) => -fd::errno_of(&e),
    }
}

/// `env.__syscall_rmdir` — wasm type `(func (param i64) (result i32))`.
///
/// `(path)`: remove an empty directory (the pre-`unlinkat` form libc still calls
/// on some paths).
#[no_mangle]
pub unsafe extern "C" fn __syscall_rmdir(path: *const u8) -> i32 {
    let path = match path_arg(path) {
        Some(path) => path,
        None => return -EFAULT,
    };
    match std::fs::remove_dir(fd::resolve(path)) {
        Ok(()) => 0,
        Err(e) => -fd::errno_of(&e),
    }
}

/// `env.__syscall_renameat` — wasm type
/// `(func (param i32 i64 i32 i64) (result i32))`.
///
/// `(olddirfd, oldpath, newdirfd, newpath)`. A cross-device rename fails with the
/// host's errno; the engine's save path renames within one directory, so
/// `std::fs::rename` semantics are exactly what it needs (atomic on both Linux
/// and Windows for same-volume renames).
#[no_mangle]
pub unsafe extern "C" fn __syscall_renameat(
    _old_dirfd: i32,
    old_path: *const u8,
    _new_dirfd: i32,
    new_path: *const u8,
) -> i32 {
    let old = match path_arg(old_path) {
        Some(path) => path,
        None => return -EFAULT,
    };
    let new = match path_arg(new_path) {
        Some(path) => path,
        None => return -EFAULT,
    };
    match std::fs::rename(fd::resolve(old), fd::resolve(new)) {
        Ok(()) => 0,
        Err(e) => -fd::errno_of(&e),
    }
}

/// `env.__syscall_getdents64` — wasm type
/// `(func (param i32 i64 i64) (result i32))`.
///
/// `(fd, buf, len)` → bytes written. The listing is a snapshot taken at `open`
/// time, so repeated calls walk `.`/`..` then the entries in host order and
/// return `0` at the end, which is what `readdir` loops expect.
#[no_mangle]
pub unsafe extern "C" fn __syscall_getdents64(fd: i32, buf: *mut u8, len: u64) -> i32 {
    let len = match usize::try_from(len) {
        Ok(len) => len,
        Err(_) => return -fd::EINVAL,
    };
    let out = match mem::bytes_mut(buf, len) {
        Some(out) => out,
        None => return -EFAULT,
    };
    fd::with(|table| match table.get_mut(fd) {
        Some(Entry::Dir(cursor)) => {
            let (written, used) = fd::encode_dirents(&cursor.entries, cursor.pos, out);
            cursor.pos += used;
            written as i32
        }
        Some(_) => -fd::ENOTDIR,
        None => -fd::EBADF,
    })
}

/// `env.__syscall_statfs64` — wasm type
/// `(func (param i64 i64 i64) (result i32))`.
///
/// `(path, len, buf)`. The engine calls this from libc's `statvfs` shim to size
/// its streaming buffers. The server answers a synthetic record (4 KiB blocks,
/// no free-space accounting: `f_blocks`/`f_bfree` are reported as 0 so no caller
/// divides by zero, and `f_namelen` = 255). Field layout follows emscripten's
/// wasm64 `struct statfs`: `f_type@0, f_bsize@8, f_blocks@16, f_bfree@24,
/// f_bavail@32, f_files@40, f_ffree@48, f_fsid@56 (2×i32), f_namelen@64,
/// f_frsize@72, f_flags@80, spare@88..`. All `i64`.
#[no_mangle]
pub unsafe extern "C" fn __syscall_statfs64(_path: *const u8, len: u64, buf: *mut u8) -> i32 {
    const STATFS_LEN: usize = 120;
    if (len as usize) < STATFS_LEN {
        return -fd::EINVAL;
    }
    let out = match mem::bytes_mut(buf, STATFS_LEN) {
        Some(out) => out,
        None => return -EFAULT,
    };
    out.fill(0);
    let mut put = |offset: usize, value: i64| {
        out[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
    };
    put(0, 0x0102_1994); // f_type: an arbitrary non-zero magic
    put(8, 4096); // f_bsize
    put(16, 0); // f_blocks
    put(24, 0); // f_bfree
    put(32, 0); // f_bavail
    put(40, 0); // f_files
    put(48, 0); // f_ffree
    put(64, 255); // f_namelen
    put(72, 4096); // f_frsize
    0
}

/// `env.__syscall_poll` — wasm type `(func (param i64 i32 i32) (result i32))`.
///
/// `(pollfd *, nfds, timeout_ms)` → number of descriptors with a non-zero
/// `revents`. `struct pollfd` is `{ fd: i32, events: i16, revents: i16 }`
/// (8 bytes).
///
/// Readiness is computed per descriptor: files and the standard streams are
/// always ready, UDP and TCP sockets are probed with a non-blocking `peek` (so
/// no data is consumed), and a descriptor that is not open reports `POLLNVAL`.
/// `POLLOUT` is always granted for a connected socket — the host has no partial
/// send queue. A positive timeout spins in 1 ms steps; a negative timeout blocks
/// until something becomes ready, which is what a blocking `poll(-1)` must do.
#[no_mangle]
pub unsafe extern "C" fn __syscall_poll(fds: *mut u8, nfds: u32, timeout_ms: i32) -> i32 {
    const POLLIN: i16 = 0x001;
    const POLLOUT: i16 = 0x004;
    const POLLERR: i16 = 0x008;
    const POLLHUP: i16 = 0x010;
    const POLLNVAL: i16 = 0x020;
    const POLLFD_LEN: usize = 8;

    let count = nfds as usize;
    let total = match count.checked_mul(POLLFD_LEN) {
        Some(total) => total,
        None => return -fd::EINVAL,
    };
    if count > 0 && mem::bytes(fds, total).is_none() {
        return -EFAULT;
    }
    let deadline = if timeout_ms >= 0 {
        Some(Instant::now() + Duration::from_millis(timeout_ms as u64))
    } else {
        None
    };
    loop {
        let mut ready = 0;
        for index in 0..count {
            let base = fds.wrapping_add(index * POLLFD_LEN);
            let fd = match mem::read_u32(base as *const u8) {
                Some(fd) => fd as i32,
                None => return -EFAULT,
            };
            let events = match mem::bytes(base.wrapping_add(4), 2) {
                Some(raw) => i16::from_le_bytes([raw[0], raw[1]]),
                None => return -EFAULT,
            };
            let revents = revents_for(fd, events, POLLIN, POLLOUT, POLLNVAL, POLLHUP, POLLERR);
            if !mem::write_bytes(
                base.wrapping_add(6),
                &revents.to_le_bytes(),
            ) {
                return -EFAULT;
            }
            if revents != 0 {
                ready += 1;
            }
        }
        if ready > 0 {
            return ready;
        }
        match deadline {
            Some(deadline) if Instant::now() >= deadline => return 0,
            Some(_) => std::thread::sleep(Duration::from_millis(1)),
            None => std::thread::sleep(Duration::from_millis(1)),
        }
    }
}

/// Compute `revents` for one `pollfd`.
fn revents_for(
    fd: i32,
    events: i16,
    pollin: i16,
    pollout: i16,
    pollnval: i16,
    pollhup: i16,
    pollerr: i16,
) -> i16 {
    fd::with(|table| {
        let entry = match table.get(fd) {
            Some(entry) => entry,
            None => return pollnval,
        };
        match entry {
            Entry::File { .. } | Entry::Stdin | Entry::Stdout | Entry::Stderr | Entry::Dir(_) => {
                events & (pollin | pollout)
            }
            Entry::TcpPending => pollhup,
            Entry::Tcp(stream) => {
                let mut revents = events & pollout;
                if events & pollin != 0 {
                    match peek_tcp(stream) {
                        Ok(0) => revents |= pollin | pollhup,
                        Ok(_) => revents |= pollin,
                        Err(PeekError::WouldBlock) => {}
                        Err(PeekError::Failed) => revents |= pollerr,
                    }
                }
                revents
            }
            Entry::Udp(sock) => {
                let mut revents = events & pollout;
                if events & pollin != 0 {
                    match peek_udp(sock) {
                        Ok(0) => revents |= pollin,
                        Ok(_) => revents |= pollin,
                        Err(PeekError::WouldBlock) => {}
                        Err(PeekError::Failed) => revents |= pollerr,
                    }
                }
                revents
            }
            Entry::Listener(_) => events & pollin,
        }
    })
}

/// Why a non-consuming peek did not produce bytes.
enum PeekError {
    /// Nothing available yet.
    WouldBlock,
    /// The socket is broken.
    Failed,
}

/// Peek at a TCP stream without consuming, restoring the blocking mode.
fn peek_tcp(stream: &TcpStream) -> Result<usize, PeekError> {
    let was_nonblocking = stream.set_nonblocking(true).is_err();
    if was_nonblocking {
        return Err(PeekError::Failed);
    }
    let mut probe = [0u8; 1];
    let result = stream.peek(&mut probe);
    let _ = stream.set_nonblocking(false);
    match result {
        Ok(n) => Ok(n),
        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Err(PeekError::WouldBlock),
        Err(_) => Err(PeekError::Failed),
    }
}

/// Peek at a UDP socket without consuming the datagram.
fn peek_udp(socket: &UdpSocket) -> Result<usize, PeekError> {
    if socket.set_nonblocking(true).is_err() {
        return Err(PeekError::Failed);
    }
    let mut probe = [0u8; 1];
    let result = socket.peek(&mut probe);
    let _ = socket.set_nonblocking(false);
    match result {
        Ok(n) => Ok(n),
        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Err(PeekError::WouldBlock),
        Err(_) => Err(PeekError::Failed),
    }
}

/// `env.__syscall_socket` — wasm type
/// `(func (param i32 i32 i32 i32 i32 i32) (result i32))`.
///
/// Consumes `(domain, type, protocol)`; the remaining three parameters are
/// emscripten's padding and are ignored. `SOCK_STREAM` creates a not-yet-connected
/// descriptor (the engine always `connect`s next), `SOCK_DGRAM` binds an ephemeral
/// local port immediately so `sendto`/`recvfrom` can work without a `bind` import.
#[no_mangle]
pub unsafe extern "C" fn __syscall_socket(
    domain: u32,
    sock_type: u32,
    _protocol: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
) -> i32 {
    let kind = sock_type & 0xf;
    const STREAM: u32 = fd::SOCK_STREAM as u32;
    const DGRAM: u32 = fd::SOCK_DGRAM as u32;
    match (domain, kind) {
        (fd::AF_INET | fd::AF_INET6, STREAM) => {
            fd::with(|table| match table.alloc(Entry::TcpPending) {
                Some(fd) => fd,
                None => -fd::EMFILE,
            })
        }
        (fd::AF_INET, DGRAM) => match UdpSocket::bind("0.0.0.0:0") {
            Ok(socket) => fd::with(|table| match table.alloc(Entry::Udp(socket)) {
                Some(fd) => fd,
                None => -fd::EMFILE,
            }),
            Err(e) => -fd::errno_of(&e),
        },
        (fd::AF_INET6, DGRAM) => match UdpSocket::bind("[::]:0") {
            Ok(socket) => fd::with(|table| match table.alloc(Entry::Udp(socket)) {
                Some(fd) => fd,
                None => -fd::EMFILE,
            }),
            Err(e) => -fd::errno_of(&e),
        },
        _ => -fd::EOPNOTSUPP,
    }
}

/// `env.__syscall_connect` — wasm type
/// `(func (param i32 i64 i32 i32 i32 i32) (result i32))`.
///
/// Consumes `(fd, sockaddr *, addrlen)`; the trailing three are padding. A TCP
/// descriptor becomes a real `TcpStream` (blocking connect, so the engine's
/// synchronous socket code sees the same semantics it had against the JS shim).
/// An already-connected socket answers `-EISCONN`, a UDP socket `-EOPNOTSUPP`
/// (UDP has no connection state in this shim; use `sendto`).
#[no_mangle]
pub unsafe extern "C" fn __syscall_connect(
    sock_fd: i32,
    addr: *const u8,
    addr_len: u32,
    _pad0: i32,
    _pad1: i32,
    _pad2: i32,
) -> i32 {
    let raw = match mem::bytes(addr, addr_len as usize) {
        Some(raw) => raw,
        None => return -EFAULT,
    };
    let target = match fd::decode_sockaddr(raw) {
        Some(target) => target,
        None => return -fd::EINVAL,
    };
    let pending = fd::with(|table| matches!(table.get(sock_fd), Some(Entry::TcpPending)));
    if !pending {
        return fd::with(|table| match table.get(sock_fd) {
            Some(Entry::Tcp(_)) => -106, // EISCONN
            Some(Entry::Udp(_)) => -fd::EOPNOTSUPP,
            Some(_) => -fd::ENOTSOCK,
            None => -fd::EBADF,
        });
    }
    match TcpStream::connect(target) {
        Ok(stream) => {
            let _ = stream.set_nodelay(true);
            fd::with(|table| {
                table.insert_at(sock_fd, Entry::Tcp(stream));
            });
            0
        }
        Err(e) => {
            // A failed connect leaves the descriptor as it was; libc's caller
            // closes it either way.
            -fd::errno_of(&e)
        }
    }
}

/// `env.__syscall_accept4` — wasm type
/// `(func (param i32 i64 i64 i32 i32 i32) (result i32))`.
///
/// Consumes `(fd, sockaddr *, addrlen *, flags)`. **Not reached by this module**:
/// no `bind`/`listen` import exists, so the engine can never produce a listening
/// socket (it is a client in the original build). The server answers `-EINVAL`
/// rather than pretending to accept.
#[no_mangle]
pub unsafe extern "C" fn __syscall_accept4(
    _sock_fd: i32,
    _addr: *mut u8,
    _addr_len: *mut u8,
    _flags: i32,
    _pad0: i32,
    _pad1: i32,
) -> i32 {
    -fd::EINVAL
}

/// `env.__syscall_getsockname` — wasm type
/// `(func (param i32 i64 i64 i32 i32 i32) (result i32))`.
///
/// Consumes `(fd, sockaddr *, addrlen *)`: writes the local address and the
/// number of bytes written. UDP and connected TCP sockets answer from
/// `local_addr()`; anything else is `-ENOTSOCK`.
#[no_mangle]
pub unsafe extern "C" fn __syscall_getsockname(
    sock_fd: i32,
    addr: *mut u8,
    addr_len: *mut u8,
    _pad0: i32,
    _pad1: i32,
    _pad2: i32,
) -> i32 {
    let local = fd::with(|table| match table.get(sock_fd) {
        Some(Entry::Tcp(stream)) => stream.local_addr().map_err(|e| fd::errno_of(&e)),
        Some(Entry::Udp(socket)) => socket.local_addr().map_err(|e| fd::errno_of(&e)),
        Some(Entry::Listener(listener)) => listener.local_addr().map_err(|e| fd::errno_of(&e)),
        Some(Entry::TcpPending) => Err(107), // ENOTCONN
        Some(_) => Err(fd::ENOTSOCK),
        None => Err(fd::EBADF),
    });
    let local = match local {
        Ok(local) => local,
        Err(errno) => return -errno,
    };
    let mut raw = [0u8; 28];
    let written = fd::encode_sockaddr(local, &mut raw);
    if !mem::write_bytes(addr, &raw[..written]) {
        return -EFAULT;
    }
    if !mem::write_u32(addr_len, written as u32) {
        return -EFAULT;
    }
    0
}

/// `env.__syscall_getsockopt` — wasm type
/// `(func (param i32 i32 i32 i64 i64 i32) (result i32))`.
///
/// Consumes `(fd, level, optname, optval *, optlen *)`. The engine's only probe is
/// `SO_ERROR`, which is always `0` here because a failed `connect` is reported
/// synchronously through its return value. Any other option answers
/// `-EOPNOTSUPP`.
#[no_mangle]
pub unsafe extern "C" fn __syscall_getsockopt(
    sock_fd: i32,
    _level: i32,
    optname: i32,
    optval: *mut u8,
    optlen: *mut u8,
    _pad: i32,
) -> i32 {
    const SO_ERROR: i32 = 4;
    let is_socket = fd::with(|table| table.get(sock_fd).map(Entry::is_socket).unwrap_or(false));
    if !is_socket {
        return -fd::ENOTSOCK;
    }
    if optname != SO_ERROR {
        return -fd::EOPNOTSUPP;
    }
    if !mem::write_u32(optval, 0) || !mem::write_u32(optlen, 4) {
        return -EFAULT;
    }
    0
}

/// `env.__syscall_setsockopt` — wasm type
/// `(func (param i32 i32 i32 i64 i32 i32) (result i32))`.
///
/// Consumes `(fd, level, optname, optval *, optlen)`. Accepted and discarded:
/// `TCP_NODELAY`, `SO_KEEPALIVE` and `SO_RCVBUF` were hints to the browser
/// transport. The server applies `TCP_NODELAY` when it connects and keeps std's
/// defaults for the rest, so reporting success is truthful at the socket API
/// level (there is no option the engine can observe afterwards).
#[no_mangle]
pub unsafe extern "C" fn __syscall_setsockopt(
    sock_fd: i32,
    _level: i32,
    _optname: i32,
    _optval: *const u8,
    _optlen: u32,
    _pad: i32,
) -> i32 {
    let is_socket = fd::with(|table| table.get(sock_fd).map(Entry::is_socket).unwrap_or(false));
    if is_socket {
        0
    } else {
        -fd::ENOTSOCK
    }
}

/// `env.__syscall_sendto` — wasm type
/// `(func (param i32 i64 i64 i32 i64 i32) (result i32))`.
///
/// Consumes `(fd, buf, len, flags, sockaddr *, addrlen)` → bytes sent. On a
/// connected TCP stream the address is ignored (that is what `sendto` does on a
/// connected socket); on UDP the address is required.
#[no_mangle]
pub unsafe extern "C" fn __syscall_sendto(
    sock_fd: i32,
    buf: *const u8,
    len: u64,
    _flags: i32,
    addr: *const u8,
    addr_len: u32,
) -> i32 {
    let len = match usize::try_from(len) {
        Ok(len) => len,
        Err(_) => return -fd::EINVAL,
    };
    let payload = match mem::bytes(buf, len) {
        Some(payload) => payload,
        None => return -EFAULT,
    };
    if addr_len > 0 {
        if let Some(raw) = mem::bytes(addr, addr_len as usize) {
            if let Some(target) = fd::decode_sockaddr(raw) {
                let is_udp = fd::with(|table| matches!(table.get(sock_fd), Some(Entry::Udp(_))));
                if is_udp {
                    return fd::with(|table| match table.get(sock_fd) {
                        Some(Entry::Udp(socket)) => match socket.send_to(payload, target) {
                            Ok(sent) => sent as i32,
                            Err(e) => -fd::errno_of(&e),
                        },
                        Some(_) => -fd::ENOTSOCK,
                        None => -fd::EBADF,
                    });
                }
            }
        }
    }
    fd::with(|table| match table.get_mut(sock_fd) {
        Some(entry) => match fd::write_entry(entry, payload) {
            Ok(sent) => sent as i32,
            Err(errno) => -errno,
        },
        None => -fd::EBADF,
    })
}

/// `env.__syscall_recvfrom` — wasm type
/// `(func (param i32 i64 i64 i32 i64 i64) (result i32))`.
///
/// Consumes `(fd, buf, len, flags, sockaddr *src, addrlen *src_len)` → bytes
/// received, filling the source address for UDP. A TCP stream ignores the address
/// fields, matching `recvfrom` on a connected socket.
#[no_mangle]
pub unsafe extern "C" fn __syscall_recvfrom(
    sock_fd: i32,
    buf: *mut u8,
    len: u64,
    _flags: i32,
    addr: *mut u8,
    addr_len: *mut u8,
) -> i32 {
    let len = match usize::try_from(len) {
        Ok(len) => len,
        Err(_) => return -fd::EINVAL,
    };
    let out = match mem::bytes_mut(buf, len) {
        Some(out) => out,
        None => return -EFAULT,
    };
    let is_udp = fd::with(|table| matches!(table.get(sock_fd), Some(Entry::Udp(_))));
    if is_udp {
        return fd::with(|table| match table.get(sock_fd) {
            Some(Entry::Udp(socket)) => match socket.recv_from(out) {
                Ok((received, from)) => {
                    if !addr.is_null() {
                        let mut raw = [0u8; 28];
                        let written = fd::encode_sockaddr(from, &mut raw);
                        if mem::write_bytes(addr, &raw[..written])
                            && mem::write_u32(addr_len, written as u32)
                        {
                            return received as i32;
                        }
                        return -EFAULT;
                    }
                    received as i32
                }
                Err(e) => -fd::errno_of(&e),
            },
            Some(_) => -fd::ENOTSOCK,
            None => -fd::EBADF,
        });
    }
    fd::with(|table| match table.get_mut(sock_fd) {
        Some(entry) => match fd::read_entry(entry, out) {
            Ok(received) => received as i32,
            Err(errno) => -errno,
        },
        None => -fd::EBADF,
    })
}

/// `env.__syscall_shutdown` — wasm type
/// `(func (param i32 i32 i32 i32 i32 i32) (result i32))`.
///
/// Consumes `(fd, how)`. `SHUT_RD`/`SHUT_WR`/`SHUT_RDWR` map onto
/// `TcpStream::shutdown`; the extra parameters are emscripten padding.
#[no_mangle]
pub unsafe extern "C" fn __syscall_shutdown(
    sock_fd: i32,
    how: i32,
    _pad0: i32,
    _pad1: i32,
    _pad2: i32,
    _pad3: i32,
) -> i32 {
    let mode = match how {
        0 => Shutdown::Read,
        1 => Shutdown::Write,
        2 => Shutdown::Both,
        _ => return -fd::EINVAL,
    };
    fd::with(|table| match table.get(sock_fd) {
        Some(Entry::Tcp(stream)) => match stream.shutdown(mode) {
            Ok(()) => 0,
            Err(e) => -fd::errno_of(&e),
        },
        Some(entry) if entry.is_socket() => -fd::EOPNOTSUPP,
        Some(_) => -fd::ENOTSOCK,
        None => -fd::EBADF,
    })
}

/// Flush a descriptor's userspace buffers (used by `wasi.fd_close`).
pub(crate) fn flush_entry(entry: &mut Entry) {
    if let Entry::File { file, .. } = entry {
        let _ = file.flush();
    }
}