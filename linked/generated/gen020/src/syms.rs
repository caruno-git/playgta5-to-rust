//! Callee declarations for crate gen020.
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
extern "C" { pub fn f_000119_rage_sysipcsignalsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000120_rage_sysipccreatemutex(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_000122_rage_sysipcunlockmutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000123_rage_sysipcdeletemutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000128_rage_sysipccreatethread_void_ptr_void_ptr_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_000131_rage_sysipcwaitthreadexit_rage_sysipcthreadid_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000135_rage_sysipcsleep_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000184_rage_netsocket_nativeshutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000185_rage_netsocket_nativebind(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000197_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom); }
extern "C" { pub fn f_000393_wasm_null_d3d_nullresource_wasm_null_d3d_stub_id3d11texture3d_d3(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000566_wasm_start_mode_now(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000599_rage_voicechat_setcapturedevicebyid32_unsigned_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000600_rage_voicechat_freecapturedevice(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000601_rage_voicechat_setplaybackdevicebyid32_unsigned_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000602_rage_voicechat_freeplaybackdevice(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000603_rage_voicechat_setcapturedevicevolume_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_000604_rage_voicechat_setplaybackdevicevolume_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_000605_rage_voicechat_setcapturemode_rage_voicechat_capturemode(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000608_rage_voicechat_nativeshutdown(ph: *mut Phantom); }
extern "C" { pub fn f_000644_rage_athashnextsize_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000646_rage_athash64_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000652_rage_atstring_reserve_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000654_rage_atstring_set_char_const_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_000656_rage_atstring_uppercase(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000658_rage_atstring_startswith_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000659_rage_atstring_indexof_char_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_000663_rage_atstring_replace_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000664_rage_atstring_split_rage_atstring_ref_rage_atstring_ref_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000665_rage_atstring_op_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000668_rage_atarray_rage_atstring_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_000675_rage_functorbase_functorbase_void_const_ptr_void_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_000680_rage_athashstringnamespacesupport_computehash_rage_athashstringn(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_000681_rage_athashstringnamespacesupport_addstring_rage_athashstringnam(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_000684_rage_athashstringnamespacesupport_trygetstring_rage_athashstring(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_000692_rage_datcallback_call_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000696_rage_bkwidget_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000703_rage_bkbank_unsetcurrentgroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000704_rage_bkbank_pushgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000705_rage_bkgroup_addgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000706_rage_bkbank_deletegroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000707_rage_bkbank_popgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000779_rage_bkmanager_createbank_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000782_rage_bkmanager_findbank_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000783_rage_bkmanager_findwidget_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000785_rage_bkmanager_destroybank_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000786_rage_bkmanager_isbankthread(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000796_rage_bkwidget_findchild_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000797_rage_bkwidget_setfillcolor_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000806_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000828_rage_bklist_addcolumnheader_int_char_const_ptr_rage_bklist_itemt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_000829_rage_bklist_additem_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_000830_rage_bklist_additem_int_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_000831_rage_bklist_removeitem_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000851_char_ptr_rage_formatf_char_8ul_char_ref_8ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000866_rage_bkgroup_addseparator_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000867_rage_bkgroup_addtitle_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000869_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000870_rage_bkgroup_addbutton_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000871_rage_bkgroup_addtext_char_const_ptr_bool_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000872_rage_bkgroup_addtext_char_const_ptr_float_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000873_rage_bkgroup_addtext_char_const_ptr_int_ptr_bool_rage_datcallbac(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_000874_rage_bkgroup_addtext_char_const_ptr_int_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000875_rage_bkgroup_addtext_char_const_ptr_long_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000876_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_000877_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_000882_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000897_rage_bkgroup_addslider_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000898_rage_bkgroup_addslider_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000902_rage_bkgroup_addslider_char_const_ptr_unsigned_short_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000906_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000907_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int_rage_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000910_rage_bkgroup_addslider_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64); }
extern "C" { pub fn f_000918_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000919_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000921_rage_bkgroup_addlist_char_const_ptr_bool_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000942_rage_bkgroup_addcolor_char_const_ptr_rage_color32_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001049_rage_matrix33_rotate_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_001098_rage_autoidinit(ph: *mut Phantom); }
extern "C" { pub fn f_001105_rage_autoidregistry_autoidregistry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001108_rage_autoidregistry_unregister_rage_autoiddescriptor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001109_rage_autoidregistry_getnamefromid_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001112_rage_datbase64_getmaxencodedsize_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_001114_rage_datbase64_encode_unsigned_char_const_ptr_unsigned_int_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_001115_rage_datbase64_decode_char_const_ptr_unsigned_int_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_001116_char_ptr_rage_formatf_char_32ul_char_ref_32ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001117_rage_datcallback_datcallback(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001119_rage_datcallback_datcallback_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001120_rage_datcallback_datcallback_void_ptr_void_ptr_void_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_001123_rage_datgrowbuffer_datgrowbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001124_rage_datgrowbuffer_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001125_rage_datgrowbuffer_dtordatgrowbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001126_rage_datgrowbuffer_init_rage_sysmemallocator_ptr_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001127_rage_datgrowbuffer_init_void_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001129_rage_datgrowbuffer_getfihandle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001130_rage_datgrowbuffer_append_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001131_rage_datgrowbuffer_preallocate_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001132_rage_datgrowbuffer_appendorfail_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001134_rage_datgrowbuffer_truncate_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001138_rage_datgrowbuffer_getbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001140_rage_datbitbuffer_readunsigned_void_const_ptr_unsigned_int_ref_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001141_rage_datbitbuffer_writeunsigned_void_ptr_unsigned_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_001143_rage_datbitbuffer_datbitbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001145_rage_datbitbuffer_setreadwritebits_void_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001147_rage_datbitbuffer_setreadonlybytes_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001148_rage_datbitbuffer_setreadwritebytes_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001149_rage_datbitbuffer_getnumbyteswritten_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001151_rage_datbitbuffer_setcursorpos_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001152_rage_datbitbuffer_getnumbytesread_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001153_rage_datbitbuffer_setnumbitsread_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001154_rage_datbitbuffer_skipbytes_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001156_rage_datbitbuffer_bumpreadcursor_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001158_rage_datbitbuffer_bumpreadwritecursor_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001161_rage_datbitbuffer_readuns_unsigned_long_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001163_rage_datbitbuffer_writeuns_unsigned_long_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001164_rage_datbitbuffer_readfloat_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001165_rage_datbitbuffer_writefloat_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_001166_rage_datbitbuffer_readbool_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001167_rage_datbitbuffer_writebool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001168_rage_datbitbuffer_readbytes_void_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001169_rage_datbitbuffer_writebytes_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001170_rage_datbitbuffer_readstr_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001171_rage_datbitbuffer_writestr_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001172_rage_datbitbuffer_peekbytes_void_ptr_int_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_001190_rage_aes_aes_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001193_rage_aes_encrypt_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001195_rage_aes_decrypt_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001197_rage_sha1_sha1(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001199_rage_sha1_final_unsigned_char_ref_20(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001201_rage_sha1_update_unsigned_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001204_rage_rsonwriter_rsonwriter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001205_rage_rsonwriter_init_char_ptr_unsigned_int_rage_rsonformat(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001206_rage_rsonwriter_rsonwriter_char_ptr_unsigned_int_rage_rsonformat(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001208_rage_rsonwriter_initnull_rage_rsonformat(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001210_rage_rsonwriter_begin_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001213_rage_rsonwriter_beginarray_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001214_rage_rsonwriter_end(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001216_rage_rsonwriter_writeint_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001218_rage_rsonwriter_writeint64_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001220_rage_rsonwriter_writeuns_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001221_rage_rsonwriter_writeuns64_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001222_rage_rsonwriter_writefloat_char_const_ptr_float_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_001224_rage_rsonwriter_writestring_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001227_rage_rsonwriter_writebinary_char_const_ptr_void_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001228_rage_rsonwriter_available_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001229_rage_rsonwriter_writebool_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001230_bool_rage_rsonwriter_writevalue_float_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_001232_rage_rsonreader_rsonreader(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001233_rage_rsonreader_rsonreader_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001234_rage_rsonreader_init_char_const_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001237_rage_rsonreader_hasmember_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001238_rage_rsonreader_getmember_char_const_ptr_rage_rsonreader_ptr_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001239_rage_rsonreader_getnextsibling_rage_rsonreader_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001240_rage_rsonreader_getfirstmember_rage_rsonreader_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001241_rage_rsonreader_hasduplicateentries_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001242_rage_rsonreader_hasname_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001243_rage_rsonreader_getname_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001245_rage_rsonreader_getvalue_char_const_ptr_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_001246_rage_rsonreader_asint_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001248_rage_rsonreader_asuns_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001249_rage_rsonreader_asuns64_unsigned_long_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001250_rage_rsonreader_asfloat_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001251_rage_rsonreader_asbool_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001252_rage_rsonreader_asatstring_rage_atstring_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001253_rage_rsonreader_asstring_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001257_rage_rsonreader_readint_char_const_ptr_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001258_rage_rsonreader_readint64_char_const_ptr_long_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001259_rage_rsonreader_readuns_char_const_ptr_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001260_rage_rsonreader_readuns64_char_const_ptr_unsigned_long_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001261_rage_rsonreader_readfloat_char_const_ptr_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001262_rage_rsonreader_readbinary_char_const_ptr_void_ptr_int_int_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_001263_rage_rsonreader_readbool_char_const_ptr_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001264_rage_rsonreader_readstring_char_const_ptr_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_001267_rage_rsonreader_checkname_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001268_rage_rsonreader_getrawvaluestring_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001269_rage_rsonreader_validatejson_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001273_rage_rsonreader_getrawmembervalue_char_const_ptr_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001274_rage_datsaxreader_datsaxreader(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001514_rage_diagdisplayf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001516_rage_fistream_write_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001523_quitf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001524_displayf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001527_warningf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001529_rage_fistream_create_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001530_rage_fistream_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001531_rage_fistream_size64(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001532_rage_fistream_close(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001533_rage_fidevice_getdevice_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001534_quitf_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001535_rage_diaglogf2_rage_diagchannel_const_ref_rage_diagseverity_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001537_rage_fistream_size(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001541_rage_diagloggedprintln_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001544_rage_diagdefaultprintstacktrace(ph: *mut Phantom); }
extern "C" { pub fn f_001549_rage_diaglogdefault_rage_diagchannel_const_ref_rage_diagseverity(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001555_rage_diaglogf_assert_rage_diagchannel_const_ref_char_const_ptr_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001560_rage_diagcontextmessage_diagcontextmessage_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001563_rage_diagcontextmessage_pop(ph: *mut Phantom); }
extern "C" { pub fn f_001565_printf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001568_rage_atmapentry_unsigned_int_wchar_t_ptr_const_ref_rage_atmap_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001577_rage_fistream_open_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001578_rage_fistream_seek_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001580_rage_fiassetmanager_open_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001582_rage_fiassetmanager_createleadingpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001605_rage_fidevice_makegrowbufferfilename_char_ptr_int_rage_datgrowbu(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001622_rage_fistream_read_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001627_rage_fiassetmanager_exists_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001635_rage_fiassetmanager_filename_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001646_rage_fistream_seek64_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001647_rage_fistream_tell(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001650_rage_fgetline_char_ptr_int_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_001652_rage_fidevice_makememoryfilename_char_ptr_int_void_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_001653_shutdownwinsock(ph: *mut Phantom); }
extern "C" { pub fn f_001737_rage_ficachepartition_init_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_001738_rage_fidevice_deletedirectory_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001739_rage_ficacheutils_listfiles_char_const_ptr_void_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001767_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_001798_rage_fidevicerelative_getdebugname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001874_void_rage_atplaceptrcreator_void_ptr_rage_fideviceinstaller_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002018_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002019_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002020_void_std_2_sort_heap_abi_nqn220108_std_2_classicalgpolicy_bool_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002021_rage_atbinarymap_int_rage_atfinalhashstring_datapair_ptr_std_2_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002022_std_2_pair_rage_atbinarymap_int_rage_atfinalhashstring_datapair(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002023_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002027_rage_invsqrtf_fast_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_002031_rage_mthrandom_mthrandom_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002091_rage_datresource_containsthisaddress_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002092_rage_datresource_getfixup_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002105_rage_pgbase_dtorpgbase_3250(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002111_rage_parmanager_parmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002112_rage_parmanager_initialize_rage_parsettings_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002128_rage_parmanager_uninitialize(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002129_rage_atmapentry_rage_atnonfinalhashstring_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002131_rage_atnonfinalhashstring_atnonfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002132_rage_parmanager_dtorparmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002147_rage_parmanager_loadtree_char_const_ptr_char_const_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002153_rage_parmanager_openinputstream_char_const_ptr_char_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002156_rage_parmanager_initfromstructure_rage_parstructure_ref_rage_par(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002158_rage_parinstancevisitor_visittoplevelstructure_rage_parparsables(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002161_rage_parstructure_issubclassof_rage_parstructure_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_002174_rage_parelement_findattributestringvalue_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_002175_rage_partree_dtorpartree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002182_rage_parelement_findattribute_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002183_rage_partreenode_findchildwithindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002184_rage_partreenode_findchildwithname_char_const_ptr_rage_partreeno(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002203_rage_parelement_findattributeboolvalue_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002205_rage_parelement_findattributedoublevalue_char_const_ptr_double_b(ph: *mut Phantom, a0: i64, a1: i64, a2: f64, a3: i32) -> f64; }
extern "C" { pub fn f_002206_rage_parelement_findattributeint64value_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002215_rage_partreenode_findnumchildren_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002257_rage_parmemberstring_getsubtype_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002278_rage_psofile_dtorpsofile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002282_rage_psoloadfile_char_const_ptr_rage_psoloadaction_rage_atfixedb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002288_rage_psoloadfromstructure_rage_psofile_ref_rage_parstructure_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002298_comparesourceaddr_rage_psorscbuilderinstance_ptr_rage_psorscbuil(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002311_rage_atarray_rage_psoenumtableentry_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002343_rage_parrestregistersingleton_char_const_ptr_rage_parparsablestr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
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
extern "C" { pub fn f_002582_rage_parelement_findattributeanycase_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002593_rage_parsettings_createstandardsettings(ph: *mut Phantom); }
extern "C" { pub fn f_002648_rage_parstream_close(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002720_rage_conststring_vcopy_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002721_rage_conststring_dtorconststring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002725_rage_atstringbuilder_atstringbuilder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002727_rage_atstringbuilder_atstringbuilder_char_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002728_rage_atstringbuilder_dtoratstringbuilder(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002731_rage_atstringbuilder_append_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002732_rage_atstringbuilder_append_char(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002734_rage_atstringbuilder_appendorfail_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002735_rage_atstringbuilder_appendorfail_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002736_rage_atstringbuilder_tostring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002739_rage_atpartialstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002743_rage_atdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002744_rage_atdatahash_unsigned_int_const_ptr_unsigned_long_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002745_rage_safestrtok_char_ptr_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002751_rage_stringstartswithi_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002753_rage_stringisequal_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002754_rage_stringnullorempty_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002757_rage_countutf8characters_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002763_rage_utf8removeinvalidsurrogates_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002767_deflateinit2(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_002770_deflateend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002773_deflate(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002777_inflateinit2(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002779_inflate(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002781_inflateend(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002782_lz4_compressbound(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_002783_lz4_compress(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002784_lz4_decompress_fast(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002790_rage_zlibinflater_zlibinflater(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002798_rage_sysappcontent_isamericanbuild(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002799_rage_sysappcontent_isjapanesebuild(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002806_rage_sysservice_adddelegate_rage_atdelegator1_void_rage_sysservi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002808_rage_atdelegator1_void_rage_sysserviceevent_ptr_removedelegate_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002812_rage_sysnamedpipe_sysnamedpipe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002839_rage_sysparam_get_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002840_rage_sysparam_get_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002841_rage_sysparam_getarray_char_const_ptr_ptr_int_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_002842_rage_sysparam_getarray_int_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002844_rage_sysparam_getparambyname_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002845_rage_sysparam_searchargarray_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002849_rage_sysbootmanager_isbootedfromdisc(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002857_rage_systimer_haselapsedintervalms_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002871_rage_sysgetenv_char_const_ptr_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002890_rage_systhreadpool_workitem_pending_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002895_rage_systhreadpool_workitem_wascanceled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002899_rage_systhreadpool_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002901_rage_systhreadpool_init(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002902_rage_systhreadpool_addthread_rage_systhreadpool_thread_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_002917_rage_atdelegator1_void_rage_sysserviceevent_ptr_dispatch_rage_sy(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002918_rage_sysservice_triggerevent_rage_sysserviceevent_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002921_rage_sysservice_getsystemlanguage(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_002939_rage_sysmemmanager_sysmemmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002945_rage_sysmempoolallocator_safeallocate_unsigned_long_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002950_rage_sysmemsimpleallocator_sysmemsimpleallocator_void_ptr_int_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_002952_rage_sysmemsimpleallocator_dtorsysmemsimpleallocator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002954_rage_sysmemsimpleallocator_setquitonfail_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002955_rage_sysmemsimpleallocator_allocate_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002961_rage_sysmemsimpleallocator_free_void_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003014_rage_sysmemfixedallocator_dumpstats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003078_rage_systinyheap_free_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003079_rage_sysmemsparseallocator_sysmemsparseallocator_unsigned_long_i(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003097_char_ptr_rage_formatf_char_16ul_char_ref_16ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003124_rage_sysmembuddyallocator_getheap_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003138_rage_geompoints_findclosestpointpointboxsurface_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_003147_rage_geomspheres_testspheretoseg_rage_vec3v_const_ref_rage_scala(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_003151_rage_geomboxes_testsegmenttobox_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_003159_rage_matrix34_fromquaternion_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003160_rage_matrix34_toquaternion_rage_quaternion_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003178_rage_vector3_reflectabout_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003179_rage_atpoolbase_init_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003181_rage_atpoolbase_init_unsigned_char_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003182_rage_atpoolbase_releasemem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003186_rage_crframe_crframe_rage_crframedata_const_ref_unsigned_char_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_003215_rage_crframe_setvector3_unsigned_char_unsigned_short_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_003296_rage_atarray_unsigned_short_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003798_rage_fwmvrequestid_op_rage_fwmvstateid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003800_rage_fwanimdirector_generatecomponentindices(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003806_rage_fwanimdirector_waitforanyactiveupdatetocomplete_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003829_rage_fwanimdirectorcomponentsyncedscene_issyncedscenelooped_shor(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003830_rage_crtagiteratorbase_rage_crtagiteratorkey_true_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003832_rage_fwanimdirectorcomponentsyncedscene_isvalidsceneid_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003835_rage_fwanimdirector_getmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003838_rage_fwclipset_getclip_rage_fwmvtemplateid_rage_fwmvclipidtype_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_003840_rage_fwclipsetmanager_getclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003848_rage_fwanimdirector_isnetworkdefloaded_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003855_rage_fwanimdirector_getframebuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003869_rage_fwanimdirectorcomponentsyncedscene_resizesyncedscenepool_in(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_003876_rage_fwanimmanager_getclipifexistsbysetid_rage_fwmvtemplateid_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003896_rage_fwanimmanager_getclipifexistsbydictindex_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_003905_rage_fwanimmanager_findslotfromhashkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003977_rage_fwclipset_request_deprecated_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_003984_rage_fwclipsetrequesthelper_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004019_rage_fwclipsetrequesthelper_request_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_004037_rage_fwclipsetrequesthelper_dtorfwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004038_rage_fwclipsetrequesthelper_fwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004040_rage_fwclipsetrequesthelper_getclipset_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004041_rage_fwclipsetrequesthelper_isloaded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004208_rage_parstructure_ptr_rage_parcgugetstructurecb_rage_fwmovedumpd(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004280_rage_fwposematcher_fwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004387_rage_fwanimdirectorcomponentsyncedscene_stopsyncedsceneplayback(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_004390_rage_fwanimdirectorcomponentsyncedscene_decsyncedsceneref_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004392_rage_fwanimdirectorcomponentsyncedscene_addsyncedsceneref_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004395_rage_fwanimdirectorcomponentsyncedscene_calcentitymatrix_short_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: f32); }
extern "C" { pub fn f_004397_rage_fwanimdirectorcomponentsyncedscene_setsyncedscenephase_shor(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_004398_rage_fwanimdirectorcomponentsyncedscene_getsyncedscenephase_shor(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004399_rage_fwanimdirectorcomponentsyncedscene_setsyncedscenerate_short(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_004401_rage_fwanimdirectorcomponentsyncedscene_setsyncedsceneorigin_sho(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_004403_rage_fwanimdirectorcomponentsyncedscene_getsyncedsceneorigin_sho(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_004406_rage_fwanimdirectorcomponentsyncedscene_setsyncedscenelooped_sho(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_004407_rage_fwanimdirectorcomponentsyncedscene_issyncedsceneholdlastfra(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004408_rage_fwanimdirectorcomponentsyncedscene_setsyncedsceneholdlastfr(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_004411_rage_fwanimdirectorcomponentsyncedscene_attachsyncedscene_short(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_004416_rage_fwanimdirectorcomponentsyncedscene_startsynchronizedscene_r(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004603_rage_fwextensionlist_add_rage_fwextension_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004619_rage_fwarchetype_getdrawable_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004620_rage_fwarchetype_getfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004630_rage_fwarchetypemanager_reset(ph: *mut Phantom); }
extern "C" { pub fn f_004631_rage_fwarchetypemanager_getarchetypename_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004632_rage_fwarchetypemanager_lookupmodelid_rage_fwarchetype_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004633_rage_fwarchetypemanager_getarchetype_rage_atnonfinalhashstring_u(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004634_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004661_rage_fwdynamicentitycomponent_fwdynamicentitycomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004676_rage_fwentity_clearcurrentphysicsinst(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004681_rage_fwentity_waitforanyactiveanimupdatetocomplete_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004693_rage_fwsceneupdate_isinsceneupdate_rage_fwentity_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004737_rage_fwdecorator_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004748_rage_fwdecoratorextension_find_rage_atnonfinalhashstring_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004767_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004788_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004792_rage_fwpickermanager_getselectedentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004949_rage_dldrawlistmgr_getdrawlistname_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004992_rage_dldrawlistmgr_getdrawlistentitycount_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005032_rage_fwlanguagepack_getdateformattype_rage_syslanguage(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005034_rage_fwlanguagepack_hasdaydelimiter_rage_syslanguage(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005035_rage_fwlanguagepack_getmonthstring_char_ref_16_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_005053_rage_fwtextutil_getbytecount_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005055_rage_fwtextutil_strcpywithcharacterandbytelimits_char_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_005093_rage_clothinstance_gettype_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005196_rage_ropeinstance_setphysinstflags_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005209_rage_ropeinstance_setnewuniqueid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005213_rage_ropemanager_remove_rage_ropeinstance_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005223_rage_ropemanager_removerope_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005225_rage_ropemanager_add_rage_vec3v_const_ref_rage_vec3v_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32, a11: f32, a12: i32, a13: i32) -> i64; }
extern "C" { pub fn f_005226_rage_ropemanager_breakrope_rage_ropeinstance_ptr_rage_ropeinstan(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32); }
extern "C" { pub fn f_005315_rage_fwrenderthreadinterface_flush_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005461_rage_fwboxstreamer_hasloadedaboutpos_rage_vec3v_const_ref_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_005486_rage_fwclipdictionarystore_addref_rage_strlocalindex_rage_strref(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005488_rage_fwclipdictionarystore_removeref_rage_strlocalindex_rage_str(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005561_sprintf_s_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005601_rage_fwmapdatastore_removeunrequired_bool(ph: *mut Phantom); }
extern "C" { pub fn f_005623_rage_fwmaptypesstore_removeunrequired(ph: *mut Phantom); }
extern "C" { pub fn f_005679_rage_fwstaticboundsstore_removeall(ph: *mut Phantom); }
extern "C" { pub fn f_005714_rage_fwtxdstore_removeref_rage_strlocalindex_rage_strrefkind(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005730_rage_fwinteriorlocation_makeinvalid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005732_rage_fwinteriorlocation_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005917_rage_fwrefawarebaseimpl_rage_datbase_dtorfwrefawarebaseimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005947_rage_aitasktree_getactiveleaftask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005949_rage_aitasktree_findtaskbytypeactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006084_rage_strstreaminginfomanager_requestobject_rage_strindex_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006162_rage_strstreaminginfo_gethandle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006175_rage_strstreaminginfomanager_clearrequiredflag_rage_strindex_uns(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006238_rage_strstreamingloadermanager_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006254_rage_strrequest_request_rage_strlocalindex_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_006255_rage_strrequest_hasloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006256_rage_strrequest_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006257_rage_strrequest_clearrequiredflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006262_rage_strrequest_getname(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006610_rage_audsound_decompressmetadata_untyped_rage_sound_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006715_rage_audsound_prepareandplay_rage_audwaveslot_ptr_bool_int_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_006717_rage_audsound_stopandforget_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006881_rage_audmetadatamanager_getobjectmetadatareffromhash_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006886_rage_audmetadatamanager_getobjectnamefromnametableoffset_unsigne(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006903_rage_audcategorymanager_getcategoryptr_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006941_rage_audcurve_calculatevalue_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_006996_rage_audenvironmentgroupinterface_baseinit_rage_audentity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007036_rage_audmetadatamanager_getobject_verify_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007047_rage_audsoundfactory_getmetadataptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007076_rage_audsoundpool_debugprintsoundpool_const(ph: *mut Phantom); }
extern "C" { pub fn f_007078_rage_audsoundset_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007079_rage_audsoundset_find_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007081_rage_audsoundsetfieldref_resolvemetadataref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007404_rage_audentity_createsound_localreference_unsigned_int_rage_auds(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007406_rage_audentity_createsound_localreference_rage_audmetadataref_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007410_rage_audentity_createandplaysound_char_const_ptr_rage_audsoundin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007411_rage_audentity_addbatchedsoundrequest_rage_audmetadataref_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007412_rage_audentity_createandplaysound_unsigned_int_rage_audsoundinit(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007419_rage_vfxlistitem_vfxlistitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007426_rage_vfxbrokenfraginfo_dtorvfxbrokenfraginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007528_rage_ptfxattach_attach_rage_ptxeffectinst_ptr_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_007559_rage_ptfxmanager_gettriggeredinst_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007562_rage_ptfxmanager_getregisteredinst_void_const_ptr_int_bool_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_007565_rage_ptfxreg_find_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007566_rage_ptfxreg_unregister_void_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007576_rage_ptfxscript_start_rage_atnonfinalhashstring_rage_atnonfinalh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: f32, a7: i32) -> i32; }
extern "C" { pub fn f_007579_rage_ptfxscript_destroy_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007581_rage_ptfxscript_doesexist_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_007584_rage_ptfxscript_updateevo_int_unsigned_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_007585_rage_ptfxscript_updatecolourtint_int_rage_vec3v_const_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_007638_rage_fwkeygen_appenddatatokey_unsigned_int_char_const_ptr_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_007639_rage_fwkeygen_appendstringtokey_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_007675_rage_fwangle_limitdegreeangle_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_007676_rage_fwangle_limitradianangleforpitch_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_007749_rage_neteventmgr_registernetworkevent_unsigned_short_void_ptr_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007752_rage_netinterface_ishost(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007753_rage_netinterface_getlocalplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007754_rage_netinterface_getnumactiveplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007755_rage_netinterface_getallactiveplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007756_rage_netinterface_flushalllogfiles_bool(ph: *mut Phantom); }
extern "C" { pub fn f_007757_rage_netinterface_getsynchronisationtime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007758_rage_netinterface_getnumremoteactiveplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007759_rage_netinterface_getnumphysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007760_rage_netinterface_getnumremotephysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007761_rage_netinterface_getremoteactiveplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007762_rage_netinterface_getallphysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007763_rage_netinterface_getremotephysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007764_rage_netplayermgrbase_getplayerfromgamerid_rage_rlgamerid_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_007765_rage_netinterface_getplayerfrompeerid_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007766_rage_netplayermgrbase_getplayerfrompeerid_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_007767_rage_netinterface_getphysicalplayerfromindex_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_007770_rage_netinterface_getnetworkobject_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_007771_rage_netobjectmgrbase_getnetworkobject_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007772_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_007773_rage_netobjectmgrbase_checknetworkobjectmapforduplicates_char_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007776_rage_netplayermgrbase_isinitialized_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007777_rage_netinterface_getdefaultlogfiletargettype(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007778_rage_netinterface_getdefaultlogfileblockingmode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007779_rage_netinterface_sendbuffer_rage_netplayer_const_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_007780_rage_netplayermgrbase_sendbuffer_rage_netplayer_const_ptr_void_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_007781_rage_netinterface_adddelegate_rage_atdelegator1_void_rage_receiv(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007782_rage_netplayermgrbase_adddelegate_rage_atdelegator1_void_rage_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007783_rage_netinterface_removedelegate_rage_atdelegator1_void_rage_rec(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007784_rage_netplayermgrbase_removedelegate_rage_atdelegator1_void_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007790_rage_netarraymanager_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007791_rage_netlogimpl_netlogimpl_char_const_ptr_rage_logopentype_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_007792_rage_netbandwidthstatistics_allocatebandwidthrecorder_char_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007793_rage_netarraymanager_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007795_rage_netlogsplitter_log_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007801_rage_nettimebarwrapper_starttimer_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_007802_rage_netplayer_ismyplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007803_rage_networklogutils_writelogevent_rage_netlogginginterface_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007804_rage_networklogutils_writemessageheader_rage_netlogginginterface(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_007805_rage_netarraymanager_getarrayhandler_unsigned_short_rage_netarra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_007810_rage_netarraymanager_registerarrayhandler_rage_netarrayhandlerba(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007811_rage_netarraymanager_unregisterarrayhandler_rage_netarrayhandler(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_007816_rage_netarrayhandlerbase_netarrayhandlerbase_unsigned_short_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_007817_rage_netarrayhandlerbase_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007819_rage_netarrayhandlerbase_resetsyncdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007820_rage_netarrayhandlerbase_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007821_rage_netarrayhandlerbase_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007825_rage_netarrayhandlerbase_playerhasleft_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007833_rage_netarrayhandlerbase_iselementsyncedwithplayer_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_007836_rage_netarrayhandlerbase_setelementdirty_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007838_rage_netarrayhandlerbase_handlechangeofarbitration_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007839_rage_netarrayhandlerbase_verifychecksum_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_007842_rage_netarrayhandlerbase_isvalidindex_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007844_rage_netarrayhandlerbase_resetelementsyncdata_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007846_rage_netarrayhandlerbase_calchash_char_const_ptr_unsigned_int_co(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007848_rage_neteventmgr_checkforspaceinpool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007849_rage_neteventmgr_prepareevent_rage_netgameevent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007850_rage_netscriptbroadcastdatahandlerbase_setscripthandler_rage_scr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007853_rage_netscriptbroadcastdatahandlerbase_cansendupdate_rage_netpla(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007855_rage_netplayer_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007857_rage_netscriptbroadcastdatahandlerbase_readupdate_rage_netplayer(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_007860_rage_netscriptbroadcastdatahandlerbase_setarraydata_rage_cbroadc(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_007864_rage_netplayerarrayhandler_rage_cbroadcastdataelement_rage_netsc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007870_rage_nethostbroadcastdatahandlerbase_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007871_rage_netscriptbroadcastdatahandlerbase_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007874_rage_nethostbroadcastdatahandlerbase_readupdate_rage_netplayer_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_007877_rage_netbandwidthmgr_dtornetbandwidthmgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007879_rage_netbandwidthmgr_init_rage_netconnectionmanager_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007881_rage_netbandwidthmgr_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007884_rage_netbandwidthmgr_getunreliableresendcount_rage_netplayer_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007889_rage_netbandwidthrecorderwrapper_getbandwidthinrate_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_007890_rage_netbandwidthrecorderwrapper_getbandwidthoutrate_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_007891_rage_netbandwidthrecorderwrapper_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007893_rage_netbandwidthstatistics_getbandwidthrecorder_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007894_rage_netbandwidthstatistics_getbandwidthrecorder_short_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_007895_rage_netbandwidthstatistics_getbandwidthinrate_short_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_007896_rage_netbandwidthstatistics_getbandwidthoutrate_short_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_007897_rage_netbandwidthstatistics_getbandwidthinrate_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_007898_rage_netbandwidthstatistics_getbandwidthoutrate_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_007899_rage_netbandwidthstatistics_gettotalbandwidthinrate(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_007900_rage_netbandwidthstatistics_gettotalbandwidthoutrate(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_007903_rage_netbandwidthstatistics_gethighestbandwidthin_unsigned_int_c(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_007904_rage_netbandwidthstatistics_gethighestbandwidthout_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_007905_rage_netcloudrequesthelper_cancel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007906_rage_netcloudrequesthelper_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007907_rage_netcloudrequesthelper_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007908_rage_netcloudrequesthelper_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007909_rage_netcloudrequesthelper_setnumretries_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007910_rage_netcloudrequesthelper_markascritical(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007911_rage_netcloudrequesthelper_getfileinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007916_rage_netcloudrequestgetmemberfile_netcloudrequestgetmemberfile_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i64, a8: i64) -> i64; }
extern "C" { pub fn f_007927_rage_netcloudrequestgettitlefile_netcloudrequestgettitlefile_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_007936_rage_neteventmgr_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007937_rage_neteventmgr_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007940_rage_neteventmgr_geteventnamefromtype_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007949_rage_netgameevent_netgameevent_unsigned_short_bool_bool_rage_net(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_007959_rage_netplayer_isremote_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007967_rage_netlogsplitter_writedatavalue_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007987_rage_csyncdatabase_serialiseunsigned_unsigned_char_ref_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_007989_rage_csyncdatabase_serialiseunsigned_unsigned_int_ref_int_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_007992_rage_netobjectmgrbase_getlogname_char_ptr_unsigned_long_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_007995_rage_matrix34_toeulersxyz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008002_rage_nettimebarwrapper_nettimebarwrapper_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008003_rage_nettimebarwrapper_pushtimer_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008004_rage_nettimebarwrapper_poptimer_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008005_rage_getupdatelevelstring_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_008056_rage_networklogutils_writeplayertext_rage_netlogginginterface_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_008059_char_ptr_rage_formatf_char_5ul_char_ref_5ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008061_rage_netremotelog_beginsendinternal(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008064_rage_netremotelog_updateclass(ph: *mut Phantom); }
extern "C" { pub fn f_008070_rage_netobject_getplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008074_rage_netobject_ispendingownerchange_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008075_rage_netobject_islocalflagset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008077_rage_netobject_isglobalflagset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008079_rage_netobject_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008081_rage_netobject_hasbeencloned_rage_netplayer_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008095_rage_netobject_setupdatelevel_unsigned_char_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_008103_rage_netplayer_getrlpeerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008121_rage_netobjectidmgr_iswaitingforobjectids_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008122_rage_netplayer_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008123_rage_netobjectidmgr_iswaitingforobjectids_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008124_rage_netobjectidmgr_getnewobjectid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008155_rage_netsynctree_update_rage_netsynctreetargetobject_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_008185_rage_netobjectmgrbase_getlocalobjectlists_rage_atdlist_rage_neto(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008187_rage_netobjectmgrbase_gettotalnumobjects_bool_ptr_rage_netobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008189_rage_netobjectmgrbase_getallobjects_rage_netobject_ptr_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_008190_rage_netobjectmgrbase_forallobjects_void_ptr_rage_netobject_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008195_rage_netobjectmgrbase_getaveragelatency_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_008196_rage_netobjectmgrbase_getpacketloss_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_008212_rage_netobjectreassignmgr_getreassignmentrunningtimems_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008224_rage_netblenderlininterp_getpredictedaccelerationvector_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008230_rage_netblenderlininterp_getlastpositionreceived_unsigned_int_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008231_rage_netblenderlininterp_getlastvelocityreceived_unsigned_int_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008239_rage_netblenderlininterp_getpositiondelta_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_008246_rage_netplayer_getrlgamerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008248_rage_netplayer_getnonphysicaldata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008320_rage_netsynctree_dirtynode_rage_netsynctreetargetobject_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008321_rage_netsynctree_forcesendofnodedata_unsigned_int_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_008336_netscgamerconfigparser_getleaderboard_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008338_rage_netleaderboardread_getrow_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008340_netscgamerconfigparser_getinputbyid_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008351_rage_netleaderboardwrite_addcolumn_rage_rlstatvalue_const_ref_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008353_rage_netleaderboardmanageallocator_init_rage_sysmemallocator_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008354_rage_netleaderboardmanageallocator_startusingmemallocator(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008355_rage_netleaderboardmanageallocator_stopusingmemallocator(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008356_rage_netleaderboardreadmgr_dtornetleaderboardreadmgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008360_rage_netleaderboardreadmgr_init_rage_sysmemallocator_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008361_rage_netleaderboardreadmgr_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008362_rage_netleaderboardreadmgr_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008363_rage_netleaderboardreadmgr_addreadbyrow_int_unsigned_int_rage_rl(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i64, a8: i32, a9: i64, a10: i32, a11: i64) -> i64; }
extern "C" { pub fn f_008365_rage_netleaderboardreadmgr_addreadbyrank_int_unsigned_int_rage_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_008366_rage_netleaderboardreadmgr_addreadbyradius_int_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_008368_rage_netleaderboardreadmgr_canreadstatistics_rage_rlgamerhandle(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008369_rage_netleaderboardreadmgr_countofleaderboards_unsigned_int_cons(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008372_rage_netleaderboardreadmgr_delete_rage_netleaderboardread_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008373_rage_netleaderboardwritemgr_dtornetleaderboardwritemgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008374_rage_netleaderboardwritemgr_cancel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008375_rage_netleaderboardwritemgr_deleteall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008376_rage_netleaderboardwritemgr_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008378_rage_netleaderboardwritemgr_getleaderboardcount_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008379_rage_netleaderboardwritemgr_addleaderboard_unsigned_int_rage_rll(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_008381_rage_atarray_leaderboard_0_unsigned_short_destruct_leaderboard_p(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008386_netscgamerconfigparser_getleaderboard_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_008390_rage_netscgamerdatamgr_requestinfo_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008391_rage_netscgamerdatamgr_ischeatersc_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008392_rage_netscgamerdatamgr_isbadsportsc_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008393_rage_netscgamerdatamgr_getactivexpbonussc_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008396_rage_netscgamerdatamgr_setactivegamerindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008405_rage_netscgamerdatamgr_ischeater_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_008406_rage_netscgamerdatamgr_isrockstardev_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_008412_netprofanityfilter_verifystringforprofanity_char_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_008413_netprofanityfilter_getstatusforrequest_int_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008442_rage_ccommerceconsumablemanager_setownershipdatadirty(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008474_rage_ccommerceutil_dodatarequest_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008512_rage_netarraymanager_dtornetarraymanager_13932(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008548_rage_netcloudrequesthelper_dtornetcloudrequesthelper_14007(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008598_rage_netscgamerdatamgr_dtornetscgamerdatamgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008606_bool_rage_datbitbuffer_writeint_signed_char_signed_char_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008656_rage_netsyncdata_dynamic_nobuffers_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008682_rage_fwbasepool_fwbasepool_int_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_008683_rage_fwconfigmanager_getsizeofpool_unsigned_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008686_rage_fwbasepool_getindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008688_rage_fwbasepool_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008689_rage_fwscriptguid_createguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008693_rage_fwscriptguid_getbasefromguid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_008701_rage_scripthandlermgr_writescriptheader_rage_netlogginginterface(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_008722_rage_scripthandlermgr_getscriptresource_unsigned_short_int_rage(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_008746_unsigned_int_rage_netmessage_readheader_rage_datbitbuffer_unsign(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008764_bool_rage_datbitbuffer_readint_int_int_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_008765_bool_rage_netmessage_writeheader_rage_datexportbuffer_unsigned_i(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_008799_rage_scripthandlernetcomponent_getparticipantusingslot_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008841_rage_scriptinterface_isnetworkscript_rage_scriptidbase_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008847_void_rage_atplaceptrcreator_void_ptr_rage_fwconfigmanager_fwpool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008917_rage_fwtimer_enduserpause(ph: *mut Phantom); }
extern "C" { pub fn f_008918_rage_fwtimer_startuserpause_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008931_bool_ref_rage_atmap_rage_strindex_bool_rage_atmaphashfn_rage_str(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_008951_rage_fwbasepool_isvalidptr_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008952_rage_fwbasepool_getat_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008953_rage_fwknownrefholder_fwknownrefholder_void_ptr_ptr_rage_fwknown(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_009048_rage_crproperty_getattribute_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009212_rage_crskeletondata_computeglobaltransform_int_rage_mat34v_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_009216_rage_crskeletondata_convertbonenametoid_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009217_rage_crskeletondata_debugconvertboneidtoname_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_009233_rage_crskeleton_getglobalmtx_unsigned_int_rage_mat34v_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009234_rage_crskeleton_setglobalmtx_unsigned_int_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009236_rage_crskeleton_getmemorysize_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009640_rage_grcviewport_isspherevisible_float_float_float_float_float_p(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_009646_rage_grcviewport_transform_rage_vec3v_const_ref_float_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_010159_rage_grcdebugdraw_setdisplaydebugtext_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010172_rage_grcdebugdraw_text_float_float_rage_color32_char_const_ptr_b(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i64, a4: i32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010179_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010184_rage_grcdebugdraw_circle_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010188_rage_grcdebugdraw_boxaxisaligned_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010189_rage_grcdebugdraw_boxoriented_rage_vec3v_const_ref_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010193_rage_grcdebugdraw_capsule_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010195_rage_grcdebugdraw_arrow_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_010217_rage_grcdebugdraw_printtoscreencoors_char_const_ptr_int_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010220_rage_grcdebugdraw_adddebugoutput_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010224_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010225_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010500_rage_iovalue_isdown_float_rage_iovalue_readoptions_const_ref_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010511_rage_iovalue_ispressed_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010514_rage_iovalue_isreleased_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_011120_void_rage_atplaceptrcreator_void_ptr_rage_spropdetails_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011400_rage_cutsmanager_getconcatsectionfortime_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_011749_rage_fraginst_getgroupbroken_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011750_rage_fraginst_getchildbroken_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011751_rage_fraginst_getcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011776_rage_fragtype_getboneindexfromid_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011791_rage_phcontactconstiterator_op(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011822_rage_matrix34_dot_rage_matrix34_const_ref_rage_matrix34_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011885_rage_phiterator_dtorphiterator(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011964_rage_fragtype_getskeletondata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012021_rage_fragphysicslodgroup_getlodbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012183_rage_ptxeffectinst_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012191_rage_ptxeffectinst_finish_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012196_rage_ptxeffectinst_setbasemtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012201_rage_ptxeffectinst_setbasepos_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012202_rage_ptxeffectinst_setoffsetmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012207_rage_ptxeffectinst_setevovaluefromhash_rage_athashvalue_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_012650_rage_snsession_leave_int_rage_netstatus_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_012651_rage_snsession_destroy_rage_netstatus_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012653_bool_rage_rltaskbase_configure_rage_sndestroytask_rage_snsession(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_012654_rage_snsession_addtask_rage_sntask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012655_rage_snsession_snsession(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012659_rage_snsession_shutdown_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_012660_rage_snsession_dtorsnsession(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012661_rage_snsession_init_rage_sysmemallocator_ptr_rage_snsessionowner(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_012662_rage_snsession_adddelegate_rage_atdelegator2_void_rage_snsession(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012664_rage_snsession_getgamers_rage_rlgamerinfo_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_012667_rage_snparty_exists_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012668_rage_snsession_host_int_rage_rlnetworkmode_int_int_rage_rlmatchi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32, a7: i64, a8: i32, a9: i64) -> i32; }
extern "C" { pub fn f_012669_rage_snsession_join_int_rage_rlsessioninfo_const_ref_rage_rlnetw(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i64, a9: i32, a10: i64) -> i32; }
extern "C" { pub fn f_012670_rage_snsession_kick_rage_rlgamerhandle_const_ptr_unsigned_int_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012671_rage_snsession_getsessionid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012675_rage_snsession_isjoinableviapresence_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012676_rage_snsession_sendinvites_rage_rlgamerhandle_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_012677_rage_snsession_getsessioninfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012678_rage_netstatus_setpending(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012679_rage_snsession_update_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012680_rage_snsession_ishost_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012681_rage_snparty_ismember_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012682_rage_snsession_ismember_rage_rlgamerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012685_rage_snsession_getpeerendpointid_unsigned_long_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012686_rage_snsession_gethostpeerid_unsigned_long_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012687_rage_snsession_getgamers_unsigned_long_rage_rlgamerinfo_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_012688_rage_snsession_gethostendpointid_unsigned_int_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012689_bool_rage_datimportbuffer_seruser_rage_rlsessioninfo_rage_rlsess(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012716_rage_snpeer_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012720_rage_snpeer_getgamers_rage_rlgamerhandle_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_012721_rage_snpeer_getpeerinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012724_rage_snsession_canhost_rage_rlnetworkmode(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_012728_rage_snsession_getsessiontoken_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012729_rage_snsession_reserveslots_rage_rlgamerhandle_const_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_012732_rage_snsession_markasnotmigrating_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012735_bool_rage_rltaskbase_configure_rage_rlfireandforgettask_rage_snm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_012736_rage_netmessage_getname_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012741_rage_snsession_getgamerinfo_rage_rlgamerid_const_ref_rage_rlgame(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_012742_rage_snsession_findpeerbyid_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012746_rage_snsession_getpeercxn_unsigned_long_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012750_rage_snsession_isestablished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012751_rage_snsession_ismigrating_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012755_rage_snsession_getrlsession_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012756_rage_snsession_ishostendpointid_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_012758_rage_snsession_setowner_rage_snsessionowner_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012759_rage_snsession_setpresenceattributenames_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_012764_rage_snsession_changeattributes_rage_rlmatchingattributes_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012765_rage_snsession_getconfig_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012766_rage_snsession_migrate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012768_rage_snsession_cancelmigrationtasks(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012769_rage_snsession_doesallowmigration_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012770_rage_snsession_cancelallserialtasks(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012771_rage_snsession_cancelallconcurrenttasks(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012772_rage_snsession_markasestablished_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012773_rage_snsession_isgamerestablished_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012775_rage_snsession_hasslotreservation_rage_rlgamerhandle_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012776_rage_snsession_getmaxslots_rage_rlslottype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_012777_rage_snsession_getnumslotswithparam_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_012778_rage_snsession_freeslots_rage_rlgamerhandle_const_ptr_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012779_rage_snsession_getgamerinfo_rage_rlgamerhandle_const_ref_rage_rl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_012780_rage_snsession_getgamers_unsigned_int_rage_rlgamerinfo_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_012781_rage_snsession_removedelegate_rage_atdelegator2_void_rage_snsess(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012782_rage_snsession_setsessionuserdata_unsigned_char_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_012783_rage_snsession_getpeeridbycxn_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012804_bool_rage_datimportbuffer_seruser_rage_rlgamerid_rage_rlgamerid(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012810_rage_textchat_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_012811_rage_textchat_removetyper_rage_rlgamerid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012812_rage_textchat_isinitialized_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012813_rage_textchat_addtyper_rage_rlgamerinfo_const_ref_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_012814_rage_textchat_setonsameteam_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_012815_rage_textchat_havetyper_rage_rlgamerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012816_rage_textchat_settextchatrecipients_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012817_rage_textchat_ismuted_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012818_rage_atdelegator4_void_rage_textchat_ptr_rage_rlgamerhandle_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_012843_rage_environmentcloth_getcloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012845_rage_environmentcloth_getoffset(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012869_rage_environmentcloth_getbbminworldspace(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012870_rage_environmentcloth_getbbmaxworldspace(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012965_bool_rage_datimportbuffer_seruser_rage_rlgamerhandle_rage_rlgame(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012985_std_2_enable_if_is_move_constructible_rage_rlpeerinfo_value_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012986_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_rage_peerp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012987_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_rage_peerp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_012988_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_peerp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_012989_void_std_2_insertion_sort_abi_nqn220108_std_2_classicalgpolicy_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012990_void_std_2_insertion_sort_unguarded_abi_nqn220108_std_2_classica(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012991_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012992_void_std_2_pop_heap_abi_nqn220108_std_2_classicalgpolicy_rage_pe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012993_rage_rlpeerinfo_ptr_std_2_partition_with_equals_on_left_abi_nqn2(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012994_std_2_pair_rage_rlpeerinfo_ptr_bool_std_2_partition_with_equals(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012995_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013248_rage_rlsessioninfo_op_rage_rlsessioninfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013249_rage_snqueryhostconfigtask_setretrypolicy_unsigned_int_unsigned(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_013269_rage_snjoingamerstorlinetask_iswaitingforaddgamers(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013289_rage_snmodifypresenceflagstask_snmodifypresenceflagstask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013368_rage_neteventconsumer_rage_netevent_dtorneteventconsumer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013373_rage_voicechat_removealltalkers(ph: *mut Phantom); }
extern "C" { pub fn f_013374_rage_voicechat_setvpdthreshold_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013375_rage_voicechat_setvpdthresholdptt_unsigned_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013390_rage_netbandwidthrecorder_netbandwidthrecorder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013402_rage_netpacketstreamrecorder_init_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013406_rage_netcompressiontrainer_dtornetcompressiontrainer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013408_rage_nethardware_getlocalipaddress_rage_netipaddress_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013409_rage_netconnectionmanager_registerchannelpacketrecorder_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_013410_rage_netconnectionmanager_netcriticalsectiontoken_lock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013411_rage_netconnectionmanager_netcriticalsectiontoken_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013415_rage_netaddress_formatattemptipv4_char_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013417_rage_netaddress_tostring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013424_rage_netrandom_getint(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013425_rage_netconnection_setpolicies_rage_netconnectionpolicies_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013427_rage_netendpoint_getid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013428_rage_netconnectionmanager_getendpointbyid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_013429_char_ptr_rage_safecatf_char_char_ptr_unsigned_long_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_013438_rage_netaddress_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013440_rage_netaddress_sanitycheck_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013444_rage_cxn_init_rage_netconnectionmanager_ptr_rage_neteventqueue_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_013451_rage_netstatus_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013452_rage_netpeeraddress_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013456_rage_nettimestep_settime_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013461_rage_netconnectionmanager_delegate_delegate(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013462_rage_netconnectionmanager_delegate_dtordelegate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013463_rage_netconnectionmanager_delegate_unregister(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013464_rage_netconnectionmanager_removechanneldelegate_rage_netconnecti(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013465_rage_netconnectionmanager_channel_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013468_rage_netaddress_gettargetaddress_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013477_rage_netpeeraddress_tostring_char_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_013478_rage_netconnectionmanager_dtornetconnectionmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013479_rage_netconnectionmanager_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013481_rage_serverinfo_clearaddr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013487_rage_netdynarray_rage_netendpoint_4u_65535u_resize_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013488_rage_netdynarray_rage_cxn_4u_65535u_resize_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013489_rage_netcrypto_generaterandombytes_unsigned_char_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013490_rage_netsocketmanager_addconcurrentdelegate_rage_netsocketeventt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_013491_rage_netrelay_addconcurrentdelegate_rage_netrelayeventtype_rage(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_013492_rage_netrelay_startrelay(ph: *mut Phantom); }
extern "C" { pub fn f_013494_rage_nettimestep_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013495_rage_netpeerid_getlocalpeerid_rage_netpeerid_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013496_rage_netdynarray_rage_netconnectionrouter_routepair_data_25u_409(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013500_rage_nettunneler_closetunnel_rage_netaddress_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013509_rage_netconnectionmanager_opentunnel_rage_netpeeraddress_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_013514_rage_netpeeraddress_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013516_rage_nettunnelrequest_pending_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013517_rage_netaddress_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013533_rage_netaddress_isrelayserveraddr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013539_rage_netconnectionmanager_closeconnection_int_rage_netclosetype(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013545_rage_netconnectionmanager_getnumtimeoutpending(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_013546_rage_netconnectionmanager_setglobalsendinterval_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013547_rage_netconnectionmanager_setchannelpolicies_unsigned_int_rage_n(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_013548_rage_netconnectionmanager_getchannelpolicies_unsigned_int_rage_n(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_013549_rage_netconnectionmanager_setdefaultconnectionpolicies_unsigned(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_013550_rage_netconnectionmanager_send_int_void_const_ptr_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_013554_rage_netconnectionmanager_getaddress_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_013558_rage_netconnectionmanager_getunackedcount_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013559_rage_netconnectionmanager_getoldestresendcount_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013561_rage_netconnectionmanager_addchanneldelegate_rage_netconnectionm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_013562_rage_netsocketaddress_tostring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013566_rage_netaddress_isdirectaddr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013571_rage_netsocket_send_rage_netsocketaddress_const_ref_void_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_013578_rage_netconnectionmanager_update_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_013579_rage_netfunctionprofiler_pushprofiler_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013580_rage_netfunctionprofiler_popprofiler_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_013587_rage_netconnectionmanager_dumpstats(ph: *mut Phantom); }
extern "C" { pub fn f_013598_rage_netpeerid_op_rage_netpeerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013602_rage_netpeerid_op_rage_netpeerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013607_rage_netconnectionmanager_getconnectionid_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_013610_rage_netconnectionmanager_isopen_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013612_rage_netconnectionmanager_ispendingopen_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013615_rage_netsocketaddress_getip_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013617_rage_netrelay_senddata_rage_netsocketaddress_const_ref_bool_void(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_013619_rage_nethardware_getpublicipaddress_rage_netipaddress_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013620_rage_netipaddress_op_rage_netipaddress_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013638_bool_rage_datexportbuffer_seruns_unsigned_int_const_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013639_bool_rage_datimportbuffer_seruns_unsigned_int_unsigned_int_ref_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013640_rage_netcrypto_init(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_013641_rage_rc4context_rc4context(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013645_rage_netp2pcrypt_key_dtorkey(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013668_rage_nethttprequest_nethttprequest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013670_rage_netrandom_reset_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013677_rage_nethttprequest_init_rage_sysmemallocator_ptr_wolfssl_ctx_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013682_rage_netsocketaddress_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013692_rage_netipaddress_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013695_rage_nethttprequest_addrequestheadervalue_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013699_char_ptr_rage_formatf_char_2048ul_char_ref_2048ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013704_rage_nethttprequest_urlencode_char_ptr_unsigned_int_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_013705_rage_nethttprequest_addformparam_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013706_rage_nethttprequest_addintformparam_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013707_rage_nethttprequest_addunsformparam_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013710_rage_nethttprequest_beginlistformparam_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013711_rage_nethttprequest_addlistformparamvalue_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013719_rage_nettimeout_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013725_rage_nettimeout_istimedout_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013733_rage_nethttprequest_setsimulatedfailpct_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013734_rage_netsocketaddress_init_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013735_rage_nettimeout_initseconds_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013743_rage_nethttpproxy_set_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013774_rage_netretrytimer_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013778_rage_netstatus_forcesucceeded_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013779_rage_netstatus_forcefailed_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013811_bool_rage_datexportbuffer_seruns_unsigned_short_const_unsigned_s(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013813_rage_netrelay_getmyrelayaddress(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_013814_rage_netnatinfo_init_bool_rage_netnattype_rage_netnatportmapping(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_013815_rage_netnatdetector_updatergsc(ph: *mut Phantom); }
extern "C" { pub fn f_013816_rage_netnatinfo_dumpinfo_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013817_rage_netsocket_setinitialvalues(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013818_rage_netnatportmapper_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013820_rage_netsocket_dtornetsocket(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013821_rage_netnatportmapper_init_rage_netsocket_ptr_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_013822_rage_nethardware_createsocket_rage_netsocket_ptr_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_013823_rage_netsocket_cansendreceive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013825_rage_netrelaypacket_resetpingheader_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013826_rage_netnatportmapper_updateonealtport_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013827_rage_netnatportmapper_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013829_rage_netsocket_receive_rage_netsocketaddress_ptr_void_ptr_int_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_013830_rage_netnatportmapper_mapport_unsigned_int_rage_netsocketaddress(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_013831_rage_netsocket_isreadytosend_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013834_rage_netrelay_adddelegate_rage_netrelayeventtype_rage_atdelegato(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_013840_rage_netnatporttimeoutchecker_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013842_rage_netnatporttimeoutchecker_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013843_rage_netnatporttimeoutchecker_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013856_rage_netnatpcpinfo_dumpinfo_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013857_rage_netnatupnpinfo_dumpinfo_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013858_rage_rlstandardmetrics_natinfometric_write_rage_rsonwriter_ptr_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013859_rage_nethardware_getnattype(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_013860_rage_netnatdetector_setnatdiscoverystate_rage_netnattypediscover(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_013861_rage_netnatpcp_deletepcpaddressreservation_rage_netstatus_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013867_rage_nethardware_getmacaddress_unsigned_char_ref_6(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013869_rage_netpool_rage_nettaskimpl_256ul_netpool(ph: *mut Phantom); }
extern "C" { pub fn f_013870_rage_netpool_rage_nettaskworkitem_64ul_netpool(ph: *mut Phantom); }
extern "C" { pub fn f_013871_rage_nettask_update(ph: *mut Phantom); }
extern "C" { pub fn f_013872_rage_nettask2_getroottask(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_013873_rage_nettask2_updatetask_rage_nettaskimpl_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013874_rage_neteventqueue_rage_netrelayevent_nexteventinternal_rage_net(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013875_rage_netrelay_connect_rage_netsocketaddress_const_ref_bool_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_013876_rage_netrelay_initendpoints(ph: *mut Phantom); }
extern "C" { pub fn f_013877_rage_netshutdown(ph: *mut Phantom); }
extern "C" { pub fn f_013882_rage_netipv4address_netipv4address(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013884_rage_netipv4address_fromstring_char_const_ptr_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013885_rage_netipaddress_format_char_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_013886_rage_netipaddress_fromstring_char_const_ptr_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013887_rage_netsocketaddress_netsocketaddress(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013888_rage_netsocketaddress_netsocketaddress_rage_netipaddress_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_013890_rage_netsocketaddress_formatattemptipv4_char_ptr_unsigned_int_bo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013902_rage_netallocationprofiler_printstats_rage_netmemallocator_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013930_rage_netfunctionprofiler_startframe(ph: *mut Phantom); }
extern "C" { pub fn f_013931_rage_netfunctionprofiler_terminateframe_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_013932_rage_netfunctionprofiler_settimethreshold_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_013933_rage_netfunctionprofiler_setshouldprofile_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_013935_rage_nettokenbucket_nettokenbucket_float_float_char_const_ptr(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64) -> i64; }
extern "C" { pub fn f_013936_rage_nettokenbucket_consume_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013937_rage_nettokenbucket_canconsume_float(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013940_rage_netsocketmanager_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013941_rage_netsocketmanager_destroysockets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013942_rage_netretrytimer_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_013944_rage_netretrytimer_initseconds_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013945_rage_netretrytimer_initmilliseconds_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013946_rage_netretrytimer_forceretry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013949_rage_netretrytimer_istimetoretry_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013950_rage_netretrytimer_restart(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013958_rage_netpeeraddress_hasnetwork(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_013959_rage_netpeeraddress_netpeeraddress(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013960_rage_netpeeraddress_clearnonadvertisabledata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013961_rage_netpeeraddress_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013968_rage_netpeercomplainer_netpeercomplainer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013970_rage_netpeercomplainer_dtornetpeercomplainer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013971_rage_netpeercomplainer_setpolicies_rage_netpeercomplainer_polici(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013972_rage_netpeercomplainer_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013973_rage_netpeercomplainer_addpeer_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013974_rage_netpeercomplainer_havepeer_unsigned_long_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013975_rage_netpeercomplainer_removepeer_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013976_rage_netpeercomplainer_unregistercomplaint_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013978_bool_rage_netconnectionmanager_sendoutofband_rage_netcomplaintms(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_013979_rage_netpeercomplainer_iscomplaintregistered_unsigned_long_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013980_rage_netpeercomplainer_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013997_rage_netrelay_freeendpoint_rage_netdtlsendpoint_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013999_rage_netretrytimer_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014000_rage_netrelaypresencestats_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014015_rage_nettask_destroy_rage_nettask_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014016_rage_nettask_run_rage_nettask_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014019_rage_nettask_hastask_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014020_rage_nettask_cancel_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014022_rage_nettask_cancelall(ph: *mut Phantom); }
extern "C" { pub fn f_014023_rage_nettask_nettask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014028_rage_nettask_allocate_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014043_rage_nettcp_getinsecuresslctx(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_014046_xmalloc(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014049_rage_nettimestep_nettimestep(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014050_rage_nettimeout_isrunning_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014051_rage_netretrytimer_netretrytimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014052_rage_netretrytimer_dtornetretrytimer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014053_rage_netretrytimer_resetintervalmilliseconds_int_int(ph: *mut Phantom); }
extern "C" { pub fn f_014054_rage_netretrytimer_getcurrentretryintervalseconds_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014055_rage_netretrytimer_getsecondsuntilretry_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014060_rage_nettimesync_dtornettimesync(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014061_rage_nettimesync_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014063_rage_nettimesync_start_rage_netconnectionmanager_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_014064_rage_nettimesync_restart_unsigned_int_unsigned_int_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_014065_rage_nettimesync_generatetoken(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014066_rage_nettimesync_update(ph: *mut Phantom); }
extern "C" { pub fn f_014067_rage_nettimesync_gettime_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014068_rage_nettimesync_hasstarted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014069_rage_nettimesync_hassynched_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014070_rage_nettunnelrequest_nettunnelrequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014072_rage_nettunnelrequest_dtornettunnelrequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014076_char_ptr_rage_formatf_char_4ul_char_ref_4ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_014154_rage_netresponsehandler_netresponsehandler(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014163_rage_nettransactor_init_rage_netconnectionmanager_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014164_rage_nettransactor_isinitialized_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014165_rage_nettransactor_update_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014166_rage_nettransactor_addrequesthandler_rage_netrequesthandler_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_014171_rage_netpacketrecorder_dtornetpacketrecorder_23323(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014248_rage_rltaskmanager_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014249_rage_rlgetnativeenvironment(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014250_rage_rlgetenvironmentstring_rage_rlenvironment(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014251_rage_rltaskmanager_init_rage_sysmemallocator_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014254_rage_rlshutdown(ph: *mut Phantom); }
extern "C" { pub fn f_014255_rage_rlcheckhostsfile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014256_rage_rlstagingenvironmentinit_char_const_ptr_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014257_rage_is_ui_showing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014259_rage_rltaskmanager_doaddtask_unsigned_long_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014260_rage_rltaskmanager_destroytask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014261_rage_rlros_shutdownclass(ph: *mut Phantom); }
extern "C" { pub fn f_014267_rage_rlpresence_refreshsigninstate_int(ph: *mut Phantom); }
extern "C" { pub fn f_014268_rage_rlpresence_refreshnetworkstatus_int(ph: *mut Phantom); }
extern "C" { pub fn f_014269_rage_rlfriendspage_init_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014270_rage_rlfriendsmanager_clear(ph: *mut Phantom); }
extern "C" { pub fn f_014271_rage_rltaskmanager_cancelall_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014274_rage_rlfriendspage_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014275_rage_rltaskmanager_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014276_rage_rlpc_update(ph: *mut Phantom); }
extern "C" { pub fn f_014277_rage_rltaskmanager_updatetask_rage_rltaskbase_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_014278_rage_ficloudhandle_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014279_rage_rltelemetry_flush(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014280_rage_rltelemetry_flushimmediate(ph: *mut Phantom); }
extern "C" { pub fn f_014281_rage_rltelemetry_flushdeferred(ph: *mut Phantom); }
extern "C" { pub fn f_014282_rage_rlfriendsmanager_activate_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_014283_rage_rlfriendsreader_read_int_unsigned_int_rage_rlfriend_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64, a6: i64, a7: i32, a8: i64) -> i32; }
extern "C" { pub fn f_014284_rage_rlfriendsmanager_isfriendswith_int_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_014285_rage_rlcloud_unwatchmemberitem_rage_rlcloudwatcher_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014286_void_rage_rldelete_rage_rlscmatch_rage_rlscmatch_ptr_unsigned_in(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014287_rage_rltaskbase_cancel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014288_rage_rltaskmanager_canceltask_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014289_rage_rltaskmanager_canceltask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014290_rage_rlrosgetforcedenvironment(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014291_rage_rlisusingstagingenvironment(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014292_rage_rlgettaskmanager(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_014293_rage_rlsetlanguage_rage_syslanguage(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_014294_rage_rlgetposixtime_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014295_rage_rlcreateuuid_unsigned_long_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014296_rage_rlcreateuuid_rage_rluuid64_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014297_rage_rlloggedallocate_unsigned_long_char_const_ptr_int_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014298_rage_rlfree_void_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014299_rage_rlrosgetenvironmentasstring_rage_rlrosenvironment(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014301_rage_rlgamerhandle_equals_rage_rlgamerhandle_const_ref_bool_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_014302_rage_rlgamerhandle_getlocalindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014303_rage_rlgamerhandle_op_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014304_rage_rlros_isonline_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014305_rage_rlworker_rlworker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014306_rage_rlworker_start_char_const_ptr_unsigned_int_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_014309_rage_rlworker_stop(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014311_rage_rlachievement_rlachievement(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014312_rage_rlachievement_dtorrlachievement(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014313_rage_rlworker_dtorrlworker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014314_rage_rlachievement_init_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014315_rage_rlgamerinfo_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014316_rage_rlgamerinfo_getgamerhandle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014317_rage_rlworker_wakeup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014318_rage_rlgamerinfo_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014319_rage_rlgamerhandle_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014321_rage_rlgamerhandle_getrockstarid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014322_rage_rlgamerinfo_issignedin_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014343_rage_rlfriendspage_clearfriendsarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014344_rage_rlfriendspage_getfriend_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014345_rage_rlfriend_getgamerhandle_rage_rlgamerhandle_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014346_rage_rlfriendspage_getfriendindex_char_const_ptr_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014347_rage_rlfriendspage_fillfriendarray_rage_atarray_rage_rlgamerhand(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014348_rage_rlfriend_op_rage_rlfriend_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014349_rage_rlgamerhandle_touserid_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014350_rage_rlfriend_isonline_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014351_rage_rlfriend_isinsametitle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014352_rage_rlfriend_isinsession_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014354_rage_rlpresence_isonline_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014355_rage_rlfriendsmanager_isreading(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014356_rage_rlfriendsmanager_gettotalnumfriends_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014357_rage_rlfriendsmanager_deactivate(ph: *mut Phantom); }
extern "C" { pub fn f_014358_rage_rlfriendsmanager_getgamerhandle_int_rage_rlgamerhandle_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_014359_rage_rlfriendsmanager_getfriends_int_rage_rlfriendspage_ptr_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_014360_rage_rlfriendsmanager_canqueuefriendsrefresh_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014361_rage_rlros_getcredentials_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014362_rage_rlroscredentials_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014363_rage_rlfriendsmanager_forceallowfriendsrefresh(ph: *mut Phantom); }
extern "C" { pub fn f_014364_rage_rlfriendsmanager_requestrefreshfriendspage_int_rage_rlfrien(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_014365_rage_rltaskmanager_findtask_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014366_rage_rlpresence_getname_int_char_ptr_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_014367_rage_rlfriend_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014368_rage_rlgamerhandle_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014369_rage_rlgamerhandle_tostring_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014370_rage_rlgamerhandle_tostring_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014371_rage_rlgamerhandle_fromstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014372_rage_rlgamerhandle_fromuserid_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014374_rage_rlgamerhandle_export_void_ptr_unsigned_int_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014375_rage_rlgamerhandle_import_void_const_ptr_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014376_rage_rlgamerhandle_op_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014377_rage_rlgamerhandle_touniqueid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014378_rage_rlgamerid_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014379_rage_rlgamerid_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014381_rage_rlgamerid_export_void_ptr_unsigned_int_unsigned_int_ptr_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014384_rage_rlpresence_issignedin_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014385_rage_rlgamerinfo_getage_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014386_rage_rlgamerinfo_getagegroup_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014387_rage_rlgamerinfo_rlgamerinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014388_rage_rlgamerinfo_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014389_rage_rlgamerinfo_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014390_rage_rlgamerinfo_isremote_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014391_rage_rlgamerinfo_isonline_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014392_rage_rlgamerinfo_getgamerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014395_rage_rlgamerinfo_op_rage_rlgamerinfo_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014396_rage_rlpeerinfo_op_rage_rlpeerinfo_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014397_rage_rlgamerinfo_op_rage_rlgamerinfo_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014398_rage_rlmatchingattributes_rlmatchingattributes(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014399_rage_rlmatchingattributes_getattrid_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014400_rage_rlmatchingattributes_setvaluebyindex_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_014401_rage_rlmatchingattributes_getvaluebyindex_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_014402_rage_rlmatchingattributes_getvaluebyid_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_014405_rage_rlmatchingfilter_rlmatchingfilter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014406_rage_rlmatchingfilter_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014407_rage_rlmatchingfilter_getsessionattrfieldidforcondition_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014408_rage_rlmatchingfilter_setvalue_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_014409_rage_rlmatchingfilter_clearvalue_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014410_rage_rlmatchingfilter_getvalue_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_014415_rage_rlpeerinfo_getpeeraddress_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014441_rage_rlros_adddelegate_rage_atdelegator1_void_rage_rlrosevent_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014443_rage_rlros_removedelegate_rage_atdelegator1_void_rage_rlrosevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014444_rage_rlpresence_fakeofflineduetouplink_bool(ph: *mut Phantom); }
extern "C" { pub fn f_014445_rage_rlpresence_getgamerhandle_int_rage_rlgamerhandle_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_014446_rage_rlroscredentials_getrockstarid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014447_rage_rlpresence_getgamerinfo_int_rage_rlgamerinfo_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_014449_rage_rlpresence_adddelegate_rage_atdelegator1_void_rage_rlpresen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014450_rage_rlpresence_removedelegate_rage_atdelegator1_void_rage_rlpre(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014452_rage_rlpresence_setintattribute_int_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014453_rage_rlpresence_setdoubleattribute_int_char_const_ptr_double_boo(ph: *mut Phantom, a0: i64, a1: f64) -> i32; }
extern "C" { pub fn f_014454_rage_rlpresence_setstringattribute_int_char_const_ptr_char_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014455_rage_rlpresence_getintattribute_int_char_const_ptr_long_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014456_rage_rlpresence_getdoubleattribute_int_char_const_ptr_double_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014457_rage_rlpresence_getattributesforgamer_int_rage_rlgamerhandle_con(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_014459_rage_rltaskbase_configure_rage_netstatus_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014460_rage_rltaskmanager_addparalleltask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014461_rage_rlclan_getprimaryclan_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014462_rage_rlpresence_postmessage_int_rage_rlgamerhandle_const_ptr_uns(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014464_rage_rlpresence_clearactinggamerinfo(ph: *mut Phantom); }
extern "C" { pub fn f_014466_rage_rlroscredentials_getrockstaraccount_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014467_rage_rlsession_ishost_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014468_rage_rlscmatchmakingmatchid_tostring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014471_rage_rlrosclient_setcredentials_rage_rlroscredentials_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014473_bool_rage_rlscpresencemessage_isa_rage_rlscpresencemessagempinvi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014474_rage_rlscpresencemessagempinvite_import_rage_rsonreader_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014486_rage_rlsessioninfo_tostring_char_ptr_unsigned_int_unsigned_int_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_014487_rage_rlsessioninfo_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014488_rage_rlprivileges_checkprivileges_int_int_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014489_rage_rlprivileges_isprivilegecheckresultready(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014490_rage_rlprivileges_isprivilegecheckinprogress(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014491_rage_rlprivileges_isprivilegechecksuccessful(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014492_rage_rlprivileges_setprivilegecheckresultnotneeded(ph: *mut Phantom); }
extern "C" { pub fn f_014493_rage_rlugc_checktext_int_char_const_ptr_char_const_ptr_char_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014499_rage_rlsession_getsessioninfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014501_rage_rlsessioninfo_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014512_rage_rlscmatchmaking_unadvertise_int_rage_rlscmatchmakingmatchid(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014514_rage_rlscmatchmakingmatchid_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014546_rage_rlsession_isestablished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014550_rage_rlsession_getpresenceflags_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014552_rage_rlsession_getconfig_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014556_rage_rlsessionconfig_export_void_ptr_unsigned_int_unsigned_int_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014560_rage_rlsessiondetail_reset_rage_rlpeerinfo_const_ref_rage_rlsess(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64, a9: i32, a10: i64, a11: i32); }
extern "C" { pub fn f_014563_rage_rlsessiondetail_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014567_rage_rlsessiondetail_rlsessiondetail(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014573_rage_rlsessioninfo_export_void_ptr_unsigned_int_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014579_rage_rlsessionmanager_getsession_rage_rlsessioninfo_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014580_rage_rlsessioninfo_rlsessioninfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014581_rage_rlscmatchmakingmatchid_op_rage_rlscmatchmakingmatchid_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014582_rage_rlsessioninfo_op_rage_rlsessioninfo_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014583_rage_rlsessionmanager_querydetail_rage_rlnetworkmode_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i64, a9: i64, a10: i64, a11: i64) -> i32; }
extern "C" { pub fn f_014584_rage_rlsessionmanager_querydetail_rage_rlnetworkmode_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014585_rage_rlsessionmanager_cancel_rage_netstatus_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014586_rage_rlsessionmanager_setretrypolicy_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_014600_rage_rlsessioninfo_fromstring_char_const_ptr_unsigned_int_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_014601_rage_rlsessionfinder_find_int_rage_rlnetworkmode_unsigned_int_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i64, a8: i64) -> i32; }
extern "C" { pub fn f_014604_rage_rlsessioninfo_op_rage_rlsessioninfo_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014605_rage_rlhttptask_rlhttptask_rage_sysmemallocator_ptr_unsigned_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_014606_rage_rlroshttptask_configure_int_rage_rlhttptaskconfig_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_014607_rage_rlhttptask_addintparameter_char_const_ptr_long_long_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_014608_rage_rlhttptask_addstringparameter_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014609_rage_rlstats_cancel_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014622_rage_rlsystemui_isuishowing_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014623_rage_rlsystemui_showsigninui_int_unsigned_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014628_rage_rltaskbase_rltaskbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014637_rage_rltaskbase_ispending_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014641_rage_rltaskmanager_appendserialtask_unsigned_long_rage_rltaskbas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014642_rage_inmap_base_unsigned_long_rage_rltaskbase_refrage_rltaskbase(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_014652_rage_zlibstream_beginencode_rage_sysmemallocator_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014655_rage_zlibstream_dtorzlibstream(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014661_rage_rlhttptask_appendcontent_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_014662_rage_rlmetric_dtorrlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014666_rage_rltelemetrypolicies_applydebugmetrics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014667_rage_rltelemetrypolicies_loadmetricsfromjson_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_014668_rage_rltelemetrypolicies_applydefaultmetrics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014670_rage_rlmetricsconfig_dtorrlmetricsconfig(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014671_rage_rlmetricsconfig_op_rage_rlmetricsconfig_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014672_rage_rltelemetrypolicies_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014673_rage_atarray_rage_atbinarymap_rage_rlmetricconfig_rage_atnonfina(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014677_rage_rltelemetrypolicies_dumpsettings_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014679_rage_rltelemetryclientconfig_cancel(ph: *mut Phantom); }
extern "C" { pub fn f_014680_rage_rltelemetryclientconfig_settelemetryclientconfigstate_rage(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_014681_rage_rlcloud_gettitlefile_char_const_ptr_rage_rlrossecurityflags(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64) -> i32; }
extern "C" { pub fn f_014682_rage_rltelemetryclientconfig_checkforretry(ph: *mut Phantom); }
extern "C" { pub fn f_014685_rage_rlmetric_rlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014690_rage_rlmetric_isdisabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014691_rage_rlmetric_shouldincludeinsample_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014692_rage_rlmetriclist_rlmetriclist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014693_rage_rlmetriclist_dtorrlmetriclist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014694_rage_rlmetriclist_pop(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014695_rage_rlmetriclist_append_rage_rlmetric_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014706_rage_rltelemetry_shouldsendmetric_rage_rlmetric_const_ref_char_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014707_rage_rltelemetry_write_int_rage_rlmetric_const_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_014708_rage_rltelemetry_dowritetostreams_int_unsigned_long_rage_rlgamer(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_014710_rage_rltelemetry_getdeferredslot_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014711_rage_rltelemetry_getimmediateslot_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014715_rage_rltelemetry_submitmetriclist_int_rage_rlmetriclist_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014716_rage_rltelemetry_cancelflushes(ph: *mut Phantom); }
extern "C" { pub fn f_014717_rage_zlibstream_zlibstream(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014718_rage_zlibstream_begindecode_rage_sysmemallocator_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_014719_rage_zlibstream_encode_void_const_ptr_unsigned_int_void_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_014720_rage_zlibstream_decode_void_const_ptr_unsigned_int_void_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_014721_rage_zlibstream_iscomplete_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014722_rage_zlibstream_haserror_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014733_char_ptr_rage_formatf_char_33ul_char_ref_33ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_014736_rage_rlhttptask_configure_rage_rlhttptaskconfig_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014759_rage_rlpc_isatleastsocialclubversion_int_int_int_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014763_rage_rlpc_showui(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014772_rage_rlprofilestats_update(ph: *mut Phantom); }
extern "C" { pub fn f_014773_rage_rlprofilestats_cancel_rage_netstatus_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014774_rage_rlprofilestats_getflushstatus_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014775_rage_rlprofilestats_readstatsbygamer_int_rage_rlgamerhandle_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_014778_rage_rlprofilestatssynchronizegroupstask_ptr_rage_rltaskmanager(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_014779_bool_rage_rltaskbase_configure_rage_rlprofilestatssynchronizegro(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_014789_rage_rlprofilestatsvalue_rlprofilestatsvalue(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014791_rage_rlprofilestatsvalue_getfloat_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_014792_rage_rlprofilestatsvalue_getint32_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014793_rage_rlprofilestatsvalue_getint64_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014794_rage_rlprofilestatsvalue_setfloat_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_014795_rage_rlprofilestatsvalue_setint32_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014796_rage_rlprofilestatsvalue_setint64_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014797_rage_rlprofilestatsvalue_tostring_char_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014798_rage_rlprofilestatsvalue_serialize_unsigned_char_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_014799_rage_rlprofilestatsrecordbase_getvalue_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_014800_rage_rlprofilestatsreadresults_rlprofilestatsreadresults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014801_rage_rlprofilestatsreadresults_reset_int_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_014802_rage_rlprofilestatsreadresults_getstatcolumn_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014803_rage_rlprofilestatsreadresults_getgamerhandle_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_014804_rage_rlprofilestatsreadresults_setgamerhandle_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_014805_rage_rlprofilestatsreadresults_getvalue_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_014806_rage_rlprofilestatsreadresults_setnumrows_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014836_rage_rlroscredentials_hasprivilege_rage_rlrosprivilegeid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014854_rage_rlros_renewcredentials_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_014855_bool_rage_rlscpresencemessage_isa_rage_rlscpresenceinvalidatetic(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014856_rage_rlrosplatformcredentials_getlocalplatformcredentials_int_ra(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_014857_rage_rlrosclient_retry(ph: *mut Phantom); }
extern "C" { pub fn f_014858_rage_rlros_isanyonline(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014859_rage_rlros_getdefaultprivilege_rage_rlrosprivilegeid(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014860_rage_rlros_getgeolocinfo_rage_rlrosgeolocinfo_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014866_rage_rlrosserviceaccesstokenstore_serviceaccessrequest_requestto(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014867_rage_rlrosserviceaccesstokenstore_serviceaccessrequest_invalidat(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014870_bool_rage_rltaskbase_configure_rage_rlroscreatetickettask_int_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_014884_rage_rlrosgetenvironment(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014885_rage_rlrostitleid_getenvironment_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014886_rage_rlroscredentials_hasprivilegeenddate_rage_rlrosprivilegeid(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_014940_rage_rlrostitleid_getenvironmentname_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_014943_rage_rlsctaskbase_rlsctaskbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014967_rage_rlugc_createcontent_int_rage_rlugccontenttype_char_const_pt(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64, a9: i32, a10: i64, a11: i64, a12: i64) -> i32; }
extern "C" { pub fn f_014968_rage_rlugccreatecontenttask_ptr_rage_rltaskmanager_createtask_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014969_rage_rlugccreatecontenttask_configure_int_rage_rlugccontenttype(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32, a8: i32, a9: i64, a10: i32, a11: i64, a12: i32) -> i32; }
extern "C" { pub fn f_014970_rage_rlugctaskbase_appendfiles_rage_rlugc_sourcefile_const_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_014971_rage_rlugc_querycontent_int_rage_rlugccontenttype_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64) -> i32; }
extern "C" { pub fn f_014972_rage_rlugcupdatecontenttask_ptr_rage_rltaskmanager_createtask_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014973_rage_rlugcupdatecontenttask_configure_int_rage_rlugccontenttype(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i32, a9: i64, a10: i64) -> i32; }
extern "C" { pub fn f_014974_rage_rlugcgetcdncontenttask_rlugcgetcdncontenttask_rage_sysmemal(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014975_rage_rlugcgetcdncontenttask_configure_int_rage_datgrowbuffer_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_014976_rage_rlugccategorytostring_rage_rlugccategory(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014978_rage_rlugccontenttypetostring_rage_rlugccontenttype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014979_rage_rlugcmetadata_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014980_rage_rlugcmetadata_rlugcmetadata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014981_rage_rlugcmetadata_composecloudabspath_int_char_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_014982_rage_rlugcmetadata_composecloudabspath_rage_rlugccontenttype_cha(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_014983_rage_rlugcmetadata_getfileid_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014984_rage_rlugcmetadata_getfileversion_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014987_rage_rlugcplayer_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014988_rage_rlugcplayer_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014989_rage_rlugccreateurltemplatesfromjson_rage_rsonreader_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014990_rage_atarray_rage_rlugcurltemplate_0_unsigned_short_destruct_rag(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014992_rage_rlugcbuildurlfromtemplate_rage_atstring_const_ref_rage_atar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_015018_rage_rlcloud_verifycloudsignature_rage_rlroscontentsignature_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_015032_bool_rage_rlscpresencemessage_isa_rage_rlscpresencemessagepublis(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015033_rage_rlscpresencemessagepublish_import_rage_rsonreader_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015039_rage_rlcloudmemberid_rlcloudmemberid_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015040_rage_rlcloudmemberid_rlcloudmemberid_rage_rlgamerhandle_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015041_rage_rlcloudmemberid_tostring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015046_rage_rlcloudwatcher_rlcloudwatcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015052_rage_rlcloud_watchmemberitem_int_rage_rlcloudonlineservice_char(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_015054_rage_rlcloud_setsimulatedfailpct_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015077_rage_rlclanclient_refreshmine_rage_rlclanclient_refreshqueuetype(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_015086_rage_rlclan_adddelegate_rage_atdelegator1_void_rage_rlclanevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015087_rage_rlclan_removedelegate_rage_atdelegator1_void_rage_rlclaneve(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015088_rage_rlclan_isserviceready_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_015089_rage_rlclan_isfullserviceready_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_015090_rage_rlclan_hasprimaryclan_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_015091_rage_rlclan_getprimarymembership_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_015092_rage_rlclan_getmine_int_rage_rlclanmembershipdata_ptr_unsigned_i(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_015093_rage_rlclanclient_queuetask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015094_rage_rlclan_leave_int_long_rage_netstatus_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_015095_rage_rlclan_setprimaryclan_int_long_rage_netstatus_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_015096_rage_rlclan_invite_int_long_int_bool_rage_rlgamerhandle_const_re(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_015097_rage_rlclan_deleteinvite_int_long_rage_netstatus_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_015098_rage_rlclan_join_int_long_rage_netstatus_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_015099_rage_rlclan_getinvites_int_int_rage_rlclaninvite_ptr_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_015101_rage_rlclan_getsentinvites_int_int_rage_rlclaninvite_ptr_unsigne(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_015102_rage_rlclanrosgetalltask_ptr_rage_rltaskmanager_createtask_rage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015103_bool_rage_rltaskbase_configure_rage_rlclanrosgetalltask_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32, a7: i64, a8: i32, a9: i64, a10: i64, a11: i64) -> i32; }
extern "C" { pub fn f_015104_rage_rlclan_getopenclans_int_int_rage_rlclandesc_ptr_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_015105_rage_rlclan_getdesc_int_long_rage_rlclandesc_ptr_rage_netstatus(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_015106_rage_rlclan_getprimaryclans_int_rage_rlgamerhandle_const_ptr_uns(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_015107_rage_rlclan_getmembersbyclanid_int_int_long_rage_rlclanmember_pt(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_015108_rage_rlclan_getranks_int_int_long_rage_rlclanrank_ptr_unsigned_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_015109_rage_rlclan_getwallmessages_int_int_long_rage_rlclanwallmessage(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_015128_rage_rlclandesc_rlclandesc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015129_rage_rlclandesc_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015130_rage_rlclandesc_isrockstarclan_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015131_rage_rlclaninvite_rlclaninvite(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015132_rage_rlclaninvite_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015133_rage_rlclanmember_rlclanmember(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015134_rage_rlclanmembershipdata_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015135_rage_rlclanmember_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015136_rage_rlclanmembershipdata_rlclanmembershipdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015137_rage_rlclanjoinrequestrecieved_rlclanjoinrequestrecieved(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015138_rage_rlclanjoinrequestrecieved_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015139_rage_rlclanwallmessage_rlclanwallmessage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015140_rage_rlclanwallmessage_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015141_rage_rlclanmetadataenum_rlclanmetadataenum(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015142_rage_rlclanrosbasetask_rlclanrosbasetask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015213_rage_rlgamerquerydata_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015214_rage_rlsessionquerydata_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015215_rage_rlpresencequery_getsessionbygamerhandle_int_rage_rlgamerhan(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_015216_rage_rlpresencequery_findgamers_rage_rlgamerinfo_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_015229_rage_rlscauth_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_015232_bool_rage_rlscpresencemessage_isa_rage_rlscpresenceaccountlinkre(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015233_rage_rlscpresenceaccountlinkresult_import_rage_rsonreader_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015234_rage_rlscauth_onerror(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015235_rage_rlscauth_formathostname_char_ptr_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015236_char_ptr_rage_safecatf_char_2048ul_char_ref_2048ul_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015237_rage_rlscmatchmaking_unadvertiseall_int_rage_netstatus_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_015256_rage_rlscmatchmanager_hasanymatches(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_015257_rage_rlscpresenceattribute_rlscpresenceattribute(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015258_rage_rlscpresenceattribute_reset_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015260_rage_rlscpresencemessagepublish_rlscpresencemessagepublish_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_015261_rage_rlscpresencemessagepublish_export_rage_rsonwriter_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015262_rage_rlscpresenceinboxnewmessage_import_rage_rsonreader_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015263_rage_rlscpresenceinboxnewmessage_istagpresent_char_const_ptr_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015266_rage_rlinboxmessageiterator_releaseresources(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015267_rage_rlinboxmessageiterator_isreleased_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015268_rage_rlinboxmessageiterator_nextmessage_char_const_ptr_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_015269_rage_rlinbox_postmessage_int_rage_rlgamerhandle_const_ptr_unsign(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_015270_rage_rlinbox_getmessages_int_unsigned_int_unsigned_int_char_cons(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_015272_rage_rlinbox_getunreadmessages_int_unsigned_int_unsigned_int_cha(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_015273_rage_rlinboxtaglist_add_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015274_rage_rlinboxtaglist_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015275_rage_rlyoutubevideoinfo_rlyoutubevideoinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015276_rage_rlyoutubevideoinfo_addtag_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015277_rage_rlyoutube_getoauthtoken_int_rage_rlyoutubeupload_ptr_rage_n(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_015278_rage_rlyoutubegetresumableuploadurltask_configure_rage_rlyoutube(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015279_rage_rlyoutubeuploadtask_configure_rage_rlyoutubeupload_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_015280_rage_rlyoutube_uploadchunk_rage_rlyoutubeupload_ptr_unsigned_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_015281_rage_rlyoutube_getuploadstatus_rage_rlyoutubeupload_ptr_rage_net(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015283_rage_rlyoutube_showaccountlinkui_int_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_015285_rage_rlyoutubegetchannelinfotask_configure_rage_rlyoutubeupload(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015286_rage_rlyoutubegetvideoinfotask_configure_rage_rlyoutubeupload_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015813_rage_phconstraintbase_getinstanceb(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015846_rage_phcontact_computeworldpointbfromlocal_rage_phmanifold_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015847_rage_phcontact_computeworldpointafromlocal_rage_phmanifold_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015890_rage_phconstraintmgr_remove_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015891_rage_phconstraintmgr_gettemporarypointer_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015893_rage_phmultireaderlocktoken_waitasreader(ph: *mut Phantom); }
extern "C" { pub fn f_015966_rage_phsleep_reset_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015969_rage_phcollider_setreferenceframevelocity_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015980_rage_phcollider_getlocalvelocityrigid_float_vector_4_const(ph: *mut Phantom, a0: i64, a1: i64, a2: u128); }
extern "C" { pub fn f_015982_rage_phcollider_applyimpulsecenterofmassrigid_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015983_rage_phcollider_applyangimpulserigid_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015997_rage_phsimulator_addinstbehavior_rage_phinstbehavior_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015998_rage_phsimulator_deleteobject_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_015999_rage_phsimulator_removeinstbehavior_rage_phinstbehavior_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016006_rage_phsimulator_applyimpetusandbreakingimpetus_float_vector_4_i(ph: *mut Phantom, a0: i64, a1: u128, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: f32); }
extern "C" { pub fn f_016008_rage_phsimulator_applyforceandbreakingforce_float_vector_4_int_r(ph: *mut Phantom, a0: i64, a1: u128, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: f32, a8: i32); }
extern "C" { pub fn f_016009_rage_phsimulator_applyimpulse_int_rage_vector3_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32); }
extern "C" { pub fn f_016013_rage_phsimulator_addinactiveobject_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016034_rage_pharchetype_pharchetype(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016036_rage_pharchetype_setbound_rage_phbound_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016087_rage_phinst_setarchetype_rage_pharchetype_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016088_rage_phinst_setinstflag_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016164_rage_matrix34_dot_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016286_rage_pharticulatedbody_applygravitytolinks_float_vector_4_float(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: f32, a4: i32); }
extern "C" { pub fn f_016318_rage_pharticulatedcollider_applyangimpulseart_float_vector_4_int(ph: *mut Phantom, a0: i64, a1: u128, a2: i32); }
extern "C" { pub fn f_016323_rage_pharticulatedcollider_setangvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016333_rage_pharticulatedcollider_getlocalvelocityart_float_vector_4_in(ph: *mut Phantom, a0: i64, a1: i64, a2: u128, a3: i32); }
extern "C" { pub fn f_016367_rage_phbound_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016385_rage_phbound_istypecomposite_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_016391_rage_phboundbox_phboundbox_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016395_rage_phboundbox_setboxsize_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016436_rage_phboundcapsule_phboundcapsule(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016439_rage_phboundcapsule_setcapsulesize_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_016444_rage_phboundcapsule_getendpointa_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016445_rage_phboundcapsule_getendpointb_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016530_rage_phboundsphere_phboundsphere_float(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_016531_rage_phboundsphere_setsphereradius_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016602_rage_phverletcloth_applyimpulse_rage_atbitset_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016731_rage_pharticulatedbody_applyangimpulse_int_float_vector_4(ph: *mut Phantom, a0: i64, a1: i32, a2: u128); }
extern "C" { pub fn f_016886_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectlocation(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016892_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectlocation(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016907_rage_phlevelnodetree_rage_phlooseoctreenode_getfirstculledobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016908_rage_phlevelnodetree_rage_phlooseoctreenode_getnextculledobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016909_rage_phlevelnodetree_rage_phlooseoctreenode_testprobe_rage_phseg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_016910_rage_phlevelnodetree_rage_phlooseoctreenode_testsphere_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_016943_rage_scrthread_printstacktrace_void_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016944_rage_scrthread_preprintstacktrace(ph: *mut Phantom); }
extern "C" { pub fn f_016949_rage_scrthread_getactivethread(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_016952_rage_scrthread_getprogramcounter_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016985_rage_scrthread_checkifmonitoredscriptarraysizehaschanged_char_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018169_rage_mvparameterbuffer_generatefreecapacity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018887_cgamelogic_setrequestedlevelindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_018889_cleveldata_getfriendlyname_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_018921_rage_fwentity_getmodelid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018984_rage_vector3_isclose_rage_vector3_const_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_019239_cstuntjumpmanager_validatestuntjumpstats_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019289_caitarget_caitarget_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019293_caitarget_caitarget_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019295_caitarget_caitarget_caitarget_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019305_cscenarioblockingareas_addscenarioblockingarea_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_019306_cscenarioblockingareas_removescenarioblockingarea_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019336_cobjectscanner_scanforentitiesinrange_centity_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019350_cspatialarray_findnearsegment_rage_vec3v_const_ref_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_019450_char_ptr_rage_formatf_char_100ul_char_ref_100ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019451_ailogging_getbooleanasstring_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_019452_ailogging_getdynamicentityisclonestringsafe_cdynamicentity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019453_ailogging_getdynamicentitynamesafe_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019611_cfacialdatacomponent_playpainfacialanim_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019612_cfacialdatacomponent_playdyingfacialanim_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019697_cmoveobject_setnetwork_rage_fwmovenetworkplayer_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_019701_cmoveobject_clearnetwork_cmovenetworkhelper_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_019723_cclipeventtags_cpadshaketag_getpadshakeduration_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019724_cclipeventtags_cpadshaketag_getpadshakeintensity_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_021008_rage_atbinarymap_int_unsigned_int_datapair_ptr_std_2_partial_sor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_021009_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_021010_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_021145_audradiostation_maintainheist4spstationstate(ph: *mut Phantom); }
extern "C" { pub fn f_021147_audradiostation_reset(ph: *mut Phantom); }
extern "C" { pub fn f_021190_audradioaudioentity_findaudiblestation_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_021219_audscriptaudioentity_shouldduckforscriptedconversation(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021220_audnorthaudioengine_registerloudsound_rage_vector3_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021222_audsuperconductor_sendconductormessage_conductors_conductormessa(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021239_audnorthaudioengine_notifyopennetwork(ph: *mut Phantom); }
extern "C" { pub fn f_021240_audnorthaudioengine_notifyclosenetwork_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021349_audweaponaudiocomponent_getweaponsettings_centity_const_ptr_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_021351_cped_shouldbedead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021359_auddamagestats_dtorauddamagestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021394_audpedfootstepaudio_playdebugfootstepevent_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_021402_audpedaudioentity_triggershellcasing_audweaponaudiocomponent_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021409_audpedaudioentity_playmeleecombathit_unsigned_int_worldprobe_csh(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_021410_audpedaudioentity_playbikemeleecombathit_worldprobe_cshapetesthi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021414_audweaponaudiocomponent_playweaponfire_centity_ptr_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_021417_audweaponaudioentity_israygun_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021418_audpedaudioentity_upbodyfallforshooting(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021570_audradiostation_locktracklist_rage_radiostationtracklist_const_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_021571_audradiostation_getstation_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_021573_audradiostation_randomizesequentialstationplayposition(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021574_audradiostation_findstationwithimmutableindex_unsigned_int_audra(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_021608_audradiostation_findstation_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_021706_audradiostation_resethistoryfornetworkgame(ph: *mut Phantom); }
extern "C" { pub fn f_021707_audradiostation_syncstation_audradiostation_audradiostationsyncd(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021711_audradiostation_populatesyncdata_audradiostation_audradiostation(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021713_audradiostation_getfavouritedstations_unsigned_int_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021761_audspeechaudioentity_saywhensafe_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_021860_audscriptaudioentity_cleanupscriptaudioentityforreplay(ph: *mut Phantom); }
extern "C" { pub fn f_022089_audvehicleaudioentity_setradiostationfromnetwork_unsigned_char_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022103_audvehicleaudioentity_triggerdynamicexplosionsounds_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022126_audvehiclecollisionaudio_getvehiclecollisionmaterialsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022205_audvoicechataudioentity_removealltalkers(ph: *mut Phantom); }
extern "C" { pub fn f_022225_audweaponaudioentity_spinupweapon_centity_const_ptr_cweapon_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_022226_void_creplaymgrinternal_recordpersistantfx_cpacketweaponspinup_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_022227_audweaponaudiocomponent_stopcookprojectile(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022228_audweaponaudiocomponent_spindownweapon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022229_audweaponaudioentity_togglethermalvision_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_022231_audweaponaudioentity_triggerstickybombattatch_rage_vec3v_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: i64, a6: i32); }
extern "C" { pub fn f_022235_cpedinventory_getweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022238_audweaponaudioentity_playbulletby_unsigned_int_rage_vector3_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_022239_audweaponaudioentity_isrubberbulletgun_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_022240_audweaponaudioentity_isriotshotgun_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_022241_audweaponaudioentity_attachprojectileloop_unsigned_int_cobject_p(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_022242_audweaponaudioentity_attachprojectileloop_char_const_ptr_cobject(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_022244_audweaponaudioentity_addplayerequippedweapon_unsigned_int_unsign(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_022358_audvoicechataudioentity_dtoraudvoicechataudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022381_void_creplayeventmanager_storeevent_cpacketsoundphonering_cpacke(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022391_naenvironmentgroup_allocate_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022397_naaudioentity_createdeferredsound_rage_audmetadataref_centity_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_022398_naaudioentity_createdeferredsound_unsigned_int_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_022522_naenvironment_areweinaninterior_bool_ptr_float_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022612_audcollisionaudioentity_getmaterialoverride_centity_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_022645_audcollisionaudioentity_reportmeleecollision_worldprobe_cshapete(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_022646_audcollisionaudioentity_triggerdeferredmeleeimpact_worldprobe_cs(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32); }
extern "C" { pub fn f_022647_audcollisionaudioentity_handleentityswinging_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022656_audcollisionaudioentity_triggerdeferredbulletimpact_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: i32, a5: i64, a6: i64, a7: i32, a8: i32); }
extern "C" { pub fn f_022657_audcollisionaudioentity_triggerdeferredbulletimpact_rage_audsoun(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i64, a6: i64, a7: i32, a8: i32, a9: i64); }
extern "C" { pub fn f_022658_cped_isdead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022800_naenvironmentgroup_updatesourceenvironment_unsigned_int_centity(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_022830_naenvironment_registerdeafeningsound_rage_vector3_ref_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_022833_audexplosionaudioentity_playexplosion_phgtaexplosioninst_ptr_eex(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_022852_audfrontendaudioentity_playsound_rage_atnonfinalhashstring_char(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023396_cpacketevent_cpacketevent_ereplaypacketid_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023416_creplayropemanager_getattachedentitya_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_023417_creplayropemanager_getattachedentityb_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_023418_cpacketaddrope_cpacketaddrope_int_rage_vec3v_const_ref_rage_vec3(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32, a7: f32, a8: i32, a9: i32, a10: i32, a11: i32, a12: f32, a13: i32, a14: i32) -> i64; }
extern "C" { pub fn f_023420_cpacketdeleterope_cpacketdeleterope_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_023423_cpacketattachropetoentity_cpacketattachropetoentity_int_rage_vec(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_023544_cpacketweaponspinup_cpacketweaponspinup_bool_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023570_cpacketbloodfx_cpacketbloodfx_vfxcollinfo_s_const_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: f32, a6: i32, a7: i32, a8: i32) -> i64; }
extern "C" { pub fn f_023726_replayfilemanager_fixname_char_ref_256_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023846_creplaymgr_replayrecordsound_unsigned_int_rage_audsoundinitparam(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_023849_creplaymgr_replayrecordsound_unsigned_int_unsigned_int_rage_auds(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_023853_creplaymgr_replaypresucksound_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_023877_creplaymgr_recordobject_cobject_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023891_replaybuffermarkermgr_addmarker_unsigned_int_unsigned_int_marker(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_023906_creplaycontrol_isplayeroutofcontrol(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023927_void_creplayeventmanager_trackrecordingevent_unsigned_long_ctrac(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_023930_creplayeventmanager_shouldproceedwithpacket_cpacketevent_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023967_creplayinterfacemanager_ensureisbeingrecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023968_creplayinterfacemanager_ensurewillberecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024266_creplaymgrinternal_isawareofworldforrecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024270_creplaymgrinternal_preprocess(ph: *mut Phantom); }
extern "C" { pub fn f_024286_creplaymgrinternal_isplaybackflagset_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_024295_cweapon_cancelcooktimer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024296_creplaymgrinternal_cleanupplayback(ph: *mut Phantom); }
extern "C" { pub fn f_024304_creplaymgrinternal_getbuildinginfo_cphysical_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_024305_creplaymgrinternal_recordmapobject_centity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026166_caminterface_isrenderingdirector_cambasedirector_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026174_camgameplaydirector_getfirstpersonshootercamera_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026176_caminterface_getplayercontrolcamaimframe(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026185_camgameplaydirector_computeshakeamplitudeforexplosion_float_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64) -> f32; }
extern "C" { pub fn f_026186_camfactory_createobject_cambaseobjectmetadata_const_ref_rage_atn(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026190_caminterface_fadeout_int_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026205_cqueriableinterface_istaskcurrentlyrunning_int_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_026209_rage_vector3_normalizesafe_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026211_worldprobe_cshapetestfixedresults_unsigned_char_32_cshapetestfix(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026219_cdynamicentity_getboneindexfrombonetag_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026248_rage_vector3_normalizesafe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026286_camgameplaydirector_getthirdpersonaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026327_rage_matrix34_dot3x3transpose_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026329_worldprobe_cshapetestresults_dtorcshapetestresults(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026367_rage_imp_transform_imp34_rage_mat34v_ref_float_vector_4_float_ve(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: u128, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_026368_cpedintelligence_geteventoftype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_026648_camdebugdirector_getcamframe_cambasecamera_ptr_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026649_camdebugdirector_getcamframenonconst_cambasecamera_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026677_rage_atarray_camgameplaydirector_texplosionsettings_0_unsigned_s(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026714_camgameplaydirector_getfirstpersonpedaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026715_camgameplaydirector_isfirstpersonshooteraiming_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026719_cweaponinfo_const_ptr_cweaponinfomanager_getinfo_cweaponinfo_rag(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027304_camsyncedscenedirector_stopanimatingcamera_camsyncedscenedirecto(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_027305_camsyncedscenedirector_animatecamera_rage_atnonfinalhashstring_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_029101_cdebug_setversionnumber(ph: *mut Phantom); }
extern "C" { pub fn f_029311_cutscenemanager_shutdowncutscene(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029325_cdebugscene_focusentities_clear(ph: *mut Phantom); }
extern "C" { pub fn f_029335_cgamesessionstatemachine_reinit_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_029377_ctask_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029433_getorcreategroup_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029455_cdebugscene_tunewidgetdata_tunewidgetdata_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_029456_cdebugscene_queuetunewidget_cdebugscene_tunewidgetdata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029612_worldprobe_cshapetestdesc_setresultsstructure_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029613_worldprobe_cshapetestresults_freeintersections(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029726_camframe_setworldmatrix_rage_matrix34_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029752_void_creplaymgrinternal_stoptrackingfx_unsigned_long_ctrackedeve(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029778_char_ptr_rage_formatf_char_70ul_char_ref_70ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029779_cblockview_getblockname_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029780_cblockview_getblockuser_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029866_cdebugscene_focusentities_get_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029890_cdebugscene_focusentities_set_centity_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_029923_cdebugdrawstore_getstorage_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_029924_cdebugdrawstore_addline_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_029927_cdebugdrawstore_addsphere_rage_vec3v_const_ref_float_rage_color3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_029931_cdebugdrawstore_addobb_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_029965_rage_fwtransform_getb_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029966_rage_fwtransform_getc_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029967_cphysical_gethealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_030309_rage_atarray_centityadd_0_unsigned_short_op_rage_atarray_centity(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030526_cdebugtextureviewerinterface_selecttxd_ctxdref_const_ref_bool_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_030565_ctxdref_ctxdref_eassettype_int_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_030722_ceventgroupped_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030723_cevent_affectsped_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_030736_ceventgroupglobal_addorcombine_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030739_cpedintelligence_addevent_cevent_const_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030741_geteventglobalgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030744_ceventnetwork_spewtotty_impl_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030746_geteventscriptnetworkgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030761_cpedintelligence_gettaskactive_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030770_cshockingeventsmanager_add_ceventshocking_ref_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_030772_ceventshocking_dtorceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030783_ceventdecisionmakermanager_getdecisionmaker_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_030801_cevent_cevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030809_cevent_communicateevent_cped_ptr_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_030814_cpeddamagecalculator_cpeddamagecalculator_centity_const_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_030815_cpeddamagecalculator_applydamageandcomputeresponse_cped_ptr_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32); }
extern "C" { pub fn f_030820_ceventrequesthelp_ceventrequesthelp_cped_ptr_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_030822_ceventrequesthelp_dtorceventrequesthelp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030824_cpedvariationinfocollection_hascomponentflagsset_epedvarcomp_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_030828_ceventdamage_ceventdamage_centity_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_030829_ceventdamage_dtorceventdamage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030845_ceventagitated_ceventagitated_cped_ptr_agitatedtype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_030846_ceventagitated_dtorceventagitated(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030902_ceventpotentialblast_ceventpotentialblast_caitarget_const_ref_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i64; }
extern "C" { pub fn f_030922_ceventnetworkfireddummyprojectile_ceventnetworkfireddummyproject(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_030931_ceventnetworkcashtransactionlog_ceventnetworkcashtransactionlog(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_031023_ceventexplosion_ceventexplosion_rage_vec3v_const_ref_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_031024_ceventexplosion_dtorceventexplosion(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031069_ceventgivepedtask_ceventgivepedtask_int_rage_aitask_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_031070_ceventgivepedtask_dtorceventgivepedtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031125_ceventshouttargetposition_ceventshouttargetposition_cped_ptr_cpe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031158_ceventfirenearby_dtorceventfirenearby(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031169_ceventswitch2nm_ceventswitch2nm_unsigned_int_rage_aitask_ptr_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_031170_ceventswitch2nm_dtorceventswitch2nm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031235_ceventshocking_ceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031242_ceventshocking_set_rage_vec3v_const_ref_centity_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_031389_ceventgunshot_ceventgunshot_centity_ptr_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32) -> i64; }
extern "C" { pub fn f_031391_ceventgunshot_dtorceventgunshot_53962(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031404_ceventgunshotwhizzedby_ceventgunshotwhizzedby_centity_ptr_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_031406_ceventgunshotwhizzedby_isbulletinrange_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_031409_ceventgunshotbulletimpact_ceventgunshotbulletimpact_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_031427_ceventfriendlyfirenearmiss_ceventfriendlyfirenearmiss_centity_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_031650_ceventexplosionheard_getsourcepos_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031689_ceventpotentialblast_dtorceventpotentialblast(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031888_ceventgunshotbulletimpact_dtorceventgunshotbulletimpact(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033790_gfxvalue_objectinterface_objectrelease_gfxvalue_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034930_cloadingscreens_committompsp_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034945_cloadingscreens_onprofilechange(ph: *mut Phantom); }
extern "C" { pub fn f_034946_cloadingscreens_areactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034949_chudtools_getwidescreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034958_chudtools_adjustforwidescreen_ewidescreen_format_rage_vector2_pt(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_034969_cwarningmessage_data_data(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035039_chudcolour_getcolourfromkey_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035040_chudcolour_getrgb_ehud_colours_unsigned_char(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_035082_cnewhud_isweaponwheelactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035084_cnewhud_populateweaponsfromwheelmemory_rage_atrangearray_rage_at(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035182_cmultiplayergamertaghud_cmultiplayergamertaghud(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035184_cmultiplayergamertaghud_dtorcmultiplayergamertaghud(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035276_cminimap_createsonarblip_rage_vector3_const_ref_float_ehud_colou(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_035277_cminimap_createsonarblipandreportstealthnoise_cped_ptr_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32) -> f32; }
extern "C" { pub fn f_035282_char_ptr_rage_formatf_char_50ul_char_ref_50ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_035294_cminimap_setminimapmodestate_esetup_state_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_035296_cminimap_reinit(ph: *mut Phantom); }
extern "C" { pub fn f_035304_cminimap_switchoffwaypoint_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035345_cminimap_updatewaypoint(ph: *mut Phantom); }
extern "C" { pub fn f_035346_cminimap_getactivewaypointid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035349_cminimap_setcurrentgolfmap_egolf_course_holes(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035354_cminimap_switchonwaypoint_float_float_unsigned_short_bool_bool_e(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_035355_cminimap_marklocallyowned_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035357_cminimap_isactivewaypoint_rage_vector2_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_035487_cbusyspinner_off_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035494_cbusyspinner_on_char_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_035727_cpartyclandata_dtorcpartyclandata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035739_rage_atarray_baseplayercarddatamanager_gamerdata_0_unsigned_shor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035825_ccontextmenuhelper_initvotes(ph: *mut Phantom); }
extern "C" { pub fn f_035913_cweaponwheel_getholsterammoinclip(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035919_cweaponwheel_setholsterammoinclip_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035920_cweaponwheel_setweaponinhand_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035973_ccrewmembersclandata_dtorccrewmembersclandata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036031_cscaleformmgr_requestremovemovie_int_emovieownertags(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_036033_cscaleformmgr_rendermovie_int_float_bool_bool_float(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i32, a4: f32); }
extern "C" { pub fn f_036034_cscaleformmoviewrapper_isactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036038_cscaleformmgr_ismovieactive_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036044_cscaleformmgr_beginmethodinternal_int_escaleform_base_class_char(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_036045_cscaleformmgr_endmethodinternal_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036048_cscaleformmgr_beginmethod_int_escaleform_base_class_char_const_p(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_036050_cscaleformmgr_endmethod_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036051_cscaleformmgr_addparambool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036053_cscaleformmgr_addparamint_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036056_cscaleformmgr_addparamstring_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036064_cscaleformmoviewrapper_removemovie(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036145_cscaleformmoviewrapper_createmovieandwaitforload_escaleform_base(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_036154_cpausemenu_close(ph: *mut Phantom); }
extern "C" { pub fn f_036162_cscaleformmoviewrapper_param_addparam(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036169_cplayercardmanager_cplayercardmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036175_cprofilesettings_set_cprofilesettings_profilesettingid_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036197_cwarningmessage_setmessage_cwarningmessage_data_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036198_cwarningmessage_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036227_cfriendclandata_dtorcfriendclandata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036230_cwarningmessage_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036247_creportmenu_creportmenu(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036252_cpausemenu_getlanguagefromsystemlanguage(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036254_cpausemenu_isactive_pm_activeflags(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036255_cpausemenu_backoutofsavegamelist(ph: *mut Phantom); }
extern "C" { pub fn f_036261_rage_atarray_rage_atstring_const_ptr_0_unsigned_short_insert_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_036265_cpausemenu_getcurrentscreen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036267_cpausemenu_iscurrentscreenvalid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036292_suicontexts_isactive_rage_athashvalue(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036301_cwarningscreen_checkallinput_bool_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_036305_cpausemenu_closedisplaycalibrationscreen(ph: *mut Phantom); }
extern "C" { pub fn f_036313_cscaleformmgr_forcemovieupdateinstantly_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036314_cwarningscreen_setmessagewithheader_ewarningmessagestyle_rage_at(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_036364_cprofilesettings_getint_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036372_cscaleformmgr_changemovieparams_int_rage_vector2_const_ref_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_036373_cscaleformmgr_setmoviedisplayconfig_int_escaleform_base_class_cs(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_036378_cpausemenu_issp(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036380_cpausemenu_islobby(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036382_cprofilesettings_exists_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036383_cpausemenu_getlanguagepreference(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036386_cprofilesettings_write_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036396_cpausemenu_setcurrentmissiondescription_bool_char_const_ptr_char(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_036429_basesocialclubmenu_basesocialclubmenu(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036435_basesocialclubmenu_setbuttons_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036481_socialclubmenu_setgenericerror_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_036485_basesocialclubmenu_flagfordeletion(ph: *mut Phantom); }
extern "C" { pub fn f_036616_ctextinputbox_removetextinputboxmovie(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036627_cwarningscreen_setmessage_ewarningmessagestyle_rage_atnonfinalha(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_036654_uiworldicon_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036655_uiworldicon_dtoruiworldicon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036658_uiworldiconmanager_uiworldiconmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036680_metricbaseinfo_metricbaseinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036681_metricbasechar_metricbasechar(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036708_cstorescreenmgr_checkoutcommerceproduct_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036926_cvideoeditormenu_rebuildmenu(ph: *mut Phantom); }
extern "C" { pub fn f_036927_cvideoeditormenu_setuserconfirmationscreenwithaction_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_036930_cvideoeditorui_close_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036933_cvideoeditorui_closeinternal(ph: *mut Phantom); }
extern "C" { pub fn f_036937_cvideoeditormenu_checkforuserconfirmationscreens(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036946_cvideoeditormenu_setuserconfirmationscreen_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_037047_cvideoeditormenu_isongallerymenu(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037100_cvideoeditormenu_isongalleryfilteringmenu(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037258_cscaleformmoviewrapper_setdisplayconfig_cscaleformmgr_sdc(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037259_cscaleformmgr_createmovieandwaitforload_char_const_ptr_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_037286_cscaleformmgr_getrequiredscalemode_int_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037315_cscaleformmgr_updatemovieparams_int_gfxmovieview_scalemodetype(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037317_cscaleformmgr_callmethod_int_escaleform_base_class_char_const_pt(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_037383_gametransactionsessionmgr_gametransactionsessionmgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037386_metricbaseinfo_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_037387_metricbasechar_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_037409_cscriptedcoronaplayercarddatamanager_dtorcscriptedcoronaplayerca(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037501_cgamestream_flushqueue_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037507_cgamestream_postmessagetext_char_const_ptr_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64, a8: f32, a9: i64, a10: i32, a11: i32, a12: i32) -> i32; }
extern "C" { pub fn f_037511_cgamestream_postticker_char_const_ptr_bool_bool_bool_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_037514_cgamestream_postcrewtag_bool_bool_char_const_ptr_int_bool_char_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i64, a8: i64, a9: i32) -> i32; }
extern "C" { pub fn f_037524_cgamestream_isscripthidingthisframe_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037642_ccheat_vehiclecheat_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037657_cwanted_setwantedlevel_rage_vector3_const_ref_int_int_setwantedl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_037661_cmodelinfo_requestassets_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037662_cmodelinfo_haveassetsloaded_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037664_time_time_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_037676_cclock_settime_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_037677_cclock_resetdebugmodifiedinnetworkgame(ph: *mut Phantom); }
extern "C" { pub fn f_037682_cclock_setdate_int_date_month_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_037683_cclock_adddays_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037685_cclock_fromdayratio_float_int_ref_int_ref_int_ref(ph: *mut Phantom, a0: f32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_037761_ccrime_reportcrime_ecrimetype_centity_ptr_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_037795_cthepopmultiplierareas_createpopmultiplierarea_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_037796_cthepopmultiplierareas_createpopmultiplierarea_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_037797_cthepopmultiplierareas_removepopmultiplierarea_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: f32, a5: i32); }
extern "C" { pub fn f_037799_cthepopmultiplierareas_getlastactiveareaindex_debugonly(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037800_cthepopmultiplierareas_getactivearea_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_037888_cseatmanager_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_037905_cwanted_shouldcompensateforslowmovingvehicle_centity_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_037934_cweather_gettypeindex_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037944_cweather_stopoverriding(ph: *mut Phantom); }
extern "C" { pub fn f_037945_cweather_forcetypenow_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037946_cweather_getprevtype(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037947_cweather_forceweathertypeovertime_unsigned_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_037953_cweather_overridetype_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037955_cweather_networkinitfromglobal_int_unsigned_int_unsigned_int_boo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_037956_cweather_getfromratio_float_int_ref_unsigned_int_ref(ph: *mut Phantom, a0: f32, a1: i64, a2: i64); }
extern "C" { pub fn f_038181_corder_getincident_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038327_cincident_getnetworkobject_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038328_cincident_getnetworkobjectidfromincidentid_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_038360_cincidentmanager_cincidentptr_clear_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038361_cincidentmanager_cincidentptr_copy_cincident_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038362_cincidentmanager_cincidentptr_createincident_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_038363_cincidentmanager_cincidentptr_getnetworkobject_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038364_cincidentmanager_clearallincidents_bool(ph: *mut Phantom); }
extern "C" { pub fn f_038367_cincidentmanager_getincidentfromincidentid_unsigned_int_unsigned(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_038369_cordermanager_corderptr_copy_corder_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038370_cordermanager_corderptr_createorder_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_038371_cordermanager_corderptr_getnetworkobject_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038372_cordermanager_clearallorders_bool(ph: *mut Phantom); }
extern "C" { pub fn f_038440_backgroundscripts_hascloudscripts(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038490_cikmanager_resetallsolvers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038493_cikmanager_issupported_ikmanagersolvertypes_flags_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038497_cikmanager_islooking(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038528_cikmanager_request_cikrequestbodyreact_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038543_cikrequestbodyreact_cikrequestbodyreact_rage_vec3v_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_038715_cmodelinfo_requestassets_cbasemodelinfo_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038719_cmodelinfo_setassetrequiredflag_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038720_cmodelinfo_clearassetrequiredflag_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038751_cseatmanager_getpedsseatindex_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038757_cseatmanager_getlastpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038851_cpedmodelinfo_isusingfemaleskeleton_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038911_cvehiclemodelinfo_loadresidentobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_039211_cvehiclevariationinstance_getkit_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039275_cweaponmodelinfo_addtohdinstancelist_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039276_cweaponmodelinfo_removefromhdinstancelist_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039561_cobject_cobject_eentityownedby_unsigned_int_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_039565_cobject_dtorcobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039593_cobject_objecthasbeenmoved_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_039620_cobject_processphysics_float_bool_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_039623_cobject_onactivate_rage_phinst_ptr_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_039625_cobject_processcollision_rage_phinst_const_ptr_centity_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: f32, a7: i64, a8: i32, a9: i32, a10: i64, a11: i32, a12: i32); }
extern "C" { pub fn f_039629_cobject_processcontrol(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039630_cweapon_sfireparams_dtorsfireparams(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039633_cobject_objectdamage_float_rage_vector3_const_ptr_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_039655_cdoorsystem_init(ph: *mut Phantom); }
extern "C" { pub fn f_039656_cdoorsystem_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_039688_cobjectpopulation_destroyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039715_cobject_setweapon_cweapon_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039716_cobject_setweaponcomponent_cweaponcomponent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039719_cobject_settask_rage_aitask_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_039720_cobject_addtoorremovefromprocesspostcameraarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039744_cobject_scaleobject_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_039761_cobjectpopulation_createobject_rage_fwmodelid_eentityownedby_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_040024_rage_normalizeandmag_rage_vector3_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_040143_cpedgroup_setdirty(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040165_cpedgroupmembership_flush_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_040167_cpedgroupmembership_op_cpedgroupmembership_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040176_cpedgroupmembership_copynetworkinfo_cpedgroupmembership_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040178_cpedgroup_cpedgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040179_cpedgroup_dtorcpedgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040185_cpedgroup_giveeventtoallmembers_cevent_ref_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040186_cpedgroup_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040187_cpedgroup_getnetworkobject_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040191_cpedgroups_removegroup_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_040192_cpedgroups_init(ph: *mut Phantom); }
extern "C" { pub fn f_040194_cpedgroups_getpedsgroup_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040220_worldprobe_cshapetesthitpoint_gethitinst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040323_cactionmanager_findactionresult_unsigned_int_ref_unsigned_int_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_040330_cactionmanager_arepedsfriendlywitheachother_cped_const_ptr_cped(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040343_cactiondefinition_getactionresult_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_040348_cactionmanager_findactiondefinition_unsigned_int_ref_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_040349_cactionmanager_shoulddebugped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040350_cactionmanager_selectsuitableaction_rage_atfixedbitset_12_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i64, a8: i32, a9: i32, a10: i32) -> i64; }
extern "C" { pub fn f_040404_rage_fraginstnm_postartmessage_char_const_ptr_art_messageparamsb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040407_rage_fraginstnm_setweaponmode_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_040409_art_messageparamsbase_addbool_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040410_art_messageparamsbase_addfloat_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_040423_rage_fraginstnm_mapragdolllodcomponentcurrenttohigh_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_040424_rage_fraginstnm_mapragdolllodcomponentcurrenttohigh_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_040425_rage_fraginstnm_mapragdolllodcomponenthightocurrent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_040500_art_messageparamsbase_addint_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040501_art_messageparamsbase_addvector3_char_const_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_040503_art_messageparamsbase_messageparamsbase_art_messageparamsbase_pa(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_040504_art_messageparamsbase_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041056_findplayerped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041063_cpedintelligence_isfriendlywith_cped_const_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041065_cplayerpedtargeting_tunables_gettargettinginfo_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041071_cpedintelligence_getclosestvehicleinrange_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041113_cpedintelligence_getlowestlevelnmtask_cped_ptr_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041116_cpedintelligence_flushimmediately_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041117_cpedintelligence_addtoeventhistory_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041122_svehiclemissionparams_dtorsvehiclemissionparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041186_cpedpopulation_removeallrandompeds(ph: *mut Phantom); }
extern "C" { pub fn f_041195_cpedintelligence_cleartasks_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041203_cpedintelligence_getrelationshipgroup_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041218_cplayerinfo_getphysicalplayerindex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041237_cpedintelligence_gettaskmelee_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041240_cpedintelligence_gettaskgun_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041263_rage_atbinarymap_ctasknmsimple_tunables_tuning_rage_atnonfinalha(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041265_cplayerinfo_isaiming_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041268_cinterestingevents_add_cinterestingevents_etype_centity_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_041281_cpedtype_getpedtypenamefromid_epedtype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_041371_cpediksettingsinfo_isikflagdisabled_ikmanagersolvertypes_flags_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041381_cpedintelligence_getcurrenttarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041385_cpedintelligence_findtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041387_cpedintelligence_gettaskmeleeactionresult_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041420_cpedintelligence_recalculateclonetasks(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041423_cpedintelligence_increasebattleawareness_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_041454_cpedmotiondata_setcurrentmoveblendratio_rage_vector2_ref_float_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_041496_findplayervehicle_cplayerinfo_ptr_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041529_cpedpopulation_addpedtopopulationcount_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041531_cpedpopulation_startspawningpeds(ph: *mut Phantom); }
extern "C" { pub fn f_041556_cpopgrouplist_findpedgroupfromnamehash_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_041565_cpopschedule_setoverridepedgroup_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041589_cexplosionmanager_cexplosionargs_dtorcexplosionargs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041649_cplayerdamageovertime_doteffect_dtordoteffect(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041676_cplayerpedtargeting_getisfreeaimassisttargetposwithinconstraints(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_041678_cplayerinfo_getfiringattackvalue(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041682_cpedweaponmanager_getequippedweaponhasfirstpersonscope_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041702_findfollowped_cplayerinfo_ptr(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041705_cplayerinfo_setplayerstate_cplayerinfo_state(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041726_cplayerinfo_getplayermeleedamagemodifier_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_041746_reportplayerresetmismatch_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_041798_cpopschedulelist_getnamefromindex_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_041864_cpedaccuracy_calculateaccuracyrange_cped_const_ref_cweaponinfo_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64); }
extern "C" { pub fn f_041933_cplayerpedtargeting_getonfoothominglockonstate_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_042006_cqueriableinterface_getnetworksynchronisedsceneid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042013_cqueriableinterface_gettasknetsequencefortype_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042019_cqueriableinterface_cantaskbegiventononlocalpeds_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_042040_crelationshipmanager_findrelationshipgroup_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_042398_cpedvariationdata_getcomponentslotfrombonetag_eanimbonetag(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_042410_cpedheadblenddata_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042432_cpedvariationinfocollection_lookupsecondaryflags_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_042484_char_ptr_rage_formatf_char_200ul_char_ref_200ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042618_cpedpropsmgr_knockoffprops_cped_ptr_bool_bool_bool_bool_rage_vec(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_042646_cpedpropsmgr_checkpropflags_cped_ptr_eanchorpoints_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_042656_rage_strrequestarray_6u_dtorstrrequestarray(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042789_cpickup_isalwaysfixed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042790_cwatertesthelper_getriverheightatposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_042793_cpickup_addexplosion_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042797_cweaponitem_getisstreamedin(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042804_cpickup_collectonshot_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042810_cpickup_cancollect_cped_const_ptr_cpickup_ecollectiontype_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_042817_worldprobe_debugfindgroundzfor3dcoord_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32, a5: i64, a6: i64) -> f32; }
extern "C" { pub fn f_042818_worldprobe_cshapetestmanager_debugsubmittest_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_042821_worldprobe_cshapetestmanager_aborttest_worldprobe_cshapetestresu(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042836_cpedinventory_getweaponbyslot_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042848_worldprobe_cshapetestdesc_setexcludeentities_centity_const_ptr_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_042878_cpickupmanager_removeallpickups_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_042883_cpickupmanager_sessionreset(ph: *mut Phantom); }
extern "C" { pub fn f_042885_cpickupmanager_createpickup_unsigned_int_rage_matrix34_const_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_042888_cpickupmanager_createpickupfromcurrentweapon_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042890_cpickupmanager_shouldusemppickups_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043046_ccollisionhistory_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043085_worldprobe_cshapetestdesc_setexcludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043096_phinstgta_phinstgta_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043153_fraginstnmgta_applybulletforce_cped_ptr_float_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_043242_cphysics_getmeasuringtoolpos_int_rage_vector3_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_043284_cflightmodelhelper_cflightmodelhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043291_cflightmodelhelper_debugdrawcontrolinputs_cphysical_const_ptr_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_043334_cwatertesthelper_getwaterlevelincludingrivers_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_043346_worldprobe_cshapetestdesc_setincludeinstance_rage_phinst_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043349_worldprobe_cshapetestdesc_setexcludeinstance_rage_phinst_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043654_postfx_bulletimpactoverlay_registerbulletimpact_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_043677_animpostfxmanager_start_rage_atnonfinalhashstring_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_043815_lights_addscenelight_clightsource_ref_clightentity_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043983_csprite2d_drawrect_float_float_float_float_float_rage_color32_co(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i64); }
extern "C" { pub fn f_044122_water_getwaterlevel_rage_vector3_const_ref_float_ptr_bool_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_044125_water_testlineagainstwater_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_044177_water_getcamerawaterdepth(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_044607_cparaboloidshadow_updatelight_clightsource_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_044787_renderphaseseethrough_updatevisualdatasettings(ph: *mut Phantom); }
extern "C" { pub fn f_044889_creplaycoordinator_isactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045014_creplayvideometadatastore_getmetadata_char_const_ptr_creplayvide(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_045375_csavegamequeue_containsasingleplayersave(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045789_cped_ismale_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045795_cped_checkagilityflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045797_cped_getcurrentmotiontask_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045806_cped_clearwetclothing(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045840_cped_getbonematrix_rage_matrix34_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045842_cped_forcemotionstatethisframe_cpedmotionstates_emotionstate_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_045852_cped_resetdesiredmainmovercapsuledata_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045857_cped_decrementstickycount(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045882_cped_switchtoragdollinternal(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045884_cped_knockpedoffvehicle_bool_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_045885_cped_switchtoragdoll_cevent_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045887_rage_fwentity_setnocollision_rage_fwentity_ptr_unsigned_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_045889_cped_getprimarymotiontask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045903_cped_getiscrouching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045909_cped_getprimarymotiontaskifexists_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045911_cped_wantstouseactionmode_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045913_cped_isfirstpersonshootermodeenabledforplayer_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045922_cped_getboneposition_rage_vector3_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045926_cped_getisdrivingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045944_cped_isbonelightlyarmoured_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045968_cped_gettaskdata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045971_cped_getscenariopoint_cdynamicentity_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045996_cped_getbonetagfromragdollcomponent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045997_cped_convertbonetobodypart_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_046002_cped_isinfirstpersonvehiclecamera_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_046012_cped_setclothpinradiusscale_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046054_cped_setusingactionmode_bool_cped_actionmodeenabled_float_bool_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_046099_cped_getragdollcomponentmatrix_rage_matrix34_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_046103_cped_getspecialability_eplayerspecialabilityslot(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046112_cped_canbeinhurt_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046116_cped_estimatepose_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046121_cped_gethasjetpackequipped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046168_cped_isprone_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046170_cped_isallowedtodamageentity_cweaponinfo_const_ptr_centity_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_046173_cped_getmaxtimeunderwater_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_046176_cped_getcontrolfromplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046180_cped_instantaiupdate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046183_cped_getragdollcomponentfrombonetag_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046184_cped_getiksettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046207_cpedvfx_getbloodvfxdisabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046217_cped_getmovementmodedataforweaponhash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046232_cped_usefirstpersonupperbodyanims_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046302_cdynamicentity_createskeleton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046303_cdynamicentity_createanimdirector_rage_rmcdrawable_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_046305_cdynamicentity_addtosceneupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046316_cbuilding_cbuilding_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046325_cgameworld_findfollowplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_046327_cplayerswitchinterface_isactive_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_046364_char_ptr_rage_safecat_char_char_4_256ul_char_ref_256ul_char_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046368_cdatafilemgr_finddatafile_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_046376_ctexturedownloadrequestdesc_ctexturedownloadrequestdesc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046379_cdownloadabletexturemanager_requesttexturedownload_int_ref_ctext(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_046383_cdownloadabletexturemanager_releasetexturedownloadrequest_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046384_cdownloadabletexturemanager_gettexturedownloadrequestresultcode(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046385_cdownloadabletexturemanager_isrequestpending_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046386_cdownloadabletexturemanager_isrequestready_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046387_cdownloadabletexturemanager_hasrequestfailed_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046388_cdownloadabletexturemanager_getrequesttxdslot_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046404_cportaltracker_setloadscollisions_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046405_cportaltracker_addtoactivatingtrackerlist_cportaltracker_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046410_centity_getinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046427_cdynamicentity_removesceneupdateflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046438_cgameworld_add_centity_ptr_rage_fwinteriorlocation_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046445_cportaltracker_collectstack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046462_cportaltracker_removefromactivatingtrackerlist_cportaltracker_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046467_cgameworld_remove_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046484_centity_setisvisibleformodule_eisvisiblemodule_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046497_centity_setphysicsinst_rage_phinst_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046499_centity_getcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046502_centity_getphysarch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046517_centity_processfxentityshot_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_046531_centity_getisvisibleformodule_eisvisiblemodule_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046545_centity_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046599_centityiterator_centityiterator_int_centity_const_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_046600_centityiterator_getnext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046672_cextracontentmanager_executetitleupdatedatapatch_unsigned_int_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046687_cextracontentmanager_getcontentbyhash_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046695_ctexlod_flushallupgrades(ph: *mut Phantom); }
extern "C" { pub fn f_046710_cextracontentmanager_executetitleupdatedatapatchgroup_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046716_cextracontentmanager_getcrc_unsigned_int_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046793_ctexlod_shutdownsession_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047012_cfocusentitymgr_getpos(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047044_cgameworld_forallentitiesintersecting_rage_fwsearchvolume_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_047054_cphysical_setisinwater_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047060_cphysical_activatephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047062_cphysical_setfixedphysics_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_047073_cphysical_sethealth_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047075_cphysical_setweapondamageinfo_centity_ptr_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_047076_cphysical_getweapondamagedtimebyentity_centity_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_047087_cphysical_setposition_rage_vector3_const_ref_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_047089_cphysical_setmatrix_rage_matrix34_const_ref_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_047093_vfxcollinfo_s_dtorvfxcollinfo_s(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047100_cphysical_applyimpulse_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047103_cphysical_applyangimpulse_rage_vector3_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047105_cphysical_applyexternalimpulse_rage_vector3_const_ref_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: f32); }
extern "C" { pub fn f_047106_cphysical_getlocalspeed_rage_vector3_const_ref_bool_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047108_cphysical_processpostphysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047117_cphysical_debugattachtoworldbasic_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_047126_cphysical_debugprocessallattachments_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_047305_cgameworld_getmainplayerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047743_cinteriorinst_addsmoketoroom_int_rage_vector3_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_047779_cinteriorproxy_dumpinteriorproxyinfo(ph: *mut Phantom); }
extern "C" { pub fn f_047880_cgameworld_findlocalplayercoors(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047982_cgameworld_findlocalplayervehicle_bool(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047998_cgameworld_findlocalplayerspeed(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_048339_networkutils_isnetworkcloneormigrating_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_054790_network_commands_commandnetworkisactivitysession(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_054827_rage_atarray_rage_rlgamerhandle_0_unsigned_short_grow_unsigned_i(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_054887_cped_isfatallyinjured_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_055218_network_commands_commandopencommercestore_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_055233_network_commands_commandtriggerplayercrchackercheck_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_055234_network_commands_commandtriggertuningcrchackercheck_int_char_con(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_055235_network_commands_commandtriggerfilecrchackercheck_int_char_const(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_055354_network_commands_commandshouldwarnofsimplemodcheck(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_056465_object_commands_objectcreationfunction_int_float_float_float_boo(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_058506_void_creplaymgrinternal_recordpersistantfx_cpacketaddrope_unsign(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_058522_void_creplaymgrinternal_recordpersistantfx_cpacketdeleterope_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_058533_void_creplaymgrinternal_recordpersistantfx_cpacketattachropetoen(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_059433_rage_atfixedarray_rage_rlgamerhandle_32_atfixedarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_060775_task_commands_commonusenearestscenariotopos_cped_ptr_rage_scrvec(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_061626_cvehicledeformation_lockdamagetexture_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_063762_cthescripts_gtastartnewthreadoverride_char_const_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_063787_cthescripts_shouldbeprocessed(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_063797_cthescripts_getcurrentgtascripthandlernetwork(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063800_cthescripts_getcurrentscriptnameandprogramcounter(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063835_cscriptglobaltamperchecker_uncheckedscope_uncheckedscope(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_063836_cscriptglobaltamperchecker_uncheckedscope_dtoruncheckedscope(ph: *mut Phantom); }
extern "C" { pub fn f_063862_cscriptareas_ispointinangledarea_rage_vector3_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_063903_rage_fweventqueue_16_void_flusheventsoftype_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064139_cstreamedscripts_calculatescriptstorefingerprint_unsigned_int_bo(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064143_cgamescripthandlermgr_getremotescriptinfo_cgamescriptid_const_re(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_064213_cgamescriptid_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064214_cgamescriptid_read_rage_datbitbuffer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064215_cgamescriptid_generatescriptidhash(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064216_cgamescriptid_write_rage_datbitbuffer_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064235_cgamescripthandlermgr_isnetworkhost_cgamescriptid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064252_cscriptresource_pedgroup_cscriptresource_pedgroup_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_064425_cgamescriptresource_dtorcgamescriptresource_102634(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064487_ragesecreportcache_getinstance(ph: *mut Phantom); }
extern "C" { pub fn f_064488_ragesecreportcache_hasbeenreported_ragesecreportcache_ereportcac(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064489_ragesecreportcache_addtoreported_ragesecreportcache_ereportcache(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064490_ragesecreportcache_clear(ph: *mut Phantom); }
extern "C" { pub fn f_064493_ragesecpopreaction(ph: *mut Phantom); }
extern "C" { pub fn f_064588_ccustomshadereffecttint_selecttintpalette_unsigned_char_rage_fwe(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064589_ccustomshadereffecttint_getmaxtintpalette(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064646_ccustomshadereffectvehicle_setcustomprimarycolor_rage_color32(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064663_ccustomshadereffectweapon_copysettings_ccustomshadereffectweapon(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064736_csettingsmanager_csettingsmanager(ph: *mut Phantom); }
extern "C" { pub fn f_064737_csettingsmanager_getsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064774_cloadedmodelgroup_countmembers_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064775_cloadedmodelgroup_getmember_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_064785_cpopulationstreaming_setnetworkmodelvariationid_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064803_cpopulationstreaming_isnetworkmodeloffsetlower_unsigned_int_unsi(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064832_cstreaming_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064913_ckeyboard_getkeyjustdown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_064925_cstreamingdebug_displayobjectsinmemory_rage_fistream_ptr_bool_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_065178_streamingmemoryresult_dtorstreamingmemoryresult(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065445_ccontrolmgr_setmainplayerindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_065451_ccontrolmgr_stoppadsshaking(ph: *mut Phantom); }
extern "C" { pub fn f_065461_ccontrolmgr_getmainfrontendcontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065469_ccontrolmgr_startplayerpadshakebyintensity_unsigned_int_float_in(ph: *mut Phantom, a0: i32, a1: f32, a2: i32); }
extern "C" { pub fn f_065609_cfilemgr_getassetsfolder(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_065614_cfilemgr_readline_void_ptr_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_065615_cfilemgr_openfile_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_065617_cfilemgr_setdir_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065621_cfilemgr_openfileforwriting_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065622_cfilemgr_openfileforappending_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065633_memorycheck_setaddressstart_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_065634_memorycheck_getversionandtype(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065635_memorycheck_getaddressstart(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065636_memorycheck_getsize(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065637_memorycheck_getvalue(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065638_memorycheck_getflags(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065875_ctasksynchronizedscene_ctasksynchronizedscene_short_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32, a5: f32, a6: i64, a7: i64, a8: f32, a9: i32, a10: i64) -> i64; }
extern "C" { pub fn f_066010_ctaskmelee_ctaskmelee_cactionresult_const_ptr_centity_ptr_rage_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_066082_ctaskanimatedhitbyexplosion_ctaskanimatedhitbyexplosion_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_066089_ctaskdyingdead_setdeathanimationbyset_rage_fwmvtemplateid_rage_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_066096_ctaskcover_ctaskcover_cweapontarget_const_ref_int_ctaskcover_efo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_066109_ctaskmelee_settargetentity_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_066206_ctaskmelee_setlocalreaction_cactionresult_const_ptr_bool_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_066251_ctaskmeleeactionresult_addtohitentitycount_centity_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_066252_ctaskmeleeactionresult_processposthitresults_cped_ptr_cped_ptr_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32); }
extern "C" { pub fn f_066297_cdeathsourceinfo_cdeathsourceinfo_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_066298_cdeathsourceinfo_cdeathsourceinfo_centity_ptr_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_066299_ctaskdyingdead_ctaskdyingdead_cdeathsourceinfo_const_ptr_int_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_066306_ctaskdyingdead_setforcednaturalmotiontask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066338_ctaskcomplexonfire_ctaskcomplexonfire_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_066351_ctaskdamageelectric_ctaskdamageelectric_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_066386_cdamageinfo_cdamageinfo_unsigned_int_rage_vector3_const_ptr_worl(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: f32, a6: i64) -> i64; }
extern "C" { pub fn f_066632_ctaskhandsup_ctaskhandsup_int_cped_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_066728_ctaskcover_getshouldusecustomlowcoveranims_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067955_ctaskmobilephone_isrunningmobilephonetask_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068772_ctaskmotionaiming_usinglefttriggeraimmode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069095_ctaskgetup_shouldabortexternal_rage_aievent_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069531_ctaskparachute_isparachutepackvariationactiveforped_cped_const_r(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069960_ctaskfallover_computefallcontext_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_069961_ctaskfallover_pickfallanimation_cped_ptr_ctaskfallover_efallcont(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_069979_ctaskfallandgetup_ctaskfallandgetup_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_070396_ctasknmbehaviour_wantstoragdoll_cped_ptr_eragdolltriggertypes_ce(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> f32; }
extern "C" { pub fn f_070397_ctasknmbehaviour_calcragdollmultiplierscore_cped_ptr_eragdolltri(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_070398_ctasknmbehaviour_canuseragdoll_cped_ptr_eragdolltriggertypes_flo(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_070399_ctasknmcontrol_ctasknmcontrol_unsigned_int_unsigned_int_rage_ait(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: f32) -> i64; }
extern "C" { pub fn f_070401_ctasknmbehaviour_canuseragdoll_cped_ptr_eragdolltriggertypes_cen(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_070479_ctasknmcontrol_setcurrenthandpose_ctasknmcontrol_enmhandpose_cmo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32); }
extern "C" { pub fn f_070529_ctasknmhighfall_ctasknmhighfall_unsigned_int_cgrabhelper_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_070532_ctasknmexplosion_ctasknmexplosion_unsigned_int_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_070535_cpedintelligence_addlocalclonetask_ctaskfsmclone_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070561_ctasknmbrace_ctasknmbrace_unsigned_int_unsigned_int_centity_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_070647_ctasknmflinch_ctasknmflinch_unsigned_int_unsigned_int_rage_vecto(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i64, a10: i64, a11: i64, a12: i64) -> i64; }
extern "C" { pub fn f_070648_ctasknmflinch_calculatehitpointinfo_cped_const_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_070651_ctasknmflinch_setflinchmessage_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070675_ctasknmsimple_ctasknmsimple_ctasknmsimple_tunables_tuning_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070707_ctasknmonfire_ctasknmonfire_unsigned_int_unsigned_int_float_rage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070739_ctasknmrelax_ctasknmrelax_unsigned_int_unsigned_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_070769_ctasknmshot_ctasknmshot_cped_ptr_unsigned_int_unsigned_int_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32, a7: i64, a8: i64, a9: i64, a10: i32) -> i64; }
extern "C" { pub fn f_070778_ctasknmshot_updateshot_cped_ptr_centity_ptr_unsigned_int_int_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_072928_ctaskscenario_getscenarioinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_074161_ctask_ctask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_074163_ctaskclassinfomanager_gettaskname_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_074164_cmovenetworkhelper_getclipsetid_rage_fwmvtemplateid_rage_fwmvcli(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075322_ctaskclassinfomanager_gettaskstatename_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_075332_ctaskfsmclone_ctaskfsmclone(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075345_cmovenetworkhelper_sendrequest_rage_fwmvrequestid_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075348_cmovenetworkhelper_getboolean_rage_fwmvtemplateid_rage_fwmvboole(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075351_ccliprequesthelper_dtorccliprequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075352_ccliprequesthelper_releaseclips(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075353_ccliprequesthelper_requestclipsbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075355_cgetupsetrequesthelper_dtorcgetupsetrequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075356_cgetupsetrequesthelper_request_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075462_cmovenetworkhelper_setclip_rage_crclip_const_ptr_rage_fwmvtempla(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075470_cmovenetworkhelper_cmovenetworkhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075472_cmovenetworkhelper_dtorcmovenetworkhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075473_cmovenetworkhelper_createnetworkplayer_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075474_cmovenetworkhelper_setfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_075475_cmovenetworkhelper_setflag_bool_rage_fwmvtemplateid_rage_fwmvfla(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_075484_cgenericcliphelper_cgenericcliphelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075486_cgenericcliphelper_blendinclipbysetandclip_cdynamicentity_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_075500_cmovenetworkhelper_setclipset_rage_fwmvtemplateid_rage_fwmvclips(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075503_cmovenetworkhelper_setboolean_rage_fwmvtemplateid_rage_fwmvboole(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075507_cmovenetworkhelper_getproperty_rage_fwmvtemplateid_rage_fwmvprop(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_075525_cvehiclecliprequesthelper_dtorcvehiclecliprequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075535_cvehiclecliprequesthelper_requestclipsfromentrypoint_cvehicle_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_075599_ctimehelpers_gettimesince_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_075644_ctasktreeclone_startlocalclonetask_ctaskfsmclone_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075645_ctasktreeclone_getactiveclonetask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075758_ctaskvehiclefsm_getpedinorusingseat_cvehicle_const_ref_int_cped(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_075798_cphysical_isbrokenflagset_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_076178_rage_phcollider_applyimpulse_float_vector_4_float_vector_4_int_f(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: i32); }
extern "C" { pub fn f_076257_ctaskmountthrowprojectile_getiscloneindamagewindow_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076771_ctaskaimgunvehicledriveby_ishidingguninfirstpersondrivercamera_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077081_cstatsvehicleusage_clear_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077086_cstatsmgr_checkwritevehiclerecords_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077087_cstatsmgr_signedout(ph: *mut Phantom); }
extern "C" { pub fn f_077089_cstatsmgr_incrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077090_cstatsmgr_decrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077091_cstatsmgr_setgreater_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077095_cstatsmgr_isplayeractive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077096_cphysical_getdistancetravelled_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_077118_cstatsmgr_playerfireweapon_cweapon_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077123_cstatsmgr_registerexplosionhitanything_centity_const_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077149_statid_char_statid_char_rage_atnonfinalhashstring_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_077151_statid_char_gethash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077152_statid_char_gethash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077153_statid_char_getstatid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077173_sstatdata_profilestatmatches_rage_rlprofilestatsvalue_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077174_sstatdata_writeoutprofilestat_rage_rlprofilestatsvalue_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077175_sstatdata_constwriteoutprofilestat_rage_rlprofilestatsvalue_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077176_sstatdata_valuetostring_char_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077188_cstatsdatamgr_endnetworkmatch(ph: *mut Phantom); }
extern "C" { pub fn f_077191_cstatsdatamgr_getstat_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077192_cstatsdatamgr_getstatptr_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077193_cstatsdatamgr_iskeyvalid_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077198_cstatsdatamgr_setstatiterdata_rage_atinbinarymap_sstatdataptr_st(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_077329_statid_set_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077331_statsinterface_getintstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077334_statsinterface_iskeyvalid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077336_statsinterface_getstatdesc_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077338_statsinterface_getcurrentmultiplayercharaterslot(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077339_statsinterface_cloudfileloadpending_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077340_statsinterface_cloudfileloadfailed_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077342_statsinterface_setstatdata_statid_const_ref_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077343_statsinterface_setstatdata_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077344_statsinterface_setstatdata_statid_const_ref_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077346_statsinterface_setstatdata_statid_const_ref_unsigned_short_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077347_statsinterface_setstatdata_statid_const_ref_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077348_statsinterface_setstatdata_statid_const_ref_unsigned_long_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_077349_statsinterface_setstatdata_statid_const_ref_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_077351_statsinterface_setallstatstosynchedbygroup_eprofilestatsgroups_b(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077352_statsinterface_setnoretryonfail_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077353_statsinterface_trymultiplayersave_esavetypes_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_077354_statsinterface_getfloatstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_077355_statsinterface_getbooleanstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077357_statsinterface_getuint16stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077358_statsinterface_getuint32stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077359_statsinterface_getuint64stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077360_statsinterface_getint64stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077364_statsinterface_getposixdate_int_ref_int_ref_int_ref_int_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_077369_statsinterface_findstatiterator_statid_const_ref_rage_atinbinary(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077370_statsinterface_isbadsport_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_077372_statsinterface_updatelocalplayerweaponinfostatid_cweaponinfo_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077373_statsinterface_getstatsmodelhashid_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077375_statsinterface_getstatsplayermodelismultiplayer(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077379_statsinterface_getbossgoonuuid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077381_statsinterface_applypedscardata_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077383_statsinterface_getlocalplayerweaponinfostatid_statsinterface_ewe(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_077394_cstatsutils_isstatstrackingenabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077403_cstatssavesmgr_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077405_cstatssavesmgr_signedoffline(ph: *mut Phantom); }
extern "C" { pub fn f_077408_cstatssavesmgr_pendingsaverequests_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077414_cstatssavesmgr_save_estatssavetype_unsigned_int_esavetypes_unsig(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_077416_cstatssavesmgr_issaveinprogress_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077418_cstatssavesmgr_cancelandclearsaverequests(ph: *mut Phantom); }
extern "C" { pub fn f_077430_moneyinterface_getvcwalletbalance_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077431_moneyinterface_getvcbankbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077432_moneyinterface_getevcbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077433_moneyinterface_getpvcbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077438_moneyinterface_getpxrvalue(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_077442_moneyinterface_incrementstatbankbalance_long_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077443_moneyinterface_decrementstatbankbalance_long_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077445_moneyinterface_decrementstatwalletbalance_long_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_077449_moneyinterface_savegamecanbuycashproducts(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077450_moneyinterface_isoverdailyusdelimit_float(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_077451_moneyinterface_isoverusdebalancelimit_float(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_077452_ccashproductsdata_skucashpackinfo_getskupackname_rage_atstring_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077453_moneyinterface_clearconsumecashproduct_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077454_moneyinterface_startpreconsumesave(ph: *mut Phantom); }
extern "C" { pub fn f_077455_moneyinterface_clearflagforresync(ph: *mut Phantom); }
extern "C" { pub fn f_077460_moneyinterface_creditbankcashstats_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077475_cnumberwithinmessage_set_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077479_ctextfile_get_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077483_csubstringwithinmessage_setliteralstring_char_const_ptr_csubstri(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_077488_cmessages_insertnumbersandsubstringsintostring_char_const_ptr_cn(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32); }
extern "C" { pub fn f_077496_cmessages_clearmessages(ph: *mut Phantom); }
extern "C" { pub fn f_077497_cmessages_clearallbriefmessages(ph: *mut Phantom); }
extern "C" { pub fn f_077507_chelpmessage_setmessagetext_int_char_const_ptr_cnumberwithinmess(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_077527_pagedcloudtext_pagedcloudtext(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077528_pagedcloudtext_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077529_pagedcloudtext_init_rage_atstring_const_ref_bool_unsigned_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_077531_pagedcloudtext_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077533_ctextlayout_ctextlayout(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077534_ctextlayout_default(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077535_ctextlayout_dtorctextlayout(ph: *mut Phantom); }
extern "C" { pub fn f_077537_ctextlayout_setscale_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077540_ctextlayout_render_rage_vector2_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077545_ctextlayout_setscale_rage_vector2_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077551_ctext_getsclanguagefromcurrentlanguagesetting(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077552_ctext_getstringofsupportedlanguagesfromcurrentlanguagesetting(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077557_ctextconversion_texttohtml_char_const_ptr_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_077570_ctextfile_getlanguagefromname_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077801_cvehicleintelligence_getnodelist_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077895_cpathfind_switchroadsoffinarea_unsigned_int_float_float_float_fl(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_077897_cpathfind_switchroadsoffinangledarea_unsigned_int_rage_vector3_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_077930_cvehicleailod_getdummymode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078867_cvehicledeformation_readfromvectoroffset_void_const_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_078871_cvehicle_containslocalplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078944_cvehicledamage_applydamage_centity_ptr_edamagetype_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: i64, a6: i64, a7: i64, a8: i32, a9: i64, a10: i32, a11: i32, a12: i32, a13: f32, a14: i32, a15: i32, a16: i32, a17: i32, a18: i32) -> f32; }
extern "C" { pub fn f_078945_cvehicle_isdriveraplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078981_cvehicle_hascontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079007_cvehicle_getnumberofpassenger_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079023_cvehicle_getattachedtrailer_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079040_cvehicle_getspecialflightmoderatio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_079047_cwheel_getwheelposandradius_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_079070_cvehicle_getseatanimationinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079123_cbike_removeconstraint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079130_cvehicle_getlayoutinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079134_cvehicle_getdriverseat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079157_cvehicle_getisjetski_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079169_cboat_getsearchlight(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079234_cthecargenerators_setcargeneratorsactiveinarea_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079244_cthecargenerators_clearscheduledqueue(ph: *mut Phantom); }
extern "C" { pub fn f_079270_cvehicle_iswindowdown_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079335_chandlingdata_gethandlingdata_ehandlingtype(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079413_cvehicle_delayedremovaltimereset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079423_cheli_doesbullethitpropellerbound_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079436_cvehicledeformation_applycollisionimpact_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_079508_cplane_islargeplane_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079509_cplane_isfastplane_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079587_cvehicle_getfragmentcomponentindex_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079643_cvehicle_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079653_ctrain_removealltrains(ph: *mut Phantom); }
extern "C" { pub fn f_079758_cvehicle_getconvertibleroofstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079836_cvehicle_istaxi_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079837_cvehicle_hasbulletresistantglass_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079843_cvehicle_carhasroof_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079854_cvehicle_containsped_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079861_cwheel_applytyredamage_centity_const_ptr_float_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_079872_cvehicle_getseathasweaponsorturrets_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079873_cvehicleweaponmgr_getnumweaponsforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079908_cvehicle_getconvertibleroofprogress_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_079939_cvehiclepopulation_clearinterestingvehicles_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_079942_cvehicle_testexplosionposandblowreardoorsopen_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079959_cvehicle_applyexplosioneffectemp_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079992_cvehicledamage_canvehiclebedamagedbasedondriverinvincibility(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079997_cwheel_doburstwheeltorim(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080005_cvehicledamage_avoidvehicleexplosionchainreactions(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_080007_cwheel_applysuspensiondamage_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_080054_cvehiclepopulation_getdesirednumberambientvehicles_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> f32; }
extern "C" { pub fn f_080080_cvehicleweaponmgr_getweaponsforseat_int_rage_atarray_cvehiclewea(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_080382_cvehiclepopulation_initvaluesfromtunables(ph: *mut Phantom); }
extern "C" { pub fn f_080383_cvehiclepopulation_initvehiclespacings(ph: *mut Phantom); }
extern "C" { pub fn f_080384_cvehiclepopulation_initplayersroadmodifiers(ph: *mut Phantom); }
extern "C" { pub fn f_080441_cvehiclepopulation_startspawningvehs(ph: *mut Phantom); }
extern "C" { pub fn f_080444_cvehiclepopulation_removeallvehs_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_080541_cvehiclemetadatamgr_getvehicledrivebyinfofromped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080542_cvehiclemetadatamgr_getdrivebyaniminfoforweapon_cped_const_ptr_u(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080666_cvehiclemetadatamgr_getseataniminfofromped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080709_cvehicledrivebyinfo_canuseweapongroup_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_080710_cvehicledrivebyinfo_canuseweapontype_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_080864_cvfxhelper_getdistsqrtocamera_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_080865_cvfxhelper_getmatrixfrombonetag_rage_mat34v_ref_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_080866_cvfxhelper_getmatrixfromboneindex_rage_mat34v_ref_bool_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_080870_cvfxhelper_getmatrixfromboneindex_lights_rage_mat34v_ref_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_080871_cvfxhelper_creatematfromvecy_rage_mat34v_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080872_cvfxhelper_creatematfromvecyalign_rage_mat34v_ref_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_080878_cvfxhelper_getlocalentityboneposdir_centity_const_ref_int_rage_v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_080879_cvfxhelper_getinterpvalue_float_float_float_bool(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_080883_cvfxhelper_getwaterdepth_rage_vec3v_const_ref_float_ref_rage_fwe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_080958_cmoviemgr_deleteall(ph: *mut Phantom); }
extern "C" { pub fn f_080970_cfiremanager_startmapfire_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: i32, a10: i64, a11: i64, a12: i32, a13: i32, a14: i32) -> i64; }
extern "C" { pub fn f_081416_ccoronas_register_rage_vec3v_const_ref_float_rage_color32_float(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: f32, a4: f32, a5: i64, a6: f32, a7: f32, a8: f32, a9: i32); }
extern "C" { pub fn f_081443_cvfxweapon_doweaponimpactvfx_vfxcollinfo_s_const_ref_edamagetype(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_081447_cfiremanager_startobjectfire_cobject_ptr_int_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32, a7: f32, a8: i32, a9: i64, a10: i64, a11: i32, a12: i32, a13: i32); }
extern "C" { pub fn f_081451_cfiremanager_cansetpedonfire_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_081452_cfiremanager_startpedfire_cped_ptr_cped_ptr_int_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: f32, a9: i32, a10: i32, a11: i32) -> i64; }
extern "C" { pub fn f_081454_cfiremanager_startvehiclefire_cvehicle_ptr_int_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32, a7: f32, a8: i64, a9: i64, a10: i32); }
extern "C" { pub fn f_081651_cptfxmanager_removeptfxfromentity_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081740_isentitysmashable_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_081741_cvehicleglassmanager_cancomponentbreak_cphysical_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_081746_cvehicleglassmanager_storecollision_vfxcollinfo_s_const_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081749_cvehicleglassmanager_storeexplosion_vfxexpinfo_s_const_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081750_cvehicleglassmanager_iscomponentsmashableglass_cphysical_const_p(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_081754_cvehicleglassmanager_getvehicleglasscomponentsmashsphere_cphysic(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_081755_cvehicleglassmanager_getvehicleglasscomponent_cphysical_const_pt(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_081759_cvfxblood_triggerptfxblood_cvfxpedinfo_const_ptr_vfxbloodinfo_s(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i64, a10: i32, a11: i32, a12: i64, a13: i64); }
extern "C" { pub fn f_081760_void_creplaymgr_recordfx_cpacketbloodfx_cpacketbloodfx_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_081761_cvfxblood_triggerdecalblood_vfxbloodinfo_s_const_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_081795_cvfxexplosion_processvfxexplosion_phgtaexplosioninst_ptr_bool_fl(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_081849_cvfxprojectile_processptfxprojfuse_cprojectile_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081895_cvfxweapon_getbreakableglassid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_081896_cvfxweapon_doweaponfiredvfx_cweapon_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081897_cvfxweapon_triggerptfxmuzzleflash_centity_ptr_cweapon_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32); }
extern "C" { pub fn f_081898_cvfxweapon_processptfxgunshell_centity_const_ptr_cweapon_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_081902_cvfxweapon_updateptfxvolumetric_cweapon_const_ptr_centity_ptr_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_081966_bool_creplaymgrinternal_handleentityforfx_cpacketvehiclebadgereq(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082003_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_082004_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_082005_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_082007_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_082008_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_082009_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_082015_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_082016_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_082017_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_082020_cairdefencezone_isentitytargetable_centity_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082026_cweapon_cweapon_unsigned_int_int_cdynamicentity_ptr_bool_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_082027_cweapon_fire_cweapon_sfireparams_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082028_cexplosionmanager_addexplosion_cexplosionmanager_cexplosionargs(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082029_cweapon_dtorcweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082030_cpedequippedweapon_getprojectileordnance(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082031_cweapon_doweaponfire_centity_ptr_rage_matrix34_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: f32, a8: i32, a9: i32, a10: i32, a11: i32); }
extern "C" { pub fn f_082032_cweapon_spinup_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082033_phgtaexplosioninst_dtorphgtaexplosioninst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082047_cairdefencemanager_fireweaponatposition_unsigned_char_rage_vec3v(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_082053_cbullet_cbullet_centity_ptr_cweaponinfo_const_ptr_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32, a7: f32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i64, a13: i64, a14: i32, a15: i32, a16: i32, a17: f32, a18: i32, a19: f32, a20: f32, a21: i32) -> i64; }
extern "C" { pub fn f_082054_cbullet_dtorcbullet(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082061_cweapondamage_doweaponimpactvfx_cweapon_ptr_centity_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i64, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_082062_cweapondamage_doweaponimpact_cweapon_ptr_centity_ptr_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i64, a6: i32, a7: i32, a8: i64, a9: i64, a10: i32, a11: i64, a12: f32, a13: f32, a14: f32, a15: i32, a16: i32, a17: i64, a18: i64, a19: i32) -> f32; }
extern "C" { pub fn f_082063_cweapon_sendfiremessage_centity_ptr_rage_vector3_const_ref_world(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i64, a11: f32, a12: f32, a13: f32, a14: i64); }
extern "C" { pub fn f_082064_cpedweaponmanager_getequippedvehicleweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082065_cpedweaponmanager_getequippedweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082067_cweapondamage_applyfalloffdamagemodifier_centity_const_ptr_cweap(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64) -> f32; }
extern "C" { pub fn f_082068_cweapondamage_doweaponimpactped_cweapon_ptr_centity_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32, a6: i64, a7: i64, a8: f32, a9: i64, a10: i32) -> f32; }
extern "C" { pub fn f_082071_cpedweaponmanager_getequippedweapon_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082086_phinstbehaviorexplosiongta_addcollidedentity_centity_ptr_rage_ph(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082087_cexplosioninfomanager_getexplosiontagdata_eexplosiontag(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082089_phinstbehaviorexplosiongta_addcollidedvehicleoccupantstolist_cen(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_082090_phinstbehaviorexplosiongta_addpedsattachedtocar_cvehicle_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_082091_phgtaexplosioninst_getdamagetype(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082094_void_std_2_introsort_std_2_classicalgpolicy_predicatesortbyexplo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082095_phinstbehaviorexplosiongta_processcollisionsforoneframe_std_2_pa(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_082098_phgtaexplosioninst_calculateexplosiondamage_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> f32; }
extern "C" { pub fn f_082099_cweaponinfomanager_getinfo_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082100_cexplosionmanager_triggertyrepoponwheel_cwheel_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082101_cexplosionmanager_applypeddamage_rage_phinst_ptr_cped_ptr_centit(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i64, a7: i64, a8: i64, a9: i64, a10: i32, a11: i32, a12: i32); }
extern "C" { pub fn f_082102_cexplosionmanager_triggerflashgrenadeeffectonlocalplayer_cexplos(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082103_phinstbehaviorexplosiongta_testagainstsphere_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_082107_ccachedsmashables_dtorccachedsmashables(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082108_ccachedsmashables_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082110_cexplosionmanager_addwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_082114_cexplosionmanager_removeexplosion_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082115_cexplosionmanager_startexplosion_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082123_cweapondamage_generatepeddamageevent_centity_ptr_cped_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64, a5: i64, a6: i64, a7: i32, a8: f32, a9: i64, a10: i32, a11: f32, a12: i32) -> f32; }
extern "C" { pub fn f_082124_cweaponinfo_getforcehitped_eanimbonetag_bool_float_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32) -> f32; }
extern "C" { pub fn f_082125_cweaponinfo_getpedmotionclipsetid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082126_cpedintelligence_networklocalhitreactionsallowed(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082129_eexplosiontag_parse_rage_athashvalue_eexplosiontag(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082138_cfiringpatterninfomanager_load(ph: *mut Phantom); }
extern "C" { pub fn f_082140_cweapondebug_initbank(ph: *mut Phantom); }
extern "C" { pub fn f_082142_cweapondebug_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082147_cfiringpattern_processburstfinished(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082151_cthrownweaponinfo_destroyselectedobject(ph: *mut Phantom); }
extern "C" { pub fn f_082152_cthrownweaponinfo_loadthrownweaponinfo(ph: *mut Phantom); }
extern "C" { pub fn f_082158_cpedequippedweapon_createprojectileordnance_cobject_ptr_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_082159_cweapon_doreload_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082161_cweapon_processstate_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_082162_cweapon_updateshadervariables_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082163_cweapon_updateshadervariablesfordrawable_cdynamicentity_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_082165_cammoitemrepository_getisweaponusinginfiniteammo_cweaponinfo_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_082166_cweapon_updateammoafterfiring_centity_ptr_cweaponinfo_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_082167_cweapon_updateanimationafterfiring_centity_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_082168_cweapon_computebulletbending_centity_ptr_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32) -> i32; }
extern "C" { pub fn f_082169_cweapon_computeaccuracy_centity_ptr_centity_const_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: f32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_082170_cprojectilemanager_createprojectile_unsigned_int_unsigned_int_ce(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: f32, a5: i32, a6: i32, a7: i64, a8: i64, a9: i32, a10: i32, a11: i32) -> i64; }
extern "C" { pub fn f_082171_cweapon_buildviewmatrix_rage_vector3_const_ref_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082172_cweapon_getmuzzlematrix_rage_matrix34_const_ref_rage_matrix34_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082174_cweapon_processanimation_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082175_cweaponinfo_getappropriateweaponclipsetid_cped_const_ptr_fpsstre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_082176_cweapon_processweaponspin_centity_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> f32; }
extern "C" { pub fn f_082177_cweapon_createmovenetworkhelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082178_cweapon_getcooktimeleft_cped_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> f32; }
extern "C" { pub fn f_082179_cpedweaponmanager_destroyequippedweaponobject_bool_char_const_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_082181_cweapon_processgunfeed_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082186_cweapon_getaccuracy_cped_const_ptr_float_sweaponaccuracy_ref_con(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_082187_cweapon_getcanreload_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082190_cweapon_startreload_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082191_cweapon_startanim_rage_fwmvtemplateid_rage_fwmvclipsetidtype_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_082192_cweapon_stopanim_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_082193_cweaponanimationsmanager_getweaponclipsethashstealth_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082194_cweaponanimationsmanager_getweaponclipsethashforclone_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082195_cweaponanimationsmanager_getalternativeclipsetwhenblocked_cped_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082196_cweaponanimationsmanager_getweaponclipsethashinjured_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082197_cweaponanimationsmanager_getweaponclipsethashhicover_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082198_cweaponanimationsmanager_getnumfpsfidgets_cped_const_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_082199_cweaponanimationsmanager_getfpsfidgetclipsethash_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_082200_cweaponanimationsmanager_getscopeweaponclipset_cped_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082201_cweaponinfo_getweaponclipsetid_cped_const_ref_fpsstreamingflags(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_082206_cweaponinfo_getweaponclipsetid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082207_cpedequippedweapon_attachobjects_cphysical_ptr_cphysical_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_082209_projectilefactory_create_unsigned_int_unsigned_int_centity_ptr_f(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: f32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_082214_cammoitemrepository_getisweaponoutofammo_cweaponinfo_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_082215_cpedweaponmanager_dropweapon_unsigned_int_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_082216_cpedequippedweapon_destroyobject(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082217_cweapon_initcomponent_cweaponcomponent_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_082218_cweapon_shutdowncomponent_cweaponcomponent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082221_cweaponcomponent_update_hd_models_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082223_cpedinventory_removeweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082224_cpedinventory_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082247_cpedinventoryloadoutmanager_load(ph: *mut Phantom); }
extern "C" { pub fn f_082251_cammoitemrepository_getweaponammo_cweaponinfo_const_ptr_cped_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_082253_weaponfactory_create_unsigned_int_int_cdynamicentity_ptr_char_co(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_082256_cweaponanimationsmanager_loadallmetafiles(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082258_cweaponscriptresource_dtorcweaponscriptresource(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082259_cpedinventoryloadoutmanager_reset(ph: *mut Phantom); }
extern "C" { pub fn f_082260_cweaponinfomanager_reset(ph: *mut Phantom); }
extern "C" { pub fn f_082261_cweaponcomponentmanager_reset(ph: *mut Phantom); }
extern "C" { pub fn f_082262_cweaponmanager_createweaponunarmed_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082265_cweaponanimations_op_cweaponanimations_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082266_cweaponanimations_cweaponanimations_cweaponanimations_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082271_cweaponcomponent_cweaponcomponent_cweaponcomponentinfo_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_082272_cweaponcomponent_dtorcweaponcomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082275_cweaponcomponent_getaimtask_cped_const_ptr_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082276_weaponcomponentfactory_create_cweaponcomponentinfo_const_ptr_cwe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_082277_cweaponboneid_getboneindex_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082279_cweaponcomponentflashlight_toggleactive_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082280_cweaponcomponentflashlight_couldbevisible_rage_matrix34_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082294_cweaponcomponentlasersight_dtorcweaponcomponentlasersight(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082298_cweaponcomponentlasersight_getendpoint_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082299_cweaponcomponentlasersight_submitprobe_centity_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082301_cweaponcomponentmanager_load_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082302_cweaponcomponentmanager_postload(ph: *mut Phantom); }
extern "C" { pub fn f_082311_cweaponcomponentreloaddata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_082320_cweaponcomponentinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_082323_cweaponboneid_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_082365_cweaponcomponentscope_updatepostfx(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082368_cweaponcomponentscope_updatespecialscope_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082370_cweaponcomponentvariantmodelinfo_dtorcweaponcomponentvariantmode(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082383_cammoinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_082386_citeminfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_082387_cammoprojectileinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_082401_cammoinfo_getammomax_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082408_cweaponanimations_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_082413_cweaponanimationsset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_082421_cweaponanimationsmanager_addweaponanimationset_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082422_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082423_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082424_cweaponanimationsmanager_getweaponanimationssetforped_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_082425_cweaponanimationsmanager_getcoveralternatemovementclipsethash_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082426_cweaponanimationsmanager_getcovermovementclipsethash_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082427_cweaponanimationsmanager_getcoverweaponclipsethash_cped_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082428_cweaponanimationsmanager_getmeleeclipsethash_cped_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082429_cweaponanimationsmanager_getjumpupperbodyclipsethash_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082430_cweaponanimationsmanager_getfromstrafetransitionupperbodyclipset(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082431_cweaponanimationsmanager_getmotioncrouchclipsethash_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082432_cweaponanimationsmanager_getmotionstrafingclipsethash_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082433_cweaponanimationsmanager_getmotionstrafingstealthclipsethash_cpe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082434_cweaponanimationsmanager_getmotionstrafingupperbodyclipsethash_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082435_cweaponanimationsmanager_getweaponclipsetstreamedhash_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082436_cweaponanimationsmanager_getswapweaponclipsethash_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082437_cweaponanimationsmanager_getfpstransitionfromidleclipsethash_cpe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082438_cweaponanimationsmanager_getfpstransitionfromrngclipsethash_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082439_cweaponanimationsmanager_getfpstransitionfromltclipsethash_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082440_cweaponanimationsmanager_getfpstransitionfromscopeclipsethash_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082441_cweaponanimationsmanager_getfpstransitionfromunholsterclipsethas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082442_cweaponanimationsmanager_getfpstransitionfromstealthclipsethash(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082443_cweaponanimationsmanager_getfpstransitiontostealthclipsethash_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082444_cweaponanimationsmanager_getfpstransitiontostealthfromunholsterc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082445_cweaponanimationsmanager_getmotionclipsethashforclone_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082449_cweapontintspecvalues_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_082455_cweaponfiringpatternaliases_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_082459_cweaponupperbodyfixupexpressiondata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_082465_cweaponcomponentpoint_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_082489_rage_atfixedarray_cweaponcomponentpoint_7_atfixedarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082502_cweaponinfo_getpedmotionclipsetid_cped_const_ref_fpsstreamingfla(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_082529_sweaponinfolist_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_082531_cweaponinfomanager_updateinfoptrs(ph: *mut Phantom); }
extern "C" { pub fn f_082534_cweaponinfoblob_sweaponslotlist_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_082543_cweaponinfomanager_load_cdatafilemgr_datafile_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082544_cweaponinfomanager_collateinfoblobs(ph: *mut Phantom); }
extern "C" { pub fn f_082545_cweaponinfoblob_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082546_cinventoryitemrepository_cweaponitem_105_removeallitems(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082547_cinventoryitemrepository_cammoitem_65_removeallitems(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082548_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_cweapon(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082551_cweaponinfoblob_addinfowidgets_rage_bkbank_ref_cweaponinfoblob_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_082555_cpedinventoryloadoutmanager_setloadout_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082557_cpedequippedweapon_releaseweaponanims(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082558_cpedequippedweapon_setweaponhash_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082560_cweaponitem_requestequippedanims_cped_const_ptr_fpsstreamingflag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_082561_cpedequippedweapon_createobject_cweaponinfo_const_ptr_int_rage_m(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: f32, a8: i32, a9: i32) -> i64; }
extern "C" { pub fn f_082562_cpedequippedweapon_attachobject_cpedequippedweapon_eattachpoint(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082563_cpedequippedweapon_createweaponcomponent_cweaponcomponentinfo_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_082564_cpedequippedweapon_createobject_cweaponinfo_const_ptr_rage_fwmod(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i32) -> i64; }
extern "C" { pub fn f_082565_cpedequippedweapon_setupasweapon_cobject_ptr_cweaponinfo_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_082566_cpedequippedweapon_releaseweaponcomponent_cobject_ptr_cweaponcom(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082567_cpedequippedweapon_destroyobject_cobject_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082570_cprojectile_cprojectile_eentityownedby_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32, a5: i32, a6: i32, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_082572_cpedinventory_addweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_082573_cinventoryitemrepository_cweaponitem_105_additem_unsigned_int_cp(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_082574_cweaponitem_dtorcweaponitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082575_cweaponitem_cweaponitem_unsigned_int_unsigned_int_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_082576_cinventoryitem_spewpoolusage(ph: *mut Phantom); }
extern "C" { pub fn f_082578_cammoitemrepository_addammo_unsigned_int_int_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_082580_cpedinventory_addweaponandammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_082581_cpedinventory_addweaponammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082582_cammoitemrepository_setweaponusinginfiniteammo_cweaponinfo_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_082584_cammoitemrepository_setweaponammo_cweaponinfo_const_ptr_int_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_082586_cpedinventory_removeweaponandammo_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082587_cinventoryitemrepository_cammoitem_65_removeitem_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082588_cinventoryitem_dtorcinventoryitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082590_cpedinventory_getisstreamedin_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082594_cpedweaponmanager_equipweapon_unsigned_int_int_bool_bool_cpedequ(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_082595_cweaponitemrepository_addcomponent_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_082596_cweaponitemrepository_removecomponent_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_082597_cpedweaponselector_getisweaponvalid_cped_const_ptr_cweaponinfo_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_082598_cpedweaponmanager_processweaponinstructions_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082599_cpedweaponselector_selectweapon_cped_ptr_epedinventoryselectopti(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_082600_cpedweaponmanager_createequippedweaponobject_cpedequippedweapon(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082601_cpedweaponselector_getselectedvehicleweaponinfo_cped_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082602_cpedweaponmanager_unregisternmweapon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082604_cweaponitem_addcomponent_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082605_cweaponitem_removecomponent_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082610_cloadoutrandom_srandomitem_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_082616_cpedinventoryloadout_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_082622_rage_atarray_cpedinventoryloadout_0_unsigned_short_destruct_cped(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082631_cpedweaponmanager_dtorcpedweaponmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082635_cpedweaponmanager_getrequiresweaponswitch_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082636_cpedweaponselector_getnextpedweapon_cped_const_ptr_int_rage_atar(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_082637_cpedweaponselector_getnextvehicleweapon_cped_const_ptr_int_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_082638_cpedweaponselector_getpreviouspedweapon_cped_const_ptr_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_082639_cpedweaponselector_getpreviousvehicleweapon_cped_const_ptr_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_082640_cpedweaponselector_getbestpedweapon_cped_const_ptr_rage_atarray(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_082641_cpedweaponselector_getnextpedweaponingroup_cped_const_ptr_eweapo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082642_cpedweaponselector_setselectedweapon_cped_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_082644_cpedweaponmanager_equipbestweapon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_082660_cpedweaponselector_getisweapondisabledbyvehiclemod_cped_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082662_cinventoryitem_cinventoryitem_unsigned_int_unsigned_int_cped_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_082668_cweaponitem_requestanims_cped_const_ptr_fpsstreamingflags(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_082669_cweaponitem_requestasset_cweaponitem_requesttype_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_082670_cweaponitem_requestcomponents(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082675_cprojectile_dtorcprojectile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082676_cprojectilemanager_removenetsyncprojectile_cprojectile_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082677_cprojectilemanager_cprojectileptr_clear_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082678_cprojectilemanager_dirtynetsyncprojectile_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_082680_cprojectile_processcontrol(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082681_cprojectile_triggerexplosion_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082682_cprojectile_createpotentialblastevent_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_082683_cprojectile_detachfromstickentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082684_cprojectile_processwhizzbyevents_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082685_cprojectile_applydeformation_cvehicle_const_ptr_void_const_ptr_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_082686_cprojectile_gethitboneindexfromfrag_centity_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082687_cprojectile_processphysics_float_bool_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_082688_cphysical_ishiddenflagset_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082693_cprojectile_shouldsticktoentity_centity_const_ptr_centity_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_082695_cprojectile_sticktoentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082696_cprojectilemanager_getexistingnetsyncprojectile_cnetfxidentifier(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082697_cprojectile_onattachment(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082698_cprojectilemanager_fireorplacependingprojectile_cped_ptr_int_cpr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_082702_cprojectile_fire_rage_vector3_const_ref_float_float_bool_bool_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i64, a9: i32, a10: i32); }
extern "C" { pub fn f_082703_cprojectile_initlifetimevalues_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_082705_cprojectile_renderdebug_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082707_cprojectile_sticktoentity_centity_ptr_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_082711_cprojectile_processaudio(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082712_cprojectilemanager_cprojectileptr_dtorcprojectileptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082713_cprojectile_findaveragedstickysurfacenormal_centity_const_ptr_ce(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_082717_cprojectilemanager_clearallnetsyncprojectile(ph: *mut Phantom); }
extern "C" { pub fn f_082718_cprojectilemanager_cprojectileptr_copy_cprojectilemanager_cproje(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082723_cprojectilemanager_removeallstickybombsattachedtoentity_centity(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_082728_cprojectilemanager_togglevisibilityofallprojectiles_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082730_cprojectilemanager_cprojectileptr_serialise_rage_csyncdatabase_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082731_cprojectilerocket_dtorcprojectilerocket(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082735_cprojectilerocket_calchomingprojectileinputs_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082806_cweaponinfoblob_dtorcweaponinfoblob(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082808_rage_atarray_cweaponinfoblob_slotentry_0_unsigned_short_op_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082809_bool_creplaymgrinternal_handleentityforfx_cpacketweaponbullettra(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082810_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_predicates(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_082811_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082815_rage_atarray_rage_atbinarymap_cweaponanimations_rage_atnonfinalh(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082816_std_2_enable_if_is_move_constructible_rage_atbinarymap_cweaponan(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082817_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_082818_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_082819_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_082820_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_082821_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_082822_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_082823_sysmemstartnetwork(ph: *mut Phantom); }
extern "C" { pub fn f_082824_sysmemendnetwork(ph: *mut Phantom); }
extern "C" { pub fn f_082830_networkexitflow_resetexitflowstate(ph: *mut Phantom); }
extern "C" { pub fn f_082831_cnetworktelemetry_flushtelemetry_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082832_networkutils_getbailreasonasstring_ebailreason(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082833_cnetworkdebugtelemetry_updateandflushdebugmetricbuffer_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082834_cnetwork_bail_sbailparameters_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082835_networkinterface_islocalplayersignedin(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082849_cusercontentmanager_requesturltemplates_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082850_cnetwork_resetconnectionpolicies_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_082851_clivemanager_getvalidsentfromgroupsfromtunables_unsigned_int_cha(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_082853_cnetwork_getsocketport(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082854_cnetwork_getversion(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082855_cnetwork_lansessionsonly(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082858_cnetwork_inittunables(ph: *mut Phantom); }
extern "C" { pub fn f_082859_cnetwork_usenetworkheap_networkmemorycategory(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082860_cnetwork_stopusingnetworkheap(ph: *mut Phantom); }
extern "C" { pub fn f_082863_clivemanager_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082864_tunableslistener_tunableslistener_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082865_cfakeplayernames_removeallfakeplayernames(ph: *mut Phantom); }
extern "C" { pub fn f_082866_ctunables_loadfile_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082868_ctunables_establishpool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082869_ctunables_requestcloudfile(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082870_cnetwork_gettimeouttime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082871_cnetworktelemetry_setupfilename_char_const_ptr_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082872_inserttelemetryheader(ph: *mut Phantom); }
extern "C" { pub fn f_082874_cprofilestats_init(ph: *mut Phantom); }
extern "C" { pub fn f_082875_clivemanager_refreshprivileges(ph: *mut Phantom); }
extern "C" { pub fn f_082876_invitemgr_clearall(ph: *mut Phantom); }
extern "C" { pub fn f_082877_clivemanager_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082881_cnetwork_update_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082882_cnetwork_closenetwork_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082883_ctunables_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082884_cnetworktextchat_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_082885_networkinterface_isanysessionactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082886_cnetworksoaktests_update(ph: *mut Phantom); }
extern "C" { pub fn f_082887_cnetwork_updatenetworkalertscreen(ph: *mut Phantom); }
extern "C" { pub fn f_082888_cnetwork_updateclockandweather(ph: *mut Phantom); }
extern "C" { pub fn f_082889_cnetwork_updatescuihotkeys(ph: *mut Phantom); }
extern "C" { pub fn f_082890_networkdebug_update(ph: *mut Phantom); }
extern "C" { pub fn f_082891_cnetwork_updateconcealedplayercachedvariables(ph: *mut Phantom); }
extern "C" { pub fn f_082892_cnetwork_updatetutorialsessioncachedvariables(ph: *mut Phantom); }
extern "C" { pub fn f_082893_networkhasherutil_updateresultsqueue_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082894_cnetworkstreamingrequestmanager_update(ph: *mut Phantom); }
extern "C" { pub fn f_082895_ctunables_stunable_update_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082896_clivemanager_update_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082897_cnetwork_flushalllogfiles_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082898_cnetwork_shutdownvoice_bool(ph: *mut Phantom); }
extern "C" { pub fn f_082899_achmgr_updateachievements(ph: *mut Phantom); }
extern "C" { pub fn f_082900_rage_richpresencemgr_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082901_invitemgr_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_082902_ccloudmanager_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082903_cprofilestats_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082904_clockoverridedata_stopoverriding(ph: *mut Phantom); }
extern "C" { pub fn f_082905_clockoverridedata_override_int_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_082906_cnetwork_getglobalclock_int_ref_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082907_cnetwork_getglobalweather_int_ref_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082908_networkinterface_ishost(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082909_cnetworktelemetry_appendmetric_rage_rlmetric_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082911_networkdebug_updatefocusentity_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082914_cnetwork_isconcealedplayerturn(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082915_cnetwork_isnetworktutorialsessionturn(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082916_networkremotecheaterdetector_remotecheaterdetected_cnetgameplaye(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_082917_clivemanager_actionprofilechange(ph: *mut Phantom); }
extern "C" { pub fn f_082918_clivemanager_bailonmultiplayer_ebailreason_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_082919_clivemanager_refreshfriendcountinmatch(ph: *mut Phantom); }
extern "C" { pub fn f_082920_cplayercardcoronamanager_sendmessage_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082921_networkscpresenceutil_updatecurrentplayerlocation_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082922_networkscpresenceutil_updatedefferedattributes_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082923_cnetwork_applyclockandweather_bool_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_082924_cnetwork_isclocktimeoverridden(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082925_cnetwork_startweathertransitiontoglobal_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082928_metrictransitiontrack_clearstages(ph: *mut Phantom); }
extern "C" { pub fn f_082930_metrictransitiontrack_addstage_unsigned_int_unsigned_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_082931_metrictransitiontrack_stransitionstage_write_rage_rsonwriter_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082932_cnetworkdebugtelemetry_appenddebugmetric_rage_rlmetric_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082934_cnetwork_ismultiplayerdisabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082937_cnetwork_checkspprologuerequirement(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082938_networkinterface_islocalplayeronline_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082939_networkinterface_hasvalidroscredentials_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082940_cnetwork_checknetworkaccess_enetworkaccessarea_unsigned_long_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_082941_clivemanager_hasrosprivilege_rage_rlrosprivilegeid_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_082942_networkinterface_checkonlineprivileges(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082943_cnetwork_canaccessnetworkfeatures_enetworkaccessarea_enetworkacc(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_082944_cnetwork_checknetworkaccessandalertiffail_enetworkaccessarea(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082945_cnetwork_formatposixtimeasuntildate_long_char_ptr_unsigned_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082947_cnetwork_ischeater_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082948_cplayercardmanager_getdatamanager_cplayercardmanager_cardtypes_c(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_082949_void_networkdebug_networkdebugmessagehandler_sendmessagetoplayer(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082950_rage_msgdebugtimepause_export_void_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_082951_networkinterface_getplayerfromconnectionid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082952_cnetwork_istimeoutmodifiedbycommandline(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082953_cnetwork_setsyncedtimeinmilliseconds_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082958_cnetwork_setgostraighttomultiplayer_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082970_networkutils_getbailerrorcodeasstring_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082977_networkdebug_viewfocusentity(ph: *mut Phantom); }
extern "C" { pub fn f_082993_cnetwork_opennetwork(ph: *mut Phantom); }
extern "C" { pub fn f_082994_cgamearraymgr_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082995_cnetwork_initvoice(ph: *mut Phantom); }
extern "C" { pub fn f_082996_cnetwork_enterlobby(ph: *mut Phantom); }
extern "C" { pub fn f_082997_cnetwork_clearpopulation_bool(ph: *mut Phantom); }
extern "C" { pub fn f_082998_networkinterface_broadcastcachedlocalplayerheadblenddata_rage_ne(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082999_networkinterface_registerobject_rage_netgameobjectwrapper_cdynam(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083004_cnetwork_setnetworkgamemode_unsigned_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083005_networkutils_getchannelasstring_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083006_clivemanager_isonline_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083008_networkinterface_iscloudavailable_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083009_networkinterface_isonlineros(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083012_networkinterface_getlocalgamername(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083013_networkinterface_getactivegamerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083014_networkinterface_getlocalgamerhandle(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083015_clivemanager_checkonlineprivileges_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083020_networkinterface_issessionleaving(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083025_networkinterface_getphysicalplayerfromindex_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083030_networkinterface_getnetworkmode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083034_networkinterface_isfriendlyfireallowed_cped_const_ptr_centity_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083035_networkinterface_isfriendlyfireallowed_cvehicle_const_ptr_centit(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083039_networkinterface_isinarcademode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083041_networkinterface_canregisterobject_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083043_networkinterface_getplayerfocusposition_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_083045_networkutils_isvisibleorclosetonetworkplayer_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i64, a5: i32, a6: i64, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_083049_networkutils_getclosestremoteplayers_rage_vector3_const_ref_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_083052_networkinterface_getplayerfromgamerid_rage_rlgamerid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083056_networkinterface_getplayerfromgamerhandle_rage_rlgamerhandle_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083057_networkinterface_getplayerfrompeerid_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083058_networkinterface_gethostplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083060_networkutils_getnetworkplayerposition_cnetgameplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083061_networkinterface_getnetworkobject_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083068_networkinterface_areplayersindifferenttutorialsessions_cnetgamep(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083069_networkinterface_setspectatorobjectid_unsigned_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083071_networkinterface_setinmpcutscene_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_083084_networkinterface_newhost(ph: *mut Phantom); }
extern "C" { pub fn f_083085_networkinterface_closealluiforinvites(ph: *mut Phantom); }
extern "C" { pub fn f_083086_cnetworkroadnodeworldstatedata_cnetworkroadnodeworldstatedata_cg(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_083087_cnetworkscriptworldstatemgr_revertworldstate_cnetworkworldstated(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083088_cnetworkroadnodeworldstatedata_cnetworkroadnodeworldstatedata_cg(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: f32, a6: i32) -> i64; }
extern "C" { pub fn f_083089_cnetworkscriptworldstatemgr_changeworldstate_cnetworkworldstated(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083092_cnetworkpopmultiplierareaworldstatedata_cnetworkpopmultiplierare(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32) -> i64; }
extern "C" { pub fn f_083094_cnetworkpopmultiplierareaworldstatedata_findarea_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_083095_cnetworkvehicleplayerlockingworldstate_cnetworkvehicleplayerlock(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_083096_cnetworkvehicleplayerlockingworldstate_setmodelidplayerlock_cgam(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_083097_cnetworkvehicleplayerlockingworldstate_setmodelinstanceplayerloc(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_083098_cnetworkvehicleplayerlockingworldstate_clearmodelidplayerlock_cg(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_083099_cnetworkvehicleplayerlockingworldstate_clearmodelinstanceplayerl(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_083100_cnetworkropeworldstatedata_cnetworkropeworldstatedata_cgamescrip(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_083101_networkinterface_deleteropeovernetwork_cgamescriptid_const_ref_i(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083103_cnetworkptfxworldstatedata_cnetworkptfxworldstatedata_cgamescrip(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64, a7: f32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i32, a14: i32, a15: i32, a16: i32, a17: i32, a18: i32, a19: f32, a20: i32, a21: f32, a22: i32, a23: i32, a24: i64, a25: i32) -> i64; }
extern "C" { pub fn f_083105_cnetworkscenarioblockingareaworldstatedata_cnetworkscenarioblock(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32) -> i64; }
extern "C" { pub fn f_083106_cnetworkscenarioblockingareaworldstatedata_removescenarioblockin(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083121_networkinterface_forcehighupdatelevel_cphysical_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083124_networkinterface_givepedscriptedtask_cped_ptr_rage_aitask_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_083134_networkinterface_gostraighttotargetposition_cdynamicentity_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083135_networkinterface_overridenetworkblenderfortime_cphysical_const_p(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083143_cnetworkstreamingrequestmanager_sanitycheckentries(ph: *mut Phantom); }
extern "C" { pub fn f_083162_networkinterface_isdamagedisabledinmp_centity_const_ref_centity(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083163_cnetworkghostcollisions_isaghostcollision_cphysical_const_ref_cp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083164_cnetworkghostcollisions_isaghostcollision_cped_const_ref_cped_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083165_cnetworkghostcollisions_isaghostcollision_cped_const_ref_cvehicl(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083166_cnetworkghostcollisions_getdriverfromvehicle_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083167_networkinterface_areinteractionsdisabledinmp_centity_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083170_cnetworkghostcollisions_registerghostcollision_cphysical_const_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083171_cnetworkghostcollisions_findfreecollisionslot_cphysical_const_re(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083175_networkinterface_locallyhideentityforaframe_centity_const_ref_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_083181_networkutils_getnetworkplayerviewport_cnetgameplayer_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083211_rage_netbroadcastdataarrayidentifier_cgamescriptid_netbroadcastd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_083222_rage_netsharedarrayhandler_cpedgroup_cpedgroupsarrayhandler_sete(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_083231_rage_netsharedarrayhandler_cincidentmanager_cincidentptr_cincide(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_083233_cincidentmanager_cincidentptr_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083239_rage_netsharedarrayhandlerwithelementscope_cordermanager_corderp(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083243_rage_netsharedarrayhandler_cordermanager_corderptr_cordersarrayh(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083247_rage_netsharedarrayhandlerwithelementscope_cordermanager_corderp(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_083248_rage_netsharedarrayhandlerwithelementscope_cordermanager_corderp(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_083251_cordermanager_corderptr_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083260_rage_netsharedarrayhandler_cprojectilemanager_cprojectileptr_cst(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_083266_csyncedped_getped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083272_cgamearraymgr_setmemheapforhandler_rage_netarrayhandlerbase_cons(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083283_cnetworkbandwidthmanager_displaytelemetrybandwidthusage_int_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083289_clivemanager_canreceivefrom_rage_rlgamerhandle_const_ref_long_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_083290_cnetworktextchat_appendchatlog_rage_rlgamerhandle_const_ref_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_083291_cnetworktextchat_removetyper_rage_rlgamerinfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083292_cnetworktextchat_removedelegate_rage_atdelegator4_void_cnetworkt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083294_cnetworktextchat_playerchathistory_playerchathistory_long_rage_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_083295_rage_atmapentry_long_cnetworktextchat_playerchathistory_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_083297_networktextfilter_addreceivedtext_rage_rlgamerhandle_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_083298_networkclan_hasprimaryclan(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083299_cnetworktextchat_playerhasleft_rage_rlgamerinfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083300_networktextfilter_init_rage_functor2_rage_rlgamerhandle_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64); }
extern "C" { pub fn f_083302_networkcrewmetadata_networkcrewmetadata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083303_cnetworkcrewdata_dtorcnetworkcrewdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083304_cnetworkcrewdata_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083305_networkcrewmetadata_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083306_networkcrewmetadata_crewmetadatacloudfile_dtorcrewmetadatacloudf(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083307_cnetworkcrewdata_releaselb_rage_fwregdref_rage_netleaderboardrea(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083308_networkclan_releaseemblemreference_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083309_cnetworkcrewdata_init_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_083310_cnetworkcrewdata_requestleaderboarddata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083311_networkcrewmetadata_setclanid_long_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083312_networkcrewemblemmgr_requestemblem_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083313_cnetworkcrewdata_readlbxp_int_long(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_083315_networkclan_getclanemblemnameforclan_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083316_networkcrewemblemmgr_releaseemblem_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083317_networkcrewmetadata_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083318_cnetworkcrewdatamgr_dtorcnetworkcrewdatamgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083320_cnetworkcrewdatamgr_remove_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083321_cnetworkcrewdatamgr_findorrequestdata_long_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_083324_networkclan_getclanemblemnameforclan_long_char_const_ptr_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_083329_networkcrewemblemmgr_dtornetworkcrewemblemmgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083330_networkcrewemblemmgr_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083332_clivemanager_checkusercontentprivileges_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083346_networkcrewmetadata_crewmetadatacloudfile_dorequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083347_networkcrewmetadata_debugprint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083350_cloudlistener_cloudlistener(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083351_ccloudmanager_requestgetcrewfile_int_rage_rlcloudmemberid_char_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083352_ccloudmanager_generatehash_char_const_ptr_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_083353_ccloudmanager_addrequest_enamespace_rage_netcloudrequesthelper_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_083362_fisubmitasyncdata_fiasyncdatarequest_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083366_cloudcache_addcachefile_unsigned_long_unsigned_int_bool_cachepar(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_083367_cloudcache_opencachefile_unsigned_long_cachepartition(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083368_cloudcache_addcacherequest_unsigned_long_cachepartition(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083369_cloudcache_validatecache_rage_fistream_ptr_int_scacheheader_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_083370_cloudcache_getcachedata_int_void_ptr_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_083371_cloudcache_getcachedatastream_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083375_ccloudmanager_dtorccloudmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083378_ccloudmanager_checkpendingrequests(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083379_ccloudmanager_onrequestfinished_int_bool_int_void_ptr_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_083380_ccloudmanager_getfullcloudpath_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083381_ccloudmanager_beginrequest_ccloudmanager_cloudrequestnode_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083382_ccloudmanager_canrequesttitlefile_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083383_ccloudmanager_requestgettitlefile_char_const_ptr_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_083384_ccloudmanager_canrequestglobalfile_unsigned_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083385_ccloudmanager_requestgetglobalfile_char_const_ptr_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_083386_ccloudmanager_requestgetwwwfile_char_const_ptr_unsigned_int_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_083387_ccloudmanager_oncachefileloaded_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_083389_ccloudmanager_addpendingrequest_char_const_ptr_unsigned_int_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_083393_ccloudmanager_requestgetugcfile_int_rage_rlugccontenttype_char_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_083396_ccloudmanager_onsignout(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083431_ccloudmanager_bank_encrypt_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083432_ccloudmanager_bank_decrypt_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083434_ccloudmanager_bank_downloadglobalfile_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083435_ccloudmanager_bank_download100files(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083438_cvideouploadmanager_readtunables(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083439_cvideouploadmanager_dtorcvideouploadmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083443_cvideouploadmanager_refreshuploadenabledflag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083444_cvideouploadmanager_gatherheaderdata_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_083447_cvideouploadmanager_custom_reset_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083451_cvideouploadmanager_custom_deleteuploadfile_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083452_rage_rlyoutubeupload_dtorrlyoutubeupload(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083453_cvideouploadmanager_custom_pauseupload(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083454_cusercontentmanager_readyoutubemetadata_char_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083457_cvideouploadmanager_custom_createuploadinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083459_cvideouploadmanager_custom_isongallerymenu(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083461_cvideouploadmanager_custom_failupload_cvideouploadmanager_custom(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_083462_cvideouploadmanager_custom_stopupload_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_083463_cvideouploadmanager_custom_startnetretrytimer_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_083464_cvideouploadmanager_custom_requestreadvideobufferblock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083465_cvideouploadmanager_custom_startpublishugc(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083466_cvideouploadmanager_custom_uploadnextblock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083467_cvideouploadmanager_custom_getuploadstatus(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083468_cvideouploadmanager_custom_startresumeupload(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083469_achmgr_awardachievement_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083470_cvideouploadmanager_custom_returnconfirmationresult_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083471_cvideouploadmanager_custom_submitstringtouploaddetails_char_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083472_cusercontentmanager_queryyoutubecontent_int_int_bool_int_ptr_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_083473_achmgr_setachievementpassed_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083475_cusercontentmanager_populateyoutubemetadata_char_const_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083476_cusercontentmanager_createyoutubecontent_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_083482_ctunables_dtorctunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083486_ctunables_loadfromjson_void_const_ptr_unsigned_int_bool_bool_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_083487_ctunables_loadvaluefromjson_rage_rsonreader_ptr_ctunables_sunion(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083488_ctunables_loadtunable_char_const_ptr_rage_rsonreader_ptr_bool_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_083489_ctunables_insertinternal_unsigned_int_char_const_ptr_ctunables_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_083491_ctunables_oncloudevent_cloudevent_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083493_ctunables_access_unsigned_int_bool_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083494_ctunables_access_unsigned_int_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083495_ctunables_access_unsigned_int_float_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083496_ctunables_tryaccess_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083497_ctunables_tryaccess_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083498_ctunables_tryaccess_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> f32; }
extern "C" { pub fn f_083499_ctunables_access_unsigned_int_ctunables_euniontype(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083500_ctunables_insert_unsigned_int_bool_unsigned_int_unsigned_int_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083507_cusercontentmanager_clearqueryresults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083508_cusercontentmanager_clearcreatorresults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083512_cusercontentmanager_dtorcusercontentmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083520_cusercontentmanager_writedescriptioncache(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083521_cusercontentmanager_releaseallcacheddescriptions(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083522_cusercontentmanager_cancelquery(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083528_cusercontentmanager_create_char_const_ptr_char_const_ptr_rage_rl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_083531_cusercontentmanager_querycontent_rage_rlugccontenttype_char_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_083532_cusercontentmanager_querycontent_rage_rlugccontenttype_char_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083534_cusercontentmanager_getmycontent_int_int_rage_rlugccontenttype_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_083537_cusercontentmanager_getbycategory_rage_rlugccategory_int_int_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083538_cusercontentmanager_getbycontentid_char_ptr_23_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_083539_cusercontentmanager_getmostrecentlycreated_int_int_rage_rlugccon(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_083542_cusercontentmanager_checkcontentmodify_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083549_cusercontentmanager_canquerydata_int_bool_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083550_cusercontentmanager_requestcdncontentdata_char_const_ptr_unsigne(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083576_cusercontentmanager_getcontentuserid_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_083595_cusercontentmanager_querycontentcreators_rage_rlugccontenttype_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_083600_cusercontentmanager_loadofflinequerydata_rage_rlugccategory(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083601_cusercontentmanager_clearofflinequerydata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083604_cusercontentmanager_oncachefileloaded_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_083607_cusercontentmanager_writeyoutubemetadata_rage_rsonwriter_ref_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083632_cusercontentmanager_bank_savecategory_rage_rlugccategory(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083633_cusercontentmanager_oncloudevent_cloudevent_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083636_ccommercemanager_sthindata_dtorsthindata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083638_ccommercemanager_dtorccommercemanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083640_ccommercemanager_populatethindata_bool(ph: *mut Phantom); }
extern "C" { pub fn f_083641_ccommercemanager_isentitledtostarterpack(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083643_ccommercemanager_isentitledtopremiumpack(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083644_ccommercemanager_isthindatapopulationinprogress(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083645_ccommercemanager_createcommerceutil_bool(ph: *mut Phantom); }
extern "C" { pub fn f_083646_ccommercemanager_initialisecommerceutil(ph: *mut Phantom); }
extern "C" { pub fn f_083647_ccommercemanager_fetchdata(ph: *mut Phantom); }
extern "C" { pub fn f_083648_networkutils_getlanguagecode(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083664_ccommercemanager_sthindata_op_ccommercemanager_sthindata_const_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083667_networkdebug_registerfailmark_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083673_networkdebug_updatepredictionlogforped_cped_ptr_rage_atarray_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_083674_networkutils_getnetworkblenderoverriddenreason_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083675_networkutils_getcanblenderrorstring_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083676_networkdebug_updatepredictionlogforvehicle_cvehicle_ptr_rage_ata(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_083677_networkdebug_updatepredictionlogforobject_cobject_ptr_rage_atarr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_083680_networkutils_getpedfromnetworkobject_rage_netobject_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083682_networkdebug_lookatandpause_unsigned_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083734_networkdebug_networkbank_getchosennetworkplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083738_cnetworkdebugplayback_updateobjects(ph: *mut Phantom); }
extern "C" { pub fn f_083739_void_networkdebug_networkdebugmessagehandler_sendmessagetoallpla(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083748_cnetworkdebugplayback_createped_rage_fwmodelid_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_083755_cnetworkdebugplayback_createvehicle_rage_fwmodelid_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_083761_cnetworkassetverifier_cnetworkassetverifier(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083763_cnetworkassetverifier_generateenvironmentcrc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083764_cnetworkassetverifier_generatestaticcrc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083765_cnetworkassetverifier_generatecrc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083766_cnetworkassetverifier_dumpsyncdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083767_cnetworkassetverifier_isdiscriminatingbysku_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083787_cnetworkkilltracker_starttracking_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_083788_cnetworkkilltracker_stoptracking_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_083789_cnetworkheadshottracker_starttracking_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_083790_cnetworkheadshottracker_stoptracking_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_083797_cfakeplayernames_addfakeplayername_cped_ptr_char_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_083798_cfakeplayernames_removefakeplayername_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083809_networkhasherutil_getgenericfilehash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083810_networkhasherutil_getrequiredhash_networkhasherutil_ehashable_sy(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083813_networkhasherutil_pushnewhashresult_unsigned_char_rage_rlgamerid(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_083818_networkhasherutil_networkbank_getchosenplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083819_char_ptr_rage_formatf_char_65ul_char_ref_65ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083820_cnetfxidentifier_isclone_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083821_cnetfxidentifier_getnextfxid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083823_cnetworkghostcollisions_cghostcollision_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083827_cnetworkghostcollisions_removeghostcollision_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_083829_cnetworkpendingprojectiles_getprojectile_cped_ptr_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_083853_cnetworkstreamingrequestmanager_addtodeadpedinfos_unsigned_short(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_083854_cnetworkstreamingrequestmanager_getanimstreaminghelperslot_unsig(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083855_cnetworkstreamingrequestmanager_isanimstreaming_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083856_cnetworkstreamingrequestmanager_requestvehicleanims_rage_netplay(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_083860_cnetworkstreamingrequestmanager_isindeadpedinfos_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083861_cnetworkstreamingrequestmanager_networkanimrequestdata_getnetwor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083863_cnetworksynchronisedscenes_scenedescription_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083864_cnetworksynchronisedscenes_stoplocalsynchronisedscene_cnetworksy(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083865_cnetworksynchronisedscenes_stoplocalsyncedsceneonped_cnetworksyn(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083866_cnetworksynchronisedscenes_stoplocalsyncedsceneonphysical_cnetwo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_083869_cnetworksynchronisedscenes_startlocalsyncedsceneonped_cnetworksy(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083870_cnetworksynchronisedscenes_startlocalsyncedsceneonphysical_cnetw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_083876_cnetworksynchronisedscenes_createscene_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: f32, a7: i64, a8: i32) -> i32; }
extern "C" { pub fn f_083877_cnetworksynchronisedscenes_allocatenewscenedescription_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083878_cnetworksynchronisedscenes_logscenedescription_rage_netloggingin(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083880_cnetworksynchronisedscenes_attachsyncedscenetoentity_int_cphysic(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_083881_cnetworksynchronisedscenes_addpedtoscene_cped_ptr_int_char_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32, a8: f32, a9: i32); }
extern "C" { pub fn f_083882_cnetworksynchronisedscenes_addentitytoscene_cphysical_ptr_int_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32); }
extern "C" { pub fn f_083883_cnetworksynchronisedscenes_addmapentitytoscene_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_083884_cnetworksynchronisedscenes_addcameratoscene_int_char_const_ptr_c(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_083888_cnetworksynchronisedscenes_startscenerunning_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083889_cnetworksynchronisedscenes_stopscenerunning_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_083902_csyncedentity_getentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083918_networkutils_getpedfromdamageentity_centity_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083919_networkutils_getplayerfromdamageentity_centity_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083922_networkutils_getcanpasscontrolerrorstring_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083930_networkutils_getsessiontypeasstring_sessiontype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083931_networkutils_getsessionpurposeasstring_sessionpurpose(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083932_networkutils_getresponsecodeasstring_ejoinresponsecode(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083933_networkutils_getsessionresponseasstring_rage_snjoinresponsecode(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083934_networkutils_getpoolasstring_emultiplayerpool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083935_networkutils_getmultiplayergamemodeintasstring_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083936_networkutils_loggamerhandle_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083938_cnetworkvehiclemodeldoorlockedtablemanager_getplayerlockindexbym(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083940_cnetworkvehiclemodeldoorlockedtablemanager_getplayerlockindexbyv(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083942_cnetworkvehiclemodeldoorlockedtablemanager_setmodelidplayerlock(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083943_cnetworkvehiclemodeldoorlockedtablemanager_setmodelinstanceplaye(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083946_cnetworkscriptworldstatemgr_addtohostresponsependinglist_cnetwor(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083947_cnetworkscriptworldstatemgr_addtohostmigrationpendinglist_cnetwo(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084002_cnetworkentityareaworldstatedata_cnetworkentityareaworldstatedat(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64, a10: f32) -> i64; }
extern "C" { pub fn f_084003_cnetworkentityareaworldstatedata_initializeboundingbox(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084009_cnetworkentityareaworldstatedata_isequalto_cnetworkworldstatedat(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084013_cnetworkentityareaworldstatedata_addentityarea_cgamescriptid_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_084014_cnetworkentityareaworldstatedata_removeentityarea_cgamescriptid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_084069_cgroupclandata_init(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_084070_rage_atarray_rage_rlgamerhandle_0_unsigned_short_op_rage_atarray(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084080_loginviteflags_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_084081_loginternalflags_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_084082_logresponseflags_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_084083_sacceptedinvite_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084084_sacceptedinvite_cancel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084086_invitemgr_resolveinvite_rage_rlgamerinfo_const_ref_rage_rlgamerh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i64, a10: i32, a11: i32); }
extern "C" { pub fn f_084087_clivemanager_showsigninui_unsigned_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084088_clivemanager_issystemuishowing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084089_sacceptedinvite_init_rage_rlsessioninfo_const_ref_rage_rlgamerin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32, a8: i64, a9: i32, a10: i32); }
extern "C" { pub fn f_084090_unacceptedinvite_reset_rage_rlsessioninfo_const_ref_rage_rlgamer(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_084092_invitemgr_clearinvite_invitemgr_clearflags(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084093_updateinviteblock_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084094_invitemgr_acceptinvite_rage_rlsessioninfo_const_ref_rage_rlgamer(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32); }
extern "C" { pub fn f_084095_invitemgr_notifyplatforminviteaccepted_sacceptedinvite_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_084096_invitemgr_showalert_invitemgr_invitealertreason_int_ptr_bool_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_084097_clivemanager_isplatformsubscriptioncheckpending_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084098_clivemanager_hasplatformsubscription_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084099_invitemgr_getconfirmtext(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_084100_invitemgr_getconfirmsubstring(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_084101_invitemgr_removeunacceptedinvite_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084102_invitemgr_removersinvite_unsigned_int_invitemgr_removereason(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_084103_invitemgr_checkandflagplatforminvitealerts(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084104_invitemgr_tryprocessplatforminvite_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084105_invitemgr_checkinviteisvalid(ph: *mut Phantom); }
extern "C" { pub fn f_084107_invitemgr_addplatforminvite_rage_rlgamerinfo_const_ref_rage_rlga(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_084111_metricaddrsinvite_metricaddrsinvite_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_084112_networkclan_getuiformattedclantag_rage_rlclandesc_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_084113_printclandesc_rage_rlclandesc_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084114_clanmembershipdata_addactiveclanmembershipdata_rage_rlgamerhandl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_084115_clanmembershipdata_printmembershipdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084117_invitemgr_acceptrsinvite_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084137_invitemgr_acceptinvite_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084138_invitemgr_addunacceptedinvite_rage_rlsessioninfo_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_084141_clivemanager_setmaingamerinfo_rage_rlgamerinfo_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084160_rage_richpresencemgr_presencemsg_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084161_networkclan_networkclanop_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084162_cprofilestats_ssynchoperation_cancel(ph: *mut Phantom); }
extern "C" { pub fn f_084166_networkclan_refreshinvitesreceived(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084167_networkclan_refreshjoinrequestsreceived(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084168_cprofilestats_ssynchoperation_operationdone_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084169_cprofilestats_resetgroup_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084170_rage_atarray_rage_rlscpresenceattribute_0_unsigned_short_op_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084173_clivemanager_clearmaingamerinfo(ph: *mut Phantom); }
extern "C" { pub fn f_084175_clivemanager_checkvoicecommunicationprivileges_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_084176_clivemanager_checktextcommunicationprivileges_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_084178_clivemanager_showgamerprofileui_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084180_clivemanager_showaccountupgradeui_unsigned_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084181_clivemanager_addplayer_rage_rlgamerinfo_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084193_clivemanager_getgamerstatus_rage_rlgamerhandle_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_084240_cgamertagfromhandle_start_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_084244_metricplayercoords_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084246_metricmatchmakingqueryresults_addcandidate_metricmatchmakingquer(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084247_metricmatchmakingqueryresults_scandidatedata_write_rage_rsonwrit(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084248_metricmatchmakingqueryresults_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084249_metricmatchmakingquerystart_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084252_metricmpsession_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084253_metricsessionstart_metricsessionstart_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084259_metricsessionmigrated_addcandidate_char_const_ptr_metricsessionm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_084260_metricsessionmigrated_setcandidateresult_char_const_ptr_metricse(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_084261_metricsessionmigrated_scandidate_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084262_metricsessionmigrated_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084264_writesku_rage_rsonwriter_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_084268_metricbasemultiplayermissiondone_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084272_metricpurchase_flow_purchasemetricinfos_savecurrentcash(ph: *mut Phantom); }
extern "C" { pub fn f_084274_getpurchaselocationhash_estorepurchaselocation(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084289_char_ptr_rage_formatf_char_2ul_char_ref_2ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084298_metriccomms_cumulativetextcommsinfo_sendtelemetry_metriccomms_ec(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_084299_metriccomms_cumulativevoicecommsinfo_sendtelemetry_cnetgameplaye(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084307_ctunables_tryaccess_unsigned_int_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_084308_cnetworktelemetry_getpurchaseflowtelemetry_char_ptr_unsigned_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_084311_cnetworkdebugtelemetry_releasedebugmetricbuffer(ph: *mut Phantom); }
extern "C" { pub fn f_084315_metricvehicle_metricvehicle_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_084316_cnetworktelemetry_releasehighvolumemetricbuffer(ph: *mut Phantom); }
extern "C" { pub fn f_084334_cnetworktelemetry_enteredsolosession_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_084335_cnetworktelemetry_networkkicked_unsigned_long_unsigned_long_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i64); }
extern "C" { pub fn f_084337_networkscpresenceutil_script_setpresenceattributeint_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_084351_cnetworktelemetry_onpeerplayerconnected_bool_unsigned_long_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_084368_cnetworkdebugtelemetry_autogpucapturetelemetrystore_dtorautogpuc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084369_rage_atarray_threadtimingresult_0_unsigned_short_resizegrow_unsi(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084370_streamingmemoryresult_op_streamingmemoryresult_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084372_cnetworkdebugtelemetry_autogpucapturetelemetrystore_op_cnetworkd(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084373_rage_atarray_threadtimingresult_0_unsigned_short_delete_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084389_metricmemory_store_metricmemory_store_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_084390_metricmemory_usage_metricmemory_usage_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_084391_cnetworkdebugtelemetry_appendmetricmemstore_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084401_metricmemory_skeleton_metricmemory_skeleton_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_084406_cnetworkdebugtelemetry_collectmetrictimebars(ph: *mut Phantom); }
extern "C" { pub fn f_084410_cprofilestatsrecords_resetrecords_unsigned_int_rage_rlprofilesta(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_084411_cprofilestatsread_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084414_cprofilestatsreadsessionstats_endstatsread(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084418_cprofilestats_dirtycache_adddirtystat_rage_rlprofiledirtystat_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_084419_cprofilestats_synchronize_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_084420_cprofilestats_setsynchronized_cprofilestats_esynctype_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084421_cprofilestats_flush_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_084422_cprofilestats_clearsynchronized_cprofilestats_esynctype(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084424_cprofilestats_synchronized_cprofilestats_esynctype_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084431_clanmembershipdata_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084432_clanmembershipdata_get_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_084433_clanmembershipdata_setactiveclanmembership_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084434_clanmembershipdata_clanmembershipdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084436_networkclan_getavailableclanop_networkclan_enetworkclanops(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_084437_networkclan_bank_updateclandetails(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084438_networkclan_rejectrequest_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_084439_networkclan_handleclanmembershiprefreshsuccess(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084440_networkclan_dispatchevent_networkclan_networkclanop_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084441_networkclan_rejectinvite_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084445_networkclan_refreshlocalmemberships_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_084446_networkclan_getmembershipcount_rage_rlgamerhandle_const_ref_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084447_networkclan_getmembership_rage_rlgamerhandle_const_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_084449_networkclan_getprimaryclan_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_084450_networkclan_refreshmemberships_rage_rlgamerhandle_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084451_networkclan_checkforanyopsoftype_networkclan_enetworkclanops_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_084452_networkclan_refreshmembershipspending_rage_rlgamerhandle_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084453_networkclan_updaterank_unsigned_int_char_const_ptr_long_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_084457_networkclan_joinclan_long(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084459_networkclan_getreceivedinvite_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_084460_networkclan_invitegamer_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084461_networkclan_acceptinvite_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084510_networkscpresenceutil_presenceattrid_set_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084511_networkscpresenceutil_defferedattributeupdatelist_clearall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084512_networkscpresenceutil_setpresenceattributestring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084513_networkscpresenceutil_updatempscpresenceinfo_rage_snsession_cons(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084515_networkscpresenceutil_adddefferedattribute_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_084516_rage_atarray_networkscpresenceutil_defferedattributeupdate_0_uns(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084525_cplayercardxmldatamanager_cardstat_setstat_char_const_ptr_bool_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_084526_playercardstatinfo_addstat_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_084527_playercardpackedstatinfo_addstat_char_const_ptr_char_const_ptr_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_084533_rage_rlprofilestatsfixedrecord_30u_rlprofilestatsfixedrecord(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084536_baseplayercarddatamanager_playerlistdata_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084537_baseplayercarddatamanager_playerlistdata_initializestatsresults(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_084538_baseplayercarddatamanager_playerlistdata_getstatvaluefromstatsre(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_084539_playercarddataparserhelper_receivestat_rage_rlprofilestatsreadre(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_084540_baseplayercarddatamanager_dtorbaseplayercarddatamanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084541_baseplayercarddatamanager_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084542_baseplayercarddatamanager_init_cplayercardxmldatamanager_const_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084543_baseplayercarddatamanager_setstate_baseplayercarddatamanager_est(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084544_baseplayercarddatamanager_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084545_baseplayercarddatamanager_printprofilestatsresults_rage_rlprofil(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084548_baseplayercarddatamanager_getcachedgamerstats_int_rage_rlgamerha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_084549_baseplayercarddatamanager_ongamernamespopulated(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084550_baseplayercarddatamanager_createrequestedrecords_baseplayercardd(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084552_baseplayercarddatamanager_hasplayedmode_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084553_cfriendplayercarddatamanager_oncachefileloaded_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_084556_baseplayercarddatamanager_refreshgamerdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084557_baseplayercarddatamanager_commonrefreshgamerdata_baseplayercardd(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084577_cpartyplayercarddatamanager_buildgamerhandles_rage_atarray_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_084588_cscriptedcoronaplayercarddatamanager_removegamerhandle_rage_rlga(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084590_cscriptedcoronaplayercarddatamanager_addgamerhandle_rage_rlgamer(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_084591_cplayercardmanager_init_cplayercardmanager_cardtypes_baseplayerc(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_084599_cclanplayercarddatamanager_dtorcclanplayercarddatamanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084600_cclanplayercarddatamanager_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084615_char_ptr_rage_formatf_char_202ul_char_ref_202ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084616_socialclubmgr_createaccount_char_const_ptr_char_const_ptr_rage_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_084620_socialclubmgr_geterrorname_rage_netstatus_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084621_socialclubmgr_geterrorname_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_084622_socialclubmgr_getdefaultdevnames_char_ptr_char_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_084626_rage_atarray_rage_rlscpresenceattribute_0_unsigned_short_grow_un(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084629_cloudlistener_dtorcloudlistener_138661(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084643_cgameplayerbroadcastdatahandler_local_dtorcgameplayerbroadcastda(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084653_rage_netsharedarrayhandlerwithelementscope_cpedgroup_cpedgroupsa(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_084656_rage_netsharedarrayhandler_cpedgroup_cpedgroupsarrayhandler_sete(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_084672_rage_netsharedarrayhandlerwithelementscope_cpedgroup_cpedgroupsa(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_084681_rage_netsharedarrayhandler_cpedgroup_cpedgroupsarrayhandler_clea(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084689_rage_netsharedarrayhandlerwithelementscope_cincidentmanager_cinc(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_084691_rage_netsharedarrayhandler_cincidentmanager_cincidentptr_cincide(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_084705_rage_netsharedarrayhandlerwithelementscope_cincidentmanager_cinc(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_084754_rage_netsharedarrayhandlerwithelementscope_cnetworkstreamingrequ(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_084757_rage_netsharedarrayhandler_cnetworkstreamingrequestmanager_netwo(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_084771_rage_netsharedarrayhandlerwithelementscope_cnetworkstreamingrequ(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_084780_rage_netsharedarrayhandler_cnetworkstreamingrequestmanager_netwo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_084782_rage_netsharedarrayhandler_cnetworkstreamingrequestmanager_netwo(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084830_cvideouploadmanager_custom_saveheaderdataworker_dtorsaveheaderda(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084835_cvideouploadmanager_custom_dtorcvideouploadmanager_custom_139039(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084971_cusercontentmanager_ugccreaterequestnode_dtorugccreaterequestnod(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085185_metricbuild_type_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085293_rage_netbroadcastdataarrayidentifierbase_read_rage_datbitbuffer(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085295_rage_netbroadcastdataarrayidentifierbase_write_rage_datbitbuffer(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085375_csocialclubinboxmgr_dtorcsocialclubinboxmgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085377_rage_atarray_cputimingresult_0_unsigned_short_op_rage_atarray_cp(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085378_rage_atarray_threadtimingresult_0_unsigned_short_copyfrom_rage_a(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085415_bool_rage_rlscpresencemessage_isa_rage_rlscpresenceinboxnewmessa(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085417_csocialclubinboxmgr_doapplyonevent_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085419_rage_netgamepresenceevent_isgamepresevent_rage_rsonreader_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085420_cgamepresenceeventdispatcher_processgameevent_rage_rlgamerinfo_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_085421_csocialclubinboxmgr_sendeventtogamers_rage_netgamepresenceevent(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_085422_rage_netgamepresenceevent_export_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085426_rage_netgamepresenceevent_geteventtypename_rage_rsonreader_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085427_getnestedmemberhelper_char_const_ptr_rage_rsonreader_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085429_csocialclubinboxmgr_getdatavalueformessage_int_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_085436_rage_netgamepresenceevent_getobjectastype_rage_rsonreader_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085437_cnetworknewsstorymgr_addnewsstoryitem_cnewsitempresenceevent_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085438_cnetworknewsstorymgr_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_085440_cgamepresenceeventdispatcher_sendeventtogamers_rage_netgameprese(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_085441_bool_rage_rlpresence_postmessage_rage_rlscpresencemessagepublish(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_085442_cgamepresenceeventdispatcher_sendeventtocommunity_rage_netgamepr(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085443_cgamepresenceeventdispatcher_registerevent_char_const_ptr_rage_n(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_085461_cnetworksession_kickplayer_rage_netplayer_const_ptr_kickreason_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_085462_cugcstatupdatepresenceevent_trigger_char_const_ptr_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085463_cbountypresenceevent_trigger_rage_rlgamerhandle_const_ptr_int_sc(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_085464_cfriendcrewjoinedpresenceevent_trigger_debug_rage_rlclandesc_con(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085465_cfriendcrewcreatedpresenceevent_trigger_debug_unsigned_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_085466_cmissionverifiedpresenceevent_trigger_debug_char_const_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085467_crockstarcrewmsgpresenceevent_trigger_debug_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_085468_cgameawardpresenceevent_trigger_debug_unsigned_int_float_unsigne(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_085469_cnewsitempresenceevent_trigger_debug_char_const_ptr_unsigned_sho(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085470_ctournamentinvite_trigger_debug_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085494_cnetworksession_handlelocalplayerbeingkicked_kickreason(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085495_cblacklistedgamers_blacklistgamer_rage_rlgamerhandle_const_ref_e(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_085496_cstatupdatepresenceevent_trigger_unsigned_int_int_unsigned_int_c(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_085497_cstatupdatepresenceevent_trigger_unsigned_int_float_unsigned_int(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_085515_crockstarmsgpresenceevent_base_deserialise_rage_rsonreader_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085517_crockstarmsgpresenceevent_base_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085519_crockstarmsgpresenceevent_base_gettext_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085520_netlocalizedstringincloudmgr_releaserequest_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085536_cnetworkvoicesession_sendjoinresponse_unsigned_int_int_rage_neta(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085537_cnetworkvoicesession_join_rage_rlsessioninfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085541_cgameinvite_trigger_rage_rlgamerhandle_const_ptr_int_rage_rlsess(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32); }
extern "C" { pub fn f_085542_networkbaseconfig_getstaticdiscriminator_staticdiscriminatortype(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_085543_networkbaseconfig_getmatchmakingversion(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_085544_networkbaseconfig_getmatchmakinguser_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_085545_cgameinvite_triggerfromjoinqueue_rage_rlgamerhandle_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_085549_cgameinvitecancel_trigger_rage_rlgamerhandle_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_085559_cfollowinvite_trigger_rage_rlgamerhandle_const_ptr_int_rage_rlse(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_085569_cnetworksession_ismysessiontoken_rage_rlsessiontoken_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085570_cnetworksession_ishost_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085571_cnetworksession_ismytransitiontoken_rage_rlsessiontoken_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085593_cnetworkvoice_ontextmessagereceived_char_const_ptr_rage_rlgamerh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_085601_socialclubeventmgr_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_085602_socialclubeventmgr_doschedulerequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085603_socialclubeventmgr_processreceivedschedule_rage_rsonreader_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085604_socialclubeventmgr_getglobaleventhash_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_085605_socialclubeventmgr_evententry_iseventtype_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085608_socialclubeventmgr_getactiveeventcode_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085609_socialclubeventmgr_oncloudevent_cloudevent_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085610_socialclubeventmgr_dumpevents(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085614_socialclubeventmgr_getextradatamember_char_const_ptr_rage_rsonre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_085618_socialclubeventmgr_getextraeventdata_int_char_const_ptr_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_085620_socialclubeventmgr_getextraeventdata_int_char_const_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_085622_socialclubeventmgr_geteventdisplaynamebyid_int_char_ptr_int_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_085623_cnetworkscemail_setsubjectandcontent_sscriptemaildescriptor_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_085624_cnetworkemailmgr_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_085626_cnetworkemailmgr_flushretrievedemails(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085627_cnetworkemailmgr_deletescemails_char_const_ptr_32_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085629_cnetworkemailmgr_sendemail_unsigned_short_sscriptemaildescriptor(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_085630_cnetworkemailmgr_retrieveemails_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085632_cnetworkemailmgr_deleteemails_int_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_085638_cnetworkscnewsstoryrequest_doimagerequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085639_cnetworkscnewsstoryrequest_setstate_cnetworkscnewsstoryrequest_s(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085640_cnetworkscnewsstoryrequest_handlereceiveddata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085642_cnetworkscnewsstoryrequest_readimagedata_rage_rsonreader_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_085643_cnetworkscnewsstoryrequest_cancel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085644_cnetworkscnewsstoryrequest_dtorcnetworkscnewsstoryrequest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085646_cnetworkscnewsstoryrequest_getsubtitle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085647_cnetworkscnewsstoryrequest_getcontent_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085648_cnetworkscnewsstoryrequest_getheadline_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085651_cnetworknewsstorymgr_parsenews_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_085652_cnetworknewsstorymgr_getnewsstoryimpressioncount_unsigned_int_in(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_085654_cnetworktransitionnewscontroller_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_085655_cnetworktransitionnewscontroller_cnetworktransitionnewscontrolle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085656_cnetworktransitionnewscontroller_skipcurrentnewsitem(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085657_cnetworktransitionnewscontroller_clearpendingstory(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085658_cnetworktransitionnewscontroller_readyrequest_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085660_cnetworktransitionnewscontroller_initcontrol_int_cnetworktransit(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_085661_prioritizedlist_generatenextlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085662_cnetworktransitionnewscontroller_makestoryrequestat_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085664_cnetworktransitionnewscontroller_bank_fillprioritizednewslistdis(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085665_cnetworktransitionnewscontroller_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085670_cnetworkpausemenunewscontroller_requeststory_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085671_cnetworkpausemenunewscontroller_clearpendingstory(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085672_cnetworkpausemenunewscontroller_ispendingstoryready_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085692_cnetworktransitionnewscontroller_bank_stopnewsdisplay(ph: *mut Phantom); }
extern "C" { pub fn f_085697_policyversions_policyversions(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085698_socialclubversionedfile_socialclubversionedfile(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085699_socialclubversionedfile_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085700_socialclubversionedfile_dtorsocialclubversionedfile(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085701_policyversions_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085702_policyversions_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085703_socialclubversionedfile_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085704_policyversions_parsefile_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_085705_socialclubversionedfile_getacceptedversion_bool_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085707_socialclubversionedfile_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085708_policyversions_isdownloading_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085709_policyversions_isuptodate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085710_policyversions_geterror_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085719_networkbaseconfig_getmatchmakingpool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_085720_networkbaseconfig_getmatchmakingregion(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_085721_networkbaseconfig_getmatchmakingaimbucket_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_085722_networkbaseconfig_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085723_networkgameconfig_logdifferences_rage_rlsessionconfig_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_085724_networkgameconfig_apply_rage_rlsessionconfig_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085726_networkgameconfig_getmatchingattributes_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085727_networkgameconfig_getgamemode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085728_networkgameconfig_debugprint_bool_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085730_networkgamefilter_applyregion_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085731_networkgamefilter_applylanguage_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085732_networkgamefilter_debugprint_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085733_loghostflags_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_085734_logjoinflags_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_085735_logmatchmakingflags_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_085737_cnetworksession_trypeercomplaint_unsigned_long_rage_netpeercompl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_085738_rage_netmessage_getname_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_085739_cnetworksession_util_getgamerlogstring_rage_snsession_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_085740_cnetworksession_util_getgamerlogstring_rage_snsession_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_085741_cnetworksession_util_getgamerlogstring_rage_snsession_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_085742_cnetworksession_forcesendtransitionparameters_rage_rlgamerinfo_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085743_msgtransitionparameters_msgtransitionparameters(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085744_ceventnetworktransitionparameterchanged_ceventnetworktransitionp(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_085745_void_rage_snsession_sendmsg_msgsessionestablished_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_085746_cnetworksession_launchtransition(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085747_cnetworksession_swaptotransitionsession(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085748_cnetworksession_setsessionstate_cnetworksession_esessionstate_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085749_cnetworksession_setsessiontimeout_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085750_cnetworksession_joinsession_rage_rlsessioninfo_const_ref_rage_rl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_085751_cnetworksession_dropsession(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085752_cnetworksession_checkforanothersessionorbail_ebailreason_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085753_ceventnetworktransitionevent_ceventnetworktransitionevent_cevent(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_085754_cnetworksession_getsessionfromtoken_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_085755_cnetworksession_getmatchmakinggroupnum_ematchmakinggroup_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085756_cnetworksession_getactivityplayernum_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085757_cnetworksession_getactivityplayermax_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085758_cnetworksession_canjoinwithcrewid_sessiontype_long(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_085759_msgdebugstall_import_void_const_ptr_unsigned_int_unsigned_int_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_085760_cnetworksession_shouldmigrate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085761_cnetworksession_spendingreservations_spendingreservations(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085762_cnetworksession_smatchmakingquery_smatchmakingquery(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085775_cnetworksession_onsessionentered_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085776_cnetworksession_initpeercomplainer_rage_netpeercomplainer_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_085777_void_networkgameconfig_reset_player_schema_matchingattributes_pl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_085778_cnetworksession_completetransitionquickmatch(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085779_cmsgjoinresponse_import_void_const_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_085780_cnetworksession_setactivityspectatorsmax_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085781_cnetworksession_setactivityplayersmax_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085782_cnetworksession_addtransitionplayer_rage_rlgamerinfo_const_ref_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_085783_cnetworksession_sendmigratemessages_unsigned_long_rage_snsession(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_085784_cnetworksession_updateactivityslots(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085785_cnetworksession_updatecurrentuniquecrews_sessiontype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085786_cnetworkvoice_playerhasjoined_rage_rlgamerinfo_const_ref_unsigne(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085787_cnetworksession_ishostplayerandrockstardev_rage_rlgamerhandle_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085788_void_rage_snsession_sendmsg_msgradiostationsyncrequest_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_085789_cnetworksession_sendlaunchmessage_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085790_cnetworksession_removequeuedjoinrequest_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085791_cnetworkvoice_playerhasleft_rage_rlgamerinfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085792_cnetworkinvitedplayers_removeinvite_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085793_cnetworksession_checkforqueuedjoinrequests(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085794_cnetworksession_droptransition(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085795_cnetworksession_leavetransition(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085796_cnetworksession_migrateorinitpeercomplainer_rage_netpeercomplain(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_085797_cnetworksession_onhandlejoinrequest_rage_snsession_ptr_rage_rlga(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_085798_void_std_2_introsort_std_2_classicalgpolicy_peerpred_ref_rage_rl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_085799_cnetworksession_buildtransitionplayerdata_cnetgameplayerdatamsg(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085800_bool_rage_playerdatamsg_sermsgpayload_rage_datexportbuffer_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085801_cnetworksession_isactivityspectator_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085802_cnetworksession_getnattypefrompeerid_sessiontype_unsigned_long(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_085803_cnetworksession_smatchmakingresults_dtorsmatchmakingresults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085804_cnetworksession_smatchmakingquery_dtorsmatchmakingquery(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085805_cnetworksession_init(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085806_cnetworksession_sendmatchmakingstarttelemetry_cnetworksession_sm(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_085807_cnetworksession_settransitionstate_cnetworksession_etransitionst(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085808_cnetworksession_sendmatchmakingresultstelemetry_cnetworksession(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085809_void_rage_snsession_broadcastmsg_msgsessionestablished_msgsessio(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085810_cnetworksession_shutdown_bool_ebailreason(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085811_cnetworksession_dumpestablishingstateinformation(ph: *mut Phantom); }
extern "C" { pub fn f_085812_cblacklistedgamers_whitelistgamer_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085813_cnetworksession_opennetwork(ph: *mut Phantom); }
extern "C" { pub fn f_085816_cnetworksession_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085817_cnetworkinvitedplayers_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085818_cnetworksession_clearsession_unsigned_int_leavesessionreason(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085819_cnetworksession_actioninvite(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085820_cnetworksession_joininvite(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085821_cnetworksession_dojoinfailureaction(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085822_cnetworksession_jointransition_rage_rlsessioninfo_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_085823_cnetworksession_updatematchmakinggroups(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085824_cnetworksession_hostsession_int_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_085825_cnetworksession_dofreeroammatchmaking_int_int_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_085826_cnetworksession_doquickmatch_int_int_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_085827_cnetworksession_sendtransitioninvites_rage_rlgamerhandle_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_085828_cnetworksession_sendinvites_rage_rlgamerhandle_const_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_085829_cnetworksession_gethostplayer_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085830_cnetworksession_joinorhostsession_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085831_cnetworksession_issessioncompatible_networkgamefilter_ptr_rage_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_085832_cnetworksession_clearblacklistedsessions(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085833_cnetworksession_processmatchmakingresults_cnetworksession_smatch(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_085834_cnetworksession_findsessions_cnetworksession_smatchmakingquery_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085835_cnetworksession_completesessioncreate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085836_cnetworksession_hostsession_networkgameconfig_ref_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_085837_cnetworksession_canstart_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085838_rage_atarray_cnetworksession_blacklistedsession_0_unsigned_short(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085839_cnetworksession_leavesession_unsigned_int_leavesessionreason(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_085840_cnetworksession_bailtransition_etransitionbailreason_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_085841_cnetworksession_swapsessions(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085842_cnetworksession_verifymultiplayeraccess_char_const_ptr_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085843_cnetworksession_buildjoinrequest_unsigned_char_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_085844_cnetworksession_updatetransitiongamerdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085845_cnetworksession_dotransitiontogame_int_rage_rlgamerhandle_ptr_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_085846_void_rage_snsession_broadcastmsg_msgtransitionparameterstring_ms(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085847_cnetworksession_hosttransition_networkgameconfig_const_ref_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_085848_cnetworksession_failtransitionhost(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085849_cnetworksession_failtransitionjoin(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085852_cnetworksession_domatchmakingcommon_int_int_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_085853_cnetworksession_applymatchmakingregionandlanguage_networkgamefil(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_085854_cnetworksession_findsessionsfriends_cnetworksession_smatchmaking(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_085855_cnetworksession_findsessionssocial_cnetworksession_smatchmakingq(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_085856_void_rage_snsession_sendmsg_msgtransitiontogamestart_unsigned_lo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_085859_cnetworksession_istransitioning(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085860_cnetworksession_setmatchmakinggroupmax_ematchmakinggroup_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085870_void_networkgamefilter_reset_player_schema_matchingfilter_activi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085871_cnetworksession_validatematchmakingquery_cnetworksession_smatchm(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085873_cnetworksession_setsessionvisibilitylock_sessiontype_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_085876_cnetworksession_initregistrationpolicies(ph: *mut Phantom); }
extern "C" { pub fn f_085877_cnetworksession_getnumbossesinfreeroam_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085878_cnetworksession_dosortmatchmakingpreference_networkgamefilter_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_085879_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_smatchm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_085881_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_cnetwor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_085882_cnetworksession_addmatchmakingresults_cnetworksession_smatchmaki(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_085883_msgsessionestablished_export_void_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_085884_cnetworksession_istransitionmember_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085887_cnetworksession_clearqueuedjoinrequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085890_cnetworksession_setsessionvisibility_sessiontype_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085891_cnetworksession_clearwaitflag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085892_cnetworksession_joinprevioussession_joinfailureaction_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_085893_cnetworksession_joinsession_rage_rlsessioninfo_const_ref_bool_jo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_085895_cnetworkinvitedplayers_addinvite_rage_rlgamerhandle_const_ref_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_085897_cnetworksession_sendpresenceinvitestotransition_rage_rlgamerhand(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_085898_cnetworksession_sendpresenceinvites_rage_rlgamerhandle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_085899_cnetworksession_removeinvites_rage_rlgamerhandle_const_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085900_cnetworksession_cancelinvites_rage_rlgamerhandle_const_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085901_cnetworkinvitedplayers_didinvite_rage_rlgamerhandle_const_ref_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085903_cnetworksession_getmatchconfig_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085904_cnetworksession_issolosession_sessiontype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085905_cnetworkvoicesession_isvoicesessionactive(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085907_cnetworksession_isclosedsession_sessiontype_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085908_cnetworksession_isprivatesession_sessiontype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085911_cnetworksession_setcontentcreator_sessiontype_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085920_cnetworksession_issessionmember_rage_rlgamerhandle_const_ref_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085921_cnetworksession_isinsession_rage_rlsessioninfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085922_cnetworkinvitedplayers_ackinvite_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085923_cnetworkinvitedplayers_setdecisionmade_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085924_cnetworksession_isinvitedecisionmade_rage_rlgamerhandle_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085925_cnetworkinvitedplayers_setinvitestatus_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_085926_cnetworksession_getinvitestatus_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085928_cnetworksession_joinorhosttransition_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085929_cnetworksession_hosttransition_int_unsigned_int_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_085930_bool_rage_netconnectionmanager_send_msgtransitionparameters_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_085931_msgtransitionparameterstring_export_void_ptr_unsigned_int_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_085932_cnetworkrecentplayers_addrecentplayer_rage_rlgamerinfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085933_cnetworksession_findactivitysessions_cnetworksession_smatchmakin(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085935_cnetworksession_dotransitionquickmatch_int_unsigned_int_int_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64, a8: i32) -> i32; }
extern "C" { pub fn f_085945_bool_rage_netconnectionmanager_send_msgtransitiontogamenotify_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_085951_bool_rage_netconnectionmanager_send_msgtransitionlaunch_int_msgt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_085955_void_std_2_introsort_std_2_classicalgpolicy_cnetworkrecentplayer(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_085956_cnetworksession_removetransitioninvites_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085958_cnetworksession_canceltransitioninvites_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085962_cnetworksession_gettransitionmembers_rage_rlgamerinfo_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_085963_cnetworksession_gettransitionnetendpointid_rage_rlgamerinfo_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_085967_void_rage_snsession_sendmsg_msgtransitionparameterstring_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_085969_bool_rage_netconnectionmanager_send_msgtransitiongamerinstructio(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_085971_cnetworksession_hastransitionplayerfullyjoined_rage_rlgamerhandl(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085972_bool_cmsgjoinresponse_sermsgpayload_rage_datimportbuffer_cmsgjoi(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085973_bool_rage_playerdatamsg_sermsgpayload_rage_datimportbuffer_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085974_bool_cmsgjoinrequest_sermsgpayload_rage_datimportbuffer_cmsgjoin(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085975_cnetworkvoice_buildmutelists_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085976_cnetworkvoice_addpendinggamer_cnetworkvoice_pendingvoicegamer_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_085978_getgamerlogstring_rage_rlgamerinfo_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085980_bool_rage_netconnectionmanager_send_msgradiostationsync_int_msgr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_085983_cnetworksession_kickrandomplayer_bool_rage_rlgamerhandle_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_085984_cblacklistedgamers_getblacklistreason_rage_rlgamerhandle_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085986_void_rage_snsession_broadcastmsg_msgdebugstall_msgdebugstall_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086019_cnetworkvoice_sendtextmessage_char_const_ptr_rage_rlgamerhandle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_086046_cnetworkinvitedplayers_getinvitedplayer_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_086047_cnetworkinvitedplayers_sinvitedgamer_iswithinackthreshold_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086052_cnetworkvoicesession_init_rage_sysmemallocator_ptr_rage_netconne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_086056_cnetworkvoicesession_shutdown_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086057_cnetworkvoicesession_bail_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086058_cnetworkvoicesession_update(ph: *mut Phantom); }
extern "C" { pub fn f_086059_cnetworkvoicesession_leave(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086061_cnetworkvoicesession_host(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086062_cnetworkvoicesession_isinvoicesession(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086064_cnetworkvoicesession_invitegamer_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086075_rage_atfixedarray_cleaderboardscriptdisplaydata_5_dtoratfixedarr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086081_cnetworkreadleaderboards_updatewidgets(ph: *mut Phantom); }
extern "C" { pub fn f_086082_cnetworkreadleaderboards_readbygamer_unsigned_int_rage_rlleaderb(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i64) -> i64; }
extern "C" { pub fn f_086083_cnetworkreadleaderboards_readbyfriends_unsigned_int_rage_rlleade(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32) -> i64; }
extern "C" { pub fn f_086097_cnetworkleaderboardmgr_dtorcnetworkleaderboardmgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086098_cnetworkleaderboardmgr_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086101_cnetworkleaderboardmgr_update(ph: *mut Phantom); }
extern "C" { pub fn f_086116_cnetworkvoice_buildmutelistforgamer_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086120_cnetworkvoice_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_086121_cnetworkvoice_init(ph: *mut Phantom); }
extern "C" { pub fn f_086122_cnetworkvoice_initworkerthread(ph: *mut Phantom); }
extern "C" { pub fn f_086123_cnetworkvoice_isenabled_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_086124_cnetworkvoice_shutdownworkerthread(ph: *mut Phantom); }
extern "C" { pub fn f_086125_cnetworkvoice_update(ph: *mut Phantom); }
extern "C" { pub fn f_086128_cnetworkvoice_ontextmessageprocessed_rage_rlgamerhandle_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_086129_cnetworkvoice_findgamer_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_086134_cnetworkvoice_islocaltalking_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_086136_cnetworkvoice_isanygamertalking_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086137_cnetworkvoice_isanyremotegamertalking_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086150_cnetworkvoice_setteamonlychat_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086225_gametransactionsessionmgr_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_086247_netcatalog_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086259_cnetworkshoppingmgr_shoulddonulltransaction_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_086262_gametransactionsessionmgr_bail_gametransactionsessionmgr_estate(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_086263_gametransactionsessionmgr_setstate_gametransactionsessionmgr_est(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086293_cnetworkshoppingmgr_shutdown_unsigned_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086296_cnetshoptransactionbase_getrequestid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086302_networkgametransactions_dogametransactionevent_int_networkgametr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_086333_gametransactionsessionmgr_isqueuebusy_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086376_networkgametransactions_gametransactionbase_serialize_rage_rsonw(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086379_networkgametransactions_applyuserinfohack_int_rage_rsonwriter_re(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_086403_networkgametransactions_gametransactionitems_serialize_rage_rson(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086412_purchgametransactionhttptask_ptr_rage_rltaskmanager_createtask_p(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086429_gametransactionbasehttptask_configure_int_unsigned_int_rage_rson(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_086436_getaccesstokenhelper_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086437_gametransactionhttptask_configure_int_unsigned_int_rage_rsonwrit(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_086454_gametransactionsessionmgr_shutdownmanager(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086456_bool_rage_rltaskbase_configure_getbalancegametransactionhttptask(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_086457_gametransactionsessionmgr_triggercatalogrefresh(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086649_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_086650_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_086651_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_cnetworkre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_086652_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086744_cnetgameplayer_getplayerped_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086745_cnetworkobjectmgr_getnetworkobjectfromplayer_unsigned_short_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_086746_cnetworkobjectmgr_registergameobject_rage_netgameobjectbase_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_086747_cnetworkobjectpopulationmgr_canregisterobject_unsigned_short_boo(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_086763_cnetobjgame_getplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086768_cnetobjdoor_cnetobjdoor_cdoor_ptr_unsigned_short_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_086770_cprojectsynctree_initialiseinitialstatebuffer_rage_netsynctreeta(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_086785_cnetobjentity_flagfordeletion(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086800_cnetworkobjectpopulationmgr_canregisterobjects_unsigned_int_unsi(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_086855_cnetobjdoor_setdoorsystemdata_cdoorsystemdata_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086860_cnetobjentity_setoverridinglocalvisibility_bool_char_const_ptr_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_086894_cnetobjobject_hastobecloned_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_086895_cnetobjobject_canbenetworked_cobject_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_086917_cnetobjentity_setoverridinglocalcollision_bool_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_086951_cnetobjentity_islocalentityvisibleovernetwork(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086958_cnetgameplayer_getplayerinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087029_cnetobjphysical_concealentity_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087124_cnetobjped_setclonepedinvehicle_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_087143_cnetobjphysical_setalphafading_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087146_cnetobjped_updatependingscriptedtask(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087154_cnetobjped_givepedscriptedtask_ctask_ptr_int_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_087295_cnetobjped_getisdamagablebyplayer_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_087322_cnetobjped_processfailmark(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087329_cnetblenderped_isstandingonobject_unsigned_short_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_087341_cnetobjphysical_clearlastdamageobject(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087342_cnetobjphysical_getlastdamageobjectentity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087352_cnetobjphysical_stopalpharamping(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087356_cnetobjphysical_activatedamagetracking(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087357_cnetobjphysical_istrackingdamage(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087390_cnetobjphysical_setalpharampinginternal_bool_bool_bool_bool_shor(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_087406_cnetobjphysical_setinvertghosting_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_087519_cnetobjplayer_updatecachedtutorialsessionstate(ph: *mut Phantom); }
extern "C" { pub fn f_087535_cnetobjplayer_setrespawninvincibilitytimer_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087541_cnetobjplayer_startgangtutorial_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087542_cnetobjplayer_endtutorial(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087625_cnetgameplayer_settransitionsessioninfo_rage_rlsessioninfo_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087627_cnetgameplayer_setstartedtransition_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087628_cnetgameplayer_invalidatetransitionsessioninfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087642_cnetobjplayer_canrespawnlocalplayerped(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087643_cnetobjplayer_respawnplayerped_rage_vector3_const_ref_bool_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_087646_cnetobjplayer_setlookattalkers_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_087658_cnetobjplayer_updatehighestwantedlevelinarea(ph: *mut Phantom); }
extern "C" { pub fn f_087659_cnetobjplayer_cacheplayerheadblenddata_unsigned_char_cpedheadble(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_087660_cnetobjplayer_applycachedplayerheadblenddata_cped_ref_unsigned_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_087661_cnetobjplayer_getextendedscopefovmultiplier_float(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_087669_cnetobjproximitymigrateable_keepproximitycontrol_unsigned_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087752_cnetobjvehicle_islockedforplayer_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_087831_cnetobjvehicle_processfailmark(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087899_cnetblenderped_updateheadingblending(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_088176_cnonphysicalplayerdata_cnonphysicalplayerdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_088187_cnetgameplayer_ischeatalreadynotified_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_088195_cnetgameplayer_isinroamingbubble_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_088196_cnetgameplayer_setlocaltextchatflag_cnetgameplayer_textchatflag(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_088197_cnetgameplayerdatamsg_cnetgameplayerdatamsg(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_088198_cnetgameplayerdatamsg_reset_unsigned_int_rage_netnattype_rage_ne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64, a8: i32, a9: i32, a10: i32); }
extern "C" { pub fn f_088203_cnetworkplayermgr_dtorcnetworkplayermgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_088217_cnetworkplayermgr_getplayerdata_rage_rlgamerid_const_ref_void_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_088232_croamingbubblemgr_allocateplayerinitialbubble_cnetgameplayer_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088235_croamingbubblemgr_update(ph: *mut Phantom); }
extern "C" { pub fn f_088247_cnetworkparty_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_088250_cnetworkparty_isinparty_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_088260_partyinvitemgr_clearall(ph: *mut Phantom); }
extern "C" { pub fn f_088261_partyinvitemgr_addinvite_rage_rlsessioninfo_const_ref_rage_rlgam(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_089329_crequestcontrolevent_trigger_rage_netobject_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_089334_cgivecontrolevent_trigger_rage_netplayer_const_ref_rage_netobjec(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_089361_cweapondamageevent_trigger_centity_ptr_centity_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i32, a14: i32, a15: i32, a16: i32, a17: i64); }
extern "C" { pub fn f_089404_cdebuggameclockevent_trigger_unsigned_int_rage_netplayer_const_p(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_089419_crespawnplayerpedevent_trigger_rage_vector3_const_ref_unsigned_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32); }
extern "C" { pub fn f_089484_cstartprojectileevent_trigger_cprojectile_ptr_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32); }
extern "C" { pub fn f_089597_cscriptworldstateevent_trigger_cnetworkworldstatedata_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_089629_cstartnetworksyncedsceneevent_trigger_int_unsigned_int_rage_netp(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_089703_centityareastatusevent_trigger_cgamescriptid_const_ref_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_089758_cplaysoundevent_trigger_rage_vector3_const_ref_unsigned_int_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_089773_cplayairdefensefireevent_trigger_rage_vector3_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_089824_cmodifyvehiclelockworldstatedataevent_trigger_unsigned_int_int_u(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_089962_cinformsilencedgunshotevent_trigger_cped_ptr_cped_ptr_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_089976_ccacheplayerheadblenddataevent_trigger_cpedheadblenddata_const_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_089992_cupdatefxnevent_trigger_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_090014_cnetworkcheckcatalogcrc_trigger_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_090021_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_090598_initssl_method(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090618_dtlsmsgpoolsend(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090649_wolfssl_ctx_new_ex(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090650_wolfssl_init(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090651_wolfssl_cleanup(ph: *mut Phantom); }
extern "C" { pub fn f_090653_wolfssl_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090654_wolfssl_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090656_wolfssl_connect(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090659_wolfssl_shutdown(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090660_wolfssl_get_error(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090661_wolfssl_err_error_string(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090665_wolfssl_ctx_set_group_messages(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090668_wolfssl_ctx_load_verify_buffer(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090669_wolfssl_ctx_set_verify(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090710_asinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090717_atof(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090741_time(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090798_ldexp(ph: *mut Phantom, a0: i32) -> f64; }
extern "C" { pub fn f_090801_localtime(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090814_memcmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090817_gmtime_r(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090826_powf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090888_pthread_mutex_trylock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090901_qsort_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_090910_srand(ph: *mut Phantom, a0: i32); }
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
extern "C" { pub fn f_090940_strncasecmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090942_strncmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090944_strnlen(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090959_strtoll(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_090960_syscall_ret(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090965_tanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090985_vsnprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090987_vsprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090998_wmemchr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_091002_emscripten_builtin_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091069_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
