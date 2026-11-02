//! Callee declarations for crate gen001.
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
extern "C" { pub fn f_000115_rage_sysipcpollsema_rage_sysipcsematag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000116_rage_sysipcwaitsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000118_rage_sysipcwaitsematimed_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_000119_rage_sysipcsignalsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000120_rage_sysipccreatemutex(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_000121_rage_sysipclockmutexcommand_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000122_rage_sysipcunlockmutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000123_rage_sysipcdeletemutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000127_rage_sysipcsetevent_rage_sysipceventtag_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000128_rage_sysipccreatethread_void_ptr_void_ptr_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_000131_rage_sysipcwaitthreadexit_rage_sysipcthreadid_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000134_rage_sysipcyield_int(ph: *mut Phantom); }
extern "C" { pub fn f_000135_rage_sysipcsleep_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000147_rage_fidevicelocal_read_fihandle_ptr_void_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_000150_rage_fidevicelocal_write_fihandle_ptr_void_const_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_000151_rage_fidevicelocal_seek_fihandle_ptr_int_rage_fiseekwhence_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_000152_rage_fidevicelocal_seek64_fihandle_ptr_long_rage_fiseekwhence_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000153_rage_fidevicelocal_close_fihandle_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000176_rage_wasm_announce_set_low_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000197_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom); }
extern "C" { pub fn f_000212_rage_fidevicetcpip_connect_char_const_ptr_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000239_rage_grcdevice_manage(ph: *mut Phantom); }
extern "C" { pub fn f_000242_rage_grcdevice_gethasfocus(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_000244_wasm_d3d_map_hint(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_000393_wasm_null_d3d_nullresource_wasm_null_d3d_stub_id3d11texture3d_d3(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000644_rage_athashnextsize_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000649_rage_dattypestruct_addfield_unsigned_long_unsigned_long_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000660_rage_atstring_indexof_char_const_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000663_rage_atstring_replace_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000671_rage_atstring_split_rage_atarray_rage_atstring_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000676_rage_op_rage_functorbase_const_ref_rage_functorbase_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000680_rage_athashstringnamespacesupport_computehash_rage_athashstringn(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_000681_rage_athashstringnamespacesupport_addstring_rage_athashstringnam(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_000682_rage_atmapentry_unsigned_int_char_const_ptr_const_ref_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000684_rage_athashstringnamespacesupport_trygetstring_rage_athashstring(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_000690_rage_bkwidget_bkwidget_rage_datcallback_ref_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_000692_rage_datcallback_call_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000696_rage_bkwidget_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000697_rage_bkbank_dtorbkbank(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000703_rage_bkbank_unsetcurrentgroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000704_rage_bkbank_pushgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000705_rage_bkgroup_addgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000706_rage_bkbank_deletegroup_rage_bkgroup_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000707_rage_bkbank_popgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000720_rage_bkwidget_addchild_rage_bkwidget_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000782_rage_bkmanager_findbank_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000785_rage_bkmanager_destroybank_rage_bkbank_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000797_rage_bkwidget_setfillcolor_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000806_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000815_rage_bkmessagebox_char_const_ptr_char_const_ptr_rage_bkmessagebo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_000851_char_ptr_rage_formatf_char_8ul_char_ref_8ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000866_rage_bkgroup_addseparator_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_000867_rage_bkgroup_addtitle_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000868_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_000869_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000870_rage_bkgroup_addbutton_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000872_rage_bkgroup_addtext_char_const_ptr_float_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_000874_rage_bkgroup_addtext_char_const_ptr_int_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000877_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_000882_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000886_rage_bkgroup_addtoggle_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000894_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000901_rage_bkgroup_addslider_char_const_ptr_unsigned_short_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_000904_rage_bkgroup_addslider_char_const_ptr_short_ptr_short_short_shor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000906_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000907_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int_rage_d(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000918_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i32) -> i64; }
extern "C" { pub fn f_000919_rage_bkgroup_addcombo_char_const_ptr_int_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_000927_rage_bkgroup_addvector_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000941_rage_bkgroup_addcolor_char_const_ptr_rage_color32_ptr_rage_datca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_000945_rage_bkgroup_addcolor_char_const_ptr_rage_vector4_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000946_rage_bkgroup_addcolor_char_const_ptr_rage_vector3_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000990_char_ptr_rage_formatf_char_512ul_char_ref_512ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001100_rage_autoiddescriptor_autoiddescriptor_unsigned_int_char_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
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
extern "C" { pub fn f_001128_rage_datgrowbuffer_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001129_rage_datgrowbuffer_getfihandle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001131_rage_datgrowbuffer_preallocate_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001138_rage_datgrowbuffer_getbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001140_rage_datbitbuffer_readunsigned_void_const_ptr_unsigned_int_ref_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001141_rage_datbitbuffer_writeunsigned_void_ptr_unsigned_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_001142_rage_datbitbuffer_copybits_void_ptr_void_const_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_001143_rage_datbitbuffer_datbitbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001144_rage_datbitbuffer_setreadonlybits_void_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001145_rage_datbitbuffer_setreadwritebits_void_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001146_rage_datbitbuffer_getreadwritebits(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001147_rage_datbitbuffer_setreadonlybytes_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001148_rage_datbitbuffer_setreadwritebytes_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001149_rage_datbitbuffer_getnumbyteswritten_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001150_rage_datbitbuffer_setnumbitswritten_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001151_rage_datbitbuffer_setcursorpos_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001152_rage_datbitbuffer_getnumbytesread_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001153_rage_datbitbuffer_setnumbitsread_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001155_rage_datbitbuffer_readbits_void_ptr_int_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001156_rage_datbitbuffer_bumpreadcursor_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001157_rage_datbitbuffer_readbits_rage_datbitbuffer_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001158_rage_datbitbuffer_bumpreadwritecursor_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001159_rage_datbitbuffer_writebits_void_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001160_rage_datbitbuffer_writebits_rage_datbitbuffer_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001161_rage_datbitbuffer_readuns_unsigned_long_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001163_rage_datbitbuffer_writeuns_unsigned_long_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001166_rage_datbitbuffer_readbool_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001167_rage_datbitbuffer_writebool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001168_rage_datbitbuffer_readbytes_void_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001169_rage_datbitbuffer_writebytes_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001176_rage_datresourceinfo_generatemap_rage_datresourcemap_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001186_rage_rijndael_setup_unsigned_char_const_ptr_int_int_rage_symmetr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001187_rage_rijndael_ecb_encrypt_unsigned_char_ptr_rage_symmetric_key_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001189_rage_rijndael_ecb_decrypt_mem_void_ptr_unsigned_int_rage_symmetr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001190_rage_aes_aes_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001191_rage_aes_aes_unsigned_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001193_rage_aes_encrypt_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001195_rage_aes_decrypt_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001196_rage_sha1_transform_unsigned_int_ptr_unsigned_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001198_rage_sha1_clear_unsigned_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001199_rage_sha1_final_unsigned_char_ref_20(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001202_rage_pkcs5_pbkdf2_char_const_ptr_unsigned_int_char_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001203_rage_datgetdigest_unsigned_char_const_ptr_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
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
extern "C" { pub fn f_001548_rage_diaggetadjustedsystemtime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_001551_rage_fprintf_rage_fistream_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001553_rage_firemoteshowmessagebox_char_const_ptr_char_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001559_rage_diagcontextmessage_diagcontextmessage_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001560_rage_diagcontextmessage_diagcontextmessage_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001562_rage_diagcontextmessage_diagcontextmessage_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_001563_rage_diagcontextmessage_pop(ph: *mut Phantom); }
extern "C" { pub fn f_001565_printf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001568_rage_atmapentry_unsigned_int_wchar_t_ptr_const_ref_rage_atmap_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001569_rage_atmap_unsigned_int_wchar_t_ptr_rage_atmaphashfn_unsigned_in(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001576_rage_fiassetmanager_fullpath_char_ptr_int_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_001577_rage_fistream_open_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001578_rage_fistream_seek_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001580_rage_fiassetmanager_open_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001582_rage_fiassetmanager_createleadingpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001599_rage_atmapentry_rage_sysipcthreadid_ptr_thread_closure_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001605_rage_fidevice_makegrowbufferfilename_char_ptr_int_rage_datgrowbu(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001615_rage_restservicemanager_removeanddeleteservice_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001622_rage_fistream_read_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001623_rage_fiassetmanager_findextensioninpath_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001626_rage_fiassetmanager_addextension_char_ptr_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001627_rage_fiassetmanager_exists_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001632_rage_fiassetmanager_pushfolder_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001633_rage_fiassetmanager_popfolder(ph: *mut Phantom); }
extern "C" { pub fn f_001635_rage_fiassetmanager_filename_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001636_rage_fiassetmanager_basename_char_ptr_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001637_rage_fiassetmanager_enumfiles_char_const_ptr_void_ptr_rage_fifin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_001639_rage_fiassetmanager_removeextensionfrompath_char_ptr_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_001642_rage_fistream_open_char_const_ptr_rage_fidevice_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001643_rage_fistream_create_char_const_ptr_rage_fidevice_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001644_rage_fistream_getch(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001645_rage_fistream_putch_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001650_rage_fgetline_char_ptr_int_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_001652_rage_fidevice_makememoryfilename_char_ptr_int_void_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_001653_shutdownwinsock(ph: *mut Phantom); }
extern "C" { pub fn f_001654_rage_fiserialize_fiserialize_rage_fistream_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_001656_rage_fibasetokenizer_init_char_const_ptr_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001667_rage_fiasciitokenizer_getint_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001669_rage_fiasciitokenizer_getfloat_bool(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_001673_rage_fibasetokenizer_gettoken_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001682_rage_fibasetokenizer_skiptoendofline(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001764_rage_atarray_rage_atbinarymap_int_rage_atfinalhashstring_datapai(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001880_rage_fipackfile_fipackfile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001881_rage_fipackfile_init_char_const_ptr_bool_rage_fipackfile_cachemo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_001882_rage_fipackfile_reinit_char_const_ptr_bool_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_001922_rage_clientrequestandwaitreplyex_fihandle_ptr_rage_remoteany_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i64, a8: i32); }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002029_rage_vec_v4float16streamunpack_float_vector_4_ptr_float_vector_4(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002031_rage_mthrandom_mthrandom_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002038_rage_pgstreamer_open_char_const_ptr_unsigned_int_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002039_rage_rscbuilder_signal_callback_void_ptr_void_ptr_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_002041_rage_pgstreamer_read_unsigned_int_rage_datresourcechunk_ptr_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64) -> i64; }
extern "C" { pub fn f_002042_rage_pgrscbuilder_freememory_rage_datresourcemap_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002043_rage_pgstreamer_close_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_002044_rage_pgrscbuilder_computeleafsize_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002045_rage_wasm_announce_entry_unsigned_int_unsigned_int_unsigned_long(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_002046_rage_pgrscbuilder_cleanup_rage_datresourcemap_ref_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002047_rage_pgrscbuilder_loadbuild_char_const_ptr_char_const_ptr_int_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_002063_rage_pgstreamer_read_unsigned_int_rage_datresourcechunk_ptr_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_002064_rage_pgstreamer_unlock(ph: *mut Phantom); }
extern "C" { pub fn f_002065_rage_ggetrawstreamer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002066_rage_pgrawstreamerinvalidateentry_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_002076_rage_pgstreamer_getsize_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_002080_rage_pgbase_pgbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002081_rage_pgbase_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002082_rage_pgbase_regeneratemap_rage_datresourcemap_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002083_rage_pgbase_validate_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002084_rage_pgbase_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002088_rage_datresource_datresource_rage_datresourcemap_const_ref_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002090_rage_datresource_dtordatresource(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002091_rage_datresource_containsthisaddress_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002092_rage_datresource_getfixup_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002094_rage_pgbaseplatformneutral_pgbaseplatformneutral(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002105_rage_pgbase_dtorpgbase_3250(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002129_rage_atmapentry_rage_atnonfinalhashstring_rage_atnonfinalhashstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002130_rage_atmap_rage_atnonfinalhashstring_rage_atnonfinalhashstring_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002145_rage_parmanager_loadtree_rage_fistream_ptr_rage_parsettings_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002147_rage_parmanager_loadtree_char_const_ptr_char_const_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002150_rage_parmanager_savetree_char_const_ptr_char_const_ptr_rage_part(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_002155_rage_parmanager_findstructure_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002157_rage_parinstancevisitor_visitstructure_rage_parparsablestructure(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002158_rage_parinstancevisitor_visittoplevelstructure_rage_parparsables(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002160_rage_parstructure_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002162_rage_parmanager_loadfromstructure_rage_fistream_ptr_rage_parstru(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002164_rage_parmanager_loadpatchfile_char_const_ptr_char_const_ptr_rage(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002168_rage_parmanager_createandloadanytype_rage_fistream_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_002175_rage_partree_dtorpartree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002176_rage_partree_createroot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002182_rage_parelement_findattribute_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002184_rage_partreenode_findchildwithname_char_const_ptr_rage_partreeno(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002188_rage_partreenode_appendaschildof_rage_partreenode_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002191_rage_parelement_addattribute_char_const_ptr_double_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f64); }
extern "C" { pub fn f_002194_rage_parelement_addattribute_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_002201_rage_parelement_addattribute_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_002227_rage_psoschemacatalog_findstructureschema_rage_atnamespacedhashv(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002241_rage_psoresourcedata_copyschemadatato_rage_psoresourcedata_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002242_rage_atmapentry_rage_atstring_rage_psoresourcedata_ptr_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002271_rage_psoloadfromstructure_rage_psostruct_ref_rage_parstructure_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_002277_rage_psofile_directfixupmatchingpsocontents(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002278_rage_psofile_dtorpsofile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002283_rage_psoloadfilefrombuffer_char_ptr_int_char_const_ptr_rage_psol(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_002285_rage_psoinitfromresource_rage_psoresourcedata_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002289_rage_psocreateandloadfromstructure_rage_psofile_ref_rage_parstru(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002295_rage_atarray_rage_psorscbuilderinstanceset_ptr_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002296_rage_atmap_unsigned_int_rage_atarray_rage_psorscbuilderinstances(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002297_char_const_ptr_ref_rage_atmap_unsigned_int_char_const_ptr_rage_a(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002302_rage_psoresourcedata_place_rage_psoresourcedata_ptr_rage_datreso(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002311_rage_atarray_rage_psoenumtableentry_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002343_rage_parrestregistersingleton_char_const_ptr_rage_parparsablestr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002397_rage_parutils_physicalfree_char_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002402_rage_parenumdata_namefromvalueunsafe_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002436_rage_parcguregisterstructure_rage_parstructure_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002437_rage_parcgustructure_createstructure_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002438_rage_parcgustructure_setfactories_rage_parstructure_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002439_rage_parcgustructure_setgetstructurecb_rage_parstructure_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002440_rage_parcgustructure_setbaseclass_rage_parstructure_ptr_rage_par(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002444_rage_parcgustructure_buildstructurefromstaticdata_void_ptr_rage(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002447_rage_parinstancevisitor_parinstancevisitor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002581_rage_parelement_setname_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002690_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_002691_rage_psorscbuilderinstance_ptr_ptr_std_2_partial_sort_impl_abi_n(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002692_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002693_rage_psorscbuilderinstance_ptr_ptr_std_2_partition_with_equals_o(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002695_rage_atbinarymap_rage_pardelegateholderbase_ptr_unsigned_int_dat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002696_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002697_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_002719_strlwr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002720_rage_conststring_vcopy_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002721_rage_conststring_dtorconststring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002725_rage_atstringbuilder_atstringbuilder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002727_rage_atstringbuilder_atstringbuilder_char_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002728_rage_atstringbuilder_dtoratstringbuilder(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002731_rage_atstringbuilder_append_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002736_rage_atstringbuilder_tostring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002739_rage_atpartialstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002743_rage_atdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002746_rage_stristr_char_ptr_char_const_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002755_rage_stringcopytruncatefront_char_ptr_char_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002759_rage_utf8towidechar_char_const_ptr_unsigned_short_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002790_rage_zlibinflater_zlibinflater(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002802_rage_syshangdetectsavezoneenter(ph: *mut Phantom); }
extern "C" { pub fn f_002803_rage_syshangdetectsavezoneexit_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002808_rage_atdelegator1_void_rage_sysserviceevent_ptr_removedelegate_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002810_rage_syshangdetecttick(ph: *mut Phantom); }
extern "C" { pub fn f_002811_rage_syshangdetectsetcrashfunc_void_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002812_rage_sysnamedpipe_sysnamedpipe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002813_rage_sysnamedpipe_open_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002814_rage_sysnamedpipe_open_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002815_rage_sysnamedpipe_read_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002816_rage_sysnamedpipe_saferead_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002817_rage_sysnamedpipe_write_void_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002818_rage_sysnamedpipe_safewrite_void_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002819_rage_sysnamedpipe_close(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002820_rage_sysnamedpipe_setnodelay_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002839_rage_sysparam_get_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002841_rage_sysparam_getarray_char_const_ptr_ptr_int_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_002849_rage_sysbootmanager_isbootedfromdisc(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002862_rage_systhreadtype_addcurrentthreadtype_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_002863_rage_sysperformancetimer_sysperformancetimer_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002864_rage_sysperformancetimer_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002865_rage_sysperformancetimer_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002866_rage_sysperformancetimer_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002867_rage_sysperformancetimer_gettimems(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_002868_rage_sysperformancetimer_getelapsedtimems(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_002881_unsigned_int_ref_rage_atmap_unsigned_int_unsigned_int_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002886_rage_systhreadpool_thread_dtorthread(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002889_rage_systhreadpool_workitem_dtorworkitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002890_rage_systhreadpool_workitem_pending_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002895_rage_systhreadpool_workitem_wascanceled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002898_rage_systhreadpool_dtorsysthreadpool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002904_rage_systhreadpool_queuework_rage_systhreadpool_workitem_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002916_rage_sysservice_updateclass(ph: *mut Phantom); }
extern "C" { pub fn f_002917_rage_atdelegator1_void_rage_sysserviceevent_ptr_dispatch_rage_sy(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002927_rage_sysmemstarttemp(ph: *mut Phantom); }
extern "C" { pub fn f_002928_rage_sysmemendtemp(ph: *mut Phantom); }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_002932_rage_physical_delete_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002934_rage_sysmemvirtualallocate_unsigned_long_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003075_rage_systinyheap_init_void_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003076_rage_systinyheap_allocate_unsigned_long_unsigned_long_unsigned_l(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_003078_rage_systinyheap_free_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003085_rage_systaskparameters_systaskparameters(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003088_rage_systaskmanager_addscheduler_char_const_ptr_int_int_rage_sys(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_003090_rage_systaskmanager_create_char_const_ptr_void_ptr_rage_systaskp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_003093_rage_systaskmanager_poll_rage_systaskhandle_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003095_rage_systaskmanager_wait_rage_systaskhandle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003097_char_ptr_rage_formatf_char_16ul_char_ref_16ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003111_rage_sysdependencyscheduler_insertinternal_rage_sysdependency_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003142_rage_geomtvalues_findtvaluesegtopoint_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_003145_rage_geomsegments_collideraytriangle_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_003159_rage_matrix34_fromquaternion_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003160_rage_matrix34_toquaternion_rage_quaternion_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003179_rage_atpoolbase_init_unsigned_long_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003180_rage_atpoolbase_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003181_rage_atpoolbase_init_unsigned_char_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003182_rage_atpoolbase_releasemem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003185_rage_crframe_crframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003188_rage_crframe_crframe_rage_crframe_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003190_rage_crframe_exchangeframedata_rage_crframedata_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003192_rage_crframe_set_rage_crframe_const_ref_rage_crframefilter_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003208_rage_crframe_inversepose_rage_crskeleton_const_ref_bool_rage_crf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003221_rage_crframe_dtorcrframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003243_rage_crframefilter_registertypeinfo_rage_crframefilter_typeinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003244_rage_craccelerator_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003281_rage_cranimtrack_converttrackindextoname_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_003322_rage_crframefilter_crframefilter_rage_crframefilter_eframefilter(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_003323_rage_crframefilter_crframefilter_rage_datresource_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003325_rage_crframefilter_serialize_rage_datserialize_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003415_rage_craccelerator_lock_lock_rage_craccelerator_lock_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003428_rage_crframedatafactory_allocateframedata_rage_crcreature_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_003431_rage_crframedatafactory_releaseframedata_rage_crframedata_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003432_rage_crframedatafactory_deleteframedata_rage_crframedata_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003434_rage_crframebufferframedata_crframebufferframedata_rage_crcommon(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003435_rage_crframebufferframedata_dtorcrframebufferframedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003436_rage_crframebufferframedata_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003438_rage_crframebufferframedata_setframedata_rage_crframedata_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003439_rage_crframebufferframedata_allocateframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003440_rage_crframebufferframedata_releaseframe_rage_crframe_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003442_rage_crcommonpool_compactcallback_void_ptr_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003559_rage_crcreature_refresh(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003579_rage_crcreaturecomponent_factorypooled_createpool_rage_crcreatur(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_003598_rage_crcreaturecomponentskeleton_setskeleton_rage_crskeleton_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003738_rage_crcreaturecomponentphysical_resetmotions(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003799_rage_etracktype_tostring_rage_etracktype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_003806_rage_fwanimdirector_waitforanyactiveupdatetocomplete_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003813_rage_fwanimdirector_dtorfwanimdirector(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003830_rage_crtagiteratorbase_rage_crtagiteratorkey_true_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003837_rage_fwanimdirector_retrieveclip_unsigned_int_unsigned_int_unsig(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_003852_rage_fwanimdirector_createnetworkplayer_rage_mvnetworkdef_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003871_rage_fwanimdirectorcomponentsyncedscene_dumpsynchronizedscenedeb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003872_rage_fwanimdirectorcomponentsyncedscene_syncedscene_dtorsyncedsc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003875_rage_fwanimhelpers_getmovertracktranslation_rage_crclip_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003880_rage_fwanimhelpers_getmovertrackrotation_rage_crclip_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_003884_rage_fwanimhelpers_getmovertrackrotationdelta_rage_crclip_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003887_rage_fwanimhelpers_getmovertrackdisplacement_rage_crclip_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_003893_rage_fwanimhelpers_applyinitialoffsetfromclip_rage_crclip_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003974_rage_atarray_rage_atbinarymap_rage_fwclipitem_ptr_rage_fwmvtempl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003975_rage_fwclipset_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004017_rage_atarray_rage_fwmemorysituation_adjustment_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004113_rage_fwmovedumpnode_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_004144_rage_fwmovedumpnodeparent_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_004184_rage_fwmovedumpnodepair_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_004193_rage_fwmovedumpnoden_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_004220_rage_fwmovedumpnetwork_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_004227_rage_createmovedumpnode_rage_fwentity_const_ptr_rage_crmtnode_co(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004228_rage_fwmovedumpnoden_fwmovedumpnoden_rage_fwentity_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004229_rage_fwmovedumpnodepair_fwmovedumpnodepair_rage_fwentity_const_p(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004231_rage_fwmovedumpnode_dtorfwmovedumpnode_6501(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004232_rage_fwmovedumpnode_getdescription_rage_fwmovedumprenderflags_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32); }
extern "C" { pub fn f_004233_rage_varfixedstring_512u_varfixedstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_004238_rage_fixedstring_512u_replace_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004247_rage_fwmovedumpnodestate_dtorfwmovedumpnodestate(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004258_rage_fwmovedumpnetwork_dtorfwmovedumpnetwork(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004261_rage_fwmovedumpdof_fwmovedumpdof_unsigned_char_unsigned_short_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_004272_rage_fwmovedumpframesequence_dtorfwmovedumpframesequence(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004281_rage_fwposematcher_dtorfwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004289_rage_fwsyncedsceneentity_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_004294_rage_fwsyncedsceneentity_dtorfwsyncedsceneentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004300_rage_csyncedsceneinfo_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_004310_rage_atarray_rage_fwsyncedsceneentity_0_unsigned_short_pushandgr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004311_rage_atarray_rage_fwsyncedsceneentity_0_unsigned_short_copyfrom(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004312_rage_fwsyncedsceneentitymanager_dtorfwsyncedsceneentitymanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004360_rage_fwanimdirectorcomponentexpressions_dtorfwanimdirectorcompon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004369_rage_fwanimdirectorcomponentragdoll_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004385_rage_fwanimdirectorcomponentsyncedscene_syncedscene_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004387_rage_fwanimdirectorcomponentsyncedscene_stopsyncedsceneplayback(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_004388_rage_fwanimdirectorcomponentsyncedscene_updatesyncedscenemover(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004389_rage_fwanimdirectorcomponentfacialrig_facialnodecontroller_setcl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004390_rage_fwanimdirectorcomponentsyncedscene_decsyncedsceneref_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004395_rage_fwanimdirectorcomponentsyncedscene_calcentitymatrix_short_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: f32); }
extern "C" { pub fn f_004396_rage_fwanimdirectorcomponentsyncedscene_syncedscene_getrootmatri(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004424_rage_fwanimdirectorcomponentfacialrig_fwanimdirectorcomponentfac(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004425_rage_fwanimdirectorcomponentfacialrig_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004430_rage_fwanimdirectorcomponentfacialrig_dtorfwanimdirectorcomponen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004438_rage_fwanimdirectorcomponentfacialrig_facialnodecontroller_updat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004453_rage_fwmovedumpnodeclip_dtorfwmovedumpnodeclip(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004506_rage_fwclipset_dtorfwclipset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004562_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004563_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_004564_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_004576_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_004577_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004578_void_std_2_sort_heap_abi_nqn220108_std_2_classicalgpolicy_bool_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004579_rage_atbinarymap_rage_fwclipitem_ptr_rage_fwmvtemplateid_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_004580_std_2_pair_rage_atbinarymap_rage_fwclipitem_ptr_rage_fwmvtemplat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004581_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_004582_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004583_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_004585_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_004591_void_std_2_iterops_std_2_classicalgpolicy_iter_swap_abi_nqn22010(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004592_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_004594_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_004595_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_004609_rage_fwarchetypemanager_registerstreamedarchetype_rage_fwarchety(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004610_rage_fwarchetypemanager_registerpermanentarchetype_rage_fwarchet(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_004611_rage_fwarchetype_setclipdictionaryindex_rage_atnonfinalhashstrin(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004612_rage_fwarchetypemanager_dumpdynamicarchetypescomponents(ph: *mut Phantom); }
extern "C" { pub fn f_004613_rage_fwarchetype_setdrawdictfile_rage_atnonfinalhashstring_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_004614_rage_fwarchetype_setdrawableorfragmentfile_rage_atnonfinalhashst(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_004615_rage_fwarchetypemanager_poolfullcallback(ph: *mut Phantom); }
extern "C" { pub fn f_004616_rage_atmapentry_unsigned_int_rage_fwmodelid_const_ref_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004636_rage_fwarchetypemanager_getarchetypeindexfromhashkey_unsigned_in(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_004638_rage_fwarchetypemanager_freearchetypes_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_004640_rage_fwarchetypemanager_scheduleremovearchetypetotypefileref_rag(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004647_rage_fwattachmententityextension_dtorfwattachmententityextension(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004649_rage_fwattachmententityextension_notifychilddetachment_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004651_rage_fwattachmententityextension_setparentattachment_rage_fwenti(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004653_rage_fwattachmententityextension_getrequirespostprerender_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004661_rage_fwdynamicentitycomponent_fwdynamicentitycomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004683_rage_fwentity_getaabb_rage_spdaabb_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004686_rage_fwextensionlist_dtorfwextensionlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004695_rage_fwsceneupdate_removefromsceneupdate_rage_fwentity_ref_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_004697_rage_fwsceneupdate_runcustomcallbackbits_unsigned_int_void_ptr_r(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_004703_rage_atpagedpool_rage_fwquaterniontransform_false_rage_fwdefault(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004704_rage_atpagedpool_rage_fwmatrixtransform_false_rage_fwdefaultrscm(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004705_rage_atpagedpool_rage_fwsimpletransform_false_rage_fwdefaultrscm(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004708_rage_fwsimpletransform_op_new_unsigned_long(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004709_rage_fwtransform_delete(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004711_rage_fwmatrixtransform_op_new_unsigned_long(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004713_rage_fwquaterniontransform_op_new_unsigned_long(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004756_rage_fwdecoratorinterface_dtorfwdecoratorinterface(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004767_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004771_rage_fwpickermanager_onwidgetchange(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004783_rage_fwpickermanager_addentitytolist_rage_fwentity_const_ptr_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004788_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004792_rage_fwpickermanager_getselectedentity(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004818_rage_fwvectormap_drawpoly_rage_vector3_const_ref_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_004825_rage_fwvectormap_draw3dboundingboxonvmap_rage_spdaabb_const_ref(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004826_rage_fwvectormap_convertpointmaptoworld_float_float_float_ref_fl(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i64); }
extern "C" { pub fn f_004856_rage_atpagedpool_rage_fwsimpletransform_false_rage_fwdefaultrscm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004857_rage_atpagedpool_rage_fwsimpletransform_false_rage_fwdefaultrscm(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004858_rage_atpagedpool_rage_fwmatrixtransform_false_rage_fwdefaultrscm(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004859_rage_atpagedpool_rage_fwquaterniontransform_false_rage_fwdefault(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004860_rage_atpagedpool_rage_fwmatrixtransform_false_rage_fwdefaultrscm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004861_rage_atpagedpool_rage_fwquaterniontransform_false_rage_fwdefault(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004864_rage_atmap_unsigned_int_rage_fwmodelid_rage_atmaphashfn_unsigned(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004924_rage_dldrawcommandbuffer_allocatedrawlistpage_rage_edrawlistpage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_004925_rage_dldrawcommandbuffer_dumpdrawlistpages(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004926_rage_dldrawcommandbuffer_allocatepagedmemory_rage_edrawlistpaget(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64) -> i64; }
extern "C" { pub fn f_004927_rage_dldrawcommandbuffer_movetonewpage_rage_edrawlistpagetype_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_004928_rage_dldrawcommandbuffer_disassemblecommand_rage_dlcmdbase_ref_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_004930_char_ptr_rage_formatf_char_10ul_char_ref_10ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004934_rage_dldrawcommandbuffer_adddatablock_void_const_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_004935_rage_dlcmdbase_op_new_unsigned_long_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004936_rage_dldrawcommandbuffer_unlockreadbuffer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004937_rage_dldrawcommandbuffer_markpagesforrecycling_unsigned_int_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_004946_void_std_2_introsort_std_2_classicalgpolicy_int_ptr_ref_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_004947_char_ptr_rage_formatf_char_192ul_char_ref_192ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004948_rage_dlcmdnewdrawlist_dlcmdnewdrawlist_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004949_rage_dldrawlistmgr_getdrawlistname_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004961_rage_dlcmdgrcdeviceupdatebuffer_dopendingbufferupdates(ph: *mut Phantom); }
extern "C" { pub fn f_004986_rage_dldrawlistmgr_dtordldrawlistmgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004993_rage_getstatstr_int_int_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_004994_rage_dldrawlistmgr_addforcedcleanupcallback_void_ptr_void_ptr_un(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_004996_rage_dldrawlistmgr_endcurrentdrawlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004997_rage_dldrawlistmgr_begindrawlistcreation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_004998_rage_dldrawlistmgr_enddrawlistcreation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005001_void_rage_dlc_addinternal_void_ptr_bool_rage_dldrawlistmgr_ehigh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_005002_rage_dldrawlistmgr_removeall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005009_rage_dldrawlistmgr_addtypefilereference_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005011_rage_dldrawlistmgr_flipupdatefenceidx(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005012_rage_dldrawlistmgr_fliprenderfenceidx(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005018_rage_dldrawlistmgr_setuplodwidgets_rage_bkgroup_ref_int_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_005037_rage_atarray_char_0_unsigned_short_resizegrow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005063_rage_fwclothdebug_drawbrush_rage_fwclothdebug_enbrush_shape(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005064_rage_fwclothdebug_updateselection_bool_ref_bool_ref_int_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_005065_rage_fwclothdebug_drawprimitivesinfo_int_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_005066_rage_fwclothdebug_selectverts_rage_vector3_const_ref_rage_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_005067_rage_fwclothdebug_drawselection_rage_vector3_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_005069_rage_fwclothgizmo_gettransform_rage_mat34v_ref_rage_vec3v_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005092_rage_clothvariationdata_deletebuffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005094_rage_clothinstance_unregisterrestinterface_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005098_rage_clothmanager_addwidget_rage_clothcontroller_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_005099_rage_clothmanager_deallocateverlet_rage_clothinstance_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005101_rage_clothmanager_instanceclothdrawable_rage_clothmanager_clothd(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005105_rage_clothmanager_allocateverlet(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005106_rage_clothmanager_allocatevardata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005107_rage_clothmanager_instancecloth_rage_mat34v_const_ref_rage_envir(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_005108_rage_clothmanager_removecloth_rage_environmentcloth_ptr_rage_fra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_005109_rage_clothmanager_removecharactercloth_rage_clothcontroller_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005110_rage_clothmanager_deallocatevardata_rage_clothvariationdata_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005115_rage_clothinstance_drawbound_rage_phbound_const_ptr_rage_mat34v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_005137_rage_clothmanager_createcustomboundgroup_rage_phboundcomposite_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_005170_rage_ropedatamanager_unloadropetexures(ph: *mut Phantom); }
extern "C" { pub fn f_005197_rage_ropeinstance_shutdownphysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005198_rage_ropeinstance_checkfordetachfromconstraintarray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005201_rage_ropeinstance_updatecontrolvertices(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005202_rage_ropeinstance_getlocalposition_rage_vec3v_const_ref_rage_phi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_005213_rage_ropemanager_remove_rage_ropeinstance_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005219_rage_ropeinstance_isattached_rage_phinst_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005225_rage_ropemanager_add_rage_vec3v_const_ref_rage_vec3v_const_ref_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i32, a10: i32, a11: f32, a12: i32, a13: i32) -> i64; }
extern "C" { pub fn f_005227_rage_dldrawcommandbuffer_dtordldrawcommandbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005242_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_int_ptr_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_005243_int_ptr_std_2_partition_with_equals_on_left_abi_nqn220108_std_2(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_005244_std_2_pair_int_ptr_bool_std_2_partition_with_equals_on_right_abi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_005245_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_005282_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_005290_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_005297_void_rage_dlc_addinternal_void_ptr_rage_grcdebugdraw_lingerbuffe(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005299_void_rage_dlc_addinternal_void_ptr_rage_grcdebugdraw_lingerbuffe(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005301_void_rage_dlc_addinternal_void_ptr_void_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005302_rage_fwrenderphasemanager_printline_bool_char_const_ptr_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_005308_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_ct(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_005315_rage_fwrenderthreadinterface_flush_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_005317_rage_fwrenderthreadinterface_dorenderfunction(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005335_rage_fwentitydef_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_005345_rage_fwinstancedmapdata_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_005350_rage_fwmapdata_dtorfwmapdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005351_rage_atarray_rage_fwgrassinstancelistdef_0_unsigned_short_destru(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005362_rage_fwboxstreamer_initwidgets_rage_bkbank_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_005384_rage_fwmapdatastore_getenablediplgroups_rage_atarray_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005385_rage_fwmapdatastore_setenablediplgroups_rage_atarray_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005386_rage_fwmapdatastore_requestgroup_rage_strlocalindex_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_005387_rage_fwmapdatastore_removegroup_rage_strlocalindex_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_005388_rage_fwimapgroupmgr_createnewswap_rage_atnonfinalhashstring_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_005389_rage_fwimapgroupswap_preventfade(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005390_rage_fwimapgroupswap_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005391_rage_fwimapgroupswap_abandonearly_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005396_rage_fwstaticboundsstore_setenabled_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_005397_rage_fwimapgroupswap_start_int_rage_atnonfinalhashstring_const_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_005406_rage_fwpropinstancelistdef_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_005414_rage_fwgrassinstancelistdef_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_005434_rage_fwentitycontainer_appendentity_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_005437_rage_fwextensiondef_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_005438_rage_fwarchetypedef_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_005449_rage_fwmaptypes_dtorfwmaptypes(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005453_rage_fwboxstreamer_init_rage_strstreamingmodule_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_005458_rage_fwboxstreamer_requestaboutpos_rage_fwboxstreamersearch_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_005459_rage_atarray_rage_fwboxstreamersearch_0_unsigned_short_grow_unsi(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005476_rage_fwassetstore_rage_crclipdictionary_rage_fwclipdictionarydef(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005486_rage_fwclipdictionarystore_addref_rage_strlocalindex_rage_strref(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005490_rage_fwclipdictionarystore_removerefwithoutdelete_rage_strlocali(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005498_rage_fwassetstore_rage_pgdictionary_rage_charactercloth_rage_fwc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005501_rage_fwassetrscstore_rage_pgdictionary_rage_charactercloth_rage(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005502_rage_fwassetstore_rage_pgdictionary_rage_charactercloth_rage_fwc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005504_rage_atarray_rage_datowner_rage_charactercloth_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005508_rage_fwassetstore_rage_rmcdrawable_rage_fwdrawabledef_dtorfwasse(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005513_rage_fwtxdstore_setcurrenttxd_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005514_rage_fwtxdstore_popcurrenttxd(ph: *mut Phantom); }
extern "C" { pub fn f_005515_rage_pgdictionary_rage_grctexture_setcurrent_rage_pgdictionary_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005523_rage_fwassetstore_rage_pgdictionary_rage_rmcdrawable_rage_dwddef(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005527_rage_atarray_rage_datowner_rage_rmcdrawable_0_unsigned_short_ata(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005535_rage_fwassetstore_rage_pgdictionary_rage_crexpressions_rage_fwas(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005537_rage_fwassetstore_rage_pgdictionary_rage_crframefilter_rage_fwas(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005538_rage_crframefilterdictionarystore_findframefilter_rage_fwmvtempl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005539_rage_crframefilterdictionarystore_findframefilternameslow_rage_c(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005541_rage_fwassetstore_rage_fragtype_rage_fwfragmentdef_dtorfwassetst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005552_rage_fwimapgroupswap_makevisible_int_rage_atnonfinalhashstring_r(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_005559_rage_fwimapgroupswap_makeinvisible_int_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_005561_sprintf_s_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005566_rage_fwassetstore_rage_pgdictionary_rage_grctexture_rage_fwtxdde(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005569_rage_fwmapdatastore_dtorfwmapdatastore(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005570_rage_fwboxstreamer_dtorfwboxstreamer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005571_rage_fwassetstore_rage_fwmapdatacontents_rage_fwmapdatadef_dtorf(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005574_rage_fwassetstore_rage_fwmapdatacontents_rage_fwmapdatadef_final(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005576_rage_fwassetstore_rage_fwmapdatacontents_rage_fwmapdatadef_regis(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005584_rage_fwpsostoreloader_load_void_ptr_int_char_const_ptr_rage_fwps(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_005585_rage_fwmapdatastore_finishloading_rage_strlocalindex_rage_fwpsos(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_005586_rage_fwpsostoreloader_readobjectsfrompsofile_rage_psofile_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_005587_rage_fwpsostoreloader_loadxmlfile_rage_fistream_ptr_rage_fwpsost(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_005588_rage_fwpsostoreloader_unload_rage_fwpsostoreloadinstance_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_005592_rage_fwpsostoreplacementcookies_cookie_ref_rage_atmap_rage_strin(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005598_rage_fwassetstore_rage_fwmapdatacontents_rage_fwmapdatadef_addsl(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005605_rage_fwmapdatastore_modifyhierarchystatusrecursive_rage_strlocal(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005610_rage_fwmapdatastore_getparentslot_rage_strlocalindex_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_005613_rage_writemapdatacacheentrydetailstotextfile_rage_fwmapdatacache(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005615_rage_fwassetstore_rage_fwmaptypescontents_rage_fwmaptypesdef_dto(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005620_rage_fwmaptypesstore_setdepth_rage_atarray_int_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_005621_rage_fwmaptypesstore_saferemovetypeslot_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005623_rage_fwmaptypesstore_removeunrequired(ph: *mut Phantom); }
extern "C" { pub fn f_005625_rage_fwmaptypesstore_printstore(ph: *mut Phantom); }
extern "C" { pub fn f_005630_rage_fwmaptypesstore_finishloading_rage_strlocalindex_rage_fwpso(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_005637_rage_fwassetstore_rage_mvnetworkdef_rage_fwassetdef_rage_mvnetwo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005641_rage_fwassetstore_rage_crposematcher_rage_fwassetdef_rage_crpose(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005646_rage_atmap_rage_strindex_rage_fwpsostoreplacementcookies_cookie(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005648_rage_fwpsostoreloader_load_char_const_ptr_rage_fwpsostoreloadins(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_005652_rage_breakon_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005654_rage_fwstaticboundsstore_dtorfwstaticboundsstore(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005656_rage_writeboundsstorecacheentrydetailstotextfile_rage_boundsstor(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005657_rage_fwstaticboundsstore_setboxstreamerentry_int_rage_spdaabb_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_005660_rage_fwassetstore_rage_phbound_rage_fwbounddef_dtorfwassetstore(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005663_rage_fwstaticboundsstore_dumpstaticboundsinfo(ph: *mut Phantom); }
extern "C" { pub fn f_005666_rage_fwstaticboundsstore_addboundtophysicslevel_int_rage_phbound(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_005669_rage_fwassetstore_rage_phbound_rage_fwbounddef_addslot_rage_atno(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005674_rage_strlocalindex_ref_rage_atmap_rage_strlocalindex_rage_strloc(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_005675_rage_atmap_rage_strlocalindex_rage_strlocalindex_rage_atmaphashf(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005690_rage_fwtextasset_dtorfwtextasset(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005692_rage_fwtextasset_fwtextasset_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_005696_rage_pgdictionary_rage_grctexture_dtorpgdictionary(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005698_rage_fwassetstore_rage_pgdictionary_rage_grctexture_rage_fwtxdde(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005702_void_rage_pgrscbuilder_load_rage_pgdictionary_rage_grctexture_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_005710_rage_atarray_rage_datowner_rage_grctexture_0_unsigned_short_atar(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005714_rage_fwtxdstore_removeref_rage_strlocalindex_rage_strrefkind(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_005717_rage_fwsoaentitycontainer_updateentitydescriptorinnodeinternal_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005722_rage_fwentitycontainer_removeentityinternal_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005723_rage_fwsoaentitycontainer_removeentityinternal_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005726_rage_flushupdatelistforcontainer_rage_atarray_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005727_rage_fwexteriorscenegraphnode_dtorfwexteriorscenegraphnode(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005730_rage_fwinteriorlocation_makeinvalid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005732_rage_fwinteriorlocation_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005766_rage_fwscancascadeshadowinfo_updateshadowviewport_rage_vec4v_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_005768_rage_fwscancascadeshadowinfo_isshadowvisible_rage_spdaabb_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005772_rage_fwscreenquad_generatefrustum_rage_mat44v_const_ref_rage_spd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_005773_rage_fwscanentities_submitscanentries(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005774_rage_soautils_areentitiesintersectingplaneset5_rage_fwentitydesc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_005775_rage_fwscanentities_isentityearlyrejected_rage_fwentitydesc_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_005776_rage_fwscanentities_isentityexcludedinrenderphase_rage_fwrenderp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_005777_rage_fwscanentities_isentityexcludedbylodrange_rage_fwrenderphas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_005778_rage_fwscanentities_calculateprojectocclusionobb_rage_fwscancasc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32); }
extern "C" { pub fn f_005779_rage_fwscanentities_isentityoccluded_rage_fwrenderphasecullshape(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_005780_rage_rstcalculatescreenspaceboundisvis_rage_rstsplatmat44v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_005781_rage_fwscanentities_isentityclippeddefault_rage_fwrenderphasecul(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_005782_rage_fwscanentities_isentityclippedscreenquad_rage_fwrenderphase(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_005783_rage_fwscanentities_isentityclipped_rage_fwrenderphasecullshape(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005784_rage_fwscanentities_processsingleentitydescsoa_int_int_rage_fwen(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_005785_rage_fwscannodes_processexteriorvisibility_rage_fwexteriorsceneg(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_005786_rage_fwscannodes_processroomvisibility_rage_fwroomscenegraphnode(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32, a6: i32, a7: i32); }
extern "C" { pub fn f_005787_rage_fwscannodes_addvisibility_rage_fwscenegraphnode_ptr_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_005788_rage_fwscannodes_processinteriorroomlessvisibility_rage_fwroomsc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_005789_rage_fwscannodesdebug_debugdrawportal_rage_fwscenegraphnode_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_005790_rage_fwscannodes_processentitycontainer_rage_fwscenegraphnode_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_005791_rage_fwscannodes_traverseportals_int_rage_fwscenegraphnode_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: f32, a7: i32, a8: i32); }
extern "C" { pub fn f_005792_rage_fwscannodesdebug_showportaltraversalinfo_rage_fwrenderphase(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_005793_rage_fwportalcorners_op_rage_fwportalcorners_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005794_rage_fwscannodesdebug_debugdrawintermediatescreenquad_rage_fwren(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005795_rage_fwscannodesdebug_drawfrustumforscreenquad_rage_fwscreenquad(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_005803_rage_fwsearchentities_process(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005804_rage_fwsearchvolume_searchvolumesintersect_rage_spdsphere_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005824_void_rage_pgrscbuilder_load_rage_crclipdictionary_rage_crclipdic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_005865_rage_fwassetstore_rage_fwtextasset_rage_fwtextassetdef_dtorfwass(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005878_rage_atarray_rage_datowner_rage_crframefilter_0_unsigned_short_a(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005881_rage_pgdictionary_rage_crframefilter_dtorpgdictionary(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005887_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_005888_rage_fwentitylist_celement_ptr_std_2_partial_sort_impl_abi_nqn22(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_005889_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_005890_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_005891_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_005892_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_005894_rage_atrangearray_rage_grcstaticinstancebufferlist_3_dtoratrange(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005901_rage_pgdictionary_rage_charactercloth_dtorpgdictionary(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005903_rage_pgdictionary_rage_rmcdrawable_dtorpgdictionary(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005908_rage_atarray_rage_datowner_rage_crexpressions_0_unsigned_short_a(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005910_rage_pgdictionary_rage_crexpressions_dtorpgdictionary(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005913_rage_atmapentry_unsigned_int_rage_datowner_rage_fwtextfield_decl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005917_rage_fwrefawarebaseimpl_rage_datbase_dtorfwrefawarebaseimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005923_rage_aitaskstatetransitiontable_checktransitions_rage_fwentity_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_005924_rage_aitask_picknewstatetransition_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_005933_rage_aitasktree_aborttaskswithpriority_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005937_rage_aitasktree_spewtaskhierarchy_rage_aitask_ptr_rage_aitaskspe(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_005942_rage_aitasktree_deletetask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005943_rage_atfixedarray_rage_taskregdref_rage_aitask_5_dtoratfixedarra(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005955_void_rage_datresource_place_rage_aisplitarray_rage_cnavmeshcompr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005956_void_rage_datresource_place_rage_aisplitarray_unsigned_short_819(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005957_void_rage_datresource_place_rage_aisplitarray_rage_tadjpoly_2048(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005958_void_rage_datresource_place_rage_aisplitarray_rage_tnavmeshpoly(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005959_rage_cnavmeshquadtree_cnavmeshquadtree_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_005960_rage_cnavmesh_dtorcnavmesh(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005961_rage_cnavmeshquadtree_dtorcnavmeshquadtree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005963_rage_cnavmesh_cnavmesh_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_005964_rage_cnavmesh_getvertex_unsigned_int_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_005969_rage_cnavmesh_getpolycentroid_unsigned_int_rage_vector3_ref_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_005972_rage_cnavmesh_switchpolysinareainquadree_rage_cnavmeshquadtree_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_005974_rage_cnavmesh_rayintersectstriangle_rage_vector3_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005975_rage_cnavmesh_calculateallpolyminmaxesfordynamicnavmesh(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005976_rage_cnavmesh_getpolycentroidquick_rage_tnavmeshpoly_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_005977_rage_cnavmesh_getquadtreeleaf_rage_cnavmeshquadtree_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_005979_rage_cnavmesh_getintersectpoly_rage_vector3_const_ref_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005980_rage_cnavmesh_getintersectpolynoquadtree_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005982_rage_cnavmesh_getclosestnavmeshpolyedger_rage_tshortminmax_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64); }
extern "C" { pub fn f_005983_rage_cnavmesh_getclosestpointinpolyr_rage_tshortminmax_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64); }
extern "C" { pub fn f_005984_rage_createpointsinpolynormal_rage_cnavmesh_const_ptr_rage_tnavm(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_005985_rage_cnavmesh_forallpolys_r_rage_cnavmeshquadtree_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_005987_rage_cnavmesh_getnonstandardadjacancyatpositionr_rage_cnavmeshqu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_005991_rage_aimeshstore_rage_cnavmesh_rage_aimeshassetdef_rage_cnavmesh(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005999_rage_fwpathserver_getnavmeshindexfromfilename_rage_ainavdomain_c(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_006016_rage_cpathserverbinheap_heapifyup_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006032_rage_atmap_int_bool_rage_atmaphashfn_int_rage_atmapequals_int_ra(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006034_rage_strcacheloader_setcacheasinvalid(ph: *mut Phantom); }
extern "C" { pub fn f_006035_rage_atmapentry_unsigned_int_bool_const_ref_rage_atmap_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006036_void_rage_getdateasstring_64ul_unsigned_long_char_ref_64ul(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006040_rage_strcacheloader_writestringtobuffer_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006041_rage_strcacheloader_cmoduledescarray_addmodule_char_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_006042_rage_strcacheloader_writedatatobuffer_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006049_rage_strpackfilemanager_addimagetolist_char_const_ptr_bool_int_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_006050_rage_strstreaminginfomanager_registerobject_char_const_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_006052_rage_strpackfilemanager_unloadrpf_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006054_rage_strpackfilemanager_closearchive_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006055_rage_strpackfilemanager_removestreamablereference_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006056_rage_strstreaminginfomanager_removeobject_rage_strindex_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006063_rage_strstreamingmodulemgr_addmodule_rage_strstreamingmodule_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006064_rage_strstreamingvisualize_send_int_void_const_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_006066_rage_strpackfilemanager_getimagefilefromhandle_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006067_rage_strpackfilemanager_getimagefileindexfromhandle_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006071_rage_strstreamingmodulemgr_getmoduleidfromindex_rage_strindex_co(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006072_rage_strstreaminginfomanager_getobjectname_rage_strindex_char_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_006073_rage_strstreaminginfomanager_addpackfilerelation_int_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006074_rage_strstreaminginfomanager_removeobjectanddependents_rage_stri(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006075_rage_strstreaminginfo_computevirtualsize_rage_strindex_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006076_rage_strstreaminginfo_computephysicalsize_rage_strindex_bool_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006077_rage_atmapentry_rage_strindex_std_2_pair_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006078_rage_datresourceinfo_getvirtualsize_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006079_rage_datresourceinfo_getphysicalsize_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006080_rage_strstreaminginfo_setflag_rage_strindex_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006082_rage_strpackfilemanager_loadrpf_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006083_rage_strpackfilemanager_addstreamablereference_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006084_rage_strstreaminginfomanager_requestobject_rage_strindex_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006089_rage_strstreaminglive_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006090_rage_strstreamingvisualize_unrequest_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006092_rage_strstreamingmodulemgr_getmodulefromindex_rage_strindex_cons(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006093_rage_strstreamingallocator_removefrommemoryused_void_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006094_rage_strstreaminginfomanager_removerequest_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006095_rage_strstreamingloadermanager_cancelrequest_rage_strindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006096_rage_strstreaminginfomanager_removeloaded_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006097_rage_strstreaminginfomanager_removedependentcountsandunrequest_r(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006098_rage_strstreaminginfomanager_printrequestlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006099_rage_strstreamingvisualize_request_rage_strindex_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006100_rage_strstreaminginfo_getdevices_rage_strindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006101_rage_strstreaminginfomanager_setrequiredflag_rage_strindex_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006102_rage_strstreamingvisualize_setflags_rage_strindex_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006103_rage_strstreaminginfomanager_adddependentcounts_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006104_rage_strstreaminginfomanager_completedummyrequest_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006107_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_st(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_006110_rage_strpackfilemanager_evictunusedimages(ph: *mut Phantom); }
extern "C" { pub fn f_006114_rage_strrecordedrequest_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_006120_rage_strresourcereference_resolve_bool_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006124_rage_strstreamingengine_getobjectname_rage_strindex(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006126_rage_strstreamingallocator_addtomemoryused_void_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006128_rage_strstreamingallocator_makespaceforallocation_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006129_rage_strexternalalloctracker_addrecord_void_ptr_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_006130_rage_strstreaminginfomanager_deletenextleastusedobject_unsigned(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_006131_rage_strstreamingallocator_streamingallocate_rage_datresourcemap(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006141_rage_strstreamingmodule_getobjectanddependencies_rage_strindex_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_006142_rage_strstreaminginfomanager_isobjectreadytodelete_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006150_rage_strstreaminginfomanager_movetopersistentloaded_rage_strinde(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006151_rage_strstreaminginfomanager_printloadedlist_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006152_rage_atmapentry_rage_strindex_rage_strloadedinfo_ref_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006153_rage_strstreaminginfomanager_initloaded_rage_strloadedinfo_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_006154_rage_strstreaminginfomanager_movetoloaded_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006155_rage_strstreaminginfomanager_addloaded_rage_strindex_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_006156_rage_atarray_rage_strindex_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006158_rage_strstreaminginfo_getcdposn_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006160_rage_strstreaminginfomanager_printinformation_int_rage_strstream(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_006161_rage_strstreaminginfomanager_printdependenciesrecurse_rage_strin(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_006162_rage_strstreaminginfo_gethandle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006163_rage_strstreaminginfomanager_setobjecttoloaded_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006164_rage_strstreaminginfo_clearflag_rage_strindex_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006165_rage_strstreaminglivefile_unlinkfile(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006166_rage_strstreamingloader_cancelrequest_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006167_rage_strstreaminginfomanager_isobjectinimage_rage_strindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006169_rage_strstreaminginfomanager_getstrindiciesforarchive_int_rage_a(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_006170_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_006171_rage_atbinarymap_bool_unsigned_int_safeinsert_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006172_rage_strstreaminginfomanager_setobjecttoloading_rage_strindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006173_rage_strstreaminginfomanager_setobjecttonotloadedfromloading_rag(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006175_rage_strstreaminginfomanager_clearrequiredflag_rage_strindex_uns(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006176_rage_strstreaminginfomanager_getnextfilesoncdnew_unsigned_int_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_006177_void_std_2_introsort_std_2_classicalgpolicy_rage_requestsort_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_006178_rage_strstreaminginfomanager_getstreamingindexbyfilename_char_co(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006181_rage_atmapentry_unsigned_int_rage_strindex_const_ref_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006182_rage_strstreamingmodule_getobjectanddependencies_rage_strindex_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_006184_rage_strstreaminginfomanager_printalldependents_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006188_rage_atarray_int_0_unsigned_short_ptr_ref_rage_atmap_int_rage_at(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006189_rage_strstreaminginfomanager_registerdummyobject_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006190_rage_strstreaminginfomanager_unregisterobject_rage_strindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006191_rage_strstreamingvisualize_unregisterobject_rage_strindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006192_rage_strstreaminginfomanager_getrequestinfopoolcapacity_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_006196_rage_strstreaminginfomanager_setdonotdefrag_rage_strindex(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006207_rage_strstreaminglive_init(ph: *mut Phantom); }
extern "C" { pub fn f_006208_rage_strstreaminglive_addlivefolder_char_const_ptr_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006211_rage_strstreamingloadermanager_flush(ph: *mut Phantom); }
extern "C" { pub fn f_006213_rage_strstreamingloader_loadrequestedobjects(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006216_rage_strstreamingloader_flush(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006218_rage_strstreamingloadermanager_submitdeferredasyncplacementreque(ph: *mut Phantom); }
extern "C" { pub fn f_006219_rage_strstreamingengine_getstrindex_void_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006220_rage_strstreamingengine_addrefbyptr_void_ptr_rage_strrefkind(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006221_rage_strstreamingengine_removerefbyptr_void_ptr_rage_strrefkind(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006225_rage_atvarstring_atvarstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_006226_rage_strstreaminglivefolder_updatedirtyfile_rage_strstreamingliv(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006230_rage_aredependenciesmet_rage_strindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006232_rage_strstreamingloader_processasyncfiles(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006233_rage_strstreamingloader_performmainthreadplacement_rage_strstrea(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006234_rage_strstreamingloader_markplacementcomplete_rage_strstreamingl(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006235_rage_strstreamingvisualize_setmarkertext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006236_rage_strstreamingloader_processfile_rage_strstreamingloader_stre(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006238_rage_strstreamingloadermanager_loadallrequestedobjects_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006241_rage_strstreamingvisualize_addmarker_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006242_rage_strstreamingmodule_stoptrackingplace_char_const_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006243_rage_strstreamingloadermanager_requestasyncplacement_rage_strstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006249_rage_strstreamingmodule_resetplacestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006250_rage_strstreamingmodule_getrefcountstring_rage_strlocalindex_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_006253_rage_strmoduletypeindextoextension_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006255_rage_strrequest_hasloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006265_rage_strmodelrequest_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006281_rage_strstreamingvisualize_addcallstacktomarker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006285_rage_strstreamingvisualize_deleteentity_rage_fwentity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006301_rage_tcinstboxtype_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_006320_rage_tccycle_load_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006335_rage_tcdebug_gettimecyclefloatvalues_int_rage_scalarv_ref_rage_v(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_006337_rage_tcdebug_gettimecyclecolorvalues_int_rage_scalarv_ref_rage_v(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_006342_rage_tcmanager_findmodifier_rage_atnonfinalhashstring_char_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006344_rage_tcdebug_setmodifierfileinfo_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006345_rage_tcdebug_calculatefilecontentchecksum_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006346_rage_tcdebug_modifierrecordeditidchanged(ph: *mut Phantom); }
extern "C" { pub fn f_006347_rage_tcdebug_modifierchanged(ph: *mut Phantom); }
extern "C" { pub fn f_006348_rage_tcmodifier_addmoddata_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32); }
extern "C" { pub fn f_006349_rage_tcdebug_updatemodifiereditorwidgets(ph: *mut Phantom); }
extern "C" { pub fn f_006355_rage_tcmanager_initmodifierdata_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006356_rage_tcmanager_loadmodifierfile_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006357_rage_tcmanager_finishmodifierload(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006358_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_006359_rage_tcdebug_modifiersavefile_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006360_rage_tcdebug_modifiercopystate_rage_tcmodifiereditstate_const_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006361_rage_atarray_float_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006389_rage_tcmanager_removemodifierboxes_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_006392_rage_tcmanager_buildmodifiertree(ph: *mut Phantom); }
extern "C" { pub fn f_006401_rage_tcmodifier_allocatevarmap_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006404_rage_tcmoddata_blend_rage_tcmoddata_const_ref_rage_tcmodifiertyp(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_006449_rage_fwassetstore_rage_cnavmesh_rage_aimeshassetdef_rage_cnavmes(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006454_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_006455_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_006456_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_rage_reque(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_006457_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006458_rage_audbiquadfiltereffect_audbiquadfiltereffect(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006463_rage_audbiquadfiltereffect_processinterleaved_4pole_float_vector(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_006464_rage_audbiquadfiltereffect_processinterleaved_2pole_float_vector(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_006470_rage_auddelayeffect4_auddelayeffect4(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006475_rage_audearlyreflectioneffect_dtoraudearlyreflectioneffect(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006476_rage_atrangearray_rage_revsimpledelay_v8_6_dtoratrangearray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006480_rage_audearlyreflectioneffect_processinterleaved_float_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006482_rage_audreverbeffect4_audreverbeffect4(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006484_rage_atrangearray_rage_fvcombfilter_8_dtoratrangearray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006488_rage_audreverbeffect4_processinterleaved_float_const_ptr_float_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006494_rage_audunderwatereffect_processinterleaved_float_const_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006503_rage_audautomationsound_process_unsigned_int_rage_audautomations(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_006504_rage_audsound_managedaudioprepare_rage_audwaveslot_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_006505_rage_audsound_managedaudioplay_unsigned_int_rage_audsoundcombine(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_006506_rage_audsound_managedaudioupdate_unsigned_int_rage_audsoundcombi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_006511_rage_audsound_computedurationmsincludingstartoffsetandpredelay_b(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006512_rage_audautomationsound_dtoraudautomationsound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006513_rage_audsound_dtoraudsound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006515_rage_audsound_audsound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006517_rage_audsound_init_void_const_ptr_rage_audsoundinternalinitparam(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_006518_rage_audsound_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006519_rage_audsound_computehierarchystring_bool_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006520_rage_audsound_computeplaybackpredelayandstartoffset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006521_rage_audsound_computedynamicvalue_rage_auddynamicvalueid_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_006522_rage_audautomationsound_readvariablelength_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006523_rage_audautomationsound_processevent_rage_audautomationsound_aut(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006524_rage_audsound_combinebuffers_rage_audsoundcombinebuffer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006526_rage_audsound_arechildrenplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006527_rage_audsound_actionhierarchyremovalrequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006530_rage_audcollapsingstereosound_setattenuationfactors_bool(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_006539_rage_audcrossfadesound_setattenuationfactors_rage_audcrossfadeso(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006540_rage_audcrossfadesound_updatechildsounds_rage_audcrossfadesounds(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006549_rage_audsound_managedaudiodestroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006558_rage_audsound_gettoplevelparent_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006567_rage_audenvelopesound_applyenvelope_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006568_rage_audenvelopesound_applyenvelopetochild_rage_audsoundcombineb(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006572_rage_audenvelopesound_getmetadataenvelope_unsigned_int_ref_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_006579_rage_audenvelopesound_actionreleaserequestinternal_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006580_rage_audsound_baseactionreleaserequest_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006583_rage_audenvironmentsound_audioupdate_unsigned_int_rage_audsoundc(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006584_rage_audenvironmentsound_freevoice(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006589_rage_audenvironmentsound_dtoraudenvironmentsound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006591_rage_audenvironmentsound_audenvironmentsound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006599_rage_audexternalstreamsound_dtoraudexternalstreamsound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006605_rage_audgranularsound_initgrainplayer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006607_rage_audgranularsound_dtoraudgranularsound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006610_rage_audsound_decompressmetadata_untyped_rage_sound_const_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_006623_rage_audkineticsound_updatekineticsoundvariables_unsigned_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006633_rage_audloopingsound_dtoraudloopingsound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006642_rage_audmodularsynthsound_dtoraudmodularsynthsound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006651_rage_audmultitracksound_dtoraudmultitracksound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006659_rage_audonstopsound_dtoraudonstopsound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006671_rage_audretriggeredoverlappedsound_setupchildsound_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_006678_rage_audsound_managedaudiostopchildren(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006684_rage_audsequentialoverlapsound_dtoraudsequentialoverlapsound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006689_rage_audsequentialsound_handlefreeslot_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006700_rage_audsimplesound_setupwaveplayer_rage_audwaveslot_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006702_rage_audsimplesound_dtoraudsimplesound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006708_rage_audsound_computeoffsetsfromvariances_rage_sound_const_ptr_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_006714_rage_audsound_play(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006715_rage_audsound_prepareandplay_rage_audwaveslot_ptr_bool_int_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_006716_rage_audsound_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006717_rage_audsound_stopandforget_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006718_rage_audsound_invalidatetracker_rage_audrequestedsettings_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006720_rage_audsound_managedaudiopreparefromsoundmanager(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006721_rage_audsound_getdynamicvariablehandlefromname_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006729_rage_audsound_searchforsoundsbytype_unsigned_int_rage_audsound_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_006731_rage_audsound_debugdraw_rage_auddebugdrawmanager_ref_bool_bool_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_006732_rage_gsoundsisoftype_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006736_rage_audspeechsound_setupwaveplayer(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006738_rage_audspeechsound_dtoraudspeechsound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006741_rage_audspeechsound_findvariationinfoforvoiceandcontext_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_006747_rage_audspeechsound_initspeech_unsigned_int_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_006760_rage_audstreamingsound_dtoraudstreamingsound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006767_rage_audtwinloopsound_getnewswaptime_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006773_rage_audwrappersound_applyvariables_rage_audsoundcombinebuffer_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006782_rage_audifsound_computebranchtobeplayed(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006788_rage_audmathoperationsound_runcomputation_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006790_rage_audmathoperationsound_dtoraudmathoperationsound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006795_rage_audparametertransformsound_releaseslots(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006796_rage_audparametertransformsound_initslots_rage_parametertransfor(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006797_rage_audparametertransformsound_processtransforms_float_bool_rag(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_006803_rage_audfluctuatorsound_updatefluctuatorsoundvariables_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_006805_rage_audfluctuatorsound_dtoraudfluctuatorsound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006813_rage_audvariableblocksound_dtoraudvariableblocksound(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006834_rage_adatcontainer_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006835_rage_adatcontainer_isvalidheader_void_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006836_rage_adatcontainer_isnativeendian_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006837_rage_adatcontainer_isdataencrypted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006838_rage_adatcontainer_findobject_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006839_rage_adatcontainer_findobjectdata_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006840_rage_adatcontainer_getobjectnamehash_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006841_rage_adatcontainer_getobjectdatachunk_int_int_unsigned_int_ref_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_006842_rage_adatcontainer_getheadersize_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006843_rage_audambisonics_setupbformatcoefficients_rage_bformatcoeffici(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006845_rage_audambisonics_calculatefitnesscoefficients_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006846_rage_audambisonics_setenvironmentdecoder_rage_ambisonicdecodermo(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006847_rage_audambisonics_tabucore_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006849_rage_audambisonics_randomizespeakerweights_rage_bformatcoefficie(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006876_rage_audcategory_initcore_unsigned_int_rage_audcategory_ptr_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_006877_rage_audmetadatamanager_getobjectmetadataptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006878_rage_audcategory_initdynamic_unsigned_int_rage_audmetadatamanage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_006879_rage_audcategory_init_unsigned_int_rage_audmetadatamanager_ptr_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_006880_rage_audcategory_pullmetadatavalues_rage_category_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006881_rage_audmetadatamanager_getobjectmetadatareffromhash_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006882_rage_audmetadatamanager_searchforobjectfromhash_unsigned_int_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_006883_rage_audcategory_getnamestring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006884_rage_audcategory_getmuteonusermusic(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006885_rage_audcategory_getrearattenuationdisabled(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006886_rage_audmetadatamanager_getobjectnamefromnametableoffset_unsigne(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006887_rage_audcategory_pullmetadatavalues(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006888_rage_audcategory_issoloactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006889_rage_audcategory_ismuteactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006890_rage_audcategory_audiothreadupdate_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_006892_rage_audmetadatamanager_dtoraudmetadatamanager(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006894_rage_audmetadatamanager_loadmetadatachunk_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_006895_rage_audmetadatamanager_init_char_const_ptr_char_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_006896_rage_audmetadatamanager_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006897_rage_audmetadatamanager_deletechunk_rage_audmetadatachunk_ref_bo(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006898_rage_audmetadatamanager_computenumberofobjects_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006899_rage_audengine_allocatevirtual_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006900_rage_audcategorymanager_shutdowndynamiccategory_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006901_rage_audcategorymanager_getdynamiccategoryptr_int_rage_category(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_006902_rage_audcategorymanager_getdynamiccategoryindex_int_rage_categor(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_006903_rage_audcategorymanager_getcategoryptr_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006908_rage_audcategorycontroller_update_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_006915_rage_printcategoryinfo_rage_audcategory_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006917_rage_audcategorycontrollermanager_createcontroller_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_006918_rage_audcategorycontrollermanager_destroycontroller_rage_audcate(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006919_rage_audcategorycontrollermanager_createmultiplecontrollers_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_006922_rage_audcategorycontrollermanager_recurseaddcategorywidget_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_006924_rage_audcontroller_clearallsounds_rage_audcontrollerentityref_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_006928_rage_audcontroller_updateentityref_rage_audcontrollerentityref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_006931_rage_audcontroller_stopallsounds_rage_audentity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_006932_rage_audcontroller_getsoundinstance_rage_audmetadataref_rage_aud(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_006933_rage_audmetadatamanager_getobjectname_rage_audmetadataref_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006934_rage_audsoundfactory_getinstancefrommetadata_void_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_006935_rage_audcontroller_getpersistentsoundinstance_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_006936_rage_audcontroller_getpersistentsoundinstance_rage_audmetadatare(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_006938_rage_audcurve_audcurve(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006940_rage_audcurve_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_006941_rage_audcurve_calculatevalue_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_006942_rage_audcurve_calculaterescaledvalue_float_float_float_float_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32) -> f32; }
extern "C" { pub fn f_006944_rage_audmetadatamanager_getobjectname_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006950_rage_audmixgroup_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006951_rage_auddynamicmixmgr_setupmixgroupmap_rage_atarray_rage_audcate(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_006952_rage_auddynamicmixmgr_initmixgroupsforchunk_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006953_rage_audmetadatamanager_getobjectmetadataptrfromspecificchunk_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_006954_rage_audmetadatamanager_findchunkid_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006955_rage_audmetadatamanager_getobjectinfofromspecificchunk_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_006956_rage_auddynamicmixmgr_getmixgroup_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_006960_rage_audmixgroup_addmixgroupreference_int_rage_audmixgroup_mixgr(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006961_rage_audmixgroup_removemixgroupreference_int_rage_audmixgroup_mi(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_006962_rage_audmixgroup_getreferencecount_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006964_rage_audengine_partialinit(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_006966_rage_audremotecontrol_init(ph: *mut Phantom); }
extern "C" { pub fn f_006967_rage_audremotedebug_init(ph: *mut Phantom); }
extern "C" { pub fn f_006969_rage_audengine_partialshutdown(ph: *mut Phantom); }
extern "C" { pub fn f_006970_rage_audremotecontrol_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_006971_rage_audengine_triggerupdate(ph: *mut Phantom); }
extern "C" { pub fn f_006983_rage_audengineutil_matchname_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_006984_rage_audengineutil_getcurrenttimeinmilliseconds(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_006993_rage_audenvironment_calculateenvironmentsoundmetricsforsinglelis(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_006994_rage_audenvironment_computespeakervolumesfrompan_rage_audenviron(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_006998_rage_audenvironmentgroupinterface_addsoundreference(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_006999_rage_audenvironmentgroupinterface_removesoundreference(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007004_rage_audenvironment_isdirectmonoeffectroute_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_007015_rage_audmetadataobjectinfo_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007017_rage_audmetadatamanager_loadmetadatachunk_internal_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_007018_rage_atarray_rage_audmetadatachunk_0_unsigned_short_grow_unsigne(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007019_rage_audmetadatamanager_computestringtablemap(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007021_rage_audmetadatamanager_loadexternalnametable_rage_audmetadatach(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_007023_rage_atarray_rage_audmetadatachunk_audmetadatarefhashentry_0_uns(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007025_rage_atmapentry_unsigned_int_unsigned_short_const_ref_rage_atmap(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007027_rage_audmetadatamanager_getstringidfromhash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007028_rage_audmetadatamanager_getobjectmetadataptr_rage_audmetadataref(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007029_rage_audmetadatamanager_addextrastringtable_rage_audmetadatachun(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007030_rage_audmetadatamanager_removeextrastringtable_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007032_rage_audmetadatamanager_getobjectinfo_unsigned_int_rage_audmetad(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007033_rage_audmetadatamanager_getobjectinfo_rage_audmetadataref_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007034_rage_audmetadatamanager_getobjectmetadataptr_unsigned_int_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007035_rage_audmetadatamanager_getobjectinfofromobjectindex_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007036_rage_audmetadatamanager_getobject_verify_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007037_rage_audmetadatamanager_getobject_verify_rage_audmetadataref_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007038_rage_audmetadatamanager_getobjectmetadataptrandchunkid_unsigned(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_007039_rage_audmetadatamanager_getstringfromtableindex_unsigned_int_con(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_007041_rage_audsoundpool_allocatesoundslot_unsigned_long_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007044_rage_audsoundfactory_getchildinstance_rage_audmetadataref_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_007045_rage_audsoundfactory_getchildinstance_unsigned_int_rage_audsound(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_007046_rage_audsoundfactory_getenvironmentsound_rage_audsound_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_007047_rage_audsoundfactory_getmetadataptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007048_rage_audsoundfactory_getmetadataptr_rage_audmetadataref_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007050_rage_audsoundfactory_processhierarchy_rage_audmetadataref_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_007051_rage_audsoundfactory_getbanknamefromindex_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007052_rage_audsoundfactory_getbankindexfromname_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007054_rage_audsoundfactory_getbankindexfrommetadataref_unsigned_int_co(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007063_rage_audsoundmanager_getvariableaddress_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007067_rage_audsoundmanager_matchesnamefilter_char_const_ptr_rage_auden(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007071_rage_audsoundpool_allocaterequestedsettingsslot_unsigned_long_un(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_007072_rage_audsoundpool_deletesound_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007073_rage_audsoundpool_deleterequestedsettings_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_007074_rage_audsoundpool_freevoice_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_007075_rage_audsoundpool_requestchildsound_unsigned_int_unsigned_int_ra(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_007076_rage_audsoundpool_debugprintsoundpool_const(ph: *mut Phantom); }
extern "C" { pub fn f_007079_rage_audsoundset_find_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007086_rage_audremotecontrol_isconnected_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007091_rage_audremotedebug_sendmessage_rage_audremotedebugcommands_unsi(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_007092_rage_audrequestedsettings_dtoraudrequestedsettings(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007093_rage_audentity_removevariableblockref_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007094_rage_audrequestedsettings_setpan_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007095_rage_audrequestedsettings_setpitch_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007096_rage_audrequestedsettings_setposition_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007098_rage_audrequestedsettings_setvolume_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007099_rage_audrequestedsettings_setpostsubmixvolumeattenuation_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007103_rage_audrequestedsettings_setlowpassfiltercutoff_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007104_rage_audrequestedsettings_sethighpassfiltercutoff_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007106_rage_audrequestedsettings_setissettostop_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007108_rage_audrequestedsettings_setvolumecurvescale_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007117_rage_audrequestedsettings_setenvironmentgroup_rage_audenvironmen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007118_rage_audsmoother_audsmoother(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007119_rage_audsmoother_init_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_007120_rage_audsmoother_init_float_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_007121_rage_audsmoother_setrates_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_007123_rage_audsmoother_calculatevalue_float(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_007126_rage_audsimplesmoother_audsimplesmoother(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007127_rage_audsimplesmoother_init_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_007128_rage_audsimplesmoother_calculatevalue_float(ph: *mut Phantom, a0: i64, a1: f32) -> f32; }
extern "C" { pub fn f_007129_rage_audsimplesmoother_calculatevalue_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32) -> f32; }
extern "C" { pub fn f_007134_rage_audfluctuatorprobbased_init_float_float_float_float_float_f(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: f32); }
extern "C" { pub fn f_007136_rage_audfluctuatorprobbased_calculatevalue(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_007140_rage_audvolumeconelite_init_rage_vec3v_const_ref_float_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_007141_rage_audvolumeconelite_computeattenuation_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_007142_rage_audvariableblock_findvariableaddress_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007148_rage_synthcore_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007149_rage_synthcore_startphys(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007150_rage_synthcore_generateframe_float_const_ptr_ptr_rage_atrangearr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007151_rage_synthcore_applypreset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007166_rage_synthcoretest_initsynth(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007167_rage_synthcoretest_validateoutputbuffer_float_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007169_rage_synthbiquadfilter_computecoefficients_allpass_float_float_f(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_007176_rage_audmixerdevice_audmixerdevice(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007177_rage_audmixerdevice_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007178_rage_audvoicemgr_setphysicalvoicelimit(ph: *mut Phantom); }
extern "C" { pub fn f_007179_rage_audmixervoice_disconnectalloutputsinternal(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007180_rage_audvoicemgr_writevoicecapturepacket_rage_audvoicecapturepac(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007181_rage_audmixervoice_writecommand_rage_audcommandpacketheader_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007182_rage_audvoicemgr_allocatephysicalvoice_rage_audvoicesettings_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007183_rage_audstreamingwaveslot_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007184_rage_audwaveslot_baseupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007186_rage_audpcmsourceinterface_getstate_rage_pcmsourcestate_ref_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007187_rage_writecapturepacket_rage_prioritisedvoice_const_ptr_rage_voi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007188_rage_audvoicemgr_setnumberofphysicalvoices_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007189_rage_audvoicemgr_printphysicalvoices(ph: *mut Phantom); }
extern "C" { pub fn f_007190_rage_audmixervoice_setoutputvolumes_unsigned_int_float_vector_4(ph: *mut Phantom, a0: i64, a1: i32, a2: u128); }
extern "C" { pub fn f_007191_rage_audcommandbuffer_writealignedpacket_rage_audcommandpackethe(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007200_char_ptr_rage_formatf_char_255ul_char_ref_255ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007201_rage_audwaveslot_getbankname_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_007203_rage_audwaveslot_loadbank_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_007204_rage_audwaveslot_computebankfilepath_char_ptr_unsigned_int_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_007205_rage_audwaveslot_findbankid_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007207_rage_audwaveslot_removefromloadedbanktable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007208_rage_atmapentry_unsigned_short_rage_audwaveslot_ptr_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007209_rage_audwaveslot_removefromloadedwavetable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007210_rage_audwaveslot_getwaveloadingstatus_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_007212_rage_audwaveslot_getreferencecount_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007213_rage_audwaveslot_handlewaveloadcompletion(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007215_rage_audwaveslot_cleanupafterloadfailure(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007220_rage_audwaveslot_findloadedbankwaveslot_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_007224_rage_audwaveslot_loadasset_unsigned_int_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_007225_rage_audwaveslot_forceunloadbank(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007227_rage_audwaveslot_getassetloadingstatus_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_007228_rage_audwaveslot_addslotreference_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007230_rage_audwaveslot_removeslotreference_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007232_rage_audstreamingwaveslot_setdebugmarker_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007233_rage_audstreamingwaveslot_dtoraudstreamingwaveslot(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007236_rage_audstreamingwaveslot_getspecificblockinfo_unsigned_int_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_007237_rage_audwaveref_init_rage_audwaveslot_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_007238_rage_audwaveref_findchunk_unsigned_int_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_007239_rage_audwaveref_findformat_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_007241_rage_audwaveref_getwavenamehash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007242_rage_audmixerdevice_getsubmixindex_rage_audmixersubmix_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007243_rage_audmixerconnection_destroyconnection_rage_audmixerconnectio(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007245_rage_auddecodemanager_allocatedecoder_rage_audwaveformat_audstre(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_007266_rage_audmixerdevice_mixbuffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007267_rage_audmixerdevice_computeprocessinggraph(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007268_rage_audmixerdevice_generatepcm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007269_rage_audmixerdevice_computeprocessinggraph_rage_audmixersubmix_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_007270_rage_audmixersubmix_allocatebuffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007271_void_rage_interleaveandmix_1to4_256u_float_vector_4_float_vector(ph: *mut Phantom, a0: u128, a1: u128, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_007272_rage_audmixerdevice_freepcmsource_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007273_rage_audmixersyncmanager_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007275_rage_audmixerdevice_createsubmix_char_const_ptr_unsigned_int_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_007276_rage_audpcmsourcefactory_initslot_unsigned_int_rage_audpcmsource(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_007277_rage_audmixerdevice_getpcmsourceslotid_rage_audpcmsource_const_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007278_rage_audmixerdevice_initclientthread_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_007279_rage_audmixerdevice_flagthreadcommandbufferreadytoprocess_unsign(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007280_rage_audmixerdevice_waitonthreadcommandbufferprocessing_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007283_rage_audpcmsource_freebuffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007296_rage_audstreamplayer_advancereadoffset_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007297_rage_audmixersyncmanager_trigger_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_007299_rage_audrateconvertorlinear_process_float_float_ptr_short_const(ph: *mut Phantom, a0: f32, a1: i64, a2: i64, a3: i32, a4: i32, a5: f32) -> i32; }
extern "C" { pub fn f_007301_rage_audgranularmix_calculatenearestloops_float_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_007303_rage_audgranularsubmix_stopphys(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007304_rage_audgranularsubmix_startphys(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007306_rage_audgranularmix_generateframe_float_ptr_unsigned_int_float_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_007307_rage_audgranularsubmix_generateframefrompcmdata_float_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: f32, a5: f32, a6: i32); }
extern "C" { pub fn f_007308_rage_audgranularsubmix_generateframefromcompresseddata_float_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: f32, a5: f32); }
extern "C" { pub fn f_007310_rage_audgrainplayer_updatevirtualisedvariableblockvalues_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007311_rage_audgranularsubmix_audgranularsubmix_rage_audwaveref_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_007315_rage_atfixedarray_rage_audgranularmix_audsyncloopdefinition_12_d(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007321_rage_audgranularsubmix_selectnewgrain_rage_audgranularsubmix_aud(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_007322_rage_audpcmsourceinterface_addref_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007323_rage_audpcmsourceinterface_release_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007325_rage_audpcmsourceinterface_allocate_rage_audpcmsourcetypeid(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_007326_rage_audpcmsourceinterface_setparam_unsigned_int_unsigned_int_fl(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_007327_rage_audpcmsourceinterface_setparam_unsigned_int_unsigned_int_un(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_007328_rage_audpcmsourceinterface_writecustomcommandpacket_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007329_rage_audpcmsourceinterface_stopandrelease_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007330_rage_audpcmsourceinterface_setsubmixoutputvolumes_int_unsigned_i(ph: *mut Phantom, a0: i32, a1: i32, a2: u128); }
extern "C" { pub fn f_007334_rage_audmixersubmix_addoutput_unsigned_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_007336_rage_audmixersubmix_setoutputvolumes_unsigned_int_float_const_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_007337_rage_audmixersubmix_seteffect_int_rage_auddspeffect_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_007338_rage_audmixersubmix_seteffectparam_int_unsigned_int_float_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32); }
extern "C" { pub fn f_007339_rage_audmixersubmix_seteffectparam_int_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_007340_rage_audmixersubmix_setflag_rage_audmixersubmix_submixflag_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007341_rage_audmixersubmix_convertbuffertoformat_rage_auddspeffectbuffe(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007343_rage_audwaveplayer_submitblock_rage_audstreamingwaveblock_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_007358_rage_audwaveplayer_addtocurrentposition_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007361_rage_audmediareader_close(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007397_rage_audentity_dtoraudentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007398_rage_audentity_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007399_rage_audentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007405_rage_audentity_setinitialsoundparams_rage_audsound_ptr_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007406_rage_audentity_createsound_localreference_rage_audmetadataref_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007409_rage_audentity_createsound_persistentreference_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007411_rage_audentity_addbatchedsoundrequest_rage_audmetadataref_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007416_rage_audentity_createandplaysound_persistent_unsigned_int_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007426_rage_vfxbrokenfraginfo_dtorvfxbrokenfraginfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007429_rage_vfxutil_inittxd_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007430_rage_vfxutil_shutdowntxd_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007431_rage_decalhelper_calcmatrix_rage_mat34v_ref_bool_ptr_rage_fwenti(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_007432_rage_decalbucket_shutdownimmediate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007433_rage_decalinst_shutdownimmediate_rage_decalinst_shutdownimmediat(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007434_rage_decalbucket_update_beforeadd(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007435_rage_decalbucket_update_afteradd_float_rage_decalrenderpass_rage(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_007436_rage_decalshader_static_setperbucketvarslocal_rage_grctexture_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32, a9: i32, a10: i64, a11: f32, a12: f32, a13: i64, a14: i64, a15: f32); }
extern "C" { pub fn f_007437_rage_decalrenderstaticbatcher_finishcurrentbatch(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007438_rage_decalbucket_getboundsphere_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007440_rage_decalbucket_preprocessoverlays_rage_fwentity_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_007441_rage_decalbucket_preprocessliquidpools_rage_decalusersettings_re(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007442_rage_decalbucket_removeinrange_rage_vec3v_const_ref_rage_scalarv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_007443_rage_decalbucket_ispointinliquid_signed_char_ref_rage_vec3v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: f32) -> i32; }
extern "C" { pub fn f_007444_rage_decalbucket_reapplyasstaticdecal_rage_decalinst_ptr_void_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_007445_rage_decalmanager_addsettings_rage_fwentity_ptr_rage_decaluserse(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_007446_rage_decalclipper_init_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_007447_rage_decalbucket_processmarkedfordelete_rage_decalmethod(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007457_rage_decalmanager_wash_float_rage_fwentity_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i64, a3: i32); }
extern "C" { pub fn f_007460_rage_decalinst_init_rage_decalsettings_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007461_rage_decallodsystem_converttodynamicdecal_rage_decalinst_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_007462_rage_asyncpool_rage_decalbucket_rage_slistdefaultalignptrconvert(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007463_rage_decalmanager_createdx11buffers_rage_decalmanager_reinitarg(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007464_rage_decalmanager_destroydx11buffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007468_rage_decalshader_static_lookupvars(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007469_rage_decalshader_dynamic_lookupvars(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007473_bool_rage_decalmanager_ondeletecmp_rage_ondeletecmpphinst_rage_o(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007475_rage_decalmanager_synchronize(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007476_rage_asyncpool_rage_decalbucket_rage_slistdefaultalignptrconvert(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_007477_rage_decalmanager_mergebuckets_rage_decalrenderpass_rage_decalme(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_007479_rage_decalmanager_remove_rage_fwentity_const_ptr_int_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_007481_rage_decalmanager_render_dynamic_rage_decalrenderpass_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_007482_rage_decalshader_dynamic_setvarsglobal(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007483_rage_decalmanager_ondelete_rage_fwentity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007484_bool_rage_decalmanager_ondeletecmp_rage_ondeletecmpfwentity_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007485_rage_decalmanagerasync_addinsttobucket_rage_decalasynctaskdescba(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i32, a11: i32, a12: i32) -> i64; }
extern "C" { pub fn f_007486_rage_decalmanager_processinstasync_rage_decalsettings_ref_rage_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007487_rage_decalmanager_kickprocessinstasync_rage_decalasynctasktype_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i64, a8: i64, a9: i32, a10: i64, a11: i64, a12: i64, a13: i64, a14: i64); }
extern "C" { pub fn f_007488_rage_decalmanager_kickprocessdrawables_rage_decalsettings_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64, a8: i32, a9: i64, a10: i64, a11: i64, a12: i64, a13: i64); }
extern "C" { pub fn f_007489_rage_decalmanager_packskeletons_unsigned_int_ptr_rage_vec4v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i32, a10: i64, a11: i64) -> i32; }
extern "C" { pub fn f_007490_rage_decalmanager_kickprocessinstasync_rage_decalasynctasktype_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_007492_rage_decalmanagerasync_processinst_char_const_ptr_rage_decalasyn(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i64); }
extern "C" { pub fn f_007493_rage_decalmanagerasync_processbound_char_const_ptr_rage_decalasy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i32, a13: i32, a14: i32, a15: i64); }
extern "C" { pub fn f_007505_rage_decalmanagerasync_getinsts_rage_decalinst_ptr_ptr_rage_deca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_007506_rage_decalmanagerasync_processboundpoly_char_const_ptr_rage_deca(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i32, a12: i32, a13: i64); }
extern "C" { pub fn f_007507_rage_decalmanagerasync_processtri_rage_decalsettings_const_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i64, a13: i64, a14: i64, a15: i64, a16: i32, a17: i64); }
extern "C" { pub fn f_007516_rage_ptfxassetstore_outputloadedassets(ph: *mut Phantom); }
extern "C" { pub fn f_007522_rage_ptfxassetstore_outputassetinforefchanged_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007525_rage_atarray_rage_ptfxattachinfo_0_unsigned_short_destruct_rage(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007527_rage_ptfxattach_detach_rage_ptfxattachinfo_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007528_rage_ptfxattach_attach_rage_ptxeffectinst_ptr_rage_fwentity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_007535_rage_ptfxwrapper_geteffectinst_rage_ptxeffectrule_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007537_rage_ptfxmanager_removeptfx_rage_ptxeffectinst_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007538_rage_ptfxreg_returnregfxinsttopool_rage_ptfxreginst_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007540_rage_ptfxdebug_outputactiveeffectinsts(ph: *mut Phantom); }
extern "C" { pub fn f_007547_rage_ptfxmanager_addptfxoverrideinfo_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_007553_rage_atarray_rage_ptfxhistoryinfo_0_unsigned_short_destruct_rage(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007557_rage_fwassetstore_rage_ptxfxlist_rage_ptxfxlistdef_dtorfwassetst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007558_rage_ptfxreg_register_void_const_ptr_int_bool_rage_ptxeffectrule(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_007560_rage_ptfxmanager_gettriggeredinst_rage_atnonfinalhashstring_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_007561_rage_ptfxmanager_queryptfxoverride_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007578_rage_ptfxscript_remove_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_007606_rage_ptxu_fogvolume_calculatefogvolumeparams_rage_ptxpointsb_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_007635_rage_atmapentry_void_ptr_unsigned_int_const_ref_rage_atmap_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007636_rage_atmapentry_unsigned_int_rage_fwidinstanceinfo_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007641_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_007651_rage_fwprofanityfilter_dtorfwprofanityfilter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007656_rage_fwprofanityfilter_substitutecharacters_char_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007657_rage_fwprofanityfilter_checkforprofanetermsraw_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007661_rage_fwptrlist_ismemberoflist_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007663_rage_fwquadtreenode_dtorfwquadtreenode(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007665_rage_fwquadtreenode_getallnodupes_rage_fwptrlistsinglelink_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007666_rage_fwquadtreenode_getallmatching_rage_fwrect_const_ref_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007667_rage_fwquadtreenode_forallmatching_rage_fwrect_const_ref_rage_fw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007669_rage_fwquadtreenode_additemrestrictdupes_void_ptr_rage_fwrect_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007670_rage_fwquadtreenode_additem_void_ptr_rage_fwrect_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007671_rage_fwquadtreenode_deleteitem_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007672_rage_fwquadtreenode_deleteallitems(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007673_rage_fwquadtreenode_getentrycount_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007674_rage_fwquadtreenode_getnodecount_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007683_rage_fwkdop18_buildverticessoa_rage_fwkdop18verticessoa_ref_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007684_rage_fwkdop18_debugdrawbound_rage_color32_const_ref_rage_grcview(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_007686_rage_polyclipedge_rage_vec3v_ptr_rage_vec3v_const_ptr_int_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_007688_rage_polycliptofrustumwithoutnearplane_rage_vec3v_ptr_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_007690_rage_planesetdebugdraw_rage_vec4v_const_ptr_int_rage_color32_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_007696_rage_fwportalcorners_calcboundingbox_rage_spdaabb_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007698_rage_fwportalcorners_calccloseness_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_007700_rage_fwportalcorners_calcdistancetopoint_rage_vec3v_const_ref_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007707_rage_fwaudioentity_handleanimevent_rage_audanimevent_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007726_rage_ptxu_light_dtorptxu_light(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007746_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007747_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007771_rage_netobjectmgrbase_getnetworkobject_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007772_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_007773_rage_netobjectmgrbase_checknetworkobjectmapforduplicates_char_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007774_rage_netobjectreassignmgr_findnetworkobject_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007775_rage_netobjectmgrbase_getnetworkobjectslow_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007780_rage_netplayermgrbase_sendbuffer_rage_netplayer_const_ptr_void_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_007791_rage_netlogimpl_netlogimpl_char_const_ptr_rage_logopentype_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_007795_rage_netlogsplitter_log_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007796_rage_netarrayhandlerbase_resetunackedsplitupdateflagsforplayer_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007798_rage_netarraymanager_logarrayelementsmsgheader_rage_netarrayhand(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_007799_rage_netbandwidthstatistics_addbandwidthout_short_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_007800_rage_netarraymanager_sendupdatemessage_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007803_rage_networklogutils_writelogevent_rage_netlogginginterface_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007804_rage_networklogutils_writemessageheader_rage_netlogginginterface(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_007806_rage_netarraymanager_logarrayelementsmsgrejection_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007807_rage_netarraymanager_logarrayelementssplitupdateack_rage_netplay(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_007808_rage_netarraymanager_addarrayhandlertobatch_rage_netarrayhandler(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007809_rage_netbandwidthstatistics_addbandwidthin_short_unsigned_int_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_007817_rage_netarrayhandlerbase_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007818_rage_netarrayhandlerbase_initialisesyncdatabuffers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007822_rage_netarrayhandlerbase_resetemptyelements(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007823_rage_netarrayhandlerbase_playerhasjoined_rage_netplayer_const_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007824_rage_netsyncdatabase_initialiseplayersyncdatafordataunit_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_007825_rage_netarrayhandlerbase_playerhasleft_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007827_rage_netarrayhandlerbase_gettotalsizeofupdatedata_rage_netplayer(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_007828_rage_netarrayhandlerbase_getsizeofmessageupdatedata_rage_netplay(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_007829_rage_netarrayhandlerbase_writeupdate_rage_netplayer_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_007830_rage_netsyncdatabase_copycurrentstatedata_unsigned_int_rage_datb(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_007831_rage_netarrayhandlerbase_readupdate_rage_netplayer_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_007838_rage_netarrayhandlerbase_handlechangeofarbitration_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007843_rage_netarrayhandlerbase_recalculatedirtyelements(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007848_rage_neteventmgr_checkforspaceinpool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007849_rage_neteventmgr_prepareevent_rage_netgameevent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007852_rage_netscriptbroadcastdatahandlerbase_playerhasjoined_rage_netp(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007864_rage_netplayerarrayhandler_rage_cbroadcastdataelement_rage_netsc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007867_rage_netplayerarrayhandler_rage_cbroadcastdataelement_rage_netsc(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_007871_rage_netscriptbroadcastdatahandlerbase_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007880_rage_netbandwidthmgr_settargetupstreambandwidth_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007882_rage_netbandwidthmgr_getbandwidththrottlingrequired_rage_netplay(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_007938_rage_neteventmgr_sendpackedeventsmessage_rage_netplayer_const_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007939_rage_neteventmgr_sendpackedeventreliablesmessage_rage_netplayer(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007941_rage_packedeventdata_serialise_rage_datimportbuffer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007942_rage_neteventmgr_handleevent_rage_netgameevent_ptr_rage_datbitbu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_007943_rage_cmsgpackedevents_addevent_rage_netplayer_const_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64, a8: i32) -> i32; }
extern "C" { pub fn f_007944_rage_cmsgpackedeventreliablesmsgs_addeventack_rage_netplayer_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_007945_rage_cmsgpackedeventreliablesmsgs_addeventreply_rage_netplayer_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_007948_rage_packedeventdata_serialise_rage_datexportbuffer_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007952_rage_csyncdatabase_serialiseobjectid_unsigned_short_ref_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007954_rage_netobjectidmgr_freeobjectid_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007955_rage_netobjectidmgr_freeobjectidprivate_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007956_rage_netobjectidmgr_releaseexcessobjectids(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007967_rage_netlogsplitter_writedatavalue_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007995_rage_matrix34_toeulersxyz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008009_rage_fwhttpquery_dtorfwhttpquery(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008012_rage_fwhttpquery_addtohttpqueryqueue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008013_rage_fwhttpquery_writebuffertofile_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008020_rage_packedlogdatawriter_writestring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008021_rage_packedlogdatawriter_addwritedatavaluedata_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_008022_rage_packedlogdatawriter_checkforframedatachange(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008023_rage_packedlogdatawriter_writestringfromtable_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008024_rage_packedlogdatawriter_writestringtableid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008025_rage_packedlogdatawriter_addstandardlogdata_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008026_rage_packedlogdatawriter_addmessageheaderdata_bool_unsigned_shor(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_008027_rage_packedlogdatawriter_addplayertextdata_char_const_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_008028_rage_packedlogdatawriter_addlogeventdata_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_008029_rage_packedlogdatawriter_addnodeheaderdata_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008031_rage_packedlogdatareader_getstringfromtable_char_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_008032_rage_netlogimpl_openinternal(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008033_rage_netlogimpl_dtornetlogimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008034_rage_netlogimpl_closeinternal_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008037_rage_netlogimpl_setupfilename_char_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008039_rage_netlogimpl_trytoswapbuffers(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008040_rage_netlogimpl_handlebufferswapfailure(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008044_rage_netlogimpl_trytoaddtostringtable_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008056_rage_networklogutils_writeplayertext_rage_netlogginginterface_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_008060_rage_netremotelog_begincompressinglog(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008068_rage_netobject_setsynctreeupdatelevelandbatch_unsigned_char_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_008083_rage_netobject_issyncedwithallplayers_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008085_rage_netobject_getclonedstate_unsigned_int_ref_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_008098_rage_netobject_updateinscopestate_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008120_rage_netobjectidmgr_trytoallocateinitialobjectids(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008121_rage_netobjectidmgr_iswaitingforobjectids_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008132_rage_packedclonesyncacksmsg_addsyncacks_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_008133_rage_packedclonesyncacksmsg_writependingsyncackstobitbuffer_unsi(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008138_rage_packedreliablesmsg_packedreliablesmsg(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008139_rage_packedreliablesmsg_addcreate_rage_netplayer_const_ref_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_008143_rage_packedreliablesmsg_addremove_rage_netplayer_const_ref_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_008145_rage_reassignconfirmmsg_writetologfile_bool_rage_netplayer_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_008146_rage_reassignresponsemsg_addpeernegotiationresponse_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008147_rage_reassignresponsemsg_writetologfile_bool_rage_netplayer_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_008149_rage_netobjectmgrbase_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008154_rage_netobjectmgrbase_destroynetworkobject_rage_netobject_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008155_rage_netsynctree_update_rage_netsynctreetargetobject_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_008156_rage_netsynctree_decreaseobjectcount_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_008157_rage_netsynctree_increaseobjectcount_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_008158_rage_netobjectreassignmgr_sendnegotiatemsg_rage_netplayer_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008159_rage_netobjectmgrmessagehandler_sendpackedsyncackmessage_rage_ne(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008160_rage_netobjectmgrmessagehandler_sendpackedreliablemessage_rage_n(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008161_rage_netobjectmgrmessagehandler_sendconnectionsyncmessage_rage_n(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008164_rage_reassignmentprocessinfo_findinfo_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008167_rage_netobjectmgrbase_assertobjectonlist_rage_atdlist_rage_netob(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008168_rage_netobjectmgrbase_removeobjectfromsyncmessageinfoqueues_unsi(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008169_rage_netsyncdatanode_update_rage_netsynctreetargetobject_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008170_rage_addnodeandexternaldependentnodes_rage_netsyncdatanode_ptr_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_008177_rage_netobjectreassignmgr_destroyobjectsnottobereassigned_unsign(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008179_rage_netsynctree_nodedatasent_rage_netsynctreetargetobject_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_008182_rage_clonesyncmsg_updatenumobjectscount(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008194_rage_netobjectmgrbase_processsyncackinsyncmessageinfoqueues_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_008198_rage_netobjectmgrmessagehandler_addsyncacks_rage_netplayer_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i64); }
extern "C" { pub fn f_008204_rage_reassignmentprocessinfo_clearplayerinvolved_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008205_rage_reassignmentprocessinfo_logreassignmentstate_rage_netloggin(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008206_rage_reassignmentprocessinfo_setgotack_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008207_rage_netobjectreassignmgr_getimportantstatechecksum_rage_netobje(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008209_rage_netobjectreassignmgr_restartreassignnegotiation_unsigned_lo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008210_rage_netobjectreassignmgr_addobjecttoreassignlist_rage_netobject(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_008211_rage_netobjectreassignmgr_preparefornegotiation_rage_netplayer_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008214_rage_netsynctree_forcesendofsyncupdatenodes_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_008215_rage_netsyncdatanode_forcesend_unsigned_int_unsigned_int_rage_ne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_008216_bool_rage_netinterface_sendresponse_rage_reassignresponsemsg_rag(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008221_rage_netblenderlininterp_dologarithmicblend_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008222_rage_netblenderlininterp_getcurrentvelocitychangedata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008223_rage_netblenderlininterp_getpositionmaxforupdatelevel_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_008224_rage_netblenderlininterp_getpredictedaccelerationvector_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008233_rage_netblenderlininterp_getpredictedposition_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008247_rage_netplayer_setnonphysicaldata_rage_nonphysicalplayerdatabase(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008265_rage_atdelegator2_void_rage_netplayer_ptr_rage_netrequest_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008267_rage_netplayermgrbase_shutdownprivate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008271_rage_netplayermgrbase_updateplayerarrays(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008307_rage_netsyncdatanode_forcesendtoplayer_unsigned_char_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_008606_bool_rage_datbitbuffer_writeint_signed_char_signed_char_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008644_rage_packedmessagetype_serialise_rage_datimportbuffer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008681_rage_packedmessagetype_serialise_rage_datexportbuffer_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008682_rage_fwbasepool_fwbasepool_int_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_008683_rage_fwconfigmanager_getsizeofpool_unsigned_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008688_rage_fwbasepool_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008689_rage_fwscriptguid_createguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008691_rage_fwscriptguid_deleteguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008693_rage_fwscriptguid_getbasefromguid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_008704_rage_scripthandlernetcomponent_getslotparticipantisusing_rage_ne(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008746_unsigned_int_rage_netmessage_readheader_rage_datbitbuffer_unsign(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008764_bool_rage_datbitbuffer_readint_int_int_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_008765_bool_rage_netmessage_writeheader_rage_datexportbuffer_unsigned_i(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_008781_rage_scripthandlernetcomponent_isplayeraparticipant_rage_netplay(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008866_rage_atmapentry_unsigned_int_int_const_ref_rage_atmap_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008909_rage_fwmetadatastore_addslot_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008912_rage_fwbasepool_new_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008927_rage_fwtimer_startlostfocuspause(ph: *mut Phantom); }
extern "C" { pub fn f_008932_rage_fwassetstorebase_isdirty_rage_strlocalindex_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008933_rage_fwassetstorebase_cleardirtyflag_rage_strlocalindex(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008937_rage_fwdescpool_new_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008938_rage_fwdescpool_defragpartial(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008940_rage_fwdescpool_delete_rage_fwdescpoolalloc_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008941_rage_fwdescpool_resize_rage_fwdescpoolalloc_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_008942_rage_fwdescpool_defragpartialnew(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008943_rage_fwbasepool_fwbasepool_char_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_008944_rage_fwbasepool_poolfullcallback(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008945_rage_fwbasepool_forall_bool_ptr_void_ptr_void_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008946_rage_fwbasepool_fwbasepool_int_unsigned_char_ptr_unsigned_char_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_008948_rage_fwbasepool_initandallocate_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008949_rage_fwbasepool_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008950_rage_fwbasepool_getjustindex_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008953_rage_fwknownrefholder_fwknownrefholder_void_ptr_ptr_rage_fwknown(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_008996_rage_fwbasepool_dtorfwbasepool_14863(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009048_rage_crproperty_getattribute_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009207_rage_crskeletondata_findbonedata_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009211_rage_crskeletondata_computeobjecttransform_int_rage_mat34v_ref_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009216_rage_crskeletondata_convertbonenametoid_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009219_rage_crskeleton_dtorcrskeleton(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009233_rage_crskeleton_getglobalmtx_unsigned_int_rage_mat34v_ref_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009236_rage_crskeleton_getmemorysize_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009273_rage_grmmodel_grmmodel(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009276_rage_grmmodel_clonewithnewvertexdata_rage_grmmodel_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64); }
extern "C" { pub fn f_009283_rage_grmmodel_getgeometry_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009291_rage_grmshader_begindraw_rage_grmshader_edrawtype_bool_rage_grce(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_009293_rage_grmshader_bind_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009294_rage_grmshader_unbind_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009295_rage_grmshader_enddraw_const(ph: *mut Phantom); }
extern "C" { pub fn f_009312_rage_grmshader_setforcedtechniquegroupid_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009314_pixend(ph: *mut Phantom); }
extern "C" { pub fn f_009317_rage_grmshader_load_char_const_ptr_rage_fitokenizer_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_009341_rage_grmshadergroup_grmshadergroup_rage_grmshadergroup_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_009393_rage_pftimebars_init_char_const_ptr_int_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32); }
extern "C" { pub fn f_009401_rage_pftimebars_insertblock_char_const_ptr_float_bool_float_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32); }
extern "C" { pub fn f_009417_pixbegin_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009428_rage_pftimebarmgr_init_char_const_ptr_int_float_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_009444_pixisgpucapturing(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_009447_rage_pfdrawitembase_beginmanager(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009448_rage_pfdrawitembase_end(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009504_rage_grcfvf_gettangentchannel_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009509_rage_grcfvf_getoffsetnoassert_rage_grcfvf_grcfvfchannels_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009531_rage_grctexturedx11_grctexturedx11_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009537_rage_grceffect_init_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009540_rage_grceffect_lookupvar_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009545_rage_grcstateblock_setblendstate_rage_grcstateblock_grcblendstat(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009555_rage_grcstateblock_setrasterizerstate_rage_grcstateblock_grcrast(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009556_rage_grcbatcher_reserve_unsigned_int_unsigned_long(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_009559_rage_grcdevice_clear_bool_rage_color32_bool_float_bool_unsigned(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: f32, a4: i32, a5: i32); }
extern "C" { pub fn f_009560_rage_grcstateblock_setdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009561_rage_grcbegin_rage_grcdrawmode_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009562_rage_grcvertex_float_float_float_float_float_float_rage_color32(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: f32, a8: f32); }
extern "C" { pub fn f_009563_rage_grcend(ph: *mut Phantom); }
extern "C" { pub fn f_009587_rage_grclightstate_setenabled_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_009603_rage_grcviewport_setworldidentity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009608_rage_grcviewport_grcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009613_rage_grcviewport_dtorgrcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009614_rage_grcviewport_setcurrent_rage_grcviewport_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009615_rage_grceffect_lookupglobalvar_char_const_ptr_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009634_rage_grcviewport_setcurrentworldidentity(ph: *mut Phantom); }
extern "C" { pub fn f_009636_rage_grcviewport_setcameramtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009638_rage_grcviewport_isspherevisible_rage_vec4v_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009646_rage_grcviewport_transform_rage_vec3v_const_ref_float_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009648_rage_grcviewport_reversetransform_float_float_rage_vec3v_ref_rag(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i64); }
extern "C" { pub fn f_009649_rage_grcworldidentity(ph: *mut Phantom); }
extern "C" { pub fn f_009650_rage_grcbindtexture_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009653_rage_grcviewport_op_rage_grcviewport_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009670_rage_grcsetup_setpaused_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009681_rage_grcresourcecache_queueforcaching_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009685_rage_grcsetup_resetticker(ph: *mut Phantom); }
extern "C" { pub fn f_009691_rage_grcstateblock_default(ph: *mut Phantom); }
extern "C" { pub fn f_009693_rage_grcgputimer_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009697_rage_grcgputimer_stop(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_009698_rage_grcdraw2dtext_float_float_char_const_ptr_bool_float_float_r(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i32, a4: f32, a5: f32, a6: i64); }
extern "C" { pub fn f_009721_rage_grcstateblock_doflush(ph: *mut Phantom); }
extern "C" { pub fn f_009723_rage_grcdevice_setvertexdeclaration_rage_grcvertexdeclaration_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009728_rage_grcvertexdeclaration_release_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009733_rage_grcdrawline_rage_vector3_const_ref_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009743_rage_grcdrawarc_float_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: f32, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32); }
extern "C" { pub fn f_009744_rage_grcdrawcapsule_float_float_rage_matrix34_const_ref_int_bool(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i32, a4: i32, a5: f32); }
extern "C" { pub fn f_009748_rage_grcdrawpolygon_rage_vector3_const_ptr_int_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009750_rage_grcdrawpolygon_rage_vector3_const_ptr_int_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009752_rage_grcworldmtx_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009754_rage_grclighting_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_009762_rage_grcdrawlabelf_rage_vector3_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009769_rage_grcvertexbuffereditor_grcvertexbuffereditor_rage_grcvertexb(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_009771_rage_grcvertexbuffereditor_dtorgrcvertexbuffereditor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009781_rage_grcvertexbuffereditor_set_rage_grcvertexbuffer_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009788_rage_grcindexbuffer_lockro_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009789_rage_grcindexbuffer_unlockwo_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009814_rage_grceffect_setvar_rage_grcinstancedata_ref_rage_grceffectvar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009831_rage_grccbuffer_init_internal_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009859_rage_grceffect_setglobalfloatcommon_int_float_ptr_int_rage_grcef(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_009867_rage_grcinstancedata_dtorgrcinstancedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009872_rage_grceffect_setglobalvar_rage_grceffectglobalvar_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009874_rage_grceffect_setglobalvar_rage_grceffectglobalvar_float_const(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_009875_rage_grceffect_setlocalcommon_rage_grcinstancedata_ref_rage_grce(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_009881_rage_grceffect_setglobalvar_rage_grceffectglobalvar_rage_grctext(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009886_rage_grceffect_pushsamplerstate_rage_grcinstancedata_ref_rage_gr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009887_rage_grceffect_popsamplerstate_rage_grcinstancedata_ref_rage_grc(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009893_rage_grcstateblock_setdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009894_rage_grcstateblock_setblendstate_rage_grcstateblock_grcblendstat(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_009897_rage_grcgputimestamp_dtorgrcgputimestamp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009903_rage_grcstateblock_setstates_rage_grcstateblock_grcrasterizersta(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_009951_rage_grcvertexbufferd3d11_lockinternal_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009952_rage_grcvertexbufferd3d11_unlockinternal_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009961_rage_grcvertexprogram_bind_rage_grcvertexdeclaration_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009964_rage_grcresourcecache_getinstance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_009976_rage_grcresourcecache_getstreamingmemory_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009979_rage_grcresourcecache_setextramemory_rage_memorytype_long(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009986_rage_grcdebugdraw_adddebugoutputex_bool_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_009993_rage_grcdevice_cpufreefence_rage_grcfencehandle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009995_rage_grcdevice_disablescissor(ph: *mut Phantom); }
extern "C" { pub fn f_009997_rage_grcdevice_lockcontext(ph: *mut Phantom); }
extern "C" { pub fn f_009999_rage_grcdevice_unlockcontext(ph: *mut Phantom); }
extern "C" { pub fn f_010000_rage_grcdevice_lockdeviceresetavailability(ph: *mut Phantom); }
extern "C" { pub fn f_010001_rage_grcdevice_unlockdeviceresetavailability(ph: *mut Phantom); }
extern "C" { pub fn f_010003_rage_grcdevice_setscissor_int_int_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_010006_rage_grcdevice_setprogramresources(ph: *mut Phantom); }
extern "C" { pub fn f_010009_rage_grcdevice_setclipplaneenable_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010010_rage_grcdevice_setclipplane_int_rage_vec4v_const_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_010019_rage_grcdevice_checkdevicestatus(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_010020_rage_grcdevice_isready(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_010027_rage_grcdevice_beginconditionalrender_iunknown_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010028_rage_grcdevice_endconditionalrender_iunknown_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_010031_rage_grcdevice_drawprimitive_rage_grcdrawmode_rage_grcvertexdecl(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010077_rage_grctexturedx11_updategpucopy_withthreadsyncing_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_010147_rage_grcbufferd3d11_updategpucopy_void_ptr_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010154_rage_batcherlock_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010161_rage_grcdebugdraw_startrecording_rage_grcdebugdraw_lingerbuffer(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_010163_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010164_rage_grcdebugdraw_line_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010166_rage_grcdebugdraw_quad_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010167_rage_grcdebugdraw_poly_rage_vec2v_const_ref_rage_vec2v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010168_rage_grcdebugdraw_cross_rage_vec2v_const_ref_float_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_010169_rage_grcdebugdraw_circle_rage_vec2v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010170_rage_grcdebugdraw_rectaxisaligned_rage_vec2v_const_ref_rage_vec2(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010172_rage_grcdebugdraw_text_float_float_rage_color32_char_const_ptr_b(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i64, a4: i32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010174_rage_grcdebugdraw_clearsubrenderbatcher2_rage_grcdebugdraw_linge(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_010176_rage_grcdebugdraw_isexpectingsubrenderthreaddebugdraw_rage_grcde(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010180_rage_grcdebugdraw_quad_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_010181_rage_grcdebugdraw_poly_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010189_rage_grcdebugdraw_boxoriented_rage_vec3v_const_ref_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010202_rage_grcdebugdraw_discardrenderbuffers(ph: *mut Phantom); }
extern "C" { pub fn f_010205_rage_grcdebugdraw_allocatesubrenderbatcher_rage_grcdebugdraw_lin(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_010214_rage_grcdebugdraw_textfontpush_rage_grcfont_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010215_rage_grcdebugdraw_textfontpop(ph: *mut Phantom); }
extern "C" { pub fn f_010216_rage_grcdebugdraw_text_float_float_rage_dd_epixelcoordspace_rage(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i32, a4: i64, a5: i32, a6: f32, a7: f32); }
extern "C" { pub fn f_010217_rage_grcdebugdraw_printtoscreencoors_char_const_ptr_int_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010219_rage_grcdebugdraw_adddebugoutputex_bool_rage_color32_char_const(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_010220_rage_grcdebugdraw_adddebugoutput_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010221_rage_grcdebugdraw_adddebugoutput_rage_color32_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_010222_rage_grcdebugdraw_adddebugoutputseparator_int(ph: *mut Phantom); }
extern "C" { pub fn f_010223_rage_grcdebugdraw_getdebugoutputcolumcount(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_010224_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_010225_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010450_rage_grcinstancebufferbasic_dtorgrcinstancebufferbasic_17026(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010668_rage_rmcdrawable_rmcdrawable(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010670_rage_grcarray_rage_datowner_rage_grmmodel_init_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_010706_rage_rmcdrawable_place_rage_rmcdrawable_ptr_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010711_rage_rmclodgroup_getcurrentlod_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_010727_rage_mnbvhtraverseboxinternal_rage_mnbvhnode_const_ptr_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_010728_rage_mnbvhtraverserayinternal_rage_mnbvhnode_const_ptr_rage_vec3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_010731_rage_mnbvhbuildmultinodebvhtree_rage_spdmnbvhboundlist_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_010732_rage_mnbvhsimpleinterface_mnbvhsimpleinterface_rage_atarray_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_010735_rage_mnbvhsimpleinterface_dtormnbvhsimpleinterface(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010737_rage_spdaabb_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_011826_rage_fraginst_setmusclesfromskeleton_rage_crskeleton_const_ref_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_011964_rage_fragtype_getskeletondata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012021_rage_fragphysicslodgroup_getlodbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012182_rage_ptxeffectinst_init_rage_ptxeffectrule_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012183_rage_ptxeffectinst_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012188_rage_ptxevolutionlist_query_rage_ptxkeyframeprop_ptr_rage_scalar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_012189_rage_ptxeffectinst_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012191_rage_ptxeffectinst_finish_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012196_rage_ptxeffectinst_setbasemtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012201_rage_ptxeffectinst_setbasepos_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012202_rage_ptxeffectinst_setoffsetmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012203_rage_ptxeffectinst_setoffsetpos_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012204_rage_ptxeffectinst_setoffsetrot_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012207_rage_ptxeffectinst_setevovaluefromhash_rage_athashvalue_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_012211_rage_ptxkeyframe_addentry_rage_scalarv_const_ref_rage_vec4v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012214_rage_ptxkeyframe_setdefn_rage_ptxkeyframedefn_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012215_rage_ptxevolutionlist_setkeyframedefn_rage_ptxkeyframeprop_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012216_rage_ptxkeyframe_getvalue_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_012218_rage_ptxeffectrule_getkeyframeentryinfo_int_ref_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_012220_rage_ptxmanager_geteffectinstfrompool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012229_rage_ptxparticlerule_setparams_rage_ptxbehaviourparams_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_012231_rage_ptxemitterinst_sortpoints_rage_ptxpool_rage_ptxpointsb_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012237_rage_ptxdomain_removedefaultkeyframes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012238_rage_ptxemitterrule_getkeyframeentryinfo_int_ref_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_012245_rage_ptxparticlerule_getkeyframeentryinfo_int_ref_int_ref_int_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_012266_rage_ptxkeyframe_ptxkeyframe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012267_rage_ptxkeyframe_init_rage_scalarv_const_ref_rage_vec4v_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012270_rage_ptxkeyframe_gettime_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_012271_rage_ptxkeyframe_gettimev_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_012272_rage_ptxkeyframe_setentry_int_float_rage_vec4v_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64); }
extern "C" { pub fn f_012275_rage_ptxkeyframe_updatewidget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012276_rage_ptxkeyframe_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012277_rage_ptxkeyframeprop_dtorptxkeyframeprop(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012373_rage_ptxmanager_geteffectrule_rage_atnonfinalhashstring_rage_ptx(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_012402_rage_ptxfxlist_ptxfxlist_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012432_rage_ptxmanager_postloadfxlist_rage_ptxfxlist_ptr_rage_atnonfina(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_012433_rage_ptxmanager_geteffectrule_rage_atnonfinalhashstring_rage_atn(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_012440_rage_ptxbehaviour_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012721_rage_snpeer_getpeerinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012819_rage_clothcontroller_clothcontroller(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012820_rage_clothcontroller_unregisterrestinterface(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012821_rage_clothcontroller_load(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012822_rage_clothcontroller_save(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012823_rage_clothcontroller_instancefromtemplate_rage_clothcontroller_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012824_rage_clothcontroller_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012828_rage_phmorphcontroller_morphlevelone_int_int_rage_vec3v_ptr_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_012843_rage_environmentcloth_getcloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012844_rage_environmentcloth_getcloth_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012845_rage_environmentcloth_getoffset(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012848_rage_environmentcloth_environmentcloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012858_rage_environmentcloth_switchtolod_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012859_rage_phmorphcontroller_phmorphcontroller(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012860_rage_clothbridgesimgfx_instancefromdata_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012861_rage_environmentcloth_allocatedamagearray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012862_rage_environmentcloth_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012866_rage_environmentcloth_setreferenceddrawable_rage_rmcdrawable_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_012867_rage_environmentcloth_checkdistance_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012868_rage_environmentcloth_createbehavior(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012869_rage_environmentcloth_getbbminworldspace(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012870_rage_environmentcloth_getbbmaxworldspace(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012871_rage_environmentcloth_updatecloth_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_012874_rage_environmentcloth_copyverletdrawvertsforrope(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012875_rage_environmentcloth_controlvertex_int_rage_vec3v_const_ref_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_012887_rage_characterclothcontroller_skinmesh_bool_rage_matrix43_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_012888_rage_characterclothcontroller_recomputenormals_int_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_012892_rage_characterclothcontroller_setpackageindex_unsigned_char_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_012893_rage_characterclothcontroller_resetw(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012900_rage_charactercloth_charactercloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012910_rage_charactercloth_charactercloth_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012911_rage_charactercloth_setpose_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012912_rage_pack_core32_float_vector_4_ref_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128) -> u128; }
extern "C" { pub fn f_012921_rage_rmcropedrawable_draw_rage_matrix34_const_ref_unsigned_int_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013019_rage_bgglassmanager_getglassbreakable_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_013061_rage_bgbreakable_getpanetriangleinfo_unsigned_int_rage_vector3_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_013062_rage_bgbreakable_getnextpanetriangle_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013154_rage_rsttestedgelistcoverage_rage_mat44v_const_ref_rage_scalarv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64, a8: i32, a9: i64, a10: i64, a11: i64, a12: i32, a13: i32, a14: i32); }
extern "C" { pub fn f_013155_rage_rsttestaabbexactedgelist_bool_rage_mat44v_const_ref_rage_sc(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i32, a13: i32, a14: i32); }
extern "C" { pub fn f_013198_rage_rsttestaabbexact_rage_mat44v_const_ref_rage_scalarv_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_013200_rage_boxoccluder_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_013202_rage_occludemodel_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_013390_rage_netbandwidthrecorder_netbandwidthrecorder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013391_rage_netbandwidthrecorder_sampledata_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013394_rage_netbandwidthrecorder_recordoutboundpacket_rage_netsocketadd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_013395_rage_netbandwidthrecorder_recordinboundpacket_rage_netsocketaddr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_013407_rage_netconnectionmanager_unregisterchannelpacketrecorder_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_013408_rage_nethardware_getlocalipaddress_rage_netipaddress_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013409_rage_netconnectionmanager_registerchannelpacketrecorder_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_013456_rage_nettimestep_settime_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013462_rage_netconnectionmanager_delegate_dtordelegate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013464_rage_netconnectionmanager_removechanneldelegate_rage_netconnecti(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013467_rage_netaddress_ispeerrelayaddr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013468_rage_netaddress_gettargetaddress_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013539_rage_netconnectionmanager_closeconnection_int_rage_netclosetype(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013546_rage_netconnectionmanager_setglobalsendinterval_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013547_rage_netconnectionmanager_setchannelpolicies_unsigned_int_rage_n(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_013548_rage_netconnectionmanager_getchannelpolicies_unsigned_int_rage_n(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_013550_rage_netconnectionmanager_send_int_void_const_ptr_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_013554_rage_netconnectionmanager_getaddress_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_013558_rage_netconnectionmanager_getunackedcount_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013559_rage_netconnectionmanager_getoldestresendcount_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013561_rage_netconnectionmanager_addchanneldelegate_rage_netconnectionm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_013562_rage_netsocketaddress_tostring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013598_rage_netpeerid_op_rage_netpeerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013602_rage_netpeerid_op_rage_netpeerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013606_rage_netconnectionmanager_establishconnection_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_013610_rage_netconnectionmanager_isopen_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013612_rage_netconnectionmanager_ispendingopen_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013665_rage_nethttprequest_dtornethttprequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013668_rage_nethttprequest_nethttprequest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013673_rage_nethttprequest_geturi_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013677_rage_nethttprequest_init_rage_sysmemallocator_ptr_wolfssl_ctx_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013684_rage_nethttprequest_begin_rage_nethttpverb_char_const_ptr_rage_n(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64) -> i32; }
extern "C" { pub fn f_013687_rage_nethttprequest_beginget_char_const_ptr_rage_netsocketaddres(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_013688_rage_nethttprequest_beginpost_char_const_ptr_rage_netsocketaddre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_013689_rage_nethttprequest_commit(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013692_rage_netipaddress_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013695_rage_nethttprequest_addrequestheadervalue_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013697_rage_nethttprequest_addbasicauth_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013713_rage_nethttprequest_appendcontent_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013718_rage_nethttprequest_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013732_rage_nethttprequest_makedatestring_unsigned_long_char_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013859_rage_nethardware_getnattype(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_013882_rage_netipv4address_netipv4address(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013885_rage_netipaddress_format_char_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_013887_rage_netsocketaddress_netsocketaddress(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013889_rage_netsocketaddress_netsocketaddress_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013928_rage_netsocket_unregisterpacketrecorder_rage_netpacketrecorder_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014044_rage_nettcp_getdebugsslctx(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_014069_rage_nettimesync_hassynched_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014152_rage_netrequesthandler_dtornetrequesthandler(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014153_rage_nettransactor_removerequesthandler_rage_netrequesthandler_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014157_rage_netresponsehandler_dtornetresponsehandler(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014158_rage_netresponsehandler_cancel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014161_rage_nettransactor_dtornettransactor(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014162_rage_nettransactor_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014163_rage_nettransactor_init_rage_netconnectionmanager_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014165_rage_nettransactor_update_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014166_rage_nettransactor_addrequesthandler_rage_netrequesthandler_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_014167_rage_nettransactor_sendrequest_int_unsigned_int_ptr_void_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_014170_rage_nettransactor_sendresponse_rage_nettransactioninfo_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_014171_rage_netpacketrecorder_dtornetpacketrecorder_23323(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014259_rage_rltaskmanager_doaddtask_unsigned_long_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014260_rage_rltaskmanager_destroytask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014294_rage_rlgetposixtime_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014318_rage_rlgamerinfo_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014361_rage_rlros_getcredentials_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014362_rage_rlroscredentials_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014390_rage_rlgamerinfo_isremote_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014392_rage_rlgamerinfo_getgamerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014459_rage_rltaskbase_configure_rage_netstatus_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014609_rage_rlstats_cancel_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014681_rage_rlcloud_gettitlefile_char_const_ptr_rage_rlrossecurityflags(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64) -> i32; }
extern "C" { pub fn f_014683_rage_rlcloudgetfiletask_ptr_rage_rltaskmanager_createtask_rage_r(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014684_bool_rage_rltaskbase_configure_rage_rlcloudgetfiletask_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i64, a8: i32, a9: i64, a10: i64, a11: i64, a12: i64, a13: i64) -> i32; }
extern "C" { pub fn f_014912_rage_rlroshttpfilter_configure_int_char_const_ptr_rage_rlrossecu(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_014974_rage_rlugcgetcdncontenttask_rlugcgetcdncontenttask_rage_sysmemal(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014975_rage_rlugcgetcdncontenttask_configure_int_rage_datgrowbuffer_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_014982_rage_rlugcmetadata_composecloudabspath_rage_rlugccontenttype_cha(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_015042_rage_rlcloud_createurl_rage_atstringbuilder_ptr_rage_rlcloudname(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_015045_rage_rlcloud_createcompleteurl_rage_atstringbuilder_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_015048_rage_rlcloud_getmemberfile_int_rage_rlcloudmemberid_const_ref_ra(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64) -> i32; }
extern "C" { pub fn f_015050_rage_rlcloud_postmemberfile_int_rage_rlcloudonlineservice_char_c(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_015053_rage_rlcloud_deletememberfile_int_rage_rlcloudonlineservice_char(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_015772_rage_createrotatedmatrix_rage_vector3_const_ref_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015812_rage_phconstraintbase_getinstancea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015813_rage_phconstraintbase_getinstanceb(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015843_rage_phconstraintdistance_getworldposa(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015844_rage_phconstraintdistance_getworldposb(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015882_rage_phconstraintmgr_insert_rage_phconstraintbase_params_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015890_rage_phconstraintmgr_remove_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015891_rage_phconstraintmgr_gettemporarypointer_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015969_rage_phcollider_setreferenceframevelocity_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015997_rage_phsimulator_addinstbehavior_rage_phinstbehavior_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015998_rage_phsimulator_deleteobject_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_015999_rage_phsimulator_removeinstbehavior_rage_phinstbehavior_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016013_rage_phsimulator_addinactiveobject_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016018_rage_phsimulator_activateobject_rage_phinst_ptr_rage_phcollider(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_016022_rage_phsimulator_getgravityv(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016036_rage_pharchetype_setbound_rage_phbound_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016043_rage_pharchetype_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016046_rage_pharchetypephys_pharchetypephys(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016082_rage_phinst_phinst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016084_rage_phinst_setdtorcallback_void_ptr_rage_phinst_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016085_rage_phinst_setarchetypechangecallback_void_ptr_rage_phinst_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016087_rage_phinst_setarchetype_rage_pharchetype_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016088_rage_phinst_setinstflag_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016130_rage_phshapetest_rage_phshapeprobe_phshapetest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016151_rage_phshapetest_rage_phshapesphere_testinlevel(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_016367_rage_phbound_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016370_rage_phbound_setcgoffset_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016401_rage_phboundgeometry_phboundgeometry(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016422_rage_phboundgeometry_init_int_int_int_int_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_016431_rage_phboundcapsule_setcapsulesize_rage_scalarv_const_ref_rage_s(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016436_rage_phboundcapsule_phboundcapsule(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016439_rage_phboundcapsule_setcapsulesize_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32); }
extern "C" { pub fn f_016454_rage_phboundcomposite_phboundcomposite(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016455_rage_phboundcomposite_init_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016464_rage_phboundcomposite_setbound_int_rage_phbound_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016519_rage_phboundgeometry_calculateboundingbox_rage_vector3_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016521_rage_phbound_place_void_ptr_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016538_rage_phpolygon_inittriangle_unsigned_short_unsigned_short_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_016539_rage_phpolygon_set_rage_phpolygon_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016588_rage_phverletcloth_phverletcloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016594_rage_phverletcloth_detachvirtualbound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016597_rage_phverletcloth_instancefromdata_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016599_rage_phverletcloth_attachvirtualboundcapsule_float_float_rage_ma(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_016600_rage_phverletcloth_createvirtualbound_int_rage_mat34v_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016602_rage_phverletcloth_applyimpulse_rage_atbitset_const_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016603_rage_phverletcloth_initedgedata_float_const_ptr_rage_atarray_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016605_rage_phverletcloth_dynamicpinvertex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016606_rage_phverletcloth_dynamicunpinvertex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016607_rage_phverletcloth_dynamicunpinall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016608_rage_phverletcloth_removecustomedge_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016609_rage_phverletcloth_drawpickededges_rage_vector3_const_ptr_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_016610_rage_phclothdata_init_rage_vec3v_const_ptr_int_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_016611_rage_phverletcloth_load_void_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016612_rage_phverletcloth_save_void_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016624_rage_phclothdata_load_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016625_rage_phclothdata_save_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016626_rage_phclothdata_transformvertexpositions_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016627_rage_phclothverletbehavior_phclothverletbehavior(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016630_rage_phclothverletbehavior_updateverletbound_rage_phverletcloth(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016631_rage_phclothverletbehavior_updateropebound_rage_phverletcloth_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016661_rage_phwind_getlocalvelocity_rage_vec3v_const_ref_rage_vec3v_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_016696_rage_phverletcloth_detectandenforcelist_rage_mat34v_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016892_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectlocation(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016913_rage_phverletcloth_updateedgessoavec3vby8s_int_float_vector_4_pt(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_016914_rage_phverletcloth_computeclothboundingvolume(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016915_rage_phverletcloth_computeboundingvolume_rage_phbound_ptr_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016916_rage_phverletcloth_updatecloth_rage_mat34v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_016917_rage_phverletcloth_resetcollisioninst(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016918_rage_phverletcloth_applyairresistance_float_const_ptr_float_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016919_rage_phverletcloth_zerodeltapositions_withcompression(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016920_rage_clothcontroller_applysimulationtomesh_rage_grmgeometryqb_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_016981_rage_scrthread_getthread_rage_scrthreadid(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_016985_rage_scrthread_checkifmonitoredscriptarraysizehaschanged_char_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017228_rage_crclipplayer_setclip_rage_crclip_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017230_rage_crclipplayer_settime_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_017236_rage_crclipdictionary_place_rage_crclipdictionary_ptr_rage_datre(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017240_rage_crexpressions_crexpressions_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_017256_rage_crposematcher_crposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017257_void_rage_datresource_place_rage_crpmpointcloud_ptr_rage_crpmpoi(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017258_rage_crposematcher_dtorcrposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017264_rage_crposematcher_findbestmatch_rage_crskeleton_const_ref_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_017757_rage_crmtmotiontree_crmtmotiontree(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017759_rage_crmtmotiontree_dtorcrmtmotiontree(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017760_void_rage_crmtiterator_iteratebatch_64_false_rage_crmtnode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017761_rage_crmtmotiontree_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017762_rage_crmtcomposerdata_refresh(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017766_rage_crmtmotiontree_request_rage_crmtrequest_ref_rage_crmtobserv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_017770_rage_crmtobserver_detach(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017786_rage_crmtobserver_crmtobserver(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_017788_rage_crmtobserver_attach_rage_crmtobserver_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017851_rage_crmtnodefactory_destroynodefactoryallocator_rage_crmtnodefa(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_017859_rage_crmtmotiontreescheduler_waitoncomplete_rage_crmtmotiontree(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_017931_rage_crmtnodeframe_init_rage_crframe_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_017966_rage_crmtnoden_getweight_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_018022_rage_crmtobserver_dtorcrmtobserver_30310(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018091_rage_crmtrequestblend_dtorcrmtrequestblend(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018112_rage_crmtrequestclip_crmtrequestclip_rage_crclip_const_ptr_float(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_018113_rage_crmtrequestclip_dtorcrmtrequestclip(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018116_rage_crmtrequestclip_setlooping_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_018162_rage_mvparameterbuffer_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018163_rage_mvparameterbuffer_append_rage_mvparameterbuffer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018164_rage_mvparameterbuffer_deduplicate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018165_rage_mvparameterbuffer_compact(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018166_rage_mvcommandbuffer_preparebuffer_rage_mvmotionweb_ref_rage_mvn(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_018169_rage_mvparameterbuffer_generatefreecapacity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018172_rage_mvnetworkdef_load_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_018174_rage_mvmotionweb_release_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018177_rage_mvnetworkdef_mvnetworkdef(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018211_rage_mvparameterbuffer_mvparameterbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_018212_rage_mvparameterbuffer_dtormvparameterbuffer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018213_rage_mvparameterbuffer_finalize(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018214_rage_mvparameterbuffer_unfinalize(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018215_rage_mvfixedheap_freeinternal_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090180_silk_get_decoder_size(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090181_silk_resetdecoder(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090197_celt_decoder_get_size(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090198_opus_custom_decoder_ctl(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090204_opus_packet_get_samples_per_frame(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090205_opus_decode_frame(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_090207_opus_decoder_ctl(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090710_asinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090717_atof(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090741_time(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090755_exp2f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090757_expf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090801_localtime(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090805_log10f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090807_logf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090808_lrint(ph: *mut Phantom, a0: f64) -> i64; }
extern "C" { pub fn f_090814_memcmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090819_nanosleep(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090826_powf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090901_qsort_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_090910_srand(ph: *mut Phantom, a0: i32); }
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
extern "C" { pub fn f_090956_strtok(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090962_system(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090965_tanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090970_tolower(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090971_toupper(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090985_vsnprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090987_vsprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_091000_emscripten_builtin_malloc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_091002_emscripten_builtin_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091023_multi3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_091069_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
