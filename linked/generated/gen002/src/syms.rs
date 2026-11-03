//! Callee declarations for crate gen002.
//!
//! Every `call $name` in the wasm module becomes a call through this
//! module: an `extern "C"` declaration of the target's generated
//! identifier.  That keeps a function's callers independent of which
//! generated crate the callee ended up in (they are all linked into the
//! same binary and the identifiers are unique).
#![allow(improper_ctypes, dead_code, unused_imports)]
use wasm_rt::Phantom;
extern "C" { pub fn f_010440_ragedirect3ddevicecontext11_csclearconstantbuffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010441_ragedirect3ddevicecontext11_dsclearconstantbuffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010442_ragedirect3ddevicecontext11_gsclearconstantbuffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010443_ragedirect3ddevicecontext11_hsclearconstantbuffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010444_ragedirect3ddevicecontext11_psclearconstantbuffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010445_ragedirect3ddevicecontext11_vsclearconstantbuffers(ph: *mut Phantom, a0: i64); }

extern "C" { pub fn f_000100_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000101_op_new_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000102_op_delete_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000109_rage_syscriticalsectiontoken_syscriticalsectiontoken_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000111_rage_syscriticalsectiontoken_lock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000113_rage_syscriticalsectiontoken_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000114_rage_sysipccreatesema_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_000116_rage_sysipcwaitsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000118_rage_sysipcwaitsematimed_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_000119_rage_sysipcsignalsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000120_rage_sysipccreatemutex(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_000121_rage_sysipclockmutexcommand_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000122_rage_sysipcunlockmutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000123_rage_sysipcdeletemutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000124_rage_sysipccreateevent(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_000125_rage_sysipcwaitevent_rage_sysipceventtag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000127_rage_sysipcsetevent_rage_sysipceventtag_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000128_rage_sysipccreatethread_void_ptr_void_ptr_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_000135_rage_sysipcsleep_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000212_rage_fidevicetcpip_connect_char_const_ptr_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000221_void_std_2_tree_balance_after_insert_abi_nqn220108_std_2_tree_no(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000222_void_std_2_tree_remove_abi_nqn220108_std_2_tree_node_base_void_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000237_rage_grcdevice_computewindowdimensions_tagrect_ref_unsigned_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000239_rage_grcdevice_manage(ph: *mut Phantom); }
extern "C" { pub fn f_000243_rage_grcdevice_registerdevicelostcallbacks_rage_functor0_rage_fu(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000244_wasm_d3d_map_hint(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_000272_wasm_null_d3d_createdxgifactory_idxgifactory_ptr_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000560_getclientrect_hwnd_ptr_tagrect_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000561_getwindowrect_hwnd_ptr_tagrect_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000562_setwindowpos_hwnd_ptr_hwnd_ptr_int_int_int_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_000643_rage_atorderlt_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000644_rage_athashnextsize_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000645_rage_athash_const_char_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000646_rage_athash64_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000649_rage_dattypestruct_addfield_unsigned_long_unsigned_long_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000650_rage_athash16u_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000653_rage_atstring_set_rage_atstring_const_ref_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_000654_rage_atstring_set_char_const_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_000655_rage_atstring_lowercase(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000663_rage_atstring_replace_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000665_rage_atstring_op_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000668_rage_atarray_rage_atstring_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_000671_rage_atstring_split_rage_atarray_rage_atstring_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000673_rage_atstring_serialize_rage_datserialize_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000675_rage_functorbase_functorbase_void_const_ptr_void_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_000680_rage_athashstringnamespacesupport_computehash_rage_athashstringn(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_000681_rage_athashstringnamespacesupport_addstring_rage_athashstringnam(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_000684_rage_athashstringnamespacesupport_trygetstring_rage_athashstring(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_000690_rage_bkwidget_bkwidget_rage_datcallback_ref_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_000692_rage_datcallback_call_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000696_rage_bkwidget_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000703_rage_bkbank_unsetcurrentgroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000704_rage_bkbank_pushgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000707_rage_bkbank_popgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000708_rage_bkbank_truncate_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000713_rage_bkremotepacket_sendpackets(ph: *mut Phantom); }
extern "C" { pub fn f_000714_rage_bkwidget_remotecreate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000716_rage_bkremotepacket_send(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000718_rage_bkremotepacket_isconnected(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000758_rage_bkmanager_openfile_char_ptr_int_char_const_ptr_bool_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_000781_rage_bkwidget_drawf_int_int_char_const_ptr_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_000782_rage_bkmanager_findbank_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000787_rage_bkconsole_bkconsole_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_000799_rage_bkwidget_drawlocal_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_000806_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000807_rage_bkio_dtorbkio(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000808_rage_bkremotepacket_bkremotepacket(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000814_rage_bkremotepacket_connect_bool_char_const_ptr(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000818_rage_bkremotewidgetmap_getid_rage_bkwidget_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000822_rage_bkimageviewer_setimage_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000843_rage_bkcombo_setstring_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_000851_char_ptr_rage_formatf_char_8ul_char_ref_8ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000866_rage_bkgroup_addseparator_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000867_rage_bkgroup_addtitle_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000868_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_000869_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000872_rage_bkgroup_addtext_char_const_ptr_float_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000874_rage_bkgroup_addtext_char_const_ptr_int_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000877_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_000880_rage_bkgroup_addtext_char_const_ptr_rage_atnonfinalhashstring_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000882_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000886_rage_bkgroup_addtoggle_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000887_rage_bkgroup_addtoggle_char_const_ptr_unsigned_short_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_000889_rage_bkgroup_addtoggle_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_000894_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000905_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000907_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int_rage_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000909_rage_bkgroup_addslider_char_const_ptr_rage_vector2_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000910_rage_bkgroup_addslider_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64); }
extern "C" { pub fn f_000911_rage_bkgroup_addslider_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000912_rage_bkgroup_addslider_char_const_ptr_rage_vector4_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000916_rage_bkgroup_addcombo_char_const_ptr_unsigned_char_ptr_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_000918_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000919_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000926_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000927_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000928_rage_bkgroup_addvector_char_const_ptr_rage_vector4_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000936_rage_bkgroup_addangle_char_const_ptr_float_ptr_rage_bkangletype(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000938_rage_bkgroup_addangle_char_const_ptr_float_ptr_rage_bkangletype(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: i64); }
extern "C" { pub fn f_000941_rage_bkgroup_addcolor_char_const_ptr_rage_color32_ptr_rage_datca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_000945_rage_bkgroup_addcolor_char_const_ptr_rage_vector4_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000946_rage_bkgroup_addcolor_char_const_ptr_rage_vector3_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001100_rage_autoiddescriptor_autoiddescriptor_unsigned_int_char_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_001102_rage_autoiddescriptor_autoiddescriptor_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_001105_rage_autoidregistry_autoidregistry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001106_rage_autoidregistry_dtorautoidregistry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001107_rage_autoidregistry_register_rage_autoiddescriptor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001108_rage_autoidregistry_unregister_rage_autoiddescriptor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001116_char_ptr_rage_formatf_char_32ul_char_ref_32ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001117_rage_datcallback_datcallback(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001119_rage_datcallback_datcallback_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001120_rage_datcallback_datcallback_void_ptr_void_ptr_void_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_001122_rage_datnewline_rage_datserialize_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001123_rage_datgrowbuffer_datgrowbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001124_rage_datgrowbuffer_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001125_rage_datgrowbuffer_dtordatgrowbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001126_rage_datgrowbuffer_init_rage_sysmemallocator_ptr_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001129_rage_datgrowbuffer_getfihandle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001130_rage_datgrowbuffer_append_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001131_rage_datgrowbuffer_preallocate_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001136_rage_datgrowbuffer_remove_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001138_rage_datgrowbuffer_getbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001140_rage_datbitbuffer_readunsigned_void_const_ptr_unsigned_int_ref_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001141_rage_datbitbuffer_writeunsigned_void_ptr_unsigned_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_001143_rage_datbitbuffer_datbitbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001144_rage_datbitbuffer_setreadonlybits_void_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001146_rage_datbitbuffer_getreadwritebits(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001147_rage_datbitbuffer_setreadonlybytes_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001148_rage_datbitbuffer_setreadwritebytes_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001149_rage_datbitbuffer_getnumbyteswritten_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001150_rage_datbitbuffer_setnumbitswritten_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001151_rage_datbitbuffer_setcursorpos_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001152_rage_datbitbuffer_getnumbytesread_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001155_rage_datbitbuffer_readbits_void_ptr_int_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001156_rage_datbitbuffer_bumpreadcursor_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001158_rage_datbitbuffer_bumpreadwritecursor_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001159_rage_datbitbuffer_writebits_void_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001161_rage_datbitbuffer_readuns_unsigned_long_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001163_rage_datbitbuffer_writeuns_unsigned_long_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001166_rage_datbitbuffer_readbool_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001167_rage_datbitbuffer_writebool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001170_rage_datbitbuffer_readstr_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001171_rage_datbitbuffer_writestr_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001197_rage_sha1_sha1(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001232_rage_rsonreader_rsonreader(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001233_rage_rsonreader_rsonreader_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001239_rage_rsonreader_getnextsibling_rage_rsonreader_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001240_rage_rsonreader_getfirstmember_rage_rsonreader_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001243_rage_rsonreader_getname_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001246_rage_rsonreader_asint_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001248_rage_rsonreader_asuns_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001250_rage_rsonreader_asfloat_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001251_rage_rsonreader_asbool_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001253_rage_rsonreader_asstring_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001269_rage_rsonreader_validatejson_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001275_rage_datsaxreader_begin(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001276_rage_datsaxreader_end(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001277_rage_datsaxreader_failed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001278_rage_datsaxreader_parse_char_const_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
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
extern "C" { pub fn f_001541_rage_diagloggedprintln_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001543_rage_diagloggedprintf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001551_rage_fprintf_rage_fistream_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001553_rage_firemoteshowmessagebox_char_const_ptr_char_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001559_rage_diagcontextmessage_diagcontextmessage_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001560_rage_diagcontextmessage_diagcontextmessage_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001563_rage_diagcontextmessage_pop(ph: *mut Phantom); }
extern "C" { pub fn f_001565_printf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001568_rage_atmapentry_unsigned_int_wchar_t_ptr_const_ref_rage_atmap_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001576_rage_fiassetmanager_fullpath_char_ptr_int_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_001577_rage_fistream_open_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001578_rage_fistream_seek_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001580_rage_fiassetmanager_open_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001581_rage_fiassetmanager_fullreadpath_char_ptr_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_001582_rage_fiassetmanager_createleadingpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001615_rage_restservicemanager_removeanddeleteservice_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001622_rage_fistream_read_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001623_rage_fiassetmanager_findextensioninpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001627_rage_fiassetmanager_exists_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001631_rage_fiassetmanager_findpath_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001632_rage_fiassetmanager_pushfolder_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001633_rage_fiassetmanager_popfolder(ph: *mut Phantom); }
extern "C" { pub fn f_001635_rage_fiassetmanager_filename_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001636_rage_fiassetmanager_basename_char_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001637_rage_fiassetmanager_enumfiles_char_const_ptr_void_ptr_rage_fifin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_001639_rage_fiassetmanager_removeextensionfrompath_char_ptr_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001640_rage_fiassetmanager_removenamefrompath_char_ptr_int_char_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001641_rage_fiassetmanager_insertpathelement_char_ptr_int_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001644_rage_fistream_getch(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001645_rage_fistream_putch_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001647_rage_fistream_tell(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001650_rage_fgetline_char_ptr_int_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_001651_rage_fistream_preload_rage_fistream_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001652_rage_fidevice_makememoryfilename_char_ptr_int_void_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_001654_rage_fiserialize_fiserialize_rage_fistream_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_001655_rage_fibasetokenizer_putstr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001656_rage_fibasetokenizer_init_char_const_ptr_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001657_rage_fibasetokenizer_ignoretoken(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001667_rage_fiasciitokenizer_getint_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001673_rage_fibasetokenizer_gettoken_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001675_rage_fibasetokenizer_getstreamposition(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001676_rage_fibasetokenizer_matchtoken_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001678_rage_fibasetokenizer_checktoken_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001679_rage_fibasetokenizer_checkitoken_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001681_rage_fibasetokenizer_gettokentochars_char_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001682_rage_fibasetokenizer_skiptoendofline(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001735_rage_fitokenizer_fitokenizer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001736_rage_fitokenizer_fitokenizer_char_const_ptr_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001949_rage_fidevicetcpip_setlasterror_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002028_rage_angleclamp_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_002031_rage_mthrandom_mthrandom_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002032_rage_mthrandom_setfullseed_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002038_rage_pgstreamer_open_char_const_ptr_unsigned_int_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002043_rage_pgstreamer_close_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_002044_rage_pgrscbuilder_computeleafsize_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002046_rage_pgrscbuilder_cleanup_rage_datresourcemap_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002063_rage_pgstreamer_read_unsigned_int_rage_datresourcechunk_ptr_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_002080_rage_pgbase_pgbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002081_rage_pgbase_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002084_rage_pgbase_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002088_rage_datresource_datresource_rage_datresourcemap_const_ref_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002090_rage_datresource_dtordatresource(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002091_rage_datresource_containsthisaddress_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002092_rage_datresource_getfixup_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002129_rage_atmapentry_rage_atnonfinalhashstring_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002130_rage_atmap_rage_atnonfinalhashstring_rage_atnonfinalhashstring_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002142_rage_parattributelist_findattribute_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002145_rage_parmanager_loadtree_rage_fistream_ptr_rage_parsettings_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002147_rage_parmanager_loadtree_char_const_ptr_char_const_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002157_rage_parinstancevisitor_visitstructure_rage_parparsablestructure(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002158_rage_parinstancevisitor_visittoplevelstructure_rage_parparsables(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002159_rage_parmanager_loadfromstructure_rage_partreenode_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_002161_rage_parstructure_issubclassof_rage_parstructure_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002165_rage_parmanager_createandloadanytype_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002168_rage_parmanager_createandloadanytype_rage_fistream_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_002169_rage_parmanager_buildtreenodefromstructure_rage_parstructure_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_002172_rage_parmanager_savefromstructure_rage_fistream_ptr_rage_parstru(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_002175_rage_partree_dtorpartree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002177_rage_partreenode_insertaschildof_rage_partreenode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002178_rage_parattributelist_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002182_rage_parelement_findattribute_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002184_rage_partreenode_findchildwithname_char_const_ptr_rage_partreeno(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002186_rage_parattribute_findint64value_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002188_rage_partreenode_appendaschildof_rage_partreenode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002192_rage_parattributelist_addattribute_char_const_ptr_double_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f64, a3: i32) -> i64; }
extern "C" { pub fn f_002194_rage_parelement_addattribute_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_002195_rage_parattributelist_addattribute_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002198_rage_parattributelist_addattribute_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_002199_rage_partreenode_createstdleaf_char_const_ptr_char_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002200_rage_parattributelist_addattribute_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_002206_rage_parelement_findattributeint64value_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002211_rage_partreenode_findvaluefrompath_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002212_rage_partreenode_findvaluefrompath_char_const_ptr_rage_vector3_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002214_rage_parattributelist_copyfrom_rage_parattributelist_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002243_rage_atmap_rage_atstring_rage_psoresourcedata_ptr_rage_atmaphash(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002278_rage_psofile_dtorpsofile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002285_rage_psoinitfromresource_rage_psoresourcedata_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002298_comparesourceaddr_rage_psorscbuilderinstance_ptr_rage_psorscbuil(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002302_rage_psoresourcedata_place_rage_psoresourcedata_ptr_rage_datreso(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002311_rage_atarray_rage_psoenumtableentry_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002338_rage_parrestexpireguid_rage_parparsablestructure_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002343_rage_parrestregistersingleton_char_const_ptr_rage_parparsablestr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002365_rage_athashedmap_char_const_ptr_char_const_ptr_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002370_rage_parattributelist_mergefrom_rage_parattributelist_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
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
extern "C" { pub fn f_002447_rage_parinstancevisitor_parinstancevisitor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002581_rage_parelement_setname_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002584_rage_parattributelist_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_002586_void_rage_parcgudestructor_rage_parattributelist_rage_parparsabl(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002619_rage_parstreamoutxml_writeescapedcharacter_rage_fistream_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_002695_rage_atbinarymap_rage_pardelegateholderbase_ptr_unsigned_int_dat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002696_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002697_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_002702_rage_qabug_clearbugbuffer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002704_rage_qabug_setowner_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002710_rage_qabug_addcustomfield_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002712_rage_qabugstarinterface_qabugstarinterface(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002713_rage_qabugstarinterface_createbug_rage_qabug_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_002719_strlwr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002720_rage_conststring_vcopy_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002721_rage_conststring_dtorconststring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002722_rage_conststring_conststring_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002723_rage_conststring_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002739_rage_atpartialstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002741_rage_atliteralstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002743_rage_atdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002744_rage_atdatahash_unsigned_int_const_ptr_unsigned_long_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002746_rage_stristr_char_ptr_char_const_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002757_rage_countutf8characters_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002759_rage_utf8towidechar_char_const_ptr_unsigned_short_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002760_rage_widetoutf8_char_ptr_unsigned_short_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002762_rage_utf8towide_unsigned_short_ptr_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002766_crc32(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002767_deflateinit2(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_002770_deflateend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002773_deflate(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002790_rage_zlibinflater_zlibinflater(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002801_rage_sysexception_hasbeenthrown(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002810_rage_syshangdetecttick(ph: *mut Phantom); }
extern "C" { pub fn f_002811_rage_syshangdetectsetcrashfunc_void_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002812_rage_sysnamedpipe_sysnamedpipe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002814_rage_sysnamedpipe_open_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002815_rage_sysnamedpipe_read_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002820_rage_sysnamedpipe_setnodelay_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002821_rage_sysnamedpipe_setblocking_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002822_rage_syspipepacket_write_u8_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002824_rage_syspipepacket_write_u32_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002825_rage_syspipepacket_write_u64_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002826_rage_syspipepacket_write_float_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_002827_rage_syspipepacket_write_const_char_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002833_rage_syspipepacket_send(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002839_rage_sysparam_get_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002840_rage_sysparam_get_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002841_rage_sysparam_getarray_char_const_ptr_ptr_int_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_002842_rage_sysparam_getarray_int_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002843_rage_sysparam_getarray_float_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002844_rage_sysparam_getparambyname_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002878_rage_atmap_unsigned_int_rage_sysstackcollector_tagentry_rage_atm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002881_unsigned_int_ref_rage_atmap_unsigned_int_unsigned_int_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002889_rage_systhreadpool_workitem_dtorworkitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002912_rage_systhreadregistry_adddelayedwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_002913_rage_systhreadregistry_enablewidgetcreation_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_002927_rage_sysmemstarttemp(ph: *mut Phantom); }
extern "C" { pub fn f_002928_rage_sysmemendtemp(ph: *mut Phantom); }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_002931_rage_physical_new_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002932_rage_physical_delete_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002935_rage_sysmemtotalfreememory(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002936_rage_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002939_rage_sysmemmanager_sysmemmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002950_rage_sysmemsimpleallocator_sysmemsimpleallocator_void_ptr_int_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_003065_rage_sysmemcontainer_sysmemcontainer_rage_sysmemcontainerdata_re(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003066_rage_sysmemcontainer_dtorsysmemcontainer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003071_rage_sysmemcontainer_init_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003072_rage_sysmemcontainer_finalize(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003073_rage_sysmemcontainer_beginshutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003074_rage_sysmemcontainer_endshutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003085_rage_systaskparameters_systaskparameters(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003090_rage_systaskmanager_create_char_const_ptr_void_ptr_rage_systaskp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_003095_rage_systaskmanager_wait_rage_systaskhandle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003097_char_ptr_rage_formatf_char_16ul_char_ref_16ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003142_rage_geomtvalues_findtvaluesegtopoint_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_003159_rage_matrix34_fromquaternion_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003160_rage_matrix34_toquaternion_rage_quaternion_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003169_rage_packeulersto32_rage_vector3_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003179_rage_atpoolbase_init_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003182_rage_atpoolbase_releasemem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003222_rage_crvcrwidget_crvcrwidget_char_const_ptr_char_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003231_rage_cranimation_cranimation(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003232_rage_cranimation_cranimation_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003233_rage_atarray_rage_datref_rage_crblockstream_0_unsigned_short_dec(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003253_rage_cranimation_load_char_const_ptr_unsigned_char_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_003297_rage_cranimdictionary_cranimdictionary(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003302_rage_cranimdictionary_loadanimations_rage_atarray_rage_atstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_003303_rage_cranimloaderbasic_cranimloaderbasic_bool_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003304_rage_cranimloaderbasic_init_bool_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003433_rage_atmap_unsigned_int_rage_crframedata_ptr_rage_atmaphashfn_un(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003607_rage_mat34v_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004280_rage_fwposematcher_fwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004603_rage_fwextensionlist_add_rage_fwextension_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004689_rage_fwextensionlist_destroy_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005454_rage_fwboxstreamer_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005502_rage_fwassetstore_rage_pgdictionary_rage_charactercloth_rage_fwc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005515_rage_pgdictionary_rage_grctexture_setcurrent_rage_pgdictionary_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005566_rage_fwassetstore_rage_pgdictionary_rage_grctexture_rage_fwtxdde(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005570_rage_fwboxstreamer_dtorfwboxstreamer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005584_rage_fwpsostoreloader_load_void_ptr_int_char_const_ptr_rage_fwps(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_005586_rage_fwpsostoreloader_readobjectsfrompsofile_rage_psofile_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_005588_rage_fwpsostoreloader_unload_rage_fwpsostoreloadinstance_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_005592_rage_fwpsostoreplacementcookies_cookie_ref_rage_atmap_rage_strin(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005646_rage_atmap_rage_strindex_rage_fwpsostoreplacementcookies_cookie(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005648_rage_fwpsostoreloader_load_char_const_ptr_rage_fwpsostoreloadins(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_005649_rage_fwpsostoreloader_runpostloadcallbacks_rage_parstructure_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005710_rage_atarray_rage_datowner_rage_grctexture_0_unsigned_short_atar(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005824_void_rage_pgrscbuilder_load_rage_crclipdictionary_rage_crclipdic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_005917_rage_fwrefawarebaseimpl_rage_datbase_dtorfwrefawarebaseimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006035_rage_atmapentry_unsigned_int_bool_const_ref_rage_atmap_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006037_rage_strstreamingloadermanager_callkeepalivecallbackifnecessary(ph: *mut Phantom); }
extern "C" { pub fn f_006056_rage_strstreaminginfomanager_removeobject_rage_strindex_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006064_rage_strstreamingvisualize_send_int_void_const_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_006072_rage_strstreaminginfomanager_getobjectname_rage_strindex_char_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_006080_rage_strstreaminginfo_setflag_rage_strindex_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006088_rage_atarray_rage_atnonfinalhashstring_0_unsigned_short_grow_uns(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006175_rage_strstreaminginfomanager_clearrequiredflag_rage_strindex_uns(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006225_rage_atvarstring_atvarstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_006243_rage_strstreamingloadermanager_requestasyncplacement_rage_strstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006249_rage_strstreamingmodule_resetplacestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006253_rage_strmoduletypeindextoextension_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006361_rage_atarray_float_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007176_rage_audmixerdevice_audmixerdevice(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007177_rage_audmixerdevice_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007200_char_ptr_rage_formatf_char_255ul_char_ref_255ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007655_rage_fwprofanityfilter_checkforprofanityandreservedterms_char_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007753_rage_netinterface_getlocalplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007756_rage_netinterface_flushalllogfiles_bool(ph: *mut Phantom); }
extern "C" { pub fn f_007759_rage_netinterface_getnumphysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007760_rage_netinterface_getnumremotephysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007762_rage_netinterface_getallphysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007763_rage_netinterface_getremotephysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007769_rage_netinterface_getlocalphysicalplayerindex(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007777_rage_netinterface_getdefaultlogfiletargettype(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007778_rage_netinterface_getdefaultlogfileblockingmode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007780_rage_netplayermgrbase_sendbuffer_rage_netplayer_const_ptr_void_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_007781_rage_netinterface_adddelegate_rage_atdelegator1_void_rage_receiv(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007783_rage_netinterface_removedelegate_rage_atdelegator1_void_rage_rec(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007791_rage_netlogimpl_netlogimpl_char_const_ptr_rage_logopentype_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_007792_rage_netbandwidthstatistics_allocatebandwidthrecorder_char_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007793_rage_netarraymanager_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007799_rage_netbandwidthstatistics_addbandwidthout_short_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_007802_rage_netplayer_ismyplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007803_rage_networklogutils_writelogevent_rage_netlogginginterface_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007804_rage_networklogutils_writemessageheader_rage_netlogginginterface(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_007809_rage_netbandwidthstatistics_addbandwidthin_short_unsigned_int_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_007812_rage_netarrayhandlerbase_getunsyncedplayers_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007813_rage_netarraymanager_script_unregisterallscriptbroadcastdata_rag(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007814_rage_netarraymanager_script_swaplocalandremoteplayerbroadcastdat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007815_rage_netarrayhandlerbase_issyncedwithallplayers_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007822_rage_netarrayhandlerbase_resetemptyelements(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007824_rage_netsyncdatabase_initialiseplayersyncdatafordataunit_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_007830_rage_netsyncdatabase_copycurrentstatedata_unsigned_int_rage_datb(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_007836_rage_netarrayhandlerbase_setelementdirty_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007839_rage_netarrayhandlerbase_verifychecksum_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_007855_rage_netplayer_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007862_rage_nethostbroadcastdatahandlerbase_setnextfreehostobjectid_uns(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007905_rage_netcloudrequesthelper_cancel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007995_rage_matrix34_toeulersxyz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008007_rage_atstring_atstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_008062_rage_encryptbuffer_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008170_rage_addnodeandexternaldependentnodes_rage_netsyncdatanode_ptr_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_008215_rage_netsyncdatanode_forcesend_unsigned_int_unsigned_int_rage_ne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_008246_rage_netplayer_getrlgamerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008247_rage_netplayer_setnonphysicaldata_rage_nonphysicalplayerdatabase(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008271_rage_netplayermgrbase_updateplayerarrays(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008278_rage_netplayermgrbase_updateplayerlistsforplayer_rage_netplayer(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008295_rage_netsyncparentnode_addchildnode_rage_netsyncnodebase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008302_rage_netsynctree_setdatanodeindices_rage_netsyncdatanode_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_008307_rage_netsyncdatanode_forcesendtoplayer_unsigned_char_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_008332_rage_netleaderboardread_init_unsigned_int_rage_rlleaderboard2typ(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_008333_rage_netleaderboardread_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008334_rage_netleaderboardread_cancel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008341_rage_netleaderboardread_allocaterowdata_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008344_rage_netleaderboardreadgamersbyrow_start_int_rage_rlleaderboard2(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_008345_rage_netleaderboardreadgamersbyrank_start_int_rage_rlleaderboard(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_008346_rage_netleaderboardreadgamersbyradius_start_int_rage_rlleaderboa(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_008353_rage_netleaderboardmanageallocator_init_rage_sysmemallocator_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008354_rage_netleaderboardmanageallocator_startusingmemallocator(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008355_rage_netleaderboardmanageallocator_stopusingmemallocator(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008357_rage_netleaderboardreadmgr_cancelall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008358_rage_netleaderboardreadmgr_deleteall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008364_rage_netleaderboardreadmgr_addreadtoreadlist_rage_netleaderboard(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008373_rage_netleaderboardwritemgr_dtornetleaderboardwritemgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008374_rage_netleaderboardwritemgr_cancel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008375_rage_netleaderboardwritemgr_deleteall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008380_rage_atarray_leaderboardcategorygroup_0_unsigned_short_op_rage_a(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008381_rage_atarray_leaderboard_0_unsigned_short_destruct_leaderboard_p(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008390_rage_netscgamerdatamgr_requestinfo_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008391_rage_netscgamerdatamgr_ischeatersc_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008392_rage_netscgamerdatamgr_isbadsportsc_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008393_rage_netscgamerdatamgr_getactivexpbonussc_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008395_rage_atarray_rage_netscgamerdatamgr_gamerdatavalue_0_unsigned_sh(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008410_netprofanityfilter_netprofanityfilterrequest_checktext_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_008414_netlicenseplatecheck_netlicenseplatecheckrequest_checktext_int_c(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_008424_netlicenseplateisvalid_netlicenseplateisvalidrequest_isvalid_int(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_008438_rage_ccommerceconsumablemanager_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008443_rage_ccommercedata_dtorccommercedata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008444_rage_ccommercedata_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008445_rage_atmap_rage_atstring_rage_atstring_rage_atmaphashfn_rage_ats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008448_rage_ccommercedata_isiteminsuppresslist_rage_ccommerceitemdata_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008449_rage_ccommercedata_hasmissingsubscriptionupsells_rage_ccommercep(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008452_rage_atmapentry_rage_atstring_int_const_ref_rage_atmap_rage_atst(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008455_rage_ccommerceitemdata_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008457_rage_ccommerceitemdata_getpriorityfortag_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008482_rage_rlv2entitlementresponse_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008483_rage_rlv2entitlementinstance_dtorrlv2entitlementinstance(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008484_rage_cscscommerceconsumablemanager_dtorcscscommerceconsumableman(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008494_rage_cscscommerceconsumabletransaction_startnexttransactioninsta(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008548_rage_netcloudrequesthelper_dtornetcloudrequesthelper_14007(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008581_rage_ccommerceitemdata_dtorccommerceitemdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008583_rage_ccommerceproductdata_dtorccommerceproductdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008594_rage_cscscommerceconsumabletransaction_dtorcscscommerceconsumabl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008598_rage_netscgamerdatamgr_dtornetscgamerdatamgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008606_bool_rage_datbitbuffer_writeint_signed_char_signed_char_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008626_bool_rage_datbitbuffer_readint_signed_char_signed_char_ref_int_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008628_bool_rage_datbitbuffer_readint_short_short_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008682_rage_fwbasepool_fwbasepool_int_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_008688_rage_fwbasepool_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008701_rage_scripthandlermgr_writescriptheader_rage_netlogginginterface(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_008702_rage_scripthandlernetcomponent_finishbroadcastingdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008704_rage_scripthandlernetcomponent_getslotparticipantisusing_rage_ne(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008718_rage_scripthandler_areallparticipantsuptodatewithhoststate_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008743_rage_msgscriptjoinhostack_log_rage_netlogginginterface_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008744_rage_msgscripthandshakeack_log_rage_netlogginginterface_ref_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008746_unsigned_int_rage_netmessage_readheader_rage_datbitbuffer_unsign(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008749_rage_scripthandlermgr_deletehandlerwithhash_rage_athashvalue(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008758_bool_rage_scripthandlermgr_sendmessagetoplayer_rage_msgscriptjoi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008759_bool_rage_msgscripthandshake_sermsgpayload_rage_datimportbuffer(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008760_bool_rage_scripthandlermgr_sendmessagetoplayer_rage_msgscripthan(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008761_bool_rage_scripthandlermgr_sendmessagetoplayer_rage_msgscriptlea(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008762_bool_rage_scripthandlermgr_sendmessagetoplayer_rage_msgscriptmig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_008763_bool_rage_scripthandlermgr_sendmessagetoplayer_rage_msgscriptver(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008764_bool_rage_datbitbuffer_readint_int_int_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_008765_bool_rage_netmessage_writeheader_rage_datexportbuffer_unsigned_i(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_008774_rage_scripthandlernetcomponent_dtorscripthandlernetcomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008775_rage_scripthandlernetcomponent_removeallcandidates_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008780_bool_rage_scripthandlermgr_sendmessagetoplayer_rage_msgscripthan(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_008782_rage_scripthandlernetcomponent_getparticipantsdata_rage_netplaye(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_008783_bool_rage_msgscripthandshake_sermsgpayload_rage_datexportbuffer(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008785_rage_scripthandlernetcomponent_removeplayerascandidate_rage_netp(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008786_rage_scripthandlernetcomponent_addplayerascandidate_rage_netplay(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008788_rage_scripthandlernetcomponent_setnextstate_rage_scripthandlerne(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008789_void_rage_scripthandlermgr_sendmessagetoallparticipants_rage_msg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008790_bool_rage_scripthandlermgr_sendmessagetoplayer_rage_msgscriptlea(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_008791_rage_scripthandlernetcomponent_setwaitingforack_rage_netplayer_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008792_rage_scripthandlernetcomponent_determinehostfromparticipants_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008793_rage_scripthandlernetcomponent_verifylocalhost(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008794_rage_scripthandlernetcomponent_handlehostmigration(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008795_bool_rage_scripthandlermgr_sendmessagetoplayer_rage_msgscriptjoi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_008796_rage_scripthandlernetcomponent_setpendinghost_rage_netplayer_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008799_rage_scripthandlernetcomponent_getparticipantusingslot_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008801_bool_rage_scripthandlermgr_sendmessagetoplayer_rage_msgscriptjoi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008809_bool_rage_scripthandlermgr_sendmessagetoplayer_rage_msgscriptbot(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_008811_bool_rage_scripthandlermgr_sendmessagetoplayer_rage_msgscriptbot(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_008812_rage_scripthandlernetcomponent_canaddplayerbot_rage_netplayer_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008818_rage_scripthandlernetcomponent_hostmigrationfailed_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008826_rage_scripthandlernetcomponent_canstartjoining(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008828_rage_scripthandlernetcomponent_getnextfreeparticipantslot_rage_n(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008850_rage_fwconfigmanager_fwpoolsizeentryarray_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_008853_rage_fwconfig_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_008858_rage_fwconfigwithfilter_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_008868_sparseseparatedstring_char_const_ptr_fwconfignamevaluepair_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_008876_rage_gameskeleton_dtorgameskeleton(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008880_rage_atarray_rage_gameskeleton_systemdata_0_unsigned_short_grow(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_008881_rage_gameskeleton_addsysdataformode_rage_gameskeleton_mode_ptr_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_008890_rage_fwassetstore_rage_fwmetadatafile_rage_fwassetdef_rage_fwmet(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008895_rage_fwassetstore_rage_fwmetadatafile_rage_fwassetdef_rage_fwmet(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008896_rage_fwbasepool_makeempty(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008912_rage_fwbasepool_new_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008916_rage_fwmetadatastore_removeall_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008917_rage_fwtimer_enduserpause(ph: *mut Phantom); }
extern "C" { pub fn f_008918_rage_fwtimer_startuserpause_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008931_bool_ref_rage_atmap_rage_strindex_bool_rage_atmaphashfn_rage_str(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_008938_rage_fwdescpool_defragpartial(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008939_rage_fwdescpool_addgap_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_008948_rage_fwbasepool_initandallocate_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008997_rage_crdumpoutput_outputf_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_008998_rage_crdumpoutput_outputf_int_char_const_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_009005_rage_crdumpoutputstdio_privateoutputf_char_const_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009012_rage_crdumpoutputstring_privateoutputf_char_const_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009016_rage_crproperties_crproperties_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009017_rage_crproperty_crproperty_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009019_rage_crproperties_removeproperty_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009020_rage_crproperty_clone_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009022_rage_crproperties_removeallproperties(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009025_rage_crproperties_serialize_rage_datserialize_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009026_rage_crproperty_serialize_rage_datserialize_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009027_rage_crproperties_serializepropertylegacy_rage_datserialize_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009028_rage_op_rage_datserialize_ref_rage_datstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009029_rage_crproperties_updatepropertyinternal_rage_crproperty_ref_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009030_rage_crproperty_addattribute_rage_crpropertyattribute_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009031_rage_crproperty_copy_rage_crproperty_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009033_rage_crproperty_dump_rage_crdumpoutput_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009039_void_rage_datresource_place_rage_crpropertyattribute_ptr_rage_cr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009040_rage_crproperty_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009041_rage_crproperty_dtorcrproperty(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009045_rage_crpropertyattribute_initclass(ph: *mut Phantom); }
extern "C" { pub fn f_009051_rage_crpropertyattribute_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009056_rage_crpropertyattribute_dump_rage_crdumpoutput_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009180_void_rage_datresource_place_rage_crtag_ptr_rage_crtag_ptr_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009181_rage_crtag_clone_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009196_rage_crtag_dtorcrtag(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009199_rage_crbonedata_load_v100_rage_fitokenizer_ref_rage_crbonedata_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_009200_rage_crbonedata_load_v101_rage_fitokenizer_ref_rage_crbonedata_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009201_rage_crbonedata_load_v104plus_rage_fitokenizer_ref_rage_crboneda(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009202_rage_crskeletondata_dtorcrskeletondata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009204_rage_crskeletondata_place_rage_crskeletondata_ptr_rage_datresour(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009205_rage_crskeletondata_allocateandload_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009206_std_2_tree_unsigned_int_std_2_less_unsigned_int_std_2_allocator(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009207_rage_crskeletondata_findbonedata_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009215_rage_bonenameregistry_addname_unsigned_short_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009218_rage_crskeleton_crskeleton(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009219_rage_crskeleton_dtorcrskeleton(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009221_rage_crskeleton_uninit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009222_rage_crskeleton_init_rage_crskeletondata_const_ref_rage_mat34v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009224_rage_crskeleton_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009225_rage_crskeleton_partialreset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009226_rage_crskeleton_partialupdate_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009229_rage_crskeleton_transform_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009230_rage_crskeleton_inverseupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009232_rage_crskeleton_attach_bool_rage_matrix43_ptr_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009233_rage_crskeleton_getglobalmtx_unsigned_int_rage_mat34v_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009239_rage_crjointrotationlimit_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_009254_rage_crjointrotationlimit_initfromeulers_rage_vec3v_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009255_rage_crjointrotationlimit_converttoeulers_rage_vec3v_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_009256_rage_crjointrotationlimit_calcphiattheta_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_009258_rage_crjointdata_place_rage_crjointdata_ptr_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009259_rage_crjointdata_crjointdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009262_rage_crjointdata_loadbone_v100_rage_fitokenizer_ref_rage_crjoint(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_009263_rage_crjointdata_loadbone_v101_rage_fitokenizer_ref_rage_crjoint(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_009264_rage_crjointdata_loadbone_v104plus_rage_fitokenizer_ref_rage_crj(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_009265_rage_crjointdata_converttoeulers_rage_crbonedata_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_009266_rage_crjointdata_gettranslationlimits_rage_crbonedata_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_009267_rage_crjointdata_haslimits_rage_crbonedata_const_ref_bool_ref_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_009271_rage_atmapentry_unsigned_int_rage_datowner_rage_crproperty_decla(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009274_rage_grmmodel_dtorgrmmodel(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009277_rage_grcarray_rage_datowner_rage_grmgeometry_init_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009278_rage_grmmodel_packetize_rage_mshmesh_const_ref_rage_mshmaterial(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_009279_bool_rage_serializefromfile_rage_mshmesh_char_const_ptr_rage_msh(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009280_rage_mshmaterial_dtormshmaterial(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009281_rage_atrangearray_rage_datowner_rage_grcvertexbuffer_4_atrangear(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009282_rage_atrangearray_rage_datowner_rage_grcindexbuffer_4_atrangearr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009283_rage_grmmodel_getgeometry_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009284_rage_msharray_dpmdata_256_reallocate_int_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009286_rage_grmmodel_setforceshader_rage_grmshader_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009287_rage_grmmodel_computebucketmask_rage_grmshadergroup_const_ref_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009288_rage_grmmodel_optimize_rage_grmshadergroup_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009289_rage_grmmodel_draw_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009290_rage_grmmodel_drawskinned_int_rage_grmmatrixset_const_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009291_rage_grmshader_begindraw_rage_grmshader_edrawtype_bool_rage_grce(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_009292_pixbeginn_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_009293_rage_grmshader_bind_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009294_rage_grmshader_unbind_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009295_rage_grmshader_enddraw_const(ph: *mut Phantom); }
extern "C" { pub fn f_009297_rage_grmmodel_drawuntessellatedportion_rage_grmshadergroup_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_009298_rage_grmmodel_drawportion_rage_grmshader_edrawtype_int_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_009299_rage_grmshader_drawinternal_rage_grmshader_edrawtype_rage_grmmod(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_009300_rage_grmmodel_drawtessellatedportion_rage_grmshadergroup_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_009301_rage_grmmodel_drawuntessellatedportionskinned_rage_grmshadergrou(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_009302_rage_grmmodel_drawskinnedportion_rage_grmshader_edrawtype_int_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32); }
extern "C" { pub fn f_009303_rage_grmmodel_drawtessellatedportionskinned_rage_grmshadergroup(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_009304_rage_grcarray_rage_datowner_rage_grmgeometry_grcarray_rage_egrca(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009305_rage_grmgeometryqb_grmgeometryqb_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009306_rage_grmmodel_place_rage_grmmodel_ptr_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009307_rage_grmmodel_sortfortessellation_rage_grmshadergroup_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009308_rage_grmmodel_setmodelcullback_rage_grccullstatus_ptr_rage_spdaa(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009314_pixend(ph: *mut Phantom); }
extern "C" { pub fn f_009317_rage_grmshader_load_char_const_ptr_rage_fitokenizer_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_009318_stuffelementarraydynamic_rage_atrangearray_rage_grcvertexelement(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64) -> i32; }
extern "C" { pub fn f_009319_stuffelementarray_rage_atrangearray_rage_grcvertexelement_64_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64) -> i32; }
extern "C" { pub fn f_009320_rage_grmmodelfactory_buildcacheddeclarator_rage_grcfvf_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_009322_rage_grmmodelfactory_computevertexelements_rage_grcfvf_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_009323_rage_grmshadergroup_dtorgrmshadergroup(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009326_rage_grmshadergroup_add_rage_grmshader_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009328_rage_atarray_rage_grceffectvarentry_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009336_void_std_2_introsort_std_2_classicalgpolicy_rage_texentrysorter(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_009337_rage_pgdictionary_rage_grctexture_addentry_char_const_ptr_rage_g(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_009338_rage_grmshadergroup_load_rage_fitokenizer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009339_rage_grmshadergroup_grmshadergroup_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009340_void_rage_datresource_place_rage_pgdictionary_rage_grctexture_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009344_rage_grmmatrixset_update_rage_crskeleton_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009345_rage_grmmatrixset_computematrix_rage_matrix34_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009349_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_declpai(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_009352_rage_grmgeometryqb_dtorgrmgeometryqb(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009358_rage_value_ptr_rage_atmap_rage_key_rage_value_rage_keyhash_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009359_bool_rage_atmap_rage_key_rage_value_rage_keyhash_rage_atmapequal(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009360_rage_atmapentry_rage_key_rage_value_const_ref_rage_atmap_rage_ke(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009374_rage_grmgeometry_computefvf_rage_mshmaterial_const_ref_int_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_009391_rage_pftimebars_dtorpftimebars(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009393_rage_pftimebars_init_char_const_ptr_int_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_009394_rage_pftimebars_frameinit_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009395_rage_pftimebars_endtimebar(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009396_rage_pftimebars_tabbeddump_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009397_rage_pftimebars_starttimebar_char_const_ptr_char_const_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_009398_rage_pftimebarmgr_markoverbudget_rage_pftimebars_sfunctime_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_009399_rage_pftimebars_smaxfunctime_ref_rage_atmap_unsigned_long_rage_p(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009400_pixbeginc_unsigned_int_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_009402_rage_pftimebars_push_char_const_ptr_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_009403_rage_drawrect_float_float_float_float_rage_color32(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_009404_char_ptr_rage_formatf_char_80ul_char_ref_80ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009406_rage_pftimebars_adjustaverages_int_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32); }
extern "C" { pub fn f_009407_rage_pftimebars_adjustbaselines_int_int_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: f32); }
extern "C" { pub fn f_009408_rage_pftimebars_recordbaseline_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009410_rage_pftimebars_storeselectedtimebarstack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009411_rage_pftimebars_dump_int_int_float_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i64); }
extern "C" { pub fn f_009412_rage_pftimebars_dumpxml_int_int_float_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i64); }
extern "C" { pub fn f_009413_rage_pftimebars_displaybudgets_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> f32; }
extern "C" { pub fn f_009414_rage_pftimebars_verifybudgets_float_int_int_int_float(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: f32) -> f32; }
extern "C" { pub fn f_009417_pixbegin_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009433_rage_pftimebarmgr_getselectedtime_const(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_009444_pixisgpucapturing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_009447_rage_pfdrawitembase_beginmanager(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009448_rage_pfdrawitembase_end(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009454_rage_matrix34_makerotateto_rage_vector3_const_ref_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009496_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_texen(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_009497_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009498_rage_grcfvf_grcfvf_rage_grcfvf_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_009499_rage_grcfvf_setposchannel_bool_rage_grcfvf_grcdatasize_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009500_rage_grcfvf_setnormalchannel_bool_rage_grcfvf_grcdatasize(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009501_rage_grcfvf_setdiffusechannel_bool_rage_grcfvf_grcdatasize(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009502_rage_grcfvf_setspecularchannel_bool_rage_grcfvf_grcdatasize(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009503_rage_grcfvf_gettexturechannel_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009504_rage_grcfvf_gettangentchannel_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009505_rage_grcfvf_getbinormalchannel_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009506_rage_grcfvf_settexturechannel_int_bool_rage_grcfvf_grcdatasize(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009507_rage_grcfvf_settangentchannel_int_bool_rage_grcfvf_grcdatasize(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009508_rage_grcfvf_setbinormalchannel_int_bool_rage_grcfvf_grcdatasize(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009509_rage_grcfvf_getoffsetnoassert_rage_grcfvf_grcfvfchannels_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009511_rage_grceffect_dtorgrceffect(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009514_rage_grctexture_setcustomloadname_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009516_rage_grcimage_createinternal_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_009517_rage_grcimage_release_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009518_rage_grcimage_create_unsigned_int_unsigned_int_unsigned_int_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_009519_rage_grcimage_makechecker_int_rage_color32_rage_color32(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_009520_rage_grcimage_recalculatestride(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009522_rage_grctexturefactory_lookuptexturereference_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009523_rage_grctexturefactory_createtexture_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009524_rage_grctexturereference_grctexturereference_char_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_009525_rage_grcimage_loaddds_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009528_rage_grceffect_create_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009529_rage_grcstateblock_createsamplerstate_rage_grcsamplerstatedesc_c(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009530_rage_grcstateblock_grcstateblockstore_rage_grcstateblock_grcblen(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009531_rage_grctexturedx11_grctexturedx11_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009532_rage_grctexture_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009535_rage_grcrendertarget_logtexture_char_const_ptr_rage_grcrendertar(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009537_rage_grceffect_init_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009538_void_std_2_introsort_std_2_classicalgpolicy_rage_sortbyhash_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_009539_rage_grceffect_lookuptechnique_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009540_rage_grceffect_lookupvar_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009541_rage_grcstateblock_getsamplerstatedesc_rage_grcstateblock_grcsam(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009543_rage_grceffect_beginpass_int_rage_grcinstancedata_const_ref_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009544_rage_grcdrawsinglequadf_float_float_float_float_float_float_floa(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: i64); }
extern "C" { pub fn f_009545_rage_grcstateblock_setblendstate_rage_grcstateblock_grcblendstat(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009546_rage_grceffect_bind_rage_grceffecttechnique_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009547_rage_grcdevice_beginindexedvertices_rage_grcdrawmode_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_009548_rage_grcdevice_drawvertices_void_const_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_009549_rage_grceffect_endpass_const(ph: *mut Phantom); }
extern "C" { pub fn f_009553_rage_grccompositerendertargethelper_drawfullscreentri_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_009554_rage_grceffect_technique_pass_bind_rage_grcvertexprogram_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_009555_rage_grcstateblock_setrasterizerstate_rage_grcstateblock_grcrast(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009556_rage_grcbatcher_reserve_unsigned_int_unsigned_long(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_009557_rage_grcdevice_drawinstancedvertices_void_const_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_009559_rage_grcdevice_clear_bool_rage_color32_bool_float_bool_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_009560_rage_grcstateblock_setdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009561_rage_grcbegin_rage_grcdrawmode_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009562_rage_grcvertex_float_float_float_float_float_float_rage_color32(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: f32, a8: f32); }
extern "C" { pub fn f_009563_rage_grcend(ph: *mut Phantom); }
extern "C" { pub fn f_009587_rage_grclightstate_setenabled_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_009588_rage_grcimage_dtorgrcimage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009590_rage_grcimage_clear_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009591_rage_grcimage_getpixeladdr_int_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_009592_rage_grcimage_getpixelcolor32_int_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_009593_rage_grcimage_getpixelblock_rage_color32_ptr_int_int_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_009595_rage_grcimage_releasealias(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009596_rage_grcimage_setpixel_int_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009597_getddsformat_rage_ddpixelformat9_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009601_rage_grcviewport_regeneratematrices(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009603_rage_grcviewport_setworldidentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009605_rage_grcviewport_regeneratenearfarclip(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009607_rage_grcdevice_setwindow_rage_grcwindow_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009608_rage_grcviewport_grcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009609_rage_grcviewport_resetwindow(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009611_rage_grcviewport_regeneratefrustumplanes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009612_rage_grcviewport_setwindow_int_int_int_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: f32, a6: f32); }
extern "C" { pub fn f_009614_rage_grcviewport_setcurrent_rage_grcviewport_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009615_rage_grceffect_lookupglobalvar_char_const_ptr_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009630_rage_grcviewport_regenerateinstvpmatrices_rage_mat44v_const_ref(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009633_rage_grcviewport_getwindow_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009635_rage_grcviewport_setcurrentworldmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009640_rage_grcviewport_isspherevisible_float_float_float_float_float_p(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_009649_rage_grcworldidentity(ph: *mut Phantom); }
extern "C" { pub fn f_009650_rage_grcbindtexture_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009654_rage_grcfont_drawscaled_float_float_float_rage_color32_float_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: f32, a6: f32, a7: i64); }
extern "C" { pub fn f_009655_rage_grcfont_getstringwidthinternal_char_const_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009656_rage_grcfont_draw_float_float_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64); }
extern "C" { pub fn f_009657_rage_grcfont_drawf_float_float_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i64); }
extern "C" { pub fn f_009658_rage_grcfont_createfontfrombitmap_unsigned_char_const_ptr_int_in(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009662_rage_grcsetup_connecttorag_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009665_rage_grcsetup_dtorgrcsetup(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009666_rage_grcgputimestamp_releasequeries(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009668_rage_grcsetup_createdefaultfactories(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009669_rage_grctexturefactorydx11_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009670_rage_grcsetup_setpaused_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009671_rage_grcsetup_destroyfactories(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009673_rage_grcdevice_initadapterordinal(ph: *mut Phantom); }
extern "C" { pub fn f_009674_rage_grcadapterd3d11output_getclosestmode_rage_grcdisplaywindow(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009675_rage_grcstateblock_createdepthstencilstate_rage_grcdepthstencils(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009676_rage_grcdevice_blit_init(ph: *mut Phantom); }
extern "C" { pub fn f_009677_rage_fastquad_init(ph: *mut Phantom); }
extern "C" { pub fn f_009678_rage_grcdevice_prepareforshutdown(ph: *mut Phantom); }
extern "C" { pub fn f_009679_rage_grcresourcecache_grcresourcecache(ph: *mut Phantom); }
extern "C" { pub fn f_009681_rage_grcresourcecache_queueforcaching_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009682_rage_grcsetup_beginupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009688_rage_grcgputimestamp_createqueries(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009689_rage_grcdevice_isfencepending_rage_grcfencehandle_ptr_unsigned_i(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009690_rage_grcdevice_changedevice_rage_grcdisplaywindow(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009691_rage_grcstateblock_default(ph: *mut Phantom); }
extern "C" { pub fn f_009692_rage_grcdevice_resetclipplanes(ph: *mut Phantom); }
extern "C" { pub fn f_009696_rage_grcdevice_capturescreenshottojpegfile_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009698_rage_grcdraw2dtext_float_float_char_const_ptr_bool_float_float_r(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i32, a4: f32, a5: f32, a6: i64); }
extern "C" { pub fn f_009699_rage_grcdevice_capturescreenshot_rage_grcimage_ptr(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009700_rage_grcimage_savejpeg_rage_fistream_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009701_rage_copyscan_image_unsigned_char_ptr_void_ptr_int_int_int_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_009702_rage_grcimage_writepng_char_const_ptr_void_ptr_unsigned_char_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: f32) -> i32; }
extern "C" { pub fn f_009704_rage_monitorconfiguration_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009705_rage_grcsetup_addscreenshotviewerwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009715_rage_grcvertexbuffer_createwithdata_int_rage_grcfvf_const_ref_ra(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_009716_rage_grcdevice_createvertexdeclaration_rage_grcvertexelement_con(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009717_rage_grcvertexbuffer_grcvertexbuffer_int_rage_grcfvf_const_ref_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_009718_rage_grcbufferd3d11_initialise_unsigned_int_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32); }
extern "C" { pub fn f_009721_rage_grcstateblock_doflush(ph: *mut Phantom); }
extern "C" { pub fn f_009722_rage_grcdevice_setuppriortodraw_rage_grcdrawmode(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009723_rage_grcdevice_setvertexdeclaration_rage_grcvertexdeclaration_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009724_rage_grcdevice_setstreamsource_unsigned_int_rage_grcvertexbuffer(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_009725_rage_grcdevice_drawprimitive_rage_grcdrawmode_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_009728_rage_grcvertexdeclaration_release_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009729_rage_grcbatcher_dtorgrcbatcher(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009731_rage_grcbatcher_setcurrent_rage_grcbatcher_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009732_rage_grcbatcher_renderinternal(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009733_rage_grcdrawline_rage_vector3_const_ref_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009734_rage_grcdrawline_rage_vector3_const_ref_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_009735_rage_grcdraw2dquad_rage_vector3_const_ref_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_009736_rage_grcdrawbox_rage_vector3_const_ref_rage_matrix34_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009737_rage_grcdrawsolidbox_rage_vector3_const_ref_rage_matrix34_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009738_rage_grcdrawfrustum_rage_matrix34_const_ref_float_float_float_ra(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_009739_rage_grcdrawellipsoid_rage_vector3_const_ref_rage_matrix34_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_009740_rage_grcdrawpartialsphere_rage_mat34v_const_ref_float_float_int(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_009741_rage_grcdrawspiral_rage_vector3_const_ref_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_009742_rage_grcdrawcircle_float_rage_vector3_const_ref_rage_vector3_con(ph: *mut Phantom, a0: f32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_009743_rage_grcdrawarc_float_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: f32, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32); }
extern "C" { pub fn f_009744_rage_grcdrawcapsule_float_float_rage_matrix34_const_ref_int_bool(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i32, a4: i32, a5: f32); }
extern "C" { pub fn f_009745_rage_grcdrawcylinder_float_float_rage_matrix34_const_ref_int_boo(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_009746_rage_grcdrawaxis_float_rage_matrix34_const_ref_bool(ph: *mut Phantom, a0: f32, a1: i64, a2: i32); }
extern "C" { pub fn f_009747_rage_grcdrawtexturedpolygon_rage_vector3_const_ptr_int_rage_vect(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_009748_rage_grcdrawpolygon_rage_vector3_const_ptr_int_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009749_rage_grcdrawtexturedpolygon_rage_vector3_const_ptr_int_rage_vect(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_009750_rage_grcdrawpolygon_rage_vector3_const_ptr_int_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009751_rage_grcdrawlabel_rage_vector3_const_ref_int_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: f32, a6: f32); }
extern "C" { pub fn f_009754_rage_grclighting_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_009762_rage_grcdrawlabelf_rage_vector3_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009763_rage_grcvertexbufferd3d11_grcvertexbufferd3d11_rage_datresource(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009765_rage_grcvertexbuffer_create_int_rage_grcfvf_const_ref_bool_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_009767_rage_grcvertexbuffer_clone_rage_grcvertexbuffer_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009768_rage_grcvertexbuffer_makereadonly(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009769_rage_grcvertexbuffereditor_grcvertexbuffereditor_rage_grcvertexb(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_009770_rage_grcbufferd3d11_lock_internal_unsigned_int_unsigned_int_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_009771_rage_grcvertexbuffereditor_dtorgrcvertexbuffereditor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009772_rage_grcbufferd3d11_unlock_internal_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009781_rage_grcvertexbuffereditor_set_rage_grcvertexbuffer_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009782_rage_grcindexbufferd3d11_grcindexbufferd3d11_rage_datresource_re(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009785_rage_grcindexbuffer_create_int_bool_void_ptr(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_009786_rage_grcindexbuffer_clone_rage_grcindexbuffer_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009788_rage_grcindexbuffer_lockro_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009789_rage_grcindexbuffer_unlockwo_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009799_copyjpegbuffer_jpeg_decompress_struct_ref_unsigned_char_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_009802_rage_grcimage_savergba8surfacetojpeg_rage_fistream_ptr_int_void(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_009806_rage_writechunk_rage_fistream_ptr_char_const_ptr_void_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_009814_rage_grceffect_setvar_rage_grcinstancedata_ref_rage_grceffectvar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009821_rage_grcparameter_dtorgrcparameter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009823_rage_grccbuffer_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009826_std_2_tree_std_2_value_type_unsigned_int_id3d11devicechild_ptr_s(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009828_std_2_tree_std_2_value_type_unsigned_int_char_ptr_std_2_map_valu(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009830_std_2_tree_std_2_value_type_unsigned_int_rage_grccbuffer_ptr_std(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009831_rage_grccbuffer_init_internal_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009832_rage_grcprogram_load_rage_fistream_ref_char_const_ptr_bool_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_009833_rage_grcprogram_createshader_char_const_ptr_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_009834_rage_grcvertexprogram_dtorgrcvertexprogram(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009835_rage_grcprogram_dtorgrcprogram(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009848_rage_grcinstancedata_load_char_const_ptr_rage_fitokenizer_ref_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_009849_rage_grcinstancedata_clone_rage_grcinstancedata_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009850_rage_grceffect_updatetexturereferences_rage_grcinstancedata_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009851_rage_grceffect_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009852_rage_grceffect_preload_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009853_rage_grcparameter_annotation_load_rage_fistream_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009854_rage_grcparameter_load_int_rage_fistream_ref_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_009855_void_rage_grceffect_setglobalinternal_rage_grcprogram_rage_grcpa(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009856_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_009857_rage_grcstateblock_grcstateblockstore_rage_grcstateblock_grcrast(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009859_rage_grceffect_setglobalfloatcommon_int_float_ptr_int_rage_grcef(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_009860_rage_grcprogram_connectcbuffer_rage_atarray_rage_grcparameter_0(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009861_rage_grcprogram_connect_rage_atarray_rage_grcparameter_0_unsigne(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009862_rage_grceffect_variableinfo_variableinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009863_rage_grceffect_findtechniquegroupid_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009864_rage_grceffect_registertechniquegroup_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009866_rage_grcinstancedata_grcinstancedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009867_rage_grcinstancedata_dtorgrcinstancedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009868_rage_grceffect_lookupvarbyhash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009869_rage_grceffect_getlocalcommon_rage_grcinstancedata_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_009875_rage_grceffect_setlocalcommon_rage_grcinstancedata_ref_rage_grce(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_009876_rage_grceffect_setvar_rage_grcinstancedata_ref_rage_grceffectvar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009877_rage_grceffect_setvar_rage_grcinstancedata_ref_rage_grceffectvar(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_009881_rage_grceffect_setglobalvar_rage_grceffectglobalvar_rage_grctext(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009889_rage_grccomputeprogram_bind_rage_grcinstancedata_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009890_rage_grccomputeprogram_setunorderedresource_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009891_rage_grcprogram_setconstantbufferdata_rage_shadertype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009895_rage_grcinstancedata_grcinstancedata_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009906_rage_monitorconfiguration_releasemonitors(ph: *mut Phantom); }
extern "C" { pub fn f_009907_rage_monitorconfiguration_emulategrid_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009908_rage_monitorconfiguration_update(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009927_rage_grcadapterd3d11_enumerateoutputs_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009928_rage_grcadapterd3d11_dtorgrcadapterd3d11(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009939_rage_basecs_run_char_const_ptr_unsigned_short_unsigned_short_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_009947_rage_grcvertexbufferd3d11_dtorgrcvertexbufferd3d11(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009948_rage_grcbufferd3d11_cleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009951_rage_grcvertexbufferd3d11_lockinternal_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009952_rage_grcvertexbufferd3d11_unlockinternal_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009956_rage_grcindexbufferd3d11_dtorgrcindexbufferd3d11(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009959_rage_grcindexbuffer_lockrw_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009961_rage_grcvertexprogram_bind_rage_grcvertexdeclaration_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009967_rage_grccbuffer_lockdirect_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009968_rage_grccbuffer_unlockdirect(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009969_rage_grcbufferd3d11resource_cleanup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009970_rage_grcresourcecache_unloadcache(ph: *mut Phantom); }
extern "C" { pub fn f_009971_rage_grcresourcecache_clearcachequeue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009972_rage_grcresourcecache_saveresource_rage_fistream_ptr_rage_resour(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009973_rage_grcresourcecache_devicelost(ph: *mut Phantom); }
extern "C" { pub fn f_009974_rage_grcresourcecache_devicereset(ph: *mut Phantom); }
extern "C" { pub fn f_009975_rage_grcresourcecache_updatetotalavailablememory(ph: *mut Phantom); }
extern "C" { pub fn f_009977_rage_grcresourcecache_getstreamingmemory_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_009978_rage_grcresourcecache_gettotalfreememory_rage_memorytype_bool_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_009982_rage_grctexturefactorydx11_getd3d10bitsperpixel_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_009983_rage_grcresourcecache_get_rage_resourcedata_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009984_rage_grcresourcecache_add_rage_resourcedata_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009987_rage_grcdebugdraw_adddebugoutputexraw_bool_rage_color32_char_con(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_009994_rage_grcdevice_clearstreamsource_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009997_rage_grcdevice_lockcontext(ph: *mut Phantom); }
extern "C" { pub fn f_009998_rage_grcdevice_setuprendertargetvars_unsigned_int_id3d11shaderre(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_009999_rage_grcdevice_unlockcontext(ph: *mut Phantom); }
extern "C" { pub fn f_010006_rage_grcdevice_setprogramresources(ph: *mut Phantom); }
extern "C" { pub fn f_010014_rage_grcdevice_setindices_rage_grcindexbuffer_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010017_rage_grcdevice_devicelost(ph: *mut Phantom); }
extern "C" { pub fn f_010018_rage_grcdevice_devicerestored(ph: *mut Phantom); }
extern "C" { pub fn f_010029_rage_grcdevice_drawindexedprimitive_rage_grcdrawmode_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_010030_rage_grcdevice_drawindexedprimitive_rage_grcdrawmode_rage_grcver(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_010032_rage_grcdevice_drawinstancedprimitive_rage_grcdrawmode_int_int_i(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_010033_rage_grcdevice_drawinstancedindexedprimitive_rage_grcdrawmode_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_010034_rage_grctexturedx11_updatecpucopy_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010035_rage_grctexturedx11_mapstagingbuffertobackingstore_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_010038_rage_grcdevice_deletetexture_id3d11resource_ptr_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010053_bool_rage_grcresourcecache_validate_id3d11resource_ptr_id3d11res(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_010059_rage_grctexturepc_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010060_rage_grctexturedx11_init_char_const_ptr_rage_grcimage_ptr_rage_g(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_010061_rage_grctexturedx11_createinternal_rage_grctexturedx11_create_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_010062_rage_grctexturedx11_creategivendimensions_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_010064_rage_grctexturedx11_deleteresources(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010067_rage_grctexturefactorydx11_translatetotypelessformat_unsigned_in(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_010068_rage_grctexturefactorydx11_translatetotextureformat_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_010069_rage_grctexturedx11_getstride_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_010071_rage_grctexturedx11_allocextradata_rage_grcstexturesynctype(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_010076_rage_grctexturedx11_updategpucopy_internal_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_010081_int_sprintf_s_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_010090_rage_grctexturefactorydx11_getimageformatstatic_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_010091_rage_grctexturefactorydx11_translatetorageformat_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_010096_rage_grctexturefactorydx11_dtorgrctexturefactorydx11(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010109_rage_grcrendertargetdx11_destroyinternaldata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010111_rage_grctexturefactorydx11_createrendertarget_char_const_ptr_id3(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_010112_rage_grcrendertargetdx11_createfromtextureobject_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_010113_rage_grcrendertargetdx11_createtargetview_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010115_rage_grctexturefactorydx11_setupmrtstack_rage_grcrendertarget_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_010117_rage_grctexturefactorydx11_unlockmrtinternal_rage_grcresolveflag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010127_rage_grcrendertargetdx11_dtorgrcrendertargetdx11(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010146_rage_grcbufferd3d11_updategpucopy_internal_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010153_rage_batcherlock_lock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010154_rage_batcherlock_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010158_rage_grcdebugdraw_setfrozenmode_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010161_rage_grcdebugdraw_startrecording_rage_grcdebugdraw_lingerbuffer(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_010163_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010164_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010167_rage_grcdebugdraw_poly_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010172_rage_grcdebugdraw_text_float_float_rage_color32_char_const_ptr_b(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i64, a4: i32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010174_rage_grcdebugdraw_clearsubrenderbatcher2_rage_grcdebugdraw_linge(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010179_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010181_rage_grcdebugdraw_poly_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010182_rage_grcdebugdraw_poly_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010189_rage_grcdebugdraw_boxoriented_rage_vec3v_const_ref_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010194_rage_grcdebugdraw_axis_rage_mat34v_const_ref_float_bool_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_010218_rage_grcdebugdraw_printtoscreencoors_char_const_ptr_rage_grcfont(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010283_rage_pgdictionary_rage_grcinstancedata_dtorpgdictionary(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010446_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_sortb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_010447_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_010448_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_010449_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_010451_void_std_2_hash_table_std_2_hash_value_type_void_ptr_rage_resour(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010452_rage_ioinput_begin_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010453_rage_ioinput_end(ph: *mut Phantom); }
extern "C" { pub fn f_010455_rage_ioeventqueue_updatefrompipe(ph: *mut Phantom); }
extern "C" { pub fn f_010456_rage_iopad_updatedebug(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010458_rage_ioadddeadzone_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_010463_rage_inputcalibration_calibration_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_010474_rage_atmap_rage_atfinalhashstring_rage_inputcalibration_calibrat(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010486_rage_iomapper_scanpad_rage_iosource_ref_rage_iomapper_scanoption(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_010487_rage_iomapper_scanjoystick_rage_iosource_ref_rage_iomapper_scano(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_010500_rage_iovalue_isdown_float_rage_iovalue_readoptions_const_ref_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010505_rage_iovalue_wasdown_float_rage_iovalue_readoptions_const_ref_co(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_010522_rage_ioeventqueue_pop_rage_ioevent_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_010526_jpeg_createcompress_jpeg_compress_struct_ptr_int_unsigned_long(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010527_jpeg_destroy_compress_jpeg_compress_struct_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010528_jpeg_finish_compress_jpeg_compress_struct_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010529_jpeg_write_marker_jpeg_compress_struct_ptr_int_unsigned_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_010530_jpeg_start_compress_jpeg_compress_struct_ptr_unsigned_char(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010537_jpeg_write_scanlines_jpeg_compress_struct_ptr_unsigned_char_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010541_compress_output_jpeg_compress_struct_ptr_unsigned_char_ptr_ptr_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_010542_jround_up_long_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_010563_select_scan_parameters_jpeg_compress_struct_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010564_per_scan_setup_jpeg_compress_struct_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010565_jpeg_alloc_huff_table_jpeg_common_struct_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010566_jpeg_set_linear_quality_jpeg_compress_struct_ptr_int_unsigned_ch(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010567_jpeg_set_quality_jpeg_compress_struct_ptr_int_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010568_jpeg_set_defaults_jpeg_compress_struct_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010569_add_huff_table_jpeg_compress_struct_ptr_jhuff_tbl_ptr_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_010576_jpeg_createdecompress_jpeg_decompress_struct_ptr_int_unsigned_lo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_010583_start_input_pass_jpeg_decompress_struct_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010586_jpeg_read_header_jpeg_decompress_struct_ptr_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_010587_jpeg_finish_decompress_jpeg_decompress_struct_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_010588_jpeg_start_decompress_jpeg_decompress_struct_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_010589_jpeg_read_scanlines_jpeg_decompress_struct_ptr_unsigned_char_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_010591_next_marker_jpeg_decompress_struct_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_010594_get_sof_jpeg_decompress_struct_ptr_unsigned_char_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_010595_examine_app0_jpeg_decompress_struct_ptr_unsigned_char_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010602_jpeg_std_error_jpeg_error_mgr_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010608_jpeg_open_backing_store_jpeg_common_struct_ptr_backing_store_str(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010611_create_colorindex_jpeg_decompress_struct_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010617_jdiv_round_up_long_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_010618_jcopy_sample_rows_unsigned_char_ptr_ptr_int_unsigned_char_ptr_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010619_jzero_far_void_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010620_rage_mshserializer_beginvalue_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010621_rage_mshserializer_endvalue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010622_rage_mshserializer_beginblock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010623_rage_mshserializer_endblock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010624_rage_serialize_rage_mshserializer_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010625_rage_serialize_rage_mshserializer_ref_unsigned_char_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010626_rage_serialize_rage_mshserializer_ref_float_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010627_rage_serialize_rage_mshserializer_ref_rage_vector2_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010628_rage_serialize_rage_mshserializer_ref_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010629_rage_serialize_rage_mshserializer_ref_rage_vector4_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010630_rage_serialize_rage_mshserializer_ref_rage_conststring_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010631_rage_mshvertex_op_rage_mshvertex_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010632_rage_mshmaterial_op_rage_mshmaterial_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010633_rage_msharray_rage_mshprimitive_256_reallocate_int_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010634_rage_mshprimitive_op_rage_mshprimitive_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010635_rage_msharray_rage_mshchannel_256_reallocate_int_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010636_rage_msharray_rage_mshvertex_256_reallocate_int_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010637_rage_msharray_int_256_append(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010638_rage_mshmaterial_triangulate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010639_rage_msharray_rage_mshmaterial_256_reallocate_int_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010640_rage_mshmaterial_gettrianglecount_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_010641_rage_mshvertex_serialize_rage_mshserializer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010642_rage_serializenrm_rage_mshserializer_ref_rage_vector3_ref_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010643_rage_mshchannel_serialize_rage_mshserializer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010644_rage_mshmaterial_serialize_rage_mshserializer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010645_rage_msharray_rage_mshprimitive_256_reset_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010646_rage_mshmaterial_triangleiterator_getvertindices_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_010647_rage_mshmaterial_begintriangles(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010648_rage_mshmaterial_endtriangles(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010649_rage_mshmaterial_triangleiterator_op(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010650_rage_weldercell_insert_rage_vector3_const_ref_int_rage_vector3_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32) -> i32; }
extern "C" { pub fn f_010651_rage_msharray_int_8_append(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010652_rage_weldercell_dtorweldercell(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010653_rage_mshmaterial_triangleiterator_op_rage_mshmaterial_triangleit(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_010657_emit_restart_phuff_entropy_encoder_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010658_emit_bits_phuff_entropy_encoder_ptr_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_010659_emit_eobrun_phuff_entropy_encoder_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010668_rage_rmcdrawable_rmcdrawable(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010669_rage_atrangearray_rage_datowner_rage_rmclod_4_atrangearray(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010670_rage_grcarray_rage_datowner_rage_grmmodel_init_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_010671_rage_rmcdrawable_dtorrmcdrawable(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010673_rage_rmcdrawable_load_char_const_ptr_rage_rmctypefileparser_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_010674_rage_rmcdrawable_load_rage_fitokenizer_ref_rage_rmctypefileparse(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_010676_rage_atfixedarray_rage_rmctypefileparser_rmsectionloaderdata_14(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010678_rage_rmctypefileparser_registerloader_char_const_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_010684_rage_rmctypefileparser_processtypefile_rage_fitokenizer_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_010686_rage_rmcdrawable_loadmesh_rage_rmctypefilecbdata_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010687_rage_rmclodgroup_load_rage_fitokenizer_ref_rage_grmshadergroup_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010688_rage_rmclodgroup_computebucketmask_rage_grmshadergroup_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010689_rage_rmclodgroup_loadlod_rage_fitokenizer_ref_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_010696_rage_rmcdrawable_optimize(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010703_rage_rmcdrawable_drawmodelgeometriesskinned_rage_grmmodel_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64); }
extern "C" { pub fn f_010705_rage_rmcdrawable_profiledrawboundingbox_rage_matrix34_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_010707_rage_rmcdrawable_rmcdrawable_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_010708_rage_rmclodgroup_sortfortessellation_rage_grmshadergroup_const_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010709_rage_grcarray_rage_datowner_rage_grmmodel_grcarray_rage_egrcarra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_010714_rage_rmctypefileparser_rmctypefileparser(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010715_rage_atfixedarray_rage_rmctypefileparser_rmentityloaderdata_25_a(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010716_rage_rmctypefileparser_dtorrmctypefileparser(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010718_rage_rmctypefileparser_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010729_rage_mnbvhdeletenode_rage_mnbvhnode_const_ptr_int_ref_void_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_010731_rage_mnbvhbuildmultinodebvhtree_rage_spdmnbvhboundlist_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_010753_jpeg_make_c_derived_tbl_jpeg_compress_struct_ptr_unsigned_char_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_010754_jpeg_gen_optimal_table_jpeg_compress_struct_ptr_jhuff_tbl_ptr_lo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_010767_emit_dqt_jpeg_compress_struct_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_010768_emit_sof_jpeg_compress_struct_ptr_jpeg_marker(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010769_emit_dht_jpeg_compress_struct_ptr_int_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_010776_jpeg_make_d_derived_tbl_jpeg_decompress_struct_ptr_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_010777_jpeg_fill_bit_buffer_bitread_working_state_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_010778_jpeg_huff_decode_bitread_working_state_ptr_int_int_d_derived_tbl(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_010783_build_ycc_rgb_table_jpeg_decompress_struct_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010791_jpeg_calc_output_dimensions_jpeg_decompress_struct_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010820_jinit_memory_mgr_jpeg_common_struct_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010822_free_pool_jpeg_common_struct_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010828_alloc_barray_jpeg_common_struct_ptr_int_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_010829_alloc_sarray_jpeg_common_struct_ptr_int_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_010831_alloc_small_jpeg_common_struct_ptr_int_unsigned_long(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_010834_init_error_limit_jpeg_decompress_struct_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010839_update_box_jpeg_decompress_struct_ptr_box_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010840_fill_inverse_cmap_jpeg_decompress_struct_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_010841_rage_mshmesh_mshmesh_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010842_rage_mshadjunct_serialize_rage_mshserializer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010843_rage_msholdprimitive_serialize_rage_mshserializer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010844_rage_msholdmtl_serialize_rage_mshserializer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010845_rage_serialize_rage_mshserializer_ref_rage_mshskin_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010846_void_rage_serialize_rage_vector3_rage_mshserializer_ref_rage_msh(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010847_void_rage_serialize_rage_vector2_rage_mshserializer_ref_rage_msh(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010848_rage_serialize_rage_mshserializer_ref_rage_mshmesh_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010849_cstripper_createstrip_int_int_int_int_ptr_int_int_int_int_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_010850_cstripper_createindices_int_int_int_ptr_unsigned_short_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_010851_cvertcache_add_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_010852_cstriplist_removestripfromlist_cstrip_ptr_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010853_cstripper_inittriangleinfo_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_010854_rage_msharray_int_256_append3_int_const_ref_int_const_ref_int_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_010855_rage_msharray_int_256_append2_int_const_ref_int_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_010856_rage_triangulatequad_rage_mshmaterial_const_ref_rage_mshprimitiv(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64); }
extern "C" { pub fn f_010857_rage_serialize_rage_mshserializer_ref_rage_mshprimtype_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010858_rage_mshprimitive_serialize_rage_mshserializer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010859_rage_mshprimitive_gettrianglecount_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_010860_rage_bwaudio_submitbuffer_binksnd_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_010872_binkfileidle_binkio_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_010877_rage_binkragehandle_read_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_010882_rage_sysmessagequeue_rage_bwmovie_sasyncdata_32_true_push_rage_b(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010883_rage_bwmovie_dtorbwmovie(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010886_rage_bwmovie_setmoviesync_rage_bwmovie_bwmovieparams_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_010891_rage_bwmovie_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010903_rage_cutfevent_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_010921_rage_atmapentry_int_rage_functor1ret_char_const_ptr_int_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_010935_rage_cutfobjectidlistevent_dtorcutfobjectidlistevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010940_rage_atarray_rage_cutfeventargs_ptr_0_unsigned_short_resizegrow(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010941_rage_atarray_int_0_unsigned_short_resizegrow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_010942_rage_cutfeventargs_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_010945_rage_cutfattributelist_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_010949_rage_cutfnameeventargs_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_010955_rage_cutffloatvalueeventargs_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_010961_rage_cutfboolvalueeventargs_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_010973_rage_cutfobjectideventargs_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_011012_rage_cutfcameracuteventargs_cutfcameracuteventargs(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011028_rage_cutfeventargs_op_rage_cutfeventargs_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_011029_rage_cutfcutscenefile2_converttocutfattributelist_rage_parattrib(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011030_rage_cutfcutscenefile2_converttoparattributelist_rage_cutfattrib(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011031_rage_cutfeventargslist_dtorcutfeventargslist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011062_rage_cutfobjectidlisteventargs_dtorcutfobjectidlisteventargs(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011072_rage_cutfobjectvariationeventargs_dtorcutfobjectvariationeventar(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011078_rage_cutfvehicleextraeventargs_dtorcutfvehicleextraeventargs(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011083_rage_atarray_rage_cutfcameracuttimeofdaydofmodifier_0_unsigned_s(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011098_rage_cutfeventdef_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_011099_rage_cutfeventargsdef_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_011106_rage_cutfeventdefs_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011144_rage_sdrawdistancedetails_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_011163_rage_cutfcutscenesection_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_011167_rage_cutfcutscenesection_cutfcutscenesection(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011173_rage_cutfcutscenefile_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011177_rage_cutfobject_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_011186_rage_cutfcutscenefile2_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011207_rage_cutfcutscenefile2_dtorcutfcutscenefile2(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011209_rage_cutfcutscenefile2_savefile_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_011210_rage_cutfcutscenefile2_enforcesequentialobjectids(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011211_rage_cutfcutscenefile2_changeobjectid_int_int_rage_cutfevent_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_011212_rage_cutfcutscenefile2_parsexml_rage_partree_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_011213_rage_cutfcutscenefile2_loadandreplacelightsincutfile_char_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011214_rage_cutfcutscenefile2_loadsubtitlefile_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_011215_rage_cutfcutscenefile2_loadgamedatatunefile_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_011216_rage_cutfcutscenefile2_getassetpathforscene_rage_atstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_011217_rage_cutfcutscenefile2_loadandparselightdata_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_011218_rage_cutfcutscenefile2_removeeventfromlist_rage_cutfevent_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011219_rage_cutfcutscenefile2_removeobject_rage_cutfobject_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011220_rage_cutfcutscenefile2_addeventtolist_rage_cutfevent_ptr_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011221_rage_cutfcutscenefile2_exporter_addeventandargs_rage_cutfevent_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011223_rage_cutfcutscenefile2_removeobjectfromeventlist_int_rage_atarra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_011226_rage_cutfcutscenefile2_changeobjectid_int_int_rage_cutfeventargs(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_011227_rage_cutfcutscenefile2_removeobjectfromeventargs_int_rage_cutfev(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_011228_rage_cutfcutscenefile2_getcurrentsectiontimelist_rage_atarray_fl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011233_void_std_2_stable_sort_std_2_classicalgpolicy_int_ptr_ref_rage_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_011236_rage_cutfcutscenefile2_fixupclonedeventargs_rage_atarray_rage_cu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011238_rage_cutfcutscenefile2_cloneobjects_rage_cutfcutscenefile2_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_011239_rage_cutfcutscenefile2_cloneevents_rage_cutfcutscenefile2_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_011240_rage_cutfcutscenefile2_changeobjectids_rage_atmap_int_int_rage_a(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011241_rage_cutfcutscenefile2_changeobjectids_rage_atmap_int_int_rage_a(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011247_rage_cutfnamedobject_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_011248_rage_cutfnamedstreamedobject_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_011249_rage_cutfnamedanimatedstreamedobject_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_011250_rage_cutfnamedanimatedobject_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_011251_rage_cutffinalnamedobject_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_011261_rage_cutfmodelobject_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_011265_rage_cutfpedmodelobject_cutfpedmodelobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011275_rage_cutffindmodelobject_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_011282_rage_cutfblockingboundsobject_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_011288_rage_cutflightobject_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_011320_rage_cutfobject_op_rage_cutfobject_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_011323_rage_atnamespacedhashstring_rage_athashstringnamespaces_0_op_cha(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011336_rage_cutfmodelobject_op_rage_cutfobject_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_011339_rage_cutfvehiclemodelobject_dtorcutfvehiclemodelobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011340_rage_cutfobject_dtorcutfobject_18407(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011349_rage_cutfblockingboundsobject_cutfblockingboundsobject_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: f32, a5: i64) -> i64; }
extern "C" { pub fn f_011353_rage_cutflightobject_cutflightobject_int_rage_atnonfinalhashstri(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i64, a8: f32, a9: f32, a10: f32, a11: i64, a12: f32, a13: f32, a14: f32, a15: f32, a16: f32, a17: i32, a18: i32, a19: i32, a20: i32, a21: i32, a22: f32, a23: f32, a24: f32, a25: i64) -> i64; }
extern "C" { pub fn f_011372_rage_cutsanimationmanagerentity_dtorcutsanimationmanagerentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011374_rage_cutsanimationmanagerentity_requestanimdictionaryforsection(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_011375_rage_cutsanimationmanagerentity_getsectionedname_char_ptr_int_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_011376_rage_getfileindex_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011377_rage_cutsanimationmanagerentity_getnextvalidsectiontoload_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011378_rage_cutsanimationmanagerentity_getpreviousvalidsectiontoload_in(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011382_rage_cutsanimationmanagerentity_clearallentityanimations_rage_cu(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011383_rage_cutsanimationmanagerentity_getsectionedanimnames_rage_cutsm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_011385_rage_cutsanimationmanagerentity_getanimation_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_011386_rage_cutsanimationmanagerentity_getclip_char_const_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_011389_rage_cutsmanager_getmodelobjectsoftype_rage_atarray_rage_cutfobj(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_011399_rage_cutsmanager_getaudioseconds_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_011401_rage_cutsmanager_getsceneorientationmatrix_int_rage_matrix34_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_011406_rage_cutsmanager_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011409_rage_cutsmanager_fadeoutgameatbeginning_rage_cutsmanager_escreen(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011411_rage_cutsmanager_fadeoutcutsceneatend_rage_cutsmanager_escreenfa(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_011412_rage_cutsmanager_stop_rage_cutsmanager_escreenfadeoverride_rage(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011413_rage_cutsmanager_populatedrawdistancelist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011414_rage_cutsmanager_populatelightlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011415_rage_cutsmanager_populatehiddenobjectlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011416_rage_cutsmanager_populatefixupobjectlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011417_rage_cutsmanager_populateblockingboundslist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011418_rage_cutsmanager_populatepedmodelfacelist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011422_rage_cutsmanager_bankstatusdisplayunitschangedcallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011424_rage_seditcutflightinfo_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011425_rage_cutsmanager_inialiselighteditinfo_rage_seditcutflightinfo_p(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011426_rage_cutsmanager_updatelightcombo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011431_rage_cutsmanager_setface_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011432_rage_cutsmanager_play(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011433_rage_cutsmanager_fastforward(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011434_rage_cutsmanager_rewind(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011435_rage_cutsmanager_findcameraobjectid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011438_rage_cutsmanager_findscreenfadeobjectid_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011439_rage_cutsmanager_getentityobject_rage_cutfobject_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_011440_rage_cutsmanager_fadein_rage_color32_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_011442_rage_cutsmanager_updatesectioninfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011445_rage_cutsmanager_getaudioentitytime(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_011446_rage_cutsmanager_convertangledareatomatrixandvectors_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i64, a5: i64, a6: f32); }
extern "C" { pub fn f_011451_rage_cutsmanager_storeoriginalorientationdata_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011452_rage_atmapentry_unsigned_int_rage_vector4_const_ref_rage_atmap_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011458_rage_cutsmanager_isconcatsectionvalidforplayback_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011466_rage_cutsmanager_getobjectidsineventlist_rage_atarray_rage_cutfe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_011471_rage_cutsmanager_updatelighteditinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011472_rage_cutsmanager_dispatcheventtoobjecttype_int_int_rage_cutfeven(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_011476_rage_cutsmanager_deleteentityobject_rage_cutfobject_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011478_rage_cutsmanager_fadeingameatend_rage_cutsmanager_escreenfadeove(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011482_rage_cutsmanager_findaudioobjectid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011531_rage_cutsmanager_savehiddenobjects(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011532_rage_cutsmanager_bankpopulatehiddenobjecteventlist_rage_cutfobje(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011533_rage_cutsmanager_savefixupobjects(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011534_rage_cutsmanager_bankpopulatefixupobjecteventlist_rage_cutfobjec(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011535_rage_cutsmanager_saveblockingboundobjects(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011536_rage_cutsmanager_bankpopulateblockingboundobjecteventlist_rage_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011539_rage_cutsmanager_bankhiddenobjectselectedcallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011545_rage_cutsmanager_bankfixupobjectselectedcallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011550_rage_cutsmanager_bankblockingboundobjectselectedcallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011557_rage_cutsmanager_updateactivelightlistcombo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011560_rage_cutsmanager_savedrawdistances(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011562_rage_cutsmanager_dispatchsetdrawdistanceevent_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011576_rage_cutsmanager_bankoriginalscenespacetoworldspace_rage_matrix3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011577_rage_cutsmanager_bankworldspacetooriginalscenespace_rage_matrix3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011668_rage_cutffinalnameeventargs_dtorcutffinalnameeventargs(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011686_rage_cutfcameracuteventargs_dtorcutfcameracuteventargs(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011732_void_std_2_stable_sort_move_std_2_classicalgpolicy_int_ptr_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_011733_void_std_2_inplace_merge_std_2_classicalgpolicy_int_ptr_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_011738_rage_matrix34_toeulerszyx_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011746_rage_fraginst_dtorfraginst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011754_rage_fragtype_getcontrollingcomponentfromboneindex_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_011758_rage_fraginst_propagateandaccumulatebreakingimpulse_float_rage_f(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_011760_rage_fraginst_breakoffabovegroup_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_011764_rage_fraginst_deleteabovegroup_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011765_rage_fragcacheentry_adjustforlostgroups_rage_atfixedbitset_128_u(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_011766_rage_fragcacheentry_setmass_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_011767_rage_fragcacheentry_updatesavedvelocityarraysizes_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011768_rage_evtplayer_stop_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_011769_rage_fraginst_zeroskeletonmatricesbygroup_rage_crskeleton_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011770_rage_fragtype_getrootlinkconstraint_rage_matrix34_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_011771_rage_fragcacheentry_calccenterofgravity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011773_rage_fraginst_restoreabovegroup_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011779_rage_fraginst_reducegroupdamagehealth_int_float_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_011780_rage_fragcacheentry_getmass_rage_fraghierarchyinst_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_011781_rage_fragcacheentry_shatterglassonbone_int_int_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_011782_rage_fraginst_hidedeadgroup_int_rage_phcontactiterator_ptr(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011783_rage_atany_ref_rage_atany_op_float_float_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011784_rage_atany_ref_rage_atany_op_rage_fraginst_ptr_rage_fraginst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011785_rage_atany_ref_rage_atany_op_rage_phinst_ptr_rage_phinst_ptr_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011786_rage_atany_ref_rage_atany_op_int_int_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011787_rage_evtplayer_startupdatestop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011788_rage_fraginst_poseskeletonfromarticulatedbody(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011790_rage_fragcacheentry_createbreakableglassonbone_int_int_rage_atde(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_011792_rage_fraginst_openlatchabove_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011793_rage_fraginst_closelatchabove_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011794_rage_fragtype_willbearticulated_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011795_rage_fragcacheentry_setlinkattachmentrecursechildren_rage_fragty(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011801_rage_fraginst_applydamage_int_float_rage_vec3v_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32); }
extern "C" { pub fn f_011803_rage_phcollider_setcollidermatrixfrominstance(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011804_rage_fraginst_forcearticulatedcollidermode(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011805_rage_fragcacheentry_lazyarticulatedinit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011806_rage_fragcacheentry_initarticulated_rage_fraghierarchyinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011807_rage_fragcacheentry_initselfcollisions(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011810_rage_fragcacheentry_uninit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011813_rage_fragcachemanager_getnewcacheentry(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011814_rage_fragcacheentry_init_rage_fragtype_const_ptr_rage_matrix34_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_011815_rage_fragcacheentry_initinternal_unsigned_short_rage_crskeleton(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_011824_rage_fraginst_posearticulatedbodyfrombounds_signed_char_signed_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_011827_rage_geomvectors_computeleanandtwistangles_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_011829_rage_fragcacheentry_reloadphysicsdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011830_rage_fragcacheentry_initbitsets_rage_fraghierarchyinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011831_rage_fragcacheentry_initbound_rage_fraghierarchyinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011832_rage_fragcacheentry_initcomponentandboneindexmaps(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011833_rage_fragcacheentry_initphysics_rage_fraghierarchyinst_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_011834_rage_fragcacheentry_initconstrained(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011835_rage_fragcacheentry_initevents(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011836_rage_fragcacheentry_initenvcloth_rage_fraghierarchyinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011840_rage_fragtypegroup_triggerallbreakfromrootevents_int_rage_fragty(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_011841_rage_fraginst_breakeffects_rage_fragcacheentry_ptr_ptr_rage_frag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011842_rage_fragcacheentry_resetlinkattachmentrecursechildren_rage_frag(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011843_rage_fragcacheentry_addjointandlink_unsigned_char_rage_mat33v_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_011844_rage_haschildwithcloth_rage_fragtypegroup_ptr_rage_fragphysicslo(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_011845_rage_fragcacheentry_recomputelink_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011856_rage_fragmanager_dtorfragmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011867_rage_fragtypegroup_dtorfragtypegroup(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011870_rage_fragtunestruct_settype_rage_fragtype_ptr_int_rage_fraginst(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011871_rage_fragtunestruct_dampingnode_initializefromleaves(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011872_rage_fragtunestruct_refreshtypelist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011873_void_std_2_introsort_std_2_classicalgpolicy_rage_basenamepred_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_011875_rage_fragtunestruct_regeneratetuningwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011877_rage_fragtunestruct_copyfromgroup_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011878_rage_fragtunestruct_pastetogroup_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011880_rage_fragtunestruct_pastetogroupsrecursive_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011881_rage_fragtype_computemassproperties_int_rage_vec3v_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_011882_rage_fragtunestruct_dampingnode_computemaximumscalingallowed(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_011883_rage_fragtunestruct_dampingnode_setaverage_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_011885_rage_phiterator_dtorphiterator(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011886_rage_fragphysicslod_computechildangularinertias(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011887_rage_fragtunestruct_applygroupbreakpresetrecursivecallback_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011888_rage_evtparamlist_init_rage_evtparamscheme_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011889_rage_evtplayer_setset_rage_evtset_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011890_rage_fragtype_scaleundamagedchildren_int_rage_scalarv_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_011891_rage_fragphysicslod_setundamagedchildmassproperties_unsigned_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_011892_rage_fragtunestruct_addgroupwidgets_rage_bkbank_ref_int_rage_fra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_011893_char_ptr_rage_formatf_char_60ul_char_ref_60ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011908_rage_evtset_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011939_rage_fraganimation_fraganimation_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_011940_rage_fragdrawable_dtorfragdrawable(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011943_rage_atbintreebase_rage_conststring_rage_fragdrawable_locator_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011944_rage_fragdrawable_locator_constructmtx_rage_matrix34_ref_rage_ma(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011955_rage_fragdrawable_fragdrawable_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_011956_rage_atarray_rage_datowner_rage_phbound_0_unsigned_short_atarray(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011957_rage_datowner_rage_atbintreebase_rage_conststring_rage_fragdrawa(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011958_rage_fragdrawable_fragdrawable_rage_datresource_ref_rage_grmshad(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011961_rage_fragtype_fragtype_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_011962_rage_atarray_rage_datowner_rage_evtinstance_0_unsigned_short_gro(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011965_rage_evtset_op_rage_evtset_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011967_rage_fragtype_profiledrawhover_rage_fragdrawable_ptr_rage_matrix(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_011968_rage_fragtypegroup_drawphys_int_rage_matrix34_const_ref_rage_fra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_011969_rage_fragtype_hasmotionlimits_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011971_rage_fragtypechild_loadascii_rage_fiasciitokenizer_ref_rage_grms(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_011983_rage_fragtypegroup_loadascii_rage_fiasciitokenizer_ref_rage_crsk(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32); }
extern "C" { pub fn f_011984_rage_fragtypegroup_makefakechildren_int_rage_fragtypechild_ptr_p(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_011985_rage_atarray_rage_datowner_rage_fragtypeenvcloth_0_unsigned_shor(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_012009_rage_fragtypegroup_fragtypegroup_rage_datresource_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012016_void_rage_datresource_place_rage_evtinstance_ptr_rage_evtinstanc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012017_rage_fragtypechild_saveascii_rage_fiasciitokenizer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012018_rage_fragtypegroup_saveascii_rage_fiasciitokenizer_ref_rage_dato(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_012019_rage_evtset_evtset_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012020_rage_fragtypechild_fragtypechild_rage_datresource_ref_rage_grmsh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012022_rage_fragtype_dtorfragtype(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012023_rage_evtplayer_update_float_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_012024_rage_evtplayer_stopinstance_rage_evtinstance_const_ref_rage_evtu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012027_rage_fragtypechild_dtorfragtypechild(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012030_rage_fragphysicslod_fragphysicslod_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012033_rage_fragphysicslod_dtorfragphysicslod(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012035_rage_fragcacheallocator_freeblock_rage_fragcacheblock_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012039_rage_fragcacheentry_initcomponentindextolinkindextable_unsigned(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012040_rage_fragcacheentry_initarticulatedbody_rage_pharticulatedbodypa(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012041_rage_fragcacheentry_updateinstabilityprevention_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012043_rage_evtplayer_start_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_012044_rage_evtplayer_updatestartindex_rage_evtplayer_direction_float_f(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_012045_rage_atmapentry_rage_evtplayer_playerinstancepair_void_ptr_ref_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012046_rage_fragcacheallocator_dtorfragcacheallocator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012048_rage_fragcachemanager_dumpcacheentries_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012064_rage_evtinstance_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_012077_rage_evtinstance_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012100_void_std_2_introsort_std_2_classicalgpolicy_startcompare_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_012101_void_std_2_introsort_std_2_classicalgpolicy_stopcompare_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_012147_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_rage_basen(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012148_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_rage_basen(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_012149_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_basen(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_012150_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012153_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_startcompa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_012154_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012155_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_stopcompar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_012156_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_012162_rage_rainupdatecputaskdrop_rage_ptxcpuupdatebaseparams_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_012182_rage_ptxeffectinst_init_rage_ptxeffectrule_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012183_rage_ptxeffectinst_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012184_rage_ptxeffectinst_updatefinalmtx(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012185_rage_ptxeffectinst_updatesetup_float_rage_grcviewport_const_ptr(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_012186_rage_ptxeffectinst_updatefinalize_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_012187_rage_ptxevolutionlist_getevolvedkeyframepropbyid_unsigned_int_co(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012188_rage_ptxevolutionlist_query_rage_ptxkeyframeprop_ptr_rage_scalar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_012189_rage_ptxeffectinst_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012191_rage_ptxeffectinst_finish_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012192_rage_ptxmanager_deactivateeffectinst_rage_ptxeffectinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012193_rage_ptxeffectinst_calccullstate_rage_grcviewport_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_012195_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_012197_rage_atarray_rage_ptxcollisionpoly_0_unsigned_short_grow_unsigne(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012199_rage_ptxeffectinst_setcollisionset_rage_ptxcollisionset_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012200_rage_ptxeffectinst_copycollisionset_rage_ptxcollisionset_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012211_rage_ptxkeyframe_addentry_rage_scalarv_const_ref_rage_vec4v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012212_rage_ptxeffectrule_dtorptxeffectrule(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012218_rage_ptxeffectrule_getkeyframeentryinfo_int_ref_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_012219_rage_ptxeffectspawner_trigger_rage_ptxpool_rage_ptxpointsb_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_012220_rage_ptxmanager_geteffectinstfrompool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012222_rage_ptxparticlerule_returnpoint_rage_ptxlist_ptr_rage_ptxpool_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012224_rage_ptxemitterinst_gettargetdomainmtxwld_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012225_rage_ptxemitterinst_updatedomains_rage_ptxeffectinst_ptr_rage_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_012226_rage_ptxdomaininst_update_rage_ptxdomaintype_rage_ptxdomain_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_012227_rage_ptxemitterinst_updateparallel_rage_ptxeffectinst_ptr_rage_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_012228_rage_ptxemitterinst_updatefinalize_rage_ptxeffectinst_ptr_rage_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_012229_rage_ptxparticlerule_setparams_rage_ptxbehaviourparams_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_012230_rage_ptxemitterinst_initpoint_rage_ptxeffectinst_ptr_rage_ptxpoo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i64, a7: i64); }
extern "C" { pub fn f_012232_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_012233_rage_ptxemitterrule_dtorptxemitterrule(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012236_rage_ptxemitterrule_prepareevodata_rage_ptxevolutionlist_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012238_rage_ptxemitterrule_getkeyframeentryinfo_int_ref_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_012239_rage_ptxdomain_getkeyframeentryinfo_int_ref_int_ref_int_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_012240_rage_ptxparticlerule_dtorptxparticlerule(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012244_rage_ptxmsg_assetwarningf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012245_rage_ptxparticlerule_getkeyframeentryinfo_int_ref_int_ref_int_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_012268_rage_ptxkeyframe_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012269_rage_atarray_rage_ptxkeyframeentry_0_unsigned_short_grow_unsigne(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012273_rage_ptxkeyframe_copyentriesfrom_rage_ptxkeyframe_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012274_rage_ptxkeyframewidget_updateuidata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012291_rage_athashedmap_rage_grceffectglobalvar_char_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012301_rage_ptxshadertemplate_dtorptxshadertemplate(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012303_rage_ptxmanager_loadtexture_char_const_ptr_bool_bool_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_012304_rage_ptxshadervartexture_settexture_rage_grctexture_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_012314_rage_ptxshadervar_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012334_rage_ptxdomain_ptxdomain_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012335_rage_ptxdomain_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012351_rage_ptxdomaincylinder_drawcylinder_float_float_float_rage_mat34(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_012356_rage_ptxdomainattractor_drawattractor_float_float_rage_mat34v_co(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i32); }
extern "C" { pub fn f_012373_rage_ptxmanager_geteffectrule_rage_atnonfinalhashstring_rage_ptx(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_012374_rage_ptxmanager_getemitterrule_rage_atnonfinalhashstring_rage_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012375_rage_ptxmanager_getparticlerule_rage_atnonfinalhashstring_rage_p(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012609_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012610_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_012611_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012612_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_012819_rage_clothcontroller_clothcontroller(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012830_rage_phmorphcontroller_createindexmap_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_012847_rage_environmentcloth_environmentcloth_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012848_rage_environmentcloth_environmentcloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012859_rage_phmorphcontroller_phmorphcontroller(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012863_rage_environmentcloth_dtorenvironmentcloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012865_rage_environmentcloth_loadlod_rage_fitokenizer_ref_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_012866_rage_environmentcloth_setreferenceddrawable_rage_rmcdrawable_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_012908_rage_charactercloth_dtorcharactercloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012910_rage_charactercloth_charactercloth_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013020_rage_bgglassmanager_hitglass_short_int_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_013023_rage_bgdrawable_dtorbgdrawable(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013025_rage_bgglassmanager_recycleglass_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013027_rage_bgbreakable_breakabledata_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013036_rage_bgbreakable_adjustlod(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013057_rage_bgbreakable_updatenormal_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_013058_rage_bgbreakable_updatereinforced_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_013108_rage_bgglassmanager_glassinfo_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013110_rage_bgglassmanager_transferbreakableglassowner_short_ptr_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013111_rage_bgglassmanager_destroybreakableglass_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_013467_rage_netaddress_ispeerrelayaddr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013468_rage_netaddress_gettargetaddress_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013539_rage_netconnectionmanager_closeconnection_int_rage_netclosetype(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013554_rage_netconnectionmanager_getaddress_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_013562_rage_netsocketaddress_tostring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013598_rage_netpeerid_op_rage_netpeerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013602_rage_netpeerid_op_rage_netpeerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013606_rage_netconnectionmanager_establishconnection_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_013610_rage_netconnectionmanager_isopen_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013641_rage_rc4context_rc4context(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013668_rage_nethttprequest_nethttprequest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013677_rage_nethttprequest_init_rage_sysmemallocator_ptr_wolfssl_ctx_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013705_rage_nethttprequest_addformparam_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013706_rage_nethttprequest_addintformparam_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013710_rage_nethttprequest_beginlistformparam_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013711_rage_nethttprequest_addlistformparamvalue_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013712_rage_nethttprequest_addlistformparamintvalue_long(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014015_rage_nettask_destroy_rage_nettask_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014016_rage_nettask_run_rage_nettask_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014019_rage_nettask_hastask_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014020_rage_nettask_cancel_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014023_rage_nettask_nettask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014028_rage_nettask_allocate_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014069_rage_nettimesync_hassynched_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014171_rage_netpacketrecorder_dtornetpacketrecorder_23323(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014259_rage_rltaskmanager_doaddtask_unsigned_long_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014260_rage_rltaskmanager_destroytask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014285_rage_rlcloud_unwatchmemberitem_rage_rlcloudwatcher_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014288_rage_rltaskmanager_canceltask_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014292_rage_rlgettaskmanager(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_014294_rage_rlgetposixtime_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014303_rage_rlgamerhandle_op_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014304_rage_rlros_isonline_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014318_rage_rlgamerinfo_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014319_rage_rlgamerhandle_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014361_rage_rlros_getcredentials_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014362_rage_rlroscredentials_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014365_rage_rltaskmanager_findtask_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014370_rage_rlgamerhandle_tostring_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014390_rage_rlgamerinfo_isremote_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014392_rage_rlgamerinfo_getgamerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014395_rage_rlgamerinfo_op_rage_rlgamerinfo_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014459_rage_rltaskbase_configure_rage_netstatus_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014493_rage_rlugc_checktext_int_char_const_ptr_char_const_ptr_char_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014494_rage_rlsocialclub_checktext_int_char_const_ptr_rage_rlsclanguage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_014606_rage_rlroshttptask_configure_int_rage_rlhttptaskconfig_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_014607_rage_rlhttptask_addintparameter_char_const_ptr_long_long_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_014609_rage_rlstats_cancel_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014610_rage_rlleaderboard2readbyranktask_ptr_rage_rltaskmanager_createt(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014611_bool_rage_rltaskbase_configure_rage_rlleaderboard2readbyranktask(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32, a7: i64, a8: i32, a9: i64, a10: i64, a11: i64, a12: i64) -> i32; }
extern "C" { pub fn f_014612_rage_rlleaderboard2readbyranktask_configure_int_int_rage_rlleade(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64, a9: i32, a10: i64, a11: i64, a12: i64) -> i32; }
extern "C" { pub fn f_014613_rage_rlleaderboard2readbygamertask_ptr_rage_rltaskmanager_create(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014614_rage_rlleaderboard2readbygamertask_configure_int_int_rage_rllead(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i64, a7: i32, a8: i64, a9: i32, a10: i64, a11: i64, a12: i64) -> i32; }
extern "C" { pub fn f_014615_rage_rlleaderboard2readgamersbyradiustask_ptr_rage_rltaskmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014616_rage_rlleaderboard2readgamersbyradiustask_configure_int_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i64, a7: i32, a8: i64, a9: i32, a10: i64, a11: i64, a12: i64) -> i32; }
extern "C" { pub fn f_014617_rage_readplayersbyscore_int_int_rage_rlleaderboard2groupselector(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i32, a8: i64, a9: i64, a10: i64, a11: i64) -> i32; }
extern "C" { pub fn f_014681_rage_rlcloud_gettitlefile_char_const_ptr_rage_rlrossecurityflags(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64) -> i32; }
extern "C" { pub fn f_014836_rage_rlroscredentials_hasprivilege_rage_rlrosprivilegeid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014859_rage_rlros_getdefaultprivilege_rage_rlrosprivilegeid(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014947_rage_rlsocialclub_addlicenseplate_int_char_const_ptr_char_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_015046_rage_rlcloudwatcher_rlcloudwatcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015052_rage_rlcloud_watchmemberitem_int_rage_rlcloudonlineservice_char(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_015563_rage_rlleaderboard2commonreadtask_rlleaderboard2commonreadtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015570_rage_rlleaderboard2commonreadtask_configure_int_int_rage_rlleade(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i64, a7: i32, a8: i64, a9: i64, a10: i64) -> i32; }
extern "C" { pub fn f_015803_rage_phmanifold_removeallcontacts(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015817_rage_phpool_rage_phmanifold_allocate_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_015819_rage_phmanifold_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015882_rage_phconstraintmgr_insert_rage_phconstraintbase_params_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015892_rage_phconstraintmgr_removeactiveconstraints_rage_phinst_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015940_rage_phmanifold_enablecompositemanifolds(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015948_rage_phmanifold_exchangethismanifoldonly(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015952_rage_phmanifold_resetwarmstart(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015956_rage_debugcollider_rage_phcollider_ptr_char_const_ptr_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015958_rage_phcollider_initinertia(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015959_rage_phcollider_phcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015962_rage_phcollider_setinstanceandreset_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015963_rage_phcollider_setinertia_float_vector_4_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128, a2: u128); }
extern "C" { pub fn f_015965_rage_phcollider_setcollidermatrixfrominstancerigid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015966_rage_phsleep_reset_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015969_rage_phcollider_setreferenceframevelocity_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015970_rage_phcollider_setreferenceframeangularvelocity_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015988_rage_phcollider_setvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015997_rage_phsimulator_addinstbehavior_rage_phinstbehavior_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015998_rage_phsimulator_deleteobject_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_015999_rage_phsimulator_removeinstbehavior_rage_phinstbehavior_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016005_rage_phsimulator_activateobject_int_rage_phcollider_ptr_rage_phi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016012_rage_phsimulator_addactiveobject_rage_phinst_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016013_rage_phsimulator_addinactiveobject_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016014_rage_phsimulator_addfixedobject_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016015_rage_phsimulator_getobjectvelocity_rage_phinst_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016017_rage_phsimulator_deactivateobject_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016018_rage_phsimulator_activateobject_rage_phinst_ptr_rage_phcollider(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_016019_rage_phsimulator_deactivateobject_rage_phinst_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016028_rage_phsleep_phsleep_rage_phcollider_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016036_rage_pharchetype_setbound_rage_phbound_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016043_rage_pharchetype_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016049_rage_pharchetypephys_setmassonly_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016051_rage_pharchetypephys_setmassonly_rage_scalarv_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016054_rage_pharchetypephys_setanginertia_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016061_rage_pharchetypedamp_copydata_rage_pharchetype_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016069_rage_pharchetype_place_rage_pharchetype_ptr_rage_datresource_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016082_rage_phinst_phinst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016087_rage_phinst_setarchetype_rage_pharchetype_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016088_rage_phinst_setinstflag_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016091_rage_phinst_applyforce_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: f32); }
extern "C" { pub fn f_016095_rage_phinstbreakable_phinstbreakable(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016229_rage_pharchetype_dtorpharchetype_27652(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016230_rage_phinst_dtorphinst_27655(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016248_rage_pharticulatedbody_resetalljointlimitadjustments(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016257_rage_pharticulatedbody_resetdownvel_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016258_rage_pharticulatedbody_seteffectorstozeropose(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016260_rage_pharticulatedbody_freeze(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016261_rage_pharticulatedbody_rejuvenatematrices(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016262_rage_pharticulatedbody_setbodyminimumstiffness_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016263_rage_pharticulatedbody_setstiffness_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016264_rage_pharticulatedbody_resetpropagationvelocities(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016267_rage_pharticulatedbody_ensurevelocitiesfullypropagated(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016269_rage_pharticulatedbody_setdrivestate_rage_phjoint_drivestate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016273_rage_pharticulatedbody_findjointlimitdofs_int_ptr_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i64) -> i32; }
extern "C" { pub fn f_016278_rage_pharticulatedbody_dtorpharticulatedbody(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016279_rage_pharticulatedbody_addroot_rage_pharticulatedbodypart_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016282_rage_pharticulatedbody_addchild_int_rage_phjoint_ref_rage_pharti(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_016283_rage_phbound_getcomputeangularinertia_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_016284_rage_pharticulatedbody_removechild_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016287_rage_pharticulatedbody_calculateinertias(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016295_rage_pharticulatedcollider_pharticulatedcollider_int_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_016296_rage_pharticulatedcollider_preinit_int_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016297_rage_pharticulatedcollider_removefromactivelist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016301_rage_pharticulatedcollider_setcollidermatrixfrominstanceart(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016302_rage_pharticulatedcollider_setinstance_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016303_rage_pharticulatedcollider_setpartscancollide_rage_atarray_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_016304_rage_pharticulatedcollider_finalizesettingpartscancollide_rage_a(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016306_rage_pharticulatedcollider_updatesavedvelocityarraysizes_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016307_rage_pharticulatedcollider_setlinkforcomponent_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016308_rage_pharticulatedcollider_removelink_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016312_rage_phboundcomposite_updatebvh_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016315_rage_pharticulatedcollider_addtoactivearray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016317_rage_phboundcomposite_calculatecompositeextents_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016322_rage_pharticulatedcollider_setvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016323_rage_pharticulatedcollider_setangvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016341_rage_phjoint_getchildlink_rage_pharticulatedbody_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016362_rage_phjoint_phjoint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016367_rage_phbound_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016370_rage_phbound_setcgoffset_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016454_rage_phboundcomposite_phboundcomposite(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016455_rage_phboundcomposite_init_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016460_rage_phbound_load_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016464_rage_phboundcomposite_setbound_int_rage_phbound_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016465_rage_phboundcomposite_calccenterofgravity_float_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016470_rage_phboundcomposite_calccenterofbound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016471_rage_phboundcomposite_allocatetypeandincludeflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016472_rage_phboundcomposite_settypeandincludeflags_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016476_rage_phboundcomposite_cloneparts(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016521_rage_phbound_place_void_ptr_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016624_rage_phclothdata_load_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016661_rage_phwind_getlocalvelocity_rage_vec3v_const_ref_rage_vec3v_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_016738_rage_phcontactmgr_removeallcontactswithinstance_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016739_rage_phcontactmgr_removeallcontactswithinstance_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016740_rage_phcontactmgr_resetwarmstartallcontactswithinstance_rage_phi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016741_rage_phcontactmgr_updatecolliderinmanifoldswithinstance_rage_phi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016878_rage_phlevelnodetree_rage_phlooseoctreenode_getuserdata_int_cons(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_016886_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectlocation(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016892_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectlocation(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016893_rage_phlevelnodetree_rage_phlooseoctreenode_rebuildcompositebvh(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016894_rage_phlevelnodetree_rage_phlooseoctreenode_updatecompositebvh_i(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016895_rage_phlevelnodetree_rage_phlooseoctreenode_compositeboundsetbou(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016896_rage_phlevelnodetree_rage_phlooseoctreenode_compositeboundremove(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016907_rage_phlevelnodetree_rage_phlooseoctreenode_getfirstculledobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016908_rage_phlevelnodetree_rage_phlooseoctreenode_getnextculledobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016943_rage_scrthread_printstacktrace_void_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016949_rage_scrthread_getactivethread(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_016985_rage_scrthread_checkifmonitoredscriptarraysizehaschanged_char_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090710_asinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090719_isspace(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090721_bsearch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090776_fiprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090814_memcmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090826_powf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
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
extern "C" { pub fn f_090945_strrchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090947_strstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090956_strtok(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090957_strtoull(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_090959_strtoll(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_090962_system(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090965_tanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090985_vsnprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090987_vsprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090996_wcstombs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_091000_emscripten_builtin_malloc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_091002_emscripten_builtin_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091003_emscripten_builtin_calloc(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_091015_wasm_setjmp(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_091016_wasm_setjmp_test(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_091017_emscripten_longjmp(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_091023_multi3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_091056_std_2_hash_memory_void_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_091059_std_2_next_prime_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_091063_op_new_impl_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_091069_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
