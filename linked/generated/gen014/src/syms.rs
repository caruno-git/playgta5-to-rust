//! Callee declarations for crate gen014.
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
extern "C" { pub fn f_000660_rage_atstring_indexof_char_const_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000680_rage_athashstringnamespacesupport_computehash_rage_athashstringn(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_000681_rage_athashstringnamespacesupport_addstring_rage_athashstringnam(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_000684_rage_athashstringnamespacesupport_trygetstring_rage_athashstring(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_000703_rage_bkbank_unsetcurrentgroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000782_rage_bkmanager_findbank_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000786_rage_bkmanager_isbankthread(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001232_rage_rsonreader_rsonreader(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001233_rage_rsonreader_rsonreader_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001234_rage_rsonreader_init_char_const_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001237_rage_rsonreader_hasmember_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001238_rage_rsonreader_getmember_char_const_ptr_rage_rsonreader_ptr_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001250_rage_rsonreader_asfloat_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001253_rage_rsonreader_asstring_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001514_rage_diagdisplayf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001524_displayf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001527_warningf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001534_quitf_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001535_rage_diaglogf2_rage_diagchannel_const_ref_rage_diagseverity_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001565_printf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001639_rage_fiassetmanager_removeextensionfrompath_char_ptr_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001874_void_rage_atplaceptrcreator_void_ptr_rage_fideviceinstaller_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002294_rage_atarray_rage_psorscenumschemavaluedata_0_unsigned_short_gro(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002402_rage_parenumdata_namefromvalueunsafe_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002739_rage_atpartialstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002743_rage_atdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002945_rage_sysmempoolallocator_safeallocate_unsigned_long_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003097_char_ptr_rage_formatf_char_16ul_char_ref_16ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003159_rage_matrix34_fromquaternion_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003829_rage_fwanimdirectorcomponentsyncedscene_issyncedscenelooped_shor(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003832_rage_fwanimdirectorcomponentsyncedscene_isvalidsceneid_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003834_rage_fwanimdirectorcomponentsyncedscene_getsyncedscenerate_short(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_003835_rage_fwanimdirector_getmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003840_rage_fwclipsetmanager_getclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003877_rage_fwanimhelpers_getcameratracktranslation_rage_crclip_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003881_rage_fwanimhelpers_getcameratrackrotation_rage_crclip_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003885_rage_fwanimhelpers_getcameratrackrotationdelta_rage_crclip_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003889_rage_fwanimhelpers_getcameratracktranslationdiffrotated_rage_crc(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003895_rage_fwanimmanager_getclipifexistsbyname_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003896_rage_fwanimmanager_getclipifexistsbydictindex_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_003904_rage_fwanimmanager_findslot_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003905_rage_fwanimmanager_findslotfromhashkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003907_rage_fwanimmanager_isvalidslot_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003984_rage_fwclipsetrequesthelper_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003992_rage_fwclipsetmanager_getclipsetid_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004010_rage_fwclipsetmanager_isstreamedin_deprecated_rage_fwmvtemplatei(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004019_rage_fwclipsetrequesthelper_request_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_004038_rage_fwclipsetrequesthelper_fwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004280_rage_fwposematcher_fwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004397_rage_fwanimdirectorcomponentsyncedscene_setsyncedscenephase_shor(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_004398_rage_fwanimdirectorcomponentsyncedscene_getsyncedscenephase_shor(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004399_rage_fwanimdirectorcomponentsyncedscene_setsyncedscenerate_short(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_004401_rage_fwanimdirectorcomponentsyncedscene_setsyncedsceneorigin_sho(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_004406_rage_fwanimdirectorcomponentsyncedscene_setsyncedscenelooped_sho(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_004407_rage_fwanimdirectorcomponentsyncedscene_issyncedsceneholdlastfra(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004408_rage_fwanimdirectorcomponentsyncedscene_setsyncedsceneholdlastfr(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_004411_rage_fwanimdirectorcomponentsyncedscene_attachsyncedscene_short(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_004412_rage_fwanimdirectorcomponentsyncedscene_detachsyncedscene_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004413_rage_fwanimdirectorcomponentsyncedscene_takeownershipofsyncedsce(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_004416_rage_fwanimdirectorcomponentsyncedscene_startsynchronizedscene_r(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004632_rage_fwarchetypemanager_lookupmodelid_rage_fwarchetype_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004633_rage_fwarchetypemanager_getarchetype_rage_atnonfinalhashstring_u(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004634_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004661_rage_fwdynamicentitycomponent_fwdynamicentitycomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004663_rage_fwdynamicentitycomponent_createattachmentextensionifmissing(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004725_rage_fwwaypointrecordingroute_uint8tombr_unsigned_char(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004729_rage_fwwaypointrecordingroute_uint8tocruisespeed_unsigned_char(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004730_rage_fwwaypointrecordingroute_getclosestwaypointindex_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_004731_rage_fwwaypointrecordingroute_getwaypointdistancefromstart_int_c(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_004740_rage_fwdecoratorextension_existson_rage_fwextensiblebase_ref_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005104_rage_clothmanager_computeclothimpacts_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_005170_rage_ropedatamanager_unloadropetexures(ph: *mut Phantom); }
extern "C" { pub fn f_005171_rage_ropedatamanager_loadropetexures(ph: *mut Phantom); }
extern "C" { pub fn f_005196_rage_ropeinstance_setphysinstflags_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005200_rage_ropeinstance_getdistancebetweenends_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_005201_rage_ropeinstance_updatecontrolvertices(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005206_rage_ropeinstance_attachobjects_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: i32, a11: i64, a12: i64, a13: i64, a14: i64); }
extern "C" { pub fn f_005207_rage_ropeinstance_attachtoobject_rage_vec3v_const_ref_rage_phins(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_005208_rage_ropeinstance_detachfromobject_rage_phinst_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005209_rage_ropeinstance_setnewuniqueid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005210_rage_ropeinstance_attachvirtualboundgeometry_int_unsigned_int_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_005211_rage_ropeinstance_detachvirtualbound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005212_rage_ropeinstance_load_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005218_rage_ropemanager_converttosimple_rage_phinst_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005220_rage_ropemanager_findrope_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005222_rage_ropemanager_removechildrope_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005225_rage_ropemanager_add_rage_vec3v_const_ref_rage_vec3v_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32, a11: f32, a12: i32, a13: i32) -> i64; }
extern "C" { pub fn f_005226_rage_ropemanager_breakrope_rage_ropeinstance_ptr_rage_ropeinstan(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32); }
extern "C" { pub fn f_005315_rage_fwrenderthreadinterface_flush_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005386_rage_fwmapdatastore_requestgroup_rage_strlocalindex_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_005387_rage_fwmapdatastore_removegroup_rage_strlocalindex_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_005397_rage_fwimapgroupswap_start_int_rage_atnonfinalhashstring_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_005398_rage_fwimapgroupmgr_completeswap_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005399_rage_fwimapgroupmgr_abandonswap_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005538_rage_crframefilterdictionarystore_findframefilter_rage_fwmvtempl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005555_rage_fwimapgroupmgr_getisloaded_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005609_rage_fwmapdatastore_createchildrencache(ph: *mut Phantom); }
extern "C" { pub fn f_005733_rage_fwinteriorlocation_getroomindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005917_rage_fwrefawarebaseimpl_rage_datbase_dtorfwrefawarebaseimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005919_rage_aitask_setstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005946_rage_aitasktree_cleartask_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005947_rage_aitasktree_getactiveleaftask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005949_rage_aitasktree_findtaskbytypeactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005950_rage_aitasktree_findtaskbytypewithpriority_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_006084_rage_strstreaminginfomanager_requestobject_rage_strindex_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006125_rage_strrequestrecorder_endrecording(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006167_rage_strstreaminginfomanager_isobjectinimage_rage_strindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006175_rage_strstreaminginfomanager_clearrequiredflag_rage_strindex_uns(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006194_rage_strstreaminginfomanager_clearflagforall_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_006255_rage_strrequest_hasloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006257_rage_strrequest_clearrequiredflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007206_rage_audwaveslot_findwaveslot_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007504_rage_decalmanager_findfirstbucket_unsigned_int_rage_fwentity_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_007675_rage_fwangle_limitdegreeangle_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_007676_rage_fwangle_limitradianangleforpitch_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_007759_rage_netinterface_getnumphysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007762_rage_netinterface_getallphysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007776_rage_netplayermgrbase_isinitialized_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007803_rage_networklogutils_writelogevent_rage_netlogginginterface_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007848_rage_neteventmgr_checkforspaceinpool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007849_rage_neteventmgr_prepareevent_rage_netgameevent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007855_rage_netplayer_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007949_rage_netgameevent_netgameevent_unsigned_short_bool_bool_rage_net(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_008056_rage_networklogutils_writeplayertext_rage_netlogginginterface_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_008074_rage_netobject_ispendingownerchange_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008249_rage_netplayer_hasteam(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008332_rage_netleaderboardread_init_unsigned_int_rage_rlleaderboard2typ(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_008334_rage_netleaderboardread_cancel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008336_netscgamerconfigparser_getleaderboard_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008337_rage_netleaderboardread_getcolumnid_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008338_rage_netleaderboardread_getrow_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008340_netscgamerconfigparser_getinputbyid_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008341_rage_netleaderboardread_allocaterowdata_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008342_rage_netleaderboardread_getcolumntype_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008343_rage_netleaderboardread_getcolumntypename_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008347_rage_netleaderboardreadgamersbyscoreint_start_int_rage_rlleaderb(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_008348_rage_netleaderboardreadgamersbyscorefloat_start_int_rage_rlleade(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: f64) -> i32; }
extern "C" { pub fn f_008351_rage_netleaderboardwrite_addcolumn_rage_rlstatvalue_const_ref_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008352_rage_netleaderboardwrite_columnexists_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008354_rage_netleaderboardmanageallocator_startusingmemallocator(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008355_rage_netleaderboardmanageallocator_stopusingmemallocator(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008362_rage_netleaderboardreadmgr_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008363_rage_netleaderboardreadmgr_addreadbyrow_int_unsigned_int_rage_rl(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i64, a8: i32, a9: i64, a10: i32, a11: i64) -> i64; }
extern "C" { pub fn f_008364_rage_netleaderboardreadmgr_addreadtoreadlist_rage_netleaderboard(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008365_rage_netleaderboardreadmgr_addreadbyrank_int_unsigned_int_rage_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_008366_rage_netleaderboardreadmgr_addreadbyradius_int_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_008367_rage_netleaderboardreadmgr_getleaderboardbytype_unsigned_int_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_008368_rage_netleaderboardreadmgr_canreadstatistics_rage_rlgamerhandle(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008370_rage_netleaderboardreadmgr_countofleaderboards_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008379_rage_netleaderboardwritemgr_addleaderboard_unsigned_int_rage_rll(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_008391_rage_netscgamerdatamgr_ischeatersc_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008395_rage_atarray_rage_netscgamerdatamgr_gamerdatavalue_0_unsigned_sh(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008401_rage_netscgamerdatamgr_getvalue_unsigned_int_bool_ref_const(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_008402_rage_netscgamerdatamgr_getvalue_unsigned_int_char_ptr_unsigned_i(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_008403_rage_netscgamerdatamgr_getvalue_unsigned_int_int_ref_const(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_008404_rage_netscgamerdatamgr_getvalue_unsigned_int_float_ref_const(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_008407_rage_netscgamerdatamgr_getactivexpbonus_float_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008412_netprofanityfilter_verifystringforprofanity_char_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_008413_netprofanityfilter_getstatusforrequest_int_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008416_netlicenseplatecheck_validatelicenseplate_char_const_ptr_rage_rl(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008417_netlicenseplatecheck_getstatusforrequest_int_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008418_netlicenseplatecheck_getcount_int_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008419_netlicenseplatecheck_getplate_int_const_ref_int_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_008420_netlicenseplatecheck_getplatedata_int_const_ref_int_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_008422_netlicenseplateadd_addlicenseplate_int_char_const_ptr_char_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_008423_netlicenseplateadd_getstatusforrequest_int_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008426_netlicenseplateisvalid_isvalidlicenseplate_char_const_ptr_rage_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008427_netlicenseplateisvalid_getstatusforrequest_int_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008429_netlicenseplatechange_changelicenseplate_int_char_const_ptr_char(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_008689_rage_fwscriptguid_createguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008691_rage_fwscriptguid_deleteguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008693_rage_fwscriptguid_getbasefromguid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_008706_rage_scripthandler_getscriptobject_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008710_rage_scripthandler_getscriptresource_unsigned_short_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_008713_rage_scripthandler_getscriptresource_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008714_rage_scripthandler_removeallscriptresourcesoftype_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008715_rage_scripthandler_getresourcereferencefromid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008770_rage_scripthandlermgr_getscripthandlerforresource_unsigned_short(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_008778_rage_scripthandlernetcomponent_ishostlocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008841_rage_scriptinterface_isnetworkscript_rage_scriptidbase_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008945_rage_fwbasepool_forall_bool_ptr_void_ptr_void_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008952_rage_fwbasepool_getat_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008953_rage_fwknownrefholder_fwknownrefholder_void_ptr_ptr_rage_fwknown(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009648_rage_grcviewport_reversetransform_float_float_rage_vec3v_ref_rag(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i64); }
extern "C" { pub fn f_009905_rage_monitorconfiguration_ismultihead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009908_rage_monitorconfiguration_update(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_010500_rage_iovalue_isdown_float_rage_iovalue_readoptions_const_ref_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_011740_rage_fraginst_setdisablebreakable_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011741_rage_fraginst_setdisabledamage_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011742_rage_fraginst_resetdisablebreakable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011743_rage_fraginst_isbreakingdisabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011751_rage_fraginst_getcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011829_rage_fragcacheentry_reloadphysicsdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012843_rage_environmentcloth_getcloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012875_rage_environmentcloth_controlvertex_int_rage_vec3v_const_ref_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_012911_rage_charactercloth_setpose_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013119_rage_bgglassmanager_setsilenthit_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_013429_char_ptr_rage_safecatf_char_char_ptr_unsigned_long_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_013705_rage_nethttprequest_addformparam_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013710_rage_nethttprequest_beginlistformparam_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013711_rage_nethttprequest_addlistformparamvalue_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013867_rage_nethardware_getmacaddress_unsigned_char_ref_6(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014067_rage_nettimesync_gettime_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014068_rage_nettimesync_hasstarted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014259_rage_rltaskmanager_doaddtask_unsigned_long_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014260_rage_rltaskmanager_destroytask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014294_rage_rlgetposixtime_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014295_rage_rlcreateuuid_unsigned_long_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014303_rage_rlgamerhandle_op_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014316_rage_rlgamerinfo_getgamerhandle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014319_rage_rlgamerhandle_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014349_rage_rlgamerhandle_touserid_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014356_rage_rlfriendsmanager_gettotalnumfriends_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014358_rage_rlfriendsmanager_getgamerhandle_int_rage_rlgamerhandle_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_014361_rage_rlros_getcredentials_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014362_rage_rlroscredentials_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014368_rage_rlgamerhandle_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014370_rage_rlgamerhandle_tostring_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014459_rage_rltaskbase_configure_rage_netstatus_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014466_rage_rlroscredentials_getrockstaraccount_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014619_rage_readclansbyscore_int_int_rage_rlleaderboard2groupselector_c(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i32, a8: i64, a9: i64, a10: i64, a11: i64) -> i32; }
extern "C" { pub fn f_014620_rage_readclanmembersbyscore_int_int_rage_rlleaderboard2groupsele(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i64, a8: i32, a9: i64, a10: i64, a11: i64, a12: i64) -> i32; }
extern "C" { pub fn f_014662_rage_rlmetric_dtorrlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014685_rage_rlmetric_rlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014721_rage_zlibstream_iscomplete_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014836_rage_rlroscredentials_hasprivilege_rage_rlrosprivilegeid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014859_rage_rlros_getdefaultprivilege_rage_rlrosprivilegeid(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014874_rage_rlroscredentials_getplayeraccountid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015563_rage_rlleaderboard2commonreadtask_rlleaderboard2commonreadtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015570_rage_rlleaderboard2commonreadtask_configure_int_int_rage_rlleade(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i64, a7: i32, a8: i64, a9: i64, a10: i64) -> i32; }
extern "C" { pub fn f_015771_rage_computeaxisangleprecise_rage_quatv_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015882_rage_phconstraintmgr_insert_rage_phconstraintbase_params_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015891_rage_phconstraintmgr_gettemporarypointer_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015965_rage_phcollider_setcollidermatrixfrominstancerigid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016088_rage_phinst_setinstflag_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016301_rage_pharticulatedcollider_setcollidermatrixfrominstanceart(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016367_rage_phbound_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016370_rage_phbound_setcgoffset_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016391_rage_phboundbox_phboundbox_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016594_rage_phverletcloth_detachvirtualbound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016599_rage_phverletcloth_attachvirtualboundcapsule_float_float_rage_ma(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_016600_rage_phverletcloth_createvirtualbound_int_rage_mat34v_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016606_rage_phverletcloth_dynamicunpinvertex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016607_rage_phverletcloth_dynamicunpinall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016738_rage_phcontactmgr_removeallcontactswithinstance_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016934_rage_scrprogram_getprogram_rage_scrprogramid(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_016944_rage_scrthread_preprintstacktrace(ph: *mut Phantom); }
extern "C" { pub fn f_016949_rage_scrthread_getactivethread(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_016962_rage_scrthread_registerunusedcommand_unsigned_long_void_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016967_rage_scrthread_registercommand_unsigned_long_void_ptr_rage_scrth(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016981_rage_scrthread_getthread_rage_scrthreadid(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_016999_rage_scrthread_killthread_rage_scrthreadid(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_017082_rage_scrthread_info_getvector3_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018834_cgen9exclusiveassets_isgen9exclusivepeddrawable_cped_const_ptr_e(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_018880_cgamelogic_hasteleportplayerfinished(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_018882_cgamelogic_teleportplayer_cped_ptr_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_018885_cgamelogic_isteleportactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_018932_cgarages_findgarageindex_char_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_018935_cgarages_isthiscarwithingaragearea_int_centity_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_019009_cwaypointrecording_getrecording_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019045_cwaypointrecording_requestrecording_char_const_ptr_rage_scrthrea(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019047_cwaypointrecording_removerecording_char_const_ptr_rage_scrthread(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019061_cwaypointrecording_getloadedwaypointrecordinginfo_cwaypointrecor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019089_cwaypointrecording_isrecordingloaded_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019108_cplayerinfo_gettimesinceplayerhitcar(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019109_cplayerinfo_gettimesinceplayerhitped(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019110_cplayerinfo_gettimesinceplayerdroveonpavement(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019111_cplayerinfo_gettimesinceplayerdroveagainsttraffic(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019130_cvehiclerecordingmgr_startplaybackrecordedcar_cvehicle_ptr_int_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_019131_cvehiclerecordingmgr_updateplaybackforvehiclerecording_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_019133_cvehiclerecordingmgr_getplaybackslot_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019135_cvehiclerecordingmgr_stopplaybackrecordedcar_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019136_cvehiclerecordingmgr_isplaybackgoingonforcar_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019137_cvehiclerecordingmgr_getplaybackidforcar_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019139_cvehiclerecordingmgr_setplaybackspeed_cvehicle_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_019140_cvehiclerecordingmgr_pauseplaybackrecordedcar_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019141_cvehiclerecordingmgr_unpauseplaybackrecordedcar_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019143_cvehiclerecordingmgr_skiptoendandstopplaybackrecordedcar_cvehicl(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019145_cvehiclerecordingmgr_skiptimeforwardinrecording_cvehicle_ptr_flo(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_019148_cvehiclerecordingmgr_setadditionalrotationforrecordedvehicle_cve(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019149_cvehiclerecordingmgr_setlocalpositionoffsetforrecordedvehicle_cv(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019150_cvehiclerecordingmgr_setglobalpositionoffsetforrecordedvehicle_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019151_cvehiclerecordingmgr_findpositioninrecording_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_019152_cvehiclerecordingmgr_findtimepositioninrecording_cvehicle_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_019156_cvehiclerecordingmgr_findindexwithfilenamenumber_int_char_const(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_019157_cvehiclerecordingmgr_hasrecordingfilebeenloaded_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_019241_cstuntjumpmanager_isastuntjumpinprogress(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_019288_caitarget_caitarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019289_caitarget_caitarget_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019290_caitarget_setentity_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019291_caitarget_caitarget_centity_const_ptr_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019292_caitarget_setentityandoffset_centity_const_ptr_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019293_caitarget_caitarget_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019294_caitarget_setposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019295_caitarget_caitarget_caitarget_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019297_caitarget_getposition_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019298_caitarget_setentityandoffsetunlimited_centity_const_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019299_caitarget_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019305_cscenarioblockingareas_addscenarioblockingarea_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_019345_cspatialarray_update_cspatialarraynode_ref_float_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_019451_ailogging_getbooleanasstring_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_019453_ailogging_getdynamicentitynamesafe_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019458_ailogging_stacktracelog_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019462_cailogmanager_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019599_cfacialdatacomponent_resetfacialidleanimation_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019600_cfacialdatacomponent_setfacialidleclipid_cped_ptr_rage_fwmvtempl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019602_cfacialdatacomponent_setfacialclipsettodefault_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019603_cfacialdatacomponent_setfacialidleanimoverride_cped_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_019606_cfacialdatacomponent_clearfacialidleanimoverride_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019607_cfacialdatacomponent_onfacialidlechanged_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019609_cfacialdatacomponent_playfacialanim_cped_ptr_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019610_cmoveped_playoneshotfacialclip_rage_crclip_const_ptr_float_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_019667_cmoveped_switchtoanimated_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_019844_rage_matrix34_inverse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020636_ctaskscriptedanimation_setrate_float_ctaskscriptedanimation_epla(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_020637_ctaskscriptedanimation_getblendphaseid_ctaskscriptedanimation_ep(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021399_audpedaudioentity_setcachedbjvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021758_audscannermanager_cancelanyplayingreportsnextfade(ph: *mut Phantom); }
extern "C" { pub fn f_021933_bool_creplaymgrinternal_recordfx_cpacketscriptedspeech_cpacketsc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022381_void_creplayeventmanager_storeevent_cpacketsoundphonering_cpacke(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022658_cped_isdead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022955_audheliaudioentity_sethelishouldskipstartup_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023396_cpacketevent_cpacketevent_ereplaypacketid_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023415_creplayropemanager_attachentites_int_centity_ptr_centity_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_023416_creplayropemanager_getattachedentitya_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_023417_creplayropemanager_getattachedentityb_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_023418_cpacketaddrope_cpacketaddrope_int_rage_vec3v_const_ref_rage_vec3(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32, a7: f32, a8: i32, a9: i32, a10: i32, a11: i32, a12: f32, a13: i32, a14: i32) -> i64; }
extern "C" { pub fn f_023420_cpacketdeleterope_cpacketdeleterope_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_023423_cpacketattachropetoentity_cpacketattachropetoentity_int_rage_vec(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_023425_cpacketattachentitiestorope_cpacketattachentitiestorope_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32, a7: f32, a8: f32, a9: i64, a10: i64, a11: i32, a12: i32) -> i64; }
extern "C" { pub fn f_023428_cpacketdetachropefromentity_cpacketdetachropefromentity_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_023434_cpacketunpinrope_cpacketunpinrope_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_023437_cpacketropewinding_cpacketropewinding_float_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_023534_cpacketscriptedspeech_cpacketscriptedspeech_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i32) -> i64; }
extern "C" { pub fn f_023829_creplaycontrol_shutdownsession_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_023832_creplaycontrol_setstoprecording(ph: *mut Phantom); }
extern "C" { pub fn f_023834_creplaycontrol_setcancelrecording(ph: *mut Phantom); }
extern "C" { pub fn f_023885_creplaymgr_recordwarpedentity_cdynamicentity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023897_creplaycontrol_setsaverecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023905_creplaycontrol_setstartrecording_replay_start_param(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_023906_creplaycontrol_isplayeroutofcontrol(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023907_creplaycontrol_isreplayavailable(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023909_creplaycontrol_isreplayrecordspaceavailable_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_023927_void_creplayeventmanager_trackrecordingevent_unsigned_long_ctrac(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_023930_creplayeventmanager_shouldproceedwithpacket_cpacketevent_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023945_replaypedextension_clearpeddecorationdata_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023967_creplayinterfacemanager_ensureisbeingrecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023968_creplayinterfacemanager_ensurewillberecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023998_rage_fwmodelid_fwmodelid_rage_fwmodelid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_024266_creplaymgrinternal_isawareofworldforrecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024287_creplaymgrinternal_pausetheclearworldtimeout_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_024304_creplaymgrinternal_getbuildinginfo_cphysical_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_024305_creplaymgrinternal_recordmapobject_centity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024324_replayrecordingscriptinterface_preventrecordingthisframe(ph: *mut Phantom); }
extern "C" { pub fn f_024325_replayrecordingscriptinterface_disablecameramovementthisframe(ph: *mut Phantom); }
extern "C" { pub fn f_026205_cqueriableinterface_istaskcurrentlyrunning_int_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_026259_rage_fwentity_getskeleton_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026279_rage_matrix34_toeulersyxz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026610_cambasedirector_getinterpolationtime_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026702_ccontrol_debugoutputscriptcallstack(ph: *mut Phantom); }
extern "C" { pub fn f_027008_ctrain_getlinkedtobackward_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027009_ctrain_getlinkedtoforward_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029377_ctask_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029386_cpedvariationinfocollection_getcollectiondrawable_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_029433_getorcreategroup_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029455_cdebugscene_tunewidgetdata_tunewidgetdata_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_029456_cdebugscene_queuetunewidget_cdebugscene_tunewidgetdata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029692_cdebugtextureviewerinterface_close_bool(ph: *mut Phantom); }
extern "C" { pub fn f_029870_cpedintelligence_addtasksecondary_rage_aitask_ptr_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030722_ceventgroupped_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030740_geteventscriptaigroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030741_geteventglobalgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030742_geteventscripterrorsgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030746_geteventscriptnetworkgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030761_cpedintelligence_gettaskactive_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030762_rage_fweventqueue_16_void_remove_rage_fwevent_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030763_ceventhandler_getcurrenteventtype_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030770_cshockingeventsmanager_add_ceventshocking_ref_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_030772_ceventshocking_dtorceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030924_ceventnetworkscriptevent_ceventnetworkscriptevent_void_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_030945_svehiclemissionparams_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_031069_ceventgivepedtask_ceventgivepedtask_int_rage_aitask_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_031070_ceventgivepedtask_dtorceventgivepedtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031169_ceventswitch2nm_ceventswitch2nm_unsigned_int_rage_aitask_ptr_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_031170_ceventswitch2nm_dtorceventswitch2nm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031205_ceventscenarioforceaction_ceventscenarioforceaction(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031206_ceventscenarioforceaction_dtorceventscenarioforceaction(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031208_ceventscenarioforceaction_settask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_031235_ceventshocking_ceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031242_ceventshocking_set_rage_vec3v_const_ref_centity_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_031400_svehiclemissionparams_settargetentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034930_cloadingscreens_committompsp_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_034944_cloadingscreens_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_034950_chudtools_getaspectratio_bool(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_035031_cnewhud_handlecharacterchange(ph: *mut Phantom); }
extern "C" { pub fn f_035087_chudtools_issuperwidescreen_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035117_cphonemgr_delete(ph: *mut Phantom); }
extern "C" { pub fn f_035118_cphonemgr_camactivate_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035120_cphonemgr_getphonemodelindex(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_035122_cphonemgr_loadmodel(ph: *mut Phantom); }
extern "C" { pub fn f_035128_cphonemgr_camischarvisible_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_035129_cphonemgr_camactivateselfiemode_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035130_cphonemgr_camactivateshallowdofmode_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035131_cphonemgr_camsetselfiemodesideoffsetscaling_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_035132_cphonemgr_horizontalactivate_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035133_cphonemgr_setlatestphoneinput_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035134_cphonemgr_camsetselfiemodehorzpanoffset_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_035135_cphonemgr_camsetselfiemodevertpanoffset_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_035136_cphonemgr_camsetselfieroll_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_035137_cphonemgr_camsetselfiedistance_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_035138_cphonemgr_camsetselfieheadyaw_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_035139_cphonemgr_camsetselfieheadroll_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_035140_cphonemgr_camsetselfieheadpitch_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_036170_cpausemenu_setmenupreference_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036175_cprofilesettings_set_cprofilesettings_profilesettingid_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036281_cuireticule_reset(ph: *mut Phantom); }
extern "C" { pub fn f_036364_cprofilesettings_getint_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036382_cprofilesettings_exists_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036386_cprofilesettings_write_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036603_cvideoeditorui_open_eopenedfromsource(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037656_cwanted_setmaximumwantedlevel_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037657_cwanted_setwantedlevel_rage_vector3_const_ref_int_int_setwantedl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_037659_cseatmanager_countpedsinseats_bool_bool_bool_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_037662_cmodelinfo_haveassetsloaded_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037761_ccrime_reportcrime_ecrimetype_centity_ptr_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_037767_cwanted_setwantedlevelnodrop_rage_vector3_const_ref_int_int_setw(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_037768_cwanted_reportcrimenow_ecrimetype_rage_vector3_const_ref_bool_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32); }
extern "C" { pub fn f_037770_cdispatchdata_getwantedlevelthreshold_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037872_cwanted_overrides_setmultiplayerhiddenevasiontimesoverride_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_037873_cwanted_overrides_resetdifficulty(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037874_cwanted_overrides_resetevasiontimearray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037875_cwanted_overrides_setdifficulty_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_037881_cwanted_clearqdcrimes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037884_cwanted_reportpolicespottingplayer_cped_ptr_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_037888_cseatmanager_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_037889_cwanted_update_rage_vector3_const_ref_cvehicle_const_ptr_bool_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_037895_cdispatchdata_getwantedzoneradius_int_cped_const_ptr(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_037901_cwanted_copsaresearching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037906_cwanted_forcestarthiddenevasion(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038130_cmodelinfo_getbasemodelinfo_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038413_bgscriptinfo_launchparam_getvaluefromdata_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038429_backgroundscripts_startcontext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038442_backgroundscripts_startcontext_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038443_backgroundscripts_endcontext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038444_backgroundscripts_endcontext_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038445_backgroundscripts_getscriptinfo_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038446_backgroundscripts_getscriptidfromhash_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038495_cikmanager_lookat_unsigned_int_centity_const_ptr_int_eanimboneta(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32); }
extern "C" { pub fn f_038497_cikmanager_islooking(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038499_cikmanager_abortlookat_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038501_cikmanager_getlookatentity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038505_cikmanager_request_cikrequestbodylook_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038507_cikmanager_setrelativearmiktarget_rage_criksolverarms_earms_cdyn(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32, a6: f32, a7: f32, a8: f32, a9: f32); }
extern "C" { pub fn f_038721_cmodelinfo_getobjectanddependencies_rage_fwmodelid_rage_atarray(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_038725_cvehiclemodelinfo_calculatemaxflatvelocityestimate_chandlingdata(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038732_centryexitpoint_canseatbereached_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038739_cmodelseatinfo_getentrypointindexforseat_int_centity_const_ptr_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_038743_cmodelseatinfo_getshuffleseatforseat_int_int_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_038751_cseatmanager_getpedsseatindex_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038757_cseatmanager_getlastpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038837_cpedmodelinfo_findpersonalitymovementmodes_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_038982_cvehiclevariationinstance_istogglemodon_evehiclemodtype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039232_cvehiclevariationinstance_getmodifier_evehiclemodtype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039630_cweapon_sfireparams_dtorsfireparams(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039644_cobject_getpointertoclosestmapobjectforscript_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_039719_cobject_settask_rage_aitask_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_039945_cpathserveropponentpedgen_startsearch_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32, a4: f32, a5: i32); }
extern "C" { pub fn f_039947_cpathserveropponentpedgen_startsearch_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: f32, a5: i32); }
extern "C" { pub fn f_039949_cpathserveropponentpedgen_getnumresults_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_039950_cpathserveropponentpedgen_getresult_int_rage_vector3_ref_const(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_039951_cpathserveropponentpedgen_cancelseach(ph: *mut Phantom); }
extern "C" { pub fn f_040044_cpathserveropponentpedgen_issearchactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040045_cpathserveropponentpedgen_issearchcomplete_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_040046_cpathserveropponentpedgen_getresultflags_int_unsigned_int_ref_co(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_040139_cpedgroupmembership_getmember_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_040140_mapscriptableformationtopedformation_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_040141_cpedformation_setspacing_float_float_float_rage_scrthreadid(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_040150_cpedgroupmembership_countmembersexcludingleader_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040175_cpedgroupmembership_isleader_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040180_cpedgroup_setformation_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_040194_cpedgroups_getpedsgroup_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040201_carcadepassiveabilityeffects_getisactive_earcadeabilityeffecttyp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_040202_carcadepassiveabilityeffects_setisactive_earcadeabilityeffecttyp(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_040203_carcadepassiveabilityeffects_getmodifier_earcadeabilityeffecttyp(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_040205_carea_dtorcarea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040207_carea_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040210_carea_set_rage_vector3_const_ref_rage_vector3_const_ref_float_ce(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32); }
extern "C" { pub fn f_040211_carea_setassphere_rage_vector3_const_ref_float_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_040213_carea_getcentre_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040218_cassistedmovementroutestore_process_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_040227_cassistedmovementtoggles_turnonthisroute_unsigned_int_rage_scrth(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_040228_cassistedmovementtoggles_turnoffthisroute_unsigned_int_rage_scrt(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_040229_cassistedmovementroute_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040236_cassistedmovementroutestore_calcscriptedroutehash_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_040323_cactionmanager_findactionresult_unsigned_int_ref_unsigned_int_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_040384_ccombatbehaviour_initfromcombatdata_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040398_cdefensivearea_cdefensivearea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040402_cdefensiveareamanager_getsecondarydefensivearea_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_040404_rage_fraginstnm_postartmessage_char_const_ptr_art_messageparamsb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040409_art_messageparamsbase_addbool_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040410_art_messageparamsbase_addfloat_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_040500_art_messageparamsbase_addint_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040501_art_messageparamsbase_addvector3_char_const_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_040502_art_messageparamsbase_addstring_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040503_art_messageparamsbase_messageparamsbase_art_messageparamsbase_pa(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041056_findplayerped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041062_cpedintelligence_findmovementtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041063_cpedintelligence_isfriendlywith_cped_const_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041064_cpedintelligence_isthreatenedby_cped_const_ref_bool_bool_bool_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_041088_cpedintelligence_getnearbyvehicles(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041089_cplayerinfo_getstealthmultiplier_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041091_cpedperception_computefovvisibility_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32) -> i32; }
extern "C" { pub fn f_041116_cpedintelligence_flushimmediately_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041117_cpedintelligence_addtoeventhistory_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041195_cpedintelligence_cleartasks_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041199_cplayerpedtargeting_addtargetableentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041203_cpedintelligence_getrelationshipgroup_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041205_cpedpopulation_generatescenariopedsinspecificarea_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_041235_cplayerinfo_areanycontrolsotherthanfrontenddisabled(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041237_cpedintelligence_gettaskmelee_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041244_cplayerspecialabilitymanager_chargeevent_abilitychargeeventtype(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_041247_cplayerspecialability_addtometercontinuous_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_041248_cplayerspecialability_addtometernormalized_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_041269_cpedintelligence_gettaskclimbladder_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041288_cpedintelligence_getactivepropmanagementhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041293_cplayerspecialability_deactivate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041298_cpedhelmetcomponent_disablehelmet_bool_rage_vector3_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_041312_cpedfootstephelper_forcesteptype_bool_audfootstepevent_auddetect(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_041335_cpedintelligence_ispedswimming_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041337_cpedhelmetcomponent_ishelmetenabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041339_cpedhelmetcomponent_enablehelmet_eanchorpoints_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_041341_cpedhelmetcomponent_gethelmetpropindex_epedcompflags_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041344_cpedhelmetcomponent_setstoredhatindices_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041346_cpedhelmetcomponent_sethelmettexindex_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041347_cpedhelmetcomponent_sethelmetpropindex_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041348_cpedhelmetcomponent_setvisorpropindices_bool_bool_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041354_cpedhelmetcomponent_addvehiclehelmetassociation_unsigned_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_041355_cpedhelmetcomponent_clearvehiclehelmetassociation_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041356_cpedhelmetcomponent_clearallvehiclehelmetassociations(ph: *mut Phantom); }
extern "C" { pub fn f_041377_cpedintelligence_setdecisionmakerid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041378_cpedintelligence_isrespondingtoevent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041382_cpedintelligence_registerthreat_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041385_cpedintelligence_findtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041387_cpedintelligence_gettaskmeleeactionresult_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041388_cpedintelligence_ispedinair_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041389_cpedintelligence_ispedclimbing_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041390_cpedintelligence_ispedvaulting_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041391_cpedintelligence_ispedfalling_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041392_cpedintelligence_ispedlanding_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041393_cpedintelligence_ispeddiving_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041394_cpedintelligence_ispedgettingup_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041398_cpedintelligence_clearprimarytaskatpriority_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041403_cpedintelligence_setrelationshipgroupdefault_crelationshipgroup(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041424_cpedintelligence_resetbattleawareness(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041426_cpedintelligence_isbattleaware_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041428_cpedintelligence_getlastbattleeventtime_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041436_cpedintelligence_forcepedpinneddown_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_041451_cpedintelligence_eventhistoryoneventremoved_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041467_cpedpopulation_choosecivilianpedmodelindexargs_choosecivilianped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041471_cpedpopulation_setscenariopeddensitymultipliers_float_float(ph: *mut Phantom, a0: f32, a1: f32); }
extern "C" { pub fn f_041472_cpedpopulation_pednonremovalareaclear(ph: *mut Phantom); }
extern "C" { pub fn f_041473_cpedpopulation_pednoncreationareaclear(ph: *mut Phantom); }
extern "C" { pub fn f_041474_cpedpopulation_setallowcreaterandomcops_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041475_cpedpopulation_setallowcreaterandomcopsonscenarios_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041476_cpedpopulation_setallowdummyconversions_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041488_cpedpopulation_usetemppopcontrolspherethisframe_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_041513_cpedpopulation_setambientpeddensitymultiplier_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_041514_cpedpopulation_setscriptambientpeddensitymultiplierthisframe_flo(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_041516_cpedpopulation_setscriptscenariopeddensitymultipliersthisframe_f(ph: *mut Phantom, a0: f32, a1: f32); }
extern "C" { pub fn f_041517_cpedpopulation_setsuppressambientpedaggressivecleanupthisframe(ph: *mut Phantom); }
extern "C" { pub fn f_041539_cpedpopulation_choosecivilianpedmodelindex_cpedpopulation_choose(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041545_cpedpopulation_iseffectinuse_cscenariopoint_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041548_cpedpopulation_getscenariopointinareaargs_getscenariopointinarea(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_041550_cpedpopulation_getscenariopointinarea_cpedpopulation_getscenario(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041551_cpedpopulation_scriptedconversioncentresetthisframe_rage_vector3(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041552_cpedpopulation_pednoncreationareaset_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041557_cpedpopulation_getcopsinarea_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_041558_cpedpopulation_instantfillpopulation(ph: *mut Phantom); }
extern "C" { pub fn f_041573_cmovementmodescriptresource_requestanimsfromclipset_rage_fwmvtem(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041575_cmovementmodescriptresourcemanager_getreference_rage_atnonfinalh(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041576_cplayerarcadeinformation_setteamandrole_earcadeteam_earcaderole(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041585_cplayerspecialability_updatefromstat(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041591_cplayerspecialability_canbeactivated_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041592_cplayerspecialability_activate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041594_cplayerspecialability_deactivatenofadeout(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041595_cplayerspecialability_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041596_cplayerspecialability_ismeterfull_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041597_cplayerspecialability_fillmeter_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041598_cplayerspecialability_depletemeter_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041599_cplayerspecialability_addtometer_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041601_cplayerspecialability_getremainingpercentagefordisplay_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041603_cplayerspecialability_stopfx_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041604_cplayerspecialabilitymanager_isabilityunlocked_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041608_cplayerspecialabilitymanager_setabilitystatus_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_041627_cpedscriptedtaskrecord_getstatus_cped_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041663_cplayerinfo_setmayonlyenterthisvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041685_cplayerinfo_computesprintdepletionrateforped_cped_const_ref_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> f32; }
extern "C" { pub fn f_041692_cplayerinfo_evaluatecarposition_centity_ptr_cped_ptr_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64, a6: i64, a7: i32); }
extern "C" { pub fn f_041696_cplayerinfo_hasplayerlefttheworld_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041697_cplayerinfo_extendworldboundary_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041706_cplayerinfo_setplayercontrol_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_041707_cplayerinfo_resetsprintenergy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041708_cplayerinfo_setplayercontrol_bool_bool_float_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_041711_cplayerinfo_restoresprintenergy_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_041714_cplayerinfo_isplayerpressinghorn_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041720_ccontrol_getvehicletargetdown_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_041724_cplayerinfo_canplayerstartmission(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041730_cplayerinfo_setsimulategaitinput_float_float_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_041737_cplayerinfo_setpreviousvariationdata_epedvarcomp_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_041738_cplayerinfo_setpedparachutevariationoverride_epedvarcomp_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041739_cplayerinfo_clearpedparachutevariationoverride(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041741_cplayerinfo_getmpspecialability_eplayerspecialabilityslot_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041742_cplayerinfo_setmpspecialability_specialabilitytype_eplayerspecia(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041746_reportplayerresetmismatch_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_041892_cpedtargetting_setinuse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041934_cplayerpedtargeting_removetargetableentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042002_cqueriableinterface_istaskpresentatpriority_int_unsigned_int_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042004_cqueriableinterface_getstatefortasktype_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042025_crelationshipgroup_setrelationship_erelationtype_crelationshipgr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_042026_crelationshipgroup_clearrelationship_crelationshipgroup_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042040_crelationshipmanager_findrelationshipgroup_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_042131_cpeddamagemanager_addpedscar_cped_ptr_epeddamagezones_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: f32, a6: i64, a7: i32, a8: i32, a9: i32, a10: f32, a11: i32); }
extern "C" { pub fn f_042149_cpeddamagemanager_addpeddamagedecal_cped_ptr_epeddamagezones_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: f32, a6: i64, a7: i32, a8: i32, a9: f32, a10: f32, a11: i32); }
extern "C" { pub fn f_042168_cpeddamagemanager_addblooddamage_unsigned_char_unsigned_short_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_042169_cpeddamagemanager_getbloodinfoanddamageid_cped_ptr_rage_atnonfin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_042171_cpeddamagemanager_addpedblood_cped_ptr_unsigned_short_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_042172_cpeddamagemanager_addpedblood_cped_ptr_epeddamagezones_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_042173_cpeddamagemanager_addpedblood_cped_ptr_epeddamagezones_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: f32, a6: i64, a7: i32, a8: f32, a9: i32, a10: i32, a11: i32) -> i32; }
extern "C" { pub fn f_042174_cpeddamagemanager_addpedstab_cped_ptr_epeddamagezones_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_042175_cpeddamagemanager_addpeddecoration_cped_ptr_rage_atnonfinalhashs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: f32, a7: i64, a8: i32, a9: i64, a10: i64, a11: i32, a12: f32, a13: i32, a14: i32, a15: i64, a16: f32, a17: f32, a18: f32, a19: i32, a20: i32); }
extern "C" { pub fn f_042179_cpeddamagemanager_addpeddamagepack_cped_ptr_rage_atnonfinalhashs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_042183_cpeddamagemanager_getdamageset_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_042193_cpeddamagemanager_cleardamagedecals_unsigned_char_epeddamagezone(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_042195_cpeddamagemanager_clearpeddecorations_unsigned_char_bool_bool_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_042196_cpeddamagemanager_clearcompressedpeddecorations_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042232_pedheadshotmanager_registerped_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_042234_pedheadshotmanager_unregisterped_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042235_pedheadshotmanager_gettexturename_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042236_pedheadshotmanager_isregistered_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042237_pedheadshotmanager_isactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042238_pedheadshotmanager_requesttextureupload_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_042285_pedheadshotmanager_releasetextureuploadrequest_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042404_cpedvariationpack_getlocalcompdata_cped_const_ptr_epedvarcomp_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_042436_cpedvariationpack_getpalettevar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042437_cpedvariationpack_gettexvar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042438_cpedvariationpack_getcompvar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042446_cpedpropsmgr_setpedprop_cped_ptr_eanchorpoints_int_int_eanchorpo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_042489_cpedpropsmgr_getpedpropidx_cped_const_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042606_cpedpropinfo_getmaxnumprops_eanchorpoints_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042618_cpedpropsmgr_knockoffprops_cped_ptr_bool_bool_bool_bool_rage_vec(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_042631_cpedpropsmgr_getpedproptexidx_cped_const_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042632_cpedpropsmgr_getpropidxwithflags_cped_ptr_eanchorpoints_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042639_cpedpropsmgr_clearpedprop_cped_ptr_eanchorpoints_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042640_cpedpropsmgr_ispropvalid_cped_ptr_eanchorpoints_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_042641_cpedpropsmgr_getmaxnumpropstex_cdynamicentity_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_042642_cpedpropsmgr_setpreloadprop_cped_ptr_eanchorpoints_int_int_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_042643_cpedpropsmgr_haspreloadpropsfinished_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042644_cpedpropsmgr_cleanuppreloadprops_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042645_cpedpropsmgr_clearallpedprops_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042647_cpedpropsmgr_doespedhaveproporcomponentwithflagset_cped_const_re(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042686_peddecorationmanager_addpeddecoration_cped_ptr_rage_atnonfinalha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32); }
extern "C" { pub fn f_042697_peddecorationmanager_getpeddecorationzone_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_042814_worldprobe_debugfindgroundzfor3dcoord_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> f32; }
extern "C" { pub fn f_042818_worldprobe_cshapetestmanager_debugsubmittest_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_042848_worldprobe_cshapetestdesc_setexcludeentities_centity_const_ptr_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_043085_worldprobe_cshapetestdesc_setexcludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043150_fraginstnmgta_setactiveposefromclip_rage_crclip_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_043211_cphysics_breakglass_cdynamicentity_ptr_rage_vec3v_const_ref_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: f32, a5: i32) -> i32; }
extern "C" { pub fn f_043234_worldprobe_cshapetestdesc_setincludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043293_cragdollconstraintcomponent_updatehooktoworldconstraint_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_043294_cragdollconstraintcomponent_enablehandcuffs_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_043295_cragdollconstraintcomponent_enableboundankles_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_043335_worldprobe_debugfindgroundzforcoord_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_043422_chorizonobjects_culleverything_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043464_meshblendmanager_getrampcolor_eramptype_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_043677_animpostfxmanager_start_rage_atnonfinalhashstring_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_044104_water_requestglobalwaterxmlfile_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_045763_csavemigration_multiplayer_requestaccounts(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045764_csavemigration_multiplayer_getaccounts_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045765_csavemigration_multiplayer_requeststatus(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045794_cped_checksexinessflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045796_cped_computedefaulttask_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045797_cped_getcurrentmotiontask_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045806_cped_clearwetclothing(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045815_cped_releasecoverpoint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045819_cped_setactivateragdolloncollision_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045830_cped_ispedvisible(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045834_cped_switchtoanimated_bool_bool_bool_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_045840_cped_getbonematrix_rage_matrix34_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045842_cped_forcemotionstatethisframe_cpedmotionstates_emotionstate_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_045843_cped_setclothisprone_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045861_cped_updateragdollroottransformfromanimatedskel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045863_cped_setpitch_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_045876_cped_setuseextractedz_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045881_cped_setragdollstate_eragdollstate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045883_cped_setpedoutofvehicle_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_045885_cped_switchtoragdoll_cevent_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045886_cped_setmyvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045889_cped_getprimarymotiontask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045891_cped_applyragdollblockingflags_eragdollblockingflagsbitset_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045892_cped_clearragdollblockingflags_eragdollblockingflagsbitset_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045903_cped_getiscrouching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045904_cped_setiscrouching_bool_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_045908_cped_isplayingagestureanim_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045910_cped_isallowedtoforcemotionstate_cpedmotionstates_emotionstate_c(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_045911_cped_wantstouseactionmode_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045912_cped_isusingactionmode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045913_cped_isfirstpersonshootermodeenabledforplayer_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045941_cped_isvariationvalid_epedvarcomp_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_045946_cped_setheadoverlay_eheadoverlayslots_unsigned_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_045948_cped_updatechildblend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045950_cped_setheadblendpalettecolor_unsigned_char_unsigned_char_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045951_cped_setheadoverlaytintindex_eheadoverlayslots_eramptype_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_045952_cped_micromorph_emicromorphtype_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_045953_cped_setheadblendeyecolor_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045954_cped_sethairtintindex_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_045955_cped_setheadblendpalettecolordisabled(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045957_cped_getheadblendeyecolor(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045958_cped_getheadoverlay_eheadoverlayslots(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045959_cped_setnewheadblendvalues_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_045960_cped_hasheadblendfinished(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045962_cped_setpreloaddata_epedvarcomp_unsigned_int_unsigned_int_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_045963_cped_haspreloaddatafinished(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045964_cped_cleanuppreloaddata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045970_cped_getscenariotype_cdynamicentity_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045977_cped_setdesiredheading_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_045982_cped_stopallmotion_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045996_cped_getbonetagfromragdollcomponent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046011_cped_setwetclothingheight_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046013_cped_setwetclothinglevel_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046016_cped_getisshelteredfromrain(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046020_cped_setstubblegrowth_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046024_cped_updatemovementmode_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046054_cped_setusingactionmode_bool_cped_actionmodeenabled_float_bool_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_046055_cped_setmovementmodeoverridehash_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046057_cped_updateragdollboundsfromanimatedskel_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046063_cped_cleardamage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046064_cped_cleardamage_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046065_cped_hideblooddamage_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046067_cped_cleardecorations(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046103_cped_getspecialability_eplayerspecialabilityslot(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046105_cped_getblockingofnontemporaryeventsthisframe_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046109_cped_setarreststate_earreststate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046110_cped_isvalidspecialabilityslot_eplayerspecialabilityslot(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_046121_cped_gethasjetpackequipped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046127_cped_setincustody_bool_cped_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_046137_cped_getpedpixelcount(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046138_cped_haveallstreamingreqscompleted(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046160_cped_setclothpackageindex_unsigned_char_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046161_cped_queueclothposeindex_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046162_cped_setclothforcepin_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046163_cped_setclothlockcounter_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046165_cped_setclothforceskin_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046168_cped_isprone_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046169_cped_willattackinjuredpeds_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046173_cped_getmaxtimeunderwater_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_046174_cped_setmaxtimeunderwater_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046176_cped_getcontrolfromplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046186_cped_resurrect_rage_vector3_const_ref_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_046187_cped_setendurance_float_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046191_cped_setmaxendurance_float_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046192_cped_isdebuginvincible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046204_cpedvfx_resetoutofwaterbonetimers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046208_cped_addpedthatcangetwetthisframe_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046327_cplayerswitchinterface_isactive_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_046410_centity_getinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046438_cgameworld_add_centity_ptr_rage_fwinteriorlocation_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046442_cinteriorproxy_getfromlocation_rage_fwinteriorlocation_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046445_cportaltracker_collectstack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046459_cdynamicentity_getscriptthatcreatedme_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046499_centity_getcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046545_centity_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046599_centityiterator_centityiterator_int_centity_const_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_046600_centityiterator_getnext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047012_cfocusentitymgr_getpos(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047013_cfocusentitymgr_getentity_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047014_cfocusentitymgr_getvel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047020_cgameworld_findlocalplayerwanted(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047025_cloadscene_stop(ph: *mut Phantom); }
extern "C" { pub fn f_047028_cloadscene_startfromscript_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_047030_cloadscene_isloaded(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_047031_cstreamvolumemgr_deregister_int(ph: *mut Phantom); }
extern "C" { pub fn f_047036_cstreamvolumemgr_registerfrustum_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_047037_cstreamvolumemgr_registersphere_rage_spdsphere_const_ref_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_047060_cphysical_activatephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047073_cphysical_sethealth_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047099_cphysical_setdamping_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_047118_cphysical_debugattachtophysicalusingphysics_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64, a10: i64, a11: f32, a12: i32, a13: f32, a14: f32) -> i32; }
extern "C" { pub fn f_047174_csceneloader_updateloadscene(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_047305_cgameworld_getmainplayerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047306_cwarpmanager_stopwarp(ph: *mut Phantom); }
extern "C" { pub fn f_047355_csceneloader_loadscene_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047374_cgameworld_changeplayerped_cped_ref_cped_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_047400_ccompentity_updatecompentitiesusinggroup_int_ecompentitystate(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_047412_clodscale_setglobalscalefromscript_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_047415_clodscale_setremappingfromscript_float_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_047656_cplayerswitchinterface_start_int_cped_ref_cped_ref_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_047657_cplayerswitchmgrlong_setestablishingshot_rage_atnonfinalhashstri(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047658_cplayerswitchinterface_setdest_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047659_cplayerswitchinterface_stop(ph: *mut Phantom); }
extern "C" { pub fn f_047660_cplayerswitchmgrbase_setcontrolflag_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_047679_cplayerswitchinterface_getidealswitchtype_rage_vec3v_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_047684_cstreamvolumemgr_registerline_rage_vec3v_const_ref_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_047685_cplayerswitchmgrlong_streamer_isfinishedstreaming(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047692_cplayerswitchmgrlong_overrideoutroscene_rage_mat34v_const_ref_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_047694_cplayerswitchmgrlong_readyfordescent(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_047695_cplayerswitchmgrlong_enablepausebeforedescent(ph: *mut Phantom); }
extern "C" { pub fn f_047696_cplayerswitchmgrlong_isskippingdescent_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_047734_cinteriorinst_findroombyhashcode_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047735_cinteriorinst_requestroom_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_047880_cgameworld_findlocalplayercoors(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047919_ciplcullbox_setboxenabled_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_047920_ciplcullbox_isboxenabled_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_047921_ciplcullbox_generateculllistforbox_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047947_cstreamvolume_setscriptinfo_rage_scrthreadid_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_047948_cstreamvolume_hasloaded_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_047992_cgameworld_stopalllawenforcersintheirtracks(ph: *mut Phantom); }
extern "C" { pub fn f_047995_cgameworld_changepedmodel_cped_ref_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_048010_cgameworldheightmap_enableheightmap_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_050004_entity_commands_findscriptedanimtask_int_bool_bool_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_050005_entity_commands_findsynchronizedscenetask_int_char_const_ptr_cha(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_050013_ctaskscriptedanimation_findfirstinstanceofclip_rage_atnonfinalha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_050109_ctaskscriptedanimation_stopplayback_ctaskscriptedanimation_eplay(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_050603_graphics_commands_drawspritetex_rage_grctexture_ptr_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i64, a13: i64, a14: i32, a15: i32); }
extern "C" { pub fn f_054827_rage_atarray_rage_rlgamerhandle_0_unsigned_short_grow_unsigned_i(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_057110_ped_commands_commandcreateped_int_int_rage_scrvector_const_ref_f(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: f32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_057111_ped_commands_commandclonepedalt_int_bool_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_057113_ped_commands_commandclonepedtotargetalt_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_057115_ped_commands_commanddeleteped_int_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_057116_ped_commands_commandispedinvehicle_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_057117_ped_commands_commandispedinmodel_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057118_ped_commands_commandispedinanyvehicle_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057120_ped_commands_commandsetpedvehicleforcedseatusage_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_057121_ped_commands_commandclearallpedvehicleforcedseatusage_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_057123_ped_commands_commandispedaimingfromcover_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057129_ped_commands_commandisconversationpeddead_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057131_ped_commands_commandforceallheadingvaluestoalign_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_057133_ped_commands_commandispedfacingped_int_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_057135_ped_commands_commandcreatepedinsidevehicle_int_int_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_057136_ped_commands_commandsetpedmovementclipset_int_char_const_ptr_flo(ph: *mut Phantom, a0: i32, a1: i64, a2: f32); }
extern "C" { pub fn f_057138_ped_commands_commandsetpedstrafeclipset_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_057140_ped_commands_commandsetpedweaponmovementclipset_int_char_const_p(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_057142_ped_commands_commandsetpeddrivebyclipsetoverride_int_char_const(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_057144_ped_commands_commandsetpedmotionincoverclipsetoverride_int_char(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_057149_ped_commands_commandisscriptedscenariopedusingconditionalanim_in(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_057150_ped_commands_commandsetpedalternatewalkanim_int_char_const_ptr_c(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_057152_ped_commands_commandsetpedalternatemovementanim_int_int_char_con(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: f32, a5: i32); }
extern "C" { pub fn f_057156_ped_commands_commandispedshootinginarea_int_rage_scrvector_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_057157_ped_commands_commandisanypedshootinginarea_rage_scrvector_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_057163_ped_commands_commandexplodepedhead_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057164_ped_commands_commandremovepedelegantly_int_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_057165_ped_commands_commandaddarmourtoped_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057166_ped_commands_commandsetpedarmour_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057173_ped_commands_createrandomped_rage_vector3_const_ref_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_057176_ped_commands_commandcreaterandompedasdriver_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057184_ped_commands_commandgetvehiclepedisin_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057197_ped_commands_commandsetblockingofnontemporaryeventsforambientped(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_057200_ped_commands_commandispedonspecificvehicle_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057201_ped_commands_commandsetpedintovehicle_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_057205_ped_commands_commandispedsittinginvehicle_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057206_ped_commands_commandispedsittinginanyvehicle_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057207_ped_commands_commandispedonfoot_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057210_ped_commands_commandgetdeadpedpickupcoords_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32); }
extern "C" { pub fn f_057246_ped_commands_commandispedjumpingoutofvehicle_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057247_ped_commands_commandispedopeningdoor_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057248_ped_commands_commandispedinparachutefreefall_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057249_ped_commands_commandgetpedparachutestate_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057260_ped_commands_commandispedinanytaxi_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057275_ped_commands_commandsetpedstealthmovement_int_bool_char_const_pt(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_057277_ped_commands_getaniminitialoffsetpositionandrotation_rage_crclip(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i32); }
extern "C" { pub fn f_057281_ped_commands_commandsetpedasgroupleader_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057282_ped_commands_commandsetpedasgroupmember_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057283_ped_commands_commandsetpedcanteleporttogroupleader_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_057284_ped_commands_commandremovegroup_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_057285_ped_commands_commandremovepedfromgroup_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_057286_ped_commands_commandispedgroupmember_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057287_ped_commands_commandispedhangingontovehicle_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057288_ped_commands_commandsetgroupseparationrange_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_057289_ped_commands_commandispedincombat_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057290_ped_commands_commandgetpedtargetfromcombatped_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057291_ped_commands_commandcanpedincombatseetarget_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057294_ped_commands_commandispedjacking_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057295_ped_commands_commandispedbeingjacked_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057296_ped_commands_commandispedbeingstunned_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057297_ped_commands_commandgetpedsjacker_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057298_ped_commands_commandgetjacktarget_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057299_ped_commands_commandispedfleeing_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057300_ped_commands_commandispedincover_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057301_ped_commands_commandispedincoverfacingleft_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057302_ped_commands_commandispedinhighcover_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057303_ped_commands_commandispedgoingintocover_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057305_ped_commands_commandgetseatpedistryingtoenter_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057306_ped_commands_commandgetvehiclepedistryingtoenter_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057315_ped_commands_commandremoverelationshipgroup_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_057317_ped_commands_setrelationshipgroupblipforped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_057319_ped_commands_commandsetpedrelationshipgrouphash_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057321_ped_commands_commandgetrelationshipbetweenpeds_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057326_ped_commands_commandtellgrouppedsinareatoattack_int_rage_scrvect(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_057328_ped_commands_commandgetposfromfiredevent_int_int_rage_vector3_re(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_057330_ped_commands_commandsetpedfiringpattern_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057336_ped_commands_commandgetpedgroupindex_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057338_ped_commands_commandgetplayerpedisfollowing_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057342_ped_commands_commandgetvehiclepedisentering_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057345_ped_commands_commandsetpeddamage_int_int_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_057346_ped_commands_commandgettimepeddamagedbyweapon_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057350_ped_commands_commandsetpedascop_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057356_ped_commands_setpedcanbeknockedoffvehicle_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_057359_ped_commands_switchtoragdollcode_cped_ptr_int_bool_ctask_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_057360_ped_commands_commandcanknockpedoffvehicle_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057361_ped_commands_commandknockpedoffvehicle_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_057378_ped_commands_commandispedgettingintoavehicle_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057379_ped_commands_commandispedtryingtoenteralockedvehicle_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057382_ped_commands_commandgetnumberofpedpropdrawablevariations_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057383_ped_commands_commandgetnumberofpeddrawablevariations_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057385_ped_commands_commandgetnumberofpedtexturevariations_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_057386_ped_commands_commandgetnumberofpedproptexturevariations_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_057388_ped_commands_commandgetmpoutfitdatafrommetadata_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_057392_ped_commands_commandsetpedcomponentvariation_int_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_057393_ped_commands_commandsetpedrandomcomponentvariation_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057394_ped_commands_commandsetpedrandomprops_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_057395_ped_commands_commandsetpeddefaultcomponentvariation_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_057396_ped_commands_commandsetpedblendfromparents_int_int_int_float_flo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_057397_ped_commands_commandsetpedheadblenddata_int_int_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: i32); }
extern "C" { pub fn f_057398_ped_commands_commandgetheadblenddata_int_scrpedheadblenddata_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_057420_ped_commands_getassethash_cpedmodelinfo_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_057421_ped_commands_commandgettintindexforlastgenhairtexture_int_int_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_057424_ped_commands_commandfinalizeheadblend_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_057436_ped_commands_commandsetpedpropindex_int_int_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_057441_ped_commands_commanddropambientprop_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_057447_ped_commands_commandsetblockingofnontemporaryevents_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057448_ped_commands_commandregistertarget_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057449_ped_commands_commandregisterhatedtargetsinarea_int_rage_scrvecto(ph: *mut Phantom, a0: i32, a1: i64, a2: f32); }
extern "C" { pub fn f_057451_ped_commands_canpedbegrabbedbyscript_centity_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_057452_ped_commands_commandgetrandompedatcoord_rage_scrvector_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_057460_ped_commands_commandcanpedragdoll_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057461_ped_commands_commandsetpedtoragdoll_int_int_int_int_bool_bool_bo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_057462_ped_commands_commandsetpedtoragdollwithfall_int_int_int_int_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64, a5: f32, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_057463_ped_commands_commandsetpedragdolloncollision_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057465_ped_commands_commandispedrunningragdolltask_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057469_ped_commands_commandismobilephonetopedear_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057470_ped_commands_commandresetpedragdolltimer_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_057471_ped_commands_commandsetpedcanragdoll_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057472_ped_commands_commandsetragdollblockingflags_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057474_ped_commands_commandsetpedangleddefensivearea_int_rage_scrvector(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_057475_ped_commands_commandsetpedspheredefensivearea_int_rage_scrvector(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_057476_ped_commands_commandsetpeddefensivesphereattachedtoped_int_int_r(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_057477_ped_commands_commandsetpeddefensivesphereattachedtovehicle_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_057478_ped_commands_commandsetpeddefensiveareaattachedtoped_int_int_rag(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: f32, a5: i32, a6: i32); }
extern "C" { pub fn f_057481_ped_commands_commantgetpeddefensiveareaposition_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_057485_ped_commands_commandreviveinjuredped_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_057486_ped_commands_commandresurrectped_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_057490_ped_commands_commandispedheadingtowardsposition_int_rage_scrvect(ph: *mut Phantom, a0: i32, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_057492_ped_commands_commandapplypedblooddamagebyzone_int_int_float_floa(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_057493_ped_commands_commandapplypedblood_int_int_rage_scrvector_const_r(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_057494_ped_commands_commandapplypedbloodbyzone_int_int_float_float_char(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: f32, a4: i64); }
extern "C" { pub fn f_057495_ped_commands_commandapplypedbloodspecific_int_int_float_float_fl(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: f32, a8: i64); }
extern "C" { pub fn f_057496_ped_commands_commandapplypeddamagedecal_int_int_float_float_floa(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i64); }
extern "C" { pub fn f_057497_ped_commands_commandapplypeddamagepack_int_char_const_ptr_float(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_057498_ped_commands_commandaddpeddecorationfromhashes_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_057502_ped_commands_commandclearpeddamagedecalbyzone_int_int_char_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_057504_ped_commands_commandgetpeddecorationsstate_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057512_ped_commands_commandclearpeddecorationsleavescars_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_057514_ped_commands_commandgetpedbonecoords_int_int_rage_scrvector_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_057516_ped_commands_commandgivepednmmessage_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_057517_ped_commands_commandaddscenariospawningblockingarea_rage_scrvect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_057518_ped_commands_commanddoesscenarioblockingareaexists_rage_scrvecto(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_057520_ped_commands_commandremovescenarioblockingarea_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057522_ped_commands_commandispedusingscenario_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_057524_ped_commands_commandsetpedpanicexitscenario_int_rage_scrvector_c(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_057525_ped_commands_commandtogglescenariopedcowerinplace_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057526_ped_commands_commandtriggerpedscenariopanicexittoflee_int_rage_s(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_057527_ped_commands_commandsetpedshouldplaydirectednormalscenarioexiton(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_057530_ped_commands_commandsetpedshouldplayfleescenarioexitonnextcomman(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_057541_ped_commands_commandplayfacialanim_int_char_const_ptr_char_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_057542_ped_commands_commandsetfacialclipset_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_057543_ped_commands_commandsetfacialidleanimoverride_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_057544_ped_commands_commandclearfacialidleanimoverride_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_057545_ped_commands_commandsetpedcanarmik_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057546_ped_commands_commandsetpedcanheadik_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057547_ped_commands_commandsetpedcanlegik_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057548_ped_commands_commandsetpedcantorsoik_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057549_ped_commands_commandsetpedcantorsoreactik_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057550_ped_commands_commandsetpedcantorsovehicleik_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057551_ped_commands_commandsetpedcanuseautoconversationlookat_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057552_ped_commands_commandispedheadtrackingped_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057553_ped_commands_commandispedheadtrackingentity_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057554_ped_commands_commandsetpedprimarylookat_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057558_ped_commands_commandsetpedconfigflag_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_057560_ped_commands_commandgetpedconfigflag_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_057564_ped_commands_commandispedevasivediving_int_int_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_057565_ped_commands_commandsetpedgeneratesdeadbodyevents_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057569_ped_commands_commandsetpedshootsatcoord_int_rage_scrvector_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_057570_ped_commands_commandsetpedmodelissuppressed_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057580_ped_commands_commandgivepedhelmetwithoptions_int_bool_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_057582_ped_commands_commandremovepedhelmet_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057588_ped_commands_commandiscurrentheadpropahelmet_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057593_ped_commands_commandsetpedlegikmode_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_057609_ped_commands_commandispedperformingstealthkill_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057611_ped_commands_commandispedbeingstealthkilled_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057612_ped_commands_commandgetmeleetargetforped_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057625_ped_commands_commandisanyhostilepednearpoint_int_rage_scrvector(ph: *mut Phantom, a0: i32, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_057635_ped_commands_haspedreceivedevent_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057636_ped_commands_commandcanpedseehatedped_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057639_ped_commands_commandgetpedboneindex_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_057650_ped_commands_commandcreatesynchronizedscene_rage_scrvector_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_057651_ped_commands_commandcreatesynchronizedscenerelativetomapobject_r(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_057665_ped_commands_commandforcepedmotionstate_int_int_bool_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_057672_ped_commands_commandgetnearbyvehicles_int_int_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_057673_ped_commands_commandgetnearbypeds_int_int_ref_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_057676_ped_commands_commandsetpedusingactionmode_int_bool_int_char_cons(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_057677_ped_commands_commandsetmovementmodeoverride_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_057702_ped_commands_commandsetiktarget_int_int_int_int_rage_scrvector_c(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_057703_ped_commands_commandforceinstantlegiksetup_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_057704_ped_commands_requestmovementmodeasset_char_const_ptr_cpedmodelin(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_057705_ped_commands_hasmovementmodeassetloaded_char_const_ptr_cpedmodel(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_057716_ped_commands_commandistargetpedinperceptionarea_int_int_float_fl(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: f32, a4: f32, a5: f32) -> i32; }
extern "C" { pub fn f_057723_ped_commands_commandsetpedtreatedasfriendly_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_058505_physics_commands_commandaddrope_rage_scrvector_const_ref_rage_sc(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: f32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32, a10: f32, a11: i32, a12: i64) -> i32; }
extern "C" { pub fn f_058506_void_creplaymgrinternal_recordpersistantfx_cpacketaddrope_unsign(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_058515_physics_commands_commandstartropewinding_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_058516_void_creplaymgrinternal_recordpersistantfx_cpacketropewinding_un(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_058517_physics_commands_commandstopropewinding_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_058518_physics_commands_commandstartropeunwindingfront_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_058519_physics_commands_commandstopropeunwindingfront_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_058521_physics_commands_commanddeleterope_int_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_058522_void_creplaymgrinternal_recordpersistantfx_cpacketdeleterope_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_058523_physics_commands_commandloadropedata_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_058524_physics_commands_commandpinropevertex_int_int_rage_scrvector_con(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_058525_physics_commands_commandunpinropevertex_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_058526_void_creplaymgrinternal_recordpersistantfx_cpacketunpinrope_unsi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_058529_physics_commands_commandropesetupdateorder_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_058530_physics_commands_commandattachentitiestorope_int_int_int_rage_sc(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i64, a5: f32, a6: i32, a7: i32, a8: i64, a9: i64); }
extern "C" { pub fn f_058531_void_creplaymgr_recordpersistantfx_cpacketattachentitiestorope_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_058532_physics_commands_commandattachropetoentity_int_int_rage_scrvecto(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_058533_void_creplaymgrinternal_recordpersistantfx_cpacketattachropetoen(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_058534_physics_commands_commanddetachropefromentity_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_058535_void_creplaymgrinternal_recordpersistantfx_cpacketdetachropefrom(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_058541_physics_commands_commanddoesscriptownrope_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_058542_physics_commands_commandropechangescriptowner_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_058545_physics_commands_commandsetcgoffset_int_rage_scrvector_const_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_058549_physics_commands_commandentitybreakglass_int_rage_scrvector_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i64, a4: f32, a5: i32, a6: i32); }
extern "C" { pub fn f_058551_physics_commands_setdisablebreaking_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_058552_physics_commands_resetdisablebreaking_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_058553_physics_commands_setdisablefragdamage_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_058625_player_commands_commandchangeplayermodel_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_058626_player_commands_commandchangeplayerped_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_058638_player_commands_commandalterwantedlevel_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_058639_player_commands_commandalterwantedlevelnodrop_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_058640_player_commands_commandapplywantedlevelchangenow_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_058646_player_commands_commandclearwantedlevel_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_058652_player_commands_commandplayerdetachvirtualbound(ph: *mut Phantom); }
extern "C" { pub fn f_058654_player_commands_commandsetplayercontrol_int_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_058674_player_commands_commandreportcrime_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_058686_player_commands_commandisplayerreadyforcutscene_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_058705_player_commands_commandgetplayergroup_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_058713_player_commands_commandisplayerfreeforambienttask_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_058722_player_commands_commandforcecleanup_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_058723_player_commands_commandforcecleanupforallthreadswiththisname_cha(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_058724_player_commands_commandforcecleanupforthreadwiththisid_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_058727_player_commands_commandsetplayermayonlyenterthisvehicle_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_058736_player_commands_commandsetplayerinvincible_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_058739_player_commands_commandsetplayerinvinciblebuthasreactions_int_bo(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_058760_player_commands_commandsetplayerforcedzoom_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_058761_player_commands_commandsetplayerforceskipaimintro_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_058763_player_commands_commanddisableplayerfiring_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_058767_player_commands_commandspecialabilityactivate_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_058770_player_commands_commandspecialabilitydeactivatemp_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_058792_player_commands_commandgetismoppingareafreeinfrontofplayer_int_f(ph: *mut Phantom, a0: i32, a1: f32) -> i32; }
extern "C" { pub fn f_058793_player_commands_commandstartplayerteleport_int_rage_scrvector_co(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_058833_player_commands_commandisplayerbattleaware_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_058856_player_commands_commandsetapplywaypointofplayer_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_059213_gta_commands_commandcreatemobilephone_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_059290_bool_creplaymgrinternal_handleentityforfx_cpacketdeleterope_cpac(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_059291_bool_creplaymgrinternal_handleentityforfx_cpacketattachentitiest(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_059296_savemigration_commands_commandmpgetaccount_int_scrsavemigrationa(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_059323_script_commands_commandgetnumberofthreadsrunningthescriptwiththi(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_059326_script_commands_commandgeteventexists_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_059327_script_commands_getevent_int_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_059330_script_commands_commandtriggerscriptevent_int_int_ref_int_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_059344_script_commands_commandwinteriscoming_int_int_ref_int_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_059349_script_commands_requestscript_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059351_script_commands_markscriptasnolongerneeded_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059353_script_commands_hasscriptloaded_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_059355_script_commands_doesscriptexist_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_059357_script_commands_requestscriptwithnamehash_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_059359_script_commands_markscriptwithnamehashasnolongerneeded_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_059400_shapetest_commands_commandstartshapetestlosprobe_rage_scrvector(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_059402_shapetest_commands_commandstartexpensivesynchronousshapetestlosp(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_059403_shapetest_commands_commandstartshapetestboundingbox_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_059404_shapetest_commands_commandstartshapetestbound_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_059405_shapetest_commands_commandstartshapetestbox_rage_scrvector_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_059406_shapetest_commands_commandstartshapetestcapsule_rage_scrvector_c(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_059407_shapetest_commands_commandstartshapetestsweptsphere_rage_scrvect(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_059408_shapetest_commands_commandstartshapetestmousecursorlosprobe_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_059409_shapetest_commands_commandgetshapetestresult_int_int_ref_rage_ve(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_059410_shapetest_commands_commandgetshapetestresultincludingmaterial_in(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_059436_socialclub_commands_commandinboxgetugcupdatedata_int_scrugcstate(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_059437_socialclub_commands_commandinboxgetbountydataatindex_int_scrboun(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_059504_socialclub_commands_commandgetpendingpausenewsstory_snewsstoryda(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_059506_socialclub_commands_commandaccountinfogetnickname(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_059618_stats_commands_checkforextrastatspew_rage_atinbinarymap_sstatdat(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_059619_stats_commands_commandstatsetint_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_059620_stats_commands_commandstatsetfloat_int_float_bool(ph: *mut Phantom, a0: i32, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_059621_stats_commands_commandstatsetbool_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_059622_stats_commands_commandstatsetgxtvalue_int_char_const_ptr_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_059623_stats_commands_commandstatsetdate_int_int_ref_int_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_059624_stats_commands_commandstatsetstring_int_char_const_ptr_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_059625_stats_commands_commandstatsetpos_int_float_float_float_bool(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_059626_stats_commands_commandstatsetmaskedint_int_int_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_059627_stats_commands_commandstatsetuserid_int_char_const_ptr_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_059628_stats_commands_commandstatsetcurrentposixtime_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_059629_stats_commands_commandstatgetint_int_int_ref_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_059630_stats_commands_commandstatgetfloat_int_float_ref_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_059631_stats_commands_commandstatgetbool_int_int_ref_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_059632_stats_commands_commandstatgetdate_int_int_ref_int_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_059633_stats_commands_commandstatgetstring_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_059634_stats_commands_commandstatgetpos_int_float_ref_float_ref_float_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_059635_stats_commands_commandstatgetmaskedint_int_int_ref_int_int_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_059636_stats_commands_commandstatgetuserid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_059637_stats_commands_commandstatnetworkincrementonsuicide_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_059639_stats_commands_commandstatincrement_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_059640_stats_commands_commandstatsetlicenseplate_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_059641_stats_commands_commandstatgetlicenseplate_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_059642_stats_commands_commandplaystatsnpcinvite_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059643_stats_commands_commandplaystatsawardxp_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_059645_stats_commands_commandplaystatsrankup_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_059647_stats_commands_commandplaystatsfmeventambmission_challenges_scrf(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059648_stats_commands_commandplaystatsfmeventambmission_vehicletarget_s(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059649_stats_commands_commandplaystatsfmeventambmission_urbanwarfare_sc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059650_stats_commands_commandplaystatsfmeventambmission_checkpointcolle(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059651_stats_commands_commandplaystatsfmeventambmission_atob_scrfmevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059652_stats_commands_commandplaystatsfmeventambmission_pennedin_scrfme(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059653_stats_commands_commandplaystatsfmeventambmission_passtheparcel_s(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059654_stats_commands_commandplaystatsfmeventambmission_hotproperty_scr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059655_stats_commands_commandplaystatsfmeventambmission_deaddrop_scrfme(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059656_stats_commands_commandplaystatsfmeventambmission_kingofthecastle(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059657_stats_commands_commandplaystatsfmeventambmission_criminaldamage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059658_stats_commands_commandplaystatsfmeventambmission_competitiveurba(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059659_stats_commands_commandplaystatsfmeventambmission_huntbeast_scrfm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059660_stats_commands_commandplaystatsbw_bossonbossdeathmatch_scrbwboss(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059661_stats_commands_commandplaystatsbw_yatchattack_scrbwyatchattack_p(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059662_stats_commands_commandplaystatsbw_hunttheboss_scrbwhunttheboss_p(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059663_stats_commands_commandplaystatsbw_sightseer_scrbwsightseer_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059664_stats_commands_commandplaystatsbw_assault_scrbwassault_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059665_stats_commands_commandplaystatsbw_bellyofthebeast_scrbwbellyofth(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059666_stats_commands_commandplaystatsbw_headhunter_scrbwheadhunter_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059667_stats_commands_commandplaystatsbw_fragilegoods_scrbwfragilegoods(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059668_stats_commands_commandplaystatsbw_airfreight_scrbwairfreight_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059669_stats_commands_commandplaystatsbc_carjacking_scrbccarjacking_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059670_stats_commands_commandplaystatsbc_smashandgrab_scrbcsmashandgrab(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059671_stats_commands_commandplaystatsbc_protectionracket_scrbcprotecti(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059672_stats_commands_commandplaystatsbc_mostwanted_scrbcmostwanted_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059673_stats_commands_commandplaystatsbc_finderskeepers_scrbcfinderskee(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059674_stats_commands_commandplaystatsbc_pointtopoint_scrbcpointtopoint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059675_stats_commands_commandplaystatsbc_cashing_scrbccashing_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059676_stats_commands_commandplaystatsbc_salvage_scrbcsalvage_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059678_stats_commands_commandplaystatsbuycontrabandmission_scrbuycontra(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059679_stats_commands_commandplaystatssellcontrabandmission_scrsellcont(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059680_stats_commands_commandplaystatsdefendcontrabandmission_scrdefend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059681_stats_commands_commandplaystatsrecovercontrabandmission_scrrecov(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059686_stats_commands_commandplaystatsmissionstarted_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_059692_stats_commands_commandplaystatscreatematchhistoryid2_int_ref_int(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_059693_stats_commands_commandplaystatsmatchstarted_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_059711_stats_commands_commandplaystatsjobactivityend_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_059712_stats_commands_commandplaystatsjobbend_char_const_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_059713_stats_commands_commandplaystatsjobltsend_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_059714_stats_commands_commandplaystatsjobltsroundend_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_059720_stats_commands_commandleaderboardsreadclearall(ph: *mut Phantom); }
extern "C" { pub fn f_059721_stats_commands_commandleaderboardsreadclear_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_059723_stats_commands_commandleaderboardsreadpending_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_059725_stats_commands_commandleaderboards2readfriendsbyrow_leaderboardr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_059726_stats_commands_commandleaderboards2readbyrow_leaderboardreaddata(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_059727_stats_commands_commandleaderboards2readbyhandle_leaderboardreadd(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_059728_stats_commands_commandleaderboards2readbyrank_leaderboardreaddat(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_059729_stats_commands_commandleaderboards2readbyradius_leaderboardreadd(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_059730_stats_commands_commandleaderboards2readbyscoreint_leaderboardrea(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_059731_stats_commands_commandleaderboards2readbyscorefloat_leaderboardr(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_059732_stats_commands_commandleaderboards2readbyplatform_leaderboardrea(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_059733_stats_commands_rankprediction_leaderboard_const_ref_leaderboardp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_059734_stats_commands_commandleaderboards2readrankprediction_leaderboar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_059735_stats_commands_commandleaderboards2readgetrowdatastart_leaderboa(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_059736_stats_commands_commandleaderboards2readgetrowdataend(ph: *mut Phantom); }
extern "C" { pub fn f_059737_stats_commands_init_leaderboard2_row_data_leaderboardrowdata_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_059739_stats_commands_commandleaderboards2readgetrowdataint_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_059740_stats_commands_commandleaderboards2readgetrowdatafloat_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> f32; }
extern "C" { pub fn f_059741_stats_commands_commandleaderboardswriteaddcolumn_int_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_059742_stats_commands_commandleaderboardswriteaddcolumnlong_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_059743_stats_commands_commandleaderboards2writeleaderboarddataforeventt(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_059747_stats_commands_commandleaderboardscachedatarow_leaderboardcached(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_059751_stats_commands_commandleaderboardsgetcachedatarow_int_int_leader(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_059754_stats_commands_commandstatclearallonlinecharacterstats_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_059756_stats_commands_commandstatgetnumberofdays_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_059757_stats_commands_commandstatgetnumberofhours_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_059758_stats_commands_commandstatgetnumberofminutes_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_059759_stats_commands_commandstatgetnumberofseconds_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_059764_stats_commands_commandstatclearslotforreload_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_059765_stats_commands_commandstatload_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_059773_stats_commands_commandstatgetissafetoprogresstompfromsp(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_059775_stats_commands_commandstatsave_int_bool_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_059836_stats_commands_commandstartbeingboss_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_059837_stats_commands_commandstartbeinggoon_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_059838_stats_commands_commandendbeingboss_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_059839_stats_commands_commandendbeinggoon_int_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_059841_stats_commands_commandorderbossvehicle_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_059843_stats_commands_commandchangegoonlookingforwork_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_059844_stats_commands_commandnetworksendghostingtoplayer_int_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059849_stats_commands_commandplaystatsmissionended_smissionended_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059850_stats_commands_commandplaystatsimpexpmissionended_simpexpmission(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059851_stats_commands_commandplaystatschangemcrole_int_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_059852_stats_commands_commandplaystatschangemcoutfit_int_int_int_int_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_059853_stats_commands_commandplaystatschangemcemblem_int_int_int_int_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_059854_stats_commands_commandplaystatsmcrequestbike_int_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_059855_stats_commands_commandplaystatskilledrivalmcmember_int_int_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_059856_stats_commands_commandplaystatsabandonedmc_int_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_059857_stats_commands_commandplaystatsearnedmcpoints_int_int_int_int_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_059858_stats_commands_commandplaystatsmcformationends_int_int_int_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_059859_stats_commands_commandplaystatsmcclubhouseactivity_int_int_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_059861_stats_commands_commandplaystatsrivalbehaviour_int_int_int_int_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_059864_stats_commands_commandplaystatsgunrunningmissionended_sgunrunnin(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059867_stats_commands_commandplaystatssmugglermissionended_ssmugglermis(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059868_stats_commands_commandplaystatsfreemodeprepend_sfreemodeprepende(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059869_stats_commands_commandplaystatsinstancedheistend_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_059870_stats_commands_commandplaystatsdarcheckpoint_sdarcheckpoint_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059871_stats_commands_commandplaystatsentersessionpack_sentersessionpac(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059873_stats_commands_commandplaystatsbusinessbattleended_sbusinessbatt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059874_stats_commands_commandplaystatswarehousemissionended_swarehousem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059875_stats_commands_commandplaystatsnightclubmissionended_snightclubm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059878_stats_commands_commandplaystatsstonehatchetend_sstonehatchetend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059881_stats_commands_commandplaystatsarenawarsend_scrarenawarsend_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059885_stats_commands_metricarcade_lovematch_metricarcade_lovematch_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_059887_stats_commands_metriccasino_story_mission_ended_metriccasino_sto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_059890_stats_commands_metricroulette_metricroulette_sroulettemetric_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_059892_stats_commands_metricblackjack_metricblackjack_sblackjackmetric(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_059893_stats_commands_commandplaystatscasinothreecardpoker_sthreecardpo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059894_stats_commands_commandplaystatscasinoslotmachine_sslotmachinemet(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059895_stats_commands_commandplaystatscasinoinsidetrack_sinsidetrackmet(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059896_stats_commands_commandplaystatscasinoluckyseven_sluckysevenmetri(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059903_stats_commands_commandplaystatsfreemodecasinomissionend_sfreemod(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059909_stats_commands_metricheist3_finale_metricheist3_finale_heist3fin(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_059911_stats_commands_metricheist3_prep_metricheist3_prep_heist3prepinf(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_059914_stats_commands_commandplaystatsmissionvote_missionvoteinfo_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059916_stats_commands_metricnjvs_vote_metricnjvs_vote_njvsvoteinfo_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_059919_stats_commands_metricheist4_prep_metricheist4_prep_heist4prepinf(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_059921_stats_commands_metricheist4_finale_metricheist4_finale_heist4fin(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_059926_stats_commands_commandplaystatsdjmissionended_djmissionendedinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_059928_stats_commands_metricrobbery_prep_metricrobbery_prep_robberyprep(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_059930_stats_commands_metricrobbery_finale_metricrobbery_finale_robbery(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_059936_stats_commands_metricfreemodemissionend_metricfreemodemissionend(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_059939_stats_commands_metricinstmissionend_metricinstmissionend_sinstmi(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_060322_streaming_commands_commandrequestmodel_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_060324_streaming_commands_hasmodelloaded_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_060325_streaming_commands_requestmodelsinroom_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_060332_streaming_commands_requestanimdict_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_060333_streaming_commands_hasanimdictloaded_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_060334_streaming_commands_removeanimdict_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_060335_streaming_commands_requestclipset_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_060336_streaming_commands_hasclipsetloaded_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_060337_streaming_commands_removeclipset_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_060338_streaming_commands_requestipl_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_060339_streaming_commands_removeipl_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_060355_streaming_commands_requestptfxasset(ph: *mut Phantom); }
extern "C" { pub fn f_060356_streaming_commands_hasptfxassetloaded(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_060357_streaming_commands_removeptfxasset(ph: *mut Phantom); }
extern "C" { pub fn f_060358_streaming_commands_requestnamedptfxasset_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_060359_streaming_commands_hasnamedptfxassetloaded_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_060360_streaming_commands_removenamedptfxasset_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_060364_streaming_commands_setfocusposandvel_rage_scrvector_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_060365_streaming_commands_setfocusentity_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_060366_streaming_commands_setrestorefocusentity_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_060381_streaming_commands_commandissafetostartplayerswitch(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_060382_streaming_commands_commandstartplayerswitch_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_060413_streaming_commands_commandiplgroupswapstart_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_060567_task_commands_commandtaskhandsup_int_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_060569_task_commands_scriptdrivertask_int_cvehicle_ptr_int_int_cvehicle(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: f32, a5: i64, a6: i64); }
extern "C" { pub fn f_060570_task_commands_commandtaskopenvehicledoor_int_int_int_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: f32); }
extern "C" { pub fn f_060571_task_commands_commandtaskentervehicle_int_int_int_int_float_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: f32, a5: i32, a6: i64); }
extern "C" { pub fn f_060572_task_commands_commandtaskleavevehicle_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_060573_task_commands_commandtaskvehicledrivetocoord_intern_int_int_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: f32, a4: i32, a5: i32, a6: f32, a7: f32); }
extern "C" { pub fn f_060575_task_commands_commandtaskvehicledrivetocoordlongrange_int_int_ra(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: f32, a4: i32, a5: f32); }
extern "C" { pub fn f_060576_task_commands_commandtaskvehicledrivewander_int_int_float_int(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_060577_task_commands_commandtaskgostraighttocoord_int_rage_scrvector_co(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: f32, a5: f32); }
extern "C" { pub fn f_060578_task_commands_commandtaskgostraighttocoordrelativetoentity_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_060579_task_commands_commandtaskachieveheading_int_float_int(ph: *mut Phantom, a0: i32, a1: f32, a2: i32); }
extern "C" { pub fn f_060582_task_commands_commandtaskfollowtooffsetofentity_int_int_rage_scr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: f32, a4: i32, a5: f32, a6: i32); }
extern "C" { pub fn f_060583_task_commands_commandtaskfollowpointroute_int_float_int(ph: *mut Phantom, a0: i32, a1: f32, a2: i32); }
extern "C" { pub fn f_060584_task_commands_commandtaskgotoentity_int_int_int_float_float_floa(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: f32, a5: f32, a6: i32); }
extern "C" { pub fn f_060585_ctaskmoveseekentity_213_centityseekposcalculatorstandard_ccloned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: i32) -> i64; }
extern "C" { pub fn f_060586_task_commands_commandtasksmartfleecoord_int_rage_scrvector_const(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_060587_task_commands_commandtasksmartfleeped_int_int_float_int_bool_boo(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_060588_task_commands_commandtaskreactandfleeped_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_060589_task_commands_commandtaskshockingeventreact_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_060591_task_commands_commandtaskwanderstandard_int_float_int(ph: *mut Phantom, a0: i32, a1: f32, a2: i32); }
extern "C" { pub fn f_060592_task_commands_commandtaskwanderspecific_int_char_const_ptr_char(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_060593_task_commands_commandtaskcombat_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_060594_task_commands_commandtaskcombathatedtargetsinarea_int_rage_scrve(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_060595_task_commands_commandtaskcombathatedtargetsaroundped_int_float_i(ph: *mut Phantom, a0: i32, a1: f32, a2: i32); }
extern "C" { pub fn f_060596_task_commands_commandtaskcombathatedtargetsaroundpedtimed_int_fl(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_060597_task_commands_commandtaskcombattimed_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_060598_task_commands_commandtaskthrowprojectile_int_rage_scrvector_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_060599_task_commands_createnavigatetask_int_rage_vector3_const_ref_floa(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_060600_task_commands_commandtaskfollownavmeshtocoord_int_rage_scrvector(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: f32, a5: i32, a6: f32); }
extern "C" { pub fn f_060610_task_commands_commandgetnavmeshroutedistanceremaining_int_float(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_060611_task_commands_commandgetnavmeshrouteresult_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_060612_task_commands_commandiscontrolledvehicleunabletogettoroad_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_060613_task_commands_commandtaskgotocoordanymeans_int_rage_scrvector_co(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: f32); }
extern "C" { pub fn f_060614_task_commands_commandtaskgotocoordanymeansextraparams_int_rage_s(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: f32, a7: f32, a8: f32, a9: i32, a10: f32); }
extern "C" { pub fn f_060615_task_commands_commandtaskgotocoordanymeansextraparamswithcruises(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: f32, a7: f32, a8: f32, a9: i32, a10: f32, a11: f32); }
extern "C" { pub fn f_060616_task_commands_playanimscriptcommand_int_int_char_const_ptr_char(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i64, a10: i64, a11: f32, a12: i32, a13: i32, a14: i32); }
extern "C" { pub fn f_060618_task_commands_verifyslotdata_ctaskscriptedanimation_scriptinitsl(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_060619_task_commands_commandtaskscriptedanimation_int_int_ref_int_ref_i(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32); }
extern "C" { pub fn f_060620_task_commands_commandplayentityscriptedanimation_int_int_ref_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32); }
extern "C" { pub fn f_060621_task_commands_commandtaskplayphonegestureanimation_int_char_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32); }
extern "C" { pub fn f_060622_task_commands_commandtaskstopphonegestureanimation_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_060624_task_commands_commandgetphonegestureanimcurrenttime_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_060626_ctaskscriptedanimation_initslotdata_initslotdata_ctaskscriptedan(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_060628_ctaskscriptedanimation_setclip_rage_crclip_const_ptr_ctaskscript(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_060629_task_commands_commandtaskclipsetblendweight_int_float_int_int_bo(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_060630_ctaskscriptedanimation_getblendweightid_ctaskscriptedanimation_e(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_060631_task_commands_commandtaskclipsetphase_int_float_int_bool(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_060633_task_commands_commandtaskclipsetlooped_int_bool_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_060634_task_commands_commandtaskplayanimadvanced_int_char_const_ptr_cha(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32, a7: i32, a8: i32, a9: f32, a10: i32, a11: i32); }
extern "C" { pub fn f_060635_task_commands_commandstopanimtask_int_char_const_ptr_char_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_060636_task_commands_commandtasksynchronizedscene_int_int_char_const_pt(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32, a8: f32, a9: i32); }
extern "C" { pub fn f_060639_task_commands_commandtaskperformsequence_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_060640_task_commands_commandtaskperformsequencelocally_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_060643_task_commands_commandgetsequenceprogress_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_060646_task_commands_commandgetactivevehiclemissiontype_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_060648_task_commands_commandtaskleaveanyvehicle_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_060649_task_commands_commandtaskaimgunscripted_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_060650_task_commands_commandtaskaimgunscriptedwithtarget_int_int_rage_s(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_060651_task_commands_commmandupdatetaskaimgunscriptedtarget_int_int_rag(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_060653_task_commands_commandtaskaimgunatentity_int_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_060654_task_commands_commandtaskturnpedtofaceentity_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_060655_task_commands_commandtaskaimgunatcoord_int_rage_scrvector_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_060656_task_commands_commandtaskshootatcoord_int_rage_scrvector_const_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_060657_task_commands_commandtaskshuffletonextvehicleseat_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_060658_task_commands_commandclearpedtasks_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_060660_task_commands_commandtaskeveryoneleavevehicle_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_060661_task_commands_commandtaskgotoentityoffset_int_int_int_float_floa(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: f32, a5: f32, a6: i32); }
extern "C" { pub fn f_060662_ctaskmoveseekentity_217_centityseekposcalculatorradiusangleoffse(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: i32) -> i64; }
extern "C" { pub fn f_060663_task_commands_commandtaskgotoentityoffsetxy_int_int_int_float_fl(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_060664_ctaskmoveseekentity_215_centityseekposcalculatorxyoffsetrotated(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: i32) -> i64; }
extern "C" { pub fn f_060665_ctaskmoveseekentity_216_centityseekposcalculatorxyoffsetfixed_cc(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: i32) -> i64; }
extern "C" { pub fn f_060666_task_commands_commandtasklookatcoord_int_rage_scrvector_const_re(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_060667_task_commands_commandtasklookatentity_int_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_060668_task_commands_commandclearlookat_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_060669_task_commands_commandtaskturnpedtofacecoord_int_rage_scrvector_c(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_060670_task_commands_commandtaskgotocoordwhileaimingatcoord_int_rage_sc(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32); }
extern "C" { pub fn f_060671_task_commands_commandtaskgotocoordwhileaimingatentity_int_rage_s(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: f32, a4: i32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32); }
extern "C" { pub fn f_060672_task_commands_commandtaskgotocoordandaimathatedentitiesnearcoord(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32); }
extern "C" { pub fn f_060673_task_commands_commandtaskgotoentitywhileaimingatcoord_int_int_ra(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: f32, a4: i32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_060674_task_commands_commandtaskgotoentitywhileaimingatentity_int_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: i32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_060675_task_commands_commandtaskvehicletempaction_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_060676_task_commands_commandtaskvehiclemission_internal_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: f32, a5: i32, a6: f32, a7: f32, a8: i32, a9: i64); }
extern "C" { pub fn f_060678_task_commands_commandtaskvehiclemissionpedtarget_internal_int_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: f32, a5: i32, a6: f32, a7: f32, a8: i32); }
extern "C" { pub fn f_060681_task_commands_addvehiclesubtaskinternal_int_ctask_ptr_char_const(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_060682_task_commands_commandaddvehiclesubtaskattackcoord_int_rage_scrve(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_060683_task_commands_commandaddvehiclesubtaskattackped_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_060684_task_commands_commandtaskhelimission_int_int_int_int_rage_scrvec(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: f32, a7: f32, a8: f32, a9: i32, a10: i32, a11: f32, a12: i32); }
extern "C" { pub fn f_060685_task_commands_commandtaskheliescort_int_int_int_rage_scrvector_c(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_060686_task_commands_commandtaskplanemission_int_int_int_int_rage_scrve(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: f32, a7: f32, a8: f32, a9: i32, a10: i32, a11: i32); }
extern "C" { pub fn f_060687_task_commands_commandtaskplanetaxi_int_int_rage_scrvector_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_060688_task_commands_commandtaskboatmission_int_int_int_int_rage_scrvec(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: f32, a7: i32, a8: f32, a9: i32); }
extern "C" { pub fn f_060689_task_commands_commandtaskvehiclepark_int_int_rage_scrvector_cons(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: f32, a4: i32, a5: f32, a6: i32); }
extern "C" { pub fn f_060690_task_commands_commandtaskstealthkill_int_int_int_float_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_060691_task_commands_commandtaskplantbomb_int_rage_scrvector_const_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: f32); }
extern "C" { pub fn f_060692_task_commands_commandtaskplanegotoprecisevtol_int_int_rage_scrve(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: f32, a7: i32); }
extern "C" { pub fn f_060693_task_commands_commandtasksubmarinegotoandstop_int_int_rage_scrve(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_060694_task_commands_commandcleardefaultprimarytask_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_060697_task_commands_commandtaskvehicleescort_int_int_int_int_float_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: f32, a5: i32, a6: f32, a7: i32, a8: f32); }
extern "C" { pub fn f_060698_task_commands_commandtaskvehiclefollow_int_int_int_float_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_060699_task_commands_commandtaskheliprotect_int_int_int_float_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: i32, a5: f32, a6: i32, a7: i32); }
extern "C" { pub fn f_060700_task_commands_commandtaskvehiclechase_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_060703_task_commands_commandtaskplanechase_int_int_rage_scrvector_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_060704_task_commands_commandtaskplaneland_int_int_rage_scrvector_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_060705_task_commands_commandtaskhelichase_int_int_rage_scrvector_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_060706_task_commands_commandcontrolmountedweapon_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_060707_task_commands_commandsetmountedweapontarget_int_int_int_rage_scr(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_060709_task_commands_commandtaskdriveby_int_int_int_rage_scrvector_cons(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: f32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_060710_task_commands_commandsetdrivebytarget_int_int_int_rage_scrvector(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_060711_task_commands_commandcleardrivebytaskunderneathdrivingtask_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_060712_task_commands_commandisdrivebytaskunderneathdrivingtask_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_060713_task_commands_commandtaskusemobilephone_int_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_060714_task_commands_commandtaskusemobilephonetimed_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_060715_task_commands_commandtaskchattoped_int_int_int_rage_scrvector_co(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: f32, a5: f32); }
extern "C" { pub fn f_060716_task_commands_commandtaskwarppedintovehicle_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_060717_task_commands_commandtaskshootatentity_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_060720_task_commands_commandtaskrappeldownwallusingclipsetoverride_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32, a5: i64, a6: i32); }
extern "C" { pub fn f_060721_task_commands_commandsgettaskrappeldownwallstate_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_060722_task_commands_commandclearpedtasksimmediately_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_060723_task_commands_commandtaskperformsequencefromprogress_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_060725_task_commands_commandsetpeddesiredmoveblendratio_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_060726_task_commands_commandtaskgotoentityaiming_int_int_float_float(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: f32); }
extern "C" { pub fn f_060727_task_commands_commandtasksetdecisionmaker_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_060728_task_commands_commandtasksetspheredefensivearea_int_rage_scrvect(ph: *mut Phantom, a0: i32, a1: i64, a2: f32); }
extern "C" { pub fn f_060729_task_commands_commandtaskcleardefensivearea_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_060730_task_commands_commandtaskpedslidetocoordwithheadingchangerate_in(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_060732_task_commands_commandtaskpedslidetocoordandplayanimwithheadingch(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64, a6: f32, a7: i32, a8: i32, a9: i32, a10: f32); }
extern "C" { pub fn f_060733_task_commands_commandtaskguardassigneddefensivearea_int_rage_scr(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_060734_task_commands_commandtaskguardspheredefensivearea_int_rage_scrve(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: f32, a4: i32, a5: i64, a6: f32); }
extern "C" { pub fn f_060735_task_commands_commandtaskguardcurrentposition_int_float_float_bo(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: i32); }
extern "C" { pub fn f_060736_task_commands_commandtaskstandguard_int_rage_scrvector_const_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_060737_task_commands_commandaddcoverpoint_rage_scrvector_const_ref_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_060741_task_commands_commandaddscriptedcoverarea_rage_scrvector_const_r(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_060742_task_commands_commandtaskseekcoverfrompos_int_rage_scrvector_con(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_060743_task_commands_commandtaskseekcoverfromped_int_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_060744_task_commands_commandtaskseekcovertocoverpoint_int_int_rage_scrv(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_060745_task_commands_commandtaskseekcovertocoords_int_rage_scrvector_co(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_060746_task_commands_createputpeddirectlyintocovertask_rage_scrvector_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: i32, a6: i32, a7: i32, a8: i32) -> i64; }
extern "C" { pub fn f_060747_task_commands_commandtaskputpeddirectlyintocover_int_rage_scrvec(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: f32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_060748_task_commands_getaitarget_caitarget_ref_int_int_rage_scrvector_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_060749_task_commands_commandtaskwarppeddirectlyintocover_int_int_bool_b(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_060750_task_commands_commandtaskexitcover_int_int_rage_scrvector_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_060751_task_commands_commandtaskputpeddirectlyintomelee_int_int_float_f(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: f32, a4: f32, a5: i32); }
extern "C" { pub fn f_060752_task_commands_commandsetdrivetaskcruisespeed_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_060753_task_commands_commandsetdrivetaskmaxcruisespeed_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_060759_task_commands_commandtaskstartscenarioinplace_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_060760_task_commands_commandtaskstartscenarioatposition_int_char_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_060763_task_commands_commandisscenariooccupied_rage_scrvector_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_060765_task_commands_commandplayanimonrunnningscenario_int_char_const_p(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_060775_task_commands_commonusenearestscenariotopos_cped_ptr_rage_scrvec(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_060778_task_commands_commandtaskusenearestscenariochaintopos_int_rage_s(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_060779_task_commands_commandtaskusenearestscenariochaintoposwarp_int_ra(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_060781_task_commands_commandispedplayingbaseclipinscenario_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_060782_ccliphelper_isplayingclip_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_060785_task_commands_commandtaskreloadweapon_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_060786_task_commands_commandtaskskydive_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_060789_task_commands_commandsetparachutetasktarget_int_rage_scrvector_c(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_060793_task_commands_commandtaskwrithe_int_int_int_int_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_060803_task_commands_commandgetpatrolnodeinfo_int_int_ref_int_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_060804_task_commands_commandvehicleshootatped_int_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_060805_task_commands_commandvehicleaimatped_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_060806_task_commands_commandvehicleshootatcoord_int_rage_scrvector_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: f32); }
extern "C" { pub fn f_060807_task_commands_commandvehicleaimatcoord_int_rage_scrvector_const(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_060808_task_commands_commandhighfall_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_060810_task_commands_commandtaskforcemotionstate_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_060811_task_commands_commandtaskmovenetworkbyname_int_char_const_ptr_fl(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_060812_task_commands_commandtaskmovenetworkbynamewithinitparams_int_cha(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32, a5: i64, a6: i32); }
extern "C" { pub fn f_060813_task_commands_commandtaskmovenetworkadvancedbyname_int_char_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32, a6: i32, a7: i64, a8: i32); }
extern "C" { pub fn f_060814_task_commands_commandtaskmovenetworkadvancedbynamewithinitparams(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: f32, a7: i32, a8: i64, a9: i32); }
extern "C" { pub fn f_060815_task_commands_commandistaskmovenetworkactive_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_060816_task_commands_commandsettaskmovenetworkclipset_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_060817_ctaskmovescripted_setclipset_rage_fwmvtemplateid_rage_fwmvclipse(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_060818_task_commands_commandistaskmovenetworkreadyfortransition_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_060819_task_commands_commandrequesttaskmovenetworkstatetransition_int_c(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_060820_task_commands_commandsetexpectedclonenexttaskmovenetworkstate_in(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_060821_task_commands_commandgettaskmovenetworkstate_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_060822_task_commands_commandsettaskmovenetworksignalfloat_int_char_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: f32); }
extern "C" { pub fn f_060823_task_commands_commandsettaskmovenetworksignalfloatlerprate_int_c(ph: *mut Phantom, a0: i32, a1: i64, a2: f32); }
extern "C" { pub fn f_060824_task_commands_commandgettaskmovenetworksignalfloat_int_char_cons(ph: *mut Phantom, a0: i32, a1: i64) -> f32; }
extern "C" { pub fn f_060825_task_commands_commandsettaskmovenetworksignalbool_int_char_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_060826_task_commands_commandsettaskmovenetworksignallocalfloat_int_char(ph: *mut Phantom, a0: i32, a1: i64, a2: f32); }
extern "C" { pub fn f_060827_task_commands_commandgettaskmovenetworksignalbool_int_char_const(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_060828_task_commands_commandgettaskmovenetworkevent_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_060829_task_commands_commandsettaskmovenetworkenablecollisiononnetworkc(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_060830_task_commands_playvehicleanimscriptcommand_int_char_const_ptr_ch(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_060841_task_commands_commandtasksweepaimentity_int_char_const_ptr_char(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: f32, a8: f32); }
extern "C" { pub fn f_060842_task_commands_commandupdatetasksweepaimentity_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_060843_task_commands_commandtasksweepaimposition_int_char_const_ptr_cha(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: f32, a8: f32); }
extern "C" { pub fn f_060845_task_commands_commandtaskarrestped_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_060846_task_commands_commandispedrunningarresttask_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_060849_task_commands_commandtaskvehiclegotonavmesh_int_int_rage_scrvect(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: f32, a4: i32, a5: f32); }
extern "C" { pub fn f_060850_task_commands_commandgivepedagitatedtaskconfront_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_061152_waypoint_commands_commandtaskfollowwaypointrecording_int_char_co(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_061159_waypoint_commands_commandgetpedwaypointdistance_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_061161_waypoint_commands_commandsetpedwaypointrouteoffset_int_rage_scrv(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_061165_waypoint_commands_commandwaypointplaybackgetispaused_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_061167_waypoint_commands_commandwaypointplaybackpause_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_061169_waypoint_commands_commandwaypointplaybackresume_int_bool_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_061171_waypoint_commands_commandwaypointplaybackoverridespeed_int_float(ph: *mut Phantom, a0: i32, a1: f32, a2: i32); }
extern "C" { pub fn f_061173_waypoint_commands_commandwaypointplaybackusedefaultspeed_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_061177_waypoint_commands_commandwaypointplaybackstartaimingatped_int_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_061180_waypoint_commands_commandwaypointplaybackstartaimingatcoord_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_061182_waypoint_commands_commandwaypointplaybackstartshootingatped_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_061185_waypoint_commands_commandwaypointplaybackstartshootingatcoord_in(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_061187_waypoint_commands_commandwaypointplaybackstopaimingorshooting_in(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_061200_waypoint_commands_commandtaskvehiclefollowwaypointrecording_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: i32, a9: f32); }
extern "C" { pub fn f_061206_waypoint_commands_commandgetvehiclewaypointtargetpoint_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_061208_waypoint_commands_commandvehiclewaypointplaybackpause_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_061211_waypoint_commands_commandvehiclewaypointplaybackresume_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_061213_waypoint_commands_commandvehiclewaypointplaybackusedefaultspeed(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_061215_waypoint_commands_commandvehiclewaypointplaybackoverridespeed_in(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_061380_vehicle_commands_canvehiclebegrabbedbyscript_cvehicle_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_061447_vehicle_commands_basegetrandomcarinspherenosave_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_061491_vehicle_commands_sethelibladesspeed_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_063737_gtathread_forcecleanupforallmatchingthreads_unsigned_int_gtathre(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_063742_gtathread_setforcecleanupreturnpoint_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063743_cthescripts_isplayerplaying_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063744_gtathread_scriptthreaditeratorreset(ph: *mut Phantom); }
extern "C" { pub fn f_063745_gtathread_scriptthreaditeratorgetnextthreadid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_063767_cscriptpatrol_openpatrolroute_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063768_cscriptpatrol_addpatrolnode_int_char_const_ptr_rage_vector3_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_063769_cscriptpatrol_addpatrollink_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_063770_cscriptpatrol_createpatrolroute(ph: *mut Phantom); }
extern "C" { pub fn f_063771_cgamescriptresource_incrementnumberofusedresources(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063772_cthescripts_getcurrentgtascripthandler(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063794_cthescripts_registerentity_cphysical_ptr_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_063796_cthescripts_unregisterentity_cphysical_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063797_cthescripts_getcurrentgtascripthandlernetwork(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063798_cthescripts_prioritizerequestsfrommissionscript_int_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063799_cthescripts_getcurrentscriptname(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063800_cthescripts_getcurrentscriptnameandprogramcounter(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063814_cthescripts_getscriptedcoverpointtomodifyfromguid_int_char_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063815_cthescripts_findlocalplayerped_int_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063816_cthescripts_findnetworkplayerped_int_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063817_cthescripts_findnetworkplayer_int_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063818_cthescripts_exportgamerhandle_rage_rlgamerhandle_const_ptr_int_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_063819_cthescripts_importgamerhandle_rage_rlgamerhandle_ptr_int_ref_int(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063823_cscripteulers_matrixfromeulers_rage_matrix34_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063825_cscripteulers_quaternionfromeulers_rage_quaternion_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063826_cscripteulers_quaterniontoeulers_rage_quaternion_ref_rage_eulera(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063854_cgamescripthandler_registerscriptresourceandgetref_rage_scriptre(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063880_cupsidedowncarcheck_iscarinupsidedowncararray_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_063881_cupsidedowncarcheck_addcartocheck_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_063882_cupsidedowncarcheck_removecarfromcheck_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_063883_cupsidedowncarcheck_hascarbeenupsidedownforawhile_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_063885_cstuckcarcheck_addcartocheck_int_float_unsigned_int_bool_bool_bo(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_063886_cstuckcarcheck_removecarfromcheck_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_063887_cstuckcarcheck_iscarinstuckcararray_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_063890_csuppressedmodels_clearallsuppressedmodels(ph: *mut Phantom); }
extern "C" { pub fn f_063892_csuppressedmodels_addtosuppressedmodelarray_unsigned_int_rage_at(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_063894_csuppressedmodels_removefromsuppressedmodelarray_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_063896_cscriptpeds_ispedstopped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063898_cscriptcars_isvehiclestopped_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063899_cscriptpeds_setpedcoordinates_cped_ptr_float_float_float_int_boo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_063900_cscriptentities_clearspaceformissionentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063901_cscriptpeds_givepedscriptedtask_int_ctasksequencelist_ptr_int_ch(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_063902_cscriptpeds_givepedscriptedtask_int_rage_aitask_ptr_int_char_con(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_063912_missioncreatorassets_init(ph: *mut Phantom); }
extern "C" { pub fn f_063913_missioncreatorassets_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_063914_missioncreatorassets_getadditionalmodelcost_rage_atuserarray_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_063915_missioncreatorassets_addmodel_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063916_missioncreatorassets_removemodel_rage_fwmodelid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063917_missioncreatorassets_getusedbudget(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_064020_cscripthud_getscaleformmovieid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064083_cscriptmetadatamanager_getmpapparelmale_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064084_cscriptmetadatamanager_getmpapparelfemale_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064087_cscriptshapetestmanager_debugcreateshapetestrequest_char_const_p(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_064088_cscriptshapetestmanager_getshapetestresult_int_worldprobe_cshape(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_064141_cstreamedscripts_getnumrefs_rage_strlocalindex_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_064173_cgamescripthandler_registerscriptresourceandgetid_rage_scriptres(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_064215_cgamescriptid_generatescriptidhash(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064241_cscriptstreamingresource_cscriptstreamingresource_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_064247_cscriptresource_ptfx_asset_cscriptresource_ptfx_asset_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_064251_cscriptresource_pedgroup_cscriptresource_pedgroup_cgamescripthan(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_064252_cscriptresource_pedgroup_cscriptresource_pedgroup_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_064255_cscriptresource_sequencetask_cscriptresource_sequencetask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064275_cscriptresource_vehiclerecording_cscriptresource_vehiclerecordin(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_064288_cscriptresource_rope_cscriptresource_rope_rage_ropeinstance_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_064299_cscriptresource_relgroup_cscriptresource_relgroup_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_064305_cscriptresource_streamedscript_cscriptresource_streamedscript_in(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_064313_cscriptresource_vehicle_asset_cscriptresource_vehicle_asset_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_064323_cscriptresource_setrelgroupdontaffectwantedlevel_cscriptresource(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_064425_cgamescriptresource_dtorcgamescriptresource_102634(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064640_ccustomshadereffectvehicle_setlicenseplatetexindex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064642_ccustomshadereffectvehicle_setlicenseplatetext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064646_ccustomshadereffectvehicle_setcustomprimarycolor_rage_color32(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064647_ccustomshadereffectvehicle_setcustomsecondarycolor_rage_color32(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064652_ccustomshadereffectvehicle_getlicenseplatetext_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064749_cislandhopper_requesttoggleisland_rage_atnonfinalhashstring_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_064759_cislandhopper_arecurrentislandiplsactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064776_cpopulationstreaming_requestvehicledriver_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064786_cpopulationstreaming_setreduceambientpedmodelbudget_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064787_cpopulationstreaming_setreduceambientvehiclemodelbudget_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064800_cstreaming_hasobjectloaded_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064813_cpopulationstreaming_isfallbackpedavailable_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064829_cpopulationstreaming_setvehmemorybudgetlevel_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064830_cpopulationstreaming_setpedmemorybudgetlevel_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064832_cstreaming_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064870_cscriptpreloaddata_haspreloadfinished_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_064876_cscriptpreloaddata_releasehandle_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064938_cstreaming_disablestreaming(ph: *mut Phantom); }
extern "C" { pub fn f_064939_cstreaming_enablestreaming(ph: *mut Phantom); }
extern "C" { pub fn f_065128_cstreamingrequestlist_beginprestream_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_065132_cstreamingrequestlist_start(ph: *mut Phantom); }
extern "C" { pub fn f_065133_cstreamingrequestlist_finish(ph: *mut Phantom); }
extern "C" { pub fn f_065136_cstreamingrequestlist_setsrlreadaheadtimes_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_065137_cstreamingrequestlist_setpostcutscenecamera_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065376_ccontrol_getinputname_rage_inputtype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065379_ccontrol_dodisableinput_unsigned_int_rage_iovalue_disableoptions(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_065446_ccontrolmgr_getmainplayercontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065875_ctasksynchronizedscene_ctasksynchronizedscene_short_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32, a5: f32, a6: i64, a7: i64, a8: f32, a9: i32, a10: i64) -> i64; }
extern "C" { pub fn f_065912_ctaskmovescripted_ctaskmovescripted_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_065916_ctaskmovescripted_requeststatetransition_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065921_ctaskmovescripted_lerpfloat_ref_rage_atmap_unsigned_int_ctaskmov(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_065926_ctaskmovescripted_setsignalfloat_char_const_ptr_float_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_065929_ctaskmovescripted_setsignalbool_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_065949_ctaskscriptedanimation_ctaskscriptedanimation_ctaskscriptedanima(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_065950_ctaskscriptedanimation_ctaskscriptedanimation_rage_atnonfinalhas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32, a6: f32, a7: i32, a8: i64, a9: f32, a10: i32, a11: i32, a12: i64, a13: i32, a14: i32) -> i64; }
extern "C" { pub fn f_065951_ctaskscriptedanimation_ctaskscriptedanimation_rage_atnonfinalhas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32, a6: f32, a7: i32, a8: i64, a9: f32, a10: i64, a11: i64, a12: i32, a13: i32, a14: i64, a15: i32, a16: i32) -> i64; }
extern "C" { pub fn f_065967_ctaskscriptedanimation_startplayback_ctaskscriptedanimation_init(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_065969_ctaskscriptedanimation_shouldplayfpsclip(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066004_ccombattaskmanager_countpedsincombatwithtarget_cped_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_066010_ctaskmelee_ctaskmelee_cactionresult_const_ptr_centity_ptr_rage_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_066062_ccoverpoint_getcoverpointposition_rage_vector3_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066096_ctaskcover_ctaskcover_cweapontarget_const_ref_int_ctaskcover_efo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_066113_ctasksetandguardarea_ctasksetandguardarea_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32, a5: f32, a6: i64, a7: i32, a8: i32) -> i64; }
extern "C" { pub fn f_066119_ctaskstandguard_ctaskstandguard_rage_vector3_const_ref_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32, a5: f32) -> i64; }
extern "C" { pub fn f_066150_ctaskcombatclosesttargetinarea_ctaskcombatclosesttargetinarea_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: f32, a5: i32) -> i64; }
extern "C" { pub fn f_066157_ctaskthreatresponse_ctaskthreatresponse_cped_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_066389_ctaskinvestigate_ctaskinvestigate_rage_vector3_const_ref_cped_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_066528_ctaskdraggingtosafety_ctaskdraggingtosafety_cped_ptr_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_066578_ctasksharkcircle_ctasksharkcircle_float_caitarget_const_ref_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_066632_ctaskhandsup_ctaskhandsup_int_cped_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_066655_ctaskwrithe_ctaskwrithe_cweapontarget_const_ref_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_066683_ccoverpointscontainer_checkifnooverlapandgetgridcell_rage_vector(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_066694_ccover_addcoverpoint_ccover_coverpooltype_ccoverpoint_ecovertype(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i64) -> i32; }
extern "C" { pub fn f_066713_ccover_addcoverblockingarea_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_066714_ccover_removecoverblockingareasatposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066715_ccover_removespecificcoverblockingarea_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_066934_ctaskmotionincover_processusemobilephone_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_067061_ctaskstayincover_ctaskstayincover(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067147_ctaskhelichase_ctaskhelichase_caitarget_const_ref_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_067212_ctaskvehiclechase_changebehaviorflag_ctaskvehiclechase_overrides(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_067214_ctaskvehiclechase_setidealdistanceforpursue_ctaskvehiclechase_ov(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_067255_ctaskvehiclecombat_ctaskvehiclecombat_caitarget_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32) -> i64; }
extern "C" { pub fn f_067305_ctaskstealthkill_ctaskstealthkill_cped_const_ptr_cactionresult_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_067827_testclipmoveragainstworld_rage_crclip_const_ptr_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_067869_cpropmanagementhelper_dropprop_cped_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_067894_ctaskambientclips_overridebaseclip_rage_fwmvtemplateid_rage_fwmv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_067915_ctaskchat_ctaskchat_cped_ptr_rage_fwflags_int_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_067917_ctaskcomplexcontrolmovement_ctaskcomplexcontrolmovement_ctask_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_067918_ctaskcomplexcontrolmovement_getrunningmovementtask_cped_const_pt(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067955_ctaskmobilephone_isrunningmobilephonetask_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067983_ctaskmobilephone_ctaskmobilephone_int_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_067993_ctaskmoveinair_ctaskmoveinair(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068030_ctaskwander_keepmovingwhilstwaitingforfirstpath_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068048_ctaskwander_ctaskwander_float_float_cconditionalanimsgroup_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i32, a5: i32, a6: f32, a7: i32, a8: i32) -> i64; }
extern "C" { pub fn f_068057_ctaskcomplexcontrolmovement_setnewmovetask_ctask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068069_ctaskwanderinarea_ctaskwanderinarea_float_float_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_068097_ctaskstandguardfsm_ctaskstandguardfsm_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i64; }
extern "C" { pub fn f_068106_ctaskpatrol_ctaskpatrol_unsigned_int_ctaskpatrol_alertstatus_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_068126_ctaskdonothing_ctaskdonothing_int_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_068132_ctaskpause_ctaskpause_int_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068139_ctaskcrouchtoggle_ctaskcrouchtoggle_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068161_ctasksequencelist_ctasksequencelist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068163_ctasksequencelist_addtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068175_ctasktriggerlookat_ctasktriggerlookat_centity_const_ptr_int_eani(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_068183_ctaskclearlookat_ctaskclearlookat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068202_ctaskusesequence_ctaskusesequence_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_068247_ctaskaffectsecondarybehaviour_ctaskaffectsecondarybehaviour_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_068253_ctaskcrouch_ctaskcrouch_int_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068257_ctasktakeoffhelmet_ctasktakeoffhelmet(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068270_ctaskgeneralsweep_ctaskgeneralsweep_int_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: f32, a8: f32) -> i64; }
extern "C" { pub fn f_068318_ctaskmobilephone_clearadditionalsecondaryanimation_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_068319_ctaskmobilephone_getadditionalsecondaryanimclipandfilter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068455_ctaskmotionbase_setinvehiclecontexthash_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_068456_ctaskmotionbase_clearoverrideweaponclipset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068457_ctaskmotionbase_resetonfootclipset_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_068458_ctaskmotionbase_resetinvehiclecontexthash_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068459_ctaskmotionbase_resetstrafeclipset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068462_ctaskmotionbase_stopalternateclip_ctaskmotionbase_alternateclipt(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_068510_ctaskmotionswimming_clearscubagearvariationforped_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068512_ctaskmotionswimming_isscubagearvariationactiveforped_cped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068515_ctaskmotionswimming_setscubagearvariationforped_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068600_ctaskfishlocomotion_togglepedswimnearsurface_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_068804_ctaskfall_ctaskfall_rage_fwflags_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068914_ctaskcower_ctaskcower_int_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068959_ctaskmovefollownavmesh_isunabletofindroute_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069009_ctaskparachute_ctaskparachute_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_069039_ctaskfollowwaypointrecording_ctaskfollowwaypointrecording_cwaypo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_069049_ctaskfollowwaypointrecording_setpause_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_069050_ctaskfollowwaypointrecording_setresume_bool_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_069051_ctaskfollowwaypointrecording_setoverridemoveblendratio_float_boo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_069052_ctaskfollowwaypointrecording_usedefaultmoveblendratio(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069053_ctaskfollowwaypointrecording_startaimingatentity_centity_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_069054_ctaskfollowwaypointrecording_startshootingatentity_centity_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_069055_ctaskfollowwaypointrecording_stopaimingorshooting(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069064_ctaskturntofaceentityorcoord_ctaskturntofaceentityorcoord_centit(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_069065_ctaskmovegotopointandstandstill_ctaskmovegotopointandstandstill(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: f32) -> i64; }
extern "C" { pub fn f_069069_ctaskmoveachieveheading_ctaskmoveachieveheading_float_float_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_069075_ctaskturntofaceentityorcoord_ctaskturntofaceentityorcoord_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_069136_ctaskmovefollowpointroute_ctaskmovefollowpointroute_float_cpoint(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32, a10: f32) -> i64; }
extern "C" { pub fn f_069149_ctaskgotopointanymeans_ctaskgotopointanymeans_float_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: f32, a5: i32, a6: i32, a7: i32, a8: f32, a9: f32, a10: f32, a11: i32, a12: f32, a13: f32, a14: f32) -> i64; }
extern "C" { pub fn f_069185_ctaskmovefollownavmesh_isfollowingnavmeshroute_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069224_ctaskmovegotopointrelativetoentityandstandstill_ctaskmovegotopoi(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: f32, a5: i32) -> i64; }
extern "C" { pub fn f_069242_ctaskgotopointaiming_ctaskgotopointaiming_caitarget_const_ref_ca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_069250_ctaskmovefollownavmesh_setscriptbehaviourflags_int_tnavmeshscrip(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_069256_ctaskmovefollowentityoffset_ctaskmovefollowentityoffset_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_069289_ctasknavbase_getdistanceleftoncurrentroutesection_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_069477_ctaskmovefollownavmesh_ctaskmovefollownavmesh_float_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: i64, a8: f32, a9: i32) -> i64; }
extern "C" { pub fn f_069529_ctaskparachute_clearparachutepackvariationforped_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069533_ctaskparachute_setparachutepackvariationforped_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069544_ctaskparachute_settargetposition_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069617_ctaskparachute_createparachutebagobject_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_069631_ctaskjetpack_setgotoparametersforai_cphysical_ptr_rage_vector3_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_069632_ctaskjetpack_setshootatparametersforai_cphysical_ptr_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_069697_ctaskseekentityaiming_ctaskseekentityaiming_centity_const_ptr_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> i64; }
extern "C" { pub fn f_069702_ctaskslidetocoord_ctaskslidetocoord_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> i64; }
extern "C" { pub fn f_069703_ctaskslidetocoord_ctaskslidetocoord_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64, a6: i32, a7: f32, a8: i32, a9: i32); }
extern "C" { pub fn f_069858_ctaskclimbladderfully_ctaskclimbladderfully_float(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_069867_ctaskhelipassengerrappel_ctaskhelipassengerrappel_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i64; }
extern "C" { pub fn f_069886_ctaskrappel_ctaskrappel_float_rage_vector3_const_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_069993_ctaskjumpvault_ctaskjumpvault_rage_fwflags_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_070096_ctaskmotionped_getmovenetworkhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070384_ctaskanimatedattach_ctaskanimatedattach_cphysical_ptr_int_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32) -> i64; }
extern "C" { pub fn f_070401_ctasknmbehaviour_canuseragdoll_cped_ptr_eragdolltriggertypes_cen(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_070465_ctasknmbehaviour_ismessagestring_enmstrings(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_070489_ctasknmbehaviour_isparamforthismessage_enmstrings_enmstrings(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_070529_ctasknmhighfall_ctasknmhighfall_unsigned_int_cgrabhelper_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_070530_ctasknmbalance_ctasknmbalance_unsigned_int_unsigned_int_centity(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64, a6: f32, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_070531_ctasknmbalance_evaluateandsettype_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070593_ctasknmdangle_setgrabparams_cped_ptr_bool_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_070636_ctasknmfalldown_ctasknmfalldown_unsigned_int_unsigned_int_ctaskn(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_070739_ctasknmrelax_ctasknmrelax_unsigned_int_unsigned_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_070859_ctaskagitated_ctaskagitated_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070881_ctaskagitatedaction_ctaskagitatedaction_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070890_ctaskconfront_ctaskconfront_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070913_ctasksmartflee_ctasksmartflee_caitarget_const_ref_float_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_071039_ctaskreactandflee_ctaskreactandflee_caitarget_const_ref_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_071041_ctaskreactandflee_ctaskreactandflee_caitarget_const_ref_ctaskrea(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_071074_ctaskreactandflee_pickdefaultgunfirereactionclipset_rage_vec3v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_071075_ctaskshockingeventhurryaway_ctaskshockingeventhurryaway_ceventsh(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_071128_ctaskshockingeventreact_ctaskshockingeventreact_ceventshocking_c(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_071141_ctaskshockingeventbackaway_ctaskshockingeventbackaway_ceventshoc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_071168_ctaskshockingeventbackaway_getdefaultbackawaypositionforped_cped(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072481_cscenariopoint_getposition_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072482_cscenarioinfomanager_getscenarioindex_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_072490_cscenariopoint_getscenariotypevirtualorreal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072539_cscenariomanager_getscenarioinfo_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_072558_cscenariomanager_setupscenarioandtask_int_rage_vector3_ref_float(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_072561_ctaskusescenario_ctaskusescenario_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_072568_cscenariomanager_setupscenariocontroltask_cped_const_ref_cscenar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_072576_ctaskusescenario_ctaskusescenario_int_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_072629_cscenariomanager_getscenariotypefromname_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072656_cscenariomanager_chooserealscenario_int_cped_ptr(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_072731_cscenariomanager_getscenariotypefromhashkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_072760_cscenariopointmanager_findgroupbynamehash_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_072912_cscenariopointmanager_resetgroupsenabledtodefaults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072914_cscenariopointmanager_setgroupenabled_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_072916_cscenariopointprioritymanager_restoregroupstooriginalpriorities(ph: *mut Phantom); }
extern "C" { pub fn f_072952_ctaskusescenario_setambientposition_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073405_cscenarioinfomanager_resetscenariotypesenabledtodefaults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073486_ctaskusescenario_clearscriptedtriggerflags_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073642_ctaskarrestped_ctaskarrestped_cped_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_073685_ctaskswatfollowinline_ctaskswatfollowinline_caitarget_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_074161_ctask_ctask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_074169_ctask_propagatenewmovespeed_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_075332_ctaskfsmclone_ctaskfsmclone(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075348_cmovenetworkhelper_getboolean_rage_fwmvtemplateid_rage_fwmvboole(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075349_cmovenetworkhelper_getfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_075350_ccliprequesthelper_ccliprequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075353_ccliprequesthelper_requestclipsbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075401_cboatfollowroutehelper_cboatfollowroutehelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075406_cpathnoderoutesearchhelper_cpathnoderoutesearchhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075462_cmovenetworkhelper_setclip_rage_crclip_const_ptr_rage_fwmvtempla(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075470_cmovenetworkhelper_cmovenetworkhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075474_cmovenetworkhelper_setfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_075491_cmovenetworkhelper_setfilter_rage_crframefilter_ptr_rage_fwmvtem(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075500_cmovenetworkhelper_setclipset_rage_fwmvtemplateid_rage_fwmvclips(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075503_cmovenetworkhelper_setboolean_rage_fwmvtemplateid_rage_fwmvboole(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075604_ctaskcontrolvehicle_ctaskcontrolvehicle_cvehicle_ptr_rage_aitask(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_075631_ctaskmove_ctaskmove_float(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_075644_ctasktreeclone_startlocalclonetask_ctaskfsmclone_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075695_ctaskleaveanycar_ctaskleaveanycar_int_cvehicleenterexitflags_fla(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_075697_ctaskexitvehicle_ctaskexitvehicle_cvehicle_ptr_cvehicleenterexit(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_075702_ctaskinvehiclebasic_ctaskinvehiclebasic_cvehicle_ptr_bool_ccondi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_075709_ctaskvehiclemountedweapon_istaskvalid_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075710_ctaskvehiclegun_ctaskvehiclegun_ctaskvehiclegun_etaskmode_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_075724_ctaskentervehicle_ctaskentervehicle_cvehicle_ptr_seatrequesttype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: f32, a6: f32, a7: i64, a8: i64) -> i64; }
extern "C" { pub fn f_075727_ctaskcardrivewander_ctaskcardrivewander_cvehicle_ptr_int_float_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_075732_ctaskdrivepointroute_ctaskdrivepointroute_cvehicle_ptr_cpointrou(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32, a6: f32, a7: i32) -> i64; }
extern "C" { pub fn f_075737_ctaskdrivepointroute_setdesiredsubtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075738_ctaskcarsettempaction_ctaskcarsettempaction_cvehicle_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_075753_ctaskinvehicleseatshuffle_ctaskinvehicleseatshuffle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_075754_ctaskvehiclefsm_canpedshuffletoorfromturretseat_cvehicle_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075755_ctaskvehiclefsm_canpedshuffletoorfromextraseat_cvehicle_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075758_ctaskvehiclefsm_getpedinorusingseat_cvehicle_const_ref_int_cped(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_075759_ctaskcomplexgetoffboat_ctaskcomplexgetoffboat_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_075769_ctaskcontrolvehicle_setdesiredsubtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075777_ctaskcontrolvehicle_isvehicletaskunabletogettoroad_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075789_ctasksetpedinvehicle_ctasksetpedinvehicle_cvehicle_ptr_int_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_076263_ctaskridetrain_ctaskridetrain_ctrain_ptr_ctask_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_076292_ctaskvehiclefsm_setscriptedvehicleentryexitflags_cvehicleenterex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_076666_ctaskaimandthrowprojectile_ctaskaimandthrowprojectile_cweapontar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_076717_ctaskswapweapon_ctaskswapweapon_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_076746_ctaskgun_ctaskgun_cweaponcontroller_weaponcontrollertype_ctaskty(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32) -> i64; }
extern "C" { pub fn f_076853_cscriptedguntaskmetadatamgr_getscriptedguntaskinfobyhash_unsigne(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_076895_ctaskreloadgun_ctaskreloadgun_cweaponcontroller_weaponcontroller(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_077080_cstatsmgr_setcheatisactive_bool(ph: *mut Phantom); }
extern "C" { pub fn f_077088_cstatsmgr_displayscriptstatupdatemessage_bool_statid_const_ref_f(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077089_cstatsmgr_incrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077090_cstatsmgr_decrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077091_cstatsmgr_setgreater_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077116_cstatsmgr_isplayervehicleaboveocean(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077119_cstatsmgr_playercompletemission(ph: *mut Phantom); }
extern "C" { pub fn f_077121_cstatsmgr_getflyingaltitude_float_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077127_cstatsmgr_getrecordedstatvalue_float_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077128_cstatsmgr_startrecordingstat_cstatsmgr_recordstat_cstatsmgr_reco(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_077129_cstatsmgr_stoprecordingstat(ph: *mut Phantom); }
extern "C" { pub fn f_077130_cstatsmgr_isrecordingstat(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077131_cstatsmgr_starttrackingstunts(ph: *mut Phantom); }
extern "C" { pub fn f_077132_cstatsmgr_stoptrackingstunts(ph: *mut Phantom); }
extern "C" { pub fn f_077152_statid_char_gethash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077153_statid_char_getstatid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077176_sstatdata_valuetostring_char_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077181_cstatsdatamgr_resetallstats_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077187_cstatsdatamgr_updatecommunitystatvalues(ph: *mut Phantom); }
extern "C" { pub fn f_077190_cstatsdatamgr_resetallonlinecharacterstats_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077193_cstatsdatamgr_iskeyvalid_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077194_cstatsdatamgr_resynchserverauthoritative_bool(ph: *mut Phantom); }
extern "C" { pub fn f_077198_cstatsdatamgr_setstatiterdata_rage_atinbinarymap_sstatdataptr_st(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_077200_cstatsdatamgr_statiterfind_statid_const_ref_rage_atinbinarymap_s(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077202_cstatsdatamgr_getisbasetype_statid_const_ref_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077328_get_stat_savetype_name_esavetypes(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077329_statid_set_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077331_statsinterface_getintstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077333_statsinterface_getkeyname_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077334_statsinterface_iskeyvalid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077335_statsinterface_getstattype_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077338_statsinterface_getcurrentmultiplayercharaterslot(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077339_statsinterface_cloudfileloadpending_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077340_statsinterface_cloudfileloadfailed_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077342_statsinterface_setstatdata_statid_const_ref_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077343_statsinterface_setstatdata_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077345_statsinterface_setstatdata_statid_const_ref_unsigned_char_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077346_statsinterface_setstatdata_statid_const_ref_unsigned_short_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077347_statsinterface_setstatdata_statid_const_ref_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077348_statsinterface_setstatdata_statid_const_ref_unsigned_long_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_077349_statsinterface_setstatdata_statid_const_ref_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_077354_statsinterface_getfloatstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_077356_statsinterface_getuint8stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077357_statsinterface_getuint16stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077358_statsinterface_getuint32stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077359_statsinterface_getuint64stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077360_statsinterface_getint64stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077362_statsinterface_unpackdate_unsigned_long_int_ref_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_077365_statsinterface_unpackpos_unsigned_long_float_ref_float_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_077371_statsinterface_setstatsmodelprefix_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077373_statsinterface_getstatsmodelhashid_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077377_statsinterface_setmaskedint_statid_const_ref_int_int_int_bool_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_077378_statsinterface_getmaskedint_int_int_ref_int_int_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_077379_statsinterface_getbossgoonuuid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077380_statsinterface_clearpedscardata(ph: *mut Phantom); }
extern "C" { pub fn f_077389_cstatsutils_converttodays_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077390_cstatsutils_converttohours_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077391_cstatsutils_converttomins_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077392_cstatsutils_converttoseconds_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077393_cstatsutils_converttomilliseconds_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077394_cstatsutils_isstatstrackingenabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077395_cstatsutils_enablestatstracking_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077396_cstatsutils_disablestatstracking(ph: *mut Phantom); }
extern "C" { pub fn f_077402_cstatssavesmgr_clearloadpending_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077408_cstatssavesmgr_pendingsaverequests_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077412_cstatssavesmgr_getlocalclouddirtyreadvalue_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077414_cstatssavesmgr_save_estatssavetype_unsigned_int_esavetypes_unsig(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_077415_cstatssavesmgr_isloadinprogress_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077416_cstatssavesmgr_issaveinprogress_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077417_cstatssavesmgr_load_estatsloadtype_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077418_cstatssavesmgr_cancelandclearsaverequests(ph: *mut Phantom); }
extern "C" { pub fn f_077430_moneyinterface_getvcwalletbalance_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077431_moneyinterface_getvcbankbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077432_moneyinterface_getevcbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077433_moneyinterface_getpvcbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077438_moneyinterface_getpxrvalue(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_077551_ctext_getsclanguagefromcurrentlanguagesetting(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077565_ctextfile_doestextlabelexist_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077742_cdriverpersonality_finddriverability_cped_const_ptr_cvehicle_con(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_077743_cdriverpersonality_finddriveraggressiveness_cped_const_ptr_cvehi(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_077801_cvehicleintelligence_getnodelist_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077803_cvehicleintelligence_getactivetask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077873_cpathfind_updatestreaming_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077970_cvehicleintelligence_getgototaskforvehicle_cvehicle_ptr_cphysica(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: f32, a6: f32) -> i64; }
extern "C" { pub fn f_077972_ctaskvehiclemissionbase_ctaskvehiclemissionbase_svehiclemissionp(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077973_ctaskvehiclegotosubmarine_ctaskvehiclegotosubmarine_svehiclemiss(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32) -> i64; }
extern "C" { pub fn f_077979_cvehicleintelligence_gethelitaskfrommissionidentifier_cvehicle_p(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32, a8: f32, a9: i32) -> i64; }
extern "C" { pub fn f_077980_cvehicleintelligence_getplanetaskfrommissionidentifier_cvehicle(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32) -> i64; }
extern "C" { pub fn f_077981_cvehicleintelligence_gettaskfrommissionidentifier_cvehicle_ptr_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: f32, a6: f32, a7: f32, a8: i32, a9: i32) -> i64; }
extern "C" { pub fn f_077988_ctaskvehiclefollow_ctaskvehiclefollow_svehiclemissionparams_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_077989_ctaskvehiclegotolongrange_ctaskvehiclegotolongrange_svehiclemiss(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077990_ctaskvehiclegotonavmesh_ctaskvehiclegotonavmesh_svehiclemissionp(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077991_ctaskvehiclegotoplane_ctaskvehiclegotoplane_svehiclemissionparam(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: f32) -> i64; }
extern "C" { pub fn f_077993_ctaskvehicleheliprotect_ctaskvehicleheliprotect_svehiclemissionp(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_077996_ctaskvehicleparknew_ctaskvehicleparknew_svehiclemissionparams_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i32) -> i64; }
extern "C" { pub fn f_078017_cheliintelligence_islandingareablocked_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078025_cvehiclenodelist_clearpathnodes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078068_ctaskvehicleconvertibleroof_latchroof_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078069_ctaskvehicleconvertibleroof_unlatchroof_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078085_ctaskvehiclemissionbase_setcruisespeed_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078191_ctaskvehicleescort_ctaskvehicleescort_svehiclemissionparams_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32, a5: f32) -> i64; }
extern "C" { pub fn f_078233_ctaskvehiclefollowwaypointrecording_ctaskvehiclefollowwaypointre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_078266_ctaskvehiclegotopointautomobile_ctaskvehiclegotopointautomobile(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_078336_ctaskvehiclelandplane_ctaskvehiclelandplane_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_078432_ctaskvehiclepursue_setidealdistance_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078472_ctaskbringvehicletohalt_ctaskbringvehicletohalt_float_int_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_078780_canchorhelper_isvehicleanchorable_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078782_canchorhelper_getanchorhelperptr_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078784_canchorhelper_anchor_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078785_canchorhelper_cananchorhere_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078827_cvehicle_setiswrecked(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078888_ccardoor_settargetdooropenratio_float_unsigned_int_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64); }
extern "C" { pub fn f_078918_cvehicle_setcardoorlocks_ecarlockstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078924_cvehicle_getwheelfromid_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_078937_ccardoor_setshutandlatched_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_078939_cvehicle_switchengineon_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_078955_cvehicle_turnsirenon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_078970_ctransmission_getturbopowermodifier_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_078978_cvehicle_fixwindow_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078986_cvehicle_getbrakeforce_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_078998_cvehicle_switchengineoff_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079023_cvehicle_getattachedtrailer_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079039_cvehicle_isturretseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079052_cvehicle_killpedsinvehicle_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079057_cvehicletrailerattachpoint_detachtrailer_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079058_cvehicle_trytomakepedsfrompretendoccupants_bool_int_const_ptr_ca(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_079065_cvehicle_getdoorfromid_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079082_cautomobile_resethydraulics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079097_ccardoor_setflag_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079130_cvehicle_getlayoutinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079134_cvehicle_getdriverseat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079152_cvehicle_updategadgetsafterteleport_rage_matrix34_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_079197_ccargenerator_switchoff(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079198_ccargenerator_switchon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079199_ccargenerator_isused_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079225_cthecargenerators_destroycargeneratorbyindex_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_079228_cvehicle_updatebodycolourremapping_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079230_cthecargenerators_createcargenerator_float_float_float_float_flo(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i32, a14: i32, a15: i32, a16: i32, a17: i32, a18: i64, a19: i32, a20: i32, a21: i32) -> i32; }
extern "C" { pub fn f_079234_cthecargenerators_setcargeneratorsactiveinarea_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079235_cthecargenerators_setallcargeneratorsbacktoactive(ph: *mut Phantom); }
extern "C" { pub fn f_079266_ccardoor_clearflag_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079267_ccardoor_getisintact_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079284_ccardoor_fix_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079285_ccardoor_audioupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079335_chandlingdata_gethandlingdata_ehandlingtype(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079347_chandlingdatamgr_gethandlingdata_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_079418_cheli_attachantiswaytoentity_cphysical_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_079422_cvehicle_getvehiclegadget_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079472_cplane_getverticalflightmodeavaliable_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079571_csubmarinehandling_forcethrottlefortime_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_079572_csubmarinehandling_getcurrentdepth_cvehicle_ptr_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_079575_csubmarinehandling_setcrushdepths_bool_float_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_079603_ctrailerlegs_updatetocurrentratio_cvehicle_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079614_ctrailer_debugattachtoparentvehicle_char_const_ptr_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: f32, a7: i32) -> i32; }
extern "C" { pub fn f_079616_cvehicletrailerattachpoint_getattachedtrailer_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_079623_ctrailer_setinversemassscale_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079653_ctrain_removealltrains(ph: *mut Phantom); }
extern "C" { pub fn f_079659_ctrain_findengine_ctrain_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079660_ctrain_disablerandomtrains_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_079661_ctrain_switchtraintrackfromscript_int_bool_rage_scrthreadid(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_079662_ctrain_settraintrackspawnfrequencyfromscript_int_int_rage_scrthr(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_079663_ctrain_createtrain_signed_char_int_bool_unsigned_char_ctrain_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_079669_ctrain_setcarriagespeed_ctrain_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079674_ctrain_findcarriage_ctrain_const_ptr_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079676_ctrain_setnewtrainposition_ctrain_ptr_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079746_cvehicle_doesvehiclehaveaconvertibleroofanimation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079758_cvehicle_getconvertibleroofstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079759_cvehicle_canbedeletedspecial_bool_bool_bool_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_079791_cvehicle_isdriver_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079825_cvehicle_removefragmentcomponent_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079832_cvehicle_setdoorlockstatefromdoorid_ehierarchyid_ecarlockstate(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079843_cvehicle_carhasroof_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079855_cvehicle_canbedriven_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079861_cwheel_applytyredamage_centity_const_ptr_float_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_079893_cvehicle_getwindownormalforsmash_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079902_cvehicle_setforwardspeed_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079903_cvehicle_setforwardspeedxy_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079904_cvehicle_placeonroadadjust_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_079907_cvehicle_updatehominglockedontostate_centity_ehominglockonstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079910_cvehicle_lowerconvertibleroof_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079911_cvehicle_raiseconvertibleroof_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079913_cvehicle_transformtosubmarine_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079914_cvehicle_transformtocar_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079997_cwheel_doburstwheeltorim(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079998_cwheel_resetdamage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080009_cvehicledamage_popoutwindscreen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080010_cvehicledamage_popoffroof_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080192_cvehiclefactory_setfardrawallvehicles_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080271_cvehiclegadgetwithjointsbase_getjoint_short(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080313_cvehiclegadgethandlerframe_detachcontainerfromframe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080412_cvehiclepopulation_dumpvehiclepooltolog(ph: *mut Phantom); }
extern "C" { pub fn f_080460_cvehiclepopulation_scriptgenerateoneemergencyservicescar_unsigne(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_080540_cvehiclemetadatamgr_getinvehicleoverrideinfo_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_081745_cvehicleglassmanager_smashcollision_centity_ptr_int_float_rage_v(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_081853_cvfxscript_getscriptptfxassetname_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082065_cpedweaponmanager_getequippedweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082071_cpedweaponmanager_getequippedweapon_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082183_cweapon_calcfirevecfrompos_centity_const_ptr_rage_matrix34_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_082215_cpedweaponmanager_dropweapon_unsigned_int_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_082223_cpedinventory_removeweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082519_cweaponinfo_getprojectilecanbeplaced_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082572_cpedinventory_addweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_082594_cpedweaponmanager_equipweapon_unsigned_int_int_bool_bool_cpedequ(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_082831_cnetworktelemetry_flushtelemetry_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082867_networkinterface_getexesize(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_082909_cnetworktelemetry_appendmetric_rage_rlmetric_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082932_cnetworkdebugtelemetry_appenddebugmetric_rage_rlmetric_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082938_networkinterface_islocalplayeronline_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082939_networkinterface_hasvalidroscredentials_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083008_networkinterface_iscloudavailable_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083011_networkinterface_hasvalidrockstarid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083014_networkinterface_getlocalgamerhandle(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083015_clivemanager_checkonlineprivileges_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083032_networkinterface_isoverridespectatedvehicleradioset(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083033_networkinterface_setoverridespectatedvehicleradio_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083038_networkinterface_isinfreemode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083041_networkinterface_canregisterobject_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083056_networkinterface_getplayerfromgamerhandle_rage_rlgamerhandle_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083061_networkinterface_getnetworkobject_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083073_networkinterface_clearusingextendedpopulationrange(ph: *mut Phantom); }
extern "C" { pub fn f_083087_cnetworkscriptworldstatemgr_revertworldstate_cnetworkworldstated(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083089_cnetworkscriptworldstatemgr_changeworldstate_cnetworkworldstated(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083100_cnetworkropeworldstatedata_cnetworkropeworldstatedata_cgamescrip(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_083101_networkinterface_deleteropeovernetwork_cgamescriptid_const_ref_i(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083105_cnetworkscenarioblockingareaworldstatedata_cnetworkscenarioblock(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32) -> i64; }
extern "C" { pub fn f_083106_cnetworkscenarioblockingareaworldstatedata_removescenarioblockin(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083112_networkinterface_settargetlockon_cvehicle_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083153_networkinterface_isremoteplayerinnonclonedvehicle_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083201_networkinterface_writedatapopulationlog_char_const_ptr_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_083202_networkinterface_hack_checkallplayersinteriorflag(ph: *mut Phantom); }
extern "C" { pub fn f_083350_cloudlistener_cloudlistener(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083469_achmgr_awardachievement_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083493_ctunables_access_unsigned_int_bool_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083641_ccommercemanager_isentitledtostarterpack(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083643_ccommercemanager_isentitledtopremiumpack(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083770_networkcolours_getnetworkcolour_networkcolours_networkcolour(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083771_networkcolours_getdefaultplayercolour_unsigned_char(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083874_cnetworksynchronisedscenes_getnetworksceneid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083886_cnetworksynchronisedscenes_setscenerate_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_083898_csyncedentity_csyncedentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083900_csyncedentity_setentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083902_csyncedentity_getentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083908_csyncedvehicle_getvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084068_achmgr_hasachievementbeenpassed_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084087_clivemanager_showsigninui_unsigned_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084088_clivemanager_issystemuishowing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084243_metricplayercoords_metricplayercoords(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084250_metricjob_ended_metricjob_ended_sjobinfo_const_ref_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_084287_metricsecchange_metricsecchange(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084309_cnetworktelemetry_setonlinejointype_unsigned_char(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084310_cnetworktelemetry_getmatchstarttime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084320_cnetworktelemetry_playerdied_cped_const_ptr_cped_const_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_084321_cnetworktelemetry_makerosbet_cped_const_ptr_cped_const_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32); }
extern "C" { pub fn f_084322_cnetworktelemetry_racecheckpoint_unsigned_int_unsigned_int_unsig(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_084323_cnetworktelemetry_missionover_char_const_ptr_unsigned_int_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_084324_cnetworktelemetry_missioncheckpoint_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_084325_cnetworktelemetry_randommissiondone_char_const_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_084326_cnetworktelemetry_multiplayerambientmissioncratedrop_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32); }
extern "C" { pub fn f_084327_cnetworktelemetry_multiplayerambientmissioncratecreated_float_fl(ph: *mut Phantom, a0: f32, a1: f32, a2: f32); }
extern "C" { pub fn f_084328_cnetworktelemetry_multiplayerambientmissionholdup_unsigned_int_u(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_084329_cnetworktelemetry_multiplayerambientmissionimportexport_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_084330_cnetworktelemetry_multiplayerambientmissionracetopoint_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_084332_cnetworktelemetry_shopitem_unsigned_int_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_084336_cnetworktelemetry_sessionmatchend(ph: *mut Phantom); }
extern "C" { pub fn f_084337_networkscpresenceutil_script_setpresenceattributeint_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_084339_cnetworktelemetry_cheatapplied_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084341_cnetworktelemetry_acquiredhiddenpackage_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084342_cnetworktelemetry_websitevisited_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_084343_cnetworktelemetry_friendsactivitydone_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_084344_cnetworktelemetry_changeweaponmod_unsigned_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_084345_cnetworktelemetry_changeplayerprop_unsigned_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_084346_cnetworktelemetry_changedplayercloth_unsigned_int_int_int_int_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_084349_cnetworktelemetry_appendsecuritymetric_metricsecchange_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084352_cnetworktelemetry_appendheistsavecheat_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_084355_cnetworktelemetry_appenddirectormode_metricdirector_mode_videocl(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084391_cnetworkdebugtelemetry_appendmetricmemstore_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084401_metricmemory_skeleton_metricmemory_skeleton_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_084402_cnetworkdebugtelemetry_appendmetricsmissionover_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084420_cprofilestats_setsynchronized_cprofilestats_esynctype_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084423_cprofilestats_synchronizedfailed_cprofilestats_esynctype_int_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_084424_cprofilestats_synchronized_cprofilestats_esynctype_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084427_cprofilestats_forcempprofilestatsgetfromcloud(ph: *mut Phantom); }
extern "C" { pub fn f_084514_networkscpresenceutil_updatecurrentmission_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084517_networkscpresenceutil_script_setpresenceattributefloat_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_084518_networkscpresenceutil_script_setpresenceattributestring_int_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_084519_networkscpresenceutil_script_setactivityattribute_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_085417_csocialclubinboxmgr_doapplyonevent_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085418_csocialclubinboxmgr_getisreadatindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085423_csocialclubinboxmgr_getmessageatindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_085424_csocialclubinboxmgr_setasreadatindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085425_csocialclubinboxmgr_geteventtypenameatindex_int_char_ptr_int_con(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_085427_getnestedmemberhelper_char_const_ptr_rage_rsonreader_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085428_csocialclubinboxmgr_getdatavalueformessage_int_char_const_ptr_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_085429_csocialclubinboxmgr_getdatavalueformessage_int_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_085436_rage_netgamepresenceevent_getobjectastype_rage_rsonreader_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085463_cbountypresenceevent_trigger_rage_rlgamerhandle_const_ptr_int_sc(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_085468_cgameawardpresenceevent_trigger_debug_unsigned_int_float_unsigne(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_085496_cstatupdatepresenceevent_trigger_unsigned_int_int_unsigned_int_c(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_085497_cstatupdatepresenceevent_trigger_unsigned_int_float_unsigned_int(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_085510_cugcstatupdatepresenceevent_trigger_rage_rlgamerhandle_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_085544_networkbaseconfig_getmatchmakinguser_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_085601_socialclubeventmgr_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_085602_socialclubeventmgr_doschedulerequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085606_socialclubeventmgr_iseventactive_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085607_socialclubeventmgr_iseventactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085608_socialclubeventmgr_getactiveeventcode_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085615_socialclubeventmgr_getextraeventdata_char_const_ptr_int_ref_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_085616_socialclubeventmgr_getextraeventdata_char_const_ptr_float_ref_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_085617_socialclubeventmgr_getextraeventdata_char_const_ptr_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_085618_socialclubeventmgr_getextraeventdata_int_char_const_ptr_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_085619_socialclubeventmgr_getextraeventdata_int_char_const_ptr_float_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_085620_socialclubeventmgr_getextraeventdata_int_char_const_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_085621_socialclubeventmgr_geteventdisplayname_char_ptr_int_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_085622_socialclubeventmgr_geteventdisplaynamebyid_int_char_ptr_int_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_085624_cnetworkemailmgr_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_085628_cnetworkemailmgr_setcurrentemailtag_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085629_cnetworkemailmgr_sendemail_unsigned_short_sscriptemaildescriptor(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_085630_cnetworkemailmgr_retrieveemails_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085631_cnetworkemailmgr_getemailatindex_int_sscriptemaildescriptor_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_085632_cnetworkemailmgr_deleteemails_int_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_085639_cnetworkscnewsstoryrequest_setstate_cnetworkscnewsstoryrequest_s(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085646_cnetworkscnewsstoryrequest_getsubtitle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085647_cnetworkscnewsstoryrequest_getcontent_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085648_cnetworkscnewsstoryrequest_getheadline_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085649_cnetworkscnewsstoryrequest_geturl_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085654_cnetworktransitionnewscontroller_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_085656_cnetworktransitionnewscontroller_skipcurrentnewsitem(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085658_cnetworktransitionnewscontroller_readyrequest_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085660_cnetworktransitionnewscontroller_initcontrol_int_cnetworktransit(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_085665_cnetworktransitionnewscontroller_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085666_cnetworktransitionnewscontroller_currentnewsitemhasextradata(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085667_cnetworktransitionnewscontroller_getcurrentnewsitemextradata_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_085668_cnetworkpausemenunewscontroller_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_085669_cnetworkpausemenunewscontroller_initcontrol_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085670_cnetworkpausemenunewscontroller_requeststory_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085672_cnetworkpausemenunewscontroller_ispendingstoryready_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085673_cnetworkpausemenunewscontroller_getpendingstory_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085674_cnetworkpausemenunewscontroller_getpendingstoryindex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085675_cnetworkpausemenunewscontroller_getnumstories(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085677_cnetworkpausemenunewscontroller_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086079_cnetworkreadleaderboards_clearcacheddisplaydata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086082_cnetworkreadleaderboards_readbygamer_unsigned_int_rage_rlleaderb(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i64) -> i64; }
extern "C" { pub fn f_086083_cnetworkreadleaderboards_readbyfriends_unsigned_int_rage_rlleade(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32) -> i64; }
extern "C" { pub fn f_086084_cnetworkreadleaderboards_clearcacheddisplaydatabyid_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086109_cnetworkcommunitystats_getcommunitystatdepthvalue_int_float_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_086230_packed_stats_getisbooleantype_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_086275_packed_stats_getisinttype_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_086276_packed_stats_getpackedvalue_int_ref_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_086277_packed_stats_setpackedvalue_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_086744_cnetgameplayer_getplayerped_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087291_cnetobjped_setistargettablebyteam_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087292_cnetobjped_setistargettablebyplayer_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087294_cnetobjped_setisdamagablebyplayer_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087750_cnetobjvehicle_lockforplayer_unsigned_char_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087751_cnetobjvehicle_lockforallplayers_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087752_cnetobjvehicle_islockedforplayer_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_087753_cnetobjvehicle_islockedforteam_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_087754_cnetobjvehicle_setpendingplayerlocks_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087755_cnetobjvehicle_evictallpeds(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087756_cnetobjvehicle_detonatephoneexplosive_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_089554_cscriptedgameevent_trigger_cgamescriptid_ref_void_ptr_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_089677_cclearpedtasksevent_trigger_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_089733_cscriptentitystatechangeevent_csettingoftaskvehicletempaction_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_089749_cscriptentitystatechangeevent_trigger_rage_netobject_ptr_cscript(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090817_gmtime_r(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090911_rand(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090918_sinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090920_snprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090921_siprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090930_strcmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090939_strlen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090940_strncasecmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
