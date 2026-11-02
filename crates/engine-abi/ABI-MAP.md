# ABI-MAP — every wasm import of the recovered engine, and what replaces it

Source of truth: the import section of the wasm64 module (`/tmp/game.wat`, lines
1971–2056). 86 entries: **1 memory** + **85 functions**. Signatures were read out
of the module's type table, not guessed from the import names — several of them
are wider than the C call they serve (emscripten's syscall imports carry its
JS-side padding), and this map records the *declared* wasm type.

Implementation crate: `crates/engine-abi`. Every row is a `#[no_mangle]
extern "C"` function whose Rust parameter list is byte-compatible with the wasm
type: `i64` pointers become `*mut u8` (same width, no truncation), `i64` scalars
become `u64`, `i32` becomes `u32`/`i32`, `f64` stays `f64`. Pointers are
**linear-memory offsets**, resolved against the mapping published by
`engine_abi::set_memory`; nothing is dereferenced unvalidated.

## Totals

| class | count | meaning |
|---|---|---|
| **real** | **69** | Performs the import's job: delegates to `engine_host::Host`, to `std::fs`/`std::net`, or to native code. |
| **no-op** | **16** | Deliberately does nothing, with the reason recorded below: the effect only existed to talk to a browser worker, or the concept has no native counterpart. |
| **not a function** | **1** | Index 0, the shared `memory` import, which the embedding runtime supplies rather than this crate. |

No-op is not "unimplemented": each one is a decision, and each is counted at
runtime (`engine_abi::state::count`) or logged, so an operator can see the call.

## Legend

* **real** — implemented.
* **no-op** — explicit, documented no-op.
* Signatures are the wasm type as declared (`(param …) (result …)`), with `()`
  for no result.
* "impl" is the Rust path inside `engine_abi` (`time::wasm_now_ms` →
  `engine_abi::time::wasm_now_ms`); every symbol is exported `#[no_mangle]` under
  the bare import name.

## Import table

| # | wasm sig | import | impl (Rust) | class | notes |
|---:|---|---|---|---|---|
| 0 | `memory i64 49152 262144 shared` | `memory` | `—` | n/a | the shared linear memory; the embedding runtime allocates it and publishes the base via `set_memory` |
| 1 | `(result f64)` | `wasm_now_ms` | `time::wasm_now_ms` | real | `Host::now_micros`/1000 — monotonic ms; 51 call sites in the module |
| 2 | `(param i64)` | `wasm_hang_line_js` | `emscripten::wasm_hang_line_js` | real | NUL-terminated line → `Host::log(Error)`; counted (the engine itself then blocks) |
| 3 | `(param i64 i32)` | `wasm_audio_publish_js` | `audio::wasm_audio_publish_js` | real | descriptor `{data_ptr@0, byte_len@8}` or direct buffer → `AudioSink::publish`; null sink counts the block |
| 4 | `(param i64)` | `wasm_httpfs_io_start_js` | `httpfs::wasm_httpfs_io_start_js` | no-op | woke `io_worker.js` to pump its request queue; server asset reads are synchronous, so there is no worker to wake — counted |
| 5 | `(param i64 i32) (result i32)` | `wasm_httpfs_manifest_js` | `httpfs::wasm_httpfs_manifest_js` | real | query (ptr=0 → needed bytes) then fill (ptr,cap → bytes written, NUL-terminated) from `AssetStore::manifest().to_text()` |
| 6 | `(result i32)` | `wasm_userdata_page_js` | `userdata::wasm_userdata_page_js` | real | row count of the userdata store (`page(None, 4096)`), −1 when unreadable |
| 7 | `(param i64 i64 i64 f64)` | `wasm_userdata_put_js` | `userdata::wasm_userdata_put_js` | real | `(key_cstr, data, len, ts)` → `UserdataStore::put`; the f64 timestamp is advisory and dropped |
| 8 | `(param i64)` | `wasm_userdata_delete_js` | `userdata::wasm_userdata_delete_js` | real | `UserdataStore::delete`; a missing key is not an error |
| 9 | `(param i64 i64 i64)` | `wgpu_start_worker` | `gpu::wgpu_start_worker` | no-op | spun up `wgpu_worker.js` over a 32 MiB shared ring; headless has no device — `Host::gpu` drains the ring in-process instead; counted |
| 10 | `(result i32)` | `wasm_has_page_js` | `userdata::wasm_has_page_js` | real | "persistent storage available?" — 1 when the store lists, else 0 |
| 11 | `(param i64)` | `wasm_input_publish_js` | `input::wasm_input_publish_js` | no-op | engine→browser direction (input worklet + debug overlay); the server has no observer and `Host::input` is the opposite direction — counted |
| 12 | `(param i64 i32) (result i32)` | `wasm_module_int_js` | `module_int::wasm_module_int_js` | real | `(key_cstr, fallback)` → `Host::module_int`, falling back when the key is unset |
| 13 | `(result f64)` | `emscripten_get_now` | `emscripten_get_now` | real | same monotonic timeline as `wasm_now_ms` (they were never allowed to disagree) |
| 14 | `(param i64)` | `wasm_print_line_js` | `emscripten::wasm_print_line_js` | real | NUL-terminated line → `Host::log(Info, "engine")` |
| 15 | `(result i64)` | `emscripten_stack_snapshot` | `emscripten::emscripten_stack_snapshot` | no-op | captured the wasm shadow-stack pointer for a JS backtrace walker; native unwinding makes the handle meaningless → 0 = "no snapshot" |
| 16 | `(param i64 i64 i32) (result i32)` | `emscripten_stack_unwind_buffer` | `emscripten::emscripten_stack_unwind_buffer` | no-op | counterpart of the snapshot above → 0 frames, the documented value for an unusable handle |
| 17 | `(param i64 i64 i64) (result i32)` | `emscripten_asm_const_int` | `emscripten::emscripten_asm_const_int` | no-op | `EM_ASM` snippet table lived in `game.js` and was UI-only (canvas, cursor, clipboard); the native port has that logic in Rust → 0, counted, warned once per code |
| 18 | `(result i64)` | `emscripten_get_heap_max` | `emscripten::emscripten_get_heap_max` | real | size of the published linear-memory reservation |
| 19 | `(param i32)` | `exit` | `wasi::exit` | real | `std::process::exit`, or the installed exit hook (tests/embedders) |
| 20 | `(param i64 i64 i64)` | `invoke_vjj` | `emscripten::invoke_vjj` | real | indirect-call trampoline → native `extern "C" fn(u64,u64)` |
| 21 | `(param i64 i64) (result i32)` | `invoke_ij` | `emscripten::invoke_ij` | real | → `fn(u64) -> u32` |
| 22 | `(param i64 i64)` | `invoke_vj` | `emscripten::invoke_vj` | real | → `fn(u64)` |
| 23 | `(param i64 i64 i32 i64)` | `invoke_vjij` | `emscripten::invoke_vjij` | real | → `fn(u64,u32,u64)` |
| 24 | `(param i64 i64 i32 i64) (result i64)` | `invoke_jjij` | `emscripten::invoke_jjij` | real | → `fn(u64,u32,u64) -> u64` |
| 25 | `(param i64 i64 i32 i32)` | `invoke_vjii` | `emscripten::invoke_vjii` | real | → `fn(u64,u32,u32)` |
| 26 | `(param i64 i64 i32) (result i32)` | `invoke_iji` | `emscripten::invoke_iji` | real | → `fn(u64,u32) -> u32` |
| 27 | `(param i64 i64 i64 i32 i32)` | `invoke_vjjii` | `emscripten::invoke_vjjii` | real | → `fn(u64,u64,u32,u32)` |
| 28 | `(param i64 i64 i32 i32 i32 i64 i32 i64) (result i64)` | `invoke_jjiiijij` | `emscripten::invoke_jjiiijij` | real | → `fn(u64,u32,u32,u32,u64,u32,u64) -> u64` |
| 29 | `(param i64 i64 i64 i32) (result i32)` | `invoke_ijji` | `emscripten::invoke_ijji` | real | → `fn(u64,u64,u32) -> u32` |
| 30 | `(param i64 i64 i64 i32 i32 i32) (result i32)` | `invoke_ijjiii` | `emscripten::invoke_ijjiii` | real | → `fn(u64,u64,u32,u32,u32) -> u32` |
| 31 | `(param i64 i64 i64) (result i32)` | `emscripten_asm_const_int_sync_on_main_thread` | `emscripten::emscripten_asm_const_int_sync_on_main_thread` | no-op | same UI-only snippet table, proxied to the main thread; there is no main thread to proxy to → 0, counted |
| 32 | `(param i32)` | `proc_exit` | `wasi::proc_exit` | real | `std::process::exit` / exit hook |
| 33 | `(param i64 i64 i64 i64)` | `_tzset_js` | `emscripten::_tzset_js` | real | fills `tzname[2]`, `daylight`, `timezone` as fixed UTC (paired with the UTC-only `_localtime_js`) |
| 34 | `()` | `_abort_js` | `emscripten::_abort_js` | real | logs then `std::process::abort()` — the trap-equivalent, deliberately not routed through the exit hook |
| 35 | `(param i32 i64 i64) (result i32)` | `clock_time_get` | `time::__wasi_clock_time_get` | real | realtime = `SystemTime`, monotonic = `Host::now_nanos`; unknown ids → 28 (EINVAL) |
| 36 | `(param i32) (result i32)` | `fd_close` | `wasi::fd_close` | real | flushes and drops the descriptor; second close → 8 (BADF) |
| 37 | `(result f64)` | `emscripten_date_now` | `emscripten_date_now` | real | wall clock ms since the epoch (`Date.now`) |
| 38 | `(param i64)` | `_emscripten_thread_set_strongref` | `emscripten::_emscripten_thread_set_strongref` | no-op | JS worker refcount; a `std::thread` is kept alive by the handle `Host::spawn` owns |
| 39 | `()` | `emscripten_exit_with_live_runtime` | `emscripten::emscripten_exit_with_live_runtime` | no-op | ended `set_main_loop` but kept the runtime alive for JS callbacks; returning from the frame is the native equivalent |
| 40 | `(param i32 i32 i64) (result i32)` | `__syscall_fcntl64` | `syscall::__syscall_fcntl64` | real | F_GETFD/F_SETFD/F_GETFL/F_SETFL → 0; anything else → −EINVAL; unknown fd → −EBADF |
| 41 | `(param i32 i64 i32 i64) (result i32)` | `__syscall_openat` | `syscall::__syscall_openat` | real | `OpenOptions` against the resolved host path; O_DIRECTORY opens a listing; errno-mapped failures |
| 42 | `(param i32 i32 i64) (result i32)` | `__syscall_ioctl` | `syscall::__syscall_ioctl` | real | −ENOTTY: nothing in the module is a terminal, which is the correct answer, not a stub |
| 43 | `(param i32 i64 i64 i64) (result i32)` | `fd_write` | `wasi::fd_write` | real | writes the iovec array in order to file/socket/stdio, reports the u64 total |
| 44 | `(param i32 i64 i64 i64) (result i32)` | `fd_read` | `wasi::fd_read` | real | fills iovecs, stops at the first short read (`readv` contract) |
| 45 | `(param i32 i64) (result i32)` | `__syscall_fstat64` | `syscall::__syscall_fstat64` | real | `Metadata` → wasm64 `struct stat`; sockets/stdio get a synthetic S_IFSOCK record |
| 46 | `(param i64 i64) (result i32)` | `__syscall_stat64` | `syscall::__syscall_stat64` | real | `std::fs::metadata` (follows symlinks) → `struct stat` |
| 47 | `(param i32 i64 i64 i32) (result i32)` | `__syscall_newfstatat` | `syscall::__syscall_newfstatat` | real | honours AT_SYMLINK_NOFOLLOW and AT_EMPTY_PATH |
| 48 | `(param i64 i64) (result i32)` | `__syscall_lstat64` | `syscall::__syscall_lstat64` | real | `symlink_metadata` → `struct stat` |
| 49 | `(param i32 i64) (result i32)` | `__syscall_ftruncate64` | `syscall::__syscall_ftruncate64` | real | `File::set_len` (the save writer truncates in place) |
| 50 | `(param i64 i64) (result i32)` | `__syscall_getcwd` | `syscall::__syscall_getcwd` | real | absolute `current_dir()`, NUL-terminated, length returned |
| 51 | `(param i64 i64) (result i32)` | `environ_sizes_get` | `wasi::environ_sizes_get` | real | `std::env::vars` count and byte size (u64 each, wasm64 `size_t`) |
| 52 | `(param i64 i64) (result i32)` | `environ_get` | `wasi::environ_get` | real | writes the offset array and the `KEY=VALUE\0` buffer |
| 53 | `(param i64)` | `_emscripten_init_main_thread_js` | `emscripten::_emscripten_init_main_thread_js` | no-op | published the main thread TLS block to the JS thread registry; each native thread has platform TLS |
| 54 | `(param i64)` | `_emscripten_thread_mailbox_await` | `emscripten::_emscripten_thread_mailbox_await` | no-op | blocked on a JS mailbox (proxy-then-block); native threads hand work over directly, blocking here would deadlock — counted |
| 55 | `(param i32 i64 i32 i64) (result i32)` | `fd_seek` | `wasi::fd_seek` | real | `Seek` over the file; sockets/streams → 70 (SPIPE) |
| 56 | `(param i32 i64 i32) (result i32)` | `__syscall_mkdirat` | `syscall::__syscall_mkdirat` | real | `create_dir_all` (the engine creates save directories with parents) |
| 57 | `(param i64) (result i64)` | `_mktime_js` | `emscripten::_mktime_js` | real | inverse of gmtime over UTC, normalising out-of-range month/day |
| 58 | `(param i64 i64) (result i32)` | `_localtime_js` | `emscripten::_localtime_js` | real | real conversion, **UTC only** (`tm_gmtoff = 0` advertises it): the engine compares epoch values |
| 59 | `(param i64 i64) (result i32)` | `_gmtime_js` | `emscripten::_gmtime_js` | real | real UTC conversion (same code path as `_localtime_js`) |
| 60 | `(param i32 i64 i64 i64 i64) (result i32)` | `fd_pread` | `wasi::fd_pread` | real | positional read that saves/restores the cursor, so the asset loader cannot corrupt it |
| 61 | `(param i32 i64 i64 i32 i64 i64 i64) (result f64)` | `_emscripten_receive_on_main_thread_js` | `emscripten::_emscripten_receive_on_main_thread_js` | no-op | re-entered the JS snippet table on the main thread; there is no proxy target and no table → 0.0, counted |
| 62 | `()` | `emscripten_check_blocking_allowed` | `emscripten::emscripten_check_blocking_allowed` | no-op | asserted "blocking is legal off the main thread"; every server thread may block, so the check always passes |
| 63 | `(param i64 i64 i64 i64) (result i32)` | `__pthread_create_js` | `emscripten::__pthread_create_js` | real | `Host::spawn` runs the native start routine; handle id written to `pthread *`; bad pointer → −EINVAL |
| 64 | `(param i64)` | `_emscripten_thread_cleanup` | `emscripten::_emscripten_thread_cleanup` | no-op | told the JS pool to recycle the worker; the native thread simply returns |
| 65 | `(param i32 i64 i64 i64 i64) (result i32)` | `fd_pwrite` | `wasi::fd_pwrite` | real | positional write with the same cursor discipline as `fd_pread` |
| 66 | `(param i32 i64 i64) (result i32)` | `__syscall_getdents64` | `syscall::__syscall_getdents64` | real | snapshot listing (`.`/`..` first) encoded as `linux_dirent64` records, 0 at the end |
| 67 | `(param i32 i64 i32) (result i32)` | `__syscall_unlinkat` | `syscall::__syscall_unlinkat` | real | removes a file, or a directory under AT_REMOVEDIR |
| 68 | `(param i64) (result i32)` | `__syscall_rmdir` | `syscall::__syscall_rmdir` | real | `remove_dir` |
| 69 | `(param i32 i64 i32 i64) (result i32)` | `__syscall_renameat` | `syscall::__syscall_renameat` | real | `std::fs::rename` (atomic on one volume, which is the save path) |
| 70 | `(param i64 i32 i32) (result i32)` | `__syscall_poll` | `syscall::__syscall_poll` | real | per-fd readiness: files always ready, sockets probed with a non-consuming `peek`, unknown fd → POLLNVAL; 1 ms steps to the timeout |
| 71 | `(param i64 i64 i64) (result i32)` | `__syscall_statfs64` | `syscall::__syscall_statfs64` | real | synthetic record (bsize/frsize 4096, namelen 255, zero block counts) so no caller divides by zero |
| 72 | `(result i32)` | `emscripten_num_logical_cores` | `emscripten::emscripten_num_logical_cores` | real | `available_parallelism()` (was `navigator.hardwareConcurrency`) |
| 73 | `(param i64) (result i32)` | `_emscripten_system` | `emscripten::_emscripten_system` | real | runs the command through `sh -c` / `cmd /C`, returns the exit status |
| 74 | `(param i64 i64)` | `_emscripten_notify_mailbox_postmessage` | `emscripten::_emscripten_notify_mailbox_postmessage` | no-op | posted a JS message queue to another worker; there is no queue between native threads — counted |
| 75 | `(param i64) (result i32)` | `emscripten_resize_heap` | `emscripten::emscripten_resize_heap` | real | 1 when the request fits the reservation, 0 (logged) when it does not — "cannot grow" without aborting |
| 76 | `()` | `_emscripten_throw_longjmp` | `emscripten::_emscripten_throw_longjmp` | real | no native `setjmp` target exists, so it panics — the trap-equivalent failure, as the wasm build did |
| 77 | `(param i32 i64 i64 i32 i32 i32) (result i32)` | `__syscall_accept4` | `syscall::__syscall_accept4` | no-op | unreachable by this module: there is no `bind`/`listen` import, so no listening socket can exist; answers −EINVAL rather than pretending |
| 78 | `(param i32 i64 i32 i32 i32 i32) (result i32)` | `__syscall_connect` | `syscall::__syscall_connect` | real | `TcpStream::connect` for a STREAM descriptor (TCP_NODELAY on), UDP → −EOPNOTSUPP |
| 79 | `(param i32 i64 i64 i32 i32 i32) (result i32)` | `__syscall_getsockname` | `syscall::__syscall_getsockname` | real | `local_addr()` → `sockaddr_in{,6}` plus the socklen |
| 80 | `(param i32 i32 i32 i64 i64 i32) (result i32)` | `__syscall_getsockopt` | `syscall::__syscall_getsockopt` | real | SO_ERROR → 0 (connect failures are synchronous here); other options → −EOPNOTSUPP |
| 81 | `(param i32 i64 i64 i32 i64 i64) (result i32)` | `__syscall_recvfrom` | `syscall::__syscall_recvfrom` | real | `recv_from` on UDP (source address filled), `read` on a connected stream |
| 82 | `(param i32 i64 i64 i32 i64 i32) (result i32)` | `__syscall_sendto` | `syscall::__syscall_sendto` | real | `send_to` on UDP, `write` on a connected stream (address ignored, as `sendto` specifies) |
| 83 | `(param i32 i32 i32 i64 i32 i32) (result i32)` | `__syscall_setsockopt` | `syscall::__syscall_setsockopt` | real | validates the descriptor and accepts: the options were transport hints and std applies its own defaults, so nothing the engine can observe is lost |
| 84 | `(param i32 i32 i32 i32 i32 i32) (result i32)` | `__syscall_shutdown` | `syscall::__syscall_shutdown` | real | `Shutdown::{Read,Write,Both}` |
| 85 | `(param i32 i32 i32 i32 i32 i32) (result i32)` | `__syscall_socket` | `syscall::__syscall_socket` | real | STREAM → not-yet-connected descriptor, DGRAM → ephemeral-bound `UdpSocket` |

## The 16 no-ops, grouped by reason

| reason | imports | why it is safe |
|---|---|---|
| Browser worker handshake | `wasm_httpfs_io_start_js`, `wgpu_start_worker` | Both exist to wake a JS worker (`io_worker.js`, `wgpu_worker.js`) that read a queue out of linear memory. The server performs the same work synchronously through `Host::assets` and drains GPU commands through `Host::gpu`, so there is no worker and no queue. |
| Browser-only observer | `wasm_input_publish_js` | Pushes the engine's input snapshot *to* the browser (input worklet, debug overlay). Host→engine input is `Host::input`, a different direction, and works. |
| JS-side snippet table | `emscripten_asm_const_int`, `emscripten_asm_const_int_sync_on_main_thread`, `_emscripten_receive_on_main_thread_js` | `EM_ASM` bodies were strings in `game.js` (canvas sizing, cursor, clipboard). The native port has that logic compiled in; there is no table to index and no main thread to proxy to. Counted and warned once per code. |
| JS thread/pool bookkeeping | `_emscripten_thread_set_strongref`, `_emscripten_init_main_thread_js`, `_emscripten_thread_mailbox_await`, `_emscripten_notify_mailbox_postmessage`, `_emscripten_thread_cleanup`, `emscripten_check_blocking_allowed`, `emscripten_exit_with_live_runtime` | These are the proxying-queue protocol between workers. Native threads share memory directly, `Host::spawn` owns their lifetime, and a server thread is always allowed to block. Reproducing the protocol would add a mailbox that nothing reads. |
| Wasm shadow stack | `emscripten_stack_snapshot`, `emscripten_stack_unwind_buffer` | Captured a wasm stack pointer so JS could walk frames. Native Rust unwinding is not that stack, so `0` / zero frames (the documented "unusable snapshot" values) is the only honest answer. |
| Unreachable by this module | `__syscall_accept4` | The module imports no `bind`/`listen`, so it can never own a listening socket; it is a client. Answering `−EINVAL` matches what the browser build did outside that path. |

## Call-site evidence used for the signatures

The map is not purely nominal — where an import's meaning depends on its
arguments, the call sites in `/tmp/game.wat` were read:

* `wasm_audio_publish_js` @268980 — descriptor at `8153472`, `byte_len`
  (`16384`) stored at `+8`, capacity passed as the second argument.
* `wasm_input_publish_js` @268974 — single pointer to the engine's input block.
* `wgpu_start_worker` @321648 — `(ring_base, 32 MiB, input_state)`.
* `wasm_httpfs_manifest_js` @283889/@283905 — query with `(0, 0)`, then
  `malloc(needed + 1)` and fill; a negative answer aborts the load.
* `wasm_userdata_put_js` @272630 — `(key_cstr, data, len, seconds)`.
* `wasm_module_int_js` @268404 — `(key_cstr, fallback)`, fallback `1024`.
* `__syscall_socket`/`connect`/`sendto`/`recvfrom` — the leading parameters are the
  POSIX ones; the trailing `i32`s are emscripten's padding (that is why
  `__syscall_socket` is declared with six parameters).

## Struct layouts written into engine memory

| struct | where | layout |
|---|---|---|
| `struct stat` (wasm64) | `fd::encode_stat` | dev@0, ino@8, mode@16, nlink@20, uid@24, gid@28, rdev@32, size@40, blksize@48, blocks@56, atim@64, mtim@80, ctim@96 (sec/nsec pairs), pad@112, 128 bytes |
| `linux_dirent64` | `fd::encode_dirents` | d_ino@0, d_off@8, d_reclen@16, d_type@18, d_name@19, 8-byte aligned |
| `sockaddr_in` / `_in6` | `fd::decode_sockaddr` / `encode_sockaddr` | family@0 (LE), port@2 (BE), addr@4 (v4) or flowinfo@4 + addr@8 + scope@24 (v6) |
| `struct tm` (wasm64) | `emscripten::write_tm` | sec@0…isdst@32 (i32 × 9), gmtoff@40 (i64), zone@48 (i64) |
| `struct pollfd` | `syscall::__syscall_poll` | fd@0 (i32), events@4 (i16), revents@6 (i16) |
| `iovec` (wasm64) | `wasi::iovecs` | buf@0 (u64 offset), len@8 (u64) |
| `struct statfs` | `syscall::__syscall_statfs64` | type@0, bsize@8, blocks@16, bfree@24, bavail@32, files@40, ffree@48, fsid@56, namelen@64, frsize@72, flags@80 |

## Verification

```
cd crates/engine-abi
cargo test          # 20 integration tests: every implemented import called once
```

`tests/imports.rs` installs a real `HeadlessHost`, publishes a 1 MiB memory
reservation, and calls each implemented import with assertions on the documented
behaviour — including the negative cases (out-of-range pointers → `−EFAULT`,
closed descriptors → `−EBADF`, unregistered trampoline targets → `0`, a bad
`resize_heap` → `0`). It also embeds this file with `include_str!` and fails if a
name in `engine_abi::IMPORTS` is missing from the table above.
