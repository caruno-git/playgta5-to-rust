//! Callee declarations for crate gen015.
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
extern "C" { pub fn f_000116_rage_sysipcwaitsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000119_rage_sysipcsignalsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000121_rage_sysipclockmutexcommand_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000122_rage_sysipcunlockmutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000125_rage_sysipcwaitevent_rage_sysipceventtag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000128_rage_sysipccreatethread_void_ptr_void_ptr_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_000131_rage_sysipcwaitthreadexit_rage_sysipcthreadid_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000132_rage_sysipcgetprocessorcount(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000135_rage_sysipcsleep_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000221_void_std_2_tree_balance_after_insert_abi_nqn220108_std_2_tree_no(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000570_wasm_input_pop_char(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000644_rage_athashnextsize_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000646_rage_athash64_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000654_rage_atstring_set_char_const_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_000655_rage_atstring_lowercase(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000657_rage_atstring_endswith_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000668_rage_atarray_rage_atstring_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
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
extern "C" { pub fn f_000794_rage_bkconsole_addcommand_char_const_ptr_rage_atfunctor3_void_ch(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000806_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000828_rage_bklist_addcolumnheader_int_char_const_ptr_rage_bklist_itemt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_000829_rage_bklist_additem_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_000830_rage_bklist_additem_int_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_000831_rage_bklist_removeitem_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000843_rage_bkcombo_setstring_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_000851_char_ptr_rage_formatf_char_8ul_char_ref_8ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000866_rage_bkgroup_addseparator_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000867_rage_bkgroup_addtitle_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000868_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_000869_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000870_rage_bkgroup_addbutton_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000871_rage_bkgroup_addtext_char_const_ptr_bool_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000872_rage_bkgroup_addtext_char_const_ptr_float_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000876_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_000877_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_000882_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000886_rage_bkgroup_addtoggle_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000889_rage_bkgroup_addtoggle_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_000890_rage_bkgroup_addtoggle_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000894_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000898_rage_bkgroup_addslider_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000905_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000906_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000907_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int_rage_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000909_rage_bkgroup_addslider_char_const_ptr_rage_vector2_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000911_rage_bkgroup_addslider_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000912_rage_bkgroup_addslider_char_const_ptr_rage_vector4_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000918_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000919_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000921_rage_bkgroup_addlist_char_const_ptr_bool_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000926_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000927_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000941_rage_bkgroup_addcolor_char_const_ptr_rage_color32_ptr_rage_datca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_000942_rage_bkgroup_addcolor_char_const_ptr_rage_color32_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000945_rage_bkgroup_addcolor_char_const_ptr_rage_vector4_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000946_rage_bkgroup_addcolor_char_const_ptr_rage_vector3_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001062_rage_matrix34_inverse_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001105_rage_autoidregistry_autoidregistry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001108_rage_autoidregistry_unregister_rage_autoiddescriptor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001116_char_ptr_rage_formatf_char_32ul_char_ref_32ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001119_rage_datcallback_datcallback_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001140_rage_datbitbuffer_readunsigned_void_const_ptr_unsigned_int_ref_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001141_rage_datbitbuffer_writeunsigned_void_ptr_unsigned_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_001156_rage_datbitbuffer_bumpreadcursor_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001158_rage_datbitbuffer_bumpreadwritecursor_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001166_rage_datbitbuffer_readbool_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001167_rage_datbitbuffer_writebool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001176_rage_datresourceinfo_generatemap_rage_datresourcemap_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001191_rage_aes_aes_unsigned_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001213_rage_rsonwriter_beginarray_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001214_rage_rsonwriter_end(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001216_rage_rsonwriter_writeint_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001218_rage_rsonwriter_writeint64_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001220_rage_rsonwriter_writeuns_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001221_rage_rsonwriter_writeuns64_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001222_rage_rsonwriter_writefloat_char_const_ptr_float_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_001224_rage_rsonwriter_writestring_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001229_rage_rsonwriter_writebool_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
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
extern "C" { pub fn f_001538_rage_diagchannel_setoutput_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_001551_rage_fprintf_rage_fistream_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001560_rage_diagcontextmessage_diagcontextmessage_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001563_rage_diagcontextmessage_pop(ph: *mut Phantom); }
extern "C" { pub fn f_001565_printf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001568_rage_atmapentry_unsigned_int_wchar_t_ptr_const_ref_rage_atmap_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001569_rage_atmap_unsigned_int_wchar_t_ptr_rage_atmaphashfn_unsigned_in(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001578_rage_fistream_seek_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001580_rage_fiassetmanager_open_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001582_rage_fiassetmanager_createleadingpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001593_mg_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001624_rage_fiassetmanager_setpath_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001627_rage_fiassetmanager_exists_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001632_rage_fiassetmanager_pushfolder_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001633_rage_fiassetmanager_popfolder(ph: *mut Phantom); }
extern "C" { pub fn f_001635_rage_fiassetmanager_filename_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001637_rage_fiassetmanager_enumfiles_char_const_ptr_void_ptr_rage_fifin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_001639_rage_fiassetmanager_removeextensionfrompath_char_ptr_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001640_rage_fiassetmanager_removenamefrompath_char_ptr_int_char_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001647_rage_fistream_tell(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001649_rage_fgets_char_ptr_int_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_001768_rage_fidevicerelative_init_char_const_ptr_bool_rage_fidevice_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_001769_rage_fidevicerelative_mountas_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001880_rage_fipackfile_fipackfile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001881_rage_fipackfile_init_char_const_ptr_bool_rage_fipackfile_cachemo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_001883_rage_fipackfile_mountas_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001885_rage_fipackfile_dtorfipackfile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002031_rage_mthrandom_mthrandom_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002040_rage_pgrscbuilder_allocatemap_rage_datresourcemap_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002042_rage_pgrscbuilder_freememory_rage_datresourcemap_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002044_rage_pgrscbuilder_computeleafsize_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002046_rage_pgrscbuilder_cleanup_rage_datresourcemap_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002047_rage_pgrscbuilder_loadbuild_char_const_ptr_char_const_ptr_int_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_002080_rage_pgbase_pgbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002088_rage_datresource_datresource_rage_datresourcemap_const_ref_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002090_rage_datresource_dtordatresource(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002091_rage_datresource_containsthisaddress_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002092_rage_datresource_getfixup_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002111_rage_parmanager_parmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002112_rage_parmanager_initialize_rage_parsettings_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002128_rage_parmanager_uninitialize(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002129_rage_atmapentry_rage_atnonfinalhashstring_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002131_rage_atnonfinalhashstring_atnonfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002132_rage_parmanager_dtorparmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002135_rage_atmap_int_rage_paroutputformatinfo_rage_atmaphashfn_int_rag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002147_rage_parmanager_loadtree_char_const_ptr_char_const_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002158_rage_parinstancevisitor_visittoplevelstructure_rage_parparsables(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002165_rage_parmanager_createandloadanytype_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_002175_rage_partree_dtorpartree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002186_rage_parattribute_findint64value_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002187_rage_parattribute_findboolvalue_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002194_rage_parelement_addattribute_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_002204_rage_parattribute_finddoublevalue_const(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_002215_rage_partreenode_findnumchildren_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002295_rage_atarray_rage_psorscbuilderinstanceset_ptr_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002311_rage_atarray_rage_psoenumtableentry_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002335_unsigned_int_ref_rage_atmap_rage_parparsablestructure_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002436_rage_parcguregisterstructure_rage_parstructure_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002437_rage_parcgustructure_createstructure_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002438_rage_parcgustructure_setfactories_rage_parstructure_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002439_rage_parcgustructure_setgetstructurecb_rage_parstructure_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002440_rage_parcgustructure_setbaseclass_rage_parstructure_ptr_rage_par(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002441_rage_parcgustructure_beginaddinginitialdelegates_rage_parstructu(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002442_rage_parcgustructure_adddelegateholder_rage_parstructure_ptr_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002443_rage_parcgustructure_endaddinginitialdelegates_rage_parstructure(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002444_rage_parcgustructure_buildstructurefromstaticdata_void_ptr_rage(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002448_int_rage_formatf_n_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002479_rage_parxmlwritervisitor_parxmlwritervisitor_rage_fistream_ptr_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002593_rage_parsettings_createstandardsettings(ph: *mut Phantom); }
extern "C" { pub fn f_002690_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002691_rage_psorscbuilderinstance_ptr_ptr_std_2_partial_sort_impl_abi_n(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002692_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002693_rage_psorscbuilderinstance_ptr_ptr_std_2_partition_with_equals_o(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002695_rage_atbinarymap_rage_pardelegateholderbase_ptr_unsigned_int_dat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002696_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002697_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_002719_strlwr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002744_rage_atdatahash_unsigned_int_const_ptr_unsigned_long_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002746_rage_stristr_char_ptr_char_const_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002747_rage_striskip_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002761_rage_widetoutf8_char_ptr_unsigned_short_const_ptr_int_int_int_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_002762_rage_utf8towide_unsigned_short_ptr_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002763_rage_utf8removeinvalidsurrogates_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002806_rage_sysservice_adddelegate_rage_atdelegator1_void_rage_sysservi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002808_rage_atdelegator1_void_rage_sysserviceevent_ptr_removedelegate_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002811_rage_syshangdetectsetcrashfunc_void_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002839_rage_sysparam_get_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002840_rage_sysparam_get_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002849_rage_sysbootmanager_isbootedfromdisc(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002854_rage_sysstack_printregisteredbacktrace_unsigned_short_void_ptr_u(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002871_rage_sysgetenv_char_const_ptr_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002878_rage_atmap_unsigned_int_rage_sysstackcollector_tagentry_rage_atm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002881_unsigned_int_ref_rage_atmap_unsigned_int_unsigned_int_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002927_rage_sysmemstarttemp(ph: *mut Phantom); }
extern "C" { pub fn f_002928_rage_sysmemendtemp(ph: *mut Phantom); }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_002933_rage_sysmemgetbucketname_rage_memorybucketids(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_002935_rage_sysmemtotalfreememory(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002939_rage_sysmemmanager_sysmemmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002945_rage_sysmempoolallocator_safeallocate_unsigned_long_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002946_rage_sysmempoolallocator_allocate_unsigned_long_unsigned_long_bo(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002947_rage_sysmempoolallocator_safefree_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003084_rage_sysmemsystemtracker_print_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003097_char_ptr_rage_formatf_char_16ul_char_ref_16ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003173_rage_vector2_vector2_rage_vector3_const_ref_rage_vector2_evector(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003186_rage_crframe_crframe_rage_crframedata_const_ref_unsigned_char_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_003214_rage_crframe_getvector3_unsigned_char_unsigned_short_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_003221_rage_crframe_dtorcrframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003313_rage_crframedata_crframedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003314_rage_crframedata_dtorcrframedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003315_rage_crframedata_init_unsigned_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_003563_rage_crcreature_getcomponentbyindex_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003809_rage_crframedata_adddof_unsigned_char_unsigned_short_unsigned_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_003831_rage_fwanimdirectorcomponentsyncedscene_isplayingsyncedscene_con(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003832_rage_fwanimdirectorcomponentsyncedscene_isvalidsceneid_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003840_rage_fwclipsetmanager_getclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003864_rage_fwanimdirector_initstatic_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_003870_rage_fwanimdirector_shutdownstatic_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_003886_rage_fwanimhelpers_getmovertrackmatrixdelta_rage_crclip_const_re(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64); }
extern "C" { pub fn f_003893_rage_fwanimhelpers_applyinitialoffsetfromclip_rage_crclip_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003896_rage_fwanimmanager_getclipifexistsbydictindex_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_003906_rage_fwanimmanager_findhashkeyfromslot_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003907_rage_fwanimmanager_isvalidslot_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003911_rage_fwanimmanager_addref_rage_strlocalindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_003912_rage_fwanimmanager_removeref_rage_strlocalindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_003913_rage_fwanimmanager_getnumrefs_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003968_rage_fwclipset_getclipdictionaryindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003969_rage_fwclipset_getclipdictionarymetadata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003970_rage_fwclipsetmanager_getclipdictionarymetadata_rage_atnonfinalh(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003977_rage_fwclipset_request_deprecated_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_003978_rage_fwclipset_clearrequiredflag_deprecated_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003979_rage_fwclipset_isstreamedin_deprecated_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003980_rage_fwclipset_addref_deprecated(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003981_rage_fwclipset_release_deprecated(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003984_rage_fwclipsetrequesthelper_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003989_rage_fwclipsetmanager_getmemorygroupmetadata_rage_atnonfinalhash(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003990_rage_fwclipsetmanager_getmemorygroupmetadatabyindex_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_003998_rage_fwclipsetmanager_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004001_rage_fwclipsetmanager_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004014_rage_atstring_ref_rage_atmap_int_rage_atstring_rage_atmaphashfn(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004019_rage_fwclipsetrequesthelper_request_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_004390_rage_fwanimdirectorcomponentsyncedscene_decsyncedsceneref_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004392_rage_fwanimdirectorcomponentsyncedscene_addsyncedsceneref_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004393_rage_fwanimdirectorcomponentsyncedscene_getsyncedsceneduration_s(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004397_rage_fwanimdirectorcomponentsyncedscene_setsyncedscenephase_shor(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_004398_rage_fwanimdirectorcomponentsyncedscene_getsyncedscenephase_shor(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004400_rage_fwanimdirectorcomponentsyncedscene_updatesyncedsceneduratio(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004402_rage_fwanimdirectorcomponentsyncedscene_setsyncedsceneorigin_sho(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_004403_rage_fwanimdirectorcomponentsyncedscene_getsyncedsceneorigin_sho(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_004404_rage_fwanimdirectorcomponentsyncedscene_issyncedscenepaused_shor(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004405_rage_fwanimdirectorcomponentsyncedscene_setsyncedscenepaused_sho(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_004419_rage_fwanimdirectorcomponentsyncedscene_preparesynchronizedscene(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_004420_rage_fwanimdirectorcomponentsyncedscene_playsynchronizedsceneaud(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004421_rage_fwanimdirectorcomponentsyncedscene_stopsynchronizedsceneaud(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004603_rage_fwextensionlist_add_rage_fwextension_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004615_rage_fwarchetypemanager_poolfullcallback(ph: *mut Phantom); }
extern "C" { pub fn f_004619_rage_fwarchetype_getdrawable_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004620_rage_fwarchetype_getfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004621_rage_fwarchetype_getassetparenttxdindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004631_rage_fwarchetypemanager_getarchetypename_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004632_rage_fwarchetypemanager_lookupmodelid_rage_fwarchetype_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004633_rage_fwarchetypemanager_getarchetype_rage_atnonfinalhashstring_u(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004634_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004635_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_rag(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004636_rage_fwarchetypemanager_getarchetypeindexfromhashkey_unsigned_in(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004640_rage_fwarchetypemanager_scheduleremovearchetypetotypefileref_rag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004661_rage_fwdynamicentitycomponent_fwdynamicentitycomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004663_rage_fwdynamicentitycomponent_createattachmentextensionifmissing(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004667_rage_fwentity_protectstreamedarchetype(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004683_rage_fwentity_getaabb_rage_spdaabb_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004685_rage_fwentity_getrootlod(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004686_rage_fwextensionlist_dtorfwextensionlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004714_rage_fwcustomshadereffect_setfloatshadergroupvar_rage_grmshaderg(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_004715_rage_fwcustomshadereffect_setvector2shadergroupvar_rage_grmshade(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004716_rage_fwcustomshadereffect_setvector3shadergroupvar_rage_grmshade(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004737_rage_fwdecorator_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004740_rage_fwdecoratorextension_existson_rage_fwextensiblebase_ref_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_004748_rage_fwdecoratorextension_find_rage_atnonfinalhashstring_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004761_rage_fwdebugbank_createbank_char_const_ptr_rage_functor0_rage_fu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_004763_rage_fwdebugbank_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004764_rage_fwdebugbank_toggleactive(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004774_rage_fwpickermanager_setenabled_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004777_rage_fwpickermanager_setpickermanagersettings_rage_fwpickermanag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004778_rage_fwpickermanager_setpickerinterfacesettings_rage_fwpickerint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004779_rage_fwpickermanager_setpickersettings_int_rage_fwbasepickersett(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_004780_rage_fwpickermanager_iscurrentowner_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004781_rage_fwpickermanager_takecontrolofpickerwidget_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004787_rage_fwpickermanager_resetlist_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004792_rage_fwpickermanager_getselectedentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004825_rage_fwvectormap_draw3dboundingboxonvmap_rage_spdaabb_const_ref(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004925_rage_dldrawcommandbuffer_dumpdrawlistpages(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004932_rage_dldrawcommandbuffer_lookupshareddata_int_rage_dlshareddatai(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_004935_rage_dlcmdbase_op_new_unsigned_long_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004949_rage_dldrawlistmgr_getdrawlistname_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004963_rage_dlcmdpushtimebar_dlcmdpushtimebar_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_004964_rage_dlcmdpoptimebar_dlcmdpoptimebar(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005003_rage_dldrawlistmgr_addtxdreference_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005053_rage_fwtextutil_getbytecount_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005209_rage_ropeinstance_setnewuniqueid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005223_rage_ropemanager_removerope_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005242_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_int_ptr_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_005243_int_ptr_std_2_partition_with_equals_on_left_abi_nqn220108_std_2(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_005244_std_2_pair_int_ptr_bool_std_2_partition_with_equals_on_right_abi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_005245_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_005301_void_rage_dlc_addinternal_void_ptr_void_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005313_rage_fwrenderthreadinterface_isusingdefaultrenderfunction(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_005315_rage_fwrenderthreadinterface_flush_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005385_rage_fwmapdatastore_setenablediplgroups_rage_atarray_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005386_rage_fwmapdatastore_requestgroup_rage_strlocalindex_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_005387_rage_fwmapdatastore_removegroup_rage_strlocalindex_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_005390_rage_fwimapgroupswap_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005391_rage_fwimapgroupswap_abandonearly_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005394_rage_fwmapdatastore_setstreamable_rage_strlocalindex_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_005395_rage_fwmapdatastore_saferemove_rage_strlocalindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005399_rage_fwimapgroupmgr_abandonswap_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005401_rage_fwmapdatadef_removemapchange(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005424_rage_fwgrassinstancelistdef_computelodinstfaderange_rage_scalarv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_005458_rage_fwboxstreamer_requestaboutpos_rage_fwboxstreamersearch_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_005459_rage_atarray_rage_fwboxstreamersearch_0_unsigned_short_grow_unsi(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005460_rage_fwboxstreamer_requestaboutpos_rage_vec3v_const_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_005502_rage_fwassetstore_rage_pgdictionary_rage_charactercloth_rage_fwc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005576_rage_fwassetstore_rage_fwmapdatacontents_rage_fwmapdatadef_regis(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005580_rage_fwmapdatastore_saferemoveall_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005601_rage_fwmapdatastore_removeunrequired_bool(ph: *mut Phantom); }
extern "C" { pub fn f_005609_rage_fwmapdatastore_createchildrencache(ph: *mut Phantom); }
extern "C" { pub fn f_005624_rage_fwmaptypesstore_update(ph: *mut Phantom); }
extern "C" { pub fn f_005625_rage_fwmaptypesstore_printstore(ph: *mut Phantom); }
extern "C" { pub fn f_005635_rage_fwassetstore_rage_fwmaptypescontents_rage_fwmaptypesdef_get(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005679_rage_fwstaticboundsstore_removeall(ph: *mut Phantom); }
extern "C" { pub fn f_005686_rage_fwstaticboundsstore_initwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005706_rage_fwtxdstore_getsafefromindex_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005730_rage_fwinteriorlocation_makeinvalid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005732_rage_fwinteriorlocation_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005733_rage_fwinteriorlocation_getroomindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005917_rage_fwrefawarebaseimpl_rage_datbase_dtorfwrefawarebaseimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005919_rage_aitask_setstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005926_rage_aitask_setnewtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005949_rage_aitasktree_findtaskbytypeactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005971_rage_cnavmesh_switchpolysinarea_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_006035_rage_atmapentry_unsigned_int_bool_const_ref_rage_atmap_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006037_rage_strstreamingloadermanager_callkeepalivecallbackifnecessary(ph: *mut Phantom); }
extern "C" { pub fn f_006052_rage_strpackfilemanager_unloadrpf_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006054_rage_strpackfilemanager_closearchive_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006056_rage_strstreaminginfomanager_removeobject_rage_strindex_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006064_rage_strstreamingvisualize_send_int_void_const_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_006065_rage_strpackfilemanager_getimagefilenamefromhandle_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006066_rage_strpackfilemanager_getimagefilefromhandle_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006067_rage_strpackfilemanager_getimagefileindexfromhandle_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006069_rage_strpackfilemanager_getstreamingmodule(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_006071_rage_strstreamingmodulemgr_getmoduleidfromindex_rage_strindex_co(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006072_rage_strstreaminginfomanager_getobjectname_rage_strindex_char_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_006075_rage_strstreaminginfo_computevirtualsize_rage_strindex_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006076_rage_strstreaminginfo_computephysicalsize_rage_strindex_bool_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006081_rage_strpackfilemanager_loadimageifunloaded_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006083_rage_strpackfilemanager_addstreamablereference_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006084_rage_strstreaminginfomanager_requestobject_rage_strindex_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006085_rage_strpackfilemanager_registerindividualfile_char_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_006086_rage_strpackfilemanager_invalidateindividualfile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006088_rage_atarray_rage_atnonfinalhashstring_0_unsigned_short_grow_uns(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006092_rage_strstreamingmodulemgr_getmodulefromindex_rage_strindex_cons(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006105_rage_strpackfilemanager_debugdraw(ph: *mut Phantom); }
extern "C" { pub fn f_006110_rage_strpackfilemanager_evictunusedimages(ph: *mut Phantom); }
extern "C" { pub fn f_006122_rage_strstreaminginfomanager_getmodule_rage_strindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006124_rage_strstreamingengine_getobjectname_rage_strindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006130_rage_strstreaminginfomanager_deletenextleastusedobject_unsigned(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_006140_rage_strstreaminginfomanager_resetinfoiteratorfordeletingnextlea(ph: *mut Phantom); }
extern "C" { pub fn f_006141_rage_strstreamingmodule_getobjectanddependencies_rage_strindex_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_006142_rage_strstreaminginfomanager_isobjectreadytodelete_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006143_rage_strstreaminginfoiterator_strstreaminginfoiterator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006144_rage_strstreaminginfoiterator_op(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006145_rage_strstreaminginfoiterator_isatend_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006149_rage_strstreaminginfomanager_destroydependentgraphs(ph: *mut Phantom); }
extern "C" { pub fn f_006156_rage_atarray_rage_strindex_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006157_rage_strstreaminginfo_getpreferreddevice_rage_strindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006158_rage_strstreaminginfo_getcdposn_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006159_rage_strstreaminginfo_computeoccupiedvirtualsize_rage_strindex_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006162_rage_strstreaminginfo_gethandle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006165_rage_strstreaminglivefile_unlinkfile(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006167_rage_strstreaminginfomanager_isobjectinimage_rage_strindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006174_rage_strstreaminginfomanager_isobjectdeletable_rage_strindex_uns(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006175_rage_strstreaminginfomanager_clearrequiredflag_rage_strindex_uns(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006179_rage_strstreaminginfomanager_getobjectanddependenciessizes_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_006180_rage_strstreaminginfomanager_getobjectanddependenciessizes_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_006181_rage_atmapentry_unsigned_int_rage_strindex_const_ref_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006183_rage_strstreaminginfomanager_getobjectindex_rage_strindex_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006185_rage_strstreaminginfomanager_purgerequestlist_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_006186_rage_strstreaminginfomanager_flushloadedlist_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_006188_rage_atarray_int_0_unsigned_short_ptr_ref_rage_atmap_int_rage_at(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006190_rage_strstreaminginfomanager_unregisterobject_rage_strindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006195_rage_strstreaminginfomanager_getstreamingindexbyname_char_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_006196_rage_strstreaminginfomanager_setdonotdefrag_rage_strindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006197_rage_strloadedinfoiterator_strloadedinfoiterator_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006198_rage_strloadedinfoiterator_getnextindex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006211_rage_strstreamingloadermanager_flush(ph: *mut Phantom); }
extern "C" { pub fn f_006212_rage_strstreamingengine_shutdownpackfiles(ph: *mut Phantom); }
extern "C" { pub fn f_006213_rage_strstreamingloader_loadrequestedobjects(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006214_rage_strstreamingloader_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006220_rage_strstreamingengine_addrefbyptr_void_ptr_rage_strrefkind(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006221_rage_strstreamingengine_removerefbyptr_void_ptr_rage_strrefkind(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006227_rage_strstreaminglive_updatealldirtyfiles(ph: *mut Phantom); }
extern "C" { pub fn f_006235_rage_strstreamingvisualize_setmarkertext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006237_rage_strstreamingloadermanager_processasyncfiles(ph: *mut Phantom); }
extern "C" { pub fn f_006238_rage_strstreamingloadermanager_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006241_rage_strstreamingvisualize_addmarker_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006244_rage_strstreamingloader_showfilestatus_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006245_rage_strstreamingloader_showasyncstats_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006246_rage_strstreamingmodule_getvirtualmemoryofloadedobj_rage_strloca(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_006247_rage_strstreamingmodule_getphysicalmemoryofloadedobj_rage_strloc(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_006248_rage_strstreamingmodule_updateplacestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006249_rage_strstreamingmodule_resetplacestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006252_rage_strstreamingmodule_streamingblockingload_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006253_rage_strmoduletypeindextoextension_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006254_rage_strrequest_request_rage_strlocalindex_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_006255_rage_strrequest_hasloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006256_rage_strrequest_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006257_rage_strrequest_clearrequiredflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006262_rage_strrequest_getname(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006281_rage_strstreamingvisualize_addcallstacktomarker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006886_rage_audmetadatamanager_getobjectnamefromnametableoffset_unsigne(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007200_char_ptr_rage_formatf_char_255ul_char_ref_255ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007419_rage_vfxlistitem_vfxlistitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007426_rage_vfxbrokenfraginfo_dtorvfxbrokenfraginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007504_rage_decalmanager_findfirstbucket_unsigned_int_rage_fwentity_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_007528_rage_ptfxattach_attach_rage_ptxeffectinst_ptr_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_007559_rage_ptfxmanager_gettriggeredinst_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007576_rage_ptfxscript_start_rage_atnonfinalhashstring_rage_atnonfinalh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: f32, a7: i32) -> i32; }
extern "C" { pub fn f_007579_rage_ptfxscript_destroy_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007632_rage_fwidinstanceinfo_dtorfwidinstanceinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007634_rage_fwuniqueobjectidgenerator_getid_void_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007637_rage_fwuniqueobjectidgenerator_getidinstanceinfofromid_unsigned(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007658_rage_fwptrlistsinglelink_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007756_rage_netinterface_flushalllogfiles_bool(ph: *mut Phantom); }
extern "C" { pub fn f_007759_rage_netinterface_getnumphysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007760_rage_netinterface_getnumremotephysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007762_rage_netinterface_getallphysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007763_rage_netinterface_getremotephysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007792_rage_netbandwidthstatistics_allocatebandwidthrecorder_char_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007802_rage_netplayer_ismyplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007803_rage_networklogutils_writelogevent_rage_netlogginginterface_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007837_rage_netarrayhandlerbase_dirtyallelements(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007848_rage_neteventmgr_checkforspaceinpool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007849_rage_neteventmgr_prepareevent_rage_netgameevent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007855_rage_netplayer_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007893_rage_netbandwidthstatistics_getbandwidthrecorder_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007940_rage_neteventmgr_geteventnamefromtype_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007949_rage_netgameevent_netgameevent_unsigned_short_bool_bool_rage_net(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_007995_rage_matrix34_toeulersxyz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008011_rage_fwhttpquery_init_char_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_008013_rage_fwhttpquery_writebuffertofile_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008017_rage_fwhttpquery_pending(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008018_rage_fwhttpquery_createtree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008070_rage_netobject_getplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008071_rage_netobject_getpendingplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008074_rage_netobject_ispendingownerchange_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008075_rage_netobject_islocalflagset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008076_rage_netobject_setlocalflag_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_008077_rage_netobject_isglobalflagset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008081_rage_netobject_hasbeencloned_rage_netplayer_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008172_rage_netobjectmgrbase_updateallinscopestateimmediately_rage_neto(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008185_rage_netobjectmgrbase_getlocalobjectlists_rage_atdlist_rage_neto(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008189_rage_netobjectmgrbase_getallobjects_rage_netobject_ptr_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_008682_rage_fwbasepool_fwbasepool_int_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_008683_rage_fwconfigmanager_getsizeofpool_unsigned_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008686_rage_fwbasepool_getindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008688_rage_fwbasepool_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008689_rage_fwscriptguid_createguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008691_rage_fwscriptguid_deleteguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008693_rage_fwscriptguid_getbasefromguid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_008695_rage_scripthandler_dtorscripthandler(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008696_rage_scripthandler_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008698_rage_scripthandler_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008699_rage_scripthandler_destroynetworkcomponent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008700_rage_scripthandler_terminate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008701_rage_scripthandlermgr_writescriptheader_rage_netlogginginterface(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_008703_rage_scripthandler_registernewscriptobject_rage_scripthandlerobj(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_008705_rage_scripthandlernetcomponent_getstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008706_rage_scripthandler_getscriptobject_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008708_rage_scripthandler_unregisterscriptobject_rage_scripthandlerobje(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008709_rage_scripthandler_registerscriptresource_rage_scriptresource_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_008710_rage_scripthandler_getscriptresource_unsigned_short_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_008711_rage_scripthandler_removescriptresource_unsigned_short_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_008712_rage_scripthandler_removescriptresource_unsigned_int_bool_bool_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_008716_rage_scripthandler_getnumscriptresourcesoftype_unsigned_short_co(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008717_rage_scripthandlernetcomponent_gethostbroadcastdatahandler_unsig(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008719_rage_scripthandler_registerexistingscriptobjectinternal_rage_scr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008720_rage_scripthandler_cleanupscriptobject_rage_scripthandlerobject(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008721_rage_scripthandler_destroyscriptresource_rage_scriptresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008723_rage_scripthandler_removeoldscriptobject_rage_scripthandlerobjec(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008747_rage_scripthandlermgr_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008748_rage_scripthandlermgr_networkinit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008749_rage_scripthandlermgr_deletehandlerwithhash_rage_athashvalue(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008750_rage_scripthandlermgr_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008751_rage_scripthandlermgr_networkupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008754_rage_scripthandlermgr_networkshutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008755_rage_scripthandlermgr_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008756_rage_scripthandlermgr_registerscript_rage_scrthread_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008767_rage_scripthandlermgr_playerhasleft_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008768_rage_scripthandlermgr_removescriptresource_unsigned_short_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_008774_rage_scripthandlernetcomponent_dtorscripthandlernetcomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008777_rage_scripthandlernetcomponent_setmaxnumparticipants_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008778_rage_scripthandlernetcomponent_ishostlocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008779_rage_scripthandlernetcomponent_playerhasjoined_rage_netplayer_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008781_rage_scripthandlernetcomponent_isplayeraparticipant_rage_netplay(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008782_rage_scripthandlernetcomponent_getparticipantsdata_rage_netplaye(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_008784_rage_scripthandlernetcomponent_handleleavingplayer_rage_netplaye(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008787_rage_scripthandlernetcomponent_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008797_rage_scripthandlernetcomponent_migratescripthost_rage_netplayer(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008798_rage_scripthandlernetcomponent_isparticipantactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008799_rage_scripthandlernetcomponent_getparticipantusingslot_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008802_rage_scripthandlernetcomponent_handlejoinackmsg_rage_msgscriptjo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008803_rage_scripthandlernetcomponent_handlejoinhostackmsg_rage_msgscri(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008825_rage_scripthandlernetcomponent_requesttobehost(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008826_rage_scripthandlernetcomponent_canstartjoining(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008830_rage_scripthandlernetcomponent_addplayerasparticipant_rage_netpl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_008831_rage_scripthandlernetcomponent_removeplayerasparticipant_rage_ne(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008832_rage_scripthandlernetcomponent_restartjoiningprocess(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008833_rage_scripthandlernetcomponent_doleavecleanup(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008834_rage_scripthandlernetcomponent_doterminationcleanup(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008835_rage_scripthandlernetcomponent_setnewscripthost_rage_netplayer_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_008836_rage_scriptid_read_rage_datbitbuffer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008837_rage_scriptid_write_rage_datbitbuffer_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008838_rage_scriptid_log_rage_netlogginginterface_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008839_rage_scriptid_initscriptid_rage_scrthread_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008841_rage_scriptinterface_isnetworkscript_rage_scriptidbase_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008897_rage_fwmetadatastore_removeunrequired(ph: *mut Phantom); }
extern "C" { pub fn f_008916_rage_fwmetadatastore_removeall_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008917_rage_fwtimer_enduserpause(ph: *mut Phantom); }
extern "C" { pub fn f_008918_rage_fwtimer_startuserpause_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008931_bool_ref_rage_atmap_rage_strindex_bool_rage_atmaphashfn_rage_str(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_008943_rage_fwbasepool_fwbasepool_char_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_008945_rage_fwbasepool_forall_bool_ptr_void_ptr_void_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008949_rage_fwbasepool_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008952_rage_fwbasepool_getat_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008953_rage_fwknownrefholder_fwknownrefholder_void_ptr_ptr_rage_fwknown(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_008971_rage_scripthandlermgr_dtorscripthandlermgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008996_rage_fwbasepool_dtorfwbasepool_14863(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009233_rage_crskeleton_getglobalmtx_unsigned_int_rage_mat34v_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009283_rage_grmmodel_getgeometry_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009290_rage_grmmodel_drawskinned_int_rage_grmmatrixset_const_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009291_rage_grmshader_begindraw_rage_grmshader_edrawtype_bool_rage_grce(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_009293_rage_grmshader_bind_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009294_rage_grmshader_unbind_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009295_rage_grmshader_enddraw_const(ph: *mut Phantom); }
extern "C" { pub fn f_009312_rage_grmshader_setforcedtechniquegroupid_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009327_rage_grmshadergroup_lookupvar_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009330_rage_grmshadergroup_lookupshader_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009331_rage_grmshadergroup_checkforshaderswithsuffix_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009333_rage_grmshadergroup_setvarcommon_rage_grmshadergroupvar_void_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_009334_rage_grmshadergroup_setvar_rage_grmshadergroupvar_rage_grctextur(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009335_rage_grmshadergroup_setvar_rage_grmshadergroupvar_rage_color32_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009345_rage_grmmatrixset_computematrix_rage_matrix34_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009392_rage_pftimebars_stimebarframe_computeframetime_bool_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_009514_rage_grctexture_setcustomloadname_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009516_rage_grcimage_createinternal_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_009517_rage_grcimage_release_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009518_rage_grcimage_create_unsigned_int_unsigned_int_unsigned_int_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_009525_rage_grcimage_loaddds_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009539_rage_grceffect_lookuptechnique_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009540_rage_grceffect_lookupvar_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009542_rage_grcstateblock_createblendstate_rage_grcblendstatedesc_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009544_rage_grcdrawsinglequadf_float_float_float_float_float_float_floa(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: i64); }
extern "C" { pub fn f_009545_rage_grcstateblock_setblendstate_rage_grcstateblock_grcblendstat(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009555_rage_grcstateblock_setrasterizerstate_rage_grcstateblock_grcrast(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009559_rage_grcdevice_clear_bool_rage_color32_bool_float_bool_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_009560_rage_grcstateblock_setdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009561_rage_grcbegin_rage_grcdrawmode_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009562_rage_grcvertex_float_float_float_float_float_float_rage_color32(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: f32, a8: f32); }
extern "C" { pub fn f_009563_rage_grcend(ph: *mut Phantom); }
extern "C" { pub fn f_009587_rage_grclightstate_setenabled_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_009588_rage_grcimage_dtorgrcimage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009615_rage_grceffect_lookupglobalvar_char_const_ptr_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009635_rage_grcviewport_setcurrentworldmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009640_rage_grcviewport_isspherevisible_float_float_float_float_float_p(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_009646_rage_grcviewport_transform_rage_vec3v_const_ref_float_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009650_rage_grcbindtexture_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009670_rage_grcsetup_setpaused_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009672_rage_grcadaptermanagerd3d11_enumerate_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009673_rage_grcdevice_initadapterordinal(ph: *mut Phantom); }
extern "C" { pub fn f_009675_rage_grcstateblock_createdepthstencilstate_rage_grcdepthstencils(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009694_rage_grcsetup_takescreenshot_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009769_rage_grcvertexbuffereditor_grcvertexbuffereditor_rage_grcvertexb(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_009771_rage_grcvertexbuffereditor_dtorgrcvertexbuffereditor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009776_rage_grcbuffereditor_getnormal_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009777_rage_grcbuffereditor_getposition_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009798_rage_grcimage_loadjpeg_char_const_ptr_rage_grcimage_ptr_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009809_rage_grcinstancedata_getvariablename_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009810_rage_grctexchangedata_grctexchangedata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009814_rage_grceffect_setvar_rage_grcinstancedata_ref_rage_grceffectvar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009815_rage_grcinstancedata_addwidgets_withload_rage_bkbank_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_009858_rage_grcstateblock_createrasterizerstate_rage_grcrasterizerstate(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009859_rage_grceffect_setglobalfloatcommon_int_float_ptr_int_rage_grcef(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_009863_rage_grceffect_findtechniquegroupid_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009868_rage_grceffect_lookupvarbyhash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009869_rage_grceffect_getlocalcommon_rage_grcinstancedata_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_009871_rage_grceffect_setglobalvar_rage_grceffectglobalvar_int_const_re(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009872_rage_grceffect_setglobalvar_rage_grceffectglobalvar_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009875_rage_grceffect_setlocalcommon_rage_grcinstancedata_ref_rage_grce(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_009876_rage_grceffect_setvar_rage_grcinstancedata_ref_rage_grceffectvar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009880_rage_grceffect_getvar_rage_grcinstancedata_const_ref_rage_grceff(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009881_rage_grceffect_setglobalvar_rage_grceffectglobalvar_rage_grctext(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009882_rage_grceffect_recordsetvar_rage_grcinstancedata_ref_rage_grceff(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009883_rage_grceffect_recordsetvar_rage_grcinstancedata_ref_rage_grceff(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_009884_rage_grceffect_recordsetvar_rage_grcinstancedata_ref_rage_grceff(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009893_rage_grcstateblock_setdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009894_rage_grcstateblock_setblendstate_rage_grcstateblock_grcblendstat(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_009903_rage_grcstateblock_setstates_rage_grcstateblock_grcrasterizersta(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_009904_rage_gridmonitor_getarea_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009905_rage_monitorconfiguration_ismultihead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009908_rage_monitorconfiguration_update(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009909_rage_grcadaptermanager_getadapter_unsigned_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_009910_rage_grcdevice_getadapterordinal(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_009912_rage_monitorconfiguration_getlandscapemonitor_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009924_rage_grcadaptermanager_shutdownclass(ph: *mut Phantom); }
extern "C" { pub fn f_009964_rage_grcresourcecache_getinstance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009976_rage_grcresourcecache_getstreamingmemory_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009977_rage_grcresourcecache_getstreamingmemory_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_009978_rage_grcresourcecache_gettotalfreememory_rage_memorytype_bool_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_009979_rage_grcresourcecache_setextramemory_rage_memorytype_long(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009986_rage_grcdebugdraw_adddebugoutputex_bool_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_009987_rage_grcdebugdraw_adddebugoutputexraw_bool_rage_color32_char_con(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_010151_rage_dxdiag_dumpdisplayableinfostring_char_ptr_ptr_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010163_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010164_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010168_rage_grcdebugdraw_cross_rage_vec2v_const_ref_float_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_010169_rage_grcdebugdraw_circle_rage_vec2v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010172_rage_grcdebugdraw_text_float_float_rage_color32_char_const_ptr_b(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i64, a4: i32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010179_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010188_rage_grcdebugdraw_boxaxisaligned_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010194_rage_grcdebugdraw_axis_rage_mat34v_const_ref_float_bool_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_010219_rage_grcdebugdraw_adddebugoutputex_bool_rage_color32_char_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_010220_rage_grcdebugdraw_adddebugoutput_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010221_rage_grcdebugdraw_adddebugoutput_rage_color32_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_010222_rage_grcdebugdraw_adddebugoutputseparator_int(ph: *mut Phantom); }
extern "C" { pub fn f_010224_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010225_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010452_rage_ioinput_begin_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010454_rage_iopad_update_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010455_rage_ioeventqueue_updatefrompipe(ph: *mut Phantom); }
extern "C" { pub fn f_010456_rage_iopad_updatedebug(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010457_rage_ioinput_recapturelostdevices(ph: *mut Phantom); }
extern "C" { pub fn f_010458_rage_ioadddeadzone_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_010478_rage_iomouse_setabsoluteonly_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010480_rage_iojoystick_getproductname_unsigned_short_ptr_unsigned_int_c(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010481_rage_iomapper_iomapper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010482_rage_iomapper_map_rage_iomappersource_unsigned_int_rage_iovalue(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_010483_rage_iomapper_removedevicemappings_rage_iovalue_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_010484_rage_iomapper_convertparametertomappervalue_rage_iomappersource(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_010485_rage_iomapper_scan_rage_iosource_ref_rage_iomapper_scanoptions_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_010488_rage_iomapper_clearvalues_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010489_rage_iovalue_setcurrentvalue_float_rage_iosource_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_010490_rage_iomapper_update_unsigned_int_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_010491_rage_iomapper_getmkbaxisnegative_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_010492_rage_iomapper_getmkbaxispositive_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_010494_rage_iomapper_load_rage_fistream_ptr_int(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_010495_rage_iomapper_save_rage_fistream_ptr_int(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_010496_rage_iomapper_getmappedsources_rage_iovalue_const_ref_rage_atfix(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_010497_rage_iomapper_getmappedvalues_rage_iosource_const_ref_rage_atfix(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_010498_rage_iomapper_makemkbaxis_rage_iomapperparameter_rage_iomapperpa(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_010499_rage_iomapper_convertparametertodevicevalue_rage_iomappersource(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_010500_rage_iovalue_isdown_float_rage_iovalue_readoptions_const_ref_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010501_rage_iovalue_historyhelddown_unsigned_int_float_rage_iovalue_rea(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_010503_rage_iovalue_isup_float_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010510_rage_iovalue_isreleasedafterhistoryhelddown_unsigned_int_float_r(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_010511_rage_iovalue_ispressed_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010514_rage_iovalue_isreleased_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010515_rage_iovalue_historyreleased_unsigned_int_unsigned_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_010517_rage_iovalue_setnextvalue_float_rage_iosource_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_010518_rage_iovalue_getnorm_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010519_rage_iovalue_getunboundnorm_rage_iovalue_readoptions_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010523_rage_iopad_setactuatorvalue_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_010737_rage_spdaabb_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_010747_rage_spdtransposedplaneset8_set_rage_grcviewport_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_011093_rage_cutfcascadeshadoweventargs_getposition_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011738_rage_matrix34_toeulerszyx_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011740_rage_fraginst_setdisablebreakable_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011745_rage_fraginst_setresetmatrix_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011750_rage_fraginst_getchildbroken_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011751_rage_fraginst_getcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011759_rage_fraginst_breakoffabove_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_011763_rage_fraginst_deleteabove_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011776_rage_fragtype_getboneindexfromid_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011804_rage_fraginst_forcearticulatedcollidermode(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011805_rage_fragcacheentry_lazyarticulatedinit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012021_rage_fragphysicslodgroup_getlodbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012183_rage_ptxeffectinst_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012202_rage_ptxeffectinst_setoffsetmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012971_rage_phmorphdata_dtorphmorphdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013390_rage_netbandwidthrecorder_netbandwidthrecorder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013429_char_ptr_rage_safecatf_char_char_ptr_unsigned_long_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_013620_rage_netipaddress_op_rage_netipaddress_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013645_rage_netp2pcrypt_key_dtorkey(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013683_rage_netipaddress_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013820_rage_netsocket_dtornetsocket(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013822_rage_nethardware_createsocket_rage_netsocket_ptr_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_013823_rage_netsocket_cansendreceive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013883_rage_netipv4address_netipv4address_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_013938_rage_netsocket_netsocket(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014067_rage_nettimesync_gettime_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014068_rage_nettimesync_hasstarted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014295_rage_rlcreateuuid_unsigned_long_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014316_rage_rlgamerinfo_getgamerhandle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014318_rage_rlgamerinfo_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014319_rage_rlgamerhandle_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014320_rage_rlpc_isinitialized(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014374_rage_rlgamerhandle_export_void_ptr_unsigned_int_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014375_rage_rlgamerhandle_import_void_const_ptr_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014662_rage_rlmetric_dtorrlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014685_rage_rlmetric_rlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015771_rage_computeaxisangleprecise_rage_quatv_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016164_rage_matrix34_dot_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016391_rage_phboundbox_phboundbox_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016660_rage_phwind_getglobalvelocity_rage_windtype_e_rage_vec3v_ref_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_016661_rage_phwind_getlocalvelocity_rage_vec3v_const_ref_rage_vec3v_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_016662_rage_phwind_getlocalvelocity_downwashonly_rage_vec3v_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016909_rage_phlevelnodetree_rage_phlooseoctreenode_testprobe_rage_phseg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_016924_rage_scrprogram_scrprogram_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016931_rage_scrprogram_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016934_rage_scrprogram_getprogram_rage_scrprogramid(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_016943_rage_scrthread_printstacktrace_void_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016944_rage_scrthread_preprintstacktrace(ph: *mut Phantom); }
extern "C" { pub fn f_016947_rage_scrthread_dtorscrthread(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016949_rage_scrthread_getactivethread(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_016950_rage_scrthread_isthisthreadrunningascript(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_016959_rage_scrthread_update_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016962_rage_scrthread_registerunusedcommand_unsigned_long_void_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016967_rage_scrthread_registercommand_unsigned_long_void_ptr_rage_scrth(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016975_rage_scrthread_createthread_rage_scrprogramid_void_const_ptr_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_016978_rage_scrthread_startnewscriptwithargs_rage_scrthread_info_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016981_rage_scrthread_getthread_rage_scrthreadid(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_016982_rage_scrthread_reset_rage_scrprogramid_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_016987_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_sc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_016997_rage_scrthread_kill(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016999_rage_scrthread_killthread_rage_scrthreadid(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_017082_rage_scrthread_info_getvector3_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_017092_rage_scrdebugger_scrdebugger_char_const_ptr_rage_scrthreadid_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_017116_rage_scrdebugger_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017186_rage_crclip_composite_rage_crframe_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_018536_grefcountimpl_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018888_cgamelogic_isrunninggta5map(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_018921_rage_fwentity_getmodelid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018932_cgarages_findgarageindex_char_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_018935_cgarages_isthiscarwithingaragearea_int_centity_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_018938_cgarages_allresprayscloseoropen_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_018977_cgpsslot_clear_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_018995_cgps_cleargpsflagsonscriptexit_rage_scrthreadid(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019004_cgps_renderracetrack_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019121_cvehiclerecordingmgr_crecording_stoprecording_unsigned_char_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019124_cvehiclerecordingmgr_getstreamingmodule(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_019129_cvehiclerecordingmgr_iscarbeingrecorded_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019133_cvehiclerecordingmgr_getplaybackslot_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019135_cvehiclerecordingmgr_stopplaybackrecordedcar_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019136_cvehiclerecordingmgr_isplaybackgoingonforcar_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019137_cvehiclerecordingmgr_getplaybackidforcar_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019139_cvehiclerecordingmgr_setplaybackspeed_cvehicle_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_019140_cvehiclerecordingmgr_pauseplaybackrecordedcar_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019141_cvehiclerecordingmgr_unpauseplaybackrecordedcar_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019142_cvehiclerecordingmgr_changecarplaybacktouseai_cvehicle_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_019143_cvehiclerecordingmgr_skiptoendandstopplaybackrecordedcar_cvehicl(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019145_cvehiclerecordingmgr_skiptimeforwardinrecording_cvehicle_ptr_flo(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_019148_cvehiclerecordingmgr_setadditionalrotationforrecordedvehicle_cve(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019149_cvehiclerecordingmgr_setlocalpositionoffsetforrecordedvehicle_cv(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019150_cvehiclerecordingmgr_setglobalpositionoffsetforrecordedvehicle_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019151_cvehiclerecordingmgr_findpositioninrecording_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_019152_cvehiclerecordingmgr_findtimepositioninrecording_cvehicle_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_019153_cvehiclerecordingmgr_getpositionofcarrecordingattime_int_float_r(ph: *mut Phantom, a0: i32, a1: f32, a2: i64); }
extern "C" { pub fn f_019154_cvehiclerecordingmgr_getrotationofcarrecordingattime_int_float_r(ph: *mut Phantom, a0: i32, a1: f32, a2: i64); }
extern "C" { pub fn f_019155_cvehiclerecordingmgr_gettotaldurationofcarrecording_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_019156_cvehiclerecordingmgr_findindexwithfilenamenumber_int_char_const(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_019157_cvehiclerecordingmgr_hasrecordingfilebeenloaded_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_019158_char_ptr_rage_formatf_char_24ul_char_ref_24ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019232_cstuntjumpmanager_abortstuntjumpinprogress(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019306_cscenarioblockingareas_removescenarioblockingarea_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019325_cspatialarray_findinsphere_rage_vec3v_const_ref_float_cspatialar(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_019450_char_ptr_rage_formatf_char_100ul_char_ref_100ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019453_ailogging_getdynamicentitynamesafe_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019462_cailogmanager_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019747_rage_atvector_rage_atstring_0_unsigned_short_growcapacity_unsign(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019753_cdebugselector_addwidgets_rage_bkbank_ptr_char_const_ptr_rage_da(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_019755_cdebugselector_removewidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019757_cdebugselector_updatecombo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019783_cdebugclipselector_getselectedclip_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019794_cdebugtimelinecontrol_setupslider_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019797_cdebugobjectselector_getselectedmodelid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019802_cdebugpedmodelselector_loadmodelnames(ph: *mut Phantom); }
extern "C" { pub fn f_019804_cdebugpedmodelselector_selectmodel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019826_canimplacementtool_startanim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019836_canimplacementtool_createped_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019837_canimplacementtool_canimplacementtool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019839_canimplacementeditor_deactivatebank(ph: *mut Phantom); }
extern "C" { pub fn f_019841_canimplacementeditor_rebuildbank(ph: *mut Phantom); }
extern "C" { pub fn f_019842_canimplacementeditor_removewidgets(ph: *mut Phantom); }
extern "C" { pub fn f_019843_canimplacementeditor_deletetool_canimplacementtool_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019844_rage_matrix34_inverse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019864_cdebugobjectselector_addwidgets_rage_bkbank_ptr_char_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_020121_canimclipeditor_startpreviewclip(ph: *mut Phantom); }
extern "C" { pub fn f_020448_canimscenemanager_update_animsceneupdatephase(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021156_audscriptaudioentity_stopstream(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021407_audspeechaudioentity_isambientspeechplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021813_audscriptaudioentity_removescript_rage_scrthreadid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021833_audscriptaudioentity_ispedincurrentconversation_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022052_audvehicleaudioentity_setbombbaydoorsopen_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022053_audvehicleaudioentity_sethydraulicsuspensioninputaxis_float_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_022081_audvehicleaudioentity_setisincarmodshop_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022082_audvehicleaudioentity_playvehiclehorn_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_022235_cpedinventory_getweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022626_rage_atmapentry_unsigned_int_unsigned_int_ref_rage_atmap_unsigne(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023888_replayattachmentmanager_pairings_dtorpairings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023998_rage_fwmodelid_fwmodelid_rage_fwmodelid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_024286_creplaymgrinternal_isplaybackflagset_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_024315_creplaymgrproxy_iseditmodeactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026141_rage_iologitechleddevice_dtoriologitechleddevice(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026148_rage_iopadactuatordevice_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026149_rage_iopadactuatordevice_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026150_rage_iorumbleeffect_update_float_rage_ioacutatordevice_ptr_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_026151_rage_iorumbleeffect_setintensity_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_026181_caminterface_findfollowped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026194_caminterface_isfadedout(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026199_caminterface_isfadedin(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026202_caminterface_isspherevisibleingameviewport_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_026205_cqueriableinterface_istaskcurrentlyrunning_int_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_026262_camcinematicdirector_isrenderinganyinvehiclefirstpersoncinematic(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026279_rage_matrix34_toeulersyxz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026636_camcutscenedirector_iscutsceneplaying(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026649_camdebugdirector_getcamframenonconst_cambasecamera_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026683_camgameplaydirector_shoulduse1stpersoncontrols_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026702_ccontrol_debugoutputscriptcallstack(ph: *mut Phantom); }
extern "C" { pub fn f_027008_ctrain_getlinkedtobackward_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027261_camscriptdirector_cleanupcamerareferences(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027266_camscriptdirector_createcamera_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_027267_camscriptdirector_activatecamera_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_028907_rage_matrix34_toeulersxzy_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_028908_rage_matrix34_toeulersyzx_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_028909_rage_matrix34_toeulerszxy_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029054_rage_ragesecpluginmanager_registerpluginfunction_ragesecpluginpa(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_029101_cdebug_setversionnumber(ph: *mut Phantom); }
extern "C" { pub fn f_029265_cgame_issessioninitialized(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_029311_cutscenemanager_shutdowncutscene(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029331_cdebugscene_drawentityboundingbox_centity_ptr_rage_color32_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_029332_cgame_shoulddofullupdaterender(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_029386_cpedvariationinfocollection_getcollectiondrawable_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_029389_cutscenemanager_calculatestreamingoffset_float_float_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_029433_getorcreategroup_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029455_cdebugscene_tunewidgetdata_tunewidgetdata_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_029456_cdebugscene_queuetunewidget_cdebugscene_tunewidgetdata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029512_cdebugscene_getworldpositionundermouse_rage_vector3_ref_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_029607_cpedvariationinfocollection_getracetype_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_029611_cdebugscene_getmousepointing_rage_vector3_ref_rage_vector3_ref_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029612_worldprobe_cshapetestdesc_setresultsstructure_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029613_worldprobe_cshapetestresults_freeintersections(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029689_cutscenemanager_requestcutscene_char_const_ptr_bool_rage_cutsman(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_029692_cdebugtextureviewerinterface_close_bool(ph: *mut Phantom); }
extern "C" { pub fn f_029699_ccutscenebinkoverlayentity_getrendertargetownertag_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029726_camframe_setworldmatrix_rage_matrix34_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029809_cdebugscene_getentityundermouse_rage_vector3_ptr_int_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_029852_cbugstarintegration_createhttpquery(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_029853_char_ptr_rage_formatf_char_260ul_char_ref_260ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_029866_cdebugscene_focusentities_get_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029878_cdebug_dumpentity_centity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029962_cdebugscene_registerentitychangecallback_rage_datcallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030146_tiledscreencapture_isenabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_030212_rage_atarray_cstreamingiteratorslot_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030526_cdebugtextureviewerinterface_selecttxd_ctxdref_const_ref_bool_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_030565_ctxdref_ctxdref_eassettype_int_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_030711_rage_atmap_unsigned_long_bool_rage_atmaphashfn_unsigned_long_rag(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030722_ceventgroupped_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030740_geteventscriptaigroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030742_geteventscripterrorsgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030744_ceventnetwork_spewtotty_impl_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030746_geteventscriptnetworkgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030751_ceventnetworkplayerscript_ceventnetworkplayerjoinscript_153_coll(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_030801_cevent_cevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030819_cped_isinjured_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030974_cinformfriendsevent_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030978_cinformgroupevent_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_031055_ceventscriptcommand_ceventscriptcommand_int_rage_aitask_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_031056_ceventscriptcommand_dtorceventscriptcommand(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031158_ceventfirenearby_dtorceventfirenearby(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033790_gfxvalue_objectinterface_objectrelease_gfxvalue_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034946_cloadingscreens_areactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034949_chudtools_getwidescreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034950_chudtools_getaspectratio_bool(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_034954_chudtools_adjustnormalized16_9valuesforcurrentaspectratio_ewides(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_034958_chudtools_adjustforwidescreen_ewidescreen_format_rage_vector2_pt(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_035029_chudtools_calculatehudposition_rage_vector2_rage_vector2_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_035059_cpedinventory_getweaponammo_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_035079_chudcolour_getrgba_ehud_colours(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035081_cnewhud_shouldwheelblockcamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035087_chudtools_issuperwidescreen_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035253_centitypoolindexforblip_centitypoolindexforblip_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_035267_cminimap_createblip_bool_eblip_type_centitypoolindexforblip_cons(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_035268_cminimap_createblip_bool_eblip_type_rage_vector3_const_ref_eblip(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_035271_cminimap_getblipnamevalue_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035274_cminimap_getuniqueblipused_cminimapblip_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035304_cminimap_switchoffwaypoint_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035305_cminimap_isflagset_cminimapblip_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_035309_cminimap_common_getcolourfromblipsettings_eblip_colours_bool_ehu(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_035310_cminimap_getblipcolourvalue_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035312_cminimap_getblipalphavalue_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035315_cminimap_getblipobjectnameid_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035317_cminimap_setblipparameter_eblip_params_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_035320_cminimap_setblipparameter_eblip_params_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_035322_cminimap_setblipparameter_eblip_params_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_035323_cminimap_isblipidinuse_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035324_cminimap_getblip_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_035328_cminimap_getblipattachedtoentity_centity_const_ptr_eblip_type_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_035346_cminimap_getactivewaypointid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035354_cminimap_switchonwaypoint_float_float_unsigned_short_bool_bool_e(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_035403_char_ptr_rage_formatf_char_20ul_char_ref_20ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_035494_cbusyspinner_on_char_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036031_cscaleformmgr_requestremovemovie_int_emovieownertags(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_036033_cscaleformmgr_rendermovie_int_float_bool_bool_float(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i32, a4: f32); }
extern "C" { pub fn f_036034_cscaleformmoviewrapper_isactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036038_cscaleformmgr_ismovieactive_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036044_cscaleformmgr_beginmethodinternal_int_escaleform_base_class_char(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_036045_cscaleformmgr_endmethodinternal_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036050_cscaleformmgr_endmethod_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036053_cscaleformmgr_addparamint_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036056_cscaleformmgr_addparamstring_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036154_cpausemenu_close(ph: *mut Phantom); }
extern "C" { pub fn f_036162_cscaleformmoviewrapper_param_addparam(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036184_cpausemenu_togglepauserenderphases_bool_enum_pause_menu_pausedrp(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_036254_cpausemenu_isactive_pm_activeflags(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036288_cpausemenu_opencorrectmenu_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036372_cscaleformmgr_changemovieparams_int_rage_vector2_const_ref_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_036572_rage_atarray_rage_atarray_rage_athashvalue_0_unsigned_short_0_un(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_036614_ctextinputbox_open_rage_iovirtualkeyboard_params_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036617_ctextinputbox_close(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037248_cscaleformmgr_beginmethod_int_escaleform_base_class_char_const_p(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_037302_cscaleformmgr_setscriptrequired_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037317_cscaleformmgr_callmethod_int_escaleform_base_class_char_const_pt(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_037507_cgamestream_postmessagetext_char_const_ptr_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64, a8: f32, a9: i64, a10: i32, a11: i32, a12: i32) -> i32; }
extern "C" { pub fn f_037510_cgamestream_poststats_char_const_ptr_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_037511_cgamestream_postticker_char_const_ptr_bool_bool_bool_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_037513_cgamestream_postaward_char_const_ptr_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_037514_cgamestream_postcrewtag_bool_bool_char_const_ptr_int_bool_char_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i64, a8: i64, a9: i32) -> i32; }
extern "C" { pub fn f_037515_cgamestream_postcrewrankup_char_const_ptr_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_037516_cgamestream_postversus_char_const_ptr_char_const_ptr_int_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_037517_cgamestream_postreplay_cgamestream_efeedreplaystate_char_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: f32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_037519_cgamestream_postunlock_char_const_ptr_char_const_ptr_int_bool_eh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_037656_cwanted_setmaximumwantedlevel_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037659_cseatmanager_countpedsinseats_bool_bool_bool_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_037660_cmodelinfo_getassetstreamflags_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037661_cmodelinfo_requestassets_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037662_cmodelinfo_haveassetsloaded_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037663_cmodelinfo_setassetsaredeletable_rage_fwmodelid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037681_cclock_passtime_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037753_rage_atarray_cscriptstacksizedataentry_0_unsigned_short_grow_uns(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037757_cgameconfig_getconfigmaxnumberofscripttextlines_char_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_037759_cgameconfig_getthreadpriority_rage_atnonfinalhashstring_rage_sys(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037888_cseatmanager_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_037946_cweather_getprevtype(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037951_cweather_forcetypeclear_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_037989_cmapareas_getareanamehashfromposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037990_cmapareas_getareaindexfromposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038010_cmapzonemanager_getzoneatcoords_rage_vector3_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038053_cwantedresponseoverrides_getinstance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_038054_cwantedresponseoverrides_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038119_cdispatchspawnproperties_getinstance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_038130_cmodelinfo_getbasemodelinfo_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038190_cdispatchmanager_flush(ph: *mut Phantom); }
extern "C" { pub fn f_038366_cincidentmanager_clearallincidentsoftype_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_038636_cbasemodelinfo_getassetlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038699_cmodelinfo_getmodelinfofromlocalindex_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_038712_cmodelinfo_haveassetsloaded_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038713_cmodelinfo_areassetsloading_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038714_cmodelinfo_areassetsrequested_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038716_cmodelinfo_getassetsaredeletable_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038717_cmodelinfo_removeassets_rage_fwmodelid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038718_cmodelinfo_getassetrequiredflag_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038719_cmodelinfo_setassetrequiredflag_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038720_cmodelinfo_clearassetrequiredflag_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038721_cmodelinfo_getobjectanddependencies_rage_fwmodelid_rage_atarray(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_038722_cmodelinfo_getobjectanddependenciessizes_rage_fwmodelid_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_038725_cvehiclemodelinfo_calculatemaxflatvelocityestimate_chandlingdata(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038734_centryexitpoint_getentrypointposition_centity_const_ptr_cped_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_038739_cmodelseatinfo_getentrypointindexforseat_int_centity_const_ptr_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_038751_cseatmanager_getpedsseatindex_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038757_cseatmanager_getlastpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038758_cseatmanager_countfreeseats_bool_bool_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_038873_cpedmodelinfo_getarehdfilesloaded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038874_cpedmodelinfo_hashelmetprop_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038875_cpedmodelinfo_getpedinstance_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038876_cpedmodelinfo_getdistancesqrtoclosestinstance_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_038926_cvehiclemodelinfo_getvehicleinstance_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038976_cvehiclevariationinstance_setmodindex_evehiclemodtype_unsigned_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_038978_cvehiclevariationinstance_getmodindexfortype_evehiclemodtype_cve(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_038981_cvehiclevariationinstance_togglemod_evehiclemodtype_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_038982_cvehiclevariationinstance_istogglemodon_evehiclemodtype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038992_cvehiclemodelinfo_gethdfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039014_cvehiclemodelinfo_getentrypointinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039023_cvehiclemodelinfo_gettrailer_unsigned_int_cvehiclemodelinfo_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_039024_cvehiclemodelinfo_getrandomdriver_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039128_cvehiclekit_getliveryname_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039129_cvehiclekit_getlivery2name_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039203_cvehiclevariationinstance_cleanuppreloadmods_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039208_cvehiclevariationinstance_haspreloadmodsfinished(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039211_cvehiclevariationinstance_getkit_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039212_cvehiclevariationinstance_getwheeltype_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_039213_cvehiclevariationinstance_getnummodsfortype_evehiclemodtype_cveh(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_039214_cvehiclevariationinstance_isgen9exclusivevehiclemod_evehiclemodt(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_039216_cvehiclevariationinstance_setmodindexfortype_evehiclemodtype_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_039217_cvehiclevariationinstance_setpreloadmodfortype_evehiclemodtype_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_039221_cvehiclevariationinstance_isboneturnedoff_cvehicle_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_039224_cvehiclevariationinstance_setmodcolor1_evehiclecolortype_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_039225_cvehiclevariationinstance_setmodcolor2_evehiclecolortype_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_039226_cvehiclevariationinstance_getmodcolor1_evehiclecolortype_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_039227_cvehiclevariationinstance_getmodcolor2_evehiclecolortype_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_039228_cvehiclevariationinstance_getmodcolor1name_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039229_cvehiclevariationinstance_getmodcolor2name_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039230_cvehiclevariationinstance_setwheeltype_evehiclewheeltype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039231_cvehiclevariationinstance_havemodsstreamedin_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039232_cvehiclevariationinstance_getmodifier_evehiclemodtype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039233_cvehiclevariationinstance_getmodshoplabelfortype_evehiclemodtype(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_039235_cvehiclevariationinstance_setxenonlightcolor_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039246_cvehiclestreamrendergfx_getfragindex_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039573_cpathservergta_updatedynamicobject_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_039626_rage_pgdictionary_rage_crexpressions_lookup_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039627_cobject_initanimlazy(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039630_cweapon_sfireparams_dtorsfireparams(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039639_cpathservergta_maybeadddynamicobject_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039688_cobjectpopulation_destroyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039693_cdummyobject_disable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039704_cobject_printskeletondata(ph: *mut Phantom); }
extern "C" { pub fn f_039715_cobject_setweapon_cweapon_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039757_cobjectpopulation_converttodummyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039758_cobjectpopulation_manageobjectpopulation_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_039761_cobjectpopulation_createobject_rage_fwmodelid_eentityownedby_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_039763_cnavmeshdataexporter_willexportcollision(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_039882_cpathserver_getclosestpositionforped_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_039889_cpathserver_getnavmeshmeshregionscriptid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_039890_cpathserver_removenavmeshregion_rage_navmeshrequiredregion_rage(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_040139_cpedgroupmembership_getmember_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_040142_cpedgroup_islocallycontrolled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040143_cpedgroup_setdirty(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040157_cpedgroupmembership_countmembers_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040169_cpedgroupmembership_removemember_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040175_cpedgroupmembership_isleader_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040189_cpedgroups_addgroup_epoptype_cgamescripthandler_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_040191_cpedgroups_removegroup_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_040192_cpedgroups_init(ph: *mut Phantom); }
extern "C" { pub fn f_040194_cpedgroups_getpedsgroup_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040204_carea_carea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040207_carea_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040210_carea_set_rage_vector3_const_ref_rage_vector3_const_ref_float_ce(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32); }
extern "C" { pub fn f_040211_carea_setassphere_rage_vector3_const_ref_float_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_041056_findplayerped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041060_cpeddebugvisualisermenu_getfocusped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041116_cpedintelligence_flushimmediately_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041117_cpedintelligence_addtoeventhistory_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041235_cplayerinfo_areanycontrolsotherthanfrontenddisabled(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041296_cpedfactory_destroyped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041385_cpedintelligence_findtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041416_cpedintelligence_recordeventforscript_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041440_cpedintelligence_setpedperception_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041454_cpedmotiondata_setcurrentmoveblendratio_rage_vector2_ref_float_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_041471_cpedpopulation_setscenariopeddensitymultipliers_float_float(ph: *mut Phantom, a0: f32, a1: f32); }
extern "C" { pub fn f_041472_cpedpopulation_pednonremovalareaclear(ph: *mut Phantom); }
extern "C" { pub fn f_041473_cpedpopulation_pednoncreationareaclear(ph: *mut Phantom); }
extern "C" { pub fn f_041474_cpedpopulation_setallowcreaterandomcops_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041475_cpedpopulation_setallowcreaterandomcopsonscenarios_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041476_cpedpopulation_setallowdummyconversions_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041479_cpedpopulation_flushpedqueues(ph: *mut Phantom); }
extern "C" { pub fn f_041496_findplayervehicle_cplayerinfo_ptr_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041513_cpedpopulation_setambientpeddensitymultiplier_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_041537_cpopulationgroup_isindexmember_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041556_cpopgrouplist_findpedgroupfromnamehash_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_041557_cpedpopulation_getcopsinarea_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_041565_cpopschedule_setoverridepedgroup_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041571_cpedpopulation_getpedlodcounts_int_ref_int_ref_int_ref_int_ref_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i64); }
extern "C" { pub fn f_041624_cpedscriptedtaskrecorddata_set_cped_ptr_int_ceventscriptcommand(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_041625_cpedscriptedtaskrecord_getvacantslot(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041663_cplayerinfo_setmayonlyenterthisvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041700_cplayerinfo_getpos(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041703_cplayerinfo_isrestartingafterdeath(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041704_cplayerinfo_isrestartingafterarrest(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041708_cplayerinfo_setplayercontrol_bool_bool_float_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_041795_cpopschedule_setoverridevehgroup_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041797_cpopgrouplist_findvehgroupfromnamehash_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_041799_cpopulationgroup_getmodelindex_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041809_cpopzones_findsmallestforposition_rage_vector3_const_ptr_ezoneca(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_041823_cpopcycle_pickmostwantinggroupofmodels_cloadedmodelgroup_ref_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_041824_cpopzones_getpopzone_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041825_cpopzones_getpopzoneid_cpopzone_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041833_cpopzones_getsearchinfofrompopzone_cpopzone_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041835_cpopzones_getpopzone_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_041836_cpopzones_getscumminess_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041837_cpopzones_setenabled_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_041990_ctaskinfo_ctaskinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042018_cqueriableinterface_istasksentovernetwork_int_bool_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_042019_cqueriableinterface_cantaskbegiventononlocalpeds_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_042025_crelationshipgroup_setrelationship_erelationtype_crelationshipgr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_042026_crelationshipgroup_clearrelationship_crelationshipgroup_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042036_crelationshipmanager_addrelationshipgroup_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042038_crelationshipmanager_findrelationshipgroup_rage_atnonfinalhashst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042039_crelationshipmanager_removerelationshipgroup_crelationshipgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042040_crelationshipmanager_findrelationshipgroup_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_042055_cwildlifemanager_checkwildlifemultiplayerspawnconditions_unsigne(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_042176_cpeddamagemanager_getclanidforped_cped_const_ptr_long_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_042179_cpeddamagemanager_addpeddamagepack_cped_ptr_rage_atnonfinalhashs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_042254_void_rage_dlc_addinternal_void_ptr_bool_bool_void_ptr_bool_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042400_cpedstreamrendergfx_getdrawable_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042425_cpedvariationpack_extractcomponent_rage_pgdictionary_rage_rmcdra(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_042433_cpedvariationpack_loadslods(ph: *mut Phantom); }
extern "C" { pub fn f_042484_char_ptr_rage_formatf_char_200ul_char_ref_200ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042791_cpickup_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042799_cpickup_setuselightoverrideforglow(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042811_cpickupdata_getreward_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042812_cpickup_setforcealphaanduseambientscale(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042817_worldprobe_debugfindgroundzfor3dcoord_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32, a5: i64, a6: i64) -> f32; }
extern "C" { pub fn f_042818_worldprobe_cshapetestmanager_debugsubmittest_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_042821_worldprobe_cshapetestmanager_aborttest_worldprobe_cshapetestresu(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042878_cpickupmanager_removeallpickups_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_042885_cpickupmanager_createpickup_unsigned_int_rage_matrix34_const_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_042888_cpickupmanager_createpickupfromcurrentweapon_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042890_cpickupmanager_shouldusemppickups_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043074_ctunableobjectmanager_gettuningentry_rage_atnonfinalhashstring_c(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043085_worldprobe_cshapetestdesc_setexcludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043091_gtafragtype_gtafragtype_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_043223_cphysics_clearmanifoldsforinst_rage_phinst_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043224_cphysics_loadaboutposition_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043230_int_ref_rage_atmap_unsigned_int_int_rage_atmaphashfn_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_043233_worldprobe_cshapetestbounddesc_setboundfromentity_centity_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043234_worldprobe_cshapetestdesc_setincludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043335_worldprobe_debugfindgroundzforcoord_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_043349_worldprobe_cshapetestdesc_setexcludeinstance_rage_phinst_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043415_gtadrawable_gtadrawable_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_043422_chorizonobjects_culleverything_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043485_water_iscameraunderwater(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043486_cmirrors_getismirrorvisible_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043508_occlusionqueries_oqgetqueryresult_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_043528_cgrassrenderer_getglobalplayerpos(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_043663_postfx_setldr16bithdr16bit(ph: *mut Phantom); }
extern "C" { pub fn f_043670_csprite2d_begincustomlist_csprite2d_customshadertype_rage_grctex(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_043675_csprite2d_endcustomlist(ph: *mut Phantom); }
extern "C" { pub fn f_043753_csprite2d_setgeneralparams_rage_vector4_const_ref_rage_vector4_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043877_void_rage_dlc_addinternal_void_ptr_bool_bool_bool_bool_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_043902_crendertargetmgr_getnamedrendertarget_rage_atfinalhashstring_con(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043945_cdrawlistmgr_clothcleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043946_cdrawlistmgr_charclothcleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043956_csprite2d_clearrenderstate(ph: *mut Phantom); }
extern "C" { pub fn f_043975_csprite2d_delete(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043977_csprite2d_setrenderstate_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043983_csprite2d_drawrect_float_float_float_float_float_rage_color32_co(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i64); }
extern "C" { pub fn f_043984_csprite2d_drawrectgui_rage_fwrect_const_ref_rage_color32_const_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043987_csprite2d_drawswitch_bool_bool_float_rage_vector2_const_ref_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64); }
extern "C" { pub fn f_043988_csprite2d_movetoscreengui_rage_vector2_ptr_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044104_water_requestglobalwaterxmlfile_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044122_water_getwaterlevel_rage_vector3_const_ref_float_ptr_bool_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_044123_water_getwaterlevelnowaves_rage_vector3_const_ref_float_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_044125_water_testlineagainstwater_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_044196_water_modifydynamicwaterspeed_float_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_044197_water_addextracalmingquad_int_int_int_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: f32) -> i32; }
extern "C" { pub fn f_044198_water_removeextracalmingquad_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044199_water_setscriptdeepoceanscaler_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_044200_water_setscriptcalmedwaveheightscaler_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_044291_crenderphasereflection_setreflectionmapext_rage_grctexture_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044400_copyoffshader_ccustomshadereffectbase_const_ptr_unsigned_int_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_044402_ccustomshadereffectdc_ccustomshadereffectdc_ccustomshadereffectb(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_044527_lights_calculatetimefade_float_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_044889_creplaycoordinator_isactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045006_creplaycoordinator_isexportingtovideofile(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045244_cgenericgamestorage_queueautosave(ph: *mut Phantom); }
extern "C" { pub fn f_045251_cgenericgamestorage_isstoragedevicebeingaccessed(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045679_cscriptsavedata_closescriptstruct_gtathread_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045797_cped_getcurrentmotiontask_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045821_cped_printskeletondata(ph: *mut Phantom); }
extern "C" { pub fn f_045844_cped_setdefaultdecisionmaker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045856_cped_deactivatephysicsanddealwithragdoll_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045883_cped_setpedoutofvehicle_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_045913_cped_isfirstpersonshootermodeenabledforplayer_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045939_cped_setvarrandom_unsigned_int_unsigned_int_rage_atfixedbitset_3(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_045962_cped_setpreloaddata_epedvarcomp_unsigned_int_unsigned_int_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_045964_cped_cleanuppreloaddata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045973_cped_issuperlodfromindex_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045977_cped_setdesiredheading_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046002_cped_isinfirstpersonvehiclecamera_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_046091_cped_debugattachpedtophysical_char_const_ptr_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i64, a8: i64, a9: f32, a10: f32); }
extern "C" { pub fn f_046103_cped_getspecialability_eplayerspecialabilityslot(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046291_canimatedbuilding_printskeletondata(ph: *mut Phantom); }
extern "C" { pub fn f_046316_cbuilding_cbuilding_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046325_cgameworld_findfollowplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_046327_cplayerswitchinterface_isactive_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_046359_cdatafilemgr_load_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046362_cdatafilemgr_expandfilename_char_const_ptr_char_ptr_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046367_cdatafilemgr_getcontentchangeset_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_046410_centity_getinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046438_cgameworld_add_centity_ptr_rage_fwinteriorlocation_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046442_cinteriorproxy_getfromlocation_rage_fwinteriorlocation_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046445_cportaltracker_collectstack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046447_cinteriorproxy_createlocation_cinteriorproxy_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046459_cdynamicentity_getscriptthatcreatedme_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046463_cdynamicentity_destroyscriptextension(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046467_cgameworld_remove_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046484_centity_setisvisibleformodule_eisvisiblemodule_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046495_centity_isinpathserver_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046496_centity_wasunabletoaddtopathserver(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046499_centity_getcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046501_centity_getisfixedroot_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046502_centity_getphysarch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046522_centity_iswithinarea_float_float_float_float_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32) -> i32; }
extern "C" { pub fn f_046537_getentitytypestr_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_046538_centity_getguid_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046545_centity_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046599_centityiterator_centityiterator_int_centity_const_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_046600_centityiterator_getnext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046682_cmapfilemgr_cleanup(ph: *mut Phantom); }
extern "C" { pub fn f_046692_cmapfilemgr_reset(ph: *mut Phantom); }
extern "C" { pub fn f_046695_ctexlod_flushallupgrades(ph: *mut Phantom); }
extern "C" { pub fn f_046696_cfileloader_executecontentchangeset_cdatafilemgr_contentchangese(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046698_cmapfilemgr_setmapfiledependencies(ph: *mut Phantom); }
extern "C" { pub fn f_046703_cfileloader_revertcontentchangeset_cdatafilemgr_contentchangeset(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046739_cextracontentmanager_getassetpathforworkingdlc_char_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046893_rage_atbinarymap_char_const_ptr_unsigned_int_safeinsert_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_047012_cfocusentitymgr_getpos(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047013_cfocusentitymgr_getentity_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047019_cinstancepriority_update(ph: *mut Phantom); }
extern "C" { pub fn f_047025_cloadscene_stop(ph: *mut Phantom); }
extern "C" { pub fn f_047026_cstreamvolume_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047032_cinteriorproxy_setrequestedstate_cinteriorproxy_erequest_module(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_047033_cinteriorproxy_findactiveintersecting_rage_fwissphereintersectin(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_047034_cportaltracker_probeforinteriorproxy_rage_vector3_const_ref_cint(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32) -> i32; }
extern "C" { pub fn f_047041_cmapchange_execute_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_047042_cmapchange_findentitiesfromworldsearch_rage_atarray_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_047044_cgameworld_forallentitiesintersecting_rage_fwsearchvolume_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_047045_cmapchangemgr_add_unsigned_int_unsigned_int_rage_spdsphere_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_047046_cmapchangemgr_remove_unsigned_int_unsigned_int_rage_spdsphere_co(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047047_rage_atarray_cmapchange_0_unsigned_short_deletefast_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047060_cphysical_activatephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047073_cphysical_sethealth_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047080_cphysical_hasbeendamagedbyhash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047081_cphysical_hasbeendamagedbygeneralweapontype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047118_cphysical_debugattachtophysicalusingphysics_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64, a10: i64, a11: f32, a12: i32, a13: f32, a14: f32) -> i32; }
extern "C" { pub fn f_047119_cphysical_debugattachtophysicalusingphysics_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: f32); }
extern "C" { pub fn f_047120_cphysical_debugattachtoworldusingphysics_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: f32, a9: i32); }
extern "C" { pub fn f_047127_cphysical_removedeletewithparentprogenyfromlist_centity_ptr_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_047296_cstreamingbucketset_waitforsorting(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047298_ctexlod_unbindallboundtxds(ph: *mut Phantom); }
extern "C" { pub fn f_047305_cgameworld_getmainplayerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047333_clodmgr_withinvisiblerange_centity_ptr_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_047355_csceneloader_loadscene_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047376_ccompentity_ccompentity_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_047400_ccompentity_updatecompentitiesusinggroup_int_ecompentitystate(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_047707_cinteriorproxy_cleanupinteriorimmediately(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047742_cinteriorinst_addinteriorcontents_int_bool_bool_rage_atarray_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_047869_cportalvistracker_getinteriorlocation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047880_cgameworld_findlocalplayercoors(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047914_ciplcullbox_resetculled(ph: *mut Phantom); }
extern "C" { pub fn f_047917_ciplcullbox_update_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047919_ciplcullbox_setboxenabled_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_047920_ciplcullbox_isboxenabled_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_047921_ciplcullbox_generateculllistforbox_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047929_cscenestreamermgr_dumpstreaminginterests_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047950_cstreamvolumemgr_getstreamingindices_rage_atarray_rage_strindex(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047971_cgameworld_findobjectskindacolliding_rage_vector3_const_ref_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32); }
extern "C" { pub fn f_047973_cgameworld_clearexcitingstufffromarea_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_047976_cgameworld_extinguishallcarfiresinarea_rage_vector3_const_ref_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_047982_cgameworld_findlocalplayervehicle_bool(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047990_cgameworld_setallcarscanbedamaged_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047994_cgameworld_createplayer_rage_vector3_ref_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_048010_cgameworldheightmap_enableheightmap_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_048062_cgameworldwaterheight_getwaterheightatpos_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_048339_networkutils_isnetworkcloneormigrating_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_050589_graphics_commands_commanddrawrect_float_float_float_float_int_in(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_050603_graphics_commands_drawspritetex_rage_grctexture_ptr_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i64, a13: i64, a14: i32, a15: i32); }
extern "C" { pub fn f_051564_hud_commands_commandsettextrenderid_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_051621_hud_commands_changeblipnamefromtextfile_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_060585_ctaskmoveseekentity_213_centityseekposcalculatorstandard_ccloned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: i32) -> i64; }
extern "C" { pub fn f_060662_ctaskmoveseekentity_217_centityseekposcalculatorradiusangleoffse(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: i32) -> i64; }
extern "C" { pub fn f_060664_ctaskmoveseekentity_215_centityseekposcalculatorxyoffsetrotated(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: i32) -> i64; }
extern "C" { pub fn f_060665_ctaskmoveseekentity_216_centityseekposcalculatorxyoffsetfixed_cc(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: i32) -> i64; }
extern "C" { pub fn f_061266_vehicle_commands_commandcreatevehicle_int_rage_scrvector_const_r(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_061267_vehicle_commands_commanddeletevehicle_int_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_061280_vehicle_commands_commandsetallvehiclegeneratorsactiveinarea_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_061285_vehicle_commands_commandsetvehicleusecutscenewheelcompression_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_061297_vehicle_commands_commandisseatwarponly_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_061307_vehicle_commands_commandsetvehicledoorslockedforplayer_int_int_b(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_061309_vehicle_commands_commandsetvehicledoorslockedforallplayers_int_b(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061310_vehicle_commands_commandsetvehicledoorslockedfornonscriptplayers(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061311_vehicle_commands_commandsetvehicledoorslockedforteam_int_int_boo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_061312_vehicle_commands_commandsetvehicledoorslockedforallteams_int_boo(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061316_vehicle_commands_commandsetvehicletimedexplosion_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_061320_vehicle_commands_commanddetonatevehiclephoneexplosivedevice(ph: *mut Phantom); }
extern "C" { pub fn f_061350_vehicle_commands_commandsetboatanchor_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061353_vehicle_commands_commandsetboatremainsanchoredwhileplayerisdrive(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061365_vehicle_commands_commandisvehicleseatfree_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_061366_vehicle_commands_commandgetpedinvehicleseat_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_061378_vehicle_commands_commandsetvehicletyreburst_int_int_bool_float(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32); }
extern "C" { pub fn f_061379_vehicle_commands_commandsetvehicledoorsshut_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061380_vehicle_commands_canvehiclebegrabbedbyscript_cvehicle_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_061387_vehicle_commands_commandstartplaybackrecordedvehicle_int_int_cha(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_061388_vehicle_commands_commandstartplaybackrecordedvehiclewithflags_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_061389_vehicle_commands_commandforceplaybackrecordedvehicleupdate_int_b(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061397_vehicle_commands_commandsetvehicledooropen_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_061398_vehicle_commands_commandsetvehicledoorautolock_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_061400_vehicle_commands_commandsmashvehiclewindow_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061408_vehicle_commands_commandattachvehicletotrailer_int_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_061412_vehicle_commands_commandstabiliseentityattachedtohelicopter_int(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_061413_vehicle_commands_commandattachvehicleontotrailer_int_int_rage_sc(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: f32); }
extern "C" { pub fn f_061414_vehicle_commands_commanddetachvehiclefromtrailer_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_061415_vehicle_commands_commandisvehicleattachedtotrailer_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_061416_vehicle_commands_commandsetvehicletyrefixed_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061417_vehicle_commands_commandsetvehiclenumberplatetext_int_char_const(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_061426_vehicle_commands_commandcreatemissiontrain_int_rage_scrvector_co(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_061427_vehicle_commands_commandallowtraintoberemovedbypopulation_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_061431_vehicle_commands_commandsetrendertrainasderailed_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061437_vehicle_commands_commandstartplaybackrecordedvehicleusingai_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_061446_vehicle_commands_commandsetvehiclemodelissuppressed_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061447_vehicle_commands_basegetrandomcarinspherenosave_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_061451_vehicle_commands_commandgetclosestvehicle_rage_scrvector_const_r(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_061454_vehicle_commands_commanddeletemissiontrain_int_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_061455_vehicle_commands_commandsetmissiontrainasnolongerneeded_int_ref(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_061457_vehicle_commands_commandhasvehicleassetloaded_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_061463_vehicle_commands_commandbringvehicletohalt_int_float_int_bool(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_061464_vehicle_commands_commandstopbringingvehicletohalt_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_061466_vehicle_commands_commandsetforkliftforkheight_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061467_vehicle_commands_commandisentityattachedtohandlerframe_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_061468_vehicle_commands_commandisanyentityattachedtohandlerframe_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_061469_vehicle_commands_commandfindhandlervehiclecontainerisattachedto(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_061470_vehicle_commands_commandishandlerframelinedupwithcontainer_int_i(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_061471_vehicle_commands_commandattachcontainertohandlerframewhenlinedup(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061472_vehicle_commands_commanddetachcontainerfromhandlerframe_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_061491_vehicle_commands_sethelibladesspeed_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061496_vehicle_commands_commandsetdontallowplayertoentervehicleiflocked(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061505_vehicle_commands_commandsetvehicleengineon_int_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_061508_vehicle_commands_commandsetvehicledoorcontrol_int_int_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32); }
extern "C" { pub fn f_061509_vehicle_commands_commandsetvehicledoorlatched_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_061512_vehicle_commands_commandgetvehicleestimatedmaxspeed_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_061513_vehicle_commands_commandgetvehiclemaxbraking_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_061514_vehicle_commands_commandgetvehiclemaxtraction_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_061515_vehicle_commands_commandgetvehicleacceleration_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_061516_vehicle_commands_commandgetvehiclemodelmaxtraction_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_061517_vehicle_commands_commandgetflyingvehiclemodelagility_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_061520_vehicle_commands_commandgetvehiclemodelaccelerationmaxmods_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_061522_vehicle_commands_commandgetvehiclemodelmaxbrakingmaxmods_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_061529_vehicle_commands_commandgetpedusingvehicledoor_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_061530_vehicle_commands_commandsetvehicledoorshut_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_061531_vehicle_commands_commandsetvehicledoorbroken_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_061533_vehicle_commands_commandsetvehiclerudderbroken_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061541_vehicle_commands_commandsetvehiclecolourcombination_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061542_vehicle_commands_commandgetvehiclecolourcombination_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_061561_vehicle_commands_commandisvehicledoordamaged_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_061562_vehicle_commands_commandsetdoorallowedtobebrokenoff_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_061564_vehicle_commands_commandisvehiclebumperbrokenoff_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_061566_vehicle_commands_commandisentrypointforseatclear_int_int_int_boo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_061568_vehicle_commands_commandcanshuffleseat_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_061570_vehicle_commands_commandgetvehiclemodelvalue_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_061586_vehicle_commands_commandsetvehiclelimitspeedwhenplayerinactive_i(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061589_vehicle_commands_commandremovevehiclesfromgeneratorsinarea_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_061592_vehicle_commands_commandturnoffvehicleextra_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_061594_vehicle_commands_commandisextrabrokenoff_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_061597_vehicle_commands_commandsetvehicledamage_int_rage_scrvector_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_061600_vehicle_commands_commandsetvehicleenginehealth_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061601_vehicle_commands_commandsetplaneenginehealth_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061605_vehicle_commands_commandsetvehiclebodyhealth_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061613_vehicle_commands_commandsoundcarhorn_int_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_061626_cvehicledeformation_lockdamagetexture_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_061637_vehicle_commands_commandrolldownwindows_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_061640_vehicle_commands_commandishelipartbroken_int_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_061661_vehicle_commands_commandsetvehicletowtruckarmposition_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061662_vehicle_commands_snapvehicleparttopositionhelper_evehiclegadgett(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_061665_vehicle_commands_commandsetvehicleturrettarget_int_int_rage_scrv(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_061667_vehicle_commands_commandsetvehicleturretspeedthisframe_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061668_vehicle_commands_commanddisableturretmovementthisframe_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_061673_vehicle_commands_commandattachvehicletotowtruck_int_int_int_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_061679_vehicle_commands_commanddetachvehiclefromtowtruck_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061680_vehicle_commands_commanddetachvehiclefromanytowtruck_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_061681_vehicle_commands_commandisvehicleattachedtotowtruck_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_061682_vehicle_commands_commandgetentityattachedtotowtruck_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_061683_vehicle_commands_commanddetachvehiclefromcargobob_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061684_vehicle_commands_commanddetachentityfromcargobob_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061685_vehicle_commands_commanddetachvehiclefromanycargobob_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_061686_vehicle_commands_commandisvehicleattachedtocargobob_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_061687_vehicle_commands_commandgetvehcileattachedtocargobob_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_061688_vehicle_commands_commandgetentityattachedtocargobob_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_061689_vehicle_commands_attachphysicaltocargobob_cvehicle_ptr_cphysical(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_061690_vehicle_commands_commandattachvehicletocargobob_int_int_int_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_061691_vehicle_commands_commandattachentitytocargobob_int_int_int_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_061692_vehicle_commands_commandcargobobforcenovehicledetach_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061693_vehicle_commands_commandcargobobsetexcludefrompickupentity_int_i(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061694_vehicle_commands_commandcancargobobpickupentity_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_061695_vehicle_commands_commandgetattachedpickuphookposition_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_061696_vehicle_commands_commanddoescargobobhavepickuprope_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_061697_vehicle_commands_commandcreatepickupropeforcargobob_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061699_vehicle_commands_commandsetpickupropelengthforcargobob_int_float(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: i32); }
extern "C" { pub fn f_061700_vehicle_commands_commandsetpickupropelengthwithoutcreatingropefo(ph: *mut Phantom, a0: i32, a1: f32, a2: f32); }
extern "C" { pub fn f_061701_vehicle_commands_commandsetcargobobpickupropedampingmultiplier_i(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061703_vehicle_commands_commanddoescargobobhavemagnet_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_061704_vehicle_commands_commandsetcargobobmagnetactive_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061705_vehicle_commands_commandsetcargobobmagnetstrength_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061706_vehicle_commands_commandsetcargobobmagnetfalloff_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061707_vehicle_commands_commandsetcargobobmagnetreducedstrength_int_flo(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061708_vehicle_commands_commandsetcargobobmagnetreducedfalloff_int_floa(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061709_vehicle_commands_commandsetcargobobmagnetpullstrength_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061710_vehicle_commands_commandsetcargobobmagnetpullropelength_int_floa(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061711_vehicle_commands_commandsetcargobobmagnetsettargetedmode_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061712_vehicle_commands_commandsetcargobobmagnetsetambientmode_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_061713_vehicle_commands_commandsetcargobobmagnetensurepickupentityuprig(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061718_vehicle_commands_commandsetvehiclehandbrake_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061724_vehicle_commands_commandgetvehicletrailervehicle_int_int_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_061726_vehicle_commands_commandopenbombbaydoors_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_061727_vehicle_commands_commandclosebombbaydoors_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_061729_vehicle_commands_commandswingboatboomtoratio_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061730_vehicle_commands_commandallowboatboomtoanimate_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061731_vehicle_commands_commandgetboatboompositionratio_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_061734_vehicle_commands_commandisvehiclesearchlighton_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_061735_vehicle_commands_commandsetvehiclesearchlight_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_061736_vehicle_commands_commanddoesvehiclehavesearchlight_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_061742_vehicle_commands_commandsetvehiclewheeltype_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061752_vehicle_commands_commandsetvehiclemod_int_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_061756_vehicle_commands_commandremovevehiclemod_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061760_vehicle_commands_commandgetmodslotname_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_061762_vehicle_commands_commandgetvehiclemodmodifiervalue_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_061763_vehicle_commands_commandgetvehiclemodidentifierhash_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_061772_vehicle_commands_commandgetvehiclecolor_int_int_ref_int_ref_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_061779_vehicle_commands_commandmodifyvehicletopspeed_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061780_vehicle_commands_commandsetvehiclemaxspeed_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061786_vehicle_commands_commandisanypedrappellingfromheli_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_061794_vehicle_commands_commandsetlastdrivenvehicle_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_061798_vehicle_commands_commandsettaskvehiclegotoplaneminheightaboveter(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061806_vehicle_commands_commandcopyvehicledamage_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061809_vehicle_commands_commandsetvehicleshootattarget_int_int_rage_scr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_061836_vehicle_commands_commanddisablevehicleweapon_bool_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_061837_vehicle_commands_commandisvehicleweapondisabled_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_061842_vehicle_commands_commandsetplayerslastvehicle_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_061857_vehicle_commands_commanddoesvehicleexistwithdecorator_char_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_061861_vehicle_commands_commandsetvehicleexclusivedriver_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_061862_vehicle_commands_commandispedexclusivedriverofvehicle_int_int_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_061872_vehicle_commands_commandsetvehicleneonindexcolour_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061890_vehicle_commands_commandsethydraulicscontrol_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061891_vehicle_commands_commandsethydraulicsuspensionraisefactor_int_in(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_061892_vehicle_commands_commandgethydraulicsuspensionraisefactor_int_in(ph: *mut Phantom, a0: i32, a1: i32) -> f32; }
extern "C" { pub fn f_061893_vehicle_commands_commandsetcanusehydraulics_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061894_vehicle_commands_commandsethydraulicvehiclestate_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061895_vehicle_commands_commandsethydraulicwheelstate_int_int_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_061897_vehicle_commands_commandhasvehiclepetroltankbeensetonfirebyentit(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_061898_vehicle_commands_commandclearvehiclepetroltankfireculprit_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_061918_vehicle_commands_commandsetrocketboostactive_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061919_vehicle_commands_commandsetrocketboostfill_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061943_vehicle_commands_commandgetallvehicles_int_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_061944_vehicle_commands_commandsetcargobobextrapickuprangeallowance_int(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061961_vehicle_commands_command_getcanvehiclebeplacedhere_int_rage_scrv(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_061964_vehicle_commands_commandsetspecialflightmodetargetratio_int_floa(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061965_vehicle_commands_commandsetspecialflightmodeallowed_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061982_vehicle_commands_commandsetoverridenitrouslevel_int_bool_float_f(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: f32, a4: f32, a5: i32); }
extern "C" { pub fn f_062875_water_commands_commandgetwaterheight_rage_scrvector_const_ref_fl(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_062876_water_commands_commandgetwaterheightnowaves_rage_scrvector_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_062878_water_commands_waterprobetesthelper_bool_rage_vec3v_const_ref_ra(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_062910_weapon_commands_commandgiveweapontoped_int_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_062911_weapon_commands_commandgivedelayedweapontoped_int_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_062913_weapon_commands_commandsetpedammo_int_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_062914_weapon_commands_commandsetpedinfiniteammo_int_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_062917_weapon_commands_commandsetcurrentpedweapon_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_062921_weapon_commands_commandsetcurrentpedvehicleweapon_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_062924_weapon_commands_commandispedarmed_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_062926_weapon_commands_commandhaspedgotweapon_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_062928_weapon_commands_commandgetpedweapontypeinslot_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_062930_weapon_commands_commandremoveweaponfromped_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_062932_weapon_commands_commandhidepedweaponforscriptedcutscene_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_062933_weapon_commands_commandsetpedcurrentweaponvisible_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_062939_weapon_commands_commandpeddropweapon_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_062940_weapon_commands_commandpeddropsinventoryweapon_int_int_rage_scrv(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_062941_weapon_commands_commandgetmaxammoinclip_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_062942_weapon_commands_commandgetammoinclip_int_int_int_ref(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_062943_weapon_commands_commandsetammoinclip_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_062944_weapon_commands_commandgetmaxammo_int_int_int_ref(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_062945_weapon_commands_commandgetmaxammobytype_int_int_int_ref(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_062951_weapon_commands_commandgetpedammotypefromweapon_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_062952_weapon_commands_commandgetpedoriginalammotypefromweapon_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_062954_weapon_commands_commandsetpedgadget_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_062957_weapon_commands_commandexplodeprojectiles_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_062959_weapon_commands_commandgetlockondistanceofcurrentpedweapon_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_062960_weapon_commands_commandgetmaxrangeofcurrentpedweapon_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_062961_weapon_commands_commandhasvehiclegotprojectileattached_int_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_062962_weapon_commands_commandgiveweaponcomponenttoped_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_062963_weapon_commands_commandremoveweaponcomponentfromped_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_062964_weapon_commands_commandhaspedgotweaponcomponent_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_062965_weapon_commands_commandispedweaponcomponentactive_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_062968_weapon_commands_requestweaponasset_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_062969_weapon_commands_hasweaponassetloaded_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_062971_weapon_commands_commandcreateweaponobject_int_int_rage_scrvector(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: f32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_062972_weapon_commands_commandgiveweaponcomponenttoweaponobject_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_062973_weapon_commands_commandremoveweaponcomponentfromweaponobject_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_062975_weapon_commands_commandgiveweaponobjecttoped_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_062976_weapon_commands_commanddoesweapontakeweaponcomponent_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_062977_weapon_commands_commandgetweaponobjectfromped_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_062979_weapon_commands_commandsetpedweapontintindex_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_062980_weapon_commands_commandgetpedweapontintindex_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_062981_weapon_commands_commandsetpedweaponcomponenttintindex_int_int_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_062982_weapon_commands_commandgetpedweaponcomponenttintindex_int_int_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_062983_weapon_commands_commandgetpedweaponcamoindex_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_062992_weapon_commands_commandgetweapondamage_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> f32; }
extern "C" { pub fn f_062996_weapon_commands_commandsetpedshootordnanceweapon_int_float(ph: *mut Phantom, a0: i32, a1: f32) -> i32; }
extern "C" { pub fn f_063008_weapon_commands_commandcanuseweapononparachute_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_063017_weapon_commands_commandsetcanpedselectiventoryweapon_int_int_boo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_063018_weapon_commands_commandsetcanpedselectallweapons_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_063171_xml_commands_cbugstarqueryop_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063200_zone_commands_commandoverridepopschedulevehiclemodel_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_063245_stats_commands_fmeventambientmission_write_rage_rsonwriter_ptr_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063288_stats_commands_bw_bosswork_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063319_stats_commands_bc_bosschallenge_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063348_stats_commands_buycontrabandmission_write_rage_rsonwriter_ptr_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063349_stats_commands_contrabandmission_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063539_stats_commands_metriccasino_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063559_stats_commands_metriccasinolight_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063676_ctaskmoveseekentity_213_centityseekposcalculatorstandard_ccloned(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_063681_ctaskmoveseekentity_213_centityseekposcalculatorstandard_ccloned(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063682_ctaskmoveseekentity_213_centityseekposcalculatorstandard_ccloned(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063699_ctaskmoveseekentity_213_centityseekposcalculatorstandard_ccloned(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_063700_ctaskmoveseekentity_217_centityseekposcalculatorradiusangleoffse(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_063708_ctaskmoveseekentity_217_centityseekposcalculatorradiusangleoffse(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_063709_ctaskmoveseekentity_215_centityseekposcalculatorxyoffsetrotated(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_063714_ctaskmoveseekentity_215_centityseekposcalculatorxyoffsetrotated(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063718_ctaskmoveseekentity_215_centityseekposcalculatorxyoffsetrotated(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_063719_ctaskmoveseekentity_216_centityseekposcalculatorxyoffsetfixed_cc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_063727_ctaskmoveseekentity_216_centityseekposcalculatorxyoffsetfixed_cc(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_063729_cscriptwidgettree_cscriptwidgetnode_dtorcscriptwidgetnode(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_063733_profdisplayf_bool_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_063743_cthescripts_isplayerplaying_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063748_gtathread_allocatedebugger(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063749_gtathread_updatearrayofnamesofrunningthreads_rage_atarray_char_c(ph: *mut Phantom); }
extern "C" { pub fn f_063750_gtathread_displayallrunningscripts_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_063754_rage_atmapmemory_int_chiddenobjects_hidden_object_struct_dealloc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063757_cgamescripthandlermgr_dtorcgamescripthandlermgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_063762_cthescripts_gtastartnewthreadoverride_char_const_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_063763_cthescripts_gtastartnewthreadwithprogramid_rage_scrprogramid_voi(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_063764_cscriptdebug_shouldthisscriptbedebugged_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063771_cgamescriptresource_incrementnumberofusedresources(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063772_cthescripts_getcurrentgtascripthandler(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063774_cgamescripthandlermgr_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063775_cscriptglobaltampercheckertunableslistener_cscriptglobaltamperch(ph: *mut Phantom); }
extern "C" { pub fn f_063777_cscriptshapetestmanager_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_063778_cscriptbraindispatcher_itemwaitingforbrain_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063781_cgamescripthandlermgr_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063782_chiddenobjects_removefromhiddenobjectmap_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_063783_cscripthud_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_063784_cdoublebufferedintrotexts_freememoryforalltextlines(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063785_cdoublebufferedintrotexts_initialise_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063786_cliteralstringsforimmediateuse_freememory(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063789_cscriptglobaltamperchecker_shouldcheckglobalblock_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_063792_csuppressedmodels_printsuppressedmodels(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063793_cscriptsforbrains_startorrequestnewstreamedscriptbrain_int_void(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_063794_cthescripts_registerentity_cphysical_ptr_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_063795_cgamescripthandlernetwork_validateregistrationrequest_cphysical(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_063796_cthescripts_unregisterentity_cphysical_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063797_cthescripts_getcurrentgtascripthandlernetwork(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063798_cthescripts_prioritizerequestsfrommissionscript_int_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063799_cthescripts_getcurrentscriptname(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063800_cthescripts_getcurrentscriptnameandprogramcounter(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063807_chiddenobjects_displayinteriordetails_rage_fwinteriorlocation_re(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063809_rage_atmapentry_int_chiddenobjects_hidden_object_struct_const_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_063817_cthescripts_findnetworkplayer_int_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063820_cthescripts_hasentitybeenregisteredwithcurrentthread_cphysical_c(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063821_cgamescriptid_initgamescriptid_rage_scrthread_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_063823_cscripteulers_matrixfromeulers_rage_matrix34_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063827_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_063828_cdlcscript_removescript_cdlcscript_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063829_cdlcscript_shutdownscript(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063839_cscriptpedaiblips_aiblip_cleanupblip(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063841_cscriptpedaiblips_aiblip_getblipfromentity_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063842_cscriptpedaiblips_aiblip_removeblip_int_ref_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_063843_cscriptpedaiblips_aiblip_addblipforentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063844_cscriptpedaiblips_aiblip_handleblipfade_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063854_cgamescripthandler_registerscriptresourceandgetref_rage_scriptre(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063856_cscriptareas_highlightscriptarea_rage_vector3_ref_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063858_cscriptareas_highlightimportantangledarea_unsigned_int_float_flo(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32); }
extern "C" { pub fn f_063859_cscriptdebug_drawdebugangledsquare_float_float_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32); }
extern "C" { pub fn f_063860_calculategroundzforvector_rage_vector3_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063863_cscriptdebug_drawdebugangledcube_float_float_float_float_float_f(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: f32, a10: i64); }
extern "C" { pub fn f_063871_cscriptbraindispatcher_addwaitingped_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063873_cscriptsforbrains_isworldpointwithinactivationrange_c2deffect_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063880_cupsidedowncarcheck_iscarinupsidedowncararray_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_063881_cupsidedowncarcheck_addcartocheck_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_063882_cupsidedowncarcheck_removecarfromcheck_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_063883_cupsidedowncarcheck_hascarbeenupsidedownforawhile_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_063884_cstuckcarcheck_init(ph: *mut Phantom); }
extern "C" { pub fn f_063885_cstuckcarcheck_addcartocheck_int_float_unsigned_int_bool_bool_bo(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_063886_cstuckcarcheck_removecarfromcheck_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_063887_cstuckcarcheck_iscarinstuckcararray_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_063893_csuppressedmodels_hasmodelbeensuppressed_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_063897_cscriptpeds_haspedmodelbeenrestrictedorsuppressed_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_063898_cscriptcars_isvehiclestopped_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063900_cscriptentities_clearspaceformissionentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063902_cscriptpeds_givepedscriptedtask_int_rage_aitask_ptr_int_char_con(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_063903_rage_fweventqueue_16_void_flusheventsoftype_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063914_missioncreatorassets_getadditionalmodelcost_rage_atuserarray_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_063920_rage_atarray_buidlingdescription_0_unsigned_short_destruct_buidl(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063923_cdebugattachmenttool_dtorcdebugattachmenttool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063925_cdebugattachmenttool_createwidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_063932_cdebugattachmenttool_scriptattachped(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063933_cdebugattachmenttool_scriptattachother(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063934_cdebugattachmenttool_getscriptcall(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_063935_cdebugattachmenttool_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063936_cdebugattachmenttool_loadbonelist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063937_cdebugattachmenttool_initialise_cphysical_ptr_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_063938_cscriptwidgettree_cscriptwidgetnode_findwidget_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_063977_cscriptdebugpedhandler_createscriptped_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063978_cscriptdebugvehiclehandler_createscriptvehicle_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063979_cscriptdebugobjecthandler_createscriptobject_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_063981_cscriptdebugpedhandler_getvectorfromtextbox_rage_vector3_ref_cha(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063986_cdisplaytextformatting_outputdebuginfo_bool_float_float(ph: *mut Phantom, a0: i32, a1: f32, a2: f32); }
extern "C" { pub fn f_063988_cdisplaytextbaseclass_draw_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_063989_cscripthud_getscreenposfromworldcoord_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_064006_intro_script_rectangle_outputvaluesofmembersfordebug_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064008_intro_script_rectangle_draw_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_064009_cscripthud_renderscriptedscaleformmovie_int_rage_vector2_rage_ve(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_064010_intro_script_rectangle_pushrefcount_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064011_cdoublebufferedintrotexts_allocatememoryforalltextlines(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064015_cliteralstringsforimmediateuse_allocatememory_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_064023_void_rage_dlc_addinternal_void_ptr_int_unsigned_int_int_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_064034_citemsetmanager_allocate_citemset_ref_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_064035_citemsetmanager_free_citemset_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064045_cmpoutfitsmap_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_064048_cmpoutfits_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_064054_cbaseelementlocationsmap_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_064057_cbaseelements_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_064072_cmpoutfits_dtorcmpoutfits(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064076_cbaseelements_dtorcbaseelements(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064078_cmpappareldata_dtorcmpappareldata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064085_cscriptshapetestresults_dtorcscriptshapetestresults(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064089_cscripttextconstruction_clear(ph: *mut Phantom); }
extern "C" { pub fn f_064131_cscripttextconstruction_addsubstringliteralstring_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064135_rage_fwassetstore_rage_scrprogram_rage_fwassetnamedef_rage_scrpr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064141_cstreamedscripts_getnumrefs_rage_strlocalindex_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_064149_cscriptentityextension_createscriptinfo_rage_scriptidbase_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_064154_cscriptentityextension_onregistration_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_064156_cscriptentityextension_onunregistration(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064158_cscriptentityextension_cleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064162_cscriptentityextension_spewdebuginfo_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064164_cgamescripthandler_cgamescripthandler_rage_scrthread_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_064166_cgamescripthandler_getscriptentity_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_064169_cgamescripthandler_terminate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064173_cgamescripthandler_registerscriptresourceandgetid_rage_scriptres(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_064183_cgamescripthandlermgr_logcurrentreservations_rage_netlogginginte(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064188_cgamescripthandlernetcomponent_broadcastremotescriptinfo_rage_ne(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_064200_cgamescripthandlermgr_writeremotescriptheader_cgamescriptid_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_064202_cgamescripthandlermgr_removeplayerfromremotescript_cgamescriptid(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_064203_cgamescripthandlermgr_moveactivescripttoterminatedqueue_unsigned(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064209_cgamescripthandlermgr_addremotescriptinfo_cgamescripthandlermgr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064215_cgamescriptid_generatescriptidhash(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064233_cgamescripthandlermgr_getremotescriptentityreservations_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_064237_cgamescripthandlermgr_clearmemorydebuginfoformoviemeshes_bool_co(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064241_cscriptstreamingresource_cscriptstreamingresource_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_064260_cscriptresource_texturedictionary_cscriptresource_texturediction(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_064275_cscriptresource_vehiclerecording_cscriptresource_vehiclerecordin(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_064309_cscriptresource_speedzone_cscriptresource_speedzone_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_064313_cscriptresource_vehicle_asset_cscriptresource_vehicle_asset_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_064325_cscriptresource_vehiclecombatavoidancearea_cscriptresource_vehic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_064425_cgamescriptresource_dtorcgamescriptresource_102634(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064486_ragesecgamepluginbondermanager_rebalancebonders(ph: *mut Phantom); }
extern "C" { pub fn f_064493_ragesecpopreaction(ph: *mut Phantom); }
extern "C" { pub fn f_064500_ccustomshadereffectbase_writeinstancedata_rage_grcvec4bufferinst(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_064504_ccustomshadereffectbasetype_setupfordrawable_rage_rmcdrawable_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_064505_ccustomshadereffectweapontype_initialise_rage_rmcdrawable_ptr_cb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_064506_ccustomshadereffectcabletype_initialise_rage_rmcdrawable_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_064507_ccustomshadereffectmirrortype_initialise_rage_rmcdrawable_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_064508_ccustomshadereffectpedtype_initialise_rage_rmcdrawable_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_064509_ccustomshadereffectvehicletype_create_rage_rmcdrawable_ptr_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_064510_ccustomshadereffectvehicletype_initialise_rage_rmcdrawable_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_064511_ccustomshadereffecttinttype_create_rage_rmcdrawable_ptr_cbasemod(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_064522_csegrassstatic_probeformingroundpos_rage_spdaabb_const_ref_rage(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_064536_void_rage_dlc_addinternal_void_ptr_ecsmrasterizerstate_ecsmraste(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064565_ccustomshadereffectped_getpedheat_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_064566_ccustomshadereffectped_updatepeddamage_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064570_ccustomshadereffectped_seteditableshadervalues_rage_grmshadergro(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_064588_ccustomshadereffecttint_selecttintpalette_unsigned_char_rage_fwe(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064589_ccustomshadereffecttint_getmaxtintpalette(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064592_ccustomshadereffecttint_setshadervariables_unsigned_char_ptr_ccu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_064605_ccustomshadereffecttree_branch_bend_and_tri_wave_micro_movements(ph: *mut Phantom); }
extern "C" { pub fn f_064619_ccustomshadereffecttree_setwindvector_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064620_ccustomshadereffecttree_sfxwindfieldcacheentry_updatetexture(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064635_ccustomshadereffectvehicletype_dtorccustomshadereffectvehicletyp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064639_ccustomshadereffectvehicle_updatevehiclecolors_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_064640_ccustomshadereffectvehicle_setlicenseplatetexindex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064641_getvehiclecolourramptexture_cvehiclemodelinfovarglobal_ptr_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_064642_ccustomshadereffectvehicle_setlicenseplatetext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064646_ccustomshadereffectvehicle_setcustomprimarycolor_rage_color32(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064647_ccustomshadereffectvehicle_setcustomsecondarycolor_rage_color32(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064650_ccustomshadereffectvehicle_setshadervariables_rage_rmcdrawable_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_064652_ccustomshadereffectvehicle_getlicenseplatetext_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064657_ccustomshadereffectweapontype_dtorccustomshadereffectweapontype(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064665_cshaderedit_regenshaderwidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064666_cshaderedit_populatewidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064667_cshaderedit_takecontrolofshadereditpicker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064680_cshaderedit_updatetexture_rage_rmcdrawable_ptr_cshaderedit_textu(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_064688_cshaderhairsort_shaderdrawskinned_rage_grmshader_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_064689_cshaderlib_usesstipplefades(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_064694_cshaderlib_setstipplealpha_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064698_cshaderlib_setglobalalpha_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_064700_cshaderlib_setglobalemissivescale_float_bool(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_064724_cshaderlib_setreticuledisttexture_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064726_cshaderlib_setstereoparams_rage_vector4_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064736_csettingsmanager_csettingsmanager(ph: *mut Phantom); }
extern "C" { pub fn f_064737_csettingsmanager_getsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064747_cislandhopper_activateheistisland(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064748_cislandhopper_deactivateheistisland(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064749_cislandhopper_requesttoggleisland_rage_atnonfinalhashstring_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_064757_cislandhopper_enableimaps_rage_atnonfinalhashstring_const_ptr_in(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064758_cislandhopper_disableimaps_rage_atnonfinalhashstring_const_ptr_i(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064759_cislandhopper_arecurrentislandiplsactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064766_cpopulationstreaming_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064767_cpopulationstreaming_addtotalmemoryused_int_int_ememorytracktype(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_064769_cpopulationstreaming_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064770_cpopulationstreaming_removestreamedcarmodelsonshutdown(ph: *mut Phantom); }
extern "C" { pub fn f_064771_cpopulationstreaming_emergencyremovevehiclestructures(ph: *mut Phantom); }
extern "C" { pub fn f_064772_cpopulationstreaming_reclassifyloadedvehiclesfornetwork(ph: *mut Phantom); }
extern "C" { pub fn f_064773_cloadedmodelgroup_merge_cloadedmodelgroup_ptr_cloadedmodelgroup(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_064776_cpopulationstreaming_requestvehicledriver_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064777_cpopulationstreaming_getfallbackpedgroup_cloadedmodelgroup_ref_b(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064778_cpopulationstreaming_streamonenewcar(ph: *mut Phantom); }
extern "C" { pub fn f_064779_cpopulationstreaming_tellstreamingaboutdeletablevehicles(ph: *mut Phantom); }
extern "C" { pub fn f_064780_cpopulationstreaming_reclassifyloadedpedsfornetwork(ph: *mut Phantom); }
extern "C" { pub fn f_064781_cpopulationstreaming_adddiscardedped_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064782_cpopulationstreaming_findoldestmodeltostreamout_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064783_cpopulationstreaming_getnumcommonvehicles(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_064784_cpopulationstreaming_adddiscardedcar_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064786_cpopulationstreaming_setreduceambientpedmodelbudget_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064787_cpopulationstreaming_setreduceambientvehiclemodelbudget_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064788_cpopulationstreaming_iscarmodelneededcurrently_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064789_cpopulationstreaming_canpedmodelridebike_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064791_cpopulationstreaming_pedhasbeenstreamedin_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064792_cpopulationstreaming_addpedmodeltocorrectlist_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064793_cpopulationstreaming_canpedmodeldrive_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064794_cpopulationstreaming_addstreamedinmodeldata_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064795_cpopulationstreaming_addstreamedindependencies_rage_atarray_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_064796_cloadedmodelgroup_removepedmodelsthatcantdrivecars_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064797_cpopulationstreaming_getadjustedmodelstartoffset_cpopulationstre(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064798_cloadedmodelgroup_addmember_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064799_cpopulationstreaming_streamonecarout_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064806_cpopulationstreaming_removestreamedindependencies_rage_atarray_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_064807_cpopulationstreaming_unrequestvehicledriver_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064819_populationstreamingdependencycost_rage_strlocalindex_int_rage_at(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_064820_getcarmemoryuse_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064821_getpedmemoryuse_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064832_cstreaming_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064838_cprioritizedclipsetrequest_dtorcprioritizedclipsetrequest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064841_cprioritizedclipsetrequest_update_rage_fwclipset_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064843_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_cpriori(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_064845_cstreaming_isobjectinimage_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064847_cprioritizedclipsetstreamer_updatememorycostforclipdictionary_ra(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064856_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_cpriori(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_064861_cprioritizedclipsetrequestmanager_getrequestindexifexists_cprior(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_064868_cscriptpreloaddata_releaseresource_unsigned_short_rage_atarray_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_064879_csituationalclipsetstreamer_tunables_avoids_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_064908_rage_czonedassetmanager_enumandprocessfolder_char_const_ptr_void(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064913_ckeyboard_getkeyjustdown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_064914_rage_czonedassetmanager_showzonedetails(ph: *mut Phantom); }
extern "C" { pub fn f_064915_rage_czonedassetmanager_drawzoneboxes(ph: *mut Phantom); }
extern "C" { pub fn f_064916_rage_czonedassetmanager_clearrequests(ph: *mut Phantom); }
extern "C" { pub fn f_064917_rage_czonedassetmanager_addrequest_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064918_cstreamingcleanup_requestflushnonmission_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064919_cstreamingdebug_collectmemoryprofileinformation_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064920_teleporttomemoryprofilelocation_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064921_cstreamingdebug_displayresourceheappages(ph: *mut Phantom); }
extern "C" { pub fn f_064922_cstreamingdebug_showcollectedmemoryprofilestats_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064923_cstreamingdebug_displaymodulememorydetailed_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_064924_cstreamingdebug_displaymodulememory_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_064925_cstreamingdebug_displayobjectsinmemory_rage_fistream_ptr_bool_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_064926_ckeyboard_getalteredkeyboardmode_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064927_ckeyboard_debugoutputscriptcallstack(ph: *mut Phantom); }
extern "C" { pub fn f_064929_cstreaming_setobjectisdeletable_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064930_cstreaming_getobjectisdeletable_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064931_cstreaming_setmissiondoesntrequireobject_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064933_cstreaming_requestobject_rage_strlocalindex_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_064937_cstreaming_loadobject_rage_strlocalindex_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064939_cstreaming_enablestreaming(ph: *mut Phantom); }
extern "C" { pub fn f_064941_rage_atarray_rage_datowner_gtadrawable_0_unsigned_short_atarray(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064943_cstreamingcleanup_deletevehiclesandpeds_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064944_cstreamingcleanup_performremoval_rage_strindex_unsigned_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_064946_cstreamingcleanup_handleoutofmemory_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_064947_cstreamingdebug_dumpmodulememorycomparedtoaverage(ph: *mut Phantom); }
extern "C" { pub fn f_064948_cstreamingcleanup_deleteentityanddependencies_deletionmap_ref_bu(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_064949_bucketentry_getvalidatedentity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064950_cstreamingdebug_loadmemoryprofilelocations(ph: *mut Phantom); }
extern "C" { pub fn f_064966_memoryfootprint_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_064969_memoryprofilelocation_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_064979_createflagstring_unsigned_int_char_ptr_unsigned_long(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_064980_addtoassetlistrecursive_rage_strindex_rage_atmap_rage_strindex_b(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_064981_adddependencycost_rage_strindex_int_ref_int_ref_rage_atmap_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_064982_computedependencycost_rage_strindex_int_ref_int_ref_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_064986_rage_atarray_rage_atbinarymap_memoryprofilemodulestat_rage_atnon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064987_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_064991_cstreamingdebug_computemodulememorystats_modulememorystats_ptr_b(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064992_cstreamingdebug_getmodulememoryusage_int_int_ref_int_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_064995_cstreamingdebug_outputline_rage_fistream_ptr_bool_rage_color32_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_064996_loadallmoduleobjectsonce_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_065011_std_2_tree_rage_strindex_std_2_less_rage_strindex_std_2_allocato(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065012_cstreamingdebug_getmemorystatsbyallocation_rage_atarray_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_065013_void_std_2_introsort_std_2_classicalgpolicy_int_ptr_ref_rage_str(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_065014_displayobjectdependencies_rage_fistream_ptr_bool_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_065015_getentitylist_rage_strindex_rage_fwregdref_centity_rage_fwrefawa(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_065016_appendfileinfoname_rage_strindex_bool_char_ptr_unsigned_long(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_065020_displaymemoryinfo_char_const_ptr_rage_sysmemallocator_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065023_displaybucketusage_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_065036_int_ref_rage_atmap_unsigned_long_int_rage_atmaphashfn_unsigned_l(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_065053_cstreamgraph_updatecategorystats_int_int_ptr_void_const_ptr_void(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_065054_cstreamgraph_updatemodulestats_int_int_ptr_void_const_ptr_void_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_065062_resourcememoryusedump_buildscenecoststreamingindices(ph: *mut Phantom); }
extern "C" { pub fn f_065063_resourcememoryusedump_dumpscenecostdetails_rage_fistream_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065067_cstreamgraphselectedfiledata_collectdependents_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_065069_cstreamgraphextensionmanager_cstreamgraphextensionmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065071_cstreamgraph_dtorcstreamgraph(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065073_cstreamgraph_drawmemorymarkers_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_065075_cstreamgraphmodulebuffer_hasnonzerocategory_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_065078_cstreamgraph_adddependenciestomap_rage_strindex_rage_atmap_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_065079_cstreamgraph_buildtxdlodcounts_centity_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065080_cstreamgraph_updatefilelist_graphentry_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_065081_cstreamgraph_objectpassesfilter_rage_strstreamingmodule_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_065082_cstreamgraph_collectresultsforautocapture_rage_atarray_streaming(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065083_cstreamgraph_updatefileinfo_graphentry_const_ref_rage_strstreami(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_065089_cstreamgraphexternalallocations_getextalloc_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_065094_cstreamgraphallocationstoresgamevirt_dtorcstreamgraphallocations(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065102_cstreamingrequestframe_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_065108_cstreamingrequestframe_dtorcstreamingrequestframe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065116_cstreamingrequestrecord_dtorcstreamingrequestrecord(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065122_cstreamingrequestframe_processrequestlist_cstreamingrequestrecor(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_065123_cstreamingrequestframe_createfullrequestlist_cstreamingrequestre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_065124_cstreamingrequestframe_setlistdeleteable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065125_cstreamingrequestlist_registerfromfile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065129_cstreamingrequestlist_deleterecording(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065130_cstreamingrequestlist_finishrecord(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065131_cstreamingrequestlist_finishplayback_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_065134_cstreamingrequestrecord_debugdrawlistdetail_rage_atarray_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_065135_cstreamingrequestframe_debugdrawlist_rage_atarray_rage_atnonfina(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_065145_cstreamingrequestlisteditor_movecurrentby_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_065146_cstreamingrequestrecord_findnext_int_rage_atnonfinalhashstring_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_065147_rage_atarray_cstreamingrequestframe_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065149_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_int_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_065173_streamingmemoryresult_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_065178_streamingmemoryresult_dtorstreamingmemoryresult(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065179_memorydistributionresult_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_065198_memorybucketusageresult_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_065201_scriptmemoryusageresult_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_065210_pedandvehiclestatresults_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_065216_metricslist_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_065221_metricslist_dtormetricslist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065228_metricscapture_storescriptmemresult_debuglocationmetricslist_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065230_metricscapture_storestreamingstats_debuglocationmetricslist_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065233_metricscapture_storememdistributionresult_debuglocationmetricsli(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065235_metricscapture_storepedandvehiclestatresult_debuglocationmetrics(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065236_metricscapture_storeframespersecondresult_debuglocationmetricsli(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065237_metricscapture_storemsperframeresult_debuglocationmetricslist_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065240_metricscapture_storecputimingresults_debuglocationmetricslist_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065241_metricscapture_storethreadtimingresults_debuglocationmetricslist(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065243_metricscapture_storememorybucketusageresults_debuglocationmetric(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065245_metricscapture_storedrawlistresults_debuglocationmetricslist_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065246_metricscapture_clearresults_debuglocationmetricslist_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065247_metricscapture_captureinstance_startcapture(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065248_metricscapture_captureinstance_startsampling_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_065269_metricscapture_saveresults_debuglocationmetricslist_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065270_metricscapture_ondemandcapturetoggle(ph: *mut Phantom); }
extern "C" { pub fn f_065273_ckeyboard_getkeydown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_065274_metricscapture_metricszonestart_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065275_metricscapture_metricszonestop(ph: *mut Phantom); }
extern "C" { pub fn f_065287_ccompaniondata_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065289_ccompanionblipnode_update_scompanionblip_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065310_ccompaniondatapc_dtorccompaniondatapc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065317_rage_atmap_rage_atfinalhashstring_rage_controlinput_gamepad_defi(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065318_rage_controlinput_dynamicmappinglist_dtordynamicmappinglist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065319_rage_controlinput_mappingsettings_dtormappingsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065323_ccontrol_dtorccontrol(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065325_ccontrol_loaddynamiccontrolmappings(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065327_ccontrol_init_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065329_rage_atmap_rage_atfinalhashstring_rage_controlinput_gamepad_defi(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065330_ccontrol_settings_rebuildgamepaddefinitionlist(ph: *mut Phantom); }
extern "C" { pub fn f_065331_rage_controlinput_inputsettings_dtorinputsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065339_ccontrol_loadcontrols_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065341_ccontrol_loadscriptedsettings_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_065342_ccontrol_loadsettings_rage_controlinput_controlsettings_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_065343_ccontrol_settings_getgamepaddefinition_rage_iojoystick_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065344_ccontrol_joystickdefinitiondata_convertpadmappingstojoystickmapp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_065345_ccontrol_updatemapping_rage_inputtype_ccontrol_mappingtype_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_065346_ccontrol_setinitialdefaultmappings_bool_bool_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_065347_ccontrol_clearinputmappings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065348_ccontrol_loadsettingsfile_char_const_ptr_bool_rage_controlinput(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_065349_ccontrol_loaddevicecontrols_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_065350_ccontrol_getusersettingspath_char_ref_260_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065351_ccontrol_addkeyboardmouseextrasettings_rage_controlinput_control(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065353_rage_atarray_rage_controlinput_mapping_0_unsigned_short_copy_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_065354_rage_controlinput_gamepad_definition_ref_rage_atmap_rage_atfinal(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_065355_rage_atarray_rage_controlinput_gamepad_source_0_unsigned_short_o(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065357_ccontrol_getaxiscompatablesource_rage_iosource_const_ref_rage_io(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_065359_ccontrol_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065360_ccontrol_resetkeyboardmousesettings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065361_ccontrol_disableinputsbyactivesources_rage_atfixedarray_rage_ios(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_065363_ccontrol_getparametertext_rage_iomapperparameter(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065364_ccontrol_updatecurrentgamepaddefinition_rage_controlinput_gamepa(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065365_ccontrol_updatejoystickdefinitionragdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065373_ccontrol_disableallinputs_unsigned_int_rage_iovalue_disableoptio(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_065374_ccontrol_setinputexclusive_int_rage_iovalue_disableoptions_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_065375_ccontrol_doenableinput_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_065378_ccontrol_enableinput_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_065379_ccontrol_dodisableinput_unsigned_int_rage_iovalue_disableoptions(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_065388_cpad_applyactuatoreffect_rage_ioactuatoreffect_ptr_bool_int_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_065394_ccontrol_updateaxisdefinitionmappings_rage_inputtype_ccontrol_ma(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_065395_ccontrol_getmappingparametername_rage_iosource_const_ref_char_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065396_ccontrol_removemapping_rage_iosource_const_ref_rage_inputtype_cc(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_065409_ccontrol_savecurrentjoystickdefinition_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_065410_ccontrol_loadcurrentjoystickdefinition_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065411_ccontrol_convertparametertogampadinput_rage_iomapperparameter(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_065416_ccontrol_saveusercontrols_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_065417_ccontrol_getconflictinginputs_rage_atarray_rage_inputtype_0_unsi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065422_ccontrol_getunmappedinputs_rage_atarray_rage_inputtype_0_unsigne(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065423_ccontrol_getconflictinginputdata_rage_inputtype_rage_controlinpu(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_065428_rage_atarray_rage_controlinput_mapping_0_unsigned_short_pushandg(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065444_ccontrolmgr_update(ph: *mut Phantom); }
extern "C" { pub fn f_065451_ccontrolmgr_stoppadsshaking(ph: *mut Phantom); }
extern "C" { pub fn f_065452_cpad_cpadstate_update_rage_iopad_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065453_cpad_updateactuatoreffects(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065454_ccontrolmgr_displaydisabledinputs(ph: *mut Phantom); }
extern "C" { pub fn f_065456_ckeyboard_convertinputbufferintoascii_char_ptr_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065458_ccontrolmgr_updatevirtualkeyboard(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065459_debuginputprint_rage_vector2_const_ref_rage_color32_char_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_065469_ccontrolmgr_startplayerpadshakebyintensity_unsigned_int_float_in(ph: *mut Phantom, a0: i32, a1: f32, a2: i32); }
extern "C" { pub fn f_065491_rage_controlinput_controlsettings_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_065500_rage_controlinput_dynamicmappings_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_065528_rage_controlinput_gamepad_definition_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_065567_rage_atfixedarray_rage_controlinput_dynamicmapping_19_dtoratfixe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065583_rage_controlinput_steamcontroller_axismappings_dtoraxismappings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065585_rage_controlinput_steamcontroller_definition_dtordefinition(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065593_enduserbenchmarks_stopbenchmark(ph: *mut Phantom); }
extern "C" { pub fn f_065594_enduserbenchmarks_outputbenchmarkresults(ph: *mut Phantom); }
extern "C" { pub fn f_065598_csettingsmanager_graphicsconfigsource_char_ptr_ref_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065599_settingsdefaults_settingsdefaults(ph: *mut Phantom); }
extern "C" { pub fn f_065614_cfilemgr_readline_void_ptr_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_065617_cfilemgr_setdir_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065619_cfilemgr_setpcdir_rage_atfixedstring_128ul_ref_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065621_cfilemgr_openfileforwriting_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065622_cfilemgr_openfileforappending_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065641_settingsdefaults_rendertargetmemsizefor_settings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065766_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_065767_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_065768_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_065769_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_065770_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_065771_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_065772_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_066695_ccover_removecoverpointwithindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_067369_crandomeventmanager_flushcopasserts(ph: *mut Phantom); }
extern "C" { pub fn f_068095_cpatrolnode_addpatrollink_cpatrolnode_ptr_linkdirection_rage_fwi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_068162_ctasksequencelist_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068163_ctasksequencelist_addtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068164_ctaskmovesequence_ctaskmovesequence(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068171_ctaskmovesequence_addtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068173_ctasksequences_init(ph: *mut Phantom); }
extern "C" { pub fn f_068174_ctasksequences_getavailableslot_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_068203_ctaskusesequence_ctaskusesequence_ctasksequencelist_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_068208_ctaskusesequence_gettasksequencelist_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068959_ctaskmovefollownavmesh_isunabletofindroute_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069065_ctaskmovegotopointandstandstill_ctaskmovegotopointandstandstill(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: f32) -> i64; }
extern "C" { pub fn f_069067_ctaskmovestandstill_ctaskmovestandstill_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_069069_ctaskmoveachieveheading_ctaskmoveachieveheading_float_float_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_069289_ctasknavbase_getdistanceleftoncurrentroutesection_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_069477_ctaskmovefollownavmesh_ctaskmovefollownavmesh_float_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: i64, a8: f32, a9: i32) -> i64; }
extern "C" { pub fn f_069530_ctaskparachute_findparachutepackvariationforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069555_ctaskparachute_doespedhaveparachute_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069675_centityseekposcalculatorxyoffsetfixed_computeentityseekpos_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_069676_centityseekposcalculatorxyoffsetrotated_computeentityseekpos_cpe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_069677_centityseekposcalculatorradiusangleoffset_computeentityseekpos_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_072533_cscenarioclusterspawnedtrackingdata_resetrefcountedmodel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072916_cscenariopointprioritymanager_restoregroupstooriginalpriorities(ph: *mut Phantom); }
extern "C" { pub fn f_075368_cprioritizedclipsetrequesthelper_dtorcprioritizedclipsetrequesth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075369_cprioritizedclipsetrequesthelper_releaseclipset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075372_cprioritizedclipsetrequesthelper_getclipsetid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075373_cprioritizedclipsetrequesthelper_requestclipset_rage_fwmvtemplat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075375_cprioritizedclipsetrequesthelper_setstreamingpriorityoverride_ra(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_075377_cprioritizedclipsetrequesthelper_shouldclipsetbestreamedout_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075396_cboatavoidancehelper_isbeached_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075523_ctuning_dtorctuning(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075543_cvehiclejunctionhelper_approachinglight_cvehicle_const_ptr_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075632_ctaskmove_dtorctaskmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075660_ctuning_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_075664_ctuning_registertuning_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_075917_ctaskvehiclefsm_isopenseatisblocked_cvehicle_const_ref_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_076078_ctaskinvehicleseatshuffle_canshuffleseat_cvehicle_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_076294_ctaskvehiclefsm_getdoorforentrypointindex_cvehicle_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_076384_ctaskmotioninvehicle_checkforstill_float_cvehicle_const_ref_cped(ph: *mut Phantom, a0: f32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32) -> i32; }
extern "C" { pub fn f_076387_ctaskmotioninvehicle_checkforreverse_cvehicle_const_ref_cped_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_077198_cstatsdatamgr_setstatiterdata_rage_atinbinarymap_sstatdataptr_st(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_077200_cstatsdatamgr_statiterfind_statid_const_ref_rage_atinbinarymap_s(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077475_cnumberwithinmessage_set_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077476_cnumberwithinmessage_set_float_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077477_cnumberwithinmessage_debugprintcontents(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077478_csubstringwithinmessage_settextlabel_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077479_ctextfile_get_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077480_ctextfile_getinternal_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077483_csubstringwithinmessage_setliteralstring_char_const_ptr_csubstri(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_077484_csubstringwithinmessage_clear_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077485_csubstringwithinmessage_getstring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077486_csubtitlemessage_set_char_const_ptr_int_cnumberwithinmessage_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32); }
extern "C" { pub fn f_077488_cmessages_insertnumbersandsubstringsintostring_char_const_ptr_cn(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32); }
extern "C" { pub fn f_077491_cmessages_addtopreviousbriefarray_csubtitlemessage_ref_eprevious(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077495_chelpmessage_clearall(ph: *mut Phantom); }
extern "C" { pub fn f_077503_cmessages_clearthisprint_csubtitlemessage_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077504_cmessages_addmessage_char_const_ptr_int_unsigned_int_bool_bool_e(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i64, a9: i32, a10: i32, a11: i32); }
extern "C" { pub fn f_077507_chelpmessage_setmessagetext_int_char_const_ptr_cnumberwithinmess(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_077515_chelpmessage_getmessagetext_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077533_ctextlayout_ctextlayout(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077534_ctextlayout_default(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077535_ctextlayout_dtorctextlayout(ph: *mut Phantom); }
extern "C" { pub fn f_077537_ctextlayout_setscale_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077538_ctextlayout_setadjustfornonwidescreen_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077539_ctextlayout_getcharacterheight(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_077540_ctextlayout_render_rage_vector2_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077543_ctextlayout_getnumberoflines_rage_vector2_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_077544_ctextformat_getstringwidth_ctextlayout_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_077545_ctextlayout_setscale_rage_vector2_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077549_ctext_flush(ph: *mut Phantom); }
extern "C" { pub fn f_077559_ctextconversion_stripnonrenderabletext_char_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077565_ctextfile_doestextlabelexist_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077801_cvehicleintelligence_getnodelist_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077803_cvehicleintelligence_getactivetask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077842_cpathfind_switchroadsoffinareaforoneregion_cnodesswitchedonoroff(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_077846_cpathfind_releaserequestednodes_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077873_cpathfind_updatestreaming_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077884_cpathfind_findnthnodeclosesttocoors_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: f32, a11: i32, a12: i32, a13: i32); }
extern "C" { pub fn f_077890_cpathfind_findnodeorientationforcarplacement_rage_cnodeaddress_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64) -> f32; }
extern "C" { pub fn f_077900_cpathfind_findnodecoorsforscript_rage_cnodeaddress_bool_ptr_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077930_cvehicleailod_getdummymode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078017_cheliintelligence_islandingareablocked_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078068_ctaskvehicleconvertibleroof_latchroof_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078069_ctaskvehicleconvertibleroof_unlatchroof_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078780_canchorhelper_isvehicleanchorable_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078782_canchorhelper_getanchorhelperptr_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078785_canchorhelper_cananchorhere_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078786_cvehicle_getnumcontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078800_cwheel_reset_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078827_cvehicle_setiswrecked(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078853_cvehicle_processsirenandhorn_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078867_cvehicledeformation_readfromvectoroffset_void_const_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_078876_camphibiousquadbike_iswheeltransitionfinished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078884_cvehicle_hasrammingscoop_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078888_ccardoor_settargetdooropenratio_float_unsigned_int_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64); }
extern "C" { pub fn f_078891_ccardoor_getislatched_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_078892_cvehicle_hasramp_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078893_cheli_getiscargobob_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078895_camphibiousquadbike_forcewheelsout(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078896_camphibiousquadbike_forcewheelsin(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078898_cvehiclepopulation_updatevehcount_cvehicle_ptr_cvehiclepopulatio(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078910_cautomobile_slipperyboxupdatelimits_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078917_cvehiclefactory_modifyvehicletopspeed_cvehicle_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078918_cvehicle_setcardoorlocks_ecarlockstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078924_cvehicle_getwheelfromid_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_078933_cvehicle_isupsidedown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078934_cvehicle_isonitsside_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078935_cvehicle_getisextraon_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078936_cvehicle_ishornon_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078937_ccardoor_setshutandlatched_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_078944_cvehicledamage_applydamage_centity_ptr_edamagetype_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: i64, a6: i64, a7: i64, a8: i32, a9: i64, a10: i32, a11: i32, a12: i32, a13: f32, a14: i32, a15: i32, a16: i32, a17: i32, a18: i32) -> f32; }
extern "C" { pub fn f_078947_cvehicledamage_getisdriveable_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_078951_cvehicle_triggercaralarm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078953_cvehicle_hasrocketboost(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078954_cvehicle_hasjump(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078955_cvehicle_turnsirenon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_078965_cvehicle_isentityattachedtotrailer_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078972_cvehicledamage_getvehiclehealthpercentage_cvehicle_const_ptr_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32) -> f32; }
extern "C" { pub fn f_078974_cvehicledamage_getenginehealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_078975_cvehicledamage_getisstuck_int_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_078978_cvehicle_fixwindow_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078979_cvehicle_rollwindowup_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078990_ccardoor_breakoff_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_078992_ccardoor_breaklatch_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078996_clandinggear_controllandinggear_cvehicle_ptr_clandinggear_ecomma(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_078998_cvehicle_switchengineoff_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079001_caircraftdamagebase_hassectionbrokenoff_cplane_const_ptr_int_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_079018_cvehicle_initextraflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079035_cvehicle_isreversetrike_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079039_cvehicle_isturretseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079041_cvehicle_rolldownwindow_ehierarchyid_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079046_cvehicle_istrike_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079052_cvehicle_killpedsinvehicle_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079065_cvehicle_getdoorfromid_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079070_cvehicle_getseatanimationinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079071_cvehicle_isentryindexvalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079079_cautomobile_enableburnoutmode_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079080_cautomobile_setreducegriplevel_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079081_cautomobile_setreducedsuspensionforce_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079082_cautomobile_resethydraulics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079099_cautomobile_isparachutedeployed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079100_cautomobile_startparachuting(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079108_cbike_setbikeonsidestand_bool_bool_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_079130_cvehicle_getlayoutinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079197_ccargenerator_switchoff(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079198_ccargenerator_switchon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079199_ccargenerator_isused_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079225_cthecargenerators_destroycargeneratorbyindex_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_079226_cvehicle_setliveryid_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079227_cvehicle_setlivery2id_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079228_cvehicle_updatebodycolourremapping_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079230_cthecargenerators_createcargenerator_float_float_float_float_flo(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i32, a14: i32, a15: i32, a16: i32, a17: i32, a18: i64, a19: i32, a20: i32, a21: i32) -> i32; }
extern "C" { pub fn f_079235_cthecargenerators_setallcargeneratorsbacktoactive(ph: *mut Phantom); }
extern "C" { pub fn f_079236_cthecargenerators_removecarsfromgeneratorsinarea_float_float_flo(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_079244_cthecargenerators_clearscheduledqueue(ph: *mut Phantom); }
extern "C" { pub fn f_079254_ccomponentreservationmanager_getdoorreservation_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079267_ccardoor_getisintact_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079275_cvehicle_haswindowtosmash_ehierarchyid_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_079277_ccardoor_setdoorallowedtobebrokenoff_bool_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_079283_cvehicle_carpartscanbreakoff_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079335_chandlingdata_gethandlingdata_ehandlingtype(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079347_chandlingdatamgr_gethandlingdata_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_079359_cturret_aimatworldpos_rage_vector3_const_ref_cvehicle_ptr_bool_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_079366_crotarywingaircraft_breakofftailboom_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079371_crotarywingaircraft_getistailboombroken_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079372_crotarywingaircraft_getismainrotorbroken_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079403_cheli_removeropesandhook(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079404_cvehiclegadgetpickuprope_detachentity_cvehicle_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_079411_cheli_findspawncoordinatesforheli_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079417_cheli_areropesattachedtononhelientity_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079419_cheli_addpickuprope(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079421_cvehiclegadgetpickuprope_setropelength_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32); }
extern "C" { pub fn f_079422_cvehicle_getvehiclegadget_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079436_cvehicledeformation_applycollisionimpact_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_079441_clandinggear_getpublicstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079472_cplane_getverticalflightmodeavaliable_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079473_cplane_getislandinggearintact_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079479_caircraftdamagebase_breakoffsection_cplane_ptr_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_079514_cplane_disableaileron_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079538_cplane_enableindividualpropeller_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079544_cvehicle_setrocketboosting_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079545_cplane_setdesiredverticalflightmoderatio_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079546_cplane_setverticalflightmoderatio_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079553_cplane_arecontrolpanelsintact_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079571_csubmarinehandling_forcethrottlefortime_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_079572_csubmarinehandling_getcurrentdepth_cvehicle_ptr_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_079575_csubmarinehandling_setcrushdepths_bool_float_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_079603_ctrailerlegs_updatetocurrentratio_cvehicle_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079616_cvehicletrailerattachpoint_getattachedtrailer_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_079623_ctrailer_setinversemassscale_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079653_ctrain_removealltrains(ph: *mut Phantom); }
extern "C" { pub fn f_079659_ctrain_findengine_ctrain_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079660_ctrain_disablerandomtrains_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_079661_ctrain_switchtraintrackfromscript_int_bool_rage_scrthreadid(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_079662_ctrain_settraintrackspawnfrequencyfromscript_int_int_rage_scrthr(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_079667_ctrain_updateposition_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079669_ctrain_setcarriagespeed_ctrain_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079673_ctrain_findcaboose_ctrain_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079674_ctrain_findcarriage_ctrain_const_ptr_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079676_ctrain_setnewtrainposition_ctrain_ptr_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079712_ctransmission_setkersallowed_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079725_cvehicle_setgravityforwheellintegrator_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079729_cvehicle_printskeletondata(ph: *mut Phantom); }
extern "C" { pub fn f_079736_cvehicle_handledeactivateforplayback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079744_cvehicle_hidetombstone_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079746_cvehicle_doesvehiclehaveaconvertibleroofanimation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079752_cvehicle_setdebugname_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079753_cvehicle_createvehicleweaponmgr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079758_cvehicle_getconvertibleroofstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079759_cvehicle_canbedeletedspecial_bool_bool_bool_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_079761_cvehiclegadgettowarm_detachvehicle_cvehicle_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_079771_cvehicle_getapproximatesize_rage_vector3_ref_rage_vector3_ref_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079825_cvehicle_removefragmentcomponent_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079832_cvehicle_setdoorlockstatefromdoorid_ehierarchyid_ecarlockstate(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079841_cvehicle_isonfire_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079843_cvehicle_carhasroof_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079846_cvehicle_deployfoldingwings_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079847_cvehicle_getarefoldingwingsdeployed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079849_cvehicle_requestdials_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079862_cvehicle_finishgliding(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079864_cvehicle_deletewindow_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079866_cvehicle_turnoffextra_ehierarchyid_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079880_cvehicle_updatedoorsfornavigation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079885_cvehicle_getentryexitpoint_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079886_cvehicle_getnumberentryexitpoints_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079887_cvehicle_isentrypointindexvalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079889_cvehicle_isentrypointclearforped_cvehicle_const_ref_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_079894_cvehicle_hasanywindowstosmash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079902_cvehicle_setforwardspeed_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079903_cvehicle_setforwardspeedxy_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079904_cvehicle_placeonroadadjust_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_079907_cvehicle_updatehominglockedontostate_centity_ehominglockonstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079910_cvehicle_lowerconvertibleroof_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079911_cvehicle_raiseconvertibleroof_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079913_cvehicle_transformtosubmarine_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079914_cvehicle_transformtocar_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079915_cvehicle_settransformrateforcurrentanimation_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079926_cvehicle_getliveryid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079928_cvehicle_getlivery2id_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079939_cvehiclepopulation_clearinterestingvehicles_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_079945_cvehicle_setexclusivedriverped_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079951_cvehicle_setbobbleheadvelocity_rage_vec3v_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079954_cvehicle_startgliding(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079955_cvehicle_setdeployoutriggers_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079959_cvehicle_applyexplosioneffectemp_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079960_cvehicle_hastombstone_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079987_cvehicledamage_copy_cvehicle_ptr_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079999_cvehicledamage_fixvehicledeformation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080001_cvehicledamage_setbodyhealth_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_080003_cvehicledamage_getnumofbrokenoffparts_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080004_cvehicledamage_getnumofbrokenloosenparts_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080006_cvehicledamage_resetstucktimer_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_080009_cvehicledamage_popoutwindscreen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080010_cvehicledamage_popoffroof_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080011_cvehicledamage_setdamagescales_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_080017_cvehiclefactory_cleardestroyedvehcache(ph: *mut Phantom); }
extern "C" { pub fn f_080029_cvehiclegadgetpickuprope_getattachedvehicle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080054_cvehiclepopulation_getdesirednumberambientvehicles_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> f32; }
extern "C" { pub fn f_080080_cvehicleweaponmgr_getweaponsforseat_int_rage_atarray_cvehiclewea(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_080192_cvehiclefactory_setfardrawallvehicles_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080236_cturret_aimatlocaldir_rage_vector3_const_ref_cvehicle_ptr_bool_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_080264_cvehiclegadget_snapcomponenttoangle_fraginstgta_ptr_int_float_fl(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64); }
extern "C" { pub fn f_080267_cvehicleweaponmgr_getvehicleweaponindexbyhash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_080271_cvehiclegadgetwithjointsbase_getjoint_short(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080274_cvehiclegadgetdiggerarm_getjointpositionratio(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_080283_cvehiclegadgettowarm_addropesandhook_cvehicle_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080294_cvehiclegadgettowarm_attachvehicletotowarm_cvehicle_ptr_cvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_080302_cvehiclegadgetforks_setdesiredacceleration_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_080309_cvehiclegadgethandlerframe_setdesiredacceleration_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_080328_cvehiclegadgetpickuprope_computeidealattachmentoffset_centity_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_080429_cvehiclepopulation_instantfillpopulation(ph: *mut Phantom); }
extern "C" { pub fn f_080432_cvehiclepopulation_setcoordsofscriptcar_cvehicle_ptr_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_080434_cvehiclepopulation_registerinterestingvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080442_cvehiclepopulation_hasinstantfillpopulationfinished(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_080443_cvehiclepopulation_removeallvehshard(ph: *mut Phantom); }
extern "C" { pub fn f_080461_cvehiclepopulation_scriptgeneratevehcreationposfrompaths_rage_ve(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32, a7: i64, a8: i64) -> i32; }
extern "C" { pub fn f_080484_cwheel_settyrewearrate_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_080545_cvehicleentrypointinfo_getwindowid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080688_cvehiclescriptresource_requestanimsfromclipset_rage_fwmvtemplate(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080709_cvehicledrivebyinfo_canuseweapongroup_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_080710_cvehicledrivebyinfo_canuseweapontype_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_080866_cvfxhelper_getmatrixfromboneindex_rage_mat34v_ref_bool_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_080872_cvfxhelper_creatematfromvecyalign_rage_mat34v_ref_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_080879_cvfxhelper_getinterpvalue_float_float_float_bool(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_080960_cfire_shutdown_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081255_rage_cloudhatmanager_flush_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081416_ccoronas_register_rage_vec3v_const_ref_float_rage_color32_float(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: f32, a4: f32, a5: i64, a6: f32, a7: f32, a8: f32, a9: i32); }
extern "C" { pub fn f_081417_cmarkers_register_markerinfo_t_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081498_cmoviemgr_preloadmovie_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_081500_cmoviemgr_release_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081501_cmoviemgr_enddraw_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081502_cmoviemgr_begindraw_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_081503_cmoviemgr_getmemoryusage_unsigned_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_081515_cmoviemgr_gethandlefromscriptslot_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_081526_cmoviemeshmanager_update(ph: *mut Phantom); }
extern "C" { pub fn f_081530_cmoviemeshmanager_loadset_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_081531_cmoviemeshmanager_releaseset_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081532_cmoviemeshmanager_gethandlefromscriptid_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_081631_clodlightmanager_cacheloadedlodlightimapidx(ph: *mut Phantom); }
extern "C" { pub fn f_081632_clodlightmanager_loadlodlightimapidxcache_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081633_clodlightmanager_unloadlodlightimapidxcache_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081853_cvfxscript_getscriptptfxassetname_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081890_cvfxweapon_resetintsmokeoverrides(ph: *mut Phantom); }
extern "C" { pub fn f_082019_cairdefencezone_setplayeristargettable_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082030_cpedequippedweapon_getprojectileordnance(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082040_cairdefencemanager_resetallairdefencezones(ph: *mut Phantom); }
extern "C" { pub fn f_082041_cairdefencemanager_createairdefencesphere_rage_vec3v_float_rage(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_082042_cairdefencemanager_createairdefencearea_rage_vec3v_rage_vec3v_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_082043_cairdefencemanager_deleteairdefencezone_unsigned_char(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082044_cairdefencemanager_getairdefencezone_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082046_cairdefencemanager_isdefencezoneinarea_rage_vec3v_float_unsigned(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_082064_cpedweaponmanager_getequippedvehicleweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082065_cpedweaponmanager_getequippedweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082070_cweaponinfo_getammoinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082071_cpedweaponmanager_getequippedweapon_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082157_cweapon_refillpedsweaponammoinstant_cped_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082162_cweapon_updateshadervariables_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082179_cpedweaponmanager_destroyequippedweaponobject_bool_char_const_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_082182_cweapon_gettimebeforenextshot_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_082187_cweapon_getcanreload_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082203_cweapon_getcamodiffusetexidx_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082204_cweapon_setcamodiffusetexidx_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082215_cpedweaponmanager_dropweapon_unsigned_int_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_082223_cpedinventory_removeweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082253_weaponfactory_create_unsigned_int_int_cdynamicentity_ptr_char_co(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_082258_cweaponscriptresource_dtorcweaponscriptresource(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082264_cweaponscriptresource_requestanimsfromclipset_rage_fwmvtemplatei(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082265_cweaponanimations_op_cweaponanimations_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082266_cweaponanimations_cweaponanimations_cweaponanimations_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082274_cweaponcomponent_updateshadervariables_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082282_cweaponcomponentflashlight_transferactiveness_cweaponcomponentfl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082286_cweaponcomponentflashlight_setflashlightfadedistance_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_082448_cweaponanimationsmanager_doesweaponanimationsetforhashexist_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082518_cweaponinfo_getammomax_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082522_cweaponinfo_getattachpoint_cweaponcomponentinfo_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082554_cpedinventoryloadoutmanager_setdefaultloadout_cped_ptr_cpedmodel(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082555_cpedinventoryloadoutmanager_setloadout_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082561_cpedequippedweapon_createobject_cweaponinfo_const_ptr_int_rage_m(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: f32, a8: i32, a9: i32) -> i64; }
extern "C" { pub fn f_082567_cpedequippedweapon_destroyobject_cobject_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082568_cpedequippedweapon_createweaponcomponent_unsigned_int_cobject_pt(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082569_cpedequippedweapon_destroyweaponcomponent_cobject_ptr_cweaponcom(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082577_cpedinventory_addammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082579_cpedinventory_setammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082580_cpedinventory_addweaponandammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_082581_cpedinventory_addweaponammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082583_cpedinventory_setweaponammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082585_cpedinventory_setweaponusinginfiniteammo_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082594_cpedweaponmanager_equipweapon_unsigned_int_int_bool_bool_cpedequ(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_082595_cweaponitemrepository_addcomponent_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_082596_cweaponitemrepository_removecomponent_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_082599_cpedweaponselector_selectweapon_cped_ptr_epedinventoryselectopti(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_082603_cweaponitem_settintindex_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082643_cpedweaponmanager_getisgadgetequipped_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082644_cpedweaponmanager_equipbestweapon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_082646_cpedweaponmanager_getbestweaponhash_unsigned_int_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_082650_cpedweaponmanager_setbackupweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082663_cammoitemrepository_getammo_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082665_cammoitemrepository_setignoreinfiniteammoflag_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082671_cweaponitem_getkeyfromhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082720_cprojectilemanager_removeallprojectilesbyammotype_unsigned_int_b(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_082724_cprojectilemanager_explodeprojectiles_centity_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082864_tunableslistener_tunableslistener_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082870_cnetwork_gettimeouttime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082885_networkinterface_isanysessionactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082897_cnetwork_flushalllogfiles_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082909_cnetworktelemetry_appendmetric_rage_rlmetric_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082932_cnetworkdebugtelemetry_appenddebugmetric_rage_rlmetric_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082999_networkinterface_registerobject_rage_netgameobjectwrapper_cdynam(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083016_networkinterface_canpause(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083019_networkinterface_issessionestablished(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083025_networkinterface_getphysicalplayerfromindex_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083028_networkinterface_isinspectatormode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083038_networkinterface_isinfreemode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083060_networkutils_getnetworkplayerposition_cnetgameplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083061_networkinterface_getnetworkobject_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083071_networkinterface_setinmpcutscene_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_083087_cnetworkscriptworldstatemgr_revertworldstate_cnetworkworldstated(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083089_cnetworkscriptworldstatemgr_changeworldstate_cnetworkworldstated(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083095_cnetworkvehicleplayerlockingworldstate_cnetworkvehicleplayerlock(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_083098_cnetworkvehicleplayerlockingworldstate_clearmodelidplayerlock_cg(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_083099_cnetworkvehicleplayerlockingworldstate_clearmodelinstanceplayerl(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_083112_networkinterface_settargetlockon_cvehicle_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083124_networkinterface_givepedscriptedtask_cped_ptr_rage_aitask_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_083136_networkinterface_isinsingleplayerprivatesession(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083154_networkinterface_getlowestpedmodelstartoffset_unsigned_int_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083155_networkinterface_getlowestvehiclemodelstartoffset_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083205_networkdebug_displaydebugmessage_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083312_networkcrewemblemmgr_requestemblem_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083342_networkclan_getcrewemblemmgr(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083474_socialclubmgr_getlocalgameraccountinfo_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083496_ctunables_tryaccess_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083497_ctunables_tryaccess_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083683_networkdebug_candebugbreakforfocus_edebugbreaktype_rage_netobjec(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_083768_cnetworkassetverifier_refreshcrc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083769_cnetworkassetverifier_setmemorytampercrc_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083899_csyncedentity_csyncedentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083900_csyncedentity_setentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083902_csyncedentity_getentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083904_csyncedentity_setentityid_unsigned_short_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083922_networkutils_getcanpasscontrolerrorstring_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083937_cnetworkvehiclemodeldoorlockedtablemanager_isvehiclemodellocked(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083938_cnetworkvehiclemodeldoorlockedtablemanager_getplayerlockindexbym(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083939_cnetworkvehiclemodeldoorlockedtablemanager_isvehicleinstanceregi(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083940_cnetworkvehiclemodeldoorlockedtablemanager_getplayerlockindexbyv(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083941_cnetworkvehiclemodeldoorlockedtablemanager_isvehicleinstancelock(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083942_cnetworkvehiclemodeldoorlockedtablemanager_setmodelidplayerlock(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083943_cnetworkvehiclemodeldoorlockedtablemanager_setmodelinstanceplaye(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083944_cnetworkvehiclemodeldoorlockedtablemanager_getplayerlock_unsigne(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083945_cnetworkvehiclemodeldoorlockedtablemanager_getnumplayerlocks(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084088_clivemanager_issystemuishowing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084244_metricplayercoords_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084251_metricjob_ended_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084350_cnetworktelemetry_appendinfometric_unsigned_int_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_084371_cnetworkdebugtelemetry_addautogputelemetry_metricslist_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084390_metricmemory_usage_metricmemory_usage_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_084391_cnetworkdebugtelemetry_appendmetricmemstore_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084604_socialclubmgr_isconnectedtosocialclub_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084631_tunableslistener_dtortunableslistener_138664(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085748_cnetworksession_setsessionstate_cnetworksession_esessionstate_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085897_cnetworksession_sendpresenceinvitestotransition_rage_rlgamerhand(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_085898_cnetworksession_sendpresenceinvites_rage_rlgamerhandle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_085985_cnetworksession_didgamerleaveforstore_rage_rlgamerhandle_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086744_cnetgameplayer_getplayerped_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086763_cnetobjgame_getplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086782_cnetgameplayer_canacceptmigratingobjects_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086785_cnetobjentity_flagfordeletion(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086786_cnetworkobjectmgr_getplayersnearbyinscope_unsigned_int_ref_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_087121_cnetobjped_setpedoutofvehicle_unsigned_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087406_cnetobjphysical_setinvertghosting_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_087730_cnetobjtrain_setusehighprecisionblending_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087752_cnetobjvehicle_islockedforplayer_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_087753_cnetobjvehicle_islockedforteam_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_088195_cnetgameplayer_isinroamingbubble_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_089334_cgivecontrolevent_trigger_rage_netplayer_const_ref_rage_netobjec(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_089430_cgiveweaponevent_trigger_cped_ptr_unsigned_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_089438_cremoveweaponevent_trigger_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_089444_cremoveallweaponsevent_trigger_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_089570_cremotescriptleaveevent_trigger_cgamescriptid_const_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_089577_cmarkasnolongerneededevent_trigger_cnetobjgame_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_089749_cscriptentitystatechangeevent_trigger_rage_netobject_ptr_cscript(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_089824_cmodifyvehiclelockworldstatedataevent_trigger_unsigned_int_int_u(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_089937_cnetworkinfochangeevent_trigger_unsigned_int_unsigned_int_unsign(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_089992_cupdatefxnevent_trigger_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_090710_asinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090741_time(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090755_exp2f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090776_fiprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090801_localtime(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090807_logf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090814_memcmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090818_modff(ph: *mut Phantom, a0: f32, a1: i64) -> f32; }
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
extern "C" { pub fn f_090938_strftime(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_090939_strlen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090940_strncasecmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090941_strncat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090942_strncmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090945_strrchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090947_strstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090956_strtok(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090971_toupper(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090987_vsprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090994_wcsstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_091023_multi3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_091041_void_std_2_introsort_std_2_classicalgpolicy_std_2_ranges_less_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
