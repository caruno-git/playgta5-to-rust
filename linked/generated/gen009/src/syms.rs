//! Callee declarations for crate gen009.
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
extern "C" { pub fn f_000116_rage_sysipcwaitsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000118_rage_sysipcwaitsematimed_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_000119_rage_sysipcsignalsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000131_rage_sysipcwaitthreadexit_rage_sysipcthreadid_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000134_rage_sysipcyield_int(ph: *mut Phantom); }
extern "C" { pub fn f_000135_rage_sysipcsleep_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000589_gwaitconditionimpl_wait_gmutex_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000644_rage_athashnextsize_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000645_rage_athash_const_char_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000650_rage_athash16u_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000652_rage_atstring_reserve_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000654_rage_atstring_set_char_const_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_000665_rage_atstring_op_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000667_rage_atstring_split_rage_atarray_rage_atstring_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
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
extern "C" { pub fn f_000706_rage_bkbank_deletegroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000707_rage_bkbank_popgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000758_rage_bkmanager_openfile_char_ptr_int_char_const_ptr_bool_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_000773_rage_bkmanager_createoutputwindow_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000779_rage_bkmanager_createbank_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000782_rage_bkmanager_findbank_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000783_rage_bkmanager_findwidget_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000785_rage_bkmanager_destroybank_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000786_rage_bkmanager_isbankthread(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000806_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000815_rage_bkmessagebox_char_const_ptr_char_const_ptr_rage_bkmessagebo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
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
extern "C" { pub fn f_000880_rage_bkgroup_addtext_char_const_ptr_rage_atnonfinalhashstring_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000882_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000891_rage_bkgroup_addtoggle_char_const_ptr_rage_atbitset_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_000894_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000897_rage_bkgroup_addslider_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000898_rage_bkgroup_addslider_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000906_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000907_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int_rage_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000916_rage_bkgroup_addcombo_char_const_ptr_unsigned_char_ptr_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_000918_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000919_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000926_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000938_rage_bkgroup_addangle_char_const_ptr_float_ptr_rage_bkangletype(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i64); }
extern "C" { pub fn f_000939_rage_bkgroup_addangle_char_const_ptr_float_ptr_rage_bkangletype(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32); }
extern "C" { pub fn f_000941_rage_bkgroup_addcolor_char_const_ptr_rage_color32_ptr_rage_datca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_000942_rage_bkgroup_addcolor_char_const_ptr_rage_color32_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000990_char_ptr_rage_formatf_char_512ul_char_ref_512ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001047_rage_computerotation_rage_vector3_const_ref_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_001049_rage_matrix33_rotate_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_001062_rage_matrix34_inverse_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001105_rage_autoidregistry_autoidregistry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001108_rage_autoidregistry_unregister_rage_autoiddescriptor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001116_char_ptr_rage_formatf_char_32ul_char_ref_32ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001117_rage_datcallback_datcallback(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001119_rage_datcallback_datcallback_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001176_rage_datresourceinfo_generatemap_rage_datresourcemap_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001210_rage_rsonwriter_begin_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001213_rage_rsonwriter_beginarray_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001214_rage_rsonwriter_end(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001216_rage_rsonwriter_writeint_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001218_rage_rsonwriter_writeint64_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001220_rage_rsonwriter_writeuns_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001221_rage_rsonwriter_writeuns64_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001222_rage_rsonwriter_writefloat_char_const_ptr_float_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_001224_rage_rsonwriter_writestring_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001230_bool_rage_rsonwriter_writevalue_float_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_001232_rage_rsonreader_rsonreader(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001234_rage_rsonreader_init_char_const_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001239_rage_rsonreader_getnextsibling_rage_rsonreader_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001240_rage_rsonreader_getfirstmember_rage_rsonreader_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001243_rage_rsonreader_getname_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001244_rage_rsonreader_getvalue_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
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
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001560_rage_diagcontextmessage_diagcontextmessage_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001561_rage_diagcontextmessage_diagcontextmessage_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_001563_rage_diagcontextmessage_pop(ph: *mut Phantom); }
extern "C" { pub fn f_001565_printf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001568_rage_atmapentry_unsigned_int_wchar_t_ptr_const_ref_rage_atmap_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001569_rage_atmap_unsigned_int_wchar_t_ptr_rage_atmaphashfn_unsigned_in(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001580_rage_fiassetmanager_open_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001581_rage_fiassetmanager_fullreadpath_char_ptr_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_001582_rage_fiassetmanager_createleadingpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001623_rage_fiassetmanager_findextensioninpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001632_rage_fiassetmanager_pushfolder_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001633_rage_fiassetmanager_popfolder(ph: *mut Phantom); }
extern "C" { pub fn f_001635_rage_fiassetmanager_filename_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001636_rage_fiassetmanager_basename_char_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001639_rage_fiassetmanager_removeextensionfrompath_char_ptr_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001652_rage_fidevice_makememoryfilename_char_ptr_int_void_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_001656_rage_fibasetokenizer_init_char_const_ptr_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001674_rage_fibasetokenizer_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001764_rage_atarray_rage_atbinarymap_int_rage_atfinalhashstring_datapai(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001806_rage_fidevice_unmount_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001880_rage_fipackfile_fipackfile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001881_rage_fipackfile_init_char_const_ptr_bool_rage_fipackfile_cachemo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_001883_rage_fipackfile_mountas_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002027_rage_invsqrtf_fast_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_002028_rage_angleclamp_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_002031_rage_mthrandom_mthrandom_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002036_rage_perlinnoise_rawnoise_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_002080_rage_pgbase_pgbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002081_rage_pgbase_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002091_rage_datresource_containsthisaddress_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002092_rage_datresource_getfixup_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002111_rage_parmanager_parmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002112_rage_parmanager_initialize_rage_parsettings_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002128_rage_parmanager_uninitialize(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002129_rage_atmapentry_rage_atnonfinalhashstring_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002130_rage_atmap_rage_atnonfinalhashstring_rage_atnonfinalhashstring_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002131_rage_atnonfinalhashstring_atnonfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002132_rage_parmanager_dtorparmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002147_rage_parmanager_loadtree_char_const_ptr_char_const_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002150_rage_parmanager_savetree_char_const_ptr_char_const_ptr_rage_part(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_002153_rage_parmanager_openinputstream_char_const_ptr_char_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002158_rage_parinstancevisitor_visittoplevelstructure_rage_parparsables(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002159_rage_parmanager_loadfromstructure_rage_partreenode_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002165_rage_parmanager_createandloadanytype_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_002175_rage_partree_dtorpartree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002176_rage_partree_createroot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002177_rage_partreenode_insertaschildof_rage_partreenode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002180_rage_partreenode_clearchildrenanddata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002182_rage_parelement_findattribute_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002184_rage_partreenode_findchildwithname_char_const_ptr_rage_partreeno(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002186_rage_parattribute_findint64value_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002187_rage_parattribute_findboolvalue_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002188_rage_partreenode_appendaschildof_rage_partreenode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002191_rage_parelement_addattribute_char_const_ptr_double_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f64); }
extern "C" { pub fn f_002194_rage_parelement_addattribute_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_002199_rage_partreenode_createstdleaf_char_const_ptr_char_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002203_rage_parelement_findattributeboolvalue_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002205_rage_parelement_findattributedoublevalue_char_const_ptr_double_b(ph: *mut Phantom, a0: i64, a1: i64, a2: f64, a3: i32) -> f64; }
extern "C" { pub fn f_002206_rage_parelement_findattributeint64value_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002209_rage_partreenode_findvaluefrompath_char_const_ptr_bool_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002210_rage_partreenode_findvaluefrompath_char_const_ptr_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002211_rage_partreenode_findvaluefrompath_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002263_void_psofilereader_copyintobuffer_rage_matrix34_void_ptr_rage_ma(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002294_rage_atarray_rage_psorscenumschemavaluedata_0_unsigned_short_gro(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002296_rage_atmap_unsigned_int_rage_atarray_rage_psorscbuilderinstances(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002311_rage_atarray_rage_psoenumtableentry_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002343_rage_parrestregistersingleton_char_const_ptr_rage_parparsablestr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002402_rage_parenumdata_namefromvalueunsafe_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
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
extern "C" { pub fn f_002582_rage_parelement_findattributeanycase_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002593_rage_parsettings_createstandardsettings(ph: *mut Phantom); }
extern "C" { pub fn f_002648_rage_parstream_close(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002695_rage_atbinarymap_rage_pardelegateholderbase_ptr_unsigned_int_dat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002696_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002697_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_002719_strlwr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002720_rage_conststring_vcopy_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002721_rage_conststring_dtorconststring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002739_rage_atpartialstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002743_rage_atdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002746_rage_stristr_char_ptr_char_const_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002748_rage_stringwildcardcompare_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002752_rage_stringendswithi_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002761_rage_widetoutf8_char_ptr_unsigned_short_const_ptr_int_int_int_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_002762_rage_utf8towide_unsigned_short_ptr_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002763_rage_utf8removeinvalidsurrogates_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002790_rage_zlibinflater_zlibinflater(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002798_rage_sysappcontent_isamericanbuild(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002799_rage_sysappcontent_isjapanesebuild(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002842_rage_sysparam_getarray_int_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002862_rage_systhreadtype_addcurrentthreadtype_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_002863_rage_sysperformancetimer_sysperformancetimer_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002864_rage_sysperformancetimer_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002865_rage_sysperformancetimer_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002866_rage_sysperformancetimer_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002867_rage_sysperformancetimer_gettimems(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_002878_rage_atmap_unsigned_int_rage_sysstackcollector_tagentry_rage_atm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002881_unsigned_int_ref_rage_atmap_unsigned_int_unsigned_int_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002888_rage_systhreadpool_workitem_workitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002904_rage_systhreadpool_queuework_rage_systhreadpool_workitem_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002927_rage_sysmemstarttemp(ph: *mut Phantom); }
extern "C" { pub fn f_002928_rage_sysmemendtemp(ph: *mut Phantom); }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_002945_rage_sysmempoolallocator_safeallocate_unsigned_long_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002950_rage_sysmemsimpleallocator_sysmemsimpleallocator_void_ptr_int_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_003085_rage_systaskparameters_systaskparameters(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003090_rage_systaskmanager_create_char_const_ptr_void_ptr_rage_systaskp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_003094_rage_systaskmanager_waitmultiple_int_rage_systaskhandle_ptr_cons(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_003095_rage_systaskmanager_wait_rage_systaskhandle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003097_char_ptr_rage_formatf_char_16ul_char_ref_16ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003136_rage_geomspheres_testspheretotriangle_rage_vec4v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_003140_rage_geomtvalues_findtvalueopensegtopoint_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_003141_rage_geomdistances_distancesegtopoint_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_003142_rage_geomtvalues_findtvaluesegtopoint_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_003159_rage_matrix34_fromquaternion_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003160_rage_matrix34_toquaternion_rage_quaternion_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003162_rage_matrix34_interpolate_rage_matrix34_const_ref_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_003164_rage_matrix34_mirroronplane_rage_vector4_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003171_rage_quaternion_fromrotation_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003172_rage_quaternion_twistangle_rage_vector3_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_003173_rage_vector2_vector2_rage_vector3_const_ref_rage_vector2_evector(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_003175_rage_vector3_vector3_rage_vector2_const_ref_rage_vector2_evector(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003179_rage_atpoolbase_init_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003549_rage_criksolverbase_criksolverbase_rage_criksolverbase_criksolve(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003555_rage_crkinematics_addsolver_rage_criksolverbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003599_rage_crcreaturecomponentskeleton_setsuppressreset_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003823_rage_fwanimdirector_swapmoveoutputbuffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003825_rage_fwanimdirector_startupdateprerender_float_rage_crmtmotiontr(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32); }
extern "C" { pub fn f_003828_rage_fwanimdirector_findpropertyinmovenetworks_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003831_rage_fwanimdirectorcomponentsyncedscene_isplayingsyncedscene_con(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003835_rage_fwanimdirector_getmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003838_rage_fwclipset_getclip_rage_fwmvtemplateid_rage_fwmvclipidtype_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_003840_rage_fwclipsetmanager_getclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003855_rage_fwanimdirector_getframebuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003857_rage_fwanimdirector_waitforprerenderupdatetocomplete(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003878_rage_fwanimhelpers_getmovertracktranslationdiff_rage_crclip_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003880_rage_fwanimhelpers_getmovertrackrotation_rage_crclip_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003882_rage_fwanimhelpers_getmovertrackrotationdiff_rage_crclip_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003896_rage_fwanimmanager_getclipifexistsbydictindex_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_003900_rage_fwclipsetmanager_getclipdictionaryindex_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003968_rage_fwclipset_getclipdictionaryindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003970_rage_fwclipsetmanager_getclipdictionarymetadata_rage_atnonfinalh(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003993_rage_fwclipsetmanager_getclipsetid_rage_atnonfinalhashstring_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004017_rage_atarray_rage_fwmemorysituation_adjustment_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004061_rage_fwexpressionsetmanager_getexpressionsetid_rage_atnonfinalha(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004062_rage_fwexpressionsetmanager_getexpressionset_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004082_rage_fwfacialclipsetgroupmanager_getfacialclipsetgroup_rage_fwmv(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004280_rage_fwposematcher_fwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004368_rage_fwanimdirectorcomponentexpressions_hasexpressions_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004603_rage_fwextensionlist_add_rage_fwextension_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004605_rage_fwextensionlist_unlink_rage_fwextension_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004606_rage_fwarchetype_dtorfwarchetype(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004607_rage_fwarchetype_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004608_rage_fwarchetype_initarchetypefromdefinition_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_004609_rage_fwarchetypemanager_registerstreamedarchetype_rage_fwarchety(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004610_rage_fwarchetypemanager_registerpermanentarchetype_rage_fwarchet(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_004611_rage_fwarchetype_setclipdictionaryindex_rage_atnonfinalhashstrin(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004612_rage_fwarchetypemanager_dumpdynamicarchetypescomponents(ph: *mut Phantom); }
extern "C" { pub fn f_004613_rage_fwarchetype_setdrawdictfile_rage_atnonfinalhashstring_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_004614_rage_fwarchetype_setdrawableorfragmentfile_rage_atnonfinalhashst(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_004617_rage_fwarchetype_findtxdslotindex_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004618_rage_fwarchetype_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004619_rage_fwarchetype_getdrawable_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004620_rage_fwarchetype_getfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004621_rage_fwarchetype_getassetparenttxdindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004622_rage_fwarchetype_updateboundingvolumes_rage_phbound_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004623_rage_fwarchetype_createdynamiccomponentifmissing(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004624_rage_fwarchetype_setphysics_rage_pharchetypedamp_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004625_rage_fwarchetype_setexpressiondictionaryindex_rage_atnonfinalhas(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004626_rage_fwarchetype_setexpressionhash_rage_atnonfinalhashstring_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004627_rage_fwarchetype_setcreaturemetadatafile_rage_atnonfinalhashstri(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004630_rage_fwarchetypemanager_reset(ph: *mut Phantom); }
extern "C" { pub fn f_004631_rage_fwarchetypemanager_getarchetypename_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004632_rage_fwarchetypemanager_lookupmodelid_rage_fwarchetype_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004633_rage_fwarchetypemanager_getarchetype_rage_atnonfinalhashstring_u(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004634_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004638_rage_fwarchetypemanager_freearchetypes_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004645_rage_fwarchetypestreamingmodule_getdependencies_rage_strlocalind(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_004646_rage_fwarchetypestreamingmodule_load_rage_strlocalindex_void_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_004648_rage_fwattachmententityextension_isattachstatebasicderived_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004661_rage_fwdynamicentitycomponent_fwdynamicentitycomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004665_rage_fwentity_fwentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004670_rage_fwentity_onactivate_rage_phinst_ptr_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004679_rage_fwentity_getskeletonsize_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004680_rage_fwentity_printskeletonsummary_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004681_rage_fwentity_waitforanyactiveanimupdatetocomplete_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004683_rage_fwentity_getaabb_rage_spdaabb_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004685_rage_fwentity_getrootlod(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004688_rage_fwextensionlist_destroy_rage_fwextension_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004689_rage_fwextensionlist_destroy_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004693_rage_fwsceneupdate_isinsceneupdate_rage_fwentity_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004702_rage_atpagedpool_rage_fwmatrixtransform_false_rage_fwdefaultrscm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_004703_rage_atpagedpool_rage_fwquaterniontransform_false_rage_fwdefault(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004704_rage_atpagedpool_rage_fwmatrixtransform_false_rage_fwdefaultrscm(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004705_rage_atpagedpool_rage_fwsimpletransform_false_rage_fwdefaultrscm(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004708_rage_fwsimpletransform_op_new_unsigned_long(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004709_rage_fwtransform_delete(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004767_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004784_rage_fwpickermanager_addentitytopickerresults_rage_fwentity_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004788_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004792_rage_fwpickermanager_getselectedentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004793_rage_fwpickermanager_getentity_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004794_rage_fwpickermanager_getnumberofentities(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_004816_rage_fwvectormap_drawline_rage_vector3_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_004820_rage_fwvectormap_drawcircle_rage_vector3_const_ref_float_rage_co(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_004864_rage_atmap_unsigned_int_rage_fwmodelid_rage_atmaphashfn_unsigned(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004935_rage_dlcmdbase_op_new_unsigned_long_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005095_rage_clothmanager_findpedinclothinstanceshutdown_rage_phboundcom(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005170_rage_ropedatamanager_unloadropetexures(ph: *mut Phantom); }
extern "C" { pub fn f_005171_rage_ropedatamanager_loadropetexures(ph: *mut Phantom); }
extern "C" { pub fn f_005315_rage_fwrenderthreadinterface_flush_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005320_rage_fwassetlocation_set_rage_estoretype_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005326_rage_fwtxdstore_getishdcapable_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005328_rage_fwdwdstore_getishdcapable_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005354_rage_fwmapdata_associateextensiondef_unsigned_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_005356_rage_fwmaptypesstore_getname_rage_strlocalindex_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005400_rage_fwmapdatastore_isscriptmanaged_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005451_rage_fwmaptypes_associatearchetypedef_unsigned_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_005452_rage_fwmaptypes_associateextensiondef_unsigned_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_005461_rage_fwboxstreamer_hasloadedaboutpos_rage_vec3v_const_ref_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_005502_rage_fwassetstore_rage_pgdictionary_rage_charactercloth_rage_fwc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005562_rage_fwassetstore_rage_pgdictionary_rage_grctexture_rage_fwtxdde(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005564_rage_fwassetstore_rage_pgdictionary_rage_grctexture_rage_fwtxdde(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005589_rage_fwpsostoreloader_unload_rage_fwpsostoreloadinstance_ref_boo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005648_rage_fwpsostoreloader_load_char_const_ptr_rage_fwpsostoreloadins(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_005650_rage_fwpsostoreloader_loaddataintoobjectcore_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005651_rage_fwpsostoreloader_makesimplefileloadercore(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005700_rage_fwtxdstore_remove_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005705_rage_fwtxdstore_register_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005706_rage_fwtxdstore_getsafefromindex_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005714_rage_fwtxdstore_removeref_rage_strlocalindex_rage_strrefkind(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005729_rage_fwinteriorlocation_fwinteriorlocation_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_005730_rage_fwinteriorlocation_makeinvalid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005732_rage_fwinteriorlocation_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005733_rage_fwinteriorlocation_getroomindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005742_rage_fwworldmgr_addtoworld_rage_fwentity_ptr_rage_fwrect_const_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005743_rage_fwworldmgr_removefromworld_rage_fwentity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005817_rage_fwassetstore_rage_crclipdictionary_rage_fwclipdictionarydef(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005839_rage_fwassetstore_rage_fragtype_rage_fwfragmentdef_addslot_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005917_rage_fwrefawarebaseimpl_rage_datbase_dtorfwrefawarebaseimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005928_rage_aitaskmanager_aitaskmanager_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_005929_rage_aitaskmanager_dtoraitaskmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005931_rage_aitaskmanager_process_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_005940_rage_aitasktree_aitasktree_rage_fwentity_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_005943_rage_atfixedarray_rage_taskregdref_rage_aitask_5_dtoratfixedarra(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005946_rage_aitasktree_cleartask_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005949_rage_aitasktree_findtaskbytypeactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005950_rage_aitasktree_findtaskbytypewithpriority_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_005954_rage_tdynamicobject_setflagsfornewobject(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005964_rage_cnavmesh_getvertex_unsigned_int_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_005966_rage_cnavmesh_getpolynormal_rage_vector3_ref_rage_tnavmeshpoly_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_005967_rage_cnavmesh_getpolyclosestedgemidtopoint_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005968_rage_cnavmesh_getpolyarea_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_005969_rage_cnavmesh_getpolycentroid_unsigned_int_rage_vector3_ref_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_005970_rage_cspeciallinkinfo_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005971_rage_cnavmesh_switchpolysinarea_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_005973_rage_cnavmesh_rayintersectspoly_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_005975_rage_cnavmesh_calculateallpolyminmaxesfordynamicnavmesh(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005976_rage_cnavmesh_getpolycentroidquick_rage_tnavmeshpoly_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_005978_rage_cnavmesh_getpolybelowpoint_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_005979_rage_cnavmesh_getintersectpoly_rage_vector3_const_ref_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005980_rage_cnavmesh_getintersectpolynoquadtree_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005981_rage_cnavmesh_getclosestnavmeshpolyedge_rage_vector3_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64) -> i64; }
extern "C" { pub fn f_005983_rage_cnavmesh_getclosestpointinpolyr_rage_tshortminmax_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64); }
extern "C" { pub fn f_005984_rage_createpointsinpolynormal_rage_cnavmesh_const_ptr_rage_tnavm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_005985_rage_cnavmesh_forallpolys_r_rage_cnavmeshquadtree_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_005986_rage_cnavmesh_getnonstandardadjacancyatposition_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_005988_rage_cnavmesh_linesegsintersect2d_rage_vector3_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_006003_rage_fwpathserver_forceabortcurrentpathrequest_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006004_rage_fwpathserver_getnavmeshindexfromsector_int_int_rage_ainavdo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006007_rage_fwpathserver_getnavmeshextentsfromindex_unsigned_int_rage_v(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_006008_rage_tpathfindvars_insertpoly_int_rage_vector3_const_ref_rage_tn(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_006010_rage_fwpedgennavmeshiterator_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006011_rage_fwpedgennavmeshiterator_addnavmesh_rage_cnavmesh_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006012_rage_fwpedgennavmeshiterator_startnewinteration_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006013_rage_fwpedgennavmeshiterator_isatend(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006014_rage_fwpedgennavmeshiterator_getnext_rage_cnavmesh_ptr_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_006016_rage_cpathserverbinheap_heapifyup_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006018_rage_cpathsearchpriorityqueue_dtorcpathsearchpriorityqueue(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006019_rage_cpathsearchpriorityqueue_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006020_rage_cpathsearchpriorityqueue_insert_float_float_rage_tnavmeshpo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i64, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_006021_rage_cpathsearchpriorityqueue_removetop_float_ref_float_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_006022_rage_caudiorequest_insertaudioproperty_float_unsigned_int_rage_v(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: f32, a5: i32); }
extern "C" { pub fn f_006023_rage_fwpathserver_detessellateallpolys(ph: *mut Phantom); }
extern "C" { pub fn f_006024_rage_fwpathserver_isroomtotessellatethispoly_rage_tnavmeshpoly_c(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006025_rage_fwpathserver_cantessellatethispoly_rage_tnavmeshpoly_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006026_rage_fwpathserver_getnextfreetessellationpolyindex(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_006035_rage_atmapentry_unsigned_int_bool_const_ref_rage_atmap_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006056_rage_strstreaminginfomanager_removeobject_rage_strindex_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006067_rage_strpackfilemanager_getimagefileindexfromhandle_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006074_rage_strstreaminginfomanager_removeobjectanddependents_rage_stri(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006075_rage_strstreaminginfo_computevirtualsize_rage_strindex_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006076_rage_strstreaminginfo_computephysicalsize_rage_strindex_bool_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006080_rage_strstreaminginfo_setflag_rage_strindex_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006084_rage_strstreaminginfomanager_requestobject_rage_strindex_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006085_rage_strpackfilemanager_registerindividualfile_char_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_006088_rage_atarray_rage_atnonfinalhashstring_0_unsigned_short_grow_uns(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006092_rage_strstreamingmodulemgr_getmodulefromindex_rage_strindex_cons(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006101_rage_strstreaminginfomanager_setrequiredflag_rage_strindex_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006124_rage_strstreamingengine_getobjectname_rage_strindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006141_rage_strstreamingmodule_getobjectanddependencies_rage_strindex_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_006156_rage_atarray_rage_strindex_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006162_rage_strstreaminginfo_gethandle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006167_rage_strstreaminginfomanager_isobjectinimage_rage_strindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006175_rage_strstreaminginfomanager_clearrequiredflag_rage_strindex_uns(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006184_rage_strstreaminginfomanager_printalldependents_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006189_rage_strstreaminginfomanager_registerdummyobject_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006190_rage_strstreaminginfomanager_unregisterobject_rage_strindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006235_rage_strstreamingvisualize_setmarkertext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006238_rage_strstreamingloadermanager_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006241_rage_strstreamingvisualize_addmarker_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006249_rage_strstreamingmodule_resetplacestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006250_rage_strstreamingmodule_getrefcountstring_rage_strlocalindex_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_006253_rage_strmoduletypeindextoextension_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006254_rage_strrequest_request_rage_strlocalindex_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_006255_rage_strrequest_hasloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006256_rage_strrequest_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006258_rage_strrequest_clearmyrequestflags_unsigned_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006610_rage_audsound_decompressmetadata_untyped_rage_sound_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006925_rage_audcontroller_clearallsounds(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007036_rage_audmetadatamanager_getobject_verify_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007047_rage_audsoundfactory_getmetadataptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007078_rage_audsoundset_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007084_rage_audtracker_audtracker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007085_rage_audtracker_dtoraudtracker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007412_rage_audentity_createandplaysound_unsigned_int_rage_audsoundinit(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007420_rage_vfxlist_vfxlist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007421_rage_vfxlist_additem_rage_vfxlistitem_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007422_rage_vfxlist_removeitem_rage_vfxlistitem_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007423_rage_vfxlist_removehead(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007424_rage_vfxlist_removeall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007636_rage_atmapentry_unsigned_int_rage_fwidinstanceinfo_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007658_rage_fwptrlistsinglelink_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007661_rage_fwptrlist_ismemberoflist_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007676_rage_fwangle_limitradianangleforpitch_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_007678_rage_fwangle_lerptowards_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_007739_rage_fwaudioentity_dtorfwaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007760_rage_netinterface_getnumremotephysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007763_rage_netinterface_getremotephysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007767_rage_netinterface_getphysicalplayerfromindex_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_007803_rage_networklogutils_writelogevent_rage_netlogginginterface_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007805_rage_netarraymanager_getarrayhandler_unsigned_short_rage_netarra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_007848_rage_neteventmgr_checkforspaceinpool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007849_rage_neteventmgr_prepareevent_rage_netgameevent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007949_rage_netgameevent_netgameevent_unsigned_short_bool_bool_rage_net(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_007995_rage_matrix34_toeulersxyz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008056_rage_networklogutils_writeplayertext_rage_netlogginginterface_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_008074_rage_netobject_ispendingownerchange_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008320_rage_netsynctree_dirtynode_rage_netsynctreetargetobject_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008321_rage_netsynctree_forcesendofnodedata_unsigned_int_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_008412_netprofanityfilter_verifystringforprofanity_char_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_008413_netprofanityfilter_getstatusforrequest_int_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008445_rage_atmap_rage_atstring_rage_atstring_rage_atmaphashfn_rage_ats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008682_rage_fwbasepool_fwbasepool_int_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_008683_rage_fwconfigmanager_getsizeofpool_unsigned_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008686_rage_fwbasepool_getindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008688_rage_fwbasepool_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008689_rage_fwscriptguid_createguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008853_rage_fwconfig_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_008866_rage_atmapentry_unsigned_int_int_const_ref_rage_atmap_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008871_rage_fwconfig_mergenonpooldata_rage_fwconfig_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008912_rage_fwbasepool_new_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008918_rage_fwtimer_startuserpause_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008943_rage_fwbasepool_fwbasepool_char_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_008945_rage_fwbasepool_forall_bool_ptr_void_ptr_void_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008949_rage_fwbasepool_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008950_rage_fwbasepool_getjustindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008951_rage_fwbasepool_isvalidptr_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008953_rage_fwknownrefholder_fwknownrefholder_void_ptr_ptr_rage_fwknown(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_008996_rage_fwbasepool_dtorfwbasepool_14863(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009048_rage_crproperty_getattribute_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009050_rage_crproperty_stringkey_calckey_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009207_rage_crskeletondata_findbonedata_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009211_rage_crskeletondata_computeobjecttransform_int_rage_mat34v_ref_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009218_rage_crskeleton_crskeleton(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009219_rage_crskeleton_dtorcrskeleton(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009222_rage_crskeleton_init_rage_crskeletondata_const_ref_rage_mat34v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009223_rage_crskeleton_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009224_rage_crskeleton_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009226_rage_crskeleton_partialupdate_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009227_rage_crskeleton_getterminatingpartialbone_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009230_rage_crskeleton_inverseupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009233_rage_crskeleton_getglobalmtx_unsigned_int_rage_mat34v_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009234_rage_crskeleton_setglobalmtx_unsigned_int_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009291_rage_grmshader_begindraw_rage_grmshader_edrawtype_bool_rage_grce(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_009293_rage_grmshader_bind_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009294_rage_grmshader_unbind_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009295_rage_grmshader_enddraw_const(ph: *mut Phantom); }
extern "C" { pub fn f_009314_pixend(ph: *mut Phantom); }
extern "C" { pub fn f_009317_rage_grmshader_load_char_const_ptr_rage_fitokenizer_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_009321_rage_grmmodelfactory_builddeclarator_rage_grcfvf_const_ptr_rage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009341_rage_grmshadergroup_grmshadergroup_rage_grmshadergroup_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009343_rage_grmmatrixset_create_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_009344_rage_grmmatrixset_update_rage_crskeleton_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009442_pixbegincn_unsigned_int_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_009454_rage_matrix34_makerotateto_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009499_rage_grcfvf_setposchannel_bool_rage_grcfvf_grcdatasize_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009500_rage_grcfvf_setnormalchannel_bool_rage_grcfvf_grcdatasize(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009501_rage_grcfvf_setdiffusechannel_bool_rage_grcfvf_grcdatasize(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009502_rage_grcfvf_setspecularchannel_bool_rage_grcfvf_grcdatasize(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009506_rage_grcfvf_settexturechannel_int_bool_rage_grcfvf_grcdatasize(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009528_rage_grceffect_create_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009529_rage_grcstateblock_createsamplerstate_rage_grcsamplerstatedesc_c(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009539_rage_grceffect_lookuptechnique_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009540_rage_grceffect_lookupvar_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009542_rage_grcstateblock_createblendstate_rage_grcblendstatedesc_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009545_rage_grcstateblock_setblendstate_rage_grcstateblock_grcblendstat(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009555_rage_grcstateblock_setrasterizerstate_rage_grcstateblock_grcrast(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009559_rage_grcdevice_clear_bool_rage_color32_bool_float_bool_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_009560_rage_grcstateblock_setdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009561_rage_grcbegin_rage_grcdrawmode_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009562_rage_grcvertex_float_float_float_float_float_float_rage_color32(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: f32, a8: f32); }
extern "C" { pub fn f_009563_rage_grcend(ph: *mut Phantom); }
extern "C" { pub fn f_009587_rage_grclightstate_setenabled_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_009608_rage_grcviewport_grcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009615_rage_grceffect_lookupglobalvar_char_const_ptr_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009635_rage_grcviewport_setcurrentworldmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009640_rage_grcviewport_isspherevisible_float_float_float_float_float_p(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_009646_rage_grcviewport_transform_rage_vec3v_const_ref_float_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009649_rage_grcworldidentity(ph: *mut Phantom); }
extern "C" { pub fn f_009650_rage_grcbindtexture_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009653_rage_grcviewport_op_rage_grcviewport_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009675_rage_grcstateblock_createdepthstencilstate_rage_grcdepthstencils(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009716_rage_grcdevice_createvertexdeclaration_rage_grcvertexelement_con(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009753_rage_grcworldmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009766_rage_grcvertexbuffer_create_int_rage_grcfvf_const_ref_rage_grcbu(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_009814_rage_grceffect_setvar_rage_grcinstancedata_ref_rage_grceffectvar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009858_rage_grcstateblock_createrasterizerstate_rage_grcrasterizerstate(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009867_rage_grcinstancedata_dtorgrcinstancedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009868_rage_grceffect_lookupvarbyhash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009875_rage_grceffect_setlocalcommon_rage_grcinstancedata_ref_rage_grce(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_009880_rage_grceffect_getvar_rage_grcinstancedata_const_ref_rage_grceff(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009908_rage_monitorconfiguration_update(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009912_rage_monitorconfiguration_getlandscapemonitor_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009986_rage_grcdebugdraw_adddebugoutputex_bool_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_009995_rage_grcdevice_disablescissor(ph: *mut Phantom); }
extern "C" { pub fn f_010003_rage_grcdevice_setscissor_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_010166_rage_grcdebugdraw_quad_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010172_rage_grcdebugdraw_text_float_float_rage_color32_char_const_ptr_b(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i64, a4: i32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010179_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010181_rage_grcdebugdraw_poly_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010182_rage_grcdebugdraw_poly_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_010183_rage_grcdebugdraw_cross_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010188_rage_grcdebugdraw_boxaxisaligned_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010192_rage_grcdebugdraw_cone_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010195_rage_grcdebugdraw_arrow_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_010216_rage_grcdebugdraw_text_float_float_rage_dd_epixelcoordspace_rage(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i32, a4: i64, a5: i32, a6: f32, a7: f32); }
extern "C" { pub fn f_010219_rage_grcdebugdraw_adddebugoutputex_bool_rage_color32_char_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_010220_rage_grcdebugdraw_adddebugoutput_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010224_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010481_rage_iomapper_iomapper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010482_rage_iomapper_map_rage_iomappersource_unsigned_int_rage_iovalue(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_010490_rage_iomapper_update_unsigned_int_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_010500_rage_iovalue_isdown_float_rage_iovalue_readoptions_const_ref_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010502_rage_iovalue_historyhelddown_unsigned_int_float_float_rage_ioval(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_010504_rage_iovalue_historyheldup_unsigned_int_float_float_rage_iovalue(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_010505_rage_iovalue_wasdown_float_rage_iovalue_readoptions_const_ref_co(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010507_rage_iovalue_historypressedandheld_unsigned_int_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_010509_rage_iovalue_historyreleasedandheld_unsigned_int_unsigned_int_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_010511_rage_iovalue_ispressed_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010513_rage_iovalue_historypressed_unsigned_int_unsigned_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_010514_rage_iovalue_isreleased_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010516_rage_iovalue_historyreleased_unsigned_int_unsigned_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_010517_rage_iovalue_setnextvalue_float_rage_iosource_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_010518_rage_iovalue_getnorm_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010521_rage_iovalue_getnorm01_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010668_rage_rmcdrawable_rmcdrawable(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010678_rage_rmctypefileparser_registerloader_char_const_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_010684_rage_rmctypefileparser_processtypefile_rage_fitokenizer_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_010688_rage_rmclodgroup_computebucketmask_rage_grmshadergroup_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010714_rage_rmctypefileparser_rmctypefileparser(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010716_rage_rmctypefileparser_dtorrmctypefileparser(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010718_rage_rmctypefileparser_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010737_rage_spdaabb_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_011734_rage_fraginst_getarticulatedbody(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011735_rage_fraginst_getarticulatedcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011745_rage_fraginst_setresetmatrix_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011746_rage_fraginst_dtorfraginst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011748_rage_fraginst_getpartbroken_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011751_rage_fraginst_getcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011752_rage_fragtype_getcomponentfromboneindex_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_011755_rage_fraginst_getgroupfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011756_rage_fragtype_getgroupfromboneindex_int_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011764_rage_fraginst_deleteabovegroup_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011766_rage_fragcacheentry_setmass_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_011768_rage_evtplayer_stop_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_011769_rage_fraginst_zeroskeletonmatricesbygroup_rage_crskeleton_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011770_rage_fragtype_getrootlinkconstraint_rage_matrix34_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_011776_rage_fragtype_getboneindexfromid_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011779_rage_fraginst_reducegroupdamagehealth_int_float_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_011798_rage_fraginst_precomputeimpacts_rage_phcontactiterator(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011803_rage_phcollider_setcollidermatrixfrominstance(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011805_rage_fragcacheentry_lazyarticulatedinit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011811_rage_fragcachemanager_releasecacheentry_rage_fragcacheentry_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011822_rage_matrix34_dot_rage_matrix34_const_ref_rage_matrix34_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011825_rage_fraginst_poseboundsfromarticulatedbody(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011826_rage_fraginst_setmusclesfromskeleton_rage_crskeleton_const_ref_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_011827_rage_geomvectors_computeleanandtwistangles_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_011837_rage_fraginst_prepareforactivation_rage_phcollider_ptr_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_011876_rage_fragphysicslod_gettotalundamagedmass_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_011881_rage_fragtype_computemassproperties_int_rage_vec3v_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_011884_rage_fragphysicslod_applydamping_rage_pharchetypedamp_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011885_rage_phiterator_dtorphiterator(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011886_rage_fragphysicslod_computechildangularinertias(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011888_rage_evtparamlist_init_rage_evtparamscheme_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011890_rage_fragtype_scaleundamagedchildren_int_rage_scalarv_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_011891_rage_fragphysicslod_setundamagedchildmassproperties_unsigned_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_011955_rage_fragdrawable_fragdrawable_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_011964_rage_fragtype_getskeletondata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011965_rage_evtset_op_rage_evtset_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011970_rage_fragtype_computeminbreakingimpulses_int_float_ptr_rage_atbi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011985_rage_atarray_rage_datowner_rage_fragtypeenvcloth_0_unsigned_shor(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_012009_rage_fragtypegroup_fragtypegroup_rage_datresource_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012015_rage_fragtypechild_set_rage_fragtypechild_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_012019_rage_evtset_evtset_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012021_rage_fragphysicslodgroup_getlodbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012030_rage_fragphysicslod_fragphysicslod_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012042_rage_fragcacheentry_resetselfcollisions(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012435_rage_atmapentry_unsigned_int_void_ptr_rage_ptxbehaviour_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012843_rage_environmentcloth_getcloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012847_rage_environmentcloth_environmentcloth_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012910_rage_charactercloth_charactercloth_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013112_rage_bgglassmanager_getglassdrawable_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014020_rage_nettask_cancel_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014067_rage_nettimesync_gettime_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014068_rage_nettimesync_hasstarted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014257_rage_is_ui_showing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014284_rage_rlfriendsmanager_isfriendswith_int_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_014285_rage_rlcloud_unwatchmemberitem_rage_rlcloudwatcher_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014288_rage_rltaskmanager_canceltask_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014292_rage_rlgettaskmanager(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_014294_rage_rlgetposixtime_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014316_rage_rlgamerinfo_getgamerhandle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014318_rage_rlgamerinfo_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014361_rage_rlros_getcredentials_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014362_rage_rlroscredentials_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014384_rage_rlpresence_issignedin_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014446_rage_rlroscredentials_getrockstarid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014449_rage_rlpresence_adddelegate_rage_atdelegator1_void_rage_rlpresen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014450_rage_rlpresence_removedelegate_rage_atdelegator1_void_rage_rlpre(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014466_rage_rlroscredentials_getrockstaraccount_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014662_rage_rlmetric_dtorrlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014685_rage_rlmetric_rlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015039_rage_rlcloudmemberid_rlcloudmemberid_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015052_rage_rlcloud_watchmemberitem_int_rage_rlcloudonlineservice_char(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_015770_rage_canslerp_rage_quatv_const_ref_rage_quatv_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015775_rage_phmathinertia_findcylinderanginertia_float_float_float_rage(ph: *mut Phantom, a0: f32, a1: f32, a2: i64); }
extern "C" { pub fn f_015812_rage_phconstraintbase_getinstancea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015813_rage_phconstraintbase_getinstanceb(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015840_rage_phcontact_computelocalpointbfromworld_rage_phmanifold_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015841_rage_phcontact_computelocalpointafromworld_rage_phmanifold_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015843_rage_phconstraintdistance_getworldposa(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015844_rage_phconstraintdistance_getworldposb(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015862_rage_phconstrainthalfspace_getworldposa(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015863_rage_phconstrainthalfspace_getworldposb(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015882_rage_phconstraintmgr_insert_rage_phconstraintbase_params_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015890_rage_phconstraintmgr_remove_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015891_rage_phconstraintmgr_gettemporarypointer_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015893_rage_phmultireaderlocktoken_waitasreader(ph: *mut Phantom); }
extern "C" { pub fn f_015958_rage_phcollider_initinertia(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015965_rage_phcollider_setcollidermatrixfrominstancerigid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015966_rage_phsleep_reset_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015980_rage_phcollider_getlocalvelocityrigid_float_vector_4_const(ph: *mut Phantom, a0: i64, a1: i64, a2: u128); }
extern "C" { pub fn f_015986_rage_phcollider_applyforcecenterofmassimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015987_rage_phcollider_applytorqueimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015988_rage_phcollider_setvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016010_rage_phsimulator_getlastinstancematrix_rage_phinst_const_ptr_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016013_rage_phsimulator_addinactiveobject_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016016_rage_phsimulator_getobjectlocalvelocity_rage_vector3_ref_rage_ph(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016017_rage_phsimulator_deactivateobject_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016036_rage_pharchetype_setbound_rage_phbound_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016043_rage_pharchetype_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016049_rage_pharchetypephys_setmassonly_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016057_rage_pharchetypedamp_pharchetypedamp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016073_rage_phasyncshapetestmgr_phasyncshapetestmgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016074_rage_phasyncshapetestmgr_init_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016076_rage_phasyncshapetestmgr_new_releaseprobe_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016086_rage_phinst_init_rage_pharchetype_ref_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016087_rage_phinst_setarchetype_rage_pharchetype_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016088_rage_phinst_setinstflag_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016130_rage_phshapetest_rage_phshapeprobe_phshapetest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016133_rage_phshapetest_rage_phshapeprobe_testoneobject_rage_phbound_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_016137_rage_phshapetest_rage_phshapeprobe_testoneobject_rage_phinst_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016172_rage_phshapetest_rage_phshapesweptsphere_phshapetest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016173_rage_phshapetest_rage_phshapesweptsphere_testbound_rage_phshapec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_016195_rage_phshapetest_rage_phshapeprobe_initprobe_rage_phsegment_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_016197_rage_phshapetest_rage_phshapesweptsphere_initsweptsphere_rage_ph(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_016248_rage_pharticulatedbody_resetalljointlimitadjustments(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016251_rage_pharticulatedbody_propagateincrementaltolink_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_016254_rage_pharticulatedbody_getlocalvelocitynoprop_int_float_vector_4(ph: *mut Phantom, a0: i64, a1: i32, a2: u128, a3: i64); }
extern "C" { pub fn f_016257_rage_pharticulatedbody_resetdownvel_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016258_rage_pharticulatedbody_seteffectorstozeropose(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016269_rage_pharticulatedbody_setdrivestate_rage_phjoint_drivestate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016271_rage_pharticulatedbody_applyincrementalimpulse_int_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_016283_rage_phbound_getcomputeangularinertia_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_016285_rage_phjoint_setminstiffness_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016287_rage_pharticulatedbody_calculateinertias(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016289_rage_phjoint1dof_applytorque_rage_pharticulatedbody_ptr_float_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: u128); }
extern "C" { pub fn f_016290_rage_phjoint_applytorque_rage_pharticulatedbody_ptr_float_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: u128, a3: u128); }
extern "C" { pub fn f_016301_rage_pharticulatedcollider_setcollidermatrixfrominstanceart(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016312_rage_phboundcomposite_updatebvh_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016317_rage_phboundcomposite_calculatecompositeextents_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016322_rage_pharticulatedcollider_setvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016323_rage_pharticulatedcollider_setangvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016326_rage_pharticulatedcollider_applyforcecenterofmassimp_float_vecto(ph: *mut Phantom, a0: i64, a1: u128, a2: u128); }
extern "C" { pub fn f_016328_rage_pharticulatedcollider_applytorqueimp_float_vector_4_float_v(ph: *mut Phantom, a0: i64, a1: u128, a2: u128); }
extern "C" { pub fn f_016333_rage_pharticulatedcollider_getlocalvelocityart_float_vector_4_in(ph: *mut Phantom, a0: i64, a1: i64, a2: u128, a3: i32); }
extern "C" { pub fn f_016341_rage_phjoint_getchildlink_rage_pharticulatedbody_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016343_rage_phjoint_getjointangularvelocityonaxis_rage_pharticulatedbod(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_016346_rage_phjoint3dof_getthreeanglelimits_float_ref_float_ref_float_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016351_rage_phjoint1dof_computecurrentangle_rage_pharticulatedbody_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016352_rage_phjoint1dof_computerotationalvelocity_rage_pharticulatedbod(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016355_rage_phjoint1dof_setanglelimits_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32); }
extern "C" { pub fn f_016358_rage_phprismaticjoint_computecurrenttranslation_rage_pharticulat(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_016367_rage_phbound_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016370_rage_phbound_setcgoffset_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016385_rage_phbound_istypecomposite_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_016391_rage_phboundbox_phboundbox_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016395_rage_phboundbox_setboxsize_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016436_rage_phboundcapsule_phboundcapsule(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016439_rage_phboundcapsule_setcapsulesize_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_016454_rage_phboundcomposite_phboundcomposite(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016455_rage_phboundcomposite_init_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016456_rage_phboundcomposite_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016464_rage_phboundcomposite_setbound_int_rage_phbound_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016465_rage_phboundcomposite_calccenterofgravity_float_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016470_rage_phboundcomposite_calccenterofbound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016471_rage_phboundcomposite_allocatetypeandincludeflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016530_rage_phboundsphere_phboundsphere_float(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_016531_rage_phboundsphere_setsphereradius_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016635_rage_phintersection_getinstance(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016651_rage_phwindsphere_apply_rage_phwindfield_ptr_unsigned_int_unsign(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016652_rage_phwindthermalgroup_update_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016661_rage_phwind_getlocalvelocity_rage_vec3v_const_ref_rage_vec3v_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_016685_rage_phwindsphere_phwindsphere_rage_phwinddisttype_e_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_016740_rage_phcontactmgr_resetwarmstartallcontactswithinstance_rage_phi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016892_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectlocation(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016893_rage_phlevelnodetree_rage_phlooseoctreenode_rebuildcompositebvh(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016899_rage_phlevelnodetree_rage_phlooseoctreenode_reserveinstlastmatri(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016907_rage_phlevelnodetree_rage_phlooseoctreenode_getfirstculledobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016908_rage_phlevelnodetree_rage_phlooseoctreenode_getnextculledobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016909_rage_phlevelnodetree_rage_phlooseoctreenode_testprobe_rage_phseg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_016915_rage_phverletcloth_computeboundingvolume_rage_phbound_ptr_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016931_rage_scrprogram_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016934_rage_scrprogram_getprogram_rage_scrprogramid(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_016975_rage_scrthread_createthread_rage_scrprogramid_void_const_ptr_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_016981_rage_scrthread_getthread_rage_scrthreadid(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_016999_rage_scrthread_killthread_rage_scrthreadid(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_018477_grefcountntsimpl_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018478_gstring_op_gstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018481_gmatrix2d_setidentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018482_gmatrix2d_prepend_gmatrix2d_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018483_gmatrix2d_append_gmatrix2d_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018496_gmatrix3d_setidentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018535_grefcountimpl_addref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018536_grefcountimpl_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018541_g_wcscmp_wchar_t_const_ptr_wchar_t_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_018544_gstring_gstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018545_gstring_gstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018548_gstring_gstring_gstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018552_gstring_appendchar_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_018556_gstring_op_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018557_gstring_op_gstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018572_gwaitable_dtorgwaitable(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018573_gwaitable_handlerarray_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018575_gwaitable_handlerarray_callwaithandlers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018740_gimageinfo_gimageinfo_gtexture_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_018765_grenderer_cxform_cxform(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018835_cgen9exclusiveassets_isgen9exclusivevehiclemod_cvehiclekit_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_018889_cleveldata_getfriendlyname_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_018921_rage_fwentity_getmodelid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018928_cgarage_ispointinsidegarage_rage_vector3_const_ref_float_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_018984_rage_vector3_isclose_rage_vector3_const_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_019061_cwaypointrecording_getloadedwaypointrecordinginfo_cwaypointrecor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019104_cpopgenshape_categorisepoint_rage_vector2_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019254_ctrafficlights_transfertrafficlightinfo_centity_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019301_csyncedentity_serialise_rage_csyncdatabase_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019325_cspatialarray_findinsphere_rage_vec3v_const_ref_float_cspatialar(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_019347_cspatialarray_findinsphere_rage_vec3v_const_ref_rage_scalarv_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_019443_caidebuginfo_caidebuginfo_caidebuginfo_debuginfoprintparams(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019450_char_ptr_rage_formatf_char_100ul_char_ref_100ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019451_ailogging_getbooleanasstring_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_019452_ailogging_getdynamicentityisclonestringsafe_cdynamicentity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019453_ailogging_getdynamicentitynamesafe_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019456_ailogging_functionlog_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019545_centrypointearlyrejectionevent_centrypointearlyrejectionevent_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_019547_centrypointscoringevent_centrypointscoringevent_int_int_float_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i64, a5: i64); }
extern "C" { pub fn f_019549_centrypointpriorityscoringevent_centrypointpriorityscoringevent(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_019552_cridableentrypointevaluationdebuginfo_reset(ph: *mut Phantom); }
extern "C" { pub fn f_019565_cpedbonetagconvertor_getboneindexfrombonetag_rage_crskeletondata(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_019703_cclipeventtags_cikeventtag_isright_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019704_cclipeventtags_carmsikeventtag_getallowed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019705_cclipeventtags_carmsikeventtag_getleft_const(ph: *mut Phantom, a0: i64) -> i32; }
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
extern "C" { pub fn f_019733_cclipeventtags_crandomizestartphaseproperty_pickrandomstartphase(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_019734_cclipeventtags_crandomizerateproperty_pickrandomrate_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_019805_rage_atarray_rage_vector3_0_unsigned_short_op_rage_atarray_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019806_rage_atarray_rage_vector3_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019948_rage_imp_inverttransformfull_imp34_rage_mat34v_ref_float_vector(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: u128, a4: i64); }
extern "C" { pub fn f_020976_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_020977_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_021148_audscannermanager_cancelanyplayingreports_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021209_audpolicescanner_reportplayercrime_rage_vector3_const_ref_ecrime(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_021332_audobjectaudioentity_audobjectaudioentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021333_audobjectaudioentity_dtoraudobjectaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021455_audpedfootstepaudio_resetwetfeet_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021559_audpolicescanner_reportpolicespottingplayer_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_021561_audscannermanager_triggercopdispatchinteraction_scannercopdispat(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021575_audradiotrack_findtextidforsoundhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021578_audradiotrack_isscoretrack_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021593_audradiotrack_iscommercial_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021595_audradioaudioentity_stopreplaytrackpreview(ph: *mut Phantom); }
extern "C" { pub fn f_021746_audreplayaudioentity_setpreviewmarkersfx_bool_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_021761_audspeechaudioentity_saywhensafe_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_021842_audscriptaudioentity_pickrandominterrupterinvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021843_audspeechaudioentity_getconversationinterruptionoffset_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_022023_rage_fwentity_getskeletondata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022077_rage_fwentity_getobjectmtx_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022381_void_creplayeventmanager_storeevent_cpacketsoundphonering_cpacke(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022388_naaudioentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022658_cped_isdead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022730_auddooraudioentity_auddooraudioentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022731_auddooraudioentity_initdoor_cdoor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022737_auddooraudioentity_processaudio_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_022738_auddooraudioentity_entitywantstoopendoor_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022851_audfrontendaudioentity_notifyfrontendinput(ph: *mut Phantom); }
extern "C" { pub fn f_022852_audfrontendaudioentity_playsound_rage_atnonfinalhashstring_char(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022853_audfrontendaudioentity_playsoundmapzoom(ph: *mut Phantom); }
extern "C" { pub fn f_022854_audfrontendaudioentity_stopsoundmapzoom(ph: *mut Phantom); }
extern "C" { pub fn f_022855_audfrontendaudioentity_playsoundmapmovement_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_022856_audfrontendaudioentity_stopsoundmapmovement(ph: *mut Phantom); }
extern "C" { pub fn f_022858_audfrontendaudioentity_stopsoundreplayscrubbing(ph: *mut Phantom); }
extern "C" { pub fn f_022985_cpathfind_findnodepointersafe_rage_cnodeaddress(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023396_cpacketevent_cpacketevent_ereplaypacketid_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023723_replayfilemanager_updatefavouritescallback(ph: *mut Phantom); }
extern "C" { pub fn f_023734_replayfilemanager_getclipuid_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023865_creplaymgr_jumptofloat_float_unsigned_int(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_023866_creplaymgrinternal_unfreezeaudio(ph: *mut Phantom); }
extern "C" { pub fn f_023868_creplaymgrinternal_setnextplaybackstate_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_023870_creplaymgr_recordmapobject_cobject_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023872_creplaymgr_shouldincreasefidelity(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023876_creplayinterfaceobject_stoprecordingmapobject_cobject_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023877_creplaymgr_recordobject_cobject_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023878_creplaymgr_shouldhidemapobject_cobject_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023880_creplaymgr_shouldforcehideentity_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023881_creplaymgr_hash_unsigned_int_const_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023891_replaybuffermarkermgr_addmarker_unsigned_int_unsigned_int_marker(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_023904_creplaymgrinternal_setcursorspeed_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_023906_creplaycontrol_isplayeroutofcontrol(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023930_creplayeventmanager_shouldproceedwithpacket_cpacketevent_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023935_void_creplayinterfacemanager_getentity_centity_centityget_centit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023967_creplayinterfacemanager_ensureisbeingrecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023968_creplayinterfacemanager_ensurewillberecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023998_rage_fwmodelid_fwmodelid_rage_fwmodelid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_024257_creplaymgrinternal_isusingrecordedcamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024266_creplaymgrinternal_isawareofworldforrecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024268_creplaymgrinternal_getnextaudiobeattime_float_const_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_024269_creplaymgrinternal_getprevaudiobeattime_float_const_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_024289_creplaymgrinternal_checkupdatefavourites_bool_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024293_creplaymgrinternal_isclipusedinanymontage_clipuid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024302_creplaymgrinternal_oncreateentity_cphysical_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024303_creplaymgrinternal_ondeleteentity_cphysical_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024304_creplaymgrinternal_getbuildinginfo_cphysical_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_024305_creplaymgrinternal_recordmapobject_centity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024340_replaymoviecontrollernew_onlinknamedrendertarget_int_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_026174_camgameplaydirector_getfirstpersonshootercamera_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026194_caminterface_isfadedout(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026199_caminterface_isfadedin(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026202_caminterface_isspherevisibleingameviewport_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_026205_cqueriableinterface_istaskcurrentlyrunning_int_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_026209_rage_vector3_normalizesafe_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026248_rage_vector3_normalizesafe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026259_rage_fwentity_getskeleton_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026367_rage_imp_transform_imp34_rage_mat34v_ref_float_vector_4_float_ve(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: u128, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_026648_camdebugdirector_getcamframe_cambasecamera_ptr_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026714_camgameplaydirector_getfirstpersonpedaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_027093_rage_matrix34_fromeulers_rage_vector3_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_027188_camreplaydirector_incrementtarget_int_bool_int_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_027251_rage_matrix34_rotateunitaxis_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_029377_ctask_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029433_getorcreategroup_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029455_cdebugscene_tunewidgetdata_tunewidgetdata_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_029456_cdebugscene_queuetunewidget_cdebugscene_tunewidgetdata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029512_cdebugscene_getworldpositionundermouse_rage_vector3_ref_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_029611_cdebugscene_getmousepointing_rage_vector3_ref_rage_vector3_ref_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029612_worldprobe_cshapetestdesc_setresultsstructure_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029613_worldprobe_cshapetestresults_freeintersections(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029777_char_ptr_rage_safecat_char_char_4_char_ptr_char_const_ref_4_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029866_cdebugscene_focusentities_get_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029878_cdebug_dumpentity_centity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029903_cdebugarchetype_checkdebugarchetypeproxy_cbasemodelinfo_const_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029923_cdebugdrawstore_getstorage_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_029926_cdebugdrawstore_addvectormapline_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_029932_cdebugdrawstore_addarrow_rage_vec3v_const_ref_rage_vec3v_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_029934_cdebugdrawstore_render(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029935_cdebugdrawstore_clearall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029965_rage_fwtransform_getb_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029967_cphysical_gethealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029978_cvectormap_makeeventripple_rage_vector3_const_ref_float_unsigned(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_030309_rage_atarray_centityadd_0_unsigned_short_op_rage_atarray_centity(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030366_rage_grcstateblock_grcdepthstencilstatehandleenum_ref_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030526_cdebugtextureviewerinterface_selecttxd_ctxdref_const_ref_bool_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_030527_cdebugtextureviewerinterface_selectpreviewtexture_rage_grctextur(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_030565_ctxdref_ctxdref_eassettype_int_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_030722_ceventgroupped_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030741_geteventglobalgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030744_ceventnetwork_spewtotty_impl_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030746_geteventscriptnetworkgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030774_cshockingeventsmanager_clearalleventswithsourceentity_centity_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030819_cped_isinjured_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030845_ceventagitated_ceventagitated_cped_ptr_agitatedtype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_030846_ceventagitated_dtorceventagitated(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031069_ceventgivepedtask_ceventgivepedtask_int_rage_aitask_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_031070_ceventgivepedtask_dtorceventgivepedtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031158_ceventfirenearby_dtorceventfirenearby(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031229_ceventcrimereported_ceventcrimereported_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_031565_ceventnetworkentitydamage_dtorceventnetworkentitydamage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031667_rage_atqueue_rage_eventregdref_rage_fwevent_4_dtoratqueue(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033040_gasenvironment_findtarget_gasstring_const_ref_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_033390_gfxglyphrastercache_init_unsigned_int_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_033429_gfxfontcachemanagerimpl_invalidateall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033465_gfxfontlib_gfxfontlib(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033468_gfxfontlib_addfontsfrom_gfxmoviedef_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033478_gfxfontlib_loadfontnames_gstringhash_gstring_gallocatorgh_gstrin(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033487_gfxfontmap_getfontmapping_gfxfontmap_mapentry_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_033573_gfxurlbuilder_defaultbuildurl_gstring_ptr_gfxurlbuilder_location(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033577_gfxloader_initloader_gfxloader_loaderconfig_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033638_gfxmeshcachemanager_setrendergenmemlimit_unsigned_long(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033640_gfxmeshcachemanager_setmeshcachememlimit_unsigned_long(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033790_gfxvalue_objectinterface_objectrelease_gfxvalue_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
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
extern "C" { pub fn f_034154_gfxvalue_objectinterface_gotoandplay_void_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_034155_gfxvalue_objectinterface_setcxform_void_ptr_grenderer_cxform_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034156_gfxvalue_objectinterface_createemptymovieclip_void_ptr_gfxvalue(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_034157_gfxvalue_objectinterface_attachmovie_void_ptr_gfxvalue_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_034837_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_sf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_034844_rage_sfpreallocatedmemorywrapper_computecurrentpreallocationneed(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_034846_rage_sfpreallocatedmemorywrapper_computepreallocation_rage_atfix(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_034863_rage_sfscaleformfunctionhandler_addmethodtoasobject_gfxmovieview(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_034864_rage_sfscaleformfunctionhandler_buildactionscriptobject_gfxmovie(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_034865_rage_sfscaleformmovieview_drawinworldspace_rage_sfscaleformmanag(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034868_rage_sfscaleformmanager_autosetcurrmoviename_autosetcurrmovienam(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034875_rage_sfscaleformmanager_deletemovieview_rage_sfscaleformmovievie(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034876_rage_sfscaleformmanager_deletemovie_rage_sfscaleformmovie_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034877_rage_sfscaleformmanager_destroypreallocatedmemoryarena_unsigned(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_034878_rage_sfscaleformmanager_shutdowndrawtextmanager(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034879_rage_sfscaleformmanager_loadmovie_char_const_ptr_bool_rage_pgdic(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_034884_rage_sfscaleformmanager_getallocatorforarena_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_034946_cloadingscreens_areactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034949_chudtools_getwidescreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034954_chudtools_adjustnormalized16_9valuesforcurrentaspectratio_ewides(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_034967_chudtools_getuiwidth(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_034972_cmousepointer_issfmousereleased_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_034973_stfeeditem_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034975_stfeeditem_dtorstfeeditem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034985_stfeeditem_op_stfeeditem_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035021_cfrontendxmlmgr_registerxmlreader_rage_atnonfinalhashstring_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035022_rage_datcallback_ref_rage_atmap_rage_atnonfinalhashstring_rage_d(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_035023_cfrontendxmlmgr_loadxml_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035026_cnewhud_activatehudcomponent_bool_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_035027_cnewhud_resetallpreviousvalues(ph: *mut Phantom); }
extern "C" { pub fn f_035028_cnewhud_initstartingvalues(ph: *mut Phantom); }
extern "C" { pub fn f_035029_chudtools_calculatehudposition_rage_vector2_rage_vector2_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_035035_cnewhud_removehudcomponent_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035039_chudcolour_getcolourfromkey_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035040_chudcolour_getrgb_ehud_colours_unsigned_char(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_035041_chudtools_getminsafezoneforscaleformmovies_float_ref_float_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_035054_cminimap_getminimapped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_035077_cnewhud_sethudcomponentposition_int_rage_vector2(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_035078_cnewhud_resethudcomponentpositionfromoriginalxmlvalues_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035079_chudcolour_getrgba_ehud_colours(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035085_cnewhud_enablesafezonerender_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035087_chudtools_issuperwidescreen_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035105_chudtools_getuiheight(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_035111_chudcolour_getkeyfromcolour_ehud_colours(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035112_chudcolour_getcolourfromrgba_rage_color32(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035141_cmoviecliptext_setformattedtextwithicons_gfxvalue_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035142_gfxvalue_gfxvalue_gfxvalue_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_035143_gfxvalue_dtorgfxvalue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035152_cmousepointer_setmousecursorstyle_emouse_cursor_style(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035204_gfxvalue_setstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035253_centitypoolindexforblip_centitypoolindexforblip_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_035263_cminimap_creategenericblip_bool_eblip_type_centitypoolindexforbl(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_035266_cminimap_setflag_cminimapblip_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035267_cminimap_createblip_bool_eblip_type_centitypoolindexforblip_cons(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_035268_cminimap_createblip_bool_eblip_type_rage_vector3_const_ref_eblip(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_035272_cminimap_removeblip_cminimapblip_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035282_char_ptr_rage_formatf_char_50ul_char_ref_50ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_035288_cminimap_shouldrenderminimap(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035294_cminimap_setminimapmodestate_esetup_state_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_035308_cminimap_findblipentity_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035312_cminimap_getblipalphavalue_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035315_cminimap_getblipobjectnameid_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035317_cminimap_setblipparameter_eblip_params_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_035320_cminimap_setblipparameter_eblip_params_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_035321_cminimap_setblipparameter_eblip_params_int_rage_vector3_const_re(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_035323_cminimap_isblipidinuse_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035324_cminimap_getblip_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_035326_cminimap_setvisible_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035330_cminimap_setinprologue_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035344_cminimap_updatecentreandnorthblips(ph: *mut Phantom); }
extern "C" { pub fn f_035377_cminimap_common_getscalefromzoomlevel_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_035422_cminimap_renderthread_renderpausemap_rage_vector2_rage_vector2_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_035426_cminimapmenucomponent_initclass(ph: *mut Phantom); }
extern "C" { pub fn f_035427_cminimapmenucomponent_resetblips(ph: *mut Phantom); }
extern "C" { pub fn f_035428_cminimapmenucomponent_setactive_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035429_cminimapmenucomponent_snaptoblipwithdistancecheck_int_float(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035430_cminimapmenucomponent_doesblipexist_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035432_cminimapmenucomponent_setwaypoint(ph: *mut Phantom); }
extern "C" { pub fn f_035487_cbusyspinner_off_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035494_cbusyspinner_on_char_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_035496_cbusyspinner_unregisterinstructionalbuttonmovie_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035513_cpausemenu_getcurrentscreendata(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_035535_ccontextmenu_closemenu(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035548_suicontexts_setactive_rage_athashvalue_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035549_cmenubase_clearwarningcolumn(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035596_cmenubase_showcolumnwarning_ex_pm_columns_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_035600_menuscreenid_getparsername_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035767_cmapmenu_setmapzoom_int_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035900_cweaponwheel_clear_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035909_cweaponwheel_updateweaponattachment_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036031_cscaleformmgr_requestremovemovie_int_emovieownertags(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_036033_cscaleformmgr_rendermovie_int_float_bool_bool_float(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i32, a4: f32); }
extern "C" { pub fn f_036034_cscaleformmoviewrapper_isactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036035_cscaleformmoviewrapper_render_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_036037_cscaleformmgr_updatemovie_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_036038_cscaleformmgr_ismovieactive_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036039_ccomplexobjectarrayitem_getstageroot_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036040_ccomplexobject_getobject_char_const_ptr_char_const_ptr_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_036041_ccomplexobjectarrayitem_settextfield_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036042_ccomplexobjectarrayitem_settextfieldhtml_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036043_ccomplexobjectarrayitem_setvisible_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036044_cscaleformmgr_beginmethodinternal_int_escaleform_base_class_char(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_036045_cscaleformmgr_endmethodinternal_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036047_ccomplexobject_setvisible_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036048_cscaleformmgr_beginmethod_int_escaleform_base_class_char_const_p(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_036049_cscaleformmgr_addparamfloat_double(ph: *mut Phantom, a0: f64); }
extern "C" { pub fn f_036050_cscaleformmgr_endmethod_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036051_cscaleformmgr_addparambool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036052_ccomplexobject_settextfield_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036053_cscaleformmgr_addparamint_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036054_cscaleformmgr_forcecollectgarbage_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036055_rage_atarray_ccomplexobjectarrayitem_0_unsigned_short_grow_unsig(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_036056_cscaleformmgr_addparamstring_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036059_cscaleformmgr_createmovieinternal_cscaleformmgr_createmovieparam(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036060_cscaleformmgr_findmoviebyfilename_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036061_cscaleformmgr_createmovie_char_const_ptr_rage_vector2_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_036062_ccomplexobject_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036063_ccomplexobjectarrayitem_releaseobjectinternal_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036064_cscaleformmoviewrapper_removemovie(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036137_cstreammoviehelper_remove(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036145_cscaleformmoviewrapper_createmovieandwaitforload_escaleform_base(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_036154_cpausemenu_close(ph: *mut Phantom); }
extern "C" { pub fn f_036160_cscaleformmgr_changemovieparams_int_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_036162_cscaleformmoviewrapper_param_addparam(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036166_cpausemenu_setcurrentmenuversion_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_036168_cpausemenu_openlite_menuscreenid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036170_cpausemenu_setmenupreference_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036171_suicontexts_activate_rage_athashvalue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036172_suicontexts_deactivate_rage_athashvalue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036174_cpausemenu_setvaluebasedonpreference_int_cpausemenu_updateprefss(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_036175_cprofilesettings_set_cprofilesettings_profilesettingid_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036176_cpausemenu_redrawinstructionalbuttons_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036177_cpausemenu_generatemenudata_menuscreenid_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036199_cscaleformmoviewrapper_callmethod_char_const_ptr_cscaleformmovie(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_036200_suicontexts_pushlist(ph: *mut Phantom); }
extern "C" { pub fn f_036201_cpausemenu_menuceptiongodeeper_menuscreenid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036202_cpausemenu_menuceptionthekick(ph: *mut Phantom); }
extern "C" { pub fn f_036205_uicontextlist_checkifpasses_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036210_cmenuscreen_drawinstructionalbuttons_menuscreenid_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_036213_cscaleformmenuhelper_set_column_scroll_pm_columns_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_036214_cscaleformmenuhelper_hide_column_scroll_pm_columns(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036215_cscaleformmoviewrapper_beginmethod_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_036216_cscaleformmoviewrapper_endmethod(ph: *mut Phantom); }
extern "C" { pub fn f_036218_suicontexts_poplist(ph: *mut Phantom); }
extern "C" { pub fn f_036219_cmenuscreen_handlecontextmenuinput_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036220_cpausemenu_playinputsound_efrontend_input(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036221_cmenuarray_getcontextmenuoption_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036239_cscaleformmgr_scaleposandsize_rage_vector2_ref_rage_vector2_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_036254_cpausemenu_isactive_pm_activeflags(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036255_cpausemenu_backoutofsavegamelist(ph: *mut Phantom); }
extern "C" { pub fn f_036263_cpausemenu_getactualscreen_menuscreenid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036265_cpausemenu_getcurrentscreen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036269_cpausemenu_setcurrentpane_menuscreenid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036271_cwarningscreen_isactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036288_cpausemenu_opencorrectmenu_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036292_suicontexts_isactive_rage_athashvalue(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036294_cpausemenu_playsound_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036295_cpausemenu_checkinput_efrontend_input_bool_unsigned_int_bool_boo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_036297_cpausemenu_setmaxpaynemode_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036301_cwarningscreen_checkallinput_bool_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_036309_cscaleformmenuhelper_display_data_slot_pm_columns(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036310_cpausemenu_menu_shift_depth_emenuceptiondir_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_036311_cscaleformmenuhelper_set_column_highlight_pm_columns_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_036312_cscaleformmenuhelper_set_column_focus_pm_columns_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036313_cscaleformmgr_forcemovieupdateinstantly_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036314_cwarningscreen_setmessagewithheader_ewarningmessagestyle_rage_at(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_036317_cpausemenu_setnavigatingcontent_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036318_cpausemenu_showtabchangewarning_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036322_cpausemenu_anylayerhasflag_emenuscreenbits(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036330_cpausemenu_renderanimatedspinner_rage_vector2_rage_vector2_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_036331_cpausemenu_renderanimatedspinner_float_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036334_cpausemenu_setgalleryloadingtexture_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036341_cscaleformmenuhelper_show_warning_message_pm_columns_int_char_co(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i32, a8: i64, a9: i32); }
extern "C" { pub fn f_036344_cscaleformmenuhelper_set_data_slot_empty_pm_columns(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036371_cpausemenu_triggerswitchpanewithtabindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036372_cscaleformmgr_changemovieparams_int_rage_vector2_const_ref_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_036373_cscaleformmgr_setmoviedisplayconfig_int_escaleform_base_class_cs(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_036377_cpausemenu_isinmapscreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036378_cpausemenu_issp(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036384_cpausemenu_getofflinereasonaslockey(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_036390_cpausemenu_entersocialclub(ph: *mut Phantom); }
extern "C" { pub fn f_036391_cpausemenu_shoulddrawinstructionalbuttons(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036424_sgeneralpausedataconfig_sgeneralpausedataconfig(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036425_sgeneralpausedataconfig_dtorsgeneralpausedataconfig(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036430_basesocialclubmenu_dtorbasesocialclubmenu(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036457_creportmenu_onacceptinput(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036459_cwarningscreen_render(ph: *mut Phantom); }
extern "C" { pub fn f_036472_cscaleformmenuhelper_show_column_pm_columns_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036473_cscaleformmenuhelper_set_column_scroll_pm_columns_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_036474_cscaleformmenuhelper_set_warning_message_visibility_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036488_sctourmanager_dtorsctourmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036489_socialclubpolicymanager_dtorsocialclubpolicymanager(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036510_cscaleformmgr_addparamstring_char_const_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036522_socialclublegalsmenu_selectcurrentoption(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036523_socialclublegalsmenu_updatepolicyhighlight(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036604_ctexttemplate_openmovie(ph: *mut Phantom); }
extern "C" { pub fn f_036617_ctextinputbox_close(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036631_cwarningscreen_setmessagewithheaderandsubtext_ewarningmessagesty(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32, a9: i32); }
extern "C" { pub fn f_036641_cscaleformmgr_updateallmoviesstatemachines(ph: *mut Phantom); }
extern "C" { pub fn f_036682_cstoremainscreen_processcheckoutselected_rage_ccommerceproductda(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_036727_cvideoprojectplaybackcontroller_jumptostart(ph: *mut Phantom); }
extern "C" { pub fn f_036731_cfileviewfilter_refresh_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036745_cvideoeditorinterface_getactiveproject(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_036746_cvideoprojectplaybackcontroller_getplaybackcurrentnondilatedtime(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_036751_cvideoeditorinterface_killplaybackorbake_bool(ph: *mut Phantom); }
extern "C" { pub fn f_036754_cvideoeditorproject_releasestagingclipthumbnail(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036755_cvideoeditorproject_releaseclipthumbnail_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036766_ctextoverlayhandle_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036768_ctextoverlayhandle_getstarttimems_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036769_ctextoverlayhandle_getdurationms_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036770_ctextoverlayhandle_gettext_unsigned_int_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036771_ctextoverlayhandle_gettextstyle_unsigned_int_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036772_ctextoverlayhandle_gettextposition_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036773_ctextoverlayhandle_gettextscale_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036774_ctextoverlayhandle_gettextcolor_unsigned_int_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036775_cmusiccliphandle_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036777_cmusiccliphandle_getclipsoundhash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036778_cmusiccliphandle_gettrimmeddurationms_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036779_cmusiccliphandle_getstarttimewithoffsetms_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036784_cvideoeditorproject_requeststagingclipthumbnail(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036787_cvideoeditorproject_getanchortimems_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_036788_cvideoeditorproject_getclipdurationtimems_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_036789_cvideoeditorproject_getnearestanchor_float_float_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i32; }
extern "C" { pub fn f_036790_cvideoeditorproject_deleteclip_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036791_cvideoeditorproject_getmusicclipcount_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036792_cvideoeditorproject_requestclipthumbnail_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036793_cvideoeditorproject_getcliptexturename_unsigned_int_rage_atstrin(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_036794_cvideoeditorproject_isclipthumbnailrequested_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036795_cvideoeditorproject_isclipthumbnailloaded_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036796_cvideoeditorproject_hasclipthumbnailloadfailed_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036798_cvideoprojectplaybackcontroller_setcursorspeed_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_036800_cvideoprojectplaybackcontroller_getplaybackcurrenttimems_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_036802_cvideoprojectplaybackcontroller_jumptonondilatedtimems_float_uns(ph: *mut Phantom, a0: f32, a1: i32) -> i32; }
extern "C" { pub fn f_036803_cvideoprojectplaybackcontroller_applyrewind(ph: *mut Phantom); }
extern "C" { pub fn f_036804_cvideoprojectplaybackcontroller_isatendofplayback_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036805_cvideoprojectplaybackcontroller_jumptoend(ph: *mut Phantom); }
extern "C" { pub fn f_036807_cvideoprojectplaybackcontroller_toggleplayorpause(ph: *mut Phantom); }
extern "C" { pub fn f_036827_cthumbnailfiledataprovider_getthumbnailname_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_036857_cfileviewfilter_getfiledatestring_int_char_ref_256_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_036887_cfileviewfilter_ismarkedasdelete_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036889_cfileviewfilter_getdisplayname_int_rage_atstring_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_036896_crawclipfileview_isclipfavourite_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036897_crawclipfileview_getclipfiledurationms_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036911_cthumbnailfileview_getthumbnailname_int_char_ref_256_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_036917_cvideoeditormenu_refreshprojectheading(ph: *mut Phantom); }
extern "C" { pub fn f_036919_cvideoeditormenu_jumptomenu_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036920_cvideoeditormenu_clearpreviewdata_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036922_cvideoeditormenu_buildmenu_int_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_036923_cvideoeditormenu_setitemselected_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036927_cvideoeditormenu_setuserconfirmationscreenwithaction_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_036928_rage_fwtemplatestring_128ul_3ul_sprintf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_036929_cvideoeditorproject_canaddclip_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036931_cvideoeditorplayback_close(ph: *mut Phantom); }
extern "C" { pub fn f_036935_cvideoeditortimeline_rebuildanchors(ph: *mut Phantom); }
extern "C" { pub fn f_036942_ctexttemplate_init(ph: *mut Phantom); }
extern "C" { pub fn f_036943_cvideoeditorui_isinblockingoperation_bool_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036945_cvideoeditormenu_updatepane(ph: *mut Phantom); }
extern "C" { pub fn f_036948_cvideoeditortimeline_clearallclips(ph: *mut Phantom); }
extern "C" { pub fn f_036950_ctexttemplate_updatetemplate_int_seditedtextproperties_rage_atno(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_036953_ctexttemplate_unloadtemplate(ph: *mut Phantom); }
extern "C" { pub fn f_036954_watermarkrenderer_cleanupplayback(ph: *mut Phantom); }
extern "C" { pub fn f_036955_cvideoeditormenu_isonaudiomenu(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036957_cvideoeditormenu_gobackonepane_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036958_cvideoeditormenu_releasethumbnailofitem(ph: *mut Phantom); }
extern "C" { pub fn f_036960_cvideoeditormenu_gotoitem_int_int_cvideoeditormenu_egotorebuildb(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_036961_cvideoeditortimeline_addambient_unsigned_int_cambientcliphandle(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_036962_cvideoeditortimeline_addaudio_unsigned_int_cmusiccliphandle_cons(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_036963_cvideoeditortimeline_addtext_unsigned_int_ctextoverlayhandle_con(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_036964_cvideoeditortimeline_settimeonplayhead_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036965_cvideoeditortimeline_setplayheadtostartofnearestvalidclip(ph: *mut Phantom); }
extern "C" { pub fn f_036967_cvideoeditortimeline_getplayheadpositionms(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036968_cvideoeditortimeline_getprojectindexneareststartpos_eplayhead_ty(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_036969_cvideoeditortimeline_setplayheadpositionms_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036970_cvideoeditortimeline_updateplayheadpreview_eplayhead_type(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036971_cvideoeditortimeline_hideplayheadpreview(ph: *mut Phantom); }
extern "C" { pub fn f_036973_cvideoeditortimeline_canpreviewclipbedroppedatposition_unsigned(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036974_cvideoeditortimeline_gettotaltimelinetimems_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036977_cscaleformcomplexobjectmgr_attachorgetobjectonstage_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036978_cvideoeditortimeline_adjustplayhead_float_bool(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_036979_cvideoeditormenu_getfileviewfilterformenu_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_036985_cvideoeditortimeline_deletetextclipfromtimeline_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036988_ctexttemplate_setuptemplate_seditedtextproperties(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036989_ccomplexobject_attachmovieclip_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_036990_ccomplexobjectarrayitem_setpositioninpixels_rage_vector2_const_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036992_ccomplexobjectarrayitem_setalpha_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036993_ccomplexobject_attachmovieclipinstance_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_036994_ccomplexobjectarrayitem_setxpositioninpixels_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_036995_cvideoeditortimeline_settimelineselected_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036997_rage_atarray_cvideoeditorpreviewdata_0_unsigned_short_destruct_c(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036998_cvideoeditorui_setmoviemode_eme_movie_mode(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037001_cvideoeditorplayback_setmenuvisible_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037002_cvideoeditorplayback_addneweditmarker(ph: *mut Phantom); }
extern "C" { pub fn f_037003_ccomplexobjectarrayitem_gotoframe_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_037004_cvideoeditorplayback_updateaddmarkerbuttonstate(ph: *mut Phantom); }
extern "C" { pub fn f_037006_cvideoeditorplayback_updatespinnerplaybackprogress(ph: *mut Phantom); }
extern "C" { pub fn f_037007_cvideoeditortimeline_getstarttimeofclipms_eplayhead_type_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_037008_cvideoeditormenu_turnoffmenu_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037009_cvideoeditormenu_jumppane(ph: *mut Phantom); }
extern "C" { pub fn f_037010_cvideoeditormenu_updateinstructionalbuttons_cscaleformmoviewrapp(ph: *mut Phantom); }
extern "C" { pub fn f_037011_cvideoeditorplayback_open_eplayback_type_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037012_ctexttemplate_updatetemplatewindow(ph: *mut Phantom); }
extern "C" { pub fn f_037013_cvideoeditorplayback_removecolumn_int(ph: *mut Phantom); }
extern "C" { pub fn f_037014_cvideoeditortimeline_setitemselected_eplayhead_type(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037015_ctexttemplate_defaulttext(ph: *mut Phantom); }
extern "C" { pub fn f_037016_cvideoeditorui_edittextduration_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037017_ctexttemplate_getdefaultpropertydata_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037018_cvideoeditortimeline_getprojectindexbetweentime_eplayhead_type_u(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_037020_cvideoeditorthumbnailmanager_releasethumbnail_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037021_cvideoeditortimeline_deletevideoclipfromtimeline_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037022_cvideoeditortimeline_deleteambientclipfromtimeline_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037023_cvideoeditortimeline_deletemusicclipfromtimeline_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037024_cvideoeditortimeline_clearstageclip_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037025_cvideoeditortimeline_highlightclip_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037026_ccomplexobjectarrayitem_adjustxpositioninpixels_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_037027_cvideoeditortimeline_deleteclip_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037028_cvideoeditortimeline_updateprojectspaceindicator(ph: *mut Phantom); }
extern "C" { pub fn f_037029_cvideoeditortimeline_updateoverviewbar(ph: *mut Phantom); }
extern "C" { pub fn f_037032_cvideoeditortimeline_repositionplayheadifneeded_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037033_cvideoeditormenu_canhighlightitem_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_037034_cvideoeditormenu_gotonextitem(ph: *mut Phantom); }
extern "C" { pub fn f_037035_cvideoeditormenu_isfilefilteringmenu_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037036_cvideoeditormenu_gotopreviouscolumn(ph: *mut Phantom); }
extern "C" { pub fn f_037037_cvideoeditormenu_removecolumn_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037040_cvideoeditormenu_actioninputup(ph: *mut Phantom); }
extern "C" { pub fn f_037041_cvideoeditormenu_actioninputdown(ph: *mut Phantom); }
extern "C" { pub fn f_037042_cvideoeditormenu_canitembeactioned_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_037043_cvideoeditormenu_isonoptionsmenu(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037044_cvideoeditormenu_isonmainmenu(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037047_cvideoeditormenu_isongallerymenu(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037048_cvideoeditortimeline_getcolourforplayheadtype_unsigned_int_eplay(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_037049_ccomplexobjectarrayitem_setcolour_rage_color32_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037051_cvideoeditormenu_setcolumnactive_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037053_ctexttemplate_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_037054_ctexttemplate_closemovie(ph: *mut Phantom); }
extern "C" { pub fn f_037055_cvideoeditortimeline_getendtimeofclipms_eplayhead_type_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_037057_cscaleformmgr_gathermemorystats_int_scaleformmemorystats_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_037058_cvideoeditorplayback_disablecurrentmenu(ph: *mut Phantom); }
extern "C" { pub fn f_037059_ccomplexobject_getwidth(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_037060_cvideoeditorplayback_jumptocurrenteditmarker(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037061_cvideoeditorplayback_refresheditmarkermenu(ph: *mut Phantom); }
extern "C" { pub fn f_037062_cvideoeditorplayback_updateeditmenustate(ph: *mut Phantom); }
extern "C" { pub fn f_037063_cvideoeditorplayback_updatemenuhelptext_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037064_cvideoeditorplayback_setitemselected_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037065_cvideoeditormenu_isitemselectable_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_037066_cvideoeditormenu_executedependentaction_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037067_ccomplexobject_setalpha_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037068_cvideoeditortimeline_findtimelineindexfromhashname_eplayhead_typ(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_037075_cvideoeditormenuitem_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037087_cvideoeditormenuarray_dtorcvideoeditormenuarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037091_ctexttemplate_edittextcolor_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037092_ctexttemplate_edittextfont_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037093_cvideoeditormenu_gotopreviousitem(ph: *mut Phantom); }
extern "C" { pub fn f_037094_cvideoeditormenu_isfilemenu_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037095_cvideoeditormenu_isclipfilteringmenu_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037099_cvideoeditormenu_mousescrolllist_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037102_rage_fwtemplatestring_32ul_3ul_sprintf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_037104_cvideoeditormenu_endaddingcolumndata_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037106_ctexttemplate_ispropertyvalidintemplate_rage_atnonfinalhashstrin(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037107_rage_atarray_cvideoeditorpreviewdata_0_unsigned_short_grow_unsig(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037108_cvideoeditorinstructionalbuttons_addbutton_rage_inputtype_char_c(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_037109_cvideoeditorinstructionalbuttons_addbutton_rage_inputgroup_char(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_037114_ccomplexobject_setxpositioninpixels_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_037115_ccomplexobjectarrayitem_setdepth_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037116_cvideoeditortimeline_setupstagepreviewclip_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037117_ccomplexobjectarrayitem_setscale_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037118_ccomplexobjectarrayitem_setwidth_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_037119_ccomplexobject_setwidth_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_037120_cvideoeditortimeline_findnearestsnappoint_eplayhead_type_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_037121_cscaleformmgr_translatewindowcoordinatetoscaleformmoviecoordinat(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_037124_cvideoeditortimeline_getdurationofclipms_eplayhead_type_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_037128_cvideoeditortimeline_adjusttonearestsnappoint_unsigned_int_int_b(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_037129_cvideoeditortimeline_isplayheadaudioscore_unsigned_int_eplayhead(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_037131_cvideoeditortimeline_getposofobject_int_char_const_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_037132_ccomplexobjectarrayitem_setypositioninpixels_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_037133_rage_atarray_stimelineitem_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037135_cvideoeditortimeline_setupindividualstagepreviewclip_int_unsigne(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_037136_cvideoeditorthumbnailmanager_removetextureoncomplexobject_unsign(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037137_ccomplexobjectarrayitem_setheight_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_037138_cvideoeditortimeline_addtexturetovideoclip_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037139_cvideoeditortimeline_setupstagehelptext(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037140_cvideoeditortimeline_settimelineobjectspinner_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037141_cvideoeditorinstructionalbuttons_addbuttoncombo_rage_inputtype_r(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_037142_cvideoeditorinstructionalbuttons_addbutton_rage_inputtype_rage_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_037157_cvideoeditorplayback_opencurrenteditmarkermenu(ph: *mut Phantom); }
extern "C" { pub fn f_037158_cvideoeditorplayback_handletoplevelbackout(ph: *mut Phantom); }
extern "C" { pub fn f_037162_cvideoeditorplayback_addmarkertoplaybar_char_const_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_037163_cvideoeditorplayback_updaterepeatedinputmarkertime_float(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_037164_cvideoeditorplayback_canskipmarkertobeat_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037165_cvideoeditorplayback_trygetmenuoptionandindex_cvideoeditorplayba(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_037167_cvideoeditorplayback_updateitemtimevalue_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037170_cvideoeditorplayback_getfinescrubbingincrement_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_037171_cvideoeditorplayback_candeletemarkeratcurrenttime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037174_cvideoeditorplayback_getnearesteditmarkerindex_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037175_cvideoeditorplayback_canaddmarkeratcurrenttime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037180_cvideoeditorplayback_seteditmarkerstate_ccomplexobject_ref_cvide(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_037182_cvideoeditorplayback_begincamerashuttereffect_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037183_cvideoeditorplayback_enableeditmarkermenu_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037184_cvideoeditorplayback_enterpostfxmenu_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037185_cvideoeditorplayback_enterdofmenu_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037186_cvideoeditorplayback_enteraudiomenu_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037187_cvideoeditorplayback_entercammenu_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037188_cvideoeditorplayback_setitemactivedisable_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037189_cvideoeditorplayback_getcurrenteditmarker(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037190_cvideoeditorplayback_cleanuptimedilationelement_ccomplexobject_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037191_cvideoeditorplayback_updatefocusedinputeditmenu_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037192_cvideoeditorplayback_refreshmarkerpositionsonplaybar(ph: *mut Phantom); }
extern "C" { pub fn f_037193_cvideoeditorplayback_updateitemtextvalue_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_037194_cvideoeditorplayback_settransitionelementvisible_ccomplexobject(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037195_cvideoeditorplayback_updatecameraoverlay(ph: *mut Phantom); }
extern "C" { pub fn f_037196_cvideoeditorplayback_exitcameditmode(ph: *mut Phantom); }
extern "C" { pub fn f_037197_cvideoeditorplayback_updaterepeatedinputmarkers_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_037198_cvideoeditorplayback_updatemarkerpositiononplaybar_ccomplexobjec(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: f32, a5: i64); }
extern "C" { pub fn f_037199_cvideoeditorplayback_updateinstructionalbuttonsplaybackshortcuts(ph: *mut Phantom); }
extern "C" { pub fn f_037200_cvideoeditorinstructionalbuttons_addbutton_einstructionbuttons_c(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037201_cvideoeditorplayback_beginaddingcolumndata_int_int_char_const_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037202_rage_atarray_cvideoeditorplayback_menuoption_0_unsigned_short_gr(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_037203_cvideoeditorplayback_updateitemfocusdependentstate(ph: *mut Phantom); }
extern "C" { pub fn f_037204_sreplaymarkerinfo_getsfxlngkey_char_ref_16_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037205_ccomplexobjectarrayitem_setrotation_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_037206_cvideoeditorplayback_updateitempercentvalue_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037207_cvideoeditorplayback_updateitemfxvalue_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037208_cvideoeditorplayback_updateitemmetresvaluefloat_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_037209_sreplaymarkerinfo_updatetransitiontype_int_replaymarkertransitio(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_037210_cvideoeditorplayback_updateitemnumbervalue_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037216_ctexttemplateitem_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037225_cpropertyitem_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037231_ctexttemplatexml_dtorctexttemplatexml(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037237_ctexttemplate_fontcanbeedited(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037244_cuireplayscaleformcontroller_deletereplaymovie_unsigned_char(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037248_cscaleformmgr_beginmethod_int_escaleform_base_class_char_const_p(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_037250_rage_atarray_scaleformmoviestruct_0_unsigned_short_destruct_scal(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037257_cscaleformmgr_addparamgfxvalue_gfxvalue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037259_cscaleformmgr_createmovieandwaitforload_char_const_ptr_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_037263_cscaleformmoviewrapper_dtorcscaleformmoviewrapper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037264_cscaleformmgr_destroymovienow_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037265_cscaleformmgr_fixupscaleformallocation_rage_partreenode_ptr_char(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037266_cscaleformmgr_inittextures_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_037267_cscaleformstore_removemovieview_rage_strlocalindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037268_printtxdref_char_const_ptr_rage_strlocalindex_int_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_037269_cscaleformstore_removeref_rage_strlocalindex_rage_strrefkind(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_037270_cscaleformmgr_clearscaleformarrayslot_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037271_cscaleformmgr_loadpreallocationinfo_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037277_rage_atvector_refcountinfo_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037281_ccomplexobjectarrayitem_dtorccomplexobjectarrayitem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037283_cscaleformmgr_ismovieshuttingdown_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037294_cscaleformstore_loadfilecore_rage_strlocalindex_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_037295_cscaleformstore_createmovieview_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037297_cscaleformmgr_doesmovieexistinimage_char_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037299_cscaleformmgr_debugturnonoffoutput_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037300_cscaleformmgr_getmovieview_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_037301_cscaleformmgr_getmappedfontname_char_const_ptr_gstring_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_037304_cscaleformmgr_doesmoviehaveinvokesorcallbackspending_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037305_cscaleformmgr_restartmovie_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_037308_cscaleformmgr_executecallbackmethod_int_char_const_ptr_gfxvalue(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_037311_gfxvalue_setnumber_double(ph: *mut Phantom, a0: i64, a1: f64); }
extern "C" { pub fn f_037313_gfxvalue_setboolean_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037317_cscaleformmgr_callmethod_int_escaleform_base_class_char_const_pt(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_037319_cscaleformmgr_addtxdref_int_char_const_ptr_char_const_ptr_bool_b(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_037322_scaleformmoviestruct_createnewtxdrequest_rage_strlocalindex_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_037323_cscaleformmgr_calladdtxdrefresponse_int_char_const_ptr_char_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_037341_cscaleformmgr_passdebugmethodparams_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037344_rage_fwassetstore_cscaleformmovieobject_cscaleformdef_dtorfwasse(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037354_cscaleformstore_load_rage_strlocalindex_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_037356_cscaleformmovieobject_dtorcscaleformmovieobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037359_ccomplexobjectarrayitem_setmemberinternal_char_const_ptr_emember(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_037360_ccomplexobjectarrayitem_getstagerootinternal_ccomplexobjectarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037361_ccomplexobjectarrayitem_getobjectinternal_ccomplexobjectarrayite(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_037362_ccomplexobjectarrayitem_createemptymovieclipinternal_ccomplexobj(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_037363_ccomplexobjectarrayitem_attachmovieclipinternal_ccomplexobjectar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_037364_ccomplexobjectarrayitem_initproperties(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037386_metricbaseinfo_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_037387_metricbasechar_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_037390_gametransactionsessionmgr_dtorgametransactionsessionmgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037398_rage_sfscaleformmanager_dtorsfscaleformmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037405_cscaleformmgr_rendermovie_int_float_bool_bool_float_debugdrawmem(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_037407_ccontextmenu_dtorccontextmenu(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037432_cfrontendstatsmgr_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037433_cfrontendstatsmgr_loaddataxmlfile_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037434_rage_atmapentry_unsigned_int_rage_atarray_suireplacementtag_0_un(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037436_cfrontendstatsmgr_getvehiclenamefromhashkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_037447_cgallerymenu_initialise(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037448_cgallerymenu_updateafterselectingathumbnail_efrontend_input_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_037449_cgallerymenu_setscrollbarcount(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037450_cgallerymenu_setmenucontext_cgallerymenu_econtextstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037451_cgallerymenu_updatewaypointcontextbutton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037452_cgallerymenu_updateuploadcontextbutton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037453_cgallerymenu_setscrollbararrows_cgallerymenu_econtextstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037454_cgallerymenu_setdescription_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037455_cgallerymenu_stepintogallerypage(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037456_cgallerymenu_stepoutofgallerypage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037457_cgallerymenu_clearphotoslots(ph: *mut Phantom); }
extern "C" { pub fn f_037458_cgallerymenu_maximizetheselectedthumbnail(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037460_cgallerymenu_shouldnavigatetonewpage_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037461_cgallerymenu_repopulate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037462_cgallerymenu_beginuploadoflocalphoto(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037463_cgallerymenu_cleanupmemeeditor(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037464_cgallerymenu_setmaximize_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037467_cgallerymenu_cancelprofanitycheck(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037468_cgallerymenu_returntothumbnailview(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037469_cgallerymenu_entermemetextentry_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037470_cgallerymenu_checkforgalleryproblems_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037471_cgallerymenu_cleanupmemeeditoraudio(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037472_cgallerymenu_setmaximize_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_037477_cgallerymenu_seterrorpages_bool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037478_cgallerymenu_displaywarningscreen_cgallerymenu_egalleryactionsta(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037484_cgallerymenu_displayphotoinslot_unsigned_int_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_037485_cgamestream_gstpost_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037486_cgamestream_gstpost_releaseandcanceltexture(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037487_cgamestream_postgametip_int_cgamestream_egamestreamtiptype(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_037488_cgamestream_gstpost_addparam_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037489_cgamestream_shouldrender_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037491_cgamestream_forcerenderoff(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037498_cgamestream_autopostgametipoff(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037506_cgamestream_hasalwaysrenderfeeditem(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037507_cgamestream_postmessagetext_char_const_ptr_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64, a8: f32, a9: i64, a10: i32, a11: i32, a12: i32) -> i32; }
extern "C" { pub fn f_037508_cgamestream_allocpost_gstposttype_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_037509_char_ptr_rage_formatf_char_3ul_char_ref_3ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_037510_cgamestream_poststats_char_const_ptr_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_037511_cgamestream_postticker_char_const_ptr_bool_bool_bool_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_037512_cgamestream_gstpost_addstringparamandparsegamernamespaces_char_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037513_cgamestream_postaward_char_const_ptr_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_037514_cgamestream_postcrewtag_bool_bool_char_const_ptr_int_bool_char_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i64, a8: i64, a9: i32) -> i32; }
extern "C" { pub fn f_037515_cgamestream_postcrewrankup_char_const_ptr_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_037516_cgamestream_postversus_char_const_ptr_char_const_ptr_int_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_037517_cgamestream_postreplay_cgamestream_efeedreplaystate_char_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: f32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_037518_cgamestream_updatefeeditemtexture_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_037519_cgamestream_postunlock_char_const_ptr_char_const_ptr_int_bool_eh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_037520_cgamestream_addinnerscaleformparams_cgamestream_gstpost_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037543_cgamestreammgr_readsetup_char_const_ptr_cgamestreammgr_movsetup(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_037598_cappdatamgr_fileupdated_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037601_cappdatamgr_deleteclouddata_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_037602_rage_atarray_spollrequest_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037605_cappdatamgr_writeclouddatatofile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037606_cappdatamgr_getappdata_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037607_cappdata_getformatteddata_char_ptr_unsigned_int_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_037611_cappdata_getformatteddata_cappdatablock_ptr_rage_atstring_char_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_037613_rage_atmapentry_rage_atstring_cappdatablock_ref_rage_atmap_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_037614_rage_atmap_rage_atstring_cappdatablock_rage_atmaphashfn_rage_ats(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037615_cappdatablock_update_rage_rsonreader_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037616_rage_atstring_ref_rage_atmap_rage_atstring_rage_atstring_rage_at(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_037618_rage_atmap_rage_atstring_rage_atstring_rage_atmaphashfn_rage_ats(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037619_rage_atmap_rage_atstring_cappdatablock_rage_atmaphashfn_rage_ats(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037620_rage_atmap_rage_atstring_cappdatablock_rage_atmaphashfn_rage_ats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037625_rage_atmapentry_rage_atstring_cappdata_ref_rage_atmap_rage_atstr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037634_ccheat_weaponcheat1(ph: *mut Phantom); }
extern "C" { pub fn f_037635_ccheat_healthcheat(ph: *mut Phantom); }
extern "C" { pub fn f_037636_ccheat_wantedlevelupcheat(ph: *mut Phantom); }
extern "C" { pub fn f_037637_cwanted_cheatwantedlevel_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037638_ccheat_wantedleveldowncheat(ph: *mut Phantom); }
extern "C" { pub fn f_037640_cweather_incrementtype(ph: *mut Phantom); }
extern "C" { pub fn f_037642_ccheat_vehiclecheat_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037655_setweaponcheatloadout(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037656_cwanted_setmaximumwantedlevel_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037657_cwanted_setwantedlevel_rage_vector3_const_ref_int_int_setwantedl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_037658_cwanted_updatewantedlevel_rage_vector3_const_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_037665_time_add_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_037666_time_addminute_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037668_date_addday_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037669_datetime_addtime_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_037670_datetime_getsecondssince1900_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037673_cclock_loadtimedata_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037679_cclock_addtime_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_037723_cconfignetscriptbroadcastdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037729_cconfigscriptstacksizes_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037733_cconfigscripttextlines_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037737_cconfigscriptresourceexpectedmaximums_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037752_cgameconfig_cgameconfig(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037753_rage_atarray_cscriptstacksizedataentry_0_unsigned_short_grow_uns(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037761_ccrime_reportcrime_ecrimetype_centity_ptr_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_037762_cwanted_evaluatecrime_ecrimetype_bool_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_037763_cwanted_addcrimetoq_ecrimetype_centity_const_ptr_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_037764_cwanted_workoutpolicepresence_rage_vec3v_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_037765_cwanted_registercrime_immediately_ecrimetype_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_037766_cwanted_setonestarparolenodrop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037767_cwanted_setwantedlevelnodrop_rage_vector3_const_ref_int_int_setw(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_037768_cwanted_reportcrimenow_ecrimetype_rage_vector3_const_ref_bool_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32); }
extern "C" { pub fn f_037775_ccrimeinformation_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037794_popmultiplierarea_op_popmultiplierarea_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_037802_cthepopmultiplierareas_getpeddensitymultiplier_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_037811_cplacename_display(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037822_cwanted_tunables_wantedlevel_difficulty_calculation_parser_regis(ph: *mut Phantom); }
extern "C" { pub fn f_037827_cwanted_tunables_wantedlevel_difficulty_helis_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037829_cwanted_tunables_wantedlevel_difficulty_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037832_cwanted_tunables_wantedlevel_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037836_cwanted_tunables_difficulty_spawning_scoring_weights_parser_regi(ph: *mut Phantom); }
extern "C" { pub fn f_037838_cwanted_tunables_difficulty_spawning_scoring_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037840_cwanted_tunables_difficulty_spawning_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037842_cwanted_tunables_difficulty_despawning_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037844_cwanted_tunables_difficulty_peds_attributes_situations_situation(ph: *mut Phantom); }
extern "C" { pub fn f_037846_cwanted_tunables_difficulty_peds_attributes_situations_parser_re(ph: *mut Phantom); }
extern "C" { pub fn f_037849_cwanted_tunables_difficulty_peds_attributes_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037852_cwanted_tunables_difficulty_peds_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037855_cwanted_tunables_difficulty_dispatch_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037857_cwanted_tunables_difficulty_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037879_cwanted_getcrimeinfo_ecrimetype_cwanted_tunables_crimeinfo_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_037881_cwanted_clearqdcrimes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037883_cwanted_passengerscommentonpolice(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037884_cwanted_reportpolicespottingplayer_cped_ptr_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_037886_cwanted_setwantedlevelforvehiclepassengers_cvehicle_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_037888_cseatmanager_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_037890_cincident_cincident_centity_ptr_rage_vector3_const_ref_cincident(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32) -> i64; }
extern "C" { pub fn f_037891_cincidentmanager_addincident_cincident_const_ref_bool_int_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_037892_cincident_dtorcincident(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037893_creportingwitness_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037894_cwantedblips_createandupdateradarblips_cwanted_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037897_cwantedblips_getblipforped_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037898_cwantedblips_ispedattackingplayer_cped_ptr_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_037899_rage_atarray_cwantedblips_copblipliststruct_0_unsigned_short_gro(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037900_cdispatchdata_gethiddenevasiontime_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037901_cwanted_copsaresearching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037903_corder_removeorderfromentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037904_cwanted_reportcrimenowforvehiclepassengers_cvehicle_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_037911_creportingwitness_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037919_cweathergpufxfromxmlfile_dtorcweathergpufxfromxmlfile(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037932_rage_atarray_cweathertypefromxmlfile_0_unsigned_short_destruct_c(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037936_cweather_updatesun(ph: *mut Phantom); }
extern "C" { pub fn f_037937_cweather_updaterain(ph: *mut Phantom); }
extern "C" { pub fn f_037938_cweather_updatesnow(ph: *mut Phantom); }
extern "C" { pub fn f_037939_cweather_updatesnowmist(ph: *mut Phantom); }
extern "C" { pub fn f_037940_cweather_updatewind(ph: *mut Phantom); }
extern "C" { pub fn f_037941_cweather_updatesandstorm(ph: *mut Phantom); }
extern "C" { pub fn f_037942_cweather_updatewatersettings(ph: *mut Phantom); }
extern "C" { pub fn f_037943_cwind_update_float_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_037944_cweather_stopoverriding(ph: *mut Phantom); }
extern "C" { pub fn f_037945_cweather_forcetypenow_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037974_scanpedwitnesscrimes_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037975_ccrimewitnesses_checklineofsightandprocessresult_cped_ref_cped_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_037977_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_anonymo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_037993_cmapzone_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_037998_rage_atarray_cmapzonearea_0_unsigned_short_destruct_cmapzonearea(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038004_cmapzonemanager_loadmapzones(ph: *mut Phantom); }
extern "C" { pub fn f_038006_cmapzonemanager_updatezonecombobox(ph: *mut Phantom); }
extern "C" { pub fn f_038008_cconvexhull2d_buildconvexhull_rage_atarray_rage_vector3_0_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_038009_cmapzonemanager_updatepointpositiontextbox(ph: *mut Phantom); }
extern "C" { pub fn f_038010_cmapzonemanager_getzoneatcoords_rage_vector3_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038011_rage_atarray_cmapzonearea_0_unsigned_short_copyfrom_rage_atarray(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038012_cmapzonemanager_rebuildzonenamesarray(ph: *mut Phantom); }
extern "C" { pub fn f_038023_rage_atarray_cconvexhull2d_thullvert_0_unsigned_short_op_rage_at(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038029_cvehicleset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_038032_rage_atarray_cconditionalvehicleset_0_unsigned_short_destruct_cc(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038037_sdispatchmodelvariations_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_038041_cwantedresponse_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_038049_cdispatchdata_dtorcdispatchdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038052_cwantedresponse_calculatenumpedstospawn_dispatchtype(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038057_cdispatchservice_resetspawntimer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038090_cdispatchhelpervolumes_tunables_angledarea_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_038092_cdispatchhelpervolumes_tunables_angledareawithposition_parser_re(ph: *mut Phantom); }
extern "C" { pub fn f_038094_cdispatchhelpervolumes_tunables_sphere_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_038096_cdispatchhelpervolumes_tunables_spherewithposition_parser_regist(ph: *mut Phantom); }
extern "C" { pub fn f_038098_cdispatchhelpervolumes_tunables_locationfornearestcarnodeoverrid(ph: *mut Phantom); }
extern "C" { pub fn f_038101_cdispatchhelpervolumes_tunables_enclosedsearchangledarea_parser(ph: *mut Phantom); }
extern "C" { pub fn f_038105_cdispatchhelpervolumes_tunables_enclosedsearchsphere_parser_regi(ph: *mut Phantom); }
extern "C" { pub fn f_038109_cdispatchhelpervolumes_tunables_enclosedsearchregions_parser_reg(ph: *mut Phantom); }
extern "C" { pub fn f_038112_cdispatchhelpervolumes_tunables_blockingareas_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_038118_cdispatchspawnproperties_cdispatchspawnproperties(ph: *mut Phantom); }
extern "C" { pub fn f_038126_cdispatchspawnhelper_dtorcdispatchspawnhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038127_cdispatchspawnhelper_canspawnatposition_rage_vec3v_const_ref_con(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038128_cdispatchspawnhelper_getidealspawndistance_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038129_cdispatchspawnhelper_getlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038130_cmodelinfo_getbasemodelinfo_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038131_cdispatchspawnhelper_spawnvehicle_rage_fwmodelid_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_038132_cdispatchspawnhelper_spawnvehicleinfront_rage_fwmodelid_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_038133_cdispatchspawnhelper_getdirection_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038134_rage_vector3_normalizesaferet(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038136_cdispatchspawnhelper_findspawnpointindirection_rage_cnodeaddress(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_038138_cdispatchspawnhelper_renderdebug(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038140_cdispatchspawnhelper_ondeactivate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038143_cdispatchspawnhelper_updatefindnearestcarnodehelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038149_cdispatchadvancedspawnhelper_dtorcdispatchadvancedspawnhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038152_cdispatchadvancedspawnhelper_cachedebug(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038161_cdispatchhelpersearch_calculate_cped_const_ref_cped_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_038178_cdispatchhelpervolumes_tunables_dtortunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038193_cinjuryincident_ispedvalidforinjuryincident_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_038194_cinjuryincident_dtorcinjuryincident(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038196_cincidentmanager_gettextposition_int_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038208_cdispatchservice_updatecurrentsearchareas_float_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038209_cdispatchservice_computespeedmodifier_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038211_cdispatchservice_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038215_cordermanager_addorder_corder_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038216_corder_dtorcorder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038218_cdispatchservice_setlastspawnfailurereason_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038221_cdispatchservice_getisentityvalidfororder_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038224_cdispatchservice_getandverifymodelids_rage_fwmodelid_ref_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_038225_cdispatchservice_requestallmodels_cdispatchresponse_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038226_cdispatchservice_findfreeresourcesfrompedsinvehicle_cvehicle_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_038229_cdispatchservicewanted_streammodels(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038230_cdispatchservicewanted_setuppedvariation_cped_ref_ewantedlevel(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038235_cdispatchservice_dtorcdispatchservice_65542(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038243_cpoliceautomobiledispatch_isordervalidforped_cpoliceorder_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038247_cpoliceautomobiledispatch_doesanyonehaveorder_int_cincident_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_038253_cwantedhelicopterdispatch_dispatchdelayedspawncallback_centity_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_038265_cincidentmanager_clearincident_cincident_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038269_cswatorder_cswatorder_dispatchtype_centity_ptr_cincident_ptr_csw(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_038293_cpoliceroadblockdispatch_generatetype_rage_atarray_rage_fwmodeli(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038294_croadblock_createfromtype_croadblock_createfromtypeinput_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_038295_cpoliceroadblockdispatch_getandverifymodelids_rage_atarray_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038296_croadblock_croadblock_cped_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_038297_croadblock_trytodespawn(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038304_createroadblock_croadblock_roadblocktype(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_038305_rage_atarray_rage_fwmodelid_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038316_cpoliceorder_cpoliceorder_dispatchtype_centity_ptr_cincident_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_038343_cfireincident_dtorcfireincident(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038345_cfireincident_getbestfire_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_038346_cfireincident_shouldclusterwith_cfire_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038348_cfireincident_updatefires_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_038362_cincidentmanager_cincidentptr_createincident_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_038370_cordermanager_corderptr_createorder_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_038373_corder_setentityfinishedwithincident(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038375_corder_assignordertoentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038382_cpedintelligence_addtaskdefault_rage_aitask_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038393_croadblock_dtorcroadblock(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038394_croadblock_disperse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038398_croadblock_calculatedimensionsformodel_rage_fwmodelid_float_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_038399_croadblock_createobj_croadblock_createobjinput_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_038400_croadblock_createtransform_croadblock_createtransforminput_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038401_croadblock_createped_croadblock_createpedinput_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_038402_croadblock_createveh_croadblock_createvehinput_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_038403_croadblock_update_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_038410_bgscriptinfo_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038411_rage_atmapentry_unsigned_int_rage_atfixedstring_64ul_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_038422_backgroundscriptsdebug_dtorbackgroundscriptsdebug(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038424_backgroundscripts_dtorbackgroundscripts_65831(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038426_backgroundscripts_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038428_backgroundscripts_loaddata_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_038429_backgroundscripts_startcontext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038431_backgroundscripts_requestcloudfile(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038433_backgroundscripts_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038437_backgroundscriptsdebug_addcontext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038438_rage_atmapentry_unsigned_int_context_const_ref_rage_atmap_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_038444_backgroundscripts_endcontext_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038447_backgroundscriptsdebug_printscriptcounts(ph: *mut Phantom); }
extern "C" { pub fn f_038463_cbaseikmanager_removeallsolvers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038464_cbaseikmanager_refreshkinematics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038468_cbaseikmanager_addsolver_ikmanagersolvertypes_flags(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038469_crootslopefixupiksolver_crootslopefixupiksolver_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_038470_ctorsoiksolver_ctorsoiksolver_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_038471_ctorsoreactiksolver_ctorsoreactiksolver_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_038472_cbodylookiksolverproxy_cbodylookiksolverproxy_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_038473_clegiksolverproxy_clegiksolverproxy(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038474_carmiksolver_carmiksolver_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_038476_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_cbaseik(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_038477_cbaseikmanager_removecompletedsolvers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038478_cikmanager_dtorcikmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038482_crootslopefixupiksolver_solveroot_rage_crskeleton_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038483_clegiksolver_update_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_038484_carmiksolver_canupdate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038485_carmiksolver_getattachparent_rage_mat34v_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_038486_carmiksolver_storehelpertransform_rage_criksolverarms_earms_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_038487_carmiksolver_calculatetargetposition_rage_crskeleton_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_038489_cbodylookiksolverproxy_updateactivestate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038493_cikmanager_issupported_ikmanagersolvertypes_flags_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038496_cbodylookiksolverproxy_request_cikrequestbodylook_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038507_cikmanager_setrelativearmiktarget_rage_criksolverarms_earms_cdyn(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32, a6: f32, a7: f32, a8: f32, a9: f32); }
extern "C" { pub fn f_038525_cikmanager_canuselegik_bool_ref_bool_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_038533_crootslopefixupiksolver_requestprobe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038550_cbodylookiksolver_interpolateyawpitch_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_038551_cbodylookiksolver_calculaterelativeanglerotation_cbodylookiksolv(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_038552_cbodylookiksolver_setdefaultyawpitch_cbodylookiksolver_component(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_038553_cbodylookiksolverproxy_dtorcbodylookiksolverproxy(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038563_clegiksolver_tunables_interpolationsettings_incrementtowards_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: f32) -> f32; }
extern "C" { pub fn f_038564_clegiksolver_shapetestdata_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038565_clegiksolver_shapetestdata_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038568_clegiksolver_partialreset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038569_clegiksolver_workoutlegsituationusingpedcapsule_rage_criksolverl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32); }
extern "C" { pub fn f_038570_clegiksolver_workoutlegsituationusingshapetests_rage_criksolverl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32); }
extern "C" { pub fn f_038571_clegiksolver_applysupportingfootmode_clegiksolver_esupportingfoo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_038572_clegiksolver_setleg_rage_criksolverlegs_goal_ref_clegiksolver_le(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32); }
extern "C" { pub fn f_038575_clegiksolver_calcsmoothednormal_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: f32, a5: i32); }
extern "C" { pub fn f_038576_clegiksolver_processresults_clegiksolver_leggoal_ptr_clegiksolve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32); }
extern "C" { pub fn f_038577_clegiksolver_clampnormal_rage_vector3_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038581_crootslopefixupiksolver_dtorcrootslopefixupiksolver(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038585_crootslopefixupiksolver_calculatepitchandrollfromgroundnormal_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_038594_ctorsoreactiksolver_clampangles_rage_matrix34_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_038604_ctorsovehicleiksolver_solvespine_rage_crskeleton_ref_ctorsovehic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_038611_cbasemodelinfo_initarchetypefromdefinition_rage_strlocalindex_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_038616_cbasemodelinfo_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038617_cbasemodelinfo_initmasterdrawabledata_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038620_cbasemodelinfo_setphysics_rage_pharchetypedamp_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038621_cbasemodelinfo_computemass_rage_pharchetypedamp_ptr_ctunableobje(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_038622_cbasemodelinfo_computedamping_rage_pharchetypedamp_ptr_ctunableo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_038623_cbasemodelinfo_copymaterialflagstosinglebound_rage_phbound_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038627_cbasemodelinfo_initmasterfragdata_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038629_cbasemodelinfo_getaudiocollisionsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038635_cbasemodelinfo_checkforhdtxdslots(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038637_cbasemodelinfo_initwatersamples(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038638_cbasemodelinfo_getlargestboundfromcomposite_rage_phboundcomposit(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038639_cbasemodelinfo_generatewatersamplesfromboundr_cwatersample_ptr_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_038640_cbasemodelinfo_createphysicsarchetype(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038641_cbasemodelinfo_addreftodrawablephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038642_cmlomodelinfo_dtorcmlomodelinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038652_cmlomodelinfo_getportaldata_unsigned_int_unsigned_int_ref_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_038699_cmodelinfo_getmodelinfofromlocalindex_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_038701_cpedmodelinfo_setpedmetadatafile_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038702_cmodelinfostreamingmodule_getclipsetdependencies_rage_fwmvtempla(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_038715_cmodelinfo_requestassets_cbasemodelinfo_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038721_cmodelinfo_getobjectanddependencies_rage_fwmodelid_rage_atarray(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_038723_cmodelinfo_postload_rage_fwarchetype_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038725_cvehiclemodelinfo_calculatemaxflatvelocityestimate_chandlingdata(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038726_cvehiclemodelinfo_dumpvehiclemodelinfos(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038728_calculateaverageandmediansizes_rage_atarray_sdebugsize_0_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i64); }
extern "C" { pub fn f_038730_cvehiclemodelinfo_calculatevehiclecoverpoints(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038733_centryexitpoint_getseat_seataccess_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_038735_cmodelseatinfo_calculateentrysituation_centity_const_ptr_cped_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32, a7: i64); }
extern "C" { pub fn f_038736_cmodelseatinfo_adjustpositionforgroundheight_rage_vector3_ref_cv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32) -> i32; }
extern "C" { pub fn f_038738_cmodelseatinfo_initfromlayoutinfo_cvehiclelayoutinfo_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_038739_cmodelseatinfo_getentrypointindexforseat_int_centity_const_ptr_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_038744_cmodelseatinfo_calculateseatsituation_centity_const_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_038746_cmodelseatinfo_calculatethroughwindowposition_cvehicle_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_038748_rage_atarray_centrypointearlyrejectionevent_0_unsigned_short_gro(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_038749_cmodelseatinfo_evaluateseataccessfromentrypoint_cped_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i64, a9: i64) -> i32; }
extern "C" { pub fn f_038750_rage_atarray_centrypointscoringevent_0_unsigned_short_grow_unsig(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_038752_cridableentrypointevaluationdebuginfo_addpriorityscoringevent_ce(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038753_rage_atarray_centrypointpriorityscoringevent_0_unsigned_short_gr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038754_cmodelseatinfo_ispositionclearforped_cped_const_ptr_centity_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_038760_cpedmodelinfo_initdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_038770_cpedmodelinfo_initdatalist_dtorinitdatalist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038771_cpedmodelinfo_personalitymovementmodes_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_038776_cpedmodelinfo_personalitymovementmodes_movementmode_parser_regis(ph: *mut Phantom); }
extern "C" { pub fn f_038777_rage_atarray_cpedmodelinfo_personalitymovementmodes_movementmode(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038784_cpedmodelinfo_personalitymovementmodes_sunholsterclipdata_parser(ph: *mut Phantom); }
extern "C" { pub fn f_038790_cpedmodelinfo_personalitybravery_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_038793_cpedmodelinfo_personalitythreatresponse_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_038797_cpedmodelinfo_personalitythreatresponse_action_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_038798_cpedmodelinfo_personalitythreatresponse_fight_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_038812_cpedmodelinfo_personalitydata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_038822_cpedmodelinfo_personalitydatalist_dtorpersonalitydatalist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038840_cmodelseatinfo_cmodelseatinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038842_cpedmodelinfo_initpeddata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038845_cpedmodelinfo_initphys(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038852_rage_atarray_cpedmodelinfo_personalitymovementmodes_movementmode(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038853_rage_atarray_cpedmodelinfo_personalitybravery_0_unsigned_short_g(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_038854_rage_atarray_cpedmodelinfo_personalitydata_0_unsigned_short_grow(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_038855_cpedmodelinfo_personalitydata_op_cpedmodelinfo_personalitydata_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038856_rage_atarray_cpedmodelinfo_personalitymovementmodes_sunholstercl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038857_rage_atarray_cpedmodelinfo_personalitymovementmodes_movementmode(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038858_cpedmodelinfo_loadpedmetafile_char_const_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_038862_cpedmodelinfo_setquadrupedprobetopposition_int_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_038863_cpedmodelinfo_getpersonalitybravery_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_038866_cpedmodelinfo_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038893_cvehiclemodelinfo_initdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_038898_cvehiclemodelinfo_initdata_dtorinitdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038904_cvehiclemodelinfo_initdatalist_dtorinitdatalist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038910_cvehiclemodelinfo_loadvehiclecolours(ph: *mut Phantom); }
extern "C" { pub fn f_038911_cvehiclemodelinfo_loadresidentobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_038913_cvehiclemodelinfovarglobal_addcarcolfile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038914_rage_atarray_sirensettings_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038915_cvehiclemodelinfo_updatemodelcolors_cvehiclevariationdata_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038916_cvehiclemodelinfo_updatemodkits_cvehiclevariationdata_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038917_rage_atarray_cvehiclevariationdata_0_unsigned_short_grow_unsigne(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038918_cvehiclevariationdata_op_cvehiclevariationdata_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038919_rage_atarray_cvehiclemodelcolorgen9_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038920_cvehiclemodelinfovarglobal_updatekitids_int_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_038921_cvehiclemodelinfovarglobal_updatelightids_int_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_038922_cvehiclemodelinfovarglobal_updatesirenids_int_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_038923_sirensettings_sirensettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038924_cvehiclemodelinfo_loadvehiclemetafile_char_const_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_038925_cvehiclemodelinfo_setuphdsharedtxd_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038927_cvehiclemodelinfo_appendvehiclecolors_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038928_cvehiclemodelinfovarglobal_cvehiclemodelinfovarglobal(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038929_rage_atarray_cvehiclekit_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038930_rage_atarray_cvehiclemodvisible_0_unsigned_short_op_rage_atarray(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038931_rage_atarray_cvehiclemodlink_0_unsigned_short_op_rage_atarray_cv(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038932_rage_atarray_cvehiclemodstat_0_unsigned_short_op_rage_atarray_cv(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038933_rage_atarray_cvehiclekit_sslotnameoverride_0_unsigned_short_op_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038934_rage_atarray_rage_conststring_0_unsigned_short_op_rage_atarray_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038935_rage_atarray_vehiclelightsettings_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038936_cvehiclemodelinfovarglobal_dtorcvehiclemodelinfovarglobal(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038937_rage_atnonfinalhashstring_ref_rage_atmap_unsigned_char_rage_atno(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_038938_cvehiclemodelcolorgen9_op_cvehiclemodelcolorgen9_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038940_cvehiclemodelinfoplates_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038946_cvehiclemodelinfovarglobal_refreshcolornames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038947_cvehiclemodelinfovarglobal_refreshwindowcolornames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038948_cvehiclemodelinfovarglobal_refreshlightnames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038949_cvehiclemodelinfovarglobal_refreshsirennames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038967_cvehiclemodelinfo_savevehiclecolours_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038970_cvehiclemodelinfovariation_deletewidgetnames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038976_cvehiclevariationinstance_setmodindex_evehiclemodtype_unsigned_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_038977_cvehiclevariationinstance_setmodcollision_cvehicle_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_038978_cvehiclevariationinstance_getmodindexfortype_evehiclemodtype_cve(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_038979_cvehiclevariationinstance_requestvehiclemodfiles_cvehicle_ptr_in(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038980_cvehiclevariationinstance_updatebonepositions_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038983_cvehiclemodelinfovarglobal_refreshvehiclewidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038984_cvehiclemodelinfovariation_refreshvehiclewidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038988_cvehiclemodelinfo_refreshallvehiclelightsettings(ph: *mut Phantom); }
extern "C" { pub fn f_038989_cvehiclekit_dtorcvehiclekit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038994_cvehiclemodelinfo_initphys(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038995_cvehiclemodelinfo_setdecalboneflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038996_cvehiclemodelinfo_setboneindexes_objectnameidassociation_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_038997_cvehiclemodelinfo_calculateheightsaboveroad_float_ptr_float_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_039002_cvehiclemodelinfo_setbonesgroupmassandangularinertia_int_float_r(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_039003_cvehiclemodelinfo_setgroupmass_int_float_rage_atfixedbitset_256(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64); }
extern "C" { pub fn f_039005_cvehiclemodelinfo_generatewatersamplesforvehicle(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039006_cvehiclemodelinfo_getvehiclebboxforbuoyancy_rage_vector3_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_039008_cvehiclemodelinfo_generatepontoonsamples_cseaplanehandlingdata_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_039033_cvehiclemodvisible_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_039052_cvehiclekit_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_039086_cvehiclemodelinfoplates_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_039087_vehiclelightsettings_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_039088_sirensettings_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_039091_cvehiclemodpearlescentcolors_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_039111_cvehiclemodelinfovarglobal_widgetchangedcoloreditindex(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039112_cvehiclemodelinfovarglobal_widgetchangedwindowcoloreditindex(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039113_cvehiclemodelinfovarglobal_widgetchangedlighteditindex(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039114_cvehiclemodelinfovarglobal_widgetchangedsireneditindex(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039116_vehiclelight_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039117_vehiclecorona_addwidgets_rage_bkbank_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_039118_vehiclelightsettings_vehiclelightsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039140_cvehiclemodelplateprobabilities_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_039144_cvehiclemodelinfoplates_reloadwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039145_cvehiclemodelplateprobabilities_addwidgets_rage_bkbank_ref_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_039148_cvehiclemodelplateprobabilities_reloadwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039165_sirenrotation_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_039170_sirenlight_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_039182_sirenrotation_addwidget_rage_bkbank_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_039192_cvehiclevariationdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_039204_cvehiclestreamrequestgfx_haveallloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039205_cvehiclestreamrequestgfx_requestscomplete(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039210_cvehiclestreamrequestgfx_addfrag_unsigned_char_rage_atnonfinalha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_039215_cvehiclevariationinstance_getglobalmodindex_evehiclemodtype_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_039236_cvehiclemodelinfovariation_dtorcvehiclemodelinfovariation(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039238_cvehiclevariationdata_addcolor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039242_cvehiclevariationdata_widgetremovevehiclecolor_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039243_cvehiclevariationdata_widgetchangedvehiclecolor_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039244_cvehiclestreamrequestgfx_dtorcvehiclestreamrequestgfx(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039246_cvehiclestreamrendergfx_getfragindex_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039247_cvehiclestreamrendergfx_storemodmatrices_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039250_cvehiclestreamrendergfx_dtorcvehiclestreamrendergfx(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039257_cweaponmodelinfo_initdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_039278_basemodelinfoboneindices_basemodelinfoboneindices_rage_crskeleto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_039279_basemodelinfoboneindices_calculatebonecount(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039280_cgameconfig_dtorcgameconfig(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039289_cpedmodelinfo_dtorcpedmodelinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039293_cvehiclemodelinfo_dtorcvehiclemodelinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039322_rage_atfixedarray_rage_strrequest_12_dtoratfixedarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039347_carmiksolver_dtorcarmiksolver(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039389_cvehiclevariationdata_dtorcvehiclevariationdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039534_rage_atarray_cpedmodelinfo_personalitymovementmodes_movementmode(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039535_cpedmodelinfo_personalitymovementmodes_movementmode_clipsets_op(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039538_ccovertuningfile_namedtuning_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_039546_ccovertuningmanager_load_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039547_rage_atmapentry_rage_atnonfinalhashstring_int_const_ref_rage_atm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_039548_ccovertuningmanager_gettuningformodel_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039551_rage_atarray_ccovertuningfile_modeltotunename_0_unsigned_short_g(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039552_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_039553_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_039559_cobject_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039560_cdoor_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039561_cobject_cobject_eentityownedby_unsigned_int_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_039562_cdoor_cdoor_eentityownedby_cdummyobject_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_039563_cobject_cobject_eentityownedby_cdummyobject_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_039564_cdoor_dtorcdoor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039565_cobject_dtorcobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039566_cobject_clearrelateddummy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039568_cdoortuningmanager_gettuningfordoormodel_unsigned_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_039571_cdoor_enforcelockedobjectstatus_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039572_cdoor_settargetdoorratio_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_039573_cpathservergta_updatedynamicobject_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_039574_cpathservergta_shouldavoidobject_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039575_cpathservergta_adddynamicobject_centity_ptr_cbasemodelinfo_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039576_cdoor_getdooropenratio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_039577_cdoor_getlimitangle_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_039578_cdoor_isdoorshut_float_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_039579_cdoor_processportaldisabling(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039580_cdoor_updatenavigation_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039581_rage_phcollider_setvelocity_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_039582_cdoor_resetcenterofgravityforfragdoor_rage_fraginst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039585_cdoor_initdoorphys_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039586_cobject_addphysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039587_cdoorsystem_generatehash32_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039588_cdoor_processraceopenings_cdoor_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039589_cdoor_processdoorbehaviour(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039590_cdoor_updateportalstate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039591_cdoorsystem_setpendingstateondoor_cdoorsystemdata_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039592_cdoor_breakoffdoor(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039593_cobject_objecthasbeenmoved_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_039594_cdoor_scriptdoorcontrol_bool_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_039596_cdoor_debuglockselecteddoor_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_039616_cobject_processprecomputeimpacts_rage_phcontactiterator(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039617_rage_phcontactconstiterator_skiptonextactivecontact_rage_phmanif(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_039618_dogolfimpactsputtinghack_rage_phcontactiterator_rage_phboundbvh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32) -> i32; }
extern "C" { pub fn f_039620_cobject_processphysics_float_bool_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_039623_cobject_onactivate_rage_phinst_ptr_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_039625_cobject_processcollision_rage_phinst_const_ptr_centity_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: f32, a7: i64, a8: i32, a9: i32, a10: i64, a11: i32, a12: i32); }
extern "C" { pub fn f_039626_rage_pgdictionary_rage_crexpressions_lookup_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039627_cobject_initanimlazy(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039629_cobject_processcontrol(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039630_cweapon_sfireparams_dtorsfireparams(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039633_cobject_objectdamage_float_rage_vector3_const_ptr_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_039634_cobject_processsniperdamage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039637_tdynobjboundsfuncs_calcboundsforentity_rage_tdynobjbounds_ref_ce(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_039638_cpathservergta_updatevehicledoordynamicobject_cvehicle_ptr_ccard(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039640_cdoor_changedoorstateforscript_unsigned_int_rage_vector3_const_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: f32, a6: i32, a7: i32, a8: f32, a9: f32, a10: i32, a11: i32) -> i32; }
extern "C" { pub fn f_039641_cdoorsystem_setlocked_rage_vector3_const_ref_bool_float_bool_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32, a4: i32, a5: f32, a6: f32, a7: i64, a8: i32); }
extern "C" { pub fn f_039646_cdoor_adddoortolist_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039648_cdoorsystem_createnewdoorsystemdata_int_int_rage_vector3_const_r(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_039650_rage_atmapentry_unsigned_int_cdoorsystemdata_ptr_ref_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039655_cdoorsystem_init(ph: *mut Phantom); }
extern "C" { pub fn f_039656_cdoorsystem_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_039668_cdoortuning_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_039672_cdoortuningfile_namedtuning_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_039682_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_039683_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_039688_cobjectpopulation_destroyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039689_cobject_dumpobjectpool(ph: *mut Phantom); }
extern "C" { pub fn f_039692_cdummyobject_enable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039693_cdummyobject_disable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039696_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_objecti(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_039697_cobject_ispressureplatepressed(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039698_cobject_createconstraintforsetdrivetotarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039699_cobject_setdrivetotarget_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_039700_cobject_isjointatlimitangle_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_039708_rage_mthrandom_reset_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039709_cobject_playautostartanim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039757_cobjectpopulation_converttodummyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039758_cobjectpopulation_manageobjectpopulation_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_039759_cobjectpopulation_managedummy_cdummyobject_ptr_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_039761_cobjectpopulation_createobject_rage_fwmodelid_eentityownedby_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_039762_cobjectpopulation_createobject_cobjectpopulation_createobjectinp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039763_cnavmeshdataexporter_willexportcollision(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_039765_cobjectpopulation_converttorealobject_cdummyobject_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039766_cobjectpopulation_convertallobjectsinareatodummyobjects_rage_vec(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_039775_cprocobjinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_039781_cplantinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_039786_proctaglookup_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_039799_cproceduralinfo_init(ph: *mut Phantom); }
extern "C" { pub fn f_039804_cprocobjectman_init(ph: *mut Phantom); }
extern "C" { pub fn f_039805_cprocobjectman_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_039806_cprocobjectman_removeobject_centityitem_ptr_rage_vfxlist_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039807_cprocobjectman_returnentitytopool_centityitem_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039808_cprocobjectman_update(ph: *mut Phantom); }
extern "C" { pub fn f_039809_cprocobjectman_addobject_rage_vector3_const_ref_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_039812_rage_fwtransform_getmatrix_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039847_rage_cnavresareaeditor_addnewarea_rage_vector3_const_ref_float_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_039857_cnavmeshgta_visualisequadtree_rage_cnavmeshquadtree_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_039861_rage_cpathserverrequestbase_dtorcpathserverrequestbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039870_cpathserverthread_close(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039872_cpathserver_reset_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_039874_cpathserverthread_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039877_cpathserver_isrequestresultready_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_039878_cpathserver_cancelrequestinternal_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_039879_cpathserver_requestpath_rage_vector3_const_ref_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64) -> i32; }
extern "C" { pub fn f_039880_cpathserver_getclosestpositionforpednolock_rage_ainavdomain_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_039881_cpathserver_requestpath_rage_trequestpathstruct_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039882_cpathserver_getclosestpositionforped_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_039885_cpathserver_addblockingobject_rage_vector3_const_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_039886_cpathserver_evictdynamicobjectforscript(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_039892_rage_fwpathserver_getnavmeshfromindex_unsigned_int_rage_ainavdom(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_039895_cpathserver_requestlineofsight_rage_vector3_const_ptr_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64, a8: f32) -> i32; }
extern "C" { pub fn f_039896_cpathserver_requestaudioproperties_rage_vector3_const_ref_rage_t(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_039897_cpathserver_requestclosestviafloodfill_rage_vector3_const_ref_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_039904_cpathserver_getlineofsightresult_unsigned_int_bool_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_039907_cpathserver_getclosestfloodfillresultandclear_unsigned_int_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_039914_cpathserverthread_getnextrequest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039915_rage_cdynamicobjectscontainer_adjustallboundsbyamount_rage_tshor(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_039918_cpathserver_disableclimbatposition_rage_vector3_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039919_dbgwritefile_rage_fistream_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_039920_cpathserver_outputpathrequestproblem_rage_cpathrequest_ptr_char(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039922_cpathserver_extractcoverpointsfromquadtree_rage_cnavmeshquadtree(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_039923_rage_tshortminmax_intersects_rage_tshortminmax_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_039924_cpathserver_drawpolydebug_rage_cnavmesh_ptr_unsigned_int_rage_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_039925_cpathserver_debugpolytext_rage_cnavmesh_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039928_cpathserver_transformworldpostoscreenpos_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_039930_cpathserverthread_calculatesearchextents_rage_cpathrequest_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_039932_cpathserverthread_findapproxnavmeshpolyforrouteendpoints_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32, a6: i32, a7: i64); }
extern "C" { pub fn f_039933_getquadtreedetails_rage_cnavmeshquadtree_ptr_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_039935_tdynobjboundsfuncs_calcboundsforblockingobject_rage_tdynobjbound(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64); }
extern "C" { pub fn f_039941_cpathserver_switchnavmeshinarea_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_039943_pathserver_testfindclosestpos_bool_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_039945_cpathserveropponentpedgen_startsearch_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32, a4: f32, a5: i32); }
extern "C" { pub fn f_039947_cpathserveropponentpedgen_startsearch_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: f32, a5: i32); }
extern "C" { pub fn f_039985_cpathserver_testshortlineofsightimmediate_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_039986_cpathserver_testshortlineofsightimmediate_rg_rage_vector3_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_039991_rage_cnavmesh_getpolyindex_rage_tnavmeshpoly_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_039992_cpathserverthread_tessellatenavmeshpolyandcreatedegenerateconnec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_039993_rage_cdynamicobjectscontainer_doesregionintersectanyobjects_rage(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039994_cpathserverthread_removeandaddobjects(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039995_cpathserverthread_updatealldynamicobjectspositionsnolock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039999_cpathserver_updateblockingobject_unsigned_short_rage_vector3_con(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_040003_tdynobjboundsfuncs_getignorecomponentsforentity_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040004_tdynobjboundsfuncs_calcboundsforvehicledoor_rage_tdynobjbounds_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_040006_cpathservergta_addvehicledoordynamicobject_cvehicle_ptr_ccardoor(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040009_cpathserverthread_testdynamicobjectloscallback_rage_tdynamicobje(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040010_cpathserverthread_testdynamicobjectlos_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_040011_cpathserverthread_testdynamicobjectlos_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_040012_cpathserverthread_findpositionclearofdynamicobjects_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_040013_cpathserverthread_getnavmeshpolyforrouteendpoints_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: f32, a7: f32, a8: i32, a9: i32, a10: i32, a11: f32, a12: i64) -> i32; }
extern "C" { pub fn f_040014_cpathserverthread_testnavmeshlos_rage_vector3_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_040015_rage_cdynamicobjectscontainer_getobjectsintersectingregion_rage(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_040018_cpathserverthread_getmultiplenavmeshpolysforrouteendpoints_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_040019_cpathserverthread_findheightchangeposalongline_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32) -> i32; }
extern "C" { pub fn f_040020_cpathserverthread_findheightchangeposalongliner_rage_tnavmeshpol(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_040021_cpathserverthread_testnavmeshlos_r_rage_tnavmeshpoly_ptr_rage_tn(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_040022_cpathserverthread_pullpathoutfromedges_rage_cpathrequest_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040023_cpathserverthread_testnavmeshlos_rage_vector3_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_040024_rage_normalizeandmag_rage_vector3_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_040027_cpathserverthread_getsharededgeendpoints_rage_tnavmeshpoly_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_040028_cpathserverthread_obtainentryedgeviaconnectionpoly_rage_tnavmesh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i64, a13: i32) -> i32; }
extern "C" { pub fn f_040032_rage_getpointinpolyfrompointenum_rage_cnavmesh_const_ptr_rage_tn(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_040033_cpathserverthread_obtainadjacentpolys_rage_tnavmeshpoly_ptr_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i32); }
extern "C" { pub fn f_040034_rage_fwpathserver_gettessinfo_rage_tnavmeshpoly_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040035_cpathserverthread_getclosestpointinpolytotargetdistsqr_rage_tvis(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> f32; }
extern "C" { pub fn f_040036_cpathserverthread_getpointinpolywithbestanglefromstartwithlos_ra(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_040037_cpathserverthread_getclosestpointinpolyfromlastpointdistsqr_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_040038_cpathserverthread_findapproxnavmeshpolyforrouteendpointsandadjus(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i32, a8: i64); }
extern "C" { pub fn f_040039_cpathserverthread_pathsearch_testadjacencywithradius_rage_cnavme(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_040049_getclosestposcallback_rage_cnavmesh_const_ptr_rage_tnavmeshpoly(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040058_rage_atarray_texternaledgepolys_0_unsigned_short_grow_unsigned_i(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_040065_cpathserverthread_resetrequesttimers(ph: *mut Phantom); }
extern "C" { pub fn f_040066_cpathserverthread_processaudiopropertiesrequestr_rage_cnavmesh_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_040075_cpathserverthread_closestpositionrequest_perpolycallback_rage_cn(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040076_cpathserverthread_closestpositionrequest_perpointcallback_rage_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_040109_rage_cpathserverrequestbase_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040116_std_2_enable_if_is_move_constructible_ccovertuningfile_modeltotu(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040117_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040118_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_040119_void_std_2_insertion_sort_abi_nqn220108_std_2_classicalgpolicy_s(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040120_void_std_2_insertion_sort_unguarded_abi_nqn220108_std_2_classica(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040121_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040122_void_std_2_pop_heap_abi_nqn220108_std_2_classicalgpolicy_std_2_l(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040123_ccovertuningfile_modeltotunename_ptr_std_2_partition_with_equals(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_040124_std_2_pair_ccovertuningfile_modeltotunename_ptr_bool_std_2_parti(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040125_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040126_void_std_2_iterops_std_2_classicalgpolicy_iter_swap_abi_nqn22010(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040127_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040128_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_040129_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_040130_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040134_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040135_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_040136_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_040137_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040139_cpedgroupmembership_getmember_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_040142_cpedgroup_islocallycontrolled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040143_cpedgroup_setdirty(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040144_rage_netsharedarrayhandler_cpedgroup_cpedgroupsarrayhandler_gete(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_040159_cpedgroupmembership_sortbydistancefromleader(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040161_cpedgroupmembership_cpedgroupmember_clearpedgroupmember(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040162_cpedgroupmembership_cpedgroupmembership(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040163_cpedgroupmembership_addmember_cped_ptr_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_040164_cpedgroupmembership_dtorcpedgroupmembership(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040165_cpedgroupmembership_flush_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_040166_cpedgroupmembership_removemember_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_040168_cpedgroupmembership_assignnetworkpedids(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040169_cpedgroupmembership_removemember_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040171_cpedgroupmembership_setleader_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040172_cpedgroupmembership_addfollower_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040174_cpedgroupmembership_ismember_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040177_cpedgroup_isplayergroup_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040182_cpedgroup_process(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040189_cpedgroups_addgroup_epoptype_cgamescripthandler_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_040194_cpedgroups_getpedsgroup_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040204_carea_carea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040205_carea_dtorcarea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040207_carea_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040209_carea_ispointinarea_rage_vector3_const_ref_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_040210_carea_set_rage_vector3_const_ref_rage_vector3_const_ref_float_ce(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32); }
extern "C" { pub fn f_040211_carea_setassphere_rage_vector3_const_ref_float_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_040214_carea_getmaxradius_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040216_cassistedmovementroute_getclosestpos_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> f32; }
extern "C" { pub fn f_040218_cassistedmovementroutestore_process_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_040219_cplayerassistedmovementcontrol_canignoreblockingintersection_wor(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040220_worldprobe_cshapetesthitpoint_gethitinst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040224_cassistedmovementroute_addpoint_rage_vector3_const_ref_float_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_040230_cassistedmovementroute_postprocessroute(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040234_cassistedmovementroutestore_calcdoorhash_cobject_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040295_cactioncondpedother_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_040300_cactioncondpedother_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040305_cactiondefinition_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040314_crelativerange_isviewerlocalpositioninrange_cped_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64, a5: f32, a6: i32) -> i32; }
extern "C" { pub fn f_040315_cactionmanager_targettypegetposdirfromentity_rage_vec3v_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: f32, a7: i64, a8: f32, a9: f32, a10: i64) -> i32; }
extern "C" { pub fn f_040316_cactionmanager_getmatrixfrombonetag_rage_mat34v_ref_centity_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040324_cactionmanager_shouldallowweapontype_cped_const_ptr_rage_atfixed(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040325_cactionmanager_playeraifiltertypedoescatchped_rage_atfixedbitset(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040326_cactionmanager_withinmovementspeed_cped_const_ptr_rage_atfixedbi(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040327_cactioncondspecial_dotest_cped_const_ptr_centity_const_ptr_cacti(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_040328_worldprobe_cshapetestresults_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040329_cplayerresetflags_isflagset_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_040330_cactionmanager_arepedsfriendlywitheachother_cped_const_ptr_cped(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040331_cactionmanager_ispedtypewithinangularthreshold_cped_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_040340_csimpleimpulsetest_dotest_ccontrol_const_ptr_cimpulsecombo_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_040341_cactiondefinition_postload(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040342_cactiondefinition_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040343_cactiondefinition_getactionresult_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_040344_cactiondefinition_issuitable_rage_atfixedbitset_12_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32, a8: i64, a9: i32, a10: i32, a11: i64, a12: i64, a13: i32) -> i32; }
extern "C" { pub fn f_040345_cactionmanager_calculatedesiredheading_cped_const_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_040346_cactionmanager_overrideactiondefinitions_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040351_cactionmanager_selectactionfromset_rage_atfixedarray_cactiondefi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_040357_cactionmanager_initwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_040374_cactiondefinition_dtorcactiondefinition(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040384_ccombatbehaviour_initfromcombatdata_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040390_ccombatinfomgr_load(ph: *mut Phantom); }
extern "C" { pub fn f_040412_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_040415_art_nmrscharacter_calculatecomvalues(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040416_art_nmrscharacter_twistcompensation_art_nmrsgenericpart_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32) -> f32; }
extern "C" { pub fn f_040418_rage_fragnmassetmanager_removeagent_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_040419_art_nmrsengine_destroycharacter_art_nmrscharacter_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040420_art_cbutaskbase_deactivate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040421_art_nmrscharacter_removefromscene(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040457_rage_nmbehavior_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_040498_rage_nmbehaviorinst_dtornmbehaviorinst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040509_art_nmrs1dofeffector_savetoshadow_art_shadow1dof_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040510_art_nmrseffectorbase_calculatejointmatrixcache_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040511_art_nmrseffectorbase_getjointposition_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040514_art_nmrs1dofeffector_applytorque_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_040515_art_nmrs1dofeffector_applyangimpulse_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_040518_art_rsragedrivetwistswingtoquat_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040520_art_nmrseffectorbase_calculatejointquatfromitmcache_art_incoming(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_040521_rage_geomvectors_quatvtoleanandtwist_rage_quatv_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040533_art_softlimitcontroller_precalcconstants(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040535_art_nmrs3dofeffector_setmuscledamping_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_040538_art_nmrs3dofeffector_updatecurrentangles_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040539_art_nmrs3dofeffector_savetoshadow_art_shadow3dof_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040545_art_nmrs3dofeffector_applytorque_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040546_art_nmrs3dofeffector_applyangimpulse_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040552_art_nmrs3dofeffector_setlimits_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_040556_art_nmrs3dofeffector_clamp_rage_vector3_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_040557_art_closestpointonellipse_float_ref_float_ref_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_040559_art_nmrs3dofeffector_clamprawleantwist_rage_quaternion_ref_float(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_040563_art_bulletapplier_newhit_int_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_040564_art_nmrsgenericpart_getarticulatedmatrix_rage_matrix34_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040565_art_bulletapplier_applytorque(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040566_art_bulletapplier_applylift(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040567_art_bulletapplier_applyimpulse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040568_art_bulletapplier_applycounterimpulse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040569_art_nmrsgenericpart_applyforce_rage_vector3_const_ref_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040570_art_nmrsgenericpart_applyimpulse_rage_vector3_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040571_art_nmrsgenericpart_applytorqueimpulse_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040574_art_nmrscharacter_initialisedata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040575_art_stayuprightupdate_art_messageparamsbase_const_ptr_art_nmrsch(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040576_art_configurebulletsupdate_art_messageparamsbase_const_ptr_art_n(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040577_art_configurebulletsextraupdate_art_messageparamsbase_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040578_art_nmrscharacter_dtornmrscharacter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040581_art_setfallingreactionupdate_art_messageparamsbase_const_ptr_art(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040582_art_setcharactercollisionsupdate_art_messageparamsbase_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040583_art_setcharacterdampingupdate_art_messageparamsbase_const_ptr_ar(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040584_art_configureselfavoidanceupdate_art_messageparamsbase_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040585_art_nmrsgenericpart_updatematrices(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040588_art_nmrsbody_postlimbinputs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040591_art_nmrsbody_setstiffness_float_float_unsigned_int_float_ptr_boo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_040592_art_nmrscharacter_setimpactfriction_rage_phcontactiterator_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_040593_art_nmrscharacter_getpredictedcomorientation_float_rage_matrix34(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_040594_art_nmrscharacter_getfacingdirectionfromcomorientation_rage_matr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_040595_art_nmrscharacter_disablelegcollisions_art_nmrsgenericpart_ptr_a(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64) -> i32; }
extern "C" { pub fn f_040596_art_nmrscharacter_disablehandcollisions_art_nmrsgenericpart_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_040597_art_nmrsgenericpart_handlecollision_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64); }
extern "C" { pub fn f_040598_art_nmrscharacter_measurecharacter_rage_matrix34_ref_rage_matrix(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64); }
extern "C" { pub fn f_040599_art_nmrsgenericpart_setmatrix_rage_matrix34_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040600_art_nmrsgenericpart_applyvelocitiestopart_rage_matrix34_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_040601_art_nmrshumanbody_addhumanarm_art_nmrshumanlimbtypes_art_nmrseff(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: f32); }
extern "C" { pub fn f_040602_art_nmrslimb_nmrslimb(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040603_art_nmrsspineinputwrapper_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040604_getstart_art_nmrscharacter_ptr_unsigned_int_ptr_int_int_ref_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_040605_art_nmrscharacter_expressiontomask_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040606_art_nmrscharacter_evaluatemaskexpression_char_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_040607_art_nmrscbudynamicbalancer_calibratelowerbodyeffectors_art_nmrsh(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040608_art_configureshotinjuredlegupdate_art_messageparamsbase_const_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040610_art_cbutaskbase_activate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040627_art_shotnewbulletupdate_art_messageparamsbase_const_ptr_art_nmrs(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040628_art_nmrscharacter_snap_float_float_int_bool_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i64, a14: f32, a15: f32, a16: f32, a17: f32); }
extern "C" { pub fn f_040629_art_nmrscbushot_controlstiffness_tick_float_art_nmrshumanbody_re(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_040630_art_shotsnapupdate_art_messageparamsbase_const_ptr_art_nmrschara(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040631_art_shotshockspinupdate_art_messageparamsbase_const_ptr_art_nmrs(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040632_art_shotfalltokneesupdate_art_messageparamsbase_const_ptr_art_nm(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040633_art_shotfrombehindupdate_art_messageparamsbase_const_ptr_art_nmr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040634_art_shotingutsupdate_art_messageparamsbase_const_ptr_art_nmrscha(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040635_art_shotheadlookupdate_art_messageparamsbase_const_ptr_art_nmrsc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040636_art_shotconfigurearmsupdate_art_messageparamsbase_const_ptr_art(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040647_art_pointgunextraupdate_art_messageparamsbase_const_ptr_art_nmrs(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040651_art_nmrscharacter_instancetoworldspace_rage_vector3_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_040652_art_nmrscharacter_instancetolocalspace_rage_vector3_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_040653_art_nmrscharacter_boundtoworldspace_rage_vector3_ptr_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_040654_art_nmrscharacter_findmatrixofbound_rage_matrix34_ref_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_040655_art_nmrscharacter_boundtolocalspace_bool_rage_vector3_ptr_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_040656_art_nmrscharacter_rotateboundtoworldspace_rage_vector3_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_040657_art_nmrscharacter_bracearm_art_nmrshumanlimbtypes_art_nmrshumanb(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: f32, a7: f32); }
extern "C" { pub fn f_040658_art_nmrsgenericpart_getcollisionzmpwithenvironment_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_040659_art_nmrs3dofeffectorinputwrapper_setdesiredangles_float_float_fl(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_040660_art_nmrscharacter_decidetobrace_float_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32, a7: f32, a8: i64, a9: i64, a10: i64, a11: i64, a12: i64, a13: i64, a14: i32); }
extern "C" { pub fn f_040661_art_nmrscharacter_decidebracehands_float_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64, a8: f32, a9: f32, a10: i64, a11: i64, a12: i64); }
extern "C" { pub fn f_040662_art_nmrscharacter_armsbrace_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32, a8: f32, a9: f32, a10: i64, a11: i64, a12: i64); }
extern "C" { pub fn f_040663_art_nmrscharacter_reachforbodypart_art_nmrshumanbody_ptr_art_rea(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: f32, a6: i32) -> f32; }
extern "C" { pub fn f_040664_art_nmrscharacter_cleverhandik_rage_vector3_ref_art_nmrsgenericp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: f32, a6: i64, a7: i64, a8: i32, a9: f32, a10: i32, a11: i32) -> i32; }
extern "C" { pub fn f_040665_art_nmrscharacter_releaseconstraintsafely_rage_phconstrainthandl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040666_art_nmrscharacter_constrainpart_rage_phconstrainthandle_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: i64, a6: i64, a7: i32, a8: i32, a9: i32, a10: f32); }
extern "C" { pub fn f_040667_art_nmrscharacter_reduceconstraintseperation_rage_phconstraintha(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_040668_art_nmrscharacter_alterarmmasses_art_nmrscharacter_nmrshand_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_040669_art_nmrscharacter_getkineticenergyperkilo_relativevelocity(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_040670_art_nmrscharacter_proberay_art_nmrscharacter_rayprobeindex_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_040671_art_nmrscharacter_getasynchproberesult_art_nmrscharacter_rayprob(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_040672_art_nmrscharacter_submitasynchprobe_rage_vector3_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_040673_art_nmrscharacter_replacewithimpactinfo_art_nmrsgenericpart_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64); }
extern "C" { pub fn f_040674_art_nmrsgenericpart_getcollisionzmpwithnotowncharacter_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_040675_art_nmrscharacter_isinstvalid_nogenidcheck_rage_phinst_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040676_art_nmrscharacter_antispinaroundverticalaxisjuice_float_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_040677_art_nmrscharacter_pdpartorientationtom_art_nmrsgenericpart_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_040678_art_nmrscharacter_pdpartposition_art_nmrsgenericpart_ptr_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i64); }
extern "C" { pub fn f_040679_art_nmrscharacter_changejointangvel_art_nmrs3dofeffector_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_040680_art_nmrscharacter_changejointangvel_art_nmrs1dofeffector_ptr_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_040681_art_nmrscharacter_controlstiffness_art_nmrshumanbody_ref_float_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: f32, a10: f32, a11: f32, a12: i32, a13: i32, a14: f32, a15: f32, a16: f32, a17: f32, a18: f32, a19: i32, a20: i32, a21: i32); }
extern "C" { pub fn f_040682_art_nmrscbudynamicbalancer_setlowerbodygravityopposition_art_nmr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040701_art_nmrsengine_dtornmrsengine(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040703_art_nmrscbupointgun_initialisecustomvariables(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040704_art_limbik_art_nmrslimbikinput_ref_art_nmrslimbikoutput_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040705_art_getselfavoidpolarpos_art_nmrslimbikinput_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040706_art_getselfavoidpos_art_nmrslimbikinput_ref_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040707_art_getfreelean_rage_quaternion_const_ref_rage_vector3_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_040708_art_fk_rage_matrix34_ref_rage_matrix34_ref_art_nmrseffectorbase(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_040709_art_geteffectormatrix2frommatrix1anddesireddriveangles_rage_matr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_040710_art_getpartmatrixfromparenteffectormatrix2_rage_matrix34_ref_art(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040711_art_set3dofparameters_art_nmrs3dofeffector_ptr_art_nmrs3dofeffec(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040712_art_set1dofparameters_art_nmrs1dofeffector_ptr_art_nmrs1dofeffec(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040716_art_nmrshumanarm_setstiffness_art_nmrsarminputwrapper_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32, a5: i64); }
extern "C" { pub fn f_040717_art_nmrs3dofeffectorinputblendable_blend_art_nmrs3dofeffectorinp(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_040718_art_nmrs1dofeffectorinputblendable_blend_art_nmrs1dofeffectorinp(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_040730_art_nmrshumanleg_setstiffness_art_nmrsleginputwrapper_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32, a5: i64); }
extern "C" { pub fn f_040741_art_nmrshumanspine_setstiffness_art_nmrsspineinputwrapper_ptr_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32, a5: i64); }
extern "C" { pub fn f_040751_art_nmrshumanspine_keepheadawayfromground_art_nmrslimbinput_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_040754_art_clamptarget_rage_vector3_ref_rage_vector3_const_ref_rage_mat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_040755_art_buildconvexhull2d_rage_vector3_ptr_int_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_040756_art_getdistancetopoint_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_040763_art_nmrscbuarmswindmill_updatecirclepose_art_nmrscbuarmswindmill(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_040764_art_nmrscbuarmswindmill_updatecirclepose_art_nmrscbuarmswindmill(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040767_art_nmrslimbinput_art_cbutaskbase_createnmrslimbinput_art_nmrsik(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040771_art_nmrscbuarmswindmilladaptive_movearm_float_art_nmrshumanarm_p(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32); }
extern "C" { pub fn f_040772_art_nmrsspineinputwrapper_setbackangles_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_040782_art_nmrscbubodyfoetal_doallthethings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040793_art_nmrscbubraceforimpact_getcardata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040796_art_nmrscbubraceforimpact_oncar(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040803_art_nmrscbucatchfall_armstate_tick_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_040804_art_nmrscbucatchfall_armstate_armik_art_nmrsikinputwrapper_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i64); }
extern "C" { pub fn f_040809_art_dbmbalancesolve_art_nmrscbudynbal_readonly_const_ref_art_nmr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_040811_art_dbmfootplacement_art_nmrscbudynbal_readonly_const_ref_art_nm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040812_art_dbmfootplacementstepchoice_art_nmrscbudynbal_readonly_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040814_art_dbmpelviscontrol_art_nmrscbudynbal_readonly_const_ref_art_nm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040820_art_nmrscbudynamicbalancer_populatebodypacket(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040821_art_nmrscbudynamicbalancer_updatebalancestatus(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040829_art_nmrscbufalloverwall_gethorizontaldistancepointfromline_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_040830_art_nmrscbufalloverwall_setheighttarget_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_040833_art_nmrscbufalloverwall_applygetoverwallforcetopart_art_nmrsgene(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: f32); }
extern "C" { pub fn f_040836_art_nmrscbuflinch_chooseflinchtype(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040837_art_nmrscbuflinch_controlstiffness_entry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040840_art_nmrscbuflinch_armbracingheadpose_art_nmrshumanarm_ptr_art_nm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_040845_art_nmrscbugrab_controlheadlook_bool_rage_vector3_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040846_art_nmrscbugrab_updategrabpos_rage_vector3_ref_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_040847_art_nmrscbugrab_handscrossed(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040848_art_nmrscbugrab_movearm_rage_vector3_ref_rage_vector3_ref_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i32, a8: i64, a9: i64, a10: i64, a11: i32); }
extern "C" { pub fn f_040849_art_nmrscbugrab_projectpointontoplane_float_rage_vector3_ref_rag(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_040857_art_nmrscbuhighfall_getleftleglength_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_040860_art_nmrscbuhighfall_states_art_statemachineevent_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_040861_art_nmrscbuhighfall_duringnonrecoverablefall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040869_art_nmrscbuonfire_setactivatebehaviours(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040875_art_nmrscbupedal_moveleg_art_nmrshumanleg_ptr_float_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_040878_art_nmrscbupointarm_pointthearm_art_nmrscbupointarm_parameters_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040880_art_nmrscbupointgun_updatearmdata_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_040883_art_nmrscbupointgun_isgunhandtargetreachable_art_nmrscbupointgun(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040884_art_nmrscbupointgun_movegunarm_art_nmrscbupointgun_armdata_ref_a(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_040885_art_nmrscbupointgun_setneutralposetype_art_nmrscbupointgun_armda(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_040886_art_nmrscbupointgun_setarmmuscles_art_nmrscbupointgun_armdata_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040887_art_nmrscbupointgun_calculategunhand_art_nmrscbupointgun_armdata(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040888_art_nmrscbupointgun_issupporthandtargetreachable_art_nmrscbupoin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_040889_art_nmrscbupointgun_selectneutraltarget_art_nmrscbupointgun_armd(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040890_art_nmrscbupointgun_setneutralpose_art_nmrscbupointgun_armdata_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040891_art_nmrscbupointgun_ik_art_nmrscbupointgun_armdata_ref_float_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: f32); }
extern "C" { pub fn f_040892_art_nmrscbupointgun_rifleconstrainttick_art_nmrscbupointgun_armd(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040893_art_nmrscbupointgun_pointgunhand_art_nmrscbupointgun_armdata_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_040896_art_nmrscburolldownstairs_applyfoetaltolegs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040903_art_nmrscburollup_plantarm_art_nmrs3dofeffector_const_ptr_art_nm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32, a7: i64) -> f32; }
extern "C" { pub fn f_040907_art_nmrscbushot_reachleft_exit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040908_art_nmrscbushot_reachright_exit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040909_art_nmrscbushot_controlstiffness_exit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040910_art_nmrscbushot_injuredleg_exit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040911_art_nmrscbushot_fling_exit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040913_art_nmrscbushot_injuredarm_tick_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040914_art_nmrscbushot_reacharm_tick_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040915_art_nmrscbushot_injuredleg_tick_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_040920_art_nmrscbusmartfall_states_art_statemachineevent_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_040921_art_nmrscbusmartfall_duringfall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040922_art_nmrscbusmartfall_dobrace(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040923_art_nmrscbusmartfall_duringglide(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040924_art_nmrscbusmartfall_duringbailout(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040925_art_nmrscbusmartfall_duringprepareforlanding(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040926_art_nmrscbusmartfall_duringsplat(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040942_art_nmrscbuyanked_headlookoravoid_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_040943_art_nmrscbuyanked_rollover(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040980_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_040981_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040992_void_rage_atplaceptrcreator_void_ptr_cpednavcapabilityinfo_void(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041001_cbipedcapsuleinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_041047_cpedcomponentsetmanager_getinfo_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_041056_findplayerped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041060_cpeddebugvisualisermenu_getfocusped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041062_cpedintelligence_findmovementtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041063_cpedintelligence_isfriendlywith_cped_const_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041073_centityboundai_getcorners_rage_vector3_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041077_cpedtargetting_getlosstatus_centity_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041085_cpedtargetting_findtarget_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041086_csingletargetinfo_arewhereaboutsknown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041088_cpedintelligence_getnearbyvehicles(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041092_cpedperception_computefovvisibility_centity_const_ptr_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_041116_cpedintelligence_flushimmediately_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041117_cpedintelligence_addtoeventhistory_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041131_cpeddebugvisualisermenu_initbank(ph: *mut Phantom); }
extern "C" { pub fn f_041197_cpedintelligence_setdefaultrelationshipgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041202_cpedintelligence_setrelationshipgroup_crelationshipgroup_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041203_cpedintelligence_getrelationshipgroup_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041209_centityboundai_centityboundai_centity_const_ref_float_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32, a7: i64, a8: f32) -> i64; }
extern "C" { pub fn f_041218_cplayerinfo_getphysicalplayerindex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041219_cpedgeometryanalyser_canpedtargetpedasync_cped_ref_cped_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: i32, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_041226_rage_syslinkeddata_unsigned_int_refreportplayerresetmismatch_uns(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041228_cpedgeometryanalyser_ispedinwateratdepth_cped_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_041237_cpedintelligence_gettaskmelee_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041265_cplayerinfo_isaiming_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041282_cpedpopulation_shouldusestartupmode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041295_cplayerinfo_setupplayergroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041296_cpedfactory_destroyped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041303_cpedfootstephelper_setgroundinfo_unsigned_int_rage_vec3v_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_041317_centityboundai_init_centity_const_ref_float_const_ref_float_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: f32); }
extern "C" { pub fn f_041318_centityboundai_projectboundingboxontozplane_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32); }
extern "C" { pub fn f_041319_centityboundai_getsphere_rage_spdsphere_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041320_centityboundai_computeboundingboxplanes_rage_vector4_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041321_centityboundai_liesinside_rage_vector3_const_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_041331_cpedgeometryanalyser_testpedcapsule_cped_const_ptr_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64, a10: f32, a11: f32, a12: f32, a13: i64, a14: f32) -> i32; }
extern "C" { pub fn f_041332_cpedgeometryanalyser_testifcollisiontypeisvalid_cped_const_ptr_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_041371_cpediksettingsinfo_isikflagdisabled_ikmanagersolvertypes_flags_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041387_cpedintelligence_gettaskmeleeactionresult_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041395_cpedintelligence_ispedclimbingladder_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041413_cpedintelligence_restartnavigationthroughregion_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_041426_cpedintelligence_isbattleaware_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041454_cpedmotiondata_setcurrentmoveblendratio_rage_vector2_ref_float_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_041496_findplayervehicle_cplayerinfo_ptr_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041509_rage_fwregdref_cscenariopoint_rage_fwrefawarebasenonvirt_reset_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041525_cpedpopulation_addpedincar_cvehicle_ptr_bool_int_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64) -> i64; }
extern "C" { pub fn f_041540_cpedpopulation_getdistancesqrtoclosestscheduledpedwithmodelid_ra(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_041541_cpedpopulation_schedulepedinvehicle_cvehicle_ptr_int_bool_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64) -> i32; }
extern "C" { pub fn f_041588_cplayerspecialability_isabilityunlocked_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041628_cpedtype_findpedtype_rage_atnonfinalhashstring_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041746_reportplayerresetmismatch_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_041809_cpopzones_findsmallestforposition_rage_vector3_const_ptr_ezoneca(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_041828_cpopzone_gettranslatedname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041889_cpedtargetting_gettargetlastseenpos_centity_const_ptr_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_041891_cpedtargetting_removethreat_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041998_cqueriableinterface_gettargetfortasktype_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042004_cqueriableinterface_getstatefortasktype_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042025_crelationshipgroup_setrelationship_erelationtype_crelationshipgr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_042026_crelationshipgroup_clearrelationship_crelationshipgroup_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042040_crelationshipmanager_findrelationshipgroup_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_042043_cvehiclecombatavoidancearea_ispointinareas_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042189_cpeddamagemanager_clearpedblood_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042193_cpeddamagemanager_cleardamagedecals_unsigned_char_epeddamagezone(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_042322_cpvdrawbldata_cpvdrawbldata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042323_cpvdrawbldata_dtorcpvdrawbldata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042386_rage_atarray_cpvtexturedata_0_unsigned_short_op_rage_atarray_cpv(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042415_cpedvariationinfo_getmaxnumdrawbls_epedvarcomp_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042425_cpedvariationpack_extractcomponent_rage_pgdictionary_rage_rmcdra(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_042484_char_ptr_rage_formatf_char_200ul_char_ref_200ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042532_cpedvariationdebug_createpedcb(ph: *mut Phantom); }
extern "C" { pub fn f_042606_cpedpropinfo_getmaxnumprops_eanchorpoints_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042790_cwatertesthelper_getriverheightatposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_042814_worldprobe_debugfindgroundzfor3dcoord_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> f32; }
extern "C" { pub fn f_042818_worldprobe_cshapetestmanager_debugsubmittest_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_042821_worldprobe_cshapetestmanager_aborttest_worldprobe_cshapetestresu(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042844_cwatertesthelper_getwaterheightatpositionincludingrivers_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_042848_worldprobe_cshapetestdesc_setexcludeentities_centity_const_ptr_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_043046_ccollisionhistory_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043071_cdeformableobjectmanager_ismodeldeformable_rage_atnonfinalhashst(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_043073_cbuoyancy_process_cphysical_ptr_float_bool_bool_float_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_043074_ctunableobjectmanager_gettuningentry_rage_atnonfinalhashstring_c(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043079_cbuoyancy_resetbuoyancy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043085_worldprobe_cshapetestdesc_setexcludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043086_cbuoyancyinfo_cbuoyancyinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043087_cwatersample_set_cwatersample_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043089_cbuoyancyinfo_dtorcbuoyancyinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043090_cwatersample_cwatersample(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043096_phinstgta_phinstgta_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043112_fraginstgta_fraginstgta_int_rage_fragtype_const_ptr_rage_matrix3(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_043195_phmaterialmgrgta_getpackedpolyflagvalue_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043205_cphysics_createinvalidragdollfilter_rage_fragtype_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043243_worldprobe_cshapetestbatchdesc_addlinetest_worldprobe_cshapetest(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043246_worldprobe_cshapetestbatchdesc_setprobedescriptors_worldprobe_cs(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043280_cphysics_addtophysicsrequestlist_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043331_cwatertesthelper_submitasyncriverboundprobe_rage_vector3_ref_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043334_cwatertesthelper_getwaterlevelincludingrivers_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_043335_worldprobe_debugfindgroundzforcoord_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_043343_worldprobe_cshapetestbatchdesc_addcapsuletest_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043344_worldprobe_cshapetestbatchdesc_setcapsuledescriptors_worldprobe(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043444_meshblendmanager_sblendslot_sblendslot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043485_water_iscameraunderwater(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043569_cplantmgr_updateend(ph: *mut Phantom); }
extern "C" { pub fn f_043661_crendertargets_lockreadonlydepth_bool_bool_rage_grcrendertarget(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_043664_crendertargets_unlockreadonlydepth(ph: *mut Phantom); }
extern "C" { pub fn f_043670_csprite2d_begincustomlist_csprite2d_customshadertype_rage_grctex(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_043675_csprite2d_endcustomlist(ph: *mut Phantom); }
extern "C" { pub fn f_043743_pausemenupostfxmanager_startfadein(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043744_pausemenupostfxmanager_startfadeout(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043751_phonephotoeditor_settext_char_const_ptr_char_const_ptr_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_043753_csprite2d_setgeneralparams_rage_vector4_const_ref_rage_vector4_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043758_phonephotoeditor_startediting_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_043902_crendertargetmgr_getnamedrendertarget_rage_atfinalhashstring_con(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043908_crendertargetmgr_releasenamedrendertarget_rage_atfinalhashstring(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_043911_crendertargetmgr_linknamedrendertargets_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_043912_crendertargetmgr_isnamedrendertargetlinked_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_043913_crendertargets_getuidepthbuffer_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_043916_crendertargets_resolvebuffer_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_043917_crendertargets_getoffscreenbuffer_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_043918_crendertargets_getoffscreenbuffer_resolved_unsigned_int(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_043921_crendertargets_lockuirendertargets(ph: *mut Phantom); }
extern "C" { pub fn f_043922_crendertargets_unlockuirendertargets_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043938_csprite2d_setrenderstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043956_csprite2d_clearrenderstate(ph: *mut Phantom); }
extern "C" { pub fn f_043962_cgtaoldloadingscreen_forceloadingrenderfunction_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043970_river_getriverflowfromposition_rage_vec3v_const_ref_rage_vector2(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043975_csprite2d_delete(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043983_csprite2d_drawrect_float_float_float_float_float_rage_color32_co(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i64); }
extern "C" { pub fn f_043984_csprite2d_drawrectgui_rage_fwrect_const_ref_rage_color32_const_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043986_csprite2d_draw_rage_vector2_const_ref_rage_vector2_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_043989_csprite2d_moveposvectortoscreengui_rage_vector2_ref_rage_vector2(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044100_csecondarywatertune_interpolate_csecondarywatertune_const_ref_cs(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_044122_water_getwaterlevel_rage_vector3_const_ref_float_ptr_bool_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_044123_water_getwaterlevelnowaves_rage_vector3_const_ref_float_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_044174_water_resetsimulation(ph: *mut Phantom); }
extern "C" { pub fn f_044176_water_getcamerawaterheight(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_044179_water_setsecondarywatertunings_csecondarywatertune_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044258_ui3ddrawmanager_assignpedtoslot_rage_atnonfinalhashstring_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_044476_centitydrawhandler_centitydrawhandler_centity_ptr_rage_rmcdrawab(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_044963_creplayaudiotrackprovider_getcountofcategoriesavailable_ereplaym(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_044964_creplayaudiotrackprovider_getmusiccategorynamekey_ereplaymusicty(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_044965_creplayaudiotrackprovider_gettotalcountoftracksforcategory_erepl(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_044966_creplayaudiotrackprovider_getcategoryandindexfrommusictrackhash(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_044967_creplayaudiotrackprovider_getmusictracksoundhash_ereplaymusictyp(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_044968_creplayaudiotrackprovider_getmusictrackusagecount_ereplaymusicty(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_044969_creplayaudiotrackprovider_getmusictracknamekey_ereplaymusictype(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_044970_creplayaudiotrackprovider_getmusictrackdurationms_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_044973_cmontage_getmusicclipsoundhash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_044974_creplayaudiotrackprovider_updatetracksusedinproject_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_044976_cmontage_deletemusicclip_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_044985_creplaycoordinator_shouldupdateoverlays(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_044991_creplaycoordinator_isrenderingvideo(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_044996_creplaycoordinator_shouldshowloadingscreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_044998_creplaycoordinator_getindexofcurrentplaybackclip(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045000_creplaycoordinator_geteffectivecurrentcliptimems(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_045005_creplaycoordinator_ispendingcleanup(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045012_creplayvideometadatastore_getmetadata_int_unsigned_int_ref_crepl(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_045013_creplayvideometadatastore_setmetadata_int_unsigned_int_creplayvi(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_045025_cclip_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045031_cclip_op_cclip_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045032_cmontage_getclip_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045042_cmontage_gettrimmedtimetoclipms_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_045064_cmontage_gettotalcliplengthtimems_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_045082_cclip_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045129_cmontage_getrawendtimeastimems_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_045130_cmontage_isvalidclipreference_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045134_cmontage_validatetransitionduration_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045136_cmontagemusicclip_validate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045137_rage_atarray_cmontagemusicclip_0_unsigned_short_grow_unsigned_in(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045138_cmontage_getmusicclip_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045139_cmontagemusicclip_isradiotrack_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045140_cmontage_getmusicclipcommercialsonlycount_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045141_cmontage_doesclipcontainmusicclipwithbeatmarkers_unsigned_int_co(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045142_cmontage_getambientclip_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045143_cmontage_deleteambientclip_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045144_cmontage_addtext_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_045145_cmontage_gettext_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045146_cmontage_gettextindex_cmontagetext_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_045147_cmontage_deletetext_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045151_cmontage_getrating(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045155_cmontagemusicclip_settrimmeddurationms_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045156_cmontagemusicclip_gettracktotaldurationms_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045157_cmontagetext_setline_unsigned_int_char_const_ptr_rage_vector2_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_045240_cqueueofphotooperations_push_cphotooperationdata_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045318_cphotomanager_getcreatesortedlistofphotosstatus_bool_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045321_cphotomanager_requestcreatesortedlistofphotos_bool_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045322_cphotomanager_islistofphotosuptodate_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045323_cphotomanager_getnumberofphotos_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045330_cphotomanager_requestunloadallgalleryphotos(ph: *mut Phantom); }
extern "C" { pub fn f_045358_cphotomanager_requestloadgalleryphoto_cundeletedentryinmergedpho(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045359_cphotomanager_getloadgalleryphotostatus_cundeletedentryinmergedp(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045360_cphotomanager_requestunloadgalleryphoto_cundeletedentryinmergedp(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045361_cphotomanager_getunloadgalleryphotostatus_cundeletedentryinmerge(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045363_cphotomanager_requestdeletegalleryphoto_cundeletedentryinmergedp(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045364_cphotomanager_gettitleofphoto_cundeletedentryinmergedphotolist_c(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045365_cphotomanager_getnameofphototexturedictionary_cundeletedentryinm(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045366_cphotomanager_getcanphotobeuploadedtosocialclub_cundeletedentryi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045367_cphotomanager_getphotolocation_cundeletedentryinmergedphotolist(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045368_cphotomanager_getsongartist_cundeletedentryinmergedphotolist_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045369_cphotomanager_gethasphotobeenuploadedtosocialclub_cundeletedentr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045370_cphotomanager_getismugshot_cundeletedentryinmergedphotolist_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045788_cped_getpedhealthinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045796_cped_computedefaulttask_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045797_cped_getcurrentmotiontask_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045800_cped_delayedremovaltimereset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045802_cped_setcharparamsbasedonmanagertype(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045824_cped_createbank(ph: *mut Phantom); }
extern "C" { pub fn f_045844_cped_setdefaultdecisionmaker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045873_cped_getclimbphysical_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045886_cped_setmyvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045889_cped_getprimarymotiontask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045903_cped_getiscrouching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045913_cped_isfirstpersonshootermodeenabledforplayer_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045922_cped_getboneposition_rage_vector3_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045926_cped_getisdrivingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045939_cped_setvarrandom_unsigned_int_unsigned_int_rage_atfixedbitset_3(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_045940_cped_setvariation_epedvarcomp_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_045973_cped_issuperlodfromindex_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045977_cped_setdesiredheading_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046002_cped_isinfirstpersonvehiclecamera_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_046048_cped_getisfpsswimming_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046052_cped_issecurityped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046087_cped_isnetworkbot_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046096_cped_getbonepositioncachedv_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046102_cped_getisenteringvehicle_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046103_cped_getspecialability_eplayerspecialabilityslot(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046104_cped_isexitingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046105_cped_getblockingofnontemporaryeventsthisframe_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046116_cped_estimatepose_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046133_cped_getbossid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046134_cped_iscivilianped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046136_cped_shouldbehavelikelaw_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046144_cped_newsay_char_const_ptr_unsigned_int_bool_bool_int_cped_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32) -> i32; }
extern "C" { pub fn f_046152_cped_setpedinvehicle_cvehicle_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_046176_cped_getcontrolfromplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046184_cped_getiksettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046192_cped_isdebuginvincible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046225_cped_gethighheelexpressiondof(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_046227_cped_getconstrainthelperdofs_bool_float_ref_rage_vec3v_ref_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_046279_canimatedbuilding_canimatedbuilding_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046285_centity_dtorcentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046286_centity_tellaudioaboutaudioeffectsremove(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046298_centity_initentityfromdefinition_rage_fwentitydef_ptr_rage_fwarc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_046299_centity_tellaudioaboutaudioeffectsadd(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046301_centity_createdrawable(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046302_cdynamicentity_createskeleton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046303_cdynamicentity_createanimdirector_rage_rmcdrawable_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_046305_cdynamicentity_addtosceneupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046306_centity_sethidden_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046309_centity_getlodrandomseed_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046311_cdynamicentity_deletedrawable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046312_cdynamicentity_removefromsceneupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046316_cbuilding_cbuilding_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046317_centity_centity_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046327_cplayerswitchinterface_isactive_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_046376_ctexturedownloadrequestdesc_ctexturedownloadrequestdesc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046379_cdownloadabletexturemanager_requesttexturedownload_int_ref_ctext(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_046383_cdownloadabletexturemanager_releasetexturedownloadrequest_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046385_cdownloadabletexturemanager_isrequestpending_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046386_cdownloadabletexturemanager_isrequestready_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046387_cdownloadabletexturemanager_hasrequestfailed_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046388_cdownloadabletexturemanager_getrequesttxdslot_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046408_cinteriorinst_getinteriorforlocation_rage_fwinteriorlocation_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046410_centity_getinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046429_cdynamicentity_updateskeleton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046438_cgameworld_add_centity_ptr_rage_fwinteriorlocation_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046442_cinteriorproxy_getfromlocation_rage_fwinteriorlocation_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046445_cportaltracker_collectstack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046450_cdynamicentity_prerender_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046456_cdynamicentity_setposition_rage_vector3_const_ref_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_046457_cdynamicentity_setmatrix_rage_matrix34_const_ref_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_046467_cgameworld_remove_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046469_cgameworld_postremove_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046472_centity_getisaddedtoworld_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046479_centity_updaterenderflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046481_centity_setmodelid_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046484_centity_setisvisibleformodule_eisvisiblemodule_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046490_centity_isvisible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046491_centity_getisonscreen_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046495_centity_isinpathserver_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046496_centity_wasunabletoaddtopathserver(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046497_centity_setphysicsinst_rage_phinst_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046499_centity_getcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046500_centity_getisstatic_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046502_centity_getphysarch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046530_centity_commondebugstr_centity_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046531_centity_getisvisibleformodule_eisvisiblemodule_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046540_centity_sethasexploded_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046545_centity_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046596_centityidhelper_computeentityid_rage_fwentity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046599_centityiterator_centityiterator_int_centity_const_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_046600_centityiterator_getnext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046687_cextracontentmanager_getcontentbyhash_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046701_cextrametadatamgr_refreshbankwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046832_cextrametadatamgr_getcreaturemetadataindices_cpedmodelinfo_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_047012_cfocusentitymgr_getpos(ph: *mut Phantom, a0: i64); }
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
extern "C" { pub fn f_047062_cphysical_setfixedphysics_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_047065_cphysical_getvelocity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047073_cphysical_sethealth_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047075_cphysical_setweapondamageinfo_centity_ptr_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_047083_cphysical_setupmissionstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047085_cphysical_updateentityfromphysics_rage_phinst_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_047090_cphysical_processcollision_rage_phinst_const_ptr_centity_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: f32, a7: i64, a8: i32, a9: i32, a10: i64, a11: i32, a12: i32); }
extern "C" { pub fn f_047091_cphysical_addcollisionrecordbeforedisablingcontact_rage_phinst_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_047092_cphysical_docollisioneffects(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047096_cphysical_applyforcecg_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_047101_cphysical_applyimpulsecg_rage_vector3_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047106_cphysical_getlocalspeed_rage_vector3_const_ref_bool_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047114_cgameworld_postadd_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047124_cphysical_debugprocessattachment_char_const_ptr_char_const_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_047134_cphysical_cleanupmissionstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047299_ctexlod_storehdmapping_rage_estoretype_unsigned_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_047305_cgameworld_getmainplayerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047333_clodmgr_withinvisiblerange_centity_ptr_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_047376_ccompentity_ccompentity_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_047617_ctxdrelationship_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_047620_cmultitxdrelationship_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_047744_cinteriorinst_testportalstate_bool_bool_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_047880_cgameworld_findlocalplayercoors(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047971_cgameworld_findobjectskindacolliding_rage_vector3_const_ref_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32); }
extern "C" { pub fn f_047982_cgameworld_findlocalplayervehicle_bool(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047993_cgameworld_clearculpritfromallcarfires_centity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_048043_cgameworldheightmap_getmaxheightfromworldheightmap_float_float_f(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_056528_object_commands_commandadddoortosystem_int_int_rage_scrvector_co(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_056530_object_commands_commanddoorsystemsetstate_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_056535_object_commands_commanddoorsystemsetopenratio_int_float_bool_boo(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_063736_gtathread_getthreadwiththreadid_rage_scrthreadid(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063778_cscriptbraindispatcher_itemwaitingforbrain_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063782_chiddenobjects_removefromhiddenobjectmap_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_063796_cthescripts_unregisterentity_cphysical_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063864_cscriptangledarea_set_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64); }
extern "C" { pub fn f_063865_cscriptangledarea_testpoint_rage_vector3_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063870_cscriptsforbrains_checkandapplyifnewentityneedsscript_centity_pt(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063874_cscriptsforbrains_getscriptfilename_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063900_cscriptentities_clearspaceformissionentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064019_cscripthud_setupscaleformmovie_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_064025_cscripthud_getfakedpausemapplayerpos_rage_vector2_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064215_cgamescriptid_generatescriptidhash(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064493_ragesecpopreaction(ph: *mut Phantom); }
extern "C" { pub fn f_064501_ccustomshadereffectbasetype_addref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064503_ccustomshadereffectbasetype_setupmasterformodelinfo_cbasemodelin(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064504_ccustomshadereffectbasetype_setupfordrawable_rage_rmcdrawable_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_064562_ccustomshadereffectped_setupfordrawablecomponent_rage_rmcdrawabl(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_064588_ccustomshadereffecttint_selecttintpalette_unsigned_char_rage_fwe(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064589_ccustomshadereffecttint_getmaxtintpalette(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064633_ccustomshadereffectvehicle_getramptxd(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_064651_ccustomshadereffectvehicle_setupburnouttexture_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064698_cshaderlib_setglobalalpha_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_064700_cshaderlib_setglobalemissivescale_float_bool(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_064771_cpopulationstreaming_emergencyremovevehiclestructures(ph: *mut Phantom); }
extern "C" { pub fn f_064772_cpopulationstreaming_reclassifyloadedvehiclesfornetwork(ph: *mut Phantom); }
extern "C" { pub fn f_064774_cloadedmodelgroup_countmembers_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064776_cpopulationstreaming_requestvehicledriver_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064788_cpopulationstreaming_iscarmodelneededcurrently_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064791_cpopulationstreaming_pedhasbeenstreamedin_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064794_cpopulationstreaming_addstreamedinmodeldata_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064795_cpopulationstreaming_addstreamedindependencies_rage_atarray_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_064800_cstreaming_hasobjectloaded_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064801_cstreaming_getobjectvirtualsize_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064802_cstreaming_getobjectphysicalsize_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064805_cpopulationstreaming_removestreamedinmodeldata_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064806_cpopulationstreaming_removestreamedindependencies_rage_atarray_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_064807_cpopulationstreaming_unrequestvehicledriver_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064809_cstreaming_getobjectanddependencies_rage_strlocalindex_int_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_064811_cpopulationstreaming_removestreamedvehiclevariation_cvehiclestre(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064832_cstreaming_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064866_cscriptpreloaddata_cscriptpreloaddata_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_064867_cscriptpreloaddata_dtorcscriptpreloaddata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064869_cscriptpreloaddata_haspreloadfinished(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064873_cscriptpreloaddata_addfrag_rage_strlocalindex_int_int_ref_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_064874_cscriptpreloaddata_adddrawable_rage_strlocalindex_int_int_ref_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_064913_ckeyboard_getkeyjustdown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_064929_cstreaming_setobjectisdeletable_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064930_cstreaming_getobjectisdeletable_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064931_cstreaming_setmissiondoesntrequireobject_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064933_cstreaming_requestobject_rage_strlocalindex_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_064934_cstreaming_removeobject_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064935_cstreaming_getobjectflags_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064936_cstreaming_getobjectanddependenciessizes_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_064937_cstreaming_loadobject_rage_strlocalindex_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064942_cstreamingcleanup_deletealldrawablereferences(ph: *mut Phantom); }
extern "C" { pub fn f_064979_createflagstring_unsigned_int_char_ptr_unsigned_long(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_065273_ckeyboard_getkeydown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_065373_ccontrol_disableallinputs_unsigned_int_rage_iovalue_disableoptio(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_065385_ccontrol_getvalue_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_065446_ccontrolmgr_getmainplayercontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065458_ccontrolmgr_updatevirtualkeyboard(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065461_ccontrolmgr_getmainfrontendcontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065469_ccontrolmgr_startplayerpadshakebyintensity_unsigned_int_float_in(ph: *mut Phantom, a0: i32, a1: f32, a2: i32); }
extern "C" { pub fn f_065470_ccontrolmgr_showvirtualkeyboard_rage_iovirtualkeyboard_params_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065471_ccontrolmgr_getvirtualkeyboardstate(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065472_ccontrolmgr_isshowingkeyboardtextbox(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065609_cfilemgr_getassetsfolder(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_065621_cfilemgr_openfileforwriting_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066004_ccombattaskmanager_countpedsincombatwithtarget_cped_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_066008_ccombatmeleemanager_findmeleegroup_centity_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066075_ctacticalanalysis_find_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066077_ctacticalanalysis_request_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066157_ctaskthreatresponse_ctaskthreatresponse_cped_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_066467_ctaskcombat_ctaskcombat_cped_const_ptr_float_rage_fwflags_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_066683_ccoverpointscontainer_checkifnooverlapandgetgridcell_rage_vector(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_066686_ccover_addcoverpointfromobject_cobject_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066707_ccover_findcoverpointheight_float_int(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_066711_ccoverpointinfo_init_rage_vector3_ref_float_float_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_067917_ctaskcomplexcontrolmovement_ctaskcomplexcontrolmovement_ctask_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_068296_ctaskcallpolice_cangivetowitness_cped_const_ref_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068298_ctaskcallpolice_givetowitness_cped_ref_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068442_ctaskmotionped_checkfordiving(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068713_ctaskmotioninturret_isseatwithhatchprotection_int_cvehicle_const(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_069063_ctaskmovegotopoint_calculatemoveblendratioforcorner_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: i64, a8: i64) -> f32; }
extern "C" { pub fn f_069358_ctasknavbase_initpathrequest_distanceaheadcalculation_float_cped(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_069477_ctaskmovefollownavmesh_ctaskmovefollownavmesh_float_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: i64, a8: f32, a9: i32) -> i64; }
extern "C" { pub fn f_070401_ctasknmbehaviour_canuseragdoll_cped_ptr_eragdolltriggertypes_cen(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_071010_ctaskgangpatrol_ctaskgangpatrol_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_072928_ctaskscenario_getscenarioinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073473_ctaskusescenario_attach_rage_vector3_const_ref_rage_quaternion_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_073764_ctaskwitness_cangivetowitness_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073765_ctaskwitness_givetowitness_cped_ref_cped_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075342_ctaskhelperfsm_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075361_cnavmeshclosestpositionhelper_startclosestpositionsearch_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_075364_cnavmeshclosestpositionhelper_getsearchresults_searchstatus_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_075367_cnavmeshclosestpositionhelper_resetsearch(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075390_cnavmeshroutesearchhelper_dtorcnavmeshroutesearchhelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075391_cnavmeshroutesearchhelper_startsearch_cped_ptr_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i64, a6: i32, a7: i64, a8: f32, a9: i32); }
extern "C" { pub fn f_075392_cnavmeshroutesearchhelper_getsearchresults_searchstatus_ref_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_075442_cnavmeshlineofsighthelper_cnavmeshlineofsighthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075443_cnavmeshclearareahelper_startclearareasearch_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: i32, a6: f32); }
extern "C" { pub fn f_075444_cnavmeshclearareahelper_getsearchresults_searchstatus_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075474_cmovenetworkhelper_setfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_075483_csimpleblendhelper_isflagset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075488_cgenericcliphelper_blendinclipbydictandhash_cdynamicentity_ptr_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_075523_ctuning_dtorctuning(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075534_ctaskvehiclefsm_candisplaceped_cped_const_ptr_cped_const_ptr_cve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_075536_ctaskvehiclefsm_canjackped_cped_const_ptr_cped_const_ptr_cvehicl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075568_cfindnearestcarnodehelper_cfindnearestcarnodehelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075569_cfindnearestcarnodehelper_getnearestcarnode_rage_cnodeaddress_re(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075570_cfindnearestcarnodehelper_setsearchposition_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075571_cfindnearestcarnodehelper_generaterandompositionnearroadnode_cfi(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075577_ccoverpointstatushelper_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075578_ccoverpointstatushelper_dtorccoverpointstatushelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075601_clastnavmeshintersectionhelper_get_cped_const_ref_rage_vec3v_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075604_ctaskcontrolvehicle_ctaskcontrolvehicle_cvehicle_ptr_rage_aitask(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_075621_ctaskmanager_processpostmovement_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_075660_ctuning_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_075664_ctuning_registertuning_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_075752_ctaskvehiclefsm_computetargettransforminworldspace_cvehicle_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_075758_ctaskvehiclefsm_getpedinorusingseat_cvehicle_const_ref_int_cped(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_075820_ccarenterexit_getdoorcomponents_cvehicle_const_ptr_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_075824_ctaskvehiclefsm_findgroundpos_centity_const_ptr_cped_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_075866_ctaskvehiclefsm_isanygroupmemberusingentrypoint_cped_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_075868_ctaskvehiclefsm_canforcepedtoshuffleacross_cped_const_ref_cped_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075872_ctaskvehiclefsm_getentrypedistryingtoenter_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075873_ctaskvehiclefsm_getseatpedistryingtoenter_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075920_ctaskvehiclefsm_shouldwarpinandoutinmp_cvehicle_const_ref_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075927_ctaskvehiclefsm_pedshouldwarpbecausehangingpedintheway_cvehicle(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075928_ctaskvehiclefsm_canpedwarpintohoveringvehicle_cped_const_ptr_cve(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_075936_ctaskvehiclefsm_ispedheadingtoleftsideofvehicle_cped_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075938_ctaskentervehiclealign_computealigntarget_rage_vector3_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i32); }
extern "C" { pub fn f_075973_ctaskvehiclefsm_shouldalterentrypointforopendoor_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_075992_ctaskexitvehicle_isvehicleonground_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076025_ctaskvehiclefsm_doesvehiclehavedoublebackdoors_cvehicle_const_re(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076093_ctaskexitvehicle_isverticalprobeclear_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_076101_ctaskexitvehicleseat_getexitclipsetid_cvehicle_const_ptr_int_cve(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_076121_ctaskvehiclefsm_pedshouldwarpbecausevehicleatawkwardangle_cvehic(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076290_ctaskvehiclefsm_getentryradiusmultiplier_centity_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_076294_ctaskvehiclefsm_getdoorforentrypointindex_cvehicle_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_076295_ctaskvehiclefsm_getpedsgroupplayerleader_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077089_cstatsmgr_incrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077091_cstatsmgr_setgreater_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077151_statid_char_gethash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077191_cstatsdatamgr_getstat_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077193_cstatsdatamgr_iskeyvalid_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077329_statid_set_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077330_statsinterface_getcharacterindex(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077331_statsinterface_getintstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077335_statsinterface_getstattype_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077342_statsinterface_setstatdata_statid_const_ref_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077343_statsinterface_setstatdata_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077354_statsinterface_getfloatstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_077355_statsinterface_getbooleanstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077358_statsinterface_getuint32stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077359_statsinterface_getuint64stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077360_statsinterface_getint64stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077362_statsinterface_unpackdate_unsigned_long_int_ref_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_077373_statsinterface_getstatsmodelhashid_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077387_cstatsutils_gettime_unsigned_long_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077388_cstatsutils_gettime_int_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_077394_cstatsutils_isstatstrackingenabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077461_moneyinterface_memorytampering_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077476_cnumberwithinmessage_set_float_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077479_ctextfile_get_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077481_ctextfile_get_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_077488_cmessages_insertnumbersandsubstringsintostring_char_const_ptr_cn(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32); }
extern "C" { pub fn f_077499_ctextfile_unload_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077507_chelpmessage_setmessagetext_int_char_const_ptr_cnumberwithinmess(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_077514_chelpmessage_clearmessage_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077533_ctextlayout_ctextlayout(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077535_ctextlayout_dtorctextlayout(ph: *mut Phantom); }
extern "C" { pub fn f_077537_ctextlayout_setscale_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077540_ctextlayout_render_rage_vector2_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077548_ctext_filterouttokensfromstring_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077549_ctext_flush(ph: *mut Phantom); }
extern "C" { pub fn f_077550_ctext_isasianlanguage(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077551_ctext_getsclanguagefromcurrentlanguagesetting(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077555_ctext_getfullfontlibfilename(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077557_ctextconversion_texttohtml_char_const_ptr_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_077559_ctextconversion_stripnonrenderabletext_char_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077560_ctextconversion_formatintforhumans_long_char_ptr_int_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_077561_ctextconversion_formatmstimeasstring_char_ptr_unsigned_long_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_077565_ctextfile_doestextlabelexist_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077571_ctextfile_load_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077575_ctextfile_requestadditionaltext_char_const_ptr_int_ctextfile_elo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077581_ctextfile_hasadditionaltextloaded_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077589_ctextformat_getinputbuttons_rage_inputtype_char_ptr_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_077591_ctextformat_getinputgroupbuttons_rage_inputgroup_char_ptr_unsign(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_077593_ctextformat_doescharacterexistinfont_int_unsigned_short(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_077791_cpathfind_checkpathnodeintegrityforzone_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077792_cpathfind_testalllinks_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077812_cpathfind_findnodeclosesttocoors_rage_vector3_const_ref_float_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: f32, a11: i32, a12: i64, a13: i32, a14: i32); }
extern "C" { pub fn f_077813_cpathfind_findnodeslinkindexbetween2nodes_rage_cnodeaddress_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_077824_cpathfind_forallnodesinarea_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_077874_cpathfind_findroadboundaries_int_int_float_bool_bool_float_ref_f(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: i32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_077889_cpathfind_recordnodesincircle_rage_vector3_const_ref_float_int_r(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_077891_cpathfind_findnodeclosesttonodefavordirection_cpathfind_findnode(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077892_cpathfind_findnodeindirection_cpathfind_findnodeindirectioninput(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077981_cvehicleintelligence_gettaskfrommissionidentifier_cvehicle_ptr_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: f32, a6: f32, a7: f32, a8: i32, a9: i32) -> i64; }
extern "C" { pub fn f_078865_cvehicle_getdefaultboneposition_int_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_078871_cvehicle_containslocalplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078901_cvehicle_destroyvehicleweaponmgr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078911_ctransmission_setuptransmission_float_float_float_unsigned_char(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: i64); }
extern "C" { pub fn f_078917_cvehiclefactory_modifyvehicletopspeed_cvehicle_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078918_cvehicle_setcardoorlocks_ecarlockstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078939_cvehicle_switchengineon_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_078955_cvehicle_turnsirenon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079007_cvehicle_getnumberofpassenger_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079023_cvehicle_getattachedtrailer_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079035_cvehicle_isreversetrike_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079039_cvehicle_isturretseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079046_cvehicle_istrike_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079071_cvehicle_isentryindexvalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079099_cautomobile_isparachutedeployed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079130_cvehicle_getlayoutinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079134_cvehicle_getdriverseat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079157_cvehicle_getisjetski_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079226_cvehicle_setliveryid_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079227_cvehicle_setlivery2id_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079228_cvehicle_updatebodycolourremapping_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079267_ccardoor_getisintact_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079271_ccardoor_getlocalmatrix_cvehicle_const_ptr_rage_matrix34_ref_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_079272_ccardoor_getlocalboundingbox_cvehicle_const_ptr_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_079278_cvehicle_initialiseweaponblades(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079279_cvehicle_initialisespikecollision(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079280_cvehicle_initialiserammingscoops(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079281_cvehicle_initialiserammingbars(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079282_cvehicle_initialiseramps(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079335_chandlingdata_gethandlingdata_ehandlingtype(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079347_chandlingdatamgr_gethandlingdata_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_079404_cvehiclegadgetpickuprope_detachentity_cvehicle_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_079410_cheli_generateheli_cped_ptr_unsigned_int_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_079412_cvehicle_setupdriver_bool_bool_unsigned_int_cambientmodelvariati(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_079413_cvehicle_delayedremovaltimereset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079422_cvehicle_getvehiclegadget_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079441_clandinggear_getpublicstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079550_cplane_isentrypointuseable_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079551_cvehicle_getentryaniminfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079552_cvehiclelayoutinfo_getentrypointinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079587_cvehicle_getfragmentcomponentindex_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079646_ctrain_trainconfigspostload(ph: *mut Phantom); }
extern "C" { pub fn f_079708_cvehicle_hasexpandedmods(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079745_cvehiclelayoutinfo_getseatanimationinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079753_cvehicle_createvehicleweaponmgr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079769_cvehicle_getminmaxboundsfromdummyvehiclebound_cvehicle_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_079772_cvehicle_getvehicleorientation_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079786_cvehicle_getexhaustmatrixfromboneindex_rage_mat34v_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_079791_cvehicle_isdriver_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079793_cvehiclepopulation_addpolicevehoccupants_cvehicle_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_079796_cvehicle_hasalivepedsinit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079797_cvehicle_hasalivevictiminit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079812_cvehicle_getentryinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079816_cvehicle_getseatinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079821_cvehicle_setuppassenger_int_bool_unsigned_int_cambientmodelvaria(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_079829_cvehicle_getdoorlockstatefromentrypoint_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079831_cvehicle_getdirectentrypointindexforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079836_cvehicle_istaxi_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079854_cvehicle_containsped_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079866_cvehicle_turnoffextra_ehierarchyid_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079870_cturret_getturretmatrixworld_rage_matrix34_ref_cvehicle_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079883_cvehiclelayoutinfo_getseatinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079885_cvehicle_getentryexitpoint_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079890_cvehicle_isdoorclearforped_rage_vector3_const_ref_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_079892_cvehicle_haswaterentry_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079904_cvehicle_placeonroadadjust_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_079926_cvehicle_getliveryid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079927_cvehicle_setliverymodfrag_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079932_cvehicleweaponmgr_getfirstturretforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079933_cturret_getlocalheading_cvehicle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_079946_cvehicle_isanexclusivedriverpedoropenseat_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079948_cpoolhelpers_getvehiclepoolsize(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_080141_cvehiclefactory_updatevehiclelist(ph: *mut Phantom); }
extern "C" { pub fn f_080144_cvehiclefactory_createbank(ph: *mut Phantom); }
extern "C" { pub fn f_080187_cvehiclefactory_createcar_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080271_cvehiclegadgetwithjointsbase_getjoint_short(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080279_cvehiclegadgetarticulateddiggerarm_getjointsetup_ediggerjoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_080411_cvehiclepopulation_removeveh_cvehicle_ptr_bool_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080424_cvehiclepopulation_generateoneemergencyservicescarwithroadnodes(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080426_cvehiclepopulation_generateoneemergencyservicesvehiclewithmatrix(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080431_cvehiclepopulation_countvehsoftype_unsigned_int_bool_bool_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_080443_cvehiclepopulation_removeallvehshard(ph: *mut Phantom); }
extern "C" { pub fn f_080546_cvehiclelayoutinfo_getentrypointaniminfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080572_cvehicleentrypointinfo_getseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080573_cvehicleextrapointsinfo_getextravehiclepointoftype_cextravehicle(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080665_cvehiclemetadatamgr_getvehiclelayoutinfo_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_080667_cvehiclemetadatamgr_getseataniminfofromseatindex_centity_const_p(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080668_cvehiclemetadatamgr_getvehiclescenariolayoutinfo_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_080687_cvehiclescenariolayoutinfo_getscenariopointinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080864_cvfxhelper_getdistsqrtocamera_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_080865_cvfxhelper_getmatrixfrombonetag_rage_mat34v_ref_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_080879_cvfxhelper_getinterpvalue_float_float_float_bool(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_080882_cvfxhelper_isentityincurrentvehicle_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080883_cvfxhelper_getwaterdepth_rage_vec3v_const_ref_float_ref_rage_fwe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_080970_cfiremanager_startmapfire_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: i32, a10: i64, a11: i64, a12: i32, a13: i32, a14: i32) -> i64; }
extern "C" { pub fn f_080997_cvehicleglasscomponent_shutdowncomponent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081205_cclouds_requestcloudtransition_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_081355_cfire_getpositionworld_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081440_cfire_calccurrradius_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_081448_cfire_getmaxradius_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_081450_cfire_registerinusebydispatch(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081459_cfiremanager_clearculpritfromallfires_centity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081466_cfiremanager_getactivefire_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_081651_cptfxmanager_removeptfxfromentity_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081719_cptfxmanager_relocateentityptfx_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081753_cvehicleglassmanager_prerender2_cphysical_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082065_cpedweaponmanager_getequippedweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082071_cpedweaponmanager_getequippedweapon_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082116_cexplosionmanager_disablecrimereportingforexplosions_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082144_cfiringpattern_setfiringpattern_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082173_cweapon_process_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_082185_cweapon_calcfirevecfromgunorientation_centity_const_ptr_rage_mat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_082187_cweapon_getcanreload_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082220_cweapon_update_hd_models_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082221_cweaponcomponent_update_hd_models_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082263_cweaponmanager_getweaponunarmed_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082524_cweaponinfo_gettorsoaimoffsetforped_rage_vector3_ref_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082555_cpedinventoryloadoutmanager_setloadout_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082580_cpedinventory_addweaponandammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_082585_cpedinventory_setweaponusinginfiniteammo_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082630_cpedinventoryloadoutmanager_hasloadout_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082644_cpedweaponmanager_equipbestweapon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_082650_cpedweaponmanager_setbackupweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082653_cpedweaponmanager_getisarmedgun_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082727_cprojectilemanager_disablecrimereportingforexplosions_centity_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082885_networkinterface_isanysessionactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082904_clockoverridedata_stopoverriding(ph: *mut Phantom); }
extern "C" { pub fn f_082909_cnetworktelemetry_appendmetric_rage_rlmetric_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082924_cnetwork_isclocktimeoverridden(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082925_cnetwork_startweathertransitiontoglobal_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082938_networkinterface_islocalplayeronline_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082939_networkinterface_hasvalidroscredentials_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082999_networkinterface_registerobject_rage_netgameobjectwrapper_cdynam(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083003_clockoverridedata_overriderate_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083006_clivemanager_isonline_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083007_clivemanager_issignedin_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083009_networkinterface_isonlineros(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083011_networkinterface_hasvalidrockstarid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083015_clivemanager_checkonlineprivileges_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083028_networkinterface_isinspectatormode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083034_networkinterface_isfriendlyfireallowed_cped_const_ptr_centity_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083036_networkinterface_ispedallowedtodamageped_cped_const_ref_cped_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083037_networkinterface_allowcrimeorthreatresponseplayerjustresurrected(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083041_networkinterface_canregisterobject_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083042_networkutils_isclosetoanyplayer_rage_vector3_const_ref_float_rag(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_083046_networkinterface_isvisibleorclosetoanyremoteplayer_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083047_networkutils_isvisibletoanyremoteplayer_rage_vector3_const_ref_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i32; }
extern "C" { pub fn f_083061_networkinterface_getnetworkobject_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083063_networkinterface_unregisterobject_rage_netgameobjectwrapper_cdyn(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083091_networkinterface_unregisterscriptdoor_cdoorsystemdata_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083120_networkinterface_onentitysmashed_cphysical_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_083123_networkinterface_shouldtriggerdamageeventforzerodamage_cphysical(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083151_networkinterface_registerkillwithnetworktracker_cphysical_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083160_networkutils_islocalplayersturn_rage_vec3v_const_ref_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i32; }
extern "C" { pub fn f_083161_networkinterface_findclosestwantedplayer_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_083172_networkinterface_isaghostvehicle_cvehicle_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083298_networkclan_hasprimaryclan(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083312_networkcrewemblemmgr_requestemblem_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083316_networkcrewemblemmgr_releaseemblem_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083333_networkcrewemblemmgr_getemblemtxdslot_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083334_networkcrewemblemmgr_getemblemname_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083342_networkclan_getcrewemblemmgr(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083343_networkcrewemblemmgr_requestpending_emblemdescriptor_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083344_networkcrewemblemmgr_requestsucceeded_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083350_cloudlistener_cloudlistener(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083356_ccloudmanager_isrequestactive_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083373_ccloudmanager_removelistener_cloudlistener_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083383_ccloudmanager_requestgettitlefile_char_const_ptr_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_083390_ccloudmanager_requestgetmemberfile_int_rage_rlcloudmemberid_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083391_ccloudmanager_requestpostmemberfile_int_char_const_ptr_void_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083392_ccloudmanager_requestdeletememberfile_int_char_const_ptr_rage_rl(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_083458_cvideouploadmanager_custom_isuploading_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083469_achmgr_awardachievement_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083496_ctunables_tryaccess_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083498_ctunables_tryaccess_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> f32; }
extern "C" { pub fn f_083680_networkutils_getpedfromnetworkobject_rage_netobject_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083898_csyncedentity_csyncedentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083899_csyncedentity_csyncedentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083902_csyncedentity_getentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083903_csyncedentity_getentityid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083904_csyncedentity_setentityid_unsigned_short_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083908_csyncedvehicle_getvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084087_clivemanager_showsigninui_unsigned_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084112_networkclan_getuiformattedclantag_rage_rlclandesc_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_084333_cnetworktelemetry_weatherchanged_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_084339_cnetworktelemetry_cheatapplied_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084340_cnetworktelemetry_emergencysvcscalled_int_rage_vector3_const_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_084421_cprofilestats_flush_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_084463_networkclan_getuiformattedclantag_bool_bool_char_const_ptr_int_r(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_084540_baseplayercarddatamanager_dtorbaseplayercarddatamanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084604_socialclubmgr_isconnectedtosocialclub_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084629_cloudlistener_dtorcloudlistener_138661(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086259_cnetworkshoppingmgr_shoulddonulltransaction_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_086304_networkgametransactions_inventoryitemset_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086743_cnetworkobjectpopulationmgr_canregisterlocalobjectsoftype_unsign(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_086744_cnetgameplayer_getplayerped_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086763_cnetobjgame_getplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086768_cnetobjdoor_cnetobjdoor_cdoor_ptr_unsigned_short_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_086785_cnetobjentity_flagfordeletion(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086797_cnetworkobjectpopulationmgr_getnumreservedobjects_unsigned_int_u(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_086798_cnetworkobjectpopulationmgr_getnumcreatedobjects_unsigned_int_un(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_086800_cnetworkobjectpopulationmgr_canregisterobjects_unsigned_int_unsi(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_086833_cnetobjobject_cnetobjobject_cobject_ptr_unsigned_short_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_086837_cnetobjobject_removeunassignedambientobject_cnetobjobject_ref_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_086854_cnetobjdoor_assigndoor_cdoor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086855_cnetobjdoor_setdoorsystemdata_cdoorsystemdata_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086894_cnetobjobject_hastobecloned_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_086895_cnetobjobject_canbenetworked_cobject_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_086896_cnetobjobject_assignambientobject_cobject_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086972_cnetobjgame_islocallyrunningscriptobject_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087003_cnetobjobject_isambientobjecttype_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_087025_cnetobjobject_addunassignedambientobject_cnetobjobject_ref_char(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_087106_cnetobjobject_trytoregisterambientobject_cobject_ref_bool_char_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_087295_cnetobjped_getisdamagablebyplayer_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_087320_cnetobjped_onpedcleardamage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_088038_csyncedpedvehicleentryscriptconfig_shouldrejectbecauseforcedtous(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_088040_csyncedpedvehicleentryscriptconfig_hasaforcingflagsetforthisvehi(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_089369_cweapondamageevent_cleardelayedkill(ph: *mut Phantom); }
extern "C" { pub fn f_089404_cdebuggameclockevent_trigger_unsigned_int_rage_netplayer_const_p(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_089412_cdebuggameweatherevent_trigger_bool_unsigned_int_unsigned_int_ra(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_089508_calterwantedlevelevent_trigger_rage_netplayer_const_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_089509_calterwantedlevelevent_trigger_rage_netplayer_const_ref_ecrimety(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_089603_cincidententityevent_trigger_unsigned_int_dispatchtype_cincident(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090710_asinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090714_atanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090717_atof(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090719_isspace(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090755_exp2f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090801_localtime(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090814_memcmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090815_mktime(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090826_powf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090829_iprintf(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090830_small_printf(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090864_pthread_cond_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090885_pthread_mutex_destroy(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090887_pthread_mutex_lock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090889_pthread_mutex_unlock(ph: *mut Phantom, a0: i64) -> i32; }
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
extern "C" { pub fn f_090940_strncasecmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090941_strncat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090942_strncmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090945_strrchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090947_strstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090956_strtok(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090965_tanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090970_tolower(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_091002_emscripten_builtin_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091035_void_std_2_sort_std_2_less_unsigned_int_unsigned_int_ref_unsigne(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
