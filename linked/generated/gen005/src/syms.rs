//! Callee declarations for crate gen005.
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
extern "C" { pub fn f_000128_rage_sysipccreatethread_void_ptr_void_ptr_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_000131_rage_sysipcwaitthreadexit_rage_sysipcthreadid_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000132_rage_sysipcgetprocessorcount(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000134_rage_sysipcyield_int(ph: *mut Phantom); }
extern "C" { pub fn f_000135_rage_sysipcsleep_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000174_rage_fidevice_getlocalsystemtime_rage_fidevice_systemtime_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000644_rage_athashnextsize_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000647_rage_atreferencecounter_atreferencecounter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000654_rage_atstring_set_char_const_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_000655_rage_atstring_lowercase(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000656_rage_atstring_uppercase(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000657_rage_atstring_endswith_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000658_rage_atstring_startswith_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000660_rage_atstring_indexof_char_const_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000661_rage_atstring_lastindexof_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_000662_rage_atstring_lastindexof_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000663_rage_atstring_replace_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000665_rage_atstring_op_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000675_rage_functorbase_functorbase_void_const_ptr_void_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_000680_rage_athashstringnamespacesupport_computehash_rage_athashstringn(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_000681_rage_athashstringnamespacesupport_addstring_rage_athashstringnam(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_000684_rage_athashstringnamespacesupport_trygetstring_rage_athashstring(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_000696_rage_bkwidget_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000703_rage_bkbank_unsetcurrentgroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000704_rage_bkbank_pushgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000705_rage_bkgroup_addgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000707_rage_bkbank_popgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000779_rage_bkmanager_createbank_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000782_rage_bkmanager_findbank_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000783_rage_bkmanager_findwidget_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000785_rage_bkmanager_destroybank_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000786_rage_bkmanager_isbankthread(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000796_rage_bkwidget_findchild_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000806_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000843_rage_bkcombo_setstring_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_000844_rage_bkcombo_getstring_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_000851_char_ptr_rage_formatf_char_8ul_char_ref_8ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000866_rage_bkgroup_addseparator_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000867_rage_bkgroup_addtitle_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000868_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
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
extern "C" { pub fn f_000894_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000898_rage_bkgroup_addslider_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000902_rage_bkgroup_addslider_char_const_ptr_unsigned_short_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000906_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000907_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int_rage_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000916_rage_bkgroup_addcombo_char_const_ptr_unsigned_char_ptr_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_000918_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000919_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000920_rage_bkgroup_addcombo_char_const_ptr_unsigned_char_ptr_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000990_char_ptr_rage_formatf_char_512ul_char_ref_512ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000991_char_ptr_rage_safecatf_char_512ul_char_ref_512ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001062_rage_matrix34_inverse_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001116_char_ptr_rage_formatf_char_32ul_char_ref_32ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001117_rage_datcallback_datcallback(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001119_rage_datcallback_datcallback_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001176_rage_datresourceinfo_generatemap_rage_datresourcemap_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
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
extern "C" { pub fn f_001537_rage_fistream_size(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001551_rage_fprintf_rage_fistream_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001559_rage_diagcontextmessage_diagcontextmessage_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001563_rage_diagcontextmessage_pop(ph: *mut Phantom); }
extern "C" { pub fn f_001565_printf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001568_rage_atmapentry_unsigned_int_wchar_t_ptr_const_ref_rage_atmap_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001569_rage_atmap_unsigned_int_wchar_t_ptr_rage_atmaphashfn_unsigned_in(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001577_rage_fistream_open_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001578_rage_fistream_seek_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001582_rage_fiassetmanager_createleadingpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001622_rage_fistream_read_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001627_rage_fiassetmanager_exists_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001647_rage_fistream_tell(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001768_rage_fidevicerelative_init_char_const_ptr_bool_rage_fidevice_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_001769_rage_fidevicerelative_mountas_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001798_rage_fidevicerelative_getdebugname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001807_rage_fidevice_unmount_rage_fidevice_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002027_rage_invsqrtf_fast_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_002082_rage_pgbase_regeneratemap_rage_datresourcemap_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002088_rage_datresource_datresource_rage_datresourcemap_const_ref_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002090_rage_datresource_dtordatresource(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002093_rage_pgbase_haspagemap_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002111_rage_parmanager_parmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002112_rage_parmanager_initialize_rage_parsettings_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002129_rage_atmapentry_rage_atnonfinalhashstring_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002130_rage_atmap_rage_atnonfinalhashstring_rage_atnonfinalhashstring_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002131_rage_atnonfinalhashstring_atnonfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002147_rage_parmanager_loadtree_char_const_ptr_char_const_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002150_rage_parmanager_savetree_char_const_ptr_char_const_ptr_rage_part(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_002158_rage_parinstancevisitor_visittoplevelstructure_rage_parparsables(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002160_rage_parstructure_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002161_rage_parstructure_issubclassof_rage_parstructure_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_002175_rage_partree_dtorpartree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002188_rage_partreenode_appendaschildof_rage_partreenode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002189_rage_partreenode_createstdleaf_char_const_ptr_double_bool(ph: *mut Phantom, a0: i64, a1: f64, a2: i32) -> i64; }
extern "C" { pub fn f_002193_rage_partreenode_createstdleaf_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002199_rage_partreenode_createstdleaf_char_const_ptr_char_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002210_rage_partreenode_findvaluefrompath_char_const_ptr_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002215_rage_partreenode_findnumchildren_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002296_rage_atmap_unsigned_int_rage_atarray_rage_psorscbuilderinstances(ph: *mut Phantom, a0: i64); }
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
extern "C" { pub fn f_002447_rage_parinstancevisitor_parinstancevisitor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002448_int_rage_formatf_n_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002581_rage_parelement_setname_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002593_rage_parsettings_createstandardsettings(ph: *mut Phantom); }
extern "C" { pub fn f_002718_strupr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002719_strlwr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002739_rage_atpartialstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002743_rage_atdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002746_rage_stristr_char_ptr_char_const_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002763_rage_utf8removeinvalidsurrogates_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002766_crc32(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002783_lz4_compress(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002784_lz4_decompress_fast(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002806_rage_sysservice_adddelegate_rage_atdelegator1_void_rage_sysservi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002808_rage_atdelegator1_void_rage_sysserviceevent_ptr_removedelegate_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002812_rage_sysnamedpipe_sysnamedpipe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002817_rage_sysnamedpipe_write_void_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002853_rage_sysstack_capturestacktrace_unsigned_long_ptr_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_002854_rage_sysstack_printregisteredbacktrace_unsigned_short_void_ptr_u(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002862_rage_systhreadtype_addcurrentthreadtype_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_002863_rage_sysperformancetimer_sysperformancetimer_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002865_rage_sysperformancetimer_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002866_rage_sysperformancetimer_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002868_rage_sysperformancetimer_getelapsedtimems(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_002878_rage_atmap_unsigned_int_rage_sysstackcollector_tagentry_rage_atm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002881_unsigned_int_ref_rage_atmap_unsigned_int_unsigned_int_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_002934_rage_sysmemvirtualallocate_unsigned_long_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002939_rage_sysmemmanager_sysmemmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002945_rage_sysmempoolallocator_safeallocate_unsigned_long_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002947_rage_sysmempoolallocator_safefree_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002948_rage_sysmempoolallocator_free_void_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002950_rage_sysmemsimpleallocator_sysmemsimpleallocator_void_ptr_int_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_003097_char_ptr_rage_formatf_char_16ul_char_ref_16ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003144_rage_geomsegments_colliderayplane_rage_vector3_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_003159_rage_matrix34_fromquaternion_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003160_rage_matrix34_toquaternion_rage_quaternion_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003161_rage_quaternion_frommatrix34_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003163_rage_quaternion_slerp_float_rage_quaternion_const_ref_rage_quate(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_003166_rage_matrix44_inverse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003179_rage_atpoolbase_init_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003182_rage_atpoolbase_releasemem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003209_rage_imp_quatfrommat33v_imp33_float_vector_4_float_vector_4_floa(ph: *mut Phantom, a0: u128, a1: u128, a2: u128) -> u128; }
extern "C" { pub fn f_003344_rage_crframefilterbonebasic_dtorcrframefilterbonebasic(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003835_rage_fwanimdirector_getmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003895_rage_fwanimmanager_getclipifexistsbyname_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003896_rage_fwanimmanager_getclipifexistsbydictindex_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_003905_rage_fwanimmanager_findslotfromhashkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004619_rage_fwarchetype_getdrawable_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004620_rage_fwarchetype_getfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004631_rage_fwarchetypemanager_getarchetypename_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004632_rage_fwarchetypemanager_lookupmodelid_rage_fwarchetype_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004633_rage_fwarchetypemanager_getarchetype_rage_atnonfinalhashstring_u(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004634_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004636_rage_fwarchetypemanager_getarchetypeindexfromhashkey_unsigned_in(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004681_rage_fwentity_waitforanyactiveanimupdatetocomplete_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004684_rage_fwentity_isentityaparentattachment_rage_fwentity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004693_rage_fwsceneupdate_isinsceneupdate_rage_fwentity_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004694_rage_fwsceneupdate_addtosceneupdate_rage_fwentity_ref_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004784_rage_fwpickermanager_addentitytopickerresults_rage_fwentity_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004792_rage_fwpickermanager_getselectedentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004816_rage_fwvectormap_drawline_rage_vector3_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_004822_rage_fwvectormap_drawrectaxisaligned_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_005032_rage_fwlanguagepack_getdateformattype_rage_syslanguage(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005035_rage_fwlanguagepack_getmonthstring_char_ref_16_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_005049_rage_fwtextutil_truncatefileextension_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005050_rage_fwtextutil_truncatefilename_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005051_rage_fwtextutil_hasfileextension_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005053_rage_fwtextutil_getbytecount_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005093_rage_clothinstance_gettype_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005114_rage_clothinstance_settypemodifiers_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005170_rage_ropedatamanager_unloadropetexures(ph: *mut Phantom); }
extern "C" { pub fn f_005171_rage_ropedatamanager_loadropetexures(ph: *mut Phantom); }
extern "C" { pub fn f_005196_rage_ropeinstance_setphysinstflags_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005199_rage_ropeinstance_disablecollision_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005203_rage_ropeinstance_attachobjectstoconstraintarray_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32) -> i64; }
extern "C" { pub fn f_005206_rage_ropeinstance_attachobjects_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: i32, a11: i64, a12: i64, a13: i64, a14: i64); }
extern "C" { pub fn f_005207_rage_ropeinstance_attachtoobject_rage_vec3v_const_ref_rage_phins(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_005208_rage_ropeinstance_detachfromobject_rage_phinst_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005209_rage_ropeinstance_setnewuniqueid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005212_rage_ropeinstance_load_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005213_rage_ropemanager_remove_rage_ropeinstance_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005215_rage_ropemanager_updatephysics_float_rage_enropeupdateorder(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_005219_rage_ropeinstance_isattached_rage_phinst_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005220_rage_ropemanager_findrope_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005223_rage_ropemanager_removerope_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005225_rage_ropemanager_add_rage_vec3v_const_ref_rage_vec3v_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32, a11: f32, a12: i32, a13: i32) -> i64; }
extern "C" { pub fn f_005315_rage_fwrenderthreadinterface_flush_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005384_rage_fwmapdatastore_getenablediplgroups_rage_atarray_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005395_rage_fwmapdatastore_saferemove_rage_strlocalindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005398_rage_fwimapgroupmgr_completeswap_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005399_rage_fwimapgroupmgr_abandonswap_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005461_rage_fwboxstreamer_hasloadedaboutpos_rage_vec3v_const_ref_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_005553_rage_fwimapgroupmgr_createnewswappersistent_rage_atnonfinalhashs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_005557_rage_fwimapgroupmgr_setispersistent_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005561_sprintf_s_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005580_rage_fwmapdatastore_saferemoveall_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005633_rage_fwmaptypesstore_verifyitypfiles(ph: *mut Phantom); }
extern "C" { pub fn f_005646_rage_atmap_rage_strindex_rage_fwpsostoreplacementcookies_cookie(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005678_rage_fwstaticboundsstore_getdummybounddata_rage_strlocalindex_sh(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_005681_rage_fwstaticboundsstore_getslotdata_int_rage_atsnode_rage_fwbou(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_005730_rage_fwinteriorlocation_makeinvalid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005731_rage_fwinteriorlocation_setroomindex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005732_rage_fwinteriorlocation_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005733_rage_fwinteriorlocation_getroomindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005734_rage_fwinteriorlocation_setportalindex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005799_rage_fwsearch_finalize(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005800_rage_fwsearch_executecallbackonresult_bool_ptr_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_005802_rage_fwsearch_dtorfwsearch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005917_rage_fwrefawarebaseimpl_rage_datbase_dtorfwrefawarebaseimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005946_rage_aitasktree_cleartask_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005947_rage_aitasktree_getactiveleaftask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005949_rage_aitasktree_findtaskbytypeactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005950_rage_aitasktree_findtaskbytypewithpriority_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_006037_rage_strstreamingloadermanager_callkeepalivecallbackifnecessary(ph: *mut Phantom); }
extern "C" { pub fn f_006056_rage_strstreaminginfomanager_removeobject_rage_strindex_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006066_rage_strpackfilemanager_getimagefilefromhandle_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006077_rage_atmapentry_rage_strindex_std_2_pair_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006084_rage_strstreaminginfomanager_requestobject_rage_strindex_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006088_rage_atarray_rage_atnonfinalhashstring_0_unsigned_short_grow_uns(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006093_rage_strstreamingallocator_removefrommemoryused_void_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006131_rage_strstreamingallocator_streamingallocate_rage_datresourcemap(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006156_rage_atarray_rage_strindex_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006162_rage_strstreaminginfo_gethandle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006167_rage_strstreaminginfomanager_isobjectinimage_rage_strindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006175_rage_strstreaminginfomanager_clearrequiredflag_rage_strindex_uns(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006188_rage_atarray_int_0_unsigned_short_ptr_ref_rage_atmap_int_rage_at(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006225_rage_atvarstring_atvarstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_006235_rage_strstreamingvisualize_setmarkertext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006254_rage_strrequest_request_rage_strlocalindex_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_006255_rage_strrequest_hasloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006256_rage_strrequest_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006257_rage_strrequest_clearrequiredflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006261_rage_strrequest_getindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006263_rage_strmodelrequest_request_rage_strlocalindex_rage_atnonfinalh(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_006264_rage_strmodelrequest_hasloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006265_rage_strmodelrequest_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006266_rage_strmodelrequest_isvalid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006267_rage_strmodelrequest_clearrequiredflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006268_rage_strmodelrequest_op_rage_strmodelrequest_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006361_rage_atarray_float_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006511_rage_audsound_computedurationmsincludingstartoffsetandpredelay_b(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006518_rage_audsound_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006521_rage_audsound_computedynamicvalue_rage_auddynamicvalueid_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_006525_rage_audsound_getwaveslot_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006575_rage_audenvelopesound_setrequestedenvelope_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_006576_rage_audenvelopesound_setchildsoundreference_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006602_rage_audexternalstreamsound_initstreamplayer_rage_audreferencedr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_006610_rage_audsound_decompressmetadata_untyped_rage_sound_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006612_rage_audgranularsound_setx_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_006613_rage_audgranularsound_setgrainplaybackstyle_unsigned_int_rage_au(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006614_rage_audgranularsound_setdebugrenderingenabled_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006615_rage_audgranularsound_lockpitch_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006616_rage_audgranularsound_setchannelvolume_unsigned_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32); }
extern "C" { pub fn f_006617_rage_audgranularsound_setaspurerandom_unsigned_int_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006618_rage_audgranularsound_setaspurerandomwithstyle_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006619_rage_audgranularsound_setmixnative_unsigned_int_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006620_rage_audgranularsound_setgrainskippingenabled_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: f32); }
extern "C" { pub fn f_006621_rage_audgranularsound_setwobbleenabled_unsigned_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_006666_rage_audrandomizedsound_scomputedurationmsexcludingstartoffsetan(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006714_rage_audsound_play(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006715_rage_audsound_prepareandplay_rage_audwaveslot_ptr_bool_int_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_006716_rage_audsound_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006717_rage_audsound_stopandforget_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006719_rage_audsound_prepare_rage_audwaveslot_ptr_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_006721_rage_audsound_getdynamicvariablehandlefromname_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006724_rage_audsound_setvariablevaluedownhierarchyfromname_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_006725_rage_audsound_setvariablevaluedownhierarchyfromname_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_006727_rage_audsound_setrequestedfrequencyratio_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_006728_rage_audsound_getpredelayremainingms_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006730_rage_audsound_getcurrentplaytime_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006732_rage_gsoundsisoftype_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006740_rage_audspeechsound_loadspeechmetadata_char_const_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006741_rage_audspeechsound_findvariationinfoforvoiceandcontext_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_006742_rage_audspeechsound_findnumvariationsforvoiceandcontext_unsigned(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_006743_rage_audspeechsound_findnumvariationsforvoiceandcontext_unsigned(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_006744_rage_audspeechsound_findbankidforvoiceandcontext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006745_rage_audspeechsound_setexclusionprefix_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006746_rage_audspeechsound_initspeech_unsigned_int_char_const_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_006747_rage_audspeechsound_initspeech_unsigned_int_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_006748_rage_audspeechsound_initspeech_char_const_ptr_char_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_006749_rage_audspeechsound_doesvoiceexist_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006750_rage_audspeechsound_batchprepare_rage_audspeechsoundinfo_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006751_rage_audspeechsound_getbankid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006752_rage_audspeechsound_getwavenamehash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006759_rage_audstreamingsound_scomputedurationmsexcludingstartoffsetand(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006844_rage_ambisonicdecodedata_ambisonicdecodedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006846_rage_audambisonics_setenvironmentdecoder_rage_ambisonicdecodermo(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006877_rage_audmetadatamanager_getobjectmetadataptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006881_rage_audmetadatamanager_getobjectmetadatareffromhash_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006883_rage_audcategory_getnamestring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006886_rage_audmetadatamanager_getobjectnamefromnametableoffset_unsigne(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006892_rage_audmetadatamanager_dtoraudmetadatamanager(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006894_rage_audmetadatamanager_loadmetadatachunk_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_006895_rage_audmetadatamanager_init_char_const_ptr_char_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_006896_rage_audmetadatamanager_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006898_rage_audmetadatamanager_computenumberofobjects_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006901_rage_audcategorymanager_getdynamiccategoryptr_int_rage_category(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_006903_rage_audcategorymanager_getcategoryptr_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006908_rage_audcategorycontroller_update_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_006909_rage_audcategorycontroller_getsettings_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006910_rage_audcategorycontroller_begingainfadelinear_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_006914_rage_auddynamicmixmgr_printdebug(ph: *mut Phantom); }
extern "C" { pub fn f_006916_rage_audcategorycontrollermanager_createcontroller_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006917_rage_audcategorycontrollermanager_createcontroller_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_006918_rage_audcategorycontrollermanager_destroycontroller_rage_audcate(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006919_rage_audcategorycontrollermanager_createmultiplecontrollers_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_006920_rage_audcategorycontrollermanager_destroymultiplecontrollers_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_006921_rage_audcategorycontrollermanager_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006922_rage_audcategorycontrollermanager_recurseaddcategorywidget_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006924_rage_audcontroller_clearallsounds_rage_audcontrollerentityref_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006925_rage_audcontroller_clearallsounds(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006926_rage_audcontroller_preupdate_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006927_rage_audcontroller_update_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006930_rage_audcontroller_stopallsounds(ph: *mut Phantom); }
extern "C" { pub fn f_006933_rage_audmetadatamanager_getobjectname_rage_audmetadataref_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006938_rage_audcurve_audcurve(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006939_rage_audcurve_init_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006940_rage_audcurve_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006941_rage_audcurve_calculatevalue_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_006942_rage_audcurve_calculaterescaledvalue_float_float_float_float_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32) -> f32; }
extern "C" { pub fn f_006943_rage_audcurve_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006944_rage_audmetadatamanager_getobjectname_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006950_rage_audmixgroup_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006952_rage_auddynamicmixmgr_initmixgroupsforchunk_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006954_rage_audmetadatamanager_findchunkid_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006955_rage_audmetadatamanager_getobjectinfofromspecificchunk_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_006956_rage_auddynamicmixmgr_getmixgroup_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006957_rage_auddynamicmixmgr_getmixgrouphash_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006958_rage_auddynamicmixmgr_createmixgroup_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006959_rage_auddynamicmixmgr_releasemixgroupreference_unsigned_int_rage(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006964_rage_audengine_partialinit(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_006969_rage_audengine_partialshutdown(ph: *mut Phantom); }
extern "C" { pub fn f_006971_rage_audengine_triggerupdate(ph: *mut Phantom); }
extern "C" { pub fn f_006972_rage_audengine_waitforaudioframe(ph: *mut Phantom); }
extern "C" { pub fn f_006973_rage_audengine_commitgamesettings_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006983_rage_audengineutil_matchname_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006985_rage_audenvironment_setvolumelistenermatrix_rage_mat34v_const_re(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006986_rage_audenvironment_setpanninglistenermatrix_rage_mat34v_const_r(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006987_rage_audenvironment_computepositionrelativetopanninglistener_rag(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006988_rage_audenvironment_computesqddistancetopanninglistener_rage_vec(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_006989_rage_audenvironment_getrolloff_unsigned_int_const(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_006990_rage_audenvironment_setdirectionalmicparameters_float_float_floa(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_006991_rage_audenvironment_setrolloff_float_unsigned_int(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_006992_rage_audenvironment_computepositionfrompan_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_006994_rage_audenvironment_computespeakervolumesfrompan_rage_audenviron(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_006996_rage_audenvironmentgroupinterface_baseinit_rage_audentity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006997_rage_audenvironmentgroupinterface_dtoraudenvironmentgroupinterfa(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006998_rage_audenvironmentgroupinterface_addsoundreference(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006999_rage_audenvironmentgroupinterface_removesoundreference(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007000_rage_audenvironmentgroupinterface_unsetmixgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007001_rage_audenvironmentgroupinterface_setprevmixgroup_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007002_rage_audenvironmentgroupmanager_isenvironmentgroupindeletedlist(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007005_rage_audenvironment_getmusicsubmix_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007006_rage_audenvironment_getreflectionsoutputsubmix_unsigned_int_cons(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_007007_rage_audenvironment_getvehicleenginesubmix_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007008_rage_audenvironment_getvehicleexhaustsubmix_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007009_rage_audenvironment_getconversationspeechsubmix_unsigned_int_con(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007010_rage_audenvironment_freeconversationspeechsubmix_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007011_rage_audenvironment_assignreflectionsoutputsubmix(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007012_rage_audenvironment_freereflectionsoutputsubmix_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007013_rage_audenvironment_isreflectionsoutputsubmixfree_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_007014_rage_audenvironment_setspecialeffectmode_rage_audspecialeffectmo(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007015_rage_audmetadataobjectinfo_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007022_rage_audremotecontrol_ispresent(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007024_rage_audmetadatamanager_unloadmetadatachunk_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_007026_rage_audmetadatamanager_reservechunk_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007027_rage_audmetadatamanager_getstringidfromhash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007028_rage_audmetadatamanager_getobjectmetadataptr_rage_audmetadataref(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007030_rage_audmetadatamanager_removeextrastringtable_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007031_rage_audmetadatamanager_getobjectmetadataptr_char_const_ptr_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_007032_rage_audmetadatamanager_getobjectinfo_unsigned_int_rage_audmetad(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007033_rage_audmetadatamanager_getobjectinfo_rage_audmetadataref_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007035_rage_audmetadatamanager_getobjectinfofromobjectindex_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007036_rage_audmetadatamanager_getobject_verify_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007037_rage_audmetadatamanager_getobject_verify_rage_audmetadataref_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007040_rage_audmetadatamanager_getassetchangelist_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007047_rage_audsoundfactory_getmetadataptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007048_rage_audsoundfactory_getmetadataptr_rage_audmetadataref_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007049_rage_audsoundfactory_processhierarchy_unsigned_int_rage_audsound(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_007050_rage_audsoundfactory_processhierarchy_rage_audmetadataref_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_007051_rage_audsoundfactory_getbanknamefromindex_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007052_rage_audsoundfactory_getbankindexfromname_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007053_rage_audsoundfactory_getbankindexfromname_unsigned_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_007054_rage_audsoundfactory_getbankindexfrommetadataref_unsigned_int_co(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007055_rage_audsoundfactory_addbankremapping_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_007056_rage_audsoundfactory_removebankremapping_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007063_rage_audsoundmanager_getvariableaddress_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007064_rage_audsimplesmootherdiscrete_init_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32); }
extern "C" { pub fn f_007065_rage_audsoundmanager_setvariablevalue_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_007068_rage_audsimplesmootherdiscrete_calculatevalue_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> f32; }
extern "C" { pub fn f_007069_rage_audsoundpool_getpoolutilisation_rage_audsoundpoolstats_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007070_rage_audsoundmanager_startunloadingchunk_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007076_rage_audsoundpool_debugprintsoundpool_const(ph: *mut Phantom); }
extern "C" { pub fn f_007077_rage_audsoundset_init_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007078_rage_audsoundset_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007079_rage_audsoundset_find_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007080_rage_audsoundset_find_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007081_rage_audsoundsetfieldref_resolvemetadataref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007082_rage_audtempomap_computetimelineposition_float_rage_audtempomap(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_007083_rage_audtempomap_computetimeforposition_rage_audtempomap_timelin(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_007085_rage_audtracker_dtoraudtracker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007086_rage_audremotecontrol_isconnected_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007087_rage_audremotecontrol_sendxmlmessage_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_007093_rage_audentity_removevariableblockref_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007094_rage_audrequestedsettings_setpan_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007095_rage_audrequestedsettings_setpitch_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007096_rage_audrequestedsettings_setposition_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007097_rage_audrequestedsettings_setposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007098_rage_audrequestedsettings_setvolume_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007099_rage_audrequestedsettings_setpostsubmixvolumeattenuation_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007100_rage_audrequestedsettings_setsourceeffectmix_float_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007101_rage_audrequestedsettings_setshouldattenuateoverdistance_rage_tr(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007102_rage_audrequestedsettings_setshoulddisablerearfiltering_rage_tri(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007103_rage_audrequestedsettings_setlowpassfiltercutoff_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007104_rage_audrequestedsettings_sethighpassfiltercutoff_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007105_rage_audrequestedsettings_setfrequencysmoothingrate_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007107_rage_audrequestedsettings_setdopplerfactor_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007108_rage_audrequestedsettings_setvolumecurvescale_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007109_rage_audrequestedsettings_setquadspeakerlevels_float_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007110_rage_audrequestedsettings_setlistenermask_unsigned_char(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007111_rage_audrequestedsettings_setenvironmentalloudness_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007112_rage_audrequestedsettings_setenvironmentalloudnessfloat_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007113_rage_audrequestedsettings_getpitch_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007114_rage_audrequestedsettings_getposition_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007115_rage_audrequestedsettings_getvolume_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_007116_rage_audrequestedsettings_getpostsubmixvolumeattenuation_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_007117_rage_audrequestedsettings_setenvironmentgroup_rage_audenvironmen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007118_rage_audsmoother_audsmoother(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007119_rage_audsmoother_init_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_007120_rage_audsmoother_init_float_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_007121_rage_audsmoother_setrates_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_007122_rage_audsmoother_setbounds_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_007123_rage_audsmoother_calculatevalue_float(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_007124_rage_audsmoother_calculatevalue_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> f32; }
extern "C" { pub fn f_007125_rage_audsmoother_calculatevalue_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_007126_rage_audsimplesmoother_audsimplesmoother(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007127_rage_audsimplesmoother_init_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_007128_rage_audsimplesmoother_calculatevalue_float(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_007129_rage_audsimplesmoother_calculatevalue_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> f32; }
extern "C" { pub fn f_007130_rage_audsimplesmoother_calculatevalue_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_007131_rage_audsimplesmootherdiscrete_audsimplesmootherdiscrete(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007132_rage_audsimplesmootherdiscrete_calculatevalue_float(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_007133_rage_audsimplesmootherdiscrete_calculatevalue_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_007134_rage_audfluctuatorprobbased_init_float_float_float_float_float_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: f32); }
extern "C" { pub fn f_007135_rage_audfluctuatorprobbased_setbands_float_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_007136_rage_audfluctuatorprobbased_calculatevalue(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_007137_rage_audvolumecone_audvolumecone(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007138_rage_audvolumecone_init_unsigned_int_rage_vec3v_const_ref_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_007139_rage_audvolumecone_computeattenuation_rage_mat34v_const_ref_unsi(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_007140_rage_audvolumeconelite_init_rage_vec3v_const_ref_float_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_007141_rage_audvolumeconelite_computeattenuation_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_007178_rage_audvoicemgr_setphysicalvoicelimit(ph: *mut Phantom); }
extern "C" { pub fn f_007189_rage_audvoicemgr_printphysicalvoices(ph: *mut Phantom); }
extern "C" { pub fn f_007192_rage_audwaveslot_getloadedbankname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007200_char_ptr_rage_formatf_char_255ul_char_ref_255ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007201_rage_audwaveslot_getbankname_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_007203_rage_audwaveslot_loadbank_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_007205_rage_audwaveslot_findbankid_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007206_rage_audwaveslot_findwaveslot_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007210_rage_audwaveslot_getwaveloadingstatus_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_007211_rage_audwaveslot_loadwave_unsigned_int_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_007212_rage_audwaveslot_getreferencecount_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007218_rage_audwaveslot_setsearchpath_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007219_rage_audwaveslot_removesearchpath_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007220_rage_audwaveslot_findloadedbankwaveslot_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_007221_rage_audwaveslot_findwaveslot_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_007226_rage_audwaveslot_getbankloadingstatus_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007230_rage_audwaveslot_removeslotreference_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007237_rage_audwaveref_init_rage_audwaveslot_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_007238_rage_audwaveref_findchunk_unsigned_int_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_007239_rage_audwaveref_findformat_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_007240_rage_audwaveref_findmarkerdata_rage_audwavemarkerlist_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007242_rage_audmixerdevice_getsubmixindex_rage_audmixersubmix_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007273_rage_audmixersyncmanager_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007278_rage_audmixerdevice_initclientthread_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_007279_rage_audmixerdevice_flagthreadcommandbufferreadytoprocess_unsign(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007280_rage_audmixerdevice_waitonthreadcommandbufferprocessing_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007326_rage_audpcmsourceinterface_setparam_unsigned_int_unsigned_int_fl(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_007327_rage_audpcmsourceinterface_setparam_unsigned_int_unsigned_int_un(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_007328_rage_audpcmsourceinterface_writecustomcommandpacket_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007334_rage_audmixersubmix_addoutput_unsigned_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_007335_rage_audmixersubmix_disconnectoutput_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007336_rage_audmixersubmix_setoutputvolumes_unsigned_int_float_const_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_007338_rage_audmixersubmix_seteffectparam_int_unsigned_int_float_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32); }
extern "C" { pub fn f_007339_rage_audmixersubmix_seteffectparam_int_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_007340_rage_audmixersubmix_setflag_rage_audmixersubmix_submixflag_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007361_rage_audmediareader_close(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007362_rage_audmediareader_destroymediastreamer_rage_audmediareader_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007395_rage_audentity_shutdownclass(ph: *mut Phantom); }
extern "C" { pub fn f_007396_rage_audentity_audentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007397_rage_audentity_dtoraudentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007398_rage_audentity_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007399_rage_audentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007400_rage_audentity_allocateentityvariableblock_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007401_rage_audentity_setentityvariable_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_007402_rage_audentity_getentityvariablevalue_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_007403_rage_audentity_stopallsounds_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007404_rage_audentity_createsound_localreference_unsigned_int_rage_auds(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007406_rage_audentity_createsound_localreference_rage_audmetadataref_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007407_rage_audentity_createsound_persistentreference_rage_audmetadatar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007408_rage_audentity_createsound_persistentreference_char_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007409_rage_audentity_createsound_persistentreference_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007410_rage_audentity_createandplaysound_char_const_ptr_rage_audsoundin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007411_rage_audentity_addbatchedsoundrequest_rage_audmetadataref_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007412_rage_audentity_createandplaysound_unsigned_int_rage_audsoundinit(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007413_rage_audentity_processbatchedsoundrequests_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007414_rage_audentity_resetdeletedobjectlists(ph: *mut Phantom); }
extern "C" { pub fn f_007415_rage_audentity_createandplaysound_persistent_char_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007416_rage_audentity_createandplaysound_persistent_unsigned_int_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007417_rage_audentity_createandplaysound_persistent_rage_audmetadataref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007418_rage_audentity_stopandforgetsoundsprivate_rage_audsound_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i32); }
extern "C" { pub fn f_007426_rage_vfxbrokenfraginfo_dtorvfxbrokenfraginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007496_rage_decalmanager_isalive_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007497_rage_decalmanager_remove_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007499_rage_decalmanager_removeinrange_rage_vec3v_const_ref_rage_scalar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007503_rage_decalmanager_findclosest_unsigned_int_signed_char_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: f32, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_007528_rage_ptfxattach_attach_rage_ptxeffectinst_ptr_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_007559_rage_ptfxmanager_gettriggeredinst_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007562_rage_ptfxmanager_getregisteredinst_void_const_ptr_int_bool_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_007570_rage_ptfxwrapper_setcolourtint_rage_ptxeffectinst_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007576_rage_ptfxscript_start_rage_atnonfinalhashstring_rage_atnonfinalh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: f32, a7: i32) -> i32; }
extern "C" { pub fn f_007577_rage_ptfxscript_stop_int_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007578_rage_ptfxscript_remove_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007579_rage_ptfxscript_destroy_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007581_rage_ptfxscript_doesexist_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_007583_rage_ptfxscript_updateoffset_int_rage_mat34v_const_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_007584_rage_ptfxscript_updateevo_int_unsigned_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_007585_rage_ptfxscript_updatecolourtint_int_rage_vec3v_const_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_007589_rage_ptfxscript_updatefarclipdist_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_007590_rage_ptfxscript_updateforcevehicleinteriorfx_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_007635_rage_atmapentry_void_ptr_unsigned_int_const_ref_rage_atmap_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007658_rage_fwptrlistsinglelink_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007661_rage_fwptrlist_ismemberoflist_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007662_rage_fwquadtreenode_fwquadtreenode_rage_fwrect_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_007663_rage_fwquadtreenode_dtorfwquadtreenode(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007664_rage_fwquadtreenode_destroytree(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007665_rage_fwquadtreenode_getallnodupes_rage_fwptrlistsinglelink_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007667_rage_fwquadtreenode_forallmatching_rage_fwrect_const_ref_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007668_rage_fwquadtreenode_forallmatching_rage_vector3_const_ref_rage_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007669_rage_fwquadtreenode_additemrestrictdupes_void_ptr_rage_fwrect_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007671_rage_fwquadtreenode_deleteitem_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007672_rage_fwquadtreenode_deleteallitems(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007673_rage_fwquadtreenode_getentrycount_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007674_rage_fwquadtreenode_getnodecount_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007682_rage_atmap_unsigned_int_rage_fwkdop18_rage_atmaphashfn_unsigned(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007694_rage_fwgeom_intersectsphereray_rage_spdsphere_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_007700_rage_fwportalcorners_calcdistancetopoint_rage_vec3v_const_ref_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007704_rage_fwaudioentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007705_rage_fwaudioentity_preupdateservice_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007706_rage_fwaudioentity_addanimevent_rage_audanimevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007739_rage_fwaudioentity_dtorfwaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007803_rage_networklogutils_writelogevent_rage_netlogginginterface_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007848_rage_neteventmgr_checkforspaceinpool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007849_rage_neteventmgr_prepareevent_rage_netgameevent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007949_rage_netgameevent_netgameevent_unsigned_short_bool_bool_rage_net(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_008075_rage_netobject_islocalflagset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008682_rage_fwbasepool_fwbasepool_int_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_008683_rage_fwconfigmanager_getsizeofpool_unsigned_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008686_rage_fwbasepool_getindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008688_rage_fwbasepool_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008872_rage_fwfsm_fwfsm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008873_rage_fwfsm_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008917_rage_fwtimer_enduserpause(ph: *mut Phantom); }
extern "C" { pub fn f_008918_rage_fwtimer_startuserpause_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008945_rage_fwbasepool_forall_bool_ptr_void_ptr_void_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008950_rage_fwbasepool_getjustindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008953_rage_fwknownrefholder_fwknownrefholder_void_ptr_ptr_rage_fwknown(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_009010_rage_crdumpoutputstring_getstring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009048_rage_crproperty_getattribute_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009050_rage_crproperty_stringkey_calckey_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009224_rage_crskeleton_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009225_rage_crskeleton_partialreset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009230_rage_crskeleton_inverseupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009231_rage_crskeleton_partialinverseupdate_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009233_rage_crskeleton_getglobalmtx_unsigned_int_rage_mat34v_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009234_rage_crskeleton_setglobalmtx_unsigned_int_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009235_rage_atmap_unsigned_int_int_rage_atmaphashfn_unsigned_int_rage_a(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009283_rage_grmmodel_getgeometry_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009314_pixend(ph: *mut Phantom); }
extern "C" { pub fn f_009363_rage_grmgeometryqb_setusesecondaryqbindex_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009417_pixbegin_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009544_rage_grcdrawsinglequadf_float_float_float_float_float_float_floa(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: i64); }
extern "C" { pub fn f_009655_rage_grcfont_getstringwidthinternal_char_const_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009657_rage_grcfont_drawf_float_float_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i64); }
extern "C" { pub fn f_010163_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010164_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010170_rage_grcdebugdraw_rectaxisaligned_rage_vec2v_const_ref_rage_vec2(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010172_rage_grcdebugdraw_text_float_float_rage_color32_char_const_ptr_b(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i64, a4: i32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010179_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010181_rage_grcdebugdraw_poly_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010184_rage_grcdebugdraw_circle_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010188_rage_grcdebugdraw_boxaxisaligned_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010189_rage_grcdebugdraw_boxoriented_rage_vec3v_const_ref_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010191_rage_grcdebugdraw_cylinder_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_010192_rage_grcdebugdraw_cone_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010193_rage_grcdebugdraw_capsule_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010195_rage_grcdebugdraw_arrow_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_010217_rage_grcdebugdraw_printtoscreencoors_char_const_ptr_int_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010220_rage_grcdebugdraw_adddebugoutput_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010221_rage_grcdebugdraw_adddebugoutput_rage_color32_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_010224_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010225_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010511_rage_iovalue_ispressed_float_rage_iovalue_readoptions_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010521_rage_iovalue_getnorm01_rage_iovalue_readoptions_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_010526_jpeg_createcompress_jpeg_compress_struct_ptr_int_unsigned_long(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010527_jpeg_destroy_compress_jpeg_compress_struct_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010528_jpeg_finish_compress_jpeg_compress_struct_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010529_jpeg_write_marker_jpeg_compress_struct_ptr_int_unsigned_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_010530_jpeg_start_compress_jpeg_compress_struct_ptr_unsigned_char(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010537_jpeg_write_scanlines_jpeg_compress_struct_ptr_unsigned_char_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010567_jpeg_set_quality_jpeg_compress_struct_ptr_int_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010568_jpeg_set_defaults_jpeg_compress_struct_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010602_jpeg_std_error_jpeg_error_mgr_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010940_rage_atarray_rage_cutfeventargs_ptr_0_unsigned_short_resizegrow(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011222_rage_cutfcutscenefile2_findobjectsoftype_int_rage_atarray_rage_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_011323_rage_atnamespacedhashstring_rage_athashstringnamespaces_0_op_cha(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011400_rage_cutsmanager_getconcatsectionfortime_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_011453_rage_cutsmanager_isconcatted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011458_rage_cutsmanager_isconcatsectionvalidforplayback_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011459_rage_cutsmanager_getconcatsectionstarttime_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_011460_rage_cutsmanager_getconcatsectionduration_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_011745_rage_fraginst_setresetmatrix_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011748_rage_fraginst_getpartbroken_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011749_rage_fraginst_getgroupbroken_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011750_rage_fraginst_getchildbroken_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011751_rage_fraginst_getcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011755_rage_fraginst_getgroupfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011763_rage_fraginst_deleteabove_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011772_rage_fraginst_restoreabove_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011774_rage_fragcacheentry_updateisfurtherbreakable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011811_rage_fragcachemanager_releasecacheentry_rage_fragcacheentry_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011822_rage_matrix34_dot_rage_matrix34_const_ref_rage_matrix34_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012021_rage_fragphysicslodgroup_getlodbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012183_rage_ptxeffectinst_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012196_rage_ptxeffectinst_setbasemtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012207_rage_ptxeffectinst_setevovaluefromhash_rage_athashvalue_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_012208_rage_ptxeffectinst_getevovaluefromhash_rage_athashvalue(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_012430_rage_ptxmanager_reset_rage_ptxeffectrule_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012843_rage_environmentcloth_getcloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012844_rage_environmentcloth_getcloth_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012875_rage_environmentcloth_controlvertex_int_rage_vec3v_const_ref_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_013019_rage_bgglassmanager_getglassbreakable_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_013034_rage_bgbreakable_getpanesize_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_013116_rage_bgglassmanager_setcontactfunctor_rage_atdelegate_void_rage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013429_char_ptr_rage_safecatf_char_char_ptr_unsigned_long_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_013602_rage_netpeerid_op_rage_netpeerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013699_char_ptr_rage_formatf_char_2048ul_char_ref_2048ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_014076_char_ptr_rage_formatf_char_4ul_char_ref_4ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_015812_rage_phconstraintbase_getinstancea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015813_rage_phconstraintbase_getinstanceb(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015843_rage_phconstraintdistance_getworldposa(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015844_rage_phconstraintdistance_getworldposb(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015891_rage_phconstraintmgr_gettemporarypointer_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016018_rage_phsimulator_activateobject_rage_phinst_ptr_rage_phcollider(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_016164_rage_matrix34_dot_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016605_rage_phverletcloth_dynamicpinvertex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016606_rage_phverletcloth_dynamicunpinvertex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016607_rage_phverletcloth_dynamicunpinall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016626_rage_phclothdata_transformvertexpositions_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016661_rage_phwind_getlocalvelocity_rage_vec3v_const_ref_rage_vec3v_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_016909_rage_phlevelnodetree_rage_phlooseoctreenode_testprobe_rage_phseg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_016914_rage_phverletcloth_computeclothboundingvolume(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016919_rage_phverletcloth_zerodeltapositions_withcompression(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016943_rage_scrthread_printstacktrace_void_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016949_rage_scrthread_getactivethread(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_016981_rage_scrthread_getthread_rage_scrthreadid(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_017232_rage_crclipdictionary_crclipdictionary_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_017233_rage_crclipdictionary_dtorcrclipdictionary(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018216_rage_mvparameterbuffer_iterator_begin_rage_mvparameterbuffer_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_018217_rage_mvparameterbuffer_iterator_next(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018897_cgarages_clearall(ph: *mut Phantom); }
extern "C" { pub fn f_019027_cleveldata_getfilename_int_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_019120_cvehiclerecordingmgr_restoreinfoformatrix_rage_matrix34_ref_cveh(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019133_cvehiclerecordingmgr_getplaybackslot_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019135_cvehiclerecordingmgr_stopplaybackrecordedcar_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019136_cvehiclerecordingmgr_isplaybackgoingonforcar_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019137_cvehiclerecordingmgr_getplaybackidforcar_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019138_cvehiclerecordingmgr_isplaybackswitchedtoaiforcar_cvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019145_cvehiclerecordingmgr_skiptimeforwardinrecording_cvehicle_ptr_flo(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_019146_cvehiclerecordingmgr_getendtimeofrecording_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_019147_cvehiclerecordingmgr_getplaybackrunningtime_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_019153_cvehiclerecordingmgr_getpositionofcarrecordingattime_int_float_r(ph: *mut Phantom, a0: i32, a1: f32, a2: i64); }
extern "C" { pub fn f_019157_cvehiclerecordingmgr_hasrecordingfilebeenloaded_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_019158_char_ptr_rage_formatf_char_24ul_char_ref_24ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019241_cstuntjumpmanager_isastuntjumpinprogress(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_019372_cambientmodelsets_parserpostload(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019383_rage_atarray_cambientmodelsetmanager_samssourceinfo_0_unsigned_s(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019416_cconditionalclipset_checkconditions_cscenariocondition_sscenario(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019419_cconditionalanims_checkconditions_cscenariocondition_sscenarioco(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_019438_cconditionalanimmanager_append_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019479_cvehiclelockdebuginfo_dtorcvehiclelockdebuginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019565_cpedbonetagconvertor_getboneindexfrombonetag_rage_crskeletondata(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_019621_cmoveanimatedbuilding_setphase_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_019622_cmoveanimatedbuilding_setrate_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_019623_cmoveanimatedbuilding_getphase(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_019624_cmoveanimatedbuilding_getrate(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_019634_cmoveped_settasknetwork_rage_fwmovenetworkplayer_ptr_float_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_019696_cmovevehicle_getmechanismphase(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_019697_cmoveobject_setnetwork_rage_fwmovenetworkplayer_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_019753_cdebugselector_addwidgets_rage_bkbank_ptr_char_const_ptr_rage_da(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_019765_cdebuganimsceneentityselector_rebuildentitylist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019770_cdebugclipselector_dtorcdebugclipselector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019784_cdebugclipselector_addwidgets_rage_bkgroup_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_019785_cdebugclipselector_removewidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019796_cdebugobjectselector_streamselected_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019797_cdebugobjectselector_getselectedmodelid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019802_cdebugpedmodelselector_loadmodelnames(ph: *mut Phantom); }
extern "C" { pub fn f_019804_cdebugpedmodelselector_selectmodel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019834_canimplacementtool_cleanupentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019844_rage_matrix34_inverse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019864_cdebugobjectselector_addwidgets_rage_bkbank_ptr_char_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_019907_cdebugfileselector_dtorcdebugfileselector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020280_canimscenematrix_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_020281_canimsceneentity_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_020314_canimscenematrix_setmatrix_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020315_canimscene_canimscene_rage_atnonfinalhashstring_eanimsceneflagsb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_020318_canimscenematrix_dtorcanimscenematrix(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020334_canimscene_getentity_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_020335_canimscene_addentity_rage_atnonfinalhashstring_canimsceneentity(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_020336_canimscene_findsectionfortime_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_020340_canimscene_setinternallooping_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_020341_canimsceneped_createdebugpedifrequired(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020342_canimsceneobject_createdebugobjifrequired(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020343_canimscenevehicle_createdebugvehifrequired(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020344_canimscene_removeentityfromscene_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020347_canimsceneentityhandle_updateentityid_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020349_canimscene_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020353_canimsceneentity_setlocationdata_rage_atnonfinalhashstring_anims(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_020356_canimscene_getscenematrix(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020383_canimsceneevent_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_020389_canimsceneevent_notifyrangeupdated(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020400_canimsceneevent_canimsceneevent_canimsceneevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_020415_canimsceneclip_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_020420_canimsceneclipset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_020423_canimsceneentityhandle_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_020436_canimsceneentityhandle_getphysicalentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020484_canimsceneped_setped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020493_canimscenemanager_getscenedescription_canimscene_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020516_ceventlayerspane_dtorceventlayerspane(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020518_canimsceneeditor_widgets_dtorwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020534_canimsceneeditor_createeventandaddtocurrentscene_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_020605_canimsceneobject_setobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_020609_canimscenevehicle_setvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020613_canimsceneplaysceneevent_processloadscene_canimscene_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_020625_canimsceneplayanimevent_onclipchanged_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_020630_canimsceneruntimedata_canimsceneplayanimevent_runtimedata_op(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020634_canimscene_getinstid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_020636_ctaskscriptedanimation_setrate_float_ctaskscriptedanimation_epla(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_020637_ctaskscriptedanimation_getblendphaseid_ctaskscriptedanimation_ep(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_020638_ctaskscriptedanimation_getblendphaseoutid_ctaskscriptedanimation(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_020670_canimsceneplayvfxevent_initparticledata_cparticleattr_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020673_canimsceneplayvfxevent_dtorcanimsceneplayvfxevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020677_canimsceneleadindata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_020707_canimsceneped_unregisterped(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020765_cridableentrypointevaluationdebuginfo_dtorcridableentrypointeval(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020807_cgivepedjackedeventdebuginfo_dtorcgivepedjackedeventdebuginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020838_canimsceneped_dtorcanimsceneped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020844_canimscenevehicle_dtorcanimscenevehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020851_canimsceneruntimedata_canimsceneobject_widgets_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020852_canimsceneruntimedata_canimsceneobject_runtimedata_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020864_canimsceneplayanimevent_dtorcanimsceneplayanimevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020913_canimsceneobjectinitialiser_dtorcanimsceneobjectinitialiser(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020918_canimscenevehicleinitialiser_dtorcanimscenevehicleinitialiser(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020931_canimsceneplaycameraanimevent_dtorcanimsceneplaycameraanimevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020935_canimsceneforcemotionstateevent_dtorcanimsceneforcemotionstateev(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020939_canimscenecreatepedevent_dtorcanimscenecreatepedevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020940_canimsceneruntimedata_canimscenecreatepedevent_runtimedata_shutd(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020944_canimscenecreateobjectevent_dtorcanimscenecreateobjectevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020948_canimscenecreatevehicleevent_dtorcanimscenecreatevehicleevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020952_canimsceneplaysceneevent_dtorcanimsceneplaysceneevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020956_canimsceneinternalloopevent_dtorcanimsceneinternalloopevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020965_canimscenemanager_dtorcanimscenemanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020970_canimplacementtool_dtorcanimplacementtool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020974_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_020975_rage_atbinarymap_float_unsigned_long_datapair_ptr_std_2_partial(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_020976_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_020977_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_021010_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_021011_animdictpred_op_cdebugclipdictionary_ptr_cdebugclipdictionary_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_021012_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_animdictpr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_021014_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_animnamegr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_021015_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_animnamegr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_021016_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_animnamegr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_021019_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_animnamegr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_021021_canimsceneeventiterator_canimsceneevent_findnextevent_canimscene(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_021022_canimsceneeventiterator_canimsceneplayanimevent_findnextevent_ca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_021039_naocclusionpathnodedata_dtornaocclusionpathnodedata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021042_audvehicleconductor_dtoraudvehicleconductor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021046_audmemoryreporter_addmetadata_char_const_ptr_rage_audmetadataman(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_021047_audnorthaudioengine_loadspdata(ph: *mut Phantom); }
extern "C" { pub fn f_021051_audreplayaudioentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021052_audnorthaudioengine_update(ph: *mut Phantom); }
extern "C" { pub fn f_021054_audweatheraudioentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021057_audradioaudioentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021058_audscannermanager_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021059_audspeechmanager_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021060_audscriptaudioentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021061_audweaponaudioentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021062_audvehiclereflectionsentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021070_audpedfootstepaudio_leftladderhand_unsigned_int_void_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_021071_audpedfootstepaudio_rightladderhand_unsigned_int_void_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_021074_audpedfootstepaudio_pedrollhandler_unsigned_int_void_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_021139_audpedaudioentity_inittennisvocalizationravesettings(ph: *mut Phantom); }
extern "C" { pub fn f_021140_audwaveslotmanager_addwaveslot_unsigned_int_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_021141_bool_creplaymgrinternal_recordfx_cpacketsetplayerpainroot_cpacke(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021142_rage_atarray_audanimalcontextinfo_0_unsigned_short_op_rage_atarr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021143_audspeechaudioentity_setambientvoicename_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021145_audradiostation_maintainheist4spstationstate(ph: *mut Phantom); }
extern "C" { pub fn f_021147_audradiostation_reset(ph: *mut Phantom); }
extern "C" { pub fn f_021148_audscannermanager_cancelanyplayingreports_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021149_audspeechmanager_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021150_audscriptaudioentity_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021151_audvehiclereflectionsentity_detachfromall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021152_audnorthaudioengine_minimalupdate(ph: *mut Phantom); }
extern "C" { pub fn f_021156_audscriptaudioentity_stopstream(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021158_audreplayaudioentity_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021160_audradiostation_mergeradiostationlist_rage_radiostationlist_cons(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021161_audradiostation_addmusictracklist_rage_radiostationtracklist_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_021162_audradiostation_addtracklist_rage_radiostationtracklist_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_021163_audradiostation_randomizecategorystrides_bool_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021164_audradiostation_audradiostation_rage_radiostationsettings_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_021165_audradiostation_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021168_audradioslot_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021169_audradiotrack_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021170_audradiostation_dtoraudradiostation(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021173_audscriptaudioentity_replayloadscriptbank_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_021174_audnorthaudioengine_loadmpdata(ph: *mut Phantom); }
extern "C" { pub fn f_021175_audnorthaudioengine_activateslowmomode_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021176_audnorthaudioengine_deactivateslowmomode_rage_slowmotype(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021177_audpedscenario_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021178_audvehicleaudioentity_ispedsatinexternalseat_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_021179_audvehicleaudioentity_setradiostation_audradiostation_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_021180_audspeechmanager_requesttennisbanks_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021181_audspeechmanager_unrequesttennisbanks(ph: *mut Phantom); }
extern "C" { pub fn f_021182_audspeechmanager_loadanimalbank_rage_animaltype_signed_char(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_021183_audspeechaudioentity_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021184_audpedscenario_debugdraw(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021186_audvehicleconductor_vehicleconductordebuginfo(ph: *mut Phantom); }
extern "C" { pub fn f_021187_audweaponaudioentity_processcachedweaponfireevents(ph: *mut Phantom); }
extern "C" { pub fn f_021188_audvehicleaudioentity_getopenness_bool(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_021189_audwaveslotmanager_debugdraw_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_021190_audradioaudioentity_findaudiblestation_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_021191_audscriptsound_audscriptsound_audscriptsound_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021192_audweaponaudioentity_handleplayerequippedweapon_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_021193_audweaponaudioentity_addallinventoryweaponsonped_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021194_audweaponfireinfo_findweaponsettings_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_021195_audweaponfireinfo_getweaponfiresound_rage_weaponsettings_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_021196_audweaponaudioentity_playweaponfireevent_audweaponfireevent_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021197_audweaponaudioentity_stopautomaticfire_audweaponfireinfo_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_021198_audnorthaudioengine_getcurrentaudiolevelname(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_021199_audnorthaudioengine_deactivateslowmomode_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021200_audnorthaudioengine_isafirstpersoncameraactive_cped_const_ptr_bo(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021201_audnorthaudioengine_issuperslowvideoeditor(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021202_audnorthaudioengine_isinslowmovideoeditor(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021203_audvehicleaudioentity_smashsiren_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021204_audspeechaudioentity_say_unsigned_int_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32, a8: f32, a9: i32, a10: i64, a11: i64, a12: i64) -> i32; }
extern "C" { pub fn f_021205_audspeechaudioentity_updatevfxdebugdrawstructs(ph: *mut Phantom); }
extern "C" { pub fn f_021206_audspeechaudioentity_updateambspeechdebugdrawstructs(ph: *mut Phantom); }
extern "C" { pub fn f_021207_audscannermanager_cleanupsentencesounds_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021208_audscannermanager_finishcopdispatchinteraction(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021209_audpolicescanner_reportplayercrime_rage_vector3_const_ref_ecrime(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_021210_audscannermanager_cancelallreports(ph: *mut Phantom); }
extern "C" { pub fn f_021211_audscannermanager_stopscannernoise(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021212_audscannermanager_preparesentence_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021213_rage_audsound_setrequestedpan_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021214_audscriptaudioentity_deactivatepolicescanneraudioscene(ph: *mut Phantom); }
extern "C" { pub fn f_021215_audnorthaudioengine_auditionsound_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021216_char_ptr_rage_safecatf_char_4096ul_char_ref_4096ul_char_const_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021217_audpedaudioentity_getbjvehicle(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_021218_audinteractivemusicmanager_ismusicplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021219_audscriptaudioentity_shouldduckforscriptedconversation(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021220_audnorthaudioengine_registerloudsound_rage_vector3_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021221_audsuperconductor_processmesage_conductors_conductormessagedata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021222_audsuperconductor_sendconductormessage_conductors_conductormessa(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021223_rage_auddebugdrawmanager_drawlinef_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_021224_audradiostation_getnextmixstationbeat_float_ref_float_ref_int_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_021227_audnorthaudioengine_mutecategory_audscene_ptr_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_021235_audscannermanager_initsentenceforscanner_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_021236_audscannermanager_getnextscannerslot_audscannersentence_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021237_audmusiceventmanager_triggerevent_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_021239_audnorthaudioengine_notifyopennetwork(ph: *mut Phantom); }
extern "C" { pub fn f_021240_audnorthaudioengine_notifyclosenetwork_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021241_audnorthaudioengine_createwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_021257_audinteractivemusicmanager_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021308_audweatheraudioentity_addweathertypewidgets_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021330_audmusicevent_init_char_const_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_021333_audobjectaudioentity_dtoraudobjectaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021339_audpedaudioentity_playanimtriggeredsound_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021340_audpedfootstepaudio_addfootstepevent_audfootstepevent(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021341_audpedfootstepaudio_playfootstepevent_audfootstepevent_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021342_bool_creplaymgrinternal_recordfx_cpacketfootstepsound_cpacketfoo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021343_audpedfootstepaudio_getcurrentmaterialsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021344_audpedaudioentity_validatemeleeweapon_unsigned_int_cped_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_021345_audspeechaudioentity_reallyinflictpain_auddamagestats_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021346_audpedaudioentity_triggerimpactsounds_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_021347_audspeechaudioentity_triggersinglecry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021348_audspeechaudioentity_playtennisvocalization_tennisvocaleffecttyp(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021349_audweaponaudiocomponent_getweaponsettings_centity_const_ptr_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_021350_audweaponaudioentity_getweaponsettings_rage_audmetadataref_unsig(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_021351_cped_shouldbedead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021352_audspeechaudioentity_playanimalvocalization_unsigned_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_021353_audpedaudioentity_startheartbeat_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021354_audspeechaudioentity_say_char_const_ptr_char_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i64) -> i32; }
extern "C" { pub fn f_021355_audscriptaudioentity_pausescriptedconversation_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_021356_audscriptaudioentity_restartscriptedconversation_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_021357_auddamagestats_auddamagestats_auddamagestats_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_021358_audspeechaudioentity_playpain_unsigned_int_auddamagestats_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_021359_auddamagestats_dtorauddamagestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021360_audspeechmanager_getwaveloadedpainvoicefromvoicetype_unsigned_in(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_021361_audspeechaudioentity_freeslot_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021362_audpedfootstepaudio_addslopedebrisevent_audfootstepevent_rage_ve(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32); }
extern "C" { pub fn f_021363_audpedaudioentity_triggerbonebreak_ragdollcomponent(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021364_audspeechaudioentity_getisanyspeechplaying(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021366_audspeechaudioentity_inflictpain_auddamagestats_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021367_audspeechaudioentity_updatepedheadmatrix(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021368_audspeechaudioentity_resetspeechsmoothers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021369_audspeechaudioentity_getspeechaudibilitymetrics_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i64) -> i32; }
extern "C" { pub fn f_021370_audspeechaudioentity_computepostsubmixvolume_audspeechtype_audsp(ph: *mut Phantom, a0: i32, a1: i32, a2: f32) -> f32; }
extern "C" { pub fn f_021371_rage_audsound_setrequestedpostsubmixvolumeattenuation_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_021372_rage_audsound_setrequestedvolumecurvescale_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_021373_rage_audsound_setrequestedhpfcutoff_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_021376_audpedaudioentity_dtoraudpedaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021379_audpedfootstepaudio_getshoesettings_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021380_audpedfootstepaudio_getupperbodyclothsounds_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021381_audpedfootstepaudio_getlowerbodyclothsounds_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021382_audpedfootstepaudio_getextraclothsounds_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_021384_audpedaudioentity_selectringtone(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021385_audspeechaudioentity_getpedvoicegroup_bool_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_021386_audspeechaudioentity_getpvghashfromrace_rage_pedracetopvg_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_021389_audspeechmanager_getambientspeechslot_audspeechaudioentity_ptr_b(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_021390_audspeechmanager_populateambientspeechslotwithplayingspeechinfo(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_021391_audspeechmanager_getambientwaveslotfromid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_021392_audspeechmanager_freeambientspeechslot_int_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021394_audpedfootstepaudio_playdebugfootstepevent_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_021396_audpedfootstepaudio_triggerfallinwatersplash_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021397_audpedaudioentity_playvehicleimpact_rage_vector3_const_ref_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_021398_audpedfootstepaudio_getglassiness_float(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_021400_rage_audsound_setrequestedpitch_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021403_audspeechaudioentity_stopspeech(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021404_audscriptaudioentity_finishscriptedconversation(ph: *mut Phantom); }
extern "C" { pub fn f_021405_audspeechaudioentity_ispainplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021406_audspeechaudioentity_isscriptedspeechplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021407_audspeechaudioentity_isambientspeechplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021414_audweaponaudiocomponent_playweaponfire_centity_ptr_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_021415_bool_creplaymgrinternal_recordfx_cpacketpedclothsound_cpacketped(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021416_creplaymgr_isrecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021417_audweaponaudioentity_israygun_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021421_audpedaudioentity_getplayerimpactmagoverride_float_centity_ptr_r(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64) -> f32; }
extern "C" { pub fn f_021422_cpedmotiondata_getisrunning_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021435_audfootstepeventtuning_addwidgets_rage_bkbank_ref_rage_atrangear(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021442_audstreamslot_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021445_audstreamslot_allocateslot_audstreamclientsettings_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021447_audpedfootstepaudio_getbodyclothsounds_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_021449_audpedfootstepaudio_iswearingscubasuit(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021454_audpedfootstepaudio_updatevariationdata_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021456_audpedfootstepaudio_playclothevent_audfootstepevent_rage_clothau(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32); }
extern "C" { pub fn f_021463_audpedfootstepaudio_changestandingmaterial_unsigned_long_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_021467_audvehiclecollisioncontext_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021468_audplaneaudioentity_dtoraudplaneaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021469_audplaneaudioentity_stopengine_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021470_audvehicleaudioentity_dtoraudvehicleaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021472_audplaneaudioentity_getplaneaudiosettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021473_audvehicleaudioentity_getvehiclemodelnamehash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021475_audvehicleaudioentity_getvehiclecollisionsettings_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021477_audvehicleaudioentity_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021479_audplaneaudioentity_setupvolumecurves(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021480_audvehicleaudioentity_getvehiclemodelname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021481_audvehicleaudioentity_setenvironmentgroupsettings_bool_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021483_audvehicleaudioentity_postupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021484_audvehicleaudioentity_triggersubmarinedivesound_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021485_audradioaudioentity_stopvehicleradio_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021486_audvehicleaudioentity_shouldrequestradio_rage_ambientradioleakag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_021487_audradioaudioentity_requestvehicleradio_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021488_audplaneaudioentity_isengineon_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021490_audwaveslotmanager_loadbankforsound_unsigned_int_unsigned_int_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: f32) -> i64; }
extern "C" { pub fn f_021491_audvehiclekerssystem_getkerssoundsetname_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021492_audvehicleaudioentity_requestwaveslot_audwaveslotmanager_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_021493_audvehicleaudioentity_requestsfxwaveslot_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021494_audvehicleaudioentity_freedspeffects(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021495_audwaveslotmanager_freewaveslot_auddynamicwaveslot_ptr_ref_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021498_audplaneaudioentity_adjustcontrolsurface_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_021502_audplaneaudioentity_startengine(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021503_audplaneaudioentity_updateengineon_audplanevalues_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021504_audplaneaudioentity_shutdownjetengine(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021505_audplaneaudioentity_triggerbackfire(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021506_audvehiclekerssystem_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021507_auddynamicwaveslot_isloaded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021508_audscriptaudioentity_triggeraircraftwarningspeech_audaircraftwar(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021509_rage_audsound_setupdateentity_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021510_rage_audsound_setclientvariable_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021511_audvehicleaudioentity_isseaplane_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021513_audvehicleaudioentity_arewarningsoundsvalid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021523_audvehicleaudioentity_shouldusedspeffects_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021530_audvehicleaudioentity_getwheelsoundupdateclientvar_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021531_audvehicleaudioentity_getwheelposition_unsigned_int_rage_vector3(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_021535_audspeechmanager_getnumvacantambientslots(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021537_audvehicleaudioentity_triggerdoorsound_unsigned_int_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021549_audpolicescanner_reportrandomcrime_rage_vector3_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021550_audpolicescanner_reportdispatch_unsigned_int_audunittype_rage_ve(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_021551_audpolicescanner_requestassistance_rage_vector3_const_ref_audsca(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021552_audpolicescanner_triggerrandomchat(ph: *mut Phantom); }
extern "C" { pub fn f_021553_audpolicescanner_describeposition_rage_vector3_const_ref_audposi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_021554_audpolicescanner_reportcrime_audcrimereport_const_ref_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021555_audscannermanager_populatesentencewithposdescription_audposition(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_021556_audpolicescanner_findspecificlocation_rage_vector3_const_ref_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_021557_audpolicescanner_chooseinstructionandcrimesound_rage_scannercrim(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_021558_audscannermanager_getnextsmallscannerslot_audscannersentence_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021559_audpolicescanner_reportpolicespottingplayer_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_021560_audpolicescanner_describevehicle_cvehicle_ptr_audvehicledescript(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_021561_audscannermanager_triggercopdispatchinteraction_scannercopdispat(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021562_audpolicescanner_reportpolicespottingnonplayer_cped_const_ptr_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_021564_audradioaudioentity_dtoraudradioaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021565_auduserradiotrackmanager_loadfilelookup(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021567_auduserradiotrackdatabase_dtorauduserradiotrackdatabase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021568_rage_atrangearray_rage_audenvironmentgamemetric_9_atrangearray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021569_audradiostation_unlocktracklist_rage_radiostationtracklist_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_021570_audradiostation_locktracklist_rage_radiostationtracklist_const_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_021571_audradiostation_getstation_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_021572_audradiostation_forcemusictracklist_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021573_audradiostation_randomizesequentialstationplayposition(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021575_audradiotrack_findtextidforsoundhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021576_audradioaudioentity_getreplaybeat_unsigned_int_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: f32) -> f32; }
extern "C" { pub fn f_021577_audinteractivemusicmanager_parsemarkers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021578_audradiotrack_isscoretrack_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021579_audradiotrack_getbeatsoundname_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021580_audradioaudioentity_stopreplaymusic_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021581_audinteractivemusicmanager_requestreplaymusic_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: f32); }
extern "C" { pub fn f_021582_audinteractivemusicmanager_playloopingtrack_unsigned_int_rage_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32, a4: i32, a5: i32, a6: f32, a7: f32, a8: i32, a9: i32); }
extern "C" { pub fn f_021583_audradioaudioentity_updatereplaymusicvolume_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021584_audinteractivemusicmanager_updatereplaymusicvolume_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021586_audreplayaudioentity_stopallambient_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021587_audreplayaudioentity_stop_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_021588_audreplayaudioentity_preload_int_unsigned_int_audreplaystreamtyp(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_021589_audreplayaudioentity_isloaded_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_021590_audreplayaudioentity_isplaying_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_021591_audreplayaudioentity_play_int_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_021592_audinteractivemusicmanager_updatereplaymusicpreview_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021594_rage_simplesound_const_ptr_rage_audsoundfactory_decompressmetada(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_021597_audvehicleaudioentity_gethasnormalradio_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021599_audradioaudioentity_retunetostation_audradiostation_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021600_audradiostation_findstation_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021601_audradiostation_computenumtracksavailable_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_021602_audradiostation_gettrack_int_unsigned_int_int_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_021603_audradiostation_getmusictracklistactivetrackindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021604_audradiostation_getmusictracklisttrackstarttimems_unsigned_int_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_021605_audradiostation_getmusictracklisttrackfortimeoffset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_021606_audradiostation_findhistoryforcategory_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_021608_audradiostation_findstation_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_021611_audradioslot_update_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021612_audradioaudioentity_preupdatereplaymusic_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021613_audradioaudioentity_chooserandomradiostationforgenre_rage_radiog(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021614_audradioslot_requestradiostationforemitter_audradiostation_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_021615_audradiostation_updateretunesounds(ph: *mut Phantom); }
extern "C" { pub fn f_021616_audradioslot_findandstopemitter_audradioemitter_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021617_audradioslot_findandretuneemitter_audradioemitter_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021618_audradioslot_findslotbyemitter_audradioemitter_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021619_audradioslot_findandskipforwardemitter_audradioemitter_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021620_audradioaudioentity_setmobilephoneradiostate_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021621_audradioaudioentity_startmobileradio(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021622_audradioslot_calculatepriority(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021623_audradioslot_addemitter_audradioemitter_ptr_bool_int_audradiosta(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_021624_audradioslot_findfreeslot_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_021625_audradioslot_findslottooverride_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_021626_audradioslot_overrideemitters_audradioemitter_ptr_audradiostatio(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_021630_audradioslot_removeemitter_audradioemitter_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_021633_audradioaudioentity_requeststaticradio_audstaticradioemitter_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_021634_audradioaudioentity_stopstaticradio_audstaticradioemitter_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021636_audradiostation_skipforwardstations_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021637_audradiostation_skipforward_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021641_audradioaudioentity_retunetostation_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021642_audradioaudioentity_playstationselectsound_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021644_audradioaudioentity_getaudibletracktextid_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021648_audradiostation_forcetrack_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021649_audradiostation_unlocktracklist_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_021650_audradiostation_forcetrack_rage_radiotracksettings_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_021651_audradiostationtracklistcollection_gettrack_int_unsigned_int_boo(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_021653_audscriptaudioentity_preloadstreaminternal_char_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_021656_audscriptaudioentity_playstreamfrontendinternal(ph: *mut Phantom); }
extern "C" { pub fn f_021657_audscriptaudioentity_playstreamfromentityinternal_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021684_audvehicleaudioentity_getambientradiovolume_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_021690_audradioslot_allocatestreamslot_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021691_audradiotrack_muteemitters(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021692_audradiostation_setphysicalstreamingstate_bool_audstreamslot_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_021694_audradiostationhistory_addtohistory_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021695_audradiotrack_updatestereoemitter_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_021696_audradiotrack_updatepositionedemitter_unsigned_int_float_float_u(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32, a9: f32); }
extern "C" { pub fn f_021701_auduserradiotrackmanager_getdatafilelocation_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_021704_audradiostationtracklistcollection_computenumtracks_unsigned_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021705_audradiostation_randomizecategorystride_rage_trackcats_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_021706_audradiostation_resethistoryfornetworkgame(ph: *mut Phantom); }
extern "C" { pub fn f_021707_audradiostation_syncstation_audradiostation_audradiostationsyncd(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021708_audradiotrack_init_int_int_unsigned_int_float_bool_bool_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: f32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_021709_audradiostation_preparenexttrack(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021710_audradiotrack_prepare_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_021711_audradiostation_populatesyncdata_audradiostation_audradiostation(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021714_audradiostation_initialisenexttrack_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021715_audradiotrack_play(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021716_audradiotrack_update_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021717_audradiostation_createcompatibledjspeech_unsigned_int_unsigned_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_021718_audradiostation_findcompatibledjspeechvariation_char_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_021719_audradiostation_createdjspeechsounds_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021720_audradiostation_randomizecategorystride_rage_trackcats_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_021721_audradiostation_skiptooffsetinmusictracklist_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021722_audradiostation_computerandomtrack_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_021723_audradiotrack_createshadowemittersounds_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_021724_audradiostation_resolveprobability_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_021725_audradiostation_isvalidcategoryforstation_rage_trackcats_bool_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_021726_audradiostation_findtrackid_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_021727_audradiostation_forcetrack_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_021731_audradiostation_queuetracklist_rage_radiostationtracklist_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_021732_audradiostation_clearqueuedtracks(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021733_audradiostation_getmusictracklisttrackforindex_unsigned_int_cons(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_021740_auduserradiotrackmanager_computetrackmakeupgain_int_const(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_021747_audreplayaudioentity_updateplay_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021748_audreplayaudioentity_updatevolume_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021749_audreplayaudioentity_updatefree_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021751_audreplayaudioentity_updatepreload_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021752_audreplayaudioentity_playinternal_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021759_audscannermanager_issentencestillvalid_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021760_audscannermanager_getvoicehashforcopspeech(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021761_audspeechaudioentity_saywhensafe_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_021762_audscannermanager_playcoptodispatchline(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021763_audspeechaudioentity_getrandomvariation_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_021764_audscannermanager_startscannernoise_bool_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_021766_audscannermanager_decrementhelisinchase(ph: *mut Phantom); }
extern "C" { pub fn f_021767_audscannermanager_triggerscriptedreport_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_021769_audspeechaudioentity_audspeechaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021771_audscriptaudioentity_dtoraudscriptaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021772_audscript_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021773_audspeechaudioentity_dtoraudspeechaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021775_audscriptaudioentity_playmobileprering_unsigned_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_021777_audspeechmanager_setplayerpainrootbankname_cped_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021778_audspeechmanager_getvoiceforphoneconversation_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021779_audspeechmanager_getphoneconversationvariationfromvoicehash_unsi(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021780_audspeechmanager_removereferencetovoice_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_021781_audspeechmanager_getpossibleconversationtopicsforpvg_unsigned_in(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021782_audscriptaudioentity_initaircraftwarningravesettings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021783_audspeechmanager_registerpedaimedatorhurt_unsigned_char_bool_boo(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_021784_audspeechaudioentity_stopambientspeech(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021785_audspeechaudioentity_playpreloadedspeech_fromscript(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021786_audspeechaudioentity_killpreloadedspeech_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021787_audspeechaudioentity_playplayerbreathing_playervocaleffecttype_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_021788_audspeechaudioentity_triggerreplyspeech_cped_ptr_unsigned_int_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_021789_audspeechaudioentity_choosevoiceifundecided_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021790_audspeechaudioentity_isincarfrontendscene_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021791_audspeechaudioentity_stoppain(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021792_audspeechaudioentity_ispreloadedspeechready(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021793_audspeechaudioentity_playpreloadedspeech_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021794_audspeechaudioentity_triggersharkattacksound_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_021795_audspeechaudioentity_completesay(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021796_bool_creplaymgrinternal_recordfx_cpacketspeech_cpacketspeech_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021797_audspeechaudioentity_getpositionforspeechsound_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_021798_audscriptaudioentity_showsubtitles_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021799_audscriptaudioentity_shouldwepausescriptedconversation_unsigned(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_021800_audscriptaudioentity_handlecalltopreloading_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_021801_audscriptaudioentity_abortscriptedconversation_bool_char_const_p(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_021802_audspeechaudioentity_isscriptedspeechplaying_int_ptr_int_ptr_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_021803_audspeechaudioentity_getlastmarkerforcategoryid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_021804_audscriptaudioentity_iscontextansfxcommand_char_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021805_audspeechaudioentity_orphanscriptedspeechecho(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021806_audscriptaudioentity_getpausetimeforconversationline_char_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021807_audscriptaudioentity_getoverlapoffsettime_char_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021808_audspeechaudioentity_computepositionedheadsetvolume_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> f32; }
extern "C" { pub fn f_021809_audspeechaudioentity_playscriptedspeech_char_const_ptr_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64, a10: i32, a11: i32, a12: i32, a13: i64, a14: i32, a15: i32); }
extern "C" { pub fn f_021811_void_creplaymgrinternal_recordpersistantfx_cpacketscriptsetvaron(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021813_audscriptaudioentity_removescript_rage_scrthreadid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021814_audscriptsound_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021815_audscriptaudioentity_lognetworkscriptbankrequest_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_021816_audscriptaudioentity_getindexfromscriptthreadid_rage_scrthreadid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_021817_audscriptaudioentity_logscriptbankrequest_char_const_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_021818_audscriptaudioentity_triggermissioncomplete_char_const_ptr_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021821_audscriptaudioentity_addscript(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_021823_bool_creplaymgrinternal_recordfx_cpacketspeechstop_cpacketspeech(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021824_audspeechaudioentity_stopplayingscriptedspeech(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021825_audscriptaudioentity_preloadscriptedspeech_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021826_audscriptaudioentity_getlipsyncpredelay_tscriptedspeechslots_ref(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_021827_audspeechaudioentity_processclipdictionaryresource_void_ptr_unsi(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021836_audscriptaudioentity_debugscriptedconversation(ph: *mut Phantom); }
extern "C" { pub fn f_021842_audscriptaudioentity_pickrandominterrupterinvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021843_audspeechaudioentity_getconversationinterruptionoffset_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_021845_audspeechaudioentity_playanimalnearvocalization_unsigned_int_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_021846_audspeechaudioentity_playanimalfarvocalization_unsigned_int_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_021847_audscriptaudioentity_registerscriptrequestedinterrupt_cped_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_021848_audscriptaudioentity_isscriptbankloaded_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021853_audscriptaudioentity_onflagstatechange_audscriptaudioflags_flagi(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_021854_audscriptaudioentity_requestscriptbank_char_const_ptr_bool_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_021855_audscriptaudioentity_requestscriptbank_unsigned_int_bool_unsigne(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_021856_audscriptaudioentity_logscriptbanks(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021857_audscriptaudioentity_scriptbanknolongerneeded_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021858_bool_creplaymgrinternal_recordfx_cpacketsndloadscriptwavebank_cp(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021860_audscriptaudioentity_cleanupscriptaudioentityforreplay(ph: *mut Phantom); }
extern "C" { pub fn f_021865_audscriptaudioentity_playsoundandpopulatescriptsoundslot_int_uns(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32, a9: i32, a10: i64, a11: i64) -> i64; }
extern "C" { pub fn f_021866_audvehicleaudioentity_allocatevehiclevariableblock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021871_audscriptaudioentity_setplayerangry_audspeechaudioentity_ptr_boo(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021873_audscriptaudioentity_playsoundfromentity_int_unsigned_int_centit(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64); }
extern "C" { pub fn f_021879_audscriptaudioentity_stopsoundfromnetwork_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021885_audscriptaudioentity_getscript_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_021887_audscript_startscene_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021895_audweaponaudioentity_loaddynamicweaponaudio_audplayerweaponslot(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021904_audscriptaudioentity_playstreamfrompositioninternal_rage_vector3(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021911_audscriptaudioentity_clearplayedlist(ph: *mut Phantom); }
extern "C" { pub fn f_021913_audmusiceventmanager_cancel_audmusicevent_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_021914_audscript_setaudioflag_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_021920_audspeechmanager_getbestvoice_unsigned_int_unsigned_int_cped_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_021921_audspeechmanager_getbestminivoice_rage_pedvoicegroups_ptr_int_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_021922_audspeechmanager_getbestgangvoice_rage_pedvoicegroups_ptr_int_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_021923_audspeechmanager_getbestprimaryvoice_rage_pedvoicegroups_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_021924_audspeechmanager_resolvevirtualspeechcontext_rage_speechcontext(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_021928_audspeechaudioentity_doescontextexistforthisped_unsigned_int_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_021930_audspeechaudioentity_setpedvoiceforcontext_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_021931_audspeechaudioentity_canpedhavechatconversationwithped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_021932_audspeechmanager_addvariationtohistory_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_021933_bool_creplaymgrinternal_recordfx_cpacketscriptedspeech_cpacketsc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021935_audspeechmanager_getspeechcontext_unsigned_int_cped_ptr_cped_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_021936_audspeechaudioentity_addambspeechdebugdrawstruct_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_021948_audspeechaudioentity_triggerpainfrompainid_int_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021949_audspeechaudioentity_say_unsigned_int_audspeechinitparams_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32, a8: f32, a9: i64, a10: i64, a11: i64) -> i32; }
extern "C" { pub fn f_021950_audspeechaudioentity_computeechoinitmetrics_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_021951_audspeechaudioentity_addvfxdebugdrawstruct_char_const_ptr_cped_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021952_audspeechaudioentity_createshadow_rage_audmetadataref_rage_audso(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_021953_audspeechaudioentity_createecho_audechometrics_const_ref_audspee(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: i32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i64, a12: i64); }
extern "C" { pub fn f_021954_audspeechaudioentity_computeechometrics_rage_vec3v_const_ref_aud(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32, a5: f32, a6: i64); }
extern "C" { pub fn f_021955_bool_creplaymgrinternal_recordfx_cpacketscriptedspeechupdate_cpa(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021958_audspeechaudioentity_attemptsaywithbackupcontext_audspeechinitpa(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32, a7: f32) -> i32; }
extern "C" { pub fn f_021959_audspeechmanager_getrepeattimeprimarytriggeredcontextcanplayonvo(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021960_audspeechmanager_updatetimebackuptriggeredcontextwaslastused_rag(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021962_audspeechaudioentity_triggerjackingspeech(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021964_audspeechaudioentity_registerfallingstart(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021965_audspeechaudioentity_gethighdeathpaintypefromdistance_float(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_021966_void_creplaymgr_recordfx_cpacketanimalvocalization_cpacketanimal(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021969_audspeechmanager_dtoraudspeechmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021971_audspeechmanager_loadtennisbanks(ph: *mut Phantom); }
extern "C" { pub fn f_021973_audspeechmanager_freeallambientspeechslots(ph: *mut Phantom); }
extern "C" { pub fn f_021974_audstreamslot_cleanupslotsforreplay(ph: *mut Phantom); }
extern "C" { pub fn f_021975_audvehicleconductorfakescenes_playfakedistancesirens(ph: *mut Phantom); }
extern "C" { pub fn f_021976_audvehicleconductor_stopstuntjump(ph: *mut Phantom); }
extern "C" { pub fn f_021979_audtraileraudioentity_gettraileraudiosettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_021982_audvehicleaudioentity_getclattersoundfromtype_rage_clattertype(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_021986_audvehicleaudioentity_setlod_audvehiclelod(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021987_audvehiclefireaudio_stopallsounds(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021988_audvehicleaudioentity_freewaveslot_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022001_audtrainaudioentity_starttrainsound_unsigned_int_rage_audsound_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_022007_wchartoutf8path_wchar_t_const_ptr_char_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022012_audvehicleaudioentity_shutdownprivate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022013_audvehiclereflectionsentity_detachfromvehicle_audvehicleaudioent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022014_audvehiclecollisioncontextlist_clearreuselist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022022_audvehicleaudioentity_getcachedwheelposition_unsigned_int_rage_v(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_022023_rage_fwentity_getskeletondata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022024_audvehicleaudioentity_reapplydspeffects(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022025_audwaveslotmanager_loadbank_unsigned_int_unsigned_int_bool_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: f32) -> i64; }
extern "C" { pub fn f_022026_audvehicleaudioentity_requestlowlatencywaveslot_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022027_audvehicleaudioentity_computeeffectiveenginehealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_022028_audvehicleaudioentity_updatemissilelock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022029_char_ptr_rage_safecatf_char_1024ul_char_ref_1024ul_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_022031_audvehicleaudioentity_computeclosestpositiononvehicleyaxis(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022032_audvehicleaudioentity_initsubmersible_rage_boataudiosettings_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022033_audvehicleengine_qualitysettingchanged(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022034_audgranularenginecomponent_init_audvehicleaudioentity_ptr_audgra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_022035_audvehicleenginecomponent_stop_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022036_audgranularenginecomponent_stop_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022038_audvehicleaudioentity_updateclatter_audvehiclevariables_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022039_audvehicleaudioentity_computeeffectivebodydamagefactor_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_022040_audvehiclecollisionaudio_getvehiclecollisionsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022041_audvehicleaudioentity_triggerclatter_rage_audmetadataref_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_022042_audvehiclecollisionaudio_triggerdebrissounds_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_022043_audvehicleaudioentity_calculatewheelmaterials_audvehiclevariable(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022044_audvehicleaudioentity_stoproadnoisesounds(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022045_audvehicleaudioentity_triggerlandinggearsound_rage_audsoundset_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_022046_audvehicleaudioentity_playhelddownhorn_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_022049_audvehicleaudioentity_getvehiclehydraulicssoundsetname_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022050_audvehicleaudioentity_shouldforceengineoff_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022051_audvehicleaudioentity_arehydraulicsactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022054_audvehicleaudioentity_triggersimplesoundfromsoundset_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022058_audvehicleaudioentity_smoothsirenvolume_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_022059_audvehicleaudioentity_isplayeronfootoronbicycle(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_022060_void_audvehicleaudioentity_updateloopwithlinearvolumeandpitch_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32, a5: i64, a6: i64, a7: i32, a8: f32, a9: i32); }
extern "C" { pub fn f_022061_audvehicleaudioentity_isanywatervfxactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022067_audvehicleaudioentity_addcommondspparameters_audvehicleaudioenti(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022068_audvehicleaudioentity_getengineupgradelevel_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_022069_audvehicleaudioentity_getexhaustupgradelevel_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_022070_audvehicleaudioentity_gethornloopref_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_022071_audvehicleaudioentity_triggeralarm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022074_audvehicleaudioentity_getinteriorsoundocclusion_float_ref_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_022075_audvehicleaudioentity_fix(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022077_rage_fwentity_getobjectmtx_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022081_audvehicleaudioentity_setisincarmodshop_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022083_audvehicleaudioentity_playaggressivehorn(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022085_audvehicleaudioentity_updatesubmersible_audvehiclevariables_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_022086_audvehicleaudioentity_triggerstuntracespeedboost(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022087_audvehicleaudioentity_triggerstuntraceslowdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022094_audvehicleaudioentity_triggerheadlightsmash_unsigned_int_rage_ve(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_022097_audvehicleaudioentity_blipsiren(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022103_audvehicleaudioentity_triggerdynamicexplosionsounds_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022104_audvehicleaudioentity_forceusegameobject_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022106_audvehicleaudioentity_skipto(ph: *mut Phantom); }
extern "C" { pub fn f_022112_audvehicleaudioentity_formatcraudiosettingsxml_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022121_audvehiclecollisioncontextlist_initandaddeventtolist_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_022126_audvehiclecollisionaudio_getvehiclecollisionmaterialsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022127_audvehicleconductor_playstuntjumpcollision_float(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_022129_audvehiclecollisionaudio_triggerjumpland_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_022130_audvehiclecollisionaudio_triggerjumplandscrape(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022137_rage_atrangearray_audvehicleconductor_vehssireninfo_2_dtoratrang(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022143_audvehicledigger_dtoraudvehicledigger(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022152_audvehicleforks_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022153_audvehicletowtruckarm_dtoraudvehicletowtruckarm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022167_audvehicleconvertibleroof_stopallsounds(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022170_audvehicleconvertibleroof_updatevolumes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022191_audvehiclereflectionsentity_dtoraudvehiclereflectionsentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022194_audvehiclereflectionsentity_updateinteriorreflectionsvoice_unsig(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022195_audvehiclereflectionsentity_calculatetunneldirection_cinteriorin(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022196_audvehiclereflectionsentity_removevehiclereflections_audvehiclea(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022218_audweaponfireinfo_populatepresuckinfo_rage_weaponsettings_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_022219_audweaponaudioentity_getinteriormetrics_audweaponinteriormetrics(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022222_audweaponspinevent_handleweaponspin(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022223_audprojectilecookevent_handlecookprojectile(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022224_audweaponaudioentity_playspinupsound_centity_const_ptr_rage_auds(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_022226_void_creplaymgrinternal_recordpersistantfx_cpacketweaponspinup_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_022230_bool_creplaymgrinternal_recordfx_cpacketthermalscopeaudio_cpacke(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022232_audweaponaudioentity_playweaponreport_audweaponfireinfo_ptr_audw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_022233_audweaponaudioentity_playtwoechos_unsigned_int_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32, a5: i32, a6: i32, a7: i64); }
extern "C" { pub fn f_022234_bool_creplaymgrinternal_recordfx_cpacketsndloadweapondata_cpacke(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022235_cpedinventory_getweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022236_audweaponaudioentity_getnextweapon_cped_ptr_unsigned_int_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022237_audweaponaudioentity_startautomaticfire_audweaponfireinfo_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_022243_audweaponaudioentity_getweaponsettings_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_022245_audweaponaudiocomponent_setlastimpacttime(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022252_audweatheraudioentity_getrainverticallatt_const(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_022253_audvehicleenginecomponent_prepare(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022254_audvehicleenginecomponent_play(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022255_audvehicleenginecomponent_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022256_audvehicleenginecomponent_update_audvehiclevariables_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022257_audgranularenginecomponent_play(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022258_audgranularenginecomponent_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022259_audgranularenginecomponent_update_audvehiclevariables_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_022263_audvehicleengine_initcommon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022264_audvehicleengine_update_audvehiclevariables_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022265_audvehicleengine_checkforenginestartstop_audvehiclevariables_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_022266_audvehicleengine_stopengine_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022267_audvehicleengine_getengineswitchfadetime_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022268_audvehicleengine_setgranularpitch_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022269_audvehicleengine_converttodummy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022270_audvehicleengine_convertfromdummy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022273_audinteractivemusicmanager_computenextplaytimeforconstraints_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> f32; }
extern "C" { pub fn f_022281_audmusicactionfactory_create_rage_audmetadataobjectinfo_const_re(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022283_audmusiceventmanager_prepareevent_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_022285_audinteractivemusicmanager_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022289_audinteractivemusicmanager_onreplayeditoractivate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022290_audinteractivemusicmanager_internalcrossfadecategoriestomood_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_022291_audwantedmusic_triggerevent_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022292_audvehiclemusic_startmusic(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022293_audvehiclemusic_stopmusic_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022294_audinteractivemusicmanager_stoponeshot_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022295_audinteractivemusicmanager_internalsetmood_rage_interactivemusic(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_022296_audinteractivemusicmanager_crossfadetostemmix_rage_stemmix_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_022297_audinteractivemusicmanager_internalsnapcategoriestomood_rage_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_022298_audinteractivemusicmanager_preloadoneshot_unsigned_int_int_int_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_022319_audcachedcarlandingtestresults_dtoraudcachedcarlandingtestresult(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022358_audvoicechataudioentity_dtoraudvoicechataudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022374_audioclipdictionary_dtoraudioclipdictionary(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022380_bool_creplaymgrinternal_handleentityforfx_cpacketsoundphonering(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_022381_void_creplayeventmanager_storeevent_cpacketsoundphonering_cpacke(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022382_cpacketsoundphonering_ptr_creplaypacketbuffer_recordpacket_cpack(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022383_bool_creplaymgrinternal_handleentityforfx_cpacketpedimpactpacket(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_022384_bool_creplaymgrinternal_handleentityforfx_cpacketpedstandingmate(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_022386_naaudioentity_naaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022388_naaudioentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022391_naenvironmentgroup_allocate_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022392_naenvironmentgroup_debugprintenvironmentgrouppool(ph: *mut Phantom); }
extern "C" { pub fn f_022395_naaudioentity_processanimevents(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022397_naaudioentity_createdeferredsound_rage_audmetadataref_centity_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_022398_naaudioentity_createdeferredsound_unsigned_int_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_022399_naaudioentity_processdeferredsounds(ph: *mut Phantom); }
extern "C" { pub fn f_022400_audcachedlos_clear(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022401_audcachedlos_addprobe_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_022402_audcachedlos_getproberesult(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022404_audcachedlosmultipleresults_audcachedlosmultipleresults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022405_audocclusionintersections_audocclusionintersections(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022407_audbicycleaudioentity_getbicycleaudiosettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022416_rage_audsound_setrequestedvolume_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_022428_audboataudioentity_getboataudiosettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022443_rage_audsound_getrequestedposition_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022463_rage_audsound_setrequestedposition_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022464_rage_audsound_setreleasetime_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022480_audcaraudioentity_getcaraudiosettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022491_audcaraudioentity_setupvolumecones(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022500_audcaraudioentity_canbedescribedasacar(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022507_void_audvehicleaudioentity_updateloopwithlinearvolumeandpitch_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32, a5: i64, a6: i64, a7: i32, a8: i32, a9: f32, a10: i32); }
extern "C" { pub fn f_022508_audcaraudioentity_triggerbrakerelease_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_022509_rage_audsound_setrequestedlistenermask_unsigned_char(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022510_audcaraudioentity_sethydraulicsuspensionactive_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022522_naenvironment_areweinaninterior_bool_ptr_float_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022523_audcaraudioentity_isengineupgraded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022524_audcaraudioentity_istransmissionupgraded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022525_audcaraudioentity_isexhaustupgraded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022526_audcaraudioentity_isturboupgraded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022601_audcollisionevent_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022603_audcollisionaudioentity_dtoraudcollisionaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022604_audcollisionevent_dtoraudcollisionevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022610_audcollisionaudioentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022611_audcollisionaudioentity_populatemodelsettingscreator_cbasemodeli(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_022612_audcollisionaudioentity_getmaterialoverride_centity_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_022614_auddynamicmixer_startscene_rage_mixerscene_const_ptr_audscene_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_022615_audscene_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022616_audcollisionaudioentity_getaudioweight_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022617_audcollisionaudioentity_computepitchandvolumefromscrapespeed_aud(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_022618_audcollisionaudioentity_getocclusiongroup_audcollisionevent_cons(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022619_void_creplaymgrinternal_recordpersistantfx_cpacketcollisionplayp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_022620_audcollisionaudioentity_computepitchandvolumefromrollspeed_audco(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_022621_auddynamicmixer_startscene_unsigned_int_audscene_ptr_ptr_audscri(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_022622_void_creplaymgrinternal_recordpersistantfx_cpacketdynamicmixersc(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_022623_void_creplaymgrinternal_stoptrackingfx_replaytrackedscene_ctrack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022624_auddynmixpatch_remove(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022625_audcollisionaudioentity_processmacsmodeloverridesforchunk_unsign(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_022626_rage_atmapentry_unsigned_int_unsigned_int_ref_rage_atmap_unsigne(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_022627_audcollisionaudioentity_removemacsmodeloverridesforchunk_unsigne(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_022629_audcollisionaudioentity_spawnravesettings(ph: *mut Phantom); }
extern "C" { pub fn f_022630_spawnmodel_cbasemodelinfo_ptr_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022631_audcollisionaudioentity_processimpactsounds_audcollisionevent_pt(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022632_audcollisionaudioentity_getaudioweightscaling_rage_audioweight_r(ph: *mut Phantom, a0: i32, a1: i32) -> f32; }
extern "C" { pub fn f_022633_audcollisionaudioentity_createmodelaudiosettings(ph: *mut Phantom); }
extern "C" { pub fn f_022634_audcollisionaudioentity_formatmacsxml_char_ptr_char_const_ptr_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_022635_audcollisionaudioentity_aremacsmaterialsoutofdate_rage_modelaudi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022636_audcollisionaudioentity_findnewmodels(ph: *mut Phantom); }
extern "C" { pub fn f_022637_audcollisionaudioentity_abortworldcollisionlistgeneration(ph: *mut Phantom); }
extern "C" { pub fn f_022638_audcollisionaudioentity_findnewmodels_rage_fistream_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022639_audcollisionaudioentity_finddefaultmaterialmodels_rage_fistream(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022640_audcollisionaudioentity_getfragcomponentmaterialsettings_rage_mo(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022642_audcollisionaudioentity_getocclusiongroup_rage_vec3v_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_022643_naenvironmentgroup_setinteriorinfowithinteriorlocation_rage_fwin(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022645_audcollisionaudioentity_reportmeleecollision_worldprobe_cshapete(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_022646_audcollisionaudioentity_triggerdeferredmeleeimpact_worldprobe_cs(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32); }
extern "C" { pub fn f_022647_audcollisionaudioentity_handleentityswinging_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022648_audcollisionaudioentity_populateaudiocollisioneventfromintersect(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022653_audcollisionaudioentity_getrollcollisioneventfromhistory_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_022654_audcollisionaudioentity_handleplayerevent_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022655_audcollisionaudioentity_getfreecollisioneventfromhistory_unsigne(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022657_audcollisionaudioentity_triggerdeferredbulletimpact_rage_audsoun(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i64, a6: i64, a7: i32, a8: i32, a9: i64); }
extern "C" { pub fn f_022658_cped_isdead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022659_naenvironmentgroup_init_naaudioentity_ptr_float_unsigned_int_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_022660_naenvironmentgroup_setsource_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022661_naenvironmentgroup_setinteriorinfowithentity_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022662_naenvironment_getinteriorsettingsforinteriorlocation_rage_fwinte(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_022663_audcollisionaudioentity_processscrapesounds_audcollisionevent_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022664_audcollisionaudioentity_isscrapefrommagnitudes_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> i32; }
extern "C" { pub fn f_022681_audcuttracksynchedscene_dtoraudcuttracksynchedscene(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022684_audcutsceneaudioentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022685_audcutsceneaudioentity_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022686_audcutsceneaudioentity_stop_bool_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_022687_audcuttracksynchedscene_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022689_audcutsceneaudioentity_prepare_audcutsceneevent_ref_cutsceneaudi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_022692_audcutsceneaudioentity_play_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022694_audcuttrackcutscene_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022695_audcuttracksynchedscene_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022696_bool_creplaymgrinternal_recordfx_ccutsceneaudiopacket_ccutscenea(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022697_audcuttrackcutscene_handleplay(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022698_audcuttracksynchedscene_getscenetime_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_022699_bool_creplaymgrinternal_recordfx_csynchsceneaudiopacket_csynchsc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022700_audcuttracksynchedscene_trigger_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022701_audcuttrackcutscene_trigger(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022704_audcuttrackcutscene_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022706_audcuttracksynchedscene_init_char_const_ptr_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_022710_audcutsceneaudioentity_stopcutscene_bool_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_022711_audscene_setvariablevalue_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_022712_audcuttrackcutscene_stop_bool_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_022713_bool_creplaymgrinternal_recordfx_ccutscenestopaudiopacket_ccutsc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022714_audcutsceneaudioentity_stopoverunaudio(ph: *mut Phantom); }
extern "C" { pub fn f_022716_bool_creplaymgrinternal_recordfx_csynchscenestopaudiopacket_csyn(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022718_audcuttrackcutscene_gettracktime_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_022723_audcuttrackcutscene_skiptotime_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022734_auddooraudioentity_createdoorenvironmentgroup_cdoor_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022735_rage_audsoundinitparams_setvariablevalue_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_022736_auddooraudioentity_findsound_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_022737_auddooraudioentity_processaudio_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_022745_auddynmixpatch_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022747_auddynmixpatch_initialize_rage_audmixgroup_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022748_auddynmixpatch_getapplyfactor(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_022749_auddynmixpatch_recalculatecategoriesusingapply_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_022750_audscene_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022751_audscene_setvariablevalue_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_022752_audscene_getpatch_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022753_audscene_isfullyappliedforpause(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022754_audscene_isinstanceof_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_022756_auddynamicmixer_dtorauddynamicmixer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022757_auddynamicmixer_shutdownsession(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022759_auddynamicmixer_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022760_auddynamicmixer_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022761_auddynamicmixer_addentitytomixgroup_centity_ptr_unsigned_int_flo(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_022762_auddynamicmixer_removeentityfrommixgroup_centity_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_022763_auddynamicmixer_stopscenesfromchunk_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022764_auddynamicmixer_getactivescenefromhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_022765_auddynamicmixer_update(ph: *mut Phantom); }
extern "C" { pub fn f_022766_auddynamicmixer_finishupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022767_auddynamicmixer_printdebug_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022781_staticemitterentry_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022783_audemitteraudioentity_dtoraudemitteraudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022784_staticemitterentry_getrequestedpcmchannel_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022785_audemitteraudioentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022786_audemitteraudioentity_addbuildinganimevent_unsigned_int_rage_vec(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_022787_audemitteraudioentity_addstaticemitter_unsigned_int_unsigned_sho(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_022788_audemitteraudioentity_reinitstaticemitters(ph: *mut Phantom); }
extern "C" { pub fn f_022789_audemitteraudioentity_stopemitter_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022790_audemitteraudioentity_createstaticemitters(ph: *mut Phantom); }
extern "C" { pub fn f_022791_audemitteraudioentity_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022793_audemitteraudioentity_shouldstaticemitterbeplaying_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_022794_audemitteraudioentity_createstaticemitterenvironmentgroup_rage_s(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_022795_audemitteraudioentity_setstaticemitterinteriorinfo_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022798_audemitteraudioentity_shouldentityemitterbeplaying_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_022799_audemitteraudioentity_computeentityemitterconeattenuation_unsign(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_022800_naenvironmentgroup_updatesourceenvironment_unsigned_int_centity(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_022801_naenvironmentgroup_setinteriorinfowithinteriorgo_rage_interiorse(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_022802_audambientzone_computezoneinfluenceratioatpoint_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_022803_naenvironmentgroup_setinteriorinfowithinteriorinstance_cinterior(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_022804_naenvironment_getinteriorsettingsforinterior_cinteriorinst_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_022808_audemitteraudioentity_notifyloudsound(ph: *mut Phantom); }
extern "C" { pub fn f_022809_naenvironmentgroup_forcesourceenvironmentupdate_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022810_audambientzone_computeclosestpointtoposition_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_022811_audambientzone_computeclosestpointonboundary_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32); }
extern "C" { pub fn f_022812_audemitteraudioentity_setemitterradiostation_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022814_audemitteraudioentity_setemitterenabled_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_022820_audambientaudioentity_updateemittertocurrentcoords_char_const_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022823_audplaceabletracker_calculateorientation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022828_audexplosionaudioentity_reallyplayexplosion_eexplosiontag_rage_v(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_022829_bool_creplaymgrinternal_recordfx_cpacketexplosion_cpacketexplosi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022830_naenvironment_registerdeafeningsound_rage_vector3_ref_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_022832_naenvironment_updateshockwave_int_ref_audshockwave_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022834_audexplosionaudioentity_playtaillimitedsound_unsigned_int_float(ph: *mut Phantom, a0: i32, a1: f32, a2: i64); }
extern "C" { pub fn f_022835_audexplosionaudioentity_playcardebrissound_unsigned_int_float_ra(ph: *mut Phantom, a0: i32, a1: f32, a2: i64); }
extern "C" { pub fn f_022836_naenvironment_spikeresonanceobjects_float_rage_vector3_const_ref(ph: *mut Phantom, a0: f32, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_022843_audfrontendaudioentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022844_audfrontendaudioentity_startloadingtune(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022845_audfrontendaudioentity_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022847_audfrontendaudioentity_updatemusic(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022848_audfrontendaudioentity_startspecialabilitysounds(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022859_audfrontendaudioentity_startspecialability_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022860_audfrontendaudioentity_stopspecialability(ph: *mut Phantom); }
extern "C" { pub fn f_022869_rage_collisionmaterialsettings_getfieldptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022870_rage_climbingaudiosettings_getfieldptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022871_rage_clothaudiosettings_getfieldptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022872_rage_radiogenre_tostring_rage_radiogenre(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_022873_rage_ambientradioleakage_tostring_rage_ambientradioleakage(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_022874_rage_slowmotype_tostring_rage_slowmotype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_022875_rage_trackcats_tostring_rage_trackcats(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_022876_rage_ggameobjectsisoftype_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_022878_audglassaudioentity_dtoraudglassaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022883_audbreakableglassevent_updatebrokenglassgroundtest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022884_audglassaudioentity_addbrokenglassarea_short_rage_vec3v_ref_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_022887_audcachedbrokenglasstestresults_dtoraudcachedbrokenglasstestresu(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022889_audglassaudioentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022892_audglassaudioentity_clearall_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022894_audglassaudioentity_getglassiness_rage_vec3v_const_ref_bool_floa(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> f32; }
extern "C" { pub fn f_022895_audgunfightconductorfakescenes_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022896_audgunfightconductor_reset(ph: *mut Phantom); }
extern "C" { pub fn f_022897_audgunfightconductor_computegunfighttargetsdebug(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022899_audgunfightconductor_emphasizemediumintensity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022900_audgunfightconductorfakescenes_handleplayerexitcover(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022901_audgunfightconductor_processsuperconductororder_conductors_condu(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022904_audgunfightconductorfakescenes_preupdateservice_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022905_audgunfightconductorfakescenes_playobjectsfakedimpacts_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022908_audgunfightconductorfakescenes_fakerunningaway(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022909_audgunfightconductorfakescenes_triggeropenspacescene(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022910_audgunfightconductorfakescenes_playvehiclesfakedimpacts(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022911_audgunfightconductorfakescenes_updatefakedvehiclevfx_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022912_audgunfightconductorfakescenes_getclosestvalidobject(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_022913_audgunfightconductorfakescenes_querysoundnamefromobjectandfield(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_022915_audgunfightconductorfakescenes_fakevehiclesmashlights_ehierarchy(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022916_audgunfightconductorfakescenes_fakevehicletyreburst_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022917_audgunfightconductorfakescenes_fakevehiclesmashwindow_ehierarchy(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022919_naenvironment_computeworldsectorindex_rage_vector3_const_ref_con(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_022922_audhashactiontable_dtoraudhashactiontable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022923_audhashactiontable_addactionhandler_unsigned_int_void_ptr_unsign(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_022924_audhashactiontable_invokeactionhandler_unsigned_int_void_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_022925_audheliaudioentity_dtoraudheliaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022926_audheliaudioentity_stophelienginesounds(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022927_naenvironment_freeshockwave_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_022931_audheliaudioentity_getheliaudiosettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022935_audheliaudioentity_setuphelivolumecones(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022942_audheliaudioentity_starthelienginesounds_audhelivalues_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_022943_audheliaudioentity_updatehelienginesoundstartingstopping_audheli(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022944_audheliaudioentity_updatehelienginesoundson_audhelivalues_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022945_audheliaudioentity_updatehelienginesounds_audhelivalues_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022946_naocclusionmanager_getexteriorocclusionfor3dposition_rage_vector(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_022947_rage_audsound_setrequestedlpfcutoff_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_022962_naocclusionmanager_getocclusionzoneindices_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_022976_audambientaudioentity_dtoraudambientaudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022977_audambientzone_calculatepositioningline(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022978_audambientaudioentity_calculateanchorpointsizecompensation_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_022979_audambientaudioentity_calculateanchorpointrotationcompensation_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_022980_audambientzone_getactivationzoneaabb_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022981_audambientzone_debugdraw_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022982_audambientzone_debugdrawplayingsounds_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022983_audambientzone_debugdrawrules_float_ref_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_022984_audambientaudioentity_ispointoverwater_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022985_cpathfind_findnodepointersafe_rage_cnodeaddress(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022986_audambientaudioentity_matchname_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_022987_audambientzone_drawanchorpoint_rage_vec3v_const_ref_rage_vec3v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_022988_audambientzone_computerandompositionforrule_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_022989_audambientzone_ispointoverwater_rage_vec3v_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_022990_audambientzone_isconditionvalid_rage_ambientrule_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_022992_audambientzone_dtoraudambientzone(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022994_audambientaudioentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022995_audambientaudioentity_checkandaddzones(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022996_audambientaudioentity_destroyallzonesandshorelines(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022997_audambientaudioentity_loadshorelines_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022998_audambientaudioentity_destroyunreferenceddlczones_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022999_audambientzone_stopallsounds_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023000_audambientzone_init_rage_ambientzone_ptr_unsigned_int_bool_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: f32); }
extern "C" { pub fn f_023001_audambientzone_calculatedynamicbankusage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023002_audambientzone_setactive_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023003_audambientzone_initbounds(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023004_audambientzone_getpositioningzoneaabb_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023005_rage_atarray_audplayingrulestate_0_unsigned_short_resizegrow_uns(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023006_audambientaudioentity_handleraveshorelinecreatednotification_rag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023007_audambientaudioentity_handleraveshorelineeditednotification(ph: *mut Phantom); }
extern "C" { pub fn f_023008_audambientaudioentity_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023009_auddirectionalambiencemanager_registerparentzone_audambientzone(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_023010_auddirectionalambiencemanager_auddirectionalambiencemanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023011_auddirectionalambiencemanager_init_rage_directionalambience_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023012_auddirectionalambiencemanager_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023016_audambientaudioentity_updatealarms_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023017_audambientzone_updatedynamicwaveslots_unsigned_short_bool_rage_v(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_023018_audambientzone_update_unsigned_int_rage_vec3v_const_ref_bool_flo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: f32, a5: i32); }
extern "C" { pub fn f_023019_audshorelineriver_stopsounds_unsigned_char(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_023022_audambientaudioentity_stopexteriorwallasounds(ph: *mut Phantom); }
extern "C" { pub fn f_023025_audambientaudioentity_stopinteriorwallasounds(ph: *mut Phantom); }
extern "C" { pub fn f_023026_audambientaudioentity_stopalarm_rage_alarmsettings_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_023027_audambientaudioentity_startalarm_rage_alarmsettings_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_023028_audambientaudioentity_stopallalarms_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023029_audambientaudioentity_updatereplayalarmstates(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023030_audambientzone_op_audambientzone_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023031_audambientaudioentity_calculatevalidambientradiostations(ph: *mut Phantom); }
extern "C" { pub fn f_023032_audshorelineocean_stopsounds(ph: *mut Phantom); }
extern "C" { pub fn f_023033_audambientaudioentity_forceoceanshorelineupdatefromnearestroad(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023034_audshorelineocean_calculatepoints_rage_vector3_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_023035_audshoreline_calculatecentrepoint_rage_vector3_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_023038_bool_creplaymgrinternal_recordfx_cpacketsetunderwaterstreamvaria(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023041_naenvironment_findinteriorsettingsforroom_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_023043_rage_audsound_setrequestedenvironmentalloudnessfloat_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023044_audambientaudioentity_updateambientzonereplaystates(ph: *mut Phantom); }
extern "C" { pub fn f_023045_naocclusionmanager_getblockedfactorfordirection_audambiencedirec(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_023046_audambientaudioentity_triggerstreamedambientoneshot_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_023048_audambientaudioentity_checkandaddpedwallavoice_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_023049_naenvironmentgroup_setsourceenvironmentupdates_unsigned_int_unsi(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023051_audambientaudioentity_preparealarm_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023054_audambientaudioentity_clearzonenonpersistentstatus_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_023056_audambientaudioentity_createzone_bool_char_const_ptr_rage_vector(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_023057_audambientaudioentity_serialisevector_char_ptr_char_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_023058_audambientaudioentity_serialisetag_char_ptr_char_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_023059_audambientaudioentity_updateravezone_audambientzone_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023060_audambientaudioentity_serialiseuint_char_ptr_char_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_023061_audambientaudioentity_moveruletocurrentcoords_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_023062_audambientaudioentity_createruleatcurrentcoords_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_023063_audshoreline_rotateactivationbox_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_023097_naocclusionmanager_getexteriorocclusionfordirection_audambienced(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_023101_audshoreline_createshorelineenvironmentgroup_rage_vector3_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023106_audshoreline_serialiseactivationbox_char_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023126_audshorelineocean_updateoceansound_rage_audmetadataref_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_023135_audshorelinepool_dtoraudshorelinepool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023153_audshorelineriver_linkedtoclosestriver_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_023158_audshorelineriver_getsoundsrefs_rage_rivertype_rage_audmetadatar(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_023159_audshorelineriver_updateriversound_rage_audmetadataref_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_023170_audwarpmanager_havevehiclemodsloaded(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023173_audcachedlosmultipleresults_dtoraudcachedlosmultipleresults(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023174_naenvironment_purgeinteriors(ph: *mut Phantom); }
extern "C" { pub fn f_023176_naenvironment_processcachedworldsectorops(ph: *mut Phantom); }
extern "C" { pub fn f_023177_debugdrawsector_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_023178_naenvironment_checkforwatersector(ph: *mut Phantom); }
extern "C" { pub fn f_023179_naenvironment_isplayerinvehicle_float_ptr_bool_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023181_naenvironment_playbgtreeswind_unsigned_char_unsigned_char_unsign(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: f32, a6: i32, a7: i64, a8: i32); }
extern "C" { pub fn f_023182_naenvironment_getroomintlocfromportalintloc_rage_fwinteriorlocat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023183_naenvironment_getbestechoforsourceposition_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_023184_naenvironment_calculateechodelayandattenuation_unsigned_int_rage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_023185_naenvironment_getechosuitablilityforsourceposition_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023186_naenvironment_getreverbfromsemetric_float_float_float_ptr_float(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_023189_naenvironment_getaveragedbuildingheightforindex_unsigned_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_023190_naenvironment_getaveragedbuildingheightfromxyoffset_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_023191_naenvironment_getaveragedbuildingcountforindex_unsigned_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_023192_naenvironment_getaveragedbuildingcountfromxyoffset_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_023193_naenvironment_getaveragedtreecountforindex_unsigned_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_023194_naenvironment_getaveragedtreecountfromxyoffset_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_023195_naenvironment_getaveragedwateramountforindex_unsigned_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_023196_naenvironment_getaveragedwateramountfromxyoffset_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_023197_naenvironment_getaveragedhighwayamountforindex_unsigned_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_023198_naenvironment_getaveragedhighwayamountfromxyoffset_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_023199_naenvironment_getspeakerreverbwet_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> f32; }
extern "C" { pub fn f_023200_naenvironment_createraveinteriorroomobject_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023201_naenvironment_createraveobjectsforcurrentinterior(ph: *mut Phantom); }
extern "C" { pub fn f_023209_naenvironmentgroup_drawdebug(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023211_naocclusionmanager_getboundingboxocclusionfactorforposition_rage(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_023212_naocclusionmanager_computeocclusionpathmetrics_naenvironmentgrou(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_023213_naocclusionmanager_getocclusionfactorforradialandelevation_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> f32; }
extern "C" { pub fn f_023214_naocclusionmanager_generateocclusionpathkey_cinteriorinst_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_023215_naocclusionpathnode_buildocclusiondatatree_naocclusionpathnodeda(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023216_naocclusionpathnode_computepathsegmentends_naocclusionpathnodeda(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023217_naocclusionpathnode_computepathsegmentstarts_naocclusionpathnode(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023218_naocclusionpathnode_computeocclusionfrompathtree_naocclusionpath(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_023219_naenvironmentgroup_getinteriorinst_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023220_naenvironmentgroup_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023223_naenvironmentgroupmanager_updateenvironmentgroup_rage_audenviron(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_023224_rage_atarray_naocclusionpathnodedata_0_unsigned_short_destruct_n(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023227_namicrophones_setupmicrophones_rage_microphonetype_rage_micropho(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_023228_namicrophones_setupfirstlistener_rage_microphonetype_rage_microp(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32) -> i32; }
extern "C" { pub fn f_023229_namicrophones_setupsecondarylistener_rage_microphonetype_rage_mi(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32) -> i32; }
extern "C" { pub fn f_023230_namicrophones_computeequivalentrolloff_unsigned_int_float(ph: *mut Phantom, a0: i32, a1: f32) -> f32; }
extern "C" { pub fn f_023231_namicrophones_isvisiblebysniper_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023232_namicrophones_isfirstperson(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023233_namicrophones_isvehbonnetmic(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023234_namicrophones_iscinematicmicactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023237_naocclusioninteriorinfo_dtornaocclusioninteriorinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023238_naocclusioninteriorinfo_initfrommetadata_naocclusioninteriormeta(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_023239_rage_atarray_naocclusionpathnodechild_0_unsigned_short_grow_unsi(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023240_naocclusionportalentity_allocate(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_023242_rage_atarray_rage_atptr_naocclusionportalinfo_0_unsigned_short_g(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023243_naocclusioninteriorinfo_getallrequiredportalinfosforpathtree_nao(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023244_naocclusioninteriorinfo_getallrequiredkeysforpathtree_naocclusio(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023245_naocclusionmanager_getpathnodefromkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_023246_naocclusioninteriorinfo_getpathdepthforpathtree_naocclusionpathn(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_023247_naocclusionmanager_getkeyforpathnode_naocclusionpathnode_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023248_naocclusionportalinfo_addentity_centity_const_ptr_audocclusionpo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_023249_naocclusionportalinfo_getmatchingportallinkedentities_cinteriori(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023257_naocclusionportalinfometadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_023262_naocclusionpathnodemetadata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_023267_naocclusioninteriormetadata_dtornaocclusioninteriormetadata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023269_naocclusionmanager_unloadocclusiondataforinterior_cinteriorproxy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023270_naocclusioninteriorinfo_ptr_ref_rage_atmap_cinteriorproxy_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023271_naocclusionmanager_debugaudioimpact_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_023272_naocclusionmanager_updateexteriorocclusionwithinterior_cinterior(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023273_naocclusionmanager_calculateexteriorblockageforportal_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32) -> f32; }
extern "C" { pub fn f_023274_naocclusionmanager_getportaldestinationinfo_cinteriorinst_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_023275_naocclusionmanager_settobuildocclusionpaths_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_023276_naocclusionmanager_writeocclusionbuildprogress(ph: *mut Phantom); }
extern "C" { pub fn f_023281_naocclusionmanager_computemaxdoorocclusion_centity_const_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_023282_naocclusionmanager_verifyshouldbuildpathsforinterior_cinteriorpr(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_023283_naocclusionmanager_incrementswapset(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023284_naocclusionmanager_updateactiveiplswapinteriorlist(ph: *mut Phantom); }
extern "C" { pub fn f_023285_naocclusionmanager_teleportpedtocurrentinterior_cinteriorproxy_c(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023286_naocclusionmanager_loadinteriorsforpathbuild_naocclusionbuildste(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_023287_naocclusionmanager_createinteriorinfoentries(ph: *mut Phantom); }
extern "C" { pub fn f_023288_naocclusionmanager_gotonextinteriorproxy(ph: *mut Phantom); }
extern "C" { pub fn f_023289_naocclusionmanager_loadmetadatacontainersfromfile(ph: *mut Phantom); }
extern "C" { pub fn f_023290_naocclusionmanager_buildinterinteriorpaths_cinteriorproxy_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023291_naocclusionmanager_isanyswapinteriorloaded_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023292_naocclusionmanager_writeocclusiondataforinteriortofile_cinterior(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023293_naocclusionmanager_writeinteriortobuiltpathsfile_cinteriorproxy(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023294_naocclusionmanager_loadinteriorandlinkedinteriors_naocclusionbui(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: f32) -> i32; }
extern "C" { pub fn f_023295_naocclusioninteriormetadata_ref_rage_atmap_cinteriorproxy_ptr_na(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023296_naocclusionmanager_buildocclusionpathtreeforcombination_cinterio(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_023297_naocclusionmanager_printmemoryinfo(ph: *mut Phantom); }
extern "C" { pub fn f_023298_naocclusionpathnode_findandcreatechildnodes_cinteriorinst_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_023299_naocclusionpathnode_findalllinkedinteriors_cinteriorinst_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_023300_naocclusionpathnode_getportalintersectionbetweenpositions_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_023301_naocclusionpathnode_drawportalpathline_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023302_naocclusionpathnodedata_op_naocclusionpathnodedata_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023307_naocclusionportalinfo_dtornaocclusionportalinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023310_rage_atmapentry_unsigned_int_rage_atarray_naocclusionperportalse(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023363_rage_atrangearray_audgunfightconductor_combatpeddata_4_dtoratran(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023364_audgunfightconductorfakescenes_dtoraudgunfightconductorfakescene(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023389_bool_creplaymgrinternal_handleentityforfx_cpacketsoundnitrousact(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_023390_bool_creplaymgrinternal_handleentityforfx_cpacketcollisionplaypa(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_023391_ccaminterp_init_replaycontroller_const_ref_cpacketcameraupdate_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023392_dumpreplaypackets_char_ptr_unsigned_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023393_cpacketinterp_store(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023394_cpacketcameraupdate_extract_replaycontroller_ref_replaycamframe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: f32, a7: i32); }
extern "C" { pub fn f_023395_cpacketpositionandquaternion20_interpolate_cpacketpositionandqua(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_023396_cpacketevent_cpacketevent_ereplaypacketid_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023400_cpacketinterp_setnextoffset_unsigned_short_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_023401_cpacketbase_print_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023402_cpacketbase_printxml_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023403_cpacketbase_traversepacketchain_unsigned_int_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_023404_cpacketevent_store_ereplaypacketid_unsigned_int_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_023405_cpacketevent_printxml_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023406_cpacketthermalvisionvalues_extract_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023407_cpackettimecyclemodifier_extract_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023408_cpacketmapchange_collectactivemapchanges_cpacketmapchange_mapcha(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023409_cpacketmapchange_extract_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023410_cpacketmapchange_attainstate_cpacketmapchange_mapchange_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023411_cpacketdistantcarstate_extract_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023414_cpacketdisabledthisframe_extract_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023422_rage_ropeinstance_getconstraint_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023430_rage_ropeinstance_getattachedto_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023444_cpacketinterior_setup_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023445_cpacketinteriorentityset_extract_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023446_cpacketinteriorentityset2_store_unsigned_int_rage_vec3v_ref_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_023447_cpacketinteriorentityset2_extract_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023450_cpacketcubicpatchwaterframe_renderpatch_cpacketcubicpatchwaterfr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_023451_cpacketcubicpatchwaterframe_renderpatchwithinterpolation_cpacket(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: f32); }
extern "C" { pub fn f_023452_cpacketcubicpatchwaterframe_extractpatcheswithgridshiftandinterp(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_023453_cpacketcubicpatchwaterframe_getgametime_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023454_cpacketcubicpatchwaterframe_getpacketnumber_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023455_cpacketwatersimulate_extract_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023459_replayclothpacketmanager_setcallbackinfoinclothcontrolleretc_cph(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_023460_replayclothpacketmanager_extract_cpacketclothpieces_const_ptr_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_023461_replayclothpacketmanager_connecttorealclothinstances_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023462_replayclothpacketmanager_resetclothverticesifpausedandwhatnot_cr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_023463_replayclothpacketmanager_processbigdeltas_cphysical_ptr_rage_clo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_023468_creplayoverlay_displayblockstatus_cbufferinfo_const_ref(ph: *mut Phantom); }
extern "C" { pub fn f_023469_creplayoverlay_displayblockdetails_creplayinterfacemanager_const(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_023472_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_creplay(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_023473_creplayoverlay_generatepacketnamelist_creplayinterfacemanager_re(ph: *mut Phantom); }
extern "C" { pub fn f_023474_bool_creplaymgrinternal_handleentityforfx_cpacketcreatebreakable(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_023475_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_023476_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_023477_cpacketsetunderwaterstreamvariable_cpacketsetunderwaterstreamvar(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i64; }
extern "C" { pub fn f_023479_rage_audsound_setrequestedposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023481_audvehiclecollisioncontext_dtoraudvehiclecollisioncontext(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023482_ccutsceneaudiopacket_ccutsceneaudiopacket_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_023483_ccutscenestopaudiopacket_ccutscenestopaudiopacket_bool_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_023484_csynchsceneaudiopacket_csynchsceneaudiopacket_unsigned_int_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64) -> i64; }
extern "C" { pub fn f_023485_csynchscenestopaudiopacket_csynchscenestopaudiopacket_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_023486_cpacketdynamicmixerscene_cpacketdynamicmixerscene_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023489_cpacketexplosion_cpacketexplosion_eexplosiontag_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_023490_cpacketfootstepsound_cpacketfootstepsound_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_023491_cpacketpedclothsound_cpacketpedclothsound_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_023492_cpacketpedstandingmaterial_cpacketpedstandingmaterial_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_023495_void_creplaymgrinternal_recordpersistantfx_cpacketsoundstop_repl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023501_replaysound_cinitparams_populate_rage_audsoundinitparams_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_023502_replaysound_extractsoundinitparams_rage_audsoundinitparams_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_023503_cpacketsoundcreatemiscwithvars_init_unsigned_int_unsigned_int_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_023526_cpacketsndloadscriptwavebank_cpacketsndloadscriptwavebank_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_023527_cpacketscriptsetvaronsound_cpacketscriptsetvaronsound_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_023530_cpacketspeech_cpacketspeech_auddeferredsaystruct_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_023532_cpacketsetplayerpainroot_cpacketsetplayerpainroot_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023533_cpacketspeechstop_cpacketspeechstop_short(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_023534_cpacketscriptedspeech_cpacketscriptedspeech_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i32) -> i64; }
extern "C" { pub fn f_023535_cpacketscriptedspeechupdate_cpacketscriptedspeechupdate_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i32, a14: i32, a15: i32) -> i64; }
extern "C" { pub fn f_023536_cpacketanimalvocalization_cpacketanimalvocalization_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_023539_audweaponfireevent_dtoraudweaponfireevent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023544_cpacketweaponspinup_cpacketweaponspinup_bool_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023546_cpacketthermalscopeaudio_cpacketthermalscopeaudio_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_023547_cpacketsndloadweapondata_cpacketsndloadweapondata_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023552_dumpreplaypackets_char_ptr_unsigned_int_39675(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023563_quantizeu8_float_float_float_int(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> i32; }
extern "C" { pub fn f_023564_dequantizeu8_unsigned_char_float_float_int(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_023565_quantizequaternions3_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023566_dequantizequaternions3_rage_quaternion_ref_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023567_quantizequaternions3_20_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023568_dequantizequaternions3_20_rage_quaternion_ref_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023572_cpacketpositionandquaternion_loadmatrix_rage_matrix34_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023605_cpacketfragdata_extract_centity_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023606_cpacketfragdata_printxml_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023607_cpacketfragdata_nodamagebits_store_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023608_cpacketfragdata_nodamagebits_extract_centity_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023609_cpacketfragbonedata_storebone_cpacketpositionandquaternion_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_023610_cpacketfragbonedata_storehighresbone_cpacketpositionandquaternio(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_023611_cpacketfragbonedata_extract_centity_ptr_cpacketfragbonedata_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_023612_cpacketpositionandquaternion_loadmatrix_rage_matrix34_ref_cpacke(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_023613_cpacketpositionandquaternion20_loadmatrix_rage_matrix34_ref_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023614_cpacketpositionandquaternion20_loadmatrix_rage_matrix34_ref_cpac(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_023615_cpacketobjectcreate_getparentid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023616_cpacketobjectcreate_printxml_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023617_cpacketobjectupdate_store_ereplaypacketid_unsigned_int_cobject_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_023618_cpacketobjectupdate_storeextensions_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023619_cpacketobjectupdate_updateboundingboxforluxorplane_cobject_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023620_cpacketobjectupdate_extract_cinterpeventinfo_cpacketobjectupdate(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023621_cpacketobjectupdate_printxml_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023622_cbasicentitypacketdata_printxml_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023623_cpacketobjectdelete_store_cobject_const_ptr_cpacketobjectcreate(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023624_cpacketanimatedobjbonedata_extract_cobject_ptr_cpacketanimatedob(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64); }
extern "C" { pub fn f_023625_cobjinterp_init_replaycontroller_const_ref_cpacketobjectupdate_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023626_cobjinterp_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023627_cobjinterp_getprevfragbonepacket(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023628_cobjinterp_getprevanimobjbonepacket(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023634_cpacketsetbuildingtint_extract_ceventinfo_void_ref_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023635_cpacketpickupcreate_getparentid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023636_cpacketpickupcreate_getweaponhash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023637_cpacketpickupupdate_extract_cinterpeventinfo_cpacketpickupupdate(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023638_cpacketpickupdelete_store_cpickup_const_ptr_cpacketpickupcreate(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023639_cpickupinterp_init_replaycontroller_const_ref_cpacketpickupupdat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023640_cpickupinterp_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023641_cpacketbrieflifetimedestroyedmapobjects_extract_creplayinterface(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_023664_creplayinterfacegame_dtorcreplayinterfacegame(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023686_fidevicereplay_refreshsizesandgettotalsize(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023694_replayfilemanager_openfile_replayfileinfo_ref_filestoreoperation(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_023695_replayfilemanager_readheader_void_ptr_replayheader_ref_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_023696_replayfilemanager_getcrc_void_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023697_fidevicereplay_loadheader_replayfileinfo_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023698_replayfilemanager_updateheadercache_char_const_ptr_replayheader(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023701_rage_atarray_cfiledata_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023702_replayfilemanager_getclipdirectory_char_ref_256_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023703_fidevicereplay_enumerate_replayfileinfo_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023704_replayfileinfo_replayfileinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023706_replayfileinfo_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023707_replayfilemanager_setsaved_replayfileinfo_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023708_rage_atarray_clipuid_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023709_fidevicereplay_deletefile_replayfileinfo_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_023710_fidevicereplay_removecachedproject_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023711_rage_atarray_clipuid_0_unsigned_short_op_rage_atarray_clipuid_0(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023713_fidevicereplay_markfile_char_const_ptr_emarkeduptype(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_023716_replayfilemanager_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_023717_replayfilemanager_stopfilethread(ph: *mut Phantom); }
extern "C" { pub fn f_023718_replayfilemanager_startfilethread(ph: *mut Phantom); }
extern "C" { pub fn f_023719_replayfilemanager_getavailablediskspace(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_023725_replayfilemanager_getclipdirectoryforuser_unsigned_long_char_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023727_replayfilemanager_startsavefile_char_const_ptr_void_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023728_replayfilemanager_isperformingfileop(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023730_replayfilemanager_startdeleteclip_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023731_replayfilemanager_checkdeleteclip_bool_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023735_replayfilemanager_getclipsequencehash_unsigned_long_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_023736_replayfilemanager_startenumerateclipfiles_rage_atarray_cfiledata(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_023737_replayfilemanager_checkenumerateclipfiles_bool_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023738_replayfilemanager_issaving_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023739_replayfilemanager_isloading_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023741_replayfilemanager_dumpreplayfiles_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_023745_creplayinterface_cambasecamera_dtorcreplayinterface(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023756_creplayinterface_cambasecamera_unlinkcreatepacket_creplayid_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_023757_creplayinterface_cambasecamera_getcreatepacket_creplayid_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023758_creplayinterface_cambasecamera_linkcreatepacket_cpacketcameradum(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023766_creplayinterface_cambasecamera_getentity_centityget_cambasecamer(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023811_printoutreplaydata_char_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023812_cpacketextension_beginwrite_cpacketbase_ptr_unsigned_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_023813_cpacketextension_endwrite_cpacketbase_ptr_unsigned_char_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023814_cpacketextension_getreadptr_cpacketbase_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023815_cpacketextension_validate_cpacketbase_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_023816_cpacketextension_gettotaldatasize_cpacketbase_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_023817_cbasicentitypacketdata_store_cphysical_const_ptr_creplayid_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023818_creplayinteriorproxy_setinterior_rage_fwinteriorlocation(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023819_cbasicentitypacketdata_extract_cphysical_ptr_cbasicentitypacketd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32); }
extern "C" { pub fn f_023820_creplayinteriorproxy_getinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023821_cbasicentitypacketdata_removeandadd_cphysical_ptr_rage_fwinterio(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_023825_creplaymgrinternal_checkallthreadsareclear(ph: *mut Phantom); }
extern "C" { pub fn f_023826_creplayeventmanager_clearallrecordingtrackables(ph: *mut Phantom); }
extern "C" { pub fn f_023827_creplaymgrinternal_freememory(ph: *mut Phantom); }
extern "C" { pub fn f_023834_creplaycontrol_setcancelrecording(ph: *mut Phantom); }
extern "C" { pub fn f_023837_creplayeventmanager_freetrackablesthisframeduringrecording(ph: *mut Phantom); }
extern "C" { pub fn f_023844_creplaymgrinternal_recordframe_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_023845_creplaymgrinternal_updatebanks_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_023846_creplaymgr_replayrecordsound_unsigned_int_rage_audsoundinitparam(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_023847_bool_creplaymgrinternal_recordfx_cpacketsoundcreatemiscwithvars(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023848_bool_creplaymgrinternal_recordfx_cpacketsoundcreatemisc_cpackets(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023849_creplaymgr_replayrecordsound_unsigned_int_unsigned_int_rage_auds(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_023850_creplaymgr_replayrecordsoundpersistant_unsigned_int_rage_audsoun(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_023851_void_creplaymgrinternal_recordpersistantfx_cpacketsoundupdateper(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023852_creplaymgr_replayrecordsoundpersistant_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_023853_creplaymgr_replaypresucksound_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_023854_creplaymgr_replayrecordplaystreamfromentity_centity_ptr_rage_aud(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_023855_creplaymgr_replayrecordplaystreamfromposition_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_023856_creplaymgr_replayrecordplaystreamfrontend_rage_audsound_ptr_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023859_replayattachmentmanager_addattachment_creplayid_const_ref_crepla(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_023860_creplayid_ref_rage_atmap_creplayid_creplayid_rage_atmaphashfn_cr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023863_replaybuffermarkermgr_clipunprocessedmarkerstarttimestocurrenten(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_023864_replaybuffermarkermgr_clipunprocessedmarkerendtimestocurrentendt(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_023866_creplaymgrinternal_unfreezeaudio(ph: *mut Phantom); }
extern "C" { pub fn f_023867_creplaymgrinternal_jumpto_float_unsigned_int(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_023868_creplaymgrinternal_setnextplaybackstate_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_023870_creplaymgr_recordmapobject_cobject_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023871_creplayinterfaceobject_recordmapobject_cobject_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023874_creplayinterface_cvehicle_getcreatepacket_creplayid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023875_cstoredvehiclevariationsreplay_storevariations_cvehicle_const_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023876_creplayinterfaceobject_stoprecordingmapobject_cobject_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023878_creplaymgr_shouldhidemapobject_cobject_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023881_creplaymgr_hash_unsigned_int_const_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023883_creplaymgr_setclothasreplaycontrolled_cphysical_ptr_rage_fragins(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_023887_replayattachmentmanager_pairings_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023888_replayattachmentmanager_pairings_dtorpairings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023890_replaybuffermarkermgr_addmarkerinternal_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_023892_rage_atarray_cblockproxy_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_023894_addmarkertoline_unsigned_int_unsigned_int_float_float_float_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_023895_replaybuffermarkermgr_drawdebug_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_023897_creplaycontrol_setsaverecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023898_creplaycontrol_shouldstoprecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023899_creplaycontrol_isoutofdiskspace_bool_float(ph: *mut Phantom, a0: i32, a1: f32) -> i32; }
extern "C" { pub fn f_023900_creplaycontrol_isclipcachefull_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_023901_creplaymgrinternal_saveallblocks(ph: *mut Phantom); }
extern "C" { pub fn f_023902_creplaymgrinternal_playcallback(ph: *mut Phantom); }
extern "C" { pub fn f_023903_creplaymgrinternal_loadclip_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023904_creplaymgrinternal_setcursorspeed_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_023906_creplaycontrol_isplayeroutofcontrol(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023908_creplaymgrinternal_shoulddisabledcameramovement(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_023910_replaycontroller_replaycontroller_caddressinreplaybuffer_ref_cbu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_023911_cblockinfo_lockforwrite(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023912_cbufferinfo_findavailablenormalblock_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023913_caddressinreplaybuffer_prepareforrecording(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023914_replaycontroller_enableplayback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023915_replaycontroller_disableplayback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023916_replaycontroller_enablemodify(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023917_replaycontroller_disablemodify(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023918_replaycontroller_isnextframe(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023919_replaycontroller_isendofplayback(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023920_replaycontroller_advanceblock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023922_creplaypacketbuffer_creplaypacketbuffer_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_023923_creplayeventmanager_playbackhistory_replaycontroller_ref_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_023924_void_creplayeventmanager_trackrecordingevent_replaytrackedsound(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_023925_void_creplayeventmanager_addexpiryinfo_replaytrackedsound_erepla(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_023926_void_creplayeventmanager_trackrecordingevent_replaytrackedscene(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_023930_creplayeventmanager_shouldproceedwithpacket_cpacketevent_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_023931_creplayeventmanager_dumpmonitoredeventstotty_creplaypacketbuffer(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023932_creplayeventmanager_resetsetupinformation(ph: *mut Phantom); }
extern "C" { pub fn f_023934_centity_ptr_creplayeventmanager_findentity_centity_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_023935_void_creplayinterfacemanager_getentity_centity_centityget_centit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023936_replayentityextension_getextindex_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023946_replaybicycleextension_getispedalling_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023947_replayparachuteextension_getisparachuting_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023952_replayvehicleextension_setvehicleasdamaged_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023953_ireplayinterface_ireplayinterface_unsigned_int_unsigned_int_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_023954_ireplayinterface_dtorireplayinterface(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023956_ireplayinterface_postrecordframe_replaycontroller_ref_unsigned_s(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_023959_creplaymodelmanager_updatepreloadrequests_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023961_ireplayinterface_printoutpacket_replaycontroller_ref_ereplaypack(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_023963_ireplayinterface_adddebugwidgets_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023966_creplayinterfacemanager_scanforanomalies_char_ptr_int_int_ref_co(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_023967_creplayinterfacemanager_ensureisbeingrecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023968_creplayinterfacemanager_ensurewillberecorded_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_023982_creplaymodelmanager_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_023987_bool_creplayinterface_cobject_recordelementinternal_cpacketobjec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_023988_creplayinterface_cobject_getcreatepacket_creplayid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023989_cpacketobjectcreate_ptr_creplayrecorder_recordpacketwithparams_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_023993_creplaymodelmanager_loadmodel_unsigned_int_rage_strlocalindex_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64, a7: i32) -> i32; }
extern "C" { pub fn f_023994_creplaymodelmanager_loadasset_rage_strlocalindex_int_bool_rage_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_023995_creplayinterfaceobject_addreplayentitytoworld_centity_ptr_rage_f(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023996_creplayinterfaceobject_sethiderecordformapobject_unsigned_int_cr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_023997_rage_atarray_creplayinterfaceobject_deferredmapobjecttohide_0_un(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_023998_rage_fwmodelid_fwmodelid_rage_fwmodelid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_023999_creplaymodelmanager_getmodelinfo_rage_fwmodelid_char_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_024000_rage_atarray_creplaymodelmanager_failstrmodelinfo_0_unsigned_sho(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024001_creplayinterfaceobject_adddeferredmapobjecttohide_mapobjectid_cr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_024003_creplayinterface_cobject_createbatchedentities_int_ref_creplayst(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_024005_bool_replaycontroller_checkpacket_cpacketobjectcreate_cpacketobj(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_024006_creplayinterface_cobject_playbacksetup_replaycontroller_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024007_cpacketobjectdelete_const_ptr_replaycontroller_readcurrentpacket(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024008_creplayinterface_cobject_convertmaptypedefhashtoindex_replaycont(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024009_creplayinterface_cobject_unlinkcreatepacket_creplayid_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_024010_creplayinterface_cobject_linkcreatepacket_cpacketobjectcreate_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024012_creplayinterface_cobject_removeelement_cobject_ptr_cpacketobject(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_024013_creplayinterfaceobject_resolvemapidandandunhide_unsigned_int_cre(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_024017_creplayinterface_cobject_ensureentityiscreated_creplayid_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_024018_creplayinterface_cobject_getentity_centityget_cobject_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024019_ireplayinterface_hasentityfailedtocreate_creplayid_const_ref_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_024025_creplayinterface_cobject_playdeletepacket_replaycontroller_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024027_cpacketdestroyedmapobjectsforclipstart_const_ptr_replaycontrolle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024028_cpacketbrieflifetimedestroyedmapobjects_const_ptr_replaycontroll(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024030_creplayinterface_cobject_ondelete_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024033_creplayinterface_cobject_trypreloadpacket_cpacketbase_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_024034_creplaymodelmanager_preloadrequest_unsigned_int_rage_strlocalind(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32); }
extern "C" { pub fn f_024035_creplaymodelmanager_preloadrequest_unsigned_int_rage_strlocalind(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_024036_creplaymodelmanager_preloadrequestinternal_creplaymodelmanager_m(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_024054_creplayinterface_cped_ondelete_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024057_creplaymodelmanager_loadmodel_unsigned_int_rage_strlocalindex_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_024059_creplayinterface_cped_removeelement_cped_ptr_cpacketpeddelete_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_024060_creplayinterface_cped_getentity_centityget_cped_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024062_void_replaycontroller_getnextpacket_cpacketpedupdate_const_cpack(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_024063_creplayinterface_cped_ensureentityiscreated_creplayid_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_024064_creplayinterface_cvehicle_getentity_centityget_cvehicle_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024065_cpacketpedupdate_extract_cinterpeventinfo_cpacketpedupdate_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_024067_cpacketpedupdate_extractbonematrices_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024068_cpacketpedupdate_extractbonematrices_cped_ptr_cpacketpedupdate_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_024069_cpacketpedupdate_extractextensions_cped_ptr_cinterpeventinfo_cpa(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024073_creplayinterface_cped_trypreloadpacket_cpacketbase_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_024082_creplayinterface_cpickup_removeelement_cpickup_ptr_cpacketpickup(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_024084_creplayinterface_cpickup_resetallentities(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024086_creplayinterface_cpickup_playbacksetup_replaycontroller_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024087_cpacketpickupdelete_validatepacket_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024088_creplayinterface_cpickup_convertmaptypedefhashtoindex_replaycont(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024089_creplayinterface_cpickup_unlinkcreatepacket_creplayid_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_024090_cpacketpickupcreate_const_ptr_replaycontroller_readcurrentpacket(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024091_creplayinterface_cpickup_linkcreatepacket_cpacketpickupcreate_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024093_creplayinterface_cpickup_ensureentityiscreated_creplayid_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_024094_creplayinterface_cpickup_getentity_centityget_cpickup_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024096_creplayinterface_cpickup_playdeletepacket_replaycontroller_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024098_creplayinterface_cpickup_ondelete_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024151_creplayinterface_cvehicle_clearworldofentities(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024155_creplayinterface_cvehicle_adddebugwidgets(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024162_creplayinterface_cvehicle_playpackets_replaycontroller_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_024163_ctrain_setengine_ctrain_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_024164_ctrain_setlinkedtoforward_ctrain_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_024165_ctrain_setlinkedtobackward_ctrain_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_024166_cpacketvehicledelete_validatepacket_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024167_cpacketvehiclecreate_const_ptr_replaycontroller_readcurrentpacke(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024169_creplayinterface_cvehicle_jumppackets_replaycontroller_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024171_creplayinterface_cvehicle_playbacksetup_replaycontroller_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024172_creplayinterface_cvehicle_unlinkcreatepacket_creplayid_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_024173_creplayinterface_cvehicle_linkcreatepacket_cpacketvehiclecreate(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024176_cpacketvehiclecreate_extractreferencewheelvaluesintovehicleexten(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024178_cpacketwheel_setupextensions_cvehicle_ptr_bool_bool_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_024185_void_replaycontroller_getnextpacket_cpacketvehicleupdate_const_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_024186_cvehinterp_formcvehinterpdata_cvehinterp_cvehinterpdata_ref_cpac(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024187_creplayinterface_cvehicle_ensureentityiscreated_creplayid_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_024189_cpacketvehicleupdate_extractextensions_cvehicle_ptr_cpacketvehic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_024192_cpacketfragbonedatausingskeletonpackets_cpacketquaternionl_extra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_024193_cpacketfragbonedatausingskeletonpackets_cpacketquaternionh_extra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_024194_cskeletonbonedata_cpacketquaternionl_extract_centity_ptr_rage_cr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_024195_cskeletonbonedata_cpacketquaternionl_extract_rage_crskeleton_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64); }
extern "C" { pub fn f_024196_cskeletonbonedata_cpacketquaternionh_extract_centity_ptr_rage_cr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_024197_cskeletonbonedata_cpacketquaternionh_extract_rage_crskeleton_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_024198_updatewindow_cvehicle_ptr_ccardoor_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_024199_rage_fwentity_getlocalmtxnonconst_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_024200_rage_fwentity_setglobalmtx_int_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_024202_creplayinterface_cvehicle_removeelement_cvehicle_ptr_cpacketvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_024206_cbufferinfo_setnumberofblocksallocated_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_024228_creplaymgrinternal_settempbuffer(ph: *mut Phantom); }
extern "C" { pub fn f_024229_creplaymgrinternal_setupreplaybuffer_unsigned_short_unsigned_sho(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_024251_replaylightingmanager_reset(ph: *mut Phantom); }
extern "C" { pub fn f_024252_creplaymgrinternal_validate_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_024253_creplaymgrinternal_requestreplaymusic(ph: *mut Phantom); }
extern "C" { pub fn f_024254_creplayadvancereader_kick_cframeinfo_const_ref_bool_ref_bool_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_024255_creplayadvancereader_handleresults_int_creplaystate_const_ref_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_024256_creplayoverlay_setprecachestring_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_024257_creplaymgrinternal_isusingrecordedcamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024258_replayheader_generatedatestring_rage_atstring_ref_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024259_creplaymgrinternal_onsavecompletefunc_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_024260_creplaythumbnailref_getthumbnail(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024261_creplaymgrinternal_findendofblock_caddressinreplaybuffer(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024264_creplaymgrinternal_playbackthisframeinterpolation_replaycontroll(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_024266_creplaymgrinternal_isawareofworldforrecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024267_creplaymgrinternal_storeentityinformation_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_024271_creplaymgrinternal_onexitclip(ph: *mut Phantom); }
extern "C" { pub fn f_024272_creplaymgrinternal_pauserenderphase(ph: *mut Phantom); }
extern "C" { pub fn f_024273_replayinteriormanager_setinteriordata_rage_atarray_replayinterio(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024274_creplaymgrinternal_getnextframe_cpacketframe_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024275_creplaymgrinternal_movecurrentframe_int_cframeinfo_ref_cframeinf(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_024276_audspeechmanager_reconcilepainbank(ph: *mut Phantom); }
extern "C" { pub fn f_024277_rage_atarray_packetrayfirereplayhandlingdata_0_unsigned_short_gr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024278_creplaymgrinternal_isreplaycursorjumping(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024280_creplaymgrinternal_calculatesizeofblock_rage_atarray_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_024282_replayinteriormanager_consolidateinteriordata_rage_atarray_repla(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_024283_creplaymgrinternal_getprevframe_cpacketframe_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024284_replayinteriormanager_resetentitysetlaststate_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_024285_replayinteriormanager_backupentitysetdata_rage_atarray_rage_atar(ph: *mut Phantom); }
extern "C" { pub fn f_024286_creplaymgrinternal_isplaybackflagset_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_024295_cweapon_cancelcooktimer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024297_rage_atmapentry_int_ptfx_replaystreamingrequest_ref_rage_atmap_i(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024298_creplaymgrinternal_addbuilding_cbuilding_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024299_creplaymgrinternal_gettotalmsinbuffer(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024304_creplaymgrinternal_getbuildinginfo_cphysical_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_024305_creplaymgrinternal_recordmapobject_centity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024310_rage_atarray_sentityset_0_unsigned_short_op_rage_atarray_sentity(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024311_replayinteriormanager_getproxyatcoords_rage_vector3_const_ref_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_024312_replayinteriormanager_findoraddpreloadrequest_replayinteriormana(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_024313_replayinteriormanager_resetroomrequests(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024317_creplaymodelmanager_flushloadrequests(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024327_creplaythumbnailref_ispopulated_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024348_rage_atfixedarray_rage_strrequest_20_dtoratfixedarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024353_replayhidemanager_captureallcutscenehidescb_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_024356_cpackedpedprops_op_cpackedpedprops_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024357_cpacketpedupdate_getbonepositionishorlbitfield_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024358_cpacketpedupdate_formworlspacematricesfrompacketlocalmatricies_u(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_024359_cpacketpedupdate_applystoredscales_cped_ptr_unsigned_short_cpack(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_024360_cpacketpedupdate_buildlocaltransformationsfromobjectmatrices_cpe(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_024361_getnumprops_cpackedpedprops_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024362_packedpropscontainsprop_cpackedpedprops_const_ref_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_024371_cpacketvehicleupdate_store_ereplaypacketid_unsigned_int_cvehicle(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_024372_cpacketvehicleupdate_storeextensions_ereplaypacketid_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024373_cpacketvehicleupdate_extract_cinterpeventinfo_cpacketvehicleupda(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024374_cpacketquadbikeupdate_extract_cinterpeventinfo_cpacketquadbikeup(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024377_cpacketwheel_getextensiondataforrecording_cvehicle_ptr_bool_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_024378_replaywheelvalues_collectfromwheel_cvehicle_ptr_cwheel_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024381_cpacketsubupdate_extract_cinterpeventinfo_cpacketsubupdate_csubm(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_024382_replaywheelvalues_extractinterpolateablevalues_cvehicle_ptr_cwhe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_024386_creplayinterfaceobject_dtorcreplayinterfaceobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024387_creplayinterface_cobject_dtorcreplayinterface(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024441_creplayinterfaceveh_dtorcreplayinterfaceveh(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024442_creplayinterface_cvehicle_dtorcreplayinterface(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024501_creplayinterface_cped_dtorcreplayinterface(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024508_cpacketpeddelete_validatepacket_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_024509_cpacketpedcreate_const_ptr_replaycontroller_readcurrentpacket_cp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024548_creplayinterface_cpickup_dtorcreplayinterface(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024602_rage_atarray_creplayrendertargethelper_rtinfo_0_unsigned_short_d(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_024615_cpacketvehiclecreate_ptr_creplayrecorder_recordpacketwithparams(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_024619_bool_creplaymgrinternal_handleentityforfx_cpacketsoundcreatemisc(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_024621_bool_creplaymgrinternal_handleentityforfx_cpacketsoundupdatepers(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_024622_bool_creplaymgrinternal_handleentityforfx_cpackettrafficlight_cp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_024625_cskeletonbonedata_cpacketquaternionl_shouldpreventscalefrombeing(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_024626_cskeletonbonedata_cpacketquaternionh_store_rage_crskeleton_ptr_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_024636_chistorypacketcontroller_ref_rage_atmap_unsigned_int_chistorypac(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_024637_cped_ptr_creplayeventmanager_findentity_cped_int_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_024638_eshouldextract_creplayeventmanager_getentitiesforpacket_cpacketp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_024657_eshouldextract_creplayeventmanager_getentitiesforpacket_cpacketp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_026138_cpacketpedcreate_ptr_creplayrecorder_recordpacketwithparams_cpac(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_026139_cpacketpickupcreate_ptr_creplayrecorder_recordpacketwithparams_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_026168_caminterface_isrenderingcamera_cambasecamera_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026170_caminterface_isdominantrenderedcameraanyfirstpersoncamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026173_camgameplaydirector_getthirdpersoncamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026176_caminterface_getplayercontrolcamaimframe(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_026179_camcinematicdirector_isrenderingcinematicmountedcamera_bool_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026190_caminterface_fadeout_int_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026194_caminterface_isfadedout(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026195_caminterface_fadein_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026198_caminterface_isfadingin(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026199_caminterface_isfadedin(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026202_caminterface_isspherevisibleingameviewport_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_026203_caminterface_isspherevisibleingameviewport_rage_vec4v_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026205_cqueriableinterface_istaskcurrentlyrunning_int_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_026211_worldprobe_cshapetestfixedresults_unsigned_char_32_cshapetestfix(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026212_caminterface_isrenderedcamerainsidevehicle(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026213_caminterface_computeshouldmakepedheadinvisible_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026219_cdynamicentity_getboneindexfrombonetag_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026222_cambaseobject_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026242_camcinematicdirector_isrenderingcinematicpointofviewcamera_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026259_rage_fwentity_getskeleton_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026261_camcinematicdirector_isrenderinganyinvehiclecinematiccamera_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026286_camgameplaydirector_getthirdpersonaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026329_worldprobe_cshapetestresults_dtorcshapetestresults(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026415_cambaseobject_getnamehash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026579_camanimscenedirector_stopanimatingcamera(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026580_camanimscenedirector_animatecamera_rage_atnonfinalhashstring_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32); }
extern "C" { pub fn f_026636_camcutscenedirector_iscutsceneplaying(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026648_camdebugdirector_getcamframe_cambasecamera_ptr_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026660_camfreecamera_setallcontrolsenabled_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026709_camgameplaydirector_setrelativecameraheading_float_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_026710_camgameplaydirector_setrelativecamerapitch_float_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026714_camgameplaydirector_getfirstpersonpedaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026722_camgameplaydirector_catchupfromframe_camframe_const_ref_float_in(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_027008_ctrain_getlinkedtobackward_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027009_ctrain_getlinkedtoforward_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027037_cdebugclipselector_cdebugclipselector_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_027189_camreplaydirector_getmaxdistanceallowedfromplayer_bool_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_028262_cviewportmanager_pushviewport_cviewport_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_028263_cviewportmanager_popviewport(ph: *mut Phantom); }
extern "C" { pub fn f_029311_cutscenemanager_shutdowncutscene(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029331_cdebugscene_drawentityboundingbox_centity_ptr_rage_color32_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_029335_cgamesessionstatemachine_reinit_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_029339_cgamesessionstatemachine_setstatetoidle(ph: *mut Phantom); }
extern "C" { pub fn f_029377_ctask_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029433_getorcreategroup_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029452_rage_fwassetstore_rage_pgdictionary_rage_grctexture_rage_fwtxdde(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_029455_cdebugscene_tunewidgetdata_tunewidgetdata_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_029456_cdebugscene_queuetunewidget_cdebugscene_tunewidgetdata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029469_cdebugscene_draw3dboundingbox_rage_vector3_const_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_029512_cdebugscene_getworldpositionundermouse_rage_vector3_ref_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_029609_cdebugscene_getmouseleftpressed(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_029612_worldprobe_cshapetestdesc_setresultsstructure_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029613_worldprobe_cshapetestresults_freeintersections(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029726_camframe_setworldmatrix_rage_matrix34_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029866_cdebugscene_focusentities_get_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029964_rage_fwtransform_geta_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029965_rage_fwtransform_getb_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029967_cphysical_gethealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029973_cdebugscene_getmouseleftreleased(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_029974_cdebugscene_getmouserightpressed(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_029975_cdebugscene_getmouserightreleased(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_029976_cdebugscene_getmousemiddlepressed(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_029977_cdebugscene_getmousemiddlereleased(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_029979_cauthoringhelper_cauthoringhelper_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_030047_entitydetails_getowner_centity_ptr_rage_atstring_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030720_rage_atpagedpooliterator_clightentity_rage_fwdefaultrscmempagedp(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030721_rage_atpagedpoolsubtypeiterator_clightentity_rage_fwdefaultrscme(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030736_ceventgroupglobal_addorcombine_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030741_geteventglobalgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030801_cevent_cevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031158_ceventfirenearby_dtorceventfirenearby(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_034931_clandingpagearbiter_shouldshowlandingpageonboot(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034946_cloadingscreens_areactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_034956_chudtools_getminsafezone_float_ref_float_ref_float_ref_float_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32); }
extern "C" { pub fn f_035061_cnewhud_issnipersightactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035081_cnewhud_shouldwheelblockcamera(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035082_cnewhud_isweaponwheelactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035276_cminimap_createsonarblip_rage_vector3_const_ref_float_ehud_colou(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_035277_cminimap_createsonarblipandreportstealthnoise_cped_ptr_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32) -> f32; }
extern "C" { pub fn f_036170_cpausemenu_setmenupreference_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_036175_cprofilesettings_set_cprofilesettings_profilesettingid_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_036184_cpausemenu_togglepauserenderphases_bool_enum_pause_menu_pausedrp(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_036254_cpausemenu_isactive_pm_activeflags(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036364_cprofilesettings_getint_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036379_cpausemenu_ismp(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036382_cprofilesettings_exists_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036386_cprofilesettings_write_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036442_cprofilesettings_set_cprofilesettings_profilesettingid_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_036443_cprofilesettings_getfloat_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_036799_cvideoprojectplaybackcontroller_getplaybackendtimems_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_036800_cvideoprojectplaybackcontroller_getplaybackcurrenttimems_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_037502_cgamestream_removeitem_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_037511_cgamestream_postticker_char_const_ptr_bool_bool_bool_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_037517_cgamestream_postreplay_cgamestream_efeedreplaystate_char_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: f32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_037525_cgamestream_setimportantparamsrgba_int_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_037526_cgamestream_resetimportantparams(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037527_cgamestream_freezenextpost(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037661_cmodelinfo_requestassets_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037662_cmodelinfo_haveassetsloaded_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037676_cclock_settime_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_037888_cseatmanager_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_037901_cwanted_copsaresearching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037946_cweather_getprevtype(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_037949_cweather_gettemperature_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_038107_void_rage_atplaceptrcreator_void_ptr_cdispatchhelpervolumes_tuna(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038467_cbaseikmanager_getsolver_ikmanagersolvertypes_flags_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038492_crootslopefixupiksolver_isactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038629_cbasemodelinfo_getaudiocollisionsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038648_cmlomodelinfo_getaudiosettings_unsigned_int_rage_interiorsetting(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_038649_cmlomodelinfo_setaudiosettings_unsigned_int_rage_interiorsetting(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_038651_cmlomodelinfo_getportalflags_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038652_cmlomodelinfo_getportaldata_unsigned_int_unsigned_int_ref_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_038751_cseatmanager_getpedsseatindex_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_039027_cvehiclemodelinfo_releasedials(ph: *mut Phantom); }
extern "C" { pub fn f_039206_cvehiclevariationinstance_getbone_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039207_cvehiclevariationinstance_getboneforlinkedmod_unsigned_char_cons(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039211_cvehiclevariationinstance_getkit_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039212_cvehiclevariationinstance_getwheeltype_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_039223_cvehiclevariationinstance_getnumbonestoturnoffforslot_unsigned_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039231_cvehiclevariationinstance_havemodsstreamedin_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039232_cvehiclevariationinstance_getmodifier_evehiclemodtype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_039234_cvehiclevariationinstance_getaudioapply_evehiclemodtype_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_039235_cvehiclevariationinstance_setxenonlightcolor_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039247_cvehiclestreamrendergfx_storemodmatrices_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039569_cdoor_getdooraudioentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039576_cdoor_getdooropenratio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_039587_cdoorsystem_generatehash32_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039644_cobject_getpointertoclosestmapobjectforscript_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_039688_cobjectpopulation_destroyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039715_cobject_setweapon_cweapon_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039717_cobject_setfragparent_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039719_cobject_settask_rage_aitask_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_039720_cobject_addtoorremovefromprocesspostcameraarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039761_cobjectpopulation_createobject_rage_fwmodelid_eentityownedby_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_039762_cobjectpopulation_createobject_cobjectpopulation_createobjectinp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039808_cprocobjectman_update(ph: *mut Phantom); }
extern "C" { pub fn f_039877_cpathserver_isrequestresultready_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_039883_rage_cnavmeshtrackedobject_isuptodate_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_039896_cpathserver_requestaudioproperties_rage_vector3_const_ref_rage_t(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_039905_cpathserver_getaudiopropertiesresult_unsigned_int_unsigned_int_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_039913_cpathserver_cancelrequest_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_040318_cstrikeboneset_getstrikebonetag_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041056_findplayerped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041060_cpeddebugvisualisermenu_getfocusped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041071_cpedintelligence_getclosestvehicleinrange_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041197_cpedintelligence_setdefaultrelationshipgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041237_cpedintelligence_gettaskmelee_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041296_cpedfactory_destroyped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041305_cpedfootstephelper_isananimal(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041306_cpedfootstephelper_getfootvelocity_feettags_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041307_cpedfootstephelper_getpositionforpedsound_audfootstepevent_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_041308_cpedfootstephelper_getpositionforpedsound_feettags_rage_vec3v_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_041309_cpedfootstephelper_getfeettagforfootstepevent_audfootstepevent_c(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041311_cpedfootstephelper_resetairflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041313_cpedfootstephelper_setwalkingonpuddle_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041314_cpedfootstephelper_iswalkingonpuddle(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041315_cpedfootstephelper_getslopedirection_rage_vec3v_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041330_cpedintelligence_getnearbyobjects(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041337_cpedhelmetcomponent_ishelmetenabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041442_cpedintelligence_honkedat_cvehicle_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041460_cpedplacement_findzcoorforped_float_rage_vector3_ptr_centity_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_041484_cpedpopulation_flushpedreusepool(ph: *mut Phantom); }
extern "C" { pub fn f_041496_findplayervehicle_cplayerinfo_ptr_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041557_cpedpopulation_getcopsinarea_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_041600_cplayerspecialability_getremainingnormalized_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041696_cplayerinfo_hasplayerlefttheworld_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041700_cplayerinfo_getpos(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041702_findfollowped_cplayerinfo_ptr(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041734_cplayerpedtargeting_gettarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041746_reportplayerresetmismatch_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_041799_cpopulationgroup_getmodelindex_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041809_cpopzones_findsmallestforposition_rage_vector3_const_ptr_ezoneca(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_041828_cpopzone_gettranslatedname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041833_cpopzones_getsearchinfofrompopzone_cpopzone_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041933_cplayerpedtargeting_getonfoothominglockonstate_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_042406_cpedvariationpack_getglobalcompdata_cped_const_ptr_epedvarcomp_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_042409_cpedheadblenddata_cpedheadblenddata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042412_cpedheadblenddata_dtorcpedheadblenddata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042438_cpedvariationpack_getcompvar_cped_const_ptr_epedvarcomp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042489_cpedpropsmgr_getpedpropidx_cped_const_ptr_eanchorpoints(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042602_cpedvariationinfocollection_lookupcompinfoaudioid_int_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_042634_cpedpropsmgr_getpedpropspacked_cdynamicentity_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042636_cpedpropsmgr_setpedpropspacked_cdynamicentity_ptr_cpackedpedprop(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042661_cpackedpedprops_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042663_cpackedpedprops_setpackedpedpropdata_unsigned_int_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_042664_cpackedpedprops_getpackedpedpropdata_unsigned_int_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_042686_peddecorationmanager_addpeddecoration_cped_ptr_rage_atnonfinalha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32); }
extern "C" { pub fn f_042696_peddecorationmanager_addcompressedpeddecoration_cped_ptr_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_042800_cpickup_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042814_worldprobe_debugfindgroundzfor3dcoord_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> f32; }
extern "C" { pub fn f_042817_worldprobe_debugfindgroundzfor3dcoord_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32, a5: i64, a6: i64) -> f32; }
extern "C" { pub fn f_042818_worldprobe_cshapetestmanager_debugsubmittest_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_042821_worldprobe_cshapetestmanager_aborttest_worldprobe_cshapetestresu(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042878_cpickupmanager_removeallpickups_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_042885_cpickupmanager_createpickup_unsigned_int_rage_matrix34_const_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_043077_cbuoyancy_getbuoyancyinfo_cphysical_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_043079_cbuoyancy_resetbuoyancy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043080_cbuoyancy_getwaterlevelonsample_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_043085_worldprobe_cshapetestdesc_setexcludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043215_cphysics_ropesimupdate_float_int(ph: *mut Phantom, a0: f32, a1: i32); }
extern "C" { pub fn f_043333_cwatertesthelper_getcachedriverboundproberesult_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_043334_cwatertesthelper_getwaterlevelincludingrivers_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_043485_water_iscameraunderwater(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_043710_animpostfxmanagerreplayeffectlookup_areallgamecamstacksidle_bool(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_043711_animpostfxmanagerreplayeffectlookup_getfiltername_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_043787_cpostscan_isonalphaoverridelist_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_043789_cpostscan_getalphaoverride_centity_const_ptr_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_043902_crendertargetmgr_getnamedrendertarget_rage_atfinalhashstring_con(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043903_crendertargetmgr_getnamedrendertarget_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_043905_crendertargetmgr_registernamedrendertarget_char_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_043906_crendertargetmgr_releasenamedrendertarget_char_const_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_043908_crendertargetmgr_releasenamedrendertarget_rage_atfinalhashstring(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_043910_crendertargetmgr_getnamedrendertargetid_char_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_043911_crendertargetmgr_linknamedrendertargets_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_043912_crendertargetmgr_isnamedrendertargetlinked_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_043970_river_getriverflowfromposition_rage_vec3v_const_ref_rage_vector2(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_044122_water_getwaterlevel_rage_vector3_const_ref_float_ptr_bool_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_044177_water_getcamerawaterdepth(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_044179_water_setsecondarywatertunings_csecondarywatertune_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_044962_creplayaudiotrackprovider_mergeradiostationlist_ereplaymusictype(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_044970_creplayaudiotrackprovider_getmusictrackdurationms_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_044996_creplaycoordinator_shouldshowloadingscreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045006_creplaycoordinator_isexportingtovideofile(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045021_cmontage_create(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_045023_cmontage_destroy_cmontage_ptr_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045030_cclip_getendnondilatedtimems_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_045032_cmontage_getclip_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045064_cmontage_gettotalcliplengthtimems_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_045150_cmontage_save_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_045152_cmontage_load_char_const_ptr_unsigned_long_bool_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_045245_csavegamequeue_push_csavegamequeuedoperation_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045248_cgenericgamestorage_queueloadmostrecentsave(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_045271_csavingmessage_begindisplaying_csavingmessage_etypeofmessage(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_045351_cphotomanager_isnameofmissioncreatortexture_char_const_ptr_char(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045388_csavegamequeuedoperation_enumeratereplayfiles_init_ereplayenumer(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_045391_csavegamequeuedoperation_replaymultidelete_init_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045395_csavegamequeuedoperation_loadreplayclip_init_char_const_ptr_cbuf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_045397_csavegamequeuedoperation_loadmontage_init_char_const_ptr_cmontag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_045788_cped_getpedhealthinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045789_cped_ismale_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045790_cped_isfiring_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045797_cped_getcurrentmotiontask_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045802_cped_setcharparamsbasedonmanagertype(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045806_cped_clearwetclothing(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045812_cped_removespeakerlistenedto(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045816_cped_removespeakerlistenedtosecondary(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045842_cped_forcemotionstatethisframe_cpedmotionstates_emotionstate_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_045844_cped_setdefaultdecisionmaker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045873_cped_getclimbphysical_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045874_cped_isonground_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045883_cped_setpedoutofvehicle_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_045886_cped_setmyvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045891_cped_applyragdollblockingflags_eragdollblockingflagsbitset_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045892_cped_clearragdollblockingflags_eragdollblockingflagsbitset_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045902_cped_updatespatialarraytypeflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045903_cped_getiscrouching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045913_cped_isfirstpersonshootermodeenabledforplayer_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045914_cped_setspeakerlistenedto_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045915_cped_setspeakerlistenedtosecondary_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045922_cped_getboneposition_rage_vector3_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045926_cped_getisdrivingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045937_cped_hasheadblend_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045940_cped_setvariation_epedvarcomp_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_045945_cped_setheadblenddata_cpedheadblenddata_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045946_cped_setheadoverlay_eheadoverlayslots_unsigned_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_045952_cped_micromorph_emicromorphtype_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_045961_cped_finalizeheadblend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045965_cped_getpedrace_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046008_cped_processbuoyancy_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_046016_cped_getisshelteredfromrain(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046026_cped_getvehiclepedentering_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046067_cped_cleardecorations(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046070_cped_updatefpscamerarelativematrix(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046071_cped_isfirstpersoncameraorfirstpersonsniper_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046095_cped_getpedaudioid_unsigned_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046096_cped_getbonepositioncachedv_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046097_cped_getbonematrixcached_rage_matrix34_ref_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_046102_cped_getisenteringvehicle_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046103_cped_getspecialability_eplayerspecialabilityslot(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046104_cped_isexitingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046107_cped_setdeathstate_edeathstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046138_cped_haveallstreamingreqscompleted(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046141_cped_isbeingjackedfromvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046144_cped_newsay_char_const_ptr_unsigned_int_bool_bool_int_cped_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32) -> i32; }
extern "C" { pub fn f_046145_cped_newsaywithparams_char_const_ptr_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_046147_cped_newsay_unsigned_int_unsigned_int_bool_bool_int_cped_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_046152_cped_setpedinvehicle_cvehicle_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_046153_cped_getpedradiocategory_int_ref_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046156_cped_getispanicking_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046162_cped_setclothforcepin_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046171_cped_getpedvoicegroup_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046172_cped_getwaterlevelonped_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_046176_cped_getcontrolfromplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046279_canimatedbuilding_canimatedbuilding_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046302_cdynamicentity_createskeleton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046303_cdynamicentity_createanimdirector_rage_rmcdrawable_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_046304_canimatedbuilding_updaterateandphase(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046305_cdynamicentity_addtosceneupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046316_cbuilding_cbuilding_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046325_cgameworld_findfollowplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_046327_cplayerswitchinterface_isactive_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_046403_cscriptdownloadabletexturemanager_isnameofatexture_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046408_cinteriorinst_getinteriorforlocation_rage_fwinteriorlocation_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046410_centity_getinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046427_cdynamicentity_removesceneupdateflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046438_cgameworld_add_centity_ptr_rage_fwinteriorlocation_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046442_cinteriorproxy_getfromlocation_rage_fwinteriorlocation_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046443_cinteriorinst_getportalinroom_unsigned_int_unsigned_int_rage_fwp(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_046444_cinteriorinst_getmatchingportalinst_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_046445_cportaltracker_collectstack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046446_cinteriorinst_getdestthruportalinroom_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046447_cinteriorproxy_createlocation_cinteriorproxy_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046449_cportaltracker_movetonewlocation_rage_fwinteriorlocation(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046453_cdynamicentity_deleteanimdirector(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046467_cgameworld_remove_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046470_centity_deleteinst_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046473_centity_getisinexterior_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046474_centity_getshouldusescreendoor_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046478_centity_getplaceabletracker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046484_centity_setisvisibleformodule_eisvisiblemodule_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046490_centity_isvisible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046515_centity_changevisibilitymask_unsigned_short_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046524_centity_getaudioenvironmentgroup_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046526_centity_getaudiointeriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046531_centity_getisvisibleformodule_eisvisiblemodule_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046533_centity_addtomixgroup_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046542_centity_setcreationframeref_frameref_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046543_centity_getcreationframeref_frameref_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_046545_centity_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046679_cmountablecontent_revertcontentchangeset_rage_atnonfinalhashstri(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_046682_cmapfilemgr_cleanup(ph: *mut Phantom); }
extern "C" { pub fn f_046689_cextracontentmanager_triggerloadingscreen_loadingscreencontext(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046690_cmountablecontent_executecontentchangeset_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_046702_cwarpmanager_setwarp_rage_vector3_const_ref_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: f32, a6: i32) -> i32; }
extern "C" { pub fn f_046902_cinteriorproxy_iscontainingimapactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046994_cfileloader_loadline_void_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047005_cfileloader_markarchetypeswhichcannotbedummy(ph: *mut Phantom); }
extern "C" { pub fn f_047020_cgameworld_findlocalplayerwanted(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047024_cloadscene_start_rage_vec3v_const_ref_rage_vec3v_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_047025_cloadscene_stop(ph: *mut Phantom); }
extern "C" { pub fn f_047030_cloadscene_isloaded(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_047032_cinteriorproxy_setrequestedstate_cinteriorproxy_erequest_module(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_047033_cinteriorproxy_findactiveintersecting_rage_fwissphereintersectin(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_047041_cmapchange_execute_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_047042_cmapchange_findentitiesfromworldsearch_rage_atarray_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_047044_cgameworld_forallentitiesintersecting_rage_fwsearchvolume_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_047045_cmapchangemgr_add_unsigned_int_unsigned_int_rage_spdsphere_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_047046_cmapchangemgr_remove_unsigned_int_unsigned_int_rage_spdsphere_co(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_047054_cphysical_setisinwater_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047063_cphysical_disablephysicsiffixed(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047073_cphysical_sethealth_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047080_cphysical_hasbeendamagedbyhash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047168_cinteriorproxy_immediatestateupdate_rage_fwptrlistsinglelink_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047305_cgameworld_getmainplayerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047337_cinteriorinst_getnumportals_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047345_cinteriorinst_createlocation_cinteriorinst_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_047374_cgameworld_changeplayerped_cped_ref_cped_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_047376_ccompentity_ccompentity_eentityownedby(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_047388_ccompentity_settostarting(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047395_ccompentity_settoending(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047398_ccompentity_triggeranim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047680_cplayerswitchinterface_shoulddisableradio_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_047703_cinteriorinst_getnumrooms_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047712_cinteriorinst_calcroomboundbox_unsigned_int_rage_spdaabb_ref_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_047713_cinteriorinst_getnumportalsinroom_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047714_cinteriorinst_getportalidxinroom_unsigned_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_047715_cinteriorinst_getroomhashcode_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047718_cinteriorproxy_movetoretainlist_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047725_cinteriorinst_issubwaymlo_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047728_cinteriorinst_getnumobjinportal_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047730_cinteriorinst_getportalobjinstanceptr_int_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_047731_cinteriorinst_islinkportal_unsigned_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_047733_cinteriorinst_getroomname_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_047734_cinteriorinst_findroombyhashcode_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_047735_cinteriorinst_requestroom_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_047739_cportalinst_getdestinationdata_cinteriorinst_const_ptr_cinterior(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_047758_cinteriorproxy_getinteriorproxyfromcollisiondata_centity_ptr_rag(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047763_cinteriorproxy_setisdisabled_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047764_cinteriorproxy_setiscappedatpartial_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047854_cportaltracker_removefrominterior(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047856_cportaltracker_probeforinterior_rage_vector3_const_ref_cinterior(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_047857_cgameworld_removeandadd_centity_ptr_rage_fwinteriorlocation_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_047880_cgameworld_findlocalplayercoors(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047965_cgameworld_findfollowplayervehicle(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047975_cgameworld_clearcarsfromarea_rage_vector3_const_ref_float_bool_b(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_047977_cgameworld_clearpedsfromarea_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047979_cgameworld_clearobjectsfromarea_rage_vector3_const_ref_float_boo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_047982_cgameworld_findlocalplayervehicle_bool(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047996_cgameworld_findlocalplayertrain(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047998_cgameworld_findlocalplayerspeed(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_048037_cgameworldheightmap_getminheightfromworldheightmap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_048043_cgameworldheightmap_getmaxheightfromworldheightmap_float_float_f(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_048062_cgameworldwaterheight_getwaterheightatpos_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_050160_entity_commands_commandsetentityalphaentity_centity_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_050164_entity_commands_commandresetentityalphaentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_050474_graphics_commands_gettexturefromstreamedtxd_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_050589_graphics_commands_commanddrawrect_float_float_float_float_int_in(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_050603_graphics_commands_drawspritetex_rage_grctexture_ptr_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i64, a13: i64, a14: i32, a15: i32); }
extern "C" { pub fn f_051564_hud_commands_commandsettextrenderid_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_063772_cthescripts_getcurrentgtascripthandler(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063796_cthescripts_unregisterentity_cphysical_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063798_cthescripts_prioritizerequestsfrommissionscript_int_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063799_cthescripts_getcurrentscriptname(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063800_cthescripts_getcurrentscriptnameandprogramcounter(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063811_cthescripts_getpoptypename_epoptype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_064215_cgamescriptid_generatescriptidhash(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064234_cgamescripthandlermgr_isscriptactive_cgamescriptid_const_ref_boo(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064588_ccustomshadereffecttint_selecttintpalette_unsigned_char_rage_fwe(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064589_ccustomshadereffecttint_getmaxtintpalette(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064640_ccustomshadereffectvehicle_setlicenseplatetexindex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064642_ccustomshadereffectvehicle_setlicenseplatetext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064646_ccustomshadereffectvehicle_setcustomprimarycolor_rage_color32(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064647_ccustomshadereffectvehicle_setcustomsecondarycolor_rage_color32(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064652_ccustomshadereffectvehicle_getlicenseplatetext_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064749_cislandhopper_requesttoggleisland_rage_atnonfinalhashstring_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_064760_cislandhopper_getcurrentactiveisland_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064800_cstreaming_hasobjectloaded_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064815_cloadedmodelgroup_pickrandomcarmodel_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_064832_cstreaming_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064913_ckeyboard_getkeyjustdown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_064928_cstreaming_purgerequestlist_unsigned_int(ph: *mut Phantom); }
extern "C" { pub fn f_064929_cstreaming_setobjectisdeletable_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064931_cstreaming_setmissiondoesntrequireobject_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064934_cstreaming_removeobject_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064942_cstreamingcleanup_deletealldrawablereferences(ph: *mut Phantom); }
extern "C" { pub fn f_065273_ckeyboard_getkeydown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_065389_ccontrol_startplayerpadshake_unsigned_int_int_unsigned_int_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_065461_ccontrolmgr_getmainfrontendcontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065615_cfilemgr_openfile_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_065617_cfilemgr_setdir_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065618_cfilemgr_setappdatadir_char_const_ptr_bool(ph: *mut Phantom); }
extern "C" { pub fn f_065620_cfilemgr_setuserdatadir_char_const_ptr_bool(ph: *mut Phantom); }
extern "C" { pub fn f_065621_cfilemgr_openfileforwriting_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065622_cfilemgr_openfileforappending_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065698_csystem_wanttoexit(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065950_ctaskscriptedanimation_ctaskscriptedanimation_rage_atnonfinalhas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32, a6: f32, a7: i32, a8: i64, a9: f32, a10: i32, a11: i32, a12: i64, a13: i32, a14: i32) -> i64; }
extern "C" { pub fn f_065951_ctaskscriptedanimation_ctaskscriptedanimation_rage_atnonfinalhas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32, a6: f32, a7: i32, a8: i64, a9: f32, a10: i64, a11: i64, a12: i32, a13: i32, a14: i64, a15: i32, a16: i32) -> i64; }
extern "C" { pub fn f_072512_cscenariochaininggraph_removechainuserifunused_cscenariopointcha(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_072544_cscenariospawnhistory_update(ph: *mut Phantom); }
extern "C" { pub fn f_072675_cscenariospawnhistory_clearhistory(ph: *mut Phantom); }
extern "C" { pub fn f_074161_ctask_ctask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075349_cmovenetworkhelper_getfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_075350_ccliprequesthelper_ccliprequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075351_ccliprequesthelper_dtorccliprequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075352_ccliprequesthelper_releaseclips(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075353_ccliprequesthelper_requestclipsbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075474_cmovenetworkhelper_setfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_075488_cgenericcliphelper_blendinclipbydictandhash_cdynamicentity_ptr_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_075632_ctaskmove_dtorctaskmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075974_ctaskvehiclefsm_getdoorforentrypointindex_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_076411_ctaskmotioninautomobile_isvehicledrivable_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076466_ctaskmotiononbicycle_ispedpedalling_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076865_ctaskgun_getisfiring_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077153_statid_char_getstatid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077354_statsinterface_getfloatstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_077475_cnumberwithinmessage_set_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077479_ctextfile_get_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077481_ctextfile_get_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_077483_csubstringwithinmessage_setliteralstring_char_const_ptr_csubstri(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_077486_csubtitlemessage_set_char_const_ptr_int_cnumberwithinmessage_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32); }
extern "C" { pub fn f_077488_cmessages_insertnumbersandsubstringsintostring_char_const_ptr_cn(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32); }
extern "C" { pub fn f_077491_cmessages_addtopreviousbriefarray_csubtitlemessage_ref_eprevious(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077500_cmessages_clearalldisplayedmessagesthatbelongtothistextblock_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_077504_cmessages_addmessage_char_const_ptr_int_unsigned_int_bool_bool_e(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i64, a9: i32, a10: i32, a11: i32); }
extern "C" { pub fn f_077507_chelpmessage_setmessagetext_int_char_const_ptr_cnumberwithinmess(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_077561_ctextconversion_formatmstimeasstring_char_ptr_unsigned_long_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_077563_rage_fwtemplatestring_16ul_3ul_sprintf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077565_ctextfile_doestextlabelexist_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077629_ctimecycle_gettimecyclekeyframedata_ctimecycle_tcnameandvalue_pt(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077630_ctimecycle_settimecyclekeyframedata_ctimecycle_tcnameandvalue_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_077791_cpathfind_checkpathnodeintegrityforzone_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077792_cpathfind_testalllinks_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077812_cpathfind_findnodeclosesttocoors_rage_vector3_const_ref_float_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: f32, a11: i32, a12: i64, a13: i32, a14: i32); }
extern "C" { pub fn f_077824_cpathfind_forallnodesinarea_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_077878_cpathfind_calcdirectionmatchvalue_rage_cpathnode_const_ptr_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_077889_cpathfind_recordnodesincircle_rage_vector3_const_ref_float_int_r(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_077890_cpathfind_findnodeorientationforcarplacement_rage_cnodeaddress_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64) -> f32; }
extern "C" { pub fn f_077891_cpathfind_findnodeclosesttonodefavordirection_cpathfind_findnode(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077959_cvehicleintelligence_isonhighway_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078784_canchorhelper_anchor_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078786_cvehicle_getnumcontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078787_camphibiousautomobile_ispropellersubmerged(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078871_cvehicle_containslocalplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078876_camphibiousquadbike_iswheeltransitionfinished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078890_cvehicle_getattachparentvehicle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078893_cheli_getiscargobob_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078909_cvehicle_istaximodelid_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078924_cvehicle_getwheelfromid_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_078933_cvehicle_isupsidedown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078934_cvehicle_isonitsside_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078935_cvehicle_getisextraon_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078936_cvehicle_ishornon_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078940_csubmarinecar_arewheelsactive(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078941_cvehicledeformation_applydeformations_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078945_cvehicle_isdriveraplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078951_cvehicle_triggercaralarm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078953_cvehicle_hasrocketboost(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078954_cvehicle_hasjump(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078973_cvehicledamage_getoverallhealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_078974_cvehicledamage_getenginehealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_078975_cvehicledamage_getisstuck_int_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_078979_cvehicle_rollwindowup_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078981_cvehicle_hascontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078982_ccardoor_replayupdatedoors_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079000_cvehicle_istank_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079001_caircraftdamagebase_hassectionbrokenoff_cplane_const_ptr_int_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_079007_cvehicle_getnumberofpassenger_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079023_cvehicle_getattachedtrailer_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079030_cvehicledeformation_resetdamage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079040_cvehicle_getspecialflightmoderatio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_079041_cvehicle_rolldownwindow_ehierarchyid_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079047_cwheel_getwheelposandradius_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_079049_cvehicle_getdefaultbonepositionsimple_int_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_079065_cvehicle_getdoorfromid_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079070_cvehicle_getseatanimationinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079071_cvehicle_isentryindexvalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079134_cvehicle_getdriverseat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079157_cvehicle_getisjetski_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079169_cboat_getsearchlight(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079226_cvehicle_setliveryid_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079227_cvehicle_setlivery2id_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079228_cvehicle_updatebodycolourremapping_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079267_ccardoor_getisintact_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079270_cvehicle_iswindowdown_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079274_cvehicle_getwindowidfromdoor_ehierarchyid(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_079285_ccardoor_audioupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079335_chandlingdata_gethandlingdata_ehandlingtype(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079422_cvehicle_getvehiclegadget_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079440_cvehicle_resetskeletonformods(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079441_clandinggear_getpublicstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079472_cplane_getverticalflightmodeavaliable_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079544_cvehicle_setrocketboosting_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079546_cplane_setverticalflightmoderatio_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079572_csubmarinehandling_getcurrentdepth_cvehicle_ptr_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_079576_csubmarinehandling_processcrushdepth_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079642_cvehicle_removepedfromseat_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_079643_cvehicle_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079653_ctrain_removealltrains(ph: *mut Phantom); }
extern "C" { pub fn f_079659_ctrain_findengine_ctrain_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079670_ctrain_validatelinkedlists_ctrain_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079677_ctrain_getnumcarriages_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079678_ctrain_gettraintrack_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_079686_ctrain_carriagesaresuspended_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079688_ctrain_isengine_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079708_cvehicle_hasexpandedmods(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079713_cvehicle_hasnitrousboost(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079741_cvehicledeformation_updateshaderdamagevars_bool_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32); }
extern "C" { pub fn f_079743_cvehicledeformation_damagetextureallocate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079758_cvehicle_getconvertibleroofstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079761_cvehiclegadgettowarm_detachvehicle_cvehicle_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_079784_cvehicle_setvariationinstance_cstoredvehiclevariations_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079831_cvehicle_getdirectentrypointindexforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079841_cvehicle_isonfire_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079843_cvehicle_carhasroof_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079845_cvehicle_ispersonalvehicle(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079849_cvehicle_requestdials_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079853_cvehicle_pickrandompassenger(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079854_cvehicle_containsped_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079861_cwheel_applytyredamage_centity_const_ptr_float_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_079865_cvehicle_hasfragmentcomponent_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079866_cvehicle_turnoffextra_ehierarchyid_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079893_cvehicle_getwindownormalforsmash_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079904_cvehicle_placeonroadadjust_float_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> i32; }
extern "C" { pub fn f_079926_cvehicle_getliveryid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079928_cvehicle_getlivery2id_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080000_cvehicledamage_setenginehealth_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_080031_cvehiclegadgettowarm_deleteropeshookandconstraints_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080072_warpplayerintorecentcarcb(ph: *mut Phantom); }
extern "C" { pub fn f_080073_warpplayeroutofcarcb(ph: *mut Phantom); }
extern "C" { pub fn f_080080_cvehicleweaponmgr_getweaponsforseat_int_rage_atarray_cvehiclewea(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_080187_cvehiclefactory_createcar_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080291_cvehiclegadgettowarm_attachropestoobject_cvehicle_ptr_cphysical(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_080469_cstoredvehiclevariations_cstoredvehiclevariations(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080866_cvfxhelper_getmatrixfromboneindex_rage_mat34v_ref_bool_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_080874_cvfxhelper_creatematfromveczalign_rage_mat34v_ref_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_080879_cvfxhelper_getinterpvalue_float_float_float_bool(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_080883_cvfxhelper_getwaterdepth_rage_vec3v_const_ref_float_ref_rage_fwe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_080884_cvfxhelper_getwaterz_rage_vec3v_const_ref_float_ref_rage_fwentit(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_080886_cvfxhelper_testlineagainstwater_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_080889_cvfxhelper_isplayerinaccuratemodewithscope(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_080960_cfire_shutdown_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_080990_cptfxmanager_update_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_081161_vfxpedfootinfo_getdecalinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081164_cvfxpedinfo_getfootvfxinfo_vfxgroup_e_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_081165_cvfxpedinfomgr_getinfo_cped_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081166_cdecalmanager_findrendersettinginfo_unsigned_int_int_ref_int_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_081191_cvfxvehicleinfomgr_getinfo_cvehicle_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081254_rage_cloudhatmanager_unloadallcloudhats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081349_cdecalmanager_addgeneric_decaltype_e_int_int_rage_vec3v_const_re(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: f32, a7: f32, a8: f32, a9: f32, a10: f32, a11: f32, a12: f32, a13: f32, a14: i32, a15: i32, a16: i32, a17: f32, a18: i32, a19: i32, a20: f32) -> i32; }
extern "C" { pub fn f_081355_cfire_getpositionworld_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081357_cdecalmanager_addbang_vfxmaterialinfo_s_const_ref_vfxcollinfo_s(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_081358_cvfxmaterial_getbanginfo_vfxgroup_e_vfxgroup_e(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_081390_cdecalmanager_addscuff_centity_ptr_int_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: f32, a6: i32, a7: f32, a8: i32, a9: f32, a10: f32, a11: i32) -> i32; }
extern "C" { pub fn f_081392_cdecalmanager_remove_rage_fwentity_const_ptr_int_void_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_081445_cfiremanager_startfire_cfiresettings_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081448_cfire_getmaxradius_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_081449_cfiremanager_getmapstatusgridindices_rage_vec3v_const_ref_int_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_081461_cfiremanager_extinguishall_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081464_cfiremanager_extinguishentityfires_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081470_cfiremanager_clearreplayfireflag_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081494_cmoviemgr_stop_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081495_cmoviemgr_releaseall(ph: *mut Phantom); }
extern "C" { pub fn f_081498_cmoviemgr_preloadmovie_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_081499_cmoviemgr_play_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081500_cmoviemgr_release_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081504_cmoviemgr_findmovie_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_081505_cmoviemgr_setfrontendaudio_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_081506_cmoviemgr_attachaudiotoentity_unsigned_int_centity_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_081509_cmoviemgr_settimereal_unsigned_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_081512_cmoviemgr_isplaying_unsigned_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_081513_cmoviemgr_isloaded_unsigned_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_081576_ctvplaylistmanager_scaletorealtime_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_081577_ctvplaylistmanager_settvchannel_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_081628_clodlightmanager_registerbrokenlightsforentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081629_clodlightmanager_unregisterbrokenlightsforentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081652_cptfxmanager_removeptfxinrange_rage_vec3v_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_081713_cptfxmanager_waitforupdatetofinish(ph: *mut Phantom); }
extern "C" { pub fn f_081745_cvehicleglassmanager_smashcollision_centity_ptr_int_float_rage_v(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_081782_cvfxentity_getptfxassetname_eparticletype_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_081785_cvfxentity_updateptfxentityanim_centity_ptr_cparticleattr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_081787_cvfxentity_triggerptfxentityanim_centity_ptr_cparticleattr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081847_cvfxped_triggerptfxpedbreath_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081848_cvfxped_updateptfxpedbreathwater_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081854_cvfxscript_getscriptptfxassetname_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_081864_cvfxvehicle_triggerptfxlightsmash_cvehicle_ptr_int_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_082065_cpedweaponmanager_getequippedweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082069_cweapon_getmuzzleposition_rage_matrix34_const_ref_rage_vector3_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082071_cpedweaponmanager_getequippedweapon_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082122_cexplosionmanager_removeexplosionsinarea_eexplosiontag_rage_vect(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_082162_cweapon_updateshadervariables_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082179_cpedweaponmanager_destroyequippedweaponobject_bool_char_const_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_082218_cweapon_shutdowncomponent_cweaponcomponent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082219_cweapon_addcomponent_cweaponcomponent_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_082220_cweapon_update_hd_models_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082228_cweapondebug_clearallprojectiles(ph: *mut Phantom); }
extern "C" { pub fn f_082253_weaponfactory_create_unsigned_int_int_cdynamicentity_ptr_char_co(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_082267_citeminfo_getmodelindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082274_cweaponcomponent_updateshadervariables_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082276_weaponcomponentfactory_create_cweaponcomponentinfo_const_ptr_cwe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_082597_cpedweaponselector_getisweaponvalid_cped_const_ptr_cweaponinfo_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_082632_cpedweaponmanager_destroyequippedgadgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082653_cpedweaponmanager_getisarmedgun_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082671_cweaponitem_getkeyfromhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083007_clivemanager_issignedin_int(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083012_networkinterface_getlocalgamername(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083028_networkinterface_isinspectatormode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083031_networkinterface_islocalplayeronsctvteam(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083032_networkinterface_isoverridespectatedvehicleradioset(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083059_networkinterface_gethostname(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083141_networkinterface_isentityconcealed_centity_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083493_ctunables_access_unsigned_int_bool_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083494_ctunables_access_unsigned_int_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083495_ctunables_access_unsigned_int_float_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083900_csyncedentity_setentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083902_csyncedentity_getentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086137_cnetworkvoice_isanyremotegamertalking_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_089540_cpedconversationlineevent_trigger_cped_const_ptr_unsigned_int_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_089758_cplaysoundevent_trigger_rage_vector3_const_ref_unsigned_int_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_089759_cplaysoundevent_trigger_centity_const_ptr_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_089767_cstopsoundevent_trigger_unsigned_char(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_089782_caudiobankrequestevent_trigger_unsigned_int_cgamescriptid_ref_bo(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_090700_tzset(ph: *mut Phantom); }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090741_time(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090755_exp2f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090762_fclose(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090775_fopen(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090782_fread(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_090784_fseeko(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090788_ftello(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090790_fwrite(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_090801_localtime(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090805_log10f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090807_logf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090814_memcmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090818_modff(ph: *mut Phantom, a0: f32, a1: i64) -> f32; }
extern "C" { pub fn f_090826_powf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090901_qsort_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_090911_rand(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090918_sinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090920_snprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090921_siprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090922_small_sprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090923_sscanf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090925_statvfs(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090926_strcasecmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090927_strcat(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090928_strchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090930_strcmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090931_strcpy(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090939_strlen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090940_strncasecmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090942_strncmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090945_strrchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090947_strstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090956_strtok(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090959_strtoll(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_090965_tanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090971_toupper(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090985_vsnprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090991_wcslen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090999_wmemcpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_091002_emscripten_builtin_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091023_multi3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
