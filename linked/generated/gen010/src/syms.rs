//! Callee declarations for crate gen010.
//!
//! Every `call $name` in the wasm module becomes a call through this
//! module: an `extern "C"` declaration of the target's generated
//! identifier.  That keeps a function's callers independent of which
//! generated crate the callee ended up in (they are all linked into the
//! same binary and the identifiers are unique).
#![allow(improper_ctypes, dead_code, unused_imports)]
use wasm_rt::Phantom;

extern "C" { pub fn f_000100_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000101_op_new_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000102_op_delete_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000109_rage_syscriticalsectiontoken_syscriticalsectiontoken_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000111_rage_syscriticalsectiontoken_lock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000112_rage_syscriticalsectiontoken_trylock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000113_rage_syscriticalsectiontoken_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000114_rage_sysipccreatesema_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_000116_rage_sysipcwaitsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000118_rage_sysipcwaitsematimed_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_000119_rage_sysipcsignalsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000120_rage_sysipccreatemutex(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_000121_rage_sysipclockmutexcommand_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000122_rage_sysipcunlockmutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000123_rage_sysipcdeletemutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000131_rage_sysipcwaitthreadexit_rage_sysipcthreadid_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000134_rage_sysipcyield_int(ph: *mut Phantom); }
extern "C" { pub fn f_000135_rage_sysipcsleep_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000197_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom); }
extern "C" { pub fn f_000239_rage_grcdevice_manage(ph: *mut Phantom); }
extern "C" { pub fn f_000643_rage_atorderlt_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000644_rage_athashnextsize_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000645_rage_athash_const_char_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000646_rage_athash64_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000654_rage_atstring_set_char_const_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_000663_rage_atstring_replace_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000671_rage_atstring_split_rage_atarray_rage_atstring_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000675_rage_functorbase_functorbase_void_const_ptr_void_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_000676_rage_op_rage_functorbase_const_ref_rage_functorbase_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000680_rage_athashstringnamespacesupport_computehash_rage_athashstringn(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_000681_rage_athashstringnamespacesupport_addstring_rage_athashstringnam(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_000684_rage_athashstringnamespacesupport_trygetstring_rage_athashstring(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_000696_rage_bkwidget_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000703_rage_bkbank_unsetcurrentgroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000704_rage_bkbank_pushgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000705_rage_bkgroup_addgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000706_rage_bkbank_deletegroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000707_rage_bkbank_popgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000709_rage_bkwidget_getnumwidgets_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_000758_rage_bkmanager_openfile_char_ptr_int_char_const_ptr_bool_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_000779_rage_bkmanager_createbank_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000782_rage_bkmanager_findbank_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000783_rage_bkmanager_findwidget_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000785_rage_bkmanager_destroybank_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000786_rage_bkmanager_isbankthread(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000806_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000843_rage_bkcombo_setstring_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_000851_char_ptr_rage_formatf_char_8ul_char_ref_8ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000866_rage_bkgroup_addseparator_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000867_rage_bkgroup_addtitle_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000868_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_000869_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000870_rage_bkgroup_addbutton_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000871_rage_bkgroup_addtext_char_const_ptr_bool_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000872_rage_bkgroup_addtext_char_const_ptr_float_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000873_rage_bkgroup_addtext_char_const_ptr_int_ptr_bool_rage_datcallbac(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_000874_rage_bkgroup_addtext_char_const_ptr_int_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000876_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_000877_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_000882_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000885_rage_bkgroup_addtoggle_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_000886_rage_bkgroup_addtoggle_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000890_rage_bkgroup_addtoggle_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000892_rage_bkgroup_addtoggle_char_const_ptr_rage_atbitset_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000893_rage_bkgroup_addtoggle_char_const_ptr_float_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000894_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000898_rage_bkgroup_addslider_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000902_rage_bkgroup_addslider_char_const_ptr_unsigned_short_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000904_rage_bkgroup_addslider_char_const_ptr_short_ptr_short_short_shor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000906_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000907_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int_rage_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000909_rage_bkgroup_addslider_char_const_ptr_rage_vector2_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000910_rage_bkgroup_addslider_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64); }
extern "C" { pub fn f_000911_rage_bkgroup_addslider_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000913_rage_bkgroup_addslider_char_const_ptr_rage_vector2_ptr_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_000914_rage_bkgroup_addslider_char_const_ptr_rage_vector3_ptr_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_000915_rage_bkgroup_addslider_char_const_ptr_rage_scalarv_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000916_rage_bkgroup_addcombo_char_const_ptr_unsigned_char_ptr_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_000918_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000919_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000920_rage_bkgroup_addcombo_char_const_ptr_unsigned_char_ptr_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000925_rage_bkgroup_addvector_char_const_ptr_rage_vector2_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000927_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000928_rage_bkgroup_addvector_char_const_ptr_rage_vector4_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000929_rage_bkgroup_addvector_char_const_ptr_rage_vector4_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000930_rage_bkgroup_addvector_char_const_ptr_rage_mat34v_ptr_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000941_rage_bkgroup_addcolor_char_const_ptr_rage_color32_ptr_rage_datca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_000942_rage_bkgroup_addcolor_char_const_ptr_rage_color32_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000945_rage_bkgroup_addcolor_char_const_ptr_rage_vector4_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000946_rage_bkgroup_addcolor_char_const_ptr_rage_vector3_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000948_rage_bkgroup_addcolor_char_const_ptr_rage_vector3_ptr_rage_datca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000990_char_ptr_rage_formatf_char_512ul_char_ref_512ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001062_rage_matrix34_inverse_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001105_rage_autoidregistry_autoidregistry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001108_rage_autoidregistry_unregister_rage_autoiddescriptor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001116_char_ptr_rage_formatf_char_32ul_char_ref_32ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001117_rage_datcallback_datcallback(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001119_rage_datcallback_datcallback_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001123_rage_datgrowbuffer_datgrowbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001125_rage_datgrowbuffer_dtordatgrowbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001126_rage_datgrowbuffer_init_rage_sysmemallocator_ptr_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001138_rage_datgrowbuffer_getbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001143_rage_datbitbuffer_datbitbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001144_rage_datbitbuffer_setreadonlybits_void_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001514_rage_diagdisplayf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001516_rage_fistream_write_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001523_quitf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001524_displayf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001527_warningf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001529_rage_fistream_create_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001530_rage_fistream_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001532_rage_fistream_close(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001534_quitf_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001535_rage_diaglogf2_rage_diagchannel_const_ref_rage_diagseverity_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001537_rage_fistream_size(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001551_rage_fprintf_rage_fistream_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001559_rage_diagcontextmessage_diagcontextmessage_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001560_rage_diagcontextmessage_diagcontextmessage_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001563_rage_diagcontextmessage_pop(ph: *mut Phantom); }
extern "C" { pub fn f_001565_printf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001577_rage_fistream_open_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001580_rage_fiassetmanager_open_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001582_rage_fiassetmanager_createleadingpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001605_rage_fidevice_makegrowbufferfilename_char_ptr_int_rage_datgrowbu(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001615_rage_restservicemanager_removeanddeleteservice_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001622_rage_fistream_read_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001623_rage_fiassetmanager_findextensioninpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001626_rage_fiassetmanager_addextension_char_ptr_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001627_rage_fiassetmanager_exists_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001629_rage_fiassetmanager_getstackedpath_char_ptr_int_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001632_rage_fiassetmanager_pushfolder_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001633_rage_fiassetmanager_popfolder(ph: *mut Phantom); }
extern "C" { pub fn f_001635_rage_fiassetmanager_filename_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001636_rage_fiassetmanager_basename_char_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001639_rage_fiassetmanager_removeextensionfrompath_char_ptr_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001652_rage_fidevice_makememoryfilename_char_ptr_int_void_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_001656_rage_fibasetokenizer_init_char_const_ptr_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001680_rage_fibasetokenizer_gettokentochar_char_ptr_int_char_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_001806_rage_fidevice_unmount_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001880_rage_fipackfile_fipackfile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001881_rage_fipackfile_init_char_const_ptr_bool_rage_fipackfile_cachemo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_001883_rage_fipackfile_mountas_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002027_rage_invsqrtf_fast_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_002031_rage_mthrandom_mthrandom_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002044_rage_pgrscbuilder_computeleafsize_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002091_rage_datresource_containsthisaddress_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002092_rage_datresource_getfixup_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002129_rage_atmapentry_rage_atnonfinalhashstring_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002130_rage_atmap_rage_atnonfinalhashstring_rage_atnonfinalhashstring_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002131_rage_atnonfinalhashstring_atnonfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002148_rage_parmanager_savetree_rage_fistream_ptr_rage_partree_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002156_rage_parmanager_initfromstructure_rage_parstructure_ref_rage_par(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002158_rage_parinstancevisitor_visittoplevelstructure_rage_parparsables(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_002188_rage_partreenode_appendaschildof_rage_partreenode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002294_rage_atarray_rage_psorscenumschemavaluedata_0_unsigned_short_gro(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002296_rage_atmap_unsigned_int_rage_atarray_rage_psorscbuilderinstances(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002298_comparesourceaddr_rage_psorscbuilderinstance_ptr_rage_psorscbuil(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002311_rage_atarray_rage_psoenumtableentry_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002335_unsigned_int_ref_rage_atmap_rage_parparsablestructure_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002338_rage_parrestexpireguid_rage_parparsablestructure_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002343_rage_parrestregistersingleton_char_const_ptr_rage_parparsablestr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002373_rage_parstructure_adddelegateholder_char_const_ptr_rage_pardeleg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
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
extern "C" { pub fn f_002581_rage_parelement_setname_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002644_rage_atmap_rage_conststring_unsigned_int_rage_atmaphashfn_rage_c(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002690_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002691_rage_psorscbuilderinstance_ptr_ptr_std_2_partial_sort_impl_abi_n(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002692_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002693_rage_psorscbuilderinstance_ptr_ptr_std_2_partition_with_equals_o(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002702_rage_qabug_clearbugbuffer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002712_rage_qabugstarinterface_qabugstarinterface(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002713_rage_qabugstarinterface_createbug_rage_qabug_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_002720_rage_conststring_vcopy_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002721_rage_conststring_dtorconststring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002739_rage_atpartialstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002743_rage_atdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002746_rage_stristr_char_ptr_char_const_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002763_rage_utf8removeinvalidsurrogates_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002839_rage_sysparam_get_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002840_rage_sysparam_get_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002854_rage_sysstack_printregisteredbacktrace_unsigned_short_void_ptr_u(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002862_rage_systhreadtype_addcurrentthreadtype_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_002878_rage_atmap_unsigned_int_rage_sysstackcollector_tagentry_rage_atm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002881_unsigned_int_ref_rage_atmap_unsigned_int_unsigned_int_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002914_rage_systhreadtype_clearcurrentthreadtype_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_002927_rage_sysmemstarttemp(ph: *mut Phantom); }
extern "C" { pub fn f_002928_rage_sysmemendtemp(ph: *mut Phantom); }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_002943_rage_sysmempoolallocator_dtorsysmempoolallocator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002945_rage_sysmempoolallocator_safeallocate_unsigned_long_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002946_rage_sysmempoolallocator_allocate_unsigned_long_unsigned_long_bo(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002948_rage_sysmempoolallocator_free_void_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002950_rage_sysmemsimpleallocator_sysmemsimpleallocator_void_ptr_int_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_003085_rage_systaskparameters_systaskparameters(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003088_rage_systaskmanager_addscheduler_char_const_ptr_int_int_rage_sys(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_003090_rage_systaskmanager_create_char_const_ptr_void_ptr_rage_systaskp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_003091_rage_systaskmanager_dispatch_rage_syspreparedtaskhandle_const_re(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003092_rage_systaskmanager_dispatchlocally_rage_syspreparedtaskhandle_c(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003093_rage_systaskmanager_poll_rage_systaskhandle_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003095_rage_systaskmanager_wait_rage_systaskhandle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003097_char_ptr_rage_formatf_char_16ul_char_ref_16ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003098_rage_systasklog_dump(ph: *mut Phantom); }
extern "C" { pub fn f_003111_rage_sysdependencyscheduler_insertinternal_rage_sysdependency_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003136_rage_geomspheres_testspheretotriangle_rage_vec4v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_003139_rage_geomdistances_distancelinetopoint_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_003146_segmenttocapsuleintersectionshelper_rage_vec3v_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_003147_rage_geomspheres_testspheretoseg_rage_vec3v_const_ref_rage_scala(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_003152_rage_geomboxes_testsegmenttobox_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64) -> i32; }
extern "C" { pub fn f_003154_rage_geomboxes_testspheretobox_rage_vec3v_const_ref_rage_scalarv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_003155_rage_geom2d_test2dsegvsseg_float_ref_float_ref_rage_vector2_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_003159_rage_matrix34_fromquaternion_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003161_rage_quaternion_frommatrix34_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003165_rage_matrix34_op_rage_matrix33_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003167_rage_matrix44_makerotx_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_003168_rage_matrix44_makerotz_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_003174_rage_setunitup_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003179_rage_atpoolbase_init_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003182_rage_atpoolbase_releasemem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003185_rage_crframe_crframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003187_rage_crframe_init_rage_crframedata_const_ref_unsigned_char_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_003192_rage_crframe_set_rage_crframe_const_ref_rage_crframefilter_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003193_rage_crframe_blend_float_rage_crframe_const_ref_bool_rage_crfram(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_003195_rage_crframe_merge_rage_crframe_const_ref_rage_crframefilter_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003206_rage_crframe_pose_rage_crskeleton_ref_bool_rage_crframefilter_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003208_rage_crframe_inversepose_rage_crskeleton_const_ref_bool_rage_crf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003218_rage_crframe_setfloat_unsigned_char_unsigned_short_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32) -> i32; }
extern "C" { pub fn f_003221_rage_crframe_dtorcrframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003313_rage_crframedata_crframedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003314_rage_crframedata_dtorcrframedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003341_rage_crframefilterbonebasic_crframefilterbonebasic_rage_crframef(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003352_rage_crweightset_setanimweight_int_float_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_003398_rage_crframedatainitializer_initializeframedata_rage_crframedata(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003402_rage_crframedatainitializerboneandmover_crframedatainitializerbo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_003801_rage_fwanimdirector_init_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_003804_rage_fwanimdirector_initexpressions_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003810_rage_fwanimdirector_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003816_rage_fwanimdirector_updatemove_float_bool_rage_fwanimdirector_pt(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003822_rage_fwanimdirector_startupdatemidphysics_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_003825_rage_fwanimdirector_startupdateprerender_float_rage_crmtmotiontr(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32); }
extern "C" { pub fn f_003826_rage_fwanimdirector_setposed_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003831_rage_fwanimdirectorcomponentsyncedscene_isplayingsyncedscene_con(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003832_rage_fwanimdirectorcomponentsyncedscene_isvalidsceneid_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003835_rage_fwanimdirector_getmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003840_rage_fwclipsetmanager_getclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003850_rage_fwanimdirector_getcreature_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003857_rage_fwanimdirector_waitforprerenderupdatetocomplete(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003859_rage_fwanimdirector_waitformotiontreebyphase_rage_fwanimdirector(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003860_rage_fwanimdirectorcomponentragdoll_posefromragdoll_rage_crcreat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003861_rage_fwanimdirector_poseskeletonusingragdollframe_rage_crskeleto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_003876_rage_fwanimmanager_getclipifexistsbysetid_rage_fwmvtemplateid_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003894_rage_fwanimhelpers_getdebugclipname_rage_crclip_const_ref_rage_a(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003896_rage_fwanimmanager_getclipifexistsbydictindex_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_003904_rage_fwanimmanager_findslot_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003905_rage_fwanimmanager_findslotfromhashkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003907_rage_fwanimmanager_isvalidslot_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003968_rage_fwclipset_getclipdictionaryindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003972_rage_fwclipset_pickrandomclip_rage_fwmvtemplateid_rage_fwmvclipi(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_003976_rage_fwclipset_streamin_deprecated(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003977_rage_fwclipset_request_deprecated_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_003984_rage_fwclipsetrequesthelper_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003993_rage_fwclipsetmanager_getclipsetid_rage_atnonfinalhashstring_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004014_rage_atstring_ref_rage_atmap_int_rage_atstring_rage_atmaphashfn(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004019_rage_fwclipsetrequesthelper_request_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_004037_rage_fwclipsetrequesthelper_dtorfwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004038_rage_fwclipsetrequesthelper_fwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004041_rage_fwclipsetrequesthelper_isloaded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004082_rage_fwfacialclipsetgroupmanager_getfacialclipsetgroup_rage_fwmv(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004280_rage_fwposematcher_fwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004401_rage_fwanimdirectorcomponentsyncedscene_setsyncedsceneorigin_sho(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_004414_rage_fwanimdirectorcomponentsyncedscene_getsyncedsceneattachenti(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004416_rage_fwanimdirectorcomponentsyncedscene_startsynchronizedscene_r(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004603_rage_fwextensionlist_add_rage_fwextension_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004619_rage_fwarchetype_getdrawable_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004620_rage_fwarchetype_getfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004621_rage_fwarchetype_getassetparenttxdindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004631_rage_fwarchetypemanager_getarchetypename_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004632_rage_fwarchetypemanager_lookupmodelid_rage_fwarchetype_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004633_rage_fwarchetypemanager_getarchetype_rage_atnonfinalhashstring_u(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004634_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004648_rage_fwattachmententityextension_isattachstatebasicderived_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004652_rage_fwattachmententityextension_getoffsetfromparent_rage_fwenti(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_004661_rage_fwdynamicentitycomponent_fwdynamicentitycomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004663_rage_fwdynamicentitycomponent_createattachmentextensionifmissing(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004676_rage_fwentity_clearcurrentphysicsinst(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004677_rage_fwentity_deleteskeleton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004681_rage_fwentity_waitforanyactiveanimupdatetocomplete_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004683_rage_fwentity_getaabb_rage_spdaabb_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004686_rage_fwextensionlist_dtorfwextensionlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004688_rage_fwextensionlist_destroy_rage_fwextension_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004689_rage_fwextensionlist_destroy_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004693_rage_fwsceneupdate_isinsceneupdate_rage_fwentity_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004717_rage_fwdrawdata_dtorfwdrawdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004719_rage_fwdrawdata_removedrawable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004776_rage_fwpickermanager_ispickeroftypeenabled_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004786_rage_fwpickermanager_gethoverentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004790_rage_fwpickermanager_getindexofentitywithinresultsarray_rage_fwe(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004792_rage_fwpickermanager_getselectedentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004793_rage_fwpickermanager_getentity_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004794_rage_fwpickermanager_getnumberofentities(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_004795_rage_fwpickermanager_getshowhidemode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_004816_rage_fwvectormap_drawline_rage_vector3_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_004819_rage_fwvectormap_drawmarker_rage_vector3_const_ref_rage_color32(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_004820_rage_fwvectormap_drawcircle_rage_vector3_const_ref_float_rage_co(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_004822_rage_fwvectormap_drawrectaxisaligned_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_004823_rage_fwvectormap_drawwedge_rage_vector3_const_ref_float_float_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_004825_rage_fwvectormap_draw3dboundingboxonvmap_rage_spdaabb_const_ref(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004867_rage_dldrawcommandbuffer_initialise(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004921_rage_dldrawcommandbuffer_registercommand_int_void_ptr_rage_dlcmd(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_004922_rage_dldrawcommandbuffer_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004923_rage_dldrawcommandbuffer_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004926_rage_dldrawcommandbuffer_allocatepagedmemory_rage_edrawlistpaget(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_004931_rage_dldrawcommandbuffer_getdatablock_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_004932_rage_dldrawcommandbuffer_lookupshareddata_int_rage_dlshareddatai(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_004933_rage_dldrawcommandbuffer_allocateshareddata_int_rage_dlshareddat(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_004934_rage_dldrawcommandbuffer_adddatablock_void_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_004935_rage_dlcmdbase_op_new_unsigned_long_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004938_rage_dldrawcommandbuffer_flipbuffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004946_void_std_2_introsort_std_2_classicalgpolicy_int_ptr_ref_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_004949_rage_dldrawlistmgr_getdrawlistname_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004950_rage_dlcmdlockrendertarget_dlcmdlockrendertarget_int_rage_grcren(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_004951_rage_dlcmdunlockrendertarget_dlcmdunlockrendertarget_int_rage_gr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004952_rage_dlcmdclearrendertarget_dlcmdclearrendertarget_bool_rage_col(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: f32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_004953_rage_dlcmdsetclipplane_dlcmdsetclipplane_unsigned_int_rage_vec4v(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004954_rage_dlcmdsetclipplaneenable_dlcmdsetclipplaneenable_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004955_rage_dlcmdgrclightstatesetenabled_dlcmdgrclightstatesetenabled_b(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004956_rage_dlcmdgrcdevicesetscissor_dlcmdgrcdevicesetscissor_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_004962_rage_dlcmdshaderfxpushforcedtechnique_dlcmdshaderfxpushforcedtec(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004963_rage_dlcmdpushtimebar_dlcmdpushtimebar_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_004964_rage_dlcmdpoptimebar_dlcmdpoptimebar(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004966_rage_dlcmdpushgputimebar_dlcmdpushgputimebar_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_004968_rage_dlcmdpushmarker_dlcmdpushmarker_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004969_rage_dlcmdsetbucketsandrendermode_dlcmdsetbucketsandrendermode_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_004970_rage_dlcmdsetbucketsandrendermode_dlcmdsetbucketsandrendermode_u(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004971_rage_dlcmdsetdepthstencilstateex_dlcmdsetdepthstencilstateex_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_004972_rage_dlcmdsetstates_dlcmdsetstates_rage_grcstateblock_grcrasteri(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_004986_rage_dldrawlistmgr_dtordldrawlistmgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004988_rage_dldrawlistmgr_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004989_rage_dldrawlistmgr_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004991_rage_dldrawlistmgr_getdrawlistgputime_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_004999_rage_dldrawlistmgr_resethighestlod_bool(ph: *mut Phantom); }
extern "C" { pub fn f_005000_rage_dldrawlistmgr_sethighestlod_bool_rage_dldrawlistmgr_ehighes(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_005003_rage_dldrawlistmgr_addtxdreference_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005004_rage_dldrawlistmgr_addclothreference_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005005_rage_dldrawlistmgr_addcharclothreference_rage_clothvariationdata(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005006_rage_dldrawlistmgr_addmapdatareference_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005007_rage_dldrawlistmgr_addarchetypereference_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005008_rage_dldrawlistmgr_addarchetype_hd_reference_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005010_rage_dldrawlistmgr_charclothremove_rage_clothvariationdata_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005013_rage_dldrawlistmgr_removeallrefs_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005015_rage_dldrawlistmgr_adjustfragmentlod_bool_unsigned_int_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_005016_rage_dldrawlistmgr_adjustvehiclehd_bool_bool_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_005017_rage_dldrawlistmgr_setstatshighlight_bool_char_const_ptr_bool_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_005018_rage_dldrawlistmgr_setuplodwidgets_rage_bkgroup_ref_int_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_005020_rage_dldrawlistmgr_setupwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005049_rage_fwtextutil_truncatefileextension_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005064_rage_fwclothdebug_updateselection_bool_ref_bool_ref_int_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_005065_rage_fwclothdebug_drawprimitivesinfo_int_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_005066_rage_fwclothdebug_selectverts_rage_vector3_const_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_005068_rage_fwclothdebug_dragverts_rage_vector3_const_ref_rage_clothcon(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_005069_rage_fwclothgizmo_gettransform_rage_mat34v_ref_rage_vec3v_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005091_rage_drawedgeselectioninfo_rage_atarray_rage_phedgedata_0_unsign(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005092_rage_clothvariationdata_deletebuffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005095_rage_clothmanager_findpedinclothinstanceshutdown_rage_phboundcom(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005098_rage_clothmanager_addwidget_rage_clothcontroller_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_005099_rage_clothmanager_deallocateverlet_rage_clothinstance_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005105_rage_clothmanager_allocateverlet(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005106_rage_clothmanager_allocatevardata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005109_rage_clothmanager_removecharactercloth_rage_clothcontroller_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005110_rage_clothmanager_deallocatevardata_rage_clothvariationdata_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005111_rage_clothinstance_drawselection_rage_vector3_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_005112_rage_clothinstance_pickedges_rage_vector3_const_ref_rage_phverle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_005113_void_rage_clothinstance_drawpervertexvalues_float_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_005115_rage_clothinstance_drawbound_rage_phbound_const_ptr_rage_mat34v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_005169_rage_ropedatamanager_load(ph: *mut Phantom); }
extern "C" { pub fn f_005170_rage_ropedatamanager_unloadropetexures(ph: *mut Phantom); }
extern "C" { pub fn f_005171_rage_ropedatamanager_loadropetexures(ph: *mut Phantom); }
extern "C" { pub fn f_005196_rage_ropeinstance_setphysinstflags_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005197_rage_ropeinstance_shutdownphysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005203_rage_ropeinstance_attachobjectstoconstraintarray_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32) -> i64; }
extern "C" { pub fn f_005206_rage_ropeinstance_attachobjects_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: i32, a11: i64, a12: i64, a13: i64, a14: i64); }
extern "C" { pub fn f_005207_rage_ropeinstance_attachtoobject_rage_vec3v_const_ref_rage_phins(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_005208_rage_ropeinstance_detachfromobject_rage_phinst_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005209_rage_ropeinstance_setnewuniqueid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005212_rage_ropeinstance_load_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005213_rage_ropemanager_remove_rage_ropeinstance_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005214_rage_ropemanager_processshutdownlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005215_rage_ropemanager_updatephysics_float_rage_enropeupdateorder(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_005216_rage_ropemanager_updatecloth_rage_enropeupdateorder(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005224_rage_ropemanager_draw_rage_matrix34_const_ref_float_void_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_005225_rage_ropemanager_add_rage_vec3v_const_ref_rage_vec3v_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32, a11: f32, a12: i32, a13: i32) -> i64; }
extern "C" { pub fn f_005226_rage_ropemanager_breakrope_rage_ropeinstance_ptr_rage_ropeinstan(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32); }
extern "C" { pub fn f_005270_rage_fwrenderlistbuilder_addentityflags_addentityflags(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005271_rage_fwrenderlistbuilder_setdrawmodeinfo_int_int_int_int_char_co(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_005272_rage_fwrenderlistbuilder_executepasslist_rage_fwdrawlistaddpass(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_005273_rage_fwrenderlistbuilder_addtodrawlistentitylistcore_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_005274_rage_fwrenderlistbuilder_setrenderpassname_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_005275_rage_fwrenderlistbuilder_setrenderbucketname_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_005279_rage_fwentitylist_addentity_rage_fwentity_ptr_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: f32, a6: i32); }
extern "C" { pub fn f_005284_rage_fwrenderlistgroup_clear(ph: *mut Phantom); }
extern "C" { pub fn f_005285_rage_fwrenderlistgroup_registerrenderphase_int_rage_fwrenderphas(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_005286_rage_fwrenderlistgroup_startsortentitylistjob(ph: *mut Phantom); }
extern "C" { pub fn f_005287_rage_fwrenderlistgroup_waitforsortentitylistjob(ph: *mut Phantom); }
extern "C" { pub fn f_005288_rage_fwrenderlistgroup_setrenderphasetypeinfo_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_005293_rage_fwrenderphase_fwrenderphase_rage_grcviewport_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_005294_rage_fwrenderphase_dtorfwrenderphase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005296_rage_fwrenderphase_drawlistprologue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005298_rage_fwrenderphase_drawlistepilogue(ph: *mut Phantom); }
extern "C" { pub fn f_005301_void_rage_dlc_addinternal_void_ptr_void_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005305_rage_fwrenderphasescanner_fwrenderphasescanner(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005313_rage_fwrenderthreadinterface_isusingdefaultrenderfunction(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_005315_rage_fwrenderthreadinterface_flush_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005317_rage_fwrenderthreadinterface_dorenderfunction(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005318_rage_fwrenderthreadinterface_setrenderfunction_rage_functor0(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005324_rage_fwassetstore_rage_pgdictionary_rage_grctexture_rage_fwtxdde(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005325_rage_fwtxdstore_isbeingdefragged_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005326_rage_fwtxdstore_getishdcapable_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005327_rage_fwdrawablestore_getishdcapable_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005328_rage_fwdwdstore_getishdcapable_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005431_std_2_function_value_func_void_ptr_rage_fwgrassinstancelistdef_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005453_rage_fwboxstreamer_init_rage_strstreamingmodule_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_005460_rage_fwboxstreamer_requestaboutpos_rage_vec3v_const_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_005461_rage_fwboxstreamer_hasloadedaboutpos_rage_vec3v_const_ref_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_005462_rage_fwboxstreamer_hasloadedwithinaabb_rage_spdaabb_const_ref_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_005463_rage_fwboxstreamer_ensureloadedaboutpos_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005513_rage_fwtxdstore_setcurrenttxd_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005514_rage_fwtxdstore_popcurrenttxd(ph: *mut Phantom); }
extern "C" { pub fn f_005538_rage_crframefilterdictionarystore_findframefilter_rage_fwmvtempl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005548_rage_fwfragmentstore_checkifinuse_rage_fragtype_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005562_rage_fwassetstore_rage_pgdictionary_rage_grctexture_rage_fwtxdde(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005589_rage_fwpsostoreloader_unload_rage_fwpsostoreloadinstance_ref_boo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005648_rage_fwpsostoreloader_load_char_const_ptr_rage_fwpsostoreloadins(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_005650_rage_fwpsostoreloader_loaddataintoobjectcore_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005651_rage_fwpsostoreloader_makesimplefileloadercore(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005664_rage_fwstaticboundsstore_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005678_rage_fwstaticboundsstore_getdummybounddata_rage_strlocalindex_sh(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_005700_rage_fwtxdstore_remove_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005707_rage_fwtxdstore_pushcurrenttxd(ph: *mut Phantom); }
extern "C" { pub fn f_005714_rage_fwtxdstore_removeref_rage_strlocalindex_rage_strrefkind(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005718_rage_fwbaseentitycontainer_getentitydescindex_rage_fwentity_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005719_rage_fwbaseentitycontainer_getapproxboundingbox_unsigned_int_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_005730_rage_fwinteriorlocation_makeinvalid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005731_rage_fwinteriorlocation_setroomindex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005732_rage_fwinteriorlocation_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005733_rage_fwinteriorlocation_getroomindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005744_rage_fwworldmgr_forallentitiesintersecting_rage_fwsearchvolume_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_005761_rage_fwscan_getgbufscreenquadpair_rage_fwscenegraphnode_const_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005766_rage_fwscancascadeshadowinfo_updateshadowviewport_rage_vec4v_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_005770_scandebugaddpedmissinggfx_unsigned_int_void_ptr_rage_spdaabb_con(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_005772_rage_fwscreenquad_generatefrustum_rage_mat44v_const_ref_rage_spd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_005780_rage_rstcalculatescreenspaceboundisvis_rage_rstsplatmat44v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_005789_rage_fwscannodesdebug_debugdrawportal_rage_fwscenegraphnode_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_005795_rage_fwscannodesdebug_drawfrustumforscreenquad_rage_fwscreenquad(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_005802_rage_fwsearch_dtorfwsearch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005888_rage_fwentitylist_celement_ptr_std_2_partial_sort_impl_abi_nqn22(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_005889_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_005890_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_005915_rage_aitask_aitask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005917_rage_fwrefawarebaseimpl_rage_datbase_dtorfwrefawarebaseimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005919_rage_aitask_setstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005922_rage_aitask_findsubtaskoftype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005931_rage_aitaskmanager_process_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_005934_rage_aitasktree_aborttasks(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005935_rage_aitaskmanager_aborttasks(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005946_rage_aitasktree_cleartask_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005947_rage_aitasktree_getactiveleaftask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005948_rage_aitasktree_gethastask_rage_aitask_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005949_rage_aitasktree_findtaskbytypeactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005950_rage_aitasktree_findtaskbytypewithpriority_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_006003_rage_fwpathserver_forceabortcurrentpathrequest_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006041_rage_strcacheloader_cmoduledescarray_addmodule_char_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_006056_rage_strstreaminginfomanager_removeobject_rage_strindex_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006066_rage_strpackfilemanager_getimagefilefromhandle_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006075_rage_strstreaminginfo_computevirtualsize_rage_strindex_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006076_rage_strstreaminginfo_computephysicalsize_rage_strindex_bool_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006084_rage_strstreaminginfomanager_requestobject_rage_strindex_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006085_rage_strpackfilemanager_registerindividualfile_char_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_006088_rage_atarray_rage_atnonfinalhashstring_0_unsigned_short_grow_uns(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006122_rage_strstreaminginfomanager_getmodule_rage_strindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006124_rage_strstreamingengine_getobjectname_rage_strindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006162_rage_strstreaminginfo_gethandle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006167_rage_strstreaminginfomanager_isobjectinimage_rage_strindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006175_rage_strstreaminginfomanager_clearrequiredflag_rage_strindex_uns(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006219_rage_strstreamingengine_getstrindex_void_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006220_rage_strstreamingengine_addrefbyptr_void_ptr_rage_strrefkind(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006225_rage_atvarstring_atvarstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_006238_rage_strstreamingloadermanager_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006241_rage_strstreamingvisualize_addmarker_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006254_rage_strrequest_request_rage_strlocalindex_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_006255_rage_strrequest_hasloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006256_rage_strrequest_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006257_rage_strrequest_clearrequiredflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006342_rage_tcmanager_findmodifier_rage_atnonfinalhashstring_char_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006343_rage_tcmanager_findmodifierindex_rage_atnonfinalhashstring_char(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006361_rage_atarray_float_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006406_rage_tcmodifier_apply_rage_tckeyframeuncompressed_ref_float_cons(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_006518_rage_audsound_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006558_rage_audsound_gettoplevelparent_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006610_rage_audsound_decompressmetadata_untyped_rage_sound_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006715_rage_audsound_prepareandplay_rage_audwaveslot_ptr_bool_int_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_006729_rage_audsound_searchforsoundsbytype_unsigned_int_rage_audsound_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_006731_rage_audsound_debugdraw_rage_auddebugdrawmanager_ref_bool_bool_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_006840_rage_adatcontainer_getobjectnamehash_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006877_rage_audmetadatamanager_getobjectmetadataptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006886_rage_audmetadatamanager_getobjectnamefromnametableoffset_unsigne(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006889_rage_audcategory_ismuteactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006903_rage_audcategorymanager_getcategoryptr_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006941_rage_audcurve_calculatevalue_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_006944_rage_audmetadatamanager_getobjectname_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006971_rage_audengine_triggerupdate(ph: *mut Phantom); }
extern "C" { pub fn f_006972_rage_audengine_waitforaudioframe(ph: *mut Phantom); }
extern "C" { pub fn f_006983_rage_audengineutil_matchname_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006984_rage_audengineutil_getcurrenttimeinmilliseconds(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007036_rage_audmetadatamanager_getobject_verify_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007037_rage_audmetadatamanager_getobject_verify_rage_audmetadataref_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007047_rage_audsoundfactory_getmetadataptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007051_rage_audsoundfactory_getbanknamefromindex_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007078_rage_audsoundset_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007079_rage_audsoundset_find_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007084_rage_audtracker_audtracker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007118_rage_audsmoother_audsmoother(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007126_rage_audsimplesmoother_audsimplesmoother(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007175_rage_auddriver_setdebugdrawrenderstates(ph: *mut Phantom); }
extern "C" { pub fn f_007189_rage_audvoicemgr_printphysicalvoices(ph: *mut Phantom); }
extern "C" { pub fn f_007200_char_ptr_rage_formatf_char_255ul_char_ref_255ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007281_rage_audmixerdevice_setreplaymixermodeloading(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007404_rage_audentity_createsound_localreference_unsigned_int_rage_auds(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007411_rage_audentity_addbatchedsoundrequest_rage_audmetadataref_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007412_rage_audentity_createandplaysound_unsigned_int_rage_audsoundinit(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007419_rage_vfxlistitem_vfxlistitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007421_rage_vfxlist_additem_rage_vfxlistitem_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007424_rage_vfxlist_removeall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007426_rage_vfxbrokenfraginfo_dtorvfxbrokenfraginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007429_rage_vfxutil_inittxd_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007430_rage_vfxutil_shutdowntxd_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007433_rage_decalinst_shutdownimmediate_rage_decalinst_shutdownimmediat(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007439_rage_decalbucket_renderdebug_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007448_rage_decalclipper_renderdebugplane_rage_decalplaneid_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007475_rage_decalmanager_synchronize(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007480_rage_decalmanager_render_rage_decalrenderpass_rage_decalrenderpa(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_007528_rage_ptfxattach_attach_rage_ptxeffectinst_ptr_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_007530_rage_ptfxflags_clearuserreservedflag_rage_ptxeffectinst_ptr_rage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007559_rage_ptfxmanager_gettriggeredinst_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007562_rage_ptfxmanager_getregisteredinst_void_const_ptr_int_bool_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_007591_rage_ptfxwrapper_geteffectrule_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007658_rage_fwptrlistsinglelink_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007661_rage_fwptrlist_ismemberoflist_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007662_rage_fwquadtreenode_fwquadtreenode_rage_fwrect_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_007663_rage_fwquadtreenode_dtorfwquadtreenode(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007670_rage_fwquadtreenode_additem_void_ptr_rage_fwrect_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007671_rage_fwquadtreenode_deleteitem_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007687_rage_polycliptofrustum_rage_vec3v_ptr_rage_vec3v_const_ptr_int_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_007688_rage_polycliptofrustumwithoutnearplane_rage_vec3v_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_007689_rage_calculatefrustumbounds_rage_vec3v_ref_rage_vec3v_ref_rage_g(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007690_rage_planesetdebugdraw_rage_vec4v_const_ptr_int_rage_color32_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_007691_rage_planesetdebugdraw_rage_spdtransposedplaneset4_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007694_rage_fwgeom_intersectsphereray_rage_spdsphere_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_007695_rage_fwportalcorners_calcboundingsphere_rage_spdsphere_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007696_rage_fwportalcorners_calcboundingbox_rage_spdaabb_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007698_rage_fwportalcorners_calccloseness_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_007700_rage_fwportalcorners_calcdistancetopoint_rage_vec3v_const_ref_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007756_rage_netinterface_flushalllogfiles_bool(ph: *mut Phantom); }
extern "C" { pub fn f_007757_rage_netinterface_getsynchronisationtime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007759_rage_netinterface_getnumphysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007760_rage_netinterface_getnumremotephysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007762_rage_netinterface_getallphysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007763_rage_netinterface_getremotephysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007767_rage_netinterface_getphysicalplayerfromindex_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_007769_rage_netinterface_getlocalphysicalplayerindex(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007771_rage_netobjectmgrbase_getnetworkobject_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007777_rage_netinterface_getdefaultlogfiletargettype(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007778_rage_netinterface_getdefaultlogfileblockingmode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007791_rage_netlogimpl_netlogimpl_char_const_ptr_rage_logopentype_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_007802_rage_netplayer_ismyplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007803_rage_networklogutils_writelogevent_rage_netlogginginterface_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007805_rage_netarraymanager_getarrayhandler_unsigned_short_rage_netarra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_007848_rage_neteventmgr_checkforspaceinpool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007849_rage_neteventmgr_prepareevent_rage_netgameevent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007855_rage_netplayer_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007884_rage_netbandwidthmgr_getunreliableresendcount_rage_netplayer_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007949_rage_netgameevent_netgameevent_unsigned_short_bool_bool_rage_net(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_008000_rage_netobjecttypepredicate_setobjecttype_unsigned_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008005_rage_getupdatelevelstring_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_008006_rage_fwhttpquery_fwhttpquery_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_008009_rage_fwhttpquery_dtorfwhttpquery(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008014_rage_fwhttpquery_initpost_char_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008015_rage_fwhttpquery_appendxmlcontent_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008016_rage_fwhttpquery_commitpost(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008017_rage_fwhttpquery_pending(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008059_char_ptr_rage_formatf_char_5ul_char_ref_5ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008069_rage_netsynctree_getnumallowedbatches_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008074_rage_netobject_ispendingownerchange_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008077_rage_netobject_isglobalflagset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008111_rage_netobject_isstarving_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008122_rage_netplayer_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008155_rage_netsynctree_update_rage_netsynctreetargetobject_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_008184_rage_netobjectmgrbase_getnumlocalobjects_bool_ptr_rage_netobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008185_rage_netobjectmgrbase_getlocalobjectlists_rage_atdlist_rage_neto(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008186_rage_netobjectmgrbase_getnumremoteobjects_bool_ptr_rage_netobjec(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008187_rage_netobjectmgrbase_gettotalnumobjects_bool_ptr_rage_netobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008195_rage_netobjectmgrbase_getaveragelatency_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_008196_rage_netobjectmgrbase_getpacketloss_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_008214_rage_netsynctree_forcesendofsyncupdatenodes_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_008217_rage_netblender_setlastsyncmessagetime_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008230_rage_netblenderlininterp_getlastpositionreceived_unsigned_int_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008231_rage_netblenderlininterp_getlastvelocityreceived_unsigned_int_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008246_rage_netplayer_getrlgamerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008311_rage_netsynctree_getobjectcount_unsigned_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008320_rage_netsynctree_dirtynode_rage_netsynctreetargetobject_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008321_rage_netsynctree_forcesendofnodedata_unsigned_int_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_008682_rage_fwbasepool_fwbasepool_int_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_008683_rage_fwconfigmanager_getsizeofpool_unsigned_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008686_rage_fwbasepool_getindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008688_rage_fwbasepool_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008689_rage_fwscriptguid_createguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008691_rage_fwscriptguid_deleteguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008705_rage_scripthandlernetcomponent_getstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008781_rage_scripthandlernetcomponent_isplayeraparticipant_rage_netplay(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008866_rage_atmapentry_unsigned_int_int_const_ref_rage_atmap_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008917_rage_fwtimer_enduserpause(ph: *mut Phantom); }
extern "C" { pub fn f_008918_rage_fwtimer_startuserpause_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008945_rage_fwbasepool_forall_bool_ptr_void_ptr_void_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008950_rage_fwbasepool_getjustindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008951_rage_fwbasepool_isvalidptr_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008953_rage_fwknownrefholder_fwknownrefholder_void_ptr_ptr_rage_fwknown(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_008954_rage_phfwfraginst_dtorphfwfraginst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008955_rage_phfwinst_phfwinst_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008956_rage_phfwinst_dtorphfwinst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009048_rage_crproperty_getattribute_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009211_rage_crskeletondata_computeobjecttransform_int_rage_mat34v_ref_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009212_rage_crskeletondata_computeglobaltransform_int_rage_mat34v_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_009216_rage_crskeletondata_convertbonenametoid_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009218_rage_crskeleton_crskeleton(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009219_rage_crskeleton_dtorcrskeleton(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009222_rage_crskeleton_init_rage_crskeletondata_const_ref_rage_mat34v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009223_rage_crskeleton_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009224_rage_crskeleton_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009227_rage_crskeleton_getterminatingpartialbone_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009228_rage_crskeleton_copyfrom_rage_crskeleton_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009233_rage_crskeleton_getglobalmtx_unsigned_int_rage_mat34v_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009234_rage_crskeleton_setglobalmtx_unsigned_int_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009235_rage_atmap_unsigned_int_int_rage_atmaphashfn_unsigned_int_rage_a(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009266_rage_crjointdata_gettranslationlimits_rage_crbonedata_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_009283_rage_grmmodel_getgeometry_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009286_rage_grmmodel_setforceshader_rage_grmshader_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009291_rage_grmshader_begindraw_rage_grmshader_edrawtype_bool_rage_grce(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_009292_pixbeginn_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_009293_rage_grmshader_bind_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009294_rage_grmshader_unbind_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009295_rage_grmshader_enddraw_const(ph: *mut Phantom); }
extern "C" { pub fn f_009308_rage_grmmodel_setmodelcullback_rage_grccullstatus_ptr_rage_spdaa(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009312_rage_grmshader_setforcedtechniquegroupid_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009313_rage_grmshader_gettechniquegroupid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_009314_pixend(ph: *mut Phantom); }
extern "C" { pub fn f_009315_rage_grmshader_copypassdata_rage_grceffecttechnique_int_rage_grm(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_009316_rage_grmshader_setinstanceddrawmode_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009317_rage_grmshader_load_char_const_ptr_rage_fitokenizer_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_009321_rage_grmmodelfactory_builddeclarator_rage_grcfvf_const_ptr_rage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009325_rage_grmshadergroup_initcount_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009326_rage_grmshadergroup_add_rage_grmshader_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009327_rage_grmshadergroup_lookupvar_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009329_rage_grmshadergroup_lookupvarforinstancing_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_009330_rage_grmshadergroup_lookupshader_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009332_rage_grmshadergroup_setvar_rage_grmshadergroupvar_rage_grcbuffer(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009333_rage_grmshadergroup_setvarcommon_rage_grmshadergroupvar_void_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_009334_rage_grmshadergroup_setvar_rage_grmshadergroupvar_rage_grctextur(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009342_rage_grmmatrixset_setdirty_bool_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009345_rage_grmmatrixset_computematrix_rage_matrix34_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009396_rage_pftimebars_tabbeddump_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009404_char_ptr_rage_formatf_char_80ul_char_ref_80ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009405_rage_pftimebars_getfunctiontotals_char_const_ptr_int_ref_float_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009408_rage_pftimebars_recordbaseline_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009409_rage_pftimebars_display_bool_bool_int_ref_float_int_int_char_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32, a5: i32, a6: i64, a7: i32); }
extern "C" { pub fn f_009417_pixbegin_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009429_rage_pftimebarmgr_frameinit_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009431_rage_pftimebarmgr_displayoverbudgets_rage_pftimebars_ref_float_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_009432_rage_pftimebarmgr_getfunctiontotals_char_const_ptr_int_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_009433_rage_pftimebarmgr_getselectedtime_const(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_009447_rage_pfdrawitembase_beginmanager(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009448_rage_pfdrawitembase_end(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009454_rage_matrix34_makerotateto_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009467_rage_pfdrawgroup_setenabled_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009473_rage_pfdrawgroup_enablechildren(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009474_rage_pfdrawgroup_disablechildren(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009478_rage_pfdrawmanager_senddrawcameraforserver_rage_matrix34_const_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009479_rage_getrageprofiledraw(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009498_rage_grcfvf_grcfvf_rage_grcfvf_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_009499_rage_grcfvf_setposchannel_bool_rage_grcfvf_grcdatasize_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009500_rage_grcfvf_setnormalchannel_bool_rage_grcfvf_grcdatasize(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009501_rage_grcfvf_setdiffusechannel_bool_rage_grcfvf_grcdatasize(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009506_rage_grcfvf_settexturechannel_int_bool_rage_grcfvf_grcdatasize(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009514_rage_grctexture_setcustomloadname_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009515_rage_grctexture_runcustomloadfunc_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009518_rage_grcimage_create_unsigned_int_unsigned_int_unsigned_int_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_009529_rage_grcstateblock_createsamplerstate_rage_grcsamplerstatedesc_c(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009533_rage_grctexturefactory_createparams_createparams_unsigned_short(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009535_rage_grcrendertarget_logtexture_char_const_ptr_rage_grcrendertar(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009539_rage_grceffect_lookuptechnique_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009540_rage_grceffect_lookupvar_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009541_rage_grcstateblock_getsamplerstatedesc_rage_grcstateblock_grcsam(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009542_rage_grcstateblock_createblendstate_rage_grcblendstatedesc_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009543_rage_grceffect_beginpass_int_rage_grcinstancedata_const_ref_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009544_rage_grcdrawsinglequadf_float_float_float_float_float_float_floa(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: i64); }
extern "C" { pub fn f_009545_rage_grcstateblock_setblendstate_rage_grcstateblock_grcblendstat(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009548_rage_grcdevice_drawvertices_void_const_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_009549_rage_grceffect_endpass_const(ph: *mut Phantom); }
extern "C" { pub fn f_009550_rage_grccompositerendertargethelper_compositeparams_compositepar(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009551_rage_grccompositerendertargethelper_downsampleparams_downsamplep(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009552_rage_grccompositerendertargethelper_compositedsttosrc_rage_grcco(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009555_rage_grcstateblock_setrasterizerstate_rage_grcstateblock_grcrast(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009556_rage_grcbatcher_reserve_unsigned_int_unsigned_long(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_009558_rage_grccompositerendertargethelper_downsampledepth_rage_grccomp(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009559_rage_grcdevice_clear_bool_rage_color32_bool_float_bool_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_009560_rage_grcstateblock_setdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009561_rage_grcbegin_rage_grcdrawmode_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009562_rage_grcvertex_float_float_float_float_float_float_rage_color32(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: f32, a8: f32); }
extern "C" { pub fn f_009563_rage_grcend(ph: *mut Phantom); }
extern "C" { pub fn f_009565_rage_grcrendertargetpoolmgr_getfreemem_unsigned_short_unsigned_c(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_009587_rage_grclightstate_setenabled_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_009588_rage_grcimage_dtorgrcimage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009596_rage_grcimage_setpixel_int_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009601_rage_grcviewport_regeneratematrices(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009603_rage_grcviewport_setworldidentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009607_rage_grcdevice_setwindow_rage_grcwindow_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009608_rage_grcviewport_grcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009609_rage_grcviewport_resetwindow(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009610_rage_grcviewport_setinvertzinprojectionmatrix_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009611_rage_grcviewport_regeneratefrustumplanes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009612_rage_grcviewport_setwindow_int_int_int_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: f32, a6: f32); }
extern "C" { pub fn f_009613_rage_grcviewport_dtorgrcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009614_rage_grcviewport_setcurrent_rage_grcviewport_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009615_rage_grceffect_lookupglobalvar_char_const_ptr_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009616_rage_grcviewport_setnormwindow_float_float_float_float_float_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_009617_rage_grcviewport_regenerateperspective(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009618_rage_grcviewport_perspective_float_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_009619_rage_grcviewport_ortho_float_float_float_float_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_009620_rage_grcviewport_setprojection_rage_mat44v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009621_rage_grcviewport_setnearclip_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_009623_rage_grcviewport_setnearfarclip_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_009624_rage_grcviewport_setworldmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009625_rage_grcviewport_setinstancing_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009630_rage_grcviewport_regenerateinstvpmatrices_rage_mat44v_const_ref(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009632_rage_grcviewport_setinstvpwindow_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009633_rage_grcviewport_getwindow_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009634_rage_grcviewport_setcurrentworldidentity(ph: *mut Phantom); }
extern "C" { pub fn f_009635_rage_grcviewport_setcurrentworldmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009636_rage_grcviewport_setcameramtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009637_rage_grcviewport_setcameraidentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009638_rage_grcviewport_isspherevisible_rage_vec4v_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009640_rage_grcviewport_isspherevisible_float_float_float_float_float_p(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_009641_rage_grcviewport_isaabbvisible_float_vector_4_float_vector_4_rag(ph: *mut Phantom, a0: u128, a1: u128, a2: i64) -> i32; }
extern "C" { pub fn f_009642_rage_grcviewport_getaabbcullstatus_float_vector_4_float_vector_4(ph: *mut Phantom, a0: u128, a1: u128, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_009643_rage_grcviewport_ispointsetvisible_rage_vec3v_const_ptr_int_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009644_rage_grcviewport_screen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009645_rage_grcviewport_getunclippedwindow_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009646_rage_grcviewport_transform_rage_vec3v_const_ref_float_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009647_rage_grcviewport_transform_rage_vec3v_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009648_rage_grcviewport_reversetransform_float_float_rage_vec3v_ref_rag(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i64); }
extern "C" { pub fn f_009649_rage_grcworldidentity(ph: *mut Phantom); }
extern "C" { pub fn f_009650_rage_grcbindtexture_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009653_rage_grcviewport_op_rage_grcviewport_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009675_rage_grcstateblock_createdepthstencilstate_rage_grcdepthstencils(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009689_rage_grcdevice_isfencepending_rage_grcfencehandle_ptr_unsigned_i(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009693_rage_grcgputimer_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009694_rage_grcsetup_takescreenshot_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009696_rage_grcdevice_capturescreenshottojpegfile_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009697_rage_grcgputimer_stop(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_009698_rage_grcdraw2dtext_float_float_char_const_ptr_bool_float_float_r(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i32, a4: f32, a5: f32, a6: i64); }
extern "C" { pub fn f_009699_rage_grcdevice_capturescreenshot_rage_grcimage_ptr(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009715_rage_grcvertexbuffer_createwithdata_int_rage_grcfvf_const_ref_ra(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_009716_rage_grcdevice_createvertexdeclaration_rage_grcvertexelement_con(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009718_rage_grcbufferd3d11_initialise_unsigned_int_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32); }
extern "C" { pub fn f_009719_rage_grcdevice_destroyvertexdeclaration_rage_grcvertexdeclaratio(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009720_rage_fastquad_draw_bool(ph: *mut Phantom); }
extern "C" { pub fn f_009721_rage_grcstateblock_doflush(ph: *mut Phantom); }
extern "C" { pub fn f_009722_rage_grcdevice_setuppriortodraw_rage_grcdrawmode(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009723_rage_grcdevice_setvertexdeclaration_rage_grcvertexdeclaration_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009724_rage_grcdevice_setstreamsource_unsigned_int_rage_grcvertexbuffer(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_009725_rage_grcdevice_drawprimitive_rage_grcdrawmode_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_009726_rage_grcbeginquadsinternal_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009727_rage_grcdevice_beginvertices_rage_grcdrawmode_unsigned_int_unsig(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_009728_rage_grcvertexdeclaration_release_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009732_rage_grcbatcher_renderinternal(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009733_rage_grcdrawline_rage_vector3_const_ref_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009736_rage_grcdrawbox_rage_vector3_const_ref_rage_matrix34_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009737_rage_grcdrawsolidbox_rage_vector3_const_ref_rage_matrix34_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009742_rage_grcdrawcircle_float_rage_vector3_const_ref_rage_vector3_con(ph: *mut Phantom, a0: f32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_009744_rage_grcdrawcapsule_float_float_rage_matrix34_const_ref_int_bool(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i32, a4: i32, a5: f32); }
extern "C" { pub fn f_009745_rage_grcdrawcylinder_float_float_rage_matrix34_const_ref_int_boo(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_009746_rage_grcdrawaxis_float_rage_matrix34_const_ref_bool(ph: *mut Phantom, a0: f32, a1: i64, a2: i32); }
extern "C" { pub fn f_009752_rage_grcworldmtx_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009753_rage_grcworldmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009754_rage_grclighting_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_009756_rage_grcdrawbox_rage_vector3_const_ref_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009757_rage_grcdrawsolidbox_rage_vector3_const_ref_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009758_rage_grcdrawsphere_float_rage_vector3_const_ref_int_bool_bool(ph: *mut Phantom, a0: f32, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_009761_rage_grcdrawlabel_rage_vector3_const_ref_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009762_rage_grcdrawlabelf_rage_vector3_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009765_rage_grcvertexbuffer_create_int_rage_grcfvf_const_ref_bool_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_009766_rage_grcvertexbuffer_create_int_rage_grcfvf_const_ref_rage_grcbu(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_009768_rage_grcvertexbuffer_makereadonly(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009769_rage_grcvertexbuffereditor_grcvertexbuffereditor_rage_grcvertexb(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_009771_rage_grcvertexbuffereditor_dtorgrcvertexbuffereditor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009774_rage_grcbuffereditor_setposition_int_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009775_rage_grcbuffereditor_setpositionandbinding_int_rage_vector4_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009777_rage_grcbuffereditor_getposition_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009801_rage_grcimage_savergba8surfacetojpeg_char_const_ptr_int_void_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_009802_rage_grcimage_savergba8surfacetojpeg_rage_fistream_ptr_int_void(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_009803_rage_grcimage_loadjpegtorgba8surface_rage_fistream_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_009805_rage_grcimage_getjpeginfoforrgba8surface_rage_fistream_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_009808_rage_grceffect_getvardesc_rage_grceffectvar_char_const_ptr_ref_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_009813_rage_grctexchangedata_updatetextureparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009814_rage_grceffect_setvar_rage_grcinstancedata_ref_rage_grceffectvar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009815_rage_grcinstancedata_addwidgets_withload_rage_bkbank_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_009819_rage_grceffect_getsamplerstate_rage_grcinstancedata_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009858_rage_grcstateblock_createrasterizerstate_rage_grcrasterizerstate(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009859_rage_grceffect_setglobalfloatcommon_int_float_ptr_int_rage_grcef(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_009863_rage_grceffect_findtechniquegroupid_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009866_rage_grcinstancedata_grcinstancedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009867_rage_grcinstancedata_dtorgrcinstancedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009869_rage_grceffect_getlocalcommon_rage_grcinstancedata_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_009872_rage_grceffect_setglobalvar_rage_grceffectglobalvar_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009873_rage_grceffect_setglobalvar_rage_grceffectglobalvar_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009874_rage_grceffect_setglobalvar_rage_grceffectglobalvar_float_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_009875_rage_grceffect_setlocalcommon_rage_grcinstancedata_ref_rage_grce(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_009876_rage_grceffect_setvar_rage_grcinstancedata_ref_rage_grceffectvar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009877_rage_grceffect_setvar_rage_grcinstancedata_ref_rage_grceffectvar(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_009878_rage_grceffect_setvaruav_rage_grcinstancedata_ref_rage_grceffect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_009879_rage_grceffect_setvaruav_rage_grcinstancedata_ref_rage_grceffect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_009880_rage_grceffect_getvar_rage_grcinstancedata_const_ref_rage_grceff(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009881_rage_grceffect_setglobalvar_rage_grceffectglobalvar_rage_grctext(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009885_rage_grceffect_setsamplerstate_rage_grcinstancedata_ref_rage_grc(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009886_rage_grceffect_pushsamplerstate_rage_grcinstancedata_ref_rage_gr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009887_rage_grceffect_popsamplerstate_rage_grcinstancedata_ref_rage_grc(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009888_rage_grceffect_setstencilrefmask_unsigned_char(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009889_rage_grccomputeprogram_bind_rage_grcinstancedata_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009893_rage_grcstateblock_setdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009894_rage_grcstateblock_setblendstate_rage_grcstateblock_grcblendstat(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_009897_rage_grcgputimestamp_dtorgrcgputimestamp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009898_rage_grcstateblock_destroyblendstate_rage_grcstateblock_grcblend(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009899_rage_grcstateblock_destroyrasterizerstate_rage_grcstateblock_grc(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009901_rage_grcstateblock_destroysamplerstate_rage_grcstateblock_grcsam(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009902_rage_grcstateblock_setwireframeoverride_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_009903_rage_grcstateblock_setstates_rage_grcstateblock_grcrasterizersta(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_009904_rage_gridmonitor_getarea_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009905_rage_monitorconfiguration_ismultihead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009908_rage_monitorconfiguration_update(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009911_rage_monitorconfiguration_getcentralmonitor_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009912_rage_monitorconfiguration_getlandscapemonitor_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009913_rage_grcdevice_getmanufacturer_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_009938_rage_basecs_dtorbasecs(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009939_rage_basecs_run_char_const_ptr_unsigned_short_unsigned_short_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_009940_rage_grcdevice_runcomputation_char_const_ptr_rage_grmshader_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_009942_rage_csmanagerbase_dtorcsmanagerbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009943_rage_grcend_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009951_rage_grcvertexbufferd3d11_lockinternal_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009952_rage_grcvertexbufferd3d11_unlockinternal_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009962_rage_grccbuffer_grccbuffer_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_009963_rage_grccbuffer_dtorgrccbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009965_rage_grccbuffer_getdataptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009966_rage_grccbuffer_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009967_rage_grccbuffer_lockdirect_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009968_rage_grccbuffer_unlockdirect(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009986_rage_grcdebugdraw_adddebugoutputex_bool_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_009994_rage_grcdevice_clearstreamsource_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009995_rage_grcdevice_disablescissor(ph: *mut Phantom); }
extern "C" { pub fn f_009997_rage_grcdevice_lockcontext(ph: *mut Phantom); }
extern "C" { pub fn f_009999_rage_grcdevice_unlockcontext(ph: *mut Phantom); }
extern "C" { pub fn f_010002_rage_grcdevice_setvertexshaderconstantbufferoverrides_unsigned_i(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_010003_rage_grcdevice_setscissor_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_010005_rage_grcdevice_copystructurecount_rage_grcbufferd3d11_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_010006_rage_grcdevice_setprogramresources(ph: *mut Phantom); }
extern "C" { pub fn f_010007_rage_grcdevice_clearrect_int_int_int_int_bool_rage_color32_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: f32, a7: i32, a8: i32); }
extern "C" { pub fn f_010008_rage_grcdevice_clearuav_bool_rage_grcbufferd3d11resource_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010011_rage_grcdevice_setvertexshaderconstantf_int_float_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_010012_rage_grcdevice_endcreatevertices_void_const_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_010014_rage_grcdevice_setindices_rage_grcindexbuffer_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010020_rage_grcdevice_isready(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_010021_rage_grcdevice_createocclusionquery_int(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_010022_rage_grcdevice_destroyocclusionquery_iunknown_ptr_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010023_rage_grcdevice_beginocclusionquery_iunknown_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010024_rage_grcdevice_endocclusionquery_iunknown_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010025_rage_grcdevice_getocclusionquerydata_iunknown_ptr_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_010026_rage_grcdevice_createconditionalquery(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_010027_rage_grcdevice_beginconditionalrender_iunknown_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010028_rage_grcdevice_endconditionalrender_iunknown_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_010029_rage_grcdevice_drawindexedprimitive_rage_grcdrawmode_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_010031_rage_grcdevice_drawprimitive_rage_grcdrawmode_rage_grcvertexdecl(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010032_rage_grcdevice_drawinstancedprimitive_rage_grcdrawmode_int_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_010033_rage_grcdevice_drawinstancedindexedprimitive_rage_grcdrawmode_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_010034_rage_grctexturedx11_updatecpucopy_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010035_rage_grctexturedx11_mapstagingbuffertobackingstore_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_010042_rage_grcdevice_setdepthboundstestenable_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_010043_rage_grcdevice_setuavsync_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010069_rage_grctexturedx11_getstride_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_010070_rage_grctexturedx11_getrowcount_unsigned_int_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_010072_rage_grctexturedx11_begintempcpuresource(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010073_rage_grctexturedx11_endtempcpuresource(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010077_rage_grctexturedx11_updategpucopy_withthreadsyncing_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_010078_rage_grctexturedx11_stripimmutability_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010079_rage_grctexturedx11_copyfromgputostagingbuffer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010080_rage_grctexturedx11_initializetempstagingtexture(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010082_rage_grctexturedx11_releasetempstagingtexture(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010111_rage_grctexturefactorydx11_createrendertarget_char_const_ptr_id3(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_010119_rage_grctexturefactorydx11_lockmrtwithuav_rage_grcrendertarget_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_010120_rage_grctexturefactorydx11_lockdepthandcurrentcolortarget_rage_g(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010121_rage_grctexturefactorydx11_unlockdepthandcurrentcolortarget_rage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010135_rage_grcrendertargetdx11_updatelockabletexture_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010144_rage_grcbufferd3d11_grcbufferd3d11(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010145_rage_grcbufferd3d11_dtorgrcbufferd3d11(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010148_rage_grcbufferd3d11resource_grcbufferd3d11resource(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010149_rage_grcbufferd3d11resource_dtorgrcbufferd3d11resource(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010150_rage_grcbufferd3d11resource_initialise_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_010153_rage_batcherlock_lock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010154_rage_batcherlock_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010159_rage_grcdebugdraw_setdisplaydebugtext_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010163_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010164_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010166_rage_grcdebugdraw_quad_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010167_rage_grcdebugdraw_poly_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010169_rage_grcdebugdraw_circle_rage_vec2v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010170_rage_grcdebugdraw_rectaxisaligned_rage_vec2v_const_ref_rage_vec2(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010171_rage_grcdebugdraw_meter_rage_vec2v_const_ref_rage_vec2v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_010172_rage_grcdebugdraw_text_float_float_rage_color32_char_const_ptr_b(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i64, a4: i32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010173_rage_grcdebugdraw_metervalue_rage_vec2v_const_ref_rage_vec2v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010174_rage_grcdebugdraw_clearsubrenderbatcher2_rage_grcdebugdraw_linge(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010179_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010180_rage_grcdebugdraw_quad_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_010181_rage_grcdebugdraw_poly_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010183_rage_grcdebugdraw_cross_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010184_rage_grcdebugdraw_circle_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010185_rage_grcdebugdraw_arc_rage_vec3v_const_ref_float_rage_color32_ra(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i64, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010188_rage_grcdebugdraw_boxaxisaligned_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010189_rage_grcdebugdraw_boxoriented_rage_vec3v_const_ref_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010190_rage_grcdebugdraw_cylinder_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010192_rage_grcdebugdraw_cone_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010193_rage_grcdebugdraw_capsule_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010194_rage_grcdebugdraw_axis_rage_mat34v_const_ref_float_bool_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_010195_rage_grcdebugdraw_arrow_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_010201_rage_grcdebugdraw_setdefaultdebugrenderstates(ph: *mut Phantom); }
extern "C" { pub fn f_010203_rage_grcdebugdraw_closebatcher_rage_grcdebugdraw_lingerbuffer(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010204_rage_grcdebugdraw_getnumbatchers_rage_grcdebugdraw_lingerbuffer(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_010207_rage_grcdebugdraw_clearrenderstate(ph: *mut Phantom); }
extern "C" { pub fn f_010210_rage_grcdebugdraw_setdodebugztest_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010214_rage_grcdebugdraw_textfontpush_rage_grcfont_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010215_rage_grcdebugdraw_textfontpop(ph: *mut Phantom); }
extern "C" { pub fn f_010216_rage_grcdebugdraw_text_float_float_rage_dd_epixelcoordspace_rage(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i32, a4: i64, a5: i32, a6: f32, a7: f32); }
extern "C" { pub fn f_010217_rage_grcdebugdraw_printtoscreencoors_char_const_ptr_int_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010219_rage_grcdebugdraw_adddebugoutputex_bool_rage_color32_char_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_010220_rage_grcdebugdraw_adddebugoutput_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010221_rage_grcdebugdraw_adddebugoutput_rage_color32_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_010222_rage_grcdebugdraw_adddebugoutputseparator_int(ph: *mut Phantom); }
extern "C" { pub fn f_010224_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010225_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010481_rage_iomapper_iomapper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010482_rage_iomapper_map_rage_iomappersource_unsigned_int_rage_iovalue(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_010500_rage_iovalue_isdown_float_rage_iovalue_readoptions_const_ref_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010501_rage_iovalue_historyhelddown_unsigned_int_float_rage_iovalue_rea(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_010503_rage_iovalue_isup_float_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010506_rage_iovalue_historypressedandheld_unsigned_int_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_010511_rage_iovalue_ispressed_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010512_rage_iovalue_historypressed_unsigned_int_unsigned_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_010515_rage_iovalue_historyreleased_unsigned_int_unsigned_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_010518_rage_iovalue_getnorm_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010519_rage_iovalue_getunboundnorm_rage_iovalue_readoptions_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010617_jdiv_round_up_long_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_010688_rage_rmclodgroup_computebucketmask_rage_grmshadergroup_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010700_rage_rmclodgroup_getbucketmask_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_010701_rage_rmcdrawable_drawnoshaderstessellationcontrolled_rage_matrix(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010704_rage_rmcdrawable_drawskinnednoshaderstessellationcontrolled_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010707_rage_rmcdrawable_rmcdrawable_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_010712_rage_rmclodgroup_islodinstanced_int_rage_grmshadergroup_const_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_010713_rage_rmclodgroup_settessellateddrawmode_rage_rmclodgroup_tessell(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010747_rage_spdtransposedplaneset8_set_rage_grcviewport_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_010895_rage_bwmovie_begindraw_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011443_rage_cutsmanager_isconcatsectionvalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011734_rage_fraginst_getarticulatedbody(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011737_rage_fraginst_fraginst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011739_rage_fraginst_fraginst_rage_fragtype_const_ptr_rage_matrix34_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_011746_rage_fraginst_dtorfraginst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011748_rage_fraginst_getpartbroken_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011749_rage_fraginst_getgroupbroken_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011750_rage_fraginst_getchildbroken_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011751_rage_fraginst_getcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011753_rage_fraginst_getcontrollingcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011755_rage_fraginst_getgroupfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011756_rage_fragtype_getgroupfromboneindex_int_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011761_rage_fragcachemanager_cangetnewcacheentries_unsigned_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011771_rage_fragcacheentry_calccenterofgravity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011775_rage_fragmemstartcacheheapfunc_rage_fragcacheentry_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011776_rage_fragtype_getboneindexfromid_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011777_rage_fragmemendcacheheap(ph: *mut Phantom); }
extern "C" { pub fn f_011779_rage_fraginst_reducegroupdamagehealth_int_float_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_011791_rage_phcontactconstiterator_op(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011796_rage_fraginst_findbreakstrength_rage_vector3_const_ptr_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_011797_rage_fraginst_shouldfindimpacts_rage_phinst_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_011798_rage_fraginst_precomputeimpacts_rage_phcontactiterator(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011802_rage_fraginst_collisiondamage_rage_fraginst_collision_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011808_rage_fraginst_isbreakable_rage_phinst_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_011812_rage_fraginst_putintocache(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011828_rage_fraginst_setcurrentphysicslod_rage_fraginst_ephysicslod(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011837_rage_fraginst_prepareforactivation_rage_phcollider_ptr_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_011850_rage_fraginst_shouldbeincachetodraw_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011853_rage_fragmanager_createallocator(ph: *mut Phantom); }
extern "C" { pub fn f_011861_rage_fragtype_profiledraw_int_rage_matrix34_const_ref_rage_vecto(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_011868_rage_fragtunestruct_dtorfragtunestruct(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011872_rage_fragtunestruct_refreshtypelist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011876_rage_fragphysicslod_gettotalundamagedmass_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_011885_rage_phiterator_dtorphiterator(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011961_rage_fragtype_fragtype_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_011964_rage_fragtype_getskeletondata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011966_rage_fragtype_getskinned_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012021_rage_fragphysicslodgroup_getlodbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012022_rage_fragtype_dtorfragtype(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012036_rage_fragcacheentry_setskeletondata_rage_crskeletondata_const_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012170_rage_ptxgpudroprendersettings_ptxgpudroprendersettings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012177_rage_ptxgpudrop_getemitterbounds_rage_mat44v_const_ref_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_012183_rage_ptxeffectinst_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012196_rage_ptxeffectinst_setbasemtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012202_rage_ptxeffectinst_setoffsetmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012207_rage_ptxeffectinst_setevovaluefromhash_rage_athashvalue_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_012290_rage_ptxshadertemplatelist_getshader_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012295_rage_ptxshadertemplateoverride_init_rage_grmshader_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012296_rage_ptxshadertemplateoverride_binddebugtechniqueoverride_rage_p(ph: *mut Phantom); }
extern "C" { pub fn f_012297_rage_ptxshadertemplateoverride_unbinddebugtechniqueoverride(ph: *mut Phantom); }
extern "C" { pub fn f_012427_rage_ptxmanager_draw_rage_ptxdrawlist_ptr_rage_ptxdrawinterface(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_012429_rage_ptxmanager_drawmanual_rage_ptxeffectinst_ptr_rage_ptxdrawin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012437_rage_ptxmanager_setshadowpass_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012819_rage_clothcontroller_clothcontroller(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012823_rage_clothcontroller_instancefromtemplate_rage_clothcontroller_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012824_rage_clothcontroller_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012843_rage_environmentcloth_getcloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012845_rage_environmentcloth_getoffset(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012849_rage_environmentcloth_getclothframe_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012867_rage_environmentcloth_checkdistance_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012874_rage_environmentcloth_copyverletdrawvertsforrope(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012875_rage_environmentcloth_controlvertex_int_rage_vec3v_const_ref_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_012887_rage_characterclothcontroller_skinmesh_bool_rage_matrix43_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_012892_rage_characterclothcontroller_setpackageindex_unsigned_char_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_012893_rage_characterclothcontroller_resetw(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012900_rage_charactercloth_charactercloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012911_rage_charactercloth_setpose_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012920_rage_ropemodel_addlod_float_int_int_rage_vector2(ph: *mut Phantom, a0: f32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_012921_rage_rmcropedrawable_draw_rage_matrix34_const_ref_unsigned_int_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013019_rage_bgglassmanager_getglassbreakable_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_013020_rage_bgglassmanager_hitglass_short_int_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_013023_rage_bgdrawable_dtorbgdrawable(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013024_rage_bgdrawable_drawpanes_int_int_bool_float_const_ptr_int_int_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_013051_rage_quaternion_transform_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013061_rage_bgbreakable_getpanetriangleinfo_unsigned_int_rage_vector3_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_013062_rage_bgbreakable_getnextpanetriangle_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013097_rage_bgcrackstemplate_initclass(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013098_rage_bgcrackstemplate_loadresources(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013100_rage_bgcrackstemplate_shutdownclass(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013101_rage_atbintreebase_unsigned_int_rage_grctexture_ptr_removenode_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013112_rage_bgglassmanager_getglassdrawable_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_013113_rage_bgglassmanager_free(ph: *mut Phantom); }
extern "C" { pub fn f_013114_rage_bgcrackstemplate_dtorbgcrackstemplate(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013118_rage_bgglassmanager_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013153_rage_rsttransformtoscreenspace_rsttransformtoscreenspace_rage_gr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_013155_rage_rsttestaabbexactedgelist_bool_rage_mat44v_const_ref_rage_sc(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i32, a13: i32, a14: i32); }
extern "C" { pub fn f_013156_rage_softrasterizer_softrasterizer_rage_rsttilelist_ptr_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013159_rage_rstcreatescantiletasks_rage_rstfasttiler_ptr_ptr_rage_vec4v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64); }
extern "C" { pub fn f_013161_rage_dependanttasklist_add_char_const_ptr_void_ptr_rage_systaskp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_013163_rage_softrasterizer_preparesubmittilersdependency(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013166_rage_dependanttasklist_taskdumpdependancygraphrecurse_rage_fistr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013170_rage_softrasterizer_wait(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013180_rage_softrasterizer_rasterizestencilmodel_rage_grcviewport_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_013184_rage_resolvezbuffer_unsigned_char_ptr_bool_float_float_bool_rage(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_013185_rage_softrasterizer_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013188_rage_occludemodel_draw_float_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013189_rage_softrasterizer_setwateroccluderresources_rage_occludemodel(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013193_rage_rstfasttiler_rstfasttiler(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013194_rage_rstfasttiler_generatetilemem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013196_rage_rsttestmodelexact_rage_mat44v_const_ref_rage_scalarv_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_013198_rage_rsttestaabbexact_rage_mat44v_const_ref_rage_scalarv_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_013199_rage_spdkdop2d_8_getuniquepoints_rage_vec2v_ptr_rage_scalarv_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013410_rage_netconnectionmanager_netcriticalsectiontoken_lock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013411_rage_netconnectionmanager_netcriticalsectiontoken_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013429_char_ptr_rage_safecatf_char_char_ptr_unsigned_long_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_013533_rage_netaddress_isrelayserveraddr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013554_rage_netconnectionmanager_getaddress_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_013558_rage_netconnectionmanager_getunackedcount_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013559_rage_netconnectionmanager_getoldestresendcount_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013619_rage_nethardware_getpublicipaddress_rage_netipaddress_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013620_rage_netipaddress_op_rage_netipaddress_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013645_rage_netp2pcrypt_key_dtorkey(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013882_rage_netipv4address_netipv4address(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013885_rage_netipaddress_format_char_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014067_rage_nettimesync_gettime_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014068_rage_nettimesync_hasstarted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014257_rage_is_ui_showing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014284_rage_rlfriendsmanager_isfriendswith_int_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_014316_rage_rlgamerinfo_getgamerhandle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014318_rage_rlgamerinfo_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014387_rage_rlgamerinfo_rlgamerinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014662_rage_rlmetric_dtorrlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014685_rage_rlmetric_rlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015750_rage_phmaterialmgr_dtorphmaterialmgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015776_rage_phworkerthread_phworkerthread_char_const_ptr_void_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_015803_rage_phmanifold_removeallcontacts(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015804_rage_phoverlappingpairarray_phoverlappingpairarray_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_015812_rage_phconstraintbase_getinstancea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015813_rage_phconstraintbase_getinstanceb(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015845_rage_phconstraintdistance_setlocalposa_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015848_rage_phconstraintdistance_setlocalposb_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015882_rage_phconstraintmgr_insert_rage_phconstraintbase_params_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015883_rage_phconstraintmgr_insertandreturntemporarypointer_rage_phcons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_015890_rage_phconstraintmgr_remove_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015891_rage_phconstraintmgr_gettemporarypointer_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015893_rage_phmultireaderlocktoken_waitasreader(ph: *mut Phantom); }
extern "C" { pub fn f_015944_rage_phcontact_computerelvelocity_rage_vec3v_const_ref_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_015953_rage_phmanifold_profiledraw(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015958_rage_phcollider_initinertia(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015965_rage_phcollider_setcollidermatrixfrominstancerigid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015966_rage_phsleep_reset_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015976_rage_phcollider_rotatecollider_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015977_rage_phcollider_setinstancematrixfromcollider(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015988_rage_phcollider_setvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015998_rage_phsimulator_deleteobject_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016005_rage_phsimulator_activateobject_int_rage_phcollider_ptr_rage_phi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016010_rage_phsimulator_getlastinstancematrix_rage_phinst_const_ptr_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016014_rage_phsimulator_addfixedobject_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016021_rage_phsimulator_setgravity_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016031_rage_phsleep_setdefaultveltolerance2_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_016032_rage_phsleep_setdefaultangveltolerance2_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_016033_rage_phsleep_setdefaultinternalveltolerance2_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_016034_rage_pharchetype_pharchetype(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016036_rage_pharchetype_setbound_rage_phbound_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016043_rage_pharchetype_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016045_rage_pharchetype_removeincludeflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016046_rage_pharchetypephys_pharchetypephys(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016057_rage_pharchetypedamp_pharchetypedamp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016073_rage_phasyncshapetestmgr_phasyncshapetestmgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016074_rage_phasyncshapetestmgr_init_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016082_rage_phinst_phinst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016086_rage_phinst_init_rage_pharchetype_ref_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016087_rage_phinst_setarchetype_rage_pharchetype_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016088_rage_phinst_setinstflag_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016106_rage_phlevelbase_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016107_rage_phshapebatch_phshapegroup_rage_phshapesweptsphere_allocate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016115_rage_phshapebatch_phshapegroup_rage_phshapeprobe_allocate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016130_rage_phshapetest_rage_phshapeprobe_phshapetest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016131_rage_phshapetest_rage_phshapeprobe_setexcludeinstancelist_rage_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016138_rage_phshapetest_rage_phshapeprobe_testinlevel_rage_phinst_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_016141_rage_phshapetest_rage_phshapeprobe_getexcludeinstancelist_rage_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016145_rage_phshapetest_rage_phshapeedge_testinlevel(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_016151_rage_phshapetest_rage_phshapesphere_testinlevel(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_016152_rage_phshapetest_rage_phshapeobject_phshapetest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016153_rage_phshapetest_rage_phshapeobject_initobject_rage_phbound_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_016158_rage_phshapetest_rage_phshapeobject_testinlevel_rage_phinst_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_016165_rage_phshapetest_rage_phshapecapsule_phshapetest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016166_rage_phshapetest_rage_phshapecapsule_testoneobject_rage_phbound(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016171_rage_phshapetest_rage_phshapecapsule_testinlevel(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_016172_rage_phshapetest_rage_phshapesweptsphere_phshapetest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016176_rage_phshapetest_rage_phshapesweptsphere_testinlevel(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_016181_rage_phshapetest_rage_phshapebatch_phshapetest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016186_rage_phshapetest_rage_phshapebatch_testinlevel_rage_phinst_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_016188_rage_phshapetest_rage_phshapebatch_getexcludeinstancelist_rage_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016191_rage_phshapebatch_dtorphshapebatch(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016193_rage_phshapebatch_setignorematerialflags_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016194_rage_phshapetest_rage_phshapesphere_initsphere_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_016195_rage_phshapetest_rage_phshapeprobe_initprobe_rage_phsegment_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_016196_rage_phshapetest_rage_phshapebatch_initprobe_rage_phsegment_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_016197_rage_phshapetest_rage_phshapesweptsphere_initsweptsphere_rage_ph(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_016198_rage_phshapetest_rage_phshapebatch_initsweptsphere_rage_phsegmen(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_016199_rage_phshapetest_rage_phshapecapsule_initcapsule_rage_phsegment(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_016255_rage_pharticulatedbody_getangularvelocity_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016256_rage_pharticulatedbody_getlinearvelocity_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016263_rage_pharticulatedbody_setstiffness_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016301_rage_pharticulatedcollider_setcollidermatrixfrominstanceart(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016303_rage_pharticulatedcollider_setpartscancollide_rage_atarray_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_016304_rage_pharticulatedcollider_finalizesettingpartscancollide_rage_a(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016312_rage_phboundcomposite_updatebvh_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016316_rage_pharticulatedcollider_updatepositionfromvelocityimp_float_v(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016317_rage_phboundcomposite_calculatecompositeextents_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016322_rage_pharticulatedcollider_setvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016323_rage_pharticulatedcollider_setangvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016367_rage_phbound_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016368_rage_phbound_gettypestring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016370_rage_phbound_setcgoffset_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016382_rage_phbound_drawangularinertia_rage_mat34v_const_ref_float_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_016384_rage_phbound_istypeconvex_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_016385_rage_phbound_istypecomposite_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_016391_rage_phboundbox_phboundbox_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016395_rage_phboundbox_setboxsize_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016437_rage_phboundcapsule_setcapsuleradius_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016454_rage_phboundcomposite_phboundcomposite(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016455_rage_phboundcomposite_init_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016464_rage_phboundcomposite_setbound_int_rage_phbound_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016471_rage_phboundcomposite_allocatetypeandincludeflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016521_rage_phbound_place_void_ptr_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016530_rage_phboundsphere_phboundsphere_float(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_016531_rage_phboundsphere_setsphereradius_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016590_rage_drawclothwireframe_rage_vec3v_const_ptr_rage_phverletcloth(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: f32, a8: i64); }
extern "C" { pub fn f_016601_rage_phclothdata_swapvertex_int_int_rage_phclothconnectivitydata(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_016609_rage_phverletcloth_drawpickededges_rage_vector3_const_ptr_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_016625_rage_phclothdata_save_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016635_rage_phintersection_getinstance(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016637_rage_drawprimitive_rage_phboundpolyhedron_ptr_int_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: f32); }
extern "C" { pub fn f_016639_rage_phoptimizedbvh_unquantize_rage_vector3_ref_short_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016640_rage_renderbvhsubtree_rage_phboundpolyhedron_ptr_int_rage_matrix(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_016641_rage_renderbvhsubtree_rage_phboundpolyhedron_ptr_int_rage_matrix(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64); }
extern "C" { pub fn f_016660_rage_phwind_getglobalvelocity_rage_windtype_e_rage_vec3v_ref_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_016661_rage_phwind_getlocalvelocity_rage_vec3v_const_ref_rage_vec3v_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_016737_rage_phcontactmgr_markscratchpadused_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016876_rage_phlevelnodetree_rage_phlooseoctreenode_initbroadphase_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016878_rage_phlevelnodetree_rage_phlooseoctreenode_getuserdata_int_cons(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_016886_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectlocation(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016892_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectlocation(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016893_rage_phlevelnodetree_rage_phlooseoctreenode_rebuildcompositebvh(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016895_rage_phlevelnodetree_rage_phlooseoctreenode_compositeboundsetbou(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016901_rage_phlevelnodetree_rage_phlooseoctreenode_setlastinstancematri(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016907_rage_phlevelnodetree_rage_phlooseoctreenode_getfirstculledobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016908_rage_phlevelnodetree_rage_phlooseoctreenode_getnextculledobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016910_rage_phlevelnodetree_rage_phlooseoctreenode_testsphere_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_016943_rage_scrthread_printstacktrace_void_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016944_rage_scrthread_preprintstacktrace(ph: *mut Phantom); }
extern "C" { pub fn f_016949_rage_scrthread_getactivethread(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_017186_rage_crclip_composite_rage_crframe_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_017261_rage_atarray_rage_vec3v_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017763_rage_crmtmotiontree_iterate_rage_crmtiterator_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017772_rage_crmtiterator_crmtiterator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017966_rage_crmtnoden_getweight_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_018477_grefcountntsimpl_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018496_gmatrix3d_setidentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018545_gstring_gstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018765_grenderer_cxform_cxform(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018888_cgamelogic_isrunninggta5map(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_018921_rage_fwentity_getmodelid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018988_cminimap_isinbigmap(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_018997_cgps_shouldgpsbevisible_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_019000_cgps_getroutelength_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_019001_cgps_getroutefound_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_019099_cpopulationhelpers_getconfigvalue_int_const_ref_int_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019103_cpopgenshape_init_rage_vector3_const_ref_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32); }
extern "C" { pub fn f_019189_cnetrespawnmgr_startsearch_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_019194_cnetrespawnmgr_getsearchresults_int_rage_vector3_ref_float_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_019288_caitarget_caitarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019289_caitarget_caitarget_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019290_caitarget_setentity_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019292_caitarget_setentityandoffset_centity_const_ptr_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019293_caitarget_caitarget_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019294_caitarget_setposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019295_caitarget_caitarget_caitarget_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019296_caitarget_getisvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019297_caitarget_getposition_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019301_csyncedentity_serialise_rage_csyncdatabase_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019307_cscenarioblockingareas_ispointinsideblockingareas_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_019310_cexpensiveprocess_cexpensiveprocess_cexpensiveprocess_processtyp(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_019312_centityscanner_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019313_cexpensiveprocess_dtorcexpensiveprocess(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019314_centityscanner_scanforentitiesinrange_centity_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019318_centityscanner_debugrender_rage_color32_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019325_cspatialarray_findinsphere_rage_vec3v_const_ref_float_cspatialar(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_019334_cobjectscanner_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019336_cobjectscanner_scanforentitiesinrange_centity_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019340_cexpensiveprocessdistributer_addprocess_cexpensiveprocess_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019341_cexpensiveprocessdistributer_removeprocess_cexpensiveprocess_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019344_cspatialarray_remove_cspatialarraynode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019350_cspatialarray_findnearsegment_rage_vec3v_const_ref_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_019358_cambientmodelvariations_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_019369_cambientmodelset_getcontainsmodel_unsigned_int_cambientmodelvari(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_019423_cconditionalanimsgroup_hasanimationdata_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019425_cconditionalanimsgroup_checkformatchingpopulationconditionalanim(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019443_caidebuginfo_caidebuginfo_caidebuginfo_debuginfoprintparams(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019450_char_ptr_rage_formatf_char_100ul_char_ref_100ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019451_ailogging_getbooleanasstring_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_019453_ailogging_getdynamicentitynamesafe_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019454_ailogging_logentityrejectionwithidentifierandreason_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019455_ailogging_logentityacceptancewithidentifierandreason_char_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019456_ailogging_functionlog_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019458_ailogging_stacktracelog_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019461_ailogging_getentitydescription_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019462_cailogmanager_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019464_cailogmanager_printdebuginfosforpedtolog_cped_ref_cvehicle_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019465_cailogmanager_printdebuginfosforvehicletolog_cvehicle_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019466_ctasksfulldebuginfo_ctasksfulldebuginfo_cped_const_ptr_caidebugi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019467_canimationdebuginfo_canimationdebuginfo_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019468_cikdebuginfo_cikdebuginfo_cped_const_ptr_caidebuginfo_debuginfop(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019469_cattachmentsdebuginfo_cattachmentsdebuginfo_cphysical_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019470_ccombattextdebuginfo_ccombattextdebuginfo_cped_const_ptr_caidebu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019471_cdamagedebuginfo_cdamagedebuginfo_cped_const_ptr_bool_caidebugin(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_019472_cmovementtextdebuginfo_cmovementtextdebuginfo_cped_const_ptr_cai(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019473_cpedflagsdebuginfo_cpedflagsdebuginfo_cped_const_ptr_caidebuginf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019474_crelationshipdebuginfo_crelationshipdebuginfo_cped_const_ptr_cai(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019475_cwanteddebuginfo_cwanteddebuginfo_cped_const_ptr_caidebuginfo_de(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019476_cvehiclelockdebuginfo_cvehiclelockdebuginfo_cped_const_ptr_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_019477_ceventsdebuginfo_ceventsdebuginfo_cped_const_ptr_rage_fwflags_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_019478_ctargetingdebuginfo_ctargetingdebuginfo_cped_const_ptr_caidebugi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019479_cvehiclelockdebuginfo_dtorcvehiclelockdebuginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019507_canimationdebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019508_cikdebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019509_cattachmentsdebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019510_ccombattextdebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019511_cdamagedebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019512_ceventsdebuginfo_geteventhistoryentrydescription_cpedintelligenc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019513_ceventsdebuginfo_geteventhistoryentrystatedescription_cpedintell(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019514_ceventsdebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019515_cpedflagsdebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019526_cmovementtextdebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019527_cmovementtextdebuginfo_visualise(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019529_crelationshipdebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019531_ctasksfulldebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019540_ctargetingdebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019541_ctargetingdebuginfo_visualise(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019543_cvehiclelockdebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019544_cvehiclelockdebuginfo_visualise(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019563_cwanteddebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019564_cwanteddebuginfo_visualise(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019601_cfacialdatacomponent_playfacialidleclip_cped_ref_rage_fwmvtempla(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_019609_cfacialdatacomponent_playfacialanim_cped_ptr_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019611_cfacialdatacomponent_playpainfacialanim_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019614_cfacialdatacomponent_requestfacialidleclip_cfacialdatacomponent(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019634_cmoveped_settasknetwork_rage_fwmovenetworkplayer_ptr_float_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_019636_cmoveped_setadditivenetwork_rage_fwmovenetworkplayer_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_019667_cmoveped_switchtoanimated_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_019770_cdebugclipselector_dtorcdebugclipselector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019784_cdebugclipselector_addwidgets_rage_bkgroup_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_019791_cdebugclipselector_getselectedclipdictionary_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019792_cdebugclipselector_getselectedclipname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019806_rage_atarray_rage_vector3_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020477_canimscenemanager_processdebugrender(ph: *mut Phantom); }
extern "C" { pub fn f_021190_audradioaudioentity_findaudiblestation_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_021223_rage_auddebugdrawmanager_drawlinef_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_021224_audradiostation_getnextmixstationbeat_float_ref_float_ref_int_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_021225_audradioslot_drawslotdebug_rage_auddebugdrawmanager_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021226_audstreamslot_drawdebug_rage_auddebugdrawmanager_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021237_audmusiceventmanager_triggerevent_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_021351_cped_shouldbedead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021359_auddamagestats_dtorauddamagestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021366_audspeechaudioentity_inflictpain_auddamagestats_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021405_audspeechaudioentity_ispainplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021406_audspeechaudioentity_isscriptedspeechplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021407_audspeechaudioentity_isambientspeechplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021415_bool_creplaymgrinternal_recordfx_cpacketpedclothsound_cpacketped(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021421_audpedaudioentity_getplayerimpactmagoverride_float_centity_ptr_r(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64) -> f32; }
extern "C" { pub fn f_021425_audpedaudioentity_setlastscrapetime(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021426_audpedaudioentity_triggerscrapestartimpact_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_021454_audpedfootstepaudio_updatevariationdata_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021575_audradiotrack_findtextidforsoundhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021580_audradioaudioentity_stopreplaymusic_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021587_audreplayaudioentity_stop_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_021588_audreplayaudioentity_preload_int_unsigned_int_audreplaystreamtyp(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_021589_audreplayaudioentity_isloaded_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_021590_audreplayaudioentity_isplaying_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_021591_audreplayaudioentity_play_int_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_021593_audradiotrack_iscommercial_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021609_audradiotrack_drawdebug_rage_auddebugdrawmanager_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021645_audradioaudioentity_getcachednextaudiblebeat_float_ref_float_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_021754_audreplayaudioentity_stop_int_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_021761_audspeechaudioentity_saywhensafe_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_021781_audspeechmanager_getpossibleconversationtopicsforpvg_unsigned_in(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021939_audspeechaudioentity_maketrevorrage(ph: *mut Phantom); }
extern "C" { pub fn f_021957_audspeechaudioentity_say_char_const_ptr_audspeechinitparams_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_022023_rage_fwentity_getskeletondata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022100_audvehicleaudioentity_triggersplash_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_022125_audvehiclecollisionaudio_processcontact_vfxcollisioninfo_s_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_022137_rage_atrangearray_audvehicleconductor_vehssireninfo_2_dtoratrang(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022381_void_creplayeventmanager_storeevent_cpacketsoundphonering_cpacke(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022386_naaudioentity_naaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022391_naenvironmentgroup_allocate_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022397_naaudioentity_createdeferredsound_rage_audmetadataref_centity_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_022400_audcachedlos_clear(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022601_audcollisionevent_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022604_audcollisionevent_dtoraudcollisionevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022612_audcollisionaudioentity_getmaterialoverride_centity_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_022616_audcollisionaudioentity_getaudioweight_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022617_audcollisionaudioentity_computepitchandvolumefromscrapespeed_aud(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_022631_audcollisionaudioentity_processimpactsounds_audcollisionevent_pt(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022641_audcollisionaudioentity_reportuproot_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022644_audcollisionaudioentity_reportfragmentbreak_rage_phinst_const_pt(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022647_audcollisionaudioentity_handleentityswinging_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022653_audcollisionaudioentity_getrollcollisioneventfromhistory_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_022654_audcollisionaudioentity_handleplayerevent_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022655_audcollisionaudioentity_getfreecollisioneventfromhistory_unsigne(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022658_cped_isdead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022659_naenvironmentgroup_init_naaudioentity_ptr_float_unsigned_int_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_022660_naenvironmentgroup_setsource_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022663_audcollisionaudioentity_processscrapesounds_audcollisionevent_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022665_audglassaudioentity_playshardcollision_centity_ptr_float_rage_ph(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_022666_auddooraudioentity_triggerdoorimpact_cped_const_ptr_rage_vec3v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_022767_auddynamicmixer_printdebug_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022777_auddynmixpatch_debugdraw_audnorthdebugdrawmanager_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022778_audnorthdebugdrawmanager_pushsection_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022809_naenvironmentgroup_forcesourceenvironmentupdate_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022864_audfrontendaudioentity_setclipmusicvolumeindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022865_audfrontendaudioentity_setclipsfxvolumeindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022866_audfrontendaudioentity_setclipdialogvolumeindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022867_audfrontendaudioentity_setclipcustomambiencevolumeindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022868_audfrontendaudioentity_setclipsfxoneshotvolumeindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022874_rage_slowmotype_tostring_rage_slowmotype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_023363_rage_atrangearray_audgunfightconductor_combatpeddata_4_dtoratran(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023380_audnorthdebugdrawmanager_drawline_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023396_cpacketevent_cpacketevent_ereplaypacketid_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023467_creplayoverlay_updaterecord_frameref_const_ref_caddressinreplayb(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_023492_cpacketpedstandingmaterial_cpacketpedstandingmaterial_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_023563_quantizeu8_float_float_float_int(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> i32; }
extern "C" { pub fn f_023719_replayfilemanager_getavailablediskspace(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_023733_replayfilemanager_isclipfilevalid_unsigned_long_char_const_ptr_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_023734_replayfilemanager_getclipuid_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023738_replayfilemanager_issaving_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023739_replayfilemanager_isloading_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023836_creplayeventmanager_checkmonitoredevents_creplaypacketbuffer_ref(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023837_creplayeventmanager_freetrackablesthisframeduringrecording(ph: *mut Phantom); }
extern "C" { pub fn f_023838_creplaytrackingcontroller_replaytrackedsound_checkalltrackablesd(ph: *mut Phantom); }
extern "C" { pub fn f_023839_creplaytrackingcontroller_replaytrackedscene_checkalltrackablesd(ph: *mut Phantom); }
extern "C" { pub fn f_023840_creplaytrackingcontroller_rage_ptxeffectinst_ptr_checkalltrackab(ph: *mut Phantom); }
extern "C" { pub fn f_023841_creplaytrackingcontroller_unsigned_long_checkalltrackablesduring(ph: *mut Phantom); }
extern "C" { pub fn f_023842_creplaytrackingcontroller_ptxfireref_checkalltrackablesduringrec(ph: *mut Phantom); }
extern "C" { pub fn f_023843_creplaytrackingcontroller_int_checkalltrackablesduringrecording(ph: *mut Phantom); }
extern "C" { pub fn f_023844_creplaymgrinternal_recordframe_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_023845_creplaymgrinternal_updatebanks_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_023866_creplaymgrinternal_unfreezeaudio(ph: *mut Phantom); }
extern "C" { pub fn f_023868_creplaymgrinternal_setnextplaybackstate_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_023872_creplaymgr_shouldincreasefidelity(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023877_creplaymgr_recordobject_cobject_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023879_creplaymgr_extractcachedwaterframe_float_ptr_float_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023881_creplaymgr_hash_unsigned_int_const_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023882_creplaymgr_recordcloth_cphysical_ptr_rage_clothcontroller_ptr_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_023906_creplaycontrol_isplayeroutofcontrol(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023930_creplayeventmanager_shouldproceedwithpacket_cpacketevent_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023939_replayobjectextension_setprophash_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023940_replayobjectextension_settexthash_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023941_replayobjectextension_setanchorid_centity_ptr_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023942_replayobjectextension_setstreamedpedprop_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023944_replaypedextension_setpeddecorationdata_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: f32); }
extern "C" { pub fn f_023945_replaypedextension_clearpeddecorationdata_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023967_creplayinterfacemanager_ensureisbeingrecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023968_creplayinterfacemanager_ensurewillberecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023998_rage_fwmodelid_fwmodelid_rage_fwmodelid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_024238_creplaymgrinternal_quitreplaycallback(ph: *mut Phantom); }
extern "C" { pub fn f_024257_creplaymgrinternal_isusingrecordedcamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024266_creplaymgrinternal_isawareofworldforrecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024286_creplaymgrinternal_isplaybackflagset_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_024292_creplaymgrinternal_loadreplaylengthfromfilefloat_unsigned_long_c(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_024294_creplaymgrinternal_beginplayback_ireplayplaybackcontroller_ref(ph: *mut Phantom); }
extern "C" { pub fn f_024296_creplaymgrinternal_cleanupplayback(ph: *mut Phantom); }
extern "C" { pub fn f_024302_creplaymgrinternal_oncreateentity_cphysical_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024303_creplaymgrinternal_ondeleteentity_cphysical_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024304_creplaymgrinternal_getbuildinginfo_cphysical_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_024305_creplaymgrinternal_recordmapobject_centity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024314_creplaymarkercontext_reset(ph: *mut Phantom); }
extern "C" { pub fn f_026165_caminterface_isdominantrendereddirector_cambasedirector_const_re(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026167_caminterface_isdominantrenderedcamera_cambasecamera_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026169_cammanager_isrenderingfirstpersoncamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026170_caminterface_isdominantrenderedcameraanyfirstpersoncamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026171_caminterface_setscriptedcameraisfirstpersonthisframe_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026174_camgameplaydirector_getfirstpersonshootercamera_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026176_caminterface_getplayercontrolcamaimframe(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026179_camcinematicdirector_isrenderingcinematicmountedcamera_bool_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026181_caminterface_findfollowped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026188_caminterface_getviewporttanfovh(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_026194_caminterface_isfadedout(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026197_caminterface_renderfade(ph: *mut Phantom); }
extern "C" { pub fn f_026202_caminterface_isspherevisibleingameviewport_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_026203_caminterface_isspherevisibleingameviewport_rage_vec4v_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026205_cqueriableinterface_istaskcurrentlyrunning_int_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_026212_caminterface_isrenderedcamerainsidevehicle(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026213_caminterface_computeshouldmakepedheadinvisible_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026219_cdynamicentity_getboneindexfrombonetag_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026225_cammanager_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026226_cammanager_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026227_rage_atarray_trenderedcameraobjectsettings_0_unsigned_short_dest(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026248_rage_vector3_normalizesafe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026259_rage_fwentity_getskeleton_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026260_camcinematicmountedcamera_getcontrolhelper_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026262_camcinematicdirector_isrenderinganyinvehiclefirstpersoncinematic(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026286_camgameplaydirector_getthirdpersonaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026288_camgameplaydirector_getfollowvehiclecamera_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026329_worldprobe_cshapetestresults_dtorcshapetestresults(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026367_rage_imp_transform_imp34_rage_mat34v_ref_float_vector_4_float_ve(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: u128, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_026368_cpedintelligence_geteventoftype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_026373_rage_atvector_rage_fwregdref_centity_const_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026457_camcontrolhelper_getviewmodeforcontext_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_026541_rage_atvector_rage_fwregdref_centity_const_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026623_rage_matrix34_toeulers_rage_vector3_ref_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026629_camgameplaydirector_getfirstpersonaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026636_camcutscenedirector_iscutsceneplaying(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026648_camdebugdirector_getcamframe_cambasecamera_ptr_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026690_camgameplaydirector_islookingbehind_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026691_cpedintelligence_findtasksecondarybytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_026703_camgameplaydirector_getfirstpersonvehiclecamera_centity_const_pt(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026713_camgameplaydirector_getfollowpedcamera_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026719_cweaponinfo_const_ptr_cweaponinfomanager_getinfo_cweaponinfo_rag(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026829_ccoverpoint_dtorccoverpoint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026839_camfirstpersonshootercamera_whensprintinguserightstick_cped_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026848_camfirstpersonshootercamera_isaimingdirectly_cped_const_ref_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32, a7: f32) -> i32; }
extern "C" { pub fn f_027008_ctrain_getlinkedtobackward_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027009_ctrain_getlinkedtoforward_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027179_camreplaydirector_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027183_camreplaydirector_getcurrentmarkerindex(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_027317_camswitchdirector_getswitchcamera(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028243_ccameradebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_028267_cviewportmanager_dlcrenderphasedrawinit(ph: *mut Phantom); }
extern "C" { pub fn f_029072_rage_sqcompressstate_nextslice(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029075_stb_compress_dxt_block_unsigned_char_ptr_unsigned_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029192_tiledscreencapture_updateenabled(ph: *mut Phantom); }
extern "C" { pub fn f_029291_cdebug_update(ph: *mut Phantom); }
extern "C" { pub fn f_029325_cdebugscene_focusentities_clear(ph: *mut Phantom); }
extern "C" { pub fn f_029329_cvectormap_drawvehicle_cvehicle_const_ptr_rage_color32_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_029330_cvectormap_drawped_centity_const_ptr_rage_color32_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_029331_cdebugscene_drawentityboundingbox_centity_ptr_rage_color32_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_029377_ctask_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029386_cpedvariationinfocollection_getcollectiondrawable_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_029395_rage_phinst_setmatrix_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029397_cutscenemanager_getcamentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029433_getorcreategroup_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029452_rage_fwassetstore_rage_pgdictionary_rage_grctexture_rage_fwtxdde(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_029455_cdebugscene_tunewidgetdata_tunewidgetdata_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_029456_cdebugscene_queuetunewidget_cdebugscene_tunewidgetdata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029512_cdebugscene_getworldpositionundermouse_rage_vector3_ref_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_029607_cpedvariationinfocollection_getracetype_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_029608_cpedvariationinfocollection_ismatchcomponent_unsigned_int_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_029609_cdebugscene_getmouseleftpressed(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_029611_cdebugscene_getmousepointing_rage_vector3_ref_rage_vector3_ref_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029612_worldprobe_cshapetestdesc_setresultsstructure_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029613_worldprobe_cshapetestresults_freeintersections(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029620_ccutscenedebugmanager_islightdisabled_clightsource_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_029682_cutscenemanager_getmbframe(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029781_cdebugbar_render(ph: *mut Phantom); }
extern "C" { pub fn f_029787_cblockview_setdisplayinfo_bool(ph: *mut Phantom); }
extern "C" { pub fn f_029809_cdebugscene_getentityundermouse_rage_vector3_ptr_int_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_029817_budgetdisplay_islightdisabled_clightsource_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_029857_colorizerutils_getdrawablepolycount_rage_rmcdrawable_const_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029858_colorizerutils_getdrawableinfo_rage_rmcdrawable_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_029861_colorizerutils_getvvrdata_rage_rmcdrawable_const_ptr_float_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029864_cdebug_addbug_bool_int_bool_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_029866_cdebugscene_focusentities_get_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029879_cdebug_renderdebugscaleforminfo(ph: *mut Phantom); }
extern "C" { pub fn f_029882_cdebug_renderreleaseinfotoscreen(ph: *mut Phantom); }
extern "C" { pub fn f_029890_cdebugscene_focusentities_set_centity_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_029900_cdebugarchetype_createdebugarchetypeproxies(ph: *mut Phantom); }
extern "C" { pub fn f_029924_cdebugdrawstore_addline_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_029926_cdebugdrawstore_addvectormapline_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_029927_cdebugdrawstore_addsphere_rage_vec3v_const_ref_float_rage_color3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_029928_cdebugdrawstore_addcapsule_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_029929_cdebugdrawstore_addtext_rage_vec3v_const_ref_int_int_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_029931_cdebugdrawstore_addobb_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_029932_cdebugdrawstore_addarrow_rage_vec3v_const_ref_rage_vec3v_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_029934_cdebugdrawstore_render(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029951_cuigadgettext_setstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029965_rage_fwtransform_getb_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029967_cphysical_gethealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029969_cdebugscene_focusentities_isingroup_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029973_cdebugscene_getmouseleftreleased(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_029994_cframedump_startcapture_char_const_ptr_bool_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: f32, a9: i64); }
extern "C" { pub fn f_029995_cframedump_stopcapture(ph: *mut Phantom); }
extern "C" { pub fn f_030064_char_const_ptr_assetanalysis_getrpfpath_t_rage_fwmaptypesstore_r(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_030081_lightprobe_isenabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_030086_capturescenecubemapface_rage_grcimage_ptr_ref_rage_grcimage_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_030145_tiledscreencapture_updatelayer_tiledscreencapture_ctsclayer_cons(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030146_tiledscreencapture_isenabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_030147_tiledscreencapture_isenabledorthographic(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_030148_tiledscreencapture_gettilebounds_rage_spdaabb_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030149_tiledscreencapture_getssaoscale(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_030150_tiledscreencapture_getwaterreflectiontexture(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030152_tiledscreencapture_setstate_int_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_030153_tiledscreencapture_tilegetfilename_int_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_030154_tiledscreencapture_updatecamera(ph: *mut Phantom); }
extern "C" { pub fn f_030157_void_rage_dlc_addinternal_void_ptr_int_int_int_int_int_int_int_i(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030184_assetanalysis_gettexturepath_rage_fwarchetype_const_ptr_rage_grc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_030247_cdebugvolume_addquad_rage_vector3_const_ref_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: f32, a10: f32, a11: i32, a12: f32); }
extern "C" { pub fn f_030248_cdebugcameralock_getviewtoworld_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030249_cdebugcameralock_gettanhfov_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030250_cdebugcameralock_gettanvfov_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030251_ckdop18face_ckdop18face_rage_vector4_const_ref_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i32) -> i64; }
extern "C" { pub fn f_030252_ckdop18_addface_ckdop18face_ptr_int_ref_ckdop18face_const_ref_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_030260_debuglights_addlighttoguidmap_clightentity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030309_rage_atarray_centityadd_0_unsigned_short_op_rage_atarray_centity(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030351_debugrenderingmgr_init(ph: *mut Phantom); }
extern "C" { pub fn f_030354_debugrendering_rendercost_rage_grcrendertarget_ptr_int_float_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_030355_clightsource_ispointinlight_rage_vector3_const_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_030357_debugdeferred_addwidgets_gbuffer_rage_bkbank_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030363_debugdeferred_stencilmaskoverlayselectedbegin_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_030364_debugdeferred_stencilmaskoverlayselectedend_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_030366_rage_grcstateblock_grcdepthstencilstatehandleenum_ref_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030376_debuglights_setupdeflight_clightsource_ptr_rage_grctexture_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030385_debuglights_getisolatedlightshaftentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030399_debuglighting_dumpparentimaps_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_030409_rage_color32_setfromrgba_rage_vec4v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030411_debuglighting_overridelodlight_clightsource_ref_unsigned_int_elo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_030412_debuglighting_overrideambient_rage_vector4_ptr_eambientoverridet(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030413_debuglighting_overridedirectional_rage_vector4_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030468_istextureusedbymodel_rage_grctexture_const_ptr_char_const_ptr_cb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_030482_stringmatch_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_030506_cassetref_removerefwithoutdelete_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030528_getfriendlytexturename_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030531_cdtvstate_render_rage_grctexture_const_ptr_int_rage_fwbox2d_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_030533_cdtvstate_dumptexture_rage_grctexture_const_ptr_rage_grctexture(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_030720_rage_atpagedpooliterator_clightentity_rage_fwdefaultrscmempagedp(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030721_rage_atpagedpoolsubtypeiterator_clightentity_rage_fwdefaultrscme(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030722_ceventgroupped_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030723_cevent_affectsped_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_030729_ceventgroupped_hasscriptcommandoftasktype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_030739_cpedintelligence_addevent_cevent_const_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030740_geteventscriptaigroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030741_geteventglobalgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030744_ceventnetwork_spewtotty_impl_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030746_geteventscriptnetworkgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030756_ceventhandlerhistory_getcurrentevent_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030759_ceventresponse_cleareventresponse_ceventresponse_responsetasktyp(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030761_cpedintelligence_gettaskactive_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030763_ceventhandler_getcurrenteventtype_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030764_ceventhandler_flushimmediately(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030769_caitarget_dtorcaitarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030770_cshockingeventsmanager_add_ceventshocking_ref_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_030771_ceventshocking_initclone_ceventshocking_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_030772_ceventshocking_dtorceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030773_cshockingeventsmanager_clearallevents_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_030779_ceventdecisionmaker_handleseventoftype_int_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030782_ceventdecisionmakermanager_getisdecisionmakervalid_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_030783_ceventdecisionmakermanager_getdecisionmaker_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_030801_cevent_cevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030814_cpeddamagecalculator_cpeddamagecalculator_centity_const_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_030815_cpeddamagecalculator_applydamageandcomputeresponse_cped_ptr_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32); }
extern "C" { pub fn f_030819_cped_isinjured_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030824_cpedvariationinfocollection_hascomponentflagsset_epedvarcomp_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_030826_cpedintelligence_gettargetting_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030828_ceventdamage_ceventdamage_centity_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_030829_ceventdamage_dtorceventdamage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030834_ceventdamage_geteventpriority_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030842_ceventranoverped_ceventranoverped_cvehicle_ptr_cped_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_030845_ceventagitated_ceventagitated_cped_ptr_agitatedtype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_030846_ceventagitated_dtorceventagitated(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030848_ceventdeath_ceventdeath_bool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_030851_ceventdeath_dtorceventdeath(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030854_ceventwrithe_ceventwrithe_cweapontarget_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_030855_ceventwrithe_dtorceventwrithe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030858_ceventhurttransition_ceventhurttransition_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030859_ceventhurttransition_dtorceventhurttransition(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030881_ceventclimbnavmeshonroute_ceventclimbnavmeshonroute_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: f32, a5: i32, a6: i64, a7: i32, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_030886_ceventpotentialbewalkedinto_ceventpotentialbewalkedinto_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_030887_ceventpotentialbewalkedinto_dtorceventpotentialbewalkedinto(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030893_ceventpotentialgetrunover_ceventpotentialgetrunover_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030902_ceventpotentialblast_ceventpotentialblast_caitarget_const_ref_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i64; }
extern "C" { pub fn f_030907_ceventpotentialwalkintovehicle_ceventpotentialwalkintovehicle_cv(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64) -> i64; }
extern "C" { pub fn f_030908_ceventpotentialwalkintovehicle_dtorceventpotentialwalkintovehicl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030913_ceventnetwork_getname_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030945_svehiclemissionparams_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030986_ceventopendoor_ceventopendoor_cdoor_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030987_ceventopendoor_dtorceventopendoor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030990_ceventshoveped_ceventshoveped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030991_ceventshoveped_dtorceventshoveped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030995_ceventvehiclecollision_ceventvehiclecollision_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32, a6: f32) -> i64; }
extern "C" { pub fn f_030996_ceventvehiclecollision_dtorceventvehiclecollision(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031002_ceventpedcollisionwithped_ceventpedcollisionwithped_unsigned_sho(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i64, a5: i64, a6: f32, a7: f32) -> i64; }
extern "C" { pub fn f_031004_ceventpedcollisionwithped_dtorceventpedcollisionwithped_53479(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031011_ceventobjectcollision_ceventobjectcollision_unsigned_short_cobje(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: f32) -> i64; }
extern "C" { pub fn f_031012_ceventobjectcollision_dtorceventobjectcollision(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031035_ceventpedenteredmyvehicle_ceventpedenteredmyvehicle_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031036_ceventpedenteredmyvehicle_dtorceventpedenteredmyvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031040_ceventacquaintanceped_ceventacquaintanceped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031051_ceventencroachingped_ceventencroachingped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031052_ceventencroachingped_dtorceventencroachingped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031055_ceventscriptcommand_ceventscriptcommand_int_rage_aitask_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_031069_ceventgivepedtask_ceventgivepedtask_int_rage_aitask_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_031070_ceventgivepedtask_dtorceventgivepedtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031086_ceventinair_ceventinair_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031087_ceventinair_dtorceventinair(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031090_ceventmustleaveboat_ceventmustleaveboat_cboat_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031091_ceventmustleaveboat_dtorceventmustleaveboat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031120_ceventhelpambientfriend_ceventhelpambientfriend_cped_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_031129_ceventvehicleonfire_ceventvehicleonfire_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031130_ceventvehicleonfire_dtorceventvehicleonfire(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031136_ceventstuckinair_ceventstuckinair_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031137_ceventstuckinair_dtorceventstuckinair(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031153_ceventcarundriveable_ceventcarundriveable_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031158_ceventfirenearby_dtorceventfirenearby(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031160_ceventpedoncarroof_ceventpedoncarroof_cped_ptr_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031161_ceventpedoncarroof_dtorceventpedoncarroof(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031169_ceventswitch2nm_ceventswitch2nm_unsigned_int_rage_aitask_ptr_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_031170_ceventswitch2nm_dtorceventswitch2nm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031187_ceventunidentifiedped_ceventunidentifiedped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031188_ceventunidentifiedped_dtorceventunidentifiedped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031210_ceventcombattaunt_ceventcombattaunt_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031211_ceventcombattaunt_dtorceventcombattaunt(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031228_cevententitydestroyed_cevententitydestroyed_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031230_ceventpedseendeadped_ceventpedseendeadped_cped_ref_cped_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031235_ceventshocking_ceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031242_ceventshocking_set_rage_vec3v_const_ref_centity_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_031378_ceventgunaimedat_ceventgunaimedat_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031380_ceventgunaimedat_dtorceventgunaimedat_53949(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031386_ceventgunaimedat_isaimingintargetseeingrange_cped_const_ref_cped(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_031389_ceventgunshot_ceventgunshot_centity_ptr_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32) -> i64; }
extern "C" { pub fn f_031400_svehiclemissionparams_settargetentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_031415_ceventmeleeaction_ceventmeleeaction_centity_ptr_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_031416_ceventmeleeaction_dtorceventmeleeaction(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031424_ceventfriendlyaimedat_ceventfriendlyaimedat_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031606_ceventacquaintanceped_dtorceventacquaintanceped_54301(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031617_ceventhelpambientfriend_dtorceventhelpambientfriend(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031689_ceventpotentialblast_dtorceventpotentialblast(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031702_cevententitydestroyed_dtorcevententitydestroyed(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031706_ceventpedseendeadped_dtorceventpedseendeadped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033371_gfxdrawtextimpl_gfxdrawtextimpl_gfxdrawtextmanager_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033411_gfxdrawtextmanager_begindisplay_gviewport_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033412_gfxdrawtextmanager_enddisplay(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033790_gfxvalue_objectinterface_objectrelease_gfxvalue_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034143_gfxvalue_objectinterface_hasmember_void_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_034144_gfxvalue_objectinterface_getmember_void_ptr_char_const_ptr_gfxva(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_034146_gfxvalue_objectinterface_invoke_void_ptr_gfxvalue_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_034150_gfxvalue_objectinterface_getdisplayinfo_void_ptr_gfxvalue_displa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_034151_gfxvalue_objectinterface_setdisplayinfo_void_ptr_gfxvalue_displa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034154_gfxvalue_objectinterface_gotoandplay_void_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_034155_gfxvalue_objectinterface_setcxform_void_ptr_grenderer_cxform_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034156_gfxvalue_objectinterface_createemptymovieclip_void_ptr_gfxvalue(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_034157_gfxvalue_objectinterface_attachmovie_void_ptr_gfxvalue_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_034865_rage_sfscaleformmovieview_drawinworldspace_rage_sfscaleformmanag(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034868_rage_sfscaleformmanager_autosetcurrmoviename_autosetcurrmovienam(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034880_rage_sfscaleformmanager_beginframe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034881_rage_sfscaleformmanager_endframe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034946_cloadingscreens_areactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034949_chudtools_getwidescreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034950_chudtools_getaspectratio_bool(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_034954_chudtools_adjustnormalized16_9valuesforcurrentaspectratio_ewides(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_034956_chudtools_getminsafezone_float_ref_float_ref_float_ref_float_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32); }
extern "C" { pub fn f_034958_chudtools_adjustforwidescreen_ewidescreen_format_rage_vector2_pt(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_034965_clandingpagearbiter_islandingpageactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034967_chudtools_getuiwidth(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_034968_cmousepointer_update(ph: *mut Phantom); }
extern "C" { pub fn f_034970_cmousepointer_resetmouseinput(ph: *mut Phantom); }
extern "C" { pub fn f_035026_cnewhud_activatehudcomponent_bool_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_035029_chudtools_calculatehudposition_rage_vector2_rage_vector2_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_035035_cnewhud_removehudcomponent_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035036_cnewhud_removehudcomponentfromactionscript_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035049_cnewhud_canrenderhud(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035053_cnewhud_shoulddrawblinders(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035054_cminimap_getminimapped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_035059_cpedinventory_getweaponammo_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_035061_cnewhud_issnipersightactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035062_cminimap_getisinsideinterior_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035079_chudcolour_getrgba_ehud_colours(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035082_cnewhud_isweaponwheelactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035083_cnewhud_isradiowheelactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035109_chudtools_getaspectratiomultiplier_bool(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_035112_chudcolour_getcolourfromrgba_rage_color32(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035121_cphonemgr_render_cphonemgr_edrawphonemode(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035184_cmultiplayergamertaghud_dtorcmultiplayergamertaghud(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035194_cmultiplayergamertaghud_ismovieactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035257_cminimap_getcurrentminimapposition(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035258_cminimap_getcurrentminimap(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035263_cminimap_creategenericblip_bool_eblip_type_centitypoolindexforbl(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_035267_cminimap_createblip_bool_eblip_type_centitypoolindexforblip_cons(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_035272_cminimap_removeblip_cminimapblip_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035273_cminimap_destroyblipobject_cminimapblip_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035278_cminimap_updatebitmaponut_mm_bitmap_version(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035279_cminimap_getcentreposition_rage_vector3_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035282_char_ptr_rage_formatf_char_50ul_char_ref_50ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_035284_cminimap_updatestatesonut(ph: *mut Phantom); }
extern "C" { pub fn f_035285_cminimap_updateblips(ph: *mut Phantom); }
extern "C" { pub fn f_035287_cminimap_shouldprocessminimap(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035294_cminimap_setminimapmodestate_esetup_state_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_035304_cminimap_switchoffwaypoint_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035308_cminimap_findblipentity_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035317_cminimap_setblipparameter_eblip_params_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_035320_cminimap_setblipparameter_eblip_params_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_035324_cminimap_getblip_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_035327_cminimap_removeblipattachedtoentity_centity_const_ptr_eblip_type(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035335_cminimap_renderthread_handleinterioratendofframe(ph: *mut Phantom); }
extern "C" { pub fn f_035337_cminimap_renderthread_handlegolfcourseatendofframe(ph: *mut Phantom); }
extern "C" { pub fn f_035338_cminimap_renderthread_removeconefrombliponstage_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035344_cminimap_updatecentreandnorthblips(ph: *mut Phantom); }
extern "C" { pub fn f_035345_cminimap_updatewaypoint(ph: *mut Phantom); }
extern "C" { pub fn f_035354_cminimap_switchonwaypoint_float_float_unsigned_short_bool_bool_e(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_035370_cblipupdatequeue_updateblips(ph: *mut Phantom); }
extern "C" { pub fn f_035376_cshaderlib_lookuptexture_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035402_csupertile_processatendofframe_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035403_char_ptr_rage_formatf_char_20ul_char_ref_20ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_035404_cminimap_renderthread_setupminimapproperties_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035405_char_ptr_rage_formatf_char_90ul_char_ref_90ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_035406_cminimap_renderthread_updatewithactionscriptonrt_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035407_cminimap_renderthread_updatetiles_bool(ph: *mut Phantom); }
extern "C" { pub fn f_035408_cminimap_renderthread_setupforpausemenu_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035412_drawcone_gfxvalue_ref_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_035443_chudmarkerrenderer_shutdownscaleformut(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035447_void_std_2_stable_sort_std_2_classicalgpolicy_chudmarkermanager(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_035448_chudmarkermanager_updatemarkerfromblip_int_shudmarkerstate_ref_c(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_035449_chudmarkermanager_createmarker_shudmarkerstate_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_035450_void_std_2_stable_sort_std_2_classicalgpolicy_chudmarkermanager(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_035491_cbusyspinner_render(ph: *mut Phantom); }
extern "C" { pub fn f_035492_cbusyspinner_canrender(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036033_cscaleformmgr_rendermovie_int_float_bool_bool_float(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i32, a4: f32); }
extern "C" { pub fn f_036034_cscaleformmoviewrapper_isactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036036_cscaleformmgr_flushinvokelist(ph: *mut Phantom); }
extern "C" { pub fn f_036038_cscaleformmgr_ismovieactive_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036044_cscaleformmgr_beginmethodinternal_int_escaleform_base_class_char(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_036045_cscaleformmgr_endmethodinternal_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036048_cscaleformmgr_beginmethod_int_escaleform_base_class_char_const_p(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_036049_cscaleformmgr_addparamfloat_double(ph: *mut Phantom, a0: f64); }
extern "C" { pub fn f_036050_cscaleformmgr_endmethod_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036051_cscaleformmgr_addparambool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036053_cscaleformmgr_addparamint_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036056_cscaleformmgr_addparamstring_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036058_cscaleformmoviewrapper_createmovie_escaleform_base_class_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_036254_cpausemenu_isactive_pm_activeflags(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036292_suicontexts_isactive_rage_athashvalue(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036324_cpausemenu_setrenderdata_pausemenurenderdataextra_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036330_cpausemenu_renderanimatedspinner_rage_vector2_rage_vector2_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_036372_cscaleformmgr_changemovieparams_int_rage_vector2_const_ref_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_036377_cpausemenu_isinmapscreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036459_cwarningscreen_render(ph: *mut Phantom); }
extern "C" { pub fn f_036639_cscaleformmgr_updateatendofframe_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036642_cscaleformcomplexobjectmgr_syncbuffers(ph: *mut Phantom); }
extern "C" { pub fn f_036648_cuireticule_transformreticuleposition_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_036657_uiworldicon_addtodrawlist_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036950_ctexttemplate_updatetemplate_int_seditedtextproperties_rage_atno(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_036953_ctexttemplate_unloadtemplate(ph: *mut Phantom); }
extern "C" { pub fn f_036988_ctexttemplate_setuptemplate_seditedtextproperties(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037236_ctexttemplate_rendersnapmatic(ph: *mut Phantom); }
extern "C" { pub fn f_037287_cscaleformmgr_prepareupdateallmoviesonrt(ph: *mut Phantom); }
extern "C" { pub fn f_037288_cscaleformmgr_ensureallmoviesupdatedonrt(ph: *mut Phantom); }
extern "C" { pub fn f_037289_cscaleformmgr_cleanupmoviesonrt(ph: *mut Phantom); }
extern "C" { pub fn f_037316_cscaleformmgr_getactionscriptobjectfromroot_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037317_cscaleformmgr_callmethod_int_escaleform_base_class_char_const_pt(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_037324_cscaleformmgr_lockmovie_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037325_cscaleformmgr_unlockmovie_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037328_cscaleformmgr_renderworldspacediffusepass(ph: *mut Phantom); }
extern "C" { pub fn f_037365_cscaleformcomplexobjectmgr_performalloutstandingactionsonrt(ph: *mut Phantom); }
extern "C" { pub fn f_037366_cscaleformcomplexobjectmgr_releaseanyflaggedobjects(ph: *mut Phantom); }
extern "C" { pub fn f_037438_cfrontendstatsmgr_shouldusemetric(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037511_cgamestream_postticker_char_const_ptr_bool_bool_bool_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_037541_cgamestreammgr_update(ph: *mut Phantom); }
extern "C" { pub fn f_037542_cgamestreammgr_render(ph: *mut Phantom); }
extern "C" { pub fn f_037581_cgolftrailinterface_updateandrender_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037637_cwanted_cheatwantedlevel_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037660_cmodelinfo_getassetstreamflags_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037661_cmodelinfo_requestassets_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037662_cmodelinfo_haveassetsloaded_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037663_cmodelinfo_setassetsaredeletable_rage_fwmodelid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037801_cthepopmultiplierareas_gettrafficdensitymultiplier_rage_vector3(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_037802_cthepopmultiplierareas_getpeddensitymultiplier_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_037813_cplacename_isnewname(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037880_cwanted_initialise(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037884_cwanted_reportpolicespottingplayer_cped_ptr_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_037888_cseatmanager_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_037901_cwanted_copsaresearching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037934_cweather_gettypeindex_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037967_cweather_gettypeindex_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_038130_cmodelinfo_getbasemodelinfo_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038181_corder_getincident_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038189_cdispatchmanager_getnetworkreservations_unsigned_int_ref_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_038222_corder_getisvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038382_cpedintelligence_addtaskdefault_rage_aitask_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038463_cbaseikmanager_removeallsolvers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038467_cbaseikmanager_getsolver_ikmanagersolvertypes_flags_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038492_crootslopefixupiksolver_isactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038527_cikmanager_getdefaultlegikmode_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038619_cbasemodelinfo_cleardynamiccovercollisionbounds_rage_pharchetype(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038621_cbasemodelinfo_computemass_rage_pharchetypedamp_ptr_ctunableobje(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_038622_cbasemodelinfo_computedamping_rage_pharchetypedamp_ptr_ctunableo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_038630_cbasemodelinfo_getnum2deffectlights_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038631_cbasemodelinfo_getnum2deffects_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038633_cbasemodelinfo_get2deffect_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038650_cmlomodelinfo_getroomflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038651_cmlomodelinfo_getportalflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038652_cmlomodelinfo_getportaldata_unsigned_int_unsigned_int_ref_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_038699_cmodelinfo_getmodelinfofromlocalindex_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_038711_cmodelinfo_addbasemodel_char_const_ptr_unsigned_int_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038714_cmodelinfo_areassetsrequested_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038715_cmodelinfo_requestassets_cbasemodelinfo_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038716_cmodelinfo_getassetsaredeletable_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038718_cmodelinfo_getassetrequiredflag_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038720_cmodelinfo_clearassetrequiredflag_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038721_cmodelinfo_getobjectanddependencies_rage_fwmodelid_rage_atarray(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_038722_cmodelinfo_getobjectanddependenciessizes_rage_fwmodelid_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_038733_centryexitpoint_getseat_seataccess_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_038734_centryexitpoint_getentrypointposition_centity_const_ptr_cped_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_038735_cmodelseatinfo_calculateentrysituation_centity_const_ptr_cped_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32, a7: i64); }
extern "C" { pub fn f_038737_cseataccessor_getseatbyindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038740_cmodelseatinfo_getboneindexfromseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038741_cmodelseatinfo_getboneindexfromentrypoint_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038744_cmodelseatinfo_calculateseatsituation_centity_const_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_038745_cmodelseatinfo_calculateexitsituation_centity_ptr_rage_vector3_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_038746_cmodelseatinfo_calculatethroughwindowposition_cvehicle_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_038747_cmodelseatinfo_evaluateseataccessfromallentrypoints_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64, a7: i32) -> i32; }
extern "C" { pub fn f_038751_cseatmanager_getpedsseatindex_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038755_cpedmodelinfo_getlayoutinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038756_cseatmanager_getlastdriver_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038837_cpedmodelinfo_findpersonalitymovementmodes_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_038863_cpedmodelinfo_getpersonalitybravery_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_038873_cpedmodelinfo_getarehdfilesloaded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038874_cpedmodelinfo_hashelmetprop_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038875_cpedmodelinfo_getpedinstance_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038992_cvehiclemodelinfo_gethdfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039013_cvehiclemodelinfo_getseatanimationinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039024_cvehiclemodelinfo_getrandomdriver_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039025_cvehiclemodelinfo_getdistancesqrtoclosestinstance_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> f32; }
extern "C" { pub fn f_039218_cvehiclevariationinstance_getmodboneindex_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039222_cvehiclevariationinstance_getbonetoturnoffforslot_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_039223_cvehiclevariationinstance_getnumbonestoturnoffforslot_unsigned_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039246_cvehiclestreamrendergfx_getfragindex_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039248_cvehiclestreamrendergfx_getfrag_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039249_cvehiclestreamrendergfx_getdrawable_evehiclemodtype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039252_cvehiclestreamrendergfx_addrefstogfx(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039256_cvehiclestreamrendergfx_getdrawablecsetype_evehiclemodtype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039322_rage_atfixedarray_rage_strrequest_12_dtoratfixedarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039561_cobject_cobject_eentityownedby_unsigned_int_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_039565_cobject_dtorcobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039569_cdoor_getdooraudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039573_cpathservergta_updatedynamicobject_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_039576_cdoor_getdooropenratio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_039581_rage_phcollider_setvelocity_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_039593_cobject_objecthasbeenmoved_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_039623_cobject_onactivate_rage_phinst_ptr_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_039630_cweapon_sfireparams_dtorsfireparams(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039635_cdoor_getdoorocclusion_float_float_float_float_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_039657_cdoorscripthandlerobject_createscriptinfo_rage_scriptidbase_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_039688_cobjectpopulation_destroyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039689_cobject_dumpobjectpool(ph: *mut Phantom); }
extern "C" { pub fn f_039707_cobject_createdrawable(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039713_cobject_fix(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039715_cobject_setweapon_cweapon_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039717_cobject_setfragparent_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039725_cobject_prerender_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039726_cobject_prerender2_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039730_cobject_canbetargetted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039731_cobject_teleport_rage_vector3_const_ref_float_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_039733_cobject_placeongroundproperly_float_bool_float_bool_bool_bool_pt(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: f32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_039744_cobject_scaleobject_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_039757_cobjectpopulation_converttodummyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039761_cobjectpopulation_createobject_rage_fwmodelid_eentityownedby_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_039763_cnavmeshdataexporter_willexportcollision(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_039799_cproceduralinfo_init(ph: *mut Phantom); }
extern "C" { pub fn f_039800_cproceduralinfo_createsprocobjects_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_039801_cproceduralinfo_createsplants_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_039804_cprocobjectman_init(ph: *mut Phantom); }
extern "C" { pub fn f_039805_cprocobjectman_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_039810_cprocobjectman_addobjecttoaddlist_rage_vector3_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_039811_cprocobjectman_addtritoremovelist_cplantloctri_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039883_rage_cnavmeshtrackedobject_isuptodate_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_039917_cpathserver_deferreddisableclimbatposition_rage_vector3_const_re(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039918_cpathserver_disableclimbatposition_rage_vector3_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039987_rage_cnavmeshtrackedobject_teleport_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039996_cpathservergta_mayberemovedynamicobject_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040008_cpathservergta_shouldpedavoidobject_cped_const_ptr_centity_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040041_cpathserverambientpedgen_arecachedpedgencoordsready_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_040042_cpathserverambientpedgen_getcachedpedgencoords_rage_cpedgencoord(ph: *mut Phantom); }
extern "C" { pub fn f_040043_cpathserverambientpedgen_isinpedgenblockedarea_rage_vector3_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040047_cpathserver_forceperformthreadhousekeeping_bool(ph: *mut Phantom); }
extern "C" { pub fn f_040141_cpedformation_setspacing_float_float_float_rage_scrthreadid(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_040142_cpedgroup_islocallycontrolled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040150_cpedgroupmembership_countmembersexcludingleader_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040157_cpedgroupmembership_countmembers_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040165_cpedgroupmembership_flush_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_040169_cpedgroupmembership_removemember_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040170_cpedgroupmembership_setmaxseparation_float_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_040171_cpedgroupmembership_setleader_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040172_cpedgroupmembership_addfollower_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040173_cpedgroupmembership_isfollower_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040175_cpedgroupmembership_isleader_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040180_cpedgroup_setformation_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_040182_cpedgroup_process(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040183_cpedgroup_flush_bool_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040189_cpedgroups_addgroup_epoptype_cgamescripthandler_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_040190_cpedgroups_addgroupatindex_epoptype_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_040194_cpedgroups_getpedsgroup_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040195_cpedgroups_getplayersgroup_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040196_cpedgroups_areinsamegroup_cped_const_ptr_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040201_carcadepassiveabilityeffects_getisactive_earcadeabilityeffecttyp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_040205_carea_dtorcarea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040209_carea_ispointinarea_rage_vector3_const_ref_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_040210_carea_set_rage_vector3_const_ref_rage_vector3_const_ref_float_ce(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32); }
extern "C" { pub fn f_040211_carea_setassphere_rage_vector3_const_ref_float_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_040212_carea_getcentreandmaxradius_rage_vector3_ref_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040402_cdefensiveareamanager_getsecondarydefensivearea_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_040404_rage_fraginstnm_postartmessage_char_const_ptr_art_messageparamsb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040405_rage_fraginstnm_setcomponenttmsfromskeleton_rage_crskeleton_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040406_rage_fraginstnm_configurecharacter_int_bool_int_int_float_float(ph: *mut Phantom, a0: i32, a1: f32, a2: f32); }
extern "C" { pub fn f_040409_art_messageparamsbase_addbool_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040410_art_messageparamsbase_addfloat_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_040411_rage_fraginstnm_precomputeimpacts_rage_phcontactiterator(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040413_rage_fraginstnm_prepareforactivation_rage_phcollider_ptr_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_040414_rage_fragnmassetmanager_getagentcount_unsigned_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_040417_rage_fraginstnm_preparefordeactivation_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_040419_art_nmrsengine_destroycharacter_art_nmrscharacter_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040420_art_cbutaskbase_deactivate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040421_art_nmrscharacter_removefromscene(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040424_rage_fraginstnm_mapragdolllodcomponentcurrenttohigh_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_040432_rage_fragnmassetmanager_load_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040433_rage_fragnmassetmanager_setincominganimvelocityscale_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_040434_rage_fragnmassetmanager_getnumactivenonnmragdolls(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_040435_rage_fragnmassetmanager_aredistributedtasksenabled_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_040436_rage_fragnmassetmanager_setfromanimationaveragevel_rage_vector3(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_040503_art_messageparamsbase_messageparamsbase_art_messageparamsbase_pa(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_040603_art_nmrsspineinputwrapper_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041047_cpedcomponentsetmanager_getinfo_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_041053_cpeddebugvisualiser_visualiseped_cped_const_ref_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041054_cpeddebugvisualiser_visualiseanimation_cdynamicentity_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041055_cpeddebugvisualiser_visualiselookabout_centity_const_ref_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041056_findplayerped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041057_cpeddebugvisualiser_visualisetasks_cped_const_ref_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041058_cpeddebugvisualiser_visualisedamage_cped_const_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041059_cpedperception_drawvisualfield(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041060_cpeddebugvisualisermenu_getfocusped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041061_getcoorsandalphafordebugtext_centity_const_ref_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_041062_cpedintelligence_findmovementtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041063_cpedintelligence_isfriendlywith_cped_const_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041064_cpedintelligence_isthreatenedby_cped_const_ref_bool_bool_bool_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_041065_cplayerpedtargeting_tunables_gettargettinginfo_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041066_cpedtargetevaluator_canbetargetedinrejectiondistance_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_041067_cplayerpedtargeting_getisfreeaimassisttargetposwithinreticuleslo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_041068_cplayerpedtargeting_getaimdistancescalecurve(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041069_cplayerpedtargeting_getcurveset_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_041070_ccurveset_render_rage_vector2_const_ref_rage_vector2_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_041071_cpedintelligence_getclosestvehicleinrange_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041072_centityboundai_computecorners_centity_const_ref_float_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32, a7: i64, a8: f32); }
extern "C" { pub fn f_041075_cpedtargetting_cpedtargetting_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041078_cpedtargetting_updatesinglelos_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041079_centityboundai_computehitsidebyposition_rage_vector3_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041080_centityboundai_computeboundingboxplanes_rage_vector3_ptr_float_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_041088_cpedintelligence_getnearbyvehicles(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041090_cpedperception_calculateviewmatrix_rage_matrix34_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041091_cpedperception_computefovvisibility_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32) -> i32; }
extern "C" { pub fn f_041092_cpedperception_computefovvisibility_centity_const_ptr_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_041111_cpeddebugvisualisermenu_updateblendfromnmsetswidgets(ph: *mut Phantom); }
extern "C" { pub fn f_041113_cpedintelligence_getlowestlevelnmtask_cped_ptr_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041115_cpedpopulation_removeallpedswithexception_cped_ptr_cpedpopulatio(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041116_cpedintelligence_flushimmediately_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041117_cpedintelligence_addtoeventhistory_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041118_cpedpopulation_removeallpedshardnotplayer(ph: *mut Phantom); }
extern "C" { pub fn f_041119_cpedtargetting_dtorcpedtargetting(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041120_cpedscriptedtaskrecord_process(ph: *mut Phantom); }
extern "C" { pub fn f_041121_cpedintelligence_buildqueriablestate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041122_svehiclemissionparams_dtorsvehiclemissionparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041126_cpeddebugvisualisermenu_warptostartcb(ph: *mut Phantom); }
extern "C" { pub fn f_041128_cpeddebugvisualisermenu_testharnesscb(ph: *mut Phantom); }
extern "C" { pub fn f_041129_rage_quaternion_fromeulers_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041130_cpedintelligence_haseventoftype_cevent_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041194_cpeddebugvisualisermenu_changefocus_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041195_cpedintelligence_cleartasks_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041196_cpeddebugvisualisermenu_addorremovepedfromgroup_cped_ptr_cped_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041197_cpedintelligence_setdefaultrelationshipgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041198_cpeddebugvisualisermenu_switchpedtonmbehaviour_ctask_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041199_cplayerpedtargeting_addtargetableentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041200_cpedintelligence_cleartaskeventresponse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041202_cpedintelligence_setrelationshipgroup_crelationshipgroup_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041204_cpedpopulation_generatepedsfromscenariopointlist_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_041208_cvehiclepotentialcollisionscanner_scan_cped_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041209_centityboundai_centityboundai_centity_const_ref_float_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32, a7: i64, a8: f32) -> i64; }
extern "C" { pub fn f_041210_cpedintelligence_getmoveblendratiofromgototask_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041212_cpedacquaintancescanner_shouldscanthisped_cped_const_ptr_cped_re(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041213_cpedacquaintancescanner_addpedtoscan_cped_ref_cped_ptr_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_041214_cpedacquaintancescanner_clearscanrequest_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041215_cpedgeometryanalyser_canpedtargetped_cped_const_ref_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_041216_cpedacquaintancescanner_gethighestacquintancetypebetweenpeds_cpe(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041217_cpedeventdecisionmaker_makedecision_cped_const_ptr_ceventeditabl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_041218_cplayerinfo_getphysicalplayerindex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041219_cpedgeometryanalyser_canpedtargetpedasync_cped_ref_cped_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: i32, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_041220_ccantargetcacheentry_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041221_cpedintelligence_canattackped_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041222_cpedtargetting_registerthreat_centity_const_ptr_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041224_ccantargetcache_getcachedresult_cped_const_ptr_cped_const_ptr_cc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i64, a6: i64, a7: i32) -> i32; }
extern "C" { pub fn f_041225_cpedgeometryanalyser_getmodifiedtargetposition_cped_const_ref_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64); }
extern "C" { pub fn f_041226_rage_syslinkeddata_unsigned_int_refreportplayerresetmismatch_uns(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041227_cpedgeometryanalyser_canpedtargetpoint_cped_const_ref_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i64, a7: i64, a8: i64) -> i32; }
extern "C" { pub fn f_041229_cpedagitationscanner_deactivatetrigger_cpedagitationscanner_trig(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041230_cpedagitationscanner_updateagitation_cped_ref_cped_ref_epedconfi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_041231_cpedagitationscanner_updatetrigger_cped_ref_cped_ref_cpedagitati(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_041232_cpedagitationscanner_areflagsvalid_cped_const_ref_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_041233_cpedagitationscanner_findfirstactivetriggerwithtype_agitatedtype(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041234_cnearbyfirescanner_scan_cped_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041236_ccollisioneventscanner_computedamagefromvehicle_cped_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64, a5: i32, a6: f32) -> f32; }
extern "C" { pub fn f_041237_cpedintelligence_gettaskmelee_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041238_cpedintelligence_gettaskwrithe_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041239_cpedintelligence_gettaskcombat_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041240_cpedintelligence_gettaskgun_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041242_ccollisioneventscanner_processbumps_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041243_ccollisioneventscanner_calculateragdollimpactdamageinstigator_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_041244_cplayerspecialabilitymanager_chargeevent_abilitychargeeventtype(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_041245_ceventdamage_setphysicalresponsetask_rage_aitask_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041246_cplayerspecialability_triggernearvehiclemiss(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041247_cplayerspecialability_addtometercontinuous_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_041248_cplayerspecialability_addtometernormalized_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_041249_cplayerspecialability_triggerlowhealth(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041250_ceventscanner_dtorceventscanner(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041253_cphysical_setattachflag_rage_eattachflags_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041254_cinwatereventscanner_isdrowninginvehicle_cped_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041255_cinwatereventscanner_candrown_cped_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041257_cped_setplayerresetflag_cplayerresetflags_playerresetflags(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041258_cpedintelligence_clearprimarytask(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041259_ceventscanner_checkfortranquilizerdamage_cped_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041260_cplayerinfo_issoftaiming_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041261_cplayerinfo_ishardaiming(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041262_cpedgeometryanalyser_isinair_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041263_rage_atbinarymap_ctasknmsimple_tunables_tuning_rage_atnonfinalha(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041265_cplayerinfo_isaiming_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041271_cpoolhelpers_getpedpoolsize(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041272_cpedfactory_dtorcpedfactory(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041275_cpedfactory_destroypedinternal_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041276_cpedfactory_logcreatedped_cped_ptr_char_const_ptr_int_rage_color(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_041277_cpedfactory_concretecreateped_ccontrolledbyinfo_const_ref_unsign(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041278_cpedpopulation_updatepedcountifnotalready_cped_ptr_cpedpopulatio(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_041279_cpedfactory_logdestroyedped_cped_ptr_char_const_ptr_int_rage_col(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_041280_cpedpopulation_iscandidateinviewfrustum_rage_vector3_const_ref_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_041284_cpedfactory_copyvariation_cped_const_ptr_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041290_cplayerinfo_cplayerinfo_cped_ptr_rage_rlgamerinfo_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041291_cplayerinfo_setplayerped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041293_cplayerspecialability_deactivate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041294_cplayerpedtargeting_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041296_cpedfactory_destroyped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041298_cpedhelmetcomponent_disablehelmet_bool_rage_vector3_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_041299_cpedhelmetcomponent_restoreoldhat_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041306_cpedfootstephelper_getfootvelocity_feettags_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041308_cpedfootstephelper_getpositionforpedsound_feettags_rage_vec3v_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_041318_centityboundai_projectboundingboxontozplane_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32); }
extern "C" { pub fn f_041321_centityboundai_liesinside_rage_vector3_const_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_041324_doesrouteseghitentity_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_041326_cpedgeometryanalyser_testcapsuleagainstcompositer_rage_phinst_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i32, a7: i64, a8: i32, a9: i64) -> i64; }
extern "C" { pub fn f_041328_ccantargetcacheentry_dtorccantargetcacheentry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041332_cpedgeometryanalyser_testifcollisiontypeisvalid_cped_const_ptr_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_041333_cpedgeometryanalyser_adjustpositionforcover_cped_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_041334_cpedtargetevaluator_isdownedpedthreat_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041340_cpedhelmetcomponent_shouldusepilotflighthelmet_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041341_cpedhelmetcomponent_gethelmetpropindex_epedcompflags_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041342_cpedhelmetcomponent_gethelmettexindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041343_cpedhelmetcomponent_useoverrideinmp_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041354_cpedhelmetcomponent_addvehiclehelmetassociation_unsigned_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_041355_cpedhelmetcomponent_clearvehiclehelmetassociation_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041364_cpediksettingsinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_041371_cpediksettingsinfo_isikflagdisabled_ikmanagersolvertypes_flags_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041374_rage_fwregdref_cscenariopoint_rage_fwrefawarebasenonvirt_dtorfwr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041379_cpedtargetting_updatetargetting_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041385_cpedintelligence_findtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041387_cpedintelligence_gettaskmeleeactionresult_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041399_cpedscriptedtaskrecorddata_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041405_cpedintelligence_isfriendlywithduetoscenarios_cped_const_ref_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041422_cpedailodmanager_shoulddofullupdate_cped_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041426_cpedintelligence_isbattleaware_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041435_cpedintelligence_getcancombatroll_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041439_cpedmotivation_setmotivationprofile_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041463_scheduledscenarioped_dtorscheduledscenarioped(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041469_cpedpopulation_initvaluesfromconfig(ph: *mut Phantom); }
extern "C" { pub fn f_041479_cpedpopulation_flushpedqueues(ph: *mut Phantom); }
extern "C" { pub fn f_041480_cpedpopulation_dropscheduledscenarioped(ph: *mut Phantom); }
extern "C" { pub fn f_041481_cpedpopulation_dropscheduledambientped(ph: *mut Phantom); }
extern "C" { pub fn f_041482_cpedpopulation_dropscheduledpedinvehicle(ph: *mut Phantom); }
extern "C" { pub fn f_041483_cpedpopulation_processpeddeletionqueue_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041484_cpedpopulation_flushpedreusepool(ph: *mut Phantom); }
extern "C" { pub fn f_041486_cpedpopulation_updatepedreusepool(ph: *mut Phantom); }
extern "C" { pub fn f_041487_cpedpopulation_setscriptcampopulationoriginoverride(ph: *mut Phantom); }
extern "C" { pub fn f_041488_cpedpopulation_usetemppopcontrolspherethisframe_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_041489_cpedpopulation_getpopulationcontrolpointinfo_cpopctrl_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041490_cpedpopulation_setupparamsforpathserverpedgenbackgroundtask_cpop(ph: *mut Phantom); }
extern "C" { pub fn f_041491_cpedpopulation_addambientpedtopopulation_bool_float_float_bool_b(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_041492_cpedpopulation_spawnscheduledpedinvehicle_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041493_cpedpopulation_spawnscheduledscenarioped(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041494_cfocusentitymgr_getpoppos(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041497_cpedpopulation_shouldremoveped_cped_ptr_rage_vector3_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_041498_cpedpopulation_canpedbereused_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041499_cpedpopulation_addpedtoreusepool_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041500_cpedpopulation_canpedberemoved_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041501_void_std_2_introsort_std_2_classicalgpolicy_copdistancelistsort(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_041502_cpedpopulation_canremovepedatpedsposition_cdynamicentity_ptr_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_041503_void_std_2_introsort_std_2_classicalgpolicy_deadsdistancelistsor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_041504_cpedpopulation_pedpopulationfailuredata_registerspawnfailure_cpe(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_041505_cpedpopulation_ispedgenerationcoordcurrentlyvalid_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32, a4: i32, a5: f32, a6: i32, a7: i64, a8: i64, a9: i64, a10: i64, a11: i32) -> i32; }
extern "C" { pub fn f_041506_cpedpopulation_addped_unsigned_int_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i64, a11: i32, a12: i32, a13: i32, a14: i64, a15: i64, a16: i32, a17: i32, a18: i32) -> i64; }
extern "C" { pub fn f_041507_cpedpopulation_managespecialstreamingconditions_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041508_cpedpopulation_reschedulepedinvehicle(ph: *mut Phantom); }
extern "C" { pub fn f_041509_rage_fwregdref_cscenariopoint_rage_fwrefawarebasenonvirt_reset_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041512_cpedpopulation_setdebugoverridemultiplier_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_041521_cpedpopulation_findpedtoreuse_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041522_cpedpopulation_reusepedfrompedreusepool_cped_ptr_rage_matrix34_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_041523_cpedpopulation_removepedfromreusepool_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041524_cpedpopulation_equipped_cped_ptr_bool_bool_bool_rage_atfixedbits(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i64, a7: i64); }
extern "C" { pub fn f_041526_cpedpopulation_findspecificdrivermodelforcartouse_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041527_cpedpopulation_pickmodelfromgroupforincar_cloadedmodelgroup_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041528_cpedpopulation_onpedspawned_cped_ref_rage_fwflags_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041536_cvehicleintelligence_resetpretendoccupanteventdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041537_cpopulationgroup_isindexmember_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041538_cpopulationgroup_findvariations_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041539_cpedpopulation_choosecivilianpedmodelindex_cpedpopulation_choose(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041540_cpedpopulation_getdistancesqrtoclosestscheduledpedwithmodelid_ra(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_041543_cpedpopulation_selectpointsrealtype_cscenariopoint_const_ref_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_041544_cpedpopulation_sp_spawnvaliditycheck_general_cscenariopoint_cons(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041546_cpedpopulation_sp_spawnvaliditycheck_interior_cscenariopoint_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041547_cpedpopulation_sp_spawnvaliditycheck_position_cscenariopoint_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_041550_cpedpopulation_getscenariopointinarea_cpedpopulation_getscenario(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041554_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_cscenar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_041555_cpedpopulation_generatepedsfromscenariopointlist_cscenariopoint(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: i32, a10: i32, a11: i32, a12: i64) -> i32; }
extern "C" { pub fn f_041556_cpopgrouplist_findpedgroupfromnamehash_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_041588_cplayerspecialability_isabilityunlocked_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041589_cexplosionmanager_cexplosionargs_dtorcexplosionargs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041591_cplayerspecialability_canbeactivated_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041592_cplayerspecialability_activate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041606_cplayerspecialabilitymanager_load_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041632_cplayerinfo_splayerstatinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_041649_cplayerdamageovertime_doteffect_dtordoteffect(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041650_cplayerinfo_getplayerstatinfoforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041654_cplayerinfo_computedurationfromstat_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_041657_cplayerinfo_multiplayerreset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041658_cplayerinfo_setinitialstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041659_cplayerinfo_dtorcplayerinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041660_cplayerpedtargeting_dtorcplayerpedtargeting(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041661_rage_syslinkeddata_unsigned_int_refreportplayerresetmismatch_uns(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041666_cplayerinfo_isrunandgunning_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_041667_cplayerinfo_isassistedaiming(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041668_cplayerpedtargeting_findlockontarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041673_cplayerpedtargeting_clearfreeaim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041674_cplayerpedtargeting_clearlockontarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041675_cplayerpedtargeting_processpedevents_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041677_cplayerinfo_handlesprintenergy_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_041678_cplayerinfo_getfiringattackvalue(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041679_cplayerinfo_isfiring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041681_cplayerinfo_controlbuttonsprint_cplayerinfo_esprinttype_bool(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_041682_cpedweaponmanager_getequippedweaponhasfirstpersonscope_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041683_cplayerinfo_processcansprint_cped_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041684_cplayerinfo_processsprintcontrol_ccontrol_const_ref_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> f32; }
extern "C" { pub fn f_041685_cplayerinfo_computesprintdepletionrateforped_cped_const_ref_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> f32; }
extern "C" { pub fn f_041686_cplayerinfo_issoftfiring(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041687_cplayerinfo_canuseassistedaiming(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041692_cplayerinfo_evaluatecarposition_centity_ptr_cped_ptr_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64, a6: i64, a7: i32); }
extern "C" { pub fn f_041699_cplayerinfo_cancarryportablepickupoftype_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_041702_findfollowped_cplayerinfo_ptr(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041706_cplayerinfo_setplayercontrol_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_041718_cplayerinfo_canplayerwarpintovehicle_cped_const_ref_cvehicle_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041720_ccontrol_getvehicletargetdown_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_041722_cplayerinfo_isselectinglefttarget_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041723_cplayerinfo_isselectingrighttarget_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041741_cplayerinfo_getmpspecialability_eplayerspecialabilityslot_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041744_cvehicle_getthrottle_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041747_cpopmodelandvariations_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_041750_cpopulationgroup_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_041761_cpopgrouplist_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041762_cpopgrouplist_dtorcpopgrouplist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041765_cpopallocation_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_041770_cpopschedule_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_041780_cpopschedulelist_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041781_cpopzonedata_szone_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_041788_cpopcycle_applyzonebinddata_cpopzonedata_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041794_cpopallocation_normalizepercentages_rage_atarray_cpopgrouppercen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041796_cpopschedulelist_load_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041798_cpopschedulelist_getnamefromindex_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_041799_cpopulationgroup_getmodelindex_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041800_cpopgrouplist_loadpatch_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041801_rage_atarray_cpopulationgroup_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041802_rage_atarray_cpopmodelandvariations_0_unsigned_short_op_rage_ata(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041804_cpopcycle_loadpopgroups(ph: *mut Phantom); }
extern "C" { pub fn f_041805_cpopcycle_loadpopzonetopopschedulebindingsdatafile_pso_char_cons(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041806_cpopcycle_loadpopgroupsfile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041807_cpopcycle_refreshpopgroupdata(ph: *mut Phantom); }
extern "C" { pub fn f_041809_cpopzones_findsmallestforposition_rage_vector3_const_ptr_ezoneca(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_041810_cpopcycle_getwantedcarmultiplier(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_041814_cpopcycle_getcurrentmaxnumparkedcars(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041830_rage_atarray_cpopzone_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041831_rage_atarray_cpopzonesearchinfo_0_unsigned_short_grow_unsigned_i(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041840_cpedailod_issafetoswitchtofullphysics(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041851_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_cpedail(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_041852_cpedailodmanager_getcenterinfostring_char_ptr_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041869_cpedtargetevaluator_getlockontargetaimatpos_centity_const_ptr_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_041870_rage_fwgeom_fwline_findclosestpointonline_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_041871_facingangleclamp_bool_float_float_float_float(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: f32, a4: f32) -> f32; }
extern "C" { pub fn f_041872_cpedtargetevaluator_findtarget_cpedtargetevaluator_tfindtargetpa(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041873_cpedtargetevaluator_findtargetinternal_cpedtargetevaluator_tfind(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64, a9: i64, a10: i32); }
extern "C" { pub fn f_041876_cpedtargetevaluator_getislineofsightblocked_cped_const_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_041877_cpedtargetevaluator_findtarget_cped_const_ptr_int_float_float_ce(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: i64, a5: i64, a6: i64, a7: f32, a8: f32, a9: f32, a10: f32, a11: f32) -> i64; }
extern "C" { pub fn f_041878_cpedtargetevaluator_checkbasictargetexclusions_cped_const_ptr_ce(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: f32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_041879_cpedtargetevaluator_istargetvalid_rage_vec3v_const_ref_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: f32, a11: f32, a12: i32, a13: i32, a14: i32, a15: i32) -> i32; }
extern "C" { pub fn f_041880_cpedtargetevaluator_getscorebasedonheading_centity_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: i32, a6: i32, a7: f32, a8: i64) -> f32; }
extern "C" { pub fn f_041881_arenetworksystemfriends_cped_const_ptr_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041882_cpedtargetevaluator_getcanpedbetargetedinavehicle_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041883_cpedtargetevaluator_getpedthreatassessment_cped_const_ptr_centit(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_041890_cpedtargetting_resettarget_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041894_ccurve_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_041898_ccurveset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_041901_ccurveset_finalise(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041903_ctargettingdifficultyinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_041912_cplayerpedtargeting_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041917_cplayerpedtargeting_tunables_dtortunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041918_rage_atfixedarray_ccurveset_4_dtoratfixedarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041919_cplayerpedtargeting_findlockonmeleetarget_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_041920_cplayerpedtargeting_getfindtargetflags_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041921_cplayerpedtargeting_getweaponinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041922_cplayerpedtargeting_canpedlockonwithweapon_cweapon_const_ptr_cwe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_041929_cplayerpedtargeting_getbulletbendingdata_cped_const_ptr_cweapon(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_041930_cplayerpedtargeting_renderaimassistconstraints_bool_rage_vector3(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_041931_cplayerpedtargeting_shouldusepullgunspeech_cweaponinfo_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041932_cplayerpedtargeting_processfreeaimassisttargetcapsuletests_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_041968_copdistancelistitem_ptr_std_2_partial_sort_impl_abi_nqn220108_st(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_041969_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041970_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_copdistanc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_041971_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_041972_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_041990_ctaskinfo_ctaskinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041992_cclonedtaskinfoweapontargethelper_cclonedtaskinfoweapontargethel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041995_cclonedtaskinfoweapontargethelper_updatetarget_caitarget_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041998_cqueriableinterface_gettargetfortasktype_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041999_cqueriableinterface_updatecachedinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042000_cqueriableinterface_addtasksequenceinfo_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042003_cqueriableinterface_gettaskinfofortasktype_int_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042004_cqueriableinterface_getstatefortasktype_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042008_cqueriableinterface_createemptytaskinfo_unsigned_int_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_042009_ctaskinfo_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042015_cqueriableinterface_getdebugtasktype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042016_cqueriableinterface_getdebugtaskinfotype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042017_cqueriableinterface_getdebugtaskpriority_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042025_crelationshipgroup_setrelationship_erelationtype_crelationshipgr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_042027_crelationshipgroup_hasanyrelationshipoftype_erelationtype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042036_crelationshipmanager_addrelationshipgroup_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042038_crelationshipmanager_findrelationshipgroup_rage_atnonfinalhashst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042039_crelationshipmanager_removerelationshipgroup_crelationshipgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042050_cwildlifemanager_isaquaticmodel_unsigned_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_042051_cwildlifemanager_process_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042052_cwildlifemanager_dispatchasharknearplayer_bool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042053_cwildlifemanager_getwaterspawncoord_rage_vector3_ref_bool_ref_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: f32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_042054_cwildlifemanager_shouldspawnmoreaquaticpeds_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_042055_cwildlifemanager_checkwildlifemultiplayerspawnconditions_unsigne(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_042056_cwildlifemanager_shouldspawnmoreaerialpeds_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042057_cpeddamagecylinderinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042063_cpeddamagecylinderinfoset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042068_cpeddamagecylinderinfoset_dtorcpeddamagecylinderinfoset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042070_cpeddamagetexture_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042074_cpeddamagetexture_cpeddamagetexture(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042075_cpedblooddamageinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042080_cpedblooddamageinfo_cpedblooddamageinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042081_cpeddamagedecalinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042085_cpeddamagepackentry_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042090_cpeddamagepack_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042101_preloadtexture_cpeddamagetexture_ref_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042102_cpeddamagedata_setuniquetexid_cpeddamagetexture_ref_int_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_042103_getpedname_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042104_cpeddamagesetbase_allocatetextures(ph: *mut Phantom); }
extern "C" { pub fn f_042106_cpeddamagesetbase_setped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_042107_cpeddamagesetbase_setupviewport_rage_grcviewport_ptr_int_bool_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_042108_cpeddamagesetbase_getdepthtarget_rage_grcrendertarget_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042109_cpeddamagesetbase_processskinalpha_rage_grcviewport_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042110_cpeddamagesetbase_setdecorationblitshadervars_rage_grctexture_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_042111_cpeddamagesetbase_drawblit_rage_vector2_const_ref_rage_vector2_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: f32) -> i32; }
extern "C" { pub fn f_042112_cpeddamagesetbase_delayedremovereffromtxd_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_042113_rage_atvector_cpeddamageblitdecoration_0_unsigned_short_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042114_cpeddamageblitdecoration_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042115_cpeddamageset_dtorcpeddamageset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042117_cpeddamageblitdecoration_copy_cpeddamageblitdecoration_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042118_cpeddamageblitdecoration_op_cpeddamageblitdecoration_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_042119_cpeddamageblitdecoration_loaddecorationtexture_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_042120_cpeddamagemanager_gettintinfo_cped_const_ptr_peddamagedecoration(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_042121_rage_atvector_cpeddamageblitdecoration_0_unsigned_short_growcapa(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042122_cpeddamageblitdecoration_cpeddamageblitdecoration(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042125_cpeddamageset_calcuvs_rage_vector2_ref_rage_vector2_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: f32, a8: i32, a9: i32, a10: i32) -> i32; }
extern "C" { pub fn f_042126_cpeddamagecylinder_maplocalpostouv_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042127_cpeddamagepusharea_calcpushamount_rage_vector2_const_ref_float_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_042128_cpeddamageset_adddamageblitbyuvs_rage_vector2_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32, a8: f32, a9: f32, a10: i32, a11: i32, a12: i32, a13: i32) -> i32; }
extern "C" { pub fn f_042129_cpeddamageset_adddecorationblit_rage_atnonfinalhashstring_rage_a(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: f32, a8: i64, a9: i64, a10: i64, a11: i32, a12: f32, a13: i64, a14: f32, a15: f32, a16: f32, a17: f32, a18: i32, a19: i32); }
extern "C" { pub fn f_042130_cpeddamageblitdecoration_set_rage_atnonfinalhashstring_rage_atno(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32, a8: i32, a9: i64, a10: i64, a11: f32, a12: i64, a13: f32, a14: f32, a15: f32, a16: f32, a17: i32, a18: i32) -> i32; }
extern "C" { pub fn f_042131_cpeddamagemanager_addpedscar_cped_ptr_epeddamagezones_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: f32, a6: i64, a7: i32, a8: i32, a9: i32, a10: f32, a11: i32); }
extern "C" { pub fn f_042132_cpeddamageset_cleardecorations_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_042135_cpeddamagemanager_predrawstatesetup(ph: *mut Phantom); }
extern "C" { pub fn f_042136_rage_atvector_cpeddamagecompressedblitdecoration_0_unsigned_shor(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042137_cpeddamagecompressedblitdecoration_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042138_ccompressedpeddamageset_dtorccompressedpeddamageset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042142_ccompressedtexturemanager_release_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042143_ccompressedpeddamageset_adddecorationblit_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: f32, a8: i64, a9: i64, a10: i64, a11: i32) -> i32; }
extern "C" { pub fn f_042144_cpeddamagecompressedblitdecoration_set_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32, a8: i32, a9: i64, a10: i64); }
extern "C" { pub fn f_042145_ccompressedpeddamageset_restartcompression(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042146_cpeddamagecompressedblitdecoration_cpeddamagecompressedblitdecor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042147_cpeddamagecompressedblitdecoration_loaddecorationtexture_cped_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_042148_ccompressedpeddamageset_loadoutputtexture(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042149_cpeddamagemanager_addpeddamagedecal_cped_ptr_epeddamagezones_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: f32, a6: i64, a7: i32, a8: i32, a9: f32, a10: f32, a11: i32); }
extern "C" { pub fn f_042150_cpeddamagecylinder_debugdraw_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042152_rage_atrangearray_cpeddamagecylinder_7_op_rage_atrangearray_cped(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042153_cpeddamagemanager_savedebugtargets_rage_grctexture_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042156_compresseddecorationbuilder_setupcurrentset(ph: *mut Phantom); }
extern "C" { pub fn f_042157_compresseddecorationbuilder_addtodrawlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042161_cpeddamagedata_cleararrays(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042164_cpeddamagemanager_allocatedamageset_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_042165_cpeddamagemanager_allocatecompresseddamagesetproxy_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_042167_cpeddamagemanager_computelocalpositionandzone_unsigned_char_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_042168_cpeddamagemanager_addblooddamage_unsigned_char_unsigned_short_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_042169_cpeddamagemanager_getbloodinfoanddamageid_cped_ptr_rage_atnonfin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_042170_cpeddamagemanager_getdamagedecalanddamageid_cped_ptr_rage_atnonf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_042172_cpeddamagemanager_addpedblood_cped_ptr_epeddamagezones_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_042173_cpeddamagemanager_addpedblood_cped_ptr_epeddamagezones_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: f32, a6: i64, a7: i32, a8: f32, a9: i32, a10: i32, a11: i32) -> i32; }
extern "C" { pub fn f_042175_cpeddamagemanager_addpeddecoration_cped_ptr_rage_atnonfinalhashs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: f32, a7: i64, a8: i32, a9: i64, a10: i64, a11: i32, a12: f32, a13: i32, a14: i32, a15: i64, a16: f32, a17: f32, a18: f32, a19: i32, a20: i32); }
extern "C" { pub fn f_042176_cpeddamagemanager_getclanidforped_cped_const_ptr_long_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_042177_emblemdescriptor_emblemdescriptor_int_unsigned_int_rage_rlclanem(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042178_peddamagedecorationtintinfo_peddamagedecorationtintinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042179_cpeddamagemanager_addpeddamagepack_cped_ptr_rage_atnonfinalhashs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_042180_cpeddamagemanager_addcompressedpeddecoration_cped_ptr_rage_atnon(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: f32, a7: i64, a8: i32, a9: i64, a10: i64, a11: i32, a12: i32) -> i32; }
extern "C" { pub fn f_042181_cpeddamagemanager_releasedamageset_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042184_cpeddamagemanager_testupdateblood(ph: *mut Phantom); }
extern "C" { pub fn f_042185_cpeddamagemanager_testupdatedecorations(ph: *mut Phantom); }
extern "C" { pub fn f_042187_cpeddamagemanager_addtodrawlist_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_042189_cpeddamagemanager_clearpedblood_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042191_cpeddamagemanager_promotebloodtoscars_unsigned_char_bool_float_f(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_042192_cpeddamagemanager_limitzoneblood_unsigned_char_epeddamagezones_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_042194_cpeddamagemanager_clonepeddamage_cped_ptr_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_042195_cpeddamagemanager_clearpeddecorations_unsigned_char_bool_bool_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_042196_cpeddamagemanager_clearcompressedpeddecorations_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042197_cpeddamagemanager_clearpeddecoration_unsigned_char_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_042198_cpeddamagemanager_clearcompressedpeddecoration_unsigned_char_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_042203_cpeddamagemanager_getscardataforlocalped_cped_ptr_rage_atarray_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_042225_pedheadshotinfo_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042226_pedheadshotmanager_markclonepedfordeletion_pedheadshotinfo_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042227_pedheadshottexturebuilder_releaserefs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042228_pedheadshotmanager_setupviewport(ph: *mut Phantom); }
extern "C" { pub fn f_042229_pedheadshotmanager_updateentrynameslist_rage_atvector_pedheadsho(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042230_pedheadshotmanager_onentrynameselected(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042232_pedheadshotmanager_registerped_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_042233_pedheadshotmanager_istextureavailableforslot_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_042234_pedheadshotmanager_unregisterped_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042238_pedheadshotmanager_requesttextureupload_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_042240_pedheadshotmanager_addtodrawlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042250_void_rage_dlc_addinternal_void_ptr_deferredrendererwrapper_ptr_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042252_void_rage_dlc_addinternal_void_ptr_int_renderpassid_void_ptr_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042253_void_rage_dlc_addinternal_void_ptr_bool_bool_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042254_void_rage_dlc_addinternal_void_ptr_bool_bool_void_ptr_bool_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042258_void_rage_dlc_addinternal_void_ptr_deferredrendererwrapper_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042260_void_rage_dlc_addinternal_void_ptr_deferredrendererwrapper_ptr_b(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042263_void_rage_dlc_addinternal_void_ptr_rage_grcrendertarget_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_042265_void_rage_dlc_addinternal_void_ptr_deferredrendererwrapper_ptr_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_042269_void_rage_dlc_addinternal_void_ptr_rage_grcrendertarget_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_042279_deferredrendererwrapper_setuiprojectionparams_edeferredshaders(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042292_cpedfacialoverlays_dtorcpedfacialoverlays(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042298_cpedskintones_dtorcpedskintones(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042303_ccomponentinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042307_cpvtexturedata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042312_cpvdrawbldata_cpvclothcomponentdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042317_cpvdrawbldata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042322_cpvdrawbldata_cpvdrawbldata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042323_cpvdrawbldata_dtorcpvdrawbldata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042324_cpvcomponentdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042335_cpedpropmetadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042342_cpedpropinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042347_cpedpropinfo_dtorcpedpropinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042358_firstpersonassetdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042362_firstpersonpeddata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042375_mptintassetdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042384_mptintdata_dtormptintdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042387_cpedvariationdata_cpedvariationdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042388_cpedvariationstream_setpalettetexture_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042390_cpedvariationstream_requeststreampedfilesinternal_cped_ptr_cpedv(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042393_cpedstreamrendergfx_releaseheadblendassets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042399_cpedvariationstream_isblendshapessetup_rage_rmcdrawable_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042403_csyncedpedvardata_extractfromped_cped_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042404_cpedvariationpack_getlocalcompdata_cped_const_ptr_epedvarcomp_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_042412_cpedheadblenddata_dtorcpedheadblenddata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042420_cpedvariationstream_setvariation_cdynamicentity_ptr_cpedpropdata(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_042421_cpedvariationpack_setvariation_cdynamicentity_ptr_cpedpropdata_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_042424_cpedvariationpack_createcloth_rage_charactercloth_ptr_rage_crske(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_042425_cpedvariationpack_extractcomponent_rage_pgdictionary_rage_rmcdra(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_042426_cpedvariationpack_updatepedforalphacomponents_centity_ptr_cpedpr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042427_cpedvariationpack_rendershadersetup_cpedmodelinfo_ptr_cpedvariat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042428_cpedvariationinfocollection_lookupcompinfoflags_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_042429_cpedvariationinfocollection_gettexturedistribution_epedvarcomp_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_042435_cpedvariationpack_renderslodped_rage_matrix34_const_ref_rage_grm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_042436_cpedvariationpack_getpalettevar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042437_cpedvariationpack_gettexvar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042438_cpedvariationpack_getcompvar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042441_cpedvariationpack_applyselectionsetbyhash_cped_ptr_cpedpropdata(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_042442_cpedvariationinfocollection_getmaxnumpropstex_eanchorpoints_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_042443_cpedpropdata_setpedprop_cped_ptr_eanchorpoints_int_int_eanchorpo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_042444_cpedpropsmgr_updatepedforalphaprops_centity_ptr_cpedpropdata_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042446_cpedpropsmgr_setpedprop_cped_ptr_eanchorpoints_int_int_eanchorpo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_042447_calternatevariations_salternatevariationcomponent_parser_registe(ph: *mut Phantom); }
extern "C" { pub fn f_042451_calternatevariations_salternatevariationswitch_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042455_calternatevariations_salternatevariationped_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042472_cpedvariationstream_registerstreampedfile_cdatafilemgr_datafile(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042473_cpedvariationstream_loadfirstpersondata(ph: *mut Phantom); }
extern "C" { pub fn f_042474_cpedvariationstream_addfirstpersondata_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042475_cpedvariationstream_addfirstpersonalternatedata_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042477_rage_atarray_firstpersontimecycledata_0_unsigned_short_op_rage_a(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042478_cpedvariationstream_updatefirstpersonobjectdata(ph: *mut Phantom); }
extern "C" { pub fn f_042479_rage_atarray_firstpersonalternate_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042480_rage_atarray_firstpersonassetdata_0_unsigned_short_copyfrom_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042481_rage_atarray_calternatevariations_salternatevariationcomponent_0(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042482_rage_atarray_calternatevariations_salternatevariationswitch_0_un(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042485_cpedstreamrequestgfx_addheadblendreq_unsigned_int_rage_atnonfina(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_042486_cpedvariationstream_addcomponentrequests_cpedstreamrequestgfx_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32) -> i32; }
extern "C" { pub fn f_042487_cpedvariationstream_setupskeletonmap_cpedmodelinfo_ptr_cpedstrea(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042488_cpedvariationstream_firstpersonpropdata_cleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042489_cpedpropsmgr_getpedpropidx_cped_const_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042493_cpedvariationstream_updatefirstpersonpropslot_cpedvariationstrea(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042496_cpedvariationstream_processsinglestreamrequest_cpedstreamrequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042497_cpedproprequestgfx_haveallloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042498_cpedpropsmgr_applystreampropfiles_cped_ptr_cpedproprequestgfx_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042499_cpedstreamrequestgfx_dtorcpedstreamrequestgfx(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042501_cpedstreamrendergfx_dtorcpedstreamrendergfx(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042503_cpedstreamrendergfx_addrefstogfx(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042504_cpedstreamrendergfx_setused_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042514_cpedvariationdebug_setcrowdsize_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_042515_cpedvariationdebug_setcrowdpedselection_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_042516_cpedvariationdebug_setdisttocreatecrowdfromcamera_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_042517_cpedvariationdebug_setwanderaftercreate_bool(ph: *mut Phantom); }
extern "C" { pub fn f_042518_cpedvariationdebug_setcreateexactgroupsize_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_042519_cpedvariationdebug_setcrowdspawnlocation_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042520_cpedvariationdebug_setcrowduniformdistribution_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_042521_cpedvariationdebug_updatepedlist(ph: *mut Phantom); }
extern "C" { pub fn f_042523_cpedvariationdebug_pedidchangecb(ph: *mut Phantom); }
extern "C" { pub fn f_042524_refreshvarset(ph: *mut Phantom); }
extern "C" { pub fn f_042526_cpedvariationdebug_checkvariation_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042527_createped_unsigned_int_rage_vector3_const_ref_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_042529_destroypedcb(ph: *mut Phantom); }
extern "C" { pub fn f_042532_cpedvariationdebug_createpedcb(ph: *mut Phantom); }
extern "C" { pub fn f_042533_cpedpropsmgr_widgetupdate(ph: *mut Phantom); }
extern "C" { pub fn f_042534_cpedpropdata_clearallpedprops_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042535_updatepropidxcb(ph: *mut Phantom); }
extern "C" { pub fn f_042536_updateattachidxcb(ph: *mut Phantom); }
extern "C" { pub fn f_042541_updatevarcomponentcb(ph: *mut Phantom); }
extern "C" { pub fn f_042542_updatevardrawblcb(ph: *mut Phantom); }
extern "C" { pub fn f_042543_updatevarpalettecb(ph: *mut Phantom); }
extern "C" { pub fn f_042544_updatevartexcb(ph: *mut Phantom); }
extern "C" { pub fn f_042549_setupped_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042553_setupdefaultgrouprelationshipstuff(ph: *mut Phantom); }
extern "C" { pub fn f_042555_createcrowdcb(ph: *mut Phantom); }
extern "C" { pub fn f_042558_cpedvariationdebug_populatecarcb_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_042559_cpedpropsmgr_setpedpropsrandomly_cped_ptr_float_float_unsigned_i(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32, a4: i64, a5: i64, a6: i64, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_042560_cpedvariationinfocollection_getpropcollectionmetadata_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042569_cpedvariationdebug_headblendcombochangecb(ph: *mut Phantom); }
extern "C" { pub fn f_042570_cpedvariationdebug_headblendsliderchangecb(ph: *mut Phantom); }
extern "C" { pub fn f_042595_cpedvariationdebug_updatemicromorphscb(ph: *mut Phantom); }
extern "C" { pub fn f_042598_cpedvariationdebug_updateheadblendeyecolorslider(ph: *mut Phantom); }
extern "C" { pub fn f_042607_cpedpropsmgr_getlocalpropdata_cped_const_ptr_eanchorpoints_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_042608_cpedpropsmgr_getpropmatrices_rage_matrix34_ptr_rage_crskeleton_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_042609_cpedpropsmgr_renderpropatposn_cpedmodelinfo_ptr_rage_matrix34_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64, a8: i32); }
extern "C" { pub fn f_042610_cpedpropsmgr_clearglobalshadervars_cpedmodelinfo_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_042611_cpedpropsmgr_setmaterialcolorscale_ccustomshadereffectped_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042612_cpedpropsmgr_setwetclothesdata_rage_vector4_const_ref_eanchorpoi(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042613_cpedpropsmgr_renderpropsinternal_cpedmodelinfo_ptr_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i32, a8: i32, a9: i32, a10: i32, a11: i64, a12: i32); }
extern "C" { pub fn f_042615_cpedpropsmgr_getmaxnumprops_cdynamicentity_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042617_cpedpropsmgr_tryknockoffprop_cped_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042618_cpedpropsmgr_knockoffprops_cped_ptr_bool_bool_bool_bool_rage_vec(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_042627_cpedpropdata_clearpedprop_cped_ptr_eanchorpoints_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042629_cpedpropsmgr_ispedpropdelayeddelete_cped_const_ptr_eanchorpoints(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042630_cpedpropsmgr_getpedpropactualanchorpoint_cped_ptr_eanchorpoints(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042631_cpedpropsmgr_getpedproptexidx_cped_const_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042632_cpedpropsmgr_getpropidxwithflags_cped_ptr_eanchorpoints_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042633_cpedpropdata_undoalternatedrawable_cped_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_042634_cpedpropsmgr_getpedpropspacked_cdynamicentity_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042635_cpackedpedprops_setpropdata_cped_const_ptr_unsigned_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_042636_cpedpropsmgr_setpedpropspacked_cdynamicentity_ptr_cpackedpedprop(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042637_cpackedpedprops_getmodelidx_unsigned_int_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_042639_cpedpropsmgr_clearpedprop_cped_ptr_eanchorpoints_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042641_cpedpropsmgr_getmaxnumpropstex_cdynamicentity_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_042642_cpedpropsmgr_setpreloadprop_cped_ptr_eanchorpoints_int_int_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_042643_cpedpropsmgr_haspreloadpropsfinished_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042644_cpedpropsmgr_cleanuppreloadprops_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042645_cpedpropsmgr_clearallpedprops_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042646_cpedpropsmgr_checkpropflags_cped_ptr_eanchorpoints_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_042647_cpedpropsmgr_doespedhaveproporcomponentwithflagset_cped_const_re(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042648_cpedpropsmgr_getnumactiveprops_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042650_replayextensionbase_replayobjectextension_hasextension_centity_c(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042651_replayextensionbase_replayobjectextension_add_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042653_cpedpropdata_cpedpropdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042654_cpedpropdata_dtorcpedpropdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042656_rage_strrequestarray_6u_dtorstrrequestarray(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042658_cpedproprendergfx_dtorcpedproprendergfx(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042659_cpedproprendergfx_addrefstogfx(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042660_cpedproprendergfx_setused_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042675_peddecorationcollection_peddecorationcollection(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042676_peddecorationcollection_dtorpeddecorationcollection(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042677_peddecorationcollection_edittool_updatepresetnameslist_rage_atar(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042686_peddecorationmanager_addpeddecoration_cped_ptr_rage_atnonfinalha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32); }
extern "C" { pub fn f_042687_peddecorationcollection_edittool_onpresetnameselected(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042688_peddecorationmanager_addpeddecorationcommon_cped_ptr_rage_atnonf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64) -> i64; }
extern "C" { pub fn f_042689_peddecorationmanager_addcollection_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042690_peddecorationmanager_buildserializerlists_rage_atfixedarray_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042691_peddecaldecorationmanager_releasedecal_peddecaldecoration_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042693_peddecorationmanager_getdecorationsbitfieldfromlocalped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_042694_peddecorationmanager_processdecorationsbitfieldforremoteped_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_042695_peddecorationmanager_clearcompressedpeddecorations_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042696_peddecorationmanager_addcompressedpeddecoration_cped_ptr_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_042698_peddecorationmanager_computeplacement_cped_ptr_peddecorationcoll(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_042699_peddecorationmanager_getpedmedalindex_cped_ptr_peddecorationcoll(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_042700_rage_atarray_peddecorationpreset_0_unsigned_short_op_rage_atarra(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042701_peddecorationcollection_edittool_op_peddecorationcollection_edit(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042702_peddecorationmanager_updatedebugtool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042779_bool_creplaymgrinternal_handleentityforfx_cpacketpedblooddamage(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_042781_cpickup_dtorcpickup(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042782_cpickupmanager_ccustomarchetypeinfo_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042783_cpickup_clearpendingcollection_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042784_cpickup_expose(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042788_cpickup_setportable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042789_cpickup_isalwaysfixed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042790_cwatertesthelper_getriverheightatposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_042792_cpickup_moveintointerior(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042793_cpickup_addexplosion_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042794_cpickup_attachportablepickuptoped_cped_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042795_cpickup_processcollectionresponse_rage_netplayer_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042796_cpickup_cancollectscript_cped_const_ptr_unsigned_int_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_042797_cweaponitem_getisstreamedin(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042798_cpickup_setforcealphaanduseambientscaletraversehierarchy_centity(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042799_cpickup_setuselightoverrideforglow(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042800_cpickup_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042801_cpickupplacement_setpickuphasbeendestroyed_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042802_cpickup_collectonfoot_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042803_cpickup_collectincar_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042804_cpickup_collectonshot_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042805_cpickup_updatelastaccessiblelocation_cped_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042806_cpickup_detachportablepickupfromped_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042807_cpickup_findsuitableportablepickupdroplocation_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042808_cpickup_movetoaccessiblelocation_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042809_cpickup_forceupdateofdroppedportablepickupdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042810_cpickup_cancollect_cped_const_ptr_cpickup_ecollectiontype_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_042813_cpickup_setuselightoverridetraversehierarchy_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042814_worldprobe_debugfindgroundzfor3dcoord_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> f32; }
extern "C" { pub fn f_042815_cpickup_dropinwater_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042816_cpickup_onpeddetachment_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042817_worldprobe_debugfindgroundzfor3dcoord_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32, a5: i64, a6: i64) -> f32; }
extern "C" { pub fn f_042818_worldprobe_cshapetestmanager_debugsubmittest_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_042819_cpickupmanager_calculatedroppedpickupposition_cpickupmanager_cpi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042820_worldprobe_cshapetestdesc_addentitytoinstancelist_rage_atfixedar(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042821_worldprobe_cshapetestmanager_aborttest_worldprobe_cshapetestresu(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042822_cpickup_hide_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042827_cpickupmanager_iscollectionprohibited_unsigned_int_unsigned_char(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_042836_cpedinventory_getweaponbyslot_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042837_cpickup_setlocalpendingcollection_cped_ref_cpickup_ecollectionty(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042838_cpickup_giverewards_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042839_cpickup_collect_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042841_cpickupmanager_addplacementtoregenerationlist_cpickupplacement_p(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042843_cped_getgroundpos_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042844_cwatertesthelper_getwaterheightatpositionincludingrivers_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_042845_worldprobe_cshapetesttaskdata_cshapetesttaskdata_worldprobe_csha(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_042846_worldprobe_cshapetesttaskdata_returnresults_worldprobe_cshapetes(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042847_worldprobe_cshapetesttaskdata_debugdrawshape_worldprobe_eresults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042848_worldprobe_cshapetestdesc_setexcludeentities_centity_const_ptr_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_042849_cpickupmanager_registerpickupcollection_cpickupplacement_ptr_cpe(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042851_cpickupplacement_getcustommodelhash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042876_cpickupmanager_updatepickuptypes(ph: *mut Phantom); }
extern "C" { pub fn f_042878_cpickupmanager_removeallpickups_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_042879_cpickupmanager_removepickupplacement_cpickupplacement_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042880_cpickupmanager_destroypickupplacement_cpickupplacement_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042881_cpickupplacement_createpickup_cnetobjpickup_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_042884_cpickupmanager_registerpickupplacement_unsigned_int_rage_vector3(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_042885_cpickupmanager_createpickup_unsigned_int_rage_matrix34_const_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_042886_cpickupmanager_clearspaceinpickuppool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_042887_void_std_2_introsort_std_2_classicalgpolicy_cpickupmanager_picku(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_042889_cpickupmanager_setstartingvelocities_cpickup_ptr_cped_ptr_ragdol(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_042896_cpickupmanager_detachallportablepickupsfromped_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042908_cpickupdatamanager_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_042913_cpickupplacement_createblip(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042915_cpickupplacement_dtorcpickupplacement(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042929_cpickupplacement_onregistration_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_042931_cpickupplacement_onunregistration(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042933_cpickupplacement_iscleanuprequired_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042935_cpickupplacement_cleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042937_cpickupplacement_spewdebuginfo_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042940_cpickuprewardweapon_cangive_cpickup_const_ptr_cped_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_042941_cpickuprewardweapon_shouldautoswap_cped_const_ptr_cweaponinfo_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_042943_cpickuprewardammo_getammoamountrewarded_cpickup_const_ptr_cped_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_042946_cpickuprewardbulletmp_getrewardedweaponinfo_cped_const_ptr_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042947_cpickuprewardbulletmp_getammoreward_cped_const_ptr_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043035_cbreakable_cbreakable(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043044_cbreakable_addtodrawbucket_rage_rmcdrawable_const_ref_rage_matri(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i32, a7: f32, a8: i32) -> i32; }
extern "C" { pub fn f_043046_ccollisionhistory_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043051_ccollisionhistory_debugrendercollisionrecordlist_ccollisionrecor(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_043052_ccollisionhistory_debugrendercollisionrecords_cphysical_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043053_cdeformablebonedata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_043058_cdeformableobjectentry_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_043066_cdeformableobjectmanager_dtorcdeformableobjectmanager(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043068_cdeformableobjectmanager_load_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043072_cdeformableobjectmanager_processdeformablebones_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_043074_ctunableobjectmanager_gettuningentry_rage_atnonfinalhashstring_c(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043075_rage_matrix34_transform_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043076_computesampleoffsetfromcomponentcg_cphysical_const_ptr_int_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_043077_cbuoyancy_getbuoyancyinfo_cphysical_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_043078_cwatertesthelper_getwaterlevelincludingriversnowaves_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_043085_worldprobe_cshapetestdesc_setexcludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043089_cbuoyancyinfo_dtorcbuoyancyinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043100_rage_fwanimdirector_getcomponentbytype_rage_fwanimdirectorcompon(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043133_fraginstnmgta_dtorfraginstnmgta(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043136_cphysics_canuseragdolls(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043139_fraginstnmgta_setimpactdamageentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043140_fraginstnmgta_gettimesinceimpactdamage_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043141_fraginstnmgta_sendanimationaverageveltonm_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_043143_fraginstnmgta_checkforvehicleassociation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043146_fraginstnmgta_grabrootframe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043153_fraginstnmgta_applybulletforce_cped_ptr_float_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_043154_fraginstnmgta_updatecounterimpulse_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043163_phmaterialdebug_renderbound_rage_phinst_ptr_rage_phbound_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_043165_phmaterialdebug_shouldrenderbound_rage_phinst_ptr_rage_phbound_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_043166_phmaterialdebug_renderpoly_rage_phpolygon_const_ref_phmaterialgt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_043167_phmaterialdebug_shouldrenderpoly_phmaterialgta_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_043168_phmaterialdebug_updatecurrcolourpoly_unsigned_long_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_043184_phmaterialmgrgta_preloadgtamtlfile_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043185_phmaterialmgrgta_load_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_043188_phmaterialmgrgta_dtorphmaterialmgrgta(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043205_cphysics_createinvalidragdollfilter_rage_fragtype_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043207_cphysics_setgravitylevel_cphysics_egravitylevel(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043213_rage_ropebankmanager_update_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043216_rage_ropebankmanager_addropegroup_rage_ropeinstance_ptr_rage_phi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_043221_cphysics_processcollisionvfxandsfx_vfxcollisioninfo_s_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_043222_cphysics_shouldprocesscollisionvfx_vfxcollisioninfo_s_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043227_updatedrawcollisionoptions(ph: *mut Phantom); }
extern "C" { pub fn f_043228_displayragdolldamageinfoforthisped_cped_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043230_int_ref_rage_atmap_unsigned_int_int_rage_atmaphashfn_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_043234_worldprobe_cshapetestdesc_setincludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043242_cphysics_getmeasuringtoolpos_int_rage_vector3_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_043247_worldprobe_cshapetestbatchdesc_initsubdescriptor_worldprobe_csha(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043283_physicshelpers_findvehiclecollisionfault_cvehicle_const_ptr_ccol(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32) -> i32; }
extern "C" { pub fn f_043287_cflightmodelhelper_applyforcecgsafe_rage_vector3_const_ref_cphys(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043288_cflightmodelhelper_applytorquesafe_rage_vector3_const_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_043290_cflightmodelhelper_addwidgetstobank_rage_bkbank_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043294_cragdollconstraintcomponent_enablehandcuffs_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_043295_cragdollconstraintcomponent_enableboundankles_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_043300_rage_ropebankmanager_loadfromropemanager(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043315_rage_ropebankmanager_deleterope_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043334_cwatertesthelper_getwaterlevelincludingrivers_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_043335_worldprobe_debugfindgroundzforcoord_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_043336_worldprobe_debugdrawintersection_rage_phintersection_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043337_worldprobe_cshapetest_drawline_rage_color32_char_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_043338_worldprobe_cshapetest_drawcapsule_rage_color32_char_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_043339_worldprobe_cshapetest_performlineagainstvehicletyrestest_rage_ph(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_043340_worldprobe_cshapetest_cshapetest_worldprobe_cshapetestprobedesc(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_043341_worldprobe_cshapetest_cshapetest_worldprobe_cshapetestcapsuledes(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_043345_worldprobe_cshapetestdesc_addweaponandcomponents_rage_atfixedarr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043346_worldprobe_cshapetestdesc_setincludeinstance_rage_phinst_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043398_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_cpickupman(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_043399_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_043408_blurcs_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043413_csgamemanager_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043414_csgamemanager_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043419_chorizonobjects_renderinternal_float_bool_bool_int_int_int_int(ph: *mut Phantom, a0: f32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_043433_lensartefacts_updatesortedlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043434_lensartefacts_edittool_updatelayernameslist_fwlinklist_lensartef(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043435_lensartefacts_isactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043436_csettingsmanager_getinstance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_043437_lensartefacts_setshaderdata_rage_grmshader_ptr_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043439_lensartefacts_rundirectionalblurpass_lensartefact_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_043444_meshblendmanager_sblendslot_sblendslot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043447_meshblendmanager_releaserendertarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043448_meshblendmanager_update(ph: *mut Phantom); }
extern "C" { pub fn f_043449_meshblendmanager_hasblendfinished_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_043450_meshblendmanager_releaseblend_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_043451_meshblendmanager_finalizeheadblend_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_043452_meshblendmanager_setnewblendvalues_unsigned_int_float_float_floa(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_043453_meshblendmanager_replaceassets_meshblendmanager_sblendentry_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_043454_meshblendmanager_getdrawabletexture_rage_rmcdrawable_ptr_bool_co(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043455_meshblendmanager_clonetexture_rage_grctexture_const_ptr_rage_grc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043456_meshblendmanager_finishcompressdxt_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043457_meshblendmanager_compressdxt_rage_grcrendertarget_ptr_rage_grcte(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_043458_meshblendmanager_renderthreadupdate(ph: *mut Phantom); }
extern "C" { pub fn f_043459_meshblendmanager_renderoverlay_unsigned_int_meshblendmanager_sbl(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_043460_meshblendmanager_getspectexturefromdrawable_rage_rmcdrawable_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043461_meshblendmanager_getoverlaytexture_rage_strlocalindex_eoverlayte(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_043462_meshblendmanager_gettinttexture_unsigned_int_eramptype_int_ref_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_043465_meshblendmanager_ishandlevalid_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_043466_meshblendmanager_getcrewpalettetexture_unsigned_int_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_043467_meshblendmanager_micromorph_unsigned_int_emicromorphtype_int_flo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: f32); }
extern "C" { pub fn f_043468_meshblendmanager_seteyecolor_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_043469_meshblendmanager_getblendskintexture_unsigned_int_eskinblendcomp(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_043470_meshblendmanager_setrequestsinprogress_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_043472_meshblendmanager_setoverlaytintindex_unsigned_int_unsigned_int_e(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_043473_meshblendmanager_setskinblendcomponentdata_unsigned_int_eskinble(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_043474_meshblendmanager_removeskinblendcomponentdata_unsigned_int_eskin(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_043475_meshblendmanager_sethairblendrampindex_unsigned_int_unsigned_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_043476_meshblendmanager_setpalettecolor_unsigned_int_bool_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_043477_cmirrors_createrendertargets(ph: *mut Phantom); }
extern "C" { pub fn f_043478_cmirrors_getresolutionx(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043479_cmirrors_getresolutiony(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043480_crendertargets_createrendertarget_rtmempoolid_char_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_043483_crendertargets_resolvesourcedepth_msdepthresolve_rage_grcrendert(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_043485_water_iscameraunderwater(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043493_cocclusion_rasterizeoccluders_int_rage_grcviewport_const_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_043494_rage_rsttilelist_rsttilelist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043505_occlusionqueries_oqallocate(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043507_occlusionqueries_oqsetboundingbox_unsigned_int_rage_vec3v_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_043525_cgrassrenderer_addtriplant_pptriplant_ptr_unsigned_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043526_cgrassrenderer_drawtriplants_pptriplant_ptr_unsigned_int_grassmo(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_043527_cgrassrenderer_lodbucket_16u_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043532_cgrassrenderer_lodbucket_16u_findbucket_rage_grmgeometry_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_043536_ccolorstack_getmediumcolor(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043537_cplantloctri_release_cplantloctriarray_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043538_cplantmgr_updateallloctris_cplantloctriarray_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_043539_cplantmgr_processentrycollisiondata_cplantloctriarray_ref_cplant(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_043540_extractpolycolorsofneighbours_rage_phboundgeometry_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_043543_loadtexture_char_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043544_cplantmgr_loadplantmodel_char_ptr_grassmodel_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043546_cplantmgr_reloadconfig(ph: *mut Phantom); }
extern "C" { pub fn f_043547_initialisegrassrenderstateblocks(ph: *mut Phantom); }
extern "C" { pub fn f_043548_cplantmgr_freestrloctritabs(ph: *mut Phantom); }
extern "C" { pub fn f_043549_cplantcolboundentry_releaseentry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043551_cplantmgrdataeditor_initwidgets_rage_bkbank_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043564_cplantmgr_preupdateoncefornewcameraposinternal_rage_vector3_cons(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043565_cplantmgr_calculatewindbending_rage_vector2_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043566_cplantmgr_colboundcache_update_rage_vector3_const_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043569_cplantmgr_updateend(ph: *mut Phantom); }
extern "C" { pub fn f_043570_cplantmgrdataeditor_saveeditableentryto_cplantloctri_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043574_deferredlighting_preparecutoutpass_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_043575_cplantmgr_dbgdrawtrianglearrays_rage_atarray_rage_vector3_0_unsi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043577_cplantmgr_dbgcountcachedbounds_unsigned_int_ptr_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_043578_cplantmgrdataeditor_findclosestentry_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_043579_cplantmgrdataeditor_findclosestentries_rage_vector3_const_ref_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_043580_gbuffer_unlocktargets(ph: *mut Phantom); }
extern "C" { pub fn f_043590_islightvisible_clightsource_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043592_processlodlightvisibility_processvisibility_rage_fwstepallocator(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i64, a8: i32, a9: i64, a10: i32, a11: i64, a12: i32, a13: i64, a14: i64, a15: i64, a16: i32); }
extern "C" { pub fn f_043604_recreatedofdiffusionrendertargets_rage_grctexturefactory_createp(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_043605_recreatedofrendertargets_rage_grctexturefactory_createparams_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_043606_postfx_setgaussianblur(ph: *mut Phantom); }
extern "C" { pub fn f_043607_recreatebokehrendertargets_rage_grctexturefactory_createparams_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_043608_postfx_simpleblitwrapper_rage_grcrendertarget_ptr_rage_grctextur(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043609_postfxblithelper_rage_grmshader_ptr_rage_grceffecttechnique_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_043610_postfx_filmictweakedtonemap_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_043611_postfx_setfilmicparams(ph: *mut Phantom); }
extern "C" { pub fn f_043613_postfxdamageoverlay_rage_grcrendertarget_ptr_rage_grceffecttechn(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_043614_postfx_setexposureparams(ph: *mut Phantom); }
extern "C" { pub fn f_043616_createscreenbasedrendertargets_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_043617_postfx_initmsaashaders(ph: *mut Phantom); }
extern "C" { pub fn f_043620_phonephotoeditor_initgalleryedit_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_043621_registeredanimpostfxstack_registeredanimpostfxstack(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043624_animpostfxmanager_dtoranimpostfxmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043626_postfx_addwidgetsondemand(ph: *mut Phantom); }
extern "C" { pub fn f_043635_rungaussianblur9x9_rage_grcrendertarget_ptr_rage_grcrendertarget(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043636_processbloombuffer_rage_grcrendertarget_ptr_rage_grctexture_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043637_ctiledlighting_rendertiles_rage_grcrendertarget_ptr_rage_grmshad(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_043638_blurbloombuffer_rage_grcrendertarget_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_043639_crendertargets_getbackbuffer_bool(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_043641_crendertargets_copybackbuffer(ph: *mut Phantom); }
extern "C" { pub fn f_043642_cocspread_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_043644_crendertargets_lockscenerendertargets_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043645_shaderutil_startshader_char_const_ptr_rage_grmshader_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_043646_shaderutil_endshader_rage_grmshader_const_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_043647_crendertargets_unlockscenerendertargets(ph: *mut Phantom); }
extern "C" { pub fn f_043650_pausemenupostfxmanager_updatecurrenteffect(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043651_animatedpostfxstack_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043652_animatedpostfx_update_inholdout(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043653_animatedpostfx_update_easeinorout(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043658_processdepthrender_rage_grmshader_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043660_crenderphasecascadeshadowsinterface_setsharedshadowmap_rage_grct(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043661_crendertargets_lockreadonlydepth_bool_bool_rage_grcrendertarget(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_043662_crendertargets_copydepthbuffer_rage_grcrendertarget_ptr_rage_grc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043663_postfx_setldr16bithdr16bit(ph: *mut Phantom); }
extern "C" { pub fn f_043664_crendertargets_unlockreadonlydepth(ph: *mut Phantom); }
extern "C" { pub fn f_043665_primebloombufferfrom_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043667_postfx_processnondepthfx(ph: *mut Phantom); }
extern "C" { pub fn f_043668_water_renderwaterfloddisc_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_043669_postfx_applyfilmeffect_int_rage_grcrendertarget_ptr_postfx_postf(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_043670_csprite2d_begincustomlist_csprite2d_customshadertype_rage_grctex(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_043671_phonephotoeditor_renderborder_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_043672_deferredlighting_getprojectionshaderparams_rage_vec4v_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_043674_crendertargets_getuibackbuffer_bool(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_043675_csprite2d_endcustomlist(ph: *mut Phantom); }
extern "C" { pub fn f_043677_animpostfxmanager_start_rage_atnonfinalhashstring_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_043688_animatedpostfxstack_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_043693_registeredanimpostfxstack_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_043707_animpostfxeventpollingmanager_addlistener_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_043709_animpostfxeventpollingmanager_removelistener_rage_atnonfinalhash(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043710_animpostfxmanagerreplayeffectlookup_areallgamecamstacksidle_bool(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_043713_animpostfxmanager_isrunning_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_043714_animpostfxmanager_stop_rage_atnonfinalhashstring_animpostfxmanag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043719_animpostfxmanager_loadmetadata(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043720_animpostfxmanager_edittool_updateregstacknameslist_rage_atarray(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043721_animpostfxmanager_startcrossfade_rage_atnonfinalhashstring_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_043751_phonephotoeditor_settext_char_const_ptr_char_const_ptr_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_043753_csprite2d_setgeneralparams_rage_vector4_const_ref_rage_vector4_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043758_phonephotoeditor_startediting_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_043759_chighqualityscreenshot_storescreenshotinbuffer_unsigned_char_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_043767_cdrawlistprototypemanager_flush_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043772_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_cgtapvs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_043775_ui3ddraweditor_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043779_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_043780_forceuplinkedinteriors_rage_fwinteriorlocation_cinteriorinst_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_043781_clightentity_processlightsource_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_043792_crendertargets_allocaterendertargets(ph: *mut Phantom); }
extern "C" { pub fn f_043794_deferredlighting_initshaders(ph: *mut Phantom); }
extern "C" { pub fn f_043795_deferredlighting_initrenderstates(ph: *mut Phantom); }
extern "C" { pub fn f_043796_gbuffer_createtargets_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_043797_ctiledlighting_setupshader(ph: *mut Phantom); }
extern "C" { pub fn f_043799_crenderphasecascadeshadowsinterface_script_initsession(ph: *mut Phantom); }
extern "C" { pub fn f_043800_cparaboloidshadow_getshadowres(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043801_cparaboloidshadow_createbasemaps_int_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043802_ssao_initshaders(ph: *mut Phantom); }
extern "C" { pub fn f_043803_ssao_createrendertargets(ph: *mut Phantom); }
extern "C" { pub fn f_043804_ssao_initshaders_hbao(ph: *mut Phantom); }
extern "C" { pub fn f_043805_ssao_createrendertargets_hbao(ph: *mut Phantom); }
extern "C" { pub fn f_043807_centityselect_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_043808_ssao_destroyrendertargets_mrssao(ph: *mut Phantom); }
extern "C" { pub fn f_043809_ssao_destroyrendertargets_hdao2(ph: *mut Phantom); }
extern "C" { pub fn f_043810_ssao_deleterendertargets(ph: *mut Phantom); }
extern "C" { pub fn f_043813_crenderer_constructrenderlistnew_unsigned_int_crenderphasescanne(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_043815_lights_addscenelight_clightsource_ref_clightentity_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043817_crenderer_addwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_043825_ccsmdebugglobals_initwidgets_softshadows_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043831_uisceneobjectdata_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043832_ui3ddraweditor_updatepresetnameslist_rage_atarray_uiscenepreset(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043833_water_makewaterflat(ph: *mut Phantom); }
extern "C" { pub fn f_043840_crenderer_settessellationvars_rage_grcviewport_const_ref_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043842_cgtarenderlistgroup_deferredaddentity_int_rage_fwentity_ptr_unsi(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_043850_crenderlistbuilder_addtodrawlist_int_int_crenderlistbuilder_edra(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_043852_void_rage_dlc_addinternal_void_ptr_unsigned_int_unsigned_int_boo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043854_void_rage_dlc_addinternal_void_ptr_int_rage_rmclodgroup_tessella(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043856_void_rage_dlc_addinternal_void_ptr_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043863_crenderlistbuilder_renderstatesetupnormal_int_int_int_int_unsign(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_043866_crenderlistbuilder_addtodrawlistalpha_int_int_crenderlistbuilder(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_043867_crenderlistbuilder_addtodrawlist_forcedalpha_int_int_crenderlist(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_043872_void_rage_dlc_addinternal_void_ptr_int_int_void_ptr_int_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_043874_void_rage_dlc_addinternal_void_ptr_float_float_void_ptr_float_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_043875_cinstancedrendererbatcher_1024_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043877_void_rage_dlc_addinternal_void_ptr_bool_bool_bool_bool_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_043881_setbucketsandrendermodeforoverridegbuf_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_043889_void_rage_dlc_addinternal_void_ptr_rage_grmshader_edrawtype_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043890_void_rage_dlc_addinternal_void_ptr_rage_grmshader_ptr_rage_grmsh(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043891_crenderlistbuilder_addtodrawlistshadowentities_int_bool_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_043894_void_rage_dlc_addinternal_void_ptr_unsigned_int_bool_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043895_cinstancedrendererbatcher_1024_renderbatch_instancedentitycache(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043900_crendertargetmgr_cleanupnamedrendertargets(ph: *mut Phantom); }
extern "C" { pub fn f_043907_crendertargetmgr_releasenamedrendertarget_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_043913_crendertargets_getuidepthbuffer_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_043914_crendertargets_resolvebackbuffer_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043915_crendertargets_lockscenerendertargets_depthcopy(ph: *mut Phantom); }
extern "C" { pub fn f_043917_crendertargets_getoffscreenbuffer_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_043921_crendertargets_lockuirendertargets(ph: *mut Phantom); }
extern "C" { pub fn f_043922_crendertargets_unlockuirendertargets_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043938_csprite2d_setrenderstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043940_crendertargets_copytarget_rage_grcrendertarget_ptr_rage_grcrende(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_043941_crendertargets_getreplaythumbnailtexture_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_043945_cdrawlistmgr_clothcleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043946_cdrawlistmgr_charclothcleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043949_capplydamage_drawdamage(ph: *mut Phantom); }
extern "C" { pub fn f_043950_water_updatedynamicgpu_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043952_loadwaterdata_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043956_csprite2d_clearrenderstate(ph: *mut Phantom); }
extern "C" { pub fn f_043961_cgtaoldloadingscreen_setloadingrenderfunction(ph: *mut Phantom); }
extern "C" { pub fn f_043965_ccsmglobals_arecloudshadowsenabled_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043968_river_getriverflowfromentity_waterentity_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_043969_river_ispositionnearriver_impl_rage_vec3v_const_ref_int_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_043970_river_getriverflowfromposition_rage_vec3v_const_ref_rage_vector2(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043975_csprite2d_delete(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043976_csprite2d_settexture_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043978_csprite2d_setrefmipblurparams_rage_vector4_const_ref_rage_grctex(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043979_csprite2d_settexelsize_rage_vector2_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043981_csprite2d_drawrect_rage_vector2_const_ref_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_043982_csprite2d_predrawshape(ph: *mut Phantom); }
extern "C" { pub fn f_043983_csprite2d_drawrect_float_float_float_float_float_rage_color32_co(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i64); }
extern "C" { pub fn f_043985_csprite2d_draw_rage_vector2_const_ref_rage_vector2_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_043986_csprite2d_draw_rage_vector2_const_ref_rage_vector2_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_043987_csprite2d_drawswitch_bool_bool_float_rage_vector2_const_ref_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64); }
extern "C" { pub fn f_043992_cuiworldiconquad_loadtexure(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043994_hdao_scenepreset_hdao_scenepreset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043996_hdao_scenepreset_dtorhdao_scenepreset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044000_ssao_initializeoffsetstexture(ph: *mut Phantom); }
extern "C" { pub fn f_044001_ssao_createrendertargets_mrssao(ph: *mut Phantom); }
extern "C" { pub fn f_044002_ssao_init_hdaoshaders(ph: *mut Phantom); }
extern "C" { pub fn f_044003_ssao_updatesettingstouse_hdao_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044004_ssao_init_hdao2shaders(ph: *mut Phantom); }
extern "C" { pub fn f_044008_ssao_downscaledepthtoquartertarget(ph: *mut Phantom); }
extern "C" { pub fn f_044009_ssao_apply(ph: *mut Phantom); }
extern "C" { pub fn f_044010_getssaopass_int_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_044011_ssao_drawfullscreenquad_mrssao_char_const_ptr_ssao_pass(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_044012_ssao_drawfullscreenquad_hdao_char_const_ptr_bool_hdao_pass(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_044013_ssao_drawfullscreenquad_hdao2_rage_gputimebarids_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_044014_ssao_processqs_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044015_ssao_processcpqrmix_ssao_technique(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044036_mr_ssao_scenepreset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_044103_water_initfogtexturetxdslots(ph: *mut Phantom); }
extern "C" { pub fn f_044110_createwaterocclusionqueries(ph: *mut Phantom); }
extern "C" { pub fn f_044111_int_waterquadgetpoints_cwaterquad_cwaterquad_const_ref_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_044113_water_createscreenresolutionbasedrendertargets(ph: *mut Phantom); }
extern "C" { pub fn f_044119_water_addwavetoresult_float_float_float_ptr_rage_vector3_ptr_flo(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_044120_finddynamicwaterheight_rt_int_int_rage_vector3_ptr_float_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> f32; }
extern "C" { pub fn f_044121_water_interptriangle_int_int_int_int_int_int_rage_vec2v_const_re(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64) -> f32; }
extern "C" { pub fn f_044122_water_getwaterlevel_rage_vector3_const_ref_float_ptr_bool_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_044123_water_getwaterlevelnowaves_rage_vector3_const_ref_float_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_044124_addwavetoresult_float_float_float_ptr_bool(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i32); }
extern "C" { pub fn f_044128_cwaterocclusionquery_collectquery(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044132_gettrisvertsa_float_float_float_float_float_float_float_float_fl(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: f32, a10: i64) -> i32; }
extern "C" { pub fn f_044133_gettrisvertsb_float_float_float_float_float_float_float_float_fl(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: f32, a10: i64) -> i32; }
extern "C" { pub fn f_044134_gettrisvertsc_float_float_float_float_float_float_float_float_fl(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: f32, a10: i64) -> i32; }
extern "C" { pub fn f_044135_gettrisvertsd_float_float_float_float_float_float_float_float_fl(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: f32, a10: i64) -> i32; }
extern "C" { pub fn f_044137_scancutblocks_rage_grcviewport_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044138_renderborderquads_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044139_scancutblocks_rage_grcviewport_const_ref_registerwaterquadforbox(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64); }
extern "C" { pub fn f_044140_addtodrawlistboatinteriorsnowater_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044143_ccsmsharedshadervars_setshadervars_ccsmglobals_const_ref_const(ph: *mut Phantom); }
extern "C" { pub fn f_044150_water_getwaterpixelsrendered_int_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_044151_renderlowdetailnearquads_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_044152_rendertessellatedquads_int_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_044154_crenderphasereflection_setreflectionmap(ph: *mut Phantom); }
extern "C" { pub fn f_044157_void_rage_dlc_addinternal_void_ptr_unsigned_int_unsigned_int_voi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_044163_void_rage_dlc_addinternal_void_ptr_float_bool_float_bool_void_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_044164_void_rage_dlc_addinternal_void_ptr_int_int_int_int_int_int_int_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_044166_renderlowdetailfarquads_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_044168_lights_beginlightweighttechniques(ph: *mut Phantom); }
extern "C" { pub fn f_044169_lights_endlightweighttechniques(ph: *mut Phantom); }
extern "C" { pub fn f_044171_downsamplebackbuffer(ph: *mut Phantom); }
extern "C" { pub fn f_044177_water_getcamerawaterdepth(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_044180_renderwaterrectangle_int_int_int_int_float_float_float_float_flo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: f32, a10: f32); }
extern "C" { pub fn f_044181_renderflatwaterrectangle_int_int_int_int_float_float_float_float(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: f32, a10: f32); }
extern "C" { pub fn f_044182_setfogtexture_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_044186_void_cshaderlib_setglobalvarincontext_rage_vec4v_rage_grceffectg(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_044187_clightgroup_setambientlightingglobals_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32); }
extern "C" { pub fn f_044192_crenderphasedebugoverlayinterface_internalpassbegin_bool_bool_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_044196_water_modifydynamicwaterspeed_float_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_044201_water_addfoam_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32); }
extern "C" { pub fn f_044206_czonedef_render_rage_vector3_const_ref_crgba(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_044207_chighqualityscreenshot_execute(ph: *mut Phantom); }
extern "C" { pub fn f_044208_chighqualityscreenshot_takescreenshot_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_044216_clowqualityscreenshot_execute(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044217_clowqualityscreenshot_drawphotoquad_float_float_float_float_floa(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i32, a5: i64); }
extern "C" { pub fn f_044218_clowqualityscreenshot_render_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044229_uilightrig_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_044235_uisceneobjectdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_044239_uiscenepreset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_044244_uiscenepreset_uiscenepreset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_044245_rage_atfixedarray_uisceneobjectdata_5_dtoratfixedarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044251_ui3ddrawmanager_ui3ddrawmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_044252_ui3ddrawmanager_dtorui3ddrawmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_044254_ui3ddrawmanager_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044255_uisceneobjectpatcheddata_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044257_ui3ddrawmanager_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044265_rage_atfixedarray_uisceneobjectdata_5_copy_uisceneobjectdata_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_044269_deferredlighting_setshadergbuffertarget_gbufferrt_rage_grctextur(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_044275_clightgroup_setambientlightingglobals_float_float_float_rage_vec(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: i64); }
extern "C" { pub fn f_044276_ui3ddrawmanager_generatelights_clightsource_ptr_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_044278_clightgroup_setforwardlights_clightsource_const_ptr_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_044279_clightgroup_setforwardlights_clightsource_const_ptr_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044281_ui3ddrawmanager_addpresetelementstodrawlist_uiscenepreset_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_044282_ui3ddrawmanager_setuiprojectionparams_edeferredshaders_rage_grcr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_044283_void_deferredlighting_setdeferredvar_rage_vector4_edeferredshade(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_044284_clightgroup_setdirectionallightingglobals_float_bool_rage_vec4v(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044288_clightgroup_resetlightingglobals(ph: *mut Phantom); }
extern "C" { pub fn f_044291_crenderphasereflection_setreflectionmapext_rage_grctexture_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044296_deferredlighting_renderdirectionallight_int_bool_multisamplemode(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_044297_deferredlighting_setlightshaderparams_edeferredshaders_clightsou(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_044298_deferredlighting_calculatepass_clightsource_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_044299_lights_beginlightrender(ph: *mut Phantom); }
extern "C" { pub fn f_044300_deferredlighting_calculatelightpass_unsigned_int_unsigned_char(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_044301_lights_rendervolumelight_int_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_044302_lights_endlightrender(ph: *mut Phantom); }
extern "C" { pub fn f_044303_void_rage_dlc_addinternal_void_ptr_float_float_float_float_void(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044304_void_rage_dlc_addinternal_void_ptr_rage_vector4_const_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_044305_void_rage_dlc_addinternal_void_ptr_rage_grcrendertarget_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_044307_void_rage_dlc_addinternal_void_ptr_rage_grcrendertarget_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_044324_ui3ddraweditor_onpresetnameselected(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044326_capplydamage_dtorcapplydamage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_044329_damagedrawvehicle_ref_rage_atmap_cvehicle_ptr_damagedrawvehicle(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_044331_rage_atmap_cvehicle_ptr_damagedrawvehicle_rage_atmaphashfn_cvehi(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044333_bool_rage_atmap_cvehicle_ptr_damagedrawvehicle_rage_atmaphashfn(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044337_centityselectdebugdraw_predebugrender_rage_grcrendertarget_ptr_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_044338_centityselectdebugdraw_postdebugrender(ph: *mut Phantom); }
extern "C" { pub fn f_044345_centityselect_getselectedentityshader(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_044346_centityselect_bindentityidshaderparam_unsigned_long(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044348_centityselect_addwidgets_rage_bkgroup_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044394_centitydrawdata_draw(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044395_centitydrawdatafm_draw(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044400_copyoffshader_ccustomshadereffectbase_const_ptr_unsigned_int_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_044403_rage_drawlistaddress_copyoffentity_centitydrawdata_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044404_crenderphasedebugoverlayinterface_bindshaderparams_unsigned_long(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_044405_submitdrawtree_rage_rmcdrawable_const_ptr_rage_matrix34_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_044406_drawlodlevel_rage_matrix34_const_ref_int_unsigned_int_int_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_044407_rage_drawlistaddress_copyoffentity_centitydrawdatafm_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044408_cdrawentityinstanceddc_shareddata_centity_ptr_rage_grcinstancebu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_044411_cgrassbatchdrawhandler_drawinstancedgrass_rage_rmcdrawable_ref_e(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_044413_copyoffmatrixset_rage_crskeleton_const_ref_int_ref_rage_eskelmat(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32, a10: f32) -> i32; }
extern "C" { pub fn f_044414_cdrawstreampeddc_cdrawstreampeddc_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_044415_centitydrawdatastreamped_draw_caddcompositeskeletoncommand_ref_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_044416_centitydrawdatastreamped_init_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044417_rage_drawlistaddress_copyoffentity_centitydrawdatadetachedpedpro(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044418_caddskeletoncommand_caddskeletoncommand_cdynamicentity_ptr_int_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_044419_rage_drawlistaddress_copyoffentity_centitydrawdatapedbig_centity(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044420_rage_drawlistaddress_copyoffentityvirtual_centitydrawdatapedbigw(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044421_rage_drawlistaddress_copyoffentityvirtual_centitydrawdatapedbigw(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044422_rage_drawlistaddress_copyoffentityvirtual_centitydrawdatapedbigw(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044423_rage_drawlistaddress_copyoffentityvirtual_centitydrawdatapedbigw(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044424_rage_drawlistaddress_copyoffentityvirtual_centitydrawdatapedbigw(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044425_rage_drawlistaddress_copyoffentityvirtual_centitydrawdatapedbigw(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044426_centitydrawdatapedbig_draw_unsigned_char_centitydrawdatapedprops(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_044427_centitydrawdatapedbig_init_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044428_rage_populatebitfields_rage_fraginst_ptr_unsigned_long_ref_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_044429_fragdrawcarwheels_cvehiclemodelinfo_ptr_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: f32, a8: i32, a9: i64, a10: i32, a11: f32); }
extern "C" { pub fn f_044430_cdrawfragdc_cdrawfragdc_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_044431_fragdraw_int_cbasemodelinfo_ptr_rage_grmmatrixset_const_ptr_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32, a8: f32, a9: i32, a10: i64); }
extern "C" { pub fn f_044434_cdrawfragtypedc_cdrawfragtypedc_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_044445_crenderphasephonemodel_setupphoneviewport_rage_grcviewport_ptr_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044446_cdrawnetworkplayername_ny_cdrawnetworkplayername_ny_rage_vector2(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64) -> i64; }
extern "C" { pub fn f_044447_cdrawnetworkdebugohd_ny_cdrawnetworkdebugohd_ny_rage_vector2_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64) -> i64; }
extern "C" { pub fn f_044448_entitycontainsshadername_centity_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_044449_ebstatic_dispatchcomputeshader_rage_rmcdrawable_ref_ebstatic_gra(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044450_centitydrawdatapedpropsbase_init_centity_ptr_rage_matrix34_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_044454_deferredlighting_setshadergbuffertargets_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044455_deferredlighting_setprojectionshaderparams_edeferredshaders_mult(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_044462_deferredlighting_renderfoliagehelper_int_multisamplemode_edgemod(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_044465_deferredlighting_renderdirectionalhelper_bool_multisamplemode_ed(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_044467_lights_calculatefadevalues_clightsource_const_ptr_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_044473_void_rage_dlc_addinternal_void_ptr_unsigned_int_unsigned_int_uns(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044477_centitydrawhandler_beforeaddtodrawlist_rage_fwentity_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32); }
extern "C" { pub fn f_044478_centitydrawhandler_setuplightsandglobalininteriorflag_centity_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_044479_lights_calculatepartialdrawablebound_centity_const_ref_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_044480_lights_uselightsinarea_rage_spdaabb_const_ref_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_044481_centitydrawhandler_afteraddtodrawlist_rage_fwentity_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_044489_centitydrawhandler_shouldskipentity_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_044490_centitydrawhandler_addbasictodrawlist_centitydrawhandler_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_044491_cdrawlistprototypemanager_setprototype_idrawlistprototype_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044492_centitydrawhandler_addfragmenttodrawlist_centitydrawhandler_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_044494_cinstancedrendererbatcher_1024_addinstanceddata_centity_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_044495_instancedentitycache_recomputelodlevel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044499_cdynamicentitydrawhandler_addtodrawlist_rage_fwentity_ptr_rage_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_044500_cdynamicentitydrawhandler_addfragmenttodrawlist_centitydrawhandl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_044501_cdynamicentitydrawhandler_addskinnedtodrawlist_centitydrawhandle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_044503_cvehicledrawhandler_dtorcvehicledrawhandler(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_044508_cpeddrawhandler_dtorcpeddrawhandler(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_044516_lights_calculateadjustedlightfade_float_clightentity_const_ptr_c(ph: *mut Phantom, a0: f32, a1: i64, a2: i64, a3: i32) -> f32; }
extern "C" { pub fn f_044517_lights_getbound_clightsource_const_ref_rage_spdaabb_ref_float_ed(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044525_lights_resetscenelights(ph: *mut Phantom); }
extern "C" { pub fn f_044528_cparaboloidshadow_getlightshadowindex_clightsource_const_ref_boo(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_044529_lights_drawdeflight_clightsource_const_ptr_edeferredshaders_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_044530_lights_renderscenelights_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_044531_cparaboloidshadow_setdeferredshadow_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_044532_lights_drawlightstencil_clightsource_const_ptr_edeferredshaders(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_044534_drawvolumetricfxlights_bool_bool_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_044536_addlighthash_clightentity_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_044538_lights_removeregisteredbrokenlight_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044539_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_044543_void_std_2_introsort_std_2_classicalgpolicy_lightsorter_ref_clig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_044551_clightentity_dtorclightentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_044553_clightentity_setlightbounds_clightattr_const_ptr_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_044554_clightentity_setlightshaftbounds_clightshaftattr_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_044555_clightentity_applyeffectstolightcommon_unsigned_char_rage_matrix(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_044564_clightentity_calclightintensity_bool_clightattr_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> f32; }
extern "C" { pub fn f_044565_clightentity_getworldmatrix_rage_matrix34_ref_c2deffect_ptr_clig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_044566_clightentity_addcorona_clightsource_const_ref_clightattr_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_044569_clightentity_getworldmatrix_rage_matrix34_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_044570_clightentity_setuplight_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044572_clightentity_getlight_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_044575_lightentitymgr_remove_clightentity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044576_lightentitymgr_update(ph: *mut Phantom); }
extern "C" { pub fn f_044577_lightentitymgr_flush(ph: *mut Phantom); }
extern "C" { pub fn f_044578_lightentitymgr_addattachedlights_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_044579_lightentitymgr_removeattachedlights_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044589_cparaboloidshadowactiveentry_setposdir_clightsource_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044590_cparaboloidshadowactiveentry_setinfo_clightsource_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044592_allocatetexturearrays_rage_atarray_rage_grcrendertarget_ptr_0_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_044594_cparaboloidshadow_testscreenspaceboundscommon_rage_atrangearray(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_044595_cparaboloidshadow_calcpointlightmatrix_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044596_cparaboloidshadow_calcspotlightmatrix_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044599_checkforjumpcutlightsshadowlights_rage_grcviewport_const_ref_boo(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_044602_void_rage_dlc_addinternal_void_ptr_int_int_int_bool_bool_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_044605_rage_dlc_set_global_var_rage_grceffectglobalvar_rage_mat44v_cons(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_044632_shadowrevealsetup_init_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044633_shadowrevealsetup_setvars_ccsmglobals_const_ref_rage_grctexture(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_044634_shadowprocessingsetup_init_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044635_ccsmdebugglobals_initresourcesondemand(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_044640_aresoftshadowsallowed(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_044641_ccsmdebugglobals_debugdrawrendertarget_ccsmstate_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32) -> f32; }
extern "C" { pub fn f_044642_ccsmdebugglobals_debugdrawdebugobjects_ccsmstate_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_044644_ccsmglobals_setsoftshadowproperties(ph: *mut Phantom); }
extern "C" { pub fn f_044645_ccsmglobals_resetnvidia(ph: *mut Phantom); }
extern "C" { pub fn f_044646_exteriorportals_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044647_ccsmglobals_initshaders_rage_grmshader_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044648_ccsmglobals_recomputeshadowres(ph: *mut Phantom); }
extern "C" { pub fn f_044649_ccsmglobals_createrendertargets(ph: *mut Phantom); }
extern "C" { pub fn f_044650_ccsmentitytracker_getfocalobject_rage_vec4v_ref_float_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_044651_exteriorportals_findandaddportals_cinteriorinst_ptr_rage_atfixed(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_044652_ccsmglobals_softshadowfullscreenblit_shadowprocessingsetup_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_044653_ccsmglobals_shadowrevealtoscreen_shadowrevealsetup_const_ref_edg(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32, a5: f32, a6: f32); }
extern "C" { pub fn f_044654_ccsmglobals_nvshadowreveal_ccsmstate_const_ref_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044655_exteriorportals_drawportalinfo_draw_rage_vec3v_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044692_void_rage_dlc_addinternal_void_ptr_rage_grcviewport_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_044693_void_rage_dlc_addinternal_void_ptr_unsigned_int_int_void_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_044694_csmsetrenderstates_ccsmstate_const_ref_int_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_044695_void_rage_dlc_addinternal_bool_ptr_bool_bool_bool_ptr_bool_char(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044697_void_rage_dlc_addinternal_void_ptr_rage_grcstateblock_grcrasteri(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_044698_grcviewportwindow_setcurrentviewport_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044700_ccsmdepthbiasstatebuffer_8_create_float_float_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_044710_cdebugoverlay_reset(ph: *mut Phantom); }
extern "C" { pub fn f_044729_crenderphasedebugoverlayinterface_togglepeddamagemodesetenabled(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044752_void_rage_dlc_addinternal_void_ptr_edebugoverlayrenderpass_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044753_void_rage_dlc_addinternal_bool_ptr_bool_bool_int_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_044755_isbumptex_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_044756_isdiffusetex_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_044772_getropeinstanceambscale_rage_ropeinstance_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044782_void_rage_dlc_addinternal_void_ptr_bool_ptr_rage_grmshadergroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044798_void_rage_dlc_addinternal_void_ptr_cvisualeffects_erendermode_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_044801_crenderphaseheight_dtorcrenderphaseheight(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_044815_crenderphaselensdistortion_draweye_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044822_setupmirrordepthtarget_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044824_void_rage_dlc_addinternal_void_ptr_rage_grcrendertarget_ptr_rage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044828_void_rage_dlc_addinternal_void_ptr_int_bool_int_bool_void_ptr_in(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044832_void_rage_dlc_addinternal_void_ptr_bool_rage_grcrendertarget_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_044834_void_rage_dlc_addinternal_void_ptr_cvisualeffects_erendermode_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_044838_void_rage_dlc_addinternal_void_ptr_lights_lightweighttechniquepa(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044839_void_rage_dlc_addinternal_void_ptr_cvisualeffects_erendermode_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_044844_void_rage_dlc_addinternal_void_ptr_int_int_int_int_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044851_crenderphaseparaboloidshadow_addcubemapfaces_int_int_bool_bool_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32); }
extern "C" { pub fn f_044854_void_rage_dlc_addinternal_void_ptr_rage_grcrendertarget_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_044864_void_rage_dlc_addinternal_void_ptr_rage_grctexture_const_ptr_rag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044865_void_rage_dlc_addinternal_void_ptr_float_float_float_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_044882_void_rage_dlc_addinternal_void_ptr_postfx_eresolvepause_postfx_e(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044888_void_rage_dlc_addinternal_void_ptr_pausemenurenderdataextra_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_044889_creplaycoordinator_isactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_044900_crenderphasedrawscene_builddrawlistalpha_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044901_crenderphasedrawscene_setupdrawlistalphastate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_044903_void_rage_dlc_addinternal_void_ptr_rage_fwinteriorlocation_unsig(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044931_waterreflectionshowrt_button(ph: *mut Phantom); }
extern "C" { pub fn f_044937_void_rage_dlc_addinternal_void_ptr_bool_rage_vec4v_const_ref_boo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044938_setwaterreflectionclipplanes_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_044945_void_rage_dlc_addinternal_void_ptr_rage_grccullstatus_ptr_rage_s(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044958_upsamplehelper_setupbilateralshader_upsamplehelper_upsampleparam(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044960_creplayradiostation_addtracklist_rage_replayradiostationtracklis(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044961_creplayradiostation_setlistreversed_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_044971_creplayaudiotrackprovider_isvalidsoundhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_044983_cclip_calculatetimefornondilatedtimems_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_044984_cmontage_getactivetransition_int_float_replaymarkertransitiontyp(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i64); }
extern "C" { pub fn f_044985_creplaycoordinator_shouldupdateoverlays(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_044987_creplaycoordinator_deactivate(ph: *mut Phantom); }
extern "C" { pub fn f_044988_creplaycoordinator_killplaybackorbake_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044996_creplaycoordinator_shouldshowloadingscreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045012_creplayvideometadatastore_getmetadata_int_unsigned_int_ref_crepl(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_045019_cmontage_cmontage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045020_cmontage_dtorcmontage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045025_cclip_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045026_cclip_populate_char_const_ptr_unsigned_long_replaymarkertransiti(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_045028_cclip_validatetimedata(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045031_cclip_op_cclip_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045047_cclip_calculatenondilatedtimefortimems_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_045081_cclip_validatetransitiontimes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045089_sreplaymarkerinfo_interhitproperties_sreplaymarkerinfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045097_cclip_calculateplaybackpercentageformarker_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_045139_cmontagemusicclip_isradiotrack_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045154_cmontagemusicclip_isambienttrack_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045169_cplantcolboundentry_dtorcplantcolboundentry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045174_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_045175_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_045176_std_2_pair_rage_spd_aabbindexed_ptr_rage_spd_aabbindexed_ptr_std(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_045178_rage_atpagedpool_clightentity_true_rage_fwdefaultrscmempagedpool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045179_rage_atpagedpool_clightentity_true_rage_fwdefaultrscmempagedpool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045208_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_lightsorte(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_045209_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_045284_cphotobuffer_fillinvaluesfornewphoto(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045285_csavingmessage_shoulddisplaysavingspinner(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045287_csavegamephotoasyncbuffer_csavegamephotoasyncbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045288_csavegamephotoasyncbuffer_dtorcsavegamephotoasyncbuffer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045290_cphotobuffer_getexactsizeofjpegdata(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045291_cphotobuffer_getjpegbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045352_cphotomanager_execute_rage_grctexture_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045353_cphotomanager_isprocessingscreenshot(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045785_cped_computewandertask_cped_const_ref_cpopcycleconditions_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045788_cped_getpedhealthinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045789_cped_ismale_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045792_cped_getmotivation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045795_cped_checkagilityflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045796_cped_computedefaulttask_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045797_cped_getcurrentmotiontask_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045798_cped_startprimarymotiontask_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045799_cped_init_bool_cpedmodelinfo_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_045800_cped_delayedremovaltimereset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045801_cped_delayedconversiontimereset_unsigned_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045802_cped_setcharparamsbasedonmanagertype(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045806_cped_clearwetclothing(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045810_cped_clearpedbrainwhendeletingped(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045815_cped_releasecoverpoint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045817_cped_releasedamageset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045818_cped_setproprequestgfx_cpedproprequestgfx_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045824_cped_createbank(ph: *mut Phantom); }
extern "C" { pub fn f_045833_cped_setupskeletondata_rage_crskeletondata_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045834_cped_switchtoanimated_bool_bool_bool_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_045835_cped_setbehaviorfromtaskdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045836_cped_inithealth(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045837_cpedvfx_init_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045838_cped_setupexternallydrivendofs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045840_cped_getbonematrix_rage_matrix34_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045842_cped_forcemotionstatethisframe_cpedmotionstates_emotionstate_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_045844_cped_setdefaultdecisionmaker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045863_cped_setpitch_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_045869_cped_bumpedbyentity_centity_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_045873_cped_getclimbphysical_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045874_cped_isonground_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045875_cped_setusinglowlodphysics_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045877_cped_setusinglowlodmotiontask_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045881_cped_setragdollstate_eragdollstate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045883_cped_setpedoutofvehicle_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_045885_cped_switchtoragdoll_cevent_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045889_cped_getprimarymotiontask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045895_cped_setdefaultremoverangemultiplier(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045899_cped_ourpedcanseethisentity_centity_const_ptr_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_045902_cped_updatespatialarraytypeflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045903_cped_getiscrouching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045904_cped_setiscrouching_bool_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_045906_cped_canpedcrouch_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045907_cped_canpedstealth_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045908_cped_isplayingagestureanim_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045913_cped_isfirstpersonshootermodeenabledforplayer_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045922_cped_getboneposition_rage_vector3_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045923_cped_blockgestures_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045926_cped_getisdrivingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045937_cped_hasheadblend_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045938_cped_getmodellodindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045939_cped_setvarrandom_unsigned_int_unsigned_int_rage_atfixedbitset_3(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_045940_cped_setvariation_epedvarcomp_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_045941_cped_isvariationvalid_epedvarcomp_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_045943_cped_setvardefault_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045945_cped_setheadblenddata_cpedheadblenddata_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045946_cped_setheadoverlay_eheadoverlayslots_unsigned_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_045947_cped_requestblendfromparents_cped_ptr_cped_ptr_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_045948_cped_updatechildblend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045949_cped_cloneheadblend_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_045950_cped_setheadblendpalettecolor_unsigned_char_unsigned_char_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045951_cped_setheadoverlaytintindex_eheadoverlayslots_eramptype_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_045952_cped_micromorph_emicromorphtype_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_045953_cped_setheadblendeyecolor_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045954_cped_sethairtintindex_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_045955_cped_setheadblendpalettecolordisabled(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045959_cped_setnewheadblendvalues_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_045961_cped_finalizeheadblend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045966_cped_delayedremovaltimehaspassed_float_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_045967_cped_setremoveassoonaspossible_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045968_cped_gettaskdata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045969_cped_removalispriority_rage_fwmodelid_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_045971_cped_getscenariopoint_cdynamicentity_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045973_cped_issuperlodfromindex_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045977_cped_setdesiredheading_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_045978_cped_setdesiredheading_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045979_cped_getfacingdirectionheading_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_045982_cped_stopallmotion_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045993_cped_shouldobjectbeimpossibletopush_centity_const_ref_rage_phins(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_045996_cped_getbonetagfromragdollcomponent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046001_cped_isusingstealthmode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046002_cped_isinfirstpersonvehiclecamera_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_046018_cped_addwounddata_cped_wounddata_const_ref_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_046020_cped_setstubblegrowth_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046026_cped_getvehiclepedentering_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046030_cped_updatefirstpersonflags_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046035_cped_processexternallydrivendofs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046048_cped_getisfpsswimming_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046054_cped_setusingactionmode_bool_cped_actionmodeenabled_float_bool_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_046056_cped_canbedeleted_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_046060_cped_setdamagesetid_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046061_cped_updatedamagecustomshadervars(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046063_cped_cleardamage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046067_cped_cleardecorations(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046068_cped_clonedamage_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046071_cped_isfirstpersoncameraorfirstpersonsniper_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046087_cped_isnetworkbot_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046088_cped_restoreheadingchangerate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046096_cped_getbonepositioncachedv_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046098_cped_getbonematrixfromragdollcomponent_rage_matrix34_ref_int_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046099_cped_getragdollcomponentmatrix_rage_matrix34_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_046102_cped_getisenteringvehicle_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046103_cped_getspecialability_eplayerspecialabilityslot(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046104_cped_isexitingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046105_cped_getblockingofnontemporaryeventsthisframe_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046107_cped_setdeathstate_edeathstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046109_cped_setarreststate_earreststate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046110_cped_isvalidspecialabilityslot_eplayerspecialabilityslot(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_046111_cped_ishurthealththreshold_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046112_cped_canbeinhurt_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046113_cped_candohurttransition_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046121_cped_gethasjetpackequipped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046129_cped_initspecialability_eplayerspecialabilityslot(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046135_cped_isgangped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046136_cped_shouldbehavelikelaw_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046140_cped_killpedwithcar_cvehicle_ptr_float_ccollisionrecord_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_046144_cped_newsay_char_const_ptr_unsigned_int_bool_bool_int_cped_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32) -> i32; }
extern "C" { pub fn f_046149_cped_canputonhelmet_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046152_cped_setpedinvehicle_cvehicle_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_046157_cped_setcoverpoint_ccoverpoint_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046162_cped_setclothforcepin_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046168_cped_isprone_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046169_cped_willattackinjuredpeds_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046170_cped_isallowedtodamageentity_cweaponinfo_const_ptr_centity_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_046173_cped_getmaxtimeunderwater_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_046176_cped_getcontrolfromplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046183_cped_getragdollcomponentfrombonetag_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046184_cped_getiksettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046187_cped_setendurance_float_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046209_cped_setplayerinfo_cplayerinfo_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046213_cped_isactionmodereasonactive_cped_actionmodeenabled_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046222_cped_setcontrolledbyinfo_ccontrolledbyinfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046223_cped_invalidateexternallydrivendofs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046229_cped_setlastplayertargetingrejectionstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046262_clightattr_clightattr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046263_clightattr_clightattr_rage_datresource_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046285_centity_dtorcentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046302_cdynamicentity_createskeleton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046305_cdynamicentity_addtosceneupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046308_cdynamicentity_addshadervarcreaturecomponent_ccustomshadereffect(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046316_cbuilding_cbuilding_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046317_centity_centity_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046325_cgameworld_findfollowplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_046326_cgameworldheightmap_getmaxheightfromworldheightmap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_046327_cplayerswitchinterface_isactive_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_046368_cdatafilemgr_finddatafile_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_046404_cportaltracker_setloadscollisions_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046405_cportaltracker_addtoactivatingtrackerlist_cportaltracker_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046408_cinteriorinst_getinteriorforlocation_rage_fwinteriorlocation_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046410_centity_getinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046426_cdynamicentity_getextendedboundbox_rage_spdaabb_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046429_cdynamicentity_updateskeleton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046438_cgameworld_add_centity_ptr_rage_fwinteriorlocation_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046442_cinteriorproxy_getfromlocation_rage_fwinteriorlocation_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046443_cinteriorinst_getportalinroom_unsigned_int_unsigned_int_rage_fwp(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_046444_cinteriorinst_getmatchingportalinst_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_046445_cportaltracker_collectstack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046446_cinteriorinst_getdestthruportalinroom_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046447_cinteriorproxy_createlocation_cinteriorproxy_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046457_cdynamicentity_setmatrix_rage_matrix34_const_ref_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_046462_cportaltracker_removefromactivatingtrackerlist_cportaltracker_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046467_cgameworld_remove_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046470_centity_deleteinst_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046474_centity_getshouldusescreendoor_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046479_centity_updaterenderflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046480_centity_getdrawablestatscontext(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046484_centity_setisvisibleformodule_eisvisiblemodule_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046490_centity_isvisible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046491_centity_getisonscreen_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046495_centity_isinpathserver_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046497_centity_setphysicsinst_rage_phinst_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046499_centity_getcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046500_centity_getisstatic_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046502_centity_getphysarch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046513_centity_getdefaultinstanceincludeflags_rage_phinst_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046518_centity_processfxfragmentbreak_int_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046519_centity_processfxfragmentdestroy_int_rage_matrix34_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_046523_centity_iswithinarea_float_float_float_float_float_float_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32) -> i32; }
extern "C" { pub fn f_046527_centity_getentitymodifiedtargetposition_centity_const_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046528_centity_setisvisibleforallmodules(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046531_centity_getisvisibleformodule_eisvisiblemodule_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046539_centity_getisvisibleinfo_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046541_centity_setaddedtodrawlistthisframe_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046545_centity_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046554_ebstatic_refreshmapdatadefindexmap(ph: *mut Phantom); }
extern "C" { pub fn f_046564_cfrustumdebug_debugrender_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046565_cfrustumdebug_set_rage_grcviewport_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046566_cfrustumdebug_debugrender_cfrustumdebug_const_ptr_rage_color32(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046582_cgrassbatch_bindsharedbuffer(ph: *mut Phantom); }
extern "C" { pub fn f_046586_cgrassbatch_computelod_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_046587_loddrawable_calclodlevelandcrossfade_rage_rmcdrawable_const_ptr(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32); }
extern "C" { pub fn f_046588_loddrawable_calclodlevelandcrossfadeforproxylod_rage_rmcdrawable(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_046589_ebstatic_preallocatedresourcepool_ebstatic_preallocatedappendbuf(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_046596_centityidhelper_computeentityid_rage_fwentity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046597_centityidhelper_getentityfromid_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046598_centityidhelper_getentityshaderindexfromid_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046599_centityiterator_centityiterator_int_centity_const_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_046600_centityiterator_getnext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046687_cextracontentmanager_getcontentbyhash_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046832_cextrametadatamgr_getcreaturemetadataindices_cpedmodelinfo_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046835_cpedvariationinfocollection_getdlcnamefrompropidx_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_046845_cextrametadatamgr_doesshoppedapparelhaverestrictiontag_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_047012_cfocusentitymgr_getpos(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047013_cfocusentitymgr_getentity_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047014_cfocusentitymgr_getvel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047020_cgameworld_findlocalplayerwanted(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047044_cgameworld_forallentitiesintersecting_rage_fwsearchvolume_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_047050_cphysical_cphysical_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_047055_cphysical_addphysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047058_cphysical_removeconstraints_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047060_cphysical_activatephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047062_cphysical_setfixedphysics_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_047063_cphysical_disablephysicsiffixed(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047072_cphysical_setmass_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047073_cphysical_sethealth_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047078_cphysical_getweapondamageentitypedsafe_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047095_cphysical_applyforce_rage_vector3_const_ref_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32); }
extern "C" { pub fn f_047096_cphysical_applyforcecg_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_047097_cphysical_applytorque_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_047098_cphysical_applytorque_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_047100_cphysical_applyimpulse_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047101_cphysical_applyimpulsecg_rage_vector3_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047102_cphysical_applyangimpulse_rage_vector3_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047106_cphysical_getlocalspeed_rage_vector3_const_ref_bool_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047115_cphysical_testnocollision_rage_phinst_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_047116_cphysical_debugattachtophysicalbasic_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i64, a8: i64, a9: i32); }
extern "C" { pub fn f_047123_cphysical_detachfromparent_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047126_cphysical_debugprocessallattachments_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_047129_cphysical_canphysicalbedamaged_centity_const_ptr_unsigned_int_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_047135_cphysical_calculategroundvelocity_cphysical_const_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_047178_cscene_setuprender_int_unsigned_int_rage_grcviewport_const_ref_c(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_047292_cstreamingbucketset_dumpallentitiesonce(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047299_ctexlod_storehdmapping_rage_estoretype_unsigned_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_047300_ctexlod_ismodelupgradedtohd_unsigned_int_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_047301_ctexlod_ismodelhdtxdcapable_unsigned_int_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_047302_ctexlod_isentitycloseenoughforhdswitch_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047305_cgameworld_getmainplayerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047312_csceneisolator_getentitycost_centity_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047333_clodmgr_withinvisiblerange_centity_ptr_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_047335_cpostscandebug_setcountmodelsrendered_bool_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_047345_cinteriorinst_createlocation_cinteriorinst_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_047349_cscenecosttracker_drawtitle_unsigned_int_ptr_unsigned_int_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_047350_cscenecosttracker_drawmembar_unsigned_int_ref_unsigned_int_ref_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_047374_cgameworld_changeplayerped_cped_ref_cped_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_047412_clodscale_setglobalscalefromscript_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_047414_loddrawable_calculatedrawablelodflags_rage_rmclodgroup_const_ref(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047713_cinteriorinst_getnumportalsinroom_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047714_cinteriorinst_getportalidxinroom_unsigned_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_047715_cinteriorinst_getroomhashcode_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047725_cinteriorinst_issubwaymlo_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047727_cinteriorinst_isallowrunmlo_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047729_cinteriorinst_getnumexitportalsinroom_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047731_cinteriorinst_islinkportal_unsigned_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_047732_cinteriorinst_ismirrorportal_unsigned_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_047736_cinteriorinst_setalphaforroomzeromodels_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047737_cinteriorinst_suppressinternallod(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047739_cportalinst_getdestinationdata_cinteriorinst_const_ptr_cinterior(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_047744_cinteriorinst_testportalstate_bool_bool_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_047757_cinteriorproxy_getmatrix_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047758_cinteriorproxy_getinteriorproxyfromcollisiondata_centity_ptr_rag(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047856_cportaltracker_probeforinterior_rage_vector3_const_ref_cinterior(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_047864_cportalvistracker_updateusingtarget_centity_const_ptr_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_047868_cportalvistracker_storeprimarydata_cinteriorproxy_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047869_cportalvistracker_getinteriorlocation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047880_cgameworld_findlocalplayercoors(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047932_cactiveentitylist_removeentity_fwlink_rage_activeentity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047960_cgameworld_hasentitybeenadded_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047970_cgameworld_findobjectsinrange_rage_vector3_const_ref_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_047971_cgameworld_findobjectskindacolliding_rage_vector3_const_ref_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32); }
extern "C" { pub fn f_047976_cgameworld_extinguishallcarfiresinarea_rage_vector3_const_ref_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_047982_cgameworld_findlocalplayervehicle_bool(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047992_cgameworld_stopalllawenforcersintheirtracks(ph: *mut Phantom); }
extern "C" { pub fn f_047997_cgameworld_findlocalplayerheading(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_048037_cgameworldheightmap_getminheightfromworldheightmap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_048041_cheightmap_unsigned_char_debugdraw_rage_vec3v_const_ref_rage_col(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_048042_cgameworldheightmap_getminheightfromworldheightmap_float_float_f(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_048043_cgameworldheightmap_getmaxheightfromworldheightmap_float_float_f(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_048044_cgameworldheightmap_getworldheightmapindex_int_ref_int_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_048061_cwatermap_getheight_bool_rage_vec3v_const_ref_rage_grcviewport_c(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> f32; }
extern "C" { pub fn f_057317_ped_commands_setrelationshipgroupblipforped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_060775_task_commands_commonusenearestscenariotopos_cped_ptr_rage_scrvec(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_062940_weapon_commands_commandpeddropsinventoryweapon_int_int_rage_scrv(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_063796_cthescripts_unregisterentity_cphysical_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063811_cthescripts_getpoptypename_epoptype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063813_cthescripts_getitemsettomodifyfromguid_int_char_const_ptr_unsign(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063823_cscripteulers_matrixfromeulers_rage_matrix34_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063824_cscripteulers_matrixtoeulers_rage_matrix34_const_ref_rage_eulera(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063825_cscripteulers_quaternionfromeulers_rage_quaternion_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063870_cscriptsforbrains_checkandapplyifnewentityneedsscript_centity_pt(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063893_csuppressedmodels_hasmodelbeensuppressed_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_063895_csuppressedmodels_displaysuppressedmodels(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063897_cscriptpeds_haspedmodelbeenrestrictedorsuppressed_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_063902_cscriptpeds_givepedscriptedtask_int_rage_aitask_ptr_int_char_con(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_064017_cscripthud_drawscripttext_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064018_cscripthud_drawscriptspritesandrectangles_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064022_cscripthud_drawscriptgraphics_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064041_citemset_clean(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064142_cgamescripthandlerobject_iscleanuprequired_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064215_cgamescriptid_generatescriptidhash(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064230_cgamescripthandlermgr_getnumreservedscriptentities_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_064231_cgamescripthandlermgr_getnumcreatedscriptentities_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_064232_cgamescripthandlermgr_getnumrequiredscriptentities_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_064233_cgamescripthandlermgr_getremotescriptentityreservations_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_064492_linkdatareporterplugin_report_linkdatareport(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064501_ccustomshadereffectbasetype_addref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064513_ccustomshadereffectcable_switchdrawcore_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064515_ccustomshadereffectcable_switchdepthwrite_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064537_cshaderlib_setfacingbackwards_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064544_ccustomshadereffectgrass_intersectsscriptflattenaabb_rage_fwenti(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064562_ccustomshadereffectped_setupfordrawablecomponent_rage_rmcdrawabl(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_064565_ccustomshadereffectped_getpedheat_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_064567_ccustomshadereffectped_enableshadervarcaching_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064569_ccustomshadereffectped_pedvariationmgr_setshadervariables_rage_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_064570_ccustomshadereffectped_seteditableshadervalues_rage_grmshadergro(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_064571_ccustomshadereffectped_pedvariationmgr_setwetnessadjust_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064572_ccustomshadereffectped_pedvariationmgr_setshaderclothvertices_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_064573_ccustomshadereffectped_clearshadervars_rage_rmcdrawable_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_064574_ccustomshadereffectped_clearglobalshadervars(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064592_ccustomshadereffecttint_setshadervariables_unsigned_char_ptr_ccu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_064616_ccustomshadereffecttree_enableshadervarcaching_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064632_ccustomshadereffecttree_sfxwindfieldcacheentry_sfxwindfieldcache(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064639_ccustomshadereffectvehicle_updatevehiclecolors_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_064644_ccustomshadereffectvehicle_copysettings_ccustomshadereffectvehic(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064650_ccustomshadereffectvehicle_setshadervariables_rage_rmcdrawable_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_064680_cshaderedit_updatetexture_rage_rmcdrawable_ptr_cshaderedit_textu(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_064681_cshaderedit_updatepeds_rage_strlocalindex_cshaderedit_textureslo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_064682_cshaderedit_updatehdvehicles_cshaderedit_textureslot_rage_grctex(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_064684_cshaderhairsort_pushhairtinttechnique(ph: *mut Phantom); }
extern "C" { pub fn f_064685_cshaderhairsort_pophairtinttechnique(ph: *mut Phantom); }
extern "C" { pub fn f_064687_cshaderhairsort_drawabledrawskinned_rage_rmcdrawable_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64); }
extern "C" { pub fn f_064689_cshaderlib_usesstipplefades(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_064693_cshaderlib_setglobals_unsigned_char_unsigned_char_bool_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_064695_cshaderlib_getstipplemask_unsigned_int_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064696_cshaderlib_setscreendooralpha_bool_unsigned_int_bool_bool_unsign(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_064697_cshaderlib_resetalpha_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064698_cshaderlib_setglobalalpha_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_064699_cshaderlib_setalphatestref_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_064700_cshaderlib_setglobalemissivescale_float_bool(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_064701_cshaderlib_setglobalnaturalambientscale_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_064702_cshaderlib_setglobalartificialambientscale_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_064703_cshaderlib_setglobalininterior_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064704_cshaderlib_getglobalininterior(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_064705_cshaderlib_setglobalambientocclusioneffect_rage_vector4_const_re(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064706_cshaderlib_setglobaltonemapscalers_rage_vector4_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064707_cshaderlib_setglobalscreensize_rage_vector2_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064708_cshaderlib_setmsaaparams_rage_grcdevice_msaamode_const_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064710_cshaderlib_updateglobaldevscreensize(ph: *mut Phantom); }
extern "C" { pub fn f_064711_cshaderlib_updateglobalgamescreensize(ph: *mut Phantom); }
extern "C" { pub fn f_064712_cshaderlib_disableglobalfoganddepthfx(ph: *mut Phantom); }
extern "C" { pub fn f_064713_cshaderlib_enableglobalfoganddepthfx(ph: *mut Phantom); }
extern "C" { pub fn f_064717_cshaderlib_setglobaldynamicbakeboostandwetness_rage_vec4v_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064719_cshaderlib_beginmotionblurmask(ph: *mut Phantom); }
extern "C" { pub fn f_064720_cshaderlib_endmotionblurmask(ph: *mut Phantom); }
extern "C" { pub fn f_064721_cshaderlib_setglobaldeferredmaterial_unsigned_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_064722_cshaderlib_forcescalarglobals2_rage_vector4_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064723_cshaderlib_resetscalarglobals2(ph: *mut Phantom); }
extern "C" { pub fn f_064725_cshaderlib_setstereotexture(ph: *mut Phantom); }
extern "C" { pub fn f_064726_cshaderlib_setstereoparams_rage_vector4_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064728_cshaderlib_setfograytexture(ph: *mut Phantom); }
extern "C" { pub fn f_064729_cshaderlib_setfograyintensity_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_064730_cshaderlib_setforcededge_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064736_csettingsmanager_csettingsmanager(ph: *mut Phantom); }
extern "C" { pub fn f_064737_csettingsmanager_getsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064768_cloadedmodelgroup_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064773_cloadedmodelgroup_merge_cloadedmodelgroup_ptr_cloadedmodelgroup(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_064774_cloadedmodelgroup_countmembers_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064775_cloadedmodelgroup_getmember_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_064776_cpopulationstreaming_requestvehicledriver_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064777_cpopulationstreaming_getfallbackpedgroup_cloadedmodelgroup_ref_b(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064789_cpopulationstreaming_canpedmodelridebike_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064790_cloadedmodelgroup_ismemberingroup_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_064800_cstreaming_hasobjectloaded_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064808_cpopulationstreaming_addstreamedpedvariation_cpedstreamrendergfx(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064810_cpopulationstreaming_removestreamedpedvariation_cpedstreamrender(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064811_cpopulationstreaming_removestreamedvehiclevariation_cvehiclestre(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064812_cpopulationstreaming_getfallbackpedmodelindex_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064813_cpopulationstreaming_isfallbackpedavailable_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064815_cloadedmodelgroup_pickrandomcarmodel_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_064816_cloadedmodelgroup_removemodelsnotingroup_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_064827_cloadedmodelgroup_sortpedsbydistance_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064830_cpopulationstreaming_setpedmemorybudgetlevel_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064831_cpopulationstreaming_flushallvehiclemodelshard(ph: *mut Phantom); }
extern "C" { pub fn f_064832_cstreaming_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064833_cpopulationstreaming_getscenarioslotname_escenariopopstreamingsl(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_064846_cprioritizedclipsetstreamer_findrequest_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_064862_cprioritizedclipsetrequestmanager_hasclipsetbeenrequested_cprior(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_064866_cscriptpreloaddata_cscriptpreloaddata_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_064867_cscriptpreloaddata_dtorcscriptpreloaddata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064869_cscriptpreloaddata_haspreloadfinished(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064871_cscriptpreloaddata_addtxd_rage_strlocalindex_int_ref_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_064872_cscriptpreloaddata_adddwd_rage_strlocalindex_int_int_ref_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_064875_cscriptpreloaddata_gethandle_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_064913_ckeyboard_getkeyjustdown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_064929_cstreaming_setobjectisdeletable_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064932_cstreaming_setdonotdefrag_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064933_cstreaming_requestobject_rage_strlocalindex_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_064937_cstreaming_loadobject_rage_strlocalindex_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064993_cstreamingdebug_fullmemorydump(ph: *mut Phantom); }
extern "C" { pub fn f_065072_cstreamgraph_drawtitle(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065073_cstreamgraph_drawmemorymarkers_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_065074_cstreamgraph_drawlegend_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_065076_cstreamgraph_createassetinfostring_rage_strindex_char_ptr_unsign(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_065077_cstreamgraph_drawlodmemorybar_int_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_065272_metricscapture_cmetricscapturewindow_draw(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065273_ckeyboard_getkeydown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_065332_ccontrol_getpedsprintisdown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065333_ccontrol_canusetogglerun_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065336_ccontrol_getpedsprinthistoryhelddown_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_065337_ccontrol_isvehicleattackinputdown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065352_ccontrol_cleartogglerun(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065368_ccontrol_setlighteffect_rage_iolighteffect_const_ptr_ccontrol_li(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_065369_ccontrol_clearlighteffect_rage_iolighteffect_const_ptr_ccontrol(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_065384_ccontrol_isinputdisabled_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_065385_ccontrol_getvalue_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_065446_ccontrolmgr_getmainplayercontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065451_ccontrolmgr_stoppadsshaking(ph: *mut Phantom); }
extern "C" { pub fn f_065454_ccontrolmgr_displaydisabledinputs(ph: *mut Phantom); }
extern "C" { pub fn f_065461_ccontrolmgr_getmainfrontendcontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065469_ccontrolmgr_startplayerpadshakebyintensity_unsigned_int_float_in(ph: *mut Phantom, a0: i32, a1: f32, a2: i32); }
extern "C" { pub fn f_065614_cfilemgr_readline_void_ptr_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_065615_cfilemgr_openfile_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_065617_cfilemgr_setdir_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065679_csettingsmanager_reverttoprevioussettings(ph: *mut Phantom); }
extern "C" { pub fn f_065690_csettingsmanager_safeupdate(ph: *mut Phantom); }
extern "C" { pub fn f_065691_csystem_beginrender(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065693_csystem_endrender(ph: *mut Phantom); }
extern "C" { pub fn f_065821_cclonedscriptclipinfo_cclonedscriptclipinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065875_ctasksynchronizedscene_ctasksynchronizedscene_short_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32, a5: f32, a6: i64, a7: i64, a8: f32, a9: i32, a10: i64) -> i64; }
extern "C" { pub fn f_065912_ctaskmovescripted_ctaskmovescripted_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_065943_ctaskreacharm_ctaskreacharm_rage_criksolverarms_earms_cdynamicen(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: f32, a8: f32) -> i64; }
extern "C" { pub fn f_065945_ctaskreacharm_setclip_rage_fwmvtemplateid_rage_fwmvclipsetidtype(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_065950_ctaskscriptedanimation_ctaskscriptedanimation_rage_atnonfinalhas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32, a6: f32, a7: i32, a8: i64, a9: f32, a10: i32, a11: i32, a12: i64, a13: i32, a14: i32) -> i64; }
extern "C" { pub fn f_065988_covervariationsinfo_initcovervariations_rage_atarray_rage_fwmvte(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_065993_ccombatdirector_addped_cped_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066005_ccombattaskmanager_getpedincombatwithplayer_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066008_ccombatmeleemanager_findmeleegroup_centity_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066062_ccoverpoint_getcoverpointposition_rage_vector3_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066067_ccoverpoint_getcoverdirectionvector_rage_vec3v_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066068_ccoverpoint_reservecoverpointforped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066090_ctaskcombatseekcover_ctaskcombatseekcover_cped_const_ptr_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_066096_ctaskcover_ctaskcover_cweapontarget_const_ref_int_ctaskcover_efo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_066097_ctaskcombatadditionaltask_ctaskcombatadditionaltask_int_cped_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32) -> i64; }
extern "C" { pub fn f_066100_ctaskseparate_ctaskseparate_rage_vector3_const_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_066105_ctaskcombatchargesubtask_ctaskcombatchargesubtask_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066119_ctaskstandguard_ctaskstandguard_rage_vector3_const_ref_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32, a5: f32) -> i64; }
extern "C" { pub fn f_066146_ccover_findcoordinatescoverpoint_ccoverpoint_ptr_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_066150_ctaskcombatclosesttargetinarea_ctaskcombatclosesttargetinarea_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: f32, a5: i32) -> i64; }
extern "C" { pub fn f_066157_ctaskthreatresponse_ctaskthreatresponse_cped_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_066173_ctaskcombatflank_ctaskcombatflank_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066195_ctaskmelee_findsuitableactionfortarget_cped_const_ptr_centity_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_066299_ctaskdyingdead_ctaskdyingdead_cdeathsourceinfo_const_ptr_int_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_066372_ctaskrevive_ctaskrevive_cped_ptr_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_066386_cdamageinfo_cdamageinfo_unsigned_int_rage_vector3_const_ptr_worl(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: f32, a6: i64) -> i64; }
extern "C" { pub fn f_066389_ctaskinvestigate_ctaskinvestigate_rage_vector3_const_ref_cped_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_066467_ctaskcombat_ctaskcombat_cped_const_ptr_float_rage_fwflags_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_066506_ccover_findclosestcoverpoint_cped_const_ptr_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_066510_ctaskreacttobuddyshot_ctaskreacttobuddyshot_caitarget_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_066515_ctaskvariedaimpose_ctaskvariedaimpose_ctask_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066528_ctaskdraggingtosafety_ctaskdraggingtosafety_cped_ptr_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_066578_ctasksharkcircle_ctasksharkcircle_float_caitarget_const_ref_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_066590_ctasksharkattack_ctasksharkattack_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066614_ctaskthreatresponse_determinethreatresponsestate_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_066632_ctaskhandsup_ctaskhandsup_int_cped_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_066642_ctaskreacttogunaimedat_ctaskreacttogunaimedat_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066648_ctasktohurttransit_ctasktohurttransit_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066702_ccover_findcovercoordinatesforped_cped_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_066708_ccoverpoint_ccoverpoint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066725_ccoverdebug_adddebugtext_rage_vec3v_const_ref_rage_color32_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_066729_ctaskcover_isplayeraimingdirectlyinfirstperson_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066736_ctaskmotionincover_computecoverheading_cped_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_066910_ctaskincover_canpedfireroundcorner_cped_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_066962_ctaskmotionincover_computefacingheadingforcoverdirection_float_b(ph: *mut Phantom, a0: f32, a1: i32) -> f32; }
extern "C" { pub fn f_067012_cdynamiccoverhelper_dtorcdynamiccoverhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067048_ccoverdebug_debug(ph: *mut Phantom); }
extern "C" { pub fn f_067061_ctaskstayincover_ctaskstayincover(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067147_ctaskhelichase_ctaskhelichase_caitarget_const_ref_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_067281_ctaskshellshocked_ctaskshellshocked_rage_vec3v_const_ref_ctasksh(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_067290_ctaskshootattarget_ctaskshootattarget_caitarget_const_ref_int_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32) -> i64; }
extern "C" { pub fn f_067845_cambientlookat_scoreinterestinglookat_cped_ptr_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_067846_cambientlookat_scoreinterestinglookatplayer_cped_ptr_centity_con(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_067854_ctaskambientclips_ctaskambientclips_unsigned_int_cconditionalani(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_067917_ctaskcomplexcontrolmovement_ctaskcomplexcontrolmovement_ctask_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_067931_ctaskplayeronfoot_ctaskplayeronfoot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067948_ctasksayaudio_ctasksayaudio_char_const_ptr_float_float_centity_p(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32, a6: f32) -> i64; }
extern "C" { pub fn f_067955_ctaskmobilephone_isrunningmobilephonetask_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068003_ctaskunalerted_ctaskunalerted_ctask_ptr_cscenariopoint_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_068035_ctaskunalerted_setscenariochainuserinfo_cscenariopointchainusein(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068048_ctaskwander_ctaskwander_float_float_cconditionalanimsgroup_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i32, a5: i32, a6: f32, a7: i32, a8: i32) -> i64; }
extern "C" { pub fn f_068097_ctaskstandguardfsm_ctaskstandguardfsm_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i64; }
extern "C" { pub fn f_068106_ctaskpatrol_ctaskpatrol_unsigned_int_ctaskpatrol_alertstatus_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_068126_ctaskdonothing_ctaskdonothing_int_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_068139_ctaskcrouchtoggle_ctaskcrouchtoggle_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068161_ctasksequencelist_ctasksequencelist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068163_ctasksequencelist_addtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068183_ctaskclearlookat_ctaskclearlookat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068203_ctaskusesequence_ctaskusesequence_ctasksequencelist_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_068215_ctaskopendoor_calculateclosestdoorposition_cped_const_ptr_cdoor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_068219_ctaskopendoor_navigatingpedmustabort_cped_const_ptr_cdoor_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068238_ctaskshoveped_istesting(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_068244_ctaskshoveped_shouldallowtask_cped_const_ptr_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068293_ctaskcallpolice_ctaskcallpolice_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_068454_ctaskmotionbase_setonfootclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_068461_ctaskmotionbase_setalternateclip_ctaskmotionbase_alternateclipty(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_068462_ctaskmotionbase_stopalternateclip_ctaskmotionbase_alternateclipt(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_068465_ctaskmotionbase_cpersistentdata_cpersistentdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068466_ctaskmotionbase_cpersistentdata_init_ctaskmotionbase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068468_ctaskmotionbase_cpersistentdata_apply_ctaskmotionbase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068471_ctaskmotionbase_getdefaultaimingstrafingclipset_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_068512_ctaskmotionswimming_isscubagearvariationactiveforped_cped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068524_ctaskmotionswimming_checkforrapids_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068658_ctaskgotoandclimbladder_scanforladdertoclimb_cped_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_068719_ctaskparachute_islanding_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068825_ctaskmotiontennis_ctaskmotiontennis_bool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068914_ctaskcower_ctaskcower_int_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068924_ctaskcomplexevasivestep_ctaskcomplexevasivestep_centity_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_069009_ctaskparachute_ctaskparachute_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_069017_ctaskparachute_iscrashlanding_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069018_ctaskfall_getmaxpedfallheight_cped_const_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069019_ctaskparachute_canpedparachute_cped_const_ref_float(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069032_ctaskflytopoint_ctaskflytopoint_float_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_069064_ctaskturntofaceentityorcoord_ctaskturntofaceentityorcoord_centit(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_069065_ctaskmovegotopointandstandstill_ctaskmovegotopointandstandstill(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: f32) -> i64; }
extern "C" { pub fn f_069067_ctaskmovestandstill_ctaskmovestandstill_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_069108_cdropdown_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069149_ctaskgotopointanymeans_ctaskgotopointanymeans_float_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: f32, a5: i32, a6: i32, a7: i32, a8: f32, a9: f32, a10: f32, a11: i32, a12: f32, a13: f32, a14: f32) -> i64; }
extern "C" { pub fn f_069166_ctaskmovegotopoint_ctaskmovegotopoint_float_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_069242_ctaskgotopointaiming_ctaskgotopointaiming_caitarget_const_ref_ca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_069256_ctaskmovefollowentityoffset_ctaskmovefollowentityoffset_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_069272_ctaskmovebeinformation_ctaskmovebeinformation_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069316_cnavmeshroute_expandroute_cnavmeshexpandedroutevert_ptr_float_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_069363_ctasknavbase_getpathintersectsregion_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_069394_ctaskmovewithinattackwindow_getattackwindowminmax_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_069443_cclimbdetector_searchforhandhold_rage_matrix34_const_ref_cclimbh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_069477_ctaskmovefollownavmesh_ctaskmovefollownavmesh_float_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: i64, a8: f32, a9: i32) -> i64; }
extern "C" { pub fn f_069530_ctaskparachute_findparachutepackvariationforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069531_ctaskparachute_isparachutepackvariationactiveforped_cped_const_r(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069534_ctaskparachute_gettintindexforparachutepack_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069543_ctaskparachute_getmodelforparachute_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069606_ctaskparachute_getmodelforparachutepack_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069607_ctaskparachute_gettintindexforparachute_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069608_ctaskparachute_isparachute_cobject_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069697_ctaskseekentityaiming_ctaskseekentityaiming_centity_const_ptr_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> i64; }
extern "C" { pub fn f_069702_ctaskslidetocoord_ctaskslidetocoord_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> i64; }
extern "C" { pub fn f_069704_ctaskslidetocoord_ctaskslidetocoord_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_069755_cclimbhandholddetected_cclimbhandholddetected(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069844_ctaskgotoandclimbladder_ctaskgotoandclimbladder_centity_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_069867_ctaskhelipassengerrappel_ctaskhelipassengerrappel_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i64; }
extern "C" { pub fn f_069915_ctaskvault_ctaskvault_cclimbhandholddetected_const_ref_rage_fwfl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_069980_ctaskfallandgetup_ctaskfallandgetup_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i64; }
extern "C" { pub fn f_069982_ctaskfallandgetup_getclipphase(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069993_ctaskjumpvault_ctaskjumpvault_rage_fwflags_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_070005_ctaskjump_ispedneargroundincradius_cped_const_ptr_float_float_ra(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_070372_cnmblendoutsetmanager_load(ph: *mut Phantom); }
extern "C" { pub fn f_070374_cnmblendoutsetmanager_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_070383_cnmdebug_setcomponenttmsfromskeleton_rage_crskeleton_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070385_ctaskanimatedattach_ctaskanimatedattach_cphysical_ptr_int_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_070399_ctasknmcontrol_ctasknmcontrol_unsigned_int_unsigned_int_rage_ait(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: f32) -> i64; }
extern "C" { pub fn f_070401_ctasknmbehaviour_canuseragdoll_cped_ptr_eragdolltriggertypes_cen(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_070422_cnmtuningset_cnmtuningset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070423_cnmtuningset_dtorcnmtuningset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070490_ctasknmbehaviour_loadtaskparameters(ph: *mut Phantom); }
extern "C" { pub fn f_070529_ctasknmhighfall_ctasknmhighfall_unsigned_int_cgrabhelper_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_070530_ctasknmbalance_ctasknmbalance_unsigned_int_unsigned_int_centity(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64, a6: f32, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_070531_ctasknmbalance_evaluateandsettype_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070532_ctasknmexplosion_ctasknmexplosion_unsigned_int_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_070561_ctasknmbrace_ctasknmbrace_unsigned_int_unsigned_int_centity_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_070636_ctasknmfalldown_ctasknmfalldown_unsigned_int_unsigned_int_ctaskn(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_070647_ctasknmflinch_ctasknmflinch_unsigned_int_unsigned_int_rage_vecto(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i64, a10: i64, a11: i64, a12: i64) -> i64; }
extern "C" { pub fn f_070675_ctasknmsimple_ctasknmsimple_ctasknmsimple_tunables_tuning_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070694_ctasknmjumprollfromroadvehicle_ctasknmjumprollfromroadvehicle_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_070707_ctasknmonfire_ctasknmonfire_unsigned_int_unsigned_int_float_rage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070715_ctasknmpose_ctasknmpose_unsigned_int_unsigned_int_rage_fwmvtempl(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: f32, a6: f32, a7: i32, a8: i32) -> i64; }
extern "C" { pub fn f_070739_ctasknmrelax_ctasknmrelax_unsigned_int_unsigned_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_070747_ctasknmriverrapids_ctasknmriverrapids_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_070769_ctasknmshot_ctasknmshot_cped_ptr_unsigned_int_unsigned_int_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32, a7: i64, a8: i64, a9: i64, a10: i32) -> i64; }
extern "C" { pub fn f_070774_ctasknmshot_getshottypename_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070846_ctaskrageragdoll_ctaskrageragdoll_bool_centity_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_070874_ctaskagitated_findagitatedtaskforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070913_ctasksmartflee_ctasksmartflee_caitarget_const_ref_float_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_070964_ctaskescapeblast_ctaskescapeblast_centity_ptr_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: f32) -> i64; }
extern "C" { pub fn f_071039_ctaskreactandflee_ctaskreactandflee_caitarget_const_ref_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_071075_ctaskshockingeventhurryaway_ctaskshockingeventhurryaway_ceventsh(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_071088_ctaskreacttoexplosion_ctaskreacttoexplosion_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_071101_ctaskreacttoimminentexplosion_ctaskreacttoimminentexplosion_cait(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i64; }
extern "C" { pub fn f_071128_ctaskshockingeventreact_ctaskshockingeventreact_ceventshocking_c(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_071141_ctaskshockingeventbackaway_ctaskshockingeventbackaway_ceventshoc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_071168_ctaskshockingeventbackaway_getdefaultbackawaypositionforped_cped(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_071797_rage_atarray_cagitatedtrigger_0_unsigned_short_kill(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071805_cagitatedtriggerreaction_cagitatedtriggerreaction(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072481_cscenariopoint_getposition_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072490_cscenariopoint_getscenariotypevirtualorreal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072507_cscenariochaininggraph_registerchainuserdummy_cscenariopoint_con(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_072509_cscenariochaininggraph_unregisterpointchainuser_cscenariopointch(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072511_cscenariochaininggraph_updatenewlyloadedscenarioregions(ph: *mut Phantom); }
extern "C" { pub fn f_072539_cscenariomanager_getscenarioinfo_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_072547_cscenariomanager_isvehiclescenariotype_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_072553_cscenariomanager_selectpedtypeforscenario_cscenariomanager_pedty(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_072558_cscenariomanager_setupscenarioandtask_int_rage_vector3_ref_float(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_072559_cscenariomanager_findpropsinenvironmentforscenario_cobject_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_072560_cscenariomanager_createscenarioped_int_cscenariopoint_ref_ctask(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i64, a11: i64, a12: i32, a13: i64) -> i64; }
extern "C" { pub fn f_072563_cscenariopoint_ishighpriority_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072564_cscenariomanager_checkscenariotimesandprobability_cscenariopoint(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_072577_ctaskscenario_ctaskscenario_int_cscenariopoint_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_072580_cscenariomanager_performcollisionchecksforscenariopoint_cscenari(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072629_cscenariomanager_getscenariotypefromname_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072662_cscenariopointmanager_findpointsinarea_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_072663_cscenariomanager_checkscenariopointenabled_cscenariopoint_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_072668_cscenariomanager_checkscenariopointmapdata_cscenariopoint_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072774_cscenariopointmanager_deletepointswithattachedentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072800_cscenariomanager_checkscenariopopulationcount_int_rage_vector3_c(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_072802_cscenariomanager_isscenariopointattachedentityuprootedorbroken_c(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072807_cscenariospawncheckcache_entry_updatetime_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_072822_cscenariomanager_getconditionalanimsgroupforscenario_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_072823_cscenariomanager_checkscenariopointenabledbygroupandavailability(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072824_cscenariomanager_checkscenariopointenabledbytype_cscenariopoint(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_072825_cscenariomanager_islookatscenariotype_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_072830_cscenariomanager_getdefaultscenarioinfoforrandomped_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_072831_cscenariomanager_applydefaultscenarioinfoforrandomped_defaultsce(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072832_cscenariopoint_isinteriorstate_cscenariopoint_interiorstate_cons(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_072839_cscenariopoint_getuses_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072840_cscenariopoint_getfailedcollisionticks(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_074161_ctask_ctask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_074163_ctaskclassinfomanager_gettaskname_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_075327_ctaskcomplex_ctaskcomplex(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075332_ctaskfsmclone_ctaskfsmclone(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075341_ctaskhelperfsm_ctaskhelperfsm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075352_ccliprequesthelper_releaseclips(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075353_ccliprequesthelper_requestclipsbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075354_cgetupsetrequesthelper_cgetupsetrequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075367_cnavmeshclosestpositionhelper_resetsearch(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075372_cprioritizedclipsetrequesthelper_getclipsetid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075389_cnavmeshroutesearchhelper_cnavmeshroutesearchhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075390_cnavmeshroutesearchhelper_dtorcnavmeshroutesearchhelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075442_cnavmeshlineofsighthelper_cnavmeshlineofsighthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075445_casyncprobehelper_casyncprobehelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075446_casyncprobehelper_dtorcasyncprobehelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075447_casyncprobehelper_resetprobe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075448_casyncprobehelper_starttestlineofsight_worldprobe_cshapetestprob(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075449_casyncprobehelper_getproberesults_probestatus_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075450_casyncprobehelper_getproberesultswithintersection_probestatus_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075451_casyncprobehelper_getproberesultswithintersectionandnormal_probe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_075452_casyncprobehelper_getproberesultsvfx_probestatus_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64) -> i32; }
extern "C" { pub fn f_075495_cchathelper_getpedstask_cped_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075523_ctuning_dtorctuning(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075524_cvehiclecliprequesthelper_cvehiclecliprequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075525_cvehiclecliprequesthelper_dtorcvehiclecliprequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075537_cvehiclecliprequesthelper_debug_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075538_ctaskentervehiclealign_getcommonclipsetid_centity_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_075539_ctaskentervehiclealign_getentryclipsetid_centity_const_ptr_int_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_075576_ccoverpointstatushelper_ccoverpointstatushelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075584_cexplosionhelper_calculatetimeuntilexplosion_centity_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075599_ctimehelpers_gettimesince_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_075604_ctaskcontrolvehicle_ctaskcontrolvehicle_cvehicle_ptr_rage_aitask(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_075625_ctaskmanager_processmovesignals_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075631_ctaskmove_ctaskmove_float(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_075642_ctasktreeclone_recalculatetasks(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075643_ctasktreeclone_createclonetaskfromtaskinfo_ctaskinfo_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_075660_ctuning_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_075664_ctuning_registertuning_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_075693_cmotiontaskdatamanager_getdataset_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_075695_ctaskleaveanycar_ctaskleaveanycar_int_cvehicleenterexitflags_fla(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_075697_ctaskexitvehicle_ctaskexitvehicle_cvehicle_ptr_cvehicleenterexit(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_075702_ctaskinvehiclebasic_ctaskinvehiclebasic_cvehicle_ptr_bool_ccondi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_075710_ctaskvehiclegun_ctaskvehiclegun_ctaskvehiclegun_etaskmode_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_075724_ctaskentervehicle_ctaskentervehicle_cvehicle_ptr_seatrequesttype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: f32, a6: f32, a7: i64, a8: i64) -> i64; }
extern "C" { pub fn f_075727_ctaskcardrivewander_ctaskcardrivewander_cvehicle_ptr_int_float_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_075752_ctaskvehiclefsm_computetargettransforminworldspace_cvehicle_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_075753_ctaskinvehicleseatshuffle_ctaskinvehicleseatshuffle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_075758_ctaskvehiclefsm_getpedinorusingseat_cvehicle_const_ref_int_cped(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_075759_ctaskcomplexgetoffboat_ctaskcomplexgetoffboat_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_075804_ctaskcarreacttovehiclecollision_ctaskcarreacttovehiclecollision(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_075819_ccarenterexit_getcollideddoorfromcollisioncomponent_cvehicle_con(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i64; }
extern "C" { pub fn f_075820_ccarenterexit_getdoorcomponents_cvehicle_const_ptr_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_075871_ctaskvehiclefsm_arepedsfriendly_cped_const_ptr_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075928_ctaskvehiclefsm_canpedwarpintohoveringvehicle_cped_const_ptr_cve(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_075991_ctaskvehiclefsm_computeidealseatmatrixforposition_cturret_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_076006_ctaskentervehicle_isconsideredgettinginvehicleforambientscale(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076144_ctaskexitvehicleseat_getclipandphaseforstate_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076232_cvehiclehullai_init_float_ehierarchyid_ptr_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_076318_ctaskvehiclemountedweapon_ctaskvehiclemountedweapon_ctaskvehicle(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_076371_ctaskvehicleprojectile_ctaskvehicleprojectile_ctaskvehicleprojec(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_076518_cvehiclehullai_cvehiclehullai_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_076519_cvehiclehullai_dtorcvehiclehullai(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076520_cvehiclecomponentvisualiser_visualisecomponents_cvehicle_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_076746_ctaskgun_ctaskgun_cweaponcontroller_weaponcontrollertype_ctaskty(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32) -> i64; }
extern "C" { pub fn f_076865_ctaskgun_getisfiring_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076895_ctaskreloadgun_ctaskreloadgun_cweaponcontroller_weaponcontroller(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_077089_cstatsmgr_incrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077095_cstatsmgr_isplayeractive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077151_statid_char_gethash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077153_statid_char_getstatid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077193_cstatsdatamgr_iskeyvalid_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077199_cstatsdatamgr_setstatdata_statid_const_ref_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_077329_statid_set_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077331_statsinterface_getintstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077338_statsinterface_getcurrentmultiplayercharaterslot(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077342_statsinterface_setstatdata_statid_const_ref_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077347_statsinterface_setstatdata_statid_const_ref_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077354_statsinterface_getfloatstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_077358_statsinterface_getuint32stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077373_statsinterface_getstatsmodelhashid_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077378_statsinterface_getmaskedint_int_int_ref_int_int_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_077394_cstatsutils_isstatstrackingenabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077479_ctextfile_get_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077496_cmessages_clearmessages(ph: *mut Phantom); }
extern "C" { pub fn f_077504_cmessages_addmessage_char_const_ptr_int_unsigned_int_bool_bool_e(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i64, a9: i32, a10: i32, a11: i32); }
extern "C" { pub fn f_077507_chelpmessage_setmessagetext_int_char_const_ptr_cnumberwithinmess(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_077508_chelpmessage_clear_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_077515_chelpmessage_getmessagetext_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077525_cloadingtext_setactive_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077533_ctextlayout_ctextlayout(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077535_ctextlayout_dtorctextlayout(ph: *mut Phantom); }
extern "C" { pub fn f_077537_ctextlayout_setscale_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077540_ctextlayout_render_rage_vector2_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077544_ctextformat_getstringwidth_ctextlayout_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_077545_ctextlayout_setscale_rage_vector2_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077549_ctext_flush(ph: *mut Phantom); }
extern "C" { pub fn f_077612_ctimecycle_calcambientscale_rage_vec3v_const_ref_rage_fwinterior(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077620_ctimecycle_setshaderdata_cvisualeffects_erendermode(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077622_cvisualsettings_get_char_const_ptr_float_bool_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_077638_cvisualsettings_getcolor_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077803_cvehicleintelligence_getactivetask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077812_cpathfind_findnodeclosesttocoors_rage_vector3_const_ref_float_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: f32, a11: i32, a12: i64, a13: i32, a14: i32); }
extern "C" { pub fn f_077849_cpathfind_generatedirections_int_ref_unsigned_int_ref_float_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_077958_cvehicleintelligence_getdrivingflagname_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077972_ctaskvehiclemissionbase_ctaskvehiclemissionbase_svehiclemissionp(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077973_ctaskvehiclegotosubmarine_ctaskvehiclegotosubmarine_svehiclemiss(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32) -> i64; }
extern "C" { pub fn f_077974_ctaskvehiclegotohelicopter_ctaskvehiclegotohelicopter_svehiclemi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_077975_cvehicleintelligence_createautomobilegototask_svehiclemissionpar(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_077977_ctaskvehicleattack_ctaskvehicleattack_svehiclemissionparams_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077978_ctaskvehiclecrash_ctaskvehiclecrash_centity_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_077979_cvehicleintelligence_gethelitaskfrommissionidentifier_cvehicle_p(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32, a8: f32, a9: i32) -> i64; }
extern "C" { pub fn f_077980_cvehicleintelligence_getplanetaskfrommissionidentifier_cvehicle(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32) -> i64; }
extern "C" { pub fn f_077981_cvehicleintelligence_gettaskfrommissionidentifier_cvehicle_ptr_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: f32, a6: f32, a7: f32, a8: i32, a9: i32) -> i64; }
extern "C" { pub fn f_077991_ctaskvehiclegotoplane_ctaskvehiclegotoplane_svehiclemissionparam(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: f32) -> i64; }
extern "C" { pub fn f_077997_ctaskvehicleplanechase_ctaskvehicleplanechase_svehiclemissionpar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_078054_ctaskvehiclemissionbase_ctaskvehiclemissionbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078780_canchorhelper_isvehicleanchorable_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078782_canchorhelper_getanchorhelperptr_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078785_canchorhelper_cananchorhere_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078786_cvehicle_getnumcontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078787_camphibiousautomobile_ispropellersubmerged(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078848_cvehicle_getslipstreameffect_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_078859_cvehicle_canbeinactiveduringrecording_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078867_cvehicledeformation_readfromvectoroffset_void_const_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_078871_cvehicle_containslocalplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078886_cvehicle_canpedsstandontop_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078889_cvehicle_getdoorhierachyidfromimpact_cvehicle_const_ref_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078909_cvehicle_istaximodelid_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078929_cvehicle_isbusmodelid_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078933_cvehicle_isupsidedown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078934_cvehicle_isonitsside_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078936_cvehicle_ishornon_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078939_cvehicle_switchengineon_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_078947_cvehicledamage_getisdriveable_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_078971_cvehicledebug_getfocusvehicle(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_078974_cvehicledamage_getenginehealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_078981_cvehicle_hascontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079038_cvehicle_getlastdriver_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079039_cvehicle_isturretseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079040_cvehicle_getspecialflightmoderatio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_079047_cwheel_getwheelposandradius_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_079065_cvehicle_getdoorfromid_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079070_cvehicle_getseatanimationinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079071_cvehicle_isentryindexvalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079097_ccardoor_setflag_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079130_cvehicle_getlayoutinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079157_cvehicle_getisjetski_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079167_cvehicle_shouldlightsbeon_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079253_ccomponentreservationmanager_getseatreservation_int_seataccess_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_079266_ccardoor_clearflag_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079267_ccardoor_getisintact_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079335_chandlingdata_gethandlingdata_ehandlingtype(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079347_chandlingdatamgr_gethandlingdata_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_079422_cvehicle_getvehiclegadget_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079441_clandinggear_getpublicstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079485_caircraftdamage_getsectionfromchildindex_cplane_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_079581_cvehicle_isindriveablestate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079587_cvehicle_getfragmentcomponentindex_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079746_cvehicle_doesvehiclehaveaconvertibleroofanimation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079758_cvehicle_getconvertibleroofstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079770_cvehicle_getvehiclecoverboundoffsetinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079781_cvehicle_hasmissioncharsinit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079790_cvehicle_trytogiveeventreponsetasktooccupant_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079791_cvehicle_isdriver_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079812_cvehicle_getentryinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079816_cvehicle_getseatinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079817_cvehicle_getseatinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_079821_cvehicle_setuppassenger_int_bool_unsigned_int_cambientmodelvaria(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_079826_cvehicle_canpedopenlocks_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079831_cvehicle_getdirectentrypointindexforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079841_cvehicle_isonfire_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079843_cvehicle_carhasroof_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079845_cvehicle_ispersonalvehicle(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079855_cvehicle_canbedriven_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079872_cvehicle_getseathasweaponsorturrets_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079873_cvehicleweaponmgr_getnumweaponsforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079885_cvehicle_getentryexitpoint_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079889_cvehicle_isentrypointclearforped_cvehicle_const_ref_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_079892_cvehicle_haswaterentry_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079899_cvehicle_getrelativevelocity_cped_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079931_cvehicle_getdirectentrypointindexforped_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079944_cvehicle_isinenterablestate_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079947_cvehicle_setgravitationalacceleration_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_079994_vehtexcacheentry_isdamagequeued(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079998_cwheel_resetdamage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080044_cvehiclefactory_carnumchangecb(ph: *mut Phantom); }
extern "C" { pub fn f_080054_cvehiclepopulation_getdesirednumberambientvehicles_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> f32; }
extern "C" { pub fn f_080187_cvehiclefactory_createcar_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080274_cvehiclegadgetdiggerarm_getjointpositionratio(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_080306_cvehiclegadgetforks_detachpalletimmediatelyandcleanup_cvehicle_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080421_cvehiclepopulation_getparkedcardensitymultiplier(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_080422_cvehiclepopulation_getlowprioparkedcardensitymultiplier(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_080444_cvehiclepopulation_removeallvehs_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_080483_cwheel_getwheelmatrixandbbox_rage_matrix34_ref_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080488_cwheelinstancerenderer_renderallwheels(ph: *mut Phantom); }
extern "C" { pub fn f_080541_cvehiclemetadatamgr_getvehicledrivebyinfofromped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080542_cvehiclemetadatamgr_getdrivebyaniminfoforweapon_cped_const_ptr_u(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080639_cvehiclecoverboundinfo_isboundactive_cvehiclecoverboundinfo_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_080670_cvehiclemetadatamgr_gettotalnumfirstpersondrivebylookarounddata(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_080671_cvehiclemetadatamgr_getfirstpersondrivebylookarounddataatindex_u(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_080865_cvfxhelper_getmatrixfrombonetag_rage_mat34v_ref_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_080866_cvfxhelper_getmatrixfromboneindex_rage_mat34v_ref_bool_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_080869_cvfxhelper_getmatrixfromboneindex_smash_rage_mat34v_ref_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_080872_cvfxhelper_creatematfromvecyalign_rage_mat34v_ref_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_080874_cvfxhelper_creatematfromveczalign_rage_mat34v_ref_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_080882_cvfxhelper_isentityincurrentvehicle_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080888_cvfxhelper_shouldrenderingameui(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_080911_cptfxmanager_beginframe(ph: *mut Phantom); }
extern "C" { pub fn f_080985_cfogvolumemgr_register_fogvolume_t_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080992_cvehicleglassmanager_updatesafe_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_080993_cdecalmanager_updatestartasync_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_080996_cvehicleglasscomponent_updatecomponent_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_080997_cvehicleglasscomponent_shutdowncomponent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080998_cvehicleglassmanager_addtorenderlist_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081001_cvisualeffects_rendersky_cvisualeffects_erendermode_bool_bool_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: f32); }
extern "C" { pub fn f_081002_rage_cloudhatmanager_draw_rage_cloudframe_const_ref_bool_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_081163_vfxpedskeletonboneinfo_getbonewaterinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081165_cvfxpedinfomgr_getinfo_cped_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081191_cvfxvehicleinfomgr_getinfo_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081355_cfire_getpositionworld_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081358_cvfxmaterial_getbanginfo_vfxgroup_e_vfxgroup_e(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_081392_cdecalmanager_remove_rage_fwentity_const_ptr_int_void_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_081416_ccoronas_register_rage_vec3v_const_ref_float_rage_color32_float(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: f32, a4: f32, a5: i64, a6: f32, a7: f32, a8: f32, a9: i32); }
extern "C" { pub fn f_081417_cmarkers_register_markerinfo_t_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081428_cdistantlights2_preparedistantcarsrenderdata(ph: *mut Phantom); }
extern "C" { pub fn f_081439_cfiremanager_getentityfire_centity_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081448_cfire_getmaxradius_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_081462_cfiremanager_extinguisharea_rage_vec3v_const_ref_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_081464_cfiremanager_extinguishentityfires_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081466_cfiremanager_getactivefire_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_081501_cmoviemgr_enddraw_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081545_puddlepass_setshadervars_rage_grmshader_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081546_ripples_addplayerripple_rage_vec3v_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_081547_ripples_drawripples_rage_grmshader_ptr_rage_vec4v_const_ptr_ripp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_081548_puddlepass_addplayerripple_rage_vec3v_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_081576_ctvplaylistmanager_scaletorealtime_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_081630_clodlightmanager_generatelodlighthash_clightentity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_081651_cptfxmanager_removeptfxfromentity_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081656_cptfxsortedmanager_addtorenderlist_unsigned_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_081716_cptfxmanager_clearptfxdepthalphamap(ph: *mut Phantom); }
extern "C" { pub fn f_081717_cptfxmanager_setglobalconstants(ph: *mut Phantom); }
extern "C" { pub fn f_081733_cvehicleglasscomponent_rendercomponent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081734_cvehicleglasscomponent_rendercomponentdebugtriangle_rage_vec3v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_081740_isentitysmashable_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_081762_cvfxblood_triggerptfxfalltodeath_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081876_cvfxwater_processvfxsplashped_cped_ptr_int_vfxwatersampledata_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_081877_cvfxwater_processvfxsplashvehicle_cvehicle_ptr_int_vfxwatersampl(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_081878_cvfxwater_processvfxsplashobject_cobject_ptr_int_vfxwatersampled(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32); }
extern "C" { pub fn f_081883_cvfxwater_processvfxboatbowrow_cvehicle_ptr_cvfxvehicleinfo_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_081884_cvfxwater_triggerptfxboatentry_cvehicle_ptr_cvfxvehicleinfo_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082028_cexplosionmanager_addexplosion_cexplosionmanager_cexplosionargs(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082064_cpedweaponmanager_getequippedvehicleweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082065_cpedweaponmanager_getequippedweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082070_cweaponinfo_getammoinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082071_cpedweaponmanager_getequippedweapon_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082087_cexplosioninfomanager_getexplosiontagdata_eexplosiontag(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082110_cexplosionmanager_addwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_082113_cexplosionmanager_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082122_cexplosionmanager_removeexplosionsinarea_eexplosiontag_rage_vect(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_082123_cweapondamage_generatepeddamageevent_centity_ptr_cped_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64, a5: i64, a6: i64, a7: i32, a8: f32, a9: i64, a10: i32, a11: f32, a12: i32) -> f32; }
extern "C" { pub fn f_082140_cweapondebug_initbank(ph: *mut Phantom); }
extern "C" { pub fn f_082144_cfiringpattern_setfiringpattern_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082158_cpedequippedweapon_createprojectileordnance_cobject_ptr_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_082162_cweapon_updateshadervariables_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082175_cweaponinfo_getappropriateweaponclipsetid_cped_const_ptr_fpsstre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_082179_cpedweaponmanager_destroyequippedweaponobject_bool_char_const_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_082184_cweapon_calcfirevecfromaimcamera_centity_const_ptr_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_082188_cpedequippedweapon_getprojectileordnance_cobject_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082196_cweaponanimationsmanager_getweaponclipsethashinjured_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082215_cpedweaponmanager_dropweapon_unsigned_int_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_082228_cweapondebug_clearallprojectiles(ph: *mut Phantom); }
extern "C" { pub fn f_082252_cpedweaponmanager_getbestweaponinfo_bool_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_082253_weaponfactory_create_unsigned_int_int_cdynamicentity_ptr_char_co(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_082518_cweaponinfo_getammomax_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082522_cweaponinfo_getattachpoint_cweaponcomponentinfo_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082553_cpedinventory_removeall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082554_cpedinventoryloadoutmanager_setdefaultloadout_cped_ptr_cpedmodel(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082555_cpedinventoryloadoutmanager_setloadout_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082565_cpedequippedweapon_setupasweapon_cobject_ptr_cweaponinfo_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_082568_cpedequippedweapon_createweaponcomponent_unsigned_int_cobject_pt(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082572_cpedinventory_addweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_082574_cweaponitem_dtorcweaponitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082575_cweaponitem_cweaponitem_unsigned_int_unsigned_int_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_082577_cpedinventory_addammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082580_cpedinventory_addweaponandammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_082594_cpedweaponmanager_equipweapon_unsigned_int_int_bool_bool_cpedequ(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_082597_cpedweaponselector_getisweaponvalid_cped_const_ptr_cweaponinfo_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_082603_cweaponitem_settintindex_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082604_cweaponitem_addcomponent_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082632_cpedweaponmanager_destroyequippedgadgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082646_cpedweaponmanager_getbestweaponhash_unsigned_int_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_082647_cpedweaponmanager_getbestweaponinfobygroup_unsigned_int_bool_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_082648_cpedweaponmanager_getequippedturretweaponinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082652_cpedweaponmanager_getisarmedmelee_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082663_cammoitemrepository_getammo_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082671_cweaponitem_getkeyfromhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082909_cnetworktelemetry_appendmetric_rage_rlmetric_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082912_networkinterface_isinsession(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082914_cnetwork_isconcealedplayerturn(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082957_networkutils_getscreencoordinatesforohd_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32) -> i32; }
extern "C" { pub fn f_082999_networkinterface_registerobject_rage_netgameobjectwrapper_cdynam(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083008_networkinterface_iscloudavailable_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083013_networkinterface_getactivegamerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083023_networkinterface_isaspectatingplayer_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083025_networkinterface_getphysicalplayerfromindex_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083028_networkinterface_isinspectatormode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083034_networkinterface_isfriendlyfireallowed_cped_const_ptr_centity_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083036_networkinterface_ispedallowedtodamageped_cped_const_ref_cped_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083038_networkinterface_isinfreemode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083039_networkinterface_isinarcademode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083040_networkinterface_canregisterlocalambientpeds_unsigned_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083041_networkinterface_canregisterobject_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083042_networkutils_isclosetoanyplayer_rage_vector3_const_ref_float_rag(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_083043_networkinterface_getplayerfocusposition_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_083046_networkinterface_isvisibleorclosetoanyremoteplayer_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083047_networkutils_isvisibletoanyremoteplayer_rage_vector3_const_ref_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i32; }
extern "C" { pub fn f_083049_networkutils_getclosestremoteplayers_rage_vector3_const_ref_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_083054_networkinterface_getactiveplayerfromindex_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083061_networkinterface_getnetworkobject_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083063_networkinterface_unregisterobject_rage_netgameobjectwrapper_cdyn(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083072_networkinterface_setprocesslocalpopulationlimits_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083073_networkinterface_clearusingextendedpopulationrange(ph: *mut Phantom); }
extern "C" { pub fn f_083108_networkinterface_isposlocallycontrolled_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083116_networkutils_getvehiclefromnetworkobject_rage_netobject_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083136_networkinterface_isinsingleplayerprivatesession(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083147_networkinterface_cansetwaypoint_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083154_networkinterface_getlowestpedmodelstartoffset_unsigned_int_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083155_networkinterface_getlowestvehiclemodelstartoffset_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083156_networkinterface_isremoteplayerstandingonobject_cped_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083162_networkinterface_isdamagedisabledinmp_centity_const_ref_centity(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083167_networkinterface_areinteractionsdisabledinmp_centity_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083169_networkinterface_registerdisabledcollisioninmp_centity_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083191_cnetworkbandwidthmanager_displaydebuginfo(ph: *mut Phantom); }
extern "C" { pub fn f_083196_networkdebug_displaysynccloneinfo_rage_netsynctree_ptr_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_083266_csyncedped_getped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083312_networkcrewemblemmgr_requestemblem_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083316_networkcrewemblemmgr_releaseemblem_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083332_clivemanager_checkusercontentprivileges_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083333_networkcrewemblemmgr_getemblemtxdslot_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083334_networkcrewemblemmgr_getemblemname_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083342_networkclan_getcrewemblemmgr(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083391_ccloudmanager_requestpostmemberfile_int_char_const_ptr_void_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083496_ctunables_tryaccess_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083680_networkutils_getpedfromnetworkobject_rage_netobject_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083770_networkcolours_getnetworkcolour_networkcolours_networkcolour(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083772_networkcolours_getteamcolour_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083773_networkcolours_getcustomteamcolour_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083831_serialiseanimation_rage_csyncdatabase_ref_rage_fwmvtemplateid_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_083832_serialiseanimation_rage_csyncdatabase_ref_int_ref_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_083846_serialisetaskstate_rage_csyncdatabase_ref_int_int_ref_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_083849_serialisehash_rage_csyncdatabase_ref_unsigned_int_ref_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_083857_csyncedped_getpedid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083859_csyncedped_setpedid_unsigned_short_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083898_csyncedentity_csyncedentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083899_csyncedentity_csyncedentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083900_csyncedentity_setentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083902_csyncedentity_getentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083903_csyncedentity_getentityid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083904_csyncedentity_setentityid_unsigned_short_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083905_csyncedped_csyncedped(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083906_csyncedped_csyncedped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083907_csyncedvehicle_csyncedvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083917_networkutils_setfontsettingsfornetworkohd_ctextlayout_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_084088_clivemanager_issystemuishowing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084449_networkclan_getprimaryclan_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_084629_cloudlistener_dtorcloudlistener_138661(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085570_cnetworksession_ishost_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086743_cnetworkobjectpopulationmgr_canregisterlocalobjectsoftype_unsign(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_086744_cnetgameplayer_getplayerped_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086746_cnetworkobjectmgr_registergameobject_rage_netgameobjectbase_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_086763_cnetobjgame_getplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086771_cnetobjphysical_cnetobjphysical_cphysical_ptr_unsigned_short_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_086772_cnetobjgame_cnetobjgame_cpickupplacement_cpickupplacement_ptr_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_086781_cnetworkobjectpopulationmgr_getnumlocalobjects_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_086786_cnetworkobjectmgr_getplayersnearbyinscope_unsigned_int_ref_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_086788_cnetworkobjectpopulationmgr_getnumexternallyrequiredentities_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_086789_cnetworkobjectpopulationmgr_getnumremoteobjects_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_086795_cnetworkobjectpopulationmgr_setcurrentexternalreservationindex_u(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_086796_cnetworkobjectpopulationmgr_clearcurrentreservationindex(ph: *mut Phantom); }
extern "C" { pub fn f_086915_cnetobjentity_setoverridingremotevisibility_bool_bool_char_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_086958_cnetgameplayer_getplayerinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087003_cnetobjobject_isambientobjecttype_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_087135_cnetobjped_updatelocaltaskslotcache(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087293_cnetobjped_getistargettablebyplayer_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_087476_cnetobjpickupplacement_setcollector_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087757_cnetobjvehicle_issuperdummystilllaunchedintoair_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_088229_croamingbubble_displaydebugtext(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_089334_cgivecontrolevent_trigger_rage_netplayer_const_ref_rage_netobjec(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_089376_crequestpickupevent_trigger_cped_const_ptr_cpickup_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090710_asinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090714_atanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090755_exp2f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090757_expf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090804_log10(ph: *mut Phantom, a0: f64) -> f64; }
extern "C" { pub fn f_090805_log10f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090807_logf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090826_powf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090829_iprintf(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090900_puts(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090901_qsort_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_090911_rand(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090918_sinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090920_snprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090921_siprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090922_small_sprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090923_sscanf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090926_strcasecmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090927_strcat(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090928_strchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090930_strcmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090931_strcpy(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090939_strlen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090942_strncmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090945_strrchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090947_strstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090956_strtok(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090962_system(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090965_tanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_091002_emscripten_builtin_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091023_multi3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_091035_void_std_2_sort_std_2_less_unsigned_int_unsigned_int_ref_unsigne(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_091063_op_new_impl_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_091069_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_091070_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
