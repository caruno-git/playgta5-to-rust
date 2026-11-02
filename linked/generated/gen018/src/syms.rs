//! Callee declarations for crate gen018.
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
extern "C" { pub fn f_000114_rage_sysipccreatesema_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_000116_rage_sysipcwaitsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000119_rage_sysipcsignalsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000135_rage_sysipcsleep_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000609_rage_voicechat_nativepackchat_int_void_ptr_unsigned_int_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000663_rage_atstring_replace_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000671_rage_atstring_split_rage_atarray_rage_atstring_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000675_rage_functorbase_functorbase_void_const_ptr_void_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_000680_rage_athashstringnamespacesupport_computehash_rage_athashstringn(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_000681_rage_athashstringnamespacesupport_addstring_rage_athashstringnam(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_000684_rage_athashstringnamespacesupport_trygetstring_rage_athashstring(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_000696_rage_bkwidget_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000703_rage_bkbank_unsetcurrentgroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000704_rage_bkbank_pushgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000706_rage_bkbank_deletegroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000707_rage_bkbank_popgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000758_rage_bkmanager_openfile_char_ptr_int_char_const_ptr_bool_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_000779_rage_bkmanager_createbank_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000782_rage_bkmanager_findbank_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000785_rage_bkmanager_destroybank_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000786_rage_bkmanager_isbankthread(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000806_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000866_rage_bkgroup_addseparator_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000867_rage_bkgroup_addtitle_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000868_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_000869_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000870_rage_bkgroup_addbutton_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000876_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_000877_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_000882_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000885_rage_bkgroup_addtoggle_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_000894_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000897_rage_bkgroup_addslider_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000898_rage_bkgroup_addslider_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000901_rage_bkgroup_addslider_char_const_ptr_unsigned_short_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_000905_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000907_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int_rage_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000918_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000919_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000926_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000942_rage_bkgroup_addcolor_char_const_ptr_rage_color32_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000943_rage_bkgroup_addcolor_char_const_ptr_float_ptr_int_rage_datcallb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000990_char_ptr_rage_formatf_char_512ul_char_ref_512ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001062_rage_matrix34_inverse_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001105_rage_autoidregistry_autoidregistry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001108_rage_autoidregistry_unregister_rage_autoiddescriptor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001116_char_ptr_rage_formatf_char_32ul_char_ref_32ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001117_rage_datcallback_datcallback(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001119_rage_datcallback_datcallback_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001123_rage_datgrowbuffer_datgrowbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001124_rage_datgrowbuffer_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001126_rage_datgrowbuffer_init_rage_sysmemallocator_ptr_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001143_rage_datbitbuffer_datbitbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001144_rage_datbitbuffer_setreadonlybits_void_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001145_rage_datbitbuffer_setreadwritebits_void_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001197_rage_sha1_sha1(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001205_rage_rsonwriter_init_char_ptr_unsigned_int_rage_rsonformat(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001216_rage_rsonwriter_writeint_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001218_rage_rsonwriter_writeint64_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001220_rage_rsonwriter_writeuns_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
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
extern "C" { pub fn f_001551_rage_fprintf_rage_fistream_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001559_rage_diagcontextmessage_diagcontextmessage_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001563_rage_diagcontextmessage_pop(ph: *mut Phantom); }
extern "C" { pub fn f_001568_rage_atmapentry_unsigned_int_wchar_t_ptr_const_ref_rage_atmap_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001582_rage_fiassetmanager_createleadingpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001627_rage_fiassetmanager_exists_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001652_rage_fidevice_makememoryfilename_char_ptr_int_void_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_001807_rage_fidevice_unmount_rage_fidevice_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001880_rage_fipackfile_fipackfile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001881_rage_fipackfile_init_char_const_ptr_bool_rage_fipackfile_cachemo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_001883_rage_fipackfile_mountas_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002027_rage_invsqrtf_fast_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_002030_rage_vec_v4float16streampack_float_vector_4_ptr_float_vector_4_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002031_rage_mthrandom_mthrandom_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002038_rage_pgstreamer_open_char_const_ptr_unsigned_int_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002041_rage_pgstreamer_read_unsigned_int_rage_datresourcechunk_ptr_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64) -> i64; }
extern "C" { pub fn f_002046_rage_pgrscbuilder_cleanup_rage_datresourcemap_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002063_rage_pgstreamer_read_unsigned_int_rage_datresourcechunk_ptr_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_002080_rage_pgbase_pgbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002088_rage_datresource_datresource_rage_datresourcemap_const_ref_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002090_rage_datresource_dtordatresource(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002092_rage_datresource_getfixup_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002111_rage_parmanager_parmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002112_rage_parmanager_initialize_rage_parsettings_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002113_rage_atbinarymap_char_const_ptr_unsigned_int_insert_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002128_rage_parmanager_uninitialize(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002131_rage_atnonfinalhashstring_atnonfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002132_rage_parmanager_dtorparmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002147_rage_parmanager_loadtree_char_const_ptr_char_const_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002150_rage_parmanager_savetree_char_const_ptr_char_const_ptr_rage_part(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_002154_rage_parmanager_findstructure_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002156_rage_parmanager_initfromstructure_rage_parstructure_ref_rage_par(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002158_rage_parinstancevisitor_visittoplevelstructure_rage_parparsables(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002161_rage_parstructure_issubclassof_rage_parstructure_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002169_rage_parmanager_buildtreenodefromstructure_rage_parstructure_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_002175_rage_partree_dtorpartree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002182_rage_parelement_findattribute_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002184_rage_partreenode_findchildwithname_char_const_ptr_rage_partreeno(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002186_rage_parattribute_findint64value_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002188_rage_partreenode_appendaschildof_rage_partreenode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002203_rage_parelement_findattributeboolvalue_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002204_rage_parattribute_finddoublevalue_const(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_002206_rage_parelement_findattributeint64value_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002278_rage_psofile_dtorpsofile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002282_rage_psoloadfile_char_const_ptr_rage_psoloadaction_rage_atfixedb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002288_rage_psoloadfromstructure_rage_psofile_ref_rage_parstructure_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002311_rage_atarray_rage_psoenumtableentry_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
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
extern "C" { pub fn f_002448_int_rage_formatf_n_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002561_rage_parmemberplacementvisitor_atstringmember_rage_atstring_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002581_rage_parelement_setname_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002582_rage_parelement_findattributeanycase_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002593_rage_parsettings_createstandardsettings(ph: *mut Phantom); }
extern "C" { pub fn f_002695_rage_atbinarymap_rage_pardelegateholderbase_ptr_unsigned_int_dat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002696_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002697_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_002720_rage_conststring_vcopy_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002721_rage_conststring_dtorconststring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002725_rage_atstringbuilder_atstringbuilder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002728_rage_atstringbuilder_dtoratstringbuilder(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002729_rage_atstringbuilder_preallocate_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002731_rage_atstringbuilder_append_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002732_rage_atstringbuilder_append_char(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002741_rage_atliteralstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002743_rage_atdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002746_rage_stristr_char_ptr_char_const_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002808_rage_atdelegator1_void_rage_sysserviceevent_ptr_removedelegate_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002840_rage_sysparam_get_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002843_rage_sysparam_getarray_float_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002863_rage_sysperformancetimer_sysperformancetimer_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002865_rage_sysperformancetimer_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002866_rage_sysperformancetimer_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002867_rage_sysperformancetimer_gettimems(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_002927_rage_sysmemstarttemp(ph: *mut Phantom); }
extern "C" { pub fn f_002928_rage_sysmemendtemp(ph: *mut Phantom); }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_002933_rage_sysmemgetbucketname_rage_memorybucketids(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_002945_rage_sysmempoolallocator_safeallocate_unsigned_long_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002946_rage_sysmempoolallocator_allocate_unsigned_long_unsigned_long_bo(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002947_rage_sysmempoolallocator_safefree_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002948_rage_sysmempoolallocator_free_void_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003085_rage_systaskparameters_systaskparameters(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003090_rage_systaskmanager_create_char_const_ptr_void_ptr_rage_systaskp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_003093_rage_systaskmanager_poll_rage_systaskhandle_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003095_rage_systaskmanager_wait_rage_systaskhandle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003140_rage_geomtvalues_findtvalueopensegtopoint_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_003141_rage_geomdistances_distancesegtopoint_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_003142_rage_geomtvalues_findtvaluesegtopoint_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_003150_rage_geomboxes_testpolygontoorientedboxfp_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_003155_rage_geom2d_test2dsegvsseg_float_ref_float_ref_rage_vector2_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_003159_rage_matrix34_fromquaternion_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003160_rage_matrix34_toquaternion_rage_quaternion_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003161_rage_quaternion_frommatrix34_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003170_rage_quaternion_slerp_float_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003172_rage_quaternion_twistangle_rage_vector3_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_003173_rage_vector2_vector2_rage_vector3_const_ref_rage_vector2_evector(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_003175_rage_vector3_vector3_rage_vector2_const_ref_rage_vector2_evector(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003176_rage_vector3_approachstraight_rage_vector3_const_ref_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003209_rage_imp_quatfrommat33v_imp33_float_vector_4_float_vector_4_floa(ph: *mut Phantom, a0: u128, a1: u128, a2: u128) -> u128; }
extern "C" { pub fn f_003798_rage_fwmvrequestid_op_rage_fwmvstateid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003830_rage_crtagiteratorbase_rage_crtagiteratorkey_true_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003835_rage_fwanimdirector_getmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003838_rage_fwclipset_getclip_rage_fwmvtemplateid_rage_fwmvclipidtype_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_003839_rage_fwmovenetworkinterface_getclipsetid_rage_fwmvtemplateid_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003840_rage_fwclipsetmanager_getclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003848_rage_fwanimdirector_isnetworkdefloaded_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003849_rage_fwanimdirector_getnetworkdef_rage_fwmvtemplateid_rage_fwmvn(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003852_rage_fwanimdirector_createnetworkplayer_rage_mvnetworkdef_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003876_rage_fwanimmanager_getclipifexistsbysetid_rage_fwmvtemplateid_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003878_rage_fwanimhelpers_getmovertracktranslationdiff_rage_crclip_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003880_rage_fwanimhelpers_getmovertrackrotation_rage_crclip_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003882_rage_fwanimhelpers_getmovertrackrotationdiff_rage_crclip_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003884_rage_fwanimhelpers_getmovertrackrotationdelta_rage_crclip_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003887_rage_fwanimhelpers_getmovertrackdisplacement_rage_crclip_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003896_rage_fwanimmanager_getclipifexistsbydictindex_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_003897_rage_fwanimmanager_getclipindict_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_003898_rage_fwanimmanager_getanimhashindict_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_003899_rage_fwanimmanager_countanimsindict_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003905_rage_fwanimmanager_findslotfromhashkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003911_rage_fwanimmanager_addref_rage_strlocalindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_003966_rage_fwclipset_getclipitembyindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003967_rage_fwclipset_getclipitem_rage_fwmvtemplateid_rage_fwmvclipidty(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003968_rage_fwclipset_getclipdictionaryindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003969_rage_fwclipset_getclipdictionarymetadata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003971_rage_fwclipset_getclipitemidbyindex_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_003977_rage_fwclipset_request_deprecated_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_003979_rage_fwclipset_isstreamedin_deprecated_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003984_rage_fwclipsetrequesthelper_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003995_rage_fwclipsetmanager_getclip_rage_fwmvtemplateid_rage_fwmvclips(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004010_rage_fwclipsetmanager_isstreamedin_deprecated_rage_fwmvtemplatei(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004015_rage_atbinarymap_rage_fwclipset_ptr_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004019_rage_fwclipsetrequesthelper_request_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_004037_rage_fwclipsetrequesthelper_dtorfwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004038_rage_fwclipsetrequesthelper_fwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004039_rage_fwclipsetrequesthelper_request_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004042_rage_fwclipsetrequesthelper_isinvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004101_rage_fwmovenetworkinterface_clearoutputparameters_rage_atarray_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004103_rage_fwmovenetworkinterface_setclipset_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004104_rage_fwmovenetworkinterface_setsubnetworkblendduration_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_004280_rage_fwposematcher_fwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004603_rage_fwextensionlist_add_rage_fwextension_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004619_rage_fwarchetype_getdrawable_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004620_rage_fwarchetype_getfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004631_rage_fwarchetypemanager_getarchetypename_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004632_rage_fwarchetypemanager_lookupmodelid_rage_fwarchetype_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004633_rage_fwarchetypemanager_getarchetype_rage_atnonfinalhashstring_u(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004634_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004661_rage_fwdynamicentitycomponent_fwdynamicentitycomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004663_rage_fwdynamicentitycomponent_createattachmentextensionifmissing(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004683_rage_fwentity_getaabb_rage_spdaabb_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004688_rage_fwextensionlist_destroy_rage_fwextension_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004723_rage_fwwaypointrecordingroute_uint8tosidedist_unsigned_char(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004725_rage_fwwaypointrecordingroute_uint8tombr_unsigned_char(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004729_rage_fwwaypointrecordingroute_uint8tocruisespeed_unsigned_char(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004740_rage_fwdecoratorextension_existson_rage_fwextensiblebase_ref_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_004767_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004788_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004816_rage_fwvectormap_drawline_rage_vector3_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_004820_rage_fwvectormap_drawcircle_rage_vector3_const_ref_float_rage_co(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_005053_rage_fwtextutil_getbytecount_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005538_rage_crframefilterdictionarystore_findframefilter_rage_fwmvtempl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005589_rage_fwpsostoreloader_unload_rage_fwpsostoreloadinstance_ref_boo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005648_rage_fwpsostoreloader_load_char_const_ptr_rage_fwpsostoreloadins(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_005651_rage_fwpsostoreloader_makesimplefileloadercore(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005732_rage_fwinteriorlocation_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005733_rage_fwinteriorlocation_getroomindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005915_rage_aitask_aitask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005916_rage_aitask_dtoraitask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005917_rage_fwrefawarebaseimpl_rage_datbase_dtorfwrefawarebaseimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005918_rage_aitask_makeabortable_rage_aitask_abortpriority_rage_aievent(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_005919_rage_aitask_setstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005921_rage_aitask_findparenttaskoftype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005922_rage_aitask_findsubtaskoftype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005926_rage_aitask_setnewtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005929_rage_aitaskmanager_dtoraitaskmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005933_rage_aitasktree_aborttaskswithpriority_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005936_rage_aitaskmanager_spewalltasks_rage_aitaskspew_spewtype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005940_rage_aitasktree_aitasktree_rage_fwentity_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_005941_rage_aitasktree_dtoraitasktree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005942_rage_aitasktree_deletetask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005944_rage_aitasktree_settask_rage_aitask_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_005945_rage_aitasktree_computeactivetaskpriority_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005946_rage_aitasktree_cleartask_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005947_rage_aitasktree_getactiveleaftask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005949_rage_aitasktree_findtaskbytypeactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005950_rage_aitasktree_findtaskbytypewithpriority_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_005951_rage_aitasktree_updatetasks_bool_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32); }
extern "C" { pub fn f_005952_rage_aitasktree_updatetask_rage_aitask_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_006035_rage_atmapentry_unsigned_int_bool_const_ref_rage_atmap_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006037_rage_strstreamingloadermanager_callkeepalivecallbackifnecessary(ph: *mut Phantom); }
extern "C" { pub fn f_006056_rage_strstreaminginfomanager_removeobject_rage_strindex_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006075_rage_strstreaminginfo_computevirtualsize_rage_strindex_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006076_rage_strstreaminginfo_computephysicalsize_rage_strindex_bool_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006084_rage_strstreaminginfomanager_requestobject_rage_strindex_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006088_rage_atarray_rage_atnonfinalhashstring_0_unsigned_short_grow_uns(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006122_rage_strstreaminginfomanager_getmodule_rage_strindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006123_rage_strstreamingloadermanager_callkeepalivecallback(ph: *mut Phantom); }
extern "C" { pub fn f_006124_rage_strstreamingengine_getobjectname_rage_strindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006167_rage_strstreaminginfomanager_isobjectinimage_rage_strindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006175_rage_strstreaminginfomanager_clearrequiredflag_rage_strindex_uns(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006189_rage_strstreaminginfomanager_registerdummyobject_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006215_rage_strstreamingloadermanager_loadrequestedobjects(ph: *mut Phantom); }
extern "C" { pub fn f_006249_rage_strstreamingmodule_resetplacestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006254_rage_strrequest_request_rage_strlocalindex_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_006255_rage_strrequest_hasloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006256_rage_strrequest_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006257_rage_strrequest_clearrequiredflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006299_rage_tcbox_calcstrengthmult_rage_vec3v_const_ref_int_int_int_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> f32; }
extern "C" { pub fn f_006316_rage_tcinstboxmgr_search_rage_spdaabb_const_ref_rage_atarray_rag(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006332_rage_tckeyframeuncompressed_setdefault(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006333_rage_tcdebug_changegraphs(ph: *mut Phantom); }
extern "C" { pub fn f_006335_rage_tcdebug_gettimecyclefloatvalues_int_rage_scalarv_ref_rage_v(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_006337_rage_tcdebug_gettimecyclecolorvalues_int_rage_scalarv_ref_rage_v(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_006339_rage_tcdebug_realignentry_rage_ptxkeyframe_ptr_int_rage_vec4v_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_006340_rage_tcmanager_getcycle_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006341_rage_tcdebug_getmodifiereditid_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_006342_rage_tcmanager_findmodifier_rage_atnonfinalhashstring_char_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006343_rage_tcmanager_findmodifierindex_rage_atnonfinalhashstring_char(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006344_rage_tcdebug_setmodifierfileinfo_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006346_rage_tcdebug_modifierrecordeditidchanged(ph: *mut Phantom); }
extern "C" { pub fn f_006347_rage_tcdebug_modifierchanged(ph: *mut Phantom); }
extern "C" { pub fn f_006349_rage_tcdebug_updatemodifiereditorwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_006355_rage_tcmanager_initmodifierdata_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006356_rage_tcmanager_loadmodifierfile_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006357_rage_tcmanager_finishmodifierload(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006358_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_006361_rage_atarray_float_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006373_rage_tcdebug_modifieraddvartomenu_rage_tcmodifiermenu_e_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006382_rage_tckeyframeuncompressed_interpolatekeyframeblend_rage_tckeyf(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_006383_rage_tckeyframeuncompressed_interpolatekeyframe_rage_tckeyframe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_006384_rage_tckeyframeuncompressed_interpolatekeyframe_rage_tckeyframe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32); }
extern "C" { pub fn f_006385_rage_tcmanager_querymodifierboxtree_rage_vec3v_const_ref_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006386_rage_tcmanager_getcycleindex_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006387_rage_tcmanager_calctimesampleinfo_unsigned_int_unsigned_int_unsi(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_006394_rage_tcmanager_findmodifierrecordindex_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006396_rage_tcmodifier_tcmodifier(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006397_rage_tcmodifier_dtortcmodifier(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006398_rage_tcmodifier_init_rage_tckeyframeuncompressed_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006399_rage_tcmodifier_init_rage_tcmodifier_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006400_rage_tcmodifier_createandlockvarmap_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006401_rage_tcmodifier_allocatevarmap_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006402_rage_tcmodifier_unlockvarmap_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006403_rage_tcmodifier_blend_rage_tcmodifier_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_006404_rage_tcmoddata_blend_rage_tcmoddata_const_ref_rage_tcmodifiertyp(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_006405_rage_tcmodifier_getmoddataunsafe_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006406_rage_tcmodifier_apply_rage_tckeyframeuncompressed_ref_float_cons(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_006407_rage_tcmodifier_getmoddatavalaunsafe_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_006412_rage_aitask_debug_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007078_rage_audsoundset_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007079_rage_audsoundset_find_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007411_rage_audentity_addbatchedsoundrequest_rage_audmetadataref_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007412_rage_audentity_createandplaysound_unsigned_int_rage_audsoundinit(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007419_rage_vfxlistitem_vfxlistitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007426_rage_vfxbrokenfraginfo_dtorvfxbrokenfraginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007528_rage_ptfxattach_attach_rage_ptxeffectinst_ptr_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_007559_rage_ptfxmanager_gettriggeredinst_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007676_rage_fwangle_limitradianangleforpitch_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_007692_rage_fwgeom_disttolinesqr_rage_vector3_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_007695_rage_fwportalcorners_calcboundingsphere_rage_spdsphere_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007697_rage_fwportalcorners_calcscreensurface_rage_grcviewport_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_007701_rage_fwportalcorners_calcsimpledistancetopointsquared_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007702_rage_fwportalcorners_calcambientscalefeather_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_007758_rage_netinterface_getnumremoteactiveplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007761_rage_netinterface_getremoteactiveplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007803_rage_networklogutils_writelogevent_rage_netlogginginterface_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007855_rage_netplayer_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008002_rage_nettimebarwrapper_nettimebarwrapper_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008074_rage_netobject_ispendingownerchange_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008075_rage_netobject_islocalflagset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008081_rage_netobject_hasbeencloned_rage_netplayer_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008682_rage_fwbasepool_fwbasepool_int_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_008683_rage_fwconfigmanager_getsizeofpool_unsigned_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008688_rage_fwbasepool_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008866_rage_atmapentry_unsigned_int_int_const_ref_rage_atmap_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008917_rage_fwtimer_enduserpause(ph: *mut Phantom); }
extern "C" { pub fn f_008918_rage_fwtimer_startuserpause_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008950_rage_fwbasepool_getjustindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008953_rage_fwknownrefholder_fwknownrefholder_void_ptr_ptr_rage_fwknown(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_009048_rage_crproperty_getattribute_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009207_rage_crskeletondata_findbonedata_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009394_rage_pftimebars_frameinit_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009432_rage_pftimebarmgr_getfunctiontotals_char_const_ptr_int_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_009454_rage_matrix34_makerotateto_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009623_rage_grcviewport_setnearfarclip_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_009653_rage_grcviewport_op_rage_grcviewport_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009694_rage_grcsetup_takescreenshot_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009859_rage_grceffect_setglobalfloatcommon_int_float_ptr_int_rage_grcef(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_009904_rage_gridmonitor_getarea_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009905_rage_monitorconfiguration_ismultihead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009908_rage_monitorconfiguration_update(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009912_rage_monitorconfiguration_getlandscapemonitor_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009995_rage_grcdevice_disablescissor(ph: *mut Phantom); }
extern "C" { pub fn f_010003_rage_grcdevice_setscissor_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_010040_rage_grcdevice_gethidef(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_010159_rage_grcdebugdraw_setdisplaydebugtext_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010163_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010170_rage_grcdebugdraw_rectaxisaligned_rage_vec2v_const_ref_rage_vec2(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010171_rage_grcdebugdraw_meter_rage_vec2v_const_ref_rage_vec2v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_010172_rage_grcdebugdraw_text_float_float_rage_color32_char_const_ptr_b(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i64, a4: i32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010173_rage_grcdebugdraw_metervalue_rage_vec2v_const_ref_rage_vec2v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010179_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010183_rage_grcdebugdraw_cross_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010184_rage_grcdebugdraw_circle_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010185_rage_grcdebugdraw_arc_rage_vec3v_const_ref_float_rage_color32_ra(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i64, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010188_rage_grcdebugdraw_boxaxisaligned_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010189_rage_grcdebugdraw_boxoriented_rage_vec3v_const_ref_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010193_rage_grcdebugdraw_capsule_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010194_rage_grcdebugdraw_axis_rage_mat34v_const_ref_float_bool_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_010195_rage_grcdebugdraw_arrow_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_010217_rage_grcdebugdraw_printtoscreencoors_char_const_ptr_int_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010220_rage_grcdebugdraw_adddebugoutput_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010221_rage_grcdebugdraw_adddebugoutput_rage_color32_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_010224_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010225_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010484_rage_iomapper_convertparametertomappervalue_rage_iomappersource(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_010491_rage_iomapper_getmkbaxisnegative_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_010492_rage_iomapper_getmkbaxispositive_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_010500_rage_iovalue_isdown_float_rage_iovalue_readoptions_const_ref_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010501_rage_iovalue_historyhelddown_unsigned_int_float_rage_iovalue_rea(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_010503_rage_iovalue_isup_float_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010511_rage_iovalue_ispressed_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010512_rage_iovalue_historypressed_unsigned_int_unsigned_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_010514_rage_iovalue_isreleased_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010515_rage_iovalue_historyreleased_unsigned_int_unsigned_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_010518_rage_iovalue_getnorm_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010521_rage_iovalue_getnorm01_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_011751_rage_fraginst_getcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011753_rage_fraginst_getcontrollingcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011755_rage_fraginst_getgroupfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011822_rage_matrix34_dot_rage_matrix34_const_ref_rage_matrix34_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012021_rage_fragphysicslodgroup_getlodbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012183_rage_ptxeffectinst_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012203_rage_ptxeffectinst_setoffsetpos_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012217_rage_ptxkeyframe_deleteentry_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012270_rage_ptxkeyframe_gettime_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_012275_rage_ptxkeyframe_updatewidget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012938_rage_atrangearray_rage_atarraywrap_unsigned_short_4_dtoratrangea(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013051_rage_quaternion_transform_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013429_char_ptr_rage_safecatf_char_char_ptr_unsigned_long_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_013451_rage_netstatus_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013641_rage_rc4context_rc4context(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013668_rage_nethttprequest_nethttprequest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013677_rage_nethttprequest_init_rage_sysmemallocator_ptr_wolfssl_ctx_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013705_rage_nethttprequest_addformparam_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014048_rage_netstopwatch_isrunning_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014164_rage_nettransactor_isinitialized_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014259_rage_rltaskmanager_doaddtask_unsigned_long_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014260_rage_rltaskmanager_destroytask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014294_rage_rlgetposixtime_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014295_rage_rlcreateuuid_unsigned_long_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014316_rage_rlgamerinfo_getgamerhandle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014319_rage_rlgamerhandle_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014361_rage_rlros_getcredentials_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014362_rage_rlroscredentials_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014368_rage_rlgamerhandle_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014459_rage_rltaskbase_configure_rage_netstatus_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014606_rage_rlroshttptask_configure_int_rage_rlhttptaskconfig_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_014609_rage_rlstats_cancel_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014662_rage_rlmetric_dtorrlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014685_rage_rlmetric_rlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014714_rage_rltelemetry_setdeferredflushblocked_bool(ph: *mut Phantom); }
extern "C" { pub fn f_014773_rage_rlprofilestats_cancel_rage_netstatus_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014775_rage_rlprofilestats_readstatsbygamer_int_rage_rlgamerhandle_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_014780_rage_rlprofilestatssynchronizetask_rlprofilestatssynchronizetask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014781_rage_rlprofilestatssynchronizetask_configure_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014791_rage_rlprofilestatsvalue_getfloat_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_014792_rage_rlprofilestatsvalue_getint32_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014793_rage_rlprofilestatsvalue_getint64_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014794_rage_rlprofilestatsvalue_setfloat_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_014795_rage_rlprofilestatsvalue_setint32_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014796_rage_rlprofilestatsvalue_setint64_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014800_rage_rlprofilestatsreadresults_rlprofilestatsreadresults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014836_rage_rlroscredentials_hasprivilege_rage_rlrosprivilegeid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014859_rage_rlros_getdefaultprivilege_rage_rlrosprivilegeid(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014948_rage_rlsclanguagetostring_rage_rlsclanguage(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_015981_rage_phcollider_applyimpulserigid_float_vector_4_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128, a2: u128); }
extern "C" { pub fn f_016010_rage_phsimulator_getlastinstancematrix_rage_phinst_const_ptr_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016011_rage_phsimulator_setlastinstancematrix_rage_phinst_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016088_rage_phinst_setinstflag_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016312_rage_phboundcomposite_updatebvh_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016317_rage_phboundcomposite_calculatecompositeextents_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016324_rage_pharticulatedcollider_applyimpulseart_float_vector_4_float(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: i32); }
extern "C" { pub fn f_016367_rage_phbound_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016436_rage_phboundcapsule_phboundcapsule(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016439_rage_phboundcapsule_setcapsulesize_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_016444_rage_phboundcapsule_getendpointa_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016445_rage_phboundcapsule_getendpointb_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016454_rage_phboundcomposite_phboundcomposite(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016455_rage_phboundcomposite_init_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016464_rage_phboundcomposite_setbound_int_rage_phbound_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016909_rage_phlevelnodetree_rage_phlooseoctreenode_testprobe_rage_phseg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_016911_rage_phlevelnodetree_rage_phlooseoctreenode_findandaddoverlappin(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016943_rage_scrthread_printstacktrace_void_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016949_rage_scrthread_getactivethread(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_018169_rage_mvparameterbuffer_generatefreecapacity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018173_rage_mvmotionweb_addref_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018477_grefcountntsimpl_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018478_gstring_op_gstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018481_gmatrix2d_setidentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018535_grefcountimpl_addref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018536_grefcountimpl_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018544_gstring_gstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018545_gstring_gstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018548_gstring_gstring_gstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018551_gutf8util_getlength_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018556_gstring_op_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018563_gstring_bernsteinhashfunctioncis_void_const_ptr_unsigned_long_un(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018565_gstringlh_gstringlh(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018566_gstringlh_copyconstructhelper_gstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018567_gstringdh_gstringdh_gmemoryheap_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018576_gtimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_018881_cpedinventory_getweaponammo_cweaponinfo_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_018889_cleveldata_getfriendlyname_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_018916_statid_op_statid_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018921_rage_fwentity_getmodelid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018981_cgps_getrelevantplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_018983_cgps_getrelevantplayercoords(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018984_rage_vector3_isclose_rage_vector3_const_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_019090_cwaypointrecording_increferencestorecording_cwaypointrecordingro(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019091_cwaypointrecording_decreferencestorecording_cwaypointrecordingro(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019112_cplayerinfo_nearvehiclemiss_cvehicle_const_ptr_cvehicle_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_019136_cvehiclerecordingmgr_isplaybackgoingonforcar_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019138_cvehiclerecordingmgr_isplaybackswitchedtoaiforcar_cvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019251_ctrafficlights_calculatenodelightcycle_rage_cpathnode_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019252_ctrafficlights_lightforpeds_int_float_float_float_bool_float(ph: *mut Phantom, a0: i32, a1: f32, a2: f32, a3: f32, a4: i32, a5: f32) -> i32; }
extern "C" { pub fn f_019288_caitarget_caitarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019289_caitarget_caitarget_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019290_caitarget_setentity_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019293_caitarget_caitarget_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019294_caitarget_setposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019295_caitarget_caitarget_caitarget_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019296_caitarget_getisvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019297_caitarget_getposition_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019299_caitarget_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019310_cexpensiveprocess_cexpensiveprocess_cexpensiveprocess_processtyp(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_019313_cexpensiveprocess_dtorcexpensiveprocess(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019340_cexpensiveprocessdistributer_addprocess_cexpensiveprocess_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019341_cexpensiveprocessdistributer_removeprocess_cexpensiveprocess_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019350_cspatialarray_findnearsegment_rage_vec3v_const_ref_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_019369_cambientmodelset_getcontainsmodel_unsigned_int_cambientmodelvari(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_019371_cambientmodelset_getrandommodelindex_cambientmodelvariations_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_019414_cconditionalanims_dtorcconditionalanims(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019416_cconditionalclipset_checkconditions_cscenariocondition_sscenario(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019422_cconditionalanims_chooseclipset_rage_atarray_cconditionalclipset(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019450_char_ptr_rage_formatf_char_100ul_char_ref_100ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019451_ailogging_getbooleanasstring_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_019452_ailogging_getdynamicentityisclonestringsafe_cdynamicentity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019453_ailogging_getdynamicentitynamesafe_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019456_ailogging_functionlog_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019458_ailogging_stacktracelog_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019460_ailogging_getdynamicentityownergamernamesafe_cdynamicentity_cons(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019466_ctasksfulldebuginfo_ctasksfulldebuginfo_cped_const_ptr_caidebugi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019474_crelationshipdebuginfo_crelationshipdebuginfo_cped_const_ptr_cai(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019476_cvehiclelockdebuginfo_cvehiclelockdebuginfo_cped_const_ptr_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_019479_cvehiclelockdebuginfo_dtorcvehiclelockdebuginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019495_crelationshipdebuginfo_dtorcrelationshipdebuginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019503_ctasksfulldebuginfo_dtorctasksfulldebuginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019529_crelationshipdebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019531_ctasksfulldebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019532_ctaskentervehicledebuginfo_ctaskentervehicledebuginfo_ctaskenter(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_019534_csetseatrequesttypedebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019535_csettargetentrypointdebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019536_csettargetseatdebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019537_csettaskfinishstatedebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019538_cgivepedjackedeventdebuginfo_cgivepedjackedeventdebuginfo_cped_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_019539_cgivepedjackedeventdebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019543_cvehiclelockdebuginfo_print(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019569_cgtaanimmanager_findframefilter_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019614_cfacialdatacomponent_requestfacialidleclip_cfacialdatacomponent(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019634_cmoveped_settasknetwork_rage_fwmovenetworkplayer_ptr_float_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_019652_cmoveped_settasknetwork_cmovenetworkhelper_ref_float_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_019653_cmoveped_cleartasknetwork_cmovenetworkhelper_ref_float_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_019654_cmoveped_setsecondarytasknetwork_cmovenetworkhelper_ref_float_un(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_019655_cmoveped_clearsecondarytasknetwork_cmovenetworkhelper_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_019667_cmoveped_switchtoanimated_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_019690_cmoveobject_setsecondarynetwork_rage_fwmovenetworkplayer_ptr_flo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019703_cclipeventtags_cikeventtag_isright_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019722_cclipeventtags_ccamerashaketag_getshakerefhash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019723_cclipeventtags_cpadshaketag_getpadshakeduration_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019724_cclipeventtags_cpadshaketag_getpadshakeintensity_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_019725_cclipeventtags_cblockragdolltag_shouldblockfromthisvehiclemoving(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019726_cclipeventtags_cmoverfixupeventtag_shouldfixuptranslation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019727_cclipeventtags_cmoverfixupeventtag_shouldfixuprotation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019728_cclipeventtags_cmoverfixupeventtag_shouldfixupx_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019729_cclipeventtags_cmoverfixupeventtag_shouldfixupy_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019730_cclipeventtags_cmoverfixupeventtag_shouldfixupz_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019731_cclipeventtags_cmoverfixupeventtag_getspecificvehiclemodelhashto(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019844_rage_matrix34_inverse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019946_rage_matrix34_dottranspose_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020807_cgivepedjackedeventdebuginfo_dtorcgivepedjackedeventdebuginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021237_audmusiceventmanager_triggerevent_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_021349_audweaponaudiocomponent_getweaponsettings_centity_const_ptr_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_021351_cped_shouldbedead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021359_auddamagestats_dtorauddamagestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021375_audpedaudioentity_handleanimeventflag_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021407_audspeechaudioentity_isambientspeechplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021410_audpedaudioentity_playbikemeleecombathit_worldprobe_cshapetesthi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021422_cpedmotiondata_getisrunning_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021761_audspeechaudioentity_saywhensafe_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_021765_audscannermanager_settimetoplaycdivehicleline(ph: *mut Phantom); }
extern "C" { pub fn f_021766_audscannermanager_decrementhelisinchase(ph: *mut Phantom); }
extern "C" { pub fn f_021767_audscannermanager_triggerscriptedreport_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_021927_audspeechaudioentity_doescontextexistforthisped_char_const_ptr_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_021957_audspeechaudioentity_say_char_const_ptr_audspeechinitparams_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_021961_audspeechaudioentity_setjackingspeechinfo_cped_ptr_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_021962_audspeechaudioentity_triggerjackingspeech(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022062_audvehicleaudioentity_requestweaponchargesfx_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_022071_audvehicleaudioentity_triggeralarm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022077_rage_fwentity_getobjectmtx_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022381_void_creplayeventmanager_storeevent_cpacketsoundphonering_cpacke(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022398_naaudioentity_createdeferredsound_unsigned_int_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_022658_cped_isdead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022666_auddooraudioentity_triggerdoorimpact_cped_const_ptr_rage_vec3v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_022738_auddooraudioentity_entitywantstoopendoor_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022739_auddooraudioentity_processcollisionsound_float_bool_centity_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022985_cpathfind_findnodepointersafe_rage_cnodeaddress(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023239_rage_atarray_naocclusionpathnodechild_0_unsigned_short_grow_unsi(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023396_cpacketevent_cpacketevent_ereplaypacketid_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023846_creplaymgr_replayrecordsound_unsigned_int_rage_audsoundinitparam(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_023849_creplaymgr_replayrecordsound_unsigned_int_unsigned_int_rage_auds(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_023877_creplaymgr_recordobject_cobject_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023891_replaybuffermarkermgr_addmarker_unsigned_int_unsigned_int_marker(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_023906_creplaycontrol_isplayeroutofcontrol(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023930_creplayeventmanager_shouldproceedwithpacket_cpacketevent_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023967_creplayinterfacemanager_ensureisbeingrecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023968_creplayinterfacemanager_ensurewillberecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024266_creplaymgrinternal_isawareofworldforrecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024286_creplaymgrinternal_isplaybackflagset_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_024295_cweapon_cancelcooktimer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024304_creplaymgrinternal_getbuildinginfo_cphysical_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_024305_creplaymgrinternal_recordmapobject_centity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026170_caminterface_isdominantrenderedcameraanyfirstpersoncamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026172_caminterface_getplayercontrolcamheading_cped_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026173_camgameplaydirector_getthirdpersoncamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026174_camgameplaydirector_getfirstpersonshootercamera_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026176_caminterface_getplayercontrolcamaimframe(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026177_camcinematicmountedcamera_shouldaffectaiming_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026181_caminterface_findfollowped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026189_caminterface_getviewporttanfovv(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_026199_caminterface_isfadedin(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026202_caminterface_isspherevisibleingameviewport_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_026205_cqueriableinterface_istaskcurrentlyrunning_int_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_026211_worldprobe_cshapetestfixedresults_unsigned_char_32_cshapetestfix(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026213_caminterface_computeshouldmakepedheadinvisible_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026215_caminterface_computepovturretcamerahash_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026219_cdynamicentity_getboneindexfrombonetag_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026223_camfirstpersonshootercamera_getrelativeheading_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_026224_camfirstpersonshootercamera_getrelativepitch_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_026238_cambasecamera_shake_unsigned_int_float_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_026243_camcinematicdirector_isrenderingcinematiccamerainsidevehicle_boo(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026245_camcinematicdirector_getfirstpersonvehiclecamera_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026248_rage_vector3_normalizesafe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026259_rage_fwentity_getskeleton_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026260_camcinematicmountedcamera_getcontrolhelper_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026261_camcinematicdirector_isrenderinganyinvehiclecinematiccamera_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026262_camcinematicdirector_isrenderinganyinvehiclefirstpersoncinematic(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026266_camgameplaydirector_isaiming_centity_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026286_camgameplaydirector_getthirdpersonaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026288_camgameplaydirector_getfollowvehiclecamera_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026293_rage_fwmatrixtransform_getpitch_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_026301_camdampedspring_update_float_float_float_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_026307_camcinematicmountedcamera_islookingleft_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026313_camcinematicmountedcamera_getfirstpersondrivebydata_cvehicle_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026329_worldprobe_cshapetestresults_dtorcshapetestresults(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026368_cpedintelligence_geteventoftype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_026415_cambaseobject_getnamehash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026457_camcontrolhelper_getviewmodeforcontext_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_026607_cambasedirector_shake_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_026623_rage_matrix34_toeulers_rage_vector3_ref_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026636_camcutscenedirector_iscutsceneplaying(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026648_camdebugdirector_getcamframe_cambasecamera_ptr_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026649_camdebugdirector_getcamframenonconst_cambasecamera_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026681_camgameplaydirector_computeweaponinfoforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026682_camgameplaydirector_computeisfirstpersonmodeallowed_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_026691_cpedintelligence_findtasksecondarybytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_026702_ccontrol_debugoutputscriptcallstack(ph: *mut Phantom); }
extern "C" { pub fn f_026703_camgameplaydirector_getfirstpersonvehiclecamera_centity_const_pt(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026704_camcontrolhelper_computeviewmodecontextforvehicle_cvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026705_cped_getplayerresetflag_cplayerresetflags_playerresetflags_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026706_camgameplaydirector_iscamerainterpolating_cambasecamera_const_pt(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026713_camgameplaydirector_getfollowpedcamera_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026714_camgameplaydirector_getfirstpersonpedaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026715_camgameplaydirector_isfirstpersonshooteraiming_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_026719_cweaponinfo_const_ptr_cweaponinfomanager_getinfo_cweaponinfo_rag(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026728_camgameplaydirector_disablefirstpersonthisupdate_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_026837_rage_imp_untransformfull_imp34_float_vector_4_float_vector_4_flo(ph: *mut Phantom, a0: u128, a1: u128, a2: u128, a3: i64, a4: i64) -> u128; }
extern "C" { pub fn f_026839_camfirstpersonshootercamera_whensprintinguserightstick_cped_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_027093_rage_matrix34_fromeulers_rage_vector3_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_028268_cviewportmanager_isusersmonitorwidescreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_029101_cdebug_setversionnumber(ph: *mut Phantom); }
extern "C" { pub fn f_029137_selectdisplayobject(ph: *mut Phantom); }
extern "C" { pub fn f_029148_deleteobject(ph: *mut Phantom); }
extern "C" { pub fn f_029335_cgamesessionstatemachine_reinit_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_029340_cgamesessionstatemachine_changelevel_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_029377_ctask_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029397_cutscenemanager_getcamentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029433_getorcreategroup_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029455_cdebugscene_tunewidgetdata_tunewidgetdata_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_029456_cdebugscene_queuetunewidget_cdebugscene_tunewidgetdata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029489_ccutscenecameraentity_getmaplodscale_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029490_ccutscenecameraentity_getreflectionlodrangestart_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029491_ccutscenecameraentity_getreflectionlodrangeend_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029492_ccutscenecameraentity_getreflectionslodrangestart_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029493_ccutscenecameraentity_getreflectionslodrangeend_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029494_ccutscenecameraentity_getlodmulthd_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029495_ccutscenecameraentity_getlodmultorphanhd_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029496_ccutscenecameraentity_getlodmultlod_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029497_ccutscenecameraentity_getlodmultslod1_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029498_ccutscenecameraentity_getlodmultslod2_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029499_ccutscenecameraentity_getlodmultslod3_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029500_ccutscenecameraentity_getlodmultslod4_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029501_ccutscenecameraentity_getwaterreflectionfarclip_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029502_ccutscenecameraentity_getlightssaointen_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029503_ccutscenecameraentity_getdirectionallightmultiplier_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029504_ccutscenecameraentity_getlensartefactmultiplier_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029505_ccutscenecameraentity_getbloommax_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029612_worldprobe_cshapetestdesc_setresultsstructure_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029613_worldprobe_cshapetestresults_freeintersections(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029650_cutscenemanager_terminateloadedonlyscene(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029689_cutscenemanager_requestcutscene_char_const_ptr_bool_rage_cutsman(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_029726_camframe_setworldmatrix_rage_matrix34_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029779_cblockview_getblockname_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029866_cdebugscene_focusentities_get_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029920_cdebugdrawstore_cdebugdrawstore(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029922_cdebugdrawstore_addaxis_rage_mat34v_const_ref_float_bool_unsigne(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_029924_cdebugdrawstore_addline_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_029927_cdebugdrawstore_addsphere_rage_vec3v_const_ref_float_rage_color3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_029928_cdebugdrawstore_addcapsule_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_029929_cdebugdrawstore_addtext_rage_vec3v_const_ref_int_int_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_029931_cdebugdrawstore_addobb_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_029932_cdebugdrawstore_addarrow_rage_vec3v_const_ref_rage_vec3v_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_029933_cdebugdrawstore_addcross_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029935_cdebugdrawstore_clearall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029965_rage_fwtransform_getb_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029966_rage_fwtransform_getc_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029967_cphysical_gethealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029969_cdebugscene_focusentities_isingroup_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029970_cdebugscene_focusentities_isempty(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_030722_ceventgroupped_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030739_cpedintelligence_addevent_cevent_const_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030741_geteventglobalgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030744_ceventnetwork_spewtotty_impl_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030746_geteventscriptnetworkgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030756_ceventhandlerhistory_getcurrentevent_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030763_ceventhandler_getcurrenteventtype_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030769_caitarget_dtorcaitarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030770_cshockingeventsmanager_add_ceventshocking_ref_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_030772_ceventshocking_dtorceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030801_cevent_cevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030814_cpeddamagecalculator_cpeddamagecalculator_centity_const_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_030815_cpeddamagecalculator_applydamageandcomputeresponse_cped_ptr_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32); }
extern "C" { pub fn f_030819_cped_isinjured_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_030828_ceventdamage_ceventdamage_centity_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_030829_ceventdamage_dtorceventdamage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030845_ceventagitated_ceventagitated_cped_ptr_agitatedtype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_030846_ceventagitated_dtorceventagitated(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030849_ceventdeath_ceventdeath_bool_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_030915_ceventnetworkentitydamage_isvehicleresponsibleforcollision_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_030931_ceventnetworkcashtransactionlog_ceventnetworkcashtransactionlog(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_030945_svehiclemissionparams_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_031017_ceventdraggedoutcar_ceventdraggedoutcar_cvehicle_const_ptr_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_031018_ceventdraggedoutcar_dtorceventdraggedoutcar(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031030_ceventpedjackingmyvehicle_ceventpedjackingmyvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_031031_ceventpedjackingmyvehicle_dtorceventpedjackingmyvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031035_ceventpedenteredmyvehicle_ceventpedenteredmyvehicle_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031036_ceventpedenteredmyvehicle_dtorceventpedenteredmyvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031069_ceventgivepedtask_ceventgivepedtask_int_rage_aitask_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_031070_ceventgivepedtask_dtorceventgivepedtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031143_ceventcopcarbeingstolen_ceventcopcarbeingstolen_cped_ptr_cvehicl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031144_ceventcopcarbeingstolen_dtorceventcopcarbeingstolen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031153_ceventcarundriveable_ceventcarundriveable_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031154_ceventcarundriveable_dtorceventcarundriveable(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031158_ceventfirenearby_dtorceventfirenearby(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031165_ceventstaticcountreachedmax_ceventstaticcountreachedmax_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_031169_ceventswitch2nm_ceventswitch2nm_unsigned_int_rage_aitask_ptr_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_031170_ceventswitch2nm_dtorceventswitch2nm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031235_ceventshocking_ceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031242_ceventshocking_set_rage_vec3v_const_ref_centity_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_031400_svehiclemissionparams_settargetentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_031406_ceventgunshotwhizzedby_isbulletinrange_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_032615_gfxtextdocview_settext_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_032693_gfxtextparagraphformat_freetabstops(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_032711_gfxtextdocview_parsehtml_char_const_ptr_unsigned_long_bool_garra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_032735_gfxtextdocview_getlinescount(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_032992_ghashsetbase_ghashnode_unsigned_long_long_gptr_gfxampfunctiondes(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033371_gfxdrawtextimpl_gfxdrawtextimpl_gfxdrawtextmanager_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033372_gfxtextformat_dtorgfxtextformat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033373_gfxfonthandle_dtorgfxfonthandle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033374_gfxdrawtextimpl_processimagetags_gfxtextdocview_ptr_gfxdrawtextm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_033383_gfxdrawtext_filter_initbydefaultvalues(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033386_gfxstatebagimpl_gfxstatebagimpl_gfxstatebagimpl_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033387_gfxtextallocator_gfxtextallocator_gmemoryheap_ptr_unsigned_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_033388_gfxfontmanager_commoninit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033389_gfxfontcachemanagerimpl_gfxfontcachemanagerimpl_gmemoryheap_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033390_gfxglyphrastercache_init_unsigned_int_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_033391_gfxresourceweaklib_gfxresourceweaklib_gfxresourcelib_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_033392_gfxmeshcachemanager_gfxmeshcachemanager_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_033398_gfxdrawtextmanager_textparams_textparams(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033399_gfxdrawtextmanager_settextparams_gfxtextdocview_ptr_gfxdrawtextm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_033401_gfxdrawtextmanager_createtempdoc_gfxdrawtextmanager_textparams_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32) -> i64; }
extern "C" { pub fn f_033402_gfxdrawtextmanager_gettextextent_char_const_ptr_float_gfxdrawtex(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64); }
extern "C" { pub fn f_033403_gfxdrawtextmanager_checkfontstateschange(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033405_gfxdrawtextmanager_gethtmltextextent_char_const_ptr_float_gfxdra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64); }
extern "C" { pub fn f_033407_gfxfontmanager_createfonthandlefromname_char_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_033411_gfxdrawtextmanager_begindisplay_gviewport_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033412_gfxdrawtextmanager_enddisplay(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_033465_gfxfontlib_gfxfontlib(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_033468_gfxfontlib_addfontsfrom_gfxmoviedef_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_033478_gfxfontlib_loadfontnames_gstringhash_gstring_gallocatorgh_gstrin(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_034749_ghashsetbase_gstringlh_hashnode_gfxfontmap_mapentry_gstring_noca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_034878_rage_sfscaleformmanager_shutdowndrawtextmanager(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_035037_cnewhud_clearsubtitletext(ph: *mut Phantom); }
extern "C" { pub fn f_035039_chudcolour_getcolourfromkey_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035040_chudcolour_getrgb_ehud_colours_unsigned_char(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_035059_cpedinventory_getweaponammo_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_035061_cnewhud_issnipersightactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035071_cnewhud_clearhelptext_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_035079_chudcolour_getrgba_ehud_colours(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035082_cnewhud_isweaponwheelactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035083_cnewhud_isradiowheelactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035106_chudtools_gethudposfromworldpos_rage_vector3_const_ref_etext_arr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_035276_cminimap_createsonarblip_rage_vector3_const_ref_float_ehud_colou(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_035277_cminimap_createsonarblipandreportstealthnoise_cped_ptr_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32) -> f32; }
extern "C" { pub fn f_035919_cweaponwheel_setholsterammoinclip_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036031_cscaleformmgr_requestremovemovie_int_emovieownertags(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_036034_cscaleformmoviewrapper_isactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036038_cscaleformmgr_ismovieactive_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036050_cscaleformmgr_endmethod_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036051_cscaleformmgr_addparambool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036053_cscaleformmgr_addparamint_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036056_cscaleformmgr_addparamstring_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036059_cscaleformmgr_createmovieinternal_cscaleformmgr_createmovieparam(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036175_cprofilesettings_set_cprofilesettings_profilesettingid_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036252_cpausemenu_getlanguagefromsystemlanguage(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036254_cpausemenu_isactive_pm_activeflags(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036364_cprofilesettings_getint_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036382_cprofilesettings_exists_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036386_cprofilesettings_write_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036680_metricbaseinfo_metricbaseinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036681_metricbasechar_metricbasechar(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037248_cscaleformmgr_beginmethod_int_escaleform_base_class_char_const_p(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_037386_metricbaseinfo_write_rage_rsonwriter_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_037657_cwanted_setwantedlevel_rage_vector3_const_ref_int_int_setwantedl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_037659_cseatmanager_countpedsinseats_bool_bool_bool_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_037661_cmodelinfo_requestassets_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037662_cmodelinfo_haveassetsloaded_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037664_time_time_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_037665_time_add_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_037667_date_date_int_date_month_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_037676_cclock_settime_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_037761_ccrime_reportcrime_ecrimetype_centity_ptr_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_037767_cwanted_setwantedlevelnodrop_rage_vector3_const_ref_int_int_setw(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_037888_cseatmanager_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_037901_cwanted_copsaresearching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037905_cwanted_shouldcompensateforslowmovingvehicle_centity_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_037907_cwanted_triggercopseesweaponaudio_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_037908_cwanted_gettimesincelastspotted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037945_cweather_forcetypenow_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037946_cweather_getprevtype(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037951_cweather_forcetypeclear_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_037957_cweather_getnettransitionprevtype(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037958_cweather_getnettransitionnexttype(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037990_cmapareas_getareaindexfromposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038130_cmodelinfo_getbasemodelinfo_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038164_cdispatchhelpersearchonfoot_cdispatchhelpersearchonfoot(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038382_cpedintelligence_addtaskdefault_rage_aitask_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038467_cbaseikmanager_getsolver_ikmanagersolvertypes_flags_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038495_cikmanager_lookat_unsigned_int_centity_const_ptr_int_eanimboneta(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32); }
extern "C" { pub fn f_038497_cikmanager_islooking(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038498_cikmanager_islooking_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038499_cikmanager_abortlookat_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038500_cikmanager_abortlookat_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038504_cikmanager_request_cikrequestarm_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038505_cikmanager_request_cikrequestbodylook_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038507_cikmanager_setrelativearmiktarget_rage_criksolverarms_earms_cdyn(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32, a6: f32, a7: f32, a8: f32, a9: f32); }
extern "C" { pub fn f_038508_cikmanager_aimweapon_rage_criksolverarms_earms_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_038509_cikmanager_pointgunatposition_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_038510_cikmanager_pointgunindirection_float_float_bool_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_038511_cikmanager_gettorsosolverstatus(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038512_cikmanager_gettorsominyaw(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038513_cikmanager_gettorsomaxyaw(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038514_cikmanager_gettorsominpitch(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038515_cikmanager_gettorsomaxpitch(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038516_cikmanager_gettorsoblend_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038517_cikmanager_settorsominyaw_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_038518_cikmanager_settorsomaxyaw_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_038519_cikmanager_settorsominpitch_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_038520_cikmanager_settorsomaxpitch_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_038521_cikmanager_settorsooffsetyaw_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_038524_cikmanager_setdisablepitchfixup_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038529_cikmanager_request_cikrequestbodyrecoil_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038531_cikmanager_request_cikrequesttorsovehicle_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038538_cikrequestbodylook_cikrequestbodylook_centity_const_ptr_eanimbon(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_038540_cikrequestarm_cikrequestarm_ikarm_centity_const_ptr_eanimbonetag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_038542_cikrequestarm_setadditive_rage_quatv_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038544_cikrequestbodyrecoil_cikrequestbodyrecoil_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038545_cikrequesttorsovehicle_cikrequesttorsovehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038650_cmlomodelinfo_getroomflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038652_cmlomodelinfo_getportaldata_unsigned_int_unsigned_int_ref_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_038732_centryexitpoint_canseatbereached_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038733_centryexitpoint_getseat_seataccess_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_038734_centryexitpoint_getentrypointposition_centity_const_ptr_cped_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_038735_cmodelseatinfo_calculateentrysituation_centity_const_ptr_cped_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32, a7: i64); }
extern "C" { pub fn f_038739_cmodelseatinfo_getentrypointindexforseat_int_centity_const_ptr_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_038740_cmodelseatinfo_getboneindexfromseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038741_cmodelseatinfo_getboneindexfromentrypoint_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038743_cmodelseatinfo_getshuffleseatforseat_int_int_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_038744_cmodelseatinfo_calculateseatsituation_centity_const_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_038745_cmodelseatinfo_calculateexitsituation_centity_ptr_rage_vector3_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_038747_cmodelseatinfo_evaluateseataccessfromallentrypoints_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64, a7: i32) -> i32; }
extern "C" { pub fn f_038749_cmodelseatinfo_evaluateseataccessfromentrypoint_cped_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i64, a9: i64) -> i32; }
extern "C" { pub fn f_038751_cseatmanager_getpedsseatindex_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038754_cmodelseatinfo_ispositionclearforped_cped_const_ptr_centity_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_038755_cpedmodelinfo_getlayoutinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038757_cseatmanager_getlastpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038851_cpedmodelinfo_isusingfemaleskeleton_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039010_cvehiclemodelinfo_getfirstpersonlookarounddata_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039011_cvehiclemodelinfo_getseatinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039012_cvehiclemodelinfo_getentrypointaniminfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039013_cvehiclemodelinfo_getseatanimationinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039014_cvehiclemodelinfo_getentrypointinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039211_cvehiclevariationinstance_getkit_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039220_cvehiclevariationinstance_hidemodonbone_cvehicle_const_ptr_int_s(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_039232_cvehiclevariationinstance_getmodifier_evehiclemodtype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039569_cdoor_getdooraudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039572_cdoor_settargetdoorratio_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_039574_cpathservergta_shouldavoidobject_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039576_cdoor_getdooropenratio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_039630_cweapon_sfireparams_dtorsfireparams(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039636_cdoor_isdoorshut_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039704_cobject_printskeletondata(ph: *mut Phantom); }
extern "C" { pub fn f_039708_rage_mthrandom_reset_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039720_cobject_addtoorremovefromprocesspostcameraarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039761_cobjectpopulation_createobject_rage_fwmodelid_eentityownedby_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_039877_cpathserver_isrequestresultready_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_039881_cpathserver_requestpath_rage_trequestpathstruct_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039882_cpathserver_getclosestpositionforped_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_039884_rage_cnavmeshtrackedobject_islastnavmeshintersectionvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039895_cpathserver_requestlineofsight_rage_vector3_const_ptr_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64, a8: f32) -> i32; }
extern "C" { pub fn f_039898_cpathserver_requestclosestcarnodesearch_rage_vector3_const_ref_f(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_039900_cpathserver_requestclosestposition_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: f32, a4: f32, a5: f32, a6: i32) -> i32; }
extern "C" { pub fn f_039902_cpathserver_lockpathresult_unsigned_int_int_ptr_rage_vector3_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_039903_cpathserver_unlockpathresultandclear_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_039904_cpathserver_getlineofsightresult_unsigned_int_bool_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_039906_cpathserver_getclosestcarnodesearchresultandclear_unsigned_int_r(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_039908_cpathserver_getclosestpositionresultandclear_unsigned_int_rage_c(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_039913_cpathserver_cancelrequest_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_040139_cpedgroupmembership_getmember_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_040150_cpedgroupmembership_countmembersexcludingleader_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040173_cpedgroupmembership_isfollower_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040174_cpedgroupmembership_ismember_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040175_cpedgroupmembership_isleader_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040177_cpedgroup_isplayergroup_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040184_cpedgroup_finddistancetonearestmember_cped_ptr_ptr_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_040194_cpedgroups_getpedsgroup_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040196_cpedgroups_areinsamegroup_cped_const_ptr_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040220_worldprobe_cshapetesthitpoint_gethitinst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040318_cstrikeboneset_getstrikebonetag_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_040319_cstrikeboneset_getstrikeboneradius_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_040322_cactionmanager_findstrikeboneset_unsigned_int_ref_unsigned_int_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_040330_cactionmanager_arepedsfriendlywitheachother_cped_const_ptr_cped(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_040998_cpednavcapabilityinfo_buildpathstyleflags_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041056_findplayerped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041062_cpedintelligence_findmovementtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041063_cpedintelligence_isfriendlywith_cped_const_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041064_cpedintelligence_isthreatenedby_cped_const_ref_bool_bool_bool_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_041073_centityboundai_getcorners_rage_vector3_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041075_cpedtargetting_cpedtargetting_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041077_cpedtargetting_getlosstatus_centity_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041079_centityboundai_computehitsidebyposition_rage_vector3_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041081_cpedmotivation_getmotivation_cpedmotivation_pedmotivationstate_c(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_041085_cpedtargetting_findtarget_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041088_cpedintelligence_getnearbyvehicles(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041116_cpedintelligence_flushimmediately_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041117_cpedintelligence_addtoeventhistory_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041129_rage_quaternion_fromeulers_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041186_cpedpopulation_removeallrandompeds(ph: *mut Phantom); }
extern "C" { pub fn f_041195_cpedintelligence_cleartasks_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041209_centityboundai_centityboundai_centity_const_ref_float_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32, a7: i64, a8: f32) -> i64; }
extern "C" { pub fn f_041210_cpedintelligence_getmoveblendratiofromgototask_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041219_cpedgeometryanalyser_canpedtargetpedasync_cped_ref_cped_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: i32, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_041222_cpedtargetting_registerthreat_centity_const_ptr_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041244_cplayerspecialabilitymanager_chargeevent_abilitychargeeventtype(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_041252_ceventscanner_scanforeventsnow_cped_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041254_cinwatereventscanner_isdrowninginvehicle_cped_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041257_cped_setplayerresetflag_cplayerresetflags_playerresetflags(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041261_cplayerinfo_ishardaiming(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041264_cpedintelligence_setcheckshockflag_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041265_cplayerinfo_isaiming_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041269_cpedintelligence_gettaskclimbladder_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041296_cpedfactory_destroyped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041298_cpedhelmetcomponent_disablehelmet_bool_rage_vector3_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_041316_incpedailoscounter(ph: *mut Phantom); }
extern "C" { pub fn f_041333_cpedgeometryanalyser_adjustpositionforcover_cped_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_041337_cpedhelmetcomponent_ishelmetenabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041339_cpedhelmetcomponent_enablehelmet_eanchorpoints_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_041340_cpedhelmetcomponent_shouldusepilotflighthelmet_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041345_cpedhelmetcomponent_ishelmetinhand(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041346_cpedhelmetcomponent_sethelmettexindex_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041347_cpedhelmetcomponent_sethelmetpropindex_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041348_cpedhelmetcomponent_setvisorpropindices_bool_bool_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041349_cpedhelmetcomponent_isalreadywearingahelmetprop_cvehicle_const_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041351_cpedhelmetcomponent_requestpreloadprop_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041352_cpedhelmetcomponent_haspreloadedprop(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041353_cpedhelmetcomponent_clearpreloadedprop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041357_cpedhelmetcomponent_getaircraftputontakeoffhelmetfpsclipset_cveh(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041371_cpediksettingsinfo_isikflagdisabled_ikmanagersolvertypes_flags_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041374_rage_fwregdref_cscenariopoint_rage_fwrefawarebasenonvirt_dtorfwr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041381_cpedintelligence_getcurrenttarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041382_cpedintelligence_registerthreat_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041385_cpedintelligence_findtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041397_cpedintelligence_clearsecondarytask_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041418_cpedintelligence_createclonetaskfortasktype_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041422_cpedailodmanager_shoulddofullupdate_cped_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041426_cpedintelligence_isbattleaware_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041427_cpedintelligence_getbattleawareness_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041428_cpedintelligence_getlastbattleeventtime_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041435_cpedintelligence_getcancombatroll_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041454_cpedmotiondata_setcurrentmoveblendratio_rage_vector2_ref_float_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_041455_cpedmotiondata_getisdesiredsprinting_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041456_cpedmotiondata_setpreviousfpsstate_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041489_cpedpopulation_getpopulationcontrolpointinfo_cpopctrl_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041496_findplayervehicle_cplayerinfo_ptr_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041509_rage_fwregdref_cscenariopoint_rage_fwrefawarebasenonvirt_reset_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041536_cvehicleintelligence_resetpretendoccupanteventdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041589_cexplosionmanager_cexplosionargs_dtorcexplosionargs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041650_cplayerinfo_getplayerstatinfoforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041652_cplayerinfo_getplayerindex_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041656_cplayerinfo_getdiveheightforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041664_cplayerinfo_issprintaimbreakoutover_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041666_cplayerinfo_isrunandgunning_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_041667_cplayerinfo_isassistedaiming(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041674_cplayerpedtargeting_clearlockontarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041679_cplayerinfo_isfiring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041680_cplayerinfo_processfpsstate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041681_cplayerinfo_controlbuttonsprint_cplayerinfo_esprinttype_bool(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_041686_cplayerinfo_issoftfiring(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041701_canimspeedups_shouldusempanimrates(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041714_cplayerinfo_isplayerpressinghorn_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041716_cplayerinfo_isfiring_s(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041718_cplayerinfo_canplayerwarpintovehicle_cped_const_ref_cvehicle_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041720_ccontrol_getvehicletargetdown_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_041721_cplayerinfo_iscombatrolling(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041722_cplayerinfo_isselectinglefttarget_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041723_cplayerinfo_isselectingrighttarget_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041727_cplayerinfo_reportstealthnoise_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_041743_cplayerinfo_canplayerperformvehiclemelee_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041744_cvehicle_getthrottle_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041746_reportplayerresetmismatch_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_041858_cpedmotivation_increasepedmotivation_cpedmotivation_pedmotivatio(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_041877_cpedtargetevaluator_findtarget_cped_const_ptr_int_float_float_ce(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: i64, a5: i64, a6: i64, a7: f32, a8: f32, a9: f32, a10: f32, a11: f32) -> i64; }
extern "C" { pub fn f_041879_cpedtargetevaluator_istargetvalid_rage_vec3v_const_ref_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: f32, a11: f32, a12: i32, a13: i32, a14: i32, a15: i32) -> i32; }
extern "C" { pub fn f_041884_cpedtargetting_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041927_cplayerpedtargeting_setlockontarget_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041928_cplayerpedtargeting_getfineaimingoffset_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041984_cserialisedfsmtaskinfo_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041990_ctaskinfo_ctaskinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041993_cclonedtaskinfoweapontargethelper_cclonedtaskinfoweapontargethel(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041994_cclonedtaskinfoweapontargethelper_readqueriablestate_caitarget_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041995_cclonedtaskinfoweapontargethelper_updatetarget_caitarget_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041998_cqueriableinterface_gettargetfortasktype_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042000_cqueriableinterface_addtasksequenceinfo_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042001_cqueriableinterface_gettasksequenceinfo_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042003_cqueriableinterface_gettaskinfofortasktype_int_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042004_cqueriableinterface_getstatefortasktype_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042007_cqueriableinterface_gettargetentrypointfortasktype_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042013_cqueriableinterface_gettasknetsequencefortype_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042131_cpeddamagemanager_addpedscar_cped_ptr_epeddamagezones_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: f32, a6: i64, a7: i32, a8: i32, a9: i32, a10: f32, a11: i32); }
extern "C" { pub fn f_042195_cpeddamagemanager_clearpeddecorations_unsigned_char_bool_bool_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_042489_cpedpropsmgr_getpedpropidx_cped_const_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042490_cpedvariationstream_gethighestweightedtcmodfirstpersonslotidx(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_042514_cpedvariationdebug_setcrowdsize_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_042515_cpedvariationdebug_setcrowdpedselection_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_042516_cpedvariationdebug_setdisttocreatecrowdfromcamera_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_042517_cpedvariationdebug_setwanderaftercreate_bool(ph: *mut Phantom); }
extern "C" { pub fn f_042518_cpedvariationdebug_setcreateexactgroupsize_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_042519_cpedvariationdebug_setcrowdspawnlocation_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042520_cpedvariationdebug_setcrowduniformdistribution_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_042555_createcrowdcb(ph: *mut Phantom); }
extern "C" { pub fn f_042629_cpedpropsmgr_ispedpropdelayeddelete_cped_const_ptr_eanchorpoints(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042632_cpedpropsmgr_getpropidxwithflags_cped_ptr_eanchorpoints_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042648_cpedpropsmgr_getnumactiveprops_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042814_worldprobe_debugfindgroundzfor3dcoord_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> f32; }
extern "C" { pub fn f_042817_worldprobe_debugfindgroundzfor3dcoord_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32, a5: i64, a6: i64) -> f32; }
extern "C" { pub fn f_042818_worldprobe_cshapetestmanager_debugsubmittest_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_042821_worldprobe_cshapetestmanager_aborttest_worldprobe_cshapetestresu(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042844_cwatertesthelper_getwaterheightatpositionincludingrivers_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_042848_worldprobe_cshapetestdesc_setexcludeentities_centity_const_ptr_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_042850_cpickupmanager_issuppressionflagset_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_043046_ccollisionhistory_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043049_ccollisionhistory_getmostsignificantcollisionrecordoftype_unsign(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043085_worldprobe_cshapetestdesc_setexcludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043226_cphysics_collideinstagainstglass_rage_phinst_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043234_worldprobe_cshapetestdesc_setincludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043242_cphysics_getmeasuringtoolpos_int_rage_vector3_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_043334_cwatertesthelper_getwaterlevelincludingrivers_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_043343_worldprobe_cshapetestbatchdesc_addcapsuletest_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043344_worldprobe_cshapetestbatchdesc_setcapsuledescriptors_worldprobe(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043485_water_iscameraunderwater(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043650_pausemenupostfxmanager_updatecurrenteffect(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043651_animatedpostfxstack_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043673_water_getwaterlevel(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_043677_animpostfxmanager_start_rage_atnonfinalhashstring_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_043706_animatedpostfx_applymodifiers_rage_tckeyframeuncompressed_ref_fl(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_043714_animpostfxmanager_stop_rage_atnonfinalhashstring_animpostfxmanag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043715_animpostfxmanager_setuserfadelevel_rage_atnonfinalhashstring_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_044123_water_getwaterlevelnowaves_rage_vector3_const_ref_float_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_044787_renderphaseseethrough_updatevisualdatasettings(ph: *mut Phantom); }
extern "C" { pub fn f_044889_creplaycoordinator_isactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045211_csavegamebuffer_freebuffercontainingcontentsofsavegamefile(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045224_csavingmessage_clear(ph: *mut Phantom); }
extern "C" { pub fn f_045245_csavegamequeue_push_csavegamequeuedoperation_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045252_cgenericgamestorage_ismpstatsloadattopofsavegamequeue(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045271_csavingmessage_begindisplaying_csavingmessage_etypeofmessage(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_045785_cped_computewandertask_cped_const_ref_cpopcycleconditions_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045788_cped_getpedhealthinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045789_cped_ismale_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045791_cped_getattackstrength_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_045793_cped_checkbraveryflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045796_cped_computedefaulttask_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045797_cped_getcurrentmotiontask_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045804_cped_clearactivateragdolloncollisionevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045821_cped_printskeletondata(ph: *mut Phantom); }
extern "C" { pub fn f_045834_cped_switchtoanimated_bool_bool_bool_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_045840_cped_getbonematrix_rage_matrix34_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045842_cped_forcemotionstatethisframe_cpedmotionstates_emotionstate_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_045874_cped_isonground_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045876_cped_setuseextractedz_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045878_cped_getmassforapplyvehicleforce_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_045882_cped_switchtoragdollinternal(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045883_cped_setpedoutofvehicle_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_045884_cped_knockpedoffvehicle_bool_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_045885_cped_switchtoragdoll_cevent_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045887_rage_fwentity_setnocollision_rage_fwentity_ptr_unsigned_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_045888_cped_applyanimatedvelocitywhilstattached_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_045889_cped_getprimarymotiontask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045903_cped_getiscrouching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045904_cped_setiscrouching_bool_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_045906_cped_canpedcrouch_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045909_cped_getprimarymotiontaskifexists_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045911_cped_wantstouseactionmode_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045912_cped_isusingactionmode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045913_cped_isfirstpersonshootermodeenabledforplayer_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045922_cped_getboneposition_rage_vector3_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045925_rage_fwmovenetworkinterface_getproperty_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045926_cped_getisdrivingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045968_cped_gettaskdata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045976_cped_setisstrafing_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045977_cped_setdesiredheading_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_045978_cped_setdesiredheading_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045979_cped_getfacingdirectionheading_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_045996_cped_getbonetagfromragdollcomponent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046001_cped_isusingstealthmode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046002_cped_isinfirstpersonvehiclecamera_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_046012_cped_setclothpinradiusscale_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046026_cped_getvehiclepedentering_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046054_cped_setusingactionmode_bool_cped_actionmodeenabled_float_bool_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_046070_cped_updatefpscamerarelativematrix(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046073_cped_processlefthandgripik_bool_bool_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_046074_cpedintelligence_gettasksecondaryactive_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046092_cped_debugattachpedtoentercar_char_const_ptr_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64); }
extern "C" { pub fn f_046096_cped_getbonepositioncachedv_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046099_cped_getragdollcomponentmatrix_rage_matrix34_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_046102_cped_getisenteringvehicle_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046103_cped_getspecialability_eplayerspecialabilityslot(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046104_cped_isexitingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046106_cped_forcedeath_bool_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046109_cped_setarreststate_earreststate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046113_cped_candohurttransition_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046122_cped_wantstomovequickly(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046131_cped_isgoonped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046132_cped_isbossped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046133_cped_getbossid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046135_cped_isgangped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046141_cped_isbeingjackedfromvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046144_cped_newsay_char_const_ptr_unsigned_int_bool_bool_int_cped_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32) -> i32; }
extern "C" { pub fn f_046146_cped_randomlysay_char_const_ptr_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_046147_cped_newsay_unsigned_int_unsigned_int_bool_bool_int_cped_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_046149_cped_canputonhelmet_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046150_cclipeventtags_ctagiteratorattributevalue_rage_crtag_rage_crprop(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_046151_csimpleblendhelper_getphase_ccliphelper_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_046152_cped_setpedinvehicle_cvehicle_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_046153_cped_getpedradiocategory_int_ref_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046160_cped_setclothpackageindex_unsigned_char_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046162_cped_setclothforcepin_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046163_cped_setclothlockcounter_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046164_cped_setclothwindscale_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046165_cped_setclothforceskin_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046167_cped_setclothnegateforce_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046170_cped_isallowedtodamageentity_cweaponinfo_const_ptr_centity_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_046175_cped_setdesiredvelocityclamped_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_046176_cped_getcontrolfromplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046181_cped_computeexternallydrivenpelvisoffsetforfpsstealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_046183_cped_getragdollcomponentfrombonetag_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046184_cped_getiksettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046192_cped_isdebuginvincible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046216_cped_hasvalidmovementmodeforweapon_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046227_cped_getconstrainthelperdofs_bool_float_ref_rage_vec3v_ref_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_046232_cped_usefirstpersonupperbodyanims_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046291_canimatedbuilding_printskeletondata(ph: *mut Phantom); }
extern "C" { pub fn f_046302_cdynamicentity_createskeleton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046303_cdynamicentity_createanimdirector_rage_rmcdrawable_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_046326_cgameworldheightmap_getmaxheightfromworldheightmap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_046327_cplayerswitchinterface_isactive_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_046362_cdatafilemgr_expandfilename_char_const_ptr_char_ptr_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046410_centity_getinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046438_cgameworld_add_centity_ptr_rage_fwinteriorlocation_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046443_cinteriorinst_getportalinroom_unsigned_int_unsigned_int_rage_fwp(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_046444_cinteriorinst_getmatchingportalinst_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_046459_cdynamicentity_getscriptthatcreatedme_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046484_centity_setisvisibleformodule_eisvisiblemodule_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046491_centity_getisonscreen_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046499_centity_getcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046500_centity_getisstatic_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046502_centity_getphysarch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046538_centity_getguid_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046545_centity_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046596_centityidhelper_computeentityid_rage_fwentity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046599_centityiterator_centityiterator_int_centity_const_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_046600_centityiterator_getnext(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046971_rage_atarray_ctasknmbrace_tunables_vehicletypeoverrides_0_unsign(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047012_cfocusentitymgr_getpos(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047020_cgameworld_findlocalplayerwanted(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047044_cgameworld_forallentitiesintersecting_rage_fwsearchvolume_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_047060_cphysical_activatephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047073_cphysical_sethealth_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047075_cphysical_setweapondamageinfo_centity_ptr_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_047100_cphysical_applyimpulse_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047106_cphysical_getlocalspeed_rage_vector3_const_ref_bool_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047121_cphysical_getglobalattachmatrix_rage_mat34v_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_047126_cphysical_debugprocessallattachments_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_047305_cgameworld_getmainplayerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047342_cscenecosttracker_writetolog_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047348_cscenecosttracker_getcosttitlestr_char_ptr_char_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_047744_cinteriorinst_testportalstate_bool_bool_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_047844_cportal_getdoorocclusion_cinteriorinst_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_047864_cportalvistracker_updateusingtarget_centity_const_ptr_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_047868_cportalvistracker_storeprimarydata_cinteriorproxy_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047869_cportalvistracker_getinteriorlocation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047880_cgameworld_findlocalplayercoors(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047982_cgameworld_findlocalplayervehicle_bool(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_048037_cgameworldheightmap_getminheightfromworldheightmap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_050115_cclipeventtags_ctagiteratorattributevalue_rage_crtag_rage_crprop(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_054887_cped_isfatallyinjured_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_059734_stats_commands_commandleaderboards2readrankprediction_leaderboar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_060664_ctaskmoveseekentity_215_centityseekposcalculatorxyoffsetrotated(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: i32) -> i64; }
extern "C" { pub fn f_060782_ccliphelper_isplayingclip_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063800_cthescripts_getcurrentscriptnameandprogramcounter(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063862_cscriptareas_ispointinangledarea_rage_vector3_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_063864_cscriptangledarea_set_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64); }
extern "C" { pub fn f_063879_cupsidedowncarcheck_iscarupsidedown_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063888_cboatisbeachedcheck_isboatbeached_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063893_csuppressedmodels_hasmodelbeensuppressed_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_063896_cscriptpeds_ispedstopped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063989_cscripthud_getscreenposfromworldcoord_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_064027_cliteralstringsforimmediateuse_clearstrings_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064028_cliteralstringsforimmediateuse_findfreeindex_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_064029_cliteralstringsforimmediateuse_setliteralstring_int_char_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_064030_cliteralstrings_clearpersistentstring_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064031_cliteralstrings_setpersistentliteraloccursinpreviousbriefs_int_b(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064032_cliteralstrings_getliteralstring_int_csubstringwithinmessage_eli(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_064033_cliteralstrings_getsingleframeliteralstringfromwritebuffer_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_064700_cshaderlib_setglobalemissivescale_float_bool(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_064705_cshaderlib_setglobalambientocclusioneffect_rage_vector4_const_re(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064714_cshaderlib_setglobalfogparams_cshaderlib_cfogparams_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064715_cshaderlib_setglobalfogcolors_rage_vec3v_const_ref_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_064724_cshaderlib_setreticuledisttexture_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064726_cshaderlib_setstereoparams_rage_vector4_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064729_cshaderlib_setfograyintensity_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_064736_csettingsmanager_csettingsmanager(ph: *mut Phantom); }
extern "C" { pub fn f_064737_csettingsmanager_getsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064790_cloadedmodelgroup_ismemberingroup_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_064832_cstreaming_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064840_cprioritizedclipsetrequest_isclipsetstreamedin_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064844_cprioritizedclipsetstreamer_addrequest_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_064846_cprioritizedclipsetstreamer_findrequest_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_064852_cprioritizedclipsetrequestmanager_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064855_cprioritizedclipsetrequestmanager_shouldclipsetbestreamedout_cpr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_064857_cprioritizedclipsetrequestmanager_isloaded_rage_fwmvtemplateid_r(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064860_cprioritizedclipsetrequestmanager_request_cprioritizedclipsetreq(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_064862_cprioritizedclipsetrequestmanager_hasclipsetbeenrequested_cprior(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_064863_cprioritizedclipsetrequestmanager_getprioritizedclipsetrequesthe(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_064999_cstreamingdebug_dumpmoduleinfoallobjects(ph: *mut Phantom); }
extern "C" { pub fn f_065000_cstreamingdebug_calculaterequestedobjectsizes_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065012_cstreamingdebug_getmemorystatsbyallocation_rage_atarray_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_065034_cstreamingdebug_getmainmemoryused_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065062_resourcememoryusedump_buildscenecoststreamingindices(ph: *mut Phantom); }
extern "C" { pub fn f_065063_resourcememoryusedump_dumpscenecostdetails_rage_fistream_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065332_ccontrol_getpedsprintisdown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065334_ccontrol_getpedsprintispressed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065335_ccontrol_getpedsprintisreleased_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065336_ccontrol_getpedsprinthistoryhelddown_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_065337_ccontrol_isvehicleattackinputdown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065338_ccontrol_isvehicleattackinputreleased_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065356_ccontrol_getmousesteeringmode_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_065358_ccontrol_getinputsource_rage_inputtype_int_unsigned_char_bool_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_065362_ccontrolmgr_reinitcontrols(ph: *mut Phantom); }
extern "C" { pub fn f_065374_ccontrol_setinputexclusive_int_rage_iovalue_disableoptions_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_065376_ccontrol_getinputname_rage_inputtype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065379_ccontrol_dodisableinput_unsigned_int_rage_iovalue_disableoptions(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_065384_ccontrol_isinputdisabled_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_065387_ccontrol_shakecontroller_int_int_bool_rage_iorumbleeffect_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_065388_cpad_applyactuatoreffect_rage_ioactuatoreffect_ptr_bool_int_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_065391_ccontrol_getinputbyname_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065392_ccontrol_getinputgroupbyname_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_065446_ccontrolmgr_getmainplayercontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065461_ccontrolmgr_getmainfrontendcontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065469_ccontrolmgr_startplayerpadshakebyintensity_unsigned_int_float_in(ph: *mut Phantom, a0: i32, a1: f32, a2: i32); }
extern "C" { pub fn f_065609_cfilemgr_getassetsfolder(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_065615_cfilemgr_openfile_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_065616_cfilemgr_readline_void_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_065617_cfilemgr_setdir_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065621_cfilemgr_openfileforwriting_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065622_cfilemgr_openfileforappending_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065801_csimpleblendhelper_setrate_ccliphelper_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_066002_ccombattaskmanager_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066062_ccoverpoint_getcoverpointposition_rage_vector3_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066067_ccoverpoint_getcoverdirectionvector_rage_vec3v_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_066079_ccover_calculatevantagepointforcoverpoint_ccoverpoint_ptr_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_066081_ctaskanimatedhitbyexplosion_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066097_ctaskcombatadditionaltask_ctaskcombatadditionaltask_int_cped_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32) -> i64; }
extern "C" { pub fn f_066122_ctaskstandguard_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066157_ctaskthreatresponse_ctaskthreatresponse_cped_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_066160_ctaskcombatadditionaltask_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066171_ctaskcombatflank_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066186_ctaskhelicopterstrafe_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066194_cnewhud_isshowinghudmenu(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_066226_ctaskmelee_setupmeleegetup_cnmblendoutsetlist_ref_cped_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_066231_ctaskmelee_getmeleegripfilter_rage_fwmvtemplateid_rage_fwmvclips(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066235_ctaskmeleeactionresult_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066253_worldprobe_cshapetesthitpoint_gethitentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066254_rage_crtagiteratorbase_rage_crtagiteratorpropertyattribute_true(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066283_ctaskmovecombatmounted_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066296_ctaskdyingdead_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066306_ctaskdyingdead_setforcednaturalmotiontask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066338_ctaskcomplexonfire_ctaskcomplexonfire_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_066350_ctaskdamageelectric_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066351_ctaskdamageelectric_ctaskdamageelectric_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_066388_ctaskinvestigate_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066463_ctaskcombat_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066533_ctaskcombat_getisplayingambientclip_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066542_ctaskreactaimweapon_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066562_ctasksearch_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066569_ctasksearchforunknownthreat_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066577_ctasksharkcircle_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066589_ctasksharkattack_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066653_bool_ccliphelper_isevent_rage_crpropertyattributebool_bool_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_066722_ctaskcover_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066728_ctaskcover_getshouldusecustomlowcoveranims_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066729_ctaskcover_isplayeraimingdirectlyinfirstperson_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066735_ctaskcover_computemvcamheadingforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_066770_rage_vector3_anglez_rage_vector3_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_066793_ctaskcover_isweaponclipsetloaded_bool_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_066797_ctaskentercover_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066812_cclipeventtags_findmoverfixupstartendphases_rage_crclip_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_066820_rage_vec_v4arctan2_float_vector_4_float_vector_4(ph: *mut Phantom, a0: u128, a1: u128) -> u128; }
extern "C" { pub fn f_066826_ctaskexitcover_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066836_ctaskincover_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066842_rage_fwentity_setanimatedvelocity_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066859_ctaskincover_restartweaponholding(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066890_ctaskincover_getthrownprojectileclipsetandclipids_cped_ptr_ctask(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i32) -> i32; }
extern "C" { pub fn f_066922_ctaskmotionincover_tunables_animstateinfos_dtoranimstateinfos(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_066923_ctaskmotionincover_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066966_ctaskaimgunfromcoverintro_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066977_ctaskaimgunfromcoveroutro_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066987_ctaskaimgunblindfire_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067003_caicoverclipvariationhelper_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067006_cplayercoverclipvariationhelper_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067010_cdynamiccoverhelper_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067032_cclipscalinghelper_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067038_ccoverdebug_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067073_ctaskadvance_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067087_ctaskaimfromground_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067091_ctaskboatchase_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067100_ctaskboatcombat_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067107_ctaskboatstrafe_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067129_ctaskdraggingtosafety_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067146_ctaskhelichase_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067159_ctaskhelicombat_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067170_ctaskplanechase_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067181_ctaskreacttobuddyshot_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067191_ctaskvariedaimpose_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067209_ctaskvehiclechase_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067233_ctaskvehiclepersuit_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067254_ctaskvehiclecombat_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067264_ctasksearchbase_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067269_ctasksearchinautomobile_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067273_ctasksearchinboat_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067276_ctasksearchinheli_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067279_ctasksearchonfoot_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067297_ctaskshootouttire_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067313_ctasksubmarinechase_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067319_ctasksubmarinecombat_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067326_ctasktargetunreachable_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067333_ctasktargetunreachableininterior_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067340_ctasktargetunreachableinexterior_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067364_crandomeventmanager_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067372_crandomeventmanager_cancreateeventoftype_erandomevent_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_067375_crandomeventmanager_seteventstarted_erandomevent(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_067383_ctaskpursuecriminal_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067395_ctaskreacttopursuit_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067409_ctaskstealvehicle_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067546_cclonedtaskinfoweapontargethelper_serialise_rage_csyncdatabase_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067591_csyncedvehicle_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067769_cambientanimationmanager_getpropsetfromhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_067771_cambientanimationmanager_chooseconditionalanimations_cscenarioco(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_067813_cambientaudio_getfinalizedhash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067814_cambientaudio_getpartialhash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067818_cambientaudiomanager_getexchange_cped_const_ref_cped_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_067838_clookathistory_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067841_cambientlookat_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067850_cclipeventtags_getmoveeventstartandendphases_rage_crclip_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_067853_ctaskambientclips_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067854_ctaskambientclips_ctaskambientclips_unsigned_int_cconditionalani(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_067880_csimpleblendhelper_getrate_ccliphelper_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_067914_ctaskchat_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067917_ctaskcomplexcontrolmovement_ctaskcomplexcontrolmovement_ctask_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_067918_ctaskcomplexcontrolmovement_getrunningmovementtask_cped_const_pt(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067921_ctaskflyingwander_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067925_ctaskplayeronfoot_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067926_ctaskplayeronfoot_checkforusemobilephone_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_067927_ctaskmobilephone_canusemobilephonetask_cped_const_ref_ctaskmobil(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_067931_ctaskplayeronfoot_ctaskplayeronfoot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067964_cclimbhandholddetected_dtorcclimbhandholddetected(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067972_ctaskslopescramble_getactivationangle(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_067980_ctaskplayeronfoot_getclimbdetectorparam_int_cped_ptr_spedtestpar(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_067982_ctaskmobilephone_createorresumemobilephonetask_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_067985_ctaskmoveplayer_ctaskmoveplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067993_ctaskmoveinair_ctaskmoveinair(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067997_ctaskswimmingwander_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068002_ctaskunalerted_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068047_ctaskwander_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068068_ctaskwanderinarea_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068126_ctaskdonothing_ctaskdonothing_int_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_068151_ctaskcomplexcontrolmovement_setmoveblendratio_cped_const_ptr_flo(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_068161_ctasksequencelist_ctasksequencelist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068163_ctasksequencelist_addtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068203_ctaskusesequence_ctaskusesequence_ctasksequencelist_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_068208_ctaskusesequence_gettasksequencelist_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068217_ctaskopendoor_ispedinfrontofdoor_cped_const_ptr_cdoor_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068261_ctasktakeoffhelmet_getvisorclipid_cped_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_068289_ctaskcallpolice_canmakecall_cped_const_ref_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068292_ctaskcallpolice_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068341_ctaskconversationhelper_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068439_ctaskmotionbase_ctaskmotionbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068440_ctaskmotionbase_dtorctaskmotionbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068459_ctaskmotionbase_resetstrafeclipset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068469_ctaskmotionbase_interpvalue_float_ref_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32) -> f32; }
extern "C" { pub fn f_068470_ctaskmotionbase_interpvalue_float_ref_float_float_bool_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_068471_ctaskmotionbase_getdefaultaimingstrafingclipset_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_068473_ctaskmotionbase_updateheadik(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068475_ctaskbirdlocomotion_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068507_ctaskmotionswimming_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068515_ctaskmotionswimming_setscubagearvariationforped_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068525_ctaskmotionswimming_candivefromposition_cped_ref_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_068571_ctaskmotionbasiclocomotionlowlod_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068585_ctaskfishlocomotion_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068614_ctaskhumanlocomotion_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068635_ctaskhumanlocomotion_checkclipsetformoveflag_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068638_ctaskhumanlocomotion_useleftfoottransition_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068664_ctaskmotionped_getweaponinfo_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068683_ctaskvehiclefsm_requestvehicleclipset_rage_fwmvtemplateid_rage_f(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_068711_ctaskmotionped_isaimingornotblocked_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068713_ctaskmotioninturret_isseatwithhatchprotection_int_cvehicle_const(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_068736_ctaskmotionaiming_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068745_ctaskmotionaiming_countfiringeventsinclip_rage_crclip_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068746_ctaskmotionaiming_setmoveflag_bool_rage_fwmvtemplateid_rage_fwmv(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_068747_ctaskmotionaiming_setmoveclip_rage_crclip_const_ptr_rage_fwmvtem(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_068748_ctaskmotionaiming_setmovefloat_float_rage_fwmvtemplateid_rage_fw(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_068749_ctaskmotionaiming_setmoveboolean_bool_rage_fwmvtemplateid_rage_f(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068750_ctaskmotionaiming_getmoveboolean_rage_fwmvtemplateid_rage_fwmvbo(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068751_ctaskmotionaiming_isplayingtransition_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068752_ctaskmotionaiming_isturning_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068778_ctaskmotionaiming_getaimintrofinished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068779_ctaskmotionaiming_stopaimintro(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068804_ctaskfall_ctaskfall_rage_fwflags_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068823_ctaskmotiontennis_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068846_ctaskquadlocomotion_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068874_ctaskmotioninturret_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068875_ctaskmotioninturret_ctaskmotioninturret_cvehicle_ptr_cmovenetwor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_068880_ctaskmotioninturret_ikhandstoturret_cped_ref_cvehicle_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_068898_ctaskmotioninturret_getadjuststepmoveratio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_068899_ctaskmotioninturret_hasvalidweapon_cped_const_ref_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_068900_ctaskmotioninturret_shouldusealternatejackduetoturretorientation(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_068923_ctaskcomplexevasivestep_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068929_ctaskgetup_ctaskgetup_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_068959_ctaskmovefollownavmesh_isunabletofindroute_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068981_ctaskfall_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069031_ctaskflytopoint_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069065_ctaskmovegotopointandstandstill_ctaskmovegotopointandstandstill(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: f32) -> i64; }
extern "C" { pub fn f_069067_ctaskmovestandstill_ctaskmovestandstill_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_069069_ctaskmoveachieveheading_ctaskmoveachieveheading_float_float_floa(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_069086_ctaskgetup_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069111_cnmblendoutsetmanager_getblendoutset_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069129_ctaskgetup_shoulduseinjuredgetup_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069130_ctaskgetup_pickstandardgetupset_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069131_cnmblendoutsetlist_settarget_caitarget_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069136_ctaskmovefollowpointroute_ctaskmovefollowpointroute_float_cpoint(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32, a10: f32) -> i64; }
extern "C" { pub fn f_069166_ctaskmovegotopoint_ctaskmovegotopoint_float_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_069177_ctaskgotoscenario_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069185_ctaskmovefollownavmesh_isfollowingnavmeshroute_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069239_rage_fwmatrixtransform_getroll_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069242_ctaskgotopointaiming_ctaskgotopointaiming_caitarget_const_ref_ca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_069298_ctaskmovetotacticalpoint_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069370_ctaskmovecrossroadattrafficlights_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069385_ctaskmovewithinattackwindow_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069440_cclimbdetector_process_rage_matrix34_ptr_spedtestparams_ptr_sped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_069445_cclimbdetector_getdetectedhandhold_cclimbhandholddetected_ref_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_069457_ctasknavbase_gettotalroutelength_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_069463_ctasknavbase_getpositionaheadonroute_cped_const_ptr_float_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_069465_ctasknavbase_getbasestatename_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_069475_ctaskmovefollownavmesh_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069477_ctaskmovefollownavmesh_ctaskmovefollownavmesh_float_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: i64, a8: f32, a9: i32) -> i64; }
extern "C" { pub fn f_069478_ctaskmovefollownavmesh_ctaskmovefollownavmesh_cnavparams_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_069525_ctaskparachute_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069529_ctaskparachute_clearparachutepackvariationforped_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069533_ctaskparachute_setparachutepackvariationforped_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069554_ctaskparachute_ispedinstatetoparachute_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069555_ctaskparachute_doespedhaveparachute_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069626_ctaskjetpack_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069662_ctaskparachuteobject_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069704_ctaskslidetocoord_ctaskslidetocoord_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_069710_ctaskslidetocoord_getsubtaskclip(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069717_ctasktakeoffpedvariation_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069735_cclimbdetector_resetflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069746_cclimbhandholddetected_calcphysicalhandholdoffset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069755_cclimbhandholddetected_cclimbhandholddetected(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069770_ctaskvault_resetragdolloncollision_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069804_ctaskclimbladder_settaskstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_069865_ctaskhelipassengerrappel_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069884_ctaskrappel_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069913_ctaskvault_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069915_ctaskvault_ctaskvault_cclimbhandholddetected_const_ref_rage_fwfl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_069993_ctaskjumpvault_ctaskjumpvault_rage_fwflags_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_069997_ctaskjumpvault_willvault_cped_ptr_rage_fwflags_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_069999_ctaskjump_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070364_cnmblendoutsetlist_add_rage_atnonfinalhashstring_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_070365_cnmblendoutsetlist_addwithfallback_rage_atnonfinalhashstring_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_070368_cnmblendoutsetmanager_requestblendoutset_rage_atnonfinalhashstri(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070369_cnmblendoutsetmanager_addrefblendoutset_rage_atnonfinalhashstrin(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070370_cnmblendoutsetmanager_releaseblendoutset_rage_atnonfinalhashstri(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070378_cgrabhelper_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070380_cgrabhelper_setgrabtarget_centity_ptr_rage_vector3_ptr_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_070385_ctaskanimatedattach_ctaskanimatedattach_cphysical_ptr_int_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_070399_ctasknmcontrol_ctasknmcontrol_unsigned_int_unsigned_int_rage_ait(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: f32) -> i64; }
extern "C" { pub fn f_070401_ctasknmbehaviour_canuseragdoll_cped_ptr_eragdolltriggertypes_cen(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_070416_ctasknmcontrol_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070419_ctasknmbehaviour_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070422_cnmtuningset_cnmtuningset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070423_cnmtuningset_dtorcnmtuningset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070428_cnmtuningset_addwidgetsinternal_rage_bkgroup_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070429_rage_atarray_rage_atbinarymap_cnmtuningset_rage_atnonfinalhashst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070430_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_070437_rage_atarray_rage_atbinarymap_cnmtuningset_rage_atnonfinalhashst(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_070438_rage_atarray_rage_atbinarymap_cnmtuningset_rage_atnonfinalhashst(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_070529_ctasknmhighfall_ctasknmhighfall_unsigned_int_cgrabhelper_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_070530_ctasknmbalance_ctasknmbalance_unsigned_int_unsigned_int_centity(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64, a6: f32, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_070535_cpedintelligence_addlocalclonetask_ctaskfsmclone_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_070539_ctasknmbalance_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070559_ctasknmbrace_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070561_ctasknmbrace_ctasknmbrace_unsigned_int_unsigned_int_centity_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_070583_ctasknmbuoyancy_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070599_ctasknmdraggingtosafety_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070613_ctasknmelectrocute_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070626_ctasknmexplosion_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070645_ctasknmflinch_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070660_ctasknmhighfall_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070680_ctasknminjuredonground_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070692_ctasknmjumprollfromroadvehicle_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070694_ctasknmjumprollfromroadvehicle_ctasknmjumprollfromroadvehicle_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_070706_ctasknmonfire_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070723_ctasknmprototype_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070745_ctasknmriverrapids_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070763_ctasknmshot_tunables_paramsets_dtorparamsets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070765_ctasknmshot_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070766_ctasknmshot_tunables_paramsets_paramsets(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070769_ctasknmshot_ctasknmshot_cped_ptr_unsigned_int_unsigned_int_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32, a7: i64, a8: i64, a9: i64, a10: i32) -> i64; }
extern "C" { pub fn f_070801_ctasknmsimple_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070834_ctasknmthroughwindscreen_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070836_ctasknmthroughwindscreen_ctasknmthroughwindscreen_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_070844_ctaskrageragdoll_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070858_ctaskagitated_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070859_ctaskagitated_ctaskagitated_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_070862_ctaskagitated_applyagitation_agitatedtype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_070889_ctaskconfront_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070912_ctasksmartflee_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070972_ctaskscenarioflee_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070997_ctaskexhaustedflee_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071004_ctaskwalkaway_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071017_ctaskgrowlandflee_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071024_ctaskintimidate_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071028_ctaskintimidate_cleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_071034_ctaskreactandflee_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071077_ctaskreactindirection_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071087_ctaskreacttoexplosion_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071100_ctaskreacttoimminentexplosion_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071117_ctaskshockingevent_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071125_ctaskshockingeventgoto_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071136_ctaskshockingeventhurryaway_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071145_ctaskshockingeventwatch_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071150_ctaskshockingeventreact_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071160_ctaskshockingeventbackaway_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071170_ctaskshockingeventreacttoaircraft_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071175_ctaskshockingpoliceinvestigate_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071185_ctaskshockingeventstopandstare_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071197_ctaskshove_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071208_ctaskshoved_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_071218_ctaskduckandcover_ctaskduckandcover(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072575_ctaskusescenario_ctaskusescenario_int_cscenariopoint_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_072605_cscenariopointmanager_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072606_cscenariopointmanager_append_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_072660_cscenarioinfomanager_updatescenariotypeswidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072684_cscenarioanimdebughelper_dtorcscenarioanimdebughelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072789_cscenariofinder_findoptions_findoptions(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072798_cscenariofinder_findnewscenario_cped_const_ptr_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_072812_cscenarioinfomanager_clearpedusedscenarios(ph: *mut Phantom); }
extern "C" { pub fn f_072815_cscenarioinfomanager_postload_rage_atarray_unsigned_int_0_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_072819_cscenarioinfomanager_cleardlcscenarioinfos_rage_atnonfinalhashst(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_072850_cscenariopointlookups_dtorcscenariopointlookups(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_072910_cscenariopointmanager_remove_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_072911_cscenariopointmanager_load(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_073088_cscenarioconditionhasprop_getpropmodelindex_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073406_cscenarioinfomanager_append_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073418_ctaskcouplescenario_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073429_ctaskcowerscenario_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073475_ctaskusescenario_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073477_ctaskusescenario_ctaskusescenario_ctaskusescenario_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_073506_ctaskusescenario_setstationaryreactionsenabledforped_cped_ref_bo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_073534_ctaskusevehiclescenario_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073536_ctaskusevehiclescenario_ctaskusevehiclescenario_ctaskusevehicles(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_073553_ctaskwanderingscenario_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073598_cpathfind_isregionloaded_rage_cnodeaddress_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_073612_ctaskpoliceorderresponse_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073626_ctaskarrestped_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073628_ctaskarrestped2_ctaskarrestped2_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_073642_ctaskarrestped_ctaskarrestped_cped_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_073652_ctaskbusted_ctaskbusted_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_073663_ctaskpolicewantedresponse_ctaskpolicewantedresponse_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_073677_ctaskswatorderresponse_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073685_ctaskswatfollowinline_ctaskswatfollowinline_caitarget_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_073698_ctaskswatwantedresponse_ctaskswatwantedresponse_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_073709_ctaskheliorderresponse_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073761_ctaskwitness_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_073763_ctaskwitness_ctaskwitness_cped_ptr_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_074161_ctask_ctask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_074163_ctaskclassinfomanager_gettaskname_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_074164_cmovenetworkhelper_getclipsetid_rage_fwmvtemplateid_rage_fwmvcli(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_074168_ctask_requestprocessmovesignalcalls(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_074172_ccliphelper_stopclip_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_074175_ceventnewtask_ceventnewtask_rage_aitask_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_074176_ceventnewtask_dtorceventnewtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_074191_ctaskvariedaimpose_aimpose_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074205_ctaskentercover_tunables_animstateinfos_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074216_ctaskexitcover_tunables_animstateinfos_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074235_ctaskaimgunfromcoverintro_aimstepinfoset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074240_ctaskaimgunfromcoveroutro_tunables_animstateinfos_parser_registe(ph: *mut Phantom); }
extern "C" { pub fn f_074247_ctaskaimgunblindfire_blindfireanimstateinfonew_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074253_ctaskmotionincover_tunables_animstateinfos_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074284_ctaskcombat_tunables_lackofhostility_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074408_ctaskreactaimweapon_tunables_ability_situation_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074412_ctaskreactaimweapon_tunables_ability_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074445_ccoverdebug_scontextdebuginfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074485_ctaskplayeronfoot_tunables_parachutepack_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074489_ctaskplayeronfoot_tunables_scubagear_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074493_ctaskplayeronfoot_tunables_jetpackdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074524_ctaskhumanlocomotion_tunables_smovingvarsset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074552_ctaskmotionswimming_scubagearvariation_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074556_ctaskmotionswimming_scubagearvariations_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074577_ctaskmovetotacticalpoint_tunables_scoring_coverpoint_penalty_par(ph: *mut Phantom); }
extern "C" { pub fn f_074579_ctaskmovetotacticalpoint_tunables_scoring_coverpoint_parser_regi(ph: *mut Phantom); }
extern "C" { pub fn f_074585_ctaskmovetotacticalpoint_tunables_scoring_navmeshpoint_penalty_p(ph: *mut Phantom); }
extern "C" { pub fn f_074587_ctaskmovetotacticalpoint_tunables_scoring_navmeshpoint_parser_re(ph: *mut Phantom); }
extern "C" { pub fn f_074591_ctaskmovetotacticalpoint_tunables_scoring_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074615_ctaskparachute_tunables_moveparameters_parachuting_parser_regist(ph: *mut Phantom); }
extern "C" { pub fn f_074617_ctaskparachute_tunables_moveparameters_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074631_ctaskparachute_tunables_parachutingai_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074636_ctaskparachute_tunables_landing_normalthresholds_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074638_ctaskparachute_tunables_landing_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074649_ctaskparachute_tunables_extraforces_parachuting_normal_parser_re(ph: *mut Phantom); }
extern "C" { pub fn f_074652_ctaskparachute_tunables_extraforces_fromstick_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074653_ctaskparachute_tunables_extraforces_fromvalue_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074654_ctaskparachute_tunables_extraforces_parachuting_braking_parser_r(ph: *mut Phantom); }
extern "C" { pub fn f_074660_ctaskparachute_tunables_extraforces_parachuting_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074663_ctaskparachute_tunables_extraforces_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074671_ctaskparachute_tunables_parachutebones_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074679_ctaskparachute_tunables_padshake_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074687_ctaskparachute_parachutepackvariation_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074691_ctaskparachute_parachutepackvariations_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074744_ctaskjetpack_tunables_camerasettings_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074746_ctaskjetpack_tunables_jetpackmodeldata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074749_ctaskjetpack_tunables_jetpackclipsets_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074781_cnmmessage_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074785_cnmtuningset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074790_cnmtuningsetgroup_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074800_ctasknmbrace_tunables_vehicletypeoverrides_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074806_ctasknmbrace_tunables_stuckonvehicle_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074811_ctasknmbrace_tunables_vehicletypetunables_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074817_ctasknmbehaviour_tunables_standardblendoutthresholds_parser_regi(ph: *mut Phantom); }
extern "C" { pub fn f_074861_ctasknmshot_tunables_impulses_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074870_ctasknmshot_tunables_paramsets_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074912_ctasknmprototype_tunables_timedtuning_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074931_ctasknmsimple_tunables_tuning_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_075024_ctuningmanager_registertuning_ctuning_ref_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_075089_ctaskexitvehicleseat_exittoaimvehicleinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_075153_ctaskvehicleblockcruiseinfront_tunables_probes_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_075328_ctaskcomplex_dtorctaskcomplex(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075332_ctaskfsmclone_ctaskfsmclone(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075333_ctaskfsmclone_readqueriablestate_cclonedfsmtaskinfo_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075334_ctaskfsmclone_isinscope_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075339_ctaskfsmclone_createsubtaskfromclone_cped_ptr_ctasktypes_etaskty(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_075342_ctaskhelperfsm_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075344_cmovenetworkhelperfunctions_setmovenetworkstate_rage_fwmvrequest(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_075345_cmovenetworkhelper_sendrequest_rage_fwmvrequestid_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075346_cmovenetworkhelper_forcestatechange_rage_fwmvstateid_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075347_cmovenetworkhelperfunctions_ismovenetworkstatefinished_rage_fwmv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_075348_cmovenetworkhelper_getboolean_rage_fwmvtemplateid_rage_fwmvboole(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075349_cmovenetworkhelper_getfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_075353_ccliprequesthelper_requestclipsbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075354_cgetupsetrequesthelper_cgetupsetrequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075355_cgetupsetrequesthelper_dtorcgetupsetrequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075356_cgetupsetrequesthelper_request_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075358_cgetupprobehelper_dtorcgetupprobehelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075362_cgetupprobehelper_submitprobes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075368_cprioritizedclipsetrequesthelper_dtorcprioritizedclipsetrequesth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075369_cprioritizedclipsetrequesthelper_releaseclipset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075370_cprioritizedclipsetrequesthelper_isclipsetstreamedin_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075372_cprioritizedclipsetrequesthelper_getclipsetid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075373_cprioritizedclipsetrequesthelper_requestclipset_rage_fwmvtemplat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075376_cprioritizedclipsetrequesthelper_shouldclipsetbestreamedin_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075377_cprioritizedclipsetrequesthelper_shouldclipsetbestreamedout_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075379_cambientcliprequesthelper_cambientcliprequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075381_cambientcliprequesthelper_dtorcambientcliprequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075382_cambientcliprequesthelper_releaseclips(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075383_cambientcliprequesthelper_requestclips_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075384_cambientcliprequesthelper_chooserandomclipset_cscenariocondition(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_075389_cnavmeshroutesearchhelper_cnavmeshroutesearchhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075398_cvehiclefollowroutehelper_getvehiclebonnetposition_cvehicle_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_075405_rage_vector3_normalize(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075406_cpathnoderoutesearchhelper_cpathnoderoutesearchhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075410_cpathnoderoutesearchhelper_findstartnodesindirection_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075411_cpathnoderoutesearchhelper_getnextnode_int_int_short_ref_bool_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i64); }
extern "C" { pub fn f_075412_cpathnoderoutesearchhelper_findpathdirection_rage_cnodeaddress_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i64) -> i32; }
extern "C" { pub fn f_075413_cpathnoderoutesearchhelper_verifynodesareloadedforarea_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075415_cpathnoderoutesearchhelper_getzerothlanepositionsfornodeswithlin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_075416_cpathnoderoutesearchhelper_findupcomingjunctionturndirection_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_075417_cpathnoderoutesearchhelper_findjunctionturndirection_rage_cnodea(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: f32, a7: i64) -> i32; }
extern "C" { pub fn f_075418_cpathnoderoutesearchhelper_findpathdirectioninternal_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_075422_cvehiclefollowroutehelper_cvehiclefollowroutehelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075423_cvehiclelanechangehelper_shouldchangelanesforthisentity_cvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_075427_cvehiclefollowroutehelper_constructfromnodelist_cvehicle_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_075428_cvehiclefollowroutehelper_calculatedesiredspeeds_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_075430_cvehiclefollowroutehelper_findclosestpointtoposition_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_075433_cvehiclefollowroutehelper_rendercaraitaskinfo_int_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_075437_cvehiclefollowroutehelper_findtargetcoorsandspeed_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32) -> f32; }
extern "C" { pub fn f_075438_cvehiclefollowroutehelper_calculatepositionandorientationonvirtu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64, a7: i64, a8: i64, a9: i32) -> i32; }
extern "C" { pub fn f_075440_rage_vec_v4arccosfast_float_vector_4(ph: *mut Phantom, a0: u128) -> u128; }
extern "C" { pub fn f_075442_cnavmeshlineofsighthelper_cnavmeshlineofsighthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075445_casyncprobehelper_casyncprobehelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075446_casyncprobehelper_dtorcasyncprobehelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075447_casyncprobehelper_resetprobe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075448_casyncprobehelper_starttestlineofsight_worldprobe_cshapetestprob(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075449_casyncprobehelper_getproberesults_probestatus_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075450_casyncprobehelper_getproberesultswithintersection_probestatus_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075457_csimpleblendhelper_blendoutclip_ccliphelper_ptr_float_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_075458_ccliphelper_startclipbysetandclip_cdynamicentity_ptr_rage_fwmvte(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32) -> i32; }
extern "C" { pub fn f_075459_csimpleblendhelper_blendinclip_cdynamicentity_ptr_ccliphelper_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i32); }
extern "C" { pub fn f_075460_csimpleblendhelper_setlooped_ccliphelper_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075461_csimpleblendhelper_setfilter_ccliphelper_ptr_rage_crframefilter(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075462_cmovenetworkhelper_setclip_rage_crclip_const_ptr_rage_fwmvtempla(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075463_ccliphelper_startclipbysetandclip_cdynamicentity_ptr_rage_fwmvte(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32, a7: f32, a8: i32) -> i32; }
extern "C" { pub fn f_075465_ccliphelper_startclipbydictandhash_cdynamicentity_ptr_int_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32, a7: f32, a8: i32) -> i32; }
extern "C" { pub fn f_075466_csimpleblendhelper_sendblendoutsignals_csimpleblendhelper_eprior(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_075468_ccliphelper_isevent_rage_fwmvtemplateid_rage_fwmvpropertyidtype(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075470_cmovenetworkhelper_cmovenetworkhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075472_cmovenetworkhelper_dtorcmovenetworkhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075473_cmovenetworkhelper_createnetworkplayer_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075474_cmovenetworkhelper_setfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_075475_cmovenetworkhelper_setflag_bool_rage_fwmvtemplateid_rage_fwmvfla(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_075479_cmovenetworkhelper_releasenetworkplayer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075484_cgenericcliphelper_cgenericcliphelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075485_cgenericcliphelper_dtorcgenericcliphelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075486_cgenericcliphelper_blendinclipbysetandclip_cdynamicentity_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_075487_cgenericcliphelper_blendinclipbyclip_cdynamicentity_ptr_rage_crc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_075489_cgenericcliphelper_blendoutclip_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_075491_cmovenetworkhelper_setfilter_rage_crframefilter_ptr_rage_fwmvtem(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075495_cchathelper_getpedstask_cped_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075499_cmovenetworkhelper_setsubnetwork_rage_fwmovenetworkplayer_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075500_cmovenetworkhelper_setclipset_rage_fwmvtemplateid_rage_fwmvclips(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075501_cmovenetworkhelper_getclipset_rage_fwmvtemplateid_rage_fwmvclips(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075502_cmovenetworkhelper_getclip_rage_fwmvtemplateid_rage_fwmvclipidty(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_075503_cmovenetworkhelper_setboolean_rage_fwmvtemplateid_rage_fwmvboole(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075504_cmovenetworkhelper_getfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075508_cmovenetworkhelper_setupdeferredinsert_cmovenetworkhelper_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_075515_cplayattachedcliphelper_setinitialoffsets_cped_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075516_cplayattachedcliphelper_computeinitialsituation_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075517_cplayattachedcliphelper_computesituationfromclip_rage_crclip_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_075518_cplayattachedcliphelper_computeoffsets_rage_crclip_const_ptr_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_075519_cplayattachedcliphelper_applyvehicleentryoffsets_rage_vector3_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i32, a7: i32, a8: i64); }
extern "C" { pub fn f_075520_ctaskvehiclefsm_getmoverfixupinfofromclip_ctaskvehiclefsm_smover(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075521_cplayattachedcliphelper_applyvehicleexitoffsets_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32); }
extern "C" { pub fn f_075523_ctuning_dtorctuning(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075524_cvehiclecliprequesthelper_cvehiclecliprequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075525_cvehiclecliprequesthelper_dtorcvehiclecliprequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075526_cvehiclecliprequesthelper_update_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_075527_cvehiclecliprequesthelper_requestclipset_rage_fwmvtemplateid_rag(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075528_ctaskentervehicle_cnc_shouldforcemichaelvehicleentryanims_cped_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075529_cvehiclecliprequesthelper_requestclipsfromvehicle_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_075530_cvehiclecliprequesthelper_getclosesttargetentrypointindexforped(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_075531_cvehiclecliprequesthelper_choosevariationclipsetfromentryindex_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_075532_cvehiclecliprequesthelper_requestdirectaccessentryclipsforseat_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_075533_cvehiclecliprequesthelper_requestclipsfromseat_cvehicle_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_075534_ctaskvehiclefsm_candisplaceped_cped_const_ptr_cped_const_ptr_cve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_075535_cvehiclecliprequesthelper_requestclipsfromentrypoint_cvehicle_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_075537_cvehiclecliprequesthelper_debug_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075538_ctaskentervehiclealign_getcommonclipsetid_centity_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_075539_ctaskentervehiclealign_getentryclipsetid_centity_const_ptr_int_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_075540_ctaskvehiclefsm_isvehicleclipsetloaded_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075542_cvehiclejunctionhelper_approachingredlight_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075550_ctalkhelper_talk_cped_ref_ctalkhelper_options_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075551_ctalkhelper_talk_cped_ref_cped_ref_cambientaudioexchange_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075552_ctalkhelper_onpedfailedtotalk_cped_const_ref_cambientaudio_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075553_ctalkhelper_onpedtalkedtoped_cped_ref_cped_ref_cambientaudio_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075556_cpedintelligence_talkinfo_talkinfo_cped_ptr_cambientaudio_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_075557_cupperbodyaimpitchhelper_calculateaimvector_cped_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_075558_cupperbodyaimpitchhelper_computedesiredpitchsignal_cped_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> f32; }
extern "C" { pub fn f_075561_cupperbodyaimpitchhelper_getdefaultclipset_cped_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075562_cupperbodyaimpitchhelper_updatepitchparameter_cped_const_ref_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075567_cequipweaponhelper_shouldpedswapweapon_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075575_creacttobeingjackedhelper_checkforandmakeoccupantsreacttobeingja(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_075578_ccoverpointstatushelper_dtorccoverpointstatushelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075589_cscenariocliphelper_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075599_ctimehelpers_gettimesince_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_075604_ctaskcontrolvehicle_ctaskcontrolvehicle_cvehicle_ptr_rage_aitask(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_075607_cfindclosestfriendlypedhelper_find_cped_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_075608_cfirstpersonikhelper_cfirstpersonikhelper_cfirstpersonikhelper_e(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_075609_cfirstpersonikhelper_reset_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075610_cfirstpersonikhelper_dtorcfirstpersonikhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075611_cfirstpersonikhelper_process_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075612_cfirstpersonikhelper_processlook_cped_ptr_rage_mat34v_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075613_rage_imp_untransformfull_imp34_rage_mat34v_ref_float_vector_4_fl(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: u128, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_075618_cpedswaitingtobeoutofsightmanager_cpedinfo_done(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075631_ctaskmove_ctaskmove_float(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_075632_ctaskmove_dtorctaskmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075642_ctasktreeclone_recalculatetasks(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075643_ctasktreeclone_createclonetaskfromtaskinfo_ctaskinfo_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_075644_ctasktreeclone_startlocalclonetask_ctaskfsmclone_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075664_ctuning_registertuning_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_075665_ctunablesgroup_load_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_075666_ctunablesgroup_addwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075677_cdefaulttaskdataset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_075683_smotiontaskdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_075687_cmotiontaskdataset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_075695_ctaskleaveanycar_ctaskleaveanycar_int_cvehicleenterexitflags_fla(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_075697_ctaskexitvehicle_ctaskexitvehicle_cvehicle_ptr_cvehicleenterexit(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_075702_ctaskinvehiclebasic_ctaskinvehiclebasic_cvehicle_ptr_bool_ccondi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_075703_ctaskinvehiclebasic_dtorctaskinvehiclebasic(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075708_ctaskinvehiclebasic_checkforplayingambients(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075709_ctaskvehiclemountedweapon_istaskvalid_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075710_ctaskvehiclegun_ctaskvehiclegun_ctaskvehiclegun_etaskmode_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_075711_ctaskmotioninvehicle_checkforclosingdoor_cvehicle_const_ref_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_075712_ctaskmotioninautomobile_wantstoduck_cped_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075713_ctaskmotioninvehicle_getopendoorfordoubledoorseat_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_075714_ctaskmotioninvehicle_getdirectaccessentrydoorfromped_cvehicle_co(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_075715_ctaskmotioninvehicle_checkifstairsconditionpasses_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075716_ctaskclosevehicledoorfrominside_isdoorclearofpeds_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_075719_ctaskcardrive_ctaskcardrive_cvehicle_ptr_float_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_075720_ctaskcardrive_ctaskcardrive_cvehicle_ptr_rage_aitask_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_075721_ctaskcardrive_dtorctaskcardrive(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075724_ctaskentervehicle_ctaskentervehicle_cvehicle_ptr_seatrequesttype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: f32, a6: f32, a7: i64, a8: i64) -> i64; }
extern "C" { pub fn f_075726_ctaskcardrive_setupcar(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075732_ctaskdrivepointroute_ctaskdrivepointroute_cvehicle_ptr_cpointrou(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32, a6: f32, a7: i32) -> i64; }
extern "C" { pub fn f_075733_ctaskdrivepointroute_dtorctaskdrivepointroute(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075738_ctaskcarsettempaction_ctaskcarsettempaction_cvehicle_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_075739_ctaskcarsettempaction_dtorctaskcarsettempaction(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075744_ctaskplayerdrive_ctaskplayerdrive(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075748_ccarenterexit_makeundraggedpassengerpedsleavecar_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075750_ctaskplayerdrive_setstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_075751_ctaskplayerdrive_getdesiredstate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075752_ctaskvehiclefsm_computetargettransforminworldspace_cvehicle_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_075753_ctaskinvehicleseatshuffle_ctaskinvehicleseatshuffle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_075754_ctaskvehiclefsm_canpedshuffletoorfromturretseat_cvehicle_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075755_ctaskvehiclefsm_canpedshuffletoorfromextraseat_cvehicle_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075756_rage_quaternion_multiplyfromleft_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075757_ctaskvehiclefsm_canpedmanualshuffletotargetseat_cvehicle_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_075758_ctaskvehiclefsm_getpedinorusingseat_cvehicle_const_ref_int_cped(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_075760_ctaskcomplexgetoffboat_dtorctaskcomplexgetoffboat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075764_ctaskcomplexgetoffboat_probeforboat_cped_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075765_ctaskcomplexgetoffboat_createsubtask_int_cped_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_075766_ctaskcomplexgetoffboat_warppedoffboat_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075769_ctaskcontrolvehicle_setdesiredsubtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075770_ctaskcontrolvehicle_dtorctaskcontrolvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075774_ctaskcontrolvehicle_entervehicle_onenter_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075784_ctaskcontrolvehicle_drive_onupdateclone_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075789_ctasksetpedinvehicle_ctasksetpedinvehicle_cvehicle_ptr_int_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_075790_ctasksetpedinvehicle_dtorctasksetpedinvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075792_ctaskvehiclefsm_computelocaltransformfromglobal_cped_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_075793_ctaskvehiclefsm_updatepedvehicleattachment_cped_ptr_rage_vec3v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_075797_ctasksmashcarwindow_getwindowtosmash_cped_const_ptr_bool_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_075798_cphysical_isbrokenflagset_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075804_ctaskcarreacttovehiclecollision_ctaskcarreacttovehiclecollision(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_075805_ctaskcarreacttovehiclecollision_dtorctaskcarreacttovehiclecollis(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075807_ctaskcarreacttovehiclecollision_isbusdriver_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075812_ctaskcarreacttovehiclecollisiongetout_ctaskcarreacttovehiclecoll(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32) -> i64; }
extern "C" { pub fn f_075813_ctaskcarreacttovehiclecollisiongetout_dtorctaskcarreacttovehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075816_ctaskcarreacttovehiclecollisiongetout_collapse_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075822_ctaskentervehicle_geteventpriorityforjackedped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075823_ctaskentervehicle_processhelipreventgoingthroughgroundlogic_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_075824_ctaskvehiclefsm_findgroundpos_centity_const_ptr_cped_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_075825_ctaskentervehicle_caninterruptenterseat_cped_ref_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_075826_ctaskmotioninautomobile_willputonhelmet_cvehicle_const_ref_cped(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075827_ctaskentervehicle_checkforshuffleinterrupt_cped_ref_cmovenetwork(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_075828_ctaskentervehicle_checkforstartengineinterrupt_cped_ref_cvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_075829_ctaskmotioninvehicle_getdirectaccessentrydoorfromseat_cvehicle_c(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_075830_ctaskentervehicle_startmovenetwork_cped_ref_cvehicle_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i64, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_075831_ctaskentervehicle_checkforstreamedentry(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075832_ctaskentervehicle_shouldusecombatentryanimlogic_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075833_ctaskentervehicle_checkforjackingpedfromoutside_cped_ptr_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075834_ctaskentervehicle_checkforenteringseat_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_075835_ctaskentervehicle_passesplayerdriveraligncondition_cped_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075836_ctaskentervehicle_processsmashwindow_cvehicle_ref_cped_ref_int_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: i64, a6: i64, a7: i32); }
extern "C" { pub fn f_075837_ctaskentervehicle_possiblyreportstolentvehicle_cped_ref_cvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075838_cclipeventtags_findeventphase_rage_crclip_const_ptr_rage_fwmvtem(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075839_ctaskentervehicle_processbikearmik_cvehicle_const_ref_cped_ref_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: i32, a6: i32); }
extern "C" { pub fn f_075840_ctaskvehiclefsm_processbikehandlearmik_cvehicle_const_ref_cped_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_075841_cclipeventtags_findspecificiktags_rage_crclip_const_ptr_float_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_075842_ctaskvehiclefsm_ctaskvehiclefsm_cvehicle_ptr_seatrequesttype_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_075843_ctaskentervehicle_dtorctaskentervehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075844_ctaskvehiclefsm_dtorctaskvehiclefsm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075849_ctaskvehiclefsm_readqueriablestate_cclonedfsmtaskinfo_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075850_ctaskentervehicle_settargetentrypoint_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_075855_ctaskentervehicle_ensurelocallyjackedpedsexitvehicle(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075859_ctaskvehiclefsm_processblockragdollactivation_cped_ref_cmovenetw(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075863_ctaskentervehicle_shouldbeinactionmodeforcombatentry_bool_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075864_ctaskentervehicle_checkshouldprocessenterrearseat_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075865_ctaskentervehicle_settargetseat_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_075866_ctaskvehiclefsm_isanygroupmemberusingentrypoint_cped_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_075867_ctaskentervehicle_setseatrequesttype_seatrequesttype_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_075868_ctaskvehiclefsm_canforcepedtoshuffleacross_cped_const_ref_cped_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075869_ctaskmotioninvehicle_processsteeringwheelarmik_cvehicle_const_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075870_ctaskvehiclefsm_isvehiclelockedforplayer_cvehicle_const_ref_cped(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075871_ctaskvehiclefsm_arepedsfriendly_cped_const_ptr_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075872_ctaskvehiclefsm_getentrypedistryingtoenter_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075873_ctaskvehiclefsm_getseatpedistryingtoenter_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075876_ctaskentervehicle_pickdoor_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075877_ctaskentervehicle_streamanimstoalign_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075878_ctaskentervehicle_streamanimstoopendoor_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075879_ctaskentervehicle_waitforstreamedentry_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075880_ctaskentervehicle_streamedentry_onenter(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075881_ctaskentervehicle_streamedentry_onupdate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075882_ctaskentervehicle_climbup_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075883_ctaskentervehicle_climbup_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075884_ctaskvehiclefsm_setremotepedusingdoors_cped_const_ref_cvehicle_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_075885_ctaskentervehicle_vaultup_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075886_ctaskentervehicle_vaultup_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075887_ctaskentervehicle_pickupbike_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075888_ctaskentervehicle_pickupbike_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075889_ctaskentervehicle_pullupbike_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075890_ctaskentervehicle_pullupbike_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075891_ctaskentervehicle_setuprightbike_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075892_ctaskentervehicle_opendoor_onenter_cped_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075893_ctaskentervehicle_aligntoenterseat_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075894_ctaskentervehicle_aligntoenterseat_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075895_ctaskentervehicle_jackpedfromoutside_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075896_ctaskentervehicle_jackpedfromoutside_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075897_ctaskentervehicle_streamanimstoenterseat_onupdate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075898_ctaskentervehicle_enterseat_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075899_ctaskentervehicle_enterseat_onupdate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075900_ctaskentervehicle_waitforpedtoleaveseat_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075901_ctaskentervehicle_waitforpedtoleaveseat_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075902_ctaskentervehicle_setpedinseat_onenter(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075903_ctaskentervehicle_setpedinseat_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075904_ctaskentervehicle_closedoor_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075905_ctaskentervehicle_closedoor_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075906_ctaskentervehicle_shuffletoseat_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075907_ctaskentervehicle_shuffletoseat_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075908_ctaskentervehicle_trytograbvehicledoor_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075909_ctaskentervehicle_trytograbvehicledoor_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075910_ctaskentervehicle_waitforvalidentrypoint_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075911_ctaskvehiclefsm_checkplayerexitconditions_cped_ref_cvehicle_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_075912_ctaskentervehicle_settaskfinishstate_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_075913_ctaskentervehicle_checkforoptionaljackorrideaspassenger_cped_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075914_ctaskentervehicle_checkforoptionalforcedentrytoturretseatandgett(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075915_ctaskentervehicle_checkforoptionalforcedentrytorearseatandgetsea(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075916_ctaskentervehicle_processoptionalforcedrearseatdelay_cped_ptr_cv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075917_ctaskvehiclefsm_isopenseatisblocked_cvehicle_const_ref_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075918_ctaskentervehicle_processfriendlyjackdelay_cped_ptr_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075919_ctaskentervehicle_processoptionalforcedturretdelay_cped_ptr_cveh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075920_ctaskvehiclefsm_shouldwarpinandoutinmp_cvehicle_const_ref_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075921_ctaskentervehicle_checkforbikeonground_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075922_ctaskentervehicle_checkforopendoor_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075923_ctaskentervehiclealign_shouldcomputeidealtargetforturret_centity(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_075924_ctaskentervehicle_storeinitialreferenceposition_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075925_ctaskentervehiclealign_shouldtriggeralign_centity_const_ref_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_075926_ctaskentervehicle_checkforgotoclimbuppoint_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075927_ctaskvehiclefsm_pedshouldwarpbecausehangingpedintheway_cvehicle(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075928_ctaskvehiclefsm_canpedwarpintohoveringvehicle_cped_const_ptr_cve(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_075929_ctaskentervehicle_checkforbikefallingover_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075930_ctaskentervehicle_checkifplayerispickingupmybike_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075931_ctaskgotocardoorandstandstill_ctaskgotocardoorandstandstill_cphy(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: f32, a6: f32, a7: f32, a8: i32, a9: i32, a10: i64) -> i64; }
extern "C" { pub fn f_075932_ctaskentervehicle_startentervehiclelookat_cped_ptr_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075933_ctaskentervehicle_pedhasreservations(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075934_ctaskentervehicle_pedshouldwaitforleadertoenter(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075935_ctaskentervehicle_isseatoccupiershuffling_cped_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075936_ctaskvehiclefsm_ispedheadingtoleftsideofvehicle_cped_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075937_ctaskgotocardoorandstandstill_setmoveblendratio_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_075938_ctaskentervehiclealign_computealigntarget_rage_vector3_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i32); }
extern "C" { pub fn f_075939_ctaskvehiclefsm_spewstreamingdebugtotty_cped_const_ref_cvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32); }
extern "C" { pub fn f_075940_ctaskentervehicle_setupragdolloncollision_cped_ref_cvehicle_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075941_ctaskentervehicle_createaligntask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075942_ctaskvehiclefsm_setpassengerstolookatplayer_cped_const_ptr_cvehi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075943_ctaskvehiclefsm_pointturretinneturalposition_cvehicle_ref_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_075944_ctaskentervehicle_processcomponentreservations_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075945_ctaskentervehicle_processdoorsteps(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075946_ctaskentervehiclealign_alignsucceeded_centity_const_ref_cped_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_075947_ctaskentervehicle_checkforrestartingalign_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075948_ctaskentervehicle_entryclipsetloadedandset(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075949_ctaskentervehicle_shouldabortbecausevehiclemovingtoofast_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075950_ctaskentervehicle_trytoreservedoor_cped_ref_cvehicle_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_075951_ctaskentervehicle_storeinitialoffsets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075952_ctaskentervehicle_setclipsforstate(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075953_ctaskentervehicle_setclipplaybackrate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075954_ctaskentervehicle_getclipandphaseforstate_float_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_075955_ctaskvehiclefsm_processopendoor_rage_crclip_const_ref_float_cveh(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_075956_ctaskopenvehicledoorfromoutside_processopendoorhelper_ctaskvehic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64, a6: i32, a7: i32); }
extern "C" { pub fn f_075957_ctaskvehiclefsm_processapplyforce_cmovenetworkhelper_ref_cvehicl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075958_ctaskentervehicle_initextragetinfixuppoints(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075959_ctaskentervehicle_updateextragetinfixuppoints_rage_crclip_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_075960_ctaskentervehicle_canunreservedoorearly_cped_const_ref_cvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075961_ctaskentervehicle_pickorpullupbike_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_075962_ctaskvehiclefsm_processseatbonearmik_cvehicle_const_ref_cped_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_075963_ctaskentervehicle_shouldreevaluatebecausepedenteredourseatwecant(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075964_ctaskentervehicle_canupdatestatefromnetwork(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075965_ctaskentervehicle_warppedtogetinpointifnecessary_cped_ref_cvehic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075966_ctaskentervehicle_processjackingstatus(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075967_ctaskvehiclefsm_processturrethandgripik_cvehicle_const_ref_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64); }
extern "C" { pub fn f_075968_ctaskvehiclefsm_setdoordrivingflags_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_075969_ctaskvehiclefsm_cleardoordrivingflags_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_075970_ctaskentervehicle_checkforshuffle(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075971_ctasktrytograbvehicledoor_ctasktrytograbvehicledoor_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_075973_ctaskvehiclefsm_shouldalterentrypointforopendoor_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_075974_ctaskvehiclefsm_getdoorforentrypointindex_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_075977_ctaskentervehicle_shoulduseclimbuptarget_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075978_ctaskentervehicle_shoulduseidealturretseattarget_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075979_ctaskentervehicle_computeidealturretseattarget_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075980_ctaskentervehicle_shouldusegetintarget_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075981_ctaskentervehicle_shoulduseentrytarget_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075982_ctaskentervehicle_computeentrytarget_rage_vector3_ref_rage_quate(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075983_cphysical_setattachoffset_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075986_ctaskentervehiclealign_passesgroundconditionforalign_cped_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075987_ctaskmovegotovehicledoor_computetargetradius_cped_const_ref_cent(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> f32; }
extern "C" { pub fn f_075988_ctaskvehiclefsm_findbestturretseatindexforvehicle_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_075989_ctaskvehiclefsm_untransformfromworldspacetorelative_cvehicle_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_075990_ctaskentervehicle_trytoreserveseat_cped_ref_cvehicle_ref_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_075991_ctaskvehiclefsm_computeidealseatmatrixforposition_cturret_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_075992_ctaskexitvehicle_isvehicleonground_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075993_ctaskentervehiclealign_ctaskentervehiclealign_centity_ptr_int_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_075994_ctaskvehiclefsm_getdoorratioforentrypoint_cvehicle_const_ref_int(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_075995_ctaskentervehicleseat_getmultipleentryanimclipratemodifier_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> f32; }
extern "C" { pub fn f_075996_ctaskvehiclefsm_drivedoorfromclip_cvehicle_ref_ccardoor_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_075997_ctaskvehiclefsm_getentrypointinfrontifbeingused_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_075998_ctaskvehiclefsm_processdoorhandlearmikhelper_cvehicle_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: i64, a6: i64); }
extern "C" { pub fn f_075999_bool_cclipeventtags_findeventphase_rage_crpropertyattributebool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_076000_ctaskvehiclefsm_isanyplayerenteringdirectlyasdriver_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076001_ctaskentervehicle_warpifpossible(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076002_ctaskentervehicle_checkisentrypointblockedbyreardoor_cphysical_p(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_076003_ctaskentervehicle_calculatelocalspaceentrymodifier_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_076011_ctaskentervehiclealign_dtorctaskentervehiclealign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076015_ctaskentervehiclealign_isenteringstandingturretseat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076016_ctaskvehiclefsm_processdoorhandlearmik_cvehicle_const_ref_cped_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32); }
extern "C" { pub fn f_076017_ctaskentervehiclealign_havealignclipsfinished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076021_ctaskentervehiclealign_isshortestaligndirectionleft_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076024_ctaskvehiclefsm_findfrontentryforentrypoint_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_076025_ctaskvehiclefsm_doesvehiclehavedoublebackdoors_cvehicle_const_re(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076030_ctaskopenvehicledoorfromoutside_trylockeddoor_onenter(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076031_ctaskopenvehicledoorfromoutside_trylockeddoor_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076032_ctaskopenvehicledoorfromoutside_forcedentry_onenter(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076033_ctaskopenvehicledoorfromoutside_forcedentry_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076034_ctaskopenvehicledoorfromoutside_opendoor_commononenter(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076035_ctaskopenvehicledoorfromoutside_opendoor_commononupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076036_ctaskopenvehicledoorfromoutside_opendoor_commononexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076037_ctaskopenvehicledoorfromoutside_getalternateclipinfoforped_int_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_076038_ctaskopenvehicledoorfromoutside_setclipplaybackrate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076039_ctaskopenvehicledoorfromoutside_processopendoor_bool_ref_bool_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_076050_ctaskclosevehicledoorfrominside_processclosedoorblend_cmovenetwo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_076051_ctaskclosevehicledoorfrominside_processclosedoor_cmovenetworkhel(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_076054_ctaskentervehicleseat_dtorctaskentervehicleseat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076059_ctaskentervehicleseat_enterseatcommon_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076060_ctaskentervehicleseat_enterseatcommon_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076066_ctaskinvehicleseatshuffle_dtorctaskinvehicleseatshuffle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076071_ctaskinvehicleseatshuffle_ensurelocallyjackedpedsexitvehicle(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076072_ctaskinvehicleseatshuffle_setclipplaybackrate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076073_ctaskinvehicleseatshuffle_setclipsforstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076074_ctaskinvehicleseatshuffle_shoulduse180clip_cturret_const_ptr_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076075_ctaskinvehicleseatshuffle_getshuffleclipsetid_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_076077_ctaskvehiclefsm_processautoclosedoor_cped_const_ref_cvehicle_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_076078_ctaskinvehicleseatshuffle_canshuffleseat_cvehicle_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_076079_ctaskinvehicleseatshuffle_decideshufflestate_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_076080_ctaskinvehicleseatshuffle_startmovenetwork(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076081_ctaskmotioninvehicle_shouldperforminvehiclehairscale_cped_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076088_ctaskinvehicleseatshuffle_canshuffletotargetseat_cmodelseatinfo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_076092_ctaskexitvehicle_startmovenetwork_cped_ref_cvehicle_const_ref_cm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32); }
extern "C" { pub fn f_076093_ctaskexitvehicle_isverticalprobeclear_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_076094_ctaskexitvehicle_dtorctaskexitvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076098_ctaskexitvehicle_shouldabortfrominairevent_rage_aievent_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076101_ctaskexitvehicleseat_getexitclipsetid_cvehicle_const_ptr_int_cve(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_076104_ctaskexitvehicle_pickdoor_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076105_ctaskexitvehicle_streamassetsforshuffle_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076106_ctaskexitvehicle_streamassetsforexit_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076107_ctaskexitvehicle_shuffletoseat_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076108_ctaskexitvehicle_shuffletoseat_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076109_ctaskexitvehicle_exitseat_onenter(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076110_ctaskexitvehicle_exitseat_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076111_ctaskexitvehicle_closedoor_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076112_ctaskexitvehicle_closedoor_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076113_ctaskexitvehicle_closedoor_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076114_ctaskexitvehicle_exitseattoidle_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076115_ctaskexitvehicle_exitseattoidle_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076116_ctaskexitvehicle_climbdown_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076117_ctaskexitvehicle_climbdown_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076118_ctaskexitvehicle_getup_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076119_ctaskexitvehicle_assignvehicletask(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076120_ctaskexitvehicle_requestclips(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076121_ctaskvehiclefsm_pedshouldwarpbecausevehicleatawkwardangle_cvehic(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076122_ctaskexitvehicle_getmovenetworkclipset_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_076123_ctaskexitvehicleseat_checkforwaterunderped_cped_const_ptr_cvehic(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_076124_ctaskexitvehicleseat_ctaskexitvehicleseat_cvehicle_ptr_int_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_076125_ctaskexitvehicle_checkfortransitiontoragdollstate_cped_ref_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_076126_ctaskexitvehicle_shouldragdollduetoreversing_cped_ref_cvehicle_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_076127_ctaskexitvehicle_shouldinterruptduetosequencetask_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076128_ctaskexitvehicle_updateattachoffsetsfromcurrentposition_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076129_ctaskclosevehicledoorfromoutside_ctaskclosevehicledoorfromoutsid(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_076130_ctaskvehiclefsm_findrearentryforentrypoint_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_076131_ctaskexitvehicle_triggerfallevent_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_076132_ctaskvehiclefsm_checkforplayermovement_cped_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076133_ctaskexitvehicle_doesprobehitsomething_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076145_ctaskvehiclefsm_checkforplayerexitinterrupt_cped_const_ref_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_076147_ctaskvehiclefsm_checkforplayerpullingawayfromvehicle_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_076153_ctaskexitvehicleseat_dtorctaskexitvehicleseat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076156_ctaskexitvehicleseat_resetragdolloncollision_cped_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076159_ctaskexitvehicleseat_decideinitialstate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076160_ctaskexitvehicleseat_requestclips_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_076161_ctaskexitvehicleseat_setupragdolloncollision_cped_ref_cvehicle_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_076162_ctaskexitvehicleseat_setclipsforstate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076163_ctaskexitvehicleseat_setclipplaybackrate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076164_ctaskexitvehicleseat_controlledsetpedoutofvehicle_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_076165_ctaskexitvehicleseat_shouldbeconsideredunderwater(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076166_ctaskexitvehicleseat_processopendoor_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_076167_ctaskexitvehicleseat_doesclonepedwanttointerruptexitseatanim_cpe(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076168_ctaskexitvehicleseat_checkforskydive_cped_const_ptr_cvehicle_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_076169_ctaskvehiclefsm_checkifpedwentthroughsomething_cped_ref_cvehicle(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076170_ctaskexitvehicleseat_isstandingonvehicle_cped_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076171_ctaskexitvehicleseat_calculateaimdirection(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_076172_ctaskvehiclefsm_precomputestaticblendweightsfromsignal_ctaskvehi(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_076173_ctaskvehiclefsm_sclipblendparams_dtorsclipblendparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076174_ctaskexitvehicleseat_sendmovesignalsforexittoaim(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076175_cevent_op_new_unsigned_long(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_076176_ctaskexitvehicleseat_checkfordiveintowater_cped_const_ptr_cvehic(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_076177_ctaskreacttobeingaskedtoleavevehicle_ctaskreacttobeingaskedtolea(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_076178_rage_phcollider_applyimpulse_float_vector_4_float_vector_4_int_f(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: i32); }
extern "C" { pub fn f_076179_ctaskexitvehicleseat_processexitseatinterrupt_cped_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076182_ctaskvehiclefsm_getgenericclipblendparams_ctaskvehiclefsm_sclipb(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_076183_ctaskvehiclefsm_computeremaininganimtransform_ctaskvehiclefsm_sc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_076184_ctaskvehiclefsm_transformrelativeoffsetstoworldspace_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_076185_ctaskexitvehicleseat_getgroundfixupheight_bool_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_076186_ctaskvehiclefsm_applygroundandorientationfixup_cvehicle_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32) -> i32; }
extern "C" { pub fn f_076187_ctaskvehiclefsm_applymoverfixup_cped_ref_cvehicle_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_076188_ctaskexitvehicleseat_updatetarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076191_ctaskclosevehicledoorfromoutside_dtorctaskclosevehicledoorfromou(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076198_ctaskreacttobeingjacked_ctaskreacttobeingjacked_cvehicle_ptr_cpe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_076199_ctaskreacttobeingjacked_dtorctaskreacttobeingjacked(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076203_ctaskreacttobeingjacked_hasjackstarted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076205_ctaskreacttobeingaskedtoleavevehicle_dtorctaskreacttobeingaskedt(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076208_ctaskgotocardoorandstandstill_dtorctaskgotocardoorandstandstill(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076211_ctaskmovegotovehicledoor_ctaskmovegotovehicledoor_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: f32, a6: f32, a7: f32, a8: i32, a9: i32) -> i64; }
extern "C" { pub fn f_076212_ctaskmovegotovehicledoor_hasachieveddoor(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076213_ctaskmovegotovehicledoor_calctargetpositions_cped_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_076214_ctaskmovegotovehicledoor_dtorctaskmovegotovehicledoor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076219_ctaskmovegotovehicledoor_adjustposforvehiclespeed_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_076222_ctaskmovegotovehicledoor_choosegotodoormbr_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_076223_ctaskmovegotovehicledoor_warpifpossible(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076224_ctaskmovegotovehicledoor_isalreadyinpositiontoenter_cped_ptr_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076225_ctaskmovegotovehicledoor_hascarmoved_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076226_cvehiclehullcalculator_calculatehullforvehicle_cvehicle_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: f32, a5: f32); }
extern "C" { pub fn f_076227_cvehiclehullai_moveoutside_rage_vector2_ref_rage_vector2_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_076228_cvehiclehullai_moveoutside_rage_vector2_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_076229_cvehiclehullai_calcshortestroutetotarget_cped_const_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i32, a7: f32, a8: i64, a9: f32, a10: i32) -> i32; }
extern "C" { pub fn f_076230_cvehiclehullai_liesinside_rage_vector2_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076231_ctaskmovegotovehicledoor_testissegmentclear_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32, a6: i64, a7: f32) -> i32; }
extern "C" { pub fn f_076233_cvehiclehullai_testisrouteclear_cped_const_ptr_cpointroute_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i64, a6: f32, a7: i32) -> i32; }
extern "C" { pub fn f_076235_ctaskmountthrowprojectile_findmeleenetworkdamageforhitped_cped_p(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_076238_ctaskmountthrowprojectile_updatetarget_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076239_ctaskmountthrowprojectile_isstatevalidforfpsik_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076241_ctaskvehiclegun_needstosmashwindowfromtargetpos_cped_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_076242_ctaskmountthrowprojectile_restartmovenetwork_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_076243_ctaskmountthrowprojectile_updatefpsmovesignals(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076244_ctaskmountthrowprojectile_checkfordrop_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076245_ctaskmountthrowprojectile_updateaiming_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_076246_ctaskmountthrowprojectile_setnextstatefromnetwork(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076247_ctaskmountthrowprojectile_getisbikemoving_cped_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076248_ctaskvehiclegun_clearoutpartialsmashedwindow_cped_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076249_ctaskmountthrowprojectile_throwprojectileimmediately_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076254_ctaskmountthrowprojectile_processcachednetworkdamage_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076257_ctaskmountthrowprojectile_getiscloneindamagewindow_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076259_ctaskvehiclegun_getdoorforped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076260_ctaskmountthrowprojectile_shouldblockhandik_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_076264_ctaskridetrain_dtorctaskridetrain(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076272_ctasktrainbase_ctasktrainbase_ctrain_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_076275_ctasktrainbase_ctasktrainbase_ctasktrainbase_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_076276_ctasktrainbase_dtorctasktrainbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076277_ctasktrainbase_createnavtask_rage_vec3v_const_ref_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_076283_ctasktrytograbvehicledoor_dtorctasktrytograbvehicledoor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076287_ctasktrytograbvehicledoor_setupbalancegrabhelper_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076296_ctaskattachedvehicleanimdebug_ctaskattachedvehicleanimdebug_cveh(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_076297_ctaskattachedvehicleanimdebug_dtorctaskattachedvehicleanimdebug(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076302_ctaskattachedvehicleanimdebug_computeidealendtransformforstate_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_076303_ctaskattachedvehicleanimdebug_getclipblendparamsforstate_int_cta(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_076304_ctaskattachedvehicleanimdebug_computeidealstarttransformforstate(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_076305_ctaskattachedvehicleanimdebug_getcliparrayforstate_int_rage_atar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_076306_ctaskattachedvehicleanimdebug_getclipidforstate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_076309_ctaskattachedvehicleanimdebug_align_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076310_ctaskattachedvehicleanimdebug_opendoor_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076311_ctaskattachedvehicleanimdebug_opendoor_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076312_ctaskattachedvehicleanimdebug_closedoorfrominside_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076313_ctaskattachedvehicleanimdebug_closedoorfromoutside_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076314_ctaskattachedvehicleanimdebug_attachpedforstate_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_076315_ctaskattachedvehicleanimdebug_setupmoveparamsandattachstate_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_076316_ctaskattachedvehicleanimdebug_processdoorangle_cvehicle_ref_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_076318_ctaskvehiclemountedweapon_ctaskvehiclemountedweapon_ctaskvehicle(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i64; }
extern "C" { pub fn f_076319_ctaskvehiclemountedweapon_dtorctaskvehiclemountedweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076322_ctaskvehiclemountedweapon_controlvehicleweapons_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_076327_ctaskvehiclemountedweapon_streamingclips_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076328_ctaskvehiclemountedweapon_idle_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076329_ctaskvehiclemountedweapon_idle_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076330_ctaskvehiclemountedweapon_gettargetinfo_cped_ptr_rage_vector3_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_076331_ctaskvehiclemountedweapon_wantstofire_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076332_ctaskvehiclemountedweapon_canfireattarget_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076333_ctaskvehiclemountedweapon_shouldpreventaimingorfiringduetolookbe(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076334_ctaskvehiclemountedweapon_computesearchtarget_rage_vector3_ref_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076336_ctaskvehiclemountedweapon_evaluateterrainforfiredirection_rage_v(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076337_ctaskvehiclemountedweapon_getvehicletargetingflags_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_076338_ctaskvehiclemountedweapon_findtarget_cvehicle_ptr_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: i64, a8: i64, a9: i64) -> i64; }
extern "C" { pub fn f_076340_ctaskvehiclemountedweapon_doterrainprobe_cvehicle_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_076350_ctaskvehiclegun_dtorctaskvehiclegun(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076353_ctaskvehiclegun_gettargetposition_cped_const_ptr_rage_vector3_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_076355_ctaskvehiclegun_init_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076356_ctaskvehiclegun_smashwindow_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076357_ctaskvehiclegun_waitingtofire_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076358_ctaskvehiclegun_gun_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076359_ctaskvehiclegun_gun_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076360_ctaskvehiclegun_selectclipset_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076361_ctaskvehiclegun_getrestrictedaimsweepheadinganglerads_cped_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076362_ctaskvehiclegun_checkfiringconditions_cped_const_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: i32) -> i32; }
extern "C" { pub fn f_076363_ctaskvehiclegun_processweaponobject_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076365_ctaskmotioninvehicle_getseatoverrideaniminfofromped_cped_const_r(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076378_ctaskvehicleprojectile_gettargetposition_cped_const_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076379_csimpleblendhelper_setphase_ccliphelper_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_076380_ctaskvehicleprojectile_throwprojectileimmediately_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076384_ctaskmotioninvehicle_checkforstill_float_cvehicle_const_ref_cped(ph: *mut Phantom, a0: f32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32) -> i32; }
extern "C" { pub fn f_076385_ctaskmotioninvehicle_checkformovingforwards_cvehicle_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076386_ctaskmotioninvehicle_checkforshunt_cvehicle_ref_cped_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_076387_ctaskmotioninvehicle_checkforreverse_cvehicle_const_ref_cped_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_076388_ctaskmotioninvehicle_checkforstartingengine_cvehicle_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076389_ctaskmotioninvehicle_checkforhotwiring_cvehicle_const_ref_cped_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076390_getsteeringlerpscale(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_076391_ctaskmotioninvehicle_computesteeringsignal_cvehicle_const_ref_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_076393_ctaskmotioninvehicle_dtorctaskmotioninvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076397_ctaskmotioninvehicle_checkcanpedmovetodriversseat_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_076399_ctaskmotioninautomobile_ctaskmotioninautomobile_cvehicle_ptr_cmo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_076400_ctaskmotiononbicycle_ctaskmotiononbicycle_cvehicle_ptr_cmovenetw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_076401_ctaskmotioninvehicle_onbicycle_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076405_ctaskmotioninautomobile_checkifvehicleisdoingburnout_cvehicle_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076410_ctaskmotioninautomobile_willabortputtingonhelmet_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076412_ctaskmotioninautomobile_dtorctaskmotioninautomobile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076413_cbikeleananglehelper_dtorcbikeleananglehelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076417_ctaskmotioninautomobile_canusepovanims_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076418_ctaskmotioninautomobile_checkforshuntfromhighfall_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_076419_ctaskmotioninautomobile_shoulduselowriderleanaltclipset_bool_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_076420_ctaskmotioninautomobile_getdefaultclipsettouse_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076421_ctaskmotioninautomobile_getdefaultclipsetid_ctaskmotioninautomob(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_076422_ctaskmotioninautomobile_shouldusebaseidle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076423_ctaskmotioninautomobile_checkforhelmetvisorswitch_cped_ref_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_076424_ctaskmotioninautomobile_getseatclipsetid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076426_ctaskmotioninautomobile_startmovenetwork_cvehicleseataniminfo_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076427_ctaskmotioninautomobile_updatefirstpersonanimationstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076428_ctaskmotioninautomobile_checkforstartingengine_cvehicle_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_076429_ctaskmotioninautomobile_getclipforstate_int_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_076430_ctaskmotioninautomobile_checkforchangingstation_cvehicle_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_076431_ctaskmotioninautomobile_shouldusesittostilltransition_cvehicle_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076432_ctaskmotioninautomobile_checkifpedwantstomove_cped_const_ref_cve(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_076433_ctaskmotioninautomobile_shouldpanic_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076434_ctaskmotioninautomobile_isclosedoorstatevalid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076435_ctaskmotioninautomobile_checkforhorn_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_076436_ctaskmotioninautomobile_setsitidlemovenetworkstate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076437_ctaskmotioninautomobile_setclipplaybackrate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076438_ctaskmotioninautomobile_pickroadrageanim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076441_ctaskmotioninautomobile_processimpactreactions_cped_ref_cvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_076443_ctaskmotioninautomobile_processdamagefromimpacts_cped_ref_cvehic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_076445_ctaskmotioninautomobile_shoulduselowriderlean_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076446_ctaskmotioninautomobile_compute2dbodyblendsteeringparam_cvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_076447_ctaskmotioninautomobile_shouldbeagitated_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076448_ctaskmotioninautomobile_hasmodifiedlowridersuspension_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076449_ctaskmotioninautomobile_cantransitiontofirstpersonanims_bool_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_076450_ctaskmotioninautomobile_canbeducked_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_076454_ctaskmotioninautomobile_playershouldbeagitated_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076456_ctaskmotiononbicycle_checkforwantstomove_cvehicle_const_ref_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_076457_ctaskmotiononbicycle_wantstodospecialmove_cped_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_076458_ctaskmotiononbicycle_wantstotrackstand_cped_const_ref_cvehicle_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076459_ctaskmotiononbicycle_getsprintresultfromped_cped_const_ref_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> f32; }
extern "C" { pub fn f_076460_ctaskmotiononbicycle_checkforstill_cvehicle_const_ref_cped_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076461_ctaskmotiononbicycle_cantransitiontostill_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076462_ctaskmotiononbicycle_pedwantstostand_cped_const_ref_cvehicle_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_076463_ctaskmotiononbicycle_dtorctaskmotiononbicycle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076469_ctaskmotiononbicycle_setseatclipset_cvehicleseataniminfo_const_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076470_ctaskmotiononbicycle_settaskstate_ctaskmotiononbicycle_eonbicycl(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_076471_ctaskmotiononbicycle_setmovenetworkstates_rage_fwmvrequestid_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_076472_ctaskmotiononbicycle_aremovenetworkstatesvalid_rage_fwmvrequesti(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076473_ctaskmotiononbicycle_checkforrestartingstateduetocameraswitch(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076474_ctaskmotiononbicycle_checkforreversingtransition_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076475_ctaskmotiononbicycle_checkforstandingtransition_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076476_ctaskmotiononbicycle_setclipsforstate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_076477_ctaskmotiononbicyclecontroller_ctaskmotiononbicyclecontroller_cm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_076478_ctaskmotiononbicycle_setblenddurationforstate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_076479_ctaskmotiononbicycle_checkforimpacttransition_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076483_ctaskmotiononbicycle_shouldchangeanimsduetocameraswitch(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076485_ctaskmotiononbicycle_visualizeclip_rage_crclip_const_ref_float_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64); }
extern "C" { pub fn f_076486_ctaskmotiononbicycle_visualizepreplaunchtags_rage_crclip_const_r(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_076487_ctaskmotiononbicycle_visualizetrackstandtags_rage_crclip_const_r(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_076490_ctaskmotiononbicyclecontroller_dtorctaskmotiononbicyclecontrolle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076495_ctaskmotiononbicyclecontroller_computespecialmoveparams_ctaskmot(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_076496_ctaskmotiononbicyclecontroller_wantstofreewheel_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_076497_ctaskmotiononbicyclecontroller_setmovenetworkstates_rage_fwmvreq(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_076498_ctaskmotiononbicyclecontroller_checkforpedaltopreparingtolauncht(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076499_ctaskmotiononbicyclecontroller_findtargetblendphaseforcriticalta(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> f32; }
extern "C" { pub fn f_076500_ctaskmotiononbicyclecontroller_checkfortotrackstandtransition_co(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076501_ctaskmotiononbicyclecontroller_checkforpedalingtransition(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076502_ctaskmotiononbicyclecontroller_checkforpreparingtolaunchtransiti(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076503_ctaskmotiononbicyclecontroller_checkforfixieskidtransition_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076504_ctaskmotiononbicyclecontroller_checkfortrackstandtransition_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076505_ctaskmotiononbicyclecontroller_setclipsforstate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_076506_ctaskmotiononbicyclecontroller_aremovenetworkstatesvalid_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_076507_ctaskmotiononbicyclecontroller_checkforinairtransition_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076514_cbikeleananglehelper_hasstickinput_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076515_cbikeleananglehelper_processleanangledefault(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076516_cvehiclehullcalculator_shouldsubboundbeculled_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_076517_cvehiclehullcalculator_projectboundingboxontozplane_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_076552_cmultipleclipsetrequesthelper_16_dtorcmultipleclipsetrequesthelp(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076562_ctasksmashcarwindow_dtorctasksmashcarwindow(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076598_ctaskaimgun_dtorctaskaimgun(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076660_cthrowprojectilehelper_calculateaimtrajectory_cped_ptr_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_076661_cthrowprojectilehelper_getpedthrowposition_cped_ptr_rage_fwmvtem(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_076662_cthrowprojectilehelper_isplayerpriming_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076663_cthrowprojectilehelper_setweaponslotafterfiring_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076665_ctaskaimandthrowprojectile_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076666_ctaskaimandthrowprojectile_ctaskaimandthrowprojectile_cweapontar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_076667_ctaskaimandthrowprojectile_dtorctaskaimandthrowprojectile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076670_ctaskaimandthrowprojectile_updateaiming_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_076671_ctaskaimandthrowprojectile_processthrowprojectile(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076672_ctaskaimandthrowprojectile_throwprojectilenow_cped_ptr_cweapon_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_076673_ctaskaimandthrowprojectile_isaiming_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076675_cweapontarget_getpositionwithfiringoffsets_cped_const_ptr_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_076678_ctaskaimgun_calculatedesiredpitch_cped_const_ptr_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> f32; }
extern "C" { pub fn f_076679_ctaskaimgun_convertradiantosignal_float_float_float_bool(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_076680_ctaskaimgun_smoothinput_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_076681_ctaskaimandthrowprojectile_handlefpsstateupdate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076682_ctaskaimandthrowprojectile_selectfpsblendtime_cped_ptr_bool_cped(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> f32; }
extern "C" { pub fn f_076683_ctaskaimandthrowprojectile_isfiring_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076684_ctaskaimandthrowprojectile_canplaceprojectile_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076685_ctaskaimandthrowprojectile_setgripclip(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076686_ctaskweaponblocked_isweaponblocked_cped_const_ptr_rage_vector3_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: i32, a8: f32) -> i32; }
extern "C" { pub fn f_076687_ctaskaimandthrowprojectile_holdprimemovenetworkrestart_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_076688_cweapontarget_update_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076701_ctaskthrowprojectile_dtorctaskthrowprojectile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076707_ctaskthrowprojectile_cantriggerturn_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076708_ctaskthrowprojectile_triggerturn_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_076716_ctaskswapweapon_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076721_ctaskswapweapon_createweapon(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076722_ctaskswapweapon_getdrawingweaponinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076723_ctaskswapweapon_getisanimationstreamed_unsigned_int_rage_fwmvtem(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_076724_ctaskswapweapon_startmovenetwork_cweaponinfo_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076725_ctaskswapweapon_getdefaultfilterid_cweaponinfo_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_076726_ctaskswapweapon_getswapweaponclipid_cweaponinfo_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_076727_ctaskswapweapon_getclipplaybackrate_cweaponinfo_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_076728_ctaskswapweapon_setgripclip_cped_ptr_cweaponinfo_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_076735_ctaskaimgunonfoot_postcameraupdate_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_076736_ctaskgun_gettorsoikblendfromweaponflag_cped_const_ref_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> f32; }
extern "C" { pub fn f_076743_ctaskgun_processuseassistedaimingcamera_cped_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_076744_ctaskgun_processuserunandgunaimingcamera_cped_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_076746_ctaskgun_ctaskgun_cweaponcontroller_weaponcontrollertype_ctaskty(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32) -> i64; }
extern "C" { pub fn f_076754_ctaskweaponblocked_stateblockedonenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076755_ctaskweaponblocked_setupfpsmovenetwork(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076767_ctaskaimgun_calculatedesiredyaw_cped_const_ptr_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_076769_ctaskaimgun_ctaskaimgun_cweaponcontroller_weaponcontrollertype_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_076770_ctaskaimgun_calculatefiringvector_cped_ptr_rage_vector3_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_076771_ctaskaimgunvehicledriveby_ishidingguninfirstpersondrivercamera_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_076772_ctaskaimgunvehicledriveby_ispovcameraconstrained_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076773_ctaskaimgun_processprefsm(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076774_ctaskgun_processtorsoik_cped_ref_rage_fwflags_int_const_ref_cwea(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_076776_ctaskaimgun_isweaponblocked_cped_ptr_bool_float_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: f32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_076777_ctaskaimgun_canfireattarget_cped_ptr_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_076778_ctaskaimgun_firegun_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_076779_ctaskaimgunfactory_createtask_ctasktypes_etasktype_cweaponcontro(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32) -> i64; }
extern "C" { pub fn f_076781_ctaskaimgunonfoot_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076786_ctaskgun_cleartoggleaim_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076789_ctaskaimgunonfoot_start_onupdate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076790_ctaskaimgunonfoot_aiming_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076791_ctaskaimgunonfoot_aiming_onupdate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076792_ctaskaimgunonfoot_aiming_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076793_ctaskaimgunonfoot_firing_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076794_ctaskaimgunonfoot_firing_onupdate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076795_ctaskaimgunonfoot_firing_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076796_ctaskaimgunonfoot_aimtoidle_onupdate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076797_ctaskaimgunonfoot_setclothpackageindex_unsigned_char_unsigned_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_076798_ctaskaimgunonfoot_combatroll_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076799_ctaskaimgunonfoot_throwingprojectilefromaiming_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076800_ctaskaimgunonfoot_throwingprojectilefromaiming_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076801_ctaskaimgunonfoot_processsmashglashfromblocked_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076802_ctaskaimgunonfoot_requestaim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076803_ctaskaimgunonfoot_aiming_onprocessmovesignals(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076804_ctaskaimgunonfoot_updateadditiveweights_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_076805_ctaskaimgunonfoot_shouldreloadweapon_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076806_ctaskaimgunonfoot_shouldthrowgrenadefromgunaim_cped_ptr_cweaponi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_076807_ctaskaimgunonfoot_findmotionaimingtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076808_ctaskaimgunonfoot_setmoveboolean_bool_rage_fwmvtemplateid_rage_f(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076809_ctaskaimgunonfoot_canspinup_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076810_ctaskaimgunonfoot_findaimingnetworkhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076811_ctaskaimgunonfoot_setuptofire_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_076812_ctaskaimgunonfoot_firing_onprocessmovesignals(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076813_ctaskaimgunonfoot_choosefiringmedclip_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076814_ctaskaimgunonfoot_triggerfire(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076823_cfiringpattern_readytofire_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076834_ctaskaimgunscripted_start_onupdate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076835_ctaskaimgunscripted_idle_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076836_ctaskaimgunscripted_idle_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076837_ctaskaimgunscripted_intro_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076838_ctaskaimgunscripted_intro_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076839_ctaskaimgunscripted_aim_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076840_ctaskaimgunscripted_aim_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076841_ctaskaimgunscripted_fire_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076842_ctaskaimgunscripted_outro_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076843_ctaskaimgunscripted_outro_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076844_ctaskaimgunscripted_reload_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076845_ctaskaimgunscripted_reload_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076846_ctaskaimgunscripted_reload_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076847_ctaskaimgunscripted_blocked_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076848_ctaskaimgunscripted_blocked_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076849_ctaskaimgunscripted_swapweapon_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076850_ctaskaimgunscripted_swapweapon_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076858_ctaskgun_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076861_ctaskgun_dtorctaskgun(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076866_ctaskgun_shoulddelayafterswap_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076868_ctaskgun_hasvalidweapon_cped_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076869_ctaskgun_updatetarget_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076871_ctaskgun_statereloadonenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076872_ctaskgun_statereloadonexit_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076873_ctaskgun_statebulletreactiononenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076874_ctaskgun_statebulletreactiononexit_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076885_ctaskgun_setupaiminggetupset_cnmblendoutsetlist_ref_cped_ptr_cai(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_076897_ctaskreloadgun_createammoclip(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076898_ctaskreloadgun_deleteheldobject(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076902_ctaskreloadgun_getreloaddata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076903_ctaskreloadgun_startreloadclips_cped_ref_cweaponcomponentreloadd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_076904_ctaskreloadgun_reloadoncleanup_cped_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076905_ctaskreloadgun_computeshouldinterrupt_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076906_ctaskreloadgun_shouldblendout(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076907_ctaskreloadgun_isweaponblocked(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076916_ctaskreloadgun_createheldobject_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_076918_ctaskaimgunvehicledriveby_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076921_ctaskaimgunvehicledriveby_isflippingsomeoneoff_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076927_ctaskaimgunvehicledriveby_start_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076928_ctaskaimgunvehicledriveby_intro_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076929_ctaskaimgunvehicledriveby_intro_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076930_ctaskaimgunvehicledriveby_aim_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076931_ctaskaimgunvehicledriveby_aim_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076932_ctaskaimgunvehicledriveby_aim_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076933_ctaskaimgunvehicledriveby_fire_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076934_ctaskaimgunvehicledriveby_fire_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076935_ctaskaimgunvehicledriveby_fire_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076936_ctaskaimgunvehicledriveby_reload_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076937_ctaskaimgunvehicledriveby_vehiclereload_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076938_ctaskaimgunvehicledriveby_outro_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076939_ctaskaimgunvehicledriveby_finish_onupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076940_ctaskaimgunvehicledriveby_updatefpmoveparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076941_ctaskaimgunvehicledriveby_checkforblocking(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076942_ctaskaimgunvehicledriveby_getblockingfirepedfirstperson_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_076943_ctaskaimgunvehicledriveby_wantstofire_cped_const_ref_cweaponinfo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_076944_ctaskaimgunvehicledriveby_updateunarmeddriveby_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_076945_ctaskaimgunvehicledriveby_shouldblendoutaimoutro(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076948_ctaskaimgunvehicledriveby_canusecloneandaiblocking_cped_const_pt(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076949_ctaskaimgunvehicledriveby_shoulduserestricteddrivebyanimsorblock(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_076952_ctaskaimgunvehicledriveby_updatemoveparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076953_ctaskaimgunvehicledriveby_computeyawsignalfromdesired_float_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_076955_ctaskaimgunvehicledriveby_processblockedikoffset_rage_vector3_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_076956_ctaskaimgunvehicledriveby_canuserestricteddrivebyanims(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_076958_ctaskaimgunvehicledriveby_isflippingsomeoneoff_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076972_cscriptedguntaskmetadatamgr_load(ph: *mut Phantom); }
extern "C" { pub fn f_076978_ctaskbomb_dtorctaskbomb(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_076982_ctaskbomb_stateslideandplant_onenter_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076983_ctaskbomb_stateslideandplant_onupdate_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076984_ctaskbomb_stateslideandplant_onexit_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_076985_ctaskbomb_stateswap_onenter_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077027_ctaskaimgunonfoot_dtorctaskaimgunonfoot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077031_ctaskaimgunscripted_dtorctaskaimgunscripted(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077064_nonflyableareaarray_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_077078_sendstuntevent_stunttype_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_077081_cstatsvehicleusage_clear_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077083_cstatsmgr_findvehiclerecord_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077084_cstatsmgr_canupdatevehiclerecord_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077085_cstatsmgr_startnewvehiclerecords_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077086_cstatsmgr_checkwritevehiclerecords_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077089_cstatsmgr_incrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077090_cstatsmgr_decrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077091_cstatsmgr_setgreater_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077093_cstatsmgr_clearplayingtimestats(ph: *mut Phantom); }
extern "C" { pub fn f_077094_cstatsmgr_endvehiclestats(ph: *mut Phantom); }
extern "C" { pub fn f_077095_cstatsmgr_isplayeractive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077096_cphysical_getdistancetravelled_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_077097_cstatsmgr_updaterecordedstat_cstatsmgr_recordstat_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_077098_cstatsmgr_registervehiclenearmissaccepted(ph: *mut Phantom); }
extern "C" { pub fn f_077099_cstatsmgr_checkvehiclegroundproberesults(ph: *mut Phantom); }
extern "C" { pub fn f_077100_cstatsmgr_startvehiclegroundprobe(ph: *mut Phantom); }
extern "C" { pub fn f_077101_cstatsmgr_checkcrashdamageaccumulator_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077102_cstatsmgr_updatevehiclehydraulics_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077104_cstatsmgr_startgroundprobe_rage_vector3_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077105_cstatsmgr_updatevehiclespins_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077106_cstatsmgr_updatevehicleflips_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077107_cstatsmgr_updatevehicleflipsstunt_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077108_cstatsmgr_updatevehiclerolls_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077109_cstatsmgr_updatevehiclezerowheel_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077110_cstatsmgr_updatevehiclewheelie_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077111_cstatsmgr_updatevehiclereversedriving_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077112_cstatsmgr_updatevehiclewallride_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077113_cstatsmgr_updateaveragespeed(ph: *mut Phantom); }
extern "C" { pub fn f_077114_cstatsmgr_updateflyingvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077115_cstatsmgr_isaboveocean_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077148_get_type_from_name_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077151_statid_char_gethash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077152_statid_char_gethash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077153_statid_char_getstatid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077155_sstatdescription_op_sstatdescription_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077157_sstatdata_setdata_void_ptr_unsigned_int_bool_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_077176_sstatdata_valuetostring_char_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077177_cstatsdatamgr_clear(ph: *mut Phantom); }
extern "C" { pub fn f_077178_cstatsdatamgr_loaddatafiles(ph: *mut Phantom); }
extern "C" { pub fn f_077179_cstatsdatamgr_postload(ph: *mut Phantom); }
extern "C" { pub fn f_077181_cstatsdatamgr_resetallstats_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077182_cstatsdatamgr_onsynchcommunitystats_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_077183_cstatsdatamgr_loaddataxmlfile_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077184_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_sstatda(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_077185_cstatsdatamgr_bank_calculatealldatasizes_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077186_cstatsdatamgr_resetstat_sstatdataptr_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_077188_cstatsdatamgr_endnetworkmatch(ph: *mut Phantom); }
extern "C" { pub fn f_077189_cstatsdatamgr_update(ph: *mut Phantom); }
extern "C" { pub fn f_077190_cstatsdatamgr_resetallonlinecharacterstats_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077191_cstatsdatamgr_getstat_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077192_cstatsdatamgr_getstatptr_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077193_cstatsdatamgr_iskeyvalid_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077198_cstatsdatamgr_setstatiterdata_rage_atinbinarymap_sstatdataptr_st(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_077199_cstatsdatamgr_setstatdata_statid_const_ref_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_077200_cstatsdatamgr_statiterfind_statid_const_ref_rage_atinbinarymap_s(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077201_cstatsdatamgr_getstatdata_statid_const_ref_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_077202_cstatsdatamgr_getisbasetype_statid_const_ref_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077203_cstatsdatamgr_registernewstat_sstatparser_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077221_cstatsdatamgr_bank_creatempwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077222_cstatsdatamgr_bank_addstatstogroup_rage_bkbank_ptr_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077224_cstatsdatamgr_bank_addstatwidgetvalue_rage_bkbank_ptr_sstatdatap(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077325_void_std_2_iterops_std_2_classicalgpolicy_iter_swap_abi_nqn22010(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077326_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_077327_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_077328_get_stat_savetype_name_esavetypes(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077329_statid_set_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077331_statsinterface_getintstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077333_statsinterface_getkeyname_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077334_statsinterface_iskeyvalid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077335_statsinterface_getstattype_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077336_statsinterface_getstatdesc_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077338_statsinterface_getcurrentmultiplayercharaterslot(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077339_statsinterface_cloudfileloadpending_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077340_statsinterface_cloudfileloadfailed_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077341_statsinterface_setstatdata_statid_const_ref_void_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_077342_statsinterface_setstatdata_statid_const_ref_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077343_statsinterface_setstatdata_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077344_statsinterface_setstatdata_statid_const_ref_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077345_statsinterface_setstatdata_statid_const_ref_unsigned_char_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077346_statsinterface_setstatdata_statid_const_ref_unsigned_short_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077347_statsinterface_setstatdata_statid_const_ref_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077348_statsinterface_setstatdata_statid_const_ref_unsigned_long_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_077349_statsinterface_setstatdata_statid_const_ref_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_077350_statsinterface_setallstatstosynched_int_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_077354_statsinterface_getfloatstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_077355_statsinterface_getbooleanstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077356_statsinterface_getuint8stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077357_statsinterface_getuint16stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077358_statsinterface_getuint32stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077359_statsinterface_getuint64stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077360_statsinterface_getint64stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077361_statsinterface_syncprofilestats_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077362_statsinterface_unpackdate_unsigned_long_int_ref_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_077365_statsinterface_unpackpos_unsigned_long_float_ref_float_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_077369_statsinterface_findstatiterator_statid_const_ref_rage_atinbinary(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077372_statsinterface_updatelocalplayerweaponinfostatid_cweaponinfo_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077373_statsinterface_getstatsmodelhashid_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077374_statsinterface_getstatsplayermodelvalid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077375_statsinterface_getstatsplayermodelismultiplayer(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077376_statsinterface_getstatsplayermodelissingleplayer(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077380_statsinterface_clearpedscardata(ph: *mut Phantom); }
extern "C" { pub fn f_077382_statsinterface_getcloudsavegameservice(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077383_statsinterface_getlocalplayerweaponinfostatid_statsinterface_ewe(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_077389_cstatsutils_converttodays_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077390_cstatsutils_converttohours_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077394_cstatsutils_isstatstrackingenabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077395_cstatsutils_enablestatstracking_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077397_cstatsutils_resetstatstracking(ph: *mut Phantom); }
extern "C" { pub fn f_077398_csavetelemetrystats_flushtelemetry_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077399_cstatssavehistory_saverequested_esavetypes(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077403_cstatssavesmgr_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077404_cstatssavesmgr_shutdown_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077405_cstatssavesmgr_signedoffline(ph: *mut Phantom); }
extern "C" { pub fn f_077406_cstatssavesmgr_queueload_estatsloadtype(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077407_cstatssavesmgr_queuesave_estatssavetype_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_077408_cstatssavesmgr_pendingsaverequests_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077409_cstatssavesmgr_getnextpendingsaverequest_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077410_cstatssavesmgr_triggerprofilestatsflush_csavetelemetrystats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077411_cstatssavesmgr_checkforpendingprofilestatsflush(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077412_cstatssavesmgr_getlocalclouddirtyreadvalue_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077414_cstatssavesmgr_save_estatssavetype_unsigned_int_esavetypes_unsig(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_077416_cstatssavesmgr_issaveinprogress_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077430_moneyinterface_getvcwalletbalance_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077431_moneyinterface_getvcbankbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077432_moneyinterface_getevcbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077433_moneyinterface_getpvcbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077435_moneyinterface_getvcdailytransferbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077437_moneyinterface_getusdedailyadditionsbalance(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_077438_moneyinterface_getpxrvalue(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_077442_moneyinterface_incrementstatbankbalance_long_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077443_moneyinterface_decrementstatbankbalance_long_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077444_moneyinterface_incrementstatwalletbalance_long_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077445_moneyinterface_decrementstatwalletbalance_long_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_077449_moneyinterface_savegamecanbuycashproducts(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077450_moneyinterface_isoverdailyusdelimit_float(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_077451_moneyinterface_isoverusdebalancelimit_float(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_077462_ccashproductsdata_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077470_sdialoguecharacters_scharacterdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_077479_ctextfile_get_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077480_ctextfile_getinternal_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077481_ctextfile_get_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_077487_csubtitlemessage_clear_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077488_cmessages_insertnumbersandsubstringsintostring_char_const_ptr_cn(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32); }
extern "C" { pub fn f_077489_cmessages_copycolouredgxtstringintostringatindex_char_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_077490_cpreviousmessages_addbrief_csubtitlemessage_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077495_chelpmessage_clearall(ph: *mut Phantom); }
extern "C" { pub fn f_077496_cmessages_clearmessages(ph: *mut Phantom); }
extern "C" { pub fn f_077497_cmessages_clearallbriefmessages(ph: *mut Phantom); }
extern "C" { pub fn f_077499_ctextfile_unload_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077500_cmessages_clearalldisplayedmessagesthatbelongtothistextblock_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_077501_cpreviousmessages_clearallpreviousbriefsbelongingtothistextblock(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077507_chelpmessage_setmessagetext_int_char_const_ptr_cnumberwithinmess(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_077521_csavinggamemessage_clear(ph: *mut Phantom); }
extern "C" { pub fn f_077523_csavinggamemessage_issavingmessageactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077529_pagedcloudtext_init_rage_atstring_const_ref_bool_unsigned_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_077530_pagedcloudtext_parsefile_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077536_ctextformat_flushfontbuffer(ph: *mut Phantom); }
extern "C" { pub fn f_077541_ctextformat_setupfontparams_gfxdrawtextmanager_textparams_ref_ct(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_077542_ctextformat_parsetoken_char_const_ptr_char_ptr_bool_rage_color32(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: i64, a6: i32, a7: i32); }
extern "C" { pub fn f_077555_ctext_getfullfontlibfilename(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077558_ctextformat_geticonlistformatstring_rage_atfixedarray_ctextforma(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_077565_ctextfile_doestextlabelexist_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077566_ctextfile_doestextlabelexist_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077567_ctextfile_getlanguagefile_char_ptr_unsigned_long_bool_ctextfile(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_077568_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_077569_ctextfile_addextracontenttext_char_const_ptr_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_077571_ctextfile_load_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077572_ctextfile_loadtextintoslot_int_char_const_ptr_bool_ctextfile_elo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_077573_ctextfile_streamintoslots_char_const_ptr_bool_cgxt2header_ptr_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_077584_cachedtextdrawing_scache_getentry_cachedtextdrawing_scache_key_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077585_ctextformat_gettokenidfrompadbutton_rage_iomappersource_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_077586_ctextformat_getdeviceiconsfrominputgroup_rage_atfixedarray_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077587_ctextformat_filliconlistfromtokenizedstring_char_ptr_int_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_077589_ctextformat_getinputbuttons_rage_inputtype_char_ptr_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_077591_ctextformat_getinputgroupbuttons_rage_inputgroup_char_ptr_unsign(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_077592_ctextformat_getdpadicondirection_rage_iosource_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077593_ctextformat_doescharacterexistinfont_int_unsigned_short(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_077605_ctimecycle_applycutscenecameraoverrides_rage_tckeyframeuncompres(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077606_ctimecycle_updateunderwater_rage_tckeyframeuncompressed_ref_unsi(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_077607_ctimecycle_applyboxmodifiers_rage_tckeyframeuncompressed_ref_flo(ph: *mut Phantom, a0: f32, a1: i32, a2: i32); }
extern "C" { pub fn f_077608_ctimecycle_applyinteriormodifiers_rage_tckeyframeuncompressed_re(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_077609_ctimecycle_applycutscenemodifiers_rage_tckeyframeuncompressed_re(ph: *mut Phantom); }
extern "C" { pub fn f_077610_ctimecycle_calcsunlightdir(ph: *mut Phantom); }
extern "C" { pub fn f_077611_ctimecycle_applylodscalemodifiers_rage_tckeyframeuncompressed_re(ph: *mut Phantom); }
extern "C" { pub fn f_077612_ctimecycle_calcambientscale_rage_vec3v_const_ref_rage_fwinterior(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077613_ctimecycle_setinteriorcache_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_077616_ctimecycle_calcdirectionallightandpos_rage_vec3v_ref_float_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32, a7: f32, a8: i64, a9: i64); }
extern "C" { pub fn f_077617_rage_tcmanager_getmodifiername_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077620_ctimecycle_setshaderdata_cvisualeffects_erendermode(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077621_cvisualsettings_get_char_const_ptr_char_const_ptr_float_bool_con(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> f32; }
extern "C" { pub fn f_077622_cvisualsettings_get_char_const_ptr_float_bool_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_077623_ctimecycle_calcmodifierambientscale_int_int_float_float_ref_floa(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_077635_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_077636_cvisualsettings_loadall(ph: *mut Phantom); }
extern "C" { pub fn f_077637_cvisualsettings_get_unsigned_int_float_bool_const(ph: *mut Phantom, a0: i32, a1: f32) -> f32; }
extern "C" { pub fn f_077638_cvisualsettings_getcolor_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077639_cvisualsettings_getvec3v_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077640_cvisualsettings_getvec4v_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077641_configlightsettings_set_cvisualsettings_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077642_configvehiclepositionlightsettings_set_cvisualsettings_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077643_configvehiclewhitelightsettings_set_cvisualsettings_const_ref_ch(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077644_configplaneemergencylightssettings_set_cvisualsettings_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077649_profiledobjectdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_077658_cobjectprofiler_process(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077662_cobjectpositionchecker_getentityguids(ph: *mut Phantom); }
extern "C" { pub fn f_077663_cobjectpositionchecker_setmode_cobjectpositionchecker_etmode(ph: *mut Phantom); }
extern "C" { pub fn f_077675_cperformancestats_tsectorsamples_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077680_csectortools_writecommon_rage_fistream_ptr_int_char_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i64, a8: i64); }
extern "C" { pub fn f_077681_cperformancestats_writesamples_rage_fistream_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077683_cperformancestats_setoutputfolder(ph: *mut Phantom); }
extern "C" { pub fn f_077684_cperformancestats_scanandexportarea(ph: *mut Phantom); }
extern "C" { pub fn f_077690_cstreamingmemusage_scanandexportarea(ph: *mut Phantom); }
extern "C" { pub fn f_077705_smoketests_processwarping_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077713_smoketests_instancevehicle_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077715_smoketests_instanceped_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077731_smoketests_removeobjectanddependencies_rage_strindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077734_rage_atarray_cstreamingmemusage_tpoolobject_0_unsigned_short_gro(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077740_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_077741_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077742_cdriverpersonality_finddriverability_cped_const_ptr_cvehicle_con(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_077743_cdriverpersonality_finddriveraggressiveness_cped_const_ptr_cvehi(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_077747_cdriverpersonality_runsamberlights_cped_ptr_cvehicle_const_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_077751_rage_atvector_cflyingvehicleavoidancemanager_closevehicle_0_unsi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077752_cflyingvehicleavoidancemanager_checkfuturecollisions_bool_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077753_cflyingvehicleavoidancemanager_steeravoidvehicle_cvehicle_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_077754_cflyingvehicleavoidancemanager_steeravoidothers_bool_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077755_cflyingvehicleavoidancemanager_steerdesiredvelocity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077756_cflyingvehicleavoidancemanager_findtargetvelocityavoidingpositio(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32) -> i32; }
extern "C" { pub fn f_077769_cjunctiontemplate_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_077773_cjunctiontemplate_cjunctiontemplate(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077784_cjunction_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077785_cjunction_removetrafficlights(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077787_cjunction_update_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_077788_cjunction_tidyjunction(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077789_cjunction_findentranceindexwithnode_rage_cnodeaddress_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077790_cjunction_calculatedistancetojunction_cvehicle_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_077791_cpathfind_checkpathnodeintegrityforzone_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077792_cpathfind_testalllinks_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077793_cjunction_removevehiclefromslot_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077794_cvehicleintelligence_findspeedmultiplierwithspeedfromnodes_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_077795_cjunction_gettrafficlightcommand_cvehicle_const_ptr_int_bool_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_077796_cjunction_vehiclerouteclasheswithothercaronjunction_int_rage_cno(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_077797_cvehicleintelligence_cachejunctionturndirection(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077799_cvehiclenodelist_getpathnode_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_077800_cvehicleintelligence_getroutesearchhelper_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077801_cvehicleintelligence_getnodelist_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077802_cvehicleintelligence_getfollowroutehelper_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077803_cvehicleintelligence_getactivetask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077804_cjunction_findentryandexitnodes_cvehicle_const_ptr_rage_cnodeadd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_077805_cjunction_addvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077807_cpathfind_findpednodeclosesttocoors_cpathfind_cfindpednodeparams(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077808_cpathfind_findpednodeclosestinregion_rage_cnodeaddress_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_077809_cjunction_deploy_rage_cnodeaddress_bool_cpathfind_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077810_cpathfind_arenodesloadedforarea_float_float_float_float_const(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_077811_cpathfind_findnodeclosesttocoors_rage_vector3_const_ref_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32); }
extern "C" { pub fn f_077812_cpathfind_findnodeclosesttocoors_rage_vector3_const_ref_float_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: f32, a11: i32, a12: i64, a13: i32, a14: i32); }
extern "C" { pub fn f_077813_cpathfind_findnodeslinkindexbetween2nodes_rage_cnodeaddress_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_077821_cjunctions_findjunctionfromnode_rage_cnodeaddress_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077824_cpathfind_forallnodesinarea_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_077826_cpathfind_findnodeclosestinregion_rage_cnodeaddress_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: f32, a7: i32); }
extern "C" { pub fn f_077830_cpathfind_findnodeclosestinregion_rage_cnodeaddress_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32, a8: f32, a9: f32, a10: f32, a11: f32, a12: i32, a13: f32, a14: i64, a15: i32, a16: i32); }
extern "C" { pub fn f_077832_cvehiclenodelist_findlastgoodnode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077833_cvehicleintelligence_cachejunctioninfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077842_cpathfind_switchroadsoffinareaforoneregion_cnodesswitchedonoroff(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_077843_cpathfind_switchoffnodeandneighbours_rage_cpathnode_ptr_rage_cpa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_077845_cpathfind_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077847_cpathfind_findrouteendnodeforgpssearch_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_077848_cpathfind_dopathsearch_rage_vector3_const_ref_rage_cnodeaddress(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: f32, a8: i64, a9: i32, a10: i32, a11: i64, a12: i32, a13: i32, a14: i32, a15: i32, a16: i32, a17: i32, a18: i32, a19: i32, a20: i32, a21: i64, a22: i32, a23: i64, a24: i64, a25: i32, a26: i32); }
extern "C" { pub fn f_077854_cpathfind_calculatefitnessvaluefornode_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_077855_cpathfind_findrouteendnodeforgpssearchasync_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_077857_cpathfind_cleargpssearchslot_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077859_cpathfind_generateroutepointsforgps_rage_cnodeaddress_rage_cnode(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i32, a11: i64, a12: i32, a13: i32, a14: i64, a15: i32, a16: i32, a17: i32) -> f32; }
extern "C" { pub fn f_077860_cpathfind_findnodepairtomatchvehicle_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: f32, a8: i32, a9: i64, a10: i64) -> i32; }
extern "C" { pub fn f_077861_cpathfind_findnodepairtomatchvehicleinregion_int_rage_vec3v_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: f32, a5: i32, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i32, a12: i32); }
extern "C" { pub fn f_077862_cvehiclenodelist_findlaneswithnodes_int_rage_vector3_const_ptr_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_077863_cvehiclenodelist_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077864_cpathfind_findnodeslinkindexbetween2nodes_rage_cpathnode_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_077868_cvehiclenodelist_shiftnodesbyone(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077871_cpathfind_identifydrivableextremes_rage_cnodeaddress_rage_cnodea(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_077873_cpathfind_updatestreaming_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077877_cpathfind_calcdisttoanyconnectinglinks_rage_cpathnode_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_077878_cpathfind_calcdirectionmatchvalue_rage_cpathnode_const_ptr_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_077886_cpathfind_recordnodesclosesttocoors_rage_vector3_const_ref_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_077887_cpathfind_findregionlinkindexbetween2nodes_rage_cnodeaddress_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077888_cpathfind_findlinkbetween2nodes_rage_cpathnode_const_ptr_rage_cp(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_077891_cpathfind_findnodeclosesttonodefavordirection_cpathfind_findnode(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077892_cpathfind_findnodeindirection_cpathfind_findnodeindirectioninput(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077896_cpathfind_switchroadsoffinarea_cnodesswitchedonoroff_const_ref_b(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077903_cvehiclenodelist_getclosestposonpath_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_077904_cpathfind_markregionsforcoors_rage_vector3_const_ref_float_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_077930_cvehicleailod_getdummymode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077949_cvehicleintelligence_invalidatecachednodelist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077952_cvehicleintelligence_updatecarhasreasontobestopped(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077954_cvehicleintelligence_allowcaraidebugdisplayforthisvehicle_cvehic(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077959_cvehicleintelligence_isonhighway_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077967_cvehicleintelligence_updatejustbeengivennewcommand(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077974_ctaskvehiclegotohelicopter_ctaskvehiclegotohelicopter_svehiclemi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i64; }
extern "C" { pub fn f_077975_cvehicleintelligence_createautomobilegototask_svehiclemissionpar(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_077982_cvehicleintelligence_createcruisetask_cvehicle_ref_svehiclemissi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_077983_ctaskvehicleflee_ctaskvehicleflee_svehiclemissionparams_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_077984_cvehicleintelligence_createvehicletaskfornetwork_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077994_ctaskvehiclehover_ctaskvehiclehover(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077995_ctaskvehiclenodriver_ctaskvehiclenodriver_ctaskvehiclenodriver_n(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_078012_ctaskvehiclestop_ctaskvehiclestop_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_078015_ctaskvehiclegotopointwithavoidanceautomobile_gettrailerparentfro(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078024_cvehiclenodelist_cvehiclenodelist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078025_cvehiclenodelist_clearpathnodes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078026_cvehiclenodelist_clearfrom_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078027_cvehiclenodelist_clearlanes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078028_cvehiclenodelist_findnumpathnodes_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078029_cvehiclenodelist_findlinkswithnodes_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078044_ctaskvehiclegotopointwithavoidanceautomobile_makewayforcarwithsi(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078054_ctaskvehiclemissionbase_ctaskvehiclemissionbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078077_ctaskvehicleapproach_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078093_ctaskvehicleattack_isabletofireweapons_cvehicle_ref_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_078106_ctaskvehicleblock_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078114_ctaskvehicleblockcruiseinfront_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078121_ctaskvehicleblockbrakeinfront_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078129_ctaskvehicleblockbackandforth_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078136_ctaskvehicleflee_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078141_ctaskvehicleflee_resetstatics(ph: *mut Phantom); }
extern "C" { pub fn f_078177_ctaskvehiclecruiseboat_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078186_ctaskvehicledeaddriver_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078202_ctaskvehiclefleeboat_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078203_ctaskvehiclefleeboat_ctaskvehiclefleeboat_svehiclemissionparams(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_078209_ctaskvehiclecrash_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078224_ctaskvehiclefleeairborne_computeflightheightaboveterrain_cvehicl(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078225_ctaskvehiclefleeairborne_ctaskvehiclefleeairborne_svehiclemissio(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_078243_ctaskvehiclegotopointwithavoidanceautomobile_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078250_ctaskvehiclegotopointwithavoidanceautomobile_slowcardownforpedss(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: i64, a7: f32, a8: i32); }
extern "C" { pub fn f_078280_ctaskvehiclegotoboat_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078290_ctaskvehiclegotohelicopter_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078302_ctaskvehiclegotoplane_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078316_ctaskvehiclegotosubmarine_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078327_ctaskvehicleheliprotect_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078332_ctaskvehiclelandplane_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078340_ctaskvehiclemissionbase_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078356_ctaskvehicleparknew_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078374_ctaskvehicleplayerdrive_istheplayercontrolinactive_ccontrol_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078406_ctaskvehicleplanechase_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078420_ctaskvehiclepullalongside_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078428_ctaskvehiclepursue_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078440_ctaskvehicleram_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078445_ctaskvehicleshottire_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078451_ctaskvehiclespinout_tunables_tunables(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078786_cvehicle_getnumcontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078800_cwheel_reset_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078831_cvehicle_getreferenceframevelocity_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078865_cvehicle_getdefaultboneposition_int_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_078884_cvehicle_hasrammingscoop_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078888_ccardoor_settargetdooropenratio_float_unsigned_int_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64); }
extern "C" { pub fn f_078890_cvehicle_getattachparentvehicle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078891_ccardoor_getislatched_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_078892_cvehicle_hasramp_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078893_cheli_getiscargobob_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078913_cvehicle_islawenforcementvehiclemodelid_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078918_cvehicle_setcardoorlocks_ecarlockstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078924_cvehicle_getwheelfromid_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_078929_cvehicle_isbusmodelid_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078930_cvehicle_getishandbrakepressed_ccontrol_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_078932_cvehicle_getvelocityincludingreferenceframe_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078933_cvehicle_isupsidedown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078934_cvehicle_isonitsside_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078936_cvehicle_ishornon_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078937_ccardoor_setshutandlatched_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_078939_cvehicle_switchengineon_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_078947_cvehicledamage_getisdriveable_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_078951_cvehicle_triggercaralarm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078953_cvehicle_hasrocketboost(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078954_cvehicle_hasjump(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078955_cvehicle_turnsirenon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_078958_ccomponentreservationmanager_getseatreservationfromseat_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_078959_ccomponentreservation_iscomponentinuse(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078967_cvirtualroad_processrouteconstraint_cvirtualroad_trouteinfo_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_078974_cvehicledamage_getenginehealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_078975_cvehicledamage_getisstuck_int_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_078981_cvehicle_hascontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078990_ccardoor_breakoff_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_078992_ccardoor_breaklatch_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078998_cvehicle_switchengineoff_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079000_cvehicle_istank_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079007_cvehicle_getnumberofpassenger_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079023_cvehicle_getattachedtrailer_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079035_cvehicle_isreversetrike_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079039_cvehicle_isturretseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079040_cvehicle_getspecialflightmoderatio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_079041_cvehicle_rolldownwindow_ehierarchyid_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079046_cvehicle_istrike_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079052_cvehicle_killpedsinvehicle_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079058_cvehicle_trytomakepedsfrompretendoccupants_bool_int_const_ptr_ca(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_079065_cvehicle_getdoorfromid_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079069_cvehicle_smashwindow_ehierarchyid_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_079070_cvehicle_getseatanimationinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079071_cvehicle_isentryindexvalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079090_cvirtualroad_processroadcollision_cvirtualroad_trouteinfo_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_079097_ccardoor_setflag_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079130_cvehicle_getlayoutinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079132_cbike_getbikeleaninputs_float_ref_float_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079134_cvehicle_getdriverseat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079136_cbike_isconsideredstill_rage_vector3_const_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_079157_cvehicle_getisjetski_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079248_ccomponentreservation_iscomponentreservedforped_cped_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079249_ccomponentreservation_reserveuseofcomponent_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079251_ccomponentreservation_setpedstillusingcomponent_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079253_ccomponentreservationmanager_getseatreservation_int_seataccess_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_079254_ccomponentreservationmanager_getdoorreservation_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079256_ccomponentreservationmanager_reservevehiclecomponent_cped_ptr_cd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_079257_ccomponentreservationmanager_unreservevehiclecomponent_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079266_ccardoor_clearflag_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079267_ccardoor_getisintact_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079269_ccardoor_setswingingfree_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079271_ccardoor_getlocalmatrix_cvehicle_const_ptr_rage_matrix34_ref_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_079272_ccardoor_getlocalboundingbox_cvehicle_const_ptr_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_079275_cvehicle_haswindowtosmash_ehierarchyid_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_079283_cvehicle_carpartscanbreakoff_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079335_chandlingdata_gethandlingdata_ehandlingtype(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079347_chandlingdatamgr_gethandlingdata_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_079359_cturret_aimatworldpos_rage_vector3_const_ref_cvehicle_ptr_bool_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_079441_clandinggear_getpublicstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079472_cplane_getverticalflightmodeavaliable_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079513_cvehicle_getisusingscriptautopilot_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079547_cplane_isinverticalflightmode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079550_cplane_isentrypointuseable_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079551_cvehicle_getentryaniminfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079552_cvehiclelayoutinfo_getentrypointinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079581_cvehicle_isindriveablestate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079643_cvehicle_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079675_ctrain_findclosestnodeontrack_rage_vector3_const_ref_int_float_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_079678_ctrain_gettraintrack_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_079679_ctrain_istrainapproachingnode_ctraintracknode_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079683_ctrain_cangetoff_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079705_ctrain_findclosestboardingpoint_rage_vec3v_const_ref_cboardingpo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_079706_cvehicle_getboardingpoints_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079711_ctransmission_disablekerseffect(ph: *mut Phantom); }
extern "C" { pub fn f_079714_cvehicle_hassideshunt_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079729_cvehicle_printskeletondata(ph: *mut Phantom); }
extern "C" { pub fn f_079730_cvehicle_setsuperdummyvelocity_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079746_cvehicle_doesvehiclehaveaconvertibleroofanimation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079753_cvehicle_createvehicleweaponmgr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079758_cvehicle_getconvertibleroofstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079769_cvehicle_getminmaxboundsfromdummyvehiclebound_cvehicle_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_079772_cvehicle_getvehicleorientation_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079781_cvehicle_hasmissioncharsinit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079791_cvehicle_isdriver_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079810_cvehicle_getdoorfromboneindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079811_cvehicle_getseconddoorfromentrypointindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079812_cvehicle_getentryinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079813_cvehicle_getdoorfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079815_cvehicle_getseatanimationinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_079816_cvehicle_getseatinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079817_cvehicle_getseatinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_079818_cvehicle_getpovtuninginfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079820_cvehicle_canseatbeaccessedviaanentrypoint_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079822_cvehicle_hasinfinitefuel_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079826_cvehicle_canpedopenlocks_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079827_cvehicle_canpedentercar_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079828_cvehicle_candoorsbedamaged_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079829_cvehicle_getdoorlockstatefromentrypoint_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079830_cvehicle_getdoorlockstatefromseatindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079831_cvehicle_getdirectentrypointindexforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079836_cvehicle_istaxi_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079839_cvehicle_getaihandlinginfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079843_cvehicle_carhasroof_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079844_cvehicle_hasraisedroof_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079845_cvehicle_ispersonalvehicle(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079848_cvehicle_getclosestboardingpoint_rage_vector3_const_ref_cboardin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_079854_cvehicle_containsped_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079855_cvehicle_canbedriven_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079868_cvehicle_addcommentonvehicleevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079870_cturret_getturretmatrixworld_rage_matrix34_ref_cvehicle_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079872_cvehicle_getseathasweaponsorturrets_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079873_cvehicleweaponmgr_getnumweaponsforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079883_cvehiclelayoutinfo_getseatinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079885_cvehicle_getentryexitpoint_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079886_cvehicle_getnumberentryexitpoints_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079887_cvehicle_isentrypointindexvalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079888_cvehicle_getdoorratiotoconsiderdooropen_cvehicle_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_079889_cvehicle_isentrypointclearforped_cvehicle_const_ref_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_079892_cvehicle_haswaterentry_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079895_cvehicle_rolldownwindows(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079899_cvehicle_getrelativevelocity_cped_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079902_cvehicle_setforwardspeed_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079904_cvehicle_placeonroadadjust_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_079908_cvehicle_getconvertibleroofprogress_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_079909_cvehicle_shouldlowerconvertibleroof_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079910_cvehicle_lowerconvertibleroof_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079911_cvehicle_raiseconvertibleroof_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079912_cvehicle_gettransformationstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079914_cvehicle_transformtocar_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079915_cvehicle_settransformrateforcurrentanimation_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079922_cvehicle_setplayerdrivetask_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079930_cvehicle_getfirstturretseat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079931_cvehicle_getdirectentrypointindexforped_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079932_cvehicleweaponmgr_getfirstturretforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079934_cvehicle_getindirectentrypointindexforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079946_cvehicle_isanexclusivedriverpedoropenseat_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079953_cvehicleweaponmgr_getindexofvehicleweapon_cvehicleweapon_const_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_080008_cvehicledamage_getpetroltankposwld_rage_vector3_ptr_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_080042_getmodelindex_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_080043_updateavailablecarscb(ph: *mut Phantom); }
extern "C" { pub fn f_080044_cvehiclefactory_carnumchangecb(ph: *mut Phantom); }
extern "C" { pub fn f_080048_createcar_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080055_deletecarcb(ph: *mut Phantom); }
extern "C" { pub fn f_080080_cvehicleweaponmgr_getweaponsforseat_int_rage_atarray_cvehiclewea(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_080200_cfixedvehicleweapon_spinup_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080237_cturret_getcurrentheading_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_080238_cturret_iswithinfiringangularthreshold_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_080239_cturret_computelocaldirfromworldpos_rage_vector3_const_ref_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_080240_cturret_computerelativeheadingdelta_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_080251_cvehicleweaponmgr_getweaponindexforturret_cturret_const_ref_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_080254_searchlightinfo_set_cvisualsettings_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080258_cvehicleweaponmgr_getweaponsandturretsforseat_int_rage_atarray_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_080268_cvehicleweaponmgr_getfirstfixedweaponforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080476_cvirtualroad_helpercreaterouteinfofromfollowrouteandcenternode_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_080478_cwheel_setcompressionfromgroundheightlocalandhitpos_float_rage_v(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_080540_cvehiclemetadatamgr_getinvehicleoverrideinfo_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_080541_cvehiclemetadatamgr_getvehicledrivebyinfofromped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080542_cvehiclemetadatamgr_getdrivebyaniminfoforweapon_cped_const_ptr_u(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080543_cvehicledrivebyinfo_getminaimsweepheadingangledegs_cped_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_080544_cvehicledrivebyinfo_getmaxaimsweepheadingangledegs_cped_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_080545_cvehicleentrypointinfo_getwindowid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080546_cvehiclelayoutinfo_getentrypointaniminfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080573_cvehicleextrapointsinfo_getextravehiclepointoftype_cextravehicle(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_080638_canimrateset_sanimratetuningpair_getrate_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_080640_cpovtuninginfo_computerestricteddesiredleanangle_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_080641_cvehiclelayoutinfo_getanimrateset_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080666_cvehiclemetadatamgr_getseataniminfofromped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080669_cvehiclemetadatamgr_canpeddodrivebywithequipedweapon_cped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080708_cvehicledrivebyaniminfo_getfirstpersonclipset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_080711_cvehicledrivebyinfo_getminrestrictedaimsweepheadingangledegs_cpe(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_080712_cvehicledrivebyinfo_getmaxrestrictedaimsweepheadingangledegs_cpe(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_080713_cvehicleseataniminfo_findanim_cvehicleseataniminfo_eseatanimtype(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_080715_cinvehicleoverrideinfo_findseatoverrideaniminfofromseataniminfo(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_080866_cvfxhelper_getmatrixfromboneindex_rage_mat34v_ref_bool_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_080876_cvfxhelper_getlocalentitypos_centity_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080877_cvfxhelper_getlocalentitydir_centity_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_080879_cvfxhelper_getinterpvalue_float_float_float_bool(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_080884_cvfxhelper_getwaterz_rage_vec3v_const_ref_float_ref_rage_fwentit(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_080889_cvfxhelper_isplayerinaccuratemodewithscope(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_081205_cclouds_requestcloudtransition_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_081452_cfiremanager_startpedfire_cped_ptr_cped_ptr_int_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: f32, a9: i32, a10: i32, a11: i32) -> i64; }
extern "C" { pub fn f_081454_cfiremanager_startvehiclefire_cvehicle_ptr_int_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32, a7: f32, a8: i64, a9: i64, a10: i32); }
extern "C" { pub fn f_081520_cwatercannons_registercannon_cvehicle_ptr_unsigned_long_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_081544_puddlepass_getsettings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081746_cvehicleglassmanager_storecollision_vfxcollinfo_s_const_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081872_cvfxvehicle_updateptfxvehicleweaponcharge_cvehicle_ptr_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_082026_cweapon_cweapon_unsigned_int_int_cdynamicentity_ptr_bool_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_082028_cexplosionmanager_addexplosion_cexplosionmanager_cexplosionargs(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082029_cweapon_dtorcweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082030_cpedequippedweapon_getprojectileordnance(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082032_cweapon_spinup_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082045_cairdefencemanager_ispositionindefencezone_rage_vec3v_unsigned_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082062_cweapondamage_doweaponimpact_cweapon_ptr_centity_ptr_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i64, a6: i32, a7: i32, a8: i64, a9: i64, a10: i32, a11: i64, a12: f32, a13: f32, a14: f32, a15: i32, a16: i32, a17: i64, a18: i64, a19: i32) -> f32; }
extern "C" { pub fn f_082063_cweapon_sendfiremessage_centity_ptr_rage_vector3_const_ref_world(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i64, a11: f32, a12: f32, a13: f32, a14: i64); }
extern "C" { pub fn f_082064_cpedweaponmanager_getequippedvehicleweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082065_cpedweaponmanager_getequippedweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082069_cweapon_getmuzzleposition_rage_matrix34_const_ref_rage_vector3_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082070_cweaponinfo_getammoinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082071_cpedweaponmanager_getequippedweapon_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082144_cfiringpattern_setfiringpattern_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082146_cfiringpattern_processfired(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082147_cfiringpattern_processburstfinished(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082159_cweapon_doreload_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082160_cpedequippedweapon_createprojectileordnance(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082166_cweapon_updateammoafterfiring_centity_ptr_cweaponinfo_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_082170_cprojectilemanager_createprojectile_unsigned_int_unsigned_int_ce(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: f32, a5: i32, a6: i32, a7: i64, a8: i64, a9: i32, a10: i32, a11: i32) -> i64; }
extern "C" { pub fn f_082172_cweapon_getmuzzlematrix_rage_matrix34_const_ref_rage_matrix34_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082173_cweapon_process_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_082175_cweaponinfo_getappropriateweaponclipsetid_cped_const_ptr_fpsstre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_082178_cweapon_getcooktimeleft_cped_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> f32; }
extern "C" { pub fn f_082179_cpedweaponmanager_destroyequippedweaponobject_bool_char_const_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_082181_cweapon_processgunfeed_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082182_cweapon_gettimebeforenextshot_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_082183_cweapon_calcfirevecfrompos_centity_const_ptr_rage_matrix34_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_082184_cweapon_calcfirevecfromaimcamera_centity_const_ptr_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_082185_cweapon_calcfirevecfromgunorientation_centity_const_ptr_rage_mat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_082187_cweapon_getcanreload_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082189_cweapon_getneedstoreload_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082190_cweapon_startreload_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082191_cweapon_startanim_rage_fwmvtemplateid_rage_fwmvclipsetidtype_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_082192_cweapon_stopanim_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_082200_cweaponanimationsmanager_getscopeweaponclipset_cped_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_082201_cweaponinfo_getweaponclipsetid_cped_const_ref_fpsstreamingflags(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_082202_cweapon_receivefiremessage_centity_ptr_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i64, a14: i32) -> i32; }
extern "C" { pub fn f_082205_cweapon_createfpsweaponnetworkplayer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082206_cweaponinfo_getweaponclipsetid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082207_cpedequippedweapon_attachobjects_cphysical_ptr_cphysical_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_082208_cweapon_setweaponidleflag_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082210_cweapon_stoppedfiring(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082211_cweaponinfo_getprojectilefusetime_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_082215_cpedweaponmanager_dropweapon_unsigned_int_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_082223_cpedinventory_removeweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082252_cpedweaponmanager_getbestweaponinfo_bool_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_082263_cweaponmanager_getweaponunarmed_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082277_cweaponboneid_getboneindex_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082282_cweaponcomponentflashlight_transferactiveness_cweaponcomponentfl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082372_cweaponcomponentvariantmodelinfo_getextracomponentmodel_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082424_cweaponanimationsmanager_getweaponanimationssetforped_cped_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_082502_cweaponinfo_getpedmotionclipsetid_cped_const_ref_fpsstreamingfla(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_082508_cweaponinfo_getalternativeclipsetwhenblockedid_cped_const_ref_fp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082509_cweaponinfo_getswapweapondata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082513_cweaponinfo_getappropriatefpstransitionclipsetid_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_082514_cweaponinfo_getblindfireratemodifier_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_082515_cweaponinfo_getfireratemodifier_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_082516_cweaponinfo_getwantingtoshootfireratemodifier_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_082519_cweaponinfo_getprojectilecanbeplaced_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082520_cweaponinfo_canbeusedasdriveby_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_082525_cweaponinfo_getappropriatereloadweaponclipsetid_cped_const_ptr_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082526_cweaponinfo_getappropriateswapweaponclipsetid_cped_const_ptr_fps(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082555_cpedinventoryloadoutmanager_setloadout_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082556_cweaponswapdata_getswapclipidforweapon_int_rage_fwmvtemplateid_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_082557_cpedequippedweapon_releaseweaponanims(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082562_cpedequippedweapon_attachobject_cpedequippedweapon_eattachpoint(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082566_cpedequippedweapon_releaseweaponcomponent_cobject_ptr_cweaponcom(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082567_cpedequippedweapon_destroyobject_cobject_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082568_cpedequippedweapon_createweaponcomponent_unsigned_int_cobject_pt(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082569_cpedequippedweapon_destroyweaponcomponent_cobject_ptr_cweaponcom(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082572_cpedinventory_addweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_082590_cpedinventory_getisstreamedin_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082594_cpedweaponmanager_equipweapon_unsigned_int_int_bool_bool_cpedequ(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_082597_cpedweaponselector_getisweaponvalid_cped_const_ptr_cweaponinfo_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_082599_cpedweaponselector_selectweapon_cped_ptr_epedinventoryselectopti(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_082600_cpedweaponmanager_createequippedweaponobject_cpedequippedweapon(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082635_cpedweaponmanager_getrequiresweaponswitch_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082640_cpedweaponselector_getbestpedweapon_cped_const_ptr_rage_atarray(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_082644_cpedweaponmanager_equipbestweapon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_082645_cpedweaponmanager_equipbestmeleeweapon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082647_cpedweaponmanager_getbestweaponinfobygroup_unsigned_int_bool_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_082649_cpedweaponmanager_backupequippedweapon_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082652_cpedweaponmanager_getisarmedmelee_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082653_cpedweaponmanager_getisarmedgun_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082654_cpedweaponmanager_getisarmedgunorcanbefiredlikegun_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082655_cpedweaponmanager_getisarmed1handed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082656_cpedweaponmanager_getisarmed2handed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082663_cammoitemrepository_getammo_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082664_cammoitemrepository_getisoutofammo_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082666_cammoitemrepository_getisusinginfiniteammo_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082671_cweaponitem_getkeyfromhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082693_cprojectile_shouldsticktoentity_centity_const_ptr_centity_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_082698_cprojectilemanager_fireorplacependingprojectile_cped_ptr_int_cpr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_082703_cprojectile_initlifetimevalues_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_082707_cprojectile_sticktoentity_centity_ptr_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_082713_cprojectile_findaveragedstickysurfacenormal_centity_const_ptr_ce(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_082724_cprojectilemanager_explodeprojectiles_centity_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082831_cnetworktelemetry_flushtelemetry_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082835_networkinterface_islocalplayersignedin(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082859_cnetwork_usenetworkheap_networkmemorycategory(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082860_cnetwork_stopusingnetworkheap(ph: *mut Phantom); }
extern "C" { pub fn f_082909_cnetworktelemetry_appendmetric_rage_rlmetric_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082938_networkinterface_islocalplayeronline_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082939_networkinterface_hasvalidroscredentials_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083008_networkinterface_iscloudavailable_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083009_networkinterface_isonlineros(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083011_networkinterface_hasvalidrockstarid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083013_networkinterface_getactivegamerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083015_clivemanager_checkonlineprivileges_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083028_networkinterface_isinspectatormode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083038_networkinterface_isinfreemode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083068_networkinterface_areplayersindifferenttutorialsessions_cnetgamep(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083109_networkinterface_forcesynchronisation_cdynamicentity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083110_networkinterface_ispedinvehicleonownermachine_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083111_networkinterface_isvehiclelockedforplayer_cvehicle_const_ptr_cpe(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083113_networkinterface_isinvalidvehiclefordriving_cped_const_ptr_cvehi(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083115_networkinterface_ignorevehicleupdates_cped_ptr_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083118_networkinterface_forcetaskstateupdate_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083121_networkinterface_forcehighupdatelevel_cphysical_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083125_networkinterface_getlastposreceivedovernetwork_cdynamicentity_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083126_networkinterface_getlastvelreceivedovernetwork_cdynamicentity_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083127_networkinterface_getpredictedvelocity_cdynamicentity_const_ptr_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_083128_networkinterface_getpredictedaccelerationvector_cdynamicentity_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083130_networkinterface_updateheadingblending_cped_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083133_networkinterface_overridemoveblendratiosthisframe_cped_ref_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083134_networkinterface_gostraighttotargetposition_cdynamicentity_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083135_networkinterface_overridenetworkblenderfortime_cphysical_const_p(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083136_networkinterface_isinsingleplayerprivatesession(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083157_networkinterface_delaymigrationfortime_cphysical_const_ptr_unsig(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083162_networkinterface_isdamagedisabledinmp_centity_const_ref_centity(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083167_networkinterface_areinteractionsdisabledinmp_centity_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083168_networkinterface_arebulletsimpactsdisabledinmp_centity_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083188_networkinterface_isremoteplayerinfirstpersonmode_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_083189_networkinterface_usefirstpersonidlemovementforremoteplayer_cped(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083266_csyncedped_getped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083350_cloudlistener_cloudlistener(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083383_ccloudmanager_requestgettitlefile_char_const_ptr_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_083493_ctunables_access_unsigned_int_bool_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083494_ctunables_access_unsigned_int_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083495_ctunables_access_unsigned_int_float_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083496_ctunables_tryaccess_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083497_ctunables_tryaccess_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083829_cnetworkpendingprojectiles_getprojectile_cped_ptr_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_083831_serialiseanimation_rage_csyncdatabase_ref_rage_fwmvtemplateid_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_083836_serialisetasktype_rage_csyncdatabase_ref_unsigned_int_ref_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_083838_serialisevehicleenterexitflags_rage_csyncdatabase_ref_cvehicleen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083857_csyncedped_getpedid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083859_csyncedped_setpedid_unsigned_short_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083898_csyncedentity_csyncedentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083899_csyncedentity_csyncedentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083902_csyncedentity_getentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083903_csyncedentity_getentityid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083904_csyncedentity_setentityid_unsigned_short_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083906_csyncedped_csyncedped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083907_csyncedvehicle_csyncedvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083908_csyncedvehicle_getvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083909_csyncedvehicle_getvehicleid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083910_csyncedvehicle_setvehicleid_unsigned_short_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_084169_cprofilestats_resetgroup_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084243_metricplayercoords_metricplayercoords(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_084318_cnetworktelemetry_playervehicledistance_unsigned_int_float_unsig(ph: *mut Phantom, a0: i32, a1: f32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_084410_cprofilestatsrecords_resetrecords_unsigned_int_rage_rlprofilesta(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_084411_cprofilestatsread_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084412_cprofilestatsread_reset_unsigned_int_int_ptr_unsigned_int_cprofi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_084414_cprofilestatsreadsessionstats_endstatsread(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084415_cprofilestatsreadsessionstats_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_084416_cprofilestatsreadsessionstats_readstats_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084419_cprofilestats_synchronize_bool_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_084420_cprofilestats_setsynchronized_cprofilestats_esynctype_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084421_cprofilestats_flush_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_084422_cprofilestats_clearsynchronized_cprofilestats_esynctype(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_084423_cprofilestats_synchronizedfailed_cprofilestats_esynctype_int_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_084424_cprofilestats_synchronized_cprofilestats_esynctype_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084425_cprofilestats_cansynchronize_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084426_cprofilestats_synchronizepending_cprofilestats_esynctype_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_084427_cprofilestats_forcempprofilestatsgetfromcloud(ph: *mut Phantom); }
extern "C" { pub fn f_084629_cloudlistener_dtorcloudlistener_138661(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086108_cnetworkcommunitystats_loadcommunitystatsdata(ph: *mut Phantom); }
extern "C" { pub fn f_086314_networkgametransactions_playerbalance_serialize_rage_rsonwriter(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086763_cnetobjgame_getplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087122_cnetobjped_setclonepedoutofvehicle_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087123_cnetobjped_migrateswithvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087125_cnetobjped_getpedstargetvehicle_int_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087128_cnetobjped_getweapondata_unsigned_int_ref_bool_ref_bool_ref_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_087140_cnetobjped_updatecachedweaponinfo_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087147_cnetobjped_cacheweaponinfo(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087150_cnetobjped_setweapondata_unsigned_int_bool_bool_unsigned_char_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_087319_cnetobjped_updatenonplayerclonetasktarget_cped_const_ptr_caitarg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_087321_cnetobjped_takecontroloforphanedmovenetworktasksubnetwork_rage_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_087535_cnetobjplayer_setrespawninvincibilitytimer_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087546_cnetobjplayer_updatetargettedentityposition(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087587_cnetobjplayer_targetcloninginformation_setaimtarget_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_087645_cnetobjplayer_setaimtarget_centity_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087752_cnetobjvehicle_islockedforplayer_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_088039_csyncedpedvehicleentryscriptconfig_getforcingseatindexforthisveh(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_088040_csyncedpedvehicleentryscriptconfig_hasaforcingflagsetforthisvehi(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_088041_csyncedpedvehicleentryscriptconfig_hasarearforcingflagsetforthis(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090710_asinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090714_atanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090717_atof(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090720_atol(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090721_bsearch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090755_exp2f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090757_expf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090768_fmod(ph: *mut Phantom, a0: f64, a1: f64) -> f64; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090814_memcmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090817_gmtime_r(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090818_modff(ph: *mut Phantom, a0: f32, a1: i64) -> f32; }
extern "C" { pub fn f_090826_powf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
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
extern "C" { pub fn f_090926_strcasecmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090927_strcat(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090930_strcmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090931_strcpy(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090939_strlen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090940_strncasecmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090942_strncmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090947_strstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090956_strtok(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090965_tanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090970_tolower(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_091002_emscripten_builtin_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091023_multi3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
