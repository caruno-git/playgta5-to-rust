//! Callee declarations for crate gen008.
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
extern "C" { pub fn f_000128_rage_sysipccreatethread_void_ptr_void_ptr_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_000131_rage_sysipcwaitthreadexit_rage_sysipcthreadid_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000135_rage_sysipcsleep_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000136_rage_sysipcestimatestackusage_unsigned_int_ref_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000173_rage_fidevice_convertfiletimetosystemtime_unsigned_long_rage_fid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000174_rage_fidevice_getlocalsystemtime_rage_fidevice_systemtime_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000242_rage_grcdevice_gethasfocus(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000574_gmutex_gmutex_bool_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000575_gmutex_dtorgmutex(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000579_gmutex_lock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000580_gmutex_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000590_gwaitcondition_gwaitcondition(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000591_gwaitcondition_dtorgwaitcondition(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000592_gwaitcondition_wait_gmutex_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000593_gwaitcondition_notifyall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000599_rage_voicechat_setcapturedevicebyid32_unsigned_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000600_rage_voicechat_freecapturedevice(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000601_rage_voicechat_setplaybackdevicebyid32_unsigned_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000602_rage_voicechat_freeplaybackdevice(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000603_rage_voicechat_setcapturedevicevolume_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_000604_rage_voicechat_setplaybackdevicevolume_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_000605_rage_voicechat_setcapturemode_rage_voicechat_capturemode(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000607_rage_voicechat_setvoicetestmode_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000609_rage_voicechat_nativepackchat_int_void_ptr_unsigned_int_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000644_rage_athashnextsize_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000645_rage_athash_const_char_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000646_rage_athash64_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000654_rage_atstring_set_char_const_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_000656_rage_atstring_uppercase(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000659_rage_atstring_indexof_char_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_000660_rage_atstring_indexof_char_const_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000661_rage_atstring_lastindexof_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_000663_rage_atstring_replace_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000665_rage_atstring_op_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000667_rage_atstring_split_rage_atarray_rage_atstring_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_000668_rage_atarray_rage_atstring_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_000669_rage_atstring_trim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000672_rage_atstring_op_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000675_rage_functorbase_functorbase_void_const_ptr_void_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_000680_rage_athashstringnamespacesupport_computehash_rage_athashstringn(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_000681_rage_athashstringnamespacesupport_addstring_rage_athashstringnam(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_000684_rage_athashstringnamespacesupport_trygetstring_rage_athashstring(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_000692_rage_datcallback_call_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000696_rage_bkwidget_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000703_rage_bkbank_unsetcurrentgroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000704_rage_bkbank_pushgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000705_rage_bkgroup_addgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000707_rage_bkbank_popgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000779_rage_bkmanager_createbank_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000782_rage_bkmanager_findbank_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000783_rage_bkmanager_findwidget_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000786_rage_bkmanager_isbankthread(ph: *mut Phantom) -> i32; }
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
extern "C" { pub fn f_000874_rage_bkgroup_addtext_char_const_ptr_int_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000876_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_000877_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_000880_rage_bkgroup_addtext_char_const_ptr_rage_atnonfinalhashstring_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000882_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000888_rage_bkgroup_addtoggle_char_const_ptr_unsigned_short_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000894_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000898_rage_bkgroup_addslider_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000900_rage_bkgroup_addslider_char_const_ptr_signed_char_ptr_signed_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_000902_rage_bkgroup_addslider_char_const_ptr_unsigned_short_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000904_rage_bkgroup_addslider_char_const_ptr_short_ptr_short_short_shor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000906_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000907_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int_rage_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000908_rage_bkgroup_addslider_char_const_ptr_rage_vector2_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64); }
extern "C" { pub fn f_000909_rage_bkgroup_addslider_char_const_ptr_rage_vector2_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000918_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000919_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000921_rage_bkgroup_addlist_char_const_ptr_bool_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000925_rage_bkgroup_addvector_char_const_ptr_rage_vector2_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001062_rage_matrix34_inverse_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001116_char_ptr_rage_formatf_char_32ul_char_ref_32ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001117_rage_datcallback_datcallback(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001119_rage_datcallback_datcallback_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001120_rage_datcallback_datcallback_void_ptr_void_ptr_void_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_001204_rage_rsonwriter_rsonwriter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001205_rage_rsonwriter_init_char_ptr_unsigned_int_rage_rsonformat(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001210_rage_rsonwriter_begin_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001214_rage_rsonwriter_end(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001216_rage_rsonwriter_writeint_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001218_rage_rsonwriter_writeint64_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001224_rage_rsonwriter_writestring_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001232_rage_rsonreader_rsonreader(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001233_rage_rsonreader_rsonreader_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001238_rage_rsonreader_getmember_char_const_ptr_rage_rsonreader_ptr_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001246_rage_rsonreader_asint_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001253_rage_rsonreader_asstring_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001514_rage_diagdisplayf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001516_rage_fistream_write_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001524_displayf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001527_warningf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001530_rage_fistream_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001531_rage_fistream_size64(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001532_rage_fistream_close(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001533_rage_fidevice_getdevice_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001534_quitf_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001535_rage_diaglogf2_rage_diagchannel_const_ref_rage_diagseverity_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001537_rage_fistream_size(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001550_char_ptr_rage_formatf_char_12ul_char_ref_12ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001554_rage_diaglogf_rage_diagchannel_const_ref_rage_diagseverity_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001560_rage_diagcontextmessage_diagcontextmessage_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001561_rage_diagcontextmessage_diagcontextmessage_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_001563_rage_diagcontextmessage_pop(ph: *mut Phantom); }
extern "C" { pub fn f_001568_rage_atmapentry_unsigned_int_wchar_t_ptr_const_ref_rage_atmap_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001578_rage_fistream_seek_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001580_rage_fiassetmanager_open_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001582_rage_fiassetmanager_createleadingpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001622_rage_fistream_read_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001623_rage_fiassetmanager_findextensioninpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001627_rage_fiassetmanager_exists_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001635_rage_fiassetmanager_filename_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001639_rage_fiassetmanager_removeextensionfrompath_char_ptr_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001646_rage_fistream_seek64_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001647_rage_fistream_tell(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001648_rage_fistream_tell64(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001652_rage_fidevice_makememoryfilename_char_ptr_int_void_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_001874_void_rage_atplaceptrcreator_void_ptr_rage_fideviceinstaller_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001908_int_rage_formatf_n_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002037_rage_pgrscbuilder_constructname_char_ptr_int_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_002044_rage_pgrscbuilder_computeleafsize_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002080_rage_pgbase_pgbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002092_rage_datresource_getfixup_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002111_rage_parmanager_parmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002112_rage_parmanager_initialize_rage_parsettings_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002128_rage_parmanager_uninitialize(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002129_rage_atmapentry_rage_atnonfinalhashstring_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002131_rage_atnonfinalhashstring_atnonfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002132_rage_parmanager_dtorparmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002147_rage_parmanager_loadtree_char_const_ptr_char_const_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002159_rage_parmanager_loadfromstructure_rage_partreenode_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002174_rage_parelement_findattributestringvalue_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_002175_rage_partree_dtorpartree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002181_rage_partreenode_findfromxpathinternal_char_const_ptr_rage_parat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002182_rage_parelement_findattribute_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002183_rage_partreenode_findchildwithindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002184_rage_partreenode_findchildwithname_char_const_ptr_rage_partreeno(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002188_rage_partreenode_appendaschildof_rage_partreenode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002203_rage_parelement_findattributeboolvalue_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002206_rage_parelement_findattributeint64value_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002213_rage_partreenode_clone(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002215_rage_partreenode_findnumchildren_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002242_rage_atmapentry_rage_atstring_rage_psoresourcedata_ptr_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002280_rage_atfinalhashstring_atfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002311_rage_atarray_rage_psoenumtableentry_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002352_rage_parenumdata_valuefromname_char_const_ptr_char_const_ptr_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002402_rage_parenumdata_namefromvalueunsafe_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002403_rage_parenumdata_verifynamesexist_bool_char_const_ptr_const(ph: *mut Phantom); }
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
extern "C" { pub fn f_002448_int_rage_formatf_n_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002582_rage_parelement_findattributeanycase_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002593_rage_parsettings_createstandardsettings(ph: *mut Phantom); }
extern "C" { pub fn f_002718_strupr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002719_strlwr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002720_rage_conststring_vcopy_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002721_rage_conststring_dtorconststring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002725_rage_atstringbuilder_atstringbuilder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002728_rage_atstringbuilder_dtoratstringbuilder(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002731_rage_atstringbuilder_append_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002736_rage_atstringbuilder_tostring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002739_rage_atpartialstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002742_rage_atpartialdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002744_rage_atdatahash_unsigned_int_const_ptr_unsigned_long_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002746_rage_stristr_char_ptr_char_const_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002748_rage_stringwildcardcompare_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002757_rage_countutf8characters_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002760_rage_widetoutf8_char_ptr_unsigned_short_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002762_rage_utf8towide_unsigned_short_ptr_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002763_rage_utf8removeinvalidsurrogates_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002778_inflateinit(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002779_inflate(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002781_inflateend(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002799_rage_sysappcontent_isjapanesebuild(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002802_rage_syshangdetectsavezoneenter(ph: *mut Phantom); }
extern "C" { pub fn f_002803_rage_syshangdetectsavezoneexit_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002811_rage_syshangdetectsetcrashfunc_void_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002840_rage_sysparam_get_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002849_rage_sysbootmanager_isbootedfromdisc(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002871_rage_sysgetenv_char_const_ptr_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002878_rage_atmap_unsigned_int_rage_sysstackcollector_tagentry_rage_atm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002881_unsigned_int_ref_rage_atmap_unsigned_int_unsigned_int_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002919_rage_sysservice_isconstrained_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002921_rage_sysservice_getsystemlanguage(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002927_rage_sysmemstarttemp(ph: *mut Phantom); }
extern "C" { pub fn f_002928_rage_sysmemendtemp(ph: *mut Phantom); }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_003097_char_ptr_rage_formatf_char_16ul_char_ref_16ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
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
extern "C" { pub fn f_003221_rage_crframe_dtorcrframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003296_rage_atarray_unsigned_short_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003816_rage_fwanimdirector_updatemove_float_bool_rage_fwanimdirector_pt(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003822_rage_fwanimdirector_startupdatemidphysics_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_003825_rage_fwanimdirector_startupdateprerender_float_rage_crmtmotiontr(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32); }
extern "C" { pub fn f_003828_rage_fwanimdirector_findpropertyinmovenetworks_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003859_rage_fwanimdirector_waitformotiontreebyphase_rage_fwanimdirector(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003876_rage_fwanimmanager_getclipifexistsbysetid_rage_fwmvtemplateid_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003896_rage_fwanimmanager_getclipifexistsbydictindex_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_003984_rage_fwclipsetrequesthelper_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003993_rage_fwclipsetmanager_getclipsetid_rage_atnonfinalhashstring_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004019_rage_fwclipsetrequesthelper_request_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_004037_rage_fwclipsetrequesthelper_dtorfwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004038_rage_fwclipsetrequesthelper_fwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004039_rage_fwclipsetrequesthelper_request_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004041_rage_fwclipsetrequesthelper_isloaded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004082_rage_fwfacialclipsetgroupmanager_getfacialclipsetgroup_rage_fwmv(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004280_rage_fwposematcher_fwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004634_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004737_rage_fwdecorator_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004748_rage_fwdecoratorextension_find_rage_atnonfinalhashstring_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004819_rage_fwvectormap_drawmarker_rage_vector3_const_ref_rage_color32(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_004820_rage_fwvectormap_drawcircle_rage_vector3_const_ref_float_rage_co(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_004930_char_ptr_rage_formatf_char_10ul_char_ref_10ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004935_rage_dlcmdbase_op_new_unsigned_long_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005003_rage_dldrawlistmgr_addtxdreference_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005007_rage_dldrawlistmgr_addarchetypereference_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005032_rage_fwlanguagepack_getdateformattype_rage_syslanguage(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005036_rage_fwlocalisedstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_005037_rage_atarray_char_0_unsigned_short_resizegrow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005049_rage_fwtextutil_truncatefileextension_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005052_rage_fwtextutil_comaprefilenamesignoreextension_char_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005053_rage_fwtextutil_getbytecount_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005054_rage_fwtextutil_bytecountforcharacter_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005055_rage_fwtextutil_strcpywithcharacterandbytelimits_char_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_005057_rage_fwtextutil_isspace_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005284_rage_fwrenderlistgroup_clear(ph: *mut Phantom); }
extern "C" { pub fn f_005313_rage_fwrenderthreadinterface_isusingdefaultrenderfunction(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_005315_rage_fwrenderthreadinterface_flush_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005318_rage_fwrenderthreadinterface_setrenderfunction_rage_functor0(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005513_rage_fwtxdstore_setcurrenttxd_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005514_rage_fwtxdstore_popcurrenttxd(ph: *mut Phantom); }
extern "C" { pub fn f_005561_sprintf_s_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005564_rage_fwassetstore_rage_pgdictionary_rage_grctexture_rage_fwtxdde(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005646_rage_atmap_rage_strindex_rage_fwpsostoreplacementcookies_cookie(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005650_rage_fwpsostoreloader_loaddataintoobjectcore_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005651_rage_fwpsostoreloader_makesimplefileloadercore(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005702_void_rage_pgrscbuilder_load_rage_pgdictionary_rage_grctexture_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_005706_rage_fwtxdstore_getsafefromindex_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005707_rage_fwtxdstore_pushcurrenttxd(ph: *mut Phantom); }
extern "C" { pub fn f_005714_rage_fwtxdstore_removeref_rage_strlocalindex_rage_strrefkind(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005732_rage_fwinteriorlocation_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005733_rage_fwinteriorlocation_getroomindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005888_rage_fwentitylist_celement_ptr_std_2_partial_sort_impl_abi_nqn22(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_005889_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_005890_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_005917_rage_fwrefawarebaseimpl_rage_datbase_dtorfwrefawarebaseimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005949_rage_aitasktree_findtaskbytypeactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005950_rage_aitasktree_findtaskbytypewithpriority_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_006084_rage_strstreaminginfomanager_requestobject_rage_strindex_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006088_rage_atarray_rage_atnonfinalhashstring_0_unsigned_short_grow_uns(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006101_rage_strstreaminginfomanager_setrequiredflag_rage_strindex_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006156_rage_atarray_rage_strindex_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006167_rage_strstreaminginfomanager_isobjectinimage_rage_strindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006175_rage_strstreaminginfomanager_clearrequiredflag_rage_strindex_uns(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006215_rage_strstreamingloadermanager_loadrequestedobjects(ph: *mut Phantom); }
extern "C" { pub fn f_006235_rage_strstreamingvisualize_setmarkertext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006237_rage_strstreamingloadermanager_processasyncfiles(ph: *mut Phantom); }
extern "C" { pub fn f_006241_rage_strstreamingvisualize_addmarker_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006246_rage_strstreamingmodule_getvirtualmemoryofloadedobj_rage_strloca(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_006250_rage_strstreamingmodule_getrefcountstring_rage_strlocalindex_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_006254_rage_strrequest_request_rage_strlocalindex_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_006255_rage_strrequest_hasloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006256_rage_strrequest_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006257_rage_strrequest_clearrequiredflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006258_rage_strrequest_clearmyrequestflags_unsigned_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006260_rage_strrequest_delayedrelease(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006717_rage_audsound_stopandforget_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006925_rage_audcontroller_clearallsounds(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006930_rage_audcontroller_stopallsounds(ph: *mut Phantom); }
extern "C" { pub fn f_007079_rage_audsoundset_find_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007411_rage_audentity_addbatchedsoundrequest_rage_audmetadataref_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007653_rage_fwprofanityfilter_checkforreservedtermsonly_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007654_rage_fwprofanityfilter_checkforunsupportedfilesystemcharacterson(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007656_rage_fwprofanityfilter_substitutecharacters_char_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007657_rage_fwprofanityfilter_checkforprofanetermsraw_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007658_rage_fwptrlistsinglelink_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007771_rage_netobjectmgrbase_getnetworkobject_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007802_rage_netplayer_ismyplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007848_rage_neteventmgr_checkforspaceinpool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007849_rage_neteventmgr_prepareevent_rage_netgameevent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007855_rage_netplayer_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007949_rage_netgameevent_netgameevent_unsigned_short_bool_bool_rage_net(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_007959_rage_netplayer_isremote_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008007_rage_atstring_atstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_008059_char_ptr_rage_formatf_char_5ul_char_ref_5ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008122_rage_netplayer_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008246_rage_netplayer_getrlgamerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008254_rage_netplayer_addkickvote_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008255_rage_netplayer_removekickvote_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008336_netscgamerconfigparser_getleaderboard_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008338_rage_netleaderboardread_getrow_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008365_rage_netleaderboardreadmgr_addreadbyrank_int_unsigned_int_rage_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_008372_rage_netleaderboardreadmgr_delete_rage_netleaderboardread_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008386_netscgamerconfigparser_getleaderboard_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_008406_rage_netscgamerdatamgr_isrockstardev_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_008412_netprofanityfilter_verifystringforprofanity_char_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_008413_netprofanityfilter_getstatusforrequest_int_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008442_rage_ccommerceconsumablemanager_setownershipdatadirty(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008449_rage_ccommercedata_hasmissingsubscriptionupsells_rage_ccommercep(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008452_rage_atmapentry_rage_atstring_int_const_ref_rage_atmap_rage_atst(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008460_rage_ccommercecategorydata_getitemindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008474_rage_ccommerceutil_dodatarequest_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008689_rage_fwscriptguid_createguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008693_rage_fwscriptguid_getbasefromguid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_008768_rage_scripthandlermgr_removescriptresource_unsigned_short_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_008866_rage_atmapentry_unsigned_int_int_const_ref_rage_atmap_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008917_rage_fwtimer_enduserpause(ph: *mut Phantom); }
extern "C" { pub fn f_008918_rage_fwtimer_startuserpause_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008925_rage_fwtimer_startscriptpause(ph: *mut Phantom); }
extern "C" { pub fn f_008926_rage_fwtimer_endscriptpause(ph: *mut Phantom); }
extern "C" { pub fn f_008950_rage_fwbasepool_getjustindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008952_rage_fwbasepool_getat_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008953_rage_fwknownrefholder_fwknownrefholder_void_ptr_ptr_rage_fwknown(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_009048_rage_crproperty_getattribute_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009223_rage_crskeleton_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009224_rage_crskeleton_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009294_rage_grmshader_unbind_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009295_rage_grmshader_enddraw_const(ph: *mut Phantom); }
extern "C" { pub fn f_009314_pixend(ph: *mut Phantom); }
extern "C" { pub fn f_009417_pixbegin_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009429_rage_pftimebarmgr_frameinit_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009511_rage_grceffect_dtorgrceffect(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009514_rage_grctexture_setcustomloadname_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009518_rage_grcimage_create_unsigned_int_unsigned_int_unsigned_int_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_009521_rage_grctexturefactory_registertexturereference_char_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009527_rage_grctexturefactory_deletetexturereference_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009529_rage_grcstateblock_createsamplerstate_rage_grcsamplerstatedesc_c(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009533_rage_grctexturefactory_createparams_createparams_unsigned_short(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009542_rage_grcstateblock_createblendstate_rage_grcblendstatedesc_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009544_rage_grcdrawsinglequadf_float_float_float_float_float_float_floa(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: i64); }
extern "C" { pub fn f_009545_rage_grcstateblock_setblendstate_rage_grcstateblock_grcblendstat(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009546_rage_grceffect_bind_rage_grceffecttechnique_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009548_rage_grcdevice_drawvertices_void_const_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_009555_rage_grcstateblock_setrasterizerstate_rage_grcstateblock_grcrast(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009559_rage_grcdevice_clear_bool_rage_color32_bool_float_bool_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_009560_rage_grcstateblock_setdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009561_rage_grcbegin_rage_grcdrawmode_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009562_rage_grcvertex_float_float_float_float_float_float_rage_color32(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: f32, a8: f32); }
extern "C" { pub fn f_009563_rage_grcend(ph: *mut Phantom); }
extern "C" { pub fn f_009587_rage_grclightstate_setenabled_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_009588_rage_grcimage_dtorgrcimage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009609_rage_grcviewport_resetwindow(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009612_rage_grcviewport_setwindow_int_int_int_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: f32, a6: f32); }
extern "C" { pub fn f_009613_rage_grcviewport_dtorgrcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009614_rage_grcviewport_setcurrent_rage_grcviewport_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009619_rage_grcviewport_ortho_float_float_float_float_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_009620_rage_grcviewport_setprojection_rage_mat44v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009623_rage_grcviewport_setnearfarclip_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_009624_rage_grcviewport_setworldmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009633_rage_grcviewport_getwindow_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009636_rage_grcviewport_setcameramtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009640_rage_grcviewport_isspherevisible_float_float_float_float_float_p(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_009646_rage_grcviewport_transform_rage_vec3v_const_ref_float_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009649_rage_grcworldidentity(ph: *mut Phantom); }
extern "C" { pub fn f_009650_rage_grcbindtexture_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009653_rage_grcviewport_op_rage_grcviewport_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009655_rage_grcfont_getstringwidthinternal_char_const_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009673_rage_grcdevice_initadapterordinal(ph: *mut Phantom); }
extern "C" { pub fn f_009691_rage_grcstateblock_default(ph: *mut Phantom); }
extern "C" { pub fn f_009719_rage_grcdevice_destroyvertexdeclaration_rage_grcvertexdeclaratio(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009723_rage_grcdevice_setvertexdeclaration_rage_grcvertexdeclaration_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009727_rage_grcdevice_beginvertices_rage_grcdrawmode_unsigned_int_unsig(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_009761_rage_grcdrawlabel_rage_vector3_const_ref_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009801_rage_grcimage_savergba8surfacetojpeg_char_const_ptr_int_void_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_009814_rage_grceffect_setvar_rage_grcinstancedata_ref_rage_grceffectvar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009819_rage_grceffect_getsamplerstate_rage_grcinstancedata_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009858_rage_grcstateblock_createrasterizerstate_rage_grcrasterizerstate(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009875_rage_grceffect_setlocalcommon_rage_grcinstancedata_ref_rage_grce(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_009885_rage_grceffect_setsamplerstate_rage_grcinstancedata_ref_rage_grc(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009886_rage_grceffect_pushsamplerstate_rage_grcinstancedata_ref_rage_gr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009887_rage_grceffect_popsamplerstate_rage_grcinstancedata_ref_rage_grc(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009892_rage_grceffect_technique_pass_unbind_bool_const(ph: *mut Phantom); }
extern "C" { pub fn f_009893_rage_grcstateblock_setdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009898_rage_grcstateblock_destroyblendstate_rage_grcstateblock_grcblend(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009904_rage_gridmonitor_getarea_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009905_rage_monitorconfiguration_ismultihead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009908_rage_monitorconfiguration_update(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009909_rage_grcadaptermanager_getadapter_unsigned_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_009910_rage_grcdevice_getadapterordinal(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_009912_rage_monitorconfiguration_getlandscapemonitor_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009913_rage_grcdevice_getmanufacturer_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_009925_rage_grcadapterd3d11output_getdesc_int_unsigned_int_rage_grcadap(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009926_rage_grcadapterd3d11output_getmode_rage_grcdisplaywindow_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009930_rage_grcadapterd3d11_getoutput_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009967_rage_grccbuffer_lockdirect_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009968_rage_grccbuffer_unlockdirect(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009986_rage_grcdebugdraw_adddebugoutputex_bool_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_009995_rage_grcdevice_disablescissor(ph: *mut Phantom); }
extern "C" { pub fn f_010003_rage_grcdevice_setscissor_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_010007_rage_grcdevice_clearrect_int_int_int_int_bool_rage_color32_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: f32, a7: i32, a8: i32); }
extern "C" { pub fn f_010012_rage_grcdevice_endcreatevertices_void_const_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_010020_rage_grcdevice_isready(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_010035_rage_grctexturedx11_mapstagingbuffertobackingstore_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_010040_rage_grcdevice_gethidef(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_010041_rage_grcdevice_setframelock_int_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010077_rage_grctexturedx11_updategpucopy_withthreadsyncing_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_010079_rage_grctexturedx11_copyfromgputostagingbuffer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010163_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010166_rage_grcdebugdraw_quad_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010170_rage_grcdebugdraw_rectaxisaligned_rage_vec2v_const_ref_rage_vec2(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010179_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010184_rage_grcdebugdraw_circle_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010189_rage_grcdebugdraw_boxoriented_rage_vec3v_const_ref_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010216_rage_grcdebugdraw_text_float_float_rage_dd_epixelcoordspace_rage(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i32, a4: i64, a5: i32, a6: f32, a7: f32); }
extern "C" { pub fn f_010219_rage_grcdebugdraw_adddebugoutputex_bool_rage_color32_char_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_010220_rage_grcdebugdraw_adddebugoutput_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010225_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010476_rage_iomouse_setmousetype_rage_iomouse_mousetype(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010500_rage_iovalue_isdown_float_rage_iovalue_readoptions_const_ref_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010503_rage_iovalue_isup_float_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010505_rage_iovalue_wasdown_float_rage_iovalue_readoptions_const_ref_co(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010508_rage_iovalue_wasup_float_rage_iovalue_readoptions_const_ref_cons(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010511_rage_iovalue_ispressed_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010512_rage_iovalue_historypressed_unsigned_int_unsigned_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_010514_rage_iovalue_isreleased_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010518_rage_iovalue_getnorm_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010519_rage_iovalue_getunboundnorm_rage_iovalue_readoptions_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010520_rage_iovalue_getlastnorm_rage_iovalue_readoptions_const_ref_cons(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010521_rage_iovalue_getnorm01_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010879_rage_bwmovie_create(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_010880_rage_bwmovie_destroy_rage_bwmovie_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010884_rage_bwmovie_init(ph: *mut Phantom); }
extern "C" { pub fn f_010886_rage_bwmovie_setmoviesync_rage_bwmovie_bwmovieparams_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_010888_rage_bwmovie_setvolume_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_010889_rage_bwmovie_updatemovietime_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010893_rage_bwmovie_play(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010895_rage_bwmovie_begindraw_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_010896_rage_bwmovie_shaderenddraw_rage_bwmovie_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011323_rage_atnamespacedhashstring_rage_athashstringnamespaces_0_op_cha(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012021_rage_fragphysicslodgroup_getlodbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012674_rage_snsession_getemptyslots_rage_rlslottype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_012815_rage_textchat_havetyper_rage_rlgamerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013374_rage_voicechat_setvpdthreshold_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013375_rage_voicechat_setvpdthresholdptt_unsigned_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013598_rage_netpeerid_op_rage_netpeerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013602_rage_netpeerid_op_rage_netpeerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013645_rage_netp2pcrypt_key_dtorkey(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013705_rage_nethttprequest_addformparam_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013706_rage_nethttprequest_addintformparam_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013710_rage_nethttprequest_beginlistformparam_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013712_rage_nethttprequest_addlistformparamintvalue_long(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013778_rage_netstatus_forcesucceeded_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013779_rage_netstatus_forcefailed_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014020_rage_nettask_cancel_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014067_rage_nettimesync_gettime_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014068_rage_nettimesync_hasstarted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014076_char_ptr_rage_formatf_char_4ul_char_ref_4ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014257_rage_is_ui_showing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014259_rage_rltaskmanager_doaddtask_unsigned_long_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014260_rage_rltaskmanager_destroytask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014284_rage_rlfriendsmanager_isfriendswith_int_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_014294_rage_rlgetposixtime_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014303_rage_rlgamerhandle_op_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014304_rage_rlros_isonline_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014315_rage_rlgamerinfo_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014316_rage_rlgamerinfo_getgamerhandle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014318_rage_rlgamerinfo_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014319_rage_rlgamerhandle_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014320_rage_rlpc_isinitialized(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014321_rage_rlgamerhandle_getrockstarid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014322_rage_rlgamerinfo_issignedin_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014344_rage_rlfriendspage_getfriend_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014345_rage_rlfriend_getgamerhandle_rage_rlgamerhandle_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014352_rage_rlfriend_isinsession_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014355_rage_rlfriendsmanager_isreading(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014356_rage_rlfriendsmanager_gettotalnumfriends_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014360_rage_rlfriendsmanager_canqueuefriendsrefresh_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014361_rage_rlros_getcredentials_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014362_rage_rlroscredentials_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014364_rage_rlfriendsmanager_requestrefreshfriendspage_int_rage_rlfrien(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_014368_rage_rlgamerhandle_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014369_rage_rlgamerhandle_tostring_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014370_rage_rlgamerhandle_tostring_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014374_rage_rlgamerhandle_export_void_ptr_unsigned_int_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014376_rage_rlgamerhandle_op_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014377_rage_rlgamerhandle_touniqueid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014378_rage_rlgamerid_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014379_rage_rlgamerid_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014383_rage_rlgamerinfo_hasstoreprivileges_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014384_rage_rlpresence_issignedin_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014385_rage_rlgamerinfo_getage_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014386_rage_rlgamerinfo_getagegroup_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014387_rage_rlgamerinfo_rlgamerinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014391_rage_rlgamerinfo_isonline_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014392_rage_rlgamerinfo_getgamerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014446_rage_rlroscredentials_getrockstarid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014459_rage_rltaskbase_configure_rage_netstatus_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014461_rage_rlclan_getprimaryclan_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014466_rage_rlroscredentials_getrockstaraccount_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014487_rage_rlsessioninfo_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014580_rage_rlsessioninfo_rlsessioninfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014606_rage_rlroshttptask_configure_int_rage_rlhttptaskconfig_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_014609_rage_rlstats_cancel_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014622_rage_rlsystemui_isuishowing_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014624_rage_rlsystemui_showwebbrowser_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014625_rage_rlsystemui_showfriendsearchui_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014662_rage_rlmetric_dtorrlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014685_rage_rlmetric_rlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014763_rage_rlpc_showui(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014791_rage_rlprofilestatsvalue_getfloat_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_014792_rage_rlprofilestatsvalue_getint32_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014793_rage_rlprofilestatsvalue_getint64_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014800_rage_rlprofilestatsreadresults_rlprofilestatsreadresults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014836_rage_rlroscredentials_hasprivilege_rage_rlrosprivilegeid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014859_rage_rlros_getdefaultprivilege_rage_rlrosprivilegeid(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014874_rage_rlroscredentials_getplayeraccountid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014943_rage_rlsctaskbase_rlsctaskbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014944_rage_rlscrequestresetpasswordtask_configure_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014945_rage_rlsocialclub_postuserfeedactivity_int_char_const_ptr_long_c(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014946_rage_rlsocialclub_registercomplaint_int_char_const_ptr_rage_nets(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_015035_char_ptr_rage_formatf_char_384ul_char_ref_384ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015086_rage_rlclan_adddelegate_rage_atdelegator1_void_rage_rlclanevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015087_rage_rlclan_removedelegate_rage_atdelegator1_void_rage_rlclaneve(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015088_rage_rlclan_isserviceready_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_015089_rage_rlclan_isfullserviceready_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_015090_rage_rlclan_hasprimaryclan_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_015092_rage_rlclan_getmine_int_rage_rlclanmembershipdata_ptr_unsigned_i(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_015094_rage_rlclan_leave_int_long_rage_netstatus_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_015095_rage_rlclan_setprimaryclan_int_long_rage_netstatus_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_015096_rage_rlclan_invite_int_long_int_bool_rage_rlgamerhandle_const_re(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_015098_rage_rlclan_join_int_long_rage_netstatus_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_015104_rage_rlclan_getopenclans_int_int_rage_rlclandesc_ptr_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_015128_rage_rlclandesc_rlclandesc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015130_rage_rlclandesc_isrockstarclan_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015133_rage_rlclanmember_rlclanmember(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015136_rage_rlclanmembershipdata_rlclanmembershipdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015137_rage_rlclanjoinrequestrecieved_rlclanjoinrequestrecieved(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015142_rage_rlclanrosbasetask_rlclanrosbasetask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015214_rage_rlsessionquerydata_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015215_rage_rlpresencequery_getsessionbygamerhandle_int_rage_rlgamerhan(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_015229_rage_rlscauth_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_016944_rage_scrthread_preprintstacktrace(ph: *mut Phantom); }
extern "C" { pub fn f_016949_rage_scrthread_getactivethread(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_016966_rage_atmap_void_ptr_nativecallloginfo_rage_atmaphashfn_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016981_rage_scrthread_getthread_rage_scrthreadid(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_017186_rage_crclip_composite_rage_crframe_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_018463_gmsgformat_parse_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018464_void_gmsgformat_formatd1_int_int_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018467_gcolor_blend_gcolor_gcolor_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_018468_g_getshortfilename_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018469_gimagebase_setpixelrgba_int_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_018470_gimagebase_getmipmaplevelsize_gimagebase_imageformat_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_018471_gimagebase_getmipmapleveldata_unsigned_int_unsigned_int_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_018472_garraydatabase_gcolor_gallocatorgh_gcolor_2_garraydefaultpolicy(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018473_gimage_gimage_gimagebase_imageformat_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_018476_gimage_createimage_gimagebase_imageformat_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_018477_grefcountntsimpl_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018478_gstring_op_gstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018479_gutf8util_decodenextchar_char_const_ptr_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_018480_gmatrix2d_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_018481_gmatrix2d_setidentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018482_gmatrix2d_prepend_gmatrix2d_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018483_gmatrix2d_append_gmatrix2d_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018484_gmatrix2d_transform_gpoint_float_ptr_gpoint_float_const_ref_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018485_gmatrix2d_transformbyinverse_gpoint_float_ptr_gpoint_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018486_gmatrix2d_setinverse_gmatrix2d_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018487_gmatrix2d_getmaxscale_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_018488_gmatrix2d_getxscale_const(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_018489_gmatrix2d_getyscale_const(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_018490_gmatrix2d_getrotation_const(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_018491_gmatrix2d_enclosetransform_grect_float_ptr_grect_float_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018492_gmatrix2d_setrecttoparl_float_float_float_float_float_const_ptr(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64); }
extern "C" { pub fn f_018493_gmatrix3d_matrixinverse_float_ptr_float_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018494_gmatrix3d_gmatrix3d_gmatrix2d_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018495_gmatrix3d_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_018496_gmatrix3d_setidentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018497_gmatrix3d_multiplymatrix_gmatrix3d_const_ref_gmatrix3d_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018498_gmatrix3d_enclosetransform_grect_float_ptr_grect_float_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_018511_glongformatter_convert(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018512_gutf8util_encodechar_char_ptr_long_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_018519_g_sprintf_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_018524_gmsgformat_gmsgformat_gmsgformat_sink_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018525_gmsgformat_dtorgmsgformat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018529_gmsgformat_nextformatter(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_018530_gmsgformat_bind_gformatter_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018531_gstring_assignstring_gstring_initstruct_const_ref_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018532_gstringbuffer_appendstring_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018534_gmsgformat_finishformatd(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018535_grefcountimpl_addref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018536_grefcountimpl_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018537_grefcountweaksupportimpl_dtorgrefcountweaksupportimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018539_grefcountweaksupportimpl_createweakproxy_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018540_gscreentoworld_getworldpoint_gpoint_float_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018541_g_wcscmp_wchar_t_const_ptr_wchar_t_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_018542_g_strtod_char_const_ptr_char_ptr_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f64; }
extern "C" { pub fn f_018543_g_towlower_wchar_t(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_018544_gstring_gstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018545_gstring_gstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018546_gstring_gstring_char_const_ptr_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_018548_gstring_gstring_gstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018549_gstring_op_wchar_t_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018550_gstring_getlength_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018551_gutf8util_getlength_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018553_gstring_appendstring_wchar_t_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018554_gstring_appendstring_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018555_gstring_assignstring_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018556_gstring_op_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018557_gstring_op_gstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018558_gstring_substring_unsigned_long_unsigned_long_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_018559_gstring_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018560_gstring_tolower_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018561_gstring_comparenocase_char_const_ptr_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_018562_gstring_bernsteinhashfunction_void_const_ptr_unsigned_long_unsig(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018563_gstring_bernsteinhashfunctioncis_void_const_ptr_unsigned_long_un(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018564_gstringbuffer_appendchar_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_018565_gstringlh_gstringlh(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018566_gstringlh_copyconstructhelper_gstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018567_gstringdh_gstringdh_gmemoryheap_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018569_gstringbuffer_gstringbuffer_gmemoryheap_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018570_gstringbuffer_dtorgstringbuffer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018571_gstringbuffer_insert_char_const_ptr_unsigned_long_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018576_gtimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_018577_gevent_wait_unsigned_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018579_gevent_dtorgevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018583_gevent_setevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018589_gutf8util_decodestring_wchar_t_ptr_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_018712_gcompoundshape_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018713_gcompoundshape_clearandrelease(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018714_gcompoundshape_getnumbytes_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018715_gcompoundshape_setcurvetolerance_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_018716_gcompoundshape_beginpath_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_018718_gcompoundshape_beginpath_int_int_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: f32, a5: f32); }
extern "C" { pub fn f_018719_garraypagedbase_gpointtype_8_64_gallocatorpagedlh_pod_gpointtype(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018720_gcompoundshape_closepath(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018721_gcompoundshape_addcurve_float_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_018723_gcompoundshape_perceivebounds_float_ptr_float_ptr_float_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_018724_gcompoundshape_expandpathbounds_gcompoundshape_spath_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_018725_gcompoundshape_pointinshape_float_float_bool_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_018726_gcompoundshape_removeshortsegments_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_018727_garraypagedbase_gedgeaa_vertextype_8_64_gallocatorpagedlh_pod_ge(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018728_garraypagedbase_gedgeaa_edgetype_10_64_gallocatorpagedlh_pod_ged(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018729_garraypagedbase_int_6_16_gallocatorpagedlh_pod_int_136_pushback(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018730_gedgeaa_calcintersectionpoint_float_float_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_018731_gedgeaa_assignstyle_unsigned_int_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_018738_gimageinfo_gimageinfo_gimage_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018739_gimageinfo_gimageinfo_gimage_ptr_unsigned_int_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_018740_gimageinfo_gimageinfo_gtexture_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_018753_grasterizer_grasterizer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018754_grasterizer_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018758_grasterizer_addshapescaled_gcompoundshape_const_ref_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_018759_grasterizer_sortcells(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_018760_grasterizer_sweepscanline_unsigned_int_unsigned_char_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_018761_garraypagedbase_grectpacker_nodetype_8_64_gallocatorpagedlh_pod(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018762_grectpacker_packrects_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_018763_garraypagedbase_grectpacker_recttype_8_64_gallocatorpagedlh_pod(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018764_garraypagedbase_grectpacker_packtype_4_16_gallocatorpagedlh_pod(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018765_grenderer_cxform_cxform(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018766_grenderer_cxform_concatenate_grenderer_cxform_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018767_grenderer_cxform_transform_gcolor_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018768_grenderer_dtorgrenderer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018769_grenderer_endframe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018773_gstrokepath_closepath_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_018774_garraypagedbase_gstrokevertextype_6_16_gallocatorpagedlh_pod_gst(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018780_gstroker_generateequidistant_gcompoundshape_spath_const_ref_gcom(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018781_gstroker_generateequidistant_gcompoundshape_const_ref_int_gcompo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_018789_gstrokeraa_tessellate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018802_gtessellator_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018803_gtessellator_monotonize_gcompoundshape_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018809_gtessellator_triangulatemonotone_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_018830_grefcountimplcore_dtorgrefcountimplcore_31596(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018921_rage_fwentity_getmodelid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018977_cgpsslot_clear_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_018978_cgpsslot_start_rage_vector3_ref_centity_ptr_int_bool_int_rage_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_018988_cminimap_isinbigmap(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_018996_cgps_forceupdateofgpsonminimap(ph: *mut Phantom); }
extern "C" { pub fn f_018997_cgps_shouldgpsbevisible_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_018999_cgps_stopradarblipgps_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019002_cgps_changeradarblipgpscolour_int_rage_color32(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_019158_char_ptr_rage_formatf_char_24ul_char_ref_24ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019450_char_ptr_rage_formatf_char_100ul_char_ref_100ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019565_cpedbonetagconvertor_getboneindexfrombonetag_rage_crskeletondata(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_021152_audnorthaudioengine_minimalupdate(ph: *mut Phantom); }
extern "C" { pub fn f_021175_audnorthaudioengine_activateslowmomode_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021199_audnorthaudioengine_deactivateslowmomode_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021222_audsuperconductor_sendconductormessage_conductors_conductormessa(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021351_cped_shouldbedead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021364_audspeechaudioentity_getisanyspeechplaying(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021571_audradiostation_getstation_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_021585_audradioaudioentity_startreplaytrackpreview_unsigned_int_unsigne(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_021593_audradiotrack_iscommercial_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021595_audradioaudioentity_stopreplaytrackpreview(ph: *mut Phantom); }
extern "C" { pub fn f_021597_audvehicleaudioentity_gethasnormalradio_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021598_audvehicleaudioentity_isradioenabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021599_audradioaudioentity_retunetostation_audradiostation_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021600_audradiostation_findstation_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021627_audradiostation_getstationindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021629_audradiostation_islocked_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021641_audradioaudioentity_retunetostation_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021642_audradioaudioentity_playstationselectsound_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021643_audradioaudioentity_playwheelhidesound(ph: *mut Phantom); }
extern "C" { pub fn f_021700_auduserradiotrackmanager_startmediascan_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021702_audradiostation_hasusertracks(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021739_auduserradiotrackmanager_gettracktitle_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_021741_auduserradiotrackmanager_gettrackartist_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_022023_rage_fwentity_getskeletondata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022235_cpedinventory_getweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022289_audinteractivemusicmanager_onreplayeditoractivate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022522_naenvironment_areweinaninterior_bool_ptr_float_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022615_audscene_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022621_auddynamicmixer_startscene_unsigned_int_audscene_ptr_ptr_audscri(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_022844_audfrontendaudioentity_startloadingtune(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022851_audfrontendaudioentity_notifyfrontendinput(ph: *mut Phantom); }
extern "C" { pub fn f_022852_audfrontendaudioentity_playsound_rage_atnonfinalhashstring_char(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022853_audfrontendaudioentity_playsoundmapzoom(ph: *mut Phantom); }
extern "C" { pub fn f_022854_audfrontendaudioentity_stopsoundmapzoom(ph: *mut Phantom); }
extern "C" { pub fn f_022855_audfrontendaudioentity_playsoundmapmovement_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_022856_audfrontendaudioentity_stopsoundmapmovement(ph: *mut Phantom); }
extern "C" { pub fn f_022857_audfrontendaudioentity_playsoundreplayscrubbing(ph: *mut Phantom); }
extern "C" { pub fn f_022858_audfrontendaudioentity_stopsoundreplayscrubbing(ph: *mut Phantom); }
extern "C" { pub fn f_023310_rage_atmapentry_unsigned_int_rage_atarray_naocclusionperportalse(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023686_fidevicereplay_refreshsizesandgettotalsize(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023700_replayfilemanager_getvideoprojectdirectory_char_ref_256_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023701_rage_atarray_cfiledata_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023702_replayfilemanager_getclipdirectory_char_ref_256_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023713_fidevicereplay_markfile_char_const_ptr_emarkeduptype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_023724_replayfilemanager_checkavailablediskspaceformontage_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_023725_replayfilemanager_getclipdirectoryforuser_unsigned_long_char_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023730_replayfilemanager_startdeleteclip_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023731_replayfilemanager_checkdeleteclip_bool_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023732_replayfilemanager_startloadmontage_char_const_ptr_cmontage_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_023733_replayfilemanager_isclipfilevalid_unsigned_long_char_const_ptr_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_023734_replayfilemanager_getclipuid_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023737_replayfilemanager_checkenumerateclipfiles_bool_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023739_replayfilemanager_isloading_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023743_replayfilemanager_ismarkedasdelete_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023827_creplaymgrinternal_freememory(ph: *mut Phantom); }
extern "C" { pub fn f_023865_creplaymgr_jumptofloat_float_unsigned_int(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_023866_creplaymgrinternal_unfreezeaudio(ph: *mut Phantom); }
extern "C" { pub fn f_023868_creplaymgrinternal_setnextplaybackstate_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_023904_creplaymgrinternal_setcursorspeed_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_023935_void_creplayinterfacemanager_getentity_centity_centityget_centit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023948_replayreticuleextension_setreadytofire_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023949_replayreticuleextension_sethit_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023950_replayreticuleextension_setisfirstperson_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_024257_creplaymgrinternal_isusingrecordedcamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024258_replayheader_generatedatestring_rage_atstring_ref_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024263_creplaymgrinternal_updateavailablediskspace(ph: *mut Phantom); }
extern "C" { pub fn f_024266_creplaymgrinternal_isawareofworldforrecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024268_creplaymgrinternal_getnextaudiobeattime_float_const_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_024269_creplaymgrinternal_getprevaudiobeattime_float_const_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_024281_creplaymgrinternal_oneditoractivate(ph: *mut Phantom); }
extern "C" { pub fn f_024288_creplaymgrinternal_startenumerateclipfiles_char_const_ptr_rage_a(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024289_creplaymgrinternal_checkupdatefavourites_bool_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024290_creplaymgrinternal_ismarkedasfavourite_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024291_creplaymgrinternal_checkmultidelete_bool_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024293_creplaymgrinternal_isclipusedinanymontage_clipuid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024294_creplaymgrinternal_beginplayback_ireplayplaybackcontroller_ref(ph: *mut Phantom); }
extern "C" { pub fn f_024296_creplaymgrinternal_cleanupplayback(ph: *mut Phantom); }
extern "C" { pub fn f_024299_creplaymgrinternal_gettotalmsinbuffer(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024314_creplaymarkercontext_reset(ph: *mut Phantom); }
extern "C" { pub fn f_024324_replayrecordingscriptinterface_preventrecordingthisframe(ph: *mut Phantom); }
extern "C" { pub fn f_026167_caminterface_isdominantrenderedcamera_cambasecamera_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026170_caminterface_isdominantrenderedcameraanyfirstpersoncamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026174_camgameplaydirector_getfirstpersonshootercamera_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026178_caminterface_computeminimapheading(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_026179_camcinematicdirector_isrenderingcinematicmountedcamera_bool_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026190_caminterface_fadeout_int_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026194_caminterface_isfadedout(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026195_caminterface_fadein_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026200_caminterface_isfading(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026205_cqueriableinterface_istaskcurrentlyrunning_int_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_026240_camcinematicdirector_shoulddisplayreticule_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026242_camcinematicdirector_isrenderingcinematicpointofviewcamera_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026279_rage_matrix34_toeulersyxz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026301_camdampedspring_update_float_float_float_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_026325_cambasecamera_isinterpolating_cambasecamera_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026329_worldprobe_cshapetestresults_dtorcshapetestresults(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026681_camgameplaydirector_computeweaponinfoforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026702_ccontrol_debugoutputscriptcallstack(ph: *mut Phantom); }
extern "C" { pub fn f_026703_camgameplaydirector_getfirstpersonvehiclecamera_centity_const_pt(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026706_camgameplaydirector_iscamerainterpolating_cambasecamera_const_pt(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026714_camgameplaydirector_getfirstpersonpedaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_027179_camreplaydirector_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_028262_cviewportmanager_pushviewport_cviewport_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_028263_cviewportmanager_popviewport(ph: *mut Phantom); }
extern "C" { pub fn f_028268_cviewportmanager_isusersmonitorwidescreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_029113_cplayercardxmldatamanager_miscdata_miscdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029335_cgamesessionstatemachine_reinit_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_029337_cgamesessionstatemachine_loadsavegame(ph: *mut Phantom); }
extern "C" { pub fn f_029433_getorcreategroup_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029455_cdebugscene_tunewidgetdata_tunewidgetdata_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_029456_cdebugscene_queuetunewidget_cdebugscene_tunewidgetdata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029612_worldprobe_cshapetestdesc_setresultsstructure_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029613_worldprobe_cshapetestresults_freeintersections(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029676_cutscenemanager_startmultiheadfade_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_029777_char_ptr_rage_safecat_char_char_4_char_ptr_char_const_ref_4_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029828_csavedbugstarrequestlist_loadbugstarrequest_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029829_csavedbugstarrequestlist_get_char_const_ptr_char_const_ptr_int_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_029866_cdebugscene_focusentities_get_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029880_cdebug_updatedebugpadmoviepositions(ph: *mut Phantom); }
extern "C" { pub fn f_029963_rage_atarray_rage_datcallback_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029986_cuigadgetlist_cuigadgetlist_float_float_float_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i64) -> i64; }
extern "C" { pub fn f_029987_cuigadgetwindow_cuigadgetwindow_float_float_float_float_cuicolor(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64) -> i64; }
extern "C" { pub fn f_029988_cuigadgetlist_setnumentries_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030051_cuigadgetbase_setpos_rage_vector2_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030052_cuigadgetbase_attachchild_cuigadgetbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030146_tiledscreencapture_isenabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_030418_cuigadgetbase_cuigadgetbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030420_cuigadgetbase_setchildrenactive_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030421_cuigadgetbase_detachfromparent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030422_cuigadgetbase_detachallchildren(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030423_cuigadgetbox_cuigadgetbox_float_float_float_float_rage_color32_c(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64) -> i64; }
extern "C" { pub fn f_030426_cuigadgetbox_getbounds_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030427_cuigadgetbutton_cuigadgetbutton_float_float_float_float_cuicolor(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64) -> i64; }
extern "C" { pub fn f_030428_cuigadgetbutton_setlabel_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030431_cuigadgetbutton_getbounds_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030443_cuigadgetlist_dtorcuigadgetlist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030446_cuigadgetlist_getbounds_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030449_cuigadgettext_cuigadgettext_float_float_rage_color32_const_ref_c(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_030452_cuigadgettext_getbounds_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030600_cuigadgetbutton_dtorcuigadgetbutton(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030720_rage_atpagedpooliterator_clightentity_rage_fwdefaultrscmempagedp(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030721_rage_atpagedpoolsubtypeiterator_clightentity_rage_fwdefaultrscme(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030746_geteventscriptnetworkgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_031650_ceventexplosionheard_getsourcepos_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032148_gasobject_gasobject_gasenvironment_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032149_gasprototypebase_setconstructor_gasobject_ptr_gasstringcontext_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_032154_gasobject_getmember_gasenvironment_ptr_gasstring_const_ref_gasva(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032157_gasobject_setmember_gasenvironment_ptr_gasstring_const_ref_gasva(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032159_gasstringhashbase_gasmember_ghashuncachedlh_gc_gasstring_gasmemb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032160_gasobject_invokewatchpoint_gasenvironment_ptr_gasstring_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032166_gasarrayobject_gasarrayobject_gasenvironment_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032167_gasobject_gasobject_gasstringcontext_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032168_gasarrayobject_dtorgasarrayobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032169_gasobject_dtorgasobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032170_gasarrayobject_resize_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_032171_garraydatabase_gasvalue_ptr_gallocatorlh_gasvalue_ptr_2_garrayde(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032176_void_gasobject_foreachchild_gc_grefcountbasegc_323_releasefuncto(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032177_void_gasobject_foreachchild_gc_grefcountbasegc_323_markincyclefu(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032178_void_gasobject_foreachchild_gc_grefcountbasegc_323_scaninusefunc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032180_gasobject_finalize_gc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032183_gasarrayobject_pushback_gasvalue_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032185_gasarrayobject_setelement_int_gasvalue_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_032194_garraydatabase_gasindexvalue_gallocatorgh_gasindexvalue_2_garray(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032204_gasobject_getmemberraw_gasstringcontext_ptr_gasstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032207_gasobject_deletemember_gasstringcontext_ptr_gasstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_032214_gasarrayobject_initarray_gasfncall_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032218_gasprototypebase_initfunctionmembers_gasobject_ptr_gasstringcont(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_032223_gasasbroadcaster_addlistener_gasenvironment_ptr_gasobjectinterfa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032225_gasasbroadcaster_broadcastmessagewithcallback_gasenvironment_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_032226_gasasbroadcaster_broadcastmessage_gasenvironment_ptr_gasobjectin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_032229_gasasbroadcaster_initializeinstance_gasstringcontext_ptr_gasobje(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032247_gasbitmapdata_commoninit_gasenvironment_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032248_gasbitmapdata_setimage_gasenvironment_ptr_gfximageresource_ptr_g(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_032293_gasobject_setmemberraw_gasstringcontext_ptr_gasstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_032308_gfxampviewstats_pushcallstack_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_032309_gfxampviewstats_popcallstack_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032330_gasmath_getnextrandom_gfxmovieroot_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_032351_gasmatrixobject_getmatrix_gasenvironment_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032411_gasobjectinterface_gasobjectinterface(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032412_gasobjectinterface_dtorgasobjectinterface(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032413_gasobjectinterface_toascharacter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032414_gasobjectinterface_toasobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032415_gasobjectinterface_tosprite(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032417_gasobjectinterface_findowner_gasstringcontext_ptr_gasstring_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_032418_gasobject_gasobject_gasrefcountcollector_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032419_gasobject_gasobject_gasstringcontext_ptr_gasobject_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_032441_gasprototypebase_dtorgasprototypebase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032443_gasprototypebase_init_gasobject_ptr_gasstringcontext_ptr_gasfunc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_032444_gasprototypebase_addinterface_gasstringcontext_ptr_int_gasfuncti(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_032445_garraydatabase_gptr_gasobject_gallocatorlh_gptr_gasobject_2_garr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032446_gasprototypebase_doesimplement_gasenvironment_ptr_gasobject_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_032447_gasprototypebase_getmemberrawconstructor_gasobject_ptr_gasstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_032479_gasrectangleobject_gasrectangleobject_gasenvironment_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032481_gasrectangleobject_getproperties_gasenvironment_ptr_grect_double(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032484_gasrectangleobject_setproperties_gasenvironment_ptr_grect_double(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032543_gasstringobject_getmemberraw_gasstringcontext_ptr_gasstring_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032553_gasstringproto_stringsubstring_gasstring_const_ref_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_032555_gasstringproto_stringsplit_gasenvironment_ptr_gasstring_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_032574_gfxtextstylemanager_clearstyles(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032578_gfxtextstylemanager_getstyle_gfxtextstylekey_keytype_char_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032589_gastextformatobject_gastextformatobject_gasenvironment_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032591_gfxtextformat_setfontlist_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032592_gfxtextformat_setitalic_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_032593_gfxtextformat_setbold_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_032594_gfxtextformat_setunderline_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_032596_gastextformatobject_settextformat_gasstringcontext_ptr_gfxtextfo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032597_gastextformatobject_setparagraphformat_gasstringcontext_ptr_gfxt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032598_gfxtextparagraphformat_op_gfxtextparagraphformat_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032600_gfxtextdocview_gfxtextdocview_gfxtextallocator_ptr_gfxfontmanage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_032601_gfxtextdocview_setviewrect_grect_float_const_ref_gfxtextdocview(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032602_gfxtextformat_initbydefaultvalues(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032603_gfxtextformat_merge_gfxtextformat_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032604_gfxtextparagraphformat_merge_gfxtextparagraphformat_const_ref_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032605_gfxstyledtext_setdefaulttextformat_gfxtextformat_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032606_gfxstyledtext_setdefaultparagraphformat_gfxtextparagraphformat_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032608_gfxstyledtext_appendstring_char_const_ptr_unsigned_long_gfxstyle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_032609_gfxtextdocview_format(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032610_gfxtextdocview_setautosizex(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032611_gfxtextdocview_setautosizey(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032612_gfxtextdocview_setwordwrap(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032613_gfxtextparagraphformat_initbydefaultvalues(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032614_gfxtextformat_setfontlist_gstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032615_gfxtextdocview_settext_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032616_gfxtextlinebuffer_removelines_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_032617_gfxtextdocview_getmaxvscroll(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_032618_gfxtextdocview_gettextwidth(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_032619_gfxtextdocview_gettextheight(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_032627_gastransformobject_gastransformobject_gasenvironment_ptr_gfxasch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_032628_void_gastransformobject_foreachchild_gc_grefcountbasegc_323_rele(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032629_void_gastransformobject_foreachchild_gc_grefcountbasegc_323_mark(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032630_void_gastransformobject_foreachchild_gc_grefcountbasegc_323_scan(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032644_gfxtextallocator_flushparagraphformatcache_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_032645_gfxtextallocator_flushtextformatcache_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_032648_grangedataarray_gptr_gfxtextformat_garraylh_grangedata_gptr_gfxt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032649_gfxtextparagraph_formatruniterator_op_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032670_gfxstyledtext_gettextandparagraphformat_gfxtextformat_ptr_gfxtex(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_032671_gfxstyledtext_gettextandparagraphformat_gfxtextformat_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032672_gfxstyledtext_setdefaulttextformat_gfxtextformat_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032674_void_gmsgformat_formatd1_gstringlh_gstringlh_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032675_gfxstyledtext_getlength_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032676_gfxstyledtext_copystyledtext_unsigned_long_unsigned_long_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_032678_gfxstyledtext_insertstring_wchar_t_const_ptr_unsigned_long_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_032679_gfxstyledtext_insertstyledtext_gfxstyledtext_const_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_032681_gfxstyledtext_remove_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032682_bool_gfxstyledtext_parsehtmlimpl_wchar_t_wchar_t_const_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_032688_gfxsgmlchariter_wchar_t_decodeescapedchar(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032690_gfxtextformat_setfontname_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032693_gfxtextparagraphformat_freetabstops(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032701_gfxtextdocview_imagesubstitutor_findimagedesc_wchar_t_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_032702_gfxtextdocview_settext_wchar_t_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032703_gfxtextdocview_settextformat_gfxtextformat_const_ref_unsigned_lo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_032704_gfxtextdocview_setparagraphformat_gfxtextparagraphformat_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_032708_gfxtextdocview_editcommand_gfxtextdocview_commandtype_void_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_032709_gfxtextdocview_gettext_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032710_gfxtextdocview_gethtml_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032711_gfxtextdocview_parsehtml_char_const_ptr_unsigned_long_bool_garra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_032712_gfxtextdocview_parsehtml_wchar_t_const_ptr_unsigned_long_bool_ga(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_032713_gfxtextdocview_replacetext_wchar_t_const_ptr_unsigned_long_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_032714_gfxtextdocview_sethscrolloffset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_032715_gfxtextdocview_setvscrolloffset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_032723_gfxtextlinebuffer_line_getformatdata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032725_gfxtextlinebuffer_glyphiterator_updatedesc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032727_gfxtexthighlighter_createhighlighter_gfxtexthighlightdesc_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032728_gfxtextdocview_display_gfxdisplaycontext_ref_gmatrix2d_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_032729_gfxtextlinebuffer_display_gfxdisplaycontext_ref_gmatrix2d_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_032730_gfxtexthighlighter_drawbackground_gfxdrawingcontext_ref_gfxtextl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32); }
extern "C" { pub fn f_032733_gfxtextdocview_gethscrolloffset(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_032734_gfxtextdocview_getvscrolloffset(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_032735_gfxtextdocview_getlinescount(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_032736_gfxtextdocview_clearwordwrap(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032737_gfxtextdocview_getviewrect(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032738_gfxtextdocview_getcharindexatpoint_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i64; }
extern "C" { pub fn f_032739_gfxtextlinebuffer_findlineatyoffset_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_032740_gfxtextdocview_getcursorposinline_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i64; }
extern "C" { pub fn f_032741_gfxtextdocview_getcursorposinlinebyoffset_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i64; }
extern "C" { pub fn f_032742_gfxtextdocview_getcursorposatpoint_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i64; }
extern "C" { pub fn f_032743_gfxtextdocview_isurlatpoint_float_float_grange_ptr(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_032744_gfxtextdocview_getlineindexofchar_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_032745_gfxtextlinebuffer_findlinebytextpos_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032746_gfxtextdocview_getlineoffset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_032747_gfxtextdocview_getlinelength_unsigned_int_bool_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_032748_gfxtextdocview_getcharboundaries_grect_float_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_032749_gfxtextdocview_getexactcharboundaries_grect_float_ptr_unsigned_l(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_032750_gfxtextdocview_getfirstcharinparagraph_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_032751_gfxtextdocview_getbottomvscroll(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_032754_gfxtextdocview_getselectionhighlighterdesc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032755_gfxtextdocview_setselection_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032756_gfxtextdocview_setdefaulttextandparaformat_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032759_gfxtexteditorkit_setcursorpos_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_032760_gfxtexteditorkit_resetblink_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_032761_gfxtexteditorkit_calccursorrectinlinebuffer_unsigned_long_grect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_032762_gfxtexteditorkit_calccursorrectonscreen_unsigned_long_grect_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_032763_gfxtexthighlighterpositerator_initcurdesc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032764_gfx_recalculaterecttofit16bit_gmatrix2d_ref_grect_float_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032765_gfxtexteditorkit_copytoclipboard_unsigned_long_unsigned_long_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_032766_gfxtexteditorkit_cuttoclipboard_unsigned_long_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_032767_gfxtexteditorkit_pastefromclipboard_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_032769_gfxtexteditorkit_invalidateselectioncolors(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032777_gfxampviewstats_getstats_gstatbag_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_032817_gastextformatobject_finalize_gc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032822_gastransformobject_dtorgastransformobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032827_void_gasprototype_gasampmarker_gasenvironment_foreachchild_gc_gr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032828_void_gasprototype_gasampmarker_gasenvironment_foreachchild_gc_gr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032858_gasprototype_gasbevelfilterobject_gasenvironment_getmemberraw_ga(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032868_void_gasprototype_gasbitmapdata_gasenvironment_foreachchild_gc_g(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032869_void_gasprototype_gasbitmapdata_gasenvironment_foreachchild_gc_g(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032873_gasprototype_gasbitmapdata_gasenvironment_getmemberraw_gasstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032882_void_gasprototype_gascolorobject_gasenvironment_foreachchild_gc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032883_void_gasprototype_gascolorobject_gasenvironment_foreachchild_gc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_032887_gasprototype_gascolorobject_gasenvironment_getmemberraw_gasstrin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032900_gasprototype_gascolortransformobject_gasenvironment_getmemberraw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032910_gasmoviecliploaderproto_dtorgasmoviecliploaderproto(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032925_gasprototype_gasstringobject_gasenvironment_getmemberraw_gasstri(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032931_gasstylesheetobject_finalize_gc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032938_gasprototype_gasstylesheetobject_gasenvironment_dtorgasprototype(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032940_gasprototype_gasstylesheetobject_gasenvironment_getmemberraw_gas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032950_gasprototype_gastextformatobject_gasenvironment_dtorgasprototype(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032952_gasprototype_gastextformatobject_gasenvironment_getmemberraw_gas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_032969_gfxampprofileframe_dtorgfxampprofileframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032990_gfxampfunctiontreestats_dtorgfxampfunctiontreestats(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_032992_ghashsetbase_ghashnode_unsigned_long_long_gptr_gfxampfunctiondes(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032997_ghashsetbase_gstringlh_hashnode_gasmoviecliploader_progressdesc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033000_ghashsetbase_ghashnode_gasstring_gasmember_gasstringhashfunctor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033001_ghashsetbase_ghashnode_gasstring_gasobject_watchpoint_gasstringh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033003_ghashsetnodeentry_gc_ghashnode_gasstring_gasobject_watchpoint_ga(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033007_ghashsetbase_ghashnode_gfxtextstylekey_gfxtextstyle_ptr_gfxtexts(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033010_garraydatabase_gfxtextstyle_ptr_gallocatorgh_gfxtextstyle_ptr_2(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033015_ghashsetbase_ghashnode_gfxtextformat_const_ptr_gptr_gfxfonthandl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033016_gasvalue_gasvalue_gasobject_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033017_gasvalue_droprefs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033018_gasfunctionobject_setprotoandctor_gasstringcontext_ptr_gasobject(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033019_grefcountbasegc_323_releaseinternal(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033020_gasfunctionref_dtorgasfunctionref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033023_gasvalue_toobject_gasenvironment_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033024_gasstringnode_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033025_gasglobalcontext_resolvefunctionname_gasstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033026_gasvalue_getpropertyvalue_gasenvironment_ptr_gasobjectinterface(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_033027_gascfunctionobject_gascfunctionobject_gasstringcontext_ptr_gasob(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_033028_gascfunctionobject_gascfunctionobject_gasstringcontext_ptr_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_033029_long_ghashsetbase_ghashnode_gasbuiltintype_gptr_gasobject_gfixed(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033030_gasglobalcontext_getprototype_gasbuiltintype(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_033032_gassuperobject_resetaltproto(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033033_gasfncall_dtorgasfncall(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033036_gasasfunctionobject_gasasfunctionobject_gasenvironment_ptr_gasac(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_033037_garraydatabase_gaswithstackentry_gallocatorlh_pod_gaswithstacken(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033038_gfxascharacter_createcharacterhandle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033040_gasenvironment_findtarget_gasstring_const_ref_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_033041_gfxcharacterhandle_resolvecharacter_gfxmovieroot_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033043_gasenvironment_createnewlocalframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033044_gasvalue_setasobject_gasobject_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033045_gasstringhashbase_gasvalue_ghashuncachedlh_gc_gasstring_gasvalue(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_033046_gasvalue_op_gasvalue_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033047_garraydatabase_gasvalue_gallocatorlh_gasvalue_2_garraydefaultpol(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033048_gfxlogbase_gasenvironment_logerror_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033049_gasstringmanager_createstringnode_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_033050_gassuperobject_gassuperobject_gasobject_ptr_gasobjectinterface_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_033051_gasenvironment_getvariable_gasstring_const_ref_gasvalue_ptr_garr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_033052_gptr_gasobject_op_gasobject_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033054_gasfncall_arg_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_033055_gasenvironment_createstring_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033056_gasvalue_setascharacter_gfxascharacter_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033057_gasenvironment_findandgetvariableraw_gasenvironment_getvarparams(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033058_gasenvironment_getvariableraw_gasenvironment_getvarparams_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033059_gasenvironment_setlocal_gasstring_const_ref_gasvalue_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033060_gasenvironment_findlocal_gasstring_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033062_gasactionbuffer_execute_gasenvironment_ptr_int_int_gasvalue_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_033063_gasvalue_add_gasenvironment_ptr_gasvalue_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033064_gasvalue_tonumber_gasenvironment_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> f64; }
extern "C" { pub fn f_033065_gasvalue_isequal_gasenvironment_ptr_gasvalue_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_033066_gasvalue_compare_gasenvironment_ptr_gasvalue_const_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_033067_gasvalue_tobool_gasenvironment_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033068_gasvalue_tostringimpl_gasenvironment_ptr_int_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_033069_gaspagedstack_gasvalue_32_pop_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_033070_gasstringnode_resolvelowercase_impl(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033071_gasenvironment_setvariable_gasstring_const_ref_gasvalue_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_033072_gasvalue_converttostringversioned_gasenvironment_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033073_gasstringmanager_createstringnode_char_const_ptr_unsigned_long_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_033074_gasenvironment_findtargetbyvalue_gasvalue_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033075_gfxascharacter_clonedisplayobject_gasstring_const_ref_int_gasobj(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_033076_gfxsprite_removedisplayobject_int_gfxresourceid(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_033077_gas_globaltrace_gasfncall_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033078_garraydatabase_gasenvironment_trydescr_gallocatorlh_gasenvironme(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033079_gasenvironment_checkexceptions_gasactionbuffer_ptr_int_int_ptr_g(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_033080_gasenvironment_findownerofmember_gasstring_const_ref_gasvalue_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_033081_gasvalue_tofunction_gasenvironment_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033082_gasenvironment_logscripterror_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033083_gasenvironment_operatornew_gasfunctionref_const_ref_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_033084_gasvalue_gasvalue_gasvalue_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033085_gasenvironment_operatornew_gasobject_ptr_gasstring_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_033086_gasvalue_toascharacter_gasenvironment_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033087_gasenvironment_primitivetotempobject_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033088_gasenvironment_getmember_gasobjectinterface_ptr_gasstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_033089_gasfncall_gasfncall_gasvalue_ptr_gasvalue_const_ref_gasenvironme(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_033090_gasvalue_toobjectinterface_gasenvironment_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033091_gfxmovieroot_addloadqueueentry_char_const_ptr_char_const_ptr_gas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_033092_gfxlogbase_gfxcharacter_logwarning_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033093_gfxmovieroot_parselevelname_char_const_ptr_char_const_ptr_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_033094_gasactionbuffer_resolveframenumber_gasenvironment_ptr_gasvalue_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_033095_gasenvironment_findvariable_gasenvironment_getvarparams_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_033096_gasfunctionrefbase_assign_gasfunctionrefbase_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033097_gasfunctionrefbase_setlocalframe_gaslocalframe_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033098_void_gasfunctionrefbase_foreachchild_gc_grefcountbasegc_323_rele(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033099_garraypagedbase_grefcountbasegc_323_ptr_10_5_gallocatorpagedlh_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033100_void_gasfunctionrefbase_foreachchild_gc_grefcountbasegc_323_mark(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033101_void_gasfunctionrefbase_foreachchild_gc_grefcountbasegc_323_scan(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033106_void_gasenvironment_push_gasvalue_gasvalue_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033113_gasnumberutil_tostring_double_char_ptr_unsigned_long_int(ph: *mut Phantom, a0: f64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_033114_gasvalue_gasvalue_gfxascharacter_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033115_gasvalue_gasvalue_gasstringcontext_ptr_void_ptr_gasfncall_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_033116_gasvalue_gasvalue_gasfunctionref_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033117_gasvalue_dtorgasvalue(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033118_void_gasvalue_foreachchild_gc_grefcountbasegc_323_releasefunctor(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033119_void_gasvalue_foreachchild_gc_grefcountbasegc_323_markincyclefun(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033120_void_gasvalue_foreachchild_gc_grefcountbasegc_323_scaninusefunct(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033122_gasstringmanager_createconststringnode_char_const_ptr_unsigned_l(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_033123_gasvalue_getcharacternamepath_gasenvironment_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033124_gasenvironment_createconststring_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033125_gasvalue_tostringversioned_gasenvironment_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_033126_stringtonumber_double_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033127_gasvalue_toint32_gasenvironment_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033128_gasvalue_touint32_gasenvironment_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033129_gasenvironment_getmovieroot_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033132_gasvalue_toprimitive_gasenvironment_ptr_gasvalue_hint_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033133_gfxactionlogger_logscriptmessagevarg_gfxlogconstants_logmessaget(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_033135_gasenvironment_getlog_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033137_gasfncall_checkthisptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_033138_gasfncall_thisptrerror_char_const_ptr_char_const_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033139_void_gmsgformat_formatd1_char_const_ptr_char_const_ptr_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033140_gas_invoke_gasvalue_const_ref_gasvalue_ptr_gasobjectinterface_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_033141_gas_invokeparsed_char_const_ptr_gasvalue_ptr_gasobjectinterface(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_033142_gas_invokeparsed_gasvalue_const_ref_gasvalue_ptr_gasobjectinterf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_033143_void_gasenvironment_push_double_double_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033144_gasglobalcontext_escapewithmask_char_const_ptr_unsigned_long_gst(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033145_gasglobalcontext_addpackage_gasstringcontext_ptr_gasobject_ptr_g(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_033146_gfxlogbase_gasfncall_logscriptmessage_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033159_gasglobalcontext_init_gfxmovieroot_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033160_void_ghashsetbase_ghashnode_gasbuiltintype_gptr_gasobject_gfixed(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033161_gasnamefunction_addconstmembers_gasobjectinterface_ptr_gasstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_033164_void_ghashsetbase_ghashnode_gasstring_gasglobalcontext_classrege(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033168_gasglobalcontext_dtorgasglobalcontext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033169_ghashsetbase_ghashnode_gasstring_gasfunctionref_gasstringhashfun(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033171_gasglobalcontext_getgc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033172_gasglobalcontext_preclean_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_033174_gasglobalcontext_getactualprototype_gasenvironment_ptr_gasbuilti(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_033175_gasglobalcontext_findclassname_gasenvironment_ptr_gasobjectinter(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033176_ghashsetbase_ghashnode_gasstring_signed_char_gasstringhashfuncto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033177_gasstringhashbase_gasfunctionref_ghashuncachedlh_gasstring_gasfu(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033178_gasglobalcontext_findregisteredclass_gasstringcontext_ptr_gasstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_033180_gasactionbufferdata_read_gfxstream_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033181_garraydatabase_gasexecutetag_ptr_gallocatorgh_gasexecutetag_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033183_gfxstream_populatebuffer_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_033186_garraydatabase_gasstring_gallocatorlh_gasstring_323_garraydefaul(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033190_gfxmovieroot_dragstate_initcenterdelta_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_033191_gfxcharacter_getworldmatrix_gmatrix2d_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033192_gfxcharacter_getworldmatrix3d_gmatrix3d_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033193_garraydatabase_gasasfunctionobject_argspec_gallocatorlh_gasasfun(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033194_gasenvironment_getprototype_gasbuiltintype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_033195_gfxsprite_adddisplayobject_gfxcharposinfo_const_ref_gasstring_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i64, a8: i64) -> i64; }
extern "C" { pub fn f_033196_stringtokenizer_nexttoken_char_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033197_gfxloadqueueentry_pconstruct_gfxloadqueueentry_loadtype_gfxchara(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_033198_gfxmovieroot_addmovieloadqueueentry_gfxloadqueueentry_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033199_gfxmovieroot_addloadqueueentrymt_gfxloadqueueentry_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033200_gasenvironment_logscriptwarning_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033202_gasenvironment_getcollector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033203_gasenvironment_checkglobalandlevels_gasenvironment_getvarparams(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033204_gasenvironment_parsepath_gasstringcontext_ptr_gasstring_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_033205_gasenvironment_getconstructor_gasbuiltintype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033206_gfxeventid_getfunctionnamebuiltintype_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_033210_grefcountcollector_323_collect_grefcountcollector_323_stats_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033211_gasrefcountcollector_forceemergencycollect(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033212_void_ghashsetbase_gasstringnode_ptr_gasstringnodehashfunc_gasstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033216_gasstringmanager_createstringnode_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033217_gasstringnode_releasenode(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033218_gasstringmanager_dtorgasstringmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033219_gfxlogbase_gfxlog_logscripterror_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033221_gfx_generatemousebuttonevents_unsigned_int_gfxmousestate_ptr_uns(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_033222_gfxbuttonrecord_read_gfxloadprocess_ptr_gfxtagtype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_033223_gfxstream_readmatrix_gmatrix2d_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033224_garraydatabase_gfxfilterdesc_gallocatorgh_gfxfilterdesc_2_garray(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033225_gfxstream_readrgb_gcolor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033226_gfxbuttonaction_dtorgfxbuttonaction(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033227_gfxbuttonaction_read_gfxstream_ptr_gfxtagtype_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_033228_garraydatabase_gasactionbufferdata_ptr_gallocatorlh_gasactionbuf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033229_gfxbuttoncharacterdef_dtorgfxbuttoncharacterdef(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033231_garraybase_garraydata_gfxbuttonrecord_gallocatorlh_gfxbuttonreco(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033232_garraydatabase_gfxbuttonaction_gallocatorlh_gfxbuttonaction_258(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033234_garraydatabase_gptr_gfxcharacter_gallocatorlh_gptr_gfxcharacter(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033235_gfxbuttoncharacter_recreatecharacters(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033238_gfxcharacter_setperspectivefov_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_033239_gfxcharacter_updateviewandperspective(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033240_gfxcharacter_getlevelmatrix_gmatrix2d_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033241_gfxcharacter_getworldcxform_grenderer_cxform_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033242_gfxcharacter_getperspective3d_bool_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033243_gfxcharacter_getview3d_bool_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033249_gfxcharacter_createscale9grid_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_033251_gfxmovieroot_removetopmostlevelcharacter_gfxcharacter_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033252_gfxcharacterhandle_gfxcharacterhandle_gasstring_const_ref_gfxasc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_033253_gfxascharacter_getabsolutepath_gstring_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033254_gfxcharacterhandle_dtorgfxcharacterhandle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033255_gfxascharacter_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033256_gptr_gfxmoviedefimpl_dtorgptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033258_gfxascharacter_dtorgfxascharacter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033260_gfxascharacter_getfocusgroupmask(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_033263_gfxascharacter_oneventunload(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033264_gfxascharacter_removefromplaylist_gfxmovieroot_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033265_gfxmovieroot_setfocusto_gfxascharacter_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033268_gfxascharacter_cloneinternaldata_gfxascharacter_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033269_ghashsetbase_ghashnode_gfxeventid_garraylh_gasvalue_323_garrayde(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033271_gfxascharacter_setname_gasstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033273_void_gmsgformat_formatd1_unsigned_int_unsigned_int_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033274_gfxascharacter_domousedrag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033276_gfxascharacter_isstandardmember_gasstring_const_ref_gasstring_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033277_gfxascharacter_getstandardmemberconstant_gasstring_const_ref_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033278_gfxascharacter_getgeomdata_gfxascharacter_geomdatatype_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033279_gfxascharacter_ensuregeomdatacreated(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033281_gfxascharacter_setstandardmember_gfxascharacter_standardmember_g(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_033282_gfxlogbase_gfxcharacter_logscripterror_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033283_gmatrix2d_appendrotation_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_033284_gfxascharacter_getstandardmember_gfxascharacter_standardmember_g(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_033285_gfxmovieroot_gettopmostentity_gpoint_float_const_ref_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_033286_garraydatabase_char_gallocatorgh_char_2_garraydefaultpolicy_resi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033287_gfxmoviedefimpl_getcharactercreateinfo_gfxresourceid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033288_gfxlogbase_gfxcharacter_logerror_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033289_gfxdisplaylist_movedisplayobject_gfxcharposinfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033290_gfxsprite_getmovieclipobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033291_gfxmoviedefimpl_getnameofexportedresource_gfxresourceid_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033292_garraydatabase_gasvalue_gallocatorgh_gasvalue_2_garraydefaultpol(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033296_gfxascharacter_modifyoptimizedplaylist_gfxmovieroot_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033297_gfxdisplaylist_adddisplayobject_gfxcharposinfo_const_ref_gfxchar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_033298_gfxmovieroot_resolvestickyvariables_gfxascharacter_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033299_gfxmovieroot_doactionsforsession_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_033300_gfxsprite_haseventhandler_gfxeventid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033301_gfxsprite_sethitarea_gfxsprite_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033302_gfxascharacter_setsingleclipeventhandler_gfxeventid_const_ref_ga(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033303_garraydatabase_gasvalue_gallocatorlh_gasvalue_323_garraydefaultp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033304_gfxascharacter_executeevent_gfxeventid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033309_gfxascharacter_deletemember_gasstringcontext_ptr_gasstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_033313_gfxascharacter_setmember_gasenvironment_ptr_gasstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_033314_gfxvalue_updatetransform_gfxascharacter_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033315_garraydatabase_gfxcharacter_ptr_gallocatordh_gfxcharacter_ptr_2(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033316_garraydatabase_gptr_gfxascharacter_gallocatorlh_gptr_gfxascharac(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033318_gfxascharacter_hasmember_gasstringcontext_ptr_gasstring_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_033322_gfxascharacter_set_proto_gasstringcontext_ptr_gasobject_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033331_gfxascharacter_onfocus_gfxascharacter_focuseventtype_gfxascharac(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_033333_garraybase_garraydata_gfxfilterdesc_gallocatorgh_gfxfilterdesc_2(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033338_gfxdisplaycontext_init_gfxstatebag_const_ptr_gfxresourceweaklib(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i64); }
extern "C" { pub fn f_033339_gfxdisplaycontext_dtorgfxdisplaycontext(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033340_gfxdisplaycontext_predisplay_gfxdisplaycontexttransforms_ref_gfx(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_033341_gfxdisplaycontext_beginfilters_gfxdisplaycontextfilters_ref_gfxc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_033342_gfxdisplaycontext_endfilters_gfxdisplaycontextfilters_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_033343_gfxdisplaylist_getcharacterbyname_gasstringcontext_ptr_gasstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_033344_gfxdisplaylist_unloadentryatindex_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033345_gfxdisplaylist_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033346_gfxdisplaylist_display_gfxdisplaycontext_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033347_gfxdisplaylist_visitmembers_gasobjectinterface_membervisitor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033348_gfxdrawingcontext_gfxdrawingcontext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033350_garraydatabase_gfxlinestyle_gallocatorlh_gfxlinestyle_259_garray(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033351_gfxlinestyle_op_gfxlinestyle_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033352_gfxdrawingcontext_setnewfill(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033353_garraydatabase_gfxfillstyle_gallocatorlh_gfxfillstyle_259_garray(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033354_gfxfillstyle_op_gfxfillstyle_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033356_gfxfillstyle_setimagefill_gfxfilltype_gfximageresource_ptr_gmatr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_033358_gfxpathpacker_setmoveto_int_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_033360_gfxpathpacker_addlineto_int_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_033361_gfxpathpacker_addcurve_int_int_int_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_033362_gfxdrawingcontext_acquirepath_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_033363_gfxpathpacker_pack_gfxpathallocator_ptr_gfxpathdata_pathsinfo_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033365_gfxdrawingcontext_display_gfxdisplaycontext_ref_gmatrix2d_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_033366_gfxshapebase_display_gfxdisplayparams_ref_bool_gfxcharacter_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_033367_gfxmeshset_display_gfxdisplayparams_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033368_gfxmeshcache_getmeshsetandlock_gfxshapebase_ptr_void_const_ptr_g(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_033369_gfxvertexarray_gfxvertexarray_gmemoryheap_ptr_grenderer_vertexfo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033370_gfxmeshcache_addmeshset_gfxshapebase_ptr_gfxmeshset_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033372_gfxtextformat_dtorgfxtextformat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033373_gfxfonthandle_dtorgfxfonthandle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033374_gfxdrawtextimpl_processimagetags_gfxtextdocview_ptr_gfxdrawtextm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033375_gfxmoviedefimpl_getexportedresource_gfxresourcebinddata_ptr_gstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_033376_gfxshapewithstyles_settoimage_gfximageresource_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033377_gfxlogbase_gfxlog_logscriptwarning_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033378_gstringhashlh_gfxresourcehandle_2_gstring_nocasehashfunctor_gstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_033379_gfxresourcebinding_getresourcedata_locked_gfxresourcebinddata_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033380_garraydatabase_gptr_gfxmoviedefimpl_gallocatorgh_gptr_gfxmoviede(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033382_gfxlogbase_gfxlog_logwarning_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033386_gfxstatebagimpl_gfxstatebagimpl_gfxstatebagimpl_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033387_gfxtextallocator_gfxtextallocator_gmemoryheap_ptr_unsigned_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_033388_gfxfontmanager_commoninit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033389_gfxfontcachemanagerimpl_gfxfontcachemanagerimpl_gmemoryheap_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033390_gfxglyphrastercache_init_unsigned_int_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_033391_gfxresourceweaklib_gfxresourceweaklib_gfxresourcelib_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033392_gfxmeshcachemanager_gfxmeshcachemanager_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_033394_gfxdrawtextmanagerimpl_dtorgfxdrawtextmanagerimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033398_gfxdrawtextmanager_textparams_textparams(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033399_gfxdrawtextmanager_settextparams_gfxtextdocview_ptr_gfxdrawtextm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_033400_gfxtextformat_op_gfxtextformat_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033401_gfxdrawtextmanager_createtempdoc_gfxdrawtextmanager_textparams_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32) -> i64; }
extern "C" { pub fn f_033403_gfxdrawtextmanager_checkfontstateschange(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033404_gfxfontmanagerstates_checkstatechange_gfxfontlib_ptr_gfxfontmap(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_033406_gfxdrawtextmanager_getcharacterbounds_char_const_ptr_int_grect_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: i64); }
extern "C" { pub fn f_033407_gfxfontmanager_createfonthandlefromname_char_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_033413_gfxmeshcachemanager_enddisplay(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033414_gfxmeshcache_deletemeshset_gfxcachedmeshset_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033415_gfxrendergen_clearandrelease(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033416_gfxedgeaagenerator_processandsortedges_gtessellator_ref_gfxfills(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32, a6: i32); }
extern "C" { pub fn f_033417_gfxedgeaagenerator_generatesolidmesh_gfxvertexarray_ptr_gfxmesh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_033418_gfxedgeaagenerator_setvertexcolor_gedgeaa_vertextype_const_ref_g(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_033419_garraydatabase_unsigned_short_gallocatorlh_unsigned_short_2_garr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033420_garraydatabase_gfxmesh_gallocatorlh_gfxmesh_131_garraydefaultpol(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033421_gfxedgeaagenerator_vertexcollector_assignfactor_unsigned_int_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_033424_gfxlogbase_gfxlog_logerror_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033425_gfxtextfilter_loadfilterdesc_gfxfilterdesc_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033427_gfxfontcachemanagerimpl_dtorgfxfontcachemanagerimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033429_gfxfontcachemanagerimpl_invalidateall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033430_gfxglyphslotqueue_computeglypharea_gfxglyphnode_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033431_gfxfontresource_removedisposehandler_gfxfontresource_disposehand(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033432_gfxglyphslotqueue_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033433_gfxglyphslotqueue_findfontinslot_gfxglyphnode_ptr_gfxfontresourc(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033434_gfxglyphslotqueue_releaseslot_gfxglyphdynaslot_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033435_void_ghashsetbase_ghashnode_gfxfontcachemanagerimpl_vectorglyphk(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033437_void_ghashsetbase_gfxfontresource_ptr_gfxfonthashop_gfxfonthasho(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033442_gfxfontcachemanagerimpl_releasebatchpackage_gfxbatchpackage_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033443_gmatrix2d_isfreerotation_float_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_033444_gfximageresource_ptr_gfxresourcebinding_op_gfximageresource_gfxr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033447_gfxfontcompactor_dtorgfxfontcompactor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033449_gfxfontcompactor_normalizelastcontour(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033450_garraypagedbase_gfxfontcompactor_vertextype_6_64_gallocatorpaged(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033451_garraypagedbase_unsigned_char_12_256_gallocatorpagedlh_pod_unsig(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033452_gfxfontcompactor_navigatetoendglyph_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_033453_gfxpathdataencoder_garraypagedlh_pod_unsigned_char_12_256_261_wr(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_033454_void_ghashsetbase_gfxfontcompactor_contourkeytype_gfxfontcompact(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033455_void_ghashsetbase_gfxfontcompactor_glyphkeytype_gfxfontcompactor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033456_garraypagedbase_gfxfontcompactor_glyphinfotype_6_64_gallocatorpa(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033457_void_ghashsetbase_unsigned_short_gfixedsizehash_unsigned_short_g(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033458_gfxfontcompactor_assignglyphcode_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_033459_gfxfontglyphpacker_dtorgfxfontglyphpacker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033461_garraydatabase_gfxtextureglyph_gallocatorlh_gfxtextureglyph_2_ga(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033462_gfxfontglyphpacker_packglyphrects_garray_gfxfontglyphpacker_glyp(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_033463_gfxtextureglyphdata_addtextureglyph_unsigned_int_gfxtextureglyph(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_033464_void_ghashsetbase_ghashnode_gfxresourceid_gfxresourceptr_gfximag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033469_garraydatabase_gptr_gfxmoviedatadef_gallocatorgh_gptr_gfxmovieda(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033470_gfxresource_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033472_gfxloadstates_gfxloadstates_gfxloaderimpl_ptr_gfxstatebag_ptr_gf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_033473_gfxloaderimpl_createmovie_loadstate_gfxloadstates_ptr_gfxmovieda(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_033474_gfxmoviedefbindstates_gfxmoviedefbindstates_gfxstatebag_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033475_gfxurlbuilder_extractfilepath_gstring_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_033476_gfxloaderimpl_createmoviedefimpl_gfxloadstates_ptr_gfxmoviedatad(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_033477_gfxloaderimpl_bindmovieandwait_gfxmoviedefimpl_ptr_gfxmoviebindp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_033478_gfxfontlib_loadfontnames_gstringhash_gstring_gallocatorgh_gstrin(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033479_void_ghashsetbase_ghashnode_gstring_gstring_gstring_nocasehashfu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033480_gfxfonthandle_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033481_gfxfontmanagerstates_getstateaddref_gfxstate_statetype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_033483_gfxfontresource_gfxfontresource_gfxfont_ptr_gfxresourcebinding_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_033484_gfxfontmanager_dtorgfxfontmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033486_gfxfontmanager_findorcreatehandle_char_const_ptr_unsigned_int_gf(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_033487_gfxfontmap_getfontmapping_gfxfontmap_mapentry_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_033488_addsearchinfo_gfxfontmanager_fontsearchpathinfo_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_033489_void_ghashsetbase_gfxfontmanager_nodeptr_gfxfontmanager_nodeptrh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033490_gfxresourceweaklib_bindresourcekey_gfxresourcelib_bindhandle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_033491_gfxresourcelib_resourceslot_resolve_gfxresource_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033492_void_ghashsetbase_gfxresourceweaklib_resourcenode_gfxresourcewea(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033493_gfxfontmap_dtorgfxfontmap(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033496_gfxfontdata_dtorgfxfontdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033498_garraydatabase_gptr_gfxshapebase_gallocatorlh_gptr_gfxshapebase(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033499_gfxconstshapenostyles_read_gfxloadprocess_ptr_gfxtagtype_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_033500_gfxstream_readstringwithlength_gmemoryheap_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033501_garraydatabase_gfxfontdata_advanceentry_gallocatorlh_gfxfontdata(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033502_gfxfontdata_readcodetable_gfxstream_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033503_gfxstream_readrect_grect_float_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033504_gfxlogbase_gfxstream_logerror_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033505_void_ghashsetbase_ghashnode_unsigned_short_unsigned_short_gident(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033515_void_g_quicksort_garray_anonymous_namespace_range_2_garraydefaul(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033516_buildstringfromranges_garray_anonymous_namespace_range_2_garrayd(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033517_void_gmsgformat_formatd1_unsigned_short_unsigned_short_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033518_gfxfontdatacompactedswf_dtorgfxfontdatacompactedswf(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033521_gfxswfpathdata_pathsiterator_pathsiterator_gfxshapebase_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033522_gfxswfpathdata_pathsiterator_readnext(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033523_gfxswfpathdata_edgesiterator_getedge_gfxswfpathdata_edgesiterato(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033528_gfxglyphpathiterator_garraypagedlh_pod_unsigned_char_12_256_261(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033532_gfxglyphpathiterator_garraypagedlh_pod_unsigned_char_12_256_261(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_033544_gfxglyphpathiterator_garrayunsafelh_pod_unsigned_char_261_readpa(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033548_gfxglyphpathiterator_garrayunsafelh_pod_unsigned_char_261_readbo(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_033554_gfxfontresource_dtorgfxfontresource(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033557_long_ghashsetbase_gfxresourceweaklib_resourcenode_gfxresourcewea(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_033558_void_ghashsetbase_gfxresourceweaklib_resourcenode_gfxresourcewea(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033559_gfxshapebase_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033560_gfxglyphslotqueue_packglyph_unsigned_int_unsigned_int_gfxglyphno(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_033561_glistallocbase_gfxglyphnode_127_gallocatorlh_pod_gfxglyphnode_19(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033562_gfxglyphslotqueue_initnewslot_gfxglyphband_ptr_unsigned_int_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_033564_gfxglyphslotqueue_releaseglyphtree_gfxglyphnode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033565_void_ghashsetbase_ghashnode_gfxglyphparam_gfxglyphnode_ptr_gfxgl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033567_gfxresourcebinding_setbinddata_unsigned_int_gfxresourcebinddata(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_033573_gfxurlbuilder_defaultbuildurl_gstring_ptr_gfxurlbuilder_location(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033574_gfximagecreator_createimage_gfximagecreateinfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033575_gfximagecreator_loadbuiltinimage_gfile_ptr_gfxfileconstants_file(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_033576_gfxlogbase_gfxlog_logmessage_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033582_gfxmoviedatadeffilekeydata_gfxmoviedatadeffilekeydata_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_033583_gfxloaderimpl_createmovie_loadstate_gfxloadstates_ptr_gfxurlbuil(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_033584_gfxresourceweaklib_unpinall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033585_gfxloaderimpl_cancelloading(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033586_gfxlog_logmessagebytype_gfxlogconstants_logmessagetype_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033587_gfxloadertask_dtorgfxloadertask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033588_gfxtextkeymap_addkeyentry_gfxtextkeymap_keymapentry_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033595_s_openanddetectfile_char_const_ptr_unsigned_int_gfxloadstates_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_033596_gfxmoviedatadef_gfxmoviedatadef_gfxresourcekey_const_ref_gfxmovi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i64) -> i64; }
extern "C" { pub fn f_033597_gfxmoviedefimpl_gfxmoviedefimpl_gfxmoviedatadef_ptr_gfxmoviedefb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i32, a8: i64) -> i64; }
extern "C" { pub fn f_033598_gfxloaderimpl_loadmovieimage_char_const_ptr_gfximageloader_ptr_g(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_033599_gfxwstringbuffer_resize_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033600_gfxmoviedefbindstates_gfxmoviedefbindstates_gfxmoviedefbindstate(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033601_gfxloadstates_dtorgfxloadstates(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033603_gfxloadprocess_dtorgfxloadprocess(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033607_gfxloadprocess_tagarraytoframe_garray_gasexecutetag_ptr_2_garray(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033608_garraybase_garraydata_gasexecutetag_ptr_gallocatorgh_gasexecutet(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033609_gfxloadprocess_commitframetags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033610_gfxloadprocess_adddataresource_gfxresourceid_gfxresourcedata_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033611_void_ghashsetbase_ghashnode_gfxresourceid_gfxresourcehandle_gfxr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033612_gfxloadprocess_addimageresource_gfxresourceid_gimage_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033613_gfxmoviedatadef_loadtaskdata_addresource_gfxresourceid_gfxresour(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033617_gfxmoviedatadef_loadtaskdata_getresourcehandle_gfxresourcehandle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_033620_gfxmesh_display_gfxdisplaycontext_const_ref_gfxfillstyle_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: f32, a7: i64, a8: i32); }
extern "C" { pub fn f_033621_gfxfillstyle_getgradientfilltexture_grenderer_filltexture_ptr_gf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64) -> i32; }
extern "C" { pub fn f_033622_gfxfillstyle_getimagefilltexture_grenderer_filltexture_ptr_gfxdi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64) -> i32; }
extern "C" { pub fn f_033623_gfxfillstyle_getfilltexture_grenderer_filltexture_ptr_gfxdisplay(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033624_gfxcachedstroke_dtorgfxcachedstroke(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033625_gfxcachedstroke_addshapepathvertices_gcompoundshape_const_ref_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_033626_gfxvertexarray_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033627_gfxvertexarray_getformatdesc_grenderer_vertexformat(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_033628_gfxvertexarray_setformat_grenderer_vertexformat(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_033629_garraydatabase_gfxfillstyle_gallocatorgh_gfxfillstyle_259_garray(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033630_gfxmeshset_dtorgfxmeshset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033632_gfxmeshset_addtessellatedshape_gcompoundshape_const_ref_gfxfills(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_033633_garraydatabase_gfxmeshset_meshsubshape_gallocatorlh_gfxmeshset_m(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033634_gfxmeshset_setimgadjustmatrices_garray_gfxscale9gridinfo_imgadju(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033635_garraydatabase_gmatrix2d_gallocatorlh_gmatrix2d_2_garraydefaultp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033636_gfxmeshcachemanager_dtorgfxmeshcachemanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033638_gfxmeshcachemanager_setrendergenmemlimit_unsigned_long(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033639_gfxmeshcachemanager_getmeshcachememlimit_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033640_gfxmeshcachemanager_setmeshcachememlimit_unsigned_long(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033641_gfxmeshcache_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033642_gfxmeshcachemanager_getnumbytes_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033643_gfxmeshcache_dtorgfxmeshcache(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033647_gfxmorphcharacterdef_dtorgfxmorphcharacterdef(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033648_gfxshapewithstyles_dtorgfxshapewithstyles(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033651_gfxfillstyle_setlerp_gfxfillstyle_const_ref_gfxfillstyle_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_033652_gfxswfpathdata_edgesiterator_getedgescount_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_033654_gfxscale9gridinfo_compute(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033655_gfxscale9gridinfo_adjustbounds_grect_float_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033656_gfxshapebase_addshapetomesh_gfxmeshset_ptr_gcompoundshape_ptr_gf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_033657_gfxscale9gridinfo_computeimgadjustmatrices(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033659_gfxshapebase_applyscale9grid_gcompoundshape_ptr_gfxscale9gridinf(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033661_void_gfxshapebase_computeboundimpl_gfxpathdata_grect_float_ptr_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033665_gfxmorphcharacterdef_readmorphfillstyle_gfxloadprocess_ptr_gfxta(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_033666_gfxgradientrecord_read_gfxloadprocess_ptr_gfxtagtype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033667_gfxlinestyle_setcomplexfill_gfxfillstyle_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033668_gfxswfpathdata_pathsiterator_getedgescount_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_033669_gfxswfpathdata_pathsiterator_skipcomplex(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033670_gfxmoviedatadef_loadtaskdata_loadtaskdata_gfxmoviedatadef_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_033675_gfxmoviedatadef_translateframestring_gfxstringhashlh_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_033676_garraydatabase_gfxtimelinedef_frame_gallocatorlh_gfxtimelinedef(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033677_gfxmoviedatadef_loadtaskdata_dtorloadtaskdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033679_gfxmoviedatadef_loadtaskdata_initimagefilemoviedef_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_033680_gfxstream_opentag_gfxtaginfo_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033681_gfxlogbase_gfxstream_logwarning_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033682_gfxmoviedatadef_loadtaskdata_finishloadingframe_gfxloadprocess_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_033683_gfxmoviebindprocess_bindnextframe(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_033685_void_ghashsetbase_gstringlh_hashnode_unsigned_int_gstring_nocase(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033690_gfxmoviedefimpl_bindtaskdata_bindtaskdata_gmemoryheap_ptr_gfxmov(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_033692_gfxmoviedefimpl_bindtaskdata_onmoviedefrelease(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033694_gfxmoviedefimpl_dtorgfxmoviedefimpl_58225(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033696_gfxmoviebindprocess_dtorgfxmoviebindprocess(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033698_garraydatabase_gptr_gfxmoviedefimpl_gallocatorlh_gptr_gfxmoviede(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033702_gfxsprite_gfxsprite_gfxtimelinedef_ptr_gfxmoviedefimpl_ptr_gfxmo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_033703_gfxsprite_setlevel_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_033704_gfxmovieroot_setlevelmovie_int_gfxsprite_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_033705_gfxmoviedefimpl_bindtaskdata_dtorbindtaskdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033712_gfxsprite_executeimportedinitactions_gfxmoviedef_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033715_gfxloadqueueentry_dtorgfxloadqueueentry(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033716_gfxmovieroot_getlevelmovie_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_033720_gfxmovieroot_releaselevelmovie_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_033721_gfxsprite_executeframe0events(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033722_gfxmovieroot_doactions(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033723_gfxmovieroot_shutdowntimers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033724_gfxsprite_executeframetags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_033725_gfxmovieroot_actionqueueiterator_getnext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033726_gfxmovieroot_actionentry_clearaction(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033727_gfxmovieroot_actionentry_dtoractionentry(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033728_void_ghashsetbase_ghashnode_gasstring_gfxmovieroot_stickyvarnode(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033738_gfxmovieroot_dtorgfxmovieroot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033739_gfxmovieroot_invokealiasinfo_dtorinvokealiasinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033740_gfxmovieroot_focusgroupdescr_dtorfocusgroupdescr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033741_gasintervaltimer_dtorgasintervaltimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033747_gfxmovieroot_addloadqueueentry_gfxascharacter_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_033748_gfxmovieroot_createimagemoviedef_gfximageresource_ptr_bool_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_033749_void_gfxascharacter_modifyoptimizedplaylistlocal_gfxsprite_gfxmo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033750_gfxsprite_checkadvancestatus_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_033751_gfxmovieroot_processloadqueue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033752_gfxmovieroot_actionqueuesessioniterator_getnext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033759_gfxmovieroot_advanceframe_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_033760_gfxmovieroot_focusgroupdescr_resettabablearray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033761_gfxmousestate_settopmostentity_gfxascharacter_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033762_gfxmousestate_istopmostentitychanged_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_033763_gfxmovieroot_changemousecursortype_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_033764_gfxmovieroot_finalizeprocessfocuskey_gfxmovieroot_processfocuske(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033765_gfxmovieroot_processfocuskey_gfxevent_eventtype_gfxinputeventsqu(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_033766_gfxmovieroot_hidefocusrect_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_033767_gfxmovieroot_queuesetfocusto_gfxascharacter_ptr_gfxascharacter_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_033768_gfxmovieroot_initfocuskeyinfo_gfxmovieroot_processfocuskeyinfo_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_033789_gfxmovieroot_asvalue2gfxvalue_gasenvironment_ptr_gasvalue_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033790_gfxvalue_objectinterface_objectrelease_gfxvalue_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033792_gfxmovieroot_gfxvalue2asvalue_gfxvalue_const_ref_gasvalue_ptr_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033793_gfxmovieroot_addstickyvariable_gasstring_const_ref_gasvalue_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_033798_gfxmovieroot_returnvalueholder_resizestringarray_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_033801_gfxmovieroot_invoke_gfxsprite_ptr_char_const_ptr_gfxvalue_ptr_gf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_033802_gfxmovieroot_resolveinvokealias_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033804_gfxmovieroot_invokealiasargs_char_const_ptr_gfxmovieroot_invokea(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_033811_gfxmovieroot_transferfocus_gfxascharacter_ptr_unsigned_int_gfxfo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_033821_gfxmovieroot_findexportedresource_gfxmoviedefimpl_ptr_gfxresourc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_033822_gfxswfevent_read_gfxstreamcontext_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033827_gfxstreamcontext_readmatrix_gmatrix2d_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033828_gfxstreamcontext_readuint_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_033831_gfxplaceobject2_executebase_gfxsprite_ptr_unsigned_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033832_gfxplaceobject2_unpackbase_gfxplaceobjectbase_unpackeddata_ref_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033833_gfxsprite_replacedisplayobject_gfxcharposinfo_const_ref_gasstrin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033834_gfxstreamcontext_readcxformrgba_grenderer_cxform_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033835_garraydatabase_gfxswfevent_ptr_gallocatorlh_gfxswfevent_ptr_260(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033836_gfxsprite_replacedisplayobject_gfxcharposinfo_const_ref_gfxchara(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033849_gfxresourcelib_resourceslot_dtorresourceslot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033851_gfxresourceweaklib_dtorgfxresourceweaklib(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033854_void_ghashsetbase_gfxresource_ptr_gfxresourcelib_gfxresourceptrh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033857_garraydatabase_gfxscale9gridinfo_imgadjust_gallocatorgh_gfxscale(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033861_gmath2d_checkcurveintersection_float_float_float_float_float_flo(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32) -> i32; }
extern "C" { pub fn f_033862_gfxpathallocator_allocpath_unsigned_int_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_033863_garraydatabase_gfxpathpacker_edge_gallocatorgh_gfxpathpacker_edg(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033865_bool_gfxshapebase_defpointtestlocalimpl_gfxswfpathdata_gpoint_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_033867_void_gfxshapebase_tessellateimpl_gfxswfpathdata_gfxmeshset_ptr_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64, a5: f32); }
extern "C" { pub fn f_033868_gfxswfpathdata_pathsiterator_addfortessellation_gcompoundshape_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033872_bool_gfxshapebase_isequalgeometryimpl_gfxswfpathdata_gfxshapebas(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033874_void_gfxshapebase_computeboundimpl_gfxswfpathdata_grect_float_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033875_gfx_readfillstyles_garray_gfxfillstyle_259_garraydefaultpolicy_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_033876_gfx_readlinestyles_garray_gfxlinestyle_259_garraydefaultpolicy_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_033877_gfxstream_readuint4(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_033878_gfxfillstyle_read_gfxloadprocess_ptr_gfxtagtype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033879_gfxlogbase_gfxloadprocess_logerror_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033881_gfxconstshapenostyles_getpathsiterator_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033882_gfxstreamcontext_readuint4(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_033883_gfxstreamcontext_readuint5(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_033885_void_gfxshapebase_tessellateimpl_gfxpathdata_gfxmeshset_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64, a5: f32); }
extern "C" { pub fn f_033886_gfxpathdata_pathsiterator_addfortessellation_gcompoundshape_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033887_gfxpathdata_pathsiterator_skipcomplex(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033889_bool_gfxshapebase_defpointtestlocalimpl_gfxpathdata_gpoint_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_033892_unsigned_int_gfxshapebase_computegeometryhashimpl_gfxpathdata_co(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_033894_bool_gfxshapebase_isequalgeometryimpl_gfxpathdata_gfxshapebase_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_033896_gfxvirtualpathiterator_gfxshapenostyles_gfxpathdata_gfxvirtualpa(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033898_gfxconstshapewithstyles_dtorgfxconstshapewithstyles(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033918_gfxspritedef_dtorgfxspritedef(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033922_gfxsprite_setscale9grid_gfxscale9grid_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033923_gfxsprite_dtorgfxsprite(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033924_gfxsprite_setmask_gfxsprite_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033925_gasenvironment_dtorgasenvironment(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033941_gfxsprite_calcdisplaylisthittestmaskarray_garray_unsigned_char_2(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033945_gfxtimelinesnapshot_add_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_033950_gfxremoveobjectbase_addtotimelinesnapshot_gasexecutetag_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_033952_gfxsprite_executesnapshot_gfxtimelinesnapshot_ptr_gfxactionprior(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033953_gfxtimelinesnapshot_sourcetags_unpack_gfxplaceobjectbase_unpacke(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033954_gfxsprite_addactionbuffer_gasactionbuffer_ptr_gfxactionpriority(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_033967_gfxsprite_getmember_gasenvironment_ptr_gasstringcontext_ptr_gass(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_034002_gfxsprite_acquirepath_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_034003_gfxsprite_setlinestyle_float_unsigned_int_bool_gfxlinestyle_line(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32); }
extern "C" { pub fn f_034004_gfxsprite_beginfill_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_034008_gasmovieclipobject_trackmemberbuttonhandler_gasstringcontext_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_034009_gasmovieclipobject_set_proto_gasstringcontext_ptr_gasobject_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034013_gasmovieclipobject_setmembercommon_gasstringcontext_ptr_gasstrin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_034021_gfxstatictextrecordlist_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034025_gfxresourcebinding_getresourcedata_gfxresourcehandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034026_gfxstatictextcharacter_dtorgfxstatictextcharacter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034027_gfxdrawingcontext_dtorgfxdrawingcontext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034034_gfxstream_readstring_gstring_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034038_gfxgradientdata_createimageresource_gfxresourceweaklib_ptr_gfxgr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_034039_gfxgradientdata_computeradialgradientimagesize_gfxgradientparams(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_034040_gfxfillstyle_gfxfillstyle(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034041_gfxfillstyle_dtorgfxfillstyle(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034042_void_ghashsetbase_unsigned_int_gfixedsizehash_unsigned_int_gfixe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_034056_undoandfilterpremultiplied_gimage_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034074_gfxlogbase_gfxloadprocess_logwarning_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034076_gfx_createimagefileresourcehandle_gfxloadprocess_ptr_gfxresource(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_034086_gfxedittextcharacter_createeditorkit(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034094_gfxlogbase_gfxcharacter_logscriptwarning_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034097_gfxedittextcharacter_processimagetags_garraydh_gfxstyledtext_htm(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034108_gfxedittextcharacter_proceedimagesubstitution_gasfncall_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_034109_gasobjectinterface_getconstmemberraw_gasstringcontext_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_034111_gfxedittextcharacter_clearidimagedescassoc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034117_gfxedittextcharacter_updatevariable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034118_gfxedittextcharacter_notifychanged(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034122_gfxedittextcharacter_changeurlformat_gfxedittextcharacter_linkev(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_034133_gfxtextkeymap_find_unsigned_int_gfxspecialkeysstate_const_ref_gf(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_034135_gasuserdefinedfunctionobject_invokeimpl_gasfncall_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034136_garraydata_gfxvalue_gallocatorgh_cpp_gfxvalue_2_garraydefaultpol(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034137_gfxvalue_objectinterface_objectaddref_gfxvalue_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034143_gfxvalue_objectinterface_hasmember_void_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_034144_gfxvalue_objectinterface_getmember_void_ptr_char_const_ptr_gfxva(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_034145_gfxvalue_objectinterface_setmember_void_ptr_char_const_ptr_gfxva(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_034146_gfxvalue_objectinterface_invoke_void_ptr_gfxvalue_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_034147_gfxvalue_objectinterface_visitmembers_void_ptr_gfxvalue_objectin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_034148_gfxvalue_objectinterface_getarraysize_void_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_034149_gfxvalue_objectinterface_getelement_void_ptr_unsigned_int_gfxval(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_034150_gfxvalue_objectinterface_getdisplayinfo_void_ptr_gfxvalue_displa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_034151_gfxvalue_objectinterface_setdisplayinfo_void_ptr_gfxvalue_displa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034152_gfxvalue_objectinterface_settext_void_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_034153_gfxvalue_objectinterface_gotoandplay_void_ptr_char_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_034154_gfxvalue_objectinterface_gotoandplay_void_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_034155_gfxvalue_objectinterface_setcxform_void_ptr_grenderer_cxform_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034156_gfxvalue_objectinterface_createemptymovieclip_void_ptr_gfxvalue(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_034157_gfxvalue_objectinterface_attachmovie_void_ptr_gfxvalue_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_034166_gaslocalframe_dtorgaslocalframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034190_gasasfunctionobject_dtorgasasfunctionobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034235_gfxdrawtextimpl_dtorgfxdrawtextimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034403_gfxedittextcharacter_dtorgfxedittextcharacter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034404_gautoptr_gfxedittextcharacter_cssholder_reset_gfxedittextcharact(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034412_void_gfxascharacter_modifyoptimizedplaylistlocal_gfxedittextchar(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034413_gfxedittextcharacter_updatetextfromvariable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034415_gfxedittextcharacter_updateandgetgeomdata_gfxascharacter_geomdat(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_034422_gfxedittextcharacter_isurlundermousecursor_unsigned_int_gpoint_f(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_034430_gfxedittextcharacter_gettextfieldasobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034434_gfxedittextcharacter_setmember_gasenvironment_ptr_gasstring_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_034435_gfxedittextcharacter_updateautosizesettings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034436_gasstringcontext_compareconststring_casecheck_gasstring_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_034437_gfxedittextcharacter_collecturlzones(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034438_gfxedittextcharacter_getmember_gasenvironment_ptr_gasstring_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_034439_gfxedittextcharacter_checkadvancestatus_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_034455_gfxedittextcharacter_settextvalue_char_const_ptr_bool_bool_trans(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034456_gfxedittextcharacter_getinitialformats_gfxtextformat_ptr_gfxtext(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034472_gasvalueproperty_dtorgasvalueproperty(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034490_gfxrendergen_dtorgfxrendergen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034529_gassuperobject_dtorgassuperobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034554_gasglobalobject_setmember_gasenvironment_ptr_gasstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_034564_grefcountcollector_323_addroot_grefcountbasegc_323_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034565_garraypagedbase_grefcountbasegc_323_ptr_10_5_gallocatorpagedlh_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_034566_gfxbuttoncharacter_dtorgfxbuttoncharacter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034585_gfxbuttoncharacter_getbuttonasobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034590_gfxbuttoncharacter_getmember_gasenvironment_ptr_gasstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_034608_gfxstatebagimpl_dtorgfxstatebagimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034610_gfxstatebagimpl_setstate_gfxstate_statetype_gfxstate_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_034611_void_ghashsetbase_gfxstatebagimpl_stateptr_gfxstatebagimpl_state(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034612_gfxstatebagimpl_getstateaddref_gfxstate_statetype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_034613_gfxstatebagimpl_getstatesaddref_gfxstate_ptr_ptr_gfxstate_statet(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_034621_void_ghashsetbase_gfxstatebagimpl_stateptr_gfxstatebagimpl_state(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_034622_gfxtextallocator_dtorgfxtextallocator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034628_gfxmoviedefbindstates_dtorgfxmoviedefbindstates(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034641_gfxmovieimageloadtask_dtorgfxmovieimageloadtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034696_gfx_spritecreategradient_gasfncall_const_ref_gfxfillstyle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034709_gfxedittextcharacter_textdocumentlistener_onscroll_gfxtextdocvie(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034718_ghashsetbase_gstringlh_hashnode_gptr_gfxtextimagedesc_gstring_no(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034723_grangedataarray_gfxedittextcharacter_cssholder_urlzone_garray_gr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_034724_grangedataarray_void_ptr_garraylh_grangedata_void_ptr_2_garrayde(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034725_grangedataarray_gfxedittextcharacter_cssholder_urlzone_garray_gr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_034726_garraydatabase_grangedata_gfxedittextcharacter_cssholder_urlzone(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034727_grangedataarray_gfxedittextcharacter_cssholder_urlzone_garray_gr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034728_ghashsetbase_ghashnode_gasbuiltintype_gptr_gasobject_gfixedsizeh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034729_ghashsetbase_ghashnode_gasstring_gasglobalcontext_classregentry(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034730_ghashsetbase_ghashnode_gasstring_gasfunctionref_gasstringhashfun(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034737_ghashsetbase_ghashnode_gasstring_gasvalue_gasstringhashfunctor_g(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034740_ghashsetbase_ghashnode_gfxeventid_garraylh_gasvalue_323_garrayde(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034741_void_ghashsetbase_ghashnode_gfxeventid_garraylh_gasvalue_323_gar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_034742_void_ghashsetbase_ghashnode_unsigned_short_unsigned_short_gident(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_034743_ghashsetbase_ghashnode_unsigned_short_unsigned_short_gidentityha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034744_ghashsetbase_ghashnode_gfxbatchpackagedata_batchinfokey_gfxbatch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034745_ghashsetbase_ghashnode_gfxfontcachemanagerimpl_vectorglyphkey_gf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034746_ghashsetbase_ghashnode_gfxfontglyphpacker_glyphgeometrykey_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034747_ghashsetbase_ghashnode_gstring_gstring_gstring_nocasehashfunctor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034748_ghashsetbase_gfxfontmanager_nodeptr_gfxfontmanager_nodeptrhashop(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034749_ghashsetbase_gstringlh_hashnode_gfxfontmap_mapentry_gstring_noca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034750_ghashsetbase_ghashnode_gfxresourceid_gfxresourceptr_gfximagereso(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034751_ghashsetbase_ghashnode_gfxfontdata_kerningpair_float_gfixedsizeh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034752_ghashsetbase_ghashnode_gfxglyphparam_gfxglyphnode_ptr_gfxglyphpa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034753_ghashsetbase_ghashnode_gfxresourceid_gfxresourcehandle_gfxresour(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034754_ghashsetbase_gstringlh_hashnode_unsigned_int_gstring_nocasehashf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034755_ghashsetbase_gstringlh_hashnode_gfxresourcehandle_gstring_nocase(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034756_ghashsetbase_ghashnode_gfxresourceid_gstringlh_gfixedsizehash_gf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034757_ghashsetbase_ghashnode_gstring_bool_gstring_nocasehashfunctor_gh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034758_ghashsetbase_gstring_gstring_nocasehashfunctor_gstring_nocasehas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034759_ghashsetbase_ghashnode_gasstring_gfxmovieroot_stickyvarnode_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034760_gfxmovieroot_invokealiasinfo_op_gfxmovieroot_invokealiasinfo_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034761_ghashsetbase_ghashnode_gasstring_gfxmovieroot_invokealiasinfo_ga(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034762_gasautotabsortfunctor_op_gfxascharacter_const_ptr_gfxascharacter(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_034763_gfxrendergenstroker_dtorgfxrendergenstroker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034771_sfrenderer_scheduletexturedeletion_sfrenderer_texturedeletioninf(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034772_sfrenderer_deletetexture_sfrenderer_texturedeletioninfo_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034785_sfrenderersinglethreaded_setcxform_grenderer_cxform_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034786_sfrenderersinglethreaded_updateblendmode_grenderer_blendtype_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_034787_sfrenderersinglethreaded_setmatrix_gmatrix2d_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034788_sfrenderersinglethreaded_settexture_unsigned_int_grenderer_fillt(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_034789_sfrenderer_dtorsfrenderer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034837_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_sf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_034838_rage_sfpreallocatedmemorywrapper_dtorsfpreallocatedmemorywrapper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034842_rage_sfpreallocatedmemorywrapper_printdebuginfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034843_rage_sfpreallocatedmemorywrapper_logalloc_rage_atrangearray_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034844_rage_sfpreallocatedmemorywrapper_computecurrentpreallocationneed(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_034846_rage_sfpreallocatedmemorywrapper_computepreallocation_rage_atfix(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_034847_void_std_2_introsort_std_2_classicalgpolicy_rage_sfsortbysize_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_034848_rage_sfresourceinfo_insertalloc_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034868_rage_sfscaleformmanager_autosetcurrmoviename_autosetcurrmovienam(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034882_rage_sfscaleformmanager_resetmeshcache(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034883_rage_sfscaleformmanager_setedgeaaenable_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_034885_rage_tweenstar_computeeasedvalue_float_rage_tweenstar_easetype(ph: *mut Phantom, a0: f32, a1: i32) -> f32; }
extern "C" { pub fn f_034897_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_034898_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_sfsor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_034902_sftweenstarliteclass_removetweenof_gfxvalue_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034904_sftweengroup_finishtween_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_034905_sftweengroup_dtorsftweengroup(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034909_rage_atarray_sftweeninstance_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034910_sftweeninstance_applytween_float_gfxvalue_ref_gfxvalue_displayin(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_034917_cloadingscreens_initupdateintromovie_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_034918_cloadingscreens_initupdatelegalmain_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_034919_cloadingscreens_setuplegalpage_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_034920_cloadingscreenbuglist_updatetodonowbuglist_rage_partree_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034921_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_buginfo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_034922_cloadingscreens_bank_addtodonowbugs_char_const_ptr_rage_atarray(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034923_cloadingscreens_bank_updatesummarytext(ph: *mut Phantom); }
extern "C" { pub fn f_034924_cloadingscreenbuglist_resettodonowbuglist(ph: *mut Phantom); }
extern "C" { pub fn f_034925_cloadingscreens_initupdateloadgame_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_034926_cloadingscreens_updateinstallprogressbar_float_char_const_ptr(ph: *mut Phantom, a0: f32, a1: i64); }
extern "C" { pub fn f_034927_cloadingscreens_refreshbuttonlayout(ph: *mut Phantom); }
extern "C" { pub fn f_034928_cloadingscreens_updateloadingprogressbar(ph: *mut Phantom); }
extern "C" { pub fn f_034929_cloadingscreens_updateinput(ph: *mut Phantom); }
extern "C" { pub fn f_034930_cloadingscreens_committompsp_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034932_cloadingscreens_setspinner_int_char_const_ptr_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_034933_cloadingscreens_setbuttonloc_int_einstructionbuttons_char_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_034934_cloadingscreens_setbutton_int_einstructionbuttons_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_034935_cloadingscreens_setscreenorder_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_034936_cloadingscreens_setnewsscreenorder_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_034937_cloadingscreens_init_loadingscreencontext_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_034938_chudcolour_setvaluesfromdatafile(ph: *mut Phantom); }
extern "C" { pub fn f_034939_cloadingscreens_loadloadingscreenmovie_loadingscreenmovieindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_034940_cloadingscreens_loadlogo(ph: *mut Phantom); }
extern "C" { pub fn f_034941_cloadingscreens_gotolanding(ph: *mut Phantom); }
extern "C" { pub fn f_034943_cloadingscreens_update(ph: *mut Phantom); }
extern "C" { pub fn f_034944_cloadingscreens_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_034946_cloadingscreens_areactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034949_chudtools_getwidescreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034950_chudtools_getaspectratio_bool(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_034952_cnewhud_drawsafezone(ph: *mut Phantom); }
extern "C" { pub fn f_034953_chudtools_getstandardspriteblendstate(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_034954_chudtools_adjustnormalized16_9valuesforcurrentaspectratio_ewides(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_034956_chudtools_getminsafezone_float_ref_float_ref_float_ref_float_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32); }
extern "C" { pub fn f_034958_chudtools_adjustforwidescreen_ewidescreen_format_rage_vector2_pt(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_034962_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_userinf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_034967_chudtools_getuiwidth(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_034969_cwarningmessage_data_data(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034971_cmousepointer_devicereset(ph: *mut Phantom); }
extern "C" { pub fn f_034972_cmousepointer_issfmousereleased_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_034973_stfeeditem_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034974_cinfomenu_dtorcinfomenu(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034975_stfeeditem_dtorstfeeditem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034978_cinfomenu_postfeeditem_cinfomenu_ststreamedfeeditems_ref_int_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_034979_cinfomenu_populatenewswiremenu(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_034980_cinfomenu_postnewswirestory_cnetworkscnewsstoryrequest_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_034981_cinfomenu_updatenewswirecolumnscroll_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_034982_cinfomenu_ststreamedfeeditems_releaseandcanceltexture(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034985_stfeeditem_op_stfeeditem_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034986_cinfomenu_populateinfomenu_cmessages_epreviousmessagetypes(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_034990_ckeymappingmenu_dtorckeymappingmenu(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034993_ckeymappingmenu_sendcontroltoscaleform_rage_inputtype_int_bool_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_034996_ckeymappingmenu_adjustwarningindicators_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_034997_ckeymappingmenu_setinsubmenu_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034998_ckeymappingmenu_sendinput_rage_inputtype_unsigned_char(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_034999_ckeymappingmenu_updatecategory_rage_atnonfinalhashstring_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_035015_rage_atrangearray_rage_strrequest_16_dtoratrangearray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035021_cfrontendxmlmgr_registerxmlreader_rage_atnonfinalhashstring_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035022_rage_datcallback_ref_rage_atmap_rage_atnonfinalhashstring_rage_d(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_035023_cfrontendxmlmgr_loadxml_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035024_cnewhud_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035026_cnewhud_activatehudcomponent_bool_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_035027_cnewhud_resetallpreviousvalues(ph: *mut Phantom); }
extern "C" { pub fn f_035028_cnewhud_initstartingvalues(ph: *mut Phantom); }
extern "C" { pub fn f_035029_chudtools_calculatehudposition_rage_vector2_rage_vector2_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_035030_cnewhud_resetpreviousvalueofhudcomponent_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035031_cnewhud_handlecharacterchange(ph: *mut Phantom); }
extern "C" { pub fn f_035032_chudtools_getdifferencefrom_16_9_tocurrentaspectratio(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_035033_cnewhud_updatedisplayconfig_int_escaleform_base_class(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_035034_cnewhud_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035035_cnewhud_removehudcomponent_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035036_cnewhud_removehudcomponentfromactionscript_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035038_cnewhud_ishudcomponentactive_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035039_chudcolour_getcolourfromkey_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035040_chudcolour_getrgb_ehud_colours_unsigned_char(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_035041_chudtools_getminsafezoneforscaleformmovies_float_ref_float_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_035042_cnewhud_sethudcomponenttobeshown_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035044_cnewhud_sethudcomponenttobehidden_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035047_cnewhud_updatevisibility(ph: *mut Phantom); }
extern "C" { pub fn f_035049_cnewhud_canrenderhud(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035050_cnewhud_updateimevisibility(ph: *mut Phantom); }
extern "C" { pub fn f_035051_cnewhud_updatehelptext(ph: *mut Phantom); }
extern "C" { pub fn f_035052_cnewhud_updatehealthinfo_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035053_cnewhud_shoulddrawblinders(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035054_cminimap_getminimapped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_035055_cnewhud_gethealthinfo_cphysical_const_ptr_cnewhud_shealthinfo_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035056_cnewhud_getcash_long_ref_long_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035057_chudtools_callhudscaleformmethod_enewhudcomponents_char_const_pt(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_035058_chudtools_beginhudscaleformmethod_enewhudcomponents_char_const_p(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_035059_cpedinventory_getweaponammo_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_035060_cpedinventory_getisweaponusinginfiniteammo_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_035061_cnewhud_issnipersightactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035062_cminimap_getisinsideinterior_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035063_cnewhud_actuallyupdatevehiclename(ph: *mut Phantom); }
extern "C" { pub fn f_035064_cnewhud_shealthinfo_set_int_int_int_int_int_int_int_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i64, a12: i32); }
extern "C" { pub fn f_035067_cnewhud_forcemultiplayercashtoreshowifactive(ph: *mut Phantom); }
extern "C" { pub fn f_035070_cnewhud_overridempcash_long_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035072_cportaltracker_isallowedtorunininterior_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035077_cnewhud_sethudcomponentposition_int_rage_vector2(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_035078_cnewhud_resethudcomponentpositionfromoriginalxmlvalues_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035079_chudcolour_getrgba_ehud_colours(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035080_chudtools_getsafezonesize(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_035082_cnewhud_isweaponwheelactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035083_cnewhud_isradiowheelactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035085_cnewhud_enablesafezonerender_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035086_cnewhud_setdrawblinder_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035087_chudtools_issuperwidescreen_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035103_cnewhud_forceshowhudcomponent_enewhudcomponents_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_035105_chudtools_getuiheight(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_035107_chudtools_getformatfromstring_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035108_chudtools_adjustforsuperwidescreen_rage_vector2_ptr_rage_vector2(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035109_chudtools_getaspectratiomultiplier_bool(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_035111_chudcolour_getkeyfromcolour_ehud_colours(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035112_chudcolour_getcolourfromrgba_rage_color32(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035114_cphonemgr_createrendertargets(ph: *mut Phantom); }
extern "C" { pub fn f_035117_cphonemgr_delete(ph: *mut Phantom); }
extern "C" { pub fn f_035122_cphonemgr_loadmodel(ph: *mut Phantom); }
extern "C" { pub fn f_035128_cphonemgr_camischarvisible_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_035142_gfxvalue_gfxvalue_gfxvalue_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_035143_gfxvalue_dtorgfxvalue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035144_cminimap_common_getblipname_int_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035151_cmousepointer_setmousecursorvisible_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035152_cmousepointer_setmousecursorstyle_emouse_cursor_style(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035157_cmultiplayerchat_setfocus_int_cmultiplayerchat_etypemode(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_035162_cmultiplayerchat_forceaborttextchat(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035166_sgamertag_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035168_cmultiplayergamertaghud_updatedebugtagname(ph: *mut Phantom); }
extern "C" { pub fn f_035182_cmultiplayergamertaghud_cmultiplayergamertaghud(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035183_cmultiplayergamertaghud_ut_createfakeplayertag_cphysical_ptr_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_035184_cmultiplayergamertaghud_dtorcmultiplayergamertaghud(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035185_cmultiplayergamertaghud_ut_setgamertagvisibility_int_emp_tag_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_035186_cmultiplayergamertaghud_ut_setgamertagvaluehudalpha_int_emp_tag(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_035187_cmultiplayergamertaghud_ut_getphysical_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_035188_cmultiplayergamertaghud_ut_removeplayertag_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035189_cmultiplayergamertaghud_ut_createplayertaghelper_int_cphysical_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_035198_cmultiplayergamertaghud_rt_setcolourtogfxvalue_gfxvalue_ptr_rage(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035199_cmultiplayergamertaghud_rt_initcomponentgfxvalue_int_emp_tag_gfx(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_035200_cmultiplayergamertaghud_rt_getcomponentgfxvalue_gfxvalue_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_035203_cmultiplayergamertaghud_rt_reinitgamertagcrewdetails_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035204_gfxvalue_setstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035209_cmultiplayergamertaghud_rt_setgamertagvalueintongfxvalue_int_emp(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_035210_cmultiplayergamertaghud_rt_setgamertagvalueintongfxvalue_gfxvalu(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_035214_cmultiplayergamertaghud_rt_updatehealthcolorhelper_gfxvalue_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_035215_cmultiplayergamertaghud_rt_updatehealtharmourhelper_gfxvalue_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_035216_cmultiplayergamertaghud_rt_updatehealtharmourvisibilityhelper_gf(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_035244_cminimap_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035253_centitypoolindexforblip_centitypoolindexforblip_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_035254_centitypoolindexforblip_getentity_eblip_type_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_035255_rage_atmapentry_int_cblipcones_sconedetails_const_ref_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035256_cminimap_renderthread_setupcontainers(ph: *mut Phantom); }
extern "C" { pub fn f_035257_cminimap_getcurrentminimapposition(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035258_cminimap_getcurrentminimap(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035260_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_035261_cminimap_renderthread_setmaskonlayer_bool_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_035263_cminimap_creategenericblip_bool_eblip_type_centitypoolindexforbl(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_035264_cminimap_getblipfromactualid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_035265_cminimap_setblipnamevalue_cminimapblip_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_035266_cminimap_setflag_cminimapblip_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035269_cminimap_getbliptypevalue_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035270_cminimap_getblipdisplayvalue_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035271_cminimap_getblipnamevalue_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035272_cminimap_removeblip_cminimapblip_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035273_cminimap_destroyblipobject_cminimapblip_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035274_cminimap_getuniqueblipused_cminimapblip_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035275_cminimap_getactualblipused_cminimapblip_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035276_cminimap_createsonarblip_rage_vector3_const_ref_float_ehud_colou(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_035278_cminimap_updatebitmaponut_mm_bitmap_version(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035279_cminimap_getcentreposition_rage_vector3_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035280_cminimap_updatebitmapstate_sbitmapstruct_ref_bool_bool_char_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_035281_cminimap_getrange_bool(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_035282_char_ptr_rage_formatf_char_50ul_char_ref_50ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_035283_cminimap_isplayerusingparachute(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035286_cminimap_getcurrentminimapmasksize(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035287_cminimap_shouldprocessminimap(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035288_cminimap_shouldrenderminimap(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035289_cblipupdatequeue_empty(ph: *mut Phantom); }
extern "C" { pub fn f_035290_cblipupdatequeue_add_cblipcomplex_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035291_cminimap_modifylabelstoallactiveblips_bool_bool(ph: *mut Phantom); }
extern "C" { pub fn f_035294_cminimap_setminimapmodestate_esetup_state_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_035295_cminimap_showyoke_bool_float_float_int(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: i32); }
extern "C" { pub fn f_035297_cminimap_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035298_cminimap_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035299_cminimap_renderthread_removecontainers(ph: *mut Phantom); }
extern "C" { pub fn f_035300_cminimap_renderthread_removeblipfromstage_cblipcomplex_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_035303_cminimap_clearwaypointblipandroute_swaypointstruct_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035304_cminimap_switchoffwaypoint_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035305_cminimap_isflagset_cminimapblip_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_035307_cminimap_getblippositionvalue_cminimapblip_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035308_cminimap_findblipentity_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035309_cminimap_common_getcolourfromblipsettings_eblip_colours_bool_ehu(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_035310_cminimap_getblipcolourvalue_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035311_cminimap_getblipcolour32value_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035312_cminimap_getblipalphavalue_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035315_cminimap_getblipobjectnameid_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035317_cminimap_setblipparameter_eblip_params_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_035318_cminimap_clearallbliproutes(ph: *mut Phantom); }
extern "C" { pub fn f_035319_cminimap_setblipparameter_eblip_params_int_rage_color32(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_035321_cminimap_setblipparameter_eblip_params_int_rage_vector3_const_re(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_035322_cminimap_setblipparameter_eblip_params_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_035324_cminimap_getblip_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_035333_cminimap_createwaypointblipandroute_swaypointstruct_ref_eblip_di(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035334_cminimap_renderthread_removeinterior(ph: *mut Phantom); }
extern "C" { pub fn f_035335_cminimap_renderthread_handleinterioratendofframe(ph: *mut Phantom); }
extern "C" { pub fn f_035336_cminimap_renderthread_removegolfcourse_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035337_cminimap_renderthread_handlegolfcourseatendofframe(ph: *mut Phantom); }
extern "C" { pub fn f_035338_cminimap_renderthread_removeconefrombliponstage_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035340_cminimap_getactualcentreblipid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035341_cminimap_renderthread_removehigherlowerblip_gfxvalue_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035342_cminimap_renderthread_setuphealtharmouronstage_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035343_cminimap_renderthread_updatemapvisibility(ph: *mut Phantom); }
extern "C" { pub fn f_035344_cminimap_updatecentreandnorthblips(ph: *mut Phantom); }
extern "C" { pub fn f_035346_cminimap_getactivewaypointid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035351_cminimap_getplayerblipposition(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035352_cminimap_addorremovepointofinterest_float_float(ph: *mut Phantom, a0: f32, a1: f32); }
extern "C" { pub fn f_035354_cminimap_switchonwaypoint_float_float_unsigned_short_bool_bool_e(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_035363_cminimap_updateminimapdebug(ph: *mut Phantom); }
extern "C" { pub fn f_035367_rage_atpagedpool_cblipcomplex_true_rage_fwdefaultrscmempagedpool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035371_cminimap_renderthread_addbliptostage_cblipcomplex_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035372_cminimap_renderthread_isareablipoutsidemapbounds_cblipcomplex_co(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035373_cminimap_renderthread_updateblipsecondaryicononstage_gfxvalue_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_035374_cminimap_renderthread_labelbliponstage_gfxvalue_ptr_cblipcomplex(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_035375_cminimap_renderthread_getblipobjectname_cblipcomplex_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035376_cshaderlib_lookuptexture_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035377_cminimap_common_getscalefromzoomlevel_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_035378_cminimap_common_getscrollspeedfromzoomlevel_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_035400_csupertile_render_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035401_csupertile_renderfow(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035403_char_ptr_rage_formatf_char_20ul_char_ref_20ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_035404_cminimap_renderthread_setupminimapproperties_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035405_char_ptr_rage_formatf_char_90ul_char_ref_90ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_035406_cminimap_renderthread_updatewithactionscriptonrt_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035407_cminimap_renderthread_updatetiles_bool(ph: *mut Phantom); }
extern "C" { pub fn f_035409_cminimap_renderthread_setupgolfcourselayout(ph: *mut Phantom); }
extern "C" { pub fn f_035410_cminimap_renderthread_updatesea_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035411_cminimap_renderthread_adjustvisualstagespacewithinmovie_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_035413_cminimap_renderthread_shouldblipuseweapontypearrows_cblipcomplex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035414_multimonitorhudhelper_multimonitorhudhelper_sminimaprenderstates(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035415_rage_atarray_debugvolume_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035416_cminimap_renderthread_isblipnametagwithinbounds_gfxvalue_ptr_cbl(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_035417_cminimap_renderthread_rendernavpath_int_int_rage_vector3_ptr_rag(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: f32, a5: f32, a6: i32, a7: f32, a8: f32, a9: f32, a10: f32); }
extern "C" { pub fn f_035418_cminimap_renderthread_setupstateforsupertilerender_rage_vector2(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035419_cminimap_renderthread_renderminimap_int_rage_vector2_rage_vector(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_035421_cminimap_renderthread_drawfeatheredrect_rage_vector2_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_035422_cminimap_renderthread_renderpausemap_rage_vector2_rage_vector2_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_035423_cminimap_renderthread_fowreveal(ph: *mut Phantom); }
extern "C" { pub fn f_035443_chudmarkerrenderer_shutdownscaleformut(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035446_chudmarkermanager_updatemovieconfig(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035447_void_std_2_stable_sort_std_2_classicalgpolicy_chudmarkermanager(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_035449_chudmarkermanager_createmarker_shudmarkerstate_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_035450_void_std_2_stable_sort_std_2_classicalgpolicy_chudmarkermanager(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_035458_void_std_2_stable_sort_std_2_classicalgpolicy_chudmarkerrenderer(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_035466_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_035467_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_035468_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_035469_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_035470_rage_atpagedpool_cblipcomplex_true_rage_fwdefaultrscmempagedpool(ph: *mut Phantom); }
extern "C" { pub fn f_035471_void_std_2_iterops_std_2_classicalgpolicy_iter_swap_abi_nqn22010(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035472_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_035473_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_035474_void_std_2_stable_sort_move_std_2_classicalgpolicy_chudmarkerman(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_035475_void_std_2_inplace_merge_std_2_classicalgpolicy_chudmarkermanage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_035476_std_2_pair_hudid_unsigned_short_unsigned_short_500_ptr_hudid_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_035477_void_std_2_stable_sort_move_std_2_classicalgpolicy_chudmarkerman(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_035478_void_std_2_inplace_merge_std_2_classicalgpolicy_chudmarkermanage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_035479_void_std_2_stable_sort_move_std_2_classicalgpolicy_chudmarkerren(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_035480_void_std_2_inplace_merge_std_2_classicalgpolicy_chudmarkerrender(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_035481_cbootupscreen_canselectonline(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035482_cbootupscreen_geteventbuttontext_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_035483_cbootupscreen_getjobbuttontext_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_035487_cbusyspinner_off_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035488_cbusyspinner_update(ph: *mut Phantom); }
extern "C" { pub fn f_035489_cbusyspinner_hidespinneronallmovies(ph: *mut Phantom); }
extern "C" { pub fn f_035490_cbusyspinner_displayspinneronallmovies(ph: *mut Phantom); }
extern "C" { pub fn f_035491_cbusyspinner_render(ph: *mut Phantom); }
extern "C" { pub fn f_035493_cbusyspinner_preload_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035494_cbusyspinner_on_char_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_035495_cbusyspinner_registerinstructionalbuttonmovie_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035496_cbusyspinner_unregisterinstructionalbuttonmovie_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035510_cmenubase_show_column_pm_columns_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035512_ccrewmenu_setcrewindexandpreparecard_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_035513_cpausemenu_getcurrentscreendata(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_035521_rage_fwlocalisedstring_append_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035522_uirequestpaginator_handlerequest_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_035523_uirequestpaginator_getrequest_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_035528_rage_fwlocalisedstring_set_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035529_ccrewmenu_clear_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035534_cmenubase_cmenubase_cmenuscreen_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_035535_ccontextmenu_closemenu(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035536_ccrewmenu_updatecolumnmodecontexts_ccrewmenu_columnmode(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035537_ccrewmenu_populatemenuforcurrentvariant(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035538_cmenubase_handlecolumns_pm_columns_ref_bool_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_035539_ccrewmenu_dtorccrewmenu(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035546_ccrewmenu_setcolumnbusy_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035548_suicontexts_setactive_rage_athashvalue_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035549_cmenubase_clearwarningcolumn(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035553_ccontextmenuhelper_handleplayercontextoptions_rage_atnonfinalhas(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_035554_ccrewmenu_getclandescforcurrentvariant_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_035557_cwarningmessage_data_dtordata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035559_ccrewmenu_joinselectedclancb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035561_ccrewmenu_setprimaryclancb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035562_ccrewmenu_getresultsforcurrentvariant(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035563_ccontextmenuhelper_incommendcooldown(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035564_ccontextmenuhelper_inkickcooldown(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035565_ccontextmenuhelper_createwarningmessage_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035566_ccontextmenuhelper_inreportcooldown(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035572_ccontextmenuhelper_buildplayercontexts_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_035574_ccrewmenu_setvariant_ccrewmenu_variant(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035576_ccrewmenu_populatewithmessage_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_035577_cplayerlistmenu_losefocuslite(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035578_cscriptmenu_losefocus(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035580_ccrewmenu_setthirdcolumnvisible_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035582_cmenubase_show_warning_message_ex_pm_columns_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_035583_ccrewmenu_populatemenuforerror_ccrewmenu_variant(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035584_cmenubase_showcolumnwarning_pm_columns_int_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32, a8: i64); }
extern "C" { pub fn f_035585_ccrewmenu_unpopulatemessage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035586_cmenubase_clearwarningcolumn_ex_pm_columns(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035591_cmenubase_getmenuscreenid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035593_ccrewmenu_updateitemcount_menuscreenid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_035594_ccrewmenu_filloutcrewcard_bool_ref_long_bool_crewcardstatus_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_035595_ccrewmenu_addcrewstat_int_char_const_ptr_int_int_bool_pm_columns(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_035596_cmenubase_showcolumnwarning_ex_pm_columns_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_035600_menuscreenid_getparsername_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035605_cplayerlistmenu_addorupdateslot_int_int_bool_playerdataslot_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_035606_cplayerlistmenu_refreshbuttonprompts(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035607_cplayerlistmenu_getboolstat_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_035626_cplayerlistmenu_cplayerlistmenu_cmenuscreen_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_035627_cscriptmenu_cscriptmenu_cmenuscreen_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_035628_cplayerlistmenu_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035629_cscriptmenu_dtorcscriptmenu(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035631_cplayerlistmenu_getgamerhandle_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_035632_cplayerlistmenu_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035633_cscriptmenu_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035634_cplayerlistmenu_updatepedstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035635_cplayerlistmenu_initpaginator_menuscreenid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035636_cplayerlistmenu_refreshplayers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035637_cplayerlistmenu_getactiveplayerindex_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035638_cplayerlistmenu_filloutplayermissioninfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035639_cscriptmenu_handlestreaming(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035640_cscriptmenu_launchscript_spausemenuscriptstruct_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035641_cplayerlistmenu_isactiveplayer_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035642_cplayerlistmenu_getnetgameplayer_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_035647_cplayerlistmenu_buildcontexts(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035648_cplayerlistmenu_getstatasfloat_int_int_double(ph: *mut Phantom, a0: i32, a1: i32, a2: f64) -> f64; }
extern "C" { pub fn f_035652_cplayerlistmenu_initjipsyncing_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035653_cplayerlistmenu_oninputcancleinvite_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035654_cplayerlistmenu_setupcentercolumnwarning_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_035658_cplayerlistmenu_setupjipfailedwarning_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035659_cplayerlistmenu_launchupdateinputscript_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035662_cplayerlistmenu_setplayercard_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_035663_cplayerlistmenu_getcharacterslot_int_cplayerlistmenu_estatmode(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035664_cplayerlistmenu_getintstat_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_035668_cmenubase_showcolumnwarning_loc_pm_columns_int_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32, a8: i32); }
extern "C" { pub fn f_035671_cplayerlistmenu_addstatparam_int_int_cplayercardxmldatamanager_c(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_035674_cplayerlistmenu_getplayercolor_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_035675_playerdataslot_op_playerdataslot_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_035676_cplayerlistmenu_updateinput_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_035677_cplayerlistmenu_updateinputforinvitemenus_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_035681_cplayerlistmenu_getcrewemblem_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_035682_cplayerlistmenu_getcrewranktitle_rage_rlgamerhandle_const_ref_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_035683_cplayerlistmenu_getcrewcolor_rage_rlgamerhandle_const_ref_rage_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_035685_cplayerlistmenu_getcrewemblem_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_035686_cplayerlistmenu_getcrewranktitle_int_char_ptr_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_035687_cplayerlistmenu_getcrewcolor_int_rage_color32_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_035688_cplayerlistmenu_populate_menuscreenid(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_035697_cplayerlistmenu_initdatamanagerandpaginator_menuscreenid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035700_cscriptmenu_addwidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035703_cplayerlistmenu_dtorcplayerlistmenu_61328(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035725_clannetstatusarray_rage_rlclanmember_32_clannetstatusarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035727_cpartyclandata_dtorcpartyclandata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035739_rage_atarray_baseplayercarddatamanager_gamerdata_0_unsigned_shor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035741_cscriptedplayersmenu_populate_menuscreenid(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_035759_hoveredblipuniqueid_dtorhoveredblipuniqueid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035763_cmapmenu_dtorcmapmenu(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035767_cmapmenu_setmapzoom_int_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035769_cmapmenu_updatemapscreeninput_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_035770_cmapmenu_updatezooming_float_float_bool(ph: *mut Phantom, a0: f32, a1: f32, a2: i32); }
extern "C" { pub fn f_035771_cmapmenu_resetcornerblipinfo_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_035772_cmapmenu_updatemouseclicksandposition_rage_vector2_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_035773_cmapmenu_getmapextents_rage_vector2_ref_rage_vector2_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_035774_cmapmenu_ismouseovermap_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035775_cmapmenu_findactualblipidnearpos_rage_vector2_const_ref_bool_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_035776_cmapmenu_setcurrentbliphover_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_035777_cmapmenu_setwaypoint_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035778_cmapmenu_displayareaname_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035779_cmapmenu_updatepausemaplegend_bool_bool(ph: *mut Phantom); }
extern "C" { pub fn f_035780_cmapmenu_handleclickaction_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035781_cmapmenu_reloadmap(ph: *mut Phantom); }
extern "C" { pub fn f_035784_cscriptmenu_render_pausemenurenderdataextra_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035792_cmapmenu_processlegend_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_035793_cmapmenu_updatelegenddisplayitem_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_035796_cmapmenu_removelegend(ph: *mut Phantom); }
extern "C" { pub fn f_035797_cmapmenu_updatelegendslotwithnameandcolour_smaplegendlist_ptr_cm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_035798_rage_atarray_smaplegendlist_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035799_rage_atarray_smaplegendlist_0_unsigned_short_deletefast_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035801_cmapmenu_highlightlegenditem_cminimapblip_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035802_cmapmenu_updatelegenddisplayitem_cminimapblip_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035803_char_ptr_rage_formatf_char_160ul_char_ref_160ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_035806_cmenubutton_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_035809_void_rage_parcgudestructor_cmenubutton_rage_parparsablestructure(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035812_cmenubuttonlist_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_035818_cmenubuttonlist_postload(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035820_cmenubutton_cmenubutton(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035821_findpriority_cmenubutton_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035822_cmenubuttonlist_draw_cscaleformmoviewrapper_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035830_ccontextmenuhelper_clbreportcommendhelper_unsigned_int_ref_stati(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_035844_scontrolgroup_dtorscontrolgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035846_ccontrollerlabelmgr_adjustlabels(ph: *mut Phantom); }
extern "C" { pub fn f_035848_rage_atarray_scontrolgroup_label_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035849_ccontrollerlabelmgr_setlabelscheme_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_035853_csettingsmenu_populatewithmessage_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_035855_csettingsmenu_populatecontrollerlabels_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035856_csettingsmenu_writegfxsettings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035857_csettingsmenu_applygfxsettings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035860_csettingsmenu_isprefcontroledbyslider_emenupref(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035874_rage_atarray_ccredititem_0_unsigned_short_destruct_ccredititem_p(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035878_ccreditsdownloadhelper_dtorccreditsdownloadhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035881_ccredits_init_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_035882_ccredits_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035887_cselectionwheel_handlebutton_rage_iovalue_const_ref_rage_vector2(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_035889_cselectionwheel_handlestick_rage_vector2_const_ref_int_int_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32) -> i32; }
extern "C" { pub fn f_035890_cselectionwheel_settargettimewarp_tw_dir(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035891_cselectionwheel_triggerfadeouteffect(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035892_cselectionwheel_createbankwidgets_rage_bkgroup_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035895_cradiowheel_update_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_035896_cradiowheel_hide(ph: *mut Phantom); }
extern "C" { pub fn f_035897_cradiowheel_retunestation_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035899_cweaponwheel_hide_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035900_cweaponwheel_clear_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035901_cweaponwheel_releasehdweapons(ph: *mut Phantom); }
extern "C" { pub fn f_035902_cweaponwheel_isweaponselectable_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035903_cweaponwheel_mapweaponslottoweaponwheel_int_rage_atnonfinalhashs(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_035904_cweaponwheel_sweaponwheelslot_setscaleform_bool_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_035905_cweaponwheel_update_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_035906_cweaponwheel_isforcedopenbytimer(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035907_cweaponwheel_updatescaleform3dsettings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035908_cweaponwheel_findandupdateweaponwheelstats_bool_rage_atnonfinalh(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_035909_cweaponwheel_updateweaponattachment_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035910_cweaponwheel_setpointer_eweaponwheelslot(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035911_cweaponwheel_requesthdweaponassets_unsigned_char_rage_atnonfinal(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_035912_cweaponwheel_updateweaponwheelstatswithattachments_bool_rage_atn(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_035913_cweaponwheel_getholsterammoinclip(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035916_cweaponwheel_getclipsizefromcomponent_cped_const_ptr_rage_atnonf(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_035928_uicrewlbpage_dtoruicrewlbpage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035933_uigeneralcrewpage_long_1_dtoruigeneralcrewpage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035939_uigeneralcrewpage_rage_rlclandesc_2_dtoruigeneralcrewpage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035973_ccrewmembersclandata_dtorccrewmembersclandata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036006_clannetstatusarray_rage_rlclandesc_32_clannetstatusarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036031_cscaleformmgr_requestremovemovie_int_emovieownertags(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_036033_cscaleformmgr_rendermovie_int_float_bool_bool_float(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i32, a4: f32); }
extern "C" { pub fn f_036034_cscaleformmoviewrapper_isactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036035_cscaleformmoviewrapper_render_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_036036_cscaleformmgr_flushinvokelist(ph: *mut Phantom); }
extern "C" { pub fn f_036037_cscaleformmgr_updatemovie_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_036038_cscaleformmgr_ismovieactive_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036039_ccomplexobjectarrayitem_getstageroot_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036040_ccomplexobject_getobject_char_const_ptr_char_const_ptr_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_036041_ccomplexobjectarrayitem_settextfield_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036042_ccomplexobjectarrayitem_settextfieldhtml_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036043_ccomplexobjectarrayitem_setvisible_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036044_cscaleformmgr_beginmethodinternal_int_escaleform_base_class_char(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_036045_cscaleformmgr_endmethodinternal_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036046_cpcgamepadcalibration_promptstringconstruct_char_const_ptr_signe(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_036047_ccomplexobject_setvisible_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036048_cscaleformmgr_beginmethod_int_escaleform_base_class_char_const_p(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_036049_cscaleformmgr_addparamfloat_double(ph: *mut Phantom, a0: f64); }
extern "C" { pub fn f_036050_cscaleformmgr_endmethod_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036051_cscaleformmgr_addparambool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036053_cscaleformmgr_addparamint_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036054_cscaleformmgr_forcecollectgarbage_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036055_rage_atarray_ccomplexobjectarrayitem_0_unsigned_short_grow_unsig(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_036056_cscaleformmgr_addparamstring_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036057_cscaleformmgr_checktxdinstringhasref_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_036058_cscaleformmoviewrapper_createmovie_escaleform_base_class_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_036059_cscaleformmgr_createmovieinternal_cscaleformmgr_createmovieparam(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036061_cscaleformmgr_createmovie_char_const_ptr_rage_vector2_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_036062_ccomplexobject_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036063_ccomplexobjectarrayitem_releaseobjectinternal_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036064_cscaleformmoviewrapper_removemovie(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036065_cmenuitem_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_036071_uicontextlist_parsefromstring_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_036072_ccontextmenuoption_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_036077_ccontextmenu_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_036081_cmenuversion_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_036087_cmenuscreen_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_036096_cmenuscreen_dtorcmenuscreen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036102_cgeneralpausedata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_036103_cmenudisplayvalue_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_036105_cmenuarray_dtorcmenuarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036107_cmenuscreen_registertypes(ph: *mut Phantom); }
extern "C" { pub fn f_036137_cstreammoviehelper_remove(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036142_rage_atfixedarray_rage_atstring_4_dtoratfixedarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036144_cstreammoviehelper_set_char_const_ptr_menuscreenid_emenuceptiond(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_036145_cscaleformmoviewrapper_createmovieandwaitforload_escaleform_base(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_036154_cpausemenu_close(ph: *mut Phantom); }
extern "C" { pub fn f_036157_dynamicpausemenu_addwidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036158_cpausemenu_createxmldrivenwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_036159_cpausemenu_scalecontentmovie_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036160_cscaleformmgr_changemovieparams_int_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_036161_rage_atbinarymap_sgeneralpausedataconfig_rage_atnonfinalhashstri(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036162_cscaleformmoviewrapper_param_addparam(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036163_cpausemenu_updatedisplayconfig(ph: *mut Phantom); }
extern "C" { pub fn f_036164_cpausemenu_open_unsigned_int_bool_emenuscreen_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_036165_cpausemenu_setupmenustructurefromxml_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036166_cpausemenu_setcurrentmenuversion_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_036167_cpausemenu_removeallmovies(ph: *mut Phantom); }
extern "C" { pub fn f_036168_cpausemenu_openlite_menuscreenid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036169_cplayercardmanager_cplayercardmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036170_cpausemenu_setmenupreference_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036171_suicontexts_activate_rage_athashvalue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036172_suicontexts_deactivate_rage_athashvalue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036173_cpausemenu_restart_unsigned_int_emenuscreen(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036174_cpausemenu_setvaluebasedonpreference_int_cpausemenu_updateprefss(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_036175_cprofilesettings_set_cprofilesettings_profilesettingid_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036176_cpausemenu_redrawinstructionalbuttons_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036177_cpausemenu_generatemenudata_menuscreenid_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036178_cpausemenu_updatememorybar_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036179_cpausemenu_dirtygfxsettings(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036180_cpausemenu_dirtyadvgfxsettings(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036181_cpausemenu_updateprofilefrommenuoptions_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036182_cpausemenu_updatevoicesettings(ph: *mut Phantom); }
extern "C" { pub fn f_036183_cpausemenu_setpauserenderphases_bool_enum_pause_menu_pausedrp_ow(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036184_cpausemenu_togglepauserenderphases_bool_enum_pause_menu_pausedrp(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_036185_cpausemenu_closeinternal(ph: *mut Phantom); }
extern "C" { pub fn f_036186_cpausemenu_closeinternal2_bool(ph: *mut Phantom); }
extern "C" { pub fn f_036187_cpausemenu_togglefullscreenmap_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036191_suicontexts_bankadd_rage_athashvalue_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036196_cwarningmessage_data_op_cwarningmessage_data_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036197_cwarningmessage_setmessage_cwarningmessage_data_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036198_cwarningmessage_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036199_cscaleformmoviewrapper_callmethod_char_const_ptr_cscaleformmovie(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_036200_suicontexts_pushlist(ph: *mut Phantom); }
extern "C" { pub fn f_036201_cpausemenu_menuceptiongodeeper_menuscreenid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036202_cpausemenu_menuceptionthekick(ph: *mut Phantom); }
extern "C" { pub fn f_036203_cpausemenu_setcurrentscreen_menuscreenid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036204_cmenuscreen_finditemindex_menuscreenid_int_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_036205_uicontextlist_checkifpasses_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036206_cmenuscreen_finditemindex_emenupref_int_ptr_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036207_cmenuscreen_findonscreenitemindex_int_emenuaction_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_036208_cmenuscreen_finddynamicdata_rage_athashvalue_bool_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_036209_cmenuscreen_finddynamicdata_rage_athashvalue_rage_atstring_ref_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_036210_cmenuscreen_drawinstructionalbuttons_menuscreenid_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_036211_cmenuscreen_init_scroll_bar_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036212_cmenuscreen_init_scroll_bar_generic_rage_atfixedbitset_16_unsign(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036213_cscaleformmenuhelper_set_column_scroll_pm_columns_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_036214_cscaleformmenuhelper_hide_column_scroll_pm_columns(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036215_cscaleformmoviewrapper_beginmethod_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_036216_cscaleformmoviewrapper_endmethod(ph: *mut Phantom); }
extern "C" { pub fn f_036217_cmenuscreen_getcontextmenu(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036218_suicontexts_poplist(ph: *mut Phantom); }
extern "C" { pub fn f_036219_cmenuscreen_handlecontextmenuinput_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036220_cpausemenu_playinputsound_efrontend_input(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036221_cmenuarray_getcontextmenuoption_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036223_dynamicpausemenu_dtordynamicpausemenu(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036224_netaction_cancel_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036225_cuimenuped_dtorcuimenuped(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036226_netaction_dtornetaction(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036227_cfriendclandata_dtorcfriendclandata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036228_ctimewarper_reset_tw_dir(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036230_cwarningmessage_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036231_ctimewarper_settargettimewarp_tw_dir(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036232_cwarningmessage_data_updatecallbacks(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036233_cwarningscreen_setmessageinternal_ewarningmessagestyle_rage_atno(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i64, a8: i64, a9: i32, a10: i32, a11: i32); }
extern "C" { pub fn f_036234_netaction_onsuccess_cwarningmessage_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036236_cuimenuped_update_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_036237_cuimenuped_setvisible_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036238_cuimenuped_hasped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036239_cscaleformmgr_scaleposandsize_rage_vector2_ref_rage_vector2_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_036241_netaction_set_char_const_ptr_char_const_ptr_char_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_036242_netaction_setexternal_networkclan_enetworkclanops_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_036245_cpausemenu_updatemenuoptionsfromprofile(ph: *mut Phantom); }
extern "C" { pub fn f_036246_cpausemenu_setdefaultvalues_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036247_creportmenu_creportmenu(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036248_cpausemenu_restorepreviousgraphicsvalues_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036249_cpausemenu_restorepreviousadvancedgraphicsvalues_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036250_cpausemenu_restoregraphicsdefaults_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036251_cpausemenu_restoreadvancedgraphicsdefaults_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036254_cpausemenu_isactive_pm_activeflags(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036255_cpausemenu_backoutofsavegamelist(ph: *mut Phantom); }
extern "C" { pub fn f_036256_cpausemenu_backoutgraphicalchanges_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036257_suicontexts_clearall(ph: *mut Phantom); }
extern "C" { pub fn f_036258_cpausemenu_closecreditsscreen(ph: *mut Phantom); }
extern "C" { pub fn f_036259_cuireticule_spreviousframehudvalues_reset(ph: *mut Phantom); }
extern "C" { pub fn f_036260_cpausemenu_resetallglobalmenuvariables_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036261_rage_atarray_rage_atstring_const_ptr_0_unsigned_short_insert_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_036262_cpausemenu_recurserequiredmovies_cmenuscreen_ref_rage_atarray_me(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036263_cpausemenu_getactualscreen_menuscreenid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036264_rage_atarray_rage_atarray_rage_athashvalue_0_unsigned_short_0_un(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036265_cpausemenu_getcurrentscreen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036266_cpausemenu_getcurrentactivepanel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036268_rage_atarray_spausemenustate_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_036269_cpausemenu_setcurrentpane_menuscreenid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036270_cpausemenu_isgameinstatewhereitcanbepaused(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036271_cwarningscreen_isactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036274_cpausemenu_isstoreavailable(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036277_cpausemenu_setupheadertextandhighlights_cmenuscreen_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036278_cpausemenu_kickoffstreamingchildmovie_char_const_ptr_menuscreeni(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_036279_cpausemenu_getmenugraphicssettings_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036280_cpausemenu_getcurrentactivepaneldata(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_036281_cuireticule_reset(ph: *mut Phantom); }
extern "C" { pub fn f_036282_socialclubmenu_open_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036283_cpausemenu_setupcodeforunpause(ph: *mut Phantom); }
extern "C" { pub fn f_036284_cuimenuped_clearped_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036285_cstorescreenmgr_requestexit(ph: *mut Phantom); }
extern "C" { pub fn f_036286_cpausemenu_clearinstructionalbuttons(ph: *mut Phantom); }
extern "C" { pub fn f_036287_cpausemenu_loadbasemovie_enum_pause_menu_movies_char_const_ptr_c(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_036288_cpausemenu_opencorrectmenu_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036289_updatestate(ph: *mut Phantom); }
extern "C" { pub fn f_036292_suicontexts_isactive_rage_athashvalue(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036293_cpausemenu_handleradiostation_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036294_cpausemenu_playsound_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036295_cpausemenu_checkinput_efrontend_input_bool_unsigned_int_bool_boo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_036296_cpausemenu_updateplayerinfoattopofscreen_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036297_cpausemenu_setmaxpaynemode_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036299_cpausemenu_showconfirmationalert_char_const_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036300_cpausemenu_closeandplaypropersoundcb(ph: *mut Phantom); }
extern "C" { pub fn f_036301_cwarningscreen_checkallinput_bool_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_036302_cpausemenu_menuversionhasheadings(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036303_cpausemenu_updateinput_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036304_cpausemenu_getcurrenthighlightedtabdata(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_036306_cscaleformmenuhelper_set_data_slot_no_label_pm_columns_int_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_036307_cpausemenu_handleprefoverride_emenupref_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_036308_cpausemenu_updatevoicebar_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036309_cscaleformmenuhelper_display_data_slot_pm_columns(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036310_cpausemenu_menu_shift_depth_emenuceptiondir_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_036311_cscaleformmenuhelper_set_column_highlight_pm_columns_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_036312_cscaleformmenuhelper_set_column_focus_pm_columns_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036313_cscaleformmgr_forcemovieupdateinstantly_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036314_cwarningscreen_setmessagewithheader_ewarningmessagestyle_rage_at(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_036315_cwarningscreen_checkinput_int_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_036316_cpausemenu_areallassetsactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036317_cpausemenu_setnavigatingcontent_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036318_cpausemenu_showtabchangewarning_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036319_cpausemenu_triggerswitchpane_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036320_cpausemenu_shouldplaynavigationsound_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036321_cpausemenu_getcurrentactivepanelhasflag_emenuscreenbits(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036322_cpausemenu_anylayerhasflag_emenuscreenbits(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036324_cpausemenu_setrenderdata_pausemenurenderdataextra_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036326_cpausemenu_isingalleryscreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036327_cpausemenu_renderbackgroundcontent_pausemenurenderdataextra_cons(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036328_cpausemenu_renderbackground_pausemenurenderdataextra_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036329_cpausemenu_renderforegroundcontent_pausemenurenderdataextra_cons(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036330_cpausemenu_renderanimatedspinner_rage_vector2_rage_vector2_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_036331_cpausemenu_renderanimatedspinner_float_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036332_cscaleformmoviewrapper_affectrendersize_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_036333_cscaleformmgr_affectrendersizeonly_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_036339_cpausemenu_setbusyspinner_bool_signed_char_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_036340_cpausemenu_populateloadgamemenu_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036341_cscaleformmenuhelper_show_warning_message_pm_columns_int_char_co(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i32, a8: i64, a9: i32); }
extern "C" { pub fn f_036343_cscaleformmenuhelper_set_data_slot_pm_columns_int_int_int_eoptio(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64, a8: i32, a9: i32, a10: i32) -> i32; }
extern "C" { pub fn f_036344_cscaleformmenuhelper_set_data_slot_empty_pm_columns(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036345_cpausemenu_updateaudioimage(ph: *mut Phantom); }
extern "C" { pub fn f_036346_cpausemenu_handlesswidth_int_emenupref_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_036347_cpausemenu_handletessellation_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036348_cpausemenu_handlemsaa_emenupref_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_036349_cpausemenu_handlegrassquality_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036350_cpausemenu_handletxaa_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036351_cpausemenu_handleshadowsoftness_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_036352_cpausemenu_handledof_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036353_cpausemenu_handlembstrength_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036354_cpausemenu_handlescreentype_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036355_cpausemenu_handlescreenresolution_int_bool_bool_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_036356_cpausemenu_handle3dstereo_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036357_cpausemenu_getdisplayinfolist_rage_atarray_rage_grcdisplaywindow(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036358_rage_atarray_rage_grcdisplaywindow_0_unsigned_short_op_rage_atar(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036359_cpausemenu_handlescreenrefreshrate_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036360_cpausemenu_handlevideoaspect_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036361_cpausemenu_handle3dstereoconvergence_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036362_cpausemenu_getstereoconvergencefrontendvalue_float(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_036363_cpausemenu_handlestartupflow_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036364_cprofilesettings_getint_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036365_rage_atarray_rage_grcdisplaywindow_0_unsigned_short_grow_unsigne(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036366_cpausemenu_handleoutputmonitor_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036369_cpausemenu_preparepaneswitch(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036370_cpausemenu_kickoffswitchpane_bool_cmenuscreen_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_036371_cpausemenu_triggerswitchpanewithtabindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036372_cscaleformmgr_changemovieparams_int_rage_vector2_const_ref_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_036373_cscaleformmgr_setmoviedisplayconfig_int_escaleform_base_class_cs(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_036377_cpausemenu_isinmapscreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036378_cpausemenu_issp(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036379_cpausemenu_ismp(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036380_cpausemenu_islobby(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036381_cpausemenu_backoutofloadgamepanes(ph: *mut Phantom); }
extern "C" { pub fn f_036382_cprofilesettings_exists_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036384_cpausemenu_getofflinereasonaslockey(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_036388_cpausemenu_closeandwarp(ph: *mut Phantom); }
extern "C" { pub fn f_036390_cpausemenu_entersocialclub(ph: *mut Phantom); }
extern "C" { pub fn f_036393_cpausemenu_refreshsocialclubcontext(ph: *mut Phantom); }
extern "C" { pub fn f_036404_sgeneralpausedataconfig_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_036409_cspinnerdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_036412_carrowdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_036415_cgradient_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_036424_sgeneralpausedataconfig_sgeneralpausedataconfig(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036425_sgeneralpausedataconfig_dtorsgeneralpausedataconfig(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036429_basesocialclubmenu_basesocialclubmenu(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036430_basesocialclubmenu_dtorbasesocialclubmenu(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036435_basesocialclubmenu_setbuttons_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036436_cmenubuttonlist_add_cmenubutton_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036440_cprofilesettings_finishread_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036447_creportmenu_dtorcreportmenu_62550(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036448_creportmenu_open_rage_rlgamerhandle_const_ref_creportmenu_emenut(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_036449_creportmenu_populate_creportlist_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036450_creportmenu_updatemenuoptions_ereportlink(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036452_creportmenu_removereportmovie(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036453_creportmenu_settitle_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036455_creportmenu_updateconfirmationscreen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036456_creportmenu_updatethankyouscreen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036457_creportmenu_onacceptinput(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036459_cwarningscreen_render(ph: *mut Phantom); }
extern "C" { pub fn f_036463_creportlist_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_036470_cscaleformmenuhelper_set_column_title_pm_columns_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_036471_cscaleformmenuhelper_set_busy_spinner_pm_columns_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036472_cscaleformmenuhelper_show_column_pm_columns_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036473_cscaleformmenuhelper_set_column_scroll_pm_columns_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_036474_cscaleformmenuhelper_set_warning_message_visibility_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036475_sctour_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036476_sctourmanager_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036477_sctour_dtorsctour(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036479_basesocialclubmenu_setbusyspinner_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036480_socialclublegalsmenu_gotodownloading(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036481_socialclubmenu_setgenericerror_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_036482_socialclubmenu_gotostate_socialclubmenu_esocialclubstates(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036485_basesocialclubmenu_flagfordeletion(ph: *mut Phantom); }
extern "C" { pub fn f_036487_socialclubmenu_gotoerror_cwarningmessage_data_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036488_sctourmanager_dtorsctourmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036489_socialclubpolicymanager_dtorsocialclubpolicymanager(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036491_socialclubmenu_dtorsocialclubmenu_62632(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036492_socialclubmenu_gotowelcome(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036493_socialclubmenu_gotodob(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036494_socialclubmenu_gotoonlinepolicies(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036495_socialclubmenu_gotoeula(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036496_socialclubmenu_gotoprivacypolicy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036497_socialclubmenu_gototos(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036498_socialclubmenu_gotosignup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036499_socialclubmenu_gotosignin(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036500_socialclubmenu_gotoresetpassword(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036501_socialclubmenu_gotopolicyupdateaccepted(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036502_socialclubmenu_gotoscauthsignup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036503_socialclubmenu_gotoscauthsignupdone(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036504_socialclubmenu_gotoscauthsignin(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036505_socialclubmenu_gotoscauthsignindone(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036506_socialclubmenu_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036509_basesocialclubmenu_callmoviewithpagedtext_char_const_ptr_pagedcl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036510_cscaleformmgr_addparamstring_char_const_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036511_basesocialclubmenu_checkscroll(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036512_basesocialclubmenu_acceptpolicyversions_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036513_socialclubmenu_updatepolicyhighlight(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036514_socialclubmenu_selectcurrentoption(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036515_basesocialclubmenu_updateoption_int_ref_int_bool_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_036519_socialclublegalsmenu_gotoonlinepolicies(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036520_socialclublegalsmenu_gotodownloaderror(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036522_socialclublegalsmenu_selectcurrentoption(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036523_socialclublegalsmenu_updatepolicyhighlight(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036527_socialclubmenu_updatescauthsignup(ph: *mut Phantom); }
extern "C" { pub fn f_036528_socialclubmenu_updatescauthsignupdone(ph: *mut Phantom); }
extern "C" { pub fn f_036529_socialclubmenu_setwelcometab_signed_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036530_socialclubmenu_checkautofill(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036532_socialclubmenu_setgenericwarning_char_const_ptr_socialclubmenu_e(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_036535_socialclubmenu_setuppasswordkeyboard(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036536_socialclubmenu_suggestnickname_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036537_socialclubmenu_validatenickname_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036539_socialclubmenu_updatesigninsubmitstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036540_socialclubmenu_filloutemail_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_036541_socialclubmenu_filloutnickname_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_036542_socialclubmenu_filloutpassword_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036549_socialclubmenu_updatesignupsubmitstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036562_cstatsmenu_updatecategory_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_036565_cstatsmenu_setmenuceptableslot_int_emenuscreen_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_036566_cstatsmenu_updatemiscinfo_emenuscreen(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036572_rage_atarray_rage_atarray_rage_athashvalue_0_unsigned_short_0_un(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_036573_cuimenuped_playanim_rage_fwmvtemplateid_rage_fwmvclipidtype_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_036575_cuimenuped_animstatedata_update_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_036576_cuimenuped_setfadein_unsigned_int_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036577_cuimenuped_setpedmodel_unsigned_char_pm_columns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036578_uidatapagebase_uidatapagebase_uipaginatorbase_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_036579_uidatapagebase_fillscaleformbase_int_menuscreenid_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_036581_uidatapagebase_setselecteditem_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_036582_uipaginatorbase_uipaginatorbase_cmenuscreen_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_036583_uipaginatorbase_initinternal_int_menuscreenid_int_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_036584_uipaginatorbase_resetpages(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036585_uipaginatorbase_shutdownbase(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036586_uipaginatorbase_findpagecontaining_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_036587_uipaginatorbase_updateinput_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036590_uipaginatorbase_itemup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036591_uipaginatorbase_itemdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036592_uipaginatorbase_updateitemselection_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036593_uipaginatorbase_setactivepage_uidatapagebase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036594_uipaginatorbase_showbusy_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036595_uipaginatorbase_losefocus(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036596_uipaginatorbase_update_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036601_cpausevideoeditormenu_posteditorstory_cnetworkscnewsstoryrequest(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_036603_cvideoeditorui_open_eopenedfromsource(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036604_ctexttemplate_openmovie(ph: *mut Phantom); }
extern "C" { pub fn f_036605_cvideoeditorinstructionalbuttons_init(ph: *mut Phantom); }
extern "C" { pub fn f_036612_rage_atarray_cmenubuttonwithsound_0_unsigned_short_resizegrow_un(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036614_ctextinputbox_open_rage_iovirtualkeyboard_params_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036615_ctextinputbox_setinstructionalbuttons_unsigned_long_cscaleformmo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036616_ctextinputbox_removetextinputboxmovie(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036628_cwarningscreen_createmovieifnecessary(ph: *mut Phantom); }
extern "C" { pub fn f_036629_cwarningscreen_removemessageoptionitems(ph: *mut Phantom); }
extern "C" { pub fn f_036637_cwarningscreen_update(ph: *mut Phantom); }
extern "C" { pub fn f_036638_cwarningscreen_setinstructionalbuttons_unsigned_long_cscaleformm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036639_cscaleformmgr_updateatendofframe_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036641_cscaleformmgr_updateallmoviesstatemachines(ph: *mut Phantom); }
extern "C" { pub fn f_036642_cscaleformcomplexobjectmgr_syncbuffers(ph: *mut Phantom); }
extern "C" { pub fn f_036644_cuireticule_spreviousframehudvalues_resetdisplay(ph: *mut Phantom); }
extern "C" { pub fn f_036646_cuireticule_update_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036647_cuireticule_drawblinders_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036648_cuireticule_transformreticuleposition_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_036649_cuireticule_isspherevisible_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_036650_cuireticule_showhitmarker_cuireticule_ereticlehitmarkerstyle(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036651_cuireticule_showhitmarkernonlethal_cuireticule_ereticlehitmarker(ph: *mut Phantom); }
extern "C" { pub fn f_036654_uiworldicon_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036656_uiworldicon_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036658_uiworldiconmanager_uiworldiconmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036660_uiworldiconmanager_find_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_036661_uiworldiconmanager_add_centity_ptr_char_const_ptr_rage_scrthread(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_036662_uiworldiconmanager_add_rage_vector3_const_ref_char_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_036666_cstoretexturemanager_releasetexture_rage_atstring_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036667_cscaleformmgr_isreturnvalueset_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036668_cscaleformmgr_getreturnvalueint_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036669_cscaleformmgr_clearreturnvalue_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036670_cstoretexturemanager_requesttexture_rage_atstring_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036671_cscaleformmgr_endmethodreturnvalue_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036673_bool_rage_atmap_rage_atstring_cstoretexturemanager_cstoretexture(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036674_cstoremainscreen_displayitemdetails_rage_ccommerceproductdata_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036675_cstoremainscreen_updatecontexthelp(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036676_cstoremainscreen_triggerhighlightchange(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036677_cstoremainscreen_updatetopofscreentextinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036678_cstoremainscreen_startcheckout_rage_ccommerceproductdata_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036679_cstoremainscreen_populateitemslotwithdata_int_rage_ccommerceitem(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_036680_metricbaseinfo_metricbaseinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036681_metricbasechar_metricbasechar(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036682_cstoremainscreen_processcheckoutselected_rage_ccommerceproductda(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_036687_cstoretexturemanager_dtorcstoretexturemanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036693_cstoretexturemanager_updategarbage(ph: *mut Phantom); }
extern "C" { pub fn f_036705_cstoretexturemanager_freealltextures(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036707_cstorescreenmgr_populatedisplayvalues(ph: *mut Phantom); }
extern "C" { pub fn f_036714_cpausestoremenu_updatestorecolumnscroll_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036715_cpausestoremenu_getstorestate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036727_cvideoprojectplaybackcontroller_jumptostart(ph: *mut Phantom); }
extern "C" { pub fn f_036728_cvideoeditorproject_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_036729_cfileviewfilter_update_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036731_cfileviewfilter_refresh_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036733_cvideoeditorinterface_deactivate(ph: *mut Phantom); }
extern "C" { pub fn f_036734_cvideoeditorinterface_cleanupactiveprojectinternal(ph: *mut Phantom); }
extern "C" { pub fn f_036735_cimposedimagehandler_cleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036736_cvideoeditorinterface_grabrawclipfileview_char_const_ptr_crawcli(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036738_cvideoeditorinterface_releaserawclipfileview_crawclipfileview_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036739_cvideoeditorinterface_grabvideoprojectfileview_char_const_ptr_cv(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036741_cvideoeditorinterface_releasevideoprojectfileview_cvideoprojectf(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036742_cvideoeditorinterface_hasreachedmaxprojectlimit_cvideoprojectfil(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036743_cfileviewfilter_doesfileexistinprovider_char_const_ptr_bool_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_036744_cvideoeditorinterface_grabnewproject_char_const_ptr_cvideoeditor(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_036746_cvideoprojectplaybackcontroller_getplaybackcurrentnondilatedtime(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_036747_cvideoeditorinterface_autogenerateandbakeproject_creplayeditorpa(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036748_cvideoeditorproject_saveproject_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_036749_cvideoeditorinterface_pausebake(ph: *mut Phantom); }
extern "C" { pub fn f_036750_cvideoeditorinterface_resumebake(ph: *mut Phantom); }
extern "C" { pub fn f_036751_cvideoeditorinterface_killplaybackorbake_bool(ph: *mut Phantom); }
extern "C" { pub fn f_036752_cvideoprojectplaybackcontroller_pause(ph: *mut Phantom); }
extern "C" { pub fn f_036753_cvideoeditorinterface_releaseproject_cvideoeditorproject_ptr_ref(ph: *mut Phantom); }
extern "C" { pub fn f_036754_cvideoeditorproject_releasestagingclipthumbnail(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036755_cvideoeditorproject_releaseclipthumbnail_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036761_cimposedimagehandler_gettexture_rage_atfinalhashstring_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036768_ctextoverlayhandle_getstarttimems_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036769_ctextoverlayhandle_getdurationms_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036770_ctextoverlayhandle_gettext_unsigned_int_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036771_ctextoverlayhandle_gettextstyle_unsigned_int_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036772_ctextoverlayhandle_gettextposition_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036773_ctextoverlayhandle_gettextscale_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036774_ctextoverlayhandle_gettextcolor_unsigned_int_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036777_cmusiccliphandle_getclipsoundhash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036778_cmusiccliphandle_gettrimmeddurationms_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036779_cmusiccliphandle_getstarttimewithoffsetms_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036782_cambientcliphandle_gettrimmeddurationms_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036783_cambientcliphandle_getstarttimewithoffsetms_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036784_cvideoeditorproject_requeststagingclipthumbnail(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036785_cvideoeditorproject_isstagingcliptexturerequested_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036786_rage_fwtemplatestring_256ul_3ul_sprintf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_036787_cvideoeditorproject_getanchortimems_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_036790_cvideoeditorproject_deleteclip_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036792_cvideoeditorproject_requestclipthumbnail_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036793_cvideoeditorproject_getcliptexturename_unsigned_int_rage_atstrin(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_036794_cvideoeditorproject_isclipthumbnailrequested_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036795_cvideoeditorproject_isclipthumbnailloaded_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036796_cvideoeditorproject_hasclipthumbnailloadfailed_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036797_cvideoeditorproject_getclipthumbnailtexture_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_036798_cvideoprojectplaybackcontroller_setcursorspeed_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_036799_cvideoprojectplaybackcontroller_getplaybackendtimems_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_036800_cvideoprojectplaybackcontroller_getplaybackcurrenttimems_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_036801_cvideoprojectplaybackcontroller_jumptopreviousclip(ph: *mut Phantom); }
extern "C" { pub fn f_036802_cvideoprojectplaybackcontroller_jumptonondilatedtimems_float_uns(ph: *mut Phantom, a0: f32, a1: i32) -> i32; }
extern "C" { pub fn f_036803_cvideoprojectplaybackcontroller_applyrewind(ph: *mut Phantom); }
extern "C" { pub fn f_036804_cvideoprojectplaybackcontroller_isatendofplayback_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036805_cvideoprojectplaybackcontroller_jumptoend(ph: *mut Phantom); }
extern "C" { pub fn f_036806_cvideoprojectplaybackcontroller_jumptonextclip(ph: *mut Phantom); }
extern "C" { pub fn f_036807_cvideoprojectplaybackcontroller_toggleplayorpause(ph: *mut Phantom); }
extern "C" { pub fn f_036812_cfiledataprovider_doesfileexist_char_const_ptr_char_const_ptr_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_036827_cthumbnailfiledataprovider_getthumbnailname_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_036837_cdebugvideoeditorscreen_close(ph: *mut Phantom); }
extern "C" { pub fn f_036838_cdebugvideoeditorscreen_setstate_cdebugvideoeditorscreen_estate(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036841_cuigadgetfileselector_cleanuppreviewpane(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036842_cuigadgetfileselector_setfileview_cthumbnailfileview_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036843_cuigadgetlist_setcell_unsigned_int_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036846_cuigadgetfileselector_cuigadgetfileselector_float_float_cimposed(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32) -> i64; }
extern "C" { pub fn f_036848_cuigadgetfileselector_dtorcuigadgetfileselector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036851_cfileviewfilter_doesfileexist_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036857_cfileviewfilter_getfiledatestring_int_char_ref_256_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_036858_cuigadgetfullscreenoverlay_dtorcuigadgetfullscreenoverlay(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036878_cuigadgetvideoeditorpane_dtorcuigadgetvideoeditorpane(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036888_cfileviewfilter_deletemarkedfiles(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036890_cfileviewfilter_startdeletefile_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036911_cthumbnailfileview_getthumbnailname_int_char_ref_256_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_036917_cvideoeditormenu_refreshprojectheading(ph: *mut Phantom); }
extern "C" { pub fn f_036918_cvideoeditorui_saveproject_bool(ph: *mut Phantom); }
extern "C" { pub fn f_036919_cvideoeditormenu_jumptomenu_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036920_cvideoeditormenu_clearpreviewdata_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036921_cvideoeditorui_leaveeditingmode(ph: *mut Phantom); }
extern "C" { pub fn f_036922_cvideoeditormenu_buildmenu_int_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_036923_cvideoeditormenu_setitemselected_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036924_cvideoeditorui_displaysaveprojectresultprompt_unsigned_long(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036925_cvideoeditormenu_exittomainmenu(ph: *mut Phantom); }
extern "C" { pub fn f_036926_cvideoeditormenu_rebuildmenu(ph: *mut Phantom); }
extern "C" { pub fn f_036927_cvideoeditormenu_setuserconfirmationscreenwithaction_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_036928_rage_fwtemplatestring_128ul_3ul_sprintf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_036929_cvideoeditorproject_canaddclip_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036930_cvideoeditorui_close_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036931_cvideoeditorplayback_close(ph: *mut Phantom); }
extern "C" { pub fn f_036933_cvideoeditorui_closeinternal(ph: *mut Phantom); }
extern "C" { pub fn f_036934_cvideoeditortimeline_rebuildtimeline(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036935_cvideoeditortimeline_rebuildanchors(ph: *mut Phantom); }
extern "C" { pub fn f_036936_cvideoeditortimeline_droppreviewclipatposition_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036937_cvideoeditormenu_checkforuserconfirmationscreens(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036938_cvideoeditorui_seterrorstate_char_const_ptr_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_036939_rage_fwtemplatestring_288ul_3ul_trim(ph: *mut Phantom); }
extern "C" { pub fn f_036940_rage_fwtemplatestring_128ul_3ul_set_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036941_rage_fwtemplatestring_288ul_3ul_set_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036942_ctexttemplate_init(ph: *mut Phantom); }
extern "C" { pub fn f_036943_cvideoeditorui_isinblockingoperation_bool_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036944_cvideoeditorui_updateinput(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036945_cvideoeditormenu_updatepane(ph: *mut Phantom); }
extern "C" { pub fn f_036946_cvideoeditormenu_setuserconfirmationscreen_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_036947_cvideoeditorui_showtimeline(ph: *mut Phantom); }
extern "C" { pub fn f_036948_cvideoeditortimeline_clearallclips(ph: *mut Phantom); }
extern "C" { pub fn f_036949_cvideoeditormenu_jumppaneonaction(ph: *mut Phantom); }
extern "C" { pub fn f_036950_ctexttemplate_updatetemplate_int_seditedtextproperties_rage_atno(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_036951_cvideoeditorinstructionalbuttons_update(ph: *mut Phantom); }
extern "C" { pub fn f_036952_cvideoeditorinstructionalbuttons_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_036953_ctexttemplate_unloadtemplate(ph: *mut Phantom); }
extern "C" { pub fn f_036954_watermarkrenderer_cleanupplayback(ph: *mut Phantom); }
extern "C" { pub fn f_036955_cvideoeditormenu_isonaudiomenu(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036956_cvideoeditormenu_showanimatedaudioicon_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036957_cvideoeditormenu_gobackonepane_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036958_cvideoeditormenu_releasethumbnailofitem(ph: *mut Phantom); }
extern "C" { pub fn f_036959_rage_atarray_cvideoeditorpreviewdata_0_unsigned_short_copy_cvide(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_036960_cvideoeditormenu_gotoitem_int_int_cvideoeditormenu_egotorebuildb(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_036961_cvideoeditortimeline_addambient_unsigned_int_cambientcliphandle(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_036962_cvideoeditortimeline_addaudio_unsigned_int_cmusiccliphandle_cons(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_036963_cvideoeditortimeline_addtext_unsigned_int_ctextoverlayhandle_con(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_036964_cvideoeditortimeline_settimeonplayhead_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036965_cvideoeditortimeline_setplayheadtostartofnearestvalidclip(ph: *mut Phantom); }
extern "C" { pub fn f_036966_cvideoeditortimeline_addvideo_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036967_cvideoeditortimeline_getplayheadpositionms(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036968_cvideoeditortimeline_getprojectindexneareststartpos_eplayhead_ty(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_036969_cvideoeditortimeline_setplayheadpositionms_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036970_cvideoeditortimeline_updateplayheadpreview_eplayhead_type(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036971_cvideoeditortimeline_hideplayheadpreview(ph: *mut Phantom); }
extern "C" { pub fn f_036972_cvideoeditortimeline_placeambientontotimeline_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036973_cvideoeditortimeline_canpreviewclipbedroppedatposition_unsigned(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036974_cvideoeditortimeline_gettotaltimelinetimems_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036975_cvideoeditortimeline_placeaudioontotimeline_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036976_cvideoeditortimeline_placeeditedtextontotimeline_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036977_cscaleformcomplexobjectmgr_attachorgetobjectonstage_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036978_cvideoeditortimeline_adjustplayhead_float_bool(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_036979_cvideoeditormenu_getfileviewfilterformenu_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_036980_cvideoeditormenu_ismultidelete(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036981_cvideoeditorui_triggereditclip(ph: *mut Phantom); }
extern "C" { pub fn f_036982_cvideoeditormenu_getfileviewindexforitem_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_036983_cvideoeditorui_checkexportduration(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036984_cvideoeditorui_checkexportlimits(ph: *mut Phantom); }
extern "C" { pub fn f_036985_cvideoeditortimeline_deletetextclipfromtimeline_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036986_cvideoeditorui_triggeraddtexttotimeline(ph: *mut Phantom); }
extern "C" { pub fn f_036987_cvideoeditorinstructionalbuttons_refresh(ph: *mut Phantom); }
extern "C" { pub fn f_036988_ctexttemplate_setuptemplate_seditedtextproperties(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036989_ccomplexobject_attachmovieclip_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_036990_ccomplexobjectarrayitem_setpositioninpixels_rage_vector2_const_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036991_ccomplexobject_createemptymovieclip_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_036992_ccomplexobjectarrayitem_setalpha_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036993_ccomplexobject_attachmovieclipinstance_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_036994_ccomplexobjectarrayitem_setxpositioninpixels_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_036995_cvideoeditortimeline_settimelineselected_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036996_cvideoeditortimeline_clearprojectspaceindicator(ph: *mut Phantom); }
extern "C" { pub fn f_036997_rage_atarray_cvideoeditorpreviewdata_0_unsigned_short_destruct_c(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036998_cvideoeditorui_setmoviemode_eme_movie_mode(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036999_cvideoeditormenu_generatemenuid_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_037000_cvideoeditorplayback_checkexportstatus(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037001_cvideoeditorplayback_setmenuvisible_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037002_cvideoeditorplayback_addneweditmarker(ph: *mut Phantom); }
extern "C" { pub fn f_037003_ccomplexobjectarrayitem_gotoframe_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_037004_cvideoeditorplayback_updateaddmarkerbuttonstate(ph: *mut Phantom); }
extern "C" { pub fn f_037005_cvideoeditorplayback_updatefocusedmarker_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_037006_cvideoeditorplayback_updatespinnerplaybackprogress(ph: *mut Phantom); }
extern "C" { pub fn f_037007_cvideoeditortimeline_getstarttimeofclipms_eplayhead_type_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_037008_cvideoeditormenu_turnoffmenu_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037009_cvideoeditormenu_jumppane(ph: *mut Phantom); }
extern "C" { pub fn f_037011_cvideoeditorplayback_open_eplayback_type_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037012_ctexttemplate_updatetemplatewindow(ph: *mut Phantom); }
extern "C" { pub fn f_037013_cvideoeditorplayback_removecolumn_int(ph: *mut Phantom); }
extern "C" { pub fn f_037014_cvideoeditortimeline_setitemselected_eplayhead_type(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037015_ctexttemplate_defaulttext(ph: *mut Phantom); }
extern "C" { pub fn f_037016_cvideoeditorui_edittextduration_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037018_cvideoeditortimeline_getprojectindexbetweentime_eplayhead_type_u(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_037019_cvideoeditorui_pickupselectedclip_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037020_cvideoeditorthumbnailmanager_releasethumbnail_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037021_cvideoeditortimeline_deletevideoclipfromtimeline_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037022_cvideoeditortimeline_deleteambientclipfromtimeline_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037023_cvideoeditortimeline_deletemusicclipfromtimeline_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037024_cvideoeditortimeline_clearstageclip_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037025_cvideoeditortimeline_highlightclip_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037026_ccomplexobjectarrayitem_adjustxpositioninpixels_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_037027_cvideoeditortimeline_deleteclip_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037029_cvideoeditortimeline_updateoverviewbar(ph: *mut Phantom); }
extern "C" { pub fn f_037030_cvideoeditorui_showoptionsmenu(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037031_cvideoeditormenu_setitemactive_int(ph: *mut Phantom); }
extern "C" { pub fn f_037032_cvideoeditortimeline_repositionplayheadifneeded_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037033_cvideoeditormenu_canhighlightitem_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_037034_cvideoeditormenu_gotonextitem(ph: *mut Phantom); }
extern "C" { pub fn f_037035_cvideoeditormenu_isfilefilteringmenu_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037037_cvideoeditormenu_removecolumn_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037038_cvideoeditormenu_isonclipmanagementmenu(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037039_cvideoeditormenu_greyoutcurrentcolumn_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037040_cvideoeditormenu_actioninputup(ph: *mut Phantom); }
extern "C" { pub fn f_037041_cvideoeditormenu_actioninputdown(ph: *mut Phantom); }
extern "C" { pub fn f_037042_cvideoeditormenu_canitembeactioned_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_037043_cvideoeditormenu_isonoptionsmenu(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037044_cvideoeditormenu_isonmainmenu(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037045_cvideoeditormenu_isonaddclipmenu(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037046_cvideoeditormenu_isonprojectmenu(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037047_cvideoeditormenu_isongallerymenu(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037048_cvideoeditortimeline_getcolourforplayheadtype_unsigned_int_eplay(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_037049_ccomplexobjectarrayitem_setcolour_rage_color32_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037050_cvideoeditorinstructionalbuttons_clear(ph: *mut Phantom); }
extern "C" { pub fn f_037051_cvideoeditormenu_setcolumnactive_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037052_cvideoeditorplayback_cleanuploadingscreen_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037055_cvideoeditortimeline_getendtimeofclipms_eplayhead_type_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_037057_cscaleformmgr_gathermemorystats_int_scaleformmemorystats_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_037059_ccomplexobject_getwidth(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_037060_cvideoeditorplayback_jumptocurrenteditmarker(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037061_cvideoeditorplayback_refresheditmarkermenu(ph: *mut Phantom); }
extern "C" { pub fn f_037065_cvideoeditormenu_isitemselectable_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_037066_cvideoeditormenu_executedependentaction_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037067_ccomplexobject_setalpha_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037090_cvideoeditormenu_adjusttogglevalue_int_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_037093_cvideoeditormenu_gotopreviousitem(ph: *mut Phantom); }
extern "C" { pub fn f_037095_cvideoeditormenu_isclipfilteringmenu_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037096_cvideoeditormenu_checkforclips(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037097_cvideoeditormenu_saveclipfavourites(ph: *mut Phantom); }
extern "C" { pub fn f_037098_cvideoeditormenu_updateitemcolouration_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_037101_cvideoeditormenu_generatelistfootertext_rage_fwtemplatestring_32(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037103_cvideoeditormenu_beginaddingcolumndata_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037104_cvideoeditormenu_endaddingcolumndata_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037105_cvideoeditormenu_checkfordependentdata_int_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037112_ccomplexobject_setposition_rage_vector2_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037113_ccomplexobject_setpositioninpixels_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037114_ccomplexobject_setxpositioninpixels_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_037116_cvideoeditortimeline_setupstagepreviewclip_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037118_ccomplexobjectarrayitem_setwidth_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_037119_ccomplexobject_setwidth_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_037121_cscaleformmgr_translatewindowcoordinatetoscaleformmoviecoordinat(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_037122_cvideoeditortimeline_ismousewithintimelinearea(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037123_cvideoeditortimeline_getmousepositionms(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037124_cvideoeditortimeline_getdurationofclipms_eplayhead_type_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_037125_ccomplexobject_gotoframe_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037126_cvideoeditortimeline_getmousemovementdirintimelinearea(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037127_cvideoeditortimeline_trimobject_eplayhead_type_unsigned_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_037128_cvideoeditortimeline_adjusttonearestsnappoint_unsigned_int_int_b(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_037129_cvideoeditortimeline_isplayheadaudioscore_unsigned_int_eplayhead(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_037130_cvideoeditortimeline_setplayheadtostartofprojectclip_eplayhead_t(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037131_cvideoeditortimeline_getposofobject_int_char_const_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_037132_ccomplexobjectarrayitem_setypositioninpixels_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_037133_rage_atarray_stimelineitem_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037134_ccomplexobject_isvisible(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037136_cvideoeditorthumbnailmanager_removetextureoncomplexobject_unsign(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037137_ccomplexobjectarrayitem_setheight_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_037156_cvideoeditorplayback_cleareditflags(ph: *mut Phantom); }
extern "C" { pub fn f_037157_cvideoeditorplayback_opencurrenteditmarkermenu(ph: *mut Phantom); }
extern "C" { pub fn f_037159_cvideoeditorplayback_cleanupmarkersonplaybar(ph: *mut Phantom); }
extern "C" { pub fn f_037160_cvideoeditorplayback_setplaybackbuttonvisibility_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037161_cvideoeditorplayback_updatetoplevelbackoutinput(ph: *mut Phantom); }
extern "C" { pub fn f_037162_cvideoeditorplayback_addmarkertoplaybar_char_const_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_037163_cvideoeditorplayback_updaterepeatedinputmarkertime_float(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_037164_cvideoeditorplayback_canskipmarkertobeat_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037165_cvideoeditorplayback_trygetmenuoptionandindex_cvideoeditorplayba(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_037166_cvideoeditorplayback_getmarkerstorage(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037167_cvideoeditorplayback_updateitemtimevalue_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037168_cvideoeditorplayback_canscrub(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037169_cvideoeditorplayback_closecurrentmenu(ph: *mut Phantom); }
extern "C" { pub fn f_037170_cvideoeditorplayback_getfinescrubbingincrement_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_037171_cvideoeditorplayback_candeletemarkeratcurrenttime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037172_cvideoeditorplayback_stopscrubbing(ph: *mut Phantom); }
extern "C" { pub fn f_037173_cvideoeditorplayback_getclosesteditmarker(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037174_cvideoeditorplayback_getnearesteditmarkerindex_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037175_cvideoeditorplayback_canaddmarkeratcurrenttime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037176_cvideoeditorplayback_exitpostfxmenu(ph: *mut Phantom); }
extern "C" { pub fn f_037177_cvideoeditorplayback_backouteditmarkermenu(ph: *mut Phantom); }
extern "C" { pub fn f_037178_cvideoeditorplayback_getnavinputcheckflags(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037179_cvideoeditorplayback_updatefocusedinput_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037180_cvideoeditorplayback_seteditmarkerstate_ccomplexobject_ref_cvide(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_037181_cvideoeditorplayback_opencamerashutters(ph: *mut Phantom); }
extern "C" { pub fn f_037182_cvideoeditorplayback_begincamerashuttereffect_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037183_cvideoeditorplayback_enableeditmarkermenu_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037190_cvideoeditorplayback_cleanuptimedilationelement_ccomplexobject_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037191_cvideoeditorplayback_updatefocusedinputeditmenu_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037192_cvideoeditorplayback_refreshmarkerpositionsonplaybar(ph: *mut Phantom); }
extern "C" { pub fn f_037194_cvideoeditorplayback_settransitionelementvisible_ccomplexobject(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037195_cvideoeditorplayback_updatecameraoverlay(ph: *mut Phantom); }
extern "C" { pub fn f_037196_cvideoeditorplayback_exitcameditmode(ph: *mut Phantom); }
extern "C" { pub fn f_037197_cvideoeditorplayback_updaterepeatedinputmarkers_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_037238_cvideoeditorthumbnailmanager_getfirstclipvisibility_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_037243_cuireplayscaleformcontroller_update(ph: *mut Phantom); }
extern "C" { pub fn f_037248_cscaleformmgr_beginmethod_int_escaleform_base_class_char_const_p(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_037257_cscaleformmgr_addparamgfxvalue_gfxvalue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037258_cscaleformmoviewrapper_setdisplayconfig_cscaleformmgr_sdc(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037259_cscaleformmgr_createmovieandwaitforload_char_const_ptr_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_037260_cscaleformmgr_hasmoviegotanystate_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037261_cscaleformmgr_addparamlocstring_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037262_cscaleformmoviewrapper_cscaleformmoviewrapper_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_037263_cscaleformmoviewrapper_dtorcscaleformmoviewrapper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037264_cscaleformmgr_destroymovienow_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037268_printtxdref_char_const_ptr_rage_strlocalindex_int_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_037281_ccomplexobjectarrayitem_dtorccomplexobjectarrayitem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037283_cscaleformmgr_ismovieshuttingdown_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037285_cscaleformmgr_getrequiredscalemode_rage_vector2_const_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_037286_cscaleformmgr_getrequiredscalemode_int_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037290_cscaleformmgr_createmovie_cscaleformmgr_createmovieparams_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037291_cscaleformmgr_createmovieandwaitforload_cscaleformmgr_createmovi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037293_rage_fwassetstore_cscaleformmovieobject_cscaleformdef_addslot_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_037294_cscaleformstore_loadfilecore_rage_strlocalindex_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_037295_cscaleformstore_createmovieview_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037296_cscaleformmgr_initialiseandsetactive_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037297_cscaleformmgr_doesmovieexistinimage_char_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037299_cscaleformmgr_debugturnonoffoutput_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037300_cscaleformmgr_getmovieview_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_037304_cscaleformmgr_doesmoviehaveinvokesorcallbackspending_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037305_cscaleformmgr_restartmovie_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_037306_cscaleformmgr_printpreallocinfo_int_int_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_037307_rage_atarray_scomplectobjectmembers_0_unsigned_short_copyfrom_ra(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037308_cscaleformmgr_executecallbackmethod_int_char_const_ptr_gfxvalue(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_037312_cscaleformmgr_callmethodinternal_int_int_escaleform_base_class_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32); }
extern "C" { pub fn f_037315_cscaleformmgr_updatemovieparams_int_gfxmovieview_scalemodetype(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037316_cscaleformmgr_getactionscriptobjectfromroot_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037317_cscaleformmgr_callmethod_int_escaleform_base_class_char_const_pt(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_037319_cscaleformmgr_addtxdref_int_char_const_ptr_char_const_ptr_bool_b(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_037320_cscaleformmgr_addparamformattedint_long_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037321_cscaleformmgr_addparamint64_long(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037324_cscaleformmgr_lockmovie_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037325_cscaleformmgr_unlockmovie_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037341_cscaleformmgr_passdebugmethodparams_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037360_ccomplexobjectarrayitem_getstagerootinternal_ccomplexobjectarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037361_ccomplexobjectarrayitem_getobjectinternal_ccomplexobjectarrayite(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_037364_ccomplexobjectarrayitem_initproperties(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037383_gametransactionsessionmgr_gametransactionsessionmgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037409_cscriptedcoronaplayercarddatamanager_dtorcscriptedcoronaplayerca(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037417_cdisplaycalibration_render(ph: *mut Phantom); }
extern "C" { pub fn f_037418_cdisplaycalibration_updateinput(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037419_cdisplaycalibration_loadcalibrationmovie_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037421_cdisplaycalibration_removecalibrationmovie(ph: *mut Phantom); }
extern "C" { pub fn f_037435_cfrontendstatsmgr_getdataasstring_sstatdataptr_const_ptr_suistat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_037437_cfrontendstatsmgr_getreplacementlocalizedhash_unsigned_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_037438_cfrontendstatsmgr_shouldusemetric(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037439_cfrontendstatsmgr_formatint64tocash_long_char_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_037440_cfrontendstatsmgr_getvisibilityfromrule_suistatdata_const_ptr_ss(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_037441_cfrontendstatsmgr_spstatsuiconstiterend(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037442_cfrontendstatsmgr_mpstatsuiconstiterend(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037443_cfrontendstatsmgr_categoryconstiterend(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037487_cgamestream_postgametip_int_cgamestream_egamestreamtiptype(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_037490_cgamestream_forcerenderon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037491_cgamestream_forcerenderoff(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037492_cgamestream_pause(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037493_cgamestream_resume(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037494_cgamestream_hideforupdates_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037497_cgamestream_autopostgametipon_cgamestream_egamestreamtiptype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037498_cgamestream_autopostgametipoff(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037501_cgamestream_flushqueue_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037503_cgamestream_forceflushqueue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037521_cgamestream_setisonloadingscreen_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037524_cgamestream_isscripthidingthisframe_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037541_cgamestreammgr_update(ph: *mut Phantom); }
extern "C" { pub fn f_037542_cgamestreammgr_render(ph: *mut Phantom); }
extern "C" { pub fn f_037659_cseatmanager_countpedsinseats_bool_bool_bool_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_037661_cmodelinfo_requestassets_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037662_cmodelinfo_haveassetsloaded_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037686_cclock_getdayofweektextid(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037809_cplacename_getformap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> i64; }
extern "C" { pub fn f_037811_cplacename_display(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037812_cplacename_forcesettodisplay(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037813_cplacename_isnewname(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037814_ccurrentvehicle_isnewname(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037815_ccurrentvehicle_display(ph: *mut Phantom); }
extern "C" { pub fn f_037816_cstreetname_isnewname(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037894_cwantedblips_createandupdateradarblips_cwanted_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037900_cdispatchdata_gethiddenevasiontime_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037901_cwanted_copsaresearching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038751_cseatmanager_getpedsseatindex_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041056_findplayerped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041064_cpedintelligence_isthreatenedby_cped_const_ref_bool_bool_bool_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_041075_cpedtargetting_cpedtargetting_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041085_cpedtargetting_findtarget_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041088_cpedintelligence_getnearbyvehicles(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041089_cplayerinfo_getstealthmultiplier_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041090_cpedperception_calculateviewmatrix_rage_matrix34_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041116_cpedintelligence_flushimmediately_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041218_cplayerinfo_getphysicalplayerindex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041228_cpedgeometryanalyser_ispedinwateratdepth_cped_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_041296_cpedfactory_destroyped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041338_cpedhelmetcomponent_haspilothelmetequippedinaircraftinfps_bool_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041460_cpedplacement_findzcoorforped_float_rage_vector3_ptr_centity_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_041496_findplayervehicle_cplayerinfo_ptr_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041601_cplayerspecialability_getremainingpercentagefordisplay_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041717_cplayerinfo_isflyingaircraft(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041727_cplayerinfo_reportstealthnoise_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_041933_cplayerpedtargeting_getonfoothominglockonstate_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041998_cqueriableinterface_gettargetfortasktype_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042004_cqueriableinterface_getstatefortasktype_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042232_pedheadshotmanager_registerped_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_042234_pedheadshotmanager_unregisterped_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042235_pedheadshotmanager_gettexturename_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042237_pedheadshotmanager_isactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042818_worldprobe_cshapetestmanager_debugsubmittest_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_042821_worldprobe_cshapetestmanager_aborttest_worldprobe_cshapetestresu(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043085_worldprobe_cshapetestdesc_setexcludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043224_cphysics_loadaboutposition_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043335_worldprobe_debugfindgroundzforcoord_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_043458_meshblendmanager_renderthreadupdate(ph: *mut Phantom); }
extern "C" { pub fn f_043480_crendertargets_createrendertarget_rtmempoolid_char_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_043505_occlusionqueries_oqallocate(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043506_occlusionqueries_oqfree_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043507_occlusionqueries_oqsetboundingbox_unsigned_int_rage_vec3v_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_043508_occlusionqueries_oqgetqueryresult_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_043621_registeredanimpostfxstack_registeredanimpostfxstack(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043649_postfx_update(ph: *mut Phantom); }
extern "C" { pub fn f_043666_postfx_dopauseresolve_postfx_eresolvepause(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043667_postfx_processnondepthfx(ph: *mut Phantom); }
extern "C" { pub fn f_043670_csprite2d_begincustomlist_csprite2d_customshadertype_rage_grctex(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_043674_crendertargets_getuibackbuffer_bool(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_043675_csprite2d_endcustomlist(ph: *mut Phantom); }
extern "C" { pub fn f_043677_animpostfxmanager_start_rage_atnonfinalhashstring_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_043713_animpostfxmanager_isrunning_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_043714_animpostfxmanager_stop_rage_atnonfinalhashstring_animpostfxmanag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043743_pausemenupostfxmanager_startfadein(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043744_pausemenupostfxmanager_startfadeout(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043745_pausemenupostfxmanager_isfading_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043753_csprite2d_setgeneralparams_rage_vector4_const_ref_rage_vector4_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043913_crendertargets_getuidepthbuffer_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_043916_crendertargets_resolvebuffer_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_043917_crendertargets_getoffscreenbuffer_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_043918_crendertargets_getoffscreenbuffer_resolved_unsigned_int(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_043921_crendertargets_lockuirendertargets(ph: *mut Phantom); }
extern "C" { pub fn f_043922_crendertargets_unlockuirendertargets_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043924_crendertargets_createrendertarget_char_const_ptr_id3d11resource(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043938_csprite2d_setrenderstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043956_csprite2d_clearrenderstate(ph: *mut Phantom); }
extern "C" { pub fn f_043961_cgtaoldloadingscreen_setloadingrenderfunction(ph: *mut Phantom); }
extern "C" { pub fn f_043962_cgtaoldloadingscreen_forceloadingrenderfunction_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043972_csprite2d_setmultifadeparams_rage_fwrect_const_ref_bool_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_043975_csprite2d_delete(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043976_csprite2d_settexture_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043977_csprite2d_setrenderstate_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043979_csprite2d_settexelsize_rage_vector2_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043980_csprite2d_settexelsize_rage_vector4_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043981_csprite2d_drawrect_rage_vector2_const_ref_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_043982_csprite2d_predrawshape(ph: *mut Phantom); }
extern "C" { pub fn f_043983_csprite2d_drawrect_float_float_float_float_float_rage_color32_co(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i64); }
extern "C" { pub fn f_043984_csprite2d_drawrectgui_rage_fwrect_const_ref_rage_color32_const_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043985_csprite2d_draw_rage_vector2_const_ref_rage_vector2_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_043986_csprite2d_draw_rage_vector2_const_ref_rage_vector2_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_043987_csprite2d_drawswitch_bool_bool_float_rage_vector2_const_ref_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64); }
extern "C" { pub fn f_043988_csprite2d_movetoscreengui_rage_vector2_ptr_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043989_csprite2d_moveposvectortoscreengui_rage_vector2_ref_rage_vector2(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043991_cuiworldiconquad_cuiworldiconquad(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043992_cuiworldiconquad_loadtexure(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043993_cuiworldiconquad_settexture_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_044125_water_testlineagainstwater_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_044258_ui3ddrawmanager_assignpedtoslot_rage_atnonfinalhashstring_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_044259_ui3ddrawmanager_assignpedtoslot_rage_atnonfinalhashstring_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: f32) -> i32; }
extern "C" { pub fn f_044260_ui3ddrawmanager_assigngloballightintensitytoslot_rage_atnonfinal(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_044261_ui3ddrawmanager_pushcurrentpreset_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_044262_ui3ddrawmanager_isusingpreset_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_044889_creplaycoordinator_isactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_044972_creplayaudiotrackprovider_cleartracksusedinproject(ph: *mut Phantom); }
extern "C" { pub fn f_044973_cmontage_getmusicclipsoundhash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_044974_creplayaudiotrackprovider_updatetracksusedinproject_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_044975_creplayaudiotrackprovider_removeinvalidtracksandupdateusage_cmon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044976_cmontage_deletemusicclip_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_044982_creplaycoordinator_ispendingpendingbakestart(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_044983_cclip_calculatetimefornondilatedtimems_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_044987_creplaycoordinator_deactivate(ph: *mut Phantom); }
extern "C" { pub fn f_044988_creplaycoordinator_killplaybackorbake_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044989_creplaycoordinator_activate(ph: *mut Phantom); }
extern "C" { pub fn f_044990_creplaycoordinator_ispreviewingvideo(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_044991_creplaycoordinator_isrenderingvideo(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_044992_creplaycoordinator_ispreviewingclip(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_044994_creplaycoordinator_isrenderingpaused(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_044995_creplaycoordinator_startautogeneratingproject_creplayeditorparam(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_044996_creplaycoordinator_shouldshowloadingscreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_044997_creplaycoordinator_hascliploadfailed(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_044998_creplaycoordinator_getindexofcurrentplaybackclip(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045000_creplaycoordinator_geteffectivecurrentcliptimems(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_045001_creplaycoordinator_playclippreview_unsigned_int_cmontage_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_045002_creplaycoordinator_playprojectpreview_cmontage_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045003_creplaycoordinator_startbakeproject_cmontage_ref_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045004_cancillaryreplayrenderer_disablereplaydefaulteffects(ph: *mut Phantom); }
extern "C" { pub fn f_045005_creplaycoordinator_ispendingcleanup(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045006_creplaycoordinator_isexportingtovideofile(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045019_cmontage_cmontage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045020_cmontage_dtorcmontage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045021_cmontage_create(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_045023_cmontage_destroy_cmontage_ptr_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045024_cmontage_setname_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045025_cclip_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045026_cclip_populate_char_const_ptr_unsigned_long_replaymarkertransiti(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_045027_cmontage_addclipcopy_cclip_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_045028_cclip_validatetimedata(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045029_cclip_getstartnondilatedtimems_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_045030_cclip_getendnondilatedtimems_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_045031_cclip_op_cclip_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045032_cmontage_getclip_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045042_cmontage_gettrimmedtimetoclipms_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_045064_cmontage_gettotalcliplengthtimems_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_045079_cclip_dtorcclip(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045082_cclip_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045127_cmontage_addclip_char_const_ptr_unsigned_long_int_replaymarkertr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_045128_cmontage_convertplaybackpercentagetoclipnondilatedtimems_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> f32; }
extern "C" { pub fn f_045129_cmontage_getrawendtimeastimems_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_045130_cmontage_isvalidclipreference_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045131_cmontage_isvalidclipfile_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_045132_cmontage_areallclipfilesvalid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045133_cmontage_deleteclip_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045134_cmontage_validatetransitionduration_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045135_cmontage_getendtransition_unsigned_int_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045138_cmontage_getmusicclip_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045141_cmontage_doesclipcontainmusicclipwithbeatmarkers_unsigned_int_co(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045142_cmontage_getambientclip_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045143_cmontage_deleteambientclip_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045144_cmontage_addtext_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_045145_cmontage_gettext_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045146_cmontage_gettextindex_cmontagetext_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_045147_cmontage_deletetext_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045148_cmontage_approximateminimumsizeforsaving(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_045149_cmontage_isspaceavailableforsaving_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045150_cmontage_save_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_045152_cmontage_load_char_const_ptr_unsigned_long_bool_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_045156_cmontagemusicclip_gettracktotaldurationms_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045157_cmontagetext_setline_unsigned_int_char_const_ptr_rage_vector2_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_045240_cqueueofphotooperations_push_cphotooperationdata_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045245_csavegamequeue_push_csavegamequeuedoperation_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045249_cgenericgamestorage_existsinqueue_csavegamequeuedoperation_ptr_b(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_045250_cgenericgamestorage_removefromsavegamequeue_csavegamequeuedopera(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045251_cgenericgamestorage_isstoragedevicebeingaccessed(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045285_csavingmessage_shoulddisplaysavingspinner(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045287_csavegamephotoasyncbuffer_csavegamephotoasyncbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045288_csavegamephotoasyncbuffer_dtorcsavegamephotoasyncbuffer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045318_cphotomanager_getcreatesortedlistofphotosstatus_bool_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045319_cphotomanager_gettakerockstareditorphotostatus(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045320_cphotomanager_getsaverockstareditorphotostatus(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045321_cphotomanager_requestcreatesortedlistofphotos_bool_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045323_cphotomanager_getnumberofphotos_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045384_csavegamequeuedoperation_manualload_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045388_csavegamequeuedoperation_enumeratereplayfiles_init_ereplayenumer(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_045391_csavegamequeuedoperation_replaymultidelete_init_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045400_csavegamequeuedoperation_loadreplayheader_init_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045789_cped_ismale_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045840_cped_getbonematrix_rage_matrix34_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045912_cped_isusingactionmode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045913_cped_isfirstpersonshootermodeenabledforplayer_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045926_cped_getisdrivingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045960_cped_hasheadblendfinished(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045961_cped_finalizeheadblend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045977_cped_setdesiredheading_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046001_cped_isusingstealthmode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046002_cped_isinfirstpersonvehiclecamera_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_046103_cped_getspecialability_eplayerspecialabilityslot(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046162_cped_setclothforcepin_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046170_cped_isallowedtodamageentity_cweaponinfo_const_ptr_centity_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_046173_cped_getmaxtimeunderwater_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_046176_cped_getcontrolfromplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046325_cgameworld_findfollowplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_046326_cgameworldheightmap_getmaxheightfromworldheightmap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_046327_cplayerswitchinterface_isactive_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_046376_ctexturedownloadrequestdesc_ctexturedownloadrequestdesc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046379_cdownloadabletexturemanager_requesttexturedownload_int_ref_ctext(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_046383_cdownloadabletexturemanager_releasetexturedownloadrequest_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046386_cdownloadabletexturemanager_isrequestready_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046387_cdownloadabletexturemanager_hasrequestfailed_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046388_cdownloadabletexturemanager_getrequesttxdslot_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046391_cdownloadabletexturemanager_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046410_centity_getinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046490_centity_isvisible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046702_cwarpmanager_setwarp_rage_vector3_const_ref_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: f32, a6: i32) -> i32; }
extern "C" { pub fn f_046733_cextracontentmanager_iscontentitemlocked_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047020_cgameworld_findlocalplayerwanted(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047063_cphysical_disablephysicsiffixed(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047073_cphysical_sethealth_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047305_cgameworld_getmainplayerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047355_csceneloader_loadscene_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047727_cinteriorinst_isallowrunmlo_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047880_cgameworld_findlocalplayercoors(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047982_cgameworld_findlocalplayervehicle_bool(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_048042_cgameworldheightmap_getminheightfromworldheightmap_float_float_f(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_048043_cgameworldheightmap_getmaxheightfromworldheightmap_float_float_f(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_054790_network_commands_commandnetworkisactivitysession(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_063738_gtathread_forcecleanup_unsigned_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063762_cthescripts_gtastartnewthreadoverride_char_const_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_063763_cthescripts_gtastartnewthreadwithprogramid_rage_scrprogramid_voi(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_063787_cthescripts_shouldbeprocessed(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_063800_cthescripts_getcurrentscriptnameandprogramcounter(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063835_cscriptglobaltamperchecker_uncheckedscope_uncheckedscope(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_063836_cscriptglobaltamperchecker_uncheckedscope_dtoruncheckedscope(ph: *mut Phantom); }
extern "C" { pub fn f_064025_cscripthud_getfakedpausemapplayerpos_rage_vector2_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064026_cscripthud_getfakedgpsplayerpos_rage_vector3_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064698_cshaderlib_setglobalalpha_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_064700_cshaderlib_setglobalemissivescale_float_bool(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_064706_cshaderlib_setglobaltonemapscalers_rage_vector4_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064710_cshaderlib_updateglobaldevscreensize(ph: *mut Phantom); }
extern "C" { pub fn f_064724_cshaderlib_setreticuledisttexture_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064725_cshaderlib_setstereotexture(ph: *mut Phantom); }
extern "C" { pub fn f_064726_cshaderlib_setstereoparams_rage_vector4_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064727_cshaderlib_setstereoparams1_rage_vector4_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064736_csettingsmanager_csettingsmanager(ph: *mut Phantom); }
extern "C" { pub fn f_064800_cstreaming_hasobjectloaded_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064832_cstreaming_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064913_ckeyboard_getkeyjustdown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_064932_cstreaming_setdonotdefrag_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064937_cstreaming_loadobject_rage_strlocalindex_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_065273_ckeyboard_getkeydown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_065289_ccompanionblipnode_update_scompanionblip_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065290_ccompaniondata_removeblip_cblipcomplex_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065293_ccompaniondata_getinstance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_065325_ccontrol_loaddynamiccontrolmappings(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065345_ccontrol_updatemapping_rage_inputtype_ccontrol_mappingtype_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_065346_ccontrol_setinitialdefaultmappings_bool_bool_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_065350_ccontrol_getusersettingspath_char_ref_260_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065356_ccontrol_getmousesteeringmode_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_065358_ccontrol_getinputsource_rage_inputtype_int_unsigned_char_bool_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_065360_ccontrol_resetkeyboardmousesettings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065362_ccontrolmgr_reinitcontrols(ph: *mut Phantom); }
extern "C" { pub fn f_065366_ccontrol_disableallinputs_rage_iovalue_disableoptions_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065368_ccontrol_setlighteffect_rage_iolighteffect_const_ptr_ccontrol_li(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_065369_ccontrol_clearlighteffect_rage_iolighteffect_const_ptr_ccontrol(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_065370_ccontrol_setlighteffectenabled_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_065373_ccontrol_disableallinputs_unsigned_int_rage_iovalue_disableoptio(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_065374_ccontrol_setinputexclusive_int_rage_iovalue_disableoptions_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_065375_ccontrol_doenableinput_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_065376_ccontrol_getinputname_rage_inputtype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065379_ccontrol_dodisableinput_unsigned_int_rage_iovalue_disableoptions(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_065382_ccontrol_disableinputsonbackbutton_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_065384_ccontrol_isinputdisabled_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_065385_ccontrol_getvalue_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_065416_ccontrol_saveusercontrols_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_065417_ccontrol_getconflictinginputs_rage_atarray_rage_inputtype_0_unsi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065422_ccontrol_getunmappedinputs_rage_atarray_rage_inputtype_0_unsigne(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065424_ccontrol_cancelcurrentmapping(ph: *mut Phantom); }
extern "C" { pub fn f_065427_ccontrol_getcategoryinputs_rage_atnonfinalhashstring_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065431_ccontrol_loadvideoeditorscriptcontrolmappings(ph: *mut Phantom); }
extern "C" { pub fn f_065432_ccontrol_doesusersmappingfileneedupdating_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065433_ccontrol_deleteautosavedusercontrols_int_const(ph: *mut Phantom); }
extern "C" { pub fn f_065434_ccontrol_scangamepaddefinitionsource_ccontrol_gamepaddefinitions(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_065435_ccontrol_cancelgampaddefinitionscan(ph: *mut Phantom); }
extern "C" { pub fn f_065436_ccontrol_isgampaddefinitionscaninprogress_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065437_ccontrol_endgamepaddefinitionscan_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_065444_ccontrolmgr_update(ph: *mut Phantom); }
extern "C" { pub fn f_065446_ccontrolmgr_getmainplayercontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065447_ccontrolmgr_setdefaultlookinversions(ph: *mut Phantom); }
extern "C" { pub fn f_065451_ccontrolmgr_stoppadsshaking(ph: *mut Phantom); }
extern "C" { pub fn f_065458_ccontrolmgr_updatevirtualkeyboard(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065461_ccontrolmgr_getmainfrontendcontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065469_ccontrolmgr_startplayerpadshakebyintensity_unsigned_int_float_in(ph: *mut Phantom, a0: i32, a1: f32, a2: i32); }
extern "C" { pub fn f_065470_ccontrolmgr_showvirtualkeyboard_rage_iovirtualkeyboard_params_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065471_ccontrolmgr_getvirtualkeyboardstate(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065615_cfilemgr_openfile_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_065616_cfilemgr_readline_void_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_065617_cfilemgr_setdir_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065621_cfilemgr_openfileforwriting_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065632_ckeyboard_setcurrentmode_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_065642_settingsdefaults_videomemsizefor_settings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065643_settingsdefaults_getinstance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_065644_csettingsmanager_getsafemodesettings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065668_csettingsmanager_requestnewsettings_settings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065673_csettingsmanager_getresolutionlist_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065675_csettingsmanager_convertfrommonitorindex_int_int_ref_int_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_065678_csettingsmanager_save_settings_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065679_csettingsmanager_reverttoprevioussettings(ph: *mut Phantom); }
extern "C" { pub fn f_065680_csettingsmanager_converttomonitorindex_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_065681_csettingsmanager_getnativeresolutionlist_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065682_csettingsmanager_getresolutionindex(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065683_csettingsmanager_getuisettings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065687_csettingsmanager_updateragdata(ph: *mut Phantom); }
extern "C" { pub fn f_065691_csystem_beginrender(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065693_csystem_endrender(ph: *mut Phantom); }
extern "C" { pub fn f_067955_ctaskmobilephone_isrunningmobilephonetask_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070096_ctaskmotionped_getmovenetworkhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075523_ctuning_dtorctuning(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075660_ctuning_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_075664_ctuning_registertuning_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_076865_ctaskgun_getisfiring_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076920_ctaskaimgunvehicledriveby_getpedweaponinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077089_cstatsmgr_incrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077090_cstatsmgr_decrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077120_cstatsmgr_getpercentageprogress(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_077150_statid_char_set_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077151_statid_char_gethash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077152_statid_char_gethash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077153_statid_char_getstatid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077192_cstatsdatamgr_getstatptr_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077193_cstatsdatamgr_iskeyvalid_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077202_cstatsdatamgr_getisbasetype_statid_const_ref_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077329_statid_set_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077330_statsinterface_getcharacterindex(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077331_statsinterface_getintstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077333_statsinterface_getkeyname_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077334_statsinterface_iskeyvalid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077338_statsinterface_getcurrentmultiplayercharaterslot(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077339_statsinterface_cloudfileloadpending_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077340_statsinterface_cloudfileloadfailed_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077342_statsinterface_setstatdata_statid_const_ref_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077344_statsinterface_setstatdata_statid_const_ref_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077347_statsinterface_setstatdata_statid_const_ref_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077353_statsinterface_trymultiplayersave_esavetypes_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_077354_statsinterface_getfloatstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_077355_statsinterface_getbooleanstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077358_statsinterface_getuint32stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077359_statsinterface_getuint64stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077360_statsinterface_getint64stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077370_statsinterface_isbadsport_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_077387_cstatsutils_gettime_unsigned_long_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077408_cstatssavesmgr_pendingsaverequests_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077416_cstatssavesmgr_issaveinprogress_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077430_moneyinterface_getvcwalletbalance_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077431_moneyinterface_getvcbankbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077432_moneyinterface_getevcbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077433_moneyinterface_getpvcbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077438_moneyinterface_getpxrvalue(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_077448_moneyinterface_canbuycash_long_float(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_077449_moneyinterface_savegamecanbuycashproducts(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077452_ccashproductsdata_skucashpackinfo_getskupackname_rage_atstring_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077458_moneyinterface_getcashpackvalue_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077475_cnumberwithinmessage_set_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077476_cnumberwithinmessage_set_float_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077478_csubstringwithinmessage_settextlabel_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077479_ctextfile_get_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077481_ctextfile_get_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_077482_csubstringwithinmessage_setcharpointer_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077483_csubstringwithinmessage_setliteralstring_char_const_ptr_csubstri(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_077486_csubtitlemessage_set_char_const_ptr_int_cnumberwithinmessage_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32); }
extern "C" { pub fn f_077488_cmessages_insertnumbersandsubstringsintostring_char_const_ptr_cn(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32); }
extern "C" { pub fn f_077491_cmessages_addtopreviousbriefarray_csubtitlemessage_ref_eprevious(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077495_chelpmessage_clearall(ph: *mut Phantom); }
extern "C" { pub fn f_077504_cmessages_addmessage_char_const_ptr_int_unsigned_int_bool_bool_e(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i64, a9: i32, a10: i32, a11: i32); }
extern "C" { pub fn f_077505_cmessages_fillstringwithpreviousmessage_cmessages_epreviousmessa(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_077507_chelpmessage_setmessagetext_int_char_const_ptr_cnumberwithinmess(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_077514_chelpmessage_clearmessage_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077515_chelpmessage_getmessagetext_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077517_chelpmessage_ishelptextnew_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077518_chelpmessage_ishelptextsame_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077519_chelpmessage_ishelptextforever_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077520_chelpmessage_getdirectionhelptext_int_rage_vector2(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_077522_csavinggamemessage_getmessagetext(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077523_csavinggamemessage_issavingmessageactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077525_cloadingtext_setactive_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077527_pagedcloudtext_pagedcloudtext(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077533_ctextlayout_ctextlayout(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077534_ctextlayout_default(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077535_ctextlayout_dtorctextlayout(ph: *mut Phantom); }
extern "C" { pub fn f_077537_ctextlayout_setscale_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077538_ctextlayout_setadjustfornonwidescreen_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077539_ctextlayout_getcharacterheight(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_077540_ctextlayout_render_rage_vector2_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077543_ctextlayout_getnumberoflines_rage_vector2_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_077545_ctextlayout_setscale_rage_vector2_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077549_ctext_flush(ph: *mut Phantom); }
extern "C" { pub fn f_077550_ctext_isasianlanguage(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077551_ctext_getsclanguagefromcurrentlanguagesetting(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077552_ctext_getstringofsupportedlanguagesfromcurrentlanguagesetting(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077557_ctextconversion_texttohtml_char_const_ptr_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_077560_ctextconversion_formatintforhumans_long_char_ptr_int_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_077561_ctextconversion_formatmstimeasstring_char_ptr_unsigned_long_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_077562_ctextconversion_formatmstimeaslongstring_char_ptr_unsigned_long(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077565_ctextfile_doestextlabelexist_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077570_ctextfile_getlanguagefromname_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077572_ctextfile_loadtextintoslot_int_char_const_ptr_bool_ctextfile_elo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_077574_ctextfile_reloadafterlanguagechange(ph: *mut Phantom); }
extern "C" { pub fn f_077575_ctextfile_requestadditionaltext_char_const_ptr_int_ctextfile_elo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077576_ctextfile_freetextslot_int_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077577_ctextfile_isrequestingadditionaltext_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077578_ctextfile_hasthisadditionaltextloaded_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077581_ctextfile_hasadditionaltextloaded_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077588_ctextformat_getinputbuttons_char_const_ptr_char_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_077589_ctextformat_getinputbuttons_rage_inputtype_char_ptr_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_077590_ctextformat_getinputgroupbuttons_char_const_ptr_char_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_077591_ctextformat_getinputgroupbuttons_rage_inputgroup_char_ptr_unsign(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_077593_ctextformat_doescharacterexistinfont_int_unsigned_short(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_077594_ctextformat_doallcharactersexistinfont_int_unsigned_short_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_077595_ctextformat_isvaliddisplaycharacter_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077596_ctextformat_isunsupportedspace_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077853_cpathfind_addnodesrequiredregionthisframe_int_rage_vector3_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_077889_cpathfind_recordnodesincircle_rage_vector3_const_ref_float_int_r(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_078909_cvehicle_istaximodelid_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079000_cvehicle_istank_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079130_cvehicle_getlayoutinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079422_cvehicle_getvehiclegadget_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079587_cvehicle_getfragmentcomponentindex_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079791_cvehicle_isdriver_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079872_cvehicle_getseathasweaponsorturrets_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079883_cvehiclelayoutinfo_getseatinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079953_cvehicleweaponmgr_getindexofvehicleweapon_cvehicleweapon_const_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_080666_cvehiclemetadatamgr_getseataniminfofromped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080888_cvfxhelper_shouldrenderingameui(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_080969_cmoviemgr_updateframe(ph: *mut Phantom); }
extern "C" { pub fn f_081417_cmarkers_register_markerinfo_t_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081514_cmoviemgr_getnummovies_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082064_cpedweaponmanager_getequippedvehicleweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082065_cpedweaponmanager_getequippedweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082070_cweaponinfo_getammoinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082071_cpedweaponmanager_getequippedweapon_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082182_cweapon_gettimebeforenextshot_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_082186_cweapon_getaccuracy_cped_const_ptr_float_sweaponaccuracy_ref_con(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_082211_cweaponinfo_getprojectilefusetime_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_082549_cweaponinfomanager_getslotlistnavigate_cped_const_ptr(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_082594_cpedweaponmanager_equipweapon_unsigned_int_int_bool_bool_cpedequ(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_082597_cpedweaponselector_getisweaponvalid_cped_const_ptr_cweaponinfo_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_082599_cpedweaponselector_selectweapon_cped_ptr_epedinventoryselectopti(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_082601_cpedweaponselector_getselectedvehicleweaponinfo_cped_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082643_cpedweaponmanager_getisgadgetequipped_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082646_cpedweaponmanager_getbestweaponhash_unsigned_int_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_082663_cammoitemrepository_getammo_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082666_cammoitemrepository_getisusinginfiniteammo_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082671_cweaponitem_getkeyfromhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082828_networkexitflow_startshutdowntasks(ph: *mut Phantom); }
extern "C" { pub fn f_082831_cnetworktelemetry_flushtelemetry_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082835_networkinterface_islocalplayersignedin(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082864_tunableslistener_tunableslistener_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082880_cnetwork_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082882_cnetwork_closenetwork_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082909_cnetworktelemetry_appendmetric_rage_rlmetric_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082912_networkinterface_isinsession(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082935_cnetwork_ismultiplayerlocked(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082938_networkinterface_islocalplayeronline_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082939_networkinterface_hasvalidroscredentials_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082942_networkinterface_checkonlineprivileges(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082943_cnetwork_canaccessnetworkfeatures_enetworkaccessarea_enetworkacc(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_082947_cnetwork_ischeater_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082948_cplayercardmanager_getdatamanager_cplayercardmanager_cardtypes_c(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_082957_networkutils_getscreencoordinatesforohd_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32) -> i32; }
extern "C" { pub fn f_082958_cnetwork_setgostraighttomultiplayer_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082959_cnetwork_setgostraighttompevent_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082960_cnetwork_setgostraighttomprandomjob_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083006_clivemanager_isonline_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083007_clivemanager_issignedin_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083008_networkinterface_iscloudavailable_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083009_networkinterface_isonlineros(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083011_networkinterface_hasvalidrockstarid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083012_networkinterface_getlocalgamername(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083013_networkinterface_getactivegamerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083014_networkinterface_getlocalgamerhandle(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083015_clivemanager_checkonlineprivileges_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083016_networkinterface_canpause(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083020_networkinterface_issessionleaving(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083024_networkinterface_isspectatinglocalplayer_unsigned_char(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083025_networkinterface_getphysicalplayerfromindex_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083026_networkinterface_isplayerpedspectatingplayerped_cped_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083028_networkinterface_isinspectatormode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083038_networkinterface_isinfreemode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083053_networkinterface_getactiveplayerindexfromgamerhandle_rage_rlgame(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083054_networkinterface_getactiveplayerfromindex_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083055_networkinterface_getphysicalplayerindexfromgamerhandle_rage_rlga(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083056_networkinterface_getplayerfromgamerhandle_rage_rlgamerhandle_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083061_networkinterface_getnetworkobject_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083070_networkinterface_getspectatorobjectid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083079_networkinterface_leavesession_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083125_networkinterface_getlastposreceivedovernetwork_cdynamicentity_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083138_networkinterface_isplayerintutorialsession_cnetgameplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083147_networkinterface_cansetwaypoint_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083168_networkinterface_arebulletsimpactsdisabledinmp_centity_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083290_cnetworktextchat_appendchatlog_rage_rlgamerhandle_const_ref_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_083292_cnetworktextchat_removedelegate_rage_atdelegator4_void_cnetworkt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083298_networkclan_hasprimaryclan(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083308_networkclan_releaseemblemreference_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083312_networkcrewemblemmgr_requestemblem_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083315_networkclan_getclanemblemnameforclan_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083316_networkcrewemblemmgr_releaseemblem_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083321_cnetworkcrewdatamgr_findorrequestdata_long_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_083322_cnetworkcrewdatamgr_getclanmetadata_long_networkcrewmetadata_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083323_cnetworkcrewdatamgr_getclanemblem_long_char_const_ptr_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_083325_cnetworkcrewdatamgr_getclanchallengewinloss_long_cnetworkcrewdat(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083332_clivemanager_checkusercontentprivileges_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083334_networkcrewemblemmgr_getemblemname_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083342_networkclan_getcrewemblemmgr(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083343_networkcrewemblemmgr_requestpending_emblemdescriptor_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083344_networkcrewemblemmgr_requestsucceeded_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083348_networkcrewmetadata_getcrewinfovaluestring_char_const_ptr_char_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_083349_networkcrewmetadata_getcrewinfocrewranktitle_int_char_ptr_int_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_083350_cloudlistener_cloudlistener(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083364_cloudcachelistener_dtorcloudcachelistener(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083385_ccloudmanager_requestgetglobalfile_char_const_ptr_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_083445_clivemanager_getsocialnetworkingsharingprivileges_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083458_cvideouploadmanager_custom_isuploading_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083474_socialclubmgr_getlocalgameraccountinfo_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083493_ctunables_access_unsigned_int_bool_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083494_ctunables_access_unsigned_int_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083495_ctunables_access_unsigned_int_float_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083496_ctunables_tryaccess_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083497_ctunables_tryaccess_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083531_cusercontentmanager_querycontent_rage_rlugccontenttype_char_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_083550_cusercontentmanager_requestcdncontentdata_char_const_ptr_unsigne(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083644_ccommercemanager_isthindatapopulationinprogress(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083648_networkutils_getlanguagecode(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083663_ccommercemanager_hasvaliddata(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083936_networkutils_loggamerhandle_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084069_cgroupclandata_init(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_084072_cgroupclandata_performfixup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084075_ccrewmembersclandata_init(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_084087_clivemanager_showsigninui_unsigned_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084088_clivemanager_issystemuishowing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084099_invitemgr_getconfirmtext(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_084100_invitemgr_getconfirmsubstring(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_084106_invitemgr_cancelinvite(ph: *mut Phantom); }
extern "C" { pub fn f_084108_invitemgr_getunacceptedinviteindex_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_084109_invitemgr_getunacceptedinvite_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_084110_invitemgr_handlejoinrequest_rage_rlsessioninfo_const_ref_rage_rl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_084129_invitemgr_autoconfirminvite(ph: *mut Phantom); }
extern "C" { pub fn f_084137_invitemgr_acceptinvite_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084167_networkclan_refreshjoinrequestsreceived(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084174_clivemanager_checkisagerestricted_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084178_clivemanager_showgamerprofileui_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084179_clivemanager_showgamerfeedbackui_rage_rlgamerhandle_const_ref(ph: *mut Phantom); }
extern "C" { pub fn f_084180_clivemanager_showaccountupgradeui_unsigned_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084183_clivemanager_addfriend_rage_rlgamerhandle_const_ref_char_const_p(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_084242_clivemanager_requestnewfriendspage_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_084272_metricpurchase_flow_purchasemetricinfos_savecurrentcash(ph: *mut Phantom); }
extern "C" { pub fn f_084273_metricpurchase_flow_purchasemetricinfos_setfromlocation_estorepu(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084292_metricugc_nav_metricugc_nav_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_084347_cnetworktelemetry_audtracktagged_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084436_networkclan_getavailableclanop_networkclan_enetworkclanops(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_084438_networkclan_rejectrequest_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_084441_networkclan_rejectinvite_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084442_networkclan_geterrorname_rage_netstatus_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084448_networkclan_hasprimaryclan_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084449_networkclan_getprimaryclan_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_084458_networkclan_getreceivedjoinrequest_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_084459_networkclan_getreceivedinvite_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_084461_networkclan_acceptinvite_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084462_networkclan_getuiformattedclantag_rage_rlclanmembershipdata_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084463_networkclan_getuiformattedclantag_bool_bool_char_const_ptr_int_r(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_084505_networkclan_adddelegate_rage_atdelegator1_void_networkclan_netwo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084506_networkclan_removedelegate_rage_atdelegator1_void_networkclan_ne(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084529_pcardtitlelistinfo_gettitle_unsigned_int_float_const(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_084530_cplayercardxmldatamanager_cardstat_getstat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_084535_baseplayercarddatamanager_gamerdata_gamerdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084538_baseplayercarddatamanager_playerlistdata_getstatvaluefromstatsre(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_084540_baseplayercarddatamanager_dtorbaseplayercarddatamanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084544_baseplayercarddatamanager_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084550_baseplayercarddatamanager_createrequestedrecords_baseplayercardd(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084552_baseplayercarddatamanager_hasplayedmode_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084557_baseplayercarddatamanager_commonrefreshgamerdata_baseplayercardd(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084567_cfriendplayercarddatamanager_requeststatsquery_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084579_cscriptedplayercarddatamanager_cscriptedplayercarddatamanager(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084591_cplayercardmanager_init_cplayercardmanager_cardtypes_baseplayerc(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_084599_cclanplayercarddatamanager_dtorcclanplayercarddatamanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084604_socialclubmgr_isconnectedtosocialclub_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084605_cclanplayercarddatamanager_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084610_cplayercardmanager_getpackedstatvalue_int_unsigned_int_int_ref_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_084611_socialclubmgr_socialclubinfo_setage_unsigned_char_unsigned_char(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_084612_socialclubmgr_isdatevalid_unsigned_char_unsigned_char_unsigned_s(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_084613_socialclubmgr_socialclubinfo_setpassword_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084614_socialclubmgr_socialclubinfo_setnickname_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084616_socialclubmgr_createaccount_char_const_ptr_char_const_ptr_rage_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_084617_socialclubmgr_linkaccountscauth_rage_rlscauth_authmode_rage_datc(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_084618_socialclubmgr_isonlinepolicyuptodate_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084620_socialclubmgr_geterrorname_rage_netstatus_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084621_socialclubmgr_geterrorname_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_084622_socialclubmgr_getdefaultdevnames_char_ptr_char_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_084629_cloudlistener_dtorcloudlistener_138661(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084631_tunableslistener_dtortunableslistener_138664(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085438_cnetworknewsstorymgr_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_085570_cnetworksession_ishost_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085601_socialclubeventmgr_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_085606_socialclubeventmgr_iseventactive_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085622_socialclubeventmgr_geteventdisplaynamebyid_int_char_ptr_int_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_085646_cnetworkscnewsstoryrequest_getsubtitle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085647_cnetworkscnewsstoryrequest_getcontent_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085648_cnetworkscnewsstoryrequest_getheadline_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085649_cnetworkscnewsstoryrequest_geturl_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085650_cnetworkscnewsstoryrequest_hasextradatamember_char_const_ptr_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085654_cnetworktransitionnewscontroller_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_085659_cnetworktransitionnewscontroller_updatedisplayconfig(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085660_cnetworktransitionnewscontroller_initcontrol_int_cnetworktransit(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_085665_cnetworktransitionnewscontroller_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085668_cnetworkpausemenunewscontroller_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_085669_cnetworkpausemenunewscontroller_initcontrol_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085670_cnetworkpausemenunewscontroller_requeststory_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085672_cnetworkpausemenunewscontroller_ispendingstoryready_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085673_cnetworkpausemenunewscontroller_getpendingstory_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085674_cnetworkpausemenunewscontroller_getpendingstoryindex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085675_cnetworkpausemenunewscontroller_getnumstories(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085676_cnetworkpausemenunewscontroller_ispendingstorydatarcvd(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085677_cnetworkpausemenunewscontroller_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085678_cnetworkpausemenunewscontroller_markpendingstoryasshown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085702_policyversions_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085708_policyversions_isdownloading_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085709_policyversions_isuptodate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085710_policyversions_geterror_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085711_socialclubversionedfile_acceptversion_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085713_socialclubpolicymanager_init_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_085714_socialclubpolicymanager_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085715_socialclubpolicymanager_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085716_socialclubpolicymanager_isdownloading_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085717_socialclubpolicymanager_aretextsinsyncwithcloud_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085718_socialclubpolicymanager_geterror_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085828_cnetworksession_sendinvites_rage_rlgamerhandle_const_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_085859_cnetworksession_istransitioning(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085884_cnetworksession_istransitionmember_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085904_cnetworksession_issolosession_sessiontype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085920_cnetworksession_issessionmember_rage_rlgamerhandle_const_ref_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085956_cnetworksession_removetransitioninvites_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085958_cnetworksession_canceltransitioninvites_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085962_cnetworksession_gettransitionmembers_rage_rlgamerinfo_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_086046_cnetworkinvitedplayers_getinvitedplayer_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_086047_cnetworkinvitedplayers_sinvitedgamer_iswithinackthreshold_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086079_cnetworkreadleaderboards_clearcacheddisplaydata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086083_cnetworkreadleaderboards_readbyfriends_unsigned_int_rage_rlleade(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32) -> i64; }
extern "C" { pub fn f_086127_cnetworkvoice_cancommunicatetextwithgamer_rage_rlgamerhandle_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086129_cnetworkvoice_findgamer_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_086135_cnetworkvoice_isgamertalking_rage_rlgamerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086139_cnetworkvoice_gamerhasheadset_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086141_cnetworkvoice_togglemute_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086143_cnetworkvoice_isgamermutedbyme_rage_rlgamerhandle_const_ref_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086159_cnetworkvoice_getplayerloudness_int(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_086744_cnetgameplayer_getplayerped_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086958_cnetgameplayer_getplayerinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087293_cnetobjped_getistargettablebyplayer_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_087657_cnetobjplayer_getstatvalue_cplayercardxmldatamanager_playercard(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_089837_cnetworkincrementstatevent_trigger_unsigned_int_int_rage_netplay(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_089857_csendkickvotesevent_trigger_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090717_atof(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090741_time(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090755_exp2f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090768_fmod(ph: *mut Phantom, a0: f64, a1: f64) -> f64; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090780_fputs(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090797_isxdigit(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090798_ldexp(ph: *mut Phantom, a0: i32) -> f64; }
extern "C" { pub fn f_090801_localtime(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090807_logf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090814_memcmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090817_gmtime_r(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090826_powf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090885_pthread_mutex_destroy(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090886_pthread_mutex_init(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090887_pthread_mutex_lock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090889_pthread_mutex_unlock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090890_pthread_mutexattr_settype(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090901_qsort_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_090911_rand(ph: *mut Phantom) -> i32; }
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
extern "C" { pub fn f_090939_strlen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090940_strncasecmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090942_strncmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090945_strrchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090946_strspn(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090947_strstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090957_strtoull(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_090959_strtoll(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_090965_tanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090987_vsprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090991_wcslen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090992_towlower(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_091063_op_new_impl_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
