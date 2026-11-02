//! Callee declarations for crate gen013.
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
extern "C" { pub fn f_000606_rage_voicechat_getpushtotalk(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000646_rage_athash64_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000668_rage_atarray_rage_atstring_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_000671_rage_atstring_split_rage_atarray_rage_atstring_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
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
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001138_rage_datgrowbuffer_getbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001140_rage_datbitbuffer_readunsigned_void_const_ptr_unsigned_int_ref_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001143_rage_datbitbuffer_datbitbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001147_rage_datbitbuffer_setreadonlybytes_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001156_rage_datbitbuffer_bumpreadcursor_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001210_rage_rsonwriter_begin_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001213_rage_rsonwriter_beginarray_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001214_rage_rsonwriter_end(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001216_rage_rsonwriter_writeint_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001218_rage_rsonwriter_writeint64_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001220_rage_rsonwriter_writeuns_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001224_rage_rsonwriter_writestring_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001229_rage_rsonwriter_writebool_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001233_rage_rsonreader_rsonreader_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001257_rage_rsonreader_readint_char_const_ptr_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001514_rage_diagdisplayf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001523_quitf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001524_displayf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001527_warningf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001534_quitf_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001535_rage_diaglogf2_rage_diagchannel_const_ref_rage_diagseverity_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001568_rage_atmapentry_unsigned_int_wchar_t_ptr_const_ref_rage_atmap_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001874_void_rage_atplaceptrcreator_void_ptr_rage_fideviceinstaller_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002129_rage_atmapentry_rage_atnonfinalhashstring_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002311_rage_atarray_rage_psoenumtableentry_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002335_unsigned_int_ref_rage_atmap_rage_parparsablestructure_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002739_rage_atpartialstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002743_rage_atdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002746_rage_stristr_char_ptr_char_const_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002843_rage_sysparam_getarray_float_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002881_unsigned_int_ref_rage_atmap_unsigned_int_unsigned_int_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002919_rage_sysservice_isconstrained_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002920_rage_sysservice_getconstraineddurationms_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002927_rage_sysmemstarttemp(ph: *mut Phantom); }
extern "C" { pub fn f_002928_rage_sysmemendtemp(ph: *mut Phantom); }
extern "C" { pub fn f_003097_char_ptr_rage_formatf_char_16ul_char_ref_16ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003835_rage_fwanimdirector_getmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003840_rage_fwclipsetmanager_getclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003871_rage_fwanimdirectorcomponentsyncedscene_dumpsynchronizedscenedeb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003886_rage_fwanimhelpers_getmovertrackmatrixdelta_rage_crclip_const_re(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64); }
extern "C" { pub fn f_003893_rage_fwanimhelpers_applyinitialoffsetfromclip_rage_crclip_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003895_rage_fwanimmanager_getclipifexistsbyname_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003896_rage_fwanimmanager_getclipifexistsbydictindex_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_003904_rage_fwanimmanager_findslot_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003905_rage_fwanimmanager_findslotfromhashkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003992_rage_fwclipsetmanager_getclipsetid_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003997_rage_fwclipsetmanager_getclipdictionarynameandclipname_rage_fwmv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_004010_rage_fwclipsetmanager_isstreamedin_deprecated_rage_fwmvtemplatei(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004280_rage_fwposematcher_fwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004619_rage_fwarchetype_getdrawable_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004620_rage_fwarchetype_getfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004632_rage_fwarchetypemanager_lookupmodelid_rage_fwarchetype_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004634_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004661_rage_fwdynamicentitycomponent_fwdynamicentitycomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004663_rage_fwdynamicentitycomponent_createattachmentextensionifmissing(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004685_rage_fwentity_getrootlod(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004740_rage_fwdecoratorextension_existson_rage_fwextensiblebase_ref_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004792_rage_fwpickermanager_getselectedentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004930_char_ptr_rage_formatf_char_10ul_char_ref_10ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_005461_rage_fwboxstreamer_hasloadedaboutpos_rage_vec3v_const_ref_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_005917_rage_fwrefawarebaseimpl_rage_datbase_dtorfwrefawarebaseimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005949_rage_aitasktree_findtaskbytypeactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006003_rage_fwpathserver_forceabortcurrentpathrequest_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006088_rage_atarray_rage_atnonfinalhashstring_0_unsigned_short_grow_uns(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007635_rage_atmapentry_void_ptr_unsigned_int_const_ref_rage_atmap_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007754_rage_netinterface_getnumactiveplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007755_rage_netinterface_getallactiveplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007756_rage_netinterface_flushalllogfiles_bool(ph: *mut Phantom); }
extern "C" { pub fn f_007759_rage_netinterface_getnumphysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007760_rage_netinterface_getnumremotephysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007762_rage_netinterface_getallphysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007763_rage_netinterface_getremotephysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007802_rage_netplayer_ismyplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007803_rage_networklogutils_writelogevent_rage_netlogginginterface_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007805_rage_netarraymanager_getarrayhandler_unsigned_short_rage_netarra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_007848_rage_neteventmgr_checkforspaceinpool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007849_rage_neteventmgr_prepareevent_rage_netgameevent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007855_rage_netplayer_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007949_rage_netgameevent_netgameevent_unsigned_short_bool_bool_rage_net(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_008056_rage_networklogutils_writeplayertext_rage_netlogginginterface_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_008059_char_ptr_rage_formatf_char_5ul_char_ref_5ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008070_rage_netobject_getplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008074_rage_netobject_ispendingownerchange_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008075_rage_netobject_islocalflagset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008076_rage_netobject_setlocalflag_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_008077_rage_netobject_isglobalflagset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008080_rage_netobject_hasbeencloned_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008172_rage_netobjectmgrbase_updateallinscopestateimmediately_rage_neto(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008208_rage_netobjectreassignmgr_isreassignmentinprogress_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008252_rage_netplayer_setvoicechannel_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008253_rage_netplayer_clearvoicechannel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008408_rage_netscgamerdatamgr_getcheaterreason_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_008409_rage_netscgamerdatamgr_hasautomuteoverride_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_008597_rage_cscscommerceconsumabletransaction_hassucceeded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008686_rage_fwbasepool_getindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008689_rage_fwscriptguid_createguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008693_rage_fwscriptguid_getbasefromguid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_008701_rage_scripthandlermgr_writescriptheader_rage_netlogginginterface(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_008702_rage_scripthandlernetcomponent_finishbroadcastingdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008704_rage_scripthandlernetcomponent_getslotparticipantisusing_rage_ne(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008705_rage_scripthandlernetcomponent_getstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008706_rage_scripthandler_getscriptobject_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008707_rage_scripthandler_registerexistingscriptobject_rage_scripthandl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008710_rage_scripthandler_getscriptresource_unsigned_short_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_008713_rage_scripthandler_getscriptresource_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008715_rage_scripthandler_getresourcereferencefromid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008716_rage_scripthandler_getnumscriptresourcesoftype_unsigned_short_co(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008717_rage_scripthandlernetcomponent_gethostbroadcastdatahandler_unsig(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008749_rage_scripthandlermgr_deletehandlerwithhash_rage_athashvalue(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008773_rage_scripthandlernetcomponent_isplayeraparticipant_unsigned_cha(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008778_rage_scripthandlernetcomponent_ishostlocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008781_rage_scripthandlernetcomponent_isplayeraparticipant_rage_netplay(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008825_rage_scripthandlernetcomponent_requesttobehost(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008840_rage_scriptinterface_isscriptregistered_rage_scriptidbase_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008841_rage_scriptinterface_isnetworkscript_rage_scriptidbase_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008952_rage_fwbasepool_getat_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009905_rage_monitorconfiguration_ismultihead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009908_rage_monitorconfiguration_update(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_010500_rage_iovalue_isdown_float_rage_iovalue_readoptions_const_ref_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010503_rage_iovalue_isup_float_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010511_rage_iovalue_ispressed_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010514_rage_iovalue_isreleased_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010518_rage_iovalue_getnorm_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010519_rage_iovalue_getunboundnorm_rage_iovalue_readoptions_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_011120_void_rage_atplaceptrcreator_void_ptr_rage_spropdetails_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011750_rage_fraginst_getchildbroken_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011759_rage_fraginst_breakoffabove_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_011762_rage_fraginst_breakoffroot(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011763_rage_fraginst_deleteabove_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011779_rage_fraginst_reducegroupdamagehealth_int_float_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_011811_rage_fragcachemanager_releasecacheentry_rage_fragcacheentry_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011876_rage_fragphysicslod_gettotalundamagedmass_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_012021_rage_fragphysicslodgroup_getlodbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012672_rage_snsession_isinvitable_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012675_rage_snsession_isjoinableviapresence_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012680_rage_snsession_ishost_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012728_rage_snsession_getsessiontoken_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012729_rage_snsession_reserveslots_rage_rlgamerhandle_const_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_012750_rage_snsession_isestablished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012752_rage_snsession_swapchannelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012776_rage_snsession_getmaxslots_rage_rlslottype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_012779_rage_snsession_getgamerinfo_rage_rlgamerhandle_const_ref_rage_rl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_012814_rage_textchat_setonsameteam_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_012816_rage_textchat_settextchatrecipients_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013620_rage_netipaddress_op_rage_netipaddress_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013645_rage_netp2pcrypt_key_dtorkey(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013859_rage_nethardware_getnattype(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_013958_rage_netpeeraddress_hasnetwork(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014067_rage_nettimesync_gettime_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014068_rage_nettimesync_hasstarted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014145_rage_neticetunneler_shouldshowconnectivitytroubleshooting(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014257_rage_is_ui_showing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014280_rage_rltelemetry_flushimmediate(ph: *mut Phantom); }
extern "C" { pub fn f_014284_rage_rlfriendsmanager_isfriendswith_int_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_014292_rage_rlgettaskmanager(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_014294_rage_rlgetposixtime_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014295_rage_rlcreateuuid_unsigned_long_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014303_rage_rlgamerhandle_op_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014316_rage_rlgamerinfo_getgamerhandle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014318_rage_rlgamerinfo_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014319_rage_rlgamerhandle_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014344_rage_rlfriendspage_getfriend_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014345_rage_rlfriend_getgamerhandle_rage_rlgamerhandle_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014346_rage_rlfriendspage_getfriendindex_char_const_ptr_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014349_rage_rlgamerhandle_touserid_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014350_rage_rlfriend_isonline_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014351_rage_rlfriend_isinsametitle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014352_rage_rlfriend_isinsession_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014361_rage_rlros_getcredentials_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014362_rage_rlroscredentials_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014368_rage_rlgamerhandle_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014370_rage_rlgamerhandle_tostring_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014371_rage_rlgamerhandle_fromstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014372_rage_rlgamerhandle_fromuserid_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014374_rage_rlgamerhandle_export_void_ptr_unsigned_int_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014375_rage_rlgamerhandle_import_void_const_ptr_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014383_rage_rlgamerinfo_hasstoreprivileges_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014384_rage_rlpresence_issignedin_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014387_rage_rlgamerinfo_rlgamerinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014392_rage_rlgamerinfo_getgamerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014400_rage_rlmatchingattributes_setvaluebyindex_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_014401_rage_rlmatchingattributes_getvaluebyindex_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_014445_rage_rlpresence_getgamerhandle_int_rage_rlgamerhandle_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_014448_rage_rlpresence_isguest_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014461_rage_rlclan_getprimaryclan_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014487_rage_rlsessioninfo_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014488_rage_rlprivileges_checkprivileges_int_int_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014489_rage_rlprivileges_isprivilegecheckresultready(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014490_rage_rlprivileges_isprivilegecheckinprogress(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014491_rage_rlprivileges_isprivilegechecksuccessful(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014492_rage_rlprivileges_setprivilegecheckresultnotneeded(ph: *mut Phantom); }
extern "C" { pub fn f_014501_rage_rlsessioninfo_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014580_rage_rlsessioninfo_rlsessioninfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014605_rage_rlhttptask_rlhttptask_rage_sysmemallocator_ptr_unsigned_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_014609_rage_rlstats_cancel_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014624_rage_rlsystemui_showwebbrowser_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014662_rage_rlmetric_dtorrlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014685_rage_rlmetric_rlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014775_rage_rlprofilestats_readstatsbygamer_int_rage_rlgamerhandle_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_014789_rage_rlprofilestatsvalue_rlprofilestatsvalue(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014791_rage_rlprofilestatsvalue_getfloat_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_014792_rage_rlprofilestatsvalue_getint32_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014793_rage_rlprofilestatsvalue_getint64_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014803_rage_rlprofilestatsreadresults_getgamerhandle_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_014805_rage_rlprofilestatsreadresults_getvalue_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_014836_rage_rlroscredentials_hasprivilege_rage_rlrosprivilegeid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014859_rage_rlros_getdefaultprivilege_rage_rlrosprivilegeid(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014886_rage_rlroscredentials_hasprivilegeenddate_rage_rlrosprivilegeid(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_014977_rage_rlugccontenttypefromstring_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014988_rage_rlugcplayer_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015039_rage_rlcloudmemberid_rlcloudmemberid_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015040_rage_rlcloudmemberid_rlcloudmemberid_rage_rlgamerhandle_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015090_rage_rlclan_hasprimaryclan_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_015091_rage_rlclan_getprimarymembership_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_015106_rage_rlclan_getprimaryclans_int_rage_rlgamerhandle_const_ptr_uns(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_015128_rage_rlclandesc_rlclandesc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015213_rage_rlgamerquerydata_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015568_char_ptr_rage_formatf_char_25ul_char_ref_25ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016010_rage_phsimulator_getlastinstancematrix_rage_phinst_const_ptr_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016011_rage_phsimulator_setlastinstancematrix_rage_phinst_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016087_rage_phinst_setarchetype_rage_pharchetype_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016515_rage_phboundgeometry_calculatepolynormals(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016944_rage_scrthread_preprintstacktrace(ph: *mut Phantom); }
extern "C" { pub fn f_016949_rage_scrthread_getactivethread(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_016950_rage_scrthread_isthisthreadrunningascript(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_016962_rage_scrthread_registerunusedcommand_unsigned_long_void_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016967_rage_scrthread_registercommand_unsigned_long_void_ptr_rage_scrth(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016981_rage_scrthread_getthread_rage_scrthreadid(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_017082_rage_scrthread_info_getvector3_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018883_cgamelogic_resurrectlocalplayer_rage_vector3_const_ref_float_boo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_018890_cgamelogic_forcestateplaying(ph: *mut Phantom); }
extern "C" { pub fn f_018904_cgarage_isgarageempty_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_018907_cgarage_isentityentirelyoutside_centity_const_ptr_float_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_018908_cgarage_isplayerentirelyinsidegarage_centity_const_ptr_float_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_018911_cgarage_findallobjectsingarage_rage_atarray_centity_ptr_0_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_018912_cgarages_isanyentityentirelyinsidegarage_int_bool_bool_bool_floa(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_018931_cgarages_findgarageindexfromnamehash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_018934_cgarage_isobjectentirelyinsidegarage_centity_const_ptr_float_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_018936_cgarages_closesafehousegarages(ph: *mut Phantom); }
extern "C" { pub fn f_018960_cgarages_isplayerpartiallyinsidegarage_int_centity_const_ptr_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_018961_cgarages_areentitiesentirelyinsidegarage_int_bool_bool_bool_floa(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_018962_cgarages_isobjectpartiallyinsidegarage_int_centity_const_ptr_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_018963_cgarage_isgarageemptyonallmachines_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_018978_cgpsslot_start_rage_vector3_ref_centity_ptr_int_bool_int_rage_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_018985_cgpsslot_getnextnodepositionalongroute_bool_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_019000_cgps_getroutelength_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_019188_cnetrespawnmgr_findlargestbunchofplayers_bool_rage_vector3_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_019189_cnetrespawnmgr_startsearch_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_019190_cnetrespawnmgr_startsearch_cnetrespawnmgr_startsearchinput_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019191_cnetrespawnmgr_cancelsearch(ph: *mut Phantom); }
extern "C" { pub fn f_019194_cnetrespawnmgr_getsearchresults_int_rage_vector3_ref_float_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_019195_cnetrespawnmgr_getsearchresultflags_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_019451_ailogging_getbooleanasstring_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_019453_ailogging_getdynamicentitynamesafe_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019458_ailogging_stacktracelog_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019621_cmoveanimatedbuilding_setphase_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_019622_cmoveanimatedbuilding_setrate_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_019623_cmoveanimatedbuilding_getphase(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_019624_cmoveanimatedbuilding_getrate(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_019634_cmoveped_settasknetwork_rage_fwmovenetworkplayer_ptr_float_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_019733_cclipeventtags_crandomizestartphaseproperty_pickrandomstartphase(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_019734_cclipeventtags_crandomizerateproperty_pickrandomrate_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_020083_rage_imp_untransformortho_imp34_rage_mat34v_ref_float_vector_4_f(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: u128, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_021359_auddamagestats_dtorauddamagestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021399_audpedaudioentity_setcachedbjvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022381_void_creplayeventmanager_storeevent_cpacketsoundphonering_cpacke(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_023396_cpacketevent_cpacketevent_ereplaypacketid_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023870_creplaymgr_recordmapobject_cobject_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023877_creplaymgr_recordobject_cobject_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023906_creplaycontrol_isplayeroutofcontrol(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023930_creplayeventmanager_shouldproceedwithpacket_cpacketevent_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023998_rage_fwmodelid_fwmodelid_rage_fwmodelid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_024266_creplaymgrinternal_isawareofworldforrecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026367_rage_imp_transform_imp34_rage_mat34v_ref_float_vector_4_float_ve(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: u128, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_026702_ccontrol_debugoutputscriptcallstack(ph: *mut Phantom); }
extern "C" { pub fn f_029335_cgamesessionstatemachine_reinit_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_029377_ctask_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029386_cpedvariationinfocollection_getcollectiondrawable_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_029433_getorcreategroup_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029453_cpedvariationinfocollection_getdlcnamefromdrawableidx_epedvarcom(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_029455_cdebugscene_tunewidgetdata_tunewidgetdata_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_029456_cdebugscene_queuetunewidget_cdebugscene_tunewidgetdata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029967_cphysical_gethealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_030722_ceventgroupped_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030723_cevent_affectsped_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_030746_geteventscriptnetworkgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030814_cpeddamagecalculator_cpeddamagecalculator_centity_const_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_030815_cpeddamagecalculator_applydamageandcomputeresponse_cped_ptr_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32); }
extern "C" { pub fn f_030822_ceventrequesthelp_dtorceventrequesthelp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030828_ceventdamage_ceventdamage_centity_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_030829_ceventdamage_dtorceventdamage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030931_ceventnetworkcashtransactionlog_ceventnetworkcashtransactionlog(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_031125_ceventshouttargetposition_ceventshouttargetposition_cped_ptr_cpe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031169_ceventswitch2nm_ceventswitch2nm_unsigned_int_rage_aitask_ptr_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_031170_ceventswitch2nm_dtorceventswitch2nm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031650_ceventexplosionheard_getsourcepos_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034950_chudtools_getaspectratio_bool(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_035253_centitypoolindexforblip_centitypoolindexforblip_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_035264_cminimap_getblipfromactualid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_035266_cminimap_setflag_cminimapblip_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035267_cminimap_createblip_bool_eblip_type_centitypoolindexforblip_cons(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_035272_cminimap_removeblip_cminimapblip_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035317_cminimap_setblipparameter_eblip_params_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_035323_cminimap_isblipidinuse_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035324_cminimap_getblip_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_035328_cminimap_getblipattachedtoentity_centity_const_ptr_eblip_type_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_036034_cscaleformmoviewrapper_isactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036175_cprofilesettings_set_cprofilesettings_profilesettingid_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036254_cpausemenu_isactive_pm_activeflags(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036263_cpausemenu_getactualscreen_menuscreenid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036265_cpausemenu_getcurrentscreen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036274_cpausemenu_isstoreavailable(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036364_cprofilesettings_getint_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036382_cprofilesettings_exists_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036386_cprofilesettings_write_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036627_cwarningscreen_setmessage_ewarningmessagestyle_rage_atnonfinalha(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_036666_cstoretexturemanager_releasetexture_rage_atstring_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036670_cstoretexturemanager_requesttexture_rage_atstring_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036672_cstorescreenmgr_setinitialitemid_rage_atstring_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036680_metricbaseinfo_metricbaseinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036681_metricbasechar_metricbasechar(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036695_cstorescreenmgr_resetnetworkgametracking(ph: *mut Phantom); }
extern "C" { pub fn f_036704_cstorescreenmgr_setbasecategory_rage_atstring_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036705_cstoretexturemanager_freealltextures(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036706_cstorescreenmgr_delaystoreopen(ph: *mut Phantom); }
extern "C" { pub fn f_036707_cstorescreenmgr_populatedisplayvalues(ph: *mut Phantom); }
extern "C" { pub fn f_036708_cstorescreenmgr_checkoutcommerceproduct_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037383_gametransactionsessionmgr_gametransactionsessionmgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037386_metricbaseinfo_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_037387_metricbasechar_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_037439_cfrontendstatsmgr_formatint64tocash_long_char_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_037662_cmodelinfo_haveassetsloaded_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037676_cclock_settime_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_037685_cclock_fromdayratio_float_int_ref_int_ref_int_ref(ph: *mut Phantom, a0: f32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_037888_cseatmanager_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038053_cwantedresponseoverrides_getinstance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_038054_cwantedresponseoverrides_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038055_cwantedresponseoverrides_setnumpedstospawn_dispatchtype_signed_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_038119_cdispatchspawnproperties_getinstance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_038121_cdispatchspawnproperties_addangledblockingarea_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_038122_cdispatchspawnproperties_addsphereblockingarea_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_038123_cdispatchspawnproperties_removeblockingarea_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038124_cdispatchspawnproperties_resetblockingareas(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038382_cpedintelligence_addtaskdefault_rage_aitask_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038431_backgroundscripts_requestcloudfile(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038441_backgroundscripts_iscloudrequestpending(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038751_cseatmanager_getpedsseatindex_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038757_cseatmanager_getlastpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039026_cvehiclemodelinfo_getcanpassengersbeknockedoff(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039576_cdoor_getdooropenratio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_039591_cdoorsystem_setpendingstateondoor_cdoorsystemdata_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039597_cdoor_openallbarriersforrace_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_039636_cdoor_isdoorshut_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039640_cdoor_changedoorstateforscript_unsigned_int_rage_vector3_const_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: f32, a6: i32, a7: i32, a8: f32, a9: f32, a10: i32, a11: i32) -> i32; }
extern "C" { pub fn f_039641_cdoorsystem_setlocked_rage_vector3_const_ref_bool_float_bool_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32, a4: i32, a5: f32, a6: f32, a7: i64, a8: i32); }
extern "C" { pub fn f_039642_cdoor_getscriptsearchradius_cbasemodelinfo_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_039643_cdoor_findclosestdoor_cbasemodelinfo_ptr_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_039644_cobject_getpointertoclosestmapobjectforscript_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_039647_cdoor_closeallbarriersopenedforrace(ph: *mut Phantom); }
extern "C" { pub fn f_039649_cdoorsystem_adddoor_int_int_rage_vector3_const_ref(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_039652_cdoorsystemdata_setpendingstate_doorscriptstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039654_cdoorsystem_getdoordata_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_039663_cdoorsystemdata_setflagforremoval(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039666_cdoorsystemdata_setholdopen_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_039667_cdoorsystemdata_getdoorstatename_doorscriptstate(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_039688_cobjectpopulation_destroyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039697_cobject_ispressureplatepressed(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039699_cobject_setdrivetotarget_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_039701_cobject_isjointatmaxangle_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039702_cobject_isjointatminangle_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039709_cobject_playautostartanim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039729_cobject_setobjecttargettable_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039746_cobject_getcsetintfromobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039748_cobject_setobjectisball_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039762_cobjectpopulation_createobject_cobjectpopulation_createobjectinp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039765_cobjectpopulation_converttorealobject_cdummyobject_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039882_cpathserver_getclosestpositionforped_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_039888_cpathserver_addnavmeshregion_rage_navmeshrequiredregion_rage_scr(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_039889_cpathserver_getnavmeshmeshregionscriptid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_039890_cpathserver_removenavmeshregion_rage_navmeshrequiredregion_rage(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_039891_cpathserver_areallnavmeshregionsloaded_rage_ainavdomain(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_039893_cpathserver_getnavmeshindexfromposition_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039941_cpathserver_switchnavmeshinarea_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_039942_cpathserver_switchpedsoffinarea_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_039997_cpathservergta_addscriptedblockingobject_rage_scrthreadid_rage_v(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_039998_cpathservergta_updatescriptedblockingobject_rage_scrthreadid_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_040000_cpathservergta_doesscriptedblockingobjectexist_rage_scrthreadid(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_040001_cpathservergta_removescriptedblockingobject_rage_scrthreadid_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_040048_cpathserver_clearpedswitchareas_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040139_cpedgroupmembership_getmember_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_040140_mapscriptableformationtopedformation_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_040141_cpedformation_setspacing_float_float_float_rage_scrthreadid(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_040142_cpedgroup_islocallycontrolled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040150_cpedgroupmembership_countmembersexcludingleader_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040166_cpedgroupmembership_removemember_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_040169_cpedgroupmembership_removemember_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040170_cpedgroupmembership_setmaxseparation_float_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_040171_cpedgroupmembership_setleader_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040172_cpedgroupmembership_addfollower_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040174_cpedgroupmembership_ismember_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040177_cpedgroup_isplayergroup_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040180_cpedgroup_setformation_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_040181_cpedgroup_getnetarrayhandlerindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040182_cpedgroup_process(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040194_cpedgroups_getpedsgroup_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040355_cactionmanager_enableaction_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_040404_rage_fraginstnm_postartmessage_char_const_ptr_art_messageparamsb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_040503_art_messageparamsbase_messageparamsbase_art_messageparamsbase_pa(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041056_findplayerped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041077_cpedtargetting_getlosstatus_centity_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041116_cpedintelligence_flushimmediately_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041117_cpedintelligence_addtoeventhistory_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041197_cpedintelligence_setdefaultrelationshipgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041202_cpedintelligence_setrelationshipgroup_crelationshipgroup_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041203_cpedintelligence_getrelationshipgroup_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041237_cpedintelligence_gettaskmelee_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041281_cpedtype_getpedtypenamefromid_epedtype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_041287_cpedpopulation_removepedfrompopulationcount_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041296_cpedfactory_destroyped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041335_cpedintelligence_ispedswimming_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041378_cpedintelligence_isrespondingtoevent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041388_cpedintelligence_ispedinair_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041389_cpedintelligence_ispedclimbing_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041390_cpedintelligence_ispedvaulting_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041391_cpedintelligence_ispedfalling_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041392_cpedintelligence_ispedlanding_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041393_cpedintelligence_ispeddiving_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041403_cpedintelligence_setrelationshipgroupdefault_crelationshipgroup(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041413_cpedintelligence_restartnavigationthroughregion_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_041436_cpedintelligence_forcepedpinneddown_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_041460_cpedplacement_findzcoorforped_float_rage_vector3_ptr_centity_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_041467_cpedpopulation_choosecivilianpedmodelindexargs_choosecivilianped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041473_cpedpopulation_pednoncreationareaclear(ph: *mut Phantom); }
extern "C" { pub fn f_041474_cpedpopulation_setallowcreaterandomcops_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041475_cpedpopulation_setallowcreaterandomcopsonscenarios_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041511_cpedpopulation_setadjustpedspawnpointdensitythisframe_int_rage_v(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_041514_cpedpopulation_setscriptambientpeddensitymultiplierthisframe_flo(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_041515_cpedpopulation_setscriptedrangemultiplierthisframe_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_041516_cpedpopulation_setscriptscenariopeddensitymultipliersthisframe_f(ph: *mut Phantom, a0: f32, a1: f32); }
extern "C" { pub fn f_041517_cpedpopulation_setsuppressambientpedaggressivecleanupthisframe(ph: *mut Phantom); }
extern "C" { pub fn f_041525_cpedpopulation_addpedincar_cvehicle_ptr_bool_int_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64) -> i64; }
extern "C" { pub fn f_041529_cpedpopulation_addpedtopopulationcount_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041539_cpedpopulation_choosecivilianpedmodelindex_cpedpopulation_choose(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041551_cpedpopulation_scriptedconversioncentresetthisframe_rage_vector3(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041552_cpedpopulation_pednoncreationareaset_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041558_cpedpopulation_instantfillpopulation(ph: *mut Phantom); }
extern "C" { pub fn f_041698_cplayerinfo_setmaxportablepickupsoftypepermitted_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041705_cplayerinfo_setplayerstate_cplayerinfo_state(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041746_reportplayerresetmismatch_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_041853_cpedailodmanager_setuseactivecamerafortimeslicingcentre(ph: *mut Phantom); }
extern "C" { pub fn f_041927_cplayerpedtargeting_setlockontarget_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041998_cqueriableinterface_gettargetfortasktype_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042003_cqueriableinterface_gettaskinfofortasktype_int_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042004_cqueriableinterface_getstatefortasktype_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042025_crelationshipgroup_setrelationship_erelationtype_crelationshipgr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_042026_crelationshipgroup_clearrelationship_crelationshipgroup_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042039_crelationshipmanager_removerelationshipgroup_crelationshipgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042040_crelationshipmanager_findrelationshipgroup_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_042409_cpedheadblenddata_cpedheadblenddata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042412_cpedheadblenddata_dtorcpedheadblenddata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042415_cpedvariationinfo_getmaxnumdrawbls_epedvarcomp_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042436_cpedvariationpack_getpalettevar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042437_cpedvariationpack_gettexvar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042438_cpedvariationpack_getcompvar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042442_cpedvariationinfocollection_getmaxnumpropstex_eanchorpoints_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_042445_cpedvariationinfocollection_getmaxnumprops_eanchorpoints_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042471_cpedvariationstream_customitoa3_unsigned_int_char_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_042559_cpedpropsmgr_setpedpropsrandomly_cped_ptr_float_float_unsigned_i(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32, a4: i64, a5: i64, a6: i64, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_042686_peddecorationmanager_addpeddecoration_cped_ptr_rage_atnonfinalha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32); }
extern "C" { pub fn f_042786_cpickupplacement_setamount_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042787_cpickup_sethidewhendetached_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042788_cpickup_setportable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042794_cpickup_attachportablepickuptoped_cped_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_042806_cpickup_detachportablepickupfromped_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042811_cpickupdata_getreward_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042819_cpickupmanager_calculatedroppedpickupposition_cpickupmanager_cpi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042827_cpickupmanager_iscollectionprohibited_unsigned_int_unsigned_char(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_042850_cpickupmanager_issuppressionflagset_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_042871_cpickupmanager_renderscriptedglow_rage_vec3v_const_ref_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042872_cpickupmanager_moveportablepickupsfromonepedtoanother_cped_ref_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042873_cpickupmanager_detachallportablepickupsfromped_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042874_cpickupmanager_cpickuppositioninfo_cpickuppositioninfo_rage_vect(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_042879_cpickupmanager_removepickupplacement_cpickupplacement_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042884_cpickupmanager_registerpickupplacement_unsigned_int_rage_vector3(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_042885_cpickupmanager_createpickup_unsigned_int_rage_matrix34_const_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_042892_cpickupmanager_haspickupbeencollected_cgamescriptobjinfo_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_042895_cpickupmanager_removeallpickupsoftype_unsigned_int_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_042897_cpickupmanager_createsomemoney_rage_vector3_const_ref_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_042898_cpickupmanager_removeprohibitedcollection_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_042900_cpickupmanager_prohibitpickupmodelcollectionforlocalplayer_unsig(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_042901_cpickupmanager_removeprohibitedpickupmodelcollection_unsigned_in(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_042919_cpickupplacement_setuncollectable_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042920_cpickupplacement_settransparentwhenuncollectable_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042921_cpickupplacement_sethiddenwhenuncollectable_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042922_cpickupplacement_setregenerationtime_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042923_cpickupplacement_setpickupglowoffset_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_043294_cragdollconstraintcomponent_enablehandcuffs_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_043295_cragdollconstraintcomponent_enableboundankles_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_043335_worldprobe_debugfindgroundzforcoord_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_043464_meshblendmanager_getrampcolor_eramptype_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_043508_occlusionqueries_oqgetqueryresult_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_044572_clightentity_getlight_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045248_cgenericgamestorage_queueloadmostrecentsave(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045797_cped_getcurrentmotiontask_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045805_cped_deletevehicleentryconfig(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045815_cped_releasecoverpoint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045844_cped_setdefaultdecisionmaker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045861_cped_updateragdollroottransformfromanimatedskel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045876_cped_setuseextractedz_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045883_cped_setpedoutofvehicle_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_045885_cped_switchtoragdoll_cevent_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045886_cped_setmyvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045889_cped_getprimarymotiontask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045903_cped_getiscrouching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045904_cped_setiscrouching_bool_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_045939_cped_setvarrandom_unsigned_int_unsigned_int_rage_atfixedbitset_3(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_045940_cped_setvariation_epedvarcomp_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_045941_cped_isvariationvalid_epedvarcomp_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_045943_cped_setvardefault_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045945_cped_setheadblenddata_cpedheadblenddata_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045946_cped_setheadoverlay_eheadoverlayslots_unsigned_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_045947_cped_requestblendfromparents_cped_ptr_cped_ptr_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32); }
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
extern "C" { pub fn f_045961_cped_finalizeheadblend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045977_cped_setdesiredheading_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_045996_cped_getbonetagfromragdollcomponent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046016_cped_getisshelteredfromrain(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046024_cped_updatemovementmode_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046055_cped_setmovementmodeoverridehash_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046057_cped_updateragdollboundsfromanimatedskel_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046083_cped_forceragdollpresetup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046125_cped_setforcedseatusage_int_int_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_046126_cped_clearforcedseatusage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046144_cped_newsay_char_const_ptr_unsigned_int_bool_bool_int_cped_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32) -> i32; }
extern "C" { pub fn f_046152_cped_setpedinvehicle_cvehicle_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_046168_cped_isprone_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046174_cped_setmaxtimeunderwater_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046180_cped_instantaiupdate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046183_cped_getragdollcomponentfrombonetag_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046192_cped_isdebuginvincible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046326_cgameworldheightmap_getmaxheightfromworldheightmap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_046368_cdatafilemgr_finddatafile_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_046396_cscriptdownloadabletexturemanager_requestmembertexture_rage_rlga(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_046398_cscriptdownloadabletexturemanager_requesttitletexture_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046399_cscriptdownloadabletexturemanager_requestugctexture_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_046400_cscriptdownloadabletexturemanager_releasetexture_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046401_cscriptdownloadabletexturemanager_hastexturefailed_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046402_cscriptdownloadabletexturemanager_gettexturename_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046410_centity_getinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046438_cgameworld_add_centity_ptr_rage_fwinteriorlocation_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046442_cinteriorproxy_getfromlocation_rage_fwinteriorlocation_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046445_cportaltracker_collectstack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046458_cdynamicentity_getthreadthatcreatedme_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046467_cgameworld_remove_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046515_centity_changevisibilitymask_unsigned_short_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046522_centity_iswithinarea_float_float_float_float_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32) -> i32; }
extern "C" { pub fn f_046523_centity_iswithinarea_float_float_float_float_float_float_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32) -> i32; }
extern "C" { pub fn f_046545_centity_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046599_centityiterator_centityiterator_int_centity_const_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_046600_centityiterator_getnext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047044_cgameworld_forallentitiesintersecting_rage_fwsearchvolume_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_047060_cphysical_activatephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047072_cphysical_setmass_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047074_cphysical_changehealth_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047077_cphysical_getweapondamagedtimebyhash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047126_cphysical_debugprocessallattachments_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_047130_cphysical_getdistancefromcentreofmasstobaseofmodel_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_047305_cgameworld_getmainplayerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047386_ccompentity_releasestreamrequests(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047388_ccompentity_settostarting(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047391_ccompentity_issueassetrequests_ecompentityassetset(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047395_ccompentity_settoending(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047396_ccompentity_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047398_ccompentity_triggeranim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047399_ccompentity_pauseanimat_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_048037_cgameworldheightmap_getminheightfromworldheightmap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_048042_cgameworldheightmap_getminheightfromworldheightmap_float_float_f(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_048043_cgameworldheightmap_getmaxheightfromworldheightmap_float_float_f(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_048339_networkutils_isnetworkcloneormigrating_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_052868_misc_commands_commandcleararea_rage_scrvector_const_ref_float_bo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_052870_misc_commands_commandclearareaofvehicles_rage_scrvector_const_re(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_052872_misc_commands_commandclearareaofobjects_rage_scrvector_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_052873_misc_commands_commandclearareaofpeds_rage_scrvector_const_ref_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_052998_misc_commands_commanddisplayonscreenkeyboardwithlongerinitialstr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i32); }
extern "C" { pub fn f_053007_misc_commands_commandsetexplosiveammothisframe_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053008_misc_commands_commandsetfireammothisframe_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053009_misc_commands_commandsetexplosivemeleethisframe_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053393_money_commands_ishandlevalid_int_ref_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_053394_money_commands_earncash_long_bool_bool_rage_rlmetric_ptr_statid(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_053395_money_commands_spendcash_long_bool_bool_bool_bool_rage_rlmetric(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64, a7: i32, a8: i32, a9: i32, a10: i32) -> i32; }
extern "C" { pub fn f_053397_money_commands_initplayercash_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053398_money_commands_clearcharacterwallet_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053399_money_commands_manualdeletecharacter_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053400_money_commands_deletecharacter_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053401_money_commands_cansharejobcash(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_053402_money_commands_giveplayersharedjobcash_int_int_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_053403_money_commands_receiveplayersharedjobcash_int_int_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_053404_money_commands_earnmoneyfrompickup_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053405_money_commands_earnmoneyfromcashingout_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053406_money_commands_earnmoneyfromgangattackpickup_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053407_money_commands_earnmoneyfromassassinatetargetkilled_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053408_money_commands_earnmoneyfromarmoredrobbery_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053409_money_commands_commandearnspinthewheelcash_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053410_money_commands_commandearnrctimetrial_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053411_money_commands_commandearndailyobjectiveevent_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053412_metricearnfromdailyobjective_metricearnfromdailyobjective_int_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_053413_money_commands_earnmoneyfromcratedrop_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053414_money_commands_earnmoneyfrombetting_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_053415_metricearnmatchid_metricearnmatchid_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_053416_money_commands_earnmoneyfromjob_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_053417_money_commands_earnmoneyfromjobx2_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_053418_money_commands_earnmoneyfrompremiumjob_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_053419_money_commands_earnmoneyfromjobbonuses_int_char_const_ptr_char_c(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_053420_money_commands_earnmoneyfromgangops_int_char_const_ptr_int_char(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_053425_money_commands_earnmoneyfrombendjob_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_053426_money_commands_earnmoneyfromchallengewin_int_char_const_ptr_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_053427_money_commands_earnmoneyfrombounty_int_int_ref_int_ref_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_053428_money_commands_earnmoneyfromimportexport_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053429_money_commands_earnmoneyfromholdups_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053430_money_commands_earnmoneyfromproperty_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053431_money_commands_earnmoneyfromaitargetkill_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053432_money_commands_earnmoneyfromnotbadsport_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053433_money_commands_earnmoneyfromrockstar_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053434_money_commands_earnmoneyfromsellingvehicle_int_int_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_053435_metricearnfromvehicle_metricearnfromvehicle_long_int_bool_int_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32) -> i64; }
extern "C" { pub fn f_053436_money_commands_earnmoneyfromsellingpersonalvehicle_int_int_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_053437_money_commands_earnfromdailyobjectives_int_char_const_ptr_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_053438_money_commands_earnfromambientjob_int_char_const_ptr_srcambientj(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_053439_money_commands_commandearnpurchaseclubhouse_int_srcclubhousedata(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_053440_money_commands_commandearnfrombusinessproduct_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053441_money_commands_commandearnfromvehicleexport_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053442_money_commands_commandearnfrombountyhunterreward_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053443_money_commands_commandearnfrombusinessbattle_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053444_money_commands_commandearnfrombusinesshubsell_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053445_money_commands_commandearnfromclubmanagementparticipation_int_in(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053446_money_commands_commandearnfromfmbbphonecallmission_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053447_money_commands_commandearnfromfmbbbosswork_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053448_money_commands_commandearnfromfmbbwagebonus_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053449_money_commands_refundcash_int_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_053450_money_commands_deductcash_int_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_053451_money_commands_canspendgtadollars_int_bool_bool_bool_int_ref_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_053454_money_commands_buyitem_int_int_int_int_bool_char_const_ptr_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_053455_metricspendshop_metricspendshop_int_bool_unsigned_int_int_int_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32) -> i64; }
extern "C" { pub fn f_053456_money_commands_spenttaxi_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053457_money_commands_commandspendrehiredj_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053458_money_commands_commandspendarenajoinspectator_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053459_money_commands_commandspendarenaspectatorbox_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053460_money_commands_commandspendspinthewheelpayment_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053461_money_commands_commandspendarenawarspremiumevent_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053462_money_commands_commandearnarenawar_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053463_metricearngeneric_metricearngeneric_char_const_ptr_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_053464_money_commands_commandearnarenawarassassinatetarget_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053465_money_commands_commandearnarenawareventcargo_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053466_money_commands_commandspendcasinomembership_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053467_money_commands_payemployeewage_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053468_money_commands_paymatchentryfee_int_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_053469_metricspendmatchid_metricspendmatchid_int_bool_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_053470_money_commands_commandmoneycanbet_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_053471_money_commands_canbet_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_053472_money_commands_commandcasinocanbet_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_053474_money_commands_commandcasinocanbetamount_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_053475_money_commands_verify_chips_use(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_053476_money_commands_commandcasinobuychips_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_053477_money_commands_commandcasinosellchips_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_053478_money_commands_spentbetting_int_int_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_053479_money_commands_spentwager_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053480_money_commands_commandspendnightclubentryfee_int_int_int_bool_bo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053481_money_commands_commandspendnightclubbardrink_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053482_money_commands_commandspendbountyhuntermission_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053483_money_commands_spentonfairgroundrides_int_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053484_money_commands_spentonjobskip_int_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_053485_money_commands_spentonboss_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_053486_money_commands_command_canpayamounttobossorgoon_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_053487_money_commands_command_earngoon_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053488_money_commands_command_earnboss_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053489_money_commands_command_earnagency_int_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053490_money_commands_commandearnfromsmuggleragency_int_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053491_money_commands_commandearnmoneyfromwarehouse_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053492_money_commands_commandearnmoneyfromcontraband_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053493_money_commands_commandearnmoneyfromdestroyingcontraband_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053494_money_commands_commandearnmoneyfromsmugglerwork_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053495_money_commands_commandearnmoneyfromhangartrade_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053496_money_commands_spentonhangarproperty_int_char_const_ptr_srcspend(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_053497_metricspent_hangarpurchandupdate_metricspent_hangarpurchandupdat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_053500_money_commands_commandspenthangarutilitycharges_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053501_money_commands_commandspenthangarstaffcharges_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053502_money_commands_command_spentgoon_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053503_money_commands_command_spentboss_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053504_money_commands_spentinstripclub_int_bool_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053505_money_commands_commandspentjukebox_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053506_money_commands_buyhealthcare_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053507_money_commands_buyplayerhealthcare_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053508_money_commands_buyairstrike_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053509_money_commands_buyhelistrike_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053510_money_commands_buygangbackup_int_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053511_money_commands_spentonammodrop_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053512_money_commands_buybounty_int_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053513_money_commands_buyproperty_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053514_money_commands_buysmokes_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053515_money_commands_spentonhelipickup_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053516_money_commands_spentonboatpickup_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053517_money_commands_spentonbullshark_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053518_money_commands_spentoncashdrop_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053519_money_commands_spentonhiremugger_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053520_money_commands_spentonrobbedbymugger_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053521_money_commands_spentonhiremercenary_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053522_money_commands_spentonwantedlevel_int_int_ref_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053523_money_commands_spentonofftheradar_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053524_money_commands_spentonrevealplayersinradar_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053525_money_commands_spentoncarwash_int_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053526_money_commands_spentoncinema_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053527_money_commands_spentontelescope_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053528_money_commands_spentonholdups_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053529_money_commands_spentonpassivemode_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053530_money_commands_spentonbankinterest_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053531_money_commands_spentonprostitute_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053532_money_commands_spentonarrestbail_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053533_money_commands_spentonvehicleinsurancepremium_int_int_int_ref_bo(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_053534_money_commands_spentonplayercall_int_int_ref_bool_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_053535_money_commands_spentonbounty_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053536_money_commands_spentonrockstar_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053537_money_commands_spentonnocops_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053538_money_commands_spentoncargosourcing_int_bool_bool_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_053539_money_commands_spentonjobrequest_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053540_money_commands_spentonbendrequest_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053541_money_commands_command_spentonmoveyacht_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053542_money_commands_command_spentonmovesubmarine_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053543_money_commands_commandspentrenameorganization_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053544_money_commands_commandbuycontrabandmission_int_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053545_money_commands_commandpaybusinesssupplies_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053546_money_commands_commandspentvehicleexportmods_int_bool_bool_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_053547_money_commands_commandspentpaserviceheli_int_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053548_money_commands_commandspentpaservicevehicle_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053549_money_commands_commandspentpaservicesnack_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053550_money_commands_commandspentpaservicedancer_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053551_money_commands_commandspentpaserviceimpound_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053552_money_commands_commandspentpahelipickup_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053553_money_commands_spentonexecutiveofficeandwarehouseproperty_int_ch(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_053554_metricspent_officeandwarehousepurchandupdate_metricspent_officea(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_053559_money_commands_spentonexecutiveofficegarage_int_char_const_ptr_s(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_053560_metricspent_officegarage_metricspent_officegarage_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_053565_money_commands_commandspenttradewarehousepropertyimpexp_int_srcb(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_053566_metricspent_spentpurchasebusinessproperty_metricspent_spentpurch(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_053567_money_commands_commandspentorderwarehousevehicle_int_int_bool_bo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053568_money_commands_commandspentorderbodyguardvehicle_int_int_bool_bo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053569_money_commands_spentonclubhouseproperty_int_char_const_ptr_srccl(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_053572_money_commands_commandspentpurchasebusinessproperty_int_srcbusin(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_053573_money_commands_commandspentupgradebusinessproperty_int_srcbusine(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_053574_money_commands_commandspenttradebusinessproperty_int_srcbusiness(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_053575_money_commands_commandspentmcability_int_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053576_money_commands_commandspentimportexportrepair_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053577_money_commands_commandspentbuytruck_int_bool_bool_srcspentontruc(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_053578_money_commands_commandspentupgradetruck_int_bool_bool_srcspenton(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_053579_money_commands_commandspentbuybunker_int_bool_bool_srcspentonbun(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_053580_money_commands_commandspentupgradebunker_int_bool_bool_srcspento(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_053581_money_commands_commandspendgangopsrepaircost_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053582_money_commands_commandearnfromsellbunker_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053583_money_commands_commandearnfromrdrbonus_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053584_money_commands_commandearnfromwagepayment_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053585_money_commands_commandearnfromwagepaymentbonus_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053586_money_commands_commandchangeappearance_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053587_money_commands_commandspentballisticequipment_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053588_money_commands_commandspentplayerbase_char_const_ptr_int_bool_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_053589_money_commands_metricspent_christmas2017_metricspent_christmas20(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_053592_money_commands_commandspenttiltrotor_char_const_ptr_int_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_053595_money_commands_commandspentemployassassins_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053596_money_commands_commandspentheistcannon_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053597_money_commands_commandspentheistmission_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053598_money_commands_commandspentcasinoheistskipmission_int_int_bool_b(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053599_money_commands_commandearnsellbase_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053600_money_commands_commandearntargetrefund_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053601_money_commands_commandearnheistwages_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053602_money_commands_commandearnheistwagesbonus_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053603_money_commands_commandearndarchallenge_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053604_money_commands_commandearndoomsdayfinalebonus_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053607_money_commands_commandspendgangopsstartstrand_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053608_money_commands_commandspendgangopstripskip_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053609_money_commands_commandearngangopsprepparticipation_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053610_metricearnfromgangopsjobs_metricearnfromgangopsjobs_int_char_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_053611_money_commands_commandearngangopssetup_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_053612_money_commands_commandearngangopsfinale_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_053613_money_commands_commandearnnightclub_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053614_money_commands_commandearnbbeventbonus_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053615_money_commands_commandearnnightclubdancing_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053616_money_commands_commandearnmoneyfromgangopsrivaldelivery_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053617_money_commands_spentonhackertruck_int_char_const_ptr_srcspendhac(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_053618_money_commands_metricspent_megabusiness_metricspent_megabusiness(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_053621_money_commands_commandearnhackertruck_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053622_money_commands_metricearnfrommegabusiness_metricearnfrommegabusi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_053623_money_commands_spentonnightclubandwarehouse_int_char_const_ptr_s(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_053626_money_commands_commandearnnightclubandwarehouse_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_053627_money_commands_nightclubspend_char_const_ptr_char_const_ptr_stat(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053628_money_commands_commandspendmiscnightclub_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053629_money_commands_commandspendrdrhatchetbonus_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053630_money_commands_commandearnarenaskilllevelprogression_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053631_money_commands_commandearnarenacareerprogression_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053632_money_commands_commandspendmakeitrain_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053633_money_commands_commandspentarena_char_const_ptr_int_bool_bool_sr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_053634_metricspent_buyandupdategeneric_metricspent_buyandupdategeneric(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_053637_money_commands_commandspentcasino_char_const_ptr_int_bool_bool_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_053641_money_commands_commandearncasinotimetrialwin_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053642_money_commands_commandearncollectableitem_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053643_money_commands_commandearncollectablecompletedcollection_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053644_money_commands_commandearncollectableactionfigures_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053645_money_commands_commandearncasinocollectablecompletedcollection_i(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053646_money_commands_commandearnsellprizevehicle_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053647_money_commands_commandearncasinomissionreward_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053648_money_commands_commandearncasinostorymissionreward_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053649_money_commands_commandearncasinomissionparticipation_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053650_money_commands_commandearncasinoaward_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053651_money_commands_spendorupgradearcade_char_const_ptr_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_053654_money_commands_commandspendcasinoheist_int_bool_bool_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32); }
extern "C" { pub fn f_053655_metricspendgeneric_metricspendgeneric_char_const_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_053656_money_commands_commandspendarcademgmt_int_bool_bool_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053657_money_commands_commandspendplayarcade_int_bool_bool_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053658_money_commands_commandspendarcade_int_bool_bool_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053659_money_commands_commandearncasinoheist_int_int_int_int_int_int_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_053660_money_commands_commandearnupgradearcade_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053661_money_commands_commandearnarcade_int_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053662_money_commands_commandearncollectables_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053663_money_commands_commandearnchallenge_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053664_money_commands_commandearncasinoheistawards_int_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053665_money_commands_commandearnyatchmission_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053666_money_commands_commandearndispatchcall_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053667_money_commands_commandspendbeachparty_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053668_money_commands_commandspendsubmarine_int_bool_bool_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053669_money_commands_commandspendcasinoclub_int_bool_bool_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_053670_money_commands_spendorupgradebuysub_char_const_ptr_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_053673_money_commands_commandspendislandheist_int_bool_bool_srcspentisl(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_053674_money_commands_commandearnislandheist_int_int_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_053675_money_commands_commandearnbeachpartylostfound_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053676_money_commands_commandearnislandheistdjmission_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053677_money_commands_commandspendcarclubmembership_int_bool_bool_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053678_money_commands_commandspendcarclubbar_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053679_money_commands_commandspendautoshopmodify_int_bool_bool_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053680_money_commands_commandspendcarclubtakeover_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053681_money_commands_spendbuyorupgradeautoshop_char_const_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_053684_money_commands_commandearnautoshopbusiness_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053685_money_commands_commandearnautoshopincome_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053686_money_commands_commandearncarclubmembership_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053687_money_commands_commandearndailyvehicle_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053688_money_commands_commandearndailyvehiclebonus_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053689_money_commands_commandearntuneraward_int_char_const_ptr_char_con(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_053690_money_commands_commandearntunerrobbery_int_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053691_money_commands_commandearnupgradeautoshop_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053692_money_commands_commandspendinteractionmenuability_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053696_money_commands_spendbuyorupgradeagency_char_const_ptr_int_bool_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_053699_money_commands_commandspendagency_int_bool_bool_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053700_money_commands_commandspendhidden_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053701_money_commands_commandspendsourcebike_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053702_money_commands_commandspendcompsuv_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053703_money_commands_commandspendsuvfsttrvl_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053704_money_commands_commandspendsupply_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053705_money_commands_commandspendbikeshop_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053706_money_commands_commandspendvehiclerequested_int_bool_bool_int_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_053707_money_commands_commandspendgunrunning_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053708_money_commands_commandearnagencysafe_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053709_money_commands_commandearnawardcontract_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053710_money_commands_commandearnagencycontract_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053711_money_commands_commandearnawardphone_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053712_money_commands_commandearnagencyphone_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_053713_money_commands_commandearnawardfixermission_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053714_money_commands_commandearnfixerprep_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053715_money_commands_commandearnfixerfinale_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053716_money_commands_commandearnfixeragencyshorttrip_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053717_money_commands_commandearnawardshorttrip_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053718_money_commands_commandearnupgradeagency_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053719_money_commands_commandearnfixerrivaldelivery_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053720_money_commands_commandspendapartmentutilities_int_bool_bool_srca(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_053721_money_commands_commandspendbusinessutilities_int_bool_bool_srcbu(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_053722_money_commands_commandearnsightseeingreward_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053723_money_commands_commandearnbikershop_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_053724_money_commands_commandearnbiker_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_053725_money_commands_commandearnyohansourcegoods_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_053738_money_commands_commandgetcantransfercash_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054059_metrictransaction_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_054490_netshopping_commands_verify_item_isvalid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054492_netshopping_commands_commandnetgameservercheckitemisvalid_char_c(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_054498_netshopping_commands_commandnetgameserverinitserver(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_054499_netshopping_commands_commandnetgameserverretrievesessioninitstat(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_054501_netshopping_commands_commandnetgameserverstartsession_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054502_netshopping_commands_commandnetgameserverretrievesessionstartsta(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_054505_netshopping_commands_commandnetgameserverissessionvalid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054506_netshopping_commands_commandnetgameservergetstateandrefreshstatu(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_054508_netshopping_commands_commandnetgameserversessionapplyreceiveddat(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054509_netshopping_commands_commandnetgameserversessionrestart_bool_boo(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_054511_netshopping_commands_commandnetgameservergetprice_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_054512_netshopping_commands_commandnetgameserverbasketstart_int_ref_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_054515_netshopping_commands_commandnetgameserverbasketadditem_srcbasket(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_054517_netshopping_commands_commandnetgameserverbasketapplyserverdata_i(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_054518_netshopping_commands_commandnetgameservercheckoutstart_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054519_netshopping_commands_commandnetgameserverbeginservice_int_ref_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_054520_netshopping_commands_commandnetgameserverendservice_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054521_netshopping_commands_commandnetgameserverdeleteslot_int_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_054523_netshopping_commands_commandnetgameserverdeleteslotsettelemetryn(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_054524_netshopping_commands_commandnetgameservertransferbanktowallet_in(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_054525_netshopping_commands_commandnetgameservertransferwallettobank_in(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_054527_netshopping_commands_commandnetgameservertransfersettelemetrynon(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_054529_netshopping_commands_commandnetgameserversettelemetrynonceseed_i(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054628_network_commands_checkmultiplayeraccess_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_054637_network_commands_commandnetworkdocrewmatchmaking_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_054650_network_commands_commandreserveslots_sessiontype_int_ref_int_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_054667_network_commands_commandnetworkaddfollowers_int_ref_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_054673_network_commands_commandnetworkfindmatchedgamers_int_float_float(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_054683_network_commands_commandnetworkgetgamerstatusresult_int_ref_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_054735_network_commands_commandnetworkdotransitionquickmatchwithgroup_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_054749_network_commands_commandnetworkinvitegamerstotransition_int_ref(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_054756_network_commands_commandnetworkdotransitiontogame_bool_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_054757_network_commands_commandnetworkdotransitiontonewgame_bool_int_bo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_054758_network_commands_commandnetworkdotransitiontofreemode_int_ref_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_054759_network_commands_commandnetworkdotransitiontonewfreemode_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_054761_network_commands_commandnetworkgettransitionmembers_int_ref_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_054786_network_commands_commandnetworkaretransitiondetailsvalid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054802_network_commands_commandnetworkgetrsinvitesessionid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054815_network_commands_commandnetworkinvitegamerstosession_int_ref_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_054822_network_commands_commandnetworksetinvitefailedmessageforinviteme(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_054823_network_commands_commandnetworksetplayertooncallforjoinedmenu_in(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_054826_network_commands_commandfilloutpmplayerlist_int_ref_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_054827_rage_atarray_rage_rlgamerhandle_0_unsigned_short_grow_unsigned_i(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_054828_network_commands_commandfilloutpmplayerlistwithnames_int_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_054829_network_commands_commandrefreshplayerliststats_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054830_network_commands_commandnetworkgetplatformpartymembers_int_ref_i(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_054840_network_commands_commandnetworksetthisscriptisnetworkscript_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_054845_network_commands_commandnetworkgetscriptstatus(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_054846_network_commands_registerhostbroadcastvariables_int_ref_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_054847_network_commands_registerplayerbroadcastvariables_int_ref_int_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_054856_network_commands_commandnetworkgetplayerindex_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054859_network_commands_commandnetworkgettotalnumplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_054861_network_commands_commandnetworkisplayeractive_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054865_network_commands_commandnetworkgethostofscript_char_const_ptr_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_054870_network_commands_commandnetworkgetnumscriptparticipants_char_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_054873_network_commands_commandnetworkisplayeraparticipantonscript_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_054881_network_commands_commandnetworkresurrectlocalplayer_rage_scrvect(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_054887_cped_isfatallyinjured_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_054888_network_commands_commandnetworkgetentitykillerofplayer_int_int_r(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_054889_network_commands_commandnetworkgetkillerofplayer_int_int_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_054890_network_commands_getdestroyerofobject_cphysical_ptr_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_054891_network_commands_commandnetworkgetdestroyerofnetworkid_int_int_r(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_054892_network_commands_commandnetworkgetdestroyerofentity_int_int_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_054893_network_commands_commandnetworkgetassistedkillofentity_int_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_054894_network_commands_commandnetworkgetassisteddamageofentity_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_054900_network_commands_commandnetworkregisterentityasnetworked_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_054901_network_commands_commandnetworkunregisternetworkedentity_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_054903_network_commands_commandnetworkrequestcontrolofnetworkid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054906_network_commands_commandnetworkrequestcontrolofentity_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054907_network_commands_commandnetworkrequestcontrolofdoor_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054909_network_commands_commandnetworkhascontrolofpickup_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054916_network_commands_commandnetworkhashfromplayerhandle_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054922_network_commands_commandnetworkgetgamertagfromhandle_int_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_054925_network_commands_commandnetworkarehandlesthesame_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_054926_network_commands_commandnetworkgetplayerindexfromgamerhandle_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_054931_network_commands_commandnetworkplayergetuserid_int_int_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_054935_network_commands_commandnetworkplayerischeater_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054939_network_commands_commandnetworkisfriendonline_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_054940_network_commands_commandnetworkisfriendhandleonline_int_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_054941_network_commands_commandnetworkisfriendindexonline_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_054942_network_commands_commandnetworkisfriendinsametitle_char_const_pt(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_054944_network_commands_commandnetworkisinactiveprofile_int_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_054958_network_commands_commandnetworkisgamertalking_int_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_054979_network_commands_commandnetworksetonsameteamfortext_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_055006_network_commands_commandsetnetworkidcanmigrate_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055007_network_commands_commandsetnetworkidexistsonallmachines_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055008_network_commands_commandsetnetworkidalwaysexistsforplayer_int_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_055009_network_commands_commandsetnetworkidcanbereassigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055010_network_commands_commandsetnetworkentitycanblend_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055013_network_commands_commandsetnetworkidvisibleincutscene_int_bool_b(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_055014_network_commands_commandsetnetworkidvisibleincutscenehack_int_bo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_055015_network_commands_commandsetnetworkidvisibleincutsceneremainhack(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055018_network_commands_commandsetnetworkidpasscontrolintutorial_int_bo(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055019_network_commands_commandisnetworkidownedbyparticipant_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_055020_network_commands_commandsetlocalplayervisibleincutscene_bool_boo(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055023_network_commands_commandsetplayerinvisiblelocally_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055024_network_commands_commandsetplayervisiblelocally_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055026_network_commands_commandsetentityvisibleincutscene_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_055029_network_commands_commandisdamagetrackeractiveonnetworkid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_055030_network_commands_commandactivatedamagetrackeronnetworkid_int_boo(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055032_network_commands_commandactivatedamagetrackeronplayer_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055035_network_commands_commandreservenetworkmissionobjects_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_055036_network_commands_commandreservenetworkmissionpeds_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_055037_network_commands_commandreservenetworkmissionvehicles_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_055038_network_commands_commandreservelocalnetworkmissionobjects_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_055039_network_commands_commandreservelocalnetworkmissionpeds_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_055040_network_commands_commandreservelocalnetworkmissionvehicles_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_055047_network_commands_commandgetnumreservedmissionobjects_bool_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_055048_network_commands_commandgetnumreservedmissionpeds_bool_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_055049_network_commands_commandgetnumreservedmissionvehicles_bool_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_055053_network_commands_commandgetreservedmissionentitiesinarea_rage_sc(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_055058_network_commands_commandnetworkallowcloningwhileintutorial_int_b(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055060_network_commands_verifytime_unsigned_int_network_commands_snetwo(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_055062_network_commands_commandgetnetworktimeaccurate(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_055073_network_commands_commandnetworksetinspectatormode_bool_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055074_network_commands_commandnetworksetinspectatormodeextended_bool_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_055084_network_commands_commandnetworksetmaxpositiondeltamultiplier_int(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_055085_network_commands_commandnetworkenablehighspeededgefalldetection(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055088_network_commands_commandsetlocalplayerasghost_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055096_network_commands_commandnetworkstartrespawnsearchforplayer_int_r(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_055097_network_commands_commandnetworkstartrespawnsearchinangledareafor(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_055111_network_commands_commandnetworkconcealplayer_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_055112_network_commands_commandnetworkconcealentity_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055116_network_commands_commandremoveallstickybombsfromentity_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055117_network_commands_commandnetworkleavepedbehindbeforewarp_int_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_055118_network_commands_commandnetworkleavepedbehindbeforecutscene_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055119_network_commands_commandnetworkkeepentitycollisiondisabledaftera(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055120_network_commands_commandnetworkisanyplayernear_int_ref_int_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_055123_network_commands_util_rlclanmembershipdatatoscrclandesc_rage_rlc(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_055124_network_commands_commandnetworkclanplayergetdesc_scrclandesc_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_055125_network_commands_commandnetworkclangetplayerclanemblemtxdname_in(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_055126_network_commands_commandnetworkclanrequestemblem_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_055127_network_commands_commandnetworkclanisemblemready_int_char_ptr_12(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_055128_network_commands_commandnetworkclanreleaseemblem_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_055132_network_commands_commandnetworkclangetmembershipdesc_scrclandesc(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_055133_network_commands_commandnetworkclandownloadmembershipdesc_int_re(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_055134_network_commands_commandnetworkclandownloadmembershipdescpending(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_055136_network_commands_commandnetworkclanremotemembersipsalreadydownlo(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_055137_network_commands_commandnetworkclangetmembershipvalid_int_ref_in(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_055138_network_commands_commandnetworkclangetmembership_int_ref_scrclan(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_055139_network_commands_commandnetworkclangetmembershipcount_int_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_055141_network_commands_commandnetworkcreatesynchronisedscene_rage_scrv(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: f32, a6: f32, a7: f32) -> i32; }
extern "C" { pub fn f_055146_network_commands_commandnetworkattachsynchronisedscenetoentity_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_055147_network_commands_commandnetworkstartsynchronisedscene_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_055154_network_commands_commandnetworkclearclocktimeoverride(ph: *mut Phantom); }
extern "C" { pub fn f_055156_network_commands_commandnetworkaddentityarea_rage_scrvector_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_055157_network_commands_commandnetworkaddentityangledarea_rage_scrvecto(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_055158_network_commands_commandnetworkaddcliententityarea_rage_scrvecto(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_055159_network_commands_commandnetworkaddcliententityangledarea_rage_sc(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_055160_network_commands_commandnetworkremoveentityarea_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_055172_network_commands_commandnetworkdoestunableexist_deprecated_char(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_055173_network_commands_commandnetworkaccesstunableint_deprecated_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_055174_network_commands_commandnetworkaccesstunablefloat_deprecated_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_055175_network_commands_commandnetworkaccesstunablebool_deprecated_char(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_055189_network_commands_commandnetworksetattributedamagetoplayer_int_in(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_055190_network_commands_commandsnetworktriggerdamageeventforzerodamage(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055191_network_commands_commandsnetworktriggerdamageeventforzeroweaponh(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055192_network_commands_commandnetworkexplodevehicle_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_055193_network_commands_commandnetworkexplodeheli_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_055195_network_commands_commandnetworkoverridecoordsandheading_int_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: f32); }
extern "C" { pub fn f_055196_network_commands_commandnetworkenableextravehicleorientationblen(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055202_network_commands_commandnetworkcachelocalplayerheadblenddata(ph: *mut Phantom); }
extern "C" { pub fn f_055204_network_commands_commandnetworkapplycachedplayerheadblenddata_in(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_055205_network_commands_commandfadeoutlocalplayer_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_055206_network_commands_commandnetworkfadeoutentity_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_055207_network_commands_commandnetworkfadeinentity_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_055218_network_commands_commandopencommercestore_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_055220_network_commands_commandrequeststoreimage_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_055230_network_commands_commandnetworkischeater(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_055234_network_commands_commandtriggertuningcrchackercheck_int_char_con(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_055235_network_commands_commandtriggerfilecrchackercheck_int_char_const(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_055258_network_commands_commandugcquerybycontentids_int_ref_int_bool_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_055266_network_commands_commandugcgetbycontentids_int_ref_int_char_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_055343_network_commands_commandhaverosprivatemsgpriv(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_055344_network_commands_commandhaverossocialclubpriv(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_055345_network_commands_commandhaverosbannedpriv(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_055346_network_commands_commandhaveroscreateticketpriv(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_055347_network_commands_commandhaverosmultiplayerpriv(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_055348_network_commands_commandhaverosleaderboardwritepriv(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_055349_network_commands_commandhasrosprivilegeplayedlastgen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_055350_network_commands_commandhasrosprivilegespecialeditioncontent(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_055351_network_commands_commandhasrosprivilege_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_055352_network_commands_commandhasrosprivilegeenddate_int_int_ref_int_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_055357_network_commands_commanddisableleaveremotepedbehind_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_055358_network_commands_commandnetworkallowremoteattachmentmodification(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_055361_network_commands_commandnetworkgetprimaryclandatastart_int_ref_i(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_055364_network_commands_commandnetworkgetprimaryclandatanew_int_ref_scr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_056463_object_commands_pointinangledareacheck_bool_rage_vector3_const_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32) -> i32; }
extern "C" { pub fn f_056465_object_commands_objectcreationfunction_int_float_float_float_boo(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_056468_object_commands_commanddeleteobject_int_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_056471_object_commands_commandgetsafepickupcoords_rage_scrvector_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_056472_object_commands_commandaddextendedpickupprobearea_rage_scrvector(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_056473_object_commands_commandclearextendedpickupprobeareas(ph: *mut Phantom); }
extern "C" { pub fn f_056474_object_commands_convertoldpickuptypetohash_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_056475_object_commands_commandcreatepickup_int_rage_scrvector_const_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_056478_object_commands_commandcreatepickuprotate_int_rage_scrvector_con(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_056480_object_commands_createambientpickup_int_rage_scrvector_const_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_056481_object_commands_commandcreateambientpickup_int_rage_scrvector_co(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_056483_object_commands_createportablepickup_int_rage_scrvector_const_re(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_056486_object_commands_commandattachportablepickuptoped_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_056488_object_commands_commandforceportablepickuplastaccessibleposition(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_056497_object_commands_commandsetpickuptrackdamageevents_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_056501_object_commands_commandhaspickupbeencollected_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_056502_object_commands_commandremovepickup_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_056503_object_commands_commandcreatemoneypickups_rage_scrvector_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_056504_object_commands_commandrotateobject_int_float_float_bool(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_056505_object_commands_commandslideobject_int_rage_scrvector_const_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_056506_object_commands_commandsetobjecttargettable_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_056508_object_commands_commandgetclosestobjectoftype_rage_scrvector_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_056510_object_commands_commandhasclosestobjectoftypebeenbroken_rage_scr(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_056511_object_commands_commandhasclosestobjectoftypebeencompletelydestr(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_056514_object_commands_commandobjectfragmentbreakchild_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_056515_object_commands_commandobjectfragmentdamagechild_int_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_056523_object_commands_commandgetcoordsandrotationofclosestobjectoftype(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_056525_object_commands_commandgetstateofclosestdooroftype_int_rage_scrv(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_056528_object_commands_commandadddoortosystem_int_int_rage_scrvector_co(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_056529_object_commands_commandremovedoorfromsystem_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_056530_object_commands_commanddoorsystemsetstate_int_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_056533_object_commands_commanddoorsystemsetautomaticrate_int_float_bool(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_056534_object_commands_commanddoorsystemsetautomaticdistance_int_float(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_056535_object_commands_commanddoorsystemsetopenratio_int_float_bool_boo(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_056537_object_commands_commanddoorsystemgetopenratio_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_056538_object_commands_commanddoorsystemsetunsprung_int_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_056539_object_commands_commanddoorsystemsetholdopen_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_056541_object_commands_commandisdoorclosed_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_056544_object_commands_commanddoorsystemsetopenforraces_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_056546_object_commands_commanddoorsystemfindexistingdoor_rage_scrvector(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_056550_object_commands_commanddoespickupobjectexist_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_056551_object_commands_commandgetpickupobject_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_056555_object_commands_commandsetnearestbuildinganimrate_rage_scrvector(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_056556_cclipeventtags_crandomizerateproperty_const_ptr_cclipeventtags_f(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_056557_object_commands_commandsetnearestbuildinganimphase_rage_scrvecto(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_056559_object_commands_commandsetobjectphysicsparams_int_float_float_ra(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: i64, a4: i64, a5: f32, a6: f32, a7: f32); }
extern "C" { pub fn f_056560_object_commands_getobjfragmentdamagedhealth_cobject_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_056565_object_commands_commandsetcompositeentitystate_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_056566_object_commands_commandgetcompentityanimationphase_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_056571_object_commands_commandsetpickupregenerationtime_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_056572_object_commands_commandforcepickupregenerate_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_056573_object_commands_commandsetplayerpermittedtocollectpickupsoftype(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_056596_object_commands_commandpreventcollectionofportablepickup_int_boo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_056606_object_commands_commandcleargarage_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_056607_object_commands_commandclearobjectsfromgarage_int_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_056615_object_commands_commandsetobjecttintindex_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_056616_object_commands_commandsettintindexclosestbuildingoftype_rage_sc(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_056618_object_commands_commandsetproplightcolor_int_bool_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_056619_object_commands_commandisproplightcoloroverriden_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_056628_object_commands_commandsetdrivearticulatedjointwithinflictor_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_056842_pad_commands_getpadstate_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_056849_pad_commands_commandgetcontrolnormal_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> f32; }
extern "C" { pub fn f_056853_pad_commands_commandisdisabledcontrolpressed_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_056854_pad_commands_commandisdisabledcontrolreleased_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_056855_pad_commands_commandisdisabledcontroljustpressed_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_056856_pad_commands_commandisdisabledcontroljustreleased_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_056857_pad_commands_commandgetdisabledcontrolnormal_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> f32; }
extern "C" { pub fn f_056874_pad_commands_commanddisablecontrolaction_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_056875_pad_commands_commandenablecontrolaction_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_056970_path_commands_setroadsinarea_rage_scrvector_const_ref_rage_scrve(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_056971_path_commands_setroadsinangledarea_rage_scrvector_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_056972_path_commands_commandsetpedpathsinarea_rage_scrvector_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_056973_path_commands_commanddisablenavmeshinarea_rage_scrvector_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_056975_path_commands_basegetclosestnode_rage_vector3_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i64, a8: f32, a9: f32) -> i32; }
extern "C" { pub fn f_056978_path_commands_commandgetvehiclenodeproperties_rage_scrvector_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_056986_path_commands_commandgetvehiclenodeposition_int_rage_vector3_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_056987_path_commands_commandgetvehiclenodeisgpsallowed_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_056988_path_commands_commandgetvehiclenodeisswitchedoff_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_056989_path_commands_commandgetclosestroad_rage_scrvector_const_ref_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i32) -> i32; }
extern "C" { pub fn f_056995_path_commands_commandswitchroadsbacktooriginal_rage_scrvector_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_056996_path_commands_commandswitchroadsbacktooriginalinangledarea_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_056997_path_commands_commandswitchpedroadsbacktooriginal_rage_scrvector(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_057005_path_commands_commandisnavmeshloadedinarea_rage_scrvector_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_057006_path_commands_commandgetnumnavmeshesexistinginarea_rage_scrvecto(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_057011_path_commands_commandgeneratedirections_outpos_rage_scrvector_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_057023_path_commands_getroadboundaryusingheading_rage_scrvector_const_r(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_057111_ped_commands_commandclonepedalt_int_bool_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_057113_ped_commands_commandclonepedtotargetalt_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_057173_ped_commands_createrandomped_rage_vector3_const_ref_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_057277_ped_commands_getaniminitialoffsetpositionandrotation_rage_crclip(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i32); }
extern "C" { pub fn f_057317_ped_commands_setrelationshipgroupblipforped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_057342_ped_commands_commandgetvehiclepedisentering_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_057356_ped_commands_setpedcanbeknockedoffvehicle_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_057359_ped_commands_switchtoragdollcode_cped_ptr_int_bool_ctask_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_057420_ped_commands_getassethash_cpedmodelinfo_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_059289_bool_creplaymgrinternal_handleentityforfx_cpacketsetobjecttint_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063735_gtathread_isanetworkthread(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063746_gtathread_sethasloadedallpathnodes_rage_scrthreadid(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_063747_gtathread_clearhasloadedallpathnodes(ph: *mut Phantom); }
extern "C" { pub fn f_063772_cthescripts_getcurrentgtascripthandler(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063794_cthescripts_registerentity_cphysical_ptr_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_063796_cthescripts_unregisterentity_cphysical_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063797_cthescripts_getcurrentgtascripthandlernetwork(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063798_cthescripts_prioritizerequestsfrommissionscript_int_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063799_cthescripts_getcurrentscriptname(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063800_cthescripts_getcurrentscriptnameandprogramcounter(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063808_chiddenobjects_addtohiddenobjectmap_int_rage_spdsphere_ref_int_c(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_063815_cthescripts_findlocalplayerped_int_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063816_cthescripts_findnetworkplayerped_int_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063817_cthescripts_findnetworkplayer_int_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063818_cthescripts_exportgamerhandle_rage_rlgamerhandle_const_ptr_int_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_063819_cthescripts_importgamerhandle_rage_rlgamerhandle_ptr_int_ref_int(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063820_cthescripts_hasentitybeenregisteredwithcurrentthread_cphysical_c(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063821_cgamescriptid_initgamescriptid_rage_scrthread_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_063822_cthescripts_getidforentityregisteredwithcurrentthread_cphysical(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063823_cscripteulers_matrixfromeulers_rage_matrix34_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063824_cscripteulers_matrixtoeulers_rage_matrix34_const_ref_rage_eulera(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063825_cscripteulers_quaternionfromeulers_rage_quaternion_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063826_cscripteulers_quaterniontoeulers_rage_quaternion_ref_rage_eulera(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063854_cgamescripthandler_registerscriptresourceandgetref_rage_scriptre(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063857_cscriptareas_newhighlightimportantarea_float_float_float_float_f(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32); }
extern "C" { pub fn f_063858_cscriptareas_highlightimportantangledarea_unsigned_int_float_flo(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32); }
extern "C" { pub fn f_063859_cscriptdebug_drawdebugangledsquare_float_float_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32); }
extern "C" { pub fn f_063863_cscriptdebug_drawdebugangledcube_float_float_float_float_float_f(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: f32, a10: i64); }
extern "C" { pub fn f_063896_cscriptpeds_ispedstopped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063899_cscriptpeds_setpedcoordinates_cped_ptr_float_float_float_int_boo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_063900_cscriptentities_clearspaceformissionentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063907_cracetimes_init_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_063908_cracetimes_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_063909_cracetimes_checkpointhit_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_063910_cracetimes_getplayertime_int_int_ref_int_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_063980_cscriptdebug_drawdebugcube_float_float_float_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_063982_cscriptdebug_drawdebugsquare_float_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_063985_caboveplayersheaddisplay_reset_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064083_cscriptmetadatamanager_getmpapparelmale_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064084_cscriptmetadatamanager_getmpapparelfemale_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064143_cgamescripthandlermgr_getremotescriptinfo_cgamescriptid_const_re(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_064164_cgamescripthandler_cgamescripthandler_rage_scrthread_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_064166_cgamescripthandler_getscriptentity_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_064173_cgamescripthandler_registerscriptresourceandgetid_rage_scriptres(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_064183_cgamescripthandlermgr_logcurrentreservations_rage_netlogginginte(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064187_cgamescripthandlernetwork_setmaxnumparticipants_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064188_cgamescripthandlernetcomponent_broadcastremotescriptinfo_rage_ne(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_064190_cgamescripthandlermgr_isplayeraparticipant_cgamescriptid_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_064198_cgamescripthandlernetcomponent_isparticipantphysical_int_cnetgam(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_064215_cgamescriptid_generatescriptidhash(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064228_cgamescripthandlermgr_trytosetscriptasanetworkscript_gtathread_r(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_064230_cgamescripthandlermgr_getnumreservedscriptentities_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_064231_cgamescripthandlermgr_getnumcreatedscriptentities_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_064232_cgamescripthandlermgr_getnumrequiredscriptentities_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_064233_cgamescripthandlermgr_getremotescriptentityreservations_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_064234_cgamescripthandlermgr_isscriptactive_cgamescriptid_const_ref_boo(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064235_cgamescripthandlermgr_isnetworkhost_cgamescriptid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064251_cscriptresource_pedgroup_cscriptresource_pedgroup_cgamescripthan(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_064252_cscriptresource_pedgroup_cscriptresource_pedgroup_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_064299_cscriptresource_relgroup_cscriptresource_relgroup_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_064323_cscriptresource_setrelgroupdontaffectwantedlevel_cscriptresource(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_064330_cscriptresource_ghostsettings_cscriptresource_ghostsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064332_cscriptresource_pickupgenerationmultiplier_cscriptresource_picku(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064425_cgamescriptresource_dtorcgamescriptresource_102634(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064588_ccustomshadereffecttint_selecttintpalette_unsigned_char_rage_fwe(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064589_ccustomshadereffecttint_getmaxtintpalette(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064736_csettingsmanager_csettingsmanager(ph: *mut Phantom); }
extern "C" { pub fn f_064737_csettingsmanager_getsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064813_cpopulationstreaming_isfallbackpedavailable_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064913_ckeyboard_getkeyjustdown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_065273_ckeyboard_getkeydown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_065293_ccompaniondata_getinstance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_065326_ccontrol_shutdownpcscriptcontrolmappings(ph: *mut Phantom); }
extern "C" { pub fn f_065340_ccontrol_enablealternatescriptedcontrolslayout(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065358_ccontrol_getinputsource_rage_inputtype_int_unsigned_char_bool_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_065366_ccontrol_disableallinputs_rage_iovalue_disableoptions_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065367_ccontrol_stopplayerpadshaking_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_065368_ccontrol_setlighteffect_rage_iolighteffect_const_ptr_ccontrol_li(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_065369_ccontrol_clearlighteffect_rage_iolighteffect_const_ptr_ccontrol(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_065374_ccontrol_setinputexclusive_int_rage_iovalue_disableoptions_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_065375_ccontrol_doenableinput_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_065376_ccontrol_getinputname_rage_inputtype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065379_ccontrol_dodisableinput_unsigned_int_rage_iovalue_disableoptions(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_065380_ccontrol_inputhowlongago_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065381_ccontrol_enableallinputs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065383_ccontrol_disableinput_rage_iomappersource_unsigned_int_rage_iova(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_065385_ccontrol_getvalue_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_065386_ccontrol_setinputvaluenextframe_rage_inputtype_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_065387_ccontrol_shakecontroller_int_int_bool_rage_iorumbleeffect_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_065390_ccontrol_getactivelayout_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065393_ccontrol_getinputgroupname_rage_inputgroup(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065401_ccontrol_isusingcursor_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065402_ccontrol_scriptcheckforcontrolschange(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065430_ccontrol_loadpcscriptcontrolmappings_rage_atnonfinalhashstring_c(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065446_ccontrolmgr_getmainplayercontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065458_ccontrolmgr_updatevirtualkeyboard(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065461_ccontrolmgr_getmainfrontendcontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065467_ccontrolmgr_cancelvirtualkeyboard(ph: *mut Phantom); }
extern "C" { pub fn f_065471_ccontrolmgr_getvirtualkeyboardstate(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065591_enduserbenchmarks_resetbenchmarks(ph: *mut Phantom); }
extern "C" { pub fn f_065592_enduserbenchmarks_startbenchmark(ph: *mut Phantom); }
extern "C" { pub fn f_065593_enduserbenchmarks_stopbenchmark(ph: *mut Phantom); }
extern "C" { pub fn f_065594_enduserbenchmarks_outputbenchmarkresults(ph: *mut Phantom); }
extern "C" { pub fn f_065596_enduserbenchmarks_getwasbenchmarkoncommandline(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065601_enduserbenchmarks_getiterationcount(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065602_enduserbenchmarks_getsinglepassinfo(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065623_cfilemgr_hasasyncinstallfinished(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065624_cfilemgr_cleanupasyncinstall(ph: *mut Phantom); }
extern "C" { pub fn f_065627_ckeyboard_getkeyjustup_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_065628_ckeyboard_getkeyup_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_065632_ckeyboard_setcurrentmode_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_066004_ccombattaskmanager_countpedsincombatwithtarget_cped_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_066056_ctacticalanalysisnavmeshpoints_addpointfromscript_rage_vec3v_con(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066058_ctacticalanalysisnavmeshpoints_clearpointsfromscript(ph: *mut Phantom); }
extern "C" { pub fn f_066846_ctaskincover_iscoverpointtoohigh_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067376_crandomeventmanager_seteventtypeenabled_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_068454_ctaskmotionbase_setonfootclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_068455_ctaskmotionbase_setinvehiclecontexthash_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_068456_ctaskmotionbase_clearoverrideweaponclipset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068457_ctaskmotionbase_resetonfootclipset_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_068458_ctaskmotionbase_resetinvehiclecontexthash_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068459_ctaskmotionbase_resetstrafeclipset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068461_ctaskmotionbase_setalternateclip_ctaskmotionbase_alternateclipty(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_068462_ctaskmotionbase_stopalternateclip_ctaskmotionbase_alternateclipt(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_069617_ctaskparachute_createparachutebagobject_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_070401_ctasknmbehaviour_canuseragdoll_cped_ptr_eragdolltriggertypes_cen(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_070836_ctasknmthroughwindscreen_ctasknmthroughwindscreen_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_074161_ctask_ctask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075349_cmovenetworkhelper_getfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_077089_cstatsmgr_incrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077090_cstatsmgr_decrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077153_statid_char_getstatid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077193_cstatsdatamgr_iskeyvalid_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077195_cstatsdatamgr_setstatsynched_statid_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077329_statid_set_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077331_statsinterface_getintstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077333_statsinterface_getkeyname_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077334_statsinterface_iskeyvalid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077336_statsinterface_getstatdesc_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077338_statsinterface_getcurrentmultiplayercharaterslot(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077339_statsinterface_cloudfileloadpending_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077340_statsinterface_cloudfileloadfailed_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077342_statsinterface_setstatdata_statid_const_ref_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077344_statsinterface_setstatdata_statid_const_ref_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077348_statsinterface_setstatdata_statid_const_ref_unsigned_long_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_077349_statsinterface_setstatdata_statid_const_ref_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_077359_statsinterface_getuint64stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077360_statsinterface_getint64stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077370_statsinterface_isbadsport_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_077373_statsinterface_getstatsmodelhashid_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077381_statsinterface_applypedscardata_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077413_cstatssavesmgr_flushprofilestats(ph: *mut Phantom); }
extern "C" { pub fn f_077430_moneyinterface_getvcwalletbalance_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077431_moneyinterface_getvcbankbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077432_moneyinterface_getevcbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077433_moneyinterface_getpvcbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077434_moneyinterface_getvcbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077435_moneyinterface_getvcdailytransferbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077436_moneyinterface_getpvcdailytransfersbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077438_moneyinterface_getpxrvalue(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_077439_moneyinterface_decrementstatpvcbalance_long_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077440_moneyinterface_decrementstatevcbalance_long_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077441_moneyinterface_incrementstatevcbalance_long_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077442_moneyinterface_incrementstatbankbalance_long_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077443_moneyinterface_decrementstatbankbalance_long_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077444_moneyinterface_incrementstatwalletbalance_long_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077445_moneyinterface_decrementstatwalletbalance_long_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_077446_moneyinterface_getplayerishighearner(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077447_moneyinterface_getamountavailablefortransfer(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077448_moneyinterface_canbuycash_long_float(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_077457_moneyinterface_economyfixedcrazynumbers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077459_moneyinterface_requestbankcashwithdrawal_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077460_moneyinterface_creditbankcashstats_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077553_ctext_issclanguagesupported_rage_rlsclanguage(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077585_ctextformat_gettokenidfrompadbutton_rage_iomappersource_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_077586_ctextformat_getdeviceiconsfrominputgroup_rage_atfixedarray_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077588_ctextformat_getinputbuttons_char_const_ptr_char_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_077590_ctextformat_getinputgroupbuttons_char_const_ptr_char_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_077786_cjunction_settomalfunction_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077791_cpathfind_checkpathnodeintegrityforzone_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077792_cpathfind_testalllinks_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077809_cjunction_deploy_rage_cnodeaddress_bool_cpathfind_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077810_cpathfind_arenodesloadedforarea_float_float_float_float_const(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_077811_cpathfind_findnodeclosesttocoors_rage_vector3_const_ref_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32); }
extern "C" { pub fn f_077812_cpathfind_findnodeclosesttocoors_rage_vector3_const_ref_float_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: f32, a11: i32, a12: i64, a13: i32, a14: i32); }
extern "C" { pub fn f_077813_cpathfind_findnodeslinkindexbetween2nodes_rage_cnodeaddress_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_077825_cjunctions_getjunctionatposition_rage_vector3_const_ref_float_bo(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077827_cjunctions_getjunctionbyindex_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077849_cpathfind_generatedirections_int_ref_unsigned_int_ref_float_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_077853_cpathfind_addnodesrequiredregionthisframe_int_rage_vector3_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_077867_cpathfind_calculatetraveldistancebetweennodes_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32) -> f32; }
extern "C" { pub fn f_077874_cpathfind_findroadboundaries_int_int_float_bool_bool_float_ref_f(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: i32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_077882_cpathfind_findbestlinkfromheading_rage_cnodeaddress_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_077883_cpathfind_getpositionbysideofroad_rage_vector3_const_ref_int_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_077884_cpathfind_findnthnodeclosesttocoors_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: f32, a11: i32, a12: i32, a13: i32); }
extern "C" { pub fn f_077890_cpathfind_findnodeorientationforcarplacement_rage_cnodeaddress_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64) -> f32; }
extern "C" { pub fn f_077894_cpathfind_findstreetnameatposition_rage_vector3_const_ref_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077895_cpathfind_switchroadsoffinarea_unsigned_int_float_float_float_fl(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_077897_cpathfind_switchroadsoffinangledarea_unsigned_int_rage_vector3_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_077898_cpathfind_getnextavailablegpsblockingregionforscript(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077899_cpathfind_setgpsblockingregionforscript_unsigned_int_rage_vector(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_077900_cpathfind_findnodecoorsforscript_rage_cnodeaddress_bool_ptr_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077907_cpathfind_getrandomcarnode_rage_vector3_ref_float_bool_int_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_078274_ctaskvehiclethreepointturn_ispointonroad_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078909_cvehicle_istaximodelid_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079070_cvehicle_getseatanimationinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079918_cvehicle_setrespotcounter_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_080540_cvehiclemetadatamgr_getinvehicleoverrideinfo_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082123_cweapondamage_generatepeddamageevent_centity_ptr_cped_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64, a5: i64, a6: i64, a7: i32, a8: f32, a9: i64, a10: i32, a11: f32, a12: i32) -> f32; }
extern "C" { pub fn f_082144_cfiringpattern_setfiringpattern_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082723_cprojectilemanager_removeallstickybombsattachedtoentity_centity(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_082825_networkexitflow_startexitsaveflow(ph: *mut Phantom); }
extern "C" { pub fn f_082828_networkexitflow_startshutdowntasks(ph: *mut Phantom); }
extern "C" { pub fn f_082834_cnetwork_bail_sbailparameters_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082835_networkinterface_islocalplayersignedin(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082855_cnetwork_lansessionsonly(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082865_cfakeplayernames_removeallfakeplayernames(ph: *mut Phantom); }
extern "C" { pub fn f_082870_cnetwork_gettimeouttime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082873_cnetwork_setscriptreadyforevents_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082885_networkinterface_isanysessionactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082897_cnetwork_flushalllogfiles_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082904_clockoverridedata_stopoverriding(ph: *mut Phantom); }
extern "C" { pub fn f_082905_clockoverridedata_override_int_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_082906_cnetwork_getglobalclock_int_ref_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082907_cnetwork_getglobalweather_int_ref_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082908_networkinterface_ishost(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082909_cnetworktelemetry_appendmetric_rage_rlmetric_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082912_networkinterface_isinsession(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082916_networkremotecheaterdetector_remotecheaterdetected_cnetgameplaye(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_082923_cnetwork_applyclockandweather_bool_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_082924_cnetwork_isclocktimeoverridden(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082927_cnetwork_starttransitiontracker_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_082929_cnetwork_addtransitiontrackerstage_int_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_082933_cnetwork_finishtransitiontracker_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_082934_cnetwork_ismultiplayerdisabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082935_cnetwork_ismultiplayerlocked(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082938_networkinterface_islocalplayeronline_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082939_networkinterface_hasvalidroscredentials_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082943_cnetwork_canaccessnetworkfeatures_enetworkaccessarea_enetworkacc(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_082944_cnetwork_checknetworkaccessandalertiffail_enetworkaccessarea(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082946_cnetwork_setscriptautomuted_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082947_cnetwork_ischeater_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082948_cplayercardmanager_getdatamanager_cplayercardmanager_cardtypes_c(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_082954_cnetwork_stopnetworktimepausable(ph: *mut Phantom); }
extern "C" { pub fn f_082955_cnetwork_restartnetworktimepausable(ph: *mut Phantom); }
extern "C" { pub fn f_082956_cnetwork_dumpnetworktimeinfo(ph: *mut Phantom); }
extern "C" { pub fn f_082999_networkinterface_registerobject_rage_netgameobjectwrapper_cdynam(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083000_cnetwork_getclocktimestoredforoverride_int_ref_int_ref_int_ref_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_083003_clockoverridedata_overriderate_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083004_cnetwork_setnetworkgamemode_unsigned_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083007_clivemanager_issignedin_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083008_networkinterface_iscloudavailable_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083010_networkinterface_isrefreshingroscredentials_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083011_networkinterface_hasvalidrockstarid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083013_networkinterface_getactivegamerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083014_networkinterface_getlocalgamerhandle(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083015_clivemanager_checkonlineprivileges_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083017_cusercontentmanager_isquerypending(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083019_networkinterface_issessionestablished(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083021_networkinterface_issessionbusy(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083022_networkinterface_cansessionleave(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083025_networkinterface_getphysicalplayerfromindex_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083028_networkinterface_isinspectatormode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083038_networkinterface_isinfreemode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083041_networkinterface_canregisterobject_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083047_networkutils_isvisibletoanyremoteplayer_rage_vector3_const_ref_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i32; }
extern "C" { pub fn f_083048_networkutils_isvisibletoplayer_cnetgameplayer_const_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_083056_networkinterface_getplayerfromgamerhandle_rage_rlgamerhandle_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083058_networkinterface_gethostplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083061_networkinterface_getnetworkobject_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083063_networkinterface_unregisterobject_rage_netgameobjectwrapper_cdyn(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083067_networkinterface_canspectateobjectid_rage_netobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083068_networkinterface_areplayersindifferenttutorialsessions_cnetgamep(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083069_networkinterface_setspectatorobjectid_unsigned_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083071_networkinterface_setinmpcutscene_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_083074_networkinterface_getmatchconfig(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083075_networkinterface_doquickmatch_int_int_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083076_networkinterface_dosocialmatchmaking_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_083077_networkinterface_doactivityquickmatch_int_unsigned_int_int_int_u(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_083078_networkinterface_hostsession_int_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083079_networkinterface_leavesession_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083080_networkinterface_leavesession_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083081_networkinterface_setscriptvalidatejoin(ph: *mut Phantom); }
extern "C" { pub fn f_083082_networkinterface_validatejoin_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083086_cnetworkroadnodeworldstatedata_cnetworkroadnodeworldstatedata_cg(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_083087_cnetworkscriptworldstatemgr_revertworldstate_cnetworkworldstated(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083088_cnetworkroadnodeworldstatedata_cnetworkroadnodeworldstatedata_cg(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: f32, a6: i32) -> i64; }
extern "C" { pub fn f_083089_cnetworkscriptworldstatemgr_changeworldstate_cnetworkworldstated(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083090_networkinterface_registerscriptdoor_cdoorsystemdata_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083091_networkinterface_unregisterscriptdoor_cdoorsystemdata_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083125_networkinterface_getlastposreceivedovernetwork_cdynamicentity_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083126_networkinterface_getlastvelreceivedovernetwork_cdynamicentity_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083127_networkinterface_getpredictedvelocity_cdynamicentity_const_ptr_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_083135_networkinterface_overridenetworkblenderfortime_cphysical_const_p(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083145_networkinterface_isrelevanttowaypoint_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083146_networkinterface_ignoreremotewaypoints(ph: *mut Phantom); }
extern "C" { pub fn f_083147_networkinterface_cansetwaypoint_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083157_networkinterface_delaymigrationfortime_cphysical_const_ptr_unsig(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083173_networkinterface_canscriptmodifyremoteattachment_cphysical_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083179_networkinterface_cacheplayerheadblenddata_unsigned_char_cpedhead(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_083182_networkinterface_isconnectedviarelay_unsigned_char(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083183_networkinterface_getaveragesynclatency_unsigned_char(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_083184_networkinterface_getaveragepacketloss_unsigned_char(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_083185_networkinterface_getnumunackedreliables_unsigned_char(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083186_networkinterface_getunreliableresendcount_unsigned_char(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083187_networkinterface_gethighestreliableresendcount_unsigned_char(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083266_csyncedped_getped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083296_cnetworktextchat_setspectatortononspectatorchat_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083298_networkclan_hasprimaryclan(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083308_networkclan_releaseemblemreference_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083314_networkclan_requestemblemforclan_long(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083315_networkclan_getclanemblemnameforclan_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083324_networkclan_getclanemblemnameforclan_long_char_const_ptr_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_083332_clivemanager_checkusercontentprivileges_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083348_networkcrewmetadata_getcrewinfovaluestring_char_const_ptr_char_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_083349_networkcrewmetadata_getcrewinfocrewranktitle_int_char_ptr_int_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_083351_ccloudmanager_requestgetcrewfile_int_rage_rlcloudmemberid_char_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083383_ccloudmanager_requestgettitlefile_char_const_ptr_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_083385_ccloudmanager_requestgetglobalfile_char_const_ptr_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_083390_ccloudmanager_requestgetmemberfile_int_rage_rlcloudmemberid_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083392_ccloudmanager_requestdeletememberfile_int_char_const_ptr_rage_rl(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_083394_ccloudmanager_hasrequestcompleted_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083395_ccloudmanager_didrequestsucceed_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083397_ccloudmanager_checkcloudavailability(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083398_ccloudmanager_getavailabilitycheckresult_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083445_clivemanager_getsocialnetworkingsharingprivileges_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083485_ctunables_startcloudrequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083490_ctunables_iscloudrequestpending(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083493_ctunables_access_unsigned_int_bool_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083494_ctunables_access_unsigned_int_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083495_ctunables_access_unsigned_int_float_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083496_ctunables_tryaccess_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083497_ctunables_tryaccess_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083498_ctunables_tryaccess_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> f32; }
extern "C" { pub fn f_083499_ctunables_access_unsigned_int_ctunables_euniontype(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083500_ctunables_insert_unsigned_int_bool_unsigned_int_unsigned_int_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083501_ctunables_insert_unsigned_int_int_unsigned_int_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083502_ctunables_insert_unsigned_int_float_unsigned_int_unsigned_int_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_083503_ctunables_modificationdetectionregistration_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_083504_ctunables_modificationdetectionregistration_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_083505_ctunables_modificationdetectionclear(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083506_ctunables_getcontentlistid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083507_cusercontentmanager_clearqueryresults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083508_cusercontentmanager_clearcreatorresults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083509_cusercontentmanager_clearcreateresult(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083510_cusercontentmanager_clearmodifyresult(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083521_cusercontentmanager_releaseallcacheddescriptions(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083522_cusercontentmanager_cancelquery(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083523_cusercontentmanager_didcreatesucceed(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083524_cusercontentmanager_didquerysucceed(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083525_cusercontentmanager_didmodifysucceed(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083526_cusercontentmanager_didquerycreatorssucceed(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083527_cusercontentmanager_requestcontentdata_rage_rlugccontenttype_cha(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_083530_cusercontentmanager_copy_char_const_ptr_rage_rlugccontenttype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_083532_cusercontentmanager_querycontent_rage_rlugccontenttype_char_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083533_cusercontentmanager_getbookmarked_int_int_rage_rlugccontenttype(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_083534_cusercontentmanager_getmycontent_int_int_rage_rlugccontenttype_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_083535_cusercontentmanager_getfriendcontent_int_int_rage_rlugccontentty(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_083536_cusercontentmanager_getclancontent_long_int_int_rage_rlugcconten(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_083537_cusercontentmanager_getbycategory_rage_rlugccategory_int_int_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083538_cusercontentmanager_getbycontentid_char_ptr_23_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_083539_cusercontentmanager_getmostrecentlycreated_int_int_rage_rlugccon(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_083540_cusercontentmanager_getmostrecentlyplayed_int_int_rage_rlugccont(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_083541_cusercontentmanager_gettoprated_int_int_rage_rlugccontenttype(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_083544_cusercontentmanager_publish_char_const_ptr_char_const_ptr_rage_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_083545_cusercontentmanager_setbookmarked_char_const_ptr_bool_rage_rlugc(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_083546_cusercontentmanager_setdeleted_char_const_ptr_bool_rage_rlugccon(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_083547_cusercontentmanager_setplayerdata_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_083548_cusercontentmanager_requestcontentdata_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083549_cusercontentmanager_canquerydata_int_bool_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083551_cusercontentmanager_requestcacheddescription_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083552_cusercontentmanager_isdescriptionrequestinprogress_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083553_cusercontentmanager_hasdescriptionrequestfinished_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083554_cusercontentmanager_diddescriptionrequestsucceed_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083555_cusercontentmanager_getcacheddescription_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_083556_cusercontentmanager_releasecacheddescription_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083558_cusercontentmanager_iscreatepending(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083559_cusercontentmanager_hascreatefinished(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083560_cusercontentmanager_getcreatecontentid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083561_cusercontentmanager_hasqueryfinished(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083562_cusercontentmanager_getcontentnum(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083563_cusercontentmanager_getcontenttotal(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083564_cusercontentmanager_getcontenthash(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083565_cusercontentmanager_getcontentid_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_083566_cusercontentmanager_getrootcontentid_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_083567_cusercontentmanager_getcontentcategory_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083568_cusercontentmanager_getcontentname_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_083570_cusercontentmanager_getcontentdescriptionhash_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083571_cusercontentmanager_getcontentfileversion_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083572_cusercontentmanager_getcontenthasloresphoto_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083573_cusercontentmanager_getcontenthashiresphoto_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083574_cusercontentmanager_getcontentlanguage_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083575_cusercontentmanager_getcontentupdateddate_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_083576_cusercontentmanager_getcontentuserid_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_083577_cusercontentmanager_getcontentcreatorgamerhandle_int_rage_rlgame(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083578_cusercontentmanager_getcontentcreatedbylocalplayer_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083579_cusercontentmanager_getcontentusername_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_083580_cusercontentmanager_getcontentisusingscnickname_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083581_cusercontentmanager_iscontentpublished_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083582_cusercontentmanager_iscontentverified_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083583_cusercontentmanager_getcontentpath_int_int_char_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_083585_cusercontentmanager_getrating_int(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_083586_cusercontentmanager_getratingcount_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_083587_cusercontentmanager_getratingpositivecount_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_083588_cusercontentmanager_getratingnegativecount_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_083589_cusercontentmanager_hasplayerrecord_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083591_cusercontentmanager_hasplayerrated_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083592_cusercontentmanager_hasplayerbookmarked_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083593_cusercontentmanager_ismodifypending(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083594_cusercontentmanager_hasmodifyfinished(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083595_cusercontentmanager_querycontentcreators_rage_rlugccontenttype_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_083596_cusercontentmanager_getcreatorsbyuserid_char_ptr_64_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_083597_cusercontentmanager_hasquerycreatorsfinished(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083600_cusercontentmanager_loadofflinequerydata_rage_rlugccategory(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083601_cusercontentmanager_clearofflinequerydata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083602_cusercontentmanager_setcontentfromoffline_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083606_cusercontentmanager_setusingofflinecontent_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083641_ccommercemanager_isentitledtostarterpack(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083643_ccommercemanager_isentitledtopremiumpack(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083644_ccommercemanager_isthindatapopulationinprogress(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083665_ccommercemanager_getthindatabyindex_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083770_networkcolours_getnetworkcolour_networkcolours_networkcolour(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083772_networkcolours_getteamcolour_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083773_networkcolours_getcustomteamcolour_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083782_networkbank_startkilltrackingcivilians(ph: *mut Phantom); }
extern "C" { pub fn f_083783_networkbank_stopkilltrackingcivilians(ph: *mut Phantom); }
extern "C" { pub fn f_083787_cnetworkkilltracker_starttracking_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_083788_cnetworkkilltracker_stoptracking_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_083789_cnetworkheadshottracker_starttracking_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_083790_cnetworkheadshottracker_stoptracking_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_083792_cnetworkbodydamagetracker_reset(ph: *mut Phantom); }
extern "C" { pub fn f_083793_cnetworkbodydamagetracker_getfatalcomponent(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083794_cnetworkbodydamagetracker_getfatalshothistory_cnetworkbodydamage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083795_cnetworkbodydamagetracker_shotinfo_getcomponenthit_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083797_cfakeplayernames_addfakeplayername_cped_ptr_char_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_083798_cfakeplayernames_removefakeplayername_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083824_cnetworkghostcollisions_isinghostcollision_cphysical_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083871_cnetworksynchronisedscenes_issceneactive_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083873_cnetworksynchronisedscenes_getlocalsceneid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083876_cnetworksynchronisedscenes_createscene_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: f32, a7: i64, a8: i32) -> i32; }
extern "C" { pub fn f_083880_cnetworksynchronisedscenes_attachsyncedscenetoentity_int_cphysic(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_083881_cnetworksynchronisedscenes_addpedtoscene_cped_ptr_int_char_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32, a8: f32, a9: i32); }
extern "C" { pub fn f_083882_cnetworksynchronisedscenes_addentitytoscene_cphysical_ptr_int_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32); }
extern "C" { pub fn f_083883_cnetworksynchronisedscenes_addmapentitytoscene_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_083884_cnetworksynchronisedscenes_addcameratoscene_int_char_const_ptr_c(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_083885_cnetworksynchronisedscenes_forcelocaluseofcamera_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083888_cnetworksynchronisedscenes_startscenerunning_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083889_cnetworksynchronisedscenes_stopscenerunning_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_083907_csyncedvehicle_csyncedvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083908_csyncedvehicle_getvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083909_csyncedvehicle_getvehicleid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083919_networkutils_getplayerfromdamageentity_centity_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083920_networkutils_getplayerdamageentity_cnetgameplayer_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083930_networkutils_getsessiontypeasstring_sessiontype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083935_networkutils_getmultiplayergamemodeintasstring_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083936_networkutils_loggamerhandle_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084013_cnetworkentityareaworldstatedata_addentityarea_cgamescriptid_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_084014_cnetworkentityareaworldstatedata_removeentityarea_cgamescriptid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_084015_cnetworkentityareaworldstatedata_hasworldstate_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084016_cnetworkentityareaworldstatedata_haveallreplied_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084017_cnetworkentityareaworldstatedata_isoccupied_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084085_sacceptedinvite_hasfailed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_084088_clivemanager_issystemuishowing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084092_invitemgr_clearinvite_invitemgr_clearflags(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084097_clivemanager_isplatformsubscriptioncheckpending_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084098_clivemanager_hasplatformsubscription_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084102_invitemgr_removersinvite_unsigned_int_invitemgr_removereason(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_084106_invitemgr_cancelinvite(ph: *mut Phantom); }
extern "C" { pub fn f_084109_invitemgr_getunacceptedinvite_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_084112_networkclan_getuiformattedclantag_rage_rlclandesc_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_084116_invitemgr_getrsinviteindexbyinviteid_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084117_invitemgr_acceptrsinvite_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084118_invitemgr_getrsinviteid_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084119_invitemgr_getrsinviteinviter_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_084120_invitemgr_getrsinvitehandle_unsigned_int_rage_rlgamerhandle_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_084121_invitemgr_getrsinvitecontentid_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_084122_invitemgr_getrsinviteplaylistlength_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084123_invitemgr_getrsinviteplaylistcurrent_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084124_invitemgr_getrsinvitefromadmin_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084125_invitemgr_getrsinviteistournament_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084126_invitemgr_actionfollowinvite(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084127_invitemgr_clearfollowinvite(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084128_invitemgr_requestconfirmevent(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084130_invitemgr_suppressprocess_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084131_invitemgr_blockinvitesfromscript_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084132_invitemgr_storeinvitethroughrestart(ph: *mut Phantom); }
extern "C" { pub fn f_084133_invitemgr_allowprocessinplayerswitch_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084134_invitemgr_blockjoinqueueinvites_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084135_invitemgr_setcanreceiversinvites_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084136_invitemgr_isawaitinginviteresponse_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084137_invitemgr_acceptinvite_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084164_clivemanager_canactionprofilechange_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084165_clivemanager_onprofilechangecommon(ph: *mut Phantom); }
extern "C" { pub fn f_084174_clivemanager_checkisagerestricted_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084175_clivemanager_checkvoicecommunicationprivileges_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_084176_clivemanager_checktextcommunicationprivileges_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_084177_clivemanager_getagegroup(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084178_clivemanager_showgamerprofileui_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084179_clivemanager_showgamerfeedbackui_rage_rlgamerhandle_const_ref(ph: *mut Phantom); }
extern "C" { pub fn f_084180_clivemanager_showaccountupgradeui_unsigned_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084182_clivemanager_isaddingfriend(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084183_clivemanager_addfriend_rage_rlgamerhandle_const_ref_char_const_p(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_084184_clivemanager_hasrecentupsellrequest_unsigned_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084185_clivemanager_clearofflineinvitepending(ph: *mut Phantom); }
extern "C" { pub fn f_084187_clivemanager_findgamersviapresence_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084188_clivemanager_isfindinggamers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084189_clivemanager_didfindgamerssucceed(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084190_clivemanager_getnumfoundgamers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084191_clivemanager_getfoundgamer_int_rage_rlgamerquerydata_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_084192_clivemanager_clearfoundgamers(ph: *mut Phantom); }
extern "C" { pub fn f_084193_clivemanager_getgamerstatus_rage_rlgamerhandle_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_084194_clivemanager_queuegamerforstatus_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_084195_clivemanager_getgamerstatusfromqueue(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084196_clivemanager_isgettinggamerstatus(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084197_clivemanager_didgetgamerstatussucceed(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084198_clivemanager_cleargetgamerstatus(ph: *mut Phantom); }
extern "C" { pub fn f_084240_cgamertagfromhandle_start_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_084241_cgamertagfromhandle_pending(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084272_metricpurchase_flow_purchasemetricinfos_savecurrentcash(ph: *mut Phantom); }
extern "C" { pub fn f_084273_metricpurchase_flow_purchasemetricinfos_setfromlocation_estorepu(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084274_getpurchaselocationhash_estorepurchaselocation(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084292_metricugc_nav_metricugc_nav_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_084312_cnetworktelemetry_incrementcloudcashtransactioncount(ph: *mut Phantom); }
extern "C" { pub fn f_084317_cnetworktelemetry_setpreviousmatchhistoryid_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084338_cnetworktelemetry_playerspawn_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_084353_cnetworktelemetry_appendcashcreated_unsigned_long_rage_rlgamerha(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_084358_cnetworktelemetry_setcurrentspawnlocationoption_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084359_cnetworktelemetry_setcurrentpubliccontentid_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084360_cnetworktelemetry_setcurrentchatoption_chatoption(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084361_cnetworktelemetry_setvehicledrivenintestdrive_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084411_cprofilestatsread_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084412_cprofilestatsread_reset_unsigned_int_int_ptr_unsigned_int_cprofi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_084413_cprofilestatsreadscriptrequest_clear(ph: *mut Phantom); }
extern "C" { pub fn f_084443_networkclan_serviceisvalid_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084444_networkclan_getlocalgamermembershipcount(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_084446_networkclan_getmembershipcount_rage_rlgamerhandle_const_ref_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084447_networkclan_getmembership_rage_rlgamerhandle_const_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_084448_networkclan_hasprimaryclan_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084450_networkclan_refreshmemberships_rage_rlgamerhandle_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084451_networkclan_checkforanyopsoftype_networkclan_enetworkclanops_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_084452_networkclan_refreshmembershipspending_rage_rlgamerhandle_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084454_networkclan_attemptjoinclan_long(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_084459_networkclan_getreceivedinvite_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_084507_cgamersprimaryclandata_clear(ph: *mut Phantom); }
extern "C" { pub fn f_084508_cgamersprimaryclandata_cancel(ph: *mut Phantom); }
extern "C" { pub fn f_084509_networkremotecheaterdetector_notifyplayerleftbadsport_rage_rlgam(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_084589_cscriptedcoronaplayercarddatamanager_setgamerhandlename_rage_rlg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_084590_cscriptedcoronaplayercarddatamanager_addgamerhandle_rage_rlgamer(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_084591_cplayercardmanager_init_cplayercardmanager_cardtypes_baseplayerc(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_084604_socialclubmgr_isconnectedtosocialclub_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084618_socialclubmgr_isonlinepolicyuptodate_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084627_networklegalrestrictionsmanager_cangamble_unsigned_int_bool_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_084628_networklegalrestrictionsmanager_evconlyoncasinochipstransactions(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_085461_cnetworksession_kickplayer_rage_netplayer_const_ptr_kickreason_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_085570_cnetworksession_ishost_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085601_socialclubeventmgr_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_085604_socialclubeventmgr_getglobaleventhash_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_085721_networkbaseconfig_getmatchmakingaimbucket_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_085727_networkgameconfig_getgamemode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085746_cnetworksession_launchtransition(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085755_cnetworksession_getmatchmakinggroupnum_ematchmakinggroup_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085756_cnetworksession_getactivityplayernum_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085757_cnetworksession_getactivityplayermax_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085780_cnetworksession_setactivityspectatorsmax_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085781_cnetworksession_setactivityplayersmax_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085785_cnetworksession_updatecurrentuniquecrews_sessiontype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085787_cnetworksession_ishostplayerandrockstardev_rage_rlgamerhandle_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085795_cnetworksession_leavetransition(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085801_cnetworksession_isactivityspectator_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085815_cnetworksession_onreturntosingleplayer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085820_cnetworksession_joininvite(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085823_cnetworksession_updatematchmakinggroups(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085824_cnetworksession_hostsession_int_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_085827_cnetworksession_sendtransitioninvites_rage_rlgamerhandle_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_085828_cnetworksession_sendinvites_rage_rlgamerhandle_const_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_085837_cnetworksession_canstart_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085839_cnetworksession_leavesession_unsigned_int_leavesessionreason(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_085840_cnetworksession_bailtransition_etransitionbailreason_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_085845_cnetworksession_dotransitiontogame_int_rage_rlgamerhandle_ptr_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_085850_cnetworksession_setmatchmakinggroup_ematchmakinggroup(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085851_cnetworksession_addactivematchmakinggroup_ematchmakinggroup(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085857_cnetworksession_isclosedfriendssession_sessiontype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085858_cnetworksession_isclosedcrewsession_sessiontype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085860_cnetworksession_setmatchmakinggroupmax_ematchmakinggroup_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085861_cnetworksession_setuniquecrewlimit_sessiontype_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085862_cnetworksession_setuniquecrewonlycrews_sessiontype_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085863_cnetworksession_getuniquecrewlimit_sessiontype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085864_cnetworksession_setcrewlimitmaxmembers_sessiontype_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085865_cnetworksession_setmatchmakingpropertyid_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085866_cnetworksession_setmatchmakingmentalstate_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085867_cnetworksession_setmatchmakingminimumrank_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085868_cnetworksession_clearfollowers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085869_cnetworksession_canentermultiplayer(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085872_cnetworksession_dofriendmatchmaking_int_int_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_085873_cnetworksession_setsessionvisibilitylock_sessiontype_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_085874_cnetworksession_hostclosed_int_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_085875_cnetworksession_hostclosedfriends_int_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_085884_cnetworksession_istransitionmember_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085885_cnetworksession_setpresenceinvitesblocked_sessiontype_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085886_cnetworksession_canqueueforprevioussessionjoin(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085887_cnetworksession_clearqueuedjoinrequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085888_cnetworksession_sendqueuedjoinrequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085889_cnetworksession_removeallqueuedjoinrequests(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085890_cnetworksession_setsessionvisibility_sessiontype_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085891_cnetworksession_clearwaitflag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085892_cnetworksession_joinprevioussession_joinfailureaction_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_085894_cnetworksession_joinpreviouslyfailed(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085896_cnetworksession_adddelayedpresenceinvite_rage_rlgamerhandle_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_085899_cnetworksession_removeinvites_rage_rlgamerhandle_const_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085900_cnetworksession_cancelinvites_rage_rlgamerhandle_const_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085901_cnetworkinvitedplayers_didinvite_rage_rlgamerhandle_const_ref_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085902_cnetworksession_removeandcancelallinvites(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085904_cnetworksession_issolosession_sessiontype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085905_cnetworkvoicesession_isvoicesessionactive(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085906_cnetworksession_issessionvisible_sessiontype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085908_cnetworksession_isprivatesession_sessiontype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085909_cnetworksession_setsessioninprogress_sessiontype_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085910_cnetworksession_getsessioninprogress_sessiontype(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085911_cnetworksession_setcontentcreator_sessiontype_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085912_cnetworksession_getcontentcreator_sessiontype(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085913_cnetworksession_setactivityisland_sessiontype_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085914_cnetworksession_getactivityisland_sessiontype(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085915_cnetworksession_issessionvisibilitylocked_sessiontype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085916_cnetworksession_changesessionslots_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085917_cnetworksession_getmatchmakinggroupfree_ematchmakinggroup_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_085918_cnetworksession_setblockjoinrequests_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085919_cnetworksession_getsessionmembers_rage_rlgamerinfo_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085920_cnetworksession_issessionmember_rage_rlgamerhandle_const_ref_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085924_cnetworksession_isinvitedecisionmade_rage_rlgamerhandle_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085926_cnetworksession_getinvitestatus_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085927_cnetworksession_canceltransitionmatchmaking(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085929_cnetworksession_hosttransition_int_unsigned_int_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_085934_cnetworksession_setactivityspectator_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085935_cnetworksession_dotransitionquickmatch_int_unsigned_int_int_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64, a8: i32) -> i32; }
extern "C" { pub fn f_085936_cnetworksession_markaspreferredactivity_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085937_cnetworksession_markaswaitingasync_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085938_cnetworksession_setinprogressfinishtime_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085939_cnetworksession_setactivitycreatorhandle_rage_rlgamerhandle_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085940_cnetworksession_setnumbosses_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085941_cnetworksession_joingroupactivity(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085942_cnetworksession_cleargroupactivity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085943_cnetworksession_retainactivitygrouponquickmatchfail(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085944_cnetworksession_dotransitiontonewgame_int_rage_rlgamerhandle_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_085946_cnetworksession_joinpreviouslyfailedtransition(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085947_cnetworksession_joinplayertransition_cnetgameplayer_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085948_cnetworksession_settransitionactivityid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085949_cnetworksession_changetransitionslots_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085950_cnetworksession_setblocktransitionjoinrequests_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085952_cnetworksession_setdonotlaunchfromjoinasmigratedhost_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085953_cnetworkvoice_setoverridetransitionchatscript_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085954_cnetworkvoice_setremainingamechat_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085956_cnetworksession_removetransitioninvites_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085957_cnetworksession_removealltransitioninvites(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085958_cnetworksession_canceltransitioninvites_rage_rlgamerhandle_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085959_cnetworksession_removeandcancelalltransitioninvites(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085960_cnetworksession_addtransitioninvites_rage_rlgamerhandle_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_085961_cnetworksession_getnumtransitionnonasyncgamers(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085962_cnetworksession_gettransitionmembers_rage_rlgamerinfo_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_085964_cnetworksession_gettransitionhost_rage_rlgamerhandle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085965_cnetworksession_applytransitionparameter_unsigned_short_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_085966_cnetworksession_applytransitionparameter_int_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_085968_cnetworksession_sendtransitiongamerinstruction_rage_rlgamerhandl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_085970_cnetworksession_marktransitionplayerasfullyjoined_rage_rlgamerha(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086019_cnetworkvoice_sendtextmessage_char_const_ptr_rage_rlgamerhandle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_086045_cnetworkinvitedplayers_isacked_rage_rlgamerhandle_const_ref_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086059_cnetworkvoicesession_leave(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086060_cnetworkvoicesession_isbusy(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086061_cnetworkvoicesession_host(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086062_cnetworkvoicesession_isinvoicesession(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086063_cnetworkvoicesession_settimeout_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086064_cnetworkvoicesession_invitegamer_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086065_cnetworkvoicesession_respondtorequest_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_086127_cnetworkvoice_cancommunicatetextwithgamer_rage_rlgamerhandle_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086130_cnetworkvoice_setoverridechatrestrictions_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_086132_cnetworkvoice_setoverridechatrestrictionssend_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_086133_cnetworkvoice_setoverridechatrestrictionsreceive_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_086134_cnetworkvoice_islocaltalking_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_086135_cnetworkvoice_isgamertalking_rage_rlgamerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086138_cnetworkvoice_localhasheadset_int_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086139_cnetworkvoice_gamerhasheadset_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086140_cnetworkvoice_mutegamer_rage_rlgamerhandle_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_086142_cnetworkvoice_hasgamerrecord_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086143_cnetworkvoice_isgamermutedbyme_rage_rlgamerhandle_const_ref_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086144_cnetworkvoice_amimutedbygamer_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086145_cnetworkvoice_isgamerblockedbyme_rage_rlgamerhandle_const_ref_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086146_cnetworkvoice_amiblockedbygamer_rage_rlgamerhandle_const_ref_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086147_cnetworkvoice_settalkerfocus_rage_rlgamerhandle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086148_cnetworkvoice_settalkerproximity_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_086149_cnetworkvoice_setcommunicationactive_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086150_cnetworkvoice_setteamonlychat_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086151_cnetworkvoice_setproximityaffectsteam_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086152_cnetworkvoice_setnospectatorchat_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086153_cnetworkvoice_setignorespectatorlimitsforsameteam_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086154_cnetworkvoice_setoverrideteam_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_086155_cnetworkvoice_setoverridespectatormode_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086156_cnetworkvoice_setoverridetutorialsession_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086157_cnetworkvoice_setoverridechatrestrictionssendall_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086158_cnetworkvoice_setoverridechatrestrictionsreceiveall_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086159_cnetworkvoice_getplayerloudness_int(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_086160_cnetworkvoice_enablebandwidthrestrictions_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086161_cnetworkvoice_disablebandwidthrestrictions_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086183_netcatalogpackedstatinventoryitem_setupstatsmapping(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086218_netcatalogbaseitem_netcatalogbaseitem_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_086219_cnetworkshoppingmgr_getcategoryisvalid_unsigned_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_086250_netcatalog_hasreadsucceeded_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_086251_netcatalog_hasreadfailed_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_086252_netcatalog_find_rage_atnonfinalhashstring_ref_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086255_netcatalog_additem_rage_atnonfinalhashstring_const_ref_netcatalo(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086259_cnetworkshoppingmgr_shoulddonulltransaction_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_086264_gametransactionsessionmgr_clearstartsession_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_086266_netcatalog_linkitems_rage_atnonfinalhashstring_ref_rage_atnonfin(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086267_netcatalog_areitemslinked_rage_atnonfinalhashstring_ref_rage_atn(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086268_netcatalog_removelinkeditems_rage_atnonfinalhashstring_ref_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086295_cnetshoptransactionbase_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086302_networkgametransactions_dogametransactionevent_int_networkgametr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_086305_cnetshoptransactionbase_registerpendingcash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086310_cnetshoptransactionbase_applydatatostats_cnetshoptransactionserv(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_086316_networkgametransactions_sendnullevent_int_rage_netstatus_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_086333_gametransactionsessionmgr_isqueuebusy_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086334_cnetworkshoppingmgr_addtogrcdebugdraw_sstandalonegametransaction(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086335_cnetworkshoppingmgr_createfreespaces_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086336_cnetworkshoppingmgr_appendnewnode_cnetshoptransactionbase_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086337_cnetworkshoppingmgr_createbasket_unsigned_int_ref_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_086338_cnetworkshoppingmgr_deletebasket(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086339_cnetworkshoppingmgr_additem_cnetshopitem_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086340_cnetworkshoppingmgr_findbasket_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_086341_cnetworkshoppingmgr_isbasketfull_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086342_cnetworkshoppingmgr_startcheckout_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_086343_cnetworkshoppingmgr_getstatus_unsigned_int_rage_netstatus_status(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_086344_cnetworkshoppingmgr_findtransaction_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_086345_cnetworkshoppingmgr_getactiontypeisvalid_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_086404_networkgametransactions_deleteslot_int_int_bool_bool_int_network(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_086415_gametransactionsessionmgr_sessionstart_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_086419_gametransactionsessionmgr_initmanager_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_086446_gametransactionsessionmgr_applydata(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086455_gametransactionsessionmgr_triggergetbalance_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_086744_cnetgameplayer_getplayerped_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086747_cnetworkobjectpopulationmgr_canregisterobject_unsigned_short_boo(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_086763_cnetobjgame_getplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086790_cnetworkobjectmgr_addinvalidobjectmodel_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086791_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_086792_cnetworkobjectmgr_removeinvalidobjectmodel_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086793_cnetworkobjectmgr_clearinvalidobjectmodels(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086800_cnetworkobjectpopulationmgr_canregisterobjects_unsigned_int_unsi(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_086860_cnetobjentity_setoverridinglocalvisibility_bool_char_const_ptr_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_086895_cnetobjobject_canbenetworked_cobject_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_086915_cnetobjentity_setoverridingremotevisibility_bool_bool_char_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_086958_cnetgameplayer_getplayerinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087016_cnetobjobject_setusehighprecisionblending_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087017_cnetobjobject_setisarenaball_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087022_cnetobjobject_setusehighprecisionrotation_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087029_cnetobjphysical_concealentity_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087143_cnetobjphysical_setalphafading_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087291_cnetobjped_setistargettablebyteam_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087292_cnetobjped_setistargettablebyplayer_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087342_cnetobjphysical_getlastdamageobjectentity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087356_cnetobjphysical_activatedamagetracking(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087357_cnetobjphysical_istrackingdamage(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087358_cnetobjphysical_deactivatedamagetracking(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087359_cnetobjphysical_getdamagedealtbyplayer_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_087390_cnetobjphysical_setalpharampinginternal_bool_bool_bool_bool_shor(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_087406_cnetobjphysical_setinvertghosting_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_087535_cnetobjplayer_setrespawninvincibilitytimer_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087536_cnetobjplayer_setdisablerespawnflashing_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087540_cnetobjplayer_startsolotutorial(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087541_cnetobjplayer_startgangtutorial_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087542_cnetobjplayer_endtutorial(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087554_cnetobjplayer_clearcachedplayerheadblenddata_unsigned_char(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_087555_cnetobjplayer_hascachedplayerheadblenddata_unsigned_char(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_087610_cnetgameplayer_applyvoiceproximityoverride_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087611_cnetgameplayer_clearvoiceproximityoverride(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087626_cnetgameplayer_setmutedata_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087635_cnetobjplayer_setantagonisticplayerindex_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087642_cnetobjplayer_canrespawnlocalplayerped(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087643_cnetobjplayer_respawnplayerped_rage_vector3_const_ref_bool_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_087646_cnetobjplayer_setlookattalkers_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_087652_cnetobjplayer_resetplayerghostedfornonscriptpartipants_char_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087653_cnetobjplayer_setplayerghostedfornonscriptpartipants_cgamescript(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_087654_cnetobjplayer_forcesendplayerappearancesyncnode(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087660_cnetobjplayer_applycachedplayerheadblenddata_cped_ref_unsigned_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_088035_csyncedpedvehicleentryscriptconfig_validatedata_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_088036_csyncedpedvehicleentryscriptconfig_spewdatatotty_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_088037_csyncedpedvehicleentryscriptconfig_hasnondefaultstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_089329_crequestcontrolevent_trigger_rage_netobject_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_089404_cdebuggameclockevent_trigger_unsigned_int_rage_netplayer_const_p(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_089419_crespawnplayerpedevent_trigger_rage_vector3_const_ref_unsigned_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32); }
extern "C" { pub fn f_089749_cscriptentitystatechangeevent_trigger_rage_netobject_ptr_cscript(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_089871_cnetworkcrchashcheckevent_triggerrequest_unsigned_char_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_089924_cupdateplayerscarsevent_trigger(ph: *mut Phantom); }
extern "C" { pub fn f_089992_cupdatefxnevent_trigger_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090817_gmtime_r(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090911_rand(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090918_sinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090920_snprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090921_siprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090926_strcasecmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090930_strcmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090939_strlen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
