//! Callee declarations for crate gen017.
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
extern "C" { pub fn f_000111_rage_syscriticalsectiontoken_lock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000113_rage_syscriticalsectiontoken_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000655_rage_atstring_lowercase(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000660_rage_atstring_indexof_char_const_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000667_rage_atstring_split_rage_atarray_rage_atstring_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_000680_rage_athashstringnamespacesupport_computehash_rage_athashstringn(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_000681_rage_athashstringnamespacesupport_addstring_rage_athashstringnam(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_000684_rage_athashstringnamespacesupport_trygetstring_rage_athashstring(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_000696_rage_bkwidget_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000703_rage_bkbank_unsetcurrentgroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000704_rage_bkbank_pushgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000705_rage_bkgroup_addgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000707_rage_bkbank_popgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000758_rage_bkmanager_openfile_char_ptr_int_char_const_ptr_bool_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_000779_rage_bkmanager_createbank_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000782_rage_bkmanager_findbank_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000783_rage_bkmanager_findwidget_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000786_rage_bkmanager_isbankthread(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000797_rage_bkwidget_setfillcolor_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000806_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000815_rage_bkmessagebox_char_const_ptr_char_const_ptr_rage_bkmessagebo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_000866_rage_bkgroup_addseparator_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000867_rage_bkgroup_addtitle_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000868_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_000869_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000870_rage_bkgroup_addbutton_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000876_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_000877_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_000880_rage_bkgroup_addtext_char_const_ptr_rage_atnonfinalhashstring_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000881_rage_bkgroup_addtext_char_const_ptr_rage_atfinalhashstring_ptr_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_000882_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000885_rage_bkgroup_addtoggle_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_000886_rage_bkgroup_addtoggle_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000894_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000898_rage_bkgroup_addslider_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000904_rage_bkgroup_addslider_char_const_ptr_short_ptr_short_short_shor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000905_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000906_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000907_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int_rage_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000918_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000919_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000926_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000927_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001062_rage_matrix34_inverse_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001105_rage_autoidregistry_autoidregistry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001108_rage_autoidregistry_unregister_rage_autoiddescriptor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001116_char_ptr_rage_formatf_char_32ul_char_ref_32ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001119_rage_datcallback_datcallback_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001514_rage_diagdisplayf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001523_quitf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001524_displayf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001527_warningf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001532_rage_fistream_close(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001533_rage_fidevice_getdevice_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001534_quitf_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001535_rage_diaglogf2_rage_diagchannel_const_ref_rage_diagseverity_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001565_printf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001568_rage_atmapentry_unsigned_int_wchar_t_ptr_const_ref_rage_atmap_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001582_rage_fiassetmanager_createleadingpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001599_rage_atmapentry_rage_sysipcthreadid_ptr_thread_closure_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001627_rage_fiassetmanager_exists_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001632_rage_fiassetmanager_pushfolder_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001633_rage_fiassetmanager_popfolder(ph: *mut Phantom); }
extern "C" { pub fn f_001635_rage_fiassetmanager_filename_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001639_rage_fiassetmanager_removeextensionfrompath_char_ptr_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001764_rage_atarray_rage_atbinarymap_int_rage_atfinalhashstring_datapai(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002018_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002019_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002020_void_std_2_sort_heap_abi_nqn220108_std_2_classicalgpolicy_bool_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002021_rage_atbinarymap_int_rage_atfinalhashstring_datapair_ptr_std_2_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002022_std_2_pair_rage_atbinarymap_int_rage_atfinalhashstring_datapair(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002023_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002027_rage_invsqrtf_fast_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_002031_rage_mthrandom_mthrandom_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002129_rage_atmapentry_rage_atnonfinalhashstring_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002131_rage_atnonfinalhashstring_atnonfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002153_rage_parmanager_openinputstream_char_const_ptr_char_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002156_rage_parmanager_initfromstructure_rage_parstructure_ref_rage_par(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002158_rage_parinstancevisitor_visittoplevelstructure_rage_parparsables(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_002206_rage_parelement_findattributeint64value_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002294_rage_atarray_rage_psorscenumschemavaluedata_0_unsigned_short_gro(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002311_rage_atarray_rage_psoenumtableentry_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
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
extern "C" { pub fn f_002446_rage_parcgustructure_addoneextraattribute_rage_parattributelist(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002447_rage_parinstancevisitor_parinstancevisitor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002479_rage_parxmlwritervisitor_parxmlwritervisitor_rage_fistream_ptr_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002648_rage_parstream_close(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002721_rage_conststring_dtorconststring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002739_rage_atpartialstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002808_rage_atdelegator1_void_rage_sysserviceevent_ptr_removedelegate_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002863_rage_sysperformancetimer_sysperformancetimer_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002865_rage_sysperformancetimer_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002866_rage_sysperformancetimer_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002867_rage_sysperformancetimer_gettimems(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_002945_rage_sysmempoolallocator_safeallocate_unsigned_long_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002946_rage_sysmempoolallocator_allocate_unsigned_long_unsigned_long_bo(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002947_rage_sysmempoolallocator_safefree_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002948_rage_sysmempoolallocator_free_void_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003097_char_ptr_rage_formatf_char_16ul_char_ref_16ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003159_rage_matrix34_fromquaternion_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003160_rage_matrix34_toquaternion_rage_quaternion_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003161_rage_quaternion_frommatrix34_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003162_rage_matrix34_interpolate_rage_matrix34_const_ref_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_003170_rage_quaternion_slerp_float_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003172_rage_quaternion_twistangle_rage_vector3_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_003176_rage_vector3_approachstraight_rage_vector3_const_ref_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003296_rage_atarray_unsigned_short_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003826_rage_fwanimdirector_setposed_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003830_rage_crtagiteratorbase_rage_crtagiteratorkey_true_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003832_rage_fwanimdirectorcomponentsyncedscene_isvalidsceneid_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003835_rage_fwanimdirector_getmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003838_rage_fwclipset_getclip_rage_fwmvtemplateid_rage_fwmvclipidtype_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_003840_rage_fwclipsetmanager_getclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003848_rage_fwanimdirector_isnetworkdefloaded_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003856_rage_fwanimdirector_waitforprephysicsupdatetocomplete(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003876_rage_fwanimmanager_getclipifexistsbysetid_rage_fwmvtemplateid_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003878_rage_fwanimhelpers_getmovertracktranslationdiff_rage_crclip_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003880_rage_fwanimhelpers_getmovertrackrotation_rage_crclip_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003882_rage_fwanimhelpers_getmovertrackrotationdiff_rage_crclip_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003884_rage_fwanimhelpers_getmovertrackrotationdelta_rage_crclip_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003886_rage_fwanimhelpers_getmovertrackmatrixdelta_rage_crclip_const_re(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64); }
extern "C" { pub fn f_003887_rage_fwanimhelpers_getmovertrackdisplacement_rage_crclip_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003888_rage_fwanimhelpers_getmovertracktranslationdiffrotated_rage_crcl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_003892_rage_fwanimhelpers_applyinitialoffsetfromclip_rage_crclip_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_003896_rage_fwanimmanager_getclipifexistsbydictindex_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_003900_rage_fwclipsetmanager_getclipdictionaryindex_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003905_rage_fwanimmanager_findslotfromhashkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003909_rage_fwanimmanager_getname_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_003915_rage_fwanimmanager_get_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_003966_rage_fwclipset_getclipitembyindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003967_rage_fwclipset_getclipitem_rage_fwmvtemplateid_rage_fwmvclipidty(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003968_rage_fwclipset_getclipdictionaryindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003971_rage_fwclipset_getclipitemidbyindex_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_003972_rage_fwclipset_pickrandomclip_rage_fwmvtemplateid_rage_fwmvclipi(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_003974_rage_atarray_rage_atbinarymap_rage_fwclipitem_ptr_rage_fwmvtempl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003977_rage_fwclipset_request_deprecated_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_003984_rage_fwclipsetrequesthelper_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003991_rage_fwclipsetmanager_getclipsetidbyindex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003994_rage_fwclipsetmanager_getclipitem_rage_fwmvtemplateid_rage_fwmvc(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003995_rage_fwclipsetmanager_getclip_rage_fwmvtemplateid_rage_fwmvclips(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003996_rage_fwclipsetmanager_pickrandomclip_rage_fwmvtemplateid_rage_fw(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_004008_rage_fwclipsetmanager_streamin_deprecated_rage_fwmvtemplateid_ra(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004009_rage_fwclipsetmanager_request_deprecated_rage_fwmvtemplateid_rag(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004010_rage_fwclipsetmanager_isstreamedin_deprecated_rage_fwmvtemplatei(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004011_rage_fwclipsetmanager_addref_deprecated_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004012_rage_fwclipsetmanager_release_deprecated_rage_fwmvtemplateid_rag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004019_rage_fwclipsetrequesthelper_request_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_004037_rage_fwclipsetrequesthelper_dtorfwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004038_rage_fwclipsetrequesthelper_fwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004039_rage_fwclipsetrequesthelper_request_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004041_rage_fwclipsetrequesthelper_isloaded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004283_rage_fwposematcher_findbestmatch_rage_crskeleton_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_004284_rage_fwposematcher_findsample_rage_fwmvtemplateid_rage_fwmvclips(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_004390_rage_fwanimdirectorcomponentsyncedscene_decsyncedsceneref_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004392_rage_fwanimdirectorcomponentsyncedscene_addsyncedsceneref_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004398_rage_fwanimdirectorcomponentsyncedscene_getsyncedscenephase_shor(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004401_rage_fwanimdirectorcomponentsyncedscene_setsyncedsceneorigin_sho(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_004406_rage_fwanimdirectorcomponentsyncedscene_setsyncedscenelooped_sho(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_004408_rage_fwanimdirectorcomponentsyncedscene_setsyncedsceneholdlastfr(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_004416_rage_fwanimdirectorcomponentsyncedscene_startsynchronizedscene_r(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004576_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_004577_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004578_void_std_2_sort_heap_abi_nqn220108_std_2_classicalgpolicy_bool_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004579_rage_atbinarymap_rage_fwclipitem_ptr_rage_fwmvtemplateid_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_004580_std_2_pair_rage_atbinarymap_rage_fwclipitem_ptr_rage_fwmvtemplat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004581_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_004603_rage_fwextensionlist_add_rage_fwextension_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004619_rage_fwarchetype_getdrawable_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004620_rage_fwarchetype_getfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004632_rage_fwarchetypemanager_lookupmodelid_rage_fwarchetype_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004633_rage_fwarchetypemanager_getarchetype_rage_atnonfinalhashstring_u(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004634_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004661_rage_fwdynamicentitycomponent_fwdynamicentitycomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004684_rage_fwentity_isentityaparentattachment_rage_fwentity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004688_rage_fwextensionlist_destroy_rage_fwextension_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004767_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004788_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004792_rage_fwpickermanager_getselectedentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004793_rage_fwpickermanager_getentity_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004794_rage_fwpickermanager_getnumberofentities(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_004820_rage_fwvectormap_drawcircle_rage_vector3_const_ref_float_rage_co(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_005196_rage_ropeinstance_setphysinstflags_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005203_rage_ropeinstance_attachobjectstoconstraintarray_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32) -> i64; }
extern "C" { pub fn f_005207_rage_ropeinstance_attachtoobject_rage_vec3v_const_ref_rage_phins(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_005209_rage_ropeinstance_setnewuniqueid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005219_rage_ropeinstance_isattached_rage_phinst_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005220_rage_ropemanager_findrope_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005225_rage_ropemanager_add_rage_vec3v_const_ref_rage_vec3v_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32, a11: f32, a12: i32, a13: i32) -> i64; }
extern "C" { pub fn f_005461_rage_fwboxstreamer_hasloadedaboutpos_rage_vec3v_const_ref_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_005462_rage_fwboxstreamer_hasloadedwithinaabb_rage_spdaabb_const_ref_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_005489_rage_fwassetstore_rage_crclipdictionary_rage_fwclipdictionarydef(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005648_rage_fwpsostoreloader_load_char_const_ptr_rage_fwpsostoreloadins(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_005651_rage_fwpsostoreloader_makesimplefileloadercore(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005733_rage_fwinteriorlocation_getroomindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005919_rage_aitask_setstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005921_rage_aitask_findparenttaskoftype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005922_rage_aitask_findsubtaskoftype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005924_rage_aitask_picknewstatetransition_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_005926_rage_aitask_setnewtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005939_rage_aitaskstatetransitiontable_addtransition_int_int_float_long(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i64); }
extern "C" { pub fn f_005946_rage_aitasktree_cleartask_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005947_rage_aitasktree_getactiveleaftask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005948_rage_aitasktree_gethastask_rage_aitask_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005949_rage_aitasktree_findtaskbytypeactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005950_rage_aitasktree_findtaskbytypewithpriority_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_006084_rage_strstreaminginfomanager_requestobject_rage_strindex_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006085_rage_strpackfilemanager_registerindividualfile_char_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_006086_rage_strpackfilemanager_invalidateindividualfile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006088_rage_atarray_rage_atnonfinalhashstring_0_unsigned_short_grow_uns(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006156_rage_atarray_rage_strindex_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006167_rage_strstreaminginfomanager_isobjectinimage_rage_strindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006175_rage_strstreaminginfomanager_clearrequiredflag_rage_strindex_uns(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006190_rage_strstreaminginfomanager_unregisterobject_rage_strindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006225_rage_atvarstring_atvarstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_006252_rage_strstreamingmodule_streamingblockingload_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006254_rage_strrequest_request_rage_strlocalindex_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_006255_rage_strrequest_hasloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006256_rage_strrequest_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006257_rage_strrequest_clearrequiredflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006412_rage_aitask_debug_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006743_rage_audspeechsound_findnumvariationsforvoiceandcontext_unsigned(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_007078_rage_audsoundset_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007079_rage_audsoundset_find_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007419_rage_vfxlistitem_vfxlistitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007426_rage_vfxbrokenfraginfo_dtorvfxbrokenfraginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007679_rage_fwangle_getnodeheadingfromvector_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> i32; }
extern "C" { pub fn f_007694_rage_fwgeom_intersectsphereray_rage_spdsphere_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_007759_rage_netinterface_getnumphysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007762_rage_netinterface_getallphysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007771_rage_netobjectmgrbase_getnetworkobject_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007979_rage_netserialiserutils_getadjustedworldextents_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008074_rage_netobject_ispendingownerchange_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008076_rage_netobject_setlocalflag_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_008085_rage_netobject_getclonedstate_unsigned_int_ref_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_008088_rage_netobject_ispendingcloning_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008230_rage_netblenderlininterp_getlastpositionreceived_unsigned_int_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008232_rage_netblenderlininterp_updateposition_rage_vector3_const_ref_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008682_rage_fwbasepool_fwbasepool_int_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_008683_rage_fwconfigmanager_getsizeofpool_unsigned_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008688_rage_fwbasepool_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008866_rage_atmapentry_unsigned_int_int_const_ref_rage_atmap_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008872_rage_fwfsm_fwfsm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008873_rage_fwfsm_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008897_rage_fwmetadatastore_removeunrequired(ph: *mut Phantom); }
extern "C" { pub fn f_008908_rage_fwmetadatastore_removeloaded_rage_strlocalindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008913_rage_fwmetadatastore_makeslotstreamable_rage_atnonfinalhashstrin(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008914_rage_fwmetadatastore_removeslot_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008950_rage_fwbasepool_getjustindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008951_rage_fwbasepool_isvalidptr_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008953_rage_fwknownrefholder_fwknownrefholder_void_ptr_ptr_rage_fwknown(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_009048_rage_crproperty_getattribute_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009211_rage_crskeletondata_computeobjecttransform_int_rage_mat34v_ref_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009233_rage_crskeleton_getglobalmtx_unsigned_int_rage_mat34v_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010179_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010180_rage_grcdebugdraw_quad_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_010184_rage_grcdebugdraw_circle_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010192_rage_grcdebugdraw_cone_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010193_rage_grcdebugdraw_capsule_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010195_rage_grcdebugdraw_arrow_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_010219_rage_grcdebugdraw_adddebugoutputex_bool_rage_color32_char_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_010224_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010225_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010500_rage_iovalue_isdown_float_rage_iovalue_readoptions_const_ref_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010511_rage_iovalue_ispressed_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010514_rage_iovalue_isreleased_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010518_rage_iovalue_getnorm_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010721_rage_spdgrid2d_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_010723_void_rage_atplaceptrcreator_void_ptr_rage_spdgrid2d_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010724_rage_spdgrid2d_initcalculatedmembers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010725_rage_spdgrid2d_init_float_float_float_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32); }
extern "C" { pub fn f_010726_rage_spdgrid2d_testcircle_float_float_float_int_int_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_010737_rage_spdaabb_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_010743_rage_spdsphere_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_011734_rage_fraginst_getarticulatedbody(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011745_rage_fraginst_setresetmatrix_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011748_rage_fraginst_getpartbroken_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011751_rage_fraginst_getcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011776_rage_fragtype_getboneindexfromid_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011822_rage_matrix34_dot_rage_matrix34_const_ref_rage_matrix34_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011876_rage_fragphysicslod_gettotalundamagedmass_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_011964_rage_fragtype_getskeletondata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012021_rage_fragphysicslodgroup_getlodbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012843_rage_environmentcloth_getcloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012875_rage_environmentcloth_controlvertex_int_rage_vec3v_const_ref_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_014067_rage_nettimesync_gettime_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014068_rage_nettimesync_hasstarted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014164_rage_nettransactor_isinitialized_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_015882_rage_phconstraintmgr_insert_rage_phconstraintbase_params_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015890_rage_phconstraintmgr_remove_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015891_rage_phconstraintmgr_gettemporarypointer_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015966_rage_phsleep_reset_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015969_rage_phcollider_setreferenceframevelocity_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015980_rage_phcollider_getlocalvelocityrigid_float_vector_4_const(ph: *mut Phantom, a0: i64, a1: i64, a2: u128); }
extern "C" { pub fn f_015982_rage_phcollider_applyimpulsecenterofmassrigid_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016130_rage_phshapetest_rage_phshapeprobe_phshapetest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016137_rage_phshapetest_rage_phshapeprobe_testoneobject_rage_phinst_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016172_rage_phshapetest_rage_phshapesweptsphere_phshapetest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016174_rage_phshapetest_rage_phshapesweptsphere_testinlevel_rage_phinst(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_016195_rage_phshapetest_rage_phshapeprobe_initprobe_rage_phsegment_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_016197_rage_phshapetest_rage_phshapesweptsphere_initsweptsphere_rage_ph(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_016256_rage_pharticulatedbody_getlinearvelocity_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016257_rage_pharticulatedbody_resetdownvel_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016258_rage_pharticulatedbody_seteffectorstozeropose(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016262_rage_pharticulatedbody_setbodyminimumstiffness_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016263_rage_pharticulatedbody_setstiffness_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016264_rage_pharticulatedbody_resetpropagationvelocities(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016267_rage_pharticulatedbody_ensurevelocitiesfullypropagated(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016269_rage_pharticulatedbody_setdrivestate_rage_phjoint_drivestate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016271_rage_pharticulatedbody_applyincrementalimpulse_int_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_016285_rage_phjoint_setminstiffness_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016286_rage_pharticulatedbody_applygravitytolinks_float_vector_4_float(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: f32, a4: i32); }
extern "C" { pub fn f_016333_rage_pharticulatedcollider_getlocalvelocityart_float_vector_4_in(ph: *mut Phantom, a0: i64, a1: i64, a2: u128, a3: i32); }
extern "C" { pub fn f_016391_rage_phboundbox_phboundbox_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016606_rage_phverletcloth_dynamicunpinvertex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016607_rage_phverletcloth_dynamicunpinall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016635_rage_phintersection_getinstance(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016636_rage_matrix34_untransform_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016910_rage_phlevelnodetree_rage_phlooseoctreenode_testsphere_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_016911_rage_phlevelnodetree_rage_phlooseoctreenode_findandaddoverlappin(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_017261_rage_atarray_rage_vec3v_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018889_cleveldata_getfriendlyname_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_018921_rage_fwentity_getmodelid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019102_cpopgenshape_cpopgenshape(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019104_cpopgenshape_categorisepoint_rage_vector2_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019288_caitarget_caitarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019289_caitarget_caitarget_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019290_caitarget_setentity_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019292_caitarget_setentityandoffset_centity_const_ptr_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019293_caitarget_caitarget_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019294_caitarget_setposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019295_caitarget_caitarget_caitarget_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019296_caitarget_getisvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019297_caitarget_getposition_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019299_caitarget_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019301_csyncedentity_serialise_rage_csyncdatabase_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019303_canimblackboard_additemtoblackboardandpedintelligenceifallowedto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_019307_cscenarioblockingareas_ispointinsideblockingareas_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_019308_cscenarioblockingareas_shouldscenarioquitfromblockingareas_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_019314_centityscanner_scanforentitiesinrange_centity_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019342_cspatialarray_cspatialarray_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_019343_cspatialarray_insert_cspatialarraynode_ref_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019345_cspatialarray_update_cspatialarraynode_ref_float_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_019347_cspatialarray_findinsphere_rage_vec3v_const_ref_rage_scalarv_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_019348_cspatialarray_findinsphereoftype_rage_vec3v_const_ref_rage_scala(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_019369_cambientmodelset_getcontainsmodel_unsigned_int_cambientmodelvari(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_019370_cambientmodelset_getrandommodelhash_float_cambientmodelvariation(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_019371_cambientmodelset_getrandommodelindex_cambientmodelvariations_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_019373_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_019387_cambientmodelsetmanager_getmodelsetindexfromhash_cambientmodelse(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_019392_cambientmodelsetmanager_addwidgetsformodelset_rage_bkbank_ref_ca(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019419_cconditionalanims_checkconditions_cscenariocondition_sscenarioco(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_019422_cconditionalanims_chooseclipset_rage_atarray_cconditionalclipset(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019423_cconditionalanimsgroup_hasanimationdata_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019424_cconditionalanimsgroup_checkformatchingconditionalanims_cscenari(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019425_cconditionalanimsgroup_checkformatchingpopulationconditionalanim(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019426_cconditionalanimsgroup_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_019442_cconditionalanimmanager_addparserwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019452_ailogging_getdynamicentityisclonestringsafe_cdynamicentity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019453_ailogging_getdynamicentitynamesafe_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019462_cailogmanager_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019565_cpedbonetagconvertor_getboneindexfrombonetag_rage_crskeletondata(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_019569_cgtaanimmanager_findframefilter_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019599_cfacialdatacomponent_resetfacialidleanimation_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019609_cfacialdatacomponent_playfacialanim_cped_ptr_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019611_cfacialdatacomponent_playpainfacialanim_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019614_cfacialdatacomponent_requestfacialidleclip_cfacialdatacomponent(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019634_cmoveped_settasknetwork_rage_fwmovenetworkplayer_ptr_float_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_019650_cmoveped_playhandanimation_rage_crclip_const_ptr_float_cmoveped(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_019652_cmoveped_settasknetwork_cmovenetworkhelper_ref_float_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_019653_cmoveped_cleartasknetwork_cmovenetworkhelper_ref_float_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_019654_cmoveped_setsecondarytasknetwork_cmovenetworkhelper_ref_float_un(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_019655_cmoveped_clearsecondarytasknetwork_cmovenetworkhelper_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_019667_cmoveped_switchtoanimated_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_019726_cclipeventtags_cmoverfixupeventtag_shouldfixuptranslation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019727_cclipeventtags_cmoverfixupeventtag_shouldfixuprotation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019728_cclipeventtags_cmoverfixupeventtag_shouldfixupx_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019729_cclipeventtags_cmoverfixupeventtag_shouldfixupy_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019730_cclipeventtags_cmoverfixupeventtag_shouldfixupz_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_020085_rage_atbinarymap_float_unsigned_long_insert_unsigned_long_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_020090_canimviewer_blendoutcapture_cdynamicentity_const_ptr_cnmblendout(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_021209_audpolicescanner_reportplayercrime_rage_vector3_const_ref_ecrime(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_021351_cped_shouldbedead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021359_auddamagestats_dtorauddamagestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021365_audspeechaudioentity_setblockfallingvelocitycheck(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021366_audspeechaudioentity_inflictpain_auddamagestats_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021374_audpedaudioentity_isbeingelectrocuted_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021385_audspeechaudioentity_getpedvoicegroup_bool_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_021401_audpedaudioentity_onscenariostarted_int_cscenarioinfo_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_021403_audspeechaudioentity_stopspeech(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021405_audspeechaudioentity_ispainplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021407_audspeechaudioentity_isambientspeechplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021464_audpedfootstepaudio_startladderslide(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021465_audpedfootstepaudio_stopladderslide(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021598_audvehicleaudioentity_isradioenabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021778_audspeechmanager_getvoiceforphoneconversation_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021844_audscriptaudioentity_registerthrownfromvehicle_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021928_audspeechaudioentity_doescontextexistforthisped_unsigned_int_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_021948_audspeechaudioentity_triggerpainfrompainid_int_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021963_audspeechaudioentity_panic_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021964_audspeechaudioentity_registerfallingstart(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022077_rage_fwentity_getobjectmtx_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022381_void_creplayeventmanager_storeevent_cpacketsoundphonering_cpacke(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022397_naaudioentity_createdeferredsound_rage_audmetadataref_centity_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_022651_audcollisionaudioentity_getmaterialfromshapetestsafe_worldprobe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022658_cped_isdead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022985_cpathfind_findnodepointersafe_rage_cnodeaddress(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023418_cpacketaddrope_cpacketaddrope_int_rage_vec3v_const_ref_rage_vec3(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32, a7: f32, a8: i32, a9: i32, a10: i32, a11: i32, a12: f32, a13: i32, a14: i32) -> i64; }
extern "C" { pub fn f_023423_cpacketattachropetoentity_cpacketattachropetoentity_int_rage_vec(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_023428_cpacketdetachropefromentity_cpacketdetachropefromentity_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_023432_cpacketrappelpinrope_cpacketrappelpinrope_int_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_023434_cpacketunpinrope_cpacketunpinrope_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_023437_cpacketropewinding_cpacketropewinding_float_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_023849_creplaymgr_replayrecordsound_unsigned_int_unsigned_int_rage_auds(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_023881_creplaymgr_hash_unsigned_int_const_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023906_creplaycontrol_isplayeroutofcontrol(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023927_void_creplayeventmanager_trackrecordingevent_unsigned_long_ctrac(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_023930_creplayeventmanager_shouldproceedwithpacket_cpacketevent_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023998_rage_fwmodelid_fwmodelid_rage_fwmodelid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_024266_creplaymgrinternal_isawareofworldforrecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026172_caminterface_getplayercontrolcamheading_cped_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026174_camgameplaydirector_getfirstpersonshootercamera_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026199_caminterface_isfadedin(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026202_caminterface_isspherevisibleingameviewport_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_026205_cqueriableinterface_istaskcurrentlyrunning_int_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_026211_worldprobe_cshapetestfixedresults_unsigned_char_32_cshapetestfix(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026219_cdynamicentity_getboneindexfrombonetag_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026248_rage_vector3_normalizesafe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026259_rage_fwentity_getskeleton_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026293_rage_fwmatrixtransform_getpitch_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026327_rage_matrix34_dot3x3transpose_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026329_worldprobe_cshapetestresults_dtorcshapetestresults(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026367_rage_imp_transform_imp34_rage_mat34v_ref_float_vector_4_float_ve(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: u128, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_026623_rage_matrix34_toeulers_rage_vector3_ref_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026649_camdebugdirector_getcamframenonconst_cambasecamera_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027008_ctrain_getlinkedtobackward_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027009_ctrain_getlinkedtoforward_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027093_rage_matrix34_fromeulers_rage_vector3_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029377_ctask_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029433_getorcreategroup_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029455_cdebugscene_tunewidgetdata_tunewidgetdata_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_029456_cdebugscene_queuetunewidget_cdebugscene_tunewidgetdata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029612_worldprobe_cshapetestdesc_setresultsstructure_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029613_worldprobe_cshapetestresults_freeintersections(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029711_void_std_2_introsort_std_2_classicalgpolicy_alphabeticalsort_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_029866_cdebugscene_focusentities_get_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029921_cdebugdrawstore_cdebugdrawstore_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_029924_cdebugdrawstore_addline_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_029927_cdebugdrawstore_addsphere_rage_vec3v_const_ref_float_rage_color3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_029929_cdebugdrawstore_addtext_rage_vec3v_const_ref_int_int_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_029936_cdebugdrawstore_clear_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_029965_rage_fwtransform_getb_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030722_ceventgroupped_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030724_cevent_cancreateevent_unsigned_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_030728_ceventgroupped_gethighestpriorityevent_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030730_ceventgroupped_hasragdollevent_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030739_cpedintelligence_addevent_cevent_const_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030741_geteventglobalgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030756_ceventhandlerhistory_getcurrentevent_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030763_ceventhandler_getcurrenteventtype_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030766_ceventnames_geteventname_eeventtype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_030769_caitarget_dtorcaitarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030770_cshockingeventsmanager_add_ceventshocking_ref_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_030772_ceventshocking_dtorceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030790_ceventeditableresponse_findtasktypeforresponse_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_030801_cevent_cevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030809_cevent_communicateevent_cped_ptr_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_030819_cped_isinjured_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030820_ceventrequesthelp_ceventrequesthelp_cped_ptr_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_030821_ceventrequesthelp_communicateevent_cped_ptr_cevent_communicateev(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_030822_ceventrequesthelp_dtorceventrequesthelp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030824_cpedvariationinfocollection_hascomponentflagsset_epedvarcomp_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_030826_cpedintelligence_gettargetting_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030847_ceventshockingpedknockedintobyplayer_ceventshockingpedknockedint(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_030848_ceventdeath_ceventdeath_bool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_030849_ceventdeath_ceventdeath_bool_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_030851_ceventdeath_dtorceventdeath(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030938_ceventreactionenemyped_ceventreactionenemyped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_031040_ceventacquaintanceped_ceventacquaintanceped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031069_ceventgivepedtask_ceventgivepedtask_int_rage_aitask_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_031070_ceventgivepedtask_dtorceventgivepedtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031086_ceventinair_ceventinair_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031087_ceventinair_dtorceventinair(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031116_ceventcrimecryforhelp_createandcommunicateevent_cped_ptr_cped_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_031158_ceventfirenearby_dtorceventfirenearby(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031165_ceventstaticcountreachedmax_ceventstaticcountreachedmax_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_031169_ceventswitch2nm_ceventswitch2nm_unsigned_int_rage_aitask_ptr_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_031170_ceventswitch2nm_dtorceventswitch2nm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031200_ceventagitatedaction_ceventagitatedaction_cped_const_ptr_rage_ai(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031201_ceventagitatedaction_dtorceventagitatedaction(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031205_ceventscenarioforceaction_ceventscenarioforceaction(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031206_ceventscenarioforceaction_dtorceventscenarioforceaction(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031208_ceventscenarioforceaction_settask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_031235_ceventshocking_ceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031239_ceventshocking_getreactposition_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_031241_ceventshocking_canbesensedby_cped_const_ref_float_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_031242_ceventshocking_set_rage_vec3v_const_ref_centity_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_031364_ceventfootstepheard_ceventfootstepheard_cped_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_031400_svehiclemissionparams_settargetentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_031606_ceventacquaintanceped_dtorceventacquaintanceped_54301(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031635_ceventreactionenemyped_dtorceventreactionenemyped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031643_ceventreactioninvestigatedeadped_dtorceventreactioninvestigatede(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031646_ceventreactioninvestigatedeadped_ceventreactioninvestigatedeadpe(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031880_ceventsoundbase_dtorceventsoundbase_54685(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035277_cminimap_createsonarblipandreportstealthnoise_cped_ptr_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32) -> f32; }
extern "C" { pub fn f_037657_cwanted_setwantedlevel_rage_vector3_const_ref_int_int_setwantedl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_037659_cseatmanager_countpedsinseats_bool_bool_bool_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_037661_cmodelinfo_requestassets_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037662_cmodelinfo_haveassetsloaded_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037684_cclock_istimeinrange_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_037761_ccrime_reportcrime_ecrimetype_centity_ptr_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_037767_cwanted_setwantedlevelnodrop_rage_vector3_const_ref_int_int_setw(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_037888_cseatmanager_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_037910_cwanted_reportwitnesssuccessful_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037912_cwanted_reportwitnessunsuccessful_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037913_cwanted_registerhostileaction_cped_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038161_cdispatchhelpersearch_calculate_cped_const_ref_cped_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_038164_cdispatchhelpersearchonfoot_cdispatchhelpersearchonfoot(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038165_cdispatchhelpersearchonfoot_isresultready_bool_ref_rage_vec3v_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_038166_cdispatchhelpersearchonfoot_start_cped_const_ref_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_038167_cdispatchhelpersearchonfoot_getmoveblendratio_cped_const_ref(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_038170_cdispatchhelpersearchinautomobile_calculate_cped_const_ref_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_038171_cdispatchhelpersearchinautomobile_getdrivingflags(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_038176_cdispatchhelpersearchinheli_calculate_cped_const_ref_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_038180_cdispatchhelperforincidentlocationonfoot_getpositionforped_cped(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038181_corder_getincident_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038182_cdispatchhelperforincidentlocationinvehicle_getcruisespeed_cped(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038183_cdispatchhelperforincidentlocationinvehicle_getdrivingflags(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_038197_cdispatchmanager_debugenabled_dispatchtype(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_038210_cdispatchservice_islocationwithinsearcharea_rage_vector3_const_r(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038222_corder_getisvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038243_cpoliceautomobiledispatch_isordervalidforped_cpoliceorder_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038244_cpoliceautomobiledispatch_requestneworder_cped_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038258_cseatmanager_getlastpedsseatindex_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038345_cfireincident_getbestfire_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_038373_corder_setentityfinishedwithincident(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038376_corder_setentityarrivedatincident(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038377_corder_getallresourcesaredone_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038378_corder_getincidenthasbeenaddressed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038379_corder_setincidenthasbeenaddressed_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038382_cpedintelligence_addtaskdefault_rage_aitask_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038383_cambulanceorder_getinjuryincident_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038384_cambulanceorder_getrespondingped_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038385_cfireorder_getfireincident_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038386_cfireorder_getfiretruck_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038387_cfireorder_getbestfire_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_038388_cfireorder_getentitythathasbeenonfire_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038389_cfireorder_getnumfires_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038390_cfireorder_incfightingfirecount_cfire_ptr_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_038391_cfireorder_decfightingfirecount_cfire_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038467_cbaseikmanager_getsolver_ikmanagersolvertypes_flags_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038495_cikmanager_lookat_unsigned_int_centity_const_ptr_int_eanimboneta(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32); }
extern "C" { pub fn f_038497_cikmanager_islooking(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038499_cikmanager_abortlookat_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038503_cikmanager_istargetwithinfov_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_038505_cikmanager_request_cikrequestbodylook_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038506_cikmanager_setabsolutearmiktarget_rage_criksolverarms_earms_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: f32, a5: f32, a6: f32, a7: f32); }
extern "C" { pub fn f_038507_cikmanager_setrelativearmiktarget_rage_criksolverarms_earms_cdyn(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32, a6: f32, a7: f32, a8: f32, a9: f32); }
extern "C" { pub fn f_038532_cikmanager_addrootslopefixup_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_038534_cikmanager_setoverridelegikblendtime_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_038535_cikmanager_resetoverridelegikblendtime(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038537_cikrequestbodylook_cikrequestbodylook(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038538_cikrequestbodylook_cikrequestbodylook_centity_const_ptr_eanimbon(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_038633_cbasemodelinfo_get2deffect_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038634_cbasemodelinfo_getnum2deffectsnolights_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038714_cmodelinfo_areassetsrequested_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038716_cmodelinfo_getassetsaredeletable_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038733_centryexitpoint_getseat_seataccess_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_038734_centryexitpoint_getentrypointposition_centity_const_ptr_cped_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_038739_cmodelseatinfo_getentrypointindexforseat_int_centity_const_ptr_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_038740_cmodelseatinfo_getboneindexfromseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038747_cmodelseatinfo_evaluateseataccessfromallentrypoints_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64, a7: i32) -> i32; }
extern "C" { pub fn f_038751_cseatmanager_getpedsseatindex_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038757_cseatmanager_getlastpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038863_cpedmodelinfo_getpersonalitybravery_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_038981_cvehiclevariationinstance_togglemod_evehiclemodtype_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_039216_cvehiclevariationinstance_setmodindexfortype_evehiclemodtype_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_039576_cdoor_getdooropenratio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_039627_cobject_initanimlazy(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039630_cweapon_sfireparams_dtorsfireparams(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039688_cobjectpopulation_destroyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039761_cobjectpopulation_createobject_rage_fwmodelid_eentityownedby_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_039877_cpathserver_isrequestresultready_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_039879_cpathserver_requestpath_rage_vector3_const_ref_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64) -> i32; }
extern "C" { pub fn f_039882_cpathserver_getclosestpositionforped_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_039885_cpathserver_addblockingobject_rage_vector3_const_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_039902_cpathserver_lockpathresult_unsigned_int_int_ptr_rage_vector3_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_039903_cpathserver_unlockpathresultandclear_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_039913_cpathserver_cancelrequest_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_040002_cpathserver_removeblockingobject_unsigned_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_040024_rage_normalizeandmag_rage_vector3_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_040139_cpedgroupmembership_getmember_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_040142_cpedgroup_islocallycontrolled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040159_cpedgroupmembership_sortbydistancefromleader(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040180_cpedgroup_setformation_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_040194_cpedgroups_getpedsgroup_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040201_carcadepassiveabilityeffects_getisactive_earcadeabilityeffecttyp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_040203_carcadepassiveabilityeffects_getmodifier_earcadeabilityeffecttyp(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_040207_carea_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040211_carea_setassphere_rage_vector3_const_ref_float_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_040385_ccombatinfomgr_getcombatinfobyhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_040404_rage_fraginstnm_postartmessage_char_const_ptr_art_messageparamsb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040406_rage_fraginstnm_configurecharacter_int_bool_int_int_float_float(ph: *mut Phantom, a0: i32, a1: f32, a2: f32); }
extern "C" { pub fn f_040408_rage_fraginstnm_getcomvel_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040409_art_messageparamsbase_addbool_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040410_art_messageparamsbase_addfloat_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_040414_rage_fragnmassetmanager_getagentcount_unsigned_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_040422_rage_fraginstnm_switchfromnmtorageragdoll_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_040434_rage_fragnmassetmanager_getnumactivenonnmragdolls(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_040483_rage_nmparam_isbool_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040484_rage_nmparam_isint_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040485_rage_nmparam_isfloat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040486_rage_nmparam_isvector3_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040487_rage_nmparam_isstring_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040490_rage_nmbehavior_getparam_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_040491_rage_nmbehavior_getparam_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_040492_rage_nmbehaviorpool_nmbehaviorpool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040493_rage_nmbehaviorpool_dtornmbehaviorpool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040495_rage_nmbehaviorpool_getbehavior_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_040496_rage_nmbehaviorpool_getbehavior_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_040497_rage_nmbehaviorpool_loadfile_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040500_art_messageparamsbase_addint_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040501_art_messageparamsbase_addvector3_char_const_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_040502_art_messageparamsbase_addstring_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040503_art_messageparamsbase_messageparamsbase_art_messageparamsbase_pa(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_040504_art_messageparamsbase_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040505_art_messageparamsbase_messageparamsbase_art_messageparamsbase_co(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_040998_cpednavcapabilityinfo_buildpathstyleflags_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041056_findplayerped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041060_cpeddebugvisualisermenu_getfocusped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041062_cpedintelligence_findmovementtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041063_cpedintelligence_isfriendlywith_cped_const_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041064_cpedintelligence_isthreatenedby_cped_const_ref_bool_bool_bool_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_041075_cpedtargetting_cpedtargetting_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041077_cpedtargetting_getlosstatus_centity_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041081_cpedmotivation_getmotivation_cpedmotivation_pedmotivationstate_c(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_041085_cpedtargetting_findtarget_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041086_csingletargetinfo_arewhereaboutsknown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041087_cplayerinfo_getpreviousvariationinfo_unsigned_int_ref_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_041088_cpedintelligence_getnearbyvehicles(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041113_cpedintelligence_getlowestlevelnmtask_cped_ptr_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041117_cpedintelligence_addtoeventhistory_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041121_cpedintelligence_buildqueriablestate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041197_cpedintelligence_setdefaultrelationshipgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041202_cpedintelligence_setrelationshipgroup_crelationshipgroup_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041205_cpedpopulation_generatescenariopedsinspecificarea_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_041219_cpedgeometryanalyser_canpedtargetpedasync_cped_ref_cped_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: i32, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_041238_cpedintelligence_gettaskwrithe_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041252_ceventscanner_scanforeventsnow_cped_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041261_cplayerinfo_ishardaiming(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041264_cpedintelligence_setcheckshockflag_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041265_cplayerinfo_isaiming_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041269_cpedintelligence_gettaskclimbladder_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041282_cpedpopulation_shouldusestartupmode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041288_cpedintelligence_getactivepropmanagementhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041296_cpedfactory_destroyped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041310_cpedfootstephelper_setclimbingladder_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041330_cpedintelligence_getnearbyobjects(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041331_cpedgeometryanalyser_testpedcapsule_cped_const_ptr_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64, a10: f32, a11: f32, a12: f32, a13: i64, a14: f32) -> i32; }
extern "C" { pub fn f_041332_cpedgeometryanalyser_testifcollisiontypeisvalid_cped_const_ptr_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_041371_cpediksettingsinfo_isikflagdisabled_ikmanagersolvertypes_flags_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041374_rage_fwregdref_cscenariopoint_rage_fwrefawarebasenonvirt_dtorfwr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041377_cpedintelligence_setdecisionmakerid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041380_cpedtargetting_getbesttarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041382_cpedintelligence_registerthreat_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041385_cpedintelligence_findtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041387_cpedintelligence_gettaskmeleeactionresult_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041408_cpedintelligence_ispedthreatening_cped_const_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041409_cpedintelligence_countfriends_float_bool_bool_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_041418_cpedintelligence_createclonetaskfortasktype_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041419_cpedintelligence_createclonetaskfrominfo_ctaskinfo_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041423_cpedintelligence_increasebattleawareness_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_041428_cpedintelligence_getlastbattleeventtime_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041437_cpedintelligence_setstartnewhangoutscenario_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041441_cpedintelligence_getlowestlevelragdolltask_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041444_cpedintelligence_getlastaudiosaidtoped_cped_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041445_cpedintelligence_hasrecentlytalkedtoped_cped_const_ref_float_con(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_041454_cpedmotiondata_setcurrentmoveblendratio_rage_vector2_ref_float_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_041459_cpedplacement_ispositionclearforped_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_041460_cpedplacement_findzcoorforped_float_rage_vector3_ptr_centity_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_041467_cpedpopulation_choosecivilianpedmodelindexargs_choosecivilianped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041489_cpedpopulation_getpopulationcontrolpointinfo_cpopctrl_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041496_findplayervehicle_cplayerinfo_ptr_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041497_cpedpopulation_shouldremoveped_cped_ptr_rage_vector3_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_041504_cpedpopulation_pedpopulationfailuredata_registerspawnfailure_cpe(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_041505_cpedpopulation_ispedgenerationcoordcurrentlyvalid_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32, a4: i32, a5: f32, a6: i32, a7: i64, a8: i64, a9: i64, a10: i64, a11: i32) -> i32; }
extern "C" { pub fn f_041506_cpedpopulation_addped_unsigned_int_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i64, a11: i32, a12: i32, a13: i32, a14: i64, a15: i64, a16: i32, a17: i32, a18: i32) -> i64; }
extern "C" { pub fn f_041509_rage_fwregdref_cscenariopoint_rage_fwrefawarebasenonvirt_reset_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041528_cpedpopulation_onpedspawned_cped_ref_rage_fwflags_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041539_cpedpopulation_choosecivilianpedmodelindex_cpedpopulation_choose(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041541_cpedpopulation_schedulepedinvehicle_cvehicle_ptr_int_bool_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64) -> i32; }
extern "C" { pub fn f_041542_cpedpopulation_checkpedmodelgenderandbulkyitems_rage_fwmodelid_g(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_041543_cpedpopulation_selectpointsrealtype_cscenariopoint_const_ref_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_041544_cpedpopulation_sp_spawnvaliditycheck_general_cscenariopoint_cons(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041545_cpedpopulation_iseffectinuse_cscenariopoint_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041546_cpedpopulation_sp_spawnvaliditycheck_interior_cscenariopoint_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041547_cpedpopulation_sp_spawnvaliditycheck_position_cscenariopoint_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_041548_cpedpopulation_getscenariopointinareaargs_getscenariopointinarea(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_041549_cpedpopulation_getscenariopointinarea_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_041550_cpedpopulation_getscenariopointinarea_cpedpopulation_getscenario(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041628_cpedtype_findpedtype_rage_atnonfinalhashstring_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041655_cplayerinfo_getfallheightforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041679_cplayerinfo_isfiring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041681_cplayerinfo_controlbuttonsprint_cplayerinfo_esprinttype_bool(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_041684_cplayerinfo_processsprintcontrol_ccontrol_const_ref_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> f32; }
extern "C" { pub fn f_041701_canimspeedups_shouldusempanimrates(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041720_ccontrol_getvehicletargetdown_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_041728_cplayerinfo_incrementaiattemptingarrestonplayer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041729_cplayerinfo_decrementaiattemptingarrestonplayer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041734_cplayerpedtargeting_gettarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041746_reportplayerresetmismatch_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_041809_cpopzones_findsmallestforposition_rage_vector3_const_ptr_ezoneca(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_041839_cpedperception_isinseeingrange_centity_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041858_cpedmotivation_increasepedmotivation_cpedmotivation_pedmotivatio(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_041859_cpedmotivation_setmotivation_cpedmotivation_pedmotivationstate_f(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_041892_cpedtargetting_setinuse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041927_cplayerpedtargeting_setlockontarget_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041935_cplayerpedtargeting_clearsoftaim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041984_cserialisedfsmtaskinfo_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041990_ctaskinfo_ctaskinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041993_cclonedtaskinfoweapontargethelper_cclonedtaskinfoweapontargethel(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041995_cclonedtaskinfoweapontargethelper_updatetarget_caitarget_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041998_cqueriableinterface_gettargetfortasktype_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042003_cqueriableinterface_gettaskinfofortasktype_int_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042004_cqueriableinterface_getstatefortasktype_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042055_cwildlifemanager_checkwildlifemultiplayerspawnconditions_unsigne(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_042647_cpedpropsmgr_doespedhaveproporcomponentwithflagset_cped_const_re(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042814_worldprobe_debugfindgroundzfor3dcoord_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> f32; }
extern "C" { pub fn f_042818_worldprobe_cshapetestmanager_debugsubmittest_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_042821_worldprobe_cshapetestmanager_aborttest_worldprobe_cshapetestresu(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042844_cwatertesthelper_getwaterheightatpositionincludingrivers_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_042848_worldprobe_cshapetestdesc_setexcludeentities_centity_const_ptr_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_043046_ccollisionhistory_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043049_ccollisionhistory_getmostsignificantcollisionrecordoftype_unsign(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043050_ccollisionhistory_getmostsignificantcollisionrecordoftypes_unsig(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043075_rage_matrix34_transform_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043077_cbuoyancy_getbuoyancyinfo_cphysical_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_043080_cbuoyancy_getwaterlevelonsample_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_043085_worldprobe_cshapetestdesc_setexcludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043088_cbuoyancyinfo_clone(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043089_cbuoyancyinfo_dtorcbuoyancyinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043136_cphysics_canuseragdolls(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043138_fraginstnmgta_checkcanactivate_cped_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_043139_fraginstnmgta_setimpactdamageentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043140_fraginstnmgta_gettimesinceimpactdamage_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043149_fraginstnmgta_getbodypartgroup_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_043150_fraginstnmgta_setactiveposefromclip_rage_crclip_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_043153_fraginstnmgta_applybulletforce_cped_ptr_float_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_043155_artfeedbackinterfacegta_artfeedbackinterfacegta(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_043156_artfeedbackinterfacegta_onbehaviourfailure(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043161_fraginstnmgta_processtwostageramp_fraginstnmgta_eramptype_fragin(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32); }
extern "C" { pub fn f_043211_cphysics_breakglass_cdynamicentity_ptr_rage_vec3v_const_ref_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: f32, a5: i32) -> i32; }
extern "C" { pub fn f_043214_rage_fwentity_resetnocollision(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043234_worldprobe_cshapetestdesc_setincludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043242_cphysics_getmeasuringtoolpos_int_rage_vector3_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_043293_cragdollconstraintcomponent_updatehooktoworldconstraint_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_043333_cwatertesthelper_getcachedriverboundproberesult_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_043335_worldprobe_debugfindgroundzforcoord_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_043348_worldprobe_cshapetestdesc_addexludeinstance_rage_phinst_const_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043349_worldprobe_cshapetestdesc_setexcludeinstance_rage_phinst_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044123_water_getwaterlevelnowaves_rage_vector3_const_ref_float_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_045502_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_045789_cped_ismale_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045793_cped_checkbraveryflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045795_cped_checkagilityflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045797_cped_getcurrentmotiontask_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045800_cped_delayedremovaltimereset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045802_cped_setcharparamsbasedonmanagertype(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045804_cped_clearactivateragdolloncollisionevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045812_cped_removespeakerlistenedto(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045819_cped_setactivateragdolloncollision_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045834_cped_switchtoanimated_bool_bool_bool_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_045840_cped_getbonematrix_rage_matrix34_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045842_cped_forcemotionstatethisframe_cpedmotionstates_emotionstate_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_045844_cped_setdefaultdecisionmaker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045865_cped_updateragdollmatrix_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045874_cped_isonground_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045876_cped_setuseextractedz_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045882_cped_switchtoragdollinternal(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045883_cped_setpedoutofvehicle_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_045885_cped_switchtoragdoll_cevent_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045886_cped_setmyvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045889_cped_getprimarymotiontask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045902_cped_updatespatialarraytypeflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045903_cped_getiscrouching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045904_cped_setiscrouching_bool_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_045908_cped_isplayingagestureanim_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045909_cped_getprimarymotiontaskifexists_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045912_cped_isusingactionmode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045913_cped_isfirstpersonshootermodeenabledforplayer_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045914_cped_setspeakerlistenedto_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045922_cped_getboneposition_rage_vector3_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045926_cped_getisdrivingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045927_cped_getheldobject_cdynamicentity_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045939_cped_setvarrandom_unsigned_int_unsigned_int_rage_atfixedbitset_3(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_045967_cped_setremoveassoonaspossible_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045968_cped_gettaskdata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045970_cped_getscenariotype_cdynamicentity_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045971_cped_getscenariopoint_cdynamicentity_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045972_cped_getweaponhash_cdynamicentity_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045974_cped_getloadingobjectindex_cdynamicentity_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045976_cped_setisstrafing_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045977_cped_setdesiredheading_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_045978_cped_setdesiredheading_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045981_cped_isinmotion_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045982_cped_stopallmotion_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045985_cped_validategroundphysical_rage_phinst_const_ref_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_045986_rage_phcollider_getlocalvelocity_float_vector_4_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: u128); }
extern "C" { pub fn f_045990_rage_vec_v3rotateaboutzaxis_float_vector_4_float_vector_4(ph: *mut Phantom, a0: u128, a1: u128) -> u128; }
extern "C" { pub fn f_045996_cped_getbonetagfromragdollcomponent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046017_cped_getattacker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046018_cped_addwounddata_cped_wounddata_const_ref_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_046026_cped_getvehiclepedentering_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046052_cped_issecurityped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046054_cped_setusingactionmode_bool_cped_actionmodeenabled_float_bool_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_046056_cped_canbedeleted_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_046057_cped_updateragdollboundsfromanimatedskel_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046083_cped_forceragdollpresetup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046086_cped_resetstanddata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046088_cped_restoreheadingchangerate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046091_cped_debugattachpedtophysical_char_const_ptr_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i64, a8: i64, a9: f32, a10: f32); }
extern "C" { pub fn f_046096_cped_getbonepositioncachedv_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046099_cped_getragdollcomponentmatrix_rage_matrix34_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_046102_cped_getisenteringvehicle_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046103_cped_getspecialability_eplayerspecialabilityslot(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046104_cped_isexitingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046109_cped_setarreststate_earreststate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046111_cped_ishurthealththreshold_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046113_cped_candohurttransition_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046116_cped_estimatepose_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046117_cped_orientboundtospine_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_046118_cped_clearbound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046120_cped_getisfpsswimmingunderwater_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046121_cped_gethasjetpackequipped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046135_cped_isgangped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046144_cped_newsay_char_const_ptr_unsigned_int_bool_bool_int_cped_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32) -> i32; }
extern "C" { pub fn f_046146_cped_randomlysay_char_const_ptr_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_046147_cped_newsay_unsigned_int_unsigned_int_bool_bool_int_cped_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_046148_cped_randomlysay_unsigned_int_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_046151_csimpleblendhelper_getphase_ccliphelper_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_046153_cped_getpedradiocategory_int_ref_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046154_cped_canputhandsup_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046155_cped_willreturntopositionpostflee_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046156_cped_getispanicking_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046168_cped_isprone_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046172_cped_getwaterlevelonped_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_046175_cped_setdesiredvelocityclamped_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_046176_cped_getcontrolfromplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046184_cped_getiksettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046210_cped_isbeingforcedtorun_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046326_cgameworldheightmap_getmaxheightfromworldheightmap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_046362_cdatafilemgr_expandfilename_char_const_ptr_char_ptr_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046408_cinteriorinst_getinteriorforlocation_rage_fwinteriorlocation_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046410_centity_getinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046438_cgameworld_add_centity_ptr_rage_fwinteriorlocation_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046445_cportaltracker_collectstack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046467_cgameworld_remove_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046484_centity_setisvisibleformodule_eisvisiblemodule_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046491_centity_getisonscreen_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046499_centity_getcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046500_centity_getisstatic_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046502_centity_getphysarch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046531_centity_getisvisibleformodule_eisvisiblemodule_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046599_centityiterator_centityiterator_int_centity_const_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_046600_centityiterator_getnext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046702_cwarpmanager_setwarp_rage_vector3_const_ref_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: f32, a6: i32) -> i32; }
extern "C" { pub fn f_046738_cextracontentmanager_getassetpathfromdevice_char_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_047012_cfocusentitymgr_getpos(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047020_cgameworld_findlocalplayerwanted(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047044_cgameworld_forallentitiesintersecting_rage_fwsearchvolume_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_047060_cphysical_activatephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047066_cphysical_getreferenceframevelocity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047077_cphysical_getweapondamagedtimebyhash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047079_cphysical_hasbeendamagedbyentity_centity_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_047095_cphysical_applyforce_rage_vector3_const_ref_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32); }
extern "C" { pub fn f_047096_cphysical_applyforcecg_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_047100_cphysical_applyimpulse_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047106_cphysical_getlocalspeed_rage_vector3_const_ref_bool_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047117_cphysical_debugattachtoworldbasic_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_047119_cphysical_debugattachtophysicalusingphysics_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: f32); }
extern "C" { pub fn f_047126_cphysical_debugprocessallattachments_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_047439_cgameworld_addscenariopointsforentity_centity_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047479_cextensiondefspawnpoint_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_047528_cextensiondefspawnpoint_cextensiondefspawnpoint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047725_cinteriorinst_issubwaymlo_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047729_cinteriorinst_getnumexitportalsinroom_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047856_cportaltracker_probeforinterior_rage_vector3_const_ref_cinterior(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_047880_cgameworld_findlocalplayercoors(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047960_cgameworld_hasentitybeenadded_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047982_cgameworld_findlocalplayervehicle_bool(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047998_cgameworld_findlocalplayerspeed(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_048339_networkutils_isnetworkcloneormigrating_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_050115_cclipeventtags_ctagiteratorattributevalue_rage_crtag_rage_crprop(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_058506_void_creplaymgrinternal_recordpersistantfx_cpacketaddrope_unsign(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_058516_void_creplaymgrinternal_recordpersistantfx_cpacketropewinding_un(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_058526_void_creplaymgrinternal_recordpersistantfx_cpacketunpinrope_unsi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_058533_void_creplaymgrinternal_recordpersistantfx_cpacketattachropetoen(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_058535_void_creplaymgrinternal_recordpersistantfx_cpacketdetachropefrom(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_060782_ccliphelper_isplayingclip_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063793_cscriptsforbrains_startorrequestnewstreamedscriptbrain_int_void(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_063797_cthescripts_getcurrentgtascripthandlernetwork(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063879_cupsidedowncarcheck_iscarupsidedown_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063888_cboatisbeachedcheck_isboatbeached_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063893_csuppressedmodels_hasmodelbeensuppressed_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_063897_cscriptpeds_haspedmodelbeenrestrictedorsuppressed_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_063900_cscriptentities_clearspaceformissionentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063903_rage_fweventqueue_16_void_flusheventsoftype_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064812_cpopulationstreaming_getfallbackpedmodelindex_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064813_cpopulationstreaming_isfallbackpedavailable_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064814_cpopulationstreaming_streamscenariopeds_rage_fwmodelid_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_064832_cstreaming_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064860_cprioritizedclipsetrequestmanager_request_cprioritizedclipsetreq(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_064933_cstreaming_requestobject_rage_strlocalindex_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_064934_cstreaming_removeobject_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_065332_ccontrol_getpedsprintisdown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065334_ccontrol_getpedsprintispressed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065469_ccontrolmgr_startplayerpadshakebyintensity_unsigned_int_float_in(ph: *mut Phantom, a0: i32, a1: f32, a2: i32); }
extern "C" { pub fn f_065609_cfilemgr_getassetsfolder(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_065801_csimpleblendhelper_setrate_ccliphelper_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_065802_ccliphelper_setphase_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_065875_ctasksynchronizedscene_ctasksynchronizedscene_short_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32, a5: f32, a6: i64, a7: i64, a8: f32, a9: i32, a10: i64) -> i64; }
extern "C" { pub fn f_065943_ctaskreacharm_ctaskreacharm_rage_criksolverarms_earms_cdynamicen(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: f32, a8: f32) -> i64; }
extern "C" { pub fn f_065945_ctaskreacharm_setclip_rage_fwmvtemplateid_rage_fwmvclipsetidtype(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_066082_ctaskanimatedhitbyexplosion_ctaskanimatedhitbyexplosion_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_066096_ctaskcover_ctaskcover_cweapontarget_const_ref_int_ctaskcover_efo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_066097_ctaskcombatadditionaltask_ctaskcombatadditionaltask_int_cped_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32) -> i64; }
extern "C" { pub fn f_066157_ctaskthreatresponse_ctaskthreatresponse_cped_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_066199_ctaskmelee_iscurrentlydoingamove_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_066231_ctaskmelee_getmeleegripfilter_rage_fwmvtemplateid_rage_fwmvclips(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066253_worldprobe_cshapetesthitpoint_gethitentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066299_ctaskdyingdead_ctaskdyingdead_cdeathsourceinfo_const_ptr_int_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_066334_ctaskwrithe_getdieclipsetandrandclip_rage_fwmvtemplateid_rage_fw(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066335_rage_vec_v3anglezv_float_vector_4_float_vector_4(ph: *mut Phantom, a0: u128, a1: u128) -> u128; }
extern "C" { pub fn f_066336_ctaskdyingdead_dealwithdeactivatedphysics_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_066467_ctaskcombat_ctaskcombat_cped_const_ptr_float_rage_fwflags_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_066495_ctaskreacttobuddyshot_isvalid_cped_const_ref_caitarget_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066510_ctaskreacttobuddyshot_ctaskreacttobuddyshot_caitarget_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_066526_ctasksearchbase_loaddefaultparamsforpedandtarget_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066527_ctasksearchbase_ctasksearchbase_ctasksearchbase_params_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066604_ctaskthreatresponse_createfortargetduetoevent_cped_const_ptr_cpe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_066614_ctaskthreatresponse_determinethreatresponsestate_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_066615_ctaskthreatresponse_shouldalwaysfleeduetorules_cped_const_ref_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_066621_ctaskthreatresponse_predeterminethreatresponsestatefromevent_cpe(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_066632_ctaskhandsup_ctaskhandsup_int_cped_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_066663_ctaskwrithe_getrandclipfromclipset_rage_fwmvtemplateid_rage_fwmv(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_066665_ctaskwrithe_streamreqresourcesin_cwritheclipsetrequesthelper_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_066677_cclonedwritheinfo_createclonefsmtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066906_ceventshockingbrokenglass_ceventshockingbrokenglass_centity_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_067243_ctaskvehiclepersuit_countnumberbuddiesenteringvehicle_cped_ptr_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067281_ctaskshellshocked_ctaskshellshocked_rage_vec3v_const_ref_ctasksh(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_067286_ctaskshellshocked_choosedirection_cped_const_ref_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_067290_ctaskshootattarget_ctaskshootattarget_caitarget_const_ref_int_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32) -> i64; }
extern "C" { pub fn f_067372_crandomeventmanager_cancreateeventoftype_erandomevent_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_067374_ctaskpursuecriminal_givetasktoped_cped_ref_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_067375_crandomeventmanager_seteventstarted_erandomevent(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_067377_ctaskpursuecriminal_ctaskpursuecriminal_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_067392_ctaskpursuecriminal_checkplayeraudio_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067407_ctaskreacttopursuit_leavevehicle_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067546_cclonedtaskinfoweapontargethelper_serialise_rage_csyncdatabase_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067768_cambientanimationmanager_randomlychoosepropforspawningped_cscena(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_067769_cambientanimationmanager_getpropsetfromhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_067770_cambientanimationmanager_fillsearchinfoforconditionalanimsgroup(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_067771_cambientanimationmanager_chooseconditionalanimations_cscenarioco(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_067772_cambientaudio_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_067813_cambientaudio_getfinalizedhash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067850_cclipeventtags_getmoveeventstartandendphases_rage_crclip_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_067851_cambientevent_cambientevent_ambienteventtype_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_067854_ctaskambientclips_ctaskambientclips_unsigned_int_cconditionalani(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_067861_ctaskambientclips_initprophelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067862_cpropmanagementhelper_givepedprop_cped_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067863_cpropmanagementhelper_requestprop_rage_atnonfinalhashstring_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067864_ctaskambientclips_addevent_cambientevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067866_cpropmanagementhelper_process_cpropmanagementhelper_context_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067869_cpropmanagementhelper_dropprop_cped_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_067871_cpropmanagementhelper_processmovesignals_cpropmanagementhelper_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067875_cpropmanagementhelper_releaseproprefifunshared_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067880_csimpleblendhelper_getrate_ccliphelper_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_067892_ctaskambientclips_getactiveclippriority(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067893_cpropmanagementhelper_updateexistingormissingprop_cped_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067894_ctaskambientclips_overridebaseclip_rage_fwmvtemplateid_rage_fwmv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_067895_ctaskambientclips_setoverrideanimdata_scenarioanimdebuginfo_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067903_ctaskambientlookatevent_ctaskambientlookatevent_cevent_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_067917_ctaskcomplexcontrolmovement_ctaskcomplexcontrolmovement_ctask_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_067918_ctaskcomplexcontrolmovement_getrunningmovementtask_cped_const_pt(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067940_ctaskplayeronfoot_getlaststickinputifvalid_rage_vector2_ref_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_067980_ctaskplayeronfoot_getclimbdetectorparam_int_cped_ptr_spedtestpar(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_067985_ctaskmoveplayer_ctaskmoveplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067995_ctaskcomplexcontrolmovement_getmoveblendratio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_068003_ctaskunalerted_ctaskunalerted_ctask_ptr_cscenariopoint_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_068020_cscenariopoint_isobstructedandreset(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068021_ctaskunalerted_canscenariobereused_cscenariopoint_ref_cscenarioc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_068030_ctaskwander_keepmovingwhilstwaitingforfirstpath_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068031_cpavementfloodfillhelper_cleanuppavementfloodfillrequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068032_cpavementfloodfillhelper_startpavementfloodfillrequest_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068033_cpavementfloodfillhelper_updatepavementfloodfillrequest_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068034_ctaskunalerted_setupdrivingtaskforemergencyvehicle_cped_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_068035_ctaskunalerted_setscenariochainuserinfo_cscenariopointchainusein(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068048_ctaskwander_ctaskwander_float_float_cconditionalanimsgroup_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i32, a5: i32, a6: f32, a7: i32, a8: i32) -> i64; }
extern "C" { pub fn f_068057_ctaskcomplexcontrolmovement_setnewmovetask_ctask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068059_ctaskcomplexcontrolmovement_setnewmainsubtask_ctask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068069_ctaskwanderinarea_ctaskwanderinarea_float_float_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_068126_ctaskdonothing_ctaskdonothing_int_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_068150_ctaskcomplexcontrolmovement_settarget_cped_const_ptr_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_068151_ctaskcomplexcontrolmovement_setmoveblendratio_cped_const_ptr_flo(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_068161_ctasksequencelist_ctasksequencelist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068163_ctasksequencelist_addtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068203_ctaskusesequence_ctaskusesequence_ctasksequencelist_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_068253_ctaskcrouch_ctaskcrouch_int_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068289_ctaskcallpolice_canmakecall_cped_const_ref_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068293_ctaskcallpolice_ctaskcallpolice_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_068297_cambientaudio_cambientaudio_char_const_ptr_rage_fwflags_unsigned(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068332_cpropmanagementhelper_cpropmanagementhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068335_cpropmanagementhelper_hasdestroyorreleasetag_cpropmanagementhelp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_068336_cpropmanagementhelper_handleclipeventdestroyprop_cpropmanagement(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068337_cpropmanagementhelper_handleclipeventreleaseprop_cpropmanagement(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068338_cpropmanagementhelper_setforcedloading(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068339_cpropmanagementhelper_disposeofpropifinappropriate_cped_ref_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_068346_cpavementfloodfillhelper_cpavementfloodfillhelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068454_ctaskmotionbase_setonfootclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_068457_ctaskmotionbase_resetonfootclipset_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_068467_ctaskmotionbase_getdefaultonfootclipset_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_068469_ctaskmotionbase_interpvalue_float_ref_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32) -> f32; }
extern "C" { pub fn f_068470_ctaskmotionbase_interpvalue_float_ref_float_float_bool_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_068505_rage_atarray_ctaskmotionswimming_scubagearvariations_0_unsigned(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_068524_ctaskmotionswimming_checkforrapids_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068525_ctaskmotionswimming_candivefromposition_cped_ref_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_068601_ctaskfishlocomotion_checkandrespectswimnearsurface_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_068615_ctaskhumanlocomotion_ctaskhumanlocomotion_ctaskhumanlocomotion_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i64, a9: f32) -> i64; }
extern "C" { pub fn f_068638_ctaskhumanlocomotion_useleftfoottransition_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068658_ctaskgotoandclimbladder_scanforladdertoclimb_cped_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_068659_ctaskgotoandclimbladder_isgettingonbottom_centity_ptr_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_068660_ctaskgotoandclimbladder_findtopandbottomofallladdersections_cent(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_068662_ctaskgotoandclimbladder_calcladderinfo_centity_ref_int_rage_vect(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_068665_ctaskmotionped_ctaskmotionped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068698_ctaskmotionswimming_ctaskmotionswimming_int_bool_float_float_flo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_068700_ctaskmotiondiving_ctaskmotiondiving_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068702_ctaskmotionaiming_ctaskmotionaiming_bool_bool_bool_bool_ctaskmot(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: f32, a7: i32, a8: f32, a9: f32, a10: f32, a11: f32, a12: i32) -> i64; }
extern "C" { pub fn f_068752_ctaskmotionaiming_isturning_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068804_ctaskfall_ctaskfall_rage_fwflags_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068825_ctaskmotiontennis_ctaskmotiontennis_bool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068875_ctaskmotioninturret_ctaskmotioninturret_cvehicle_ptr_cmovenetwor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_068908_ctaskanimatedfallback_ctaskanimatedfallback_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32) -> i64; }
extern "C" { pub fn f_068914_ctaskcower_ctaskcower_int_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068929_ctaskgetup_ctaskgetup_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068941_ctaskmovegotopoint_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068942_ctaskwalkroundentity_ctaskwalkroundentity_float_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_068951_ctaskwalkroundcarwhilewandering_ctaskwalkroundcarwhilewandering(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_068958_ctasknavbase_ctasknavbase_float_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i64; }
extern "C" { pub fn f_068959_ctaskmovefollownavmesh_isunabletofindroute_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068960_ctaskmovewalkroundvehicle_ctaskmovewalkroundvehicle_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_069002_ctaskjump_ispednearground_cped_const_ptr_float_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_069006_ctaskparachute_canpedparachutefromposition_cped_const_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32) -> i32; }
extern "C" { pub fn f_069013_ctaskfall_updateheading_cped_ptr_ccontrol_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_069016_ctaskfall_getishighfall_cped_ptr_bool_float_ref_float_float_floa(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32, a4: f32, a5: f32) -> i32; }
extern "C" { pub fn f_069018_ctaskfall_getmaxpedfallheight_cped_const_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069032_ctaskflytopoint_ctaskflytopoint_float_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_069064_ctaskturntofaceentityorcoord_ctaskturntofaceentityorcoord_centit(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_069065_ctaskmovegotopointandstandstill_ctaskmovegotopointandstandstill(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: f32) -> i64; }
extern "C" { pub fn f_069067_ctaskmovestandstill_ctaskmovestandstill_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_069069_ctaskmoveachieveheading_ctaskmoveachieveheading_float_float_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_069075_ctaskturntofaceentityorcoord_ctaskturntofaceentityorcoord_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_069090_ctaskgetup_ctaskgetup_rage_atnonfinalhashstring_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069103_ctaskgetup_findgetupcontrollingtask_cped_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069122_cdropdown_cdropdown_cdropdown_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069123_ctaskgetup_selectblendoutsets_cped_ptr_cnmblendoutsetlist_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_069132_cnmblendoutsetlist_relinquishmovetask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069136_ctaskmovefollowpointroute_ctaskmovefollowpointroute_float_cpoint(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32, a10: f32) -> i64; }
extern "C" { pub fn f_069149_ctaskgotopointanymeans_ctaskgotopointanymeans_float_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: f32, a5: i32, a6: i32, a7: i32, a8: f32, a9: f32, a10: f32, a11: i32, a12: f32, a13: f32, a14: f32) -> i64; }
extern "C" { pub fn f_069166_ctaskmovegotopoint_ctaskmovegotopoint_float_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_069178_ctaskgotoscenario_ctaskgotoscenario_ctaskgotoscenario_tasksettin(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069182_ctaskgotoscenario_setadditionaltaskduringapproach_ctask_const_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069185_ctaskmovefollownavmesh_isfollowingnavmeshroute_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069186_ctaskgotoscenario_setnewtarget_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_069224_ctaskmovegotopointrelativetoentityandstandstill_ctaskmovegotopoi(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: f32, a5: i32) -> i64; }
extern "C" { pub fn f_069239_rage_fwmatrixtransform_getroll_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069242_ctaskgotopointaiming_ctaskgotopointaiming_caitarget_const_ref_ca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_069256_ctaskmovefollowentityoffset_ctaskmovefollowentityoffset_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_069271_ctaskmovebeinformation_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069272_ctaskmovebeinformation_ctaskmovebeinformation_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069284_ctaskmovebeinformation_isinposition(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069287_ctaskmovetrackingentity_ctaskmovetrackingentity_float_centity_pt(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_069331_ctaskmovewander_ctaskmovewander_float_float_bool_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32, a4: f32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_069387_ctaskmovewithinattackwindow_ctaskmovewithinattackwindow_cped_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069396_ctaskmovewithindefensivearea_ctaskmovewithindefensivearea_cped_c(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069440_cclimbdetector_process_rage_matrix34_ptr_spedtestparams_ptr_sped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_069441_cclimbhandholddetected_op_cclimbhandholddetected_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069442_cclimbhandholddetected_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069457_ctasknavbase_gettotalroutelength_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069458_ctasknavbase_istotalroutelongerthan_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_069459_ctasknavbase_getwaypointflag_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_069460_ctasknavbase_getthiswaypointline_cped_const_ptr_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_069461_ctasknavbase_getpreviouswaypoint_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069462_ctasknavbase_getcurrentwaypoint_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069463_ctasknavbase_getpositionaheadonroute_cped_const_ptr_float_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_069477_ctaskmovefollownavmesh_ctaskmovefollownavmesh_float_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: i64, a8: f32, a9: i32) -> i64; }
extern "C" { pub fn f_069478_ctaskmovefollownavmesh_ctaskmovefollownavmesh_cnavparams_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069501_ctaskuseclimbonroute_ctaskuseclimbonroute_float_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: i64, a5: i64, a6: i32, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_069507_ctaskusedropdownonroute_ctaskusedropdownonroute_float_rage_vecto(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: i64, a5: i64, a6: i32, a7: i64, a8: i32, a9: i32) -> i64; }
extern "C" { pub fn f_069512_ctaskuseladderonroute_ctaskuseladderonroute_float_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: i64) -> i64; }
extern "C" { pub fn f_069523_rage_atarray_ctaskparachute_parachutepackvariations_0_unsigned_s(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_069543_ctaskparachute_getmodelforparachute_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069606_ctaskparachute_getmodelforparachutepack_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069618_ctasktakeoffpedvariation_createprop_cped_ptr_epedvarcomp_rage_fw(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_069619_ctasktakeoffpedvariation_attachprop_cobject_ptr_cped_ptr_eanimbo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_069620_ctasktakeoffpedvariation_clearvariation_cped_ptr_epedvarcomp_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_069683_ctaskfollowleaderinformation_ctaskfollowleaderinformation_cpedgr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_069691_ctaskfollowleaderanymeans_ctaskfollowleaderanymeans_cpedgroup_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_069697_ctaskseekentityaiming_ctaskseekentityaiming_centity_const_ptr_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> i64; }
extern "C" { pub fn f_069719_ctasktakeoffpedvariation_dtorctasktakeoffpedvariation(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069723_ctasktakeoffpedvariation_detachprop_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_069732_isintersectionclimbable_worldprobe_cshapetesthitpoint_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069733_canfindfurtherintersections_worldprobe_cshapetesthitpoint_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069736_cclimbdetector_getmaxclimbheight_spedtestparams_ptr_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069737_cclimbdetector_testcapsule_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_069738_cclimbdetector_getisvalidintersection_worldprobe_cshapetesthitpo(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069740_cclimbdetector_setclimbphysical_cclimbhandholddetected_ref_world(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_069741_cclimbdetector_getminclimbheight_spedtestparams_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> f32; }
extern "C" { pub fn f_069742_cclimbdetector_getmaxheightdropallowance_const(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_069743_cclimbhandholddetected_cclimbhandholddetected_cclimbhandholddete(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069746_cclimbhandholddetected_calcphysicalhandholdoffset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069747_cclimbdetector_adddebugintersections_worldprobe_cshapetesthitpoi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_069748_cclimbdetector_isintersectionentityanunclimbabledoor_centity_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_069751_cclimbdetector_validatehandholdanddeterminevolume_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_069752_cclimbhandholddetected_getphysicalmatrix_rage_matrix34_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069753_cclimbdetector_doslopetest_rage_matrix34_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32) -> i32; }
extern "C" { pub fn f_069754_ctaskvault_ishandholdsupported_cclimbhandholddetected_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_069756_cclimbhandholddetected_getbonefromhitcomponent_cphysical_const_p(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_069758_cdropdowndetector_adddebugintersections_worldprobe_cshapetesthit(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_069760_ctaskjump_dojumpgaptest_cped_const_ptr_jumpanalysis_ref_bool_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_069761_ctaskdropdown_dtorctaskdropdown(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069765_ctaskdropdown_dumptolog(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069769_ctaskvault_setupragdolloncollision_cped_ptr_cphysical_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_069770_ctaskvault_resetragdolloncollision_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069771_ctaskdropdown_getdropdownspeed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069773_ctaskdropdown_updatehandik_ctaskdropdown_ehandik_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_069786_ctaskclimbladder_ctaskclimbladder_centity_ptr_int_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32) -> i64; }
extern "C" { pub fn f_069787_ctaskclimbladder_dtorctaskclimbladder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069788_ctaskclimbladderfully_sladderblock_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069791_ctaskclimbladder_distancetotop_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_069792_ctaskclimbladder_ismovementblocked_cped_const_ptr_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64) -> i32; }
extern "C" { pub fn f_069793_ctaskclimbladder_ismovementupblockedbypedgettingontopofladdermp(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069794_ctaskclimbladder_iscontrolleddown_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069795_ctaskclimbladder_iscontrolledup_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_069796_ctaskclimbladder_scanfornextladdersection(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069797_ctaskclimbladder_findnext2deffect_centity_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_069798_ctaskclimbladder_getnextladdersection_centity_ptr_centity_ptr_ct(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_069799_ctaskclimbladder_checkforwaterdismount_cped_ptr_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_069804_ctaskclimbladder_settaskstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_069810_ctaskclimbladder_stateinitial_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069811_ctaskclimbladder_stateinitial_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069812_ctaskclimbladder_statemounting_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069813_ctaskclimbladder_statemounting_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069814_ctaskclimbladder_stateclimbingidle_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069815_ctaskclimbladder_stateclimbingup_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069816_ctaskclimbladder_stateclimbingup_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069817_ctaskclimbladder_stateclimbingdown_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069818_ctaskclimbladder_stateclimbingdown_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069819_ctaskclimbladder_stateattopdismount_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069820_ctaskclimbladder_stateattopdismount_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069821_ctaskclimbladder_stateatbottomdismount_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069822_ctaskclimbladder_stateatbottomdismount_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069823_ctaskclimbladder_statesliding_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069824_ctaskclimbladder_statesliding_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069825_ctaskclimbladder_stateslidedismount_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069826_ctaskclimbladder_stateslidedismount_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069827_ctaskclimbladder_stateblocked_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069828_ctaskclimbladder_stateblocked_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069829_ctaskclimbladder_getbaseclimbrate_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069830_ctaskclimbladder_isinterruptcondition(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069831_ctaskclimbladder_calculateperfectstart_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_069832_ctaskclimbladder_shoulddismountinsteadofslide(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069833_ctaskclimbladder_getactivephase(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069837_ctaskclimbladder_calculatetotalanimrotation_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32) -> f32; }
extern "C" { pub fn f_069838_ctaskclimbladder_calculatetotalanimtranslation_float_float_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_069839_ctaskclimbladder_drivetowardsperfect_rage_matrix34_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: f32); }
extern "C" { pub fn f_069844_ctaskgotoandclimbladder_ctaskgotoandclimbladder_centity_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_069845_ctaskgotoandclimbladder_dtorctaskgotoandclimbladder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069849_ctaskgotoandclimbladder_ispedorientatedcorrectlytoladdertogeton(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_069850_ctaskgotoandclimbladder_calculatepositionsforthisladder_cped_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069853_ctaskgotoandclimbladder_shouldabortifstuck(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069859_ctaskclimbladderfully_dtorctaskclimbladderfully(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069863_ctaskclimbladderfully_isladderphysicallyblockedatbasemp_cped_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_069867_ctaskhelipassengerrappel_ctaskhelipassengerrappel_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i64; }
extern "C" { pub fn f_069874_ctaskhelipassengerrappel_getpedtogrounddiff_float_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_069875_ctaskhelipassengerrappel_isusingaccurateropemode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069879_void_creplaymgrinternal_recordpersistantfx_cpacketrappelpinrope(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069892_ctaskrappel_start_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069893_ctaskrappel_climboverwall_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069894_ctaskrappel_idle_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069895_ctaskrappel_idle_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069896_ctaskrappel_descend_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069897_ctaskrappel_descend_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069898_ctaskrappel_jump_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069899_ctaskrappel_jump_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069900_ctaskrappel_smashwindow_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069901_ctaskrappel_smashwindow_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069902_ctaskrappel_updaterope(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069903_ctaskrappel_start_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069904_ctaskrappel_findandbreakglass_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_069908_ctaskrappel_idle_onupdateclone(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069915_ctaskvault_ctaskvault_cclimbhandholddetected_const_ref_rage_fwfl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_069916_ctaskvault_dtorctaskvault(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069919_ctaskvault_getactiveclip_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_069920_ctaskvault_getactivephase_int_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069926_ctaskvault_processfpmovesignals(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069928_ctaskvault_calctotalanimateddisplacement(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069929_ctaskvault_dumptolog(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069930_ctaskvault_processdisablevaulting(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069931_ctaskvault_setdesiredvelocityadjustmentrange_rage_crclip_const_p(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_069932_ctaskvault_computeattachmentbreakoff_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_069933_ctaskvault_haspedwarped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069934_ctaskvault_computeisonground_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069935_ctaskvault_getvaultspeed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069936_ctaskvault_getdroppointforslide_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069938_ctaskvault_updatehandik_ctaskvault_ehandik_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_069940_ctaskvault_ispushingstickindirection_cped_const_ptr_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_069941_ctaskvault_canbreakouttomovementtask_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069959_ctaskfallover_ctaskfallover_rage_fwmvtemplateid_rage_fwmvclipset(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_069960_ctaskfallover_computefallcontext_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_069961_ctaskfallover_pickfallanimation_cped_ptr_ctaskfallover_efallcont(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_069967_ctaskfallover_statefallonenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069968_ctaskfallover_statefallonupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069969_ctaskfallover_statelieongroundonenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069982_ctaskfallandgetup_getclipphase(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069997_ctaskjumpvault_willvault_cped_ptr_rage_fwflags_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_070002_ctaskjump_getjumplandthreshold_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_070003_ctaskjump_cansnaptoground_cped_ptr_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_070004_ctaskjump_getentitysetzthreshold_const(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_070007_ctaskjump_resetragdolloncollision(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070008_ctaskjump_stateland_onexit_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070009_jumpanalysis_getlandingposition_bool_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070010_ctaskjump_activateragdolloncollision_cphysical_ptr_float_float_b(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_070011_ctaskjump_getjumptestendposition_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070012_ctaskjump_getassistedjumplandphase_rage_crclip_const_ptr_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_070013_ctaskjump_checkforblockedjump_cped_const_ptr_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_070018_jumpanalysis_doprobe_cped_const_ptr_rage_vector3_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_070028_cclonedtaskdropdowninfo_dtorcclonedtaskdropdowninfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070060_ctaskmovefacetarget_dtorctaskmovefacetarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070222_cclonedgotopointaiminginfo_dtorcclonedgotopointaiminginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070293_ctaskjump_dtorctaskjump(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070327_cclipposehelper_dtorcclipposehelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070329_cnmtuningset_post_cped_ref_ctasknmbehaviour_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_070330_cclipposehelper_disable_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070349_cnmblendoutset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_070358_cnmblendoutset_findposeitem_rage_fwmvtemplateid_rage_fwmvclipset(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_070362_cnmblendoutsetlist_dtorcnmblendoutsetlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070363_cnmblendoutsetlist_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070364_cnmblendoutsetlist_add_rage_atnonfinalhashstring_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_070366_cnmblendoutsetmanager_isblendoutsetstreamedin_rage_atnonfinalhas(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070367_cnmblendoutsetlist_findposeitem_rage_fwmvtemplateid_rage_fwmvcli(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_070368_cnmblendoutsetmanager_requestblendoutset_rage_atnonfinalhashstri(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070369_cnmblendoutsetmanager_addrefblendoutset_rage_atnonfinalhashstrin(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070370_cnmblendoutsetmanager_releaseblendoutset_rage_atnonfinalhashstri(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070371_cnmblendoutsetmanager_finditem_rage_atnonfinalhashstring_unsigne(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_070373_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_070379_cgrabhelper_cgrabhelper_cgrabhelper_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070381_cgrabhelper_behaviourfailure_cped_ptr_artfeedbackinterfacegta_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_070382_cgrabhelper_startbehaviour_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070384_ctaskanimatedattach_ctaskanimatedattach_cphysical_ptr_int_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32) -> i64; }
extern "C" { pub fn f_070385_ctaskanimatedattach_ctaskanimatedattach_cphysical_ptr_int_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_070386_ctaskanimatedattach_dtorctaskanimatedattach(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070395_ctaskanimatedattach_debugvisualise_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070396_ctasknmbehaviour_wantstoragdoll_cped_ptr_eragdolltriggertypes_ce(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> f32; }
extern "C" { pub fn f_070397_ctasknmbehaviour_calcragdollmultiplierscore_cped_ptr_eragdolltri(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_070398_ctasknmbehaviour_canuseragdoll_cped_ptr_eragdolltriggertypes_flo(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_070399_ctasknmcontrol_ctasknmcontrol_unsigned_int_unsigned_int_rage_ait(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: f32) -> i64; }
extern "C" { pub fn f_070400_rage_atarray_ctaskanimatedattach_sattachednmcomponents_0_unsigne(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070401_ctasknmbehaviour_canuseragdoll_cped_ptr_eragdolltriggertypes_cen(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_070403_ctaskblendfromnm_ctaskblendfromnm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070404_ctaskblendfromnm_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070405_ctaskblendfromnm_ctaskblendfromnm_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070406_ctaskblendfromnm_dtorctaskblendfromnm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070410_ctasknmbehaviour_doesresponsehandleragdoll_cped_const_ptr_rage_a(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_070418_ctasknmbehaviour_tunables_dtortunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070421_cnmmessagelist_getmessage_rage_atfinalhashstring_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_070428_cnmtuningset_addwidgetsinternal_rage_bkgroup_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070429_rage_atarray_rage_atbinarymap_cnmtuningset_rage_atnonfinalhashst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070430_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_070436_cnmmessage_addwidgets_rage_bkgroup_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070437_rage_atarray_rage_atbinarymap_cnmtuningset_rage_atnonfinalhashst(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_070438_rage_atarray_rage_atbinarymap_cnmtuningset_rage_atnonfinalhashst(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_070449_cnmparametervector_cnmparametervector_rage_nmparam_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070451_cnmparameterbool_addwidgets_rage_bkgroup_ref_rage_nmbehavior_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_070460_cnmmessage_addparameter_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_070461_cnmmessage_removeparameter_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070462_cnmmessage_regenerateparamwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070464_cnmtuningset_removemessage_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070469_ctasknmbehaviour_sendstartmessages_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070470_ctasknmbehaviour_addtuningset_cnmtuningset_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070471_ctasknmbehaviour_applytuningsetstolist_cnmmessagelist_ref_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070474_ctasknmbehaviour_behaviourfailure_cped_ptr_artfeedbackinterfaceg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_070475_ctasknmbehaviour_behavioursuccess_cped_ptr_artfeedbackinterfaceg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_070476_ctasknmbehaviour_behaviourstart_cped_ptr_artfeedbackinterfacegta(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_070477_ctasknmbehaviour_behaviourfinish_cped_ptr_artfeedbackinterfacegt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_070478_ctasknmbehaviour_behaviourevent_cped_ptr_artfeedbackinterfacegta(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_070479_ctasknmcontrol_setcurrenthandpose_ctasknmcontrol_enmhandpose_cmo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32); }
extern "C" { pub fn f_070480_ctasknmbehaviour_shouldabort_rage_aitask_abortpriority_rage_aiev(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_070482_ctasknmbehaviour_processprefsm(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070486_ctasknmbehaviour_createanimatedfallbacktask_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_070492_ctasknmbehaviour_processphysics_float_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_070493_ctasknmbehaviour_processbalancebehaviourevent_cped_ptr_artfeedba(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_070494_ctasknmbehaviour_onmovingground_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_070495_ctasknmbehaviour_checkblendoutthreshold_cped_ptr_ctasknmbehaviou(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_070496_ctasknmbehaviour_processfinishconditionsbase_cped_ptr_ctasknmbeh(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_070497_ctasknmbehaviour_calculateactivationforce_cped_const_ptr_eragdol(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32) -> f32; }
extern "C" { pub fn f_070499_ctasknmbehaviour_shouldusearmedbumpresistance_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070500_ctasknmbehaviour_calcragdolleventscore_cped_ptr_eragdolltriggert(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> f32; }
extern "C" { pub fn f_070501_ctasknmbehaviour_ragdollpoolhasspaceforped_ctasknmbehaviour_erag(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: f32) -> i32; }
extern "C" { pub fn f_070503_ctasknmcontrol_dtorctasknmcontrol(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070510_ctasknmcontrol_cleanupunhandledragdoll_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070518_ctasknmcontrol_controllingtask_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070519_ctasknmcontrol_switchclonepedtoragdoll_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070520_ctasknmcontrol_createnewnmtaskclone_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070521_ctasknmcontrol_createnewnmtask_int_cped_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_070526_ctasknmcontrol_processweaponhiding_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070529_ctasknmhighfall_ctasknmhighfall_unsigned_int_cgrabhelper_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_070530_ctasknmbalance_ctasknmbalance_unsigned_int_unsigned_int_centity(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64, a6: f32, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_070531_ctasknmbalance_evaluateandsettype_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070532_ctasknmexplosion_ctasknmexplosion_unsigned_int_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_070534_ctasknmbehaviour_addtoragdollpool_ctasknmbehaviour_eragdollpool(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_070535_cpedintelligence_addlocalclonetask_ctaskfsmclone_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070538_ctasknmbalance_tunables_dtortunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070541_ctasknmbalance_dtorctasknmbalance(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070553_ctasknmbalance_probeforsignificantdrop_cped_ptr_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_070557_ctasknmbrace_tunables_dtortunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070558_ctasknmbrace_tunables_stuckonvehicle_dtorstuckonvehicle(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070561_ctasknmbrace_ctasknmbrace_unsigned_int_unsigned_int_centity_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_070562_ctasknmbrace_dtorctasknmbrace(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070572_cthreatenedhelper_start_cthreatenedhelper_euseofgun_cthreatenedh(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_070574_ctasknmbrace_tunables_applyforce_getforcevec_rage_vec3v_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_070575_cthreatenedhelper_controlbehaviour_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070576_cthreatenedhelper_update_cped_ptr_rage_vector3_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_070600_ctasknmdraggingtosafety_ctasknmdraggingtosafety_cped_ptr_cped_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_070601_ctasknmdraggingtosafety_dtorctasknmdraggingtosafety(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070607_ctasknmdraggingtosafety_createconstraint_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_070612_ctasknmelectrocute_tunables_dtortunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070621_ctasknmelectrocute_createanimatedfallbacktask_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_070636_ctasknmfalldown_ctasknmfalldown_unsigned_int_unsigned_int_ctaskn(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_070637_ctasknmfalldown_dtorctasknmfalldown(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070644_ctasknmflinch_tunables_dtortunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070647_ctasknmflinch_ctasknmflinch_unsigned_int_unsigned_int_rage_vecto(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i64, a10: i64, a11: i64, a12: i64) -> i64; }
extern "C" { pub fn f_070648_ctasknmflinch_calculatehitpointinfo_cped_const_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_070649_ctasknmflinch_dtorctasknmflinch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070651_ctasknmflinch_setflinchmessage_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070659_ctasknmhighfall_tunables_dtortunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070663_ctasknmhighfall_dtorctasknmhighfall(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070675_ctasknmsimple_ctasknmsimple_ctasknmsimple_tunables_tuning_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070694_ctasknmjumprollfromroadvehicle_ctasknmjumprollfromroadvehicle_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_070695_ctasknmjumprollfromroadvehicle_dtorctasknmjumprollfromroadvehicl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070712_ctasknmonfire_createanimatedfallbacktask_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070715_ctasknmpose_ctasknmpose_unsigned_int_unsigned_int_rage_fwmvtempl(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: f32, a6: f32, a7: i32, a8: i32) -> i64; }
extern "C" { pub fn f_070718_ctasknmpose_postnmmessagesforstiffness_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070722_ctasknmprototype_tunables_dtortunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070732_ctasknmprototype_senddynamicset_cnmtuningset_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070739_ctasknmrelax_ctasknmrelax_unsigned_int_unsigned_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_070763_ctasknmshot_tunables_paramsets_dtorparamsets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070766_ctasknmshot_tunables_paramsets_paramsets(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070767_ctasknmshot_tunables_hitpointrandomisation_getrandomhitpoint_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_070769_ctasknmshot_ctasknmshot_cped_ptr_unsigned_int_unsigned_int_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32, a7: i64, a8: i64, a9: i64, a10: i32) -> i64; }
extern "C" { pub fn f_070770_ctasknmshot_checkinputnormalandhitposition(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070771_ctasknmshot_dtorctasknmshot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070778_ctasknmshot_updateshot_cped_ptr_centity_ptr_unsigned_int_int_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_070779_ctasknmshot_setshotmessage_cped_ptr_bool_cnmmessagelist_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_070780_ctasknmshot_gathersituationaldata_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070781_ctasknmshot_checkforobjectinpath_cped_ptr_rage_vector3_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_070782_ctasknmshot_handleapplybulletimpulse_cped_ptr_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_070783_ctasknmshot_calculatehealth_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070784_ctasknmshot_getarmouredratio_cped_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_070785_ctasknmshot_handlenewbullet_cped_ptr_rage_vector3_ref_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_070787_ctasknmshot_handlepointgun_cped_ptr_bool_cnmmessagelist_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_070788_ctasknmshot_handlesnap_cped_ptr_cnmmessagelist_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_070792_void_std_2_introsort_std_2_classicalgpolicy_netplayerdistsortpre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_070794_ctasknmshot_createanimatedfallbacktask_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_070798_rage_atarray_rage_atbinarymap_ctasknmsimple_tunables_tuning_rage(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_070806_ctasknmsimple_tunables_refreshtuningwidgets_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_070808_rage_atarray_rage_atbinarymap_ctasknmsimple_tunables_tuning_rage(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_070809_ctasknmsimple_tunables_tuning_op_ctasknmsimple_tunables_tuning_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070810_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_070818_ctasknmsit_ctasknmsit_unsigned_int_unsigned_int_rage_fwmvtemplat(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_070819_ctasknmsit_dtorctasknmsit(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070822_ctasknmsit_updatelookat_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_070825_ctasknmslungovershoulder_dtorctasknmslungovershoulder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070833_ctasknmthroughwindscreen_tunables_dtortunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070836_ctasknmthroughwindscreen_ctasknmthroughwindscreen_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_070837_ctasknmthroughwindscreen_dtorctasknmthroughwindscreen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070846_ctaskrageragdoll_ctaskrageragdoll_bool_centity_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_070859_ctaskagitated_ctaskagitated_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070860_ctaskagitated_dtorctaskagitated(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070863_ctaskagitated_changeresponse_rage_atnonfinalhashstring_rage_atno(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_070864_cagitatedresponse_dtorcagitatedresponse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070865_ctaskagitated_changestate_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070866_cagitatedstate_dtorcagitatedstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070867_ctaskagitated_findfirstvalidtalkresponse_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070868_cagitatedreaction_dtorcagitatedreaction(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070875_ctaskagitated_gettypeforevent_cped_const_ref_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_070876_ctaskagitated_getcontext_cped_const_ref_rage_atnonfinalhashstrin(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070878_cpedintelligence_talkinfo_talkinfo_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070881_ctaskagitatedaction_ctaskagitatedaction_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070882_ctaskagitatedaction_dtorctaskagitatedaction(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070890_ctaskconfront_ctaskconfront_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070891_ctaskconfront_dtorctaskconfront(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070897_ctaskintimidate_isvalid_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070898_ctaskintimidate_ctaskintimidate_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070899_ctaskconfront_createmovetask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070900_ctaskconfront_calculatetargetposition_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070901_ctaskshove_ctaskshove_cped_ptr_rage_fwflags_unsigned_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_070902_ctaskintimidate_getclipsetforidle_cped_const_ref_ctaskintimidate(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_070903_ctaskintimidate_getclipsetfortransition_cped_const_ref_ctaskinti(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_070910_cpedswaitingtobeoutofsightmanager_cpedinfo_dtorcpedinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070913_ctasksmartflee_ctasksmartflee_caitarget_const_ref_float_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_070914_ctasksmartflee_dtorctasksmartflee(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070917_ctasksmartflee_shouldfleeourownvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070918_ctasksmartflee_canaccelerateinvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070919_ctasksmartflee_canexitvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070920_ctasksmartflee_isvehiclestuck_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070921_ctasksmartflee_canreverseinvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070922_ctasksmartflee_shouldexitvehicleduetorouteandtarget_cvehicle_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_070923_ctasksmartflee_gettargetexitvehiclerouteinterceptionparams_centi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070924_ctasksmartflee_isdrivingparalleltotarget_cvehicle_const_ref_cent(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_070925_ctasksmartflee_shouldwaitbeforeexitvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070926_ctasksmartflee_createsubtaskforambientclips_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070927_ctasksmartflee_doesvehiclehavenodriver_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070928_ctasksmartflee_adjusttargetposbynearbypeds_cped_const_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i64, a13: i64); }
extern "C" { pub fn f_070930_ctasksmartflee_spheresofinfluencebuilderfortarget_rage_tinfluenc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_070932_ctasksmartflee_setupfleeinggetup_cnmblendoutsetlist_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_070935_ctasksmartflee_shouldcower_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070938_ctasksmartflee_setmoveblendratio_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_070939_ctasksmartflee_checkfornewpriorityevent(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070940_ctasksmartflee_trytoscream(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070945_ctasksmartflee_handsup_onenter_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070946_ctasksmartflee_start_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070947_ctasksmartflee_trytoplaytrappedspeech(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070948_ctasksmartflee_addinfluencesphere_cped_const_ref_rage_tinfluence(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_070949_ctasksmartflee_shouldlaunchemergencystopduetorouteandtarget_cveh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_070950_ctasksmartflee_getbasetimetocowers(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_070959_cclonedcontroltasksmartfleeinfo_dtorcclonedcontroltasksmartfleei(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070964_ctaskescapeblast_ctaskescapeblast_centity_ptr_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: f32) -> i64; }
extern "C" { pub fn f_070973_ctaskscenarioflee_ctaskscenarioflee_centity_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_070974_ctaskscenarioflee_dtorctaskscenarioflee(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070977_ctaskscenarioflee_cleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070979_ctaskscenarioflee_findnewfleescenario(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070986_ctaskflyaway_ctaskflyaway_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070987_ctaskflyaway_dtorctaskflyaway(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070998_ctaskexhaustedflee_ctaskexhaustedflee_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070999_ctaskexhaustedflee_dtorctaskexhaustedflee(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071005_ctaskwalkaway_ctaskwalkaway_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_071006_ctaskwalkaway_dtorctaskwalkaway(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071010_ctaskgangpatrol_ctaskgangpatrol_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_071011_ctaskgangpatrol_dtorctaskgangpatrol(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071018_ctaskgrowlandflee_ctaskgrowlandflee_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_071019_ctaskgrowlandflee_dtorctaskgrowlandflee(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071025_ctaskintimidate_dtorctaskintimidate(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071028_ctaskintimidate_cleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071038_cclonedreactandfleeinfo_getfleeaitarget_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_071039_ctaskreactandflee_ctaskreactandflee_caitarget_const_ref_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_071041_ctaskreactandflee_ctaskreactandflee_caitarget_const_ref_ctaskrea(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_071042_ctaskreactandflee_dtorctaskreactandflee(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071044_ctaskreactandflee_setmoveblendratio_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_071059_ctaskreactandflee_start_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071060_ctaskreactandflee_streamreact_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071061_ctaskreactandflee_react_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071062_ctaskreactandflee_react_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071063_ctaskreactandflee_react_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071064_ctaskreactandflee_streamreacttoflee_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071065_ctaskreactandflee_reacttoflee_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071066_ctaskreactandflee_reacttoflee_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071067_ctaskreactandflee_flee_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071068_ctaskreactandflee_streamreactcomplete_clone_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071069_ctaskreactandflee_tasksetstate_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_071070_ctaskreactandflee_setclipsetforstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071071_ctaskreactandflee_scream(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071073_ctaskreactandflee_pickappropriatereactionclipsetfromhash_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_071075_ctaskshockingeventhurryaway_ctaskshockingeventhurryaway_ceventsh(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_071078_ctaskreactindirection_dtorctaskreactindirection(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071088_ctaskreacttoexplosion_ctaskreacttoexplosion_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_071089_ctaskreacttoexplosion_dtorctaskreacttoexplosion(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071092_ctaskreacttoexplosion_choosestateforthreatresponse_cped_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_071093_ctaskreacttoexplosion_choosestate_cped_const_ref_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_071102_ctaskreacttoimminentexplosion_dtorctaskreacttoimminentexplosion(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071109_ctaskreacttodeadped_ctaskreacttodeadped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_071110_ctaskreacttodeadped_dtorctaskreacttodeadped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071120_ctaskshockingevent_lookatevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071121_ctaskshockingevent_sayshockingspeech_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_071122_ctaskshockingevent_adjustangularvelocityforintroclip_float_rage(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: i64); }
extern "C" { pub fn f_071128_ctaskshockingeventreact_ctaskshockingeventreact_ceventshocking_c(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_071129_ctaskshockingeventgoto_updatecrowdroundposition(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071130_ctaskshockingeventgoto_setmovetaskwithambientclips_ctaskmove_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_071131_ctaskshockingeventgoto_stategotocommonupdate_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_071140_ctaskshockingeventhurryaway_shouldfleefromshockingevent(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_071141_ctaskshockingeventbackaway_ctaskshockingeventbackaway_ceventshoc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_071154_ctaskshockingeventreact_gettarget_caitarget_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_071177_ctaskshockingevent_dtorctaskshockingevent_114677(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071182_ctaskshockingpoliceinvestigate_assignvehicletask(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_071191_ctaskshockingeventthreatresponse_dtorctaskshockingeventthreatres(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071198_ctaskshove_dtorctaskshove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071201_ctaskshove_hasmadecontact_ragdollcomponent_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_071202_ctaskshoved_ctaskshoved_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_071209_ctaskshoved_dtorctaskshoved(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071367_cagitatedactiontask_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_071425_cagitatedsay_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_071444_cagitatedconditionalaction_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_071463_cagitatedconditionmulti_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_071731_cagitatedcontext_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_071734_cagitatedsituation_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_071735_cagitatedpersonality_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_071743_rage_atarray_rage_atbinarymap_cagitatedpersonality_rage_atnonfin(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_071744_cagitatedreaction_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_071749_cagitatedconditionalsay_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_071750_cagitatedresponse_onfootclipset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_071755_cagitatedresponse_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_071760_cagitatedtalkresponse_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_071761_cagitatedstate_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_071773_cagitatedconditionaltalkresponse_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_071784_cagitatedtriggerreaction_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_071788_cagitatedtrigger_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_071794_cagitatedtriggerset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_071797_rage_atarray_cagitatedtrigger_0_unsigned_short_kill(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071801_cagitatedmanager_dtorcagitatedmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071836_ctaskescapeblast_dtorctaskescapeblast(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072438_std_2_enable_if_is_move_constructible_rage_atbinarymap_cnmtuning(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072439_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_072440_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_072442_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_netplayerd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_072443_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_netplayerd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_072444_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_netplayerd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_072445_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_072446_std_2_enable_if_is_move_constructible_rage_atbinarymap_ctasknmsi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072447_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_072448_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_072449_ctasknmsimple_tunables_tuning_tuning_ctasknmsimple_tunables_tuni(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_072456_cscenarioanimdebughelper_addwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072475_cscenarioanimdebughelper_updatefcongroupnames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072476_cscenarioanimdebughelper_onselfcongroupchange(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072477_cscenarioanimdebughelper_updateufclipnames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072478_cscenarioanimdebughelper_updatecconclipsnames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072479_cscenarioanimdebughelper_onselcconclipschange(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072480_cscenarioanimdebughelper_updatequeueui(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072481_cscenariopoint_getposition_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072482_cscenarioinfomanager_getscenarioindex_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_072483_cscenarioanimdebughelper_onselftypechange(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072484_cscenarioanimdebughelper_updatefclipnames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072485_cscenarioanimdebughelper_updatecclipsetnames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072486_cscenarioanimdebughelper_updatecclipnames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072487_cscenarioanimdebughelper_queueclip_rage_fwmvtemplateid_rage_fwmv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_072488_rage_atarray_cscenarioanimdebughelper_scenarioanimqueueitem_0_un(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072489_cscenarioanimdebughelper_enqueuefromfile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072490_cscenariopoint_getscenariotypevirtualorreal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072493_cscenariochaininggraph_dtorcscenariochaininggraph(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072494_rage_atarray_cscenariochainingnode_0_unsigned_short_grow_unsigne(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072495_cscenariochaininggraph_updatenodetochainindexmapping(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072496_rage_atarray_cscenariochain_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072497_cscenariochaininggraph_deletechain_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_072498_cscenariochaininggraph_deleteedge_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_072499_cscenariochaininggraph_deleteedgeinternal_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_072500_cscenariochaininggraph_deletenodeinternal_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_072501_cscenariochain_op_cscenariochain_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072502_cscenariochaininggraph_registerpointchainuser_cscenariopoint_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_072503_cscenariopointmanager_updatecachedregionptrs_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072504_cscenariochaininggraph_addchainuser_cscenariopointchainuseinfo_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_072510_cscenariochaininggraph_removechainuser_cscenariopointchainuseinf(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072512_cscenariochaininggraph_removechainuserifunused_cscenariopointcha(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072513_cscenariochaininggraph_updatenodetoscenariopointmapping_cscenari(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_072514_cscenariopointregion_findpointsinarea_rage_spdsphere_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_072518_cparkingspacefreefromobstructionstesthelper_isparkingspacefreefr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_072519_cscenariochaintests_dtorcscenariochaintests(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072520_cscenariopointcontainer_cleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072521_cscenariopointcontainer_dtorcscenariopointcontainer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072522_cscenariopointcontainer_postload_cscenariopointloadlookups_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072523_cscenarioclustering_getnewwrapper(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_072524_cscenariopoint_initarchetypeextensionfromdefinition_rage_fwexten(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072525_cscenariopoint_destroyforeditor_cscenariopoint_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072527_cscenariopointcontainer_prepsaveobject_cscenariopointcontainer_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_072528_cscenariopoint_dtorcscenariopoint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072529_cscenariopointcontainer_removepoint_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_072530_cscenariomanager_onremovescenario_cscenariopoint_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072531_cscenariopoint_op_cscenariopoint_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_072532_cscenariomanager_onaddscenario_cscenariopoint_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072533_cscenarioclusterspawnedtrackingdata_resetrefcountedmodel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072535_cspclusterfsmwrapper_despawnandcleartrackeddata_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_072536_cspclusterfsmwrapper_isvehiclereadytoberemoved_cvehicle_ref_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072537_cscenarioclusterspawnedtrackingdata_dtorcscenarioclusterspawnedt(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072539_cscenariomanager_getscenarioinfo_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_072541_cspclusterfsmwrapper_requeststreamingdependencies_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_072542_cspclusterfsmwrapper_isstillvalidtospawn_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072543_cspclusterfsmwrapper_couldspawnat_cscenariopoint_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072544_cscenariospawnhistory_update(ph: *mut Phantom); }
extern "C" { pub fn f_072545_cspclusterfsmwrapper_spawnpeds_onupdatepass2(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072546_cspclusterfsmwrapper_spawnpeds_onupdatepass3(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072547_cscenariomanager_isvehiclescenariotype_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_072548_cscenariospawnhistory_areinsamechain_cscenariopoint_const_ptr_cs(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_072549_cscenariospawnhistory_removevehicleentry_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_072550_cscenariospawnhistory_deletespawnpedindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_072551_cscenariomanager_allowextrachaineduser_cscenariopoint_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072552_cspclusterfsmwrapper_streamandschedulecargenpoint_cscenarioclust(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_072553_cscenariomanager_selectpedtypeforscenario_cscenariomanager_pedty(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_072554_cspclusterfsmwrapper_checkvisibilityforvehicle_cscenariopoint_co(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072555_cscenariomanager_spawnpedsnearvehicleusinglayout_cvehicle_ref_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_072556_cscenariomanager_populatecargenvehicle_cvehicle_ref_ccargenerato(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_072557_cscenariospawnhistory_add_cvehicle_ref_cscenariopoint_ref_int_cs(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_072558_cscenariomanager_setupscenarioandtask_int_rage_vector3_ref_float(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_072559_cscenariomanager_findpropsinenvironmentforscenario_cobject_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_072560_cscenariomanager_createscenarioped_int_cscenariopoint_ref_ctask(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i64, a11: i64, a12: i32, a13: i64) -> i64; }
extern "C" { pub fn f_072561_ctaskusescenario_ctaskusescenario_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_072562_cscenariopoint_getcargen_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072563_cscenariopoint_ishighpriority_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072564_cscenariomanager_checkscenariotimesandprobability_cscenariopoint(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_072565_cscenariomanager_checkvehiclescenariopopulationcount_int_rage_ve(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_072566_cscenariomanager_findvehiclemodelinmodelset_unsigned_int_cambien(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_072567_cscenariomanager_getpedtypeforscenario_int_cscenariopoint_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_072568_cscenariomanager_setupscenariocontroltask_cped_const_ref_cscenar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_072569_cscenariomanager_applyvehiclevariations_cvehicle_ref_cambientveh(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072570_cscenariomanager_vehiclesetupbrokendown_cvehicle_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072571_cscenariomanager_getvehiclescenariodistscalefactorformodel_rage(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_072572_cscenariospawnhistory_findislimitedusescenario_cscenariopoint_co(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072573_cscenariomanager_getrandomdrivermodelindex_rage_fwmodelid_cambie(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_072574_cscenariomanager_gatherblockedmodelsetsfromchainpt_cscenariopoin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_072575_ctaskusescenario_ctaskusescenario_int_cscenariopoint_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_072576_ctaskusescenario_ctaskusescenario_int_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_072577_ctaskscenario_ctaskscenario_int_cscenariopoint_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_072578_cscenariomanager_setupscenariopointheading_cscenarioinfo_const_p(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_072580_cscenariomanager_performcollisionchecksforscenariopoint_cscenari(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072581_cscenariospawnhistory_add_cped_ref_cscenariopoint_ref_int_cscena(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_072582_cscenarioclustering_addwidgets_cscenariopointcluster_ref_rage_bk(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_072589_cscenarioeditor_dtorcscenarioeditor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072605_cscenariopointmanager_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072606_cscenariopointmanager_append_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_072607_cscenarioeditor_postloaddata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072608_cscenarioeditor_rebuildscenariogroupnamelist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072614_cscenariodebug_destroyscenarioprop(ph: *mut Phantom); }
extern "C" { pub fn f_072620_cscenariodebug_spawnprop(ph: *mut Phantom); }
extern "C" { pub fn f_072623_cscenariopointmanager_updatescenariogroupwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072626_rage_matrix34_rotatefullz_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_072627_cscenariodebug_getclosestscenariopoint_rage_vec3v_const_ref_csce(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_072628_cscenariodebug_spawnped_rage_vector3_const_ref_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i64; }
extern "C" { pub fn f_072630_cscenarioeditor_resetselections(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072632_cscenarioeditor_updatecurrentscenariogroupwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072633_cscenarioeditor_updateeditingpointwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072634_cscenarioeditor_rebuildregionnames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072636_cscenarioeditor_updatecurrentscenarioregionwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072637_cmodelsetnamedata_init_char_const_ptr_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072638_cscenarioeditor_rebuildscenarionames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072655_cscenariodebug_warpto_cped_ref_cscenariopoint_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_072656_cscenariomanager_chooserealscenario_int_cped_ptr(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_072659_cscenarioinfomanager_addparserwidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072660_cscenarioinfomanager_updatescenariotypeswidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072662_cscenariopointmanager_findpointsinarea_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_072663_cscenariomanager_checkscenariopointenabled_cscenariopoint_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_072665_cscenariodebug_canrenderpoint_cscenariopoint_const_ref_cpopzone(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_072666_cscenariomanager_checkscenarioprobability_cscenariopoint_const_p(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_072667_editorpoint_getscenariopointfromid_unsigned_int_cscenariopointre(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_072668_cscenariomanager_checkscenariopointmapdata_cscenariopoint_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072670_cscenariopointmanager_getregionname_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_072671_ctaskpolice_ctaskpolice_ctaskunalerted_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_072672_ctaskarmy_ctaskarmy_ctaskunalerted_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_072673_ctaskambulancepatrol_ctaskambulancepatrol_ctaskunalerted_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_072674_ctaskfirepatrol_ctaskfirepatrol_ctaskunalerted_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_072682_cscenarioeditor_scenariopointdataupdate_rage_vec3v_const_ptr_flo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072683_rage_atfixedarray_cscenarioeditor_cscenariopointundoitem_20_dtor(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072684_cscenarioanimdebughelper_dtorcscenarioanimdebughelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072687_cscenariopointregion_findoverrideforentity_centity_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_072688_cscenarioeditor_pushundoaction_cscenarioeditor_eundoaction_int_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_072693_cscenarioeditor_gatherselectedclusters_editorpoint_ref_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072717_cscenariopoint_copytodefinition_rage_fwextensiondef_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072718_cscenarioeditor_pushundoitem_cscenarioeditor_cscenariopointundoi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_072719_cscenariopointmanager_findoraddrequiredimapbyhash_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_072720_cscenariopointregioneditinterface_calculateaabb_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072721_cscenarioeditor_afterscenariopointdataupdate_cscenariopointregio(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_072722_cscenariopointcontainer_translateintopoint_cextensiondefspawnpoi(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072723_cscenariopointregioneditinterface_insertpoint_int_cscenariopoint(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_072724_cscenariopointmanager_findoraddinteriorname_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_072725_cscenariopointmanager_flagregionasedited_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_072726_cscenarioeditor_undo_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_072727_cscenarioeditor_currentregioncombochanged(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072730_cscenariopointregioneditinterface_flipchainedgedirection_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_072731_cscenariomanager_getscenariotypefromhashkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_072733_rage_atarray_cscenariopointmanager_savedata_0_unsigned_short_gro(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072734_rage_atarray_cscenariopointmanager_savedata_0_unsigned_short_des(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_072735_cscenariopointmanager_saveregion_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_072737_cscenariopointregion_dtorcscenariopointregion(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072738_cscenariopointregioneditinterface_onremovepoint_cscenariopoint_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_072739_cscenarioeditor_dodetachpoint_cscenariopoint_ref_cscenariopointr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_072740_cscenarioeditor_doattachpoint_int_cscenariopointregion_ref_centi(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_072741_cscenarioeditor_doremoveoverrideonentity_centity_ref_cscenariopo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072742_cscenariopointregioneditinterface_findoraddoverrideforentity_cen(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_072743_cscenariopointregion_applyoverridesforentity_centity_ref_bool_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_072744_cscenarioeditor_dorenamescenariogroup_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_072745_cscenarioeditor_doremovegroup_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_072746_cscenariopointmanager_addgroup_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072747_cscenariopointregioneditinterface_addchainingedge_cscenariochain(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_072748_cscenariopointregioneditinterface_deletechainingedgebyindex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_072749_cscenarioeditor_dorenameregion_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_072750_cscenarioeditor_doremoveregion_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_072751_cscenariopointmanager_addregion_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072752_cscenariopointregioneditinterface_deletepointsinaabb_rage_spdaab(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_072753_cscenariopointregioneditinterface_updatechaininggraphmappings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072754_cscenariopointregioneditinterface_removepointfromcluster_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_072755_cscenariopointregioneditinterface_deletecluster_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_072756_rage_atarray_cscenarioentityoverride_0_unsigned_short_delete_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_072757_cscenarioentityoverrideverification_clearinfoforregion_cscenario(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072758_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_072759_cscenariopointmanager_initcachedregionptrs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072761_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_072762_cscenariopointmanager_deletegroupbyindex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_072763_rage_atarray_cextensiondefspawnpoint_0_unsigned_short_op_rage_at(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072764_rage_atarray_rage_fwregdref_cscenariopoint_rage_fwrefawarebaseno(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072765_cscenariopointregioneditinterface_transferpoint_cscenariopoint_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_072766_rage_atarray_cscenarioentityoverride_0_unsigned_short_grow_unsig(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072767_rage_atarray_cscenarioentityoverride_0_unsigned_short_insert_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_072768_cscenarioentityoverride_init_centity_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072769_rage_atarray_cextensiondefspawnpoint_0_unsigned_short_copyfrom_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072770_cscenariopointregioneditinterface_logpointdependenciesfortransfe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_072771_cscenariopointregioneditinterface_addnewcluster(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072772_cscenariopointregioneditinterface_addpointtocluster_int_cscenari(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_072773_cscenariopointmanager_updateregionaabb_cscenariopointregion_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072774_cscenariopointmanager_deletepointswithattachedentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072775_cscenariopointmanager_addentitypoint_cscenariopoint_ref_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072776_cscenarioeditor_findselectedchainindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072777_cscenarioeditor_addtocluster_editorpoint_ref_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072778_rage_atarray_cscenariopointcluster_0_unsigned_short_grow_unsigne(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072779_cscenariopointcontainer_op_cscenariopointcontainer_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_072780_cscenariopointregioneditinterface_findcargensonchain_cscenarioch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_072781_cscenariopointregioneditinterface_resetgroupforusersof_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_072782_cscenariopointregioneditinterface_changegroup_unsigned_char_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_072784_cscenariomanager_shouldhavecargenundercurrentconditions_cscenari(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_072787_cscenarioentityoverrideverification_updateentityoverride_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_072789_cscenariofinder_findoptions_findoptions(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072794_cscenariopoint_removeuse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072795_cscenariopoint_adduse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072798_cscenariofinder_findnewscenario_cped_const_ptr_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_072799_cscenariofinder_checkcanscenariobeused_cscenariopoint_ref_cped_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_072800_cscenariomanager_checkscenariopopulationcount_int_rage_vector3_c(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_072801_cscenariopointmanager_ischainedwithincomingedges_cscenariopoint(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_072804_cscenariopointmanager_findchainedscenariopoints_cscenariopoint_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_072812_cscenarioinfomanager_clearpedusedscenarios(ph: *mut Phantom); }
extern "C" { pub fn f_072813_cscenarioinfomanager_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072814_rage_atarray_cscenarioinfomanager_sscenariosourceinfo_0_unsigned(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072815_cscenarioinfomanager_postload_rage_atarray_unsigned_int_0_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_072816_cscenariopointregion_callonaddscenarioforpoints(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072818_cscenariopointregion_callonremovescenarioforpoints(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072819_cscenarioinfomanager_cleardlcscenarioinfos_rage_atnonfinalhashst(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072821_cscenariomanager_updatecargen_cscenariopoint_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_072822_cscenariomanager_getconditionalanimsgroupforscenario_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_072825_cscenariomanager_islookatscenariotype_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_072826_cscenariomanager_checkcargenscenarioconditionsa_ccargenerator_co(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072828_cscenariomanager_applyvehiclecolors_cvehicle_ref_cambientvehicle(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072834_cscenariomanager_createtask_cscenariopoint_ptr_int_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32) -> i64; }
extern "C" { pub fn f_072837_cscenariopoint_cleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072841_cscenarioentityoverride_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_072846_cscenarioentityoverride_dtorcscenarioentityoverride(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072850_cscenariopointlookups_dtorcscenariopointlookups(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072856_cscenariopointcontainer_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_072857_cscenariochaininggraph_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_072858_cscenariopointcluster_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_072875_cscenariopointregiondef_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_072899_cscenariopointregion_removefromstore(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072901_cscenariopointmanager_dtorcscenariopointmanager_117504(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072905_cscenariopointmanager_invalidateregion_cscenariopointregiondef_p(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072906_cscenariopointmanager_loadpso_char_const_ptr_cscenariopointmanif(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_072907_cscenariopointmanager_handlefilesources_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_072908_cscenariopointmanager_populatesourcedata_char_const_ptr_cscenari(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_072909_cscenariopointmanager_postload(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072913_sgeneratefilenameformanifestpsosource_char_ptr_int_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_072917_rage_atarray_cscenariopointcluster_0_unsigned_short_destruct_csc(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_072924_cclonedscenariofsminfobase_getscenariopoint_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_072925_ctaskscenario_setscenariopoint_cscenariopoint_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072931_ctaskscenario_getworldscenariopointindex_cscenariopoint_ptr_int(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_072932_ctaskscenario_readqueriablestate_cclonedfsmtaskinfo_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072934_ctaskscenario_clonefindscenariopoint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072935_ctaskscenario_clonecheckscenariopointvalidandgetpointer_cscenari(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072936_cscenarioactiontriggers_dtorcscenarioactiontriggers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072938_ctaskusescenario_triggerexittoflee_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072942_ctaskusescenario_triggerexittocombat_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072946_ctaskusescenario_triggerimmediateexitonevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072949_ctaskusescenario_triggernormalexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072951_ctaskusescenario_hasavalidnormalexitanimation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073011_cscenarioactiontrigger_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_073063_cscenarioconditionpopulation_getpedmodelinfo_cscenariocondition(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073095_rage_atfixedbitsetbase_462_unsigned_int_issubsetof_rage_atfixedb(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_073132_cscenariocondition_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_073133_cscenarioconditionset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_073139_cscenarioconditionworld_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_073140_cscenarioconditionpopulation_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_073336_cscenarioinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_073354_cscenariovehicleinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_073396_cscenariotypegroup_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_073402_cscenarioinfomanager_parserpostload(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073404_cscenarioinfomanager_dtorcscenarioinfomanager_118278(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073406_cscenarioinfomanager_append_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073407_ctaskchatscenario_ctaskchatscenario_int_cped_ptr_int_cscenariopo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_073408_ctaskchatscenario_dtorctaskchatscenario(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073416_ctaskchatscenario_listening_onupdate_int_cped_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_073419_ctaskcouplescenario_dtorctaskcouplescenario(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073425_ctaskcouplescenario_isfollowerdeliberatelywaiting_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_073430_ctaskcowerscenario_dtorctaskcowerscenario(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073434_ctaskcowerscenario_possiblydestroypedcoweringoffscreen_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073436_ctaskcowerscenario_sendclipinformation_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_073437_ctaskcowerscenario_senddirectedclipinformation_rage_fwmvtemplate(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073438_ctaskcowerscenario_updatedirectedcowerblend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073439_ctaskcowerscenario_playidleaudio(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073440_ctaskcowerscenario_playflinchaudio(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073444_ctaskcowerscenario_shouldupdatecowerstatus_rage_aievent_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_073445_ctaskcowerscenario_updatecowerstatus_rage_aievent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073465_ctaskparkedvehiclescenario_dtorctaskparkedvehiclescenario(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073472_ctaskusescenarioentityextension_removetask_ctaskusescenario_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073473_ctaskusescenario_attach_rage_vector3_const_ref_rage_quaternion_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_073476_ctaskusescenario_setdummyobjectforattachedentity_cdummyobject_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073477_ctaskusescenario_ctaskusescenario_ctaskusescenario_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_073478_ctaskusescenario_setadditionaltaskduringapproach_ctask_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073479_ctaskusescenario_dtorctaskusescenario(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073482_ctaskusescenario_hastriggeredexit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073485_ctaskusescenario_forgetaboutagitation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073487_ctaskusescenario_triggerexitforcowarddisputes_bool_cped_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_073488_ctaskusescenario_chooseclipinclipset_cconditionalanims_eanimtype(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_073489_ctaskusescenario_areenterclipsstreamed(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073491_ctaskusescenario_networksetuppedpositionandattachments(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073495_ctaskusescenario_lookattarget_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073496_ctaskusescenario_handleonenterambientscenarioinfoflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073497_ctaskusescenario_handlepanicsetup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073498_ctaskusescenario_leavescenario_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_073499_ctaskusescenario_adjusttargetforclips_rage_vector3_ref_float_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_073500_ctaskusescenario_adjustgotopositionforpedheight_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073501_ctaskusescenario_setupforscenariotransition_cscenariotransitioni(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_073502_ctaskusescenario_checkattachstatus(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073503_ctaskusescenario_ragdolloutofscenario(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073504_ctaskusescenario_determineexitanimations_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_073505_ctaskusescenario_readytoleavescenario_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_073506_ctaskusescenario_setstationaryreactionsenabledforped_cped_ref_bo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073507_ctaskusescenario_updatetargetsituation_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_073508_ctaskusescenario_randomizecowerpanicrateandupdatelastrate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073509_ctaskusescenario_computenextphase_ccliphelper_const_ref_rage_crc(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_073510_ctaskusescenario_endbecauseoffalltagging(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073511_ctaskusescenario_checkforinterruptibletag_float_rage_crclip_cons(ph: *mut Phantom, a0: f32, a1: i64) -> i32; }
extern "C" { pub fn f_073512_ctaskusescenario_startnewscenarioafterhangout(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073514_ctaskusescenario_calculatetargetsituation_rage_vector3_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_073515_ctaskusescenario_setstationaryreactionsfromscenariopoint_cscenar(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073516_ctaskusescenario_getcliporientationproperty_rage_crclip_const_pt(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_073524_ctaskusescenario_createmedicscenariopoint_rage_vector3_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_073535_ctaskusevehiclescenario_ctaskusevehiclescenario_int_cscenariopoi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: f32, a5: i64) -> i64; }
extern "C" { pub fn f_073536_ctaskusevehiclescenario_ctaskusevehiclescenario_ctaskusevehicles(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_073540_ctaskusevehiclescenario_shouldslowdown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073554_ctaskwanderingscenario_dtorctaskwanderingscenario(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073561_ctaskwanderingscenario_waitfornextscenario_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073562_ctaskwanderingscenario_createtasktomoveto_rage_vec3v_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_073563_ctaskwanderingscenario_getwanderscenariotousenext_int_cped_ptr(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_073577_ctaskarmy_dtorctaskarmy(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073580_ctaskarmy_checkfororder(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073586_ctaskfirepatrol_dtorctaskfirepatrol(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073594_ctaskfirepatrol_patrolaspassenger_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073595_ctaskfirepatrol_attempttocreateresumableunalertedtask(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073596_ctaskfirepatrol_getbesttrucktargetlocation_cped_ptr_cfireorder_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_073597_ctaskfirepatrol_evaluateusingmountedwaterhose_cfireorder_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_073598_cpathfind_isregionloaded_rage_cnodeaddress_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073599_ctaskfirepatrol_registerfightingfire_cfire_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073601_ctaskpolice_dtorctaskpolice(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073608_ccriminalscanner_scan_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_073613_ctaskpoliceorderresponse_dtorctaskpoliceorderresponse(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073618_ctaskpoliceorderresponse_searchinvehicle_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073619_ctaskpoliceorderresponse_generatepoliceordertype_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073620_ctaskpoliceorderresponse_generateorderstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073621_ctaskpoliceorderresponse_processinvestigateincidentinvehiclelook(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073623_ctaskpoliceorderresponse_hastargetpassedofficer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073624_ctaskpoliceorderresponse_willtargetvehicleovertakeofficervehicle(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_073628_ctaskarrestped2_ctaskarrestped2_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_073629_ctaskarrestped2_dtorctaskarrestped2(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073632_ctaskarrestped2_setplayerlockedon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073643_ctaskarrestped_dtorctaskarrestped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073648_ctaskarrestped_canarrestpedinvehicle_cped_const_ptr_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_073650_ctaskarrestped_isanothercopclosertotarget(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073651_ctaskarrestped_updatetalktargetdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073652_ctaskbusted_ctaskbusted_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_073655_ctaskbusted_dtorctaskbusted(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073664_ctaskpolicewantedresponse_dtorctaskpolicewantedresponse(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073673_ctaskswat_activatetimeslicing(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073678_ctaskswatorderresponse_dtorctaskswatorderresponse(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073682_ctaskswatorderresponse_aretargetswhereaboutsknown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073683_ctaskswatorderresponse_searchonfoot_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073686_ctaskswatfollowinline_dtorctaskswatfollowinline(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073691_ctaskswatfollowinline_leaderstartadvancing_corder_const_ptr_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_073695_ctaskswatgotostagingarea_aretargetswhereaboutsknown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073699_ctaskswatwantedresponse_dtorctaskswatwantedresponse(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073710_ctaskheliorderresponse_dtorctaskheliorderresponse(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073713_ctaskheliorderresponse_hasrappellingpeds_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073714_ctaskheliorderresponse_haspedswaitingtobedroppedoff_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073717_ctaskheliorderresponse_findtargetposition_cped_ptr_corder_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_073718_ctaskheliorderresponse_activatetimeslicing(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073719_ctaskheliorderresponse_canseetarget_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073721_ctaskambulancepatrol_dtorctaskambulancepatrol(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073725_ctaskambulancepatrol_pullfromvehicle_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073726_ctaskambulancepatrol_pullfromvehicle_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073727_ctaskambulancepatrol_exitvehicleandchooseresponse_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073728_ctaskambulancepatrol_exitvehicleandchooseresponse_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073729_ctaskambulancepatrol_playrecoverysequence_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073730_ctaskambulancepatrol_playrecoverysequence_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073731_ctaskambulancepatrol_playrecoverysequence_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073732_ctaskambulancepatrol_returntoambulance_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073733_ctaskambulancepatrol_returntoambulance_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073734_ctaskambulancepatrol_superviserecovery_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073735_ctaskambulancepatrol_superviserecovery_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073736_ctaskambulancepatrol_superviserecovery_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073737_ctaskambulancepatrol_revivefailed_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073738_ctaskambulancepatrol_revivefailed_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073739_ctaskambulancepatrol_waitforbothmedicstofinish_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073740_ctaskambulancepatrol_waitforbothmedicstofinish_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073741_ctaskambulancepatrol_waitinambulance_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073742_ctaskambulancepatrol_waitinambulance_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073743_ctaskambulancepatrol_attempttocreateresumableunalertedtask(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073744_ctaskambulancepatrol_checkbodycanbereached_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_073745_ctaskambulancepatrol_hasbodymoved_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_073746_ctaskambulancepatrol_lookattarget_cped_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073747_ctaskambulancepatrol_updatecheckfornewreviver(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073763_ctaskwitness_ctaskwitness_cped_ptr_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_074161_ctask_ctask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_074162_ctask_getname_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_074163_ctaskclassinfomanager_gettaskname_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_074164_cmovenetworkhelper_getclipsetid_rage_fwmvtemplateid_rage_fwmvcli(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_074168_ctask_requestprocessmovesignalcalls(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_074169_ctask_propagatenewmovespeed_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_074170_ctask_isvalidformotiontask_ctaskmotionbase_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_074172_ccliphelper_stopclip_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_075327_ctaskcomplex_ctaskcomplex(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075332_ctaskfsmclone_ctaskfsmclone(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075333_ctaskfsmclone_readqueriablestate_cclonedfsmtaskinfo_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075334_ctaskfsmclone_isinscope_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075337_ctaskfsmclone_makeabortable_rage_aitask_abortpriority_rage_aieve(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_075339_ctaskfsmclone_createsubtaskfromclone_cped_ptr_ctasktypes_etaskty(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_075341_ctaskhelperfsm_ctaskhelperfsm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075343_ctaskhelperfsm_setstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
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
extern "C" { pub fn f_075361_cnavmeshclosestpositionhelper_startclosestpositionsearch_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_075367_cnavmeshclosestpositionhelper_resetsearch(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075368_cprioritizedclipsetrequesthelper_dtorcprioritizedclipsetrequesth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075369_cprioritizedclipsetrequesthelper_releaseclipset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075370_cprioritizedclipsetrequesthelper_isclipsetstreamedin_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075372_cprioritizedclipsetrequesthelper_getclipsetid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075373_cprioritizedclipsetrequesthelper_requestclipset_rage_fwmvtemplat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075374_cprioritizedclipsetrequesthelper_requestclipsetifexists_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075375_cprioritizedclipsetrequesthelper_setstreamingpriorityoverride_ra(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_075386_cambientcliprequesthelper_pickclipbydict_fullconditions_int_unsi(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_075387_cambientcliprequesthelper_fillcliphasharraywithexistingclips_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075388_cambientcliprequesthelper_pickclipbygroup_simpleconditions_cdyna(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075389_cnavmeshroutesearchhelper_cnavmeshroutesearchhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075390_cnavmeshroutesearchhelper_dtorcnavmeshroutesearchhelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075391_cnavmeshroutesearchhelper_startsearch_cped_ptr_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i64, a6: i32, a7: i64, a8: f32, a9: i32); }
extern "C" { pub fn f_075392_cnavmeshroutesearchhelper_getsearchresults_searchstatus_ref_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_075398_cvehiclefollowroutehelper_getvehiclebonnetposition_cvehicle_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_075422_cvehiclefollowroutehelper_cvehiclefollowroutehelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075428_cvehiclefollowroutehelper_calculatedesiredspeeds_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_075442_cnavmeshlineofsighthelper_cnavmeshlineofsighthelper(ph: *mut Phantom, a0: i64) -> i64; }
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
extern "C" { pub fn f_075476_csimpleblendhelper_findclipbyindexandhash_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_075479_cmovenetworkhelper_releasenetworkplayer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075482_csimpleblendhelper_blendoutclip_rage_eanimpriority_float_unsigne(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_075484_cgenericcliphelper_cgenericcliphelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075485_cgenericcliphelper_dtorcgenericcliphelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075486_cgenericcliphelper_blendinclipbysetandclip_cdynamicentity_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_075490_cgenericcliphelper_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075491_cmovenetworkhelper_setfilter_rage_crframefilter_ptr_rage_fwmvtem(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075500_cmovenetworkhelper_setclipset_rage_fwmvtemplateid_rage_fwmvclips(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075502_cmovenetworkhelper_getclip_rage_fwmvtemplateid_rage_fwmvclipidty(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_075503_cmovenetworkhelper_setboolean_rage_fwmvtemplateid_rage_fwmvboole(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075504_cmovenetworkhelper_getfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075514_cplayattachedcliphelper_cplayattachedcliphelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075515_cplayattachedcliphelper_setinitialoffsets_cped_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075516_cplayattachedcliphelper_computeinitialsituation_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075517_cplayattachedcliphelper_computesituationfromclip_rage_crclip_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_075518_cplayattachedcliphelper_computeoffsets_rage_crclip_const_ptr_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_075523_ctuning_dtorctuning(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075524_cvehiclecliprequesthelper_cvehiclecliprequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075525_cvehiclecliprequesthelper_dtorcvehiclecliprequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075529_cvehiclecliprequesthelper_requestclipsfromvehicle_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_075531_cvehiclecliprequesthelper_choosevariationclipsetfromentryindex_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_075547_cdirectionhelper_calculateclosestdirection_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_075555_ctalkhelper_talk_cped_ref_cped_ref_cambientaudio_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075564_cweaponinfohelper_calculatestate_cped_const_ref_cweaponinfohelpe(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075567_cequipweaponhelper_shouldpedswapweapon_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075583_cheadingfixuphelper_apply_cped_ref_float_rage_crclip_const_ref_f(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_075584_cexplosionhelper_calculatetimeuntilexplosion_centity_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075586_cscenariocliphelper_cscenariocliphelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075587_cscenariocliphelper_resetpreviouswinners(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075588_cscenariocliphelper_dtorcscenariocliphelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075589_cscenariocliphelper_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075591_cscenariocliphelper_canstoreanymorewinningindices_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075593_cscenariocliphelper_pickclipfromset_cped_const_ptr_centity_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: f32, a12: f32) -> i32; }
extern "C" { pub fn f_075594_cscenariospeedhelper_shouldusespeedmatching_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075595_cscenariospeedhelper_getmbrtomatchspeed_cped_const_ref_cped_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_075596_ctaskmovebase_computemoveblend_cped_ref_float(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_075597_cfleehelpers_initializesecondarytarget_caitarget_const_ref_caita(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075598_cfleehelpers_processtargets_caitarget_ref_caitarget_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075599_ctimehelpers_gettimesince_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_075602_chelirefuelhelper_process_cped_const_ref_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075603_chelifleehelper_createtaskforped_cped_const_ref_caitarget_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_075604_ctaskcontrolvehicle_ctaskcontrolvehicle_cvehicle_ptr_rage_aitask(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_075616_cpedswaitingtobeoutofsightmanager_add_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075617_cpedswaitingtobeoutofsightmanager_remove_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075631_ctaskmove_ctaskmove_float(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_075632_ctaskmove_dtorctaskmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075636_ctaskmovebase_ctaskmovebase_float(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_075644_ctasktreeclone_startlocalclonetask_ctaskfsmclone_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075660_ctuning_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_075664_ctuning_registertuning_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_075693_cmotiontaskdatamanager_getdataset_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_075697_ctaskexitvehicle_ctaskexitvehicle_cvehicle_ptr_cvehicleenterexit(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_075702_ctaskinvehiclebasic_ctaskinvehiclebasic_cvehicle_ptr_bool_ccondi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_075710_ctaskvehiclegun_ctaskvehiclegun_ctaskvehiclegun_etaskmode_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_075724_ctaskentervehicle_ctaskentervehicle_cvehicle_ptr_seatrequesttype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: f32, a6: f32, a7: i64, a8: i64) -> i64; }
extern "C" { pub fn f_075727_ctaskcardrivewander_ctaskcardrivewander_cvehicle_ptr_int_float_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_075738_ctaskcarsettempaction_ctaskcarsettempaction_cvehicle_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_075758_ctaskvehiclefsm_getpedinorusingseat_cvehicle_const_ref_int_cped(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_075819_ccarenterexit_getcollideddoorfromcollisioncomponent_cvehicle_con(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i64; }
extern "C" { pub fn f_076102_ctaskexitvehicle_selectrappelclipsetforheliseat_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_076263_ctaskridetrain_ctaskridetrain_ctrain_ptr_ctask_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_076293_ctaskvehiclefsm_getexittoaimclipsetidforped_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076318_ctaskvehiclemountedweapon_ctaskvehiclemountedweapon_ctaskvehicle(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_076655_ctaskplayerweapon_ctaskplayerweapon_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_076717_ctaskswapweapon_ctaskswapweapon_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_076853_cscriptedguntaskmetadatamgr_getscriptedguntaskinfobyhash_unsigne(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_076885_ctaskgun_setupaiminggetupset_cnmblendoutsetlist_ref_cped_ptr_cai(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_076890_ctaskaimsweep_ctaskaimsweep_ctaskaimsweep_aimtype_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_076958_ctaskaimgunvehicledriveby_isflippingsomeoneoff_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077126_cstatsmgr_startbailvehicle_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077153_statid_char_getstatid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077331_statsinterface_getintstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077533_ctextlayout_ctextlayout(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077535_ctextlayout_dtorctextlayout(ph: *mut Phantom); }
extern "C" { pub fn f_077537_ctextlayout_setscale_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077745_cdriverpersonality_findmaxcruisespeed_cped_const_ptr_cvehicle_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> f32; }
extern "C" { pub fn f_077791_cpathfind_checkpathnodeintegrityforzone_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077792_cpathfind_testalllinks_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077801_cvehicleintelligence_getnodelist_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077802_cvehicleintelligence_getfollowroutehelper_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077803_cvehicleintelligence_getactivetask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077810_cpathfind_arenodesloadedforarea_float_float_float_float_const(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_077812_cpathfind_findnodeclosesttocoors_rage_vector3_const_ref_float_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: f32, a11: i32, a12: i64, a13: i32, a14: i32); }
extern "C" { pub fn f_077864_cpathfind_findnodeslinkindexbetween2nodes_rage_cpathnode_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_077874_cpathfind_findroadboundaries_int_int_float_bool_bool_float_ref_f(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: i32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_077892_cpathfind_findnodeindirection_cpathfind_findnodeindirectioninput(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077935_cvehicleailodmanager_computerealdriversandpassengersremaining(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077959_cvehicleintelligence_isonhighway_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077962_cvehicleintelligence_checkrouteinterception_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: f32) -> i32; }
extern "C" { pub fn f_077966_cvehicleintelligence_doesroutepassbyposition_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_077970_cvehicleintelligence_getgototaskforvehicle_cvehicle_ptr_cphysica(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: f32, a6: f32) -> i64; }
extern "C" { pub fn f_077974_ctaskvehiclegotohelicopter_ctaskvehiclegotohelicopter_svehiclemi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_077975_cvehicleintelligence_createautomobilegototask_svehiclemissionpar(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_077982_cvehicleintelligence_createcruisetask_cvehicle_ref_svehiclemissi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_077983_ctaskvehicleflee_ctaskvehicleflee_svehiclemissionparams_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_077985_ctaskvehicleapproach_ctaskvehicleapproach_svehiclemissionparams(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077991_ctaskvehiclegotoplane_ctaskvehiclegotoplane_svehiclemissionparam(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: f32) -> i64; }
extern "C" { pub fn f_077996_ctaskvehicleparknew_ctaskvehicleparknew_svehiclemissionparams_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i32) -> i64; }
extern "C" { pub fn f_078012_ctaskvehiclestop_ctaskvehiclestop_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_078017_cheliintelligence_islandingareablocked_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078018_cheliintelligence_islanded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078085_ctaskvehiclemissionbase_setcruisespeed_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078144_ctaskvehiclecruisenew_setpreferredtargetnodelist_cvehiclenodelis(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078203_ctaskvehiclefleeboat_ctaskvehiclefleeboat_svehiclemissionparams(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_078224_ctaskvehiclefleeairborne_computeflightheightaboveterrain_cvehicl(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078225_ctaskvehiclefleeairborne_ctaskvehiclefleeairborne_svehiclemissio(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_078336_ctaskvehiclelandplane_ctaskvehiclelandplane_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_078472_ctaskbringvehicletohalt_ctaskbringvehicletohalt_float_int_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_078784_canchorhelper_anchor_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078831_cvehicle_getreferenceframevelocity_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078886_cvehicle_canpedsstandontop_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078888_ccardoor_settargetdooropenratio_float_unsigned_int_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64); }
extern "C" { pub fn f_078918_cvehicle_setcardoorlocks_ecarlockstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078924_cvehicle_getwheelfromid_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_078932_cvehicle_getvelocityincludingreferenceframe_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078933_cvehicle_isupsidedown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078934_cvehicle_isonitsside_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078935_cvehicle_getisextraon_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078939_cvehicle_switchengineon_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_078955_cvehicle_turnsirenon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_078958_ccomponentreservationmanager_getseatreservationfromseat_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_078959_ccomponentreservation_iscomponentinuse(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078975_cvehicledamage_getisstuck_int_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_078981_cvehicle_hascontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078996_clandinggear_controllandinggear_cvehicle_ptr_clandinggear_ecomma(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_079000_cvehicle_istank_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079007_cvehicle_getnumberofpassenger_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079023_cvehicle_getattachedtrailer_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079039_cvehicle_isturretseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079058_cvehicle_trytomakepedsfrompretendoccupants_bool_int_const_ptr_ca(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_079065_cvehicle_getdoorfromid_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079070_cvehicle_getseatanimationinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079071_cvehicle_isentryindexvalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079098_cvehicle_selectappropriategearforspeed(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079130_cvehicle_getlayoutinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079134_cvehicle_getdriverseat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079157_cvehicle_getisjetski_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079169_cboat_getsearchlight(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079197_ccargenerator_switchoff(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079199_ccargenerator_isused_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079201_ccargenerator_checkisactiveforgen(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079202_ccargenerator_checkpopulationhasspace(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079203_ccargenerator_pickvalidmodels_rage_fwmodelid_ref_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_079204_ccargenerator_checkscenarioconditionsb_rage_fwmodelid_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_079205_ccargenerator_checknetworkobjectlimits_rage_fwmodelid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079206_ccargenerator_checkhasspacetoschedule(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079207_ccargenerator_registerspawnfailure_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079208_ccargenerator_buildcreationmatrix_rage_matrix34_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079209_ccargenerator_insertvehforgen_rage_fwmodelid_const_ref_rage_fwmo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_079210_ccargenerator_dosyncprobeforblockingobjects_rage_fwmodelid_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_079213_ccargenerator_checkifwithinrangeofanyplayers_bool_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_079217_cthecargenerators_addtogameworld_cvehicle_ref_cvehicle_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079225_cthecargenerators_destroycargeneratorbyindex_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_079226_cvehicle_setliveryid_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079227_cvehicle_setlivery2id_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079228_cvehicle_updatebodycolourremapping_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079230_cthecargenerators_createcargenerator_float_float_float_float_flo(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i32, a14: i32, a15: i32, a16: i32, a17: i32, a18: i64, a19: i32, a20: i32, a21: i32) -> i32; }
extern "C" { pub fn f_079231_cthecargenerators_dumpcargeninfo(ph: *mut Phantom); }
extern "C" { pub fn f_079245_cthecargenerators_requestvehiclemodelforscenario_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079253_ccomponentreservationmanager_getseatreservation_int_seataccess_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_079255_ccomponentreservationmanager_findcomponentreservationbyarrayinde(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079267_ccardoor_getisintact_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079335_chandlingdata_gethandlingdata_ehandlingtype(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079372_crotarywingaircraft_getismainrotorbroken_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079412_cvehicle_setupdriver_bool_bool_unsigned_int_cambientmodelvariati(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_079413_cvehicle_delayedremovaltimereset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079414_cheli_getropeattachposition_rage_vector3_ref_bool_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_079417_cheli_areropesattachedtononhelientity_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079490_caircraftdamage_getliftmult_cplane_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_079581_cvehicle_isindriveablestate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079730_cvehicle_setsuperdummyvelocity_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079746_cvehicle_doesvehiclehaveaconvertibleroofanimation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079758_cvehicle_getconvertibleroofstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079791_cvehicle_isdriver_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079795_cvehiclepopulation_adddrivingflagsforambientvehicle_int_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079797_cvehicle_hasalivevictiminit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079812_cvehicle_getentryinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079813_cvehicle_getdoorfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079815_cvehicle_getseatanimationinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_079838_cvehicle_fleesfromcombat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079841_cvehicle_isonfire_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079843_cvehicle_carhasroof_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079854_cvehicle_containsped_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079855_cvehicle_canbedriven_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079866_cvehicle_turnoffextra_ehierarchyid_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079883_cvehiclelayoutinfo_getseatinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079885_cvehicle_getentryexitpoint_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079887_cvehicle_isentrypointindexvalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079889_cvehicle_isentrypointclearforped_cvehicle_const_ref_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_079899_cvehicle_getrelativevelocity_cped_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079902_cvehicle_setforwardspeed_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079932_cvehicleweaponmgr_getfirstturretforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080000_cvehicledamage_setenginehealth_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_080402_cvehiclepopulation_findnumrandomvehsthatcanbecreated(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_080410_cvehiclepopulation_shouldremovevehicle_cvehicle_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_080666_cvehiclemetadatamgr_getseataniminfofromped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080668_cvehiclemetadatamgr_getvehiclescenariolayoutinfo_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_080687_cvehiclescenariolayoutinfo_getscenariopointinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080852_bool_creplaymgrinternal_handleentityforfx_cpacketattachobjectsto(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_081355_cfire_getpositionworld_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081439_cfiremanager_getentityfire_centity_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081450_cfire_registerinusebydispatch(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081750_cvehicleglassmanager_iscomponentsmashableglass_cphysical_const_p(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082065_cpedweaponmanager_getequippedweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082069_cweapon_getmuzzleposition_rage_matrix34_const_ref_rage_vector3_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082071_cpedweaponmanager_getequippedweapon_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082124_cweaponinfo_getforcehitped_eanimbonetag_bool_float_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32) -> f32; }
extern "C" { pub fn f_082175_cweaponinfo_getappropriateweaponclipsetid_cped_const_ptr_fpsstre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_082179_cpedweaponmanager_destroyequippedweaponobject_bool_char_const_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_082185_cweapon_calcfirevecfromgunorientation_centity_const_ptr_rage_mat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_082189_cweapon_getneedstoreload_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082206_cweaponinfo_getweaponclipsetid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082215_cpedweaponmanager_dropweapon_unsigned_int_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_082252_cpedweaponmanager_getbestweaponinfo_bool_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_082429_cweaponanimationsmanager_getjumpupperbodyclipsethash_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082502_cweaponinfo_getpedmotionclipsetid_cped_const_ref_fpsstreamingfla(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_082503_cweaponinfo_getpedmotionfilterid_cped_const_ref_fpsstreamingflag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082507_cweaponinfo_getshellshockedclipsetid_cped_const_ref_fpsstreaming(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082553_cpedinventory_removeall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082572_cpedinventory_addweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_082580_cpedinventory_addweaponandammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_082594_cpedweaponmanager_equipweapon_unsigned_int_int_bool_bool_cpedequ(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_082600_cpedweaponmanager_createequippedweaponobject_cpedequippedweapon(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082602_cpedweaponmanager_unregisternmweapon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082635_cpedweaponmanager_getrequiresweaponswitch_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082644_cpedweaponmanager_equipbestweapon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_082647_cpedweaponmanager_getbestweaponinfobygroup_unsigned_int_bool_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_082653_cpedweaponmanager_getisarmedgun_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082655_cpedweaponmanager_getisarmed1handed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082656_cpedweaponmanager_getisarmed2handed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082671_cweaponitem_getkeyfromhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083023_networkinterface_isaspectatingplayer_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083026_networkinterface_isplayerpedspectatingplayerped_cped_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083027_networkinterface_isspectatingped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083028_networkinterface_isinspectatormode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083041_networkinterface_canregisterobject_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083043_networkinterface_getplayerfocusposition_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_083061_networkinterface_getnetworkobject_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083107_networkinterface_getlocalropeidfromnetworkid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083108_networkinterface_isposlocallycontrolled_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083109_networkinterface_forcesynchronisation_cdynamicentity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083113_networkinterface_isinvalidvehiclefordriving_cped_const_ptr_cvehi(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083114_networkinterface_isinvalidvehicleforambientpeddriving_cvehicle_c(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083121_networkinterface_forcehighupdatelevel_cphysical_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083125_networkinterface_getlastposreceivedovernetwork_cdynamicentity_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083131_networkinterface_useanimatedragdollfallbackblending_cped_const_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083134_networkinterface_gostraighttotargetposition_cdynamicentity_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083135_networkinterface_overridenetworkblenderfortime_cphysical_const_p(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083142_networkinterface_requestnetworkanimrequest_cped_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_083167_networkinterface_areinteractionsdisabledinmp_centity_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083175_networkinterface_locallyhideentityforaframe_centity_const_ref_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_083201_networkinterface_writedatapopulationlog_char_const_ptr_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_083680_networkutils_getpedfromnetworkobject_rage_netobject_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083831_serialiseanimation_rage_csyncdatabase_ref_rage_fwmvtemplateid_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_083898_csyncedentity_csyncedentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083899_csyncedentity_csyncedentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083900_csyncedentity_setentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083902_csyncedentity_getentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083903_csyncedentity_getentityid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083904_csyncedentity_setentityid_unsigned_short_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083907_csyncedvehicle_csyncedvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083908_csyncedvehicle_getvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083909_csyncedvehicle_getvehicleid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083910_csyncedvehicle_setvehicleid_unsigned_short_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086744_cnetgameplayer_getplayerped_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087136_cnetobjped_forceresendalldata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087160_cnetobjped_forceresendofalltaskdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087329_cnetblenderped_isstandingonobject_unsigned_short_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_089329_crequestcontrolevent_trigger_rage_netobject_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_089522_cragdollrequestevent_trigger_unsigned_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090714_atanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090717_atof(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090818_modff(ph: *mut Phantom, a0: f32, a1: i64) -> f32; }
extern "C" { pub fn f_090901_qsort_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_090910_srand(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_090911_rand(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090918_sinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090920_snprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090921_siprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090922_small_sprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090923_sscanf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090926_strcasecmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090930_strcmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090931_strcpy(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090932_strcspn(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090939_strlen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090940_strncasecmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090947_strstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090956_strtok(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090962_system(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090965_tanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_091029_void_std_2_sort_std_2_less_unsigned_short_unsigned_short_ref_uns(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
