//! Callee declarations for crate gen011.
//!
//! Every `call $name` in the wasm module becomes a call through this
//! module: an `extern "C"` declaration of the target's generated
//! identifier.  That keeps a function's callers independent of which
//! generated crate the callee ended up in (they are all linked into the
//! same binary and the identifiers are unique).
#![allow(improper_ctypes, dead_code, unused_imports)]
use wasm_rt::Phantom;

extern "C" { pub fn f_000094_cxx_global_array_dtor_28(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000100_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000101_op_new_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000102_op_delete_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000109_rage_syscriticalsectiontoken_syscriticalsectiontoken_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000111_rage_syscriticalsectiontoken_lock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000113_rage_syscriticalsectiontoken_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000114_rage_sysipccreatesema_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_000115_rage_sysipcpollsema_rage_sysipcsematag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000116_rage_sysipcwaitsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000118_rage_sysipcwaitsematimed_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_000119_rage_sysipcsignalsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000121_rage_sysipclockmutexcommand_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000122_rage_sysipcunlockmutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000128_rage_sysipccreatethread_void_ptr_void_ptr_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_000131_rage_sysipcwaitthreadexit_rage_sysipcthreadid_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000134_rage_sysipcyield_int(ph: *mut Phantom); }
extern "C" { pub fn f_000135_rage_sysipcsleep_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000136_rage_sysipcestimatestackusage_unsigned_int_ref_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000317_non_virtual_thunk_to_wasm_null_d3d_nullresource_wasm_null_d3d_st(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000644_rage_athashnextsize_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000645_rage_athash_const_char_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000654_rage_atstring_set_char_const_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_000656_rage_atstring_uppercase(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000657_rage_atstring_endswith_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000659_rage_atstring_indexof_char_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_000660_rage_atstring_indexof_char_const_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000662_rage_atstring_lastindexof_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000663_rage_atstring_replace_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000664_rage_atstring_split_rage_atstring_ref_rage_atstring_ref_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000668_rage_atarray_rage_atstring_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_000671_rage_atstring_split_rage_atarray_rage_atstring_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000675_rage_functorbase_functorbase_void_const_ptr_void_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_000680_rage_athashstringnamespacesupport_computehash_rage_athashstringn(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_000681_rage_athashstringnamespacesupport_addstring_rage_athashstringnam(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_000684_rage_athashstringnamespacesupport_trygetstring_rage_athashstring(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_000696_rage_bkwidget_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000703_rage_bkbank_unsetcurrentgroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000704_rage_bkbank_pushgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000705_rage_bkgroup_addgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000706_rage_bkbank_deletegroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000707_rage_bkbank_popgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000758_rage_bkmanager_openfile_char_ptr_int_char_const_ptr_bool_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_000779_rage_bkmanager_createbank_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000782_rage_bkmanager_findbank_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000783_rage_bkmanager_findwidget_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000785_rage_bkmanager_destroybank_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000786_rage_bkmanager_isbankthread(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000798_rage_bkwidget_setreadonly_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000806_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000828_rage_bklist_addcolumnheader_int_char_const_ptr_rage_bklist_itemt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_000829_rage_bklist_additem_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_000830_rage_bklist_additem_int_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_000831_rage_bklist_removeitem_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000843_rage_bkcombo_setstring_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_000844_rage_bkcombo_getstring_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_000851_char_ptr_rage_formatf_char_8ul_char_ref_8ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000866_rage_bkgroup_addseparator_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000867_rage_bkgroup_addtitle_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000868_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_000869_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000870_rage_bkgroup_addbutton_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000872_rage_bkgroup_addtext_char_const_ptr_float_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000874_rage_bkgroup_addtext_char_const_ptr_int_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000876_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_000877_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_000880_rage_bkgroup_addtext_char_const_ptr_rage_atnonfinalhashstring_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000882_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000885_rage_bkgroup_addtoggle_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_000886_rage_bkgroup_addtoggle_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000893_rage_bkgroup_addtoggle_char_const_ptr_float_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000894_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000898_rage_bkgroup_addslider_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000905_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000906_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000907_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int_rage_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000908_rage_bkgroup_addslider_char_const_ptr_rage_vector2_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64); }
extern "C" { pub fn f_000912_rage_bkgroup_addslider_char_const_ptr_rage_vector4_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000918_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000919_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000921_rage_bkgroup_addlist_char_const_ptr_bool_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000926_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000927_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000928_rage_bkgroup_addvector_char_const_ptr_rage_vector4_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000941_rage_bkgroup_addcolor_char_const_ptr_rage_color32_ptr_rage_datca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_000942_rage_bkgroup_addcolor_char_const_ptr_rage_color32_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000990_char_ptr_rage_formatf_char_512ul_char_ref_512ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001041_rage_matrix33_inverse_rage_matrix33_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001062_rage_matrix34_inverse_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001105_rage_autoidregistry_autoidregistry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001108_rage_autoidregistry_unregister_rage_autoiddescriptor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001116_char_ptr_rage_formatf_char_32ul_char_ref_32ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001117_rage_datcallback_datcallback(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001119_rage_datcallback_datcallback_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001120_rage_datcallback_datcallback_void_ptr_void_ptr_void_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_001123_rage_datgrowbuffer_datgrowbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001124_rage_datgrowbuffer_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001125_rage_datgrowbuffer_dtordatgrowbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001126_rage_datgrowbuffer_init_rage_sysmemallocator_ptr_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001129_rage_datgrowbuffer_getfihandle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001138_rage_datgrowbuffer_getbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001190_rage_aes_aes_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001193_rage_aes_encrypt_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001195_rage_aes_decrypt_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001197_rage_sha1_sha1(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001206_rage_rsonwriter_rsonwriter_char_ptr_unsigned_int_rage_rsonformat(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001207_rage_rsonwriter_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001210_rage_rsonwriter_begin_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001213_rage_rsonwriter_beginarray_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001214_rage_rsonwriter_end(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001216_rage_rsonwriter_writeint_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001218_rage_rsonwriter_writeint64_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001219_bool_rage_rsonwriter_write_long_char_const_ptr_char_const_ptr_lo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_001220_rage_rsonwriter_writeuns_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001221_rage_rsonwriter_writeuns64_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001222_rage_rsonwriter_writefloat_char_const_ptr_float_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_001224_rage_rsonwriter_writestring_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001229_rage_rsonwriter_writebool_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001232_rage_rsonreader_rsonreader(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001233_rage_rsonreader_rsonreader_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001234_rage_rsonreader_init_char_const_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001237_rage_rsonreader_hasmember_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001238_rage_rsonreader_getmember_char_const_ptr_rage_rsonreader_ptr_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001239_rage_rsonreader_getnextsibling_rage_rsonreader_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001240_rage_rsonreader_getfirstmember_rage_rsonreader_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001243_rage_rsonreader_getname_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001244_rage_rsonreader_getvalue_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001245_rage_rsonreader_getvalue_char_const_ptr_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_001246_rage_rsonreader_asint_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001247_rage_rsonreader_asint64_long_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001248_rage_rsonreader_asuns_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001249_rage_rsonreader_asuns64_unsigned_long_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001250_rage_rsonreader_asfloat_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001251_rage_rsonreader_asbool_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001253_rage_rsonreader_asstring_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001268_rage_rsonreader_getrawvaluestring_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001269_rage_rsonreader_validatejson_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001514_rage_diagdisplayf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001516_rage_fistream_write_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001523_quitf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001524_displayf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001527_warningf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001529_rage_fistream_create_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001530_rage_fistream_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001532_rage_fistream_close(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001533_rage_fidevice_getdevice_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001534_quitf_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001535_rage_diaglogf2_rage_diagchannel_const_ref_rage_diagseverity_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001537_rage_fistream_size(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001550_char_ptr_rage_formatf_char_12ul_char_ref_12ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001551_rage_fprintf_rage_fistream_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001559_rage_diagcontextmessage_diagcontextmessage_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001560_rage_diagcontextmessage_diagcontextmessage_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001563_rage_diagcontextmessage_pop(ph: *mut Phantom); }
extern "C" { pub fn f_001565_printf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001568_rage_atmapentry_unsigned_int_wchar_t_ptr_const_ref_rage_atmap_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001576_rage_fiassetmanager_fullpath_char_ptr_int_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_001577_rage_fistream_open_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001578_rage_fistream_seek_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001580_rage_fiassetmanager_open_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001582_rage_fiassetmanager_createleadingpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001622_rage_fistream_read_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001623_rage_fiassetmanager_findextensioninpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001626_rage_fiassetmanager_addextension_char_ptr_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001627_rage_fiassetmanager_exists_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001630_rage_atarray_rage_atbinarymap_rage_atstring_unsigned_int_datapai(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001631_rage_fiassetmanager_findpath_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001632_rage_fiassetmanager_pushfolder_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001633_rage_fiassetmanager_popfolder(ph: *mut Phantom); }
extern "C" { pub fn f_001634_rage_fiassetmanager_getattributes_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001635_rage_fiassetmanager_filename_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001636_rage_fiassetmanager_basename_char_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001639_rage_fiassetmanager_removeextensionfrompath_char_ptr_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001640_rage_fiassetmanager_removenamefrompath_char_ptr_int_char_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001643_rage_fistream_create_char_const_ptr_rage_fidevice_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001645_rage_fistream_putch_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001647_rage_fistream_tell(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001652_rage_fidevice_makememoryfilename_char_ptr_int_void_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_001737_rage_ficachepartition_init_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_001759_rage_fidevice_checkfilevalidity_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001764_rage_atarray_rage_atbinarymap_int_rage_atfinalhashstring_datapai(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001768_rage_fidevicerelative_init_char_const_ptr_bool_rage_fidevice_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_001769_rage_fidevicerelative_mountas_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001806_rage_fidevice_unmount_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001807_rage_fidevice_unmount_rage_fidevice_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001880_rage_fipackfile_fipackfile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001881_rage_fipackfile_init_char_const_ptr_bool_rage_fipackfile_cachemo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_001883_rage_fipackfile_mountas_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001885_rage_fipackfile_dtorfipackfile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001908_int_rage_formatf_n_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001950_rage_getsavefilepath_unsigned_int_char_ref_256_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_001953_rage_fisavegame_checkselectdevice_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_001954_rage_fisavegame_endselectdevice_int(ph: *mut Phantom); }
extern "C" { pub fn f_001955_rage_fisavegame_getselecteddevice_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_001958_rage_fisavegame_beginenumeration_int_unsigned_int_rage_fisavegam(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_001959_rage_fisavegame_checkenumeration_int_int_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_001960_rage_fisavegame_endenumeration_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_001962_rage_fisavegame_beginsave_int_unsigned_int_unsigned_short_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001963_rage_fisavegame_checksave_int_bool_ref_bool_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001964_rage_fisavegame_endsave_int(ph: *mut Phantom); }
extern "C" { pub fn f_001965_rage_fisavegame_checkfreespacecheck_int_int_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001966_rage_fisavegame_endfreespacecheck_int(ph: *mut Phantom); }
extern "C" { pub fn f_001967_rage_fisavegame_checkextraspacecheck_int_int_ref_int_ref_int_ref(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_001968_rage_fisavegame_endextraspacecheck_int(ph: *mut Phantom); }
extern "C" { pub fn f_001969_rage_fisavegame_checkreadnamesandsizesofallsavegames_int_int_ref(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_001970_rage_fisavegame_endreadnamesandsizesofallsavegames_int(ph: *mut Phantom); }
extern "C" { pub fn f_001971_rage_fisavegame_beginload_int_unsigned_int_unsigned_int_char_con(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001972_rage_fisavegame_checkload_int_bool_ref_unsigned_int_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001973_rage_fisavegame_endload_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_001974_rage_fisavegame_beginiconsave_int_unsigned_int_char_const_ptr_vo(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_001975_rage_fisavegame_checkiconsave_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_001976_rage_fisavegame_endiconsave_int(ph: *mut Phantom); }
extern "C" { pub fn f_001977_rage_fisavegame_checkcontentverify_int_bool_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001978_rage_fisavegame_endcontentverify_int(ph: *mut Phantom); }
extern "C" { pub fn f_001979_rage_fisavegame_checkdelete_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_001980_rage_fisavegame_enddelete_int(ph: *mut Phantom); }
extern "C" { pub fn f_001981_rage_fisavegame_begingetcreator_int_unsigned_int_unsigned_int_ch(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_001982_rage_fisavegame_checkgetcreator_int_bool_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001983_rage_fisavegame_endgetcreator_int(ph: *mut Phantom); }
extern "C" { pub fn f_001984_rage_fisavegame_getstate_int_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_001985_rage_fisavegame_geterror_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_001986_rage_fisavegame_setstatetoidle_int(ph: *mut Phantom); }
extern "C" { pub fn f_001987_rage_fisavegame_setselecteddevicetoany_int(ph: *mut Phantom); }
extern "C" { pub fn f_001988_rage_fisavegame_checkgetfilesize_int_unsigned_long_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001989_rage_fisavegame_endgetfilesize_int(ph: *mut Phantom); }
extern "C" { pub fn f_001990_rage_fisavegame_checkgetfilemodifiedtime_int_unsigned_int_ref_un(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_001991_rage_fisavegame_endgetfilemodifiedtime_int(ph: *mut Phantom); }
extern "C" { pub fn f_002018_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002019_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002020_void_std_2_sort_heap_abi_nqn220108_std_2_classicalgpolicy_bool_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002021_rage_atbinarymap_int_rage_atfinalhashstring_datapair_ptr_std_2_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002022_std_2_pair_rage_atbinarymap_int_rage_atfinalhashstring_datapair(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002023_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002027_rage_invsqrtf_fast_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_002032_rage_mthrandom_setfullseed_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002044_rage_pgrscbuilder_computeleafsize_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002077_rage_pgstreamer_drain(ph: *mut Phantom); }
extern "C" { pub fn f_002080_rage_pgbase_pgbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002083_rage_pgbase_validate_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002091_rage_datresource_containsthisaddress_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002092_rage_datresource_getfixup_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002111_rage_parmanager_parmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002112_rage_parmanager_initialize_rage_parsettings_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002114_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_002128_rage_parmanager_uninitialize(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002129_rage_atmapentry_rage_atnonfinalhashstring_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002131_rage_atnonfinalhashstring_atnonfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002132_rage_parmanager_dtorparmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002141_rage_partreenode_dtorpartreenode(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002145_rage_parmanager_loadtree_rage_fistream_ptr_rage_parsettings_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002146_rage_parmanager_findinputformat_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002147_rage_parmanager_loadtree_char_const_ptr_char_const_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002148_rage_parmanager_savetree_rage_fistream_ptr_rage_partree_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002156_rage_parmanager_initfromstructure_rage_parstructure_ref_rage_par(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002158_rage_parinstancevisitor_visittoplevelstructure_rage_parparsables(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002159_rage_parmanager_loadfromstructure_rage_partreenode_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_002161_rage_parstructure_issubclassof_rage_parstructure_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002165_rage_parmanager_createandloadanytype_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002169_rage_parmanager_buildtreenodefromstructure_rage_parstructure_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_002175_rage_partree_dtorpartree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002179_rage_partreenode_removeself(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002181_rage_partreenode_findfromxpathinternal_char_const_ptr_rage_parat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002184_rage_partreenode_findchildwithname_char_const_ptr_rage_partreeno(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002188_rage_partreenode_appendaschildof_rage_partreenode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002189_rage_partreenode_createstdleaf_char_const_ptr_double_bool(ph: *mut Phantom, a0: i64, a1: f64, a2: i32) -> i64; }
extern "C" { pub fn f_002193_rage_partreenode_createstdleaf_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002199_rage_partreenode_createstdleaf_char_const_ptr_char_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002226_rage_psotype_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002227_rage_psoschemacatalog_findstructureschema_rage_atnamespacedhashv(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002232_rage_psostruct_getmemberbyindex_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002233_rage_psomember_getsubstructure(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002234_rage_psostruct_getmemberbyname_rage_atnamespacedhashvalue_rage_a(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002271_rage_psoloadfromstructure_rage_psostruct_ref_rage_parstructure_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_002278_rage_psofile_dtorpsofile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002282_rage_psoloadfile_char_const_ptr_rage_psoloadaction_rage_atfixedb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002288_rage_psoloadfromstructure_rage_psofile_ref_rage_parstructure_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002294_rage_atarray_rage_psorscenumschemavaluedata_0_unsigned_short_gro(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002295_rage_atarray_rage_psorscbuilderinstanceset_ptr_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002296_rage_atmap_unsigned_int_rage_atarray_rage_psorscbuilderinstances(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002304_rage_psobuilder_findstructschema_rage_atnamespacedhashvalue_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002305_rage_psobuilderstructschema_addmemberrawmemidxcount_rage_atnames(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_002306_rage_psobuilderstructschema_addmemberrawrefhash_rage_atnamespace(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002307_rage_psobuilderstructschema_addmembersimple_rage_atnamespacedhas(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_002308_rage_psobuilderstructschema_addmemberpointer_rage_atnamespacedha(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002310_rage_psobuilderstructschema_finishbuilding(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002311_rage_atarray_rage_psoenumtableentry_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002312_rage_psobuilderschemacatalog_dtorpsobuilderschemacatalog(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002313_rage_psobuilderinstance_addfixup_int_char_const_ptr_rage_psobuil(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002315_rage_psobuilderinstancedatacatalog_dtorpsobuilderinstancedatacat(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002318_rage_psobuilder_psobuilder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002319_rage_psobuilder_createstructschema_rage_atnamespacedhashvalue_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002321_rage_psobuilder_createstructinstances_rage_psobuilderstructschem(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002323_rage_psobuilder_finishbuilding(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002324_rage_psobuilder_computesize(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002325_rage_psobuilder_getfilesize(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002326_rage_psobuilder_savetobuffer_char_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002332_rage_psoaddparsableobjectfromstructure_rage_psobuilder_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002343_rage_parrestregistersingleton_char_const_ptr_rage_parparsablestr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002373_rage_parstructure_adddelegateholder_char_const_ptr_rage_pardeleg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002374_rage_parrtstructure_parrtstructure_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002375_rage_parrtstructure_dtorparrtstructure(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002377_rage_parrtstructure_addstructure_char_const_ptr_rage_parstructur(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002378_rage_parrtstructure_addsimplemember_char_const_ptr_rage_parparsa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_002436_rage_parcguregisterstructure_rage_parstructure_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002437_rage_parcgustructure_createstructure_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002438_rage_parcgustructure_setfactories_rage_parstructure_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002439_rage_parcgustructure_setgetstructurecb_rage_parstructure_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002440_rage_parcgustructure_setbaseclass_rage_parstructure_ptr_rage_par(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002441_rage_parcgustructure_beginaddinginitialdelegates_rage_parstructu(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002442_rage_parcgustructure_adddelegateholder_rage_parstructure_ptr_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002443_rage_parcgustructure_endaddinginitialdelegates_rage_parstructure(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002444_rage_parcgustructure_buildstructurefromstaticdata_void_ptr_rage(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002445_rage_parcgustructure_addoneextraattribute_rage_parattributelist(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002446_rage_parcgustructure_addoneextraattribute_rage_parattributelist(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002447_rage_parinstancevisitor_parinstancevisitor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002448_int_rage_formatf_n_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002581_rage_parelement_setname_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002593_rage_parsettings_createstandardsettings(ph: *mut Phantom); }
extern "C" { pub fn f_002695_rage_atbinarymap_rage_pardelegateholderbase_ptr_unsigned_int_dat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002696_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002697_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_002721_rage_conststring_dtorconststring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002739_rage_atpartialstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002741_rage_atliteralstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002743_rage_atdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002744_rage_atdatahash_unsigned_int_const_ptr_unsigned_long_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002746_rage_stristr_char_ptr_char_const_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002757_rage_countutf8characters_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002759_rage_utf8towidechar_char_const_ptr_unsigned_short_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002763_rage_utf8removeinvalidsurrogates_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002777_inflateinit2(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002779_inflate(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002781_inflateend(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002782_lz4_compressbound(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_002783_lz4_compress(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002784_lz4_decompress_fast(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002790_rage_zlibinflater_zlibinflater(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002799_rage_sysappcontent_isjapanesebuild(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002806_rage_sysservice_adddelegate_rage_atdelegator1_void_rage_sysservi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002808_rage_atdelegator1_void_rage_sysserviceevent_ptr_removedelegate_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002812_rage_sysnamedpipe_sysnamedpipe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002839_rage_sysparam_get_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002840_rage_sysparam_get_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002843_rage_sysparam_getarray_float_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002852_rage_sysstack_printcapturedstacktrace_unsigned_long_const_ptr_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_002853_rage_sysstack_capturestacktrace_unsigned_long_ptr_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002863_rage_sysperformancetimer_sysperformancetimer_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002864_rage_sysperformancetimer_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002865_rage_sysperformancetimer_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002866_rage_sysperformancetimer_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002867_rage_sysperformancetimer_gettimems(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_002875_rage_atmap_unsigned_int_rage_sysstackcollector_tagentry_rage_atm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002877_rage_sysstackcollector_istagused_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002878_rage_atmap_unsigned_int_rage_sysstackcollector_tagentry_rage_atm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002879_rage_sysstackcollector_unregistertag_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002881_unsigned_int_ref_rage_atmap_unsigned_int_unsigned_int_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002882_rage_sysstackcollector_collectstack_unsigned_int_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_002884_rage_sysstackcollector_printinfofortag_unsigned_int_void_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_002886_rage_systhreadpool_thread_dtorthread(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002889_rage_systhreadpool_workitem_dtorworkitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002890_rage_systhreadpool_workitem_pending_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002898_rage_systhreadpool_dtorsysthreadpool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002899_rage_systhreadpool_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002901_rage_systhreadpool_init(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002902_rage_systhreadpool_addthread_rage_systhreadpool_thread_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_002904_rage_systhreadpool_queuework_rage_systhreadpool_workitem_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002916_rage_sysservice_updateclass(ph: *mut Phantom); }
extern "C" { pub fn f_002927_rage_sysmemstarttemp(ph: *mut Phantom); }
extern "C" { pub fn f_002928_rage_sysmemendtemp(ph: *mut Phantom); }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_002936_rage_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002937_rage_sysmemstartfragcache(ph: *mut Phantom); }
extern "C" { pub fn f_002938_rage_sysmemendfragcache(ph: *mut Phantom); }
extern "C" { pub fn f_002943_rage_sysmempoolallocator_dtorsysmempoolallocator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002944_rage_sysmempoolallocator_init_void_ptr_unsigned_long_unsigned_lo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002945_rage_sysmempoolallocator_safeallocate_unsigned_long_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002949_rage_sysmempoolallocator_forall_bool_ptr_void_ptr_void_ptr_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003097_char_ptr_rage_formatf_char_16ul_char_ref_16ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003111_rage_sysdependencyscheduler_insertinternal_rage_sysdependency_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003139_rage_geomdistances_distancelinetopoint_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_003159_rage_matrix34_fromquaternion_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003160_rage_matrix34_toquaternion_rage_quaternion_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003174_rage_setunitup_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003175_rage_vector3_vector3_rage_vector2_const_ref_rage_vector2_evector(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003176_rage_vector3_approachstraight_rage_vector3_const_ref_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_003179_rage_atpoolbase_init_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003182_rage_atpoolbase_releasemem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003185_rage_crframe_crframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003192_rage_crframe_set_rage_crframe_const_ref_rage_crframefilter_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003194_rage_vec_v4arccos_float_vector_4(ph: *mut Phantom, a0: u128) -> u128; }
extern "C" { pub fn f_003197_rage_crframe_zero_rage_crframefilter_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003205_rage_crframe_invalidate_rage_crframefilter_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003206_rage_crframe_pose_rage_crskeleton_ref_bool_rage_crframefilter_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003208_rage_crframe_inversepose_rage_crskeleton_const_ref_bool_rage_crf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003213_rage_crframe_setmatrix_unsigned_char_unsigned_char_unsigned_shor(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_003214_rage_crframe_getvector3_unsigned_char_unsigned_short_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_003215_rage_crframe_setvector3_unsigned_char_unsigned_short_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_003217_rage_crframe_getfloat_unsigned_char_unsigned_short_float_ref_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_003218_rage_crframe_setfloat_unsigned_char_unsigned_short_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32) -> i32; }
extern "C" { pub fn f_003221_rage_crframe_dtorcrframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003313_rage_crframedata_crframedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003314_rage_crframedata_dtorcrframedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003315_rage_crframedata_init_unsigned_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_003398_rage_crframedatainitializer_initializeframedata_rage_crframedata(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003407_rage_crframedatainitializerdofs_crframedatainitializerdofs_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_003557_rage_crcreature_dtorcrcreature(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003558_rage_crcreature_addcomponent_rage_crcreaturecomponent_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003559_rage_crcreature_refresh(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003560_rage_crcreature_allocatecomponent_rage_crcreaturecomponent_ecrea(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003564_rage_crcreature_removecomponenttype_rage_crcreaturecomponent_ecr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003598_rage_crcreaturecomponentskeleton_setskeleton_rage_crskeleton_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003620_rage_crcreaturecomponentextradofs_init_rage_crcreature_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_003738_rage_crcreaturecomponentphysical_resetmotions(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003801_rage_fwanimdirector_init_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_003806_rage_fwanimdirector_waitforanyactiveupdatetocomplete_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003808_rage_fwanimdirector_addexpression_rage_crexpressions_const_ref_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_003815_rage_fwanimdirector_preupdate_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003816_rage_fwanimdirector_updatemove_float_bool_rage_fwanimdirector_pt(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003820_rage_fwanimdirector_updatevelocityandangularvelocity_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_003821_rage_fwanimdirectorcomponentmove_swapoutputbuffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003822_rage_fwanimdirector_startupdatemidphysics_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_003823_rage_fwanimdirector_swapmoveoutputbuffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003824_rage_fwanimdirector_startupdatepostcamera_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_003825_rage_fwanimdirector_startupdateprerender_float_rage_crmtmotiontr(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32); }
extern "C" { pub fn f_003826_rage_fwanimdirector_setposed_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003831_rage_fwanimdirectorcomponentsyncedscene_isplayingsyncedscene_con(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003832_rage_fwanimdirectorcomponentsyncedscene_isvalidsceneid_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003835_rage_fwanimdirector_getmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003838_rage_fwclipset_getclip_rage_fwmvtemplateid_rage_fwmvclipidtype_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_003840_rage_fwclipsetmanager_getclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003845_rage_fwdictionaryloader_rage_mvnetworkdef_rage_fwnetworkdefstore(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003850_rage_fwanimdirector_getcreature_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003855_rage_fwanimdirector_getframebuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003856_rage_fwanimdirector_waitforprephysicsupdatetocomplete(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003857_rage_fwanimdirector_waitforprerenderupdatetocomplete(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003858_rage_fwanimdirector_waitforallupdatestocomplete(ph: *mut Phantom); }
extern "C" { pub fn f_003860_rage_fwanimdirectorcomponentragdoll_posefromragdoll_rage_crcreat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003873_rage_fwanimdirector_queue_pop(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_003874_rage_fwanimdirector_queue_clear(ph: *mut Phantom); }
extern "C" { pub fn f_003895_rage_fwanimmanager_getclipifexistsbyname_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003896_rage_fwanimmanager_getclipifexistsbydictindex_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_003904_rage_fwanimmanager_findslot_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003905_rage_fwanimmanager_findslotfromhashkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003908_rage_fwanimmanager_removerefwithoutdelete_rage_strlocalindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_003911_rage_fwanimmanager_addref_rage_strlocalindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_003941_void_rage_atplaceptrcreator_void_ptr_rage_fwmemorygroupmetadata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003968_rage_fwclipset_getclipdictionaryindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003974_rage_atarray_rage_atbinarymap_rage_fwclipitem_ptr_rage_fwmvtempl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003977_rage_fwclipset_request_deprecated_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_003984_rage_fwclipsetrequesthelper_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004000_rage_fwclipsetmanager_rebuildclipsetnames(ph: *mut Phantom); }
extern "C" { pub fn f_004002_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_004003_rage_atarray_rage_atbinarymap_rage_fwclipdictionarymetadata_rage(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004004_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_004005_rage_atarray_rage_atbinarymap_rage_fwmemorygroupmetadata_rage_at(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004006_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_004007_rage_fwclipsetmanager_buildclipdictionarynamearray(ph: *mut Phantom); }
extern "C" { pub fn f_004013_void_rage_appendbinmapfreeobjectptr_rage_fwclipset_rage_fwclipse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004014_rage_atstring_ref_rage_atmap_int_rage_atstring_rage_atmaphashfn(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004015_rage_atbinarymap_rage_fwclipset_ptr_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004016_rage_atarray_rage_atbinarymap_rage_fwmemorysituation_rage_atnonf(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004017_rage_atarray_rage_fwmemorysituation_adjustment_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004018_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_004019_rage_fwclipsetrequesthelper_request_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_004037_rage_fwclipsetrequesthelper_dtorfwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004038_rage_fwclipsetrequesthelper_fwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004039_rage_fwclipsetrequesthelper_request_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004041_rage_fwclipsetrequesthelper_isloaded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004042_rage_fwclipsetrequesthelper_isinvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004059_void_rage_appendbinmapfreeobjectptr_rage_fwexpressionset_rage_fw(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004060_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_004061_rage_fwexpressionsetmanager_getexpressionsetid_rage_atnonfinalha(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004062_rage_fwexpressionsetmanager_getexpressionset_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004081_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_004082_rage_fwfacialclipsetgroupmanager_getfacialclipsetgroup_rage_fwmv(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004105_rage_fwmove_fwmove_rage_fwentity_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004280_rage_fwposematcher_fwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004377_rage_fwsyncedsceneaudiointerface_dtorfwsyncedsceneaudiointerface(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004388_rage_fwanimdirectorcomponentsyncedscene_updatesyncedscenemover(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004403_rage_fwanimdirectorcomponentsyncedscene_getsyncedsceneorigin_sho(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_004414_rage_fwanimdirectorcomponentsyncedscene_getsyncedsceneattachenti(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004415_rage_fwanimdirectorcomponentsyncedscene_updatesyncedscenes_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_004418_rage_fwanimdirectorcomponentsyncedscene_preparesynchronizedscene(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004419_rage_fwanimdirectorcomponentsyncedscene_preparesynchronizedscene(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_004420_rage_fwanimdirectorcomponentsyncedscene_playsynchronizedsceneaud(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004421_rage_fwanimdirectorcomponentsyncedscene_stopsynchronizedsceneaud(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004449_rage_fwanimdirectorcomponentfacialrig_setvoicedrivenmouthmovemen(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_004576_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_004577_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004578_void_std_2_sort_heap_abi_nqn220108_std_2_classicalgpolicy_bool_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004579_rage_atbinarymap_rage_fwclipitem_ptr_rage_fwmvtemplateid_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_004580_std_2_pair_rage_atbinarymap_rage_fwclipitem_ptr_rage_fwmvtemplat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004581_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_004585_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_004586_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004587_void_std_2_sort_heap_abi_nqn220108_std_2_classicalgpolicy_bool_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004588_rage_atbinarymap_rage_fwmemorygroupmetadata_rage_atnonfinalhashs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_004589_std_2_pair_rage_atbinarymap_rage_fwmemorygroupmetadata_rage_atno(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004590_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_004603_rage_fwextensionlist_add_rage_fwextension_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004604_rage_fwaltskeletonextension_removeextension_rage_fwentity_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004609_rage_fwarchetypemanager_registerstreamedarchetype_rage_fwarchety(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004610_rage_fwarchetypemanager_registerpermanentarchetype_rage_fwarchet(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_004613_rage_fwarchetype_setdrawdictfile_rage_atnonfinalhashstring_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_004614_rage_fwarchetype_setdrawableorfragmentfile_rage_atnonfinalhashst(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_004617_rage_fwarchetype_findtxdslotindex_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004619_rage_fwarchetype_getdrawable_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004620_rage_fwarchetype_getfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004621_rage_fwarchetype_getassetparenttxdindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004623_rage_fwarchetype_createdynamiccomponentifmissing(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004625_rage_fwarchetype_setexpressiondictionaryindex_rage_atnonfinalhas(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004626_rage_fwarchetype_setexpressionhash_rage_atnonfinalhashstring_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004631_rage_fwarchetypemanager_getarchetypename_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004632_rage_fwarchetypemanager_lookupmodelid_rage_fwarchetype_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004633_rage_fwarchetypemanager_getarchetype_rage_atnonfinalhashstring_u(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004634_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004635_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_rag(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004639_rage_fwarchetypemanager_forallarchetypesinfile_int_void_ptr_rage(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_004648_rage_fwattachmententityextension_isattachstatebasicderived_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004650_rage_fwattachmententityextension_clearparentattachmentvars_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004652_rage_fwattachmententityextension_getoffsetfromparent_rage_fwenti(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_004653_rage_fwattachmententityextension_getrequirespostprerender_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004654_rage_fwattachmententityextension_attachtoentity_rage_fwentity_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_004655_rage_fwattachmententityextension_addconstrainthandle_rage_phcons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004657_rage_fwattachmententityextension_findconstraint_rage_phinst_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_004658_rage_fwattachmententityextension_debugsetinvocationdata_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_004660_rage_fwcamerarelativeextension_removeextension_rage_fwentity_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004661_rage_fwdynamicentitycomponent_fwdynamicentitycomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004662_rage_fwdynamicentitycomponent_deleteanimdirector(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004663_rage_fwdynamicentitycomponent_createattachmentextensionifmissing(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004665_rage_fwentity_fwentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004666_rage_fwentity_dtorfwentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004670_rage_fwentity_onactivate_rage_phinst_ptr_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004671_rage_fwentity_ondeactivate_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004673_rage_fwentity_initentityfromdefinition_rage_fwentitydef_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_004674_rage_fwentity_setcurrentphysicsinstinternal_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004675_rage_fwentity_setcurrentphysicsinst_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004676_rage_fwentity_clearcurrentphysicsinst(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004677_rage_fwentity_deleteskeleton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004679_rage_fwentity_getskeletonsize_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004680_rage_fwentity_printskeletonsummary_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004681_rage_fwentity_waitforanyactiveanimupdatetocomplete_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004683_rage_fwentity_getaabb_rage_spdaabb_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004684_rage_fwentity_isentityaparentattachment_rage_fwentity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_004685_rage_fwentity_getrootlod(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004688_rage_fwextensionlist_destroy_rage_fwextension_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004689_rage_fwextensionlist_destroy_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004692_rage_fwsceneupdate_registersceneupdate_unsigned_int_void_ptr_rag(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_004693_rage_fwsceneupdate_isinsceneupdate_rage_fwentity_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004694_rage_fwsceneupdate_addtosceneupdate_rage_fwentity_ref_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004695_rage_fwsceneupdate_removefromsceneupdate_rage_fwentity_ref_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_004696_rage_fwsceneupdate_update_unsigned_int_void_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_004697_rage_fwsceneupdate_runcustomcallbackbits_unsigned_int_void_ptr_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_004698_rage_fwsceneupdate_updateentitytype_unsigned_int_unsigned_int_vo(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_004708_rage_fwsimpletransform_op_new_unsigned_long(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004709_rage_fwtransform_delete(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004717_rage_fwdrawdata_dtorfwdrawdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004719_rage_fwdrawdata_removedrawable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004725_rage_fwwaypointrecordingroute_uint8tombr_unsigned_char(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004729_rage_fwwaypointrecordingroute_uint8tocruisespeed_unsigned_char(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004737_rage_fwdecorator_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004740_rage_fwdecoratorextension_existson_rage_fwextensiblebase_ref_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_004748_rage_fwdecoratorextension_find_rage_atnonfinalhashstring_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004767_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004771_rage_fwpickermanager_onwidgetchange(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004774_rage_fwpickermanager_setenabled_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004777_rage_fwpickermanager_setpickermanagersettings_rage_fwpickermanag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004779_rage_fwpickermanager_setpickersettings_int_rage_fwbasepickersett(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_004780_rage_fwpickermanager_iscurrentowner_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004781_rage_fwpickermanager_takecontrolofpickerwidget_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004783_rage_fwpickermanager_addentitytolist_rage_fwentity_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_004784_rage_fwpickermanager_addentitytopickerresults_rage_fwentity_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004787_rage_fwpickermanager_resetlist_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004788_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004789_rage_fwpickermanager_setindexofselectedentity_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004790_rage_fwpickermanager_getindexofentitywithinresultsarray_rage_fwe(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004792_rage_fwpickermanager_getselectedentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004793_rage_fwpickermanager_getentity_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004794_rage_fwpickermanager_getnumberofentities(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_004795_rage_fwpickermanager_getshowhidemode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_004825_rage_fwvectormap_draw3dboundingboxonvmap_rage_spdaabb_const_ref(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004934_rage_dldrawcommandbuffer_adddatablock_void_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_004935_rage_dlcmdbase_op_new_unsigned_long_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004963_rage_dlcmdpushtimebar_dlcmdpushtimebar_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_004964_rage_dlcmdpoptimebar_dlcmdpoptimebar(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005010_rage_dldrawlistmgr_charclothremove_rage_clothvariationdata_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005053_rage_fwtextutil_getbytecount_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005055_rage_fwtextutil_strcpywithcharacterandbytelimits_char_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_005056_rage_fwtextutil_makeuppercaseasciionly_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005058_rage_fwtextutil_copychar16stringintoasciibuffer_char_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_005059_rage_fwtextutil_copyasciistringintochar16buffer_unsigned_short_p(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005103_rage_clothmanager_preupdate_rage_grcviewport_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005109_rage_clothmanager_removecharactercloth_rage_clothcontroller_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005204_rage_ropeinstance_copyconstraintparams_rage_phconstraintdistance(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005205_rage_ropeinstance_swapconstraint_rage_phconstraintdistance_param(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005213_rage_ropemanager_remove_rage_ropeinstance_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005214_rage_ropemanager_processshutdownlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005215_rage_ropemanager_updatephysics_float_rage_enropeupdateorder(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_005216_rage_ropemanager_updatecloth_rage_enropeupdateorder(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005221_rage_ropemanager_findrope_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_005303_rage_fwrenderphasemanager_showscanresults_bool_const(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005315_rage_fwrenderthreadinterface_flush_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005320_rage_fwassetlocation_set_rage_estoretype_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005321_rage_fwassetlocation_getparentasset_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005322_rage_fwassetlocation_getstreamingmodule_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005323_rage_fwassetlocation_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005328_rage_fwdwdstore_getishdcapable_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005329_rage_fwassetlocation_getisboundhd_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005330_rage_fwassetlocation_validate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005331_rage_fwassetlocation_setisboundhd_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005332_rage_fwtxdstore_setisboundhd_rage_strlocalindex_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_005333_rage_fwassetlocation_gettxdlist_rage_fwptrlistsinglelink_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005335_rage_fwentitydef_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_005342_rage_fwmapdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_005348_rage_fwentitydef_dtorfwentitydef(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005350_rage_fwmapdata_dtorfwmapdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005355_rage_fwmapdata_constructentity_rage_fwentitydef_ptr_int_rage_fwa(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_005356_rage_fwmaptypesstore_getname_rage_strlocalindex_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005362_rage_fwboxstreamer_initwidgets_rage_bkbank_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_005387_rage_fwmapdatastore_removegroup_rage_strlocalindex_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_005388_rage_fwimapgroupmgr_createnewswap_rage_atnonfinalhashstring_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_005390_rage_fwimapgroupswap_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005392_rage_fwmapdatadebug_getblockindex_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005396_rage_fwstaticboundsstore_setenabled_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_005398_rage_fwimapgroupmgr_completeswap_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005399_rage_fwimapgroupmgr_abandonswap_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005400_rage_fwmapdatastore_isscriptmanaged_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005401_rage_fwmapdatadef_removemapchange(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005421_rage_fwpropinstancelistdef_instancedata_getmatrixcopy_rage_mat34(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005422_rage_fwpropinstancelistdef_releasedeviceresources_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005423_rage_fwgrassinstancelistdef_instancedata_computenormal_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005425_rage_fwgrassinstancelistdef_releasedeviceresources_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005430_rage_fwgrassinstancelistdef_postpsoplace_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005432_rage_fwmapdatacontents_dtorfwmapdatacontents(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005433_rage_fwsoaentitycontainer_fwsoaentitycontainer_rage_fwscenegraph(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_005434_rage_fwentitycontainer_appendentity_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_005435_rage_fwmapdatacontents_getguid_rage_fwentity_ptr_unsigned_int_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_005436_rage_fwmaptypescontents_dtorfwmaptypescontents(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005437_rage_fwextensiondef_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_005438_rage_fwarchetypedef_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_005441_rage_fwmaptypes_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_005446_rage_fwarchetypedef_dtorfwarchetypedef(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005448_rage_fwmaptypes_fwmaptypes(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005449_rage_fwmaptypes_dtorfwmaptypes(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005460_rage_fwboxstreamer_requestaboutpos_rage_vec3v_const_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_005461_rage_fwboxstreamer_hasloadedaboutpos_rage_vec3v_const_ref_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_005486_rage_fwclipdictionarystore_addref_rage_strlocalindex_rage_strref(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005488_rage_fwclipdictionarystore_removeref_rage_strlocalindex_rage_str(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005502_rage_fwassetstore_rage_pgdictionary_rage_charactercloth_rage_fwc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005538_rage_crframefilterdictionarystore_findframefilter_rage_fwmvtempl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005555_rage_fwimapgroupmgr_getisloaded_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005556_rage_fwimapgroupmgr_getisswapimaploaded_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005558_rage_fwimapgroupmgr_setstart_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005560_rage_fwimapgroupmgr_setend_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005561_sprintf_s_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005562_rage_fwassetstore_rage_pgdictionary_rage_grctexture_rage_fwtxdde(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005563_rage_fwtxdstore_set_rage_strlocalindex_rage_pgdictionary_rage_gr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_005566_rage_fwassetstore_rage_pgdictionary_rage_grctexture_rage_fwtxdde(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005577_rage_fwmapdatastore_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005580_rage_fwmapdatastore_saferemoveall_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005589_rage_fwpsostoreloader_unload_rage_fwpsostoreloadinstance_ref_boo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005601_rage_fwmapdatastore_removeunrequired_bool(ph: *mut Phantom); }
extern "C" { pub fn f_005602_rage_fwmapdatastore_savescriptimapstates(ph: *mut Phantom); }
extern "C" { pub fn f_005603_rage_fwmapdatastore_restorescriptimapstates(ph: *mut Phantom); }
extern "C" { pub fn f_005607_rage_fwmapdatastore_fixupallentriesafterregistration_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005609_rage_fwmapdatastore_createchildrencache(ph: *mut Phantom); }
extern "C" { pub fn f_005610_rage_fwmapdatastore_getparentslot_rage_strlocalindex_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005619_rage_fwmaptypesstore_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005623_rage_fwmaptypesstore_removeunrequired(ph: *mut Phantom); }
extern "C" { pub fn f_005633_rage_fwmaptypesstore_verifyitypfiles(ph: *mut Phantom); }
extern "C" { pub fn f_005648_rage_fwpsostoreloader_load_char_const_ptr_rage_fwpsostoreloadins(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_005650_rage_fwpsostoreloader_loaddataintoobjectcore_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005651_rage_fwpsostoreloader_makesimplefileloadercore(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005668_rage_fwstaticboundsstore_addslot_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005676_rage_fwstaticboundsstore_setasdummybound_rage_strlocalindex_shor(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_005678_rage_fwstaticboundsstore_getdummybounddata_rage_strlocalindex_sh(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_005679_rage_fwstaticboundsstore_removeall(ph: *mut Phantom); }
extern "C" { pub fn f_005681_rage_fwstaticboundsstore_getslotdata_int_rage_atsnode_rage_fwbou(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_005683_rage_fwstaticboundsstore_storeimapgroupboundslist_rage_atnonfina(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005684_rage_fwstaticboundsstore_postregistration(ph: *mut Phantom); }
extern "C" { pub fn f_005686_rage_fwstaticboundsstore_initwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005700_rage_fwtxdstore_remove_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005701_rage_fwtxdstore_loadfile_rage_strlocalindex_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_005706_rage_fwtxdstore_getsafefromindex_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005708_rage_fwtxdstore_adddummyslot_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005714_rage_fwtxdstore_removeref_rage_strlocalindex_rage_strrefkind(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005715_rage_fwentitycontainer_removeallentities(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005716_rage_fwbaseentitycontainer_updateentitydescriptorinnode_rage_fwe(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005718_rage_fwbaseentitycontainer_getentitydescindex_rage_fwentity_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005719_rage_fwbaseentitycontainer_getapproxboundingbox_unsigned_int_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_005720_rage_fwbaseentitycontainer_setloddistcached_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005721_rage_fwbaseentitycontainer_removeentity_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005724_rage_fwentitycontainer_getentityarray_rage_atarray_rage_fwentity(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005725_rage_fwfixedentitycontainer_fwfixedentitycontainer_rage_fwsceneg(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_005728_rage_fwexteriorscenegraphnode_removefromstraddlingportalcontaine(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005730_rage_fwinteriorlocation_makeinvalid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005731_rage_fwinteriorlocation_setroomindex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005732_rage_fwinteriorlocation_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005733_rage_fwinteriorlocation_getroomindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005734_rage_fwinteriorlocation_setportalindex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005735_rage_fwscenegraph_initpools(ph: *mut Phantom); }
extern "C" { pub fn f_005736_rage_fwscenegraphnode_addchild_rage_fwscenegraphnode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005737_rage_fwscenegraphnode_removechild_rage_fwscenegraphnode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005738_rage_fwscenegraphvisitor_fwscenegraphvisitor_rage_fwscenegraphno(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_005739_rage_fwscenegraphvisitor_next(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005740_rage_fwworldmgr_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_005742_rage_fwworldmgr_addtoworld_rage_fwentity_ptr_rage_fwrect_const_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005743_rage_fwworldmgr_removefromworld_rage_fwentity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005744_rage_fwworldmgr_forallentitiesintersecting_rage_fwsearchvolume_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_005745_rage_fwworldmgr_prescan(ph: *mut Phantom); }
extern "C" { pub fn f_005760_rage_fwscan_addcullshape(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_005778_rage_fwscanentities_calculateprojectocclusionobb_rage_fwscancasc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32); }
extern "C" { pub fn f_005780_rage_rstcalculatescreenspaceboundisvis_rage_rstsplatmat44v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_005798_rage_fwsearch_start_rage_fwscenegraphnode_ptr_rage_fwsearchvolum(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_005804_rage_fwsearchvolume_searchvolumesintersect_rage_spdsphere_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005917_rage_fwrefawarebaseimpl_rage_datbase_dtorfwrefawarebaseimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005928_rage_aitaskmanager_aitaskmanager_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_005929_rage_aitaskmanager_dtoraitaskmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005931_rage_aitaskmanager_process_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_005932_rage_aitaskmanager_aborttasksattreeindex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005933_rage_aitasktree_aborttaskswithpriority_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005934_rage_aitasktree_aborttasks(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005939_rage_aitaskstatetransitiontable_addtransition_int_int_float_long(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i64); }
extern "C" { pub fn f_005940_rage_aitasktree_aitasktree_rage_fwentity_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_005946_rage_aitasktree_cleartask_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005947_rage_aitasktree_getactiveleaftask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005949_rage_aitasktree_findtaskbytypeactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005950_rage_aitasktree_findtaskbytypewithpriority_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_005955_void_rage_datresource_place_rage_aisplitarray_rage_cnavmeshcompr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005956_void_rage_datresource_place_rage_aisplitarray_unsigned_short_819(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005957_void_rage_datresource_place_rage_aisplitarray_rage_tadjpoly_2048(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005958_void_rage_datresource_place_rage_aisplitarray_rage_tnavmeshpoly(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005959_rage_cnavmeshquadtree_cnavmeshquadtree_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_005966_rage_cnavmesh_getpolynormal_rage_vector3_ref_rage_tnavmeshpoly_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_006017_rage_cpathsearchpriorityqueue_cpathsearchpriorityqueue_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006018_rage_cpathsearchpriorityqueue_dtorcpathsearchpriorityqueue(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006031_rage_strcacheloader_init_char_const_ptr_char_const_ptr_rage_strc(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_006033_rage_strcacheloader_load(ph: *mut Phantom); }
extern "C" { pub fn f_006035_rage_atmapentry_unsigned_int_bool_const_ref_rage_atmap_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006037_rage_strstreamingloadermanager_callkeepalivecallbackifnecessary(ph: *mut Phantom); }
extern "C" { pub fn f_006038_rage_strcacheloader_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_006039_rage_strcacheloader_save(ph: *mut Phantom); }
extern "C" { pub fn f_006041_rage_strcacheloader_cmoduledescarray_addmodule_char_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_006042_rage_strcacheloader_writedatatobuffer_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006049_rage_strpackfilemanager_addimagetolist_char_const_ptr_bool_int_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_006052_rage_strpackfilemanager_unloadrpf_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006054_rage_strpackfilemanager_closearchive_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006055_rage_strpackfilemanager_removestreamablereference_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006056_rage_strstreaminginfomanager_removeobject_rage_strindex_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006061_rage_strpackfilemanager_findimage_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006064_rage_strstreamingvisualize_send_int_void_const_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_006066_rage_strpackfilemanager_getimagefilefromhandle_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006067_rage_strpackfilemanager_getimagefileindexfromhandle_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006069_rage_strpackfilemanager_getstreamingmodule(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_006070_rage_strpackfilemanager_setextracontentindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006072_rage_strstreaminginfomanager_getobjectname_rage_strindex_char_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_006074_rage_strstreaminginfomanager_removeobjectanddependents_rage_stri(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006075_rage_strstreaminginfo_computevirtualsize_rage_strindex_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006076_rage_strstreaminginfo_computephysicalsize_rage_strindex_bool_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006080_rage_strstreaminginfo_setflag_rage_strindex_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006081_rage_strpackfilemanager_loadimageifunloaded_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006083_rage_strpackfilemanager_addstreamablereference_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006084_rage_strstreaminginfomanager_requestobject_rage_strindex_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006085_rage_strpackfilemanager_registerindividualfile_char_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_006087_rage_strpackfilemanager_unregisterdevice_rage_fidevice_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006088_rage_atarray_rage_atnonfinalhashstring_0_unsigned_short_grow_uns(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006090_rage_strstreamingvisualize_unrequest_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006099_rage_strstreamingvisualize_request_rage_strindex_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006101_rage_strstreaminginfomanager_setrequiredflag_rage_strindex_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006120_rage_strresourcereference_resolve_bool_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006121_rage_strstreamingmodulemgr_getmodulefromfileextension_char_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006122_rage_strstreaminginfomanager_getmodule_rage_strindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006123_rage_strstreamingloadermanager_callkeepalivecallback(ph: *mut Phantom); }
extern "C" { pub fn f_006124_rage_strstreamingengine_getobjectname_rage_strindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006149_rage_strstreaminginfomanager_destroydependentgraphs(ph: *mut Phantom); }
extern "C" { pub fn f_006156_rage_atarray_rage_strindex_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006159_rage_strstreaminginfo_computeoccupiedvirtualsize_rage_strindex_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006162_rage_strstreaminginfo_gethandle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006164_rage_strstreaminginfo_clearflag_rage_strindex_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006167_rage_strstreaminginfomanager_isobjectinimage_rage_strindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006168_rage_strstreaminginfomanager_createdependentsgraph_rage_atarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006175_rage_strstreaminginfomanager_clearrequiredflag_rage_strindex_uns(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006183_rage_strstreaminginfomanager_getobjectindex_rage_strindex_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006186_rage_strstreaminginfomanager_flushloadedlist_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_006187_rage_strstreaminginfomanager_getpackfilerelationships_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006188_rage_atarray_int_0_unsigned_short_ptr_ref_rage_atmap_int_rage_at(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006189_rage_strstreaminginfomanager_registerdummyobject_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006192_rage_strstreaminginfomanager_getrequestinfopoolcapacity_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_006193_rage_strstreaminginfomanager_modifyhierarchystatusforarchive_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006194_rage_strstreaminginfomanager_clearflagforall_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_006196_rage_strstreaminginfomanager_setdonotdefrag_rage_strindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006197_rage_strloadedinfoiterator_strloadedinfoiterator_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006198_rage_strloadedinfoiterator_getnextindex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006210_rage_strstreamingengine_initlevelwithmapunloaded(ph: *mut Phantom); }
extern "C" { pub fn f_006211_rage_strstreamingloadermanager_flush(ph: *mut Phantom); }
extern "C" { pub fn f_006213_rage_strstreamingloader_loadrequestedobjects(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006215_rage_strstreamingloadermanager_loadrequestedobjects(ph: *mut Phantom); }
extern "C" { pub fn f_006216_rage_strstreamingloader_flush(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006220_rage_strstreamingengine_addrefbyptr_void_ptr_rage_strrefkind(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006221_rage_strstreamingengine_removerefbyptr_void_ptr_rage_strrefkind(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006225_rage_atvarstring_atvarstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_006232_rage_strstreamingloader_processasyncfiles(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006235_rage_strstreamingvisualize_setmarkertext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006238_rage_strstreamingloadermanager_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006241_rage_strstreamingvisualize_addmarker_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006254_rage_strrequest_request_rage_strlocalindex_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_006255_rage_strrequest_hasloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006256_rage_strrequest_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006257_rage_strrequest_clearrequiredflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006259_rage_strrequest_setrequiredflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006281_rage_strstreamingvisualize_addcallstacktomarker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006343_rage_tcmanager_findmodifierindex_rage_atnonfinalhashstring_char(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006356_rage_tcmanager_loadmodifierfile_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006357_rage_tcmanager_finishmodifierload(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006389_rage_tcmanager_removemodifierboxes_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006391_rage_tcmanager_addmodifierbox_rage_spdaabb_const_ref_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_006511_rage_audsound_computedurationmsincludingstartoffsetandpredelay_b(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006518_rage_audsound_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006525_rage_audsound_getwaveslot_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006610_rage_audsound_decompressmetadata_untyped_rage_sound_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006715_rage_audsound_prepareandplay_rage_audwaveslot_ptr_bool_int_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_006717_rage_audsound_stopandforget_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006743_rage_audspeechsound_findnumvariationsforvoiceandcontext_unsigned(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_006752_rage_audspeechsound_getwavenamehash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006877_rage_audmetadatamanager_getobjectmetadataptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006881_rage_audmetadatamanager_getobjectmetadatareffromhash_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006886_rage_audmetadatamanager_getobjectnamefromnametableoffset_unsigne(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006938_rage_audcurve_audcurve(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006940_rage_audcurve_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006941_rage_audcurve_calculatevalue_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_006942_rage_audcurve_calculaterescaledvalue_float_float_float_float_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32) -> f32; }
extern "C" { pub fn f_006944_rage_audmetadatamanager_getobjectname_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007032_rage_audmetadatamanager_getobjectinfo_unsigned_int_rage_audmetad(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007036_rage_audmetadatamanager_getobject_verify_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007037_rage_audmetadatamanager_getobject_verify_rage_audmetadataref_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007047_rage_audsoundfactory_getmetadataptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007050_rage_audsoundfactory_processhierarchy_rage_audmetadataref_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_007077_rage_audsoundset_init_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007078_rage_audsoundset_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007079_rage_audsoundset_find_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007080_rage_audsoundset_find_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007085_rage_audtracker_dtoraudtracker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007095_rage_audrequestedsettings_setpitch_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007098_rage_audrequestedsettings_setvolume_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007113_rage_audrequestedsettings_getpitch_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007115_rage_audrequestedsettings_getvolume_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_007120_rage_audsmoother_init_float_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_007121_rage_audsmoother_setrates_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_007125_rage_audsmoother_calculatevalue_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_007212_rage_audwaveslot_getreferencecount_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007226_rage_audwaveslot_getbankloadingstatus_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007229_rage_audwaveslot_removereference(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007230_rage_audwaveslot_removeslotreference_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007237_rage_audwaveref_init_rage_audwaveslot_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_007238_rage_audwaveref_findchunk_unsigned_int_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_007400_rage_audentity_allocateentityvariableblock_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007407_rage_audentity_createsound_persistentreference_rage_audmetadatar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007409_rage_audentity_createsound_persistentreference_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007411_rage_audentity_addbatchedsoundrequest_rage_audmetadataref_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007412_rage_audentity_createandplaysound_unsigned_int_rage_audsoundinit(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007416_rage_audentity_createandplaysound_persistent_unsigned_int_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007418_rage_audentity_stopandforgetsoundsprivate_rage_audsound_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i32); }
extern "C" { pub fn f_007426_rage_vfxbrokenfraginfo_dtorvfxbrokenfraginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007429_rage_vfxutil_inittxd_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007430_rage_vfxutil_shutdowntxd_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007465_rage_decalmanager_initsettings_int_rage_decaltypesettings_ptr_in(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_007483_rage_decalmanager_ondelete_rage_fwentity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007499_rage_decalmanager_removeinrange_rage_vec3v_const_ref_rage_scalar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007502_rage_decalmanager_ispointinliquid_unsigned_int_signed_char_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32) -> i32; }
extern "C" { pub fn f_007528_rage_ptfxattach_attach_rage_ptxeffectinst_ptr_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_007531_rage_ptfxattach_getisattachedto_rage_ptxeffectinst_ptr_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007554_rage_ptfxhistory_add_rage_atnonfinalhashstring_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_007555_rage_ptfxhistory_check_rage_atnonfinalhashstring_unsigned_int_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32) -> i32; }
extern "C" { pub fn f_007559_rage_ptfxmanager_gettriggeredinst_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007562_rage_ptfxmanager_getregisteredinst_void_const_ptr_int_bool_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_007565_rage_ptfxreg_find_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007570_rage_ptfxwrapper_setcolourtint_rage_ptxeffectinst_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007587_rage_ptfxwrapper_setalphatint_rage_ptxeffectinst_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007591_rage_ptfxwrapper_geteffectrule_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007642_rage_forcenopopupsbegin_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007643_rage_forcenopopupsend_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007655_rage_fwprofanityfilter_checkforprofanityandreservedterms_char_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007658_rage_fwptrlistsinglelink_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007659_rage_fwptrlistdoublelink_flush(ph: *mut Phantom); }
extern "C" { pub fn f_007660_rage_fwptrlist_countelements_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007661_rage_fwptrlist_ismemberoflist_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007666_rage_fwquadtreenode_getallmatching_rage_fwrect_const_ref_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007676_rage_fwangle_limitradianangleforpitch_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_007698_rage_fwportalcorners_calccloseness_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_007699_rage_fwportalcorners_haspassedthrough_rage_fwgeom_fwline_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_007700_rage_fwportalcorners_calcdistancetopoint_rage_vec3v_const_ref_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007701_rage_fwportalcorners_calcsimpledistancetopointsquared_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007703_rage_fwportalcorners_createextendedlineforportaltraversalcheck_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007754_rage_netinterface_getnumactiveplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007755_rage_netinterface_getallactiveplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007757_rage_netinterface_getsynchronisationtime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007759_rage_netinterface_getnumphysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007762_rage_netinterface_getallphysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007803_rage_networklogutils_writelogevent_rage_netlogginginterface_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007848_rage_neteventmgr_checkforspaceinpool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007849_rage_neteventmgr_prepareevent_rage_netgameevent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007949_rage_netgameevent_netgameevent_unsigned_short_bool_bool_rage_net(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_008074_rage_netobject_ispendingownerchange_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008077_rage_netobject_isglobalflagset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008083_rage_netobject_issyncedwithallplayers_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008155_rage_netsynctree_update_rage_netsynctreetargetobject_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_008172_rage_netobjectmgrbase_updateallinscopestateimmediately_rage_neto(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008214_rage_netsynctree_forcesendofsyncupdatenodes_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_008234_rage_netblenderlininterp_updatevelocity_rage_vector3_const_ref_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008380_rage_atarray_leaderboardcategorygroup_0_unsigned_short_op_rage_a(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008406_rage_netscgamerdatamgr_isrockstardev_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_008452_rage_atmapentry_rage_atstring_int_const_ref_rage_atmap_rage_atst(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008682_rage_fwbasepool_fwbasepool_int_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_008683_rage_fwconfigmanager_getsizeofpool_unsigned_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008686_rage_fwbasepool_getindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008688_rage_fwbasepool_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008689_rage_fwscriptguid_createguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008693_rage_fwscriptguid_getbasefromguid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_008710_rage_scripthandler_getscriptresource_unsigned_short_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_008714_rage_scripthandler_removeallscriptresourcesoftype_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008768_rage_scripthandlermgr_removescriptresource_unsigned_short_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_008769_rage_scripthandlermgr_removeallscriptresourcesoftype_unsigned_sh(ph: *mut Phantom); }
extern "C" { pub fn f_008773_rage_scripthandlernetcomponent_isplayeraparticipant_unsigned_cha(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008912_rage_fwbasepool_new_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008917_rage_fwtimer_enduserpause(ph: *mut Phantom); }
extern "C" { pub fn f_008918_rage_fwtimer_startuserpause_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008924_rage_fwtimer_update(ph: *mut Phantom); }
extern "C" { pub fn f_008925_rage_fwtimer_startscriptpause(ph: *mut Phantom); }
extern "C" { pub fn f_008926_rage_fwtimer_endscriptpause(ph: *mut Phantom); }
extern "C" { pub fn f_008935_rage_setdebugownerhistoryresolver_void_ptr_void_const_ptr_rage_a(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008936_rage_fwdescpool_fwdescpool_unsigned_int_unsigned_int_rage_memory(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008937_rage_fwdescpool_new_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008940_rage_fwdescpool_delete_rage_fwdescpoolalloc_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008941_rage_fwdescpool_resize_rage_fwdescpoolalloc_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_008945_rage_fwbasepool_forall_bool_ptr_void_ptr_void_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008949_rage_fwbasepool_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008950_rage_fwbasepool_getjustindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008951_rage_fwbasepool_isvalidptr_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008952_rage_fwbasepool_getat_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008953_rage_fwknownrefholder_fwknownrefholder_void_ptr_ptr_rage_fwknown(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_009048_rage_crproperty_getattribute_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009050_rage_crproperty_stringkey_calckey_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009212_rage_crskeletondata_computeglobaltransform_int_rage_mat34v_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_009218_rage_crskeleton_crskeleton(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009222_rage_crskeleton_init_rage_crskeletondata_const_ref_rage_mat34v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009224_rage_crskeleton_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009226_rage_crskeleton_partialupdate_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009228_rage_crskeleton_copyfrom_rage_crskeleton_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009230_rage_crskeleton_inverseupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009233_rage_crskeleton_getglobalmtx_unsigned_int_rage_mat34v_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009283_rage_grmmodel_getgeometry_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009314_pixend(ph: *mut Phantom); }
extern "C" { pub fn f_009317_rage_grmshader_load_char_const_ptr_rage_fitokenizer_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_009320_rage_grmmodelfactory_buildcacheddeclarator_rage_grcfvf_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_009321_rage_grmmodelfactory_builddeclarator_rage_grcfvf_const_ptr_rage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009322_rage_grmmodelfactory_computevertexelements_rage_grcfvf_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_009337_rage_pgdictionary_rage_grctexture_addentry_char_const_ptr_rage_g(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_009342_rage_grmmatrixset_setdirty_bool_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009343_rage_grmmatrixset_create_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_009400_pixbeginc_unsigned_int_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_009447_rage_pfdrawitembase_beginmanager(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009448_rage_pfdrawitembase_end(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009499_rage_grcfvf_setposchannel_bool_rage_grcfvf_grcdatasize_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009514_rage_grctexture_setcustomloadname_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009517_rage_grcimage_release_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009518_rage_grcimage_create_unsigned_int_unsigned_int_unsigned_int_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_009519_rage_grcimage_makechecker_int_rage_color32_rage_color32(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_009521_rage_grctexturefactory_registertexturereference_char_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009525_rage_grcimage_loaddds_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009526_rage_grcimage_load_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009531_rage_grctexturedx11_grctexturedx11_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009533_rage_grctexturefactory_createparams_createparams_unsigned_short(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009539_rage_grceffect_lookuptechnique_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009540_rage_grceffect_lookupvar_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009542_rage_grcstateblock_createblendstate_rage_grcblendstatedesc_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009544_rage_grcdrawsinglequadf_float_float_float_float_float_float_floa(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: i64); }
extern "C" { pub fn f_009545_rage_grcstateblock_setblendstate_rage_grcstateblock_grcblendstat(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009556_rage_grcbatcher_reserve_unsigned_int_unsigned_long(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_009559_rage_grcdevice_clear_bool_rage_color32_bool_float_bool_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_009560_rage_grcstateblock_setdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009588_rage_grcimage_dtorgrcimage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009589_rage_grcimage_getextramipcount_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009590_rage_grcimage_clear_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009594_rage_grcimage_byteswapdata_void_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009597_getddsformat_rage_ddpixelformat9_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009598_rage_grcimage_savedds_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009603_rage_grcviewport_setworldidentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009608_rage_grcviewport_grcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009613_rage_grcviewport_dtorgrcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009615_rage_grceffect_lookupglobalvar_char_const_ptr_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009618_rage_grcviewport_perspective_float_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_009622_rage_grcviewport_setfarclip_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_009636_rage_grcviewport_setcameramtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009641_rage_grcviewport_isaabbvisible_float_vector_4_float_vector_4_rag(ph: *mut Phantom, a0: u128, a1: u128, a2: i64) -> i32; }
extern "C" { pub fn f_009646_rage_grcviewport_transform_rage_vec3v_const_ref_float_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009649_rage_grcworldidentity(ph: *mut Phantom); }
extern "C" { pub fn f_009675_rage_grcstateblock_createdepthstencilstate_rage_grcdepthstencils(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009716_rage_grcdevice_createvertexdeclaration_rage_grcvertexelement_con(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009718_rage_grcbufferd3d11_initialise_unsigned_int_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32); }
extern "C" { pub fn f_009754_rage_grclighting_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_009756_rage_grcdrawbox_rage_vector3_const_ref_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009765_rage_grcvertexbuffer_create_int_rage_grcfvf_const_ref_bool_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_009768_rage_grcvertexbuffer_makereadonly(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009800_rage_grcimagejpeggetclosestscaledenom_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_009803_rage_grcimage_loadjpegtorgba8surface_rage_fistream_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_009805_rage_grcimage_getjpeginfoforrgba8surface_rage_fistream_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_009808_rage_grceffect_getvardesc_rage_grceffectvar_char_const_ptr_ref_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_009858_rage_grcstateblock_createrasterizerstate_rage_grcrasterizerstate(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009863_rage_grceffect_findtechniquegroupid_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009867_rage_grcinstancedata_dtorgrcinstancedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009869_rage_grceffect_getlocalcommon_rage_grcinstancedata_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_009880_rage_grceffect_getvar_rage_grcinstancedata_const_ref_rage_grceff(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009904_rage_gridmonitor_getarea_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009908_rage_monitorconfiguration_update(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009912_rage_monitorconfiguration_getlandscapemonitor_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009921_rage_grcstaticinstancebufferlist_grcstaticinstancebufferlist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009922_rage_grcstaticinstancebufferlist_init_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009951_rage_grcvertexbufferd3d11_lockinternal_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009952_rage_grcvertexbufferd3d11_unlockinternal_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009986_rage_grcdebugdraw_adddebugoutputex_bool_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_009992_rage_grcdevice_allocfence_unsigned_int(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009993_rage_grcdevice_cpufreefence_rage_grcfencehandle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010022_rage_grcdevice_destroyocclusionquery_iunknown_ptr_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010056_rage_grctexturepc_hdoverrideswap_rage_grctexturepc_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010091_rage_grctexturefactorydx11_translatetorageformat_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_010102_rage_grctexturefactorydx11_getd3dfromatfromgrcimageformat_unsign(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_010111_rage_grctexturefactorydx11_createrendertarget_char_const_ptr_id3(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_010144_rage_grcbufferd3d11_grcbufferd3d11(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010145_rage_grcbufferd3d11_dtorgrcbufferd3d11(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010148_rage_grcbufferd3d11resource_grcbufferd3d11resource(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010149_rage_grcbufferd3d11resource_dtorgrcbufferd3d11resource(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010150_rage_grcbufferd3d11resource_initialise_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_010154_rage_batcherlock_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010161_rage_grcdebugdraw_startrecording_rage_grcdebugdraw_lingerbuffer(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_010163_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010164_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010166_rage_grcdebugdraw_quad_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010170_rage_grcdebugdraw_rectaxisaligned_rage_vec2v_const_ref_rage_vec2(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010172_rage_grcdebugdraw_text_float_float_rage_color32_char_const_ptr_b(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i64, a4: i32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010179_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010180_rage_grcdebugdraw_quad_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_010181_rage_grcdebugdraw_poly_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010183_rage_grcdebugdraw_cross_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010188_rage_grcdebugdraw_boxaxisaligned_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010189_rage_grcdebugdraw_boxoriented_rage_vec3v_const_ref_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010192_rage_grcdebugdraw_cone_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010193_rage_grcdebugdraw_capsule_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010194_rage_grcdebugdraw_axis_rage_mat34v_const_ref_float_bool_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_010195_rage_grcdebugdraw_arrow_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_010214_rage_grcdebugdraw_textfontpush_rage_grcfont_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010215_rage_grcdebugdraw_textfontpop(ph: *mut Phantom); }
extern "C" { pub fn f_010216_rage_grcdebugdraw_text_float_float_rage_dd_epixelcoordspace_rage(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i32, a4: i64, a5: i32, a6: f32, a7: f32); }
extern "C" { pub fn f_010219_rage_grcdebugdraw_adddebugoutputex_bool_rage_color32_char_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_010220_rage_grcdebugdraw_adddebugoutput_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010221_rage_grcdebugdraw_adddebugoutput_rage_color32_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_010224_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010225_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010500_rage_iovalue_isdown_float_rage_iovalue_readoptions_const_ref_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010501_rage_iovalue_historyhelddown_unsigned_int_float_rage_iovalue_rea(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_010505_rage_iovalue_wasdown_float_rage_iovalue_readoptions_const_ref_co(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010511_rage_iovalue_ispressed_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010514_rage_iovalue_isreleased_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010518_rage_iovalue_getnorm_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010602_jpeg_std_error_jpeg_error_mgr_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010671_rage_rmcdrawable_dtorrmcdrawable(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010688_rage_rmclodgroup_computebucketmask_rage_grmshadergroup_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010700_rage_rmclodgroup_getbucketmask_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_010710_rage_rmclodgroup_isempty(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_010712_rage_rmclodgroup_islodinstanced_int_rage_grmshadergroup_const_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_010737_rage_spdaabb_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_010747_rage_spdtransposedplaneset8_set_rage_grcviewport_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_010931_rage_cutfobjectidevent_cutfobjectidevent_int_float_int_rage_cutf(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_010941_rage_atarray_int_0_unsigned_short_resizegrow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011047_rage_cutffloatvalueeventargs_cutffloatvalueeventargs_float(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_011155_void_rage_atplaceptrcreator_void_ptr_rage_slightdetails_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011218_rage_cutfcutscenefile2_removeeventfromlist_rage_cutfevent_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011220_rage_cutfcutscenefile2_addeventtolist_rage_cutfevent_ptr_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011225_rage_cutfcutscenefile2_findeventsforobjectidonly_int_rage_atarra(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_011734_rage_fraginst_getarticulatedbody(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011739_rage_fraginst_fraginst_rage_fragtype_const_ptr_rage_matrix34_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_011740_rage_fraginst_setdisablebreakable_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011748_rage_fraginst_getpartbroken_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011750_rage_fraginst_getchildbroken_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011751_rage_fraginst_getcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011753_rage_fraginst_getcontrollingcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011757_rage_fraginst_getbonematrix_rage_matrix34_ref_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_011760_rage_fraginst_breakoffabovegroup_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_011769_rage_fraginst_zeroskeletonmatricesbygroup_rage_crskeleton_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011776_rage_fragtype_getboneindexfromid_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011778_rage_fraginst_damageallgroups(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011788_rage_fraginst_poseskeletonfromarticulatedbody(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011794_rage_fragtype_willbearticulated_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011801_rage_fraginst_applydamage_int_float_rage_vec3v_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32); }
extern "C" { pub fn f_011822_rage_matrix34_dot_rage_matrix34_const_ref_rage_matrix34_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011825_rage_fraginst_poseboundsfromarticulatedbody(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011858_rage_fragmanager_setinterestfrustum_rage_grcviewport_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011859_rage_fragmanager_update_float_int(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_011876_rage_fragphysicslod_gettotalundamagedmass_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_011964_rage_fragtype_getskeletondata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012021_rage_fragphysicslodgroup_getlodbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012037_rage_fragcacheentry_cloneboundparts(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012183_rage_ptxeffectinst_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012189_rage_ptxeffectinst_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012196_rage_ptxeffectinst_setbasemtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012201_rage_ptxeffectinst_setbasepos_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012202_rage_ptxeffectinst_setoffsetmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012203_rage_ptxeffectinst_setoffsetpos_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012205_rage_ptxeffectinst_setoffsetrot_rage_quatv_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012207_rage_ptxeffectinst_setevovaluefromhash_rage_athashvalue_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_012209_rage_ptxeffectinst_setoverridedistsqrtocamera_float_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_012269_rage_atarray_rage_ptxkeyframeentry_0_unsigned_short_grow_unsigne(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012435_rage_atmapentry_unsigned_int_void_ptr_rage_ptxbehaviour_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012671_rage_snsession_getsessionid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012843_rage_environmentcloth_getcloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012869_rage_environmentcloth_getbbminworldspace(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012870_rage_environmentcloth_getbbmaxworldspace(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012892_rage_characterclothcontroller_setpackageindex_unsigned_char_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_012971_rage_phmorphdata_dtorphmorphdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013019_rage_bgglassmanager_getglassbreakable_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_013109_rage_bgglassmanager_setviewport_rage_grcviewport_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013153_rage_rsttransformtoscreenspace_rsttransformtoscreenspace_rage_gr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_013191_rage_rstgetboxhull_rage_vec3v_const_ref_rage_vec3v_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_013192_rage_rsttransformhullandgetminz_rage_mat44v_const_ref_rage_vec2v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64) -> i32; }
extern "C" { pub fn f_013198_rage_rsttestaabbexact_rage_mat44v_const_ref_rage_scalarv_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_013451_rage_netstatus_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013598_rage_netpeerid_op_rage_netpeerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013602_rage_netpeerid_op_rage_netpeerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013641_rage_rc4context_rc4context(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013668_rage_nethttprequest_nethttprequest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013671_rage_nethttprequest_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013677_rage_nethttprequest_init_rage_sysmemallocator_ptr_wolfssl_ctx_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013693_rage_nethttprequest_hasrequestheader_char_const_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013695_rage_nethttprequest_addrequestheadervalue_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013705_rage_nethttprequest_addformparam_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013706_rage_nethttprequest_addintformparam_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013713_rage_nethttprequest_appendcontent_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_014067_rage_nettimesync_gettime_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014068_rage_nettimesync_hasstarted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014259_rage_rltaskmanager_doaddtask_unsigned_long_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014260_rage_rltaskmanager_destroytask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014276_rage_rlpc_update(ph: *mut Phantom); }
extern "C" { pub fn f_014292_rage_rlgettaskmanager(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_014294_rage_rlgetposixtime_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014316_rage_rlgamerinfo_getgamerhandle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014318_rage_rlgamerinfo_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014320_rage_rlpc_isinitialized(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014349_rage_rlgamerhandle_touserid_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014361_rage_rlros_getcredentials_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014362_rage_rlroscredentials_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014378_rage_rlgamerid_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014379_rage_rlgamerid_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014384_rage_rlpresence_issignedin_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014392_rage_rlgamerinfo_getgamerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014448_rage_rlpresence_isguest_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014449_rage_rlpresence_adddelegate_rage_atdelegator1_void_rage_rlpresen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014450_rage_rlpresence_removedelegate_rage_atdelegator1_void_rage_rlpre(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014459_rage_rltaskbase_configure_rage_netstatus_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014460_rage_rltaskmanager_addparalleltask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014487_rage_rlsessioninfo_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014606_rage_rlroshttptask_configure_int_rage_rlhttptaskconfig_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_014607_rage_rlhttptask_addintparameter_char_const_ptr_long_long_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_014609_rage_rlstats_cancel_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014733_char_ptr_rage_formatf_char_33ul_char_ref_33ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_014734_char_ptr_rage_formatf_char_97ul_char_ref_97ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_014735_rage_rlhttptask_addmultipartunsparameter_char_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014836_rage_rlroscredentials_hasprivilege_rage_rlrosprivilegeid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014859_rage_rlros_getdefaultprivilege_rage_rlrosprivilegeid(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014945_rage_rlsocialclub_postuserfeedactivity_int_char_const_ptr_long_c(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014981_rage_rlugcmetadata_composecloudabspath_int_char_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_014983_rage_rlugcmetadata_getfileid_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014984_rage_rlugcmetadata_getfileversion_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014985_rage_rlugcgta5photometadatadata_getstagingurl_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014986_rage_rlugcgta5photometadatadata_getstagingauthtoken_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015040_rage_rlcloudmemberid_rlcloudmemberid_rage_rlgamerhandle_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015048_rage_rlcloud_getmemberfile_int_rage_rlcloudmemberid_const_ref_ra(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64) -> i32; }
extern "C" { pub fn f_015050_rage_rlcloud_postmemberfile_int_rage_rlcloudonlineservice_char_c(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_015220_rage_rlsavemigrationstate_set_int_bool_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_015771_rage_computeaxisangleprecise_rage_quatv_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015775_rage_phmathinertia_findcylinderanginertia_float_float_float_rage(ph: *mut Phantom, a0: f32, a1: f32, a2: i64); }
extern "C" { pub fn f_015819_rage_phmanifold_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015840_rage_phcontact_computelocalpointbfromworld_rage_phmanifold_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015841_rage_phcontact_computelocalpointafromworld_rage_phmanifold_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015843_rage_phconstraintdistance_getworldposa(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015844_rage_phconstraintdistance_getworldposb(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015864_rage_phconstrainthalfspace_setworldnormal_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015882_rage_phconstraintmgr_insert_rage_phconstraintbase_params_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015890_rage_phconstraintmgr_remove_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015891_rage_phconstraintmgr_gettemporarypointer_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015892_rage_phconstraintmgr_removeactiveconstraints_rage_phinst_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015897_rage_phconstraintrotation_setworldaxis_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015916_rage_phconstraintattachment_setlocalposa_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015917_rage_phconstraintattachment_setlocalposb_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015944_rage_phcontact_computerelvelocity_rage_vec3v_const_ref_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_015951_rage_phmanifold_removecontactpoint_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_015954_rage_phmanifold_refreshcolliderpointers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015958_rage_phcollider_initinertia(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015962_rage_phcollider_setinstanceandreset_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015965_rage_phcollider_setcollidermatrixfrominstancerigid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015966_rage_phsleep_reset_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015969_rage_phcollider_setreferenceframevelocity_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015970_rage_phcollider_setreferenceframeangularvelocity_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015980_rage_phcollider_getlocalvelocityrigid_float_vector_4_const(ph: *mut Phantom, a0: i64, a1: i64, a2: u128); }
extern "C" { pub fn f_015981_rage_phcollider_applyimpulserigid_float_vector_4_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128, a2: u128); }
extern "C" { pub fn f_015982_rage_phcollider_applyimpulsecenterofmassrigid_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015983_rage_phcollider_applyangimpulserigid_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015985_rage_phcollider_applyforceimp_float_vector_4_float_vector_4_int(ph: *mut Phantom, a0: i64, a1: u128, a2: u128); }
extern "C" { pub fn f_015986_rage_phcollider_applyforcecenterofmassimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015987_rage_phcollider_applytorqueimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015988_rage_phcollider_setvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015998_rage_phsimulator_deleteobject_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016005_rage_phsimulator_activateobject_int_rage_phcollider_ptr_rage_phi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016006_rage_phsimulator_applyimpetusandbreakingimpetus_float_vector_4_i(ph: *mut Phantom, a0: i64, a1: u128, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: f32); }
extern "C" { pub fn f_016008_rage_phsimulator_applyforceandbreakingforce_float_vector_4_int_r(ph: *mut Phantom, a0: i64, a1: u128, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: f32, a8: i32); }
extern "C" { pub fn f_016010_rage_phsimulator_getlastinstancematrix_rage_phinst_const_ptr_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016011_rage_phsimulator_setlastinstancematrix_rage_phinst_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016012_rage_phsimulator_addactiveobject_rage_phinst_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016013_rage_phsimulator_addinactiveobject_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016014_rage_phsimulator_addfixedobject_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016015_rage_phsimulator_getobjectvelocity_rage_phinst_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016017_rage_phsimulator_deactivateobject_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016018_rage_phsimulator_activateobject_rage_phinst_ptr_rage_phcollider(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_016019_rage_phsimulator_deactivateobject_rage_phinst_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016029_rage_phsleep_blocksleepfinishing_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016043_rage_pharchetype_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016045_rage_pharchetype_removeincludeflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016086_rage_phinst_init_rage_pharchetype_ref_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016087_rage_phinst_setarchetype_rage_pharchetype_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016088_rage_phinst_setinstflag_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016164_rage_matrix34_dot_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016254_rage_pharticulatedbody_getlocalvelocitynoprop_int_float_vector_4(ph: *mut Phantom, a0: i64, a1: i32, a2: u128, a3: i64); }
extern "C" { pub fn f_016256_rage_pharticulatedbody_getlinearvelocity_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016257_rage_pharticulatedbody_resetdownvel_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016263_rage_pharticulatedbody_setstiffness_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016267_rage_pharticulatedbody_ensurevelocitiesfullypropagated(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016271_rage_pharticulatedbody_applyincrementalimpulse_int_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_016273_rage_pharticulatedbody_findjointlimitdofs_int_ptr_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i64) -> i32; }
extern "C" { pub fn f_016278_rage_pharticulatedbody_dtorpharticulatedbody(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016283_rage_phbound_getcomputeangularinertia_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_016285_rage_phjoint_setminstiffness_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016287_rage_pharticulatedbody_calculateinertias(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016297_rage_pharticulatedcollider_removefromactivelist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016301_rage_pharticulatedcollider_setcollidermatrixfrominstanceart(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016312_rage_phboundcomposite_updatebvh_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016317_rage_phboundcomposite_calculatecompositeextents_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016318_rage_pharticulatedcollider_applyangimpulseart_float_vector_4_int(ph: *mut Phantom, a0: i64, a1: u128, a2: i32); }
extern "C" { pub fn f_016321_rage_pharticulatedcollider_resetsavedvelocities(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016322_rage_pharticulatedcollider_setvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016323_rage_pharticulatedcollider_setangvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016324_rage_pharticulatedcollider_applyimpulseart_float_vector_4_float(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: i32); }
extern "C" { pub fn f_016325_rage_pharticulatedcollider_applyforceimp_float_vector_4_float_ve(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: i32, a4: u128); }
extern "C" { pub fn f_016326_rage_pharticulatedcollider_applyforcecenterofmassimp_float_vecto(ph: *mut Phantom, a0: i64, a1: u128, a2: u128); }
extern "C" { pub fn f_016327_rage_pharticulatedcollider_applyimpulsecenterofmassart_float_vec(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016328_rage_pharticulatedcollider_applytorqueimp_float_vector_4_float_v(ph: *mut Phantom, a0: i64, a1: u128, a2: u128); }
extern "C" { pub fn f_016333_rage_pharticulatedcollider_getlocalvelocityart_float_vector_4_in(ph: *mut Phantom, a0: i64, a1: i64, a2: u128, a3: i32); }
extern "C" { pub fn f_016341_rage_phjoint_getchildlink_rage_pharticulatedbody_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016370_rage_phbound_setcgoffset_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016385_rage_phbound_istypecomposite_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_016395_rage_phboundbox_setboxsize_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016437_rage_phboundcapsule_setcapsuleradius_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016439_rage_phboundcapsule_setcapsulesize_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_016464_rage_phboundcomposite_setbound_int_rage_phbound_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016471_rage_phboundcomposite_allocatetypeandincludeflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016476_rage_phboundcomposite_cloneparts(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016531_rage_phboundsphere_setsphereradius_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016740_rage_phcontactmgr_resetwarmstartallcontactswithinstance_rage_phi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016886_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectlocation(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016889_rage_phlevelnodetree_rage_phlooseoctreenode_commitdeferredoctree(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016892_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectlocation(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016893_rage_phlevelnodetree_rage_phlooseoctreenode_rebuildcompositebvh(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016897_rage_phlevelnodetree_rage_phlooseoctreenode_getfirstactiveindex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_016898_rage_phlevelnodetree_rage_phlooseoctreenode_getnextactiveindex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_016899_rage_phlevelnodetree_rage_phlooseoctreenode_reserveinstlastmatri(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016900_rage_phlevelnodetree_rage_phlooseoctreenode_releaseinstlastmatri(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016901_rage_phlevelnodetree_rage_phlooseoctreenode_setlastinstancematri(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016909_rage_phlevelnodetree_rage_phlooseoctreenode_testprobe_rage_phseg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_016911_rage_phlevelnodetree_rage_phlooseoctreenode_findandaddoverlappin(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016944_rage_scrthread_preprintstacktrace(ph: *mut Phantom); }
extern "C" { pub fn f_016949_rage_scrthread_getactivethread(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_016952_rage_scrthread_getprogramcounter_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016958_rage_scrthread_isvalidglobal_rage_scrvalue_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_016962_rage_scrthread_registerunusedcommand_unsigned_long_void_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016967_rage_scrthread_registercommand_unsigned_long_void_ptr_rage_scrth(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016981_rage_scrthread_getthread_rage_scrthreadid(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_017281_rage_crexpressions_initializecreature_rage_crcreature_ref_rage_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017294_rage_atarray_rage_crexpressions_track_0_unsigned_short_grow_unsi(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_017859_rage_crmtmotiontreescheduler_waitoncomplete_rage_crmtmotiontree(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_018169_rage_mvparameterbuffer_generatefreecapacity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018834_cgen9exclusiveassets_isgen9exclusivepeddrawable_cped_const_ptr_e(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_018877_cleveldata_loadxmldata(ph: *mut Phantom); }
extern "C" { pub fn f_018884_cgarage_storeandremovecarsforthishideout_cstoredvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_018889_cleveldata_getfriendlyname_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_018921_rage_fwentity_getmodelid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018928_cgarage_ispointinsidegarage_rage_vector3_const_ref_float_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_018931_cgarages_findgarageindexfromnamehash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_018937_cgarages_isvehiclewithinhideoutgaragethatcansaveit_cvehicle_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019027_cleveldata_getfilename_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_019031_cwaypointrecordingroute_addwaypoint_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32); }
extern "C" { pub fn f_019086_cwaypointrecording_edit_initwaypointwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_019099_cpopulationhelpers_getconfigvalue_int_const_ref_int_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019121_cvehiclerecordingmgr_crecording_stoprecording_unsigned_char_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019136_cvehiclerecordingmgr_isplaybackgoingonforcar_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019144_cvehiclerecordingmgr_retrievedataforthisframe(ph: *mut Phantom); }
extern "C" { pub fn f_019239_cstuntjumpmanager_validatestuntjumpstats_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019251_ctrafficlights_calculatenodelightcycle_rage_cpathnode_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019253_ctrafficlights_renderlight_etrafficlightcommand_rage_mat34v_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i32); }
extern "C" { pub fn f_019255_ctrafficlights_renderpedlights_centity_ptr_basemodelinfoboneindi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_019293_caitarget_caitarget_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019310_cexpensiveprocess_cexpensiveprocess_cexpensiveprocess_processtyp(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_019311_centityscanner_dtorcentityscanner(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019313_cexpensiveprocess_dtorcexpensiveprocess(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019321_cpedscanner_cpedscanner_cexpensiveprocess_processtype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019325_cspatialarray_findinsphere_rage_vec3v_const_ref_float_cspatialar(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_019326_cvehiclescanner_cvehiclescanner_cexpensiveprocess_processtype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019330_cobjectscanner_cobjectscanner_cexpensiveprocess_processtype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019331_cobjectscanner_dtorcobjectscanner(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019340_cexpensiveprocessdistributer_addprocess_cexpensiveprocess_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019342_cspatialarray_cspatialarray_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_019343_cspatialarray_insert_cspatialarraynode_ref_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019344_cspatialarray_remove_cspatialarraynode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019345_cspatialarray_update_cspatialarraynode_ref_float_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_019346_cspatialarray_settypeflags_cspatialarraynode_ref_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019349_cspatialarray_findbelowz_rage_scalarv_const_ref_cspatialarraynod(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_019387_cambientmodelsetmanager_getmodelsetindexfromhash_cambientmodelse(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_019452_ailogging_getdynamicentityisclonestringsafe_cdynamicentity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019453_ailogging_getdynamicentitynamesafe_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019458_ailogging_stacktracelog_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019462_cailogmanager_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019565_cpedbonetagconvertor_getboneindexfrombonetag_rage_crskeletondata(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_019567_cdebugclipdictionary_pushassetfolder_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019607_cfacialdatacomponent_onfacialidlechanged_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019608_cfacialdatacomponent_setfacialidlemode_cped_ptr_cfacialdatacompo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019609_cfacialdatacomponent_playfacialanim_cped_ptr_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019613_cfacialdatacomponent_processfacialhashkey_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019615_cgesturemanager_canusevehiclegestureclipset_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019616_cgesturemanager_canusescenariogestureclipset_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019619_cmoveanimatedbuilding_dtorcmoveanimatedbuilding(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019621_cmoveanimatedbuilding_setphase_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_019622_cmoveanimatedbuilding_setrate_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_019634_cmoveped_settasknetwork_rage_fwmovenetworkplayer_ptr_float_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_019635_cmoveped_setsecondarytasknetwork_rage_fwmovenetworkplayer_ptr_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_019636_cmoveped_setadditivenetwork_rage_fwmovenetworkplayer_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_019637_cmoveped_setfacialnetwork_rage_fwmovenetworkplayer_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_019638_cmoveped_dtorcmoveped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019649_cmoveped_playvisemeclip_rage_crclip_const_ptr_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_019651_cmoveped_clearhandanimation_float_cmoveped_epedhand(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019666_cmoveped_switchtostaticframe_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019667_cmoveped_switchtoanimated_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_019686_cmoveobject_dtorcmoveobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019704_cclipeventtags_carmsikeventtag_getallowed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019733_cclipeventtags_crandomizestartphaseproperty_pickrandomstartphase(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_019734_cclipeventtags_crandomizerateproperty_pickrandomrate_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_019740_cclipeventtags_cmotionblureventtag_update(ph: *mut Phantom); }
extern "C" { pub fn f_019844_rage_matrix34_inverse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019945_rage_matrix34_dotfromleft_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019954_canimviewer_ondynamicentityprerender(ph: *mut Phantom); }
extern "C" { pub fn f_020448_canimscenemanager_update_animsceneupdatephase(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021008_rage_atbinarymap_int_unsigned_int_datapair_ptr_std_2_partial_sor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_021009_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_021010_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_021143_audspeechaudioentity_setambientvoicename_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021156_audscriptaudioentity_stopstream(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021169_audradiotrack_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021179_audvehicleaudioentity_setradiostation_audradiostation_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_021180_audspeechmanager_requesttennisbanks_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021181_audspeechmanager_unrequesttennisbanks(ph: *mut Phantom); }
extern "C" { pub fn f_021183_audspeechaudioentity_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021193_audweaponaudioentity_addallinventoryweaponsonped_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021210_audscannermanager_cancelallreports(ph: *mut Phantom); }
extern "C" { pub fn f_021217_audpedaudioentity_getbjvehicle(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_021218_audinteractivemusicmanager_ismusicplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021222_audsuperconductor_sendconductormessage_conductors_conductormessa(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021340_audpedfootstepaudio_addfootstepevent_audfootstepevent(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021346_audpedaudioentity_triggerimpactsounds_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_021352_audspeechaudioentity_playanimalvocalization_unsigned_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_021354_audspeechaudioentity_say_char_const_ptr_char_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i64) -> i32; }
extern "C" { pub fn f_021355_audscriptaudioentity_pausescriptedconversation_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_021356_audscriptaudioentity_restartscriptedconversation_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_021359_auddamagestats_dtorauddamagestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021361_audspeechaudioentity_freeslot_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021362_audpedfootstepaudio_addslopedebrisevent_audfootstepevent_rage_ve(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32); }
extern "C" { pub fn f_021365_audspeechaudioentity_setblockfallingvelocitycheck(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021366_audspeechaudioentity_inflictpain_auddamagestats_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021375_audpedaudioentity_handleanimeventflag_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021376_audpedaudioentity_dtoraudpedaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021378_audpedaudioentity_init_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021380_audpedfootstepaudio_getupperbodyclothsounds_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021382_audpedfootstepaudio_getextraclothsounds_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_021385_audspeechaudioentity_getpedvoicegroup_bool_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_021386_audspeechaudioentity_getpvghashfromrace_rage_pedracetopvg_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_021387_audpedaudioentity_playringtone_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021388_audpedaudioentity_stopringtone(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021389_audspeechmanager_getambientspeechslot_audspeechaudioentity_ptr_b(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_021393_audpedaudioentity_isringtoneplaying(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021395_audpedaudioentity_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021397_audpedaudioentity_playvehicleimpact_rage_vector3_const_ref_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_021400_rage_audsound_setrequestedpitch_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021403_audspeechaudioentity_stopspeech(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021405_audspeechaudioentity_ispainplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021406_audspeechaudioentity_isscriptedspeechplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021407_audspeechaudioentity_isambientspeechplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021419_audpedfootstepaudio_addbushevent_audfootstepevent_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021420_audpedaudioentity_handleprecomputecontact_rage_phcontactiterator(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021422_cpedmotiondata_getisrunning_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021423_audpedaudioentity_playunderovervehiclesounds_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_021424_audpedaudioentity_triggerbigsplash(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021447_audpedfootstepaudio_getbodyclothsounds_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_021448_audpedfootstepaudio_getshoetype(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021450_audpedfootstepaudio_iswearingheels(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021451_audpedfootstepaudio_iswearinghighheels(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021452_audpedfootstepaudio_setscriptsweetenersoundset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021453_audpedfootstepaudio_resetscriptsweetenersoundset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021454_audpedfootstepaudio_updatevariationdata_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021455_audpedfootstepaudio_resetwetfeet_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021457_audpedfootstepaudio_addwaterevent_audfootstepevent(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021458_audpedfootstepaudio_addclothevent_audfootstepevent(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021459_audpedfootstepaudio_addpetrolcanevent_audfootstepevent_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021460_audpedfootstepaudio_addmolotovevent_audfootstepevent_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021461_audpedfootstepaudio_addscriptsweetenerevent_audfootstepevent(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021462_audpedfootstepaudio_setstandingmaterial_unsigned_long_centity_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_021463_audpedfootstepaudio_changestandingmaterial_unsigned_long_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_021465_audpedfootstepaudio_stopladderslide(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021508_audscriptaudioentity_triggeraircraftwarningspeech_audaircraftwar(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021543_audplaneaudioentity_setstallwarningenabled_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021553_audpolicescanner_describeposition_rage_vector3_const_ref_audposi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_021554_audpolicescanner_reportcrime_audcrimereport_const_ref_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021559_audpolicescanner_reportpolicespottingplayer_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_021561_audscannermanager_triggercopdispatchinteraction_scannercopdispat(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021562_audpolicescanner_reportpolicespottingnonplayer_cped_const_ptr_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_021570_audradiostation_locktracklist_rage_radiostationtracklist_const_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_021571_audradiostation_getstation_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_021572_audradiostation_forcemusictracklist_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021575_audradiotrack_findtextidforsoundhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021579_audradiotrack_getbeatsoundname_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021596_audradioaudioentity_setfrontendfadetime_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_021597_audvehicleaudioentity_gethasnormalradio_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021598_audvehicleaudioentity_isradioenabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021599_audradioaudioentity_retunetostation_audradiostation_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021600_audradiostation_findstation_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021603_audradiostation_getmusictracklistactivetrackindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021604_audradiostation_getmusictracklisttrackstarttimems_unsigned_int_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_021605_audradiostation_getmusictracklisttrackfortimeoffset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_021606_audradiostation_findhistoryforcategory_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_021607_audradiotrack_getnextbeat_float_ref_float_ref_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_021608_audradiostation_findstation_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_021620_audradioaudioentity_setmobilephoneradiostate_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021627_audradiostation_getstationindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021628_audradiostation_isfavourited_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021635_audradioaudioentity_skipstationsforward(ph: *mut Phantom); }
extern "C" { pub fn f_021638_audradioaudioentity_getplayerradiostationnamependingretune_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_021639_audradioaudioentity_retuneradioup(ph: *mut Phantom); }
extern "C" { pub fn f_021640_audradioaudioentity_retuneradiodown(ph: *mut Phantom); }
extern "C" { pub fn f_021641_audradioaudioentity_retunetostation_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021644_audradioaudioentity_getaudibletracktextid_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021645_audradioaudioentity_getcachednextaudiblebeat_float_ref_float_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_021646_audradioaudioentity_startendcredits(ph: *mut Phantom); }
extern "C" { pub fn f_021647_audradioaudioentity_stopendcredits(ph: *mut Phantom); }
extern "C" { pub fn f_021649_audradiostation_unlocktracklist_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_021693_audradioslot_setpositionedplayervehicleradioemitterenabled_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021712_audradiostation_setfavourited_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021721_audradiostation_skiptooffsetinmusictracklist_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021727_audradiostation_forcetrack_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_021728_audradiostation_unlockmissionnewsstory_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021729_audradiostation_forcenexttrack_char_const_ptr_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_021731_audradiostation_queuetracklist_rage_radiostationtracklist_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_021732_audradiostation_clearqueuedtracks(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021733_audradiostation_getmusictracklisttrackforindex_unsigned_int_cons(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_021742_rage_audsound_getrequestedvolume_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_021761_audspeechaudioentity_saywhensafe_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_021763_audspeechaudioentity_getrandomvariation_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_021767_audscannermanager_triggerscriptedreport_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_021769_audspeechaudioentity_audspeechaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021776_audspeechaudioentity_init_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021784_audspeechaudioentity_stopambientspeech(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021792_audspeechaudioentity_ispreloadedspeechready(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021793_audspeechaudioentity_playpreloadedspeech_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021801_audscriptaudioentity_abortscriptedconversation_bool_char_const_p(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_021816_audscriptaudioentity_getindexfromscriptthreadid_rage_scrthreadid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_021818_audscriptaudioentity_triggermissioncomplete_char_const_ptr_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021819_audscriptaudioentity_isplayingmissioncomplete_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021820_audscriptaudioentity_setdesiredpedwalladensity_float_float_bool(ph: *mut Phantom, a0: f32, a1: f32, a2: i32); }
extern "C" { pub fn f_021821_audscriptaudioentity_addscript(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021822_audscriptaudioentity_handlescriptaudioevent_unsigned_int_centity(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_021827_audspeechaudioentity_processclipdictionaryresource_void_ptr_unsi(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021828_audscriptaudioentity_ispreloadedscriptedspeechready(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021829_audscriptaudioentity_startnewscriptedconversationbuffer(ph: *mut Phantom); }
extern "C" { pub fn f_021830_audscriptaudioentity_addlinetoscriptedconversation_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i32); }
extern "C" { pub fn f_021831_audscriptaudioentity_addconversationspeaker_unsigned_int_cped_pt(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_021832_audscriptaudioentity_setconversationcontrolledbyanimtriggers_boo(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021833_audscriptaudioentity_ispedincurrentconversation_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021834_audscriptaudioentity_setconversationplaceholder_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021835_audscriptaudioentity_startconversation_bool_bool_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_021837_audscriptaudioentity_setnoduckingforconversationfromscript_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021838_audscriptaudioentity_getvariationchosenforscriptedline_unsigned(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021839_audscriptaudioentity_setpositionfornullconvped_int_rage_vector3(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_021840_audscriptaudioentity_setentityfornullconvped_int_centity_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_021841_audscriptaudioentity_sayambientspeechfromposition_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_021847_audscriptaudioentity_registerscriptrequestedinterrupt_cped_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_021849_audscriptaudioentity_isscriptedconversationloaded(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021850_audscriptaudioentity_getcurrentscriptedconversationline(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021851_audscriptaudioentity_skiptonextscriptedconversationline(ph: *mut Phantom); }
extern "C" { pub fn f_021852_audscriptaudioentity_setscriptcleanuptime_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021854_audscriptaudioentity_requestscriptbank_char_const_ptr_bool_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_021859_audscriptaudioentity_hintscriptbank_char_const_ptr_bool_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021861_audscriptaudioentity_scriptbanknolongerneeded_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021862_audscriptaudioentity_scriptbanknolongerneeded(ph: *mut Phantom); }
extern "C" { pub fn f_021863_audscriptaudioentity_getsoundid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021864_audscriptaudioentity_releasesoundid_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021865_audscriptaudioentity_playsoundandpopulatescriptsoundslot_int_uns(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32, a9: i32, a10: i64, a11: i64) -> i64; }
extern "C" { pub fn f_021868_audscriptaudioentity_setambientzonenonpersistentstate_unsigned_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_021869_audscriptaudioentity_clearambientzonenonpersistentstate_unsigned(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_021870_audscriptaudioentity_setvehiclescriptpriorityforcurrentscript_au(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021871_audscriptaudioentity_setplayerangry_audspeechaudioentity_ptr_boo(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021872_audscriptaudioentity_playsoundfromentity_int_char_const_ptr_cent(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_021873_audscriptaudioentity_playsoundfromentity_int_unsigned_int_centit(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64); }
extern "C" { pub fn f_021874_audscriptaudioentity_playsoundfromposition_int_char_const_ptr_ra(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_021875_audscriptaudioentity_updatesoundposition_int_rage_vector3_const(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_021876_audscriptaudioentity_playsound_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_021877_audscriptaudioentity_playsoundfrontend_int_char_const_ptr_char_c(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_021878_audscriptaudioentity_stopsound_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021880_audscriptaudioentity_getnetworkidfromsoundid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021881_audscriptaudioentity_getsoundidfromnetworkid_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021882_audscriptaudioentity_setvariableonsound_int_char_const_ptr_float(ph: *mut Phantom, a0: i32, a1: i64, a2: f32); }
extern "C" { pub fn f_021883_audscriptaudioentity_setvariableonstream_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_021884_audscriptaudioentity_hassoundfinished_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021885_audscriptaudioentity_getscript_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_021886_audscriptaudioentity_startscene_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021887_audscript_startscene_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021888_audscriptaudioentity_stopscene_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021889_audscriptaudioentity_stopscenes(ph: *mut Phantom); }
extern "C" { pub fn f_021890_audscriptaudioentity_setscenevariable_char_const_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_021891_audscriptaudioentity_issceneactive_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021892_audscript_setaudioflag_audscriptaudioflags_flagid_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021893_audscriptaudioentity_enableaggressivehorns_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021894_audscriptaudioentity_dontabortcarconversations_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_021895_audweaponaudioentity_loaddynamicweaponaudio_audplayerweaponslot(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021896_audscriptaudioentity_releasescriptweaponaudio(ph: *mut Phantom); }
extern "C" { pub fn f_021897_audscriptaudioentity_requestscriptvehicleaudio_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021898_audscriptaudioentity_preloadstream_char_const_ptr_unsigned_int_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_021899_audscriptaudioentity_playstreamfromentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021900_audscriptaudioentity_playstreamfrontend(ph: *mut Phantom); }
extern "C" { pub fn f_021901_audscriptaudioentity_getstreamplaytime_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021902_audscriptaudioentity_isplayingstream(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021903_audscriptaudioentity_playstreamfromposition_rage_vector3_const_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021905_audscriptaudioentity_stoppedspeaking_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021906_audscriptaudioentity_blockallspeechfromped_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021907_audscriptaudioentity_stoppedspeakingsynced_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021908_audscriptaudioentity_stoppedpainaudio_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021914_audscript_setaudioflag_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_021915_audscript_activateslowmomode_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021916_audscript_deactivateslowmomode_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021920_audspeechmanager_getbestvoice_unsigned_int_unsigned_int_cped_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_021924_audspeechmanager_resolvevirtualspeechcontext_rage_speechcontext(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_021925_audspeechaudioentity_setpedgender_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021926_audspeechaudioentity_forcefullvoice_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021927_audspeechaudioentity_doescontextexistforthisped_char_const_ptr_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_021929_audspeechaudioentity_setpedvoiceforcontext_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_021932_audspeechmanager_addvariationtohistory_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_021935_audspeechmanager_getspeechcontext_unsigned_int_cped_ptr_cped_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_021938_audspeechaudioentity_isanimalvocalizationplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021939_audspeechaudioentity_maketrevorrage(ph: *mut Phantom); }
extern "C" { pub fn f_021940_audspeechaudioentity_overridetrevorragecontext_char_const_ptr_bo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021941_audspeechaudioentity_resettrevorragecontext_bool(ph: *mut Phantom); }
extern "C" { pub fn f_021942_audspeechaudioentity_addcontextblock_char_const_ptr_audcontextbl(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021943_audspeechaudioentity_removecontextblock_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021944_audspeechaudioentity_getactivespeechplaytimems_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_021945_audspeechaudioentity_getlastmetadatahash_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_021946_audspeechaudioentity_addreftoactivesoundslot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021947_audspeechaudioentity_getlastgesturedataforcategoryid_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_021949_audspeechaudioentity_say_unsigned_int_audspeechinitparams_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32, a8: f32, a9: i64, a10: i64, a11: i64) -> i32; }
extern "C" { pub fn f_021957_audspeechaudioentity_say_char_const_ptr_audspeechinitparams_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_021979_audtraileraudioentity_gettraileraudiosettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022004_rage_audsound_getrequestedpitch_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022014_audvehiclecollisioncontextlist_clearreuselist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022023_rage_fwentity_getskeletondata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022040_audvehiclecollisionaudio_getvehiclecollisionsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022042_audvehiclecollisionaudio_triggerdebrissounds_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_022046_audvehicleaudioentity_playhelddownhorn_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_022051_audvehicleaudioentity_arehydraulicsactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022058_audvehicleaudioentity_smoothsirenvolume_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_022073_audvehicleaudioentity_ishornon(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022077_rage_fwentity_getobjectmtx_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022090_audvehicleaudioentity_setloudradio_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022097_audvehicleaudioentity_blipsiren(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022104_audvehicleaudioentity_forceusegameobject_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022129_audvehiclecollisionaudio_triggerjumpland_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_022235_cpedinventory_getweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022280_audmusiceventmanager_triggerevent_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_022282_audmusiceventmanager_cancelevent_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_022283_audmusiceventmanager_prepareevent_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_022381_void_creplayeventmanager_storeevent_cpacketsoundphonering_cpacke(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022384_bool_creplaymgrinternal_handleentityforfx_cpacketpedstandingmate(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_022398_naaudioentity_createdeferredsound_unsigned_int_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_022401_audcachedlos_addprobe_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_022402_audcachedlos_getproberesult(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022403_audcachedlosoneresult_clear(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022416_rage_audsound_setrequestedvolume_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_022428_audboataudioentity_getboataudiosettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022612_audcollisionaudioentity_getmaterialoverride_centity_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_022615_audscene_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022616_audcollisionaudioentity_getaudioweight_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022621_auddynamicmixer_startscene_unsigned_int_audscene_ptr_ptr_audscri(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_022632_audcollisionaudioentity_getaudioweightscaling_rage_audioweight_r(ph: *mut Phantom, a0: i32, a1: i32) -> f32; }
extern "C" { pub fn f_022649_audcollisionaudioentity_reporthelibladecollision_worldprobe_csha(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022652_audcollisionaudioentity_computestartoffsetandvolumefromimpactmag(ph: *mut Phantom, a0: f32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_022654_audcollisionaudioentity_handleplayerevent_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022658_cped_isdead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022688_audcutsceneaudioentity_setvariableonsound_unsigned_int_float_int(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_022700_audcuttracksynchedscene_trigger_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022705_audcutsceneaudioentity_issyncscenetrackprepared_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022706_audcuttracksynchedscene_init_char_const_ptr_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_022709_audcutsceneaudioentity_triggersyncscene_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022714_audcutsceneaudioentity_stopoverunaudio(ph: *mut Phantom); }
extern "C" { pub fn f_022715_audcuttracksynchedscene_stop_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022719_audcutsceneaudioentity_getsyncscenetime_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_022728_audcuttracksynchedscene_setentity_unsigned_int_centity_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_022729_audcuttracksynchedscene_setposition_unsigned_int_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_022751_audscene_setvariablevalue_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_022754_audscene_isinstanceof_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_022761_auddynamicmixer_addentitytomixgroup_centity_ptr_unsigned_int_flo(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_022762_auddynamicmixer_removeentityfrommixgroup_centity_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_022792_audemitteraudioentity_unregisteremittersforentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022805_audambientaudioentity_isalarmactive_rage_alarmsettings_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022806_audambientaudioentity_registereffectaudio_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_022812_audemitteraudioentity_setemitterradiostation_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022813_audemitteraudioentity_setemitterlinkedobject_char_const_ptr_cent(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022814_audemitteraudioentity_setemitterenabled_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_022836_naenvironment_spikeresonanceobjects_float_rage_vector3_const_ref(ph: *mut Phantom, a0: f32, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_022837_audexplosionaudioentity_stopsmokegrenadeexplosions(ph: *mut Phantom); }
extern "C" { pub fn f_022852_audfrontendaudioentity_playsound_rage_atnonfinalhashstring_char(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022861_audfrontendaudioentity_triggerlongswitchskyloop_earcadeteam(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022862_audfrontendaudioentity_stoplongswitchskyloop(ph: *mut Phantom); }
extern "C" { pub fn f_022863_audfrontendaudioentity_triggerlongswitchsound_rage_atnonfinalhas(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022892_audglassaudioentity_clearall_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022893_audglassaudioentity_scripthandlebreakableglassevent_rage_vec3v_r(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_022901_audgunfightconductor_processsuperconductororder_conductors_condu(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022919_naenvironment_computeworldsectorindex_rage_vector3_const_ref_con(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_022985_cpathfind_findnodepointersafe_rage_cnodeaddress(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023028_audambientaudioentity_stopallalarms_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023033_audambientaudioentity_forceoceanshorelineupdatefromnearestroad(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023036_audambientaudioentity_overrideunderwaterstream_char_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023037_audambientaudioentity_setvariableonunderwaterstream_char_const_p(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_023041_naenvironment_findinteriorsettingsforroom_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_023050_audambientaudioentity_forcepedpanic(ph: *mut Phantom); }
extern "C" { pub fn f_023051_audambientaudioentity_preparealarm_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023052_audambientaudioentity_startalarm_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023053_audambientaudioentity_stopalarm_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023055_audambientaudioentity_setzonepersistentstatus_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_023170_audwarpmanager_havevehiclemodsloaded(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023174_naenvironment_purgeinteriors(ph: *mut Phantom); }
extern "C" { pub fn f_023176_naenvironment_processcachedworldsectorops(ph: *mut Phantom); }
extern "C" { pub fn f_023187_naenvironment_removetreefromworld_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023188_naenvironment_removebuildingfromworld_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023231_namicrophones_isvisiblebysniper_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023232_namicrophones_isfirstperson(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023309_naocclusionportalinfo_setportalsettingsscriptoverride_char_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023310_rage_atmapentry_unsigned_int_rage_atarray_naocclusionperportalse(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023311_naocclusionportalinfo_removeportalsettingsscriptoverride_char_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023396_cpacketevent_cpacketevent_ereplaypacketid_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023449_cpacketrayfirepreload_cpacketrayfirepreload_bool_unsigned_int_cc(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_023529_cpacketscriptaudiospecialeffectmode_cpacketscriptaudiospecialeff(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_023563_quantizeu8_float_float_float_int(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> i32; }
extern "C" { pub fn f_023565_quantizequaternions3_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023600_cpacketptexmtlbang_cpacketptexmtlbang_vfxcollinfo_s_const_ref_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_023700_replayfilemanager_getvideoprojectdirectory_char_ref_256_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023702_replayfilemanager_getclipdirectory_char_ref_256_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023706_replayfileinfo_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023708_rage_atarray_clipuid_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023710_fidevicereplay_removecachedproject_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023724_replayfilemanager_checkavailablediskspaceformontage_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_023728_replayfilemanager_isperformingfileop(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023729_replayfilemanager_deletefile_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023734_replayfilemanager_getclipuid_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023738_replayfilemanager_issaving_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023757_creplayinterface_cambasecamera_getcreatepacket_creplayid_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023846_creplaymgr_replayrecordsound_unsigned_int_rage_audsoundinitparam(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_023850_creplaymgr_replayrecordsoundpersistant_unsigned_int_rage_audsoun(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_023857_creplaymgr_recordentityattachment_creplayid_const_ref_creplayid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023861_replayattachmentmanager_adddetachment_creplayid_const_ref_crepla(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023868_creplaymgrinternal_setnextplaybackstate_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_023869_creplaymgr_settrafficlightcommands_centity_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023870_creplaymgr_recordmapobject_cobject_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023873_creplaymgr_shouldpreventmapchangeexecution_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023880_creplaymgr_shouldforcehideentity_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023881_creplaymgr_hash_unsigned_int_const_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023884_creplaymgr_preclothsimupdate_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023885_creplaymgr_recordwarpedentity_cdynamicentity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023886_creplaymgr_cancelrecordingwarpedonentity_cphysical_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023906_creplaycontrol_isplayeroutofcontrol(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023927_void_creplayeventmanager_trackrecordingevent_unsigned_long_ctrac(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_023930_creplayeventmanager_shouldproceedwithpacket_cpacketevent_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023936_replayentityextension_getextindex_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023945_replaypedextension_clearpeddecorationdata_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023967_creplayinterfacemanager_ensureisbeingrecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023968_creplayinterfacemanager_ensurewillberecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023997_rage_atarray_creplayinterfaceobject_deferredmapobjecttohide_0_un(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024023_rage_atarray_creplayinterfaceobject_counterpartlessobjecttoadd_0(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024199_rage_fwentity_getlocalmtxnonconst_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_024257_creplaymgrinternal_isusingrecordedcamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024266_creplaymgrinternal_isawareofworldforrecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024278_creplaymgrinternal_isreplaycursorjumping(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024286_creplaymgrinternal_isplaybackflagset_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_024298_creplaymgrinternal_addbuilding_cbuilding_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024303_creplaymgrinternal_ondeleteentity_cphysical_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024304_creplaymgrinternal_getbuildinginfo_cphysical_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_024305_creplaymgrinternal_recordmapobject_centity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024310_rage_atarray_sentityset_0_unsigned_short_op_rage_atarray_sentity(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024314_creplaymarkercontext_reset(ph: *mut Phantom); }
extern "C" { pub fn f_024324_replayrecordingscriptinterface_preventrecordingthisframe(ph: *mut Phantom); }
extern "C" { pub fn f_024344_rayfirereplayloadingdata_release_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_024348_rage_atfixedarray_rage_strrequest_20_dtoratfixedarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024350_replayrayfiremanager_load_packetrayfirepreloaddata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024354_replayhidemanager_addnewhide_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_024355_replayhidemanager_isnonreplaytrackedobjecttype_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026165_caminterface_isdominantrendereddirector_cambasedirector_const_re(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026166_caminterface_isrenderingdirector_cambasedirector_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026167_caminterface_isdominantrenderedcamera_cambasecamera_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026169_cammanager_isrenderingfirstpersoncamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026170_caminterface_isdominantrenderedcameraanyfirstpersoncamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026171_caminterface_setscriptedcameraisfirstpersonthisframe_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026173_camgameplaydirector_getthirdpersoncamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026174_camgameplaydirector_getfirstpersonshootercamera_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026176_caminterface_getplayercontrolcamaimframe(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026179_camcinematicdirector_isrenderingcinematicmountedcamera_bool_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026180_camcinematicdirector_isrenderingcinematicidlecamera_bool_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026181_caminterface_findfollowped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026183_caminterface_setdeathfaileffectstate_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026184_camcinematicdirector_invalidateidlecamera(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026186_camfactory_createobject_cambaseobjectmetadata_const_ref_rage_atn(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026187_cambasedirector_isshaking_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026190_caminterface_fadeout_int_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026193_caminterface_isfadingout(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026194_caminterface_isfadedout(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026195_caminterface_fadein_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026198_caminterface_isfadingin(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026199_caminterface_isfadedin(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026202_caminterface_isspherevisibleingameviewport_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_026205_cqueriableinterface_istaskcurrentlyrunning_int_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_026207_caminterface_computefocuspedonscreen_float_int_float_float_float(ph: *mut Phantom, a0: f32, a1: i32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32, a8: i32) -> i64; }
extern "C" { pub fn f_026209_rage_vector3_normalizesafe_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026211_worldprobe_cshapetestfixedresults_unsigned_char_32_cshapetestfix(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026212_caminterface_isrenderedcamerainsidevehicle(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026214_caminterface_disablenearclipscanthisupdate(ph: *mut Phantom); }
extern "C" { pub fn f_026218_caminterface_applypedboneoverridesforfirstperson_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026219_cdynamicentity_getboneindexfrombonetag_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026239_cambasecamera_shake_cambaseshakemetadata_const_ref_float_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_026241_camcinematicdirector_invalidatecinematicvehicleidlemode(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026242_camcinematicdirector_isrenderingcinematicpointofviewcamera_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026245_camcinematicdirector_getfirstpersonvehiclecamera_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026246_camcinematicdirector_getbonnetvehiclecamera_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026259_rage_fwentity_getskeleton_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026263_camcinematicdirector_findcontext_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_026267_cambasecinematiccontext_getcamera(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026268_camgameplaydirector_ishintactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026286_camgameplaydirector_getthirdpersonaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026288_camgameplaydirector_getfollowvehiclecamera_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026314_camgameplaydirector_getfollowcamera_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026318_cambasecamera_stopshaking_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026325_cambasecamera_isinterpolating_cambasecamera_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026329_worldprobe_cshapetestresults_dtorcshapetestresults(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026367_rage_imp_transform_imp34_rage_mat34v_ref_float_vector_4_float_ve(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: u128, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_026368_cpedintelligence_geteventoftype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_026419_cambasecinematiccontext_getcurrentshot_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026457_camcontrolhelper_getviewmodeforcontext_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_026461_camcinematicscriptcontext_deletecinematicshot_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_026463_camcinematicscriptcontext_isscriptcinematicshotactive_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026541_rage_atvector_rage_fwregdref_centity_const_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026582_cambasecamera_stopinterpolating(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026592_cambaseshakemetadata_const_ptr_camfactory_findobjectmetadata_cam(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026593_camanimatedshakemetadata_const_ptr_camfactory_findobjectmetadata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026594_camanimatedframeshaker_setanimation_char_const_ptr_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_026601_cambasedirector_stopshaking_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026605_camframeinterpolator_setcurvetypeforframecomponent_camframe_ecom(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_026607_cambasedirector_shake_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_026608_cambasedirector_shake_char_const_ptr_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_026609_cambasedirector_setshakeamplitude_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_026620_camscriptdirector_iscamerascriptcontrollable_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026623_rage_matrix34_toeulers_rage_vector3_ref_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026636_camcutscenedirector_iscutsceneplaying(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026648_camdebugdirector_getcamframe_cambasecamera_ptr_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026649_camdebugdirector_getcamframenonconst_cambasecamera_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026652_camdebugdirector_activatefreecamandcloneframe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026673_camgameplaydirector_registervehiclepartbroken_cvehicle_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_026678_camgameplaydirector_stopswitchbehaviour(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026687_camgameplaydirector_setscriptoverriddenfollowpedcamerasettingsth(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026688_camgameplaydirector_scriptrequestvehiclecamstuntsettingsthisupda(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026689_camgameplaydirector_scriptrequestdedicatedstuntcamerathisupdate(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026690_camgameplaydirector_islookingbehind_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026700_camgameplaydirector_getactiveviewmodecontext_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026702_ccontrol_debugoutputscriptcallstack(ph: *mut Phantom); }
extern "C" { pub fn f_026703_camgameplaydirector_getfirstpersonvehiclecamera_centity_const_pt(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026705_cped_getplayerresetflag_cplayerresetflags_playerresetflags_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026707_camgameplaydirector_starthinting_centity_const_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_026708_camgameplaydirector_stophinting_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026711_camgameplaydirector_getrelativecameraheading_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026712_camgameplaydirector_getrelativecamerapitch_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026713_camgameplaydirector_getfollowpedcamera_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026714_camgameplaydirector_getfirstpersonpedaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026715_camgameplaydirector_isfirstpersonshooteraiming_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026716_camgameplaydirector_setscriptoverriddenvehicleexitentrystate_cve(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_026719_cweaponinfo_const_ptr_cweaponinfomanager_getinfo_cweaponinfo_rag(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026720_camgameplaydirector_setscriptoverriddentablegamescamerasettingst(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026721_camgameplaydirector_scriptforcevehiclecamstuntsettingsthisupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026723_camgameplaydirector_startswitchbehaviour_int_bool_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: f32); }
extern "C" { pub fn f_026724_camgameplaydirector_isswitchbehaviourfinished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026726_cammanager_triggerfirstpersonflasheffect_cammanager_efirstperson(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_026727_camgameplaydirector_starthinting_rage_vector3_const_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_026728_camgameplaydirector_disablefirstpersonthisupdate_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_026753_cphysical_isasleep_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026778_rage_atarray_camthirdpersoncamera_tnearbyvehiclesettings_0_unsig(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026794_camthirdpersoncamera_bypassbuoyancythisupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027008_ctrain_getlinkedtobackward_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027009_ctrain_getlinkedtoforward_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027059_camcontrolhelper_setviewmodeforcontext_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_027060_camcontrolhelper_storecontextviewmodesinmap_rage_atbinarymap_rag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027062_camcontrolhelper_restorecontextviewmodesfrommap_rage_atbinarymap(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027064_camcontrolhelper_overridezoomfactorlimits_float_float(ph: *mut Phantom, a0: f32, a1: f32); }
extern "C" { pub fn f_027070_camframe_camframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027093_rage_matrix34_fromeulers_rage_vector3_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_027098_camsplinenode_computelookatmatrix_rage_matrix34_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027099_camsplinenode_camsplinenode_camframe_const_ref_camsplinenode_efl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_027179_camreplaydirector_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027189_camreplaydirector_getmaxdistanceallowedfromplayer_bool_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_027231_camscriptedcamera_clearlookattarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027237_cambasesplinecamera_getcurrentnode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_027264_camscriptdirector_stoprendering_bool_unsigned_int_bool_gtathread(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: f32, a6: i32); }
extern "C" { pub fn f_027265_camscriptdirector_forcestoprendering_bool_unsigned_int_bool_floa(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: f32, a5: i32); }
extern "C" { pub fn f_027267_camscriptdirector_activatecamera_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_027268_camscriptdirector_deactivatecamera_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_027269_camscriptdirector_iscameraactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_027270_camscriptdirector_setcameraframeparameters_int_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_027271_camscriptdirector_animatecamera_int_rage_atnonfinalhashstring_ra(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_027272_camscriptdirector_setcameraanimationphase_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_027273_camscriptdirector_getcameraanimationphase_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_027280_cviewportmanager_setwidescreenborders_bool_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_027282_camscriptdirector_debugsetscriptenabledshakename_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027283_camscriptdirector_debugsetscriptdisabledshake(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027286_camscriptedcamera_setattachbonetag_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_027287_camscriptedcamera_setattachoffset_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027288_camscriptedcamera_setattachrotationoffset_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027289_camscriptedcamera_setattachoffsetrelativetomatrixstate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_027290_camscriptedcamera_lookat_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027291_camscriptedcamera_lookat_cambasecamera_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027293_camscriptedcamera_setlookatbonetag_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_027294_camscriptedcamera_setlookatoffset_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027295_camscriptedcamera_setlookatoffsetrelativetomatrixstate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_027296_camscriptedcamera_setrolltarget_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027308_camswitchcamera_setestablishingshotmetadata_cplayerswitchestabli(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027309_camswitchcamera_debugoverridecameraheading_float_ref_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_027310_camswitchcamera_computebaseframe_camframe_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_027311_camswitchcamera_computeframeforascentcut_unsigned_int_camframe_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_027312_camswitchcamera_computeframefordescentcut_unsigned_int_camframe(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_027317_camswitchdirector_getswitchcamera(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028241_cammanager_debugregisterscriptcommand_char_const_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_028265_cviewportmanager_calcviewportforfragmanager(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_029075_stb_compress_dxt_block_unsigned_char_ptr_unsigned_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029078_rage_fwimage_t_rage_vector4_convertto_rage_grcimage_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029092_rage_fwtextureconversion_buildmipchain_rage_fwimage_t_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_029093_rage_fwtextureconversion_postprocessmip_rage_fwimage_t_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_029094_rage_computealphacoverage_rage_fwimage_t_rage_vector4_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i64) -> f32; }
extern "C" { pub fn f_029095_rage_fwtextureconversion_preconvert_rage_fwimage_t_rage_vector4(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_029096_rage_fwtextureconversionlog_write_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029101_cdebug_setversionnumber(ph: *mut Phantom); }
extern "C" { pub fn f_029102_capp_checkexit(ph: *mut Phantom); }
extern "C" { pub fn f_029237_cstreamingiteratormanager_finishandreset(ph: *mut Phantom); }
extern "C" { pub fn f_029324_cdumpgeometrytoobj_materialgroupend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029331_cdebugscene_drawentityboundingbox_centity_ptr_rage_color32_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_029335_cgamesessionstatemachine_reinit_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_029337_cgamesessionstatemachine_loadsavegame(ph: *mut Phantom); }
extern "C" { pub fn f_029338_cgamesessionstatemachine_handleloadedsavegame(ph: *mut Phantom); }
extern "C" { pub fn f_029339_cgamesessionstatemachine_setstatetoidle(ph: *mut Phantom); }
extern "C" { pub fn f_029377_ctask_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029386_cpedvariationinfocollection_getcollectiondrawable_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_029395_rage_phinst_setmatrix_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029397_cutscenemanager_getcamentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029433_getorcreategroup_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029453_cpedvariationinfocollection_getdlcnamefromdrawableidx_epedvarcom(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_029455_cdebugscene_tunewidgetdata_tunewidgetdata_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_029456_cdebugscene_queuetunewidget_cdebugscene_tunewidgetdata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029489_ccutscenecameraentity_getmaplodscale_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029512_cdebugscene_getworldpositionundermouse_rage_vector3_ref_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_029592_cutscenemanager_savegamedatacutfile(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029607_cpedvariationinfocollection_getracetype_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_029611_cdebugscene_getmousepointing_rage_vector3_ref_rage_vector3_ref_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029612_worldprobe_cshapetestdesc_setresultsstructure_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029613_worldprobe_cshapetestresults_freeintersections(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029643_cutscenemanager_getisolatedpedentity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029646_cutscenemanager_pushassetfolder_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029676_cutscenemanager_startmultiheadfade_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_029690_cdebugtextureviewer_clear_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_029691_cdebugtextureviewerwindow_dtorcdebugtextureviewerwindow(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029726_camframe_setworldmatrix_rage_matrix34_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029735_camframe_clonefrom_camframe_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029752_void_creplaymgrinternal_stoptrackingfx_unsigned_long_ctrackedeve(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029779_cblockview_getblockname_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029780_cblockview_getblockuser_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029786_cblockview_addblock_char_const_ptr_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_029808_cboundsdebug_drawstaticbounds_rage_strlocalindex_centity_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_029809_cdebugscene_getentityundermouse_rage_vector3_ptr_int_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_029860_colorizerutils_getpolycount_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029866_cdebugscene_focusentities_get_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029874_displayobject_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_029878_cdebug_dumpentity_centity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029900_cdebugarchetype_createdebugarchetypeproxies(ph: *mut Phantom); }
extern "C" { pub fn f_029903_cdebugarchetype_checkdebugarchetypeproxy_cbasemodelinfo_const_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029924_cdebugdrawstore_addline_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_029927_cdebugdrawstore_addsphere_rage_vec3v_const_ref_float_rage_color3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_029935_cdebugdrawstore_clearall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029939_cdumpgeometrytoobj_addquad_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_029940_cdumpgeometrytoobj_addcylinder_rage_vec3v_const_ref_rage_vec3v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_029941_cdumpgeometrytoobj_addboxedges_rage_vec3v_const_ref_rage_vec3v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_029951_cuigadgettext_setstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029962_cdebugscene_registerentitychangecallback_rage_datcallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029965_rage_fwtransform_getb_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029966_rage_fwtransform_getc_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029967_cphysical_gethealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029969_cdebugscene_focusentities_isingroup_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029986_cuigadgetlist_cuigadgetlist_float_float_float_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i64) -> i64; }
extern "C" { pub fn f_029987_cuigadgetwindow_cuigadgetwindow_float_float_float_float_cuicolor(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64) -> i64; }
extern "C" { pub fn f_029988_cuigadgetlist_setnumentries_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030052_cuigadgetbase_attachchild_cuigadgetbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030065_getshadersusedbyentity_rage_atarray_rage_grmshader_const_ptr_0_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_030082_setcamera_internal_debugtextureviewerdtq_edtqcubeface(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_030083_exportscenecubemap_rage_grcimage_const_ptr_rage_grcimage_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_030089_cmarketingtools_shoulddisablebloodforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030146_tiledscreencapture_isenabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_030184_assetanalysis_gettexturepath_rage_fwarchetype_const_ptr_rage_grc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_030193_getsimpletimestring(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030211_cstreamingiterator_rage_fwdwdstore_rage_pgdictionary_rage_rmcdra(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030212_rage_atarray_cstreamingiteratorslot_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030255_void_rage_atpagedpool_forall_clightentity_rage_fwdefaultrscmempa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_030272_cmapliveedit_start_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030273_cmapliveedit_rebuildguidmap(ph: *mut Phantom); }
extern "C" { pub fn f_030274_rage_atarray_centityeditset_0_unsigned_short_destruct_centityedi(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030307_rage_atarray_centityeditset_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030308_centityeditset_op_centityeditset_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030309_rage_atarray_centityadd_0_unsigned_short_op_rage_atarray_centity(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030345_cmapeditrestinterface_initifrequired(ph: *mut Phantom); }
extern "C" { pub fn f_030410_debuglighting_updatestatslodlights_unsigned_int_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_030415_cuicolorscheme_setallcolors_rage_color32_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030416_cuigadgetbase_propagateevent_cuievent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030419_cuigadgetbase_updateall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030421_cuigadgetbase_detachfromparent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030423_cuigadgetbox_cuigadgetbox_float_float_float_float_rage_color32_c(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64) -> i64; }
extern "C" { pub fn f_030449_cuigadgettext_cuigadgettext_float_float_rage_color32_const_ref_c(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_030465_cdebugtextureviewer_setupwindow_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030466_getassociatedtxds_modelinfo_rage_atarray_ctxdref_0_unsigned_shor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_030467_ctxdref_gettxd_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030468_istextureusedbymodel_rage_grctexture_const_ptr_char_const_ptr_cb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_030470_cdebugtextureviewer_setupwindowfinish(ph: *mut Phantom); }
extern "C" { pub fn f_030473_cdebugtextureviewer_populate_selecttxd_char_const_ptr_ctxdref_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030474_cassetref_addref_cassetrefinterface_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030475_cassetref_getdesc_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_030476_cassetref_getassetindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030477_cassetref_isvalidassetslot_eassettype_rage_strlocalindex(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_030478_cassetref_getstreamingassetsize_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_030484_cdtventry_modelinfo_cdtventry_modelinfo_char_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_030493_cdtventry_setupwindow_cdebugtextureviewerwindow_ptr_cdtvcolumnfl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030495_cassetref_getassetname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030496_rage_atarray_ctxdref_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030508_cdebugtextureviewer_edit_metadatainworkbench(ph: *mut Phantom); }
extern "C" { pub fn f_030513_rage_fwtextureconversionparams_dtorfwtextureconversionparams(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030525_cdebugtextureviewerinterface_selectmodelinfo_unsigned_int_centit(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_030526_cdebugtextureviewerinterface_selecttxd_ctxdref_const_ref_bool_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_030527_cdebugtextureviewerinterface_selectpreviewtexture_rage_grctextur(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_030528_getfriendlytexturename_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030558_gettexturesizeinbytes_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030565_ctxdref_ctxdref_eassettype_int_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_030568_cdebugtextureviewerwindow_getlistbounds_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030722_ceventgroupped_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030723_cevent_affectsped_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_030727_ceventgroupped_removeinvalidevents_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_030728_ceventgroupped_gethighestpriorityevent_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030732_ceventgroupvehicle_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030736_ceventgroupglobal_addorcombine_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030737_rage_fweventqueue_32_void_remove_rage_fwevent_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030739_cpedintelligence_addevent_cevent_const_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030741_geteventglobalgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030746_geteventscriptnetworkgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030752_ceventhandlerhistory_cleartempevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030753_ceventhandlerhistory_clearnontempevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030754_ceventhandlerhistory_clearmovementevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030755_ceventhandlerhistory_clearstoredevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030756_ceventhandlerhistory_getcurrentevent_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030757_ceventhandlerhistory_setcurrentevent_cevent_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_030758_ceventhandlerhistory_clearragdollresponse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030759_ceventresponse_cleareventresponse_ceventresponse_responsetasktyp(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030760_cpedintelligence_getactivemovementtask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030762_rage_fweventqueue_16_void_remove_rage_fwevent_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030763_ceventhandler_getcurrenteventtype_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030766_ceventnames_geteventname_eeventtype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_030770_cshockingeventsmanager_add_ceventshocking_ref_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_030772_ceventshocking_dtorceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030814_cpeddamagecalculator_cpeddamagecalculator_centity_const_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_030815_cpeddamagecalculator_applydamageandcomputeresponse_cped_ptr_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32); }
extern "C" { pub fn f_030819_cped_isinjured_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030824_cpedvariationinfocollection_hascomponentflagsset_epedvarcomp_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_030827_cpeddamagecalculator_wasdamagefromentitybeforewantedlevelcleared(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_030828_ceventdamage_ceventdamage_centity_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_030829_ceventdamage_dtorceventdamage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030845_ceventagitated_ceventagitated_cped_ptr_agitatedtype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_030846_ceventagitated_dtorceventagitated(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030848_ceventdeath_ceventdeath_bool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_030851_ceventdeath_dtorceventdeath(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030920_ceventnetworkplayerarrest_ceventnetworkplayerarrest_cped_ptr_cpe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_030945_svehiclemissionparams_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030974_cinformfriendsevent_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030978_cinformgroupevent_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030995_ceventvehiclecollision_ceventvehiclecollision_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32, a6: f32) -> i64; }
extern "C" { pub fn f_030996_ceventvehiclecollision_dtorceventvehiclecollision(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031055_ceventscriptcommand_ceventscriptcommand_int_rage_aitask_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_031056_ceventscriptcommand_dtorceventscriptcommand(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031069_ceventgivepedtask_ceventgivepedtask_int_rage_aitask_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_031070_ceventgivepedtask_dtorceventgivepedtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031097_ceventcallforcover_ceventcallforcover_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031098_ceventcallforcover_dtorceventcallforcover(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031143_ceventcopcarbeingstolen_ceventcopcarbeingstolen_cped_ptr_cvehicl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031144_ceventcopcarbeingstolen_dtorceventcopcarbeingstolen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031158_ceventfirenearby_dtorceventfirenearby(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031169_ceventswitch2nm_ceventswitch2nm_unsigned_int_rage_aitask_ptr_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_031170_ceventswitch2nm_dtorceventswitch2nm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031181_ceventsuspiciousactivity_ceventsuspiciousactivity_cped_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_031182_ceventsuspiciousactivity_dtorceventsuspiciousactivity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031226_cevententitydamaged_cevententitydamaged_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031228_cevententitydestroyed_cevententitydestroyed_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031235_ceventshocking_ceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031242_ceventshocking_set_rage_vec3v_const_ref_centity_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_031364_ceventfootstepheard_ceventfootstepheard_cped_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_031662_rage_fweventqueue_16_void_dtorfweventqueue(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031880_ceventsoundbase_dtorceventsoundbase_54685(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032070_rage_fweventqueue_16_void_flushexpiredevents(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033790_gfxvalue_objectinterface_objectrelease_gfxvalue_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034930_cloadingscreens_committompsp_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034937_cloadingscreens_init_loadingscreencontext_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_034943_cloadingscreens_update(ph: *mut Phantom); }
extern "C" { pub fn f_034944_cloadingscreens_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_034946_cloadingscreens_areactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034949_chudtools_getwidescreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034951_cloadingscreens_isdisplayinglegalscreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034959_cloadingscreens_suspend(ph: *mut Phantom); }
extern "C" { pub fn f_034960_cloadingscreens_resume(ph: *mut Phantom); }
extern "C" { pub fn f_035031_cnewhud_handlecharacterchange(ph: *mut Phantom); }
extern "C" { pub fn f_035046_cnewhud_settoggleabilitybar_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035082_cnewhud_isweaponwheelactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035083_cnewhud_isradiowheelactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035277_cminimap_createsonarblipandreportstealthnoise_cped_ptr_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32) -> f32; }
extern "C" { pub fn f_035326_cminimap_setvisible_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035327_cminimap_removeblipattachedtoentity_centity_const_ptr_eblip_type(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035352_cminimap_addorremovepointofinterest_float_float(ph: *mut Phantom, a0: f32, a1: f32); }
extern "C" { pub fn f_035353_cminimap_getpointofinterestdetails_int_rage_vector2_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_035376_cshaderlib_lookuptexture_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036044_cscaleformmgr_beginmethodinternal_int_escaleform_base_class_char(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_036045_cscaleformmgr_endmethodinternal_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036061_cscaleformmgr_createmovie_char_const_ptr_rage_vector2_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_036154_cpausemenu_close(ph: *mut Phantom); }
extern "C" { pub fn f_036162_cscaleformmoviewrapper_param_addparam(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036170_cpausemenu_setmenupreference_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036171_suicontexts_activate_rage_athashvalue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036172_suicontexts_deactivate_rage_athashvalue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036176_cpausemenu_redrawinstructionalbuttons_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036233_cwarningscreen_setmessageinternal_ewarningmessagestyle_rage_atno(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i64, a8: i64, a9: i32, a10: i32, a11: i32); }
extern "C" { pub fn f_036254_cpausemenu_isactive_pm_activeflags(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036272_cpausemenu_lockmenucontrol_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036275_cpausemenu_resetmenuitemtriggered(ph: *mut Phantom); }
extern "C" { pub fn f_036292_suicontexts_isactive_rage_athashvalue(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036295_cpausemenu_checkinput_efrontend_input_bool_unsigned_int_bool_boo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_036301_cwarningscreen_checkallinput_bool_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_036309_cscaleformmenuhelper_display_data_slot_pm_columns(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036310_cpausemenu_menu_shift_depth_emenuceptiondir_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_036311_cscaleformmenuhelper_set_column_highlight_pm_columns_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_036312_cscaleformmenuhelper_set_column_focus_pm_columns_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036314_cwarningscreen_setmessagewithheader_ewarningmessagestyle_rage_at(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_036323_cpausemenu_unlockmenucontrol(ph: *mut Phantom); }
extern "C" { pub fn f_036338_cpausemenu_getcolumnforspinner(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036339_cpausemenu_setbusyspinner_bool_signed_char_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_036341_cscaleformmenuhelper_show_warning_message_pm_columns_int_char_co(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i32, a8: i64, a9: i32); }
extern "C" { pub fn f_036342_cpausemenu_insertsavegameintomenu_int_char_ptr_char_ptr_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_036376_cpausemenu_isinsavegameorloadgamemenus(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036381_cpausemenu_backoutofloadgamepanes(ph: *mut Phantom); }
extern "C" { pub fn f_036386_cprofilesettings_write_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036387_cpausemenu_abandonloadingofsavedgame(ph: *mut Phantom); }
extern "C" { pub fn f_036392_cpausemenu_setnovalidsavegamefiles_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036474_cscaleformmenuhelper_set_warning_message_visibility_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036627_cwarningscreen_setmessage_ewarningmessagestyle_rage_atnonfinalha(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_036633_cwarningscreen_setwarningmessageinuse(ph: *mut Phantom); }
extern "C" { pub fn f_036637_cwarningscreen_update(ph: *mut Phantom); }
extern "C" { pub fn f_037236_ctexttemplate_rendersnapmatic(ph: *mut Phantom); }
extern "C" { pub fn f_037271_cscaleformmgr_loadpreallocationinfo_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037503_cgamestream_forceflushqueue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037601_cappdatamgr_deleteclouddata_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_037606_cappdatamgr_getappdata_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037616_rage_atstring_ref_rage_atmap_rage_atstring_rage_atstring_rage_at(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_037617_cappdatablock_clearmembers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037621_cappdata_getblock(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037622_cappdata_setblock_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037623_cappdata_closeblock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037628_cappdatamgr_pushfile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037629_cappdatamgr_islinkedtosocialclub(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037657_cwanted_setwantedlevel_rage_vector3_const_ref_int_int_setwantedl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_037660_cmodelinfo_getassetstreamflags_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037661_cmodelinfo_requestassets_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037662_cmodelinfo_haveassetsloaded_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037663_cmodelinfo_setassetsaredeletable_rage_fwmodelid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037676_cclock_settime_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_037682_cclock_setdate_int_date_month_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_037761_ccrime_reportcrime_ecrimetype_centity_ptr_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_037764_cwanted_workoutpolicepresence_rage_vec3v_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_037791_cmodelindex_matchallmodelstrings(ph: *mut Phantom); }
extern "C" { pub fn f_037810_cplacename_process(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037815_ccurrentvehicle_display(ph: *mut Phantom); }
extern "C" { pub fn f_037880_cwanted_initialise(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037888_cseatmanager_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_037889_cwanted_update_rage_vector3_const_ref_cvehicle_const_ptr_bool_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_037949_cweather_gettemperature_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_037959_cweather_processentitywinddisturbance_centity_ptr_cwinddisturban(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038130_cmodelinfo_getbasemodelinfo_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038190_cdispatchmanager_flush(ph: *mut Phantom); }
extern "C" { pub fn f_038222_corder_getisvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038334_cwantedincident_findnearbyincidents_float_cwantedincident_ptr_pt(ph: *mut Phantom, a0: f32, a1: i64) -> i32; }
extern "C" { pub fn f_038335_cwantedincident_getwantedlevel_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038464_cbaseikmanager_refreshkinematics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038467_cbaseikmanager_getsolver_ikmanagersolvertypes_flags_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038476_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_cbaseik(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_038477_cbaseikmanager_removecompletedsolvers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038478_cikmanager_dtorcikmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038480_cikmanager_init_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038490_cikmanager_resetallsolvers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038491_cikmanager_isactive_ikmanagersolvertypes_flags_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038495_cikmanager_lookat_unsigned_int_centity_const_ptr_int_eanimboneta(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32); }
extern "C" { pub fn f_038503_cikmanager_istargetwithinfov_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_038504_cikmanager_request_cikrequestarm_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038505_cikmanager_request_cikrequestbodylook_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038507_cikmanager_setrelativearmiktarget_rage_criksolverarms_earms_cdyn(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32, a6: f32, a7: f32, a8: f32, a9: f32); }
extern "C" { pub fn f_038527_cikmanager_getdefaultlegikmode_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038530_cikmanager_request_cikrequestleg_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038539_cikrequestbodylook_settargetoffset_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038541_cikrequestarm_settargetoffset_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038624_cbasemodelinfo_get2deffects_rage_atarray_c2deffect_ptr_0_unsigne(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038632_cbasemodelinfo_has2deffects_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038635_cbasemodelinfo_checkforhdtxdslots(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038636_cbasemodelinfo_getassetlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038640_cbasemodelinfo_createphysicsarchetype(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038641_cbasemodelinfo_addreftodrawablephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038649_cmlomodelinfo_setaudiosettings_unsigned_int_rage_interiorsetting(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_038650_cmlomodelinfo_getroomflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038651_cmlomodelinfo_getportalflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038652_cmlomodelinfo_getportaldata_unsigned_int_unsigned_int_ref_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_038653_cmlomodelinfo_getroomname_rage_fwinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_038706_cmodelinfo_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_038707_cmodelinfo_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_038711_cmodelinfo_addbasemodel_char_const_ptr_unsigned_int_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038713_cmodelinfo_areassetsloading_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038714_cmodelinfo_areassetsrequested_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038715_cmodelinfo_requestassets_cbasemodelinfo_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038716_cmodelinfo_getassetsaredeletable_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038719_cmodelinfo_setassetrequiredflag_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038720_cmodelinfo_clearassetrequiredflag_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038722_cmodelinfo_getobjectanddependenciessizes_rage_fwmodelid_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_038723_cmodelinfo_postload_rage_fwarchetype_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038733_centryexitpoint_getseat_seataccess_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_038735_cmodelseatinfo_calculateentrysituation_centity_const_ptr_cped_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32, a7: i64); }
extern "C" { pub fn f_038737_cseataccessor_getseatbyindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038739_cmodelseatinfo_getentrypointindexforseat_int_centity_const_ptr_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_038740_cmodelseatinfo_getboneindexfromseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038744_cmodelseatinfo_calculateseatsituation_centity_const_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_038751_cseatmanager_getpedsseatindex_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038754_cmodelseatinfo_ispositionclearforped_cped_const_ptr_centity_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_038757_cseatmanager_getlastpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038834_cpedmodelinfo_personalitymovementmodes_movementmode_clipsets_isv(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038835_cpedmodelinfo_personalitymovementmodes_movementmode_clipsets_get(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038836_cpedmodelinfo_personalitymovementmodes_findmovementmode_cped_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_038837_cpedmodelinfo_findpersonalitymovementmodes_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_038848_cpedmodelinfo_removepedmetadatafile_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038851_cpedmodelinfo_isusingfemaleskeleton_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038858_cpedmodelinfo_loadpedmetafile_char_const_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_038861_cpedmodelinfo_getquadrupedprobetopposition_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038863_cpedmodelinfo_getpersonalitybravery_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_038866_cpedmodelinfo_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038873_cpedmodelinfo_getarehdfilesloaded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038875_cpedmodelinfo_getpedinstance_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038919_rage_atarray_cvehiclemodelcolorgen9_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038924_cvehiclemodelinfo_loadvehiclemetafile_char_const_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_038938_cvehiclemodelcolorgen9_op_cvehiclemodelcolorgen9_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038973_cvehiclemodelinfo_gethdmemoryusage_unsigned_int_ref_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038992_cvehiclemodelinfo_gethdfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039020_cvehiclemodelinfo_addtohdinstancelist_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039021_cvehiclemodelinfo_removefromhdinstancelist_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039026_cvehiclemodelinfo_getcanpassengersbeknockedoff(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039027_cvehiclemodelinfo_releasedials(ph: *mut Phantom); }
extern "C" { pub fn f_039231_cvehiclevariationinstance_havemodsstreamedin_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039267_cweaponmodelinfo_setuphdfiles_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039275_cweaponmodelinfo_addtohdinstancelist_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039276_cweaponmodelinfo_removefromhdinstancelist_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039322_rage_atfixedarray_rage_strrequest_12_dtoratfixedarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039535_cpedmodelinfo_personalitymovementmodes_movementmode_clipsets_op(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039546_ccovertuningmanager_load_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039547_rage_atmapentry_rage_atnonfinalhashstring_int_const_ref_rage_atm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_039566_cobject_clearrelateddummy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039569_cdoor_getdooraudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039573_cpathservergta_updatedynamicobject_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_039576_cdoor_getdooropenratio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_039578_cdoor_isdoorshut_float_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_039587_cdoorsystem_generatehash32_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039591_cdoorsystem_setpendingstateondoor_cdoorsystemdata_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039594_cdoor_scriptdoorcontrol_bool_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_039617_rage_phcontactconstiterator_skiptonextactivecontact_rage_phmanif(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_039630_cweapon_sfireparams_dtorsfireparams(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039635_cdoor_getdoorocclusion_float_float_float_float_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_039636_cdoor_isdoorshut_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039639_cpathservergta_maybeadddynamicobject_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039646_cdoor_adddoortolist_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039650_rage_atmapentry_unsigned_int_cdoorsystemdata_ptr_ref_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039651_cdoorsystem_addsaveddoor_int_int_doorscriptstate_rage_vector3_co(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_039653_cdoorsystem_swapmodel_unsigned_int_unsigned_int_rage_vector3_ref(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: f32); }
extern "C" { pub fn f_039654_cdoorsystem_getdoordata_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_039688_cobjectpopulation_destroyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039692_cdummyobject_enable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039732_cobject_canbeusedtotakecoverbehind(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039751_cobjectintelligence_processpostmovement(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039757_cobjectpopulation_converttodummyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039758_cobjectpopulation_manageobjectpopulation_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_039759_cobjectpopulation_managedummy_cdummyobject_ptr_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_039761_cobjectpopulation_createobject_rage_fwmodelid_eentityownedby_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_039762_cobjectpopulation_createobject_cobjectpopulation_createobjectinp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039763_cnavmeshdataexporter_willexportcollision(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_039764_cobjectpopulation_deletealltempobjectsinarea_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_039766_cobjectpopulation_convertallobjectsinareatodummyobjects_rage_vec(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_039767_cobjectpopulation_flushallsuppresseddummyobjects(ph: *mut Phantom); }
extern "C" { pub fn f_039870_cpathserverthread_close(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039871_cpathserver_unloadallmeshes_bool(ph: *mut Phantom); }
extern "C" { pub fn f_039872_cpathserver_reset_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_039876_cpathserver_process_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039877_cpathserver_isrequestresultready_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_039882_cpathserver_getclosestpositionforped_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_039895_cpathserver_requestlineofsight_rage_vector3_const_ptr_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64, a8: f32) -> i32; }
extern "C" { pub fn f_039904_cpathserver_getlineofsightresult_unsigned_int_bool_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_039913_cpathserver_cancelrequest_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_039985_cpathserver_testshortlineofsightimmediate_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_039986_cpathserver_testshortlineofsightimmediate_rg_rage_vector3_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_039987_rage_cnavmeshtrackedobject_teleport_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039990_cpathserver_updatetrackedobject_rage_cnavmeshtrackedobject_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_039996_cpathservergta_mayberemovedynamicobject_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040024_rage_normalizeandmag_rage_vector3_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_040051_cpathserver_invalidatetrackers_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_040142_cpedgroup_islocallycontrolled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040143_cpedgroup_setdirty(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040165_cpedgroupmembership_flush_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_040169_cpedgroupmembership_removemember_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040170_cpedgroupmembership_setmaxseparation_float_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_040173_cpedgroupmembership_isfollower_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040175_cpedgroupmembership_isleader_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040176_cpedgroupmembership_copynetworkinfo_cpedgroupmembership_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040182_cpedgroup_process(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040189_cpedgroups_addgroup_epoptype_cgamescripthandler_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_040192_cpedgroups_init(ph: *mut Phantom); }
extern "C" { pub fn f_040194_cpedgroups_getpedsgroup_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040204_carea_carea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040205_carea_dtorcarea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040208_carea_get_rage_vector3_ref_rage_vector3_ref_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_040209_carea_ispointinarea_rage_vector3_const_ref_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_040212_carea_getcentreandmaxradius_rage_vector3_ref_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040220_worldprobe_cshapetesthitpoint_gethitinst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040233_cassistedmovementroutestore_maybeaddroutefordoor_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040235_cassistedmovementroutestore_mayberemoveroutefordoor_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040367_cbrawlingstylemanager_init_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040368_cbrawlingstylemanager_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_040384_ccombatbehaviour_initfromcombatdata_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040387_ccombatbehaviour_getdefaultcombatfloat_cped_const_ref_combatfloa(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_040403_cdefensiveareamanager_getdefensiveareaforcoversearch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040404_rage_fraginstnm_postartmessage_char_const_ptr_art_messageparamsb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040405_rage_fraginstnm_setcomponenttmsfromskeleton_rage_crskeleton_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040407_rage_fraginstnm_setweaponmode_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_040409_art_messageparamsbase_addbool_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040412_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_040415_art_nmrscharacter_calculatecomvalues(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040416_art_nmrscharacter_twistcompensation_art_nmrsgenericpart_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32) -> f32; }
extern "C" { pub fn f_040425_rage_fraginstnm_mapragdolllodcomponenthightocurrent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_040500_art_messageparamsbase_addint_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040501_art_messageparamsbase_addvector3_char_const_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_040503_art_messageparamsbase_messageparamsbase_art_messageparamsbase_pa(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_040511_art_nmrseffectorbase_getjointposition_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040514_art_nmrs1dofeffector_applytorque_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_040534_art_softlimitcontroller_tick_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040545_art_nmrs3dofeffector_applytorque_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040564_art_nmrsgenericpart_getarticulatedmatrix_rage_matrix34_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040569_art_nmrsgenericpart_applyforce_rage_vector3_const_ref_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040572_art_nmrsgenericpart_applytorque_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040586_art_nmrscharacter_applyuprightconstraintforces_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32); }
extern "C" { pub fn f_040587_art_nmrscharacter_stayuprightbycomtorques_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_040588_art_nmrsbody_postlimbinputs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040599_art_nmrsgenericpart_setmatrix_rage_matrix34_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040600_art_nmrsgenericpart_applyvelocitiestopart_rage_matrix34_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_041033_cpedclothcollision_getnextpedclothcollision_rage_crskeleton_cons(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041035_cpedclothcollision_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041056_findplayerped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041060_cpeddebugvisualisermenu_getfocusped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041062_cpedintelligence_findmovementtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041063_cpedintelligence_isfriendlywith_cped_const_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041065_cplayerpedtargeting_tunables_gettargettinginfo_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041066_cpedtargetevaluator_canbetargetedinrejectiondistance_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_041083_cpedintelligence_getcombatorders_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041113_cpedintelligence_getlowestlevelnmtask_cped_ptr_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041116_cpedintelligence_flushimmediately_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041117_cpedintelligence_addtoeventhistory_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041119_cpedtargetting_dtorcpedtargetting(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041120_cpedscriptedtaskrecord_process(ph: *mut Phantom); }
extern "C" { pub fn f_041121_cpedintelligence_buildqueriablestate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041122_svehiclemissionparams_dtorsvehiclemissionparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041186_cpedpopulation_removeallrandompeds(ph: *mut Phantom); }
extern "C" { pub fn f_041197_cpedintelligence_setdefaultrelationshipgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041201_cpedintelligence_flushevents(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041202_cpedintelligence_setrelationshipgroup_crelationshipgroup_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041203_cpedintelligence_getrelationshipgroup_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041210_cpedintelligence_getmoveblendratiofromgototask_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041217_cpedeventdecisionmaker_makedecision_cped_const_ptr_ceventeditabl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_041218_cplayerinfo_getphysicalplayerindex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041219_cpedgeometryanalyser_canpedtargetpedasync_cped_ref_cped_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: i32, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_041220_ccantargetcacheentry_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041226_rage_syslinkeddata_unsigned_int_refreportplayerresetmismatch_uns(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041237_cpedintelligence_gettaskmelee_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041241_ccollisioneventscanner_processragdollimpact_cped_ptr_float_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64, a5: i32, a6: i64); }
extern "C" { pub fn f_041244_cplayerspecialabilitymanager_chargeevent_abilitychargeeventtype(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_041245_ceventdamage_setphysicalresponsetask_rage_aitask_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041250_ceventscanner_dtorceventscanner(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041254_cinwatereventscanner_isdrowninginvehicle_cped_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041255_cinwatereventscanner_candrown_cped_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041256_ceventscanner_scanforevents_cped_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041257_cped_setplayerresetflag_cplayerresetflags_playerresetflags(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041258_cpedintelligence_clearprimarytask(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041260_cplayerinfo_issoftaiming_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041261_cplayerinfo_ishardaiming(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041269_cpedintelligence_gettaskclimbladder_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041271_cpoolhelpers_getpedpoolsize(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041275_cpedfactory_destroypedinternal_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041279_cpedfactory_logdestroyedped_cped_ptr_char_const_ptr_int_rage_col(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_041287_cpedpopulation_removepedfrompopulationcount_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041291_cplayerinfo_setplayerped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041293_cplayerspecialability_deactivate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041295_cplayerinfo_setupplayergroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041296_cpedfactory_destroyped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041298_cpedhelmetcomponent_disablehelmet_bool_rage_vector3_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_041299_cpedhelmetcomponent_restoreoldhat_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041300_cpedresetflags_resetpreaipostinventory(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041304_cpedfootstephelper_computefootinfo_feettags_rage_mat34v_ref_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32); }
extern "C" { pub fn f_041311_cpedfootstephelper_resetairflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041314_cpedfootstephelper_iswalkingonpuddle(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041329_cpedgeometryanalyser_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041337_cpedhelmetcomponent_ishelmetenabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041344_cpedhelmetcomponent_setstoredhatindices_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041346_cpedhelmetcomponent_sethelmettexindex_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041347_cpedhelmetcomponent_sethelmetpropindex_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041374_rage_fwregdref_cscenariopoint_rage_fwrefawarebasenonvirt_dtorfwr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041375_ceventresponse_dtorceventresponse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041376_cpedintelligence_seventhistoryentry_dtorseventhistoryentry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041377_cpedintelligence_setdecisionmakerid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041379_cpedtargetting_updatetargetting_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041385_cpedintelligence_findtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041386_cpedintelligence_findtaskcombat_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041387_cpedintelligence_gettaskmeleeactionresult_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041390_cpedintelligence_ispedvaulting_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041391_cpedintelligence_ispedfalling_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041400_cpedintelligence_process_nearbyentitylists(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041401_cpedintelligence_process_tasks_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_041402_cpedintelligence_processphysics_float_int_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_041422_cpedailodmanager_shoulddofullupdate_cped_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041423_cpedintelligence_increasebattleawareness_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_041424_cpedintelligence_resetbattleawareness(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041425_cpedintelligence_isbattleawareforcingrun_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041426_cpedintelligence_isbattleaware_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041428_cpedintelligence_getlastbattleeventtime_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041432_cpedintelligence_increaseamountpinneddown_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_041439_cpedmotivation_setmotivationprofile_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041440_cpedintelligence_setpedperception_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041446_cpedintelligence_bumped_centity_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_041450_cpedintelligence_seteventhistoryentrystate_cevent_const_ref_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041451_cpedintelligence_eventhistoryoneventremoved_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041454_cpedmotiondata_setcurrentmoveblendratio_rage_vector2_ref_float_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_041458_cpedphonecomponent_getphonemodelhashsafe_cpedphonecomponent_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041460_cpedplacement_findzcoorforped_float_rage_vector3_ptr_centity_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_041487_cpedpopulation_setscriptcampopulationoriginoverride(ph: *mut Phantom); }
extern "C" { pub fn f_041496_findplayervehicle_cplayerinfo_ptr_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041498_cpedpopulation_canpedbereused_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041499_cpedpopulation_addpedtoreusepool_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041510_cpedpopulation_forcepopulationinit_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041529_cpedpopulation_addpedtopopulationcount_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041537_cpopulationgroup_isindexmember_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041558_cpedpopulation_instantfillpopulation(ph: *mut Phantom); }
extern "C" { pub fn f_041563_cpedpopulation_removescenariopointsfromentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041585_cplayerspecialability_updatefromstat(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041587_cplayerspecialability_process_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041589_cexplosionmanager_cexplosionargs_dtorcexplosionargs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041593_cplayerspecialabilitymanager_updatedlcmultipliers(ph: *mut Phantom); }
extern "C" { pub fn f_041606_cplayerspecialabilitymanager_load_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041607_cplayerspecialabilitymanager_destroy_cplayerspecialability_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041609_cplayerspecialabilitymanager_processcontrol_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041626_cpedscriptedtaskrecord_getrecordassociatedwithevent_ceventscript(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041652_cplayerinfo_getplayerindex_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041653_cplayerinfo_getmaxtimeunderwaterforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041658_cplayerinfo_setinitialstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041664_cplayerinfo_issprintaimbreakoutover_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041665_cplayerinfo_decaystealthnoise_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_041666_cplayerinfo_isrunandgunning_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_041667_cplayerinfo_isassistedaiming(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041668_cplayerpedtargeting_findlockontarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041669_cplayerpedtargeting_findmeleetarget_cped_const_ptr_cweaponinfo_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64) -> i64; }
extern "C" { pub fn f_041670_cplayerpedtargeting_shouldupdatetargeting_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041671_cplayerpedtargeting_gettargetingmode_cweapon_const_ptr_cweaponin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_041672_cplayerpedtargeting_updatefreeaim_cweaponinfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041673_cplayerpedtargeting_clearfreeaim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041674_cplayerpedtargeting_clearlockontarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041675_cplayerpedtargeting_processpedevents_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041676_cplayerpedtargeting_getisfreeaimassisttargetposwithinconstraints(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_041677_cplayerinfo_handlesprintenergy_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_041678_cplayerinfo_getfiringattackvalue(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041680_cplayerinfo_processfpsstate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041690_cplayerinfo_processlighteffects(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041701_canimspeedups_shouldusempanimrates(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041702_findfollowped_cplayerinfo_ptr(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041705_cplayerinfo_setplayerstate_cplayerinfo_state(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041706_cplayerinfo_setplayercontrol_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_041708_cplayerinfo_setplayercontrol_bool_bool_float_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_041734_cplayerpedtargeting_gettarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041741_cplayerinfo_getmpspecialability_eplayerspecialabilityslot_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041746_reportplayerresetmismatch_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_041809_cpopzones_findsmallestforposition_rage_vector3_const_ptr_ezoneca(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_041818_cpopcycle_getcurrentzonepeddirtmax(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_041819_cpopcycle_getcurrentzonedirtcol(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041830_rage_atarray_cpopzone_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041831_rage_atarray_cpopzonesearchinfo_0_unsigned_short_grow_unsigned_i(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041861_cpedmotivationinfomanager_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_041865_spedaccuracymodifiers_init(ph: *mut Phantom); }
extern "C" { pub fn f_041872_cpedtargetevaluator_findtarget_cpedtargetevaluator_tfindtargetpa(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041876_cpedtargetevaluator_getislineofsightblocked_cped_const_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_041879_cpedtargetevaluator_istargetvalid_rage_vec3v_const_ref_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: f32, a11: f32, a12: i32, a13: i32, a14: i32, a15: i32) -> i32; }
extern "C" { pub fn f_041882_cpedtargetevaluator_getcanpedbetargetedinavehicle_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041922_cplayerpedtargeting_canpedlockonwithweapon_cweapon_const_ptr_cwe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_041923_cplayerpedtargeting_cannotlockon_cweapon_const_ptr_cweaponinfo_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_041924_cplayerpedtargeting_findnextlockontarget_centity_ptr_cplayerpedt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_041925_cplayerpedtargeting_findandsetlockontarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041926_cplayerpedtargeting_updatefineaim_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041927_cplayerpedtargeting_setlockontarget_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041997_cqueriableinterface_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041998_cqueriableinterface_gettargetfortasktype_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042003_cqueriableinterface_gettaskinfofortasktype_int_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042004_cqueriableinterface_getstatefortasktype_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042005_cqueriableinterface_getrunningscenariotype_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042011_cqueriableinterface_addtaskinfofromnetwork_ctaskinfo_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042012_cqueriableinterface_unlinktaskinfofromnetwork_ctaskinfo_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042036_crelationshipmanager_addrelationshipgroup_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042039_crelationshipmanager_removerelationshipgroup_crelationshipgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042052_cwildlifemanager_dispatchasharknearplayer_bool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042104_cpeddamagesetbase_allocatetextures(ph: *mut Phantom); }
extern "C" { pub fn f_042113_rage_atvector_cpeddamageblitdecoration_0_unsigned_short_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042136_rage_atvector_cpeddamagecompressedblitdecoration_0_unsigned_shor(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042161_cpeddamagedata_cleararrays(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042166_cpeddamagemanager_updatepriority_unsigned_char_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_042175_cpeddamagemanager_addpeddecoration_cped_ptr_rage_atnonfinalhashs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: f32, a7: i64, a8: i32, a9: i64, a10: i64, a11: i32, a12: f32, a13: i32, a14: i32, a15: i64, a16: f32, a17: f32, a18: f32, a19: i32, a20: i32); }
extern "C" { pub fn f_042181_cpeddamagemanager_releasedamageset_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042182_cpeddamagemanager_releasecompresseddamageset_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042185_cpeddamagemanager_testupdatedecorations(ph: *mut Phantom); }
extern "C" { pub fn f_042189_cpeddamagemanager_clearpedblood_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042191_cpeddamagemanager_promotebloodtoscars_unsigned_char_bool_float_f(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_042192_cpeddamagemanager_limitzoneblood_unsigned_char_epeddamagezones_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_042193_cpeddamagemanager_cleardamagedecals_unsigned_char_epeddamagezone(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_042194_cpeddamagemanager_clonepeddamage_cped_ptr_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_042195_cpeddamagemanager_clearpeddecorations_unsigned_char_bool_bool_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_042196_cpeddamagemanager_clearcompressedpeddecorations_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042199_cpeddamagemanager_getplayerdamageset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042203_cpeddamagemanager_getscardataforlocalped_cped_ptr_rage_atarray_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_042225_pedheadshotinfo_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042228_pedheadshotmanager_setupviewport(ph: *mut Phantom); }
extern "C" { pub fn f_042229_pedheadshotmanager_updateentrynameslist_rage_atvector_pedheadsho(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042387_cpedvariationdata_cpedvariationdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042388_cpedvariationstream_setpalettetexture_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042389_cpedvariationdata_setheadblenddata_cped_ptr_cpedheadblenddata_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_042390_cpedvariationstream_requeststreampedfilesinternal_cped_ptr_cpedv(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042391_cpedvariationdata_setheadoverlay_cped_ptr_eheadoverlayslots_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32); }
extern "C" { pub fn f_042392_cpedvariationdata_setheadoverlaytintindex_cped_ptr_eheadoverlays(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_042393_cpedstreamrendergfx_releaseheadblendassets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042394_cpedvariationdata_setheadblendpalettecolordisabled_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042395_cpedvariationdata_setheadblendeyecolor_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042396_cpedvariationdata_micromorph_cped_ptr_emicromorphtype_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_042397_cpedvariationdata_sethairtintindex_cped_ptr_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_042398_cpedvariationdata_getcomponentslotfrombonetag_eanimbonetag(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_042399_cpedvariationstream_isblendshapessetup_rage_rmcdrawable_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042404_cpedvariationpack_getlocalcompdata_cped_const_ptr_epedvarcomp_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_042406_cpedvariationpack_getglobalcompdata_cped_const_ptr_epedvarcomp_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_042409_cpedheadblenddata_cpedheadblenddata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042411_cpedheadblenddata_op_cpedheadblenddata_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042414_cpedheadblenddata_updatefromparents(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042415_cpedvariationinfo_getmaxnumdrawbls_epedvarcomp_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042416_cpedvariationinfocollection_getpaletteid_unsigned_int_int_int_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_042417_cpedvariationinfocollection_sorttextures_cdynamicentity_ptr_eped(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_042418_cpedvariationinfocollection_getinclusions_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042419_cpedvariationinfocollection_getexclusions_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042420_cpedvariationstream_setvariation_cdynamicentity_ptr_cpedpropdata(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_042421_cpedvariationpack_setvariation_cdynamicentity_ptr_cpedpropdata_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_042425_cpedvariationpack_extractcomponent_rage_pgdictionary_rage_rmcdra(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_042426_cpedvariationpack_updatepedforalphacomponents_centity_ptr_cpedpr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042427_cpedvariationpack_rendershadersetup_cpedmodelinfo_ptr_cpedvariat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042428_cpedvariationinfocollection_lookupcompinfoflags_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_042430_cpedvariationinfo_getdlcname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042432_cpedvariationinfocollection_lookupsecondaryflags_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_042433_cpedvariationpack_loadslods(ph: *mut Phantom); }
extern "C" { pub fn f_042434_cpedvariationpack_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_042437_cpedvariationpack_gettexvar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042438_cpedvariationpack_getcompvar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042439_cpedvariationpack_getexpressionforcomponent_cped_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_042444_cpedpropsmgr_updatepedforalphaprops_centity_ptr_cpedpropdata_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042446_cpedpropsmgr_setpedprop_cped_ptr_eanchorpoints_int_int_eanchorpo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_042472_cpedvariationstream_registerstreampedfile_cdatafilemgr_datafile(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042473_cpedvariationstream_loadfirstpersondata(ph: *mut Phantom); }
extern "C" { pub fn f_042479_rage_atarray_firstpersonalternate_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042489_cpedpropsmgr_getpedpropidx_cped_const_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042495_cpedvariationstream_processstreamrequests(ph: *mut Phantom); }
extern "C" { pub fn f_042521_cpedvariationdebug_updatepedlist(ph: *mut Phantom); }
extern "C" { pub fn f_042523_cpedvariationdebug_pedidchangecb(ph: *mut Phantom); }
extern "C" { pub fn f_042524_refreshvarset(ph: *mut Phantom); }
extern "C" { pub fn f_042532_cpedvariationdebug_createpedcb(ph: *mut Phantom); }
extern "C" { pub fn f_042536_updateattachidxcb(ph: *mut Phantom); }
extern "C" { pub fn f_042602_cpedvariationinfocollection_lookupcompinfoaudioid_int_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_042603_cpedvariationinfocollection_getcompexpressionsmods_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042604_cpedvariationinfocollection_getexpressionmods_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042606_cpedpropinfo_getmaxnumprops_eanchorpoints_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042614_cpedproprendergfx_getdrawable_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042615_cpedpropsmgr_getmaxnumprops_cdynamicentity_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042628_cpedpropsmgr_getglobalpropdata_cped_const_ptr_eanchorpoints_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_042631_cpedpropsmgr_getpedproptexidx_cped_const_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042634_cpedpropsmgr_getpedpropspacked_cdynamicentity_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042636_cpedpropsmgr_setpedpropspacked_cdynamicentity_ptr_cpackedpedprop(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042639_cpedpropsmgr_clearpedprop_cped_ptr_eanchorpoints_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042640_cpedpropsmgr_ispropvalid_cped_ptr_eanchorpoints_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_042646_cpedpropsmgr_checkpropflags_cped_ptr_eanchorpoints_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_042653_cpedpropdata_cpedpropdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042654_cpedpropdata_dtorcpedpropdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042655_cpedproprequestgfx_dtorcpedproprequestgfx(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042657_cpedproprequestgfx_clearwaitforgfx(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042661_cpackedpedprops_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042663_cpackedpedprops_setpackedpedpropdata_unsigned_int_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_042664_cpackedpedprops_getpackedpedpropdata_unsigned_int_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_042675_peddecorationcollection_peddecorationcollection(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042676_peddecorationcollection_dtorpeddecorationcollection(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042686_peddecorationmanager_addpeddecoration_cped_ptr_rage_atnonfinalha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32); }
extern "C" { pub fn f_042689_peddecorationmanager_addcollection_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042690_peddecorationmanager_buildserializerlists_rage_atfixedarray_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042691_peddecaldecorationmanager_releasedecal_peddecaldecoration_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042791_cpickup_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042797_cweaponitem_getisstreamedin(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042800_cpickup_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042814_worldprobe_debugfindgroundzfor3dcoord_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> f32; }
extern "C" { pub fn f_042818_worldprobe_cshapetestmanager_debugsubmittest_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_042819_cpickupmanager_calculatedroppedpickupposition_cpickupmanager_cpi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042821_worldprobe_cshapetestmanager_aborttest_worldprobe_cshapetestresu(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042843_cped_getgroundpos_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042848_worldprobe_cshapetestdesc_setexcludeentities_centity_const_ptr_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_042871_cpickupmanager_renderscriptedglow_rage_vec3v_const_ref_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042873_cpickupmanager_detachallportablepickupsfromped_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042874_cpickupmanager_cpickuppositioninfo_cpickuppositioninfo_rage_vect(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_042878_cpickupmanager_removeallpickups_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_042880_cpickupmanager_destroypickupplacement_cpickupplacement_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042881_cpickupplacement_createpickup_cnetobjpickup_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_042882_cpickupplacement_regenerate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042885_cpickupmanager_createpickup_unsigned_int_rage_matrix34_const_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_042896_cpickupmanager_detachallportablepickupsfromped_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042897_cpickupmanager_createsomemoney_rage_vector3_const_ref_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_043046_ccollisionhistory_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043048_ccollisionhistory_addcollisionrecord_rage_phinst_const_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32, a8: i32, a9: i64, a10: i32, a11: i32); }
extern "C" { pub fn f_043068_cdeformableobjectmanager_load_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043073_cbuoyancy_process_cphysical_ptr_float_bool_bool_float_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_043074_ctunableobjectmanager_gettuningentry_rage_atnonfinalhashstring_c(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043077_cbuoyancy_getbuoyancyinfo_cphysical_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_043079_cbuoyancy_resetbuoyancy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043080_cbuoyancy_getwaterlevelonsample_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_043085_worldprobe_cshapetestdesc_setexcludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043096_phinstgta_phinstgta_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043100_rage_fwanimdirector_getcomponentbytype_rage_fwanimdirectorcompon(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043112_fraginstgta_fraginstgta_int_rage_fragtype_const_ptr_rage_matrix3(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_043138_fraginstnmgta_checkcanactivate_cped_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_043140_fraginstnmgta_gettimesinceimpactdamage_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043141_fraginstnmgta_sendanimationaverageveltonm_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_043143_fraginstnmgta_checkforvehicleassociation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043146_fraginstnmgta_grabrootframe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043148_fraginstnmgta_setragdollejectedfromvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043151_fraginstnmgta_setactiveposefromskel_rage_crskeleton_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043154_fraginstnmgta_updatecounterimpulse_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043164_phmaterialdebug_userrenderbound_rage_phinst_ptr_rage_phbound_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_043206_cphysics_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043208_cphysics_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043210_cphysics_getshouldnetworkblendbeforephysics_cphysical_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043212_cphysics_verletsimupdate_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043213_rage_ropebankmanager_update_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043214_rage_fwentity_resetnocollision(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043219_cphysics_waitforverlettasks(ph: *mut Phantom); }
extern "C" { pub fn f_043220_cphysics_iteratemanifold_rage_phmanifold_ptr_centity_ptr_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_043223_cphysics_clearmanifoldsforinst_rage_phinst_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043224_cphysics_loadaboutposition_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043225_cphysics_getfoliageradius_rage_phinst_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> f32; }
extern "C" { pub fn f_043226_cphysics_collideinstagainstglass_rage_phinst_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043233_worldprobe_cshapetestbounddesc_setboundfromentity_centity_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043234_worldprobe_cshapetestdesc_setincludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043280_cphysics_addtophysicsrequestlist_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043292_cragdollconstraintcomponent_removeragdollconstraints(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043294_cragdollconstraintcomponent_enablehandcuffs_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_043295_cragdollconstraintcomponent_enableboundankles_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_043331_cwatertesthelper_submitasyncriverboundprobe_rage_vector3_ref_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043333_cwatertesthelper_getcachedriverboundproberesult_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_043334_cwatertesthelper_getwaterlevelincludingrivers_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_043335_worldprobe_debugfindgroundzforcoord_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_043349_worldprobe_cshapetestdesc_setexcludeinstance_rage_phinst_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043436_csettingsmanager_getinstance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_043449_meshblendmanager_hasblendfinished_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_043451_meshblendmanager_finalizeheadblend_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_043452_meshblendmanager_setnewblendvalues_unsigned_int_float_float_floa(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_043463_meshblendmanager_extracontentupdated_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043465_meshblendmanager_ishandlevalid_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_043476_meshblendmanager_setpalettecolor_unsigned_int_bool_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_043477_cmirrors_createrendertargets(ph: *mut Phantom); }
extern "C" { pub fn f_043480_crendertargets_createrendertarget_rtmempoolid_char_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_043485_water_iscameraunderwater(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043486_cmirrors_getismirrorvisible_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043489_cocclusion_gethizbuffer_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_043490_cocclusion_getminmaxboundsforbuffer_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_043491_cocclusion_getminzforbuffer_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_043492_cocclusion_waitforallrasterizerstofinish(ph: *mut Phantom); }
extern "C" { pub fn f_043500_cocclusion_getnewocclusionaabbexactsimpletestpixelsthreshold(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043501_cocclusion_getrasterizertransform_int_rage_mat44v_ref_float_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_043508_occlusionqueries_oqgetqueryresult_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_043563_cplantmgr_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043568_cplantmgr_preupdateoncefornewcamerapos_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043670_csprite2d_begincustomlist_csprite2d_customshadertype_rage_grctex(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_043671_phonephotoeditor_renderborder_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_043675_csprite2d_endcustomlist(ph: *mut Phantom); }
extern "C" { pub fn f_043677_animpostfxmanager_start_rage_atnonfinalhashstring_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_043714_animpostfxmanager_stop_rage_atnonfinalhashstring_animpostfxmanag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043715_animpostfxmanager_setuserfadelevel_rage_atnonfinalhashstring_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_043753_csprite2d_setgeneralparams_rage_vector4_const_ref_rage_vector4_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043767_cdrawlistprototypemanager_flush_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043773_cpostscan_reset(ph: *mut Phantom); }
extern "C" { pub fn f_043782_cpostscan_waitforsortpvsdependency(ph: *mut Phantom); }
extern "C" { pub fn f_043783_cpostscan_addtoalwaysprerenderlist_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043784_cpostscan_removefromalwaysprerenderlist_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043813_crenderer_constructrenderlistnew_unsigned_int_crenderphasescanne(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_043815_lights_addscenelight_clightsource_ref_clightentity_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043962_cgtaoldloadingscreen_forceloadingrenderfunction_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043967_river_removefromriverentitylist_centity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043975_csprite2d_delete(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043983_csprite2d_drawrect_float_float_float_float_float_rage_color32_co(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i64); }
extern "C" { pub fn f_044103_water_initfogtexturetxdslots(ph: *mut Phantom); }
extern "C" { pub fn f_044116_water_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044118_water_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044122_water_getwaterlevel_rage_vector3_const_ref_float_ptr_bool_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_044177_water_getcamerawaterdepth(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_044207_chighqualityscreenshot_execute(ph: *mut Phantom); }
extern "C" { pub fn f_044208_chighqualityscreenshot_takescreenshot_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_044209_chighqualityscreenshot_resetstate(ph: *mut Phantom); }
extern "C" { pub fn f_044210_chighqualityscreenshot_begintakehighqualityphoto_cphotobuffer_pt(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_044211_chighqualityscreenshot_resetstateforhighqualityphoto_cphotobuffe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044213_clowqualityscreenshot_begincreatelowqualitycopyofphotofromhighqu(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_044214_clowqualityscreenshot_requestfreememoryforlowqualityphoto(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044215_clowqualityscreenshot_shutdown_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_044216_clowqualityscreenshot_execute(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044344_centityselect_getselectedentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_044345_centityselect_getselectedentityshader(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_044403_rage_drawlistaddress_copyoffentity_centitydrawdata_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044412_getskeletonfordraw_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_044476_centitydrawhandler_centitydrawhandler_centity_ptr_rage_rmcdrawab(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_044477_centitydrawhandler_beforeaddtodrawlist_rage_fwentity_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32); }
extern "C" { pub fn f_044481_centitydrawhandler_afteraddtodrawlist_rage_fwentity_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_044489_centitydrawhandler_shouldskipentity_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_044490_centitydrawhandler_addbasictodrawlist_centitydrawhandler_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_044492_centitydrawhandler_addfragmenttodrawlist_centitydrawhandler_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_044500_cdynamicentitydrawhandler_addfragmenttodrawlist_centitydrawhandl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_044501_cdynamicentitydrawhandler_addskinnedtodrawlist_centitydrawhandle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_044520_cvolumevertexbuffer_generatepointspotvolumevb_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_044547_lights_startpedlightfadeup_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_044568_clightentity_getworldposition_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_044576_lightentitymgr_update(ph: *mut Phantom); }
extern "C" { pub fn f_044577_lightentitymgr_flush(ph: *mut Phantom); }
extern "C" { pub fn f_044578_lightentitymgr_addattachedlights_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_044579_lightentitymgr_removeattachedlights_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044607_cparaboloidshadow_updatelight_clightsource_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_044663_crenderphasecascadeshadowsinterface_script_setshadowtype_ecsmsha(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044728_crenderphasedebugoverlayinterface_togglepedhdmodesetstate_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044956_crenderphasewaterreflectioninterface_updateinwater_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_044971_creplayaudiotrackprovider_isvalidsoundhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045011_creplayeditorparameters_printmusicloadresultstrings_unsigned_lon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045025_cclip_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045028_cclip_validatetimedata(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045077_cclip_clipheader_current_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045136_cmontagemusicclip_validate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045137_rage_atarray_cmontagemusicclip_0_unsigned_short_grow_unsigned_in(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045151_cmontage_getrating(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045153_cmontagemusicclip_load_void_ptr_unsigned_int_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_045154_cmontagemusicclip_isambienttrack_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045165_centitydrawhandler_dtorcentitydrawhandler_77346(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045174_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_045178_rage_atpagedpool_clightentity_true_rage_fwdefaultrscmempagedpool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045208_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_lightsorte(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_045211_csavegamebuffer_freebuffercontainingcontentsofsavegamefile(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045215_csavegameclouddata_dtorcsavegameclouddata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045220_cscriptsavedata_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_045221_csavegamesave_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_045226_csavegamesave_genericsave_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045227_cgenericgamestorage_finishallunfinishedsavegameoperations(ph: *mut Phantom); }
extern "C" { pub fn f_045228_cscriptsavedata_shutdown_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_045229_csavegamebuffer_shutdown_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045230_csavegamefilenames_makedisplaynameforsavegame_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_045231_csavegameinitialchecks_initialchecks(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045232_csavegamedamagedcheck_checkifslotisdamaged_char_const_ptr_bool_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_045233_cdate_extractdatefromstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_045234_csavegamedialogscreens_handlesavegamedialogbox_unsigned_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_045235_csavegamebuffer_allocatedatabuffer(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045239_cphotomanager_getsavecameraphotostatus(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045240_cqueueofphotooperations_push_cphotooperationdata_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045243_csavegamebuffer_loadblockdata_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045245_csavegamequeue_push_csavegamequeuedoperation_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045254_cgenericgamestorage_createsavedataandcalculatesize_etypeofsavega(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_045255_cgenericgamestorage_findclosestsavehouse_bool_rage_vector3_ref_f(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_045256_cscriptsavearrayofstructs_createtreenodesfromscriptdata_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_045259_csavegamelist_getdisplaynameanddateforthissavegameitem_int_char(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_045260_csavegamedialogscreens_displayrockstardevwarning_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_045264_csavegamefilenames_makevalidsavenameforlocalfile_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_045265_csavegamedevices_isdevicevalid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045266_csavegameinitialchecks_showloggedsigninui(ph: *mut Phantom); }
extern "C" { pub fn f_045267_cscriptsavestack_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045268_cnestedscriptdata_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045269_csavegamedelete_begin_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045270_csavegamedelete_update(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045271_csavingmessage_begindisplaying_csavingmessage_etypeofmessage(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_045273_csavegamefilenames_makevalidsavenameforlocalphotofile_csavegamep(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045274_csavegamefrontend_setloadmenucontext_csavegamefrontend_eloadmenu(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_045277_csavegamefrontend_savegamelistscreensetup_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_045278_csavegamefrontend_savegamemenucheck_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045279_csavegamegetfilesize_getfilesize_char_const_ptr_bool_unsigned_lo(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045280_csavegameload_genericload(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045283_cphotobuffer_allocateandfillbufferwhenloadingcloudphoto_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045284_cphotobuffer_fillinvaluesfornewphoto(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045287_csavegamephotoasyncbuffer_csavegamephotoasyncbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045288_csavegamephotoasyncbuffer_dtorcsavegamephotoasyncbuffer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045289_cphotobuffer_readandverifyloadedbuffer(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045292_csavegamephotometadata_writemetadatatostring_char_ptr_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_045293_cphotobuffer_correctakamaitag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045294_csavegamephotometadata_readmetadatafromstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_045295_csavegamephotometadata_dtorcsavegamephotometadata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045297_csavegamephotoclouddeleter_addcontentidtoqueue_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045303_csavegamephotocloudlist_addtoarray_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_045304_csavegamephotocloudlist_removephotofromlist_csavegamephotounique(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045305_csavegamephotoloadlocal_dolocalphotoload_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045307_bool_rage_atmap_int_cmetadataforonephoto_rage_atmaphashfn_int_ra(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045308_csavegamephotometadata_csavegamephotometadata_csavegamephotometa(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045318_cphotomanager_getcreatesortedlistofphotosstatus_bool_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045319_cphotomanager_gettakerockstareditorphotostatus(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045320_cphotomanager_getsaverockstareditorphotostatus(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045326_cphotomanager_removephonephotofromtxd_bool_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_045327_cphotomanager_removemissioncreatorphotofromtxd_bool_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_045329_csavegamephotometadata_isentityvisible_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045332_ctexturedictionariesforgalleryphotos_texturedictionaryforgallery(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045333_csavegamephotometadata_addpedtogamertagarray_cped_ptr_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_045336_cphotomanager_takephoto_bool_ehighqualityphototype(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_045337_cphotomanager_savephoto_bool_ehighqualityphototype(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_045357_ctexturedictionariesforgalleryphotos_gettexturedictionaryindexfo(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045362_cphotomanager_unloadgalleryphoto_bool_csavegamephotouniqueid_con(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045371_csavegamephotometadata_getnormalisedscreencoordfromworldcoord_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_045373_csavegamephotosavelocal_dosavephotolocal_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045437_svenode_readjsonvalue_rage_rsonreader_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045444_svedict_dtorsvedict_77919(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045448_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_svedict(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_045459_csavegamebuffer_freealldatatobesaved_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045460_cnestedscriptdata_addactivescriptdatatopsotobesaved_rage_psobuil(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_045461_cscriptsavearrayofstructs_displaycurrentvaluesofsavedscriptvaria(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_045462_csavegamebuffer_freesavegamememorybuffer_unsigned_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045463_csavegamebuffer_loadxmltreefromstream_rage_fistream_ptr_rage_par(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_045467_cscriptsavedata_deleteallrtstructures_etypeofsavegame(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_045469_void_csavegamebuffer_loadsingleobject_cmultiplayerstatssavestruc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045470_cmultiplayerstatssavestructure_postload(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045471_cbasestatssavestructure_dtorcbasestatssavestructure(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045472_cnestedscriptdata_readanddeserializeallscripttreesfromloadedpso(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_045485_csimplevariablessavestructure_postload(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045486_cplayerpedsavestructure_postload(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045487_csavegarages_postload(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045488_cstatssavestructure_cstatssavestructure(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045489_cstatssavestructure_postload(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045490_cstatssavestructure_dtorcstatssavestructure(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045491_cradiostationsavestructure_postload(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045492_rage_atarray_cradiostationsavestructure_cradiostationstruct_0_un(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045493_cscriptsavepsofiletracker_append_rage_atnamespacedhashvalue_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_045502_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_045508_csavedoors_presave(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045518_csavegarages_csavedvehicle_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_045521_csavegarages_csavedvehiclesinsafehouse_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_045530_csavegarages_presave(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045531_csavegarages_csavedvehicle_copyvehicleintosavedvehicle_cstoredve(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045534_csavedvehicle_migration_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_045536_csavedvehiclesinsafehouse_migration_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_045542_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_045564_cplayerpedsavestructure_presave(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045565_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_045578_cradiostationsavestructure_presave(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045579_cradiostationsavestructure_cradiostationstruct_op_cradiostations(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045585_csimplevariablessavestructure_presave(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045606_cbasestatssavestructure_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_045616_cstatssavestructure_presave(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045617_cbasestatssavestructure_postloadbasestats_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045618_cbasestatssavestructure_presavebasestats_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_045619_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_045627_cstatssavestructure_migration_cstatssavestructure_migration(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045632_cstatssavestructure_migration_dtorcstatssavestructure_migration(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045647_void_crtstructuredatatobesaved_addchildobject_cmultiplayerstatss(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_045652_crtstructuredatatobesaved_singleplayer_deleteallrtstructuredata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045656_cscriptsavestruct_dtorcscriptsavestruct(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045657_cscriptsavearrayofstructs_addnewscriptstruct_int_bool_char_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_045658_rage_atarray_cscriptsavestruct_savedataitem_0_unsigned_short_cop(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045659_rage_atmap_rage_atstring_int_rage_atmaphashfn_rage_atstring_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045660_cscriptsavearrayofstructs_clearregisteredelementnames_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045661_cscriptsavearrayofstructs_creatertstructureforloading_int_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_045663_cscriptsavearrayofstructs_getsizeofstructanditschildren_int_rage(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045664_cscriptsavearrayofstructs_computesizeinbyteswithgaps_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045665_cscriptsavearrayofstructs_createpsostructureschemaforsaving_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_045666_cscriptsavearrayofstructs_loadfrompsostruct_int_unsigned_char_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_045670_cnestedscriptdata_dtorcnestedscriptdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045672_cscriptsavepsofiletracker_forcecleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045676_cnestedscriptdata_addmembertocurrentstruct_char_const_ptr_int_vo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_045677_cnestedscriptdata_checkelementname_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_045733_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_045734_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_045735_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_045736_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_045739_csavemigrationgetsingleplayersavestate_dtorcsavemigrationgetsing(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045745_csavemigrationtask_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045746_csavemigrationtask_onwait(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045780_perfclearinghouse_system_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045783_perfclearinghouse_system_render_float_float_ref_int_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32, a4: f32, a5: f32); }
extern "C" { pub fn f_045785_cped_computewandertask_cped_const_ref_cpopcycleconditions_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045786_cped_computedefaultdrivingtask_cped_const_ref_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_045787_cped_getmotiontaskfromtype_smotiontaskdata_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_045788_cped_getpedhealthinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045792_cped_getmotivation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045796_cped_computedefaulttask_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045797_cped_getcurrentmotiontask_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045802_cped_setcharparamsbasedonmanagertype(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045803_cped_setragdollstateinternal_eragdollstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045804_cped_clearactivateragdolloncollisionevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045805_cped_deletevehicleentryconfig(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045807_cped_setstairflags_worldprobe_cshapetesthitpoint_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045808_cped_isstairs_worldprobe_cshapetesthitpoint_const_ptr_rage_phins(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045809_cped_dtorcped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045810_cped_clearpedbrainwhendeletingped(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045811_cped_blendoutgestures_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045812_cped_removespeakerlistenedto(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045813_cped_destroyjetpack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045814_rage_syslinkeddata_float_refreportarmormismatch_unsigned_int_uns(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045815_cped_releasecoverpoint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045817_cped_releasedamageset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045819_cped_setactivateragdolloncollision_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045824_cped_createbank(ph: *mut Phantom); }
extern "C" { pub fn f_045833_cped_setupskeletondata_rage_crskeletondata_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045834_cped_switchtoanimated_bool_bool_bool_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_045835_cped_setbehaviorfromtaskdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045836_cped_inithealth(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045837_cpedvfx_init_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045838_cped_setupexternallydrivendofs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045839_cped_switchtostaticframe_float_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045840_cped_getbonematrix_rage_matrix34_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045841_cped_recomputecachedboneoffsets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045842_cped_forcemotionstatethisframe_cpedmotionstates_emotionstate_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_045843_cped_setclothisprone_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045844_cped_setdefaultdecisionmaker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045845_rage_syslinkeddata_float_refreporthealthmismatch_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045846_rage_syslinkeddata_float_refreporthealthmismatch_unsigned_int_un(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045847_cped_removerotationalconstraint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045848_cped_addrotationalconstraint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045849_cped_processrotationalconstraint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045852_cped_resetdesiredmainmovercapsuledata_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045853_cped_setboundsize_cbipedcapsuleinfo_const_ptr_float_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_045856_cped_deactivatephysicsanddealwithragdoll_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045861_cped_updateragdollroottransformfromanimatedskel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045865_cped_updateragdollmatrix_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045871_cped_canattachtoground_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045872_cped_attachtoground(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045874_cped_isonground_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045878_cped_getmassforapplyvehicleforce_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_045881_cped_setragdollstate_eragdollstate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045882_cped_switchtoragdollinternal(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045883_cped_setpedoutofvehicle_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_045884_cped_knockpedoffvehicle_bool_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_045885_cped_switchtoragdoll_cevent_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045886_cped_setmyvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045887_rage_fwentity_setnocollision_rage_fwentity_ptr_unsigned_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_045888_cped_applyanimatedvelocitywhilstattached_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_045889_cped_getprimarymotiontask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045890_rage_matrix34_dottranspose_rage_matrix34_const_ref_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_045892_cped_clearragdollblockingflags_eragdollblockingflagsbitset_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045893_cped_getragdollblockingflags(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045900_cped_updatecachedsceneupdateflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045901_cped_startaiupdate(ph: *mut Phantom); }
extern "C" { pub fn f_045902_cped_updatespatialarraytypeflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045903_cped_getiscrouching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045905_cped_canpedstandup(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045909_cped_getprimarymotiontaskifexists_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045911_cped_wantstouseactionmode_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045913_cped_isfirstpersonshootermodeenabledforplayer_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045917_cped_invalidateupperbodyfixupdofs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045921_cped_blockconversationlookat_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_045922_cped_getboneposition_rage_vector3_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045923_cped_blockgestures_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045924_cped_getgesturecontext_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045925_rage_fwmovenetworkinterface_getproperty_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045926_cped_getisdrivingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045928_cped_processgesturehashkey_audgesturedata_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_045929_cped_blendingesture_rage_crclip_const_ptr_rage_fwmvtemplateid_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32); }
extern "C" { pub fn f_045930_cped_printgestureinformation_rage_fwmvtemplateid_rage_fwmvclipse(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_045931_cped_controlsecondarytaskblend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045932_cped_processincarconversationlookatduringwait_cped_ptr_bool_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_045933_cped_computeconversationlookatholdtimeandflags_cped_ptr_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: f32, a7: f32, a8: f32, a9: i64) -> i32; }
extern "C" { pub fn f_045934_cped_processbodyandfacialgesturesembeddedinaudioassets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045935_cped_processvoicedrivenmouthmovement(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045936_cped_stopvoicedrivenmouthmovement(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045937_cped_hasheadblend_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045940_cped_setvariation_epedvarcomp_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_045950_cped_setheadblendpalettecolor_unsigned_char_unsigned_char_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045965_cped_getpedrace_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045967_cped_setremoveassoonaspossible_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045972_cped_getweaponhash_cdynamicentity_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045973_cped_issuperlodfromindex_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045977_cped_setdesiredheading_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_045985_cped_validategroundphysical_rage_phinst_const_ref_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_045986_rage_phcollider_getlocalvelocity_float_vector_4_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: u128); }
extern "C" { pub fn f_045987_cped_checkforplayerbeingsquashedbykinematicped_rage_phcontactite(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_045988_cped_dokickped_rage_phcontactiterator_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_045989_rage_phcontact_setworldnormal_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045990_rage_vec_v3rotateaboutzaxis_float_vector_4_float_vector_4(ph: *mut Phantom, a0: u128, a1: u128) -> u128; }
extern "C" { pub fn f_045992_rage_phcontactconstiterator_getimpulse_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045993_cped_shouldobjectbeimpossibletopush_centity_const_ref_rage_phins(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_045994_cped_processprecomputemaincapsuleimpact_rage_phcontactiterator_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_045995_cped_candisableimpactforenterexitvehicle_centity_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_045996_cped_getbonetagfromragdollcomponent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045997_cped_convertbonetobodypart_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045998_cped_getboneindexfromragdollcomponent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045999_cped_isgroundbetterthancurrent_centity_const_ptr_rage_phinst_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i32; }
extern "C" { pub fn f_046000_cped_setground_centity_ptr_rage_phinst_const_ptr_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64); }
extern "C" { pub fn f_046003_cped_applyquadrupedspring_float_float_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_046004_cped_processpedstanding_float_rage_phcollider_ptr(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_046007_cped_applymovementrequests_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046008_cped_processbuoyancy_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_046009_cped_getinflictorfordrowninghandcuffedinvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046010_cped_getcanusehighdetailwaterlevel_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046011_cped_setwetclothingheight_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046014_cped_incwetclothing_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046015_cped_getisshelteredinvehicle(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046016_cped_getisshelteredfromrain(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046021_cped_processcontrol_resetvariables(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046022_cped_processcontrol_intelligence_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_046023_cped_processcontrol_weaponsandaccessoriespostintelligence_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046024_cped_updatemovementmode_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046025_cped_setdynamicentityflagsforweapon_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046026_cped_getvehiclepedentering_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046028_cped_processspecialnetworkleave(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046029_setprotectedbaseflagandfpvextensiononchildattachments_cped_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_046030_cped_updatefirstpersonflags_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046032_cped_processcontrol_animstateupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046033_cped_playfootsteps(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046034_cped_processactionmodelookat(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046035_cped_processexternallydrivendofs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046036_cped_processinjuredclipsetrequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046037_cped_processcontrol_graphics_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046038_cped_processcontrol_physics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046039_cped_releasedesiredcoverpoint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046044_cped_processbounds_float_bool(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_046045_cped_setclothisskydiving_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046046_cped_isnearmirror_rage_scalarv_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046047_rage_matrix34_fromeulersxyz_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046054_cped_setusingactionmode_bool_cped_actionmodeenabled_float_bool_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_046055_cped_setmovementmodeoverridehash_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046056_cped_canbedeleted_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_046057_cped_updateragdollboundsfromanimatedskel_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046062_cped_updatedamagepriority_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046069_cped_calculatedynamicambientscaleswithvehiclescaler(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046070_cped_updatefpscamerarelativematrix(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046073_cped_processlefthandgripik_bool_bool_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_046074_cpedintelligence_gettasksecondaryactive_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046076_clightsource_clightsource_elighttype_int_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: f32) -> i64; }
extern "C" { pub fn f_046077_clightsource_setdirtangent_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046078_cped_renderpedlight_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046079_cped_renderlightonphone_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046086_cped_resetstanddata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046087_cped_isnetworkbot_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046090_cped_flagtodestroywhennextprocessed_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046092_cped_debugattachpedtoentercar_char_const_ptr_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64); }
extern "C" { pub fn f_046095_cped_getpedaudioid_unsigned_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046096_cped_getbonepositioncachedv_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046099_cped_getragdollcomponentmatrix_rage_matrix34_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_046102_cped_getisenteringvehicle_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046103_cped_getspecialability_eplayerspecialabilityslot(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046105_cped_getblockingofnontemporaryeventsthisframe_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046108_cped_shoulduseinjuredmovement_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046123_cped_createvehicleentryconfigifrequired(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046129_cped_initspecialability_eplayerspecialabilityslot(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046134_cped_iscivilianped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046144_cped_newsay_char_const_ptr_unsigned_int_bool_bool_int_cped_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32) -> i32; }
extern "C" { pub fn f_046147_cped_newsay_unsigned_int_unsigned_int_bool_bool_int_cped_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_046157_cped_setcoverpoint_ccoverpoint_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046166_cped_setclothisfalling_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046167_cped_setclothnegateforce_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046168_cped_isprone_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046176_cped_getcontrolfromplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046177_cped_resetmovementmode(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046179_cped_instantanimupdateend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046180_cped_instantaiupdate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046181_cped_computeexternallydrivenpelvisoffsetforfpsstealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_046183_cped_getragdollcomponentfrombonetag_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046185_cped_setinitialstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046206_cpedvfx_processvfxfootstep_bool_bool_rage_mat34v_const_ref_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_046209_cped_setplayerinfo_cplayerinfo_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046222_cped_setcontrolledbyinfo_ccontrolledbyinfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046245_bool_creplaymgrinternal_handleentityforfx_cpacketpedvariationcha(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_046246_rage_crtagiteratorpropertyattributevalue_rage_crpropertyattribut(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046280_cdynamicentity_cdynamicentity_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046281_cdynamicentity_logbaseflagchange_int_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_046282_cportaltracker_setowner_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046283_canimatedbuilding_dtorcanimatedbuilding(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046284_cportaltracker_dtorcportaltracker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046285_centity_dtorcentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046293_cinteriorinst_appendobjectinstance_centity_ptr_rage_fwinteriorlo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046296_centity_removefrominterior(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046298_centity_initentityfromdefinition_rage_fwentitydef_ptr_rage_fwarc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_046299_centity_tellaudioaboutaudioeffectsadd(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046301_centity_createdrawable(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046302_cdynamicentity_createskeleton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046303_cdynamicentity_createanimdirector_rage_rmcdrawable_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_046304_canimatedbuilding_updaterateandphase(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046305_cdynamicentity_addtosceneupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046306_centity_sethidden_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046308_cdynamicentity_addshadervarcreaturecomponent_ccustomshadereffect(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046311_cdynamicentity_deletedrawable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046316_cbuilding_cbuilding_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046318_cbuilding_dtorcbuilding(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046324_cgameworld_findfollowplayerspeed(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046325_cgameworld_findfollowplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_046326_cgameworldheightmap_getmaxheightfromworldheightmap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_046327_cplayerswitchinterface_isactive_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_046333_cdatafilemgr_datafilearray_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_046338_cdatafilemgr_changesetdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_046343_cdatafilemgr_changesetdata_dtorchangesetdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046346_executionconditions_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_046348_cdatafilemgr_contentchangeset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_046353_cdatafilemgr_contentchangeset_dtorcontentchangeset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046357_cdatafilemgr_contentsofdatafilexml_dtorcontentsofdatafilexml(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046358_cdatafilemgr_changesetdata_isvalid_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046359_cdatafilemgr_load_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046360_cfileloader_checkfileforreplacements_char_const_ptr_char_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046361_cdatafilemgr_contentchangeset_op_cdatafilemgr_contentchangeset_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046362_cdatafilemgr_expandfilename_char_const_ptr_char_ptr_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046363_cdatafilemgr_addfile_cdatafilemgr_datafiletype_char_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32, a9: i32, a10: i32, a11: i32); }
extern "C" { pub fn f_046364_char_ptr_rage_safecat_char_char_4_256ul_char_ref_256ul_char_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046365_rage_atarray_cdatafilemgr_resourcereference_0_unsigned_short_cop(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046366_cdatafilemgr_createdependentsgraph_rage_atnonfinalhashstring_boo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046369_rage_atarray_cdatafilemgr_datafile_0_unsigned_short_grow_unsigne(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046377_cdownloadabletexturemanager_dtorcdownloadabletexturemanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046379_cdownloadabletexturemanager_requesttexturedownload_int_ref_ctext(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_046380_cdownloadabletexturemanager_removedownloadrequest_ctexturedownlo(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046381_cdownloadabletexturemanager_preparedownloadrequestfordecoding_ct(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046382_rage_sysmessagequeue_decodetextureentry_64_false_push_decodetext(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046383_cdownloadabletexturemanager_releasetexturedownloadrequest_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046386_cdownloadabletexturemanager_isrequestready_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046387_cdownloadabletexturemanager_hasrequestfailed_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046388_cdownloadabletexturemanager_getrequesttxdslot_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046391_cdownloadabletexturemanager_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046392_cdownloadabletexturememorymanager_dumpmanagedtxdlist_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046393_cscriptdownloadabletexturemanager_printtextures_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046397_cscriptdownloadabletexturemanager_requesttexture_ctexturedownloa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_046404_cportaltracker_setloadscollisions_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046405_cportaltracker_addtoactivatingtrackerlist_cportaltracker_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046408_cinteriorinst_getinteriorforlocation_rage_fwinteriorlocation_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046410_centity_getinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046411_cinteriorproxy_removefromretainlist_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046426_cdynamicentity_getextendedboundbox_rage_spdaabb_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046429_cdynamicentity_updateskeleton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046430_cdynamicentity_inverseupdateskeleton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046431_cdynamicentity_startanimupdate_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046432_cdynamicentity_updatevelocityandangularvelocity_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046433_cdynamicentity_startanimupdateaftercamera_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046434_cdynamicentity_endanimupdate_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046436_cportaltracker_update_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046438_cgameworld_add_centity_ptr_rage_fwinteriorlocation_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046439_ptupdatedebug_intlocprinter_rage_fwinteriorlocation(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046440_cportaltracker_determineinteriorlocationfromcoord_rage_fwinterio(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32) -> i32; }
extern "C" { pub fn f_046441_setdestloc_unsigned_int_rage_fwinteriorlocation_ref_rage_fwinter(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_046442_cinteriorproxy_getfromlocation_rage_fwinteriorlocation_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046443_cinteriorinst_getportalinroom_unsigned_int_unsigned_int_rage_fwp(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_046444_cinteriorinst_getmatchingportalinst_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_046445_cportaltracker_collectstack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046446_cinteriorinst_getdestthruportalinroom_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046447_cinteriorproxy_createlocation_cinteriorproxy_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046448_cportalinst_findintersectingportals_rage_vec3v_const_ref_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_046449_cportaltracker_movetonewlocation_rage_fwinteriorlocation(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046451_centity_prerender_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046454_centity_deletedrawable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046455_cdynamicentity_setheading_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046456_cdynamicentity_setposition_rage_vector3_const_ref_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_046457_cdynamicentity_setmatrix_rage_matrix34_const_ref_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_046461_cdynamicentity_cleanupmissionstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046464_cgameworld_updateentitydesc_rage_fwentity_ptr_rage_fwentitydesc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046467_cgameworld_remove_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046469_cgameworld_postremove_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046470_centity_deleteinst_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046481_centity_setmodelid_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046484_centity_setisvisibleformodule_eisvisiblemodule_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046488_centity_calculatedynamicambientscales(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046490_centity_isvisible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046496_centity_wasunabletoaddtopathserver(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046497_centity_setphysicsinst_rage_phinst_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046499_centity_getcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046502_centity_getphysarch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046507_centity_enablecollision_rage_phinst_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046508_centity_disablecollision_rage_phinst_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046515_centity_changevisibilitymask_unsigned_short_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046516_centity_processfxentitycollision_rage_vector3_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32); }
extern "C" { pub fn f_046520_centity_trytoexplode_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_046521_centity_processcollisionvfx_vfxcollisioninfo_s_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046528_centity_setisvisibleforallmodules(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046530_centity_commondebugstr_centity_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046531_centity_getisvisibleformodule_eisvisiblemodule_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046532_centity_copyisvisibleflagsfrom_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046538_centity_getguid_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046543_centity_getcreationframeref_frameref_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046544_centity_getusedinreplay_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046545_centity_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046551_ebstatic_updateselectedentitybatchwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046552_ebstatic_onmapnamecombochanged(ph: *mut Phantom); }
extern "C" { pub fn f_046553_ebstatic_onselectedentitybatchchanged(ph: *mut Phantom); }
extern "C" { pub fn f_046554_ebstatic_refreshmapdatadefindexmap(ph: *mut Phantom); }
extern "C" { pub fn f_046558_ebstatic_displaypropentityinfo_centitybatch_const_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046559_ebstatic_displaygrassentityinfo_cgrassbatch_const_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046561_void_ebstatic_debugdrawlinkedentitybounds_rage_fwpropinstancelis(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046563_void_ebstatic_debugdrawentitybatchinstance_cgrassbatch_cgrassbat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046564_cfrustumdebug_debugrender_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046566_cfrustumdebug_debugrender_cfrustumdebug_const_ptr_rage_color32(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046567_cfrustumdebug_computeintersectionof3planesdebug_rage_vector4_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_046587_loddrawable_calclodlevelandcrossfade_rage_rmcdrawable_const_ptr(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32); }
extern "C" { pub fn f_046591_cinstancelistassetloader_requestallmodels_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046594_cinstancelistassetloader_releaseallmodels_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046596_centityidhelper_computeentityid_rage_fwentity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046597_centityidhelper_getentityfromid_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046598_centityidhelper_getentityshaderindexfromid_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046643_ssetupdata_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046644_ssetupdata_dtorssetupdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046666_cmountablecontent_registerexecutioncheck_rage_atdelegate_bool_ra(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046668_cmountablecontent_cmountablecontent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046669_rage_atarray_cmountablecontent_0_unsigned_short_grow_unsigned_in(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046670_cmountablecontent_op_cmountablecontent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046671_cextracontentmanager_loadcontent_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046672_cextracontentmanager_executetitleupdatedatapatch_unsigned_int_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046673_cextracontentmanager_executecontentchangesetgroupinternal_cmount(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046674_cextracontentmanager_beginenumeratecontent_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046675_cmapfilemgr_loadglobaltxdparents_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046677_cextracontentmanager_revertcontentchangesetgroupinternal_cmounta(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046679_cmountablecontent_revertcontentchangeset_rage_atnonfinalhashstri(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_046680_cmountablecontent_shutdowncontent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046682_cmapfilemgr_cleanup(ph: *mut Phantom); }
extern "C" { pub fn f_046684_cextracontentmanager_updatecontentarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046685_cextracontentmanager_revertcontentchangesetinternal_cmountableco(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_046686_cextracontentmanager_getcontentbydevice_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_046688_cextracontentmanager_executecontentchangesetinternal_cmountablec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_046689_cextracontentmanager_triggerloadingscreen_loadingscreencontext(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046690_cmountablecontent_executecontentchangeset_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046691_cmountablecontent_parseexpression_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046692_cmapfilemgr_reset(ph: *mut Phantom); }
extern "C" { pub fn f_046695_ctexlod_flushallupgrades(ph: *mut Phantom); }
extern "C" { pub fn f_046696_cfileloader_executecontentchangeset_cdatafilemgr_contentchangese(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046697_rage_fwmapdatastore_destroychildrencache(ph: *mut Phantom); }
extern "C" { pub fn f_046698_cmapfilemgr_setmapfiledependencies(ph: *mut Phantom); }
extern "C" { pub fn f_046699_cmanagedimapgroupmgr_markmanagedimapfiles(ph: *mut Phantom); }
extern "C" { pub fn f_046700_cinteriorproxy_addalldummybounds_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_046701_cextrametadatamgr_refreshbankwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046702_cwarpmanager_setwarp_rage_vector3_const_ref_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: f32, a6: i32) -> i32; }
extern "C" { pub fn f_046703_cfileloader_revertcontentchangeset_cdatafilemgr_contentchangeset(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046705_cextracontentmanager_executependingoverlays(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046706_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_046707_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_046708_cmountablecontent_mount_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046710_cextracontentmanager_executetitleupdatedatapatchgroup_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046711_cmapfilemgr_setuptxdparent_ctxdrelationship_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046713_cmountablecontent_unmountcontent_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046715_cextracontentmanager_populatecontenttablewidget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046716_cextracontentmanager_getcrc_unsigned_int_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046717_cextracontentmanager_getmapchangescrc_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046719_cextracontentmanager_recursiveenumextracontent_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046720_cextracontentmanager_addcontentfolder_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046721_cextracontentmanager_iscontentfilenamepresent_char_const_ptr_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046722_cextracontentmanager_getcontenthash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046723_cextracontentmanager_resetsavegameinstalledpackagesinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046724_cextracontentmanager_setsavegameinstalledpackage_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046725_cmountablecontent_mountcontent_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046726_cmountablecontent_loadsetupxml_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046727_cmountablecontent_parseversionstring_char_const_ptr_int_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046728_cextracontentmanager_insertcontentlock_unsigned_int_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046729_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_046730_cextracontentmanager_displaycontentlocks_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046731_cextracontentmanager_addcontentlocktobanklist_unsigned_int_scont(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_046732_cextracontentmanager_removecontentlock_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046734_cextracontentmanager_modifycontentlockstate_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046741_cextrametadatamgr_displaytattoos_int_unsigned_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046742_cextrametadatamgr_vehiclerefreshcb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046743_cextrametadatamgr_weaponrefreshcb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046744_cextrametadatamgr_interiorrefreshcb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046745_cextrametadatamgr_displaypedcomponents_int_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046746_cextrametadatamgr_displaypedprops_int_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046747_cextrametadatamgr_displaypedoutfits_int_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046748_cextracontentmanager_bank_changesetgroupchanged(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046779_clayoutmanager_selectroomclickcb_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046780_clayoutmanager_selectlayoutclickcb_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046784_cextracontentmanager_populateopenfilesatpage_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_046785_getdevicecontentsrecursive_rage_atbinarymap_rage_atstring_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046786_cextracontentmanager_populateunusedfilesatpage_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_046791_cmountablecontent_initcrcdevice_char_const_ptr_rage_fidevice_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_046792_cinteriorproxy_setisdisableddlc_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046795_cfileloader_buildcontentchangesetactionlist_cdatafilemgr_content(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_046796_cfileloader_executechangesetaction_cdatafilemgr_changesetaction(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046806_cextrametadatamgr_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046807_rage_atarray_speddlcmetafiles_0_unsigned_short_destruct_speddlcm(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046811_cextrametadatamgr_ismodelinmemory_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046812_cextrametadatamgr_addvehiclescollection_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046813_cextrametadatamgr_buildvehiclelookup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046814_cextrametadatamgr_removevehiclescollection_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046815_rage_atarray_shopvehicledata_0_unsigned_short_copyfrom_rage_atar(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046816_rage_atarray_shopvehiclemod_0_unsigned_short_op_rage_atarray_sho(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046820_cextrametadatamgr_buildweaponlookup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046821_rage_atarray_weaponshopitem_0_unsigned_short_op_rage_atarray_wea(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046827_rage_atarray_tattooshopitem_0_unsigned_short_copyfrom_rage_atarr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046828_cextrametadatamgr_rebuildapparellookup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046829_rage_atbinarymap_cextrametadatamgr_shoppedapparellookupentry_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046830_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_046831_speddlcpackmetafiles_op_speddlcpackmetafiles_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046832_cextrametadatamgr_getcreaturemetadataindices_cpedmodelinfo_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046835_cpedvariationinfocollection_getdlcnamefrompropidx_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_046837_cextrametadatamgr_getshoppedapparel_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046861_cextrametadatamgr_componentdblclickcb_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046867_cextrametadatamgr_propdblclickcb_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046879_cextrametadatamgr_weaponcombocb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046883_cextrametadatamgr_vehiclecombocb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046893_rage_atbinarymap_char_const_ptr_unsigned_int_safeinsert_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046895_cextrametadatamgr_proxyindexcombocb(ph: *mut Phantom); }
extern "C" { pub fn f_046897_cextrametadatamgr_addcomponenttobanklist_shoppedcomponent_ptr_in(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046898_cextrametadatamgr_addproptobanklist_shoppedprop_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046899_cextrametadatamgr_addoutfittobanklist_shoppedoutfit_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046903_cextrametadatamgr_moveproxytofilecb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046907_nmtuningsetentry_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_046910_nmtasksimpletunableentry_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_046917_restrictiontags_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_046920_componentdescription_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_046923_propdescription_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_046926_baseshoppedapparel_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_046929_shoppedcomponent_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_046932_shoppedprop_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_046935_shoppedoutfit_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_046941_tattooshopitem_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_046947_shopweaponcomponent_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_046950_weaponshopitem_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_046959_shopvehicledata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_046969_nmextratunables_nmextratunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046970_nmextratunables_dtornmextratunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046971_rage_atarray_ctasknmbrace_tunables_vehicletypeoverrides_0_unsign(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046975_baseshoppedapparel_dtorbaseshoppedapparel(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046979_shoppedapparel_dtorshoppedapparel(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046985_weaponshopitem_dtorweaponshopitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046995_cfileloader_addrpffile_cdatafilemgr_datafile_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046996_cinteriorproxy_disableallcollisions_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046997_cfileloader_buildinvalidatelist_rage_atarray_rage_atstring_0_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046998_rage_atarray_cdatafilemgr_changesetaction_0_unsigned_short_grow(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046999_cfileloader_buildrpfenablementlist_cdatafilemgr_contentchangeset(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_047000_cfileloader_builddatafileenablementlist_cdatafilemgr_contentchan(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_047001_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_047002_cfileloader_addallrpffiles_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047003_cfileloader_loadobjecttypesxml_char_const_ptr_cdatafilemgr_dataf(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_047006_ciplcullbox_registercontainers(ph: *mut Phantom); }
extern "C" { pub fn f_047007_ciplcullbox_readfile_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047009_cinteriorproxy_getchecksum(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_047010_ciplcullboxculllist_setactive_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047012_cfocusentitymgr_getpos(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047019_cinstancepriority_update(ph: *mut Phantom); }
extern "C" { pub fn f_047020_cgameworld_findlocalplayerwanted(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047024_cloadscene_start_rage_vec3v_const_ref_rage_vec3v_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_047025_cloadscene_stop(ph: *mut Phantom); }
extern "C" { pub fn f_047026_cstreamvolume_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047027_cinteriorproxy_updateimappinstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047029_cloadscene_startstreamingforexterior(ph: *mut Phantom); }
extern "C" { pub fn f_047030_cloadscene_isloaded(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_047033_cinteriorproxy_findactiveintersecting_rage_fwissphereintersectin(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_047034_cportaltracker_probeforinteriorproxy_rage_vector3_const_ref_cint(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32) -> i32; }
extern "C" { pub fn f_047035_cstreamvolume_initcommon_unsigned_char_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_047041_cmapchange_execute_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_047042_cmapchange_findentitiesfromworldsearch_rage_atarray_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_047045_cmapchangemgr_add_unsigned_int_unsigned_int_rage_spdsphere_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_047047_rage_atarray_cmapchange_0_unsigned_short_deletefast_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047051_cphysical_dtorcphysical(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047052_cgameworld_removeentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047054_cphysical_setisinwater_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047056_cphysical_removephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047057_cphysical_removeblip_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047058_cphysical_removeconstraints_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047059_cphysical_updatephysicsfromentity_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047060_cphysical_activatephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047061_cphysical_deactivatephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047064_cphysical_setvelocity_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047065_cphysical_getvelocity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047068_cphysical_getangvelocity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047073_cphysical_sethealth_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047074_cphysical_changehealth_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047075_cphysical_setweapondamageinfo_centity_ptr_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_047076_cphysical_getweapondamagedtimebyentity_centity_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_047083_cphysical_setupmissionstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047085_cphysical_updateentityfromphysics_rage_phinst_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_047088_cphysical_resetwaterstatus(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047089_cphysical_setmatrix_rage_matrix34_const_ref_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_047090_cphysical_processcollision_rage_phinst_const_ptr_centity_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: f32, a7: i64, a8: i32, a9: i32, a10: i64, a11: i32, a12: i32); }
extern "C" { pub fn f_047091_cphysical_addcollisionrecordbeforedisablingcontact_rage_phinst_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_047092_cphysical_docollisioneffects(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047093_vfxcollinfo_s_dtorvfxcollinfo_s(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047095_cphysical_applyforce_rage_vector3_const_ref_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32); }
extern "C" { pub fn f_047096_cphysical_applyforcecg_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_047098_cphysical_applytorque_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_047100_cphysical_applyimpulse_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047101_cphysical_applyimpulsecg_rage_vector3_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047104_cphysical_applyexternalforce_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: f32, a6: f32); }
extern "C" { pub fn f_047106_cphysical_getlocalspeed_rage_vector3_const_ref_bool_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047107_cphysical_processprephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047108_cphysical_processpostphysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047113_cphysical_iscollisionloadedaroundposition(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047114_cgameworld_postadd_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047118_cphysical_debugattachtophysicalusingphysics_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64, a10: i64, a11: f32, a12: i32, a13: f32, a14: f32) -> i32; }
extern "C" { pub fn f_047123_cphysical_detachfromparent_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047124_cphysical_debugprocessattachment_char_const_ptr_char_const_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_047125_rage_matrix34_dot3x3_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047126_cphysical_debugprocessallattachments_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_047127_cphysical_removedeletewithparentprogenyfromlist_centity_ptr_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_047131_cphysical_updatepossiblytoucheswaterflag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047133_cphysical_setdontloadcollision_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047134_cphysical_cleanupmissionstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047135_cphysical_calculategroundvelocity_cphysical_const_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_047136_cphysical_calculategroundvelocityintegrated_cphysical_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64); }
extern "C" { pub fn f_047137_cphysical_calculategroundangularvelocity_cphysical_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_047138_cphysical_getrootgroundphysicalforphysical_cphysical_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047140_ggtadataaddressresolverhelper_void_const_ptr_rage_atstring_ref_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_047147_cinteriorgroupmanager_init(ph: *mut Phantom); }
extern "C" { pub fn f_047148_cgameworld_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047155_cgameworld_sceneupdateupdatecb_rage_fwentity_ref_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047164_cgameworld_validateshutdown(ph: *mut Phantom); }
extern "C" { pub fn f_047167_ccompentity_update(ph: *mut Phantom); }
extern "C" { pub fn f_047168_cinteriorproxy_immediatestateupdate_rage_fwptrlistsinglelink_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047169_cinteriorinst_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047171_ccompentity_immediateupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047172_cgameworld_updateentitiesthroughanimqueue_sceneupdateparameters(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047173_cgameworld_removeoutofmapped_cped_ptr_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047174_csceneloader_updateloadscene(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_047175_cscenememorycheck_setstate_cscenememorycheck_estate(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047176_cmapoptimizehelper_performnewscan_rage_vector3_const_ref_bool_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_047177_ciplcullboxentry_generateculllist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047180_cscene_addwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_047252_cmapoptimizehelper_switchmodecb(ph: *mut Phantom); }
extern "C" { pub fn f_047276_ciplcullboxeditor_findnextcb(ph: *mut Phantom); }
extern "C" { pub fn f_047287_rage_atarray_cmipswitcher_0_unsigned_short_construct_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_047288_ctxdpairbinding_dtorctxdpairbinding(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047290_ctexlod_triggerhdtxdupgrade_rage_fwassetlocation_unsigned_int_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_047292_cstreamingbucketset_dumpallentitiesonce(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047296_cstreamingbucketset_waitforsorting(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047297_ctexlod_setswapstateall_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047298_ctexlod_unbindallboundtxds(ph: *mut Phantom); }
extern "C" { pub fn f_047305_cgameworld_getmainplayerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047308_cworldpoints_reactivateallworldpointbrainsthatarewaitingtillouto(ph: *mut Phantom); }
extern "C" { pub fn f_047309_cworldpoints_reactivatenamedworldpointbrainswaitingtilloutofrang(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047311_centityreportentry_tostring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047312_csceneisolator_getentitycost_centity_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047323_gettxdrefsusedbyentity_rage_atmap_ctxdref_unsigned_int_rage_atma(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047327_rage_atmap_ctxdref_unsigned_int_rage_atmaphashfn_ctxdref_rage_at(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047328_gettxdrefshortdesc_ctxdref_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047333_clodmgr_withinvisiblerange_centity_ptr_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_047339_cloddebug_findchildren_centity_ptr_rage_atarray_centity_ptr_0_un(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047355_csceneloader_loadscene_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047361_cplayerswitchdbg_syncwithcurrentsettings(ph: *mut Phantom); }
extern "C" { pub fn f_047372_csceneloader_process_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047374_cgameworld_changeplayerped_cped_ref_cped_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_047378_ccompentity_dtorccompentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047379_ccompentity_writepreloadpacketifrequired_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047380_ccompentity_releasepreloadedassets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047381_ccompentity_removecurrentassetsetexceptfor_ecompentityassetset(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047382_bool_creplaymgrinternal_recordfx_cpacketrayfirepreload_cpacketra(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047386_ccompentity_releasestreamrequests(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047388_ccompentity_settostarting(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047389_ccompentity_removeunwanted(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047390_ccompentity_addwanted(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047391_ccompentity_issueassetrequests_ecompentityassetset(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047392_ccompentity_imapobjectsremove_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047393_ccompentity_createnewchildobject_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047394_ccompentity_triggerobjectanim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047395_ccompentity_settoending(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047396_ccompentity_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047397_ccompentity_isassetsetready_ecompentityassetset(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047407_cloddebug_drawinfo_centity_ptr_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_047409_cloddisttweaker_changeallchildrenloddists_centity_ptr_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047410_cloddebuglodoverride_cloddebuglodoverride_centity_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_047411_rage_atarray_cloddebuglodoverride_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047431_clodlight_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_047432_cdistantlodlight_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_047433_cmapdata_dtorcmapdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047434_clodlight_dtorclodlight(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047439_cgameworld_addscenariopointsforentity_centity_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047441_cinteriorproxy_add_cmloinstancedef_ptr_rage_strlocalindex_rage_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_047442_cinteriorproxy_initfrommloinstancedef_cmloinstancedef_ptr_int_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_047446_cextensiondef_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_047531_centitydef_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_047536_cbasearchetypedef_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_047547_ccompositeentityanimation_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_047569_cmloentityset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_047583_cmloarchetypedef_dtorcmloarchetypedef(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047600_ccompentityanims_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_047605_ccompositeentitytype_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_047655_cplayerswitchmgrlong_setdest_cped_ref_cplayerswitchparams_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047656_cplayerswitchinterface_start_int_cped_ref_cped_ref_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_047677_cplayerswitchmgrbase_dtorcplayerswitchmgrbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047683_cplayerswitchmgrlong_streamer_startstreaming_cplayerswitchmgrlon(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047685_cplayerswitchmgrlong_streamer_isfinishedstreaming(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047687_cplayerswitchmgrlong_setstate_cplayerswitchmgrlong_state_const_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047688_cplayerswitchmgrlong_getnextstate_cplayerswitchmgrlong_state_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_047689_cplayerswitchmgrlong_startmainswitcheffectoutro(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047701_cfrustumdebug_set_rage_matrix34_const_ref_float_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32); }
extern "C" { pub fn f_047704_cinteriorinst_dtorcinteriorinst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047705_cinteriorinst_deletedetaildrawables(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047706_cinteriorinst_capturepedsandvehicles(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047709_cinteriorinst_createentity_rage_fwentitydef_ptr_rage_fwinteriorl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_047710_cinteriorinst_createinteriorentities_cmlomodelinfo_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_047711_cinteriorinst_createinterfaceportalinsts(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047712_cinteriorinst_calcroomboundbox_unsigned_int_rage_spdaabb_ref_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_047716_cinteriorinst_holdroomzeromodels_cinteriorinst_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047717_cinteriorproxy_shouldentityfreezeandretain_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047718_cinteriorproxy_movetoretainlist_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047727_cinteriorinst_isallowrunmlo_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047730_cinteriorinst_getportalobjinstanceptr_int_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_047740_cinteriorinst_getlodstate_char_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047741_cinteriorinst_addroomcontents_unsigned_int_rage_atarray_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_047748_cinteriorproxy_activateentityset_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047759_rage_atarray_rage_spdaabb_0_unsigned_short_ref_rage_atmap_cinter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047762_cinteriorproxy_deleteentityset_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047766_writeinteriorproxycacheentrydetailstotextfile_cinteriorproxycach(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047789_clayoutmanager_visitinterior_cmilointerior_ptr_clayoutmanager_ep(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047792_clayoutmanager_visitlayout_clayoutnode_ptr_clayoutmanager_eproce(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_047797_clayoutmanager_unloadlayout(ph: *mut Phantom); }
extern "C" { pub fn f_047798_clayoutmanager_loadlayout_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047799_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_047800_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_047801_clayoutmanager_registerlayouthandle_clayoutnode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047802_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_047803_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_047804_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_047805_clayoutmanager_populateinteriorwithlayout_cinteriorinst_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047806_clayoutmanager_deletelayout(ph: *mut Phantom); }
extern "C" { pub fn f_047810_crsref_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_047813_clayoutnode_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_047814_cgroup_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_047819_cmiloroom_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_047829_crsref_dtorcrsref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047837_cmilointerior_dtorcmilointerior(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047843_cportaldebughelper_updateentitysetlistcb(ph: *mut Phantom); }
extern "C" { pub fn f_047854_cportaltracker_removefrominterior(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047861_cportaltracker_getcurrroomhash(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047874_displayinteriorarraycontents_rage_atarray_unsigned_int_0_unsigne(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047876_mloidchangecb(ph: *mut Phantom); }
extern "C" { pub fn f_047880_cgameworld_findlocalplayercoors(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047888_cportaldebughelper_updatemlolist(ph: *mut Phantom); }
extern "C" { pub fn f_047900_cportaldebughelper_initwidgets_getscenegraphnodedebugname_func_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047902_ciplcullboxentry_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_047915_rage_atarray_ciplcullboxentry_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047916_ciplcullboxentry_op_ciplcullboxentry_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047917_ciplcullbox_update_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047918_ciplcullboxentry_recursivelyaddculleddeps_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047929_cscenestreamermgr_dumpstreaminginterests_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047930_void_std_2_introsort_std_2_classicalgpolicy_bucketentrysort_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_047932_cactiveentitylist_removeentity_fwlink_rage_activeentity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047934_cscenestreamermgr_streamerdebugprintf_bool_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_047939_cstreamvolume_extractentitiesfrommapdata_rage_atarray_centity_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047941_cscenestreamermgr_dtorcscenestreamermgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047951_cgamescan_findstartingscenenode_crenderphasescanned_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047952_cgamescan_setupstreamingvolumecullshapes_crenderphasescanned_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_047963_cgameworld_removeoutofmapvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047964_cgameworld_removeoutofmapobject_cobject_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047965_cgameworld_findfollowplayervehicle(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047967_cgameworld_processattachmentafterallmovement_cphysical_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047975_cgameworld_clearcarsfromarea_rage_vector3_const_ref_float_bool_b(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_047976_cgameworld_extinguishallcarfiresinarea_rage_vector3_const_ref_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_047980_cgameworld_logclearcarfromarea_char_const_ptr_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_047981_cgameworld_deletethisvehicleifpossible_cvehicle_ptr_bool_bool_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: f32, a7: i64); }
extern "C" { pub fn f_047982_cgameworld_findlocalplayervehicle_bool(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047983_isvehiclescripttrain_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047994_cgameworld_createplayer_rage_vector3_ref_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047995_cgameworld_changepedmodel_cped_ref_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_048000_cheightmap_unsigned_char_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_048007_cheightmap_unsigned_char_load_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_048008_cheightmap_unsigned_char_cheightmapheader_byteswap(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_048009_cheightmap_unsigned_char_converttorle(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_048012_cheightmap_unsigned_char_addwidgets_rage_bkbank_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_048015_cheightmapboundsinterface_addwidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_048037_cgameworldheightmap_getminheightfromworldheightmap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_048038_getheightmap_float_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> i64; }
extern "C" { pub fn f_048039_cheightmap_unsigned_char_getminheightatbound_float_float_float_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32) -> f32; }
extern "C" { pub fn f_048040_cheightmap_unsigned_char_getmaxheightatbound_float_float_float_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32) -> f32; }
extern "C" { pub fn f_048043_cgameworldheightmap_getmaxheightfromworldheightmap_float_float_f(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_048045_cheightmap_unsigned_char_reset_int_int_int_float_float_float_flo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: i32); }
extern "C" { pub fn f_048046_cheightmap_unsigned_short_reset_int_int_int_float_float_float_fl(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: i32); }
extern "C" { pub fn f_048047_cheightmap_unsigned_char_save_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_048048_void_cheightmap_exportddsdata_unsigned_short_unsigned_short_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64, a7: i32); }
extern "C" { pub fn f_048049_cheightmap_exportddsmask_unsigned_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_048050_void_cheightmap_exportddsdata_float_float_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: f32, a7: f32, a8: i32); }
extern "C" { pub fn f_048051_void_cheightmap_exportddsmaskraw_unsigned_long_unsigned_long_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_048052_void_cheightmap_exportddsmaskraw_unsigned_short_unsigned_short_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_048053_void_cheightmap_exportddsdensitymap_unsigned_char_unsigned_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_048054_void_cheightmap_exportddsdensitymap_unsigned_short_float_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_048055_cheightmap_exportddsidmap_unsigned_int_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_048056_void_cheightmap_exportddsdata_unsigned_char_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64, a7: i32); }
extern "C" { pub fn f_048061_cwatermap_getheight_bool_rage_vec3v_const_ref_rage_grcviewport_c(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> f32; }
extern "C" { pub fn f_048215_cextracontentmanager_dtorcextracontentmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_048216_rage_atarray_cmountablecontent_0_unsigned_short_destruct_cmounta(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_048218_ssetupdata_op_ssetupdata_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_048219_rage_atarray_contentchangesetgroup_0_unsigned_short_copy_content(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_048220_void_std_2_iterops_std_2_classicalgpolicy_iter_swap_abi_nqn22010(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_048221_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_048222_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_048223_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_048224_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_048225_cmountablecontent_cmountablecontent_cmountablecontent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_048226_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_048227_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_048228_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_048229_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_048230_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_048231_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_048233_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_048234_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bucketentr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_048235_bool_creplaymgrinternal_handleentityforfx_cpacketplayerswitch_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_048240_cheightmap_importddsmask_unsigned_char_ptr_char_const_ptr_int_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_048241_cheightmap_unsigned_short_converttorle(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_048242_cheightmap_unsigned_short_save_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_048244_apps_commands_commanddatavalid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_048245_apps_commands_commandgetint_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_048246_apps_commands_commandgetfloat_char_const_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_048247_apps_commands_commandgetstring_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_048248_apps_commands_commandsetint_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_048249_apps_commands_commandsetfloat_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_048250_apps_commands_commandsetstring_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_048279_audio_commands_commandplaypedringtone_char_const_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_048286_audio_commands_commandsetmicrophoneposition_bool_rage_scrvector(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_048338_audio_commands_commandsayambientspeechandclonenative_int_char_co(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_048340_audio_commands_commandsayambientspeechnative_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_048341_audio_commands_commandsayambientspeechwithvoicenative_int_char_c(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_048346_audio_commands_commandplaypain_int_int_float_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_048353_audio_commands_commandsetpedracetopedvoicegroup_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_048359_audio_commands_commandisanyspeechplaying_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_048361_audio_commands_commanddoescontextexistforthisped_int_char_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_048364_audio_commands_commandplayanimalvocalization_int_int_char_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_048389_audio_commands_commandlockradiostationtracklist_char_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_048390_audio_commands_commandlockradiostation_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_048412_audio_commands_commandsetvehicleradiostation_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_048426_audio_commands_commandsetvehiclestartuprevsound_int_char_const_p(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_048472_audio_commands_commandsethornpermanentlyon_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_048473_audio_commands_commandsethornpermanentlyontime_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_048501_audio_commands_commandaddentitytomixgroup_int_char_const_ptr_flo(ph: *mut Phantom, a0: i32, a1: i64, a2: f32); }
extern "C" { pub fn f_048540_bool_creplaymgrinternal_recordfx_cpacketscriptaudiospecialeffect(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_048895_brain_commands_commandallocatescripttorandomped_char_const_ptr_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_048896_brain_commands_commandregisterobjectscriptbrain_char_const_ptr_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_048898_brain_commands_commandregisterworldpointscriptbrain_char_const_p(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_048899_brain_commands_commandisworldpointwithinbrainactivationrange(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_048922_camera_commands_commandrenderscriptcams_bool_bool_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_048923_camera_commands_commandstoprenderingscriptcamsusingcatchup_bool(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_048926_camera_commands_commandcreatecamerawithparams_int_rage_scrvector(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_048927_camera_commands_commandcreatecamwithparams_char_const_ptr_rage_s(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_048931_camera_commands_commandsetcamactive_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_048943_camera_commands_commandsetcamparams_int_rage_scrvector_const_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_048945_camera_commands_commandsetcamrotation_int_rage_scrvector_const_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_048947_camera_commands_commandsetcamnearclip_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_048949_camera_commands_commandforcecamfarclip_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_048958_camera_commands_commandsetdofoverriddenfocusdistance_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_048959_camera_commands_commandsetdofoverriddenfocusdistanceblendlevel_i(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_048960_camera_commands_getoverriddendepthoffieldsettingsfromcameraindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_048966_camera_commands_commandattachcamtoentity_int_int_rage_scrvector(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_048967_camera_commands_commandattachcamtopedbone_int_int_int_rage_scrve(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_048968_camera_commands_commandhardattachcamtopedbone_int_int_int_rage_s(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_048969_camera_commands_commandhardattachcamtoentity_int_int_rage_scrvec(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_048970_camera_commands_commandattachcamtovehiclebone_int_int_int_bool_r(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_048972_camera_commands_commandsetcaminheritrollvehicle_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_048973_camera_commands_commandpointcamatcoord_int_rage_scrvector_const(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_048974_camera_commands_commandpointcamatentity_int_int_rage_scrvector_c(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_048975_camera_commands_commandpointcamatpedbone_int_int_int_rage_scrvec(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_048976_camera_commands_commandstopcampointing_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_048978_camera_commands_commandsetcamcontrolsminimapheading_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_050119_entity_commands_setentityasnolongerneeded_cphysical_ptr_int_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_050160_entity_commands_commandsetentityalphaentity_centity_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_052872_misc_commands_commandclearareaofobjects_rage_scrvector_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_057356_ped_commands_setpedcanbeknockedoffvehicle_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_061626_cvehicledeformation_lockdamagetexture_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_063736_gtathread_getthreadwiththreadid_rage_scrthreadid(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063772_cthescripts_getcurrentgtascripthandler(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063778_cscriptbraindispatcher_itemwaitingforbrain_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063794_cthescripts_registerentity_cphysical_ptr_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_063796_cthescripts_unregisterentity_cphysical_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063799_cthescripts_getcurrentscriptname(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063800_cthescripts_getcurrentscriptnameandprogramcounter(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063801_cmissionreplaystats_clearreplaystatsstructure(ph: *mut Phantom); }
extern "C" { pub fn f_063816_cthescripts_findnetworkplayerped_int_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063823_cscripteulers_matrixfromeulers_rage_matrix34_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063824_cscripteulers_matrixtoeulers_rage_matrix34_const_ref_rage_eulera(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063835_cscriptglobaltamperchecker_uncheckedscope_uncheckedscope(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_063836_cscriptglobaltamperchecker_uncheckedscope_dtoruncheckedscope(ph: *mut Phantom); }
extern "C" { pub fn f_063854_cgamescripthandler_registerscriptresourceandgetref_rage_scriptre(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063861_cscriptareas_isvecin3dangledarea_rage_vector3_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_063862_cscriptareas_ispointinangledarea_rage_vector3_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_063867_cscriptsforbrains_enablebrainset_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_063868_cscriptsforbrains_disablebrainset_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_063869_cscriptsforbrains_addscriptbrainforentity_rage_atfinalhashstring(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: f32, a6: i32); }
extern "C" { pub fn f_063870_cscriptsforbrains_checkandapplyifnewentityneedsscript_centity_pt(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063872_cscriptsforbrains_isobjectwithinactivationrange_cobject_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063873_cscriptsforbrains_isworldpointwithinactivationrange_c2deffect_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063874_cscriptsforbrains_getscriptfilename_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063875_cscriptbraindispatcher_reactivateallobjectbrainsthatarewaitingti(ph: *mut Phantom); }
extern "C" { pub fn f_063876_cscriptbraindispatcher_reactivatenamedobjectbrainswaitingtillout(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063899_cscriptpeds_setpedcoordinates_cped_ptr_float_float_float_int_boo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_063900_cscriptentities_clearspaceformissionentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063976_cscriptdebug_setcontentsoffadecommandtextwidget_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064143_cgamescripthandlermgr_getremotescriptinfo_cgamescriptid_const_re(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_064241_cscriptstreamingresource_cscriptstreamingresource_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_064290_cscriptresource_camera_cscriptresource_camera_char_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_064291_cscriptresource_camera_cscriptresource_camera_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_064425_cgamescriptresource_dtorcgamescriptresource_102634(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064504_ccustomshadereffectbasetype_setupfordrawable_rage_rmcdrawable_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_064505_ccustomshadereffectweapontype_initialise_rage_rmcdrawable_ptr_cb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_064566_ccustomshadereffectped_updatepeddamage_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064665_cshaderedit_regenshaderwidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064690_cshaderlib_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_064736_csettingsmanager_csettingsmanager(ph: *mut Phantom); }
extern "C" { pub fn f_064737_csettingsmanager_getsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064760_cislandhopper_getcurrentactiveisland_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064761_cislandhopper_isanyislandactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064774_cloadedmodelgroup_countmembers_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064775_cloadedmodelgroup_getmember_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_064776_cpopulationstreaming_requestvehicledriver_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064779_cpopulationstreaming_tellstreamingaboutdeletablevehicles(ph: *mut Phantom); }
extern "C" { pub fn f_064781_cpopulationstreaming_adddiscardedped_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064788_cpopulationstreaming_iscarmodelneededcurrently_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064832_cstreaming_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064858_cprioritizedclipsetrequestmanager_processrequests_cprioritizedcl(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_064859_cprioritizedclipsetrequestmanager_addrequesthelperifnoneexists_c(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064866_cscriptpreloaddata_cscriptpreloaddata_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_064867_cscriptpreloaddata_dtorcscriptpreloaddata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064869_cscriptpreloaddata_haspreloadfinished(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064871_cscriptpreloaddata_addtxd_rage_strlocalindex_int_ref_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_064872_cscriptpreloaddata_adddwd_rage_strlocalindex_int_int_ref_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_064875_cscriptpreloaddata_gethandle_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_064909_cstreaming_shouldloadstaticdata(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_064911_cstreaming_getreloadpackfilesonrestart(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_064913_ckeyboard_getkeyjustdown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_064916_rage_czonedassetmanager_clearrequests(ph: *mut Phantom); }
extern "C" { pub fn f_064925_cstreamingdebug_displayobjectsinmemory_rage_fistream_ptr_bool_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_064928_cstreaming_purgerequestlist_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_064929_cstreaming_setobjectisdeletable_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064931_cstreaming_setmissiondoesntrequireobject_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064933_cstreaming_requestobject_rage_strlocalindex_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_064937_cstreaming_loadobject_rage_strlocalindex_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064942_cstreamingcleanup_deletealldrawablereferences(ph: *mut Phantom); }
extern "C" { pub fn f_064948_cstreamingcleanup_deleteentityanddependencies_deletionmap_ref_bu(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_064951_cstreamingcleanup_requestflush_bool(ph: *mut Phantom); }
extern "C" { pub fn f_065010_cstreamingdebug_getmemorystats_rage_atarray_unsigned_int_0_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_065337_ccontrol_isvehicleattackinputdown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065360_ccontrol_resetkeyboardmousesettings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065367_ccontrol_stopplayerpadshaking_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_065373_ccontrol_disableallinputs_unsigned_int_rage_iovalue_disableoptio(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_065374_ccontrol_setinputexclusive_int_rage_iovalue_disableoptions_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_065375_ccontrol_doenableinput_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_065376_ccontrol_getinputname_rage_inputtype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065384_ccontrol_isinputdisabled_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_065385_ccontrol_getvalue_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_065390_ccontrol_getactivelayout_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065444_ccontrolmgr_update(ph: *mut Phantom); }
extern "C" { pub fn f_065445_ccontrolmgr_setmainplayerindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_065446_ccontrolmgr_getmainplayercontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065461_ccontrolmgr_getmainfrontendcontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065469_ccontrolmgr_startplayerpadshakebyintensity_unsigned_int_float_in(ph: *mut Phantom, a0: i32, a1: f32, a2: i32); }
extern "C" { pub fn f_065614_cfilemgr_readline_void_ptr_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_065615_cfilemgr_openfile_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_065616_cfilemgr_readline_void_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_065617_cfilemgr_setdir_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065621_cfilemgr_openfileforwriting_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065988_covervariationsinfo_initcovervariations_rage_atarray_rage_fwmvte(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_065991_ccombatdirector_removeped_cped_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_065995_ctaskthreatresponse_setfleefromcombat_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065996_ccombatmountedattackergroup_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065997_ccombatmeleegroup_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065999_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_ccombat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_066000_ccombattaskmanager_findpedstoremove_ccombattaskmanager_scombatpe(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_066001_ccombattaskmanager_getnumlawpedsincombat_ccombattaskmanager_scom(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_066004_ccombattaskmanager_countpedsincombatwithtarget_cped_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_066007_ctaskmelee_setqueuetype_ctaskmelee_queuetype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_066057_ctacticalanalysisnavmeshpoints_setpointsfromscript(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066059_ctacticalanalysisnavmeshpoints_resetpoint_ctacticalanalysisnavme(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066060_ctacticalanalysisnavmeshpoints_setpoint_ctacticalanalysisnavmesh(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066061_ctacticalanalysis_startlineofsighttest_casyncprobehelper_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066062_ccoverpoint_getcoverpointposition_rage_vector3_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066063_ccover_findcoordinatescoverpointnotreserved_ccoverpoint_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_066066_ccover_removecoverpoint_ccoverpoint_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066067_ccoverpoint_getcoverdirectionvector_rage_vec3v_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066068_ccoverpoint_reservecoverpointforped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066096_ctaskcover_ctaskcover_cweapontarget_const_ref_int_ctaskcover_efo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_066297_cdeathsourceinfo_cdeathsourceinfo_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_066299_ctaskdyingdead_ctaskdyingdead_cdeathsourceinfo_const_ptr_int_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_066336_ctaskdyingdead_dealwithdeactivatedphysics_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_066683_ccoverpointscontainer_checkifnooverlapandgetgridcell_rage_vector(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_066684_ccoverpointscontainer_movecoverpointtoanothergridcellifrequired(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066686_ccover_addcoverpointfromobject_cobject_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066687_ccover_ispointwithinvalidarea_rage_vector3_const_ref_ccoverpoint(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_066691_ccover_addcoverpointfromvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066694_ccover_addcoverpoint_ccover_coverpooltype_ccoverpoint_ecovertype(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i64) -> i32; }
extern "C" { pub fn f_066695_ccover_removecoverpointwithindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_066706_ccoverpoint_releasecoverpointforped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066709_ccoverpoint_copy_ccoverpoint_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066710_ccoverpoint_pedclosesttofacingleft_cped_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066733_cdynamiccoverhelper_dolineprobetest_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i64; }
extern "C" { pub fn f_066753_cdynamiccoverhelper_setcoverentryentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066841_ctaskincover_getstaticcoverpointforplayer_cped_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067049_ccoverfinderfsm_validatecoverpoints(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067054_ccoverfinderfsm_getlastsearchresult_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_067213_ctaskvehiclechase_ismakingaggressivemove_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067369_crandomeventmanager_flushcopasserts(ph: *mut Phantom); }
extern "C" { pub fn f_067534_ctaskmovecombatmounted_gettarget_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067604_ctaskaimgunblindfire_getisreadytofire_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067931_ctaskplayeronfoot_ctaskplayeronfoot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067955_ctaskmobilephone_isrunningmobilephonetask_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067964_cclimbhandholddetected_dtorcclimbhandholddetected(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068003_ctaskunalerted_ctaskunalerted_ctask_ptr_cscenariopoint_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_068126_ctaskdonothing_ctaskdonothing_int_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_068157_ctasklist_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068161_ctasksequencelist_ctasksequencelist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068163_ctasksequencelist_addtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068173_ctasksequences_init(ph: *mut Phantom); }
extern "C" { pub fn f_068203_ctaskusesequence_ctaskusesequence_ctasksequencelist_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_068238_ctaskshoveped_istesting(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_068442_ctaskmotionped_checkfordiving(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068454_ctaskmotionbase_setonfootclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_068456_ctaskmotionbase_clearoverrideweaponclipset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068457_ctaskmotionbase_resetonfootclipset_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_068460_ctaskmotionbase_setdefaultclipsets_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068461_ctaskmotionbase_setalternateclip_ctaskmotionbase_alternateclipty(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_068463_ctaskmotionbase_getalternatecliplooping_ctaskmotionbase_alternat(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_068465_ctaskmotionbase_cpersistentdata_cpersistentdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068466_ctaskmotionbase_cpersistentdata_init_ctaskmotionbase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068467_ctaskmotionbase_getdefaultonfootclipset_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_068468_ctaskmotionbase_cpersistentdata_apply_ctaskmotionbase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068510_ctaskmotionswimming_clearscubagearvariationforped_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068512_ctaskmotionswimming_isscubagearvariationactiveforped_cped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068515_ctaskmotionswimming_setscubagearvariationforped_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068665_ctaskmotionped_ctaskmotionped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068904_cjetpackmanager_createjetpackpickup_rage_matrix34_const_ref_cped(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068906_cjetpack_destroyobject(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069067_ctaskmovestandstill_ctaskmovestandstill_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_069108_cdropdown_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069110_cdropdowndetector_deleteproberesults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069394_ctaskmovewithinattackwindow_getattackwindowminmax_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_069440_cclimbdetector_process_rage_matrix34_ptr_spedtestparams_ptr_sped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_069529_ctaskparachute_clearparachutepackvariationforped_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069532_ctaskparachute_ispeddependentonparachute_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069541_ctaskparachute_shouldignorepedcollisionwithentity_centity_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069542_ctaskparachute_detachpedfromparachute(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069691_ctaskfollowleaderanymeans_ctaskfollowleaderanymeans_cpedgroup_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_069734_cclimbdetector_sshapetestresult_dtorsshapetestresult(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069961_ctaskfallover_pickfallanimation_cped_ptr_ctaskfallover_efallcont(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_069977_ctaskfallover_applyinitialvehicleimpact_cvehicle_ptr_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069979_ctaskfallandgetup_ctaskfallandgetup_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_070373_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_070399_ctasknmcontrol_ctasknmcontrol_unsigned_int_unsigned_int_rage_ait(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: f32) -> i64; }
extern "C" { pub fn f_070401_ctasknmbehaviour_canuseragdoll_cped_ptr_eragdolltriggertypes_cen(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_070422_cnmtuningset_cnmtuningset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070423_cnmtuningset_dtorcnmtuningset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070469_ctasknmbehaviour_sendstartmessages_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070497_ctasknmbehaviour_calculateactivationforce_cped_const_ptr_eragdol(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32) -> f32; }
extern "C" { pub fn f_070498_ctasknmbehaviour_doesgroundphysicalmatch_cphysical_const_ptr_cve(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_070529_ctasknmhighfall_ctasknmhighfall_unsigned_int_cgrabhelper_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_070530_ctasknmbalance_ctasknmbalance_unsigned_int_unsigned_int_centity(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64, a6: f32, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_070534_ctasknmbehaviour_addtoragdollpool_ctasknmbehaviour_eragdollpool(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_070536_ctasknmbehaviour_removefromragdollpool_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070558_ctasknmbrace_tunables_stuckonvehicle_dtorstuckonvehicle(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070561_ctasknmbrace_ctasknmbrace_unsigned_int_unsigned_int_centity_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_070566_ctasknmbrace_setsideswipeimpulsepos_rage_vec3v_const_ref_cped_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_070579_ctasknmbrace_tunables_stuckonvehicle_op_ctasknmbrace_tunables_st(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070739_ctasknmrelax_ctasknmrelax_unsigned_int_unsigned_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_070808_rage_atarray_rage_atbinarymap_ctasknmsimple_tunables_tuning_rage(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_070809_ctasknmsimple_tunables_tuning_op_ctasknmsimple_tunables_tuning_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070810_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_070836_ctasknmthroughwindscreen_ctasknmthroughwindscreen_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_070910_cpedswaitingtobeoutofsightmanager_cpedinfo_dtorcpedinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072482_cscenarioinfomanager_getscenarioindex_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_072503_cscenariopointmanager_updatecachedregionptrs_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072525_cscenariopoint_destroyforeditor_cscenariopoint_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072528_cscenariopoint_dtorcscenariopoint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072530_cscenariomanager_onremovescenario_cscenariopoint_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072539_cscenariomanager_getscenarioinfo_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_072671_ctaskpolice_ctaskpolice_ctaskunalerted_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_072672_ctaskarmy_ctaskarmy_ctaskunalerted_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_072673_ctaskambulancepatrol_ctaskambulancepatrol_ctaskunalerted_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_072674_ctaskfirepatrol_ctaskfirepatrol_ctaskunalerted_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_072743_cscenariopointregion_applyoverridesforentity_centity_ref_bool_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_072760_cscenariopointmanager_findgroupbynamehash_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_072775_cscenariopointmanager_addentitypoint_cscenariopoint_ref_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072911_cscenariopointmanager_load(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072922_cvehiclescenariomanager_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072924_cclonedscenariofsminfobase_getscenariopoint_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_072928_ctaskscenario_getscenarioinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073669_ctaskswat_ctaskswat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_074161_ctask_ctask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_074785_cnmtuningset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074800_ctasknmbrace_tunables_vehicletypeoverrides_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074931_ctasknmsimple_tunables_tuning_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_075342_ctaskhelperfsm_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075349_cmovenetworkhelper_getfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_075350_ccliprequesthelper_ccliprequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075351_ccliprequesthelper_dtorccliprequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075354_cgetupsetrequesthelper_cgetupsetrequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075355_cgetupsetrequesthelper_dtorcgetupsetrequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075356_cgetupsetrequesthelper_request_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075361_cnavmeshclosestpositionhelper_startclosestpositionsearch_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_075364_cnavmeshclosestpositionhelper_getsearchresults_searchstatus_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_075367_cnavmeshclosestpositionhelper_resetsearch(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075369_cprioritizedclipsetrequesthelper_releaseclipset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075373_cprioritizedclipsetrequesthelper_requestclipset_rage_fwmvtemplat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075398_cvehiclefollowroutehelper_getvehiclebonnetposition_cvehicle_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_075445_casyncprobehelper_casyncprobehelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075446_casyncprobehelper_dtorcasyncprobehelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075447_casyncprobehelper_resetprobe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075448_casyncprobehelper_starttestlineofsight_worldprobe_cshapetestprob(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075449_casyncprobehelper_getproberesults_probestatus_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075451_casyncprobehelper_getproberesultswithintersectionandnormal_probe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_075469_csimpleblendhelper_csimpleblendhelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075470_cmovenetworkhelper_cmovenetworkhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075474_cmovenetworkhelper_setfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_075484_cgenericcliphelper_cgenericcliphelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075487_cgenericcliphelper_blendinclipbyclip_cdynamicentity_ptr_rage_crc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_075526_cvehiclecliprequesthelper_update_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_075577_ccoverpointstatushelper_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075599_ctimehelpers_gettimesince_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_075618_cpedswaitingtobeoutofsightmanager_cpedinfo_done(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075621_ctaskmanager_processpostmovement_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_075622_ctaskmanager_processpostcamera_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_075623_ctaskmanager_processprerender2_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_075624_ctaskmanager_processpostprerender_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_075654_ctasktreeped_ctasktreeped_int_rage_fwentity_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_075666_ctunablesgroup_addwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075673_cnmtuningsetgroup_revert_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075674_cnmtuningsetgroup_append_cnmtuningset_ptr_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075695_ctaskleaveanycar_ctaskleaveanycar_int_cvehicleenterexitflags_fla(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_075702_ctaskinvehiclebasic_ctaskinvehiclebasic_cvehicle_ptr_bool_ccondi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_075727_ctaskcardrivewander_ctaskcardrivewander_cvehicle_ptr_int_float_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_075744_ctaskplayerdrive_ctaskplayerdrive(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075752_ctaskvehiclefsm_computetargettransforminworldspace_cvehicle_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_075824_ctaskvehiclefsm_findgroundpos_centity_const_ptr_cped_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_075974_ctaskvehiclefsm_getdoorforentrypointindex_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_076006_ctaskentervehicle_isconsideredgettinginvehicleforambientscale(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076144_ctaskexitvehicleseat_getclipandphaseforstate_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076865_ctaskgun_getisfiring_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077081_cstatsvehicleusage_clear_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077083_cstatsmgr_findvehiclerecord_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077085_cstatsmgr_startnewvehiclerecords_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077086_cstatsmgr_checkwritevehiclerecords_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077089_cstatsmgr_incrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077093_cstatsmgr_clearplayingtimestats(ph: *mut Phantom); }
extern "C" { pub fn f_077095_cstatsmgr_isplayeractive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077103_cstatsmgr_vehiclehasbeendriven_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_077153_statid_char_getstatid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077182_cstatsdatamgr_onsynchcommunitystats_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_077193_cstatsdatamgr_iskeyvalid_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077197_cstatsdatamgr_getstatisnotnull_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077198_cstatsdatamgr_setstatiterdata_rage_atinbinarymap_sstatdataptr_st(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_077200_cstatsdatamgr_statiterfind_statid_const_ref_rage_atinbinarymap_s(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077201_cstatsdatamgr_getstatdata_statid_const_ref_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_077329_statid_set_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077331_statsinterface_getintstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077338_statsinterface_getcurrentmultiplayercharaterslot(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077342_statsinterface_setstatdata_statid_const_ref_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077348_statsinterface_setstatdata_statid_const_ref_unsigned_long_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_077354_statsinterface_getfloatstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_077359_statsinterface_getuint64stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077361_statsinterface_syncprofilestats_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077371_statsinterface_setstatsmodelprefix_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077373_statsinterface_getstatsmodelhashid_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077381_statsinterface_applypedscardata_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077382_statsinterface_getcloudsavegameservice(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077414_cstatssavesmgr_save_estatssavetype_unsigned_int_esavetypes_unsig(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_077417_cstatssavesmgr_load_estatsloadtype_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077430_moneyinterface_getvcwalletbalance_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077431_moneyinterface_getvcbankbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077475_cnumberwithinmessage_set_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077479_ctextfile_get_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077481_ctextfile_get_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_077488_cmessages_insertnumbersandsubstringsintostring_char_const_ptr_cn(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32); }
extern "C" { pub fn f_077521_csavinggamemessage_clear(ph: *mut Phantom); }
extern "C" { pub fn f_077523_csavinggamemessage_issavingmessageactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077533_ctextlayout_ctextlayout(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077535_ctextlayout_dtorctextlayout(ph: *mut Phantom); }
extern "C" { pub fn f_077537_ctextlayout_setscale_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077540_ctextlayout_render_rage_vector2_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077565_ctextfile_doestextlabelexist_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077569_ctextfile_addextracontenttext_char_const_ptr_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_077612_ctimecycle_calcambientscale_rage_vec3v_const_ref_rage_fwinterior(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077622_cvisualsettings_get_char_const_ptr_float_bool_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_077633_ctimecycleasyncquerymgr_addnonpedentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077742_cdriverpersonality_finddriverability_cped_const_ptr_cvehicle_con(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_077743_cdriverpersonality_finddriveraggressiveness_cped_const_ptr_cvehi(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_077745_cdriverpersonality_findmaxcruisespeed_cped_const_ptr_cvehicle_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> f32; }
extern "C" { pub fn f_077789_cjunction_findentranceindexwithnode_rage_cnodeaddress_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077791_cpathfind_checkpathnodeintegrityforzone_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077792_cpathfind_testalllinks_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077795_cjunction_gettrafficlightcommand_cvehicle_const_ptr_int_bool_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_077803_cvehicleintelligence_getactivetask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077806_cjunction_findtrafficlightentranceids_rage_vector3_const_ref_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_077812_cpathfind_findnodeclosesttocoors_rage_vector3_const_ref_float_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: f32, a11: i32, a12: i64, a13: i32, a14: i32); }
extern "C" { pub fn f_077813_cpathfind_findnodeslinkindexbetween2nodes_rage_cnodeaddress_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_077821_cjunctions_findjunctionfromnode_rage_cnodeaddress_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077827_cjunctions_getjunctionbyindex_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077829_cjunctions_getjunctionatpositionforrailwaycrossing_rage_vector3(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077845_cpathfind_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077847_cpathfind_findrouteendnodeforgpssearch_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_077848_cpathfind_dopathsearch_rage_vector3_const_ref_rage_cnodeaddress(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: f32, a8: i64, a9: i32, a10: i32, a11: i64, a12: i32, a13: i32, a14: i32, a15: i32, a16: i32, a17: i32, a18: i32, a19: i32, a20: i32, a21: i64, a22: i32, a23: i64, a24: i64, a25: i32, a26: i32); }
extern "C" { pub fn f_077849_cpathfind_generatedirections_int_ref_unsigned_int_ref_float_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_077860_cpathfind_findnodepairtomatchvehicle_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: f32, a8: i32, a9: i64, a10: i64) -> i32; }
extern "C" { pub fn f_077870_cpathfind_findnearestgroundheightforbuildpaths_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_077873_cpathfind_updatestreaming_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077889_cpathfind_recordnodesincircle_rage_vector3_const_ref_float_int_r(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_077901_cpathfind_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077923_void_rage_atplaceptrcreator_void_ptr_cpathzonemappingarray_void(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077930_cvehicleailod_getdummymode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077934_cvehicleailodmanager_shoulddofullupdate_cvehicle_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077936_cvehicleailodmanager_disabletimeslicingimmediately_cvehicle_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077959_cvehicleintelligence_isonhighway_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077974_ctaskvehiclegotohelicopter_ctaskvehiclegotohelicopter_svehiclemi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_077982_cvehicleintelligence_createcruisetask_cvehicle_ref_svehiclemissi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_077991_ctaskvehiclegotoplane_ctaskvehiclegotoplane_svehiclemissionparam(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: f32) -> i64; }
extern "C" { pub fn f_078012_ctaskvehiclestop_ctaskvehiclestop_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_078018_cheliintelligence_islanded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078024_cvehiclenodelist_cvehiclenodelist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078053_aideferredtasks_runall(ph: *mut Phantom); }
extern "C" { pub fn f_078431_ctaskvehiclepursue_setdesiredoffset_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078433_ctaskvehiclepursue_calculatetargetmatrix_cphysical_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078786_cvehicle_getnumcontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078818_cautomobile_prerendersuspensionmod_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078827_cvehicle_setiswrecked(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078831_cvehicle_getreferenceframevelocity_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078871_cvehicle_containslocalplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078876_camphibiousquadbike_iswheeltransitionfinished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078886_cvehicle_canpedsstandontop_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078889_cvehicle_getdoorhierachyidfromimpact_cvehicle_const_ref_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078892_cvehicle_hasramp_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078893_cheli_getiscargobob_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078918_cvehicle_setcardoorlocks_ecarlockstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078926_cvehicle_calculatenonarticulatedmaximumextents_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078932_cvehicle_getvelocityincludingreferenceframe_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078933_cvehicle_isupsidedown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078934_cvehicle_isonitsside_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078936_cvehicle_ishornon_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078945_cvehicle_isdriveraplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078946_cvehicle_hasalivelawpedsinit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078951_cvehicle_triggercaralarm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078953_cvehicle_hasrocketboost(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078954_cvehicle_hasjump(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078971_cvehicledebug_getfocusvehicle(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_078974_cvehicledamage_getenginehealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_078981_cvehicle_hascontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078998_cvehicle_switchengineoff_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079000_cvehicle_istank_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079007_cvehicle_getnumberofpassenger_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079023_cvehicle_getattachedtrailer_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079039_cvehicle_isturretseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079040_cvehicle_getspecialflightmoderatio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_079065_cvehicle_getdoorfromid_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079070_cvehicle_getseatanimationinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079071_cvehicle_isentryindexvalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079097_ccardoor_setflag_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079108_cbike_setbikeonsidestand_bool_bool_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_079117_cbike_isunderwater_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079130_cvehicle_getlayoutinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079134_cvehicle_getdriverseat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079157_cvehicle_getisjetski_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079198_ccargenerator_switchon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079229_cvehicle_finishaiupdate_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079230_cthecargenerators_createcargenerator_float_float_float_float_flo(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i32, a14: i32, a15: i32, a16: i32, a17: i32, a18: i64, a19: i32, a20: i32, a21: i32) -> i32; }
extern "C" { pub fn f_079232_ccargenerator_removeregdentities(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079233_ccargenerator_dtorccargenerator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079246_cthecargenerators_instantfill(ph: *mut Phantom); }
extern "C" { pub fn f_079266_ccardoor_clearflag_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079267_ccardoor_getisintact_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079271_ccardoor_getlocalmatrix_cvehicle_const_ptr_rage_matrix34_ref_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_079335_chandlingdata_gethandlingdata_ehandlingtype(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079358_csearchlight_processsearchlight_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079422_cvehicle_getvehiclegadget_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079472_cplane_getverticalflightmodeavaliable_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079479_caircraftdamagebase_breakoffsection_cplane_ptr_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_079513_cvehicle_getisusingscriptautopilot_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079551_cvehicle_getentryaniminfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079616_cvehicletrailerattachpoint_getattachedtrailer_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_079642_cvehicle_removepedfromseat_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_079643_cvehicle_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079644_cvehicle_processseatcollisionbound_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079651_ctrain_removetrainsfromarea_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079713_cvehicle_hasnitrousboost(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079714_cvehicle_hassideshunt_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079739_cvehicledeformation_handledamageadded_void_ptr_bool_bool_bool_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_079740_cvehicledeformation_savedamagetexture_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079745_cvehiclelayoutinfo_getseatanimationinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079746_cvehicle_doesvehiclehaveaconvertibleroofanimation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079758_cvehicle_getconvertibleroofstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079759_cvehicle_canbedeletedspecial_bool_bool_bool_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_079772_cvehicle_getvehicleorientation_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079781_cvehicle_hasmissioncharsinit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079791_cvehicle_isdriver_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079795_cvehiclepopulation_adddrivingflagsforambientvehicle_int_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079812_cvehicle_getentryinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079814_cvehicle_addpedinseat_cped_ptr_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_079816_cvehicle_getseatinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079842_cvehicle_extinguishcarfire_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079843_cvehicle_carhasroof_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079845_cvehicle_ispersonalvehicle(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079849_cvehicle_requestdials_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079854_cvehicle_containsped_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079862_cvehicle_finishgliding(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079891_cvehicledebug_rendertestcapsuleatposition_cped_const_ptr_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32); }
extern "C" { pub fn f_079904_cvehicle_placeonroadadjust_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_079908_cvehicle_getconvertibleroofprogress_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_079923_cvehicle_pregameworldaiupdate(ph: *mut Phantom); }
extern "C" { pub fn f_079945_cvehicle_setexclusivedriverped_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079948_cpoolhelpers_getvehiclepoolsize(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_080000_cvehicledamage_setenginehealth_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_080017_cvehiclefactory_cleardestroyedvehcache(ph: *mut Phantom); }
extern "C" { pub fn f_080080_cvehicleweaponmgr_getweaponsforseat_int_rage_atarray_cvehiclewea(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_080187_cvehiclefactory_createcar_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080401_cvehiclepopulation_setscriptcampopulationoriginoverride(ph: *mut Phantom); }
extern "C" { pub fn f_080403_cvehiclepopulation_catchvehicleremovedinview_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_080404_cvehiclepopulation_getdistancefromallplayers_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_080405_cvehiclepopulation_isvehiclewreckedorempty_cvehicle_ptr_float_bo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_080406_cvehiclepopulation_isentityvisibletoremoteplayer_centity_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_080429_cvehiclepopulation_instantfillpopulation(ph: *mut Phantom); }
extern "C" { pub fn f_080434_cvehiclepopulation_registerinterestingvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080442_cvehiclepopulation_hasinstantfillpopulationfinished(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_080445_cvehiclepopulation_removeallvehssoft(ph: *mut Phantom); }
extern "C" { pub fn f_080862_cvfxdeepsurfaceinfo_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080863_cvfxdeepsurfaceinfo_process_rage_vec3v_const_ref_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_080864_cvfxhelper_getdistsqrtocamera_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_080865_cvfxhelper_getmatrixfrombonetag_rage_mat34v_ref_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_080866_cvfxhelper_getmatrixfromboneindex_rage_mat34v_ref_bool_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_080870_cvfxhelper_getmatrixfromboneindex_lights_rage_mat34v_ref_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_080873_cvfxhelper_creatematfromvecz_rage_mat34v_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080874_cvfxhelper_creatematfromveczalign_rage_mat34v_ref_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_080875_cvfxhelper_createrelativemat_rage_mat34v_ref_rage_mat34v_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080879_cvfxhelper_getinterpvalue_float_float_float_bool(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_080880_cvfxhelper_isininterior_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080882_cvfxhelper_isentityincurrentvehicle_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080883_cvfxhelper_getwaterdepth_rage_vec3v_const_ref_float_ref_rage_fwe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_080884_cvfxhelper_getwaterz_rage_vec3v_const_ref_float_ref_rage_fwentit(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_080885_cvfxhelper_getoceanwaterz_rage_vec3v_const_ref_float_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_080952_cvfxentity_loaddata(ph: *mut Phantom); }
extern "C" { pub fn f_080953_cvfxfogvolumeinfomgr_loaddata(ph: *mut Phantom); }
extern "C" { pub fn f_081161_vfxpedfootinfo_getdecalinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081162_vfxpedwadeinfo_getptfxinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081164_cvfxpedinfo_getfootvfxinfo_vfxgroup_e_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_081165_cvfxpedinfomgr_getinfo_cped_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081357_cdecalmanager_addbang_vfxmaterialinfo_s_const_ref_vfxcollinfo_s(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_081389_cdecalmanager_addfootprint_int_int_float_float_rage_color32_unsi(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: f32, a4: i32, a5: i64, a6: i64, a7: i64, a8: i64, a9: i32, a10: f32) -> i32; }
extern "C" { pub fn f_081415_cbrightlights_register_brightlighttype_e_rage_vec3v_const_ref_ra(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: f32, a7: f32, a8: i32, a9: f32); }
extern "C" { pub fn f_081416_ccoronas_register_rage_vec3v_const_ref_float_rage_color32_float(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: f32, a4: f32, a5: i64, a6: f32, a7: f32, a8: f32, a9: i32); }
extern "C" { pub fn f_081431_cdistantlights2_loaddata_impl_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081462_cfiremanager_extinguisharea_rage_vec3v_const_ref_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_081464_cfiremanager_extinguishentityfires_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081517_cscrollbars_register_centity_ptr_cscrollbarattr_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_081519_cvfxlens_register_vfxlenstype_float_float_float_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_081521_cwatercannons_clearallwatercannonsprayinworld(ph: *mut Phantom); }
extern "C" { pub fn f_081576_ctvplaylistmanager_scaletorealtime_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_081609_clensflare_changesettings_unsigned_char(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081610_clensflare_transitionsettingsrandom(ph: *mut Phantom); }
extern "C" { pub fn f_081628_clodlightmanager_registerbrokenlightsforentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081651_cptfxmanager_removeptfxfromentity_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081711_cvfxtrail_registerpoint_vfxtrailinfo_t_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081719_cptfxmanager_relocateentityptfx_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081740_isentitysmashable_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_081746_cvehicleglassmanager_storecollision_vfxcollinfo_s_const_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081749_cvehicleglassmanager_storeexplosion_vfxexpinfo_s_const_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081763_cvfxblood_updateptfxbloodmist_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081782_cvfxentity_getptfxassetname_eparticletype_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_081783_cvfxentity_processptfxentityambient_centity_ptr_cparticleattr_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_081784_cvfxentity_triggerptfxentityshot_centity_ptr_cparticleattr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081788_cvfxentity_triggerptfxentityinwater_centity_ptr_cparticleattr_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081789_cvfxentity_triggerdecalgeneric_vfxentitygenericdecalinfo_s_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_081795_cvfxexplosion_processvfxexplosion_phgtaexplosioninst_ptr_bool_fl(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_081839_cvfxliquidattachinfo_update_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_081840_cvfxliquidattachinfo_set_vfxliquidattachtype_e_vfxliquidtype_e(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_081841_cvfxliquidattachinfo_getcurralpha_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_081842_cvfxmaterial_domtlscrapefx_centity_ptr_int_centity_ptr_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: f32, a11: f32, a12: f32, a13: f32, a14: f32); }
extern "C" { pub fn f_081847_cvfxped_triggerptfxpedbreath_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081848_cvfxped_updateptfxpedbreathwater_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081863_cvfxvehicle_updateptfxlighttrail_cvehicle_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_081966_bool_creplaymgrinternal_handleentityforfx_cpacketvehiclebadgereq(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_081967_bool_creplaymgrinternal_handleentityforfx_cpacketblooddecal_cpac(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_081968_bool_creplaymgrinternal_handleentityforfx_cpacketdecalremove_cpa(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082028_cexplosionmanager_addexplosion_cexplosionmanager_cexplosionargs(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082045_cairdefencemanager_ispositionindefencezone_rage_vec3v_unsigned_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082064_cpedweaponmanager_getequippedvehicleweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082065_cpedweaponmanager_getequippedweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082070_cweaponinfo_getammoinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082071_cpedweaponmanager_getequippedweapon_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082087_cexplosioninfomanager_getexplosiontagdata_eexplosiontag(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082114_cexplosionmanager_removeexplosion_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082115_cexplosionmanager_startexplosion_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082121_cexplosionmanager_testforexplosionsinsphere_eexplosiontag_rage_v(ph: *mut Phantom, a0: i32, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_082122_cexplosionmanager_removeexplosionsinarea_eexplosiontag_rage_vect(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_082123_cweapondamage_generatepeddamageevent_centity_ptr_cped_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64, a5: i64, a6: i64, a7: i32, a8: f32, a9: i64, a10: i32, a11: f32, a12: i32) -> f32; }
extern "C" { pub fn f_082128_rage_atarray_cexplosiontagdata_0_unsigned_short_construct_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082129_eexplosiontag_parse_rage_athashvalue_eexplosiontag(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082144_cfiringpattern_setfiringpattern_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082145_cfiringpatterninfo_getnumberofshotsperburst_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082157_cweapon_refillpedsweaponammoinstant_cped_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082159_cweapon_doreload_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082162_cweapon_updateshadervariables_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082179_cpedweaponmanager_destroyequippedweaponobject_bool_char_const_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_082184_cweapon_calcfirevecfromaimcamera_centity_const_ptr_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_082223_cpedinventory_removeweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082251_cammoitemrepository_getweaponammo_cweaponinfo_const_ptr_cped_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_082262_cweaponmanager_createweaponunarmed_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082263_cweaponmanager_getweaponunarmed_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082424_cweaponanimationsmanager_getweaponanimationssetforped_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_082498_cweaponinfo_getmeleetauntclipsetid_cped_const_ref_fpsstreamingfl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082499_cweaponinfo_getmeleevariationclipsetid_cped_const_ref_fpsstreami(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082522_cweaponinfo_getattachpoint_cweaponcomponentinfo_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082523_cweaponinfo_getisweaponcomponentdefault_rage_atnonfinalhashstrin(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082554_cpedinventoryloadoutmanager_setdefaultloadout_cped_ptr_cpedmodel(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082555_cpedinventoryloadoutmanager_setloadout_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082558_cpedequippedweapon_setweaponhash_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082559_cpedequippedweapon_requestweaponanims(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082562_cpedequippedweapon_attachobject_cpedequippedweapon_eattachpoint(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082571_cpedinventory_process(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082572_cpedinventory_addweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_082580_cpedinventory_addweaponandammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_082591_cpedinventory_getisstreamedin(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082594_cpedweaponmanager_equipweapon_unsigned_int_int_bool_bool_cpedequ(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_082597_cpedweaponselector_getisweaponvalid_cped_const_ptr_cweaponinfo_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_082598_cpedweaponmanager_processweaponinstructions_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082600_cpedweaponmanager_createequippedweaponobject_cpedequippedweapon(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082622_rage_atarray_cpedinventoryloadout_0_unsigned_short_destruct_cped(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082632_cpedweaponmanager_destroyequippedgadgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082634_cpedweaponmanager_processpreai(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082644_cpedweaponmanager_equipbestweapon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_082649_cpedweaponmanager_backupequippedweapon_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082651_cpedweaponmanager_restorebackupweapon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082664_cammoitemrepository_getisoutofammo_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082671_cweaponitem_getkeyfromhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082681_cprojectile_triggerexplosion_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082722_cprojectilemanager_removeallprojectilesinbounds_rage_spdaabb_con(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082725_cprojectilemanager_canexplodetriggeredprojectiles_centity_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082823_sysmemstartnetwork(ph: *mut Phantom); }
extern "C" { pub fn f_082824_sysmemendnetwork(ph: *mut Phantom); }
extern "C" { pub fn f_082936_cnetwork_setmultiplayerlocked_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082938_networkinterface_islocalplayeronline_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083008_networkinterface_iscloudavailable_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083013_networkinterface_getactivegamerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083018_networkinterface_bail_ebailreason_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_083028_networkinterface_isinspectatormode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083034_networkinterface_isfriendlyfireallowed_cped_const_ptr_centity_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083036_networkinterface_ispedallowedtodamageped_cped_const_ref_cped_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083043_networkinterface_getplayerfocusposition_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_083046_networkinterface_isvisibleorclosetoanyremoteplayer_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083047_networkutils_isvisibletoanyremoteplayer_rage_vector3_const_ref_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i32; }
extern "C" { pub fn f_083056_networkinterface_getplayerfromgamerhandle_rage_rlgamerhandle_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083061_networkinterface_getnetworkobject_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083063_networkinterface_unregisterobject_rage_netgameobjectwrapper_cdyn(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083069_networkinterface_setspectatorobjectid_unsigned_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083070_networkinterface_getspectatorobjectid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083071_networkinterface_setinmpcutscene_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_083115_networkinterface_ignorevehicleupdates_cped_ptr_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083118_networkinterface_forcetaskstateupdate_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083132_networkinterface_ondesiredvelocitycalculated_cped_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083149_networkinterface_resolvewaypoint(ph: *mut Phantom); }
extern "C" { pub fn f_083158_networkinterface_getremoteplayercameramatrix_cped_ptr_rage_matri(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083159_networkinterface_getremoteplayercamerafov_cped_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_083160_networkutils_islocalplayersturn_rage_vec3v_const_ref_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i32; }
extern "C" { pub fn f_083162_networkinterface_isdamagedisabledinmp_centity_const_ref_centity(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083168_networkinterface_arebulletsimpactsdisabledinmp_centity_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083170_cnetworkghostcollisions_registerghostcollision_cphysical_const_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083181_networkutils_getnetworkplayerviewport_cnetgameplayer_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083188_networkinterface_isremoteplayerinfirstpersonmode_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_083298_networkclan_hasprimaryclan(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083350_cloudlistener_cloudlistener(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083351_ccloudmanager_requestgetcrewfile_int_rage_rlcloudmemberid_char_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083383_ccloudmanager_requestgettitlefile_char_const_ptr_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_083385_ccloudmanager_requestgetglobalfile_char_const_ptr_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_083386_ccloudmanager_requestgetwwwfile_char_const_ptr_unsigned_int_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_083390_ccloudmanager_requestgetmemberfile_int_rage_rlcloudmemberid_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083391_ccloudmanager_requestpostmemberfile_int_char_const_ptr_void_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083392_ccloudmanager_requestdeletememberfile_int_char_const_ptr_rage_rl(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_083393_ccloudmanager_requestgetugcfile_int_rage_rlugccontenttype_char_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_083493_ctunables_access_unsigned_int_bool_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083496_ctunables_tryaccess_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083523_cusercontentmanager_didcreatesucceed(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083525_cusercontentmanager_didmodifysucceed(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083529_cusercontentmanager_create_char_const_ptr_char_const_ptr_rage_rl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_083531_cusercontentmanager_querycontent_rage_rlugccontenttype_char_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_083543_cusercontentmanager_updatecontent_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i64, a8: i32) -> i32; }
extern "C" { pub fn f_083544_cusercontentmanager_publish_char_const_ptr_char_const_ptr_rage_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_083546_cusercontentmanager_setdeleted_char_const_ptr_bool_rage_rlugccon(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_083558_cusercontentmanager_iscreatepending(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083559_cusercontentmanager_hascreatefinished(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083560_cusercontentmanager_getcreatecontentid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083594_cusercontentmanager_hasmodifyfinished(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083762_cnetworkassetverifier_generatefilecrc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083765_cnetworkassetverifier_generatecrc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083776_cnetworkdamagetracker_resetall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083898_csyncedentity_csyncedentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083899_csyncedentity_csyncedentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083900_csyncedentity_setentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083902_csyncedentity_getentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083908_csyncedvehicle_getvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083927_networkutils_getspecialalpharampingvalue_short_ref_bool_ref_shor(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_083929_networkutils_getspecialalpharampingfadeouttime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084087_clivemanager_showsigninui_unsigned_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084088_clivemanager_issystemuishowing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084163_clivemanager_unregistersingleplayercloudsavefile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084318_cnetworktelemetry_playervehicledistance_unsigned_int_float_unsig(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_084331_cnetworktelemetry_loadgame(ph: *mut Phantom); }
extern "C" { pub fn f_084347_cnetworktelemetry_audtracktagged_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084354_cnetworktelemetry_appendsingleplayersavemigration_bool_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_084421_cprofilestats_flush_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_084426_cprofilestats_synchronizepending_cprofilestats_esynctype_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084449_networkclan_getprimaryclan_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_084629_cloudlistener_dtorcloudlistener_138661(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086135_cnetworkvoice_isgamertalking_rage_rlgamerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086139_cnetworkvoice_gamerhasheadset_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086159_cnetworkvoice_getplayerloudness_int(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_086744_cnetgameplayer_getplayerped_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086752_cnetworkobjectmgr_convertrandomobjecttoscriptobject_cnetobjgame(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086763_cnetobjgame_getplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086785_cnetobjentity_flagfordeletion(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087003_cnetobjobject_isambientobjecttype_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_087107_cnetobjped_cnetobjped_cped_ptr_unsigned_short_unsigned_short_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_087122_cnetobjped_setclonepedoutofvehicle_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087135_cnetobjped_updatelocaltaskslotcache(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087158_cnetobjped_handleunmanagedcloneragdoll(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087183_cnetobjped_changeped_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_087212_csyncedpedvehicleentryscriptconfigdata_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087295_cnetobjped_getisdamagablebyplayer_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_087320_cnetobjped_onpedcleardamage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087518_cnetobjplayer_cnetobjplayer_cped_ptr_unsigned_short_unsigned_sho(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_088037_csyncedpedvehicleentryscriptconfig_hasnondefaultstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_089419_crespawnplayerpedevent_trigger_rage_vector3_const_ref_unsigned_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32); }
extern "C" { pub fn f_089450_cvehiclecomponentcontrolevent_trigger_unsigned_short_unsigned_sh(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_089529_cplayertauntevent_trigger_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_089540_cpedconversationlineevent_trigger_cped_const_ptr_unsigned_int_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_089541_cpedconversationlineevent_trigger_cped_const_ptr_unsigned_int_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090710_asinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090717_atof(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090755_exp2f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090757_expf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090764_fflush(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090805_log10f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090807_logf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090826_powf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090829_iprintf(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090901_qsort_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_090911_rand(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090918_sinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090920_snprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090921_siprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090922_small_sprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090923_sscanf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090925_statvfs(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090926_strcasecmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090927_strcat(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090928_strchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090930_strcmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090931_strcpy(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090932_strcspn(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090939_strlen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090940_strncasecmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090942_strncmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090945_strrchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090947_strstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090956_strtok(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090959_strtoll(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_090962_system(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090965_tanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090966_tanhf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090970_tolower(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090971_toupper(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090998_wmemchr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_091002_emscripten_builtin_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091015_wasm_setjmp(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_091016_wasm_setjmp_test(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_091017_emscripten_longjmp(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_091035_void_std_2_sort_std_2_less_unsigned_int_unsigned_int_ref_unsigne(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
