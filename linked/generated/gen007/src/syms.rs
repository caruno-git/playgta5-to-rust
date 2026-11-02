//! Callee declarations for crate gen007.
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
extern "C" { pub fn f_000110_rage_syscriticalsectiontoken_dtorsyscriticalsectiontoken(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000111_rage_syscriticalsectiontoken_lock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000113_rage_syscriticalsectiontoken_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000114_rage_sysipccreatesema_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_000115_rage_sysipcpollsema_rage_sysipcsematag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000116_rage_sysipcwaitsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000119_rage_sysipcsignalsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000120_rage_sysipccreatemutex(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_000121_rage_sysipclockmutexcommand_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000122_rage_sysipcunlockmutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000123_rage_sysipcdeletemutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000124_rage_sysipccreateevent(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_000128_rage_sysipccreatethread_void_ptr_void_ptr_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_000131_rage_sysipcwaitthreadexit_rage_sysipcthreadid_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000134_rage_sysipcyield_int(ph: *mut Phantom); }
extern "C" { pub fn f_000135_rage_sysipcsleep_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000212_rage_fidevicetcpip_connect_char_const_ptr_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000243_rage_grcdevice_registerdevicelostcallbacks_rage_functor0_rage_fu(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000565_wasm_input_test_key(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_000568_wasm_start_mode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000644_rage_athashnextsize_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000646_rage_athash64_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000650_rage_athash16u_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000653_rage_atstring_set_rage_atstring_const_ref_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_000654_rage_atstring_set_char_const_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_000655_rage_atstring_lowercase(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000658_rage_atstring_startswith_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000659_rage_atstring_indexof_char_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_000664_rage_atstring_split_rage_atstring_ref_rage_atstring_ref_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000665_rage_atstring_op_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000666_rage_atstring_split_rage_atstring_ref_rage_atstring_ref_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_000668_rage_atarray_rage_atstring_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_000669_rage_atstring_trim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000671_rage_atstring_split_rage_atarray_rage_atstring_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000675_rage_functorbase_functorbase_void_const_ptr_void_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_000680_rage_athashstringnamespacesupport_computehash_rage_athashstringn(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_000681_rage_athashstringnamespacesupport_addstring_rage_athashstringnam(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_000684_rage_athashstringnamespacesupport_trygetstring_rage_athashstring(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_000685_rage_athashstringnamespacesupport_savehashstrings_rage_athashstr(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000690_rage_bkwidget_bkwidget_rage_datcallback_ref_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_000692_rage_datcallback_call_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000695_rage_bkremotepacket_receivepackets(ph: *mut Phantom); }
extern "C" { pub fn f_000696_rage_bkwidget_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000702_rage_bkbank_setcurrentgroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000703_rage_bkbank_unsetcurrentgroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000704_rage_bkbank_pushgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000705_rage_bkgroup_addgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000706_rage_bkbank_deletegroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000707_rage_bkbank_popgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000758_rage_bkmanager_openfile_char_ptr_int_char_const_ptr_bool_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_000773_rage_bkmanager_createoutputwindow_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000779_rage_bkmanager_createbank_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000782_rage_bkmanager_findbank_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000783_rage_bkmanager_findwidget_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000785_rage_bkmanager_destroybank_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000786_rage_bkmanager_isbankthread(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000787_rage_bkconsole_bkconsole_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_000794_rage_bkconsole_addcommand_char_const_ptr_rage_atfunctor3_void_ch(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000798_rage_bkwidget_setreadonly_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000806_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000814_rage_bkremotepacket_connect_bool_char_const_ptr(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000815_rage_bkmessagebox_char_const_ptr_char_const_ptr_rage_bkmessagebo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_000816_rage_bkremotepacket_printcommandline(ph: *mut Phantom); }
extern "C" { pub fn f_000828_rage_bklist_addcolumnheader_int_char_const_ptr_rage_bklist_itemt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_000829_rage_bklist_additem_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_000830_rage_bklist_additem_int_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_000841_rage_bkcombo_bkcombo_rage_datcallback_ref_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_000843_rage_bkcombo_setstring_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_000844_rage_bkcombo_getstring_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
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
extern "C" { pub fn f_000878_rage_bkgroup_addtext_char_const_ptr_rage_atnonfinalhashstring_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_000880_rage_bkgroup_addtext_char_const_ptr_rage_atnonfinalhashstring_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000882_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000884_rage_bkgroup_addtoggle_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000885_rage_bkgroup_addtoggle_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_000886_rage_bkgroup_addtoggle_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000887_rage_bkgroup_addtoggle_char_const_ptr_unsigned_short_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_000888_rage_bkgroup_addtoggle_char_const_ptr_unsigned_short_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000889_rage_bkgroup_addtoggle_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_000890_rage_bkgroup_addtoggle_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000891_rage_bkgroup_addtoggle_char_const_ptr_rage_atbitset_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_000892_rage_bkgroup_addtoggle_char_const_ptr_rage_atbitset_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000893_rage_bkgroup_addtoggle_char_const_ptr_float_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000894_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000897_rage_bkgroup_addslider_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000898_rage_bkgroup_addslider_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000899_rage_bkgroup_addslider_char_const_ptr_signed_char_ptr_signed_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_000900_rage_bkgroup_addslider_char_const_ptr_signed_char_ptr_signed_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_000901_rage_bkgroup_addslider_char_const_ptr_unsigned_short_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_000902_rage_bkgroup_addslider_char_const_ptr_unsigned_short_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000903_rage_bkgroup_addslider_char_const_ptr_short_ptr_short_short_shor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_000904_rage_bkgroup_addslider_char_const_ptr_short_ptr_short_short_shor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000905_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000906_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000907_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int_rage_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000908_rage_bkgroup_addslider_char_const_ptr_rage_vector2_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64); }
extern "C" { pub fn f_000910_rage_bkgroup_addslider_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64); }
extern "C" { pub fn f_000911_rage_bkgroup_addslider_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000912_rage_bkgroup_addslider_char_const_ptr_rage_vector4_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000913_rage_bkgroup_addslider_char_const_ptr_rage_vector2_ptr_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_000916_rage_bkgroup_addcombo_char_const_ptr_unsigned_char_ptr_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_000918_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000919_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000921_rage_bkgroup_addlist_char_const_ptr_bool_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000922_rage_bkgroup_adddatawidget_char_const_ptr_unsigned_char_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_000923_rage_bkgroup_addvector_char_const_ptr_rage_vector2_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000925_rage_bkgroup_addvector_char_const_ptr_rage_vector2_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000926_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000927_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000928_rage_bkgroup_addvector_char_const_ptr_rage_vector4_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000929_rage_bkgroup_addvector_char_const_ptr_rage_vector4_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000931_rage_bkgroup_addmatrix_char_const_ptr_rage_matrix33_ptr_float_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000933_rage_bkgroup_addmatrix_char_const_ptr_rage_matrix34_ptr_float_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000934_rage_bkgroup_addmatrix_char_const_ptr_rage_matrix34_ptr_float_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000935_rage_bkgroup_addmatrix_char_const_ptr_rage_matrix44_ptr_float_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000936_rage_bkgroup_addangle_char_const_ptr_float_ptr_rage_bkangletype(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000937_rage_bkgroup_addangle_char_const_ptr_float_ptr_rage_bkangletype(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000939_rage_bkgroup_addangle_char_const_ptr_float_ptr_rage_bkangletype(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32); }
extern "C" { pub fn f_000940_rage_bkgroup_addangle_char_const_ptr_rage_vector2_ptr_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_000941_rage_bkgroup_addcolor_char_const_ptr_rage_color32_ptr_rage_datca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_000942_rage_bkgroup_addcolor_char_const_ptr_rage_color32_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000943_rage_bkgroup_addcolor_char_const_ptr_float_ptr_int_rage_datcallb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_000946_rage_bkgroup_addcolor_char_const_ptr_rage_vector3_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000947_rage_bkgroup_addcolor_char_const_ptr_rage_vector4_ptr_rage_datca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_000948_rage_bkgroup_addcolor_char_const_ptr_rage_vector3_ptr_rage_datca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000990_char_ptr_rage_formatf_char_512ul_char_ref_512ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001098_rage_autoidinit(ph: *mut Phantom); }
extern "C" { pub fn f_001099_rage_autoidshutdown(ph: *mut Phantom); }
extern "C" { pub fn f_001116_char_ptr_rage_formatf_char_32ul_char_ref_32ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001117_rage_datcallback_datcallback(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001119_rage_datcallback_datcallback_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001123_rage_datgrowbuffer_datgrowbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001124_rage_datgrowbuffer_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001125_rage_datgrowbuffer_dtordatgrowbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001126_rage_datgrowbuffer_init_rage_sysmemallocator_ptr_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001127_rage_datgrowbuffer_init_void_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001138_rage_datgrowbuffer_getbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001143_rage_datbitbuffer_datbitbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001148_rage_datbitbuffer_setreadwritebytes_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001149_rage_datbitbuffer_getnumbyteswritten_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001167_rage_datbitbuffer_writebool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001171_rage_datbitbuffer_writestr_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001514_rage_diagdisplayf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001516_rage_fistream_write_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001519_rage_diagchannel_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001522_rage_diagchannel_sendsubchannels(ph: *mut Phantom, a0: i64); }
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
extern "C" { pub fn f_001538_rage_diagchannel_setoutput_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_001543_rage_diagloggedprintf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001551_rage_fprintf_rage_fistream_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001553_rage_firemoteshowmessagebox_char_const_ptr_char_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001554_rage_diaglogf_rage_diagchannel_const_ref_rage_diagseverity_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001560_rage_diagcontextmessage_diagcontextmessage_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001563_rage_diagcontextmessage_pop(ph: *mut Phantom); }
extern "C" { pub fn f_001564_rage_diagcontextmessage_displaycontext_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001565_printf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001568_rage_atmapentry_unsigned_int_wchar_t_ptr_const_ref_rage_atmap_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001569_rage_atmap_unsigned_int_wchar_t_ptr_rage_atmaphashfn_unsigned_in(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001576_rage_fiassetmanager_fullpath_char_ptr_int_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_001577_rage_fistream_open_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001578_rage_fistream_seek_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001580_rage_fiassetmanager_open_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001582_rage_fiassetmanager_createleadingpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001605_rage_fidevice_makegrowbufferfilename_char_ptr_int_rage_datgrowbu(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001606_rage_restservice_restservice_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001611_rage_restservicedirectory_addchild_rage_restservice_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001614_rage_restservicemanager_createdirectories_char_const_ptr(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_001617_rage_restregistersimpleservice_char_const_ptr_rage_atdelegate_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001619_rage_restservicefileaccess_dtorrestservicefileaccess(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001623_rage_fiassetmanager_findextensioninpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001624_rage_fiassetmanager_setpath_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001627_rage_fiassetmanager_exists_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001632_rage_fiassetmanager_pushfolder_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001633_rage_fiassetmanager_popfolder(ph: *mut Phantom); }
extern "C" { pub fn f_001635_rage_fiassetmanager_filename_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001636_rage_fiassetmanager_basename_char_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001637_rage_fiassetmanager_enumfiles_char_const_ptr_void_ptr_rage_fifin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_001639_rage_fiassetmanager_removeextensionfrompath_char_ptr_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001640_rage_fiassetmanager_removenamefrompath_char_ptr_int_char_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001650_rage_fgetline_char_ptr_int_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_001651_rage_fistream_preload_rage_fistream_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001652_rage_fidevice_makememoryfilename_char_ptr_int_void_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_001656_rage_fibasetokenizer_init_char_const_ptr_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001680_rage_fibasetokenizer_gettokentochar_char_ptr_int_char_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_001735_rage_fitokenizer_fitokenizer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001764_rage_atarray_rage_atbinarymap_int_rage_atfinalhashstring_datapai(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001768_rage_fidevicerelative_init_char_const_ptr_bool_rage_fidevice_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_001769_rage_fidevicerelative_mountas_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001798_rage_fidevicerelative_getdebugname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001806_rage_fidevice_unmount_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001807_rage_fidevice_unmount_rage_fidevice_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001818_rage_fidevice_isanythingmountedatmountpoint_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001881_rage_fipackfile_init_char_const_ptr_bool_rage_fipackfile_cachemo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_001883_rage_fipackfile_mountas_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001884_rage_fipackfile_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001956_rage_fisavegame_setselecteddevice_int_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_001987_rage_fisavegame_setselecteddevicetoany_int(ph: *mut Phantom); }
extern "C" { pub fn f_002027_rage_invsqrtf_fast_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_002031_rage_mthrandom_mthrandom_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002032_rage_mthrandom_setfullseed_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002035_rage_mthrandommwc_getint(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002037_rage_pgrscbuilder_constructname_char_ptr_int_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_002048_rage_pgstreamer_initclass_int_rage_sysipcpriority_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_002077_rage_pgstreamer_drain(ph: *mut Phantom); }
extern "C" { pub fn f_002078_rage_pgstreamerdevicehandler_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002079_rage_pgstreamer_setreadbulkcallback_void_ptr_unsigned_long_int_i(ph: *mut Phantom); }
extern "C" { pub fn f_002080_rage_pgbase_pgbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002091_rage_datresource_containsthisaddress_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002092_rage_datresource_getfixup_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002111_rage_parmanager_parmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002112_rage_parmanager_initialize_rage_parsettings_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002128_rage_parmanager_uninitialize(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002129_rage_atmapentry_rage_atnonfinalhashstring_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002130_rage_atmap_rage_atnonfinalhashstring_rage_atnonfinalhashstring_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002131_rage_atnonfinalhashstring_atnonfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002132_rage_parmanager_dtorparmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002141_rage_partreenode_dtorpartreenode(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002142_rage_parattributelist_findattribute_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002145_rage_parmanager_loadtree_rage_fistream_ptr_rage_parsettings_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002147_rage_parmanager_loadtree_char_const_ptr_char_const_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002148_rage_parmanager_savetree_rage_fistream_ptr_rage_partree_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002150_rage_parmanager_savetree_char_const_ptr_char_const_ptr_rage_part(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_002156_rage_parmanager_initfromstructure_rage_parstructure_ref_rage_par(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002158_rage_parinstancevisitor_visittoplevelstructure_rage_parparsables(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002161_rage_parstructure_issubclassof_rage_parstructure_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_002174_rage_parelement_findattributestringvalue_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_002175_rage_partree_dtorpartree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002176_rage_partree_createroot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002178_rage_parattributelist_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002181_rage_partreenode_findfromxpathinternal_char_const_ptr_rage_parat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002183_rage_partreenode_findchildwithindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002184_rage_partreenode_findchildwithname_char_const_ptr_rage_partreeno(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002188_rage_partreenode_appendaschildof_rage_partreenode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002191_rage_parelement_addattribute_char_const_ptr_double_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f64); }
extern "C" { pub fn f_002192_rage_parattributelist_addattribute_char_const_ptr_double_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f64, a3: i32) -> i64; }
extern "C" { pub fn f_002194_rage_parelement_addattribute_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_002195_rage_parattributelist_addattribute_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002197_rage_parelement_addattribute_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_002198_rage_parattributelist_addattribute_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_002199_rage_partreenode_createstdleaf_char_const_ptr_char_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002200_rage_parattributelist_addattribute_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_002201_rage_parelement_addattribute_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_002203_rage_parelement_findattributeboolvalue_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002204_rage_parattribute_finddoublevalue_const(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_002205_rage_parelement_findattributedoublevalue_char_const_ptr_double_b(ph: *mut Phantom, a0: i64, a1: i64, a2: f64, a3: i32) -> f64; }
extern "C" { pub fn f_002206_rage_parelement_findattributeint64value_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002214_rage_parattributelist_copyfrom_rage_parattributelist_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002215_rage_partreenode_findnumchildren_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002216_rage_partreenode_swapwith_rage_partreenode_ref_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002254_rage_parattributelist_findattributestringvalue_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002263_void_psofilereader_copyintobuffer_rage_matrix34_void_ptr_rage_ma(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002278_rage_psofile_dtorpsofile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002282_rage_psoloadfile_char_const_ptr_rage_psoloadaction_rage_atfixedb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002288_rage_psoloadfromstructure_rage_psofile_ref_rage_parstructure_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002335_unsigned_int_ref_rage_atmap_rage_parparsablestructure_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
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
extern "C" { pub fn f_002447_rage_parinstancevisitor_parinstancevisitor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002509_rage_parattributelist_findattributeintvalue_char_const_ptr_int_b(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002581_rage_parelement_setname_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002583_rage_parattributelist_findattributeanycase_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002586_void_rage_parcgudestructor_rage_parattributelist_rage_parparsabl(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002591_rage_parattribute_setvalue_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002592_rage_parattributelist_findattributefloatvalue_char_const_ptr_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> f32; }
extern "C" { pub fn f_002593_rage_parsettings_createstandardsettings(ph: *mut Phantom); }
extern "C" { pub fn f_002702_rage_qabug_clearbugbuffer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002703_rage_qabug_setid_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002704_rage_qabug_setowner_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002705_rage_qabug_beginfield_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002706_rage_qabug_addcustomfielddata_void_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002707_rage_qabug_endcustomfielddata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002708_rage_qabug_setproduct_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002709_rage_qabug_setdescription_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002712_rage_qabugstarinterface_qabugstarinterface(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002713_rage_qabugstarinterface_createbug_rage_qabug_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_002719_strlwr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002720_rage_conststring_vcopy_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002721_rage_conststring_dtorconststring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002725_rage_atstringbuilder_atstringbuilder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002726_rage_atstringbuilder_init_rage_sysmemallocator_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002739_rage_atpartialstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002743_rage_atdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002746_rage_stristr_char_ptr_char_const_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002747_rage_striskip_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002798_rage_sysappcontent_isamericanbuild(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002799_rage_sysappcontent_isjapanesebuild(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002802_rage_syshangdetectsavezoneenter(ph: *mut Phantom); }
extern "C" { pub fn f_002803_rage_syshangdetectsavezoneexit_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002811_rage_syshangdetectsetcrashfunc_void_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002812_rage_sysnamedpipe_sysnamedpipe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002819_rage_sysnamedpipe_close(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002834_rage_sysparam_findcommandlinearg_char_const_ptr_char_ptr_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002839_rage_sysparam_get_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002840_rage_sysparam_get_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002842_rage_sysparam_getarray_int_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002843_rage_sysparam_getarray_float_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002849_rage_sysbootmanager_isbootedfromdisc(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002852_rage_sysstack_printcapturedstacktrace_unsigned_long_const_ptr_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_002853_rage_sysstack_capturestacktrace_unsigned_long_ptr_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_002857_rage_systimer_haselapsedintervalms_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002862_rage_systhreadtype_addcurrentthreadtype_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_002871_rage_sysgetenv_char_const_ptr_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002879_rage_sysstackcollector_unregistertag_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002881_unsigned_int_ref_rage_atmap_unsigned_int_unsigned_int_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002886_rage_systhreadpool_thread_dtorthread(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002888_rage_systhreadpool_workitem_workitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002889_rage_systhreadpool_workitem_dtorworkitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002894_rage_systhreadpool_workitem_finished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002898_rage_systhreadpool_dtorsysthreadpool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002899_rage_systhreadpool_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002901_rage_systhreadpool_init(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002902_rage_systhreadpool_addthread_rage_systhreadpool_thread_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_002904_rage_systhreadpool_queuework_rage_systhreadpool_workitem_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002909_rage_systhreadinfo_adjustpriobasedonparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002910_rage_systhreadregistry_registerthread_rage_sysipcthreadid_ptr_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_002912_rage_systhreadregistry_adddelayedwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_002914_rage_systhreadtype_clearcurrentthreadtype_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_002916_rage_sysservice_updateclass(ph: *mut Phantom); }
extern "C" { pub fn f_002927_rage_sysmemstarttemp(ph: *mut Phantom); }
extern "C" { pub fn f_002928_rage_sysmemendtemp(ph: *mut Phantom); }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_002933_rage_sysmemgetbucketname_rage_memorybucketids(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_002935_rage_sysmemtotalfreememory(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002939_rage_sysmemmanager_sysmemmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002944_rage_sysmempoolallocator_init_void_ptr_unsigned_long_unsigned_lo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002945_rage_sysmempoolallocator_safeallocate_unsigned_long_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002947_rage_sysmempoolallocator_safefree_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002949_rage_sysmempoolallocator_forall_bool_ptr_void_ptr_void_ptr_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003084_rage_sysmemsystemtracker_print_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003088_rage_systaskmanager_addscheduler_char_const_ptr_int_int_rage_sys(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_003090_rage_systaskmanager_create_char_const_ptr_void_ptr_rage_systaskp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_003095_rage_systaskmanager_wait_rage_systaskhandle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003097_char_ptr_rage_formatf_char_16ul_char_ref_16ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003099_rage_syscompressionfactory_dtorsyscompressionfactory(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003100_rage_syscompressionfactory_registercompressiondata_rage_syscompr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003102_rage_syscompressiondata_syscompressiondata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003111_rage_sysdependencyscheduler_insertinternal_rage_sysdependency_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003136_rage_geomspheres_testspheretotriangle_rage_vec4v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_003159_rage_matrix34_fromquaternion_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003160_rage_matrix34_toquaternion_rage_quaternion_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003175_rage_vector3_vector3_rage_vector2_const_ref_rage_vector2_evector(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003186_rage_crframe_crframe_rage_crframedata_const_ref_unsigned_char_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_003214_rage_crframe_getvector3_unsigned_char_unsigned_short_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_003217_rage_crframe_getfloat_unsigned_char_unsigned_short_float_ref_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_003221_rage_crframe_dtorcrframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003296_rage_atarray_unsigned_short_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003313_rage_crframedata_crframedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003314_rage_crframedata_dtorcrframedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003315_rage_crframedata_init_unsigned_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_003809_rage_crframedata_adddof_unsigned_char_unsigned_short_unsigned_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_003830_rage_crtagiteratorbase_rage_crtagiteratorkey_true_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003835_rage_fwanimdirector_getmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003840_rage_fwclipsetmanager_getclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003878_rage_fwanimhelpers_getmovertracktranslationdiff_rage_crclip_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003887_rage_fwanimhelpers_getmovertrackdisplacement_rage_crclip_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003904_rage_fwanimmanager_findslot_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003910_rage_fwanimmanager_streamingrequest_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_003915_rage_fwanimmanager_get_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_003979_rage_fwclipset_isstreamedin_deprecated_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004038_rage_fwclipsetrequesthelper_fwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004280_rage_fwposematcher_fwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004389_rage_fwanimdirectorcomponentfacialrig_facialnodecontroller_setcl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004603_rage_fwextensionlist_add_rage_fwextension_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004619_rage_fwarchetype_getdrawable_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004620_rage_fwarchetype_getfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004621_rage_fwarchetype_getassetparenttxdindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004631_rage_fwarchetypemanager_getarchetypename_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004632_rage_fwarchetypemanager_lookupmodelid_rage_fwarchetype_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004633_rage_fwarchetypemanager_getarchetype_rage_atnonfinalhashstring_u(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004634_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004636_rage_fwarchetypemanager_getarchetypeindexfromhashkey_unsigned_in(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004639_rage_fwarchetypemanager_forallarchetypesinfile_int_void_ptr_rage(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_004667_rage_fwentity_protectstreamedarchetype(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004683_rage_fwentity_getaabb_rage_spdaabb_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004685_rage_fwentity_getrootlod(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004697_rage_fwsceneupdate_runcustomcallbackbits_unsigned_int_void_ptr_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_004761_rage_fwdebugbank_createbank_char_const_ptr_rage_functor0_rage_fu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_004763_rage_fwdebugbank_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004765_rage_fwdebugbank_removeallmainwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004767_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004774_rage_fwpickermanager_setenabled_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004775_rage_fwpickermanager_addpicker_rage_fwbasepicker_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004776_rage_fwpickermanager_ispickeroftypeenabled_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004777_rage_fwpickermanager_setpickermanagersettings_rage_fwpickermanag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004778_rage_fwpickermanager_setpickerinterfacesettings_rage_fwpickerint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004779_rage_fwpickermanager_setpickersettings_int_rage_fwbasepickersett(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_004780_rage_fwpickermanager_iscurrentowner_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004781_rage_fwpickermanager_takecontrolofpickerwidget_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004782_rage_fwpickermanager_addentitytypemask_unsigned_int_char_const_p(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_004784_rage_fwpickermanager_addentitytopickerresults_rage_fwentity_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004786_rage_fwpickermanager_gethoverentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004787_rage_fwpickermanager_resetlist_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004788_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004789_rage_fwpickermanager_setindexofselectedentity_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004790_rage_fwpickermanager_getindexofentitywithinresultsarray_rage_fwe(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004791_rage_fwpickermanager_getindexofselectedentity(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_004792_rage_fwpickermanager_getselectedentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004793_rage_fwpickermanager_getentity_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004794_rage_fwpickermanager_getnumberofentities(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_004798_rage_fwworldprobepicker_addarchetypemask_unsigned_int_char_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_004807_rage_fwvectormap_updatecore(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004814_rage_fwvectormap_updaterotation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004815_rage_fwvectormap_drawcore(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004816_rage_fwvectormap_drawline_rage_vector3_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_004817_rage_fwvectormap_drawlinethick_rage_vector3_const_ref_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_004819_rage_fwvectormap_drawmarker_rage_vector3_const_ref_rage_color32(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_004820_rage_fwvectormap_drawcircle_rage_vector3_const_ref_float_rage_co(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_004821_rage_fwvectormap_drawarc_rage_vector3_const_ref_float_float_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_004822_rage_fwvectormap_drawrectaxisaligned_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_004824_rage_fwvectormap_drawstring_rage_vector3_const_ref_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_004825_rage_fwvectormap_draw3dboundingboxonvmap_rage_spdaabb_const_ref(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004826_rage_fwvectormap_convertpointmaptoworld_float_float_float_ref_fl(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_004828_rage_fwvectormap_cleansettingscore(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004829_rage_fwvectormap_goodsettingscore(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004921_rage_dldrawcommandbuffer_registercommand_int_void_ptr_rage_dlcmd(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_004926_rage_dldrawcommandbuffer_allocatepagedmemory_rage_edrawlistpaget(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_004927_rage_dldrawcommandbuffer_movetonewpage_rage_edrawlistpagetype_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_004930_char_ptr_rage_formatf_char_10ul_char_ref_10ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004935_rage_dlcmdbase_op_new_unsigned_long_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004948_rage_dlcmdnewdrawlist_dlcmdnewdrawlist_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004961_rage_dlcmdgrcdeviceupdatebuffer_dopendingbufferupdates(ph: *mut Phantom); }
extern "C" { pub fn f_004963_rage_dlcmdpushtimebar_dlcmdpushtimebar_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_004964_rage_dlcmdpoptimebar_dlcmdpoptimebar(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004992_rage_dldrawlistmgr_getdrawlistentitycount_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004996_rage_dldrawlistmgr_endcurrentdrawlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004997_rage_dldrawlistmgr_begindrawlistcreation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004998_rage_dldrawlistmgr_enddrawlistcreation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005011_rage_dldrawlistmgr_flipupdatefenceidx(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005017_rage_dldrawlistmgr_setstatshighlight_bool_char_const_ptr_bool_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_005058_rage_fwtextutil_copychar16stringintoasciibuffer_char_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_005279_rage_fwentitylist_addentity_rage_fwentity_ptr_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: f32, a6: i32); }
extern "C" { pub fn f_005284_rage_fwrenderlistgroup_clear(ph: *mut Phantom); }
extern "C" { pub fn f_005287_rage_fwrenderlistgroup_waitforsortentitylistjob(ph: *mut Phantom); }
extern "C" { pub fn f_005301_void_rage_dlc_addinternal_void_ptr_void_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005303_rage_fwrenderphasemanager_showscanresults_bool_const(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005315_rage_fwrenderthreadinterface_flush_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005316_rage_fwrenderthreadinterface_flushanddevicecheck(ph: *mut Phantom); }
extern "C" { pub fn f_005318_rage_fwrenderthreadinterface_setrenderfunction_rage_functor0(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005319_rage_fwrenderthreadinterface_setdefaultrenderfunction(ph: *mut Phantom); }
extern "C" { pub fn f_005333_rage_fwassetlocation_gettxdlist_rage_fwptrlistsinglelink_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005356_rage_fwmaptypesstore_getname_rage_strlocalindex_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005386_rage_fwmapdatastore_requestgroup_rage_strlocalindex_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_005387_rage_fwmapdatastore_removegroup_rage_strlocalindex_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_005392_rage_fwmapdatadebug_getblockindex_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005400_rage_fwmapdatastore_isscriptmanaged_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005460_rage_fwboxstreamer_requestaboutpos_rage_vec3v_const_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_005463_rage_fwboxstreamer_ensureloadedaboutpos_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005470_rage_fwboxstreamer_updatedebug(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005471_rage_fwboxstreamer_renderdebug_rage_color32_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005502_rage_fwassetstore_rage_pgdictionary_rage_charactercloth_rage_fwc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005561_sprintf_s_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005562_rage_fwassetstore_rage_pgdictionary_rage_grctexture_rage_fwtxdde(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005573_rage_fwmapdatastore_registerstreamingmodule(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005574_rage_fwassetstore_rage_fwmapdatacontents_rage_fwmapdatadef_final(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005577_rage_fwmapdatastore_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005580_rage_fwmapdatastore_saferemoveall_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005619_rage_fwmaptypesstore_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005620_rage_fwmaptypesstore_setdepth_rage_atarray_int_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_005621_rage_fwmaptypesstore_saferemovetypeslot_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005624_rage_fwmaptypesstore_update(ph: *mut Phantom); }
extern "C" { pub fn f_005681_rage_fwstaticboundsstore_getslotdata_int_rage_atsnode_rage_fwbou(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_005701_rage_fwtxdstore_loadfile_rage_strlocalindex_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_005706_rage_fwtxdstore_getsafefromindex_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005718_rage_fwbaseentitycontainer_getentitydescindex_rage_fwentity_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005719_rage_fwbaseentitycontainer_getapproxboundingbox_unsigned_int_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_005724_rage_fwentitycontainer_getentityarray_rage_atarray_rage_fwentity(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005730_rage_fwinteriorlocation_makeinvalid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005731_rage_fwinteriorlocation_setroomindex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005732_rage_fwinteriorlocation_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005733_rage_fwinteriorlocation_getroomindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005745_rage_fwworldmgr_prescan(ph: *mut Phantom); }
extern "C" { pub fn f_005761_rage_fwscan_getgbufscreenquadpair_rage_fwscenegraphnode_const_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005798_rage_fwsearch_start_rage_fwscenegraphnode_ptr_rage_fwsearchvolum(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_005799_rage_fwsearch_finalize(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005800_rage_fwsearch_executecallbackonresult_bool_ptr_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_005917_rage_fwrefawarebaseimpl_rage_datbase_dtorfwrefawarebaseimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005922_rage_aitask_findsubtaskoftype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005931_rage_aitaskmanager_process_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_005946_rage_aitasktree_cleartask_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005947_rage_aitasktree_getactiveleaftask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005949_rage_aitasktree_findtaskbytypeactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005950_rage_aitasktree_findtaskbytypewithpriority_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_005969_rage_cnavmesh_getpolycentroid_unsigned_int_rage_vector3_ref_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_005978_rage_cnavmesh_getpolybelowpoint_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_005989_rage_ainavmeshstore_ainavmeshstore_rage_ainavdomain_rage_fwnavme(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_006004_rage_fwpathserver_getnavmeshindexfromsector_int_int_rage_ainavdo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006032_rage_atmap_int_bool_rage_atmaphashfn_int_rage_atmapequals_int_ra(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006035_rage_atmapentry_unsigned_int_bool_const_ref_rage_atmap_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006056_rage_strstreaminginfomanager_removeobject_rage_strindex_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006063_rage_strstreamingmodulemgr_addmodule_rage_strstreamingmodule_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006064_rage_strstreamingvisualize_send_int_void_const_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_006066_rage_strpackfilemanager_getimagefilefromhandle_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006067_rage_strpackfilemanager_getimagefileindexfromhandle_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006069_rage_strpackfilemanager_getstreamingmodule(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_006072_rage_strstreaminginfomanager_getobjectname_rage_strindex_char_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_006075_rage_strstreaminginfo_computevirtualsize_rage_strindex_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006076_rage_strstreaminginfo_computephysicalsize_rage_strindex_bool_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006084_rage_strstreaminginfomanager_requestobject_rage_strindex_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006085_rage_strpackfilemanager_registerindividualfile_char_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_006088_rage_atarray_rage_atnonfinalhashstring_0_unsigned_short_grow_uns(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006101_rage_strstreaminginfomanager_setrequiredflag_rage_strindex_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006110_rage_strpackfilemanager_evictunusedimages(ph: *mut Phantom); }
extern "C" { pub fn f_006122_rage_strstreaminginfomanager_getmodule_rage_strindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006123_rage_strstreamingloadermanager_callkeepalivecallback(ph: *mut Phantom); }
extern "C" { pub fn f_006124_rage_strstreamingengine_getobjectname_rage_strindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006125_rage_strrequestrecorder_endrecording(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006156_rage_atarray_rage_strindex_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006159_rage_strstreaminginfo_computeoccupiedvirtualsize_rage_strindex_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006162_rage_strstreaminginfo_gethandle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006167_rage_strstreaminginfomanager_isobjectinimage_rage_strindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006175_rage_strstreaminginfomanager_clearrequiredflag_rage_strindex_uns(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006179_rage_strstreaminginfomanager_getobjectanddependenciessizes_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_006188_rage_atarray_int_0_unsigned_short_ptr_ref_rage_atmap_int_rage_at(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006207_rage_strstreaminglive_init(ph: *mut Phantom); }
extern "C" { pub fn f_006208_rage_strstreaminglive_addlivefolder_char_const_ptr_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006215_rage_strstreamingloadermanager_loadrequestedobjects(ph: *mut Phantom); }
extern "C" { pub fn f_006225_rage_atvarstring_atvarstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_006231_rage_strstreamingloader_getloaderstats_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006235_rage_strstreamingvisualize_setmarkertext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006237_rage_strstreamingloadermanager_processasyncfiles(ph: *mut Phantom); }
extern "C" { pub fn f_006238_rage_strstreamingloadermanager_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006241_rage_strstreamingvisualize_addmarker_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006246_rage_strstreamingmodule_getvirtualmemoryofloadedobj_rage_strloca(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_006247_rage_strstreamingmodule_getphysicalmemoryofloadedobj_rage_strloc(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_006249_rage_strstreamingmodule_resetplacestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006252_rage_strstreamingmodule_streamingblockingload_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006253_rage_strmoduletypeindextoextension_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006254_rage_strrequest_request_rage_strlocalindex_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_006255_rage_strrequest_hasloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006256_rage_strrequest_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006257_rage_strrequest_clearrequiredflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006259_rage_strrequest_setrequiredflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006261_rage_strrequest_getindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006263_rage_strmodelrequest_request_rage_strlocalindex_rage_atnonfinalh(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_006264_rage_strmodelrequest_hasloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006265_rage_strmodelrequest_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006266_rage_strmodelrequest_isvalid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006267_rage_strmodelrequest_clearrequiredflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006284_rage_strstreamingvisualize_registerdevice_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_006361_rage_atarray_float_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006886_rage_audmetadatamanager_getobjectnamefromnametableoffset_unsigne(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006971_rage_audengine_triggerupdate(ph: *mut Phantom); }
extern "C" { pub fn f_006972_rage_audengine_waitforaudioframe(ph: *mut Phantom); }
extern "C" { pub fn f_007036_rage_audmetadatamanager_getobject_verify_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007078_rage_audsoundset_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007079_rage_audsoundset_find_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007281_rage_audmixerdevice_setreplaymixermodeloading(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007412_rage_audentity_createandplaysound_unsigned_int_rage_audsoundinit(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007427_rage_vfxutil_outputfxbucketmemorycurrent_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007428_rage_vfxutil_outputfxbucketmemorytotal_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007497_rage_decalmanager_remove_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007569_rage_ptfxscript_trigger_rage_atnonfinalhashstring_rage_atnonfina(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_007576_rage_ptfxscript_start_rage_atnonfinalhashstring_rage_atnonfinalh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: f32, a7: i32) -> i32; }
extern "C" { pub fn f_007577_rage_ptfxscript_stop_int_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007578_rage_ptfxscript_remove_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007583_rage_ptfxscript_updateoffset_int_rage_mat34v_const_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_007633_rage_fwuniqueobjectidgenerator_reset(ph: *mut Phantom); }
extern "C" { pub fn f_007635_rage_atmapentry_void_ptr_unsigned_int_const_ref_rage_atmap_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007640_rage_fworientator_addpoint_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007641_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_007644_rage_forcenopopupsauto_forcenopopupsauto_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007645_rage_forcenopopupsauto_dtorforcenopopupsauto(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007658_rage_fwptrlistsinglelink_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007668_rage_fwquadtreenode_forallmatching_rage_vector3_const_ref_rage_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007680_rage_fwkdop18face_fwkdop18face_rage_vec4v_const_ref_rage_vec3v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i32) -> i64; }
extern "C" { pub fn f_007682_rage_atmap_unsigned_int_rage_fwkdop18_rage_atmaphashfn_unsigned(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007683_rage_fwkdop18_buildverticessoa_rage_fwkdop18verticessoa_ref_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007684_rage_fwkdop18_debugdrawbound_rage_color32_const_ref_rage_grcview(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_007685_rage_polyclip_rage_vec3v_ptr_int_rage_vec3v_const_ptr_int_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_007694_rage_fwgeom_intersectsphereray_rage_spdsphere_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_007754_rage_netinterface_getnumactiveplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007760_rage_netinterface_getnumremotephysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007763_rage_netinterface_getremotephysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007780_rage_netplayermgrbase_sendbuffer_rage_netplayer_const_ptr_void_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_007855_rage_netplayer_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007995_rage_matrix34_toeulersxyz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008006_rage_fwhttpquery_fwhttpquery_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_008008_rage_fwhttpquery_fwhttpquery_char_const_ptr_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_008009_rage_fwhttpquery_dtorfwhttpquery(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008011_rage_fwhttpquery_init_char_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_008013_rage_fwhttpquery_writebuffertofile_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008014_rage_fwhttpquery_initpost_char_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008015_rage_fwhttpquery_appendxmlcontent_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008016_rage_fwhttpquery_commitpost(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008017_rage_fwhttpquery_pending(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008018_rage_fwhttpquery_createtree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008059_char_ptr_rage_formatf_char_5ul_char_ref_5ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008070_rage_netobject_getplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008074_rage_netobject_ispendingownerchange_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008122_rage_netplayer_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008213_rage_netobjectreassignmgr_adddebugwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_008397_rage_netscgamerdatamgr_createdebugwidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008682_rage_fwbasepool_fwbasepool_int_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_008683_rage_fwconfigmanager_getsizeofpool_unsigned_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008686_rage_fwbasepool_getindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008688_rage_fwbasepool_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008689_rage_fwscriptguid_createguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008693_rage_fwscriptguid_getbasefromguid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_008765_bool_rage_netmessage_writeheader_rage_datexportbuffer_unsigned_i(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_008768_rage_scripthandlermgr_removescriptresource_unsigned_short_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_008867_rage_fwconfigmanager_load_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008869_rage_fwconfigmanager_fwconfigmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008874_rage_fwassetstore_rage_pgdictionary_rage_rmcdrawable_rage_dwddef(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008876_rage_gameskeleton_dtorgameskeleton(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008878_rage_gameskeleton_setcurrentupdatetype_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008879_rage_gameskeleton_registerinitsystem_void_ptr_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008882_rage_gameskeleton_registershutdownsystem_void_ptr_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008883_rage_gameskeleton_registerupdatesystem_void_ptr_rage_atnonfinalh(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008884_rage_gameskeleton_pushupdategroup_rage_atnonfinalhashstring_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_008885_rage_gameskeleton_popupdategroup(ph: *mut Phantom); }
extern "C" { pub fn f_008886_rage_gameskeleton_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008887_rage_gameskeleton_shutdown_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008888_rage_gameskeleton_update_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008917_rage_fwtimer_enduserpause(ph: *mut Phantom); }
extern "C" { pub fn f_008918_rage_fwtimer_startuserpause_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008921_rage_fwtimer_setforcedfps_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_008922_rage_fwtimer_unsetforcedfps(ph: *mut Phantom); }
extern "C" { pub fn f_008923_rage_fwtimer_setkey_pause_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008924_rage_fwtimer_update(ph: *mut Phantom); }
extern "C" { pub fn f_008931_bool_ref_rage_atmap_rage_strindex_bool_rage_atmaphashfn_rage_str(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_008943_rage_fwbasepool_fwbasepool_char_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_008945_rage_fwbasepool_forall_bool_ptr_void_ptr_void_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008949_rage_fwbasepool_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008950_rage_fwbasepool_getjustindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008951_rage_fwbasepool_isvalidptr_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008952_rage_fwbasepool_getat_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008953_rage_fwknownrefholder_fwknownrefholder_void_ptr_ptr_rage_fwknown(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_008996_rage_fwbasepool_dtorfwbasepool_14863(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009048_rage_crproperty_getattribute_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009207_rage_crskeletondata_findbonedata_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009232_rage_crskeleton_attach_bool_rage_matrix43_ptr_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009233_rage_crskeleton_getglobalmtx_unsigned_int_rage_mat34v_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009266_rage_crjointdata_gettranslationlimits_rage_crbonedata_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_009283_rage_grmmodel_getgeometry_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009287_rage_grmmodel_computebucketmask_rage_grmshadergroup_const_ref_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009291_rage_grmshader_begindraw_rage_grmshader_edrawtype_bool_rage_grce(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_009293_rage_grmshader_bind_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009294_rage_grmshader_unbind_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009295_rage_grmshader_enddraw_const(ph: *mut Phantom); }
extern "C" { pub fn f_009337_rage_pgdictionary_rage_grctexture_addentry_char_const_ptr_rage_g(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_009393_rage_pftimebars_init_char_const_ptr_int_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_009394_rage_pftimebars_frameinit_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009405_rage_pftimebars_getfunctiontotals_char_const_ptr_int_ref_float_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009412_rage_pftimebars_dumpxml_int_int_float_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i64); }
extern "C" { pub fn f_009428_rage_pftimebarmgr_init_char_const_ptr_int_float_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_009429_rage_pftimebarmgr_frameinit_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009430_rage_pftimebarmgr_frameend(ph: *mut Phantom); }
extern "C" { pub fn f_009432_rage_pftimebarmgr_getfunctiontotals_char_const_ptr_int_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_009449_rage_pfdrawitembase_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009457_rage_pfdrawitemslider_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009461_rage_pfdrawitemsliderfull_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009463_rage_pfdrawitemsliderintfull_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009479_rage_getrageprofiledraw(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009503_rage_grcfvf_gettexturechannel_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009509_rage_grcfvf_getoffsetnoassert_rage_grcfvf_grcfvfchannels_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009517_rage_grcimage_release_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009518_rage_grcimage_create_unsigned_int_unsigned_int_unsigned_int_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_009520_rage_grcimage_recalculatestride(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009530_rage_grcstateblock_grcstateblockstore_rage_grcstateblock_grcblen(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009539_rage_grceffect_lookuptechnique_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009540_rage_grceffect_lookupvar_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009542_rage_grcstateblock_createblendstate_rage_grcblendstatedesc_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009544_rage_grcdrawsinglequadf_float_float_float_float_float_float_floa(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: i64); }
extern "C" { pub fn f_009545_rage_grcstateblock_setblendstate_rage_grcstateblock_grcblendstat(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009548_rage_grcdevice_drawvertices_void_const_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_009555_rage_grcstateblock_setrasterizerstate_rage_grcstateblock_grcrast(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009560_rage_grcstateblock_setdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009561_rage_grcbegin_rage_grcdrawmode_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009562_rage_grcvertex_float_float_float_float_float_float_rage_color32(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: f32, a8: f32); }
extern "C" { pub fn f_009563_rage_grcend(ph: *mut Phantom); }
extern "C" { pub fn f_009586_rage_gettexturetemplatetypestring_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009588_rage_grcimage_dtorgrcimage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009589_rage_grcimage_getextramipcount_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009593_rage_grcimage_getpixelblock_rage_color32_ptr_int_int_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_009595_rage_grcimage_releasealias(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009598_rage_grcimage_savedds_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009599_rage_grcimage_getlayercount_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009614_rage_grcviewport_setcurrent_rage_grcviewport_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009615_rage_grceffect_lookupglobalvar_char_const_ptr_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009618_rage_grcviewport_perspective_float_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_009619_rage_grcviewport_ortho_float_float_float_float_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_009620_rage_grcviewport_setprojection_rage_mat44v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009634_rage_grcviewport_setcurrentworldidentity(ph: *mut Phantom); }
extern "C" { pub fn f_009638_rage_grcviewport_isspherevisible_rage_vec4v_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009646_rage_grcviewport_transform_rage_vec3v_const_ref_float_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009648_rage_grcviewport_reversetransform_float_float_rage_vec3v_ref_rag(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i64); }
extern "C" { pub fn f_009650_rage_grcbindtexture_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009654_rage_grcfont_drawscaled_float_float_float_rage_color32_float_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: f32, a6: f32, a7: i64); }
extern "C" { pub fn f_009655_rage_grcfont_getstringwidthinternal_char_const_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009662_rage_grcsetup_connecttorag_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009672_rage_grcadaptermanagerd3d11_enumerate_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009675_rage_grcstateblock_createdepthstencilstate_rage_grcdepthstencils(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009694_rage_grcsetup_takescreenshot_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009696_rage_grcdevice_capturescreenshottojpegfile_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009698_rage_grcdraw2dtext_float_float_char_const_ptr_bool_float_float_r(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i32, a4: f32, a5: f32, a6: i64); }
extern "C" { pub fn f_009699_rage_grcdevice_capturescreenshot_rage_grcimage_ptr(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009702_rage_grcimage_writepng_char_const_ptr_void_ptr_unsigned_char_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: f32) -> i32; }
extern "C" { pub fn f_009704_rage_monitorconfiguration_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009705_rage_grcsetup_addscreenshotviewerwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009726_rage_grcbeginquadsinternal_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009730_rage_grcbatcher_init_unsigned_long_rage_grcbatcher_bufferfullmod(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009769_rage_grcvertexbuffereditor_grcvertexbuffereditor_rage_grcvertexb(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_009771_rage_grcvertexbuffereditor_dtorgrcvertexbuffereditor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009776_rage_grcbuffereditor_getnormal_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009777_rage_grcbuffereditor_getposition_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009780_rage_grcbuffereditor_getblendindices_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009788_rage_grcindexbuffer_lockro_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009789_rage_grcindexbuffer_unlockwo_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009808_rage_grceffect_getvardesc_rage_grceffectvar_char_const_ptr_ref_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_009809_rage_grcinstancedata_getvariablename_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009814_rage_grceffect_setvar_rage_grcinstancedata_ref_rage_grceffectvar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009847_rage_grceffect_setshaderquality_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009848_rage_grcinstancedata_load_char_const_ptr_rage_fitokenizer_ref_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_009858_rage_grcstateblock_createrasterizerstate_rage_grcrasterizerstate(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009859_rage_grceffect_setglobalfloatcommon_int_float_ptr_int_rage_grcef(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_009869_rage_grceffect_getlocalcommon_rage_grcinstancedata_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_009871_rage_grceffect_setglobalvar_rage_grceffectglobalvar_int_const_re(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009872_rage_grceffect_setglobalvar_rage_grceffectglobalvar_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009875_rage_grceffect_setlocalcommon_rage_grcinstancedata_ref_rage_grce(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_009880_rage_grceffect_getvar_rage_grcinstancedata_const_ref_rage_grceff(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009893_rage_grcstateblock_setdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009894_rage_grcstateblock_setblendstate_rage_grcstateblock_grcblendstat(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_009895_rage_grcinstancedata_grcinstancedata_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009903_rage_grcstateblock_setstates_rage_grcstateblock_grcrasterizersta(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_009904_rage_gridmonitor_getarea_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009905_rage_monitorconfiguration_ismultihead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009908_rage_monitorconfiguration_update(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009909_rage_grcadaptermanager_getadapter_unsigned_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_009912_rage_monitorconfiguration_getlandscapemonitor_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009925_rage_grcadapterd3d11output_getdesc_int_unsigned_int_rage_grcadap(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009930_rage_grcadapterd3d11_getoutput_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009964_rage_grcresourcecache_getinstance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009997_rage_grcdevice_lockcontext(ph: *mut Phantom); }
extern "C" { pub fn f_009999_rage_grcdevice_unlockcontext(ph: *mut Phantom); }
extern "C" { pub fn f_010000_rage_grcdevice_lockdeviceresetavailability(ph: *mut Phantom); }
extern "C" { pub fn f_010001_rage_grcdevice_unlockdeviceresetavailability(ph: *mut Phantom); }
extern "C" { pub fn f_010012_rage_grcdevice_endcreatevertices_void_const_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_010019_rage_grcdevice_checkdevicestatus(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_010039_rage_grcdevice_getwidescreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_010040_rage_grcdevice_gethidef(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_010091_rage_grctexturefactorydx11_translatetorageformat_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_010102_rage_grctexturefactorydx11_getd3dfromatfromgrcimageformat_unsign(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_010151_rage_dxdiag_dumpdisplayableinfostring_char_ptr_ptr_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010155_rage_batcherring_batcherring_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010159_rage_grcdebugdraw_setdisplaydebugtext_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010163_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010166_rage_grcdebugdraw_quad_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010167_rage_grcdebugdraw_poly_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010169_rage_grcdebugdraw_circle_rage_vec2v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010170_rage_grcdebugdraw_rectaxisaligned_rage_vec2v_const_ref_rage_vec2(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010172_rage_grcdebugdraw_text_float_float_rage_color32_char_const_ptr_b(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i64, a4: i32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010179_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010181_rage_grcdebugdraw_poly_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010182_rage_grcdebugdraw_poly_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_010183_rage_grcdebugdraw_cross_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010184_rage_grcdebugdraw_circle_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010188_rage_grcdebugdraw_boxaxisaligned_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010189_rage_grcdebugdraw_boxoriented_rage_vec3v_const_ref_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010190_rage_grcdebugdraw_cylinder_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010191_rage_grcdebugdraw_cylinder_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_010192_rage_grcdebugdraw_cone_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010193_rage_grcdebugdraw_capsule_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010194_rage_grcdebugdraw_axis_rage_mat34v_const_ref_float_bool_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_010195_rage_grcdebugdraw_arrow_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_010196_rage_grcdebugdraw_gizmoposition_internal_rage_mat34v_const_ref_f(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_010197_rage_grcdebugdraw_pushdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010198_rage_grcdebugdraw_popdepthstencilstate(ph: *mut Phantom); }
extern "C" { pub fn f_010199_rage_grcdebugdraw_gizmorotation_internal_rage_mat34v_const_ref_f(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_010200_rage_grcdebugdraw_setcullingviewport_rage_grcviewport_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010209_rage_grcdebugdraw_setdisableculling_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010210_rage_grcdebugdraw_setdodebugztest_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010211_rage_grcdebugdraw_setscreenspacetextheight_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010212_rage_grcdebugdraw_setdrawlabelscalex_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_010213_rage_grcdebugdraw_setdrawlabelscaley_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_010214_rage_grcdebugdraw_textfontpush_rage_grcfont_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010215_rage_grcdebugdraw_textfontpop(ph: *mut Phantom); }
extern "C" { pub fn f_010216_rage_grcdebugdraw_text_float_float_rage_dd_epixelcoordspace_rage(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i32, a4: i64, a5: i32, a6: f32, a7: f32); }
extern "C" { pub fn f_010217_rage_grcdebugdraw_printtoscreencoors_char_const_ptr_int_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010218_rage_grcdebugdraw_printtoscreencoors_char_const_ptr_rage_grcfont(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010219_rage_grcdebugdraw_adddebugoutputex_bool_rage_color32_char_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_010220_rage_grcdebugdraw_adddebugoutput_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010221_rage_grcdebugdraw_adddebugoutput_rage_color32_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_010224_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010225_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010453_rage_ioinput_end(ph: *mut Phantom); }
extern "C" { pub fn f_010480_rage_iojoystick_getproductname_unsigned_short_ptr_unsigned_int_c(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010496_rage_iomapper_getmappedsources_rage_iovalue_const_ref_rage_atfix(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_010500_rage_iovalue_isdown_float_rage_iovalue_readoptions_const_ref_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010511_rage_iovalue_ispressed_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010518_rage_iovalue_getnorm_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010521_rage_iovalue_getnorm01_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010700_rage_rmclodgroup_getbucketmask_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_010736_rage_spdplane_intersectrayv_rage_vec3v_const_ref_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_010919_rage_cutfevent_initclass(ph: *mut Phantom); }
extern "C" { pub fn f_010920_rage_cutfevent_seteventiddisplaynamefunc_rage_functor1ret_char_c(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010922_rage_cutfevent_getdisplayname_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_010931_rage_cutfobjectidevent_cutfobjectidevent_int_float_int_rage_cutf(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_010932_rage_cutfobjectidevent_cutfobjectidevent_int_float_int_bool_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32, a4: i32, a5: i32, a6: i64); }
extern "C" { pub fn f_011012_rage_cutfcameracuteventargs_cutfcameracuteventargs(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011024_rage_cutfeventargs_seteventargstypenamefunc_rage_functor1ret_cha(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011030_rage_cutfcutscenefile2_converttoparattributelist_rage_cutfattrib(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011036_rage_cutfnameeventargs_cutfnameeventargs_char_const_ptr_rage_par(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_011041_rage_cutfboolvalueeventargs_cutfboolvalueeventargs_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_011044_rage_cutffloatboolvalueeventargs_cutffloatboolvalueeventargs_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i64; }
extern "C" { pub fn f_011047_rage_cutffloatvalueeventargs_cutffloatvalueeventargs_float(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_011050_rage_cutftwofloatvalueseventargs_cutftwofloatvalueseventargs_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i64; }
extern "C" { pub fn f_011062_rage_cutfobjectidlisteventargs_dtorcutfobjectidlisteventargs(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011071_rage_cutfobjectvariationeventargs_cutfobjectvariationeventargs_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_011072_rage_cutfobjectvariationeventargs_dtorcutfobjectvariationeventar(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011085_rage_cutfcameracuteventargs_removecocmodifier_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011089_rage_cutfcascadeshadoweventargs_cutfcascadeshadoweventargs_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_011093_rage_cutfcascadeshadoweventargs_getposition_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011094_rage_cutfcascadeshadoweventargs_getcameracuthash_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011206_rage_cutfcutscenefile2_cutfcutscenefile2(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011209_rage_cutfcutscenefile2_savefile_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_011213_rage_cutfcutscenefile2_loadandreplacelightsincutfile_char_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011214_rage_cutfcutscenefile2_loadsubtitlefile_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_011215_rage_cutfcutscenefile2_loadgamedatatunefile_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_011216_rage_cutfcutscenefile2_getassetpathforscene_rage_atstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_011217_rage_cutfcutscenefile2_loadandparselightdata_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_011218_rage_cutfcutscenefile2_removeeventfromlist_rage_cutfevent_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011219_rage_cutfcutscenefile2_removeobject_rage_cutfobject_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011220_rage_cutfcutscenefile2_addeventtolist_rage_cutfevent_ptr_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011222_rage_cutfcutscenefile2_findobjectsoftype_int_rage_atarray_rage_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_011224_rage_cutfcutscenefile2_addobject_rage_cutfobject_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011225_rage_cutfcutscenefile2_findeventsforobjectidonly_int_rage_atarra(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_011228_rage_cutfcutscenefile2_getcurrentsectiontimelist_rage_atarray_fl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011335_rage_cutfmodelobject_gettypefile_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011368_rage_cutfrayfireobject_getstartposition_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011369_rage_cutsentity_dispatchevent_rage_cutsmanager_ptr_rage_cutfobje(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011370_rage_cutsuniqueentity_cutsuniqueentity_rage_cutfobject_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_011371_rage_cutsanimationmanagerentity_cutsanimationmanagerentity_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_011372_rage_cutsanimationmanagerentity_dtorcutsanimationmanagerentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011375_rage_cutsanimationmanagerentity_getsectionedname_char_ptr_int_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_011379_rage_cutsmanager_getsectionfortime_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_011380_rage_cutsanimationmanagerentity_getsectionedname_unsigned_int_un(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_011381_rage_cutsanimationmanagerentity_dispatchevent_rage_cutsmanager_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32, a6: i32); }
extern "C" { pub fn f_011384_rage_cutsmanager_getobjectbyid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_011389_rage_cutsmanager_getmodelobjectsoftype_rage_atarray_rage_cutfobj(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_011390_rage_cutsmanager_setisloading_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011391_rage_cutsmanager_setisloaded_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_011395_rage_cutsmanager_getentitybyobjectid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_011398_rage_cutsmanager_geteditlightinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_011400_rage_cutsmanager_getconcatsectionfortime_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_011401_rage_cutsmanager_getsceneorientationmatrix_int_rage_matrix34_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_011402_rage_cutsmanager_getsceneorientationmatrix_rage_matrix34_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011405_rage_cutsmanager_dtorcutsmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011406_rage_cutsmanager_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011409_rage_cutsmanager_fadeoutgameatbeginning_rage_cutsmanager_escreen(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011411_rage_cutsmanager_fadeoutcutsceneatend_rage_cutsmanager_escreenfa(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_011412_rage_cutsmanager_stop_rage_cutsmanager_escreenfadeoverride_rage(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011420_rage_cutsmanager_setscenerotation_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_011424_rage_seditcutflightinfo_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011425_rage_cutsmanager_inialiselighteditinfo_rage_seditcutflightinfo_p(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011426_rage_cutsmanager_updatelightcombo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011432_rage_cutsmanager_play(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011436_rage_cutfevent_dtorcutfevent_18542(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011437_rage_cutsmanager_fadeout_rage_color32_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_011439_rage_cutsmanager_getentityobject_rage_cutfobject_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_011441_rage_cutsmanager_presceneupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011442_rage_cutsmanager_updatesectioninfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011443_rage_cutsmanager_isconcatsectionvalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011446_rage_cutsmanager_convertangledareatomatrixandvectors_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64, a5: i64, a6: f32); }
extern "C" { pub fn f_011448_rage_cutsmanager_debugdraw(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011450_rage_cutsmanager_isloading_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011453_rage_cutsmanager_isconcatted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011454_rage_cutsmanager_getoriginalsceneorientationmatrix_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011455_rage_cutsmanager_getsectionstarttime_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_011456_rage_cutsmanager_isconcatdatavalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011457_rage_cutsmanager_getnextvalidconcatsection_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011458_rage_cutsmanager_isconcatsectionvalidforplayback_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011459_rage_cutsmanager_getconcatsectionstarttime_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_011460_rage_cutsmanager_getconcatsectionduration_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_011463_rage_cutsmanager_doloadingcutfilestate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011466_rage_cutsmanager_getobjectidsineventlist_rage_atarray_rage_cutfe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_011467_rage_cutsmanager_doloadingstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011469_rage_cutsmanager_updateplaybackevents(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011470_rage_cutsmanager_doplaystate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011472_rage_cutsmanager_dispatcheventtoobjecttype_int_int_rage_cutfeven(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_011473_rage_cutsmanager_dostoppedstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011476_rage_cutsmanager_deleteentityobject_rage_cutfobject_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011478_rage_cutsmanager_fadeingameatend_rage_cutsmanager_escreenfadeove(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011481_rage_cutsmanager_dispatcheventtoallentities_int_rage_cutfeventar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_011482_rage_cutsmanager_findaudioobjectid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011483_rage_cutsmanager_getstatename_rage_cutsmanager_ecutscenestate(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_011484_rage_cutsmanager_createvcrwidget_rage_bkbank_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011485_rage_cutsmanager_bankfastforwardcallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011486_rage_cutsmanager_bankstepforwardcallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011487_rage_cutsmanager_bankpausecallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011488_rage_cutsmanager_bankstepbackwardcallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011489_rage_cutsmanager_bankplayforwardscallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011490_rage_cutsmanager_bankplaybackwardscallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011491_rage_cutsmanager_bankrewindcallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011530_rage_cutsmanager_removewidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011537_rage_cutsmanager_getselectedlightinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011556_rage_cutsmanager_addlighttoactivelist_rage_seditcutflightinfo_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011557_rage_cutsmanager_updateactivelightlistcombo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011559_rage_cutsmanager_duplicatelighteditinfo_rage_cutfobject_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_011560_rage_cutsmanager_savedrawdistances(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011563_rage_cutsmanager_setframerangeforplaybacksliders(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011564_rage_cutsmanager_bankdisplaylightlineschangedcallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011565_rage_cutsmanager_bankactivelightselectedcallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011566_rage_cutsmanager_banklightselectedcallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011576_rage_cutsmanager_bankoriginalscenespacetoworldspace_rage_matrix3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011745_rage_fraginst_setresetmatrix_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011751_rage_fraginst_getcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011776_rage_fragtype_getboneindexfromid_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011853_rage_fragmanager_createallocator(ph: *mut Phantom); }
extern "C" { pub fn f_011870_rage_fragtunestruct_settype_rage_fragtype_ptr_int_rage_fraginst(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011885_rage_phiterator_dtorphiterator(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011966_rage_fragtype_getskinned_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012021_rage_fragphysicslodgroup_getlodbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012050_rage_fragcacheallocator_externalallocate_unsigned_long_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_012211_rage_ptxkeyframe_addentry_rage_scalarv_const_ref_rage_vec4v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012271_rage_ptxkeyframe_gettimev_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_012272_rage_ptxkeyframe_setentry_int_float_rage_vec4v_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64); }
extern "C" { pub fn f_012312_rage_ptxshaderinst_setcustomoverridefunc_void_ptr_rage_atarray_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012373_rage_ptxmanager_geteffectrule_rage_atnonfinalhashstring_rage_ptx(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_012433_rage_ptxmanager_geteffectrule_rage_atnonfinalhashstring_rage_atn(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_012892_rage_characterclothcontroller_setpackageindex_unsigned_char_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_012911_rage_charactercloth_setpose_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013050_rage_matrix34_transform3x3_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013155_rage_rsttestaabbexactedgelist_bool_rage_mat44v_const_ref_rage_sc(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i32, a13: i32, a14: i32); }
extern "C" { pub fn f_013191_rage_rstgetboxhull_rage_vec3v_const_ref_rage_vec3v_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_013408_rage_nethardware_getlocalipaddress_rage_netipaddress_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013429_char_ptr_rage_safecatf_char_char_ptr_unsigned_long_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_013479_rage_netconnectionmanager_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013620_rage_netipaddress_op_rage_netipaddress_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013669_rage_nettcpasyncop_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013672_rage_nettcpasyncop_dtornettcpasyncop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013676_rage_nettcp_close_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_013699_char_ptr_rage_formatf_char_2048ul_char_ref_2048ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013726_rage_nettcp_connectasync_rage_netsocketaddress_const_ref_rage_ne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_013727_rage_nettcpasyncop_pending_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013729_rage_nettcp_receive_int_void_ptr_unsigned_int_unsigned_int_ptr_i(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_013867_rage_nethardware_getmacaddress_unsigned_char_ref_6(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013871_rage_nettask_update(ph: *mut Phantom); }
extern "C" { pub fn f_013877_rage_netshutdown(ph: *mut Phantom); }
extern "C" { pub fn f_013882_rage_netipv4address_netipv4address(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013883_rage_netipv4address_netipv4address_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_013885_rage_netipaddress_format_char_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_013887_rage_netsocketaddress_netsocketaddress(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013888_rage_netsocketaddress_netsocketaddress_rage_netipaddress_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014013_rage_nettask_dtornettask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014015_rage_nettask_destroy_rage_nettask_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014016_rage_nettask_run_rage_nettask_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014017_rage_nettask_wascanceled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014020_rage_nettask_cancel_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014023_rage_nettask_nettask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014028_rage_nettask_allocate_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014039_rage_nettcp_sendbuffer_int_void_const_ptr_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014254_rage_rlshutdown(ph: *mut Phantom); }
extern "C" { pub fn f_014257_rage_is_ui_showing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014293_rage_rlsetlanguage_rage_syslanguage(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_014316_rage_rlgamerinfo_getgamerhandle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014318_rage_rlgamerinfo_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014319_rage_rlgamerhandle_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014320_rage_rlpc_isinitialized(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014370_rage_rlgamerhandle_tostring_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014374_rage_rlgamerhandle_export_void_ptr_unsigned_int_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014378_rage_rlgamerid_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014384_rage_rlpresence_issignedin_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014391_rage_rlgamerinfo_isonline_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014392_rage_rlgamerinfo_getgamerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014449_rage_rlpresence_adddelegate_rage_atdelegator1_void_rage_rlpresen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014609_rage_rlstats_cancel_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014662_rage_rlmetric_dtorrlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014685_rage_rlmetric_rlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014758_rage_rlpc_getsocialclubversion(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015893_rage_phmultireaderlocktoken_waitasreader(ph: *mut Phantom); }
extern "C" { pub fn f_016011_rage_phsimulator_setlastinstancematrix_rage_phinst_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016018_rage_phsimulator_activateobject_rage_phinst_ptr_rage_phcollider(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_016078_rage_phasyncshapetestmgr_kickasyncworker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016088_rage_phinst_setinstflag_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016256_rage_pharticulatedbody_getlinearvelocity_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016368_rage_phbound_gettypestring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016439_rage_phboundcapsule_setcapsulesize_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_016594_rage_phverletcloth_detachvirtualbound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016599_rage_phverletcloth_attachvirtualboundcapsule_float_float_rage_ma(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_016600_rage_phverletcloth_createvirtualbound_int_rage_mat34v_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016907_rage_phlevelnodetree_rage_phlooseoctreenode_getfirstculledobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016908_rage_phlevelnodetree_rage_phlooseoctreenode_getnextculledobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016943_rage_scrthread_printstacktrace_void_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016949_rage_scrthread_getactivethread(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_016981_rage_scrthread_getthread_rage_scrthreadid(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_017186_rage_crclip_composite_rage_crframe_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_017187_rage_crclip_initclass(ph: *mut Phantom); }
extern "C" { pub fn f_017189_rage_crclip_calcblockpassed_float_float_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i32; }
extern "C" { pub fn f_017190_rage_crclip_calcblockedtimeinternal_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_017261_rage_atarray_rage_vec3v_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018463_gmsgformat_parse_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018464_void_gmsgformat_formatd1_int_int_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018472_garraydatabase_gcolor_gallocatorgh_gcolor_2_garraydefaultpolicy(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018473_gimage_gimage_gimagebase_imageformat_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_018477_grefcountntsimpl_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018478_gstring_op_gstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018479_gutf8util_decodenextchar_char_const_ptr_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_018481_gmatrix2d_setidentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018482_gmatrix2d_prepend_gmatrix2d_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018483_gmatrix2d_append_gmatrix2d_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018484_gmatrix2d_transform_gpoint_float_ptr_gpoint_float_const_ref_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018486_gmatrix2d_setinverse_gmatrix2d_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018488_gmatrix2d_getxscale_const(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_018489_gmatrix2d_getyscale_const(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_018490_gmatrix2d_getrotation_const(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_018491_gmatrix2d_enclosetransform_grect_float_ptr_grect_float_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018494_gmatrix3d_gmatrix3d_gmatrix2d_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018496_gmatrix3d_setidentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018497_gmatrix3d_multiplymatrix_gmatrix3d_const_ref_gmatrix3d_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018511_glongformatter_convert(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018512_gutf8util_encodechar_char_ptr_long_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_018513_glongformatter_getresult_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018524_gmsgformat_gmsgformat_gmsgformat_sink_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018525_gmsgformat_dtorgmsgformat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018527_garraydatabase_gmsgformat_fmt_record_gallocatorgh_pod_gmsgformat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018529_gmsgformat_nextformatter(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_018530_gmsgformat_bind_gformatter_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018532_gstringbuffer_appendstring_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018534_gmsgformat_finishformatd(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018535_grefcountimpl_addref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018536_grefcountimpl_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018539_grefcountweaksupportimpl_createweakproxy_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018542_g_strtod_char_const_ptr_char_ptr_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f64; }
extern "C" { pub fn f_018543_g_towlower_wchar_t(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_018544_gstring_gstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018545_gstring_gstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018547_gstring_gstring_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_018550_gstring_getlength_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018552_gstring_appendchar_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_018553_gstring_appendstring_wchar_t_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018554_gstring_appendstring_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018556_gstring_op_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018557_gstring_op_gstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018559_gstring_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018561_gstring_comparenocase_char_const_ptr_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_018562_gstring_bernsteinhashfunction_void_const_ptr_unsigned_long_unsig(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018563_gstring_bernsteinhashfunctioncis_void_const_ptr_unsigned_long_un(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018564_gstringbuffer_appendchar_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_018565_gstringlh_gstringlh(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018566_gstringlh_copyconstructhelper_gstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018567_gstringdh_gstringdh_gmemoryheap_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018568_gstringdh_copyconstructhelper_gstring_const_ref_gmemoryheap_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018569_gstringbuffer_gstringbuffer_gmemoryheap_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018570_gstringbuffer_dtorgstringbuffer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018576_gtimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_018589_gutf8util_decodestring_wchar_t_ptr_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_018754_grasterizer_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018755_grasterizer_line_int_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_018758_grasterizer_addshapescaled_gcompoundshape_const_ref_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_018759_grasterizer_sortcells(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_018760_grasterizer_sweepscanline_unsigned_int_unsigned_char_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_018764_garraypagedbase_grectpacker_packtype_4_16_gallocatorpagedlh_pod(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018765_grenderer_cxform_cxform(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018766_grenderer_cxform_concatenate_grenderer_cxform_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018767_grenderer_cxform_transform_gcolor_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018830_grefcountimplcore_dtorgrefcountimplcore_31596(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018886_cgamelogic_setcurrentlevelindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_018887_cgamelogic_setrequestedlevelindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_018888_cgamelogic_isrunninggta5map(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_018889_cleveldata_getfriendlyname_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_018891_cgamelogic_getgamelogicbank(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_019027_cleveldata_getfilename_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_019088_cwaypointrecordingroute_draw_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019104_cpopgenshape_categorisepoint_rage_vector2_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019106_cpopgenshape_draw_bool_bool_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_019107_cpopgenshape_drawdebugwedge_rage_vector3_const_ref_float_float_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_019125_cvehiclerecordingstreamingmodule_cvehiclerecordingstreamingmodul(ph: *mut Phantom); }
extern "C" { pub fn f_019135_cvehiclerecordingmgr_stopplaybackrecordedcar_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019226_cslownesszonemanager_initwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019251_ctrafficlights_calculatenodelightcycle_rage_cpathnode_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019288_caitarget_caitarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019289_caitarget_caitarget_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019290_caitarget_setentity_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019293_caitarget_caitarget_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019295_caitarget_caitarget_caitarget_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019296_caitarget_getisvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019297_caitarget_getposition_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019305_cscenarioblockingareas_addscenarioblockingarea_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_019306_cscenarioblockingareas_removescenarioblockingarea_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019314_centityscanner_scanforentitiesinrange_centity_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019315_centityscanner_sortandstoreentities_rage_atfixedarray_centitysca(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019325_cspatialarray_findinsphere_rage_vec3v_const_ref_float_cspatialar(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_019387_cambientmodelsetmanager_getmodelsetindexfromhash_cambientmodelse(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_019418_cconditionalanims_getconditionalprobability_cscenariocondition_s(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_019421_cconditionalanims_checkclipset_rage_atarray_cconditionalclipset(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019424_cconditionalanimsgroup_checkformatchingconditionalanims_cscenari(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019450_char_ptr_rage_formatf_char_100ul_char_ref_100ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019462_cailogmanager_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019464_cailogmanager_printdebuginfosforpedtolog_cped_ref_cvehicle_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019565_cpedbonetagconvertor_getboneindexfrombonetag_rage_crskeletondata(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_019599_cfacialdatacomponent_resetfacialidleanimation_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019706_cclipeventtags_clookikeventtag_getturnspeed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019707_cclipeventtags_clookikeventtag_getblendinspeed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019708_cclipeventtags_clookikeventtag_getblendoutspeed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019709_cclipeventtags_clookikeventtag_geteyereferencedirection_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019710_cclipeventtags_clookikeventtag_getheadreferencedirection_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019711_cclipeventtags_clookikeventtag_getneckreferencedirection_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019712_cclipeventtags_clookikeventtag_gettorsoreferencedirection_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019713_cclipeventtags_clookikeventtag_getheadyawdeltalimit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019714_cclipeventtags_clookikeventtag_getheadpitchdeltalimit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019715_cclipeventtags_clookikeventtag_getneckyawdeltalimit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019716_cclipeventtags_clookikeventtag_getneckpitchdeltalimit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019717_cclipeventtags_clookikeventtag_gettorsoyawdeltalimit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019718_cclipeventtags_clookikeventtag_gettorsopitchdeltalimit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019719_cclipeventtags_clookikeventtag_getheadattitude_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019720_cclipeventtags_clookikeventtag_getleftarmcompensation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019721_cclipeventtags_clookikeventtag_getrightarmcompensation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019783_cdebugclipselector_getselectedclip_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019848_canimplacementeditor_isusingped_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019918_canimviewer_initmodelnamecombo(ph: *mut Phantom); }
extern "C" { pub fn f_019919_canimviewer_init(ph: *mut Phantom); }
extern "C" { pub fn f_019924_canimclipeditor_shutdownmainwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_019944_canimviewer_debugrender(ph: *mut Phantom); }
extern "C" { pub fn f_020056_canimviewer_selectped(ph: *mut Phantom); }
extern "C" { pub fn f_020448_canimscenemanager_update_animsceneupdatephase(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_020477_canimscenemanager_processdebugrender(ph: *mut Phantom); }
extern "C" { pub fn f_021185_audnorthaudioengine_finishupdate(ph: *mut Phantom); }
extern "C" { pub fn f_021204_audspeechaudioentity_say_unsigned_int_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32, a8: f32, a9: i32, a10: i64, a11: i64, a12: i64) -> i32; }
extern "C" { pub fn f_021209_audpolicescanner_reportplayercrime_rage_vector3_const_ref_ecrime(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_021222_audsuperconductor_sendconductormessage_conductors_conductormessa(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021227_audnorthaudioengine_mutecategory_audscene_ptr_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_021241_audnorthaudioengine_createwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_021351_cped_shouldbedead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021354_audspeechaudioentity_say_char_const_ptr_char_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i64) -> i32; }
extern "C" { pub fn f_021359_auddamagestats_dtorauddamagestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021366_audspeechaudioentity_inflictpain_auddamagestats_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021407_audspeechaudioentity_isambientspeechplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021414_audweaponaudiocomponent_playweaponfire_centity_ptr_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_021635_audradioaudioentity_skipstationsforward(ph: *mut Phantom); }
extern "C" { pub fn f_021761_audspeechaudioentity_saywhensafe_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_021801_audscriptaudioentity_abortscriptedconversation_bool_char_const_p(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_021927_audspeechaudioentity_doescontextexistforthisped_char_const_ptr_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_022082_audvehicleaudioentity_playvehiclehorn_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_022235_cpedinventory_getweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022381_void_creplayeventmanager_storeevent_cpacketsoundphonering_cpacke(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022397_naaudioentity_createdeferredsound_rage_audmetadataref_centity_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_022500_audcaraudioentity_canbedescribedasacar(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022658_cped_isdead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022701_audcuttrackcutscene_trigger(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022703_audcutsceneaudioentity_initcutscenetrack_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022707_audcutsceneaudioentity_iscutscenetrackprepared(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_022710_audcutsceneaudioentity_stopcutscene_bool_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_022717_audcutsceneaudioentity_getcutscenetime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_022723_audcuttrackcutscene_skiptotime_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022806_audambientaudioentity_registereffectaudio_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_022844_audfrontendaudioentity_startloadingtune(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022852_audfrontendaudioentity_playsound_rage_atnonfinalhashstring_char(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022986_audambientaudioentity_matchname_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_023396_cpacketevent_cpacketevent_ereplaypacketid_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023404_cpacketevent_store_ereplaypacketid_unsigned_int_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_023473_creplayoverlay_generatepacketnamelist_creplayinterfacemanager_re(ph: *mut Phantom); }
extern "C" { pub fn f_023565_quantizequaternions3_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023567_quantizequaternions3_20_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023579_cpacketstartscriptptfx_cpacketstartscriptptfx_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32) -> i64; }
extern "C" { pub fn f_023585_cpacketstopscriptptfx_cpacketstopscriptptfx(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023592_cpackettriggeredscriptptfx_cpackettriggeredscriptptfx_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_023702_replayfilemanager_getclipdirectory_char_ref_256_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023728_replayfilemanager_isperformingfileop(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023739_replayfilemanager_isloading_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023742_replayfilemanager_dumpreplayfile_char_const_ptr_char_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023849_creplaymgr_replayrecordsound_unsigned_int_unsigned_int_rage_auds(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_023870_creplaymgr_recordmapobject_cobject_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023877_creplaymgr_recordobject_cobject_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023888_replayattachmentmanager_pairings_dtorpairings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023891_replaybuffermarkermgr_addmarker_unsigned_int_unsigned_int_marker(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_023906_creplaycontrol_isplayeroutofcontrol(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023927_void_creplayeventmanager_trackrecordingevent_unsigned_long_ctrac(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_023930_creplayeventmanager_shouldproceedwithpacket_cpacketevent_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023967_creplayinterfacemanager_ensureisbeingrecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023968_creplayinterfacemanager_ensurewillberecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024252_creplaymgrinternal_validate_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_024266_creplaymgrinternal_isawareofworldforrecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024270_creplaymgrinternal_preprocess(ph: *mut Phantom); }
extern "C" { pub fn f_024279_creplaymgrinternal_postrender(ph: *mut Phantom); }
extern "C" { pub fn f_024286_creplaymgrinternal_isplaybackflagset_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_024304_creplaymgrinternal_getbuildinginfo_cphysical_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_024305_creplaymgrinternal_recordmapobject_centity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024354_replayhidemanager_addnewhide_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_024355_replayhidemanager_isnonreplaytrackedobjecttype_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026165_caminterface_isdominantrendereddirector_cambasedirector_const_re(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026169_cammanager_isrenderingfirstpersoncamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026174_camgameplaydirector_getfirstpersonshootercamera_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026182_camgameplaydirector_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026186_camfactory_createobject_cambaseobjectmetadata_const_ref_rage_atn(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026188_caminterface_getviewporttanfovh(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_026189_caminterface_getviewporttanfovv(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_026190_caminterface_fadeout_int_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026192_caminterface_fadeout_int_bool_rage_color32_const_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_026193_caminterface_isfadingout(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026194_caminterface_isfadedout(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026195_caminterface_fadein_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026196_caminterface_fadein_int_rage_color32_const_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_026199_caminterface_isfadedin(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026200_caminterface_isfading(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026202_caminterface_isspherevisibleingameviewport_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_026205_cqueriableinterface_istaskcurrentlyrunning_int_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_026227_rage_atarray_trenderedcameraobjectsettings_0_unsigned_short_dest(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026231_cambasedirector_cambasedirector_cambaseobjectmetadata_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026234_cambasedirector_dtorcambasedirector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026238_cambasecamera_shake_unsigned_int_float_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_026239_cambasecamera_shake_cambaseshakemetadata_const_ref_float_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_026253_camenvelope_start_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_026266_camgameplaydirector_isaiming_centity_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026272_camanimatedcamera_camanimatedcamera_cambaseobjectmetadata_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026273_cambasecamera_cambasecamera_cambaseobjectmetadata_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026274_camanimatedcamera_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026278_camanimatedcamera_updateframe_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_026302_camcinematicmountedcamera_createdampedlookathelpers_rage_atnonfi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026304_cambasecamera_dtorcambasecamera(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026321_camcontrolhelper_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026346_camcinematiccammancamera_camcinematiccammancamera_cambaseobjectm(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026373_rage_atvector_rage_fwregdref_centity_const_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026410_cambasecinematiccontext_cambasecinematiccontext_cambaseobjectmet(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026457_camcontrolhelper_getviewmodeforcontext_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_026499_camanimatedcamera_setcameraclip_rage_vector3_const_ref_rage_atno(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32) -> i32; }
extern "C" { pub fn f_026619_camanimatedcamera_dtorcamanimatedcamera_46011(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026626_rage_atvector_camcutscenedirector_scutscenecams_0_unsigned_short(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026631_camcutscenedirector_getactivecutscenecamera_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026634_camcutscenedirector_setdrawdistance_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_026635_camcutscenedirector_setusedaycoctrack_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026637_camcutscenedirector_iscamerawithidactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026638_camcutscenedirector_overridecameramatrixthisframe_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026639_camcutscenedirector_overridedofplanestrengthmodifier_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_026640_camcutscenedirector_overridecamerafovthisframe_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_026641_camcutscenedirector_getcamframe_camframe_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026642_camcutscenedirector_interpolateouttoendofscene_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_026647_camfreecamera_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026648_camdebugdirector_getcamframe_cambasecamera_ptr_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026649_camdebugdirector_getcamframenonconst_cambasecamera_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026703_camgameplaydirector_getfirstpersonvehiclecamera_centity_const_pt(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026709_camgameplaydirector_setrelativecameraheading_float_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_026710_camgameplaydirector_setrelativecamerapitch_float_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026719_cweaponinfo_const_ptr_cweaponinfomanager_getinfo_cweaponinfo_rag(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026722_camgameplaydirector_catchupfromframe_camframe_const_ref_float_in(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026726_cammanager_triggerfirstpersonflasheffect_cammanager_efirstperson(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_026740_camthirdpersoncamera_camthirdpersoncamera_cambaseobjectmetadata(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026741_camthirdpersoncamera_dtorcamthirdpersoncamera(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026800_camaimcamera_camaimcamera_cambaseobjectmetadata_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026810_camfirstpersonaimcamera_tryandcreateordestroystickyaimhelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026816_camstickyaimhelper_resetstickyaimparams_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_026824_camfirstpersonpedaimcamera_dtorcamfirstpersonpedaimcamera(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026863_camthirdpersonaimcamera_camthirdpersonaimcamera_cambaseobjectmet(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026865_camthirdpersonaimcamera_dtorcamthirdpersonaimcamera_46464(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026885_camthirdpersonpedaimcamera_camthirdpersonpedaimcamera_cambaseobj(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026957_camfollowcamera_camfollowcamera_cambaseobjectmetadata_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_027008_ctrain_getlinkedtobackward_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027039_cambaseframeshaker_cambaseframeshaker_cambaseobjectmetadata_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_027070_camframe_camframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027109_cammanager_abortpendingfirstpersonflasheffect_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027123_cammarketingfreecamera_cammarketingfreecamera_cambaseobjectmetad(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_027179_camreplaydirector_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027223_camscriptedcamera_camscriptedcamera_cambaseobjectmetadata_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_027257_camscriptdirector_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027280_cviewportmanager_setwidescreenborders_bool_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_027314_camswitchdirector_dtorcamswitchdirector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028228_cammanager_debugrender(ph: *mut Phantom); }
extern "C" { pub fn f_028245_cviewport_cviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028257_cviewportmanager_shutdownrenderphases(ph: *mut Phantom); }
extern "C" { pub fn f_028261_cviewportmanager_shutdownsession(ph: *mut Phantom); }
extern "C" { pub fn f_028262_cviewportmanager_pushviewport_cviewport_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_028263_cviewportmanager_popviewport(ph: *mut Phantom); }
extern "C" { pub fn f_028266_cviewportmanager_buildrenderlists(ph: *mut Phantom); }
extern "C" { pub fn f_028270_cviewportmanager_recacheviewports(ph: *mut Phantom); }
extern "C" { pub fn f_028329_cambasecinematiccontext_dtorcambasecinematiccontext_49116(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028363_camcinematicwatercrashcontext_dtorcamcinematicwatercrashcontext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028378_cambasecinematicshot_dtorcambasecinematicshot_49198(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028553_cambaseswitchhelper_dtorcambaseswitchhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028595_cambasesplinecamera_dtorcambasesplinecamera_49580(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029042_rage_sysmemdebug_printallocator_char_const_ptr_rage_sysmemalloca(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029047_rage_ragesecpluginmanager_destroyplugin_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_029048_rage_ragesecengine_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_029062_squish_computeprinciplecomponent_squish_sym3x3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029063_squish_getmultiplicity1evector_squish_sym3x3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_029064_squish_clusterfit_constructordering_squish_vec3_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_029065_squish_writecolourblock3_squish_vec3_const_ref_squish_vec3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_029066_squish_writecolourblock4_squish_vec3_const_ref_squish_vec3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_029067_squish_colourfit_compress3_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029068_squish_colourfit_compress4_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029069_squish_singlecolourfit_computeendpoints_squish_singlecolourlooku(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029070_squish_compressimage_unsigned_char_const_ptr_int_int_int_int_voi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32, a8: i64); }
extern "C" { pub fn f_029073_stb_compress_dxt_block_unsigned_char_ptr_unsigned_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_029074_stb_matchcolorsblock_unsigned_char_ptr_unsigned_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_029076_rage_copyfromblockalpha_rage_dxt_argb8888_const_ptr_rage_vector4(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_029077_rage_copyfromblockargb_rage_dxt_argb8888_const_ptr_rage_vector4(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_029079_rage_copytoblockalpha_rage_dxt_abgr8888_ptr_rage_vector4_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_029080_rage_copytoblockabgr_rage_dxt_abgr8888_ptr_rage_vector4_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_029088_rage_fwtextureconversion_resamplesource_rage_grcimage_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_029089_rage_edt_maureredt_int_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_029090_rage_fwtextureconversion_preprocessimage_rage_fwimage_t_rage_vec(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029091_rage_filtereddownsample_rage_fwimage_t_rage_vector4_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_029094_rage_computealphacoverage_rage_fwimage_t_rage_vector4_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i64) -> f32; }
extern "C" { pub fn f_029100_cgamesessionstatemachine_update(ph: *mut Phantom); }
extern "C" { pub fn f_029101_cdebug_setversionnumber(ph: *mut Phantom); }
extern "C" { pub fn f_029102_capp_checkexit(ph: *mut Phantom); }
extern "C" { pub fn f_029108_cbugstarassetlookup_init_char_const_ptr_char_const_ptr_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_029111_keepalivecallback(ph: *mut Phantom); }
extern "C" { pub fn f_029113_cplayercardxmldatamanager_miscdata_miscdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029114_processscuicloselogic(ph: *mut Phantom); }
extern "C" { pub fn f_029115_cgame_shutdownsession(ph: *mut Phantom); }
extern "C" { pub fn f_029116_cgame_shutdownlevelwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_029136_loaddisplayobjectslist(ph: *mut Phantom); }
extern "C" { pub fn f_029143_deletelegion(ph: *mut Phantom); }
extern "C" { pub fn f_029148_deleteobject(ph: *mut Phantom); }
extern "C" { pub fn f_029160_cnodeviewer_activate(ph: *mut Phantom); }
extern "C" { pub fn f_029191_tiledscreencapture_ctsclayer_addwidgets_rage_bkbank_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029192_tiledscreencapture_updateenabled(ph: *mut Phantom); }
extern "C" { pub fn f_029214_scenegeometrycapture_updatecamerafacing_cb(ph: *mut Phantom); }
extern "C" { pub fn f_029218_scenegeometrycapture_updatecombo_cb(ph: *mut Phantom); }
extern "C" { pub fn f_029222_scenegeometrycapture_updateselectedsphere_cb(ph: *mut Phantom); }
extern "C" { pub fn f_029224_scenegeometrycapture_updateselectedsphereindex_cb(ph: *mut Phantom); }
extern "C" { pub fn f_029254_cblockview_updateblocklist(ph: *mut Phantom); }
extern "C" { pub fn f_029256_cblockview_selectblock(ph: *mut Phantom); }
extern "C" { pub fn f_029263_cgame_initlevel_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_029264_cgame_initsession_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_029274_cnodeviewer_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_029311_cutscenemanager_shutdowncutscene(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029312_rage_atarray_notenodeset_0_unsigned_short_destruct_notenodeset_p(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_029313_anonymous_namespace_stopdebugflight(ph: *mut Phantom); }
extern "C" { pub fn f_029315_cnodeviewer_addnewnode(ph: *mut Phantom); }
extern "C" { pub fn f_029316_cnodeviewer_deletenode(ph: *mut Phantom); }
extern "C" { pub fn f_029317_cnodeviewer_resetnodeset(ph: *mut Phantom); }
extern "C" { pub fn f_029318_cnodeviewer_setnodetype(ph: *mut Phantom); }
extern "C" { pub fn f_029319_cnodeviewer_setnodetoground(ph: *mut Phantom); }
extern "C" { pub fn f_029320_cnodeviewer_setscalejaxxvisibility(ph: *mut Phantom); }
extern "C" { pub fn f_029321_cnodeviewer_setnodename(ph: *mut Phantom); }
extern "C" { pub fn f_029322_scenegeometrycapture_warptosphere_int_float_bool(ph: *mut Phantom, a0: i32, a1: f32, a2: i32); }
extern "C" { pub fn f_029323_scenegeometrycapture_capturegeometry_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_029324_cdumpgeometrytoobj_materialgroupend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029325_cdebugscene_focusentities_clear(ph: *mut Phantom); }
extern "C" { pub fn f_029326_cdebugscene_focusaddclosestaiped_cdebugscene_focusfilter_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029327_cuigadgetinspector_cuigadgetinspector_float_float_float_unsigned(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_029329_cvectormap_drawvehicle_cvehicle_const_ptr_rage_color32_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_029330_cvectormap_drawped_centity_const_ptr_rage_color32_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_029331_cdebugscene_drawentityboundingbox_centity_ptr_rage_color32_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_029333_cgamesessionstatemachine_processloadlevelstate_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_029334_cuigadgetbase_drawall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029336_cgamesessionstatemachine_updateinitialdialoguescreensonly(ph: *mut Phantom); }
extern "C" { pub fn f_029341_cgamesessionstatemachine_processrestartfade(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_029342_cgamesessionstatemachine_askbeforerestart(ph: *mut Phantom); }
extern "C" { pub fn f_029343_ccutsceneanimatedmodelentity_ccutsceneanimatedmodelentity_rage_c(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029344_ccutsceneanimatedmodelentity_dtorccutsceneanimatedmodelentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029345_ccutsceneanimatedmodelentity_deletenonregisteredentities(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029346_cutscenemanager_registergameentity_cdynamicentity_ptr_rage_atnon(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_029347_cutscenemanager_getanimatedmodelentityfromscenehandle_rage_atnon(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_029348_ccutsceneanimatedmodelentity_setvisibility_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029349_ccutsceneanimatedmodelentity_commondebugstr_rage_cutfobject_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029350_cutscenemanager_commondebugstr_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029351_ccutsceneanimatedmodelentity_createrepositiononlygameentitywhenm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029352_ccutsceneassetmgrentity_getmodelstreamingrequestid_int_rage_atno(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_029353_ccutsceneassetmgrentity_removestreamingrequest_int_bool_estreami(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_029354_ccutsceneassetmgrentity_removestreamingrequest_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_029355_ccutsceneassetmgrentity_releasestreamingrequests_ccutscenestream(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029356_ccutsceneanimatedmodelentity_createanimatedentitywhenmodelloaded(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029357_ccutsceneanimatedmodelentity_getcutscenetaskforentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029358_ccutsceneanimatedmodelentity_dispatchevent_rage_cutsmanager_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32, a6: i32); }
extern "C" { pub fn f_029359_ccutsceneanimatedmodelentity_requestrepositiononlyentitymodelwhe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029360_ccutsceneanimatedmodelentity_computescriptvisibletagsonanimset_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029361_ccutsceneanimatedmodelentity_findscriptvisibletags_rage_crclip_c(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_029362_ccutsceneanimatedmodelentity_computescriptvisibletagsonanimclear(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029363_cutscenemanager_getanimatedmodelentityfrommodelhash_rage_atnonfi(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029364_ccutsceneassetmgrentity_setmodelstreamingrequest_rage_atnonfinal(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029365_ccutsceneanimatedmodelentity_updatecutscenetaskphase_ctask_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029366_cutscenemanager_getphaseupdateamount_rage_crclip_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> f32; }
extern "C" { pub fn f_029367_ccutsceneanimatedmodelentity_removeentityfromtheworld_centity_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_029373_ccutsceneanimatedpropentity_setgameentityreadyforcutscene(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029376_ccutsceneanimatedmodelentity_createcutscenetask_cutscenemanager(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_029377_ctask_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029382_ccutsceneanimatedactorentity_storeactorvariationdata_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_029384_ccutsceneanimatedactorentity_setcurrentactorvariation_cped_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_029385_cpedvariationinfocollection_getmaxnumdrawbls_epedvarcomp_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_029386_cpedvariationinfocollection_getcollectiondrawable_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_029388_ccutsceneanimatedactorentity_prestreamvariations_cutscenemanager(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_029389_cutscenemanager_calculatestreamingoffset_float_float_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_029390_ccutsceneassetmgrentity_setactorcomponentvariationstreamingreque(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_029391_ccutsceneassetmgrentity_setactorpropvariationstreamingrequest_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_029395_rage_phinst_setmatrix_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029401_ccutsceneanimatedactorentity_dtorccutsceneanimatedactorentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029403_ccutsceneanimatedvehicleentity_dtorccutsceneanimatedvehicleentit(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029406_ccutsceneanimatedvehicleentity_setvehicledamage_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_029407_ccutsceneanimatedvehicleentity_setgamevehicleextra_rage_atarray(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029418_cgenericcliphelper_setrate_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029419_cgenericcliphelper_setphase_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_029420_ccutsceneanimation_ccutsceneanimation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029422_ccutsceneanimation_init_rage_crclip_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_029424_ccutsceneanimation_getphase_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_029425_ccutsceneanimation_getquaternionvalue_rage_crclip_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_029426_ccutsceneanimation_getvector3value_rage_crclip_const_ptr_int_flo(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_029427_ccutsceneanimation_getfloatvalue_rage_crclip_const_ptr_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_029428_ccutsceneanimmgrentity_dtorccutsceneanimmgrentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029433_getorcreategroup_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029446_ccutsceneassetmgrentity_removeallstreamingrequests(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029448_ccutsceneassetmgrentity_commondebugstr_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029449_ccutsceneassetmgrentity_shouldaddrequestrefforobject_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_029450_rage_atvector_ccutscenestreaminginfo_ptr_0_unsigned_short_pushan(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029451_ccutsceneassetmgrentity_removestreamingrequest_int_bool_estreami(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_029452_rage_fwassetstore_rage_pgdictionary_rage_grctexture_rage_fwtxdde(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_029453_cpedvariationinfocollection_getdlcnamefromdrawableidx_epedvarcom(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_029455_cdebugscene_tunewidgetdata_tunewidgetdata_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_029456_cdebugscene_queuetunewidget_cdebugscene_tunewidgetdata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029458_ccutsceneassetmgrentity_printstreamingrequests_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029463_ccutscenehiddenboundsentity_setobjectinareavisibility_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_029464_ccutscenehiddenboundsentity_computemodelindex(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029465_ccutscenehiddenboundsentity_commondebugstr_rage_cutfobject_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029469_cdebugscene_draw3dboundingbox_rage_vector3_const_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_029478_ccutscenecameraentity_dtorccutscenecameraentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029480_ccutscenecameraentity_updatefaceviewer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029481_ccutscenecameraentity_updateoverriddencamera_cutscenemanager_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029482_ccutscenecameraentity_shouldtriggerfirstpersoncameraflash_cutsce(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029483_ccutscenecameraentity_willexittofirstpersoncamera_cutscenemanage(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029485_ccutscenecameraentity_shouldusedaycoctrack_rage_cutsmanager_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029486_ccutscenecameraentity_commondebugstr_rage_cutfobject_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029487_ccutscenecameraentity_setcamerareadyforgame_rage_cutsmanager_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029506_ccutscenedebugmanager_dtorccutscenedebugmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029509_ccutscenedebugmanager_updatemousecursorposition_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_029510_ccutscenedebugmanager_updatecocmodifiereventoverridescb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029511_ccascadeshadowboundsdebug_updateshadowboxcombo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029512_cdebugscene_getworldpositionundermouse_rage_vector3_ref_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_029513_ccutscenedebugmanager_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029515_ccutscenedebugmanager_populateselectorlist_rage_bkcombo_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_029519_ccutscenedebugmanager_updatefirstpersoneventlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029520_ccutscenedebugmanager_populatecameracuteventlist_rage_cutfobject(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_029522_ccutscenedebugmanager_updatetimecycleparamscb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029524_ccutscenedebugmanager_populatevehiclevariationeventlist_rage_cut(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029525_ccutscenedebugmanager_populatevehicleextraeventlist_rage_cutfobj(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029526_ccutscenedebugmanager_populatevehicledamageeventlist_rage_cutfob(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029590_ccutscenedebugmanager_populatewaterrefractionindexeventlist_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029591_ccascadeshadowboundsdebug_populatesetshadowboundseventlist_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029592_cutscenemanager_savegamedatacutfile(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029593_ccutscenedebugmanager_savecocmodifier(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029594_ccutscenedebugmanager_populateinteriorloadeventlist_rage_cutfobj(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029595_ccutscenedebugmanager_updatecutscenevarcomponentcb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029596_ccutscenedebugmanager_updatecutscenevardrawblcb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029597_ccutscenedebugmanager_updatecutscenevartexcb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029598_ccutscenedebugmanager_populatepedvariationeventlist_rage_cutfobj(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029599_ccutscenedebugmanager_processcutfilesinmemory_rage_cutfcutscenef(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029607_cpedvariationinfocollection_getracetype_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_029608_cpedvariationinfocollection_ismatchcomponent_unsigned_int_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_029610_ccutscenedebugmanager_displayhiddenobject(ph: *mut Phantom); }
extern "C" { pub fn f_029611_cdebugscene_getmousepointing_rage_vector3_ref_rage_vector3_ref_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029612_worldprobe_cshapetestdesc_setresultsstructure_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029613_worldprobe_cshapetestresults_freeintersections(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029614_ccutscenedebugmanager_convertmatrixandvectorlocatetofourcornersa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_029615_ccutscenedebugmanager_populateoverlayeventlist_rage_cutfobject_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029616_ccutscenedebugmanager_saveoverlayevent_rage_cutfobject_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_029617_cutscenemanager_isdlccutscene(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029618_ccascadeshadowboundsdebug_bankdeletesetshadowboundseventcallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029619_rage_bktext_setstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029621_ccutscenedebugmanager_canaddeventfirstpersoncameraevent(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029624_ccutscenelight_dtorccutscenelight(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029625_ccutscenelight_getattachparentsboneindex_rage_cutsmanager_ptr_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_029626_ccutscenelight_updatelightdebugging_rage_cutsmanager_ptr_rage_ma(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_029627_cauthoringhelper_update_rage_matrix34_const_ref_float_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_029646_cutscenemanager_pushassetfolder_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029647_cutscenemanager_dtorcutscenemanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029649_cutscenemanager_setcutscenetogamestate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_029650_cutscenemanager_terminateloadedonlyscene(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029651_setplayercontrol_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_029652_cutscenemanager_addscriptresource_rage_scrthreadid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_029657_cutscenemanager_getplaytime_float(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_029661_cutscenemanager_validateevent_int_rage_fwflags_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_029662_cutscenemanager_updatestreaming_float_int_ref_rage_fwflags_int_f(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32, a4: f32); }
extern "C" { pub fn f_029665_cutscenemanager_createlightauthoringmat_rage_seditcutflightinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029667_cutscenemanager_dispatcheventtoobjectsoftype_int_int_rage_cutfev(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_029670_cutscenemanager_resetcutscenevars(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029675_cutscenemanager_triggercutsceneskip_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_029676_cutscenemanager_startmultiheadfade_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_029677_cutscenemanager_updateskip(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029678_cutscenemanager_setgametocutscenestate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029679_cutscenemanager_forcefade(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029680_cutscenemanager_startendcutscenebutton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029681_cutscenemanager_adjusttimeforblockingtags_float(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_029682_cutscenemanager_getmbframe(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029683_cutscenemanager_updatecameratolight_rage_seditcutflightinfo_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029685_cutscenemanager_playseamlesscutscene_rage_scrthreadid_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_029689_cutscenemanager_requestcutscene_char_const_ptr_bool_rage_cutsman(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_029690_cdebugtextureviewer_clear_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_029691_cdebugtextureviewerwindow_dtorcdebugtextureviewerwindow(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029699_ccutscenebinkoverlayentity_getrendertargetownertag_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029702_cutscenemanager_capturedrawable_centity_ptr_rage_rmcdrawable_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_029703_rage_atarray_rage_fwentity_ptr_0_unsigned_int_resizegrow_unsigne(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_029704_cutscenemanager_generatecapsule_rage_mat34v_const_ref_rage_phbou(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_029705_cutscenemanager_refreshobjlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029711_void_std_2_introsort_std_2_classicalgpolicy_alphabeticalsort_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_029726_camframe_setworldmatrix_rage_matrix34_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029729_cutscenemanager_setlighttocamera_rage_seditcutflightinfo_ptr_cam(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029735_camframe_clonefrom_camframe_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029736_cutscenemanager_addlightwithevents_rage_cutflightobject_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_029739_cutscenemanager_modifymaxlightobject_rage_cutflightobject_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029745_ccutscenescaleformoverlayentity_getrendertargetownertag(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029746_ccutscenebinkoverlayentity_linktorendertarget_unsigned_int_char(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_029750_ccutsceneanimatedparticleeffect_stopparticleeffect_rage_cutsmana(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029751_void_creplaymgrinternal_recordpersistantfx_cpacketstopscriptptfx(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029752_void_creplaymgrinternal_stoptrackingfx_unsigned_long_ctrackedeve(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029754_void_creplaymgrinternal_recordpersistantfx_cpacketstartscriptptf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029756_bool_creplaymgrinternal_recordfx_cpackettriggeredscriptptfx_cpac(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029759_ccutsceneparticleeffectsentity_hasparticleeffectloaded_rage_cuts(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_029761_ccutsceneparticleeffectsentity_commondebugstr_rage_cutfobject_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029769_ccutsceneaudioentity_commondebugstr_rage_cutfobject_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029772_rage_fwassetstore_rage_cutfcutscenefile2_cutfcutscenefile2def_dt(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029776_automatedtest_parseparameter_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029777_char_ptr_rage_safecat_char_char_4_char_ptr_char_const_ref_4_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029778_char_ptr_rage_formatf_char_70ul_char_ref_70ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029802_cmultislider_cmultislider_float_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i64; }
extern "C" { pub fn f_029804_cmultislider_addslider_rage_bkbank_ref_char_const_ptr_int_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_029809_cdebugscene_getentityundermouse_rage_vector3_ptr_int_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_029829_csavedbugstarrequestlist_get_char_const_ptr_char_const_ptr_int_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_029830_cbugstarassertupdatequery_dtorcbugstarassertupdatequery(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029831_cbugstarassertupdatequery_createbugdescription_char_ptr_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029832_cbugstarassertupdatequery_addbug(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029833_cbugstarassertupdatequery_getsubsequentbugs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029834_cbugstarintegration_createxmltreefromhttpquery_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_029835_cbugstarintegration_createxmltreefromhttpquery_char_const_ptr_ch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029836_cbugstarintegration_postbugchanges_int_rage_partree_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_029837_cbugstarintegration_converttominimalbug_rage_partree_ptr_cminima(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029838_cbugstarintegration_performfulltextsearch_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_029850_cbugstarintegration_updatefilteredbuglist(ph: *mut Phantom); }
extern "C" { pub fn f_029851_cbugstarintegration_passesstatusfilter_cminimalbug_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029853_char_ptr_rage_formatf_char_260ul_char_ref_260ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_029858_colorizerutils_getdrawableinfo_rage_rmcdrawable_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_029859_colorizerutils_getfraginfo_centity_ptr_cbasemodelinfo_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32); }
extern "C" { pub fn f_029860_colorizerutils_getpolycount_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029866_cdebugscene_focusentities_get_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029869_anonymous_namespace_restoredebugflightphysics(ph: *mut Phantom); }
extern "C" { pub fn f_029870_cpedintelligence_addtasksecondary_rage_aitask_ptr_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029874_displayobject_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_029876_debuglocation_beammeup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029877_debuglocation_setfromcurrent(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029883_savedisplayobjectlist(ph: *mut Phantom); }
extern "C" { pub fn f_029887_debuglocationlist_load(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029890_cdebugscene_focusentities_set_centity_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_029896_cdebug_pickobjecttospawn(ph: *mut Phantom); }
extern "C" { pub fn f_029900_cdebugarchetype_createdebugarchetypeproxies(ph: *mut Phantom); }
extern "C" { pub fn f_029902_cdebugarchetypeproxy_create_cbasemodelinfo_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_029923_cdebugdrawstore_getstorage_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_029924_cdebugdrawstore_addline_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_029934_cdebugdrawstore_render(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029937_geometryutil_addtrianglesfordrawable_rage_rmcdrawable_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_029938_cdumpgeometrytoobj_addtriangle_rage_vec3v_const_ref_rage_vec3v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_029939_cdumpgeometrytoobj_addquad_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_029940_cdumpgeometrytoobj_addcylinder_rage_vec3v_const_ref_rage_vec3v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_029941_cdumpgeometrytoobj_addboxedges_rage_vec3v_const_ref_rage_vec3v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_029942_cdumpgeometrytoobj_addtext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029950_cgtapickerinterface_detachinspectorchild_cuigadgetinspector_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029951_cuigadgettext_setstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029952_debugnodesys_init(ph: *mut Phantom); }
extern "C" { pub fn f_029963_rage_atarray_rage_datcallback_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029964_rage_fwtransform_geta_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029965_rage_fwtransform_getb_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029966_rage_fwtransform_getc_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029967_cphysical_gethealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029969_cdebugscene_focusentities_isingroup_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029972_cdebugscene_drawboundcollisiongeometry_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029981_cauthoringhelper_createboxmatminandmaxvectors_rage_matrix34_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_029982_cauthoringhelper_updatetranslationaxis_rage_matrix34_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_029984_cuigadgetbase_containsmouse(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029985_cuigadgetsimplelistandwindow_cuigadgetsimplelistandwindow_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: i32) -> i64; }
extern "C" { pub fn f_029986_cuigadgetlist_cuigadgetlist_float_float_float_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i64) -> i64; }
extern "C" { pub fn f_029987_cuigadgetwindow_cuigadgetwindow_float_float_float_float_cuicolor(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64) -> i64; }
extern "C" { pub fn f_029988_cuigadgetlist_setnumentries_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_029991_cvoxelanalysis_findregions_recurse_func_int_int_int_unsigned_cha(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_030049_cgtapickerinterface_showuigadgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030050_cuigadgetbase_applymovement_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030052_cuigadgetbase_attachchild_cuigadgetbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030064_char_const_ptr_assetanalysis_getrpfpath_t_rage_fwmaptypesstore_r(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_030065_getshadersusedbyentity_rage_atarray_rage_grmshader_const_ptr_0_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_030066_getshadersusedbydwd_rage_atarray_rage_grmshader_const_ptr_0_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_030067_assetanalysis_getassetpath_rage_fwmapdatastore_ref_int_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030068_assetanalysis_getdrawablepath_cbasemodelinfo_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030069_assetanalysis_getassetpath_rage_fwfragmentstore_ref_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_030070_assetanalysis_getassetpath_rage_fwdrawablestore_ref_int_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030071_assetanalysis_getassetpath_rage_fwdwdstore_ref_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_030082_setcamera_internal_debugtextureviewerdtq_edtqcubeface(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_030083_exportscenecubemap_rage_grcimage_const_ptr_rage_grcimage_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_030085_scaleimagebrightness_rage_grcimage_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_030087_cmarketingtools_widgetaddlight(ph: *mut Phantom); }
extern "C" { pub fn f_030088_cmarketingtools_widgetrefreshlightwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_030096_cnodeviewer_setallnodesempty(ph: *mut Phantom); }
extern "C" { pub fn f_030101_cnodeviewer_setclosestnode(ph: *mut Phantom); }
extern "C" { pub fn f_030102_cnodeviewer_setcameratoclosestnode(ph: *mut Phantom); }
extern "C" { pub fn f_030106_cnodeviewer_savefile_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_030107_cnodeviewer_deletescalejaxx(ph: *mut Phantom); }
extern "C" { pub fn f_030108_cnodeviewer_createscalejaxx(ph: *mut Phantom); }
extern "C" { pub fn f_030121_coverviewscreenmemory_formatmemoryusedfree_char_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_030122_coverviewscreenmemory_formatmemoryusedfree_char_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_030124_coverviewscreenmanager_writetofile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030125_calculatedependencycost_rage_strlocalindex_int_rage_atarray_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_030143_scenegeometrycapture_makenameunique_char_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030145_tiledscreencapture_updatelayer_tiledscreencapture_ctsclayer_cons(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030152_tiledscreencapture_setstate_int_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_030153_tiledscreencapture_tilegetfilename_int_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_030159_tiledscreencapture_capturetile_char_const_ptr_rage_grcrendertarg(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_030162_assetanalysis_beginwritingstreams_private(ph: *mut Phantom); }
extern "C" { pub fn f_030163_assetanalysis_writestring_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030164_assetanalysis_getassetpath_rage_fwtxdstore_ref_int_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030165_assetanalysis_getassetpath_rage_ptfxassetstore_ref_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_030166_assetanalysis_getstream_assetanalysis_estreamtype_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_030167_assetanalysis_processtxdslot_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_030168_assetanalysis_writetologfile_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030169_assetanalysis_writestring64unique_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030170_assetanalysis_writestring64append_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030171_assetanalysis_writetexture_char_const_ptr_rage_grctexture_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030172_unsigned_long_assetanalysis_writematerialtexture_rage_fwdwdstore(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_030173_assetanalysis_gettexturepath_rage_fwdwdstore_ref_int_int_rage_rm(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_030174_unsigned_long_assetanalysis_writematerialtexture_rage_fwdrawable(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_030175_assetanalysis_gettexturepath_rage_fwdrawablestore_ref_int_int_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_030176_unsigned_long_assetanalysis_writematerialtexture_rage_fwfragment(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_030177_void_assetanalysis_writedrawable_rage_fwfragmentstore_rage_fwfra(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_030178_void_assetanalysis_writetexturedictionary_rage_ptfxassetstore_ra(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_030179_unsigned_long_assetanalysis_writematerialtexture_rage_ptfxassets(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_030180_assetanalysis_gettexturepath_rage_ptfxassetstore_ref_int_int_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_030181_assetanalysis_writearchetypeinstance_centity_const_ptr_cbasemode(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_030182_assetanalysis_gettexturename_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030183_assetanalysis_gettexturepath_rage_fwfragmentstore_ref_int_int_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_030193_getsimpletimestring(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030194_cstreamingiterator_rage_fwtxdstore_rage_pgdictionary_rage_grctex(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030195_streamingiteratorfilter_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_030196_streamingiteratortesttexturesize_rage_grctexture_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030197_streamingiteratortesttexturemipsize_rage_grctexture_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030198_streamingiteratortesttexturenonpow2_rage_grctexture_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030199_streamingiteratortesttexturepaths_rage_grctexture_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030200_streamingiteratortestdrawable_init(ph: *mut Phantom); }
extern "C" { pub fn f_030201_streamingiteratortestdrawableorientation_rage_rmcdrawable_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030202_streamingiteratortestdrawablelods_rage_rmcdrawable_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030203_streamingiteratortestdrawablecableshadervars_rage_rmcdrawable_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030204_streamingiteratortestdrawablecablenormals_rage_rmcdrawable_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030205_streamingiteratortestdrawablehdwithsubmeshes_rage_rmcdrawable_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030206_streamingiteratortestdrawablemultiplebuckets_rage_rmcdrawable_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030207_streamingiteratortestmateriallowfresnel_rage_grmshader_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030208_streamingiteratortestmaterialnodiffusetex_rage_grmshader_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030209_void_streamingiteratortestdrawable_t_rage_fwfragmentstore_rage_f(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_030212_rage_atarray_cstreamingiteratorslot_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030215_rage_atarray_geomcollector_cgcdrawable_0_unsigned_short_destruct(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030216_geomcollector_cgcdrawablegroup_processdrawableinternal_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_030217_geomcollector_cgcdrawablegroup_processdrawableinternal_char_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030218_rage_atarray_geomcollector_cgcgeometry_0_unsigned_short_grow_uns(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030219_rage_atarray_geomcollector_cgcgeometry_0_unsigned_short_op_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030220_geomcollector_cgcdrawablegroup_addentityinternal_cbasemodelinfo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i64); }
extern "C" { pub fn f_030221_geomcollector_init(ph: *mut Phantom); }
extern "C" { pub fn f_030222_void_geomcollector_processdrawable_t_rage_fwfragmentstore_rage_f(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_030224_rage_atarray_geomcollector_cpropgroup_0_unsigned_short_destruct(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030225_rage_atarray_geomcollector_cpropgroup_0_unsigned_short_pushandgr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030226_rage_atarray_geomcollector_cpropgroupentry_0_unsigned_short_grow(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030227_rage_atarray_geomcollector_cpropgroupentry_0_unsigned_short_copy(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030246_cmissionlookup_missiondef_ref_rage_atmap_rage_athashvalue_cmissi(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030255_void_rage_atpagedpool_forall_clightentity_rage_fwdefaultrscmempa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_030256_debuglights_setcurrentlightentity_clightentity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030260_debuglights_addlighttoguidmap_clightentity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030272_cmapliveedit_start_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030273_cmapliveedit_rebuildguidmap(ph: *mut Phantom); }
extern "C" { pub fn f_030274_rage_atarray_centityeditset_0_unsigned_short_destruct_centityedi(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030288_centityeditset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_030306_cmapliveeditutils_recursivelyaddchildren_int_rage_atarray_int_0(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_030307_rage_atarray_centityeditset_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030308_centityeditset_op_centityeditset_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030309_rage_atarray_centityadd_0_unsigned_short_op_rage_atarray_centity(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030343_cmapeditrestinterface_dtorcmapeditrestinterface(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030352_editlightsource_change_rage_fwregdref_clightentity_rage_fwrefawa(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030353_editlightsource_update_clightentity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030357_debugdeferred_addwidgets_gbuffer_rage_bkbank_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030358_debugrendering_setshaderparams(ph: *mut Phantom); }
extern "C" { pub fn f_030359_debugrendering_rendercostlegend_bool_int_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_030362_debugdeferred_overridegbuffertarget_egbufferoverride_rage_vector(ph: *mut Phantom, a0: i32, a1: i64, a2: f32); }
extern "C" { pub fn f_030377_editlightsource_applychanges_clightattr_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030386_debuglights_initwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_030399_debuglighting_dumpparentimaps_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_030415_cuicolorscheme_setallcolors_rage_color32_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030416_cuigadgetbase_propagateevent_cuievent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030419_cuigadgetbase_updateall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030421_cuigadgetbase_detachfromparent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030435_rage_atarray_cuigadgettext_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030436_rage_atarray_cuigadgetbox_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030437_cuigadgetinspector_dtorcuigadgetinspector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030443_cuigadgetlist_dtorcuigadgetlist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030453_cuigadgetwindowtitlebar_dtorcuigadgetwindowtitlebar(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030455_cuigadgetwindow_dtorcuigadgetwindow(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030461_cdebugtextureviewer_dtorcdebugtextureviewer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030462_cdtvstate_dtorcdtvstate(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030464_cdebugtextureviewer_populate_selectmodelinfo_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_030465_cdebugtextureviewer_setupwindow_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030466_getassociatedtxds_modelinfo_rage_atarray_ctxdref_0_unsigned_shor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_030467_ctxdref_gettxd_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030468_istextureusedbymodel_rage_grctexture_const_ptr_char_const_ptr_cb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_030469_cdtventry_texture_cdtventry_texture_ctxdref_const_ref_cassetsear(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_030470_cdebugtextureviewer_setupwindowfinish(ph: *mut Phantom); }
extern "C" { pub fn f_030472_ctxdref_loadtxd_bool_bool_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_030473_cdebugtextureviewer_populate_selecttxd_char_const_ptr_ctxdref_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030474_cassetref_addref_cassetrefinterface_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030475_cassetref_getdesc_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_030476_cassetref_getassetindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030477_cassetref_isvalidassetslot_eassettype_rage_strlocalindex(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_030478_cassetref_getstreamingassetsize_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_030479_rage_atarray_cassetref_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030481_cassetref_getrpfpathname_eassettype_rage_strlocalindex(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_030482_stringmatch_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_030484_cdtventry_modelinfo_cdtventry_modelinfo_char_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_030493_cdtventry_setupwindow_cdebugtextureviewerwindow_ptr_cdtvcolumnfl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030495_cassetref_getassetname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030496_rage_atarray_ctxdref_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030497_findtextures_buildshaderusagemapall_rage_atmap_rage_grctexture_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030498_findtextures_checksizeandflags_rage_grctexture_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_030499_gettexturehash_rage_grctexture_const_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_030500_rage_atarray_ctextureref_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030501_getassociatedtxds_modelinfo_push_rage_atarray_ctxdref_0_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_030502_bool_istextureusedbydrawable_rage_fwdwdstore_rage_fwdwdstore_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_030505_cdtvstate_cdtvstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030506_cassetref_removerefwithoutdelete_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030508_cdebugtextureviewer_edit_metadatainworkbench(ph: *mut Phantom); }
extern "C" { pub fn f_030509_cassetref_getrpfname_eassettype_rage_strlocalindex(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_030513_rage_fwtextureconversionparams_dtorfwtextureconversionparams(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030516_textureviewer_getrenderbucketinfostrings_rage_rmcdrawable_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_030528_getfriendlytexturename_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030530_cassetref_getstring_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_030533_cdtvstate_dumptexture_rage_grctexture_const_ptr_rage_grctexture(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_030542_loadasset_addref_eassettype_int_cassetrefinterface_const_ref(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_030547_cassetref_getparenttxdindex_eassettype_rage_strlocalindex(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_030548_findassets_currentdependentmodelinfos_rage_atarray_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030549_findassets_modelinfo_rage_atarray_unsigned_int_0_unsigned_short(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030550_getmodelinfodesc_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_030557_gettexturedictionarysizeinbytes_rage_pgdictionary_rage_grctextur(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_030558_gettexturesizeinbytes_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030566_findtextures_buildshaderusagemap_rage_atmap_rage_grctexture_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_030600_cuigadgetbutton_dtorcuigadgetbutton(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030603_cdtventry_modelinfo_dtorcdtventry_modelinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030608_cdtventry_texture_dtorcdtventry_texture_52825(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030641_priorityoperation_setpriority(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030675_csettingsmanager_dtorcsettingsmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030682_initcarscore(ph: *mut Phantom); }
extern "C" { pub fn f_030691_initpedscore(ph: *mut Phantom); }
extern "C" { pub fn f_030707_capturescenecubemapface_internal_rage_grcimage_ptr_ref_rage_grci(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_030711_rage_atmap_unsigned_long_bool_rage_atmaphashfn_unsigned_long_rag(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030712_assetanalysis_writestring32unique_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030714_rage_atarray_geomcollector_cgcdrawablegroup_0_unsigned_short_gro(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030715_rage_atarray_geomcollector_cgcdrawable_0_unsigned_short_copyfrom(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030718_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_alphabetic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_030719_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_030720_rage_atpagedpooliterator_clightentity_rage_fwdefaultrscmempagedp(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030721_rage_atpagedpoolsubtypeiterator_clightentity_rage_fwdefaultrscme(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030722_ceventgroupped_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030724_cevent_cancreateevent_unsigned_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_030725_rage_fweventqueue_16_void_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030731_ceventgroupped_haspaireddamageevent_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030733_rage_fweventqueue_4_void_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030735_rage_fweventqueue_32_void_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030737_rage_fweventqueue_32_void_remove_rage_fwevent_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030738_rage_fweventqueue_32_clockingeventgroupbase_remove_rage_fwevent(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030739_cpedintelligence_addevent_cevent_const_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030744_ceventnetwork_spewtotty_impl_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030745_ceventgroupscript_500_flushallandmovepending(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030748_rage_fweventqueue_500_void_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030752_ceventhandlerhistory_cleartempevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030753_ceventhandlerhistory_clearnontempevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030754_ceventhandlerhistory_clearmovementevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030755_ceventhandlerhistory_clearstoredevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030759_ceventresponse_cleareventresponse_ceventresponse_responsetasktyp(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030760_cpedintelligence_getactivemovementtask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030761_cpedintelligence_gettaskactive_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030762_rage_fweventqueue_16_void_remove_rage_fwevent_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030765_ceventnames_addeventnames(ph: *mut Phantom); }
extern "C" { pub fn f_030768_ceventresponsefactory_createeventresponse_cped_ptr_cevent_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_030769_caitarget_dtorcaitarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030770_cshockingeventsmanager_add_ceventshocking_ref_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_030771_ceventshocking_initclone_ceventshocking_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_030772_ceventshocking_dtorceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030780_ceventdecisionmakermanager_init(ph: *mut Phantom); }
extern "C" { pub fn f_030781_ceventdecisionmakermanager_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_030785_ceventdataresponse_decision_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_030790_ceventeditableresponse_findtasktypeforresponse_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_030795_ceventdecisionmakerresponse_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_030799_ceventdecisionmakerresponse_makedecision_cped_const_ptr_cevented(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_030803_cevent_initializedelay_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030810_cevent_communicateevent_cped_ptr_cevent_communicateeventinput_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_030811_cevent_caneventbeignoredbecauseofinteriorstatus_cped_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_030816_cped_sdamagerecord_sdamagerecord_centity_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: f32, a7: f32, a8: i32, a9: i32, a10: i32, a11: i32) -> i64; }
extern "C" { pub fn f_030817_cpeddamagecalculator_updatedamagetrackers_cped_ptr_cpeddamageres(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_030818_cped_setarmour_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_030819_cped_isinjured_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030820_ceventrequesthelp_ceventrequesthelp_cped_ptr_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_030821_ceventrequesthelp_communicateevent_cped_ptr_cevent_communicateev(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_030822_ceventrequesthelp_dtorceventrequesthelp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030823_cpeddamagecalculator_registerkill_cped_const_ptr_centity_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64); }
extern "C" { pub fn f_030824_cpedvariationinfocollection_hascomponentflagsset_epedvarcomp_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_030825_cped_getarmour_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_030826_cpedintelligence_gettargetting_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030829_ceventdamage_dtorceventdamage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030842_ceventranoverped_ceventranoverped_cvehicle_ptr_cped_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_030843_ceventranoverped_dtorceventranoverped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030845_ceventagitated_ceventagitated_cped_ptr_agitatedtype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_030846_ceventagitated_dtorceventagitated(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030847_ceventshockingpedknockedintobyplayer_ceventshockingpedknockedint(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_030851_ceventdeath_dtorceventdeath(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030855_ceventwrithe_dtorceventwrithe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030858_ceventhurttransition_ceventhurttransition_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030859_ceventhurttransition_dtorceventhurttransition(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030881_ceventclimbnavmeshonroute_ceventclimbnavmeshonroute_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: f32, a5: i32, a6: i64, a7: i32, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_030882_ceventclimbnavmeshonroute_dtorceventclimbnavmeshonroute(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030886_ceventpotentialbewalkedinto_ceventpotentialbewalkedinto_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_030887_ceventpotentialbewalkedinto_dtorceventpotentialbewalkedinto(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030893_ceventpotentialgetrunover_ceventpotentialgetrunover_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030894_ceventpotentialgetrunover_dtorceventpotentialgetrunover(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030906_cpedintelligence_findtaskprimarybytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_030907_ceventpotentialwalkintovehicle_ceventpotentialwalkintovehicle_cv(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64) -> i64; }
extern "C" { pub fn f_030908_ceventpotentialwalkintovehicle_dtorceventpotentialwalkintovehicl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030938_ceventreactionenemyped_ceventreactionenemyped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_030945_svehiclemissionparams_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030949_ceventdisturbance_ceventdisturbance_cped_ptr_cped_ptr_ceventdist(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_030973_cinformfriendsevent_dtorcinformfriendsevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030974_cinformfriendsevent_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030977_cinformgroupevent_dtorcinformgroupevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030978_cinformgroupevent_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030986_ceventopendoor_ceventopendoor_cdoor_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030987_ceventopendoor_dtorceventopendoor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030990_ceventshoveped_ceventshoveped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030991_ceventshoveped_dtorceventshoveped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030995_ceventvehiclecollision_ceventvehiclecollision_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32, a6: f32) -> i64; }
extern "C" { pub fn f_030996_ceventvehiclecollision_dtorceventvehiclecollision(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031002_ceventpedcollisionwithped_ceventpedcollisionwithped_unsigned_sho(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i64, a5: i64, a6: f32, a7: f32) -> i64; }
extern "C" { pub fn f_031004_ceventpedcollisionwithped_dtorceventpedcollisionwithped_53479(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031007_ceventpedcollisionwithped_affectspedcore_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_031011_ceventobjectcollision_ceventobjectcollision_unsigned_short_cobje(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: f32) -> i64; }
extern "C" { pub fn f_031012_ceventobjectcollision_dtorceventobjectcollision(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031015_ceventobjectcollision_getroutetarget_cped_const_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031017_ceventdraggedoutcar_ceventdraggedoutcar_cvehicle_const_ptr_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_031018_ceventdraggedoutcar_dtorceventdraggedoutcar(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031023_ceventexplosion_ceventexplosion_rage_vec3v_const_ref_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_031024_ceventexplosion_dtorceventexplosion(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031030_ceventpedjackingmyvehicle_ceventpedjackingmyvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_031031_ceventpedjackingmyvehicle_dtorceventpedjackingmyvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031035_ceventpedenteredmyvehicle_ceventpedenteredmyvehicle_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031036_ceventpedenteredmyvehicle_dtorceventpedenteredmyvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031040_ceventacquaintanceped_ceventacquaintanceped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031042_ceventacquaintanceped_affectspedcore_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_031051_ceventencroachingped_ceventencroachingped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031052_ceventencroachingped_dtorceventencroachingped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031055_ceventscriptcommand_ceventscriptcommand_int_rage_aitask_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_031056_ceventscriptcommand_dtorceventscriptcommand(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031069_ceventgivepedtask_ceventgivepedtask_int_rage_aitask_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_031070_ceventgivepedtask_dtorceventgivepedtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031086_ceventinair_ceventinair_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031087_ceventinair_dtorceventinair(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031091_ceventmustleaveboat_dtorceventmustleaveboat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031098_ceventcallforcover_dtorceventcallforcover(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031101_ceventprovidingcover_dtorceventprovidingcover(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031104_ceventrequesthelp_affectspedcore_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_031110_ceventcrimecryforhelp_ceventcrimecryforhelp_cped_const_ptr_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031111_ceventcrimecryforhelp_dtorceventcrimecryforhelp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031120_ceventhelpambientfriend_ceventhelpambientfriend_cped_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_031126_ceventshoutblockinglos_dtorceventshoutblockinglos(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031129_ceventvehicleonfire_ceventvehicleonfire_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031130_ceventvehicleonfire_dtorceventvehicleonfire(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031136_ceventstuckinair_ceventstuckinair_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031137_ceventstuckinair_dtorceventstuckinair(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031143_ceventcopcarbeingstolen_ceventcopcarbeingstolen_cped_ptr_cvehicl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031144_ceventcopcarbeingstolen_dtorceventcopcarbeingstolen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031148_ceventvehicledamage_ceventvehicledamage_cvehicle_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_031153_ceventcarundriveable_ceventcarundriveable_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031154_ceventcarundriveable_dtorceventcarundriveable(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031160_ceventpedoncarroof_ceventpedoncarroof_cped_ptr_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031161_ceventpedoncarroof_dtorceventpedoncarroof(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031167_ceventstaticcountreachedmax_testcapsule_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_031169_ceventswitch2nm_ceventswitch2nm_unsigned_int_rage_aitask_ptr_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_031170_ceventswitch2nm_dtorceventswitch2nm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031181_ceventsuspiciousactivity_ceventsuspiciousactivity_cped_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_031182_ceventsuspiciousactivity_dtorceventsuspiciousactivity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031186_ctaskinvestigate_setinvestigationpos_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_031187_ceventunidentifiedped_ceventunidentifiedped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031188_ceventunidentifiedped_dtorceventunidentifiedped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031191_ceventdeadpedfound_ceventdeadpedfound_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_031200_ceventagitatedaction_ceventagitatedaction_cped_const_ptr_rage_ai(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031201_ceventagitatedaction_dtorceventagitatedaction(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031206_ceventscenarioforceaction_dtorceventscenarioforceaction(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031210_ceventcombattaunt_ceventcombattaunt_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031211_ceventcombattaunt_dtorceventcombattaunt(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031213_ceventcombattaunt_shouldpedignoreevent_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_031218_ceventrespondedtothreat_ceventrespondedtothreat_cped_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031219_ceventrespondedtothreat_dtorceventrespondedtothreat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031224_ceventplayerdeath_ceventplayerdeath_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031226_cevententitydamaged_cevententitydamaged_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031228_cevententitydestroyed_cevententitydestroyed_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031240_ceventshocking_affectspedcore_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_031241_ceventshocking_canbesensedby_cped_const_ref_float_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_031353_ceventsoundbase_ceventsoundbase_centity_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_031357_ceventcommunicateevent_dtorceventcommunicateevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031378_ceventgunaimedat_ceventgunaimedat_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031380_ceventgunaimedat_dtorceventgunaimedat_53949(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031388_ceventgunshot_getdriveroffiringvehicleifleaderislocalplayer_cveh(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031389_ceventgunshot_ceventgunshot_centity_ptr_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32) -> i64; }
extern "C" { pub fn f_031391_ceventgunshot_dtorceventgunshot_53962(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031393_ceventgunshot_iseventready_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_031400_svehiclemissionparams_settargetentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_031406_ceventgunshotwhizzedby_isbulletinrange_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_031415_ceventmeleeaction_ceventmeleeaction_centity_ptr_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_031416_ceventmeleeaction_dtorceventmeleeaction(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031430_ceventfriendlyfirenearmiss_isbulletinrange_cped_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_031432_ceventdatadecisionmaker_postload(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_031435_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_031565_ceventnetworkentitydamage_dtorceventnetworkentitydamage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031582_ceventnetworkvehicleundrivable_dtorceventnetworkvehicleundrivabl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031587_rage_fweventqueue_32_void_dtorfweventqueue(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031593_rage_fweventqueue_500_void_dtorfweventqueue(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031606_ceventacquaintanceped_dtorceventacquaintanceped_54301(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031617_ceventhelpambientfriend_dtorceventhelpambientfriend(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031622_ceventvehicledamage_dtorceventvehicledamage_54333(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031627_ceventdeadpedfound_dtorceventdeadpedfound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031630_ceventplayerdeath_dtorceventplayerdeath(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031635_ceventreactionenemyped_dtorceventreactionenemyped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031638_ceventreactioninvestigatethreat_dtorceventreactioninvestigatethr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031643_ceventreactioninvestigatedeadped_dtorceventreactioninvestigatede(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031646_ceventreactioninvestigatedeadped_ceventreactioninvestigatedeadpe(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031651_ceventdisturbance_dtorceventdisturbance(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031662_rage_fweventqueue_16_void_dtorfweventqueue(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031667_rage_atqueue_rage_eventregdref_rage_fwevent_4_dtoratqueue(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031671_rage_fweventqueue_4_void_getlowestpriorityeventthatcanberemoved(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031689_ceventpotentialblast_dtorceventpotentialblast(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031706_ceventpedseendeadped_dtorceventpedseendeadped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031880_ceventsoundbase_dtorceventsoundbase_54685(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031888_ceventgunshotbulletimpact_dtorceventgunshotbulletimpact(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032070_rage_fweventqueue_16_void_flushexpiredevents(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032073_rage_fweventqueue_4_void_flushexpiredevents(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032146_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_032147_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_032149_gasprototypebase_setconstructor_gasobject_ptr_gasstringcontext_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_032151_gfxampviewstats_addmarker_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032156_gasampmarkerctorfunction_setmember_gasenvironment_ptr_gasstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032157_gasobject_setmember_gasenvironment_ptr_gasstring_const_ref_gasva(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032159_gasstringhashbase_gasmember_ghashuncachedlh_gc_gasstring_gasmemb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032160_gasobject_invokewatchpoint_gasenvironment_ptr_gasstring_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032163_gasarraysortfunctor_compare_gasvalue_const_ptr_gasvalue_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_032164_garraydatabase_gasarraysortfunctor_gallocatorgh_gasarraysortfunc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032165_gasarraysortonfunctor_compare_gasvalue_const_ptr_gasvalue_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_032167_gasobject_gasobject_gasstringcontext_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032168_gasarrayobject_dtorgasarrayobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032169_gasobject_dtorgasobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032171_garraydatabase_gasvalue_ptr_gallocatorlh_gasvalue_ptr_2_garrayde(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032176_void_gasobject_foreachchild_gc_grefcountbasegc_323_releasefuncto(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032177_void_gasobject_foreachchild_gc_grefcountbasegc_323_markincyclefu(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032178_void_gasobject_foreachchild_gc_grefcountbasegc_323_scaninusefunc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032179_gasarrayobject_finalize_gc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032180_gasobject_finalize_gc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032181_gasarrayobject_jointostring_gasenvironment_ptr_gstringbuffer_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_032182_gasarrayobject_concat_gasenvironment_ptr_gasvalue_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032184_gasarrayobject_removeelements_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_032185_gasarrayobject_setelement_int_gasvalue_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_032194_garraydatabase_gasindexvalue_gallocatorgh_gasindexvalue_2_garray(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032196_garraydatabase_gasarraysortfunctor_gallocatorgh_gasarraysortfunc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032198_gasarrayobject_gas_arraytostring_gasfncall_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032201_gasarrayobject_setmember_gasenvironment_ptr_gasstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032203_gasarrayobject_getmemberraw_gasstringcontext_ptr_gasstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032204_gasobject_getmemberraw_gasstringcontext_ptr_gasstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032206_gasarrayobject_deletemember_gasstringcontext_ptr_gasstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_032207_gasobject_deletemember_gasstringcontext_ptr_gasstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_032209_gasarrayobject_visitmembers_gasstringcontext_ptr_gasobjectinterf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_032210_gasobject_visitmembers_gasstringcontext_ptr_gasobjectinterface_m(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_032212_gasarrayobject_gettextvalue_gasenvironment_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032214_gasarrayobject_initarray_gasfncall_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032216_gasobject_hasmember_gasstringcontext_ptr_gasstring_const_ref_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_032218_gasprototypebase_initfunctionmembers_gasobject_ptr_gasstringcont(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_032223_gasasbroadcaster_addlistener_gasenvironment_ptr_gasobjectinterfa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032225_gasasbroadcaster_broadcastmessagewithcallback_gasenvironment_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_032229_gasasbroadcaster_initializeinstance_gasstringcontext_ptr_gasobje(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032237_gasbevelfilterobject_setmember_gasenvironment_ptr_gasstring_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032239_gasbevelfilterobject_getmember_gasenvironment_ptr_gasstring_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032243_gasstringcontext_createconststring_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032246_gasbitmapfilterctorfunction_register_gasglobalcontext_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032247_gasbitmapdata_commoninit_gasenvironment_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032248_gasbitmapdata_setimage_gasenvironment_ptr_gfximageresource_ptr_g(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_032249_gasrectangleobject_setproperties_gasstringcontext_ptr_gasvalue_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032252_gasbitmapdata_getmember_gasenvironment_ptr_gasstring_const_ref_g(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032257_gasblurfilterobject_setmember_gasenvironment_ptr_gasstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032259_gasblurfilterobject_getmember_gasenvironment_ptr_gasstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032279_gascolormatrixfilterobject_setmember_gasenvironment_ptr_gasstrin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032281_gascolormatrixfilterobject_getmember_gasenvironment_ptr_gasstrin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032287_gascolortransformobject_getmember_gasenvironment_ptr_gasstring_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032289_gascolortransformobject_setmember_gasenvironment_ptr_gasstring_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032293_gasobject_setmemberraw_gasstringcontext_ptr_gasstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032294_void_ghashsetbase_ghashnode_gasstring_gasmember_gasstringhashfun(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032298_gasdropshadowfilterobject_setmember_gasenvironment_ptr_gasstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032300_gasdropshadowfilterobject_getmember_gasenvironment_ptr_gasstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032308_gfxampviewstats_pushcallstack_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_032309_gfxampviewstats_popcallstack_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032311_gasexternalinterfacectorfunction_getmember_gasenvironment_ptr_ga(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032315_gasglowfilterobject_setmember_gasenvironment_ptr_gasstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032317_gasglowfilterobject_getmember_gasenvironment_ptr_gasstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032329_gaskeyctorfunction_notifylisteners_gasstringcontext_ptr_int_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_032330_gasmath_getnextrandom_gfxmovieroot_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_032350_gasmatrixobject_setmatrix_gasenvironment_ptr_gmatrix2d_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032351_gasmatrixobject_getmatrix_gasenvironment_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032352_gasmatrixobject_setmatrixtwips_gasstringcontext_ptr_gmatrix2d_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032370_gasmousectorfunction_updatelistenersarray_gasstringcontext_ptr_g(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032378_gasmousectorfunction_onmousedown_gasenvironment_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_032380_gasmousectorfunction_onmouseup_gasenvironment_ptr_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_032382_gasmousectorfunction_onmousewheel_gasenvironment_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_032387_gasmousectorfunction_setmember_gasenvironment_ptr_gasstring_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032389_gasmousectorfunction_getmember_gasenvironment_ptr_gasstring_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032396_gaspagedstack_gasvalue_32_pop3(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032421_gasstringhashbase_gasobject_watchpoint_ghashuncachedlh_gc_gasstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032428_gasstringhashbase_gasmember_ghashuncachedlh_gc_gasstring_gasmemb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_032437_gasobject_watch_gasstringcontext_ptr_gasstring_const_ref_gasfunc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032439_gasobject_unwatch_gasstringcontext_ptr_gasstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_032441_gasprototypebase_dtorgasprototypebase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032444_gasprototypebase_addinterface_gasstringcontext_ptr_int_gasfuncti(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_032445_garraydatabase_gptr_gasobject_gallocatorlh_gptr_gasobject_2_garr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032446_gasprototypebase_doesimplement_gasenvironment_ptr_gasobject_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_032447_gasprototypebase_getmemberrawconstructor_gasobject_ptr_gasstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_032462_gaspointobject_getmember_gasenvironment_ptr_gasstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032480_gasrectangleobject_getmember_gasenvironment_ptr_gasstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032481_gasrectangleobject_getproperties_gasenvironment_ptr_grect_double(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032483_gasrectangleobject_setmember_gasenvironment_ptr_gasstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032484_gasrectangleobject_setproperties_gasenvironment_ptr_grect_double(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032486_gasrectangleobject_getproperties_gasstringcontext_ptr_gasvalue_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032491_gfxobject_getrectangleproperties_gasenvironment_ptr_gasobject_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032523_gasselectionctorfunction_getmember_gasenvironment_ptr_gasstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032525_gasselectionctorfunction_setmember_gasenvironment_ptr_gasstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032529_gasstagectorfunction_getmemberraw_gasstringcontext_ptr_gasstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032531_gasstagectorfunction_createrectangleobject_gasenvironment_ptr_gr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032532_gasstagectorfunction_getmember_gasenvironment_ptr_gasstring_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032534_gasstagectorfunction_setmember_gasenvironment_ptr_gasstring_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032535_gasstagectorfunction_notifyonresize_gasenvironment_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032537_gasstagectorfunction_setmemberraw_gasstringcontext_ptr_gasstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032543_gasstringobject_getmemberraw_gasstringcontext_ptr_gasstring_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032555_gasstringproto_stringsplit_gasenvironment_ptr_gasstring_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_032570_gasstylesheetobject_notifyonload_gasenvironment_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_032571_cssparser_wchar_t_parse_wchar_t_const_ptr_unsigned_long_csshandl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032572_gasstylesheetobject_gasstylesheetobject_gasenvironment_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032574_gfxtextstylemanager_clearstyles(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032576_gfxtextstylemanager_getstyle_gfxtextstylekey_keytype_gstring_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_032579_gfxtextformat_getfontlist_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032584_gfxtextstylemanager_clearstyle_gfxtextstylekey_keytype_gstring_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_032589_gastextformatobject_gastextformatobject_gasenvironment_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032590_gastextformatobject_setmember_gasenvironment_ptr_gasstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032591_gfxtextformat_setfontlist_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032592_gfxtextformat_setitalic_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_032600_gfxtextdocview_gfxtextdocview_gfxtextallocator_ptr_gfxfontmanage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_032601_gfxtextdocview_setviewrect_grect_float_const_ref_gfxtextdocview(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032602_gfxtextformat_initbydefaultvalues(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032603_gfxtextformat_merge_gfxtextformat_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032604_gfxtextparagraphformat_merge_gfxtextparagraphformat_const_ref_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032605_gfxstyledtext_setdefaulttextformat_gfxtextformat_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032606_gfxstyledtext_setdefaultparagraphformat_gfxtextparagraphformat_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032607_gfxstyledtext_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032608_gfxstyledtext_appendstring_char_const_ptr_unsigned_long_gfxstyle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_032609_gfxtextdocview_format(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032614_gfxtextformat_setfontlist_gstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032616_gfxtextlinebuffer_removelines_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_032617_gfxtextdocview_getmaxvscroll(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_032624_gasintervaltimer_set_gasfncall_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_032627_gastransformobject_gastransformobject_gasenvironment_ptr_gfxasch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_032628_void_gastransformobject_foreachchild_gc_grefcountbasegc_323_rele(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032629_void_gastransformobject_foreachchild_gc_grefcountbasegc_323_mark(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032630_void_gastransformobject_foreachchild_gc_grefcountbasegc_323_scan(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032633_gastransformobject_getmember_gasenvironment_ptr_gasstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032635_gastransformobject_setmember_gasenvironment_ptr_gasstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032640_gfxtextparagraph_gfxtextparagraph_gfxtextparagraph_const_ref_gfx(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_032641_garraydatabase_grangedata_gptr_gfxtextformat_gallocatorlh_grange(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032642_gfxtextallocator_allocateparagraphformat_gfxtextparagraphformat(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032643_gfxtextallocator_allocatetextformat_gfxtextformat_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032644_gfxtextallocator_flushparagraphformatcache_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_032645_gfxtextallocator_flushtextformatcache_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_032646_gconstructormov_grangedata_gptr_gfxtextformat_destructarray_gran(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032647_gfxtextparagraph_settextformat_gfxtextallocator_ptr_gfxtextforma(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_032648_grangedataarray_gptr_gfxtextformat_garraylh_grangedata_gptr_gfxt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032649_gfxtextparagraph_formatruniterator_op_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032650_grangedataarray_gptr_gfxtextformat_garraylh_grangedata_gptr_gfxt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_032651_gfxtextparagraph_styledtextrun_set_wchar_t_const_ptr_long_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_032652_gfxtextparagraph_gettextformat_unsigned_long_unsigned_long_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_032653_gfxtextformat_intersection_gfxtextformat_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032654_gconstructormov_grangedata_gptr_gfxtextformat_destruct_grangedat(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032655_gfxtextparagraph_setformat_gfxtextallocator_ptr_gfxtextparagraph(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032656_gfxtextparagraph_createposition_gfxtextallocator_ptr_unsigned_lo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_032657_gfxtextparagraph_settermnullformat(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032658_grangedataarray_gptr_gfxtextformat_garraylh_grangedata_gptr_gfxt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032659_gfxtextparagraph_insertstring_gfxtextallocator_ptr_wchar_t_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_032660_gfxtextparagraph_appendtermnull_gfxtextallocator_ptr_gfxtextform(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032661_gfxtextparagraph_copy_gfxtextallocator_ptr_gfxtextparagraph_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_032662_gfxtextparagraph_charactersiterator_op_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032663_gfxstyledtext_gfxstyledtext_gfxtextallocator_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032664_gfxstyledtext_appendnewparagraph_gfxtextparagraphformat_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032665_garraydatabase_gfxstyledtext_paragraphptrwrapper_gallocatorlh_gf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032666_gfxstyledtext_getallocator_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032667_gfxstyledtext_appendcopyofparagraph_gfxtextparagraph_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032668_gfxstyledtext_insertnewparagraph_garraybase_garraydata_gfxstyled(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_032669_gfxstyledtext_appendstring_wchar_t_const_ptr_unsigned_long_gfxst(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_032671_gfxstyledtext_gettextandparagraphformat_gfxtextformat_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032672_gfxstyledtext_setdefaulttextformat_gfxtextformat_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032673_void_gmsgformat_formatd1_bool_bool_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032674_void_gmsgformat_formatd1_gstringlh_gstringlh_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032676_gfxstyledtext_copystyledtext_unsigned_long_unsigned_long_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_032677_gfxstyledtext_copystyledtext_gfxstyledtext_ptr_unsigned_long_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_032678_gfxstyledtext_insertstring_wchar_t_const_ptr_unsigned_long_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_032679_gfxstyledtext_insertstyledtext_gfxstyledtext_const_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_032680_garraybase_garraydata_gfxstyledtext_paragraphptrwrapper_gallocat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032681_gfxstyledtext_remove_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032682_bool_gfxstyledtext_parsehtmlimpl_wchar_t_wchar_t_const_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_032683_gfxsgmlelementdesc_const_ptr_gfxsgmlelementdesc_findelem_wchar_t(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_032684_gfxsgmlparser_wchar_t_getnextattribute_wchar_t_const_ptr_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_032685_gfxsgmlparser_wchar_t_getnextattributevalue_wchar_t_const_ptr_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_032686_gconstructormov_gfxsgmlstackelemdesc_wchar_t_destructarray_gfxsg(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032687_gfxsgmlparser_wchar_t_parsename_wchar_t_const_ptr_ptr_unsigned_l(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032688_gfxsgmlchariter_wchar_t_decodeescapedchar(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032689_garraydatabase_gfxsgmlstackelemdesc_wchar_t_gallocatordh_gfxsgml(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032691_gfxtextformat_hashfunctor_op_gfxtextformat_const_ref_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032692_gfxtextparagraphformat_gfxtextparagraphformat_gfxtextparagraphfo(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032694_gfxtextparagraphformat_hashfunctor_op_gfxtextparagraphformat_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032695_ghashsetbase_gfxtextformatptrwrapper_gfxtextformat_gfxtextformat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032696_ghashsetbase_gfxtextformatptrwrapper_gfxtextparagraphformat_gfxt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032697_gfxtextformat_op_gfxtextformat_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_032698_ghashsetcachedentry_gfxtextformatptrwrapper_gfxtextformat_gfxtex(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032699_gfxtextdocview_dtorgfxtextdocview(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032701_gfxtextdocview_imagesubstitutor_findimagedesc_wchar_t_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_032708_gfxtextdocview_editcommand_gfxtextdocview_commandtype_void_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_032714_gfxtextdocview_sethscrolloffset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_032715_gfxtextdocview_setvscrolloffset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_032718_gfxtextdocview_findfont_gfxtextdocview_findfontinfo_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_032719_gfxtextlinebuffer_line_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032720_gfxlinecursor_dtorgfxlinecursor(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032721_gfxlinecursor_op_gfxlinecursor_ref_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032722_gfxlinecursor_op_gfxlinecursor_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032723_gfxtextlinebuffer_line_getformatdata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032724_gfxparagraphformatter_finalizeline(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032725_gfxtextlinebuffer_glyphiterator_updatedesc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032726_gfxlinecursor_op_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032727_gfxtexthighlighter_createhighlighter_gfxtexthighlightdesc_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032729_gfxtextlinebuffer_display_gfxdisplaycontext_ref_gmatrix2d_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_032730_gfxtexthighlighter_drawbackground_gfxdrawingcontext_ref_gfxtextl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32); }
extern "C" { pub fn f_032731_gfxtextlinebuffer_drawmask_gfxdisplaycontext_ref_gmatrix2d_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_032732_gfxtextlinebuffer_drawunderline_float_float_float_float_grect_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i64, a10: i64); }
extern "C" { pub fn f_032739_gfxtextlinebuffer_findlineatyoffset_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_032741_gfxtextdocview_getcursorposinlinebyoffset_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i64; }
extern "C" { pub fn f_032745_gfxtextlinebuffer_findlinebytextpos_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032749_gfxtextdocview_getexactcharboundaries_grect_float_ptr_unsigned_l(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_032750_gfxtextdocview_getfirstcharinparagraph_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032751_gfxtextdocview_getbottomvscroll(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_032752_gfxtexthighlighterrangeiterator_initcurdesc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032753_garraydatabase_gfxtexthighlightdesc_gallocatorlh_gfxtexthighligh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032754_gfxtextdocview_getselectionhighlighterdesc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032756_gfxtextdocview_setdefaulttextandparaformat_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032759_gfxtexteditorkit_setcursorpos_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_032761_gfxtexteditorkit_calccursorrectinlinebuffer_unsigned_long_grect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_032762_gfxtexteditorkit_calccursorrectonscreen_unsigned_long_grect_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_032763_gfxtexthighlighterpositerator_initcurdesc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032765_gfxtexteditorkit_copytoclipboard_unsigned_long_unsigned_long_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_032770_gfxtexthighlighterpositerator_isfinished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_032771_gfxtexthighlighterpositerator_op_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032772_gfxtextlinebuffer_islinevisible_unsigned_int_float_const(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_032773_garraydatabase_gptr_functreeitem_gallocatorlh_gptr_functreeitem(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032774_gfxampviewstats_dtorgfxampviewstats(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032776_garraydatabase_gptr_functreeitem_gallocatorlh_gptr_functreeitem(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032778_void_functreeitem_visit_funcstatsvisitor_funcstatsvisitor_ref_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032779_gfxampviewstats_updatestats_unsigned_long_long_unsigned_int_gfxa(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_032804_gasmousectorfunction_dtorgasmousectorfunction_56277(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032810_gfxstyledtext_dtorgfxstyledtext_56300(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032822_gastransformobject_dtorgastransformobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032827_void_gasprototype_gasampmarker_gasenvironment_foreachchild_gc_gr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032828_void_gasprototype_gasampmarker_gasenvironment_foreachchild_gc_gr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032832_gasprototype_gasampmarker_gasenvironment_getmemberraw_gasstringc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032845_gasprototype_gasarrayobject_gasenvironment_getmemberraw_gasstrin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032997_ghashsetbase_gstringlh_hashnode_gasmoviecliploader_progressdesc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033000_ghashsetbase_ghashnode_gasstring_gasmember_gasstringhashfunctor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033001_ghashsetbase_ghashnode_gasstring_gasobject_watchpoint_gasstringh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033002_ghashsetnodeentry_gc_ghashnode_gasstring_gasobject_watchpoint_ga(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033003_ghashsetnodeentry_gc_ghashnode_gasstring_gasobject_watchpoint_ga(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033004_gfxsgmlparser_wchar_t_skipattribute(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033008_csstokenizer_wchar_t_getnexttoken(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033010_garraydatabase_gfxtextstyle_ptr_gallocatorgh_gfxtextstyle_ptr_2(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033013_gfxtextformatptrwrapper_gfxtextformat_op_gfxtextformatptrwrapper(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033014_long_ghashsetbase_gfxtextformatptrwrapper_gfxtextparagraphformat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_033015_ghashsetbase_ghashnode_gfxtextformat_const_ptr_gptr_gfxfonthandl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033016_gasvalue_gasvalue_gasobject_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033017_gasvalue_droprefs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033019_grefcountbasegc_323_releaseinternal(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033023_gasvalue_toobject_gasenvironment_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033024_gasstringnode_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033027_gascfunctionobject_gascfunctionobject_gasstringcontext_ptr_gasob(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_033028_gascfunctionobject_gascfunctionobject_gasstringcontext_ptr_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_033030_gasglobalcontext_getprototype_gasbuiltintype(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_033038_gfxascharacter_createcharacterhandle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033040_gasenvironment_findtarget_gasstring_const_ref_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_033041_gfxcharacterhandle_resolvecharacter_gfxmovieroot_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033044_gasvalue_setasobject_gasobject_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033046_gasvalue_op_gasvalue_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033049_gasstringmanager_createstringnode_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_033053_gasvalue_setasobjectinterface_gasobjectinterface_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033054_gasfncall_arg_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_033056_gasvalue_setascharacter_gfxascharacter_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033063_gasvalue_add_gasenvironment_ptr_gasvalue_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033064_gasvalue_tonumber_gasenvironment_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> f64; }
extern "C" { pub fn f_033067_gasvalue_tobool_gasenvironment_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033068_gasvalue_tostringimpl_gasenvironment_ptr_int_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_033070_gasstringnode_resolvelowercase_impl(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033073_gasstringmanager_createstringnode_char_const_ptr_unsigned_long_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_033074_gasenvironment_findtargetbyvalue_gasvalue_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033081_gasvalue_tofunction_gasenvironment_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033082_gasenvironment_logscripterror_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033084_gasvalue_gasvalue_gasvalue_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033085_gasenvironment_operatornew_gasobject_ptr_gasstring_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_033086_gasvalue_toascharacter_gasenvironment_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033090_gasvalue_toobjectinterface_gasenvironment_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033091_gfxmovieroot_addloadqueueentry_char_const_ptr_char_const_ptr_gas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_033093_gfxmovieroot_parselevelname_char_const_ptr_char_const_ptr_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_033095_gasenvironment_findvariable_gasenvironment_getvarparams_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_033096_gasfunctionrefbase_assign_gasfunctionrefbase_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033097_gasfunctionrefbase_setlocalframe_gaslocalframe_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033098_void_gasfunctionrefbase_foreachchild_gc_grefcountbasegc_323_rele(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033099_garraypagedbase_grefcountbasegc_323_ptr_10_5_gallocatorpagedlh_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033100_void_gasfunctionrefbase_foreachchild_gc_grefcountbasegc_323_mark(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033101_void_gasfunctionrefbase_foreachchild_gc_grefcountbasegc_323_scan(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033113_gasnumberutil_tostring_double_char_ptr_unsigned_long_int(ph: *mut Phantom, a0: f64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_033114_gasvalue_gasvalue_gfxascharacter_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033115_gasvalue_gasvalue_gasstringcontext_ptr_void_ptr_gasfncall_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_033116_gasvalue_gasvalue_gasfunctionref_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033117_gasvalue_dtorgasvalue(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033118_void_gasvalue_foreachchild_gc_grefcountbasegc_323_releasefunctor(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033119_void_gasvalue_foreachchild_gc_grefcountbasegc_323_markincyclefun(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033120_void_gasvalue_foreachchild_gc_grefcountbasegc_323_scaninusefunct(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033121_gasvalue_finalize_gc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033122_gasstringmanager_createconststringnode_char_const_ptr_unsigned_l(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_033123_gasvalue_getcharacternamepath_gasenvironment_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033127_gasvalue_toint32_gasenvironment_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033128_gasvalue_touint32_gasenvironment_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033129_gasenvironment_getmovieroot_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033130_gasvalue_setasfunction_gasfunctionrefbase_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033131_gasvalue_setasresolvehandler_gasfunctionrefbase_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033132_gasvalue_toprimitive_gasenvironment_ptr_gasvalue_hint_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033134_gasvalue_setpropertyvalue_gasenvironment_ptr_gasobjectinterface(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033135_gasenvironment_getlog_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033137_gasfncall_checkthisptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_033138_gasfncall_thisptrerror_char_const_ptr_char_const_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033160_void_ghashsetbase_ghashnode_gasbuiltintype_gptr_gasobject_gfixed(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033161_gasnamefunction_addconstmembers_gasobjectinterface_ptr_gasstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_033171_gasglobalcontext_getgc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033173_gasglobalcontext_getbuiltinclassregistrar_gasstring(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033177_gasstringhashbase_gasfunctionref_ghashuncachedlh_gasstring_gasfu(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033186_garraydatabase_gasstring_gallocatorlh_gasstring_323_garraydefaul(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033187_gasstring_getlength_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_033188_gasstring_getcharat_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_033189_gasstring_substring_int_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_033191_gfxcharacter_getworldmatrix_gmatrix2d_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033192_gfxcharacter_getworldmatrix3d_gmatrix3d_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033199_gfxmovieroot_addloadqueueentrymt_gfxloadqueueentry_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033200_gasenvironment_logscriptwarning_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033202_gasenvironment_getcollector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033205_gasenvironment_getconstructor_gasbuiltintype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033206_gfxeventid_getfunctionnamebuiltintype_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_033209_gasstringcontext_invalidateoptadvancelist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033213_gasstring_toupper_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033214_gasstring_tolower_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033215_gasstring_localecompare_casecheck_char_const_ptr_unsigned_long_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_033216_gasstringmanager_createstringnode_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033217_gasstringnode_releasenode(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033219_gfxlogbase_gfxlog_logscripterror_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033240_gfxcharacter_getlevelmatrix_gmatrix2d_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033253_gfxascharacter_getabsolutepath_gstring_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033254_gfxcharacterhandle_dtorgfxcharacterhandle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033273_void_gmsgformat_formatd1_unsigned_int_unsigned_int_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033278_gfxascharacter_getgeomdata_gfxascharacter_geomdatatype_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033285_gfxmovieroot_gettopmostentity_gpoint_float_const_ref_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_033292_garraydatabase_gasvalue_gallocatorgh_gasvalue_2_garraydefaultpol(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033348_gfxdrawingcontext_gfxdrawingcontext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033349_gfxdrawingcontext_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033355_gfxdrawingcontext_setfill_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_033357_gfxdrawingcontext_moveto_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_033359_gfxdrawingcontext_lineto_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_033364_gfxdrawingcontext_addpath(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033365_gfxdrawingcontext_display_gfxdisplaycontext_ref_gmatrix2d_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_033366_gfxshapebase_display_gfxdisplayparams_ref_bool_gfxcharacter_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_033373_gfxfonthandle_dtorgfxfonthandle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033382_gfxlogbase_gfxlog_logwarning_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033387_gfxtextallocator_gfxtextallocator_gmemoryheap_ptr_unsigned_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_033408_gfxfontmanager_createfonthandle_char_const_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_033424_gfxlogbase_gfxlog_logerror_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033429_gfxfontcachemanagerimpl_invalidateall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033434_gfxglyphslotqueue_releaseslot_gfxglyphdynaslot_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033436_gfxglyphrastercache_updatetextures_grenderer_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033437_void_ghashsetbase_gfxfontresource_ptr_gfxfonthashop_gfxfonthasho(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033438_gfxfontcachemanagerimpl_getglyphshape_nolock_gfxfontresource_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: f32, a7: f32, a8: i32, a9: i64) -> i64; }
extern "C" { pub fn f_033439_gfxtextlinebuffer_line_begin_gfxtexthighlighter_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033441_gfx_clipbitmapdesc_grenderer_bitmapdesc_ptr_grect_float_const_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033442_gfxfontcachemanagerimpl_releasebatchpackage_gfxbatchpackage_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033444_gfximageresource_ptr_gfxresourcebinding_op_gfximageresource_gfxr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033445_long_ghashsetbase_ghashnode_gfxglyphparam_gfxglyphnode_ptr_gfxgl(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033470_gfxresource_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033480_gfxfonthandle_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033553_gfxfontresource_calcloweruppertop_gfxlog_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033559_gfxshapebase_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033562_gfxglyphslotqueue_initnewslot_gfxglyphband_ptr_unsigned_int_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_033563_gfxglyphslotqueue_packglyph_unsigned_int_unsigned_int_gfxglyphdy(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_033566_garraypagedbase_gfxglyphfitter_vertextype_6_16_gallocatorpagedlh(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033598_gfxloaderimpl_loadmovieimage_char_const_ptr_gfximageloader_ptr_g(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_033599_gfxwstringbuffer_resize_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033685_void_ghashsetbase_gstringlh_hashnode_unsigned_int_gstring_nocase(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033716_gfxmovieroot_getlevelmovie_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_033739_gfxmovieroot_invokealiasinfo_dtorinvokealiasinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033741_gasintervaltimer_dtorgasintervaltimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033747_gfxmovieroot_addloadqueueentry_gfxascharacter_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_033764_gfxmovieroot_finalizeprocessfocuskey_gfxmovieroot_processfocuske(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033765_gfxmovieroot_processfocuskey_gfxevent_eventtype_gfxinputeventsqu(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_033766_gfxmovieroot_hidefocusrect_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_033768_gfxmovieroot_initfocuskeyinfo_gfxmovieroot_processfocuskeyinfo_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_033789_gfxmovieroot_asvalue2gfxvalue_gasenvironment_ptr_gasvalue_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033790_gfxvalue_objectinterface_objectrelease_gfxvalue_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033809_gfxmovieroot_setmodalclip_gfxsprite_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033810_gfxmovieroot_getmodalclip_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_033811_gfxmovieroot_transferfocus_gfxascharacter_ptr_unsigned_int_gfxfo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_033812_gfxmovieroot_setkeyboardfocusto_gfxascharacter_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033821_gfxmovieroot_findexportedresource_gfxmoviedefimpl_ptr_gfxresourc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_033846_gfxrandom_nextrandom(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034035_gfxwstringbuffer_setstring_wchar_t_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034040_gfxfillstyle_gfxfillstyle(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034041_gfxfillstyle_dtorgfxfillstyle(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034043_gfxlinestyle_dtorgfxlinestyle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034560_ghashsetnodeentry_ghashnode_gasstring_gasfunctionref_gasstringha(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034730_ghashsetbase_ghashnode_gasstring_gasfunctionref_gasstringhashfun(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034731_ghashsetnodeentry_ghashnode_gasstring_gasfunctionref_gasstringha(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034744_ghashsetbase_ghashnode_gfxbatchpackagedata_batchinfokey_gfxbatch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034752_ghashsetbase_ghashnode_gfxglyphparam_gfxglyphnode_ptr_gfxglyphpa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034760_gfxmovieroot_invokealiasinfo_op_gfxmovieroot_invokealiasinfo_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034761_ghashsetbase_ghashnode_gasstring_gfxmovieroot_invokealiasinfo_ga(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034913_cloadingscreens_sleepuntilpastlegalscreens(ph: *mut Phantom); }
extern "C" { pub fn f_034925_cloadingscreens_initupdateloadgame_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_034930_cloadingscreens_committompsp_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034931_clandingpagearbiter_shouldshowlandingpageonboot(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034935_cloadingscreens_setscreenorder_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_034936_cloadingscreens_setnewsscreenorder_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_034937_cloadingscreens_init_loadingscreencontext_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_034941_cloadingscreens_gotolanding(ph: *mut Phantom); }
extern "C" { pub fn f_034943_cloadingscreens_update(ph: *mut Phantom); }
extern "C" { pub fn f_034944_cloadingscreens_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_034946_cloadingscreens_areactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034948_cloadingscreens_keepaliveupdate(ph: *mut Phantom); }
extern "C" { pub fn f_034949_chudtools_getwidescreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034950_chudtools_getaspectratio_bool(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_034956_chudtools_getminsafezone_float_ref_float_ref_float_ref_float_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32); }
extern "C" { pub fn f_034957_cloadingscreens_tickloadingprogress_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_034959_cloadingscreens_suspend(ph: *mut Phantom); }
extern "C" { pub fn f_034960_cloadingscreens_resume(ph: *mut Phantom); }
extern "C" { pub fn f_034965_clandingpagearbiter_islandingpageactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034968_cmousepointer_update(ph: *mut Phantom); }
extern "C" { pub fn f_034969_cwarningmessage_data_data(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034970_cmousepointer_resetmouseinput(ph: *mut Phantom); }
extern "C" { pub fn f_035029_chudtools_calculatehudposition_rage_vector2_rage_vector2_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_035037_cnewhud_clearsubtitletext(ph: *mut Phantom); }
extern "C" { pub fn f_035039_chudcolour_getcolourfromkey_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035042_cnewhud_sethudcomponenttobeshown_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035044_cnewhud_sethudcomponenttobehidden_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035080_chudtools_getsafezonesize(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_035087_chudtools_issuperwidescreen_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035286_cminimap_getcurrentminimapmasksize(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035304_cminimap_switchoffwaypoint_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035326_cminimap_setvisible_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035481_cbootupscreen_canselectonline(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035482_cbootupscreen_geteventbuttontext_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_035483_cbootupscreen_getjobbuttontext_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_035848_rage_atarray_scontrolgroup_label_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036031_cscaleformmgr_requestremovemovie_int_emovieownertags(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_036033_cscaleformmgr_rendermovie_int_float_bool_bool_float(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i32, a4: f32); }
extern "C" { pub fn f_036034_cscaleformmoviewrapper_isactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036038_cscaleformmgr_ismovieactive_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036048_cscaleformmgr_beginmethod_int_escaleform_base_class_char_const_p(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_036050_cscaleformmgr_endmethod_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036053_cscaleformmgr_addparamint_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036061_cscaleformmgr_createmovie_char_const_ptr_rage_vector2_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_036064_cscaleformmoviewrapper_removemovie(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036145_cscaleformmoviewrapper_createmovieandwaitforload_escaleform_base(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_036146_cpausemenu_createbankwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_036154_cpausemenu_close(ph: *mut Phantom); }
extern "C" { pub fn f_036162_cscaleformmoviewrapper_param_addparam(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036164_cpausemenu_open_unsigned_int_bool_emenuscreen_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_036170_cpausemenu_setmenupreference_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036171_suicontexts_activate_rage_athashvalue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036172_suicontexts_deactivate_rage_athashvalue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036174_cpausemenu_setvaluebasedonpreference_int_cpausemenu_updateprefss(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_036175_cprofilesettings_set_cprofilesettings_profilesettingid_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036181_cpausemenu_updateprofilefrommenuoptions_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036197_cwarningmessage_setmessage_cwarningmessage_data_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036199_cscaleformmoviewrapper_callmethod_char_const_ptr_cscaleformmovie(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_036215_cscaleformmoviewrapper_beginmethod_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_036216_cscaleformmoviewrapper_endmethod(ph: *mut Phantom); }
extern "C" { pub fn f_036246_cpausemenu_setdefaultvalues_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036252_cpausemenu_getlanguagefromsystemlanguage(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036254_cpausemenu_isactive_pm_activeflags(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036291_cpausemenu_update(ph: *mut Phantom); }
extern "C" { pub fn f_036295_cpausemenu_checkinput_efrontend_input_bool_unsigned_int_bool_boo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_036301_cwarningscreen_checkallinput_bool_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_036313_cscaleformmgr_forcemovieupdateinstantly_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036314_cwarningscreen_setmessagewithheader_ewarningmessagestyle_rage_at(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_036364_cprofilesettings_getint_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036372_cscaleformmgr_changemovieparams_int_rage_vector2_const_ref_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_036382_cprofilesettings_exists_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036386_cprofilesettings_write_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036439_cprofilesettings_updateprofilesettings_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036632_cwarningscreen_setcontrollerreconnectmessagewithheader_ewarningm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_036637_cwarningscreen_update(ph: *mut Phantom); }
extern "C" { pub fn f_036639_cscaleformmgr_updateatendofframe_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036641_cscaleformmgr_updateallmoviesstatemachines(ph: *mut Phantom); }
extern "C" { pub fn f_036652_cuireticule_registerpedincapacitated_cped_const_ref_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_037258_cscaleformmoviewrapper_setdisplayconfig_cscaleformmgr_sdc(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037262_cscaleformmoviewrapper_cscaleformmoviewrapper_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_037263_cscaleformmoviewrapper_dtorcscaleformmoviewrapper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037264_cscaleformmgr_destroymovienow_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037266_cscaleformmgr_inittextures_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_037271_cscaleformmgr_loadpreallocationinfo_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037280_cscaleformmgr_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037286_cscaleformmgr_getrequiredscalemode_int_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037315_cscaleformmgr_updatemovieparams_int_gfxmovieview_scalemodetype(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037418_cdisplaycalibration_updateinput(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037419_cdisplaycalibration_loadcalibrationmovie_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037421_cdisplaycalibration_removecalibrationmovie(ph: *mut Phantom); }
extern "C" { pub fn f_037439_cfrontendstatsmgr_formatint64tocash_long_char_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_037501_cgamestream_flushqueue_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037537_cgamestreammgr_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037540_cgamestreammgr_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037634_ccheat_weaponcheat1(ph: *mut Phantom); }
extern "C" { pub fn f_037635_ccheat_healthcheat(ph: *mut Phantom); }
extern "C" { pub fn f_037657_cwanted_setwantedlevel_rage_vector3_const_ref_int_int_setwantedl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_037661_cmodelinfo_requestassets_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037662_cmodelinfo_haveassetsloaded_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037674_cclock_settimesample_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037676_cclock_settime_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_037678_cclock_getdisplaycode(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037679_cclock_addtime_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_037754_cthreadsetup_getpriority_rage_sysipcpriority_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037758_cgameconfig_getthreadsetup_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_037759_cgameconfig_getthreadpriority_rage_atnonfinalhashstring_rage_sys(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037761_ccrime_reportcrime_ecrimetype_centity_ptr_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_037767_cwanted_setwantedlevelnodrop_rage_vector3_const_ref_int_int_setw(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_037791_cmodelindex_matchallmodelstrings(ph: *mut Phantom); }
extern "C" { pub fn f_037801_cthepopmultiplierareas_gettrafficdensitymultiplier_rage_vector3(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_037802_cthepopmultiplierareas_getpeddensitymultiplier_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_037888_cseatmanager_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_037913_cwanted_registerhostileaction_cped_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037945_cweather_forcetypenow_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037951_cweather_forcetypeclear_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_037963_cweather_forcelightningflash(ph: *mut Phantom); }
extern "C" { pub fn f_038118_cdispatchspawnproperties_cdispatchspawnproperties(ph: *mut Phantom); }
extern "C" { pub fn f_038181_corder_getincident_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038196_cincidentmanager_gettextposition_int_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038222_corder_getisvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038337_cwantedincident_shouldignorecombatevent_cped_const_ref_cped_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038368_cincidentmanager_renderdispatchedtypesforincident_int_ref_rage_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038495_cikmanager_lookat_unsigned_int_centity_const_ptr_int_eanimboneta(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32); }
extern "C" { pub fn f_038497_cikmanager_islooking(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038505_cikmanager_request_cikrequestbodylook_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038506_cikmanager_setabsolutearmiktarget_rage_criksolverarms_earms_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: f32, a5: f32, a6: f32, a7: f32); }
extern "C" { pub fn f_038507_cikmanager_setrelativearmiktarget_rage_criksolverarms_earms_cdyn(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32, a6: f32, a7: f32, a8: f32, a9: f32); }
extern "C" { pub fn f_038538_cikrequestbodylook_cikrequestbodylook_centity_const_ptr_eanimbon(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_038630_cbasemodelinfo_getnum2deffectlights_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038631_cbasemodelinfo_getnum2deffects_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038636_cbasemodelinfo_getassetlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038650_cmlomodelinfo_getroomflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038699_cmodelinfo_getmodelinfofromlocalindex_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_038712_cmodelinfo_haveassetsloaded_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038716_cmodelinfo_getassetsaredeletable_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038721_cmodelinfo_getobjectanddependencies_rage_fwmodelid_rage_atarray(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_038722_cmodelinfo_getobjectanddependenciessizes_rage_fwmodelid_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_038747_cmodelseatinfo_evaluateseataccessfromallentrypoints_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64, a7: i32) -> i32; }
extern "C" { pub fn f_038751_cseatmanager_getpedsseatindex_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038757_cseatmanager_getlastpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038758_cseatmanager_countfreeseats_bool_bool_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_038873_cpedmodelinfo_getarehdfilesloaded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038875_cpedmodelinfo_getpedinstance_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038926_cvehiclemodelinfo_getvehicleinstance_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038973_cvehiclemodelinfo_gethdmemoryusage_unsigned_int_ref_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038992_cvehiclemodelinfo_gethdfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039016_cvehiclemodelinfo_choosevehiclecolourfancy_cvehicle_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_039246_cvehiclestreamrendergfx_getfragindex_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039576_cdoor_getdooropenratio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_039590_cdoor_updateportalstate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039614_cdoor_doortypeopensforpedswhennotlocked_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_039645_cdoor_getdoorisatlimit_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_039688_cobjectpopulation_destroyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039704_cobject_printskeletondata(ph: *mut Phantom); }
extern "C" { pub fn f_039713_cobject_fix(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039719_cobject_settask_rage_aitask_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_039757_cobjectpopulation_converttodummyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039761_cobjectpopulation_createobject_rage_fwmodelid_eentityownedby_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_039882_cpathserver_getclosestpositionforped_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_039891_cpathserver_areallnavmeshregionsloaded_rage_ainavdomain(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_039912_cpathserverambientpedgen_clearallpedgenblockingareas_bool_bool(ph: *mut Phantom); }
extern "C" { pub fn f_039924_cpathserver_drawpolydebug_rage_cnavmesh_ptr_unsigned_int_rage_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_039928_cpathserver_transformworldpostoscreenpos_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_039929_cpathserver_visualisenavmesh_rage_cnavmesh_ptr_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_039930_cpathserverthread_calculatesearchextents_rage_cpathrequest_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_039931_cpathserver_visualisepathservercommon_rage_ainavdomain_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_039933_getquadtreedetails_rage_cnavmeshquadtree_ptr_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_039942_cpathserver_switchpedsoffinarea_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040044_cpathserveropponentpedgen_issearchactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040048_cpathserver_clearpedswitchareas_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040175_cpedgroupmembership_isleader_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040185_cpedgroup_giveeventtoallmembers_cevent_ref_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040193_cpedgroups_registerkillbyplayer(ph: *mut Phantom); }
extern "C" { pub fn f_040194_cpedgroups_getpedsgroup_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040196_cpedgroups_areinsamegroup_cped_const_ptr_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040201_carcadepassiveabilityeffects_getisactive_earcadeabilityeffecttyp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_040203_carcadepassiveabilityeffects_getmodifier_earcadeabilityeffecttyp(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_040209_carea_ispointinarea_rage_vector3_const_ref_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_040323_cactionmanager_findactionresult_unsigned_int_ref_unsigned_int_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_040357_cactionmanager_initwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_040385_ccombatinfomgr_getcombatinfobyhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_041056_findplayerped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041058_cpeddebugvisualiser_visualisedamage_cped_const_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041060_cpeddebugvisualisermenu_getfocusped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041061_getcoorsandalphafordebugtext_centity_const_ref_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_041063_cpedintelligence_isfriendlywith_cped_const_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041064_cpedintelligence_isthreatenedby_cped_const_ref_bool_bool_bool_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_041072_centityboundai_computecorners_centity_const_ref_float_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32, a7: i64, a8: f32); }
extern "C" { pub fn f_041073_centityboundai_getcorners_rage_vector3_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041074_cpeddebugvisualiser_visualiseall(ph: *mut Phantom); }
extern "C" { pub fn f_041075_cpedtargetting_cpedtargetting_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041076_cpedtargetting_getcurrenttarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041077_cpedtargetting_getlosstatus_centity_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041079_centityboundai_computehitsidebyposition_rage_vector3_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041080_centityboundai_computeboundingboxplanes_rage_vector3_ptr_float_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_041081_cpedmotivation_getmotivation_cpedmotivation_pedmotivationstate_c(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_041085_cpedtargetting_findtarget_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041088_cpedintelligence_getnearbyvehicles(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041090_cpedperception_calculateviewmatrix_rage_matrix34_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041091_cpedperception_computefovvisibility_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32) -> i32; }
extern "C" { pub fn f_041115_cpedpopulation_removeallpedswithexception_cped_ptr_cpedpopulatio(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041116_cpedintelligence_flushimmediately_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041117_cpedintelligence_addtoeventhistory_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041118_cpedpopulation_removeallpedshardnotplayer(ph: *mut Phantom); }
extern "C" { pub fn f_041131_cpeddebugvisualisermenu_initbank(ph: *mut Phantom); }
extern "C" { pub fn f_041132_cpeddebugvisualisermenu_createbank(ph: *mut Phantom); }
extern "C" { pub fn f_041187_cpeddebugvisualisermenu_stopthreatresponse(ph: *mut Phantom); }
extern "C" { pub fn f_041197_cpedintelligence_setdefaultrelationshipgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041203_cpedintelligence_getrelationshipgroup_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041207_croutetodoorcalculator_testrouteforobstruction_cpointroute_ptr_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_041209_centityboundai_centityboundai_centity_const_ref_float_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32, a7: i64, a8: f32) -> i64; }
extern "C" { pub fn f_041215_cpedgeometryanalyser_canpedtargetped_cped_const_ref_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_041218_cplayerinfo_getphysicalplayerindex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041221_cpedintelligence_canattackped_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041222_cpedtargetting_registerthreat_centity_const_ptr_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041237_cpedintelligence_gettaskmelee_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041238_cpedintelligence_gettaskwrithe_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041244_cplayerspecialabilitymanager_chargeevent_abilitychargeeventtype(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_041262_cpedgeometryanalyser_isinair_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041265_cplayerinfo_isaiming_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041268_cinterestingevents_add_cinterestingevents_etype_centity_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_041282_cpedpopulation_shouldusestartupmode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041296_cpedfactory_destroyped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041324_doesrouteseghitentity_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_041382_cpedintelligence_registerthreat_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041385_cpedintelligence_findtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041387_cpedintelligence_gettaskmeleeactionresult_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041399_cpedscriptedtaskrecorddata_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041404_cpedintelligence_isfriendlywithentity_centity_const_ptr_bool_boo(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041405_cpedintelligence_isfriendlywithduetoscenarios_cped_const_ref_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041406_cpedintelligence_isfriendlywithbyanymeans_cped_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041407_cpedintelligence_isfriendlywithduetosamevehicle_cped_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041408_cpedintelligence_ispedthreatening_cped_const_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041416_cpedintelligence_recordeventforscript_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041422_cpedailodmanager_shoulddofullupdate_cped_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041423_cpedintelligence_increasebattleawareness_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_041429_cpedintelligence_canentityincreasebattleawareness_centity_ptr_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041431_cpedintelligence_canpindownothers_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041432_cpedintelligence_increaseamountpinneddown_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_041433_cpedintelligence_increasealertnessstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041447_cpedintelligence_dodged_centity_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_041448_cpedintelligence_collidedwithplayer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041471_cpedpopulation_setscenariopeddensitymultipliers_float_float(ph: *mut Phantom, a0: f32, a1: f32); }
extern "C" { pub fn f_041473_cpedpopulation_pednoncreationareaclear(ph: *mut Phantom); }
extern "C" { pub fn f_041489_cpedpopulation_getpopulationcontrolpointinfo_cpopctrl_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041496_findplayervehicle_cplayerinfo_ptr_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041510_cpedpopulation_forcepopulationinit_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041513_cpedpopulation_setambientpeddensitymultiplier_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_041518_cpedpopulation_setpopcyclemaxpopulationscaler_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_041519_cpedpopulation_getpopcyclemaxpopulationscaler(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_041530_cpedpopulation_stopspawningpeds(ph: *mut Phantom); }
extern "C" { pub fn f_041531_cpedpopulation_startspawningpeds(ph: *mut Phantom); }
extern "C" { pub fn f_041552_cpedpopulation_pednoncreationareaset_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041571_cpedpopulation_getpedlodcounts_int_ref_int_ref_int_ref_int_ref_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i64); }
extern "C" { pub fn f_041586_cplayerspecialability_getdefensemultiplier_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041626_cpedscriptedtaskrecord_getrecordassociatedwithevent_ceventscript(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041700_cplayerinfo_getpos(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041709_cplayerinfo_setplayercanbedamaged_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041715_cplayerinfo_affectedbywatercannon(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041726_cplayerinfo_getplayermeleedamagemodifier_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_041746_reportplayerresetmismatch_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_041809_cpopzones_findsmallestforposition_rage_vector3_const_ptr_ezoneca(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_041810_cpopcycle_getwantedcarmultiplier(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_041811_cpopcycle_getcurrentpercentageofmaxcars(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_041814_cpopcycle_getcurrentmaxnumparkedcars(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041838_cpedperception_computefovvisibility_rage_vector3_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041840_cpedailod_issafetoswitchtofullphysics(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041858_cpedmotivation_increasepedmotivation_cpedmotivation_pedmotivatio(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_041892_cpedtargetting_setinuse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041998_cqueriableinterface_gettargetfortasktype_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042179_cpeddamagemanager_addpeddamagepack_cped_ptr_rage_atnonfinalhashs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_042193_cpeddamagemanager_cleardamagedecals_unsigned_char_epeddamagezone(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_042400_cpedstreamrendergfx_getdrawable_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042401_cpedvariationdata_getvarorpropslotname_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_042415_cpedvariationinfo_getmaxnumdrawbls_epedvarcomp_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042425_cpedvariationpack_extractcomponent_rage_pgdictionary_rage_rmcdra(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_042428_cpedvariationinfocollection_lookupcompinfoflags_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_042429_cpedvariationinfocollection_gettexturedistribution_epedvarcomp_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_042430_cpedvariationinfo_getdlcname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042436_cpedvariationpack_getpalettevar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042437_cpedvariationpack_gettexvar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042438_cpedvariationpack_getcompvar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042445_cpedvariationinfocollection_getmaxnumprops_eanchorpoints_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042446_cpedpropsmgr_setpedprop_cped_ptr_eanchorpoints_int_int_eanchorpo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_042471_cpedvariationstream_customitoa3_unsigned_int_char_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_042489_cpedpropsmgr_getpedpropidx_cped_const_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042532_cpedvariationdebug_createpedcb(ph: *mut Phantom); }
extern "C" { pub fn f_042560_cpedvariationinfocollection_getpropcollectionmetadata_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042599_cpedvariationdebug_getpolycountplayerinternal_cpedstreamrendergf(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_042600_cpedvariationdebug_getpolycountpedinternal_cpedmodelinfo_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_042602_cpedvariationinfocollection_lookupcompinfoaudioid_int_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_042603_cpedvariationinfocollection_getcompexpressionsmods_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042615_cpedpropsmgr_getmaxnumprops_cdynamicentity_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042639_cpedpropsmgr_clearpedprop_cped_ptr_eanchorpoints_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042641_cpedpropsmgr_getmaxnumpropstex_cdynamicentity_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_042646_cpedpropsmgr_checkpropflags_cped_ptr_eanchorpoints_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_042817_worldprobe_debugfindgroundzfor3dcoord_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32, a5: i64, a6: i64) -> f32; }
extern "C" { pub fn f_042818_worldprobe_cshapetestmanager_debugsubmittest_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_042821_worldprobe_cshapetestmanager_aborttest_worldprobe_cshapetestresu(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042847_worldprobe_cshapetesttaskdata_debugdrawshape_worldprobe_eresults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042848_worldprobe_cshapetestdesc_setexcludeentities_centity_const_ptr_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_042912_cpickupmanager_shutdownlevelwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_043046_ccollisionhistory_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043074_ctunableobjectmanager_gettuningentry_rage_atnonfinalhashstring_c(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043085_worldprobe_cshapetestdesc_setexcludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043164_phmaterialdebug_userrenderbound_rage_phinst_ptr_rage_phbound_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_043195_phmaterialmgrgta_getpackedpolyflagvalue_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043219_cphysics_waitforverlettasks(ph: *mut Phantom); }
extern "C" { pub fn f_043224_cphysics_loadaboutposition_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043268_cphysics_createphysicsbank(ph: *mut Phantom); }
extern "C" { pub fn f_043269_cphysics_createfragtunebank(ph: *mut Phantom); }
extern "C" { pub fn f_043281_physicshelpers_getcollisionrecordinfo_cphysical_const_ptr_cphysi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_043336_worldprobe_debugdrawintersection_rage_phintersection_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043489_cocclusion_gethizbuffer_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_043500_cocclusion_getnewocclusionaabbexactsimpletestpixelsthreshold(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043508_occlusionqueries_oqgetqueryresult_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_043551_cplantmgrdataeditor_initwidgets_rage_bkbank_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043573_cplantmgr_setforcehdgrass_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043577_cplantmgr_dbgcountcachedbounds_unsigned_int_ptr_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_043581_gbuffer_locksingletarget_gbufferrt_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043582_gbuffer_unlocksingletarget_gbufferrt(ph: *mut Phantom); }
extern "C" { pub fn f_043626_postfx_addwidgetsondemand(ph: *mut Phantom); }
extern "C" { pub fn f_043645_shaderutil_startshader_char_const_ptr_rage_grmshader_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_043646_shaderutil_endshader_rage_grmshader_const_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_043654_postfx_bulletimpactoverlay_registerbulletimpact_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_043677_animpostfxmanager_start_rage_atnonfinalhashstring_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_043773_cpostscan_reset(ph: *mut Phantom); }
extern "C" { pub fn f_043782_cpostscan_waitforsortpvsdependency(ph: *mut Phantom); }
extern "C" { pub fn f_043785_cpostscan_waitforpostscanhelper(ph: *mut Phantom); }
extern "C" { pub fn f_043787_cpostscan_isonalphaoverridelist_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043792_crendertargets_allocaterendertargets(ph: *mut Phantom); }
extern "C" { pub fn f_043799_crenderphasecascadeshadowsinterface_script_initsession(ph: *mut Phantom); }
extern "C" { pub fn f_043811_crenderer_clearrenderlists(ph: *mut Phantom); }
extern "C" { pub fn f_043812_printandcountshaders_rage_fwrenderlistdesc_ref_renderpassid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_043814_crendertargets_updatebank_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043815_lights_addscenelight_clightsource_ref_clightentity_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043817_crenderer_addwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_043825_ccsmdebugglobals_initwidgets_softshadows_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043902_crendertargetmgr_getnamedrendertarget_rage_atfinalhashstring_con(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043905_crendertargetmgr_registernamedrendertarget_char_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_043908_crendertargetmgr_releasenamedrendertarget_rage_atfinalhashstring(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_043909_crendertargetmgr_getnamedrendertargetid_char_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_043911_crendertargetmgr_linknamedrendertargets_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_043912_crendertargetmgr_isnamedrendertargetlinked_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_043929_crendertargets_isshowingrendertarget_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043930_crendertargets_showrendertarget_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043931_crendertargets_showrendertargetbrightness_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_043932_crendertargets_showrendertargetnativeres_bool_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043933_crendertargets_showrendertargetfullscreen_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043934_crendertargets_showrendertargetinfo_bool(ph: *mut Phantom); }
extern "C" { pub fn f_043935_crendertargets_showrendertargetreset(ph: *mut Phantom); }
extern "C" { pub fn f_043962_cgtaoldloadingscreen_forceloadingrenderfunction_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043972_csprite2d_setmultifadeparams_rage_fwrect_const_ref_bool_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_043974_csprite2d_drawmultifade_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043983_csprite2d_drawrect_float_float_float_float_float_rage_color32_co(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i64); }
extern "C" { pub fn f_044101_csecondarywatertune_initwidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044122_water_getwaterlevel_rage_vector3_const_ref_float_ptr_bool_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_044125_water_testlineagainstwater_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_044126_water_blocktillupdatecomplete(ph: *mut Phantom); }
extern "C" { pub fn f_044178_water_setunderwaterrefractionindex_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_044203_water_renderdebug_rage_color32_rage_color32_rage_color32_bool_bo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_044297_deferredlighting_setlightshaderparams_edeferredshaders_clightsou(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_044344_centityselect_getselectedentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_044348_centityselect_addwidgets_rage_bkgroup_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044401_cdrawrectdc_cdrawrectdc_rage_fwrect_const_ref_rage_color32(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_044428_rage_populatebitfields_rage_fraginst_ptr_unsigned_long_ref_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_044455_deferredlighting_setprojectionshaderparams_edeferredshaders_mult(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_044528_cparaboloidshadow_getlightshadowindex_clightsource_const_ref_boo(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_044545_lights_calculatefromambient_clightsource_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044547_lights_startpedlightfadeup_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_044568_clightentity_getworldposition_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_044571_clightentity_getlightshaft_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_044572_clightentity_getlight_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_044602_void_rage_dlc_addinternal_void_ptr_int_int_int_bool_bool_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_044607_cparaboloidshadow_updatelight_clightsource_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_044663_crenderphasecascadeshadowsinterface_script_setshadowtype_ecsmsha(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044664_crenderphasecascadeshadowsinterface_script_setcascadebounds_int(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: f32); }
extern "C" { pub fn f_044665_crenderphasecascadeshadowsinterface_script_setcascadeboundshfov(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_044666_crenderphasecascadeshadowsinterface_script_setcascadeboundsvfov(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_044667_crenderphasecascadeshadowsinterface_script_setcascadeboundsscale(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_044668_crenderphasecascadeshadowsinterface_script_setentitytrackerscale(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_044669_crenderphasecascadeshadowsinterface_setentitytrackeractive_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044670_crenderphasecascadeshadowsinterface_script_setworldheightupdate(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044671_crenderphasecascadeshadowsinterface_script_setworldheightminmax(ph: *mut Phantom, a0: f32, a1: f32); }
extern "C" { pub fn f_044672_crenderphasecascadeshadowsinterface_script_setrecvrheightupdate(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044673_crenderphasecascadeshadowsinterface_script_setrecvrheightminmax(ph: *mut Phantom, a0: f32, a1: f32); }
extern "C" { pub fn f_044674_crenderphasecascadeshadowsinterface_script_setditherradiusscale(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_044675_crenderphasecascadeshadowsinterface_script_setdepthbias_bool_flo(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_044676_crenderphasecascadeshadowsinterface_script_setslopebias_bool_flo(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_044678_crenderphasecascadeshadowsinterface_script_setaircraftmode_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044679_crenderphasecascadeshadowsinterface_script_setsplitzexpweight_fl(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_044682_crenderphasecascadeshadowsinterface_script_setflycameramode_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044683_crenderphasecascadeshadowsinterface_script_enabledynamicdepthmod(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044684_crenderphasecascadeshadowsinterface_setdynamicdepthvalue_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_044727_crenderphasedebugoverlayinterface_setcolourmode_edebugoverlaycol(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044763_crenderphasedebugoverlayinterface_stencilmaskoverlaysetwatermode(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_045219_cgenericgamestorage_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_045227_cgenericgamestorage_finishallunfinishedsavegameoperations(ph: *mut Phantom); }
extern "C" { pub fn f_045232_csavegamedamagedcheck_checkifslotisdamaged_char_const_ptr_bool_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_045233_cdate_extractdatefromstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_045234_csavegamedialogscreens_handlesavegamedialogbox_unsigned_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_045244_cgenericgamestorage_queueautosave(ph: *mut Phantom); }
extern "C" { pub fn f_045257_cgenericgamestorage_dogamespecificstuffaftersuccessload_bool(ph: *mut Phantom); }
extern "C" { pub fn f_045258_csavegameautoload_autoloadcheckforsignedinplayer(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045261_csavegameautoload_autoloadenumeratecontent(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045262_csavegamedialogscreens_setsavegameerror_savegamedialogcode(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_045263_csavegamedialogscreens_handlesavegameerrorcode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045280_csavegameload_genericload(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045281_csavegameload_deserialize(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045783_perfclearinghouse_system_render_float_float_ref_int_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32, a4: f32, a5: f32); }
extern "C" { pub fn f_045788_cped_getpedhealthinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045789_cped_ismale_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045793_cped_checkbraveryflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045795_cped_checkagilityflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045797_cped_getcurrentmotiontask_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045802_cped_setcharparamsbasedonmanagertype(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045821_cped_printskeletondata(ph: *mut Phantom); }
extern "C" { pub fn f_045824_cped_createbank(ph: *mut Phantom); }
extern "C" { pub fn f_045834_cped_switchtoanimated_bool_bool_bool_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_045844_cped_setdefaultdecisionmaker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045852_cped_resetdesiredmainmovercapsuledata_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045869_cped_bumpedbyentity_centity_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_045875_cped_setusinglowlodphysics_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045876_cped_setuseextractedz_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045877_cped_setusinglowlodmotiontask_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045881_cped_setragdollstate_eragdollstate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045903_cped_getiscrouching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045904_cped_setiscrouching_bool_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_045913_cped_isfirstpersonshootermodeenabledforplayer_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045922_cped_getboneposition_rage_vector3_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045926_cped_getisdrivingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045940_cped_setvariation_epedvarcomp_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_045942_cped_isvariationinrange_epedvarcomp_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_045943_cped_setvardefault_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045944_cped_isbonelightlyarmoured_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045964_cped_cleanuppreloaddata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045968_cped_gettaskdata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045969_cped_removalispriority_rage_fwmodelid_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_045970_cped_getscenariotype_cdynamicentity_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045971_cped_getscenariopoint_cdynamicentity_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045977_cped_setdesiredheading_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_045982_cped_stopallmotion_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045983_cped_blendoutanynonmovementanims_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045996_cped_getbonetagfromragdollcomponent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046002_cped_isinfirstpersonvehiclecamera_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_046054_cped_setusingactionmode_bool_cped_actionmodeenabled_float_bool_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_046065_cped_hideblooddamage_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046066_cped_limitblooddamage_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046068_cped_clonedamage_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046096_cped_getbonepositioncachedv_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046102_cped_getisenteringvehicle_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046103_cped_getspecialability_eplayerspecialabilityslot(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046104_cped_isexitingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046105_cped_getblockingofnontemporaryeventsthisframe_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046108_cped_shoulduseinjuredmovement_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046118_cped_clearbound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046121_cped_gethasjetpackequipped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046135_cped_isgangped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046136_cped_shouldbehavelikelaw_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046138_cped_haveallstreamingreqscompleted(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046139_addassetstreamingindextolist_rage_strindex_rage_atarray_rage_str(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_046142_cped_canshoveped_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046143_cped_setlastpedshoved_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046144_cped_newsay_char_const_ptr_unsigned_int_bool_bool_int_cped_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32) -> i32; }
extern "C" { pub fn f_046152_cped_setpedinvehicle_cvehicle_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_046154_cped_canputhandsup_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046161_cped_queueclothposeindex_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046169_cped_willattackinjuredpeds_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046180_cped_instantaiupdate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046187_cped_setendurance_float_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046192_cped_isdebuginvincible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046203_cped_adddamagerecord_cped_sdamagerecord_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046211_cped_setforcedrundelay_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046226_cped_getfirstpersoncameradofs_rage_vec3v_ref_rage_quatv_ref_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64) -> i32; }
extern "C" { pub fn f_046262_clightattr_clightattr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046279_canimatedbuilding_canimatedbuilding_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046291_canimatedbuilding_printskeletondata(ph: *mut Phantom); }
extern "C" { pub fn f_046316_cbuilding_cbuilding_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046325_cgameworld_findfollowplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_046326_cgameworldheightmap_getmaxheightfromworldheightmap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_046359_cdatafilemgr_load_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046408_cinteriorinst_getinteriorforlocation_rage_fwinteriorlocation_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046410_centity_getinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046436_cportaltracker_update_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046438_cgameworld_add_centity_ptr_rage_fwinteriorlocation_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046442_cinteriorproxy_getfromlocation_rage_fwinteriorlocation_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046445_cportaltracker_collectstack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046467_cgameworld_remove_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046472_centity_getisaddedtoworld_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046473_centity_getisinexterior_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046483_centity_calculateambientscales(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046484_centity_setisvisibleformodule_eisvisiblemodule_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046490_centity_isvisible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046491_centity_getisonscreen_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046499_centity_getcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046500_centity_getisstatic_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046502_centity_getphysarch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046531_centity_getisvisibleformodule_eisvisiblemodule_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046537_getentitytypestr_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_046538_centity_getguid_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046539_centity_getisvisibleinfo_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046545_centity_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046597_centityidhelper_getentityfromid_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046599_centityiterator_centityiterator_int_centity_const_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_046600_centityiterator_getnext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046663_cextracontentwrapper_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_046672_cextracontentmanager_executetitleupdatedatapatch_unsigned_int_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046674_cextracontentmanager_beginenumeratecontent_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046686_cextracontentmanager_getcontentbydevice_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_046692_cmapfilemgr_reset(ph: *mut Phantom); }
extern "C" { pub fn f_046702_cwarpmanager_setwarp_rage_vector3_const_ref_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: f32, a6: i32) -> i32; }
extern "C" { pub fn f_046710_cextracontentmanager_executetitleupdatedatapatchgroup_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_047002_cfileloader_addallrpffiles_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047009_cinteriorproxy_getchecksum(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_047012_cfocusentitymgr_getpos(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047014_cfocusentitymgr_getvel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047020_cgameworld_findlocalplayerwanted(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047024_cloadscene_start_rage_vec3v_const_ref_rage_vec3v_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_047025_cloadscene_stop(ph: *mut Phantom); }
extern "C" { pub fn f_047030_cloadscene_isloaded(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_047032_cinteriorproxy_setrequestedstate_cinteriorproxy_erequest_module(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_047044_cgameworld_forallentitiesintersecting_rage_fwsearchvolume_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_047060_cphysical_activatephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047073_cphysical_sethealth_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047074_cphysical_changehealth_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047075_cphysical_setweapondamageinfo_centity_ptr_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_047076_cphysical_getweapondamagedtimebyentity_centity_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_047079_cphysical_hasbeendamagedbyentity_centity_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_047164_cgameworld_validateshutdown(ph: *mut Phantom); }
extern "C" { pub fn f_047165_cscene_loadmap_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047167_ccompentity_update(ph: *mut Phantom); }
extern "C" { pub fn f_047175_cscenememorycheck_setstate_cscenememorycheck_estate(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047180_cscene_addwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_047285_chdassetmanager_addcutscenerequestforhd_rage_atnonfinalhashstrin(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047286_chdassetmanager_flushcutscenerequestshd(ph: *mut Phantom); }
extern "C" { pub fn f_047291_ctexlod_updateinternal(ph: *mut Phantom); }
extern "C" { pub fn f_047305_cgameworld_getmainplayerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047334_cobjexporter_addnewline_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047335_cpostscandebug_setcountmodelsrendered_bool_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_047351_cscenecosttracker_setjunctioncapturemode_bool(ph: *mut Phantom); }
extern "C" { pub fn f_047355_csceneloader_loadscene_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047366_cplayerswitchdbg_stopswitchcb(ph: *mut Phantom); }
extern "C" { pub fn f_047376_ccompentity_ccompentity_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_047397_ccompentity_isassetsetready_ecompentityassetset(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047402_dumpmodels_rage_atarray_unsigned_int_0_unsigned_short_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047659_cplayerswitchinterface_stop(ph: *mut Phantom); }
extern "C" { pub fn f_047869_cportalvistracker_getinteriorlocation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047880_cgameworld_findlocalplayercoors(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047937_cstreaminglists_isrequired_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047982_cgameworld_findlocalplayervehicle_bool(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047985_cgameworld_clearcarsfromangledarea_rage_vector3_const_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_047994_cgameworld_createplayer_rage_vector3_ref_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047997_cgameworld_findlocalplayerheading(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_048042_cgameworldheightmap_getminheightfromworldheightmap_float_float_f(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_048043_cgameworldheightmap_getmaxheightfromworldheightmap_float_float_f(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_050589_graphics_commands_commanddrawrect_float_float_float_float_int_in(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_051564_hud_commands_commandsettextrenderid_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_052593_interior_commands_getproxyatcoords_rage_scrvector_const_ref_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_056541_object_commands_commandisdoorclosed_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_056561_object_commands_commandgetobjectfragmentdamagehealth_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> f32; }
extern "C" { pub fn f_056565_object_commands_commandsetcompositeentitystate_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_056567_object_commands_commandgetcompositeentitystate_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_056569_object_commands_commandgetcompositeentityatinternal_rage_scrvect(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063736_gtathread_getthreadwiththreadid_rage_scrthreadid(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063771_cgamescriptresource_incrementnumberofusedresources(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063788_cthescripts_process(ph: *mut Phantom); }
extern "C" { pub fn f_063796_cthescripts_unregisterentity_cphysical_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063800_cthescripts_getcurrentscriptnameandprogramcounter(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063811_cthescripts_getpoptypename_epoptype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063821_cgamescriptid_initgamescriptid_rage_scrthread_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_063879_cupsidedowncarcheck_iscarupsidedown_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063888_cboatisbeachedcheck_isboatbeached_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063970_cscriptdebug_initwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_064139_cstreamedscripts_calculatescriptstorefingerprint_unsigned_int_bo(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064425_cgamescriptresource_dtorcgamescriptresource_102634(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064668_cshaderedit_initwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064679_cshaderedit_findchangedata_char_const_ptr_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_064690_cshaderlib_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_064736_csettingsmanager_csettingsmanager(ph: *mut Phantom); }
extern "C" { pub fn f_064737_csettingsmanager_getsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064768_cloadedmodelgroup_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064773_cloadedmodelgroup_merge_cloadedmodelgroup_ptr_cloadedmodelgroup(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_064774_cloadedmodelgroup_countmembers_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064775_cloadedmodelgroup_getmember_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_064798_cloadedmodelgroup_addmember_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064800_cstreaming_hasobjectloaded_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064801_cstreaming_getobjectvirtualsize_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064802_cstreaming_getobjectphysicalsize_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064809_cstreaming_getobjectanddependencies_rage_strlocalindex_int_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_064826_cloadedmodelgroup_sortcarsbyname(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064832_cstreaming_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064845_cstreaming_isobjectinimage_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064900_cstreaming_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064913_ckeyboard_getkeyjustdown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_064929_cstreaming_setobjectisdeletable_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064934_cstreaming_removeobject_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064951_cstreamingcleanup_requestflush_bool(ph: *mut Phantom); }
extern "C" { pub fn f_065000_cstreamingdebug_calculaterequestedobjectsizes_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065012_cstreamingdebug_getmemorystatsbyallocation_rage_atarray_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_065128_cstreamingrequestlist_beginprestream_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_065132_cstreamingrequestlist_start(ph: *mut Phantom); }
extern "C" { pub fn f_065133_cstreamingrequestlist_finish(ph: *mut Phantom); }
extern "C" { pub fn f_065147_rage_atarray_cstreamingrequestframe_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065227_metricscapture_updatescriptmemresult_metricscapture_scriptmemory(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065228_metricscapture_storescriptmemresult_debuglocationmetricslist_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065229_metricscapture_updatestreamingstats_rage_atarray_metricscapture(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065230_metricscapture_storestreamingstats_debuglocationmetricslist_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065232_metricscapture_updatememdistributionresult_rage_atarray_metricsc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065233_metricscapture_storememdistributionresult_debuglocationmetricsli(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065234_metricscapture_updatepedandvehiclestatresult_metricscapture_peda(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065235_metricscapture_storepedandvehiclestatresult_debuglocationmetrics(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065236_metricscapture_storeframespersecondresult_debuglocationmetricsli(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065237_metricscapture_storemsperframeresult_debuglocationmetricslist_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065238_metricscapture_updatecputimingresults_rage_atarray_metricscaptur(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065239_metricscapture_updatethreadtimingresults_rage_atarray_metricscap(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065240_metricscapture_storecputimingresults_debuglocationmetricslist_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065241_metricscapture_storethreadtimingresults_debuglocationmetricslist(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065242_metricscapture_updatememorybucketusageresults_metricscapture_mem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065243_metricscapture_storememorybucketusageresults_debuglocationmetric(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065245_metricscapture_storedrawlistresults_debuglocationmetricslist_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065246_metricscapture_clearresults_debuglocationmetricslist_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065247_metricscapture_captureinstance_startcapture(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065269_metricscapture_saveresults_debuglocationmetricslist_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065270_metricscapture_ondemandcapturetoggle(ph: *mut Phantom); }
extern "C" { pub fn f_065273_ckeyboard_getkeydown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_065332_ccontrol_getpedsprintisdown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065367_ccontrol_stopplayerpadshaking_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_065374_ccontrol_setinputexclusive_int_rage_iovalue_disableoptions_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_065386_ccontrol_setinputvaluenextframe_rage_inputtype_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_065395_ccontrol_getmappingparametername_rage_iosource_const_ref_char_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065403_ccontrol_initwidgets_rage_bkbank_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_065405_ccontrol_addkeyboardmouseupdatebuttontorag_rage_bkbank_ref_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_065410_ccontrol_loadcurrentjoystickdefinition_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065426_ccontrolmgr_isdebugpadvalueson(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065432_ccontrol_doesusersmappingfileneedupdating_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065442_ccontrolmgr_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_065444_ccontrolmgr_update(ph: *mut Phantom); }
extern "C" { pub fn f_065446_ccontrolmgr_getmainplayercontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065449_ccontrolmgr_startupdatethread(ph: *mut Phantom); }
extern "C" { pub fn f_065450_ccontrolmgr_waitforupdatethread(ph: *mut Phantom); }
extern "C" { pub fn f_065451_ccontrolmgr_stoppadsshaking(ph: *mut Phantom); }
extern "C" { pub fn f_065456_ckeyboard_convertinputbufferintoascii_char_ptr_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065469_ccontrolmgr_startplayerpadshakebyintensity_unsigned_int_float_in(ph: *mut Phantom, a0: i32, a1: f32, a2: i32); }
extern "C" { pub fn f_065598_csettingsmanager_graphicsconfigsource_char_ptr_ref_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065599_settingsdefaults_settingsdefaults(ph: *mut Phantom); }
extern "C" { pub fn f_065611_cfilemgr_allocateextraresourcemem_unsigned_long(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065613_cfilemgr_mountaudiopacks_char_const_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_065614_cfilemgr_readline_void_ptr_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_065615_cfilemgr_openfile_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_065616_cfilemgr_readline_void_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_065617_cfilemgr_setdir_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065621_cfilemgr_openfileforwriting_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065629_ckeyboard_getkeyusage_int_int_char_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_065630_ckeyboard_getkeydept_int_int_char_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_065631_ckeyboard_haskeyusage_int_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_065632_ckeyboard_setcurrentmode_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_065640_cpad_getdebugbuttons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065667_csettingsmanager_initializeresolutionlists(ph: *mut Phantom); }
extern "C" { pub fn f_065672_csettingsmanager_applysettings(ph: *mut Phantom); }
extern "C" { pub fn f_065674_csettingsmanager_applyvideosettings_cvideosettings_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_065676_csettingsmanager_load(ph: *mut Phantom); }
extern "C" { pub fn f_065690_csettingsmanager_safeupdate(ph: *mut Phantom); }
extern "C" { pub fn f_065696_csystem_beginupdate(ph: *mut Phantom); }
extern "C" { pub fn f_065697_csystem_endupdate(ph: *mut Phantom); }
extern "C" { pub fn f_065698_csystem_wanttoexit(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065793_ctaskrunclip_ctaskrunclip_rage_fwmvtemplateid_rage_fwmvclipsetid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_065898_ctaskcutscene_ctaskcutscene_rage_crclip_const_ptr_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_065901_ctaskcutscene_setphase_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_065902_ctaskcutscene_setclip_rage_crclip_const_ptr_rage_matrix34_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_065950_ctaskscriptedanimation_ctaskscriptedanimation_rage_atnonfinalhas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32, a6: f32, a7: i32, a8: i64, a9: f32, a10: i32, a11: i32, a12: i64, a13: i32, a14: i32) -> i64; }
extern "C" { pub fn f_065995_ctaskthreatresponse_setfleefromcombat_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066008_ccombatmeleemanager_findmeleegroup_centity_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066062_ccoverpoint_getcoverpointposition_rage_vector3_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066063_ccover_findcoordinatescoverpointnotreserved_ccoverpoint_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_066066_ccover_removecoverpoint_ccoverpoint_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066067_ccoverpoint_getcoverdirectionvector_rage_vec3v_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066079_ccover_calculatevantagepointforcoverpoint_ccoverpoint_ptr_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_066089_ctaskdyingdead_setdeathanimationbyset_rage_fwmvtemplateid_rage_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_066157_ctaskthreatresponse_ctaskthreatresponse_cped_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_066297_cdeathsourceinfo_cdeathsourceinfo_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_066299_ctaskdyingdead_ctaskdyingdead_cdeathsourceinfo_const_ptr_int_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_066306_ctaskdyingdead_setforcednaturalmotiontask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066338_ctaskcomplexonfire_ctaskcomplexonfire_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_066389_ctaskinvestigate_ctaskinvestigate_rage_vector3_const_ref_cped_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_066590_ctasksharkattack_ctasksharkattack_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066604_ctaskthreatresponse_createfortargetduetoevent_cped_const_ptr_cpe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_066616_ctaskthreatresponse_ondirectlythreatened(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066617_ctaskthreatresponse_onperceivedgun(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066618_ctaskthreatresponse_settargetwantedlevelduetoevent_cevent_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066619_ctaskthreatresponse_setwantedlevelforped_cped_ptr_cevent_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066620_ctaskthreatresponse_ongunshotbulletimpact_ceventgunshotbulletimp(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066642_ctaskreacttogunaimedat_ctaskreacttogunaimedat_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066648_ctasktohurttransit_ctasktohurttransit_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066655_ctaskwrithe_ctaskwrithe_cweapontarget_const_ref_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_066700_ccover_calculatecoverarcdirections_ccoverpoint_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066705_ccover_renderboundingbox_rage_vector3_const_ref_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_066854_ctaskincover_onbulletevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067420_ctaskclip_dtorctaskclip_107850(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067851_cambientevent_cambientevent_ambienteventtype_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_067864_ctaskambientclips_addevent_cambientevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067903_ctaskambientlookatevent_ctaskambientlookatevent_cevent_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_067917_ctaskcomplexcontrolmovement_ctaskcomplexcontrolmovement_ctask_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_067955_ctaskmobilephone_isrunningmobilephonetask_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067993_ctaskmoveinair_ctaskmoveinair(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068126_ctaskdonothing_ctaskdonothing_int_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_068161_ctasksequencelist_ctasksequencelist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068163_ctasksequencelist_addtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068203_ctaskusesequence_ctaskusesequence_ctasksequencelist_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_068208_ctaskusesequence_gettasksequencelist_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068213_ctaskopendoor_ctaskopendoor_cdoor_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_068239_ctaskshoveped_ctaskshoveped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_068244_ctaskshoveped_shouldallowtask_cped_const_ptr_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068253_ctaskcrouch_ctaskcrouch_int_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068719_ctaskparachute_islanding_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068804_ctaskfall_ctaskfall_rage_fwflags_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068914_ctaskcower_ctaskcower_int_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068924_ctaskcomplexevasivestep_ctaskcomplexevasivestep_centity_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_068942_ctaskwalkroundentity_ctaskwalkroundentity_float_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_068951_ctaskwalkroundcarwhilewandering_ctaskwalkroundcarwhilewandering(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_068960_ctaskmovewalkroundvehicle_ctaskmovewalkroundvehicle_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_069017_ctaskparachute_iscrashlanding_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069019_ctaskparachute_canpedparachute_cped_const_ref_float(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069064_ctaskturntofaceentityorcoord_ctaskturntofaceentityorcoord_centit(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_069069_ctaskmoveachieveheading_ctaskmoveachieveheading_float_float_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_069160_ctaskgetoutofwater_ctaskgetoutofwater_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i64; }
extern "C" { pub fn f_069224_ctaskmovegotopointrelativetoentityandstandstill_ctaskmovegotopoi(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: f32, a5: i32) -> i64; }
extern "C" { pub fn f_069272_ctaskmovebeinformation_ctaskmovebeinformation_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069318_ctaskmovewithinattackwindow_ispositionoutsideattackwindow_cped_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_069440_cclimbdetector_process_rage_matrix34_ptr_spedtestparams_ptr_sped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_069445_cclimbdetector_getdetectedhandhold_cclimbhandholddetected_ref_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069454_ctasknavbase_setscanforobstructionsnextframe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069455_ctasknavbase_gettimesincelastscanforobstructions(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069460_ctasknavbase_getthiswaypointline_cped_const_ptr_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_069501_ctaskuseclimbonroute_ctaskuseclimbonroute_float_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: i64, a5: i64, a6: i32, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_069507_ctaskusedropdownonroute_ctaskusedropdownonroute_float_rage_vecto(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: i64, a5: i64, a6: i32, a7: i64, a8: i32, a9: i32) -> i64; }
extern "C" { pub fn f_069512_ctaskuseladderonroute_ctaskuseladderonroute_float_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: i64) -> i64; }
extern "C" { pub fn f_069627_ctaskjetpack_ctaskjetpack_rage_fwflags_unsigned_char(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069697_ctaskseekentityaiming_ctaskseekentityaiming_centity_const_ptr_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> i64; }
extern "C" { pub fn f_069755_cclimbhandholddetected_cclimbhandholddetected(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069757_cclimbhandholddetected_getmaxgroundposition_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069993_ctaskjumpvault_ctaskjumpvault_rage_fwflags_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_070399_ctasknmcontrol_ctasknmcontrol_unsigned_int_unsigned_int_rage_ait(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: f32) -> i64; }
extern "C" { pub fn f_070401_ctasknmbehaviour_canuseragdoll_cped_ptr_eragdolltriggertypes_cen(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_070694_ctasknmjumprollfromroadvehicle_ctasknmjumprollfromroadvehicle_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_070859_ctaskagitated_ctaskagitated_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070862_ctaskagitated_applyagitation_agitatedtype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_070874_ctaskagitated_findagitatedtaskforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070875_ctaskagitated_gettypeforevent_cped_const_ref_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_070876_ctaskagitated_getcontext_cped_const_ref_rage_atnonfinalhashstrin(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070877_ctaskagitated_shouldignoreevent_cped_const_ref_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_070881_ctaskagitatedaction_ctaskagitatedaction_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070913_ctasksmartflee_ctasksmartflee_caitarget_const_ref_float_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_070937_ctasksmartflee_setfleeflags_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_070964_ctaskescapeblast_ctaskescapeblast_centity_ptr_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: f32) -> i64; }
extern "C" { pub fn f_070973_ctaskscenarioflee_ctaskscenarioflee_centity_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_070986_ctaskflyaway_ctaskflyaway_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070998_ctaskexhaustedflee_ctaskexhaustedflee_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_071005_ctaskwalkaway_ctaskwalkaway_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_071018_ctaskgrowlandflee_ctaskgrowlandflee_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_071075_ctaskshockingeventhurryaway_ctaskshockingeventhurryaway_ceventsh(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_071088_ctaskreacttoexplosion_ctaskreacttoexplosion_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_071092_ctaskreacttoexplosion_choosestateforthreatresponse_cped_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_071101_ctaskreacttoimminentexplosion_ctaskreacttoimminentexplosion_cait(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i64; }
extern "C" { pub fn f_071105_ctaskreacttoimminentexplosion_isvalid_cped_const_ref_caitarget_c(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_071109_ctaskreacttodeadped_ctaskreacttodeadped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_071128_ctaskshockingeventreact_ctaskshockingeventreact_ceventshocking_c(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_071141_ctaskshockingeventbackaway_ctaskshockingeventbackaway_ceventshoc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_071168_ctaskshockingeventbackaway_getdefaultbackawaypositionforped_cped(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_071218_ctaskduckandcover_ctaskduckandcover(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071797_rage_atarray_cagitatedtrigger_0_unsigned_short_kill(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072503_cscenariopointmanager_updatecachedregionptrs_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072539_cscenariomanager_getscenarioinfo_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_072564_cscenariomanager_checkscenariotimesandprobability_cscenariopoint(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_072661_cscenariodebug_renderpoint_cscenariopoint_const_ref_int_bool_cpo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_072662_cscenariopointmanager_findpointsinarea_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_072663_cscenariomanager_checkscenariopointenabled_cscenariopoint_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_072664_cscenariopointmanager_renderpointpopularity_rage_atarray_cscenar(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072665_cscenariodebug_canrenderpoint_cscenariopoint_const_ref_cpopzone(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_072928_ctaskscenario_getscenarioinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073444_ctaskcowerscenario_shouldupdatecowerstatus_rage_aievent_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_073445_ctaskcowerscenario_updatecowerstatus_rage_aievent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073452_ctaskcowerscenario_notifycoweringpedofevent_cped_ref_cevent_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073663_ctaskpolicewantedresponse_ctaskpolicewantedresponse_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_073698_ctaskswatwantedresponse_ctaskswatwantedresponse_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_074161_ctask_ctask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_074163_ctaskclassinfomanager_gettaskname_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_075322_ctaskclassinfomanager_gettaskstatename_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_075327_ctaskcomplex_ctaskcomplex(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075372_cprioritizedclipsetrequesthelper_getclipsetid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075447_casyncprobehelper_resetprobe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075474_cmovenetworkhelper_setfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_075523_ctuning_dtorctuning(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075543_cvehiclejunctionhelper_approachinglight_cvehicle_const_ptr_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075557_cupperbodyaimpitchhelper_calculateaimvector_cped_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_075604_ctaskcontrolvehicle_ctaskcontrolvehicle_cvehicle_ptr_rage_aitask(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_075644_ctasktreeclone_startlocalclonetask_ctaskfsmclone_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075658_ctasktypes_iscombattask_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_075660_ctuning_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_075664_ctuning_registertuning_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_075695_ctaskleaveanycar_ctaskleaveanycar_int_cvehicleenterexitflags_fla(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_075697_ctaskexitvehicle_ctaskexitvehicle_cvehicle_ptr_cvehicleenterexit(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_075710_ctaskvehiclegun_ctaskvehiclegun_ctaskvehiclegun_etaskmode_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_075724_ctaskentervehicle_ctaskentervehicle_cvehicle_ptr_seatrequesttype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: f32, a6: f32, a7: i64, a8: i64) -> i64; }
extern "C" { pub fn f_075759_ctaskcomplexgetoffboat_ctaskcomplexgetoffboat_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_075804_ctaskcarreacttovehiclecollision_ctaskcarreacttovehiclecollision(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_075812_ctaskcarreacttovehiclecollisiongetout_ctaskcarreacttovehiclecoll(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32) -> i64; }
extern "C" { pub fn f_075819_ccarenterexit_getcollideddoorfromcollisioncomponent_cvehicle_con(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i64; }
extern "C" { pub fn f_075820_ccarenterexit_getdoorcomponents_cvehicle_const_ptr_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_076198_ctaskreacttobeingjacked_ctaskreacttobeingjacked_cvehicle_ptr_cpe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_076717_ctaskswapweapon_ctaskswapweapon_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_076882_ctaskgun_ongunshotwhizzedby_ceventgunshotwhizzedby_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077089_cstatsmgr_incrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077091_cstatsmgr_setgreater_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077094_cstatsmgr_endvehiclestats(ph: *mut Phantom); }
extern "C" { pub fn f_077095_cstatsmgr_isplayeractive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077097_cstatsmgr_updaterecordedstat_cstatsmgr_recordstat_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_077117_cstatsmgr_updatestatswhenpedrundown_unsigned_short_cvehicle_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_077122_cstatsmgr_registerexplosionhit_centity_const_ptr_centity_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_077152_statid_char_gethash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077153_statid_char_getstatid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077193_cstatsdatamgr_iskeyvalid_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077329_statid_set_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077334_statsinterface_iskeyvalid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077366_statsinterface_getpedoftype_cped_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077367_statsinterface_isplayervagos_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077368_statsinterface_isplayerlost_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077373_statsinterface_getstatsmodelhashid_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077383_statsinterface_getlocalplayerweaponinfostatid_statsinterface_ewe(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_077394_cstatsutils_isstatstrackingenabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077475_cnumberwithinmessage_set_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077479_ctextfile_get_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077481_ctextfile_get_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_077492_cpreviousmessages_init_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077496_cmessages_clearmessages(ph: *mut Phantom); }
extern "C" { pub fn f_077499_ctextfile_unload_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077504_cmessages_addmessage_char_const_ptr_int_unsigned_int_bool_bool_e(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i64, a9: i32, a10: i32, a11: i32); }
extern "C" { pub fn f_077507_chelpmessage_setmessagetext_int_char_const_ptr_cnumberwithinmess(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_077521_csavinggamemessage_clear(ph: *mut Phantom); }
extern "C" { pub fn f_077533_ctextlayout_ctextlayout(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077534_ctextlayout_default(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077535_ctextlayout_dtorctextlayout(ph: *mut Phantom); }
extern "C" { pub fn f_077537_ctextlayout_setscale_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077539_ctextlayout_getcharacterheight(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_077540_ctextlayout_render_rage_vector2_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077544_ctextformat_getstringwidth_ctextlayout_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_077545_ctextlayout_setscale_rage_vector2_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077546_ctext_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077547_ctext_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077549_ctext_flush(ph: *mut Phantom); }
extern "C" { pub fn f_077554_ctext_getlanguagefilename_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077570_ctextfile_getlanguagefromname_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077571_ctextfile_load_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077574_ctextfile_reloadafterlanguagechange(ph: *mut Phantom); }
extern "C" { pub fn f_077575_ctextfile_requestadditionaltext_char_const_ptr_int_ctextfile_elo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077578_ctextfile_hasthisadditionaltextloaded_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077604_ctimecycle_update_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077618_ctimecycledebug_createlevelwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_077663_cobjectpositionchecker_setmode_cobjectpositionchecker_etmode(ph: *mut Phantom); }
extern "C" { pub fn f_077664_cobjectpositionchecker_isinterestingobject_cobject_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077665_cobjectpositionchecker_writelog_rage_atarray_rage_atstring_0_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_077666_rage_atarray_rage_matrix34_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077676_cperformancestats_vecwithincurrentsector_rage_vector3_const_ref(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077677_cperformancestats_writelog(ph: *mut Phantom); }
extern "C" { pub fn f_077678_cperformancestats_readtimebarsamples_rage_atrangearray_rage_atar(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077679_cperformancestats_readgrcsamples_rage_atrangearray_rage_atarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077680_csectortools_writecommon_rage_fistream_ptr_int_char_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i64, a8: i64); }
extern "C" { pub fn f_077682_cperformancestats_movetonextsector(ph: *mut Phantom); }
extern "C" { pub fn f_077692_void_getsectormodelinfoidsforpool_canimatedbuilding_rage_fwpool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_077693_void_getsectorinteriormodelinfoidsforpool_canimatedbuilding_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_077694_void_getsectorobjectsforpool_canimatedbuilding_rage_fwpool_canim(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_077695_void_getsectorobjectsforpoolnomemorystats_cinteriorinst_rage_fwp(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_077732_cstreamingmemjunctionstats_spawn(ph: *mut Phantom); }
extern "C" { pub fn f_077733_cstreamingmemjunctionstats_capture(ph: *mut Phantom); }
extern "C" { pub fn f_077735_cstreamingmemusage_getstoreobjectslist_rage_atarray_rage_strinde(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077736_cstreamingmemusage_writepoolmemoryusage_rage_fistream_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077737_cstreamingmemusage_writestorememoryusage_rage_fistream_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077789_cjunction_findentranceindexwithnode_rage_cnodeaddress_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077790_cjunction_calculatedistancetojunction_cvehicle_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_077791_cpathfind_checkpathnodeintegrityforzone_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077792_cpathfind_testalllinks_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077813_cpathfind_findnodeslinkindexbetween2nodes_rage_cnodeaddress_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_077821_cjunctions_findjunctionfromnode_rage_cnodeaddress_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077869_displaypathsdifferencedebugforpoint_rage_vector3_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077870_cpathfind_findnearestgroundheightforbuildpaths_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_077871_cpathfind_identifydrivableextremes_rage_cnodeaddress_rage_cnodea(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_077872_cpathfind_computepositionforpedcrossreservation_rage_cnodeaddres(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_077873_cpathfind_updatestreaming_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077888_cpathfind_findlinkbetween2nodes_rage_cpathnode_const_ptr_rage_cp(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077930_cvehicleailod_getdummymode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077934_cvehicleailodmanager_shoulddofullupdate_cvehicle_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077936_cvehicleailodmanager_disabletimeslicingimmediately_cvehicle_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077946_cvehicleintelligence_hasbeenslowlypushingplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077983_ctaskvehicleflee_ctaskvehicleflee_svehiclemissionparams_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_078054_ctaskvehiclemissionbase_ctaskvehiclemissionbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078274_ctaskvehiclethreepointturn_ispointonroad_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078852_cvehicle_getisbeingtowed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078871_cvehicle_containslocalplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078935_cvehicle_getisextraon_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078936_cvehicle_ishornon_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078939_cvehicle_switchengineon_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_078944_cvehicledamage_applydamage_centity_ptr_edamagetype_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: i64, a6: i64, a7: i64, a8: i32, a9: i64, a10: i32, a11: i32, a12: i32, a13: f32, a14: i32, a15: i32, a16: i32, a17: i32, a18: i32) -> f32; }
extern "C" { pub fn f_078971_cvehicledebug_getfocusvehicle(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_078975_cvehicledamage_getisstuck_int_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_078981_cvehicle_hascontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079000_cvehicle_istank_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079007_cvehicle_getnumberofpassenger_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079038_cvehicle_getlastdriver_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079076_cvehicle_resetsuspension(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079134_cvehicle_getdriverseat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079226_cvehicle_setliveryid_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079227_cvehicle_setlivery2id_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079228_cvehicle_updatebodycolourremapping_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079436_cvehicledeformation_applycollisionimpact_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_079512_cplane_isplayerdrivingonground_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079634_cvehicle_hascollisionloadedaroundvehicle(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079729_cvehicle_printskeletondata(ph: *mut Phantom); }
extern "C" { pub fn f_079781_cvehicle_hasmissioncharsinit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079791_cvehicle_isdriver_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079796_cvehicle_hasalivepedsinit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079825_cvehicle_removefragmentcomponent_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079853_cvehicle_pickrandompassenger(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079854_cvehicle_containsped_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079865_cvehicle_hasfragmentcomponent_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079866_cvehicle_turnoffextra_ehierarchyid_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079904_cvehicle_placeonroadadjust_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_079916_cvehicle_getattachparentvehicledummyorphysical_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079926_cvehicle_getliveryid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079928_cvehicle_getlivery2id_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079948_cpoolhelpers_getvehiclepoolsize(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_080054_cvehiclepopulation_getdesirednumberambientvehicles_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> f32; }
extern "C" { pub fn f_080186_cvehiclefactory_updatecarlist(ph: *mut Phantom); }
extern "C" { pub fn f_080187_cvehiclefactory_createcar_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080258_cvehicleweaponmgr_getweaponsandturretsforseat_int_rage_atarray_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_080383_cvehiclepopulation_initvehiclespacings(ph: *mut Phantom); }
extern "C" { pub fn f_080420_cvehiclepopulation_getrandomvehdensitymultiplier_float_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_080428_cvehiclepopulation_forcepopulationinit(ph: *mut Phantom); }
extern "C" { pub fn f_080436_cvehiclepopulation_getvehiclelodcounts_bool_int_ref_int_ref_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_080440_cvehiclepopulation_stopspawningvehs(ph: *mut Phantom); }
extern "C" { pub fn f_080441_cvehiclepopulation_startspawningvehs(ph: *mut Phantom); }
extern "C" { pub fn f_080443_cvehiclepopulation_removeallvehshard(ph: *mut Phantom); }
extern "C" { pub fn f_080666_cvehiclemetadatamgr_getseataniminfofromped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080866_cvfxhelper_getmatrixfromboneindex_rage_mat34v_ref_bool_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_080906_cmoviemgr_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080911_cptfxmanager_beginframe(ph: *mut Phantom); }
extern "C" { pub fn f_080993_cdecalmanager_updatestartasync_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_081027_cvisualeffects_createwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_081166_cdecalmanager_findrendersettinginfo_unsigned_int_int_ref_int_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_081349_cdecalmanager_addgeneric_decaltype_e_int_int_rage_vec3v_const_re(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: f32, a7: f32, a8: f32, a9: f32, a10: f32, a11: f32, a12: f32, a13: f32, a14: i32, a15: i32, a16: i32, a17: f32, a18: i32, a19: i32, a20: f32) -> i32; }
extern "C" { pub fn f_081355_cfire_getpositionworld_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081416_ccoronas_register_rage_vec3v_const_ref_float_rage_color32_float(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: f32, a4: f32, a5: i64, a6: f32, a7: f32, a8: f32, a9: i32); }
extern "C" { pub fn f_081451_cfiremanager_cansetpedonfire_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_081452_cfiremanager_startpedfire_cped_ptr_cped_ptr_int_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: f32, a9: i32, a10: i32, a11: i32) -> i64; }
extern "C" { pub fn f_081461_cfiremanager_extinguishall_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081494_cmoviemgr_stop_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081498_cmoviemgr_preloadmovie_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_081499_cmoviemgr_play_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081500_cmoviemgr_release_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081530_cmoviemeshmanager_loadset_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_081531_cmoviemeshmanager_releaseset_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081551_scriptim_polytex_rage_vec3v_const_ref_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i64, a9: i64); }
extern "C" { pub fn f_081631_clodlightmanager_cacheloadedlodlightimapidx(ph: *mut Phantom); }
extern "C" { pub fn f_081632_clodlightmanager_loadlodlightimapidxcache_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081633_clodlightmanager_unloadlodlightimapidxcache_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081745_cvehicleglassmanager_smashcollision_centity_ptr_int_float_rage_v(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_081750_cvehicleglassmanager_iscomponentsmashableglass_cphysical_const_p(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_081855_cvfxscript_getcutsceneptfxassetname_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082028_cexplosionmanager_addexplosion_cexplosionmanager_cexplosionargs(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082065_cpedweaponmanager_getequippedweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082070_cweaponinfo_getammoinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082071_cpedweaponmanager_getequippedweapon_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082121_cexplosionmanager_testforexplosionsinsphere_eexplosiontag_rage_v(ph: *mut Phantom, a0: i32, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_082224_cpedinventory_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082251_cammoitemrepository_getweaponammo_cweaponinfo_const_ptr_cped_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_082252_cpedweaponmanager_getbestweaponinfo_bool_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_082554_cpedinventoryloadoutmanager_setdefaultloadout_cped_ptr_cpedmodel(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082565_cpedequippedweapon_setupasweapon_cobject_ptr_cweaponinfo_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_082594_cpedweaponmanager_equipweapon_unsigned_int_int_bool_bool_cpedequ(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_082644_cpedweaponmanager_equipbestweapon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_082648_cpedweaponmanager_getequippedturretweaponinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082653_cpedweaponmanager_getisarmedgun_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082825_networkexitflow_startexitsaveflow(ph: *mut Phantom); }
extern "C" { pub fn f_082826_networkexitflow_isshutdowntasksrunning(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082827_networkexitflow_issaveflowfinished(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082828_networkexitflow_startshutdowntasks(ph: *mut Phantom); }
extern "C" { pub fn f_082829_networkexitflow_isreadytoshutdown(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082830_networkexitflow_resetexitflowstate(ph: *mut Phantom); }
extern "C" { pub fn f_082831_cnetworktelemetry_flushtelemetry_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082832_networkutils_getbailreasonasstring_ebailreason(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082834_cnetwork_bail_sbailparameters_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082836_networkexitflow_getexitstring(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_082863_clivemanager_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082881_cnetwork_update_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082885_networkinterface_isanysessionactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082896_clivemanager_update_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082909_cnetworktelemetry_appendmetric_rage_rlmetric_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082911_networkdebug_updatefocusentity_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082958_cnetwork_setgostraighttomultiplayer_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082959_cnetwork_setgostraighttompevent_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082960_cnetwork_setgostraighttomprandomjob_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082970_networkutils_getbailerrorcodeasstring_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083013_networkinterface_getactivegamerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083014_networkinterface_getlocalgamerhandle(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083018_networkinterface_bail_ebailreason_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_083025_networkinterface_getphysicalplayerfromindex_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083036_networkinterface_ispedallowedtodamageped_cped_const_ref_cped_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083037_networkinterface_allowcrimeorthreatresponseplayerjustresurrected(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083046_networkinterface_isvisibleorclosetoanyremoteplayer_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083123_networkinterface_shouldtriggerdamageeventforzerodamage_cphysical(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083150_networkinterface_registerheadshotwithnetworktracker_cped_const_p(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083151_networkinterface_registerkillwithnetworktracker_cphysical_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083174_networkinterface_onaieventadded_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083176_networkinterface_cutscenestartedonentity_centity_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083177_networkinterface_cutscenefinishedonentity_centity_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083300_networktextfilter_init_rage_functor2_rage_rlgamerhandle_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64); }
extern "C" { pub fn f_083326_cnetworkcrewdatamgr_initwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083335_networkcrewemblemmgr_bank_initwidgets_rage_bkbank_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083401_ccloudmanager_initwidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083493_ctunables_access_unsigned_int_bool_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083609_cusercontentmanager_initwidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083649_ccommercemanager_initwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083667_networkdebug_registerfailmark_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083735_void_networkdebug_addpedblendingwidgets_cpedblenderdata_cpedblen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083736_void_networkdebug_addphysicalblendingwidgets_cphysicalblenderdat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083737_void_networkdebug_addvehicleblendingwidgets_cvehicleblenderdata(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083762_cnetworkassetverifier_generatefilecrc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083763_cnetworkassetverifier_generateenvironmentcrc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083764_cnetworkassetverifier_generatestaticcrc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083765_cnetworkassetverifier_generatecrc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083766_cnetworkassetverifier_dumpsyncdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083770_networkcolours_getnetworkcolour_networkcolours_networkcolour(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083771_networkcolours_getdefaultplayercolour_unsigned_char(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083825_cnetworkghostcollisions_islocalplayeringhostcollision_cphysical(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083900_csyncedentity_setentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083902_csyncedentity_getentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083917_networkutils_setfontsettingsfornetworkohd_ctextlayout_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_083918_networkutils_getpedfromdamageentity_centity_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083958_cnetworkcargenworldstatedata_adddebugwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_083959_cnetworkentityareaworldstatedata_adddebugwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_083960_cnetworkpopgroupoverrideworldstatedata_adddebugwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_083961_cnetworkpopmultiplierareaworldstatedata_adddebugwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_083962_cnetworkroadnodeworldstatedata_adddebugwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_083963_cnetworkropeworldstatedata_adddebugwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_083964_cnetworkscenarioblockingareaworldstatedata_adddebugwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_083965_cnetworkvehicleplayerlockingworldstate_adddebugwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_084172_clivemanager_refreshactiveplayer(ph: *mut Phantom); }
extern "C" { pub fn f_084186_clivemanager_shouldskipbootui_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084287_metricsecchange_metricsecchange(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084320_cnetworktelemetry_playerdied_cped_const_ptr_cped_const_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_084331_cnetworktelemetry_loadgame(ph: *mut Phantom); }
extern "C" { pub fn f_084349_cnetworktelemetry_appendsecuritymetric_metricsecchange_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084521_cplayercardxmldatamanager_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084522_cplayercardxmldatamanager_loaddataxmlfile_rage_partreenode_ptr_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_084523_cplayercardxmldatamanager_loaddataxmlfile_rage_partreenode_ptr_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_084524_cplayercardxmldatamanager_loaddataxmlfile_rage_partreenode_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_084525_cplayercardxmldatamanager_cardstat_setstat_char_const_ptr_bool_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_084615_char_ptr_rage_formatf_char_202ul_char_ref_202ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085430_csocialclubinboxmgr_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085438_cnetworknewsstorymgr_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_085471_gamepresenceevents_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085601_socialclubeventmgr_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_085612_socialclubeventmgr_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085624_cnetworkemailmgr_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_085633_cnetworkemailmgr_initwidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085654_cnetworktransitionnewscontroller_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_085663_cnetworktransitionnewscontroller_setpaused_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085679_cnetworknewsstorymgr_initwidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085697_policyversions_policyversions(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085701_policyversions_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085702_policyversions_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085712_policyversions_initwidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086086_cnetworkreadleaderboards_initwidgets_rage_bkbank_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_086092_cnetworkwriteleaderboards_initwidgets_rage_bkbank_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_086225_gametransactionsessionmgr_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_086346_networkgametransactions_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086347_gametransactionsessionmgr_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086744_cnetgameplayer_getplayerped_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086763_cnetobjgame_getplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086787_cnetworkobjectpopulationmgr_adddebugwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_087295_cnetobjped_getisdamagablebyplayer_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_087661_cnetobjplayer_getextendedscopefovmultiplier_float(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_087750_cnetobjvehicle_lockforplayer_unsigned_char_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087751_cnetobjvehicle_lockforallplayers_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_090695_errno_location(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_090706_r(ph: *mut Phantom, a0: f64) -> f64; }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090710_asinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090711_atan(ph: *mut Phantom, a0: f64) -> f64; }
extern "C" { pub fn f_090712_atan2(ph: *mut Phantom, a0: f64, a1: f64) -> f64; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090717_atof(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090726_rem_pio2(ph: *mut Phantom, a0: f64, a1: i64) -> i32; }
extern "C" { pub fn f_090728_cos(ph: *mut Phantom, a0: f64) -> f64; }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090748_exp(ph: *mut Phantom, a0: f64) -> f64; }
extern "C" { pub fn f_090755_exp2f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090757_expf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090762_fclose(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090764_fflush(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090775_fopen(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090776_fiprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090777_small_fprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090781_memcpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090790_fwrite(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_090797_isxdigit(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090803_log(ph: *mut Phantom, a0: f64) -> f64; }
extern "C" { pub fn f_090807_logf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090809_lrintf(ph: *mut Phantom, a0: f32) -> i64; }
extern "C" { pub fn f_090814_memcmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090815_mktime(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090823_pow(ph: *mut Phantom, a0: f64, a1: f64) -> f64; }
extern "C" { pub fn f_090826_powf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090885_pthread_mutex_destroy(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090887_pthread_mutex_lock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090889_pthread_mutex_unlock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090900_puts(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090901_qsort_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_090910_srand(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_090911_rand(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090917_sin(ph: *mut Phantom, a0: f64) -> f64; }
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
extern "C" { pub fn f_090941_strncat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090942_strncmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090945_strrchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090947_strstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090956_strtok(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090957_strtoull(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_090959_strtoll(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_090960_syscall_ret(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090962_system(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090963_tan(ph: *mut Phantom, a0: f64, a1: f64, a2: i32) -> f64; }
extern "C" { pub fn f_090965_tanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090970_tolower(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090985_vsnprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090991_wcslen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_091000_emscripten_builtin_malloc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_091002_emscripten_builtin_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091069_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
