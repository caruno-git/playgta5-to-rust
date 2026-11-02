//! Callee declarations for crate gen016.
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
extern "C" { pub fn f_000120_rage_sysipccreatemutex(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_000121_rage_sysipclockmutexcommand_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000122_rage_sysipcunlockmutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000128_rage_sysipccreatethread_void_ptr_void_ptr_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_000132_rage_sysipcgetprocessorcount(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000135_rage_sysipcsleep_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000238_rage_grcdevice_createdevicewindow_hwnd_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000644_rage_athashnextsize_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000647_rage_atreferencecounter_atreferencecounter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000654_rage_atstring_set_char_const_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_000663_rage_atstring_replace_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000667_rage_atstring_split_rage_atarray_rage_atstring_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_000671_rage_atstring_split_rage_atarray_rage_atstring_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000675_rage_functorbase_functorbase_void_const_ptr_void_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_000680_rage_athashstringnamespacesupport_computehash_rage_athashstringn(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_000681_rage_athashstringnamespacesupport_addstring_rage_athashstringnam(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_000684_rage_athashstringnamespacesupport_trygetstring_rage_athashstring(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_000696_rage_bkwidget_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000703_rage_bkbank_unsetcurrentgroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000704_rage_bkbank_pushgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000707_rage_bkbank_popgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000782_rage_bkmanager_findbank_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000783_rage_bkmanager_findwidget_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000786_rage_bkmanager_isbankthread(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000806_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000869_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000876_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_000877_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_000882_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000894_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000918_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001062_rage_matrix34_inverse_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001114_rage_datbase64_encode_unsigned_char_const_ptr_unsigned_int_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_001116_char_ptr_rage_formatf_char_32ul_char_ref_32ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001176_rage_datresourceinfo_generatemap_rage_datresourcemap_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001514_rage_diagdisplayf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001516_rage_fistream_write_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001523_quitf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001524_displayf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001527_warningf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001530_rage_fistream_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001532_rage_fistream_close(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001533_rage_fidevice_getdevice_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001534_quitf_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001535_rage_diaglogf2_rage_diagchannel_const_ref_rage_diagseverity_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001537_rage_fistream_size(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001551_rage_fprintf_rage_fistream_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001561_rage_diagcontextmessage_diagcontextmessage_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_001563_rage_diagcontextmessage_pop(ph: *mut Phantom); }
extern "C" { pub fn f_001565_printf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001569_rage_atmap_unsigned_int_wchar_t_ptr_rage_atmaphashfn_unsigned_in(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001577_rage_fistream_open_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001578_rage_fistream_seek_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001580_rage_fiassetmanager_open_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001589_mg_printf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001590_mg_write(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001622_rage_fistream_read_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001623_rage_fiassetmanager_findextensioninpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001627_rage_fiassetmanager_exists_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001632_rage_fiassetmanager_pushfolder_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001633_rage_fiassetmanager_popfolder(ph: *mut Phantom); }
extern "C" { pub fn f_001635_rage_fiassetmanager_filename_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001636_rage_fiassetmanager_basename_char_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001639_rage_fiassetmanager_removeextensionfrompath_char_ptr_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001652_rage_fidevice_makememoryfilename_char_ptr_int_void_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_001768_rage_fidevicerelative_init_char_const_ptr_bool_rage_fidevice_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_001769_rage_fidevicerelative_mountas_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001806_rage_fidevice_unmount_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001881_rage_fipackfile_init_char_const_ptr_bool_rage_fipackfile_cachemo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_001883_rage_fipackfile_mountas_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002027_rage_invsqrtf_fast_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_002037_rage_pgrscbuilder_constructname_char_ptr_int_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_002044_rage_pgrscbuilder_computeleafsize_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002080_rage_pgbase_pgbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002081_rage_pgbase_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002091_rage_datresource_containsthisaddress_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002092_rage_datresource_getfixup_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002105_rage_pgbase_dtorpgbase_3250(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002111_rage_parmanager_parmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002112_rage_parmanager_initialize_rage_parsettings_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002128_rage_parmanager_uninitialize(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002129_rage_atmapentry_rage_atnonfinalhashstring_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002130_rage_atmap_rage_atnonfinalhashstring_rage_atnonfinalhashstring_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002131_rage_atnonfinalhashstring_atnonfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002132_rage_parmanager_dtorparmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002156_rage_parmanager_initfromstructure_rage_parstructure_ref_rage_par(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002158_rage_parinstancevisitor_visittoplevelstructure_rage_parparsables(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002161_rage_parstructure_issubclassof_rage_parstructure_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002165_rage_parmanager_createandloadanytype_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
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
extern "C" { pub fn f_002593_rage_parsettings_createstandardsettings(ph: *mut Phantom); }
extern "C" { pub fn f_002695_rage_atbinarymap_rage_pardelegateholderbase_ptr_unsigned_int_dat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002696_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002697_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_002720_rage_conststring_vcopy_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002721_rage_conststring_dtorconststring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002739_rage_atpartialstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002799_rage_sysappcontent_isjapanesebuild(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002834_rage_sysparam_findcommandlinearg_char_const_ptr_char_ptr_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002839_rage_sysparam_get_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002840_rage_sysparam_get_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002849_rage_sysbootmanager_isbootedfromdisc(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002878_rage_atmap_unsigned_int_rage_sysstackcollector_tagentry_rage_atm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002916_rage_sysservice_updateclass(ph: *mut Phantom); }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_002945_rage_sysmempoolallocator_safeallocate_unsigned_long_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002946_rage_sysmempoolallocator_allocate_unsigned_long_unsigned_long_bo(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002947_rage_sysmempoolallocator_safefree_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002948_rage_sysmempoolallocator_free_void_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003075_rage_systinyheap_init_void_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003140_rage_geomtvalues_findtvalueopensegtopoint_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_003142_rage_geomtvalues_findtvaluesegtopoint_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_003143_rage_geomtvalues_findtvalueopensegtopointv_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_003151_rage_geomboxes_testsegmenttobox_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_003159_rage_matrix34_fromquaternion_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003160_rage_matrix34_toquaternion_rage_quaternion_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003161_rage_quaternion_frommatrix34_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003162_rage_matrix34_interpolate_rage_matrix34_const_ref_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_003170_rage_quaternion_slerp_float_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003172_rage_quaternion_twistangle_rage_vector3_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_003175_rage_vector3_vector3_rage_vector2_const_ref_rage_vector2_evector(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003176_rage_vector3_approachstraight_rage_vector3_const_ref_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_003179_rage_atpoolbase_init_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003185_rage_crframe_crframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003186_rage_crframe_crframe_rage_crframedata_const_ref_unsigned_char_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_003189_rage_crframe_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003194_rage_vec_v4arccos_float_vector_4(ph: *mut Phantom, a0: u128) -> u128; }
extern "C" { pub fn f_003198_rage_crframe_identityfromskel_rage_crskeletondata_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003211_rage_crframe_getsituation_unsigned_char_unsigned_char_unsigned_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_003214_rage_crframe_getvector3_unsigned_char_unsigned_short_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_003215_rage_crframe_setvector3_unsigned_char_unsigned_short_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_003216_rage_crframe_getmatrix3x3_unsigned_char_unsigned_short_rage_mat3(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_003217_rage_crframe_getfloat_unsigned_char_unsigned_short_float_ref_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_003219_rage_crframe_initcreateboneandmoverdofs_rage_crskeletondata_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_003221_rage_crframe_dtorcrframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003291_rage_cranimtolerancesimple_cranimtolerancesimple_float_float_flo(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_003306_rage_cranimloadercompress_cranimloadercompress(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003307_rage_cranimloadercompress_init_rage_cranimtolerance_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003313_rage_crframedata_crframedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003314_rage_crframedata_dtorcrframedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003315_rage_crframedata_init_unsigned_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_003352_rage_crweightset_setanimweight_int_float_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_003443_rage_crweightset_crweightset_rage_crskeletondata_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003444_rage_crweightset_dtorcrweightset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003738_rage_crcreaturecomponentphysical_resetmotions(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003798_rage_fwmvrequestid_op_rage_fwmvstateid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003800_rage_fwanimdirector_generatecomponentindices(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003806_rage_fwanimdirector_waitforanyactiveupdatetocomplete_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003809_rage_crframedata_adddof_unsigned_char_unsigned_short_unsigned_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_003828_rage_fwanimdirector_findpropertyinmovenetworks_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003829_rage_fwanimdirectorcomponentsyncedscene_issyncedscenelooped_shor(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003830_rage_crtagiteratorbase_rage_crtagiteratorkey_true_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003831_rage_fwanimdirectorcomponentsyncedscene_isplayingsyncedscene_con(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003832_rage_fwanimdirectorcomponentsyncedscene_isvalidsceneid_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003833_rage_fwanimdirectorcomponentsyncedscene_getsyncedscenetime_short(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_003834_rage_fwanimdirectorcomponentsyncedscene_getsyncedscenerate_short(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_003835_rage_fwanimdirector_getmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003838_rage_fwclipset_getclip_rage_fwmvtemplateid_rage_fwmvclipidtype_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_003839_rage_fwmovenetworkinterface_getclipsetid_rage_fwmvtemplateid_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003840_rage_fwclipsetmanager_getclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003841_rage_fwclipset_getclipdictionary_bool_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003848_rage_fwanimdirector_isnetworkdefloaded_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003850_rage_fwanimdirector_getcreature_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003852_rage_fwanimdirector_createnetworkplayer_rage_mvnetworkdef_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003853_rage_fwanimdirector_createcomponentsyncedscene(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003854_rage_fwanimdirector_requestupdatepostcamera(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003856_rage_fwanimdirector_waitforprephysicsupdatetocomplete(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003860_rage_fwanimdirectorcomponentragdoll_posefromragdoll_rage_crcreat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003865_rage_fwdictionaryloader_rage_pgdictionary_rage_crexpressions_rag(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003875_rage_fwanimhelpers_getmovertracktranslation_rage_crclip_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003876_rage_fwanimmanager_getclipifexistsbysetid_rage_fwmvtemplateid_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003878_rage_fwanimhelpers_getmovertracktranslationdiff_rage_crclip_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003879_rage_fwanimhelpers_getmovertracktranslationdiff_rage_fwmvtemplat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003880_rage_fwanimhelpers_getmovertrackrotation_rage_crclip_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003882_rage_fwanimhelpers_getmovertrackrotationdiff_rage_crclip_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003884_rage_fwanimhelpers_getmovertrackrotationdelta_rage_crclip_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003886_rage_fwanimhelpers_getmovertrackmatrixdelta_rage_crclip_const_re(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64); }
extern "C" { pub fn f_003887_rage_fwanimhelpers_getmovertrackdisplacement_rage_crclip_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003888_rage_fwanimhelpers_getmovertracktranslationdiffrotated_rage_crcl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_003890_rage_fwanimhelpers_getmovertrackvelocity_rage_crclip_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003891_rage_fwanimhelpers_getmovertrackvelocity_rage_fwmvtemplateid_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003892_rage_fwanimhelpers_applyinitialoffsetfromclip_rage_crclip_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_003893_rage_fwanimhelpers_applyinitialoffsetfromclip_rage_crclip_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003896_rage_fwanimmanager_getclipifexistsbydictindex_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_003900_rage_fwclipsetmanager_getclipdictionaryindex_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003904_rage_fwanimmanager_findslot_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003905_rage_fwanimmanager_findslotfromhashkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003906_rage_fwanimmanager_findhashkeyfromslot_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003908_rage_fwanimmanager_removerefwithoutdelete_rage_strlocalindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_003909_rage_fwanimmanager_getname_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_003911_rage_fwanimmanager_addref_rage_strlocalindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_003941_void_rage_atplaceptrcreator_void_ptr_rage_fwmemorygroupmetadata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003967_rage_fwclipset_getclipitem_rage_fwmvtemplateid_rage_fwmvclipidty(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003968_rage_fwclipset_getclipdictionaryindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003971_rage_fwclipset_getclipitemidbyindex_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_003977_rage_fwclipset_request_deprecated_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_003979_rage_fwclipset_isstreamedin_deprecated_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003980_rage_fwclipset_addref_deprecated(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003981_rage_fwclipset_release_deprecated(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003984_rage_fwclipsetrequesthelper_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003994_rage_fwclipsetmanager_getclipitem_rage_fwmvtemplateid_rage_fwmvc(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003995_rage_fwclipsetmanager_getclip_rage_fwmvtemplateid_rage_fwmvclips(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003996_rage_fwclipsetmanager_pickrandomclip_rage_fwmvtemplateid_rage_fw(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_004008_rage_fwclipsetmanager_streamin_deprecated_rage_fwmvtemplateid_ra(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004009_rage_fwclipsetmanager_request_deprecated_rage_fwmvtemplateid_rag(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004010_rage_fwclipsetmanager_isstreamedin_deprecated_rage_fwmvtemplatei(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004019_rage_fwclipsetrequesthelper_request_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_004037_rage_fwclipsetrequesthelper_dtorfwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004038_rage_fwclipsetrequesthelper_fwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004039_rage_fwclipsetrequesthelper_request_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004040_rage_fwclipsetrequesthelper_getclipset_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004041_rage_fwclipsetrequesthelper_isloaded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004042_rage_fwclipsetrequesthelper_isinvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004102_rage_fwmovenetworkinterface_getlasteventthisframe_rage_fwmvid_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_004283_rage_fwposematcher_findbestmatch_rage_crskeleton_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_004284_rage_fwposematcher_findsample_rage_fwmvtemplateid_rage_fwmvclips(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_004387_rage_fwanimdirectorcomponentsyncedscene_stopsyncedsceneplayback(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_004390_rage_fwanimdirectorcomponentsyncedscene_decsyncedsceneref_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004391_rage_fwanimdirectorcomponentsyncedscene_startsyncedsceneplayback(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: f32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_004392_rage_fwanimdirectorcomponentsyncedscene_addsyncedsceneref_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004393_rage_fwanimdirectorcomponentsyncedscene_getsyncedsceneduration_s(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004394_rage_fwanimdirectorcomponentsyncedscene_setsyncedsceneduration_s(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_004395_rage_fwanimdirectorcomponentsyncedscene_calcentitymatrix_short_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: f32); }
extern "C" { pub fn f_004398_rage_fwanimdirectorcomponentsyncedscene_getsyncedscenephase_shor(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004399_rage_fwanimdirectorcomponentsyncedscene_setsyncedscenerate_short(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_004401_rage_fwanimdirectorcomponentsyncedscene_setsyncedsceneorigin_sho(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_004402_rage_fwanimdirectorcomponentsyncedscene_setsyncedsceneorigin_sho(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_004403_rage_fwanimdirectorcomponentsyncedscene_getsyncedsceneorigin_sho(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_004404_rage_fwanimdirectorcomponentsyncedscene_issyncedscenepaused_shor(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004406_rage_fwanimdirectorcomponentsyncedscene_setsyncedscenelooped_sho(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_004407_rage_fwanimdirectorcomponentsyncedscene_issyncedsceneholdlastfra(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004408_rage_fwanimdirectorcomponentsyncedscene_setsyncedsceneholdlastfr(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_004409_rage_fwanimdirectorcomponentsyncedscene_issyncedsceneabort_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004410_rage_fwanimdirectorcomponentsyncedscene_setsyncedsceneabort_shor(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004416_rage_fwanimdirectorcomponentsyncedscene_startsynchronizedscene_r(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004633_rage_fwarchetypemanager_getarchetype_rage_atnonfinalhashstring_u(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004634_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004661_rage_fwdynamicentitycomponent_fwdynamicentitycomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004681_rage_fwentity_waitforanyactiveanimupdatetocomplete_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004683_rage_fwentity_getaabb_rage_spdaabb_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004686_rage_fwextensionlist_dtorfwextensionlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004694_rage_fwsceneupdate_addtosceneupdate_rage_fwentity_ref_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004725_rage_fwwaypointrecordingroute_uint8tombr_unsigned_char(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004727_rage_fwwaypointrecordingroute_uint8tohdg_unsigned_char(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004816_rage_fwvectormap_drawline_rage_vector3_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_004817_rage_fwvectormap_drawlinethick_rage_vector3_const_ref_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_004820_rage_fwvectormap_drawcircle_rage_vector3_const_ref_float_rage_co(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_004921_rage_dldrawcommandbuffer_registercommand_int_void_ptr_rage_dlcmd(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_004990_rage_dldrawlistmgr_setdrawlistinfo_int_char_const_ptr_unsigned_c(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_005538_rage_crframefilterdictionarystore_findframefilter_rage_fwmvtempl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005539_rage_crframefilterdictionarystore_findframefilternameslow_rage_c(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005561_sprintf_s_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005562_rage_fwassetstore_rage_pgdictionary_rage_grctexture_rage_fwtxdde(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005589_rage_fwpsostoreloader_unload_rage_fwpsostoreloadinstance_ref_boo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005648_rage_fwpsostoreloader_load_char_const_ptr_rage_fwpsostoreloadins(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_005651_rage_fwpsostoreloader_makesimplefileloadercore(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005683_rage_fwstaticboundsstore_storeimapgroupboundslist_rage_atnonfina(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005732_rage_fwinteriorlocation_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005768_rage_fwscancascadeshadowinfo_isshadowvisible_rage_spdaabb_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005833_rage_fwassetstore_rage_rmcdrawable_rage_fwdrawabledef_addslot_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005836_rage_fwassetstore_rage_pgdictionary_rage_rmcdrawable_rage_dwddef(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005839_rage_fwassetstore_rage_fragtype_rage_fwfragmentdef_addslot_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005917_rage_fwrefawarebaseimpl_rage_datbase_dtorfwrefawarebaseimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005918_rage_aitask_makeabortable_rage_aitask_abortpriority_rage_aievent(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_005919_rage_aitask_setstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005921_rage_aitask_findparenttaskoftype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005922_rage_aitask_findsubtaskoftype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005923_rage_aitaskstatetransitiontable_checktransitions_rage_fwentity_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_005924_rage_aitask_picknewstatetransition_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_005925_rage_aitask_periodicallycheckforstatetransitions_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_005926_rage_aitask_setnewtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005936_rage_aitaskmanager_spewalltasks_rage_aitaskspew_spewtype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005939_rage_aitaskstatetransitiontable_addtransition_int_int_float_long(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i64); }
extern "C" { pub fn f_005947_rage_aitasktree_getactiveleaftask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005949_rage_aitasktree_findtaskbytypeactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005950_rage_aitasktree_findtaskbytypewithpriority_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_005988_rage_cnavmesh_linesegsintersect2d_rage_vector3_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_006035_rage_atmapentry_unsigned_int_bool_const_ref_rage_atmap_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006063_rage_strstreamingmodulemgr_addmodule_rage_strstreamingmodule_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006065_rage_strpackfilemanager_getimagefilenamefromhandle_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006076_rage_strstreaminginfo_computephysicalsize_rage_strindex_bool_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006084_rage_strstreaminginfomanager_requestobject_rage_strindex_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006086_rage_strpackfilemanager_invalidateindividualfile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006121_rage_strstreamingmodulemgr_getmodulefromfileextension_char_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006124_rage_strstreamingengine_getobjectname_rage_strindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006156_rage_atarray_rage_strindex_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006159_rage_strstreaminginfo_computeoccupiedvirtualsize_rage_strindex_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006162_rage_strstreaminginfo_gethandle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006164_rage_strstreaminginfo_clearflag_rage_strindex_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006167_rage_strstreaminginfomanager_isobjectinimage_rage_strindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006175_rage_strstreaminginfomanager_clearrequiredflag_rage_strindex_uns(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006225_rage_atvarstring_atvarstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_006235_rage_strstreamingvisualize_setmarkertext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006238_rage_strstreamingloadermanager_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006254_rage_strrequest_request_rage_strlocalindex_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_006255_rage_strrequest_hasloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006256_rage_strrequest_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006257_rage_strrequest_clearrequiredflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006286_rage_strstreamingvisualize_registerentitytype_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_006287_rage_strstreamingvisualize_registerlodtype_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_006295_rage_strstreamingvisualize_registernamedflag_rage_strstreamingvi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_006412_rage_aitask_debug_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006717_rage_audsound_stopandforget_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006743_rage_audspeechsound_findnumvariationsforvoiceandcontext_unsigned(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_006944_rage_audmetadatamanager_getobjectname_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007036_rage_audmetadatamanager_getobject_verify_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007412_rage_audentity_createandplaysound_unsigned_int_rage_audsoundinit(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007419_rage_vfxlistitem_vfxlistitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007426_rage_vfxbrokenfraginfo_dtorvfxbrokenfraginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007467_rage_decalmanager_initshaders(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007528_rage_ptfxattach_attach_rage_ptxeffectinst_ptr_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_007559_rage_ptfxmanager_gettriggeredinst_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007562_rage_ptfxmanager_getregisteredinst_void_const_ptr_int_bool_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_007570_rage_ptfxwrapper_setcolourtint_rage_ptxeffectinst_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007659_rage_fwptrlistdoublelink_flush(ph: *mut Phantom); }
extern "C" { pub fn f_007676_rage_fwangle_limitradianangleforpitch_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_007677_rage_fwangle_lerp_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_007678_rage_fwangle_lerptowards_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_007694_rage_fwgeom_intersectsphereray_rage_spdsphere_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_007771_rage_netobjectmgrbase_getnetworkobject_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007803_rage_networklogutils_writelogevent_rage_netlogginginterface_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007995_rage_matrix34_toeulersxyz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008075_rage_netobject_islocalflagset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008076_rage_netobject_setlocalflag_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_008077_rage_netobject_isglobalflagset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008235_rage_netblenderlininterp_hasstoppedpredictingposition_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008321_rage_netsynctree_forcesendofnodedata_unsigned_int_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_008682_rage_fwbasepool_fwbasepool_int_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_008683_rage_fwconfigmanager_getsizeofpool_unsigned_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008686_rage_fwbasepool_getindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008688_rage_fwbasepool_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008689_rage_fwscriptguid_createguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008693_rage_fwscriptguid_getbasefromguid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_008713_rage_scripthandler_getscriptresource_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008870_rage_fwconfigmanager_dtorfwconfigmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008924_rage_fwtimer_update(ph: *mut Phantom); }
extern "C" { pub fn f_008931_bool_ref_rage_atmap_rage_strindex_bool_rage_atmaphashfn_rage_str(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_008953_rage_fwknownrefholder_fwknownrefholder_void_ptr_ptr_rage_fwknown(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_009031_rage_crproperty_copy_rage_crproperty_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009041_rage_crproperty_dtorcrproperty(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009048_rage_crproperty_getattribute_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009049_rage_crproperty_forallattributes_bool_ptr_rage_crpropertyattribu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009050_rage_crproperty_stringkey_calckey_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009210_rage_crskeletondata_convertboneindextoid_int_unsigned_short_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009224_rage_crskeleton_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009229_rage_crskeleton_transform_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009230_rage_crskeleton_inverseupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009233_rage_crskeleton_getglobalmtx_unsigned_int_rage_mat34v_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009234_rage_crskeleton_setglobalmtx_unsigned_int_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009283_rage_grmmodel_getgeometry_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009317_rage_grmshader_load_char_const_ptr_rage_fitokenizer_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_009454_rage_matrix34_makerotateto_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009528_rage_grceffect_create_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009529_rage_grcstateblock_createsamplerstate_rage_grcsamplerstatedesc_c(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009530_rage_grcstateblock_grcstateblockstore_rage_grcstateblock_grcblen(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009533_rage_grctexturefactory_createparams_createparams_unsigned_short(ph: *mut Phantom, a0: i64) -> i64; }
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
extern "C" { pub fn f_009614_rage_grcviewport_setcurrent_rage_grcviewport_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009615_rage_grceffect_lookupglobalvar_char_const_ptr_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009639_rage_grcviewport_getspherecullstatus_rage_vec4v_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009644_rage_grcviewport_screen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009649_rage_grcworldidentity(ph: *mut Phantom); }
extern "C" { pub fn f_009650_rage_grcbindtexture_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009651_rage_grcviewport_initclass(ph: *mut Phantom); }
extern "C" { pub fn f_009655_rage_grcfont_getstringwidthinternal_char_const_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009656_rage_grcfont_draw_float_float_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64); }
extern "C" { pub fn f_009672_rage_grcadaptermanagerd3d11_enumerate_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009673_rage_grcdevice_initadapterordinal(ph: *mut Phantom); }
extern "C" { pub fn f_009674_rage_grcadapterd3d11output_getclosestmode_rage_grcdisplaywindow(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009675_rage_grcstateblock_createdepthstencilstate_rage_grcdepthstencils(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009676_rage_grcdevice_blit_init(ph: *mut Phantom); }
extern "C" { pub fn f_009677_rage_fastquad_init(ph: *mut Phantom); }
extern "C" { pub fn f_009690_rage_grcdevice_changedevice_rage_grcdisplaywindow(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009698_rage_grcdraw2dtext_float_float_char_const_ptr_bool_float_float_r(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i32, a4: f32, a5: f32, a6: i64); }
extern "C" { pub fn f_009718_rage_grcbufferd3d11_initialise_unsigned_int_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32); }
extern "C" { pub fn f_009847_rage_grceffect_setshaderquality_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009857_rage_grcstateblock_grcstateblockstore_rage_grcstateblock_grcrast(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009858_rage_grcstateblock_createrasterizerstate_rage_grcrasterizerstate(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009859_rage_grceffect_setglobalfloatcommon_int_float_ptr_int_rage_grcef(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_009864_rage_grceffect_registertechniquegroup_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009867_rage_grcinstancedata_dtorgrcinstancedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009896_rage_grcgputimestamp_grcgputimestamp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009898_rage_grcstateblock_destroyblendstate_rage_grcstateblock_grcblend(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009899_rage_grcstateblock_destroyrasterizerstate_rage_grcstateblock_grc(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009900_rage_grcstateblock_destroydepthstencilstate_rage_grcstateblock_g(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009906_rage_monitorconfiguration_releasemonitors(ph: *mut Phantom); }
extern "C" { pub fn f_009909_rage_grcadaptermanager_getadapter_unsigned_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_009924_rage_grcadaptermanager_shutdownclass(ph: *mut Phantom); }
extern "C" { pub fn f_009925_rage_grcadapterd3d11output_getdesc_int_unsigned_int_rage_grcadap(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009926_rage_grcadapterd3d11output_getmode_rage_grcdisplaywindow_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009930_rage_grcadapterd3d11_getoutput_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009935_rage_grcadapterd3d11_getgpucount(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009948_rage_grcbufferd3d11_cleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009969_rage_grcbufferd3d11resource_cleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009992_rage_grcdevice_allocfence_unsigned_int(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_010021_rage_grcdevice_createocclusionquery_int(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_010026_rage_grcdevice_createconditionalquery(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_010036_rage_grcdevice_setadapterordinal_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010037_rage_grcdevice_setoutputmonitor_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010152_rage_dxdiag_getcardname_dxgi_adapter_desc_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010163_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010169_rage_grcdebugdraw_circle_rage_vec2v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010171_rage_grcdebugdraw_meter_rage_vec2v_const_ref_rage_vec2v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_010172_rage_grcdebugdraw_text_float_float_rage_color32_char_const_ptr_b(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i64, a4: i32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010173_rage_grcdebugdraw_metervalue_rage_vec2v_const_ref_rage_vec2v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010179_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010181_rage_grcdebugdraw_poly_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010183_rage_grcdebugdraw_cross_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010184_rage_grcdebugdraw_circle_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010185_rage_grcdebugdraw_arc_rage_vec3v_const_ref_float_rage_color32_ra(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i64, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010193_rage_grcdebugdraw_capsule_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010194_rage_grcdebugdraw_axis_rage_mat34v_const_ref_float_bool_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_010195_rage_grcdebugdraw_arrow_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_010216_rage_grcdebugdraw_text_float_float_rage_dd_epixelcoordspace_rage(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i32, a4: i64, a5: i32, a6: f32, a7: f32); }
extern "C" { pub fn f_010221_rage_grcdebugdraw_adddebugoutput_rage_color32_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_010224_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010225_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010459_rage_ioaddrounddeadzone_float_ref_float_ref_float_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> f32; }
extern "C" { pub fn f_010477_rage_iomouse_setcursorvisible_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010478_rage_iomouse_setabsoluteonly_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010500_rage_iovalue_isdown_float_rage_iovalue_readoptions_const_ref_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010502_rage_iovalue_historyhelddown_unsigned_int_float_float_rage_ioval(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_010503_rage_iovalue_isup_float_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010510_rage_iovalue_isreleasedafterhistoryhelddown_unsigned_int_float_r(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_010511_rage_iovalue_ispressed_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010512_rage_iovalue_historypressed_unsigned_int_unsigned_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_010514_rage_iovalue_isreleased_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010515_rage_iovalue_historyreleased_unsigned_int_unsigned_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_010518_rage_iovalue_getnorm_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010519_rage_iovalue_getunboundnorm_rage_iovalue_readoptions_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010521_rage_iovalue_getnorm01_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_011400_rage_cutsmanager_getconcatsectionfortime_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_011401_rage_cutsmanager_getsceneorientationmatrix_int_rage_matrix34_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_011458_rage_cutsmanager_isconcatsectionvalidforplayback_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011734_rage_fraginst_getarticulatedbody(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011748_rage_fraginst_getpartbroken_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011751_rage_fraginst_getcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011805_rage_fragcacheentry_lazyarticulatedinit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011964_rage_fragtype_getskeletondata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012021_rage_fragphysicslodgroup_getlodbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012183_rage_ptxeffectinst_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012201_rage_ptxeffectinst_setbasepos_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012203_rage_ptxeffectinst_setoffsetpos_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012205_rage_ptxeffectinst_setoffsetrot_rage_quatv_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013429_char_ptr_rage_safecatf_char_char_ptr_unsigned_long_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_013938_rage_netsocket_netsocket(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014067_rage_nettimesync_gettime_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014068_rage_nettimesync_hasstarted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014164_rage_nettransactor_isinitialized_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014318_rage_rlgamerinfo_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014940_rage_rlrostitleid_getenvironmentname_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_015770_rage_canslerp_rage_quatv_const_ref_rage_quatv_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015813_rage_phconstraintbase_getinstanceb(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015862_rage_phconstrainthalfspace_getworldposa(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015863_rage_phconstrainthalfspace_getworldposb(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015864_rage_phconstrainthalfspace_setworldnormal_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015882_rage_phconstraintmgr_insert_rage_phconstraintbase_params_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015890_rage_phconstraintmgr_remove_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015891_rage_phconstraintmgr_gettemporarypointer_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015966_rage_phsleep_reset_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015980_rage_phcollider_getlocalvelocityrigid_float_vector_4_const(ph: *mut Phantom, a0: i64, a1: i64, a2: u128); }
extern "C" { pub fn f_015996_rage_phsleep_init_rage_phcollider_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016011_rage_phsimulator_setlastinstancematrix_rage_phinst_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016017_rage_phsimulator_deactivateobject_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016029_rage_phsleep_blocksleepfinishing_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016044_rage_pharchetype_addincludeflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016045_rage_pharchetype_removeincludeflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016088_rage_phinst_setinstflag_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016107_rage_phshapebatch_phshapegroup_rage_phshapesweptsphere_allocate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016115_rage_phshapebatch_phshapegroup_rage_phshapeprobe_allocate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016164_rage_matrix34_dot_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016181_rage_phshapetest_rage_phshapebatch_phshapetest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016186_rage_phshapetest_rage_phshapebatch_testinlevel_rage_phinst_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_016191_rage_phshapebatch_dtorphshapebatch(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016192_rage_phshapebatch_setcullbox_rage_matrix34_const_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016193_rage_phshapebatch_setignorematerialflags_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016196_rage_phshapetest_rage_phshapebatch_initprobe_rage_phsegment_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_016198_rage_phshapetest_rage_phshapebatch_initsweptsphere_rage_phsegmen(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_016260_rage_pharticulatedbody_freeze(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016262_rage_pharticulatedbody_setbodyminimumstiffness_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016263_rage_pharticulatedbody_setstiffness_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016269_rage_pharticulatedbody_setdrivestate_rage_phjoint_drivestate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016312_rage_phboundcomposite_updatebvh_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016317_rage_phboundcomposite_calculatecompositeextents_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016333_rage_pharticulatedcollider_getlocalvelocityart_float_vector_4_in(ph: *mut Phantom, a0: i64, a1: i64, a2: u128, a3: i32); }
extern "C" { pub fn f_016367_rage_phbound_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016370_rage_phbound_setcgoffset_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016391_rage_phboundbox_phboundbox_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016436_rage_phboundcapsule_phboundcapsule(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016439_rage_phboundcapsule_setcapsulesize_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_016444_rage_phboundcapsule_getendpointa_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016445_rage_phboundcapsule_getendpointb_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016454_rage_phboundcomposite_phboundcomposite(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016455_rage_phboundcomposite_init_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016464_rage_phboundcomposite_setbound_int_rage_phbound_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016886_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectlocation(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016911_rage_phlevelnodetree_rage_phlooseoctreenode_findandaddoverlappin(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_017186_rage_crclip_composite_rage_crframe_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_017189_rage_crclip_calcblockpassed_float_float_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i32; }
extern "C" { pub fn f_017191_rage_crclip_allocateandload_char_const_ptr_rage_cranimloader_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_017249_rage_crexpressions_load_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_017256_rage_crposematcher_crposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017257_void_rage_datresource_place_rage_crpmpointcloud_ptr_rage_crpmpoi(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017258_rage_crposematcher_dtorcrposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017259_rage_crpmpointcloud_addframe_rage_crframe_const_ref_rage_crskele(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_017260_rage_atarray_rage_datowner_rage_crposematcher_matchsample_0_unsi(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017262_rage_crposematcher_addpointcloud_rage_crpmpointcloud_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_017264_rage_crposematcher_findbestmatch_rage_crskeleton_const_ref_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_017598_rage_crpmpointcloud_crpmpointcloud(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018169_rage_mvparameterbuffer_generatefreecapacity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018172_rage_mvnetworkdef_load_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_018177_rage_mvnetworkdef_mvnetworkdef(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018216_rage_mvparameterbuffer_iterator_begin_rage_mvparameterbuffer_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_018217_rage_mvparameterbuffer_iterator_next(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018916_statid_op_statid_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018984_rage_vector3_isclose_rage_vector3_const_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_019090_cwaypointrecording_increferencestorecording_cwaypointrecordingro(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019091_cwaypointrecording_decreferencestorecording_cwaypointrecordingro(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019101_cpopulationhelpers_onsystemcpuratingchanged(ph: *mut Phantom); }
extern "C" { pub fn f_019288_caitarget_caitarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019289_caitarget_caitarget_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019290_caitarget_setentity_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019292_caitarget_setentityandoffset_centity_const_ptr_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019293_caitarget_caitarget_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019294_caitarget_setposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019295_caitarget_caitarget_caitarget_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019296_caitarget_getisvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019297_caitarget_getposition_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019298_caitarget_setentityandoffsetunlimited_centity_const_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019299_caitarget_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019300_caitarget_op_caitarget_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019301_csyncedentity_serialise_rage_csyncdatabase_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019303_canimblackboard_additemtoblackboardandpedintelligenceifallowedto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_019307_cscenarioblockingareas_ispointinsideblockingareas_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_019308_cscenarioblockingareas_shouldscenarioquitfromblockingareas_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_019310_cexpensiveprocess_cexpensiveprocess_cexpensiveprocess_processtyp(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_019313_cexpensiveprocess_dtorcexpensiveprocess(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019314_centityscanner_scanforentitiesinrange_centity_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019324_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_cspatia(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_019325_cspatialarray_findinsphere_rage_vec3v_const_ref_float_cspatialar(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_019336_cobjectscanner_scanforentitiesinrange_centity_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019340_cexpensiveprocessdistributer_addprocess_cexpensiveprocess_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019369_cambientmodelset_getcontainsmodel_unsigned_int_cambientmodelvari(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_019371_cambientmodelset_getrandommodelindex_cambientmodelvariations_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_019387_cambientmodelsetmanager_getmodelsetindexfromhash_cambientmodelse(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_019416_cconditionalclipset_checkconditions_cscenariocondition_sscenario(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019418_cconditionalanims_getconditionalprobability_cscenariocondition_s(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_019419_cconditionalanims_checkconditions_cscenariocondition_sscenarioco(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_019420_cconditionalanims_getimmediateconditions_unsigned_int_cscenarioc(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_019421_cconditionalanims_checkclipset_rage_atarray_cconditionalclipset(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019424_cconditionalanimsgroup_checkformatchingconditionalanims_cscenari(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019451_ailogging_getbooleanasstring_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_019452_ailogging_getdynamicentityisclonestringsafe_cdynamicentity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019453_ailogging_getdynamicentitynamesafe_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019458_ailogging_stacktracelog_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019461_ailogging_getentitydescription_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019462_cailogmanager_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019464_cailogmanager_printdebuginfosforpedtolog_cped_ref_cvehicle_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019565_cpedbonetagconvertor_getboneindexfrombonetag_rage_crskeletondata(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_019569_cgtaanimmanager_findframefilter_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019608_cfacialdatacomponent_setfacialidlemode_cped_ptr_cfacialdatacompo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019609_cfacialdatacomponent_playfacialanim_cped_ptr_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019610_cmoveped_playoneshotfacialclip_rage_crclip_const_ptr_float_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_019611_cfacialdatacomponent_playpainfacialanim_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019612_cfacialdatacomponent_playdyingfacialanim_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019614_cfacialdatacomponent_requestfacialidleclip_cfacialdatacomponent(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019634_cmoveped_settasknetwork_rage_fwmovenetworkplayer_ptr_float_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_019635_cmoveped_setsecondarytasknetwork_rage_fwmovenetworkplayer_ptr_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_019646_cmoveped_setmotiontasknetwork_rage_fwmovenetworkplayer_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_019647_cmoveped_clearmotiontasknetwork_cmovenetworkhelper_ref_float(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019648_cmoveped_clearoneshotfacialblock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019652_cmoveped_settasknetwork_cmovenetworkhelper_ref_float_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_019653_cmoveped_cleartasknetwork_cmovenetworkhelper_ref_float_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_019654_cmoveped_setsecondarytasknetwork_cmovenetworkhelper_ref_float_un(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_019655_cmoveped_clearsecondarytasknetwork_cmovenetworkhelper_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_019656_cmoveped_setadditivenetwork_cmovenetworkhelper_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_019657_cmoveped_clearadditivenetwork_cmovenetworkhelper_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_019667_cmoveped_switchtoanimated_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_019670_cmoveped_resetblendhelpercallbacks(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019690_cmoveobject_setsecondarynetwork_rage_fwmovenetworkplayer_ptr_flo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019697_cmoveobject_setnetwork_rage_fwmovenetworkplayer_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_019701_cmoveobject_clearnetwork_cmovenetworkhelper_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_019728_cclipeventtags_cmoverfixupeventtag_shouldfixupx_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019729_cclipeventtags_cmoverfixupeventtag_shouldfixupy_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019732_cclipeventtags_cdistancemarkertag_getdistance_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_019735_cclipeventtags_cbranchgetuptag_gettrigger_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019736_cclipeventtags_cbranchgetuptag_getblendoutitemid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020090_canimviewer_blendoutcapture_cdynamicentity_const_ptr_cnmblendout(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_020170_rage_crtagiteratorbase_rage_crtagiteratorkey_true_getprev(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020344_canimscene_removeentityfromscene_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020636_ctaskscriptedanimation_setrate_float_ctaskscriptedanimation_epla(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_020637_ctaskscriptedanimation_getblendphaseid_ctaskscriptedanimation_ep(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_020638_ctaskscriptedanimation_getblendphaseoutid_ctaskscriptedanimation(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_020975_rage_atbinarymap_float_unsigned_long_datapair_ptr_std_2_partial(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_020976_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_020977_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_021143_audspeechaudioentity_setambientvoicename_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021209_audpolicescanner_reportplayercrime_rage_vector3_const_ref_ecrime(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_021222_audsuperconductor_sendconductormessage_conductors_conductormessa(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021359_auddamagestats_dtorauddamagestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021366_audspeechaudioentity_inflictpain_auddamagestats_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021375_audpedaudioentity_handleanimeventflag_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021385_audspeechaudioentity_getpedvoicegroup_bool_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_021387_audpedaudioentity_playringtone_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021388_audpedaudioentity_stopringtone(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021393_audpedaudioentity_isringtoneplaying(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021403_audspeechaudioentity_stopspeech(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021405_audspeechaudioentity_ispainplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021407_audspeechaudioentity_isambientspeechplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021409_audpedaudioentity_playmeleecombathit_unsigned_int_worldprobe_csh(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_021422_cpedmotiondata_getisrunning_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021607_audradiotrack_getnextbeat_float_ref_float_ref_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_021761_audspeechaudioentity_saywhensafe_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_021767_audscannermanager_triggerscriptedreport_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_021778_audspeechmanager_getvoiceforphoneconversation_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021779_audspeechmanager_getphoneconversationvariationfromvoicehash_unsi(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021794_audspeechaudioentity_triggersharkattacksound_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_021927_audspeechaudioentity_doescontextexistforthisped_char_const_ptr_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_021928_audspeechaudioentity_doescontextexistforthisped_unsigned_int_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_021929_audspeechaudioentity_setpedvoiceforcontext_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_021930_audspeechaudioentity_setpedvoiceforcontext_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_022023_rage_fwentity_getskeletondata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022046_audvehicleaudioentity_playhelddownhorn_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_022235_cpedinventory_getweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022381_void_creplayeventmanager_storeevent_cpacketsoundphonering_cpacke(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022480_audcaraudioentity_getcaraudiosettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022650_audcollisionaudioentity_getmaterialfromintersection_rage_phinter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022651_audcollisionaudioentity_getmaterialfromshapetestsafe_worldprobe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022658_cped_isdead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022666_auddooraudioentity_triggerdoorimpact_cped_const_ptr_rage_vec3v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_022738_auddooraudioentity_entitywantstoopendoor_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022739_auddooraudioentity_processcollisionsound_float_bool_centity_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023396_cpacketevent_cpacketevent_ereplaypacketid_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023846_creplaymgr_replayrecordsound_unsigned_int_rage_audsoundinitparam(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_023872_creplaymgr_shouldincreasefidelity(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023877_creplaymgr_recordobject_cobject_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023885_creplaymgr_recordwarpedentity_cdynamicentity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023886_creplaymgr_cancelrecordingwarpedonentity_cphysical_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023891_replaybuffermarkermgr_addmarker_unsigned_int_unsigned_int_marker(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_023906_creplaycontrol_isplayeroutofcontrol(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023930_creplayeventmanager_shouldproceedwithpacket_cpacketevent_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023998_rage_fwmodelid_fwmodelid_rage_fwmodelid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_024266_creplaymgrinternal_isawareofworldforrecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024295_cweapon_cancelcooktimer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026149_rage_iopadactuatordevice_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026167_caminterface_isdominantrenderedcamera_cambasecamera_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026170_caminterface_isdominantrenderedcameraanyfirstpersoncamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026172_caminterface_getplayercontrolcamheading_cped_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026173_camgameplaydirector_getthirdpersoncamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026174_camgameplaydirector_getfirstpersonshootercamera_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026175_caminterface_getplayercontrolcampitch_cped_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026176_caminterface_getplayercontrolcamaimframe(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026202_caminterface_isspherevisibleingameviewport_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_026205_cqueriableinterface_istaskcurrentlyrunning_int_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_026211_worldprobe_cshapetestfixedresults_unsigned_char_32_cshapetestfix(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026223_camfirstpersonshootercamera_getrelativeheading_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_026224_camfirstpersonshootercamera_getrelativepitch_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_026238_cambasecamera_shake_unsigned_int_float_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_026239_cambasecamera_shake_cambaseshakemetadata_const_ref_float_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_026245_camcinematicdirector_getfirstpersonvehiclecamera_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026248_rage_vector3_normalizesafe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026259_rage_fwentity_getskeleton_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026260_camcinematicmountedcamera_getcontrolhelper_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026279_rage_matrix34_toeulersyxz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026280_camcinematicanimatedcamera_computecamerapositionfromanim_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_026281_camcinematicanimatedcamera_computeiscamerapositionvalid_cped_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_026286_camgameplaydirector_getthirdpersonaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026318_cambasecamera_stopshaking_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026329_worldprobe_cshapetestresults_dtorcshapetestresults(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026457_camcontrolhelper_getviewmodeforcontext_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_026623_rage_matrix34_toeulers_rage_vector3_ref_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026629_camgameplaydirector_getfirstpersonaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026681_camgameplaydirector_computeweaponinfoforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026683_camgameplaydirector_shoulduse1stpersoncontrols_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026691_cpedintelligence_findtasksecondarybytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_026703_camgameplaydirector_getfirstpersonvehiclecamera_centity_const_pt(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026705_cped_getplayerresetflag_cplayerresetflags_playerresetflags_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026714_camgameplaydirector_getfirstpersonpedaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026715_camgameplaydirector_isfirstpersonshooteraiming_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026719_cweaponinfo_const_ptr_cweaponinfomanager_getinfo_cweaponinfo_rag(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026728_camgameplaydirector_disablefirstpersonthisupdate_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_026829_ccoverpoint_dtorccoverpoint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026839_camfirstpersonshootercamera_whensprintinguserightstick_cped_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026852_camfirstpersonshootercamera_getlowcoverheighttvalue_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_027093_rage_matrix34_fromeulers_rage_vector3_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_027251_rage_matrix34_rotateunitaxis_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_027304_camsyncedscenedirector_stopanimatingcamera_camsyncedscenedirecto(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_027305_camsyncedscenedirector_animatecamera_rage_atnonfinalhashstring_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_028260_cviewportmanager_computevendormsaa_rage_grctexturefactory_create(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_028262_cviewportmanager_pushviewport_cviewport_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_028263_cviewportmanager_popviewport(ph: *mut Phantom); }
extern "C" { pub fn f_029101_cdebug_setversionnumber(ph: *mut Phantom); }
extern "C" { pub fn f_029347_cutscenemanager_getanimatedmodelentityfromscenehandle_rage_atnon(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_029377_ctask_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029433_getorcreategroup_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029455_cdebugscene_tunewidgetdata_tunewidgetdata_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_029456_cdebugscene_queuetunewidget_cdebugscene_tunewidgetdata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029612_worldprobe_cshapetestdesc_setresultsstructure_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029613_worldprobe_cshapetestresults_freeintersections(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029866_cdebugscene_focusentities_get_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029920_cdebugdrawstore_cdebugdrawstore(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029921_cdebugdrawstore_cdebugdrawstore_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_029922_cdebugdrawstore_addaxis_rage_mat34v_const_ref_float_bool_unsigne(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_029924_cdebugdrawstore_addline_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_029927_cdebugdrawstore_addsphere_rage_vec3v_const_ref_float_rage_color3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_029928_cdebugdrawstore_addcapsule_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_029929_cdebugdrawstore_addtext_rage_vec3v_const_ref_int_int_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_029931_cdebugdrawstore_addobb_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_029932_cdebugdrawstore_addarrow_rage_vec3v_const_ref_rage_vec3v_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_029934_cdebugdrawstore_render(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029935_cdebugdrawstore_clearall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029964_rage_fwtransform_geta_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029965_rage_fwtransform_getb_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029966_rage_fwtransform_getc_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029967_cphysical_gethealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029969_cdebugscene_focusentities_isingroup_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030351_debugrenderingmgr_init(ph: *mut Phantom); }
extern "C" { pub fn f_030722_ceventgroupped_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030724_cevent_cancreateevent_unsigned_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_030727_ceventgroupped_removeinvalidevents_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_030728_ceventgroupped_gethighestpriorityevent_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030739_cpedintelligence_addevent_cevent_const_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030740_geteventscriptaigroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030741_geteventglobalgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030756_ceventhandlerhistory_getcurrentevent_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030758_ceventhandlerhistory_clearragdollresponse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030759_ceventresponse_cleareventresponse_ceventresponse_responsetasktyp(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030762_rage_fweventqueue_16_void_remove_rage_fwevent_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030763_ceventhandler_getcurrenteventtype_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030769_caitarget_dtorcaitarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030770_cshockingeventsmanager_add_ceventshocking_ref_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_030772_ceventshocking_dtorceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030809_cevent_communicateevent_cped_ptr_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_030810_cevent_communicateevent_cped_ptr_cevent_communicateeventinput_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_030814_cpeddamagecalculator_cpeddamagecalculator_centity_const_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_030815_cpeddamagecalculator_applydamageandcomputeresponse_cped_ptr_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32); }
extern "C" { pub fn f_030819_cped_isinjured_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030822_ceventrequesthelp_dtorceventrequesthelp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030826_cpedintelligence_gettargetting_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030827_cpeddamagecalculator_wasdamagefromentitybeforewantedlevelcleared(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_030828_ceventdamage_ceventdamage_centity_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_030829_ceventdamage_dtorceventdamage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030848_ceventdeath_ceventdeath_bool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_030851_ceventdeath_dtorceventdeath(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030854_ceventwrithe_ceventwrithe_cweapontarget_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_030855_ceventwrithe_dtorceventwrithe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030881_ceventclimbnavmeshonroute_ceventclimbnavmeshonroute_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: f32, a5: i32, a6: i64, a7: i32, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_030882_ceventclimbnavmeshonroute_dtorceventclimbnavmeshonroute(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030945_svehiclemissionparams_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030949_ceventdisturbance_ceventdisturbance_cped_ptr_cped_ptr_ceventdist(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_031040_ceventacquaintanceped_ceventacquaintanceped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031069_ceventgivepedtask_ceventgivepedtask_int_rage_aitask_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_031070_ceventgivepedtask_dtorceventgivepedtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031086_ceventinair_ceventinair_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031087_ceventinair_dtorceventinair(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031090_ceventmustleaveboat_ceventmustleaveboat_cboat_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031091_ceventmustleaveboat_dtorceventmustleaveboat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031097_ceventcallforcover_ceventcallforcover_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031098_ceventcallforcover_dtorceventcallforcover(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031125_ceventshouttargetposition_ceventshouttargetposition_cped_ptr_cpe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031126_ceventshoutblockinglos_dtorceventshoutblockinglos(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031129_ceventvehicleonfire_ceventvehicleonfire_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031130_ceventvehicleonfire_dtorceventvehicleonfire(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031158_ceventfirenearby_dtorceventfirenearby(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031165_ceventstaticcountreachedmax_ceventstaticcountreachedmax_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_031169_ceventswitch2nm_ceventswitch2nm_unsigned_int_rage_aitask_ptr_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_031170_ceventswitch2nm_dtorceventswitch2nm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031186_ctaskinvestigate_setinvestigationpos_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_031191_ceventdeadpedfound_ceventdeadpedfound_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_031218_ceventrespondedtothreat_ceventrespondedtothreat_cped_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031219_ceventrespondedtothreat_dtorceventrespondedtothreat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031224_ceventplayerdeath_ceventplayerdeath_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031235_ceventshocking_ceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031239_ceventshocking_getreactposition_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_031241_ceventshocking_canbesensedby_cped_const_ref_float_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_031242_ceventshocking_set_rage_vec3v_const_ref_centity_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_031364_ceventfootstepheard_ceventfootstepheard_cped_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_031400_svehiclemissionparams_settargetentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_031402_ceventgunshot_wasshotfiredwithintime_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_031430_ceventfriendlyfirenearmiss_isbulletinrange_cped_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_031606_ceventacquaintanceped_dtorceventacquaintanceped_54301(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031627_ceventdeadpedfound_dtorceventdeadpedfound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031630_ceventplayerdeath_dtorceventplayerdeath(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031638_ceventreactioninvestigatethreat_dtorceventreactioninvestigatethr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031651_ceventdisturbance_dtorceventdisturbance(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031667_rage_atqueue_rage_eventregdref_rage_fwevent_4_dtoratqueue(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031880_ceventsoundbase_dtorceventsoundbase_54685(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034880_rage_sfscaleformmanager_beginframe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034881_rage_sfscaleformmanager_endframe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035059_cpedinventory_getweaponammo_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_035072_cportaltracker_isallowedtorunininterior_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035082_cnewhud_isweaponwheelactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035083_cnewhud_isradiowheelactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035132_cphonemgr_horizontalactivate_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035133_cphonemgr_setlatestphoneinput_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035277_cminimap_createsonarblipandreportstealthnoise_cped_ptr_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32) -> f32; }
extern "C" { pub fn f_036036_cscaleformmgr_flushinvokelist(ph: *mut Phantom); }
extern "C" { pub fn f_036254_cpausemenu_isactive_pm_activeflags(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036358_rage_atarray_rage_grcdisplaywindow_0_unsigned_short_op_rage_atar(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_036365_rage_atarray_rage_grcdisplaywindow_0_unsigned_short_grow_unsigne(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036385_cpausemenu_devicereset(ph: *mut Phantom); }
extern "C" { pub fn f_037287_cscaleformmgr_prepareupdateallmoviesonrt(ph: *mut Phantom); }
extern "C" { pub fn f_037288_cscaleformmgr_ensureallmoviesupdatedonrt(ph: *mut Phantom); }
extern "C" { pub fn f_037289_cscaleformmgr_cleanupmoviesonrt(ph: *mut Phantom); }
extern "C" { pub fn f_037365_cscaleformcomplexobjectmgr_performalloutstandingactionsonrt(ph: *mut Phantom); }
extern "C" { pub fn f_037366_cscaleformcomplexobjectmgr_releaseanyflaggedobjects(ph: *mut Phantom); }
extern "C" { pub fn f_037659_cseatmanager_countpedsinseats_bool_bool_bool_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_037661_cmodelinfo_requestassets_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037662_cmodelinfo_haveassetsloaded_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037752_cgameconfig_cgameconfig(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037761_ccrime_reportcrime_ecrimetype_centity_ptr_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_037764_cwanted_workoutpolicepresence_rage_vec3v_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_037767_cwanted_setwantedlevelnodrop_rage_vector3_const_ref_int_int_setw(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_037769_cwanted_getwithinamnestytime_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037888_cseatmanager_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_037890_cincident_cincident_centity_ptr_rage_vector3_const_ref_cincident(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32) -> i64; }
extern "C" { pub fn f_037891_cincidentmanager_addincident_cincident_const_ref_bool_int_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_037901_cwanted_copsaresearching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037907_cwanted_triggercopseesweaponaudio_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_037909_cwanted_haspendingcrimesnotwitnessedbylaw_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037910_cwanted_reportwitnesssuccessful_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037912_cwanted_reportwitnessunsuccessful_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037949_cweather_gettemperature_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038058_cdispatchdata_findconditionalvehicleset_unsigned_int(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_038130_cmodelinfo_getbasemodelinfo_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038134_rage_vector3_normalizesaferet(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038193_cinjuryincident_ispedvalidforinjuryincident_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_038194_cinjuryincident_dtorcinjuryincident(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038222_corder_getisvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038258_cseatmanager_getlastpedsseatindex_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038265_cincidentmanager_clearincident_cincident_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038382_cpedintelligence_addtaskdefault_rage_aitask_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038464_cbaseikmanager_refreshkinematics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038467_cbaseikmanager_getsolver_ikmanagersolvertypes_flags_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038470_ctorsoiksolver_ctorsoiksolver_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_038490_cikmanager_resetallsolvers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038491_cikmanager_isactive_ikmanagersolvertypes_flags_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038495_cikmanager_lookat_unsigned_int_centity_const_ptr_int_eanimboneta(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32); }
extern "C" { pub fn f_038497_cikmanager_islooking(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038499_cikmanager_abortlookat_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038500_cikmanager_abortlookat_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038501_cikmanager_getlookatentity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038503_cikmanager_istargetwithinfov_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_038504_cikmanager_request_cikrequestarm_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038505_cikmanager_request_cikrequestbodylook_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038506_cikmanager_setabsolutearmiktarget_rage_criksolverarms_earms_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: f32, a5: f32, a6: f32, a7: f32); }
extern "C" { pub fn f_038507_cikmanager_setrelativearmiktarget_rage_criksolverarms_earms_cdyn(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32, a6: f32, a7: f32, a8: f32, a9: f32); }
extern "C" { pub fn f_038508_cikmanager_aimweapon_rage_criksolverarms_earms_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_038509_cikmanager_pointgunatposition_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_038511_cikmanager_gettorsosolverstatus(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038512_cikmanager_gettorsominyaw(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038513_cikmanager_gettorsomaxyaw(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038514_cikmanager_gettorsominpitch(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038515_cikmanager_gettorsomaxpitch(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038517_cikmanager_settorsominyaw_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_038518_cikmanager_settorsomaxyaw_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_038519_cikmanager_settorsominpitch_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_038520_cikmanager_settorsomaxpitch_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_038521_cikmanager_settorsooffsetyaw_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_038522_cikmanager_gettorsoyaw_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038523_cikmanager_gettorsopitch_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038526_cikmanager_getpelvisdeltazforcamera_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_038527_cikmanager_getdefaultlegikmode_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038528_cikmanager_request_cikrequestbodyreact_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038529_cikmanager_request_cikrequestbodyrecoil_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038532_cikmanager_addrootslopefixup_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_038534_cikmanager_setoverridelegikblendtime_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_038535_cikmanager_resetoverridelegikblendtime(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038536_cikmanager_getspringstrengthforstairs(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038537_cikrequestbodylook_cikrequestbodylook(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038538_cikrequestbodylook_cikrequestbodylook_centity_const_ptr_eanimbon(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_038539_cikrequestbodylook_settargetoffset_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038540_cikrequestarm_cikrequestarm_ikarm_centity_const_ptr_eanimbonetag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_038541_cikrequestarm_settargetoffset_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038542_cikrequestarm_setadditive_rage_quatv_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038543_cikrequestbodyreact_cikrequestbodyreact_rage_vec3v_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_038544_cikrequestbodyrecoil_cikrequestbodyrecoil_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038585_crootslopefixupiksolver_calculatepitchandrollfromgroundnormal_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_038633_cbasemodelinfo_get2deffect_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038634_cbasemodelinfo_getnum2deffectsnolights_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038733_centryexitpoint_getseat_seataccess_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_038734_centryexitpoint_getentrypointposition_centity_const_ptr_cped_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_038735_cmodelseatinfo_calculateentrysituation_centity_const_ptr_cped_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32, a7: i64); }
extern "C" { pub fn f_038739_cmodelseatinfo_getentrypointindexforseat_int_centity_const_ptr_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_038740_cmodelseatinfo_getboneindexfromseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038741_cmodelseatinfo_getboneindexfromentrypoint_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038747_cmodelseatinfo_evaluateseataccessfromallentrypoints_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64, a7: i32) -> i32; }
extern "C" { pub fn f_038751_cseatmanager_getpedsseatindex_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038757_cseatmanager_getlastpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038758_cseatmanager_countfreeseats_bool_bool_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_038836_cpedmodelinfo_personalitymovementmodes_findmovementmode_cped_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_038849_cpedmodelinfo_getrandommovementclipset_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_038850_cpedmodelinfo_getappropriatestrafeclipset_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_038851_cpedmodelinfo_isusingfemaleskeleton_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038863_cpedmodelinfo_getpersonalitybravery_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_039014_cvehiclemodelinfo_getentrypointinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039016_cvehiclemodelinfo_choosevehiclecolourfancy_cvehicle_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_039548_ccovertuningmanager_gettuningformodel_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039569_cdoor_getdooraudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039572_cdoor_settargetdoorratio_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_039573_cpathservergta_updatedynamicobject_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_039574_cpathservergta_shouldavoidobject_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039576_cdoor_getdooropenratio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_039583_cdoor_isdoorfullyopen_float_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039627_cobject_initanimlazy(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039630_cweapon_sfireparams_dtorsfireparams(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039639_cpathservergta_maybeadddynamicobject_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039645_cdoor_getdoorisatlimit_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_039704_cobject_printskeletondata(ph: *mut Phantom); }
extern "C" { pub fn f_039708_rage_mthrandom_reset_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039715_cobject_setweapon_cweapon_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039718_cobject_gettask_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039719_cobject_settask_rage_aitask_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_039720_cobject_addtoorremovefromprocesspostcameraarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039732_cobject_canbeusedtotakecoverbehind(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039762_cobjectpopulation_createobject_cobjectpopulation_createobjectinp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039877_cpathserver_isrequestresultready_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_039879_cpathserver_requestpath_rage_vector3_const_ref_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64) -> i32; }
extern "C" { pub fn f_039881_cpathserver_requestpath_rage_trequestpathstruct_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039882_cpathserver_getclosestpositionforped_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_039883_rage_cnavmeshtrackedobject_isuptodate_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_039884_rage_cnavmeshtrackedobject_islastnavmeshintersectionvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039885_cpathserver_addblockingobject_rage_vector3_const_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_039893_cpathserver_getnavmeshindexfromposition_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039897_cpathserver_requestclosestviafloodfill_rage_vector3_const_ref_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_039899_cpathserver_requesthasnearbypavementsearch_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_039901_cpathserver_isrequestresultready_unsigned_int_rage_epathserverre(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_039902_cpathserver_lockpathresult_unsigned_int_int_ptr_rage_vector3_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_039903_cpathserver_unlockpathresultandclear_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_039907_cpathserver_getclosestfloodfillresultandclear_unsigned_int_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_039913_cpathserver_cancelrequest_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_039988_rage_cnavmeshtrackedobject_queryloscheck_rage_vector3_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_039989_rage_cnavmeshtrackedobject_requestloscheck_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039999_cpathserver_updateblockingobject_unsigned_short_rage_vector3_con(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_040002_cpathserver_removeblockingobject_unsigned_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_040003_tdynobjboundsfuncs_getignorecomponentsforentity_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040043_cpathserverambientpedgen_isinpedgenblockedarea_rage_vector3_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040139_cpedgroupmembership_getmember_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_040157_cpedgroupmembership_countmembers_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040171_cpedgroupmembership_setleader_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040172_cpedgroupmembership_addfollower_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040174_cpedgroupmembership_ismember_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040177_cpedgroup_isplayergroup_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040182_cpedgroup_process(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040184_cpedgroup_finddistancetonearestmember_cped_ptr_ptr_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_040185_cpedgroup_giveeventtoallmembers_cevent_ref_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040189_cpedgroups_addgroup_epoptype_cgamescripthandler_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_040194_cpedgroups_getpedsgroup_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040205_carea_dtorcarea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040207_carea_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040208_carea_get_rage_vector3_ref_rage_vector3_ref_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_040209_carea_ispointinarea_rage_vector3_const_ref_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_040210_carea_set_rage_vector3_const_ref_rage_vector3_const_ref_float_ce(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32); }
extern "C" { pub fn f_040211_carea_setassphere_rage_vector3_const_ref_float_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_040212_carea_getcentreandmaxradius_rage_vector3_ref_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040213_carea_getcentre_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040214_carea_getmaxradius_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040215_cplayerassistedmovementcontrol_cplayerassistedmovementcontrol(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040217_cplayerassistedmovementcontrol_process_cped_ptr_rage_vector2(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040220_worldprobe_cshapetesthitpoint_gethitinst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040221_cplayerassistedmovementcontrol_scanforsnaptoroute_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040315_cactionmanager_targettypegetposdirfromentity_rage_vec3v_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: f32, a7: i64, a8: f32, a9: f32, a10: i64) -> i32; }
extern "C" { pub fn f_040317_choming_calculatehomingworldoffset_cped_ptr_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32, a8: i32); }
extern "C" { pub fn f_040318_cstrikeboneset_getstrikebonetag_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_040319_cstrikeboneset_getstrikeboneradius_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_040320_cactionbranchset_getbranchactionid_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_040321_cactionbranchset_getbranchfiltertype_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040323_cactionmanager_findactionresult_unsigned_int_ref_unsigned_int_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_040325_cactionmanager_playeraifiltertypedoescatchped_rage_atfixedbitset(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040328_worldprobe_cshapetestresults_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040329_cplayerresetflags_isflagset_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_040330_cactionmanager_arepedsfriendlywitheachother_cped_const_ptr_cped(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040332_cimpulse_getimpulsename_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_040333_cimpulsecombo_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040334_cimpulsecombo_cimpulsecombo_csimpleimpulsetest_impulse(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_040335_cimpulsecombo_recordfirstimpulse_csimpleimpulsetest_impulse(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_040336_cimpulsecombo_getimpulseatindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_040337_csimpleimpulsetest_getiovaluesforimpulse_ccontrol_const_ptr_csim(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040338_cimpulsecombo_haspendingimpulse_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040339_cimpulsecombo_getimpulseptratindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_040343_cactiondefinition_getactionresult_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_040344_cactiondefinition_issuitable_rage_atfixedbitset_12_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32, a8: i64, a9: i32, a10: i32, a11: i64, a12: i64, a13: i32) -> i32; }
extern "C" { pub fn f_040345_cactionmanager_calculatedesiredheading_cped_const_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_040348_cactionmanager_findactiondefinition_unsigned_int_ref_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_040349_cactionmanager_shoulddebugped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040350_cactionmanager_selectsuitableaction_rage_atfixedbitset_12_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i64, a8: i32, a9: i32, a10: i32) -> i64; }
extern "C" { pub fn f_040351_cactionmanager_selectactionfromset_rage_atfixedarray_cactiondefi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_040354_cactionmanager_isactionsuitable_unsigned_int_rage_atfixedbitset(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_040356_cactionmanager_calculatepredictedtargetposition_cped_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_040369_cbrawlingstylemanager_getbrawlingdata_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_040385_ccombatinfomgr_getcombatinfobyhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_040386_ccombatbehaviour_getcombattype_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040398_cdefensivearea_cdefensivearea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040402_cdefensiveareamanager_getsecondarydefensivearea_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_040403_cdefensiveareamanager_getdefensiveareaforcoversearch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040404_rage_fraginstnm_postartmessage_char_const_ptr_art_messageparamsb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040409_art_messageparamsbase_addbool_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040422_rage_fraginstnm_switchfromnmtorageragdoll_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_040500_art_messageparamsbase_addint_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040503_art_messageparamsbase_messageparamsbase_art_messageparamsbase_pa(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_040998_cpednavcapabilityinfo_buildpathstyleflags_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041056_findplayerped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041060_cpeddebugvisualisermenu_getfocusped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041062_cpedintelligence_findmovementtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041063_cpedintelligence_isfriendlywith_cped_const_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041064_cpedintelligence_isthreatenedby_cped_const_ref_bool_bool_bool_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_041065_cplayerpedtargeting_tunables_gettargettinginfo_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041071_cpedintelligence_getclosestvehicleinrange_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041072_centityboundai_computecorners_centity_const_ref_float_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32, a7: i64, a8: f32); }
extern "C" { pub fn f_041073_centityboundai_getcorners_rage_vector3_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041075_cpedtargetting_cpedtargetting_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041076_cpedtargetting_getcurrenttarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041077_cpedtargetting_getlosstatus_centity_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041079_centityboundai_computehitsidebyposition_rage_vector3_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041080_centityboundai_computeboundingboxplanes_rage_vector3_ptr_float_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_041083_cpedintelligence_getcombatorders_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041084_cpedtargetting_gettargetatindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041085_cpedtargetting_findtarget_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041087_cplayerinfo_getpreviousvariationinfo_unsigned_int_ref_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_041088_cpedintelligence_getnearbyvehicles(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041092_cpedperception_computefovvisibility_centity_const_ptr_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_041116_cpedintelligence_flushimmediately_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041117_cpedintelligence_addtoeventhistory_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041121_cpedintelligence_buildqueriablestate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041195_cpedintelligence_cleartasks_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041202_cpedintelligence_setrelationshipgroup_crelationshipgroup_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041203_cpedintelligence_getrelationshipgroup_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041208_cvehiclepotentialcollisionscanner_scan_cped_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041209_centityboundai_centityboundai_centity_const_ref_float_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32, a7: i64, a8: f32) -> i64; }
extern "C" { pub fn f_041211_centityboundai_testlineofsightreturndistance_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_041219_cpedgeometryanalyser_canpedtargetpedasync_cped_ref_cped_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: i32, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_041221_cpedintelligence_canattackped_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041222_cpedtargetting_registerthreat_centity_const_ptr_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041223_cpedtargetting_aretargetswhereaboutsknown_rage_vector3_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_041227_cpedgeometryanalyser_canpedtargetpoint_cped_const_ref_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i64, a7: i64, a8: i64) -> i32; }
extern "C" { pub fn f_041234_cnearbyfirescanner_scan_cped_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041237_cpedintelligence_gettaskmelee_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041238_cpedintelligence_gettaskwrithe_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041239_cpedintelligence_gettaskcombat_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041244_cplayerspecialabilitymanager_chargeevent_abilitychargeeventtype(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_041252_ceventscanner_scanforeventsnow_cped_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041257_cped_setplayerresetflag_cplayerresetflags_playerresetflags(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041258_cpedintelligence_clearprimarytask(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041261_cplayerinfo_ishardaiming(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041263_rage_atbinarymap_ctasknmsimple_tunables_tuning_rage_atnonfinalha(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041264_cpedintelligence_setcheckshockflag_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041265_cplayerinfo_isaiming_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041269_cpedintelligence_gettaskclimbladder_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041288_cpedintelligence_getactivepropmanagementhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041298_cpedhelmetcomponent_disablehelmet_bool_rage_vector3_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_041299_cpedhelmetcomponent_restoreoldhat_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041311_cpedfootstephelper_resetairflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041316_incpedailoscounter(ph: *mut Phantom); }
extern "C" { pub fn f_041317_centityboundai_init_centity_const_ref_float_const_ref_float_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: f32); }
extern "C" { pub fn f_041318_centityboundai_projectboundingboxontozplane_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32); }
extern "C" { pub fn f_041321_centityboundai_liesinside_rage_vector3_const_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_041322_centityboundai_testlineofsight_rage_vec3v_const_ref_rage_vec3v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_041323_cpedgroutecalculator_computerouteouttoedgeofbound_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: f32); }
extern "C" { pub fn f_041324_doesrouteseghitentity_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_041325_createclockwiseoranticlockwiseroute_cped_const_ptr_centity_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i64, a8: i32, a9: i32, a10: i64, a11: i64, a12: i32) -> i32; }
extern "C" { pub fn f_041326_cpedgeometryanalyser_testcapsuleagainstcompositer_rage_phinst_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i32, a7: i64, a8: i32, a9: i64) -> i64; }
extern "C" { pub fn f_041330_cpedintelligence_getnearbyobjects(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041331_cpedgeometryanalyser_testpedcapsule_cped_const_ptr_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64, a10: f32, a11: f32, a12: f32, a13: i64, a14: f32) -> i32; }
extern "C" { pub fn f_041337_cpedhelmetcomponent_ishelmetenabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041339_cpedhelmetcomponent_enablehelmet_eanchorpoints_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_041347_cpedhelmetcomponent_sethelmetpropindex_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041350_cpedhelmetcomponent_iswearinghelmetwithgoggles(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041351_cpedhelmetcomponent_requestpreloadprop_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041352_cpedhelmetcomponent_haspreloadedprop(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041353_cpedhelmetcomponent_clearpreloadedprop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041371_cpediksettingsinfo_isikflagdisabled_ikmanagersolvertypes_flags_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041374_rage_fwregdref_cscenariopoint_rage_fwrefawarebasenonvirt_dtorfwr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041375_ceventresponse_dtorceventresponse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041377_cpedintelligence_setdecisionmakerid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041380_cpedtargetting_getbesttarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041381_cpedintelligence_getcurrenttarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041382_cpedintelligence_registerthreat_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041383_cpedintelligence_gettimesincelastaimedatbyplayer_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041384_cpedintelligence_getcoverfinder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041385_cpedintelligence_findtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041386_cpedintelligence_findtaskcombat_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041387_cpedintelligence_gettaskmeleeactionresult_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041388_cpedintelligence_ispedinair_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041389_cpedintelligence_ispedclimbing_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041395_cpedintelligence_ispedclimbingladder_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041396_cpedintelligence_ispedinmelee_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041397_cpedintelligence_clearsecondarytask_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041398_cpedintelligence_clearprimarytaskatpriority_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041406_cpedintelligence_isfriendlywithbyanymeans_cped_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041409_cpedintelligence_countfriends_float_bool_bool_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_041410_cpedintelligence_recordfailedclimbonroute_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041411_cpedintelligence_recordfinishedcrossingroad(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041412_cpedintelligence_shouldresumecrossingroad_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041414_cpedintelligence_releasecombatdirector(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041415_cpedintelligence_requestcombatdirector(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041418_cpedintelligence_createclonetaskfortasktype_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041419_cpedintelligence_createclonetaskfrominfo_ctaskinfo_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041422_cpedailodmanager_shoulddofullupdate_cped_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041425_cpedintelligence_isbattleawareforcingrun_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041426_cpedintelligence_isbattleaware_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041430_cpedintelligence_canbepinneddown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041434_cpedintelligence_decreasealertnessstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041437_cpedintelligence_setstartnewhangoutscenario_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041444_cpedintelligence_getlastaudiosaidtoped_cped_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041445_cpedintelligence_hasrecentlytalkedtoped_cped_const_ref_float_con(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_041449_cpedintelligence_getcurrenttintindexforparachute_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041451_cpedintelligence_eventhistoryoneventremoved_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041454_cpedmotiondata_setcurrentmoveblendratio_rage_vector2_ref_float_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_041455_cpedmotiondata_getisdesiredsprinting_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041457_cpedphonecomponent_getphonemodelindexsafe_cpedphonecomponent_con(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041458_cpedphonecomponent_getphonemodelhashsafe_cpedphonecomponent_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041460_cpedplacement_findzcoorforped_float_rage_vector3_ptr_centity_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_041467_cpedpopulation_choosecivilianpedmodelindexargs_choosecivilianped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041509_rage_fwregdref_cscenariopoint_rage_fwrefawarebasenonvirt_reset_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041539_cpedpopulation_choosecivilianpedmodelindex_cpedpopulation_choose(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041541_cpedpopulation_schedulepedinvehicle_cvehicle_ptr_int_bool_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64) -> i32; }
extern "C" { pub fn f_041545_cpedpopulation_iseffectinuse_cscenariopoint_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041589_cexplosionmanager_cexplosionargs_dtorcexplosionargs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041599_cplayerspecialability_addtometer_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041609_cplayerspecialabilitymanager_processcontrol_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041628_cpedtype_findpedtype_rage_atnonfinalhashstring_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041651_cplayerinfo_getrunsprintspeedmultiplier_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041655_cplayerinfo_getfallheightforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041656_cplayerinfo_getdiveheightforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041664_cplayerinfo_issprintaimbreakoutover_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041668_cplayerpedtargeting_findlockontarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041669_cplayerpedtargeting_findmeleetarget_cped_const_ptr_cweaponinfo_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64) -> i64; }
extern "C" { pub fn f_041674_cplayerpedtargeting_clearlockontarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041679_cplayerinfo_isfiring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041681_cplayerinfo_controlbuttonsprint_cplayerinfo_esprinttype_bool(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_041682_cpedweaponmanager_getequippedweaponhasfirstpersonscope_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041683_cplayerinfo_processcansprint_cped_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041684_cplayerinfo_processsprintcontrol_ccontrol_const_ref_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> f32; }
extern "C" { pub fn f_041689_cplayerinfo_canusecover_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041694_cplayerinfo_scanforvehicletoenter_cped_ptr_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_041695_cplayerinfo_scanforanimaltoride_cped_ptr_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_041696_cplayerinfo_hasplayerlefttheworld_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041701_canimspeedups_shouldusempanimrates(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041713_cplayerinfo_controlbuttonduck(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041716_cplayerinfo_isfiring_s(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041719_cplayerinfo_getcachedmeleeinputs_bool_ref_bool_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041720_ccontrol_getvehicletargetdown_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_041721_cplayerinfo_iscombatrolling(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041731_cplayerinfo_havenumenemieschargedwithinqueryperiod_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_041732_cplayerinfo_havenumenemiesthrownsmokewithinqueryperiod_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_041733_cplayerinfo_reportenemystartedsmokethrow(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041734_cplayerpedtargeting_gettarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041736_cplayerinfo_clearpreviousvariationdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041737_cplayerinfo_setpreviousvariationdata_epedvarcomp_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_041740_cplayerinfo_isplayerstatevalidforfpsmanipulation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041746_reportplayerresetmismatch_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_041809_cpopzones_findsmallestforposition_rage_vector3_const_ptr_ezoneca(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_041838_cpedperception_computefovvisibility_rage_vector3_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041839_cpedperception_isinseeingrange_centity_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041870_rage_fwgeom_fwline_findclosestpointonline_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_041877_cpedtargetevaluator_findtarget_cped_const_ptr_int_float_float_ce(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: i64, a5: i64, a6: i64, a7: f32, a8: f32, a9: f32, a10: f32, a11: f32) -> i64; }
extern "C" { pub fn f_041886_cpedtargetting_gettargetinfoatindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041887_cpedtargetting_getlosstatusatindex_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041888_cpedtargetting_findtargetindex_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041889_cpedtargetting_gettargetlastseenpos_centity_const_ptr_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_041891_cpedtargetting_removethreat_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041892_cpedtargetting_setinuse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041893_cpedtargetting_getsecondbesttarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041919_cplayerpedtargeting_findlockonmeleetarget_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_041927_cplayerpedtargeting_setlockontarget_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041935_cplayerpedtargeting_clearsoftaim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041984_cserialisedfsmtaskinfo_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041990_ctaskinfo_ctaskinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041993_cclonedtaskinfoweapontargethelper_cclonedtaskinfoweapontargethel(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041994_cclonedtaskinfoweapontargethelper_readqueriablestate_caitarget_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041995_cclonedtaskinfoweapontargethelper_updatetarget_caitarget_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041998_cqueriableinterface_gettargetfortasktype_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041999_cqueriableinterface_updatecachedinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042003_cqueriableinterface_gettaskinfofortasktype_int_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042004_cqueriableinterface_getstatefortasktype_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042005_cqueriableinterface_getrunningscenariotype_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042040_crelationshipmanager_findrelationshipgroup_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_042043_cvehiclecombatavoidancearea_ispointinareas_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042149_cpeddamagemanager_addpeddamagedecal_cped_ptr_epeddamagezones_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: f32, a6: i64, a7: i32, a8: i32, a9: f32, a10: f32, a11: i32); }
extern "C" { pub fn f_042179_cpeddamagemanager_addpeddamagepack_cped_ptr_rage_atnonfinalhashs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_042199_cpeddamagemanager_getplayerdamageset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042436_cpedvariationpack_getpalettevar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042437_cpedvariationpack_gettexvar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042446_cpedpropsmgr_setpedprop_cped_ptr_eanchorpoints_int_int_eanchorpo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_042489_cpedpropsmgr_getpedpropidx_cped_const_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042532_cpedvariationdebug_createpedcb(ph: *mut Phantom); }
extern "C" { pub fn f_042618_cpedpropsmgr_knockoffprops_cped_ptr_bool_bool_bool_bool_rage_vec(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_042629_cpedpropsmgr_ispedpropdelayeddelete_cped_const_ptr_eanchorpoints(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042630_cpedpropsmgr_getpedpropactualanchorpoint_cped_ptr_eanchorpoints(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042639_cpedpropsmgr_clearpedprop_cped_ptr_eanchorpoints_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042648_cpedpropsmgr_getnumactiveprops_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042814_worldprobe_debugfindgroundzfor3dcoord_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> f32; }
extern "C" { pub fn f_042818_worldprobe_cshapetestmanager_debugsubmittest_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_042821_worldprobe_cshapetestmanager_aborttest_worldprobe_cshapetestresu(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042843_cped_getgroundpos_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042844_cwatertesthelper_getwaterheightatpositionincludingrivers_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_042848_worldprobe_cshapetestdesc_setexcludeentities_centity_const_ptr_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_042885_cpickupmanager_createpickup_unsigned_int_rage_matrix34_const_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_042888_cpickupmanager_createpickupfromcurrentweapon_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042890_cpickupmanager_shouldusemppickups_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042914_cpickupmanager_cancreatepickupifunarmed_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043046_ccollisionhistory_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043049_ccollisionhistory_getmostsignificantcollisionrecordoftype_unsign(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043074_ctunableobjectmanager_gettuningentry_rage_atnonfinalhashstring_c(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043075_rage_matrix34_transform_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043078_cwatertesthelper_getwaterlevelincludingriversnowaves_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_043085_worldprobe_cshapetestdesc_setexcludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043153_fraginstnmgta_applybulletforce_cped_ptr_float_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_043195_phmaterialmgrgta_getpackedpolyflagvalue_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043211_cphysics_breakglass_cdynamicentity_ptr_rage_vec3v_const_ref_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: f32, a5: i32) -> i32; }
extern "C" { pub fn f_043234_worldprobe_cshapetestdesc_setincludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043242_cphysics_getmeasuringtoolpos_int_rage_vector3_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_043286_cflightmodelhelper_processflightmodel_cphysical_ptr_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: i64, a6: i32, a7: f32, a8: f32, a9: i32); }
extern "C" { pub fn f_043287_cflightmodelhelper_applyforcecgsafe_rage_vector3_const_ref_cphys(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043288_cflightmodelhelper_applytorquesafe_rage_vector3_const_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_043289_cflightmodelhelper_makecircularinputsquare_rage_vector2_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043333_cwatertesthelper_getcachedriverboundproberesult_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_043334_cwatertesthelper_getwaterlevelincludingrivers_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_043343_worldprobe_cshapetestbatchdesc_addcapsuletest_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043344_worldprobe_cshapetestbatchdesc_setcapsuledescriptors_worldprobe(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043349_worldprobe_cshapetestdesc_setexcludeinstance_rage_phinst_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043617_postfx_initmsaashaders(ph: *mut Phantom); }
extern "C" { pub fn f_043794_deferredlighting_initshaders(ph: *mut Phantom); }
extern "C" { pub fn f_043795_deferredlighting_initrenderstates(ph: *mut Phantom); }
extern "C" { pub fn f_043797_ctiledlighting_setupshader(ph: *mut Phantom); }
extern "C" { pub fn f_043802_ssao_initshaders(ph: *mut Phantom); }
extern "C" { pub fn f_043814_crendertargets_updatebank_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043942_videoresmanager_getscenescale_int_int_ref_int_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_044002_ssao_init_hdaoshaders(ph: *mut Phantom); }
extern "C" { pub fn f_044004_ssao_init_hdao2shaders(ph: *mut Phantom); }
extern "C" { pub fn f_044122_water_getwaterlevel_rage_vector3_const_ref_float_ptr_bool_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_044123_water_getwaterlevelnowaves_rage_vector3_const_ref_float_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_044644_ccsmglobals_setsoftshadowproperties(ph: *mut Phantom); }
extern "C" { pub fn f_044647_ccsmglobals_initshaders_rage_grmshader_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044678_crenderphasecascadeshadowsinterface_script_setaircraftmode_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_045785_cped_computewandertask_cped_const_ref_cpopcycleconditions_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045786_cped_computedefaultdrivingtask_cped_const_ref_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_045788_cped_getpedhealthinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045789_cped_ismale_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045791_cped_getattackstrength_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_045793_cped_checkbraveryflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045794_cped_checksexinessflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045795_cped_checkagilityflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045797_cped_getcurrentmotiontask_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045804_cped_clearactivateragdolloncollisionevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045808_cped_isstairs_worldprobe_cshapetesthitpoint_const_ptr_rage_phins(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045813_cped_destroyjetpack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045815_cped_releasecoverpoint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045819_cped_setactivateragdolloncollision_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045821_cped_printskeletondata(ph: *mut Phantom); }
extern "C" { pub fn f_045824_cped_createbank(ph: *mut Phantom); }
extern "C" { pub fn f_045834_cped_switchtoanimated_bool_bool_bool_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_045839_cped_switchtostaticframe_float_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045840_cped_getbonematrix_rage_matrix34_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045842_cped_forcemotionstatethisframe_cpedmotionstates_emotionstate_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_045847_cped_removerotationalconstraint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045852_cped_resetdesiredmainmovercapsuledata_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045873_cped_getclimbphysical_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045874_cped_isonground_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045876_cped_setuseextractedz_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045882_cped_switchtoragdollinternal(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045883_cped_setpedoutofvehicle_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_045885_cped_switchtoragdoll_cevent_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045886_cped_setmyvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045887_rage_fwentity_setnocollision_rage_fwentity_ptr_unsigned_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_045889_cped_getprimarymotiontask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045891_cped_applyragdollblockingflags_eragdollblockingflagsbitset_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045892_cped_clearragdollblockingflags_eragdollblockingflagsbitset_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045893_cped_getragdollblockingflags(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045903_cped_getiscrouching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045904_cped_setiscrouching_bool_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_045905_cped_canpedstandup(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045906_cped_canpedcrouch_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045907_cped_canpedstealth_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045908_cped_isplayingagestureanim_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045909_cped_getprimarymotiontaskifexists_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045910_cped_isallowedtoforcemotionstate_cpedmotionstates_emotionstate_c(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_045911_cped_wantstouseactionmode_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045912_cped_isusingactionmode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045913_cped_isfirstpersonshootermodeenabledforplayer_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045922_cped_getboneposition_rage_vector3_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045926_cped_getisdrivingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045927_cped_getheldobject_cdynamicentity_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045940_cped_setvariation_epedvarcomp_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_045961_cped_finalizeheadblend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045967_cped_setremoveassoonaspossible_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045968_cped_gettaskdata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045970_cped_getscenariotype_cdynamicentity_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045971_cped_getscenariopoint_cdynamicentity_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045974_cped_getloadingobjectindex_cdynamicentity_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045976_cped_setisstrafing_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045977_cped_setdesiredheading_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_045978_cped_setdesiredheading_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045979_cped_getfacingdirectionheading_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_045980_cped_getvelocityheading_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045981_cped_isinmotion_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045982_cped_stopallmotion_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045985_cped_validategroundphysical_rage_phinst_const_ref_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_045996_cped_getbonetagfromragdollcomponent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046001_cped_isusingstealthmode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046002_cped_isinfirstpersonvehiclecamera_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_046005_cped_isfireresistant_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046006_cped_computematrixfromheadingandpitch_rage_mat34v_ref_float_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_046010_cped_getcanusehighdetailwaterlevel_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046012_cped_setclothpinradiusscale_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046019_cped_setrenderdelayflag_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046024_cped_updatemovementmode_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046026_cped_getvehiclepedentering_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046039_cped_releasedesiredcoverpoint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046044_cped_processbounds_float_bool(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_046045_cped_setclothisskydiving_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046046_cped_isnearmirror_rage_scalarv_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046047_rage_matrix34_fromeulersxyz_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046048_cped_getisfpsswimming_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046049_cped_createdeadpedpickups(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046052_cped_issecurityped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046054_cped_setusingactionmode_bool_cped_actionmodeenabled_float_bool_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_046055_cped_setmovementmodeoverridehash_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046056_cped_canbedeleted_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_046073_cped_processlefthandgripik_bool_bool_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_046086_cped_resetstanddata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046088_cped_restoreheadingchangerate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046091_cped_debugattachpedtophysical_char_const_ptr_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i64, a8: i64, a9: f32, a10: f32); }
extern "C" { pub fn f_046096_cped_getbonepositioncachedv_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046099_cped_getragdollcomponentmatrix_rage_matrix34_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_046102_cped_getisenteringvehicle_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046103_cped_getspecialability_eplayerspecialabilityslot(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046104_cped_isexitingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046105_cped_getblockingofnontemporaryeventsthisframe_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046106_cped_forcedeath_bool_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046107_cped_setdeathstate_edeathstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046109_cped_setarreststate_earreststate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046112_cped_canbeinhurt_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046113_cped_candohurttransition_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046114_cped_willpedenterwater_float_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046116_cped_estimatepose_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046117_cped_orientboundtospine_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_046118_cped_clearbound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046120_cped_getisfpsswimmingunderwater_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046121_cped_gethasjetpackequipped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046127_cped_setincustody_bool_cped_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_046131_cped_isgoonped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046134_cped_iscivilianped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046135_cped_isgangped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046136_cped_shouldbehavelikelaw_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046141_cped_isbeingjackedfromvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046142_cped_canshoveped_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046143_cped_setlastpedshoved_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046144_cped_newsay_char_const_ptr_unsigned_int_bool_bool_int_cped_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32) -> i32; }
extern "C" { pub fn f_046145_cped_newsaywithparams_char_const_ptr_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_046146_cped_randomlysay_char_const_ptr_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_046147_cped_newsay_unsigned_int_unsigned_int_bool_bool_int_cped_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_046148_cped_randomlysay_unsigned_int_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_046150_cclipeventtags_ctagiteratorattributevalue_rage_crtag_rage_crprop(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_046151_csimpleblendhelper_getphase_ccliphelper_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_046152_cped_setpedinvehicle_cvehicle_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_046153_cped_getpedradiocategory_int_ref_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046157_cped_setcoverpoint_ccoverpoint_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046158_cped_setdesiredcoverpoint_ccoverpoint_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046159_cped_computelowcoverheightoffsetfrommover_float_float_float_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32) -> f32; }
extern "C" { pub fn f_046162_cped_setclothforcepin_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046169_cped_willattackinjuredpeds_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046170_cped_isallowedtodamageentity_cweaponinfo_const_ptr_centity_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_046172_cped_getwaterlevelonped_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_046175_cped_setdesiredvelocityclamped_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_046176_cped_getcontrolfromplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046183_cped_getragdollcomponentfrombonetag_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046184_cped_getiksettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046192_cped_isdebuginvincible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046207_cpedvfx_getbloodvfxdisabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046210_cped_isbeingforcedtorun_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046211_cped_setforcedrundelay_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046214_cped_getmovementmodeweaponhash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046215_cped_hasstreamedstealthmodeclips_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046218_cped_isstreamingactionmodeclips_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046219_cped_getmovementmodeclipset_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046220_cped_getmovementmodeidletransitionclipset_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046221_cped_isbattleeventblockingactionmodeidle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046227_cped_getconstrainthelperdofs_bool_float_ref_rage_vec3v_ref_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_046231_cped_isfirstpersonshootermodestickwithinstrafeanglethreshold_con(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046232_cped_usefirstpersonupperbodyanims_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046291_canimatedbuilding_printskeletondata(ph: *mut Phantom); }
extern "C" { pub fn f_046325_cgameworld_findfollowplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_046326_cgameworldheightmap_getmaxheightfromworldheightmap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_046327_cplayerswitchinterface_isactive_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_046362_cdatafilemgr_expandfilename_char_const_ptr_char_ptr_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046410_centity_getinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046427_cdynamicentity_removesceneupdateflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046430_cdynamicentity_inverseupdateskeleton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046438_cgameworld_add_centity_ptr_rage_fwinteriorlocation_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046445_cportaltracker_collectstack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046473_centity_getisinexterior_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046484_centity_setisvisibleformodule_eisvisiblemodule_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046490_centity_isvisible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046491_centity_getisonscreen_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046494_centity_isreducedbbforpathfind_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046495_centity_isinpathserver_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046496_centity_wasunabletoaddtopathserver(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046499_centity_getcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046502_centity_getphysarch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046531_centity_getisvisibleformodule_eisvisiblemodule_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046545_centity_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046599_centityiterator_centityiterator_int_centity_const_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_046600_centityiterator_getnext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047020_cgameworld_findlocalplayerwanted(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047044_cgameworld_forallentitiesintersecting_rage_fwsearchvolume_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_047054_cphysical_setisinwater_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047060_cphysical_activatephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047066_cphysical_getreferenceframevelocity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047072_cphysical_setmass_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047073_cphysical_sethealth_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047076_cphysical_getweapondamagedtimebyentity_centity_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_047078_cphysical_getweapondamageentitypedsafe_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047079_cphysical_hasbeendamagedbyentity_centity_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_047084_cphysical_clearallcachedobjectcoverobstructed(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047096_cphysical_applyforcecg_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_047100_cphysical_applyimpulse_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047101_cphysical_applyimpulsecg_rage_vector3_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047103_cphysical_applyangimpulse_rage_vector3_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047126_cphysical_debugprocessallattachments_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_047299_ctexlod_storehdmapping_rage_estoretype_unsigned_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_047305_cgameworld_getmainplayerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047417_cmanagedimapgroup_init_cmapdatagroup_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047418_rage_atarray_cmanagedimapgroup_0_unsigned_short_grow_unsigned_in(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047725_cinteriorinst_issubwaymlo_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047727_cinteriorinst_isallowrunmlo_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047880_cgameworld_findlocalplayercoors(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047929_cscenestreamermgr_dumpstreaminginterests_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047998_cgameworld_findlocalplayerspeed(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_048037_cgameworldheightmap_getminheightfromworldheightmap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_050110_ctaskscriptedanimation_cprioritydata_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_050115_cclipeventtags_ctagiteratorattributevalue_rage_crtag_rage_crprop(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_054887_cped_isfatallyinjured_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_060585_ctaskmoveseekentity_213_centityseekposcalculatorstandard_ccloned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: i32) -> i64; }
extern "C" { pub fn f_060626_ctaskscriptedanimation_initslotdata_initslotdata_ctaskscriptedan(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_060628_ctaskscriptedanimation_setclip_rage_crclip_const_ptr_ctaskscript(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_060630_ctaskscriptedanimation_getblendweightid_ctaskscriptedanimation_e(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_060662_ctaskmoveseekentity_217_centityseekposcalculatorradiusangleoffse(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: i32) -> i64; }
extern "C" { pub fn f_060664_ctaskmoveseekentity_215_centityseekposcalculatorxyoffsetrotated(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: i32) -> i64; }
extern "C" { pub fn f_060665_ctaskmoveseekentity_216_centityseekposcalculatorxyoffsetfixed_cc(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: i32) -> i64; }
extern "C" { pub fn f_060782_ccliphelper_isplayingclip_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_060817_ctaskmovescripted_setclipset_rage_fwmvtemplateid_rage_fwmvclipse(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_063681_ctaskmoveseekentity_213_centityseekposcalculatorstandard_ccloned(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063682_ctaskmoveseekentity_213_centityseekposcalculatorstandard_ccloned(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063800_cthescripts_getcurrentscriptnameandprogramcounter(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063825_cscripteulers_quaternionfromeulers_rage_quaternion_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063888_cboatisbeachedcheck_isboatbeached_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064041_citemset_clean(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064588_ccustomshadereffecttint_selecttintpalette_unsigned_char_rage_fwe(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064698_cshaderlib_setglobalalpha_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_064700_cshaderlib_setglobalemissivescale_float_bool(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_064725_cshaderlib_setstereotexture(ph: *mut Phantom); }
extern "C" { pub fn f_064736_csettingsmanager_csettingsmanager(ph: *mut Phantom); }
extern "C" { pub fn f_064776_cpopulationstreaming_requestvehicledriver_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064800_cstreaming_hasobjectloaded_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064813_cpopulationstreaming_isfallbackpedavailable_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064846_cprioritizedclipsetstreamer_findrequest_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_064857_cprioritizedclipsetrequestmanager_isloaded_rage_fwmvtemplateid_r(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064860_cprioritizedclipsetrequestmanager_request_cprioritizedclipsetreq(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_064862_cprioritizedclipsetrequestmanager_hasclipsetbeenrequested_cprior(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_064863_cprioritizedclipsetrequestmanager_getprioritizedclipsetrequesthe(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_064907_rage_czonedassetmanager_loadfile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064908_rage_czonedassetmanager_enumandprocessfolder_char_const_ptr_void(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064909_cstreaming_shouldloadstaticdata(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_064911_cstreaming_getreloadpackfilesonrestart(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_064924_cstreamingdebug_displaymodulememory_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_064926_ckeyboard_getalteredkeyboardmode_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064927_ckeyboard_debugoutputscriptcallstack(ph: *mut Phantom); }
extern "C" { pub fn f_064945_cstreamingcleanup_makespaceformap_rage_datresourcemap_ref_rage_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_065125_cstreamingrequestlist_registerfromfile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065329_rage_atmap_rage_atfinalhashstring_rage_controlinput_gamepad_defi(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065332_ccontrol_getpedsprintisdown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065334_ccontrol_getpedsprintispressed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065335_ccontrol_getpedsprintisreleased_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065336_ccontrol_getpedsprinthistoryhelddown_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_065352_ccontrol_cleartogglerun(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065374_ccontrol_setinputexclusive_int_rage_iovalue_disableoptions_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_065380_ccontrol_inputhowlongago_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065384_ccontrol_isinputdisabled_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_065385_ccontrol_getvalue_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_065446_ccontrolmgr_getmainplayercontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065469_ccontrolmgr_startplayerpadshakebyintensity_unsigned_int_float_in(ph: *mut Phantom, a0: i32, a1: f32, a2: i32); }
extern "C" { pub fn f_065567_rage_atfixedarray_rage_controlinput_dynamicmapping_19_dtoratfixe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065599_settingsdefaults_settingsdefaults(ph: *mut Phantom); }
extern "C" { pub fn f_065613_cfilemgr_mountaudiopacks_char_const_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_065641_settingsdefaults_rendertargetmemsizefor_settings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065648_cgraphicssettings_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_065651_csystemsettings_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_065654_caudiosettings_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_065657_cvideosettings_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_065660_settings_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_065667_csettingsmanager_initializeresolutionlists(ph: *mut Phantom); }
extern "C" { pub fn f_065668_csettingsmanager_requestnewsettings_settings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065670_csettingsmanager_applyresetsettings(ph: *mut Phantom); }
extern "C" { pub fn f_065671_csettingsmanager_initresolutionindex(ph: *mut Phantom); }
extern "C" { pub fn f_065672_csettingsmanager_applysettings(ph: *mut Phantom); }
extern "C" { pub fn f_065673_csettingsmanager_getresolutionlist_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065674_csettingsmanager_applyvideosettings_cvideosettings_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_065676_csettingsmanager_load(ph: *mut Phantom); }
extern "C" { pub fn f_065677_csettingsmanager_generatedefaultsettings(ph: *mut Phantom); }
extern "C" { pub fn f_065678_csettingsmanager_save_settings_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065689_csettingsmanager_applyragsettings_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_065691_csystem_beginrender(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065693_csystem_endrender(ph: *mut Phantom); }
extern "C" { pub fn f_065758_rage_pgdictionary_gtadrawable_dtorpgdictionary(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065766_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_065767_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_065768_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_065772_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_065778_rage_atmap_rage_atnonfinalhashstring_rage_controlinput_dynamicma(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065789_ctaskclip_restorephysicsstate_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065792_ctaskclip_applyphysicsstatechanges_cphysical_ptr_rage_crclip_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_065793_ctaskrunclip_ctaskrunclip_rage_fwmvtemplateid_rage_fwmvclipsetid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_065794_ctaskrunclip_ctaskrunclip_int_unsigned_int_float_int_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32) -> i64; }
extern "C" { pub fn f_065798_ctaskrunclip_start_onupdate_cphysical_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065799_ctaskrunclip_startclip_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065800_ctaskrunclip_playingclip_onupdate_cphysical_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065801_csimpleblendhelper_setrate_ccliphelper_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_065802_ccliphelper_setphase_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_065809_ctaskrunnamedclip_ctaskrunnamedclip_int_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: f32, a7: i32, a8: i32, a9: f32) -> i64; }
extern "C" { pub fn f_065813_ctaskrunnamedclip_playingclip_onenter_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065814_ctaskrunnamedclip_playingclip_onupdate_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065815_ctaskrunnamedclip_playingclip_onexit_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065820_ctaskrunnamedclip_computeposeerror_float_rage_fraginst_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64) -> f32; }
extern "C" { pub fn f_065821_cclonedscriptclipinfo_cclonedscriptclipinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065829_cmoveparameterwidgetclip_dtorcmoveparameterwidgetclip(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065831_cmoveparameterwidgetclip_buildclipmap_rage_atmap_rage_athashvalu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_065833_cmoveparameterwidgetclip_addvariableclipsetwidgets_rage_bkbank_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065834_cmoveparameterwidgetclip_onvariableclipsetchanged(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065835_cmoveparameterwidgetclip_addabsoluteclipsetwidgets_rage_bkbank_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065836_cmoveparameterwidgetclip_onabsoluteclipsetchanged(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065837_cmoveparameterwidgetclip_addclipdictionarywidgets_rage_bkbank_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065838_cmoveparameterwidgetclip_onclipdictionarychanged(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065839_cmoveparameterwidgetclip_addfilepathwidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065840_cmoveparameterwidgetclip_onfilepathchanged(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065852_cmoveparameterwidgetexpression_addexpressiondictionarywidgets_ra(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065853_cmoveparameterwidgetexpression_onexpressiondictionarychanged(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065854_cmoveparameterwidgetexpression_addfilepathwidgets_rage_bkbank_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065855_cmoveparameterwidgetexpression_onfilepathchanged(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065866_cmoveparameterwidgetfilter_onfilepathchanged(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065875_ctasksynchronizedscene_ctasksynchronizedscene_short_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32, a5: f32, a6: i64, a7: i64, a8: f32, a9: i32, a10: i64) -> i64; }
extern "C" { pub fn f_065876_ctasksynchronizedscene_dtorctasksynchronizedscene(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065879_ctasksynchronizedscene_interpolatemover(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065883_ctaskscriptedanimation_processikcontrolflags_cped_ref_eikcontrol(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065888_cped_setactivateragdolloncollisionevent_cevent_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065889_networkinterface_getnetworktime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065898_ctaskcutscene_ctaskcutscene_rage_crclip_const_ptr_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_065903_ctaskcutscene_setblendouttag_rage_crclip_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065913_ctaskmovescripted_setinitparams_ctaskmovescripted_scriptinitialp(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065914_ctaskmovescripted_dtorctaskmovescripted(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065916_ctaskmovescripted_requeststatetransition_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065919_ctaskmovescripted_setsignalfloat_unsigned_int_float_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_065921_ctaskmovescripted_lerpfloat_ref_rage_atmap_unsigned_int_ctaskmov(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_065922_ctaskmovescripted_setsignalbool_unsigned_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_065924_rage_atmapentry_unsigned_int_ctaskmovescripted_lerpfloat_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_065943_ctaskreacharm_ctaskreacharm_rage_criksolverarms_earms_cdynamicen(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: f32, a8: f32) -> i64; }
extern "C" { pub fn f_065947_ctaskscriptedanimation_cprioritydata_cprioritydata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065948_ctaskscriptedanimation_cpriorityslot_setinitdata_ctaskscriptedan(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065950_ctaskscriptedanimation_ctaskscriptedanimation_rage_atnonfinalhas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32, a6: f32, a7: i32, a8: i64, a9: f32, a10: i32, a11: i32, a12: i64, a13: i32, a14: i32) -> i64; }
extern "C" { pub fn f_065951_ctaskscriptedanimation_ctaskscriptedanimation_rage_atnonfinalhas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32, a6: f32, a7: i32, a8: i64, a9: f32, a10: i64, a11: i64, a12: i32, a13: i32, a14: i64, a15: i32, a16: i32) -> i64; }
extern "C" { pub fn f_065952_ctaskscriptedanimation_dtorctaskscriptedanimation(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065956_ctaskscriptedanimation_applymovertrackfixup_cphysical_ptr_ctasks(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_065959_ctaskscriptedanimation_processfirstpersonik_eikcontrolflagsbitse(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065961_ctaskscriptedanimation_requestcloneclips(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065962_ctaskscriptedanimation_start_onupdate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065963_ctaskscriptedanimation_running_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065964_ctaskscriptedanimation_running_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065965_ctaskscriptedanimation_running_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065966_cpedintelligence_addtaskmovement_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065967_ctaskscriptedanimation_startplayback_ctaskscriptedanimation_init(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_065968_ctaskscriptedanimation_settaskstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_065969_ctaskscriptedanimation_shouldplayfpsclip(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065970_ctaskscriptedanimation_processcommon_ctaskscriptedanimation_epla(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_065988_covervariationsinfo_initcovervariations_rage_atarray_rage_fwmvte(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_065989_ccombatdirector_dtorccombatdirector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065991_ccombatdirector_removeped_cped_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_065993_ccombatdirector_addped_cped_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_065995_ctaskthreatresponse_setfleefromcombat_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065999_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_ccombat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_066004_ccombattaskmanager_countpedsincombatwithtarget_cped_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_066005_ccombattaskmanager_getpedincombatwithplayer_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066007_ctaskmelee_setqueuetype_ctaskmelee_queuetype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_066009_ccombatmeleegroup_remove_ccombatmeleeopponent_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066010_ctaskmelee_ctaskmelee_cactionresult_const_ptr_centity_ptr_rage_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_066020_ccombatsituationfallback_doestargethavescaryweapon_ccombatdirect(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066021_ccombatsituationlull_isvalid_ccombatdirector_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066053_ctacticalanalysisnavmeshpoints_releasehandle_cped_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_066054_ctacticalanalysisnavmeshpoints_findpointforhandle_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_066055_ctacticalanalysisnavmeshpoints_reservehandle_cped_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_066057_ctacticalanalysisnavmeshpoints_setpointsfromscript(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066059_ctacticalanalysisnavmeshpoints_resetpoint_ctacticalanalysisnavme(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066060_ctacticalanalysisnavmeshpoints_setpoint_ctacticalanalysisnavmesh(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066062_ccoverpoint_getcoverpointposition_rage_vector3_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066063_ccover_findcoordinatescoverpointnotreserved_ccoverpoint_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_066066_ccover_removecoverpoint_ccoverpoint_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066067_ccoverpoint_getcoverdirectionvector_rage_vec3v_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066068_ccoverpoint_reservecoverpointforped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066070_ctacticalanalysiscoverpoints_findpointforcoverpoint_ccoverpoint(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066071_ctacticalanalysiscoverpoints_reservecoverpoint_cped_ptr_ccoverpo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066073_ctacticalanalysis_dtorctacticalanalysis(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066075_ctacticalanalysis_find_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066077_ctacticalanalysis_request_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066079_ccover_calculatevantagepointforcoverpoint_ccoverpoint_ptr_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_066082_ctaskanimatedhitbyexplosion_ctaskanimatedhitbyexplosion_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_066089_ctaskdyingdead_setdeathanimationbyset_rage_fwmvtemplateid_rage_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_066090_ctaskcombatseekcover_ctaskcombatseekcover_cped_const_ptr_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_066091_ctaskcombatseekcover_dtorctaskcombatseekcover(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066095_ctaskcombatseekcover_checkforfinish(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066096_ctaskcover_ctaskcover_cweapontarget_const_ref_int_ctaskcover_efo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_066097_ctaskcombatadditionaltask_ctaskcombatadditionaltask_int_cped_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32) -> i64; }
extern "C" { pub fn f_066099_ctaskseparate_needstoseparate_cped_ptr_cped_const_ptr_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_066100_ctaskseparate_ctaskseparate_rage_vector3_const_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_066101_ctaskcombat_choosemovementattributes_cped_ptr_bool_ref_float_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066102_ctaskcombat_generatecombatrunningflags_bool_float(ph: *mut Phantom, a0: i32, a1: f32) -> i32; }
extern "C" { pub fn f_066103_ctaskcover_setsubtasktousewhilstsearching_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066105_ctaskcombatchargesubtask_ctaskcombatchargesubtask_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066106_ctaskcombatchargesubtask_dtorctaskcombatchargesubtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066109_ctaskmelee_settargetentity_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_066110_ctaskmelee_canchangetarget_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066114_carea_carea_carea_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066119_ctaskstandguard_ctaskstandguard_rage_vector3_const_ref_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32, a5: f32) -> i64; }
extern "C" { pub fn f_066130_ctaskstandguard_setupnewstandingpoint_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066132_ccover_findcoverpointinarc_cped_const_ptr_rage_vector3_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32, a7: i64, a8: i64, a9: i64, a10: i32, a11: i32, a12: i64, a13: i32, a14: i64, a15: i64, a16: i64) -> i32; }
extern "C" { pub fn f_066133_ccoverpointfilterbase_ccoverpointfilterbase_cped_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066134_ccover_checkfortargetsinvehicle_ccoverpoint_ptr_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066135_ccover_doescoverpointprovidecoverfromtargets_cped_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_066137_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_ccover(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_066139_ctaskseparate_dtorctaskseparate(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066145_ctaskseparate_createfleepathrequest_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_066146_ccover_findcoordinatescoverpoint_ccoverpoint_ptr_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_066151_ctaskcombatclosesttargetinarea_dtorctaskcombatclosesttargetinare(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066156_ctaskcombatclosesttargetinarea_searchfornewtargets_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066157_ctaskthreatresponse_ctaskthreatresponse_cped_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_066162_ctaskcombatadditionaltask_dtorctaskcombatadditionaltask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066166_ctaskcombatadditionaltask_hasclearlos(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066167_ctaskcombatadditionaltask_getdesiredtaskstate_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066168_ctaskcombatadditionaltask_processstate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066169_ctaskinvestigate_shouldpedclearcorner_cped_const_ptr_ctaskmovefo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_066173_ctaskcombatflank_ctaskcombatflank_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066174_ctaskcombatflank_dtorctaskcombatflank(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066187_ctaskhelicopterstrafe_ctaskhelicopterstrafe_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066188_ctaskhelicopterstrafe_dtorctaskhelicopterstrafe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066193_ctaskmelee_shouldcheckformeleeambientmove_cped_ptr_bool_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_066194_cnewhud_isshowinghudmenu(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_066196_ctaskmelee_checkforandgetmeleeambientmove_cped_const_ptr_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_066202_ctaskmelee_cancelpendinglosrequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066203_ctaskmeleeactionresult_shouldbranchoncurrentframe_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_066204_ctaskmeleeactionresult_gethoming_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066205_ctaskmelee_resettaunttimer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066207_ctaskmelee_processunreachablenavmeshtest_cped_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066208_ctaskmeleeactionresult_getcurrentcliptime_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_066212_ctaskmelee_start_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066213_ctaskmelee_start_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066214_ctaskmelee_waitformeleeactiondecide_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066215_ctaskmelee_waitformeleeactiondecide_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066216_ctaskmelee_waitformeleeaction_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066217_ctaskmelee_inactiveobserveridle_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066218_ctaskmelee_meleeaction_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066219_ctaskmelee_meleeaction_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066220_ctaskmelee_meleeaction_onexit_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066221_ctaskmelee_manageactionsubtaskandbranches_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066222_ctaskmelee_updateblocking_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_066223_ctaskmelee_updateblockingtargetevaluation_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066224_ctaskmeleeactionresult_ctaskmeleeactionresult_cactionresult_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: f32) -> i64; }
extern "C" { pub fn f_066226_ctaskmelee_setupmeleegetup_cnmblendoutsetlist_ref_cped_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066231_ctaskmelee_getmeleegripfilter_rage_fwmvtemplateid_rage_fwmvclips(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066238_ctaskmeleeactionresult_findmeleenetworkdamageforhitped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066239_ctaskmeleeactionresult_cacheactionresult_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066241_ctaskmeleeactionresult_processcachednetworkdamage_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066242_ctaskmeleeactionresult_processselfdamage_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_066243_ctaskmeleeactionresult_canclonequitforcedreaction_cped_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066246_ctaskmeleeactionresult_processprehitresults_cped_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066252_ctaskmeleeactionresult_processposthitresults_cped_ptr_cped_ptr_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32); }
extern "C" { pub fn f_066253_worldprobe_cshapetesthitpoint_gethitentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066254_rage_crtagiteratorbase_rage_crtagiteratorpropertyattribute_true(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066255_rage_crtagiteratorbase_rage_crtagiteratorpropertyattribute_true(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066271_ctaskmeleeactionresult_isoverridingnetblendercommon_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066276_ctaskcombatmounted_ctaskcombatmounted_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066279_ctaskmovecombatmounted_ctaskmovecombatmounted_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066286_ctaskmovecombatmounted_registerattacker_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066293_ctaskmovecircle_computepointoncircle_rage_vec3v_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_066299_ctaskdyingdead_ctaskdyingdead_cdeathsourceinfo_const_ptr_int_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_066300_ctaskdyingdead_dtorctaskdyingdead(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066301_ctaskdyingdead_disablesleep_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066312_ctaskdyingdead_startmovenetwork_cped_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_066313_ctaskdyingdead_getdeathclip_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066314_ctaskdyingdead_updateblockingobject_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066315_ctaskdyingdead_dropweapon_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066316_ctaskdyingdead_processpushfromvehicle(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066317_ctaskdyingdead_isoffscreenandcansnaptoground_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066318_ctaskdyingdead_initdeadstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066319_ctaskdyingdead_triggerbloodpool_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066320_ctaskdyingdead_switchtoragdollframe_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066334_ctaskwrithe_getdieclipsetandrandclip_rage_fwmvtemplateid_rage_fw(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066335_rage_vec_v3anglezv_float_vector_4_float_vector_4(ph: *mut Phantom, a0: u128, a1: u128) -> u128; }
extern "C" { pub fn f_066342_ctaskcomplexonfire_createsubtask_cped_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_066351_ctaskdamageelectric_ctaskdamageelectric_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_066352_ctaskdamageelectric_dtorctaskdamageelectric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066357_ctaskdamageelectric_stateinit_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066358_ctaskdamageelectric_statedamageanimated_onenter(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066359_ctaskdamageelectric_statedamageanimated_onupdate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066360_ctaskdamageelectric_statedamagenm_onupdate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066369_cclonedreviveinfo_dtorcclonedreviveinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066372_ctaskrevive_ctaskrevive_cped_ptr_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_066374_ctaskrevive_dtorctaskrevive(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066389_ctaskinvestigate_ctaskinvestigate_rage_vector3_const_ref_cped_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_066390_ctaskinvestigate_dtorctaskinvestigate(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066395_ctaskinvestigate_enternearbyvehicle_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066467_ctaskcombat_ctaskcombat_cped_const_ptr_float_rage_fwflags_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_066469_ctaskcombat_spheresofinfluencebuilder_rage_tinfluencesphere_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066470_ctaskcombat_potentialblast_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066471_ctaskcombat_logtargetinformation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066472_ctaskcombat_dtorctaskcombat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066475_ctaskcombat_releasetacticalanalysis(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066478_ctaskcombat_updateguntarget_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066479_ctaskcombat_updatecommunication_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066481_ctaskentercover_prestreamaicoverentryanims_cped_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066482_ctaskcombat_changetarget_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066483_ctaskcombat_isvehiclemovingslowerthanspeed_cvehicle_const_ref_fl(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_066484_ccoverfinder_setupsearchparams_cped_const_ptr_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i64); }
extern "C" { pub fn f_066485_ctaskcombat_getclosestdefensiveareapointtonavigateto_cped_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_066487_ctaskcombat_canpedbeconsideredincover_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066488_ctaskcombat_shouldchasetarget_cped_ptr_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066489_ctaskcombat_canreacttoimminentexplosion_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066490_ctaskcombat_canreacttoexplosion_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066491_ctaskcombat_canreacttobuddyshot_cped_ptr_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066492_ctaskcombat_isvehiclesafeforjackorarrest_cvehicle_const_ptr_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066493_ctaskcombat_helpergetequippedweaponinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066494_ctaskcombat_getaimtarget_caitarget_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066495_ctaskreacttobuddyshot_isvalid_cped_const_ref_caitarget_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066502_ctaskcombat_settaskcombatstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_066506_ccover_findclosestcoverpoint_cped_const_ptr_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_066509_ctaskcombat_reacttoexplosion_onenter_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066510_ctaskreacttobuddyshot_ctaskreacttobuddyshot_caitarget_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_066512_ctaskcombat_advance_onenter_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066513_ctaskcombat_advance_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066514_ctaskcombat_equipweaponforthreat(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066515_ctaskvariedaimpose_ctaskvariedaimpose_ctask_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066516_ctaskcombat_updateambientanimplayback_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066517_ctaskcombat_checkfortargetlossresponse(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066518_ctaskcombat_checkforchargetarget_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_066519_ctaskcombat_checkforthrowsmokegrenade_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_066520_ctaskcombat_activatetimeslicing(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066521_ctaskadvance_clearsubtasktouseduringmovement(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066522_ctaskcombat_checkforpossiblestatechanges_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_066523_ctaskcharge_clearsubtasktouseduringmovement(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066524_ctaskcombat_doescoverpointprovidecover_ccoverpoint_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066525_ctaskcombat_gettargetsentrypoint_cvehicle_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066526_ctasksearchbase_loaddefaultparamsforpedandtarget_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066527_ctasksearchbase_ctasksearchbase_ctasksearchbase_params_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066528_ctaskdraggingtosafety_ctaskdraggingtosafety_cped_ptr_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_066529_ctaskcombat_isstatestillvalid_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_066533_ctaskcombat_getisplayingambientclip_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066534_ctaskcombat_getambientclipfromheadingdelta_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_066535_ccoverpoint_getarctouse_cped_const_ref_centity_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066544_ctaskreactaimweapon_dtorctaskreactaimweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066549_ctaskreactaimweapon_playreaction_onenter(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066550_ctaskreactaimweapon_playreaction_onupdate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066551_ctaskreactaimweapon_calculatedirection(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_066563_ctasksearchbase_dtorctasksearchbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066590_ctasksharkattack_ctasksharkattack_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066591_ctasksharkattack_dtorctasksharkattack(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066596_ctasksharkattack_imminentcollisionwithshore(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066602_ctaskthreatresponse_dtorctaskthreatresponse(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066608_ctaskthreatresponse_checkforresponsestatechange_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066609_ctaskthreatresponse_shouldfleefromcombat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066610_ctaskthreatresponse_generatefleetarget_caitarget_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066614_ctaskthreatresponse_determinethreatresponsestate_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_066615_ctaskthreatresponse_shouldalwaysfleeduetorules_cped_const_ref_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_066616_ctaskthreatresponse_ondirectlythreatened(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066617_ctaskthreatresponse_onperceivedgun(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066618_ctaskthreatresponse_settargetwantedlevelduetoevent_cevent_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066619_ctaskthreatresponse_setwantedlevelforped_cped_ptr_cevent_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066622_ctaskthreatresponse_getthreatresponsestate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066623_ctaskthreatresponse_builddecisioncache_ctaskthreatresponse_decis(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066628_ctaskthreatresponse_shouldpedbroadcastrespondedtothreatevent_cpe(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066629_ctaskthreatresponse_broadcastpedrespondedtothreatevent_cped_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066632_ctaskhandsup_ctaskhandsup_int_cped_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_066642_ctaskreacttogunaimedat_ctaskreacttogunaimedat_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066643_ctaskreacttogunaimedat_dtorctaskreacttogunaimedat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066649_ctasktohurttransit_dtorctasktohurttransit(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066653_bool_ccliphelper_isevent_rage_crpropertyattributebool_bool_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_066656_ctaskwrithe_dtorctaskwrithe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066661_ctaskwrithe_getistargetvalid_centity_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066663_ctaskwrithe_getrandclipfromclipset_rage_fwmvtemplateid_rage_fwmv(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066665_ctaskwrithe_streamreqresourcesin_cwritheclipsetrequesthelper_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_066683_ccoverpointscontainer_checkifnooverlapandgetgridcell_rage_vector(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_066687_ccover_ispointwithinvalidarea_rage_vector3_const_ref_ccoverpoint(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_066688_ccachedobjectcovermanager_addcoverpointsforphysical_cphysical_pt(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066689_ccachedobjectcover_setup_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066690_ccachedobjectcover_addcoverpoints_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066692_ccover_removecoverpointsforvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066693_rage_vector3_angle_rage_vector3_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_066694_ccover_addcoverpoint_ccover_coverpooltype_ccoverpoint_ecovertype(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i64) -> i32; }
extern "C" { pub fn f_066696_ccover_findcoverpoints_cped_ptr_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i32, a8: f32, a9: f32, a10: i64, a11: i64, a12: i64) -> i32; }
extern "C" { pub fn f_066697_ccover_scorecoverpoint_ccover_sscorecoverpointparams_ref_ccoverp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066699_ccover_findclosestcoverpointwithcb_cped_const_ptr_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_066702_ccover_findcovercoordinatesforped_cped_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_066703_ccoverpoint_setworldposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066704_ccoverpoint_setworlddirection_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_066706_ccoverpoint_releasecoverpointforped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066708_ccoverpoint_ccoverpoint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066709_ccoverpoint_copy_ccoverpoint_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066710_ccoverpoint_pedclosesttofacingleft_cped_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066714_ccover_removecoverblockingareasatposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066715_ccover_removespecificcoverblockingarea_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_066720_ccoversupport_doeshemispheretouchboxfacevectorized_rage_vec4v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64); }
extern "C" { pub fn f_066723_ctaskcover_computedesiredprotectiondirection_cped_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066724_ccoverdebug_adddebugpositionsphere_ccoverdebug_sdebugsphereparam(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066725_ccoverdebug_adddebugtext_rage_vec3v_const_ref_rage_color32_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_066726_cdynamiccoverhelper_getstickinputforcoversearch_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066727_cdynamiccoverhelper_computeinitialsearchdirection_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_066729_ctaskcover_isplayeraimingdirectlyinfirstperson_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066730_ctaskcover_canusethirdpersoncoverinfirstperson_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066731_ctaskcover_hitinstisfoliage_rage_phinst_const_ref_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_066732_ctaskcover_isplayerpedincoveragainstvehicledoor_cped_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066733_cdynamiccoverhelper_dolineprobetest_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i64; }
extern "C" { pub fn f_066734_ctaskentercover_checkifhitvehiclecoverdoor_rage_phinst_const_ref(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_066735_ctaskcover_computemvcamheadingforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_066738_ctaskcover_dtorctaskcover(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066741_ctaskcover_computestickinput_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066742_ctaskcover_iscoverpositionvalid_rage_vector3_const_ref_cped_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066743_ctaskcover_iscoverpointpositiontoofarandacute_cped_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066744_ctaskcover_doescoverpointprovidecoverfromthreat_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066748_ctaskcover_getweaponclipsetforarmament_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_066750_ctaskcover_usecover_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066751_ctaskcover_exitcover_onenter(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066752_ctaskcover_exitcover_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066753_cdynamiccoverhelper_setcoverentryentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066754_ccover_doescoverpointprovidecoverfromtargets_cped_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_066755_ctaskcover_getcoverpointposition_rage_vector3_const_ref_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_066756_ctaskcover_computefacingdirection_rage_vector3_const_ref_bool_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_066757_ctaskcover_ispathclearforped_cped_const_ref_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_066758_ctaskentercover_areaicoverentryclipsetsloaded_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066759_ctaskentercover_ispedangletocoveracceptableformovingentry_cped_c(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066760_ctaskentercover_shouldpeddostandingentry_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066761_ctaskentercover_chooseenddirectionforped_cped_ptr_rage_vector3_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> f32; }
extern "C" { pub fn f_066762_ctaskcover_activatetimeslicing(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066763_ctaskincover_findcoveredgeandupdatecoverposition_cped_ptr_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_066764_ctaskentercover_getfacingdirectionifblocked_cped_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_066765_ctaskentercover_getpedblockingmycover_cped_const_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_066766_ctaskexitcover_istaskvalid_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066767_ctaskexitcover_canpedusecornerexit_cped_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_066768_ctaskcover_startcovernetwork_bool_bool_bool_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: f32) -> i32; }
extern "C" { pub fn f_066770_rage_vector3_anglez_rage_vector3_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_066773_ctaskcover_checkformovingcover_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066781_ctaskcover_docapsuletest_cped_const_ref_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066782_ctaskincover_findedgeintersection_cped_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: i32) -> i32; }
extern "C" { pub fn f_066783_ccoverdebug_adddebugline_ccoverdebug_sdebuglineparams_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066784_cdynamiccoverhelper_dolinetestagainstvehicledummybounds_cped_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_066785_ccoverdebug_adddebugdirectionarrow_ccoverdebug_sdebugarrowparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066786_ctaskmotionincover_ispedsheadingcloseenoughtofacingdirection_cpe(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32) -> i32; }
extern "C" { pub fn f_066787_ctaskcover_restartusecovernetwork_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066788_ctaskcover_restartmotionincovernetwork_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_066789_cdynamiccoverhelper_findnewcoverpoint_ccoverpoint_ptr_cped_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_066790_ctaskmotionincover_shouldusefirstpersonlocoanimations_cped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066791_ctaskmotionincover_setgripclip_cped_ptr_cmovenetworkhelper_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066792_ctaskcover_iscoverclipsetloaded_rage_fwmvtemplateid_rage_fwmvcli(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066793_ctaskcover_isweaponclipsetloaded_bool_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066796_ctaskentercover_tunables_dtortunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066798_ctaskentercover_arecorecoverclipsetsloaded_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066802_cdynamiccoverhelper_processroundcoverdirection_cped_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_066805_ctaskentercover_setclipsfromanimstateinfo_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_066806_ctaskentercover_checkforweaponremoval_cped_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_066807_ctaskentercover_checkforplayerinterrupt_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066808_ctaskentercover_checkfornetworkinterrupt_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066809_ctaskentercover_computeinitialphase_rage_crclip_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066812_cclipeventtags_findmoverfixupstartendphases_rage_crclip_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_066813_ctaskentercover_computetransformfromoffsetssplat_rage_vec3v_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066814_ctaskentercover_computetargetmatrix_rage_vec3v_const_ref_rage_qu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_066815_ctaskmotionincover_playercheckformovingabout_cped_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_066816_ctaskmotionincover_playercheckforturning_cped_const_ref_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_066817_ctaskmotionincover_findnewdynamiccoverpoint_cped_ptr_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_066818_ctaskentercover_scaledesiredvelocitytopreventovershoot_cped_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32); }
extern "C" { pub fn f_066820_rage_vec_v4arctan2_float_vector_4_float_vector_4(ph: *mut Phantom, a0: u128, a1: u128) -> u128; }
extern "C" { pub fn f_066821_ctaskincover_slidepedintocover_cped_ref_ctaskincover_ptr_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066825_ctaskexitcover_tunables_dtortunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066831_ctaskexitcover_setupcoverexit_cped_ref_ctaskexitcover_eexitclipt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_066832_ctaskexitcover_updateexitdirection_cped_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066833_ctaskexitcover_getweaponholdingclipsetid_cped_const_ref_ctaskexi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_066838_cclonedcoverinfo_dtorcclonedcoverinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066842_rage_fwentity_setanimatedvelocity_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066845_ctaskincover_activatetimeslicing(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066846_ctaskincover_iscoverpointtoohigh_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066847_ctaskaimgunfromcoverintro_checkforblockedaimposition_bool_ref_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_066848_cdynamiccoverhelper_docapsuletest_rage_vector3_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_066850_ctaskincover_computecoveraimpositionandheading_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_066851_ctaskincover_shouldaimdirectly_cped_const_ref_bool_bool_ref_floa(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_066852_ctaskincover_computetargetintropositionandheading_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_066853_ctaskmotionincover_shouldusefirstpersonaiminganimations_cped_con(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066855_ctaskincover_computedesiredarc_cped_const_ref_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_066857_ctaskincover_calculatecoverposition(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066858_ctaskincover_calculatetimenextpeekallowed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066859_ctaskincover_restartweaponholding(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066860_ctaskmotionincover_getanimstateinfoforstate(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066862_ctaskincover_idle_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066863_ctaskincover_peeking_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066864_ctaskincover_peekingtoreload_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066865_ctaskincover_peekingtoreload_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066866_ctaskincover_blindfiring_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066867_ctaskincover_aimintro_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066868_ctaskincover_aimintro_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066869_ctaskincover_aim_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066870_ctaskincover_aimoutro_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066871_ctaskincover_aimoutro_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066872_ctaskincover_aimoutro_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066873_ctaskincover_throwingprojectile_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066874_ctaskincover_throwingprojectile_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066875_ctaskincover_reloading_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066876_ctaskincover_swapweapon_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066877_ctaskincover_swapweapon_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066878_ctaskincover_checkforexit(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066879_ctaskincover_processreloadsecondarytask(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066880_ctaskincover_processweaponswapsecondarytask(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066881_ctaskincover_setstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_066882_ctaskincover_checkformovingouttofire_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066883_ctaskincover_checkforswitchingweapon(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066884_ctaskincover_checkforreload_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_066885_ctaskincover_checkforthrowingprojectile(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066886_ctaskincover_checkforpeeking(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066887_ctaskincover_processpelvisoffset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066888_ctaskincover_createfiretask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066889_ctaskincover_processcoverstep_ctaskmotionaiming_ptr_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066890_ctaskincover_getthrownprojectileclipsetandclipids_cped_ptr_ctask(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i32) -> i32; }
extern "C" { pub fn f_066891_cpedinventory_getisweaponoutofammo_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_066892_ctaskmotionincover_choosepinnedvariationclipsetforped_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_066897_ctaskincover_isinmovingmotionstate_bool_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066898_ctaskincover_isfpspeekpositionobstructed_cped_const_ref_bool_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_066899_ctaskincover_checkforblindfireoverheadclearance_cped_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066900_ctaskincover_checkforoverthetopfiring(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066901_ctaskincover_findwallintersection_cped_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: i32) -> i32; }
extern "C" { pub fn f_066902_ctaskincover_canblindfirecorner_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066903_ctaskmotionincover_checkforturn_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_066904_ctaskincover_isaimbuttonhelddown(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066905_ctaskincover_setnextstatefromnetwork(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066906_ceventshockingbrokenglass_ceventshockingbrokenglass_centity_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_066907_ctaskaimgunfromcoverintro_computecurrentstepoutposition_bool_cpe(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_066908_ctaskincover_checkforaimoutrointerrupt_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066909_ctaskmotionincover_arestreamedanimsloaded_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_066911_ccoverdebug_adddebugcapsule_ccoverdebug_sdebugcapsuleparams_cons(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066921_ctaskmotionincover_tunables_dtortunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066922_ctaskmotionincover_tunables_animstateinfos_dtoranimstateinfos(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066929_cdynamiccoverhelper_checkforroundcover_rage_phintersection_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066932_ctaskmotionincover_checkforatedge_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066933_ctaskmotionincover_setclipsfromanimstateinfo_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_066934_ctaskmotionincover_processusemobilephone_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_066935_ctaskmotionincover_shouldrestartstateduetocameraswitch_cmovenetw(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066936_ctaskmotionincover_checkforinsidecornertransition(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066937_ctaskmotionincover_checkforidlevariation_cped_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066938_ctaskmotionincover_checkforstepping(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066939_ctaskmotionincover_checkformovingabout_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066940_ctaskmotionincover_checkforcovertocover(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066941_ctaskmotionincover_checkforsettle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066942_ctaskmotionincover_checkforidle_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_066943_ctaskmotionincover_checkformovingaroundcorner(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066944_ctaskmotionincover_setvariationclip_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066945_ctaskmotionincover_processstopping_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_066946_ctaskmotionincover_forceresendofsyncdata_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066948_ctaskmotionincover_sendcommonmovesignals(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066949_ctaskmotionincover_computemvcamheading_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_066956_ctaskmotionincover_processstoppingphysics_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_066959_ctaskmotionincover_iscoverpointtoohigh_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066960_ctaskmotionincover_chooseidlevariationclipsetforped_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_066961_ctaskmotionincover_computegaussianvalueforrestrictedcamerapitch(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_066971_ctaskaimgunfromcoverintro_processintrorotation_cped_ref_float_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_066982_ctaskaimgunfromcoveroutro_processoutrorotation_cped_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_066988_ctaskaimgunblindfire_dtorctaskaimgunblindfire(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067008_cplayercoverclipvariationhelper_updatecovervariationsfrominfo_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_067012_cdynamiccoverhelper_dtorcdynamiccoverhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067015_cdynamiccoverhelper_doinitialcollisioncheck_cped_ptr_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: f32); }
extern "C" { pub fn f_067016_cdynamiccoverhelper_checkforcoverandcalculatenormal_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i64, a11: i64) -> i32; }
extern "C" { pub fn f_067017_cdynamiccoverhelper_rendernewcoverpositionandnormal_rage_vec3v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_067018_cdynamiccoverhelper_updatecoverpointfromresults_cped_ptr_probere(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_067019_cdynamiccoverhelper_detectobstructionsinfrontofped_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_067020_cdynamiccoverhelper_docollisioncheckformovementincover_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_067021_cdynamiccoverhelper_checkforcoverandcalculatenormalwhenmoving_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: f32) -> i32; }
extern "C" { pub fn f_067022_cdynamiccoverhelper_docollisioncheckwhenidleincover_cped_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: f32); }
extern "C" { pub fn f_067023_cdynamiccoverhelper_computetestmatrix_cped_ptr_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64); }
extern "C" { pub fn f_067024_cdynamiccoverhelper_dobatchshapetests_rage_mat34v_ref_collisiont(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i64, a5: i64, a6: i64, a7: f32); }
extern "C" { pub fn f_067025_cdynamiccoverhelper_dovehiclecoverboundboxtests_rage_mat34v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i32); }
extern "C" { pub fn f_067026_cdynamiccoverhelper_dovehicleboxtests_rage_mat34v_const_ref_cveh(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_067027_cdynamiccoverhelper_removeconflictingnormalresults_proberesults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067028_rage_phboundpolyhedron_getvertex_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_067029_cdynamiccoverhelper_isoutsidedepthrange_float_const_ref_float_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_067030_cdynamiccoverhelper_isoutsideangularrange_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_067033_ccoverdebug_scontextdebuginfo_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067034_ccoverdebug_adddebugpositionsphereanddirection_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_067049_ccoverfinderfsm_validatecoverpoints(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067052_ccoverfinderfsm_dtorccoverfinderfsm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067056_ccoverfinderfsm_reset_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_067058_ccoverfinderfsm_checktacticalanalysisinfo_onupdate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_067059_ccoverfinderfsm_buildaccessibilitycapsuledesc_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_067065_ctaskstayincover_calculateaimatposition_cped_ptr_rage_vector3_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_067068_cboatchasedirector_dtorcboatchasedirector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067074_ctaskadvance_dtorctaskadvance(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067077_ctaskadvance_scoreposition_ctaskmovetotacticalpoint_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_067081_ctaskadvance_canseek_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067085_ctaskmoveseekentity_214_centityseekposcalculatorlastnavmeshinter(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: i32) -> i64; }
extern "C" { pub fn f_067092_ctaskboatchase_dtorctaskboatchase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067095_ctaskboatchase_removefromdirector(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067101_ctaskboatcombat_dtorctaskboatcombat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067108_ctaskboatstrafe_dtorctaskboatstrafe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067113_ctaskcharge_dtorctaskcharge(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067119_ctaskdraggedtosafety_ctaskdraggedtosafety_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_067120_ctaskdraggedtosafety_dtorctaskdraggedtosafety(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067125_ctaskdraggedtosafety_blendfromnmtoanimation_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_067130_ctaskdraggingtosafety_dtorctaskdraggingtosafety(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067134_ctaskdraggingtosafety_setcoverpoint_ccoverpoint_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067139_ctaskdraggingtosafety_calculatetargetandheading_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_067141_ctaskdraggingtosafety_chooseclipforpickupentry_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067148_ctaskhelichase_dtorctaskhelichase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067156_ctaskhelichase_calculatetargetposition_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067157_ctaskhelichase_calculatespeed_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_067160_ctaskhelicombat_dtorctaskhelicombat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067163_ctaskhelicombat_calculatetargetoffsetforchase_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067171_ctaskplanechase_dtorctaskplanechase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067182_ctaskreacttobuddyshot_dtorctaskreacttobuddyshot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067192_ctaskvariedaimpose_dtorctaskvariedaimpose(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067197_ctaskvariedaimpose_playtransition_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067198_ctaskvariedaimpose_playtransition_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067199_ctaskvariedaimpose_playtransition_ctaskvariedaimpose_aimpose_tra(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067210_ctaskvehiclechase_dtorctaskvehiclechase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067213_ctaskvehiclechase_ismakingaggressivemove_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067217_ctaskvehiclechase_removefromvehiclechasedirector(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067218_cvehiclechasedirector_cvehiclechasedirector_cphysical_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_067220_ctaskvehiclechase_canmakeaggressivemove_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067222_ctaskvehiclechase_loadvehiclemissionparameters_svehiclemissionpa(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067223_ctaskvehiclechase_issituationvalid_ctaskvehiclechase_issituation(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067224_ctaskvehiclechase_isaggressivemovesuccessful_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067225_ctaskvehiclechase_canram_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067226_ctaskvehiclechase_canspinout_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067227_ctaskvehiclechase_ispullalongsidesuccessful_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067229_ctaskvehiclechase_canblock_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067230_ctaskvehiclechase_isblockappropriate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067231_ctaskvehiclechase_cananyoneinvehicledodrivebys_cvehicle_const_re(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067234_ctaskvehiclepersuit_dtorctaskvehiclepersuit(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067238_ctaskvehiclepersuit_shouldwaitincar_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067239_ctaskvehiclepersuit_shouldchaseincar_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067240_ctaskvehiclepersuit_choosedefaultstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067241_ctaskvehiclepersuit_getdesiredstate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067242_ctaskvehiclepersuit_getdesiredaircraftstate_cped_ptr_ctaskvehicl(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067243_ctaskvehiclepersuit_countnumberbuddiesenteringvehicle_cped_ptr_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067244_ctaskvehiclepersuit_createslowdowngototask_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067245_ctaskvehiclepersuit_shouldexitvehicletoaim_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067247_ctaskvehiclepersuit_getobstructioninformation_ctaskvehiclegotopo(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067248_ctaskvehiclepersuit_iscollisionwithlawenforcementimminent_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_067249_ctaskvehiclepersuit_gettargetpositionforemergencystop_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067250_ctaskvehiclepersuit_findobstructionatangle_cvehicle_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32) -> i32; }
extern "C" { pub fn f_067256_ctaskvehiclecombat_dtorctaskvehiclecombat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067260_ctaskvehiclecombat_getdesiredstate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067261_ctaskvehiclecombat_checktargetinglos_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067266_ctasksearchbase_processprefsm(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067286_ctaskshellshocked_choosedirection_cped_const_ref_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_067291_ctaskshootattarget_dtorctaskshootattarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067294_ctaskshootattarget_choosestate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067298_ctaskshootouttire_dtorctaskshootouttire(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067303_ctaskshootouttire_getaimposition_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067305_ctaskstealthkill_ctaskstealthkill_cped_const_ptr_cactionresult_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_067306_ctaskstealthkill_dtorctaskstealthkill(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067314_ctasksubmarinechase_dtorctasksubmarinechase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067320_ctasksubmarinecombat_dtorctasksubmarinecombat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067327_ctasktargetunreachable_dtorctasktargetunreachable(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067334_ctasktargetunreachableininterior_dtorctasktargetunreachableinint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067341_ctasktargetunreachableinexterior_dtorctasktargetunreachableinext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067346_ctasktargetunreachableinexterior_createsubtaskforcombat_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067347_cvehiclechasedirector_dtorcvehiclechasedirector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067357_ccrimeinfomanager_load(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067367_crandomeventmanager_updatepossibilities(ph: *mut Phantom); }
extern "C" { pub fn f_067369_crandomeventmanager_flushcopasserts(ph: *mut Phantom); }
extern "C" { pub fn f_067371_rage_strrequestarray_4u_haveallloaded(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_067372_crandomeventmanager_cancreateeventoftype_erandomevent_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_067373_rage_strrequestarray_4u_pushrequest_unsigned_int_unsigned_int_un(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_067374_ctaskpursuecriminal_givetasktoped_cped_ref_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_067375_crandomeventmanager_seteventstarted_erandomevent(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_067377_ctaskpursuecriminal_ctaskpursuecriminal_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_067381_ccrimeinfomanager_dtorccrimeinfomanager_107786(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067384_ctaskpursuecriminal_dtorctaskpursuecriminal(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067391_ctaskreacttopursuit_addcop_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067392_ctaskpursuecriminal_checkplayeraudio_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067393_ctaskreacttopursuit_givetasktoped_cped_ref_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067397_rage_atfixedarray_rage_fwregdref_cped_rage_fwrefawarebase_3_dtor(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067406_ctaskreacttopursuit_getbestcop_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_067407_ctaskreacttopursuit_leavevehicle_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067410_ctaskstealvehicle_ctaskstealvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_067411_ctaskstealvehicle_dtorctaskstealvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067420_ctaskclip_dtorctaskclip_107850(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067444_ctaskreacharm_dtorctaskreacharm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067448_ctaskscriptedanimation_cpriorityslot_op_ctaskscriptedanimation_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067454_ctaskhandsup_dtorctaskhandsup(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067499_ctasksharkcircle_dtorctasksharkcircle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067511_ctaskmelee_dtorctaskmelee(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067517_ctaskmovemeleemovement_dtorctaskmovemeleemovement(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067529_ctaskmovecombatmounted_dtorctaskmovecombatmounted(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067546_cclonedtaskinfoweapontargethelper_serialise_rage_csyncdatabase_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067591_csyncedvehicle_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067646_ctaskvehiclecombat_supportsterminationrequest(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067757_ctaskmoveseekentity_214_centityseekposcalculatorlastnavmeshinter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067766_ctaskmoveseekentity_214_centityseekposcalculatorlastnavmeshinter(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_067768_cambientanimationmanager_randomlychoosepropforspawningped_cscena(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_067769_cambientanimationmanager_getpropsetfromhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_067770_cambientanimationmanager_fillsearchinfoforconditionalanimsgroup(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_067771_cambientanimationmanager_chooseconditionalanimations_cscenarioco(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_067772_cambientaudio_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_067777_cambientaudioexchange_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_067785_cambientaudioconditions_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_067791_cambientaudiotrigger_conditionalexchange_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_067796_cambientaudiotrigger_conditionalexchange_dtorconditionalexchange(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067799_cambientaudiotrigger_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_067804_cambientaudiotrigger_dtorcambientaudiotrigger(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067805_cambientaudioset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_067814_cambientaudio_getpartialhash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067815_cambientaudiomanager_dtorcambientaudiomanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067817_cambientaudiomanager_getexchangefromtriggersforset_rage_atnonfin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_067821_cambientaudiomanager_evaluateconditions_cambientaudioconditions(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_067822_cambientaudiomanager_evaluatefollowup_cambientaudioconditions_fo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_067823_cambientaudiomanager_ispedfleeing_cped_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067824_cambientaudiomanager_ispedincombat_cped_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067825_cambientaudiomanager_getexchangefromtrigger_cambientaudiotrigger(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_067826_cambientaudiomanager_processexchange_cambientaudioexchange_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_067842_cambientlookat_getlookatblockingstate_cped_const_ptr_ctaskambien(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_067843_cambientlookat_pickcameralookatplayer_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067844_cambientlookat_pickmotionlookatplayer_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067845_cambientlookat_scoreinterestinglookat_cped_ptr_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_067846_cambientlookat_scoreinterestinglookatplayer_cped_ptr_centity_con(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_067847_cambientlookat_getmodifiedscorebasedonhistory_cped_ref_centity_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> f32; }
extern "C" { pub fn f_067848_cambientlookat_loadaudioexchangebetweenpeds_cped_const_ref_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_067849_cambientlookat_lookatnearbyscenariopoint_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067850_cclipeventtags_getmoveeventstartandendphases_rage_crclip_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_067854_ctaskambientclips_ctaskambientclips_unsigned_int_cconditionalani(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_067855_ctaskambientclips_dtorctaskambientclips(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067856_cambientlookat_dtorcambientlookat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067861_ctaskambientclips_initprophelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067862_cpropmanagementhelper_givepedprop_cped_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067866_cpropmanagementhelper_process_cpropmanagementhelper_context_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067867_cpropmanagementhelper_disposeofprop_cped_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067868_ctaskambientclips_createhangoutconversationhelper_cped_ptr_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067869_cpropmanagementhelper_dropprop_cped_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_067871_cpropmanagementhelper_processmovesignals_cpropmanagementhelper_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067872_ctaskambientclips_updateanimationvfx_ccliphelper_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067874_ctaskambientmigratehelper_applymigratebaseclip_ctaskambientclips(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_067875_cpropmanagementhelper_releaseproprefifunshared_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067876_ctaskambientclips_setconditionalanimchosen_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_067877_ctaskambientclips_getidleblockingstate_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_067878_ctaskambientclips_checkcliphasvfxtags_rage_crclip_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067879_ctaskambientclips_checkcliphasproptags_rage_crclip_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067880_csimpleblendhelper_getrate_ccliphelper_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_067881_ctaskambientclips_startsynchronizedbaseclip_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_067886_ctaskambientclips_createlookat_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_067887_ctaskambientclips_updatebaseclipplayback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067888_ctaskambientclips_sendlowriderarmclipmoveparams_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_067890_ctaskambientclips_createphoneconversationhelper(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067891_ctaskmotionbasiclocomotion_isperformingidleturn_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067904_ctaskambientlookatevent_dtorctaskambientlookatevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067912_ctaskambientmigratehelper_dtorctaskambientmigratehelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067915_ctaskchat_ctaskchat_cped_ptr_rage_fwflags_int_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_067917_ctaskcomplexcontrolmovement_ctaskcomplexcontrolmovement_ctask_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_067918_ctaskcomplexcontrolmovement_getrunningmovementtask_cped_const_pt(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067926_ctaskplayeronfoot_checkforusemobilephone_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067927_ctaskmobilephone_canusemobilephonetask_cped_const_ref_ctaskmobil(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_067928_ctaskplayeronfoot_testifroutetocoverpointisclear_ccoverpoint_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_067929_ctaskplayeronfoot_didcapsuletesthitsomething_cped_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_067930_ctaskplayeronfoot_didprobetesthitsomething_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_067931_ctaskplayeronfoot_ctaskplayeronfoot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067932_ctaskplayeronfoot_dtorctaskplayeronfoot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067936_ctaskplayeronfoot_shouldmeleetargetped_cped_const_ptr_cweapon_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_067938_ctaskplayeronfoot_isstatevalidforfpsik_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_067939_ctaskplayeronfoot_isplayerstickinputvalid_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067940_ctaskplayeronfoot_getlaststickinputifvalid_rage_vector2_ref_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067942_ctaskplayeronfoot_scorecoverpoint_rage_vector3_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> f32; }
extern "C" { pub fn f_067943_ctaskplayeronfoot_iscovervalid_rage_vector3_const_ref_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_067946_ctaskplayeronfoot_hasvalidentervehicleinput_ccontrol_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067948_ctasksayaudio_ctasksayaudio_char_const_ptr_float_float_centity_p(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32, a6: f32) -> i64; }
extern "C" { pub fn f_067951_ctaskplayeronfoot_playeridles_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067952_ctaskplayeronfoot_takeoffjetpack_onupdate_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067953_ctaskplayeronfoot_checkfornearbycover_cped_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_067954_ctaskplayeronfoot_keepwaitingforladder_cped_ptr_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_067955_ctaskmobilephone_isrunningmobilephonetask_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067956_ctaskplayeronfoot_getbreakouttomovementstate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067957_ctaskplayeronfoot_canperformuncuff(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067958_ctaskplayeronfoot_dojumpcheck_cped_ptr_ccontrol_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_067959_ctaskplayeronfoot_checkcanentervehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067960_ctaskslopescramble_candoslopescramble_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_067961_ctaskplayeronfoot_checkforusingjetpack_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067962_ctaskplayeronfoot_checkshouldclimbladder_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067963_ctaskplayeronfoot_checkshouldentervehicle_cped_ptr_ccontrol_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_067964_cclimbhandholddetected_dtorcclimbhandholddetected(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067965_ctaskplayeronfoot_checkshouldslopescramble(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067966_ctaskplayeronfoot_hasvalidentercoverinput_ccontrol_const_ref_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067967_ccoverpoint_op_ccoverpoint_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067968_ctaskplayeronfoot_checkfordropdown_cped_ptr_ccontrol_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_067969_ctaskplayeronfoot_canjumporvault_cped_ptr_spedtestresults_ref_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_067970_ctaskplayeronfoot_canuncuffped_cped_const_ptr_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_067971_ctaskslopescramble_nogroundsteepslopecheck_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067972_ctaskslopescramble_getactivationangle(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_067973_ctaskslopescramble_getcheckdistance_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_067974_ctaskslopescramble_checkaheadforobjects_cped_ptr_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_067980_ctaskplayeronfoot_getclimbdetectorparam_int_cped_ptr_spedtestpar(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_067982_ctaskmobilephone_createorresumemobilephonetask_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067983_ctaskmobilephone_ctaskmobilephone_int_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_067985_ctaskmoveplayer_ctaskmoveplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067992_ctaskmoveplayer_processstdmove_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067993_ctaskmoveinair_ctaskmoveinair(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067995_ctaskcomplexcontrolmovement_getmoveblendratio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_068003_ctaskunalerted_ctaskunalerted_ctask_ptr_cscenariopoint_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_068004_ctaskunalerted_dtorctaskunalerted(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068012_ctaskunalerted_trytoobtainchainuserinfo_cscenariopoint_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_068013_ctaskunalerted_cleanupscenariopointusage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068015_ctaskunalerted_shouldholsterweapon_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068016_ctaskunalerted_pickwanderstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068017_ctaskwander_init_float_float_cconditionalanimsgroup_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i32, a5: f32, a6: i32); }
extern "C" { pub fn f_068018_ctaskunalerted_startscenariofindersearch(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068019_cpavementfloodfillhelper_updatepavementfloodfillrequest_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068020_cscenariopoint_isobstructedandreset(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068021_ctaskunalerted_canscenariobereused_cscenariopoint_ref_cscenarioc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_068022_ctaskunalerted_canreturntovehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068023_ctaskunalerted_findvehicleusingmychain_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068024_ctaskunalerted_canscenariobereusedwithcurrentrealtype_cscenariop(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_068025_ctaskunalerted_canscenariobereusedwithnewrealtype_cscenariopoint(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_068031_cpavementfloodfillhelper_cleanuppavementfloodfillrequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068049_ctaskwander_dtorctaskwander(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068056_ctaskwander_processcrossroadatlights(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068057_ctaskcomplexcontrolmovement_setnewmovetask_ctask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068058_ctaskwander_createambientclipstask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068059_ctaskcomplexcontrolmovement_setnewmainsubtask_ctask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068060_ctaskwander_helpertrytocrossroadatlights_ctaskwander_roadcrosssc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_068064_ctaskwander_computescanforroadcrossings_cped_ptr_ctaskgametimer(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_068065_ctaskwander_doesnodeandlinkmatchlastcrossing_cped_ptr_rage_cpath(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_068074_ctaskslopescramble_canslideonsurface_rage_vector3_const_ref_unsi(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068081_ctaskslopescramble_stateinitial_onupdate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068082_ctaskslopescramble_statedoslopescramble_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068083_ctaskslopescramble_statedoslopescramble_onupdate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068084_ctaskslopescramble_statedoslopescrambleoutro_onupdate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068085_ctaskslopescramble_getfacingdircliptype_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_068093_cpatrolnode_dtorcpatrolnode(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068102_ctaskstandguardfsm_getdirectionvector_testdirections_cped_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_068106_ctaskpatrol_ctaskpatrol_unsigned_int_ctaskpatrol_alertstatus_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_068108_ctaskpatrol_dtorctaskpatrol(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068114_ctaskpatrol_start_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068115_ctaskpatrol_chattoped_onenter_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068116_ctaskpatrol_chattoped_onupdate_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068117_ctaskpatrol_playscenario_onenter_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068118_ctaskpatrol_playscenario_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068119_ctaskpatrol_getnextpatrolnode(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068126_ctaskdonothing_ctaskdonothing_int_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_068132_ctaskpause_ctaskpause_int_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068142_ctaskcomplexcontrolmovement_dtorctaskcomplexcontrolmovement(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068144_ctaskcomplexcontrolmovement_beginmovementtasks_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_068150_ctaskcomplexcontrolmovement_settarget_cped_const_ptr_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_068151_ctaskcomplexcontrolmovement_setmoveblendratio_cped_const_ptr_flo(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_068155_ctasklist_ctasklist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068156_ctasklist_dtorctasklist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068157_ctasklist_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068159_ctasklist_addtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068160_ctasklist_from_ctasklist_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068162_ctasksequencelist_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068164_ctaskmovesequence_ctaskmovesequence(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068171_ctaskmovesequence_addtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068176_ctasktriggerlookat_dtorctasktriggerlookat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068186_ctaskcomplexusemobilephone_dtorctaskcomplexusemobilephone(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068204_ctaskusesequence_dtorctaskusesequence(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068208_ctaskusesequence_gettasksequencelist_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068213_ctaskopendoor_ctaskopendoor_cdoor_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_068215_ctaskopendoor_calculateclosestdoorposition_cped_const_ptr_cdoor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_068216_ctaskopendoor_shoulduselefthand_cped_const_ptr_cdoor_ptr_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_068217_ctaskopendoor_ispedinfrontofdoor_cped_const_ptr_cdoor_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068219_ctaskopendoor_navigatingpedmustabort_cped_const_ptr_cdoor_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068220_ctaskopendoor_updateconstraint_cped_ptr_bool_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: f32); }
extern "C" { pub fn f_068221_ctaskopendoor_applyclosedelay_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_068222_ctaskopendoor_sendparams_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068225_ctasksayaudio_dtorctasksayaudio(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068239_ctaskshoveped_ctaskshoveped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_068242_ctaskshoveped_sendparams_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_068247_ctaskaffectsecondarybehaviour_ctaskaffectsecondarybehaviour_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_068249_ctaskaffectsecondarybehaviour_dtorctaskaffectsecondarybehaviour(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068259_ctasktakeoffhelmet_processfinished_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068269_ctaskgeneralsweep_ctaskgeneralsweep_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: f32, a8: f32) -> i64; }
extern "C" { pub fn f_068270_ctaskgeneralsweep_ctaskgeneralsweep_int_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: f32, a8: f32) -> i64; }
extern "C" { pub fn f_068271_ctaskgeneralsweep_dtorctaskgeneralsweep(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068274_ctaskgeneralsweep_stateinitial_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068275_ctaskgeneralsweep_calculatedesiredpitchandyaw(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068276_ctaskgeneralsweep_statedosweep_onupdate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068277_ctaskgeneralsweep_getyawminmaxandpitch_rage_crclip_const_ptr_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_068284_ctaskaggressiverubberneck_dtorctaskaggressiverubberneck(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068289_ctaskcallpolice_canmakecall_cped_const_ref_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068293_ctaskcallpolice_ctaskcallpolice_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_068294_ctaskcallpolice_dtorctaskcallpolice(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068297_cambientaudio_cambientaudio_char_const_ptr_rage_fwflags_unsigned(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068309_ctaskmobilephone_removepedphone(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068310_ctaskmobilephone_blendoutnetwork(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068311_ctaskmobilephone_isstatevalidforfpsik_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068312_ctaskmobilephone_shouldpause_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068313_ctaskmobilephone_givepedphone(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068317_ctaskmobilephone_setadditionalsecondaryanimation_rage_crclip_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_068318_ctaskmobilephone_clearadditionalsecondaryanimation_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_068319_ctaskmobilephone_getadditionalsecondaryanimclipandfilter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068322_ctaskmobilephone_testsetadditionalsecondaryanims(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068324_ctaskmobilephone_processclipmoveeventtags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068327_ctaskmobilephone_putuptext_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068328_ctaskmobilephone_textloop_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068329_ctaskmobilephone_putdownfromtext_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068330_ctaskmobilephone_puttoearfromtext_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068333_cpropmanagementhelper_dtorcpropmanagementhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068335_cpropmanagementhelper_hasdestroyorreleasetag_cpropmanagementhelp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_068336_cpropmanagementhelper_handleclipeventdestroyprop_cpropmanagement(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068337_cpropmanagementhelper_handleclipeventreleaseprop_cpropmanagement(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068342_ctaskconversationhelper_dtorctaskconversationhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068346_cpavementfloodfillhelper_cpavementfloodfillhelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068363_ctaskopendoor_dtorctaskopendoor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068367_ctaskshoveped_dtorctaskshoveped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068373_ctaskchat_dtorctaskchat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068435_cpatrollink_dtorcpatrollink(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068441_ctaskmotionbase_calcdesiredvelocity_rage_mat34v_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_068442_ctaskmotionped_checkfordiving(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068443_ctaskmotionbase_calcvelchangelimitandclamp_cped_const_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_068444_ctaskmotionbase_retrievemovespeeds_rage_fwclipset_ref_cmoveblend(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_068445_rage_atmapentry_unsigned_int_float_const_ref_rage_atmap_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_068448_ctaskmotionbase_updategaitreduction(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068453_ctaskmotionbase_getbaseidleclip(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068454_ctaskmotionbase_setonfootclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_068456_ctaskmotionbase_clearoverrideweaponclipset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068457_ctaskmotionbase_resetonfootclipset_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_068464_ctaskmotionbase_setindependentmoverframe_float_rage_fwmvrequesti(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_068467_ctaskmotionbase_getdefaultonfootclipset_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_068469_ctaskmotionbase_interpvalue_float_ref_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32) -> f32; }
extern "C" { pub fn f_068470_ctaskmotionbase_interpvalue_float_ref_float_float_bool_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_068471_ctaskmotionbase_getdefaultaimingstrafingclipset_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_068472_ctaskmotionbase_getfloatpropertyfromclip_rage_fwmvtemplateid_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_068473_ctaskmotionbase_updateheadik(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068480_ctaskbirdlocomotion_getstaticstatename_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_068481_ctaskbirdlocomotion_setnewgroundpitchforbird_cped_ref_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_068482_ctaskbirdlocomotion_canlandnow_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068485_ctaskbirdlocomotion_setnewheading(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068505_rage_atarray_ctaskmotionswimming_scubagearvariations_0_unsigned(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_068511_ctaskmotionswimming_findscubagearvariationforped_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068512_ctaskmotionswimming_isscubagearvariationactiveforped_cped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068513_ctaskmotionswimming_onpeddiving_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068514_ctaskmotionswimming_setscubagearvariationforped_cped_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_068517_ctaskmotionswimming_pitchcapsulebound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068518_ctaskmotiondiving_updatedivegaitreduction_cped_ptr_bool_ref_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_068520_ctaskmotionswimming_checkforwaterfall_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068521_ctaskmotionswimming_processfloworientation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068522_ctaskmotionswimming_sendparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068524_ctaskmotionswimming_checkforrapids_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068525_ctaskmotionswimming_candivefromposition_cped_ref_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_068529_ctaskmotiondiving_testgroundclearance_cped_ptr_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32) -> f32; }
extern "C" { pub fn f_068531_ctaskmotionswimming_getswimmingclipsetid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068533_ctaskmotionswimming_gethighestwaterlevelaroundpos_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64) -> f32; }
extern "C" { pub fn f_068535_ctaskmotionswimming_setweaponclipset_rage_fwmvtemplateid_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_068540_ctaskmotiondiving_sendweaponclipset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068541_ctaskmotiondiving_setweaponholdingclip_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_068544_ctaskmotiondiving_sendparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068545_ctaskmotiondiving_getdivingclipsetid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068546_ctaskmotiondiving_updateriverreorientation_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068551_ctaskmotiondiving_setweaponclipset_rage_fwmvtemplateid_rage_fwmv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_068552_ctaskmotionbasiclocomotion_dtorctaskmotionbasiclocomotion(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068557_rage_atmapentry_unsigned_int_float_const_ref_rage_atmap_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_068558_ctaskmotionbasiclocomotion_sendparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068559_ctaskmotionbasiclocomotion_sendalternatewalksignals(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068560_cpedmotiondata_getgaitreduceddesiredmoveblendratio_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068563_ctaskmotionbasiclocomotion_processmovement_cped_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_068565_ctaskmotionbasiclocomotion_startmoving(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068566_ctaskmotionbasiclocomotion_adjustturnfordesireddirection_float_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068590_ctaskfishlocomotion_statedead_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068592_ctaskfishlocomotion_interpolatespeed_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_068593_ctaskfishlocomotion_setnewheading(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068594_ctaskfishlocomotion_setnewpitch(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068600_ctaskfishlocomotion_togglepedswimnearsurface_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_068601_ctaskfishlocomotion_checkandrespectswimnearsurface_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_068602_ctaskfishlocomotion_togglefishsurvivesbeingoutofwater_cped_ptr_b(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_068608_ctaskflightlessbirdlocomotion_setnewheading_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_068615_ctaskhumanlocomotion_ctaskhumanlocomotion_ctaskhumanlocomotion_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i64, a9: f32) -> i64; }
extern "C" { pub fn f_068616_ctaskhumanlocomotion_setmovementclipset_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068617_ctaskhumanlocomotion_setweaponclipset_rage_fwmvtemplateid_rage_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_068618_ctaskhumanlocomotion_sendweaponclipset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068619_ctaskhumanlocomotion_dtorctaskhumanlocomotion(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068626_ctaskhumanlocomotion_computeandsendpitchedweaponholding(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068627_ctaskhumanlocomotion_sendidleturnsignals(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068634_ctaskhumanlocomotion_allowladderrunmount_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068636_ctaskhumanlocomotion_startalternateclip_ctaskmotionbase_alternat(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_068637_ctaskhumanlocomotion_sendalternateparams_ctaskmotionbase_alterna(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_068639_ctaskhumanlocomotion_islastfootleft_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068643_ctaskhumanlocomotion_isfootdown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068645_ctaskhumanlocomotion_processsnowdepth_cped_const_ptr_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_068646_ctaskhumanlocomotion_processslopesandstairs_cped_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_068648_ctaskmotionaimingtransition_usemotionaimingtransition_float_floa(ph: *mut Phantom, a0: f32, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_068649_ctaskhumanlocomotion_istransitionblocked_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068650_ctaskhumanlocomotion_flipfootflagsforstopping_ctaskmovegotopoint(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068651_ctaskhumanlocomotion_initfromstrafeupperbodytransition(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068652_ctaskhumanlocomotion_updatefromstrafeupperbodytransition(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068655_ctaskhumanlocomotion_resetslopesandstairs_cped_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068657_ctaskhumanlocomotion_isonslopesandstairs_cped_const_ptr_bool_flo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_068658_ctaskgotoandclimbladder_scanforladdertoclimb_cped_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_068659_ctaskgotoandclimbladder_isgettingonbottom_centity_ptr_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_068660_ctaskgotoandclimbladder_findtopandbottomofallladdersections_cent(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_068661_ctaskgotoandclimbladder_isclosestladder_centity_ptr_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_068662_ctaskgotoandclimbladder_calcladderinfo_centity_ref_int_rage_vect(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_068663_ctaskclimbladderfully_isladderbaseortopblockedbypedmp_centity_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_068664_ctaskmotionped_getweaponinfo_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068666_ctaskmotionped_dtorctaskmotionped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068668_ctaskmotionped_tasksetstate_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_068669_ctaskmotionped_getstaticstatename_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_068683_ctaskvehiclefsm_requestvehicleclipset_rage_fwmvtemplateid_rage_f(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_068687_ctaskmotionped_forcestatechange_rage_fwmvstateid_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068688_ctaskmotionped_forcesubstatechange_rage_fwmvstateid_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068689_ctaskmotionped_requestaimingclipset(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068690_ctaskmotionped_shoulduseaimingstrafe_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068691_ctaskmotionped_createonfoottask_rage_fwmvtemplateid_rage_fwmvcli(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_068692_ctaskmotionped_applyweaponclipset_ctaskhumanlocomotion_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068693_ctaskmotionped_doaimingtransitionindependentmover_ctaskhumanloco(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068694_ctaskmotionped_processclonefpsidle_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068695_ctaskmotionped_isparachuting_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068696_ctaskmotionped_isaimingtransitionnotblocked_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068697_ctaskmotionped_isaimingindependentmovernotblocked_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068698_ctaskmotionswimming_ctaskmotionswimming_int_bool_float_float_flo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_068699_ctaskmotionped_applyweaponclipset_ctask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068700_ctaskmotiondiving_ctaskmotiondiving_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068701_ctaskmotionaiming_islastfootleft_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068702_ctaskmotionaiming_ctaskmotionaiming_bool_bool_bool_bool_ctaskmot(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: f32, a7: i32, a8: f32, a9: f32, a10: f32, a11: f32, a12: i32) -> i64; }
extern "C" { pub fn f_068703_ctaskmotionped_selectfpsblendtime_cped_ptr_bool_cpedmotiondata_e(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> f32; }
extern "C" { pub fn f_068704_ctaskmotionaiming_willstartinmotion_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068705_ctaskmotionaiming_restartupperbodyaimnetwork_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068706_ctaskmotionaiming_restartlowerbodyaimnetwork(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068707_ctaskmotionped_checkfortransitionbreakout(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068708_ctaskmotioninturret_keepturrethatchopen_int_cvehicle_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_068710_ctaskmotionaiming_istransitionblocked_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068712_ctaskmotionaiming_isfootdown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068714_ctaskmotionaiming_getcanmove_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068715_ctaskmotionaiming_setaimclipset_rage_fwmvtemplateid_rage_fwmvcli(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068716_ctaskmotionaiming_computepitchsignal_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_068717_ctaskmotionaiming_computeaimintrosignals_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_068718_ctaskmotionaiming_sendstrafeaimsignals(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068737_ctaskmotionaiming_dtorctaskmotionaiming(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068740_ctaskmotionaiming_isvalidforunarmedik_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068745_ctaskmotionaiming_countfiringeventsinclip_rage_crclip_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068751_ctaskmotionaiming_isplayingtransition_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068752_ctaskmotionaiming_isturning_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068754_ctaskmotionaiming_shouldturnoffupperbodyanimations_float_ref_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068755_ctaskmotionaiming_choosenewaimingclip(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068758_ctaskmotionaiming_setstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_068759_ctaskmotionaiming_getusestartsandstops_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068760_ctaskmotionaiming_dopostcamerasetheading_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068761_ctaskmotionaiming_computestandingidlesignals(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068762_ctaskmotionaiming_sendstandingidlesignals(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068763_ctaskmotionaiming_sendfpsswimmingmovesignals(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068764_ctaskmotionaiming_strafing_onprocessmovesignals(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068765_ctaskmotionaiming_calculatenormalisevelocity_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_068769_ctaskmotionaiming_processonfootaiminglefthandgripik_cped_ref_cwe(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_068772_ctaskmotionaiming_usinglefttriggeraimmode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068790_ctaskmotionjetpack_dtorctaskmotionjetpack(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068801_ctaskmotionparachuting_dtorctaskmotionparachuting(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068804_ctaskfall_ctaskfall_rage_fwflags_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068808_ctaskparachute_calcdesiredvelocity_rage_matrix34_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_068817_ctaskmotionstrafing_sendparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068819_ctaskmotionstrafing_getclipset(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068826_ctaskmotiontennis_dtorctaskmotiontennis(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068832_ctaskmotiontennis_updatedesiredheading_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068833_ctaskmotiontennis_sendparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068835_ctaskmotiontennis_playdiveanim_ctaskmotiontennis_edivedirection(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32, a5: i32); }
extern "C" { pub fn f_068836_ctaskmotiontennis_playtennisswinganim_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32); }
extern "C" { pub fn f_068847_ctaskquadlocomotion_dtorctaskquadlocomotion(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068851_ctaskquadlocomotion_statedead_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068852_ctaskquadlocomotion_processgaits(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068853_ctaskquadlocomotion_processturnphase(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068854_ctaskquadlocomotion_sendlocomotionmovesignals(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068876_ctaskmotioninturret_dtorctaskmotioninturret(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068880_ctaskmotioninturret_ikhandstoturret_cped_ref_cvehicle_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_068881_ctaskmotioninturret_updateangvelocityandacceleration_cturret_con(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_068882_ctaskmotioninturret_updatemvturnspeed_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_068883_ctaskmotioninturret_updatemvpitch_cturret_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_068884_ctaskmotioninturret_updatemvheading_cturret_const_ptr_cped_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_068886_ctaskmotioninturret_computeheadingdelta_cturret_const_ptr_float(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> f32; }
extern "C" { pub fn f_068888_ctaskmotioninturret_assetshavebeenstreamed_cped_const_ptr_cvehic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_068889_ctaskmotioninturret_getadjuststepclips_rage_atarray_rage_crclip(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068890_ctaskmotioninturret_getappropriateclipsetid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068891_ctaskmotioninturret_processfiring(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068892_ctaskmotioninturret_checkforturning_bool_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068904_cjetpackmanager_createjetpackpickup_rage_matrix34_const_ref_cped(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068905_cjetpack_dtorcjetpack(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068906_cjetpack_destroyobject(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068908_ctaskanimatedfallback_ctaskanimatedfallback_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32) -> i64; }
extern "C" { pub fn f_068924_ctaskcomplexevasivestep_ctaskcomplexevasivestep_centity_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_068925_ctaskcomplexevasivestep_dtorctaskcomplexevasivestep(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068929_ctaskgetup_ctaskgetup_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068931_rage_quaternion_toeulers_rage_vector3_ref_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068941_ctaskmovegotopoint_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068943_ctaskwalkroundentity_dtorctaskwalkroundentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068952_ctaskwalkroundcarwhilewandering_dtorctaskwalkroundcarwhilewander(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068958_ctasknavbase_ctasknavbase_float_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i64; }
extern "C" { pub fn f_068959_ctaskmovefollownavmesh_isunabletofindroute_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068961_ctaskmovewalkroundvehicle_dtorctaskmovewalkroundvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068979_ctaskcrawl_orientbounds_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068988_ctaskfall_stateinitial_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068989_ctaskfall_stateinitial_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068990_ctaskfall_initfallstatedata_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068991_ctaskfall_statefall_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068992_ctaskfall_statefall_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068993_ctaskfall_statedive_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068994_ctaskfall_statedive_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068995_ctaskfall_statecrashlanding_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068996_ctaskfall_statehighfall_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068997_ctaskfall_statehighfall_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068998_ctaskfall_stateparachute_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068999_ctaskfall_statelanding_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069000_ctaskfall_statelanding_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069001_ctaskfall_statelanding_onexit_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069002_ctaskjump_ispednearground_cped_const_ptr_float_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_069003_ctaskfall_calculatefalldistance(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069004_ctaskfall_getinitialfalltestangle_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069005_ctaskfall_calculatelocomotionexitparams_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069006_ctaskparachute_canpedparachutefromposition_cped_const_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32) -> i32; }
extern "C" { pub fn f_069007_ctaskfall_processhighfall_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069008_ctaskfall_gettestendusingangle_cped_const_ptr_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_069009_ctaskparachute_ctaskparachute_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_069010_ctaskfall_gethasinterruptedclip_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069011_ctaskfall_istryingtomove_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069013_ctaskfall_updateheading_cped_ptr_ccontrol_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_069016_ctaskfall_getishighfall_cped_ptr_bool_float_ref_float_float_floa(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32, a4: f32, a5: f32) -> i32; }
extern "C" { pub fn f_069032_ctaskflytopoint_ctaskflytopoint_float_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_069039_ctaskfollowwaypointrecording_ctaskfollowwaypointrecording_cwaypo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_069040_ctaskfollowwaypointrecording_dtorctaskfollowwaypointrecording(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069046_ctaskfollowwaypointrecording_modifygotopointtask_cped_ptr_ctaskm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_069047_ctaskfollowwaypointrecording_getpositionahead_cped_ptr_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32) -> i32; }
extern "C" { pub fn f_069048_ctaskfollowwaypointrecording_calculateprogressfromposition_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_069057_ctaskfollowwaypointrecording_statefollowingroute_onenter_cped_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069058_ctaskfollowwaypointrecording_statefollowingroute_onupdate_cped_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069059_ctaskfollowwaypointrecording_orientatetotarget_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069060_ctaskfollowwaypointrecording_statefollowingrouteaiming_onupdate(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069061_ctaskfollowwaypointrecording_createaimingorshootingtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069062_ctaskfollowwaypointrecording_getdistanceremainingbeforestopping(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> f32; }
extern "C" { pub fn f_069063_ctaskmovegotopoint_calculatemoveblendratioforcorner_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: i64, a8: i64) -> f32; }
extern "C" { pub fn f_069064_ctaskturntofaceentityorcoord_ctaskturntofaceentityorcoord_centit(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_069065_ctaskmovegotopointandstandstill_ctaskmovegotopointandstandstill(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: f32) -> i64; }
extern "C" { pub fn f_069066_ctaskmovegotopointandstandstill_areclipsstillmoving(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069067_ctaskmovestandstill_ctaskmovestandstill_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_069069_ctaskmoveachieveheading_ctaskmoveachieveheading_float_float_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_069075_ctaskturntofaceentityorcoord_ctaskturntofaceentityorcoord_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_069076_ctaskturntofaceentityorcoord_dtorctaskturntofaceentityorcoord(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069089_ctaskgetup_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069090_ctaskgetup_ctaskgetup_rage_atnonfinalhashstring_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069091_ctaskgetup_dtorctaskgetup(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069095_ctaskgetup_shouldabortexternal_rage_aievent_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069097_ctaskgetup_gettreatasplayer_cped_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069103_ctaskgetup_findgetupcontrollingtask_cped_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069104_ctaskgetup_processupperbodyaim_cped_ptr_rage_crclip_const_ptr_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_069105_ctaskgetup_processdropdown_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069106_cdropdowndetector_cdropdowndetectorparams_cdropdowndetectorparam(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069107_cdropdowndetector_process_rage_matrix34_ptr_cdropdowndetector_cd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_069108_cdropdown_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069109_cdropdowndetector_resetpendingresults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069110_cdropdowndetector_deleteproberesults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069111_cnmblendoutsetmanager_getblendoutset_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069113_ctaskgetup_playerwantstoaimorfire_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069115_ctaskgetup_setstatefromblendoutitem_cnmblendoutitem_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069116_ctaskgetup_startmovenetwork(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069117_ctaskgetup_getdefaultarmsduration_rage_crclip_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069118_ctaskgetup_getplaybackratemodifier_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069119_ctaskgetup_decideonnextaction_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069120_ctaskgetup_processmoverfixup_cped_ptr_rage_crclip_const_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32) -> f32; }
extern "C" { pub fn f_069121_ctaskgetup_processbranchtags_rage_atnonfinalhashstring_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069122_cdropdown_cdropdown_cdropdown_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069123_ctaskgetup_selectblendoutsets_cped_ptr_cnmblendoutsetlist_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_069124_ctaskgetup_isvalidblockingentity_cped_ptr_centity_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_069125_ctaskgetup_testlos_rage_vector3_const_ref_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_069127_ctaskgetup_findclipsetwithgetup_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069128_ctaskgetup_getrequestedblendoutsets_cnmblendoutsetlist_ref_cped(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069129_ctaskgetup_shoulduseinjuredgetup_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069130_ctaskgetup_pickstandardgetupset_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069131_cnmblendoutsetlist_settarget_caitarget_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069132_cnmblendoutsetlist_relinquishmovetask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069141_ctaskmovefollowpointroute_createsubtask_int_cped_ptr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_069149_ctaskgotopointanymeans_ctaskgotopointanymeans_float_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: f32, a5: i32, a6: i32, a7: i32, a8: f32, a9: f32, a10: f32, a11: i32, a12: f32, a13: f32, a14: f32) -> i64; }
extern "C" { pub fn f_069151_ctaskgotopointanymeans_dtorctaskgotopointanymeans(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069156_ctaskgotopointanymeans_findandregisterthreats_cped_ptr_cpedtarge(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_069157_ctaskgotopointanymeans_warppedifstuck_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069158_ctaskgotopointanymeans_isvehiclecloseenoughtotargettouse_cvehicl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_069159_ctaskgotopointanymeans_findbesttargetped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069160_ctaskgetoutofwater_ctaskgetoutofwater_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i64; }
extern "C" { pub fn f_069166_ctaskmovegotopoint_ctaskmovegotopoint_float_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_069173_ctaskmovecrowdaroundlocation_adjustcrowdrndposforotherpeds_cped(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069175_ctaskmovecrowdaroundlocation_createsubtask_int_cped_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_069178_ctaskgotoscenario_ctaskgotoscenario_ctaskgotoscenario_tasksettin(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069179_ctaskgotoscenario_dtorctaskgotoscenario(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069182_ctaskgotoscenario_setadditionaltaskduringapproach_ctask_const_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069185_ctaskmovefollownavmesh_isfollowingnavmeshroute_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069188_ctaskmovegotopoint_dtorctaskmovegotopoint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069192_ctaskmovegotopoint_debug_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069193_taskgotopointavoidance_processavoidance_taskgotopointavoidance_s(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069194_taskgotopointavoidance_box_avoidance_compute_ray_box_intersectio(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_069215_ctaskmovegotopointandstandstill_goingtopoint_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069216_ctaskmovegotopointandstandstill_selectmoveblendratio_ctaskmovego(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32) -> i32; }
extern "C" { pub fn f_069217_ctaskmovegotopointandstandstill_goingtopoint_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069218_ctaskmovegotopointandstandstill_processstoppingbehaviour_cped_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069219_ctaskmovegotopointandstandstill_updateangularvelocity_cped_ptr_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_069225_ctaskmovegotopointrelativetoentityandstandstill_dtorctaskmovegot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069232_ctaskmovegotopointstandstillachieveheading_createsubtask_cped_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_069242_ctaskgotopointaiming_ctaskgotopointaiming_caitarget_const_ref_ca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_069243_ctaskgotopointaiming_dtorctaskgotopointaiming(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069250_ctaskmovefollownavmesh_setscriptbehaviourflags_int_tnavmeshscrip(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_069251_ctaskgotopointaiming_createguntask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069252_ctaskgotopointaiming_getdesiredweaponcontrollertype(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069256_ctaskmovefollowentityoffset_ctaskmovefollowentityoffset_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_069257_ctaskmovefollowentityoffset_dtorctaskmovefollowentityoffset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069265_ctaskmovefollowentityoffset_issueprobe_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069267_ctaskmovefollowentityoffset_checkforstatechange_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069271_ctaskmovebeinformation_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069272_ctaskmovebeinformation_ctaskmovebeinformation_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069273_ctaskmovebeinformation_dtorctaskmovebeinformation(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069274_ctaskmovebeinformation_cancelprobes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069282_ctaskmovebeinformation_updatepedpositioning_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069283_ctaskmovebeinformation_createsubtask_int_cped_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_069284_ctaskmovebeinformation_isinposition(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069285_ctaskmovebeinformation_istooclosetoleader(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069286_ctaskmovebeinformation_getmoveblendratiotouse_cped_ptr_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32) -> f32; }
extern "C" { pub fn f_069287_ctaskmovetrackingentity_ctaskmovetrackingentity_float_centity_pt(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_069288_ctaskmovebeinformation_arepedsmovingtowardseachother_cped_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069289_ctasknavbase_getdistanceleftoncurrentroutesection_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_069290_ctaskmovetrackingentity_dtorctaskmovetrackingentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069299_ctaskmovetotacticalpoint_ctaskmovetotacticalpoint_caitarget_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_069301_ctaskmovetotacticalpoint_defaultscorenavmeshpoint_ctaskmovetotac(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069302_ctaskmovetotacticalpoint_defaultscorecoverpoint_ctaskmovetotacti(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069303_ctaskmovetotacticalpoint_dtorctaskmovetotacticalpoint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069304_ctaskmovetotacticalpoint_clearsubtasktouseduringmovement(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069306_ctaskmovetotacticalpoint_ismovingtopoint_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069307_ctaskmovetotacticalpoint_setsubtasktouseduringmovement_ctask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069308_ctaskmovetotacticalpoint_gettargetped_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069310_ctaskmovetotacticalpoint_iscoverpointnevervalidtomoveto_ctactica(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069311_ctaskmovetotacticalpoint_isnavmeshpointnevervalidtomoveto_ctacti(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069312_ctaskmovetotacticalpoint_excludepointtomoveto(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069313_ctaskmovetotacticalpoint_findpointtomoveto_ctaskmovetotacticalpo(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069314_ctaskmovetotacticalpoint_hasroutefailed(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069315_ctasknavbase_testspheresintersectingroute_int_rage_spdsphere_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_069316_cnavmeshroute_expandroute_cnavmeshexpandedroutevert_ptr_float_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_069317_ctaskmovetotacticalpoint_isclosetopositiontomoveto_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069318_ctaskmovewithinattackwindow_ispositionoutsideattackwindow_cped_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_069319_ctaskmovetotacticalpoint_ismovingtoinvalidcoverpoint_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069320_ctaskmovetotacticalpoint_ismovingtoinvalidnavmeshpoint_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069321_ctaskmovetotacticalpoint_onunabletofindroute(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069322_ctaskmovetotacticalpoint_onroutetooclosetotarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069323_ctaskmovetotacticalpoint_releasepoint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069324_ctaskmovetotacticalpoint_reservepoint_ctaskmovetotacticalpoint_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069325_ctaskmovetotacticalpoint_shouldnotuseladdersorclimbs_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069331_ctaskmovewander_ctaskmovewander_float_float_bool_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32, a4: f32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_069332_ctaskmovewander_init_cwanderparams_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069333_ctaskmovewander_dtorctaskmovewander(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069334_ctasknavbase_dtorctasknavbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069335_ctasknavbase_conentityroutedata_dtorconentityroutedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069336_cladderclipsetrequesthelperpool_releaseclipsetrequesthelper_clad(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069341_ctaskmovewander_getwandertarget_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069344_ctasknavbase_processprefsm(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069345_ctasknavbase_calculateclosestsegmentonexpandedroute_cnavmeshexpa(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_069347_ctasknavbase_updatefsm_int_rage_aitask_fsm_event(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_069349_ctasknavbase_statefollowingpath_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069350_ctasknavbase_requestpath_cped_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_069351_ctasknavbase_retrievepath_rage_tpathresultinfo_ref_cped_ptr_cnav(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_069352_ctasknavbase_switchtonextroute(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069353_ctasknavbase_calctimeneededtocompleteroutesegment_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_069354_ctasknavbase_getpositionaheadonroute_cnavmeshexpandedroutevert_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64, a9: i64) -> i32; }
extern "C" { pub fn f_069355_ctasknavbase_processlookahead_cped_ptr_bool_rage_vector3_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_069356_ctasknavbase_createroutefollowingtask_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069358_ctasknavbase_initpathrequest_distanceaheadcalculation_float_cped(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_069362_ctasknavbase_onsuccessfulroute_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069366_ctasknavbase_drawpedtext_cped_const_ptr_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_069371_ctaskmovecrossroadattrafficlights_ctaskmovecrossroadattrafficlig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_069380_ctaskmovecrossroadattrafficlights_setwaitorcrossroadstate_cped_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069381_ctaskmovecrossroadattrafficlights_createwaittask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069383_ctaskmovewaitfortraffic_issafetocross_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069387_ctaskmovewithinattackwindow_ctaskmovewithinattackwindow_cped_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069388_ctaskmovewithinattackwindow_dtorctaskmovewithinattackwindow(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069393_ctaskmovewithinattackwindow_findgotoposition_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069394_ctaskmovewithinattackwindow_getattackwindowminmax_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_069396_ctaskmovewithindefensivearea_ctaskmovewithindefensivearea_cped_c(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069397_ctaskmovewithindefensivearea_dtorctaskmovewithindefensivearea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069401_ctaskmovewithindefensivearea_updategotolocation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069411_ctasknavbase_forcevehicledooropenfornavigation_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069414_ctasknavbase_getistasksequencedadjacent_cped_const_ptr_ctaskmove(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_069424_cladderclipsetrequesthelperpool_getnextfreehelper_cped_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069425_ctaskclimbladder_streamreqresourcesin_cladderclipsetrequesthelpe(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_069426_ctasknavbase_checkifclimbispossible_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_069427_ctasknavbase_getnearbyentitiesforlineofsighttest_cped_ptr_centit(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32, a6: f32) -> i32; }
extern "C" { pub fn f_069439_ctasknavbase_testclearance_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_069440_cclimbdetector_process_rage_matrix34_ptr_spedtestparams_ptr_sped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_069441_cclimbhandholddetected_op_cclimbhandholddetected_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069442_cclimbhandholddetected_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069443_cclimbdetector_searchforhandhold_rage_matrix34_const_ref_cclimbh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_069444_cclimbdetector_resetpendingresults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069445_cclimbdetector_getdetectedhandhold_cclimbhandholddetected_ref_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069450_ctasknavbase_settarget_cped_const_ptr_rage_vec3v_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_069453_ctasknavbase_handlenewtarget_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069457_ctasknavbase_gettotalroutelength_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069458_ctasknavbase_istotalroutelongerthan_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_069459_ctasknavbase_getwaypointflag_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_069460_ctasknavbase_getthiswaypointline_cped_const_ptr_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_069463_ctasknavbase_getpositionaheadonroute_cped_const_ptr_float_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_069477_ctaskmovefollownavmesh_ctaskmovefollownavmesh_float_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: i64, a8: f32, a9: i32) -> i64; }
extern "C" { pub fn f_069478_ctaskmovefollownavmesh_ctaskmovefollownavmesh_cnavparams_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069488_ctaskmovefollownavmesh_shouldtrytogetonnavmesh_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069502_ctaskuseclimbonroute_dtorctaskuseclimbonroute(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069508_ctaskusedropdownonroute_dtorctaskusedropdownonroute(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069512_ctaskuseladderonroute_ctaskuseladderonroute_float_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: i64) -> i64; }
extern "C" { pub fn f_069513_ctaskuseladderonroute_dtorctaskuseladderonroute(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069523_rage_atarray_ctaskparachute_parachutepackvariations_0_unsigned_s(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_069526_ctaskparachute_dtorctaskparachute(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069529_ctaskparachute_clearparachutepackvariationforped_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069530_ctaskparachute_findparachutepackvariationforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069531_ctaskparachute_isparachutepackvariationactiveforped_cped_const_r(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069533_ctaskparachute_setparachutepackvariationforped_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069534_ctaskparachute_gettintindexforparachutepack_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069540_ctaskparachute_isparachuteout_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069541_ctaskparachute_shouldignorepedcollisionwithentity_centity_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069542_ctaskparachute_detachpedfromparachute(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069543_ctaskparachute_getmodelforparachute_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069546_ctaskparachute_pitchcapsulebound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069547_ctaskparachute_processclonevehiclecollision_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_069548_ctaskparachute_initialisepedinair_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069554_ctaskparachute_ispedinstatetoparachute_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069556_ctaskparachute_tasksetstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_069559_ctaskparachute_start_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069560_ctaskparachute_start_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069561_ctaskparachute_waitingforbasetostream_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069562_ctaskparachute_blendfromnm_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069563_ctaskparachute_blendfromnm_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069564_ctaskparachute_waitingformovenetworktostream_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069565_ctaskparachute_transitiontoskydive_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069566_ctaskparachute_transitiontoskydive_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069567_ctaskparachute_waitingforskydivetostream_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069568_ctaskparachute_waitingforskydivetostream_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069569_ctaskparachute_skydiving_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069570_ctaskparachute_skydiving_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069571_ctaskparachute_deploying_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069572_ctaskparachute_deploying_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069573_ctaskparachute_parachuting_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069574_ctaskparachute_parachuting_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069575_ctaskparachute_landing_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069576_ctaskparachute_landing_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069577_ctaskparachute_crashlanding_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069578_ctaskparachute_crashlanding_onupdate_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069579_ctaskparachute_candeployparachute_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069580_ctaskparachute_getparachuteobjecttask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069581_ctaskparachute_attachparachutetoped(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069582_ctaskparachute_attachpedtoparachute(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069583_ctaskparachute_setparachutetintindex(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069584_ctaskparachute_attachmovenetwork(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069585_ctaskparachute_getclipsetforstate_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069586_ctaskparachute_setmovenetworkstate_rage_fwmvrequestid_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_069587_ctaskparachute_scanforcrashlanding(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069588_ctaskparachute_startfirstpersoncamerashakeforskydiving(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069589_ctaskparachute_checkfordeploy(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069590_ctaskparachute_setstateforskydiving(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069591_ctaskparachute_processparachutingcontrolsforai(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069592_ctaskparachute_processparachutingcontrolsforplayer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069593_ctaskparachute_createtaskforambientclips_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069594_ctaskparachute_handleparachutelanding_rage_fwflags_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_069597_ctaskparachute_processfirstpersonparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069600_ctaskparachute_probeforparachuteintersections(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069601_ctaskparachute_destroyparachute(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069602_ctaskparachuteobject_makevisible_cobject_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_069603_ctaskparachute_canconfigurecloneparachute_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069604_ctaskparachute_applyextraforcesfromvalue_float_float_cphysical_p(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_069605_ctaskparachute_calculatedesiredflightanglesforparachutingwithlim(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_069606_ctaskparachute_getmodelforparachutepack_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069610_ctaskparachute_updateparachutebone_ctaskparachute_eparachutebone(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32, a5: i32); }
extern "C" { pub fn f_069611_ctaskparachute_processparachutingcontrols_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_069618_ctasktakeoffpedvariation_createprop_cped_ptr_epedvarcomp_rage_fw(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_069619_ctasktakeoffpedvariation_attachprop_cobject_ptr_cped_ptr_eanimbo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_069620_ctasktakeoffpedvariation_clearvariation_cped_ptr_epedvarcomp_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_069627_ctaskjetpack_ctaskjetpack_rage_fwflags_unsigned_char(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069628_ctaskjetpack_dtorctaskjetpack(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069633_ctaskjetpack_calculatetargetgotoposition(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069639_ctaskjetpack_getanglesformove_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069640_ctaskjetpack_processaimcontrols_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_069669_ctaskparachuteobject_idle_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069670_ctaskparachuteobject_idle_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069683_ctaskfollowleaderinformation_ctaskfollowleaderinformation_cpedgr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_069684_ctaskfollowleaderinformation_dtorctaskfollowleaderinformation(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069688_ctaskfollowleaderinformation_checkentervehicle_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069689_ctaskfollowleaderinformation_shouldseekcover_cped_const_ptr_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069692_ctaskfollowleaderanymeans_dtorctaskfollowleaderanymeans(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069698_ctaskseekentityaiming_dtorctaskseekentityaiming(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069708_ctaskslidetocoord_processslidephysics_rage_vector3_const_ref_cpe(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32) -> i32; }
extern "C" { pub fn f_069718_ctasktakeoffpedvariation_ctasktakeoffpedvariation_rage_fwmvtempl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32, a8: i64, a9: i32, a10: i64) -> i64; }
extern "C" { pub fn f_069733_canfindfurtherintersections_worldprobe_cshapetesthitpoint_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069736_cclimbdetector_getmaxclimbheight_spedtestparams_ptr_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069737_cclimbdetector_testcapsule_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_069738_cclimbdetector_getisvalidintersection_worldprobe_cshapetesthitpo(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069740_cclimbdetector_setclimbphysical_cclimbhandholddetected_ref_world(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_069741_cclimbdetector_getminclimbheight_spedtestparams_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> f32; }
extern "C" { pub fn f_069742_cclimbdetector_getmaxheightdropallowance_const(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_069743_cclimbhandholddetected_cclimbhandholddetected_cclimbhandholddete(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069744_cclimbdetector_dovalidationtestindirection_rage_matrix34_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_069745_ctaskvault_isslidesupported_cclimbhandholddetected_const_ref_cpe(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_069746_cclimbhandholddetected_calcphysicalhandholdoffset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069747_cclimbdetector_adddebugintersections_worldprobe_cshapetesthitpoi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_069748_cclimbdetector_isintersectionentityanunclimbabledoor_centity_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_069749_cclimbdetector_testoffsetcapsule_rage_vector3_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64, a6: f32, a7: i32) -> i32; }
extern "C" { pub fn f_069750_cclimbdetector_findfirstvalidintersection_worldprobe_cshapetesth(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_069755_cclimbhandholddetected_cclimbhandholddetected(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069758_cdropdowndetector_adddebugintersections_worldprobe_cshapetesthit(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_069759_cdropdowndetector_testcapsule_rage_vector3_const_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64) -> i32; }
extern "C" { pub fn f_069760_ctaskjump_dojumpgaptest_cped_const_ptr_jumpanalysis_ref_bool_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_069771_ctaskdropdown_getdropdownspeed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069844_ctaskgotoandclimbladder_ctaskgotoandclimbladder_centity_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_069848_ctaskgotoandclimbladder_isladdertopblockedmp_centity_ptr_cped_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_069849_ctaskgotoandclimbladder_ispedorientatedcorrectlytoladdertogeton(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_069863_ctaskclimbladderfully_isladderphysicallyblockedatbasemp_cped_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_069935_ctaskvault_getvaultspeed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069941_ctaskvault_canbreakouttomovementtask_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069959_ctaskfallover_ctaskfallover_rage_fwmvtemplateid_rage_fwmvclipset(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_069960_ctaskfallover_computefallcontext_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_069961_ctaskfallover_pickfallanimation_cped_ptr_ctaskfallover_efallcont(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_069977_ctaskfallover_applyinitialvehicleimpact_cvehicle_ptr_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069979_ctaskfallandgetup_ctaskfallandgetup_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_069982_ctaskfallandgetup_getclipphase(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069987_ctaskgetupandstandstill_ctaskgetupandstandstill(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069993_ctaskjumpvault_ctaskjumpvault_rage_fwflags_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_069997_ctaskjumpvault_willvault_cped_ptr_rage_fwflags_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_070005_ctaskjump_ispedneargroundincradius_cped_const_ptr_float_float_ra(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_070109_ctaskmotionpedlowlod_isonfoot(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070361_cnmblendoutsetlist_cnmblendoutsetlist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070362_cnmblendoutsetlist_dtorcnmblendoutsetlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070363_cnmblendoutsetlist_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070364_cnmblendoutsetlist_add_rage_atnonfinalhashstring_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_070366_cnmblendoutsetmanager_isblendoutsetstreamedin_rage_atnonfinalhas(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070367_cnmblendoutsetlist_findposeitem_rage_fwmvtemplateid_rage_fwmvcli(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_070371_cnmblendoutsetmanager_finditem_rage_atnonfinalhashstring_unsigne(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_070372_cnmblendoutsetmanager_load(ph: *mut Phantom); }
extern "C" { pub fn f_070374_cnmblendoutsetmanager_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_070396_ctasknmbehaviour_wantstoragdoll_cped_ptr_eragdolltriggertypes_ce(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> f32; }
extern "C" { pub fn f_070399_ctasknmcontrol_ctasknmcontrol_unsigned_int_unsigned_int_rage_ait(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: f32) -> i64; }
extern "C" { pub fn f_070401_ctasknmbehaviour_canuseragdoll_cped_ptr_eragdolltriggertypes_cen(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_070403_ctaskblendfromnm_ctaskblendfromnm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070405_ctaskblendfromnm_ctaskblendfromnm_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070410_ctasknmbehaviour_doesresponsehandleragdoll_cped_const_ptr_rage_a(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_070501_ctasknmbehaviour_ragdollpoolhasspaceforped_ctasknmbehaviour_erag(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: f32) -> i32; }
extern "C" { pub fn f_070510_ctasknmcontrol_cleanupunhandledragdoll_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070519_ctasknmcontrol_switchclonepedtoragdoll_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070529_ctasknmhighfall_ctasknmhighfall_unsigned_int_cgrabhelper_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_070530_ctasknmbalance_ctasknmbalance_unsigned_int_unsigned_int_centity(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64, a6: f32, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_070534_ctasknmbehaviour_addtoragdollpool_ctasknmbehaviour_eragdollpool(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_070592_ctasknmdangle_createqueriablestate_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070600_ctasknmdraggingtosafety_ctasknmdraggingtosafety_cped_ptr_cped_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_070636_ctasknmfalldown_ctasknmfalldown_unsigned_int_unsigned_int_ctaskn(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_070675_ctasknmsimple_ctasknmsimple_ctasknmsimple_tunables_tuning_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070681_ctasknminjuredonground_ctasknminjuredonground_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_070694_ctasknmjumprollfromroadvehicle_ctasknmjumprollfromroadvehicle_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_070707_ctasknmonfire_ctasknmonfire_unsigned_int_unsigned_int_float_rage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070739_ctasknmrelax_ctasknmrelax_unsigned_int_unsigned_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_070769_ctasknmshot_ctasknmshot_cped_ptr_unsigned_int_unsigned_int_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32, a7: i64, a8: i64, a9: i64, a10: i32) -> i64; }
extern "C" { pub fn f_070874_ctaskagitated_findagitatedtaskforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070913_ctasksmartflee_ctasksmartflee_caitarget_const_ref_float_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_070916_ctasksmartflee_accelerateinvehicleforevent_cped_const_ptr_cevent(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_070932_ctasksmartflee_setupfleeinggetup_cnmblendoutsetlist_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_070937_ctasksmartflee_setfleeflags_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_070938_ctasksmartflee_setmoveblendratio_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_071028_ctaskintimidate_cleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071039_ctaskreactandflee_ctaskreactandflee_caitarget_const_ref_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_071044_ctaskreactandflee_setmoveblendratio_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_071073_ctaskreactandflee_pickappropriatereactionclipsetfromhash_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_071074_ctaskreactandflee_pickdefaultgunfirereactionclipset_rage_vec3v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_071080_ctaskreactindirection_isvalid_rage_fwmvtemplateid_rage_fwmvclips(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_071088_ctaskreacttoexplosion_ctaskreacttoexplosion_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_071093_ctaskreacttoexplosion_choosestate_cped_const_ref_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_071101_ctaskreacttoimminentexplosion_ctaskreacttoimminentexplosion_cait(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i64; }
extern "C" { pub fn f_071105_ctaskreacttoimminentexplosion_isvalid_cped_const_ref_caitarget_c(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_071218_ctaskduckandcover_ctaskduckandcover(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072481_cscenariopoint_getposition_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072482_cscenarioinfomanager_getscenarioindex_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_072490_cscenariopoint_getscenariotypevirtualorreal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072503_cscenariopointmanager_updatecachedregionptrs_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072504_cscenariochaininggraph_addchainuser_cscenariopointchainuseinfo_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_072505_cscenariopointmanager_getnumregions_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072506_cscenariopointmanager_getregion_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_072510_cscenariochaininggraph_removechainuser_cscenariopointchainuseinf(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072512_cscenariochaininggraph_removechainuserifunused_cscenariopointcha(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072539_cscenariomanager_getscenarioinfo_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_072547_cscenariomanager_isvehiclescenariotype_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_072561_ctaskusescenario_ctaskusescenario_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_072564_cscenariomanager_checkscenariotimesandprobability_cscenariopoint(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_072575_ctaskusescenario_ctaskusescenario_int_cscenariopoint_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_072577_ctaskscenario_ctaskscenario_int_cscenariopoint_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_072669_cscenariopoint_hastimeoverride_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072731_cscenariomanager_getscenariotypefromhashkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_072789_cscenariofinder_findoptions_findoptions(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072790_cscenariofinder_cscenariofinder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072791_cscenariofinder_dtorcscenariofinder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072792_cscenariofinder_setscenariopoint_cscenariopoint_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_072796_cscenariofinder_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072797_cscenariofinder_startsearchfornextinchain_cped_ref_cscenariofind(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_072798_cscenariofinder_findnewscenario_cped_const_ptr_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_072802_cscenariomanager_isscenariopointattachedentityuprootedorbroken_c(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072822_cscenariomanager_getconditionalanimsgroupforscenario_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_072834_cscenariomanager_createtask_cscenariopoint_ptr_int_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32) -> i64; }
extern "C" { pub fn f_072840_cscenariopoint_getfailedcollisionticks(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_073088_cscenarioconditionhasprop_getpropmodelindex_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073407_ctaskchatscenario_ctaskchatscenario_int_cped_ptr_int_cscenariopo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_073478_ctaskusescenario_setadditionaltaskduringapproach_ctask_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073506_ctaskusescenario_setstationaryreactionsenabledforped_cped_ref_bo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073535_ctaskusevehiclescenario_ctaskusevehiclescenario_int_cscenariopoi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: f32, a5: i64) -> i64; }
extern "C" { pub fn f_073542_ctaskusevehiclescenario_setscenariopointnxt_cscenariopoint_ptr_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_073543_ctaskusevehiclescenario_getscenarioconditionalanimsgroup_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073546_cclonedtaskusevehiclescenarioinfo_getnextscenariopoint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073563_ctaskwanderingscenario_getwanderscenariotousenext_int_cped_ptr(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_073570_cclonedtaskwanderingscenarioinfo_getcurrscenariopoint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073571_cclonedtaskwanderingscenarioinfo_getnextscenariopoint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073642_ctaskarrestped_ctaskarrestped_cped_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_073648_ctaskarrestped_canarrestpedinvehicle_cped_const_ptr_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_074161_ctask_ctask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_074162_ctask_getname_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_074163_ctaskclassinfomanager_gettaskname_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_074164_cmovenetworkhelper_getclipsetid_rage_fwmvtemplateid_rage_fwmvcli(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_074168_ctask_requestprocessmovesignalcalls(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_074169_ctask_propagatenewmovespeed_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_074170_ctask_isvalidformotiontask_ctaskmotionbase_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_074172_ccliphelper_stopclip_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_075322_ctaskclassinfomanager_gettaskstatename_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_075327_ctaskcomplex_ctaskcomplex(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075328_ctaskcomplex_dtorctaskcomplex(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075332_ctaskfsmclone_ctaskfsmclone(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075333_ctaskfsmclone_readqueriablestate_cclonedfsmtaskinfo_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075334_ctaskfsmclone_isinscope_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075336_ctaskfsmclone_leavescope_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075337_ctaskfsmclone_makeabortable_rage_aitask_abortpriority_rage_aieve(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_075339_ctaskfsmclone_createsubtaskfromclone_cped_ptr_ctasktypes_etaskty(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_075340_ctaskfsmclone_checkandrequestsyncdictionary_unsigned_int_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_075341_ctaskhelperfsm_ctaskhelperfsm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075342_ctaskhelperfsm_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075343_ctaskhelperfsm_setstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_075344_cmovenetworkhelperfunctions_setmovenetworkstate_rage_fwmvrequest(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_075345_cmovenetworkhelper_sendrequest_rage_fwmvrequestid_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075346_cmovenetworkhelper_forcestatechange_rage_fwmvstateid_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075348_cmovenetworkhelper_getboolean_rage_fwmvtemplateid_rage_fwmvboole(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075349_cmovenetworkhelper_getfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_075350_ccliprequesthelper_ccliprequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075351_ccliprequesthelper_dtorccliprequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075352_ccliprequesthelper_releaseclips(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075353_ccliprequesthelper_requestclipsbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075354_cgetupsetrequesthelper_cgetupsetrequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075355_cgetupsetrequesthelper_dtorcgetupsetrequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075356_cgetupsetrequesthelper_request_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075357_cgetupprobehelper_cgetupprobehelper_float(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_075360_cgetupprobehelper_startclosestpositionsearch_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_075361_cnavmeshclosestpositionhelper_startclosestpositionsearch_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_075362_cgetupprobehelper_submitprobes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075363_cgetupprobehelper_getallresultsready(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075364_cnavmeshclosestpositionhelper_getsearchresults_searchstatus_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_075365_cgetupprobehelper_getclosestposition_rage_vector3_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_075366_cgetupprobehelper_getendposition_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_075367_cnavmeshclosestpositionhelper_resetsearch(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075368_cprioritizedclipsetrequesthelper_dtorcprioritizedclipsetrequesth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075369_cprioritizedclipsetrequesthelper_releaseclipset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075370_cprioritizedclipsetrequesthelper_isclipsetstreamedin_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075371_cprioritizedclipsetrequesthelper_getclipset_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075372_cprioritizedclipsetrequesthelper_getclipsetid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075373_cprioritizedclipsetrequesthelper_requestclipset_rage_fwmvtemplat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075376_cprioritizedclipsetrequesthelper_shouldclipsetbestreamedin_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075377_cprioritizedclipsetrequesthelper_shouldclipsetbestreamedout_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075378_cprioritizedclipsetrequesthelper_shoulduseclipset_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075379_cambientcliprequesthelper_cambientcliprequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075380_cambientcliprequesthelper_op_cambientcliprequesthelper_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075381_cambientcliprequesthelper_dtorcambientcliprequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075382_cambientcliprequesthelper_releaseclips(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075383_cambientcliprequesthelper_requestclips_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075384_cambientcliprequesthelper_chooserandomclipset_cscenariocondition(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_075385_cambientcliprequesthelper_setclipandprop_rage_fwmvtemplateid_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075386_cambientcliprequesthelper_pickclipbydict_fullconditions_int_unsi(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_075387_cambientcliprequesthelper_fillcliphasharraywithexistingclips_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075388_cambientcliprequesthelper_pickclipbygroup_simpleconditions_cdyna(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075389_cnavmeshroutesearchhelper_cnavmeshroutesearchhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075390_cnavmeshroutesearchhelper_dtorcnavmeshroutesearchhelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075391_cnavmeshroutesearchhelper_startsearch_cped_ptr_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i64, a6: i32, a7: i64, a8: f32, a9: i32); }
extern "C" { pub fn f_075392_cnavmeshroutesearchhelper_getsearchresults_searchstatus_ref_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_075397_cnavmeshlineofsighthelper_resettest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075399_cnavmeshlineofsighthelper_gettestresults_searchstatus_ref_bool_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_075400_cnavmeshlineofsighthelper_startlostest_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: f32); }
extern "C" { pub fn f_075442_cnavmeshlineofsighthelper_cnavmeshlineofsighthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075445_casyncprobehelper_casyncprobehelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075446_casyncprobehelper_dtorcasyncprobehelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075447_casyncprobehelper_resetprobe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075448_casyncprobehelper_starttestlineofsight_worldprobe_cshapetestprob(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075449_casyncprobehelper_getproberesults_probestatus_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075450_casyncprobehelper_getproberesultswithintersection_probestatus_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075453_ccliphelper_computeheadingdeltafromleft_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_075454_ccliphelper_computeheadingdeltafromright_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_075455_ccliphelper_ccliphelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075456_ccliphelper_dtorccliphelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075458_ccliphelper_startclipbysetandclip_cdynamicentity_ptr_rage_fwmvte(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32) -> i32; }
extern "C" { pub fn f_075462_cmovenetworkhelper_setclip_rage_crclip_const_ptr_rage_fwmvtempla(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075463_ccliphelper_startclipbysetandclip_cdynamicentity_ptr_rage_fwmvte(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32, a7: f32, a8: i32) -> i32; }
extern "C" { pub fn f_075465_ccliphelper_startclipbydictandhash_cdynamicentity_ptr_int_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32, a7: f32, a8: i32) -> i32; }
extern "C" { pub fn f_075467_ccliphelper_getphaseupdateamount_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_075468_ccliphelper_isevent_rage_fwmvtemplateid_rage_fwmvpropertyidtype(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075470_cmovenetworkhelper_cmovenetworkhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075472_cmovenetworkhelper_dtorcmovenetworkhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075473_cmovenetworkhelper_createnetworkplayer_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075474_cmovenetworkhelper_setfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_075475_cmovenetworkhelper_setflag_bool_rage_fwmvtemplateid_rage_fwmvfla(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_075479_cmovenetworkhelper_releasenetworkplayer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075483_csimpleblendhelper_isflagset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075484_cgenericcliphelper_cgenericcliphelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075485_cgenericcliphelper_dtorcgenericcliphelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075486_cgenericcliphelper_blendinclipbysetandclip_cdynamicentity_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_075491_cmovenetworkhelper_setfilter_rage_crframefilter_ptr_rage_fwmvtem(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075492_cchathelper_createchathelper(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_075493_cchathelper_processchat_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075494_cchathelper_finishchat(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075496_cchathelper_shouldgotochatstate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075497_cchathelper_debug_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075498_cclearancesearchhelper_cclearancesearchhelper_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075499_cmovenetworkhelper_setsubnetwork_rage_fwmovenetworkplayer_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075500_cmovenetworkhelper_setclipset_rage_fwmvtemplateid_rage_fwmvclips(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075501_cmovenetworkhelper_getclipset_rage_fwmvtemplateid_rage_fwmvclips(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075502_cmovenetworkhelper_getclip_rage_fwmvtemplateid_rage_fwmvclipidty(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_075503_cmovenetworkhelper_setboolean_rage_fwmvtemplateid_rage_fwmvboole(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075504_cmovenetworkhelper_getfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075505_cmovenetworkhelper_getboolean_rage_fwmvtemplateid_rage_fwmvboole(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075506_cmovenetworkhelper_setframe_rage_crframe_ptr_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075507_cmovenetworkhelper_getproperty_rage_fwmvtemplateid_rage_fwmvprop(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_075508_cmovenetworkhelper_setupdeferredinsert_cmovenetworkhelper_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_075509_cmovenetworkhelper_deferredinsert(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075510_creactionclipvariationhelper_creactionclipvariationhelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075511_creactionclipvariationhelper_dtorcreactionclipvariationhelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075512_creactionclipvariationhelper_update_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075513_creactionclipvariationhelper_cleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075514_cplayattachedcliphelper_cplayattachedcliphelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075523_ctuning_dtorctuning(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075537_cvehiclecliprequesthelper_debug_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075543_cvehiclejunctionhelper_approachinglight_cvehicle_const_ptr_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075547_cdirectionhelper_calculateclosestdirection_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_075548_cdirectionhelper_calculatedirectionvalue_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_075549_cvehicledrifthelper_update_float_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_075550_ctalkhelper_talk_cped_ref_ctalkhelper_options_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075551_ctalkhelper_talk_cped_ref_cped_ref_cambientaudioexchange_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075554_ctalkhelper_talk_cped_ref_cambientaudio_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075557_cupperbodyaimpitchhelper_calculateaimvector_cped_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_075558_cupperbodyaimpitchhelper_computedesiredpitchsignal_cped_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> f32; }
extern "C" { pub fn f_075559_cupperbodyaimpitchhelper_cupperbodyaimpitchhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075560_cupperbodyaimpitchhelper_blendinupperbodyaim_cmovenetworkhelper(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_075561_cupperbodyaimpitchhelper_getdefaultclipset_cped_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075563_cupperbodyaimpitchhelper_update_cped_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075564_cweaponinfohelper_calculatestate_cped_const_ref_cweaponinfohelpe(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075565_cequipweaponhelper_releaseequippedweaponanims_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075566_cequipweaponhelper_areequippedweaponanimsstreamedin_cped_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075567_cequipweaponhelper_shouldpedswapweapon_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075576_ccoverpointstatushelper_ccoverpointstatushelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075577_ccoverpointstatushelper_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075578_ccoverpointstatushelper_dtorccoverpointstatushelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075582_cnearestpedincombathelper_find_cped_const_ref_rage_fwflags_unsig(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_075583_cheadingfixuphelper_apply_cped_ref_float_rage_crclip_const_ref_f(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_075594_cscenariospeedhelper_shouldusespeedmatching_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075595_cscenariospeedhelper_getmbrtomatchspeed_cped_const_ref_cped_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_075596_ctaskmovebase_computemoveblend_cped_ref_float(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_075599_ctimehelpers_gettimesince_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_075600_clastnavmeshintersectionhelper_get_centity_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075602_chelirefuelhelper_process_cped_const_ref_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075603_chelifleehelper_createtaskforped_cped_const_ref_caitarget_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_075604_ctaskcontrolvehicle_ctaskcontrolvehicle_cvehicle_ptr_rage_aitask(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_075605_cvehicleundriveablehelper_isundriveable_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075606_cvehiclestuckduringcombathelper_isstuck_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075607_cfindclosestfriendlypedhelper_find_cped_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_075608_cfirstpersonikhelper_cfirstpersonikhelper_cfirstpersonikhelper_e(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_075609_cfirstpersonikhelper_reset_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075610_cfirstpersonikhelper_dtorcfirstpersonikhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075611_cfirstpersonikhelper_process_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075612_cfirstpersonikhelper_processlook_cped_ptr_rage_mat34v_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075614_ctargetinwaterhelper_isinwater_centity_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075615_ccreekhackhelpergtav_isaffected_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075630_ctaskmoveinterface_setmoveblendratio_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_075631_ctaskmove_ctaskmove_float(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_075632_ctaskmove_dtorctaskmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075635_ctaskmove_processstrafeinputwithrestrictions_cped_ptr_centity_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64); }
extern "C" { pub fn f_075636_ctaskmovebase_ctaskmovebase_float(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_075644_ctasktreeclone_startlocalclonetask_ctaskfsmclone_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075660_ctuning_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_075664_ctuning_registertuning_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_075693_cmotiontaskdatamanager_getdataset_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_075695_ctaskleaveanycar_ctaskleaveanycar_int_cvehicleenterexitflags_fla(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_075697_ctaskexitvehicle_ctaskexitvehicle_cvehicle_ptr_cvehicleenterexit(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_075698_ctaskvehiclefsm_getvehiclecliprequesthelper_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075702_ctaskinvehiclebasic_ctaskinvehiclebasic_cvehicle_ptr_bool_ccondi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_075710_ctaskvehiclegun_ctaskvehiclegun_ctaskvehiclegun_etaskmode_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_075712_ctaskmotioninautomobile_wantstoduck_cped_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075724_ctaskentervehicle_ctaskentervehicle_cvehicle_ptr_seatrequesttype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: f32, a6: f32, a7: i64, a8: i64) -> i64; }
extern "C" { pub fn f_075727_ctaskcardrivewander_ctaskcardrivewander_cvehicle_ptr_int_float_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_075738_ctaskcarsettempaction_ctaskcarsettempaction_cvehicle_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_075748_ccarenterexit_makeundraggedpassengerpedsleavecar_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075758_ctaskvehiclefsm_getpedinorusingseat_cvehicle_const_ref_int_cped(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_075777_ctaskcontrolvehicle_isvehicletaskunabletogettoroad_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075818_ccarenterexit_isvehiclestealable_cvehicle_const_ref_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_075820_ccarenterexit_getdoorcomponents_cvehicle_const_ptr_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_075857_ctaskvehiclefsm_shoulddestroyvehicleweaponmanager_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075870_ctaskvehiclefsm_isvehiclelockedforplayer_cvehicle_const_ref_cped(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075874_ctaskmotioninautomobile_getinvehicleclipsetid_cped_const_ref_cve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_075920_ctaskvehiclefsm_shouldwarpinandoutinmp_cvehicle_const_ref_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075937_ctaskgotocardoorandstandstill_setmoveblendratio_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_075957_ctaskvehiclefsm_processapplyforce_cmovenetworkhelper_ref_cvehicl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075988_ctaskvehiclefsm_findbestturretseatindexforvehicle_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_076001_ctaskentervehicle_warpifpossible(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076063_ctaskvehiclefsm_ikhandtoturret_cped_ref_cvehicle_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i64); }
extern "C" { pub fn f_076077_ctaskvehiclefsm_processautoclosedoor_cped_const_ref_cvehicle_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_076211_ctaskmovegotovehicledoor_ctaskmovegotovehicledoor_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: f32, a6: f32, a7: f32, a8: i32, a9: i32) -> i64; }
extern "C" { pub fn f_076227_cvehiclehullai_moveoutside_rage_vector2_ref_rage_vector2_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_076229_cvehiclehullai_calcshortestroutetotarget_cped_const_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i32, a7: f32, a8: i64, a9: f32, a10: i32) -> i32; }
extern "C" { pub fn f_076230_cvehiclehullai_liesinside_rage_vector2_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076232_cvehiclehullai_init_float_ehierarchyid_ptr_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_076291_ctaskvehiclefsm_shouldclipsetbestreamedout_rage_fwmvtemplateid_r(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076293_ctaskvehiclefsm_getexittoaimclipsetidforped_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076318_ctaskvehiclemountedweapon_ctaskvehiclemountedweapon_ctaskvehicle(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_076330_ctaskvehiclemountedweapon_gettargetinfo_cped_ptr_rage_vector3_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_076371_ctaskvehicleprojectile_ctaskvehicleprojectile_ctaskvehicleprojec(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_076386_ctaskmotioninvehicle_checkforshunt_cvehicle_ref_cped_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_076392_ctaskmotioninvehicle_ctaskmotioninvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076405_ctaskmotioninautomobile_checkifvehicleisdoingburnout_cvehicle_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076419_ctaskmotioninautomobile_shoulduselowriderleanaltclipset_bool_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_076423_ctaskmotioninautomobile_checkforhelmetvisorswitch_cped_ref_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_076518_cvehiclehullai_cvehiclehullai_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_076519_cvehiclehullai_dtorcvehiclehullai(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076655_ctaskplayerweapon_ctaskplayerweapon_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_076666_ctaskaimandthrowprojectile_ctaskaimandthrowprojectile_cweapontar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_076675_cweapontarget_getpositionwithfiringoffsets_cped_const_ptr_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_076678_ctaskaimgun_calculatedesiredpitch_cped_const_ptr_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> f32; }
extern "C" { pub fn f_076679_ctaskaimgun_convertradiantosignal_float_float_float_bool(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_076680_ctaskaimgun_smoothinput_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_076686_ctaskweaponblocked_isweaponblocked_cped_const_ptr_rage_vector3_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: i32, a8: f32) -> i32; }
extern "C" { pub fn f_076688_cweapontarget_update_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076691_cgunikinfo_applyikinfo_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076717_ctaskswapweapon_ctaskswapweapon_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_076731_ctaskgun_setblockfiringtime_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_076746_ctaskgun_ctaskgun_cweaponcontroller_weaponcontrollertype_ctaskty(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32) -> i64; }
extern "C" { pub fn f_076751_ctaskweaponblocked_ctaskweaponblocked_cweaponcontroller_weaponco(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076768_ctaskaimgun_shoulduseaimingmotion_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076769_ctaskaimgun_ctaskaimgun_cweaponcontroller_weaponcontrollertype_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_076773_ctaskaimgun_processprefsm(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076775_ctaskaimgun_cleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076778_ctaskaimgun_firegun_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_076811_ctaskaimgunonfoot_setuptofire_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_076860_ctaskgun_gettargetheading_cped_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_076865_ctaskgun_getisfiring_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076866_ctaskgun_shoulddelayafterswap_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076885_ctaskgun_setupaiminggetupset_cnmblendoutsetlist_ref_cped_ptr_cai(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_076890_ctaskaimsweep_ctaskaimsweep_ctaskaimsweep_aimtype_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_076895_ctaskreloadgun_ctaskreloadgun_cweaponcontroller_weaponcontroller(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_077089_cstatsmgr_incrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077095_cstatsmgr_isplayeractive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077153_statid_char_getstatid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077193_cstatsdatamgr_iskeyvalid_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077331_statsinterface_getintstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077373_statsinterface_getstatsmodelhashid_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077394_cstatsutils_isstatstrackingenabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077622_cvisualsettings_get_char_const_ptr_float_bool_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_077745_cdriverpersonality_findmaxcruisespeed_cped_const_ptr_cvehicle_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> f32; }
extern "C" { pub fn f_077791_cpathfind_checkpathnodeintegrityforzone_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077792_cpathfind_testalllinks_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077794_cvehicleintelligence_findspeedmultiplierwithspeedfromnodes_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_077803_cvehicleintelligence_getactivetask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077807_cpathfind_findpednodeclosesttocoors_cpathfind_cfindpednodeparams(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077812_cpathfind_findnodeclosesttocoors_rage_vector3_const_ref_float_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: f32, a11: i32, a12: i64, a13: i32, a14: i32); }
extern "C" { pub fn f_077814_cjunction_removewaitingforlightped(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077815_cjunction_removeactivelycrossingped(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077827_cjunctions_getjunctionbyindex_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077828_cjunctions_getlightstatusforped_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_077864_cpathfind_findnodeslinkindexbetween2nodes_rage_cpathnode_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_077872_cpathfind_computepositionforpedcrossreservation_rage_cnodeaddres(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_077884_cpathfind_findnthnodeclosesttocoors_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: f32, a11: i32, a12: i32, a13: i32); }
extern "C" { pub fn f_077892_cpathfind_findnodeindirection_cpathfind_findnodeindirectioninput(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077893_cpathfind_releasepedcrossreservation_rage_cnodeaddress_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077902_cpathfind_ispositionoffroadgivennodelist_cvehicle_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_077923_void_rage_atplaceptrcreator_void_ptr_cpathzonemappingarray_void(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077930_cvehicleailod_getdummymode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077936_cvehicleailodmanager_disabletimeslicingimmediately_cvehicle_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077937_cvehicleailodmanager_calcdesiredlodforpoint_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077961_cvehicleintelligence_isonsmallroad_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077970_cvehicleintelligence_getgototaskforvehicle_cvehicle_ptr_cphysica(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: f32, a6: f32) -> i64; }
extern "C" { pub fn f_077972_ctaskvehiclemissionbase_ctaskvehiclemissionbase_svehiclemissionp(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077973_ctaskvehiclegotosubmarine_ctaskvehiclegotosubmarine_svehiclemiss(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32) -> i64; }
extern "C" { pub fn f_077974_ctaskvehiclegotohelicopter_ctaskvehiclegotohelicopter_svehiclemi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_077975_cvehicleintelligence_createautomobilegototask_svehiclemissionpar(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_077977_ctaskvehicleattack_ctaskvehicleattack_svehiclemissionparams_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077982_cvehicleintelligence_createcruisetask_cvehicle_ref_svehiclemissi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_077983_ctaskvehicleflee_ctaskvehicleflee_svehiclemissionparams_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_077985_ctaskvehicleapproach_ctaskvehicleapproach_svehiclemissionparams(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077986_ctaskvehicleblock_ctaskvehicleblock_svehiclemissionparams_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077987_ctaskvehicledeaddriver_ctaskvehicledeaddriver_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077988_ctaskvehiclefollow_ctaskvehiclefollow_svehiclemissionparams_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_077989_ctaskvehiclegotolongrange_ctaskvehiclegotolongrange_svehiclemiss(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077991_ctaskvehiclegotoplane_ctaskvehiclegotoplane_svehiclemissionparam(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: f32) -> i64; }
extern "C" { pub fn f_077992_ctaskvehiclegotoboat_ctaskvehiclegotoboat_svehiclemissionparams(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_077997_ctaskvehicleplanechase_ctaskvehicleplanechase_svehiclemissionpar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_078009_ctaskvehiclepursue_ctaskvehiclepursue_svehiclemissionparams_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> i64; }
extern "C" { pub fn f_078010_ctaskvehicleram_ctaskvehicleram_svehiclemissionparams_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_078012_ctaskvehiclestop_ctaskvehiclestop_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_078013_cvehicleintelligence_multiplycruisespeedwithmultiplierandclip_fl(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i32, a4: i64) -> f32; }
extern "C" { pub fn f_078018_cheliintelligence_islanded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078054_ctaskvehiclemissionbase_ctaskvehiclemissionbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078063_ctaskvehicleconvertibleroof_getroofstate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078064_ctaskvehicleconvertibleroof_getroofprogress(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_078080_ctaskvehicleapproach_gettargetposition_rage_vec3v_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078085_ctaskvehiclemissionbase_setcruisespeed_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078095_ctaskvehiclegotohelicopter_setorientationrequested_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078166_ctaskvehiclemissionbase_gettargetposition(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078203_ctaskvehiclefleeboat_ctaskvehiclefleeboat_svehiclemissionparams(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_078224_ctaskvehiclefleeairborne_computeflightheightaboveterrain_cvehicl(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078225_ctaskvehiclefleeairborne_ctaskvehiclefleeairborne_svehiclemissio(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_078421_ctaskvehiclepullalongside_ctaskvehiclepullalongside_svehiclemiss(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i64; }
extern "C" { pub fn f_078432_ctaskvehiclepursue_setidealdistance_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078786_cvehicle_getnumcontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078886_cvehicle_canpedsstandontop_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078888_ccardoor_settargetdooropenratio_float_unsigned_int_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64); }
extern "C" { pub fn f_078889_cvehicle_getdoorhierachyidfromimpact_cvehicle_const_ref_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078918_cvehicle_setcardoorlocks_ecarlockstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078924_cvehicle_getwheelfromid_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_078932_cvehicle_getvelocityincludingreferenceframe_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078933_cvehicle_isupsidedown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078934_cvehicle_isonitsside_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078936_cvehicle_ishornon_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078939_cvehicle_switchengineon_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_078955_cvehicle_turnsirenon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_078981_cvehicle_hascontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078995_cvehicle_forcenosleep(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078998_cvehicle_switchengineoff_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079000_cvehicle_istank_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079007_cvehicle_getnumberofpassenger_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079023_cvehicle_getattachedtrailer_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079038_cvehicle_getlastdriver_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079039_cvehicle_isturretseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079041_cvehicle_rolldownwindow_ehierarchyid_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079047_cwheel_getwheelposandradius_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_079058_cvehicle_trytomakepedsfrompretendoccupants_bool_int_const_ptr_ca(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_079065_cvehicle_getdoorfromid_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079070_cvehicle_getseatanimationinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079071_cvehicle_isentryindexvalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079097_ccardoor_setflag_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079110_cvehicle_detachfromparentvehiclegadget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079117_cbike_isunderwater_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079130_cvehicle_getlayoutinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079134_cvehicle_getdriverseat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079157_cvehicle_getisjetski_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079226_cvehicle_setliveryid_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079227_cvehicle_setlivery2id_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079228_cvehicle_updatebodycolourremapping_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079266_ccardoor_clearflag_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079267_ccardoor_getisintact_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079271_ccardoor_getlocalmatrix_cvehicle_const_ptr_rage_matrix34_ref_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_079272_ccardoor_getlocalboundingbox_cvehicle_const_ptr_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_079372_crotarywingaircraft_getismainrotorbroken_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079410_cheli_generateheli_cped_ptr_unsigned_int_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_079412_cvehicle_setupdriver_bool_bool_unsigned_int_cambientmodelvariati(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_079551_cvehicle_getentryaniminfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079581_cvehicle_isindriveablestate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079643_cvehicle_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079729_cvehicle_printskeletondata(ph: *mut Phantom); }
extern "C" { pub fn f_079746_cvehicle_doesvehiclehaveaconvertibleroofanimation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079769_cvehicle_getminmaxboundsfromdummyvehiclebound_cvehicle_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_079770_cvehicle_getvehiclecoverboundoffsetinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079777_cvehiclepopulation_pickcruisespeedwithvehmodel_cvehicle_const_pt(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_079791_cvehicle_isdriver_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079793_cvehiclepopulation_addpolicevehoccupants_cvehicle_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_079796_cvehicle_hasalivepedsinit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079810_cvehicle_getdoorfromboneindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079815_cvehicle_getseatanimationinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_079816_cvehicle_getseatinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079817_cvehicle_getseatinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_079821_cvehicle_setuppassenger_int_bool_unsigned_int_cambientmodelvaria(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_079827_cvehicle_canpedentercar_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079831_cvehicle_getdirectentrypointindexforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079838_cvehicle_fleesfromcombat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079839_cvehicle_getaihandlinginfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079841_cvehicle_isonfire_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079844_cvehicle_hasraisedroof_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079854_cvehicle_containsped_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079861_cwheel_applytyredamage_centity_const_ptr_float_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_079872_cvehicle_getseathasweaponsorturrets_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079880_cvehicle_updatedoorsfornavigation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079885_cvehicle_getentryexitpoint_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079886_cvehicle_getnumberentryexitpoints_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079887_cvehicle_isentrypointindexvalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079889_cvehicle_isentrypointclearforped_cvehicle_const_ref_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_079899_cvehicle_getrelativevelocity_cped_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079930_cvehicle_getfirstturretseat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079931_cvehicle_getdirectentrypointindexforped_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079932_cvehicleweaponmgr_getfirstturretforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079933_cturret_getlocalheading_cvehicle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_079944_cvehicle_isinenterablestate_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079997_cwheel_doburstwheeltorim(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080053_cvehiclepopulation_choosemodel_bool_ptr_rage_vector3_const_ref_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_080144_cvehiclefactory_createbank(ph: *mut Phantom); }
extern "C" { pub fn f_080187_cvehiclefactory_createcar_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080237_cturret_getcurrentheading_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_080268_cvehicleweaponmgr_getfirstfixedweaponforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080403_cvehiclepopulation_catchvehicleremovedinview_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_080411_cvehiclepopulation_removeveh_cvehicle_ptr_bool_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080420_cvehiclepopulation_getrandomvehdensitymultiplier_float_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_080423_cvehiclepopulation_generatecopvehiclechasingcriminal_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_080425_rage_cpathnodelink_initiallanecenteroffset_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_080541_cvehiclemetadatamgr_getvehicledrivebyinfofromped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080542_cvehiclemetadatamgr_getdrivebyaniminfoforweapon_cped_const_ptr_u(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080543_cvehicledrivebyinfo_getminaimsweepheadingangledegs_cped_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_080544_cvehicledrivebyinfo_getmaxaimsweepheadingangledegs_cped_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_080545_cvehicleentrypointinfo_getwindowid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080639_cvehiclecoverboundinfo_isboundactive_cvehiclecoverboundinfo_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_080666_cvehiclemetadatamgr_getseataniminfofromped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080667_cvehiclemetadatamgr_getseataniminfofromseatindex_centity_const_p(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080669_cvehiclemetadatamgr_canpeddodrivebywithequipedweapon_cped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080708_cvehicledrivebyaniminfo_getfirstpersonclipset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_080713_cvehicleseataniminfo_findanim_cvehicleseataniminfo_eseatanimtype(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_080714_cvehicleseataniminfo_findaniminclipset_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_080861_cvfxasyncprobemanager_submitworldprobe_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_080864_cvfxhelper_getdistsqrtocamera_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_080866_cvfxhelper_getmatrixfromboneindex_rage_mat34v_ref_bool_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_080884_cvfxhelper_getwaterz_rage_vec3v_const_ref_float_ref_rage_fwentit(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_080886_cvfxhelper_testlineagainstwater_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_080929_cfogvolumemgr_initshaders(ph: *mut Phantom); }
extern "C" { pub fn f_081439_cfiremanager_getentityfire_centity_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081452_cfiremanager_startpedfire_cped_ptr_cped_ptr_int_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: f32, a9: i32, a10: i32, a11: i32) -> i64; }
extern "C" { pub fn f_081634_terraingrid_initshaders(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081762_cvfxblood_triggerptfxfalltodeath_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081785_cvfxentity_updateptfxentityanim_centity_ptr_cparticleattr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_081787_cvfxentity_triggerptfxentityanim_centity_ptr_cparticleattr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081797_cvfxgadget_updateptfxjetpacknozzle_cped_ptr_cobject_ptr_int_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: f32, a5: i32); }
extern "C" { pub fn f_081846_cvfxped_processvfxpedparachutepedtrails_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081966_bool_creplaymgrinternal_handleentityforfx_cpacketvehiclebadgereq(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082020_cairdefencezone_isentitytargetable_centity_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082021_cairdefencezone_shouldfireweapon_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082026_cweapon_cweapon_unsigned_int_int_cdynamicentity_ptr_bool_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_082028_cexplosionmanager_addexplosion_cexplosionmanager_cexplosionargs(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082029_cweapon_dtorcweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082030_cpedequippedweapon_getprojectileordnance(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082044_cairdefencemanager_getairdefencezone_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082045_cairdefencemanager_ispositionindefencezone_rage_vec3v_unsigned_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082047_cairdefencemanager_fireweaponatposition_unsigned_char_rage_vec3v(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_082062_cweapondamage_doweaponimpact_cweapon_ptr_centity_ptr_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i64, a6: i32, a7: i32, a8: i64, a9: i64, a10: i32, a11: i64, a12: f32, a13: f32, a14: f32, a15: i32, a16: i32, a17: i64, a18: i64, a19: i32) -> f32; }
extern "C" { pub fn f_082063_cweapon_sendfiremessage_centity_ptr_rage_vector3_const_ref_world(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i64, a11: f32, a12: f32, a13: f32, a14: i64); }
extern "C" { pub fn f_082064_cpedweaponmanager_getequippedvehicleweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082065_cpedweaponmanager_getequippedweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082069_cweapon_getmuzzleposition_rage_matrix34_const_ref_rage_vector3_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082070_cweaponinfo_getammoinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082071_cpedweaponmanager_getequippedweapon_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082123_cweapondamage_generatepeddamageevent_centity_ptr_cped_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64, a5: i64, a6: i64, a7: i32, a8: f32, a9: i64, a10: i32, a11: f32, a12: i32) -> f32; }
extern "C" { pub fn f_082159_cweapon_doreload_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082170_cprojectilemanager_createprojectile_unsigned_int_unsigned_int_ce(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: f32, a5: i32, a6: i32, a7: i64, a8: i64, a9: i32, a10: i32, a11: i32) -> i64; }
extern "C" { pub fn f_082175_cweaponinfo_getappropriateweaponclipsetid_cped_const_ptr_fpsstre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_082178_cweapon_getcooktimeleft_cped_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> f32; }
extern "C" { pub fn f_082179_cpedweaponmanager_destroyequippedweaponobject_bool_char_const_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_082182_cweapon_gettimebeforenextshot_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_082183_cweapon_calcfirevecfrompos_centity_const_ptr_rage_matrix34_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_082184_cweapon_calcfirevecfromaimcamera_centity_const_ptr_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_082185_cweapon_calcfirevecfromgunorientation_centity_const_ptr_rage_mat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_082187_cweapon_getcanreload_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082189_cweapon_getneedstoreload_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082191_cweapon_startanim_rage_fwmvtemplateid_rage_fwmvclipsetidtype_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_082192_cweapon_stopanim_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_082193_cweaponanimationsmanager_getweaponclipsethashstealth_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082198_cweaponanimationsmanager_getnumfpsfidgets_cped_const_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_082199_cweaponanimationsmanager_getfpsfidgetclipsethash_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_082201_cweaponinfo_getweaponclipsetid_cped_const_ref_fpsstreamingflags(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_082202_cweapon_receivefiremessage_centity_ptr_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i64, a14: i32) -> i32; }
extern "C" { pub fn f_082204_cweapon_setcamodiffusetexidx_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082206_cweaponinfo_getweaponclipsetid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082208_cweapon_setweaponidleflag_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082210_cweapon_stoppedfiring(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082211_cweaponinfo_getprojectilefusetime_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_082215_cpedweaponmanager_dropweapon_unsigned_int_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_082219_cweapon_addcomponent_cweaponcomponent_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_082223_cpedinventory_removeweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082252_cpedweaponmanager_getbestweaponinfo_bool_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_082263_cweaponmanager_getweaponunarmed_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082276_weaponcomponentfactory_create_cweaponcomponentinfo_const_ptr_cwe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_082424_cweaponanimationsmanager_getweaponanimationssetforped_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_082430_cweaponanimationsmanager_getfromstrafetransitionupperbodyclipset(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082431_cweaponanimationsmanager_getmotioncrouchclipsethash_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082492_cweaponinfo_shouldpedusealternatecoverclipset_cped_const_ref_con(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082493_cweaponinfo_getcovermovementclipsethashforped_cped_const_ref_fps(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082494_cweaponinfo_getalternateaimingcrouchingclipsetid_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082495_cweaponinfo_getalternateaimingstandingclipsetid_cped_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082496_cweaponinfo_getmeleebaseclipsetid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082497_cweaponinfo_getmeleeclipsetid_cped_const_ref_fpsstreamingflags_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082498_cweaponinfo_getmeleetauntclipsetid_cped_const_ref_fpsstreamingfl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082499_cweaponinfo_getmeleevariationclipsetid_cped_const_ref_fpsstreami(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082500_cweaponinfo_getpedcovermovementextraclipsetid_cped_const_ref_fps(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082501_cweaponinfo_getpedcoverweaponclipsetid_cped_const_ref_fpsstreami(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_082502_cweaponinfo_getpedmotionclipsetid_cped_const_ref_fpsstreamingfla(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_082503_cweaponinfo_getpedmotionfilterid_cped_const_ref_fpsstreamingflag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082504_cweaponinfo_getpedmotionstrafingclipsetid_cped_const_ref_fpsstre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082505_cweaponinfo_getpedmotionstrafingstealthclipsetid_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082506_cweaponinfo_getpedmotionstrafingupperbodyclipsetid_cped_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082507_cweaponinfo_getshellshockedclipsetid_cped_const_ref_fpsstreaming(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082508_cweaponinfo_getalternativeclipsetwhenblockedid_cped_const_ref_fp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082509_cweaponinfo_getswapweapondata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082510_cweaponinfo_getgesturebeckonoverride_cped_const_ref_fpsstreaming(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082511_cweaponinfo_getgestureoverthereoverride_cped_const_ref_fpsstream(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082512_cweaponinfo_getgesturehaltoverride_cped_const_ref_fpsstreamingfl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082513_cweaponinfo_getappropriatefpstransitionclipsetid_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_082514_cweaponinfo_getblindfireratemodifier_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_082515_cweaponinfo_getfireratemodifier_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_082517_cweaponinfo_getblocklefthandikwhileaiming_cped_const_ref_fpsstre(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082520_cweaponinfo_canbeusedasdriveby_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082555_cpedinventoryloadoutmanager_setloadout_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082556_cweaponswapdata_getswapclipidforweapon_int_rage_fwmvtemplateid_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_082557_cpedequippedweapon_releaseweaponanims(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082559_cpedequippedweapon_requestweaponanims(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082562_cpedequippedweapon_attachobject_cpedequippedweapon_eattachpoint(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082565_cpedequippedweapon_setupasweapon_cobject_ptr_cweaponinfo_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_082571_cpedinventory_process(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082572_cpedinventory_addweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_082580_cpedinventory_addweaponandammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_082586_cpedinventory_removeweaponandammo_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082589_cpedinventory_removeallweaponsexcept_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082590_cpedinventory_getisstreamedin_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082591_cpedinventory_getisstreamedin(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082594_cpedweaponmanager_equipweapon_unsigned_int_int_bool_bool_cpedequ(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_082600_cpedweaponmanager_createequippedweaponobject_cpedequippedweapon(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082635_cpedweaponmanager_getrequiresweaponswitch_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082640_cpedweaponselector_getbestpedweapon_cped_const_ptr_rage_atarray(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_082642_cpedweaponselector_setselectedweapon_cped_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_082644_cpedweaponmanager_equipbestweapon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_082645_cpedweaponmanager_equipbestmeleeweapon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082647_cpedweaponmanager_getbestweaponinfobygroup_unsigned_int_bool_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_082649_cpedweaponmanager_backupequippedweapon_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082650_cpedweaponmanager_setbackupweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082651_cpedweaponmanager_restorebackupweapon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082652_cpedweaponmanager_getisarmedmelee_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082653_cpedweaponmanager_getisarmedgun_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082654_cpedweaponmanager_getisarmedgunorcanbefiredlikegun_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082655_cpedweaponmanager_getisarmed1handed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082656_cpedweaponmanager_getisarmed2handed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082661_cpedweaponselector_getisnewequippableweaponselected_cped_ptr_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082664_cammoitemrepository_getisoutofammo_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082667_cinventoryitem_setmodelhash_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082671_cweaponitem_getkeyfromhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083034_networkinterface_isfriendlyfireallowed_cped_const_ptr_centity_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083037_networkinterface_allowcrimeorthreatresponseplayerjustresurrected(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083041_networkinterface_canregisterobject_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083042_networkutils_isclosetoanyplayer_rage_vector3_const_ref_float_rag(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_083047_networkutils_isvisibletoanyremoteplayer_rage_vector3_const_ref_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i32; }
extern "C" { pub fn f_083061_networkinterface_getnetworkobject_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083113_networkinterface_isinvalidvehiclefordriving_cped_const_ptr_cvehi(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083117_networkinterface_forcemotionstateupdate_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083125_networkinterface_getlastposreceivedovernetwork_cdynamicentity_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083128_networkinterface_getpredictedaccelerationvector_cdynamicentity_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083129_networkinterface_getlastheadingreceivedovernetwork_cped_const_pt(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_083131_networkinterface_useanimatedragdollfallbackblending_cped_const_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083132_networkinterface_ondesiredvelocitycalculated_cped_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083134_networkinterface_gostraighttotargetposition_cdynamicentity_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083142_networkinterface_requestnetworkanimrequest_cped_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_083144_networkinterface_removeallnetworkanimrequests_cped_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083157_networkinterface_delaymigrationfortime_cphysical_const_ptr_unsig(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083167_networkinterface_areinteractionsdisabledinmp_centity_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083178_networkinterface_setlocaldeadplayerweaponpickupandobject_cpickup(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083188_networkinterface_isremoteplayerinfirstpersonmode_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_083189_networkinterface_usefirstpersonidlemovementforremoteplayer_cped(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083201_networkinterface_writedatapopulationlog_char_const_ptr_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_083266_csyncedped_getped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083680_networkutils_getpedfromnetworkobject_rage_netobject_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083682_networkdebug_lookatandpause_unsigned_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083831_serialiseanimation_rage_csyncdatabase_ref_rage_fwmvtemplateid_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_083832_serialiseanimation_rage_csyncdatabase_ref_int_ref_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_083833_serialiseanimblend_rage_csyncdatabase_ref_float_ref_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_083849_serialisehash_rage_csyncdatabase_ref_unsigned_int_ref_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_083850_serialisepartialhash_rage_csyncdatabase_ref_unsigned_int_ref_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_083857_csyncedped_getpedid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083859_csyncedped_setpedid_unsigned_short_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083887_cnetworksynchronisedscenes_getscenerate_int_float_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_083898_csyncedentity_csyncedentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083899_csyncedentity_csyncedentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083900_csyncedentity_setentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083901_csyncedentity_csyncedentity_csyncedentity_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083902_csyncedentity_getentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083903_csyncedentity_getentityid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083904_csyncedentity_setentityid_unsigned_short_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083906_csyncedped_csyncedped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083907_csyncedvehicle_csyncedvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083908_csyncedvehicle_getvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086748_cnetworkobjectmgr_trytomakespaceforobject_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086763_cnetobjgame_getplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086860_cnetobjentity_setoverridinglocalvisibility_bool_char_const_ptr_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_086917_cnetobjentity_setoverridinglocalcollision_bool_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_086936_cnetobjentity_forcesendofpositiondata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087020_cnetobjobject_setuseparachuteblending_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087122_cnetobjped_setclonepedoutofvehicle_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087124_cnetobjped_setclonepedinvehicle_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_087125_cnetobjped_getpedstargetvehicle_int_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087127_cnetobjped_dumptaskinformation_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087136_cnetobjped_forceresendalldata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087160_cnetobjped_forceresendofalltaskdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087319_cnetobjped_updatenonplayerclonetasktarget_cped_const_ptr_caitarg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_087321_cnetobjped_takecontroloforphanedmovenetworktasksubnetwork_rage_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_087329_cnetblenderped_isstandingonobject_unsigned_short_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_087645_cnetobjplayer_setaimtarget_centity_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_089361_cweapondamageevent_trigger_centity_ptr_centity_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i32, a14: i32, a15: i32, a16: i32, a17: i64); }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090710_asinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090757_expf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090814_memcmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090817_gmtime_r(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090818_modff(ph: *mut Phantom, a0: f32, a1: i64) -> f32; }
extern "C" { pub fn f_090826_powf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090829_iprintf(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090901_qsort_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_090910_srand(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_090911_rand(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090918_sinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090921_siprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090922_small_sprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090923_sscanf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090926_strcasecmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090927_strcat(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090930_strcmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090931_strcpy(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090938_strftime(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_090939_strlen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090945_strrchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090947_strstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090955_strtod(ph: *mut Phantom, a0: i64, a1: i64) -> f64; }
extern "C" { pub fn f_090965_tanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090994_wcsstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_091100_recv(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32) -> i64; }
