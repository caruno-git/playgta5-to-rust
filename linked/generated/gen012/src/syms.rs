//! Callee declarations for crate gen012.
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
extern "C" { pub fn f_000121_rage_sysipclockmutexcommand_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000122_rage_sysipcunlockmutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000566_wasm_start_mode_now(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000568_wasm_start_mode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000644_rage_athashnextsize_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000650_rage_athash16u_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000680_rage_athashstringnamespacesupport_computehash_rage_athashstringn(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_000681_rage_athashstringnamespacesupport_addstring_rage_athashstringnam(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_000684_rage_athashstringnamespacesupport_trygetstring_rage_athashstring(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_000703_rage_bkbank_unsetcurrentgroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000704_rage_bkbank_pushgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000707_rage_bkbank_popgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000782_rage_bkmanager_findbank_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000786_rage_bkmanager_isbankthread(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000806_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000867_rage_bkgroup_addtitle_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000877_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000884_rage_bkgroup_addtoggle_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000894_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000907_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int_rage_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000918_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000990_char_ptr_rage_formatf_char_512ul_char_ref_512ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001124_rage_datgrowbuffer_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001125_rage_datgrowbuffer_dtordatgrowbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001132_rage_datgrowbuffer_appendorfail_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001138_rage_datgrowbuffer_getbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001232_rage_rsonreader_rsonreader(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001234_rage_rsonreader_init_char_const_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001239_rage_rsonreader_getnextsibling_rage_rsonreader_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001240_rage_rsonreader_getfirstmember_rage_rsonreader_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001243_rage_rsonreader_getname_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001269_rage_rsonreader_validatejson_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001514_rage_diagdisplayf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001516_rage_fistream_write_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001524_displayf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001527_warningf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001530_rage_fistream_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001532_rage_fistream_close(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001534_quitf_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001535_rage_diaglogf2_rage_diagchannel_const_ref_rage_diagseverity_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001537_rage_fistream_size(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001555_rage_diaglogf_assert_rage_diagchannel_const_ref_char_const_ptr_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001560_rage_diagcontextmessage_diagcontextmessage_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001563_rage_diagcontextmessage_pop(ph: *mut Phantom); }
extern "C" { pub fn f_001568_rage_atmapentry_unsigned_int_wchar_t_ptr_const_ref_rage_atmap_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001580_rage_fiassetmanager_open_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001582_rage_fiassetmanager_createleadingpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001622_rage_fistream_read_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001635_rage_fiassetmanager_filename_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001636_rage_fiassetmanager_basename_char_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001640_rage_fiassetmanager_removenamefrompath_char_ptr_int_char_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001643_rage_fistream_create_char_const_ptr_rage_fidevice_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001652_rage_fidevice_makememoryfilename_char_ptr_int_void_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002035_rage_mthrandommwc_getint(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002129_rage_atmapentry_rage_atnonfinalhashstring_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002131_rage_atnonfinalhashstring_atnonfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002161_rage_parstructure_issubclassof_rage_parstructure_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_002296_rage_atmap_unsigned_int_rage_atarray_rage_psorscbuilderinstances(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002739_rage_atpartialstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002743_rage_atdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002757_rage_countutf8characters_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002762_rage_utf8towide_unsigned_short_ptr_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002790_rage_zlibinflater_zlibinflater(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002799_rage_sysappcontent_isjapanesebuild(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002845_rage_sysparam_searchargarray_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002881_unsigned_int_ref_rage_atmap_unsigned_int_unsigned_int_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002886_rage_systhreadpool_thread_dtorthread(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002888_rage_systhreadpool_workitem_workitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002889_rage_systhreadpool_workitem_dtorworkitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002894_rage_systhreadpool_workitem_finished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002898_rage_systhreadpool_dtorsysthreadpool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002904_rage_systhreadpool_queuework_rage_systhreadpool_workitem_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002919_rage_sysservice_isconstrained_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002920_rage_sysservice_getconstraineddurationms_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002921_rage_sysservice_getsystemlanguage(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_002945_rage_sysmempoolallocator_safeallocate_unsigned_long_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003137_rage_geompoints_ispointbehindplane_rage_vector3_const_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_003143_rage_geomtvalues_findtvalueopensegtopointv_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_003144_rage_geomsegments_colliderayplane_rage_vector3_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_003159_rage_matrix34_fromquaternion_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003163_rage_quaternion_slerp_float_rage_quaternion_const_ref_rage_quate(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_003175_rage_vector3_vector3_rage_vector2_const_ref_rage_vector2_evector(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003831_rage_fwanimdirectorcomponentsyncedscene_isplayingsyncedscene_con(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003832_rage_fwanimdirectorcomponentsyncedscene_isvalidsceneid_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003835_rage_fwanimdirector_getmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003836_rage_fwanimdirector_findscriptvisiblepropertyinphaserange_rage_c(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32) -> i64; }
extern "C" { pub fn f_003895_rage_fwanimmanager_getclipifexistsbyname_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003896_rage_fwanimmanager_getclipifexistsbydictindex_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_003904_rage_fwanimmanager_findslot_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003905_rage_fwanimmanager_findslotfromhashkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004280_rage_fwposematcher_fwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004377_rage_fwsyncedsceneaudiointerface_dtorfwsyncedsceneaudiointerface(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004388_rage_fwanimdirectorcomponentsyncedscene_updatesyncedscenemover(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004398_rage_fwanimdirectorcomponentsyncedscene_getsyncedscenephase_shor(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004407_rage_fwanimdirectorcomponentsyncedscene_issyncedsceneholdlastfra(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004603_rage_fwextensionlist_add_rage_fwextension_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004619_rage_fwarchetype_getdrawable_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004632_rage_fwarchetypemanager_lookupmodelid_rage_fwarchetype_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004634_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004661_rage_fwdynamicentitycomponent_fwdynamicentitycomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004693_rage_fwsceneupdate_isinsceneupdate_rage_fwentity_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004735_rage_fwdecorator_get_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004736_rage_fwdecorator_get_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004737_rage_fwdecorator_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004738_rage_fwdecorator_get_rage_atnonfinalhashstring_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004739_rage_fwdecorator_get_rage_fwdecorator_time_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004740_rage_fwdecoratorextension_existson_rage_fwextensiblebase_ref_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_004741_rage_fwdecoratorextension_removefrom_rage_fwextensiblebase_ref_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_004742_rage_fwdecoratorextension_removeallfrom_rage_fwextensiblebase_re(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004744_rage_fwdecoratorextension_fwdecoratorextension_rage_fwextensible(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004747_rage_fwdecoratorextension_update_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_004748_rage_fwdecoratorextension_find_rage_atnonfinalhashstring_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004749_rage_fwdecoratorextension_update_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_004750_rage_fwdecoratorextension_update_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_004751_rage_fwdecoratorextension_update_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004752_rage_fwdecoratorextension_update_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_004758_rage_fwdecoratorinterface_register_rage_atnonfinalhashstring_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_004759_rage_fwdecoratorinterface_isregisteredastype_rage_atnonfinalhash(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004926_rage_dldrawcommandbuffer_allocatepagedmemory_rage_edrawlistpaget(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_004949_rage_dldrawlistmgr_getdrawlistname_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_005032_rage_fwlanguagepack_getdateformattype_rage_syslanguage(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005033_rage_fwlanguagepack_getdateformatdelimitertype_rage_syslanguage(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005054_rage_fwtextutil_bytecountforcharacter_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005055_rage_fwtextutil_strcpywithcharacterandbytelimits_char_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_005461_rage_fwboxstreamer_hasloadedaboutpos_rage_vec3v_const_ref_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_005606_rage_fwmapdatastore_performstreamingintegritycheck(ph: *mut Phantom); }
extern "C" { pub fn f_005730_rage_fwinteriorlocation_makeinvalid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005732_rage_fwinteriorlocation_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005733_rage_fwinteriorlocation_getroomindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005780_rage_rstcalculatescreenspaceboundisvis_rage_rstsplatmat44v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_005946_rage_aitasktree_cleartask_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005947_rage_aitasktree_getactiveleaftask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005949_rage_aitasktree_findtaskbytypeactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005950_rage_aitasktree_findtaskbytypewithpriority_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_006175_rage_strstreaminginfomanager_clearrequiredflag_rage_strindex_uns(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006225_rage_atvarstring_atvarstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_006235_rage_strstreamingvisualize_setmarkertext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006241_rage_strstreamingvisualize_addmarker_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006248_rage_strstreamingmodule_updateplacestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006343_rage_tcmanager_findmodifierindex_rage_atnonfinalhashstring_char(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006393_rage_tcmanager_addmodifieroverride_rage_atnonfinalhashstring_rag(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007025_rage_atmapentry_unsigned_int_unsigned_short_const_ref_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007412_rage_audentity_createandplaysound_unsigned_int_rage_audsoundinit(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007457_rage_decalmanager_wash_float_rage_fwentity_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32); }
extern "C" { pub fn f_007494_rage_decalmanager_washinrange_float_rage_vec3v_const_ref_rage_sc(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_007495_rage_decalmanager_getwashlevel_int(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_007496_rage_decalmanager_isalive_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007497_rage_decalmanager_remove_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007498_rage_decalmanager_removefacing_rage_vec3v_const_ref_rage_fwentit(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007499_rage_decalmanager_removeinrange_rage_vec3v_const_ref_rage_scalar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007500_rage_decalmanager_fadeinrange_rage_vec3v_const_ref_rage_scalarv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_007501_rage_decalmanager_updateattachentity_rage_fwentity_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007503_rage_decalmanager_findclosest_unsigned_int_signed_char_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: f32, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_007514_rage_decalsettingsmanager_patchdiffusemap_unsigned_int_rage_grct(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_007528_rage_ptfxattach_attach_rage_ptxeffectinst_ptr_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_007547_rage_ptfxmanager_addptfxoverrideinfo_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_007549_rage_ptfxmanager_removeptfxoverrideinfo_char_const_ptr_rage_scrt(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007559_rage_ptfxmanager_gettriggeredinst_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007568_rage_ptfxscript_trigger_rage_atnonfinalhashstring_rage_atnonfina(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: f32, a7: i32) -> i32; }
extern "C" { pub fn f_007571_rage_ptfxscript_settriggeredcolourtint_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32); }
extern "C" { pub fn f_007572_rage_ptfxscript_settriggeredalphatint_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_007573_rage_ptfxscript_settriggeredscale_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_007574_rage_ptfxscript_settriggeredemittersize_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007575_rage_ptfxscript_settriggeredforcevehicleinteriorfx_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007580_rage_ptfxscript_setremovewhendestroyed_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_007581_rage_ptfxscript_doesexist_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_007582_rage_ptfxscript_updateoffset_int_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_007585_rage_ptfxscript_updatecolourtint_int_rage_vec3v_const_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_007586_rage_ptfxscript_updatealphatint_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_007588_rage_ptfxscript_updatescale_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_007589_rage_ptfxscript_updatefarclipdist_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_007590_rage_ptfxscript_updateforcevehicleinteriorfx_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_007632_rage_fwidinstanceinfo_dtorfwidinstanceinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007634_rage_fwuniqueobjectidgenerator_getid_void_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007637_rage_fwuniqueobjectidgenerator_getidinstanceinfofromid_unsigned(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007753_rage_netinterface_getlocalplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007759_rage_netinterface_getnumphysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007762_rage_netinterface_getallphysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007803_rage_networklogutils_writelogevent_rage_netlogginginterface_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007848_rage_neteventmgr_checkforspaceinpool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007849_rage_neteventmgr_prepareevent_rage_netgameevent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007862_rage_nethostbroadcastdatahandlerbase_setnextfreehostobjectid_uns(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007949_rage_netgameevent_netgameevent_unsigned_short_bool_bool_rage_net(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_008074_rage_netobject_ispendingownerchange_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008103_rage_netplayer_getrlpeerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008406_rage_netscgamerdatamgr_isrockstardev_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_008686_rage_fwbasepool_getindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008689_rage_fwscriptguid_createguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008692_rage_fwscriptguid_getguidfrombase_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008693_rage_fwscriptguid_getbasefromguid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_008706_rage_scripthandler_getscriptobject_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008710_rage_scripthandler_getscriptresource_unsigned_short_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_008713_rage_scripthandler_getscriptresource_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008768_rage_scripthandlermgr_removescriptresource_unsigned_short_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_008770_rage_scripthandlermgr_getscripthandlerforresource_unsigned_short(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_008778_rage_scripthandlernetcomponent_ishostlocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008781_rage_scripthandlernetcomponent_isplayeraparticipant_rage_netplay(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008841_rage_scriptinterface_isnetworkscript_rage_scriptidbase_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008925_rage_fwtimer_startscriptpause(ph: *mut Phantom); }
extern "C" { pub fn f_008926_rage_fwtimer_endscriptpause(ph: *mut Phantom); }
extern "C" { pub fn f_008950_rage_fwbasepool_getjustindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008952_rage_fwbasepool_getat_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008953_rage_fwknownrefholder_fwknownrefholder_void_ptr_ptr_rage_fwknown(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_009048_rage_crproperty_getattribute_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009233_rage_crskeleton_getglobalmtx_unsigned_int_rage_mat34v_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009556_rage_grcbatcher_reserve_unsigned_int_unsigned_long(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_009748_rage_grcdrawpolygon_rage_vector3_const_ptr_int_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_010040_rage_grcdevice_gethidef(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_010154_rage_batcherlock_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010159_rage_grcdebugdraw_setdisplaydebugtext_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010161_rage_grcdebugdraw_startrecording_rage_grcdebugdraw_lingerbuffer(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_010163_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010164_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010167_rage_grcdebugdraw_poly_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010172_rage_grcdebugdraw_text_float_float_rage_color32_char_const_ptr_b(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i64, a4: i32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010179_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010181_rage_grcdebugdraw_poly_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010182_rage_grcdebugdraw_poly_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_010183_rage_grcdebugdraw_cross_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010188_rage_grcdebugdraw_boxaxisaligned_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010224_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010225_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_011222_rage_cutfcutscenefile2_findobjectsoftype_int_rage_atarray_rage_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_011419_rage_cutsmanager_setsceneoffset_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011420_rage_cutsmanager_setscenerotation_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_011443_rage_cutsmanager_isconcatsectionvalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011451_rage_cutsmanager_storeoriginalorientationdata_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011453_rage_cutsmanager_isconcatted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011483_rage_cutsmanager_getstatename_rage_cutsmanager_ecutscenestate(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_011751_rage_fraginst_getcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011804_rage_fraginst_forcearticulatedcollidermode(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012183_rage_ptxeffectinst_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012196_rage_ptxeffectinst_setbasemtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012207_rage_ptxeffectinst_setevovaluefromhash_rage_athashvalue_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_012871_rage_environmentcloth_updatecloth_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_013198_rage_rsttestaabbexact_rage_mat44v_const_ref_rage_scalarv_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_014013_rage_nettask_dtornettask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014015_rage_nettask_destroy_rage_nettask_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014016_rage_nettask_run_rage_nettask_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014017_rage_nettask_wascanceled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014023_rage_nettask_nettask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014028_rage_nettask_allocate_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014257_rage_is_ui_showing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014259_rage_rltaskmanager_doaddtask_unsigned_long_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014260_rage_rltaskmanager_destroytask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014303_rage_rlgamerhandle_op_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014315_rage_rlgamerinfo_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014316_rage_rlgamerinfo_getgamerhandle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014320_rage_rlpc_isinitialized(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014361_rage_rlros_getcredentials_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014362_rage_rlroscredentials_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014459_rage_rltaskbase_configure_rage_netstatus_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014662_rage_rlmetric_dtorrlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014685_rage_rlmetric_rlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014791_rage_rlprofilestatsvalue_getfloat_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_014792_rage_rlprofilestatsvalue_getint32_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014793_rage_rlprofilestatsvalue_getint64_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014836_rage_rlroscredentials_hasprivilege_rage_rlrosprivilegeid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014859_rage_rlros_getdefaultprivilege_rage_rlrosprivilegeid(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014972_rage_rlugcupdatecontenttask_ptr_rage_rltaskmanager_createtask_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014973_rage_rlugcupdatecontenttask_configure_int_rage_rlugccontenttype(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i32, a9: i64, a10: i64) -> i32; }
extern "C" { pub fn f_014977_rage_rlugccontenttypefromstring_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014979_rage_rlugcmetadata_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014988_rage_rlugcplayer_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015040_rage_rlcloudmemberid_rlcloudmemberid_rage_rlgamerhandle_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015053_rage_rlcloud_deletememberfile_int_rage_rlcloudonlineservice_char(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_015057_rage_fidevicecloud_setaccessparameters_int_rage_rlcloudnamespace(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_015058_rage_fidevicecloud_pending_fihandle_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015059_rage_fidevicecloud_succeeded_fihandle_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015882_rage_phconstraintmgr_insert_rage_phconstraintbase_params_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015966_rage_phsleep_reset_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016011_rage_phsimulator_setlastinstancematrix_rage_phinst_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016045_rage_pharchetype_removeincludeflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016088_rage_phinst_setinstflag_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016899_rage_phlevelnodetree_rage_phlooseoctreenode_reserveinstlastmatri(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016944_rage_scrthread_preprintstacktrace(ph: *mut Phantom); }
extern "C" { pub fn f_016946_rage_scrthread_setfaultonfunc_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016949_rage_scrthread_getactivethread(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_016950_rage_scrthread_isthisthreadrunningascript(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_016962_rage_scrthread_registerunusedcommand_unsigned_long_void_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016967_rage_scrthread_registercommand_unsigned_long_void_ptr_rage_scrth(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016984_rage_scrthread_checkstackavailability_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_017082_rage_scrthread_info_getvector3_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018890_cgamelogic_forcestateplaying(ph: *mut Phantom); }
extern "C" { pub fn f_018977_cgpsslot_clear_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_018978_cgpsslot_start_rage_vector3_ref_centity_ptr_int_bool_int_rage_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_018994_cgps_setgpsflagsforscript_rage_scrthreadid_unsigned_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_018995_cgps_cleargpsflagsonscriptexit_rage_scrthreadid(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_018998_cgps_setgpsflashes_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019003_cgps_addpointtoracetrack_rage_vector3_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019004_cgps_renderracetrack_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019005_cgps_startmultigps_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_019006_cgps_rendermultigps_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019007_cgps_startcustomroute_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_019008_cgps_initcustomroutefromwaypointrecordingroute_char_const_ptr_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_019010_cgps_initcustomroutefromassistedmovementroute_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_019011_cgps_rendercustomroute_bool_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_019174_crestart_addhospitalrestartpoint_rage_vector3_const_ref_float_in(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_019175_crestart_disablehospitalrestartpoint_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_019176_crestart_addpolicerestartpoint_rage_vector3_const_ref_float_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_019177_crestart_disablepolicerestartpoint_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_019178_crestart_addsavehouse_rage_vector3_const_ref_float_char_const_pt(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_019179_crestart_setsavehouseenabled_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_019180_crestart_setsavehouseoverride_bool_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i32, a1: i64, a2: f32); }
extern "C" { pub fn f_019181_crestart_overridenextrestart_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_019182_crestart_canceloverriderestart(ph: *mut Phantom); }
extern "C" { pub fn f_019183_crestart_setignorenextrestart_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019232_cstuntjumpmanager_abortstuntjumpinprogress(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019237_cstuntjumpmanager_getstuntjumpcompletedstat(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_019238_cstuntjumpmanager_gettotalstuntjumpcompletedstat(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_019241_cstuntjumpmanager_isastuntjumpinprogress(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_019242_cstuntjumpmanager_isstuntjumpmessageshowing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_019243_cstuntjumpmanager_addone_rage_vec3v_const_ref_rage_vec3v_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_019245_cstuntjumpmanager_addoneangled_rage_vec3v_const_ref_rage_vec3v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64, a6: f32, a7: i64, a8: i32, a9: i32, a10: i32) -> i32; }
extern "C" { pub fn f_019246_cstuntjumpmanager_deleteone_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019247_cstuntjumpmanager_enableset_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019248_cstuntjumpmanager_disableset_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019451_ailogging_getbooleanasstring_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_019453_ailogging_getdynamicentitynamesafe_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019969_canimviewer_cameracapture_stop(ph: *mut Phantom); }
extern "C" { pub fn f_020636_ctaskscriptedanimation_setrate_float_ctaskscriptedanimation_epla(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_020637_ctaskscriptedanimation_getblendphaseid_ctaskscriptedanimation_ep(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_020638_ctaskscriptedanimation_getblendphaseoutid_ctaskscriptedanimation(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022229_audweaponaudioentity_togglethermalvision_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_022381_void_creplayeventmanager_storeevent_cpacketsoundphonering_cpacke(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_023396_cpacketevent_cpacketevent_ereplaypacketid_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023579_cpacketstartscriptptfx_cpacketstartscriptptfx_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32) -> i64; }
extern "C" { pub fn f_023585_cpacketstopscriptptfx_cpacketstopscriptptfx(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023591_cpackettriggeredscriptptfx_cpackettriggeredscriptptfx_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: f32, a7: i32) -> i64; }
extern "C" { pub fn f_023593_cpackettriggeredscriptptfxcolour_cpackettriggeredscriptptfxcolou(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32) -> i64; }
extern "C" { pub fn f_023604_cpacketdisabledecalrendering_cpacketdisabledecalrendering(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023829_creplaycontrol_shutdownsession_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_023885_creplaymgr_recordwarpedentity_cdynamicentity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023906_creplaycontrol_isplayeroutofcontrol(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023927_void_creplayeventmanager_trackrecordingevent_unsigned_long_ctrac(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_023929_void_creplayeventmanager_trackrecordingevent_int_ctrackedeventin(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_023930_creplayeventmanager_shouldproceedwithpacket_cpacketevent_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023933_creplayeventmanager_reset(ph: *mut Phantom); }
extern "C" { pub fn f_023964_creplayinterfacemanager_registerallcurrententities(ph: *mut Phantom); }
extern "C" { pub fn f_023965_creplayinterfacemanager_deregisterallcurrententities(ph: *mut Phantom); }
extern "C" { pub fn f_023967_creplayinterfacemanager_ensureisbeingrecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023968_creplayinterfacemanager_ensurewillberecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024266_creplaymgrinternal_isawareofworldforrecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024286_creplaymgrinternal_isplaybackflagset_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_024304_creplaymgrinternal_getbuildinginfo_cphysical_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_024305_creplaymgrinternal_recordmapobject_centity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024318_creplaytrailcontroller_creplaytrailcontroller(ph: *mut Phantom); }
extern "C" { pub fn f_024319_creplaytrailcontroller_addpolyfromgame_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64, a10: i64); }
extern "C" { pub fn f_024340_replaymoviecontrollernew_onlinknamedrendertarget_int_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_026165_caminterface_isdominantrendereddirector_cambasedirector_const_re(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026171_caminterface_setscriptedcameraisfirstpersonthisframe_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026173_camgameplaydirector_getthirdpersoncamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026174_camgameplaydirector_getfirstpersonshootercamera_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026179_camcinematicdirector_isrenderingcinematicmountedcamera_bool_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026180_camcinematicdirector_isrenderingcinematicidlecamera_bool_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026183_caminterface_setdeathfaileffectstate_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026184_camcinematicdirector_invalidateidlecamera(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026187_cambasedirector_isshaking_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026190_caminterface_fadeout_int_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026193_caminterface_isfadingout(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026194_caminterface_isfadedout(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026195_caminterface_fadein_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026198_caminterface_isfadingin(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026199_caminterface_isfadedin(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026202_caminterface_isspherevisibleingameviewport_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_026207_caminterface_computefocuspedonscreen_float_int_float_float_float(ph: *mut Phantom, a0: f32, a1: i32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32, a8: i32) -> i64; }
extern "C" { pub fn f_026214_caminterface_disablenearclipscanthisupdate(ph: *mut Phantom); }
extern "C" { pub fn f_026241_camcinematicdirector_invalidatecinematicvehicleidlemode(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026242_camcinematicdirector_isrenderingcinematicpointofviewcamera_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026246_camcinematicdirector_getbonnetvehiclecamera_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026259_rage_fwentity_getskeleton_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026263_camcinematicdirector_findcontext_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_026267_cambasecinematiccontext_getcamera(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026268_camgameplaydirector_ishintactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026279_rage_matrix34_toeulersyxz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026286_camgameplaydirector_getthirdpersonaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026288_camgameplaydirector_getfollowvehiclecamera_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026415_cambaseobject_getnamehash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026419_cambasecinematiccontext_getcurrentshot_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026457_camcontrolhelper_getviewmodeforcontext_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_026461_camcinematicscriptcontext_deletecinematicshot_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_026463_camcinematicscriptcontext_isscriptcinematicshotactive_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026601_cambasedirector_stopshaking_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026607_cambasedirector_shake_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_026608_cambasedirector_shake_char_const_ptr_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_026609_cambasedirector_setshakeamplitude_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_026620_camscriptdirector_iscamerascriptcontrollable_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026687_camgameplaydirector_setscriptoverriddenfollowpedcamerasettingsth(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026688_camgameplaydirector_scriptrequestvehiclecamstuntsettingsthisupda(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026689_camgameplaydirector_scriptrequestdedicatedstuntcamerathisupdate(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026690_camgameplaydirector_islookingbehind_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026700_camgameplaydirector_getactiveviewmodecontext_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026708_camgameplaydirector_stophinting_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_026711_camgameplaydirector_getrelativecameraheading_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026712_camgameplaydirector_getrelativecamerapitch_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026713_camgameplaydirector_getfollowpedcamera_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026714_camgameplaydirector_getfirstpersonpedaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026715_camgameplaydirector_isfirstpersonshooteraiming_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026720_camgameplaydirector_setscriptoverriddentablegamescamerasettingst(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026721_camgameplaydirector_scriptforcevehiclecamstuntsettingsthisupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026727_camgameplaydirector_starthinting_rage_vector3_const_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_026728_camgameplaydirector_disablefirstpersonthisupdate_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_026794_camthirdpersoncamera_bypassbuoyancythisupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_027059_camcontrolhelper_setviewmodeforcontext_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_027064_camcontrolhelper_overridezoomfactorlimits_float_float(ph: *mut Phantom, a0: f32, a1: f32); }
extern "C" { pub fn f_027189_camreplaydirector_getmaxdistanceallowedfromplayer_bool_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_027237_cambasesplinecamera_getcurrentnode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_027272_camscriptdirector_setcameraanimationphase_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_027273_camscriptdirector_getcameraanimationphase_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_027280_cviewportmanager_setwidescreenborders_bool_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_027282_camscriptdirector_debugsetscriptenabledshakename_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027283_camscriptdirector_debugsetscriptdisabledshake(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_028241_cammanager_debugregisterscriptcommand_char_const_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029101_cdebug_setversionnumber(ph: *mut Phantom); }
extern "C" { pub fn f_029340_cgamesessionstatemachine_changelevel_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_029346_cutscenemanager_registergameentity_cdynamicentity_ptr_rage_atnon(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_029347_cutscenemanager_getanimatedmodelentityfromscenehandle_rage_atnon(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_029350_cutscenemanager_commondebugstr_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029377_ctask_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029382_ccutsceneanimatedactorentity_storeactorvariationdata_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_029433_getorcreategroup_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029455_cdebugscene_tunewidgetdata_tunewidgetdata_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_029456_cdebugscene_queuetunewidget_cdebugscene_tunewidgetdata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029612_worldprobe_cshapetestdesc_setresultsstructure_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029613_worldprobe_cshapetestresults_freeintersections(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029639_cutscenemanager_cansetcutsceneexitstateforentity_rage_atnonfinal(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_029640_cutscenemanager_cansetcutsceneexitstateforcamera_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_029641_cutscenemanager_cansetcutsceneenterstateforentity_rage_atnonfina(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_029642_cutscenemanager_setseamlesstriggerorigin_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029650_cutscenemanager_terminateloadedonlyscene(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029675_cutscenemanager_triggercutsceneskip_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_029676_cutscenemanager_startmultiheadfade_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_029685_cutscenemanager_playseamlesscutscene_rage_scrthreadid_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_029687_cutscenemanager_overrideconcatsectionposition_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029688_cutscenemanager_overrideconcatsectionheading_float_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029689_cutscenemanager_requestcutscene_char_const_ptr_bool_rage_cutsman(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_029751_void_creplaymgrinternal_recordpersistantfx_cpacketstopscriptptfx(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029752_void_creplaymgrinternal_stoptrackingfx_unsigned_long_ctrackedeve(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029754_void_creplaymgrinternal_recordpersistantfx_cpacketstartscriptptf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_029756_bool_creplaymgrinternal_recordfx_cpackettriggeredscriptptfx_cpac(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029866_cdebugscene_focusentities_get_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029868_cdebug_setcurrentmissiontag_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029876_debuglocation_beammeup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029967_cphysical_gethealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029994_cframedump_startcapture_char_const_ptr_bool_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: f32, a9: i64); }
extern "C" { pub fn f_029995_cframedump_stopcapture(ph: *mut Phantom); }
extern "C" { pub fn f_030737_rage_fweventqueue_32_void_remove_rage_fwevent_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030741_geteventglobalgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030770_cshockingeventsmanager_add_ceventshocking_ref_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_030771_ceventshocking_initclone_ceventshocking_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_030773_cshockingeventsmanager_clearallevents_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_030775_cshockingeventsmanager_clearallblockingareas(ph: *mut Phantom); }
extern "C" { pub fn f_030776_ceventdecisionmakermodifiablecomponent_clearresponse_eeventtype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030777_ceventdecisionmakermodifiablecomponent_unblockevent_eeventtype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030778_ceventdecisionmaker_blockevent_eeventtype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030782_ceventdecisionmakermanager_getisdecisionmakervalid_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_030783_ceventdecisionmakermanager_getdecisionmaker_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_030789_ceventdataresponse_decision_postloadcb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_031235_ceventshocking_ceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031242_ceventshocking_set_rage_vec3v_const_ref_centity_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_033790_gfxvalue_objectinterface_objectrelease_gfxvalue_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034137_gfxvalue_objectinterface_objectaddref_gfxvalue_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034946_cloadingscreens_areactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034949_chudtools_getwidescreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034950_chudtools_getaspectratio_bool(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_035019_chudcolour_setintcolour_ehud_colours_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_035026_cnewhud_activatehudcomponent_bool_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_035031_cnewhud_handlecharacterchange(ph: *mut Phantom); }
extern "C" { pub fn f_035033_cnewhud_updatedisplayconfig_int_escaleform_base_class(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_035036_cnewhud_removehudcomponentfromactionscript_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035038_cnewhud_ishudcomponentactive_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035040_chudcolour_getrgb_ehud_colours_unsigned_char(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_035042_cnewhud_sethudcomponenttobeshown_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035043_cnewhud_ishelptexthiddenbyscriptthisframe_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035044_cnewhud_sethudcomponenttobehidden_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035045_cnewhud_doesscriptwanttoforcehidethisframe_int_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035046_cnewhud_settoggleabilitybar_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035054_cminimap_getminimapped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_035058_chudtools_beginhudscaleformmethod_enewhudcomponents_char_const_p(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_035062_cminimap_getisinsideinterior_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035065_cnewhud_sethealthoverride_int_int_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_035068_cnewhud_usefakempcash_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035069_cnewhud_changefakempcash_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_035073_cnewhud_blinkabilitybar_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035074_cnewhud_setabilityoverride_float_float(ph: *mut Phantom, a0: f32, a1: f32); }
extern "C" { pub fn f_035077_cnewhud_sethudcomponentposition_int_rage_vector2(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_035078_cnewhud_resethudcomponentpositionfromoriginalxmlvalues_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035079_chudcolour_getrgba_ehud_colours(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035080_chudtools_getsafezonesize(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_035106_chudtools_gethudposfromworldpos_rage_vector3_const_ref_etext_arr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_035110_chudcolour_setrgbavalue_ehud_colours_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_035150_cmousepointer_setmousecursorthisframe(ph: *mut Phantom); }
extern "C" { pub fn f_035151_cmousepointer_setmousecursorvisible_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035152_cmousepointer_setmousecursorstyle_emouse_cursor_style(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035161_cmultiplayerchat_overridecolor_bool_ehud_colours(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_035162_cmultiplayerchat_forceaborttextchat(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035183_cmultiplayergamertaghud_ut_createfakeplayertag_cphysical_ptr_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_035185_cmultiplayergamertaghud_ut_setgamertagvisibility_int_emp_tag_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_035186_cmultiplayergamertaghud_ut_setgamertagvaluehudalpha_int_emp_tag(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_035188_cmultiplayergamertaghud_ut_removeplayertag_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_035190_cmultiplayergamertaghud_ut_setgamertagsshouldusevehiclehealth_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_035191_cmultiplayergamertaghud_ut_setgamertagsshouldusepointhealth_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_035192_cmultiplayergamertaghud_ut_setgamertagvalueint_int_emp_tag_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_035193_cmultiplayergamertaghud_ut_setbigtext_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_035194_cmultiplayergamertaghud_ismovieactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035195_cmultiplayergamertaghud_ut_createplayertag_int_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_035196_cmultiplayergamertaghud_ut_isgamertagactive_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035197_cmultiplayergamertaghud_ut_isgamertagfree_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035201_cmultiplayergamertaghud_ut_isreinitinggamernameandcrewtag_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_035202_cmultiplayergamertaghud_ut_setgamername_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_035205_cmultiplayergamertaghud_ut_setallgamertagsvisibility_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_035206_cmultiplayergamertaghud_ut_setgamertagspointhealth_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_035207_cmultiplayergamertaghud_ut_setgamertagvaluehudcolour_int_emp_tag(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_035208_cmultiplayergamertaghud_ut_setgamertaghealthbarcolour_int_ehud_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_035253_centitypoolindexforblip_centitypoolindexforblip_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_035262_cminimap_togglecomponent_int_bool_bool_ehud_colours(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_035264_cminimap_getblipfromactualid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_035269_cminimap_getbliptypevalue_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035270_cminimap_getblipdisplayvalue_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035271_cminimap_getblipnamevalue_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035274_cminimap_getuniqueblipused_cminimapblip_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035276_cminimap_createsonarblip_rage_vector3_const_ref_float_ehud_colou(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_035279_cminimap_getcentreposition_rage_vector3_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035292_cminimap_common_getfogofwardiscoveryratio(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_035293_cminimap_common_iscoordinaterevealed_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035294_cminimap_setminimapmodestate_esetup_state_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_035295_cminimap_showyoke_bool_float_float_int(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: i32); }
extern "C" { pub fn f_035304_cminimap_switchoffwaypoint_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035305_cminimap_isflagset_cminimapblip_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_035306_cminimap_addconecolorforblip_int_ehud_colours(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_035307_cminimap_getblippositionvalue_cminimapblip_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035308_cminimap_findblipentity_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035309_cminimap_common_getcolourfromblipsettings_eblip_colours_bool_ehu(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_035310_cminimap_getblipcolourvalue_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035312_cminimap_getblipalphavalue_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035313_cminimap_addfakeconedata_int_float_float_float_float_float_float(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32, a8: i32); }
extern "C" { pub fn f_035314_cminimap_destroyfakeconedata_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035315_cminimap_getblipobjectnameid_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035316_cminimap_getblipdirectionvalue_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_035317_cminimap_setblipparameter_eblip_params_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_035318_cminimap_clearallbliproutes(ph: *mut Phantom); }
extern "C" { pub fn f_035319_cminimap_setblipparameter_eblip_params_int_rage_color32(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_035320_cminimap_setblipparameter_eblip_params_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_035321_cminimap_setblipparameter_eblip_params_int_rage_vector3_const_re(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_035322_cminimap_setblipparameter_eblip_params_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_035323_cminimap_isblipidinuse_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035324_cminimap_getblip_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_035325_cminimap_isbliponstage_cminimapblip_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035326_cminimap_setvisible_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035328_cminimap_getblipattachedtoentity_centity_const_ptr_eblip_type_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_035329_cminimap_showsonarsweep_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035330_cminimap_setinprologue_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035331_cminimap_setonislandmap_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035332_cminimap_flashminimapdisplay_ehud_colours(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035339_cminimap_getactualnorthblipid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035340_cminimap_getactualcentreblipid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035345_cminimap_updatewaypoint(ph: *mut Phantom); }
extern "C" { pub fn f_035346_cminimap_getactivewaypointid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035349_cminimap_setcurrentgolfmap_egolf_course_holes(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035350_cminimap_revealcoordinate_rage_vector2_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035354_cminimap_switchonwaypoint_float_float_unsigned_short_bool_bool_e(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_035357_cminimap_isactivewaypoint_rage_vector2_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_035427_cminimapmenucomponent_resetblips(ph: *mut Phantom); }
extern "C" { pub fn f_035428_cminimapmenucomponent_setactive_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035431_cminimapmenucomponent_createblip_rage_vector3_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035432_cminimapmenucomponent_setwaypoint(ph: *mut Phantom); }
extern "C" { pub fn f_035449_chudmarkermanager_createmarker_shudmarkerstate_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_035452_chudmarkermanager_getmarkerstate_hudid_unsigned_short_unsigned_s(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_035453_chudmarkermanager_getmarkergroup_hudid_unsigned_char_unsigned_ch(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_035487_cbusyspinner_off_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035493_cbusyspinner_preload_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035495_cbusyspinner_registerinstructionalbuttonmovie_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035600_menuscreenid_getparsername_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035663_cplayerlistmenu_getcharacterslot_int_cplayerlistmenu_estatmode(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035781_cmapmenu_reloadmap(ph: *mut Phantom); }
extern "C" { pub fn f_035881_ccredits_init_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_035882_ccredits_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035900_cweaponwheel_clear_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035903_cweaponwheel_mapweaponslottoweaponwheel_int_rage_atnonfinalhashs(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_035914_cweaponwheel_setforcedbyscript_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035915_cweaponwheel_setspecialvehicleabilityfromscript(ph: *mut Phantom); }
extern "C" { pub fn f_035917_cweaponwheel_setweaponinmemory_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035918_cweaponwheel_getweaponinmemory_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036034_cscaleformmoviewrapper_isactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036038_cscaleformmgr_ismovieactive_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036044_cscaleformmgr_beginmethodinternal_int_escaleform_base_class_char(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_036045_cscaleformmgr_endmethodinternal_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036048_cscaleformmgr_beginmethod_int_escaleform_base_class_char_const_p(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_036049_cscaleformmgr_addparamfloat_double(ph: *mut Phantom, a0: f64); }
extern "C" { pub fn f_036050_cscaleformmgr_endmethod_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036051_cscaleformmgr_addparambool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036053_cscaleformmgr_addparamint_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036056_cscaleformmgr_addparamstring_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036060_cscaleformmgr_findmoviebyfilename_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036154_cpausemenu_close(ph: *mut Phantom); }
extern "C" { pub fn f_036160_cscaleformmgr_changemovieparams_int_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_036162_cscaleformmoviewrapper_param_addparam(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036171_suicontexts_activate_rage_athashvalue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036172_suicontexts_deactivate_rage_athashvalue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036173_cpausemenu_restart_unsigned_int_emenuscreen(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036175_cprofilesettings_set_cprofilesettings_profilesettingid_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036176_cpausemenu_redrawinstructionalbuttons_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036184_cpausemenu_togglepauserenderphases_bool_enum_pause_menu_pausedrp(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_036187_cpausemenu_togglefullscreenmap_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036198_cwarningmessage_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036201_cpausemenu_menuceptiongodeeper_menuscreenid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036202_cpausemenu_menuceptionthekick(ph: *mut Phantom); }
extern "C" { pub fn f_036215_cscaleformmoviewrapper_beginmethod_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_036217_cmenuscreen_getcontextmenu(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_036237_cuimenuped_setvisible_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036254_cpausemenu_isactive_pm_activeflags(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036263_cpausemenu_getactualscreen_menuscreenid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036265_cpausemenu_getcurrentscreen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036267_cpausemenu_iscurrentscreenvalid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036272_cpausemenu_lockmenucontrol_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036273_cpausemenu_resetpauserenderphases_enum_pause_menu_pausedrp_owner(ph: *mut Phantom); }
extern "C" { pub fn f_036276_cpausemenu_queueopen_unsigned_int_bool_emenuscreen_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_036281_cuireticule_reset(ph: *mut Phantom); }
extern "C" { pub fn f_036282_socialclubmenu_open_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036284_cuimenuped_clearped_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036290_cpausemenu_getstateforscript(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036292_suicontexts_isactive_rage_athashvalue(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036304_cpausemenu_getcurrenthighlightedtabdata(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_036305_cpausemenu_closedisplaycalibrationscreen(ph: *mut Phantom); }
extern "C" { pub fn f_036319_cpausemenu_triggerswitchpane_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036323_cpausemenu_unlockmenucontrol(ph: *mut Phantom); }
extern "C" { pub fn f_036335_cpausemenu_getposition(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036336_cpausemenu_getscreencodewantsscripttocontrol(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036337_cpausemenu_geteventoccurreduniqueid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036339_cpausemenu_setbusyspinner_bool_signed_char_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_036364_cprofilesettings_getint_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036367_cpausemenu_getcurrentselectedmissioncreatorblip(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036368_cpausemenu_ishoveringonmissioncreatorblip(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036377_cpausemenu_isinmapscreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036382_cprofilesettings_exists_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036386_cprofilesettings_write_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036389_cpausemenu_isrestarting(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036390_cpausemenu_entersocialclub(ph: *mut Phantom); }
extern "C" { pub fn f_036394_cpausemenu_setcurrentmissionactive_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036395_cpausemenu_setcurrentmissionlabel_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036396_cpausemenu_setcurrentmissiondescription_bool_char_const_ptr_char(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_036448_creportmenu_open_rage_rlgamerhandle_const_ref_creportmenu_emenut(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_036451_creportmenu_close(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036485_basesocialclubmenu_flagfordeletion(ph: *mut Phantom); }
extern "C" { pub fn f_036576_cuimenuped_setfadein_unsigned_int_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036577_cuimenuped_setpedmodel_unsigned_char_pm_columns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036617_ctextinputbox_close(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036629_cwarningscreen_removemessageoptionitems(ph: *mut Phantom); }
extern "C" { pub fn f_036630_cwarningscreen_setmessageandsubtext_ewarningmessagestyle_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64, a7: i32, a8: i32); }
extern "C" { pub fn f_036631_cwarningscreen_setmessagewithheaderandsubtext_ewarningmessagesty(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32, a9: i32); }
extern "C" { pub fn f_036633_cwarningscreen_setwarningmessageinuse(ph: *mut Phantom); }
extern "C" { pub fn f_036634_cwarningscreen_setmessageoptionitems_int_char_const_ptr_int_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_036635_cwarningscreen_setmessageoptionhighlight_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036636_cwarningscreen_getactivemessage(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036660_uiworldiconmanager_find_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_036661_uiworldiconmanager_add_centity_ptr_char_const_ptr_rage_scrthread(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_036662_uiworldiconmanager_add_rage_vector3_const_ref_char_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_036663_uiworldiconmanager_find_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_036667_cscaleformmgr_isreturnvalueset_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036668_cscaleformmgr_getreturnvalueint_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036671_cscaleformmgr_endmethodreturnvalue_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037247_cuireplayscaleformcontroller_setmovieid_unsigned_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037248_cscaleformmgr_beginmethod_int_escaleform_base_class_char_const_p(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_037290_cscaleformmgr_createmovie_cscaleformmgr_createmovieparams_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037309_cscaleformmgr_getreturnvaluebool_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037310_cscaleformmgr_getreturnvaluestring_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_037318_cscaleformmgr_callmethodfromscript_int_escaleform_base_class_cha(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: i64, a9: i64, a10: i64, a11: i64, a12: i64, a13: i32); }
extern "C" { pub fn f_037321_cscaleformmgr_addparamint64_long(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037327_cscaleformmgr_requestrendermovie3d_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037438_cfrontendstatsmgr_shouldusemetric(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037490_cgamestream_forcerenderon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037491_cgamestream_forcerenderoff(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037492_cgamestream_pause(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037493_cgamestream_resume(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037495_cgamestream_hide(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037496_cgamestream_show(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037497_cgamestream_autopostgametipon_cgamestream_egamestreamtiptype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037498_cgamestream_autopostgametipoff(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037499_cgamestream_setrenderenabledglobal_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037500_cgamestream_hidethisupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037501_cgamestream_flushqueue_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037502_cgamestream_removeitem_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037504_cgamestream_enablefeedtype_int_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037505_cgamestream_enableallfeedtypes_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037511_cgamestream_postticker_char_const_ptr_bool_bool_bool_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_037518_cgamestream_updatefeeditemtexture_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_037522_cgamestream_reportlogoison(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037523_cgamestream_reportlogoisoff(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037525_cgamestream_setimportantparamsrgba_int_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_037526_cgamestream_resetimportantparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037527_cgamestream_freezenextpost(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037528_cgamestream_clearfrozenpost(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037529_cgamestream_setnextpostbackgroundcolor_ehud_colours(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037530_cgamestream_isshowenabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037531_cgamestream_getlastshownphoneactivatablescriptid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037532_cgamestream_clearcache(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037582_cgolftrailinterface_script_settrailenabled_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037583_cgolftrailinterface_script_settrailpath_rage_vec3v_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_037584_cgolftrailinterface_script_settrailradius_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32); }
extern "C" { pub fn f_037585_cgolftrailinterface_script_settrailcolour_rage_color32_rage_colo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_037586_cgolftrailinterface_script_settrailtessellation_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037587_cgolftrailinterface_script_settrailfixedcontrolpointsenabled_boo(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037588_cgolftrailinterface_script_settrailfixedcontrolpoint_int_rage_ve(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_037589_cgolftrailinterface_script_settrailshaderparams_float_float_floa(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_037590_cgolftrailinterface_script_settrailfacing_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037591_cgolftrailinterface_script_gettrailmaxheight(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_037592_cgolftrailinterface_script_gettrailvisualcontrolpoint_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037652_ccheat_hascheatactivated_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_037653_ccheat_haspccheatactivated_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037669_datetime_addtime_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_037674_cclock_settimesample_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037676_cclock_settime_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_037679_cclock_addtime_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_037680_cclock_advancetimeto_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_037682_cclock_setdate_int_date_month_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_037795_cthepopmultiplierareas_createpopmultiplierarea_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_037796_cthepopmultiplierareas_createpopmultiplierarea_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_037797_cthepopmultiplierareas_removepopmultiplierarea_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: f32, a5: i32); }
extern "C" { pub fn f_037800_cthepopmultiplierareas_getactivearea_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_037891_cincidentmanager_addincident_cincident_const_ref_bool_int_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_037892_cincident_dtorcincident(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037896_cincidentmanager_getincident_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_037934_cweather_gettypeindex_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037944_cweather_stopoverriding(ph: *mut Phantom); }
extern "C" { pub fn f_037945_cweather_forcetypenow_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037947_cweather_forceweathertypeovertime_unsigned_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_037950_cweather_forcetype_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037951_cweather_forcetypeclear_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_037952_cweather_settypenow_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_037953_cweather_overridetype_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037954_cweather_randomisetype(ph: *mut Phantom); }
extern "C" { pub fn f_037963_cweather_forcelightningflash(ph: *mut Phantom); }
extern "C" { pub fn f_037965_cweather_fadeinwateroverrideovertime_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_037966_cweather_fadeoutwateroverrideovertime_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_037967_cweather_gettypeindex_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_037990_cmapareas_getareaindexfromposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038053_cwantedresponseoverrides_getinstance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_038054_cwantedresponseoverrides_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038055_cwantedresponseoverrides_setnumpedstospawn_dispatchtype_signed_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_038057_cdispatchservice_resetspawntimer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038119_cdispatchspawnproperties_getinstance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_038121_cdispatchspawnproperties_addangledblockingarea_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_038122_cdispatchspawnproperties_addsphereblockingarea_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_038123_cdispatchspawnproperties_removeblockingarea_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038124_cdispatchspawnproperties_resetblockingareas(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038135_cdispatchspawnhelper_findspawnpointindirection_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_038191_cdispatchmanager_enabledispatch_dispatchtype_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_038192_cdispatchmanager_blockdispatchresourcecreation_dispatchtype_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_038265_cincidentmanager_clearincident_cincident_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038329_cincident_setdirty(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038352_cscriptincident_cscriptincident_centity_ptr_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_039573_cpathservergta_updatedynamicobject_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_039630_cweapon_sfireparams_dtorsfireparams(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039639_cpathservergta_maybeadddynamicobject_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039644_cobject_getpointertoclosestmapobjectforscript_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_039688_cobjectpopulation_destroyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039719_cobject_settask_rage_aitask_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_039720_cobject_addtoorremovefromprocesspostcameraarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040355_cactionmanager_enableaction_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041056_findplayerped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041218_cplayerinfo_getphysicalplayerindex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041296_cpedfactory_destroyped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041377_cpedintelligence_setdecisionmakerid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041385_cpedintelligence_findtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041487_cpedpopulation_setscriptcampopulationoriginoverride(ph: *mut Phantom); }
extern "C" { pub fn f_041589_cexplosionmanager_cexplosionargs_dtorcexplosionargs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041705_cplayerinfo_setplayerstate_cplayerinfo_state(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041746_reportplayerresetmismatch_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_041834_cpopzones_ispointinpopzoneswithtext_rage_vector3_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041853_cpedailodmanager_setuseactivecamerafortimeslicingcentre(ph: *mut Phantom); }
extern "C" { pub fn f_042390_cpedvariationstream_requeststreampedfilesinternal_cped_ptr_cpedv(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042404_cpedvariationpack_getlocalcompdata_cped_const_ptr_epedvarcomp_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_042406_cpedvariationpack_getglobalcompdata_cped_const_ptr_epedvarcomp_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_042437_cpedvariationpack_gettexvar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042438_cpedvariationpack_getcompvar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042489_cpedpropsmgr_getpedpropidx_cped_const_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042607_cpedpropsmgr_getlocalpropdata_cped_const_ptr_eanchorpoints_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_042628_cpedpropsmgr_getglobalpropdata_cped_const_ptr_eanchorpoints_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_042631_cpedpropsmgr_getpedproptexidx_cped_const_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042799_cpickup_setuselightoverrideforglow(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042812_cpickup_setforcealphaanduseambientscale(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042817_worldprobe_debugfindgroundzfor3dcoord_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32, a5: i64, a6: i64) -> f32; }
extern "C" { pub fn f_042818_worldprobe_cshapetestmanager_debugsubmittest_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_042821_worldprobe_cshapetestmanager_aborttest_worldprobe_cshapetestresu(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042844_cwatertesthelper_getwaterheightatpositionincludingrivers_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_042848_worldprobe_cshapetestdesc_setexcludeentities_centity_const_ptr_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_042854_cpickup_setincludeprojectileflag_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042913_cpickupplacement_createblip(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043046_ccollisionhistory_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043047_ccollisionhistory_findcollisionrecord_centity_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_043085_worldprobe_cshapetestdesc_setexcludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043207_cphysics_setgravitylevel_cphysics_egravitylevel(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043233_worldprobe_cshapetestbounddesc_setboundfromentity_centity_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043234_worldprobe_cshapetestdesc_setincludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043335_worldprobe_debugfindgroundzforcoord_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_043489_cocclusion_gethizbuffer_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_043505_occlusionqueries_oqallocate(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043506_occlusionqueries_oqfree_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043507_occlusionqueries_oqsetboundingbox_unsigned_int_rage_vec3v_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_043508_occlusionqueries_oqgetqueryresult_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_043622_postfx_bulletimpactoverlay_init(ph: *mut Phantom); }
extern "C" { pub fn f_043623_postfx_screenblurfade_init(ph: *mut Phantom); }
extern "C" { pub fn f_043634_postfx_lockadaptivedofdistance_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043648_postfx_setflashparameters_float_float_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_043654_postfx_bulletimpactoverlay_registerbulletimpact_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_043655_postfx_screenblurfade_trigger_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_043677_animpostfxmanager_start_rage_atnonfinalhashstring_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_043707_animpostfxeventpollingmanager_addlistener_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_043709_animpostfxeventpollingmanager_removelistener_rage_atnonfinalhash(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043712_animpostfxmanager_cancelstartrequest_rage_atnonfinalhashstring_a(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043713_animpostfxmanager_isrunning_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_043714_animpostfxmanager_stop_rage_atnonfinalhashstring_animpostfxmanag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043716_animpostfxmanager_stopall_animpostfxmanager_eanimpostfxuser(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043718_animpostfxmanager_getcurrenttime_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_043721_animpostfxmanager_startcrossfade_rage_atnonfinalhashstring_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_043750_phonephotoeditor_disable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043751_phonephotoeditor_settext_char_const_ptr_char_const_ptr_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_043752_phonephotoeditor_setbordertexture_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043783_cpostscan_addtoalwaysprerenderlist_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043784_cpostscan_removefromalwaysprerenderlist_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043786_cpostscan_addalphaoverride_centity_ptr_int_rage_atstring_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_043788_cpostscan_getalphaoverride_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043790_cpostscan_removealphaoverride_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043791_cpostscan_addentitysortbias_centity_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_043799_crenderphasecascadeshadowsinterface_script_initsession(ph: *mut Phantom); }
extern "C" { pub fn f_043815_lights_addscenelight_clightsource_ref_clightentity_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043901_crendertargetmgr_setnamedrendertargetdelayload_rage_atfinalhashs(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_043904_crendertargetmgr_getnamedrendertargetfromuniqueid_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_043905_crendertargetmgr_registernamedrendertarget_char_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_043906_crendertargetmgr_releasenamedrendertarget_char_const_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_043909_crendertargetmgr_getnamedrendertargetid_char_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_043911_crendertargetmgr_linknamedrendertargets_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_043912_crendertargetmgr_isnamedrendertargetlinked_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_044258_ui3ddrawmanager_assignpedtoslot_rage_atnonfinalhashstring_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_044259_ui3ddrawmanager_assignpedtoslot_rage_atnonfinalhashstring_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: f32) -> i32; }
extern "C" { pub fn f_044260_ui3ddrawmanager_assigngloballightintensitytoslot_rage_atnonfinal(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_044261_ui3ddrawmanager_pushcurrentpreset_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_044547_lights_startpedlightfadeup_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_044548_lights_setlightoverridemaxintensityscale_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_044580_lightentitymgr_updatelightsforentity_rage_fwentity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044607_cparaboloidshadow_updatelight_clightsource_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
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
extern "C" { pub fn f_044677_crenderphasecascadeshadowsinterface_script_setshadowsampletype_c(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044678_crenderphasecascadeshadowsinterface_script_setaircraftmode_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044679_crenderphasecascadeshadowsinterface_script_setsplitzexpweight_fl(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_044680_crenderphasecascadeshadowsinterface_script_setdynamicdepthmode_b(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044681_crenderphasecascadeshadowsinterface_script_setboundposition_floa(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_044682_crenderphasecascadeshadowsinterface_script_setflycameramode_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044684_crenderphasecascadeshadowsinterface_setdynamicdepthvalue_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_044685_crenderphasecascadeshadowsinterface_setscreensizecheckenabled_bo(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044787_renderphaseseethrough_updatevisualdatasettings(ph: *mut Phantom); }
extern "C" { pub fn f_044957_crenderphasewaterreflectioninterface_setscriptobjectvisibility_b(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_044996_creplaycoordinator_shouldshowloadingscreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045006_creplaycoordinator_isexportingtovideofile(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045239_cphotomanager_getsavecameraphotostatus(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045240_cqueueofphotooperations_push_cphotooperationdata_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045241_cphotomanager_requestsavecameraphoto(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045244_cgenericgamestorage_queueautosave(ph: *mut Phantom); }
extern "C" { pub fn f_045246_cgenericgamestorage_queuemissionrepeatload(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045247_cgenericgamestorage_queuemissionrepeatsave_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045251_cgenericgamestorage_isstoragedevicebeingaccessed(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045255_cgenericgamestorage_findclosestsavehouse_bool_rage_vector3_ref_f(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_045265_csavegamedevices_isdevicevalid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045276_csavegamefrontend_sethighlightedmenuitem_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_045290_cphotobuffer_getexactsizeofjpegdata(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045291_cphotobuffer_getjpegbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045318_cphotomanager_getcreatesortedlistofphotosstatus_bool_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045321_cphotomanager_requestcreatesortedlistofphotos_bool_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045323_cphotomanager_getnumberofphotos_bool(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045331_cphotomanager_clearcreatesortedlistofphotosstatus(ph: *mut Phantom); }
extern "C" { pub fn f_045334_cphotomanager_requesttakecameraphoto(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045335_cphotomanager_gettakecameraphotostatus(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045338_cphotomanager_requesttakemissioncreatorphoto(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045339_cphotomanager_gettakemissioncreatorphotostatus(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045340_cphotomanager_requestloadmissioncreatorphoto_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_045341_cphotomanager_getloadmissioncreatorphotostatus_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045342_cphotomanager_requestfreememoryforhighqualitycameraphoto(ph: *mut Phantom); }
extern "C" { pub fn f_045343_cphotomanager_requestfreememoryformissioncreatorphoto(ph: *mut Phantom); }
extern "C" { pub fn f_045344_cphotomanager_requestcreatelowqualitycopyofcameraphoto_elowquali(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045345_cphotomanager_getcreatelowqualitycopyofcameraphotostatus_elowqua(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045346_cphotomanager_requestcreatemissioncreatorphotopreview_elowqualit(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045347_cphotomanager_getstatusofcreatemissioncreatorphotopreview_elowqu(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_045348_cphotomanager_requestfreememoryforlowqualityphoto(ph: *mut Phantom); }
extern "C" { pub fn f_045349_cphotomanager_requestfreememoryformissioncreatorphotopreview(ph: *mut Phantom); }
extern "C" { pub fn f_045350_cphotomanager_drawlowqualityphototophone_bool_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_045351_cphotomanager_isnameofmissioncreatortexture_char_const_ptr_char(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045354_cphotomanager_setcurrentphonephotoismugshot_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_045355_cphotomanager_setarenathemeandvariationforcurrentphoto_unsigned(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_045356_cphotomanager_setonislandxforcurrentphoto_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_045374_csavegamequeue_containsanautosave(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045437_svenode_readjsonvalue_rage_rsonreader_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045445_svedict_remove_rage_atfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045449_scloudfile_loadfile_void_const_ptr_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_045452_svearray_append_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045453_svearray_append_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045454_svearray_append_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_045455_svearray_append_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045456_svearray_append_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045673_cscriptsavedata_openscriptsavedata_etypeofsavegame_void_const_pt(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_045674_cscriptsavedata_getsizeofsavedscriptdata_etypeofsavegame_gtathre(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_045675_cscriptsavedata_addmembertocurrentstruct_char_const_ptr_int_void(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_045678_cscriptsavedata_openscriptstruct_char_const_ptr_void_ptr_int_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_045679_cscriptsavedata_closescriptstruct_gtathread_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045798_cped_startprimarymotiontask_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045844_cped_setdefaultdecisionmaker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045899_cped_ourpedcanseethisentity_centity_const_ptr_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_045977_cped_setdesiredheading_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046091_cped_debugattachpedtophysical_char_const_ptr_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i64, a8: i64, a9: f32, a10: f32); }
extern "C" { pub fn f_046098_cped_getbonematrixfromragdollcomponent_rage_matrix34_ref_int_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046128_cped_setisahighprioritytarget_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046302_cdynamicentity_createskeleton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046303_cdynamicentity_createanimdirector_rage_rmcdrawable_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_046305_cdynamicentity_addtosceneupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046327_cplayerswitchinterface_isactive_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_046403_cscriptdownloadabletexturemanager_isnameofatexture_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046410_centity_getinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046438_cgameworld_add_centity_ptr_rage_fwinteriorlocation_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046442_cinteriorproxy_getfromlocation_rage_fwinteriorlocation_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046445_cportaltracker_collectstack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046449_cportaltracker_movetonewlocation_rage_fwinteriorlocation(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046458_cdynamicentity_getthreadthatcreatedme_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046459_cdynamicentity_getscriptthatcreatedme_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046467_cgameworld_remove_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046474_centity_getshouldusescreendoor_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046484_centity_setisvisibleformodule_eisvisiblemodule_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046495_centity_isinpathserver_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046496_centity_wasunabletoaddtopathserver(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046499_centity_getcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046500_centity_getisstatic_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046502_centity_getphysarch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046514_centity_setmaxspeed_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046515_centity_changevisibilitymask_unsigned_short_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046531_centity_getisvisibleformodule_eisvisiblemodule_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046545_centity_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046673_cextracontentmanager_executecontentchangesetgroupinternal_cmount(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046677_cextracontentmanager_revertcontentchangesetgroupinternal_cmounta(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046685_cextracontentmanager_revertcontentchangesetinternal_cmountableco(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_046687_cextracontentmanager_getcontentbyhash_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046688_cextracontentmanager_executecontentchangesetinternal_cmountablec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_046704_cextracontentmanager_areanyccspending(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046712_cextracontentmanager_executecontentchangesetgroupforall_unsigned(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046714_cextracontentmanager_revertcontentchangesetgroupforall_unsigned(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046717_cextracontentmanager_getmapchangescrc_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046733_cextracontentmanager_iscontentitemlocked_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046752_cextracontentmanager_bank_enablecloudwidgetsoverride(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_046817_cextrametadatamgr_getshopvehicledata_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046818_cextrametadatamgr_getshopvehiclemodelnamehash_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046819_cextrametadatamgr_getgenericvehiclemodlist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046822_cextrametadatamgr_getnumdlcweaponssp_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046823_cextrametadatamgr_getindexforspweapons_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046824_cextrametadatamgr_getdlcweaponshopitem_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046825_cextrametadatamgr_getnumdlcweaponcomponents_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046826_cextrametadatamgr_getnumdlcweaponcomponentssp_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046833_cextrametadatamgr_gethashnameforcomponent_int_epedvarcomp_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_046834_cextrametadatamgr_gethashnameforprop_int_eanchorpoints_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_046836_cextrametadatamgr_getshoppedapparelvariantpropscount_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046838_cextrametadatamgr_getshoppedapparelvariantcomponentscount_unsign(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046839_cextrametadatamgr_getvariantprop_unsigned_int_unsigned_int_int_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_046840_cextrametadatamgr_getvariantcomponent_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_046841_cextrametadatamgr_getshoppedapparelforcedpropscount_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046842_cextrametadatamgr_getshoppedapparelforcedcomponentscount_unsigne(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046843_cextrametadatamgr_getforcedprop_unsigned_int_unsigned_int_int_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_046844_cextrametadatamgr_getforcedcomponent_unsigned_int_unsigned_int_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_046845_cextrametadatamgr_doesshoppedapparelhaverestrictiontag_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_046846_cextrametadatamgr_setupshoppedcomponentquery_escrcharacters_esho(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_046847_cextrametadatamgr_setupshoppedpropquery_escrcharacters_eshopenum(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_046848_cextrametadatamgr_setupshoppedoutfitquery_escrcharacters_eshopen(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_046849_cextrametadatamgr_getshoppedqueryoutfit_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046850_cextrametadatamgr_getshoppedquerycomponentindex_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046851_cextrametadatamgr_getshoppedquerypropindex_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046852_cextrametadatamgr_getshoppedoutfit_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046853_cextrametadatamgr_getshoppedapparelcharacter_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046902_cinteriorproxy_iscontainingimapactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046904_cextrametadatamgr_gettattooshopitem_etattoofaction_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_046905_cextrametadatamgr_getnumtattooshopitemsinfaction_etattoofaction(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046906_cextrametadatamgr_gettattooshopitemindex_etattoofaction_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_047005_cfileloader_markarchetypeswhichcannotbedummy(ph: *mut Phantom); }
extern "C" { pub fn f_047018_cinstancepriority_setmode_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047020_cgameworld_findlocalplayerwanted(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047044_cgameworld_forallentitiesintersecting_rage_fwsearchvolume_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_047045_cmapchangemgr_add_unsigned_int_unsigned_int_rage_spdsphere_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_047046_cmapchangemgr_remove_unsigned_int_unsigned_int_rage_spdsphere_co(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047057_cphysical_removeblip_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047060_cphysical_activatephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047073_cphysical_sethealth_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047076_cphysical_getweapondamagedtimebyentity_centity_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_047079_cphysical_hasbeendamagedbyentity_centity_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_047082_cphysical_hasbeendamagedbyanyobject_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047095_cphysical_applyforce_rage_vector3_const_ref_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32); }
extern "C" { pub fn f_047096_cphysical_applyforcecg_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_047098_cphysical_applytorque_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_047100_cphysical_applyimpulse_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047101_cphysical_applyimpulsecg_rage_vector3_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047104_cphysical_applyexternalforce_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: f32, a6: f32); }
extern "C" { pub fn f_047105_cphysical_applyexternalimpulse_rage_vector3_const_ref_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: f32); }
extern "C" { pub fn f_047118_cphysical_debugattachtophysicalusingphysics_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64, a10: i64, a11: f32, a12: i32, a13: f32, a14: f32) -> i32; }
extern "C" { pub fn f_047120_cphysical_debugattachtoworldusingphysics_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: f32, a9: i32); }
extern "C" { pub fn f_047126_cphysical_debugprocessallattachments_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_047133_cphysical_setdontloadcollision_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047175_cscenememorycheck_setstate_cscenememorycheck_estate(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047305_cgameworld_getmainplayerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047345_cinteriorinst_createlocation_cinteriorinst_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_047413_clodmgr_overrideloddist_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047707_cinteriorproxy_cleanupinteriorimmediately(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047715_cinteriorinst_getroomhashcode_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047718_cinteriorproxy_movetoretainlist_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047734_cinteriorinst_findroombyhashcode_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047748_cinteriorproxy_activateentityset_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047756_cinteriorproxy_getheading(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_047757_cinteriorproxy_getmatrix_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047760_cinteriorproxy_findentitysetbyname_rage_atnonfinalhashstring_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_047761_cinteriorproxy_deactivateentityset_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047763_cinteriorproxy_setisdisabled_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047764_cinteriorproxy_setiscappedatpartial_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047765_cinteriorproxy_disablemetro_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047790_clayoutmanager_isactivegroupid_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_047791_clayoutmanager_getinteriorfromhandle_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_047793_clayoutmanager_getroomfromhandle_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_047794_clayoutmanager_getlayoutnode_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_047795_clayoutmanager_getgroupnode_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_047796_clayoutmanager_getrsrefnode_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_047797_clayoutmanager_unloadlayout(ph: *mut Phantom); }
extern "C" { pub fn f_047798_clayoutmanager_loadlayout_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047805_clayoutmanager_populateinteriorwithlayout_cinteriorinst_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047806_clayoutmanager_deletelayout(ph: *mut Phantom); }
extern "C" { pub fn f_047854_cportaltracker_removefrominterior(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047856_cportaltracker_probeforinterior_rage_vector3_const_ref_cinterior(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_047859_cportaltracker_triggerforceallactiveinteriorcapture(ph: *mut Phantom); }
extern "C" { pub fn f_047861_cportaltracker_getcurrroomhash(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047865_cportaltracker_clearinteriorstateforentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047880_cgameworld_findlocalplayercoors(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047961_cgameworld_overridefreezeflags_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_047973_cgameworld_clearexcitingstufffromarea_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_047975_cgameworld_clearcarsfromarea_rage_vector3_const_ref_float_bool_b(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_047977_cgameworld_clearpedsfromarea_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047979_cgameworld_clearobjectsfromarea_rage_vector3_const_ref_float_boo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_047985_cgameworld_clearcarsfromangledarea_rage_vector3_const_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_047987_cgameworld_clearcopsfromarea_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_048339_networkutils_isnetworkcloneormigrating_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_048981_camera_commands_commandsetcamdebugname_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_048983_camera_commands_commandaddcamsplinenode_int_rage_scrvector_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_048984_camera_commands_commandaddcamsplinenodeusingcameraframe_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_048985_camera_commands_commandaddcamsplinenodeusingcamera_int_int_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_048986_camera_commands_commandaddcamsplinenodeusinggameplayframe_int_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_048993_camera_commands_commandsetcamsplinenodeease_int_int_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32); }
extern "C" { pub fn f_048994_camera_commands_commandsetcamsplinenodevelocityscale_int_int_flo(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_048995_camera_commands_commandoverridecamsplinevelocity_int_int_float_f(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: f32); }
extern "C" { pub fn f_048996_camera_commands_commandoverridecamsplinemotionblur_int_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: f32); }
extern "C" { pub fn f_048997_camera_commands_commandsetcamsplinenodeextraflags_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_048999_camera_commands_commandsetcamactivewithinterp_int_int_int_int_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_049000_camera_commands_commandiscaminterpolating_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_049011_camera_commands_commandshakecam_int_char_const_ptr_float(ph: *mut Phantom, a0: i32, a1: i64, a2: f32); }
extern "C" { pub fn f_049012_camera_commands_commandanimatedshakecam_int_char_const_ptr_char(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_049014_camera_commands_commandsetcamshakeamplitude_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_049015_camera_commands_commandstopcamshaking_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_049020_camera_commands_commandtriggervehiclepartbrokencamerashake_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_049021_camera_commands_commandplaycamanim_int_char_const_ptr_char_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_049022_camera_commands_commandplaysynchronizedcamanim_int_int_char_cons(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_049023_camera_commands_commandiscamplayinganim_int_char_const_ptr_char(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_049046_camera_commands_commandsetgameplaycamrelativeheading_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_049050_camera_commands_commandforcecamerarelativeheadingandpitch_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32); }
extern "C" { pub fn f_049077_camera_commands_commanddisablegameaimcamthisupdate(ph: *mut Phantom); }
extern "C" { pub fn f_049095_camera_commands_commandsetgameplaycamfollowpedthisupdate_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_049112_camera_commands_commandcreatecinematicshot_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_049118_camera_commands_commandisinvehiclemobilephonecamerarendering(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_049127_camera_commands_commandsetgameplayentityhint_int_rage_scrvector(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_049147_camera_commands_commandsetinvehiclecamerastatethisupdate_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_049151_camera_commands_commandsetgameplaypedhint_int_rage_scrvector_con(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_049152_camera_commands_commandsetgameplayvehiclehint_int_rage_scrvector(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_049153_camera_commands_commandsetgameplayobjecthint_int_rage_scrvector(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_049473_cutscene_commands_commandnewsetcutsceneorigin_rage_scrvector_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_049474_cutscene_commands_commandnewsetcutsceneoriginandorientation_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_049475_cutscene_commands_commandrequestcutscene_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_049476_cutscene_commands_commandrequestcutscenewithplaybacklist_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_049477_cutscene_commands_commandunloadstreamedseamlesscutscene(ph: *mut Phantom); }
extern "C" { pub fn f_049478_cutscene_commands_commandrequestcutfile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_049479_cutscene_commands_commandhascutfileloaded_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_049480_cutscene_commands_commandremovecutfile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_049481_cutscene_commands_commandgetcutfileconcatcount_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_049482_cutscene_commands_commandplayseamlesscutscene_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_049483_cutscene_commands_commandplayseamlesscutsceneatcoord_rage_scrvec(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_049484_cutscene_commands_commandstopcutscene_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_049485_cutscene_commands_commandstopcutsceneanddontprogressanim(ph: *mut Phantom); }
extern "C" { pub fn f_049497_cutscene_commands_commandiscutsceneauthorized_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_049500_cutscene_commands_commanddoescutscenehandleexist_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_049501_cutscene_commands_commandregisterenitywithcutscene_int_char_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_049502_cphysical_const_ptr_cthescripts_getentitytoqueryfromguid_cphysic(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_049503_cutscene_commands_commandgetregisteredentityindexcreatedbycutsce(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_049504_cutscene_commands_commandsetvehiclemodelplayerwillexitthescenein(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_049505_cutscene_commands_commandsetseamlesscstriggerarea_rage_scrvector(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_049509_cutscene_commands_commandsetpadcanvibrateduringcutscene_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_049510_cutscene_commands_commandsetcutscenefadevalues_bool_bool_bool_bo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_049511_cutscene_commands_commandsetcutscenecanbeskippedinmutiplayer_boo(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_049512_cutscene_commands_commandsetcargeneratorscanbeactiveduringcutsce(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_049514_cutscene_commands_commandsetcutscenecanbeskipped_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_049516_cutscene_commands_commandsetcandisplayminimapduringcutscenethisu(ph: *mut Phantom); }
extern "C" { pub fn f_049517_cutscene_commands_commandsetcutscenepedvariation_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_049518_cutscene_commands_commandsetcutscenepedpropvariation_char_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_049519_cutscene_commands_commandsetcutscenepedvariationfromped_char_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_049520_cutscene_commands_commandsetcutsceneentitystreamingflags_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_049522_cutscene_commands_commandgetentityindexfromcutsceneentity_char_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_049606_datafile_commands_checkdatafileindex_int_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_049616_datafile_commands_commanddatafileselectactivefile_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_049617_datafile_commands_svefileobject_setfilefromfilerequest_sfilerequ(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_049618_datafile_commands_svefileobject_setfile_std_2_unique_ptr_svedict(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_049619_datafile_commands_commanddatafiledeletefile_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_049620_datafile_commands_svefileobject_clearfile(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_049621_datafile_commands_commanddatafilecreateugc_int_ref_int_char_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_049622_datafile_commands_readsvefileobject_datafile_commands_svefileobj(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_049623_datafile_commands_commanddatafilecreatemissionugc_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_049624_datafile_commands_commanddatafileupdateugc_char_const_ptr_int_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32) -> i32; }
extern "C" { pub fn f_049625_datafile_commands_commanddatafileupdatemissionugc_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_049626_datafile_commands_commanddatafileugcsetplayerdata_char_const_ptr(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_049627_datafile_commands_commanddatafileselectugcdata_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_049628_datafile_commands_commanddatafileselectugcstats_int_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_049629_datafile_commands_commanddatafileselectugcplayerdata_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_049630_datafile_commands_commanddatafileselectcreatorstats_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_049631_datafile_commands_loadofflineinternal_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_049636_datafile_commands_svefileobject_takefile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_049638_datafile_commands_loadfromcloudop_start_char_const_ptr_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_049640_datafile_commands_savetocloudop_start_char_const_ptr_svedict_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_049642_datafile_commands_savetocloudop_update(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_049649_void_datafile_commands_dictset_svebool_bool_svedict_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_049651_void_datafile_commands_dictset_sveint_int_svedict_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_049653_void_datafile_commands_dictset_svefloat_float_svedict_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_049655_void_datafile_commands_dictset_svestring_char_const_ptr_svedict(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_049657_void_datafile_commands_dictset_svevec3_rage_vec3v_const_ref_sved(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_049658_datafile_commands_dictcreatedict_svedict_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_049659_datafile_commands_dictcreatearray_svedict_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_049660_datafile_commands_dictgetbool_svedict_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_049661_datafile_commands_dictgetint_svedict_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_049662_datafile_commands_dictgetfloat_svedict_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_049664_char_const_ptr_datafile_commands_dictget_svestring_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_049665_datafile_commands_dictgetvector_svedict_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_049765_debug_commands_commandsaveinttonameddebugfile_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_049766_debug_commands_commandsavefloattonameddebugfile_float_char_const(ph: *mut Phantom, a0: f32, a1: i64, a2: i64); }
extern "C" { pub fn f_049767_debug_commands_commandsavenewlinetonameddebugfile_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_049768_debug_commands_commandsavestringtonameddebugfile_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_049869_decorator_commands_getbool_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_049870_decorator_commands_getfloat_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> f32; }
extern "C" { pub fn f_049871_decorator_commands_getint_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_049909_extrametadata_commands_commandgetshoppedquerycomponent_int_scrsh(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_049911_extrametadata_commands_commandgetshoppedcomponent_int_scrshopped(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_049912_extrametadata_commands_commandgetshoppedqueryprop_int_scrshopped(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_049914_extrametadata_commands_commandgetshoppedprop_int_scrshoppedprop(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_049931_extrametadata_commands_commanddoescurrentpedcomponenthaverestric(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_049932_extrametadata_commands_commanddoescurrentpedprophaverestrictiont(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_049937_extrametadata_commands_commandgetdlcweapondata_int_scrshopweapon(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_049939_extrametadata_commands_commandgetdlcweaponcomponentdata_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_050003_entity_commands_getentitygenericcliphelpertoquery_int_char_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_050004_entity_commands_findscriptedanimtask_int_bool_bool_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_050005_entity_commands_findsynchronizedscenetask_int_char_const_ptr_cha(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_050012_entity_commands_commandhasentityanimfinished_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_050013_ctaskscriptedanimation_findfirstinstanceofclip_rage_atnonfinalha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_050015_entity_commands_commandhasentitybeendamagedbyanyped_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_050016_entity_commands_commandhasentitybeendamagedbyanyvehicle_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_050017_entity_commands_commandhasentitybeendamagedbyentity_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_050018_entity_commands_commandhasentityclearlostoentity_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_050019_entity_commands_commandhasentityclearlostoentityadjustforcover_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_050020_entity_commands_commandhasentityclearlostoentityinfront_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_050022_entity_commands_commandgetlastmaterialhitbyentity_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_050024_entity_commands_commandgetentityanimcurrenttime_int_char_const_p(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_050025_entity_commands_commandgetentityanimtotaltime_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_050027_entity_commands_commandgetentityattachedto_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_050033_entity_commands_commandgetentityheadingfromeulers_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_050036_entity_commands_commandsetentitymaxhealth_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_050037_entity_commands_commandgetentityheight_int_rage_scrvector_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32) -> f32; }
extern "C" { pub fn f_050040_entity_commands_commandgetentitymodel_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_050044_entity_commands_commandgetentityquaternion_int_float_ref_float_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_050053_entity_commands_commandgetworldpositionofentitybone_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_050061_entity_commands_commandisentityatcoord_int_rage_scrvector_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_050062_entity_commands_commandisentityatentity_int_int_rage_scrvector_c(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_050067_entity_commands_commandisentityattachedtoentity_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_050068_entity_commands_commandisentitydead_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_050069_entity_commands_commandisentityinair_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_050070_entity_commands_commandisentityinangledarea_int_rage_scrvector_c(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_050071_entity_commands_commandisentityinarea_int_rage_scrvector_const_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_050073_entity_commands_commandisentityinwater_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_050077_entity_commands_commandisentityplayinganim_int_char_const_ptr_ch(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_050079_entity_commands_commandisentitytouchingentity_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_050080_entity_commands_commandisentitytouchingmodel_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_050085_entity_commands_commandisentityoccluded_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_050086_entity_commands_commandwouldentitybeoccluded_int_rage_scrvector(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_050088_entity_commands_commandapplyforcetoentity_int_int_rage_scrvector(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_050089_entity_commands_commandapplyforcetoentitycenterofmasshelper_cphy(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_050091_entity_commands_commandprocessentityattachments_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_050092_entity_commands_attachentitytoentity_cphysical_ptr_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32); }
extern "C" { pub fn f_050093_entity_commands_commandattachentitytoentity_int_int_int_rage_scr(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32); }
extern "C" { pub fn f_050094_entity_commands_commandattachentitybonetoentitybone_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_050095_entity_commands_commandattachentitybonetoentityboneyforward_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_050096_entity_commands_commandattachentitytoentityphysicallyoverrideinv(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i64, a7: f32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: f32, a14: f32); }
extern "C" { pub fn f_050098_entity_commands_commandgetentityboneindexbyname_int_char_const_p(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_050099_entity_commands_commandclearentitylastdamageentity_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_050100_entity_commands_commanddeleteentity_int_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_050101_entity_commands_commanddetachentity_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_050102_entity_commands_commandfreezeentityposition_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_050104_entity_commands_commandplayentityanim_int_char_const_ptr_char_co(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32, a6: i32, a7: f32, a8: i32) -> i32; }
extern "C" { pub fn f_050105_entity_commands_commandforceentityaianimupdate_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_050106_entity_commands_commandplaysynchronizedentityanim_int_int_char_c(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: f32) -> i32; }
extern "C" { pub fn f_050108_entity_commands_commandstopentityanim_int_char_const_ptr_char_co(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_050109_ctaskscriptedanimation_stopplayback_ctaskscriptedanimation_eplay(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_050110_ctaskscriptedanimation_cprioritydata_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_050111_entity_commands_commandstopsynchronizedentityanim_int_float_bool(ph: *mut Phantom, a0: i32, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_050113_entity_commands_commandhasanimeventfired_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_050114_entity_commands_commandfindanimeventphase_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_050115_cclipeventtags_ctagiteratorattributevalue_rage_crtag_rage_crprop(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_050116_entity_commands_commandsetentityanimcurrenttime_int_char_const_p(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_050117_entity_commands_commandsetentityanimspeed_int_char_const_ptr_cha(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_050118_entity_commands_commandsetentityasmissionentity_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_050119_entity_commands_setentityasnolongerneeded_cphysical_ptr_int_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_050121_entity_commands_commandsetentitycanbedamaged_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_050123_entity_commands_commandsetentitycanbedamagedbyrelgroup_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_050124_entity_commands_commandsetentitycanonlybedamagedbyscriptparticip(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_050126_entity_commands_setentitycollision_int_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_050130_entity_commands_commandsetentitycoordsext_int_rage_scrvector_con(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_050133_entity_commands_commandsetentitycoordsnooffset_int_rage_scrvecto(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_050134_entity_commands_commandsetentitydynamic_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_050135_entity_commands_commandsetentityheading_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_050136_entity_commands_commandsetentityhealth_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_050137_entity_commands_commandsetentityinvincible_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_050138_entity_commands_commandsetentityistargetpriority_int_bool_float(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_050143_entity_commands_commandsetentityonlydamagedbyplayer_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_050144_entity_commands_commandsetentityonlydamagedbyrelgroup_int_bool_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_050149_entity_commands_commandsetentityrotation_int_rage_scrvector_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_050150_entity_commands_commandsetentityvisible_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_050160_entity_commands_commandsetentityalphaentity_centity_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_050164_entity_commands_commandresetentityalphaentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_050165_entity_commands_commandresetpickupentityglow_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_050179_entity_commands_commandsetentitynocollisionentity_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_050183_entity_commands_commandgetnearestplayertoentityhelper_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_050188_entity_commands_commandgetentitybonerotation_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_050189_entity_commands_commandgetentityboneposition_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_050190_entity_commands_commandgetentityboneobjectrotation_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_050191_entity_commands_commandgetentitybonecount_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_050193_entity_commands_commandsetentitycanonlydamageentity_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_050195_entity_commands_commandsetallowmigratetospectator_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_050196_entity_commands_commandgetentityoftypeattachedtoentity_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_050397_event_commands_createshockingevent_eeventtype_char_const_ptr(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_050399_event_commands_commandaddshockingeventforentity_int_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_050401_event_commands_commandremoveshockingevent_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_050428_fire_commands_commandstartscriptfire_rage_scrvector_const_ref_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_050437_fire_commands_commandaddexplosion_rage_scrvector_const_ref_int_f(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32, a4: i32, a5: f32, a6: i32); }
extern "C" { pub fn f_050438_fire_commands_commandaddownedexplosion_int_rage_scrvector_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: f32, a4: i32, a5: i32, a6: f32); }
extern "C" { pub fn f_050439_fire_commands_commandaddexplosionwithuservfx_rage_scrvector_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32, a5: i32, a6: f32); }
extern "C" { pub fn f_050474_graphics_commands_gettexturefromstreamedtxd_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_050477_graphics_commands_commanddraw_texturedpoly_rage_scrvector_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64); }
extern "C" { pub fn f_050478_graphics_commands_commanddraw_texturedpoly3colours_rage_scrvecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32, a7: i64, a8: i32, a9: i64, a10: i64, a11: i64, a12: i64, a13: i64); }
extern "C" { pub fn f_050508_graphics_commands_commanddrawlightwithrangeex_rage_scrvector_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: f32, a5: f32, a6: f32); }
extern "C" { pub fn f_050510_graphics_commands_commanddrawspotlight_rage_scrvector_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: f32, a6: f32, a7: f32, a8: f32, a9: f32); }
extern "C" { pub fn f_050511_graphics_commands_commanddrawshadowedspotlight_rage_scrvector_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: f32, a6: f32, a7: f32, a8: f32, a9: f32, a10: i32); }
extern "C" { pub fn f_050516_graphics_commands_commanddrawmarkerex_int_rage_scrvector_const_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i64, a14: i64, a15: i32, a16: i32, a17: i32); }
extern "C" { pub fn f_050533_graphics_commands_commandrequeststreamedtxd_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_050534_graphics_commands_hasstreamedtxdloaded_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_050535_graphics_commands_commandmarkstreamedtxdasnolongerneeded_char_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_050590_graphics_commands_commanddrawscaleformmovieinternal_int_int_floa(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: f32, a4: f32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_050593_graphics_commands_commanddrawscaleformmoviefullscreenmasked_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_050594_graphics_commands_commanddrawscaleformmovie3d_int_rage_scrvector(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_050595_graphics_commands_commanddrawscaleformmovie3dsolid_int_rage_scrv(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_050603_graphics_commands_drawspritetex_rage_grctexture_ptr_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i64, a13: i64, a14: i32, a15: i32); }
extern "C" { pub fn f_050605_graphics_commands_commandoverridepedcrewlogotexture_int_char_con(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_050634_graphics_commands_commanddrawspritenamedrt_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32); }
extern "C" { pub fn f_050664_graphics_commands_commandtriggerptfxinternal_boneindex_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64, a6: f32, a7: i32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_050665_graphics_commands_commandtriggerptfxonentityboneinternal_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: f32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_050666_graphics_commands_commandtriggerptfxinternal_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32) -> i32; }
extern "C" { pub fn f_050675_bool_creplaymgrinternal_recordfx_cpackettriggeredscriptptfxcolou(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_050680_graphics_commands_commandstartptfx_char_const_ptr_rage_scrvector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_050681_graphics_commands_commandstartptfxonpedbone_char_const_ptr_int_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: f32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_050682_graphics_commands_startptfxonentityinternal_char_const_ptr_int_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32, a10: f32, a11: f32, a12: f32, a13: i32) -> i32; }
extern "C" { pub fn f_050687_graphics_commands_commandstopptfx_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_050688_graphics_commands_commandremoveptfx_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_050691_graphics_commands_commandforceptfxinvehicleinterior_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_050694_graphics_commands_commandupdateptfxcolour_int_float_float_float(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_050697_graphics_commands_commandupdateptfxfarclipdist_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_050698_graphics_commands_commandevolveptfx_int_char_const_ptr_float_boo(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_050729_graphics_commands_commandadddecal_int_rage_scrvector_const_ref_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: f32, a10: f32, a11: i32, a12: i32, a13: i32) -> i32; }
extern "C" { pub fn f_050735_graphics_commands_commandremovedecal_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_050741_bool_creplaymgrinternal_recordfx_cpacketdisabledecalrendering_cp(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_050745_graphics_commands_commandmovevehicledecals_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_050746_graphics_commands_commandaddvehiclecrewemblem_int_int_int_rage_s(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: f32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_050775_graphics_commands_commandsetcurrentplayertimecyclemodifier_char(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_050785_graphics_commands_commandrequestscaleformmoviecommon_char_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_050797_graphics_commands_commandsetscaleformmovieasnolongerneeded_int_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_050804_graphics_commands_commandbeginscaleformmoviemethodonfrontend_cha(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_050805_graphics_commands_commandbeginscaleformmoviemethodonfrontendhead(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_050821_graphics_commands_commandpasskeyboardinputtoscaleform_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_050829_graphics_commands_commanddrawtvchannel_float_float_float_float_f(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_050841_graphics_commands_commandui3dsceneassignpedtoslot_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_050859_graphics_commands_commandphonephotoeditorsetframetxd_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_051352_dlc_commands_commandisdlcpresent_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_051390_netloadcloudfiletask_dtornetloadcloudfiletask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_051413_rage_crtagiteratorpropertyattributevalue_rage_crpropertyattribut(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_051505_hud_commands_commandaddtextcomponentsubstringtime_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_051506_hud_commands_commandaddtextcomponentformattedinteger_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_051531_hud_commands_getstringfromstringinternal_char_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_051538_int_rage_formatf_n_char_32ul_char_ref_32ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_051566_hud_commands_commandregisternamedrendertarget_char_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_051568_hud_commands_commmandisnamedrendertargetlinked_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_051570_hud_commands_commandlinknamedrendertarget_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_051583_hud_commands_commandishelpmessagebeingdisplayed(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_051593_hud_commands_getclosestblipinfoid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_051594_hud_commands_getnextblipinfoid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_051599_hud_commands_getblipfromentity_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_051600_hud_commands_getblipinfoidentityindex_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_051602_hud_commands_addblipforarea_rage_scrvector_const_ref_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> i32; }
extern "C" { pub fn f_051603_hud_commands_addblipforentity_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_051605_hud_commands_addblipforpickup_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_051610_hud_commands_commanddisplayradar_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_051618_hud_commands_commandsetradarzoomtoblip_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_051625_hud_commands_changeblipalpha_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_051667_hud_commands_commandremoveblip_int_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_051772_hud_commands_commandsetbigmapactive_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_051774_hud_commands_commandsetmissionname_bool_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_051794_hud_commands_commandcreatefakempgamertagwithcrewcolor_int_char_c(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_051816_hud_commands_commandsetwarningmessagewithheader_char_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32, a9: i32); }
extern "C" { pub fn f_051817_hud_commands_commandsetwarningmessagewithheaderextended_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32, a7: i64, a8: i64, a9: i32, a10: i32); }
extern "C" { pub fn f_051818_hud_commands_commandsetwarningmessagewithheaderandsubstringflags(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32, a7: i64, a8: i64, a9: i32, a10: i32); }
extern "C" { pub fn f_051819_hud_commands_commandsetwarningmessagewithheaderandsubstringflags(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64, a10: i32, a11: i32); }
extern "C" { pub fn f_051825_hud_commands_commandsetwarningmessage_char_const_ptr_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64, a7: i32, a8: i32); }
extern "C" { pub fn f_051829_hud_commands_commandactivatefrontendmenu_int_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_051836_hud_commands_commmandsetfrontendactive_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_051848_hud_commands_commandpausemenuiscontextmenuactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_051867_hud_commands_getmenupedu64stat_int_unsigned_long_ref_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_051870_hud_commands_commandgetmenupedintstat_int_int_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_051871_hud_commands_commandgetcharactermenupedintstat_int_int_ref_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_051872_hud_commands_commandgetmenupedfloatstat_int_float_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_051873_hud_commands_commandgetcharactermenupedfloatstat_int_float_ref_i(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_051874_hud_commands_commandgetmenupedboolstat_int_int_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_051878_hud_commands_commandgivepedtopausemenu_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_051882_hud_commands_commandareonlinepoliciesuptodate(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_052588_interior_commands_setroomforgameviewportbykey_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_052595_interior_commands_commandclearroomforentity_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_052597_interior_commands_commandforceroomforentity_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_052598_interior_commands_commandforceroomforgameviewport_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_052600_interior_commands_commandgetinteriorfromentity_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_052604_interior_commands_commandgetroomkeyfromentity_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_052609_interior_commands_commandgetinterioratcoordsoftype_rage_scrvecto(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_052781_misc_commands_commandsetmissionflag_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_052784_misc_commands_commandsetrandomeventflag_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_052844_misc_commands_commandgetgroundzfor3dcoord_rage_scrvector_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_052845_misc_commands_commandgetgroundzandnormalfor3dcoord_rage_scrvecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_052846_misc_commands_commandgetgroundzexcludeobjectsfor3dcoord_rage_scr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_052858_misc_commands_commandpointareaoverlap_rage_scrvector_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64, a5: f32) -> i32; }
extern "C" { pub fn f_052863_misc_commands_commandisareaoccupied_rage_scrvector_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_052864_misc_commands_commandisareaoccupiedaccurate_rage_scrvector_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_052868_misc_commands_commandcleararea_rage_scrvector_const_ref_float_bo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_052869_misc_commands_commandcleararealeavevehiclehealth_rage_scrvector(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_052870_misc_commands_commandclearareaofvehicles_rage_scrvector_const_re(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_052871_misc_commands_commandclearangledareaofvehicles_rage_scrvector_co(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_052872_misc_commands_commandclearareaofobjects_rage_scrvector_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_052873_misc_commands_commandclearareaofpeds_rage_scrvector_const_ref_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_052874_misc_commands_commandclearareaofcops_rage_scrvector_const_ref_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_052889_misc_commands_commandfiresinglebulletignoreentitynew_rage_scrvec(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: f32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i32, a14: i32, a15: i32, a16: i32); }
extern "C" { pub fn f_052895_misc_commands_commandslerpnearquaternion_float_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: i64, a10: i64, a11: i64, a12: i64); }
extern "C" { pub fn f_052898_misc_commands_commandsetminigameinprogress_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_052910_misc_commands_commandgetcoordsofprojectiletypeinarea_rage_scrvec(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_052911_misc_commands_commandgetcoordsofprojectiletypeinangledarea_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_052914_misc_commands_commandisprojectiletypewithindistance_rage_scrvect(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_052915_misc_commands_commandgetprojectileofprojectiletypewithindistance(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_052960_misc_commands_commandcreateincident_int_rage_scrvector_const_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: f32, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_052961_misc_commands_commandcreateincidentwithentity_int_int_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_052967_misc_commands_addpopmultiplierareaovernetwork_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_052968_misc_commands_commandaddpopmultiplierarea_rage_scrvector_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_052969_misc_commands_commandaddpopmultipliersphere_rage_scrvector_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_052971_misc_commands_commandremovepopdensitymultiplierarea_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_052972_misc_commands_commandispopdensitymultiplierareanetworked_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_052974_misc_commands_commandenabletennismode_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_052981_misc_commands_commandsettennismovenetworksignalfloat_int_char_co(ph: *mut Phantom, a0: i32, a1: i64, a2: f32); }
extern "C" { pub fn f_052987_misc_commands_commandsetdispatchtimebetweenspawnattempts_int_flo(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_052988_misc_commands_commandsetdispatchtimebetweenspawnattemptsmultipli(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_052998_misc_commands_commanddisplayonscreenkeyboardwithlongerinitialstr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i32); }
extern "C" { pub fn f_054930_network_commands_commandnetworkplayergetname_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_059401_shapetest_commands_getphysicsflags_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_063739_gtathread_killallthreadswiththisname_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063740_gtathread_pauseallthreadsbarthisone_gtathread_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063741_gtathread_unpauseallthreads(ph: *mut Phantom); }
extern "C" { pub fn f_063754_rage_atmapmemory_int_chiddenobjects_hidden_object_struct_dealloc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063771_cgamescriptresource_incrementnumberofusedresources(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063772_cthescripts_getcurrentgtascripthandler(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063779_cthescripts_setcontentidofcurrentugcmission_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063785_cdoublebufferedintrotexts_initialise_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063794_cthescripts_registerentity_cphysical_ptr_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_063796_cthescripts_unregisterentity_cphysical_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063797_cthescripts_getcurrentgtascripthandlernetwork(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063799_cthescripts_getcurrentscriptname(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063800_cthescripts_getcurrentscriptnameandprogramcounter(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063801_cmissionreplaystats_clearreplaystatsstructure(ph: *mut Phantom); }
extern "C" { pub fn f_063802_cmissionreplaystats_beginreplaystatsstructure_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_063803_cmissionreplaystats_addstatvaluetoreplaystatsstructure_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_063804_cmissionreplaystats_endreplaystatsstructure(ph: *mut Phantom); }
extern "C" { pub fn f_063805_cmissionreplaystats_getnumberofmissionstats(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_063806_cmissionreplaystats_getstatvalueatindex_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_063807_chiddenobjects_displayinteriordetails_rage_fwinteriorlocation_re(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063809_rage_atmapentry_int_chiddenobjects_hidden_object_struct_const_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_063812_cthescripts_getextensiblebasetomodifyfromguid_int_char_const_ptr(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063813_cthescripts_getitemsettomodifyfromguid_int_char_const_ptr_unsign(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063815_cthescripts_findlocalplayerped_int_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063816_cthescripts_findnetworkplayerped_int_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063819_cthescripts_importgamerhandle_rage_rlgamerhandle_ptr_int_ref_int(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063823_cscripteulers_matrixfromeulers_rage_matrix34_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063824_cscripteulers_matrixtoeulers_rage_matrix34_const_ref_rage_eulera(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063825_cscripteulers_quaternionfromeulers_rage_quaternion_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063826_cscripteulers_quaterniontoeulers_rage_quaternion_ref_rage_eulera(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063845_cscriptpedaiblips_setpedhasaiblip_int_int_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_063846_cscriptpedaiblips_doespedhaveaiblip_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_063847_cscriptpedaiblips_setpedaiblipgangid_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_063848_cscriptpedaiblips_setpedhasconeaiblip_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_063849_cscriptpedaiblips_setpedhasforcedblip_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_063850_cscriptpedaiblips_setpedaiblipnoticerange_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_063851_cscriptpedaiblips_setpedaiblipsprite_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_063852_cscriptpedaiblips_getaiblippedblipidx_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_063853_cscriptpedaiblips_getaiblipvehicleblipidx_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_063854_cgamescripthandler_registerscriptresourceandgetref_rage_scriptre(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063857_cscriptareas_newhighlightimportantarea_float_float_float_float_f(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32); }
extern "C" { pub fn f_063862_cscriptareas_ispointinangledarea_rage_vector3_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_063899_cscriptpeds_setpedcoordinates_cped_ptr_float_float_float_int_boo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_063900_cscriptentities_clearspaceformissionentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063907_cracetimes_init_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_063908_cracetimes_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_063909_cracetimes_checkpointhit_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_063910_cracetimes_getplayertime_int_int_ref_int_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_063938_cscriptwidgettree_cscriptwidgetnode_findwidget_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_063939_cscriptwidgettree_deletewidget_unsigned_int_bool_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_063944_cscriptdebug_addwidget_rage_bkwidget_ptr_ewidgettype(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_063976_cscriptdebug_setcontentsoffadecommandtextwidget_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063980_cscriptdebug_drawdebugcube_float_float_float_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_063982_cscriptdebug_drawdebugsquare_float_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_063987_cdisplaytextformatting_getcharacterheightonscreen(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_063989_cscripthud_getscreenposfromworldcoord_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_064002_intro_script_rectangle_setcommonvalues_float_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_064003_chudalignment_calculatehudposition_rage_vector2(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064004_cdoublebuffereddraworigins_getnextfreeindex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064005_cdoublebuffereddraworigins_set_int_rage_vector3_ref_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_064007_cdoublebufferedintrorects_getnextfree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064012_cblipfades_getblipfadedirection_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064013_cblipfades_addblipfade_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_064019_cscripthud_setupscaleformmovie_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_064038_citemset_addentitytoset_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_064039_citemset_removeentityfromset_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064040_citemset_findinset_rage_fwextensiblebase_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_064041_citemset_clean(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064082_cscriptmetadatamanager_getbaseelementlocationfromblock_cbaseelem(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_064089_cscripttextconstruction_clear(ph: *mut Phantom); }
extern "C" { pub fn f_064090_cscripttextconstruction_beginprint_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064091_cscripttextconstruction_endprint_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064092_cscripttextconstruction_beginbusyspinneron_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064093_cscripttextconstruction_endbusyspinneron_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064094_cscripttextconstruction_beginoverridebuttontext_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064095_cscripttextconstruction_endoverridebuttontext_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064096_cscripttextconstruction_beginthefeedpost_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064097_cscripttextconstruction_endthefeedpoststats_char_const_ptr_int_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_064098_cscripttextconstruction_endthefeedpostmessagetext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64, a6: f32, a7: i64, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_064099_cscripttextconstruction_endthefeedpostticker_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_064100_cscripttextconstruction_endthefeedpostaward_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_064101_cscripttextconstruction_endthefeedpostcrewtag_bool_bool_char_con(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_064102_cscripttextconstruction_endthefeedpostunlock_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_064103_cscripttextconstruction_endthefeedpostcrewrankup_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_064104_cscripttextconstruction_endthefeedpostversus_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_064105_cscripttextconstruction_endthefeedpostreplay_cgamestream_efeedre(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_064106_cscripttextconstruction_endthefeedpostreplayinput_cgamestream_ef(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_064107_cscripttextconstruction_begindisplaytext_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064108_cscripttextconstruction_enddisplaytext_float_float_bool_int(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_064109_cscripttextconstruction_beginismessagedisplayed_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064110_cscripttextconstruction_endismessagedisplayed(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_064111_cscripttextconstruction_begingetscreenwidthofdisplaytext_char_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064112_cscripttextconstruction_endgetscreenwidthofdisplaytext_bool(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_064113_cscripttextconstruction_begingetnumberoflinesforstring_char_cons(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064114_cscripttextconstruction_endgetnumberoflinesforstring_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> i32; }
extern "C" { pub fn f_064115_cscripttextconstruction_begindisplayhelp_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064116_cscripttextconstruction_enddisplayhelp_int_bool_bool_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_064117_cscripttextconstruction_beginisthishelpmessagebeingdisplayed_cha(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064118_cscripttextconstruction_endisthishelpmessagebeingdisplayed_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_064119_cscripttextconstruction_beginscaleformstring_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064120_cscripttextconstruction_endscaleformstring_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064121_cscripttextconstruction_beginsetblipname_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064122_cscripttextconstruction_endsetblipname_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064123_cscripttextconstruction_beginadddirectlytopreviousbriefs_char_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064124_cscripttextconstruction_endadddirectlytopreviousbriefs_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064125_cscripttextconstruction_beginclearprint_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064126_cscripttextconstruction_endclearprint(ph: *mut Phantom); }
extern "C" { pub fn f_064127_cscripttextconstruction_addinteger_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064128_cscripttextconstruction_addfloat_float_int(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_064129_cscripttextconstruction_addsubstringtextlabel_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064130_cscripttextconstruction_addsubstringtextlabelhashkey_unsigned_in(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064131_cscripttextconstruction_addsubstringliteralstring_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064132_cscripttextconstruction_addsubstringblipname_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064133_cscripttextconstruction_setcolourofnexttextcomponent_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064173_cgamescripthandler_registerscriptresourceandgetid_rage_scriptres(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_064215_cgamescriptid_generatescriptidhash(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064229_cgamescripthandlermgr_setscriptassafetoruninmp_gtathread_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064241_cscriptstreamingresource_cscriptstreamingresource_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_064245_cscriptresource_ptfx_cscriptresource_ptfx_char_const_ptr_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: f32, a8: i32) -> i64; }
extern "C" { pub fn f_064258_cscriptresource_checkpoint_cscriptresource_checkpoint_int_rage_v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32) -> i64; }
extern "C" { pub fn f_064260_cscriptresource_texturedictionary_cscriptresource_texturediction(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_064285_cscriptresource_radarblip_cscriptresource_radarblip_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_064286_cscriptresource_radarblip_cscriptresource_radarblip_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: f32) -> i64; }
extern "C" { pub fn f_064294_cscriptresource_mlo_cscriptresource_mlo_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_064307_cscriptresource_itemset_cscriptresource_itemset_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_064317_cscriptresource_binkmovie_cscriptresource_binkmovie_char_const_p(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_064320_cscriptresource_moviemeshset_cscriptresource_moviemeshset_char_c(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_064425_cgamescriptresource_dtorcgamescriptresource_102634(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064523_csegrassbankstatic_scriptvehicleclearfromrag(ph: *mut Phantom); }
extern "C" { pub fn f_064542_ccustomshadereffectgrass_setscriptvehicleflatten_rage_spdaabb_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_064543_ccustomshadereffectgrass_setscriptvehicleflatten_rage_spdsphere(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_064736_csettingsmanager_csettingsmanager(ph: *mut Phantom); }
extern "C" { pub fn f_064737_csettingsmanager_getsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064762_cislandhopper_disableallpreemptedimaps(ph: *mut Phantom); }
extern "C" { pub fn f_064800_cstreaming_hasobjectloaded_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_065069_cstreamgraphextensionmanager_cstreamgraphextensionmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065071_cstreamgraph_dtorcstreamgraph(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065082_cstreamgraph_collectresultsforautocapture_rage_atarray_streaming(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065133_cstreamingrequestlist_finish(ph: *mut Phantom); }
extern "C" { pub fn f_065228_metricscapture_storescriptmemresult_debuglocationmetricslist_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065230_metricscapture_storestreamingstats_debuglocationmetricslist_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065231_metricscapture_storestreamingmodulememory_debuglocationmetricsli(ph: *mut Phantom); }
extern "C" { pub fn f_065233_metricscapture_storememdistributionresult_debuglocationmetricsli(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065235_metricscapture_storepedandvehiclestatresult_debuglocationmetrics(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065236_metricscapture_storeframespersecondresult_debuglocationmetricsli(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065237_metricscapture_storemsperframeresult_debuglocationmetricslist_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065240_metricscapture_storecputimingresults_debuglocationmetricslist_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065241_metricscapture_storethreadtimingresults_debuglocationmetricslist(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065243_metricscapture_storememorybucketusageresults_debuglocationmetric(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065244_metricscapture_storememheapresults_debuglocationmetricslist_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_065245_metricscapture_storedrawlistresults_debuglocationmetricslist_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065246_metricscapture_clearresults_debuglocationmetricslist_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065247_metricscapture_captureinstance_startcapture(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065248_metricscapture_captureinstance_startsampling_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_065269_metricscapture_saveresults_debuglocationmetricslist_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065271_metricscapture_metricszonesdisplay_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_065274_metricscapture_metricszonestart_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065275_metricscapture_metricszonestop(ph: *mut Phantom); }
extern "C" { pub fn f_065285_metricscapture_getscriptpeakaveragetime_rage_scrthreadid_float_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_065290_ccompaniondata_removeblip_cblipcomplex_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065293_ccompaniondata_getinstance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_065446_ccontrolmgr_getmainplayercontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065458_ccontrolmgr_updatevirtualkeyboard(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065467_ccontrolmgr_cancelvirtualkeyboard(ph: *mut Phantom); }
extern "C" { pub fn f_065470_ccontrolmgr_showvirtualkeyboard_rage_iovirtualkeyboard_params_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065471_ccontrolmgr_getvirtualkeyboardstate(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065591_enduserbenchmarks_resetbenchmarks(ph: *mut Phantom); }
extern "C" { pub fn f_065592_enduserbenchmarks_startbenchmark(ph: *mut Phantom); }
extern "C" { pub fn f_065593_enduserbenchmarks_stopbenchmark(ph: *mut Phantom); }
extern "C" { pub fn f_065594_enduserbenchmarks_outputbenchmarkresults(ph: *mut Phantom); }
extern "C" { pub fn f_065596_enduserbenchmarks_getwasbenchmarkoncommandline(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065601_enduserbenchmarks_getiterationcount(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065602_enduserbenchmarks_getsinglepassinfo(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065615_cfilemgr_openfile_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_065617_cfilemgr_setdir_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065621_cfilemgr_openfileforwriting_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065622_cfilemgr_openfileforappending_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065623_cfilemgr_hasasyncinstallfinished(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065624_cfilemgr_cleanupasyncinstall(ph: *mut Phantom); }
extern "C" { pub fn f_065875_ctasksynchronizedscene_ctasksynchronizedscene_short_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32, a5: f32, a6: i64, a7: i64, a8: f32, a9: i32, a10: i64) -> i64; }
extern "C" { pub fn f_065950_ctaskscriptedanimation_ctaskscriptedanimation_rage_atnonfinalhas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32, a6: f32, a7: i32, a8: i64, a9: f32, a10: i32, a11: i32, a12: i64, a13: i32, a14: i32) -> i64; }
extern "C" { pub fn f_066056_ctacticalanalysisnavmeshpoints_addpointfromscript_rage_vec3v_con(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066058_ctacticalanalysisnavmeshpoints_clearpointsfromscript(ph: *mut Phantom); }
extern "C" { pub fn f_067376_crandomeventmanager_seteventtypeenabled_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_068825_ctaskmotiontennis_ctaskmotiontennis_bool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068835_ctaskmotiontennis_playdiveanim_ctaskmotiontennis_edivedirection(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32, a5: i32); }
extern "C" { pub fn f_068838_ctaskmotiontennis_playtennisswinganim_char_const_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32); }
extern "C" { pub fn f_068839_ctaskmotiontennis_gettennisswinganimcomplete(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068840_ctaskmotiontennis_gettennisswinganimcanbeinterrupted(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068841_ctaskmotiontennis_gettennisswinganimswung(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072675_cscenariospawnhistory_clearhistory(ph: *mut Phantom); }
extern "C" { pub fn f_075345_cmovenetworkhelper_sendrequest_rage_fwmvrequestid_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075349_cmovenetworkhelper_getfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_075474_cmovenetworkhelper_setfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_075489_cgenericcliphelper_blendoutclip_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_077153_statid_char_getstatid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077331_statsinterface_getintstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077336_statsinterface_getstatdesc_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077363_statsinterface_getbstdate_int_ref_int_ref_int_ref_int_ref_int_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_077364_statsinterface_getposixdate_int_ref_int_ref_int_ref_int_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_077479_ctextfile_get_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077481_ctextfile_get_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_077486_csubtitlemessage_set_char_const_ptr_int_cnumberwithinmessage_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32); }
extern "C" { pub fn f_077488_cmessages_insertnumbersandsubstringsintostring_char_const_ptr_cn(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32); }
extern "C" { pub fn f_077495_chelpmessage_clearall(ph: *mut Phantom); }
extern "C" { pub fn f_077496_cmessages_clearmessages(ph: *mut Phantom); }
extern "C" { pub fn f_077497_cmessages_clearallbriefmessages(ph: *mut Phantom); }
extern "C" { pub fn f_077500_cmessages_clearalldisplayedmessagesthatbelongtothistextblock_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_077503_cmessages_clearthisprint_csubtitlemessage_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077505_cmessages_fillstringwithpreviousmessage_cmessages_epreviousmessa(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_077506_cmessages_ispreviousmessageempty_cmessages_epreviousmessagetypes(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077507_chelpmessage_setmessagetext_int_char_const_ptr_cnumberwithinmess(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_077508_chelpmessage_clear_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_077510_chelpmessage_setscreenposition_int_rage_vector2(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_077511_chelpmessage_setworldposition_int_rage_vector3_const_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_077512_chelpmessage_setworldpositionfromentity_int_cphysical_ptr_rage_v(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_077513_chelpmessage_setstyle_int_int_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_077515_chelpmessage_getmessagetext_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077516_chelpmessage_ismessagefadingout_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077520_chelpmessage_getdirectionhelptext_int_rage_vector2(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_077524_cloadingtext_clear(ph: *mut Phantom); }
extern "C" { pub fn f_077526_cloadingtext_settext_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077533_ctextlayout_ctextlayout(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077535_ctextlayout_dtorctextlayout(ph: *mut Phantom); }
extern "C" { pub fn f_077537_ctextlayout_setscale_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077539_ctextlayout_getcharacterheight(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_077565_ctextfile_doestextlabelexist_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077575_ctextfile_requestadditionaltext_char_const_ptr_int_ctextfile_elo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077577_ctextfile_isrequestingadditionaltext_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077578_ctextfile_hasthisadditionaltextloaded_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077580_ctextfile_getadditionaltextindex_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077581_ctextfile_hasadditionaltextloaded_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077624_ctimecycle_presetinteriorambientcache_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077625_ctimecycle_pushscriptmodifier(ph: *mut Phantom); }
extern "C" { pub fn f_077626_ctimecycle_popscriptmodifier(ph: *mut Phantom); }
extern "C" { pub fn f_077627_ctimecycle_settransitiontoscriptmodifierid_rage_atnonfinalhashst(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_077628_ctimecycle_settransitionoutofscriptmodifierid_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_077812_cpathfind_findnodeclosesttocoors_rage_vector3_const_ref_float_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: f32, a11: i32, a12: i64, a13: i32, a14: i32); }
extern "C" { pub fn f_079152_cvehicle_updategadgetsafterteleport_rage_matrix34_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_079619_cvehicle_resetafterteleport(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079841_cvehicle_isonfire_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079842_cvehicle_extinguishcarfire_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079904_cvehicle_placeonroadadjust_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_080401_cvehiclepopulation_setscriptcampopulationoriginoverride(ph: *mut Phantom); }
extern "C" { pub fn f_080428_cvehiclepopulation_forcepopulationinit(ph: *mut Phantom); }
extern "C" { pub fn f_080432_cvehiclepopulation_setcoordsofscriptcar_cvehicle_ptr_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_080866_cvfxhelper_getmatrixfromboneindex_rage_mat34v_ref_bool_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_080873_cvfxhelper_creatematfromvecz_rage_mat34v_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080884_cvfxhelper_getwaterz_rage_vec3v_const_ref_float_ref_rage_fwentit(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_080983_cdecalmanager_registermarker_int_int_rage_vec3v_const_ref_rage_v(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32); }
extern "C" { pub fn f_081026_cvisualeffects_overridenightvisionlightrange_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_081166_cdecalmanager_findrendersettinginfo_unsigned_int_int_ref_int_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_081182_cvfxregioninfomgr_setdisableregionvfx_bool_rage_scrthreadid(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_081204_cclouds_overridecloudsettingname_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081241_rage_cloudhatmanager_loadcloudhatbyname_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_081242_rage_cloudhatmanager_unloadcloudhatbyname_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_081243_rage_cloudhatmanager_preloadcloudhatbyname_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081254_rage_cloudhatmanager_unloadallcloudhats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081342_cdecalmanager_getrendersettingsindex_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_081349_cdecalmanager_addgeneric_decaltype_e_int_int_rage_vec3v_const_re(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: f32, a7: f32, a8: f32, a9: f32, a10: f32, a11: f32, a12: f32, a13: f32, a14: i32, a15: i32, a16: i32, a17: f32, a18: i32, a19: i32, a20: f32) -> i32; }
extern "C" { pub fn f_081354_cdecalmanager_addpool_vfxliquidtype_e_int_int_rage_vec3v_const_r(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i64, a5: f32, a6: f32, a7: f32, a8: i32, a9: i32, a10: i32) -> i32; }
extern "C" { pub fn f_081361_cdecalmanager_processvehiclebadge_cvehicle_const_ptr_cvehiclebad(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: f32, a7: i32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_081363_cvehiclebadgedesc_set_emblemdescriptor_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081365_cdecalmanager_removeallvehiclebadges_cvehicle_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081368_cdecalmanager_abortvehiclebadgerequests_cvehicle_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081372_cdecalmanager_addscriptedtrailinfo_rage_vec3v_const_ref_float_bo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_081373_cdecalmanager_startscriptedtrail_decaltype_e_vfxliquidtype_e_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_081375_cdecalmanager_endscriptedtrail(ph: *mut Phantom); }
extern "C" { pub fn f_081388_cdecalmanager_doesvehiclehavebadge_cvehicle_const_ptr_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_081392_cdecalmanager_remove_rage_fwentity_const_ptr_int_void_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_081393_cdecalmanager_removevehiclebadge_cvehicle_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081394_cdecalmanager_getvehiclebadgerequeststate_cvehicle_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_081395_cdecalmanager_setdisablecompositeshotgunimpacts_bool_rage_scrthr(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_081396_cdecalmanager_setdisablescuffs_bool_rage_scrthreadid(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_081397_cdecalmanager_applywheeltrailhackforarenamode_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081406_cptfxgpumanager_setuseoverridesettings_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081407_cptfxgpumanager_setoverridecurrlevel_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_081417_cmarkers_register_markerinfo_t_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081418_ccheckpoints_setinsidecylinderheightscale_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_081419_ccheckpoints_setinsidecylinderscale_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_081420_ccheckpoints_setcylinderheight_int_float_float_float(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_081421_ccheckpoints_setrgba_int_unsigned_char_unsigned_char_unsigned_ch(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_081422_ccheckpoints_setrgba2_int_unsigned_char_unsigned_char_unsigned_c(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_081423_ccheckpoints_setclipplane_int_rage_vec3v_const_ref_rage_vec3v_co(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_081424_ccheckpoints_setforceoldarrowpointing_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081425_ccheckpoints_setdecalrotalignedtocamrot_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081426_ccheckpoints_setforcedirection_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081427_ccheckpoints_setdirection_int_rage_vec3v_const_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_081460_cfiremanager_setscriptflammabilitymultiplier_float_rage_scrthrea(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_081462_cfiremanager_extinguisharea_rage_vec3v_const_ref_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_081464_cfiremanager_extinguishentityfires_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081467_cfiremanager_getnumfiresinrange_rage_vec3v_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_081468_cfiremanager_getclosestfirepos_rage_vec3v_ref_rage_vec3v_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_081469_cfiremanager_startscriptfire_rage_vec3v_const_ref_centity_ptr_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_081473_gameglows_addfullscreengameglow_rage_vec3v_const_ref_float_rage(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_081494_cmoviemgr_stop_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081499_cmoviemgr_play_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081505_cmoviemgr_setfrontendaudio_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_081506_cmoviemgr_attachaudiotoentity_unsigned_int_centity_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_081507_cmoviemgr_gettime_unsigned_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_081508_cmoviemgr_settime_unsigned_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_081510_cmoviemgr_setvolume_unsigned_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_081511_cmoviemgr_setshouldskip_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_081515_cmoviemgr_gethandlefromscriptslot_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_081519_cvfxlens_register_vfxlenstype_float_float_float_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_081529_cmoviemeshmanager_querysetstatus_unsigned_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_081532_cmoviemeshmanager_gethandlefromscriptid_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_081549_scriptim_line_rage_vec3v_const_ref_rage_vec3v_const_ref_rage_col(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_081550_scriptim_poly_rage_vec3v_const_ref_rage_vec3v_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_081551_scriptim_polytex_rage_vec3v_const_ref_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i64, a9: i64); }
extern "C" { pub fn f_081552_scriptim_boxaxisaligned_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_081553_scriptim_sprite3d_rage_vec3v_const_ref_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_081554_scriptim_setbackfaceculling_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081555_scriptim_setdepthwriting_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081577_ctvplaylistmanager_settvchannel_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081578_ctvplaylistmanager_settvchannelplaylist_int_char_const_ptr_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_081579_ctvplaylistmanager_settvchannelplaylistfromhour_int_char_const_p(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_081580_ctvplaylistmanager_cleartvchannelplaylist_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081581_ctvplaylistmanager_isplaylistonchannel_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_081582_ctvplaylistmanager_istvshowcurrentlyplaying_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_081583_ctvplaylistmanager_settvvolume_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_081584_ctvplaylistmanager_getcurrentshownamehash(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_081585_ctvplaylistmanager_setwatchingtvthisframe(ph: *mut Phantom); }
extern "C" { pub fn f_081628_clodlightmanager_registerbrokenlightsforentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081629_clodlightmanager_unregisterbrokenlightsforentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081635_terraingrid_setboxparams_rage_vector3_const_ref_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_081636_terraingrid_setcolours_rage_color32_rage_color32_rage_color32(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_081651_cptfxmanager_removeptfxfromentity_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081652_cptfxmanager_removeptfxinrange_rage_vec3v_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_081720_cptfxmanager_setforcevehicleinteriorflag_bool_rage_scrthreadid(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_081757_cvfxblood_setbloodptfxuserzoomscale_float_rage_scrthreadid(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_081758_cvfxblood_setoverrideweapongroupid_int_rage_scrthreadid(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_081781_cvfxentity_setinwaterptfxdisabled_bool_rage_scrthreadid(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_081843_cvfxmaterial_setbangscrapeptfxlodrangescale_float_rage_scrthread(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_081844_cvfxped_setfootvfxlodrangescale_float_rage_scrthreadid(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_081845_cvfxped_setfootptfxoverridehashname_rage_atnonfinalhashstring_ra(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081853_cvfxscript_getscriptptfxassetname_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081858_cvfxvehicle_setdownwashptfxdisabled_bool_rage_scrthreadid(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_081859_cvfxvehicle_setslipstreamptfxlodrangescale_float_rage_scrthreadi(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_081890_cvfxweapon_resetintsmokeoverrides(ph: *mut Phantom); }
extern "C" { pub fn f_081891_cvfxweapon_setbulletimpactptfxuserzoomscale_float_rage_scrthread(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_081892_cvfxweapon_setbulletimpactptfxlodrangescale_float_rage_scrthread(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_081893_cvfxweapon_setbulletimpactdecalrangescale_float_rage_scrthreadid(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_081894_cvfxweapon_setbullettracenoanglereject_bool_rage_scrthreadid(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_081905_cvfxwheel_setwheelskidmarkrangescale_float_rage_scrthreadid(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_082026_cweapon_cweapon_unsigned_int_int_cdynamicentity_ptr_bool_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_082027_cweapon_fire_cweapon_sfireparams_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082028_cexplosionmanager_addexplosion_cexplosionmanager_cexplosionargs(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082029_cweapon_dtorcweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082070_cweaponinfo_getammoinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082072_cbulletmanager_computeisbulletinangledarea_rage_vector3_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_082073_cbulletmanager_computeisbulletinsphere_rage_vector3_const_ref_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_082074_cbulletmanager_computeisbulletinbox_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_082075_cbulletmanager_computehasbulletimpactedinsphere_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_082076_cbulletmanager_computehasbulletimpactedinbox_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_082117_cexplosionmanager_getownerofexplosioninangledarea_eexplosiontag(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_082118_cexplosionmanager_getownerofexplosioninsphere_eexplosiontag_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_082119_cexplosionmanager_testforexplosionsinarea_eexplosiontag_rage_vec(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_082120_cexplosionmanager_testforexplosionsinangledarea_eexplosiontag_ra(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_082121_cexplosionmanager_testforexplosionsinsphere_eexplosiontag_rage_v(ph: *mut Phantom, a0: i32, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_082123_cweapondamage_generatepeddamageevent_centity_ptr_cped_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64, a5: i64, a6: i64, a7: i32, a8: f32, a9: i64, a10: i32, a11: f32, a12: i32) -> f32; }
extern "C" { pub fn f_082171_cweapon_buildviewmatrix_rage_vector3_const_ref_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082188_cpedequippedweapon_getprojectileordnance_cobject_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082522_cweaponinfo_getattachpoint_cweaponcomponentinfo_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082523_cweaponinfo_getisweaponcomponentdefault_rage_atnonfinalhashstrin(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082549_cweaponinfomanager_getslotlistnavigate_cped_const_ptr(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_082722_cprojectilemanager_removeallprojectilesinbounds_rage_spdaabb_con(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082729_cprojectilemanager_getisanyprojectileinbounds_rage_spdaabb_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082828_networkexitflow_startshutdowntasks(ph: *mut Phantom); }
extern "C" { pub fn f_082834_cnetwork_bail_sbailparameters_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082885_networkinterface_isanysessionactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082925_cnetwork_startweathertransitiontoglobal_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082932_cnetworkdebugtelemetry_appenddebugmetric_rage_rlmetric_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082948_cplayercardmanager_getdatamanager_cplayercardmanager_cardtypes_c(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_082958_cnetwork_setgostraighttomultiplayer_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082959_cnetwork_setgostraighttompevent_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082960_cnetwork_setgostraighttomprandomjob_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082961_cnetwork_setshutdownsessionclearsthegostraighttomultiplayerflag(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083008_networkinterface_iscloudavailable_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083014_networkinterface_getlocalgamerhandle(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083025_networkinterface_getphysicalplayerfromindex_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083028_networkinterface_isinspectatormode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083068_networkinterface_areplayersindifferenttutorialsessions_cnetgamep(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083087_cnetworkscriptworldstatemgr_revertworldstate_cnetworkworldstated(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083089_cnetworkscriptworldstatemgr_changeworldstate_cnetworkworldstated(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083092_cnetworkpopmultiplierareaworldstatedata_cnetworkpopmultiplierare(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32) -> i64; }
extern "C" { pub fn f_083093_networkinterface_findpopmultiplierareaovernetwork_cgamescriptid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083102_networkinterface_startptfxovernetwork_cgamescriptid_const_ref_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: f32, a6: i32, a7: i32, a8: i64, a9: i32, a10: f32, a11: f32, a12: f32, a13: i32, a14: i64); }
extern "C" { pub fn f_083104_networkinterface_stopptfxovernetwork_cgamescriptid_const_ref_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083136_networkinterface_isinsingleplayerprivatesession(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083152_networkinterface_hasbeendamagedby_rage_netobject_ptr_cphysical_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083158_networkinterface_getremoteplayercameramatrix_cped_ptr_rage_matri(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083159_networkinterface_getremoteplayercamerafov_cped_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_083173_networkinterface_canscriptmodifyremoteattachment_cphysical_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083332_clivemanager_checkusercontentprivileges_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083350_cloudlistener_cloudlistener(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083360_cloudfree_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083361_cloudallocate_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083496_ctunables_tryaccess_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083528_cusercontentmanager_create_char_const_ptr_char_const_ptr_rage_rl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_083529_cusercontentmanager_create_char_const_ptr_char_const_ptr_rage_rl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_083542_cusercontentmanager_checkcontentmodify_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083543_cusercontentmanager_updatecontent_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i64, a8: i32) -> i32; }
extern "C" { pub fn f_083547_cusercontentmanager_setplayerdata_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_083549_cusercontentmanager_canquerydata_int_bool_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083557_cusercontentmanager_getstringfromscriptdescription_cusercontentm(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083558_cusercontentmanager_iscreatepending(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083569_cusercontentmanager_getcontentdatasize_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083584_cusercontentmanager_getcontentstatssize_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083590_cusercontentmanager_getplayerdatasize_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083593_cusercontentmanager_ismodifypending(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083598_cusercontentmanager_getcreatorstatssize_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083599_cusercontentmanager_getabspathfromrelpath_char_const_ptr_char_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083603_cusercontentmanager_getofflinecontentpath_char_const_ptr_int_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_083768_cnetworkassetverifier_refreshcrc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083769_cnetworkassetverifier_setmemorytampercrc_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084087_clivemanager_showsigninui_unsigned_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_084339_cnetworktelemetry_cheatapplied_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084350_cnetworktelemetry_appendinfometric_unsigned_int_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_084362_cnetworktelemetry_appendpedattached_int_bool_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_084371_cnetworkdebugtelemetry_addautogputelemetry_metricslist_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084406_cnetworkdebugtelemetry_collectmetrictimebars(ph: *mut Phantom); }
extern "C" { pub fn f_084449_networkclan_getprimaryclan_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_084463_networkclan_getuiformattedclantag_bool_bool_char_const_ptr_int_r(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_084546_cplayercardmanager_extractpackedvalue_unsigned_long_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_084609_cplayercardmanager_getcorrectedmultistatforcurrentgamer_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_084610_cplayercardmanager_getpackedstatvalue_int_unsigned_int_int_ref_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_085697_policyversions_policyversions(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085702_policyversions_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085705_socialclubversionedfile_getacceptedversion_bool_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085708_policyversions_isdownloading_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085709_policyversions_isuptodate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085710_policyversions_geterror_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086744_cnetgameplayer_getplayerped_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086763_cnetobjgame_getplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086935_cnetobjentity_isvisibletoscript_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087023_cnetobjphysical_getentityattachedto_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087340_cnetobjphysical_getiscloneattached_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087341_cnetobjphysical_clearlastdamageobject(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087368_cnetobjphysical_setpendingscriptobjinfo_cgamescriptobjinfo_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087755_cnetobjvehicle_evictallpeds(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_089577_cmarkasnolongerneededevent_trigger_cnetobjgame_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_089587_cconverttoscriptentityevent_trigger_cnetobjphysical_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_089612_cclearareaevent_trigger_cgamescriptid_const_ref_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090710_asinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090717_atof(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090741_time(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090801_localtime(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090901_qsort_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_090911_rand(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090918_sinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090920_snprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090921_siprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090922_small_sprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090926_strcasecmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090927_strcat(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090928_strchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090930_strcmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090931_strcpy(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090939_strlen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090940_strncasecmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090942_strncmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090965_tanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
