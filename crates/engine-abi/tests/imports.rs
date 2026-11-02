//! Integration test for `engine-abi`: install a real `HeadlessHost`, publish a
//! linear-memory reservation, then call every implemented import once and assert
//! the behaviour documented on it.
//!
//! One process holds one host and one memory mapping (that is the ABI's shape), so
//! every test takes [`serial`] — cargo runs test functions concurrently in one
//! binary, and two hosts installed at once would make the assertions meaningless.

use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicI32, AtomicI64, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use engine_abi as abi;
use engine_abi::fd;
use engine_abi::state::Counter;
use engine_host::{HeadlessConfig, HeadlessHost, Host};

/// Serialises tests that touch the process-global host/memory.
fn serial() -> MutexGuard<'static, ()> {
    static LOCK: Mutex<()> = Mutex::new(());
    match LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

/// A directory under the OS temp dir that deletes itself.
struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(tag: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let path = std::env::temp_dir().join(format!(
            "engine-abi-{tag}-{}-{nanos}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).expect("create temp dir");
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

/// The engine's linear memory plus the host it runs against.
struct Harness {
    host: Arc<HeadlessHost>,
    _guard: abi::InstallGuard,
    memory: Vec<u8>,
    /// Next free offset in the string arena.
    bump: usize,
    dir: TempDir,
}

/// Size of the reservation: enough for every test's fixtures.
const MEMORY_LEN: usize = 1 << 20;

impl Harness {
    fn new(tag: &str) -> Self {
        let dir = TempDir::new(tag);
        // The dir-backed asset store requires its root to exist.
        std::fs::create_dir_all(dir.path().join("assets")).expect("asset root");
        let host = Arc::new(
            HeadlessHost::new(
                HeadlessConfig::new(dir.path().join("userdata"))
                    .with_asset_root(dir.path().join("assets"))
                    .with_module_int("renderer", 2)
                    .with_module_int("locale", 8),
            )
            .expect("headless host"),
        );
        let mut memory = vec![0u8; MEMORY_LEN];
        let bump = 0x8_0000;
        // SAFETY: `memory` is leaked into the harness for the whole test and is
        // never reallocated, so the published base stays valid.
        unsafe { abi::set_memory(memory.as_mut_ptr(), memory.len()) };
        abi::set_fs_root(dir.path().to_path_buf());
        let guard = abi::install(Arc::clone(&host) as Arc<dyn engine_host::Host>);
        Self {
            host,
            _guard: guard,
            memory,
            bump,
            dir,
        }
    }

    /// A linear-memory "pointer" (an offset, exactly what the engine passes).
    fn ptr(offset: usize) -> *mut u8 {
        offset as *mut u8
    }

    /// Write `bytes` at `offset`.
    fn put(&mut self, offset: usize, bytes: &[u8]) {
        self.memory[offset..offset + bytes.len()].copy_from_slice(bytes);
    }

    /// Write a NUL-terminated string into the string arena (from 0x80000 up),
    /// returning its offset. The arena keeps fixtures from overlapping each other
    /// and the fixed-size data blocks below 0x8000.
    fn put_str(&mut self, text: &str) -> *mut u8 {
        let mut bytes = text.as_bytes().to_vec();
        bytes.push(0);
        let offset = self.bump;
        self.put(offset, &bytes);
        self.bump += bytes.len();
        Self::ptr(offset)
    }

    /// Read `len` bytes at `offset`.
    fn get(&self, offset: usize, len: usize) -> &[u8] {
        &self.memory[offset..offset + len]
    }

    /// Read a little-endian `u64` at `offset`.
    fn u64_at(&self, offset: usize) -> u64 {
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&self.memory[offset..offset + 8]);
        u64::from_le_bytes(raw)
    }

    /// Read a little-endian `u32` at `offset`.
    fn u32_at(&self, offset: usize) -> u32 {
        let mut raw = [0u8; 4];
        raw.copy_from_slice(&self.memory[offset..offset + 4]);
        u32::from_le_bytes(raw)
    }

    /// Read a little-endian `i32` at `offset`.
    fn i32_at(&self, offset: usize) -> i32 {
        self.u32_at(offset) as i32
    }
}

// ---------------------------------------------------------------------------
// Time
// ---------------------------------------------------------------------------

#[test]
fn clock_imports_are_monotonic_and_distinct_from_wall_clock() {
    let _serial = serial();
    let harness = Harness::new("time");

    let first = abi::time::wasm_now_ms();
    std::thread::sleep(std::time::Duration::from_millis(2));
    let second = abi::time::wasm_now_ms();
    assert!(first >= 0.0, "monotonic ms must not be negative");
    assert!(second >= first, "{second} must be >= {first}");
    assert!(
        abi::time::emscripten_get_now() >= second - 1.0,
        "emscripten_get_now shares the monotonic timeline"
    );

    // Date.now(): wall clock, roughly "now" in ms since the epoch.
    let wall = abi::time::emscripten_date_now();
    let expected = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as f64)
        .unwrap_or(0.0);
    assert!((wall - expected).abs() < 60_000.0, "wall clock {wall} vs {expected}");
    assert!(wall > 1.0e12, "wall clock must be a unix timestamp");

    // clock_time_get(CLOCK_MONOTONIC, _, &out) writes 8 bytes of nanos.
    // SAFETY: `out` is an offset inside the harness reservation.
    let rc = unsafe { abi::time::clock_time_get(1, 0, Harness::ptr(0x0100)) };
    assert_eq!(rc, 0);
    assert!(harness.u64_at(0x0100) > 0, "monotonic nanos must be non-zero");

    // An unknown clock id is EINVAL, not a panic.
    // SAFETY: as above.
    let bad = unsafe { abi::time::clock_time_get(99, 0, Harness::ptr(0x0108)) };
    assert_eq!(bad, -28, "unknown clock id is -EINVAL");
}

// ---------------------------------------------------------------------------
// module_int / userdata / audio / httpfs / input / gpu
// ---------------------------------------------------------------------------

#[test]
fn module_int_reads_the_host_table_and_falls_back() {
    let _serial = serial();
    let mut harness = Harness::new("module-int");

    let key = harness.put_str("renderer");
    // SAFETY: `key` is a NUL-terminated string inside the reservation.
    let value = unsafe { abi::module_int::wasm_module_int_js(key, 99) };
    assert_eq!(value, 2, "configured tunable wins");

    let missing = harness.put_str("not-a-tunable");
    // SAFETY: as above.
    let fallback = unsafe { abi::module_int::wasm_module_int_js(missing, 1024) };
    assert_eq!(fallback, 1024, "absent key answers the engine's fallback");

    // A pointer outside linear memory is the fallback too, never a crash.
    // SAFETY: `ptr` is deliberately out of range.
    let out_of_range = unsafe { abi::module_int::wasm_module_int_js(Harness::ptr(usize::MAX >> 1), 7) };
    assert_eq!(out_of_range, 7);
}

#[test]
fn userdata_put_page_has_page_and_delete_round_trip() {
    let _serial = serial();
    let mut harness = Harness::new("userdata");

    let key = harness.put_str("sgta400/save1");
    harness.put(0x0380, b"save-blob-v1");
    // SAFETY: both pointers are NUL-terminated/in-range inside the reservation.
    unsafe { abi::userdata::wasm_userdata_put_js(key, Harness::ptr(0x0380), 12, 1.7e9) };

    let stored = harness
        .host
        .userdata()
        .get("sgta400/save1")
        .expect("get")
        .expect("blob present");
    assert_eq!(stored, b"save-blob-v1");

    assert_eq!(abi::userdata::wasm_userdata_page_js(), 1, "one row page");
    assert_eq!(abi::userdata::wasm_has_page_js(), 1, "storage available");

    // SAFETY: `key` is still in range.
    unsafe { abi::userdata::wasm_userdata_delete_js(key) };
    assert!(harness
        .host
        .userdata()
        .get("sgta400/save1")
        .expect("get")
        .is_none());
    assert_eq!(abi::userdata::wasm_userdata_page_js(), 0, "empty page after delete");
    assert_eq!(
        abi::userdata::wasm_has_page_js(),
        1,
        "storage stays available when empty"
    );

    // Deleting a missing key is not an error and must not panic.
    // SAFETY: as above.
    unsafe { abi::userdata::wasm_userdata_delete_js(key) };
}

#[test]
fn audio_publish_reaches_the_sink_through_the_descriptor() {
    let _serial = serial();
    let mut harness = Harness::new("audio");

    // Descriptor { data_ptr: u64 @0, byte_len: u32 @8 } plus 256 f32 samples
    // (128 stereo frames) at 0x0500.
    let samples = 256usize;
    let mut pcm = Vec::with_capacity(samples * 4);
    for i in 0..samples {
        pcm.extend_from_slice(&((i as f32) * 0.5).to_le_bytes());
    }
    harness.put(0x0500, &pcm);
    harness.put(0x0400, &(0x0500u64).to_le_bytes());
    harness.put(0x0408, &((samples * 4) as u32).to_le_bytes());

    let before = harness.host.audio_sink().blocks();
    // SAFETY: the descriptor and payload are inside the reservation.
    unsafe { abi::audio::wasm_audio_publish_js(Harness::ptr(0x0400), (samples * 4) as u32) };
    assert_eq!(
        harness.host.audio_sink().blocks(),
        before + 1,
        "the null sink counts one published block"
    );
    assert_eq!(
        harness.host.audio_sink().frames(),
        128,
        "256 samples of stereo = 128 frames"
    );

    // A descriptor whose data pointer is out of range falls back to treating the
    // argument itself as the payload, so a malformed descriptor is not fatal.
    let before = harness.host.audio_sink().blocks();
    // SAFETY: as above; the payload path is the descriptor address range.
    unsafe { abi::audio::wasm_audio_publish_js(Harness::ptr(0x0400), 0) };
    assert_eq!(
        harness.host.audio_sink().blocks(),
        before,
        "a zero-length publish is dropped"
    );
}

#[test]
fn httpfs_manifest_uses_the_query_then_fill_protocol() {
    let _serial = serial();
    let harness = Harness::new("manifest");

    // Query mode: ptr = 0 answers the required length (excluding the NUL).
    // SAFETY: a null pointer is the documented query form.
    let needed = unsafe { abi::httpfs::wasm_httpfs_manifest_js(std::ptr::null_mut(), 0) };
    assert!(needed > 0, "the manifest text is non-empty, got {needed}");

    // Fill mode: a buffer of exactly needed + 1 bytes succeeds and is NUL
    // terminated.
    let capacity = (needed as u32) + 1;
    // SAFETY: the buffer at 0x0600 is `capacity` bytes inside the reservation.
    let written = unsafe { abi::httpfs::wasm_httpfs_manifest_js(Harness::ptr(0x0600), capacity) };
    assert_eq!(written, needed, "fill returns the byte count");
    let text = std::str::from_utf8(harness.get(0x0600, needed as usize)).expect("utf-8");
    assert!(engine_host::AssetManifest::parse_engine_json(text).expect("engine JSON manifest").is_empty());
    assert_eq!(harness.get(0x0600 + needed as usize, 1), b"\0", "NUL terminated");

    // A short buffer is refused rather than truncated.
    // SAFETY: as above.
    let short = unsafe { abi::httpfs::wasm_httpfs_manifest_js(Harness::ptr(0x0600), 4) };
    assert_eq!(short, needed, "insufficient capacity still reports the required length");

    let before = abi::state::count(Counter::IoStart);
    unsafe { abi::httpfs::wasm_httpfs_io_start_js(Harness::ptr(0x0700)) };
    assert_eq!(abi::state::count(Counter::IoStart), before + 1);
}

#[test]
fn input_publish_and_gpu_start_worker_are_counted_no_ops() {
    let _serial = serial();
    let harness = Harness::new("noop");

    let before = abi::state::count(Counter::InputPublish);
    // SAFETY: the import only records the pointer value.
    unsafe { abi::input::wasm_input_publish_js(Harness::ptr(0x0800)) };
    assert_eq!(abi::state::count(Counter::InputPublish), before + 1);
    assert_eq!(harness.host.gpu_sink().frames(), 0, "no GPU work was queued");

    let before = abi::state::count(Counter::WgpuStartWorker);
    // SAFETY: neither pointer is dereferenced.
    unsafe { abi::gpu::wgpu_start_worker(Harness::ptr(0x0900), 33_554_432, Harness::ptr(0x0800)) };
    assert_eq!(abi::state::count(Counter::WgpuStartWorker), before + 1);
}

// ---------------------------------------------------------------------------
// Filesystem syscalls
// ---------------------------------------------------------------------------

#[test]
fn file_syscalls_round_trip_through_the_descriptor_table() {
    let _serial = serial();
    let mut harness = Harness::new("files");
    let file_path = harness.dir.path().join("hello.bin");
    let path = harness.put_str(file_path.to_str().expect("utf-8 path"));

    // openat(path, O_CREAT|O_RDWR|O_TRUNC) — the flags libc's fopen("w+b") uses.
    let flags = (fd::O_CREAT | fd::O_RDWR | fd::O_TRUNC) as u32;
    // SAFETY: `path` is a NUL-terminated string inside the reservation.
    let open_fd = unsafe { abi::syscall::__syscall_openat(-100, path, flags, 0o644) };
    assert!(open_fd >= 3, "a fresh descriptor, got {open_fd}");

    // fd_write over a two-element iovec: "hello " + "world".
    harness.put(0x1100, b"hello ");
    harness.put(0x1110, b"world");
    let mut iov = Vec::new();
    iov.extend_from_slice(&0x1100u64.to_le_bytes());
    iov.extend_from_slice(&6u64.to_le_bytes());
    iov.extend_from_slice(&0x1110u64.to_le_bytes());
    iov.extend_from_slice(&5u64.to_le_bytes());
    harness.put(0x1200, &iov);
    // SAFETY: the iovec and both buffers are inside the reservation.
    let rc = unsafe { abi::wasi::fd_write(open_fd, Harness::ptr(0x1200), 2, Harness::ptr(0x1300)) };
    assert_eq!(rc, 0, "fd_write succeeds");
    assert_eq!(harness.u64_at(0x1300), 11, "eleven bytes written");

    // fd_seek back to 0, then fd_read the same 11 bytes.
    // SAFETY: as above.
    let rc = unsafe { abi::wasi::fd_seek(open_fd, 0, 0, Harness::ptr(0x1308)) };
    assert_eq!(rc, 0);
    assert_eq!(harness.u64_at(0x1308), 0, "seek reports offset 0");
    let mut riov = Vec::new();
    riov.extend_from_slice(&0x1400u64.to_le_bytes());
    riov.extend_from_slice(&11u64.to_le_bytes());
    harness.put(0x1210, &riov);
    // SAFETY: as above.
    let rc = unsafe { abi::wasi::fd_read(open_fd, Harness::ptr(0x1210), 1, Harness::ptr(0x1310)) };
    assert_eq!(rc, 0);
    assert_eq!(harness.u64_at(0x1310), 11);
    assert_eq!(harness.get(0x1400, 11), b"hello world");

    // fstat64 reports S_IFREG and the real size at offset 40.
    // SAFETY: the stat buffer is inside the reservation.
    let rc = unsafe { abi::syscall::__syscall_fstat64(open_fd, Harness::ptr(0x1500)) };
    assert_eq!(rc, 0);
    let mode = harness.u32_at(0x1510);
    assert_eq!(mode & 0o170000, 0o100000, "S_IFREG, got {mode:o}");
    assert_eq!(harness.u64_at(0x1528), 11, "st_size at offset 40");

    // ftruncate64 shortens the file.
    // SAFETY: as above.
    let rc = unsafe { abi::syscall::__syscall_ftruncate64(open_fd, 5) };
    assert_eq!(rc, 0);
    assert_eq!(std::fs::metadata(&file_path).expect("stat").len(), 5);

    // fd_close drops the descriptor; a second close is EBADF.
    // SAFETY: `open_fd` came from this table.
    assert_eq!(unsafe { abi::wasi::fd_close(open_fd) }, 0);
    // SAFETY: as above.
    assert_eq!(unsafe { abi::wasi::fd_close(open_fd) }, abi::wasi::WASI_EBADF);

    // stat64 / lstat64 / newfstatat all see the file.
    // SAFETY: the path and stat buffer are in range.
    assert_eq!(unsafe { abi::syscall::__syscall_stat64(path, Harness::ptr(0x1500)) }, 0);
    assert_eq!(harness.u64_at(0x1528), 5, "size after truncate");
    // SAFETY: as above.
    assert_eq!(unsafe { abi::syscall::__syscall_lstat64(path, Harness::ptr(0x1500)) }, 0);
    // SAFETY: as above.
    assert_eq!(
        unsafe { abi::syscall::__syscall_newfstatat(-100, path, Harness::ptr(0x1500), 0) },
        0
    );
    // A missing path is ENOENT, not a crash.
    let missing = harness.put_str(harness.dir.path().join("nope.bin").to_str().unwrap());
    // SAFETY: as above.
    assert_eq!(
        unsafe { abi::syscall::__syscall_stat64(missing, Harness::ptr(0x1500)) },
        -fd::ENOENT
    );
    // A pointer outside memory is EFAULT.
    // SAFETY: deliberately out of range.
    assert_eq!(
        unsafe { abi::syscall::__syscall_stat64(Harness::ptr(usize::MAX >> 1), Harness::ptr(0x1500)) },
        -fd::EFAULT
    );

    // mkdirat / getdents64 / unlinkat / rmdir / renameat.
    let dir_path = harness.dir.path().join("sub");
    let dir_str = harness.put_str(dir_path.to_str().expect("utf-8"));
    // SAFETY: as above.
    assert_eq!(unsafe { abi::syscall::__syscall_mkdirat(-100, dir_str, 0o755) }, 0);
    assert!(dir_path.is_dir());
    std::fs::write(dir_path.join("a.txt"), b"x").expect("write fixture");

    let dflags = (fd::O_DIRECTORY | fd::O_RDONLY) as u32;
    // SAFETY: as above.
    let dir_fd = unsafe { abi::syscall::__syscall_openat(-100, dir_str, dflags, 0) };
    assert!(dir_fd >= 3, "directory descriptor");
    // SAFETY: the output buffer is inside the reservation.
    let bytes = unsafe { abi::syscall::__syscall_getdents64(dir_fd, Harness::ptr(0x1600), 512) };
    assert!(bytes > 0, "getdents64 produced records");
    let listing = harness.get(0x1600, bytes as usize);
    assert!(
        listing.windows(5).any(|w| w == b"a.txt"),
        "directory listing contains a.txt"
    );
    // SAFETY: as above.
    assert_eq!(unsafe { abi::wasi::fd_close(dir_fd) }, 0);

    let file_in_dir = harness.put_str(dir_path.join("a.txt").to_str().unwrap());
    // SAFETY: as above.
    assert_eq!(unsafe { abi::syscall::__syscall_unlinkat(-100, file_in_dir, 0) }, 0);
    assert!(!dir_path.join("a.txt").exists());
    // SAFETY: as above.
    assert_eq!(unsafe { abi::syscall::__syscall_rmdir(dir_str) }, 0);
    assert!(!dir_path.exists());

    // renameat moves a file.
    let from = harness.put_str(file_path.to_str().unwrap());
    let to_path = harness.dir.path().join("renamed.bin");
    let to = harness.put_str(to_path.to_str().unwrap());
    // SAFETY: both paths are in range.
    assert_eq!(unsafe { abi::syscall::__syscall_renameat(-100, from, -100, to) }, 0);
    assert!(to_path.exists());

    // getcwd fills a buffer with the host working directory.
    // SAFETY: the buffer is inside the reservation.
    let written = unsafe { abi::syscall::__syscall_getcwd(Harness::ptr(0x1800), 256) };
    assert!(written > 1, "getcwd wrote {written} bytes");
    let cwd = std::str::from_utf8(harness.get(0x1800, written as usize - 1)).expect("utf-8");
    assert_eq!(cwd, std::env::current_dir().unwrap().to_string_lossy());
}

#[test]
fn pread_pwrite_do_not_move_the_file_cursor() {
    let _serial = serial();
    let mut harness = Harness::new("pread");
    let file_path = harness.dir.path().join("p.bin");
    std::fs::write(&file_path, b"0123456789").expect("write fixture");
    let path = harness.put_str(file_path.to_str().unwrap());

    // SAFETY: the path is a NUL-terminated string in range.
    let open_fd = unsafe {
        abi::syscall::__syscall_openat(-100, path, (fd::O_RDWR) as u32, 0)
    };
    assert!(open_fd >= 3);

    // Position the cursor at 2 and read 3 bytes at offset 7.
    // SAFETY: the out pointers are in range.
    assert_eq!(unsafe { abi::wasi::fd_seek(open_fd, 2, 0, Harness::ptr(0x2100)) }, 0);
    let mut iov = Vec::new();
    iov.extend_from_slice(&0x2200u64.to_le_bytes());
    iov.extend_from_slice(&3u64.to_le_bytes());
    harness.put(0x2300, &iov);
    // SAFETY: as above.
    let rc = unsafe { abi::wasi::fd_pread(open_fd, Harness::ptr(0x2300), 1, 7, Harness::ptr(0x2400)) };
    assert_eq!(rc, 0);
    assert_eq!(harness.u64_at(0x2400), 3);
    assert_eq!(harness.get(0x2200, 3), b"789");

    // The cursor is still at 2: pwrite at offset 0 must not disturb it either.
    let mut wiov = Vec::new();
    wiov.extend_from_slice(&0x2500u64.to_le_bytes());
    wiov.extend_from_slice(&2u64.to_le_bytes());
    harness.put(0x2600, &wiov);
    harness.put(0x2500, b"AB");
    // SAFETY: as above.
    let rc = unsafe { abi::wasi::fd_pwrite(open_fd, Harness::ptr(0x2600), 1, 0, Harness::ptr(0x2410)) };
    assert_eq!(rc, 0);
    assert_eq!(harness.u64_at(0x2410), 2);
    assert_eq!(std::fs::read(&file_path).expect("read"), b"AB23456789");

    // Reading two bytes now must start at the cursor (2), i.e. "23".
    let mut riov = Vec::new();
    riov.extend_from_slice(&0x2700u64.to_le_bytes());
    riov.extend_from_slice(&2u64.to_le_bytes());
    harness.put(0x2800, &riov);
    // SAFETY: as above.
    assert_eq!(unsafe { abi::wasi::fd_read(open_fd, Harness::ptr(0x2800), 1, Harness::ptr(0x2420)) }, 0);
    assert_eq!(harness.get(0x2700, 2), b"23", "cursor survived pread and pwrite");
    // SAFETY: as above.
    assert_eq!(unsafe { abi::wasi::fd_close(open_fd) }, 0);
}

#[test]
fn misc_syscalls_answer_instead_of_failing() {
    let _serial = serial();
    let mut harness = Harness::new("misc");
    let file_path = harness.dir.path().join("m.bin");
    std::fs::write(&file_path, b"x").expect("fixture");
    let path = harness.put_str(file_path.to_str().unwrap());

    // fcntl64: the descriptor-flag subset answers 0, anything else EINVAL.
    // SAFETY: the descriptor is valid; `fcntl` reads no memory.
    let open_fd = unsafe { abi::syscall::__syscall_openat(-100, path, fd::O_RDONLY as u32, 0) };
    assert!(open_fd >= 3);
    // SAFETY: as above.
    assert_eq!(unsafe { abi::syscall::__syscall_fcntl64(open_fd, 3, 0) }, 0, "F_GETFL");
    // SAFETY: as above.
    assert_eq!(unsafe { abi::syscall::__syscall_fcntl64(open_fd, 7, 0) }, -fd::EINVAL);
    // SAFETY: as above.
    assert_eq!(unsafe { abi::syscall::__syscall_fcntl64(9999, 3, 0) }, -fd::EBADF);
    // SAFETY: `ioctl` on a regular file is not a tty.
    assert_eq!(unsafe { abi::syscall::__syscall_ioctl(open_fd, 0x5401, 0) }, -fd::ENOTTY);

    // poll(2) on a file descriptor: always ready for write, and it must write
    // revents at offset +6 of the pollfd.
    harness.put(0x3100, &(open_fd as u32).to_le_bytes());
    harness.put(0x3104, &(4i16).to_le_bytes()); // POLLOUT
    harness.put(0x3106, &(0i16).to_le_bytes());
    // SAFETY: one 8-byte pollfd lives at 0x3100.
    let ready = unsafe { abi::syscall::__syscall_poll(Harness::ptr(0x3100), 1, 10) };
    assert_eq!(ready, 1, "the file is writable");
    let revents = i16::from_le_bytes([harness.memory[0x3106], harness.memory[0x3107]]);
    assert_eq!(revents, 4, "POLLOUT reported");

    // A poll with a zero timeout and no ready descriptor returns 0.
    harness.put(0x3110, &(9999u32).to_le_bytes());
    harness.put(0x3114, &(1i16).to_le_bytes());
    harness.put(0x3116, &(0i16).to_le_bytes());
    // SAFETY: as above.
    let none = unsafe { abi::syscall::__syscall_poll(Harness::ptr(0x3110), 1, 0) };
    assert_eq!(none, 1, "an unknown descriptor reports POLLNVAL");
    let nval = i16::from_le_bytes([harness.memory[0x3116], harness.memory[0x3117]]);
    assert_eq!(nval, 0x20, "POLLNVAL");

    // statfs64 writes a plausible record and refuses a short buffer.
    // SAFETY: 120 bytes at 0x3200 are in range.
    assert_eq!(unsafe { abi::syscall::__syscall_statfs64(path, 120, Harness::ptr(0x3200)) }, 0);
    assert_eq!(harness.u64_at(0x3208), 4096, "f_bsize");
    // SAFETY: as above, but the declared length is too small.
    assert_eq!(
        unsafe { abi::syscall::__syscall_statfs64(path, 8, Harness::ptr(0x3200)) },
        -fd::EINVAL
    );

    // accept4 cannot be reached by this module (no bind/listen import).
    // SAFETY: the pointers are ignored by the EINVAL path.
    assert_eq!(
        unsafe { abi::syscall::__syscall_accept4(open_fd, Harness::ptr(0x3300), Harness::ptr(0x3310), 0, 0, 0) },
        -fd::EINVAL
    );
    // SAFETY: as above.
    assert_eq!(unsafe { abi::wasi::fd_close(open_fd) }, 0);
}

// ---------------------------------------------------------------------------
// Sockets
// ---------------------------------------------------------------------------

/// Encode a `sockaddr_in` for 127.0.0.1:`port` at `offset`.
fn put_sockaddr_in(harness: &mut Harness, offset: usize, port: u16) {
    let mut raw = [0u8; 16];
    raw[0..2].copy_from_slice(&2u16.to_le_bytes()); // AF_INET
    raw[2..4].copy_from_slice(&port.to_be_bytes()); // network order
    raw[4..8].copy_from_slice(&[127, 0, 0, 1]);
    harness.put(offset, &raw);
}

#[test]
fn tcp_socket_connect_send_receive_shutdown() {
    let _serial = serial();
    let mut harness = Harness::new("tcp");

    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let port = listener.local_addr().expect("local addr").port();
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept");
        let mut buf = [0u8; 4];
        use std::io::{Read, Write};
        stream.read_exact(&mut buf).expect("read ping");
        stream.write_all(b"pong").expect("write pong");
        buf
    });

    // socket(AF_INET, SOCK_STREAM, 0)
    // SAFETY: no memory is passed.
    let sock = unsafe { abi::syscall::__syscall_socket(2, 1, 0, 0, 0, 0) };
    assert!(sock >= 3, "socket descriptor, got {sock}");

    // connect(sock, sockaddr_in, 16)
    put_sockaddr_in(&mut harness, 0x4000, port);
    // SAFETY: the sockaddr is in range.
    assert_eq!(unsafe { abi::syscall::__syscall_connect(sock, Harness::ptr(0x4000), 16, 0, 0, 0) }, 0);

    // getsockname reports the ephemeral local port.
    // SAFETY: the out pointers are in range.
    assert_eq!(
        unsafe { abi::syscall::__syscall_getsockname(sock, Harness::ptr(0x4100), Harness::ptr(0x4110), 0, 0, 0) },
        0
    );
    let local_port = u16::from_be_bytes([harness.memory[0x4102], harness.memory[0x4103]]);
    assert_ne!(local_port, 0, "the local port is bound");

    // sendto on a connected stream ignores the address and writes the payload.
    harness.put(0x4200, b"ping");
    // SAFETY: the payload is in range; the address form is unused for TCP.
    let sent = unsafe { abi::syscall::__syscall_sendto(sock, Harness::ptr(0x4200), 4, 0, std::ptr::null(), 0) };
    assert_eq!(sent, 4, "four bytes written, got {sent}");

    // recvfrom reads the reply.
    // SAFETY: the destination buffer is in range.
    let received = unsafe { abi::syscall::__syscall_recvfrom(sock, Harness::ptr(0x4300), 4, 0, std::ptr::null_mut(), std::ptr::null_mut()) };
    assert_eq!(received, 4, "four bytes read, got {received}");
    assert_eq!(harness.get(0x4300, 4), b"pong");

    // SO_ERROR reads as 0; any other option is EOPNOTSUPP.
    // SAFETY: the out pointers are in range.
    assert_eq!(
        unsafe { abi::syscall::__syscall_getsockopt(sock, 1, 4, Harness::ptr(0x4400), Harness::ptr(0x4410), 0) },
        0
    );
    // SAFETY: as above.
    assert_eq!(
        unsafe { abi::syscall::__syscall_getsockopt(sock, 1, 999, Harness::ptr(0x4400), Harness::ptr(0x4410), 0) },
        -fd::EOPNOTSUPP
    );
    // setsockopt is accepted for a socket and rejected for a file.
    // SAFETY: `optval` is not read.
    assert_eq!(unsafe { abi::syscall::__syscall_setsockopt(sock, 6, 1, std::ptr::null(), 4, 0) }, 0);

    // shutdown(SHUT_WR) then close.
    // SAFETY: no memory is passed.
    assert_eq!(unsafe { abi::syscall::__syscall_shutdown(sock, 1, 0, 0, 0, 0) }, 0);
    // SAFETY: as above.
    assert_eq!(unsafe { abi::wasi::fd_close(sock) }, 0);

    assert_eq!(server.join().expect("server thread"), *b"ping");
}

#[test]
fn udp_socket_sends_and_receives_datagrams() {
    let _serial = serial();
    let mut harness = Harness::new("udp");

    let peer = std::net::UdpSocket::bind("127.0.0.1:0").expect("bind peer");
    let peer_port = peer.local_addr().expect("local addr").port();

    // socket(AF_INET, SOCK_DGRAM, 0) binds an ephemeral local port.
    // SAFETY: no memory is passed.
    let sock = unsafe { abi::syscall::__syscall_socket(2, 2, 0, 0, 0, 0) };
    assert!(sock >= 3, "udp descriptor, got {sock}");

    harness.put(0x5000, b"dgram");
    put_sockaddr_in(&mut harness, 0x5100, peer_port);
    // SAFETY: payload and address are inside the reservation.
    let sent = unsafe { abi::syscall::__syscall_sendto(sock, Harness::ptr(0x5000), 5, 0, Harness::ptr(0x5100), 16) };
    assert_eq!(sent, 5, "datagram sent, got {sent}");

    let mut buf = [0u8; 16];
    let (n, from) = peer.recv_from(&mut buf).expect("peer receives");
    assert_eq!(&buf[..n], b"dgram");

    // Reply, then receive it through recvfrom with the source address filled in.
    peer.send_to(b"reply", from).expect("peer replies");
    // SAFETY: all out pointers are in range.
    let received = unsafe {
        abi::syscall::__syscall_recvfrom(
            sock,
            Harness::ptr(0x5200),
            5,
            0,
            Harness::ptr(0x5300),
            Harness::ptr(0x5310),
        )
    };
    assert_eq!(received, 5, "datagram received, got {received}");
    assert_eq!(harness.get(0x5200, 5), b"reply");
    assert_eq!(harness.memory[0x5300], 2, "AF_INET reported back");
    assert_eq!(harness.u32_at(0x5310), 16, "socklen reported back");
    // SAFETY: no memory is passed.
    assert_eq!(unsafe { abi::wasi::fd_close(sock) }, 0);
}

// ---------------------------------------------------------------------------
// environ
// ---------------------------------------------------------------------------

#[test]
fn environ_calls_describe_the_process_environment() {
    let _serial = serial();
    let harness = Harness::new("environ");
    let _ = &harness;

    // SAFETY: both out pointers are inside the reservation.
    let rc = unsafe { abi::wasi::environ_sizes_get(Harness::ptr(0x6000), Harness::ptr(0x6008)) };
    assert_eq!(rc, 0);
    let count = harness.u64_at(0x6000);
    let bytes = harness.u64_at(0x6008);
    assert_eq!(count, std::env::vars().count() as u64);
    assert!(bytes >= count * 2, "every entry needs at least KEY=\\0");

    let ptr_array = 0x6100usize;
    let buf = 0x40000usize;
    // SAFETY: the array holds `count` u64s and the string buffer is large enough
    // for the environment as reported above.
    let rc = unsafe { abi::wasi::environ_get(Harness::ptr(ptr_array), Harness::ptr(buf)) };
    assert_eq!(rc, 0);
    let first_offset = harness.u64_at(ptr_array) as usize;
    let first = abi::mem::cstr(Harness::ptr(first_offset)).expect("first entry readable");
    assert!(first.contains('='), "KEY=VALUE form, got {first:?}");
}

// ---------------------------------------------------------------------------
// emscripten runtime glue
// ---------------------------------------------------------------------------

#[test]
fn heap_imports_describe_the_reservation() {
    let _serial = serial();
    let _harness = Harness::new("heap");

    assert_eq!(abi::emscripten::emscripten_get_heap_max(), MEMORY_LEN as u64);
    assert_eq!(abi::emscripten::emscripten_resize_heap(MEMORY_LEN as u64), 1);
    assert_eq!(
        abi::emscripten::emscripten_resize_heap(MEMORY_LEN as u64 + 1),
        0,
        "the reservation is a hard limit"
    );
    assert_eq!(abi::emscripten::emscripten_stack_snapshot(), 0);
    // SAFETY: the output buffer is inside the reservation.
    assert_eq!(
        unsafe { abi::emscripten::emscripten_stack_unwind_buffer(0, Harness::ptr(0x6800), 16) },
        0
    );
}

#[test]
fn logging_and_inline_asm_imports_do_not_trap() {
    let _serial = serial();
    let mut harness = Harness::new("logging");

    let line = harness.put_str("streaming: rpf index missing");
    // SAFETY: the string is NUL terminated and in range.
    unsafe { abi::emscripten::wasm_print_line_js(line) };
    // SAFETY: as above.
    unsafe { abi::emscripten::wasm_hang_line_js(line) };
    assert!(abi::state::count(Counter::HangLine) >= 1);

    // An unreadable string is dropped, not dereferenced.
    // SAFETY: deliberately out of range.
    unsafe { abi::emscripten::wasm_print_line_js(Harness::ptr(usize::MAX >> 1)) };

    // EM_ASM has no native table: answers 0 and is counted.
    let before = abi::state::count(Counter::AsmConst);
    // SAFETY: neither argument is dereferenced.
    let value = unsafe {
        abi::emscripten::emscripten_asm_const_int(1, std::ptr::null(), std::ptr::null_mut())
    };
    assert_eq!(value, 0);
    assert_eq!(abi::state::count(Counter::AsmConst), before + 1);
    // SAFETY: as above.
    assert_eq!(
        unsafe {
            abi::emscripten::emscripten_asm_const_int_sync_on_main_thread(
                2,
                std::ptr::null(),
                std::ptr::null_mut(),
            )
        },
        0
    );
    // SAFETY: as above.
    assert_eq!(
        unsafe {
            abi::emscripten::_emscripten_receive_on_main_thread_js(
                3,
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                0,
                0,
                0,
            )
        },
        0.0
    );

    assert!(abi::emscripten::emscripten_num_logical_cores() >= 1);
    abi::emscripten::emscripten_check_blocking_allowed();
    abi::emscripten::emscripten_exit_with_live_runtime();
}

#[test]
fn time_zone_and_calendar_imports_agree_with_utc() {
    let _serial = serial();
    let mut harness = Harness::new("calendar");

    // 2023-11-14T22:13:20Z
    let seconds: i64 = 1_700_000_000;
    harness.put(0x7000, &seconds.to_le_bytes());
    // SAFETY: time and tm buffers are in range.
    assert_eq!(
        unsafe { abi::emscripten::_gmtime_js(Harness::ptr(0x7000), Harness::ptr(0x7040)) },
        0
    );
    assert_eq!(harness.i32_at(0x7040), 20, "tm_sec");
    assert_eq!(harness.i32_at(0x7044), 13, "tm_min");
    assert_eq!(harness.i32_at(0x7048), 22, "tm_hour");
    assert_eq!(harness.i32_at(0x704c), 14, "tm_mday");
    assert_eq!(harness.i32_at(0x7050), 10, "tm_mon (0-based November)");
    assert_eq!(harness.i32_at(0x7054), 123, "tm_year (since 1900)");

    // mktime is the inverse over UTC.
    // SAFETY: the tm buffer is in range.
    let round_trip = unsafe { abi::emscripten::_mktime_js(Harness::ptr(0x7040)) };
    assert_eq!(round_trip, seconds, "gmtime -> mktime round trip");

    // _localtime_js is UTC on this host, and says so via tm_gmtoff = 0.
    // SAFETY: as above.
    assert_eq!(
        unsafe { abi::emscripten::_localtime_js(Harness::ptr(0x7000), Harness::ptr(0x7080)) },
        0
    );
    assert_eq!(harness.get(0x7080, 33), harness.get(0x7040, 33), "same fields as gmtime");
    assert_eq!(harness.u64_at(0x70a8), 0, "tm_gmtoff is 0 (documented UTC-only)");

    // _tzset_js reports a fixed UTC zone.
    // SAFETY: both out buffers are in range; `tz` is an input.
    unsafe {
        abi::emscripten::_tzset_js(
            Harness::ptr(0x7100),
            Harness::ptr(0x7110),
            Harness::ptr(0x7114),
            Harness::ptr(0x7120),
        )
    };
    assert_eq!(harness.u64_at(0x7100), 0, "tzname[0] unset → empty");
    assert_eq!(harness.u32_at(0x7110), 0, "daylight = 0");
    assert_eq!(harness.u32_at(0x7114), 0, "timezone = 0 (UTC)");

    // A pre-epoch timestamp still round-trips.
    let before: i64 = -86_400;
    harness.put(0x7008, &before.to_le_bytes());
    // SAFETY: as above.
    assert_eq!(
        unsafe { abi::emscripten::_gmtime_js(Harness::ptr(0x7008), Harness::ptr(0x7200)) },
        0
    );
    // SAFETY: as above.
    assert_eq!(
        unsafe { abi::emscripten::_mktime_js(Harness::ptr(0x7200)) },
        before
    );
}

// ---------------------------------------------------------------------------
// Threads, indirect calls, exit
// ---------------------------------------------------------------------------

static THREAD_RAN: AtomicI32 = AtomicI32::new(0);

extern "C" fn thread_entry(arg: u64) -> u64 {
    THREAD_RAN.store(arg as i32, Ordering::SeqCst);
    arg
}

#[test]
fn pthread_create_js_runs_a_real_thread() {
    let _serial = serial();
    let _harness = Harness::new("pthread");
    THREAD_RAN.store(0, Ordering::SeqCst);

    // SAFETY: `thread_entry` has the pthread start-routine signature and is kept
    // alive for the whole process (a `fn` item address).
    let rc = unsafe {
        abi::emscripten::__pthread_create_js(
            Harness::ptr(0x8000),
            std::ptr::null(),
            thread_entry as *const () as usize as u64,
            77,
        )
    };
    assert_eq!(rc, 0, "thread created");
    assert!(abi::mem::read_u64(Harness::ptr(0x8000)).unwrap_or(0) >= 1, "handle id written");

    for _ in 0..200 {
        if THREAD_RAN.load(Ordering::SeqCst) == 77 {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    assert_eq!(THREAD_RAN.load(Ordering::SeqCst), 77, "the entry point ran");

    // A non-callable pointer is refused, not transmuted.
    // SAFETY: the argument is a bogus address and must be rejected before use.
    let bad = unsafe {
        abi::emscripten::__pthread_create_js(Harness::ptr(0x8008), std::ptr::null(), 1, 0)
    };
    assert_eq!(bad, -22, "unresolvable entry answers EINVAL");

    // The mailbox/thread bookkeeping imports are no-ops.
    // SAFETY: none of them dereference their arguments.
    unsafe {
        abi::emscripten::_emscripten_thread_set_strongref(Harness::ptr(0x8010));
        abi::emscripten::_emscripten_init_main_thread_js(Harness::ptr(0x8018));
        abi::emscripten::_emscripten_thread_mailbox_await(Harness::ptr(0x8020));
        abi::emscripten::_emscripten_notify_mailbox_postmessage(
            Harness::ptr(0x8028),
            Harness::ptr(0x8030),
        );
        abi::emscripten::_emscripten_thread_cleanup(Harness::ptr(0x8038));
    }
    assert!(abi::state::count(Counter::MailboxPost) >= 1);
}

#[test]
fn emscripten_system_runs_a_real_command() {
    let _serial = serial();
    let mut harness = Harness::new("system");

    let ok = harness.put_str("exit 0");
    // SAFETY: the command string is NUL terminated and in range.
    assert_eq!(unsafe { abi::emscripten::_emscripten_system(ok) }, 0);

    let failing = harness.put_str("exit 3");
    // SAFETY: as above.
    assert_eq!(
        unsafe { abi::emscripten::_emscripten_system(failing) },
        3,
        "the shell exit status is reported"
    );

    // Unreadable command → -EFAULT, not a panic.
    // SAFETY: deliberately out of range.
    assert_eq!(
        unsafe { abi::emscripten::_emscripten_system(Harness::ptr(usize::MAX >> 1)) },
        -fd::EFAULT
    );
}

// ---------------------------------------------------------------------------
// invoke_* trampolines
// ---------------------------------------------------------------------------

static SUM: AtomicI64 = AtomicI64::new(0);
static LAST: AtomicU64 = AtomicU64::new(0);

extern "C" fn cb_vjj(a: u64, b: u64) {
    SUM.store((a + b) as i64, Ordering::SeqCst);
}
extern "C" fn cb_ij(a: u64) -> u32 {
    a as u32 + 1
}
extern "C" fn cb_vj(a: u64) {
    LAST.store(a, Ordering::SeqCst);
}
extern "C" fn cb_vjij(a: u64, b: u32, c: u64) {
    SUM.store((a + u64::from(b) + c) as i64, Ordering::SeqCst);
}
extern "C" fn cb_jjij(a: u64, b: u32, c: u64) -> u64 {
    a + u64::from(b) + c
}
extern "C" fn cb_vjii(a: u64, b: u32, c: u32) {
    SUM.store((a + u64::from(b) + u64::from(c)) as i64, Ordering::SeqCst);
}
extern "C" fn cb_iji(a: u64, b: u32) -> u32 {
    a as u32 + b
}
extern "C" fn cb_vjjii(a: u64, b: u64, c: u32, d: u32) {
    SUM.store((a + b + u64::from(c) + u64::from(d)) as i64, Ordering::SeqCst);
}
extern "C" fn cb_jjiiijij(a: u64, b: u32, c: u32, d: u32, e: u64, f: u32, g: u64) -> u64 {
    a + u64::from(b + c + d) + e + u64::from(f) + g
}
extern "C" fn cb_ijji(a: u64, b: u64, c: u32) -> u32 {
    (a + b) as u32 + c
}
extern "C" fn cb_ijjiii(a: u64, b: u64, c: u32, d: u32, e: u32) -> u32 {
    (a + b) as u32 + c + d + e
}

#[test]
fn every_invoke_trampoline_dispatches_to_a_native_function() {
    let _serial = serial();
    let _harness = Harness::new("invoke");
    let addr = |f: usize| f as u64;

    // SAFETY: every pointer is the address of a function with exactly the
    // signature the trampoline expects (checked by construction).
    unsafe {
        abi::emscripten::invoke_vjj(addr(cb_vjj as *const () as usize), 2, 3);
        assert_eq!(SUM.load(Ordering::SeqCst), 5);

        assert_eq!(abi::emscripten::invoke_ij(addr(cb_ij as *const () as usize), 41), 42);

        abi::emscripten::invoke_vj(addr(cb_vj as *const () as usize), 7);
        assert_eq!(LAST.load(Ordering::SeqCst), 7);

        abi::emscripten::invoke_vjij(addr(cb_vjij as *const () as usize), 1, 2, 3);
        assert_eq!(SUM.load(Ordering::SeqCst), 6);

        assert_eq!(abi::emscripten::invoke_jjij(addr(cb_jjij as *const () as usize), 1, 2, 3), 6);

        abi::emscripten::invoke_vjii(addr(cb_vjii as *const () as usize), 1, 2, 3);
        assert_eq!(SUM.load(Ordering::SeqCst), 6);

        assert_eq!(abi::emscripten::invoke_iji(addr(cb_iji as *const () as usize), 40, 2), 42);

        abi::emscripten::invoke_vjjii(addr(cb_vjjii as *const () as usize), 1, 2, 3, 4);
        assert_eq!(SUM.load(Ordering::SeqCst), 10);

        assert_eq!(
            abi::emscripten::invoke_jjiiijij(addr(cb_jjiiijij as *const () as usize), 1, 2, 3, 4, 5, 6, 7),
            1 + 9 + 5 + 6 + 7
        );

        assert_eq!(abi::emscripten::invoke_ijji(addr(cb_ijji as *const () as usize), 20, 20, 2), 42);

        assert_eq!(
            abi::emscripten::invoke_ijjiii(addr(cb_ijjiii as *const () as usize), 20, 19, 1, 1, 1),
            42
        );
    }

    // A table-index-shaped target that was never registered answers the zero
    // value instead of jumping into the void.
    // SAFETY: `3` is below the pointer threshold and unregistered, so the
    // trampoline never transmutes it.
    assert_eq!(unsafe { abi::emscripten::invoke_ij(3, 1) }, 0);

    // Registering a table slot makes the same call dispatch.
    abi::emscripten::register_indirect(3, cb_ij as *const () as usize);
    // SAFETY: slot 3 now maps to `cb_ij`, whose signature matches.
    assert_eq!(unsafe { abi::emscripten::invoke_ij(3, 9) }, 10);
}

// ---------------------------------------------------------------------------
// exit
// ---------------------------------------------------------------------------

static EXIT_CODE: AtomicI32 = AtomicI32::new(-1);

#[test]
fn exit_imports_are_interceptable() {
    let _serial = serial();
    let _harness = Harness::new("exit");

    abi::set_exit_hook(Some(Box::new(|code| {
        EXIT_CODE.store(code as i32, Ordering::SeqCst);
    })));
    abi::wasi::proc_exit(7);
    assert_eq!(EXIT_CODE.load(Ordering::SeqCst), 7, "proc_exit delivered 7");
    abi::wasi::exit(9);
    assert_eq!(EXIT_CODE.load(Ordering::SeqCst), 9, "env.exit shares the hook");
    abi::set_exit_hook(None);
    assert!(!abi::state::has_exit_hook(), "hook removed");
}

// ---------------------------------------------------------------------------
// The import manifest itself
// ---------------------------------------------------------------------------

/// `ABI-MAP.md`, embedded so the documentation cannot drift from the code.
const ABI_MAP: &str = include_str!("../ABI-MAP.md");

#[test]
fn the_import_manifest_matches_the_binary_and_the_documentation() {
    assert_eq!(abi::IMPORT_COUNT, 86, "86 entries in the import section");
    assert_eq!(abi::FUNCTION_IMPORT_COUNT, 85, "85 function imports + memory");
    assert_eq!(abi::IMPORTS[0].1, "memory", "index 0 is the memory import");

    let mut seen = std::collections::BTreeSet::new();
    for (index, name, module) in abi::IMPORTS {
        assert!(
            seen.insert(name),
            "duplicate import name {name} at index {index}"
        );
        assert!(
            module == "env" || module == "wasi_snapshot_preview1",
            "unexpected module {module}"
        );
        assert!(
            ABI_MAP.contains(name),
            "{name} (index {index}) is missing from ABI-MAP.md"
        );
    }
    assert_eq!(seen.len(), 86);
    assert!(ABI_MAP.contains("| real |"), "the map classifies implementations");
    assert!(ABI_MAP.contains("| no-op |"), "the map classifies no-ops");
}
