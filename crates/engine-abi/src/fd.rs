//! The emscripten file-descriptor table and the struct layouts the syscall
//! shims read and write.
//!
//! The engine is C++ compiled to wasm: it reaches the outside world through
//! libc (`fopen`, `read`, `stat`, `opendir`, `socket`, …), which reaches the host
//! through `env.__syscall_*` and `wasi_snapshot_preview1.*`. Both families need
//! one shared descriptor namespace, and that is this module.
//!
//! Descriptors 0–2 are the standard streams. Anything the engine opens gets the
//! next free number from 3 up, exactly like a POSIX process. Nothing here is
//! per-thread: file descriptors are process-wide in the engine too.
//!
//! ## Struct layouts
//!
//! The wasm module is wasm64, so every `long`/pointer field is 8 bytes. The
//! layouts below follow emscripten's wasm64 `struct stat` / `struct dirent64` /
//! `struct sockaddr_in{,6}` field order; they are documented field by field in
//! the code that writes them so a mismatch is a one-line fix, not a hunt.

use std::collections::BTreeMap;
use std::fs::{File, Metadata, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::state;

// ---------------------------------------------------------------------------
// errno values (Linux generic numbering, which is what emscripten's musl uses)
// ---------------------------------------------------------------------------

/// Permission denied.
pub const EACCES: i32 = 13;
/// Bad file descriptor.
pub const EBADF: i32 = 9;
/// Resource temporarily unavailable / would block.
pub const EAGAIN: i32 = 11;
/// Address already in use.
pub const EADDRINUSE: i32 = 98;
/// Connection refused.
pub const ECONNREFUSED: i32 = 111;
/// File exists.
pub const EEXIST: i32 = 17;
/// Bad address (a pointer outside linear memory).
pub const EFAULT: i32 = 14;
/// Interrupted system call.
pub const EINTR: i32 = 4;
/// Invalid argument.
pub const EINVAL: i32 = 22;
/// I/O error.
pub const EIO: i32 = 5;
/// Is a directory.
pub const EISDIR: i32 = 21;
/// Too many open files.
pub const EMFILE: i32 = 24;
/// No such file or directory.
pub const ENOENT: i32 = 2;
/// Not a directory.
pub const ENOTDIR: i32 = 20;
/// Directory not empty.
pub const ENOTEMPTY: i32 = 39;
/// Function not implemented.
pub const ENOSYS: i32 = 38;
/// Socket operation on non-socket.
pub const ENOTSOCK: i32 = 88;
/// Operation not supported.
pub const EOPNOTSUPP: i32 = 95;
/// Broken pipe.
pub const EPIPE: i32 = 32;
/// Not a typewriter (ioctl on a non-tty).
pub const ENOTTY: i32 = 25;
/// Connection timed out.
pub const ETIMEDOUT: i32 = 110;
/// Not connected.
pub const ENOTCONN: i32 = 107;
/// No space left on device.
pub const ENOSPC: i32 = 28;

// ---------------------------------------------------------------------------
// open(2) flags
// ---------------------------------------------------------------------------

/// `O_RDONLY`
pub const O_RDONLY: i32 = 0;
/// `O_WRONLY`
pub const O_WRONLY: i32 = 1;
/// `O_RDWR`
pub const O_RDWR: i32 = 2;
/// `O_CREAT`
pub const O_CREAT: i32 = 0o100;
/// `O_EXCL`
pub const O_EXCL: i32 = 0o200;
/// `O_TRUNC`
pub const O_TRUNC: i32 = 0o1000;
/// `O_APPEND`
pub const O_APPEND: i32 = 0o2000;
/// `O_DIRECTORY`
pub const O_DIRECTORY: i32 = 0o200000;

// ---------------------------------------------------------------------------
// Descriptor table
// ---------------------------------------------------------------------------

/// One record of a directory listing, snapshotted when the directory is opened
/// so `getdents64` can be called repeatedly without holding the OS cursor.
#[derive(Clone, Debug)]
pub struct DirRecord {
    /// Name within the directory.
    pub name: String,
    /// Inode-ish identifier (a stable index; the engine only uses it as a key).
    pub ino: u64,
    /// `DT_DIR` (4), `DT_REG` (8) or `DT_UNKNOWN` (0).
    pub d_type: u8,
}

/// A directory opened with `O_DIRECTORY`, plus the cursor `getdents64` advances.
#[derive(Debug)]
pub struct DirCursor {
    /// Host path the listing came from (diagnostics only).
    pub path: PathBuf,
    /// Snapshot of the entries, `.`/`..` included like a real `getdents`.
    pub entries: Vec<DirRecord>,
    /// Index of the next entry to emit.
    pub pos: usize,
}

/// What a descriptor refers to.
#[derive(Debug)]
pub enum Entry {
    /// Descriptor 0.
    Stdin,
    /// Descriptor 1.
    Stdout,
    /// Descriptor 2.
    Stderr,
    /// A regular file opened through `__syscall_openat`.
    File {
        /// The OS handle.
        file: File,
        /// Whether writes append (affects `fd_write` under `O_APPEND`).
        append: bool,
    },
    /// A directory opened for `getdents64`.
    Dir(DirCursor),
    /// A connected TCP stream.
    Tcp(TcpStream),
    /// A socket created by `socket()` that has not been connected yet.
    TcpPending,
    /// A bound UDP socket.
    Udp(UdpSocket),
    /// A listening socket (no `bind`/`listen` import exists in this module, so
    /// this can only appear if host code inserts one).
    Listener(TcpListener),
}

impl Entry {
    /// Whether this descriptor is a socket (the `poll`/`getsockname` paths care).
    pub fn is_socket(&self) -> bool {
        matches!(
            self,
            Entry::Tcp(_) | Entry::TcpPending | Entry::Udp(_) | Entry::Listener(_)
        )
    }
}

/// The process descriptor table.
#[derive(Debug)]
pub struct Table {
    next: i32,
    entries: BTreeMap<i32, Entry>,
}

impl Table {
    /// A table with stdin/stdout/stderr already present.
    pub fn new() -> Self {
        let mut entries = BTreeMap::new();
        entries.insert(0, Entry::Stdin);
        entries.insert(1, Entry::Stdout);
        entries.insert(2, Entry::Stderr);
        Self { next: 3, entries }
    }

    /// Install `entry` under the next free descriptor.
    pub fn alloc(&mut self, entry: Entry) -> Option<i32> {
        let mut fd = self.next;
        while self.entries.contains_key(&fd) {
            fd = fd.checked_add(1)?;
        }
        self.next = fd.checked_add(1)?;
        self.entries.insert(fd, entry);
        Some(fd)
    }

    /// Install `entry` at a specific descriptor (used by tests and by
    /// `dup`-style host code).
    pub fn insert_at(&mut self, fd: i32, entry: Entry) -> Option<Entry> {
        self.entries.insert(fd, entry)
    }

    /// Look a descriptor up.
    pub fn get(&self, fd: i32) -> Option<&Entry> {
        self.entries.get(&fd)
    }

    /// Look a descriptor up mutably.
    pub fn get_mut(&mut self, fd: i32) -> Option<&mut Entry> {
        self.entries.get_mut(&fd)
    }

    /// Remove a descriptor.
    pub fn remove(&mut self, fd: i32) -> Option<Entry> {
        self.entries.remove(&fd)
    }

    /// Number of live descriptors.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the table is empty (never: 0–2 are pinned).
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

static TABLE: Mutex<Option<Table>> = Mutex::new(None);

/// Run `f` with exclusive access to the process descriptor table.
pub fn with<R>(f: impl FnOnce(&mut Table) -> R) -> R {
    let mut guard = match TABLE.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let table = guard.get_or_insert_with(Table::new);
    f(table)
}

/// Number of live descriptors (diagnostics).
pub fn open_count() -> usize {
    with(|table| table.len())
}

/// Close every descriptor opened by the engine (tests, host teardown).
pub fn close_all() {
    with(|table| {
        let fds: Vec<i32> = table.entries.keys().copied().filter(|fd| *fd > 2).collect();
        for fd in fds {
            table.remove(fd);
        }
        table.next = 3;
    });
}

// ---------------------------------------------------------------------------
// Path resolution
// ---------------------------------------------------------------------------

/// Resolve an engine path to a host path.
///
/// The wasm build's virtual filesystem rooted everything at `/`: `/assets/…`,
/// `/userdata/…`, `/tmp/…`. The server maps `/` onto
/// [`state::fs_root`] (the extracted asset tree) and leaves native absolute
/// paths (a Windows `C:\…` or a UNC path) untouched, so host code can pass either
/// form.
pub fn resolve(path: &str) -> PathBuf {
    let looks_native = path.len() > 1
        && (path.as_bytes()[1] == b':'
            || path.starts_with("\\\\")
            || path.starts_with("//"));
    if looks_native {
        return PathBuf::from(path);
    }
    let rel = path.trim_start_matches('/');
    let root = state::fs_root();
    if rel.is_empty() {
        root
    } else {
        root.join(rel)
    }
}

/// Open a path with emscripten `open(2)` flags.
///
/// `flags` carries the access mode in its low two bits plus `O_CREAT`, `O_EXCL`,
/// `O_TRUNC`, `O_APPEND` and `O_DIRECTORY`. Returns the new descriptor or a
/// negative errno.
pub fn open_path(path: &str, flags: i32) -> i32 {
    let host_path = resolve(path);
    if flags & O_DIRECTORY != 0 {
        return open_dir(&host_path);
    }
    let access = flags & 3;
    let mut opts = OpenOptions::new();
    match access {
        O_WRONLY => {
            opts.write(true);
        }
        O_RDWR => {
            opts.read(true).write(true);
        }
        _ => {
            opts.read(true);
        }
    }
    if flags & O_CREAT != 0 {
        opts.create(true);
    }
    if flags & O_EXCL != 0 {
        opts.create_new(true);
    }
    if flags & O_TRUNC != 0 {
        opts.truncate(true);
    }
    if flags & O_APPEND != 0 {
        opts.append(true);
    }
    match opts.open(&host_path) {
        Ok(file) => with(|table| match table.alloc(Entry::File { file, append: flags & O_APPEND != 0 }) {
            Some(fd) => fd,
            None => -EMFILE,
        }),
        Err(e) => -errno_of(&e),
    }
}

/// Open a directory for `getdents64`.
fn open_dir(path: &Path) -> i32 {
    let read = match std::fs::read_dir(path) {
        Ok(read) => read,
        Err(e) => return -errno_of(&e),
    };
    let mut entries = Vec::new();
    entries.push(DirRecord {
        name: ".".to_owned(),
        ino: 1,
        d_type: 4,
    });
    entries.push(DirRecord {
        name: "..".to_owned(),
        ino: 2,
        d_type: 4,
    });
    for (index, next) in read.enumerate() {
        let entry = match next {
            Ok(entry) => entry,
            Err(e) => return -errno_of(&e),
        };
        let name = entry.file_name().to_string_lossy().into_owned();
        let d_type = match entry.file_type() {
            Ok(ft) if ft.is_dir() => 4,
            Ok(ft) if ft.is_file() => 8,
            _ => 0,
        };
        entries.push(DirRecord {
            name,
            ino: index as u64 + 3,
            d_type,
        });
    }
    with(|table| {
        match table.alloc(Entry::Dir(DirCursor {
            path: path.to_owned(),
            entries,
            pos: 0,
        })) {
            Some(fd) => fd,
            None => -EMFILE,
        }
    })
}

/// Map an `io::Error` to the closest errno.
pub fn errno_of(error: &io::Error) -> i32 {
    // Platform codes that `ErrorKind` does not distinguish everywhere.
    match error.raw_os_error() {
        Some(145) => return ENOTEMPTY,  // Windows ERROR_DIR_NOT_EMPTY
        Some(183) => return EEXIST,     // Windows ERROR_ALREADY_EXISTS
        Some(267) => return ENOTDIR,    // Windows ERROR_DIRECTORY
        Some(5) => return EACCES,       // Windows ERROR_ACCESS_DENIED
        Some(32) | Some(33) => return EACCES, // sharing / lock violation
        _ => {}
    }
    match error.kind() {
        io::ErrorKind::NotFound => ENOENT,
        io::ErrorKind::PermissionDenied => EACCES,
        io::ErrorKind::AlreadyExists => EEXIST,
        io::ErrorKind::InvalidInput => EINVAL,
        io::ErrorKind::BrokenPipe => EPIPE,
        io::ErrorKind::WouldBlock => EAGAIN,
        io::ErrorKind::ConnectionRefused => ECONNREFUSED,
        io::ErrorKind::TimedOut => ETIMEDOUT,
        io::ErrorKind::NotADirectory => ENOTDIR,
        io::ErrorKind::IsADirectory => EISDIR,
        io::ErrorKind::DirectoryNotEmpty => ENOTEMPTY,
        io::ErrorKind::StorageFull => ENOSPC,
        io::ErrorKind::Unsupported => EOPNOTSUPP,
        _ => EIO,
    }
}

// ---------------------------------------------------------------------------
// struct stat (wasm64)
// ---------------------------------------------------------------------------

/// Size of the `struct stat` the shims write.
pub const STAT_LEN: usize = 128;

/// Encode a `Metadata` into an emscripten wasm64 `struct stat`.
///
/// Offsets: `dev@0, ino@8, mode@16, nlink@20, uid@24, gid@28, rdev@32, size@40,
/// blksize@48, blocks@56, atim{sec@64,nsec@72}, mtim{sec@80,nsec@88},
/// ctim{sec@96,nsec@104}, pad[2]@112`. All little-endian.
pub fn encode_stat(meta: &Metadata, out: &mut [u8]) -> bool {
    if out.len() < STAT_LEN {
        return false;
    }
    out[..STAT_LEN].fill(0);
    let mode: u32 = if meta.is_dir() {
        0o040755
    } else if meta.file_type().is_symlink() {
        0o120777
    } else {
        0o100644
    };
    let size = meta.len();
    let mtime = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .unwrap_or_default();
    let atime = meta
        .accessed()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .unwrap_or_default();
    let secs = mtime.as_secs() as i64;
    let nsecs = mtime.subsec_nanos() as i64;
    put64(out, 0, 0); // dev
    put64(out, 8, 1); // ino: synthetic but stable per stat call
    out[16..20].copy_from_slice(&mode.to_le_bytes());
    out[20..24].copy_from_slice(&1u32.to_le_bytes()); // nlink
    out[24..28].copy_from_slice(&0u32.to_le_bytes()); // uid
    out[28..32].copy_from_slice(&0u32.to_le_bytes()); // gid
    put64(out, 32, 0); // rdev
    put64(out, 40, size);
    put64(out, 48, 4096); // blksize
    put64(out, 56, size.div_ceil(512));
    put64(out, 64, atime.as_secs());
    put64(out, 72, u64::from(atime.subsec_nanos()));
    put64(out, 80, secs as u64);
    put64(out, 88, nsecs as u64);
    put64(out, 96, secs as u64);
    put64(out, 104, nsecs as u64);
    true
}

/// Little-endian `u64` store into a stat buffer (bounds already checked).
fn put64(out: &mut [u8], offset: usize, value: u64) {
    out[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

/// Encode a synthetic `struct stat` for a socket, pipe or character device.
///
/// Sockets report `S_IFSOCK` and no size; the engine's libc only checks
/// `st_mode` and `st_size` on them.
pub fn encode_stat_socket(out: &mut [u8]) -> bool {
    if out.len() < STAT_LEN {
        return false;
    }
    out[..STAT_LEN].fill(0);
    out[16..20].copy_from_slice(&0o140777u32.to_le_bytes());
    out[20..24].copy_from_slice(&1u32.to_le_bytes());
    true
}

// ---------------------------------------------------------------------------
// getdents64
// ---------------------------------------------------------------------------

/// Size of one `linux_dirent64` header before the name.
pub const DIRENT_HEADER: usize = 19;

/// Encode `records` starting at `cursor` into `out`, returning
/// `(bytes_written, records_consumed)`.
///
/// Record layout: `d_ino@0 u64, d_off@8 i64, d_reclen@16 u16, d_type@18 u8,
/// d_name@19 (NUL terminated)`, padded to 8 bytes. `d_off` is the cursor value
/// after this record, which is what the engine's `readdir` loop advances by.
pub fn encode_dirents(records: &[DirRecord], cursor: usize, out: &mut [u8]) -> (usize, usize) {
    let mut written = 0;
    let mut used = 0;
    for (index, record) in records.iter().enumerate().skip(cursor) {
        let name = record.name.as_bytes();
        let raw = DIRENT_HEADER + name.len() + 1;
        let reclen = (raw + 7) & !7;
        if written + reclen > out.len() {
            break;
        }
        let slot = &mut out[written..written + reclen];
        slot.fill(0);
        slot[0..8].copy_from_slice(&record.ino.to_le_bytes());
        slot[8..16].copy_from_slice(&((index as i64) + 1).to_le_bytes());
        slot[16..18].copy_from_slice(&(reclen as u16).to_le_bytes());
        slot[18] = record.d_type;
        slot[DIRENT_HEADER..DIRENT_HEADER + name.len()].copy_from_slice(name);
        written += reclen;
        used += 1;
    }
    (written, used)
}

// ---------------------------------------------------------------------------
// sockaddr
// ---------------------------------------------------------------------------

/// `AF_INET`
pub const AF_INET: u32 = 2;
/// `AF_INET6`
pub const AF_INET6: u32 = 10;
/// `SOCK_STREAM`
pub const SOCK_STREAM: i32 = 1;
/// `SOCK_DGRAM`
pub const SOCK_DGRAM: i32 = 2;

/// Decode a `sockaddr_in`/`sockaddr_in6` from linear memory.
///
/// `sockaddr_in`: `family@0 u16, port@2 u16 (big endian), addr@4 [4]u8, zero@8`.
/// `sockaddr_in6`: `family@0 u16, port@2 u16 (big endian), flowinfo@4 u32,
/// addr@8 [16]u8, scope@24 u32`.
pub fn decode_sockaddr(bytes: &[u8]) -> Option<SocketAddr> {
    if bytes.len() < 8 {
        return None;
    }
    let family = u16::from_le_bytes([bytes[0], bytes[1]]);
    let port = u16::from_be_bytes([bytes[2], bytes[3]]);
    match u32::from(family) {
        AF_INET => {
            if bytes.len() < 8 {
                return None;
            }
            let ip = std::net::Ipv4Addr::new(bytes[4], bytes[5], bytes[6], bytes[7]);
            Some(SocketAddr::from((ip, port)))
        }
        AF_INET6 => {
            if bytes.len() < 24 {
                return None;
            }
            let mut octets = [0u8; 16];
            octets.copy_from_slice(&bytes[8..24]);
            let ip = std::net::Ipv6Addr::from(octets);
            Some(SocketAddr::from((ip, port)))
        }
        _ => None,
    }
}

/// Encode a `SocketAddr` as a `sockaddr_in`/`sockaddr_in6` into `out` and
/// return the number of bytes a real `accept`/`getsockname` would report.
pub fn encode_sockaddr(addr: SocketAddr, out: &mut [u8]) -> usize {
    match addr {
        SocketAddr::V4(v4) => {
            if out.len() >= 16 {
                out[..16].fill(0);
                out[0..2].copy_from_slice(&(AF_INET as u16).to_le_bytes());
                out[2..4].copy_from_slice(&v4.port().to_be_bytes());
                out[4..8].copy_from_slice(&v4.ip().octets());
            }
            16
        }
        SocketAddr::V6(v6) => {
            if out.len() >= 28 {
                out[..28].fill(0);
                out[0..2].copy_from_slice(&(AF_INET6 as u16).to_le_bytes());
                out[2..4].copy_from_slice(&v6.port().to_be_bytes());
                out[8..24].copy_from_slice(&v6.ip().octets());
                out[24..28].copy_from_slice(&v6.scope_id().to_le_bytes());
            }
            28
        }
    }
}

/// Read a little-endian `u32` from a byte slice (bounds-checked).
pub fn le_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    let slice = bytes.get(offset..offset + 4)?;
    Some(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

/// Read a little-endian `u64` from a byte slice (bounds-checked).
pub fn le_u64(bytes: &[u8], offset: usize) -> Option<u64> {
    let slice = bytes.get(offset..offset + 8)?;
    let mut raw = [0u8; 8];
    raw.copy_from_slice(slice);
    Some(u64::from_le_bytes(raw))
}

/// Read up to `len` bytes from a descriptor, whatever it is.
pub fn read_entry(entry: &mut Entry, buf: &mut [u8]) -> Result<usize, i32> {
    match entry {
        Entry::File { file, .. } => file.read(buf).map_err(|e| errno_of(&e)),
        Entry::Stdin => io::stdin().read(buf).map_err(|e| errno_of(&e)),
        Entry::Udp(sock) => sock.recv(buf).map_err(|e| errno_of(&e)),
        Entry::Tcp(stream) => stream.read(buf).map_err(|e| errno_of(&e)),
        Entry::TcpPending => Err(ENOTCONN),
        Entry::Dir(_) => Err(EISDIR),
        Entry::Stdout | Entry::Stderr => Err(EBADF),
        Entry::Listener(_) => Err(EINVAL),
    }
}

/// Write to a descriptor, whatever it is.
pub fn write_entry(entry: &mut Entry, buf: &[u8]) -> Result<usize, i32> {
    match entry {
        Entry::File { file, .. } => file.write_all(buf).map(|()| buf.len()).map_err(|e| errno_of(&e)),
        Entry::Stdout => io::stdout().write_all(buf).map(|()| buf.len()).map_err(|e| errno_of(&e)),
        Entry::Stderr => io::stderr().write_all(buf).map(|()| buf.len()).map_err(|e| errno_of(&e)),
        Entry::Udp(sock) => sock.send(buf).map_err(|e| errno_of(&e)),
        Entry::Tcp(stream) => stream.write(buf).map_err(|e| errno_of(&e)),
        Entry::TcpPending => Err(ENOTCONN),
        Entry::Stdin | Entry::Dir(_) => Err(EBADF),
        Entry::Listener(_) => Err(EINVAL),
    }
}

/// Seek a descriptor that supports it.
pub fn seek_entry(entry: &mut Entry, offset: i64, whence: i32) -> Result<u64, i32> {
    const SEEK_SET: i32 = 0;
    const SEEK_CUR: i32 = 1;
    const SEEK_END: i32 = 2;
    let file = match entry {
        Entry::File { file, .. } => file,
        Entry::Stdin | Entry::Stdout | Entry::Stderr => return Err(ESPIPE),
        _ => return Err(ESPIPE),
    };
    let from = match whence {
        SEEK_SET => SeekFrom::Start(offset.max(0) as u64),
        SEEK_CUR => SeekFrom::Current(offset),
        SEEK_END => SeekFrom::End(offset),
        _ => return Err(EINVAL),
    };
    file.seek(from).map_err(|e| errno_of(&e))
}

/// `ESPIPE` — seek on a pipe/socket.
pub const ESPIPE: i32 = 29;

/// Whether the engine asked for a socket to be created but not yet connected.
pub fn is_pending_socket(fd: i32) -> bool {
    with(|table| matches!(table.get(fd), Some(Entry::TcpPending)))
}