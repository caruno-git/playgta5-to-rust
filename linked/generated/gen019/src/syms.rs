//! Callee declarations for crate gen019.
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
extern "C" { pub fn f_000113_rage_syscriticalsectiontoken_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000114_rage_sysipccreatesema_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_000115_rage_sysipcpollsema_rage_sysipcsematag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000116_rage_sysipcwaitsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000119_rage_sysipcsignalsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000122_rage_sysipcunlockmutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000123_rage_sysipcdeletemutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000128_rage_sysipccreatethread_void_ptr_void_ptr_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_000131_rage_sysipcwaitthreadexit_rage_sysipcthreadid_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000134_rage_sysipcyield_int(ph: *mut Phantom); }
extern "C" { pub fn f_000135_rage_sysipcsleep_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000243_rage_grcdevice_registerdevicelostcallbacks_rage_functor0_rage_fu(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000644_rage_athashnextsize_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000647_rage_atreferencecounter_atreferencecounter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000650_rage_athash16u_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000655_rage_atstring_lowercase(ph: *mut Phantom, a0: i64); }
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
extern "C" { pub fn f_000713_rage_bkremotepacket_sendpackets(ph: *mut Phantom); }
extern "C" { pub fn f_000758_rage_bkmanager_openfile_char_ptr_int_char_const_ptr_bool_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_000761_rage_bksliderval_float_bksliderval_rage_datcallback_ref_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32, a7: f32, a8: i64, a9: i32, a10: i32) -> i64; }
extern "C" { pub fn f_000773_rage_bkmanager_createoutputwindow_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000779_rage_bkmanager_createbank_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000782_rage_bkmanager_findbank_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000785_rage_bkmanager_destroybank_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000786_rage_bkmanager_isbankthread(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000796_rage_bkwidget_findchild_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000806_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000828_rage_bklist_addcolumnheader_int_char_const_ptr_rage_bklist_itemt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_000830_rage_bklist_additem_int_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_000831_rage_bklist_removeitem_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000843_rage_bkcombo_setstring_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_000844_rage_bkcombo_getstring_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_000866_rage_bkgroup_addseparator_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000867_rage_bkgroup_addtitle_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000868_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_000869_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000870_rage_bkgroup_addbutton_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000872_rage_bkgroup_addtext_char_const_ptr_float_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000874_rage_bkgroup_addtext_char_const_ptr_int_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000876_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_000877_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_000882_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000884_rage_bkgroup_addtoggle_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000893_rage_bkgroup_addtoggle_char_const_ptr_float_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000894_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000897_rage_bkgroup_addslider_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000898_rage_bkgroup_addslider_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000899_rage_bkgroup_addslider_char_const_ptr_signed_char_ptr_signed_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_000901_rage_bkgroup_addslider_char_const_ptr_unsigned_short_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_000905_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000906_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000907_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int_rage_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000909_rage_bkgroup_addslider_char_const_ptr_rage_vector2_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000911_rage_bkgroup_addslider_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000912_rage_bkgroup_addslider_char_const_ptr_rage_vector4_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000914_rage_bkgroup_addslider_char_const_ptr_rage_vector3_ptr_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_000915_rage_bkgroup_addslider_char_const_ptr_rage_scalarv_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000918_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000919_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000920_rage_bkgroup_addcombo_char_const_ptr_unsigned_char_ptr_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000921_rage_bkgroup_addlist_char_const_ptr_bool_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000925_rage_bkgroup_addvector_char_const_ptr_rage_vector2_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000926_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000927_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000929_rage_bkgroup_addvector_char_const_ptr_rage_vector4_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000937_rage_bkgroup_addangle_char_const_ptr_float_ptr_rage_bkangletype(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000939_rage_bkgroup_addangle_char_const_ptr_float_ptr_rage_bkangletype(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32); }
extern "C" { pub fn f_000942_rage_bkgroup_addcolor_char_const_ptr_rage_color32_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000944_rage_bkgroup_addcolor_char_const_ptr_float_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000945_rage_bkgroup_addcolor_char_const_ptr_rage_vector4_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000946_rage_bkgroup_addcolor_char_const_ptr_rage_vector3_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000990_char_ptr_rage_formatf_char_512ul_char_ref_512ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001040_rage_matrix33_rotateto_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001049_rage_matrix33_rotate_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_001062_rage_matrix34_inverse_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001105_rage_autoidregistry_autoidregistry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001108_rage_autoidregistry_unregister_rage_autoiddescriptor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001116_char_ptr_rage_formatf_char_32ul_char_ref_32ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001117_rage_datcallback_datcallback(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001119_rage_datcallback_datcallback_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001140_rage_datbitbuffer_readunsigned_void_const_ptr_unsigned_int_ref_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001141_rage_datbitbuffer_writeunsigned_void_ptr_unsigned_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_001156_rage_datbitbuffer_bumpreadcursor_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001158_rage_datbitbuffer_bumpreadwritecursor_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001514_rage_diagdisplayf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001516_rage_fistream_write_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001523_quitf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001524_displayf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001527_warningf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001529_rage_fistream_create_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001532_rage_fistream_close(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001533_rage_fidevice_getdevice_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001534_quitf_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001535_rage_diaglogf2_rage_diagchannel_const_ref_rage_diagseverity_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001537_rage_fistream_size(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001543_rage_diagloggedprintf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001549_rage_diaglogdefault_rage_diagchannel_const_ref_rage_diagseverity(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001559_rage_diagcontextmessage_diagcontextmessage_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001563_rage_diagcontextmessage_pop(ph: *mut Phantom); }
extern "C" { pub fn f_001565_printf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001577_rage_fistream_open_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001580_rage_fiassetmanager_open_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001622_rage_fistream_read_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001623_rage_fiassetmanager_findextensioninpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001627_rage_fiassetmanager_exists_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001632_rage_fiassetmanager_pushfolder_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001633_rage_fiassetmanager_popfolder(ph: *mut Phantom); }
extern "C" { pub fn f_001637_rage_fiassetmanager_enumfiles_char_const_ptr_void_ptr_rage_fifin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_001639_rage_fiassetmanager_removeextensionfrompath_char_ptr_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001656_rage_fibasetokenizer_init_char_const_ptr_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001667_rage_fiasciitokenizer_getint_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001669_rage_fiasciitokenizer_getfloat_bool(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_001673_rage_fibasetokenizer_gettoken_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001680_rage_fibasetokenizer_gettokentochar_char_ptr_int_char_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_001682_rage_fibasetokenizer_skiptoendofline(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001684_rage_fibasetokenizer_putstrline_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001736_rage_fitokenizer_fitokenizer_char_const_ptr_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002018_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002019_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002020_void_std_2_sort_heap_abi_nqn220108_std_2_classicalgpolicy_bool_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002021_rage_atbinarymap_int_rage_atfinalhashstring_datapair_ptr_std_2_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002022_std_2_pair_rage_atbinarymap_int_rage_atfinalhashstring_datapair(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002023_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002027_rage_invsqrtf_fast_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_002030_rage_vec_v4float16streampack_float_vector_4_ptr_float_vector_4_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002031_rage_mthrandom_mthrandom_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002032_rage_mthrandom_setfullseed_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002033_rage_mthrandom_getgaussian_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_002044_rage_pgrscbuilder_computeleafsize_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002092_rage_datresource_getfixup_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002111_rage_parmanager_parmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002112_rage_parmanager_initialize_rage_parsettings_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002128_rage_parmanager_uninitialize(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002129_rage_atmapentry_rage_atnonfinalhashstring_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002131_rage_atnonfinalhashstring_atnonfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002132_rage_parmanager_dtorparmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002147_rage_parmanager_loadtree_char_const_ptr_char_const_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002158_rage_parinstancevisitor_visittoplevelstructure_rage_parparsables(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002162_rage_parmanager_loadfromstructure_rage_fistream_ptr_rage_parstru(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002165_rage_parmanager_createandloadanytype_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_002174_rage_parelement_findattributestringvalue_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_002175_rage_partree_dtorpartree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002203_rage_parelement_findattributeboolvalue_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002205_rage_parelement_findattributedoublevalue_char_const_ptr_double_b(ph: *mut Phantom, a0: i64, a1: i64, a2: f64, a3: i32) -> f64; }
extern "C" { pub fn f_002206_rage_parelement_findattributeint64value_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002294_rage_atarray_rage_psorscenumschemavaluedata_0_unsigned_short_gro(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002298_comparesourceaddr_rage_psorscbuilderinstance_ptr_rage_psorscbuil(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002311_rage_atarray_rage_psoenumtableentry_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
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
extern "C" { pub fn f_002446_rage_parcgustructure_addoneextraattribute_rage_parattributelist(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002447_rage_parinstancevisitor_parinstancevisitor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002593_rage_parsettings_createstandardsettings(ph: *mut Phantom); }
extern "C" { pub fn f_002690_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002692_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002694_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002695_rage_atbinarymap_rage_pardelegateholderbase_ptr_unsigned_int_dat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002696_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002697_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_002721_rage_conststring_dtorconststring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002743_rage_atdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002744_rage_atdatahash_unsigned_int_const_ptr_unsigned_long_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002790_rage_zlibinflater_zlibinflater(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002806_rage_sysservice_adddelegate_rage_atdelegator1_void_rage_sysservi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002808_rage_atdelegator1_void_rage_sysserviceevent_ptr_removedelegate_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002811_rage_syshangdetectsetcrashfunc_void_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002824_rage_syspipepacket_write_u32_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002840_rage_sysparam_get_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002852_rage_sysstack_printcapturedstacktrace_unsigned_long_const_ptr_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_002853_rage_sysstack_capturestacktrace_unsigned_long_ptr_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002862_rage_systhreadtype_addcurrentthreadtype_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_002881_unsigned_int_ref_rage_atmap_unsigned_int_unsigned_int_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002927_rage_sysmemstarttemp(ph: *mut Phantom); }
extern "C" { pub fn f_002928_rage_sysmemendtemp(ph: *mut Phantom); }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_002939_rage_sysmemmanager_sysmemmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002943_rage_sysmempoolallocator_dtorsysmempoolallocator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002945_rage_sysmempoolallocator_safeallocate_unsigned_long_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002946_rage_sysmempoolallocator_allocate_unsigned_long_unsigned_long_bo(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002947_rage_sysmempoolallocator_safefree_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002948_rage_sysmempoolallocator_free_void_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003085_rage_systaskparameters_systaskparameters(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003088_rage_systaskmanager_addscheduler_char_const_ptr_int_int_rage_sys(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_003090_rage_systaskmanager_create_char_const_ptr_void_ptr_rage_systaskp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_003094_rage_systaskmanager_waitmultiple_int_rage_systaskhandle_ptr_cons(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_003097_char_ptr_rage_formatf_char_16ul_char_ref_16ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003111_rage_sysdependencyscheduler_insertinternal_rage_sysdependency_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003136_rage_geomspheres_testspheretotriangle_rage_vec4v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_003139_rage_geomdistances_distancelinetopoint_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_003140_rage_geomtvalues_findtvalueopensegtopoint_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_003147_rage_geomspheres_testspheretoseg_rage_vec3v_const_ref_rage_scala(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_003152_rage_geomboxes_testsegmenttobox_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64) -> i32; }
extern "C" { pub fn f_003154_rage_geomboxes_testspheretobox_rage_vec3v_const_ref_rage_scalarv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_003156_rage_geom2d_intersectpointflat_float_float_float_float_float_flo(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: f32) -> i32; }
extern "C" { pub fn f_003157_rage_geom2d_sweeppolygon_rage_vec2f_ptr_rage_vec2f_ptr_int_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_003159_rage_matrix34_fromquaternion_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003160_rage_matrix34_toquaternion_rage_quaternion_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003161_rage_quaternion_frommatrix34_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003170_rage_quaternion_slerp_float_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003172_rage_quaternion_twistangle_rage_vector3_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_003173_rage_vector2_vector2_rage_vector3_const_ref_rage_vector2_evector(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_003175_rage_vector3_vector3_rage_vector2_const_ref_rage_vector2_evector(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003177_rage_vector3_rotateaboutaxis_float_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_003179_rage_atpoolbase_init_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003182_rage_atpoolbase_releasemem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003186_rage_crframe_crframe_rage_crframedata_const_ref_unsigned_char_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_003194_rage_vec_v4arccos_float_vector_4(ph: *mut Phantom, a0: u128) -> u128; }
extern "C" { pub fn f_003221_rage_crframe_dtorcrframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003258_rage_cranimation_gettrack_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003296_rage_atarray_unsigned_short_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003313_rage_crframedata_crframedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003314_rage_crframedata_dtorcrframedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003315_rage_crframedata_init_unsigned_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_003599_rage_crcreaturecomponentskeleton_setsuppressreset_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003809_rage_crframedata_adddof_unsigned_char_unsigned_short_unsigned_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_003825_rage_fwanimdirector_startupdateprerender_float_rage_crmtmotiontr(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32); }
extern "C" { pub fn f_003831_rage_fwanimdirectorcomponentsyncedscene_isplayingsyncedscene_con(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003835_rage_fwanimdirector_getmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003838_rage_fwclipset_getclip_rage_fwmvtemplateid_rage_fwmvclipidtype_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_003840_rage_fwclipsetmanager_getclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003857_rage_fwanimdirector_waitforprerenderupdatetocomplete(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003878_rage_fwanimhelpers_getmovertracktranslationdiff_rage_crclip_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003880_rage_fwanimhelpers_getmovertrackrotation_rage_crclip_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003882_rage_fwanimhelpers_getmovertrackrotationdiff_rage_crclip_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003895_rage_fwanimmanager_getclipifexistsbyname_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003896_rage_fwanimmanager_getclipifexistsbydictindex_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_003904_rage_fwanimmanager_findslot_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003905_rage_fwanimmanager_findslotfromhashkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003941_void_rage_atplaceptrcreator_void_ptr_rage_fwmemorygroupmetadata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003948_void_rage_parcgudestructor_rage_fwmemorysituation_rage_parparsab(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003968_rage_fwclipset_getclipdictionaryindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003974_rage_atarray_rage_atbinarymap_rage_fwclipitem_ptr_rage_fwmvtempl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004038_rage_fwclipsetrequesthelper_fwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004105_rage_fwmove_fwmove_rage_fwentity_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004280_rage_fwposematcher_fwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004281_rage_fwposematcher_dtorfwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004576_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_004577_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004578_void_std_2_sort_heap_abi_nqn220108_std_2_classicalgpolicy_bool_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004579_rage_atbinarymap_rage_fwclipitem_ptr_rage_fwmvtemplateid_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_004580_std_2_pair_rage_atbinarymap_rage_fwclipitem_ptr_rage_fwmvtemplat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004581_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_004603_rage_fwextensionlist_add_rage_fwextension_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004619_rage_fwarchetype_getdrawable_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004620_rage_fwarchetype_getfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004623_rage_fwarchetype_createdynamiccomponentifmissing(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004631_rage_fwarchetypemanager_getarchetypename_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004632_rage_fwarchetypemanager_lookupmodelid_rage_fwarchetype_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004633_rage_fwarchetypemanager_getarchetype_rage_atnonfinalhashstring_u(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004634_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004635_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_rag(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004648_rage_fwattachmententityextension_isattachstatebasicderived_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004651_rage_fwattachmententityextension_setparentattachment_rage_fwenti(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004656_rage_fwattachmententityextension_getconstrainthandle_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_004658_rage_fwattachmententityextension_debugsetinvocationdata_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_004661_rage_fwdynamicentitycomponent_fwdynamicentitycomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004663_rage_fwdynamicentitycomponent_createattachmentextensionifmissing(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004670_rage_fwentity_onactivate_rage_phinst_ptr_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004671_rage_fwentity_ondeactivate_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004674_rage_fwentity_setcurrentphysicsinstinternal_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004679_rage_fwentity_getskeletonsize_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004680_rage_fwentity_printskeletonsummary_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004681_rage_fwentity_waitforanyactiveanimupdatetocomplete_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004683_rage_fwentity_getaabb_rage_spdaabb_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004693_rage_fwsceneupdate_isinsceneupdate_rage_fwentity_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004694_rage_fwsceneupdate_addtosceneupdate_rage_fwentity_ref_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004723_rage_fwwaypointrecordingroute_uint8tosidedist_unsigned_char(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004740_rage_fwdecoratorextension_existson_rage_fwextensiblebase_ref_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_004741_rage_fwdecoratorextension_removefrom_rage_fwextensiblebase_ref_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_004744_rage_fwdecoratorextension_fwdecoratorextension_rage_fwextensible(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004747_rage_fwdecoratorextension_update_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_004750_rage_fwdecoratorextension_update_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_004767_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004792_rage_fwpickermanager_getselectedentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004816_rage_fwvectormap_drawline_rage_vector3_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_004819_rage_fwvectormap_drawmarker_rage_vector3_const_ref_rage_color32(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_004820_rage_fwvectormap_drawcircle_rage_vector3_const_ref_float_rage_co(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_004825_rage_fwvectormap_draw3dboundingboxonvmap_rage_spdaabb_const_ref(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004921_rage_dldrawcommandbuffer_registercommand_int_void_ptr_rage_dlcmd(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_004926_rage_dldrawcommandbuffer_allocatepagedmemory_rage_edrawlistpaget(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_004935_rage_dlcmdbase_op_new_unsigned_long_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005007_rage_dldrawlistmgr_addarchetypereference_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005016_rage_dldrawlistmgr_adjustvehiclehd_bool_bool_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_005018_rage_dldrawlistmgr_setuplodwidgets_rage_bkgroup_ref_int_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_005196_rage_ropeinstance_setphysinstflags_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005206_rage_ropeinstance_attachobjects_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: i32, a11: i64, a12: i64, a13: i64, a14: i64); }
extern "C" { pub fn f_005208_rage_ropeinstance_detachfromobject_rage_phinst_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005209_rage_ropeinstance_setnewuniqueid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005213_rage_ropemanager_remove_rage_ropeinstance_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005225_rage_ropemanager_add_rage_vec3v_const_ref_rage_vec3v_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32, a11: f32, a12: i32, a13: i32) -> i64; }
extern "C" { pub fn f_005394_rage_fwmapdatastore_setstreamable_rage_strlocalindex_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_005395_rage_fwmapdatastore_saferemove_rage_strlocalindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005400_rage_fwmapdatastore_isscriptmanaged_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005461_rage_fwboxstreamer_hasloadedaboutpos_rage_vec3v_const_ref_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_005462_rage_fwboxstreamer_hasloadedwithinaabb_rage_spdaabb_const_ref_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_005513_rage_fwtxdstore_setcurrenttxd_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005514_rage_fwtxdstore_popcurrenttxd(ph: *mut Phantom); }
extern "C" { pub fn f_005648_rage_fwpsostoreloader_load_char_const_ptr_rage_fwpsostoreloadins(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_005650_rage_fwpsostoreloader_loaddataintoobjectcore_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005651_rage_fwpsostoreloader_makesimplefileloadercore(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005678_rage_fwstaticboundsstore_getdummybounddata_rage_strlocalindex_sh(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_005707_rage_fwtxdstore_pushcurrenttxd(ph: *mut Phantom); }
extern "C" { pub fn f_005730_rage_fwinteriorlocation_makeinvalid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005732_rage_fwinteriorlocation_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005801_rage_fwsearch_fwsearch_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005802_rage_fwsearch_dtorfwsearch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005915_rage_aitask_aitask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005919_rage_aitask_setstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005921_rage_aitask_findparenttaskoftype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005922_rage_aitask_findsubtaskoftype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005926_rage_aitask_setnewtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005928_rage_aitaskmanager_aitaskmanager_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_005931_rage_aitaskmanager_process_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_005935_rage_aitaskmanager_aborttasks(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005940_rage_aitasktree_aitasktree_rage_fwentity_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_005949_rage_aitasktree_findtaskbytypeactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005950_rage_aitasktree_findtaskbytypewithpriority_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_005953_rage_aitasktree_setallowdelete_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005954_rage_tdynamicobject_setflagsfornewobject(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006056_rage_strstreaminginfomanager_removeobject_rage_strindex_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006072_rage_strstreaminginfomanager_getobjectname_rage_strindex_char_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_006075_rage_strstreaminginfo_computevirtualsize_rage_strindex_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006076_rage_strstreaminginfo_computephysicalsize_rage_strindex_bool_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006084_rage_strstreaminginfomanager_requestobject_rage_strindex_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006088_rage_atarray_rage_atnonfinalhashstring_0_unsigned_short_grow_uns(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006142_rage_strstreaminginfomanager_isobjectreadytodelete_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006143_rage_strstreaminginfoiterator_strstreaminginfoiterator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006167_rage_strstreaminginfomanager_isobjectinimage_rage_strindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006175_rage_strstreaminginfomanager_clearrequiredflag_rage_strindex_uns(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006179_rage_strstreaminginfomanager_getobjectanddependenciessizes_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_006225_rage_atvarstring_atvarstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_006254_rage_strrequest_request_rage_strlocalindex_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_006255_rage_strrequest_hasloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006256_rage_strrequest_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006320_rage_tccycle_load_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006332_rage_tckeyframeuncompressed_setdefault(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006343_rage_tcmanager_findmodifierindex_rage_atnonfinalhashstring_char(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006387_rage_tcmanager_calctimesampleinfo_unsigned_int_unsigned_int_unsi(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_006390_rage_tcmanager_addregionbox_rage_spdaabb_const_ref_unsigned_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006400_rage_tcmodifier_createandlockvarmap_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006402_rage_tcmodifier_unlockvarmap_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006405_rage_tcmodifier_getmoddataunsafe_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006412_rage_aitask_debug_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006518_rage_audsound_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006602_rage_audexternalstreamsound_initstreamplayer_rage_audreferencedr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_006715_rage_audsound_prepareandplay_rage_audwaveslot_ptr_bool_int_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_006717_rage_audsound_stopandforget_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006881_rage_audmetadatamanager_getobjectmetadatareffromhash_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006938_rage_audcurve_audcurve(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006941_rage_audcurve_calculatevalue_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_007025_rage_atmapentry_unsigned_int_unsigned_short_const_ref_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007036_rage_audmetadatamanager_getobject_verify_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007047_rage_audsoundfactory_getmetadataptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007064_rage_audsimplesmootherdiscrete_init_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32); }
extern "C" { pub fn f_007078_rage_audsoundset_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007079_rage_audsoundset_find_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007080_rage_audsoundset_find_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007084_rage_audtracker_audtracker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007085_rage_audtracker_dtoraudtracker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007096_rage_audrequestedsettings_setposition_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007098_rage_audrequestedsettings_setvolume_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007118_rage_audsmoother_audsmoother(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007121_rage_audsmoother_setrates_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_007124_rage_audsmoother_calculatevalue_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> f32; }
extern "C" { pub fn f_007126_rage_audsimplesmoother_audsimplesmoother(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007127_rage_audsimplesmoother_init_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_007131_rage_audsimplesmootherdiscrete_audsimplesmootherdiscrete(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007134_rage_audfluctuatorprobbased_init_float_float_float_float_float_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: f32); }
extern "C" { pub fn f_007324_rage_audpcmvariableblock_findvariableaddress_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007396_rage_audentity_audentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007407_rage_audentity_createsound_persistentreference_rage_audmetadatar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007408_rage_audentity_createsound_persistentreference_char_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007409_rage_audentity_createsound_persistentreference_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007411_rage_audentity_addbatchedsoundrequest_rage_audmetadataref_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007412_rage_audentity_createandplaysound_unsigned_int_rage_audsoundinit(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007416_rage_audentity_createandplaysound_persistent_unsigned_int_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007417_rage_audentity_createandplaysound_persistent_rage_audmetadataref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007418_rage_audentity_stopandforgetsoundsprivate_rage_audsound_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i32); }
extern "C" { pub fn f_007419_rage_vfxlistitem_vfxlistitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007420_rage_vfxlist_vfxlist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007421_rage_vfxlist_additem_rage_vfxlistitem_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007422_rage_vfxlist_removeitem_rage_vfxlistitem_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007423_rage_vfxlist_removehead(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007424_rage_vfxlist_removeall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007426_rage_vfxbrokenfraginfo_dtorvfxbrokenfraginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007427_rage_vfxutil_outputfxbucketmemorycurrent_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007428_rage_vfxutil_outputfxbucketmemorytotal_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007429_rage_vfxutil_inittxd_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007430_rage_vfxutil_shutdowntxd_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007444_rage_decalbucket_reapplyasstaticdecal_rage_decalinst_ptr_void_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_007445_rage_decalmanager_addsettings_rage_fwentity_ptr_rage_decaluserse(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_007457_rage_decalmanager_wash_float_rage_fwentity_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32); }
extern "C" { pub fn f_007458_rage_decaldebug_addstatswidgets_rage_bkbank_ptr_char_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007463_rage_decalmanager_createdx11buffers_rage_decalmanager_reinitarg(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007464_rage_decalmanager_destroydx11buffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007465_rage_decalmanager_initsettings_int_rage_decaltypesettings_ptr_in(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_007467_rage_decalmanager_initshaders(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007474_rage_decalmanager_shutdown_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007478_rage_decalmanager_updatestartasync_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007479_rage_decalmanager_remove_rage_fwentity_const_ptr_int_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_007480_rage_decalmanager_render_rage_decalrenderpass_rage_decalrenderpa(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_007494_rage_decalmanager_washinrange_float_rage_vec3v_const_ref_rage_sc(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_007497_rage_decalmanager_remove_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007499_rage_decalmanager_removeinrange_rage_vec3v_const_ref_rage_scalar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007501_rage_decalmanager_updateattachentity_rage_fwentity_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007502_rage_decalmanager_ispointinliquid_unsigned_int_signed_char_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32) -> i32; }
extern "C" { pub fn f_007503_rage_decalmanager_findclosest_unsigned_int_signed_char_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: f32, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_007504_rage_decalmanager_findfirstbucket_unsigned_int_rage_fwentity_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_007508_rage_decalinstsettings_decalinstsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007509_rage_decalbucketsettings_decalbucketsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007510_rage_decalpipelinesettings_decalpipelinesettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007511_rage_decallodsettings_decallodsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007512_rage_decalsettingsmanager_findrendersettinginfo_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_007513_rage_decalsettingsmanager_getrendersettingsindex_unsigned_int_un(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_007514_rage_decalsettingsmanager_patchdiffusemap_unsigned_int_rage_grct(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_007516_rage_ptfxassetstore_outputloadedassets(ph: *mut Phantom); }
extern "C" { pub fn f_007525_rage_atarray_rage_ptfxattachinfo_0_unsigned_short_destruct_rage(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007527_rage_ptfxattach_detach_rage_ptfxattachinfo_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007528_rage_ptfxattach_attach_rage_ptxeffectinst_ptr_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_007529_rage_ptfxattach_find_rage_ptxeffectinst_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007530_rage_ptfxflags_clearuserreservedflag_rage_ptxeffectinst_ptr_rage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007531_rage_ptfxattach_getisattachedto_rage_ptxeffectinst_ptr_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007532_rage_ptfxattach_getattachentity_rage_ptxeffectinst_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007537_rage_ptfxmanager_removeptfx_rage_ptxeffectinst_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007538_rage_ptfxreg_returnregfxinsttopool_rage_ptfxreginst_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007553_rage_atarray_rage_ptfxhistoryinfo_0_unsigned_short_destruct_rage(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007554_rage_ptfxhistory_add_rage_atnonfinalhashstring_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_007555_rage_ptfxhistory_check_rage_atnonfinalhashstring_unsigned_int_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32) -> i32; }
extern "C" { pub fn f_007558_rage_ptfxreg_register_void_const_ptr_int_bool_rage_ptxeffectrule(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_007559_rage_ptfxmanager_gettriggeredinst_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007562_rage_ptfxmanager_getregisteredinst_void_const_ptr_int_bool_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_007565_rage_ptfxreg_find_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007566_rage_ptfxreg_unregister_void_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007570_rage_ptfxwrapper_setcolourtint_rage_ptxeffectinst_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007580_rage_ptfxscript_setremovewhendestroyed_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_007581_rage_ptfxscript_doesexist_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_007587_rage_ptfxwrapper_setalphatint_rage_ptxeffectinst_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007591_rage_ptfxwrapper_geteffectrule_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007630_rage_fwvehicleglasswindowdata_getwindow_rage_fwvehicleglasswindo(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007635_rage_atmapentry_void_ptr_unsigned_int_const_ref_rage_atmap_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007658_rage_fwptrlistsinglelink_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007676_rage_fwangle_limitradianangleforpitch_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_007678_rage_fwangle_lerptowards_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_007692_rage_fwgeom_disttolinesqr_rage_vector3_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_007693_rage_fwgeom_distalongline2d_float_float_float_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32) -> f32; }
extern "C" { pub fn f_007704_rage_fwaudioentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007756_rage_netinterface_flushalllogfiles_bool(ph: *mut Phantom); }
extern "C" { pub fn f_007759_rage_netinterface_getnumphysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007760_rage_netinterface_getnumremotephysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007762_rage_netinterface_getallphysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007763_rage_netinterface_getremotephysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007771_rage_netobjectmgrbase_getnetworkobject_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007803_rage_networklogutils_writelogevent_rage_netlogginginterface_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007805_rage_netarraymanager_getarrayhandler_unsigned_short_rage_netarra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_007848_rage_neteventmgr_checkforspaceinpool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007849_rage_neteventmgr_prepareevent_rage_netgameevent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007949_rage_netgameevent_netgameevent_unsigned_short_bool_bool_rage_net(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_007979_rage_netserialiserutils_getadjustedworldextents_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007980_rage_netserialiserutils_getrealmapcoords_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007981_rage_netserialiserutils_getadjustedmapcoords_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007985_rage_csyncdatabase_serialiseinteger_int_ref_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_007989_rage_csyncdatabase_serialiseunsigned_unsigned_int_ref_int_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_007991_rage_csyncdatabase_serialisepackedfloat_float_ref_float_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_007993_rage_csyncdatabase_serialiseposition_rage_vector3_ref_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_007995_rage_matrix34_toeulersxyz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008000_rage_netobjecttypepredicate_setobjecttype_unsigned_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008074_rage_netobject_ispendingownerchange_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008075_rage_netobject_islocalflagset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008125_rage_netobjectidmgr_hasfreeobjectidsavailable_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008187_rage_netobjectmgrbase_gettotalnumobjects_bool_ptr_rage_netobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008246_rage_netplayer_getrlgamerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008321_rage_netsynctree_forcesendofnodedata_unsigned_int_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_008606_bool_rage_datbitbuffer_writeint_signed_char_signed_char_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008682_rage_fwbasepool_fwbasepool_int_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_008683_rage_fwconfigmanager_getsizeofpool_unsigned_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008688_rage_fwbasepool_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008689_rage_fwscriptguid_createguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008691_rage_fwscriptguid_deleteguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008693_rage_fwscriptguid_getbasefromguid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_008764_bool_rage_datbitbuffer_readint_int_int_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_008768_rage_scripthandlermgr_removescriptresource_unsigned_short_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_008903_rage_fwmetadatastore_saferemove_rage_strlocalindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008918_rage_fwtimer_startuserpause_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008945_rage_fwbasepool_forall_bool_ptr_void_ptr_void_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008953_rage_fwknownrefholder_fwknownrefholder_void_ptr_ptr_rage_fwknown(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_009207_rage_crskeletondata_findbonedata_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009211_rage_crskeletondata_computeobjecttransform_int_rage_mat34v_ref_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009224_rage_crskeleton_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009225_rage_crskeleton_partialreset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009226_rage_crskeleton_partialupdate_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009231_rage_crskeleton_partialinverseupdate_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009233_rage_crskeleton_getglobalmtx_unsigned_int_rage_mat34v_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009234_rage_crskeleton_setglobalmtx_unsigned_int_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009265_rage_crjointdata_converttoeulers_rage_crbonedata_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_009266_rage_crjointdata_gettranslationlimits_rage_crbonedata_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_009283_rage_grmmodel_getgeometry_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009291_rage_grmshader_begindraw_rage_grmshader_edrawtype_bool_rage_grce(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_009293_rage_grmshader_bind_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009294_rage_grmshader_unbind_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009295_rage_grmshader_enddraw_const(ph: *mut Phantom); }
extern "C" { pub fn f_009308_rage_grmmodel_setmodelcullback_rage_grccullstatus_ptr_rage_spdaa(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009312_rage_grmshader_setforcedtechniquegroupid_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009314_pixend(ph: *mut Phantom); }
extern "C" { pub fn f_009317_rage_grmshader_load_char_const_ptr_rage_fitokenizer_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_009321_rage_grmmodelfactory_builddeclarator_rage_grcfvf_const_ptr_rage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009327_rage_grmshadergroup_lookupvar_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009333_rage_grmshadergroup_setvarcommon_rage_grmshadergroupvar_void_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_009334_rage_grmshadergroup_setvar_rage_grmshadergroupvar_rage_grctextur(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009417_pixbegin_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009428_rage_pftimebarmgr_init_char_const_ptr_int_float_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_009429_rage_pftimebarmgr_frameinit_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009445_rage_pfdrawarrow_rage_vector3_const_ref_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009447_rage_pfdrawitembase_beginmanager(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009448_rage_pfdrawitembase_end(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009499_rage_grcfvf_setposchannel_bool_rage_grcfvf_grcdatasize_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009500_rage_grcfvf_setnormalchannel_bool_rage_grcfvf_grcdatasize(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009501_rage_grcfvf_setdiffusechannel_bool_rage_grcfvf_grcdatasize(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009506_rage_grcfvf_settexturechannel_int_bool_rage_grcfvf_grcdatasize(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009511_rage_grceffect_dtorgrceffect(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009518_rage_grcimage_create_unsigned_int_unsigned_int_unsigned_int_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_009528_rage_grceffect_create_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009529_rage_grcstateblock_createsamplerstate_rage_grcsamplerstatedesc_c(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009533_rage_grctexturefactory_createparams_createparams_unsigned_short(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009539_rage_grceffect_lookuptechnique_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009540_rage_grceffect_lookupvar_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009542_rage_grcstateblock_createblendstate_rage_grcblendstatedesc_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009544_rage_grcdrawsinglequadf_float_float_float_float_float_float_floa(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: i64); }
extern "C" { pub fn f_009545_rage_grcstateblock_setblendstate_rage_grcstateblock_grcblendstat(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009548_rage_grcdevice_drawvertices_void_const_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_009555_rage_grcstateblock_setrasterizerstate_rage_grcstateblock_grcrast(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009556_rage_grcbatcher_reserve_unsigned_int_unsigned_long(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_009559_rage_grcdevice_clear_bool_rage_color32_bool_float_bool_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_009560_rage_grcstateblock_setdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009561_rage_grcbegin_rage_grcdrawmode_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009562_rage_grcvertex_float_float_float_float_float_float_rage_color32(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: f32, a8: f32); }
extern "C" { pub fn f_009563_rage_grcend(ph: *mut Phantom); }
extern "C" { pub fn f_009587_rage_grclightstate_setenabled_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_009588_rage_grcimage_dtorgrcimage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009592_rage_grcimage_getpixelcolor32_int_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_009596_rage_grcimage_setpixel_int_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009603_rage_grcviewport_setworldidentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009607_rage_grcdevice_setwindow_rage_grcwindow_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009608_rage_grcviewport_grcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009609_rage_grcviewport_resetwindow(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009610_rage_grcviewport_setinvertzinprojectionmatrix_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009612_rage_grcviewport_setwindow_int_int_int_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: f32, a6: f32); }
extern "C" { pub fn f_009613_rage_grcviewport_dtorgrcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009614_rage_grcviewport_setcurrent_rage_grcviewport_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009615_rage_grceffect_lookupglobalvar_char_const_ptr_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009620_rage_grcviewport_setprojection_rage_mat44v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009623_rage_grcviewport_setnearfarclip_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_009624_rage_grcviewport_setworldmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009634_rage_grcviewport_setcurrentworldidentity(ph: *mut Phantom); }
extern "C" { pub fn f_009635_rage_grcviewport_setcurrentworldmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009636_rage_grcviewport_setcameramtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009637_rage_grcviewport_setcameraidentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009638_rage_grcviewport_isspherevisible_rage_vec4v_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009641_rage_grcviewport_isaabbvisible_float_vector_4_float_vector_4_rag(ph: *mut Phantom, a0: u128, a1: u128, a2: i64) -> i32; }
extern "C" { pub fn f_009645_rage_grcviewport_getunclippedwindow_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009648_rage_grcviewport_reversetransform_float_float_rage_vec3v_ref_rag(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i64); }
extern "C" { pub fn f_009649_rage_grcworldidentity(ph: *mut Phantom); }
extern "C" { pub fn f_009650_rage_grcbindtexture_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009653_rage_grcviewport_op_rage_grcviewport_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009675_rage_grcstateblock_createdepthstencilstate_rage_grcdepthstencils(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009700_rage_grcimage_savejpeg_rage_fistream_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009715_rage_grcvertexbuffer_createwithdata_int_rage_grcfvf_const_ref_ra(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_009716_rage_grcdevice_createvertexdeclaration_rage_grcvertexelement_con(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009719_rage_grcdevice_destroyvertexdeclaration_rage_grcvertexdeclaratio(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009723_rage_grcdevice_setvertexdeclaration_rage_grcvertexdeclaration_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009724_rage_grcdevice_setstreamsource_unsigned_int_rage_grcvertexbuffer(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_009725_rage_grcdevice_drawprimitive_rage_grcdrawmode_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_009727_rage_grcdevice_beginvertices_rage_grcdrawmode_unsigned_int_unsig(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_009733_rage_grcdrawline_rage_vector3_const_ref_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009739_rage_grcdrawellipsoid_rage_vector3_const_ref_rage_matrix34_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_009747_rage_grcdrawtexturedpolygon_rage_vector3_const_ptr_int_rage_vect(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_009748_rage_grcdrawpolygon_rage_vector3_const_ptr_int_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009749_rage_grcdrawtexturedpolygon_rage_vector3_const_ptr_int_rage_vect(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_009750_rage_grcdrawpolygon_rage_vector3_const_ptr_int_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009754_rage_grclighting_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_009757_rage_grcdrawsolidbox_rage_vector3_const_ref_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009766_rage_grcvertexbuffer_create_int_rage_grcfvf_const_ref_rage_grcbu(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_009769_rage_grcvertexbuffereditor_grcvertexbuffereditor_rage_grcvertexb(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_009776_rage_grcbuffereditor_getnormal_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009777_rage_grcbuffereditor_getposition_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009778_rage_grcbuffereditor_getuv_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_009779_rage_grcbuffereditor_getdiffuse_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009780_rage_grcbuffereditor_getblendindices_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009787_rage_grcindexbuffer_createwithdata_int_rage_grcbuffercreatetype(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_009788_rage_grcindexbuffer_lockro_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009789_rage_grcindexbuffer_unlockwo_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009798_rage_grcimage_loadjpeg_char_const_ptr_rage_grcimage_ptr_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009814_rage_grceffect_setvar_rage_grcinstancedata_ref_rage_grceffectvar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009815_rage_grcinstancedata_addwidgets_withload_rage_bkbank_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_009858_rage_grcstateblock_createrasterizerstate_rage_grcrasterizerstate(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009859_rage_grceffect_setglobalfloatcommon_int_float_ptr_int_rage_grcef(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_009863_rage_grceffect_findtechniquegroupid_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009867_rage_grcinstancedata_dtorgrcinstancedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009868_rage_grceffect_lookupvarbyhash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009869_rage_grceffect_getlocalcommon_rage_grcinstancedata_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_009872_rage_grceffect_setglobalvar_rage_grceffectglobalvar_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009873_rage_grceffect_setglobalvar_rage_grceffectglobalvar_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009875_rage_grceffect_setlocalcommon_rage_grcinstancedata_ref_rage_grce(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_009876_rage_grceffect_setvar_rage_grcinstancedata_ref_rage_grceffectvar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009880_rage_grceffect_getvar_rage_grcinstancedata_const_ref_rage_grceff(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009881_rage_grceffect_setglobalvar_rage_grceffectglobalvar_rage_grctext(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009886_rage_grceffect_pushsamplerstate_rage_grcinstancedata_ref_rage_gr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009887_rage_grceffect_popsamplerstate_rage_grcinstancedata_ref_rage_grc(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009893_rage_grcstateblock_setdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009894_rage_grcstateblock_setblendstate_rage_grcstateblock_grcblendstat(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_009898_rage_grcstateblock_destroyblendstate_rage_grcstateblock_grcblend(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009900_rage_grcstateblock_destroydepthstencilstate_rage_grcstateblock_g(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009902_rage_grcstateblock_setwireframeoverride_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_009903_rage_grcstateblock_setstates_rage_grcstateblock_grcrasterizersta(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_009951_rage_grcvertexbufferd3d11_lockinternal_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009952_rage_grcvertexbufferd3d11_unlockinternal_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009965_rage_grccbuffer_getdataptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009966_rage_grccbuffer_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009986_rage_grcdebugdraw_adddebugoutputex_bool_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_009994_rage_grcdevice_clearstreamsource_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010003_rage_grcdevice_setscissor_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_010004_rage_grcdevice_setdepthbuffer_id3d11shaderresourceview_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010009_rage_grcdevice_setclipplaneenable_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010010_rage_grcdevice_setclipplane_int_rage_vec4v_const_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_010012_rage_grcdevice_endcreatevertices_void_const_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_010014_rage_grcdevice_setindices_rage_grcindexbuffer_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010021_rage_grcdevice_createocclusionquery_int(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_010022_rage_grcdevice_destroyocclusionquery_iunknown_ptr_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010023_rage_grcdevice_beginocclusionquery_iunknown_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010024_rage_grcdevice_endocclusionquery_iunknown_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010029_rage_grcdevice_drawindexedprimitive_rage_grcdrawmode_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_010031_rage_grcdevice_drawprimitive_rage_grcdrawmode_rage_grcvertexdecl(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010032_rage_grcdevice_drawinstancedprimitive_rage_grcdrawmode_int_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_010154_rage_batcherlock_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010161_rage_grcdebugdraw_startrecording_rage_grcdebugdraw_lingerbuffer(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_010163_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010164_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010168_rage_grcdebugdraw_cross_rage_vec2v_const_ref_float_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_010169_rage_grcdebugdraw_circle_rage_vec2v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010171_rage_grcdebugdraw_meter_rage_vec2v_const_ref_rage_vec2v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_010172_rage_grcdebugdraw_text_float_float_rage_color32_char_const_ptr_b(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i64, a4: i32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010173_rage_grcdebugdraw_metervalue_rage_vec2v_const_ref_rage_vec2v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010179_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010180_rage_grcdebugdraw_quad_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_010181_rage_grcdebugdraw_poly_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010183_rage_grcdebugdraw_cross_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010184_rage_grcdebugdraw_circle_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010188_rage_grcdebugdraw_boxaxisaligned_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010189_rage_grcdebugdraw_boxoriented_rage_vec3v_const_ref_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010193_rage_grcdebugdraw_capsule_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010194_rage_grcdebugdraw_axis_rage_mat34v_const_ref_float_bool_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_010195_rage_grcdebugdraw_arrow_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_010208_rage_grcdebugdraw_getdisableculling(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_010209_rage_grcdebugdraw_setdisableculling_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010214_rage_grcdebugdraw_textfontpush_rage_grcfont_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010215_rage_grcdebugdraw_textfontpop(ph: *mut Phantom); }
extern "C" { pub fn f_010216_rage_grcdebugdraw_text_float_float_rage_dd_epixelcoordspace_rage(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i32, a4: i64, a5: i32, a6: f32, a7: f32); }
extern "C" { pub fn f_010217_rage_grcdebugdraw_printtoscreencoors_char_const_ptr_int_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010220_rage_grcdebugdraw_adddebugoutput_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010224_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010225_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010489_rage_iovalue_setcurrentvalue_float_rage_iosource_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_010500_rage_iovalue_isdown_float_rage_iovalue_readoptions_const_ref_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010501_rage_iovalue_historyhelddown_unsigned_int_float_rage_iovalue_rea(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_010503_rage_iovalue_isup_float_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010505_rage_iovalue_wasdown_float_rage_iovalue_readoptions_const_ref_co(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010510_rage_iovalue_isreleasedafterhistoryhelddown_unsigned_int_float_r(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_010511_rage_iovalue_ispressed_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010512_rage_iovalue_historypressed_unsigned_int_unsigned_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_010514_rage_iovalue_isreleased_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010515_rage_iovalue_historyreleased_unsigned_int_unsigned_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_010518_rage_iovalue_getnorm_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010519_rage_iovalue_getunboundnorm_rage_iovalue_readoptions_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010520_rage_iovalue_getlastnorm_rage_iovalue_readoptions_const_ref_cons(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010521_rage_iovalue_getnorm01_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010732_rage_mnbvhsimpleinterface_mnbvhsimpleinterface_rage_atarray_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_010735_rage_mnbvhsimpleinterface_dtormnbvhsimpleinterface(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010747_rage_spdtransposedplaneset8_set_rage_grcviewport_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_010879_rage_bwmovie_create(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_010880_rage_bwmovie_destroy_rage_bwmovie_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010882_rage_sysmessagequeue_rage_bwmovie_sasyncdata_32_true_push_rage_b(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010884_rage_bwmovie_init(ph: *mut Phantom); }
extern "C" { pub fn f_010888_rage_bwmovie_setvolume_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_010889_rage_bwmovie_updatemovietime_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010891_rage_bwmovie_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010893_rage_bwmovie_play(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010895_rage_bwmovie_begindraw_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_010896_rage_bwmovie_shaderenddraw_rage_bwmovie_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010897_rage_bwmovie_getmovietimereal_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_010899_rage_bwmovie_setmovietimereal_float(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_011155_void_rage_atplaceptrcreator_void_ptr_rage_slightdetails_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011734_rage_fraginst_getarticulatedbody(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011735_rage_fraginst_getarticulatedcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011740_rage_fraginst_setdisablebreakable_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011742_rage_fraginst_resetdisablebreakable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011743_rage_fraginst_isbreakingdisabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011744_rage_fraginst_isdamagedisabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011745_rage_fraginst_setresetmatrix_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011749_rage_fraginst_getgroupbroken_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011750_rage_fraginst_getchildbroken_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011751_rage_fraginst_getcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011752_rage_fragtype_getcomponentfromboneindex_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_011753_rage_fraginst_getcontrollingcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011754_rage_fragtype_getcontrollingcomponentfromboneindex_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_011755_rage_fraginst_getgroupfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011756_rage_fragtype_getgroupfromboneindex_int_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011759_rage_fraginst_breakoffabove_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_011760_rage_fraginst_breakoffabovegroup_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_011761_rage_fragcachemanager_cangetnewcacheentries_unsigned_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011763_rage_fraginst_deleteabove_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011764_rage_fraginst_deleteabovegroup_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011766_rage_fragcacheentry_setmass_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_011769_rage_fraginst_zeroskeletonmatricesbygroup_rage_crskeleton_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011772_rage_fraginst_restoreabove_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011773_rage_fraginst_restoreabovegroup_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011775_rage_fragmemstartcacheheapfunc_rage_fragcacheentry_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011776_rage_fragtype_getboneindexfromid_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011777_rage_fragmemendcacheheap(ph: *mut Phantom); }
extern "C" { pub fn f_011780_rage_fragcacheentry_getmass_rage_fraghierarchyinst_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_011791_rage_phcontactconstiterator_op(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011792_rage_fraginst_openlatchabove_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011793_rage_fraginst_closelatchabove_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011803_rage_phcollider_setcollidermatrixfrominstance(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011804_rage_fraginst_forcearticulatedcollidermode(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011805_rage_fragcacheentry_lazyarticulatedinit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011811_rage_fragcachemanager_releasecacheentry_rage_fragcacheentry_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011817_rage_fraginst_getlinkindexfromcomponent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011822_rage_matrix34_dot_rage_matrix34_const_ref_rage_matrix34_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011824_rage_fraginst_posearticulatedbodyfrombounds_signed_char_signed_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_011826_rage_fraginst_setmusclesfromskeleton_rage_crskeleton_const_ref_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_011879_rage_fragtype_hasmotionlimits_rage_crbonedata_const_ref_rage_crj(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_011884_rage_fragphysicslod_applydamping_rage_pharchetypedamp_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011885_rage_phiterator_dtorphiterator(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011964_rage_fragtype_getskeletondata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012021_rage_fragphysicslodgroup_getlodbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012037_rage_fragcacheentry_cloneboundparts(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012038_rage_fragcacheentry_decloneboundparts(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012050_rage_fragcacheallocator_externalallocate_unsigned_long_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_012157_rage_ptxgpushader_init_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012158_rage_ptxgpushader_settechnique_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012159_rage_ptxgpuupdatebaseshader_setheightmap_rage_grctexture_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012165_void_rage_atplaceptrcreator_void_ptr_rage_ptxgpudropemittersetti(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012170_rage_ptxgpudroprendersettings_ptxgpudroprendersettings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012171_rage_ptxgpudroprendersettings_interpolate_rage_ptxgpudroprenders(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_012172_rage_ptxgpudroprendersettings_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012178_rage_ptxgpudrop_renderdebug_rage_grcviewport_const_ref_rage_colo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012179_rage_ptxdatavolume_getpointstrength_rage_vec3v_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> f32; }
extern "C" { pub fn f_012180_rage_ptxdatavolume_ispointinside_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012181_rage_ptxeffectinst_dtorptxeffectinst(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012183_rage_ptxeffectinst_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012185_rage_ptxeffectinst_updatesetup_float_rage_grcviewport_const_ptr(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_012186_rage_ptxeffectinst_updatefinalize_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_012187_rage_ptxevolutionlist_getevolvedkeyframepropbyid_unsigned_int_co(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012188_rage_ptxevolutionlist_query_rage_ptxkeyframeprop_ptr_rage_scalar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_012189_rage_ptxeffectinst_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012191_rage_ptxeffectinst_finish_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012193_rage_ptxeffectinst_calccullstate_rage_grcviewport_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_012196_rage_ptxeffectinst_setbasemtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012198_rage_ptxeffectinst_clearcollisionset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012199_rage_ptxeffectinst_setcollisionset_rage_ptxcollisionset_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012200_rage_ptxeffectinst_copycollisionset_rage_ptxcollisionset_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012201_rage_ptxeffectinst_setbasepos_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012202_rage_ptxeffectinst_setoffsetmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012203_rage_ptxeffectinst_setoffsetpos_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012205_rage_ptxeffectinst_setoffsetrot_rage_quatv_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012206_rage_ptxeffectinst_setevovaluefromindex_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_012207_rage_ptxeffectinst_setevovaluefromhash_rage_athashvalue_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_012208_rage_ptxeffectinst_getevovaluefromhash_rage_athashvalue(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_012209_rage_ptxeffectinst_setoverridedistsqrtocamera_float_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_012211_rage_ptxkeyframe_addentry_rage_scalarv_const_ref_rage_vec4v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012214_rage_ptxkeyframe_setdefn_rage_ptxkeyframedefn_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012216_rage_ptxkeyframe_getvalue_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_012227_rage_ptxemitterinst_updateparallel_rage_ptxeffectinst_ptr_rage_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_012243_rage_ptxshadertemplatelist_loadshader_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012251_rage_ptxkeyframe_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_012265_rage_ptxkeyframeentry_ptxkeyframeentry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012266_rage_ptxkeyframe_ptxkeyframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012271_rage_ptxkeyframe_gettimev_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_012273_rage_ptxkeyframe_copyentriesfrom_rage_ptxkeyframe_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012275_rage_ptxkeyframe_updatewidget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012276_rage_ptxkeyframe_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012279_rage_ptxkeyframeprop_ptxkeyframeprop_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012292_rage_ptxshadertemplatelist_setglobalvars(ph: *mut Phantom); }
extern "C" { pub fn f_012295_rage_ptxshadertemplateoverride_init_rage_grmshader_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012296_rage_ptxshadertemplateoverride_binddebugtechniqueoverride_rage_p(ph: *mut Phantom); }
extern "C" { pub fn f_012297_rage_ptxshadertemplateoverride_unbinddebugtechniqueoverride(ph: *mut Phantom); }
extern "C" { pub fn f_012298_rage_ptxshadertemplateoverride_disablealltechniques(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012299_rage_ptxshadertemplateoverride_enabletechnique_rage_grceffecttec(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012300_rage_ptxshadertemplateoverride_overridetechniquewith_rage_grceff(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_012328_rage_ptxmanager_createfxlistiterator(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012358_rage_ptxmanager_setupptxkeyframepropthreadid(ph: *mut Phantom); }
extern "C" { pub fn f_012359_rage_ptxcollisionset_init_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012360_rage_ptxcollisionset_addtri_rage_vec3v_const_ptr_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_012361_rage_ptxcollisionset_addquad_rage_vec3v_const_ptr_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_012373_rage_ptxmanager_geteffectrule_rage_atnonfinalhashstring_rage_ptx(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_012403_rage_ptxfxlist_findexternalreferencecount(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012407_rage_ptxmanager_registerbehaviour_char_const_ptr_rage_ptxbehavio(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012424_rage_ptxmanager_resetallimmediately(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012426_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_012427_rage_ptxmanager_draw_rage_ptxdrawlist_ptr_rage_ptxdrawinterface(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_012431_rage_ptxmanager_createdrawlist_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012436_rage_ptxbehaviour_ptxbehaviour_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012787_rage_atdelegate0_bool_op_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012843_rage_environmentcloth_getcloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012871_rage_environmentcloth_updatecloth_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_012923_rage_cpedsafezone_getpedsafeareamultiplier_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013050_rage_matrix34_transform3x3_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013051_rage_quaternion_transform_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013116_rage_bgglassmanager_setcontactfunctor_rage_atdelegate_void_rage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013155_rage_rsttestaabbexactedgelist_bool_rage_mat44v_const_ref_rage_sc(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i32, a13: i32, a14: i32); }
extern "C" { pub fn f_013882_rage_netipv4address_netipv4address(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014067_rage_nettimesync_gettime_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014068_rage_nettimesync_hasstarted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014316_rage_rlgamerinfo_getgamerhandle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014662_rage_rlmetric_dtorrlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014685_rage_rlmetric_rlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014764_char_ptr_rage_formatf_char_15ul_char_ref_15ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015751_rage_phmaterialmgr_findcombinedfrictionandelasticity_unsigned_lo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_015771_rage_computeaxisangleprecise_rage_quatv_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015774_rage_phmathinertia_getinverseinertiamatrix_rage_mat33v_const_ref(ph: *mut Phantom, a0: i64, a1: u128, a2: i64); }
extern "C" { pub fn f_015803_rage_phmanifold_removeallcontacts(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015812_rage_phconstraintbase_getinstancea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015813_rage_phconstraintbase_getinstanceb(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015831_rage_phmanifold_getlocaltoworldtransforma_rage_matrix34_ref_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015832_rage_phmanifold_getlocaltoworldtransformb_rage_matrix34_ref_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015840_rage_phcontact_computelocalpointbfromworld_rage_phmanifold_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015841_rage_phcontact_computelocalpointafromworld_rage_phmanifold_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015843_rage_phconstraintdistance_getworldposa(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015844_rage_phconstraintdistance_getworldposb(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015845_rage_phconstraintdistance_setlocalposa_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015846_rage_phcontact_computeworldpointbfromlocal_rage_phmanifold_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015847_rage_phcontact_computeworldpointafromlocal_rage_phmanifold_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015848_rage_phconstraintdistance_setlocalposb_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015872_rage_phcollider_drawtorque_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015882_rage_phconstraintmgr_insert_rage_phconstraintbase_params_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015883_rage_phconstraintmgr_insertandreturntemporarypointer_rage_phcons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_015890_rage_phconstraintmgr_remove_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015891_rage_phconstraintmgr_gettemporarypointer_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015893_rage_phmultireaderlocktoken_waitasreader(ph: *mut Phantom); }
extern "C" { pub fn f_015897_rage_phconstraintrotation_setworldaxis_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015944_rage_phcontact_computerelvelocity_rage_vec3v_const_ref_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_015951_rage_phmanifold_removecontactpoint_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_015956_rage_debugcollider_rage_phcollider_ptr_char_const_ptr_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015958_rage_phcollider_initinertia(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015963_rage_phcollider_setinertia_float_vector_4_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128, a2: u128); }
extern "C" { pub fn f_015965_rage_phcollider_setcollidermatrixfrominstancerigid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015966_rage_phsleep_reset_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015969_rage_phcollider_setreferenceframevelocity_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015970_rage_phcollider_setreferenceframeangularvelocity_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015977_rage_phcollider_setinstancematrixfromcollider(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015980_rage_phcollider_getlocalvelocityrigid_float_vector_4_const(ph: *mut Phantom, a0: i64, a1: i64, a2: u128); }
extern "C" { pub fn f_015981_rage_phcollider_applyimpulserigid_float_vector_4_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128, a2: u128); }
extern "C" { pub fn f_015986_rage_phcollider_applyforcecenterofmassimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015987_rage_phcollider_applytorqueimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015988_rage_phcollider_setvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015998_rage_phsimulator_deleteobject_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016005_rage_phsimulator_activateobject_int_rage_phcollider_ptr_rage_phi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016010_rage_phsimulator_getlastinstancematrix_rage_phinst_const_ptr_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016011_rage_phsimulator_setlastinstancematrix_rage_phinst_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016012_rage_phsimulator_addactiveobject_rage_phinst_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016013_rage_phsimulator_addinactiveobject_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016017_rage_phsimulator_deactivateobject_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016018_rage_phsimulator_activateobject_rage_phinst_ptr_rage_phcollider(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_016019_rage_phsimulator_deactivateobject_rage_phinst_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016045_rage_pharchetype_removeincludeflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016084_rage_phinst_setdtorcallback_void_ptr_rage_phinst_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016085_rage_phinst_setarchetypechangecallback_void_ptr_rage_phinst_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016086_rage_phinst_init_rage_pharchetype_ref_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016087_rage_phinst_setarchetype_rage_pharchetype_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016088_rage_phinst_setinstflag_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016164_rage_matrix34_dot_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016255_rage_pharticulatedbody_getangularvelocity_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016257_rage_pharticulatedbody_resetdownvel_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016264_rage_pharticulatedbody_resetpropagationvelocities(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016267_rage_pharticulatedbody_ensurevelocitiesfullypropagated(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016274_rage_phjoint_responsetounittorqueonaxis_rage_pharticulatedbody_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016275_rage_phjoint_responsetounitforceonaxis_rage_pharticulatedbody_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016287_rage_pharticulatedbody_calculateinertias(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016289_rage_phjoint1dof_applytorque_rage_pharticulatedbody_ptr_float_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: u128); }
extern "C" { pub fn f_016301_rage_pharticulatedcollider_setcollidermatrixfrominstanceart(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016307_rage_pharticulatedcollider_setlinkforcomponent_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016310_rage_pharticulatedcollider_updatecurrentmatrices(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016312_rage_phboundcomposite_updatebvh_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016317_rage_phboundcomposite_calculatecompositeextents_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016322_rage_pharticulatedcollider_setvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016323_rage_pharticulatedcollider_setangvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016324_rage_pharticulatedcollider_applyimpulseart_float_vector_4_float(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: i32); }
extern "C" { pub fn f_016326_rage_pharticulatedcollider_applyforcecenterofmassimp_float_vecto(ph: *mut Phantom, a0: i64, a1: u128, a2: u128); }
extern "C" { pub fn f_016328_rage_pharticulatedcollider_applytorqueimp_float_vector_4_float_v(ph: *mut Phantom, a0: i64, a1: u128, a2: u128); }
extern "C" { pub fn f_016332_rage_pharticulatedcollider_getinverseinertiamatrixart_rage_mat33(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016333_rage_pharticulatedcollider_getlocalvelocityart_float_vector_4_in(ph: *mut Phantom, a0: i64, a1: i64, a2: u128, a3: i32); }
extern "C" { pub fn f_016341_rage_phjoint_getchildlink_rage_pharticulatedbody_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016345_rage_phjoint3dof_setanglelimits_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_016351_rage_phjoint1dof_computecurrentangle_rage_pharticulatedbody_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016352_rage_phjoint1dof_computerotationalvelocity_rage_pharticulatedbod(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016353_rage_phjoint1dof_matchchildtoparentpositionandvelocity_rage_phar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016355_rage_phjoint1dof_setanglelimits_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32); }
extern "C" { pub fn f_016358_rage_phprismaticjoint_computecurrenttranslation_rage_pharticulat(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_016359_rage_phprismaticjoint_computevelocity_rage_pharticulatedbody_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_016361_rage_phprismaticjoint_setanglelimits_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_016367_rage_phbound_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016370_rage_phbound_setcgoffset_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016385_rage_phbound_istypecomposite_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_016391_rage_phboundbox_phboundbox_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016437_rage_phboundcapsule_setcapsuleradius_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016438_rage_phboundcapsule_setcapsulelength_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016470_rage_phboundcomposite_calccenterofbound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016471_rage_phboundcomposite_allocatetypeandincludeflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016494_rage_phbounddisc_setdiscradius_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016496_rage_phbounddisc_setmargin_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016515_rage_phboundgeometry_calculatepolynormals(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016516_rage_phboundgeometry_recomputeoctantmap(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016519_rage_phboundgeometry_calculateboundingbox_rage_vector3_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016594_rage_phverletcloth_detachvirtualbound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016599_rage_phverletcloth_attachvirtualboundcapsule_float_float_rage_ma(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_016600_rage_phverletcloth_createvirtualbound_int_rage_mat34v_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016636_rage_matrix34_untransform_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016650_rage_phwind_initsession(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016660_rage_phwind_getglobalvelocity_rage_windtype_e_rage_vec3v_ref_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_016661_rage_phwind_getlocalvelocity_rage_vec3v_const_ref_rage_vec3v_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_016685_rage_phwindsphere_phwindsphere_rage_phwinddisttype_e_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_016886_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectlocation(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016892_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectlocation(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016893_rage_phlevelnodetree_rage_phlooseoctreenode_rebuildcompositebvh(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016894_rage_phlevelnodetree_rage_phlooseoctreenode_updatecompositebvh_i(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016899_rage_phlevelnodetree_rage_phlooseoctreenode_reserveinstlastmatri(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016900_rage_phlevelnodetree_rage_phlooseoctreenode_releaseinstlastmatri(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016907_rage_phlevelnodetree_rage_phlooseoctreenode_getfirstculledobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016908_rage_phlevelnodetree_rage_phlooseoctreenode_getnextculledobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016909_rage_phlevelnodetree_rage_phlooseoctreenode_testprobe_rage_phseg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_016943_rage_scrthread_printstacktrace_void_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016944_rage_scrthread_preprintstacktrace(ph: *mut Phantom); }
extern "C" { pub fn f_016949_rage_scrthread_getactivethread(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_016981_rage_scrthread_getthread_rage_scrthreadid(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_016983_rage_sysobfuscated_unsigned_long_true_ref_rage_atmap_rage_sysobf(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017186_rage_crclip_composite_rage_crframe_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_018921_rage_fwentity_getmodelid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018937_cgarages_isvehiclewithinhideoutgaragethatcansaveit_cvehicle_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_018989_cpathfind_isregionloaded_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_019099_cpopulationhelpers_getconfigvalue_int_const_ref_int_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019102_cpopgenshape_cpopgenshape(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019103_cpopgenshape_init_rage_vector3_const_ref_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32); }
extern "C" { pub fn f_019104_cpopgenshape_categorisepoint_rage_vector2_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019105_rage_doeslinetouchraywedge_rage_vector2_const_ref_rage_vector2_c(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_019112_cplayerinfo_nearvehiclemiss_cvehicle_const_ptr_cvehicle_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_019128_cvehiclerecordingmgr_stopplaybackwithindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019129_cvehiclerecordingmgr_iscarbeingrecorded_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019133_cvehiclerecordingmgr_getplaybackslot_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019134_cvehiclerecordingmgr_getplaybackslotfast_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019135_cvehiclerecordingmgr_stopplaybackrecordedcar_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019136_cvehiclerecordingmgr_isplaybackgoingonforcar_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019138_cvehiclerecordingmgr_isplaybackswitchedtoaiforcar_cvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019142_cvehiclerecordingmgr_changecarplaybacktouseai_cvehicle_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_019220_cvehicle_updateinslownesszone(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019232_cstuntjumpmanager_abortstuntjumpinprogress(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019241_cstuntjumpmanager_isastuntjumpinprogress(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_019288_caitarget_caitarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019289_caitarget_caitarget_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019290_caitarget_setentity_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019294_caitarget_setposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019295_caitarget_caitarget_caitarget_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019296_caitarget_getisvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019297_caitarget_getposition_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019298_caitarget_setentityandoffsetunlimited_centity_const_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019299_caitarget_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019301_csyncedentity_serialise_rage_csyncdatabase_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019310_cexpensiveprocess_cexpensiveprocess_cexpensiveprocess_processtyp(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_019311_centityscanner_dtorcentityscanner(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019312_centityscanner_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019313_cexpensiveprocess_dtorcexpensiveprocess(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019314_centityscanner_scanforentitiesinrange_centity_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019321_cpedscanner_cpedscanner_cexpensiveprocess_processtype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019326_cvehiclescanner_cvehiclescanner_cexpensiveprocess_processtype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019330_cobjectscanner_cobjectscanner_cexpensiveprocess_processtype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019331_cobjectscanner_dtorcobjectscanner(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019334_cobjectscanner_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019336_cobjectscanner_scanforentitiesinrange_centity_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019340_cexpensiveprocessdistributer_addprocess_cexpensiveprocess_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019343_cspatialarray_insert_cspatialarraynode_ref_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019344_cspatialarray_remove_cspatialarraynode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019345_cspatialarray_update_cspatialarraynode_ref_float_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_019369_cambientmodelset_getcontainsmodel_unsigned_int_cambientmodelvari(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_019387_cambientmodelsetmanager_getmodelsetindexfromhash_cambientmodelse(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_019402_cconditionalclipset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_019450_char_ptr_rage_formatf_char_100ul_char_ref_100ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019452_ailogging_getdynamicentityisclonestringsafe_cdynamicentity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019453_ailogging_getdynamicentitynamesafe_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019459_ailogging_getentitylocalitynameandpointer_cdynamicentity_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019462_cailogmanager_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019565_cpedbonetagconvertor_getboneindexfrombonetag_rage_crskeletondata(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_019685_cmovevehicle_dtorcmovevehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019689_cmovevehicle_startmechanism_rage_crclip_const_ptr_float_float_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_019691_cmovevehicle_openmechanism_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_019692_cmovevehicle_closemechanism_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_019693_cmovevehicle_ismechanismfullyopen(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019694_cmovevehicle_ismechanismfullyclosed(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019695_cmovevehicle_setmechanismphase_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_019696_cmovevehicle_getmechanismphase(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_019844_rage_matrix34_inverse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020765_cridableentrypointevaluationdebuginfo_dtorcridableentrypointeval(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021203_audvehicleaudioentity_smashsiren_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021209_audpolicescanner_reportplayercrime_rage_vector3_const_ref_ecrime(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_021222_audsuperconductor_sendconductormessage_conductors_conductormessa(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021238_audnorthaudioengine_notifylocalplayerplanecrashed(ph: *mut Phantom); }
extern "C" { pub fn f_021349_audweaponaudiocomponent_getweaponsettings_centity_const_ptr_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_021351_cped_shouldbedead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021352_audspeechaudioentity_playanimalvocalization_unsigned_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_021354_audspeechaudioentity_say_char_const_ptr_char_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i64) -> i32; }
extern "C" { pub fn f_021359_auddamagestats_dtorauddamagestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021366_audspeechaudioentity_inflictpain_auddamagestats_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021416_creplaymgr_isrecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021466_audvehicleaudioentity_audvehicleaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021473_audvehicleaudioentity_getvehiclemodelnamehash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021491_audvehiclekerssystem_getkerssoundsetname_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021497_audplaneaudioentity_setpropellorspeedmult_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_021505_audplaneaudioentity_triggerbackfire(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021511_audvehicleaudioentity_isseaplane_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021520_audplaneaudioentity_onverticalflightmodechanged_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_021534_audplaneaudioentity_triggerpropbreak(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021638_audradioaudioentity_getplayerradiostationnamependingretune_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_021739_auduserradiotrackmanager_gettracktitle_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_021741_auduserradiotrackmanager_gettrackartist_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_021761_audspeechaudioentity_saywhensafe_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_021766_audscannermanager_decrementhelisinchase(ph: *mut Phantom); }
extern "C" { pub fn f_021842_audscriptaudioentity_pickrandominterrupterinvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021843_audspeechaudioentity_getconversationinterruptionoffset_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_021930_audspeechaudioentity_setpedvoiceforcontext_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_022014_audvehiclecollisioncontextlist_clearreuselist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022017_audvehicleaudioentity_createvehiclegadget_audvehiclegadget_audve(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022018_audvehicleaudioentity_stopallroofloops(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022023_rage_fwentity_getskeletondata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022040_audvehiclecollisionaudio_getvehiclecollisionsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022045_audvehicleaudioentity_triggerlandinggearsound_rage_audsoundset_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_022052_audvehicleaudioentity_setbombbaydoorsopen_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022053_audvehicleaudioentity_sethydraulicsuspensioninputaxis_float_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_022054_audvehicleaudioentity_triggersimplesoundfromsoundset_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022055_audvehicleaudioentity_triggersideshunt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022056_audvehicleaudioentity_cancelspecialflightmodetransform(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022057_audvehicleaudioentity_onreducedsuspensionforceset_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022063_audvehicleaudioentity_onsubmarinecaranimationstatechanged_unsign(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022071_audvehicleaudioentity_triggeralarm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022072_audvehicleaudioentity_playvehiclealarm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022073_audvehicleaudioentity_ishornon(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022077_rage_fwentity_getobjectmtx_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022078_audvehicleaudioentity_deployoutriggers_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022079_audvehicleaudioentity_startgliding(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022080_audvehicleaudioentity_stopgliding(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022082_audvehicleaudioentity_playvehiclehorn_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_022086_audvehicleaudioentity_triggerstuntracespeedboost(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022087_audvehicleaudioentity_triggerstuntraceslowdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022088_audvehicleaudioentity_triggermissilebatteryreloadsound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022094_audvehicleaudioentity_triggerheadlightsmash_unsigned_int_rage_ve(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_022095_audvehicleaudioentity_triggertyrepuncture_ehierarchyid_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022096_audvehicleaudioentity_revlimiterhit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022098_audvehicleaudioentity_sirenstatechanged_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022100_audvehicleaudioentity_triggersplash_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_022101_audvehicleaudioentity_triggertowinghookattachsound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022102_audvehicleaudioentity_triggertowinghookdetachsound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022103_audvehicleaudioentity_triggerdynamicexplosionsounds_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022121_audvehiclecollisioncontextlist_initandaddeventtolist_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_022123_audvehiclecollisionaudio_processimpactboat_rage_phcontactiterato(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022124_audvehiclecollisionaudio_startprocessimpacts(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022128_audvehiclecollisionaudio_headlightsmash(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022137_rage_atrangearray_audvehicleconductor_vehssireninfo_2_dtoratrang(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022138_audvehicleconductor_updatesirensinfo_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022142_audvehiclefireaudio_registervehgenericfire_cfire_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022164_audvehicleturret_setturretlocalposition_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022188_audvehiclegadgetdynamicspoiler_setspoilerstrutstate_audvehiclega(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022189_audvehiclegadgetdynamicspoiler_setspoileranglestate_audvehiclega(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022228_audweaponaudiocomponent_spindownweapon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022251_audweatheraudioentity_requeststreamedweatheroneshot_rage_audmeta(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: f32, a7: i32, a8: f32); }
extern "C" { pub fn f_022260_audvehiclekerssystem_setkersstate_audvehiclekerssystem_audkersst(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022261_audvehiclekerssystem_triggerkersboostactivate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022262_audvehicleengine_audvehicleengine(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022381_void_creplayeventmanager_storeevent_cpacketsoundphonering_cpacke(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022382_cpacketsoundphonering_ptr_creplaypacketbuffer_recordpacket_cpack(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022391_naenvironmentgroup_allocate_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022397_naaudioentity_createdeferredsound_rage_audmetadataref_centity_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_022398_naaudioentity_createdeferredsound_unsigned_int_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_022503_audcaraudioentity_triggerroofstucksound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022612_audcollisionaudioentity_getmaterialoverride_centity_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_022649_audcollisionaudioentity_reporthelibladecollision_worldprobe_csha(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022658_cped_isdead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022659_naenvironmentgroup_init_naaudioentity_ptr_float_unsigned_int_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_022660_naenvironmentgroup_setsource_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022661_naenvironmentgroup_setinteriorinfowithentity_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022735_rage_audsoundinitparams_setvariablevalue_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_022806_audambientaudioentity_registereffectaudio_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_022885_audsmashableglassevent_reportsmash_centity_const_ptr_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_022948_audheliaudioentity_triggergoingdownsound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022955_audheliaudioentity_sethelishouldskipstartup_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022985_cpathfind_findnodepointersafe_rage_cnodeaddress(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023396_cpacketevent_cpacketevent_ereplaypacketid_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023416_creplayropemanager_getattachedentitya_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_023417_creplayropemanager_getattachedentityb_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_023418_cpacketaddrope_cpacketaddrope_int_rage_vec3v_const_ref_rage_vec3(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32, a7: f32, a8: i32, a9: i32, a10: i32, a11: i32, a12: f32, a13: i32, a14: i32) -> i64; }
extern "C" { pub fn f_023420_cpacketdeleterope_cpacketdeleterope_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_023422_rage_ropeinstance_getconstraint_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023425_cpacketattachentitiestorope_cpacketattachentitiestorope_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32, a7: f32, a8: f32, a9: i64, a10: i64, a11: i32, a12: i32) -> i64; }
extern "C" { pub fn f_023430_rage_ropeinstance_getattachedto_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023437_cpacketropewinding_cpacketropewinding_float_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_023479_rage_audsound_setrequestedposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023565_quantizequaternions3_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023567_quantizequaternions3_20_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023569_cpacketrequestcloudhat_cpacketrequestcloudhat_bool_int_float_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32, a5: i32, a6: f32, a7: i32) -> i64; }
extern "C" { pub fn f_023571_cpacketentityambientfx_cpacketentityambientfx_cparticleattr_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023574_cfire_dtorcfire(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023577_cpacketregistervehiclefire_cpacketregistervehiclefire_int_rage_v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: f32, a5: i32) -> i64; }
extern "C" { pub fn f_023595_cpacketwatersplashvehiclefx_cpacketwatersplashvehiclefx_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_023597_cpacketptexweaponshot_cpacketptexweaponshot_vfxcollinfo_s_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_023602_cpacketptexmtlscrape_cpacketptexmtlscrape_centity_ptr_int_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: f32, a7: i32, a8: f32, a9: i32, a10: f32, a11: f32, a12: i32, a13: i32) -> i64; }
extern "C" { pub fn f_023739_replayfilemanager_isloading_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023846_creplaymgr_replayrecordsound_unsigned_int_rage_audsoundinitparam(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_023849_creplaymgr_replayrecordsound_unsigned_int_unsigned_int_rage_auds(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_023850_creplaymgr_replayrecordsoundpersistant_unsigned_int_rage_audsoun(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_023874_creplayinterface_cvehicle_getcreatepacket_creplayid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023875_cstoredvehiclevariationsreplay_storevariations_cvehicle_const_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023877_creplaymgr_recordobject_cobject_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023881_creplaymgr_hash_unsigned_int_const_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023882_creplaymgr_recordcloth_cphysical_ptr_rage_clothcontroller_ptr_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_023891_replaybuffermarkermgr_addmarker_unsigned_int_unsigned_int_marker(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_023906_creplaycontrol_isplayeroutofcontrol(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023928_void_creplayeventmanager_trackrecordingevent_ptxfireref_ctracked(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_023930_creplayeventmanager_shouldproceedwithpacket_cpacketevent_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023931_creplayeventmanager_dumpmonitoredeventstotty_creplaypacketbuffer(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023952_replayvehicleextension_setvehicleasdamaged_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023967_creplayinterfacemanager_ensureisbeingrecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023968_creplayinterfacemanager_ensurewillberecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023998_rage_fwmodelid_fwmodelid_rage_fwmodelid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_024163_ctrain_setengine_ctrain_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_024164_ctrain_setlinkedtoforward_ctrain_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_024165_ctrain_setlinkedtobackward_ctrain_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_024199_rage_fwentity_getlocalmtxnonconst_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_024257_creplaymgrinternal_isusingrecordedcamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024266_creplaymgrinternal_isawareofworldforrecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024286_creplaymgrinternal_isplaybackflagset_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_024302_creplaymgrinternal_oncreateentity_cphysical_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024303_creplaymgrinternal_ondeleteentity_cphysical_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024304_creplaymgrinternal_getbuildinginfo_cphysical_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_024305_creplaymgrinternal_recordmapobject_centity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024315_creplaymgrproxy_iseditmodeactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024375_cpacketvector4comp_store_rage_vector4_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024622_bool_creplaymgrinternal_handleentityforfx_cpackettrafficlight_cp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026165_caminterface_isdominantrendereddirector_cambasedirector_const_re(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026166_caminterface_isrenderingdirector_cambasedirector_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026168_caminterface_isrenderingcamera_cambasecamera_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026169_cammanager_isrenderingfirstpersoncamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026170_caminterface_isdominantrenderedcameraanyfirstpersoncamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026176_caminterface_getplayercontrolcamaimframe(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026177_camcinematicmountedcamera_shouldaffectaiming_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026179_camcinematicdirector_isrenderingcinematicmountedcamera_bool_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026188_caminterface_getviewporttanfovh(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_026202_caminterface_isspherevisibleingameviewport_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_026203_caminterface_isspherevisibleingameviewport_rage_vec4v_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026205_cqueriableinterface_istaskcurrentlyrunning_int_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_026209_rage_vector3_normalizesafe_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026211_worldprobe_cshapetestfixedresults_unsigned_char_32_cshapetestfix(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026212_caminterface_isrenderedcamerainsidevehicle(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026227_rage_atarray_trenderedcameraobjectsettings_0_unsigned_short_dest(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026237_camcinematicdirector_registervehicledamage_cvehicle_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_026238_cambasecamera_shake_unsigned_int_float_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_026243_camcinematicdirector_isrenderingcinematiccamerainsidevehicle_boo(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026244_camcinematicdirector_getvehicleimpactshakemindamage_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026245_camcinematicdirector_getfirstpersonvehiclecamera_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026248_rage_vector3_normalizesafe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026259_rage_fwentity_getskeleton_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026261_camcinematicdirector_isrenderinganyinvehiclecinematiccamera_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026279_rage_matrix34_toeulersyxz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026286_camgameplaydirector_getthirdpersonaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026293_rage_fwmatrixtransform_getpitch_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026314_camgameplaydirector_getfollowcamera_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026328_rage_matrix34_dot3x3fromleft_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026329_worldprobe_cshapetestresults_dtorcshapetestresults(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026367_rage_imp_transform_imp34_rage_mat34v_ref_float_vector_4_float_ve(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: u128, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_026607_cambasedirector_shake_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_026648_camdebugdirector_getcamframe_cambasecamera_ptr_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026681_camgameplaydirector_computeweaponinfoforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026690_camgameplaydirector_islookingbehind_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026714_camgameplaydirector_getfirstpersonpedaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026753_cphysical_isasleep_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_027008_ctrain_getlinkedtobackward_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027009_ctrain_getlinkedtoforward_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027091_camhandshaker_camhandshaker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027092_camhandshaker_update_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_027093_rage_matrix34_fromeulers_rage_vector3_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_027251_rage_matrix34_rotateunitaxis_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_029377_ctask_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029433_getorcreategroup_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029455_cdebugscene_tunewidgetdata_tunewidgetdata_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_029456_cdebugscene_queuetunewidget_cdebugscene_tunewidgetdata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029512_cdebugscene_getworldpositionundermouse_rage_vector3_ref_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_029612_worldprobe_cshapetestdesc_setresultsstructure_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029613_worldprobe_cshapetestresults_freeintersections(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029866_cdebugscene_focusentities_get_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029874_displayobject_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_029924_cdebugdrawstore_addline_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_029925_cdebugdrawstore_addline_rage_vector2_const_ref_rage_vector2_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_029927_cdebugdrawstore_addsphere_rage_vec3v_const_ref_float_rage_color3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_029928_cdebugdrawstore_addcapsule_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_029930_cdebugdrawstore_add2dtext_rage_vec3v_const_ref_char_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_029932_cdebugdrawstore_addarrow_rage_vec3v_const_ref_rage_vec3v_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_029934_cdebugdrawstore_render(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029935_cdebugdrawstore_clearall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029964_rage_fwtransform_geta_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029965_rage_fwtransform_getb_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029966_rage_fwtransform_getc_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029967_cphysical_gethealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029969_cdebugscene_focusentities_isingroup_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029970_cdebugscene_focusentities_isempty(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_029971_cdebugscene_focusentities_isneargroup_rage_vec3v_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_029978_cvectormap_makeeventripple_rage_vector3_const_ref_float_unsigned(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_030146_tiledscreencapture_isenabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_030309_rage_atarray_centityadd_0_unsigned_short_op_rage_atarray_centity(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030722_ceventgroupped_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030723_cevent_affectsped_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_030739_cpedintelligence_addevent_cevent_const_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030746_geteventscriptnetworkgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030756_ceventhandlerhistory_getcurrentevent_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030757_ceventhandlerhistory_setcurrentevent_cevent_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_030761_cpedintelligence_gettaskactive_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030770_cshockingeventsmanager_add_ceventshocking_ref_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_030771_ceventshocking_initclone_ceventshocking_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_030772_ceventshocking_dtorceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030814_cpeddamagecalculator_cpeddamagecalculator_centity_const_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_030815_cpeddamagecalculator_applydamageandcomputeresponse_cped_ptr_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32); }
extern "C" { pub fn f_030819_cped_isinjured_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030828_ceventdamage_ceventdamage_centity_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_030829_ceventdamage_dtorceventdamage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030839_ceventdamage_affectspedcore_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_030845_ceventagitated_ceventagitated_cped_ptr_agitatedtype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_030846_ceventagitated_dtorceventagitated(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030893_ceventpotentialgetrunover_ceventpotentialgetrunover_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030894_ceventpotentialgetrunover_dtorceventpotentialgetrunover(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030914_ceventnetworkentitydamage_getvehicle_centity_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030915_ceventnetworkentitydamage_isvehicleresponsibleforcollision_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_030932_ceventnetworkvehicleundrivable_ceventnetworkvehicleundrivable_ce(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_030945_svehiclemissionparams_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_031069_ceventgivepedtask_ceventgivepedtask_int_rage_aitask_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_031070_ceventgivepedtask_dtorceventgivepedtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031129_ceventvehicleonfire_ceventvehicleonfire_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031130_ceventvehicleonfire_dtorceventvehicleonfire(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031148_ceventvehicledamage_ceventvehicledamage_cvehicle_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_031169_ceventswitch2nm_ceventswitch2nm_unsigned_int_rage_aitask_ptr_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_031170_ceventswitch2nm_dtorceventswitch2nm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031235_ceventshocking_ceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031242_ceventshocking_set_rage_vec3v_const_ref_centity_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_031400_svehiclemissionparams_settargetentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_031582_ceventnetworkvehicleundrivable_dtorceventnetworkvehicleundrivabl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031622_ceventvehicledamage_dtorceventvehicledamage_54333(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031667_rage_atqueue_rage_eventregdref_rage_fwevent_4_dtoratqueue(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032073_rage_fweventqueue_4_void_flushexpiredevents(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035046_cnewhud_settoggleabilitybar_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035058_chudtools_beginhudscaleformmethod_enewhudcomponents_char_const_p(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_035066_cnewhud_forcespecialvehiclestatusupdate(ph: *mut Phantom); }
extern "C" { pub fn f_035074_cnewhud_setabilityoverride_float_float(ph: *mut Phantom, a0: f32, a1: f32); }
extern "C" { pub fn f_035376_cshaderlib_lookuptexture_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036031_cscaleformmgr_requestremovemovie_int_emovieownertags(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_036038_cscaleformmgr_ismovieactive_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036048_cscaleformmgr_beginmethod_int_escaleform_base_class_char_const_p(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_036049_cscaleformmgr_addparamfloat_double(ph: *mut Phantom, a0: f64); }
extern "C" { pub fn f_036050_cscaleformmgr_endmethod_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036051_cscaleformmgr_addparambool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036053_cscaleformmgr_addparamint_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036056_cscaleformmgr_addparamstring_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036254_cpausemenu_isactive_pm_activeflags(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037659_cseatmanager_countpedsinseats_bool_bool_bool_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_037661_cmodelinfo_requestassets_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037662_cmodelinfo_haveassetsloaded_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037663_cmodelinfo_setassetsaredeletable_rage_fwmodelid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037671_datetime_getsecondssince_int_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037684_cclock_istimeinrange_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_037760_ccrime_reportdestroyvehicle_cvehicle_ptr_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037761_ccrime_reportcrime_ecrimetype_centity_ptr_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_037766_cwanted_setonestarparolenodrop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037767_cwanted_setwantedlevelnodrop_rage_vector3_const_ref_int_int_setw(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_037801_cthepopmultiplierareas_gettrafficdensitymultiplier_rage_vector3(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_037888_cseatmanager_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_037891_cincidentmanager_addincident_cincident_const_ref_bool_int_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_037903_corder_removeorderfromentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037919_cweathergpufxfromxmlfile_dtorcweathergpufxfromxmlfile(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037932_rage_atarray_cweathertypefromxmlfile_0_unsigned_short_destruct_c(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037933_cweather_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037934_cweather_gettypeindex_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037935_cweather_update(ph: *mut Phantom); }
extern "C" { pub fn f_037946_cweather_getprevtype(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037948_cweather_renderdebug(ph: *mut Phantom); }
extern "C" { pub fn f_037949_cweather_gettemperature_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_037957_cweather_getnettransitionprevtype(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037958_cweather_getnettransitionnexttype(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_038130_cmodelinfo_getbasemodelinfo_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038336_cwantedincident_isincidentnearby_float_centity_const_ptr_cincide(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_038343_cfireincident_dtorcfireincident(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038365_cincidentmanager_findfireincidentforfire_cfire_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038490_cikmanager_resetallsolvers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038624_cbasemodelinfo_get2deffects_rage_atarray_c2deffect_ptr_0_unsigne(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038625_cbasemodelinfo_bumpdrawablerefcount(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038626_cbasemodelinfo_releasedrawable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038636_cbasemodelinfo_getassetlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038714_cmodelinfo_areassetsrequested_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038716_cmodelinfo_getassetsaredeletable_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038718_cmodelinfo_getassetrequiredflag_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038721_cmodelinfo_getobjectanddependencies_rage_fwmodelid_rage_atarray(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_038722_cmodelinfo_getobjectanddependenciessizes_rage_fwmodelid_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_038724_cmodelinfo_updatevehicleclassinfos(ph: *mut Phantom); }
extern "C" { pub fn f_038732_centryexitpoint_canseatbereached_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038733_centryexitpoint_getseat_seataccess_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_038735_cmodelseatinfo_calculateentrysituation_centity_const_ptr_cped_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32, a7: i64); }
extern "C" { pub fn f_038739_cmodelseatinfo_getentrypointindexforseat_int_centity_const_ptr_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_038740_cmodelseatinfo_getboneindexfromseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038741_cmodelseatinfo_getboneindexfromentrypoint_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038743_cmodelseatinfo_getshuffleseatforseat_int_int_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_038751_cseatmanager_getpedsseatindex_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038754_cmodelseatinfo_ispositionclearforped_cped_const_ptr_centity_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_038755_cpedmodelinfo_getlayoutinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038756_cseatmanager_getlastdriver_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038757_cseatmanager_getlastpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038904_cvehiclemodelinfo_initdatalist_dtorinitdatalist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038926_cvehiclemodelinfo_getvehicleinstance_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038940_cvehiclemodelinfoplates_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038946_cvehiclemodelinfovarglobal_refreshcolornames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038947_cvehiclemodelinfovarglobal_refreshwindowcolornames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038948_cvehiclemodelinfovarglobal_refreshlightnames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038949_cvehiclemodelinfovarglobal_refreshsirennames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038970_cvehiclemodelinfovariation_deletewidgetnames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038976_cvehiclevariationinstance_setmodindex_evehiclemodtype_unsigned_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_038977_cvehiclevariationinstance_setmodcollision_cvehicle_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_038978_cvehiclevariationinstance_getmodindexfortype_evehiclemodtype_cve(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_038980_cvehiclevariationinstance_updatebonepositions_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038981_cvehiclevariationinstance_togglemod_evehiclemodtype_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_038982_cvehiclevariationinstance_istogglemodon_evehiclemodtype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038992_cvehiclemodelinfo_gethdfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038994_cvehiclemodelinfo_initphys(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038997_cvehiclemodelinfo_calculateheightsaboveroad_float_ptr_float_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_039009_cvehiclemodelinfo_computebuoyancyconstantfromsubmergevalue_float(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_039011_cvehiclemodelinfo_getseatinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039012_cvehiclemodelinfo_getentrypointaniminfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039013_cvehiclemodelinfo_getseatanimationinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039014_cvehiclemodelinfo_getentrypointinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039015_cvehiclemodelinfo_choosevehiclecolour_unsigned_char_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_039016_cvehiclemodelinfo_choosevehiclecolourfancy_cvehicle_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_039017_cvehiclemodelinfo_getsirensettings_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039020_cvehiclemodelinfo_addtohdinstancelist_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039021_cvehiclemodelinfo_removefromhdinstancelist_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039022_cvehiclemodelinfo_getrandomloadedtrailer_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039025_cvehiclemodelinfo_getdistancesqrtoclosestinstance_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> f32; }
extern "C" { pub fn f_039026_cvehiclemodelinfo_getcanpassengersbeknockedoff(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039028_cvehiclemodelinfo_getsteeringwheelmult_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_039126_cvehiclemodelinfodoors_containsthisdoorwithcollision_ehierarchyi(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039127_cvehiclemodelinfodoors_containsthisdriveabledoor_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039160_populatecommonvehiclecoronaproperties_vehiclecorona_const_ref_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: f32, a5: f32); }
extern "C" { pub fn f_039185_sireninstancedata_reset_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039186_sireninstancedata_issirenlighton_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_039187_sireninstancedata_getlightintensitymutiplier_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> f32; }
extern "C" { pub fn f_039203_cvehiclevariationinstance_cleanuppreloadmods_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039206_cvehiclevariationinstance_getbone_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039207_cvehiclevariationinstance_getboneforlinkedmod_unsigned_char_cons(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039211_cvehiclevariationinstance_getkit_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039212_cvehiclevariationinstance_getwheeltype_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_039213_cvehiclevariationinstance_getnummodsfortype_evehiclemodtype_cveh(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_039216_cvehiclevariationinstance_setmodindexfortype_evehiclemodtype_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_039218_cvehiclevariationinstance_getmodboneindex_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039219_cvehiclevariationinstance_getarmourdamagemultiplier_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_039220_cvehiclevariationinstance_hidemodonbone_cvehicle_const_ptr_int_s(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_039221_cvehiclevariationinstance_isboneturnedoff_cvehicle_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_039222_cvehiclevariationinstance_getbonetoturnoffforslot_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_039223_cvehiclevariationinstance_getnumbonestoturnoffforslot_unsigned_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039230_cvehiclevariationinstance_setwheeltype_evehiclewheeltype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039232_cvehiclevariationinstance_getmodifier_evehiclemodtype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039247_cvehiclestreamrendergfx_storemodmatrices_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039248_cvehiclestreamrendergfx_getfrag_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039253_cvehiclestreamrendergfx_getextralightmodindex_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039254_cvehiclestreamrendergfx_getextralightboneindex_ehierarchyid_cons(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039255_cvehiclestreamrendergfx_getextralightcount_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039573_cpathservergta_updatedynamicobject_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_039576_cdoor_getdooropenratio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_039613_cdoor_doortypeautoopensforvehicles_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_039617_rage_phcontactconstiterator_skiptonextactivecontact_rage_phmanif(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_039627_cobject_initanimlazy(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039630_cweapon_sfireparams_dtorsfireparams(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039638_cpathservergta_updatevehicledoordynamicobject_cvehicle_ptr_ccard(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039688_cobjectpopulation_destroyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039699_cobject_setdrivetotarget_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_039701_cobject_isjointatmaxangle_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039702_cobject_isjointatminangle_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039718_cobject_gettask_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039719_cobject_settask_rage_aitask_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_039723_cobject_decouplefrommap(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039761_cobjectpopulation_createobject_rage_fwmodelid_eentityownedby_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_039762_cobjectpopulation_createobject_cobjectpopulation_createobjectinp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039763_cnavmeshdataexporter_willexportcollision(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_039812_rage_fwtransform_getmatrix_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039877_cpathserver_isrequestresultready_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_039895_cpathserver_requestlineofsight_rage_vector3_const_ptr_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64, a8: f32) -> i32; }
extern "C" { pub fn f_039904_cpathserver_getlineofsightresult_unsigned_int_bool_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_039913_cpathserver_cancelrequest_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_039916_cpathserver_isnavmeshloadedatposition_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039996_cpathservergta_mayberemovedynamicobject_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040005_tdynobjboundsfuncs_calcboundsforfire_rage_tdynobjbounds_ref_cfir(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040006_cpathservergta_addvehicledoordynamicobject_cvehicle_ptr_ccardoor(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040007_cpathservergta_mayberemovedynamicobjectnavmesh_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040139_cpedgroupmembership_getmember_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_040194_cpedgroups_getpedsgroup_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040220_worldprobe_cshapetesthitpoint_gethitinst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040328_worldprobe_cshapetestresults_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040423_rage_fraginstnm_mapragdolllodcomponentcurrenttohigh_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041056_findplayerped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041060_cpeddebugvisualisermenu_getfocusped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041075_cpedtargetting_cpedtargetting_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041077_cpedtargetting_getlosstatus_centity_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041116_cpedintelligence_flushimmediately_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041117_cpedintelligence_addtoeventhistory_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041122_svehiclemissionparams_dtorsvehiclemissionparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041129_rage_quaternion_fromeulers_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041197_cpedintelligence_setdefaultrelationshipgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041202_cpedintelligence_setrelationshipgroup_crelationshipgroup_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041209_centityboundai_centityboundai_centity_const_ref_float_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32, a7: i64, a8: f32) -> i64; }
extern "C" { pub fn f_041211_centityboundai_testlineofsightreturndistance_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_041241_ccollisioneventscanner_processragdollimpact_cped_ptr_float_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64, a5: i32, a6: i64); }
extern "C" { pub fn f_041265_cplayerinfo_isaiming_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041268_cinterestingevents_add_cinterestingevents_etype_centity_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_041280_cpedpopulation_iscandidateinviewfrustum_rage_vector3_const_ref_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_041296_cpedfactory_destroyped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041306_cpedfootstephelper_getfootvelocity_feettags_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041313_cpedfootstephelper_setwalkingonpuddle_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041332_cpedgeometryanalyser_testifcollisiontypeisvalid_cped_const_ptr_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_041335_cpedintelligence_ispedswimming_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041337_cpedhelmetcomponent_ishelmetenabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041338_cpedhelmetcomponent_haspilothelmetequippedinaircraftinfps_bool_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041380_cpedtargetting_getbesttarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041385_cpedintelligence_findtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041394_cpedintelligence_ispedgettingup_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041443_cpedintelligence_revvedat_cvehicle_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041447_cpedintelligence_dodged_centity_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_041489_cpedpopulation_getpopulationcontrolpointinfo_cpopctrl_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041496_findplayervehicle_cplayerinfo_ptr_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041498_cpedpopulation_canpedbereused_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041499_cpedpopulation_addpedtoreusepool_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041504_cpedpopulation_pedpopulationfailuredata_registerspawnfailure_cpe(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_041509_rage_fwregdref_cscenariopoint_rage_fwrefawarebasenonvirt_reset_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041525_cpedpopulation_addpedincar_cvehicle_ptr_bool_int_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64) -> i64; }
extern "C" { pub fn f_041526_cpedpopulation_findspecificdrivermodelforcartouse_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041536_cvehicleintelligence_resetpretendoccupanteventdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041537_cpopulationgroup_isindexmember_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041541_cpedpopulation_schedulepedinvehicle_cvehicle_ptr_int_bool_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64) -> i32; }
extern "C" { pub fn f_041543_cpedpopulation_selectpointsrealtype_cscenariopoint_const_ref_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_041589_cexplosionmanager_cexplosionargs_dtorcexplosionargs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041590_cplayerspecialability_getfxstrength_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041594_cplayerspecialability_deactivatenofadeout(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041602_cplayerspecialability_shouldapplyfx_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041609_cplayerspecialabilitymanager_processcontrol_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041650_cplayerinfo_getplayerstatinfoforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041663_cplayerinfo_setmayonlyenterthisvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041681_cplayerinfo_controlbuttonsprint_cplayerinfo_esprinttype_bool(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_041701_canimspeedups_shouldusempanimrates(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041712_cplayerinfo_getbuttonsprintresults_cplayerinfo_esprinttype(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041725_cplayerinfo_keepareaaroundplayerclear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041743_cplayerinfo_canplayerperformvehiclemelee_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041744_cvehicle_getthrottle_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041795_cpopschedule_setoverridevehgroup_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041797_cpopgrouplist_findvehgroupfromnamehash_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_041809_cpopzones_findsmallestforposition_rage_vector3_const_ptr_ezoneca(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_041810_cpopcycle_getwantedcarmultiplier(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_041811_cpopcycle_getcurrentpercentageofmaxcars(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_041812_cpopcycle_getinteriorcarmultiplier(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_041813_cpopcycle_getcombatcarmultiplier(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_041814_cpopcycle_getcurrentmaxnumparkedcars(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041817_cpopcycle_findnewcarmodelindex_unsigned_int_ptr_cloadedmodelgrou(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_041819_cpopcycle_getcurrentzonedirtcol(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041820_cpopcycle_iscurrentzoneairport(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041821_cpopcycle_getcurrentzonevfxregionhashname(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041822_cpopcycle_calchighwaycarmultiplier(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_042004_cqueriableinterface_getstatefortasktype_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042005_cqueriableinterface_getrunningscenariotype_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042125_cpeddamageset_calcuvs_rage_vector2_ref_rage_vector2_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: f32, a8: i32, a9: i32, a10: i32) -> i32; }
extern "C" { pub fn f_042129_cpeddamageset_adddecorationblit_rage_atnonfinalhashstring_rage_a(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: f32, a8: i64, a9: i64, a10: i64, a11: i32, a12: f32, a13: i64, a14: f32, a15: f32, a16: f32, a17: f32, a18: i32, a19: i32); }
extern "C" { pub fn f_042167_cpeddamagemanager_computelocalpositionandzone_unsigned_char_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_042168_cpeddamagemanager_addblooddamage_unsigned_char_unsigned_short_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_042169_cpeddamagemanager_getbloodinfoanddamageid_cped_ptr_rage_atnonfin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_042170_cpeddamagemanager_getdamagedecalanddamageid_cped_ptr_rage_atnonf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_042171_cpeddamagemanager_addpedblood_cped_ptr_unsigned_short_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_042440_cpedvariationpack_checkvariation_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042484_char_ptr_rage_formatf_char_200ul_char_ref_200ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042489_cpedpropsmgr_getpedpropidx_cped_const_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042559_cpedpropsmgr_setpedpropsrandomly_cped_ptr_float_float_unsigned_i(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32, a4: i64, a5: i64, a6: i64, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_042621_cpedpropsmgr_setrandomhelmet_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042639_cpedpropsmgr_clearpedprop_cped_ptr_eanchorpoints_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042790_cwatertesthelper_getriverheightatposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_042814_worldprobe_debugfindgroundzfor3dcoord_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> f32; }
extern "C" { pub fn f_042817_worldprobe_debugfindgroundzfor3dcoord_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32, a5: i64, a6: i64) -> f32; }
extern "C" { pub fn f_042818_worldprobe_cshapetestmanager_debugsubmittest_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_042821_worldprobe_cshapetestmanager_aborttest_worldprobe_cshapetestresu(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042844_cwatertesthelper_getwaterheightatpositionincludingrivers_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_042848_worldprobe_cshapetestdesc_setexcludeentities_centity_const_ptr_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_042897_cpickupmanager_createsomemoney_rage_vector3_const_ref_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_043046_ccollisionhistory_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043047_ccollisionhistory_findcollisionrecord_centity_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_043048_ccollisionhistory_addcollisionrecord_rage_phinst_const_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32, a8: i32, a9: i64, a10: i32, a11: i32); }
extern "C" { pub fn f_043049_ccollisionhistory_getmostsignificantcollisionrecordoftype_unsign(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043050_ccollisionhistory_getmostsignificantcollisionrecordoftypes_unsig(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043071_cdeformableobjectmanager_ismodeldeformable_rage_atnonfinalhashst(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_043073_cbuoyancy_process_cphysical_ptr_float_bool_bool_float_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_043074_ctunableobjectmanager_gettuningentry_rage_atnonfinalhashstring_c(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043075_rage_matrix34_transform_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043077_cbuoyancy_getbuoyancyinfo_cphysical_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_043078_cwatertesthelper_getwaterlevelincludingriversnowaves_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_043079_cbuoyancy_resetbuoyancy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043080_cbuoyancy_getwaterlevelonsample_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_043085_worldprobe_cshapetestdesc_setexcludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043088_cbuoyancyinfo_clone(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043089_cbuoyancyinfo_dtorcbuoyancyinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043096_phinstgta_phinstgta_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043100_rage_fwanimdirector_getcomponentbytype_rage_fwanimdirectorcompon(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043112_fraginstgta_fraginstgta_int_rage_fragtype_const_ptr_rage_matrix3(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_043115_fraginstgta_setdontbreakflagallchildren_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043162_phmaterialgta_findvfxgroup_char_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043194_phmaterialmgrgta_getmtlrumbleprofile_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_043209_cphysics_removevehicledeformation_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043223_cphysics_clearmanifoldsforinst_rage_phinst_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043225_cphysics_getfoliageradius_rage_phinst_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> f32; }
extern "C" { pub fn f_043234_worldprobe_cshapetestdesc_setincludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043242_cphysics_getmeasuringtoolpos_int_rage_vector3_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_043282_physicshelpers_findvehiclecollisionfault_cvehicle_const_ptr_cveh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32) -> i32; }
extern "C" { pub fn f_043284_cflightmodelhelper_cflightmodelhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043289_cflightmodelhelper_makecircularinputsquare_rage_vector2_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043291_cflightmodelhelper_debugdrawcontrolinputs_cphysical_const_ptr_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_043331_cwatertesthelper_submitasyncriverboundprobe_rage_vector3_ref_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043332_cwatertesthelper_forceupdateriverproberesultsynchronously_rage_v(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043333_cwatertesthelper_getcachedriverboundproberesult_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_043334_cwatertesthelper_getwaterlevelincludingrivers_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_043335_worldprobe_debugfindgroundzforcoord_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_043342_worldprobe_cshapetestdesc_setexcludeinstances_rage_phinst_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043348_worldprobe_cshapetestdesc_addexludeinstance_rage_phinst_const_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043349_worldprobe_cshapetestdesc_setexcludeinstance_rage_phinst_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043350_worldprobe_cshapetestdesc_addexcludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043480_crendertargets_createrendertarget_rtmempoolid_char_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_043485_water_iscameraunderwater(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043489_cocclusion_gethizbuffer_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_043500_cocclusion_getnewocclusionaabbexactsimpletestpixelsthreshold(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043502_isaabboccludedasync_rage_spdaabb_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_043508_occlusionqueries_oqgetqueryresult_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_043590_islightvisible_clightsource_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043639_crendertargets_getbackbuffer_bool(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_043640_crendertargets_getbackbuffercopy_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_043641_crendertargets_copybackbuffer(ph: *mut Phantom); }
extern "C" { pub fn f_043644_crendertargets_lockscenerendertargets_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043645_shaderutil_startshader_char_const_ptr_rage_grmshader_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_043646_shaderutil_endshader_rage_grmshader_const_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_043647_crendertargets_unlockscenerendertargets(ph: *mut Phantom); }
extern "C" { pub fn f_043654_postfx_bulletimpactoverlay_registerbulletimpact_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_043660_crenderphasecascadeshadowsinterface_setsharedshadowmap_rage_grct(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043661_crendertargets_lockreadonlydepth_bool_bool_rage_grcrendertarget(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_043662_crendertargets_copydepthbuffer_rage_grcrendertarget_ptr_rage_grc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043664_crendertargets_unlockreadonlydepth(ph: *mut Phantom); }
extern "C" { pub fn f_043670_csprite2d_begincustomlist_csprite2d_customshadertype_rage_grctex(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_043672_deferredlighting_getprojectionshaderparams_rage_vec4v_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_043673_water_getwaterlevel(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_043675_csprite2d_endcustomlist(ph: *mut Phantom); }
extern "C" { pub fn f_043676_postfx_downsamplebackbuffer_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043677_animpostfxmanager_start_rage_atnonfinalhashstring_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_043679_postfx_setupblendfordof_rage_grcblendstatedesc_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043712_animpostfxmanager_cancelstartrequest_rage_atnonfinalhashstring_a(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043713_animpostfxmanager_isrunning_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_043714_animpostfxmanager_stop_rage_atnonfinalhashstring_animpostfxmanag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043717_animpostfxmanager_isstartpending_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_043745_pausemenupostfxmanager_isfading_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043753_csprite2d_setgeneralparams_rage_vector4_const_ref_rage_vector4_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043783_cpostscan_addtoalwaysprerenderlist_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043790_cpostscan_removealphaoverride_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043815_lights_addscenelight_clightsource_ref_clightentity_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043842_cgtarenderlistgroup_deferredaddentity_int_rage_fwentity_ptr_unsi(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_043902_crendertargetmgr_getnamedrendertarget_rage_atfinalhashstring_con(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043912_crendertargetmgr_isnamedrendertargetlinked_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_043915_crendertargets_lockscenerendertargets_depthcopy(ph: *mut Phantom); }
extern "C" { pub fn f_043937_csprite2d_settexture_rage_grctexture_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043939_csprite2d_setdepthtexture_rage_grctexture_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043970_river_getriverflowfromposition_rage_vec3v_const_ref_rage_vector2(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043975_csprite2d_delete(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043978_csprite2d_setrefmipblurparams_rage_vector4_const_ref_rage_grctex(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043979_csprite2d_settexelsize_rage_vector2_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043983_csprite2d_drawrect_float_float_float_float_float_rage_color32_co(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i64); }
extern "C" { pub fn f_044119_water_addwavetoresult_float_float_float_ptr_rage_vector3_ptr_flo(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_044122_water_getwaterlevel_rage_vector3_const_ref_float_ptr_bool_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_044123_water_getwaterlevelnowaves_rage_vector3_const_ref_float_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_044125_water_testlineagainstwater_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_044143_ccsmsharedshadervars_setshadervars_ccsmglobals_const_ref_const(ph: *mut Phantom); }
extern "C" { pub fn f_044154_crenderphasereflection_setreflectionmap(ph: *mut Phantom); }
extern "C" { pub fn f_044168_lights_beginlightweighttechniques(ph: *mut Phantom); }
extern "C" { pub fn f_044169_lights_endlightweighttechniques(ph: *mut Phantom); }
extern "C" { pub fn f_044176_water_getcamerawaterheight(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_044177_water_getcamerawaterdepth(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_044187_clightgroup_setambientlightingglobals_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32); }
extern "C" { pub fn f_044189_lights_setupdirectionalandambientglobals(ph: *mut Phantom); }
extern "C" { pub fn f_044190_crenderphasecascadeshadowsinterface_setdeferredlightingshadervar(ph: *mut Phantom); }
extern "C" { pub fn f_044196_water_modifydynamicwaterspeed_float_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_044201_water_addfoam_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32); }
extern "C" { pub fn f_044202_water_findnearestwaterheightfromquads_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_044269_deferredlighting_setshadergbuffertarget_gbufferrt_rage_grctextur(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_044279_clightgroup_setforwardlights_clightsource_const_ptr_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044284_clightgroup_setdirectionallightingglobals_float_bool_rage_vec4v(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044288_clightgroup_resetlightingglobals(ph: *mut Phantom); }
extern "C" { pub fn f_044299_lights_beginlightrender(ph: *mut Phantom); }
extern "C" { pub fn f_044302_lights_endlightrender(ph: *mut Phantom); }
extern "C" { pub fn f_044328_capplydamage_enqueuedamage_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_044330_capplydamage_getnumdamagepending_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_044332_capplydamage_removevehicledeformation_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044412_getskeletonfordraw_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_044476_centitydrawhandler_centitydrawhandler_centity_ptr_rage_rmcdrawab(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_044479_lights_calculatepartialdrawablebound_centity_const_ref_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_044480_lights_uselightsinarea_rage_spdaabb_const_ref_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_044506_cvehicledrawhandler_shadereffect_hd_createinstance_cvehiclemodel(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_044507_cvehicledrawhandler_shadereffect_hd_destroyinstance_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044518_casynclightocclusionmgr_addlight(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_044520_cvolumevertexbuffer_generatepointspotvolumevb_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_044527_lights_calculatetimefade_float_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_044537_lights_registerbrokenlight_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044538_lights_removeregisteredbrokenlight_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044541_lights_setupdirectionalglobals_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_044544_lights_calculatetimefade_unsigned_int(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_044607_cparaboloidshadow_updatelight_clightsource_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_044661_crenderphasecascadeshadowsinterface_restoresharedshadowmap(ph: *mut Phantom); }
extern "C" { pub fn f_044915_crenderphasedrawsceneinterface_getlatevehicleinteralphastate_ren(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045786_cped_computedefaultdrivingtask_cped_const_ref_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_045788_cped_getpedhealthinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045789_cped_ismale_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045793_cped_checkbraveryflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045795_cped_checkagilityflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045797_cped_getcurrentmotiontask_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045802_cped_setcharparamsbasedonmanagertype(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045844_cped_setdefaultdecisionmaker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045878_cped_getmassforapplyvehicleforce_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_045883_cped_setpedoutofvehicle_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_045884_cped_knockpedoffvehicle_bool_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_045885_cped_switchtoragdoll_cevent_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045886_cped_setmyvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045889_cped_getprimarymotiontask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045902_cped_updatespatialarraytypeflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045913_cped_isfirstpersonshootermodeenabledforplayer_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045922_cped_getboneposition_rage_vector3_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045926_cped_getisdrivingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045939_cped_setvarrandom_unsigned_int_unsigned_int_rage_atfixedbitset_3(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_045940_cped_setvariation_epedvarcomp_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_045966_cped_delayedremovaltimehaspassed_float_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_045968_cped_gettaskdata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045977_cped_setdesiredheading_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_045989_rage_phcontact_setworldnormal_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045996_cped_getbonetagfromragdollcomponent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045998_cped_getboneindexfromragdollcomponent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046002_cped_isinfirstpersonvehiclecamera_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_046005_cped_isfireresistant_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046014_cped_incwetclothing_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046015_cped_getisshelteredinvehicle(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046016_cped_getisshelteredfromrain(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046052_cped_issecurityped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046056_cped_canbedeleted_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_046074_cpedintelligence_gettasksecondaryactive_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046076_clightsource_clightsource_elighttype_int_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: f32) -> i64; }
extern "C" { pub fn f_046077_clightsource_setdirtangent_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046090_cped_flagtodestroywhennextprocessed_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046099_cped_getragdollcomponentmatrix_rage_matrix34_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_046103_cped_getspecialability_eplayerspecialabilityslot(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046105_cped_getblockingofnontemporaryeventsthisframe_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046125_cped_setforcedseatusage_int_int_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_046126_cped_clearforcedseatusage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046130_cped_countnearbyhostilepedstryingtoentermyvehicle_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_046135_cped_isgangped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046144_cped_newsay_char_const_ptr_unsigned_int_bool_bool_int_cped_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32) -> i32; }
extern "C" { pub fn f_046146_cped_randomlysay_char_const_ptr_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_046152_cped_setpedinvehicle_cvehicle_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_046176_cped_getcontrolfromplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046180_cped_instantaiupdate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046183_cped_getragdollcomponentfrombonetag_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046192_cped_isdebuginvincible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046205_cpedvfx_setoutofwaterbonetimer_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_046207_cpedvfx_getbloodvfxdisabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046285_centity_dtorcentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046302_cdynamicentity_createskeleton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046303_cdynamicentity_createanimdirector_rage_rmcdrawable_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_046305_cdynamicentity_addtosceneupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046311_cdynamicentity_deletedrawable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046317_centity_centity_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046325_cgameworld_findfollowplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_046326_cgameworldheightmap_getmaxheightfromworldheightmap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_046327_cplayerswitchinterface_isactive_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_046404_cportaltracker_setloadscollisions_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046405_cportaltracker_addtoactivatingtrackerlist_cportaltracker_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046408_cinteriorinst_getinteriorforlocation_rage_fwinteriorlocation_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046410_centity_getinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046423_centity_getboundingboxmax_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046426_cdynamicentity_getextendedboundbox_rage_spdaabb_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046428_cdynamicentity_getismainsceneupdateflagset_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046429_cdynamicentity_updateskeleton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046431_cdynamicentity_startanimupdate_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046433_cdynamicentity_startanimupdateaftercamera_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046438_cgameworld_add_centity_ptr_rage_fwinteriorlocation_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046442_cinteriorproxy_getfromlocation_rage_fwinteriorlocation_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046445_cportaltracker_collectstack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046447_cinteriorproxy_createlocation_cinteriorproxy_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046450_cdynamicentity_prerender_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046459_cdynamicentity_getscriptthatcreatedme_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046467_cgameworld_remove_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046478_centity_getplaceabletracker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046479_centity_updaterenderflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046481_centity_setmodelid_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046484_centity_setisvisibleformodule_eisvisiblemodule_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046490_centity_isvisible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046491_centity_getisonscreen_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046495_centity_isinpathserver_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046497_centity_setphysicsinst_rage_phinst_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046499_centity_getcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046500_centity_getisstatic_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046501_centity_getisfixedroot_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046502_centity_getphysarch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046506_centity_losingfragcacheentry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046516_centity_processfxentitycollision_rage_vector3_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32); }
extern "C" { pub fn f_046520_centity_trytoexplode_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_046524_centity_getaudioenvironmentgroup_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046528_centity_setisvisibleforallmodules(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046530_centity_commondebugstr_centity_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046531_centity_getisvisibleformodule_eisvisiblemodule_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046545_centity_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046596_centityidhelper_computeentityid_rage_fwentity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046599_centityiterator_centityiterator_int_centity_const_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_046600_centityiterator_getnext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047020_cgameworld_findlocalplayerwanted(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047044_cgameworld_forallentitiesintersecting_rage_fwsearchvolume_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_047050_cphysical_cphysical_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_047051_cphysical_dtorcphysical(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047054_cphysical_setisinwater_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047055_cphysical_addphysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047056_cphysical_removephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047057_cphysical_removeblip_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047058_cphysical_removeconstraints_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047059_cphysical_updatephysicsfromentity_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047060_cphysical_activatephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047061_cphysical_deactivatephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047064_cphysical_setvelocity_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047065_cphysical_getvelocity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047067_cphysical_setangvelocity_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047068_cphysical_getangvelocity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047069_cphysical_calculateangvelocityfrommatrices_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_047072_cphysical_setmass_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047073_cphysical_sethealth_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047074_cphysical_changehealth_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047075_cphysical_setweapondamageinfo_centity_ptr_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_047083_cphysical_setupmissionstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047085_cphysical_updateentityfromphysics_rage_phinst_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_047088_cphysical_resetwaterstatus(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047089_cphysical_setmatrix_rage_matrix34_const_ref_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_047090_cphysical_processcollision_rage_phinst_const_ptr_centity_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: f32, a7: i64, a8: i32, a9: i32, a10: i64, a11: i32, a12: i32); }
extern "C" { pub fn f_047092_cphysical_docollisioneffects(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047095_cphysical_applyforce_rage_vector3_const_ref_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32); }
extern "C" { pub fn f_047096_cphysical_applyforcecg_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_047097_cphysical_applytorque_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_047098_cphysical_applytorque_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_047100_cphysical_applyimpulse_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047101_cphysical_applyimpulsecg_rage_vector3_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047102_cphysical_applyangimpulse_rage_vector3_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047104_cphysical_applyexternalforce_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: f32, a6: f32); }
extern "C" { pub fn f_047106_cphysical_getlocalspeed_rage_vector3_const_ref_bool_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047107_cphysical_processprephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047108_cphysical_processpostphysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047113_cphysical_iscollisionloadedaroundposition(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047115_cphysical_testnocollision_rage_phinst_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_047118_cphysical_debugattachtophysicalusingphysics_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64, a10: i64, a11: f32, a12: i32, a13: f32, a14: f32) -> i32; }
extern "C" { pub fn f_047123_cphysical_detachfromparent_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047130_cphysical_getdistancefromcentreofmasstobaseofmodel_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_047134_cphysical_cleanupmissionstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047136_cphysical_calculategroundvelocityintegrated_cphysical_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64); }
extern "C" { pub fn f_047137_cphysical_calculategroundangularvelocity_cphysical_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_047305_cgameworld_getmainplayerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047345_cinteriorinst_createlocation_cinteriorinst_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_047479_cextensiondefspawnpoint_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_047681_cplayerswitchinterface_shouldusefullcircleforpopstreaming_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_047682_cplayerswitchinterface_getonscreenspawndistance_const(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_047715_cinteriorinst_getroomhashcode_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047725_cinteriorinst_issubwaymlo_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047729_cinteriorinst_getnumexitportalsinroom_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047745_cinteriorinst_findintersectingmlos_rage_fwissphereintersecting_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047755_cinteriorproxy_generateuniquehashfromlocation_rage_fwinteriorloc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047758_cinteriorproxy_getinteriorproxyfromcollisiondata_centity_ptr_rag(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047880_cgameworld_findlocalplayercoors(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047965_cgameworld_findfollowplayervehicle(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047971_cgameworld_findobjectskindacolliding_rage_vector3_const_ref_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32); }
extern "C" { pub fn f_047975_cgameworld_clearcarsfromarea_rage_vector3_const_ref_float_bool_b(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_047977_cgameworld_clearpedsfromarea_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047979_cgameworld_clearobjectsfromarea_rage_vector3_const_ref_float_boo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_047980_cgameworld_logclearcarfromarea_char_const_ptr_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_047982_cgameworld_findlocalplayervehicle_bool(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047996_cgameworld_findlocalplayertrain(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_048037_cgameworldheightmap_getminheightfromworldheightmap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_048062_cgameworldwaterheight_getwaterheightatpos_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_048339_networkutils_isnetworkcloneormigrating_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_050603_graphics_commands_drawspritetex_rage_grctexture_ptr_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i64, a13: i64, a14: i32, a15: i32); }
extern "C" { pub fn f_054790_network_commands_commandnetworkisactivitysession(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_054930_network_commands_commandnetworkplayergetname_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_056465_object_commands_objectcreationfunction_int_float_float_float_boo(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_058506_void_creplaymgrinternal_recordpersistantfx_cpacketaddrope_unsign(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_058516_void_creplaymgrinternal_recordpersistantfx_cpacketropewinding_un(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_058522_void_creplaymgrinternal_recordpersistantfx_cpacketdeleterope_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_058531_void_creplaymgr_recordpersistantfx_cpacketattachentitiestorope_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_061626_cvehicledeformation_lockdamagetexture_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_063796_cthescripts_unregisterentity_cphysical_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063800_cthescripts_getcurrentscriptnameandprogramcounter(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063893_csuppressedmodels_hasmodelbeensuppressed_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_063900_cscriptentities_clearspaceformissionentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064486_ragesecgamepluginbondermanager_rebalancebonders(ph: *mut Phantom); }
extern "C" { pub fn f_064493_ragesecpopreaction(ph: *mut Phantom); }
extern "C" { pub fn f_064509_ccustomshadereffectvehicletype_create_rage_rmcdrawable_ptr_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_064588_ccustomshadereffecttint_selecttintpalette_unsigned_char_rage_fwe(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064639_ccustomshadereffectvehicle_updatevehiclecolors_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_064640_ccustomshadereffectvehicle_setlicenseplatetexindex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064642_ccustomshadereffectvehicle_setlicenseplatetext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064644_ccustomshadereffectvehicle_copysettings_ccustomshadereffectvehic(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064652_ccustomshadereffectvehicle_getlicenseplatetext_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064653_ccustomshadereffectvehicle_trackuvanimadd_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064654_ccustomshadereffectvehicle_trackammouvanimadd_rage_vector2_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064655_cvehicleglassshader_setcracktextureparams_rage_grctexture_ptr_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64); }
extern "C" { pub fn f_064698_cshaderlib_setglobalalpha_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_064700_cshaderlib_setglobalemissivescale_float_bool(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_064701_cshaderlib_setglobalnaturalambientscale_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_064702_cshaderlib_setglobalartificialambientscale_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_064703_cshaderlib_setglobalininterior_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064710_cshaderlib_updateglobaldevscreensize(ph: *mut Phantom); }
extern "C" { pub fn f_064711_cshaderlib_updateglobalgamescreensize(ph: *mut Phantom); }
extern "C" { pub fn f_064721_cshaderlib_setglobaldeferredmaterial_unsigned_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_064736_csettingsmanager_csettingsmanager(ph: *mut Phantom); }
extern "C" { pub fn f_064737_csettingsmanager_getsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064768_cloadedmodelgroup_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064773_cloadedmodelgroup_merge_cloadedmodelgroup_ptr_cloadedmodelgroup(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_064774_cloadedmodelgroup_countmembers_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064775_cloadedmodelgroup_getmember_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_064776_cpopulationstreaming_requestvehicledriver_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064778_cpopulationstreaming_streamonenewcar(ph: *mut Phantom); }
extern "C" { pub fn f_064789_cpopulationstreaming_canpedmodelridebike_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064790_cloadedmodelgroup_ismemberingroup_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_064798_cloadedmodelgroup_addmember_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064804_cpopulationstreaming_canspawninmp_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064813_cpopulationstreaming_isfallbackpedavailable_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064815_cloadedmodelgroup_pickrandomcarmodel_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_064816_cloadedmodelgroup_removemodelsnotingroup_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_064817_cloadedmodelgroup_removemodelsthathavehitmaximumnumber(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064818_cloadedmodelgroup_removevehmodelswithflagset_cvehiclemodelinfofl(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_064828_cpopulationstreaming_iscardiscarded_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064829_cpopulationstreaming_setvehmemorybudgetlevel_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064831_cpopulationstreaming_flushallvehiclemodelshard(ph: *mut Phantom); }
extern "C" { pub fn f_064832_cstreaming_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064860_cprioritizedclipsetrequestmanager_request_cprioritizedclipsetreq(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_064913_ckeyboard_getkeyjustdown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_064929_cstreaming_setobjectisdeletable_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064932_cstreaming_setdonotdefrag_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064937_cstreaming_loadobject_rage_strlocalindex_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_065356_ccontrol_getmousesteeringmode_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_065384_ccontrol_isinputdisabled_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_065389_ccontrol_startplayerpadshake_unsigned_int_int_unsigned_int_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_065398_ccontrol_setvehiclesteeringexclusive(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065399_ccontrol_setvehicleflysteeringexclusive(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065400_ccontrol_setvehiclesubsteeringexclusive(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065429_ccontrol_applydirectionalforce_float_float_int(ph: *mut Phantom); }
extern "C" { pub fn f_065446_ccontrolmgr_getmainplayercontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065460_ccontrolmgr_getmaincameracontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065461_ccontrolmgr_getmainfrontendcontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065469_ccontrolmgr_startplayerpadshakebyintensity_unsigned_int_float_in(ph: *mut Phantom, a0: i32, a1: f32, a2: i32); }
extern "C" { pub fn f_065615_cfilemgr_openfile_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_065616_cfilemgr_readline_void_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_065617_cfilemgr_setdir_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065621_cfilemgr_openfileforwriting_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065778_rage_atmap_rage_atnonfinalhashstring_rage_controlinput_dynamicma(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065889_networkinterface_getnetworktime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_066335_rage_vec_v3anglezv_float_vector_4_float_vector_4(ph: *mut Phantom, a0: u128, a1: u128) -> u128; }
extern "C" { pub fn f_066691_ccover_addcoverpointfromvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066693_rage_vector3_angle_rage_vector3_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_067917_ctaskcomplexcontrolmovement_ctaskcomplexcontrolmovement_ctask_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_067964_cclimbhandholddetected_dtorcclimbhandholddetected(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068035_ctaskunalerted_setscenariochainuserinfo_cscenariopointchainusein(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068048_ctaskwander_ctaskwander_float_float_cconditionalanimsgroup_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i32, a5: i32, a6: f32, a7: i32, a8: i32) -> i64; }
extern "C" { pub fn f_068126_ctaskdonothing_ctaskdonothing_int_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_068132_ctaskpause_ctaskpause_int_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068161_ctasksequencelist_ctasksequencelist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068163_ctasksequencelist_addtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068203_ctaskusesequence_ctaskusesequence_ctasksequencelist_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_068455_ctaskmotionbase_setinvehiclecontexthash_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_068458_ctaskmotionbase_resetinvehiclecontexthash_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068751_ctaskmotionaiming_isplayingtransition_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068898_ctaskmotioninturret_getadjuststepmoveratio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069048_ctaskfollowwaypointrecording_calculateprogressfromposition_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_069239_rage_fwmatrixtransform_getroll_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069477_ctaskmovefollownavmesh_ctaskmovefollownavmesh_float_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: i64, a8: f32, a9: i32) -> i64; }
extern "C" { pub fn f_069608_ctaskparachute_isparachute_cobject_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069613_ctaskparachuteobject_isout_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069867_ctaskhelipassengerrappel_ctaskhelipassengerrappel_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i64; }
extern "C" { pub fn f_070399_ctasknmcontrol_ctasknmcontrol_unsigned_int_unsigned_int_rage_ait(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: f32) -> i64; }
extern "C" { pub fn f_070401_ctasknmbehaviour_canuseragdoll_cped_ptr_eragdolltriggertypes_cen(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_070530_ctasknmbalance_ctasknmbalance_unsigned_int_unsigned_int_centity(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64, a6: f32, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_070561_ctasknmbrace_ctasknmbrace_unsigned_int_unsigned_int_centity_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_070647_ctasknmflinch_ctasknmflinch_unsigned_int_unsigned_int_rage_vecto(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i64, a10: i64, a11: i64, a12: i64) -> i64; }
extern "C" { pub fn f_070739_ctasknmrelax_ctasknmrelax_unsigned_int_unsigned_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_070913_ctasksmartflee_ctasksmartflee_caitarget_const_ref_float_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_072481_cscenariopoint_getposition_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072505_cscenariopointmanager_getnumregions_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072506_cscenariopointmanager_getregion_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_072507_cscenariochaininggraph_registerchainuserdummy_cscenariopoint_con(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_072508_cscenariochaininggraph_updatedummychainuser_cscenariopointchainu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_072509_cscenariochaininggraph_unregisterpointchainuser_cscenariopointch(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072511_cscenariochaininggraph_updatenewlyloadedscenarioregions(ph: *mut Phantom); }
extern "C" { pub fn f_072516_cparkingspacefreefromobstructionstesthelper_resettest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072517_cparkingspacefreefromobstructionstesthelper_gettestresults_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_072518_cparkingspacefreefromobstructionstesthelper_isparkingspacefreefr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_072524_cscenariopoint_initarchetypeextensionfromdefinition_rage_fwexten(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072528_cscenariopoint_dtorcscenariopoint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072539_cscenariomanager_getscenarioinfo_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_072551_cscenariomanager_allowextrachaineduser_cscenariopoint_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072556_cscenariomanager_populatecargenvehicle_cvehicle_ref_ccargenerato(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_072557_cscenariospawnhistory_add_cvehicle_ref_cscenariopoint_ref_int_cs(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_072558_cscenariomanager_setupscenarioandtask_int_rage_vector3_ref_float(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_072560_cscenariomanager_createscenarioped_int_cscenariopoint_ref_ctask(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i64, a11: i64, a12: i32, a13: i64) -> i64; }
extern "C" { pub fn f_072562_cscenariopoint_getcargen_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072567_cscenariomanager_getpedtypeforscenario_int_cscenariopoint_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_072571_cscenariomanager_getvehiclescenariodistscalefactorformodel_rage(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_072573_cscenariomanager_getrandomdrivermodelindex_rage_fwmodelid_cambie(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_072626_rage_matrix34_rotatefullz_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_072669_cscenariopoint_hastimeoverride_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072670_cscenariopointmanager_getregionname_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_072785_cscenariomanager_getcargentrailertouse_ccargenerator_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_072826_cscenariomanager_checkcargenscenarioconditionsa_ccargenerator_co(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072827_cscenariomanager_getscenariopointforcargen_ccargenerator_const_r(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072828_cscenariomanager_applyvehiclecolors_cvehicle_ref_cambientvehicle(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072829_cscenariomanager_getscenariovehicleinfoforcargen_ccargenerator_c(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072833_cscenariomanager_givescenariotodrivingcar_cvehicle_ptr_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073132_cscenariocondition_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_073598_cpathfind_isregionloaded_rage_cnodeaddress_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_074161_ctask_ctask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_074163_ctaskclassinfomanager_gettaskname_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_075332_ctaskfsmclone_ctaskfsmclone(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075342_ctaskhelperfsm_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075348_cmovenetworkhelper_getboolean_rage_fwmvtemplateid_rage_fwmvboole(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075349_cmovenetworkhelper_getfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_075389_cnavmeshroutesearchhelper_cnavmeshroutesearchhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075390_cnavmeshroutesearchhelper_dtorcnavmeshroutesearchhelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075391_cnavmeshroutesearchhelper_startsearch_cped_ptr_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i64, a6: i32, a7: i64, a8: f32, a9: i32); }
extern "C" { pub fn f_075392_cnavmeshroutesearchhelper_getsearchresults_searchstatus_ref_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_075393_cboatavoidancehelper_isblocked_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075394_cboatavoidancehelper_setreverse_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_075395_cboatavoidancehelper_isavoiding_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075396_cboatavoidancehelper_isbeached_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075397_cnavmeshlineofsighthelper_resettest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075398_cvehiclefollowroutehelper_getvehiclebonnetposition_cvehicle_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_075399_cnavmeshlineofsighthelper_gettestresults_searchstatus_ref_bool_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_075400_cnavmeshlineofsighthelper_startlostest_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: f32); }
extern "C" { pub fn f_075401_cboatfollowroutehelper_cboatfollowroutehelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075402_cboatfollowroutehelper_computetarget_svehiclemissionparams_ref_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075403_cboatfollowroutehelper_computetarget_rage_vector3_ref_cvehicle_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075404_cboatfollowroutehelper_islastsegment_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075406_cpathnoderoutesearchhelper_cpathnoderoutesearchhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075407_cpathnoderoutesearchhelper_dtorcpathnoderoutesearchhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075411_cpathnoderoutesearchhelper_getnextnode_int_int_short_ref_bool_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i64); }
extern "C" { pub fn f_075413_cpathnoderoutesearchhelper_verifynodesareloadedforarea_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075414_cpathnoderoutesearchhelper_startsearch_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64); }
extern "C" { pub fn f_075415_cpathnoderoutesearchhelper_getzerothlanepositionsfornodeswithlin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_075416_cpathnoderoutesearchhelper_findupcomingjunctionturndirection_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_075417_cpathnoderoutesearchhelper_findjunctionturndirection_rage_cnodea(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: f32, a7: i64) -> i32; }
extern "C" { pub fn f_075419_helperisvehicleinlanesegment_rage_cnodeaddress_const_ref_rage_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_075420_cvehiclefollowroutehelper_getvehiclebonnetpositionfornavmeshquer(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075421_cvehiclefollowroutehelper_getvehicledrivedir_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075422_cvehiclefollowroutehelper_cvehiclefollowroutehelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075423_cvehiclelanechangehelper_shouldchangelanesforthisentity_cvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_075424_cvehiclelanechangehelper_issegmentobstructed_int_cvehiclefollowr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: f32, a5: i64, a6: i64, a7: i64, a8: i32) -> i32; }
extern "C" { pub fn f_075425_cvehicleconvertibleroofhelper_isroofdoinganimatedtransition_cveh(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075426_cvehicleconvertibleroofhelper_potentiallyraiseorlowerroof_cvehic(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_075427_cvehiclefollowroutehelper_constructfromnodelist_cvehicle_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_075428_cvehiclefollowroutehelper_calculatedesiredspeeds_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_075429_cvehiclefollowroutehelper_constructfromwaypointrecording_cvehicl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32); }
extern "C" { pub fn f_075430_cvehiclefollowroutehelper_findclosestpointtoposition_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_075431_cvehiclefollowroutehelper_getboundarysegmentsbetweenpoints_int_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64, a7: i64, a8: i64, a9: f32, a10: f32) -> i32; }
extern "C" { pub fn f_075432_croutepoint_findroadboundaries_float_ref_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075434_cvehiclefollowroutehelper_estimatepathlength_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_075435_cvehiclefollowroutehelper_finddistancetoendofpath_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_075436_cvehiclelanechangehelper_canswitchlanestoavoidobstruction_cvehic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_075437_cvehiclefollowroutehelper_findtargetcoorsandspeed_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32) -> f32; }
extern "C" { pub fn f_075439_cvehiclefollowroutehelper_processsuperdummy_cvehicle_ptr_rage_sc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_075441_cvehiclefollowroutehelper_computeonroadposition_cvehicle_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_075442_cnavmeshlineofsighthelper_cnavmeshlineofsighthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075446_casyncprobehelper_dtorcasyncprobehelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075447_casyncprobehelper_resetprobe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075448_casyncprobehelper_starttestlineofsight_worldprobe_cshapetestprob(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075449_casyncprobehelper_getproberesults_probestatus_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075452_casyncprobehelper_getproberesultsvfx_probestatus_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64) -> i32; }
extern "C" { pub fn f_075470_cmovenetworkhelper_cmovenetworkhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075474_cmovenetworkhelper_setfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_075484_cgenericcliphelper_cgenericcliphelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075487_cgenericcliphelper_blendinclipbyclip_cdynamicentity_ptr_rage_crc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_075488_cgenericcliphelper_blendinclipbydictandhash_cdynamicentity_ptr_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_075489_cgenericcliphelper_blendoutclip_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_075523_ctuning_dtorctuning(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075541_cvehiclejunctionhelper_processtrafficlights_cvehicle_ptr_float_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_075542_cvehiclejunctionhelper_approachingredlight_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075544_cvehiclewaypointrecordinghelper_cvehiclewaypointrecordinghelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075545_cvehiclewaypointrecordinghelper_dtorcvehiclewaypointrecordinghel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075546_cvehiclewaypointrecordinghelper_init_cwaypointrecordingroute_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_075549_cvehicledrifthelper_update_float_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_075568_cfindnearestcarnodehelper_cfindnearestcarnodehelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075569_cfindnearestcarnodehelper_getnearestcarnode_rage_cnodeaddress_re(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075570_cfindnearestcarnodehelper_setsearchposition_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075571_cfindnearestcarnodehelper_generaterandompositionnearroadnode_cfi(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075599_ctimehelpers_gettimesince_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_075604_ctaskcontrolvehicle_ctaskcontrolvehicle_cvehicle_ptr_rage_aitask(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_075606_cvehiclestuckduringcombathelper_isstuck_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075614_ctargetinwaterhelper_isinwater_centity_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075615_ccreekhackhelpergtav_isaffected_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075627_cvehicletaskmanager_dtorcvehicletaskmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075632_ctaskmove_dtorctaskmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075659_ctasktypes_isplayerdrivetask_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_075660_ctuning_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_075664_ctuning_registertuning_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_075697_ctaskexitvehicle_ctaskexitvehicle_cvehicle_ptr_cvehicleenterexit(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_075702_ctaskinvehiclebasic_ctaskinvehiclebasic_cvehicle_ptr_bool_ccondi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_075710_ctaskvehiclegun_ctaskvehiclegun_ctaskvehiclegun_etaskmode_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_075724_ctaskentervehicle_ctaskentervehicle_cvehicle_ptr_seatrequesttype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: f32, a6: f32, a7: i64, a8: i64) -> i64; }
extern "C" { pub fn f_075753_ctaskinvehicleseatshuffle_ctaskinvehicleseatshuffle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_075758_ctaskvehiclefsm_getpedinorusingseat_cvehicle_const_ref_int_cped(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_075798_cphysical_isbrokenflagset_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075857_ctaskvehiclefsm_shoulddestroyvehicleweaponmanager_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075870_ctaskvehiclefsm_isvehiclelockedforplayer_cvehicle_const_ref_cped(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075974_ctaskvehiclefsm_getdoorforentrypointindex_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_075992_ctaskexitvehicle_isvehicleonground_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076093_ctaskexitvehicle_isverticalprobeclear_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_076123_ctaskexitvehicleseat_checkforwaterunderped_cped_const_ptr_cvehic(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_076178_rage_phcollider_applyimpulse_float_vector_4_float_vector_4_int_f(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: i32); }
extern "C" { pub fn f_076290_ctaskvehiclefsm_getentryradiusmultiplier_centity_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_076294_ctaskvehiclefsm_getdoorforentrypointindex_cvehicle_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_076296_ctaskattachedvehicleanimdebug_ctaskattachedvehicleanimdebug_cveh(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_076334_ctaskvehiclemountedweapon_computesearchtarget_rage_vector3_ref_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076406_ctaskmotioninautomobile_isfirstpersondrivingbicycle_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076411_ctaskmotioninautomobile_isvehicledrivable_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076457_ctaskmotiononbicycle_wantstodospecialmove_cped_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_076520_cvehiclecomponentvisualiser_visualisecomponents_cvehicle_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077089_cstatsmgr_incrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077095_cstatsmgr_isplayeractive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077096_cphysical_getdistancetravelled_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_077122_cstatsmgr_registerexplosionhit_centity_const_ptr_centity_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_077124_cstatsmgr_registervehicleblownupbyplayer_cvehicle_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_077125_cstatsmgr_registervehicledamage_cvehicle_ptr_centity_ptr_edamage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_077153_statid_char_getstatid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077331_statsinterface_getintstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077461_moneyinterface_memorytampering_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077479_ctextfile_get_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077481_ctextfile_get_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_077500_cmessages_clearalldisplayedmessagesthatbelongtothistextblock_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_077504_cmessages_addmessage_char_const_ptr_int_unsigned_int_bool_bool_e(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i64, a9: i32, a10: i32, a11: i32); }
extern "C" { pub fn f_077508_chelpmessage_clear_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_077509_chelpmessage_setmessagetextandaddtobrief_int_char_const_ptr_cnum(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077565_ctextfile_doestextlabelexist_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077575_ctextfile_requestadditionaltext_char_const_ptr_int_ctextfile_elo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077576_ctextfile_freetextslot_int_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077578_ctextfile_hasthisadditionaltextloaded_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077581_ctextfile_hasadditionaltextloaded_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077582_ctextfile_isrequestingthisadditionaltext_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077603_ctimecycle_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077612_ctimecycle_calcambientscale_rage_vec3v_const_ref_rage_fwinterior(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077620_ctimecycle_setshaderdata_cvisualeffects_erendermode(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077621_cvisualsettings_get_char_const_ptr_char_const_ptr_float_bool_con(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> f32; }
extern "C" { pub fn f_077622_cvisualsettings_get_char_const_ptr_float_bool_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_077636_cvisualsettings_loadall(ph: *mut Phantom); }
extern "C" { pub fn f_077637_cvisualsettings_get_unsigned_int_float_bool_const(ph: *mut Phantom, a0: i32, a1: f32) -> f32; }
extern "C" { pub fn f_077638_cvisualsettings_getcolor_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077742_cdriverpersonality_finddriverability_cped_const_ptr_cvehicle_con(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_077743_cdriverpersonality_finddriveraggressiveness_cped_const_ptr_cvehi(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_077744_cdriverpersonality_findmaxacceleratorinput_cped_const_ptr_cvehic(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_077745_cdriverpersonality_findmaxcruisespeed_cped_const_ptr_cvehicle_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> f32; }
extern "C" { pub fn f_077746_cdriverpersonality_finddelaybeforeacceleratingafterobstructiongo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_077748_cdriverpersonality_willchangelanes_cped_const_ptr_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077750_cflyingvehicleavoidancemanager_updateplaneavoidance_bool_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077751_rage_atvector_cflyingvehicleavoidancemanager_closevehicle_0_unsi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077752_cflyingvehicleavoidancemanager_checkfuturecollisions_bool_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077753_cflyingvehicleavoidancemanager_steeravoidvehicle_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_077754_cflyingvehicleavoidancemanager_steeravoidothers_bool_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077755_cflyingvehicleavoidancemanager_steerdesiredvelocity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077757_cflyingvehicleavoidancemanager_slidedestinationtarget_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_077758_cflyingvehicleavoidancemanager_removevehicle_rage_fwregdref_cveh(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077759_cflyingvehicleavoidancemanager_addvehicle_rage_fwregdref_cvehicl(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077789_cjunction_findentranceindexwithnode_rage_cnodeaddress_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077790_cjunction_calculatedistancetojunction_cvehicle_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_077791_cpathfind_checkpathnodeintegrityforzone_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077792_cpathfind_testalllinks_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077793_cjunction_removevehiclefromslot_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077794_cvehicleintelligence_findspeedmultiplierwithspeedfromnodes_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_077797_cvehicleintelligence_cachejunctionturndirection(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077799_cvehiclenodelist_getpathnode_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_077801_cvehicleintelligence_getnodelist_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077802_cvehicleintelligence_getfollowroutehelper_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077803_cvehicleintelligence_getactivetask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077804_cjunction_findentryandexitnodes_cvehicle_const_ptr_rage_cnodeadd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_077805_cjunction_addvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077810_cpathfind_arenodesloadedforarea_float_float_float_float_const(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_077811_cpathfind_findnodeclosesttocoors_rage_vector3_const_ref_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32); }
extern "C" { pub fn f_077812_cpathfind_findnodeclosesttocoors_rage_vector3_const_ref_float_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: f32, a11: i32, a12: i64, a13: i32, a14: i32); }
extern "C" { pub fn f_077813_cpathfind_findnodeslinkindexbetween2nodes_rage_cnodeaddress_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_077821_cjunctions_findjunctionfromnode_rage_cnodeaddress_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077827_cjunctions_getjunctionbyindex_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077831_cvehicle_updatejunctions(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077832_cvehiclenodelist_findlastgoodnode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077862_cvehiclenodelist_findlaneswithnodes_int_rage_vector3_const_ptr_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_077863_cvehiclenodelist_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077864_cpathfind_findnodeslinkindexbetween2nodes_rage_cpathnode_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_077865_markplayersroadnodes_onvisitnode_rage_cpathnode_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077866_markplayersroadnodes_shouldvisitadjacentnode_rage_cpathnode_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_077874_cpathfind_findroadboundaries_int_int_float_bool_bool_float_ref_f(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: i32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_077875_cpathfind_findxcoorsforregion_int_const(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_077879_cpathfind_findstartandendnodebasedonyrange_int_float_float_int_r(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: i64, a4: i64); }
extern "C" { pub fn f_077880_cpathfind_findxregionforcoors_float_const(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_077885_cpathfind_find2nodesforcarcreation_rage_vector3_const_ref_rage_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_077887_cpathfind_findregionlinkindexbetween2nodes_rage_cnodeaddress_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077888_cpathfind_findlinkbetween2nodes_rage_cpathnode_const_ptr_rage_cp(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077892_cpathfind_findnodeindirection_cpathfind_findnodeindirectioninput(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077901_cpathfind_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077902_cpathfind_ispositionoffroadgivennodelist_cvehicle_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_077903_cvehiclenodelist_getclosestposonpath_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_077917_cpathzonearray_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_077921_cpathzonemappingarray_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_077930_cvehicleailod_getdummymode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077932_cvehicleailodmanager_setlodflag_cvehicle_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077933_cvehicleailodmanager_clearlodflag_cvehicle_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077934_cvehicleailodmanager_shoulddofullupdate_cvehicle_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077935_cvehicleailodmanager_computerealdriversandpassengersremaining(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077937_cvehicleailodmanager_calcdesiredlodforpoint_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077940_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_cvehicl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_077942_cvehicleintelligence_cvehicleintelligence_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077943_cvehicleintelligence_setcarwearebehind_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077945_cvehicleintelligence_dtorcvehicleintelligence_126646(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077946_cvehicleintelligence_hasbeenslowlypushingplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077947_cvehicleintelligence_resetvariables_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_077948_cvehicleintelligence_process_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_077949_cvehicleintelligence_invalidatecachednodelist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077950_cvehicleintelligence_invalidatecachedfollowroute(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077951_cvehicleintelligence_process_ondeferredtaskcompletion(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077953_cvehicleintelligence_arewebehindcar_cvehicle_const_ptr_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_077954_cvehicleintelligence_allowcaraidebugdisplayforthisvehicle_cvehic(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077955_aideferredtasks_taskdata_taskdata_ctask_ptr_cvehicle_ptr_float_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_077956_cvehicleintelligence_cachenodelist_ctaskvehiclemissionbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077957_cvehicleintelligence_cachefollowroute_ctaskvehiclemissionbase_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077959_cvehicleintelligence_isonhighway_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077960_cvehicleintelligence_isonsingletrackroad_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077961_cvehicleintelligence_isonsmallroad_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077963_rage_atarray_cvehicleintelligence_sdebug_routeinterceptiontest_s(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077964_cvehicleintelligence_findtargetcoorswithnode_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32, a8: i64); }
extern "C" { pub fn f_077965_cvehicleintelligence_checkentityinterceptionxy_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i64) -> i32; }
extern "C" { pub fn f_077966_cvehicleintelligence_doesroutepassbyposition_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_077967_cvehicleintelligence_updatejustbeengivennewcommand(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077968_cvehicleintelligence_getactivetasksimplest_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077970_cvehicleintelligence_getgototaskforvehicle_cvehicle_ptr_cphysica(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: f32, a6: f32) -> i64; }
extern "C" { pub fn f_077971_ctaskvehiclecruisenew_ctaskvehiclecruisenew_svehiclemissionparam(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_077972_ctaskvehiclemissionbase_ctaskvehiclemissionbase_svehiclemissionp(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077973_ctaskvehiclegotosubmarine_ctaskvehiclegotosubmarine_svehiclemiss(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32) -> i64; }
extern "C" { pub fn f_077974_ctaskvehiclegotohelicopter_ctaskvehiclegotohelicopter_svehiclemi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_077975_cvehicleintelligence_createautomobilegototask_svehiclemissionpar(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_077976_cvehicleintelligence_clearscanners(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077977_ctaskvehicleattack_ctaskvehicleattack_svehiclemissionparams_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077978_ctaskvehiclecrash_ctaskvehiclecrash_centity_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_077982_cvehicleintelligence_createcruisetask_cvehicle_ref_svehiclemissi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_077983_ctaskvehicleflee_ctaskvehicleflee_svehiclemissionparams_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_077985_ctaskvehicleapproach_ctaskvehicleapproach_svehiclemissionparams(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077986_ctaskvehicleblock_ctaskvehicleblock_svehiclemissionparams_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077987_ctaskvehicledeaddriver_ctaskvehicledeaddriver_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077988_ctaskvehiclefollow_ctaskvehiclefollow_svehiclemissionparams_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_077989_ctaskvehiclegotolongrange_ctaskvehiclegotolongrange_svehiclemiss(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077990_ctaskvehiclegotonavmesh_ctaskvehiclegotonavmesh_svehiclemissionp(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077991_ctaskvehiclegotoplane_ctaskvehiclegotoplane_svehiclemissionparam(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: f32) -> i64; }
extern "C" { pub fn f_077992_ctaskvehiclegotoboat_ctaskvehiclegotoboat_svehiclemissionparams(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_077993_ctaskvehicleheliprotect_ctaskvehicleheliprotect_svehiclemissionp(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_077994_ctaskvehiclehover_ctaskvehiclehover(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077995_ctaskvehiclenodriver_ctaskvehiclenodriver_ctaskvehiclenodriver_n(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_077996_ctaskvehicleparknew_ctaskvehicleparknew_svehiclemissionparams_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i32) -> i64; }
extern "C" { pub fn f_077997_ctaskvehicleplanechase_ctaskvehicleplanechase_svehiclemissionpar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_077998_ctaskvehicleplayerdriveautomobile_ctaskvehicleplayerdriveautomob(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077999_ctaskvehicleplayerdriveamphibiousautomobile_ctaskvehicleplayerdr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078000_ctaskvehicleplayerdrivebike_ctaskvehicleplayerdrivebike(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078001_ctaskvehicleplayerdriveboat_ctaskvehicleplayerdriveboat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078002_ctaskvehicleplayerdrivesubmarine_ctaskvehicleplayerdrivesubmarin(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078003_ctaskvehicleplayerdrivesubmarinecar_ctaskvehicleplayerdrivesubma(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078004_ctaskvehicleplayerdriveplane_ctaskvehicleplayerdriveplane(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078005_ctaskvehicleplayerdriveheli_ctaskvehicleplayerdriveheli(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078006_ctaskvehicleplayerdriveautogyro_ctaskvehicleplayerdriveautogyro(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078007_ctaskvehicleplayerdrivetrain_ctaskvehicleplayerdrivetrain(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078008_ctaskvehiclepullover_ctaskvehiclepullover_bool_ctaskvehiclepullo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_078009_ctaskvehiclepursue_ctaskvehiclepursue_svehiclemissionparams_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> i64; }
extern "C" { pub fn f_078010_ctaskvehicleram_ctaskvehicleram_svehiclemissionparams_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_078011_ctaskvehiclereacttocopsiren_ctaskvehiclereacttocopsiren_ctaskveh(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_078012_ctaskvehiclestop_ctaskvehiclestop_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_078013_cvehicleintelligence_multiplycruisespeedwithmultiplierandclip_fl(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i32, a4: i64) -> f32; }
extern "C" { pub fn f_078014_cvehicleintelligence_updateisthisadriveablecar(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078019_vehiclenavigationhelper_helperexcludecollisionentities_worldprob(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078020_vehiclenavigationhelper_cannavigateoverobject_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_078021_vehiclenavigationhelper_worldcollisiontest_rage_vector3_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_078022_vehiclenavigationhelper_cannavigatestraightto_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_078023_vehiclenavigationhelper_update_float_rage_vector3_const_ref_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32); }
extern "C" { pub fn f_078024_cvehiclenodelist_cvehiclenodelist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078026_cvehiclenodelist_clearfrom_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078028_cvehiclenodelist_findnumpathnodes_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078037_aideferredtasks_deferredtask_createworkersandwait_void_ptr_aidef(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078038_aideferredtasks_vehicleavoidancetask_run(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078041_ctaskvehiclegotopointwithavoidanceautomobile_gotopointwithavoida(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078042_ctaskvehiclegotopointwithavoidanceautomobile_gotopointwithavoida(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078043_ctaskvehiclegotopointwithavoidanceautomobile_gotopointwithavoida(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078044_ctaskvehiclegotopointwithavoidanceautomobile_makewayforcarwithsi(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078045_aideferredtasks_vehiclegototask_run(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078047_ctaskvehiclegotopointautomobile_gotopoint_ondeferredtask_aidefer(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078048_ctaskvehiclegotopointautomobile_humanisecarcontrolinput_cvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32); }
extern "C" { pub fn f_078049_aideferredtasks_vehicleintelligenceprocesstask_run(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078051_aideferredtasks_vehicleinterpolationtask_run(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078053_aideferredtasks_runall(ph: *mut Phantom); }
extern "C" { pub fn f_078056_ctaskvehiclemissionbase_dtorctaskvehiclemissionbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078060_ctaskvehicleconvertibleroof_ctaskvehicleconvertibleroof_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_078063_ctaskvehicleconvertibleroof_getroofstate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078064_ctaskvehicleconvertibleroof_getroofprogress(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_078066_ctaskvehicleconvertibleroof_lowerroof_onenter_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078067_ctaskvehicleconvertibleroof_raiseroof_onenter_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078068_ctaskvehicleconvertibleroof_latchroof_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078069_ctaskvehicleconvertibleroof_unlatchroof_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078072_ctaskvehicletransformtosubmarine_ctaskvehicletransformtosubmarin(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_078073_ctaskvehicletransformtosubmarine_gettransformstate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078090_ctaskvehicleattack_fireattarget_cped_ptr_cvehicle_ptr_cvehiclewe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_078091_ctaskvehicleattack_stop_onenter_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078092_ctaskvehicleattack_selectbestvehicleweapon_cvehicle_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078093_ctaskvehicleattack_isabletofireweapons_cvehicle_ref_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_078094_ctaskvehiclemissionbase_gettargetentity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078112_ctaskvehicleblock_getinfront_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078125_ctaskvehicleblockbrakeinfront_getinposition_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078126_ctaskvehicleblockbrakeinfront_calculatefuturecollisiondistancean(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_078127_ctaskvehiclemissionbase_humanisecarcontrolinput_cvehicle_ptr_cve(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078140_ctaskvehicleflee_shouldwaitforaccident_cvehicle_ptr_rage_vec3v_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_078143_ctaskvehiclecruisenew_dtorctaskvehiclecruisenew(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078144_ctaskvehiclecruisenew_setpreferredtargetnodelist_cvehiclenodelis(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078148_ctaskvehiclecruisenew_attemptlanechange_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_078149_ctaskvehiclecruisenew_processprefsm_shared(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078150_ctaskvehiclecruisenew_updatespeedmultiplier_cvehicle_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078154_ctaskvehiclecruisenew_updateusingwanderfallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078155_ctaskvehiclecruisenew_addadditionalsearchflags_cvehicle_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_078156_ctaskvehiclecruisenew_getmintimetofindnewpathaftersearch_cvehicl(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078157_ctaskvehiclecruisenew_updatenavmeshlostestforcruising_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_078158_ctaskvehiclecruisenew_findtargetpositionandcapspeed_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> f32; }
extern "C" { pub fn f_078159_ctaskvehiclecruisenew_updateshouldfindnewroute_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_078160_ctaskvehiclecruisenew_isescorting_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078161_ctaskvehiclecruisenew_canusenavmeshtoreachpoint_cvehicle_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_078162_ctaskvehiclecruisenew_navmesh_shared_onupdate_rage_vector3_ref_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_078163_ctaskvehiclecruisenew_gettargetpositionforgettoroadusingnavmesh(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_078165_ctaskvehiclecruisenew_debug_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078166_ctaskvehiclemissionbase_gettargetposition(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078180_ctaskvehiclecruiseboat_pickcruisepoint_cvehicle_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078187_ctaskvehicledeaddriver_dtorctaskvehicledeaddriver(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078194_ctaskvehicleescort_adjusttargetpositiondependingondistance_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_078195_ctaskvehicleescort_handlebrakingifaheadoftarget_cvehicle_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_078205_ctaskvehiclefleeboat_calculatetargetcoords_cvehicle_const_ref_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_078210_ctaskvehiclecrash_dtorctaskvehiclecrash(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078212_ctaskvehiclecrash_doblowupvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078216_ctaskvehiclegotohelicopter_dtorctaskvehiclegotohelicopter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078219_ctaskvehiclegotohelicopter_steertotarget_cheli_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_078220_ctaskvehiclegotohelicopter_steertovelocity_cheli_ptr_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_078227_ctaskvehiclefleeairborne_calculatetargetcoords_cvehicle_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_078231_ctaskvehiclefollowwaypointrecording_getlookaheaddistanceforfollo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> f32; }
extern "C" { pub fn f_078237_ctaskvehiclefollowwaypointrecording_findtargetpositionandcapspee(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> f32; }
extern "C" { pub fn f_078244_ctaskvehiclegotopointwithavoidanceautomobile_dtorctaskvehiclegot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078247_ctaskvehiclegotopointwithavoidanceautomobile_wait_onupdate_cvehi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078248_ctaskvehiclethreepointturn_shoulddothreepointturn_cvehicle_ptr_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_078249_ctaskvehiclegotopointwithavoidanceautomobile_slowcardownforother(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32, a8: i64, a9: f32, a10: i32, a11: i32, a12: i32); }
extern "C" { pub fn f_078250_ctaskvehiclegotopointwithavoidanceautomobile_slowcardownforpedss(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: i64, a7: f32, a8: i32); }
extern "C" { pub fn f_078251_helpermaybeplayhorn_cvehicle_ptr_bool_bool_cphysical_const_ptr_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_078252_helpergetheliavoidancebounds_cvehicle_const_ptr_rage_spdaabb_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078253_ctaskvehiclegotopointwithavoidanceautomobile_test2movingrectcoll(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64, a6: i64, a7: i64, a8: i32) -> f32; }
extern "C" { pub fn f_078254_ctaskvehiclegotopointwithavoidanceautomobile_test2movingrectcoll(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64, a6: i64, a7: i64, a8: i32) -> f32; }
extern "C" { pub fn f_078255_ctaskvehiclegotopointwithavoidanceautomobile_isthereroomtooverta(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_078256_helpermaybedolonghorn_cvehicle_const_ptr_cvehicle_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_078257_avoidancetaskobstructiondata_debugdraw_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078258_helperdodrivebyontarget_cvehicle_ptr_cped_ptr_cphysical_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_078259_ctaskvehiclegotopointwithavoidanceautomobile_testdirectionsagain(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64, a7: i32) -> i32; }
extern "C" { pub fn f_078260_ctaskvehiclegotopointwithavoidanceautomobile_scoreoneobstruction(ph: *mut Phantom, a0: i32, a1: f32, a2: i64, a3: i64, a4: f32) -> f32; }
extern "C" { pub fn f_078261_ctaskvehiclegotopointwithavoidanceautomobile_shouldignorenondirt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_078262_ctaskvehiclegotopointwithavoidanceautomobile_shouldnotavoidvehic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_078263_helpershouldvehiclestopvsswervefordoor_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_078264_ctaskvehiclegotopointwithavoidanceautomobile_findavoidancepoints(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_078268_ctaskvehiclegotopointautomobile_calculatemaximumthrottlebasedont(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_078274_ctaskvehiclethreepointturn_ispointonroad_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078276_ctaskvehiclegotonavmesh_findtargetpositionandcapspeed_cvehicle_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> f32; }
extern "C" { pub fn f_078286_ctaskvehiclegotoboat_shouldusethreepointturn_cvehicle_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_078298_ctaskvehiclegotolongrange_getextentsforrequirednodestoload_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_078309_ctaskvehiclegotoplane_vehicle_testforfutureworldcollisions_cphys(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> f32; }
extern "C" { pub fn f_078310_ctaskvehiclegotoplane_plane_modifytargetforarrivaltolerance_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_078311_ctaskvehiclegotoplane_computeapproachtangentposition_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32); }
extern "C" { pub fn f_078312_ctaskvehiclegotoplane_vehicle_worldcollsiontest_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_078318_ctaskvehiclegotosubmarine_dtorctaskvehiclegotosubmarine(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078322_ctaskvehiclegotosubmarine_computetargetztoavoidheightmap_csubmar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64) -> f32; }
extern "C" { pub fn f_078323_ctaskvehiclegotosubmarine_updateobjectavoidance_csubmarine_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_078351_ctaskvehiclepullover_getpointtoaimfor_cvehicle_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> f32; }
extern "C" { pub fn f_078357_ctaskvehicleparknew_dtorctaskvehicleparknew(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078362_ctaskvehicleparknew_helpergetpositiontosteerto_cvehicle_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_078363_ctaskvehicleparknew_getdistanceinfrontofspace_cvehicle_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_078364_ctaskvehicleparknew_iscurrentpositionwithinxtolerance_cvehicle_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_078365_ctaskvehicleparknew_helpergetbackawayfromspaceposition_cvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_078372_ctaskvehicleplayerdrive_drive_onupdate_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078373_ctaskvehicleplayerdrive_nodriver_onenter_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078374_ctaskvehicleplayerdrive_istheplayercontrolinactive_ccontrol_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078375_ctaskvehicleplayerdrive_drive_onenter_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078376_ctaskvehicleplayerdrive_processplayercontrolinputsforbrakeandgas(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_078379_ctaskvehicleplayerdriveautomobile_processdriverinputsforplayeron(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_078396_ctaskvehicleplayerdriverotarywingaircraft_processdriverinputsfor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_078403_ctaskvehiclenodriver_processsub_csubmarine_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078407_ctaskvehicleplanechase_dtorctaskvehicleplanechase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078411_ctaskvehicleplanechase_calculatetargetfutureposition_float_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078412_ctaskvehicleplanechase_calculateevadetargetposition(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078426_ctaskvehiclepullalongside_getinposition_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078429_ctaskvehiclepursue_dtorctaskvehiclepursue(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078437_ctaskvehiclepursue_isseekappropriate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078438_ctaskvehiclepursue_calculatetargetposition_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078482_ctaskvehicleflydirection_findflightheight_cplane_ptr_float(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_078483_ctaskvehicleflydirection_findheightforverticalangle_cplane_ptr_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_078492_ctaskvehiclethreepointturn_updatedesiredspeedandsteerangle_float(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_078495_cheliintelligence_dtorcheliintelligence(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078717_ctaskvehiclemissionbase_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078748_ctaskvehiclegotohelicopter_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078752_ctaskvehiclefleeairborne_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078759_ctaskvehiclegotoplane_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078766_ctaskvehicleheliprotect_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078773_ctaskvehicleparknew_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078777_ctaskvehicleplanechase_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078781_chandlingdata_getseaplanehandlingdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078783_canchorhelper_createanchorconstraints(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078784_canchorhelper_anchor_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078785_canchorhelper_cananchorhere_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078786_cvehicle_getnumcontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078787_camphibiousautomobile_ispropellersubmerged(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078788_cautomobile_cautomobile_eentityownedby_unsigned_int_vehicletype(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_078789_cvehicle_cvehicle_eentityownedby_unsigned_int_vehicletype(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_078790_camphibiousautomobile_dtorcamphibiousautomobile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078791_cautomobile_dtorcautomobile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078792_cwheel_dtorcwheel(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078793_cvehicle_dtorcvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078796_camphibiousautomobile_processlowlodbuoyancy(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078797_cautomobile_placeonroadadjustinternal_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_078798_camphibiousautomobile_computelowlodbuoyancyplanenormal_rage_matr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_078799_cboat_rejuvenatematrixifnecessary_rage_matrix34_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078800_cwheel_reset_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078801_cautomobile_placeonroadproperly_cautomobile_ptr_rage_matrix34_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: i32, a8: i64, a9: i64, a10: i32, a11: f32, a12: f32) -> i32; }
extern "C" { pub fn f_078804_cautomobile_initphys(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078805_cvehicle_istunervehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078806_cvehicle_initphys(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078808_cboathandling_updatepropeller_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078809_cautomobile_prerender_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078810_cvehicle_setbonerotation_int_erotationaxis_float_bool_rage_vecto(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_078811_cvehicledamage_prerender(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078812_ccardoor_processprerender_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078813_cvehicle_prerender_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078814_cwheel_processwheelmatrixcore_rage_crskeleton_ref_cvehiclestruct(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32); }
extern "C" { pub fn f_078815_cwheel_processwheelmatrixfortrike(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078817_cautomobile_prerender2_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078818_cautomobile_prerendersuspensionmod_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078819_cvehicle_prerender2_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078820_cvehiclelightasyncmgr_addvehicle_cvehicle_ptr_rage_fwinteriorloc(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_078821_camphibiousautomobile_processphysics_float_bool_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_078822_cboat_updatebuoyancylodmode_cvehicle_ptr_canchorhelper_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078823_cvehicle_processrespotcounter_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078824_cboathandling_processphysics_cvehicle_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_078825_cautomobile_processphysics_float_bool_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_078826_cvehicle_resetrespotcounter_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078827_cvehicle_setiswrecked(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078828_cvehicle_processisasleep(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078829_ctransmission_processboat_cvehicle_ptr_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_078830_cvehicle_processphysics_float_bool_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_078831_cvehicle_getreferenceframevelocity_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078832_ctransmission_process_cvehicle_ptr_rage_mat34v_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i64, a6: i32, a7: i32, a8: f32) -> f32; }
extern "C" { pub fn f_078833_ccardoor_processphysics_cvehicle_ptr_float_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_078834_cwheel_setbrakeanddriveforce_float_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_078835_cwheel_processasleep_rage_vector3_const_ref_float_float_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_078836_cvehicle_freeze1dofjointincurrentposition_fraginstgta_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078837_cvehicle_freeze1dofjointincurrentposition_fraginstgta_ptr_int_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_078838_cvehicle_autolevelandheading_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_078839_caircraftdamage_getyawmult_cplane_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_078840_chandlingdata_getcarhandlingdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078841_cvehicle_processslowmotionspecialabilityprephysics_cped_const_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078842_cautomobile_startwheelintegratortask_rage_phcollider_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_078844_cvehicle_updatephysicsfromentity_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078846_cautomobile_doprocesscontrol_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_078847_cboathandling_updateprow_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078848_cvehicle_getslipstreameffect_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_078849_cvehicle_processintelligence_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_078850_cvehicledamage_process_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_078851_cautomobile_updatedesiredgroundgroundclearance_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_078852_cvehicle_getisbeingtowed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078853_cvehicle_processsirenandhorn_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078854_cvehicle_processdirtlevel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078855_cvehicle_updatedangerousdriverevents_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078856_cvehicle_doprocesscontrol_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_078857_camphibiousautomobile_wantstobeawake(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078858_cvehicle_wantstobeawake(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078859_cvehicle_canbeinactiveduringrecording_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078861_cautomobile_fix_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_078862_cvehicle_fix_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_078864_cautomobile_applydeformationtobones_void_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078865_cvehicle_getdefaultboneposition_int_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_078866_cvehicle_getdefaultbonepositionforsetup_ehierarchyid_rage_vector(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_078868_cvehicledeformation_readfromtexposition_void_const_ptr_rage_vec2(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_078870_chandlingdata_getbikehandlingdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078871_cvehicle_containslocalplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078873_cvehicle_processpostphysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078875_cwheel_suspensionsetup_void_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078876_camphibiousquadbike_iswheeltransitionfinished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078879_cvehicle_processprecomputeimpacts_rage_phcontactiterator(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078880_rage_phcachedcontactconstiterator_initfromiterator_rage_phcontac(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_078881_cvehicle_processboostpadimpact_rage_phcachedcontactiterator_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_078882_cvehicle_hasincreasedrammingforce(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078883_cvehicle_addtofoliagedrag_int_rage_vector3_const_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_078884_cvehicle_hasrammingscoop_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078885_cvehicle_applyspikeimpacts_rage_phcachedcontactiterator_ref_cent(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_078886_cvehicle_canpedsstandontop_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078887_cvehicle_getchassisboundmax_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_078888_ccardoor_settargetdooropenratio_float_unsigned_int_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64); }
extern "C" { pub fn f_078889_cvehicle_getdoorhierachyidfromimpact_cvehicle_const_ref_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078890_cvehicle_getattachparentvehicle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078891_ccardoor_getislatched_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_078892_cvehicle_hasramp_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078893_cheli_getiscargobob_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078894_rage_fwtransform_untransform_rage_vec3v_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_078898_cvehiclepopulation_updatevehcount_cvehicle_ptr_cvehiclepopulatio(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078899_cvehicledamage_fixvehicledamage_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_078900_cvehicledeformation_damagetexturefree(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078901_cvehicle_destroyvehicleweaponmgr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078902_cvehicle_dummydetachchild_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078903_cvehicle_dummydetachallchildren(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078907_cautomobile_setmodelid_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078908_cvehicle_setmodelid_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078909_cvehicle_istaximodelid_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078910_cautomobile_slipperyboxupdatelimits_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078911_ctransmission_setuptransmission_float_float_float_unsigned_char(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: i64); }
extern "C" { pub fn f_078912_cvehicle_islawenforcementcarmodelid_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078913_cvehicle_islawenforcementvehiclemodelid_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078914_cvehicle_freezeallturretjoints_gtafragtype_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_078915_cvehicle_setupweaponmodcollision(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078916_cvehicleweaponmgr_createvehicleweaponmgr_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078917_cvehiclefactory_modifyvehicletopspeed_cvehicle_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078918_cvehicle_setcardoorlocks_ecarlockstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078919_cautomobile_initwheels(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078920_cwheel_cwheel(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078921_cwheel_init_cvehicle_ptr_ehierarchyid_float_float_int_signed_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: f32, a5: i32, a6: i32); }
extern "C" { pub fn f_078922_cvehicle_calculateheightsaboveroad_rage_fwmodelid_float_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_078923_cvehicledeformation_unlockdamagetexture(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078924_cvehicle_getwheelfromid_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_078926_cvehicle_calculatenonarticulatedmaximumextents_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078927_cwheel_calcwheelmatrices_rage_mat34v_ref_rage_mat34v_ref_rage_ph(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_078928_cautomobile_processdriverinputsforstability_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078929_cvehicle_isbusmodelid_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078930_cvehicle_getishandbrakepressed_ccontrol_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_078931_cvehicle_applyinternaltorqueandforce_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_078932_cvehicle_getvelocityincludingreferenceframe_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078933_cvehicle_isupsidedown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078934_cvehicle_isonitsside_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078936_cvehicle_ishornon_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078937_ccardoor_setshutandlatched_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_078939_cvehicle_switchengineon_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_078941_cvehicledeformation_applydeformations_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078942_cvehicledamage_processstuckcheck_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078943_cvehicledamage_applypadshakeinternal(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078944_cvehicledamage_applydamage_centity_ptr_edamagetype_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: i64, a6: i64, a7: i64, a8: i32, a9: i64, a10: i32, a11: i32, a12: i32, a13: f32, a14: i32, a15: i32, a16: i32, a17: i32, a18: i32) -> f32; }
extern "C" { pub fn f_078945_cvehicle_isdriveraplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078946_cvehicle_hasalivelawpedsinit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078947_cvehicledamage_getisdriveable_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_078948_crotarywingaircraft_gethelirotordestroyedbyped(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078949_cvehicledamage_canvehiclebedamaged_centity_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_078950_ctransmission_applyenginedamage_cvehicle_ptr_centity_ptr_edamage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32); }
extern "C" { pub fn f_078951_cvehicle_triggercaralarm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078952_cvehicledamage_breakoffwheel_int_float_float_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_078953_cvehicle_hasrocketboost(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078954_cvehicle_hasjump(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078955_cvehicle_turnsirenon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_078956_ceventshockingmaddriverbicycle_ceventshockingmaddriverbicycle_ce(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_078957_ceventshockingmaddriverextreme_ceventshockingmaddriverextreme_ce(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_078958_ccomponentreservationmanager_getseatreservationfromseat_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_078960_cvehicle_processinterpolation_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_078961_cautomobile_getmaxlengthfordummyconstraintsafterfailedconversion(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_078964_cvirtualroad_calculatepathinfo_cvehicle_const_ref_bool_bool_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_078965_cvehicle_isentityattachedtotrailer_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078966_cvirtualroad_helpercreaterouteinfofromfollowrouteandcenterpos_cv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_078967_cvirtualroad_processrouteconstraint_cvirtualroad_trouteinfo_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_078969_cvehicle_renderdebug_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078970_ctransmission_getturbopowermodifier_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_078971_cvehicledebug_getfocusvehicle(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_078972_cvehicledamage_getvehiclehealthpercentage_cvehicle_const_ptr_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32) -> f32; }
extern "C" { pub fn f_078973_cvehicledamage_getoverallhealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_078974_cvehicledamage_getenginehealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_078976_cvehicledamage_getstucktimer_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_078977_cvehicle_initfragmentextras(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078978_cvehicle_fixwindow_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078979_cvehicle_rollwindowup_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078980_cvehicle_updatewingbones_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_078981_cvehicle_hascontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078982_ccardoor_replayupdatedoors_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078983_ctransmission_processengine_cvehicle_ptr_rage_mat34v_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32, a6: f32, a7: f32) -> f32; }
extern "C" { pub fn f_078984_cvehicle_processrocketboost(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078985_cvehicle_processboostphysics_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078986_cvehicle_getbrakeforce_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_078987_ctransmission_processenginecvt_cvehicle_ptr_int_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: f32) -> f32; }
extern "C" { pub fn f_078988_ctransmission_drawtransmissionstats_cvehicle_ptr_float_float_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_078989_caircraftdamage_getthrustmult_cplane_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_078990_ccardoor_breakoff_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_078991_ccardoor_printdoorflags_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078992_ccardoor_breaklatch_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078993_ccardoor_needstoanimateopen_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_078994_ccardoor_drivedooropen_float_rage_phjoint_ptr_cvehicle_ptr_rage(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_078995_cvehicle_forcenosleep(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078996_clandinggear_controllandinggear_cvehicle_ptr_clandinggear_ecomma(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_078997_ceventshockingcarcrash_ceventshockingcarcrash_centity_ref_centit(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_078998_cvehicle_switchengineoff_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078999_cvehicle_getvehiclewassunkbyaplayerped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079000_cvehicle_istank_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079001_caircraftdamagebase_hassectionbrokenoff_cplane_const_ptr_int_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_079002_cvehicle_applyparachutestabilisation_rage_vector3_ref_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_079003_cautomobile_processparachutephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079004_cvehicle_iswheelcontactphysicalmoving_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079005_cvehicle_slidemechanicalpart_int_float_float_ref_rage_vector3_fl(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i64, a5: f32); }
extern "C" { pub fn f_079006_cautomobile_finishparachuting(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079007_cvehicle_getnumberofpassenger_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079008_cvehicle_setupwheelcollisions_rage_phboundcomposite_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079009_cwheelintegrator_processintegration_rage_phcollider_ptr_cvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_079010_clandinggear_processphysics_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079012_cwheelintegrator_processintegrationtask_rage_phcollider_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: f32, a7: i32, a8: f32, a9: i32, a10: f32); }
extern "C" { pub fn f_079014_cwheel_processpresimupdate_void_const_ptr_rage_phcollider_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_079015_cvehicle_processslowmotionspecialabilitypostphysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079017_cwheel_integrationfinalprocess_rage_phcollider_ptr_rage_scalarv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079018_cvehicle_initextraflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079019_cvehicle_updateextras(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079023_cvehicle_getattachedtrailer_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079025_cvehicle_initcompositebound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079026_cvehicle_initialisemodcollision(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079027_cvehiclegadgetforks_init_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079028_cbouncingpanel_processpanel_cvehicle_ptr_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_079029_cvehicle_setcomponentrotation_ehierarchyid_erotationaxis_float_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32); }
extern "C" { pub fn f_079030_cvehicledeformation_resetdamage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079031_cvehicledamage_damagevehicle_centity_ptr_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i64); }
extern "C" { pub fn f_079032_cvehicle_processlightcovers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079033_cvehicle_gethdmoderequests_bool_ref_bool_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079034_cvehicle_updateweaponbones_cvehicleweaponbattery_const_ptr_int_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: f32, a6: f32, a7: f32); }
extern "C" { pub fn f_079036_cwheel_getwheelcompressionfromstaticincburstandsink_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_079037_rage_matrix34_rotatelocalx_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079038_cvehicle_getlastdriver_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079039_cvehicle_isturretseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079040_cvehicle_getspecialflightmoderatio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_079041_cvehicle_rolldownwindow_ehierarchyid_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079042_cvehicle_setsuspensiontransformation_ehierarchyid_ehierarchyid_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_079043_setsuspensiontransformation_rage_crskeleton_ptr_ehierarchyid_ehi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_079044_cvehicle_settransmissionrotation_ehierarchyid_ehierarchyid_ehier(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_079045_settransmissionrotation_rage_crskeleton_ptr_ehierarchyid_ehierar(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_079047_cwheel_getwheelposandradius_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_079049_cvehicle_getdefaultbonepositionsimple_int_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_079050_cautomobile_blowupcar_centity_ptr_bool_bool_bool_unsigned_int_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_079051_cvehicle_blowupcar_centity_ptr_bool_bool_bool_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_079052_cvehicle_killpedsinvehicle_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079053_cvehicledamage_blowupcarparts_centity_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079054_cvehicledamage_blowupvehicleparts_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079055_cvehicledamage_addvehicleexplosiondeformations_cvehicle_ptr_cent(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_079056_cvehicledamage_updatedamagetrackers_centity_ptr_unsigned_int_eda(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: i32, a6: i32, a7: i64); }
extern "C" { pub fn f_079057_cvehicletrailerattachpoint_detachtrailer_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079058_cvehicle_trytomakepedsfrompretendoccupants_bool_int_const_ptr_ca(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_079059_cvehicledamage_breakoffpart_ehierarchyid_int_rage_fraginst_ptr_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: f32, a7: f32, a8: i32); }
extern "C" { pub fn f_079060_cvehicledamage_headoncollision_cvehicle_ptr_float_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079061_cvehicledamage_rearendcollision_cvehicle_ptr_float_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079064_cvehicle_cleanupmissionstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079065_cvehicle_getdoorfromid_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079068_ccardoor_setlatch_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079069_cvehicle_smashwindow_ehierarchyid_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_079072_cvirtualroad_testprobestovirtualroadrouteinfo_cvirtualroad_trout(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_079073_cvirtualroad_helpercreaterouteinfofromnodelistandcenternode_cveh(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_079074_cwheel_setcompressionfromhitpos_rage_matrix34_const_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_079075_cautomobile_cleanupplacementarray_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_079077_cautomobile_dowhitelighteffects_int_configvehiclewhitelightsetti(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: f32); }
extern "C" { pub fn f_079078_cautomobile_doposlighteffects_int_configvehiclepositionlightsett(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: f32); }
extern "C" { pub fn f_079079_cautomobile_enableburnoutmode_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079084_cwheel_processimpactresults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079086_cvirtualroad_testprobestovirtualroadtworouteinfos_cvirtualroad_t(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_079087_cwheel_probeprocessresults_rage_matrix34_const_ptr_worldprobe_cs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079088_cvehicle_fixupintersectionwithwheelsurface_worldprobe_cshapetest(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079089_cvirtualroad_helpercreaterouteinfofromroutepoints_croutepoint_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i32); }
extern "C" { pub fn f_079090_cvirtualroad_processroadcollision_cvirtualroad_trouteinfo_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_079091_findfakelevelindexforshapetestresult(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_079092_cwheel_updatecontactsafternetworkblend_rage_matrix34_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079095_ccardoor_init_cvehicle_ptr_ehierarchyid_float_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: f32, a5: i32); }
extern "C" { pub fn f_079098_cvehicle_selectappropriategearforspeed(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079100_cautomobile_startparachuting(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079107_cbike_processconstraint_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079108_cbike_setbikeonsidestand_bool_bool_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_079110_cvehicle_detachfromparentvehiclegadget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079122_cvehicle_switchtofullfragmentphysics_evehicledummymode(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079123_cbike_removeconstraint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079124_cvehicle_setvelocityfromsuperdummyvelocity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079126_cvehicle_switchtodummyphysics_evehicledummymode(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079128_cvehicle_switchtosuperdummyphysics_evehicledummymode(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079130_cvehicle_getlayoutinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079132_cbike_getbikeleaninputs_float_ref_float_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079134_cvehicle_getdriverseat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079136_cbike_isconsideredstill_rage_vector3_const_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_079145_cbike_placeonroadproperly_cbike_ptr_rage_matrix34_ptr_cinteriori(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: i32, a8: i64, a9: i64, a10: i32, a11: f32, a12: f32, a13: i32) -> i32; }
extern "C" { pub fn f_079150_cvehicle_teleport_rage_vector3_const_ref_float_bool_bool_bool_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_079151_cvehicle_teleportwithoutupdategadgets_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_079152_cvehicle_updategadgetsafterteleport_rage_matrix34_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_079156_cboathandling_setmodelid_cvehicle_ptr_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079157_cvehicle_getisjetski_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079165_cboat_dolight_int_configboatlightsettings_ref_float_cvehiclemode(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i32, a5: f32); }
extern "C" { pub fn f_079167_cvehicle_shouldlightsbeon_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079169_cboat_getsearchlight(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079171_cboat_processlowlodbuoyancy(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079175_cvehicle_updateentityfromphysics_rage_phinst_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079177_cvehicle_setupmissionstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079194_scheduledvehicle_dtorscheduledvehicle(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079200_ccargenerator_checkifreadyforschedule_bool_bool_ref_rage_fwmodel(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_079201_ccargenerator_checkisactiveforgen(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079202_ccargenerator_checkpopulationhasspace(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079203_ccargenerator_pickvalidmodels_rage_fwmodelid_ref_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_079204_ccargenerator_checkscenarioconditionsb_rage_fwmodelid_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_079205_ccargenerator_checknetworkobjectlimits_rage_fwmodelid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079207_ccargenerator_registerspawnfailure_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079208_ccargenerator_buildcreationmatrix_rage_matrix34_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079209_ccargenerator_insertvehforgen_rage_fwmodelid_const_ref_rage_fwmo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_079210_ccargenerator_dosyncprobeforblockingobjects_rage_fwmodelid_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_079211_ccargenerator_placeonroadproperly_cvehicle_ptr_rage_matrix34_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_079212_ccargenerator_process_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079213_ccargenerator_checkifwithinrangeofanyplayers_bool_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_079214_ccargenerator_checkifwithinrangeofplayer_cplayerinfo_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_079215_cthecargenerators_postcreationsetup_scheduledvehicle_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079216_cvehiclepopulation_addvehicletospawnhistory_cvehicle_ptr_cvehicl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079217_cthecargenerators_addtogameworld_cvehicle_ref_cvehicle_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079218_cvehiclefactory_destroyboattrailerifnecessary_ctrailer_ptr_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079219_cvehiclepopulation_makevehicleintodummybasedondistance_cvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079220_ccargenerator_debugrender_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079221_cthecargenerators_init(ph: *mut Phantom); }
extern "C" { pub fn f_079222_cthecargenerators_shouldgeneratevehicles(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_079223_cthecargenerators_dropscheduledvehicle_scheduledvehicle_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079224_cvehiclepopulation_willvehiclecollidewithothercarusingrestrictiv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_079225_cthecargenerators_destroycargeneratorbyindex_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_079228_cvehicle_updatebodycolourremapping_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079229_cvehicle_finishaiupdate_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079231_cthecargenerators_dumpcargeninfo(ph: *mut Phantom); }
extern "C" { pub fn f_079232_ccargenerator_removeregdentities(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079233_ccargenerator_dtorccargenerator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079234_cthecargenerators_setcargeneratorsactiveinarea_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079237_rage_atarray_cargenpriorityareas_area_0_unsigned_short_grow_unsi(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079245_cthecargenerators_requestvehiclemodelforscenario_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079246_cthecargenerators_instantfill(ph: *mut Phantom); }
extern "C" { pub fn f_079247_ccomponentreservation_dtorccomponentreservation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079250_ccomponentreservation_unreserveuseofcomponent_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079253_ccomponentreservationmanager_getseatreservation_int_seataccess_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_079258_coronaandlightlist_addlight_clightsource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079260_addvehicleambientoccluder_rage_vector3_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: i32, a9: i64); }
extern "C" { pub fn f_079261_cvehicle_doheadlightseffect_bool_bool_int_int_float_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: f32, a6: f32, a7: f32, a8: i64, a9: i64, a10: i32, a11: i32, a12: i64, a13: f32, a14: f32, a15: i64, a16: i64, a17: i32, a18: i32, a19: i32, a20: i64); }
extern "C" { pub fn f_079262_cvehicle_dotaillightseffect_int_int_int_bool_bool_bool_float_flo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: i32, a11: i64, a12: i64, a13: i32, a14: i64, a15: f32, a16: f32, a17: i64, a18: i64, a19: i64); }
extern "C" { pub fn f_079263_cvehicle_doadditionallightseffect_int_float_configlightsettings(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i64, a5: i32, a6: i64); }
extern "C" { pub fn f_079264_cvehicle_doindicatorlighteffect_int_float_float_float_cvehiclemo(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64); }
extern "C" { pub fn f_079265_cvehicle_dointeriorlighteffect_int_float_cvehiclemodelinfo_ptr_r(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_079267_ccardoor_getisintact_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079268_cvehicledeformation_applydamagetobound_void_const_ptr_rage_atfix(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_079270_cvehicle_iswindowdown_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079271_ccardoor_getlocalmatrix_cvehicle_const_ptr_rage_matrix34_ref_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_079273_cplane_updatewindowbound_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079275_cvehicle_haswindowtosmash_ehierarchyid_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_079278_cvehicle_initialiseweaponblades(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079279_cvehicle_initialisespikecollision(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079280_cvehicle_initialiserammingscoops(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079281_cvehicle_initialiserammingbars(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079282_cvehicle_initialiseramps(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079284_ccardoor_fix_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079289_chandlingdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_079292_cbasesubhandlingdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_079335_chandlingdata_gethandlingdata_ehandlingtype(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079336_chandlingdata_converttogameunits_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079338_chandlingdata_clearhandlingdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079341_chandlingdatamgr_reset(ph: *mut Phantom); }
extern "C" { pub fn f_079342_chandlingdatamgr_loadhandlingmetadata_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079343_chandlingdatamgr_appendhandlingmetadata_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079345_chandlingdatamgr_loadvehicleextrasfile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079347_chandlingdatamgr_gethandlingdata_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_079348_crotarywingaircraft_crotarywingaircraft_eentityownedby_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_079350_crotarywingaircraft_initphys(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079352_crotarywingaircraft_updaterotorbounds(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079354_crotarywingaircraft_wantstobeawake(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079355_crotarywingaircraft_computeadditionalyawfromtransform_float_rage(ph: *mut Phantom, a0: f32, a1: i64, a2: f32) -> f32; }
extern "C" { pub fn f_079356_crotarywingaircraft_getisjetpack_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079357_crotarywingaircraft_doprocesscontrol_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_079358_csearchlight_processsearchlight_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079359_cturret_aimatworldpos_rage_vector3_const_ref_cvehicle_ptr_bool_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_079360_crotarywingaircraft_processprecomputeimpacts_rage_phcontactitera(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079361_cpropellercollision_processprecomputeimpacts_cvehicle_ptr_rage_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_079362_crotarywingaircraft_processpostphysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079363_crotarywingaircraft_startcrashingbehavior_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_079364_crotarywingaircraft_breakoffmainrotor(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079365_crotarywingaircraft_breakoffrearrotor(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079366_crotarywingaircraft_breakofftailboom_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079375_crotarywingaircraft_prerender2_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079376_cvehicle_dointeriorlighteffect_int_float_cvehiclemodelinfo_ptr_r(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_079377_cvehicle_isenteringinsideorexiting(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_079378_cvehicle_addlight_elighttype_int_configlightsettings_const_ref_f(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32); }
extern "C" { pub fn f_079380_cpropeller_applydeformation_cvehicle_ptr_void_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079381_crotarywingaircraft_fix_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079382_crotarywingaircraft_blowupcar_centity_ptr_bool_bool_bool_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_079384_crotarywingaircraft_finishblowingupvehicle_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_079386_crotarywingaircraft_setdampingforflight_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079387_crotarywingaircraft_processflighthandling_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079391_cheli_dtorcheli_129271(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079393_clandinggear_initphys_cvehicle_ptr_ehierarchyid_ehierarchyid_ehi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079396_cpropellercollision_init_ehierarchyid_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_079397_clandinggear_initcompositebound_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079399_cheli_updateropes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079400_cheli_isropeattachedtononhelientity_rage_ropeinstance_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079403_cheli_removeropesandhook(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079404_cvehiclegadgetpickuprope_detachentity_cvehicle_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_079405_cvehiclegadgetpickuprope_removeropesandprop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079408_cheli_calculatehoverforce_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_079411_cheli_findspawncoordinatesforheli_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079412_cvehicle_setupdriver_bool_bool_unsigned_int_cambientmodelvariati(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_079415_rage_ropeinstance_getinstancea_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079416_rage_ropeinstance_getinstanceb_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079419_cheli_addpickuprope(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079420_cvehiclegadgetpickupropewithmagnet_cvehiclegadgetpickupropewithm(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_079422_cvehicle_getvehiclegadget_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079429_clandinggear_prerenderdoors_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079431_clandinggear_applydeformation_cvehicle_ptr_void_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079433_clandinggear_fix_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079435_cvehicle_processcollision_rage_phinst_const_ptr_centity_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: f32, a7: i64, a8: i32, a9: i32, a10: i64, a11: i32, a12: i32); }
extern "C" { pub fn f_079436_cvehicledeformation_applycollisionimpact_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_079439_cvehicle_processprephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079440_cvehicle_resetskeletonformods(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079441_clandinggear_getpublicstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079443_cvehicledamage_applydamagetoengine_centity_ptr_edamagetype_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64, a5: i64, a6: i64, a7: i32, a8: i32, a9: f32, a10: i32, a11: i32, a12: i32); }
extern "C" { pub fn f_079444_cvehicle_updatelinkattachmentmatricesrecursive_rage_phboundcompo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079445_cpropellercollision_updatebound_cvehicle_ptr_int_float_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_079446_caircraftdamage_applydamagetoengine_centity_ptr_cplane_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i64, a6: i64) -> f32; }
extern "C" { pub fn f_079447_cvehicledamage_findclosestpetroltankpos_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079448_rage_spdaabb_intersectssphere_rage_spdsphere_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079449_cvehicledamage_setpetroltankonfire_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079459_cblimp_breakblimp(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079462_clandinggearpartphysicalrot_finddefaultjointlimits_fraginstgta_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_079470_clandinggearpartphysicalrot_drivetotargetopenratio_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_079471_clandinggearpartphysicalrot_snap1dofjointtoopenratio_fraginstgta(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_079472_cplane_getverticalflightmodeavaliable_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079473_cplane_getislandinggearintact_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079476_caircraftdamagebase_applydamagetoplane_cplane_ptr_centity_ptr_ed(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: f32, a6: i64, a7: i64, a8: i32, a9: i32, a10: f32); }
extern "C" { pub fn f_079477_caircraftdamagebase_breakoffabovegrouprecursive_int_rage_fragins(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_079478_caircraftdamagebase_initcompositebound_cplane_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079479_caircraftdamagebase_breakoffsection_cplane_ptr_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_079481_caircraftdamage_dtorcaircraftdamage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079488_caircraftdamage_getrollmult_cplane_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_079489_caircraftdamage_getpitchmult_cplane_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_079490_caircraftdamage_getliftmult_cplane_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_079492_caircraftdamage_computeflamedamageandcreatnewfire_cplane_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_079503_cplane_dtorcplane(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079507_cplane_computenoseposition_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079511_ceventshockingplaneflyby_ceventshockingplaneflyby_centity_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_079512_cplane_isplayerdrivingonground_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079518_cplane_computewindmult_float(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_079519_cplane_modifycontrolsbasedonflyingstats_cped_ptr_cflyinghandling(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: f32); }
extern "C" { pub fn f_079525_cplane_doemergencylighteffects_rage_crskeleton_ptr_int_configpla(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: f32); }
extern "C" { pub fn f_079526_cplane_docontrolpanellighteffects_int_configplanecontrolpanelset(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_079527_cplane_doinsidehulllighteffects_int_configplaneinsidehullsetting(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_079530_cvehicle_getcanmakeintodummy_evehicledummymode(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079532_cvehicle_trytomakeintodummy_evehicledummymode_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_079533_cvehicle_testfordummyinstclearance_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079534_cvehicle_dummyattachchild_cvehicle_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079540_cplane_finishblowingupvehicle_centity_ptr_bool_bool_bool_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_079541_cplane_blowupplaneparts(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079545_cplane_setdesiredverticalflightmoderatio_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079546_cplane_setverticalflightmoderatio_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079547_cplane_isinverticalflightmode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079562_cpropeller_getsubmergefraction_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_079568_csubmarinehandling_csubmarinehandling(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079569_csubmarinehandling_dtorcsubmarinehandling(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079570_csubmarinehandling_setmodelid_cvehicle_ptr_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079573_csubmarinehandling_implode_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079574_cvehicledamage_damagevehiclebydriver_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_079576_csubmarinehandling_processcrushdepth_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079577_csubmarinehandling_processphysics_cvehicle_ptr_float_bool_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_079578_csubmarinehandling_processprops_cvehicle_ptr_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_079579_csubmarinehandling_doprocesscontrol_cvehicle_ptr_bool_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_079580_csubmarinehandling_processbuoyancy_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079582_csubmarinehandling_prerender2_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079587_cvehicle_getfragmentcomponentindex_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079591_csubmarinecar_updatewheels_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079592_csubmarinecar_updatewheelcovers_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079599_csubmarinecar_setsubmarinemode_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079603_ctrailerlegs_updatetocurrentratio_cvehicle_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079604_ctrailer_dtorctrailer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079608_ctrailer_startwheelintegratorifnecessary_ephysicsresult_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_079610_cvehicledamage_breakoffmisccomponents_float_float_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64); }
extern "C" { pub fn f_079611_chandlingdata_gettrailerhandlingdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079614_ctrailer_debugattachtoparentvehicle_char_const_ptr_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: f32, a7: i32) -> i32; }
extern "C" { pub fn f_079615_ctrailer_warptoposition_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079617_ctrailer_getphysicallyattachedobjects_cphysical_ptr_rage_atfixed(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079619_cvehicle_resetafterteleport(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079621_ctrailer_removecargovehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079632_cvehicle_trytomakefromdummy_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079633_cvehicle_doboundingboxintersectiontest_int_int_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_079634_cvehicle_hascollisionloadedaroundvehicle(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079639_ctrain_performdebugteleportbasecb_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_079642_cvehicle_removepedfromseat_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_079643_cvehicle_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079644_cvehicle_processseatcollisionbound_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079645_ctrain_traintrackspostload(ph: *mut Phantom); }
extern "C" { pub fn f_079646_ctrain_trainconfigspostload(ph: *mut Phantom); }
extern "C" { pub fn f_079647_ctrain_loadtrainconfigs_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079648_ctrain_loadtrackxml_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079651_ctrain_removetrainsfromarea_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079652_ctrain_settunablesvalues(ph: *mut Phantom); }
extern "C" { pub fn f_079655_ctrain_settrackactive_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_079656_ctrain_determinehostapproval_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_079657_ctrain_receivedhostapprovaldata_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_079658_ctrain_processtrainnetworkpending_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_079663_ctrain_createtrain_signed_char_int_bool_unsigned_char_ctrain_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_079664_ctrain_getworldposfromtrackpos_rage_vector3_ref_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: f32, a6: i64); }
extern "C" { pub fn f_079665_ctrain_settrackposfromworldpos_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079666_ctrain_getstationrelativeinfo_int_ref_int_ref_float_ref_float_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_079667_ctrain_updateposition_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079668_ctrain_getworldposfromtracksegmentpos_rage_vector3_ref_float_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i64, a6: f32) -> i32; }
extern "C" { pub fn f_079671_ctrain_changetrainstate_ctrain_trainstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079672_ctrain_processdoors_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_079681_ctrain_dtorctrain(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079688_ctrain_isengine_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079690_ctrain_processtraincontrol_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079691_ctrain_islocalplayerstandinginthistrain_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079696_ctrain_isentityblockingprogress_centity_ptr_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64) -> i32; }
extern "C" { pub fn f_079703_ctrain_signaltrainpadshake_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079709_ctransmission_processnitrous_cvehicle_ptr_float_ref_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_079711_ctransmission_disablekerseffect(ph: *mut Phantom); }
extern "C" { pub fn f_079727_cturretphysical_resetturretsmuscles_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079730_cvehicle_setsuperdummyvelocity_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079739_cvehicledeformation_handledamageadded_void_ptr_bool_bool_bool_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_079740_cvehicledeformation_savedamagetexture_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079741_cvehicledeformation_updateshaderdamagevars_bool_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32); }
extern "C" { pub fn f_079743_cvehicledeformation_damagetextureallocate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079746_cvehicle_doesvehiclehaveaconvertibleroofanimation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079749_cwheel_gettyreshouldburst_rage_vector3_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079750_cwheel_processwheelburst(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079754_cvehicle_initialiseweaponbladecollision_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079755_updatecollisiononallchildrenrecursive_rage_fragphysicslod_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_079756_cvehicle_getdefaultbonerotation_int_rage_quaternion_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_079757_cvehicle_toggleroofcollision_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079758_cvehicle_getconvertibleroofstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079759_cvehicle_canbedeletedspecial_bool_bool_bool_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_079761_cvehiclegadgettowarm_detachvehicle_cvehicle_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_079762_cvehicle_processoverriddendummymode_evehicledummymode(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079763_cvehicle_testvehicleboundforcollision_rage_matrix34_const_ptr_cv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_079765_cvehicle_switchawayfromtimeslicedlod(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079766_cvehicle_setinterpolationtargetfromextrapolatedvelocity_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079767_cvehicle_processsuperdummy_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_079768_cvehicle_lerpclonesuperdummymatrix_rage_matrix34_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079771_cvehicle_getapproximatesize_rage_vector3_ref_rage_vector3_ref_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079772_cvehicle_getvehicleorientation_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079776_cvehicle_givedefaulttask(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079777_cvehiclepopulation_pickcruisespeedwithvehmodel_cvehicle_const_pt(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_079780_cvehicle_haspedsinit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079781_cvehicle_hasmissioncharsinit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079783_cvehiclefactory_logdestroyedvehicle_cvehicle_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_079784_cvehicle_setvariationinstance_cstoredvehiclevariations_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079785_cvehicle_getexhaustmatrix_ehierarchyid_rage_mat34v_ref_int_ref_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_079786_cvehicle_getexhaustmatrixfromboneindex_rage_mat34v_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_079787_cvehicle_getextralightmatrix_ehierarchyid_rage_mat34v_ref_int_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_079790_cvehicle_trytogiveeventreponsetasktooccupant_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079792_cvehicle_logpretendoccupantsfail_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079793_cvehiclepopulation_addpolicevehoccupants_cvehicle_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_079794_cvehiclepopulation_adddriverandpassengersforveh_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_079798_cvehicle_updateattachedclonepedsafterwarp(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079799_cvehiclegadgettowarm_updateconstraintdistance_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079800_cvehiclegadgettowarm_warptoposition_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079801_cvehiclegadgetpickuprope_updateattachments_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079802_cvehiclegadgetpickuprope_warptoposition_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079803_cvehicle_triggerhydraulicbouncesound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079804_cvehicle_triggerhydraulicactivationsound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079805_cvehicle_triggerhydraulicdeactivationsound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079806_cvehicle_hastowinggadget_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079807_cvehicle_getentitybeingtowed_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079808_cvehiclegadgettowarm_actuallydetachvehicle_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079810_cvehicle_getdoorfromboneindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079819_cvehicle_resetexclusivedriverforreuse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079821_cvehicle_setuppassenger_int_bool_unsigned_int_cambientmodelvaria(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_079824_cvehicledamage_getpetroltankposlcl_rage_vector3_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_079825_cvehicle_removefragmentcomponent_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079827_cvehicle_canpedentercar_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079837_cvehicle_hasbulletresistantglass_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079839_cvehicle_getaihandlinginfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079840_cvehicle_isleakingpetrol_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079841_cvehicle_isonfire_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079842_cvehicle_extinguishcarfire_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079843_cvehicle_carhasroof_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079845_cvehicle_ispersonalvehicle(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079846_cvehicle_deployfoldingwings_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079850_cvehicle_weareblockingsomeone(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079854_cvehicle_containsped_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079856_cvehicle_stophorn(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079857_addvehiclelight_rage_vector3_const_ref_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: i64, a10: i32, a11: i64, a12: i64, a13: i32, a14: i32, a15: i32, a16: f32, a17: i32, a18: i32, a19: i32, a20: i32, a21: i32, a22: f32, a23: f32); }
extern "C" { pub fn f_079858_cvehicle_doesvehiclehaveatransformanimation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079859_cvehicle_shouldapplyramptoimpact_cvehicle_ptr_cvehicle_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_079860_cvehicle_applyspikedamagetoped_cped_ptr_rage_vec3v_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_079861_cwheel_applytyredamage_centity_const_ptr_float_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_079863_cvehicledamage_applypadshake_centity_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_079864_cvehicle_deletewindow_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079866_cvehicle_turnoffextra_ehierarchyid_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079868_cvehicle_addcommentonvehicleevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079869_cfixedvehicleweapon_cfixedvehicleweapon_unsigned_int_ehierarchyi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_079870_cturret_getturretmatrixworld_rage_matrix34_ref_cvehicle_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079871_cvehiclegadget_drivecomponenttoangle_rage_fraghierarchyinst_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: i64, a5: f32, a6: f32, a7: f32, a8: f32, a9: i32, a10: f32); }
extern "C" { pub fn f_079873_cvehicleweaponmgr_getnumweaponsforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079877_cvehicle_getnumflattires_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079879_cvehicle_detachtombstone(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079880_cvehicle_updatedoorsfornavigation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079882_cvehicle_processrumble_cwheel_const_ptr_const_ptr_int_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32) -> i32; }
extern "C" { pub fn f_079890_cvehicle_isdoorclearforped_rage_vector3_const_ref_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_079895_cvehicle_rolldownwindows(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079896_cvehicle_everyoneinsidewillflythroughwindscreen_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079903_cvehicle_setforwardspeedxy_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079909_cvehicle_shouldlowerconvertibleroof_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079910_cvehicle_lowerconvertibleroof_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079911_cvehicle_raiseconvertibleroof_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079916_cvehicle_getattachparentvehicledummyorphysical_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079917_cvehicle_iscaminsidevehicleinterior(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079925_cvehicle_givewarning(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079938_cvehicle_sethandlingcheatvaluesforpolicecarinpursuit_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079939_cvehiclepopulation_clearinterestingvehicles_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_079943_cvehicle_getchassisboundmin_bool_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079952_cvehicle_offsetboneposition_int_float_rage_vector3(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64); }
extern "C" { pub fn f_079956_cvehicle_calculatedesiredup_float_float_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_079957_cvehicle_applyhovermodestabilisation_rage_vector3_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_079958_cvehicle_calculategroundheight_float_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_079969_cvehicledamage_damagecurrentcar_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_079980_cvehicledeformation_applydeformationsfromnetwork_unsigned_int_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079987_cvehicledamage_copy_cvehicle_ptr_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079988_getdamagetexturefilename_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079997_cwheel_doburstwheeltorim(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080005_cvehicledamage_avoidvehicleexplosionchainreactions(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_080008_cvehicledamage_getpetroltankposwld_rage_vector3_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_080009_cvehicledamage_popoutwindscreen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080010_cvehicledamage_popoffroof_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080016_cvehiclefactory_dtorcvehiclefactory(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080020_cvehiclefactory_registervehicletonetwork_cvehicle_ptr_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_080021_cvehiclegadgetwithjointsbase_cvehiclegadgetwithjointsbase_short(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32) -> i64; }
extern "C" { pub fn f_080022_cvehiclegadgetwithjointsbase_initjointlimits_cvehiclemodelinfo_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080024_probeforwheelposition_cvehicle_const_ptr_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> f32; }
extern "C" { pub fn f_080031_cvehiclegadgettowarm_deleteropeshookandconstraints_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080032_rage_phinst_setposition_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080042_getmodelindex_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_080043_updateavailablecarscb(ph: *mut Phantom); }
extern "C" { pub fn f_080044_cvehiclefactory_carnumchangecb(ph: *mut Phantom); }
extern "C" { pub fn f_080046_createcaratmatrix_unsigned_int_rage_matrix34_ptr_rage_vector3_co(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_080048_createcar_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080049_cvehicledebug_setvehicleaspersonalcb(ph: *mut Phantom); }
extern "C" { pub fn f_080050_cvehiclefactory_variationdebugupdatemods(ph: *mut Phantom); }
extern "C" { pub fn f_080053_cvehiclepopulation_choosemodel_bool_ptr_rage_vector3_const_ref_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_080054_cvehiclepopulation_getdesirednumberambientvehicles_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> f32; }
extern "C" { pub fn f_080055_deletecarcb(ph: *mut Phantom); }
extern "C" { pub fn f_080061_firecarcb(ph: *mut Phantom); }
extern "C" { pub fn f_080063_firecarwithobjectcb(ph: *mut Phantom); }
extern "C" { pub fn f_080066_fireplayerragdollcb(ph: *mut Phantom); }
extern "C" { pub fn f_080068_fireobjectcb(ph: *mut Phantom); }
extern "C" { pub fn f_080072_warpplayerintorecentcarcb(ph: *mut Phantom); }
extern "C" { pub fn f_080075_firecarwithplayercb(ph: *mut Phantom); }
extern "C" { pub fn f_080080_cvehicleweaponmgr_getweaponsforseat_int_rage_atarray_cvehiclewea(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_080141_cvehiclefactory_updatevehiclelist(ph: *mut Phantom); }
extern "C" { pub fn f_080164_void_std_2_introsort_std_2_classicalgpolicy_int_ptr_ref_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_080165_setvehicleonfire_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080166_searchlightlightinfo_initwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080188_getboardingpointsfromvehiclefactoryindex_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_080189_cboardingpointeditor_createnextboat_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080190_cvehiclegadgettowarm_removeropesandhook(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080196_cfixedvehicleweapon_dtorcfixedvehicleweapon_130455(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080197_cfixedvehicleweapon_processcontrol_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080205_cfixedvehicleweapon_applybulletdirectionoffset_rage_matrix34_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080206_cfixedvehicleweapon_fireinternal_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_080208_cfixedvehicleweapon_processpostprerender_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080236_cturret_aimatlocaldir_rage_vector3_const_ref_cvehicle_ptr_bool_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_080239_cturret_computelocaldirfromworldpos_rage_vector3_const_ref_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_080240_cturret_computerelativeheadingdelta_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_080241_cturret_initjointlimits_cvehiclemodelinfo_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080244_cturretphysical_driveturret_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_080249_cturretphysical_clampangletojointlimits_float_cvehicle_const_ref(ph: *mut Phantom, a0: f32, a1: i64, a2: i32) -> f32; }
extern "C" { pub fn f_080251_cvehicleweaponmgr_getweaponindexforturret_cturret_const_ref_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_080256_csearchlight_dtorcsearchlight(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080264_cvehiclegadget_snapcomponenttoangle_fraginstgta_ptr_int_float_fl(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64); }
extern "C" { pub fn f_080265_cvehicleweaponmgr_dtorcvehicleweaponmgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080267_cvehicleweaponmgr_getvehicleweaponindexbyhash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_080271_cvehiclegadgetwithjointsbase_getjoint_short(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080273_cvehiclegadgetwithjointsbase_processphysics_cvehicle_ptr_float_i(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_080278_cvehiclegadgetarticulateddiggerarm_getnumberofsetupdiggerjoint(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080279_cvehiclegadgetarticulateddiggerarm_getjointsetup_ediggerjoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_080281_cvehiclegadgettowarm_dtorcvehiclegadgettowarm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080283_cvehiclegadgettowarm_addropesandhook_cvehicle_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080285_rage_quaternion_fromvectors_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080286_cvehiclegadgettowarm_gettowhookidealposition_cvehicle_ptr_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_080291_cvehiclegadgettowarm_attachropestoobject_cvehicle_ptr_cphysical(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_080292_cvehiclegadgettowarm_updatehookposition_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_080293_cvehiclegadgettowarm_attachvehicletotowarm_cvehicle_ptr_cvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_080294_cvehiclegadgettowarm_attachvehicletotowarm_cvehicle_ptr_cvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_080297_cvehiclegadgettowarm_processtowarmandhook_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080298_ceventshockingvehicletowed_ceventshockingvehicletowed_centity_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_080299_rage_ropeinstance_getworldpositionb_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080302_cvehiclegadgetforks_setdesiredacceleration_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_080306_cvehiclegadgetforks_detachpalletimmediatelyandcleanup_cvehicle_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080309_cvehiclegadgethandlerframe_setdesiredacceleration_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_080313_cvehiclegadgethandlerframe_detachcontainerfromframe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080319_cvehiclegadgetpickuprope_computeandsetinversemassscale_cvehicle(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080325_cvehiclegadgetpickuprope_actuallydetachvehicle_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_080326_cvehiclegadgetpickuprope_attachropestoobject_cvehicle_ptr_cphysi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080327_cvehiclegadgetpickuprope_updatehookposition_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_080328_cvehiclegadgetpickuprope_computeidealattachmentoffset_centity_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_080329_cvehiclegadgetpickuprope_attachentitytopickupprop_cvehicle_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_080330_cvehiclegadgetpickuprope_processprecomputeimpacts_cvehicle_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080331_cvehiclegadgetpickuprope_processphysics3_cvehicle_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_080332_cvehiclegadgetpickuprope_processcontrol_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080334_cvehiclegadgetpickuprope_trigger(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080338_cvehiclegadgetpickupropewithmagnet_dtorcvehiclegadgetpickupropew(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080339_cvehiclegadgetpickuprope_dtorcvehiclegadgetpickuprope_130656(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080342_cvehiclegadgetpickupropewithmagnet_pointmagnetattarget_cvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080346_cvehiclegadgetpickupropewithmagnet_doreducedmagnet_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_080380_cvehiclepopulation_initvaluesfromconfig(ph: *mut Phantom); }
extern "C" { pub fn f_080382_cvehiclepopulation_initvaluesfromtunables(ph: *mut Phantom); }
extern "C" { pub fn f_080383_cvehiclepopulation_initvehiclespacings(ph: *mut Phantom); }
extern "C" { pub fn f_080384_cvehiclepopulation_initplayersroadmodifiers(ph: *mut Phantom); }
extern "C" { pub fn f_080385_cvehiclepopulation_updatecurrentvehiclespacings_float(ph: *mut Phantom); }
extern "C" { pub fn f_080386_cvehiclepopulation_scanfornodesonplayersroad_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080391_cvehiclepopulation_cleanupplayersjunctionnode(ph: *mut Phantom); }
extern "C" { pub fn f_080392_rage_aitaskmanager_getactivetask_int_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080398_cvehiclepopulation_calculatepopulationrangescale(ph: *mut Phantom); }
extern "C" { pub fn f_080399_cvehiclepopulation_updatekeyholeshape_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_080400_cvehiclepopulation_startasyncupdatejob(ph: *mut Phantom); }
extern "C" { pub fn f_080401_cvehiclepopulation_setscriptcampopulationoriginoverride(ph: *mut Phantom); }
extern "C" { pub fn f_080402_cvehiclepopulation_findnumrandomvehsthatcanbecreated(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_080403_cvehiclepopulation_catchvehicleremovedinview_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_080404_cvehiclepopulation_getdistancefromallplayers_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_080405_cvehiclepopulation_isvehiclewreckedorempty_cvehicle_ptr_float_bo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_080406_cvehiclepopulation_isentityvisibletoremoteplayer_centity_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_080408_cvehiclepopulation_shouldemptycoporwreckedvehberemovedaggressive(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i32; }
extern "C" { pub fn f_080409_void_std_2_introsort_std_2_classicalgpolicy_excessemptynetlistso(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_080410_cvehiclepopulation_shouldremovevehicle_cvehicle_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_080412_cvehiclepopulation_dumpvehiclepooltolog(ph: *mut Phantom); }
extern "C" { pub fn f_080413_cvehiclepopulation_disablecreationonnode_rage_cnodeaddress_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080414_cvehiclepopulation_trytopreventsubsequentcreationattemptsoncurre(ph: *mut Phantom); }
extern "C" { pub fn f_080415_cvehiclepopulation_registervehiclepopulationfailure_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_080416_cvehiclepopulation_findvehicletoreuse_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_080417_cvehiclepopulation_canrejectnetworkspawn_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_080418_cvehiclepopulation_reusevehiclefromvehiclereusepool_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_080419_cvehiclepopulation_removevehiclefromreusepool_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080420_cvehiclepopulation_getrandomvehdensitymultiplier_float_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_080422_cvehiclepopulation_getlowprioparkedcardensitymultiplier(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_080424_cvehiclepopulation_generateoneemergencyservicescarwithroadnodes(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080425_rage_cpathnodelink_initiallanecenteroffset_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_080426_cvehiclepopulation_generateoneemergencyservicesvehiclewithmatrix(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080427_cvehiclepopulation_generatevehcreationposfrompaths_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i64, a9: i64, a10: i64, a11: i64, a12: i32, a13: i32) -> i32; }
extern "C" { pub fn f_080430_cvehiclepopulation_flushvehiclereusepool(ph: *mut Phantom); }
extern "C" { pub fn f_080444_cvehiclepopulation_removeallvehs_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_080475_cvirtualroad_helpergetlongestjunctionedge_rage_cnodeaddress_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_080477_cvirtualroad_processrouteconstraintwithprecalcvalues_cvirtualroa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i32); }
extern "C" { pub fn f_080479_cwheel_calculatetractionvectorsnew_rage_matrix34_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i64); }
extern "C" { pub fn f_080480_cwheel_calculatetractionvectors_rage_matrix34_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32); }
extern "C" { pub fn f_080481_cwheel_gettractioncoefficientfromratio_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_080482_cwheel_setwheelonfire_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_080483_cwheel_getwheelmatrixandbbox_rage_matrix34_ref_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080485_cwheelintegrator_wheelintegrationevaluate_wheelintegrationderiva(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64, a6: f32); }
extern "C" { pub fn f_080486_cwheelintegrator_addveltomatrix_rage_matrix34_ref_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_080493_caihandlinginfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_080515_cvehicledebug_spawnpedcb(ph: *mut Phantom); }
extern "C" { pub fn f_080520_cvehicledebug_clearallstatesfromarraycb(ph: *mut Phantom); }
extern "C" { pub fn f_080541_cvehiclemetadatamgr_getvehicledrivebyinfofromped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080542_cvehiclemetadatamgr_getdrivebyaniminfoforweapon_cped_const_ptr_u(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080547_centryanimvariations_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_080562_cvehicleextrapointsinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_080580_cvehicleexplosioninfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_080589_cvehiclelayoutinfo_scellphoneclipsets_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_080596_cvehiclecoverboundoffsetinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_080603_cbicycleinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_080610_cclipsetmap_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_080614_canimrateset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_080620_cfirstpersondrivebylookaroundsidedata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_080626_cfirstpersondrivebylookarounddata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_080633_cvehiclelayoutinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_080648_cvehicledrivebyaniminfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_080659_cvehiclescenariolayoutinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_080661_cinvehicleoverrideinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_080662_cvehiclemetadatamgr_dtorcvehiclemetadatamgr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080668_cvehiclemetadatamgr_getvehiclescenariolayoutinfo_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_080673_cvehiclemetadatamgr_reset(ph: *mut Phantom); }
extern "C" { pub fn f_080674_cvehicleseatinfo_loadlinkptrs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080675_cvehiclemetadatamgr_validatedataforfinal(ph: *mut Phantom); }
extern "C" { pub fn f_080676_cvehiclemetadatamgr_addvehiclelayoutfile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080678_void_cvehiclemetadatamgr_removeentry_cbicycleinfo_ptr_rage_atarr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080683_cvehiclemetadatamgr_refreshanimlayout_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080684_cvehiclemetadatamgr_save_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080687_cvehiclescenariolayoutinfo_getscenariopointinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080714_cvehicleseataniminfo_findaniminclipset_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_080771_crotarywingaircraft_dtorcrotarywingaircraft_131572(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080795_cvehiclegadgetforks_dtorcvehiclegadgetforks(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080853_bool_creplaymgrinternal_handleentityforfx_cpacketvehdamageupdate(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_080854_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_080855_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_excessempt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_080860_cvfxasyncprobemanager_init(ph: *mut Phantom); }
extern "C" { pub fn f_080861_cvfxasyncprobemanager_submitworldprobe_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_080862_cvfxdeepsurfaceinfo_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080863_cvfxdeepsurfaceinfo_process_rage_vec3v_const_ref_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_080864_cvfxhelper_getdistsqrtocamera_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_080866_cvfxhelper_getmatrixfromboneindex_rage_mat34v_ref_bool_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_080867_cvfxhelper_getglobalmtx_centity_const_ptr_rage_crskeleton_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_080868_cvfxhelper_getmatrixfromboneindex_skinned_rage_mat34v_ref_bool_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_080869_cvfxhelper_getmatrixfromboneindex_smash_rage_mat34v_ref_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_080872_cvfxhelper_creatematfromvecyalign_rage_mat34v_ref_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_080873_cvfxhelper_creatematfromvecz_rage_mat34v_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080874_cvfxhelper_creatematfromveczalign_rage_mat34v_ref_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_080875_cvfxhelper_createrelativemat_rage_mat34v_ref_rage_mat34v_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080876_cvfxhelper_getlocalentitypos_centity_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080877_cvfxhelper_getlocalentitydir_centity_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080878_cvfxhelper_getlocalentityboneposdir_centity_const_ref_int_rage_v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_080879_cvfxhelper_getinterpvalue_float_float_float_bool(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_080880_cvfxhelper_isininterior_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080881_cvfxhelper_getinteriorlocationfromstaticbounds_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_080882_cvfxhelper_isentityincurrentvehicle_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080884_cvfxhelper_getwaterz_rage_vec3v_const_ref_float_ref_rage_fwentit(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_080885_cvfxhelper_getoceanwaterz_rage_vec3v_const_ref_float_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_080887_cvfxhelper_testlineagainstriverwater_rage_vec3v_const_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_080888_cvfxhelper_shouldrenderingameui(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_080892_cvfxfire_loaddata(ph: *mut Phantom); }
extern "C" { pub fn f_080893_cvfxscript_loaddata(ph: *mut Phantom); }
extern "C" { pub fn f_080894_cvfxmaterial_loaddata(ph: *mut Phantom); }
extern "C" { pub fn f_080895_cvfxliquid_loaddata(ph: *mut Phantom); }
extern "C" { pub fn f_080896_cvfxpedinfomgr_setupdecalrendersettings(ph: *mut Phantom); }
extern "C" { pub fn f_080897_cvfxlightningmanager_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080898_ccheckpoints_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080899_cvfxweapon_loaddata(ph: *mut Phantom); }
extern "C" { pub fn f_080900_cvfxwheel_loaddata(ph: *mut Phantom); }
extern "C" { pub fn f_080901_cfiremanager_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080902_cmarkers_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080906_cmoviemgr_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080907_ctvplaylistmanager_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080908_cwatercannons_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080909_cmarkers_initsessionsync(ph: *mut Phantom); }
extern "C" { pub fn f_080911_cptfxmanager_beginframe(ph: *mut Phantom); }
extern "C" { pub fn f_080915_csky_loadskydomemodel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080916_rage_cloudhatmanager_cloudhatmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080917_rage_cloudhatmanager_load_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080924_rage_cloudhatsettings_synctotimecycletimes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080925_cvfxblood_loaddata(ph: *mut Phantom); }
extern "C" { pub fn f_080926_cvfxexplosion_loaddata(ph: *mut Phantom); }
extern "C" { pub fn f_080927_rage_atarray_vfxvehiclerecentlycrashedinfo_s_0_unsigned_short_de(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_080928_rage_vfxlightningsettings_load_char_const_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_080929_cfogvolumemgr_initshaders(ph: *mut Phantom); }
extern "C" { pub fn f_080930_cdistantlightsvertexbuffer_createvertexbuffers(ph: *mut Phantom); }
extern "C" { pub fn f_080933_char_ptr_rage_safecpy_char_char_9_21ul_char_ref_21ul_char_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080938_cmoviemgr_allocateasync_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_080940_cmoviemgr_cmovie_dtorcmovie(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080942_ctvplaylistmanager_cachedtvshow_release_cmoviemgr_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080952_cvfxentity_loaddata(ph: *mut Phantom); }
extern "C" { pub fn f_080953_cvfxfogvolumeinfomgr_loaddata(ph: *mut Phantom); }
extern "C" { pub fn f_080955_cmoviemeshmanager_reset_bool(ph: *mut Phantom); }
extern "C" { pub fn f_080956_ctvplaylistmanager_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080957_cbinkdebugtool_removeall(ph: *mut Phantom); }
extern "C" { pub fn f_080958_cmoviemgr_deleteall(ph: *mut Phantom); }
extern "C" { pub fn f_080959_cmarkers_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080960_cfire_shutdown_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_080961_cvfxtrail_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080963_ctrailgroup_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080966_cptfxmanager_processstoreddata(ph: *mut Phantom); }
extern "C" { pub fn f_080967_cclouds_update(ph: *mut Phantom); }
extern "C" { pub fn f_080968_cvfxweather_update_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_080969_cmoviemgr_updateframe(ph: *mut Phantom); }
extern "C" { pub fn f_080970_cfiremanager_startmapfire_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: i32, a10: i64, a11: i64, a12: i32, a13: i32, a14: i32) -> i64; }
extern "C" { pub fn f_080971_cvfxweapon_triggerptfxbulletimpact_vfxweaponinfo_s_const_ptr_cen(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_080972_rage_cloudsettingsmap_gettimecyclecloudset(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080973_rage_cloudsettingsmap_bankonly_oncombochanged(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080974_rage_cloudhatsettings_getcloudsettings_float_rage_cloudframe_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_080975_cvfxlightningmanager_recordreplaylightning(ph: *mut Phantom); }
extern "C" { pub fn f_080976_cvfxlightningmanager_addlightningstrike(ph: *mut Phantom); }
extern "C" { pub fn f_080977_cvfxlightningmanager_createcloudlightningsequence_float_float_bo(ph: *mut Phantom, a0: f32, a1: f32, a2: i32); }
extern "C" { pub fn f_080978_cvfxlightningmanager_updatecloudlightningsequence_float_int(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_080979_cwatercannon_update_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_080980_cvfxweather_updateptfxwinddebris_rage_mat34v_const_ref_float_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_080981_cvfxweather_updateptfxclouds_rage_mat34v_const_ref_float_float_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_080982_cptfxgpudropsystem_debugupdaterendersettings_int_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_080983_cdecalmanager_registermarker_int_int_rage_vec3v_const_ref_rage_v(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32); }
extern "C" { pub fn f_080985_cfogvolumemgr_register_fogvolume_t_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080986_cmoviemgr_releaseonfailure_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_080987_cmoviemgr_delete_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_080989_cmoviemgr_cmovie_processcancelledsubtitlerequests(ph: *mut Phantom); }
extern "C" { pub fn f_080991_ctrailgroup_addpoint_vfxtrailinfo_t_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_080994_cvehicleglassmanager_processcollision_vfxcollinfo_s_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080995_cvehicleglassmanager_processexplosion_vfxexpinfo_s_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080996_cvehicleglasscomponent_updatecomponent_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_080997_cvehicleglasscomponent_shutdowncomponent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080998_cvehicleglassmanager_addtorenderlist_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080999_cdecaldebug_update(ph: *mut Phantom); }
extern "C" { pub fn f_081000_cvehiclebadgemanager_update(ph: *mut Phantom); }
extern "C" { pub fn f_081002_rage_cloudhatmanager_draw_rage_cloudframe_const_ref_bool_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_081005_cdistantlightsvertexbuffer_renderbufferbegin_cvisualeffects_eren(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: f32, a4: i64); }
extern "C" { pub fn f_081006_cdistantlightsvertexbuffer_reserverenderbufferspace_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_081007_cdistantlightsvertexbuffer_renderbuffer_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081009_rendermiscvfxpredraw(ph: *mut Phantom); }
extern "C" { pub fn f_081010_renderdecalsandptfx_bool(ph: *mut Phantom); }
extern "C" { pub fn f_081011_rendermiscvfxpostdraw(ph: *mut Phantom); }
extern "C" { pub fn f_081012_cfogvolumemgr_renderregistered(ph: *mut Phantom); }
extern "C" { pub fn f_081015_cptfxmanager_render_rage_grcrendertarget_ptr_rage_grcrendertarge(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081022_ccoronas_rendercorona_int_cvisualeffects_erendermode_ccoronas_co(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32) -> i32; }
extern "C" { pub fn f_081023_ccoronas_renderflares_ccoronas_coronarenderinfo_const_ref_corona(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_081033_cptfxgpudropsystem_initwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081048_rage_cloudhatmanager_addcloudhatwidgetsandio_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081049_rage_cloudsettingsmap_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081093_cmoviemgr_initrenderstateblocks(ph: *mut Phantom); }
extern "C" { pub fn f_081094_clensflare_initfxwidgets_rage_bkbank_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_081106_cvfxfogvolumeinfomgr_printfogvolumeparams_cvfxfogvolumeinfo_cons(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081113_cvfxinteriorsetup_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_081122_cvfxinteriorinfomgr_getinfo_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_081149_cvfxpedinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_081159_cvfxpedinfomgr_dtorcvfxpedinfomgr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081166_cdecalmanager_findrendersettinginfo_unsigned_int_int_ref_int_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_081171_cvfxregioninfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_081179_cvfxregiongpuptfxinfo_update_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_081180_cvfxregioninfo_updategpuptfxinfos_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_081181_cvfxregioninfomgr_getinfo_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081190_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081191_cvfxvehicleinfomgr_getinfo_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081202_rage_cloudsettingsmap_dtorcloudsettingsmap(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081203_rage_cloudhatsettings_dtorcloudhatsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081206_rage_cloudhatmanager_requestweathercloudhat_int_float_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_081207_rage_cloudhatmanager_updatetransitions(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081208_rage_cloudhatfraglayer_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081209_rage_cloudhatfragcontainer_updatebufferedstates(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081217_rage_cloudhatsettings_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081218_rage_cloudhatsettings_setcloudsettings_float_rage_cloudframe_con(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_081219_rage_cloudlistcontroller_fixupbitset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081220_rage_cloudlistcontroller_setuplist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081221_rage_cloudlistcontroller_updateprobabilitycontrols(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081225_rage_cloudhatfragcontainer_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_081231_rage_cloudhatfragcontainer_dtorcloudhatfragcontainer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081232_rage_cloudhatfraglayer_deactivate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081236_rage_cloudhatfraglayer_activate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081237_rage_cloudhatfragcontainer_starttransitionin_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_081238_rage_cloudhatfragcontainer_starttransitionout_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_081239_rage_cloudhatfragcontainer_transitionoutupdate_bool(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_081240_rage_cloudhatmanager_requestscriptcloudhat_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_081247_rage_cloudhatfragcontainer_addmaterialgroup_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081248_bool_creplaymgrinternal_recordfx_cpacketrequestcloudhat_cpacketr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081253_rage_anonymous_namespace_sbankonly_reloadsettings_rage_cloudhatm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081256_rage_cloudhatsettings_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_081260_rage_cloudlistcontroller_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_081262_rage_cloudhatsettings_cloudhatsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081263_rage_cloudhatsettings_synctchours_rage_ptxkeyframe_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081267_rage_cloudlistcontroller_setupprobabilitycontrols_rage_bkbank_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081272_rage_cloudlistcontroller_recomputelistprobabilitycol(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081273_rage_cloudlistcontroller_selectnewactivecloudhat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_081274_rage_atbitsetcore_unsigned_short_getnthbitindex_unsigned_int_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_081278_rage_cloudsettingsnamed_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_081290_rage_atmapentry_rage_atnonfinalhashstring_rage_cloudhatsettings(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_081291_rage_cloudhatsettings_cloudhatsettings_rage_cloudhatsettings_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_081292_rage_cloudsettingsmapbank_sbankonly_reloadsettings_rage_cloudset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081342_cdecalmanager_getrendersettingsindex_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_081343_cdecalmanager_addsettings_centity_ptr_rage_decalusersettings_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_081349_cdecalmanager_addgeneric_decaltype_e_int_int_rage_vec3v_const_re(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: f32, a7: f32, a8: f32, a9: f32, a10: f32, a11: f32, a12: f32, a13: f32, a14: i32, a15: i32, a16: i32, a17: f32, a18: i32, a19: i32, a20: f32) -> i32; }
extern "C" { pub fn f_081352_cdecalmanager_addbloodsplat_vfxbloodinfo_s_const_ref_vfxcollinfo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_081354_cdecalmanager_addpool_vfxliquidtype_e_int_int_rage_vec3v_const_r(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i64, a5: f32, a6: f32, a7: f32, a8: i32, a9: i32, a10: i32) -> i32; }
extern "C" { pub fn f_081355_cfire_getpositionworld_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081357_cdecalmanager_addbang_vfxmaterialinfo_s_const_ref_vfxcollinfo_s(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_081359_rage_decalusersettings_decalusersettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081360_cdecaldebug_placeplayervehiclebadge(ph: *mut Phantom); }
extern "C" { pub fn f_081361_cdecalmanager_processvehiclebadge_cvehicle_const_ptr_cvehiclebad(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: f32, a7: i32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_081362_cvehiclebadgemanager_request_cvehicle_const_ptr_cvehiclebadgedes(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: f32, a7: i32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_081365_cdecalmanager_removeallvehiclebadges_cvehicle_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081366_cvehiclebadgemanager_remove_cvehicle_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081372_cdecalmanager_addscriptedtrailinfo_rage_vec3v_const_ref_float_bo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_081374_cdecalmanager_addtrail_decaltype_e_vfxliquidtype_e_int_rage_vec3(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: f32, a7: i32, a8: i32, a9: f32, a10: i32, a11: i64, a12: i64, a13: i32, a14: i32) -> i32; }
extern "C" { pub fn f_081390_cdecalmanager_addscuff_centity_ptr_int_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: f32, a6: i32, a7: f32, a8: i32, a9: f32, a10: f32, a11: i32) -> i32; }
extern "C" { pub fn f_081391_cdecalmanager_addweaponimpact_vfxweaponinfo_s_const_ref_vfxcolli(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_081392_cdecalmanager_remove_rage_fwentity_const_ptr_int_void_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_081400_cptfxgpudropsystem_init_unsigned_int_int_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_081401_cptfxgpudropsystem_update_int_int_float_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_081402_cptfxgpudropsystem_interpolate_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_081404_cptfxgpusystem_prerender_rage_grcviewport_ref_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_081405_cptfxgpudropsystem_render_rage_grcviewport_ref_rage_ptxrenderset(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i64); }
extern "C" { pub fn f_081414_cptfxgpudropsystem_getsettingindex_rage_athashvalue(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_081416_ccoronas_register_rage_vec3v_const_ref_float_rage_color32_float(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: f32, a4: f32, a5: i64, a6: f32, a7: f32, a8: f32, a9: i32); }
extern "C" { pub fn f_081428_cdistantlights2_preparedistantcarsrenderdata(ph: *mut Phantom); }
extern "C" { pub fn f_081429_cdistantlights2_placelightsalonggroup_rage_scalarv_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32, a8: i32, a9: i32, a10: i32, a11: i64) -> i32; }
extern "C" { pub fn f_081430_cdistantlights2_loadtempcachedata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081431_cdistantlights2_loaddata_impl_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081432_cdistantlights2_findnextcompatiblelink_int_int_rage_ctempnode_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_081433_cdistantlights2_updategrouppositionandsize_distantlightgroup2_t(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081438_cfire_init_cfiresettings_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_081440_cfire_calccurrradius_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_081442_cfire_spread_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081443_cvfxweapon_doweaponimpactvfx_vfxcollinfo_s_const_ref_edamagetype(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_081444_cfiremanager_startpetrolfire_centity_ptr_rage_vec3v_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i32, a8: i32, a9: i32) -> i64; }
extern "C" { pub fn f_081445_cfiremanager_startfire_cfiresettings_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081446_cfire_findmtlburninfo_float_ref_float_ref_float_ref_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_081447_cfiremanager_startobjectfire_cobject_ptr_int_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32, a7: f32, a8: i32, a9: i64, a10: i64, a11: i32, a12: i32, a13: i32); }
extern "C" { pub fn f_081452_cfiremanager_startpedfire_cped_ptr_cped_ptr_int_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: f32, a9: i32, a10: i32, a11: i32) -> i64; }
extern "C" { pub fn f_081453_cfiremanager_startvehiclewreckedfire_cvehicle_ptr_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: f32); }
extern "C" { pub fn f_081454_cfiremanager_startvehiclefire_cvehicle_ptr_int_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32, a7: f32, a8: i64, a9: i64, a10: i32); }
extern "C" { pub fn f_081455_cfiremanager_registervehicletyrefire_cvehicle_ptr_int_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: i32) -> i64; }
extern "C" { pub fn f_081456_cfiremanager_registerfire_cfiresettings_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081457_bool_creplaymgrinternal_recordfx_cpacketregistervehiclefire_cpac(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081458_cfiremanager_registervehicletankfire_cvehicle_ptr_int_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i64); }
extern "C" { pub fn f_081463_cvfxfire_updateptfxfiresteam_cfire_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081464_cfiremanager_extinguishentityfires_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081472_cfogvolumemgr_renderfogvolume_fogvolume_t_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081488_cmoviemgr_removememoryallocationinfo_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081489_rage_sysmessagequeue_cmoviemgr_sallocdata_32_true_push_cmoviemgr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081490_cmoviemgr_cmovie_setvolume_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_081493_cmoviemgr_cmovie_play(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081494_cmoviemgr_stop_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081498_cmoviemgr_preloadmovie_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_081519_cvfxlens_register_vfxlenstype_float_float_float_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_081520_cwatercannons_registercannon_cvehicle_ptr_unsigned_long_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081524_cmoviemeshmanager_releasemoviesfromset_moviemeshset_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081525_moviemeshset_dtormoviemeshset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081527_cmoviemeshmanager_processloadrequest_moviemeshset_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081528_cmoviemeshmanager_updateset_moviemeshset_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081529_cmoviemeshmanager_querysetstatus_unsigned_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_081530_cmoviemeshmanager_loadset_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_081544_puddlepass_getsettings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081546_ripples_addplayerripple_rage_vec3v_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_081567_ctvplaylistcontainer_dtorctvplaylistcontainer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081569_ctvplaylistslot_getvideoforthisslot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081570_ctvchannelinfo_setplaylist_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_081571_ctvchannelinfo_setplaylistfromhour_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_081574_ctvplaylistmanager_getvideoinfo_int_int_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_081575_ctvplaylistmanager_dumpcachecontentinfo(ph: *mut Phantom); }
extern "C" { pub fn f_081600_cflaretypefx_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_081611_clensflare_addsinglefxbank_int_int_rage_bkgroup_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_081620_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_cflaref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081634_terraingrid_initshaders(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081637_cptfxcolnbase_renderdebug(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081639_cptfxcollision_processgroundbounds_rage_ptxcollisionset_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_081640_cptfxcollision_processnearbybounds_rage_ptxcollisionset_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_081641_cptfxcollision_addinstboundstocolnset_rage_ptxcollisionset_ptr_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_081643_cptfxcollision_addboundstocolnset_rage_ptxcollisionset_ptr_int_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i32); }
extern "C" { pub fn f_081651_cptfxmanager_removeptfxfromentity_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081708_cptfxmanager_initrmptfxtexture(ph: *mut Phantom); }
extern "C" { pub fn f_081711_cvfxtrail_registerpoint_vfxtrailinfo_t_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081712_cptfxmanager_storeglassimpact_ptfxglassimpactinfo_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081717_cptfxmanager_setglobalconstants(ph: *mut Phantom); }
extern "C" { pub fn f_081719_cptfxmanager_relocateentityptfx_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081723_cshadervariable_rage_pgref_rage_grctexture_cshadervariable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081724_cvehicleglasscomponent_initcomponentgeom_rage_grmgeometry_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_081725_cvehicleglasscomponent_updatevb_cvgstacktriangle_const_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_081726_cvfxweapon_getinfo_vfxgroup_e_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_081727_cvehicleglasscomponent_shutdowncomponentgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081728_cvehicleglassvb_destroy_cvehicleglassvb_ptr_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081729_cvehicleglasscomponent_processtessellation_cphysical_const_ptr_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32, a6: i32, a7: i64); }
extern "C" { pub fn f_081730_cvehicleglassmanager_getsmashwindowtargetcomponent(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_081731_cvehicleglasscomponent_addtriangletessellated_cvgstacktriangle_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i64, a13: i64, a14: i64, a15: i64, a16: i64, a17: i64, a18: i64, a19: i64); }
extern "C" { pub fn f_081732_cvehicleglassmanager_removefurthestcomponent_cphysical_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081733_cvehicleglasscomponent_rendercomponent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081735_cvehicleglasscomponent_recursivetessellatenospheretest_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_081736_cvehicleglasscomponent_recursivetessellatewithspheretest_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i64, a10: i64, a11: i64, a12: f32); }
extern "C" { pub fn f_081741_cvehicleglassmanager_cancomponentbreak_cphysical_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_081742_getsmashableglassmaterialid_cphysical_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_081743_cvehicleglassmanager_createcomponent_cphysical_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_081745_cvehicleglassmanager_smashcollision_centity_ptr_int_float_rage_v(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_081746_cvehicleglassmanager_storecollision_vfxcollinfo_s_const_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081747_bool_creplaymgrinternal_recordfx_cpacketptexweaponshot_cpacketpt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081748_cvehicleglassmanager_smashexplosion_centity_ptr_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_081750_cvehicleglassmanager_iscomponentsmashableglass_cphysical_const_p(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_081751_cvehicleglassmanager_removecomponent_cphysical_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081752_cvehicleglassmanager_applyvehicledamage_cvehicle_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081753_cvehicleglassmanager_prerender2_cphysical_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081763_cvfxblood_updateptfxbloodmist_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081765_cvfxentity_loadgenericptfxdata_rage_fiasciitokenizer_ref_vfxenti(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_081766_rage_atarray_rage_atbinarymap_vfxentitybreakptfxinfo_s_rage_atno(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081767_cvfxentity_loadgenericdecaldata_rage_fiasciitokenizer_ref_vfxent(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_081768_rage_atarray_rage_atbinarymap_vfxentitybreakdecalinfo_s_rage_atn(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081769_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081770_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081771_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081772_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081773_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081774_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081775_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081776_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081777_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081778_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081779_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081780_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081784_cvfxentity_triggerptfxentityshot_centity_ptr_cparticleattr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081786_void_creplaymgr_recordfx_cpacketentityambientfx_cpacketentityamb(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081791_cvfxexplosion_triggerptfxexplosion_vfxexplosioninstdata_s_ref_vf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_081792_cvfxexplosion_readdata_cdatafilemgr_datafile_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081793_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081794_rage_atarray_rage_atbinarymap_vfxexplosioninfo_s_rage_atnonfinal(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_081796_vfxexplosioninstdata_s_dtorvfxexplosioninstdata_s(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081803_rage_vfxlightningdirectionalburstsettings_addwidgets_rage_bkbank(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081804_rage_vfxlightningkeyframes_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_081813_rage_vfxlightningcloudburstsettings_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_081822_rage_vfxlightningstrikevariationsettings_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_081825_rage_vfxlightningstrikesettings_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_081828_rage_vfxlightningstrikesettings_vfxlightningstrikesettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081837_rage_vfxlightningcloudburstcommonsettings_addwidgets_rage_bkbank(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081838_rage_vfxlightningstrikevariationsettings_addwidgets_rage_bkbank(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081839_cvfxliquidattachinfo_update_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_081840_cvfxliquidattachinfo_set_vfxliquidattachtype_e_vfxliquidtype_e(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_081841_cvfxliquidattachinfo_getcurralpha_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_081842_cvfxmaterial_domtlscrapefx_centity_ptr_int_centity_ptr_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: f32, a11: f32, a12: f32, a13: f32, a14: f32); }
extern "C" { pub fn f_081850_cvfxprojectile_updateptfxprojfuse_cprojectile_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_081852_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081860_cvfxvehicle_processpetroltankdamage_cvehicle_ptr_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081861_cvfxvehicle_updateptfxthrusterjet_cvehicle_ptr_int_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32); }
extern "C" { pub fn f_081862_cvfxvehicle_updateptfxelectricrambar_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081864_cvfxvehicle_triggerptfxlightsmash_cvehicle_ptr_int_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_081865_cvfxvehicle_updateptfxplaneafterburner_cvehicle_ptr_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_081866_cvfxvehicle_updateptfxaircraftsectiondamagesmoke_cvehicle_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_081867_cvfxvehicle_processgrounddisturb_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081868_cvfxvehicle_processdownwash_cvehicle_ptr_cvfxvehicleinfo_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32); }
extern "C" { pub fn f_081869_cvfxvehicle_triggerptfxpropellerdestroy_cplane_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081870_cvfxvehicle_triggerptfxhelirotordestroy_crotarywingaircraft_ptr(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081871_cvfxvehicle_updateptfxtrainwheelsparks_cvehicle_ptr_int_rage_vec(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_081873_cvfxvehicle_processwreckedfires_cvehicle_ptr_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081874_cvfxvehicle_triggerptfxvehicledebris_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081875_cvfxvehicle_hasrecentlybeendamagedbyplayer_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_081879_cvfxwater_triggerptfxsplashpedin_cped_ptr_cvfxpedinfo_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i32); }
extern "C" { pub fn f_081880_cvfxwater_triggerptfxsplashpedout_cped_ptr_cvfxpedinfo_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32); }
extern "C" { pub fn f_081881_cvfxwater_updateptfxsplashpedwade_cped_ptr_cvfxpedinfo_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: f32, a8: i32); }
extern "C" { pub fn f_081882_bool_creplaymgrinternal_recordfx_cpacketwatersplashvehiclefx_cpa(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081885_cvfxwater_updateptfxboatbow_cvehicle_ptr_cvfxvehicleinfo_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_081886_cvfxwater_updateptfxboatwash_cvehicle_ptr_cvfxvehicleinfo_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_081887_cvfxwater_updateptfxpropeller_cvehicle_ptr_int_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32); }
extern "C" { pub fn f_081888_cvfxwater_triggerptfxsplashheliblade_centity_ptr_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081899_cvfxweapon_triggerptfxgunshell_centity_const_ptr_cweapon_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_081901_cvfxweapon_getweapontintcolour_unsigned_char_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081904_cptfxgpudropsystem_dtorcptfxgpudropsystem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081906_cvfxwheel_processwheellights_cwheel_ptr_cvehicle_ptr_cvfxvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32); }
extern "C" { pub fn f_081907_cvfxwheel_updateptfxwheelfriction_cwheel_ptr_cvehicle_ptr_cvfxve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32, a7: f32, a8: i32); }
extern "C" { pub fn f_081908_cvfxwheel_updateptfxwheeldisplacement_cwheel_ptr_cvehicle_ptr_cv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: f32, a7: f32, a8: f32, a9: i32); }
extern "C" { pub fn f_081909_cvfxwheel_updateptfxwheelburnout_cwheel_ptr_cvehicle_ptr_cvfxveh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_081910_cvfxwheel_triggerptfxwheelpuncture_cwheel_ptr_cvehicle_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_081911_cvfxwheel_triggerptfxwheelburst_cwheel_ptr_cvehicle_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_081966_bool_creplaymgrinternal_handleentityforfx_cpacketvehiclebadgereq(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_081967_bool_creplaymgrinternal_handleentityforfx_cpacketblooddecal_cpac(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_081968_bool_creplaymgrinternal_handleentityforfx_cpacketdecalremove_cpa(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_081969_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_081970_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_081971_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_081972_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_081973_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_081974_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_081975_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_081976_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_081977_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_081978_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_081979_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_081980_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_081981_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_081982_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_081983_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_081984_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_081985_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_081986_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_081987_void_std_2_pop_heap_abi_nqn220108_std_2_classicalgpolicy_bool_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_081988_rage_atbinarymap_vfxentitybreakptfxinfo_s_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_081989_std_2_pair_rage_atbinarymap_vfxentitybreakptfxinfo_s_rage_atnonf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_081990_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_081991_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_081992_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_081993_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_081994_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_081995_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_081996_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_081997_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_081998_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_081999_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_082000_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_082001_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_082002_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_082003_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_082004_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_082005_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_082006_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_082007_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_082008_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_082009_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_082010_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_082011_void_std_2_pop_heap_abi_nqn220108_std_2_classicalgpolicy_bool_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_082012_rage_atbinarymap_vfxentitybreakdecalinfo_s_rage_atnonfinalhashst(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_082013_std_2_pair_rage_atbinarymap_vfxentitybreakdecalinfo_s_rage_atnon(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_082014_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_082015_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_082016_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_082017_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_082018_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_082020_cairdefencezone_isentitytargetable_centity_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082021_cairdefencezone_shouldfireweapon_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082026_cweapon_cweapon_unsigned_int_int_cdynamicentity_ptr_bool_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_082028_cexplosionmanager_addexplosion_cexplosionmanager_cexplosionargs(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082032_cweapon_spinup_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082044_cairdefencemanager_getairdefencezone_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082045_cairdefencemanager_ispositionindefencezone_rage_vec3v_unsigned_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082047_cairdefencemanager_fireweaponatposition_unsigned_char_rage_vec3v(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_082064_cpedweaponmanager_getequippedvehicleweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082070_cweaponinfo_getammoinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082071_cpedweaponmanager_getequippedweapon_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082087_cexplosioninfomanager_getexplosiontagdata_eexplosiontag(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082091_phgtaexplosioninst_getdamagetype(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082092_phgtaexplosioninst_getphysicaltype(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082123_cweapondamage_generatepeddamageevent_centity_ptr_cped_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64, a5: i64, a6: i64, a7: i32, a8: f32, a9: i64, a10: i32, a11: f32, a12: i32) -> f32; }
extern "C" { pub fn f_082129_eexplosiontag_parse_rage_athashvalue_eexplosiontag(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082130_phgtaexplosioninst_getweapongroup(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082146_cfiringpattern_processfired(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082173_cweapon_process_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_082176_cweapon_processweaponspin_centity_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> f32; }
extern "C" { pub fn f_082184_cweapon_calcfirevecfromaimcamera_centity_const_ptr_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_082267_citeminfo_getmodelindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082521_cweaponinfo_getvehicleweaponinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082555_cpedinventoryloadoutmanager_setloadout_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082580_cpedinventory_addweaponandammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_082594_cpedweaponmanager_equipweapon_unsigned_int_int_bool_bool_cpedequ(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_082685_cprojectile_applydeformation_cvehicle_const_ptr_void_const_ptr_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_082823_sysmemstartnetwork(ph: *mut Phantom); }
extern "C" { pub fn f_082824_sysmemendnetwork(ph: *mut Phantom); }
extern "C" { pub fn f_082885_networkinterface_isanysessionactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082908_networkinterface_ishost(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082909_cnetworktelemetry_appendmetric_rage_rlmetric_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082999_networkinterface_registerobject_rage_netgameobjectwrapper_cdynam(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083019_networkinterface_issessionestablished(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083020_networkinterface_issessionleaving(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083025_networkinterface_getphysicalplayerfromindex_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083028_networkinterface_isinspectatormode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083034_networkinterface_isfriendlyfireallowed_cped_const_ptr_centity_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083035_networkinterface_isfriendlyfireallowed_cvehicle_const_ptr_centit(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083041_networkinterface_canregisterobject_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083042_networkutils_isclosetoanyplayer_rage_vector3_const_ref_float_rag(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_083043_networkinterface_getplayerfocusposition_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_083044_networkinterface_isvisibleorclosetoanyplayer_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_083046_networkinterface_isvisibleorclosetoanyremoteplayer_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083047_networkutils_isvisibletoanyremoteplayer_rage_vector3_const_ref_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i32; }
extern "C" { pub fn f_083048_networkutils_isvisibletoplayer_cnetgameplayer_const_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_083049_networkutils_getclosestremoteplayers_rage_vector3_const_ref_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_083051_networkinterface_getnumactiveplayers(ph: *mut Phantom); }
extern "C" { pub fn f_083061_networkinterface_getnetworkobject_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083062_networkinterface_getplayercameraposition_rage_netplayer_const_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083063_networkinterface_unregisterobject_rage_netgameobjectwrapper_cdyn(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083064_networkinterface_forcesortnetworkobjectmap(ph: *mut Phantom); }
extern "C" { pub fn f_083065_networkinterface_allowotherthreadsaccesstonetworkmanager_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083066_networkinterface_forceveryhighupdatelevelonvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083072_networkinterface_setprocesslocalpopulationlimits_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083108_networkinterface_isposlocallycontrolled_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083111_networkinterface_isvehiclelockedforplayer_cvehicle_const_ptr_cpe(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083112_networkinterface_settargetlockon_cvehicle_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083114_networkinterface_isinvalidvehicleforambientpeddriving_cvehicle_c(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083116_networkutils_getvehiclefromnetworkobject_rage_netobject_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083119_networkinterface_forcehealthnodeupdate_cvehicle_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083120_networkinterface_onentitysmashed_cphysical_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_083122_networkinterface_onvehiclepartdamage_cvehicle_ptr_ehierarchyid_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_083123_networkinterface_shouldtriggerdamageeventforzerodamage_cphysical(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083125_networkinterface_getlastposreceivedovernetwork_cdynamicentity_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083126_networkinterface_getlastvelreceivedovernetwork_cdynamicentity_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083136_networkinterface_isinsingleplayerprivatesession(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083151_networkinterface_registerkillwithnetworktracker_cphysical_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083161_networkinterface_findclosestwantedplayer_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_083162_networkinterface_isdamagedisabledinmp_centity_const_ref_centity(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083167_networkinterface_areinteractionsdisabledinmp_centity_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083172_networkinterface_isaghostvehicle_cvehicle_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083181_networkutils_getnetworkplayerviewport_cnetgameplayer_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083201_networkinterface_writedatapopulationlog_char_const_ptr_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_083312_networkcrewemblemmgr_requestemblem_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083316_networkcrewemblemmgr_releaseemblem_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083334_networkcrewemblemmgr_getemblemname_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083342_networkclan_getcrewemblemmgr(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083350_cloudlistener_cloudlistener(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083493_ctunables_access_unsigned_int_bool_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083494_ctunables_access_unsigned_int_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083496_ctunables_tryaccess_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083497_ctunables_tryaccess_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083498_ctunables_tryaccess_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> f32; }
extern "C" { pub fn f_083770_networkcolours_getnetworkcolour_networkcolours_networkcolour(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083771_networkcolours_getdefaultplayercolour_unsigned_char(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083820_cnetfxidentifier_isclone_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083821_cnetfxidentifier_getnextfxid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083898_csyncedentity_csyncedentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083899_csyncedentity_csyncedentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083900_csyncedentity_setentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083901_csyncedentity_csyncedentity_csyncedentity_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083902_csyncedentity_getentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083903_csyncedentity_getentityid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083904_csyncedentity_setentityid_unsigned_short_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084088_clivemanager_issystemuishowing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084340_cnetworktelemetry_emergencysvcscalled_int_rage_vector3_const_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_084348_cnetworktelemetry_appendmetrictvshow_unsigned_int_unsigned_int_u(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_084350_cnetworktelemetry_appendinfometric_unsigned_int_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_084449_networkclan_getprimaryclan_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_086743_cnetworkobjectpopulationmgr_canregisterlocalobjectsoftype_unsign(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_086744_cnetgameplayer_getplayerped_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086763_cnetobjgame_getplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086767_cnetobjtrain_cnetobjtrain_ctrain_ptr_unsigned_short_unsigned_sho(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_086769_cnetobjvehicle_cnetobjvehicle_cvehicle_ptr_unsigned_short_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_086781_cnetworkobjectpopulationmgr_getnumlocalobjects_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_086785_cnetobjentity_flagfordeletion(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086794_cnetworkobjectpopulationmgr_findallocatedexternalreservations_un(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_086795_cnetworkobjectpopulationmgr_setcurrentexternalreservationindex_u(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_086796_cnetworkobjectpopulationmgr_clearcurrentreservationindex(ph: *mut Phantom); }
extern "C" { pub fn f_086799_cnetworkobjectpopulationmgr_getmaxobjectsallowedtoownoftype_unsi(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_086800_cnetworkobjectpopulationmgr_canregisterobjects_unsigned_int_unsi(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_086801_cnetworkobjectpopulationmgr_getlastfailtypestring(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_086813_cnetblenderphysical_getlastmatrixreceived_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086894_cnetobjobject_hastobecloned_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_087068_cnetobjphysical_updateblenderstate_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_087106_cnetobjobject_trytoregisterambientobject_cobject_ref_bool_char_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_087352_cnetobjphysical_stopalpharamping(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087390_cnetobjphysical_setalpharampinginternal_bool_bool_bool_bool_shor(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_087537_cnetobjplayer_getaccurateremoteplayerposition_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087669_cnetobjproximitymigrateable_keepproximitycontrol_unsigned_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087744_cnetobjvehicle_resetvehiclebadge_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087750_cnetobjvehicle_lockforplayer_unsigned_char_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087751_cnetobjvehicle_lockforallplayers_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087755_cnetobjvehicle_evictallpeds(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087757_cnetobjvehicle_issuperdummystilllaunchedintoair_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087789_cnetobjvehicle_dirtyvehicledamagestatusdatasyncnode(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_088035_csyncedpedvehicleentryscriptconfig_validatedata_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_088036_csyncedpedvehicleentryscriptconfig_spewdatatotty_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_088041_csyncedpedvehicleentryscriptconfig_hasarearforcingflagsetforthis(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_089329_crequestcontrolevent_trigger_rage_netobject_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_089361_cweapondamageevent_trigger_centity_ptr_centity_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i32, a14: i32, a15: i32, a16: i32, a17: i64); }
extern "C" { pub fn f_089462_cfireevent_getplayersinscopeforfiredata_cfireevent_sfiredata_con(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_089463_cfireevent_addfiredata_cfireevent_sfiredata_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_089691_ccarhornevent_trigger_bool_unsigned_short_unsigned_char(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_089749_cscriptentitystatechangeevent_trigger_rage_netobject_ptr_cscript(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_089759_cplaysoundevent_trigger_centity_const_ptr_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_089817_cnetworktrainreportevent_trigger_int_bool_unsigned_char(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_089849_crequestdetachmentevent_trigger_cphysical_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_089881_cblowupvehicleevent_trigger_cvehicle_ref_centity_ptr_bool_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_089890_cactivatevehiclespecialabilityevent_trigger_cvehicle_ref_cactiva(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_089937_cnetworkinfochangeevent_trigger_unsigned_int_unsigned_int_unsign(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090710_asinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090714_atanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090717_atof(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090757_expf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090804_log10(ph: *mut Phantom, a0: f64) -> f64; }
extern "C" { pub fn f_090818_modff(ph: *mut Phantom, a0: f32, a1: i64) -> f32; }
extern "C" { pub fn f_090826_powf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
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
extern "C" { pub fn f_090940_strncasecmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090945_strrchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090947_strstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090956_strtok(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090965_tanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_091002_emscripten_builtin_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091023_multi3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
