//! Callee declarations for crate gen004.
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
extern "C" { pub fn f_000115_rage_sysipcpollsema_rage_sysipcsematag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000116_rage_sysipcwaitsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000118_rage_sysipcwaitsematimed_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_000119_rage_sysipcsignalsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000197_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom); }
extern "C" { pub fn f_000204_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom); }
extern "C" { pub fn f_000566_wasm_start_mode_now(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000569_startchoice(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000575_gmutex_dtorgmutex(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000579_gmutex_lock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000580_gmutex_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000591_gwaitcondition_dtorgwaitcondition(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000592_gwaitcondition_wait_gmutex_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000593_gwaitcondition_notifyall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000609_rage_voicechat_nativepackchat_int_void_ptr_unsigned_int_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000644_rage_athashnextsize_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000645_rage_athash_const_char_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000649_rage_dattypestruct_addfield_unsigned_long_unsigned_long_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000654_rage_atstring_set_char_const_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_000655_rage_atstring_lowercase(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000657_rage_atstring_endswith_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000658_rage_atstring_startswith_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000660_rage_atstring_indexof_char_const_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000661_rage_atstring_lastindexof_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_000663_rage_atstring_replace_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000664_rage_atstring_split_rage_atstring_ref_rage_atstring_ref_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000665_rage_atstring_op_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000667_rage_atstring_split_rage_atarray_rage_atstring_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_000668_rage_atarray_rage_atstring_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_000669_rage_atstring_trim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000671_rage_atstring_split_rage_atarray_rage_atstring_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000673_rage_atstring_serialize_rage_datserialize_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000675_rage_functorbase_functorbase_void_const_ptr_void_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_000680_rage_athashstringnamespacesupport_computehash_rage_athashstringn(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_000681_rage_athashstringnamespacesupport_addstring_rage_athashstringnam(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_000684_rage_athashstringnamespacesupport_trygetstring_rage_athashstring(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_000690_rage_bkwidget_bkwidget_rage_datcallback_ref_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_000692_rage_datcallback_call_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000696_rage_bkwidget_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000698_rage_bkwidget_dtorbkwidget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000703_rage_bkbank_unsetcurrentgroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000704_rage_bkbank_pushgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000705_rage_bkgroup_addgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000706_rage_bkbank_deletegroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000707_rage_bkbank_popgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000709_rage_bkwidget_getnumwidgets_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_000713_rage_bkremotepacket_sendpackets(ph: *mut Phantom); }
extern "C" { pub fn f_000714_rage_bkwidget_remotecreate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000716_rage_bkremotepacket_send(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000718_rage_bkremotepacket_isconnected(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000757_rage_bkremotepacket_addtype_int_void_ptr_rage_bkremotepacket_con(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_000758_rage_bkmanager_openfile_char_ptr_int_char_const_ptr_bool_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_000774_rage_bkmanager_sendsubchannelinfo_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000779_rage_bkmanager_createbank_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000782_rage_bkmanager_findbank_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000783_rage_bkmanager_findwidget_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000785_rage_bkmanager_destroybank_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000786_rage_bkmanager_isbankthread(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000806_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000808_rage_bkremotepacket_bkremotepacket(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000815_rage_bkmessagebox_char_const_ptr_char_const_ptr_rage_bkmessagebo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_000817_rage_bkremotewidgetmap_getwidget_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_000818_rage_bkremotewidgetmap_getid_rage_bkwidget_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000828_rage_bklist_addcolumnheader_int_char_const_ptr_rage_bklist_itemt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_000829_rage_bklist_additem_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_000830_rage_bklist_additem_int_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_000831_rage_bklist_removeitem_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000843_rage_bkcombo_setstring_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_000844_rage_bkcombo_getstring_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_000851_char_ptr_rage_formatf_char_8ul_char_ref_8ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000866_rage_bkgroup_addseparator_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000867_rage_bkgroup_addtitle_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000868_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_000869_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000870_rage_bkgroup_addbutton_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000872_rage_bkgroup_addtext_char_const_ptr_float_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000873_rage_bkgroup_addtext_char_const_ptr_int_ptr_bool_rage_datcallbac(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_000874_rage_bkgroup_addtext_char_const_ptr_int_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000876_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_000877_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_000878_rage_bkgroup_addtext_char_const_ptr_rage_atnonfinalhashstring_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_000880_rage_bkgroup_addtext_char_const_ptr_rage_atnonfinalhashstring_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000882_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000886_rage_bkgroup_addtoggle_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000892_rage_bkgroup_addtoggle_char_const_ptr_rage_atbitset_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000894_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000897_rage_bkgroup_addslider_char_const_ptr_unsigned_char_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000901_rage_bkgroup_addslider_char_const_ptr_unsigned_short_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_000906_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000907_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int_rage_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000910_rage_bkgroup_addslider_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64); }
extern "C" { pub fn f_000916_rage_bkgroup_addcombo_char_const_ptr_unsigned_char_ptr_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_000917_rage_bkgroup_addcombo_char_const_ptr_short_ptr_int_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_000918_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000919_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000920_rage_bkgroup_addcombo_char_const_ptr_unsigned_char_ptr_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000921_rage_bkgroup_addlist_char_const_ptr_bool_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000926_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000927_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000929_rage_bkgroup_addvector_char_const_ptr_rage_vector4_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000934_rage_bkgroup_addmatrix_char_const_ptr_rage_matrix34_ptr_float_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000936_rage_bkgroup_addangle_char_const_ptr_float_ptr_rage_bkangletype(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000937_rage_bkgroup_addangle_char_const_ptr_float_ptr_rage_bkangletype(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000976_rage_bktreelist_addreadonlycolumn_char_const_ptr_rage_bktreelist(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000977_rage_bktreelist_addcolumn_char_const_ptr_rage_bktreelist_columnc(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_000978_rage_bktreelist_addnode_char_const_ptr_int_rage_bktreelist_noded(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_000979_rage_bktreelist_additem_int_char_const_ptr_rage_bktreelist_itemd(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_000980_rage_bktreelist_addreadonlyitem_int_char_const_ptr_int_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_000981_rage_bktreelist_addtextboxitem_int_char_const_ptr_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_000982_rage_bktreelist_removenode_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000990_char_ptr_rage_formatf_char_512ul_char_ref_512ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001040_rage_matrix33_rotateto_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001062_rage_matrix34_inverse_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001105_rage_autoidregistry_autoidregistry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001108_rage_autoidregistry_unregister_rage_autoiddescriptor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001116_char_ptr_rage_formatf_char_32ul_char_ref_32ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001117_rage_datcallback_datcallback(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001118_rage_datcallback_datcallback_void_rage_datbase_ptr_rage_datbase(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001119_rage_datcallback_datcallback_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001122_rage_datnewline_rage_datserialize_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001175_rage_datdecompressinplace_unsigned_char_ptr_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_001190_rage_aes_aes_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001195_rage_aes_decrypt_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001514_rage_diagdisplayf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001516_rage_fistream_write_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001517_rage_diagchannel_diagchannel_rage_diagchannel_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_001523_quitf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001524_displayf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001526_rage_diagchannel_initargs(ph: *mut Phantom); }
extern "C" { pub fn f_001527_warningf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001529_rage_fistream_create_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001530_rage_fistream_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001532_rage_fistream_close(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001533_rage_fidevice_getdevice_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001534_quitf_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001535_rage_diaglogf2_rage_diagchannel_const_ref_rage_diagseverity_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001537_rage_fistream_size(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001543_rage_diagloggedprintf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001548_rage_diaggetadjustedsystemtime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_001549_rage_diaglogdefault_rage_diagchannel_const_ref_rage_diagseverity(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001553_rage_firemoteshowmessagebox_char_const_ptr_char_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001554_rage_diaglogf_rage_diagchannel_const_ref_rage_diagseverity_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001565_printf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001568_rage_atmapentry_unsigned_int_wchar_t_ptr_const_ref_rage_atmap_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001576_rage_fiassetmanager_fullpath_char_ptr_int_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_001577_rage_fistream_open_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001580_rage_fiassetmanager_open_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001581_rage_fiassetmanager_fullreadpath_char_ptr_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_001582_rage_fiassetmanager_createleadingpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001622_rage_fistream_read_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001623_rage_fiassetmanager_findextensioninpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001627_rage_fiassetmanager_exists_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001628_rage_fiassetmanager_makefolderrelative_char_ptr_int_char_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001632_rage_fiassetmanager_pushfolder_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001633_rage_fiassetmanager_popfolder(ph: *mut Phantom); }
extern "C" { pub fn f_001635_rage_fiassetmanager_filename_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001636_rage_fiassetmanager_basename_char_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001637_rage_fiassetmanager_enumfiles_char_const_ptr_void_ptr_rage_fifin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_001639_rage_fiassetmanager_removeextensionfrompath_char_ptr_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001640_rage_fiassetmanager_removenamefrompath_char_ptr_int_char_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001641_rage_fiassetmanager_insertpathelement_char_ptr_int_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001649_rage_fgets_char_ptr_int_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_001650_rage_fgetline_char_ptr_int_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_001652_rage_fidevice_makememoryfilename_char_ptr_int_void_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_001654_rage_fiserialize_fiserialize_rage_fistream_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_001676_rage_fibasetokenizer_matchtoken_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001678_rage_fibasetokenizer_checktoken_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001680_rage_fibasetokenizer_gettokentochar_char_ptr_int_char_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_001709_rage_fiasciitokenizer_put_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001736_rage_fitokenizer_fitokenizer_char_const_ptr_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002027_rage_invsqrtf_fast_float(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_002046_rage_pgrscbuilder_cleanup_rage_datresourcemap_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002080_rage_pgbase_pgbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002081_rage_pgbase_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002084_rage_pgbase_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002086_rage_pgbaserefcounted_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002088_rage_datresource_datresource_rage_datresourcemap_const_ref_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002090_rage_datresource_dtordatresource(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002091_rage_datresource_containsthisaddress_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002092_rage_datresource_getfixup_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002105_rage_pgbase_dtorpgbase_3250(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002111_rage_parmanager_parmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002112_rage_parmanager_initialize_rage_parsettings_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002128_rage_parmanager_uninitialize(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002131_rage_atnonfinalhashstring_atnonfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002132_rage_parmanager_dtorparmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002147_rage_parmanager_loadtree_char_const_ptr_char_const_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002150_rage_parmanager_savetree_char_const_ptr_char_const_ptr_rage_part(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_002154_rage_parmanager_findstructure_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002158_rage_parinstancevisitor_visittoplevelstructure_rage_parparsables(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002160_rage_parstructure_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002161_rage_parstructure_issubclassof_rage_parstructure_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002167_rage_parstructure_create(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_002175_rage_partree_dtorpartree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002182_rage_parelement_findattribute_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002184_rage_partreenode_findchildwithname_char_const_ptr_rage_partreeno(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002188_rage_partreenode_appendaschildof_rage_partreenode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002189_rage_partreenode_createstdleaf_char_const_ptr_double_bool(ph: *mut Phantom, a0: i64, a1: f64, a2: i32) -> i64; }
extern "C" { pub fn f_002193_rage_partreenode_createstdleaf_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002196_rage_partreenode_createstdleaf_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
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
extern "C" { pub fn f_002581_rage_parelement_setname_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002593_rage_parsettings_createstandardsettings(ph: *mut Phantom); }
extern "C" { pub fn f_002644_rage_atmap_rage_conststring_unsigned_int_rage_atmaphashfn_rage_c(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002695_rage_atbinarymap_rage_pardelegateholderbase_ptr_unsigned_int_dat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002696_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002697_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_002718_strupr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002719_strlwr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002720_rage_conststring_vcopy_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002721_rage_conststring_dtorconststring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002722_rage_conststring_conststring_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002739_rage_atpartialstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002743_rage_atdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002744_rage_atdatahash_unsigned_int_const_ptr_unsigned_long_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002746_rage_stristr_char_ptr_char_const_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002748_rage_stringwildcardcompare_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002766_crc32(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002776_inflatereset(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002778_inflateinit(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002779_inflate(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002781_inflateend(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002813_rage_sysnamedpipe_open_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002816_rage_sysnamedpipe_saferead_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002818_rage_sysnamedpipe_safewrite_void_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002819_rage_sysnamedpipe_close(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002822_rage_syspipepacket_write_u8_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002824_rage_syspipepacket_write_u32_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002827_rage_syspipepacket_write_const_char_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002828_rage_syspipepacket_read_u8_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002830_rage_syspipepacket_read_u32_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002832_rage_syspipepacket_read_const_char_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002839_rage_sysparam_get_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002847_rage_sysstack_defaultdisplayline_unsigned_long_char_const_ptr_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002852_rage_sysstack_printcapturedstacktrace_unsigned_long_const_ptr_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_002853_rage_sysstack_capturestacktrace_unsigned_long_ptr_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002881_unsigned_int_ref_rage_atmap_unsigned_int_unsigned_int_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002927_rage_sysmemstarttemp(ph: *mut Phantom); }
extern "C" { pub fn f_002928_rage_sysmemendtemp(ph: *mut Phantom); }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_002934_rage_sysmemvirtualallocate_unsigned_long_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002941_rage_sysmemstartminiheap_void_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002942_rage_sysmemendminiheap(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002945_rage_sysmempoolallocator_safeallocate_unsigned_long_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003076_rage_systinyheap_allocate_unsigned_long_unsigned_long_unsigned_l(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_003078_rage_systinyheap_free_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003085_rage_systaskparameters_systaskparameters(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003090_rage_systaskmanager_create_char_const_ptr_void_ptr_rage_systaskp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_003097_char_ptr_rage_formatf_char_16ul_char_ref_16ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003111_rage_sysdependencyscheduler_insertinternal_rage_sysdependency_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003112_rage_sysdependencyscheduler_outputdebug(ph: *mut Phantom); }
extern "C" { pub fn f_003141_rage_geomdistances_distancesegtopoint_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_003142_rage_geomtvalues_findtvaluesegtopoint_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_003148_rage_geomboxes_testcapsuletoalignedboxfp_float_vector_4_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: u128, a3: u128, a4: u128) -> i32; }
extern "C" { pub fn f_003149_rage_geomboxes_testsegmenttocenteredalignedbox_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_003159_rage_matrix34_fromquaternion_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003160_rage_matrix34_toquaternion_rage_quaternion_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003161_rage_quaternion_frommatrix34_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003172_rage_quaternion_twistangle_rage_vector3_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_003182_rage_atpoolbase_releasemem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003185_rage_crframe_crframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003186_rage_crframe_crframe_rage_crframedata_const_ref_unsigned_char_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_003187_rage_crframe_init_rage_crframedata_const_ref_unsigned_char_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_003189_rage_crframe_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003190_rage_crframe_exchangeframedata_rage_crframedata_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003192_rage_crframe_set_rage_crframe_const_ref_rage_crframefilter_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003193_rage_crframe_blend_float_rage_crframe_const_ref_bool_rage_crfram(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_003196_rage_mirroriterator_iteratedof_rage_crframedata_dof_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003198_rage_crframe_identityfromskel_rage_crskeletondata_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003200_rage_crframe_composite_rage_cranimation_const_ref_float_rage_crf(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003201_rage_crframeiterator_rage_compositeiterator_iterateanim_rage_cra(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003202_rage_crframe_calcmoverdeltainternal_rage_cranimation_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: i64) -> i32; }
extern "C" { pub fn f_003203_rage_crframe_setsituation_unsigned_char_unsigned_char_unsigned_s(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003204_rage_crframe_compositewithdelta_rage_cranimation_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003205_rage_crframe_invalidate_rage_crframefilter_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003206_rage_crframe_pose_rage_crskeleton_ref_bool_rage_crframefilter_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003208_rage_crframe_inversepose_rage_crskeleton_const_ref_bool_rage_crf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003209_rage_imp_quatfrommat33v_imp33_float_vector_4_float_vector_4_floa(ph: *mut Phantom, a0: u128, a1: u128, a2: u128) -> u128; }
extern "C" { pub fn f_003212_rage_crframe_getmatrix_unsigned_char_unsigned_char_unsigned_shor(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_003214_rage_crframe_getvector3_unsigned_char_unsigned_short_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_003215_rage_crframe_setvector3_unsigned_char_unsigned_short_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_003217_rage_crframe_getfloat_unsigned_char_unsigned_short_float_ref_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_003218_rage_crframe_setfloat_unsigned_char_unsigned_short_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32) -> i32; }
extern "C" { pub fn f_003219_rage_crframe_initcreateboneandmoverdofs_rage_crskeletondata_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_003221_rage_crframe_dtorcrframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003222_rage_crvcrwidget_crvcrwidget_char_const_ptr_char_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003231_rage_cranimation_cranimation(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003232_rage_cranimation_cranimation_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003233_rage_atarray_rage_datref_rage_crblockstream_0_unsigned_short_dec(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003234_rage_cranimation_dtorcranimation(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003238_rage_cranimation_compositeframe_float_rage_crframe_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003239_rage_cranimation_compositeframewithmover_float_float_rage_crfram(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003240_rage_cranimation_setname_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003241_rage_crframefilter_initclass(ph: *mut Phantom); }
extern "C" { pub fn f_003242_rage_cranimchannel_registerchanneltypeinfo_rage_cranimchannel_ch(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003243_rage_crframefilter_registertypeinfo_rage_crframefilter_typeinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003244_rage_craccelerator_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003245_rage_cranimation_finddof_unsigned_char_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_003246_rage_cranimation_findtrack_unsigned_char_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_003247_rage_cranimation_findtrackindex_unsigned_char_unsigned_short_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_003249_rage_cranimation_deletetrack_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_003251_rage_atarray_rage_datref_rage_crblockstream_0_unsigned_short_res(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003252_rage_cranimation_allocateandload_char_const_ptr_unsigned_char_pt(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003255_rage_cranimation_save_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_003256_rage_cranimation_create_unsigned_int_float_bool_bool_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_003257_rage_cranimation_createtrack_rage_cranimtrack_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_003258_rage_cranimation_gettrack_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003259_rage_cranimation_hastrack_unsigned_char_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_003260_rage_cranimation_createfromframes_rage_atarray_rage_crframe_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003261_rage_createiterator_true_iterate_rage_crframefilter_ptr_float_bo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003262_rage_cranimation_computesize_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003263_rage_cranimcache_cranimcache_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003264_rage_cranimcache_findanimblock_rage_craccelerator_lock_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_003266_rage_craccelerator_insertinternal_unsigned_long_unsigned_int_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_003272_rage_cranimtrack_cranimtrack_unsigned_char_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_003273_rage_cranimtrack_dtorcranimtrack(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003274_rage_cranimtrack_clone_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003275_rage_cranimtrack_setid_unsigned_short(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003276_rage_cranimtrack_createvector3_rage_atarray_rage_vec3v_0_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_003278_rage_cranimtrack_createquaternion_rage_atarray_rage_quatv_0_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_003280_rage_cranimtrack_createfloat_rage_atarray_float_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_003281_rage_cranimtrack_converttrackindextoname_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_003282_rage_cranimtrack_evaluatevector3_float_rage_vec3v_ref_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003283_rage_cranimtrack_evaluatequaternion_float_rage_quatv_ref_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003285_rage_cranimtrack_evaluatefloat_float_float_ref_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003286_rage_cranimplayer_cranimplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003287_rage_cranimplayer_setanimation_rage_cranimation_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003288_rage_cranimplayer_setfunctor_rage_functor0_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003291_rage_cranimtolerancesimple_cranimtolerancesimple_float_float_flo(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_003303_rage_cranimloaderbasic_cranimloaderbasic_bool_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003306_rage_cranimloadercompress_cranimloadercompress(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003307_rage_cranimloadercompress_init_rage_cranimtolerance_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003311_rage_crblockstream_evaluateframe_unsigned_char_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64); }
extern "C" { pub fn f_003312_rage_crblockstream_evaluatechannels_unsigned_char_ptr_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32, a5: i64); }
extern "C" { pub fn f_003313_rage_crframedata_crframedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003314_rage_crframedata_dtorcrframedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003315_rage_crframedata_init_unsigned_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_003316_rage_crframedata_reservedofs_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003317_rage_crframedata_sortdofs_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003318_rage_crframedata_calcoffsets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003320_rage_crframedata_calcsignature(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003322_rage_crframefilter_crframefilter_rage_crframefilter_eframefilter(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003323_rage_crframefilter_crframefilter_rage_datresource_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003341_rage_crframefilterbonebasic_crframefilterbonebasic_rage_crframef(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003342_rage_crweightset_crweightset_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003343_rage_crframefilterbonebasic_crframefilterbonebasic_rage_datresou(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003344_rage_crframefilterbonebasic_dtorcrframefilterbonebasic(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003352_rage_crweightset_setanimweight_int_float_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_003353_rage_crframefilterbonebasic_filterdof_unsigned_char_unsigned_sho(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_003355_rage_crframefilterbonebasic_calcsignature_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003367_rage_crframefiltermultiweight_crframefiltermultiweight_rage_datr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003368_rage_crframefiltermultiweight_dtorcrframefiltermultiweight(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003373_rage_crframefiltermultiweight_iscastabletotype_rage_crframefilte(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_003398_rage_crframedatainitializer_initializeframedata_rage_crframedata(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003407_rage_crframedatainitializerdofs_crframedatainitializerdofs_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_003410_rage_crframedatainitializeranimation_crframedatainitializeranima(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_003413_rage_craccelerator_getentry_unsigned_long_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_003414_rage_craccelerator_lock_lock(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003416_rage_craccelerator_lock_dtorlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003418_rage_crframeaccelerator_findframeindices_rage_craccelerator_lock(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_003420_rage_crframeaccelerator_getsizeframeanimindices_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003421_rage_crframeaccelerator_findframeanimindices_rage_craccelerator(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_003423_rage_crframeaccelerator_calcframeanimindices_rage_crframedata_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003424_rage_crframeaccelerator_findframeskelindices_rage_craccelerator(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_003429_rage_crcommonpool_allocate_unsigned_char_ptr_ptr_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_003430_rage_atmapentry_unsigned_int_rage_crframedata_ptr_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003443_rage_crweightset_crweightset_rage_crskeletondata_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003444_rage_crweightset_dtorcrweightset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003446_rage_crweightmodifierslowinslowout_crweightmodifierslowinslowout(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_003550_rage_criksolverarms_solve_rage_criksolverbase_solverhelper_ref_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003552_rage_criksolverlegs_solve_rage_criksolverbase_solverhelper_ref_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003554_rage_criksolverquadruped_goal_solve2bone_rage_criksolverbase_sol(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_003558_rage_crcreature_addcomponent_rage_crcreaturecomponent_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003560_rage_crcreature_allocatecomponent_rage_crcreaturecomponent_ecrea(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003563_rage_crcreature_getcomponentbyindex_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003567_rage_crcreaturecomponent_crcreaturecomponent_rage_crcreaturecomp(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003569_rage_crcreaturecomponent_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003607_rage_mat34v_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003620_rage_crcreaturecomponentextradofs_init_rage_crcreature_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_003736_rage_crcreaturecomponentphysical_init_rage_crcreature_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003739_rage_crmotiondescription_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003809_rage_crframedata_adddof_unsigned_char_unsigned_short_unsigned_ch(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_003828_rage_fwanimdirector_findpropertyinmovenetworks_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003830_rage_crtagiteratorbase_rage_crtagiteratorkey_true_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003831_rage_fwanimdirectorcomponentsyncedscene_isplayingsyncedscene_con(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003832_rage_fwanimdirectorcomponentsyncedscene_isvalidsceneid_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003833_rage_fwanimdirectorcomponentsyncedscene_getsyncedscenetime_short(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_003835_rage_fwanimdirector_getmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003838_rage_fwclipset_getclip_rage_fwmvtemplateid_rage_fwmvclipidtype_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_003840_rage_fwclipsetmanager_getclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003841_rage_fwclipset_getclipdictionary_bool_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003850_rage_fwanimdirector_getcreature_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003853_rage_fwanimdirector_createcomponentsyncedscene(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003855_rage_fwanimdirector_getframebuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003864_rage_fwanimdirector_initstatic_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_003866_rage_fwdictionaryloader_rage_pgdictionary_rage_crframefilter_rag(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003870_rage_fwanimdirector_shutdownstatic_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_003871_rage_fwanimdirectorcomponentsyncedscene_dumpsynchronizedscenedeb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003875_rage_fwanimhelpers_getmovertracktranslation_rage_crclip_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003876_rage_fwanimmanager_getclipifexistsbysetid_rage_fwmvtemplateid_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003878_rage_fwanimhelpers_getmovertracktranslationdiff_rage_crclip_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003880_rage_fwanimhelpers_getmovertrackrotation_rage_crclip_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003882_rage_fwanimhelpers_getmovertrackrotationdiff_rage_crclip_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003886_rage_fwanimhelpers_getmovertrackmatrixdelta_rage_crclip_const_re(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64); }
extern "C" { pub fn f_003893_rage_fwanimhelpers_applyinitialoffsetfromclip_rage_crclip_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003894_rage_fwanimhelpers_getdebugclipname_rage_crclip_const_ref_rage_a(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003895_rage_fwanimmanager_getclipifexistsbyname_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003896_rage_fwanimmanager_getclipifexistsbydictindex_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_003897_rage_fwanimmanager_getclipindict_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_003898_rage_fwanimmanager_getanimhashindict_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_003899_rage_fwanimmanager_countanimsindict_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003900_rage_fwclipsetmanager_getclipdictionaryindex_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003901_rage_fwanimmanager_getlocaltranslation_rage_crbonedata_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_003902_rage_fwanimmanager_getlocalrotation_rage_crbonedata_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_003903_rage_fwanimmanager_getobjectmatrix_rage_crskeletondata_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_003904_rage_fwanimmanager_findslot_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003905_rage_fwanimmanager_findslotfromhashkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003907_rage_fwanimmanager_isvalidslot_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003908_rage_fwanimmanager_removerefwithoutdelete_rage_strlocalindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_003909_rage_fwanimmanager_getname_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_003913_rage_fwanimmanager_getnumrefs_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003914_rage_fwanimmanager_getnumusedslots(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_003915_rage_fwanimmanager_get_rage_strlocalindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_003968_rage_fwclipset_getclipdictionaryindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003970_rage_fwclipsetmanager_getclipdictionarymetadata_rage_atnonfinalh(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003971_rage_fwclipset_getclipitemidbyindex_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_003974_rage_atarray_rage_atbinarymap_rage_fwclipitem_ptr_rage_fwmvtempl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003984_rage_fwclipsetrequesthelper_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003989_rage_fwclipsetmanager_getmemorygroupmetadata_rage_atnonfinalhash(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003990_rage_fwclipsetmanager_getmemorygroupmetadatabyindex_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_003991_rage_fwclipsetmanager_getclipsetidbyindex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003995_rage_fwclipsetmanager_getclip_rage_fwmvtemplateid_rage_fwmvclips(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004008_rage_fwclipsetmanager_streamin_deprecated_rage_fwmvtemplateid_ra(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004009_rage_fwclipsetmanager_request_deprecated_rage_fwmvtemplateid_rag(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004010_rage_fwclipsetmanager_isstreamedin_deprecated_rage_fwmvtemplatei(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004011_rage_fwclipsetmanager_addref_deprecated_rage_fwmvtemplateid_rage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004012_rage_fwclipsetmanager_release_deprecated_rage_fwmvtemplateid_rag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004015_rage_atbinarymap_rage_fwclipset_ptr_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004019_rage_fwclipsetrequesthelper_request_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_004035_rage_fwdictionaryloader_rage_crclipdictionary_rage_fwclipdiction(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004037_rage_fwclipsetrequesthelper_dtorfwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004044_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_004045_rage_fwcliprpfbuildmetadata_findclipdictionarybuildmetadata_rage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004062_rage_fwexpressionsetmanager_getexpressionset_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004082_rage_fwfacialclipsetgroupmanager_getfacialclipsetgroup_rage_fwmv(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004101_rage_fwmovenetworkinterface_clearoutputparameters_rage_atarray_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004104_rage_fwmovenetworkinterface_setsubnetworkblendduration_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_004106_rage_fwmove_dtorfwmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004108_rage_fwmove_init_rage_fwentity_ref_rage_mvnetworkdef_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004110_rage_fwmove_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004233_rage_varfixedstring_512u_varfixedstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_004238_rage_fixedstring_512u_replace_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004258_rage_fwmovedumpnetwork_dtorfwmovedumpnetwork(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004260_rage_fwmovedumpnetwork_fwmovedumpnetwork_rage_fwentity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_004265_rage_fwmovedumpdof_gettrackname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004274_rage_fwmovedumpframesequence_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004280_rage_fwposematcher_fwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004281_rage_fwposematcher_dtorfwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004282_rage_fwposematcher_addposematcher_rage_crposematcher_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004283_rage_fwposematcher_findbestmatch_rage_crskeleton_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_004294_rage_fwsyncedsceneentity_dtorfwsyncedsceneentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004309_rage_fwsyncedsceneentity_fwsyncedsceneentity_unsigned_int_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_004310_rage_atarray_rage_fwsyncedsceneentity_0_unsigned_short_pushandgr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004312_rage_fwsyncedsceneentitymanager_dtorfwsyncedsceneentitymanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004315_rage_fwsyncedsceneentitymanager_save_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_004316_rage_fwsyncedsceneentitymanager_load_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_004317_rage_fwsyncedsceneentitymanager_storesyncedsceneentity_rage_sync(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_004387_rage_fwanimdirectorcomponentsyncedscene_stopsyncedsceneplayback(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_004390_rage_fwanimdirectorcomponentsyncedscene_decsyncedsceneref_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004391_rage_fwanimdirectorcomponentsyncedscene_startsyncedsceneplayback(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: f32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_004392_rage_fwanimdirectorcomponentsyncedscene_addsyncedsceneref_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004393_rage_fwanimdirectorcomponentsyncedscene_getsyncedsceneduration_s(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004397_rage_fwanimdirectorcomponentsyncedscene_setsyncedscenephase_shor(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_004398_rage_fwanimdirectorcomponentsyncedscene_getsyncedscenephase_shor(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004400_rage_fwanimdirectorcomponentsyncedscene_updatesyncedsceneduratio(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004402_rage_fwanimdirectorcomponentsyncedscene_setsyncedsceneorigin_sho(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_004403_rage_fwanimdirectorcomponentsyncedscene_getsyncedsceneorigin_sho(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_004404_rage_fwanimdirectorcomponentsyncedscene_issyncedscenepaused_shor(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004405_rage_fwanimdirectorcomponentsyncedscene_setsyncedscenepaused_sho(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_004406_rage_fwanimdirectorcomponentsyncedscene_setsyncedscenelooped_sho(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_004411_rage_fwanimdirectorcomponentsyncedscene_attachsyncedscene_short(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_004412_rage_fwanimdirectorcomponentsyncedscene_detachsyncedscene_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004416_rage_fwanimdirectorcomponentsyncedscene_startsynchronizedscene_r(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004419_rage_fwanimdirectorcomponentsyncedscene_preparesynchronizedscene(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_004421_rage_fwanimdirectorcomponentsyncedscene_stopsynchronizedsceneaud(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004576_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_004577_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004578_void_std_2_sort_heap_abi_nqn220108_std_2_classicalgpolicy_bool_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004579_rage_atbinarymap_rage_fwclipitem_ptr_rage_fwmvtemplateid_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_004580_std_2_pair_rage_atbinarymap_rage_fwclipitem_ptr_rage_fwmvtemplat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004581_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_004603_rage_fwextensionlist_add_rage_fwextension_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004605_rage_fwextensionlist_unlink_rage_fwextension_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004620_rage_fwarchetype_getfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004631_rage_fwarchetypemanager_getarchetypename_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004632_rage_fwarchetypemanager_lookupmodelid_rage_fwarchetype_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004633_rage_fwarchetypemanager_getarchetype_rage_atnonfinalhashstring_u(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004634_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004661_rage_fwdynamicentitycomponent_fwdynamicentitycomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004681_rage_fwentity_waitforanyactiveanimupdatetocomplete_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004693_rage_fwsceneupdate_isinsceneupdate_rage_fwentity_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004697_rage_fwsceneupdate_runcustomcallbackbits_unsigned_int_void_ptr_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_004722_rage_fwwaypointrecordingroute_sidedisttounit8_float(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_004723_rage_fwwaypointrecordingroute_uint8tosidedist_unsigned_char(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004724_rage_fwwaypointrecordingroute_mbrtouint8_float(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_004725_rage_fwwaypointrecordingroute_uint8tombr_unsigned_char(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004726_rage_fwwaypointrecordingroute_hdgtouint8_float(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_004727_rage_fwwaypointrecordingroute_uint8tohdg_unsigned_char(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004728_rage_fwwaypointrecordingroute_cruisespeedtouint8_float(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_004729_rage_fwwaypointrecordingroute_uint8tocruisespeed_unsigned_char(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_004730_rage_fwwaypointrecordingroute_getclosestwaypointindex_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_004731_rage_fwwaypointrecordingroute_getwaypointdistancefromstart_int_c(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_004732_rage_fwwaypointrecordingroute_load_rage_fistream_ptr_rage_fwwayp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004737_rage_fwdecorator_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004748_rage_fwdecoratorextension_find_rage_atnonfinalhashstring_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004761_rage_fwdebugbank_createbank_char_const_ptr_rage_functor0_rage_fu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_004763_rage_fwdebugbank_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004764_rage_fwdebugbank_toggleactive(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004765_rage_fwdebugbank_removeallmainwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004774_rage_fwpickermanager_setenabled_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004783_rage_fwpickermanager_addentitytolist_rage_fwentity_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_004784_rage_fwpickermanager_addentitytopickerresults_rage_fwentity_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004786_rage_fwpickermanager_gethoverentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004787_rage_fwpickermanager_resetlist_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004792_rage_fwpickermanager_getselectedentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004793_rage_fwpickermanager_getentity_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004816_rage_fwvectormap_drawline_rage_vector3_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_004818_rage_fwvectormap_drawpoly_rage_vector3_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_004823_rage_fwvectormap_drawwedge_rage_vector3_const_ref_float_float_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_005502_rage_fwassetstore_rage_pgdictionary_rage_charactercloth_rage_fwc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005538_rage_crframefilterdictionarystore_findframefilter_rage_fwmvtempl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005539_rage_crframefilterdictionarystore_findframefilternameslow_rage_c(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005650_rage_fwpsostoreloader_loaddataintoobjectcore_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005651_rage_fwpsostoreloader_makesimplefileloadercore(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005730_rage_fwinteriorlocation_makeinvalid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005802_rage_fwsearch_dtorfwsearch(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005917_rage_fwrefawarebaseimpl_rage_datbase_dtorfwrefawarebaseimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005946_rage_aitasktree_cleartask_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_005949_rage_aitasktree_findtaskbytypeactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005950_rage_aitasktree_findtaskbytypewithpriority_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_005969_rage_cnavmesh_getpolycentroid_unsigned_int_rage_vector3_ref_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_006056_rage_strstreaminginfomanager_removeobject_rage_strindex_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006066_rage_strpackfilemanager_getimagefilefromhandle_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006075_rage_strstreaminginfo_computevirtualsize_rage_strindex_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006084_rage_strstreaminginfomanager_requestobject_rage_strindex_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006088_rage_atarray_rage_atnonfinalhashstring_0_unsigned_short_grow_uns(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006162_rage_strstreaminginfo_gethandle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006164_rage_strstreaminginfo_clearflag_rage_strindex_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006167_rage_strstreaminginfomanager_isobjectinimage_rage_strindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006175_rage_strstreaminginfomanager_clearrequiredflag_rage_strindex_uns(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006225_rage_atvarstring_atvarstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_006249_rage_strstreamingmodule_resetplacestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006361_rage_atarray_float_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006881_rage_audmetadatamanager_getobjectmetadatareffromhash_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007200_char_ptr_rage_formatf_char_255ul_char_ref_255ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007559_rage_ptfxmanager_gettriggeredinst_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007570_rage_ptfxwrapper_setcolourtint_rage_ptxeffectinst_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007759_rage_netinterface_getnumphysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007762_rage_netinterface_getallphysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007777_rage_netinterface_getdefaultlogfiletargettype(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007778_rage_netinterface_getdefaultlogfileblockingmode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007791_rage_netlogimpl_netlogimpl_char_const_ptr_rage_logopentype_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_007803_rage_networklogutils_writelogevent_rage_netlogginginterface_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007855_rage_netplayer_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007995_rage_matrix34_toeulersxyz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008007_rage_atstring_atstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_008056_rage_networklogutils_writeplayertext_rage_netlogginginterface_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_008076_rage_netobject_setlocalflag_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_008682_rage_fwbasepool_fwbasepool_int_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_008683_rage_fwconfigmanager_getsizeofpool_unsigned_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008686_rage_fwbasepool_getindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008688_rage_fwbasepool_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008689_rage_fwscriptguid_createguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008693_rage_fwscriptguid_getbasefromguid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_008840_rage_scriptinterface_isscriptregistered_rage_scriptidbase_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008843_rage_scriptinterface_isplayeraparticipant_rage_scriptidbase_cons(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008917_rage_fwtimer_enduserpause(ph: *mut Phantom); }
extern "C" { pub fn f_008918_rage_fwtimer_startuserpause_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008921_rage_fwtimer_setforcedfps_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_008922_rage_fwtimer_unsetforcedfps(ph: *mut Phantom); }
extern "C" { pub fn f_008931_bool_ref_rage_atmap_rage_strindex_bool_rage_atmaphashfn_rage_str(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_008945_rage_fwbasepool_forall_bool_ptr_void_ptr_void_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008949_rage_fwbasepool_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008952_rage_fwbasepool_getat_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008953_rage_fwknownrefholder_fwknownrefholder_void_ptr_ptr_rage_fwknown(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_008997_rage_crdumpoutput_outputf_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_008998_rage_crdumpoutput_outputf_int_char_const_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_008999_rage_crdumpoutput_outputf_int_char_const_ptr_int_int_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_009003_rage_crdumpoutputstdio_crdumpoutputstdio(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009015_rage_crproperties_crproperties(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009018_rage_crproperties_copyproperties_rage_crproperties_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009019_rage_crproperties_removeproperty_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009021_rage_crproperties_dtorcrproperties(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009022_rage_crproperties_removeallproperties(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009023_rage_crproperties_place_rage_crproperties_ptr_rage_datresource_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009024_rage_crproperties_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009025_rage_crproperties_serialize_rage_datserialize_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009027_rage_crproperties_serializepropertylegacy_rage_datserialize_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009028_rage_op_rage_datserialize_ref_rage_datstring_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009030_rage_crproperty_addattribute_rage_crpropertyattribute_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009031_rage_crproperty_copy_rage_crproperty_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009032_rage_crproperties_dump_rage_crdumpoutput_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009034_rage_crproperty_setname_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009036_rage_crproperty_calcsignature(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009037_rage_crproperties_addproperty_rage_crproperty_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009038_rage_crproperty_crproperty(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009041_rage_crproperty_dtorcrproperty(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009043_rage_crpropertyattribute_allocate_rage_crpropertyattribute_etype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_009044_rage_crproperty_initclass(ph: *mut Phantom); }
extern "C" { pub fn f_009045_rage_crpropertyattribute_initclass(ph: *mut Phantom); }
extern "C" { pub fn f_009046_rage_crproperty_shutdownclass(ph: *mut Phantom); }
extern "C" { pub fn f_009047_rage_crproperty_stringkey_setstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009048_rage_crproperty_getattribute_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009049_rage_crproperty_forallattributes_bool_ptr_rage_crpropertyattribu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009050_rage_crproperty_stringkey_calckey_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009052_rage_crpropertyattribute_findtypeinfo_rage_crpropertyattribute_e(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_009053_rage_crpropertyattribute_dtorcrpropertyattribute(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009059_rage_crpropertyattributefloat_crpropertyattributefloat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009078_rage_crpropertyattributebool_crpropertyattributebool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009088_rage_crpropertyattributestring_crpropertyattributestring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009090_rage_crpropertyattributestring_dtorcrpropertyattributestring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009112_rage_crpropertyattributevector3_crpropertyattributevector3(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009131_rage_crpropertyattributequaternion_crpropertyattributequaternion(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009167_rage_crpropertyattributehashstring_crpropertyattributehashstring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009169_rage_crpropertyattributehashstring_dtorcrpropertyattributehashst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009178_rage_crtags_crtags(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009179_rage_crtags_copytags_rage_crtags_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009182_rage_crtags_dtorcrtags(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009183_rage_crtags_place_rage_crtags_ptr_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009184_rage_crtags_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009185_rage_crtags_removealltags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009186_rage_crtags_serialize_rage_datserialize_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009188_rage_crtags_dump_rage_crdumpoutput_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009189_rage_crtags_addtag_rage_crtag_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009190_rage_crtags_sorttags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009191_rage_crtags_removetag_rage_crtag_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009192_rage_crtags_removealltags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009193_rage_crtag_setstart_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_009194_rage_crtag_setend_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_009195_rage_crtag_crtag(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009196_rage_crtag_dtorcrtag(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009198_rage_crtag_setstartend_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_009205_rage_crskeletondata_allocateandload_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009207_rage_crskeletondata_findbonedata_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009208_rage_crskeletondata_getnumanimatabledofs_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009215_rage_bonenameregistry_addname_unsigned_short_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009217_rage_crskeletondata_debugconvertboneidtoname_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_009218_rage_crskeleton_crskeleton(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009219_rage_crskeleton_dtorcrskeleton(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009220_rage_crskeleton_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009222_rage_crskeleton_init_rage_crskeletondata_const_ref_rage_mat34v_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009223_rage_crskeleton_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009224_rage_crskeleton_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009228_rage_crskeleton_copyfrom_rage_crskeleton_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009233_rage_crskeleton_getglobalmtx_unsigned_int_rage_mat34v_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009257_rage_crjointrotationlimit_applylimits_rage_quatv_ref_rage_quatv(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009259_rage_crjointdata_crjointdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009260_rage_crjointdata_dtorcrjointdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009314_pixend(ph: *mut Phantom); }
extern "C" { pub fn f_009417_pixbegin_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009445_rage_pfdrawarrow_rage_vector3_const_ref_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009447_rage_pfdrawitembase_beginmanager(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009448_rage_pfdrawitembase_end(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009504_rage_grcfvf_gettangentchannel_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009545_rage_grcstateblock_setblendstate_rage_grcstateblock_grcblendstat(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009555_rage_grcstateblock_setrasterizerstate_rage_grcstateblock_grcrast(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009560_rage_grcstateblock_setdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009561_rage_grcbegin_rage_grcdrawmode_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009562_rage_grcvertex_float_float_float_float_float_float_rage_color32(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: f32, a8: f32); }
extern "C" { pub fn f_009563_rage_grcend(ph: *mut Phantom); }
extern "C" { pub fn f_009608_rage_grcviewport_grcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009613_rage_grcviewport_dtorgrcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009614_rage_grcviewport_setcurrent_rage_grcviewport_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009644_rage_grcviewport_screen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009649_rage_grcworldidentity(ph: *mut Phantom); }
extern "C" { pub fn f_009650_rage_grcbindtexture_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009655_rage_grcfont_getstringwidthinternal_char_const_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009698_rage_grcdraw2dtext_float_float_char_const_ptr_bool_float_float_r(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i32, a4: f32, a5: f32, a6: i64); }
extern "C" { pub fn f_009733_rage_grcdrawline_rage_vector3_const_ref_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009734_rage_grcdrawline_rage_vector3_const_ref_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_009736_rage_grcdrawbox_rage_vector3_const_ref_rage_matrix34_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009737_rage_grcdrawsolidbox_rage_vector3_const_ref_rage_matrix34_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009742_rage_grcdrawcircle_float_rage_vector3_const_ref_rage_vector3_con(ph: *mut Phantom, a0: f32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_009744_rage_grcdrawcapsule_float_float_rage_matrix34_const_ref_int_bool(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i32, a4: i32, a5: f32); }
extern "C" { pub fn f_009745_rage_grcdrawcylinder_float_float_rage_matrix34_const_ref_int_boo(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_009750_rage_grcdrawpolygon_rage_vector3_const_ptr_int_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009752_rage_grcworldmtx_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009753_rage_grcworldmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009754_rage_grclighting_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_009756_rage_grcdrawbox_rage_vector3_const_ref_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009758_rage_grcdrawsphere_float_rage_vector3_const_ref_int_bool_bool(ph: *mut Phantom, a0: f32, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_009759_rage_grcdrawsphere_float_rage_matrix34_const_ref_int_bool_bool(ph: *mut Phantom, a0: f32, a1: i64, a2: i32); }
extern "C" { pub fn f_009761_rage_grcdrawlabel_rage_vector3_const_ref_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009762_rage_grcdrawlabelf_rage_vector3_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009769_rage_grcvertexbuffereditor_grcvertexbuffereditor_rage_grcvertexb(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_009771_rage_grcvertexbuffereditor_dtorgrcvertexbuffereditor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009788_rage_grcindexbuffer_lockro_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009789_rage_grcindexbuffer_unlockwo_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010163_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010164_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010166_rage_grcdebugdraw_quad_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010171_rage_grcdebugdraw_meter_rage_vec2v_const_ref_rage_vec2v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_010172_rage_grcdebugdraw_text_float_float_rage_color32_char_const_ptr_b(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i64, a4: i32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010173_rage_grcdebugdraw_metervalue_rage_vec2v_const_ref_rage_vec2v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010179_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010180_rage_grcdebugdraw_quad_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_010181_rage_grcdebugdraw_poly_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010183_rage_grcdebugdraw_cross_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010187_rage_grcdebugdraw_partialsphere_rage_vec3v_const_ref_float_rage(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_010188_rage_grcdebugdraw_boxaxisaligned_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010189_rage_grcdebugdraw_boxoriented_rage_vec3v_const_ref_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010193_rage_grcdebugdraw_capsule_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010194_rage_grcdebugdraw_axis_rage_mat34v_const_ref_float_bool_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_010195_rage_grcdebugdraw_arrow_rage_vec3v_const_ref_rage_vec3v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_010214_rage_grcdebugdraw_textfontpush_rage_grcfont_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010215_rage_grcdebugdraw_textfontpop(ph: *mut Phantom); }
extern "C" { pub fn f_010216_rage_grcdebugdraw_text_float_float_rage_dd_epixelcoordspace_rage(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i32, a4: i64, a5: i32, a6: f32, a7: f32); }
extern "C" { pub fn f_010217_rage_grcdebugdraw_printtoscreencoors_char_const_ptr_int_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010220_rage_grcdebugdraw_adddebugoutput_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010224_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010225_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010737_rage_spdaabb_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_010940_rage_atarray_rage_cutfeventargs_ptr_0_unsigned_short_resizegrow(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011093_rage_cutfcascadeshadoweventargs_getposition_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011402_rage_cutsmanager_getsceneorientationmatrix_rage_matrix34_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_011752_rage_fragtype_getcomponentfromboneindex_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_011787_rage_evtplayer_startupdatestop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011805_rage_fragcacheentry_lazyarticulatedinit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011885_rage_phiterator_dtorphiterator(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011889_rage_evtplayer_setset_rage_evtset_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012021_rage_fragphysicslodgroup_getlodbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012028_rage_evtplayer_dtorevtplayer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012058_rage_evtplayer_evtplayer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012078_rage_evtparamlist_getparam_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012183_rage_ptxeffectinst_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012196_rage_ptxeffectinst_setbasemtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012269_rage_atarray_rage_ptxkeyframeentry_0_unsigned_short_grow_unsigne(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012434_rage_atmapentry_char_const_ptr_rage_ptxbehaviour_ptr_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012912_rage_pack_core32_float_vector_4_ref_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128) -> u128; }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014662_rage_rlmetric_dtorrlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014685_rage_rlmetric_rlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014725_rage_atmapentry_unsigned_int_rage_atstring_const_ref_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_014807_rage_atdelegate1_void_rage_rlprofilestatsevent_const_ptr_invoke(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015754_rage_phmaterialmgr_getdebugcolor_rage_phmaterial_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015774_rage_phmathinertia_getinverseinertiamatrix_rage_mat33v_const_ref(ph: *mut Phantom, a0: i64, a1: u128, a2: i64); }
extern "C" { pub fn f_015775_rage_phmathinertia_findcylinderanginertia_float_float_float_rage(ph: *mut Phantom, a0: f32, a1: f32, a2: i64); }
extern "C" { pub fn f_015777_rage_phworkerthread_dtorphworkerthread(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015778_rage_phworkerthread_initiate_rage_systaskparameters_const_ref_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_015779_rage_phworkerthread_wait(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015803_rage_phmanifold_removeallcontacts(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015804_rage_phoverlappingpairarray_phoverlappingpairarray_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_015806_rage_phoverlappingpairarray_computeislands_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_015812_rage_phconstraintbase_getinstancea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015836_rage_phmanifold_refreshcontactpoints_int_rage_scalarv_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_015890_rage_phconstraintmgr_remove_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015891_rage_phconstraintmgr_gettemporarypointer_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015893_rage_phmultireaderlocktoken_waitasreader(ph: *mut Phantom); }
extern "C" { pub fn f_015917_rage_phconstraintattachment_setlocalposb_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015945_rage_updatecontactstask_rage_systaskparameters_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015946_rage_phmanifold_exchange(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015949_rage_phmanifold_resetgjkcache(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015950_rage_phmanifold_objectsincontact_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015952_rage_phmanifold_resetwarmstart(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015955_rage_phbound_getmaterialidfrompartindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_015960_rage_phcollider_dtorphcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015980_rage_phcollider_getlocalvelocityrigid_float_vector_4_const(ph: *mut Phantom, a0: i64, a1: i64, a2: u128); }
extern "C" { pub fn f_015981_rage_phcollider_applyimpulserigid_float_vector_4_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128, a2: u128); }
extern "C" { pub fn f_015998_rage_phsimulator_deleteobject_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_015999_rage_phsimulator_removeinstbehavior_rage_phinstbehavior_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016007_rage_phsimulator_applyimpetus_float_vector_4_int_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: u128, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: f32) -> i32; }
extern "C" { pub fn f_016018_rage_phsimulator_activateobject_rage_phinst_ptr_rage_phcollider(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_016043_rage_pharchetype_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016080_rage_phcullshape_checksphere_rage_vec3v_const_ref_rage_scalarv_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_016087_rage_phinst_setarchetype_rage_pharchetype_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016088_rage_phinst_setinstflag_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016104_rage_phiterator_checkinstance_rage_phinst_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_016130_rage_phshapetest_rage_phshapeprobe_phshapetest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016138_rage_phshapetest_rage_phshapeprobe_testinlevel_rage_phinst_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_016146_rage_phshapetest_rage_phshapesphere_testbound_rage_phshapesphere(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_016149_rage_phshapetest_rage_phshapesphere_testinlevel_rage_phinst_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_016172_rage_phshapetest_rage_phshapesweptsphere_phshapetest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016174_rage_phshapetest_rage_phshapesweptsphere_testinlevel_rage_phinst(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_016194_rage_phshapetest_rage_phshapesphere_initsphere_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_016195_rage_phshapetest_rage_phshapeprobe_initprobe_rage_phsegment_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_016197_rage_phshapetest_rage_phshapesweptsphere_initsweptsphere_rage_ph(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_016230_rage_phinst_dtorphinst_27655(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016249_rage_pharticulatedbody_getlocalvelocitynoprop_int_float_vector_4(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: u128); }
extern "C" { pub fn f_016250_rage_pharticulatedbody_getlocalvelocity_int_float_vector_4(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: u128); }
extern "C" { pub fn f_016251_rage_pharticulatedbody_propagateincrementaltolink_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_016252_rage_pharticulatedbody_propagateonceup_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016255_rage_pharticulatedbody_getangularvelocity_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016270_rage_pharticulatedbody_applyincrementaljointlimitimpulse_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_016271_rage_pharticulatedbody_applyincrementalimpulse_int_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_016281_rage_atarray_rage_vec4v_0_unsigned_short_resizegrow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016291_rage_pharticulatedbody_restorevelocities_rage_vec3v_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016311_rage_phboundcomposite_calculateboundbox(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016312_rage_phboundcomposite_updatebvh_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016317_rage_phboundcomposite_calculatecompositeextents_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016324_rage_pharticulatedcollider_applyimpulseart_float_vector_4_float(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: i32); }
extern "C" { pub fn f_016329_rage_pharticulatedcollider_getinversemassmatrix_rage_mat33v_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: u128, a3: i32); }
extern "C" { pub fn f_016330_rage_pharticulatedcollider_getinversemassmatrixself_rage_mat33v(ph: *mut Phantom, a0: i64, a1: i64, a2: u128, a3: u128, a4: i32, a5: i32); }
extern "C" { pub fn f_016332_rage_pharticulatedcollider_getinverseinertiamatrixart_rage_mat33(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016333_rage_pharticulatedcollider_getlocalvelocityart_float_vector_4_in(ph: *mut Phantom, a0: i64, a1: i64, a2: u128, a3: i32); }
extern "C" { pub fn f_016344_rage_phjoint3dof_getjointlimitunitimpulse_int_rage_phphasespacev(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016354_rage_phjoint1dof_getjointlimitunitimpulse_int_rage_phphasespacev(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016360_rage_phprismaticjoint_getjointlimitunitimpulse_int_rage_phphases(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016367_rage_phbound_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016369_rage_phbound_localgetsupportingvertexwithoutmarginnotinlined_flo(ph: *mut Phantom, a0: i64, a1: u128, a2: i64); }
extern "C" { pub fn f_016372_rage_phbound_getmaterialidfromfile_rage_fiasciitokenizer_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016376_void_rage_datresource_place_rage_phoptimizedbvh_ptr_rage_phoptim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016388_rage_phbound_virtualconstructfromptr_rage_datresource_ref_rage_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016390_rage_phbound_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016403_rage_phboundgeometry_dtorphboundgeometry(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016408_rage_phoptimizedbvh_draw_rage_mat34v_const_ref_int_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_016409_rage_phpolygon_isthin_rage_phboundpolyhedron_const_ptr_rage_scal(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_016410_rage_phpolygon_hasbadnormal_rage_phboundpolyhedron_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_016411_rage_phpolygon_hasbadneighbors_rage_phboundpolyhedron_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_016423_rage_phboundpolyhedron_octantmapallocateandcopy_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016428_rage_phoptimizedbvh_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016431_rage_phboundcapsule_setcapsulesize_rage_scalarv_const_ref_rage_s(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016436_rage_phboundcapsule_phboundcapsule(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016439_rage_phboundcapsule_setcapsulesize_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_016454_rage_phboundcomposite_phboundcomposite(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016455_rage_phboundcomposite_init_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016464_rage_phboundcomposite_setbound_int_rage_phbound_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016513_void_std_2_introsort_std_2_classicalgpolicy_rage_vertexsortpredi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_016514_void_std_2_introsort_std_2_classicalgpolicy_rage_polygonsortpred(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_016518_rage_phboundgeometry_changebvhextentsfromembeddedprim_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_016520_rage_phboundgeometry_findelementindex_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_016521_rage_phbound_place_void_ptr_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016526_rage_shrinkboundingsphererecurse_rage_vec3v_ref_rage_scalarv_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_016531_rage_phboundsphere_setsphereradius_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016537_void_std_2_introsort_std_2_classicalgpolicy_rage_compareleafnode(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_016540_rage_pharticulatedcollider_dtorpharticulatedcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016548_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_rage_lexic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016549_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_lexic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016551_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_verte(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_016552_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_016553_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_rage_polyg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016554_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_polyg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016555_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016556_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_rage_compa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016557_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_rage_compa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016558_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_compa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_016559_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_016560_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016562_rage_getclosestpointsspecial_capsulecapsule_discretecollisiondet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016563_rage_getclosestpointsspecial_capsuletriangle_discretecollisionde(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016564_rage_getclosestpointsspecial_disctriangle_discretecollisiondetec(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016565_rage_gjkpairdetector_dogjkiterations_discretecollisiondetectorin(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_016566_rage_phconvexintersector_gjkpenetrationsolve_discretecollisionde(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016567_rage_phconvexintersector_writesimpleresult_discretecollisiondete(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016568_rage_voronoisimplexsolver_updateclosestvectorandpoints_rage_scal(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_016569_trianglepenetrationdepthsolver_computepenetrationseparation_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_016571_rage_seedsimplex_rage_phbound_const_ptr_rage_vec3v_ptr_int_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_016572_rage_geomdistances_findclosestsegdisctotri_rage_vec3v_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_016576_rage_voronoisinglesimplexsolver_computeclosestpointandnormalontr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_016577_rage_voronoisinglesimplexsolver_computeclosestpointandnormalonte(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_016579_rage_voronoidegeneratesimplexsolver_reducevertices_rage_vecboolv(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016580_rage_voronoidegeneratesimplexsolver_closestptpointtriangle_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016581_rage_phexplosionmgr_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016588_rage_phverletcloth_phverletcloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016589_rage_drawedges_rage_vector3_const_ref_rage_phverletcloth_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: f32, a8: f32, a9: i64, a10: i32, a11: i32, a12: i32); }
extern "C" { pub fn f_016592_rage_phverletcloth_dtorphverletcloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016593_rage_phclothdata_dtorphclothdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016604_rage_spaceedges_int_rage_phedgedata_ptr_int_ptr_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_016631_rage_phclothverletbehavior_updateropebound_rage_phverletcloth_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016633_rage_phmouseinput_shutdownboxgun(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016637_rage_drawprimitive_rage_phboundpolyhedron_ptr_int_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: f32); }
extern "C" { pub fn f_016638_rage_renderbvhleafnode_rage_phboundpolyhedron_ptr_int_rage_matri(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_016640_rage_renderbvhsubtree_rage_phboundpolyhedron_ptr_int_rage_matrix(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_016641_rage_renderbvhsubtree_rage_phboundpolyhedron_ptr_int_rage_matrix(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64); }
extern "C" { pub fn f_016650_rage_phwind_initsession(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016651_rage_phwindsphere_apply_rage_phwindfield_ptr_unsigned_int_unsign(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016693_rage_phverletcloth_detectandenforceonecapsule_bool_rage_scalarv(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_016694_rage_quickrejectlist_int_rage_scalarv_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_016695_rage_phverletcloth_detectandenforceone_bool_rage_phbound_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_016696_rage_phverletcloth_detectandenforcelist_rage_mat34v_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016710_rage_phwind_dtorphwind(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016720_rage_phforcesolver_solve(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016721_rage_phcontactmgr_dtorphcontactmgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016725_rage_phcontactmgr_updatecontacts_rage_phoverlappingpairarray_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_016726_rage_phcontactmgr_splitpairs_rage_phoverlappingpairarray_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016727_rage_phcontactmgr_solveimpulsesphase1and2_rage_phoverlappingpair(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_016728_rage_phcontactmgr_solveimpulses_rage_phoverlappingpairarray_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_016729_void_std_2_introsort_std_2_classicalgpolicy_rage_manifoldindexso(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_016730_void_std_2_introsort_std_2_classicalgpolicy_rage_manifoldindexso(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_016733_rage_phforcesolver_iterateoneconstraint_rage_phmanifold_ref_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016734_rage_phforcesolver_releasephase_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016735_rage_phcontactmgr_collectimpulsesfrommanifold_rage_phmanifold_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_016736_rage_phcontactmgr_breakobjectsinisland(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016743_rage_updatecontactsmovandfix_rage_phmanifold_ref_rage_phforcesol(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016745_rage_updatecontactsartandfix_rage_phmanifold_ref_rage_phforcesol(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016747_rage_updatecontactsfixedpointmovandfix_rage_phmanifold_ref_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016753_rage_updatecontactsartandmov_rage_phmanifold_ref_rage_phforcesol(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016756_rage_updatecontactsfixedpointartandmov_rage_phmanifold_ref_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016775_rage_preresponseartandmov_rage_phmanifold_ref_rage_phforcesolver(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016779_rage_preresponsefixedpointartandmov_rage_phmanifold_ref_rage_phf(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016790_rage_applyimpulsemovandfix_rage_phmanifold_ref_rage_phforcesolve(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016792_rage_applyimpulseartandfix_rage_phmanifold_ref_rage_phforcesolve(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016797_rage_applyimpulsefixedrotationartandfix_rage_phmanifold_ref_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016802_rage_applyimpulsepivotrotationartandfix_rage_phmanifold_ref_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016805_rage_applyimpulseartandmov_rage_phmanifold_ref_rage_phforcesolve(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016809_rage_applyimpulsefixedpointartandmov_rage_phmanifold_ref_rage_ph(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016813_rage_applyimpulsefixedrotationartandmov_rage_phmanifold_ref_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016817_rage_applyimpulsesliderotationartandmov_rage_phmanifold_ref_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016830_rage_applyimpulseandpushmovandfix_rage_phmanifold_ref_rage_phfor(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016832_rage_applyimpulseandpushartandfix_rage_phmanifold_ref_rage_phfor(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016835_rage_applyimpulseandpushfixedpointartandfix_rage_phmanifold_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016840_rage_applyimpulseandpushsliderotationartandfix_rage_phmanifold_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016843_rage_applyimpulseandpushpivotrotationartandfix_rage_phmanifold_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016846_rage_applyimpulseandpushartandmov_rage_phmanifold_ref_rage_phfor(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016850_rage_applyimpulseandpushfixedpointartandmov_rage_phmanifold_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016854_rage_applyimpulseandpushfixedrotationartandmov_rage_phmanifold_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016861_rage_applyimpulseandpushpivotrotationartandmov_rage_phmanifold_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016869_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_manif(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_016870_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_016871_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_manif(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_016872_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_016874_rage_phlevelnodetree_rage_phlooseoctreenode_dtorphlevelnodetree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016880_rage_phlevelnodetree_rage_phlooseoctreenode_dumpobjects(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016881_rage_phlevelnodetree_rage_phlooseoctreenode_createchild_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_016884_rage_phlevelnodetree_rage_phlooseoctreenode_removeobjectfromoctr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32); }
extern "C" { pub fn f_016887_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectposition(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016888_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectposition(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32); }
extern "C" { pub fn f_016889_rage_phlevelnodetree_rage_phlooseoctreenode_commitdeferredoctree(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016890_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectlocation(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016891_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectradiusin(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32); }
extern "C" { pub fn f_016892_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectlocation(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016893_rage_phlevelnodetree_rage_phlooseoctreenode_rebuildcompositebvh(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016903_rage_phlevelnodetree_rage_phlooseoctreenode_checknodeagainstcull(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016904_rage_phlevelnodetree_rage_phlooseoctreenode_getnextcullednode_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016905_rage_phlevelnodetree_rage_phlooseoctreenode_getnextculledobjectf(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016906_rage_phlevelnodetree_rage_phlooseoctreenode_getfirstculledobject(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_016907_rage_phlevelnodetree_rage_phlooseoctreenode_getfirstculledobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016908_rage_phlevelnodetree_rage_phlooseoctreenode_getnextculledobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016909_rage_phlevelnodetree_rage_phlooseoctreenode_testprobe_rage_phseg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_016912_minkowskipenetrationdepthsolver_computepenetrationseparation_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_016913_rage_phverletcloth_updateedgessoavec3vby8s_int_float_vector_4_pt(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_016915_rage_phverletcloth_computeboundingvolume_rage_phbound_ptr_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016925_rage_scrprogram_setglobals(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016926_rage_scrthread_validate_rage_scrprogram_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_016927_scrprogramregistry_insert_rage_scrprogram_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016929_rage_scrprogram_dtorscrprogram(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016931_rage_scrprogram_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016933_rage_scrprogram_setbreakpoint_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_016941_rage_cscriptarraysizemonitor_checkifmonitoredaddresshaschanged_c(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016943_rage_scrthread_printstacktrace_void_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016944_rage_scrthread_preprintstacktrace(ph: *mut Phantom); }
extern "C" { pub fn f_016949_rage_scrthread_getactivethread(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_016951_rage_scrthread_fault_int_char_const_ptr_rage_scrthreadfaultid(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_016952_rage_scrthread_getprogramcounter_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016954_rage_scrthread_isvalidreference_rage_scrvalue_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016955_char_ptr_rage_formatf_char_48ul_char_ref_48ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016956_rage_scrthread_dumparrayinfo_rage_scrvalue_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016957_rage_scrthread_setbreakpoint_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_016963_rage_scrcommandhash_void_ptr_rage_scrthread_info_ref_insert_unsi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016964_rage_atmapentry_void_ptr_nativecallloginfo_const_ref_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016965_rage_athashedmap_scrsignatureandcmd_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016966_rage_atmap_void_ptr_nativecallloginfo_rage_atmaphashfn_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016974_rage_scrthread_createthread_char_const_ptr_void_const_ptr_int_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_016975_rage_scrthread_createthread_rage_scrprogramid_void_const_ptr_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_016977_rage_scrthread_startnewscriptinternal_rage_scrthread_info_ref_ra(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016981_rage_scrthread_getthread_rage_scrthreadid(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_016983_rage_sysobfuscated_unsigned_long_true_ref_rage_atmap_rage_sysobf(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016987_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_sc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_017079_printcommon_rage_diagchannel_ref_rage_diagseverity_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_017080_cprintcommon_rage_scrthread_info_ref_char_const_ptr_rage_diagsev(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_017081_char_ptr_rage_formatf_char_96ul_char_ref_96ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_017084_rage_scrdebugger_resolve_rage_scrthread_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_017087_rage_scrdebugger_parsetype_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_017088_rage_scrdebugger_type_t_ptr_ref_rage_atmap_rage_conststring_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_017091_rage_atarray_rage_scrdebugger_item_0_unsigned_short_destruct_rag(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_017092_rage_scrdebugger_scrdebugger_char_const_ptr_rage_scrthreadid_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_017102_rage_atarray_rage_scrdebugger_item_0_unsigned_short_grow_unsigne(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017108_rage_scrdebugger_continue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017113_rage_scrdebugger_populatemasterdebuggercombobox(ph: *mut Phantom); }
extern "C" { pub fn f_017114_rage_scrdebugger_bankcomboboxchanged(ph: *mut Phantom); }
extern "C" { pub fn f_017115_rage_scrdebugger_getlineinfofromprogramcounter_int_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_017116_rage_scrdebugger_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017117_rage_scrdebugger_item_op_rage_scrdebugger_item_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017119_rage_scrdebugger_dtorscrdebugger(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017121_rage_scrwidgeteditor_exit_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_017122_rage_scrdebugger_updatelineinfo_rage_scrthread_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_017123_rage_scrwidgeteditor_setprogramcounter_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_017124_rage_scrwidgeteditor_continue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017125_rage_scrwidgeteditor_sendbreakpointrequestresponse_int_char_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_017126_rage_scrwidgeteditor_operationfailed(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017127_rage_scrdebugger_item_finditem_rage_scrdebugger_ptr_rage_atarray(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_017128_rage_scrwidgeteditor_processdebuggeraction_rage_bkremotepacket_c(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_017178_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_017179_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_017181_rage_crclip_crclip_rage_crclip_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_017182_rage_crclip_crclip_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_017183_rage_crclip_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017184_rage_crclip_dtorcrclip(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017186_rage_crclip_composite_rage_crframe_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_017187_rage_crclip_initclass(ph: *mut Phantom); }
extern "C" { pub fn f_017188_rage_crexpressionop_initclass(ph: *mut Phantom); }
extern "C" { pub fn f_017190_rage_crclip_calcblockedtimeinternal_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_017191_rage_crclip_allocateandload_char_const_ptr_rage_cranimloader_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_017192_rage_crclip_save_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_017193_rage_crclip_serialize_rage_datserialize_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017194_rage_crclip_allocateandloadanimation_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_017195_rage_crclip_dump_rage_crdumpoutput_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017196_rage_crclipanimation_crclipanimation_rage_crclip_ecliptype(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017197_rage_crclipanimation_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017198_rage_crclipanimation_dtorcrclipanimation(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017199_rage_crclipanimation_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017205_rage_crclipanimation_serialize_rage_datserialize_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017206_rage_crclipanimation_dump_rage_crdumpoutput_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017207_rage_crclipanimation_create_rage_cranimation_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_017209_rage_crclipanimation_composite_rage_crframecompositor_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_017215_rage_crclipanimations_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017228_rage_crclipplayer_setclip_rage_crclip_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017229_rage_crclipplayer_update_float_float_ref_bool_ref_rage_functor4(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_017230_rage_crclipplayer_settime_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_017231_rage_crclipplayer_settime_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_017232_rage_crclipdictionary_crclipdictionary_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_017233_rage_crclipdictionary_dtorcrclipdictionary(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017234_rage_crclipdictionary_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017240_rage_crexpressions_crexpressions_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_017249_rage_crexpressions_load_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_017257_void_rage_datresource_place_rage_crpmpointcloud_ptr_rage_crpmpoi(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017260_rage_atarray_rage_datowner_rage_crposematcher_matchsample_0_unsi(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017261_rage_atarray_rage_vec3v_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017267_rage_crcreaturecomponentevent_flushpendingevents(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017278_void_rage_datresource_place_rage_crexpression_ptr_rage_crexpress(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017279_rage_crexpressions_dtorcrexpressions(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017282_rage_crexpressions_findexpressionframeindices_rage_craccelerator(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_017284_rage_crexpressions_findexpressionskelindices_rage_craccelerator(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_017286_rage_crexpressionop_dumphelper_outputf_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_017287_rage_crexpressions_calcinputoutputdofs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017288_rage_crexpressions_calcvariables(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017289_rage_calcinputoutputdofsiterator_visit_rage_crexpressionop_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_017290_rage_calcvariablesiterator_visit_rage_crexpressionop_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_017291_rage_simplifyiterator_visit_rage_crexpressionop_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_017292_rage_crexpressionpacker_convertrecursive_rage_crexpressionop_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017293_rage_calcstackdepthiterator_visit_rage_crexpressionop_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017294_rage_atarray_rage_crexpressions_track_0_unsigned_short_grow_unsi(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_017297_rage_crexpressionop_serializehelper_op_rage_datowner_rage_crexpr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017301_rage_crexpressionop_dumphelper_internalprintf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_017322_rage_crexpressionopgetset_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017323_rage_crexpressionopgetset_serialize_rage_crexpressionop_serializ(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017332_rage_crexpressionopget_serialize_rage_crexpressionop_serializehe(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017354_rage_crexpressionopcomponentgetset_serialize_rage_crexpressionop(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017356_rage_imp_quatvtoeulers_imp_rage_quatv_const_ref_rage_eulerangleo(ph: *mut Phantom, a0: i64, a1: i32) -> u128; }
extern "C" { pub fn f_017382_rage_crexpressionopobjectspaceget_calcparentobjectspacematrix_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_017383_rage_crexpressionopobjectspaceset_crexpressionopobjectspaceset_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_017441_rage_atrangearray_rage_datowner_rage_crexpressionop_2_atrangearr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017456_rage_atrangearray_rage_datowner_rage_crexpressionop_3_atrangearr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017471_rage_atarray_rage_datowner_rage_crexpressionop_0_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_017509_rage_crexpressionopspeciallinear_getsourcevalue_rage_crexpressio(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_017558_rage_crexpressionpacker_scratchstream_insert_rage_exprop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017559_rage_crexpressionpacker_createbranch_rage_exprop_rage_crexpressi(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_017560_rage_crexpressionpacker_createbranch_rage_exprop_rage_crexpressi(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_017561_rage_crexpressionpacker_convertgetsetop_rage_crexpressionopgetse(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017562_rage_crexpressionplayer_crexpressionplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017563_rage_crexpressionplayer_setexpressions_rage_crexpressions_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017564_rage_crexpressionplayer_setfunctor_rage_functor0_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017565_rage_crexpropframe_calcobjectspacematrix_rage_exprstreamhelper_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_017569_rage_crpmmotioncontroller_initbase_rage_crskeletondata_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_017570_rage_crpmmotioncontrollerclip_dtorcrpmmotioncontrollerclip(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017571_rage_crpmmotioncontrollerclip_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017581_rage_crpmmotioncontrollerrc_init_rage_crpmregistrationcurvedata(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_017582_rage_crpmregistrationcurveplayer_init_rage_crpmregistrationcurve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_017583_rage_crpmregistrationcurveplayer_composite_rage_crframe_ref_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017584_rage_crpmmotioncontrollerrc_dtorcrpmmotioncontrollerrc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017586_rage_crpmregistrationcurvedata_maputoclipu_float_int_const(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> f32; }
extern "C" { pub fn f_017590_rage_crpmregistrationcurveplayer_update_float_rage_crpmweights_c(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_017591_rage_crpmregistrationcurveplayer_calcdeltatime_float_rage_crpmwe(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> f32; }
extern "C" { pub fn f_017595_rage_crpmregistrationcurveplayer_calctotaltime_rage_crpmweights(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_017599_void_rage_serializeatlarrayasstlarray_float_rage_datserialize_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017600_rage_crpmblenddatabase_mapparametertoweights_rage_crpmparameter(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_017601_rage_crpmblenddatabase_calcparameterdistancesqr_rage_crpmparamet(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> f32; }
extern "C" { pub fn f_017602_rage_dumpparameter_int_char_const_ptr_rage_crpmparameter_const_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_017603_rage_crpmparameterization_init_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_017604_rage_atarray_rage_crpmparameterization_dimensioninfo_0_unsigned(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017605_rage_crpmparameterinterpolator_reset_rage_crpmparameter_const_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017606_rage_crpmparameterinterpolator_update_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_017609_rage_crpmparameterizedmotiondata_declarestruct_rage_dattypestruc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017611_rage_crpmparameterizedmotionsimpledata_load_char_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_017612_rage_crpmparameterizedmotionvariabledata_load_char_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_017617_rage_crpmparameterizedmotion_getnumparameterdimensions_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_017619_rage_crpmparameterizedmotionsimpledata_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017621_rage_crpmparameterizedmotionsimpledata_declarestruct_rage_dattyp(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017623_rage_crpmparameterizedmotionsimpledata_serialize_rage_datseriali(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017625_rage_crpmparameterizedmotionsimple_dtorcrpmparameterizedmotionsi(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017627_rage_crpmparameterizedmotionsimple_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017653_rage_crpmparameterizedmotionvariabledata_dtorcrpmparameterizedmo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017654_rage_crpmparameterizedmotionvariabledata_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017655_rage_atarray_rage_crpmparameterizedmotionvariabledata_variation(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_017666_rage_crpmparameterizedmotionplayer_crpmparameterizedmotionplayer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017667_rage_crpmparametersampler_dtorcrpmparametersampler(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017669_rage_crpmparametersampler_createsampler_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017670_rage_crpmparametersampler_configsampler_rage_crpmparametersample(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_017672_rage_crpmparametersampler_configparser_stripfloatproperty_float(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_017673_rage_crpmparametersampler_configparser_stripstringproperty_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_017674_rage_crpmparametersampler_configparser_stripintproperty_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_017675_rage_crpmparametersampler_configparser_stripboolproperty_bool_re(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_017676_rage_crpmparametersampler_configparser_stripfloatarrayproperty_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_017677_rage_crpmparametersampler_configproperty_char_const_ptr_rage_crp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_017684_rage_crpmparametersamplerphysical_configproperty_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_017685_rage_crpmparametersamplercontainer_dtorcrpmparametersamplerconta(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017686_rage_crpmparametersamplercontainer_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017690_rage_crpmparametersampler_crpmparametersampler_rage_crpmparamete(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_017691_rage_atarray_rage_crpmparameter_0_unsigned_short_copy_rage_crpmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_017694_rage_atarray_rage_crpmparameter_0_unsigned_short_grow_unsigned_i(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017698_rage_crpmparametersamplerclipproperty_dtorcrpmparametersamplercl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017714_rage_crpmparametersamplerdirectional_dtorcrpmparametersamplerdir(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017724_rage_crpmparametersamplerspatial_dtorcrpmparametersamplerspatial(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017755_rage_atmapentry_unsigned_int_rage_datowner_rage_crclip_declarest(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017756_rage_atmapentry_unsigned_int_rage_datref_rage_crpmparameterizedm(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017760_void_rage_crmtiterator_iteratebatch_64_false_rage_crmtnode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017763_rage_crmtmotiontree_iterate_rage_crmtiterator_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017764_rage_crmtiterator_iterate_rage_crmtnode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017765_void_rage_crmtiterator_iteratebatch_64_true_rage_crmtnode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017767_rage_crmtrequest_generate_rage_crmtmotiontree_ref_rage_crmtnode(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_017768_rage_crmtnode_addobserver_rage_crmtobserver_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017769_rage_crmtobserver_attach_rage_crmtnode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017770_rage_crmtobserver_detach(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017772_rage_crmtiterator_crmtiterator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017775_rage_crmtdestructor_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017777_rage_crmtnode_releaseinternal_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017778_rage_crmtvalidator_reportf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_017786_rage_crmtobserver_crmtobserver(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017790_rage_crmtinterceptor_registermessageintercept_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_017791_rage_crmtsynchronizer_dtorcrmtsynchronizer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017794_rage_crmtsynchronizer_addtargetunsafe_rage_crmtnode_ref_bool_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_017795_rage_crmtsynchronizer_resetalltargetweights(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017801_rage_crmtsynchronizer_syncobserver_dtorsyncobserver(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017802_rage_crmtsynchronizer_syncobserver_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017808_rage_crmtsynchronizerphase_crmtsynchronizerphase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017809_rage_crmtsynchronizerphase_dtorcrmtsynchronizerphase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017812_rage_crmtsynchronizerphase_synchronize_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_017813_rage_crmtsynchronizertag_crmtsynchronizertag(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017814_rage_crmtsynchronizertag_dtorcrmtsynchronizertag(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017815_rage_crmtsynchronizertag_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017818_rage_crmtsynchronizertag_synctagobserver_dtorsynctagobserver(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017823_rage_crmtsynchronizertag_synchronize_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_017824_rage_crmtsynchronizertag_insertuniquepattern_rage_crmtsynchroniz(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_017825_rage_crmtsynchronizertag_calcpatternmatchwithoffset_rage_crmtsyn(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_017827_calcmoverdeltaunsafe_rage_crblockstream_const_ptr_float_float_ra(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_017843_constraint_rage_crmotiondescription_const_ref_rage_transformv_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_017845_rage_crmtcomposer_pushanimation_rage_cranimation_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: i64, a8: i32) -> i32; }
extern "C" { pub fn f_017846_rage_crmtcomposer_pushinvalid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017847_rage_crmtcomposer_pushexpressions_rage_crexpressions_const_ptr_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: i64, a6: i32, a7: i64, a8: i64); }
extern "C" { pub fn f_017850_rage_crmtrequest_dtorcrmtrequest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017852_rage_crmtnodefactory_alloc_rage_crmtnodefactory_ref_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_017857_rage_crmtmotiontreescheduler_signalthreads_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_017861_rage_crmtnode_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017862_rage_crmtnode_preinit_rage_crmtmotiontree_ref_rage_crmtnode_enod(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_017863_rage_crmtnode_dump_rage_crmtdumper_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017865_rage_crmtnode_orphan(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017866_rage_crmtnode_replace_rage_crmtnode_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_017867_rage_crmtnodeparent_replacechild_rage_crmtnode_ref_rage_crmtnode(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_017868_rage_crmtnodeparent_removechild_rage_crmtnode_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_017872_rage_crmtnode_nodetypeinfo_register_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017873_rage_crmtnode_createnodefromnodefactory_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_017875_rage_crmtnodeparent_crmtnodeparent_rage_crmtnode_enodes(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_017876_rage_crmtnodeparent_crmtnodeparent_rage_datresource_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017877_rage_crmtnodeparent_dtorcrmtnodeparent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017878_rage_crmtnodeparent_removechildren(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017879_rage_crmtnodeparent_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017880_rage_crmtnodeparent_dump_rage_crmtdumper_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017882_rage_crmtnodeparent_addchild_rage_crmtnode_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_017883_rage_crmtnodeparent_promotechild_rage_crmtnode_ref_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017885_rage_crmtnodeanimation_dtorcrmtnodeanimation(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017887_rage_crmtnodeanimation_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017895_rage_crmtnodepair_dtorcrmtnodepair(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017896_rage_crmtnodepair_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017897_rage_crmtnodepair_setfilter_rage_crframefilter_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017904_rage_crmtnodepairweighted_dump_rage_crmtdumper_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017907_rage_crmtnodeblend_dtorcrmtnodeblend(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017908_rage_crmtnodeblend_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017910_rage_crmtnodeblend_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017914_rage_crmtnodeblend_init_float_float_bool_bool_rage_crframefilter(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_017917_rage_crmtnodeblend_setweight_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_017919_rage_crmtnodeaddsubtract_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017925_rage_crmtnodeframe_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017927_rage_crmtnodeframe_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017931_rage_crmtnodeframe_init_rage_crframe_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017935_rage_crmtnodefilter_dtorcrmtnodefilter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017937_rage_crmtnodefilter_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017944_rage_crmtnodemirror_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017946_rage_crmtnodemirror_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017950_rage_crmtnodemirror_init_rage_crframefilter_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017952_rage_crmtnodeik_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017954_rage_crmtnodeik_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017960_rage_crmtnoden_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017961_rage_crmtnoden_init_rage_crframefilter_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017964_rage_crmtnoden_setweight_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_017965_rage_crmtnoden_setinputfilter_unsigned_int_rage_crframefilter_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_017966_rage_crmtnoden_getweight_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_017968_rage_crmtnodeblendn_dtorcrmtnodeblendn(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017970_rage_crmtnodeblendn_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017975_rage_crmtnodeclip_crmtnodeclip(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017979_rage_crmtnodeclip_dtorcrmtnodeclip(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017981_rage_crmtnodeclip_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017991_rage_crmtnodepm_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018001_rage_crmtnodeextrapolate_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018003_rage_crmtnodeextrapolate_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018008_rage_crmtnodeexpression_dtorcrmtnodeexpression(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018009_rage_crmtnodeexpression_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018011_rage_crmtnodeexpression_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018015_rage_crmtnodeexpression_init_rage_crexpressions_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018019_rage_crmtnodeexpression_setweight_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_018020_rage_crmtnodeproxy_dtorcrmtnodeproxy(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018021_rage_crmtnodeproxy_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018022_rage_crmtobserver_dtorcrmtobserver_30310(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018024_rage_crmtnodeproxy_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018028_rage_crmtnodeproxy_setproxynode_rage_crmtnode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018031_rage_crmtnodecapture_dtorcrmtnodecapture(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018032_rage_crmtnodecapture_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018034_rage_crmtnodecapture_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018040_rage_crmtnodeaddn_dtorcrmtnodeaddn(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018042_rage_crmtnodeaddn_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018048_rage_crmtnodeidentity_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018055_rage_crmtnodepose_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018063_rage_crmtnodemerge_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018069_rage_crmtnodemergen_dtorcrmtnodemergen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018071_rage_crmtnodemergen_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018077_rage_crmtnodeinvalid_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018083_rage_crmtnodejointlimit_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018085_rage_crmtnodejointlimit_createnodebase_rage_crmtmotiontree_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018097_rage_crmtrequestsource_2u_generatesourcenodes_rage_crmtmotiontre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_018100_rage_crmtrequestframe_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018108_rage_crmtrequestsource_1u_generatesourcenodes_rage_crmtmotiontre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_018114_rage_crmtrequestclip_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018154_rage_mvfixedheap_alloc_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_018156_rage_mvcommandbuffer_setmanager_rage_mvcommandbuffer_manager_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018158_rage_mvfixedheap_free_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018159_rage_mvparameterbuffer_buffer_free_rage_mvparameterbuffer_buffer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018161_rage_mvparameterbuffer_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018162_rage_mvparameterbuffer_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018164_rage_mvparameterbuffer_deduplicate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018165_rage_mvparameterbuffer_compact(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018167_rage_mvparameterbuffer_converttosignal_rage_mvnetworkdef_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018168_rage_mvparameterbuffer_move_rage_mvparameterbuffer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018169_rage_mvparameterbuffer_generatefreecapacity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018171_rage_mvparameterbuffer_generatemap_rage_mvparameterbuffer_map_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_018173_rage_mvmotionweb_addref_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018176_rage_mvnodestatedef_registerhandlers_rage_mvmotionweb_ptr_rage_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018189_rage_mvparameterbuffer_copy_rage_mvparameterbuffer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018215_rage_mvfixedheap_freeinternal_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018223_rage_mvsynchronizer_attachtoblend_rage_crmtnodeblend_ref_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_018227_rage_addweightedtargetiterator_iteratenosiblings_rage_crmtnode_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018238_rage_mvnodepairdef_create_rage_crmtnodepairweighted_ptr_rage_mvc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018249_rage_mvnodendef_create_rage_crmtnoden_ptr_rage_mvcreatecontext_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018312_rage_mvupdateeventfromcliptag_isclipnoderelevant_float_float_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_018408_rage_mvnodestate_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018415_rage_mvnodestatemachine_initinternal_bool_rage_mvmotionweb_ptr_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_018416_rage_mvnodestatemachine_applystate_rage_mvnodedef_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018425_rage_mvnodestatebasedef_init_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018426_rage_mvnodestatebasedef_inittransitions_rage_fistream_ptr_rage_m(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_018448_rage_mvbuffers_rage_mvparameterbuffer_dtormvbuffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018457_rage_addweightedtargetiterator_iterate_rage_crmtnode_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_018458_rage_addweightedtargetiterator_iteratenosiblings_rage_crmtnode_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_018465_gmsgformat_bindnonpos(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018466_gmsgformat_makestring(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018473_gimage_gimage_gimagebase_imageformat_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_018479_gutf8util_decodenextchar_char_const_ptr_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_018493_gmatrix3d_matrixinverse_float_ptr_float_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018497_gmatrix3d_multiplymatrix_gmatrix3d_const_ref_gmatrix3d_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018509_gnumericbase_readprintformat_gstringdataptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018512_gutf8util_encodechar_char_ptr_long_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_018519_g_sprintf_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_018521_gresourceformatter_parse_gstringdataptr_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018527_garraydatabase_gmsgformat_fmt_record_gallocatorgh_pod_gmsgformat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018528_gmsgformat_evaluate_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018554_gstring_appendstring_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018575_gwaitable_handlerarray_callwaithandlers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018602_gzlibfileimpl_inflate_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_018606_gzlibfileimpl_setposition_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_018609_gheapbitset2_getblocksize_unsigned_int_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018610_gheapfreebin_push_unsigned_char_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018611_gheapfreebin_pull_unsigned_char_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018612_gheaptreebin_pullnode_gheaptnode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018613_gheapallocbitset1_alloc_unsigned_long_gheapsegment_ptr_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_018614_gheapfreebin_pullbest_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018615_gheaplistbin_pullbest_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018616_gheaptreebin_findbest_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018617_gheapfreebin_merge_unsigned_char_ptr_unsigned_long_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_018618_gheapallocbitset2_alloc_unsigned_long_gheapsegment_ptr_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_018619_gheapbitset2_markbusy_unsigned_int_ptr_unsigned_long_unsigned_lo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_018620_gheapallocbitset2_alloc_unsigned_long_unsigned_long_gheapsegment(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_018621_gheaplistbin_pullbest_unsigned_long_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_018622_gheapallocengine_allocsegmentbitset_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_018623_gheapallocengine_allocsegment_unsigned_int_unsigned_long_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_018624_gheapallocengine_freesegment_gheapsegment_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018625_gheappagetable_unmaprange_void_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018626_gheapbookkeeper_free_void_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018627_gheapallocengine_calcdynasize_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018628_gheapbookkeeper_alloc_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018629_gheappagetable_maprange_void_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_018630_gheappagetable_setsegmentinrange_unsigned_long_unsigned_long_ghe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_018631_gheapgranulator_alloc_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_018632_gheapgranulator_free_void_ptr_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_018633_gheapallocengine_allocsysdirect_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_018634_gheapallocengine_allocsegmenttiny_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_018635_gheapallocengine_freesegmenttiny_gheapsegment_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018636_gheapallocengine_alloc_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_018637_gheapallocengine_free_gheapsegment_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018638_gheapallocengine_alloc_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018639_gheapallocengine_realloc_gheapsegment_ptr_void_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_018640_gheapalloclite_visittree_gheapdualtnode_const_ptr_gheapsegment_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_018641_gheapfreebin_visitmem_gheapmemvisitor_ptr_unsigned_long_gheapmem(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_018642_gheapfreebin_visitunused_gheapsegvisitor_ptr_unsigned_long_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_018643_gradixtreemulti_gheapdualtnode_gheapalloclite_sizeaccessor_remov(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018644_gheapalloclite_pullbest_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_018645_gheapalloclite_splitnode_gheapdualtnode_ptr_unsigned_char_ptr_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_018646_gheapalloclite_free_gheaptreeseg_ptr_void_ptr_unsigned_long_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_018647_gheapalloclite_reallocinplace_gheaptreeseg_ptr_void_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_018648_gheapalloclite_extend_gheaptreeseg_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018649_gheapalloclite_visitunusedintree_gheapdualtnode_const_ptr_gheaps(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_018650_gheapfreebin_visitunusedintree_gheaptnode_const_ptr_gheapsegvisi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_018651_gheapfreebin_visittree_gheaptnode_const_ptr_gheapmemvisitor_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_018652_gheapgranulator_allocsegment_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_018653_gheapgranulator_visitsegments_gheaptreeseg_const_ptr_gheapsegvis(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_018654_gsysallocgranulator_init_gsysallocpaged_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018665_gheaproot_destroyarena_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018666_gheaproot_createheap_char_const_ptr_gmemoryheappt_ptr_gmemoryhea(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_018717_garraypagedbase_gcompoundshape_spath_6_16_gallocatorpagedlh_pod(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018719_garraypagedbase_gpointtype_8_64_gallocatorpagedlh_pod_gpointtype(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018722_gcompoundshape_flattenquadraticcurve_float_float_float_float_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32); }
extern "C" { pub fn f_018727_garraypagedbase_gedgeaa_vertextype_8_64_gallocatorpagedlh_pod_ge(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018755_grasterizer_line_int_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_018756_garraypagedbase_grasterizer_cell_8_64_gallocatorpagedlh_pod_gras(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018757_grasterizer_horline_int_int_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_018761_garraypagedbase_grectpacker_nodetype_8_64_gallocatorpagedlh_pod(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018762_grectpacker_packrects_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_018773_gstrokepath_closepath_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_018774_garraypagedbase_gstrokevertextype_6_16_gallocatorpagedlh_pod_gst(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018775_gstroker_calcarc_gcompoundshape_ref_float_float_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32); }
extern "C" { pub fn f_018776_gstroker_calcmiter_gcompoundshape_ref_gstrokevertextype_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32, a7: f32, a8: f32, a9: i32, a10: f32, a11: f32, a12: f32); }
extern "C" { pub fn f_018777_gstroker_calccap_gcompoundshape_ref_gstrokevertextype_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i32); }
extern "C" { pub fn f_018778_gstroker_calcjoin_gcompoundshape_ref_gstrokevertextype_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: f32); }
extern "C" { pub fn f_018779_gstroker_generateequidistant_gcompoundshape_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_018782_gstrokeraa_calcbuttcap_gstrokevertextype_const_ref_gstrokevertex(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32); }
extern "C" { pub fn f_018783_garraypagedbase_gstrokeraa_vertextype_8_64_gallocatorpagedlh_pod(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018784_gstrokeraa_calcroundcap_gstrokevertextype_const_ref_gstrokeverte(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32); }
extern "C" { pub fn f_018785_gstrokeraa_calcroundjoin_gstrokevertextype_const_ref_gstrokeraa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_018786_gstrokeraa_calcmiterjoin_gstrokevertextype_const_ref_gstrokeraa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_018787_gstrokeraa_calcbeveljoin_gstrokevertextype_const_ref_gstrokeraa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_018788_gstrokeraa_calcjoinparam_gstrokevertextype_const_ref_gstrokevert(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_018790_gtessellator_buildedgelist_unsigned_int_unsigned_int_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_018791_garraypagedbase_gtessellator_edgetype_10_64_gallocatorpagedlh_po(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018792_garraypagedbase_gpointtype_10_64_gallocatorpagedlh_pod_gpointtyp(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018793_gtessellator_perceivestyles_garrayunsafelh_pod_gtessellator_mono(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018794_garraypagedbase_gtessellator_monotonetype_6_64_gallocatorpagedlh(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018795_gtessellator_growmonotoneandconnect_gtessellator_scanchaintype_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_018796_garraypagedbase_gtessellator_monovertextype_10_64_gallocatorpage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018797_gtessellator_replacemonotone_gtessellator_scanchaintype_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_018798_gtessellator_growmonotone_gtessellator_monotonetype_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_018799_gtessellator_connectstarting_gtessellator_scanchaintype_ptr_gtes(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018800_gtessellator_sweepscanbeam_garrayunsafelh_pod_gtessellator_monoc(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_018801_gtessellator_addeventvertex_gtessellator_monochaintype_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_018804_gtessellator_triangulatecoherentcurve(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018805_garraypagedbase_gtessellator_triangletype_10_64_gallocatorpagedl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018806_gtessellator_triangulateconvexshape(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018807_garraypagedbase_unsigned_int_8_64_gallocatorpagedlh_pod_unsigned(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018808_gtessellator_addtriangle_unsigned_int_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_018829_ghashsetbase_ghashnode_int_gstringdataptr_gfixedsizehash_int_gha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_018839_pedcomponentsforonedlcpack_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_018842_pedmodeldata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_018862_vehiclemodkitdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_018877_cleveldata_loadxmldata(ph: *mut Phantom); }
extern "C" { pub fn f_018880_cgamelogic_hasteleportplayerfinished(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_018881_cpedinventory_getweaponammo_cweaponinfo_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_018882_cgamelogic_teleportplayer_cped_ptr_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_018883_cgamelogic_resurrectlocalplayer_rage_vector3_const_ref_float_boo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_018884_cgarage_storeandremovecarsforthishideout_cstoredvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_018891_cgamelogic_getgamelogicbank(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_018895_cgarage_dtorcgarage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018897_cgarages_clearall(ph: *mut Phantom); }
extern "C" { pub fn f_018899_cgarages_load(ph: *mut Phantom); }
extern "C" { pub fn f_018900_cgarages_addone_rage_atarray_cgarage_box_0_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_018902_cgarage_updatesize_int_cgarage_box_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_018904_cgarage_isgarageempty_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_018905_cgarageeditor_displaygarages(ph: *mut Phantom); }
extern "C" { pub fn f_018906_cgarageeditor_refreshtweakvals(ph: *mut Phantom); }
extern "C" { pub fn f_018907_cgarage_isentityentirelyoutside_centity_const_ptr_float_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_018908_cgarage_isplayerentirelyinsidegarage_centity_const_ptr_float_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_018909_cgarage_isentitytouching3d_centity_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_018910_cgarage_isentityentirelyinside3d_centity_const_ptr_float_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_018911_cgarage_findallobjectsingarage_rage_atarray_centity_ptr_0_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_018912_cgarages_isanyentityentirelyinsidegarage_int_bool_bool_bool_floa(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_018913_cgarage_buildquadtestpoints_centity_const_ptr_rage_vector3_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_018914_cgarage_buildboxtestpoints_centity_const_ptr_rage_vector3_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_018915_cgarage_islineintersectinggarage_rage_vector3_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_018916_statid_op_statid_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018917_cgarage_printmessage_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018918_cplayerinfo_disablecontrolsgarages(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018919_cgarage_throwcarsneardooroutofgarage_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018920_cgarage_slidedoorclosed(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_018921_rage_fwentity_getmodelid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018922_cgarage_slidedooropen(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_018923_cplayerinfo_enablecontrolsgarages(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018924_cgarage_isbeyondstoredistance2_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_018925_cgarage_safesetdoorfixedphysics_bool_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018926_cgarage_iswithinrestoredistance2_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_018927_cgarage_restorecarsforthishideout_cstoredvehicle_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_018928_cgarage_ispointinsidegarage_rage_vector3_const_ref_float_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32) -> i32; }
extern "C" { pub fn f_018929_cgarage_entityhasaspherewayoutsidegarage_centity_ptr_float(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_018930_cgarage_op_cgarage_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018931_cgarages_findgarageindexfromnamehash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_018933_cgarage_canthisvehiclebestoredinthisgarage_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_018967_cgarages_cgarageinitdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_018976_cgpsslot_init_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_018977_cgpsslot_clear_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_018978_cgpsslot_start_rage_vector3_ref_centity_ptr_int_bool_int_rage_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_018979_cgpsslot_update_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_018980_cgpsslot_calcdistancetotarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018981_cgps_getrelevantplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_018982_cgpsslot_checkforreachingdestination(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018983_cgps_getrelevantplayercoords(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018984_rage_vector3_isclose_rage_vector3_const_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_018985_cgpsslot_getnextnodepositionalongroute_bool_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_018986_cgpsslot_calcclosestsegment_int_float_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_018987_cgpsslot_shiftroute_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_018988_cminimap_isinbigmap(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_018989_cpathfind_isregionloaded_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_018997_cgps_shouldgpsbevisible_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_019008_cgps_initcustomroutefromwaypointrecordingroute_char_const_ptr_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_019010_cgps_initcustomroutefromassistedmovementroute_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_019026_sleveldata_dtorsleveldata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019031_cwaypointrecordingroute_addwaypoint_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i32); }
extern "C" { pub fn f_019037_cwaypointrecording_startplayback_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019045_cwaypointrecording_requestrecording_char_const_ptr_rage_scrthrea(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019047_cwaypointrecording_removerecording_char_const_ptr_rage_scrthread(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019050_testplaystreamedrecording_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019086_cwaypointrecording_edit_initwaypointwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_019087_cwaypointrecordingroute_insertwaypoint_int_rage_vector3_const_re(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_019099_cpopulationhelpers_getconfigvalue_int_const_ref_int_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_019107_cpopgenshape_drawdebugwedge_rage_vector3_const_ref_float_float_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_019120_cvehiclerecordingmgr_restoreinfoformatrix_rage_matrix34_ref_cveh(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019121_cvehiclerecordingmgr_crecording_stoprecording_unsigned_char_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019122_cvehiclerecordingmgr_initialisedata(ph: *mut Phantom); }
extern "C" { pub fn f_019125_cvehiclerecordingstreamingmodule_cvehiclerecordingstreamingmodul(ph: *mut Phantom); }
extern "C" { pub fn f_019128_cvehiclerecordingmgr_stopplaybackwithindex_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019130_cvehiclerecordingmgr_startplaybackrecordedcar_cvehicle_ptr_int_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_019131_cvehiclerecordingmgr_updateplaybackforvehiclerecording_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_019132_cvehiclerecordingmgr_restoreinfoforcar_cvehicle_ptr_cvehiclestat(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_019133_cvehiclerecordingmgr_getplaybackslot_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019136_cvehiclerecordingmgr_isplaybackgoingonforcar_cvehicle_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019142_cvehiclerecordingmgr_changecarplaybacktouseai_cvehicle_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_019146_cvehiclerecordingmgr_getendtimeofrecording_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_019159_stoprecordingplayback(ph: *mut Phantom); }
extern "C" { pub fn f_019171_crestart_resetdata(ph: *mut Phantom); }
extern "C" { pub fn f_019188_cnetrespawnmgr_findlargestbunchofplayers_bool_rage_vector3_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_019190_cnetrespawnmgr_startsearch_cnetrespawnmgr_startsearchinput_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019192_cnetrespawnmgr_copyenemyandfriendlypositionsfromoverrides_rage_a(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_019193_cnetrespawnmgr_findenemyandfriendlyplayerpositionsfromnetwork_ra(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019196_cnetrespawnmgr_findenemyandfriendlyaipositionsfromworld_rage_atf(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019197_cnetrespawnmgr_findfieldofviewatposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> f32; }
extern "C" { pub fn f_019198_cnetrespawnmgr_findclosestenemyandfriendlypositions_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_019199_cnetrespawnmgr_evaluatepositions_cnetrespawnmgr_evaluateposition(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019200_cnetrespawnmgr_calculateprojectedposition_cped_const_ref_float_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_019212_cslownesszone_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_019218_cslownesszonemanager_reload(ph: *mut Phantom); }
extern "C" { pub fn f_019233_cstuntjumpmanager_printbigmessagestats_char_const_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_019234_cstuntjumpmanager_printbigmessage_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_019244_cstuntjumpmanager_registeraddedstuntjump_cstuntjump_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019294_caitarget_setposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019311_centityscanner_dtorcentityscanner(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019312_centityscanner_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019315_centityscanner_sortandstoreentities_rage_atfixedarray_centitysca(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019317_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_019324_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_cspatia(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_019331_cobjectscanner_dtorcobjectscanner(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019351_cambientmodel_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_019355_cambientmodelset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_019370_cambientmodelset_getrandommodelhash_float_cambientmodelvariation(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_019373_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_019382_cambientmodelsetmanager_loadmodelsets_cambientmodelsetmanager_mo(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019383_rage_atarray_cambientmodelsetmanager_samssourceinfo_0_unsigned_s(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019397_cambientmodelsetmanager_savemodelsets_cambientmodelsetmanager_mo(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019402_cconditionalclipset_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_019409_cconditionalanims_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_019414_cconditionalanims_dtorcconditionalanims(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019415_cconditionalhelpers_getimmediateconditionshelper_rage_atarray_cs(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_019417_clearconditionalclipsetarray_rage_atarray_cconditionalclipset_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019426_cconditionalanimsgroup_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_019433_cconditionalanimmanager_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019435_cconditionalanimmanager_dtorcconditionalanimmanager_32490(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019437_cconditionalanimmanager_load(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019438_cconditionalanimmanager_append_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019442_cconditionalanimmanager_addparserwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019444_caidebuginfo_println_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019445_caidebuginfo_colorprintln_rage_color32_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_019446_caidebuginfo_writeheader_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019447_caidebuginfo_writedatavalue_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_019448_caidebuginfo_writelogevent_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019449_caidebuginfo_writeboolvalue_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_019450_char_ptr_rage_formatf_char_100ul_char_ref_100ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019461_ailogging_getentitydescription_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019466_ctasksfulldebuginfo_ctasksfulldebuginfo_cped_const_ptr_caidebugi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019467_canimationdebuginfo_canimationdebuginfo_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019468_cikdebuginfo_cikdebuginfo_cped_const_ptr_caidebuginfo_debuginfop(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019469_cattachmentsdebuginfo_cattachmentsdebuginfo_cphysical_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019470_ccombattextdebuginfo_ccombattextdebuginfo_cped_const_ptr_caidebu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019471_cdamagedebuginfo_cdamagedebuginfo_cped_const_ptr_bool_caidebugin(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_019472_cmovementtextdebuginfo_cmovementtextdebuginfo_cped_const_ptr_cai(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019473_cpedflagsdebuginfo_cpedflagsdebuginfo_cped_const_ptr_caidebuginf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019474_crelationshipdebuginfo_crelationshipdebuginfo_cped_const_ptr_cai(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019475_cwanteddebuginfo_cwanteddebuginfo_cped_const_ptr_caidebuginfo_de(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019476_cvehiclelockdebuginfo_cvehiclelockdebuginfo_cped_const_ptr_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_019477_ceventsdebuginfo_ceventsdebuginfo_cped_const_ptr_rage_fwflags_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_019478_ctargetingdebuginfo_ctargetingdebuginfo_cped_const_ptr_caidebugi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019479_cvehiclelockdebuginfo_dtorcvehiclelockdebuginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019504_cvehicledebuginfo_cvehicledebuginfo_cvehicle_const_ptr_caidebugi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_019505_canimclipdebugiterator_getclipdebuginfo_rage_crclip_const_ptr_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64); }
extern "C" { pub fn f_019512_ceventsdebuginfo_geteventhistoryentrydescription_cpedintelligenc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019513_ceventsdebuginfo_geteventhistoryentrystatedescription_cpedintell(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019533_ctaskentervehicledebuginfo_printcommondebug(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019552_cridableentrypointevaluationdebuginfo_reset(ph: *mut Phantom); }
extern "C" { pub fn f_019559_cvehicletaskdebuginfo_printtask_ctask_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019560_cvehicletaskdebuginfo_printtaskhistory_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019567_cdebugclipdictionary_pushassetfolder_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019591_cpedpropexpressiondata_dtorcpedpropexpressiondata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019593_cpedcompexpressiondata_dtorcpedcompexpressiondata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019595_ccreaturemetadata_dtorccreaturemetadata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019600_cfacialdatacomponent_setfacialidleclipid_cped_ptr_rage_fwmvtempl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_019601_cfacialdatacomponent_playfacialidleclip_cped_ref_rage_fwmvtempla(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32); }
extern "C" { pub fn f_019604_cmoveped_setfacialidleclip_rage_crclip_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_019605_cmoveped_requestfacialstate_cmoveped_efacialstate_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_019610_cmoveped_playoneshotfacialclip_rage_crclip_const_ptr_float_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_019623_cmoveanimatedbuilding_getphase(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_019624_cmoveanimatedbuilding_getrate(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_019634_cmoveped_settasknetwork_rage_fwmovenetworkplayer_ptr_float_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_019635_cmoveped_setsecondarytasknetwork_rage_fwmovenetworkplayer_ptr_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_019636_cmoveped_setadditivenetwork_rage_fwmovenetworkplayer_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_019637_cmoveped_setfacialnetwork_rage_fwmovenetworkplayer_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_019638_cmoveped_dtorcmoveped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019646_cmoveped_setmotiontasknetwork_rage_fwmovenetworkplayer_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_019649_cmoveped_playvisemeclip_rage_crclip_const_ptr_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_019650_cmoveped_playhandanimation_rage_crclip_const_ptr_float_cmoveped(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_019666_cmoveped_switchtostaticframe_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019667_cmoveped_switchtoanimated_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_019685_cmovevehicle_dtorcmovevehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019686_cmoveobject_dtorcmoveobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019690_cmoveobject_setsecondarynetwork_rage_fwmovenetworkplayer_ptr_flo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019697_cmoveobject_setnetwork_rage_fwmovenetworkplayer_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_019737_cclipeventtags_cfirstpersoncameraeventtag_getblendinduration_con(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_019738_cclipeventtags_cfirstpersoncameraeventtag_getblendoutduration_co(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_019741_cclipeventtags_cmotionblureventtag_motionblurdata_clear(ph: *mut Phantom); }
extern "C" { pub fn f_019745_cdebugclipdictionary_streamin(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019746_cdebugclipdictionary_loadanimnames_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019747_rage_atvector_rage_atstring_0_unsigned_short_growcapacity_unsign(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019749_cdebugclipdictionary_loadclipdictionaries_bool(ph: *mut Phantom); }
extern "C" { pub fn f_019750_void_std_2_introsort_std_2_classicalgpolicy_animdictpred_ref_cde(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_019751_cdebugclipdictionary_findlocaldirectoryfordictionary_char_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019753_cdebugselector_addwidgets_rage_bkbank_ptr_char_const_ptr_rage_da(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_019756_cdebugselector_select_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019757_cdebugselector_updatecombo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019765_cdebuganimsceneentityselector_rebuildentitylist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019769_canimscenesectioniterator_findnextsection(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019770_cdebugclipselector_dtorcdebugclipselector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019772_cdebugclipselector_onclipcontextselected(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019773_cdebugclipselector_onclipdictionaryselected(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019776_cdebugclipselector_loadclipsetnames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019778_cdebugclipselector_loadclipsetclipnames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019779_cdebugclipselector_loadclipdictionarynames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019781_cdebugclipselector_loadclipdictionaryclipnames(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019782_cdebugclipselector_selectclip_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_019783_cdebugclipselector_getselectedclip_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019784_cdebugclipselector_addwidgets_rage_bkgroup_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_019785_cdebugclipselector_removewidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019787_buildclipsetclipnamesmap_rage_fwmvtemplateid_rage_fwmvclipsetidt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019788_cdebugclipselector_onclipsetselected(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019789_cdebugclipselector_onlocalfilechanged(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019790_cdebugclipselector_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019794_cdebugtimelinecontrol_setupslider_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019796_cdebugobjectselector_streamselected_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019797_cdebugobjectselector_getselectedmodelid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019802_cdebugpedmodelselector_loadmodelnames(ph: *mut Phantom); }
extern "C" { pub fn f_019804_cdebugpedmodelselector_selectmodel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019807_cclipfileiterator_iterateclips_char_const_ptr_char_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_019822_csyncedsceneentitymanager_save_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_019823_csyncedsceneentitymanager_load_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_019825_canimplacementeditor_addanimplacementtool_rage_fwmodelid_rage_ma(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019826_canimplacementtool_startanim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019827_csyncedsceneentitymanager_createattachedwidgets_unsigned_int_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_019828_canimplacementeditor_addanimplacementtoolobject_rage_fwmodelid_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019829_canimplacementeditor_addanimplacementtoolvehicle_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019830_canimplacementeditor_addanimplacementtoolattachedprop_rage_fwmod(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019831_rage_atarray_canimplacementtool_0_unsigned_short_grow_unsigned_i(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_019832_canimplacementtool_op_canimplacementtool_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019833_canimplacementtool_initialise(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019834_canimplacementtool_cleanupentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019835_canimplacementtool_setobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019836_canimplacementtool_createped_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019837_canimplacementtool_canimplacementtool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019841_canimplacementeditor_rebuildbank(ph: *mut Phantom); }
extern "C" { pub fn f_019842_canimplacementeditor_removewidgets(ph: *mut Phantom); }
extern "C" { pub fn f_019843_canimplacementeditor_deletetool_canimplacementtool_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019845_canimplacementtool_removewidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019846_canimplacementeditor_updatelocationwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_019847_canimplacementeditor_scenefolderchanged(ph: *mut Phantom); }
extern "C" { pub fn f_019855_canimplacementeditor_selectcameraanim(ph: *mut Phantom); }
extern "C" { pub fn f_019864_cdebugobjectselector_addwidgets_rage_bkbank_ptr_char_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_019872_rage_atarray_rage_atstring_0_unsigned_short_resizegrow_unsigned(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_019882_rage_atvector_canimviewer_spmdictionary_0_unsigned_short_destruc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019899_canimviewer_sdebugexpressions_dtorsdebugexpressions(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019907_cdebugfileselector_dtorcdebugfileselector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019912_canimviewer_initbonenames(ph: *mut Phantom); }
extern "C" { pub fn f_019914_canimviewer_initspringnames(ph: *mut Phantom); }
extern "C" { pub fn f_019915_canimviewer_selectspring(ph: *mut Phantom); }
extern "C" { pub fn f_019916_cleardofwidgets_rage_bkgroup_ref_canimviewer_springdof_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019917_canimviewer_populatespringwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_019918_canimviewer_initmodelnamecombo(ph: *mut Phantom); }
extern "C" { pub fn f_019922_canimviewer_toggleuninterruptabletask(ph: *mut Phantom); }
extern "C" { pub fn f_019924_canimclipeditor_shutdownmainwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_019926_cgestureeditor_shutdowngesturetool(ph: *mut Phantom); }
extern "C" { pub fn f_019927_canimviewer_blendoutdata_dtorblendoutdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019928_canimviewer_findmovepreviewtask(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_019929_canimviewer_setselectedentity_cdynamicentity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019930_canimviewer_focusentitychanged_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019932_canimviewer_rebuildparamwidgetoptions(ph: *mut Phantom); }
extern "C" { pub fn f_019933_cgestureeditor_selectfromspeakermodelcombo(ph: *mut Phantom); }
extern "C" { pub fn f_019934_cgestureeditor_selectfromlistenermodelcombo(ph: *mut Phantom); }
extern "C" { pub fn f_019935_cgestureeditor_updategesturetool(ph: *mut Phantom); }
extern "C" { pub fn f_019936_canimviewer_adddebugskeleton_cdynamicentity_ref_rage_color32_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_019937_canimviewer_sdebugexpressions_updateexpressiondebugwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019938_canimviewer_sdebugexpressions_removeexpressiondebugwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_019939_canimviewer_updatedoftrackingforekg(ph: *mut Phantom); }
extern "C" { pub fn f_019940_canimclipeditor_loadselectedclipfromdisk(ph: *mut Phantom); }
extern "C" { pub fn f_019941_canimclipeditor_verifytagswithincliprange_rage_crclip_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_019943_canimviewer_rendermovertrack_rage_crclip_const_ptr_rage_matrix34(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019945_rage_matrix34_dotfromleft_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019946_rage_matrix34_dottranspose_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019947_canimviewer_renderskeleton_rage_crskeleton_const_ref_rage_color3(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_019948_rage_imp_inverttransformfull_imp34_rage_mat34v_ref_float_vector(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: u128, a4: i64); }
extern "C" { pub fn f_019951_rage_atvector_canimviewer_sanimdictsummary_0_unsigned_short_grow(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_019952_void_std_2_introsort_std_2_classicalgpolicy_animnamegroupsizenam(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_019953_void_std_2_introsort_std_2_classicalgpolicy_animnamegroupsizesiz(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_019955_canimviewer_dumpposeprint_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019981_canimviewer_setphase(ph: *mut Phantom); }
extern "C" { pub fn f_020032_canimviewer_blendoutdata_addwidgets_rage_bkgroup_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020042_canimviewer_removecurrentpreviewnetwork(ph: *mut Phantom); }
extern "C" { pub fn f_020063_centitycapture_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020064_canimviewer_playforwardsanim(ph: *mut Phantom); }
extern "C" { pub fn f_020065_canimviewer_playbackwardsanim(ph: *mut Phantom); }
extern "C" { pub fn f_020067_dumpanimationwrtselectedentity_cdynamicentity_ptr_rage_cranimati(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020081_canimviewer_onselectedblendoutsetlistchanged_canimviewer_blendou(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020082_canimviewer_sdebugexpressions_addexpressiondebugwidgets_cdynamic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_020083_rage_imp_untransformortho_imp34_rage_mat34v_ref_float_vector_4_f(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: u128, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_020084_canimviewer_blendoutdata_onmatchchanged(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020085_rage_atbinarymap_float_unsigned_long_insert_unsigned_long_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_020086_canimviewer_onrecalculatematch_canimviewer_blendoutdata_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020088_canimviewer_onselectedblendoutsetchanged_canimviewer_blendoutdat(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020092_canimviewer_onselectedblendoutchanged_canimviewer_blendoutdata_p(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020094_canimviewer_addtrackikroot_rage_cranimation_ptr_rage_atstring_re(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_020096_rage_crproperties_findproperty_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_020100_fwmotiontreevisualiser_rendertext_char_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_020105_frameeditorwidgets_addwidgets_rage_bkgroup_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020106_resetwidgetiterator_iteratedof_rage_crframedata_dof_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_020107_centitycapture_dtorcentitycapture(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020116_canimviewer_sdebugexpressions_resetexpressiondebugwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020121_canimclipeditor_startpreviewclip(ph: *mut Phantom); }
extern "C" { pub fn f_020148_canimclipeditor_updatepropertiescollectionwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_020149_canimclipeditor_previewnexttag_rage_fwmvtemplateid_rage_fwmvprop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020150_canimclipeditor_previewprevioustag_rage_fwmvtemplateid_rage_fwmv(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020151_canimclipeditor_updatetagwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_020152_canimclipeditor_clearallmoveeventtagsbutton_rage_atnonfinalhashs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020153_canimclipeditor_generateuseleftfootstrafetransitiontags_rage_crc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_020154_canimclipeditor_generateuseleftfootstrafetransitiontagsfordictio(ph: *mut Phantom); }
extern "C" { pub fn f_020155_canimclipeditor_generateblocktransitiontags_rage_crclip_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_020159_canimclipeditor_addpropertywidgets_rage_crproperty_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020168_canimclipeditor_clearallmoveeventtags_rage_crclip_ptr_rage_atnon(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020169_rage_atarray_rage_crtag_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020170_rage_crtagiteratorbase_rage_crtagiteratorkey_true_getprev(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020206_cliptagproperty_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_020209_cliptagaction_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_020212_cliptagactiongroup_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_020247_cliptagaction_dtorcliptagaction(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020262_cgestureeditor_selectfromspeakercontextcombo(ph: *mut Phantom); }
extern "C" { pub fn f_020265_cgestureeditor_selectfromlistenercontextcombo(ph: *mut Phantom); }
extern "C" { pub fn f_020267_cgestureeditor_selectfromcameracombo(ph: *mut Phantom); }
extern "C" { pub fn f_020268_cgestureeditor_triggerbodygesture_cped_ptr_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: f32, a10: i32, a11: i32); }
extern "C" { pub fn f_020269_cgestureeditor_triggerfacialgesture_cped_ptr_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_020270_canimscenesection_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_020273_canimsceneeventlist_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_020277_canimscene_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_020280_canimscenematrix_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_020281_canimsceneentity_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_020283_canimscene_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020292_canimsceneeventlist_dtorcanimsceneeventlist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020294_canimscene_calcsectionstarttime_canimscenesection_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_020295_canimsceneevent_init_canimscene_ptr_canimscenesection_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_020296_canimscenesection_shutdownevents_canimscene_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020297_canimsceneevent_onexitpreloadinternal_canimscene_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_020298_canimsceneevent_onenterinternal_canimscene_ptr_animsceneupdateph(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_020299_canimsceneevent_onupdateinternal_canimscene_ptr_animsceneupdatep(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_020300_canimsceneevent_onexitinternal_canimscene_ptr_animsceneupdatepha(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_020301_canimsceneevent_shutdown_canimscene_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020302_canimscenesection_preloadevents_canimscene_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_020303_canimsceneevent_onenterpreloadinternal_canimscene_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_020304_canimsceneevent_onupdatepreloadinternal_canimscene_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_020305_canimscenesection_processevents_canimscene_ptr_float_float_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_020306_canimscenesection_isvalidid_rage_atnonfinalhashstring_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_020307_canimscenesection_getstart_canimscene_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> f32; }
extern "C" { pub fn f_020308_canimsceneeventlist_sceneeditorrender_canimscene_ptr_canimscenee(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_020313_canimscene_addsection_rage_atnonfinalhashstring_canimscenesectio(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_020314_canimscenematrix_setmatrix_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020315_canimscene_canimscene_rage_atnonfinalhashstring_eanimsceneflagsb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_020316_canimscene_dtorcanimscene(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020317_canimscene_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020318_canimscenematrix_dtorcanimscenematrix(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020319_canimscene_shutdownfsm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020320_canimscene_shutdownentitiesandevents(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020322_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_020323_canimscene_recalculatescenedurations(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020324_canimscene_preloadassets_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020325_canimscene_skipping_onexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020326_canimscene_getparentmatrix_rage_mat34v_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020327_canimscene_update_animsceneupdatephase(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_020328_canimscene_loadmetadata_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020329_canimscene_preloadassets_onenter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020330_canimscene_setstate_canimscene_state(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_020331_canimsceneentityiterator_findfirstentity_int_ref_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_020332_canimsceneeventiterator_canimsceneevent_findnextevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020333_canimsceneevent_startofscene_canimscene_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020334_canimscene_getentity_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_020335_canimscene_addentity_rage_atnonfinalhashstring_canimsceneentity(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_020336_canimscene_findsectionfortime_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_020337_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_020338_canimscene_addplaybacklist_rage_atnonfinalhashstring_canimscenep(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_020339_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_020341_canimsceneped_createdebugpedifrequired(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020342_canimsceneobject_createdebugobjifrequired(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020343_canimscenevehicle_createdebugvehifrequired(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020345_canimscene_calcentitylocationdata_rage_atnonfinalhashstring_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_020346_canimsceneeventiterator_canimsceneplayanimevent_findnextevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020347_canimsceneentityhandle_updateentityid_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020348_canimsceneplayanimevent_calcmoveroffset_canimscene_ptr_rage_mat3(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_020349_canimscene_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020350_canimscene_setplaybacklist_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020351_canimscenemanager_getfilenamefromscenename_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020352_canimscene_save_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_020353_canimsceneentity_setlocationdata_rage_atnonfinalhashstring_anims(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_020354_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_020355_canimscene_detachfromentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020356_canimscene_getscenematrix(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020357_canimscenemanager_destroyanimscene_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_020358_canimscene_setpaused_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_020359_canimscene_getpaused_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_020360_canimscene_addsectiontoplaybacklist_rage_atnonfinalhashstring_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_020361_canimscene_removesectionfromplaybacklist_rage_atnonfinalhashstri(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_020362_canimsceneeventlist_deleteevent_canimsceneevent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020363_canimsceneevent_countchildren(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_020364_canimscene_isentityinscene_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_020365_canimscene_deletesection_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020383_canimsceneevent_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_020389_canimsceneevent_notifyrangeupdated(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020398_canimsceneeventlist_shutdownwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020399_rage_atarray_rage_functor0_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020419_canimsceneclip_addclipwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020446_canimsceneeditor_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020449_canimscenemanager_createanimscene_char_const_ptr_eanimsceneflags(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_020455_canimscenemanager_loadscenenamelist(ph: *mut Phantom); }
extern "C" { pub fn f_020456_canimsceneeditor_widgets_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020457_canimscenemanager_initsceneeditor_canimscene_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020459_canimscenemanager_iteratescenesondisk_char_const_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020466_canimsceneeditor_setscreenextents_cscreenextents_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020472_canimsceneeditor_regenentitytypewidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020478_canimsceneeditor_selectevent_canimsceneevent_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020479_canimsceneeditor_regeneventtypewidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020480_canimsceneeditor_clearselectionwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020482_cdebugclipsetselector_addwidgets_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_020483_canimsceneped_ptr_canimscene_getentity_canimsceneped_rage_atnonf(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_020484_canimsceneped_setped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020485_conscreentimeline_movemarkertovisibletimelineposition_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_020486_canimsceneeditor_selectentity_canimsceneentity_ptr_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020487_canimsceneeditor_isdragging(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_020488_canimscenescrollbar_processmouseinput(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020489_rendertextwithbackground_char_const_ptr_float_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i64, a5: f32, a6: i32, a7: i64); }
extern "C" { pub fn f_020490_canimscenescrollbar_computebarproperties(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020491_canimscenescrollbar_renderhorizontal(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020492_canimscenescrollbar_rendervertical(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020493_canimscenemanager_getscenedescription_canimscene_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020494_gettextwidth_char_const_ptr(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_020495_rendertext_char_const_ptr_float_float_rage_color32_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64); }
extern "C" { pub fn f_020501_conscreenbutton_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020502_cauthoringhelperdisplay_drawbuttonbackground_cscreenextents_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_020503_cauthoringhelperdisplay_setcanvassize_cscreenextents_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020511_conscreenvideocontrol_setcanvassize_cscreenextents_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020512_conscreenvideocontrol_drawbuttonbackground_cscreenextents_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_020513_ceventpanelgroup_dtorceventpanelgroup(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020515_ceventpanelgroup_render_canimsceneeditor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020516_ceventlayerspane_dtorceventlayerspane(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020518_canimsceneeditor_widgets_dtorwidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_020534_canimsceneeditor_createeventandaddtocurrentscene_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_020605_canimsceneobject_setobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_020609_canimscenevehicle_setvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_020710_canimsceneruntimedata_canimsceneobject_widgets_op(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_020975_rage_atbinarymap_float_unsigned_long_datapair_ptr_std_2_partial(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_020976_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_020977_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_021008_rage_atbinarymap_int_unsigned_int_datapair_ptr_std_2_partial_sor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_021009_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_021010_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_021011_animdictpred_op_cdebugclipdictionary_ptr_cdebugclipdictionary_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_021012_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_animdictpr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_021013_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_021014_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_animnamegr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_021015_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_animnamegr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_021016_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_animnamegr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_021017_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_021018_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_021019_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_animnamegr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_021020_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_021021_canimsceneeventiterator_canimsceneevent_findnextevent_canimscene(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_021022_canimsceneeventiterator_canimsceneplayanimevent_findnextevent_ca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_021222_audsuperconductor_sendconductormessage_conductors_conductormessa(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021235_audscannermanager_initsentenceforscanner_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_021236_audscannermanager_getnextscannerslot_audscannersentence_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021237_audmusiceventmanager_triggerevent_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_022023_rage_fwentity_getskeletondata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022235_cpedinventory_getweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022615_audscene_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022852_audfrontendaudioentity_playsound_rage_atnonfinalhashstring_char(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023998_rage_fwmodelid_fwmodelid_rage_fwmodelid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_024286_creplaymgrinternal_isplaybackflagset_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_026174_camgameplaydirector_getfirstpersonshootercamera_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026182_camgameplaydirector_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_026190_caminterface_fadeout_int_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026195_caminterface_fadein_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_026200_caminterface_isfading(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026205_cqueriableinterface_istaskcurrentlyrunning_int_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_026219_cdynamicentity_getboneindexfrombonetag_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_026222_cambaseobject_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026223_camfirstpersonshootercamera_getrelativeheading_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_026224_camfirstpersonshootercamera_getrelativepitch_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_026259_rage_fwentity_getskeleton_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026367_rage_imp_transform_imp34_rage_mat34v_ref_float_vector_4_float_ve(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: u128, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_026623_rage_matrix34_toeulers_rage_vector3_ref_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_026648_camdebugdirector_getcamframe_cambasecamera_ptr_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026649_camdebugdirector_getcamframenonconst_cambasecamera_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027036_rage_matrix34_isclose_rage_matrix34_const_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_027037_cdebugclipselector_cdebugclipselector_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_027304_camsyncedscenedirector_stopanimatingcamera_camsyncedscenedirecto(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_027305_camsyncedscenedirector_animatecamera_rage_atnonfinalhashstring_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_028262_cviewportmanager_pushviewport_cviewport_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_028263_cviewportmanager_popviewport(ph: *mut Phantom); }
extern "C" { pub fn f_029331_cdebugscene_drawentityboundingbox_centity_ptr_rage_color32_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_029433_getorcreategroup_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029455_cdebugscene_tunewidgetdata_tunewidgetdata_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_029456_cdebugscene_queuetunewidget_cdebugscene_tunewidgetdata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029469_cdebugscene_draw3dboundingbox_rage_vector3_const_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_029612_worldprobe_cshapetestdesc_setresultsstructure_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029613_worldprobe_cshapetestresults_freeintersections(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029627_cauthoringhelper_update_rage_matrix34_const_ref_float_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_029711_void_std_2_introsort_std_2_classicalgpolicy_alphabeticalsort_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_029726_camframe_setworldmatrix_rage_matrix34_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_029866_cdebugscene_focusentities_get_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029934_cdebugdrawstore_render(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029967_cphysical_gethealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_029968_cdebugscene_getentitydescription_centity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029979_cauthoringhelper_cauthoringhelper_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_029980_cauthoringhelper_dtorcauthoringhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029983_cauthoringhelper_update_rage_matrix34_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029986_cuigadgetlist_cuigadgetlist_float_float_float_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i64) -> i64; }
extern "C" { pub fn f_029987_cuigadgetwindow_cuigadgetwindow_float_float_float_float_cuicolor(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64) -> i64; }
extern "C" { pub fn f_029988_cuigadgetlist_setnumentries_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_030052_cuigadgetbase_attachchild_cuigadgetbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030415_cuicolorscheme_setallcolors_rage_color32_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030675_csettingsmanager_dtorcsettingsmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030722_ceventgroupped_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030744_ceventnetwork_spewtotty_impl_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030756_ceventhandlerhistory_getcurrentevent_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030766_ceventnames_geteventname_eeventtype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_030773_cshockingeventsmanager_clearallevents_bool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_030774_cshockingeventsmanager_clearalleventswithsourceentity_centity_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_030775_cshockingeventsmanager_clearallblockingareas(ph: *mut Phantom); }
extern "C" { pub fn f_030783_ceventdecisionmakermanager_getdecisionmaker_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_031069_ceventgivepedtask_ceventgivepedtask_int_rage_aitask_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_031070_ceventgivepedtask_dtorceventgivepedtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035040_chudcolour_getrgb_ehud_colours_unsigned_char(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_035062_cminimap_getisinsideinterior_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035079_chudcolour_getrgba_ehud_colours(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_035082_cnewhud_isweaponwheelactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035083_cnewhud_isradiowheelactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035304_cminimap_switchoffwaypoint_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035307_cminimap_getblippositionvalue_cminimapblip_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_035324_cminimap_getblip_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_035356_cminimap_getactivewaypoint_swaypointstruct_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035420_cminimap_renderthread_candisplaygpsline_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036034_cscaleformmoviewrapper_isactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_036035_cscaleformmoviewrapper_render_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_036056_cscaleformmgr_addparamstring_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_036058_cscaleformmoviewrapper_createmovie_escaleform_base_class_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_036064_cscaleformmoviewrapper_removemovie(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_036184_cpausemenu_togglepauserenderphases_bool_enum_pause_menu_pausedrp(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_036199_cscaleformmoviewrapper_callmethod_char_const_ptr_cscaleformmovie(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_036215_cscaleformmoviewrapper_beginmethod_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_036216_cscaleformmoviewrapper_endmethod(ph: *mut Phantom); }
extern "C" { pub fn f_036254_cpausemenu_isactive_pm_activeflags(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_036281_cuireticule_reset(ph: *mut Phantom); }
extern "C" { pub fn f_036326_cpausemenu_isingalleryscreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036377_cpausemenu_isinmapscreen(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_036640_cscaleformmoviewrapper_addparamlocstring_char_const_ptr_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037262_cscaleformmoviewrapper_cscaleformmoviewrapper_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_037263_cscaleformmoviewrapper_dtorcscaleformmoviewrapper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037438_cfrontendstatsmgr_shouldusemetric(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_037657_cwanted_setwantedlevel_rage_vector3_const_ref_int_int_setwantedl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_037660_cmodelinfo_getassetstreamflags_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037661_cmodelinfo_requestassets_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037662_cmodelinfo_haveassetsloaded_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037681_cclock_passtime_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_037764_cwanted_workoutpolicepresence_rage_vec3v_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_037769_cwanted_getwithinamnestytime_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037888_cseatmanager_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_037901_cwanted_copsaresearching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037902_cwanted_gettimesincesearchlastrefocused_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037903_corder_removeorderfromentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_037908_cwanted_gettimesincelastspotted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038297_croadblock_trytodespawn(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038364_cincidentmanager_clearallincidents_bool(ph: *mut Phantom); }
extern "C" { pub fn f_038366_cincidentmanager_clearallincidentsoftype_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_038372_cordermanager_clearallorders_bool(ph: *mut Phantom); }
extern "C" { pub fn f_038394_croadblock_disperse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038467_cbaseikmanager_getsolver_ikmanagersolvertypes_flags_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038717_cmodelinfo_removeassets_rage_fwmodelid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038720_cmodelinfo_clearassetrequiredflag_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_038733_centryexitpoint_getseat_seataccess_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_038744_cmodelseatinfo_calculateseatsituation_centity_const_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_038751_cseatmanager_getpedsseatindex_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038757_cseatmanager_getlastpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_038863_cpedmodelinfo_getpersonalitybravery_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_039572_cdoor_settargetdoorratio_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_039627_cobject_initanimlazy(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039688_cobjectpopulation_destroyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039718_cobject_gettask_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039719_cobject_settask_rage_aitask_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_039761_cobjectpopulation_createobject_rage_fwmodelid_eentityownedby_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_039888_cpathserver_addnavmeshregion_rage_navmeshrequiredregion_rage_scr(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_039890_cpathserver_removenavmeshregion_rage_navmeshrequiredregion_rage(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_039891_cpathserver_areallnavmeshregionsloaded_rage_ainavdomain(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_039985_cpathserver_testshortlineofsightimmediate_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_040157_cpedgroupmembership_countmembers_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040194_cpedgroups_getpedsgroup_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040218_cassistedmovementroutestore_process_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041056_findplayerped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041060_cpeddebugvisualisermenu_getfocusped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041063_cpedintelligence_isfriendlywith_cped_const_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_041064_cpedintelligence_isthreatenedby_cped_const_ref_bool_bool_bool_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_041076_cpedtargetting_getcurrenttarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041081_cpedmotivation_getmotivation_cpedmotivation_pedmotivationstate_c(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_041082_cpedmotivation_getmotivationratemodifier_cpedmotivation_pedmotiv(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_041084_cpedtargetting_gettargetatindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041085_cpedtargetting_findtarget_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041087_cplayerinfo_getpreviousvariationinfo_unsigned_int_ref_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_041088_cpedintelligence_getnearbyvehicles(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041089_cplayerinfo_getstealthmultiplier_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041116_cpedintelligence_flushimmediately_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041117_cpedintelligence_addtoeventhistory_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041118_cpedpopulation_removeallpedshardnotplayer(ph: *mut Phantom); }
extern "C" { pub fn f_041132_cpeddebugvisualisermenu_createbank(ph: *mut Phantom); }
extern "C" { pub fn f_041195_cpedintelligence_cleartasks_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041197_cpedintelligence_setdefaultrelationshipgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041203_cpedintelligence_getrelationshipgroup_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041223_cpedtargetting_aretargetswhereaboutsknown_rage_vector3_ptr_centi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_041244_cplayerspecialabilitymanager_chargeevent_abilitychargeeventtype(ph: *mut Phantom, a0: i32, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_041271_cpoolhelpers_getpedpoolsize(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041281_cpedtype_getpedtypenamefromid_epedtype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_041296_cpedfactory_destroyped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041371_cpediksettingsinfo_isikflagdisabled_ikmanagersolvertypes_flags_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041385_cpedintelligence_findtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041397_cpedintelligence_clearsecondarytask_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041428_cpedintelligence_getlastbattleeventtime_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041469_cpedpopulation_initvaluesfromconfig(ph: *mut Phantom); }
extern "C" { pub fn f_041496_findplayervehicle_cplayerinfo_ptr_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041532_cpedpopulation_removeallpedshardnotmission(ph: *mut Phantom); }
extern "C" { pub fn f_041600_cplayerspecialability_getremainingnormalized_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041650_cplayerinfo_getplayerstatinfoforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041651_cplayerinfo_getrunsprintspeedmultiplier_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041653_cplayerinfo_getmaxtimeunderwaterforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_041654_cplayerinfo_computedurationfromstat_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32) -> f32; }
extern "C" { pub fn f_041668_cplayerpedtargeting_findlockontarget(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041670_cplayerpedtargeting_shouldupdatetargeting_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041700_cplayerinfo_getpos(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041705_cplayerinfo_setplayerstate_cplayerinfo_state(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041715_cplayerinfo_affectedbywatercannon(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041726_cplayerinfo_getplayermeleedamagemodifier_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_041746_reportplayerresetmismatch_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_041864_cpedaccuracy_calculateaccuracyrange_cped_const_ref_cweaponinfo_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64); }
extern "C" { pub fn f_041921_cplayerpedtargeting_getweaponinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042040_crelationshipmanager_findrelationshipgroup_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_042299_cpedcomprestriction_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042301_cpedproprestriction_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_042404_cpedvariationpack_getlocalcompdata_cped_const_ptr_epedvarcomp_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_042521_cpedvariationdebug_updatepedlist(ph: *mut Phantom); }
extern "C" { pub fn f_042523_cpedvariationdebug_pedidchangecb(ph: *mut Phantom); }
extern "C" { pub fn f_042524_refreshvarset(ph: *mut Phantom); }
extern "C" { pub fn f_042533_cpedpropsmgr_widgetupdate(ph: *mut Phantom); }
extern "C" { pub fn f_042818_worldprobe_cshapetestmanager_debugsubmittest_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_042821_worldprobe_cshapetestmanager_aborttest_worldprobe_cshapetestresu(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042878_cpickupmanager_removeallpickups_bool_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_043046_ccollisionhistory_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043049_ccollisionhistory_getmostsignificantcollisionrecordoftype_unsign(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_043077_cbuoyancy_getbuoyancyinfo_cphysical_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_043085_worldprobe_cshapetestdesc_setexcludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043164_phmaterialdebug_userrenderbound_rage_phinst_ptr_rage_phbound_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_043224_cphysics_loadaboutposition_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043333_cwatertesthelper_getcachedriverboundproberesult_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_043485_water_iscameraunderwater(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_044518_casynclightocclusionmgr_addlight(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_045796_cped_computedefaulttask_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045797_cped_getcurrentmotiontask_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045802_cped_setcharparamsbasedonmanagertype(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045842_cped_forcemotionstatethisframe_cpedmotionstates_emotionstate_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_045844_cped_setdefaultdecisionmaker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045874_cped_isonground_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045875_cped_setusinglowlodphysics_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045876_cped_setuseextractedz_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045889_cped_getprimarymotiontask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045903_cped_getiscrouching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045908_cped_isplayingagestureanim_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045911_cped_wantstouseactionmode_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045914_cped_setspeakerlistenedto_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045923_cped_blockgestures_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045929_cped_blendingesture_rage_crclip_const_ptr_rage_fwmvtemplateid_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32); }
extern "C" { pub fn f_045940_cped_setvariation_epedvarcomp_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_045944_cped_isbonelightlyarmoured_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045977_cped_setdesiredheading_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_045996_cped_getbonetagfromragdollcomponent_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046103_cped_getspecialability_eplayerspecialabilityslot(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046105_cped_getblockingofnontemporaryeventsthisframe_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046146_cped_randomlysay_char_const_ptr_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_046168_cped_isprone_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046172_cped_getwaterlevelonped_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_046184_cped_getiksettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046186_cped_resurrect_rage_vector3_const_ref_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_046192_cped_isdebuginvincible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046213_cped_isactionmodereasonactive_cped_actionmodeenabled_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046221_cped_isbattleeventblockingactionmodeidle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046225_cped_gethighheelexpressiondof(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_046226_cped_getfirstpersoncameradofs_rage_vec3v_ref_rage_quatv_ref_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64) -> i32; }
extern "C" { pub fn f_046227_cped_getconstrainthelperdofs_bool_float_ref_rage_vec3v_ref_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_046302_cdynamicentity_createskeleton(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046303_cdynamicentity_createanimdirector_rage_rmcdrawable_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_046305_cdynamicentity_addtosceneupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046327_cplayerswitchinterface_isactive_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_046438_cgameworld_add_centity_ptr_rage_fwinteriorlocation_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046445_cportaltracker_collectstack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046459_cdynamicentity_getscriptthatcreatedme_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046467_cgameworld_remove_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046473_centity_getisinexterior_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046499_centity_getcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046539_centity_getisvisibleinfo_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046545_centity_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046702_cwarpmanager_setwarp_rage_vector3_const_ref_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: f32, a6: i32) -> i32; }
extern "C" { pub fn f_046994_cfileloader_loadline_void_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047020_cgameworld_findlocalplayerwanted(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047044_cgameworld_forallentitiesintersecting_rage_fwsearchvolume_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_047060_cphysical_activatephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047069_cphysical_calculateangvelocityfrommatrices_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_047073_cphysical_sethealth_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047133_cphysical_setdontloadcollision_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047305_cgameworld_getmainplayerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047306_cwarpmanager_stopwarp(ph: *mut Phantom); }
extern "C" { pub fn f_047725_cinteriorinst_issubwaymlo_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047880_cgameworld_findlocalplayercoors(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047970_cgameworld_findobjectsinrange_rage_vector3_const_ref_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_047972_cgameworld_findobjectsintersectingcube_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_047973_cgameworld_clearexcitingstufffromarea_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_047982_cgameworld_findlocalplayervehicle_bool(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_047993_cgameworld_clearculpritfromallcarfires_centity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047995_cgameworld_changepedmodel_cped_ref_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_047998_cgameworld_findlocalplayerspeed(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_048037_cgameworldheightmap_getminheightfromworldheightmap_float_float(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_054930_network_commands_commandnetworkplayergetname_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_056555_object_commands_commandsetnearestbuildinganimrate_rage_scrvector(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_056557_object_commands_commandsetnearestbuildinganimphase_rage_scrvecto(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_056606_object_commands_commandcleargarage_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_056607_object_commands_commandclearobjectsfromgarage_int_bool_bool_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_056609_object_commands_commandenablesavingingarage_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_063798_cthescripts_prioritizerequestsfrommissionscript_int_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063824_cscripteulers_matrixtoeulers_rage_matrix34_const_ref_rage_eulera(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_063864_cscriptangledarea_set_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64); }
extern "C" { pub fn f_063865_cscriptangledarea_testpoint_rage_vector3_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_063866_cscriptangledarea_debugdraw_rage_color32_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_063900_cscriptentities_clearspaceformissionentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063925_cdebugattachmenttool_createwidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_063932_cdebugattachmenttool_scriptattachped(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063933_cdebugattachmenttool_scriptattachother(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063934_cdebugattachmenttool_getscriptcall(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_063937_cdebugattachmenttool_initialise_cphysical_ptr_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_063977_cscriptdebugpedhandler_createscriptped_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063978_cscriptdebugvehiclehandler_createscriptvehicle_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063979_cscriptdebugobjecthandler_createscriptobject_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_064026_cscripthud_getfakedgpsplayerpos_rage_vector3_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064736_csettingsmanager_csettingsmanager(ph: *mut Phantom); }
extern "C" { pub fn f_064737_csettingsmanager_getsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064800_cstreaming_hasobjectloaded_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064832_cstreaming_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_064845_cstreaming_isobjectinimage_rage_strlocalindex_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064847_cprioritizedclipsetstreamer_updatememorycostforclipdictionary_ra(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064855_cprioritizedclipsetrequestmanager_shouldclipsetbestreamedout_cpr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_064857_cprioritizedclipsetrequestmanager_isloaded_rage_fwmvtemplateid_r(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064913_ckeyboard_getkeyjustdown_int_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_064933_cstreaming_requestobject_rage_strlocalindex_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_065288_ccompaniondata_clearroutenodecount_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_065293_ccompaniondata_getinstance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_065367_ccontrol_stopplayerpadshaking_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_065384_ccontrol_isinputdisabled_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_065446_ccontrolmgr_getmainplayercontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065615_cfilemgr_openfile_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_065617_cfilemgr_setdir_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065621_cfilemgr_openfileforwriting_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065622_cfilemgr_openfileforappending_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065808_ctaskrunnamedclip_ctaskrunnamedclip_rage_atnonfinalhashstring_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_065870_ctaskpreviewmovenetwork_addwidgets_rage_bkbank_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_065871_ctaskpreviewmovenetwork_removewidgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_065872_ctaskpreviewmovenetwork_applyclipset_rage_fwmvtemplateid_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_065875_ctasksynchronizedscene_ctasksynchronizedscene_short_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: f32, a5: f32, a6: i64, a7: i64, a8: f32, a9: i32, a10: i64) -> i64; }
extern "C" { pub fn f_065949_ctaskscriptedanimation_ctaskscriptedanimation_ctaskscriptedanima(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_065950_ctaskscriptedanimation_ctaskscriptedanimation_rage_atnonfinalhas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32, a6: f32, a7: i32, a8: i64, a9: f32, a10: i32, a11: i32, a12: i64, a13: i32, a14: i32) -> i64; }
extern "C" { pub fn f_065951_ctaskscriptedanimation_ctaskscriptedanimation_rage_atnonfinalhas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32, a6: f32, a7: i32, a8: i64, a9: f32, a10: i64, a11: i64, a12: i32, a13: i32, a14: i64, a15: i32, a16: i32) -> i64; }
extern "C" { pub fn f_067931_ctaskplayeronfoot_ctaskplayeronfoot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067983_ctaskmobilephone_ctaskmobilephone_int_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_068126_ctaskdonothing_ctaskdonothing_int_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_068161_ctasksequencelist_ctasksequencelist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068163_ctasksequencelist_addtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068203_ctaskusesequence_ctaskusesequence_ctasksequencelist_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_068454_ctaskmotionbase_setonfootclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_068456_ctaskmotionbase_clearoverrideweaponclipset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068467_ctaskmotionbase_getdefaultonfootclipset_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_068510_ctaskmotionswimming_clearscubagearvariationforped_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068512_ctaskmotionswimming_isscubagearvariationactiveforped_cped_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068929_ctaskgetup_ctaskgetup_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_069039_ctaskfollowwaypointrecording_ctaskfollowwaypointrecording_cwaypo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_069049_ctaskfollowwaypointrecording_setpause_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_069050_ctaskfollowwaypointrecording_setresume_bool_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_069051_ctaskfollowwaypointrecording_setoverridemoveblendratio_float_boo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_069052_ctaskfollowwaypointrecording_usedefaultmoveblendratio(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069053_ctaskfollowwaypointrecording_startaimingatentity_centity_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_069054_ctaskfollowwaypointrecording_startshootingatentity_centity_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_069055_ctaskfollowwaypointrecording_stopaimingorshooting(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069529_ctaskparachute_clearparachutepackvariationforped_cped_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_069530_ctaskparachute_findparachutepackvariationforped_cped_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_069531_ctaskparachute_isparachutepackvariationactiveforped_cped_const_r(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069534_ctaskparachute_gettintindexforparachutepack_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_069543_ctaskparachute_getmodelforparachute_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069606_ctaskparachute_getmodelforparachutepack_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_069607_ctaskparachute_gettintindexforparachute_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_070361_cnmblendoutsetlist_cnmblendoutsetlist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_070362_cnmblendoutsetlist_dtorcnmblendoutsetlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070363_cnmblendoutsetlist_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_070364_cnmblendoutsetlist_add_rage_atnonfinalhashstring_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_070367_cnmblendoutsetlist_findposeitem_rage_fwmvtemplateid_rage_fwmvcli(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_070774_ctasknmshot_getshottypename_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_072812_cscenarioinfomanager_clearpedusedscenarios(ph: *mut Phantom); }
extern "C" { pub fn f_073132_cscenariocondition_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_074161_ctask_ctask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_074163_ctaskclassinfomanager_gettaskname_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_075349_cmovenetworkhelper_getfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_075350_ccliprequesthelper_ccliprequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075351_ccliprequesthelper_dtorccliprequesthelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075353_ccliprequesthelper_requestclipsbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075389_cnavmeshroutesearchhelper_cnavmeshroutesearchhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075398_cvehiclefollowroutehelper_getvehiclebonnetposition_cvehicle_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_075442_cnavmeshlineofsighthelper_cnavmeshlineofsighthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075446_casyncprobehelper_dtorcasyncprobehelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075447_casyncprobehelper_resetprobe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075448_casyncprobehelper_starttestlineofsight_worldprobe_cshapetestprob(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075450_casyncprobehelper_getproberesultswithintersection_probestatus_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_075456_ccliphelper_dtorccliphelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075462_cmovenetworkhelper_setclip_rage_crclip_const_ptr_rage_fwmvtempla(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075463_ccliphelper_startclipbysetandclip_cdynamicentity_ptr_rage_fwmvte(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32, a7: f32, a8: i32) -> i32; }
extern "C" { pub fn f_075464_ccliphelper_startclipbyclip_cdynamicentity_ptr_rage_crclip_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_075470_cmovenetworkhelper_cmovenetworkhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075471_csimpleblendhelper_dtorcsimpleblendhelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075472_cmovenetworkhelper_dtorcmovenetworkhelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075473_cmovenetworkhelper_createnetworkplayer_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_075474_cmovenetworkhelper_setfloat_rage_fwmvtemplateid_rage_fwmvfloatid(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_075476_csimpleblendhelper_findclipbyindexandhash_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_075477_csimpleblendhelper_getnumactiveclips(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_075478_csimpleblendhelper_update_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_075479_cmovenetworkhelper_releasenetworkplayer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075480_csimpleblendhelper_updatepostmotiontree(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075481_csimpleblendhelper_updatepreoutputbufferflip(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075485_cgenericcliphelper_dtorcgenericcliphelper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075486_cgenericcliphelper_blendinclipbysetandclip_cdynamicentity_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_075487_cgenericcliphelper_blendinclipbyclip_cdynamicentity_ptr_rage_crc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_075489_cgenericcliphelper_blendoutclip_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_075490_cgenericcliphelper_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075503_cmovenetworkhelper_setboolean_rage_fwmvtemplateid_rage_fwmvboole(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075604_ctaskcontrolvehicle_ctaskcontrolvehicle_cvehicle_ptr_rage_aitask(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_075758_ctaskvehiclefsm_getpedinorusingseat_cvehicle_const_ref_int_cped(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_075769_ctaskcontrolvehicle_setdesiredsubtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075871_ctaskvehiclefsm_arepedsfriendly_cped_const_ptr_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_076337_ctaskvehiclemountedweapon_getvehicletargetingflags_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_076338_ctaskvehiclemountedweapon_findtarget_cvehicle_ptr_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: i64, a8: i64, a9: i64) -> i64; }
extern "C" { pub fn f_077089_cstatsmgr_incrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077090_cstatsmgr_decrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077091_cstatsmgr_setgreater_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077093_cstatsmgr_clearplayingtimestats(ph: *mut Phantom); }
extern "C" { pub fn f_077094_cstatsmgr_endvehiclestats(ph: *mut Phantom); }
extern "C" { pub fn f_077095_cstatsmgr_isplayeractive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077153_statid_char_getstatid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077193_cstatsdatamgr_iskeyvalid_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077329_statid_set_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077331_statsinterface_getintstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077338_statsinterface_getcurrentmultiplayercharaterslot(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077342_statsinterface_setstatdata_statid_const_ref_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_077348_statsinterface_setstatdata_statid_const_ref_unsigned_long_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_077353_statsinterface_trymultiplayersave_esavetypes_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_077359_statsinterface_getuint64stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077373_statsinterface_getstatsmodelhashid_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077381_statsinterface_applypedscardata_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077394_cstatsutils_isstatstrackingenabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077475_cnumberwithinmessage_set_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077476_cnumberwithinmessage_set_float_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077479_ctextfile_get_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077488_cmessages_insertnumbersandsubstringsintostring_char_const_ptr_cn(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32); }
extern "C" { pub fn f_077495_chelpmessage_clearall(ph: *mut Phantom); }
extern "C" { pub fn f_077496_cmessages_clearmessages(ph: *mut Phantom); }
extern "C" { pub fn f_077504_cmessages_addmessage_char_const_ptr_int_unsigned_int_bool_bool_e(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i64, a9: i32, a10: i32, a11: i32); }
extern "C" { pub fn f_077508_chelpmessage_clear_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_077509_chelpmessage_setmessagetextandaddtobrief_int_char_const_ptr_cnum(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077533_ctextlayout_ctextlayout(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077535_ctextlayout_dtorctextlayout(ph: *mut Phantom); }
extern "C" { pub fn f_077537_ctextlayout_setscale_rage_vector2_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077539_ctextlayout_getcharacterheight(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_077540_ctextlayout_render_rage_vector2_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_077544_ctextformat_getstringwidth_ctextlayout_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_077549_ctext_flush(ph: *mut Phantom); }
extern "C" { pub fn f_077791_cpathfind_checkpathnodeintegrityforzone_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077792_cpathfind_testalllinks_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077800_cvehicleintelligence_getroutesearchhelper_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077801_cvehicleintelligence_getnodelist_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077802_cvehicleintelligence_getfollowroutehelper_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077803_cvehicleintelligence_getactivetask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077812_cpathfind_findnodeclosesttocoors_rage_vector3_const_ref_float_cp(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32, a7: f32, a8: f32, a9: f32, a10: f32, a11: i32, a12: i64, a13: i32, a14: i32); }
extern "C" { pub fn f_077827_cjunctions_getjunctionbyindex_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077832_cvehiclenodelist_findlastgoodnode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077846_cpathfind_releaserequestednodes_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077847_cpathfind_findrouteendnodeforgpssearch_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_077848_cpathfind_dopathsearch_rage_vector3_const_ref_rage_cnodeaddress(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: f32, a8: i64, a9: i32, a10: i32, a11: i64, a12: i32, a13: i32, a14: i32, a15: i32, a16: i32, a17: i32, a18: i32, a19: i32, a20: i32, a21: i64, a22: i32, a23: i64, a24: i64, a25: i32, a26: i32); }
extern "C" { pub fn f_077855_cpathfind_findrouteendnodeforgpssearchasync_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_077857_cpathfind_cleargpssearchslot_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_077858_cpathfind_generateroutepointsforgps_rage_cnodeaddress_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32, a8: i64, a9: i32, a10: i32, a11: i64, a12: i32, a13: i32, a14: i32) -> f32; }
extern "C" { pub fn f_077859_cpathfind_generateroutepointsforgps_rage_cnodeaddress_rage_cnode(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i32, a11: i64, a12: i32, a13: i32, a14: i64, a15: i32, a16: i32, a17: i32) -> f32; }
extern "C" { pub fn f_077870_cpathfind_findnearestgroundheightforbuildpaths_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_077874_cpathfind_findroadboundaries_int_int_float_bool_bool_float_ref_f(ph: *mut Phantom, a0: i32, a1: i32, a2: f32, a3: i32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_077876_cpathfind_makerequestfornodestobeloaded_float_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_077905_cpathfind_haverequestednodesbeenloaded_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077958_cvehicleintelligence_getdrivingflagname_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077964_cvehicleintelligence_findtargetcoorswithnode_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32, a8: i64); }
extern "C" { pub fn f_077968_cvehicleintelligence_getactivetasksimplest_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077969_cvehicleintelligence_setrecordingnumber_signed_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078229_ctaskvehiclefollowrecording_ctaskvehiclefollowrecording_svehicle(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_078233_ctaskvehiclefollowwaypointrecording_ctaskvehiclefollowwaypointre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_078786_cvehicle_getnumcontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078801_cautomobile_placeonroadproperly_cautomobile_ptr_rage_matrix34_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: i32, a8: i64, a9: i64, a10: i32, a11: f32, a12: f32) -> i32; }
extern "C" { pub fn f_078909_cvehicle_istaximodelid_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078924_cvehicle_getwheelfromid_ehierarchyid(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_078939_cvehicle_switchengineon_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_078971_cvehicledebug_getfocusvehicle(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_078976_cvehicledamage_getstucktimer_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_079007_cvehicle_getnumberofpassenger_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079023_cvehicle_getattachedtrailer_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079038_cvehicle_getlastdriver_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079040_cvehicle_getspecialflightmoderatio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_079071_cvehicle_isentryindexvalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079076_cvehicle_resetsuspension(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079092_cwheel_updatecontactsafternetworkblend_rage_matrix34_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079098_cvehicle_selectappropriategearforspeed(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079134_cvehicle_getdriverseat_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079145_cbike_placeonroadproperly_cbike_ptr_rage_matrix34_ptr_cinteriori(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64, a7: i32, a8: i64, a9: i64, a10: i32, a11: f32, a12: f32, a13: i32) -> i32; }
extern "C" { pub fn f_079152_cvehicle_updategadgetsafterteleport_rage_matrix34_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_079228_cvehicle_updatebodycolourremapping_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079253_ccomponentreservationmanager_getseatreservation_int_seataccess_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_079254_ccomponentreservationmanager_getdoorreservation_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079335_chandlingdata_gethandlingdata_ehandlingtype(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079409_cheli_removepolicehelisupondeatharrest(ph: *mut Phantom); }
extern "C" { pub fn f_079614_ctrailer_debugattachtoparentvehicle_char_const_ptr_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: f32, a7: i32) -> i32; }
extern "C" { pub fn f_079665_ctrain_settrackposfromworldpos_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079764_cvehicle_trytomakefromdummyincludingparents_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079780_cvehicle_haspedsinit_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079813_cvehicle_getdoorfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079815_cvehicle_getseatanimationinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_079845_cvehicle_ispersonalvehicle(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079885_cvehicle_getentryexitpoint_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079886_cvehicle_getnumberentryexitpoints_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079899_cvehicle_getrelativevelocity_cped_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079939_cvehiclepopulation_clearinterestingvehicles_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_079948_cpoolhelpers_getvehiclepoolsize(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_080258_cvehicleweaponmgr_getweaponsandturretsforseat_int_rage_atarray_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_080380_cvehiclepopulation_initvaluesfromconfig(ph: *mut Phantom); }
extern "C" { pub fn f_080432_cvehiclepopulation_setcoordsofscriptcar_cvehicle_ptr_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_080445_cvehiclepopulation_removeallvehssoft(ph: *mut Phantom); }
extern "C" { pub fn f_080470_cstoredvehicle_storevehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080471_cstoredvehicle_restorevehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080472_cstoredvehicle_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080473_cstoredvehicle_isused(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080540_cvehiclemetadatamgr_getinvehicleoverrideinfo_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_080715_cinvehicleoverrideinfo_findseatoverrideaniminfofromseataniminfo(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_080870_cvfxhelper_getmatrixfromboneindex_lights_rage_mat34v_ref_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_080872_cvfxhelper_creatematfromvecyalign_rage_mat34v_ref_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_080888_cvfxhelper_shouldrenderingameui(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_081415_cbrightlights_register_brightlighttype_e_rage_vec3v_const_ref_ra(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: f32, a7: f32, a8: i32, a9: f32); }
extern "C" { pub fn f_081459_cfiremanager_clearculpritfromallfires_centity_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082064_cpedweaponmanager_getequippedvehicleweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082065_cpedweaponmanager_getequippedweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082070_cweaponinfo_getammoinfo_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_082071_cpedweaponmanager_getequippedweapon_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082116_cexplosionmanager_disablecrimereportingforexplosions_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082215_cpedweaponmanager_dropweapon_unsigned_int_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_082549_cweaponinfomanager_getslotlistnavigate_cped_const_ptr(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_082580_cpedinventory_addweaponandammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_082583_cpedinventory_setweaponammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082594_cpedweaponmanager_equipweapon_unsigned_int_int_bool_bool_cpedequ(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_082642_cpedweaponselector_setselectedweapon_cped_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_082663_cammoitemrepository_getammo_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082671_cweaponitem_getkeyfromhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082721_cprojectilemanager_removealltriggeredpedprojectiles_centity_cons(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082727_cprojectilemanager_disablecrimereportingforexplosions_centity_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082831_cnetworktelemetry_flushtelemetry_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082909_cnetworktelemetry_appendmetric_rage_rlmetric_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083028_networkinterface_isinspectatormode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083038_networkinterface_isinfreemode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083061_networkinterface_getnetworkobject_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083068_networkinterface_areplayersindifferenttutorialsessions_cnetgamep(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083070_networkinterface_getspectatorobjectid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083201_networkinterface_writedatapopulationlog_char_const_ptr_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_083898_csyncedentity_csyncedentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083899_csyncedentity_csyncedentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083900_csyncedentity_setentity_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083901_csyncedentity_csyncedentity_csyncedentity_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083902_csyncedentity_getentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083908_csyncedvehicle_getvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083937_cnetworkvehiclemodeldoorlockedtablemanager_isvehiclemodellocked(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083939_cnetworkvehiclemodeldoorlockedtablemanager_isvehicleinstanceregi(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083941_cnetworkvehiclemodeldoorlockedtablemanager_isvehicleinstancelock(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_084338_cnetworktelemetry_playerspawn_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_086744_cnetgameplayer_getplayerped_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086763_cnetobjgame_getplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087642_cnetobjplayer_canrespawnlocalplayerped(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087643_cnetobjplayer_respawnplayerped_rage_vector3_const_ref_bool_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_087752_cnetobjvehicle_islockedforplayer_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_089369_cweapondamageevent_cleardelayedkill(ph: *mut Phantom); }
extern "C" { pub fn f_089711_cgarageoccupiedstatusevent_trigger_int_unsigned_int_unsigned_cha(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090710_asinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090712_atan2(ph: *mut Phantom, a0: f64, a1: f64) -> f64; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090714_atanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090717_atof(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090719_isspace(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090734_coshf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090741_time(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090757_expf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090795_isalnum(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090801_localtime(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090805_log10f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090807_logf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090814_memcmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090826_powf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090885_pthread_mutex_destroy(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090886_pthread_mutex_init(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090887_pthread_mutex_lock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090889_pthread_mutex_unlock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090890_pthread_mutexattr_settype(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090901_qsort_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_090911_rand(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090918_sinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090919_sinhf(ph: *mut Phantom, a0: f32) -> f32; }
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
extern "C" { pub fn f_090942_strncmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090945_strrchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090947_strstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090955_strtod(ph: *mut Phantom, a0: i64, a1: i64) -> f64; }
extern "C" { pub fn f_090956_strtok(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090959_strtoll(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_090965_tanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090966_tanhf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090971_toupper(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090985_vsnprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090987_vsprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_091002_emscripten_builtin_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091023_multi3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
