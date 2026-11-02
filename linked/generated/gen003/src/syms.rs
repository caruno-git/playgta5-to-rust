//! Callee declarations for crate gen003.
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
extern "C" { pub fn f_000112_rage_syscriticalsectiontoken_trylock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000113_rage_syscriticalsectiontoken_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000114_rage_sysipccreatesema_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_000116_rage_sysipcwaitsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000118_rage_sysipcwaitsematimed_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_000119_rage_sysipcsignalsema_rage_sysipcsematag_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_000121_rage_sysipclockmutexcommand_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000122_rage_sysipcunlockmutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000125_rage_sysipcwaitevent_rage_sysipceventtag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000127_rage_sysipcsetevent_rage_sysipceventtag_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000128_rage_sysipccreatethread_void_ptr_void_ptr_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_000131_rage_sysipcwaitthreadexit_rage_sysipcthreadid_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000135_rage_sysipcsleep_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000183_rage_netsocket_sizeofpacket_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000184_rage_netsocket_nativeshutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000185_rage_netsocket_nativebind(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000186_rage_netsocket_nativegetreceivebufferqueuelength_unsigned_int_re(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_000187_rage_netsocket_nativegetportrangeforrandomselection_unsigned_sho(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000384_non_virtual_thunk_to_wasm_null_d3d_nullresource_wasm_null_d3d_st(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000609_rage_voicechat_nativepackchat_int_void_ptr_unsigned_int_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000644_rage_athashnextsize_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000645_rage_athash_const_char_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000649_rage_dattypestruct_addfield_unsigned_long_unsigned_long_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000659_rage_atstring_indexof_char_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_000666_rage_atstring_split_rage_atstring_ref_rage_atstring_ref_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_000667_rage_atstring_split_rage_atarray_rage_atstring_0_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_000675_rage_functorbase_functorbase_void_const_ptr_void_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_000680_rage_athashstringnamespacesupport_computehash_rage_athashstringn(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_000681_rage_athashstringnamespacesupport_addstring_rage_athashstringnam(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_000684_rage_athashstringnamespacesupport_trygetstring_rage_athashstring(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_000692_rage_datcallback_call_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_000704_rage_bkbank_pushgroup_char_const_ptr_bool_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_000707_rage_bkbank_popgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000806_rage_bkgroup_addslider_char_const_ptr_int_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000812_char_ptr_rage_formatf_char_128ul_char_ref_128ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000813_char_ptr_rage_formatf_char_64ul_char_ref_64ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000867_rage_bkgroup_addtitle_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000869_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000870_rage_bkgroup_addbutton_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000874_rage_bkgroup_addtext_char_const_ptr_int_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000882_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000884_rage_bkgroup_addtoggle_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000888_rage_bkgroup_addtoggle_char_const_ptr_unsigned_short_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_000894_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64, a8: i64, a9: i32) -> i64; }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000902_rage_bkgroup_addslider_char_const_ptr_unsigned_short_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000910_rage_bkgroup_addslider_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32, a6: i64, a7: i64); }
extern "C" { pub fn f_000911_rage_bkgroup_addslider_char_const_ptr_rage_vector3_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000925_rage_bkgroup_addvector_char_const_ptr_rage_vector2_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_000929_rage_bkgroup_addvector_char_const_ptr_rage_vector4_ptr_float_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000990_char_ptr_rage_formatf_char_512ul_char_ref_512ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001099_rage_autoidshutdown(ph: *mut Phantom); }
extern "C" { pub fn f_001100_rage_autoiddescriptor_autoiddescriptor_unsigned_int_char_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_001102_rage_autoiddescriptor_autoiddescriptor_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_001105_rage_autoidregistry_autoidregistry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001106_rage_autoidregistry_dtorautoidregistry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001107_rage_autoidregistry_register_rage_autoiddescriptor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001108_rage_autoidregistry_unregister_rage_autoiddescriptor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001109_rage_autoidregistry_getnamefromid_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001112_rage_datbase64_getmaxencodedsize_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_001113_rage_datbase64_getmaxdecodedsize_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001114_rage_datbase64_encode_unsigned_char_const_ptr_unsigned_int_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_001115_rage_datbase64_decode_char_const_ptr_unsigned_int_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_001116_char_ptr_rage_formatf_char_32ul_char_ref_32ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001119_rage_datcallback_datcallback_void_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001123_rage_datgrowbuffer_datgrowbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001124_rage_datgrowbuffer_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001125_rage_datgrowbuffer_dtordatgrowbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001126_rage_datgrowbuffer_init_rage_sysmemallocator_ptr_unsigned_int_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001128_rage_datgrowbuffer_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001129_rage_datgrowbuffer_getfihandle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001130_rage_datgrowbuffer_append_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001132_rage_datgrowbuffer_appendorfail_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001133_rage_datgrowbuffer_prependorfail_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001134_rage_datgrowbuffer_truncate_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001136_rage_datgrowbuffer_remove_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001137_rage_datgrowbuffer_remove_void_ptr_unsigned_int_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001138_rage_datgrowbuffer_getbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001140_rage_datbitbuffer_readunsigned_void_const_ptr_unsigned_int_ref_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001141_rage_datbitbuffer_writeunsigned_void_ptr_unsigned_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_001143_rage_datbitbuffer_datbitbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001147_rage_datbitbuffer_setreadonlybytes_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001148_rage_datbitbuffer_setreadwritebytes_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001149_rage_datbitbuffer_getnumbyteswritten_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001152_rage_datbitbuffer_getnumbytesread_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001153_rage_datbitbuffer_setnumbitsread_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001154_rage_datbitbuffer_skipbytes_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001156_rage_datbitbuffer_bumpreadcursor_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001158_rage_datbitbuffer_bumpreadwritecursor_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001161_rage_datbitbuffer_readuns_unsigned_long_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001163_rage_datbitbuffer_writeuns_unsigned_long_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001166_rage_datbitbuffer_readbool_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001167_rage_datbitbuffer_writebool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001168_rage_datbitbuffer_readbytes_void_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001169_rage_datbitbuffer_writebytes_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001170_rage_datbitbuffer_readstr_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001171_rage_datbitbuffer_writestr_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001172_rage_datbitbuffer_peekbytes_void_ptr_int_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_001173_rage_datcompress_unsigned_char_ptr_unsigned_int_unsigned_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_001174_rage_datdecompress_unsigned_char_ptr_unsigned_int_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_001186_rage_rijndael_setup_unsigned_char_const_ptr_int_int_rage_symmetr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001195_rage_aes_decrypt_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001197_rage_sha1_sha1(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001198_rage_sha1_clear_unsigned_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001199_rage_sha1_final_unsigned_char_ref_20(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001200_rage_sha1_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001201_rage_sha1_update_unsigned_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001204_rage_rsonwriter_rsonwriter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001205_rage_rsonwriter_init_char_ptr_unsigned_int_rage_rsonformat(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001206_rage_rsonwriter_rsonwriter_char_ptr_unsigned_int_rage_rsonformat(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001208_rage_rsonwriter_initnull_rage_rsonformat(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001209_rage_rsonwriter_resettocontinue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001210_rage_rsonwriter_begin_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001213_rage_rsonwriter_beginarray_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001214_rage_rsonwriter_end(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001215_rage_rsonwriter_undo_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001216_rage_rsonwriter_writeint_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001218_rage_rsonwriter_writeint64_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001219_bool_rage_rsonwriter_write_long_char_const_ptr_char_const_ptr_lo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_001220_rage_rsonwriter_writeuns_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001221_rage_rsonwriter_writeuns64_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001224_rage_rsonwriter_writestring_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001228_rage_rsonwriter_available_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001229_rage_rsonwriter_writebool_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001231_rage_rsonwriter_haserror_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001232_rage_rsonreader_rsonreader(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001233_rage_rsonreader_rsonreader_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001234_rage_rsonreader_init_char_const_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001235_rage_rsonreader_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001237_rage_rsonreader_hasmember_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001238_rage_rsonreader_getmember_char_const_ptr_rage_rsonreader_ptr_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001239_rage_rsonreader_getnextsibling_rage_rsonreader_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001240_rage_rsonreader_getfirstmember_rage_rsonreader_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001242_rage_rsonreader_hasname_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001246_rage_rsonreader_asint_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001247_rage_rsonreader_asint64_long_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001251_rage_rsonreader_asbool_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001252_rage_rsonreader_asatstring_rage_atstring_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001253_rage_rsonreader_asstring_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001257_rage_rsonreader_readint_char_const_ptr_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001259_rage_rsonreader_readuns_char_const_ptr_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001261_rage_rsonreader_readfloat_char_const_ptr_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001263_rage_rsonreader_readbool_char_const_ptr_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001264_rage_rsonreader_readstring_char_const_ptr_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_001266_rage_rsonreader_readstring_char_const_ptr_rage_atstring_ref_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001267_rage_rsonreader_checkname_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001269_rage_rsonreader_validatejson_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001274_rage_datsaxreader_datsaxreader(ph: *mut Phantom, a0: i64) -> i64; }
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
extern "C" { pub fn f_001541_rage_diagloggedprintln_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001551_rage_fprintf_rage_fistream_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001555_rage_diaglogf_assert_rage_diagchannel_const_ref_char_const_ptr_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001569_rage_atmap_unsigned_int_wchar_t_ptr_rage_atmaphashfn_unsigned_in(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001577_rage_fistream_open_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001580_rage_fiassetmanager_open_char_const_ptr_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001612_rage_atarray_rage_restservice_ptr_0_unsigned_short_grow_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001615_rage_restservicemanager_removeanddeleteservice_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001627_rage_fiassetmanager_exists_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001629_rage_fiassetmanager_getstackedpath_char_ptr_int_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001632_rage_fiassetmanager_pushfolder_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001633_rage_fiassetmanager_popfolder(ph: *mut Phantom); }
extern "C" { pub fn f_001635_rage_fiassetmanager_filename_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001640_rage_fiassetmanager_removenamefrompath_char_ptr_int_char_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001649_rage_fgets_char_ptr_int_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_001652_rage_fidevice_makememoryfilename_char_ptr_int_void_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_001653_shutdownwinsock(ph: *mut Phantom); }
extern "C" { pub fn f_001656_rage_fibasetokenizer_init_char_const_ptr_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001657_rage_fibasetokenizer_ignoretoken(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001673_rage_fibasetokenizer_gettoken_char_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001676_rage_fibasetokenizer_matchtoken_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001678_rage_fibasetokenizer_checktoken_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001679_rage_fibasetokenizer_checkitoken_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001683_rage_fibasetokenizer_pop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001709_rage_fiasciitokenizer_put_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001736_rage_fitokenizer_fitokenizer_char_const_ptr_rage_fistream_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_001798_rage_fidevicerelative_getdebugname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001874_void_rage_atplaceptrcreator_void_ptr_rage_fideviceinstaller_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002031_rage_mthrandom_mthrandom_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002080_rage_pgbase_pgbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002081_rage_pgbase_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002084_rage_pgbase_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002091_rage_datresource_containsthisaddress_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002092_rage_datresource_getfixup_void_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002105_rage_pgbase_dtorpgbase_3250(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002111_rage_parmanager_parmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002112_rage_parmanager_initialize_rage_parsettings_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002128_rage_parmanager_uninitialize(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002130_rage_atmap_rage_atnonfinalhashstring_rage_atnonfinalhashstring_r(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002131_rage_atnonfinalhashstring_atnonfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002132_rage_parmanager_dtorparmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002143_rage_parattribute_getstringrepr_char_ptr_int_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_002147_rage_parmanager_loadtree_char_const_ptr_char_const_ptr_rage_pars(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002156_rage_parmanager_initfromstructure_rage_parstructure_ref_rage_par(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_002174_rage_parelement_findattributestringvalue_char_const_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_002175_rage_partree_dtorpartree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002182_rage_parelement_findattribute_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002184_rage_partreenode_findchildwithname_char_const_ptr_rage_partreeno(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002185_rage_partreenode_findchildwithnameignorens_char_const_ptr_rage_p(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002186_rage_parattribute_findint64value_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002187_rage_parattribute_findboolvalue_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002203_rage_parelement_findattributeboolvalue_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002204_rage_parattribute_finddoublevalue_const(ph: *mut Phantom, a0: i64) -> f64; }
extern "C" { pub fn f_002206_rage_parelement_findattributeint64value_char_const_ptr_long_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002296_rage_atmap_unsigned_int_rage_atarray_rage_psorscbuilderinstances(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002311_rage_atarray_rage_psoenumtableentry_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002393_rage_parmemberarray_readcsvdatum_char_ptr_unsigned_int_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002436_rage_parcguregisterstructure_rage_parstructure_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002437_rage_parcgustructure_createstructure_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002438_rage_parcgustructure_setfactories_rage_parstructure_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002439_rage_parcgustructure_setgetstructurecb_rage_parstructure_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002440_rage_parcgustructure_setbaseclass_rage_parstructure_ptr_rage_par(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002444_rage_parcgustructure_buildstructurefromstaticdata_void_ptr_rage(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002582_rage_parelement_findattributeanycase_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002593_rage_parsettings_createstandardsettings(ph: *mut Phantom); }
extern "C" { pub fn f_002644_rage_atmap_rage_conststring_unsigned_int_rage_atmaphashfn_rage_c(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002695_rage_atbinarymap_rage_pardelegateholderbase_ptr_unsigned_int_dat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_002696_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002697_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_002720_rage_conststring_vcopy_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002721_rage_conststring_dtorconststring(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002722_rage_conststring_conststring_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002723_rage_conststring_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002724_rage_prettyprinter_char_ptr_unsigned_long_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002725_rage_atstringbuilder_atstringbuilder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002726_rage_atstringbuilder_init_rage_sysmemallocator_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002727_rage_atstringbuilder_atstringbuilder_char_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002728_rage_atstringbuilder_dtoratstringbuilder(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002730_rage_atstringbuilder_append_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002731_rage_atstringbuilder_append_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002732_rage_atstringbuilder_append_char(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002733_rage_atstringbuilder_appendorfail_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002734_rage_atstringbuilder_appendorfail_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002736_rage_atstringbuilder_tostring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002737_rage_atstringbuilder_endswith_char_const_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002738_rage_atstringbuilder_startswith_char_const_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002743_rage_atdatahash_char_const_ptr_unsigned_long_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002746_rage_stristr_char_ptr_char_const_ptr_char_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002748_rage_stringwildcardcompare_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002749_rage_stringtrim_char_ptr_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_002750_rage_stringrtrim_char_ptr_char_const_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002751_rage_stringstartswithi_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002752_rage_stringendswithi_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002754_rage_stringnullorempty_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002767_deflateinit2(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_002770_deflateend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002773_deflate(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002777_inflateinit2(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002779_inflate(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002781_inflateend(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002808_rage_atdelegator1_void_rage_sysserviceevent_ptr_removedelegate_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002812_rage_sysnamedpipe_sysnamedpipe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002839_rage_sysparam_get_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002841_rage_sysparam_getarray_char_const_ptr_ptr_int_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002852_rage_sysstack_printcapturedstacktrace_unsigned_long_const_ptr_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_002853_rage_sysstack_capturestacktrace_unsigned_long_ptr_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_002857_rage_systimer_haselapsedintervalms_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002886_rage_systhreadpool_thread_dtorthread(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002888_rage_systhreadpool_workitem_workitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002889_rage_systhreadpool_workitem_dtorworkitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002890_rage_systhreadpool_workitem_pending_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002891_rage_systhreadpool_workitem_getid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002892_rage_systhreadpool_workitem_active_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002893_rage_systhreadpool_workitem_queued_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002894_rage_systhreadpool_workitem_finished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002895_rage_systhreadpool_workitem_wascanceled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002898_rage_systhreadpool_dtorsysthreadpool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002899_rage_systhreadpool_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002900_rage_systhreadpool_cancelwork_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002904_rage_systhreadpool_queuework_rage_systhreadpool_workitem_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002905_rage_systhreadpool_numqueuedworkitems(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002906_rage_systhreadpool_numactiveworkitems(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002917_rage_atdelegator1_void_rage_sysserviceevent_ptr_dispatch_rage_sy(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002922_rage_sysuri_sysuri(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002923_rage_sysuri_parse_char_const_ptr_rage_sysuri_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002927_rage_sysmemstarttemp(ph: *mut Phantom); }
extern "C" { pub fn f_002928_rage_sysmemendtemp(ph: *mut Phantom); }
extern "C" { pub fn f_002929_rage_sysmemstartdebug(ph: *mut Phantom); }
extern "C" { pub fn f_002930_rage_sysmemenddebug(ph: *mut Phantom); }
extern "C" { pub fn f_002950_rage_sysmemsimpleallocator_sysmemsimpleallocator_void_ptr_int_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_002952_rage_sysmemsimpleallocator_dtorsysmemsimpleallocator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002954_rage_sysmemsimpleallocator_setquitonfail_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002955_rage_sysmemsimpleallocator_allocate_unsigned_long_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_002959_rage_sysmemsimpleallocator_getlargestavailableblock(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002960_rage_sysmemsimpleallocator_resize_void_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002961_rage_sysmemsimpleallocator_free_void_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002963_rage_sysmemsimpleallocator_getsizewithoverhead_void_const_ptr_co(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002964_rage_sysmemsimpleallocator_isvalidpointer_void_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002966_rage_sysmemsimpleallocator_sanitycheck(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002970_rage_sysmemsimpleallocator_getmemoryused_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002972_rage_sysmemsimpleallocator_beginlayer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002975_rage_sysmemsimpleallocator_endlayer_char_const_ptr_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_003014_rage_sysmemfixedallocator_dumpstats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003065_rage_sysmemcontainer_sysmemcontainer_rage_sysmemcontainerdata_re(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003066_rage_sysmemcontainer_dtorsysmemcontainer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003071_rage_sysmemcontainer_init_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003072_rage_sysmemcontainer_finalize(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003076_rage_systinyheap_allocate_unsigned_long_unsigned_long_unsigned_l(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_003077_rage_systinyheap_getsize_void_const_ptr_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003078_rage_systinyheap_free_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003085_rage_systaskparameters_systaskparameters(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003089_rage_systaskmanager_setup_char_const_ptr_void_ptr_rage_systaskpa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_003090_rage_systaskmanager_create_char_const_ptr_void_ptr_rage_systaskp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_003091_rage_systaskmanager_dispatch_rage_syspreparedtaskhandle_const_re(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003092_rage_systaskmanager_dispatchlocally_rage_syspreparedtaskhandle_c(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003093_rage_systaskmanager_poll_rage_systaskhandle_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_003094_rage_systaskmanager_waitmultiple_int_rage_systaskhandle_ptr_cons(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_003095_rage_systaskmanager_wait_rage_systaskhandle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003097_char_ptr_rage_formatf_char_16ul_char_ref_16ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003098_rage_systasklog_dump(ph: *mut Phantom); }
extern "C" { pub fn f_003111_rage_sysdependencyscheduler_insertinternal_rage_sysdependency_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003134_rage_colorvector3_hsvtorgb(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003138_rage_geompoints_findclosestpointpointboxsurface_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_003139_rage_geomdistances_distancelinetopoint_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> f32; }
extern "C" { pub fn f_003146_segmenttocapsuleintersectionshelper_rage_vec3v_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_003147_rage_geomspheres_testspheretoseg_rage_vec3v_const_ref_rage_scala(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_003148_rage_geomboxes_testcapsuletoalignedboxfp_float_vector_4_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: u128, a3: u128, a4: u128) -> i32; }
extern "C" { pub fn f_003149_rage_geomboxes_testsegmenttocenteredalignedbox_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_003150_rage_geomboxes_testpolygontoorientedboxfp_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_003152_rage_geomboxes_testsegmenttobox_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64) -> i32; }
extern "C" { pub fn f_003153_rage_geomboxes_getfacenormal_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_003154_rage_geomboxes_testspheretobox_rage_vec3v_const_ref_rage_scalarv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_003158_rage_vector3_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003165_rage_matrix34_op_rage_matrix33_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003182_rage_atpoolbase_releasemem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003233_rage_atarray_rage_datref_rage_crblockstream_0_unsigned_short_dec(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003296_rage_atarray_unsigned_short_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003607_rage_mat34v_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004280_rage_fwposematcher_fwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005527_rage_atarray_rage_datowner_rage_rmcdrawable_0_unsigned_short_ata(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_005780_rage_rstcalculatescreenspaceboundisvis_rage_rstsplatmat44v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_007200_char_ptr_rage_formatf_char_255ul_char_ref_255ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007995_rage_matrix34_toeulersxyz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008265_rage_atdelegator2_void_rage_netplayer_ptr_rage_netrequest_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008483_rage_rlv2entitlementinstance_dtorrlv2entitlementinstance(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008513_rage_netobject_hassyncdata_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008606_bool_rage_datbitbuffer_writeint_signed_char_signed_char_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008746_unsigned_int_rage_netmessage_readheader_rage_datbitbuffer_unsign(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008764_bool_rage_datbitbuffer_readint_int_int_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_008765_bool_rage_netmessage_writeheader_rage_datexportbuffer_unsigned_i(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_009279_bool_rage_serializefromfile_rage_mshmesh_char_const_ptr_rage_msh(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009280_rage_mshmaterial_dtormshmaterial(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009283_rage_grmmodel_getgeometry_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_009286_rage_grmmodel_setforceshader_rage_grmshader_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009291_rage_grmshader_begindraw_rage_grmshader_edrawtype_bool_rage_grce(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_009292_pixbeginn_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_009294_rage_grmshader_unbind_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009295_rage_grmshader_enddraw_const(ph: *mut Phantom); }
extern "C" { pub fn f_009313_rage_grmshader_gettechniquegroupid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_009314_pixend(ph: *mut Phantom); }
extern "C" { pub fn f_009318_stuffelementarraydynamic_rage_atrangearray_rage_grcvertexelement(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64) -> i32; }
extern "C" { pub fn f_009319_stuffelementarray_rage_atrangearray_rage_grcvertexelement_64_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64) -> i32; }
extern "C" { pub fn f_009320_rage_grmmodelfactory_buildcacheddeclarator_rage_grcfvf_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_009321_rage_grmmodelfactory_builddeclarator_rage_grcfvf_const_ptr_rage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009340_void_rage_datresource_place_rage_pgdictionary_rage_grctexture_pt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009417_pixbegin_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_009445_rage_pfdrawarrow_rage_vector3_const_ref_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009447_rage_pfdrawitembase_beginmanager(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009448_rage_pfdrawitembase_end(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009451_rage_pfdrawitem_drawtick_rage_vector3_const_ref_float_rage_color(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_009452_rage_pfdrawitem_drawbox_rage_matrix34_const_ref_rage_vector3_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_009453_rage_pfdrawitem_drawcapsule_rage_vector3_const_ref_rage_vector3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i32); }
extern "C" { pub fn f_009473_rage_pfdrawgroup_enablechildren(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009474_rage_pfdrawgroup_disablechildren(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009499_rage_grcfvf_setposchannel_bool_rage_grcfvf_grcdatasize_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009500_rage_grcfvf_setnormalchannel_bool_rage_grcfvf_grcdatasize(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_009506_rage_grcfvf_settexturechannel_int_bool_rage_grcfvf_grcdatasize(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_009514_rage_grctexture_setcustomloadname_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009518_rage_grcimage_create_unsigned_int_unsigned_int_unsigned_int_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_009523_rage_grctexturefactory_createtexture_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009539_rage_grceffect_lookuptechnique_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009540_rage_grceffect_lookupvar_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009543_rage_grceffect_beginpass_int_rage_grcinstancedata_const_ref_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009545_rage_grcstateblock_setblendstate_rage_grcstateblock_grcblendstat(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009547_rage_grcdevice_beginindexedvertices_rage_grcdrawmode_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i32) -> i64; }
extern "C" { pub fn f_009548_rage_grcdevice_drawvertices_void_const_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_009549_rage_grceffect_endpass_const(ph: *mut Phantom); }
extern "C" { pub fn f_009555_rage_grcstateblock_setrasterizerstate_rage_grcstateblock_grcrast(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009560_rage_grcstateblock_setdepthstencilstate_rage_grcstateblock_grcde(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_009561_rage_grcbegin_rage_grcdrawmode_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_009562_rage_grcvertex_float_float_float_float_float_float_rage_color32(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: i32, a7: f32, a8: f32); }
extern "C" { pub fn f_009563_rage_grcend(ph: *mut Phantom); }
extern "C" { pub fn f_009588_rage_grcimage_dtorgrcimage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009612_rage_grcviewport_setwindow_int_int_int_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: f32, a6: f32); }
extern "C" { pub fn f_009613_rage_grcviewport_dtorgrcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009615_rage_grceffect_lookupglobalvar_char_const_ptr_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_009617_rage_grcviewport_regenerateperspective(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009630_rage_grcviewport_regenerateinstvpmatrices_rage_mat44v_const_ref(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_009634_rage_grcviewport_setcurrentworldidentity(ph: *mut Phantom); }
extern "C" { pub fn f_009638_rage_grcviewport_isspherevisible_rage_vec4v_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009640_rage_grcviewport_isspherevisible_float_float_float_float_float_p(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_009643_rage_grcviewport_ispointsetvisible_rage_vec3v_const_ptr_int_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_009649_rage_grcworldidentity(ph: *mut Phantom); }
extern "C" { pub fn f_009650_rage_grcbindtexture_rage_grctexture_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009723_rage_grcdevice_setvertexdeclaration_rage_grcvertexdeclaration_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009727_rage_grcdevice_beginvertices_rage_grcdrawmode_unsigned_int_unsig(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_009733_rage_grcdrawline_rage_vector3_const_ref_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009734_rage_grcdrawline_rage_vector3_const_ref_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_009736_rage_grcdrawbox_rage_vector3_const_ref_rage_matrix34_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009737_rage_grcdrawsolidbox_rage_vector3_const_ref_rage_matrix34_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_009741_rage_grcdrawspiral_rage_vector3_const_ref_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_009742_rage_grcdrawcircle_float_rage_vector3_const_ref_rage_vector3_con(ph: *mut Phantom, a0: f32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_009743_rage_grcdrawarc_float_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: f32, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: i32, a7: i32); }
extern "C" { pub fn f_009744_rage_grcdrawcapsule_float_float_rage_matrix34_const_ref_int_bool(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i32, a4: i32, a5: f32); }
extern "C" { pub fn f_009745_rage_grcdrawcylinder_float_float_rage_matrix34_const_ref_int_boo(ph: *mut Phantom, a0: f32, a1: f32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_009751_rage_grcdrawlabel_rage_vector3_const_ref_int_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: f32, a6: f32); }
extern "C" { pub fn f_009752_rage_grcworldmtx_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009753_rage_grcworldmtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009754_rage_grclighting_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_009755_rage_grcdrawarrowortho_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_009758_rage_grcdrawsphere_float_rage_vector3_const_ref_int_bool_bool(ph: *mut Phantom, a0: f32, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_009760_rage_grcdrawaxis_float_rage_mat34v_const_ref_bool(ph: *mut Phantom, a0: f32, a1: i64); }
extern "C" { pub fn f_009762_rage_grcdrawlabelf_rage_vector3_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_009765_rage_grcvertexbuffer_create_int_rage_grcfvf_const_ref_bool_bool(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_009768_rage_grcvertexbuffer_makereadonly(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009769_rage_grcvertexbuffereditor_grcvertexbuffereditor_rage_grcvertexb(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_009771_rage_grcvertexbuffereditor_dtorgrcvertexbuffereditor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009774_rage_grcbuffereditor_setposition_int_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009785_rage_grcindexbuffer_create_int_bool_void_ptr(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_009789_rage_grcindexbuffer_unlockwo_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009814_rage_grceffect_setvar_rage_grcinstancedata_ref_rage_grceffectvar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009849_rage_grcinstancedata_clone_rage_grcinstancedata_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009858_rage_grcstateblock_createrasterizerstate_rage_grcrasterizerstate(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_009859_rage_grceffect_setglobalfloatcommon_int_float_ptr_int_rage_grcef(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_009867_rage_grcinstancedata_dtorgrcinstancedata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009868_rage_grceffect_lookupvarbyhash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_009869_rage_grceffect_getlocalcommon_rage_grcinstancedata_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_009870_rage_grceffect_getvardesc_rage_grceffectvar_unsigned_int_ref_rag(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_009875_rage_grceffect_setlocalcommon_rage_grcinstancedata_ref_rage_grce(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_009880_rage_grceffect_getvar_rage_grcinstancedata_const_ref_rage_grceff(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_009952_rage_grcvertexbufferd3d11_unlockinternal_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_009959_rage_grcindexbuffer_lockrw_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009965_rage_grccbuffer_getdataptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009966_rage_grccbuffer_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010011_rage_grcdevice_setvertexshaderconstantf_int_float_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_010012_rage_grcdevice_endcreatevertices_void_const_ptr(ph: *mut Phantom); }
extern "C" { pub fn f_010013_rage_grcdevice_drawvertices_void_const_ptr_unsigned_int_rage_grc(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_010015_rage_grcdevice_drawindexedinstancedvertices_void_const_ptr_unsig(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_010029_rage_grcdevice_drawindexedprimitive_rage_grcdrawmode_int_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_010030_rage_grcdevice_drawindexedprimitive_rage_grcdrawmode_rage_grcver(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_010033_rage_grcdevice_drawinstancedindexedprimitive_rage_grcdrawmode_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010183_rage_grcdebugdraw_cross_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010188_rage_grcdebugdraw_boxaxisaligned_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010194_rage_grcdebugdraw_axis_rage_mat34v_const_ref_float_bool_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_010225_rage_grcdebugdraw_text_rage_vec3v_const_ref_rage_color32_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i32); }
extern "C" { pub fn f_010525_rage_iovirtualkeyboard_getresult(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_010640_rage_mshmaterial_gettrianglecount_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_010646_rage_mshmaterial_triangleiterator_getvertindices_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_010647_rage_mshmaterial_begintriangles(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010648_rage_mshmaterial_endtriangles(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010649_rage_mshmaterial_triangleiterator_op(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_010653_rage_mshmaterial_triangleiterator_op_rage_mshmaterial_triangleit(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_010668_rage_rmcdrawable_rmcdrawable(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010671_rage_rmcdrawable_dtorrmcdrawable(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_010706_rage_rmcdrawable_place_rage_rmcdrawable_ptr_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010711_rage_rmclodgroup_getcurrentlod_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_010841_rage_mshmesh_mshmesh_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_011120_void_rage_atplaceptrcreator_void_ptr_rage_spropdetails_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011791_rage_phcontactconstiterator_op(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011827_rage_geomvectors_computeleanandtwistangles_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_011885_rage_phiterator_dtorphiterator(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012181_rage_ptxeffectinst_dtorptxeffectinst(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012187_rage_ptxevolutionlist_getevolvedkeyframepropbyid_unsigned_int_co(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012188_rage_ptxevolutionlist_query_rage_ptxkeyframeprop_ptr_rage_scalar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_012191_rage_ptxeffectinst_finish_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012194_rage_ptxeffectinst_draw_rage_ptxdrawinterface_ptxmultipledrawint(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012210_rage_ptxevolutionlist_ptxevolutionlist_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012211_rage_ptxkeyframe_addentry_rage_scalarv_const_ref_rage_vec4v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012221_rage_ptxemitterinst_stop_rage_ptxeffectinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012222_rage_ptxparticlerule_returnpoint_rage_ptxlist_ptr_rage_ptxpool_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012223_rage_ptxemitterinst_getattractordomainlcltowldmtx_rage_mat34v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012224_rage_ptxemitterinst_gettargetdomainmtxwld_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012225_rage_ptxemitterinst_updatedomains_rage_ptxeffectinst_ptr_rage_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_012227_rage_ptxemitterinst_updateparallel_rage_ptxeffectinst_ptr_rage_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32); }
extern "C" { pub fn f_012228_rage_ptxemitterinst_updatefinalize_rage_ptxeffectinst_ptr_rage_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_012229_rage_ptxparticlerule_setparams_rage_ptxbehaviourparams_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i32); }
extern "C" { pub fn f_012231_rage_ptxemitterinst_sortpoints_rage_ptxpool_rage_ptxpointsb_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012235_rage_ptxdomain_place_rage_ptxdomain_ptr_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012236_rage_ptxemitterrule_prepareevodata_rage_ptxevolutionlist_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012243_rage_ptxshadertemplatelist_loadshader_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012244_rage_ptxmsg_assetwarningf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012246_rage_ptxparticlerule_prepareevodata_rage_ptxevolutionlist_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012268_rage_ptxkeyframe_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012273_rage_ptxkeyframe_copyentriesfrom_rage_ptxkeyframe_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012286_rage_ptxshaderinst_begin(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012287_rage_drawtrailbuildcontrolpoints_rage_ptxtrailcontrolpoint_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i32); }
extern "C" { pub fn f_012292_rage_ptxshadertemplatelist_setglobalvars(ph: *mut Phantom); }
extern "C" { pub fn f_012305_rage_ptxshadertemplate_setshadervars_int_rage_vec4v_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_012310_rage_ptxshadervar_place_rage_ptxshadervar_ptr_rage_datresource_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012373_rage_ptxmanager_geteffectrule_rage_atnonfinalhashstring_rage_ptx(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_012381_rage_ptxdrawableentry_ptxdrawableentry_rage_datresource_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012383_rage_ptxevolutionlist_dtorptxevolutionlist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012385_rage_ptxevent_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012398_rage_ptxevolvedkeyframeprop_dtorptxevolvedkeyframeprop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012399_void_rage_datresource_place_rage_pgdictionary_rage_rmcdrawable_p(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012400_rage_ptxfxlist_dtorptxfxlist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012405_rage_ptxmanager_dtorptxmanager_20062(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012426_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_012428_void_std_2_introsort_std_2_classicalgpolicy_std_2_greater_rage_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_012434_rage_atmapentry_char_const_ptr_rage_ptxbehaviour_ptr_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012435_rage_atmapentry_unsigned_int_void_ptr_rage_ptxbehaviour_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012436_rage_ptxbehaviour_ptxbehaviour_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012440_rage_ptxbehaviour_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012444_rage_ptxrenderstate_setrenderstate_rage_ptxprojectionmode(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012470_rage_ptxu_animatetexture_updatetextures_rage_ptxbehaviourparams(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012531_rage_ptxdomain_dtorptxdomain_20231(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012573_rage_ptxu_colour_dtorptxu_colour(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012585_rage_ptxu_noise_dtorptxu_noise(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012591_rage_ptxu_rotation_dtorptxu_rotation(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012595_rage_ptxu_size_dtorptxu_size(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012610_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_012612_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_012613_rage_pgdictionary_rage_ptxparticlerule_dtorpgdictionary(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012615_rage_pgdictionary_rage_ptxeffectrule_dtorpgdictionary(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012617_rage_pgdictionary_rage_ptxemitterrule_dtorpgdictionary(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012619_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_std_2_grea(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_012620_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012650_rage_snsession_leave_int_rage_netstatus_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_012651_rage_snsession_destroy_rage_netstatus_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012652_rage_snsession_dropgamers_rage_snpeer_const_ptr_rage_rlgamerhand(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_012653_bool_rage_rltaskbase_configure_rage_sndestroytask_rage_snsession(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_012654_rage_snsession_addtask_rage_sntask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012658_rage_snpeer_getgamers_rage_rlgamerinfo_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_012659_rage_snsession_shutdown_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_012663_rage_snmsgremovegamersfromsessioncmd_snmsgremovegamersfromsessio(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012664_rage_snsession_getgamers_rage_rlgamerinfo_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_012665_rage_snsession_removetask_rage_sntask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012666_rage_snsession_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012667_rage_snparty_exists_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012671_rage_snsession_getsessionid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012672_rage_snsession_isinvitable_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012673_rage_snsession_isacceptingjoinrequests_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012674_rage_snsession_getemptyslots_rage_rlslottype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_012675_rage_snsession_isjoinableviapresence_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012677_rage_snsession_getsessioninfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012678_rage_netstatus_setpending(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012680_rage_snsession_ishost_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012681_rage_snparty_ismember_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012682_rage_snsession_ismember_rage_rlgamerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012683_rage_snsession_getgamers_rage_rlgamerid_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012684_rage_snsession_getgamers_rage_rlgamerhandle_ptr_unsigned_int_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012689_bool_rage_datimportbuffer_seruser_rage_rlsessioninfo_rage_rlsess(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012714_rage_snpeer_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012715_rage_snpeer_initlocal_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012716_rage_snpeer_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012717_rage_snpeer_getgamer_rage_rlgamerid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012718_rage_snpeer_getgamercount_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012719_rage_snpeer_getgamers_rage_rlgamerid_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_012720_rage_snpeer_getgamers_rage_rlgamerhandle_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_012721_rage_snpeer_getpeerinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012722_rage_snpeer_getpeerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012723_rage_snpeer_allowmigration_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012727_rage_snmsgaddgamertosessioncmd_snmsgaddgamertosessioncmd(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012728_rage_snsession_getsessiontoken_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012729_rage_snsession_reserveslots_rage_rlgamerhandle_const_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_012730_rage_snsession_getjoingamerstorlinetask_bool_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012731_rage_snsession_findpeerbycxn_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012732_rage_snsession_markasnotmigrating_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012733_rage_snsession_handleremovegamercmd_rage_snpeer_const_ptr_rage_s(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012734_bool_rage_rltaskbase_configure_rage_rlfireandforgettask_rage_snc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_012735_bool_rage_rltaskbase_configure_rage_rlfireandforgettask_rage_snm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_012736_rage_netmessage_getname_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012737_rage_snmsgjoinrequest_snmsgjoinrequest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012738_rage_snmsgjoinrequest_import_void_const_ptr_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_012739_rage_snsession_handlejoinrequest_rage_snmsgjoinrequest_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_012740_bool_rage_snmsgremovegamersfromsessioncmd_sermsgpayload_rage_dat(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012741_rage_snsession_getgamerinfo_rage_rlgamerid_const_ref_rage_rlgame(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_012742_rage_snsession_findpeerbyid_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012743_rage_snmsgsessionmemberids_snmsgsessionmemberids(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012744_rage_snsession_droppeer_unsigned_long_rage_snleavegamersfromrlin(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012745_rage_snsession_disconnectpeer_rage_snpeer_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012747_rage_snsession_disablenetwork(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012748_rage_snsession_removegamer_rage_rlgamerhandle_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_012749_rage_snsession_removepeer_rage_snpeer_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012753_rage_snsession_enablenetwork(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012754_rage_snsession_setacceptingjoinrequests_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012757_rage_snsession_getlocalpeerinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012760_rage_snsession_updatepresenceattributes_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_012761_rage_snsession_isjoining(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012762_bool_rage_nettransactor_sendrequest_rage_snmsgremovegamersfromse(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_012763_rage_snsession_printmembers_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012765_rage_snsession_getconfig_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012766_rage_snsession_migrate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012767_rage_snsession_dispatchevent_rage_snevent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012774_rage_snsession_getavailableslots_rage_rlslottype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_012784_void_std_2_introsort_std_2_classicalgpolicy_rage_peerpred_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_012785_rage_snsession_addpeer_int_unsigned_int_rage_rlpeerinfo_const_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_012786_rage_snsession_haspeer_unsigned_long_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012787_rage_atdelegate0_bool_op_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012788_rage_snsession_removepeer_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012789_rage_snsession_isconnectedtopeer_unsigned_long_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012790_rage_snsession_denyjoinrequest_rage_netaddress_const_ref_rage_sn(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32); }
extern "C" { pub fn f_012791_bool_rage_snmsgjoinresponse_sermsgpayload_rage_datexportbuffer_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012792_bool_rage_netconnectionmanager_send_rage_snmsgjoinresponse_int_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_012793_rage_snsession_fillslotreservation_rage_rlgamerhandle(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012794_rage_snsession_addgamer_rage_rlgamerinfo_const_ref_rage_rlslotty(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_012795_rage_atdelegate3_void_rage_snsession_ptr_rage_rlgamerinfo_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_012796_rage_rlfireandforgettask_rage_sndropgamerstask_ptr_rage_rltaskma(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012797_bool_rage_rltaskbase_configure_rage_rlfireandforgettask_rage_snd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_012798_rage_snsession_findgamerbyid_rage_rlgamerid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012799_rage_snsession_sendaddgamercommandtopeer_rage_rlgamerhandle_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_012800_rage_snmsgaddgamertosessionbase_reset_unsigned_long_rage_rlgamer(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32); }
extern "C" { pub fn f_012801_bool_rage_nettransactor_sendrequest_rage_snmsgaddgamertosessionc(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_012804_bool_rage_datimportbuffer_seruser_rage_rlgamerid_rage_rlgamerid(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012805_rage_snsession_connecttopeer_rage_rlpeerinfo_const_ref_int_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_012811_rage_textchat_removetyper_rage_rlgamerid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012818_rage_atdelegator4_void_rage_textchat_ptr_rage_rlgamerhandle_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_012819_rage_clothcontroller_clothcontroller(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012821_rage_clothcontroller_load(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012822_rage_clothcontroller_save(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012824_rage_clothcontroller_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012825_rage_clothcontroller_dtorclothcontroller(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012827_rage_clothcontroller_clothcontroller_rage_datresource_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012828_rage_phmorphcontroller_morphlevelone_int_int_rage_vec3v_ptr_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_012830_rage_phmorphcontroller_createindexmap_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_012858_rage_environmentcloth_switchtolod_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012862_rage_environmentcloth_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012863_rage_environmentcloth_dtorenvironmentcloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012889_rage_characterclothcontroller_dtorcharacterclothcontroller(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012891_rage_characterclothcontroller_characterclothcontroller_rage_datr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012900_rage_charactercloth_charactercloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012908_rage_charactercloth_dtorcharactercloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012912_rage_pack_core32_float_vector_4_ref_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128) -> u128; }
extern "C" { pub fn f_012917_rage_atrangearray_rage_phmorphdata_4_atrangearray_rage_datresour(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012918_rage_atrangearray_rage_atarraywrap_unsigned_short_4_atrangearray(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012919_rage_phmorphdata_phmorphdata_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012921_rage_rmcropedrawable_draw_rage_matrix34_const_ref_unsigned_int_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_012938_rage_atrangearray_rage_atarraywrap_unsigned_short_4_dtoratrangea(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012965_bool_rage_datimportbuffer_seruser_rage_rlgamerhandle_rage_rlgame(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012966_bool_rage_datimportbuffer_seruser_rage_rlgamerinfo_rage_rlgameri(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012967_bool_rage_datimportbuffer_seruser_rage_netaddress_rage_netaddres(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012970_rage_phmorphcontroller_dtorphmorphcontroller(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012971_rage_phmorphdata_dtorphmorphdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012976_bool_rage_snmsgremovegamersfromsessioncmd_sermsgpayload_rage_dat(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012978_rage_snmigratesessiontask_dtorsnmigratesessiontask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012985_std_2_enable_if_is_move_constructible_rage_rlpeerinfo_value_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012986_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_rage_peerp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012987_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_rage_peerp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_012988_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_peerp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_012989_void_std_2_insertion_sort_abi_nqn220108_std_2_classicalgpolicy_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012990_void_std_2_insertion_sort_unguarded_abi_nqn220108_std_2_classica(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_012991_void_std_2_sift_down_abi_nqn220108_std_2_classicalgpolicy_false(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012992_void_std_2_pop_heap_abi_nqn220108_std_2_classicalgpolicy_rage_pe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012993_rage_rlpeerinfo_ptr_std_2_partition_with_equals_on_left_abi_nqn2(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012994_std_2_pair_rage_rlpeerinfo_ptr_bool_std_2_partition_with_equals(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_012995_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013008_rage_rlfireandforgettask_rage_snchangeattributestask_dtorrlfirea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013012_rage_rlfireandforgettask_rage_snmodifypresenceflagstask_dtorrlfi(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013020_rage_bgglassmanager_hitglass_short_int_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_013023_rage_bgdrawable_dtorbgdrawable(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013025_rage_bgglassmanager_recycleglass_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013026_rage_bgglassmanager_freeglass_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013027_rage_bgbreakable_breakabledata_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013036_rage_bgbreakable_adjustlod(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013037_rage_bgbreakable_convertcrackmesh_rage_bgmesh_ref_rage_bghitinfo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_013038_rage_bgclipmesh_rage_bgmesh_ref_rage_vector2_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013039_rage_atarray_rage_bgpanepiece_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013040_rage_atarray_rage_bgpanepiece_0_unsigned_short_destruct_rage_bgp(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013041_rage_bgbreakable_constructpanepiece_rage_bgedgeadder_rage_bgbrea(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64, a7: i32); }
extern "C" { pub fn f_013042_rage_bgbreakable_killpanepiece_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013043_rage_bgedgeref_ref_rage_atmap_rage_bgedgekey_rage_bgedgeref_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013044_rage_bgbreakable_pieceflyoutonimpact_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013045_rage_bgbreakable_isfloating_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013046_rage_bgbreakable_setfloatstate_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013047_rage_bgbreakable_applyimpulse_rage_bgfallingphysics_ref_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: i64, a6: i64, a7: f32); }
extern "C" { pub fn f_013048_rage_bgbreakable_createonefallingpiece_rage_bgpanepiece_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_013049_rage_bgbreakable_transform_rage_vector2_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013050_rage_matrix34_transform3x3_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013051_rage_quaternion_transform_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013052_rage_bgswapvector_rage_bgmeshpolygon_rage_bgswapclearer_rage_bgm(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013053_void_std_2_vector_rage_bgedgeref_rage_stlallocator_rage_bgedgere(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013054_std_2_vector_rage_bgedgeref_rage_stlallocator_rage_bgedgeref_thr(ph: *mut Phantom); }
extern "C" { pub fn f_013055_void_std_2_vector_rage_bgedge_rage_stlallocator_rage_bgedge_empl(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013056_rage_atarray_rage_bgarray_unsigned_short_8_unsigned_short_op_rag(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013059_rage_atarray_rage_bghitinfo_0_unsigned_short_op_rage_atarray_rag(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013060_rage_atarray_rage_bgpanepiece_0_unsigned_short_op_rage_atarray_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013063_rage_bgcrackstarmap_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_013078_rage_bgshadervars_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_013084_rage_bgglasstypeconfig_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_013093_rage_bgcracktype_dtorbgcracktype(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013095_rage_bgconfig_dtorbgconfig(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013096_rage_bgshadervars_dtorbgshadervars(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013099_rage_overridetexturevars_rage_atarray_rage_conststring_0_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013101_rage_atbintreebase_unsigned_int_rage_grctexture_ptr_removenode_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013107_rage_atarray_rage_bgglassmanager_glassinfo_0_unsigned_short_dest(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013108_rage_bgglassmanager_glassinfo_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013111_rage_bgglassmanager_destroybreakableglass_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_013115_rage_bgglassmanager_glassinfo_sync(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013120_std_2_vector_unsigned_short_rage_stlallocator_unsigned_short_res(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013153_rage_rsttransformtoscreenspace_rsttransformtoscreenspace_rage_gr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_013154_rage_rsttestedgelistcoverage_rage_mat44v_const_ref_rage_scalarv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64, a8: i32, a9: i64, a10: i64, a11: i64, a12: i32, a13: i32, a14: i32); }
extern "C" { pub fn f_013157_rage_softrasterizer_dtorsoftrasterizer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013161_rage_dependanttasklist_add_char_const_ptr_void_ptr_rage_systaskp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_013166_rage_dependanttasklist_taskdumpdependancygraphrecurse_rage_fistr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013167_rage_rstfasttiler_addindexedtrilist_rage_vec3v_const_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_013168_rage_rstfasttiler_addpttri_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_013169_rage_rstfasttiler_addindexedhtrilistwithclipping_rage_vec4v_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_013174_rage_rstselectmodeloccluders_rage_rsttransformtoscreenspace_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_013175_void_std_2_introsort_std_2_classicalgpolicy_std_2_greater_int_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_013178_rage_occludemodel_unpackvertices_rage_vec3v_ptr_int_rage_scalarv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013182_testptinpolyxy_rage_scalarv_const_ref_rage_scalarv_const_ref_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_013183_rage_rststencilhiz_rage_vec4v_ptr_rage_pretri2_const_ptr_int_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_013187_rage_rstfasttiler_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013190_rage_rstgeneratetris_rage_mat44v_const_ref_rage_scalarv_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i32, a8: i32, a9: i64) -> i32; }
extern "C" { pub fn f_013192_rage_rsttransformhullandgetminz_rage_mat44v_const_ref_rage_vec2v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64) -> i32; }
extern "C" { pub fn f_013195_rage_rstscantile_rage_scantileinfo_const_ptr_rage_scanuserdata_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_013197_rage_rsttesthiz_rage_vec4v_const_ptr_rage_pretri2_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_013212_void_std_2_vector_rage_bgedgeref_rage_stlallocator_rage_bgedgere(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013225_rage_bgcrackstarmap_dtorbgcrackstarmap(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013246_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013247_rage_sntask_configurecontext_rage_rltask_rage_snsession_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013248_rage_rlsessioninfo_op_rage_rlsessioninfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013255_rage_snconnecttopeertask_dtorsnconnecttopeertask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013257_rage_snconnecttopeertask_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013258_rage_snconnecttopeertask_finish_rage_rltaskbase_finishtype_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013260_rage_snaddremotegamertask_dtorsnaddremotegamertask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013266_rage_snjoingamerstorlinetask_dtorsnjoingamerstorlinetask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013268_rage_snjoingamerstorlinetask_addgamer_rage_rlgamerhandle_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_013269_rage_snjoingamerstorlinetask_iswaitingforaddgamers(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013273_rage_snjoingamerstorlinetask_dropgamers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013289_rage_snmodifypresenceflagstask_snmodifypresenceflagstask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013293_rage_snestablishsessiontask_dtorsnestablishsessiontask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013296_rage_snestablishsessiontask_finish_rage_rltaskbase_finishtype_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013297_rage_snestablishsessiontask_update_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013304_bool_rage_snmsgjoinresponse_sermsgpayload_rage_datimportbuffer_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013305_rage_snjoinsessiontask_dtorsnjoinsessiontask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013313_bool_rage_snmsgmigratehostrequest_export_rage_datexportbuffer_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013318_rage_snmigratesessiontask_migrate_rage_rlpeerinfo_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013322_rage_snchangeattributestask_snchangeattributestask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013326_rage_snsendinvitestask_dtorsnsendinvitestask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013357_bool_rage_datimportbuffer_seruser_rage_rlsessionconfig_rage_rlse(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013368_rage_neteventconsumer_rage_netevent_dtorneteventconsumer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013369_rage_neteventqueue_rage_netevent_unsubscribeinternal_rage_neteve(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013371_rage_voicechat_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013372_rage_voicechat_removetalker_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_013376_rage_voicechat_addtalker_rage_rlgamerinfo_const_ref_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013377_rage_voicechat_findtalkerbygamerid_rage_rlgamerid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013388_rage_neteventqueue_rage_netevent_nexteventinternal_rage_netevent(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013389_rage_neteventqueue_rage_netevent_remove_rage_neteventbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013398_rage_netframe_getpayload_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013400_rage_netpacketstreamrecorder_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013404_rage_netpacketstreamrecorder_recordoutboundpacket_rage_netsocket(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_013405_rage_netpacketstreamrecorder_recordinboundpacket_rage_netsocketa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_013406_rage_netcompressiontrainer_dtornetcompressiontrainer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013407_rage_netconnectionmanager_unregisterchannelpacketrecorder_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_013408_rage_nethardware_getlocalipaddress_rage_netipaddress_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013410_rage_netconnectionmanager_netcriticalsectiontoken_lock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013411_rage_netconnectionmanager_netcriticalsectiontoken_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013412_rage_netconnection_dtornetconnection(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013413_rage_netconnection_close(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013415_rage_netaddress_formatattemptipv4_char_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013416_rage_netconnection_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013417_rage_netaddress_tostring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013418_rage_inmap_base_unsigned_short_rage_netoutframe_refrage_netoutfr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013419_rage_inmap_base_unsigned_short_rage_netconnection_inframe_refrag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013420_rage_netconnection_synchronize_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013421_rage_netconnection_queueframe_void_const_ptr_unsigned_int_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_013422_rage_netframe_getpayload(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013423_rage_netrandom_getranged_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_013424_rage_netrandom_getint(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013425_rage_netconnection_setpolicies_rage_netconnectionpolicies_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013426_rage_netconnection_alloccritical_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_013427_rage_netendpoint_getid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013428_rage_netconnectionmanager_getendpointbyid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_013429_char_ptr_rage_safecatf_char_char_ptr_unsigned_long_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_013430_rage_netconnectionmanager_sendreservedchannelpacket_rage_netaddr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_013431_rage_netframe_validate_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013432_rage_netconnection_queueevent_rage_netevent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013433_rage_netconnection_allocinframe_rage_netframe_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_013434_rage_neteventqueue_rage_netevent_queueevent_rage_netevent_ptr_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_013435_rage_netconnection_dropoutboundunreliableframes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013436_rage_netconnection_reclaimmem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013437_rage_netconnectionmanager_reclaimmem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013438_rage_netaddress_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013439_rage_netaddress_op_rage_netaddress_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013440_rage_netaddress_sanitycheck_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013441_rage_netaddress_netaddress_rage_netaddress_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013444_rage_cxn_init_rage_netconnectionmanager_ptr_rage_neteventqueue_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_013445_rage_cxn_close(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013446_rage_netendpoint_netendpoint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013447_rage_netendpoint_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013449_rage_neteventqueue_rage_netevent_removeevents_rage_atfunctor1_bo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013450_rage_neteventbase_dtorneteventbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013451_rage_netstatus_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013452_rage_netpeeraddress_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013454_rage_netconnectionmanager_send(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013455_rage_netconnectionmanager_channel_update_rage_nettimestep_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013456_rage_nettimestep_settime_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013457_rage_netconnectionmanager_sendpacket_rage_netaddress_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_013458_rage_netconnectionmanager_getactiveendpointbyaddr_rage_netaddres(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013459_rage_netconnectionmanager_closeconnection_rage_cxn_ptr_rage_netc(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_013460_rage_netconnectionmanager_queueoutofmemory_rage_cxn_ptr_bool_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_013461_rage_netconnectionmanager_delegate_delegate(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013462_rage_netconnectionmanager_delegate_dtordelegate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013464_rage_netconnectionmanager_removechanneldelegate_rage_netconnecti(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013466_rage_netconnectionmanager_channel_recordoutbound_rage_netaddress(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_013468_rage_netaddress_gettargetaddress_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013469_rage_netconnectionmanager_channel_dispatchevent_rage_netconnecti(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013475_rage_netconnectionmanager_handlereceivedpacket_rage_netaddress_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_013476_rage_netconnectionmanager_sendrelayaddresschangedmsg_rage_netpee(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_013477_rage_netpeeraddress_tostring_char_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_013478_rage_netconnectionmanager_dtornetconnectionmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013479_rage_netconnectionmanager_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013480_rage_nettunneler_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013481_rage_serverinfo_clearaddr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013482_rage_netconnectionrouter_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013483_rage_netconnectionmanager_destroyendpoint_rage_netendpoint_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013484_rage_netconnectionmanager_freeendpoint_rage_netendpoint_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013487_rage_netdynarray_rage_netendpoint_4u_65535u_resize_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013488_rage_netdynarray_rage_cxn_4u_65535u_resize_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013489_rage_netcrypto_generaterandombytes_unsigned_char_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013493_rage_netconnectionmanager_isinitialized_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013495_rage_netpeerid_getlocalpeerid_rage_netpeerid_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013496_rage_netdynarray_rage_netconnectionrouter_routepair_data_25u_409(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013497_rage_inmap_base_rage_netpeerid_rage_netconnectionrouter_node_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013498_rage_netconnectionrouter_node_clearprecursorlist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013499_rage_inmap_base_rage_netconnectionrouter_routepair_key_rage_netc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013500_rage_nettunneler_closetunnel_rage_netaddress_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013501_rage_netaddress_op_rage_netaddress_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013502_rage_inmap_base_rage_netaddress_rage_netendpoint_refrage_netendp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013503_rage_netconnectionmanager_getactiveendpointbypeerid_rage_netpeer(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013504_rage_inmap_base_rage_netpeerid_rage_netendpoint_refrage_netendpo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013505_rage_inmap_base_unsigned_int_rage_netendpoint_refrage_netendpoin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013506_rage_netpeeraddress_tostring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013507_rage_nettunneler_cancelrequest_rage_nettunnelrequest_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013508_rage_netconnectionmanager_startlistening_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013509_rage_netconnectionmanager_opentunnel_rage_netpeeraddress_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_013510_rage_netconnectionmanager_allocendpoint(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013511_rage_netaddress_init_rage_netsocketaddress_const_ref_rage_netrel(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_013512_rage_netconnectionmanager_hasactiveendpoint_rage_netaddress_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013513_rage_nettunneler_overriderequest_rage_nettunnelrequest_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013514_rage_netpeeraddress_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013515_rage_netpeeraddress_getrelayaddress_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013516_rage_nettunnelrequest_pending_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013517_rage_netaddress_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013518_rage_neticetunneler_cancelrequest_rage_netstatus_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013519_rage_nettunnelrequest_getrelayaddress_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013520_rage_netkeyexchange_updatep2pkey_rage_netpeerid_const_ref_rage_n(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013521_rage_nettunnelrequest_setstate_rage_nettunnelrequest_state_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013522_rage_netconnectionmanager_canceltunnelrequest_rage_nettunnelrequ(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013523_rage_netconnectionmanager_removecachedrelayaddress_rage_netaddre(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013524_rage_nettunnelrelaycache_removeentry_rage_netaddress_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013525_rage_netconnectionmanager_resolvenewendpointrequest_rage_netpeer(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_013526_rage_netconnectionmanager_createendpoint_rage_netpeeraddress_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_013527_rage_netconnectionmanager_createconnection_rage_netendpoint_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_013528_rage_netconnectionmanager_striprelaytoken_rage_netaddress_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013529_rage_netconnectionmanager_reassignendpoint_rage_netendpoint_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_013530_rage_netconnectionmanager_getactiveendpointbyrelayaddr_rage_neta(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013531_rage_inmap_base_rage_netpeerid_rage_netendpoint_refrage_netendpo(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013532_rage_inmap_base_rage_netaddress_rage_netendpoint_refrage_netendp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_013533_rage_netaddress_isrelayserveraddr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013534_rage_netconnectionmanager_p2pencrypt_rage_netaddress_const_ref_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_013535_rage_netconnectionrouter_preparetoforward_rage_netpeerrelaypacke(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013536_rage_netsocketmanager_send_rage_netsocketaddress_const_ref_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_013537_rage_netconnectionmanager_addsentpacketstats_rage_netaddress_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_013538_rage_netrelay_send_rage_netaddress_const_ref_rage_netrelaypacket(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013539_rage_netconnectionmanager_closeconnection_int_rage_netclosetype(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013541_rage_netconnectionrouter_getnodebypeerid_rage_netpeerid_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013542_rage_netconnectionrouter_routebroken_rage_netconnectionrouter_no(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013543_rage_netconnectionrouter_node_removeprecursor_rage_netconnection(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013544_rage_netconnectionrouter_switchtorelayserver_rage_netpeerid_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013550_rage_netconnectionmanager_send_int_void_const_ptr_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_013551_rage_netconnectionmanager_checkexceededbandwidth_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013552_rage_netconnectionmanager_sendoutofband_rage_netaddress_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_013553_rage_netconnectionmanager_sendconnectionless_rage_netaddress_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_013554_rage_netconnectionmanager_getaddress_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_013555_rage_netconnectionmanager_getrelayaddress_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_013556_rage_netconnectionmanager_getpeerid_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_013557_rage_netconnectionmanager_getqos_unsigned_int_rage_netendpoint_e(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_013560_rage_netconnectionmanager_getageofoldestunackedframe_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013561_rage_netconnectionmanager_addchanneldelegate_rage_netconnectionm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_013562_rage_netsocketaddress_tostring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013563_rage_netconnectionrouter_trackroutepair_rage_netpeerid_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_013564_rage_netconnectionrouter_node_isrouteactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013565_rage_netconnectionrouter_getnexthopnode_rage_netpeerid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013566_rage_netaddress_isdirectaddr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013567_rage_netconnectionrouter_processunreachabledestinations_rage_net(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_013568_rage_nettunneler_getp2pkey_rage_netaddress_const_ref_rage_netp2p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013569_rage_netp2pcrypt_encrypt_rage_netp2pcrypt_key_const_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_013570_rage_netsocketaddress_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013571_rage_netsocket_send_rage_netsocketaddress_const_ref_void_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_013572_rage_netconnectionmanager_p2pdecrypt_rage_netaddress_const_ref_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_013573_rage_netconnectionmanager_unpack_rage_netaddress_const_ref_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_013574_rage_netaddress_init_rage_netsocketaddress_const_ref_rage_netpee(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013575_rage_netp2pcrypt_decrypt_rage_netp2pcrypt_key_const_ref_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013576_rage_netconnectionmanager_getpendingendpointbyaddr_rage_netaddre(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013577_rage_netaddress_isequivalent_rage_netaddress_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013579_rage_netfunctionprofiler_pushprofiler_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013580_rage_netfunctionprofiler_popprofiler_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_013581_rage_netconnectionmanager_dispatch(ph: *mut Phantom); }
extern "C" { pub fn f_013582_rage_netconnectionmanager_updatetunnelrequests(ph: *mut Phantom); }
extern "C" { pub fn f_013583_rage_netfunctionprofiler_dump_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013585_rage_netconnectionmanager_dispatchevent_rage_netevent_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013586_rage_dumpendpoint_rage_netendpoint_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013588_rage_netconnectionmanager_sendtunnelingpacket_rage_netaddress_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_013589_rage_netevent_getendpointid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013590_rage_netconnectionrouter_node_setnextdiscoverytime_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013591_rage_netconnectionrouter_node_sethopcount_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013592_rage_netconnectionrouter_newconnectionestablished(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013593_rage_netconnectionrouter_node_dumproutingentry_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013594_rage_netconnectionmanager_getconnectionid_rage_netaddress_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013595_rage_nettunnelrequest_getnetaddress_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013596_rage_netaddress_getrelaytoken_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013597_rage_netrelaytoken_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013598_rage_netpeerid_op_rage_netpeerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013599_rage_netaddress_op_rage_netaddress_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013600_rage_netaddress_op_rage_netaddress_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013601_rage_netconnectionmanager_updateendpointaddress_rage_netpeerid_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_013602_rage_netpeerid_op_rage_netpeerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013603_rage_netpeeraddress_getlocalpeeraddress_int_rage_netpeeraddress(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_013604_rage_netrelay_cancelconnect_rage_netstatus_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013605_rage_netkeyexchange_cancelkeyexchange_rage_netstatus_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013606_rage_netconnectionmanager_establishconnection_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_013608_rage_netconnectionmanager_getendpointid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013609_rage_netconnectionmanager_getchannelid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013610_rage_netconnectionmanager_isopen_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013611_rage_netconnectionmanager_isclosed_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013612_rage_netconnectionmanager_ispendingopen_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013613_rage_netkeyexchangeentry_ismatch_rage_netaddress_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013614_rage_netp2pcrypt_generatehmac_unsigned_char_const_ptr_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_013616_rage_netrelaypacket_resetp2pheader_unsigned_int_rage_netaddress(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_013617_rage_netrelay_senddata_rage_netsocketaddress_const_ref_bool_void(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_013618_bool_rage_msgcxnrelayaddrchanged_sermsgpayload_rage_datexportbuf(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013619_rage_nethardware_getpublicipaddress_rage_netipaddress_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013620_rage_netipaddress_op_rage_netipaddress_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013626_rage_netconnectionrouter_node_addprecursor_rage_netconnectionrou(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013627_rage_inmap_base_rage_netconnectionrouter_routepair_key_rage_netc(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013628_rage_netconnectionrouter_getnexthoppeerid_rage_netpeerid_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013629_rage_netconnectionrouter_blacklistpeer_rage_netpeerid_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013630_rage_netconnectionrouter_multicastrouterequest_rage_netrouterequ(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013631_rage_netrouterequestmsg_dump_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013632_rage_netconnectionrouter_unicastroutereply_rage_netroutereplymsg(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013633_rage_netconnectionrouter_ispeerblacklisted_rage_netpeerid_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013634_rage_netrouteerrmsg_dump_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013635_rage_netaddress_netaddress_rage_netsocketaddress_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_013636_rage_netpeerid_export_void_ptr_unsigned_int_unsigned_int_ptr_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013637_rage_netconnectionrouter_sequencenumber_export_void_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013638_bool_rage_datexportbuffer_seruns_unsigned_int_const_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013639_bool_rage_datimportbuffer_seruns_unsigned_int_unsigned_int_ref_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013640_rage_netcrypto_init(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_013641_rage_rc4context_rc4context(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013642_rage_rc4context_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013643_rage_rc4context_reset_unsigned_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_013644_rage_rc4context_encrypt_unsigned_char_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_013645_rage_netp2pcrypt_key_dtorkey(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013646_rage_netp2pcrypt_publickey_export_void_ptr_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013650_rage_netp2pcrypt_getlocalpublickey(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_013651_rage_netp2pcrypt_generatep2pkey_rage_netp2pcrypt_publickey_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013665_rage_nethttprequest_dtornethttprequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013666_rage_nethttpfilter_dumphttp_char_const_ptr_char_const_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_013667_rage_nethttprequest_dumphttp_char_const_ptr_char_const_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_013668_rage_nethttprequest_nethttprequest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013669_rage_nettcpasyncop_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013670_rage_netrandom_reset_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013671_rage_nethttprequest_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013672_rage_nettcpasyncop_dtornettcpasyncop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013673_rage_nethttprequest_geturi_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013674_rage_nethttprequest_abort_rage_netstatus_statuscode_rage_nethttp(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_013675_rage_nethttprequest_deletechunk_rage_nethttprequest_chunk_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013676_rage_nettcp_close_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_013677_rage_nethttprequest_init_rage_sysmemallocator_ptr_wolfssl_ctx_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013678_rage_nethttprequest_pending_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013679_rage_nethttprequest_cancel_rage_nethttpstatuscodes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013680_rage_nettcp_completeop_unsigned_int_rage_netstatus_statuscode_ra(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_013681_rage_nettcp_freesocketinfo_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_013682_rage_netsocketaddress_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013683_rage_netipaddress_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013684_rage_nethttprequest_begin_rage_nethttpverb_char_const_ptr_rage_n(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64) -> i32; }
extern "C" { pub fn f_013685_rage_nethttprequest_startrequest_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013686_rage_nethttprequest_appenduseragentstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013687_rage_nethttprequest_beginget_char_const_ptr_rage_netsocketaddres(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_013688_rage_nethttprequest_beginpost_char_const_ptr_rage_netsocketaddre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_013689_rage_nethttprequest_commit(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013690_rage_nethttprequest_urlencode_char_ptr_unsigned_int_ptr_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_013691_rage_netstatus_setstatus_rage_netstatus_statuscode_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013693_rage_nethttprequest_hasrequestheader_char_const_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013694_rage_nethttprequest_hasheadervalue_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013695_rage_nethttprequest_addrequestheadervalue_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013696_rage_nethttprequest_addrequestheadervalue_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_013698_rage_nethttprequest_addformparam_char_const_ptr_char_const_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_013699_char_ptr_rage_formatf_char_2048ul_char_ref_2048ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013700_rage_nethttprequest_queuerawdata_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013701_rage_nethttprequest_queueurlencodeddata_void_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013702_rage_nethttprequest_newchunk(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013703_rage_nethttprequest_addqueryparam_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_013704_rage_nethttprequest_urlencode_char_ptr_unsigned_int_ptr_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_013705_rage_nethttprequest_addformparam_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013706_rage_nethttprequest_addintformparam_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013707_rage_nethttprequest_addunsformparam_char_const_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013708_rage_nethttprequest_adddoubleformparam_char_const_ptr_double_int(ph: *mut Phantom, a0: i64, a1: f64) -> i32; }
extern "C" { pub fn f_013709_rage_nethttprequest_addbinaryformparam_char_const_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_013710_rage_nethttprequest_beginlistformparam_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013711_rage_nethttprequest_addlistformparamvalue_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013713_rage_nethttprequest_appendcontent_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013714_rage_nethttprequest_appendcontentzerocopy_void_const_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013715_rage_nethttprequest_succeeded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013716_rage_nethttprequest_getstatuscodestring_char_ptr_unsigned_int_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013717_rage_nethttprequest_getstatuscode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013718_rage_nethttprequest_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013719_rage_nettimeout_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013720_rage_nethttprequest_getresponseheadervalue_char_const_ptr_char_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_013721_rage_nethttprequest_getheadervalue_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_013722_rage_nethttprequest_setsucceeded(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013723_rage_nethttprequest_readbodyonce_unsigned_char_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_013724_rage_nethttprequest_consumebody_unsigned_char_ptr_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_013726_rage_nettcp_connectasync_rage_netsocketaddress_const_ref_rage_ne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_013728_rage_nettcp_sendbufferasync_int_void_const_ptr_unsigned_int_unsi(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_013729_rage_nettcp_receive_int_void_ptr_unsigned_int_unsigned_int_ptr_i(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_013730_rage_nethttprequest_processbouncecontent_unsigned_char_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_013731_rage_nethttprequest_getresponsecontentlength_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013732_rage_nethttprequest_makedatestring_unsigned_long_char_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013734_rage_netsocketaddress_init_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013735_rage_nettimeout_initseconds_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013736_rage_nettcp_queueop_rage_nettcpasyncop_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013737_rage_nettcpasyncop_complete_rage_netstatus_statuscode_rage_nettc(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_013738_rage_nettcp_connectfd_rage_netsocketaddress_const_ref_int_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013739_rage_nethttprequest_isbinaryresponse(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013740_rage_nettcp_getsocketcontextstr_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_013741_rage_nethttprequest_readbody_unsigned_char_ptr_unsigned_int_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_013742_rage_nethttprequest_seturlencode_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013766_rage_netkeyexchangesession_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013768_rage_netkeyexchangelogger_getelapsedtime(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013769_rage_netkeyexchangesession_sendanswer_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_013770_rage_netkeyexchangesession_complete_rage_netkeyexchangesession_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013771_rage_netkeyexchangeentry_update_rage_netaddress_const_ref_rage_n(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_013772_rage_netkeyexchange_freekey_rage_netkeyexchangeentry_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013773_rage_netkeyexchangeentry_init_rage_netpeerid_const_ref_rage_neta(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_013774_rage_netretrytimer_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013775_rage_netkeyexchangetransaction_send(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013776_bool_rage_netkeyexchangesessionoffer_sermsgpayload_rage_datimpor(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013777_rage_netkeyexchangesession_sendoffer(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013778_rage_netstatus_forcesucceeded_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013779_rage_netstatus_forcefailed_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013780_rage_netkeyexchangesession_init_unsigned_int_unsigned_int_rage_n(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_013789_rage_netkeyexchange_onpeerdiscovered_rage_netpeerid_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013790_rage_netkeyexchangeentry_update_rage_netaddress_const_ref_rage_n(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_013791_rage_netkeyexchangeentry_init_rage_netpeerid_const_ref_rage_neta(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_013792_rage_netkeyexchangeentry_dump(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013793_rage_netkeyexchangeentry_setaddr_rage_netaddress_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013794_rage_netkeyexchange_allocatesession(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_013806_bool_rage_datimportbuffer_seruser_rage_netsocketaddress_rage_net(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013807_bool_rage_datimportbuffer_seruns_unsigned_char_unsigned_char_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013808_bool_rage_datimportbuffer_seruns_unsigned_short_unsigned_short_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013809_rage_netsocketaddress_export_void_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013810_bool_rage_datexportbuffer_seruns_unsigned_char_const_unsigned_ch(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013811_bool_rage_datexportbuffer_seruns_unsigned_short_const_unsigned_s(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013812_bool_rage_datexportbuffer_seruser_rage_netsocketaddress_const_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013813_rage_netrelay_getmyrelayaddress(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_013815_rage_netnatdetector_updatergsc(ph: *mut Phantom); }
extern "C" { pub fn f_013816_rage_netnatinfo_dumpinfo_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013817_rage_netsocket_setinitialvalues(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013818_rage_netnatportmapper_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013819_rage_nethardware_destroysocket_rage_netsocket_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013820_rage_netsocket_dtornetsocket(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013821_rage_netnatportmapper_init_rage_netsocket_ptr_unsigned_int_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_013822_rage_nethardware_createsocket_rage_netsocket_ptr_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_013824_rage_netnatportmapper_updateone_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013825_rage_netrelaypacket_resetpingheader_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013826_rage_netnatportmapper_updateonealtport_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013827_rage_netnatportmapper_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013828_rage_netsocket_select_int_fd_set_ptr_fd_set_ptr_fd_set_ptr_timev(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_013829_rage_netsocket_receive_rage_netsocketaddress_ptr_void_ptr_int_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_013830_rage_netnatportmapper_mapport_unsigned_int_rage_netsocketaddress(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_013831_rage_netsocket_isreadytosend_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013832_rage_netnatsharedsocketportmapper_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013835_rage_netrelay_removedelegate_rage_netrelayeventtype_rage_atdeleg(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_013836_rage_netnatsharedsocketportmapper_updateone_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013837_rage_netnatsharedsocketportmapper_mapport_unsigned_int_rage_nets(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_013838_rage_netnatcombinedportmapper_netnatcombinedportmapper(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013840_rage_netnatporttimeoutchecker_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013841_rage_netnatporttimeoutchecker_dtornetnatporttimeoutchecker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013844_rage_netrelaytoken_export_void_ptr_unsigned_int_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013856_rage_netnatpcpinfo_dumpinfo_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013857_rage_netnatupnpinfo_dumpinfo_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013862_rage_netipaddress_import_void_const_ptr_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_013863_rage_netipaddress_export_void_ptr_unsigned_int_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013867_rage_nethardware_getmacaddress_unsigned_char_ref_6(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013869_rage_netpool_rage_nettaskimpl_256ul_netpool(ph: *mut Phantom); }
extern "C" { pub fn f_013870_rage_netpool_rage_nettaskworkitem_64ul_netpool(ph: *mut Phantom); }
extern "C" { pub fn f_013872_rage_nettask2_getroottask(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_013873_rage_nettask2_updatetask_rage_nettaskimpl_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013874_rage_neteventqueue_rage_netrelayevent_nexteventinternal_rage_net(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013875_rage_netrelay_connect_rage_netsocketaddress_const_ref_bool_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_013876_rage_netrelay_initendpoints(ph: *mut Phantom); }
extern "C" { pub fn f_013878_rage_neteventqueue_rage_netrelayevent_unsubscribeinternal_rage_n(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013880_rage_netrandom_netrandom_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_013881_rage_netrandom_getfloat(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_013882_rage_netipv4address_netipv4address(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013883_rage_netipv4address_netipv4address_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_013884_rage_netipv4address_fromstring_char_const_ptr_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013886_rage_netipaddress_fromstring_char_const_ptr_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013887_rage_netsocketaddress_netsocketaddress(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013888_rage_netsocketaddress_netsocketaddress_rage_netipaddress_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_013891_rage_netaddress_netaddress(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013892_rage_netaddress_netaddress_rage_netsocketaddress_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013893_rage_netaddress_netaddress_rage_netsocketaddress_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_013894_rage_netaddress_export_void_ptr_unsigned_int_unsigned_int_ptr_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_013895_bool_rage_datimport_rage_datimportbuffer_const_rage_netrelaytoke(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013896_bool_rage_datimport_rage_datimportbuffer_const_rage_netpeerid_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013901_rage_netmemallocator_printbasicmemoryusage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013902_rage_netallocationprofiler_printstats_rage_netmemallocator_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013928_rage_netsocket_unregisterpacketrecorder_rage_netpacketrecorder_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013934_rage_netgeneratenonce(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_013938_rage_netsocket_netsocket(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013939_rage_netsocketmanager_endpoint_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013941_rage_netsocketmanager_destroysockets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013942_rage_netretrytimer_shutdown(ph: *mut Phantom); }
extern "C" { pub fn f_013944_rage_netretrytimer_initseconds_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013945_rage_netretrytimer_initmilliseconds_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013946_rage_netretrytimer_forceretry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013947_rage_netsocketmanager_destroyendpoint_rage_netsocketmanager_endp(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013948_rage_netsocketmanager_onreceivenonrelaypacket_rage_netsocketaddr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_013949_rage_netretrytimer_istimetoretry_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013950_rage_netretrytimer_restart(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013951_rage_netsocketmanager_destroysymmetricsocket_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013952_rage_inmap_base_unsigned_int_rage_netsocketmanager_endpoint_refr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013953_rage_netsocketmanager_associatesymmetricsocket_unsigned_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_013954_rage_netsocketmanager_getsymmetricsocketaddress_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_013955_rage_netsocketmanager_getsymmetricsocket_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_013956_rage_netrelay_getcreateendpoint_rage_netsocketaddress_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013958_rage_netpeeraddress_hasnetwork(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_013959_rage_netpeeraddress_netpeeraddress(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013960_rage_netpeeraddress_clearnonadvertisabledata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013961_rage_netpeeraddress_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013962_rage_netpeeraddress_exportinternal_void_ptr_unsigned_int_bool_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_013963_rage_netpeeraddress_importinternal_void_const_ptr_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_013964_rage_netpeeraddress_fromstring_char_const_ptr_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013965_rage_netpeeraddress_getprivateaddress_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013976_rage_netpeercomplainer_unregistercomplaint_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013977_rage_netpeercomplainer_resendcomplaints(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013978_bool_rage_netconnectionmanager_sendoutofband_rage_netcomplaintms(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_013981_void_std_2_introsort_std_2_classicalgpolicy_rage_bootcandidateco(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_013983_rage_netdtlsendpoint_dtornetdtlsendpoint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013989_rage_serverinfo_dtorserverinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013995_rage_neteventqueue_rage_netrelayevent_remove_rage_neteventbase_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013997_rage_netrelay_freeendpoint_rage_netdtlsendpoint_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013998_rage_netretrytimer_initmilliseconds_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013999_rage_netretrytimer_stop(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014001_rage_netretrytimer_scalerange_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014002_rage_netretrytimer_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014004_rage_neteventqueue_rage_netrelayevent_queueevent_rage_netrelayev(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014005_rage_netrelay_handlerelaypong_rage_netrelaypacket_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014006_rage_netrelay_handlepresencepong_rage_netrelaypacket_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014007_rage_netrelay_handlemessage_rage_netrelaypacket_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014009_rage_netrelayeventaddresschanged_netrelayeventaddresschanged_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_014013_rage_nettask_dtornettask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014015_rage_nettask_destroy_rage_nettask_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014016_rage_nettask_run_rage_nettask_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014017_rage_nettask_wascanceled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014018_rage_nettask_find_rage_nettaskbase_ptr_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014019_rage_nettask_hastask_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014020_rage_nettask_cancel_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014021_rage_nettask_cancel_rage_nettask_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014023_rage_nettask_nettask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014028_rage_nettask_allocate_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014029_rage_nettask2_freetask_rage_nettaskimpl_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014030_rage_nettaskimpl_addchild_rage_nettaskimpl_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014033_rage_netpool_rage_nettaskworkitem_64ul_free_rage_nettaskworkitem(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014034_rage_nettaskimpl_dtornettaskimpl(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014036_rage_nettcp_allocsocketinfo_char_const_ptr_rage_connectreason(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014039_rage_nettcp_sendbuffer_int_void_const_ptr_unsigned_int_unsigned(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_014040_rage_nettcp_cancelopsbysocketid_int_rage_inlist_rage_nettcpasync(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_014042_rage_nettcp_send_int_void_const_ptr_unsigned_int_unsigned_int_pt(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_014043_rage_nettcp_getinsecuresslctx(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_014045_rage_nettcp_getsocketaddr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014046_xmalloc(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014050_rage_nettimeout_isrunning_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014051_rage_netretrytimer_netretrytimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014052_rage_netretrytimer_dtornetretrytimer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014053_rage_netretrytimer_resetintervalmilliseconds_int_int(ph: *mut Phantom); }
extern "C" { pub fn f_014054_rage_netretrytimer_getcurrentretryintervalseconds_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014055_rage_netretrytimer_getsecondsuntilretry_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014059_bool_rage_netconnectionmanager_sendoutofband_rage_nettimesyncmsg(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014060_rage_nettimesync_dtornettimesync(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014070_rage_nettunnelrequest_nettunnelrequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014071_rage_nettunnelrequest_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014072_rage_nettunnelrequest_dtornettunnelrequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014073_rage_nettunnelrequest_queryrelayaddress(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014074_rage_rlstandardmetrics_tunnelerresult_write_rage_rsonwriter_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014077_rage_neticesession_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014078_rage_neticetunneler_allocatesession(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_014079_rage_neticesession_init_unsigned_int_unsigned_int_rage_netconnec(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_014080_rage_neticesession_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014089_rage_symmetricinference_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014090_rage_symmetricsocket_symmetricsocket(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014091_rage_symmetricsocket_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014092_rage_symmetricsocket_dtorsymmetricsocket(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014093_rage_symmetricinference_dtorsymmetricinference(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014094_rage_neticesession_complete_rage_neticesession_completiontype_ra(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_014095_rage_neticesession_cansendviarelay_rage_neticesession_failreason(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014096_rage_neticesession_detach_rage_neticesession_failreason(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014097_rage_neticesession_report_rage_neticesession_completiontype_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_014098_rage_candidatepair_tostring_char_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014099_rage_neticetransaction_send(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014100_rage_neticesession_needssymmetricsocket_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014101_rage_neticesession_startsymmetricsocket(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014102_rage_neticesession_sendindirectoffer_rage_neticesession_failreas(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014103_rage_neticesession_startcrossrelaycandidatecheck(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014104_rage_neticesession_indirectoffertimedout(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014105_rage_candidatepair_setstate_rage_candidatepair_pairstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014106_rage_checklist_setstate_rage_checklist_checkliststate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014107_rage_rlstandardmetrics_nattraversal_write_rage_rsonwriter_ptr_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014108_rage_neticesession_sendportopenermessage_rage_netsocketaddress_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014109_rage_neticesession_sendinitialoffers(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014110_rage_neticesession_addremotecandidatetochecklist_rage_candidate(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_014111_rage_candidate_tostring_char_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014112_rage_neticesession_iscontrolling_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014113_rage_neticesession_gatherlocalcandidates(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014114_rage_neticesession_gatherremotecandidates(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014115_rage_neticesession_createchecklist(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014116_rage_neticesession_addremotesymmetricinferencecandidates(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014117_rage_neticesession_getadditionallocalcandidates_rage_netsocketad(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014118_rage_neticesessionoffer_neticesessionoffer_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i32, a7: i64, a8: i32, a9: i32, a10: i32, a11: i64, a12: i64, a13: i64, a14: i64) -> i64; }
extern "C" { pub fn f_014119_bool_rage_neticetransactor_sendrequest_rage_neticesessionoffer_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_014120_rage_neticesession_nominatepair_rage_candidatepair_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014121_rage_neticesessionoffer_neticesessionoffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014122_bool_rage_neticesessionoffer_sermsgpayload_rage_datimportbuffer(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014123_rage_neticesession_sendrelayroutecheckrequest(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014124_bool_rage_netconnectionmanager_sendtunnelingpacket_rage_neticese(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_014125_rage_remotecandidates_addcandidate_rage_remotecandidate_const_re(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014126_rage_triggeredcheckqueue_push_rage_candidatepair_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014127_rage_discoveredpairs_addcandidatepair_rage_candidatepair_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014128_rage_checklist_addcandidatepair_rage_candidatepair_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014129_rage_candidatelist_rage_remotecandidate_10u_addcandidate_rage_re(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014130_rage_triggeredcheckqueue_findmatchingpair_rage_candidatepair_con(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014131_bool_rage_netconnectionmanager_sendtunnelingpacket_rage_neticese(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_014132_rage_neticesessionping_neticesessionping(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014133_bool_rage_neticesessionping_sermsgpayload_rage_datimportbuffer_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014134_rage_localcandidates_addcandidate_rage_localcandidate_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014136_rage_neticetransactor_sendrequest_unsigned_int_rage_netp2pcrypt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_014137_rage_neticesessiondebuginfo_dumpinfo_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014138_rage_neticetunneler_getpeerreflexiveaddresses_rage_netsocketaddr(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014139_rage_neticesession_sendanswer_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_014140_bool_rage_netconnectionmanager_sendtunnelingpacket_rage_neticese(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_014141_rage_neticesession_needssymmetricinference_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014142_rage_neticesession_startsymmetricinference(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014143_rage_neticesession_addremotecandidates_rage_neticesessionoffer_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_014144_rage_neticesession_findfreenetstatusindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014150_rage_netrequesthandler_netrequesthandler(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014152_rage_netrequesthandler_dtornetrequesthandler(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014153_rage_nettransactor_removerequesthandler_rage_netrequesthandler_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014154_rage_netresponsehandler_netresponsehandler(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014156_rage_netresponsehandler_timeout(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014157_rage_netresponsehandler_dtornetresponsehandler(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014158_rage_netresponsehandler_cancel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014159_rage_nettransactor_abortrequest_rage_netresponsehandler_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014160_rage_nettransactor_nettransactor(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014161_rage_nettransactor_dtornettransactor(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014162_rage_nettransactor_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014163_rage_nettransactor_init_rage_netconnectionmanager_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014165_rage_nettransactor_update_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014166_rage_nettransactor_addrequesthandler_rage_netrequesthandler_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_014167_rage_nettransactor_sendrequest_int_unsigned_int_ptr_void_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_014168_rage_nettransactor_sendrqstbufoutofband_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_014169_rage_netconnectionmanager_sendoutofband_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_014170_rage_nettransactor_sendresponse_rage_nettransactioninfo_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_014194_bool_rage_datimportbuffer_seruser_rage_netconnectionrouter_seque(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014200_bool_rage_datimportbuffer_seruser_rage_netpeeraddress_rage_netpe(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014201_bool_rage_datimportbuffer_seruser_rage_netp2pcrypt_publickey_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014238_rage_neticetransaction_neticetransaction(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014239_bool_rage_datimportbuffer_serarraysize_unsigned_char_rage_netsoc(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014243_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_bootc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_014244_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014245_bool_rage_datexportbuffer_serarraysize_unsigned_char_const_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014248_rage_rltaskmanager_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014255_rage_rlcheckhostsfile_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014259_rage_rltaskmanager_doaddtask_unsigned_long_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014260_rage_rltaskmanager_destroytask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014261_rage_rlros_shutdownclass(ph: *mut Phantom); }
extern "C" { pub fn f_014267_rage_rlpresence_refreshsigninstate_int(ph: *mut Phantom); }
extern "C" { pub fn f_014268_rage_rlpresence_refreshnetworkstatus_int(ph: *mut Phantom); }
extern "C" { pub fn f_014270_rage_rlfriendsmanager_clear(ph: *mut Phantom); }
extern "C" { pub fn f_014271_rage_rltaskmanager_cancelall_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014274_rage_rlfriendspage_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014275_rage_rltaskmanager_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014277_rage_rltaskmanager_updatetask_rage_rltaskbase_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_014278_rage_ficloudhandle_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014279_rage_rltelemetry_flush(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014283_rage_rlfriendsreader_read_int_unsigned_int_rage_rlfriend_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i64, a6: i64, a7: i32, a8: i64) -> i32; }
extern "C" { pub fn f_014285_rage_rlcloud_unwatchmemberitem_rage_rlcloudwatcher_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014286_void_rage_rldelete_rage_rlscmatch_rage_rlscmatch_ptr_unsigned_in(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014287_rage_rltaskbase_cancel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014288_rage_rltaskmanager_canceltask_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014289_rage_rltaskmanager_canceltask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014290_rage_rlrosgetforcedenvironment(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014292_rage_rlgettaskmanager(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_014294_rage_rlgetposixtime_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014295_rage_rlcreateuuid_unsigned_long_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014296_rage_rlcreateuuid_rage_rluuid64_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014297_rage_rlloggedallocate_unsigned_long_char_const_ptr_int_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014301_rage_rlgamerhandle_equals_rage_rlgamerhandle_const_ref_bool_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_014303_rage_rlgamerhandle_op_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014306_rage_rlworker_start_char_const_ptr_unsigned_int_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_014309_rage_rlworker_stop(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014315_rage_rlgamerinfo_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014316_rage_rlgamerinfo_getgamerhandle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014318_rage_rlgamerinfo_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014319_rage_rlgamerhandle_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014320_rage_rlpc_isinitialized(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_014321_rage_rlgamerhandle_getrockstarid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014349_rage_rlgamerhandle_touserid_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014353_rage_rlfriendsmanager_canceltaskifnecessary_rage_netstatus_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014354_rage_rlpresence_isonline_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014361_rage_rlros_getcredentials_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014365_rage_rltaskmanager_findtask_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014368_rage_rlgamerhandle_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014369_rage_rlgamerhandle_tostring_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014370_rage_rlgamerhandle_tostring_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014371_rage_rlgamerhandle_fromstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014372_rage_rlgamerhandle_fromuserid_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014373_rage_rlgamerhandle_getservicefromstring_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014374_rage_rlgamerhandle_export_void_ptr_unsigned_int_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014375_rage_rlgamerhandle_import_void_const_ptr_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014380_rage_rlgamerid_tohexstring_char_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014381_rage_rlgamerid_export_void_ptr_unsigned_int_unsigned_int_ptr_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014382_rage_rlgamerid_import_void_const_ptr_unsigned_int_unsigned_int_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014384_rage_rlpresence_issignedin_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014387_rage_rlgamerinfo_rlgamerinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014388_rage_rlgamerinfo_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014389_rage_rlgamerinfo_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014390_rage_rlgamerinfo_isremote_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014392_rage_rlgamerinfo_getgamerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014393_rage_rlgamerinfo_export_void_ptr_unsigned_int_unsigned_int_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014394_rage_rlpeerinfo_export_void_ptr_unsigned_int_unsigned_int_ptr_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014395_rage_rlgamerinfo_op_rage_rlgamerinfo_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014396_rage_rlpeerinfo_op_rage_rlpeerinfo_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014398_rage_rlmatchingattributes_rlmatchingattributes(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014403_rage_rlmatchingattributes_export_void_ptr_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014404_rage_rlmatchingattributes_import_void_const_ptr_unsigned_int_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014411_rage_rlpeerinfo_getlocalpeerinfo_rage_rlpeerinfo_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014412_rage_rlpeerinfo_rlpeerinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014413_rage_rlpeerinfo_init_unsigned_long_rage_netpeeraddress_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_014415_rage_rlpeerinfo_getpeeraddress_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014442_rage_internal_getgamerhandle_int_rage_rlgamerhandle_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014445_rage_rlpresence_getgamerhandle_int_rage_rlgamerhandle_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_014447_rage_rlpresence_getgamerinfo_int_rage_rlgamerinfo_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_014449_rage_rlpresence_adddelegate_rage_atdelegator1_void_rage_rlpresen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014457_rage_rlpresence_getattributesforgamer_int_rage_rlgamerhandle_con(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_014459_rage_rltaskbase_configure_rage_netstatus_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014460_rage_rltaskmanager_addparalleltask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014461_rage_rlclan_getprimaryclan_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014462_rage_rlpresence_postmessage_int_rage_rlgamerhandle_const_ptr_uns(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014463_rage_rlpresence_query_int_char_const_ptr_char_const_ptr_int_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64, a8: i64, a9: i64) -> i32; }
extern "C" { pub fn f_014465_rage_internal_getofflinename_int_char_ptr_unsigned_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014467_rage_rlsession_ishost_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014468_rage_rlscmatchmakingmatchid_tostring_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014471_rage_rlrosclient_setcredentials_rage_rlroscredentials_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014472_rage_rlsocialclub_setlogin_int_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014473_bool_rage_rlscpresencemessage_isa_rage_rlscpresencemessagempinvi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014474_rage_rlscpresencemessagempinvite_import_rage_rsonreader_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014477_rage_rlroscreateticketresponse_rlroscreateticketresponse(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014478_rage_rlrosxmlcreateticketresponseparser_parse_rage_partreenode_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014479_rage_rlroscreateticketresponsehelper_applycreateticketresponse_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014481_rage_rlpresenceattributehelper_setbool_int_bool_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_014482_rage_rlpresenceattributehelper_getbool_int_bool_ref_char_const_p(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014483_rage_rlpresenceattributehelper_setsessiontoken_int_rage_rlsessio(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_014484_rage_rlpresenceattributehelper_setsessionid_int_unsigned_long_ch(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_014485_rage_rlpresenceattributehelper_setsessioninfo_int_rage_rlsession(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_014486_rage_rlsessioninfo_tostring_char_ptr_unsigned_int_unsigned_int_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_014487_rage_rlsessioninfo_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014493_rage_rlugc_checktext_int_char_const_ptr_char_const_ptr_char_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014494_rage_rlsocialclub_checktext_int_char_const_ptr_rage_rlsclanguage(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_014495_rage_rlprofanitycheck_extractprofaneword_rage_partreenode_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014496_rage_rlhttptaskhelper_readstring_rage_partreenode_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_014499_rage_rlsession_getsessioninfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014500_rage_rlsession_data_data(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014502_rage_rlsession_data_reset_rage_rlgamerinfo_const_ref_unsigned_lo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i64, a8: i32, a9: i32, a10: i64, a11: i64) -> i32; }
extern "C" { pub fn f_014503_rage_rlsession_data_logdetails_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014504_rage_rlsession_data_validate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014506_rage_rlsession_cmd_dtorcmd_23951(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014507_rage_rlsession_cmd_cancel(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014508_rage_rlsession_cmd_getcmdname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014509_rage_rlsession_cmdupdatescadvertisement_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014510_rage_rlscmatchmaking_advertise_int_unsigned_int_unsigned_int_rag(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_014511_rage_rlscmatchmaking_update_int_rage_rlscmatchmakingmatchid_cons(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_014512_rage_rlscmatchmaking_unadvertise_int_rage_rlscmatchmakingmatchid(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014513_rage_rlsession_isonline_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014515_rage_rlsession_getemptyslots_rage_rlslottype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014516_bool_rage_rltaskbase_configure_rage_rlscmatchmakingadvertisetask(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_014517_rage_rlsession_getowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014518_rage_rlsession_getmaxslots_rage_rlslottype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014519_bool_rage_rltaskbase_configure_rage_rlscmatchmakingupdatetask_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_014520_bool_rage_rltaskbase_configure_rage_rlscmatchmakingunadvertiseta(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014521_rage_rlsession_cmdupdatescadvertisement_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014523_rage_rlsession_cmddestroy_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014524_rage_rlsession_canunadvertisescmatch_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014525_rage_rlsession_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014526_rage_rlsession_cmdhost_cmdhost_rage_rlsession_ptr_rage_rlgamerin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i64) -> i64; }
extern "C" { pub fn f_014529_rage_rlsession_canadvertisescmatch_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014535_rage_rlsession_ismember_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014537_rage_rlsession_canupdatescmatch_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014542_rage_rlsession_dtorrlsession(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014543_void_rage_rldelete_rage_rlsession_cmd_rage_rlsession_cmd_ptr_uns(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014545_rage_rlsession_docmd_rage_rlsession_cmd_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014546_rage_rlsession_isestablished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014547_rage_rlsession_destroy_rage_netstatus_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014548_rage_rlsession_join_rage_rlgamerhandle_const_ptr_rage_rlslottype(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_014549_rage_rlsession_cancel_rage_netstatus_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014550_rage_rlsession_getpresenceflags_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014551_rage_rlsession_sendinvites_rage_rlgamerhandle_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014552_rage_rlsession_getconfig_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014553_rage_rlsession_getcreateflags_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014554_rage_rlsession_getfilledslots_rage_rlslottype_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014555_bool_rage_rlpresence_getname_17_int_char_ref_17(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_014556_rage_rlsessionconfig_export_void_ptr_unsigned_int_unsigned_int_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014560_rage_rlsessiondetail_reset_rage_rlpeerinfo_const_ref_rage_rlsess(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64, a9: i32, a10: i64, a11: i32); }
extern "C" { pub fn f_014566_bool_rage_rlsessiondetailrequest_sermsgpayload_rage_datexportbuf(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014567_rage_rlsessiondetail_rlsessiondetail(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014568_rage_rlsessionquerydetailtask_dtorrlsessionquerydetailtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014569_rage_rlsessionquerydetailtask_squery_dtorsquery(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014573_rage_rlsessioninfo_export_void_ptr_unsigned_int_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014574_bool_rage_datexportbuffer_seruser_rage_rlgamerhandle_const_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014575_rage_rlsessioninfo_import_void_const_ptr_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_014579_rage_rlsessionmanager_getsession_rage_rlsessioninfo_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014580_rage_rlsessioninfo_rlsessioninfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014581_rage_rlscmatchmakingmatchid_op_rage_rlscmatchmakingmatchid_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014582_rage_rlsessioninfo_op_rage_rlsessioninfo_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014583_rage_rlsessionmanager_querydetail_rage_rlnetworkmode_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i64, a9: i64, a10: i64, a11: i64) -> i32; }
extern "C" { pub fn f_014584_rage_rlsessionmanager_querydetail_rage_rlnetworkmode_unsigned_in(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014585_rage_rlsessionmanager_cancel_rage_netstatus_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014589_rage_rlfindsessionstask_dtorrlfindsessionstask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014592_bool_rage_rltaskbase_configure_rage_rlscmatchmakingfindtask_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_014595_rage_rlsessionfindsocialtask_dtorrlsessionfindsocialtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014599_long_std_2_uniform_int_distribution_long_op_std_2_rs_default_std(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014600_rage_rlsessioninfo_fromstring_char_const_ptr_unsigned_int_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_014605_rage_rlhttptask_rlhttptask_rage_sysmemallocator_ptr_unsigned_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_014606_rage_rlroshttptask_configure_int_rage_rlhttptaskconfig_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_014607_rage_rlhttptask_addintparameter_char_const_ptr_long_long_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_014608_rage_rlhttptask_addstringparameter_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014612_rage_rlleaderboard2readbyranktask_configure_int_int_rage_rlleade(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64, a9: i32, a10: i64, a11: i64, a12: i64) -> i32; }
extern "C" { pub fn f_014618_rage_rlleaderboard2readgamersbyscoretask_ptr_rage_rltaskmanager(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014626_rage_rltaskbase_finishstring_rage_rltaskbase_finishtype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014627_rage_rltaskbase_finishsucceeded_rage_rltaskbase_finishtype(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014628_rage_rltaskbase_rltaskbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014629_rage_rltaskbase_isactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014630_rage_rltaskmanager_removetask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014631_rage_inmap_base_rage_netstatus_const_ptr_rage_rltaskbase_refrage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_014632_rage_rltaskbase_dtorrltaskbase_24266(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014633_rage_rltaskbase_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014634_rage_rltaskbase_update_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014635_rage_rltaskbase_isstarted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014636_rage_rltaskbase_isfinished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014637_rage_rltaskbase_ispending_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014638_rage_rltaskbase_wascanceled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014640_rage_rltaskbase_finish_rage_rltaskbase_finishtype_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_014642_rage_inmap_base_unsigned_long_rage_rltaskbase_refrage_rltaskbase(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_014649_rage_rlrostelemetrysubmissiontask_configure_int_rage_rlmetriclis(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_014652_rage_zlibstream_beginencode_rage_sysmemallocator_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014653_rage_rlrostelemetrysubmissiontask_compressandappend_rage_zlibstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_014654_rage_rltelemetry_export_rage_rsonwriter_ptr_char_const_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_014655_rage_zlibstream_dtorzlibstream(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014656_rage_rlrostelemetrysubmissiontask_configure_rage_rltelemetry_gam(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014658_rage_rlhttptaskhelper_readuns_unsigned_int_ref_rage_partreenode(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014659_rage_rltelemetryconfigversionhistory_insert_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014660_rage_rlrostelemetryimmediatesubmissiontask_configure_rage_rltele(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014661_rage_rlhttptask_appendcontent_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_014662_rage_rlmetric_dtorrlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014666_rage_rltelemetrypolicies_applydebugmetrics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014667_rage_rltelemetrypolicies_loadmetricsfromjson_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_014668_rage_rltelemetrypolicies_applydefaultmetrics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014669_rage_atarray_rage_atbinarymap_rage_rlmetricconfig_rage_atnonfina(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014672_rage_rltelemetrypolicies_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014673_rage_atarray_rage_atbinarymap_rage_rlmetricconfig_rage_atnonfina(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014674_rage_rltelemetrypolicies_applydebugmetrics_char_const_ptr_ptr_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_014675_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_014676_rage_rltelemetrypolicies_applydebugmetricschannel_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_014677_rage_rltelemetrypolicies_dumpsettings_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014678_rage_rltelemetry_writelinetostream_char_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014683_rage_rlcloudgetfiletask_ptr_rage_rltaskmanager_createtask_rage_r(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014684_bool_rage_rltaskbase_configure_rage_rlcloudgetfiletask_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i64, a8: i32, a9: i64, a10: i64, a11: i64, a12: i64, a13: i64) -> i32; }
extern "C" { pub fn f_014685_rage_rlmetric_rlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014690_rage_rlmetric_isdisabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014691_rage_rlmetric_shouldincludeinsample_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014707_rage_rltelemetry_write_int_rage_rlmetric_const_ref(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_014708_rage_rltelemetry_dowritetostreams_int_unsigned_long_rage_rlgamer(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_014709_rage_rltelemetry_getslot_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014712_rage_rlfireandforgettask_rage_rlrostelemetryimmediatesubmissiont(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014713_bool_rage_rltaskbase_configure_rage_rlfireandforgettask_rage_rlr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014724_rage_rlhttptask_dtorrlhttptask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014726_rage_rlrosresult_iserror_char_const_ptr_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014731_rage_rlhttptaskhelper_readbool_bool_ref_rage_partreenode_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_014736_rage_rlhttptask_configure_rage_rlhttptaskconfig_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014737_rage_rlhttptask_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014738_rage_rlhttptask_finish_rage_rltaskbase_finishtype_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_014740_rage_rlhttptask_update_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014741_rage_rlhttpcdnuploadphototask_dtorrlhttpcdnuploadphototask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014764_char_ptr_rage_formatf_char_15ul_char_ref_15ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_014776_rage_rlprofilestatsreadstatsbygamertask_ptr_rage_rltaskmanager_c(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_014777_bool_rage_rltaskbase_configure_rage_rlprofilestatsreadstatsbygam(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014778_rage_rlprofilestatssynchronizegroupstask_ptr_rage_rltaskmanager(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_014779_bool_rage_rltaskbase_configure_rage_rlprofilestatssynchronizegro(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_014780_rage_rlprofilestatssynchronizetask_rlprofilestatssynchronizetask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014781_rage_rlprofilestatssynchronizetask_configure_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014790_rage_rlprofilestatsvalue_op_rage_rlprofilestatsvalue_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014797_rage_rlprofilestatsvalue_tostring_char_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014805_rage_rlprofilestatsreadresults_getvalue_unsigned_int_unsigned_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_014806_rage_rlprofilestatsreadresults_setnumrows_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014807_rage_atdelegate1_void_rage_rlprofilestatsevent_const_ptr_invoke(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014808_rage_rlprofilestatsstreamparser_parsestream_rage_rlprofilestatss(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_014809_rage_rlprofilestatsflushtask_dtorrlprofilestatsflushtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014812_rage_rlprofilestatsflushtask_setrandomdelayuntilnextflush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014817_rage_rlprofilestatsreadstatsbygamertask_dtorrlprofilestatsreadst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014826_rage_rlprofilestatsreadstatsbygamertask_getnextstatid_int_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014841_rage_rlprofilestatsconditionalsynchronizegroupstask_dtorrlprofil(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014854_rage_rlros_renewcredentials_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_014855_bool_rage_rlscpresencemessage_isa_rage_rlscpresenceinvalidatetic(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014856_rage_rlrosplatformcredentials_getlocalplatformcredentials_int_ra(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_014860_rage_rlros_getgeolocinfo_rage_rlrosgeolocinfo_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014861_rage_rlros_getservicehost_char_const_ptr_char_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014862_rage_rlros_issslservice_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014863_rage_rlros_processcommonerrors_int_rage_rlrosresult_const_ref(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_014866_rage_rlrosserviceaccesstokenstore_serviceaccessrequest_requestto(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014867_rage_rlrosserviceaccesstokenstore_serviceaccessrequest_invalidat(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014868_rage_rlrosclient_refreshcredtask_dtorrefreshcredtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014870_bool_rage_rltaskbase_configure_rage_rlroscreatetickettask_int_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_014887_rage_rlroscreateticketresponse_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014892_rage_rlroshttptask_processresponse_char_const_ptr_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014893_rage_rlroshttptask_processrosresult_rage_rlrosresult_rage_partre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_014896_rage_rlroscredentialschangingtask_processsuccess_rage_rlrosresul(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_014912_rage_rlroshttpfilter_configure_int_char_const_ptr_rage_rlrossecu(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_014921_rage_atdelegate3_bool_rage_rlroscontentsignature_const_ref_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_014940_rage_rlrostitleid_getenvironmentname_const(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_014943_rage_rlsctaskbase_rlsctaskbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014949_rage_rlsctaskbase_handleerror_rage_rlrosresult_const_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014968_rage_rlugccreatecontenttask_ptr_rage_rltaskmanager_createtask_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014969_rage_rlugccreatecontenttask_configure_int_rage_rlugccontenttype(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64, a7: i32, a8: i32, a9: i64, a10: i32, a11: i64, a12: i32) -> i32; }
extern "C" { pub fn f_014978_rage_rlugccontenttypetostring_rage_rlugccontenttype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014982_rage_rlugcmetadata_composecloudabspath_rage_rlugccontenttype_cha(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_014989_rage_rlugccreateurltemplatesfromjson_rage_rsonreader_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014990_rage_atarray_rage_rlugcurltemplate_0_unsigned_short_destruct_rag(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014991_rage_rlugcurltemplate_op_rage_rlugcurltemplate_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014992_rage_rlugcbuildurlfromtemplate_rage_atstring_const_ref_rage_atar(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_014993_rage_rlugctaskbase_readmetadatavalue_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014994_rage_rlugctaskbase_readplayervalue_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014995_rage_rlugctaskbase_readratingsvalue_char_const_ptr_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014996_rage_rlugctaskbase_processerror_rage_rlrosresult_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_014999_rage_rlugccreatecontenttask_writejson_char_const_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64, a8: i64) -> i32; }
extern "C" { pub fn f_015005_rage_atdelegate8_void_int_char_const_ptr_rage_rlugcmetadata_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64); }
extern "C" { pub fn f_015010_rage_rlugcupdatecontenttask_writejson_char_const_ptr_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_015012_rage_rlugcgetcdncontenttask_dtorrlugcgetcdncontenttask_25047(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015015_rage_populatefileinfo_rage_rlcloudfileinfo_ptr_rage_nethttpreque(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015018_rage_rlcloud_verifycloudsignature_rage_rlroscontentsignature_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_015032_bool_rage_rlscpresencemessage_isa_rage_rlscpresencemessagepublis(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015033_rage_rlscpresencemessagepublish_import_rage_rsonreader_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015034_rage_rlscpresencemessagecloudfilechanged_import_rage_rsonreader(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015035_char_ptr_rage_formatf_char_384ul_char_ref_384ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015036_rage_getmemberid_int_rage_rlcloudonlineservice_rage_rlcloudmembe(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_015038_rage_ficloudhandle_dtorficloudhandle(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015042_rage_rlcloud_createurl_rage_atstringbuilder_ptr_rage_rlcloudname(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_015043_char_ptr_rage_formatf_char_257ul_char_ref_257ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_015044_rage_getonlineservicestring_rage_rlcloudonlineservice(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_015049_rage_ficloudhandle_get_int_rage_rlcloudnamespace_rage_rlcloudonl(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32, a7: i64, a8: i32, a9: i64, a10: i64) -> i32; }
extern "C" { pub fn f_015051_rage_ficloudhandle_post_int_rage_rlcloudnamespace_rage_rlcloudon(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_015056_rage_ficloudhandle_close(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015075_bool_rage_rlscpresencemessage_isa_rage_rlscpresencemessageclanjo(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015076_rage_rlscpresencemessageclanjoined_import_rage_rsonreader_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015077_rage_rlclanclient_refreshmine_rage_rlclanclient_refreshqueuetype(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_015078_bool_rage_rlscpresencemessage_isa_rage_rlscpresencemessageclanle(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015079_rage_rlclanclient_removemembership_long_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015080_rage_rlclan_dispatchevent_rage_rlclanevent_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015081_rage_rlclan_updatecachedclanrank_int_long_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_015082_bool_rage_rlscpresencemessage_isa_rage_rlscpresencemessageclanno(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015083_rage_rlscpresencemessageclannotify_import_rage_rlscpresencemessa(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015084_rage_rlclanpresenceeventlistener_getgamerhandlefromchannel_char(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015093_rage_rlclanclient_queuetask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015100_bool_rage_rltaskbase_configure_rage_rlclanrosgetinvitestask_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_015102_rage_rlclanrosgetalltask_ptr_rage_rltaskmanager_createtask_rage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015103_bool_rage_rltaskbase_configure_rage_rlclanrosgetalltask_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32, a7: i64, a8: i32, a9: i64, a10: i64, a11: i64) -> i32; }
extern "C" { pub fn f_015110_rage_rlclanclient_addmembership_rage_rlclanmembershipdata_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_015111_rage_rlclanclient_updateprimaryclan_long_long(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015112_rage_rlclanrosrefreshminetask_configure_rage_rlclanclient_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015142_rage_rlclanrosbasetask_rlclanrosbasetask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015143_rage_rlclanrosbasetask_convertrosresulttoclanerror_rage_rlrosres(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015144_rage_rlclanrosbasetask_processextractclaninfo_rage_partreenode_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_015145_rage_rlclanrosbasetask_processextractplayerinfo_rage_partreenode(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015146_rage_rlclanrosbasetask_processextractrankinfo_rage_partreenode_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_015147_rage_rlclanrosbasetask_processmembershiprecord_rage_partreenode(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_015182_rage_rlclanrosgetmembershiptaskbase_processsuccess_rage_rlrosres(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_015198_rage_rlsessionbygamerhandletask_dtorrlsessionbygamerhandletask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015202_rage_rlfindgamersessionstask_dtorrlfindgamersessionstask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015210_rage_rlgetgamerstatetask_dtorrlgetgamerstatetask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015220_rage_rlsavemigrationstate_set_int_bool_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_015232_bool_rage_rlscpresencemessage_isa_rage_rlscpresenceaccountlinkre(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015233_rage_rlscpresenceaccountlinkresult_import_rage_rsonreader_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015234_rage_rlscauth_onerror(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015235_rage_rlscauth_formathostname_char_ptr_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015237_rage_rlscmatchmaking_unadvertiseall_int_rage_netstatus_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_015238_bool_rage_rltaskbase_configure_rage_rlscmatchmakingunadvertiseal(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015239_rage_rlscmatchmakingadvertisetask_rlscmatchmakingadvertisetask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015240_rage_rlscmatchmakingupdatetask_rlscmatchmakingupdatetask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015241_rage_rlscmatchmakingfindtask_rlscmatchmakingfindtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015242_rage_rlscmatchmakingunadvertisetask_rlscmatchmakingunadvertiseta(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015243_rage_rlscmatchmakingunadvertisealltask_rlscmatchmakingunadvertis(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015244_rage_rlscmatchmakingunadvertisealltask_configure_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015245_rage_populateattrsjson_rage_rsonwriter_ptr_rage_rlsessioninfo_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_015247_rage_rlscmatch_reset_int_rage_rlscmatchmakingmatchid_const_ref_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_015248_rage_rlscmatch_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015257_rage_rlscpresenceattribute_rlscpresenceattribute(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015258_rage_rlscpresenceattribute_reset_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015259_rage_rlscpresenceattribute_getvalue_char_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015260_rage_rlscpresencemessagepublish_rlscpresencemessagepublish_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_015261_rage_rlscpresencemessagepublish_export_rage_rsonwriter_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015269_rage_rlinbox_postmessage_int_rage_rlgamerhandle_const_ptr_unsign(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_015271_rage_getinboxmessages_int_unsigned_int_unsigned_int_char_const_p(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_015280_rage_rlyoutube_uploadchunk_rage_rlyoutubeupload_ptr_unsigned_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_015281_rage_rlyoutube_getuploadstatus_rage_rlyoutubeupload_ptr_rage_net(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015282_rage_rlyoutubegetuploadstatustask_configure_rage_rlyoutubeupload(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015284_rage_rlyoutubeshowaccountlinkuitask_configure_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_015303_char_ptr_rage_safecatf_char_3216ul_char_ref_3216ul_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015418_rage_rlugcquerycontenttask_dtorrlugcquerycontenttask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015562_rage_rlleaderboard2readgamersbyscoretask_configure_int_int_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i64, a7: i32, a8: i32, a9: i64, a10: i32, a11: i64, a12: i64, a13: i64) -> i32; }
extern "C" { pub fn f_015563_rage_rlleaderboard2commonreadtask_rlleaderboard2commonreadtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015568_char_ptr_rage_formatf_char_25ul_char_ref_25ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015570_rage_rlleaderboard2commonreadtask_configure_int_int_rage_rlleade(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32, a6: i64, a7: i32, a8: i64, a9: i64, a10: i64) -> i32; }
extern "C" { pub fn f_015626_rage_friendsreadertask_dtorfriendsreadertask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015632_rage_getattributesforgamertask_dtorgetattributesforgamertask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015638_rage_rlfireandforgettask_rage_rlpresencepublishtomanyfriendstask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015646_rage_postmessagetask_dtorpostmessagetask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015673_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_015674_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_015675_rage_rlrostelemetrysubmissiontask_rlrostelemetrysubmissiontask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015676_rage_rlfireandforgettask_rage_rlrostelemetrysubmissiontask_dtorr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015678_rage_rlfireandforgettask_rage_rlrostelemetryimmediatesubmissiont(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015688_rage_rlfireandforgettask_rage_rlscpostuserfeedactivitytask_dtorr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015716_rage_rlclouddeletefiletask_dtorrlclouddeletefiletask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015721_rage_rlfireandforgettask_rage_rlscmatchmakingadvertisetask_dtorr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015723_rage_rlfireandforgettask_rage_rlscmatchmakingupdatetask_dtorrlfi(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015725_rage_rlfireandforgettask_rage_rlscmatchmakingfindtask_dtorrlfire(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015727_rage_rlfireandforgettask_rage_rlscmatchmakingunadvertisetask_dto(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015729_rage_rlfireandforgettask_rage_rlscmatchmakingunadvertisealltask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015731_rage_rlfireandforgettask_rage_rlinboxpostmessagetask_dtorrlfirea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015737_rage_rlinboxpublishtomanyfriendstask_dtorrlinboxpublishtomanyfri(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015742_rage_rlfireandforgettask_rage_rlinboxpublishmessagetask_dtorrlfi(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015746_rage_rlfireandforgettask_rage_rlinboxdeletemessagestask_dtorrlfi(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015751_rage_phmaterialmgr_findcombinedfrictionandelasticity_unsigned_lo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_015754_rage_phmaterialmgr_getdebugcolor_rage_phmaterial_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015773_rage_phmathinertia_clampanginertia_rage_vector3_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015774_rage_phmathinertia_getinverseinertiamatrix_rage_mat33v_const_ref(ph: *mut Phantom, a0: i64, a1: u128, a2: i64); }
extern "C" { pub fn f_015775_rage_phmathinertia_findcylinderanginertia_float_float_float_rage(ph: *mut Phantom, a0: f32, a1: f32, a2: i64); }
extern "C" { pub fn f_015777_rage_phworkerthread_dtorphworkerthread(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015778_rage_phworkerthread_initiate_rage_systaskparameters_const_ref_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_015779_rage_phworkerthread_wait(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015781_rage_btaxissweep3_dtorbtaxissweep3(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015785_rage_btaxissweep3_sortmindownaddnewpairs_rage_btaxissweep3_handl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_015786_rage_btaxissweep3_addoverlappingpairinternal_unsigned_short_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_015788_void_std_2_introsort_std_2_classicalgpolicy_rage_sapedgesortergt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_015789_rage_btaxissweep3_sortedgesin_int_rage_btaxissweep3_edge_ptr_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_015791_void_std_2_introsort_std_2_classicalgpolicy_std_2_less_void_void(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_015802_rage_phpool_rage_phmanifold_release_rage_phmanifold_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015803_rage_phmanifold_removeallcontacts(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015804_rage_phoverlappingpairarray_phoverlappingpairarray_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_015805_rage_phoverlappingpairarray_dtorphoverlappingpairarray(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015806_rage_phoverlappingpairarray_computeislands_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_015807_rage_atinunionfindtemplate_rage_phislandgrain_rage_atinunionfind(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015808_rage_phconstraintbase_phconstraintbase_rage_phconstraintbase_par(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015815_rage_phconstraintbase_requestworldlimitenforcement(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015816_rage_phconstraintbase_allocatemanifold_rage_phinst_ptr_rage_phin(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_015817_rage_phpool_rage_phmanifold_allocate_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_015818_rage_phmanifold_addmanifoldpoint_rage_phcontact_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015820_rage_phmanifold_disablecompositemanifolds(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015821_rage_phconstraintbase_addmanifoldtosolver_rage_phmanifold_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_015822_rage_phsimulator_dumpactivatingpairs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015823_rage_phconstraintcylindrical_phconstraintcylindrical_rage_phcons(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015828_rage_phconstraintcylindrical_updatebreaking(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015830_rage_phconstraintdistance_phconstraintdistance_rage_phconstraint(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015831_rage_phmanifold_getlocaltoworldtransforma_rage_matrix34_ref_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015832_rage_phmanifold_getlocaltoworldtransformb_rage_matrix34_ref_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015836_rage_phmanifold_refreshcontactpoints_int_rage_scalarv_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_015849_rage_phconstraintfixed_phconstraintfixed_rage_phconstraintfixed(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015855_rage_phconstrainthalfspace_phconstrainthalfspace_rage_phconstrai(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015865_rage_phconstrainthinge_phconstrainthinge_rage_phconstrainthinge(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015866_rage_phconstraintrotation_phconstraintrotation_rage_phconstraint(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015867_rage_phconstrainthinge_dtorphconstrainthinge(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015871_rage_phconstraintrotation_virtualupdate_float_vector_4_rage_phin(ph: *mut Phantom, a0: i64, a1: u128, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_015872_rage_phcollider_drawtorque_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015874_rage_phconstraintrotation_enforceworldconstraints(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015878_rage_drawcontacthelper_rage_phcontact_const_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015880_rage_phconstraintrotation_profiledraw(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015881_rage_phconstraintmgr_remove_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_015883_rage_phconstraintmgr_insertandreturntemporarypointer_rage_phcons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_015884_rage_phconstraintmgr_allocateandconstruct_rage_phconstraintbase(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015885_rage_phconstraintmgr_dumpconstraints(ph: *mut Phantom); }
extern "C" { pub fn f_015886_rage_phconstraintspherical_phconstraintspherical_rage_phconstrai(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015887_rage_phconstraintfixedrotation_phconstraintfixedrotation_rage_ph(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015888_rage_phconstraintattachment_phconstraintattachment_rage_phconstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015889_rage_phconstraintprismatic_phconstraintprismatic_rage_phconstrai(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_015893_rage_phmultireaderlocktoken_waitasreader(ph: *mut Phantom); }
extern "C" { pub fn f_015894_rage_phconstraintrotation_setworldaxis_rage_vec3v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_015903_rage_phconstraintattachment_dtorphconstraintattachment(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015909_rage_phconstraintfixedrotation_virtualupdate_float_vector_4_rage(ph: *mut Phantom, a0: i64, a1: u128, a2: i64, a3: i64, a4: i64, a5: i64, a6: i32); }
extern "C" { pub fn f_015918_rage_phconstraintprismatic_dtorphconstraintprismatic(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015932_rage_componentinfo_init_int_rage_mat34v_const_ref_rage_mat34v_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_015933_rage_obbtest_rage_componentinfo_const_ref_rage_componentinfo_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_015934_rage_phmidphase_processcollisionnew_rage_phcollisioninput_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015935_rage_midphaseintersector_intersectbvh_rage_midphaseintersector_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_015936_rage_phcollisioncompositemanifoldmgr_84_postobjectcollisionfinal(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015937_rage_midphaseintersector_processresultcomposite_rage_newcollisio(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_015938_rage_midphaseintersector_getcomponentinfo_rage_midphaseintersect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_015939_rage_midphaseintersector_componentbvhintersect_unsigned_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_015940_rage_phmanifold_enablecompositemanifolds(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015941_rage_phcollisioncompositemanifoldmgr_84_preleafcollisionfindmani(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_015942_rage_phmanifoldresult_addcontactpoint_rage_mat34v_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: u128, a4: u128, a5: u128, a6: i64, a7: i32, a8: i32); }
extern "C" { pub fn f_015947_rage_phmanifold_updateseparatebias_rage_scalarv_const_ref_rage_s(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_015948_rage_phmanifold_exchangethismanifoldonly(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015950_rage_phmanifold_objectsincontact_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015951_rage_phmanifold_removecontactpoint_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_015953_rage_phmanifold_profiledraw(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015955_rage_phbound_getmaterialidfrompartindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_015956_rage_debugcollider_rage_phcollider_ptr_char_const_ptr_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015957_rage_phcollider_settype_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_015959_rage_phcollider_phcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015961_rage_phcollider_init_rage_phinst_ptr_rage_phsleep_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015962_rage_phcollider_setinstanceandreset_rage_phinst_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_015964_rage_phcollider_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015965_rage_phcollider_setcollidermatrixfrominstancerigid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015966_rage_phsleep_reset_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015967_rage_phsleep_wakeup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015968_rage_phcollider_freeze(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015969_rage_phcollider_setreferenceframevelocity_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015970_rage_phcollider_setreferenceframeangularvelocity_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015971_rage_phcollider_zeroforces(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015972_rage_phcollider_profiledraw_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015973_rage_phsleep_computestillness_float_float_float_ref_float_ref_fl(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_015974_rage_phcollider_drawforce_rage_vec3v_const_ref_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_015975_rage_phcollider_rejuvenateimp(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_015976_rage_phcollider_rotatecollider_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015977_rage_phcollider_setinstancematrixfromcollider(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015978_rage_phcollider_dampmotionimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015979_rage_phcollider_updatevelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015984_rage_phcollider_applyimpetus_rage_vec3v_const_ref_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: u128, a7: f32); }
extern "C" { pub fn f_015988_rage_phcollider_setvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_015989_rage_updatecolliderstask_rage_systaskparameters_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015992_rage_phsimulator_processpairlisttask_rage_systaskparameters_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015993_rage_phsimulator_dtorphsimulator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015998_rage_phsimulator_deleteobject_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016000_rage_phsimulator_deactivateobject_int_rage_phinst_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_016001_rage_phsimulator_getinstbehaviorbysearch_int_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_016002_void_std_2_introsort_std_2_classicalgpolicy_rage_compareinstbeha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_016003_rage_bvcache_createcache_rage_phinst_const_ptr_rage_phcollider_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_016004_rage_phsimulator_activateobject_int_rage_phinst_ptr_rage_phcolli(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_016007_rage_phsimulator_applyimpetus_float_vector_4_int_rage_vector3_co(ph: *mut Phantom, a0: i64, a1: u128, a2: i32, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: f32) -> i32; }
extern "C" { pub fn f_016009_rage_phsimulator_applyimpulse_int_rage_vector3_const_ref_rage_ve(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: f32); }
extern "C" { pub fn f_016012_rage_phsimulator_addactiveobject_rage_phinst_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016017_rage_phsimulator_deactivateobject_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016022_rage_phsimulator_getgravityv(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016034_rage_pharchetype_pharchetype(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016036_rage_pharchetype_setbound_rage_phbound_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016038_rage_pharchetype_loaddata_rage_fiasciitokenizer_ref_rage_phbound(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016043_rage_pharchetype_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016046_rage_pharchetypephys_pharchetypephys(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016048_rage_pharchetypephys_loaddata_rage_fiasciitokenizer_ref_rage_phb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016050_rage_pharchetypephys_savedata_rage_fiasciitokenizer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016052_rage_pharchetypephys_setmass_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_016064_rage_pharchetypephys_addwidgets_rage_bkbank_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016068_rage_pharchetype_pharchetype_rage_datresource_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016070_rage_pharchetype_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016071_rage_pharchetypephys_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016076_rage_phasyncshapetestmgr_new_releaseprobe_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016079_rage_phcullshape_localize_rage_phcullshape_ref_rage_mat34v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016080_rage_phcullshape_checksphere_rage_vec3v_const_ref_rage_scalarv_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_016081_rage_phcullshape_draw_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016082_rage_phinst_phinst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016087_rage_phinst_setarchetype_rage_pharchetype_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016108_rage_phshapesweptsphere_drawshape_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016109_rage_phshapesweptsphere_drawintersections_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016110_rage_phshapesweptsphere_testboundprimitive_rage_phshapecapsule_l(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_016111_rage_phshapebase_addintersection_rage_vec3v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_016112_rage_phshapesphere_intersectsphereagainstbound_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_016113_rage_phshapesweptsphere_testpolygon_rage_phshapecapsule_localize(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_016114_rage_phshapebase_addpolygonintersection_rage_vec3v_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_016116_rage_phshapebatch_phshapegroup_rage_phshapeprobe_phshapegroup_in(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016117_rage_phshapeprobe_drawintersections_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016118_rage_phshapeprobe_testboundprimitive_rage_phshapeprobe_localized(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_016119_rage_phshapeprobe_testpolygon_rage_phshapeprobe_localizeddata_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_016120_rage_phshapeedge_drawshape_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016121_rage_phshapeedge_drawintersections_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016122_rage_phshapeedge_testboundprimitive_rage_phshapeprobe_localizedd(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_016123_rage_phshapeedge_testpolygon_rage_phshapeprobe_localizeddata_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_016124_rage_phshapecapsule_drawintersections_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016125_rage_phshapecapsule_testboundprimitive_rage_phshapecapsule_local(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_016126_rage_phshapecapsule_testpolygon_rage_phshapecapsule_localizeddat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_016127_rage_phshapesphere_drawintersections_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016128_rage_phshapesphere_testboundprimitive_rage_phshapesphere_localiz(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_016129_rage_phshapesphere_testpolygon_rage_phshapesphere_localizeddata(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_016130_rage_phshapetest_rage_phshapeprobe_phshapetest(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016133_rage_phshapetest_rage_phshapeprobe_testoneobject_rage_phbound_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_016134_rage_phshapetest_rage_phshapeprobe_testbound_rage_phshapeprobe_l(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_016135_rage_phshapeprobe_populateculler_rage_phshapeprobe_localizeddata(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016136_rage_phshapetest_rage_phshapeprobe_handledmadecompressverticesfo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016138_rage_phshapetest_rage_phshapeprobe_testinlevel_rage_phinst_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_016139_rage_phshapetest_rage_phshapeprobe_testinstanceinlevel_rage_phsh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_016140_rage_phbvhlinesegmentoverlapcallback_testagainstsubtree_short_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_016142_rage_phshapetest_rage_phshapeedge_testbound_rage_phshapeprobe_lo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_016143_rage_phshapetest_rage_phshapeedge_testinlevel_rage_phinst_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_016144_rage_phshapetest_rage_phshapeedge_testinstanceinlevel_rage_phsha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_016146_rage_phshapetest_rage_phshapesphere_testbound_rage_phshapesphere(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_016147_rage_phbvhaabboverlapcallback_setaabb_rage_vector3_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016148_void_rage_phoptimizedbvh_walkstacklesstree_rage_phbvhaabboverlap(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016149_rage_phshapetest_rage_phshapesphere_testinlevel_rage_phinst_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_016150_rage_phshapetest_rage_phshapesphere_testinstanceinlevel_rage_phs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_016154_rage_phshapetest_rage_phshapeobject_testbound_rage_phshapeobject(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_016155_rage_phshapeobject_testboundprimitive_rage_phshapeobject_localiz(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_016156_rage_phshapeobject_testpolygon_rage_phshapeobject_localizeddata(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64, a7: i64) -> i32; }
extern "C" { pub fn f_016157_rage_phshapeobject_populateculler_rage_phshapeobject_localizedda(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016159_rage_phshapetest_rage_phshapeobject_testinstanceinlevel_rage_phs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_016160_rage_phshapeobject_computecullbox_rage_matrix34_const_ref_rage_m(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016161_rage_phshapeobject_convertcontactpointstointersections_rage_phma(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_016162_rage_phshapecomposite_populatecullerfromuserprovidedcullshape_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016163_rage_phbvhcapsuleoverlapcallback_testagainstsubtree_short_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_016164_rage_matrix34_dot_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016167_rage_phshapetest_rage_phshapecapsule_testbound_rage_phshapecapsu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_016168_rage_phshapecapsule_populateculler_rage_phshapecapsule_localized(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016169_rage_phshapetest_rage_phshapecapsule_testinlevel_rage_phinst_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_016170_rage_phshapetest_rage_phshapecapsule_testinstanceinlevel_rage_ph(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_016173_rage_phshapetest_rage_phshapesweptsphere_testbound_rage_phshapec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_016174_rage_phshapetest_rage_phshapesweptsphere_testinlevel_rage_phinst(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_016175_rage_phshapetest_rage_phshapesweptsphere_testinstanceinlevel_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_016177_rage_phshapetest_rage_phshapescalingsweptquad_testbound_rage_phs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_016178_rage_phshapescalingsweptquad_testboundprimitive_rage_phshapescal(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_016179_rage_phshapescalingsweptquad_populateculler_rage_phshapescalings(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016182_rage_phshapebatch_transformlocalizeddata_rage_phshapebatch_local(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016183_rage_phshapetest_rage_phshapebatch_testbound_rage_phshapebatch_l(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_016184_rage_phshapebatch_testboundprimitive_rage_phshapebatch_localized(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_016185_rage_phshapebatch_rejectbound_rage_phshapebatch_localizeddata_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_016186_rage_phshapetest_rage_phshapebatch_testinlevel_rage_phinst_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_016187_rage_phshapetest_rage_phshapebatch_testinstanceinlevel_rage_phsh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_016195_rage_phshapetest_rage_phshapeprobe_initprobe_rage_phsegment_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_016229_rage_pharchetype_dtorpharchetype_27652(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016230_rage_phinst_dtorphinst_27655(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016240_rage_midphaseintersector_processleafnode_rage_midphaseintersecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_016241_rage_midphaseintersector_intersectbvh1_rage_midphaseintersector(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_016242_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_saped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016243_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016244_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016245_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_std_2_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016246_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_compa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016247_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016251_rage_pharticulatedbody_propagateincrementaltolink_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_016252_rage_pharticulatedbody_propagateonceup_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016253_rage_pharticulatedbody_propagateoncedown_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016259_rage_pharticulatedbody_updatepositionsfromvelocities_float_vecto(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016265_rage_pharticulatedbody_clampvelocities_float_vector_4_float_vect(ph: *mut Phantom, a0: i64, a1: u128, a2: u128); }
extern "C" { pub fn f_016266_rage_pharticulatedbody_moveposition_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016268_rage_pharticulatedbody_rotatebodyto_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016272_rage_pharticulatedbody_getimm_helper_int_float_vector_4_float_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: u128, a4: u128, a5: i32, a6: u128); }
extern "C" { pub fn f_016273_rage_pharticulatedbody_findjointlimitdofs_int_ptr_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: f32, a6: i64) -> i32; }
extern "C" { pub fn f_016274_rage_phjoint_responsetounittorqueonaxis_rage_pharticulatedbody_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016275_rage_phjoint_responsetounitforceonaxis_rage_pharticulatedbody_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016280_rage_atarray_rage_phphasespacevector_0_unsigned_short_resizegrow(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016281_rage_atarray_rage_vec4v_0_unsigned_short_resizegrow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016283_rage_phbound_getcomputeangularinertia_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_016286_rage_pharticulatedbody_applygravitytolinks_float_vector_4_float(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: f32, a4: i32); }
extern "C" { pub fn f_016287_rage_pharticulatedbody_calculateinertias(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016288_rage_pharticulatedbodyinertia_inverse_rage_pharticulatedbodyiner(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016289_rage_phjoint1dof_applytorque_rage_pharticulatedbody_ptr_float_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: u128); }
extern "C" { pub fn f_016290_rage_phjoint_applytorque_rage_pharticulatedbody_ptr_float_vector(ph: *mut Phantom, a0: i64, a1: i64, a2: u128, a3: u128); }
extern "C" { pub fn f_016291_rage_pharticulatedbody_restorevelocities_rage_vec3v_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016292_rage_phjoint1dof_matchchildtoparentpositionandvelocity_rage_phar(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016293_rage_phprismaticjoint_matchchildtoparentpositionandvelocity_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016294_rage_pharticulatedbody_joinbackpositionsandvelocities(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016296_rage_pharticulatedcollider_preinit_int_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016300_rage_pharticulatedcollider_rejuvenateimp(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016301_rage_pharticulatedcollider_setcollidermatrixfrominstanceart(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016305_void_std_2_introsort_std_2_classicalgpolicy_rage_lexicographicly(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_016310_rage_pharticulatedcollider_updatecurrentmatrices(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016311_rage_phboundcomposite_calculateboundbox(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016312_rage_phboundcomposite_updatebvh_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016315_rage_pharticulatedcollider_addtoactivearray(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016316_rage_pharticulatedcollider_updatepositionfromvelocityimp_float_v(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016317_rage_phboundcomposite_calculatecompositeextents_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016320_rage_pharticulatedcollider_clearactive(ph: *mut Phantom); }
extern "C" { pub fn f_016322_rage_pharticulatedcollider_setvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016323_rage_pharticulatedcollider_setangvelocityimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016324_rage_pharticulatedcollider_applyimpulseart_float_vector_4_float(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: i32); }
extern "C" { pub fn f_016325_rage_pharticulatedcollider_applyforceimp_float_vector_4_float_ve(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: i32, a4: u128); }
extern "C" { pub fn f_016326_rage_pharticulatedcollider_applyforcecenterofmassimp_float_vecto(ph: *mut Phantom, a0: i64, a1: u128, a2: u128); }
extern "C" { pub fn f_016328_rage_pharticulatedcollider_applytorqueimp_float_vector_4_float_v(ph: *mut Phantom, a0: i64, a1: u128, a2: u128); }
extern "C" { pub fn f_016331_rage_pharticulatedcollider_getinertiamatriximp_rage_mat33v_ref_i(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016332_rage_pharticulatedcollider_getinverseinertiamatrixart_rage_mat33(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016333_rage_pharticulatedcollider_getlocalvelocityart_float_vector_4_in(ph: *mut Phantom, a0: i64, a1: i64, a2: u128, a3: i32); }
extern "C" { pub fn f_016334_rage_pharticulatedcollider_getangvelocityofbodypart_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016335_rage_phboundcomposite_buildprimitivedatafromprimitives_rage_bvhp(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_016336_rage_phoptimizedbvh_buildtree_rage_bvhprimitivedata_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_016337_rage_pharticulatedcollider_dampmotionimp_float_vector_4(ph: *mut Phantom, a0: i64, a1: u128); }
extern "C" { pub fn f_016339_rage_phjointtype_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016342_rage_phjoint3dof_checkhardleanlimitstatus_rage_pharticulatedbody(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016347_rage_phjoint3dof_computemuscletorquefromaccel_rage_pharticulated(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i64); }
extern "C" { pub fn f_016348_rage_phjoint_totalresponsetotorque_rage_pharticulatedbody_ptr_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016359_rage_phprismaticjoint_computevelocity_rage_pharticulatedbody_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_016367_rage_phbound_release_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_016369_rage_phbound_localgetsupportingvertexwithoutmarginnotinlined_flo(ph: *mut Phantom, a0: i64, a1: u128, a2: i64); }
extern "C" { pub fn f_016370_rage_phbound_setcgoffset_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016372_rage_phbound_getmaterialidfromfile_rage_fiasciitokenizer_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016376_void_rage_datresource_place_rage_phoptimizedbvh_ptr_rage_phoptim(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016384_rage_phbound_istypeconvex_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_016385_rage_phbound_istypecomposite_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_016388_rage_phbound_virtualconstructfromptr_rage_datresource_ref_rage_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016389_rage_phboundgeometry_phboundgeometry_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016390_rage_phbound_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016391_rage_phboundbox_phboundbox_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016395_rage_phboundbox_setboxsize_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016400_rage_phboundbvh_phboundbvh(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016401_rage_phboundgeometry_phboundgeometry(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016403_rage_phboundgeometry_dtorphboundgeometry(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016405_rage_phboundbvh_computeprimitivedensitycolor_unsigned_short_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_016407_rage_phboundpolyhedron_draw_rage_mat34v_const_ref_bool_bool_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_016408_rage_phoptimizedbvh_draw_rage_mat34v_const_ref_int_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_016409_rage_phpolygon_isthin_rage_phboundpolyhedron_const_ptr_rage_scal(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_016410_rage_phpolygon_hasbadnormal_rage_phboundpolyhedron_const_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_016411_rage_phpolygon_hasbadneighbors_rage_phboundpolyhedron_ptr_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_016412_rage_phboundpolyhedron_drawinternal_int_rage_vec3v_const_ref_con(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016413_rage_phboundbvh_constructboundfromprimitive_rage_phprimcapsule_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016414_rage_phboundbvh_constructboundfromprimitive_rage_phprimbox_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016415_rage_phboundbvh_constructboundfromprimitive_rage_phprimcylinder(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016416_rage_phboundpolyhedron_computevertexnormals_rage_vec3v_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016417_rage_phboundbvh_drawactivepolygons_rage_mat34v_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016419_rage_phoptimizedbvh_buildfromprimitivedata_rage_bvhprimitivedata(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_016421_rage_phboundgeometry_copy_rage_phbound_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016422_rage_phboundgeometry_init_int_int_int_int_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_016423_rage_phboundpolyhedron_octantmapallocateandcopy_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016427_rage_phboundgeometry_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016428_rage_phoptimizedbvh_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016429_rage_phboundpolyhedron_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016430_rage_phboundbvh_constructboundfromprimitive_rage_phprimsphere_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_016431_rage_phboundcapsule_setcapsulesize_rage_scalarv_const_ref_rage_s(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016436_rage_phboundcapsule_phboundcapsule(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016446_rage_phboundcapsule_segmenttocapsuleintersections_rage_vec3v_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64) -> i32; }
extern "C" { pub fn f_016450_rage_phboundcomposite_parser_register(ph: *mut Phantom); }
extern "C" { pub fn f_016454_rage_phboundcomposite_phboundcomposite(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016455_rage_phboundcomposite_init_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016456_rage_phboundcomposite_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016460_rage_phbound_load_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016461_rage_phbound_load_rage_fiasciitokenizer_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016462_rage_phbounddisc_phbounddisc_float(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016463_rage_phboundcylinder_calculatecylinderextents_rage_scalarv_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016464_rage_phboundcomposite_setbound_int_rage_phbound_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016467_rage_writeheader_110_rage_fiasciitokenizer_ref_rage_phbound_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016493_rage_phboundcylinder_testagainstsegment_rage_vec3v_const_ref_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_016506_rage_phpolygon_declarestruct_rage_dattypestruct_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016512_rage_phboundgeometry_weldvertices_float_int_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016515_rage_phboundgeometry_calculatepolynormals(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016521_rage_phbound_place_void_ptr_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016523_rage_phboundpolyhedron_computeneighbors_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016530_rage_phboundsphere_phboundsphere_float(ph: *mut Phantom, a0: i64, a1: f32) -> i64; }
extern "C" { pub fn f_016537_void_std_2_introsort_std_2_classicalgpolicy_rage_compareleafnode(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_016538_rage_phpolygon_inittriangle_unsigned_short_unsigned_short_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_016539_rage_phpolygon_set_rage_phpolygon_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016548_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_rage_lexic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016549_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_rage_lexic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_016550_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016561_continuousconvexcollision_computetimeofimpact_rage_newcollisioni(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016570_continuousconvexcollision_computetimeofimpactgjk_rage_newcollisi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016573_rage_phconvexintersector_getclosestpoints_discretecollisiondetec(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016574_rage_gjkcachequeryinput_setcachedatabase_bool_rage_gjkcachesyste(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_016575_bool_rage_iterativecast_rage_castpointinput_rage_castpointinput(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_016576_rage_voronoisinglesimplexsolver_computeclosestpointandnormalontr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_016577_rage_voronoisinglesimplexsolver_computeclosestpointandnormalonte(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_016578_bool_rage_iterativecast_rage_castscalingboundinput_rage_castscal(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_016588_rage_phverletcloth_phverletcloth(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016591_rage_phverletcloth_phverletcloth_rage_datresource_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016596_rage_phverletcloth_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016598_rage_phverletcloth_instancefromtemplate_rage_phverletcloth_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016601_rage_phclothdata_swapvertex_int_int_rage_phclothconnectivitydata(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_016603_rage_phverletcloth_initedgedata_float_const_ptr_rage_atarray_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016605_rage_phverletcloth_dynamicpinvertex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016610_rage_phclothdata_init_rage_vec3v_const_ptr_int_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_016611_rage_phverletcloth_load_void_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016612_rage_phverletcloth_save_void_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016624_rage_phclothdata_load_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016625_rage_phclothdata_save_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_016627_rage_phclothverletbehavior_phclothverletbehavior(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016628_rage_phclothverletbehavior_dtorphclothverletbehavior(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_016630_rage_phclothverletbehavior_updateverletbound_rage_phverletcloth(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016661_rage_phwind_getlocalvelocity_rage_vec3v_const_ref_rage_vec3v_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_016724_rage_phcontactmgr_update_rage_phoverlappingpairarray_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_016737_rage_phcontactmgr_markscratchpadused_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016876_rage_phlevelnodetree_rage_phlooseoctreenode_initbroadphase_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016878_rage_phlevelnodetree_rage_phlooseoctreenode_getuserdata_int_cons(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_016879_rage_phlevelnodetree_rage_phlooseoctreenode_addtobroadphase_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_016882_rage_phlevelnodetree_rage_phlooseoctreenode_addobject_rage_phins(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_016883_rage_phlevelnodetree_rage_phlooseoctreenode_deleteobject1_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016885_rage_phlevelnodetree_rage_phlooseoctreenode_commitdelayeddeleteo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_016889_rage_phlevelnodetree_rage_phlooseoctreenode_commitdeferredoctree(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016890_rage_phlevelnodetree_rage_phlooseoctreenode_updateobjectlocation(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_016901_rage_phlevelnodetree_rage_phlooseoctreenode_setlastinstancematri(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_016907_rage_phlevelnodetree_rage_phlooseoctreenode_getfirstculledobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016908_rage_phlevelnodetree_rage_phlooseoctreenode_getnextculledobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_016911_rage_phlevelnodetree_rage_phlooseoctreenode_findandaddoverlappin(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_016912_minkowskipenetrationdepthsolver_computepenetrationseparation_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_016914_rage_phverletcloth_computeclothboundingvolume(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016916_rage_phverletcloth_updatecloth_rage_mat34v_const_ref_rage_vec3v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_016917_rage_phverletcloth_resetcollisioninst(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_016920_rage_clothcontroller_applysimulationtomesh_rage_grmgeometryqb_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_090208_wc_aessetkey(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_090211_wc_aescbcencrypt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090212_wc_aescbcdecrypt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090216_wc_aesinit(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090226_wc_rsapublickeydecode(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090235_wc_encodesignature(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_090254_wc_freedhkey(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090257_wc_dhgeneratekeypair(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_090258_wc_dhagree(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_090261_wc_dhsetkey(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_090317_wc_hmacsetkey(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090319_wc_hmacupdate(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090321_wc_hmacfinal(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090333_wc_freerng(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090334_wc_initrng(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090335_wc_rng_generateblock(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090339_wc_freersakey(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090343_wc_rsassl_verifyinline(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090353_wc_sha256update(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090355_wc_sha256final(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090356_wc_initsha256(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090562_mp_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090563_mp_init_multi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_090598_initssl_method(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090649_wolfssl_ctx_new_ex(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090650_wolfssl_init(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090652_wolfssl_ctx_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090653_wolfssl_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090654_wolfssl_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090656_wolfssl_connect(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090657_wolfssl_write(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090658_wolfssl_read(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090659_wolfssl_shutdown(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090660_wolfssl_get_error(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090661_wolfssl_err_error_string(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090665_wolfssl_ctx_set_group_messages(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090668_wolfssl_ctx_load_verify_buffer(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090669_wolfssl_ctx_set_verify(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_090671_wolfsslv23_client_method(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_090672_wolfssl_check_domain_name(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090689_embedsend(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_090692_wolfssl_ctx_setiosend(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090695_errno_location(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_090705_abort(ph: *mut Phantom); }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090719_isspace(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090722_close(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090741_time(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090755_exp2f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090763_fcntl(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090769_fmodf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090805_log10f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090814_memcmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090815_mktime(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090817_gmtime_r(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090826_powf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090901_qsort_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_090910_srand(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_090911_rand(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090916_select(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_090918_sinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090921_siprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090922_small_sprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090923_sscanf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090926_strcasecmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090927_strcat(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090928_strchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090930_strcmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090931_strcpy(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090938_strftime(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_090939_strlen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090940_strncasecmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090942_strncmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090944_strnlen(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090945_strrchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090947_strstr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090956_strtok(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090960_syscall_ret(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090970_tolower(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090985_vsnprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_091002_emscripten_builtin_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091023_multi3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_091029_void_std_2_sort_std_2_less_unsigned_short_unsigned_short_ref_uns(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_091064_std_2_rs_default_dtor_rs_default(ph: *mut Phantom); }
extern "C" { pub fn f_091065_std_2_rs_default_op(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_091066_std_2_rs_get(ph: *mut Phantom); }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091099_getsockname(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_091100_recv(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_091102_send(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_091104_setsockopt(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_091105_shutdown(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_091106_socket(ph: *mut Phantom, a0: i32) -> i32; }
