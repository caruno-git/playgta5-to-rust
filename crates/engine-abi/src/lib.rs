//! # `engine-abi`
//!
//! Rust stand-ins for **every** wasm import of the reconstructed GTA V engine.
//!
//! The module being ported is a wasm64 binary with 86 entries in its import
//! section: one shared `memory`, and 85 functions that used to be JavaScript
//! (`game.js`, `wgpu_worker.js`, `io_worker.js`, `audio-worklet.js`, plus
//! emscripten's own `library_*.js`). The translated Rust engine still calls those
//! functions by name, so a server build needs them to exist, to be callable, and
//! to be backed by something real.
//!
//! ```text
//! generated engine code ──call──▶ engine_abi::wasm_now_ms()
//!                                        │
//!                                        ▼
//!                              engine_host::Host  (HeadlessHost on a server)
//! ```
//!
//! Nothing in this crate reaches for a browser, an async runtime or a global
//! singleton other than the two pieces of process state a free-function ABI
//! cannot avoid (see [`state`]): the installed [`engine_host::Host`] and the base
//! of the engine's linear memory.
//!
//! ## ABI rules
//!
//! * **Names are the import field names**, exported with `#[no_mangle]
//!   extern "C"` so the linker resolves the calls without a `.wat` in sight. The
//!   wasi imports also get `__wasi_*` aliases under the local symbol names the
//!   `.wat` uses.
//! * **Types are byte-compatible with the wasm signatures.** An `i64` that is a
//!   pointer becomes `*mut u8` / `*const u8` (same 64-bit width); an `i64` that is
//!   a scalar stays `u64`; `i32` → `u32`/`i32`, `f64` → `f64`. Never a widened
//!   struct, never an added length that the caller did not pass.
//! * **No hidden allocations on the calling convention.** Buffers the engine
//!   hands in are borrowed in place ([`mem`] range-checks them against the
//!   mapping). The only owned copies are where a host trait can only take
//!   ownership — `AudioBlock::samples`, userdata payloads and the manifest text —
//!   and each is called out in its function's docs.
//! * **No `unwrap` in library paths.** A bad pointer, a closed descriptor or a
//!   host error becomes the documented error return (`-errno`, `0`, `-1`). Only
//!   two paths abort the process on purpose (`_abort_js`,
//!   `_emscripten_throw_longjmp`), because the wasm build trapped there too.
//!
//! ## Module map
//!
//! | module | imports |
//! |---|---|
//! | [`time`] | `wasm_now_ms`, `emscripten_get_now`, `emscripten_date_now`, `clock_time_get` |
//! | [`audio`] | `wasm_audio_publish_js` |
//! | [`httpfs`] | `wasm_httpfs_io_start_js`, `wasm_httpfs_manifest_js` |
//! | [`userdata`] | `wasm_userdata_put_js`, `wasm_userdata_delete_js`, `wasm_userdata_page_js`, `wasm_has_page_js` |
//! | [`input`] | `wasm_input_publish_js` |
//! | [`module_int`] | `wasm_module_int_js` |
//! | [`gpu`] | `wgpu_start_worker` |
//! | [`emscripten`] | `wasm_print_line_js`, `wasm_hang_line_js`, `emscripten_stack_*`, `emscripten_asm_const_int*`, `emscripten_get_heap_max`, `emscripten_resize_heap`, `_abort_js`, `_emscripten_throw_longjmp`, the thread/mailbox family, `_tzset_js`, `_mktime_js`, `_localtime_js`, `_gmtime_js`, `emscripten_num_logical_cores`, `_emscripten_system`, `invoke_*` |
//! | [`syscall`] | all 22 `__syscall_*` (file, directory, socket) |
//! | [`wasi`] | `proc_exit`, `fd_read`/`fd_write`/`fd_close`/`fd_seek`/`fd_pread`/`fd_pwrite`, `environ_*` |
//! | [`fd`] | the descriptor table and struct layouts shared by the two above |
//! | [`mem`] | linear-memory accessors |
//! | [`state`] | host installation, memory base, counters |
//!
//! `ABI-MAP.md` in the crate root lists all 86 imports with their index,
//! signature, implementation and real/no-op classification.
//!
//! ## Installing a host
//!
//! ```no_run
//! use std::sync::Arc;
//! use engine_host::{HeadlessConfig, HeadlessHost};
//!
//! let dir = std::env::temp_dir().join("engine-userdata");
//! let host = HeadlessHost::new(HeadlessConfig::new(dir)).expect("headless host");
//! let _guard = engine_abi::install(Arc::new(host));
//!
//! // The engine's memory reservation, once it exists.
//! let mut memory = vec![0u8; 1 << 20];
//! unsafe { engine_abi::set_memory(memory.as_mut_ptr(), memory.len()) };
//!
//! assert!(engine_abi::time::wasm_now_ms() >= 0.0);
//! ```
//!
//! The guard restores the previous host when dropped, so tests can nest.

// Every `unsafe extern "C"` shim documents its contract in prose: the pointer is a
// linear-memory offset, range-checked by `mem::*` before any access (see the
// aliasing model documented in `mem`). The identical `# Safety` block repeated 79
// times would add no information, so clippy's boilerplate-doc lint is off.
#![allow(clippy::missing_safety_doc)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![warn(missing_docs)]

pub mod audio;
pub mod emscripten;
pub mod fd;
pub mod gpu;
pub mod httpfs;
pub mod input;
pub mod mem;
pub mod module_int;
pub mod state;
pub mod syscall;
pub mod time;
pub mod userdata;
pub mod wasi;

pub use state::{bump, count, install, set_exit_hook, set_fs_root, set_memory, Counter, InstallGuard};

/// Every import name this crate provides, as `(index, name, module)`.
///
/// Index is the position in the module's import section (0 is the `memory`
/// entry, which is not a function). Kept in code so the integration test can
/// assert the list has not drifted from `ABI-MAP.md`.
pub const IMPORTS: [(u32, &str, &str); 86] = [
    (0, "memory", "env"),
    (1, "wasm_now_ms", "env"),
    (2, "wasm_hang_line_js", "env"),
    (3, "wasm_audio_publish_js", "env"),
    (4, "wasm_httpfs_io_start_js", "env"),
    (5, "wasm_httpfs_manifest_js", "env"),
    (6, "wasm_userdata_page_js", "env"),
    (7, "wasm_userdata_put_js", "env"),
    (8, "wasm_userdata_delete_js", "env"),
    (9, "wgpu_start_worker", "env"),
    (10, "wasm_has_page_js", "env"),
    (11, "wasm_input_publish_js", "env"),
    (12, "wasm_module_int_js", "env"),
    (13, "emscripten_get_now", "env"),
    (14, "wasm_print_line_js", "env"),
    (15, "emscripten_stack_snapshot", "env"),
    (16, "emscripten_stack_unwind_buffer", "env"),
    (17, "emscripten_asm_const_int", "env"),
    (18, "emscripten_get_heap_max", "env"),
    (19, "exit", "env"),
    (20, "invoke_vjj", "env"),
    (21, "invoke_ij", "env"),
    (22, "invoke_vj", "env"),
    (23, "invoke_vjij", "env"),
    (24, "invoke_jjij", "env"),
    (25, "invoke_vjii", "env"),
    (26, "invoke_iji", "env"),
    (27, "invoke_vjjii", "env"),
    (28, "invoke_jjiiijij", "env"),
    (29, "invoke_ijji", "env"),
    (30, "invoke_ijjiii", "env"),
    (31, "emscripten_asm_const_int_sync_on_main_thread", "env"),
    (32, "proc_exit", "wasi_snapshot_preview1"),
    (33, "_tzset_js", "env"),
    (34, "_abort_js", "env"),
    (35, "clock_time_get", "wasi_snapshot_preview1"),
    (36, "fd_close", "wasi_snapshot_preview1"),
    (37, "emscripten_date_now", "env"),
    (38, "_emscripten_thread_set_strongref", "env"),
    (39, "emscripten_exit_with_live_runtime", "env"),
    (40, "__syscall_fcntl64", "env"),
    (41, "__syscall_openat", "env"),
    (42, "__syscall_ioctl", "env"),
    (43, "fd_write", "wasi_snapshot_preview1"),
    (44, "fd_read", "wasi_snapshot_preview1"),
    (45, "__syscall_fstat64", "env"),
    (46, "__syscall_stat64", "env"),
    (47, "__syscall_newfstatat", "env"),
    (48, "__syscall_lstat64", "env"),
    (49, "__syscall_ftruncate64", "env"),
    (50, "__syscall_getcwd", "env"),
    (51, "environ_sizes_get", "wasi_snapshot_preview1"),
    (52, "environ_get", "wasi_snapshot_preview1"),
    (53, "_emscripten_init_main_thread_js", "env"),
    (54, "_emscripten_thread_mailbox_await", "env"),
    (55, "fd_seek", "wasi_snapshot_preview1"),
    (56, "__syscall_mkdirat", "env"),
    (57, "_mktime_js", "env"),
    (58, "_localtime_js", "env"),
    (59, "_gmtime_js", "env"),
    (60, "fd_pread", "wasi_snapshot_preview1"),
    (61, "_emscripten_receive_on_main_thread_js", "env"),
    (62, "emscripten_check_blocking_allowed", "env"),
    (63, "__pthread_create_js", "env"),
    (64, "_emscripten_thread_cleanup", "env"),
    (65, "fd_pwrite", "wasi_snapshot_preview1"),
    (66, "__syscall_getdents64", "env"),
    (67, "__syscall_unlinkat", "env"),
    (68, "__syscall_rmdir", "env"),
    (69, "__syscall_renameat", "env"),
    (70, "__syscall_poll", "env"),
    (71, "__syscall_statfs64", "env"),
    (72, "emscripten_num_logical_cores", "env"),
    (73, "_emscripten_system", "env"),
    (74, "_emscripten_notify_mailbox_postmessage", "env"),
    (75, "emscripten_resize_heap", "env"),
    (76, "_emscripten_throw_longjmp", "env"),
    (77, "__syscall_accept4", "env"),
    (78, "__syscall_connect", "env"),
    (79, "__syscall_getsockname", "env"),
    (80, "__syscall_getsockopt", "env"),
    (81, "__syscall_recvfrom", "env"),
    (82, "__syscall_sendto", "env"),
    (83, "__syscall_setsockopt", "env"),
    (84, "__syscall_shutdown", "env"),
    (85, "__syscall_socket", "env"),
];

/// Count of imports (functions plus the memory entry).
pub const IMPORT_COUNT: usize = IMPORTS.len();

/// Number of function imports (everything except the `memory` entry).
pub const FUNCTION_IMPORT_COUNT: usize = IMPORT_COUNT - 1;