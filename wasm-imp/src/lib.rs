//! Declarations for the 85 `env.*` imports of game.wasm.
//!
//! These are provided by the JS host at runtime; the translated Rust only
//! needs them to type-check and to keep the call graph visible.
#![allow(improper_ctypes, dead_code, non_snake_case, clippy::all)]

#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "wasm_now_ms"]
    pub fn imp_0_wasm_now_ms() -> f64;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "wasm_hang_line_js"]
    pub fn imp_1_wasm_hang_line_js(a0: i64);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "wasm_audio_publish_js"]
    pub fn imp_2_wasm_audio_publish_js(a0: i64, a1: i32);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "wasm_httpfs_io_start_js"]
    pub fn imp_3_wasm_httpfs_io_start_js(a0: i64);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "wasm_httpfs_manifest_js"]
    pub fn imp_4_wasm_httpfs_manifest_js(a0: i64, a1: i32) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "wasm_userdata_page_js"]
    pub fn imp_5_wasm_userdata_page_js() -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "wasm_userdata_put_js"]
    pub fn imp_6_wasm_userdata_put_js(a0: i64, a1: i64, a2: i64, a3: f64);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "wasm_userdata_delete_js"]
    pub fn imp_7_wasm_userdata_delete_js(a0: i64);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "wgpu_start_worker"]
    pub fn imp_8_wgpu_start_worker(a0: i64, a1: i64, a2: i64);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "wasm_has_page_js"]
    pub fn imp_9_wasm_has_page_js() -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "wasm_input_publish_js"]
    pub fn imp_10_wasm_input_publish_js(a0: i64);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "wasm_module_int_js"]
    pub fn imp_11_wasm_module_int_js(a0: i64, a1: i32) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "emscripten_get_now"]
    pub fn imp_12_emscripten_get_now() -> f64;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "wasm_print_line_js"]
    pub fn imp_13_wasm_print_line_js(a0: i64);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "emscripten_stack_snapshot"]
    pub fn imp_14_emscripten_stack_snapshot() -> i64;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "emscripten_stack_unwind_buffer"]
    pub fn imp_15_emscripten_stack_unwind_buffer(a0: i64, a1: i64, a2: i32) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "emscripten_asm_const_int"]
    pub fn imp_16_emscripten_asm_const_int(a0: i64, a1: i64, a2: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "emscripten_get_heap_max"]
    pub fn imp_17_emscripten_get_heap_max() -> i64;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "exit"]
    pub fn imp_18_exit(a0: i32);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "invoke_vjj"]
    pub fn imp_19_invoke_vjj(a0: i64, a1: i64, a2: i64);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "invoke_ij"]
    pub fn imp_20_invoke_ij(a0: i64, a1: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "invoke_vj"]
    pub fn imp_21_invoke_vj(a0: i64, a1: i64);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "invoke_vjij"]
    pub fn imp_22_invoke_vjij(a0: i64, a1: i64, a2: i32, a3: i64);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "invoke_jjij"]
    pub fn imp_23_invoke_jjij(a0: i64, a1: i64, a2: i32, a3: i64) -> i64;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "invoke_vjii"]
    pub fn imp_24_invoke_vjii(a0: i64, a1: i64, a2: i32, a3: i32);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "invoke_iji"]
    pub fn imp_25_invoke_iji(a0: i64, a1: i64, a2: i32) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "invoke_vjjii"]
    pub fn imp_26_invoke_vjjii(a0: i64, a1: i64, a2: i64, a3: i32, a4: i32);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "invoke_jjiiijij"]
    pub fn imp_27_invoke_jjiiijij(a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32, a7: i64) -> i64;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "invoke_ijji"]
    pub fn imp_28_invoke_ijji(a0: i64, a1: i64, a2: i64, a3: i32) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "invoke_ijjiii"]
    pub fn imp_29_invoke_ijjiii(a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "emscripten_asm_const_int_sync_on_main_thread"]
    pub fn imp_30_emscripten_asm_const_int_sync_on_main_thread(a0: i64, a1: i64, a2: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "wasi_snapshot_preview1"))]
extern "C" {
    #[link_name = "proc_exit"]
    pub fn imp_31_wasi_proc_exit(a0: i32);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "_tzset_js"]
    pub fn imp_32_tzset_js(a0: i64, a1: i64, a2: i64, a3: i64);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "_abort_js"]
    pub fn imp_33_abort_js();
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "wasi_snapshot_preview1"))]
extern "C" {
    #[link_name = "clock_time_get"]
    pub fn imp_34_wasi_clock_time_get(a0: i32, a1: i64, a2: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "wasi_snapshot_preview1"))]
extern "C" {
    #[link_name = "fd_close"]
    pub fn imp_35_wasi_fd_close(a0: i32) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "emscripten_date_now"]
    pub fn imp_36_emscripten_date_now() -> f64;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "_emscripten_thread_set_strongref"]
    pub fn imp_37_emscripten_thread_set_strongref(a0: i64);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "emscripten_exit_with_live_runtime"]
    pub fn imp_38_emscripten_exit_with_live_runtime();
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_fcntl64"]
    pub fn imp_39_syscall_fcntl64(a0: i32, a1: i32, a2: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_openat"]
    pub fn imp_40_syscall_openat(a0: i32, a1: i64, a2: i32, a3: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_ioctl"]
    pub fn imp_41_syscall_ioctl(a0: i32, a1: i32, a2: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "wasi_snapshot_preview1"))]
extern "C" {
    #[link_name = "fd_write"]
    pub fn imp_42_wasi_fd_write(a0: i32, a1: i64, a2: i64, a3: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "wasi_snapshot_preview1"))]
extern "C" {
    #[link_name = "fd_read"]
    pub fn imp_43_wasi_fd_read(a0: i32, a1: i64, a2: i64, a3: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_fstat64"]
    pub fn imp_44_syscall_fstat64(a0: i32, a1: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_stat64"]
    pub fn imp_45_syscall_stat64(a0: i64, a1: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_newfstatat"]
    pub fn imp_46_syscall_newfstatat(a0: i32, a1: i64, a2: i64, a3: i32) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_lstat64"]
    pub fn imp_47_syscall_lstat64(a0: i64, a1: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_ftruncate64"]
    pub fn imp_48_syscall_ftruncate64(a0: i32, a1: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_getcwd"]
    pub fn imp_49_syscall_getcwd(a0: i64, a1: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "wasi_snapshot_preview1"))]
extern "C" {
    #[link_name = "environ_sizes_get"]
    pub fn imp_50_wasi_environ_sizes_get(a0: i64, a1: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "wasi_snapshot_preview1"))]
extern "C" {
    #[link_name = "environ_get"]
    pub fn imp_51_wasi_environ_get(a0: i64, a1: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "_emscripten_init_main_thread_js"]
    pub fn imp_52_emscripten_init_main_thread_js(a0: i64);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "_emscripten_thread_mailbox_await"]
    pub fn imp_53_emscripten_thread_mailbox_await(a0: i64);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "wasi_snapshot_preview1"))]
extern "C" {
    #[link_name = "fd_seek"]
    pub fn imp_54_wasi_fd_seek(a0: i32, a1: i64, a2: i32, a3: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_mkdirat"]
    pub fn imp_55_syscall_mkdirat(a0: i32, a1: i64, a2: i32) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "_mktime_js"]
    pub fn imp_56_mktime_js(a0: i64) -> i64;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "_localtime_js"]
    pub fn imp_57_localtime_js(a0: i64, a1: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "_gmtime_js"]
    pub fn imp_58_gmtime_js(a0: i64, a1: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "wasi_snapshot_preview1"))]
extern "C" {
    #[link_name = "fd_pread"]
    pub fn imp_59_wasi_fd_pread(a0: i32, a1: i64, a2: i64, a3: i64, a4: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "_emscripten_receive_on_main_thread_js"]
    pub fn imp_60_emscripten_receive_on_main_thread_js(a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64) -> f64;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "emscripten_check_blocking_allowed"]
    pub fn imp_61_emscripten_check_blocking_allowed();
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__pthread_create_js"]
    pub fn imp_62_pthread_create_js(a0: i64, a1: i64, a2: i64, a3: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "_emscripten_thread_cleanup"]
    pub fn imp_63_emscripten_thread_cleanup(a0: i64);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "wasi_snapshot_preview1"))]
extern "C" {
    #[link_name = "fd_pwrite"]
    pub fn imp_64_wasi_fd_pwrite(a0: i32, a1: i64, a2: i64, a3: i64, a4: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_getdents64"]
    pub fn imp_65_syscall_getdents64(a0: i32, a1: i64, a2: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_unlinkat"]
    pub fn imp_66_syscall_unlinkat(a0: i32, a1: i64, a2: i32) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_rmdir"]
    pub fn imp_67_syscall_rmdir(a0: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_renameat"]
    pub fn imp_68_syscall_renameat(a0: i32, a1: i64, a2: i32, a3: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_poll"]
    pub fn imp_69_syscall_poll(a0: i64, a1: i32, a2: i32) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_statfs64"]
    pub fn imp_70_syscall_statfs64(a0: i64, a1: i64, a2: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "emscripten_num_logical_cores"]
    pub fn imp_71_emscripten_num_logical_cores() -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "_emscripten_system"]
    pub fn imp_72_emscripten_system(a0: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "_emscripten_notify_mailbox_postmessage"]
    pub fn imp_73_emscripten_notify_mailbox_postmessage(a0: i64, a1: i64);
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "emscripten_resize_heap"]
    pub fn imp_74_emscripten_resize_heap(a0: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "_emscripten_throw_longjmp"]
    pub fn imp_75_emscripten_throw_longjmp();
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_accept4"]
    pub fn imp_76_syscall_accept4(a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_connect"]
    pub fn imp_77_syscall_connect(a0: i32, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_getsockname"]
    pub fn imp_78_syscall_getsockname(a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_getsockopt"]
    pub fn imp_79_syscall_getsockopt(a0: i32, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_recvfrom"]
    pub fn imp_80_syscall_recvfrom(a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_sendto"]
    pub fn imp_81_syscall_sendto(a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_setsockopt"]
    pub fn imp_82_syscall_setsockopt(a0: i32, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_shutdown"]
    pub fn imp_83_syscall_shutdown(a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32;
}
#[cfg_attr(target_family = "wasm", link(wasm_import_module = "env"))]
extern "C" {
    #[link_name = "__syscall_socket"]
    pub fn imp_84_syscall_socket(a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32;
}
