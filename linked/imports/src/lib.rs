#![allow(unused_variables)]
pub use host_imports::*;
use wasm_rt::Phantom;
extern "C" { fn f_091110_emscripten_thread_init(ph: *mut Phantom, a: i64, b: i32, c: i32, d: i32, e: i32, f: i32); fn f_090693_emscripten_tls_init(ph: *mut Phantom) -> i64; }
pub unsafe extern "C" fn table_import_0(ph: *mut Phantom) -> f64 {
    host_imports::imp_0_wasm_now_ms()
}
pub unsafe extern "C" fn table_import_1(ph: *mut Phantom, a0: i64) {
    host_imports::imp_1_wasm_hang_line_js(a0)
}
pub unsafe extern "C" fn table_import_2(ph: *mut Phantom, a0: i64, a1: i32) {
    host_imports::imp_2_wasm_audio_publish_js(a0, a1)
}
pub unsafe extern "C" fn table_import_3(ph: *mut Phantom, a0: i64) {
    host_imports::imp_3_wasm_httpfs_io_start_js(a0)
}
pub unsafe extern "C" fn table_import_4(ph: *mut Phantom, a0: i64, a1: i32) -> i32 {
    host_imports::imp_4_wasm_httpfs_manifest_js(a0, a1)
}
pub unsafe extern "C" fn table_import_5(ph: *mut Phantom) -> i32 {
    host_imports::imp_5_wasm_userdata_page_js()
}
pub unsafe extern "C" fn table_import_6(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f64) {
    host_imports::imp_6_wasm_userdata_put_js(a0, a1, a2, a3)
}
pub unsafe extern "C" fn table_import_7(ph: *mut Phantom, a0: i64) {
    host_imports::imp_7_wasm_userdata_delete_js(a0)
}
pub unsafe extern "C" fn table_import_8(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) {
    host_imports::imp_8_wgpu_start_worker(a0, a1, a2)
}
pub unsafe extern "C" fn table_import_9(ph: *mut Phantom) -> i32 {
    host_imports::imp_9_wasm_has_page_js()
}
pub unsafe extern "C" fn table_import_10(ph: *mut Phantom, a0: i64) {
    host_imports::imp_10_wasm_input_publish_js(a0)
}
pub unsafe extern "C" fn table_import_11(ph: *mut Phantom, a0: i64, a1: i32) -> i32 {
    host_imports::imp_11_wasm_module_int_js(a0, a1)
}
pub unsafe extern "C" fn table_import_12(ph: *mut Phantom) -> f64 {
    host_imports::imp_12_emscripten_get_now()
}
pub unsafe extern "C" fn table_import_13(ph: *mut Phantom, a0: i64) {
    host_imports::imp_13_wasm_print_line_js(a0)
}
pub unsafe extern "C" fn table_import_14(ph: *mut Phantom) -> i64 {
    host_imports::imp_14_emscripten_stack_snapshot()
}
pub unsafe extern "C" fn table_import_15(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32 {
    host_imports::imp_15_emscripten_stack_unwind_buffer(a0, a1, a2)
}
pub unsafe extern "C" fn table_import_16(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32 {
    host_imports::imp_16_emscripten_asm_const_int(a0, a1, a2)
}
pub unsafe extern "C" fn table_import_17(ph: *mut Phantom) -> i64 {
    host_imports::imp_17_emscripten_get_heap_max()
}
pub unsafe extern "C" fn table_import_18(ph: *mut Phantom, a0: i32) {
    host_imports::imp_18_exit(a0)
}
pub unsafe fn imp_19_invoke_vjj(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) {
    let address = (*ph).table_entry_typed(a0, 3);
    #[cfg(target_family = "wasm")]
    { host_imports::imp_19_invoke_vjj(a0, a1, a2) }
    #[cfg(not(target_family = "wasm"))]
    {
    let function: unsafe extern "C" fn(*mut Phantom, i64, i64) = core::mem::transmute(address);
    function(ph as *mut Phantom, a1, a2)
}
}
pub unsafe extern "C" fn table_import_19(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) {
    imp_19_invoke_vjj(ph, a0, a1, a2)
}
pub unsafe fn imp_20_invoke_ij(ph: *mut Phantom, a0: i64, a1: i64) -> i32 {
    let address = (*ph).table_entry_typed(a0, 4);
    #[cfg(target_family = "wasm")]
    { host_imports::imp_20_invoke_ij(a0, a1) }
    #[cfg(not(target_family = "wasm"))]
    {
    let function: unsafe extern "C" fn(*mut Phantom, i64) -> i32 = core::mem::transmute(address);
    function(ph as *mut Phantom, a1)
}
}
pub unsafe extern "C" fn table_import_20(ph: *mut Phantom, a0: i64, a1: i64) -> i32 {
    imp_20_invoke_ij(ph, a0, a1)
}
pub unsafe fn imp_21_invoke_vj(ph: *mut Phantom, a0: i64, a1: i64) {
    let address = (*ph).table_entry_typed(a0, 1);
    #[cfg(target_family = "wasm")]
    { host_imports::imp_21_invoke_vj(a0, a1) }
    #[cfg(not(target_family = "wasm"))]
    {
    let function: unsafe extern "C" fn(*mut Phantom, i64) = core::mem::transmute(address);
    function(ph as *mut Phantom, a1)
}
}
pub unsafe extern "C" fn table_import_21(ph: *mut Phantom, a0: i64, a1: i64) {
    imp_21_invoke_vj(ph, a0, a1)
}
pub unsafe fn imp_22_invoke_vjij(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) {
    let address = (*ph).table_entry_typed(a0, 20);
    #[cfg(target_family = "wasm")]
    { host_imports::imp_22_invoke_vjij(a0, a1, a2, a3) }
    #[cfg(not(target_family = "wasm"))]
    {
    let function: unsafe extern "C" fn(*mut Phantom, i64, i32, i64) = core::mem::transmute(address);
    function(ph as *mut Phantom, a1, a2, a3)
}
}
pub unsafe extern "C" fn table_import_22(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) {
    imp_22_invoke_vjij(ph, a0, a1, a2, a3)
}
pub unsafe fn imp_23_invoke_jjij(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64 {
    let address = (*ph).table_entry_typed(a0, 69);
    #[cfg(target_family = "wasm")]
    { host_imports::imp_23_invoke_jjij(a0, a1, a2, a3) }
    #[cfg(not(target_family = "wasm"))]
    {
    let function: unsafe extern "C" fn(*mut Phantom, i64, i32, i64) -> i64 = core::mem::transmute(address);
    function(ph as *mut Phantom, a1, a2, a3)
}
}
pub unsafe extern "C" fn table_import_23(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64 {
    imp_23_invoke_jjij(ph, a0, a1, a2, a3)
}
pub unsafe fn imp_24_invoke_vjii(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) {
    let address = (*ph).table_entry_typed(a0, 19);
    #[cfg(target_family = "wasm")]
    { host_imports::imp_24_invoke_vjii(a0, a1, a2, a3) }
    #[cfg(not(target_family = "wasm"))]
    {
    let function: unsafe extern "C" fn(*mut Phantom, i64, i32, i32) = core::mem::transmute(address);
    function(ph as *mut Phantom, a1, a2, a3)
}
}
pub unsafe extern "C" fn table_import_24(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) {
    imp_24_invoke_vjii(ph, a0, a1, a2, a3)
}
pub unsafe fn imp_25_invoke_iji(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32 {
    let address = (*ph).table_entry_typed(a0, 10);
    #[cfg(target_family = "wasm")]
    { host_imports::imp_25_invoke_iji(a0, a1, a2) }
    #[cfg(not(target_family = "wasm"))]
    {
    let function: unsafe extern "C" fn(*mut Phantom, i64, i32) -> i32 = core::mem::transmute(address);
    function(ph as *mut Phantom, a1, a2)
}
}
pub unsafe extern "C" fn table_import_25(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32 {
    imp_25_invoke_iji(ph, a0, a1, a2)
}
pub unsafe fn imp_26_invoke_vjjii(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) {
    let address = (*ph).table_entry_typed(a0, 33);
    #[cfg(target_family = "wasm")]
    { host_imports::imp_26_invoke_vjjii(a0, a1, a2, a3, a4) }
    #[cfg(not(target_family = "wasm"))]
    {
    let function: unsafe extern "C" fn(*mut Phantom, i64, i64, i32, i32) = core::mem::transmute(address);
    function(ph as *mut Phantom, a1, a2, a3, a4)
}
}
pub unsafe extern "C" fn table_import_26(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) {
    imp_26_invoke_vjjii(ph, a0, a1, a2, a3, a4)
}
pub unsafe fn imp_27_invoke_jjiiijij(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32, a7: i64) -> i64 {
    let address = (*ph).table_entry_typed(a0, 135);
    #[cfg(target_family = "wasm")]
    { host_imports::imp_27_invoke_jjiiijij(a0, a1, a2, a3, a4, a5, a6, a7) }
    #[cfg(not(target_family = "wasm"))]
    {
    let function: unsafe extern "C" fn(*mut Phantom, i64, i32, i32, i32, i64, i32, i64) -> i64 = core::mem::transmute(address);
    function(ph as *mut Phantom, a1, a2, a3, a4, a5, a6, a7)
}
}
pub unsafe extern "C" fn table_import_27(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32, a7: i64) -> i64 {
    imp_27_invoke_jjiiijij(ph, a0, a1, a2, a3, a4, a5, a6, a7)
}
pub unsafe fn imp_28_invoke_ijji(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32 {
    let address = (*ph).table_entry_typed(a0, 27);
    #[cfg(target_family = "wasm")]
    { host_imports::imp_28_invoke_ijji(a0, a1, a2, a3) }
    #[cfg(not(target_family = "wasm"))]
    {
    let function: unsafe extern "C" fn(*mut Phantom, i64, i64, i32) -> i32 = core::mem::transmute(address);
    function(ph as *mut Phantom, a1, a2, a3)
}
}
pub unsafe extern "C" fn table_import_28(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32 {
    imp_28_invoke_ijji(ph, a0, a1, a2, a3)
}
pub unsafe fn imp_29_invoke_ijjiii(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32 {
    let address = (*ph).table_entry_typed(a0, 115);
    #[cfg(target_family = "wasm")]
    { host_imports::imp_29_invoke_ijjiii(a0, a1, a2, a3, a4, a5) }
    #[cfg(not(target_family = "wasm"))]
    {
    let function: unsafe extern "C" fn(*mut Phantom, i64, i64, i32, i32, i32) -> i32 = core::mem::transmute(address);
    function(ph as *mut Phantom, a1, a2, a3, a4, a5)
}
}
pub unsafe extern "C" fn table_import_29(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32 {
    imp_29_invoke_ijjiii(ph, a0, a1, a2, a3, a4, a5)
}
pub unsafe extern "C" fn table_import_30(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32 {
    host_imports::imp_30_emscripten_asm_const_int_sync_on_main_thread(a0, a1, a2)
}
pub unsafe extern "C" fn table_import_31(ph: *mut Phantom, a0: i32) {
    host_imports::imp_31_wasi_proc_exit(a0)
}
pub unsafe extern "C" fn table_import_32(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) {
    host_imports::imp_32_tzset_js(a0, a1, a2, a3)
}
pub unsafe extern "C" fn table_import_33(ph: *mut Phantom) {
    host_imports::imp_33_abort_js()
}
pub unsafe extern "C" fn table_import_34(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32 {
    host_imports::imp_34_wasi_clock_time_get(a0, a1, a2)
}
pub unsafe extern "C" fn table_import_35(ph: *mut Phantom, a0: i32) -> i32 {
    host_imports::imp_35_wasi_fd_close(a0)
}
pub unsafe extern "C" fn table_import_36(ph: *mut Phantom) -> f64 {
    host_imports::imp_36_emscripten_date_now()
}
pub unsafe extern "C" fn table_import_37(ph: *mut Phantom, a0: i64) {
    host_imports::imp_37_emscripten_thread_set_strongref(a0)
}
pub unsafe extern "C" fn table_import_38(ph: *mut Phantom) {
    host_imports::imp_38_emscripten_exit_with_live_runtime()
}
pub unsafe extern "C" fn table_import_39(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32 {
    host_imports::imp_39_syscall_fcntl64(a0, a1, a2)
}
pub unsafe extern "C" fn table_import_40(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64) -> i32 {
    host_imports::imp_40_syscall_openat(a0, a1, a2, a3)
}
pub unsafe extern "C" fn table_import_41(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32 {
    host_imports::imp_41_syscall_ioctl(a0, a1, a2)
}
pub unsafe extern "C" fn table_import_42(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64) -> i32 {
    host_imports::imp_42_wasi_fd_write(a0, a1, a2, a3)
}
pub unsafe extern "C" fn table_import_43(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64) -> i32 {
    host_imports::imp_43_wasi_fd_read(a0, a1, a2, a3)
}
pub unsafe extern "C" fn table_import_44(ph: *mut Phantom, a0: i32, a1: i64) -> i32 {
    host_imports::imp_44_syscall_fstat64(a0, a1)
}
pub unsafe extern "C" fn table_import_45(ph: *mut Phantom, a0: i64, a1: i64) -> i32 {
    host_imports::imp_45_syscall_stat64(a0, a1)
}
pub unsafe extern "C" fn table_import_46(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32) -> i32 {
    host_imports::imp_46_syscall_newfstatat(a0, a1, a2, a3)
}
pub unsafe extern "C" fn table_import_47(ph: *mut Phantom, a0: i64, a1: i64) -> i32 {
    host_imports::imp_47_syscall_lstat64(a0, a1)
}
pub unsafe extern "C" fn table_import_48(ph: *mut Phantom, a0: i32, a1: i64) -> i32 {
    host_imports::imp_48_syscall_ftruncate64(a0, a1)
}
pub unsafe extern "C" fn table_import_49(ph: *mut Phantom, a0: i64, a1: i64) -> i32 {
    host_imports::imp_49_syscall_getcwd(a0, a1)
}
pub unsafe extern "C" fn table_import_50(ph: *mut Phantom, a0: i64, a1: i64) -> i32 {
    host_imports::imp_50_wasi_environ_sizes_get(a0, a1)
}
pub unsafe extern "C" fn table_import_51(ph: *mut Phantom, a0: i64, a1: i64) -> i32 {
    host_imports::imp_51_wasi_environ_get(a0, a1)
}
pub unsafe fn imp_52_emscripten_init_main_thread_js(ph: *mut Phantom, a0: i64) {
    #[cfg(target_family = "wasm")]
    { host_imports::imp_52_emscripten_init_main_thread_js(a0); }
    #[cfg(not(target_family = "wasm"))]
    { f_091110_emscripten_thread_init(ph, a0, 1, 1, 1, 1048576, 0);
      f_090693_emscripten_tls_init(ph); }
}
pub unsafe extern "C" fn table_import_52(ph: *mut Phantom, a0: i64) {
    imp_52_emscripten_init_main_thread_js(ph, a0)
}
pub unsafe extern "C" fn table_import_53(ph: *mut Phantom, a0: i64) {
    host_imports::imp_53_emscripten_thread_mailbox_await(a0)
}
pub unsafe extern "C" fn table_import_54(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64) -> i32 {
    host_imports::imp_54_wasi_fd_seek(a0, a1, a2, a3)
}
pub unsafe extern "C" fn table_import_55(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32 {
    host_imports::imp_55_syscall_mkdirat(a0, a1, a2)
}
pub unsafe extern "C" fn table_import_56(ph: *mut Phantom, a0: i64) -> i64 {
    host_imports::imp_56_mktime_js(a0)
}
pub unsafe extern "C" fn table_import_57(ph: *mut Phantom, a0: i64, a1: i64) -> i32 {
    host_imports::imp_57_localtime_js(a0, a1)
}
pub unsafe extern "C" fn table_import_58(ph: *mut Phantom, a0: i64, a1: i64) -> i32 {
    host_imports::imp_58_gmtime_js(a0, a1)
}
pub unsafe extern "C" fn table_import_59(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64) -> i32 {
    host_imports::imp_59_wasi_fd_pread(a0, a1, a2, a3, a4)
}
pub unsafe extern "C" fn table_import_60(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64) -> f64 {
    host_imports::imp_60_emscripten_receive_on_main_thread_js(a0, a1, a2, a3, a4, a5, a6)
}
pub unsafe extern "C" fn table_import_61(ph: *mut Phantom) {
    host_imports::imp_61_emscripten_check_blocking_allowed()
}
pub unsafe extern "C" fn table_import_62(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32 {
    host_imports::imp_62_pthread_create_js(a0, a1, a2, a3)
}
pub unsafe extern "C" fn table_import_63(ph: *mut Phantom, a0: i64) {
    host_imports::imp_63_emscripten_thread_cleanup(a0)
}
pub unsafe extern "C" fn table_import_64(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64) -> i32 {
    host_imports::imp_64_wasi_fd_pwrite(a0, a1, a2, a3, a4)
}
pub unsafe extern "C" fn table_import_65(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32 {
    host_imports::imp_65_syscall_getdents64(a0, a1, a2)
}
pub unsafe extern "C" fn table_import_66(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32 {
    host_imports::imp_66_syscall_unlinkat(a0, a1, a2)
}
pub unsafe extern "C" fn table_import_67(ph: *mut Phantom, a0: i64) -> i32 {
    host_imports::imp_67_syscall_rmdir(a0)
}
pub unsafe extern "C" fn table_import_68(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64) -> i32 {
    host_imports::imp_68_syscall_renameat(a0, a1, a2, a3)
}
pub unsafe extern "C" fn table_import_69(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32 {
    host_imports::imp_69_syscall_poll(a0, a1, a2)
}
pub unsafe extern "C" fn table_import_70(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32 {
    host_imports::imp_70_syscall_statfs64(a0, a1, a2)
}
pub unsafe extern "C" fn table_import_71(ph: *mut Phantom) -> i32 {
    host_imports::imp_71_emscripten_num_logical_cores()
}
pub unsafe extern "C" fn table_import_72(ph: *mut Phantom, a0: i64) -> i32 {
    host_imports::imp_72_emscripten_system(a0)
}
pub unsafe extern "C" fn table_import_73(ph: *mut Phantom, a0: i64, a1: i64) {
    host_imports::imp_73_emscripten_notify_mailbox_postmessage(a0, a1)
}
pub unsafe extern "C" fn table_import_74(ph: *mut Phantom, a0: i64) -> i32 {
    host_imports::imp_74_emscripten_resize_heap(a0)
}
pub unsafe extern "C" fn table_import_75(ph: *mut Phantom) {
    host_imports::imp_75_emscripten_throw_longjmp()
}
pub unsafe extern "C" fn table_import_76(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32 {
    host_imports::imp_76_syscall_accept4(a0, a1, a2, a3, a4, a5)
}
pub unsafe extern "C" fn table_import_77(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i32 {
    host_imports::imp_77_syscall_connect(a0, a1, a2, a3, a4, a5)
}
pub unsafe extern "C" fn table_import_78(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32 {
    host_imports::imp_78_syscall_getsockname(a0, a1, a2, a3, a4, a5)
}
pub unsafe extern "C" fn table_import_79(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32) -> i32 {
    host_imports::imp_79_syscall_getsockopt(a0, a1, a2, a3, a4, a5)
}
pub unsafe extern "C" fn table_import_80(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32 {
    host_imports::imp_80_syscall_recvfrom(a0, a1, a2, a3, a4, a5)
}
pub unsafe extern "C" fn table_import_81(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32) -> i32 {
    host_imports::imp_81_syscall_sendto(a0, a1, a2, a3, a4, a5)
}
pub unsafe extern "C" fn table_import_82(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32) -> i32 {
    host_imports::imp_82_syscall_setsockopt(a0, a1, a2, a3, a4, a5)
}
pub unsafe extern "C" fn table_import_83(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32 {
    host_imports::imp_83_syscall_shutdown(a0, a1, a2, a3, a4, a5)
}
pub unsafe extern "C" fn table_import_84(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32 {
    host_imports::imp_84_syscall_socket(a0, a1, a2, a3, a4, a5)
}
