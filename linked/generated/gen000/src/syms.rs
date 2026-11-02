//! Callee declarations for crate gen000.
//!
//! Every `call $name` in the wasm module becomes a call through this
//! module: an `extern "C"` declaration of the target's generated
//! identifier.  That keeps a function's callers independent of which
//! generated crate the callee ended up in (they are all linked into the
//! same binary and the identifiers are unique).
#![allow(improper_ctypes, dead_code, unused_imports)]
use wasm_rt::Phantom;

extern "C" { pub fn f_000090_initgameheap_t_initgameheap_t(ph: *mut Phantom); }
extern "C" { pub fn f_000094_cxx_global_array_dtor_28(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000100_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000101_op_new_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000102_op_delete_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000104_main(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000107_wasm_streaming_report(ph: *mut Phantom); }
extern "C" { pub fn f_000109_rage_syscriticalsectiontoken_syscriticalsectiontoken_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000110_rage_syscriticalsectiontoken_dtorsyscriticalsectiontoken(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000111_rage_syscriticalsectiontoken_lock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000112_rage_syscriticalsectiontoken_trylock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000113_rage_syscriticalsectiontoken_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000114_rage_sysipccreatesema_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_000115_rage_sysipcpollsema_rage_sysipcsematag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000116_rage_sysipcwaitsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000117_rage_sematakeuntil_rage_sysipcsematag_ptr_double(ph: *mut Phantom, a0: i64, a1: f64) -> i32; }
extern "C" { pub fn f_000118_rage_sysipcwaitsematimed_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_000119_rage_sysipcsignalsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000120_rage_sysipccreatemutex(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_000121_rage_sysipclockmutexcommand_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000122_rage_sysipcunlockmutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000123_rage_sysipcdeletemutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000124_rage_sysipccreateevent(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_000125_rage_sysipcwaitevent_rage_sysipceventtag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000126_rage_sysipcpollevent_rage_sysipceventtag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000127_rage_sysipcsetevent_rage_sysipceventtag_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000128_rage_sysipccreatethread_void_ptr_void_ptr_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_000131_rage_sysipcwaitthreadexit_rage_sysipcthreadid_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000134_rage_sysipcyield_int(ph: *mut Phantom); }
extern "C" { pub fn f_000135_rage_sysipcsleep_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000136_rage_sysipcestimatestackusage_unsigned_int_ref_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000137_wasm_wait_begin(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000138_wasm_wait_end(ph: *mut Phantom); }
extern "C" { pub fn f_000139_wasm_note_output(ph: *mut Phantom); }
extern "C" { pub fn f_000140_rage_append_char_ptr_int_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_000141_rage_appendtop_char_ptr_int_int_rage_samplehist_const_ref_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_000142_rage_appendkey_char_ptr_int_int_rage_samplekey_const_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_000164_rage_nextentry_rage_fiwasmfind_ptr_rage_fifinddata_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000173_rage_fidevice_convertfiletimetosystemtime_unsigned_long_rage_fid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000175_rage_wasm_announce_read_rage_fidevice_const_ptr_fihandle_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000191_wasm_httpfs_active(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000194_wasm_httpfs_lookup(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_000195_anonymous_namespace_key_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000196_std_2_hash_iterator_std_2_hash_node_std_2_hash_value_type_std_2(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000197_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom); }
extern "C" { pub fn f_000198_wasm_httpfs_pread(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_000199_std_2_hash_table_std_2_hash_value_type_std_2_basic_string_char_s(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000200_anonymous_namespace_hex4_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000201_anonymous_namespace_dirof_std_2_basic_string_char_std_2_char_tra(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000202_std_2_pair_std_2_hash_iterator_std_2_hash_node_std_2_hash_value(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_000203_void_std_2_vector_anonymous_namespace_direntry_std_2_allocator_a(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000204_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom); }
extern "C" { pub fn f_000205_std_throw_bad_array_new_length_abi_nqn220108(ph: *mut Phantom); }
extern "C" { pub fn f_000206_std_2_vector_anonymous_namespace_fileentry_std_2_allocator_anony(ph: *mut Phantom); }
extern "C" { pub fn f_000207_std_2_throw_length_error_abi_nqn220108_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000208_void_std_2_hash_table_std_2_hash_value_type_std_2_basic_string_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000212_rage_fidevicetcpip_connect_char_const_ptr_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000219_std_2_tree_std_2_value_type_std_2_basic_string_char_std_2_char_t(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000220_anonymous_namespace_scan_std_2_basic_string_char_std_2_char_trai(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000221_void_std_2_tree_balance_after_insert_abi_nqn220108_std_2_tree_no(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000222_void_std_2_tree_remove_abi_nqn220108_std_2_tree_node_base_void_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000223_std_2_tree_std_2_value_type_std_2_basic_string_char_std_2_char_t(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000237_rage_grcdevice_computewindowdimensions_tagrect_ref_unsigned_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000251_wasm_null_d3d_getformatinfo_dxgi_format(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000258_wasm_null_d3d_nullresource_wasm_null_d3d_stub_id3d11texture2d_d3(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000261_wasm_null_d3d_nullresource_wasm_null_d3d_stub_id3d11texture3d_d3(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000263_wasm_null_d3d_nullresource_wasm_null_d3d_stub_id3d11texture3d_d3(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000266_wasm_null_d3d_gpuuploadbuffer_unsigned_int_unsigned_long_void_co(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_000267_wasm_null_d3d_gpuuploadtexture_unsigned_int_dxgi_format_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64, a8: i64, a9: i32, a10: i32); }
extern "C" { pub fn f_000268_wasm_null_d3d_nullresource_wasm_null_d3d_stub_id3d11texture2d_d3(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000311_wasm_null_d3d_hasgpuid_waitreadback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000430_void_std_2_vector_unsigned_int_std_2_allocator_unsigned_int_empl(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000473_wasm_null_d3d_nullcontext_setcbuffers_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_000475_wasm_null_d3d_nullcontext_setsrvs_unsigned_int_unsigned_int_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_000477_wasm_null_d3d_nullcontext_setshader_unsigned_int_iunknown_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_000479_wasm_null_d3d_nullcontext_setsamplerstates_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_000488_wasm_null_d3d_nullcontext_changedslots_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_000503_wasm_null_d3d_nullcontext_setuavs_unsigned_int_unsigned_int_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_000516_wasm_null_d3d_forgetuploaded_id3d11resource_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000517_wasm_null_d3d_gpureadback_id3d11resource_ptr_unsigned_int_id3d11(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_000523_wasm_null_d3d_nullcontext_clearuav_id3d11unorderedaccessview_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000548_wasm_gpu_enabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000549_wasm_gpu_init(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000550_wasm_gpu_emit_unsigned_int_unsigned_int_const_ptr_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_000551_wasm_gpu_reserve_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_000552_wasm_gpu_emitdata_unsigned_int_unsigned_int_const_ptr_unsigned_i(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_000553_wasm_gpu_fnv64_void_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000554_wasm_gpu_allocid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000555_wasm_gpu_createtexture_unsigned_int_unsigned_int_unsigned_int_un(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_000556_wasm_gpu_destroyobject_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000557_wasm_gpu_argvalue_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000558_wasm_gpu_fence(ph: *mut Phantom); }
extern "C" { pub fn f_000559_wasm_gpu_countunsupported_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000560_getclientrect_hwnd_ptr_tagrect_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000561_getwindowrect_hwnd_ptr_tagrect_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000562_setwindowpos_hwnd_ptr_hwnd_ptr_int_int_int_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_000566_wasm_start_mode_now(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000567_argstartmode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000569_startchoice(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000589_gwaitconditionimpl_wait_gmutex_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000609_rage_voicechat_nativepackchat_int_void_ptr_unsigned_int_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000611_fse_readncount_bmi2(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_000612_huf_readstats_wksp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_000613_fse_initdstate(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000614_bit_initdstream(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000615_zstd_xxh64(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000616_zstd_xxh64_update(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000617_zstd_xxh64_digest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000618_zstd_iserror(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000619_huf_readdtablex1_wksp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000620_huf_readdtablex2_wksp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_000621_huf_decompress1x1_usingdtable_internal(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_000622_huf_decompress4x2_usingdtable_internal(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_000623_huf_decompress4x1_usingdtable_internal(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_000624_huf_decompressfastargs_init(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_000627_zstd_copyddictparameters(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000628_zstd_freeddict(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000629_zstd_getdictid_fromddict(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000630_zstd_getframeheader_advanced(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000631_zstd_decodeframeheader(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000632_zstd_decompresscontinue(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_000633_zstd_decompressstream(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000634_zstd_getcblocksize(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000635_zstd_allocateliteralsbuffer(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32); }
extern "C" { pub fn f_000636_zstd_buildfsetable(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i64); }
extern "C" { pub fn f_000637_zstd_buildseqtable(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i32, a11: i32, a12: i32, a13: i64) -> i64; }
extern "C" { pub fn f_000638_zstd_decompressblock_internal(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_000639_zstd_execsequenceendsplitlitbuffer(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64) -> i64; }
extern "C" { pub fn f_000640_zstd_execsequenceend(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_000641_zstd_checkcontinuity(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000642_zstd_safecopy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_000644_rage_athashnextsize_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000645_rage_athash_const_char_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000649_rage_dattypestruct_addfield_unsigned_long_unsigned_long_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000650_rage_athash16u_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000653_rage_atstring_set_rage_atstring_const_ref_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_000655_rage_atstring_lowercase(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000657_rage_atstring_endswith_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000658_rage_atstring_startswith_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000659_rage_atstring_indexof_char_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_000662_rage_atstring_lastindexof_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000663_rage_atstring_replace_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000665_rage_atstring_op_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000667_rage_atstring_split_rage_atarray_rage_atstring_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_000668_rage_atarray_rage_atstring_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_000669_rage_atstring_trim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000670_rage_atstring_trimleft(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000671_rage_atstring_split_rage_atarray_rage_atstring_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000672_rage_atstring_op_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000674_rage_atwidestring_append_unsigned_short_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000675_rage_functorbase_functorbase_void_const_ptr_void_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_000678_rage_athashstringnamespacesupport_shutdownnamespaces(ph: *mut Phantom); }
extern "C" { pub fn f_000679_rage_athashstringnamespacesupport_shutdownnamespace_rage_athashs(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000680_rage_athashstringnamespacesupport_computehash_rage_athashstringn(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_000681_rage_athashstringnamespacesupport_addstring_rage_athashstringnam(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_000682_rage_atmapentry_unsigned_int_char_const_ptr_const_ref_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000683_rage_atmap_unsigned_int_char_const_ptr_rage_atmaphashfn_unsigned(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000684_rage_athashstringnamespacesupport_trygetstring_rage_athashstring(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_000686_rage_athashstringnamespacesupport_initnamespaces(ph: *mut Phantom); }
extern "C" { pub fn f_000690_rage_bkwidget_bkwidget_rage_datcallback_ref_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_000692_rage_datcallback_call_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000695_rage_bkremotepacket_receivepackets(ph: *mut Phantom); }
extern "C" { pub fn f_000696_rage_bkwidget_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000698_rage_bkwidget_dtorbkwidget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000703_rage_bkbank_unsetcurrentgroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000704_rage_bkbank_pushgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000707_rage_bkbank_popgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000709_rage_bkwidget_getnumwidgets_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_000711_rage_bkremotewidgetmap_findnextid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000712_rage_bkremotewidgetmap_addwidgetandid_rage_bkwidget_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000713_rage_bkremotepacket_sendpackets(ph: *mut Phantom); }
extern "C" { pub fn f_000714_rage_bkwidget_remotecreate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000716_rage_bkremotepacket_send(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000720_rage_bkwidget_addchild_rage_bkwidget_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000757_rage_bkremotepacket_addtype_int_void_ptr_rage_bkremotepacket_con(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_000758_rage_bkmanager_openfile_char_ptr_int_char_const_ptr_bool_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_000759_rage_bktext_bktext_rage_datcallback_ref_char_const_ptr_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i64) -> i64; }
extern "C" { pub fn f_000760_rage_inmap_base_unsigned_int_rage_bkwidget_refrage_bkwidget_m_re(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000761_rage_bksliderval_float_bksliderval_rage_datcallback_ref_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32, a7: f32, a8: i64, a9: i32, a10: i32) -> i64; }
extern "C" { pub fn f_000762_rage_bksliderval_unsigned_char_bksliderval_rage_datcallback_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i32, a10: i32) -> i64; }
extern "C" { pub fn f_000763_rage_bksliderval_signed_char_bksliderval_rage_datcallback_ref_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i32, a10: i32) -> i64; }
extern "C" { pub fn f_000764_rage_bksliderval_unsigned_short_bksliderval_rage_datcallback_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i32, a10: i32) -> i64; }
extern "C" { pub fn f_000765_rage_bksliderval_short_bksliderval_rage_datcallback_ref_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i32, a10: i32) -> i64; }
extern "C" { pub fn f_000766_rage_bksliderval_unsigned_int_bksliderval_rage_datcallback_ref_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i32, a10: i32) -> i64; }
extern "C" { pub fn f_000767_rage_bksliderval_int_bksliderval_rage_datcallback_ref_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i32, a10: i32) -> i64; }
extern "C" { pub fn f_000768_rage_bkcolor_remotereadcolor_rage_bkremotepacket_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000769_rage_bkvector_remotehandler_rage_bkremotepacket_const_ref_rage_b(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000770_rage_bkmatrix_remotehandler_rage_bkremotepacket_const_ref_rage_b(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000773_rage_bkmanager_createoutputwindow_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000774_rage_bkmanager_sendsubchannelinfo_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000777_rage_bkmanager_dtorbkmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000781_rage_bkwidget_drawf_int_int_char_const_ptr_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_000784_rage_bkmanager_findwidget_rage_bkwidget_ptr_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000789_rage_atmapentry_rage_conststring_rage_atfunctor3_void_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000799_rage_bkwidget_drawlocal_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_000808_rage_bkremotepacket_bkremotepacket(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000815_rage_bkmessagebox_char_const_ptr_char_const_ptr_rage_bkmessagebo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_000816_rage_bkremotepacket_printcommandline(ph: *mut Phantom); }
extern "C" { pub fn f_000817_rage_bkremotewidgetmap_getwidget_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_000818_rage_bkremotewidgetmap_getid_rage_bkwidget_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000822_rage_bkimageviewer_setimage_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000841_rage_bkcombo_bkcombo_rage_datcallback_ref_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_000846_rage_bkcombo_updatecombo_char_const_ptr_int_char_const_ptr_ptr_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_000851_char_ptr_rage_formatf_char_8ul_char_ref_8ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000867_rage_bkgroup_addtitle_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000869_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000870_rage_bkgroup_addbutton_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000874_rage_bkgroup_addtext_char_const_ptr_int_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000876_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_000877_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_000878_rage_bkgroup_addtext_char_const_ptr_rage_atnonfinalhashstring_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_000879_rage_bktext_bktext_rage_datcallback_ref_char_const_ptr_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_000881_rage_bkgroup_addtext_char_const_ptr_rage_atfinalhashstring_ptr_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_000882_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000885_rage_bkgroup_addtoggle_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_000887_rage_bkgroup_addtoggle_char_const_ptr_unsigned_short_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_000889_rage_bkgroup_addtoggle_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_000891_rage_bkgroup_addtoggle_char_const_ptr_rage_atbitset_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_000894_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000897_rage_bkgroup_addslider_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000899_rage_bkgroup_addslider_char_const_ptr_signed_char_ptr_signed_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_000901_rage_bkgroup_addslider_char_const_ptr_unsigned_short_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_000903_rage_bkgroup_addslider_char_const_ptr_short_ptr_short_short_shor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_000905_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000907_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int_rage_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000908_rage_bkgroup_addslider_char_const_ptr_rage_vector2_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64); }
extern "C" { pub fn f_000910_rage_bkgroup_addslider_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64); }
extern "C" { pub fn f_000916_rage_bkgroup_addcombo_char_const_ptr_unsigned_char_ptr_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_000918_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000919_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000923_rage_bkgroup_addvector_char_const_ptr_rage_vector2_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000924_rage_bkvector_checkinitialvalues(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000926_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000928_rage_bkgroup_addvector_char_const_ptr_rage_vector4_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000931_rage_bkgroup_addmatrix_char_const_ptr_rage_matrix33_ptr_float_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000932_rage_bkmatrix_checkinitialvalues(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000933_rage_bkgroup_addmatrix_char_const_ptr_rage_matrix34_ptr_float_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000935_rage_bkgroup_addmatrix_char_const_ptr_rage_matrix44_ptr_float_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000936_rage_bkgroup_addangle_char_const_ptr_float_ptr_rage_bkangletype(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000938_rage_bkgroup_addangle_char_const_ptr_float_ptr_rage_bkangletype(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i64); }
extern "C" { pub fn f_000941_rage_bkgroup_addcolor_char_const_ptr_rage_color32_ptr_rage_datca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_000947_rage_bkgroup_addcolor_char_const_ptr_rage_vector4_ptr_rage_datca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_000948_rage_bkgroup_addcolor_char_const_ptr_rage_vector3_ptr_rage_datca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_000949_rage_bktext_dtorbktext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000953_rage_bktext_gethashstringcstr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000964_rage_bkcolor_getcomponentf_rage_bkcolor_ecolorcomponent_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_000965_char_ptr_rage_safecatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000968_rage_bkcolor_getcomponent_rage_bkcolor_ecolorcomponent_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_000972_rage_bkcolor_remotewritecolor_rage_bkremotepacket_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000977_rage_bktreelist_addcolumn_char_const_ptr_rage_bktreelist_columnc(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_000979_rage_bktreelist_additem_int_char_const_ptr_rage_bktreelist_itemd(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_000990_char_ptr_rage_formatf_char_512ul_char_ref_512ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000991_char_ptr_rage_safecatf_char_512ul_char_ref_512ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001040_rage_matrix33_rotateto_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001041_rage_matrix33_inverse_rage_matrix33_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001042_rage_matrix33_normalizesafe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001047_rage_computerotation_rage_vector3_const_ref_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_001049_rage_matrix33_rotate_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_001050_rage_matrix33_makerotate_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_001062_rage_matrix34_inverse_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001101_rage_generaterandomautoidname_char_ref_10(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001103_rage_autoiddescriptor_getfullname_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001104_rage_autoiddescriptor_getreversefullname_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001110_void_std_2_introsort_std_2_classicalgpolicy_rage_autoiddescripto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_001111_void_std_2_introsort_std_2_classicalgpolicy_rage_autoiddescripto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_001114_rage_datbase64_encode_unsigned_char_const_ptr_unsigned_int_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_001115_rage_datbase64_decode_char_const_ptr_unsigned_int_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_001116_char_ptr_rage_formatf_char_32ul_char_ref_32ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001117_rage_datcallback_datcallback(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001119_rage_datcallback_datcallback_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001120_rage_datcallback_datcallback_void_ptr_void_ptr_void_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_001121_rage_datcallback_setbase_rage_datbase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001123_rage_datgrowbuffer_datgrowbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001124_rage_datgrowbuffer_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001125_rage_datgrowbuffer_dtordatgrowbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001126_rage_datgrowbuffer_init_rage_sysmemallocator_ptr_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001127_rage_datgrowbuffer_init_void_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001130_rage_datgrowbuffer_append_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001131_rage_datgrowbuffer_preallocate_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001132_rage_datgrowbuffer_appendorfail_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001138_rage_datgrowbuffer_getbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001140_rage_datbitbuffer_readunsigned_void_const_ptr_unsigned_int_ref_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001141_rage_datbitbuffer_writeunsigned_void_ptr_unsigned_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_001142_rage_datbitbuffer_copybits_void_ptr_void_const_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_001143_rage_datbitbuffer_datbitbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001147_rage_datbitbuffer_setreadonlybytes_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001148_rage_datbitbuffer_setreadwritebytes_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001156_rage_datbitbuffer_bumpreadcursor_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001157_rage_datbitbuffer_readbits_rage_datbitbuffer_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001158_rage_datbitbuffer_bumpreadwritecursor_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001162_bool_rage_datbitbuffer_readuns_unsigned_int_unsigned_int_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001168_rage_datbitbuffer_readbytes_void_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001169_rage_datbitbuffer_writebytes_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001173_rage_datcompress_unsigned_char_ptr_unsigned_int_unsigned_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_001174_rage_datdecompress_unsigned_char_ptr_unsigned_int_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_001175_rage_datdecompressinplace_unsigned_char_ptr_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_001176_rage_datresourceinfo_generatemap_rage_datresourcemap_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001177_rage_generatehelper_rage_datresourceinfo_sizes_unsigned_long_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_001186_rage_rijndael_setup_unsigned_char_const_ptr_int_int_rage_symmetr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001187_rage_rijndael_ecb_encrypt_unsigned_char_ptr_rage_symmetric_key_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001188_rage_rijndael_ecb_decrypt_unsigned_char_ptr_rage_symmetric_key_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001189_rage_rijndael_ecb_decrypt_mem_void_ptr_unsigned_int_rage_symmetr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001190_rage_aes_aes_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001191_rage_aes_aes_unsigned_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001192_rage_aes_istransformitkeyid_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_001193_rage_aes_encrypt_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001194_rage_aes_decrypt_unsigned_int_unsigned_int_void_ptr_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_001195_rage_aes_decrypt_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001196_rage_sha1_transform_unsigned_int_ptr_unsigned_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001197_rage_sha1_sha1(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001198_rage_sha1_clear_unsigned_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001199_rage_sha1_final_unsigned_char_ref_20(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001204_rage_rsonwriter_rsonwriter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001211_bool_rage_rsonwriter_write_char_char_const_ptr_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001212_rage_rsonwriter_write_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001217_bool_rage_rsonwriter_write_int_char_const_ptr_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_001219_bool_rage_rsonwriter_write_long_char_const_ptr_char_const_ptr_lo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_001223_bool_rage_rsonwriter_write_float_char_const_ptr_char_const_ptr_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_001225_bool_rage_rsonwriter_write_char_1_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001234_rage_rsonreader_init_char_const_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001236_rage_rsonreader_endofobject_char_const_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_001238_rage_rsonreader_getmember_char_const_ptr_rage_rsonreader_ptr_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001239_rage_rsonreader_getnextsibling_rage_rsonreader_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001248_rage_rsonreader_asuns_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001251_rage_rsonreader_asbool_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001252_rage_rsonreader_asatstring_rage_atstring_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001270_rage_jsonvalidator_validateobject_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001271_rage_jsonvalidator_validatestring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001272_rage_jsonvalidator_validatevalue(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001279_rage_datsaxreader_xmldecode_char_ptr_unsigned_int_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_001506_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_rage_autoi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001507_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_rage_autoi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_001508_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_autoi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_001509_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001510_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001511_rage_autoiddescriptor_ptr_ptr_std_2_partial_sort_impl_abi_nqn220(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001512_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_autoi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_001513_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001514_rage_diagdisplayf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001515_rage_diagchannel_closelogfile(ph: *mut Phantom); }
extern "C" { pub fn f_001516_rage_fistream_write_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001517_rage_diagchannel_diagchannel_rage_diagchannel_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_001519_rage_diagchannel_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001522_rage_diagchannel_sendsubchannels(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001523_quitf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001524_displayf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001525_rage_diagchannel_flushlogfile_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_001526_rage_diagchannel_initargs(ph: *mut Phantom); }
extern "C" { pub fn f_001527_warningf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001529_rage_fistream_create_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001530_rage_fistream_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001532_rage_fistream_close(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001533_rage_fidevice_getdevice_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001534_quitf_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001535_rage_diaglogf2_rage_diagchannel_const_ref_rage_diagseverity_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001536_rage_fistream_allocstream_char_const_ptr_fihandle_ptr_rage_fidev(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001537_rage_fistream_size(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001538_rage_diagchannel_setoutput_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_001539_rage_diagprintdefault_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001540_rage_bufferedoutputdebugstring_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001542_rage_diagprintf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001543_rage_diagloggedprintf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001550_char_ptr_rage_formatf_char_12ul_char_ref_12ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001551_rage_fprintf_rage_fistream_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001553_rage_firemoteshowmessagebox_char_const_ptr_char_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001558_rage_diagcontextmessage_push_char_const_ptr_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_001560_rage_diagcontextmessage_diagcontextmessage_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001561_rage_diagcontextmessage_diagcontextmessage_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_001563_rage_diagcontextmessage_pop(ph: *mut Phantom); }
extern "C" { pub fn f_001565_printf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001568_rage_atmapentry_unsigned_int_wchar_t_ptr_const_ref_rage_atmap_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001569_rage_atmap_unsigned_int_wchar_t_ptr_rage_atmaphashfn_unsigned_in(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001576_rage_fiassetmanager_fullpath_char_ptr_int_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_001577_rage_fistream_open_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001578_rage_fistream_seek_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001580_rage_fiassetmanager_open_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001581_rage_fiassetmanager_fullreadpath_char_ptr_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_001582_rage_fiassetmanager_createleadingpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001585_mg_cry(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001586_send_http_error(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_001587_mg_snprintf_mg_connection_ptr_char_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_001588_should_keep_alive_mg_connection_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001589_mg_printf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001590_mg_write(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001591_mg_fopen_mg_connection_ptr_char_const_ptr_char_const_ptr_file_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_001592_send_file_data_mg_connection_ptr_file_ptr_long_long_long_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_001594_mg_join_thread_rage_sysipcthreadid_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001595_free_context_mg_context_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001596_check_acl_mg_context_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001601_rage_restcommand_reporterror_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001602_rage_restcommand_reporterrormethodnotallowed_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001603_restprocessorresponse_rage_reststatus_enum_rage_restcommand_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_001605_rage_fidevice_makegrowbufferfilename_char_ptr_int_rage_datgrowbu(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001606_rage_restservice_restservice_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001607_rage_restservice_printfullname_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001608_rage_restservicedirectory_dtorrestservicedirectory(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001611_rage_restservicedirectory_addchild_rage_restservice_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001613_rage_restservicemanager_createdirectories_rage_atarray_rage_atst(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001618_rage_restservicefileaccess_restservicefileaccess_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001619_rage_restservicefileaccess_dtorrestservicefileaccess(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001622_rage_fistream_read_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001624_rage_fiassetmanager_setpath_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001625_rage_fiassetmanager_makerelative_char_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001626_rage_fiassetmanager_addextension_char_ptr_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001627_rage_fiassetmanager_exists_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001629_rage_fiassetmanager_getstackedpath_char_ptr_int_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001630_rage_atarray_rage_atbinarymap_rage_atstring_unsigned_int_datapai(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001632_rage_fiassetmanager_pushfolder_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001633_rage_fiassetmanager_popfolder(ph: *mut Phantom); }
extern "C" { pub fn f_001634_rage_fiassetmanager_getattributes_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001635_rage_fiassetmanager_filename_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001636_rage_fiassetmanager_basename_char_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001637_rage_fiassetmanager_enumfiles_char_const_ptr_void_ptr_rage_fifin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_001638_rage_atbinarymap_rage_mount_t_unsigned_int_safeget_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001639_rage_fiassetmanager_removeextensionfrompath_char_ptr_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001642_rage_fistream_open_char_const_ptr_rage_fidevice_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001644_rage_fistream_getch(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001645_rage_fistream_putch_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001646_rage_fistream_seek64_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001647_rage_fistream_tell(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001650_rage_fgetline_char_ptr_int_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_001652_rage_fidevice_makememoryfilename_char_ptr_int_void_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_001655_rage_fibasetokenizer_putstr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001673_rage_fibasetokenizer_gettoken_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001738_rage_fidevice_deletedirectory_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001739_rage_ficacheutils_listfiles_char_const_ptr_void_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001752_rage_encode_length_rage_encoderstate_ref_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001759_rage_fidevice_checkfilevalidity_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001764_rage_atarray_rage_atbinarymap_int_rage_atfinalhashstring_datapai(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001767_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_001770_rage_fidevice_mount_char_const_ptr_rage_fidevice_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001805_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_001815_rage_fidevice_saferead_fihandle_ptr_void_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_001874_void_rage_atplaceptrcreator_void_ptr_rage_fideviceinstaller_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001880_rage_fipackfile_fipackfile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001882_rage_fipackfile_reinit_char_const_ptr_bool_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_001884_rage_fipackfile_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001887_rage_fipackfile_fillparentindices_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001889_rage_fipackfile_openinternal_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001897_rage_fipackfile_readinternal_rage_fipackfilestate_ref_void_ptr_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_001908_int_rage_formatf_n_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001922_rage_clientrequestandwaitreplyex_fihandle_ptr_rage_remoteany_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i64, a8: i32); }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001947_rage_fideviceremote_dtorfideviceremote(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001949_rage_fidevicetcpip_setlasterror_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001950_rage_getsavefilepath_unsigned_int_char_ref_256_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_001951_rage_header_import_void_const_ptr_unsigned_int_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001995_rage_restservice_dtorrestservice_3059(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002010_parse_net_char_const_ptr_unsigned_int_ptr_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002011_skip_quoted_char_ptr_ptr_char_const_ptr_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002012_match_prefix_char_const_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_002013_send_ssi_file_mg_connection_ptr_char_const_ptr_file_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_002015_scan_directory_mg_connection_ptr_char_const_ptr_void_ptr_void_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002018_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002019_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002020_void_std_2_sort_heap_abi_nqn220108_std_2_classicalgpolicy_bool_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002021_rage_atbinarymap_int_rage_atfinalhashstring_datapair_ptr_std_2_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002022_std_2_pair_rage_atbinarymap_int_rage_atfinalhashstring_datapair(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002023_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002024_std_2_enable_if_is_move_constructible_rage_atbinarymap_rage_moun(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002025_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002026_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002027_rage_invsqrtf_fast_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_002031_rage_mthrandom_mthrandom_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002034_rage_mthrandommwc_mthrandommwc_unsigned_int_bool(ph: *mut Phantom); }
extern "C" { pub fn f_002037_rage_pgrscbuilder_constructname_char_ptr_int_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_002038_rage_pgstreamer_open_char_const_ptr_unsigned_int_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002040_rage_pgrscbuilder_allocatemap_rage_datresourcemap_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002041_rage_pgstreamer_read_unsigned_int_rage_datresourcechunk_ptr_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64) -> i64; }
extern "C" { pub fn f_002042_rage_pgrscbuilder_freememory_rage_datresourcemap_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002045_rage_wasm_announce_entry_unsigned_int_unsigned_int_unsigned_long(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_002048_rage_pgstreamer_initclass_int_rage_sysipcpriority_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_002049_rage_pgreaddata_init_rage_sysipcpriority_int_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_002055_rage_pgreaddata_request_rage_fidevice_const_ptr_fihandle_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i64, a10: i32, a11: i64, a12: i32, a13: i32); }
extern "C" { pub fn f_002056_rage_pgreaddata_processchunk(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002057_rage_pgreaddata_freeactivechunk_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002058_rage_pgreadbulk_rage_fidevice_const_ptr_fihandle_ptr_unsigned_lo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_002059_rage_atrangearray_rage_pgstreamerdevicehandler_2_atrangearray(ph: *mut Phantom); }
extern "C" { pub fn f_002060_rage_pgstreamerdevicehandler_processdecompression(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002061_rage_pgstreamerdevicehandler_markrequestprocessed_rage_pgstreame(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002062_rage_pgstreamerdevicehandler_getnextrequest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002067_rage_pgrawstreamer_checkentry_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002070_rage_pgrawstreamer_allocatehandle_rage_fidevice_const_ptr_fihand(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002080_rage_pgbase_pgbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002081_rage_pgbase_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002082_rage_pgbase_regeneratemap_rage_datresourcemap_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002084_rage_pgbase_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002085_rage_pgbasemetadata_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002086_rage_pgbaserefcounted_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002089_void_std_2_introsort_std_2_classicalgpolicy_rage_sortbyregionadd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_002091_rage_datresource_containsthisaddress_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002092_rage_datresource_getfixup_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002094_rage_pgbaseplatformneutral_pgbaseplatformneutral(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002109_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002110_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_sortb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002113_rage_atbinarymap_char_const_ptr_unsigned_int_insert_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002114_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_002118_rage_atmapentry_int_rage_parinputformatinfo_const_ref_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002121_rage_atmapentry_int_rage_paroutputformatinfo_const_ref_rage_atma(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002129_rage_atmapentry_rage_atnonfinalhashstring_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002130_rage_atmap_rage_atnonfinalhashstring_rage_atnonfinalhashstring_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002131_rage_atnonfinalhashstring_atnonfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002133_rage_partree_loadfromstream_rage_parstreamin_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002134_rage_partree_savetostream_rage_parstreamout_ptr_rage_partreenode(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002135_rage_atmap_int_rage_paroutputformatinfo_rage_atmaphashfn_int_rag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002136_rage_parmanager_trypreload_rage_fistream_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002137_rage_partreenodeutils_findandresolveallxincludes_rage_partreenod(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002141_rage_partreenode_dtorpartreenode(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002142_rage_parattributelist_findattribute_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002143_rage_parattribute_getstringrepr_char_ptr_int_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_002144_rage_parutils_iscurrentplatform_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002145_rage_parmanager_loadtree_rage_fistream_ptr_rage_parsettings_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002146_rage_parmanager_findinputformat_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002147_rage_parmanager_loadtree_char_const_ptr_char_const_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002149_rage_parmanager_savetree_rage_fistream_ptr_rage_partreenode_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_002151_rage_parutils_openwritablefilewithoptionalcheckout_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002152_rage_parutils_p4prompthelper_bool_bool_char_const_ptr_char_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002157_rage_parinstancevisitor_visitstructure_rage_parparsablestructure(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002159_rage_parmanager_loadfromstructure_rage_partreenode_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002166_rage_parmanager_createandloadanytype_rage_partreenode_ptr_rage_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_002167_rage_parstructure_create(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002169_rage_parmanager_buildtreenodefromstructure_rage_parstructure_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_002170_rage_parbuildtreevisitor_begintoplevelstruct_rage_parparsablestr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002172_rage_parmanager_savefromstructure_rage_fistream_ptr_rage_parstru(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_002174_rage_parelement_findattributestringvalue_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_002175_rage_partree_dtorpartree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002178_rage_parattributelist_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002181_rage_partreenode_findfromxpathinternal_char_const_ptr_rage_parat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002182_rage_parelement_findattribute_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002186_rage_parattribute_findint64value_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002187_rage_parattribute_findboolvalue_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002189_rage_partreenode_createstdleaf_char_const_ptr_double_bool(ph: *mut Phantom, a0: i64, a1: f64, a2: i32) -> i64; }
extern "C" { pub fn f_002190_rage_atarray_rage_parattribute_0_unsigned_short_grow_unsigned_in(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002192_rage_parattributelist_addattribute_char_const_ptr_double_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f64, a3: i32) -> i64; }
extern "C" { pub fn f_002193_rage_partreenode_createstdleaf_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002195_rage_parattributelist_addattribute_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002196_rage_partreenode_createstdleaf_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_002198_rage_parattributelist_addattribute_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_002199_rage_partreenode_createstdleaf_char_const_ptr_char_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002200_rage_parattributelist_addattribute_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_002202_rage_partreenode_setdata_char_const_ptr_unsigned_int_rage_parstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_002204_rage_parattribute_finddoublevalue_const(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_002206_rage_parelement_findattributeint64value_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002207_rage_partreenode_readstdleafvector3_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002208_rage_parstream_findencoding_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002213_rage_partreenode_clone(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002214_rage_parattributelist_copyfrom_rage_parattributelist_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002216_rage_partreenode_swapwith_rage_partreenode_ref_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002217_rage_partreenode_sortlistrecursive_rage_partreenode_ptr_rage_atd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002221_rage_psoswapschemacatalog_rage_psoschemacatalogdata_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002222_rage_psoswapstructarrays_char_ptr_rage_psoschemacatalog_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_002223_rage_psotype_convertoldstructarraytypetopsotype_unsigned_int_boo(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_002224_swapstructarray_rage_psotype_char_ptr_unsigned_long(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_002225_swapinstancememberdata_rage_psoschemacatalog_rage_psostructuresc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_002228_rage_psoifftorscconversion_converttonewformat_rage_psostructurem(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_002229_rage_psoschemacatalog_findmembersize_rage_psostructureschema_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002230_rage_psomemberarrayinterface_getarraycount(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002231_rage_psomemberarrayinterface_getarraycontents(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002233_rage_psomember_getsubstructure(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002235_rage_psomember_getwidestringdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002236_rage_psomember_getenumvalueandname_int_ref_rage_atnamespacedhash(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002237_rage_psodebugjsonwriter_writememberschema_rage_psomemberschema_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002238_rage_psodebugjsonwriter_writestructureschema_rage_psostructuresc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002239_rage_psodebugjsonwriter_writeenumvalue_rage_psoenumschema_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002240_rage_psodebugjsonwriter_writeenumschema_rage_psoenumschema_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002241_rage_psoresourcedata_copyschemadatato_rage_psoresourcedata_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002243_rage_atmap_rage_atstring_rage_psoresourcedata_ptr_rage_atmaphash(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002244_rage_psodebugmismatchedfiles_buildnewschemasonlyresourcefromparm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002245_rage_psobuildschemafromstructure_rage_psorscbuilder_ref_rage_par(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002246_rage_psorscbuilder_constructresourcedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002247_rage_psorscbuilderinstancedatacatalog_dtorpsorscbuilderinstanced(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002248_rage_psorscbuilderschemacatalog_dtorpsorscbuilderschemacatalog(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002249_rage_parstructure_findalign_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002250_addmembertoschema_rage_psorscbuilder_ref_rage_psorscbuilderstruc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_002251_rage_psorscbuilderinstancedatacatalog_addstringstoresource_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002252_rage_psorscenumschemadata_computebasesignature(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002253_rage_psoschemacatalog_computesignature_rage_psostructureschema_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002256_rage_parmemberstring_setfromstring_rage_parparsablestructure_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002258_rage_parmemberbitset_setbit_rage_parparsablestructure_ptr_int_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002259_rage_parmemberbitset_getsize_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002260_rage_parinstancevisitor_visitmember_rage_parparsablestructure_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002261_psofilereader_readparmemberfrompso_rage_parmember_ptr_rage_parpa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002263_void_psofilereader_copyintobuffer_rage_matrix34_void_ptr_rage_ma(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002264_rage_psotype_convertparsertypetopsotype_rage_parmembertype_enum(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_002265_rage_parmemberarray_setarraysize_rage_parparsablestructure_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_002266_rage_parmemberarray_getarraycontentsandcountfromstruct_rage_parp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002267_psofilereader_fixuppsomember_rage_psostruct_ref_rage_psomember_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002268_rage_parmemberarray_resizerawarray_rage_parparsablemember_ptr_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_002269_psofilereader_fixuppsomembers_rage_psostruct_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002270_rage_psocreateandloadfromstructure_rage_psostruct_ref_rage_parst(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002271_rage_psoloadfromstructure_rage_psostruct_ref_rage_parstructure_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_002272_rage_psoobjectfixup_buildfromstructure_rage_parstructure_ref_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002273_rage_psoobjectfixup_findvptr_rage_parstructure_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002274_rage_psoobjectfixup_buildfrommember_rage_parstructure_ref_rage_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_002275_rage_parstructure_place_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002276_rage_parstructure_findmember_rage_atnamespacedhashvalue_rage_ath(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002279_rage_psoresourcedata_deleteschemadata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002280_rage_atfinalhashstring_atfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002281_rage_psoifftorscconversion_converttonewformat_rage_psoschemacata(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002283_rage_psoloadfilefrombuffer_char_ptr_int_char_const_ptr_rage_psol(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_002284_psofilereader_compareschemaswithruntime_rage_psofile_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002286_rage_psorscstructschemadata_computebasesignature_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002287_rage_psoschemacatalog_computemembersignature_rage_psostructuresc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_002290_rage_psorscbuilderstructschema_findmembertraits_rage_psorscmembe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002291_rage_atarray_rage_psorscmemberschemadata_0_unsigned_short_grow_u(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002292_rage_psorscbuilderstructschema_addmembergeneric_rage_atnamespace(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_002293_rage_psorscbuilderstructschema_addmemberenum_rage_atnamespacedha(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_002294_rage_atarray_rage_psorscenumschemavaluedata_0_unsigned_short_gro(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002295_rage_atarray_rage_psorscbuilderinstanceset_ptr_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002296_rage_atmap_unsigned_int_rage_atarray_rage_psorscbuilderinstances(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002297_char_const_ptr_ref_rage_atmap_unsigned_int_char_const_ptr_rage_a(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002299_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_ps(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_002300_rage_psorscbuilder_convertpointerstostructids_member_rage_psostr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002301_buildschemafromenum_rage_psorscbuilder_ref_rage_parenumdata_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002303_rage_psobuilderstructschema_findmembertraits_rage_psoschemamembe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002305_rage_psobuilderstructschema_addmemberrawmemidxcount_rage_atnames(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_002306_rage_psobuilderstructschema_addmemberrawrefhash_rage_atnamespace(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002309_rage_psobuilderstructschema_copymember_rage_psoschemamemberdata(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002310_rage_psobuilderstructschema_finishbuilding(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002311_rage_atarray_rage_psoenumtableentry_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002314_rage_psobuilderinstanceset_dtorpsobuilderinstanceset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002316_rage_psobuilderinstancedatacatalog_findorcreateinstanceset_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_002317_rage_psobuilderinstancedatacatalog_computesize(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002320_rage_psobuilder_addstructinstances_rage_psobuilderstructschema_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_002322_rage_psobuilder_addpodarray_rage_parmembertype_enum_rage_parpars(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_002323_rage_psobuilder_finishbuilding(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002327_addmembertoschema_rage_psobuilder_ref_rage_psobuilderstructschem(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_002328_buildschemafromenum_rage_psobuilder_ref_rage_parenumdata_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002329_rage_psobuildschemafromstructure_rage_psobuilder_ref_rage_parstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002330_buildpsovisitor_visittoplevelstructure_rage_parparsablestructure(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002335_unsigned_int_ref_rage_atmap_rage_parparsablestructure_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002338_rage_parrestexpireguid_rage_parparsablestructure_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002340_rage_parrestprocessparsableobject_rage_parparsablestructure_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002341_processparsableobject_setfromfile_rage_restcommand_ref_rage_parf(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002342_processparsableobject_setfromstringdata_rage_restcommand_ref_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002344_rage_parneedstreeforoutputvisitor_visitstructure_rage_parparsabl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002345_rage_parxmlwritervisitor_begintoplevelstruct_rage_parparsablestr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002347_rage_parmemberstruct_getconcretestructure_rage_parparsablestruct(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002348_processparsableobject_setstructfromxmltree_rage_restcommand_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002349_rage_parmemberstruct_createnewdata_rage_partreenode_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002350_processparsableobject_setarrayfromxmltree_rage_restcommand_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002351_rage_parmemberarray_deleteitem_rage_parparsablestructure_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002352_rage_parenumdata_valuefromname_char_const_ptr_char_const_ptr_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002353_rage_parmemberbitset_setfromstring_rage_parparsablestructure_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002354_rage_parmemberbitset_unionblock_rage_parparsablestructure_ptr_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_002355_rage_parfindmemberfromuripath_helper_rage_parparsablestructure_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_002361_rage_parstructure_dtorparstructure_3632(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002362_rage_parstructure_buildstructurefromstaticdata_rage_parstructure(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002363_rage_parstreaminxml_addstaticstring_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002364_rage_parstructure_creatememberfrommemberdata_rage_parmembercommo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002365_rage_athashedmap_char_const_ptr_char_const_ptr_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002367_void_rage_parstructure_invoke_rage_partreenode_ptr_rage_parparsa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002368_void_rage_parstructure_invoke_rage_partreenode_ptr_rage_parmembe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_002370_rage_parattributelist_mergefrom_rage_parattributelist_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002371_void_std_2_introsort_std_2_classicalgpolicy_attrless_ref_rage_pa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_002372_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_002375_rage_parrtstructure_dtorparrtstructure(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002379_rage_parmember_loadextraattributes_rage_partreenode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002380_rage_parmembersimple_dtorparmembersimple(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002390_rage_parmemberarray_dtorparmemberarray(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002400_rage_parmemberenum_dtorparmemberenum_3681(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002407_rage_parmemberbitset_generatestringrepr_rage_parparsablestructur(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_002409_rage_parmembermap_dtorparmembermap(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002414_rage_parmemberatbinarymapgenericinterface_parmemberatbinarymapge(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002436_rage_parcguregisterstructure_rage_parstructure_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002437_rage_parcgustructure_createstructure_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002438_rage_parcgustructure_setfactories_rage_parstructure_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002439_rage_parcgustructure_setgetstructurecb_rage_parstructure_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002440_rage_parcgustructure_setbaseclass_rage_parstructure_ptr_rage_par(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002444_rage_parcgustructure_buildstructurefromstaticdata_void_ptr_rage(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002448_int_rage_formatf_n_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002492_void_rage_parstreamoutxml_writeintarraystring_unsigned_char_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002493_void_rage_parstreamoutxml_writeintarraystring_unsigned_short_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002494_void_rage_parstreamoutxml_writeintarraystring_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002504_void_rage_parstreamoutxml_writeintarraystring_char_rage_fistream(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002505_void_rage_parstreamoutxml_writeintarraystring_short_rage_fistrea(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002506_void_rage_parstreamoutxml_writeintarraystring_long_rage_fistream(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002507_void_rage_parstreamoutxml_writefloatarraystring_float_rage_fistr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_002508_void_rage_parstreamoutxml_writefloatarraystring_double_rage_fist(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002586_void_rage_parcgudestructor_rage_parattributelist_rage_parparsabl(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002598_rage_parstreaminxml_pushback_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002599_rage_parstreaminxml_pushbackandescape_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002600_rage_parstreaminxml_internalreadutf16char_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002601_rage_parstreaminxml_peek_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002602_rage_parstreaminxml_match_char_const_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002603_rage_parstreaminxml_skipwhitespaceandcomments_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002604_int_rage_parstreaminxml_readanytoken_refrage_parstreaminxml_isxm(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002605_rage_parstreaminxml_skipto_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002606_int_rage_parstreaminxml_readanytoken_refrage_parstreaminxml_isfl(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002607_rage_parstreaminxml_readdatainternal_char_ptr_int_rage_parstream(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_002608_int_rage_parstreaminxml_readfloatarraystring_float_float_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002609_int_rage_parstreaminxml_readanytoken_refrage_parstreaminxml_isin(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002610_rage_parstreaminxml_readelementinternal_rage_parelement_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002612_rage_parstreaminxml_findutfencodingfromheader(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002619_rage_parstreamoutxml_writeescapedcharacter_rage_fistream_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_002622_rage_parstreamoutxml_writebeginelementinternal_rage_parelement_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002629_rage_parstreaminrbf_readfullrecordid_unsigned_short_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_002630_rage_parstreaminrbf_readelementinternal_rage_parelement_ref_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002641_rage_parstreamoutrbf_writedatainternal_char_const_ptr_int_rage_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_002643_rage_parstreamoutrbf_writerecordid_unsigned_short_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_002644_rage_atmap_rage_conststring_unsigned_int_rage_atmaphashfn_rage_c(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002654_rage_parinitvisitor_defaultstringmember_rage_parmemberstring_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002672_void_swapmeas_rage_mat33v_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002673_void_swapmeas_rage_mat34v_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002690_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002691_rage_psorscbuilderinstance_ptr_ptr_std_2_partial_sort_impl_abi_n(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002692_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002693_rage_psorscbuilderinstance_ptr_ptr_std_2_partition_with_equals_o(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002694_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002695_rage_atbinarymap_rage_pardelegateholderbase_ptr_unsigned_int_dat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002696_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002697_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_002698_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_attrless_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002699_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_attrless_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002700_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002703_rage_qabug_setid_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002705_rage_qabug_beginfield_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002708_rage_qabug_setproduct_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002714_rage_qabugstarinterface_sendcustomfield_binary_char_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002719_strlwr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002720_rage_conststring_vcopy_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002721_rage_conststring_dtorconststring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002739_rage_atpartialstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002741_rage_atliteralstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002742_rage_atpartialdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002743_rage_atdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002744_rage_atdatahash_unsigned_int_const_ptr_unsigned_long_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002746_rage_stristr_char_ptr_char_const_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002749_rage_stringtrim_char_ptr_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002753_rage_stringisequal_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002754_rage_stringnullorempty_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002756_rage_widestringduplicate_unsigned_short_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002757_rage_countutf8characters_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002758_rage_widetoutf8char_unsigned_short_char_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_002759_rage_utf8towidechar_char_const_ptr_unsigned_short_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002760_rage_widetoutf8_char_ptr_unsigned_short_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002762_rage_utf8towide_unsigned_short_ptr_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002765_rage_anonymous_namespace_pututf8_rage_anonymous_namespace_u16wri(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002766_crc32(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002770_deflateend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002771_init_block_internal_state_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002772_adler32(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002774_flush_pending_z_stream_s_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002775_tr_stored_block_internal_state_ptr_char_ptr_unsigned_long_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_002776_inflatereset(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002777_inflateinit2(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002779_inflate(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002780_inflate_table_codetype_unsigned_short_ptr_unsigned_int_code_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002781_inflateend(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002782_lz4_compressbound(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_002785_tr_flush_block_internal_state_ptr_char_ptr_unsigned_long_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_002786_build_tree_internal_state_ptr_tree_desc_s_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002787_compress_block_internal_state_ptr_ct_data_s_ptr_ct_data_s_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002788_send_tree_internal_state_ptr_ct_data_s_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002789_pqdownheap_internal_state_ptr_ct_data_s_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002790_rage_zlibinflater_zlibinflater(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002791_rage_zlibinflater_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002793_fill_window_internal_state_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002795_longest_match_internal_state_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002796_longest_match_fast_internal_state_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002801_rage_sysexception_hasbeenthrown(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002802_rage_syshangdetectsavezoneenter(ph: *mut Phantom); }
extern "C" { pub fn f_002803_rage_syshangdetectsavezoneexit_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002808_rage_atdelegator1_void_rage_sysserviceevent_ptr_removedelegate_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002812_rage_sysnamedpipe_sysnamedpipe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002814_rage_sysnamedpipe_open_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002815_rage_sysnamedpipe_read_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002816_rage_sysnamedpipe_saferead_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002817_rage_sysnamedpipe_write_void_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002819_rage_sysnamedpipe_close(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002820_rage_sysnamedpipe_setnodelay_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002821_rage_sysnamedpipe_setblocking_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002822_rage_syspipepacket_write_u8_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002823_rage_syspipepacket_write_u16_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002824_rage_syspipepacket_write_u32_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002825_rage_syspipepacket_write_u64_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002826_rage_syspipepacket_write_float_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_002827_rage_syspipepacket_write_const_char_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002828_rage_syspipepacket_read_u8_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002829_rage_syspipepacket_read_u16_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002830_rage_syspipepacket_read_u32_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002831_rage_syspipepacket_read_float_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_002832_rage_syspipepacket_read_const_char_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002834_rage_sysparam_findcommandlinearg_char_const_ptr_char_ptr_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002835_rage_sysparam_sysparam_int_char_const_ptr_bool_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002836_rage_sysparam_help_char_const_ptr_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002842_rage_sysparam_getarray_int_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002848_rage_sysstack_opensymbolfile(ph: *mut Phantom); }
extern "C" { pub fn f_002850_rage_sysstack_parsemapfilesymbol_char_ptr_int_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002852_rage_sysstack_printcapturedstacktrace_unsigned_long_const_ptr_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002860_rage_recurseresponsefile_char_const_ptr_int_ref_char_ptr_ptr_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_002863_rage_sysperformancetimer_sysperformancetimer_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002869_rage_sysis64bit_char(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_002870_rage_sysgetbyteswap_char(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_002871_rage_sysgetenv_char_const_ptr_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002876_rage_sysstackcollector_tagentry_ref_rage_atmap_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002878_rage_atmap_unsigned_int_rage_sysstackcollector_tagentry_rage_atm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002880_rage_sysstackcollector_stackentry_ref_rage_atmap_unsigned_int_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002881_unsigned_int_ref_rage_atmap_unsigned_int_unsigned_int_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002885_rage_systhreadpool_thread_thread(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002887_rage_systhreadpool_removethread_rage_systhreadpool_thread_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002888_rage_systhreadpool_workitem_workitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002897_rage_systhreadpool_systhreadpool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002899_rage_systhreadpool_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002900_rage_systhreadpool_cancelwork_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002909_rage_systhreadinfo_adjustpriobasedonparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002910_rage_systhreadregistry_registerthread_rage_sysipcthreadid_ptr_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_002917_rage_atdelegator1_void_rage_sysserviceevent_ptr_dispatch_rage_sy(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002921_rage_sysservice_getsystemlanguage(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002927_rage_sysmemstarttemp(ph: *mut Phantom); }
extern "C" { pub fn f_002928_rage_sysmemendtemp(ph: *mut Phantom); }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_002931_rage_physical_new_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002934_rage_sysmemvirtualallocate_unsigned_long_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002936_rage_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002939_rage_sysmemmanager_sysmemmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002946_rage_sysmempoolallocator_allocate_unsigned_long_unsigned_long_bo(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002948_rage_sysmempoolallocator_free_void_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002950_rage_sysmemsimpleallocator_sysmemsimpleallocator_void_ptr_int_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_002951_rage_sysmemsimpleallocator_initheap_void_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_002952_rage_sysmemsimpleallocator_dtorsysmemsimpleallocator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002954_rage_sysmemsimpleallocator_setquitonfail_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002955_rage_sysmemsimpleallocator_allocate_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002956_rage_sysmemsimpleallocator_doallocate_unsigned_long_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002957_rage_sysmemsimpleallocator_printmemoryusage_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002967_rage_dumpmemorynear_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002978_rage_sysmemmultiallocator_dtorsysmemmultiallocator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002979_rage_sysmemmultiallocator_addallocator_rage_sysmemallocator_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003015_rage_sysbuddyheap_allocaterecurse_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_003035_rage_sysmemgrowbuddyallocator_sysmemgrowbuddyallocator_unsigned(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003036_rage_sysmemgrowbuddyallocator_growheap(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003075_rage_systinyheap_init_void_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003076_rage_systinyheap_allocate_unsigned_long_unsigned_long_unsigned_l(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_003078_rage_systinyheap_free_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003079_rage_sysmemsparseallocator_sysmemsparseallocator_unsigned_long_i(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003080_rage_sysmemsparseallocator_dtorsysmemsparseallocator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003085_rage_systaskparameters_systaskparameters(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003087_rage_systaskscheduler_systaskscheduler_char_const_ptr_int_int_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_003089_rage_systaskmanager_setup_char_const_ptr_void_ptr_rage_systaskpa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_003098_rage_systasklog_dump(ph: *mut Phantom); }
extern "C" { pub fn f_003099_rage_syscompressionfactory_dtorsyscompressionfactory(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003101_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_003102_rage_syscompressiondata_syscompressiondata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003103_rage_syscompressiondata_dtorsyscompressiondata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003110_rage_chain_pop(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_003111_rage_sysdependencyscheduler_insertinternal_rage_sysdependency_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003112_rage_sysdependencyscheduler_outputdebug(ph: *mut Phantom); }
extern "C" { pub fn f_003152_rage_geomboxes_testsegmenttobox_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64) -> i32; }
extern "C" { pub fn f_003159_rage_matrix34_fromquaternion_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003161_rage_quaternion_frommatrix34_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003162_rage_matrix34_interpolate_rage_matrix34_const_ref_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_003164_rage_matrix34_mirroronplane_rage_vector4_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003166_rage_matrix44_inverse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003167_rage_matrix44_makerotx_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_003168_rage_matrix44_makerotz_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_003177_rage_vector3_rotateaboutaxis_float_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_003178_rage_vector3_reflectabout_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003179_rage_atpoolbase_init_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003182_rage_atpoolbase_releasemem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003185_rage_crframe_crframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003186_rage_crframe_crframe_rage_crframedata_const_ref_unsigned_char_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_003187_rage_crframe_init_rage_crframedata_const_ref_unsigned_char_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_003189_rage_crframe_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003191_rage_crframeiterator_rage_setiterator_iteratepair_rage_crframe_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003192_rage_crframe_set_rage_crframe_const_ref_rage_crframefilter_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003198_rage_crframe_identityfromskel_rage_crskeletondata_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003199_rage_crframe_compositemoveronly_rage_cranimation_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_003200_rage_crframe_composite_rage_cranimation_const_ref_float_rage_crf(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003201_rage_crframeiterator_rage_compositeiterator_iterateanim_rage_cra(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003202_rage_crframe_calcmoverdeltainternal_rage_cranimation_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: i64) -> i32; }
extern "C" { pub fn f_003203_rage_crframe_setsituation_unsigned_char_unsigned_char_unsigned_s(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003204_rage_crframe_compositewithdelta_rage_cranimation_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003206_rage_crframe_pose_rage_crskeleton_ref_bool_rage_crframefilter_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003207_rage_mat33vfromquatv_rage_mat33v_ref_rage_quatv_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003208_rage_crframe_inversepose_rage_crskeleton_const_ref_bool_rage_crf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003210_rage_crframe_swapdofvalue_unsigned_char_unsigned_short_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_003211_rage_crframe_getsituation_unsigned_char_unsigned_char_unsigned_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_003212_rage_crframe_getmatrix_unsigned_char_unsigned_char_unsigned_shor(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_003213_rage_crframe_setmatrix_unsigned_char_unsigned_char_unsigned_shor(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_003214_rage_crframe_getvector3_unsigned_char_unsigned_short_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_003215_rage_crframe_setvector3_unsigned_char_unsigned_short_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_003217_rage_crframe_getfloat_unsigned_char_unsigned_short_float_ref_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_003218_rage_crframe_setfloat_unsigned_char_unsigned_short_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32) -> i32; }
extern "C" { pub fn f_003220_rage_crframe_calcmoverinternalunsafe_rage_cranimation_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_003221_rage_crframe_dtorcrframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003224_rage_crvcrwidget_buttonpressed_rage_crvcrwidget_button(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003233_rage_atarray_rage_datref_rage_crblockstream_0_unsigned_short_dec(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003234_rage_cranimation_dtorcranimation(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003235_rage_cranimation_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003236_rage_cranimtrack_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003243_rage_crframefilter_registertypeinfo_rage_crframefilter_typeinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003248_rage_cranimation_addtrack_rage_cranimtrack_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_003250_rage_cranimpacker_pack_bool_bool_rage_cranimation_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_003251_rage_atarray_rage_datref_rage_crblockstream_0_unsigned_short_res(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003253_rage_cranimation_load_char_const_ptr_unsigned_char_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_003254_rage_cranimation_serializeani8_rage_crarchive_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003261_rage_createiterator_true_iterate_rage_crframefilter_ptr_float_bo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003262_rage_cranimation_computesize_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003263_rage_cranimcache_cranimcache_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003264_rage_cranimcache_findanimblock_rage_craccelerator_lock_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_003266_rage_craccelerator_insertinternal_unsigned_long_unsigned_int_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_003268_rage_atpackedarray_serialize_rage_crarchive_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003269_rage_atpackedarray_init_unsigned_int_rage_atarray_unsigned_int_0(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_003270_rage_atpackedsequence_serialize_rage_crarchive_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003271_rage_atpackedsequence_initandcreate_rage_atarray_int_0_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_003276_rage_cranimtrack_createvector3_rage_atarray_rage_vec3v_0_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_003277_rage_cranimchunk_createfloatchannel_float_ptr_int_int_float_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i64) -> i64; }
extern "C" { pub fn f_003278_rage_cranimtrack_createquaternion_rage_atarray_rage_quatv_0_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_003279_rage_cranimchunk_optimizetolerance_unsigned_int_ref_unsigned_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_003280_rage_cranimtrack_createfloat_rage_atarray_float_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_003282_rage_cranimtrack_evaluatevector3_float_rage_vec3v_ref_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003283_rage_cranimtrack_evaluatequaternion_float_rage_quatv_ref_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003284_rage_cranimchunk_evaluatequaternion_float_rage_quatv_ref_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003285_rage_cranimtrack_evaluatefloat_float_float_ref_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003296_rage_atarray_unsigned_short_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003298_rage_cranimdictionary_dtorcranimdictionary(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003299_rage_cranimdictionary_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003310_rage_crblockstream_findchannels_unsigned_int_unsigned_int_ref_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_003311_rage_crblockstream_evaluateframe_unsigned_char_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64); }
extern "C" { pub fn f_003312_rage_crblockstream_evaluatechannels_unsigned_char_ptr_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32, a5: i64); }
extern "C" { pub fn f_003313_rage_crframedata_crframedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003314_rage_crframedata_dtorcrframedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003315_rage_crframedata_init_unsigned_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_003316_rage_crframedata_reservedofs_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003317_rage_crframedata_sortdofs_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003318_rage_crframedata_calcoffsets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003319_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_003320_rage_crframedata_calcsignature(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003321_rage_crframedata_fastadddof_unsigned_char_unsigned_short_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_003322_rage_crframefilter_crframefilter_rage_crframefilter_eframefilter(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003329_rage_crframefilter_typeinfo_typeinfo_rage_crframefilter_eframefi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_003336_rage_crframefilterbone_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003341_rage_crframefilterbonebasic_crframefilterbonebasic_rage_crframef(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003342_rage_crweightset_crweightset_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003343_rage_crframefilterbonebasic_crframefilterbonebasic_rage_datresou(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003367_rage_crframefiltermultiweight_crframefiltermultiweight_rage_datr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003398_rage_crframedatainitializer_initializeframedata_rage_crframedata(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003399_rage_crframedata_iterator_rage_crframedatainitializer_addframeda(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003402_rage_crframedatainitializerboneandmover_crframedatainitializerbo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_003405_rage_crframedatainitializerboneandmover_initializeskeleton_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_003410_rage_crframedatainitializeranimation_crframedatainitializeranima(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_003414_rage_craccelerator_lock_lock(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003416_rage_craccelerator_lock_dtorlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003417_rage_crframeaccelerator_crframeaccelerator(ph: *mut Phantom); }
extern "C" { pub fn f_003420_rage_crframeaccelerator_getsizeframeanimindices_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003421_rage_crframeaccelerator_findframeanimindices_rage_craccelerator(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_003423_rage_crframeaccelerator_calcframeanimindices_rage_crframedata_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003427_rage_crframedatafactory_crframedatafactory(ph: *mut Phantom); }
extern "C" { pub fn f_003428_rage_crframedatafactory_allocateframedata_rage_crcreature_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_003429_rage_crcommonpool_allocate_unsigned_char_ptr_ptr_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_003430_rage_atmapentry_unsigned_int_rage_crframedata_ptr_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003432_rage_crframedatafactory_deleteframedata_rage_crframedata_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003433_rage_atmap_unsigned_int_rage_crframedata_ptr_rage_atmaphashfn_un(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003438_rage_crframebufferframedata_setframedata_rage_crframedata_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003439_rage_crframebufferframedata_allocateframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003441_rage_crcommonpool_crcommonpool(ph: *mut Phantom); }
extern "C" { pub fn f_003458_rage_crarchive_op_rage_vec3v_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003463_rage_crarchive_op_rage_quatv_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003501_void_rage_compute_float_float_ptr_float_ptr_int_int_float_int_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: i64, a6: i64); }
extern "C" { pub fn f_003503_rage_createbspline_int_float_const_ptr_int_int_int_rage_atpacked(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: f32, a6: i64, a7: i64, a8: i64) -> i32; }
extern "C" { pub fn f_003504_rage_cranimchannelquantizefloat_quantizefloats_float_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_003526_rage_cranimchannelsmallestthreequaternion_reconstructquaternion(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_003530_rage_cranimchannelsmallestthreequaternion_quantizedfloats_serial(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003545_rage_cranimchannellinearfloat_reconstructfloat_unsigned_int_cons(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_003551_rage_criksolverarms_armgoal_solve_rage_criksolverbase_solverhelp(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003553_rage_criksolverlegs_goal_solve_rage_criksolverbase_solverhelper(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003556_void_std_2_introsort_std_2_classicalgpolicy_rage_solvertypesort(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_003568_rage_crcreaturecomponent_crcreaturecomponent_rage_datresource_re(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003569_rage_crcreaturecomponent_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003574_rage_crcreaturecomponent_creaturecomponenttypeinfo_creaturecompo(ph: *mut Phantom); }
extern "C" { pub fn f_003607_rage_mat34v_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003622_rage_crcreaturecomponentextradofs_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003679_rage_crcreaturecomponentroot_initdofs_rage_crframedatainitialize(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_003779_rage_atrangearray_rage_cranimchannelsmallestthreequaternion_quan(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003790_rage_matrixt_double_matrixt_short_short(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_003791_void_rage_compute_double_double_ptr_double_ptr_int_int_double_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f64, a5: i64, a6: i64); }
extern "C" { pub fn f_003794_rage_atmapentry_unsigned_int_rage_datowner_rage_cranimation_decl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003795_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_003796_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_003797_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003802_rage_fwanimdirector_addcomponent_rage_fwanimdirectorcomponent_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003803_rage_fwanimdirector_initmidphysicsmotiontree(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003804_rage_fwanimdirector_initexpressions_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003805_rage_fwanimdirectorcomponentmotiontree_init_rage_fwanimdirector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_003806_rage_fwanimdirector_waitforanyactiveupdatetocomplete_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003807_rage_fwanimdirector_shutdownexpressions(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003808_rage_fwanimdirector_addexpression_rage_crexpressions_const_ref_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_003809_rage_crframedata_adddof_unsigned_char_unsigned_short_unsigned_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_003812_rage_fwanimdirectorcomponentexpressions_removeallexpressions(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003813_rage_fwanimdirector_dtorfwanimdirector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003816_rage_fwanimdirector_updatemove_float_bool_rage_fwanimdirector_pt(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003821_rage_fwanimdirectorcomponentmove_swapoutputbuffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003824_rage_fwanimdirector_startupdatepostcamera_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_003827_rage_fwmovenetworkinterface_updateaudioandtriggeredfacialevents(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003829_rage_fwanimdirectorcomponentsyncedscene_issyncedscenelooped_shor(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003830_rage_crtagiteratorbase_rage_crtagiteratorkey_true_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003838_rage_fwclipset_getclip_rage_fwmvtemplateid_rage_fwmvclipidtype_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_003841_rage_fwclipset_getclipdictionary_bool_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003843_rage_fwanimdirector_findnetworkdefname_rage_mvnetworkdef_const_p(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003845_rage_fwdictionaryloader_rage_mvnetworkdef_rage_fwnetworkdefstore(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003851_rage_fwanimdirectorcomponentexpressions_init_rage_fwanimdirector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003865_rage_fwdictionaryloader_rage_pgdictionary_rage_crexpressions_rag(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003866_rage_fwdictionaryloader_rage_pgdictionary_rage_crframefilter_rag(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003868_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_003869_rage_fwanimdirectorcomponentsyncedscene_resizesyncedscenepool_in(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_003871_rage_fwanimdirectorcomponentsyncedscene_dumpsynchronizedscenedeb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003872_rage_fwanimdirectorcomponentsyncedscene_syncedscene_dtorsyncedsc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003873_rage_fwanimdirector_queue_pop(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_003877_rage_fwanimhelpers_getcameratracktranslation_rage_crclip_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003878_rage_fwanimhelpers_getmovertracktranslationdiff_rage_crclip_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003880_rage_fwanimhelpers_getmovertrackrotation_rage_crclip_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003881_rage_fwanimhelpers_getcameratrackrotation_rage_crclip_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003882_rage_fwanimhelpers_getmovertrackrotationdiff_rage_crclip_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003883_rage_fwanimhelpers_getcameratrackrotationdiff_rage_crclip_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003884_rage_fwanimhelpers_getmovertrackrotationdelta_rage_crclip_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003887_rage_fwanimhelpers_getmovertrackdisplacement_rage_crclip_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003888_rage_fwanimhelpers_getmovertracktranslationdiffrotated_rage_crcl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_003901_rage_fwanimmanager_getlocaltranslation_rage_crbonedata_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_003902_rage_fwanimmanager_getlocalrotation_rage_crbonedata_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_003921_rage_fwclipitemwithprops_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_003927_rage_fwclipset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_003947_rage_fwmemorysituation_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_003957_rage_fwclipsetmanager_dtorfwclipsetmanager(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003972_rage_fwclipset_pickrandomclip_rage_fwmvtemplateid_rage_fwmvclipi(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_003973_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_003974_rage_atarray_rage_atbinarymap_rage_fwclipitem_ptr_rage_fwmvtempl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003976_rage_fwclipset_streamin_deprecated(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003977_rage_fwclipset_request_deprecated_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_003979_rage_fwclipset_isstreamedin_deprecated_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003980_rage_fwclipset_addref_deprecated(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003981_rage_fwclipset_release_deprecated(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003982_rage_fwclipset_createclipitem_rage_fwmvtemplateid_rage_fwmvclipi(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003984_rage_fwclipsetrequesthelper_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003997_rage_fwclipsetmanager_getclipdictionarynameandclipname_rage_fwmv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_003999_rage_fwclipsetmanager_load(ph: *mut Phantom); }
extern "C" { pub fn f_004000_rage_fwclipsetmanager_rebuildclipsetnames(ph: *mut Phantom); }
extern "C" { pub fn f_004002_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_004003_rage_atarray_rage_atbinarymap_rage_fwclipdictionarymetadata_rage(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004004_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_004005_rage_atarray_rage_atbinarymap_rage_fwmemorygroupmetadata_rage_at(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004006_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_004007_rage_fwclipsetmanager_buildclipdictionarynamearray(ph: *mut Phantom); }
extern "C" { pub fn f_004016_rage_atarray_rage_atbinarymap_rage_fwmemorysituation_rage_atnonf(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004017_rage_atarray_rage_fwmemorysituation_adjustment_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004018_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_004019_rage_fwclipsetrequesthelper_request_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_004023_rage_fwclipsetmanager_buildclipnamearray(ph: *mut Phantom); }
extern "C" { pub fn f_004027_rage_fwclipsetmanager_selectclipset(ph: *mut Phantom); }
extern "C" { pub fn f_004035_rage_fwdictionaryloader_rage_crclipdictionary_rage_fwclipdiction(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004036_rage_fwclipsetmanager_updateclipsetcombobox(ph: *mut Phantom); }
extern "C" { pub fn f_004038_rage_fwclipsetrequesthelper_fwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004044_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_004057_rage_fwexpressionsetmanager_deleteallexpressionsets(ph: *mut Phantom); }
extern "C" { pub fn f_004060_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_004067_rage_fwfacialclipsetgroup_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_004076_rage_fwfacialclipsetgroupmanager_deleteallfacialclipsetgroups(ph: *mut Phantom); }
extern "C" { pub fn f_004080_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_004081_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_004086_rage_fwframefilter_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_004090_rage_fwframefilterdictionary_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_004097_rage_fwmovenetworkinterface_clearclipsets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004098_rage_fwmovenetworkinterface_dtorfwmovenetworkinterface(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004113_rage_fwmovedumpnode_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_004144_rage_fwmovedumpnodeparent_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_004280_rage_fwposematcher_fwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004314_rage_fwsyncedsceneentitymanager_fwsyncedsceneentitymanager(ph: *mut Phantom); }
extern "C" { pub fn f_004378_rage_fwsyncedsceneaudiointerface_fwsyncedsceneaudiointerface(ph: *mut Phantom); }
extern "C" { pub fn f_004562_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004563_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_004564_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_004565_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_004576_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_004577_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004578_void_std_2_sort_heap_abi_nqn220108_std_2_classicalgpolicy_bool_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004579_rage_atbinarymap_rage_fwclipitem_ptr_rage_fwmvtemplateid_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_004580_std_2_pair_rage_atbinarymap_rage_fwclipitem_ptr_rage_fwmvtemplat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004581_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_004582_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004583_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_004584_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_004585_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_004586_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004587_void_std_2_sort_heap_abi_nqn220108_std_2_classicalgpolicy_bool_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004588_rage_atbinarymap_rage_fwmemorygroupmetadata_rage_atnonfinalhashs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_004589_std_2_pair_rage_atbinarymap_rage_fwmemorygroupmetadata_rage_atno(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004590_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_004591_void_std_2_iterops_std_2_classicalgpolicy_iter_swap_abi_nqn22010(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004592_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_004593_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_004594_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004595_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_004596_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_004661_rage_fwdynamicentitycomponent_fwdynamicentitycomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004765_rage_fwdebugbank_removeallmainwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004767_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004772_rage_fwpickermanager_fwpickermanager(ph: *mut Phantom); }
extern "C" { pub fn f_005239_rage_autoiddescriptor_t_rage_fwextension_autoiddescriptor_t_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_005320_rage_fwassetlocation_set_rage_estoretype_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005478_rage_fwclipdictionarydef_findclipifexists_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005479_rage_fwassetstore_rage_crclipdictionary_rage_fwclipdictionarydef(ph: *mut Phantom); }
extern "C" { pub fn f_005481_rage_fwclipdictionarystore_getnumrefs_rage_strlocalindex_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005486_rage_fwclipdictionarystore_addref_rage_strlocalindex_rage_strref(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005488_rage_fwclipdictionarystore_removeref_rage_strlocalindex_rage_str(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005490_rage_fwclipdictionarystore_removerefwithoutdelete_rage_strlocali(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005499_rage_fwassetstore_rage_pgdictionary_rage_charactercloth_rage_fwc(ph: *mut Phantom); }
extern "C" { pub fn f_005509_rage_fwassetstore_rage_rmcdrawable_rage_fwdrawabledef_fwassetsto(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005524_rage_fwassetstore_rage_pgdictionary_rage_rmcdrawable_rage_dwddef(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005542_rage_fwassetstore_rage_fragtype_rage_fwfragmentdef_fwassetstore(ph: *mut Phantom); }
extern "C" { pub fn f_005572_rage_fwmapdatastore_fwmapdatastore(ph: *mut Phantom); }
extern "C" { pub fn f_005589_rage_fwpsostoreloader_unload_rage_fwpsostoreloadinstance_ref_boo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005616_rage_fwassetstore_rage_fwmaptypescontents_rage_fwmaptypesdef_fwa(ph: *mut Phantom); }
extern "C" { pub fn f_005638_rage_fwassetstore_rage_mvnetworkdef_rage_fwassetdef_rage_mvnetwo(ph: *mut Phantom); }
extern "C" { pub fn f_005642_rage_fwassetstore_rage_crposematcher_rage_fwassetdef_rage_crpose(ph: *mut Phantom); }
extern "C" { pub fn f_005647_rage_fwpsostoreloader_fwpsostoreloader(ph: *mut Phantom); }
extern "C" { pub fn f_005648_rage_fwpsostoreloader_load_char_const_ptr_rage_fwpsostoreloadins(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_005651_rage_fwpsostoreloader_makesimplefileloadercore(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005659_rage_fwassetstore_rage_phbound_rage_fwbounddef_fwassetstore_char(ph: *mut Phantom); }
extern "C" { pub fn f_005694_rage_fwtextstore_fwtextstore(ph: *mut Phantom); }
extern "C" { pub fn f_005699_rage_fwassetstore_rage_pgdictionary_rage_grctexture_rage_fwtxdde(ph: *mut Phantom); }
extern "C" { pub fn f_005730_rage_fwinteriorlocation_makeinvalid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005796_rage_fwscannodesdebug_setbreakonselectedentityincontainercallbac(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005801_rage_fwsearch_fwsearch_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005880_rage_fwassetstore_rage_pgdictionary_rage_crframefilter_rage_fwas(ph: *mut Phantom); }
extern "C" { pub fn f_005905_rage_fwassetstore_rage_pgdictionary_rage_crexpressions_rage_fwas(ph: *mut Phantom); }
extern "C" { pub fn f_005938_rage_aitaskstatetransitiontable_aitaskstatetransitiontable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006009_rage_fwpedgennavmeshiterator_fwpedgennavmeshiterator(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006015_rage_cpedgensortedlist_cpedgensortedlist(ph: *mut Phantom); }
extern "C" { pub fn f_006056_rage_strstreaminginfomanager_removeobject_rage_strindex_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006084_rage_strstreaminginfomanager_requestobject_rage_strindex_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006143_rage_strstreaminginfoiterator_strstreaminginfoiterator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006146_rage_strstreaminginfomanager_strstreaminginfomanager(ph: *mut Phantom); }
extern "C" { pub fn f_006164_rage_strstreaminginfo_clearflag_rage_strindex_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006167_rage_strstreaminginfomanager_isobjectinimage_rage_strindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006175_rage_strstreaminginfomanager_clearrequiredflag_rage_strindex_uns(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006225_rage_atvarstring_atvarstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_006238_rage_strstreamingloadermanager_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006246_rage_strstreamingmodule_getvirtualmemoryofloadedobj_rage_strloca(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_006247_rage_strstreamingmodule_getphysicalmemoryofloadedobj_rage_strloc(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_006249_rage_strstreamingmodule_resetplacestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006300_rage_tcboxtree_reset(ph: *mut Phantom); }
extern "C" { pub fn f_006361_rage_atarray_float_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006923_rage_audcontroller_audcontroller(ph: *mut Phantom); }
extern "C" { pub fn f_006938_rage_audcurve_audcurve(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006965_rage_audengine_audengine(ph: *mut Phantom); }
extern "C" { pub fn f_007003_rage_atrangearray_rage_audgameeffectsettings_5_atrangearray(ph: *mut Phantom); }
extern "C" { pub fn f_007016_rage_audmetadatamanager_audmetadatamanager(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007118_rage_audsmoother_audsmoother(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007126_rage_audsimplesmoother_audsimplesmoother(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007396_rage_audentity_audentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007419_rage_vfxlistitem_vfxlistitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007420_rage_vfxlist_vfxlist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007515_rage_fwassetstore_rage_ptxfxlist_rage_ptxfxlistdef_fwassetstore(ph: *mut Phantom); }
extern "C" { pub fn f_007707_rage_fwaudioentity_handleanimevent_rage_audanimevent_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007875_rage_netbandwidthmgr_netbandwidthmgr(ph: *mut Phantom); }
extern "C" { pub fn f_008387_rage_netscgamerdatamgr_netscgamerdatamgr(ph: *mut Phantom); }
extern "C" { pub fn f_008645_rage_autoiddescriptor_t_rage_netinodedataaccessor_autoiddescript(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_008683_rage_fwconfigmanager_getsizeofpool_unsigned_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008872_rage_fwfsm_fwfsm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008873_rage_fwfsm_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008875_rage_gameskeleton_gameskeleton(ph: *mut Phantom); }
extern "C" { pub fn f_008892_rage_fwassetstore_rage_fwmetadatafile_rage_fwassetdef_rage_fwmet(ph: *mut Phantom); }
extern "C" { pub fn f_008985_rage_autoiddescriptor_t_rage_netmessage_autoiddescriptor_t_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009038_rage_crproperty_crproperty(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009048_rage_crproperty_getattribute_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009178_rage_crtags_crtags(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009208_rage_crskeletondata_getnumanimatabledofs_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009210_rage_crskeletondata_convertboneindextoid_int_unsigned_short_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009214_rage_bonenameregistry_bonenameregistry(ph: *mut Phantom); }
extern "C" { pub fn f_009223_rage_crskeleton_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009224_rage_crskeleton_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009230_rage_crskeleton_inverseupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009314_pixend(ph: *mut Phantom); }
extern "C" { pub fn f_009333_rage_grmshadergroup_setvarcommon_rage_grmshadergroupvar_void_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_009415_rage_pftimebarmgr_pftimebarmgr(ph: *mut Phantom); }
extern "C" { pub fn f_009417_pixbegin_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009446_rage_pfdrawitembase_pfdrawitembase_char_const_ptr_rage_color32_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_009466_rage_pfdrawgroup_pfdrawgroup_char_const_ptr_rage_pfdrawgroup_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_009479_rage_getrageprofiledraw(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009561_rage_grcbegin_rage_grcdrawmode_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009562_rage_grcvertex_float_float_float_float_float_float_rage_color32(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: f32, a8: f32); }
extern "C" { pub fn f_009563_rage_grcend(ph: *mut Phantom); }
extern "C" { pub fn f_009608_rage_grcviewport_grcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009649_rage_grcworldidentity(ph: *mut Phantom); }
extern "C" { pub fn f_009674_rage_grcadapterd3d11output_getclosestmode_rage_grcdisplaywindow(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009678_rage_grcdevice_prepareforshutdown(ph: *mut Phantom); }
extern "C" { pub fn f_009679_rage_grcresourcecache_grcresourcecache(ph: *mut Phantom); }
extern "C" { pub fn f_009690_rage_grcdevice_changedevice_rage_grcdisplaywindow(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009733_rage_grcdrawline_rage_vector3_const_ref_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009738_rage_grcdrawfrustum_rage_matrix34_const_ref_float_float_float_ra(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_009754_rage_grclighting_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_009758_rage_grcdrawsphere_float_rage_vector3_const_ref_int_bool_bool(ph: *mut Phantom, a0: f32, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_009760_rage_grcdrawaxis_float_rage_mat34v_const_ref_bool(ph: *mut Phantom, a0: f32, a1: i64); }
extern "C" { pub fn f_009810_rage_grctexchangedata_grctexchangedata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009896_rage_grcgputimestamp_grcgputimestamp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009909_rage_grcadaptermanager_getadapter_unsigned_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_009925_rage_grcadapterd3d11output_getdesc_int_unsigned_int_rage_grcadap(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009930_rage_grcadapterd3d11_getoutput_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009941_rage_csmanagerbase_csmanagerbase(ph: *mut Phantom); }
extern "C" { pub fn f_009970_rage_grcresourcecache_unloadcache(ph: *mut Phantom); }
extern "C" { pub fn f_009973_rage_grcresourcecache_devicelost(ph: *mut Phantom); }
extern "C" { pub fn f_009974_rage_grcresourcecache_devicereset(ph: *mut Phantom); }
extern "C" { pub fn f_010016_rage_grcdevice_forcedevicereset(ph: *mut Phantom); }
extern "C" { pub fn f_010017_rage_grcdevice_devicelost(ph: *mut Phantom); }
extern "C" { pub fn f_010018_rage_grcdevice_devicerestored(ph: *mut Phantom); }
extern "C" { pub fn f_010020_rage_grcdevice_isready(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_010144_rage_grcbufferd3d11_grcbufferd3d11(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010148_rage_grcbufferd3d11resource_grcbufferd3d11resource(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010160_rage_grcdebugdraw_getdisplaydebugtext(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_010477_rage_iomouse_setcursorvisible_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010524_rage_iovirtualkeyboard_iovirtualkeyboard(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010719_rage_pntriangletessellationglobals_pntriangletessellationglobals(ph: *mut Phantom); }
extern "C" { pub fn f_011093_rage_cutfcascadeshadoweventargs_getposition_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011120_void_rage_atplaceptrcreator_void_ptr_rage_spropdetails_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012721_rage_snpeer_getpeerinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012808_rage_textchat_textchat(ph: *mut Phantom); }
extern "C" { pub fn f_012941_rage_autoiddescriptor_t_rage_snpartyevent_autoiddescriptor_t_uns(ph: *mut Phantom); }
extern "C" { pub fn f_012943_rage_autoiddescriptor_t_rage_snpartyevent_autoiddescriptor_t_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_012955_rage_autoiddescriptor_t_rage_snevent_autoiddescriptor_t_unsigned(ph: *mut Phantom); }
extern "C" { pub fn f_012957_rage_autoiddescriptor_t_rage_snevent_autoiddescriptor_t_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_013390_rage_netbandwidthrecorder_netbandwidthrecorder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013393_rage_netbandwidthrecorder_setsampleinterval_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013438_rage_netaddress_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013452_rage_netpeeraddress_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013461_rage_netconnectionmanager_delegate_delegate(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013470_rage_netconnectionmanager_netconnectionmanager(ph: *mut Phantom); }
extern "C" { pub fn f_013481_rage_serverinfo_clearaddr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013641_rage_rc4context_rc4context(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013668_rage_nethttprequest_nethttprequest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013765_rage_netkeyexchangesession_netkeyexchangesession(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013817_rage_netsocket_setinitialvalues(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013818_rage_netnatportmapper_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013839_rage_netnatporttimeoutchecker_netnatporttimeoutchecker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013840_rage_netnatporttimeoutchecker_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013879_rage_netrandom_netrandom(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013880_rage_netrandom_netrandom_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_013882_rage_netipv4address_netipv4address(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013887_rage_netsocketaddress_netsocketaddress(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013938_rage_netsocket_netsocket(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013959_rage_netpeeraddress_netpeeraddress(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014049_rage_nettimestep_nettimestep(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014051_rage_netretrytimer_netretrytimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014057_rage_nettimesync_nettimesync(ph: *mut Phantom); }
extern "C" { pub fn f_014088_rage_neticesession_neticesession(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014150_rage_netrequesthandler_netrequesthandler(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014160_rage_nettransactor_nettransactor(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014195_rage_autoiddescriptor_t_rage_netevent_autoiddescriptor_t_unsigne(ph: *mut Phantom); }
extern "C" { pub fn f_014197_rage_autoiddescriptor_t_rage_netevent_autoiddescriptor_t_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_014205_rage_autoiddescriptor_t_rage_netmessage_autoiddescriptor_t_unsig(ph: *mut Phantom); }
extern "C" { pub fn f_014206_rage_autoiddescriptor_t_rage_netnatevent_autoiddescriptor_t_unsi(ph: *mut Phantom); }
extern "C" { pub fn f_014208_rage_autoiddescriptor_t_rage_netnatevent_autoiddescriptor_t_unsi(ph: *mut Phantom); }
extern "C" { pub fn f_014225_rage_autoiddescriptor_t_rage_nethardwareevent_autoiddescriptor_t(ph: *mut Phantom); }
extern "C" { pub fn f_014227_rage_autoiddescriptor_t_rage_nethardwareevent_autoiddescriptor_t(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_014300_rage_rlachievementinfo_rlachievementinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014342_rage_rlfriendspage_rlfriendspage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014672_rage_rltelemetrypolicies_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014685_rage_rlmetric_rlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014692_rage_rlmetriclist_rlmetriclist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014789_rage_rlprofilestatsvalue_rlprofilestatsvalue(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014800_rage_rlprofilestatsreadresults_rlprofilestatsreadresults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014873_rage_rlrosclient_rlrosclient(ph: *mut Phantom); }
extern "C" { pub fn f_015028_rage_rlunlocksrosreadtask_rlunlocksrosreadtask(ph: *mut Phantom); }
extern "C" { pub fn f_015047_rage_rlcloudwatcher_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015128_rage_rlclandesc_rlclandesc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015517_rage_autoiddescriptor_t_rage_rlfriendevent_autoiddescriptor_t_un(ph: *mut Phantom); }
extern "C" { pub fn f_015519_rage_autoiddescriptor_t_rage_rlfriendevent_autoiddescriptor_t_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_015522_rage_autoiddescriptor_t_rage_rlpresenceevent_autoiddescriptor_t(ph: *mut Phantom); }
extern "C" { pub fn f_015524_rage_autoiddescriptor_t_rage_rlpresenceevent_autoiddescriptor_t(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_015572_rage_autoiddescriptor_t_rage_rltelemetryevent_autoiddescriptor_t(ph: *mut Phantom); }
extern "C" { pub fn f_015574_rage_autoiddescriptor_t_rage_rltelemetryevent_autoiddescriptor_t(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_015579_rage_autoiddescriptor_t_rage_rlpcevent_autoiddescriptor_t_unsign(ph: *mut Phantom); }
extern "C" { pub fn f_015581_rage_autoiddescriptor_t_rage_rlpcevent_autoiddescriptor_t_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_015582_rage_autoiddescriptor_t_rage_rlprofilestatsevent_autoiddescripto(ph: *mut Phantom); }
extern "C" { pub fn f_015584_rage_autoiddescriptor_t_rage_rlprofilestatsevent_autoiddescripto(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_015592_rage_autoiddescriptor_t_rage_rlrosevent_autoiddescriptor_t_unsig(ph: *mut Phantom); }
extern "C" { pub fn f_015594_rage_autoiddescriptor_t_rage_rlrosevent_autoiddescriptor_t_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_015617_rage_autoiddescriptor_t_rage_rlclanevent_autoiddescriptor_t_unsi(ph: *mut Phantom); }
extern "C" { pub fn f_015619_rage_autoiddescriptor_t_rage_rlclanevent_autoiddescriptor_t_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016130_rage_phshapetest_rage_phshapeprobe_phshapetest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016165_rage_phshapetest_rage_phshapecapsule_phshapetest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016172_rage_phshapetest_rage_phshapesweptsphere_phshapetest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017186_rage_crclip_composite_rage_crframe_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_017281_rage_crexpressions_initializecreature_rage_crcreature_ref_rage_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017760_void_rage_crmtiterator_iteratebatch_64_false_rage_crmtnode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017762_rage_crmtcomposerdata_refresh(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017766_rage_crmtmotiontree_request_rage_crmtrequest_ref_rage_crmtobserv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_017768_rage_crmtnode_addobserver_rage_crmtobserver_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017769_rage_crmtobserver_attach_rage_crmtnode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017770_rage_crmtobserver_detach(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017771_rage_crmtmotiontree_dump_int_char_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017786_rage_crmtobserver_crmtobserver(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017788_rage_crmtobserver_attach_rage_crmtobserver_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017850_rage_crmtrequest_dtorcrmtrequest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017854_rage_crmtmotiontreescheduler_schedule_rage_crmtmotiontree_ref_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64, a5: i64, a6: i64, a7: i32, a8: i64); }
extern "C" { pub fn f_017858_rage_crmtmotiontreescheduler_iscomplete_rage_crmtmotiontree_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_017859_rage_crmtmotiontreescheduler_waitoncomplete_rage_crmtmotiontree(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_017871_rage_crmtnode_nodetypeinfo_nodetypeinfo_rage_crmtnode_enodes_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_017883_rage_crmtnodeparent_promotechild_rage_crmtnode_ref_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018090_rage_crmtrequestblend_crmtrequestblend_rage_crmtrequest_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_018091_rage_crmtrequestblend_dtorcrmtrequestblend(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018095_rage_crmtrequestblend_setmergeblend_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018098_rage_crmtrequestframe_crmtrequestframe_rage_crframe_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018099_rage_crmtrequestframe_dtorcrmtrequestframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018103_rage_crmtrequestfilter_dtorcrmtrequestfilter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018109_rage_crmtrequestik_dtorcrmtrequestik(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018112_rage_crmtrequestclip_crmtrequestclip_rage_crclip_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018113_rage_crmtrequestclip_dtorcrmtrequestclip(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018116_rage_crmtrequestclip_setlooping_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_018118_rage_crmtrequestexpression_dtorcrmtrequestexpression(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018122_rage_crmtrequestcapture_crmtrequestcapture(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018123_rage_crmtrequestcapture_dtorcrmtrequestcapture(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018125_rage_crmtrequestcapture_setframe_rage_crframe_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018128_rage_crmtrequestpose_dtorcrmtrequestpose(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018131_rage_crmtrequestmerge_crmtrequestmerge(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018132_rage_crmtrequestmerge_dtorcrmtrequestmerge(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018154_rage_mvfixedheap_alloc_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_018155_rage_mvcommandbuffer_mvcommandbuffer_rage_mvcommandbuffer_manage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018156_rage_mvcommandbuffer_setmanager_rage_mvcommandbuffer_manager_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018157_rage_mvcommandbuffer_dtormvcommandbuffer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018158_rage_mvfixedheap_free_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018160_rage_mvcommandbuffer_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018169_rage_mvparameterbuffer_generatefreecapacity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018170_rage_mvcommandbuffer_manager_manager(ph: *mut Phantom); }
extern "C" { pub fn f_018173_rage_mvmotionweb_addref_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018174_rage_mvmotionweb_release_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018211_rage_mvparameterbuffer_mvparameterbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018212_rage_mvparameterbuffer_dtormvparameterbuffer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018214_rage_mvparameterbuffer_unfinalize(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018216_rage_mvparameterbuffer_iterator_begin_rage_mvparameterbuffer_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_018217_rage_mvparameterbuffer_iterator_next(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018496_gmatrix3d_setidentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018572_gwaitable_dtorgwaitable(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018573_gwaitable_handlerarray_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018575_gwaitable_handlerarray_callwaithandlers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018576_gtimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_019102_cpopgenshape_cpopgenshape(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019184_cnetrespawnmgr_tspawnrecord_tspawnrecord(ph: *mut Phantom); }
extern "C" { pub fn f_019288_caitarget_caitarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019551_cridableentrypointevaluationdebuginfo_cridableentrypointevaluati(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019741_cclipeventtags_cmotionblureventtag_motionblurdata_clear(ph: *mut Phantom); }
extern "C" { pub fn f_021037_naocclusionmanager_naocclusionmanager(ph: *mut Phantom); }
extern "C" { pub fn f_021446_rage_atrangearray_audpedscenario_3_atrangearray(ph: *mut Phantom); }
extern "C" { pub fn f_021768_audscriptaudioentity_audscriptaudioentity(ph: *mut Phantom); }
extern "C" { pub fn f_021967_audspeechmanager_audspeechmanager(ph: *mut Phantom); }
extern "C" { pub fn f_022192_audvehiclereflectionsentity_audvehiclereflectionsentity(ph: *mut Phantom); }
extern "C" { pub fn f_022246_audweatheraudioentity_audweatheraudioentity(ph: *mut Phantom); }
extern "C" { pub fn f_022366_audgunfightconductor_audgunfightconductor(ph: *mut Phantom); }
extern "C" { pub fn f_022367_audvehicleconductor_audvehicleconductor(ph: *mut Phantom); }
extern "C" { pub fn f_022373_rage_atrangearray_rage_atmultirangearray_rage_audcurve_4_0_0_0_0(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022386_naaudioentity_naaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022600_audcollisionaudioentity_audcollisionaudioentity(ph: *mut Phantom); }
extern "C" { pub fn f_022683_audcutsceneaudioentity_audcutsceneaudioentity(ph: *mut Phantom); }
extern "C" { pub fn f_022755_auddynamicmixer_auddynamicmixer(ph: *mut Phantom); }
extern "C" { pub fn f_022780_audemitteraudioentity_audemitteraudioentity(ph: *mut Phantom); }
extern "C" { pub fn f_022824_rage_atrangearray_deferredexplosionstruct_10_atrangearray(ph: *mut Phantom); }
extern "C" { pub fn f_022886_audglassaudioentity_audglassaudioentity(ph: *mut Phantom); }
extern "C" { pub fn f_022921_audhashactiontable_audhashactiontable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022991_audambientaudioentity_audambientaudioentity(ph: *mut Phantom); }
extern "C" { pub fn f_023172_naenvironment_naenvironment(ph: *mut Phantom); }
extern "C" { pub fn f_023226_namicrophones_namicrophones(ph: *mut Phantom); }
extern "C" { pub fn f_023396_cpacketevent_cpacketevent_ereplaypacketid_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023706_replayfileinfo_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023714_replayheadercache_replayheadercache(ph: *mut Phantom); }
extern "C" { pub fn f_023921_creplayeventmanager_creplayeventmanager(ph: *mut Phantom); }
extern "C" { pub fn f_027037_cdebugclipselector_cdebugclipselector_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_027070_camframe_camframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028254_cviewportfader_reset(ph: *mut Phantom); }
extern "C" { pub fn f_028255_cviewportmanager_cviewportmanager(ph: *mut Phantom); }
extern "C" { pub fn f_029773_rage_fwassetstore_rage_cutfcutscenefile2_cutfcutscenefile2def_fw(ph: *mut Phantom); }
extern "C" { pub fn f_029920_cdebugdrawstore_cdebugdrawstore(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029921_cdebugdrawstore_cdebugdrawstore_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_030305_cmapliveedit_cmapliveedit(ph: *mut Phantom); }
extern "C" { pub fn f_030415_cuicolorscheme_setallcolors_rage_color32_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030495_cassetref_getassetname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030504_cdebugtextureviewer_cdebugtextureviewer(ph: *mut Phantom); }
extern "C" { pub fn f_033108_gasnumberutil_nan(ph: *mut Phantom) -> f64; }
extern "C" { pub fn f_035244_cminimap_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035465_rage_atrangearray_rage_atrangearray_rage_pgref_rage_rmcdrawable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035725_clannetstatusarray_rage_rlclanmember_32_clannetstatusarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035893_cradiowheel_cradiowheel(ph: *mut Phantom); }
extern "C" { pub fn f_035898_cweaponwheel_cweaponwheel(ph: *mut Phantom); }
extern "C" { pub fn f_036135_rage_atrangearray_cstreammoviehelper_15_atrangearray(ph: *mut Phantom); }
extern "C" { pub fn f_036645_cuireticule_cuireticule(ph: *mut Phantom); }
extern "C" { pub fn f_037242_cuireplayscaleformcontroller_cuireplayscaleformcontroller(ph: *mut Phantom); }
extern "C" { pub fn f_037262_cscaleformmoviewrapper_cscaleformmoviewrapper_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_037345_rage_fwassetstore_cscaleformmovieobject_cscaleformdef_fwassetsto(ph: *mut Phantom); }
extern "C" { pub fn f_037790_cmodelindex_cmodelindex_rage_atnonfinalhashstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038160_rage_atrangearray_cdispatchadvancedspawnhelper_3_atrangearray(ph: *mut Phantom); }
extern "C" { pub fn f_040062_cpathserverthread_cpathserverthread(ph: *mut Phantom); }
extern "C" { pub fn f_040162_cpedgroupmembership_cpedgroupmembership(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040492_rage_nmbehaviorpool_nmbehaviorpool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041267_cinterestingevents_cinterestingevents(ph: *mut Phantom); }
extern "C" { pub fn f_041572_cpedpopulation_pedpopulationfailuredata_pedpopulationfailuredata(ph: *mut Phantom); }
extern "C" { pub fn f_041761_cpopgrouplist_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041780_cpopschedulelist_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041912_cplayerpedtargeting_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042409_cpedheadblenddata_cpedheadblenddata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043285_cflightmodelhelper_cflightmodelhelper_float_float_float_float_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: f32, a10: i64, a11: i64, a12: i64, a13: i64, a14: i64, a15: i64); }
extern "C" { pub fn f_043593_postfx_postfxparamblock_paramblock_paramblock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043898_crendersettings_reset(ph: *mut Phantom); }
extern "C" { pub fn f_043994_hdao_scenepreset_hdao_scenepreset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_044212_clowqualityscreenshot_clowqualityscreenshot(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044643_ccsmglobals_ccsmglobals(ph: *mut Phantom); }
extern "C" { pub fn f_045078_cclip_cclip(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045298_clistofugcphotosoncloud_clistofugcphotosoncloud(ph: *mut Phantom); }
extern "C" { pub fn f_045760_csavemigration_multiplayer_csavemigration_multiplayer(ph: *mut Phantom); }
extern "C" { pub fn f_046262_clightattr_clightattr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047788_clayoutmanager_clayoutmanager(ph: *mut Phantom); }
extern "C" { pub fn f_055064_network_commands_commandgettimeoffset_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_059433_rage_atfixedarray_rage_rlgamerhandle_32_atfixedarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063984_cdblbuf_cdoublebufferedintrorects_cdblbuf(ph: *mut Phantom); }
extern "C" { pub fn f_064136_rage_fwassetstore_rage_scrprogram_rage_fwassetnamedef_rage_scrpr(ph: *mut Phantom); }
extern "C" { pub fn f_064219_cgamescripthandlermgr_cgamescripthandlermgr_cnetworkbandwidthman(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064491_rage_atqueue_linkdatareport_256_atqueue(ph: *mut Phantom); }
extern "C" { pub fn f_064765_cpopulationstreaming_cpopulationstreaming(ph: *mut Phantom); }
extern "C" { pub fn f_065246_metricscapture_clearresults_debuglocationmetricslist_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065322_ccontrol_ccontrol(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065639_cpad_cpad(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066712_ccachedobjectcover_ccachedobjectcover(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067007_cplayercoverclipvariationhelper_cplayercoverclipvariationhelper(ph: *mut Phantom); }
extern "C" { pub fn f_067011_cdynamiccoverhelper_cdynamiccoverhelper(ph: *mut Phantom); }
extern "C" { pub fn f_068155_ctasklist_ctasklist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069788_ctaskclimbladderfully_sladderblock_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070539_ctasknmbalance_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070559_ctasknmbrace_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070645_ctasknmflinch_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070660_ctasknmhighfall_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070766_ctasknmshot_tunables_paramsets_paramsets(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072680_cscenarioeditor_cscenarioeditor(ph: *mut Phantom); }
extern "C" { pub fn f_075024_ctuningmanager_registertuning_ctuning_ref_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_075341_ctaskhelperfsm_ctaskhelperfsm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075350_ccliprequesthelper_ccliprequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075354_cgetupsetrequesthelper_cgetupsetrequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075445_casyncprobehelper_casyncprobehelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075455_ccliphelper_ccliphelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075470_cmovenetworkhelper_cmovenetworkhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075484_cgenericcliphelper_cgenericcliphelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075524_cvehiclecliprequesthelper_cvehiclecliprequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075568_cfindnearestcarnodehelper_cfindnearestcarnodehelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075576_ccoverpointstatushelper_ccoverpointstatushelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075664_ctuning_registertuning_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_077072_cstatsdatamgr_cstatsdatamgr(ph: *mut Phantom); }
extern "C" { pub fn f_077081_cstatsvehicleusage_clear_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077149_statid_char_statid_char_rage_atnonfinalhashstring_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_077329_statid_set_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077533_ctextlayout_ctextlayout(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077675_cperformancestats_tsectorsamples_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077835_cpathfind_cpathfind(ph: *mut Phantom); }
extern "C" { pub fn f_080469_cstoredvehiclevariations_cstoredvehiclevariations(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080472_cstoredvehicle_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081523_cmoviemeshmanager_cmoviemeshmanager(ph: *mut Phantom); }
extern "C" { pub fn f_081657_cptfxmanager_cptfxmanager(ph: *mut Phantom); }
extern "C" { pub fn f_081721_csky_csky(ph: *mut Phantom); }
extern "C" { pub fn f_081800_cvfxlightningmanager_cvfxlightningmanager(ph: *mut Phantom); }
extern "C" { pub fn f_081828_rage_vfxlightningstrikesettings_vfxlightningstrikesettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081963_cptfxgpudropsystem_cptfxgpudropsystem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082106_ccachedsmashables_ccachedsmashables(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082841_cnetworkscratchpads_cnetworkscratchpads(ph: *mut Phantom); }
extern "C" { pub fn f_083363_cloudcachelistener_cloudcachelistener(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083761_cnetworkassetverifier_cnetworkassetverifier(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083823_cnetworkghostcollisions_cghostcollision_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083897_networktextfilter_networktextfilter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084091_invitemgr_invitemgr(ph: *mut Phantom); }
extern "C" { pub fn f_084532_rage_atfixedarray_rage_rlprofilestatsfixedrecord_30u_16_atfixeda(ph: *mut Phantom); }
extern "C" { pub fn f_084534_rage_atfixedarray_rage_rlprofilestatsfixedrecord_1u_16_atfixedar(ph: *mut Phantom); }
extern "C" { pub fn f_084619_socialclubmgr_socialclubmgr(ph: *mut Phantom); }
extern "C" { pub fn f_086049_cnetworkvoicesession_cnetworkvoicesession(ph: *mut Phantom); }
extern "C" { pub fn f_086096_cnetworkleaderboardmgr_cnetworkleaderboardmgr(ph: *mut Phantom); }
extern "C" { pub fn f_086117_cnetworkvoice_cnetworkvoice(ph: *mut Phantom); }
extern "C" { pub fn f_088202_cnetworkplayermgr_cnetworkplayermgr_rage_netbandwidthmgr_ref(ph: *mut Phantom); }
extern "C" { pub fn f_088230_croamingbubblemgr_croamingbubblemgr(ph: *mut Phantom); }
extern "C" { pub fn f_088259_partyinvitemgr_partyinvitemgr(ph: *mut Phantom); }
extern "C" { pub fn f_088609_cpedblenderdata_cpedblenderdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090585_wc_unlockmutex(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090694_exit(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_090695_errno_location(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090709_aligned_alloc(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090710_asinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090717_atof(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090719_isspace(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090720_atol(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090721_bsearch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_090722_close(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090723_closedir(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090735_do_futex_wait(ph: *mut Phantom, a0: i64, a1: i32, a2: f64) -> i32; }
extern "C" { pub fn f_090737_emscripten_futex_wake(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_090739_emscripten_get_heap_size(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_090741_time(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090743_emscripten_yield(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090762_fclose(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090763_fcntl(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090764_fflush(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090767_fgetws(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090775_fopen(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090776_fiprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090777_small_fprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090782_fread(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_090784_fseeko(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090785_fstat(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_090788_ftello(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090790_fwrite(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_090791_getenv(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090792_bswap_32(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090794_bswap_16(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090795_isalnum(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090796_isprint(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090797_isxdigit(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090799_getpid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090800_emscripten_thread_is_valid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090801_localtime(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090805_log10f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090807_logf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090810_lseek(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_090812_mbstowcs(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090813_memchr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_090814_memcmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090816_localtime_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090817_gmtime_r(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090819_nanosleep(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090821_open(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090822_opendir(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090826_powf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090829_iprintf(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090860_pthread_attr_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090861_pthread_attr_setstacksize(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090862_pthread_cond_broadcast(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090864_pthread_cond_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090868_pthread_cond_timedwait(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090879_pthread_create(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090884_pthread_join(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090885_pthread_mutex_destroy(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090886_pthread_mutex_init(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090887_pthread_mutex_lock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090888_pthread_mutex_trylock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090889_pthread_mutex_unlock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090890_pthread_mutexattr_settype(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090898_pthread_setcancelstate(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_090901_qsort_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_090910_srand(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_090911_rand(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090912_read(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090913_readdir(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090916_select(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_090918_sinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090920_snprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090921_siprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090922_small_sprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090923_sscanf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090924_stat(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090926_strcasecmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090928_strchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090930_strcmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090931_strcpy(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090932_strcspn(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090933_strerror_l(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_090938_strftime(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_090939_strlen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090940_strncasecmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090941_strncat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090942_strncmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090945_strrchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090946_strspn(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090947_strstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090956_strtok(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090957_strtoull(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_090959_strtoll(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_090960_syscall_ret(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090961_sysconf(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_090962_system(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090965_tanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090968_emscripten_thread_mailbox_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090970_tolower(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090971_toupper(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090985_vsnprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090987_vsprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090990_wasi_syscall_ret(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090991_wcslen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090993_wcschr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090996_wcstombs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090999_wmemcpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_091000_emscripten_builtin_malloc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_091002_emscripten_builtin_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091003_emscripten_builtin_calloc(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_091006_dlmallinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091008_sbrk(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_091023_multi3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_091025_emscripten_stack_get_base(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_091026_emscripten_stack_get_end(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_091055_std_2_call_once_unsigned_long_volatile_ref_void_ptr_void_ptr_voi(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091056_std_2_hash_memory_void_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_091059_std_2_next_prime_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_091061_std_2_mutex_lock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091062_std_2_mutex_dtormutex(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091069_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_091070_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_091071_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_091073_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_091074_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_091075_std_2_libcpp_verbose_abort_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_091099_getsockname(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_091100_recv(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_091102_send(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_091104_setsockopt(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_091105_shutdown(ph: *mut Phantom, a0: i32, a1: i32); }
