//! Callee declarations for crate gen006.
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
extern "C" { pub fn f_000116_rage_sysipcwaitsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000119_rage_sysipcsignalsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000121_rage_sysipclockmutexcommand_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000122_rage_sysipcunlockmutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000126_rage_sysipcpollevent_rage_sysipceventtag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000134_rage_sysipcyield_int(ph: *mut Phantom); }
extern "C" { pub fn f_000135_rage_sysipcsleep_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000644_rage_athashnextsize_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000647_rage_atreferencecounter_atreferencecounter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000653_rage_atstring_set_rage_atstring_const_ref_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_000663_rage_atstring_replace_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000668_rage_atarray_rage_atstring_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_000675_rage_functorbase_functorbase_void_const_ptr_void_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_000680_rage_athashstringnamespacesupport_computehash_rage_athashstringn(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_000681_rage_athashstringnamespacesupport_addstring_rage_athashstringnam(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_000684_rage_athashstringnamespacesupport_trygetstring_rage_athashstring(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_000696_rage_bkwidget_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000703_rage_bkbank_unsetcurrentgroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000704_rage_bkbank_pushgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000705_rage_bkgroup_addgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000707_rage_bkbank_popgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000782_rage_bkmanager_findbank_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000783_rage_bkmanager_findwidget_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000786_rage_bkmanager_isbankthread(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000806_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000866_rage_bkgroup_addseparator_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000867_rage_bkgroup_addtitle_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000868_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_000869_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000876_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_000877_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_000882_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000894_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000906_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000909_rage_bkgroup_addslider_char_const_ptr_rage_vector2_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000910_rage_bkgroup_addslider_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64); }
extern "C" { pub fn f_000911_rage_bkgroup_addslider_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000912_rage_bkgroup_addslider_char_const_ptr_rage_vector4_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000918_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000919_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000922_rage_bkgroup_adddatawidget_char_const_ptr_unsigned_char_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_000927_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001116_char_ptr_rage_formatf_char_32ul_char_ref_32ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001514_rage_diagdisplayf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001516_rage_fistream_write_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001524_displayf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001527_warningf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001530_rage_fistream_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001532_rage_fistream_close(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001534_quitf_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001535_rage_diaglogf2_rage_diagchannel_const_ref_rage_diagseverity_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001549_rage_diaglogdefault_rage_diagchannel_const_ref_rage_diagseverity(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_001551_rage_fprintf_rage_fistream_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001565_printf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001568_rage_atmapentry_unsigned_int_wchar_t_ptr_const_ref_rage_atmap_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001632_rage_fiassetmanager_pushfolder_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001633_rage_fiassetmanager_popfolder(ph: *mut Phantom); }
extern "C" { pub fn f_001643_rage_fistream_create_char_const_ptr_rage_fidevice_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
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
extern "C" { pub fn f_002156_rage_parmanager_initfromstructure_rage_parstructure_ref_rage_par(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002158_rage_parinstancevisitor_visittoplevelstructure_rage_parparsables(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002161_rage_parstructure_issubclassof_rage_parstructure_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002164_rage_parmanager_loadpatchfile_char_const_ptr_char_const_ptr_rage(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_002257_rage_parmemberstring_getsubtype_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002260_rage_parinstancevisitor_visitmember_rage_parparsablestructure_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002278_rage_psofile_dtorpsofile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002282_rage_psoloadfile_char_const_ptr_rage_psoloadaction_rage_atfixedb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002288_rage_psoloadfromstructure_rage_psofile_ref_rage_parstructure_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002294_rage_atarray_rage_psorscenumschemavaluedata_0_unsigned_short_gro(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002343_rage_parrestregistersingleton_char_const_ptr_rage_parparsablestr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002369_rage_parstructure_findmember_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002402_rage_parenumdata_namefromvalueunsafe_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002436_rage_parcguregisterstructure_rage_parstructure_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002437_rage_parcgustructure_createstructure_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002438_rage_parcgustructure_setfactories_rage_parstructure_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002439_rage_parcgustructure_setgetstructurecb_rage_parstructure_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002440_rage_parcgustructure_setbaseclass_rage_parstructure_ptr_rage_par(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002444_rage_parcgustructure_buildstructurefromstaticdata_void_ptr_rage(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002447_rage_parinstancevisitor_parinstancevisitor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002571_rage_parbuildwidgetsvisitor_stringmember_rage_parparsablemember(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002743_rage_atdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002839_rage_sysparam_get_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002840_rage_sysparam_get_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002852_rage_sysstack_printcapturedstacktrace_unsigned_long_const_ptr_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002875_rage_atmap_unsigned_int_rage_sysstackcollector_tagentry_rage_atm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002879_rage_sysstackcollector_unregistertag_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002881_unsigned_int_ref_rage_atmap_unsigned_int_unsigned_int_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002882_rage_sysstackcollector_collectstack_unsigned_int_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_002884_rage_sysstackcollector_printinfofortag_unsigned_int_void_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_003065_rage_sysmemcontainer_sysmemcontainer_rage_sysmemcontainerdata_re(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003066_rage_sysmemcontainer_dtorsysmemcontainer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003071_rage_sysmemcontainer_init_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003072_rage_sysmemcontainer_finalize(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003075_rage_systinyheap_init_void_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003111_rage_sysdependencyscheduler_insertinternal_rage_sysdependency_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003159_rage_matrix34_fromquaternion_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003160_rage_matrix34_toquaternion_rage_quaternion_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003161_rage_quaternion_frommatrix34_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003163_rage_quaternion_slerp_float_rage_quaternion_const_ref_rage_quate(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_003166_rage_matrix44_inverse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003170_rage_quaternion_slerp_float_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003173_rage_vector2_vector2_rage_vector3_const_ref_rage_vector2_evector(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_003176_rage_vector3_approachstraight_rage_vector3_const_ref_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003433_rage_atmap_unsigned_int_rage_crframedata_ptr_rage_atmaphashfn_un(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003806_rage_fwanimdirector_waitforanyactiveupdatetocomplete_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003825_rage_fwanimdirector_startupdateprerender_float_rage_crmtmotiontr(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32); }
extern "C" { pub fn f_003828_rage_fwanimdirector_findpropertyinmovenetworks_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003829_rage_fwanimdirectorcomponentsyncedscene_issyncedscenelooped_shor(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003832_rage_fwanimdirectorcomponentsyncedscene_isvalidsceneid_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003833_rage_fwanimdirectorcomponentsyncedscene_getsyncedscenetime_short(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_003838_rage_fwclipset_getclip_rage_fwmvtemplateid_rage_fwmvclipidtype_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_003840_rage_fwclipsetmanager_getclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003858_rage_fwanimdirector_waitforallupdatestocomplete(ph: *mut Phantom); }
extern "C" { pub fn f_003859_rage_fwanimdirector_waitformotiontreebyphase_rage_fwanimdirector(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003893_rage_fwanimhelpers_applyinitialoffsetfromclip_rage_crclip_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003896_rage_fwanimmanager_getclipifexistsbydictindex_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_003904_rage_fwanimmanager_findslot_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003905_rage_fwanimmanager_findslotfromhashkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003907_rage_fwanimmanager_isvalidslot_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003908_rage_fwanimmanager_removerefwithoutdelete_rage_strlocalindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_003910_rage_fwanimmanager_streamingrequest_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_003911_rage_fwanimmanager_addref_rage_strlocalindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_003915_rage_fwanimmanager_get_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004038_rage_fwclipsetrequesthelper_fwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004390_rage_fwanimdirectorcomponentsyncedscene_decsyncedsceneref_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004392_rage_fwanimdirectorcomponentsyncedscene_addsyncedsceneref_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004393_rage_fwanimdirectorcomponentsyncedscene_getsyncedsceneduration_s(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004394_rage_fwanimdirectorcomponentsyncedscene_setsyncedsceneduration_s(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_004395_rage_fwanimdirectorcomponentsyncedscene_calcentitymatrix_short_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: f32); }
extern "C" { pub fn f_004403_rage_fwanimdirectorcomponentsyncedscene_getsyncedsceneorigin_sho(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_004407_rage_fwanimdirectorcomponentsyncedscene_issyncedsceneholdlastfra(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004409_rage_fwanimdirectorcomponentsyncedscene_issyncedsceneabort_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004417_rage_fwanimdirectorcomponentsyncedscene_removecamerafromsyncedsc(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_004603_rage_fwextensionlist_add_rage_fwextension_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004619_rage_fwarchetype_getdrawable_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004620_rage_fwarchetype_getfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004621_rage_fwarchetype_getassetparenttxdindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004632_rage_fwarchetypemanager_lookupmodelid_rage_fwarchetype_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004634_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004681_rage_fwentity_waitforanyactiveanimupdatetocomplete_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004683_rage_fwentity_getaabb_rage_spdaabb_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004685_rage_fwentity_getrootlod(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004767_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004788_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004792_rage_fwpickermanager_getselectedentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004793_rage_fwpickermanager_getentity_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004794_rage_fwpickermanager_getnumberofentities(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_004795_rage_fwpickermanager_getshowhidemode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_004825_rage_fwvectormap_draw3dboundingboxonvmap_rage_spdaabb_const_ref(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004921_rage_dldrawcommandbuffer_registercommand_int_void_ptr_rage_dlcmd(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_004935_rage_dlcmdbase_op_new_unsigned_long_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005220_rage_ropemanager_findrope_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005281_rage_fwentityprerenderlist_sortandremoveduplicates(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005283_rage_fwentityprerenderlist_removeelements_rage_fwentityprerender(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005284_rage_fwrenderlistgroup_clear(ph: *mut Phantom); }
extern "C" { pub fn f_005285_rage_fwrenderlistgroup_registerrenderphase_int_rage_fwrenderphas(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_005287_rage_fwrenderlistgroup_waitforsortentitylistjob(ph: *mut Phantom); }
extern "C" { pub fn f_005293_rage_fwrenderphase_fwrenderphase_rage_grcviewport_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_005304_rage_fwrenderphasemanager_registerrenderphase_rage_fwrenderphase(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005305_rage_fwrenderphasescanner_fwrenderphasescanner(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005318_rage_fwrenderthreadinterface_setrenderfunction_rage_functor0(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005319_rage_fwrenderthreadinterface_setdefaultrenderfunction(ph: *mut Phantom); }
extern "C" { pub fn f_005392_rage_fwmapdatadebug_getblockindex_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005554_rage_fwimapgroupmgr_getswapentities_unsigned_int_rage_atarray_ra(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_005556_rage_fwimapgroupmgr_getisswapimaploaded_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005730_rage_fwinteriorlocation_makeinvalid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005731_rage_fwinteriorlocation_setroomindex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005732_rage_fwinteriorlocation_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005733_rage_fwinteriorlocation_getroomindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005744_rage_fwworldmgr_forallentitiesintersecting_rage_fwsearchvolume_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_005772_rage_fwscreenquad_generatefrustum_rage_mat44v_const_ref_rage_spd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_005799_rage_fwsearch_finalize(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005800_rage_fwsearch_executecallbackonresult_bool_ptr_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_005917_rage_fwrefawarebaseimpl_rage_datbase_dtorfwrefawarebaseimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005922_rage_aitask_findsubtaskoftype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005947_rage_aitasktree_getactiveleaftask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005949_rage_aitasktree_findtaskbytypeactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005950_rage_aitasktree_findtaskbytypewithpriority_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_006064_rage_strstreamingvisualize_send_int_void_const_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_006066_rage_strpackfilemanager_getimagefilefromhandle_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006075_rage_strstreaminginfo_computevirtualsize_rage_strindex_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006076_rage_strstreaminginfo_computephysicalsize_rage_strindex_bool_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006090_rage_strstreamingvisualize_unrequest_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006094_rage_strstreaminginfomanager_removerequest_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006099_rage_strstreamingvisualize_request_rage_strindex_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006104_rage_strstreaminginfomanager_completedummyrequest_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006122_rage_strstreaminginfomanager_getmodule_rage_strindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006156_rage_atarray_rage_strindex_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006162_rage_strstreaminginfo_gethandle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006188_rage_atarray_int_0_unsigned_short_ptr_ref_rage_atmap_int_rage_at(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006194_rage_strstreaminginfomanager_clearflagforall_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_006225_rage_atvarstring_atvarstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_006235_rage_strstreamingvisualize_setmarkertext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006241_rage_strstreamingvisualize_addmarker_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006261_rage_strrequest_getindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006294_rage_strstreamingvisualize_registervisflag_unsigned_int_bool_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_006297_rage_strstreamingvisualize_setstreamingvolume_int_rage_strstream(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: f32, a6: i32); }
extern "C" { pub fn f_006388_rage_tcmanager_setmodifierboxtreerebuildprohibited_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006392_rage_tcmanager_buildmodifiertree(ph: *mut Phantom); }
extern "C" { pub fn f_006408_rage_tcmodifier_setonlymainthreadisupdating_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006725_rage_audsound_setvariablevaluedownhierarchyfromname_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_006881_rage_audmetadatamanager_getobjectmetadatareffromhash_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006999_rage_audenvironmentgroupinterface_removesoundreference(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007036_rage_audmetadatamanager_getobject_verify_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007078_rage_audsoundset_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007079_rage_audsoundset_find_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007081_rage_audsoundsetfieldref_resolvemetadataref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007126_rage_audsimplesmoother_audsimplesmoother(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007411_rage_audentity_addbatchedsoundrequest_rage_audmetadataref_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007412_rage_audentity_createandplaysound_unsigned_int_rage_audsoundinit(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007421_rage_vfxlist_additem_rage_vfxlistitem_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007422_rage_vfxlist_removeitem_rage_vfxlistitem_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007496_rage_decalmanager_isalive_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007497_rage_decalmanager_remove_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007508_rage_decalinstsettings_decalinstsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007509_rage_decalbucketsettings_decalbucketsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007510_rage_decalpipelinesettings_decalpipelinesettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007511_rage_decallodsettings_decallodsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007528_rage_ptfxattach_attach_rage_ptxeffectinst_ptr_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_007554_rage_ptfxhistory_add_rage_atnonfinalhashstring_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_007555_rage_ptfxhistory_check_rage_atnonfinalhashstring_unsigned_int_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32) -> i32; }
extern "C" { pub fn f_007559_rage_ptfxmanager_gettriggeredinst_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007562_rage_ptfxmanager_getregisteredinst_void_const_ptr_int_bool_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_007566_rage_ptfxreg_unregister_void_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007568_rage_ptfxscript_trigger_rage_atnonfinalhashstring_rage_atnonfina(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: f32, a7: i32) -> i32; }
extern "C" { pub fn f_007569_rage_ptfxscript_trigger_rage_atnonfinalhashstring_rage_atnonfina(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_007570_rage_ptfxwrapper_setcolourtint_rage_ptxeffectinst_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007571_rage_ptfxscript_settriggeredcolourtint_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32); }
extern "C" { pub fn f_007581_rage_ptfxscript_doesexist_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_007587_rage_ptfxwrapper_setalphatint_rage_ptxeffectinst_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007658_rage_fwptrlistsinglelink_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007661_rage_fwptrlist_ismemberoflist_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007675_rage_fwangle_limitdegreeangle_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_007676_rage_fwangle_limitradianangleforpitch_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_007677_rage_fwangle_lerp_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_007678_rage_fwangle_lerptowards_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_007696_rage_fwportalcorners_calcboundingbox_rage_spdaabb_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007697_rage_fwportalcorners_calcscreensurface_rage_grcviewport_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_007699_rage_fwportalcorners_haspassedthrough_rage_fwgeom_fwline_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_007701_rage_fwportalcorners_calcsimpledistancetopointsquared_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007703_rage_fwportalcorners_createextendedlineforportaltraversalcheck_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007756_rage_netinterface_flushalllogfiles_bool(ph: *mut Phantom); }
extern "C" { pub fn f_007759_rage_netinterface_getnumphysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007762_rage_netinterface_getallphysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007995_rage_matrix34_toeulersxyz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008686_rage_fwbasepool_getindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008688_rage_fwbasepool_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008770_rage_scripthandlermgr_getscripthandlerforresource_unsigned_short(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_008917_rage_fwtimer_enduserpause(ph: *mut Phantom); }
extern "C" { pub fn f_008918_rage_fwtimer_startuserpause_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008945_rage_fwbasepool_forall_bool_ptr_void_ptr_void_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008950_rage_fwbasepool_getjustindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008952_rage_fwbasepool_getat_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008953_rage_fwknownrefholder_fwknownrefholder_void_ptr_ptr_rage_fwknown(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_009048_rage_crproperty_getattribute_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009224_rage_crskeleton_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009225_rage_crskeleton_partialreset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009226_rage_crskeleton_partialupdate_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009233_rage_crskeleton_getglobalmtx_unsigned_int_rage_mat34v_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009283_rage_grmmodel_getgeometry_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009314_pixend(ph: *mut Phantom); }
extern "C" { pub fn f_009317_rage_grmshader_load_char_const_ptr_rage_fitokenizer_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_009417_pixbegin_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009445_rage_pfdrawarrow_rage_vector3_const_ref_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009447_rage_pfdrawitembase_beginmanager(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009448_rage_pfdrawitembase_end(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009451_rage_pfdrawitem_drawtick_rage_vector3_const_ref_float_rage_color(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_009452_rage_pfdrawitem_drawbox_rage_matrix34_const_ref_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_009509_rage_grcfvf_getoffsetnoassert_rage_grcfvf_grcfvfchannels_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009514_rage_grctexture_setcustomloadname_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009533_rage_grctexturefactory_createparams_createparams_unsigned_short(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009539_rage_grceffect_lookuptechnique_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009540_rage_grceffect_lookupvar_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009542_rage_grcstateblock_createblendstate_rage_grcblendstatedesc_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009545_rage_grcstateblock_setblendstate_rage_grcstateblock_grcblendstat(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009555_rage_grcstateblock_setrasterizerstate_rage_grcstateblock_grcrast(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009560_rage_grcstateblock_setdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009603_rage_grcviewport_setworldidentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009608_rage_grcviewport_grcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009609_rage_grcviewport_resetwindow(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009613_rage_grcviewport_dtorgrcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009615_rage_grceffect_lookupglobalvar_char_const_ptr_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009616_rage_grcviewport_setnormwindow_float_float_float_float_float_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_009617_rage_grcviewport_regenerateperspective(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009618_rage_grcviewport_perspective_float_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_009619_rage_grcviewport_ortho_float_float_float_float_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_009633_rage_grcviewport_getwindow_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009636_rage_grcviewport_setcameramtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009637_rage_grcviewport_setcameraidentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009638_rage_grcviewport_isspherevisible_rage_vec4v_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009640_rage_grcviewport_isspherevisible_float_float_float_float_float_p(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_009645_rage_grcviewport_getunclippedwindow_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009646_rage_grcviewport_transform_rage_vec3v_const_ref_float_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009647_rage_grcviewport_transform_rage_vec3v_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009648_rage_grcviewport_reversetransform_float_float_rage_vec3v_ref_rag(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i64); }
extern "C" { pub fn f_009649_rage_grcworldidentity(ph: *mut Phantom); }
extern "C" { pub fn f_009650_rage_grcbindtexture_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009651_rage_grcviewport_initclass(ph: *mut Phantom); }
extern "C" { pub fn f_009652_rage_grcviewport_shutdownclass(ph: *mut Phantom); }
extern "C" { pub fn f_009653_rage_grcviewport_op_rage_grcviewport_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009673_rage_grcdevice_initadapterordinal(ph: *mut Phantom); }
extern "C" { pub fn f_009675_rage_grcstateblock_createdepthstencilstate_rage_grcdepthstencils(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009692_rage_grcdevice_resetclipplanes(ph: *mut Phantom); }
extern "C" { pub fn f_009765_rage_grcvertexbuffer_create_int_rage_grcfvf_const_ref_bool_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_009785_rage_grcindexbuffer_create_int_bool_void_ptr(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_009789_rage_grcindexbuffer_unlockwo_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009858_rage_grcstateblock_createrasterizerstate_rage_grcrasterizerstate(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009859_rage_grceffect_setglobalfloatcommon_int_float_ptr_int_rage_grcef(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_009863_rage_grceffect_findtechniquegroupid_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009872_rage_grceffect_setglobalvar_rage_grceffectglobalvar_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009904_rage_gridmonitor_getarea_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009905_rage_monitorconfiguration_ismultihead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009908_rage_monitorconfiguration_update(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009911_rage_monitorconfiguration_getcentralmonitor_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009913_rage_grcdevice_getmanufacturer_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_009951_rage_grcvertexbufferd3d11_lockinternal_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009952_rage_grcvertexbufferd3d11_unlockinternal_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009959_rage_grcindexbuffer_lockrw_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009986_rage_grcdebugdraw_adddebugoutputex_bool_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_010039_rage_grcdevice_getwidescreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_010111_rage_grctexturefactorydx11_createrendertarget_char_const_ptr_id3(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_010163_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010166_rage_grcdebugdraw_quad_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010172_rage_grcdebugdraw_text_float_float_rage_color32_char_const_ptr_b(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i64, a4: i32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010179_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010181_rage_grcdebugdraw_poly_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010183_rage_grcdebugdraw_cross_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010188_rage_grcdebugdraw_boxaxisaligned_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010194_rage_grcdebugdraw_axis_rage_mat34v_const_ref_float_bool_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_010195_rage_grcdebugdraw_arrow_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_010200_rage_grcdebugdraw_setcullingviewport_rage_grcviewport_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010214_rage_grcdebugdraw_textfontpush_rage_grcfont_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010215_rage_grcdebugdraw_textfontpop(ph: *mut Phantom); }
extern "C" { pub fn f_010219_rage_grcdebugdraw_adddebugoutputex_bool_rage_color32_char_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_010220_rage_grcdebugdraw_adddebugoutput_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010221_rage_grcdebugdraw_adddebugoutput_rage_color32_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_010224_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010225_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010459_rage_ioaddrounddeadzone_float_ref_float_ref_float_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> f32; }
extern "C" { pub fn f_010493_rage_iovalue_getvalue_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_010500_rage_iovalue_isdown_float_rage_iovalue_readoptions_const_ref_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010501_rage_iovalue_historyhelddown_unsigned_int_float_rage_iovalue_rea(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_010503_rage_iovalue_isup_float_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010505_rage_iovalue_wasdown_float_rage_iovalue_readoptions_const_ref_co(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010510_rage_iovalue_isreleasedafterhistoryhelddown_unsigned_int_float_r(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_010511_rage_iovalue_ispressed_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010512_rage_iovalue_historypressed_unsigned_int_unsigned_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_010514_rage_iovalue_isreleased_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010518_rage_iovalue_getnorm_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010519_rage_iovalue_getunboundnorm_rage_iovalue_readoptions_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010521_rage_iovalue_getnorm01_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010523_rage_iopad_setactuatorvalue_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_010736_rage_spdplane_intersectrayv_rage_vec3v_const_ref_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_010747_rage_spdtransposedplaneset8_set_rage_grcviewport_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_011012_rage_cutfcameracuteventargs_cutfcameracuteventargs(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011072_rage_cutfobjectvariationeventargs_dtorcutfobjectvariationeventar(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011083_rage_atarray_rage_cutfcameracuttimeofdaydofmodifier_0_unsigned_s(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011402_rage_cutsmanager_getsceneorientationmatrix_rage_matrix34_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011738_rage_matrix34_toeulerszyx_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011751_rage_fraginst_getcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011781_rage_fragcacheentry_shatterglassonbone_int_int_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_011782_rage_fraginst_hidedeadgroup_int_rage_phcontactiterator_ptr(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011788_rage_fraginst_poseskeletonfromarticulatedbody(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011885_rage_phiterator_dtorphiterator(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011966_rage_fragtype_getskinned_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012021_rage_fragphysicslodgroup_getlodbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012183_rage_ptxeffectinst_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012196_rage_ptxeffectinst_setbasemtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012198_rage_ptxeffectinst_clearcollisionset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012201_rage_ptxeffectinst_setbasepos_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012202_rage_ptxeffectinst_setoffsetmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012203_rage_ptxeffectinst_setoffsetpos_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012205_rage_ptxeffectinst_setoffsetrot_rage_quatv_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012207_rage_ptxeffectinst_setevovaluefromhash_rage_athashvalue_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_013018_rage_phglass_regeneratebound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013020_rage_bgglassmanager_hitglass_short_int_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_013025_rage_bgglassmanager_recycleglass_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013026_rage_bgglassmanager_freeglass_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013028_rage_bggeometrydata_op_rage_bggeometrydata_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013035_rage_bgbreakable_hitinternal_int_rage_vec2v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_013036_rage_bgbreakable_adjustlod(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013050_rage_matrix34_transform3x3_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013057_rage_bgbreakable_updatenormal_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_013058_rage_bgbreakable_updatereinforced_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_013102_rage_bggeometrybuilder_calculatepointlocations_rage_bggeometrybu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32); }
extern "C" { pub fn f_013103_rage_bggeometrybuilder_createpanepiecesgeometry_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_013104_rage_bggeometrybuilder_addcrackgeometry_rage_atarray_rage_bggeom(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_013105_rage_bggeometrybuilder_createsinglefallingpiecegeometry_rage_bgf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32); }
extern "C" { pub fn f_013108_rage_bgglassmanager_glassinfo_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013110_rage_bgglassmanager_transferbreakableglassowner_short_ptr_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013111_rage_bgglassmanager_destroybreakableglass_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_013115_rage_bgglassmanager_glassinfo_sync(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013119_rage_bgglassmanager_setsilenthit_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_013155_rage_rsttestaabbexactedgelist_bool_rage_mat44v_const_ref_rage_sc(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i32, a13: i32, a14: i32); }
extern "C" { pub fn f_013215_rage_bgpackabledata_rage_phglass_1_pack_rage_sysmemallocator_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013429_char_ptr_rage_safecatf_char_char_ptr_unsigned_long_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_015891_rage_phconstraintmgr_gettemporarypointer_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015893_rage_phmultireaderlocktoken_waitasreader(ph: *mut Phantom); }
extern "C" { pub fn f_016013_rage_phsimulator_addinactiveobject_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016036_rage_pharchetype_setbound_rage_phbound_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016081_rage_phcullshape_draw_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016087_rage_phinst_setarchetype_rage_pharchetype_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016088_rage_phinst_setinstflag_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016132_rage_phshapetest_rage_phshapeprobe_setincludeinstancecallback_ra(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016145_rage_phshapetest_rage_phshapeedge_testinlevel(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_016151_rage_phshapetest_rage_phshapesphere_testinlevel(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_016171_rage_phshapetest_rage_phshapecapsule_testinlevel(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_016176_rage_phshapetest_rage_phshapesweptsphere_testinlevel(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_016180_rage_phshapetest_rage_phshapescalingsweptquad_testinstanceinleve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_016194_rage_phshapetest_rage_phshapesphere_initsphere_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_016195_rage_phshapetest_rage_phshapeprobe_initprobe_rage_phsegment_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_016197_rage_phshapetest_rage_phshapesweptsphere_initsweptsphere_rage_ph(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_016199_rage_phshapetest_rage_phshapecapsule_initcapsule_rage_phsegment(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_016367_rage_phbound_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016737_rage_phcontactmgr_markscratchpadused_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016907_rage_phlevelnodetree_rage_phlooseoctreenode_getfirstculledobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016908_rage_phlevelnodetree_rage_phlooseoctreenode_getnextculledobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016909_rage_phlevelnodetree_rage_phlooseoctreenode_testprobe_rage_phseg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_016943_rage_scrthread_printstacktrace_void_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016949_rage_scrthread_getactivethread(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_016981_rage_scrthread_getthread_rage_scrthreadid(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_017189_rage_crclip_calcblockpassed_float_float_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i32; }
extern "C" { pub fn f_018885_cgamelogic_isteleportactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_019136_cvehiclerecordingmgr_isplaybackgoingonforcar_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019241_cstuntjumpmanager_isastuntjumpinprogress(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_019314_centityscanner_scanforentitiesinrange_centity_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019445_caidebuginfo_colorprintln_rage_color32_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_019446_caidebuginfo_writeheader_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019448_caidebuginfo_writelogevent_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019505_canimclipdebugiterator_getclipdebuginfo_rage_crclip_const_ptr_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64); }
extern "C" { pub fn f_019565_cpedbonetagconvertor_getboneindexfrombonetag_rage_crskeletondata(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_019704_cclipeventtags_carmsikeventtag_getallowed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019737_cclipeventtags_cfirstpersoncameraeventtag_getblendinduration_con(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_019738_cclipeventtags_cfirstpersoncameraeventtag_getblendoutduration_co(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_019770_cdebugclipselector_dtorcdebugclipselector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019784_cdebugclipselector_addwidgets_rage_bkgroup_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_019791_cdebugclipselector_getselectedclipdictionary_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019792_cdebugclipselector_getselectedclipname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019954_canimviewer_ondynamicentityprerender(ph: *mut Phantom); }
extern "C" { pub fn f_020448_canimscenemanager_update_animsceneupdatephase(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021156_audscriptaudioentity_stopstream(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021173_audscriptaudioentity_replayloadscriptbank_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_021187_audweaponaudioentity_processcachedweaponfireevents(ph: *mut Phantom); }
extern "C" { pub fn f_021194_audweaponfireinfo_findweaponsettings_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_021196_audweaponaudioentity_playweaponfireevent_audweaponfireevent_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021202_audnorthaudioengine_isinslowmovideoeditor(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021340_audpedfootstepaudio_addfootstepevent_audfootstepevent(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021346_audpedaudioentity_triggerimpactsounds_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_021352_audspeechaudioentity_playanimalvocalization_unsigned_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_021358_audspeechaudioentity_playpain_unsigned_int_auddamagestats_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_021362_audpedfootstepaudio_addslopedebrisevent_audfootstepevent_rage_ve(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32); }
extern "C" { pub fn f_021397_audpedaudioentity_playvehicleimpact_rage_vector3_const_ref_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_021403_audspeechaudioentity_stopspeech(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021415_bool_creplaymgrinternal_recordfx_cpacketpedclothsound_cpacketped(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021419_audpedfootstepaudio_addbushevent_audfootstepevent_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021423_audpedaudioentity_playunderovervehiclesounds_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_021457_audpedfootstepaudio_addwaterevent_audfootstepevent(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021458_audpedfootstepaudio_addclothevent_audfootstepevent(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021459_audpedfootstepaudio_addpetrolcanevent_audfootstepevent_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021460_audpedfootstepaudio_addmolotovevent_audfootstepevent_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021461_audpedfootstepaudio_addscriptsweetenerevent_audfootstepevent(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021653_audscriptaudioentity_preloadstreaminternal_char_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_021784_audspeechaudioentity_stopambientspeech(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021791_audspeechaudioentity_stoppain(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021821_audscriptaudioentity_addscript(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021824_audspeechaudioentity_stopplayingscriptedspeech(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021853_audscriptaudioentity_onflagstatechange_audscriptaudioflags_flagi(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_021885_audscriptaudioentity_getscript_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_021909_audscriptaudioentity_hasalreadyplayed_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_021910_audscriptaudioentity_updatetimestamp_unsigned_int_unsigned_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_021912_audscriptaudioentity_preloadreplayscriptedspeech_unsigned_int_un(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_021914_audscript_setaudioflag_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_021934_audspeechaudioentity_replaysay_unsigned_int_unsigned_int_int_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i64, a11: i32, a12: i64, a13: i32, a14: i32); }
extern "C" { pub fn f_021937_audspeechaudioentity_replayplayscriptedspeech_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32); }
extern "C" { pub fn f_021970_audspeechmanager_playedpainforreplay_unsigned_int_unsigned_int_u(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_022062_audvehicleaudioentity_requestweaponchargesfx_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_022082_audvehicleaudioentity_playvehiclehorn_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_022095_audvehicleaudioentity_triggertyrepuncture_ehierarchyid_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022100_audvehicleaudioentity_triggersplash_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_022122_audvehiclecollisionaudio_getvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022218_audweaponfireinfo_populatepresuckinfo_rage_weaponsettings_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_022229_audweaponaudioentity_togglethermalvision_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_022238_audweaponaudioentity_playbulletby_unsigned_int_rage_vector3_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_022244_audweaponaudioentity_addplayerequippedweapon_unsigned_int_unsign(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_022381_void_creplayeventmanager_storeevent_cpacketsoundphonering_cpacke(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022614_auddynamicmixer_startscene_rage_mixerscene_const_ptr_audscene_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_022615_audscene_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022687_audcuttracksynchedscene_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022701_audcuttrackcutscene_trigger(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022703_audcutsceneaudioentity_initcutscenetrack_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022705_audcutsceneaudioentity_issyncscenetrackprepared_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022706_audcuttracksynchedscene_init_char_const_ptr_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_022707_audcutsceneaudioentity_iscutscenetrackprepared(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_022709_audcutsceneaudioentity_triggersyncscene_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022710_audcutsceneaudioentity_stopcutscene_bool_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_022715_audcuttracksynchedscene_stop_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022717_audcutsceneaudioentity_getcutscenetime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_022719_audcutsceneaudioentity_getsyncscenetime_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_022726_audcuttracksynchedscene_hassyncscenebeeninitialized_char_const_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_022728_audcuttracksynchedscene_setentity_unsigned_int_centity_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_022729_audcuttracksynchedscene_setposition_unsigned_int_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_022764_auddynamicmixer_getactivescenefromhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_022828_audexplosionaudioentity_reallyplayexplosion_eexplosiontag_rage_v(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_022852_audfrontendaudioentity_playsound_rage_atnonfinalhashstring_char(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023038_bool_creplaymgrinternal_recordfx_cpacketsetunderwaterstreamvaria(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023396_cpacketevent_cpacketevent_ereplaypacketid_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023405_cpacketevent_printxml_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023419_cpacketaddrope_extract_ctrackedeventinfo_unsigned_long_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023421_cpacketdeleterope_extract_ctrackedeventinfo_unsigned_long_ref_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023424_cpacketattachropetoentity_extract_ctrackedeventinfo_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023426_cpacketattachentitiestorope_extract_ctrackedeventinfo_unsigned_l(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023427_cpacketattachobjectstoropearray_extract_ctrackedeventinfo_unsign(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023429_cpacketdetachropefromentity_extract_ctrackedeventinfo_unsigned_l(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023431_cpacketpinrope_extract_ctrackedeventinfo_unsigned_long_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023433_cpacketrappelpinrope_extract_ctrackedeventinfo_unsigned_long_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023435_cpacketunpinrope_extract_ctrackedeventinfo_unsigned_long_ref_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023436_cpacketloadropedata_extract_ctrackedeventinfo_unsigned_long_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023438_cpacketropewinding_extract_ctrackedeventinfo_unsigned_long_ref_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023439_cpacketropeupdateorder_extract_ctrackedeventinfo_unsigned_long_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023440_cpacketaddrope_old_extract_ctrackedeventinfo_unsigned_long_ref_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023441_cpacketdeleterope_old_extract_ctrackedeventinfo_unsigned_long_re(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023442_cpacketdetachropefromentity_old_extract_ctrackedeventinfo_unsign(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023448_cpacketforceroomforentity_extract_ctrackedeventinfo_int_ref_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023477_cpacketsetunderwaterstreamvariable_cpacketsetunderwaterstreamvar(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i64; }
extern "C" { pub fn f_023478_cpacketcollisionplaypacket_extract_ctrackedeventinfo_replaytrack(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023480_cpacketcollisionupdatepacket_extract_ctrackedeventinfo_replaytra(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023481_audvehiclecollisioncontext_dtoraudvehiclecollisioncontext(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023487_cpacketdynamicmixerscene_extract_ctrackedeventinfo_replaytracked(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023488_cpacketscenesetvariable_extract_ctrackedeventinfo_replaytrackeds(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023492_cpacketpedstandingmaterial_cpacketpedstandingmaterial_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_023502_replaysound_extractsoundinitparams_rage_audsoundinitparams_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_023504_cpacketsoundcreatepersistant_extract_ctrackedeventinfo_replaytra(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023505_cpacketsoundupdatepersistant_extract_ctrackedeventinfo_replaytra(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023506_cpacketsoundphonering_extract_ctrackedeventinfo_replaytrackedsou(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023507_cpacketsoundstopphonering_extract_ctrackedeventinfo_replaytracke(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023508_cpacketsoundupdatelpfcutoff_extract_ctrackedeventinfo_replaytrac(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023509_cpacketsoundupdatepitch_extract_ctrackedeventinfo_replaytrackeds(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023510_cpacketsoundupdateposition_extract_ctrackedeventinfo_replaytrack(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023511_cpacketsoundupdatepostsubmix_extract_ctrackedeventinfo_replaytra(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023512_cpacketsoundupdatevolume_extract_ctrackedeventinfo_replaytracked(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023513_cpacketsoundupdatepitchpos_extract_ctrackedeventinfo_replaytrack(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023514_cpacketsoundupdatepitchvol_extract_ctrackedeventinfo_replaytrack(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023515_cpacketsoundupdateposvol_extract_ctrackedeventinfo_replaytracked(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023516_cpacketsoundupdatepitchposvol_extract_ctrackedeventinfo_replaytr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023517_cpacketsoundupdatedoppler_extract_ctrackedeventinfo_replaytracke(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023518_cpacketsoundsetvaluedh_extract_ctrackedeventinfo_replaytrackedso(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023519_cpacketsoundsetclientvar_extract_ctrackedeventinfo_replaytracked(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023520_cpacketsoundstop_extract_ctrackedeventinfo_replaytrackedsound_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023521_cpacketsoundstart_extract_ctrackedeventinfo_replaytrackedsound_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023522_cpacketplaystreamfromentity_extract_ctrackedeventinfo_replaytrac(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023523_cpacketplaystreamfromposition_extract_ctrackedeventinfo_replaytr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023524_cpacketplaystreamfrontend_extract_ctrackedeventinfo_replaytracke(ph: *mut Phantom); }
extern "C" { pub fn f_023528_cpacketscriptsetvaronsound_extract_ctrackedeventinfo_replaytrack(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023531_audspeechmanager_setreplaypainbanks_unsigned_char_unsigned_char(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_023537_cpacketweaponsoundfireevent_preplaycore_ceventinfo_centity_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023538_cpacketweaponsoundfireevent_getvehicleweapon_cvehicle_ptr_int_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_023539_audweaponfireevent_dtoraudweaponfireevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023540_cpacketweaponsoundfireeventold_preplay_ceventinfo_centity_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_023541_cpacketweaponsoundfireeventold_getvehicleweapon_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_023542_cpacketweaponpersistant_extract_ctrackedeventinfo_replaytrackeds(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023543_cpacketweaponpersistantstop_extract_ctrackedeventinfo_replaytrac(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023545_cpacketweaponspinup_extract_ctrackedeventinfo_replaytrackedsound(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023552_dumpreplaypackets_char_ptr_unsigned_int_39675(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023565_quantizequaternions3_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023567_quantizequaternions3_20_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023568_dequantizequaternions3_20_rage_quaternion_ref_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023572_cpacketpositionandquaternion_loadmatrix_rage_matrix34_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023573_cfire_cfire_cfire_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023574_cfire_dtorcfire(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023575_cpacketfirefx_extract_ctrackedeventinfo_ptxfireref_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023576_cpacketstopfirefx_extract_ctrackedeventinfo_ptxfireref_ref_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023580_cpacketstartscriptptfx_extract_ctrackedeventinfo_unsigned_long_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023581_cpacketstartscriptptfx_getreplayid_unsigned_char_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023582_cpacketstartnetworkscriptptfx_extract_ctrackedeventinfo_unsigned(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023583_cpacketstartnetworkscriptptfx_shouldextract_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_023584_cpacketforcevehinteriorscriptptfx_extract_ctrackedeventinfo_unsi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023586_cpacketstopscriptptfx_extract_ctrackedeventinfo_unsigned_long_re(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023587_cpacketdestroyscriptptfx_extract_ctrackedeventinfo_unsigned_long(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023588_cpacketupdateoffsetscriptptfx_extract_ctrackedeventinfo_unsigned(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023589_cpacketevolveptfx_extract_ctrackedeventinfo_unsigned_long_ref_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023590_cpacketupdateptfxfarclipdist_extract_ctrackedeventinfo_unsigned(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023594_cpacketupdatescriptptfxcolour_extract_ctrackedeventinfo_unsigned(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023596_cpacketweaponbullettracefx_cpacketweaponbullettracefx_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32) -> i64; }
extern "C" { pub fn f_023598_cpacketaddscriptdecal_extract_ctrackedeventinfo_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023599_cpacketremovescriptdecal_extract_ctrackedeventinfo_int_ref_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023601_cpacketptexmtlbang_extract_ceventinfo_centity_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023603_cpacketptexmtlscrape_extract_ceventinfo_centity_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023634_cpacketsetbuildingtint_extract_ceventinfo_void_ref_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023812_cpacketextension_beginwrite_cpacketbase_ptr_unsigned_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_023813_cpacketextension_endwrite_cpacketbase_ptr_unsigned_char_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023891_replaybuffermarkermgr_addmarker_unsigned_int_unsigned_int_marker(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_023906_creplaycontrol_isplayeroutofcontrol(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023916_replaycontroller_enablemodify(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023917_replaycontroller_disablemodify(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023930_creplayeventmanager_shouldproceedwithpacket_cpacketevent_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023934_centity_ptr_creplayeventmanager_findentity_centity_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_023935_void_creplayinterfacemanager_getentity_centity_centityget_centit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023936_replayentityextension_getextindex_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024018_creplayinterface_cobject_getentity_centityget_cobject_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024064_creplayinterface_cvehicle_getentity_centityget_cvehicle_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024257_creplaymgrinternal_isusingrecordedcamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024266_creplaymgrinternal_isawareofworldforrecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024286_creplaymgrinternal_isplaybackflagset_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_024297_rage_atmapentry_int_ptfx_replaystreamingrequest_ref_rage_atmap_i(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024311_replayinteriormanager_getproxyatcoords_rage_vector3_const_ref_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_024349_replayrayfiremanager_submitpacket_packetrayfireinfo_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024376_cpacketvehdamageupdate_extract_ceventinfo_cvehicle_const_ref_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024602_rage_atarray_creplayrendertargethelper_rtinfo_0_unsigned_short_d(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_024637_cped_ptr_creplayeventmanager_findentity_cped_int_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_024638_eshouldextract_creplayeventmanager_getentitiesforpacket_cpacketp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_024639_void_ptr_ref_rage_atmap_unsigned_int_void_ptr_rage_atmaphashfn_u(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_024657_eshouldextract_creplayeventmanager_getentitiesforpacket_cpacketp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_024672_cvehicle_ptr_creplayeventmanager_findentity_cvehicle_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_024673_eshouldextract_creplayeventmanager_getentitiesforpacket_cpacketv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_024727_eshouldextract_creplayeventmanager_getentitiesforpacket_cpacketw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_024741_eshouldextract_creplayeventmanager_shouldextract_cpacketbloodfx(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_024742_eshouldextract_creplayeventmanager_getentitiesforpacket_cpacketb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_024752_eshouldextract_creplayeventmanager_shouldextract_cpacketblooddec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_024753_eshouldextract_creplayeventmanager_getentitiesforpacket_cpacketb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_024789_cobject_ptr_creplayeventmanager_findentity_cobject_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_024790_eshouldextract_creplayeventmanager_getentitiesforpacket_cpacketb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_024957_eshouldextract_creplayeventmanager_getentitiesforpacket_cpacketp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_025079_eshouldextract_creplayeventmanager_shouldextract_cpacketspeechpa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_025080_eshouldextract_creplayeventmanager_getentitiesforpacket_cpackets(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_025199_eshouldextract_creplayeventmanager_getentitiesforpacket_cpacketf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_025200_cpacketfirefx_printxml_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_025264_eshouldextract_creplayeventmanager_getentitiesforpacket_cpackett(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_025307_eshouldextract_creplayeventmanager_getentitiesforpacket_cpackets(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_025308_void_creplayeventmanager_freetrackableduringrecording_unsigned_l(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_025426_eshouldextract_creplayeventmanager_getentitiesforpacket_cpacketd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_025445_eshouldextract_creplayeventmanager_shouldextract_cpacketattachen(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_025446_eshouldextract_creplayeventmanager_getentitiesforpacket_cpacketa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_025447_cpacketentityinfostaticaware_unsigned_char_2_centitychecker_cent(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_025564_eshouldextract_creplayeventmanager_getentitiesforpacket_cpacketw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_026167_caminterface_isdominantrenderedcamera_cambasecamera_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026169_cammanager_isrenderingfirstpersoncamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026174_camgameplaydirector_getfirstpersonshootercamera_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026179_camcinematicdirector_isrenderingcinematicmountedcamera_bool_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026180_camcinematicdirector_isrenderingcinematicidlecamera_bool_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026181_caminterface_findfollowped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026185_camgameplaydirector_computeshakeamplitudeforexplosion_float_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64) -> f32; }
extern "C" { pub fn f_026186_camfactory_createobject_cambaseobjectmetadata_const_ref_rage_atn(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026191_cviewportfader_set_int_bool_rage_color32_const_ptr_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_026194_caminterface_isfadedout(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026199_caminterface_isfadedin(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026204_caminterface_computevehicleentryexitpointpositionandorientation(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_026205_cqueriableinterface_istaskcurrentlyrunning_int_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_026208_caminterface_computehasclearlineofsighttopedbone_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026209_rage_vector3_normalizesafe_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026211_worldprobe_cshapetestfixedresults_unsigned_char_32_cshapetestfix(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026212_caminterface_isrenderedcamerainsidevehicle(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026215_caminterface_computepovturretcamerahash_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026216_camgameplaydirector_isturretseat_cvehicle_const_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026217_camgameplaydirector_computeseatindexpedisin_cvehicle_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_026219_cdynamicentity_getboneindexfrombonetag_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026227_rage_atarray_trenderedcameraobjectsettings_0_unsigned_short_dest(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026228_camshakedebugger_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_026229_caminterface_istripleheaddisplay(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026230_camcinematicdirector_isspectatingcopsandcrooksescape(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026232_cambasedirector_basereset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026233_camcinematicdirector_dtorcamcinematicdirector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026234_cambasedirector_dtorcambasedirector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026235_cambaseframeshaker_removeref_rage_fwregdref_cambaseframeshaker_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026238_cambasecamera_shake_unsigned_int_float_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_026239_cambasecamera_shake_cambaseshakemetadata_const_ref_float_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_026247_camcinematicdirector_getsideofactionline_centity_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026248_rage_vector3_normalizesafe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026249_camcinematicdirector_iscamerapositionvalidwithrespecttotheline_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_026251_cambasecamera_baseupdate_camframe_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026252_camgameplaydirector_getactivecontrol_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026253_camenvelope_start_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_026254_camenvelope_update_float_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> f32; }
extern "C" { pub fn f_026256_cambaseframeshaker_update_camframe_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_026257_camcinematicdirector_dosecondupdateforturretcamera_cambasecamera(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_026258_cambasedirector_postupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026259_rage_fwentity_getskeleton_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026264_camenvelope_calculateattackresponse_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_026265_camenvelope_calculatereleaseresponse_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_026266_camgameplaydirector_isaiming_centity_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026273_cambasecamera_cambasecamera_cambaseobjectmetadata_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026274_camanimatedcamera_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026277_camanimatedcamera_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026278_camanimatedcamera_updateframe_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_026279_rage_matrix34_toeulersyxz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026282_camcinematicanimatedcamera_addentitytoexceptionlist_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026283_camcinematicvehicleorbitcamera_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026284_camgameplaydirector_isusingvehicleturret_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026285_camthirdpersoncamera_computedesiredorbitdistancelimits_float_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_026286_camgameplaydirector_getthirdpersonaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026287_camcatchuphelper_computeorbitrelativepivotoffset_camthirdpersonc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026291_camcinematicvehicleorbitcamera_computecollision_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_026292_camcollision_computeisoveroceanorriver_rage_vector3_const_ref_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i64, a5: f32, a6: i32) -> i32; }
extern "C" { pub fn f_026293_rage_fwmatrixtransform_getpitch_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026294_camcinematicvehicleloworbitcamera_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026297_camcinematicvehicleloworbitcamera_computecollision_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_026298_camcinematicvehicleloworbitcamera_updatespeedrelativeshake_cphys(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_026299_camframe_computeworldmatrixfromfront_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026300_rage_matrix34_transform_rage_vector3_const_ref_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026301_camdampedspring_update_float_float_float_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_026302_camcinematicmountedcamera_createdampedlookathelpers_rage_atnonfi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026303_camcinematicmountedcamera_dtorcamcinematicmountedcamera(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026304_cambasecamera_dtorcambasecamera(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026308_camcinematicmountedcamera_computeispeddrivebyaiming_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026310_camcontrolhelper_update_centity_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026311_camcinematicmountedcamera_computeattachmatrix_cphysical_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026312_camcinematicmountedcamera_shouldallowrearseatlookback(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026313_camcinematicmountedcamera_getfirstpersondrivebydata_cvehicle_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026314_camgameplaydirector_getfollowcamera_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026315_camcinematicmountedcamera_restorecameraorientation_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_026316_camcinematicmountedcamera_isclipping_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_026317_camlookatdampinghelper_update_rage_matrix34_ref_float_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_026318_cambasecamera_stopshaking_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026319_camcinematicmountedcamera_updatespeedrelativeshake_cphysical_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_026320_camcontrolhelper_initviewmodecontext_centity_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026321_camcontrolhelper_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026322_camcinematicmountedcamera_isfollowpedsittingintheleftofthevehicl(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026323_camcinematicvehicletrackingcamera_computeworldspacepositionrelat(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_026324_camgameplaydirector_getturretforped_cvehicle_const_ptr_cped_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026326_camcinematicmountedcamera_computeideallookatposition_rage_matrix(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_026327_rage_matrix34_dot3x3transpose_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026328_rage_matrix34_dot3x3fromleft_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026329_worldprobe_cshapetestresults_dtorcshapetestresults(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026330_camspringmount_update_rage_matrix34_ref_cphysical_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026331_camcinematicmountedcamera_blendmatrix_float_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32); }
extern "C" { pub fn f_026333_camlookaheadhelper_updateconsideringlookattarget_cphysical_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_026335_cambasecamera_updatedof(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026338_camgameplaydirector_findvehiclecustomsettings_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_026348_camcinematiccammancamera_extendcamerapositionifclosetovehiclefro(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64); }
extern "C" { pub fn f_026350_camcinematiccammancamera_randomiseinitialheading_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_026352_camcollision_computeislineofsightclear_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_026353_caminconsistentbehaviourzoomhelper_calculateistargetbehavinginco(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_026357_camcinematicfirstpersonidlecamera_dtorcamcinematicfirstpersonidl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026362_camcinematicfirstpersonidlecamera_updatetargetposition_cped_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026363_camcinematicfirstpersonidlecamera_cleartarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026364_camcinematicfirstpersonidlecamera_testlineofsight_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_026365_camcinematicfirstpersonidlecamera_testeventtarget_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_026366_camcinematicfirstpersonidlecamera_addtargettohistory_centity_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026367_rage_imp_transform_imp34_rage_mat34v_ref_float_vector_4_float_ve(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: u128, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_026368_cpedintelligence_geteventoftype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_026372_camcinematicgroupcamera_setentitylist_rage_atvector_rage_fwregdr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026373_rage_atvector_rage_fwregdref_centity_const_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026374_camcinematicidlecamera_cuttonewshot_camframe_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026375_camcinematicidlecamera_computeocclusion_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_026380_camcinematichelichasecamera_settrackingaccuracymode_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026382_camcinematichelichasecamera_computefov_rage_vector3_const_ref_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> f32; }
extern "C" { pub fn f_026384_camframe_debugrender_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026389_camcinematicpositioncamera_computecampositionrelativetobaseorbit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026400_camcinematictwoshotcamera_computeiscameraclipping_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_026402_camcinematictwoshotcamera_setattachentityposition_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026403_camcinematicvehicletrackingcamera_computestartandendpositions_ca(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026404_camcinematicvehicletrackingcamera_computecollision_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_026407_camcinematicwatercrashcamera_computecollision_rage_vector3_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_026411_cambasecinematiccontext_createrandomindexarray_unsigned_int_rage(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_026412_cambasecinematiccontext_selectshot_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026413_cambasecinematiccontext_tryandcreateanewcamera_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026415_cambaseobject_getnamehash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026421_camcinematicbustedcontext_computeispedarrestingped_cped_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026429_camcinematicintraincontext_gettrainplayerisin_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026436_camgameplaydirector_computeshouldusefirstpersondriveby_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026440_camcinematicinvehiclewantedcontext_isplayerwanted_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026472_cambasecinematicshot_createandtestcamera_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026473_cambasecinematicshot_deletecamera_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026478_cambasecinematicshot_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026479_cambasecinematicshot_isvalidforvehicletype_cvehicle_const_ptr_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026480_cambasecinematicshot_isvalidforcustomvehiclesettings_cvehicle_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026481_cambasecinematicshot_clearshot_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026499_camanimatedcamera_setcameraclip_rage_vector3_const_ref_rage_atno(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32) -> i32; }
extern "C" { pub fn f_026520_camcinematicvehiclebonnetshot_getseatspecificcameraoffset_rage_v(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026522_camframeinterpolator_camframeinterpolator_camframe_const_ref_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_026524_camcinematicvehiclebonnetshot_computeshouldterminateforlookbehin(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026527_camcinematicmountedcamerametadata_const_ptr_camfactory_findobjec(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_026540_camcinematicvehiclegroupshot_getplayerpositions(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026541_rage_atvector_rage_fwregdref_centity_const_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026579_camanimscenedirector_stopanimatingcamera(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026586_cambasecamera_applycustomfocusbehaviourstodofsettings_camdepthof(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026589_cambasecamera_blenddofsettings_float_camdepthoffieldsettingsmeta(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_026591_camframeinterpolator_camframeinterpolator_cambasecamera_ref_camb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_026592_cambaseshakemetadata_const_ptr_camfactory_findobjectmetadata_cam(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026593_camanimatedshakemetadata_const_ptr_camfactory_findobjectmetadata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026594_camanimatedframeshaker_setanimation_char_const_ptr_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_026595_cambasecamera_clonefrom_cambasecamera_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026596_cambasecamera_getobjectspacecameramatrix_centity_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_026597_camframe_debugrenderfrustum_rage_color32_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026598_camframe_debugrenderplane_rage_color32_float_float_const(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32); }
extern "C" { pub fn f_026600_cambasecamera_debugappenddetailedtext_char_ptr_unsigned_int_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_026602_cambasedirector_baseupdate_camframe_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026603_cambasedirector_updateactiverenderstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026604_camframeinterpolator_update_camframe_const_ref_unsigned_int_camb(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_026606_camframeinterpolator_finish(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026607_cambasedirector_shake_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_026608_cambasedirector_shake_char_const_ptr_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_026619_camanimatedcamera_dtorcamanimatedcamera_46011(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026620_camscriptdirector_iscamerascriptcontrollable_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026622_camanimatedcamera_setsyncedscene_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026623_rage_matrix34_toeulers_rage_vector3_ref_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026626_rage_atvector_camcutscenedirector_scutscenecams_0_unsigned_short(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026636_camcutscenedirector_iscutsceneplaying(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026644_camdebugdirector_dtorcamdebugdirector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026647_camfreecamera_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026653_camfreecamera_dtorcamfreecamera(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026656_camfreecamera_teleporttarget_rage_vector3_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_026657_camnearclipscanner_computemaxnearclipatposition_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32) -> f32; }
extern "C" { pub fn f_026667_camshakedebugger_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026673_camgameplaydirector_registervehiclepartbroken_cvehicle_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_026674_camgameplaydirector_resettowedvehiclestatus(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026675_camgameplaydirector_stopexplosionshakes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026676_camgameplaydirector_dtorcamgameplaydirector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026677_rage_atarray_camgameplaydirector_texplosionsettings_0_unsigned_s(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026681_camgameplaydirector_computeweaponinfoforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026682_camgameplaydirector_computeisfirstpersonmodeallowed_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_026683_camgameplaydirector_shoulduse1stpersoncontrols_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026684_camgameplaydirector_createactivecamera_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026685_camcollision_addentitytolist_centity_const_ref_rage_atarray_cent(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_026686_camthirdpersoncamera_clonebaseorientation_cambasecamera_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_026690_camgameplaydirector_islookingbehind_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026691_cpedintelligence_findtasksecondarybytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_026692_camgameplaydirector_computefollowpedcamerasettings_cped_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026693_camgameplaydirector_cachepreviouscameraorientation_camframe_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026694_camgameplaydirector_updateblendouttablegamescamera_tgameplaycame(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_026695_camgameplaydirector_updateblendintablegamescamera_tgameplaycamer(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_026696_camgameplaydirector_computefollowvehiclecamerasettings_cvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026697_camgameplaydirector_computeisvehicleaiming_cvehicle_const_ref_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_026698_camgameplaydirector_computevehicleaimcamerasettings_cvehicle_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_026699_camframe_slerporientation_float_rage_matrix34_const_ref_rage_mat(ph: *mut Phantom, a0: f32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_026702_ccontrol_debugoutputscriptcallstack(ph: *mut Phantom); }
extern "C" { pub fn f_026705_cped_getplayerresetflag_cplayerresetflags_playerresetflags_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026707_camgameplaydirector_starthinting_centity_const_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_026714_camgameplaydirector_getfirstpersonpedaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026719_cweaponinfo_const_ptr_cweaponinfomanager_getinfo_cweaponinfo_rag(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026725_cambaseswitchhelper_computeblendlevel_float_int_bool_const(ph: *mut Phantom, a0: f32, a1: i32, a2: i32) -> f32; }
extern "C" { pub fn f_026726_cammanager_triggerfirstpersonflasheffect_cammanager_efirstperson(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_026740_camthirdpersoncamera_camthirdpersoncamera_cambaseobjectmetadata(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026741_camthirdpersoncamera_dtorcamthirdpersoncamera(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026743_camthirdpersoncamera_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026744_camthirdpersoncamera_computeorbitrelativepivotoffset_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026745_camhinthelper_computehintorientation_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64, a6: f32, a7: f32, a8: i64); }
extern "C" { pub fn f_026746_camthirdpersoncamera_limitorbitheading_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026747_camthirdpersoncamera_computeattachparentattachentity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026748_camcatchuphelper_computedesiredorbitorientation_camthirdpersonca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_026749_camcatchuphelper_updatelookatorientationdelta_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_026750_camthirdpersoncamera_updatecontrolhelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026752_camthirdpersoncamera_updateattachparentmatrix(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026753_cphysical_isasleep_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026754_camthirdpersoncamera_updatebaseattachposition(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026756_camthirdpersoncamera_updatebasepivotposition(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026758_camthirdpersoncamera_computeattachparentrollforbasepivotposition(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026760_camthirdpersoncamera_updateorbitdistance(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026761_camthirdpersoncamera_updateorbitdistancelimits(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026764_camthirdpersoncamera_computebasefov_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026766_camthirdpersoncamera_computeorbitorientation_float_ref_float_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026768_camthirdpersoncamera_applyoverriddenorbitorientation_float_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026772_camthirdpersoncamera_updatepivotposition_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026775_camthirdpersoncamera_computeorbitrelativepivotoffsetinternal_rag(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026776_camthirdpersoncamera_updatecollision_rage_vector3_ref_float_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_026777_camcollision_update_rage_vector3_const_ref_float_bool_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_026778_rage_atarray_camthirdpersoncamera_tnearbyvehiclesettings_0_unsig(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026780_camthirdpersoncamera_computemaxsafecollisionradiusforattachparen(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026781_camthirdpersoncamera_updatecollisionorigin_float_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026783_camthirdpersoncamera_updatecollisionrootposition_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026784_camthirdpersoncamera_computecollisionfallbackposition_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_026788_camframe_slerporientation_float_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: f32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_026789_camthirdpersoncamera_computebaselookatfront_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: f32, a8: i32, a9: i64); }
extern "C" { pub fn f_026790_camthirdpersoncamera_computedesiredroll_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_026791_camthirdpersoncamera_updatelensparameters(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026793_camthirdpersoncamera_computemotionblurstrength(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026797_camframe_interpolatedofsettings_float_camframe_const_ref_camfram(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_026798_camthirdpersoncamera_computedofsettings_float_float_camdepthoffi(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64); }
extern "C" { pub fn f_026799_camthirdpersoncamera_debugrender(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026801_camaimcamera_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026806_camaimcamera_applyorientationlimits_rage_matrix34_const_ref_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_026808_camaimcamera_clonebaseorientation_cambasecamera_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026810_camfirstpersonaimcamera_tryandcreateordestroystickyaimhelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026813_camfirstpersonaimcamera_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026814_camstickyaimhelper_update_rage_vector3_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_026815_camstickyaimhelper_postcameraupdate_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026816_camstickyaimhelper_resetstickyaimparams_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_026817_camstickyaimhelper_transformpositionrelativetouprighttarget_cent(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026819_camfirstpersonaimcamera_updateorientation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026820_camstickyaimhelper_updateorientation_float_ref_float_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_026824_camfirstpersonpedaimcamera_dtorcamfirstpersonpedaimcamera(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026829_ccoverpoint_dtorccoverpoint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026834_camfirstpersonshootercamera_dtorcamfirstpersonshootercamera(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026837_rage_imp_untransformfull_imp34_float_vector_4_float_vector_4_flo(ph: *mut Phantom, a0: u128, a1: u128, a2: u128, a3: i64, a4: i64) -> u128; }
extern "C" { pub fn f_026841_camfirstpersonshootercamera_computeuseheadspineboneorientation_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_026843_camfirstpersonshootercamera_computeinitialvehiclecameraroll_rage(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_026844_camfirstpersonshootercamera_computeinitialvehiclecameraposition(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_026846_camfirstpersonshootercamera_animatedlookatmeleetarget_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_026847_camfirstpersonshootercamera_calculaterelativeheadingpitch_camfir(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: f32, a5: i64); }
extern "C" { pub fn f_026848_camfirstpersonshootercamera_isaimingdirectly_cped_const_ref_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32, a7: f32) -> i32; }
extern "C" { pub fn f_026849_camdampedspring_updateangular_float_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_026853_camfirstpersonshootercamera_getboneheadingandpitch_cped_const_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: f32) -> i32; }
extern "C" { pub fn f_026854_camfirstpersonshootercamera_handleplayergettingcarjacked_cped_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_026855_camfirstpersonshootercamera_pushcameraforwardforheadandarms_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32); }
extern "C" { pub fn f_026856_camfirstpersonshootercamera_computeshoulduselockonaiming_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026863_camthirdpersonaimcamera_camthirdpersonaimcamera_cambaseobjectmet(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026865_camthirdpersonaimcamera_dtorcamthirdpersonaimcamera_46464(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026867_camthirdpersonaimcamera_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026869_camthirdpersonaimcamera_updatelockon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026870_camthirdpersonaimcamera_computelockontarget_centity_const_ptr_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_026872_camthirdpersonaimcamera_updatelockonenvelopes_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026874_camthirdpersonaimcamera_computeorbitorientation_float_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026878_camthirdpersonaimcamera_computedesiredfov(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026880_camthirdpersonaimcamera_updatereticulesettings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026882_camthirdpersonaimcamera_computedofoverriddenfocussettings_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026883_camthirdpersonaimcamera_computedofsettings_float_float_camdeptho(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64); }
extern "C" { pub fn f_026888_camthirdpersonpedaimcamera_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026897_camthirdpersonpedaimcamera_computeorbitrelativepivotoffsetintern(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026899_camthirdpersonpedaimcamera_updatelockon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026907_camthirdpersonpedaimcamera_updatedof(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026908_camthirdpersonpedaimcamera_computedofsettings_float_float_camdep(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64); }
extern "C" { pub fn f_026927_camthirdpersonpedassistedaimcamera_dtorcamthirdpersonpedassisted(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026931_camthirdpersonpedassistedaimcamera_updatemovementshake_camthirdp(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_026932_camthirdpersonpedassistedaimcamera_computetargetpositionforcinem(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026938_camthirdpersonpedassistedaimcamera_computedesiredlockonorientati(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_026960_camfollowcamera_updateattachparentinairblendlevel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026961_camfollowcamera_updatewaterbobshake(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026964_camcontrolhelper_getaimsensitivityscaling_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026967_camfollowcamera_applypullaroundorientationoffsets_float_ref_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026970_camfollowcamera_computeattachparentvelocitytoconsiderforpullarou(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026974_camfollowcamera_computedesiredfov(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026977_camfollowobjectcamera_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026982_camfollowpedcamera_dtorcamfollowpedcamera(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026985_camfollowpedcamera_computeshouldapplyvaultbehavour_bool_ref_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026989_camfollowpedcamera_computeattachpedrootbonematrix_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_027008_ctrain_getlinkedtobackward_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027009_ctrain_getlinkedtoforward_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027036_rage_matrix34_isclose_rage_matrix34_const_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_027040_camanimatedframeshaker_dtorcamanimatedframeshaker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027048_isanimloaded_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_027056_camcollision_testsweptsphereagainstworld_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_027057_camcollision_testcapsuleagainstworld_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_027061_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_027063_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_027067_camcontrolhelper_validateviewmode_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027068_camcontrolhelper_computeisviewmodevalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_027069_camcontrolhelper_updatelookaroundinputenvelope_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_027071_camframeinterpolator_cachesourcecamerarelativetodestinationinter(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_027073_camframeinterpolator_dtorcamframeinterpolator_46750(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027077_camframeinterpolator_applycurve_camframeinterpolator_ecurvetypes(ph: *mut Phantom, a0: i32, a1: f32) -> f32; }
extern "C" { pub fn f_027080_cviewport_propagatecameraframe_camframe_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027081_camoscillatingframeshaker_dtorcamoscillatingframeshaker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027086_camoscillatingframeshaker_appyshaketoframecomponent_float_camfra(ph: *mut Phantom, a0: f32, a1: i64, a2: i32); }
extern "C" { pub fn f_027092_camhandshaker_update_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_027093_rage_matrix34_fromeulers_rage_vector3_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_027098_camsplinenode_computelookatmatrix_rage_matrix34_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027099_camsplinenode_camsplinenode_camframe_const_ref_camsplinenode_efl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_027100_camstickyaimhelper_dtorcamstickyaimhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027102_camthirdpersonframeinterpolator_cachesourcecamerarelativetodesti(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_027109_cammanager_abortpendingfirstpersonflasheffect_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027124_cammarketingfreecamera_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027125_cammarketingatobcamera_dtorcammarketingatobcamera(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027128_cammarketingfreecamera_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_027130_cammarketingfreecamera_updatemisccontrol_cpad_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027131_camsplinenode_dtorcamsplinenode(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027133_cammarketingfreecamera_addwidgetsforinstance(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027138_cammarketingdirector_dtorcammarketingdirector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027143_cammarketingfreecamera_applytranslation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027144_cammarketingfreecamera_applyrotation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027146_cammarketingfreecamera_findnearestentitytoposition_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_027172_camreplaybasecamera_init_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_027175_camreplaybasecamera_loadmarkercamerainfo_sreplaymarkerinfo_ref_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_027176_camreplaybasecamera_validateentities_sreplaymarkerinfo_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027180_camreplaydirector_dtorcamreplaydirector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027183_camreplaydirector_getcurrentmarkerindex(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_027184_camreplaydirector_createactivecamera_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_027185_camreplaydirector_getnextmarkerindex_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_027187_camreplayfreecamera_getcurrentphaseforblend_float_bool_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> f32; }
extern "C" { pub fn f_027196_camreplayfreecamera_settosafeposition_rage_vector3_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_027197_camreplayfreecamera_computecollisionrootposition_centity_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_027201_camreplayfreecamera_loadentities_sreplaymarkerinfo_const_ref_cam(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_027202_camreplayfreecamera_updateposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027203_camreplayfreecamera_updaterotation_rage_vector3_const_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_027204_camreplayfreecamera_rendercameraconstraintmarker_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_027205_camreplayfreecamera_computelookatframewithoffset_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i64); }
extern "C" { pub fn f_027212_camreplaypresetcamera_updatetrackingentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027213_camreplaypresetcamera_updatetrackingentitymatrix(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027214_camreplaypresetcamera_getboundradius_centity_const_ptr_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_027216_camreplaypresetcamera_computelookatorientation_rage_matrix34_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_027218_camreplaypresetcamera_updaterotation_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_027219_camreplaypresetcamera_updateposition_float_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_027224_camscriptedcamera_dtorcamscriptedcamera(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027225_cambasesplinecamera_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_027226_cambasesplinecamera_updatesplines(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027227_cambasesplinecamera_computeposition_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_027228_camscriptedcamera_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_027229_cambasesplinecamera_lerplensparameters_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_027230_cambasesplinecamera_computenodeindexandnodephase_float_unsigned(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_027231_camscriptedcamera_clearlookattarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027233_cambasesplinecamera_computeorientation_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_027234_cambasesplinecamera_setphase_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_027235_cambasesplinecamera_getphase_int_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_027245_camroundedsplinecamera_buildtranslationspline(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027246_camsmoothedsplinecamera_smoothtranslationvelocities(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027249_camcustomtimedsplinecamera_updateoverridenmotionblur(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027251_rage_matrix34_rotateunitaxis_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_027257_camscriptdirector_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027258_camscriptdirector_dtorcamscriptdirector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027261_camscriptdirector_cleanupcamerareferences(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027264_camscriptdirector_stoprendering_bool_unsigned_int_bool_gtathread(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: f32, a6: i32); }
extern "C" { pub fn f_027266_camscriptdirector_createcamera_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_027268_camscriptdirector_deactivatecamera_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_027279_camscriptdirector_debugdumpcameraframeparamstoscript_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_027290_camscriptedcamera_lookat_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027291_camscriptedcamera_lookat_cambasecamera_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027309_camswitchcamera_debugoverridecameraheading_float_ref_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_027310_camswitchcamera_computebaseframe_camframe_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_027314_camswitchdirector_dtorcamswitchdirector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027328_cambaseshakemetadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027344_camcontrolhelpermetadataviewmodes_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027347_camcontrolhelpermetadatalookaround_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027392_caminconsistentbehaviourzoomhelperdetectsuddenmovementsettings_p(ph: *mut Phantom); }
extern "C" { pub fn f_027395_caminconsistentbehaviourzoomhelperdetectfastcameraturnsettings_p(ph: *mut Phantom); }
extern "C" { pub fn f_027398_caminconsistentbehaviourzoomhelperairbornesettings_parser_regist(ph: *mut Phantom); }
extern "C" { pub fn f_027401_caminconsistentbehaviourzoomhelperlossettings_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027419_cambaseswitchhelpermetadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027456_camvehiclecustomsettingsmetadataseatspecficcameras_parser_regist(ph: *mut Phantom); }
extern "C" { pub fn f_027468_caminterpolatormetadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027474_cambasecamerametadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027481_camthirdpersoncamerametadatabasepivotposition_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027511_camthirdpersoncamerametadatacustomorbitdistance_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027514_camthirdpersoncamerametadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027518_camthirdpersoncamerasettings_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027521_camthirdpersoncamerametadata_camthirdpersoncamerametadata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027537_camfollowcamerametadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027538_camaimcamerametadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027542_camfirstpersonaimcamerametadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027555_camfirstpersonshootercamerametadatacoversettings_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027567_camfirstpersonshootercamerametadata_camfirstpersonshootercameram(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027571_camthirdpersonaimcamerametadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027578_camthirdpersonpedaimcamerametadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027599_camthirdpersonpedassistedaimcameracinematicmomentsettings_parser(ph: *mut Phantom); }
extern "C" { pub fn f_027611_camthirdpersonpedassistedaimcamerametadata_camthirdpersonpedassi(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027621_camthirdpersonpedaimincovercamerametadata_camthirdpersonpedaimin(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027625_camthirdpersonpedmeleeaimcamerametadata_camthirdpersonpedmeleeai(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027629_cambasecinematictrackingcamerametadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027659_camanimatedcamerametadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027671_camcinematiccameraoperatorshakesettings_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027674_camcinematiccameramancamerametadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027680_camcinematichelichasecamerametadata_camcinematichelichasecameram(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027714_camcinematicmountedcamerametadata_camcinematicmountedcamerametad(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027721_camcinematictraintrackingcamerametadata_camcinematictraintrackin(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027734_camfollowpedcamerametadataassistedmovementalignment_parser_regis(ph: *mut Phantom); }
extern "C" { pub fn f_027737_camfollowpedcamerametadataladderalignment_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027764_camfollowpedcamerametadata_camfollowpedcamerametadata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027771_camfollowvehiclecamerametadataduckunderoverheadcollisionsettings(ph: *mut Phantom); }
extern "C" { pub fn f_027789_camfollowcamerametadata_camfollowcamerametadata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027790_camfollowobjectcamerametadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027802_cammarketingfreecamerametadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027817_camscriptedcamerametadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027826_cambasesplinecamerametadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027829_camroundedsplinecamerametadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027832_camsmoothedsplinecamerametadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027835_camtimedsplinecamerametadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027850_camreplaybasecamerametadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027862_camreplayfreecamerametadata_camreplayfreecamerametadata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027863_camcinematicshotmetadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027977_camcinematiccontextmetadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_027980_camcinematicinvehiclecontextmetadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_028028_camcinematicintraincontextmetadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_028049_cambasedirectormetadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_028074_camgameplaydirectormetadata_camgameplaydirectormetadata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028119_camvehiclecustomsettingsmetadata_dtorcamvehiclecustomsettingsmet(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028125_camthirdpersoncamerametadata_dtorcamthirdpersoncamerametadata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028135_camfirstpersonshootercamerametadata_dtorcamfirstpersonshootercam(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028141_camthirdpersonpedassistedaimcamerametadata_dtorcamthirdpersonped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028157_camcinematicmountedcamerametadata_dtorcamcinematicmountedcameram(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028165_camfollowpedcamerametadata_dtorcamfollowpedcamerametadata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028169_camfollowvehiclecamerametadata_dtorcamfollowvehiclecamerametadat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028179_camreplayfreecamerametadata_dtorcamreplayfreecamerametadata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028191_cammetadatastore_dtorcammetadatastore(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028197_camfactory_addwidgetsformetadata_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_028202_camfactory_debugdumpobjectpool_rage_fwbasepool_const_ptr_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_028203_camfactory_debugdumpcreateobjectcallstack_rage_fwbasepool_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_028219_cammanager_updateinternal(ph: *mut Phantom); }
extern "C" { pub fn f_028220_rage_atarray_trenderedcameraobjectsettings_0_unsigned_short_grow(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028221_cammanager_removeblendedoutrenderedobjects_rage_atarray_trendere(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_028222_rage_atarray_trenderedcameraobjectsettings_0_unsigned_short_op_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_028223_cammanager_addwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_028229_cammanager_debugselectpreviouscamera(ph: *mut Phantom); }
extern "C" { pub fn f_028230_cammanager_debugselectnextcamera(ph: *mut Phantom); }
extern "C" { pub fn f_028231_cammanager_debugselectvalidation_rage_atarray_cambasecamera_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_028232_cammanager_debugrendertableentry_unsigned_char_unsigned_char_cha(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_028234_cammanager_debugrendercameratext_cambasecamera_const_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_028244_ccameradebuginfo_printcamerainfo_cambasecamera_const_ref_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_028257_cviewportmanager_shutdownrenderphases(ph: *mut Phantom); }
extern "C" { pub fn f_028259_cviewportmanager_createfinalscreenrenderphaselist(ph: *mut Phantom); }
extern "C" { pub fn f_028260_cviewportmanager_computevendormsaa_rage_grctexturefactory_create(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_028262_cviewportmanager_pushviewport_cviewport_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_028263_cviewportmanager_popviewport(ph: *mut Phantom); }
extern "C" { pub fn f_028329_cambasecinematiccontext_dtorcambasecinematiccontext_49116(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028363_camcinematicwatercrashcontext_dtorcamcinematicwatercrashcontext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028378_cambasecinematicshot_dtorcambasecinematicshot_49198(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028553_cambaseswitchhelper_dtorcambaseswitchhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028583_camreplayfreecamera_dtorcamreplayfreecamera(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028595_cambasesplinecamera_dtorcambasesplinecamera_49580(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_028907_rage_matrix34_toeulersxzy_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_028908_rage_matrix34_toeulersyzx_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_028909_rage_matrix34_toeulerszxy_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029265_cgame_issessioninitialized(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_029420_ccutsceneanimation_ccutsceneanimation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029421_ccutsceneanimation_dtorccutsceneanimation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029422_ccutsceneanimation_init_rage_crclip_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_029423_ccutsceneanimation_remove(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029424_ccutsceneanimation_getphase_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_029425_ccutsceneanimation_getquaternionvalue_rage_crclip_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_029426_ccutsceneanimation_getvector3value_rage_crclip_const_ptr_int_flo(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_029427_ccutsceneanimation_getfloatvalue_rage_crclip_const_ptr_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_029433_getorcreategroup_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029455_cdebugscene_tunewidgetdata_tunewidgetdata_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_029456_cdebugscene_queuetunewidget_cdebugscene_tunewidgetdata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029612_worldprobe_cshapetestdesc_setresultsstructure_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029613_worldprobe_cshapetestresults_freeintersections(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029726_camframe_setworldmatrix_rage_matrix34_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029735_camframe_clonefrom_camframe_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029809_cdebugscene_getentityundermouse_rage_vector3_ptr_int_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_029866_cdebugscene_focusentities_get_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029927_cdebugdrawstore_addsphere_rage_vec3v_const_ref_float_rage_color3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_029967_cphysical_gethealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_030081_lightprobe_isenabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_030146_tiledscreencapture_isenabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_030151_tiledscreencapture_updateviewportperspective_rage_grcviewport_re(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32) -> i32; }
extern "C" { pub fn f_030476_cassetref_getassetindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030746_geteventscriptnetworkgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030816_cped_sdamagerecord_sdamagerecord_centity_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: f32, a7: f32, a8: i32, a9: i32, a10: i32, a11: i32) -> i64; }
extern "C" { pub fn f_034946_cloadingscreens_areactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034949_chudtools_getwidescreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035061_cnewhud_issnipersightactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035082_cnewhud_isweaponwheelactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036034_cscaleformmoviewrapper_isactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036254_cpausemenu_isactive_pm_activeflags(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036364_cprofilesettings_getint_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036382_cprofilesettings_exists_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036574_cuimenuped_updatepedpose(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036745_cvideoeditorinterface_getactiveproject(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037888_cseatmanager_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038467_cbaseikmanager_getsolver_ikmanagersolvertypes_flags_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038514_cikmanager_gettorsominpitch(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038515_cikmanager_gettorsomaxpitch(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038526_cikmanager_getpelvisdeltazforcamera_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_038652_cmlomodelinfo_getportaldata_unsigned_int_unsigned_int_ref_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_038715_cmodelinfo_requestassets_cbasemodelinfo_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038720_cmodelinfo_clearassetrequiredflag_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038733_centryexitpoint_getseat_seataccess_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_038735_cmodelseatinfo_calculateentrysituation_centity_const_ptr_cped_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32, a7: i64); }
extern "C" { pub fn f_038737_cseataccessor_getseatbyindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038739_cmodelseatinfo_getentrypointindexforseat_int_centity_const_ptr_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_038740_cmodelseatinfo_getboneindexfromseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038751_cseatmanager_getpedsseatindex_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038978_cvehiclevariationinstance_getmodindexfortype_evehiclemodtype_cve(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_039010_cvehiclemodelinfo_getfirstpersonlookarounddata_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039211_cvehiclevariationinstance_getkit_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039634_cobject_processsniperdamage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039644_cobject_getpointertoclosestmapobjectforscript_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_039708_rage_mthrandom_reset_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039765_cobjectpopulation_converttorealobject_cdummyobject_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039882_cpathserver_getclosestpositionforped_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_040013_cpathserverthread_getnavmeshpolyforrouteendpoints_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: f32, a7: f32, a8: i32, a9: i32, a10: i32, a11: f32, a12: i64) -> i32; }
extern "C" { pub fn f_041056_findplayerped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041063_cpedintelligence_isfriendlywith_cped_const_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041065_cplayerpedtargeting_tunables_gettargettinginfo_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041067_cplayerpedtargeting_getisfreeaimassisttargetposwithinreticuleslo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_041068_cplayerpedtargeting_getaimdistancescalecurve(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041069_cplayerpedtargeting_getcurveset_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_041226_rage_syslinkeddata_unsigned_int_refreportplayerresetmismatch_uns(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041237_cpedintelligence_gettaskmelee_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041265_cplayerinfo_isaiming_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041300_cpedresetflags_resetpreaipostinventory(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041385_cpedintelligence_findtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041387_cpedintelligence_gettaskmeleeactionresult_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041389_cpedintelligence_ispedclimbing_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041422_cpedailodmanager_shoulddofullupdate_cped_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041438_cpedintelligence_getoverriddengameplaycamerasettings_tgameplayca(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041455_cpedmotiondata_getisdesiredsprinting_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041589_cexplosionmanager_cexplosionargs_dtorcexplosionargs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041666_cplayerinfo_isrunandgunning_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_041674_cplayerpedtargeting_clearlockontarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041676_cplayerpedtargeting_getisfreeaimassisttargetposwithinconstraints(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_041686_cplayerinfo_issoftfiring(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041688_cplayerinfo_isfirstpersonmodesupportedforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041716_cplayerinfo_isfiring_s(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041735_cplayerinfo_gettimesincelastaimedms(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041746_reportplayerresetmismatch_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_041878_cpedtargetevaluator_checkbasictargetexclusions_cped_const_ptr_ce(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: f32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_041928_cplayerpedtargeting_getfineaimingoffset_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_042003_cqueriableinterface_gettaskinfofortasktype_int_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042004_cqueriableinterface_getstatefortasktype_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042007_cqueriableinterface_gettargetentrypointfortasktype_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042166_cpeddamagemanager_updatepriority_unsigned_char_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_042168_cpeddamagemanager_addblooddamage_unsigned_char_unsigned_short_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_042169_cpeddamagemanager_getbloodinfoanddamageid_cped_ptr_rage_atnonfin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_042173_cpeddamagemanager_addpedblood_cped_ptr_epeddamagezones_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: f32, a6: i64, a7: i32, a8: f32, a9: i32, a10: i32, a11: i32) -> i32; }
extern "C" { pub fn f_042176_cpeddamagemanager_getclanidforped_cped_const_ptr_long_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_042189_cpeddamagemanager_clearpedblood_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042190_cpeddamagemanager_replayisblitidvalid_unsigned_char_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_042400_cpedstreamrendergfx_getdrawable_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042404_cpedvariationpack_getlocalcompdata_cped_const_ptr_epedvarcomp_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_042409_cpedheadblenddata_cpedheadblenddata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042412_cpedheadblenddata_dtorcpedheadblenddata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042425_cpedvariationpack_extractcomponent_rage_pgdictionary_rage_rmcdra(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_042437_cpedvariationpack_gettexvar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042438_cpedvariationpack_getcompvar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042489_cpedpropsmgr_getpedpropidx_cped_const_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042814_worldprobe_debugfindgroundzfor3dcoord_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> f32; }
extern "C" { pub fn f_042818_worldprobe_cshapetestmanager_debugsubmittest_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_042821_worldprobe_cshapetestmanager_aborttest_worldprobe_cshapetestresu(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042848_worldprobe_cshapetestdesc_setexcludeentities_centity_const_ptr_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_043046_ccollisionhistory_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043077_cbuoyancy_getbuoyancyinfo_cphysical_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_043080_cbuoyancy_getwaterlevelonsample_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_043085_worldprobe_cshapetestdesc_setexcludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043163_phmaterialdebug_renderbound_rage_phinst_ptr_rage_phbound_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_043195_phmaterialmgrgta_getpackedpolyflagvalue_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043211_cphysics_breakglass_cdynamicentity_ptr_rage_vec3v_const_ref_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: f32, a5: i32) -> i32; }
extern "C" { pub fn f_043212_cphysics_verletsimupdate_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043215_cphysics_ropesimupdate_float_int(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_043234_worldprobe_cshapetestdesc_setincludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043243_worldprobe_cshapetestbatchdesc_addlinetest_worldprobe_cshapetest(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043246_worldprobe_cshapetestbatchdesc_setprobedescriptors_worldprobe_cs(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043342_worldprobe_cshapetestdesc_setexcludeinstances_rage_phinst_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043346_worldprobe_cshapetestdesc_setincludeinstance_rage_phinst_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043347_worldprobe_cshapetestdesc_setincludeentities_centity_const_ptr_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043348_worldprobe_cshapetestdesc_addexludeinstance_rage_phinst_const_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043478_cmirrors_getresolutionx(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043479_cmirrors_getresolutiony(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043480_crendertargets_createrendertarget_rtmempoolid_char_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_043486_cmirrors_getismirrorvisible_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043489_cocclusion_gethizbuffer_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_043592_processlodlightvisibility_processvisibility_rage_fwstepallocator(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i64, a8: i32, a9: i64, a10: i32, a11: i64, a12: i32, a13: i64, a14: i64, a15: i64, a16: i32); }
extern "C" { pub fn f_043666_postfx_dopauseresolve_postfx_eresolvepause(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043677_animpostfxmanager_start_rage_atnonfinalhashstring_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_043714_animpostfxmanager_stop_rage_atnonfinalhashstring_animpostfxmanag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043774_clightentity_processvisiblelight_rage_fwvisibilityflags_const_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043775_ui3ddraweditor_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043779_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_043780_forceuplinkedinteriors_rage_fwinteriorlocation_cinteriorinst_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_043782_cpostscan_waitforsortpvsdependency(ph: *mut Phantom); }
extern "C" { pub fn f_043815_lights_addscenelight_clightsource_ref_clightentity_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043816_cparaboloidshadow_checkandupdatemovedlight_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043901_crendertargetmgr_setnamedrendertargetdelayload_rage_atfinalhashs(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_043903_crendertargetmgr_getnamedrendertarget_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_043905_crendertargetmgr_registernamedrendertarget_char_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_043909_crendertargetmgr_getnamedrendertargetid_char_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_043911_crendertargetmgr_linknamedrendertargets_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_043961_cgtaoldloadingscreen_setloadingrenderfunction(ph: *mut Phantom); }
extern "C" { pub fn f_043969_river_ispositionnearriver_impl_rage_vec3v_const_ref_int_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_043983_csprite2d_drawrect_float_float_float_float_float_rage_color32_co(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i64); }
extern "C" { pub fn f_044122_water_getwaterlevel_rage_vector3_const_ref_float_ptr_bool_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_044125_water_testlineagainstwater_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_044189_lights_setupdirectionalandambientglobals(ph: *mut Phantom); }
extern "C" { pub fn f_044201_water_addfoam_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32); }
extern "C" { pub fn f_044428_rage_populatebitfields_rage_fraginst_ptr_unsigned_long_ref_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_044527_lights_calculatetimefade_float_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_044537_lights_registerbrokenlight_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044538_lights_removeregisteredbrokenlight_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044545_lights_calculatefromambient_clightsource_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044567_clodlights_getupdatelodlightrange_elodlightcategory(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_044607_cparaboloidshadow_updatelight_clightsource_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_044845_crenderphaseparaboloidshadow_crenderphaseparaboloidshadow_cviewp(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_044880_crenderphaseprerenderviewport_crenderphaseprerenderviewport_cvie(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_044893_crenderphasescript2d_crenderphasescript2d_cviewport_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_044996_creplaycoordinator_shouldshowloadingscreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045789_cped_ismale_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045793_cped_checkbraveryflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045794_cped_checksexinessflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045797_cped_getcurrentmotiontask_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045840_cped_getbonematrix_rage_matrix34_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045889_cped_getprimarymotiontask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045903_cped_getiscrouching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045912_cped_isusingactionmode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045913_cped_isfirstpersonshootermodeenabledforplayer_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045922_cped_getboneposition_rage_vector3_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045926_cped_getisdrivingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045945_cped_setheadblenddata_cpedheadblenddata_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045952_cped_micromorph_emicromorphtype_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_045979_cped_getfacingdirectionheading_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_045996_cped_getbonetagfromragdollcomponent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046002_cped_isinfirstpersonvehiclecamera_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_046021_cped_processcontrol_resetvariables(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046022_cped_processcontrol_intelligence_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_046023_cped_processcontrol_weaponsandaccessoriespostintelligence_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046024_cped_updatemovementmode_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046026_cped_getvehiclepedentering_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046048_cped_getisfpsswimming_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046078_cped_renderpedlight_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046079_cped_renderlightonphone_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046097_cped_getbonematrixcached_rage_matrix34_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_046102_cped_getisenteringvehicle_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046103_cped_getspecialability_eplayerspecialabilityslot(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046104_cped_isexitingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046119_cped_calccameraattachpositionforragdoll_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046120_cped_getisfpsswimmingunderwater_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046141_cped_isbeingjackedfromvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046159_cped_computelowcoverheightoffsetfrommover_float_float_float_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32) -> f32; }
extern "C" { pub fn f_046170_cped_isallowedtodamageentity_cweaponinfo_const_ptr_centity_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_046176_cped_getcontrolfromplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046179_cped_instantanimupdateend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046180_cped_instantaiupdate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046203_cped_adddamagerecord_cped_sdamagerecord_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046225_cped_gethighheelexpressiondof(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_046226_cped_getfirstpersoncameradofs_rage_vec3v_ref_rage_quatv_ref_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64) -> i32; }
extern "C" { pub fn f_046324_cgameworld_findfollowplayerspeed(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046325_cgameworld_findfollowplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_046327_cplayerswitchinterface_isactive_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_046404_cportaltracker_setloadscollisions_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046405_cportaltracker_addtoactivatingtrackerlist_cportaltracker_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046410_centity_getinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046442_cinteriorproxy_getfromlocation_rage_fwinteriorlocation_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046443_cinteriorinst_getportalinroom_unsigned_int_unsigned_int_rage_fwp(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_046444_cinteriorinst_getmatchingportalinst_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_046445_cportaltracker_collectstack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046449_cportaltracker_movetonewlocation_rage_fwinteriorlocation(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046478_centity_getplaceabletracker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046484_centity_setisvisibleformodule_eisvisiblemodule_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046489_centity_getistouching_centity_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046491_centity_getisonscreen_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046499_centity_getcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046502_centity_getphysarch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046515_centity_changevisibilitymask_unsigned_short_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046524_centity_getaudioenvironmentgroup_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046537_getentitytypestr_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_046544_centity_getusedinreplay_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046545_centity_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046566_cfrustumdebug_debugrender_cfrustumdebug_const_ptr_rage_color32(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046597_centityidhelper_getentityfromid_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046599_centityiterator_centityiterator_int_centity_const_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_046600_centityiterator_getnext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047012_cfocusentitymgr_getpos(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047013_cfocusentitymgr_getentity_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047014_cfocusentitymgr_getvel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047034_cportaltracker_probeforinteriorproxy_rage_vector3_const_ref_cint(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32) -> i32; }
extern "C" { pub fn f_047044_cgameworld_forallentitiesintersecting_rage_fwsearchvolume_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_047066_cphysical_getreferenceframevelocity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047106_cphysical_getlocalspeed_rage_vector3_const_ref_bool_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047126_cphysical_debugprocessallattachments_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_047176_cmapoptimizehelper_performnewscan_rage_vector3_const_ref_bool_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_047292_cstreamingbucketset_dumpallentitiesonce(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047310_centityreportentry_centityreportentry_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_047312_csceneisolator_getentitycost_centity_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047322_cmapoptimizehelper_appendtolistcb_centity_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_047323_gettxdrefsusedbyentity_rage_atmap_ctxdref_unsigned_int_rage_atma(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047327_rage_atmap_ctxdref_unsigned_int_rage_atmaphashfn_ctxdref_rage_at(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047333_clodmgr_withinvisiblerange_centity_ptr_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_047339_cloddebug_findchildren_centity_ptr_rage_atarray_centity_ptr_0_un(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047340_cscenecosttracker_resetthisframe(ph: *mut Phantom); }
extern "C" { pub fn f_047341_cscenecosttracker_addcostsforentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047342_cscenecosttracker_writetolog_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047344_cpostscandebug_processprerenderentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047345_cinteriorinst_createlocation_cinteriorinst_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_047355_csceneloader_loadscene_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047374_cgameworld_changeplayerped_cped_ref_cped_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_047701_cfrustumdebug_set_rage_matrix34_const_ref_float_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32); }
extern "C" { pub fn f_047713_cinteriorinst_getnumportalsinroom_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047714_cinteriorinst_getportalidxinroom_unsigned_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_047734_cinteriorinst_findroombyhashcode_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047736_cinteriorinst_setalphaforroomzeromodels_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047737_cinteriorinst_suppressinternallod(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047844_cportal_getdoorocclusion_cinteriorinst_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_047856_cportaltracker_probeforinterior_rage_vector3_const_ref_cinterior(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_047860_cportaltracker_ispedinsidestiltapartment_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047861_cportaltracker_getcurrroomhash(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047862_cportalvistracker_cportalvistracker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047869_cportalvistracker_getinteriorlocation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047924_cscenestreamer_sendfakesceneremainder_rage_atarray_bucketentry_1(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047930_void_std_2_introsort_std_2_classicalgpolicy_bucketentrysort_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_047931_cstreamingbucketset_isnearbyinteriorshell_cgtapvsentry_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_047935_cstreamingbucketset_showbucketusage_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047936_cstreamingbucketset_showbucketcontents_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047937_cstreaminglists_isrequired_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047938_cstreaminglists_addneeded_cgtapvsentry_ptr_centity_ptr_rage_fwvi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_047939_cstreamvolume_extractentitiesfrommapdata_rage_atarray_centity_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047949_cstreamvolume_updateloadedstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047970_cgameworld_findobjectsinrange_rage_vector3_const_ref_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_048037_cgameworldheightmap_getminheightfromworldheightmap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_048043_cgameworldheightmap_getmaxheightfromworldheightmap_float_float_f(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_063823_cscripteulers_matrixfromeulers_rage_matrix34_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_064141_cstreamedscripts_getnumrefs_rage_strlocalindex_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_064588_ccustomshadereffecttint_selecttintpalette_unsigned_char_rage_fwe(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064698_cshaderlib_setglobalalpha_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_064708_cshaderlib_setmsaaparams_rage_grcdevice_msaamode_const_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064725_cshaderlib_setstereotexture(ph: *mut Phantom); }
extern "C" { pub fn f_064736_csettingsmanager_csettingsmanager(ph: *mut Phantom); }
extern "C" { pub fn f_064737_csettingsmanager_getsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064800_cstreaming_hasobjectloaded_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064832_cstreaming_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064845_cstreaming_isobjectinimage_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064913_ckeyboard_getkeyjustdown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_064914_rage_czonedassetmanager_showzonedetails(ph: *mut Phantom); }
extern "C" { pub fn f_064915_rage_czonedassetmanager_drawzoneboxes(ph: *mut Phantom); }
extern "C" { pub fn f_064916_rage_czonedassetmanager_clearrequests(ph: *mut Phantom); }
extern "C" { pub fn f_064917_rage_czonedassetmanager_addrequest_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064933_cstreaming_requestobject_rage_strlocalindex_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_064980_addtoassetlistrecursive_rage_strindex_rage_atmap_rage_strindex_b(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_065127_cstreamingrequestlist_enablerecording_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_065368_ccontrol_setlighteffect_rage_iolighteffect_const_ptr_ccontrol_li(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_065369_ccontrol_clearlighteffect_rage_iolighteffect_const_ptr_ccontrol(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_065374_ccontrol_setinputexclusive_int_rage_iovalue_disableoptions_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_065376_ccontrol_getinputname_rage_inputtype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065379_ccontrol_dodisableinput_unsigned_int_rage_iovalue_disableoptions(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_065380_ccontrol_inputhowlongago_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065385_ccontrol_getvalue_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_065390_ccontrol_getactivelayout_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065397_ccontrol_ismousesteeringon_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_065446_ccontrolmgr_getmainplayercontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065457_cpad_getpresseddebugbuttons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065460_ccontrolmgr_getmaincameracontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065461_ccontrolmgr_getmainfrontendcontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065469_ccontrolmgr_startplayerpadshakebyintensity_unsigned_int_float_in(ph: *mut Phantom, a0: i32, a1: f32, a2: i32); }
extern "C" { pub fn f_065609_cfilemgr_getassetsfolder(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_065622_cfilemgr_openfileforappending_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065640_cpad_getdebugbuttons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066062_ccoverpoint_getcoverpointposition_rage_vector3_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066067_ccoverpoint_getcoverdirectionvector_rage_vec3v_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066208_ctaskmeleeactionresult_getcurrentcliptime_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_066701_ccover_findcoverdirectionforped_cped_const_ref_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066702_ccover_findcovercoordinatesforped_cped_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_066708_ccoverpoint_ccoverpoint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066729_ctaskcover_isplayeraimingdirectlyinfirstperson_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066730_ctaskcover_canusethirdpersoncoverinfirstperson_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066736_ctaskmotionincover_computecoverheading_cped_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_066850_ctaskincover_computecoveraimpositionandheading_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_066897_ctaskincover_isinmovingmotionstate_bool_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066910_ctaskincover_canpedfireroundcorner_cped_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_066924_cdynamiccoverhelper_findnewcoverpointaroundcorner_ccoverpoint_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_066930_ctaskmotionincover_iscoverflagset_int_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066961_ctaskmotionincover_computegaussianvalueforrestrictedcamerapitch(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_066962_ctaskmotionincover_computefacingheadingforcoverdirection_float_b(ph: *mut Phantom, a0: f32, a1: i32) -> f32; }
extern "C" { pub fn f_068471_ctaskmotionbase_getdefaultaimingstrafingclipset_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_068713_ctaskmotioninturret_isseatwithhatchprotection_int_cvehicle_const(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_069540_ctaskparachute_isparachuteout_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075349_cmovenetworkhelper_getfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_075502_cmovenetworkhelper_getclip_rage_fwmvtemplateid_rage_fwmvclipidty(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_075829_ctaskmotioninvehicle_getdirectaccessentrydoorfromseat_cvehicle_c(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_075974_ctaskvehiclefsm_getdoorforentrypointindex_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_076005_ctaskentervehicle_isconsideredgettinginvehicleforcamera_float(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076148_ctaskexitvehicle_isconsideredgettingoutofvehicleforcamera_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_076241_ctaskvehiclegun_needstosmashwindowfromtargetpos_cped_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_076294_ctaskvehiclefsm_getdoorforentrypointindex_cvehicle_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_076331_ctaskvehiclemountedweapon_wantstofire_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076339_ctaskvehiclemountedweapon_wantstoaim_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076449_ctaskmotioninautomobile_cantransitiontofirstpersonanims_bool_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_076767_ctaskaimgun_calculatedesiredyaw_cped_const_ptr_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_076865_ctaskgun_getisfiring_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077354_statsinterface_getfloatstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_077373_statsinterface_getstatsmodelhashid_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077475_cnumberwithinmessage_set_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077479_ctextfile_get_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077507_chelpmessage_setmessagetext_int_char_const_ptr_cnumberwithinmess(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_077525_cloadingtext_setactive_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077533_ctextlayout_ctextlayout(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077535_ctextlayout_dtorctextlayout(ph: *mut Phantom); }
extern "C" { pub fn f_077537_ctextlayout_setscale_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077540_ctextlayout_render_rage_vector2_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077604_ctimecycle_update_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077622_cvisualsettings_get_char_const_ptr_float_bool_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_077633_ctimecycleasyncquerymgr_addnonpedentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078786_cvehicle_getnumcontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078871_cvehicle_containslocalplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078876_camphibiousquadbike_iswheeltransitionfinished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078891_ccardoor_getislatched_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_078893_cheli_getiscargobob_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078909_cvehicle_istaximodelid_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078930_cvehicle_getishandbrakepressed_ccontrol_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_078978_cvehicle_fixwindow_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078979_cvehicle_rollwindowup_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078981_cvehicle_hascontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079000_cvehicle_istank_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079001_caircraftdamagebase_hassectionbrokenoff_cplane_const_ptr_int_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_079007_cvehicle_getnumberofpassenger_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079023_cvehicle_getattachedtrailer_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079039_cvehicle_isturretseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079047_cwheel_getwheelposandradius_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_079065_cvehicle_getdoorfromid_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079070_cvehicle_getseatanimationinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079071_cvehicle_isentryindexvalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079130_cvehicle_getlayoutinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079157_cvehicle_getisjetski_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079228_cvehicle_updatebodycolourremapping_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079335_chandlingdata_gethandlingdata_ehandlingtype(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079422_cvehicle_getvehiclegadget_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079423_cheli_doesbullethitpropellerbound_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079424_cheli_doesbullethitpropeller_int_rage_vector3_const_ref_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_079472_cplane_getverticalflightmodeavaliable_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079508_cplane_islargeplane_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079551_cvehicle_getentryaniminfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079643_cvehicle_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079659_ctrain_findengine_ctrain_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079673_ctrain_findcaboose_ctrain_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079678_ctrain_gettraintrack_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_079680_ctrain_getworldposfromtracknode_rage_vector3_ref_signed_char_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079688_ctrain_isengine_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079745_cvehiclelayoutinfo_getseatanimationinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079746_cvehicle_doesvehiclehaveaconvertibleroofanimation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079758_cvehicle_getconvertibleroofstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079784_cvehicle_setvariationinstance_cstoredvehiclevariations_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079785_cvehicle_getexhaustmatrix_ehierarchyid_rage_mat34v_ref_int_ref_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_079806_cvehicle_hastowinggadget_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079807_cvehicle_getentitybeingtowed_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079812_cvehicle_getentryinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079815_cvehicle_getseatanimationinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_079816_cvehicle_getseatinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079817_cvehicle_getseatinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_079870_cturret_getturretmatrixworld_rage_matrix34_ref_cvehicle_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079872_cvehicle_getseathasweaponsorturrets_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079883_cvehiclelayoutinfo_getseatinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079885_cvehicle_getentryexitpoint_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079908_cvehicle_getconvertibleroofprogress_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_079918_cvehicle_setrespotcounter_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079930_cvehicle_getfirstturretseat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079932_cvehicleweaponmgr_getfirstturretforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080029_cvehiclegadgetpickuprope_getattachedvehicle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080080_cvehicleweaponmgr_getweaponsforseat_int_rage_atarray_cvehiclewea(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_080237_cturret_getcurrentheading_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_080268_cvehicleweaponmgr_getfirstfixedweaponforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080469_cstoredvehiclevariations_cstoredvehiclevariations(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080541_cvehiclemetadatamgr_getvehicledrivebyinfofromped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080671_cvehiclemetadatamgr_getfirstpersondrivebylookarounddataatindex_u(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_080711_cvehicledrivebyinfo_getminrestrictedaimsweepheadingangledegs_cpe(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_080712_cvehicledrivebyinfo_getmaxrestrictedaimsweepheadingangledegs_cpe(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_080864_cvfxhelper_getdistsqrtocamera_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_080865_cvfxhelper_getmatrixfrombonetag_rage_mat34v_ref_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_080866_cvfxhelper_getmatrixfromboneindex_rage_mat34v_ref_bool_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_080871_cvfxhelper_creatematfromvecy_rage_mat34v_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080873_cvfxhelper_creatematfromvecz_rage_mat34v_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080875_cvfxhelper_createrelativemat_rage_mat34v_ref_rage_mat34v_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080879_cvfxhelper_getinterpvalue_float_float_float_bool(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_080963_ctrailgroup_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080976_cvfxlightningmanager_addlightningstrike(ph: *mut Phantom); }
extern "C" { pub fn f_080990_cptfxmanager_update_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_080991_ctrailgroup_addpoint_vfxtrailinfo_t_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_081163_vfxpedskeletonboneinfo_getbonewaterinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081165_cvfxpedinfomgr_getinfo_cped_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081166_cdecalmanager_findrendersettinginfo_unsigned_int_int_ref_int_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_081191_cvfxvehicleinfomgr_getinfo_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081206_rage_cloudhatmanager_requestweathercloudhat_int_float_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_081240_rage_cloudhatmanager_requestscriptcloudhat_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_081254_rage_cloudhatmanager_unloadallcloudhats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081343_cdecalmanager_addsettings_centity_ptr_rage_decalusersettings_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_081349_cdecalmanager_addgeneric_decaltype_e_int_int_rage_vec3v_const_re(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: f32, a7: f32, a8: f32, a9: f32, a10: f32, a11: f32, a12: f32, a13: f32, a14: i32, a15: i32, a16: i32, a17: f32, a18: i32, a19: i32, a20: f32) -> i32; }
extern "C" { pub fn f_081362_cvehiclebadgemanager_request_cvehicle_const_ptr_cvehiclebadgedes(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: f32, a7: i32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_081363_cvehiclebadgedesc_set_emblemdescriptor_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081388_cdecalmanager_doesvehiclehavebadge_cvehicle_const_ptr_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_081389_cdecalmanager_addfootprint_int_int_float_float_rage_color32_unsi(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: f32, a4: i32, a5: i64, a6: i64, a7: i64, a8: i64, a9: i32, a10: f32) -> i32; }
extern "C" { pub fn f_081391_cdecalmanager_addweaponimpact_vfxweaponinfo_s_const_ref_vfxcolli(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_081392_cdecalmanager_remove_rage_fwentity_const_ptr_int_void_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_081394_cdecalmanager_getvehiclebadgerequeststate_cvehicle_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_081416_ccoronas_register_rage_vec3v_const_ref_float_rage_color32_float(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: f32, a4: f32, a5: i64, a6: f32, a7: f32, a8: f32, a9: i32); }
extern "C" { pub fn f_081443_cvfxweapon_doweaponimpactvfx_vfxcollinfo_s_const_ref_edamagetype(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_081445_cfiremanager_startfire_cfiresettings_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081448_cfire_getmaxradius_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_081449_cfiremanager_getmapstatusgridindices_rage_vec3v_const_ref_int_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_081455_cfiremanager_registervehicletyrefire_cvehicle_ptr_int_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: i32) -> i64; }
extern "C" { pub fn f_081458_cfiremanager_registervehicletankfire_cvehicle_ptr_int_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i64); }
extern "C" { pub fn f_081465_cfiremanager_getfire_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_081473_gameglows_addfullscreengameglow_rage_vec3v_const_ref_float_rage(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_081554_scriptim_setbackfaceculling_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081555_scriptim_setdepthwriting_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081726_cvfxweapon_getinfo_vfxgroup_e_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_081746_cvehicleglassmanager_storecollision_vfxcollinfo_s_const_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081749_cvehicleglassmanager_storeexplosion_vfxexpinfo_s_const_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081751_cvehicleglassmanager_removecomponent_cphysical_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081759_cvfxblood_triggerptfxblood_cvfxpedinfo_const_ptr_vfxbloodinfo_s(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i64, a10: i32, a11: i32, a12: i64, a13: i64); }
extern "C" { pub fn f_081785_cvfxentity_updateptfxentityanim_centity_ptr_cparticleattr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_081787_cvfxentity_triggerptfxentityanim_centity_ptr_cparticleattr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081789_cvfxentity_triggerdecalgeneric_vfxentitygenericdecalinfo_s_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_081854_cvfxscript_getscriptptfxassetname_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081870_cvfxvehicle_triggerptfxhelirotordestroy_crotarywingaircraft_ptr(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081872_cvfxvehicle_updateptfxvehicleweaponcharge_cvehicle_ptr_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_081879_cvfxwater_triggerptfxsplashpedin_cped_ptr_cvfxpedinfo_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i32); }
extern "C" { pub fn f_081880_cvfxwater_triggerptfxsplashpedout_cped_ptr_cvfxpedinfo_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32); }
extern "C" { pub fn f_081881_cvfxwater_updateptfxsplashpedwade_cped_ptr_cvfxpedinfo_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: f32, a8: i32); }
extern "C" { pub fn f_081884_cvfxwater_triggerptfxboatentry_cvehicle_ptr_cvfxvehicleinfo_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_081885_cvfxwater_updateptfxboatbow_cvehicle_ptr_cvfxvehicleinfo_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_081886_cvfxwater_updateptfxboatwash_cvehicle_ptr_cvfxvehicleinfo_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_081888_cvfxwater_triggerptfxsplashheliblade_centity_ptr_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081900_cvfxweapon_triggerptfxbullettrace_centity_const_ptr_cweapon_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_081901_cvfxweapon_getweapontintcolour_unsigned_char_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081966_bool_creplaymgrinternal_handleentityforfx_cpacketvehiclebadgereq(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082020_cairdefencezone_isentitytargetable_centity_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082026_cweapon_cweapon_unsigned_int_int_cdynamicentity_ptr_bool_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_082028_cexplosionmanager_addexplosion_cexplosionmanager_cexplosionargs(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082029_cweapon_dtorcweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082054_cbullet_dtorcbullet(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082055_cbullet_generatewhizzedbyeventatendposition_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082056_cbullet_processhitsoceanwater_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082057_cbullet_computeclothimpacts_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082058_cbullet_computeropeimpacts_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082060_cbullet_computepassesthrough_worldprobe_cshapetesthitpoint_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> f32; }
extern "C" { pub fn f_082061_cweapondamage_doweaponimpactvfx_cweapon_ptr_centity_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i64, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_082062_cweapondamage_doweaponimpact_cweapon_ptr_centity_ptr_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i64, a6: i32, a7: i32, a8: i64, a9: i64, a10: i32, a11: i64, a12: f32, a13: f32, a14: f32, a15: i32, a16: i32, a17: i64, a18: i64, a19: i32) -> f32; }
extern "C" { pub fn f_082063_cweapon_sendfiremessage_centity_ptr_rage_vector3_const_ref_world(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i64, a11: f32, a12: f32, a13: f32, a14: i64); }
extern "C" { pub fn f_082064_cpedweaponmanager_getequippedvehicleweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082065_cpedweaponmanager_getequippedweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082066_bool_creplaymgrinternal_recordfx_cpacketweaponbullettracefx_cpac(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082069_cweapon_getmuzzleposition_rage_matrix34_const_ref_rage_vector3_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082071_cpedweaponmanager_getequippedweapon_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082174_cweapon_processanimation_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082180_cprojectile_postprerender(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082184_cweapon_calcfirevecfromaimcamera_centity_const_ptr_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_082186_cweapon_getaccuracy_cped_const_ptr_float_sweaponaccuracy_ref_con(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_082368_cweaponcomponentscope_updatespecialscope_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082601_cpedweaponselector_getselectedvehicleweaponinfo_cped_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082634_cpedweaponmanager_processpreai(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082652_cpedweaponmanager_getisarmedmelee_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082671_cweaponitem_getkeyfromhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082726_cprojectilemanager_getprojectilesforowner_centity_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082809_bool_creplaymgrinternal_handleentityforfx_cpacketweaponbullettra(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083028_networkinterface_isinspectatormode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083029_networkinterface_isentityhiddennetworkedplayer_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083061_networkinterface_getnetworkobject_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083070_networkinterface_getspectatorobjectid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083125_networkinterface_getlastposreceivedovernetwork_cdynamicentity_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083126_networkinterface_getlastvelreceivedovernetwork_cdynamicentity_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083168_networkinterface_arebulletsimpactsdisabledinmp_centity_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083902_csyncedentity_getentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084088_clivemanager_issystemuishowing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_086744_cnetgameplayer_getplayerped_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090710_asinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090714_atanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090757_expf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090805_log10f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090823_pow(ph: *mut Phantom, a0: f64, a1: f64) -> f64; }
extern "C" { pub fn f_090826_powf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090901_qsort_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_090911_rand(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090918_sinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090920_snprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090921_siprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090922_small_sprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090926_strcasecmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090930_strcmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090931_strcpy(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090939_strlen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090965_tanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
