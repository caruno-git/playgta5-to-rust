//! Callee declarations for crate gen021.
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
extern "C" { pub fn f_000121_rage_sysipclockmutexcommand_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000122_rage_sysipcunlockmutex_rage_sysipcmutextag_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000128_rage_sysipccreatethread_void_ptr_void_ptr_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_000131_rage_sysipcwaitthreadexit_rage_sysipcthreadid_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000135_rage_sysipcsleep_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_000165_char_ptr_rage_formatf_char_256ul_char_ref_256ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000606_rage_voicechat_getpushtotalk(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000608_rage_voicechat_nativeshutdown(ph: *mut Phantom); }
extern "C" { pub fn f_000644_rage_athashnextsize_unsigned_short(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_000645_rage_athash_const_char_char_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000646_rage_athash64_unsigned_long(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_000651_rage_atstring_append_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_000654_rage_atstring_set_char_const_ptr_int_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_000663_rage_atstring_replace_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000675_rage_functorbase_functorbase_void_const_ptr_void_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
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
extern "C" { pub fn f_000869_rage_bkgroup_addbutton_char_const_ptr_rage_datcallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000875_rage_bkgroup_addtext_char_const_ptr_long_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_000877_rage_bkgroup_addtext_char_const_ptr_char_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_000883_rage_bkgroup_addtoggle_char_const_ptr_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_000891_rage_bkgroup_addtoggle_char_const_ptr_rage_atbitset_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_000895_char_ptr_rage_formatf_char_char_ptr_unsigned_long_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_000896_rage_bkgroup_addslider_char_const_ptr_float_ptr_float_float_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: f32) -> i64; }
extern "C" { pub fn f_000906_rage_bkgroup_addslider_char_const_ptr_unsigned_int_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_000957_char_ptr_rage_formatf_char_1024ul_char_ref_1024ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001040_rage_matrix33_rotateto_rage_vector3_const_ref_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001062_rage_matrix34_inverse_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001105_rage_autoidregistry_autoidregistry(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001108_rage_autoidregistry_unregister_rage_autoiddescriptor_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001127_rage_datgrowbuffer_init_void_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001140_rage_datbitbuffer_readunsigned_void_const_ptr_unsigned_int_ref_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001141_rage_datbitbuffer_writeunsigned_void_ptr_unsigned_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_001143_rage_datbitbuffer_datbitbuffer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001144_rage_datbitbuffer_setreadonlybits_void_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_001145_rage_datbitbuffer_setreadwritebits_void_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001146_rage_datbitbuffer_getreadwritebits(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001147_rage_datbitbuffer_setreadonlybytes_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001148_rage_datbitbuffer_setreadwritebytes_void_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001149_rage_datbitbuffer_getnumbyteswritten_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001151_rage_datbitbuffer_setcursorpos_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001152_rage_datbitbuffer_getnumbytesread_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001155_rage_datbitbuffer_readbits_void_ptr_int_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001156_rage_datbitbuffer_bumpreadcursor_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001158_rage_datbitbuffer_bumpreadwritecursor_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001159_rage_datbitbuffer_writebits_void_const_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001160_rage_datbitbuffer_writebits_rage_datbitbuffer_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001161_rage_datbitbuffer_readuns_unsigned_long_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001163_rage_datbitbuffer_writeuns_unsigned_long_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001166_rage_datbitbuffer_readbool_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001167_rage_datbitbuffer_writebool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001169_rage_datbitbuffer_writebytes_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001170_rage_datbitbuffer_readstr_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001171_rage_datbitbuffer_writestr_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001190_rage_aes_aes_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_001193_rage_aes_encrypt_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001195_rage_aes_decrypt_void_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001204_rage_rsonwriter_rsonwriter(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001205_rage_rsonwriter_init_char_ptr_unsigned_int_rage_rsonformat(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_001206_rage_rsonwriter_rsonwriter_char_ptr_unsigned_int_rage_rsonformat(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001208_rage_rsonwriter_initnull_rage_rsonformat(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_001210_rage_rsonwriter_begin_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001213_rage_rsonwriter_beginarray_char_const_ptr_int_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001214_rage_rsonwriter_end(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001216_rage_rsonwriter_writeint_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001218_rage_rsonwriter_writeint64_char_const_ptr_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001222_rage_rsonwriter_writefloat_char_const_ptr_float_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64) -> i32; }
extern "C" { pub fn f_001224_rage_rsonwriter_writestring_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001226_rage_rsonwriter_writedouble_char_const_ptr_double(ph: *mut Phantom, a0: i64, a1: i64, a2: f64) -> i32; }
extern "C" { pub fn f_001229_rage_rsonwriter_writebool_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001232_rage_rsonreader_rsonreader(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_001233_rage_rsonreader_rsonreader_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_001234_rage_rsonreader_init_char_const_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001237_rage_rsonreader_hasmember_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001238_rage_rsonreader_getmember_char_const_ptr_rage_rsonreader_ptr_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001239_rage_rsonreader_getnextsibling_rage_rsonreader_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001240_rage_rsonreader_getfirstmember_rage_rsonreader_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001242_rage_rsonreader_hasname_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001246_rage_rsonreader_asint_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001247_rage_rsonreader_asint64_long_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001248_rage_rsonreader_asuns_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001251_rage_rsonreader_asbool_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001253_rage_rsonreader_asstring_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001254_bool_rage_rsonreader_asvalue_unsigned_char_unsigned_char_ref_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001255_bool_rage_rsonreader_asvalue_unsigned_long_unsigned_long_ref_con(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001256_bool_rage_rsonreader_asvalue_double_double_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001257_rage_rsonreader_readint_char_const_ptr_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001258_rage_rsonreader_readint64_char_const_ptr_long_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001259_rage_rsonreader_readuns_char_const_ptr_unsigned_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001260_rage_rsonreader_readuns64_char_const_ptr_unsigned_long_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001263_rage_rsonreader_readbool_char_const_ptr_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001264_rage_rsonreader_readstring_char_const_ptr_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_001265_rage_rsonreader_readreader_char_const_ptr_rage_rsonreader_ref_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001266_rage_rsonreader_readstring_char_const_ptr_rage_atstring_ref_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_001267_rage_rsonreader_checkname_char_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_001269_rage_rsonreader_validatejson_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_001514_rage_diagdisplayf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001516_rage_fistream_write_void_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_001523_quitf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001524_displayf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001527_warningf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001532_rage_fistream_close(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_001534_quitf_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001535_rage_diaglogf2_rage_diagchannel_const_ref_rage_diagseverity_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_001544_rage_diagdefaultprintstacktrace(ph: *mut Phantom); }
extern "C" { pub fn f_001552_errorf_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_001556_rage_diagerrorf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001559_rage_diagcontextmessage_diagcontextmessage_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001563_rage_diagcontextmessage_pop(ph: *mut Phantom); }
extern "C" { pub fn f_001568_rage_atmapentry_unsigned_int_wchar_t_ptr_const_ref_rage_atmap_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_001579_rage_fiassetmanager_create_char_const_ptr_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_001874_void_rage_atplaceptrcreator_void_ptr_rage_fideviceinstaller_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_001934_rage_atarray_unsigned_int_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002031_rage_mthrandom_mthrandom_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002131_rage_atnonfinalhashstring_atnonfinalhashstring_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002162_rage_parmanager_loadfromstructure_rage_fistream_ptr_rage_parstru(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002163_rage_parmanager_loadfromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_002173_rage_parmanager_savefromstructure_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_002226_rage_psotype_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002227_rage_psoschemacatalog_findstructureschema_rage_atnamespacedhashv(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002233_rage_psomember_getsubstructure(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002234_rage_psostruct_getmemberbyname_rage_atnamespacedhashvalue_rage_a(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002271_rage_psoloadfromstructure_rage_psostruct_ref_rage_parstructure_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_002278_rage_psofile_dtorpsofile(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002283_rage_psoloadfilefrombuffer_char_ptr_int_char_const_ptr_rage_psol(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_002308_rage_psobuilderstructschema_addmemberpointer_rage_atnamespacedha(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_002310_rage_psobuilderstructschema_finishbuilding(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002311_rage_atarray_rage_psoenumtableentry_0_unsigned_short_grow_unsign(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002312_rage_psobuilderschemacatalog_dtorpsobuilderschemacatalog(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002313_rage_psobuilderinstance_addfixup_int_char_const_ptr_rage_psobuil(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_002315_rage_psobuilderinstancedatacatalog_dtorpsobuilderinstancedatacat(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002318_rage_psobuilder_psobuilder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002319_rage_psobuilder_createstructschema_rage_atnamespacedhashvalue_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002321_rage_psobuilder_createstructinstances_rage_psobuilderstructschem(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002323_rage_psobuilder_finishbuilding(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002325_rage_psobuilder_getfilesize(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002326_rage_psobuilder_savetobuffer_char_ptr_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_002332_rage_psoaddparsableobjectfromstructure_rage_psobuilder_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002436_rage_parcguregisterstructure_rage_parstructure_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002437_rage_parcgustructure_createstructure_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002438_rage_parcgustructure_setfactories_rage_parstructure_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_002439_rage_parcgustructure_setgetstructurecb_rage_parstructure_ptr_rag(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002440_rage_parcgustructure_setbaseclass_rage_parstructure_ptr_rage_par(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002441_rage_parcgustructure_beginaddinginitialdelegates_rage_parstructu(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002442_rage_parcgustructure_adddelegateholder_rage_parstructure_ptr_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_002443_rage_parcgustructure_endaddinginitialdelegates_rage_parstructure(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002444_rage_parcgustructure_buildstructurefromstaticdata_void_ptr_rage(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_002740_rage_atstringhash_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_002744_rage_atdatahash_unsigned_int_const_ptr_unsigned_long_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002764_rage_vformatf_n_char_ptr_unsigned_long_char_const_ptr_void_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_002782_lz4_compressbound(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_002783_lz4_compress(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002784_lz4_decompress_fast(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_002798_rage_sysappcontent_isamericanbuild(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002799_rage_sysappcontent_isjapanesebuild(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002807_rage_systimer_getsystemmstime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_002808_rage_atdelegator1_void_rage_sysserviceevent_ptr_removedelegate_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_002809_rage_systimer_getticks(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_002812_rage_sysnamedpipe_sysnamedpipe(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002838_rage_sysparam_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002851_rage_sysstack_printstacktrace_void_ptr_unsigned_long_char_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002857_rage_systimer_haselapsedintervalms_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_002858_rage_systimer_systimer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002878_rage_atmap_unsigned_int_rage_sysstackcollector_tagentry_rage_atm(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002886_rage_systhreadpool_thread_dtorthread(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002888_rage_systhreadpool_workitem_workitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002889_rage_systhreadpool_workitem_dtorworkitem(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_002890_rage_systhreadpool_workitem_pending_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002891_rage_systhreadpool_workitem_getid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002892_rage_systhreadpool_workitem_active_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002894_rage_systhreadpool_workitem_finished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002895_rage_systhreadpool_workitem_wascanceled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_002898_rage_systhreadpool_dtorsysthreadpool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002899_rage_systhreadpool_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_002900_rage_systhreadpool_cancelwork_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_002904_rage_systhreadpool_queuework_rage_systhreadpool_workitem_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_002945_rage_sysmempoolallocator_safeallocate_unsigned_long_unsigned_lon(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_002950_rage_sysmemsimpleallocator_sysmemsimpleallocator_void_ptr_int_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_002952_rage_sysmemsimpleallocator_dtorsysmemsimpleallocator(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003097_char_ptr_rage_formatf_char_16ul_char_ref_16ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_003159_rage_matrix34_fromquaternion_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003161_rage_quaternion_frommatrix34_rage_matrix34_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003170_rage_quaternion_slerp_float_rage_quaternion_const_ref(ph: *mut Phantom, a0: i64, a1: f32, a2: i64); }
extern "C" { pub fn f_003183_rage_atpoolbase_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003184_rage_atpoolbase_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_003835_rage_fwanimdirector_getmove(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_003876_rage_fwanimmanager_getclipifexistsbysetid_rage_fwmvtemplateid_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_003905_rage_fwanimmanager_findslotfromhashkey_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_003984_rage_fwclipsetrequesthelper_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_003997_rage_fwclipsetmanager_getclipdictionarynameandclipname_rage_fwmv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_004019_rage_fwclipsetrequesthelper_request_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_004037_rage_fwclipsetrequesthelper_dtorfwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004038_rage_fwclipsetrequesthelper_fwclipsetrequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004041_rage_fwclipsetrequesthelper_isloaded_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004042_rage_fwclipsetrequesthelper_isinvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004082_rage_fwfacialclipsetgroupmanager_getfacialclipsetgroup_rage_fwmv(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004102_rage_fwmovenetworkinterface_getlasteventthisframe_rage_fwmvid_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_004208_rage_parstructure_ptr_rage_parcgugetstructurecb_rage_fwmovedumpd(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004280_rage_fwposematcher_fwposematcher(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004399_rage_fwanimdirectorcomponentsyncedscene_setsyncedscenerate_short(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_004603_rage_fwextensionlist_add_rage_fwextension_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004620_rage_fwarchetype_getfragtype_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004631_rage_fwarchetypemanager_getarchetypename_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_004632_rage_fwarchetypemanager_lookupmodelid_rage_fwarchetype_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004634_rage_fwarchetypemanager_getarchetypefromhashkey_unsigned_int_uns(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_004656_rage_fwattachmententityextension_getconstrainthandle_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_004661_rage_fwdynamicentitycomponent_fwdynamicentitycomponent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_004681_rage_fwentity_waitforanyactiveanimupdatetocomplete_bool_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004683_rage_fwentity_getaabb_rage_spdaabb_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004687_rage_fwextensionlist_get_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_004693_rage_fwsceneupdate_isinsceneupdate_rage_fwentity_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004735_rage_fwdecorator_get_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004736_rage_fwdecorator_get_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004737_rage_fwdecorator_get_int_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004738_rage_fwdecorator_get_rage_atnonfinalhashstring_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004739_rage_fwdecorator_get_rage_fwdecorator_time_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004742_rage_fwdecoratorextension_removeallfrom_rage_fwextensiblebase_re(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_004744_rage_fwdecoratorextension_fwdecoratorextension_rage_fwextensible(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_004747_rage_fwdecoratorextension_update_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_004749_rage_fwdecoratorextension_update_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_004750_rage_fwdecoratorextension_update_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_004751_rage_fwdecoratorextension_update_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_004752_rage_fwdecoratorextension_update_rage_atnonfinalhashstring_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_004767_rage_atarray_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_0(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_004785_rage_fwregdref_rage_fwentity_rage_fwrefawarebase_reset_rage_fwen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_004935_rage_dlcmdbase_op_new_unsigned_long_bool(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005461_rage_fwboxstreamer_hasloadedaboutpos_rage_vec3v_const_ref_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_005462_rage_fwboxstreamer_hasloadedwithinaabb_rage_spdaabb_const_ref_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_005730_rage_fwinteriorlocation_makeinvalid(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005732_rage_fwinteriorlocation_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005733_rage_fwinteriorlocation_getroomindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_005888_rage_fwentitylist_celement_ptr_std_2_partial_sort_impl_abi_nqn22(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_005889_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_005890_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_005917_rage_fwrefawarebaseimpl_rage_datbase_dtorfwrefawarebaseimpl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_005931_rage_aitaskmanager_process_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_005932_rage_aitaskmanager_aborttasksattreeindex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_005935_rage_aitaskmanager_aborttasks(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_005949_rage_aitasktree_findtaskbytypeactive_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_005950_rage_aitasktree_findtaskbytypewithpriority_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_006037_rage_strstreamingloadermanager_callkeepalivecallbackifnecessary(ph: *mut Phantom); }
extern "C" { pub fn f_006084_rage_strstreaminginfomanager_requestobject_rage_strindex_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_006088_rage_atarray_rage_atnonfinalhashstring_0_unsigned_short_grow_uns(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_006123_rage_strstreamingloadermanager_callkeepalivecallback(ph: *mut Phantom); }
extern "C" { pub fn f_006167_rage_strstreaminginfomanager_isobjectinimage_rage_strindex(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_006255_rage_strrequest_hasloaded(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_006257_rage_strrequest_clearrequiredflags_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_006518_rage_audsound_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_006602_rage_audexternalstreamsound_initstreamplayer_rage_audreferencedr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_006715_rage_audsound_prepareandplay_rage_audwaveslot_ptr_bool_int_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_006717_rage_audsound_stopandforget_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007027_rage_audmetadatamanager_getstringidfromhash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007036_rage_audmetadatamanager_getobject_verify_unsigned_int_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007078_rage_audsoundset_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007079_rage_audsoundset_find_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007097_rage_audrequestedsettings_setposition_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007098_rage_audrequestedsettings_setvolume_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007099_rage_audrequestedsettings_setpostsubmixvolumeattenuation_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_007103_rage_audrequestedsettings_setlowpassfiltercutoff_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007104_rage_audrequestedsettings_sethighpassfiltercutoff_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007111_rage_audrequestedsettings_setenvironmentalloudness_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007203_rage_audwaveslot_loadbank_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_007212_rage_audwaveslot_getreferencecount_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007396_rage_audentity_audentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007407_rage_audentity_createsound_persistentreference_rage_audmetadatar(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_007411_rage_audentity_addbatchedsoundrequest_rage_audmetadataref_rage_a(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007412_rage_audentity_createandplaysound_unsigned_int_rage_audsoundinit(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_007568_rage_ptfxscript_trigger_rage_atnonfinalhashstring_rage_atnonfina(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64, a6: f32, a7: i32) -> i32; }
extern "C" { pub fn f_007571_rage_ptfxscript_settriggeredcolourtint_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32); }
extern "C" { pub fn f_007572_rage_ptfxscript_settriggeredalphatint_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_007584_rage_ptfxscript_updateevo_int_unsigned_int_float(ph: *mut Phantom, a0: i32, a1: i32, a2: f32); }
extern "C" { pub fn f_007658_rage_fwptrlistsinglelink_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_007754_rage_netinterface_getnumactiveplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007755_rage_netinterface_getallactiveplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007756_rage_netinterface_flushalllogfiles_bool(ph: *mut Phantom); }
extern "C" { pub fn f_007757_rage_netinterface_getsynchronisationtime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007758_rage_netinterface_getnumremoteactiveplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007759_rage_netinterface_getnumphysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007760_rage_netinterface_getnumremotephysicalplayers(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007761_rage_netinterface_getremoteactiveplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007762_rage_netinterface_getallphysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007763_rage_netinterface_getremotephysicalplayers(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_007764_rage_netplayermgrbase_getplayerfromgamerid_rage_rlgamerid_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_007766_rage_netplayermgrbase_getplayerfrompeerid_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_007767_rage_netinterface_getphysicalplayerfromindex_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_007769_rage_netinterface_getlocalphysicalplayerindex(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_007770_rage_netinterface_getnetworkobject_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_007771_rage_netobjectmgrbase_getnetworkobject_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_007776_rage_netplayermgrbase_isinitialized_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007779_rage_netinterface_sendbuffer_rage_netplayer_const_ptr_void_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_007780_rage_netplayermgrbase_sendbuffer_rage_netplayer_const_ptr_void_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_007782_rage_netplayermgrbase_adddelegate_rage_atdelegator1_void_rage_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007784_rage_netplayermgrbase_removedelegate_rage_atdelegator1_void_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007792_rage_netbandwidthstatistics_allocatebandwidthrecorder_char_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007799_rage_netbandwidthstatistics_addbandwidthout_short_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_007802_rage_netplayer_ismyplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007803_rage_networklogutils_writelogevent_rage_netlogginginterface_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007804_rage_networklogutils_writemessageheader_rage_netlogginginterface(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64); }
extern "C" { pub fn f_007805_rage_netarraymanager_getarrayhandler_unsigned_short_rage_netarra(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_007809_rage_netbandwidthstatistics_addbandwidthin_short_unsigned_int_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_007833_rage_netarrayhandlerbase_iselementsyncedwithplayer_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_007848_rage_neteventmgr_checkforspaceinpool_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_007849_rage_neteventmgr_prepareevent_rage_netgameevent_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007855_rage_netplayer_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007882_rage_netbandwidthmgr_getbandwidththrottlingrequired_rage_netplay(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_007893_rage_netbandwidthstatistics_getbandwidthrecorder_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_007940_rage_neteventmgr_geteventnamefromtype_unsigned_short_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007942_rage_neteventmgr_handleevent_rage_netgameevent_ptr_rage_datbitbu(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_007946_rage_neteventmgr_getexistingeventfromeventlist_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_007947_rage_neteventmgr_destroyalleventsoftype_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007949_rage_netgameevent_netgameevent_unsigned_short_bool_bool_rage_net(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_007952_rage_csyncdatabase_serialiseobjectid_unsigned_short_ref_char_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007959_rage_netplayer_isremote_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_007967_rage_netlogsplitter_writedatavalue_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007968_rage_netgameevent_dtornetgameevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_007971_rage_arraydataverifyevent_prepare_rage_datbitbuffer_ref_rage_net(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007972_rage_arraydataverifyevent_handle_rage_datbitbuffer_ref_rage_netp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_007973_rage_arraydataverifyevent_decide_rage_netplayer_const_ref_rage_n(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_007974_rage_arraydataverifyevent_writeeventtologfile_bool_bool_rage_dat(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_007976_rage_netgameevent_delaysending_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_007979_rage_netserialiserutils_getadjustedworldextents_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007980_rage_netserialiserutils_getrealmapcoords_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007981_rage_netserialiserutils_getadjustedmapcoords_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007982_rage_csyncdatabase_serialisebool_bool_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007985_rage_csyncdatabase_serialiseinteger_int_ref_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_007987_rage_csyncdatabase_serialiseunsigned_unsigned_char_ref_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_007988_rage_csyncdatabase_serialiseunsigned_unsigned_short_ref_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_007989_rage_csyncdatabase_serialiseunsigned_unsigned_int_ref_int_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_007990_rage_csyncdatabase_serialiseunsigned_unsigned_long_ref_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_007991_rage_csyncdatabase_serialisepackedfloat_float_ref_float_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_007992_rage_netobjectmgrbase_getlogname_char_ptr_unsigned_long_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_007993_rage_csyncdatabase_serialiseposition_rage_vector3_ref_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_007994_rage_csyncdatabase_serialiseorientation_rage_matrix34_ref_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_007995_rage_matrix34_toeulersxyz_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_007996_rage_csyncdatabase_serialisevector_rage_vector3_ref_float_int_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_007999_rage_csyncdatabase_serialisestring_char_ptr_unsigned_int_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_008000_rage_netobjecttypepredicate_setobjecttype_unsigned_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_008005_rage_getupdatelevelstring_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_008056_rage_networklogutils_writeplayertext_rage_netlogginginterface_re(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_008065_rage_netobject_dtornetobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008067_rage_netobject_init_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008070_rage_netobject_getplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008071_rage_netobject_getpendingplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008074_rage_netobject_ispendingownerchange_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008075_rage_netobject_islocalflagset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008076_rage_netobject_setlocalflag_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_008077_rage_netobject_isglobalflagset_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008079_rage_netobject_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008080_rage_netobject_hasbeencloned_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008081_rage_netobject_hasbeencloned_rage_netplayer_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008082_rage_netobject_setbeencloned_rage_netplayer_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008083_rage_netobject_issyncedwithallplayers_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008084_rage_netobject_iscriticalstatesyncedwithallplayers_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008085_rage_netobject_getclonedstate_unsigned_int_ref_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_008086_rage_netobject_setclonedstate_unsigned_int_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_008087_rage_netobject_setpendingcloning_rage_netplayer_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008088_rage_netobject_ispendingcloning_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008089_rage_netobject_ispendingcloning_rage_netplayer_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008090_rage_netobject_ispendingremoval_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008091_rage_netobject_ispendingremoval_rage_netplayer_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008093_rage_netobject_destroynetblender(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008095_rage_netobject_setupdatelevel_unsigned_char_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_008096_rage_netobject_isinscope_rage_netplayer_const_ref_unsigned_int_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_008097_rage_netobject_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008099_rage_netobject_startsynchronising(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008102_rage_netobject_canpasscontrol_rage_netplayer_const_ref_rage_emig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_008103_rage_netplayer_getrlpeerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008104_rage_netobject_canacceptcontrol_rage_netplayer_const_ref_rage_em(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_008105_rage_netobject_changeowner_rage_netplayer_const_ref_rage_emigrat(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008107_rage_netobject_postmigrate_rage_emigrationtype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008108_rage_netobject_playerhasjoined_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008109_rage_netobject_playerhasleft_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008110_rage_netobject_checkplayerhasauthorityoverobject_rage_netplayer(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008111_rage_netobject_isstarving_unsigned_char_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008113_rage_netobject_getcannotdeletereason_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008114_rage_netobject_canclone_rage_netplayer_const_ref_unsigned_int_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_008116_rage_netobject_candelete_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008122_rage_netplayer_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008124_rage_netobjectidmgr_getnewobjectid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008125_rage_netobjectidmgr_hasfreeobjectidsavailable_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008142_rage_packedreliablesmsg_addcreateack_rage_netplayer_const_ref_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_008144_rage_packedreliablesmsg_addremoveack_rage_netplayer_const_ref_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_008148_rage_netobjectmgrbase_dtornetobjectmgrbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008149_rage_netobjectmgrbase_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008150_rage_netobjectmgrbase_init(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008153_rage_netobjectmgrbase_update_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008155_rage_netsynctree_update_rage_netsynctreetargetobject_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_008160_rage_netobjectmgrmessagehandler_sendpackedreliablemessage_rage_n(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008165_rage_netobjectmgrbase_getnetworkobjectfromplayer_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_008171_rage_netobjectmgrbase_registernetworkobject_rage_netobject_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008173_rage_netobjectmgrbase_unregisternetworkobject_rage_netobject_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_008174_rage_netobjectmgrbase_playerhasjoined_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008175_rage_netobjectmgrbase_playerhasleft_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008176_rage_netobjectmgrbase_checkforunregistrationprocesscomplete_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008178_rage_netobjectmgrbase_cloneobject_rage_netobject_ptr_rage_netpla(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_008180_rage_netobjectmgrbase_removeclone_rage_netobject_ptr_rage_netpla(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_008181_rage_netobjectmgrbase_syncclone_rage_netobject_ptr_rage_netplaye(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_008183_rage_netobjectmgrbase_updateobjectonallplayers_rage_netobject_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008184_rage_netobjectmgrbase_getnumlocalobjects_bool_ptr_rage_netobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008185_rage_netobjectmgrbase_getlocalobjectlists_rage_atdlist_rage_neto(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008186_rage_netobjectmgrbase_getnumremoteobjects_bool_ptr_rage_netobjec(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008187_rage_netobjectmgrbase_gettotalnumobjects_bool_ptr_rage_netobject(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008188_rage_netobjectreassignmgr_getnumobjectsbeingreassigned_bool_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008189_rage_netobjectmgrbase_getallobjects_rage_netobject_ptr_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_008190_rage_netobjectmgrbase_forallobjects_void_ptr_rage_netobject_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008191_rage_netobjectmgrbase_forceremovalofunregisteringobject_rage_net(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008192_rage_netobjectmgrbase_getackcodename_rage_netobjectackcode_const(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_008193_rage_netobjectmgrbase_getunregistrationreason_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008208_rage_netobjectreassignmgr_isreassignmentinprogress_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008210_rage_netobjectreassignmgr_addobjecttoreassignlist_rage_netobject(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_008212_rage_netobjectreassignmgr_getreassignmentrunningtimems_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008214_rage_netsynctree_forcesendofsyncupdatenodes_unsigned_int_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_008217_rage_netblender_setlastsyncmessagetime_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008218_rage_netblenderlininterp_netblenderlininterp_rage_clininterpblen(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_008219_rage_netblenderlininterp_onownerchange_rage_blenderresetflags(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008220_rage_netblenderlininterp_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008221_rage_netblenderlininterp_dologarithmicblend_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008223_rage_netblenderlininterp_getpositionmaxforupdatelevel_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_008225_rage_netblenderlininterp_processendofblendtime_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008226_rage_netblenderlininterp_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008227_rage_netblenderlininterp_gostraighttotarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008228_rage_netblenderlininterp_processpostphysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008229_rage_netblenderlininterp_updateblendermode_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_008230_rage_netblenderlininterp_getlastpositionreceived_unsigned_int_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008231_rage_netblenderlininterp_getlastvelocityreceived_unsigned_int_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008232_rage_netblenderlininterp_updateposition_rage_vector3_const_ref_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008234_rage_netblenderlininterp_updatevelocity_rage_vector3_const_ref_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008235_rage_netblenderlininterp_hasstoppedpredictingposition_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008237_rage_netblenderlininterp_getpredictedfutureposition_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008238_rage_netblenderlininterp_getpredictedvelocity_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_008239_rage_netblenderlininterp_getpositiondelta_bool_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_008242_rage_netplayer_dtornetplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008243_rage_netplayer_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008244_rage_netplayer_isactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008246_rage_netplayer_getrlgamerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008247_rage_netplayer_setnonphysicaldata_rage_nonphysicalplayerdatabase(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008248_rage_netplayer_getnonphysicaldata_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008249_rage_netplayer_hasteam(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008252_rage_netplayer_setvoicechannel_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008254_rage_netplayer_addkickvote_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008255_rage_netplayer_removekickvote_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008259_rage_playerdatamsg_playerdatamsg(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008260_rage_playerdatamsg_reset_unsigned_int_rage_netnattype(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_008262_rage_playerdatamsg_writetologfile_bool_unsigned_short_rage_netpl(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_008266_rage_netplayermgrbase_dtornetplayermgrbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008267_rage_netplayermgrbase_shutdownprivate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008268_rage_netplayermgrbase_init_rage_netconnectionmanager_ptr_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_008270_rage_netplayermgrbase_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008272_rage_netplayermgrbase_addplayer_rage_rlgamerinfo_const_ref_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_008273_rage_netplayermgrbase_removeplayer_rage_rlgamerinfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008274_rage_netplayermgrbase_addtemporaryplayer_rage_rlgamerinfo_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_008277_rage_netplayermgrbase_setphysicalplayerindex_rage_netplayer_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_008278_rage_netplayermgrbase_updateplayerlistsforplayer_rage_netplayer(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008279_rage_netplayermgrbase_addremoteplayer_rage_rlgamerinfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i64; }
extern "C" { pub fn f_008280_rage_netplayermgrbase_addlocalplayer_rage_rlgamerinfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_008281_rage_netplayermgrbase_removeplayer_rage_netplayer_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008285_rage_netsyncnodebase_setnodename_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008286_rage_netsyncparentnode_netsyncparentnode(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008287_rage_netsyncparentnode_dtornetsyncparentnode(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008291_rage_netsyncparentnode_write_unsigned_int_unsigned_int_rage_nets(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i64, a7: i32, a8: i64, a9: i64) -> i32; }
extern "C" { pub fn f_008292_rage_netsyncparentnode_read_unsigned_int_unsigned_int_rage_datbi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_008297_rage_netsyncdatanode_netsyncdatanode(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008298_rage_netsyncdatanode_setnodedependency_rage_netsyncdatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008303_rage_netsyncdatanode_write_unsigned_int_unsigned_int_rage_netsyn(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i64, a7: i32, a8: i64, a9: i64) -> i32; }
extern "C" { pub fn f_008304_rage_netsyncdatanode_read_unsigned_int_unsigned_int_rage_datbitb(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_008310_rage_netsynctree_dtornetsynctree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008312_rage_netsynctree_initialisetree(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008313_rage_netsynctree_shutdowntree(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008314_rage_netsynctree_settargetobject_rage_netsynctreetargetobject_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008315_rage_netsynctree_updatecurrentobjectbatches_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_008318_rage_netsynctree_write_unsigned_int_unsigned_int_rage_netsynctre(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i32, a6: i64, a7: i32, a8: i64) -> i32; }
extern "C" { pub fn f_008319_rage_netsynctree_read_unsigned_int_unsigned_int_rage_datbitbuffe(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_008320_rage_netsynctree_dirtynode_rage_netsynctreetargetobject_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008321_rage_netsynctree_forcesendofnodedata_unsigned_int_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_008322_rage_netsynctree_forcesendofnodedatatoplayer_unsigned_char_unsig(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_008323_rage_netsynctree_canapplynodedata_rage_netsynctreetargetobject_p(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008324_rage_netsynctree_applynodedata_rage_netsynctreetargetobject_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008326_rage_netsynctree_settreename_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008328_rage_netsynctree_pushparent_rage_netsyncparentnode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008329_rage_netsynctree_popparent(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008330_rage_netsynctree_setcurrentparent_rage_netsyncparentnode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008331_rage_netsynctree_addchild_rage_netsyncdatanode_ref_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_008373_rage_netleaderboardwritemgr_dtornetleaderboardwritemgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008376_rage_netleaderboardwritemgr_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_008406_rage_netscgamerdatamgr_isrockstardev_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_008409_rage_netscgamerdatamgr_hasautomuteoverride_const(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_008452_rage_atmapentry_rage_atstring_int_const_ref_rage_atmap_rage_atst(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008606_bool_rage_datbitbuffer_writeint_signed_char_signed_char_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008615_rage_csyncdatawriter_serialiseinteger_long_ref_int_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_008632_rage_csyncdatareader_serialisebitfield_unsigned_int_ref_int_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_008636_rage_csyncdatareader_serialiseinteger_long_ref_int_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_008642_rage_csyncdatareader_serialiseorientation_rage_matrix34_ref_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_008682_rage_fwbasepool_fwbasepool_int_char_const_ptr_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_008683_rage_fwconfigmanager_getsizeofpool_unsigned_int_int_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_008688_rage_fwbasepool_delete_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008689_rage_fwscriptguid_createguid_rage_fwextensiblebase_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008690_rage_fwbasepool_new(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008705_rage_scripthandlernetcomponent_getstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008706_rage_scripthandler_getscriptobject_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008707_rage_scripthandler_registerexistingscriptobject_rage_scripthandl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_008713_rage_scripthandler_getscriptresource_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_008746_unsigned_int_rage_netmessage_readheader_rage_datbitbuffer_unsign(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008764_bool_rage_datbitbuffer_readint_int_int_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_008765_bool_rage_netmessage_writeheader_rage_datexportbuffer_unsigned_i(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_008778_rage_scripthandlernetcomponent_ishostlocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008781_rage_scripthandlernetcomponent_isplayeraparticipant_rage_netplay(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_008840_rage_scriptinterface_isscriptregistered_rage_scriptidbase_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_008842_rage_scriptinterface_gethostofscript_rage_scriptidbase_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_008843_rage_scriptinterface_isplayeraparticipant_rage_scriptidbase_cons(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_008844_rage_scriptinterface_getparticipants_rage_scriptidbase_const_ref(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_008953_rage_fwknownrefholder_fwknownrefholder_void_ptr_ptr_rage_fwknown(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_009209_rage_crskeletondata_convertboneidtoindex_unsigned_short_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_009608_rage_grcviewport_grcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009612_rage_grcviewport_setwindow_int_int_int_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: f32, a6: f32); }
extern "C" { pub fn f_009613_rage_grcviewport_dtorgrcviewport(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_009618_rage_grcviewport_perspective_float_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32); }
extern "C" { pub fn f_009636_rage_grcviewport_setcameramtx_rage_mat34v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_009640_rage_grcviewport_isspherevisible_float_float_float_float_float_p(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: f32, a5: i64) -> i32; }
extern "C" { pub fn f_010159_rage_grcdebugdraw_setdisplaydebugtext_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_010172_rage_grcdebugdraw_text_float_float_rage_color32_char_const_ptr_b(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i64, a4: i32, a5: f32, a6: f32, a7: i32); }
extern "C" { pub fn f_010178_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_010179_rage_grcdebugdraw_line_rage_vec3v_const_ref_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_010186_rage_grcdebugdraw_sphere_rage_vec3v_const_ref_float_rage_color32(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010189_rage_grcdebugdraw_boxoriented_rage_vec3v_const_ref_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_010194_rage_grcdebugdraw_axis_rage_mat34v_const_ref_float_bool_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_010220_rage_grcdebugdraw_adddebugoutput_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_010500_rage_iovalue_isdown_float_rage_iovalue_readoptions_const_ref_con(ph: *mut Phantom, a0: i64, a1: f32, a2: i64) -> i32; }
extern "C" { pub fn f_011740_rage_fraginst_setdisablebreakable_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011741_rage_fraginst_setdisabledamage_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011743_rage_fraginst_isbreakingdisabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011744_rage_fraginst_isdamagedisabled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_011750_rage_fraginst_getchildbroken_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011751_rage_fraginst_getcomponentfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_011760_rage_fraginst_breakoffabovegroup_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_011762_rage_fraginst_breakoffroot(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011763_rage_fraginst_deleteabove_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011764_rage_fraginst_deleteabovegroup_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_011778_rage_fraginst_damageallgroups(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_011779_rage_fraginst_reducegroupdamagehealth_int_float_rage_vec3v_const(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_012021_rage_fragphysicslodgroup_getlodbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_012664_rage_snsession_getgamers_rage_rlgamerinfo_ptr_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_012667_rage_snparty_exists_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012668_rage_snsession_host_int_rage_rlnetworkmode_int_int_rage_rlmatchi(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32, a7: i64, a8: i32, a9: i64) -> i32; }
extern "C" { pub fn f_012669_rage_snsession_join_int_rage_rlsessioninfo_const_ref_rage_rlnetw(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i64, a9: i32, a10: i64) -> i32; }
extern "C" { pub fn f_012672_rage_snsession_isinvitable_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012677_rage_snsession_getsessioninfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_012678_rage_netstatus_setpending(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_012679_rage_snsession_update_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_012680_rage_snsession_ishost_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012681_rage_snparty_ismember_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012682_rage_snsession_ismember_rage_rlgamerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012685_rage_snsession_getpeerendpointid_unsigned_long_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012686_rage_snsession_gethostpeerid_unsigned_long_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012716_rage_snpeer_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012742_rage_snsession_findpeerbyid_unsigned_long(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_012746_rage_snsession_getpeercxn_unsigned_long_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_012779_rage_snsession_getgamerinfo_rage_rlgamerhandle_const_ref_rage_rl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_012817_rage_textchat_ismuted_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_012965_bool_rage_datimportbuffer_seruser_rage_rlgamerhandle_rage_rlgame(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013369_rage_neteventqueue_rage_netevent_unsubscribeinternal_rage_neteve(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013371_rage_voicechat_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013372_rage_voicechat_removetalker_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_013373_rage_voicechat_removealltalkers(ph: *mut Phantom); }
extern "C" { pub fn f_013374_rage_voicechat_setvpdthreshold_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013376_rage_voicechat_addtalker_rage_rlgamerinfo_const_ref_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_013378_rage_voicechat_havetalker_rage_rlgamerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013379_rage_voicechat_addtalker_rage_rlgamerinfo_const_ref_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013380_rage_voicechat_gettalkerindex_rage_rlgamerid_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013381_rage_voicechat_removetalker_rage_rlgamerid_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013382_rage_voicechat_istalking_rage_rlgamerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013383_rage_voicechat_hasheadset_int_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013384_rage_voicechat_gettalkerflags_int_const(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_013385_rage_voicechat_gettalkerflags_rage_rlgamerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013386_rage_voicechat_settalkerflags_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_013387_rage_voicechat_settalkeraudioparam_int_unsigned_int_float(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_013390_rage_netbandwidthrecorder_netbandwidthrecorder(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013456_rage_nettimestep_settime_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_013461_rage_netconnectionmanager_delegate_delegate(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_013464_rage_netconnectionmanager_removechanneldelegate_rage_netconnecti(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_013547_rage_netconnectionmanager_setchannelpolicies_unsigned_int_rage_n(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_013550_rage_netconnectionmanager_send_int_void_const_ptr_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_013561_rage_netconnectionmanager_addchanneldelegate_rage_netconnectionm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_013598_rage_netpeerid_op_rage_netpeerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013602_rage_netpeerid_op_rage_netpeerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_013608_rage_netconnectionmanager_getendpointid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_013620_rage_netipaddress_op_rage_netipaddress_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_013638_bool_rage_datexportbuffer_seruns_unsigned_int_const_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013639_bool_rage_datimportbuffer_seruns_unsigned_int_unsigned_int_ref_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013645_rage_netp2pcrypt_key_dtorkey(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013696_rage_nethttprequest_addrequestheadervalue_char_const_ptr_char_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_013713_rage_nethttprequest_appendcontent_void_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013731_rage_nethttprequest_getresponsecontentlength_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013778_rage_netstatus_forcesucceeded_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_013808_bool_rage_datimportbuffer_seruns_unsigned_short_unsigned_short_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_013811_bool_rage_datexportbuffer_seruns_unsigned_short_const_unsigned_s(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_014049_rage_nettimestep_nettimestep(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014067_rage_nettimesync_gettime_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014068_rage_nettimesync_hasstarted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014150_rage_netrequesthandler_netrequesthandler(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014160_rage_nettransactor_nettransactor(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014246_rage_networkverifyf_rage_diagchannel_const_ref_char_const_ptr_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_014260_rage_rltaskmanager_destroytask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014287_rage_rltaskbase_cancel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014288_rage_rltaskmanager_canceltask_rage_netstatus_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_014292_rage_rlgettaskmanager(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_014294_rage_rlgetposixtime_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014295_rage_rlcreateuuid_unsigned_long_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014303_rage_rlgamerhandle_op_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014315_rage_rlgamerinfo_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014316_rage_rlgamerinfo_getgamerhandle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014318_rage_rlgamerinfo_getname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014319_rage_rlgamerhandle_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014321_rage_rlgamerhandle_getrockstarid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014345_rage_rlfriend_getgamerhandle_rage_rlgamerhandle_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014350_rage_rlfriend_isonline_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014361_rage_rlros_getcredentials_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014362_rage_rlroscredentials_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014366_rage_rlpresence_getname_int_char_ptr_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_014367_rage_rlfriend_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014368_rage_rlgamerhandle_islocal_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014369_rage_rlgamerhandle_tostring_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014370_rage_rlgamerhandle_tostring_char_ptr_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_014374_rage_rlgamerhandle_export_void_ptr_unsigned_int_unsigned_int_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014376_rage_rlgamerhandle_op_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014377_rage_rlgamerhandle_touniqueid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014384_rage_rlpresence_issignedin_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014387_rage_rlgamerinfo_rlgamerinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014390_rage_rlgamerinfo_isremote_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014392_rage_rlgamerinfo_getgamerid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014398_rage_rlmatchingattributes_rlmatchingattributes(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014445_rage_rlpresence_getgamerhandle_int_rage_rlgamerhandle_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_014446_rage_rlroscredentials_getrockstarid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014449_rage_rlpresence_adddelegate_rage_atdelegator1_void_rage_rlpresen(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014450_rage_rlpresence_removedelegate_rage_atdelegator1_void_rage_rlpre(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014459_rage_rltaskbase_configure_rage_netstatus_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014460_rage_rltaskmanager_addparalleltask_rage_rltaskbase_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014462_rage_rlpresence_postmessage_int_rage_rlgamerhandle_const_ptr_uns(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_014486_rage_rlsessioninfo_tostring_char_ptr_unsigned_int_unsigned_int_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_014487_rage_rlsessioninfo_isvalid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014501_rage_rlsessioninfo_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014546_rage_rlsession_isestablished_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014580_rage_rlsessioninfo_rlsessioninfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014600_rage_rlsessioninfo_fromstring_char_const_ptr_unsigned_int_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_014603_rage_rlsessioninfo_isclear_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014604_rage_rlsessioninfo_op_rage_rlsessioninfo_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014605_rage_rlhttptask_rlhttptask_rage_sysmemallocator_ptr_unsigned_cha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_014628_rage_rltaskbase_rltaskbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014629_rage_rltaskbase_isactive_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014632_rage_rltaskbase_dtorrltaskbase_24266(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014633_rage_rltaskbase_start(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014634_rage_rltaskbase_update_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014637_rage_rltaskbase_ispending_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014638_rage_rltaskbase_wascanceled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014641_rage_rltaskmanager_appendserialtask_unsigned_long_rage_rltaskbas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_014655_rage_zlibstream_dtorzlibstream(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014662_rage_rlmetric_dtorrlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014685_rage_rlmetric_rlmetric(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014717_rage_zlibstream_zlibstream(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_014718_rage_zlibstream_begindecode_rage_sysmemallocator_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_014720_rage_zlibstream_decode_void_const_ptr_unsigned_int_void_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i64); }
extern "C" { pub fn f_014722_rage_zlibstream_haserror_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014736_rage_rlhttptask_configure_rage_rlhttptaskconfig_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_014738_rage_rlhttptask_finish_rage_rltaskbase_finishtype_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_014740_rage_rlhttptask_update_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_014836_rage_rlroscredentials_hasprivilege_rage_rlrosprivilegeid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_014859_rage_rlros_getdefaultprivilege_rage_rlrosprivilegeid(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014864_rage_rlros_getserviceaccessinfo_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_014865_rage_rlros_requestserviceaccessinfo_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_014867_rage_rlrosserviceaccesstokenstore_serviceaccessrequest_invalidat(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_014874_rage_rlroscredentials_getplayeraccountid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_014946_rage_rlsocialclub_registercomplaint_int_char_const_ptr_rage_nets(ph: *mut Phantom, a0: i32, a1: i64, a2: i64); }
extern "C" { pub fn f_015091_rage_rlclan_getprimarymembership_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_015134_rage_rlclanmembershipdata_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_015136_rage_rlclanmembershipdata_rlclanmembershipdata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_015891_rage_phconstraintmgr_gettemporarypointer_rage_phconstrainthandle(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_016088_rage_phinst_setinstflag_unsigned_short_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_016286_rage_pharticulatedbody_applygravitytolinks_float_vector_4_float(ph: *mut Phantom, a0: i64, a1: u128, a2: u128, a3: f32, a4: i32); }
extern "C" { pub fn f_016985_rage_scrthread_checkifmonitoredscriptarraysizehaschanged_char_co(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_018880_cgamelogic_hasteleportplayerfinished(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_018921_rage_fwentity_getmodelid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_018984_rage_vector3_isclose_rage_vector3_const_ref_float_const(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i32; }
extern "C" { pub fn f_019289_caitarget_caitarget_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_019290_caitarget_setentity_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019452_ailogging_getdynamicentityisclonestringsafe_cdynamicentity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019453_ailogging_getdynamicentitynamesafe_cdynamicentity_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_019458_ailogging_stacktracelog_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019459_ailogging_getentitylocalitynameandpointer_cdynamicentity_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019462_cailogmanager_init_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_019602_cfacialdatacomponent_setfacialclipsettodefault_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019603_cfacialdatacomponent_setfacialidleanimoverride_cped_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_019606_cfacialdatacomponent_clearfacialidleanimoverride_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019607_cfacialdatacomponent_onfacialidlechanged_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_019634_cmoveped_settasknetwork_rage_fwmovenetworkplayer_ptr_float_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i64); }
extern "C" { pub fn f_021008_rage_atbinarymap_int_unsigned_int_datapair_ptr_std_2_partial_sor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_021009_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_021010_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_021143_audspeechaudioentity_setambientvoicename_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_021183_audspeechaudioentity_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021203_audvehicleaudioentity_smashsiren_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_021352_audspeechaudioentity_playanimalvocalization_unsigned_int_char_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_021354_audspeechaudioentity_say_char_const_ptr_char_const_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i64) -> i32; }
extern "C" { pub fn f_021359_auddamagestats_dtorauddamagestats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021365_audspeechaudioentity_setblockfallingvelocitycheck(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021366_audspeechaudioentity_inflictpain_auddamagestats_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_021395_audpedaudioentity_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021406_audspeechaudioentity_isscriptedspeechplaying_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021520_audplaneaudioentity_onverticalflightmodechanged_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_021534_audplaneaudioentity_triggerpropbreak(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021627_audradiostation_getstationindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_021809_audspeechaudioentity_playscriptedspeech_char_const_ptr_char_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64, a10: i32, a11: i32, a12: i32, a13: i64, a14: i32, a15: i32); }
extern "C" { pub fn f_021815_audscriptaudioentity_lognetworkscriptbankrequest_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_021824_audspeechaudioentity_stopplayingscriptedspeech(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021856_audscriptaudioentity_logscriptbanks(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021858_bool_creplaymgrinternal_recordfx_cpacketsndloadscriptwavebank_cp(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_021867_audscriptaudioentity_playsoundandpopulatescriptsoundslotfromnetw(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64, a6: i64, a7: i32) -> i32; }
extern "C" { pub fn f_021879_audscriptaudioentity_stopsoundfromnetwork_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_021957_audspeechaudioentity_say_char_const_ptr_audspeechinitparams_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_022055_audvehicleaudioentity_triggersideshunt(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022073_audvehicleaudioentity_ishornon(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022081_audvehicleaudioentity_setisincarmodshop_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_022082_audvehicleaudioentity_playvehiclehorn_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_022089_audvehicleaudioentity_setradiostationfromnetwork_unsigned_char_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022094_audvehicleaudioentity_triggerheadlightsmash_unsigned_int_rage_ve(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_022095_audvehicleaudioentity_triggertyrepuncture_ehierarchyid_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_022202_audvoicechataudioentity_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022203_audvoicechataudioentity_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022204_audvoicechataudioentity_removetalker_rage_rlgamerid_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_022205_audvoicechataudioentity_removealltalkers(ph: *mut Phantom); }
extern "C" { pub fn f_022235_cpedinventory_getweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_022391_naenvironmentgroup_allocate_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_022398_naaudioentity_createdeferredsound_unsigned_int_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_022658_cped_isdead_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_022659_naenvironmentgroup_init_naaudioentity_ptr_float_unsigned_int_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_022660_naenvironmentgroup_setsource_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_022809_naenvironmentgroup_forcesourceenvironmentupdate_centity_const_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_023526_cpacketsndloadscriptwavebank_cpacketsndloadscriptwavebank_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_023591_cpackettriggeredscriptptfx_cpackettriggeredscriptptfx_unsigned_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: f32, a7: i32) -> i64; }
extern "C" { pub fn f_023593_cpackettriggeredscriptptfxcolour_cpackettriggeredscriptptfxcolou(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32) -> i64; }
extern "C" { pub fn f_023846_creplaymgr_replayrecordsound_unsigned_int_rage_audsoundinitparam(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_023849_creplaymgr_replayrecordsound_unsigned_int_unsigned_int_rage_auds(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_023877_creplaymgr_recordobject_cobject_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023886_creplaymgr_cancelrecordingwarpedonentity_cphysical_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_023998_rage_fwmodelid_fwmodelid_rage_fwmodelid_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_024163_ctrain_setengine_ctrain_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_024164_ctrain_setlinkedtoforward_ctrain_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_024165_ctrain_setlinkedtobackward_ctrain_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_024266_creplaymgrinternal_isawareofworldforrecording(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024315_creplaymgrproxy_iseditmodeactive(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_024356_cpackedpedprops_op_cpackedpedprops_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_026166_caminterface_isrenderingdirector_cambasedirector_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026199_caminterface_isfadedin(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_026202_caminterface_isspherevisibleingameviewport_rage_vector3_const_re(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_026205_cqueriableinterface_istaskcurrentlyrunning_int_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_026211_worldprobe_cshapetestfixedresults_unsigned_char_32_cshapetestfix(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026261_camcinematicdirector_isrenderinganyinvehiclecinematiccamera_cons(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_026329_worldprobe_cshapetestresults_dtorcshapetestresults(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026714_camgameplaydirector_getfirstpersonpedaimcamera_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_026719_cweaponinfo_const_ptr_cweaponinfomanager_getinfo_cweaponinfo_rag(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_026753_cphysical_isasleep_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_027008_ctrain_getlinkedtobackward_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_027009_ctrain_getlinkedtoforward_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029101_cdebug_setversionnumber(ph: *mut Phantom); }
extern "C" { pub fn f_029377_ctask_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_029433_getorcreategroup_rage_bkbank_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029455_cdebugscene_tunewidgetdata_tunewidgetdata_int_char_const_ptr_cha(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64) -> i64; }
extern "C" { pub fn f_029456_cdebugscene_queuetunewidget_cdebugscene_tunewidgetdata_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_029612_worldprobe_cshapetestdesc_setresultsstructure_worldprobe_cshapet(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029613_worldprobe_cshapetestresults_freeintersections(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_029638_cutscenemanager_getanimatedmodelentityfromentity_centity_const_p(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_029756_bool_creplaymgrinternal_recordfx_cpackettriggeredscriptptfx_cpac(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029866_cdebugscene_focusentities_get_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_029870_cpedintelligence_addtasksecondary_rage_aitask_ptr_int(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029965_rage_fwtransform_getb_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_029967_cphysical_gethealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_030722_ceventgroupped_add_rage_fwevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030723_cevent_affectsped_cped_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_030739_cpedintelligence_addevent_cevent_const_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_030740_geteventscriptaigroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030744_ceventnetwork_spewtotty_impl_char_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_030746_geteventscriptnetworkgroup(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_030770_cshockingeventsmanager_add_ceventshocking_ref_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> i32; }
extern "C" { pub fn f_030772_ceventshocking_dtorceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030814_cpeddamagecalculator_cpeddamagecalculator_centity_const_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_030815_cpeddamagecalculator_applydamageandcomputeresponse_cped_ptr_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32); }
extern "C" { pub fn f_030822_ceventrequesthelp_dtorceventrequesthelp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030823_cpeddamagecalculator_registerkill_cped_const_ptr_centity_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64); }
extern "C" { pub fn f_030828_ceventdamage_ceventdamage_centity_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_030829_ceventdamage_dtorceventdamage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_030915_ceventnetworkentitydamage_isvehicleresponsibleforcollision_cvehi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_030921_ceventnetworktimedexplosion_ceventnetworktimedexplosion_centity(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_030924_ceventnetworkscriptevent_ceventnetworkscriptevent_void_ptr_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_030926_ceventnetworkpedleftbehind_ceventnetworkpedleftbehind_cped_ptr_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_030931_ceventnetworkcashtransactionlog_ceventnetworkcashtransactionlog(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64, a6: i32) -> i64; }
extern "C" { pub fn f_030932_ceventnetworkvehicleundrivable_ceventnetworkvehicleundrivable_ce(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_031055_ceventscriptcommand_ceventscriptcommand_int_rage_aitask_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_031056_ceventscriptcommand_dtorceventscriptcommand(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031069_ceventgivepedtask_ceventgivepedtask_int_rage_aitask_ptr_bool_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_031070_ceventgivepedtask_dtorceventgivepedtask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031125_ceventshouttargetposition_ceventshouttargetposition_cped_ptr_cpe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031169_ceventswitch2nm_ceventswitch2nm_unsigned_int_rage_aitask_ptr_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_031170_ceventswitch2nm_dtorceventswitch2nm(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031218_ceventrespondedtothreat_ceventrespondedtothreat_cped_const_ptr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031219_ceventrespondedtothreat_dtorceventrespondedtothreat(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031230_ceventpedseendeadped_ceventpedseendeadped_cped_ref_cped_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_031235_ceventshocking_ceventshocking(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031242_ceventshocking_set_rage_vec3v_const_ref_centity_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_031378_ceventgunaimedat_ceventgunaimedat_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031380_ceventgunaimedat_dtorceventgunaimedat_53949(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031386_ceventgunaimedat_isaimingintargetseeingrange_cped_const_ref_cped(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_031391_ceventgunshot_dtorceventgunshot_53962(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031404_ceventgunshotwhizzedby_ceventgunshotwhizzedby_centity_ptr_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_031409_ceventgunshotbulletimpact_ceventgunshotbulletimpact_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32) -> i64; }
extern "C" { pub fn f_031415_ceventmeleeaction_ceventmeleeaction_centity_ptr_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_031416_ceventmeleeaction_dtorceventmeleeaction(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031424_ceventfriendlyaimedat_ceventfriendlyaimedat_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_031565_ceventnetworkentitydamage_dtorceventnetworkentitydamage(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031582_ceventnetworkvehicleundrivable_dtorceventnetworkvehicleundrivabl(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031706_ceventpedseendeadped_dtorceventpedseendeadped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_031888_ceventgunshotbulletimpact_dtorceventgunshotbulletimpact(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_035282_char_ptr_rage_formatf_char_50ul_char_ref_50ul_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_035304_cminimap_switchoffwaypoint_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_035346_cminimap_getactivewaypointid(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_035354_cminimap_switchonwaypoint_float_float_unsigned_short_bool_bool_e(ph: *mut Phantom, a0: f32, a1: f32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_035356_cminimap_getactivewaypoint_swaypointstruct_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_035357_cminimap_isactivewaypoint_rage_vector2_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_035827_ccontextmenuhelper_playerhasjoined_unsigned_char(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_036182_cpausemenu_updatevoicesettings(ph: *mut Phantom); }
extern "C" { pub fn f_036364_cprofilesettings_getint_cprofilesettings_profilesettingid(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_036652_cuireticule_registerpedincapacitated_cped_const_ref_cped_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_036673_bool_rage_atmap_rage_atstring_cstoretexturemanager_cstoretexture(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037383_gametransactionsessionmgr_gametransactionsessionmgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_037511_cgamestream_postticker_char_const_ptr_bool_bool_bool_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_037657_cwanted_setwantedlevel_rage_vector3_const_ref_int_int_setwantedl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_037658_cwanted_updatewantedlevel_rage_vector3_const_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_037659_cseatmanager_countpedsinseats_bool_bool_bool_cvehicle_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_037661_cmodelinfo_requestassets_rage_fwmodelid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_037662_cmodelinfo_haveassetsloaded_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037664_time_time_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_037667_date_date_int_date_month_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_037669_datetime_addtime_int_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_037670_datetime_getsecondssince1900_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037677_cclock_resetdebugmodifiedinnetworkgame(ph: *mut Phantom); }
extern "C" { pub fn f_037682_cclock_setdate_int_date_month_int(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_037767_cwanted_setwantedlevelnodrop_rage_vector3_const_ref_int_int_setw(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_037768_cwanted_reportcrimenow_ecrimetype_rage_vector3_const_ref_bool_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32); }
extern "C" { pub fn f_037884_cwanted_reportpolicespottingplayer_cped_ptr_rage_vector3_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_037888_cseatmanager_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_037889_cwanted_update_rage_vector3_const_ref_cvehicle_const_ptr_bool_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_037901_cwanted_copsaresearching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037902_cwanted_gettimesincesearchlastrefocused_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037908_cwanted_gettimesincelastspotted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_037945_cweather_forcetypenow_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_038327_cincident_getnetworkobject_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038329_cincident_setdirty(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038365_cincidentmanager_findfireincidentforfire_cfire_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038367_cincidentmanager_getincidentfromincidentid_unsigned_int_unsigned(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_038382_cpedintelligence_addtaskdefault_rage_aitask_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038490_cikmanager_resetallsolvers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038495_cikmanager_lookat_unsigned_int_centity_const_ptr_int_eanimboneta(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32, a7: i32); }
extern "C" { pub fn f_038497_cikmanager_islooking(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038499_cikmanager_abortlookat_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_038501_cikmanager_getlookatentity_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038502_cikmanager_getlookattargetposition_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038505_cikmanager_request_cikrequestbodylook_const_ref_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_038537_cikrequestbodylook_cikrequestbodylook(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038539_cikrequestbodylook_settargetoffset_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038632_cbasemodelinfo_has2deffects_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_038739_cmodelseatinfo_getentrypointindexforseat_int_centity_const_ptr_s(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_038740_cmodelseatinfo_getboneindexfromseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038742_cmodelseatinfo_getseatfromboneindex_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_038751_cseatmanager_getpedsseatindex_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_038756_cseatmanager_getlastdriver_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_038759_cseatmanager_setlastdriverfromnetwork_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_038976_cvehiclevariationinstance_setmodindex_evehiclemodtype_unsigned_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_038981_cvehiclevariationinstance_togglemod_evehiclemodtype_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_039014_cvehiclemodelinfo_getentrypointinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039211_cvehiclevariationinstance_getkit_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039212_cvehiclevariationinstance_getwheeltype_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_039230_cvehiclevariationinstance_setwheeltype_evehiclewheeltype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039235_cvehiclevariationinstance_setxenonlightcolor_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039572_cdoor_settargetdoorratio_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32); }
extern "C" { pub fn f_039576_cdoor_getdooropenratio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_039578_cdoor_isdoorshut_float_float_const(ph: *mut Phantom, a0: i64, a1: f32) -> i32; }
extern "C" { pub fn f_039591_cdoorsystem_setpendingstateondoor_cdoorsystemdata_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039594_cdoor_scriptdoorcontrol_bool_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_039630_cweapon_sfireparams_dtorsfireparams(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039636_cdoor_isdoorshut_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039642_cdoor_getscriptsearchradius_cbasemodelinfo_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_039643_cdoor_findclosestdoor_cbasemodelinfo_ptr_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: f32) -> i64; }
extern "C" { pub fn f_039644_cobject_getpointertoclosestmapobjectforscript_float_float_float(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_039649_cdoorsystem_adddoor_int_int_rage_vector3_const_ref(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_039652_cdoorsystemdata_setpendingstate_doorscriptstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039654_cdoorsystem_getdoordata_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_039663_cdoorsystemdata_setflagforremoval(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039666_cdoorsystemdata_setholdopen_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_039667_cdoorsystemdata_getdoorstatename_doorscriptstate(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_039688_cobjectpopulation_destroyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039698_cobject_createconstraintforsetdrivetotarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039715_cobject_setweapon_cweapon_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_039718_cobject_gettask_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_039719_cobject_settask_rage_aitask_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_039723_cobject_decouplefrommap(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_039729_cobject_setobjecttargettable_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039730_cobject_canbetargetted_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_039746_cobject_getcsetintfromobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039757_cobjectpopulation_converttodummyobject_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_039762_cobjectpopulation_createobject_cobjectpopulation_createobjectinp(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039765_cobjectpopulation_converttorealobject_cdummyobject_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_039990_cpathserver_updatetrackedobject_rage_cnavmeshtrackedobject_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_040142_cpedgroup_islocallycontrolled_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040143_cpedgroup_setdirty(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040144_rage_netsharedarrayhandler_cpedgroup_cpedgroupsarrayhandler_gete(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_040165_cpedgroupmembership_flush_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_040169_cpedgroupmembership_removemember_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040176_cpedgroupmembership_copynetworkinfo_cpedgroupmembership_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_040177_cpedgroup_isplayergroup_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040178_cpedgroup_cpedgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040179_cpedgroup_dtorcpedgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040181_cpedgroup_getnetarrayhandlerindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040182_cpedgroup_process(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040183_cpedgroup_flush_bool_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040186_cpedgroup_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040194_cpedgroups_getpedsgroup_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040195_cpedgroups_getplayersgroup_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_040207_carea_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_040210_carea_set_rage_vector3_const_ref_rage_vector3_const_ref_float_ce(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32); }
extern "C" { pub fn f_040211_carea_setassphere_rage_vector3_const_ref_float_centity_const_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64); }
extern "C" { pub fn f_040213_carea_getcentre_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040214_carea_getmaxradius_float_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_040220_worldprobe_cshapetesthitpoint_gethitinst(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_040424_rage_fraginstnm_mapragdolllodcomponentcurrenttohigh_int_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_041056_findplayerped(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041064_cpedintelligence_isthreatenedby_cped_const_ref_bool_bool_bool_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_041071_cpedintelligence_getclosestvehicleinrange_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041075_cpedtargetting_cpedtargetting_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041092_cpedperception_computefovvisibility_centity_const_ptr_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_041116_cpedintelligence_flushimmediately_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_041117_cpedintelligence_addtoeventhistory_cevent_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041121_cpedintelligence_buildqueriablestate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041195_cpedintelligence_cleartasks_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041197_cpedintelligence_setdefaultrelationshipgroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041200_cpedintelligence_cleartaskeventresponse(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041201_cpedintelligence_flushevents(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041202_cpedintelligence_setrelationshipgroup_crelationshipgroup_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041203_cpedintelligence_getrelationshipgroup_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041218_cplayerinfo_getphysicalplayerindex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041258_cpedintelligence_clearprimarytask(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041259_ceventscanner_checkfortranquilizerdamage_cped_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041262_cpedgeometryanalyser_isinair_cped_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041287_cpedpopulation_removepedfrompopulationcount_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041290_cplayerinfo_cplayerinfo_cped_ptr_rage_rlgamerinfo_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_041291_cplayerinfo_setplayerped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_041295_cplayerinfo_setupplayergroup(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041296_cpedfactory_destroyped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041298_cpedhelmetcomponent_disablehelmet_bool_rage_vector3_const_ptr_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_041300_cpedresetflags_resetpreaipostinventory(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041371_cpediksettingsinfo_isikflagdisabled_ikmanagersolvertypes_flags_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041377_cpedintelligence_setdecisionmakerid_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041379_cpedtargetting_updatetargetting_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_041385_cpedintelligence_findtaskbytype_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_041387_cpedintelligence_gettaskmeleeactionresult_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_041399_cpedscriptedtaskrecorddata_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041401_cpedintelligence_process_tasks_bool_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_041403_cpedintelligence_setrelationshipgroupdefault_crelationshipgroup(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_041417_cpedintelligence_canforcefullyallowcontrolpass_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041420_cpedintelligence_recalculateclonetasks(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041421_cpedintelligence_updatetaskspecificdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041422_cpedailodmanager_shoulddofullupdate_cped_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041426_cpedintelligence_isbattleaware_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041449_cpedintelligence_getcurrenttintindexforparachute_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_041454_cpedmotiondata_setcurrentmoveblendratio_rage_vector2_ref_float_f(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_041494_cfocusentitymgr_getpoppos(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041521_cpedpopulation_findpedtoreuse_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_041522_cpedpopulation_reusepedfrompedreusepool_cped_ptr_rage_matrix34_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_041523_cpedpopulation_removepedfromreusepool_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_041529_cpedpopulation_addpedtopopulationcount_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041530_cpedpopulation_stopspawningpeds(ph: *mut Phantom); }
extern "C" { pub fn f_041576_cplayerarcadeinformation_setteamandrole_earcadeteam_earcaderole(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_041589_cexplosionmanager_cexplosionargs_dtorcexplosionargs(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041624_cpedscriptedtaskrecorddata_set_cped_ptr_int_ceventscriptcommand(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_041625_cpedscriptedtaskrecord_getvacantslot(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041657_cplayerinfo_multiplayerreset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041665_cplayerinfo_decaystealthnoise_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_041702_findfollowped_cplayerinfo_ptr(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_041705_cplayerinfo_setplayerstate_cplayerinfo_state(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_041708_cplayerinfo_setplayercontrol_bool_bool_float_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i32); }
extern "C" { pub fn f_041727_cplayerinfo_reportstealthnoise_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_041746_reportplayerresetmismatch_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_041820_cpopcycle_iscurrentzoneairport(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_041839_cpedperception_isinseeingrange_centity_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041933_cplayerpedtargeting_getonfoothominglockonstate_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_041996_cqueriableinterface_resettasksequenceinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041997_cqueriableinterface_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_041999_cqueriableinterface_updatecachedinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042002_cqueriableinterface_istaskpresentatpriority_int_unsigned_int_con(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042003_cqueriableinterface_gettaskinfofortasktype_int_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_042004_cqueriableinterface_getstatefortasktype_int_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042006_cqueriableinterface_getnetworksynchronisedsceneid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042008_cqueriableinterface_createemptytaskinfo_unsigned_int_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_042010_cqueriableinterface_logtaskinfo_rage_netlogginginterface_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_042011_cqueriableinterface_addtaskinfofromnetwork_ctaskinfo_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042012_cqueriableinterface_unlinktaskinfofromnetwork_ctaskinfo_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042013_cqueriableinterface_gettasknetsequencefortype_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042014_cqueriableinterface_handleclonetolocaltaskswitch_ctaskfsmclone_r(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042018_cqueriableinterface_istasksentovernetwork_int_bool_ptr(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_042019_cqueriableinterface_cantaskbegiventononlocalpeds_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_042040_crelationshipmanager_findrelationshipgroup_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_042114_cpeddamageblitdecoration_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042131_cpeddamagemanager_addpedscar_cped_ptr_epeddamagezones_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: f32, a6: i64, a7: i32, a8: i32, a9: i32, a10: f32, a11: i32); }
extern "C" { pub fn f_042173_cpeddamagemanager_addpedblood_cped_ptr_epeddamagezones_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: f32, a5: f32, a6: i64, a7: i32, a8: f32, a9: i32, a10: i32, a11: i32) -> i32; }
extern "C" { pub fn f_042179_cpeddamagemanager_addpeddamagepack_cped_ptr_rage_atnonfinalhashs(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32); }
extern "C" { pub fn f_042202_cpeddamagemanager_getblooddataforlocalped_cped_ptr_rage_atarray(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_042203_cpeddamagemanager_getscardataforlocalped_cped_ptr_rage_atarray_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_042390_cpedvariationstream_requeststreampedfilesinternal_cped_ptr_cpedv(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042402_csyncedpedvardata_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042403_csyncedpedvardata_extractfromped_cped_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042404_cpedvariationpack_getlocalcompdata_cped_const_ptr_epedvarcomp_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64); }
extern "C" { pub fn f_042405_csyncedpedvardata_applytoped_cped_ref_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042407_csyncedpedvardata_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042408_cpedheadblenddata_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042409_cpedheadblenddata_cpedheadblenddata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042410_cpedheadblenddata_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042411_cpedheadblenddata_op_cpedheadblenddata_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042412_cpedheadblenddata_dtorcpedheadblenddata(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_042446_cpedpropsmgr_setpedprop_cped_ptr_eanchorpoints_int_int_eanchorpo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_042634_cpedpropsmgr_getpedpropspacked_cdynamicentity_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042636_cpedpropsmgr_setpedpropspacked_cdynamicentity_ptr_cpackedpedprop(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042637_cpackedpedprops_getmodelidx_unsigned_int_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_042638_cpackedpedprops_getanchorid_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_042639_cpedpropsmgr_clearpedprop_cped_ptr_eanchorpoints_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042661_cpackedpedprops_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042662_cpackedpedprops_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042686_peddecorationmanager_addpeddecoration_cped_ptr_rage_atnonfinalha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: f32); }
extern "C" { pub fn f_042693_peddecorationmanager_getdecorationsbitfieldfromlocalped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_042694_peddecorationmanager_processdecorationsbitfieldforremoteped_cped(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_042695_peddecorationmanager_clearcompressedpeddecorations_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042696_peddecorationmanager_addcompressedpeddecoration_cped_ptr_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_042780_cpickupdata_getmodelindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042786_cpickupplacement_setamount_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042788_cpickup_setportable(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042789_cpickup_isalwaysfixed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042793_cpickup_addexplosion_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042795_cpickup_processcollectionresponse_rage_netplayer_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042799_cpickup_setuselightoverrideforglow(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042800_cpickup_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042801_cpickupplacement_setpickuphasbeendestroyed_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042806_cpickup_detachportablepickupfromped_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_042808_cpickup_movetoaccessiblelocation_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042810_cpickup_cancollect_cped_const_ptr_cpickup_ecollectiontype_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_042811_cpickupdata_getreward_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_042812_cpickup_setforcealphaanduseambientscale(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042815_cpickup_dropinwater_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042818_worldprobe_cshapetestmanager_debugsubmittest_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_042821_worldprobe_cshapetestmanager_aborttest_worldprobe_cshapetestresu(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042839_cpickup_collect_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042840_cpickup_getcancollectfailurestring_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_042844_cwatertesthelper_getwaterheightatpositionincludingrivers_rage_ve(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i32) -> i32; }
extern "C" { pub fn f_042848_worldprobe_cshapetestdesc_setexcludeentities_centity_const_ptr_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_042849_cpickupmanager_registerpickupcollection_cpickupplacement_ptr_cpe(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042850_cpickupmanager_issuppressionflagset_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_042851_cpickupplacement_getcustommodelhash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042852_cpickup_setremotependingcollection_cped_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_042854_cpickup_setincludeprojectileflag_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042879_cpickupmanager_removepickupplacement_cpickupplacement_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_042881_cpickupplacement_createpickup_cnetobjpickup_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_042882_cpickupplacement_regenerate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042884_cpickupmanager_registerpickupplacement_unsigned_int_rage_vector3(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_042885_cpickupmanager_createpickup_unsigned_int_rage_matrix34_const_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_042886_cpickupmanager_clearspaceinpickuppool_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_042891_cpickupplacement_getpickuphash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042893_cpickupmanager_getregenerationtime_cpickupplacement_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_042894_cpickupmanager_migrateplacementownership_cpickupplacement_ptr_un(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_042899_cpickupmanager_removeplayerfromallprohibitedcollections_unsigned(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_042922_cpickupplacement_setregenerationtime_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_043085_worldprobe_cshapetestdesc_setexcludeentity_centity_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043138_fraginstnmgta_checkcanactivate_cped_const_ptr_bool_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_043214_rage_fwentity_resetnocollision(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043233_worldprobe_cshapetestbounddesc_setboundfromentity_centity_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043294_cragdollconstraintcomponent_enablehandcuffs_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_043295_cragdollconstraintcomponent_enableboundankles_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_043331_cwatertesthelper_submitasyncriverboundprobe_rage_vector3_ref_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043334_cwatertesthelper_getwaterlevelincludingrivers_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_043335_worldprobe_debugfindgroundzforcoord_char_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32) -> f32; }
extern "C" { pub fn f_043346_worldprobe_cshapetestdesc_setincludeinstance_rage_phinst_const_p(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_043347_worldprobe_cshapetestdesc_setincludeentities_centity_const_ptr_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_043783_cpostscan_addtoalwaysprerenderlist_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043784_cpostscan_removefromalwaysprerenderlist_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_043788_cpostscan_getalphaoverride_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_044122_water_getwaterlevel_rage_vector3_const_ref_float_ptr_bool_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_044123_water_getwaterlevelnowaves_rage_vector3_const_ref_float_ptr_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_044446_cdrawnetworkplayername_ny_cdrawnetworkplayername_ny_rage_vector2(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64) -> i64; }
extern "C" { pub fn f_044447_cdrawnetworkdebugohd_ny_cdrawnetworkdebugohd_ny_rage_vector2_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64) -> i64; }
extern "C" { pub fn f_045786_cped_computedefaultdrivingtask_cped_const_ref_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_045789_cped_ismale_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045796_cped_computedefaulttask_cped_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045797_cped_getcurrentmotiontask_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045798_cped_startprimarymotiontask_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045802_cped_setcharparamsbasedonmanagertype(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045805_cped_deletevehicleentryconfig(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045806_cped_clearwetclothing(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045815_cped_releasecoverpoint(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045834_cped_switchtoanimated_bool_bool_bool_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_045842_cped_forcemotionstatethisframe_cpedmotionstates_emotionstate_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_045844_cped_setdefaultdecisionmaker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045871_cped_canattachtoground_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045872_cped_attachtoground(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045873_cped_getclimbphysical_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045874_cped_isonground_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045881_cped_setragdollstate_eragdollstate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045883_cped_setpedoutofvehicle_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_045885_cped_switchtoragdoll_cevent_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_045886_cped_setmyvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045889_cped_getprimarymotiontask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045891_cped_applyragdollblockingflags_eragdollblockingflagsbitset_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045892_cped_clearragdollblockingflags_eragdollblockingflagsbitset_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045893_cped_getragdollblockingflags(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_045894_cped_setspecialnetworkleave(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045902_cped_updatespatialarraytypeflags(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045903_cped_getiscrouching_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045904_cped_setiscrouching_bool_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_045909_cped_getprimarymotiontaskifexists_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_045911_cped_wantstouseactionmode_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_045913_cped_isfirstpersonshootermodeenabledforplayer_bool_bool_bool_boo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045926_cped_getisdrivingvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_045936_cped_stopvoicedrivenmouthmovement(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045943_cped_setvardefault_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045945_cped_setheadblenddata_cpedheadblenddata_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_045946_cped_setheadoverlay_eheadoverlayslots_unsigned_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_045949_cped_cloneheadblend_cped_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_045950_cped_setheadblendpalettecolor_unsigned_char_unsigned_char_unsign(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_045951_cped_setheadoverlaytintindex_eheadoverlayslots_eramptype_unsigne(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_045952_cped_micromorph_emicromorphtype_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_045953_cped_setheadblendeyecolor_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045954_cped_sethairtintindex_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_045955_cped_setheadblendpalettecolordisabled(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045956_cped_getheadblendpalettecolor_unsigned_char_ref_unsigned_char_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_045961_cped_finalizeheadblend(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_045971_cped_getscenariopoint_cdynamicentity_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_045976_cped_setisstrafing_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_045977_cped_setdesiredheading_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_045982_cped_stopallmotion_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046002_cped_isinfirstpersonvehiclecamera_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_046021_cped_processcontrol_resetvariables(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046023_cped_processcontrol_weaponsandaccessoriespostintelligence_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046024_cped_updatemovementmode_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046026_cped_getvehiclepedentering_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046032_cped_processcontrol_animstateupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046033_cped_playfootsteps(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046037_cped_processcontrol_graphics_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046038_cped_processcontrol_physics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046040_cped_processcontrol_animation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046048_cped_getisfpsswimming_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046054_cped_setusingactionmode_bool_cped_actionmodeenabled_float_bool_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: i32); }
extern "C" { pub fn f_046055_cped_setmovementmodeoverridehash_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046056_cped_canbedeleted_bool_bool_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_046063_cped_cleardamage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046086_cped_resetstanddata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046091_cped_debugattachpedtophysical_char_const_ptr_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i64, a8: i64, a9: f32, a10: f32); }
extern "C" { pub fn f_046092_cped_debugattachpedtoentercar_char_const_ptr_char_const_ptr_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64); }
extern "C" { pub fn f_046099_cped_getragdollcomponentmatrix_rage_matrix34_ref_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_046102_cped_getisenteringvehicle_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046107_cped_setdeathstate_edeathstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046109_cped_setarreststate_earreststate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046123_cped_createvehicleentryconfigifrequired(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046124_csyncedpedvehicleentryscriptconfig_op_csyncedpedvehicleentryscri(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046127_cped_setincustody_bool_cped_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_046128_cped_setisahighprioritytarget_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046151_csimpleblendhelper_getphase_ccliphelper_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_046152_cped_setpedinvehicle_cvehicle_ptr_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_046176_cped_getcontrolfromplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046180_cped_instantaiupdate_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046183_cped_getragdollcomponentfrombonetag_eanimbonetag_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046184_cped_getiksettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046186_cped_resurrect_rage_vector3_const_ref_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32); }
extern "C" { pub fn f_046187_cped_setendurance_float_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046191_cped_setmaxendurance_float_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046192_cped_isdebuginvincible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046201_cped_staticcreatenetworkobject_unsigned_short_unsigned_char_unsi(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_046209_cped_setplayerinfo_cplayerinfo_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046213_cped_isactionmodereasonactive_cped_actionmodeenabled_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046222_cped_setcontrolledbyinfo_ccontrolledbyinfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046231_cped_isfirstpersonshootermodestickwithinstrafeanglethreshold_con(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046305_cdynamicentity_addtosceneupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046325_cgameworld_findfollowplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_046404_cportaltracker_setloadscollisions_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_046408_cinteriorinst_getinteriorforlocation_rage_fwinteriorlocation_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046410_centity_getinteriorlocation_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_046428_cdynamicentity_getismainsceneupdateflagset_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046438_cgameworld_add_centity_ptr_rage_fwinteriorlocation_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_046442_cinteriorproxy_getfromlocation_rage_fwinteriorlocation_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046445_cportaltracker_collectstack(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046463_cdynamicentity_destroyscriptextension(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046467_cgameworld_remove_centity_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_046472_centity_getisaddedtoworld_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046478_centity_getplaceabletracker(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046484_centity_setisvisibleformodule_eisvisiblemodule_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_046490_centity_isvisible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046491_centity_getisonscreen_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046499_centity_getcollider(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_046500_centity_getisstatic_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046509_centity_getiscollisioncompletelydisabled(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_046510_centity_setiscollisioncompletelydisabled(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046514_centity_setmaxspeed_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_046520_centity_trytoexplode_centity_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_046524_centity_getaudioenvironmentgroup_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_046528_centity_setisvisibleforallmodules(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_046531_centity_getisvisibleformodule_eisvisiblemodule_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_046545_centity_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047009_cinteriorproxy_getchecksum(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_047012_cfocusentitymgr_getpos(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047044_cgameworld_forallentitiesintersecting_rage_fwsearchvolume_ptr_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32); }
extern "C" { pub fn f_047054_cphysical_setisinwater_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_047060_cphysical_activatephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047062_cphysical_setfixedphysics_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_047063_cphysical_disablephysicsiffixed(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047073_cphysical_sethealth_float_bool_bool(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047075_cphysical_setweapondamageinfo_centity_ptr_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_047096_cphysical_applyforcecg_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_047112_cphysical_getdebugname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_047118_cphysical_debugattachtophysicalusingphysics_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: i64, a10: i64, a11: f32, a12: i32, a13: f32, a14: f32) -> i32; }
extern "C" { pub fn f_047119_cphysical_debugattachtophysicalusingphysics_char_const_ptr_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i64, a9: f32); }
extern "C" { pub fn f_047126_cphysical_debugprocessallattachments_char_const_ptr_char_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_047374_cgameworld_changeplayerped_cped_ref_cped_ref_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_047717_cinteriorproxy_shouldentityfreezeandretain_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047718_cinteriorproxy_movetoretainlist_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_047725_cinteriorinst_issubwaymlo_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047726_cinteriorinst_isofficemlo_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047754_cinteriorproxy_getfromproxyindex_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_047880_cgameworld_findlocalplayercoors(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_047960_cgameworld_hasentitybeenadded_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_047973_cgameworld_clearexcitingstufffromarea_rage_vector3_const_ref_flo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_047975_cgameworld_clearcarsfromarea_rage_vector3_const_ref_float_bool_b(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32); }
extern "C" { pub fn f_047977_cgameworld_clearpedsfromarea_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047979_cgameworld_clearobjectsfromarea_rage_vector3_const_ref_float_boo(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_047987_cgameworld_clearcopsfromarea_rage_vector3_const_ref_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_047995_cgameworld_changepedmodel_cped_ref_rage_fwmodelid(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_050160_entity_commands_commandsetentityalphaentity_centity_ptr_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_050162_entity_commands_commandgetentityalphaentity_centity_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_050164_entity_commands_commandresetentityalphaentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_050675_bool_creplaymgrinternal_recordfx_cpackettriggeredscriptptfxcolou(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_057317_ped_commands_setrelationshipgroupblipforped_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_060782_ccliphelper_isplayingclip_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_063796_cthescripts_unregisterentity_cphysical_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_063800_cthescripts_getcurrentscriptnameandprogramcounter(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_063808_chiddenobjects_addtohiddenobjectmap_int_rage_spdsphere_ref_int_c(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i64, a5: i32); }
extern "C" { pub fn f_063811_cthescripts_getpoptypename_epoptype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_063900_cscriptentities_clearspaceformissionentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_063903_rage_fweventqueue_16_void_flusheventsoftype_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064143_cgamescripthandlermgr_getremotescriptinfo_cgamescriptid_const_re(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_064183_cgamescripthandlermgr_logcurrentreservations_rage_netlogginginte(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064189_cgamescripthandlernetwork_canacceptmigratingobject_rage_scriptha(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_064190_cgamescripthandlermgr_isplayeraparticipant_cgamescriptid_const_r(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_064202_cgamescripthandlermgr_removeplayerfromremotescript_cgamescriptid(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_064209_cgamescripthandlermgr_addremotescriptinfo_cgamescripthandlermgr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064213_cgamescriptid_getlogname_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064214_cgamescriptid_read_rage_datbitbuffer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064215_cgamescriptid_generatescriptidhash(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064216_cgamescriptid_write_rage_datbitbuffer_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064220_cgamescripthandlermgr_cremotescriptinfo_serialise_rage_csyncdata(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064232_cgamescripthandlermgr_getnumrequiredscriptentities_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_064234_cgamescripthandlermgr_isscriptactive_cgamescriptid_const_ref_boo(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064235_cgamescripthandlermgr_isnetworkhost_cgamescriptid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_064236_cgamescripthandlermgr_removefromreservationcount_cnetobjgame_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_064487_ragesecreportcache_getinstance(ph: *mut Phantom); }
extern "C" { pub fn f_064488_ragesecreportcache_hasbeenreported_ragesecreportcache_ereportcac(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_064489_ragesecreportcache_addtoreported_ragesecreportcache_ereportcache(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_064588_ccustomshadereffecttint_selecttintpalette_unsigned_char_rage_fwe(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064640_ccustomshadereffectvehicle_setlicenseplatetexindex_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064642_ccustomshadereffectvehicle_setlicenseplatetext_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_064646_ccustomshadereffectvehicle_setcustomprimarycolor_rage_color32(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064647_ccustomshadereffectvehicle_setcustomsecondarycolor_rage_color32(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_064652_ccustomshadereffectvehicle_getlicenseplatetext_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_064736_csettingsmanager_csettingsmanager(ph: *mut Phantom); }
extern "C" { pub fn f_064737_csettingsmanager_getsettings(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_065358_ccontrol_getinputsource_rage_inputtype_int_unsigned_char_bool_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_065446_ccontrolmgr_getmainplayercontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065461_ccontrolmgr_getmainfrontendcontrol_bool(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_065615_cfilemgr_openfile_char_const_ptr_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_065889_networkinterface_getnetworktime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_065912_ctaskmovescripted_ctaskmovescripted_rage_fwmvtemplateid_rage_fwm(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64, a5: i64, a6: i64) -> i64; }
extern "C" { pub fn f_065922_ctaskmovescripted_setsignalbool_unsigned_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_065927_ctaskmovescripted_setclonesignalfloat_unsigned_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_065928_ctaskmovescripted_getclonemappedfloatsignal_unsigned_int_ref_flo(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_065950_ctaskscriptedanimation_ctaskscriptedanimation_rage_atnonfinalhas(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: f32, a6: f32, a7: i32, a8: i64, a9: f32, a10: i32, a11: i32, a12: i64, a13: i32, a14: i32) -> i64; }
extern "C" { pub fn f_066242_ctaskmeleeactionresult_processselfdamage_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_066253_worldprobe_cshapetesthitpoint_gethitentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_066299_ctaskdyingdead_ctaskdyingdead_cdeathsourceinfo_const_ptr_int_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_066306_ctaskdyingdead_setforcednaturalmotiontask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_066891_cpedinventory_getisweaponoutofammo_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_067591_csyncedvehicle_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_067880_csimpleblendhelper_getrate_ccliphelper_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_067912_ctaskambientmigratehelper_dtorctaskambientmigratehelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_067983_ctaskmobilephone_ctaskmobilephone_int_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_068030_ctaskwander_keepmovingwhilstwaitingforfirstpath_cped_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068126_ctaskdonothing_ctaskdonothing_int_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_068161_ctasksequencelist_ctasksequencelist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068162_ctasksequencelist_flush(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068163_ctasksequencelist_addtask_rage_aitask_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_068174_ctasksequences_getavailableslot_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_068200_cclonedtasksequenceinfo_getsequenceresourceid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068203_ctaskusesequence_ctaskusesequence_ctasksequencelist_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_068208_ctaskusesequence_gettasksequencelist_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068257_ctasktakeoffhelmet_ctasktakeoffhelmet(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_068318_ctaskmobilephone_clearadditionalsecondaryanimation_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_068319_ctaskmobilephone_getadditionalsecondaryanimclipandfilter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068443_ctaskmotionbase_calcvelchangelimitandclamp_cped_const_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_068454_ctaskmotionbase_setonfootclipset_rage_fwmvtemplateid_rage_fwmvcl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_068455_ctaskmotionbase_setinvehiclecontexthash_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_068456_ctaskmotionbase_clearoverrideweaponclipset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_068467_ctaskmotionbase_getdefaultonfootclipset_bool_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_068657_ctaskhumanlocomotion_isonslopesandstairs_cped_const_ptr_bool_flo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_068669_ctaskmotionped_getstaticstatename_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_068714_ctaskmotionaiming_getcanmove_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068772_ctaskmotionaiming_usinglefttriggeraimmode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_068835_ctaskmotiontennis_playdiveanim_ctaskmotiontennis_edivedirection(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32, a5: i32); }
extern "C" { pub fn f_068836_ctaskmotiontennis_playtennisswinganim_rage_atnonfinalhashstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: f32, a5: i32); }
extern "C" { pub fn f_070399_ctasknmcontrol_ctasknmcontrol_unsigned_int_unsigned_int_rage_ait(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: f32) -> i64; }
extern "C" { pub fn f_070401_ctasknmbehaviour_canuseragdoll_cped_ptr_eragdolltriggertypes_cen(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: f32) -> i32; }
extern "C" { pub fn f_070529_ctasknmhighfall_ctasknmhighfall_unsigned_int_cgrabhelper_const_p(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_070739_ctasknmrelax_ctasknmrelax_unsigned_int_unsigned_int_float_float(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: f32, a4: f32) -> i64; }
extern "C" { pub fn f_070913_ctasksmartflee_ctasksmartflee_caitarget_const_ref_float_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_074161_ctask_ctask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_074163_ctaskclassinfomanager_gettaskname_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_074172_ccliphelper_stopclip_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_075348_cmovenetworkhelper_getboolean_rage_fwmvtemplateid_rage_fwmvboole(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_075353_ccliprequesthelper_requestclipsbyindex_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075379_cambientcliprequesthelper_cambientcliprequesthelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075380_cambientcliprequesthelper_op_cambientcliprequesthelper_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_075382_cambientcliprequesthelper_releaseclips(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075383_cambientcliprequesthelper_requestclips_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_075385_cambientcliprequesthelper_setclipandprop_rage_fwmvtemplateid_rag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075405_rage_vector3_normalize(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_075422_cvehiclefollowroutehelper_cvehiclefollowroutehelper(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075427_cvehiclefollowroutehelper_constructfromnodelist_cvehicle_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_075640_ctasktreeclone_ctasktreeclone_rage_fwentity_ptr_int(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_075644_ctasktreeclone_startlocalclonetask_ctaskfsmclone_ptr_unsigned_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_075645_ctasktreeclone_getactiveclonetask(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_075654_ctasktreeped_ctasktreeped_int_rage_fwentity_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i32) -> i64; }
extern "C" { pub fn f_075697_ctaskexitvehicle_ctaskexitvehicle_cvehicle_ptr_cvehicleenterexit(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: f32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_075738_ctaskcarsettempaction_ctaskcarsettempaction_cvehicle_ptr_int_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_076768_ctaskaimgun_shoulduseaimingmotion_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_076869_ctaskgun_updatetarget_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077089_cstatsmgr_incrementstat_statid_const_ref_float_unsigned_int(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_077117_cstatsmgr_updatestatswhenpedrundown_unsigned_short_cvehicle_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_077150_statid_char_set_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077151_statid_char_gethash_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077152_statid_char_gethash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077153_statid_char_getstatid_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077154_statid_char_getonlinestatid_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_077193_cstatsdatamgr_iskeyvalid_statid_const_ref_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077196_cstatsdatamgr_setisplayerinventory_rage_atinbinarymap_sstatdatap(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077200_cstatsdatamgr_statiterfind_statid_const_ref_rage_atinbinarymap_s(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_077202_cstatsdatamgr_getisbasetype_statid_const_ref_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_077329_statid_set_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_077330_statsinterface_getcharacterindex(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077331_statsinterface_getintstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077332_statsinterface_getcharacterslot(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077333_statsinterface_getkeyname_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077334_statsinterface_iskeyvalid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_077335_statsinterface_getstattype_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077336_statsinterface_getstatdesc_statid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077337_statsinterface_canaccessstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
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
extern "C" { pub fn f_077353_statsinterface_trymultiplayersave_esavetypes_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_077354_statsinterface_getfloatstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_077355_statsinterface_getbooleanstat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077358_statsinterface_getuint32stat_statid_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077377_statsinterface_setmaskedint_statid_const_ref_int_int_int_bool_un(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_077378_statsinterface_getmaskedint_int_int_ref_int_int_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_077394_cstatsutils_isstatstrackingenabled(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_077430_moneyinterface_getvcwalletbalance_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_077431_moneyinterface_getvcbankbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077432_moneyinterface_getevcbalance(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_077456_moneyinterface_applyvirtualcashbalance_int_networkgametransactio(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_077479_ctextfile_get_char_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077533_ctextlayout_ctextlayout(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077535_ctextlayout_dtorctextlayout(ph: *mut Phantom); }
extern "C" { pub fn f_077544_ctextformat_getstringwidth_ctextlayout_const_ref_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_077791_cpathfind_checkpathnodeintegrityforzone_int_const(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_077792_cpathfind_testalllinks_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077801_cvehicleintelligence_getnodelist_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077803_cvehicleintelligence_getactivetask_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_077862_cvehiclenodelist_findlaneswithnodes_int_rage_vector3_const_ptr_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_077863_cvehiclenodelist_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077930_cvehicleailod_getdummymode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077937_cvehicleailodmanager_calcdesiredlodforpoint_rage_vec3v_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_077949_cvehicleintelligence_invalidatecachednodelist(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077950_cvehicleintelligence_invalidatecachedfollowroute(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_077984_cvehicleintelligence_createvehicletaskfornetwork_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_078024_cvehiclenodelist_cvehiclenodelist(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078029_cvehiclenodelist_findlinkswithnodes_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078085_ctaskvehiclemissionbase_setcruisespeed_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078342_ctaskvehiclemissionbase_readtaskdata_rage_datbitbuffer_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078343_ctaskvehiclemissionbase_readtaskmigrationdata_rage_datbitbuffer(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078472_ctaskbringvehicletohalt_ctaskbringvehicletohalt_float_int_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i32) -> i64; }
extern "C" { pub fn f_078784_canchorhelper_anchor_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078785_canchorhelper_cananchorhere_bool_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078826_cvehicle_resetrespotcounter_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078827_cvehicle_setiswrecked(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078850_cvehicledamage_process_float_bool(ph: *mut Phantom, a0: i64, a1: f32, a2: i32); }
extern "C" { pub fn f_078852_cvehicle_getisbeingtowed_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078871_cvehicle_containslocalplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078886_cvehicle_canpedsstandontop_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078888_ccardoor_settargetdooropenratio_float_unsigned_int_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: f32, a2: i32, a3: i64); }
extern "C" { pub fn f_078890_cvehicle_getattachparentvehicle_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_078891_ccardoor_getislatched_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_078893_cheli_getiscargobob_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078898_cvehiclepopulation_updatevehcount_cvehicle_ptr_cvehiclepopulatio(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078903_cvehicle_dummydetachallchildren(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_078917_cvehiclefactory_modifyvehicletopspeed_cvehicle_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078933_cvehicle_isupsidedown_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078934_cvehicle_isonitsside_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078935_cvehicle_getisextraon_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_078936_cvehicle_ishornon_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078937_ccardoor_setshutandlatched_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_078938_cvehicle_processcaralarm_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_078939_cvehicle_switchengineon_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_078941_cvehicledeformation_applydeformations_bool_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_078952_cvehicledamage_breakoffwheel_int_float_float_float_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: i32); }
extern "C" { pub fn f_078955_cvehicle_turnsirenon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_078959_ccomponentreservation_iscomponentinuse(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078974_cvehicledamage_getenginehealth_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_078981_cvehicle_hascontactwheels_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_078990_ccardoor_breakoff_cvehicle_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_078992_ccardoor_breaklatch_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_078999_cvehicle_getvehiclewassunkbyaplayerped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079001_caircraftdamagebase_hassectionbrokenoff_cplane_const_ptr_int_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_079019_cvehicle_updateextras(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079023_cvehicle_getattachedtrailer_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079038_cvehicle_getlastdriver_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079039_cvehicle_isturretseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079040_cvehicle_getspecialflightmoderatio_const(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_079041_cvehicle_rolldownwindow_ehierarchyid_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079057_cvehicletrailerattachpoint_detachtrailer_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079070_cvehicle_getseatanimationinfo_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079071_cvehicle_isentryindexvalid_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079081_cautomobile_setreducedsuspensionforce_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079152_cvehicle_updategadgetsafterteleport_rage_matrix34_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_079169_cboat_getsearchlight(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079226_cvehicle_setliveryid_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079227_cvehicle_setlivery2id_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079228_cvehicle_updatebodycolourremapping_bool(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079236_cthecargenerators_removecarsfromgeneratorsinarea_float_float_flo(ph: *mut Phantom, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32); }
extern "C" { pub fn f_079249_ccomponentreservation_reserveuseofcomponent_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079250_ccomponentreservation_unreserveuseofcomponent_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079252_ccomponentreservationmanager_findcomponentreservation_int_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079253_ccomponentreservationmanager_getseatreservation_int_seataccess_i(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_079255_ccomponentreservationmanager_findcomponentreservationbyarrayinde(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079266_ccardoor_clearflag_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079267_ccardoor_getisintact_cvehicle_const_ptr_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079270_cvehicle_iswindowdown_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079275_cvehicle_haswindowtosmash_ehierarchyid_bool_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_079276_ccardoor_recentlybeingused_unsigned_int(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079277_ccardoor_setdoorallowedtobebrokenoff_bool_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_079335_chandlingdata_gethandlingdata_ehandlingtype(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079349_crotarywingaircraft_getsearchlighton(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079356_crotarywingaircraft_getisjetpack_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079359_cturret_aimatworldpos_rage_vector3_const_ref_cvehicle_ptr_bool_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_079364_crotarywingaircraft_breakoffmainrotor(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079365_crotarywingaircraft_breakoffrearrotor(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079366_crotarywingaircraft_breakofftailboom_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079371_crotarywingaircraft_getistailboombroken_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079399_cheli_updateropes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079404_cvehiclegadgetpickuprope_detachentity_cvehicle_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_079419_cheli_addpickuprope(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079420_cvehiclegadgetpickupropewithmagnet_cvehiclegadgetpickupropewithm(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_079422_cvehicle_getvehiclegadget_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079433_clandinggear_fix_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079441_clandinggear_getpublicstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079459_cblimp_breakblimp(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079472_cplane_getverticalflightmodeavaliable_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079474_clandinggear_setfrompublicstate_cvehicle_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079479_caircraftdamagebase_breakoffsection_cplane_ptr_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_079487_caircraftdamage_getsectionhealthasfraction_caircraftdamage_eairc(ph: *mut Phantom, a0: i64, a1: i32) -> f32; }
extern "C" { pub fn f_079511_ceventshockingplaneflyby_ceventshockingplaneflyby_centity_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_079512_cplane_isplayerdrivingonground_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079513_cvehicle_getisusingscriptautopilot_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079538_cplane_enableindividualpropeller_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079545_cplane_setdesiredverticalflightmoderatio_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079546_cplane_setverticalflightmoderatio_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079547_cplane_isinverticalflightmode_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079580_csubmarinehandling_processbuoyancy_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079599_csubmarinecar_setsubmarinemode_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079607_ctrailer_detachcargovehicles(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079614_ctrailer_debugattachtoparentvehicle_char_const_ptr_char_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32, a6: f32, a7: i32) -> i32; }
extern "C" { pub fn f_079618_ctrailer_istrailerconstraintindestructible_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079619_cvehicle_resetafterteleport(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079620_ctrailer_findcargovehicle_cvehicle_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079621_ctrailer_removecargovehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079632_cvehicle_trytomakefromdummy_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079642_cvehicle_removepedfromseat_cped_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_079643_cvehicle_getpedinseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079655_ctrain_settrackactive_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_079656_ctrain_determinehostapproval_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_079657_ctrain_receivedhostapprovaldata_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_079659_ctrain_findengine_ctrain_const_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_079669_ctrain_setcarriagespeed_ctrain_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_079670_ctrain_validatelinkedlists_ctrain_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079671_ctrain_changetrainstate_ctrain_trainstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079676_ctrain_setnewtrainposition_ctrain_ptr_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079677_ctrain_getnumcarriages_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079688_ctrain_isengine_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079710_cvehicle_getwheelbroken_unsigned_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079730_cvehicle_setsuperdummyvelocity_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079746_cvehicle_doesvehiclehaveaconvertibleroofanimation_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079749_cwheel_gettyreshouldburst_rage_vector3_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079753_cvehicle_createvehicleweaponmgr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079758_cvehicle_getconvertibleroofstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079759_cvehicle_canbedeletedspecial_bool_bool_bool_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32) -> i32; }
extern "C" { pub fn f_079761_cvehiclegadgettowarm_detachvehicle_cvehicle_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_079764_cvehicle_trytomakefromdummyincludingparents_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_079791_cvehicle_isdriver_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079798_cvehicle_updateattachedclonepedsafterwarp(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079803_cvehicle_triggerhydraulicbouncesound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079804_cvehicle_triggerhydraulicactivationsound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079805_cvehicle_triggerhydraulicdeactivationsound(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079814_cvehicle_addpedinseat_cped_ptr_int_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_079827_cvehicle_canpedentercar_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079833_cvehicle_ispedusingdoor_ccardoor_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079837_cvehicle_hasbulletresistantglass_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079841_cvehicle_isonfire_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079842_cvehicle_extinguishcarfire_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_079845_cvehicle_ispersonalvehicle(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079854_cvehicle_containsped_cped_const_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_079861_cwheel_applytyredamage_centity_const_ptr_float_rage_vector3_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_079870_cturret_getturretmatrixworld_rage_matrix34_ref_cvehicle_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_079874_cvehicle_aretyresdamaged_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079875_cwheel_iswheelonfire_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079876_cvehicle_issuspensiondamaged_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079878_cvehicle_processcontrol(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079879_cvehicle_detachtombstone(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079893_cvehicle_getwindownormalforsmash_ehierarchyid_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079907_cvehicle_updatehominglockedontostate_centity_ehominglockonstate(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079910_cvehicle_lowerconvertibleroof_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079911_cvehicle_raiseconvertibleroof_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079913_cvehicle_transformtosubmarine_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079914_cvehicle_transformtocar_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079918_cvehicle_setrespotcounter_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079922_cvehicle_setplayerdrivetask_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_079926_cvehicle_getliveryid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079928_cvehicle_getlivery2id_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_079932_cvehicleweaponmgr_getfirstturretforseat_int_const(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_079945_cvehicle_setexclusivedriverped_cped_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_079955_cvehicle_setdeployoutriggers_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_079980_cvehicledeformation_applydeformationsfromnetwork_unsigned_int_pt(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_079998_cwheel_resetdamage(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080001_cvehicledamage_setbodyhealth_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_080002_cvehicledamage_setpartdamage_ehierarchyid_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_080007_cwheel_applysuspensiondamage_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_080021_cvehiclegadgetwithjointsbase_cvehiclegadgetwithjointsbase_short(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32) -> i64; }
extern "C" { pub fn f_080033_cvehiclepopulation_getlastremovalreason(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_080054_cvehiclepopulation_getdesirednumberambientvehicles_int_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> f32; }
extern "C" { pub fn f_080080_cvehicleweaponmgr_getweaponsforseat_int_rage_atarray_cvehiclewea(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_080255_csearchlight_csearchlight_unsigned_int_ehierarchyid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080256_csearchlight_dtorcsearchlight(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080258_cvehicleweaponmgr_getweaponsandturretsforseat_int_rage_atarray_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_080262_csearchlight_copytargetinfo_csearchlight_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_080294_cvehiclegadgettowarm_attachvehicletotowarm_cvehicle_ptr_cvehicle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_080338_cvehiclegadgetpickupropewithmagnet_dtorcvehiclegadgetpickupropew(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080339_cvehiclegadgetpickuprope_dtorcvehiclegadgetpickuprope_130656(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080392_rage_aitaskmanager_getactivetask_int_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_080416_cvehiclepopulation_findvehicletoreuse_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_080418_cvehiclepopulation_reusevehiclefromvehiclereusepool_cvehicle_ptr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_080419_cvehiclepopulation_removevehiclefromreusepool_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_080433_cvehiclepopulation_calculatevehiclesharemultiplier(ph: *mut Phantom) -> f32; }
extern "C" { pub fn f_080434_cvehiclepopulation_registerinterestingvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080439_cvehiclepopulation_removeexcessnetworkvehicle_cvehicle_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_080440_cvehiclepopulation_stopspawningvehs(ph: *mut Phantom); }
extern "C" { pub fn f_080482_cwheel_setwheelonfire_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_080484_cwheel_settyrewearrate_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_080545_cvehicleentrypointinfo_getwindowid_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_080799_cvehiclegadgetwithjointsbase_dtorcvehiclegadgetwithjointsbase_13(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081361_cdecalmanager_processvehiclebadge_cvehicle_const_ptr_cvehiclebad(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: f32, a7: i32, a8: i32, a9: i32) -> i32; }
extern "C" { pub fn f_081391_cdecalmanager_addweaponimpact_vfxweaponinfo_s_const_ref_vfxcolli(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_081393_cdecalmanager_removevehiclebadge_cvehicle_const_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081394_cdecalmanager_getvehiclebadgerequeststate_cvehicle_const_ptr_uns(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_081445_cfiremanager_startfire_cfiresettings_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_081521_cwatercannons_clearallwatercannonsprayinworld(ph: *mut Phantom); }
extern "C" { pub fn f_081628_clodlightmanager_registerbrokenlightsforentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081629_clodlightmanager_unregisterbrokenlightsforentity_centity_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_081726_cvfxweapon_getinfo_vfxgroup_e_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_081741_cvehicleglassmanager_cancomponentbreak_cphysical_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_081743_cvehicleglassmanager_createcomponent_cphysical_ptr_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_081745_cvehicleglassmanager_smashcollision_centity_ptr_int_float_rage_v(ph: *mut Phantom, a0: i64, a1: i32, a2: f32, a3: i64, a4: i32); }
extern "C" { pub fn f_081751_cvehicleglassmanager_removecomponent_cphysical_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081755_cvehicleglassmanager_getvehicleglasscomponent_cphysical_const_pt(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_081864_cvfxvehicle_triggerptfxlightsmash_cvehicle_ptr_int_rage_vec3v_co(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_081869_cvfxvehicle_triggerptfxpropellerdestroy_cplane_ptr_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_081911_cvfxwheel_triggerptfxwheelburst_cwheel_ptr_cvehicle_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_082028_cexplosionmanager_addexplosion_cexplosionmanager_cexplosionargs(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082065_cpedweaponmanager_getequippedweapon(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082069_cweapon_getmuzzleposition_rage_matrix34_const_ref_rage_vector3_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_082071_cpedweaponmanager_getequippedweapon_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082123_cweapondamage_generatepeddamageevent_centity_ptr_cped_ptr_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64, a5: i64, a6: i64, a7: i32, a8: f32, a9: i64, a10: i32, a11: f32, a12: i32) -> f32; }
extern "C" { pub fn f_082144_cfiringpattern_setfiringpattern_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082162_cweapon_updateshadervariables_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082170_cprojectilemanager_createprojectile_unsigned_int_unsigned_int_ce(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i64, a4: f32, a5: i32, a6: i32, a7: i64, a8: i64, a9: i32, a10: i32, a11: i32) -> i64; }
extern "C" { pub fn f_082175_cweaponinfo_getappropriateweaponclipsetid_cped_const_ptr_fpsstre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_082179_cpedweaponmanager_destroyequippedweaponobject_bool_char_const_pt(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_082202_cweapon_receivefiremessage_centity_ptr_centity_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: i64, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i64, a14: i32) -> i32; }
extern "C" { pub fn f_082215_cpedweaponmanager_dropweapon_unsigned_int_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_082223_cpedinventory_removeweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082253_weaponfactory_create_unsigned_int_int_cdynamicentity_ptr_char_co(ph: *mut Phantom, a0: i32, a1: i32, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_082267_citeminfo_getmodelindex_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_082372_cweaponcomponentvariantmodelinfo_getextracomponentmodel_unsigned(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082554_cpedinventoryloadoutmanager_setdefaultloadout_cped_ptr_cpedmodel(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082568_cpedequippedweapon_createweaponcomponent_unsigned_int_cobject_pt(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_082572_cpedinventory_addweapon_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_082573_cinventoryitemrepository_cweaponitem_105_additem_unsigned_int_cp(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_082577_cpedinventory_addammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082579_cpedinventory_setammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082580_cpedinventory_addweaponandammo_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_082589_cpedinventory_removeallweaponsexcept_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_082590_cpedinventory_getisstreamedin_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082594_cpedweaponmanager_equipweapon_unsigned_int_int_bool_bool_cpedequ(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_082595_cweaponitemrepository_addcomponent_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_082600_cpedweaponmanager_createequippedweaponobject_cpedequippedweapon(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082603_cweaponitem_settintindex_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_082604_cweaponitem_addcomponent_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082605_cweaponitem_removecomponent_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082632_cpedweaponmanager_destroyequippedgadgets(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082634_cpedweaponmanager_processpreai(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082644_cpedweaponmanager_equipbestweapon_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_082671_cweaponitem_getkeyfromhash_unsigned_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082676_cprojectilemanager_removenetsyncprojectile_cprojectile_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082696_cprojectilemanager_getexistingnetsyncprojectile_cnetfxidentifier(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_082698_cprojectilemanager_fireorplacependingprojectile_cped_ptr_int_cpr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_082719_cprojectilemanager_removeprojectile_cnetfxidentifier_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082722_cprojectilemanager_removeallprojectilesinbounds_rage_spdaabb_con(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082723_cprojectilemanager_removeallstickybombsattachedtoentity_centity(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_082726_cprojectilemanager_getprojectilesforowner_centity_const_ptr_rage(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082832_networkutils_getbailreasonasstring_ebailreason(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_082854_cnetwork_getversion(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082855_cnetwork_lansessionsonly(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082859_cnetwork_usenetworkheap_networkmemorycategory(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_082860_cnetwork_stopusingnetworkheap(ph: *mut Phantom); }
extern "C" { pub fn f_082867_networkinterface_getexesize(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_082898_cnetwork_shutdownvoice_bool(ph: *mut Phantom); }
extern "C" { pub fn f_082904_clockoverridedata_stopoverriding(ph: *mut Phantom); }
extern "C" { pub fn f_082905_clockoverridedata_override_int_int_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_082908_networkinterface_ishost(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082909_cnetworktelemetry_appendmetric_rage_rlmetric_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_082912_networkinterface_isinsession(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082916_networkremotecheaterdetector_remotecheaterdetected_cnetgameplaye(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_082924_cnetwork_isclocktimeoverridden(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_082938_networkinterface_islocalplayeronline_bool(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082941_clivemanager_hasrosprivilege_rage_rlrosprivilegeid_int(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_082947_cnetwork_ischeater_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_082949_void_networkdebug_networkdebugmessagehandler_sendmessagetoplayer(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_082950_rage_msgdebugtimepause_export_void_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_082957_networkutils_getscreencoordinatesforohd_rage_vector3_const_ref_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: f32, a5: f32, a6: f32, a7: f32, a8: f32) -> i32; }
extern "C" { pub fn f_082995_cnetwork_initvoice(ph: *mut Phantom); }
extern "C" { pub fn f_082998_networkinterface_broadcastcachedlocalplayerheadblenddata_rage_ne(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_082999_networkinterface_registerobject_rage_netgameobjectwrapper_cdynam(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083000_cnetwork_getclocktimestoredforoverride_int_ref_int_ref_int_ref_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_083012_networkinterface_getlocalgamername(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083013_networkinterface_getactivegamerinfo(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083014_networkinterface_getlocalgamerhandle(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083015_clivemanager_checkonlineprivileges_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083018_networkinterface_bail_ebailreason_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_083019_networkinterface_issessionestablished(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083023_networkinterface_isaspectatingplayer_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083025_networkinterface_getphysicalplayerfromindex_unsigned_char(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083027_networkinterface_isspectatingped_cped_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083028_networkinterface_isinspectatormode(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083034_networkinterface_isfriendlyfireallowed_cped_const_ptr_centity_co(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083041_networkinterface_canregisterobject_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083043_networkinterface_getplayerfocusposition_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_083044_networkinterface_isvisibleorclosetoanyplayer_rage_vector3_const(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_083046_networkinterface_isvisibleorclosetoanyremoteplayer_rage_vector3(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32, a4: i64, a5: i32, a6: i32) -> i32; }
extern "C" { pub fn f_083048_networkutils_isvisibletoplayer_cnetgameplayer_const_ptr_rage_vec(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: f32) -> i32; }
extern "C" { pub fn f_083052_networkinterface_getplayerfromgamerid_rage_rlgamerid_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083056_networkinterface_getplayerfromgamerhandle_rage_rlgamerhandle_con(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083058_networkinterface_gethostplayer(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083060_networkutils_getnetworkplayerposition_cnetgameplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083061_networkinterface_getnetworkobject_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083062_networkinterface_getplayercameraposition_rage_netplayer_const_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083063_networkinterface_unregisterobject_rage_netgameobjectwrapper_cdyn(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083068_networkinterface_areplayersindifferenttutorialsessions_cnetgamep(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083069_networkinterface_setspectatorobjectid_unsigned_short(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083072_networkinterface_setprocesslocalpopulationlimits_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083083_networkinterface_ismigrating(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083087_cnetworkscriptworldstatemgr_revertworldstate_cnetworkworldstated(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083089_cnetworkscriptworldstatemgr_changeworldstate_cnetworkworldstated(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_083090_networkinterface_registerscriptdoor_cdoorsystemdata_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083116_networkutils_getvehiclefromnetworkobject_rage_netobject_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083123_networkinterface_shouldtriggerdamageeventforzerodamage_cphysical(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083125_networkinterface_getlastposreceivedovernetwork_cdynamicentity_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083130_networkinterface_updateheadingblending_cped_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083133_networkinterface_overridemoveblendratiosthisframe_cped_ref_float(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083135_networkinterface_overridenetworkblenderfortime_cphysical_const_p(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083137_networkinterface_isintutorialsession(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083138_networkinterface_isplayerintutorialsession_cnetgameplayer_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083139_networkinterface_istutorialsessionchangepending(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083140_networkinterface_getplayertutorialsessionindex_cnetgameplayer_co(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083141_networkinterface_isentityconcealed_centity_const_ref(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083145_networkinterface_isrelevanttowaypoint_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083147_networkinterface_cansetwaypoint_cped_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083148_networkinterface_getwaypointoverridecolor(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083150_networkinterface_registerheadshotwithnetworktracker_cped_const_p(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083166_cnetworkghostcollisions_getdriverfromvehicle_cvehicle_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083175_networkinterface_locallyhideentityforaframe_centity_const_ref_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_083179_networkinterface_cacheplayerheadblenddata_unsigned_char_cpedhead(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_083180_networkinterface_isplayerusingscopedweapon_rage_netplayer_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_083181_networkutils_getnetworkplayerviewport_cnetgameplayer_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083183_networkinterface_getaveragesynclatency_unsigned_char(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_083190_networkinterface_registernodewithbandwidthrecorder_cprojectbases(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083297_networktextfilter_addreceivedtext_rage_rlgamerhandle_const_ref_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_083300_networktextfilter_init_rage_functor2_rage_rlgamerhandle_const_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i32, a7: i64); }
extern "C" { pub fn f_083301_networktextfilter_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083308_networkclan_releaseemblemreference_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083314_networkclan_requestemblemforclan_long(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083332_clivemanager_checkusercontentprivileges_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083334_networkcrewemblemmgr_getemblemname_emblemdescriptor_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083342_networkclan_getcrewemblemmgr(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_083363_cloudcachelistener_cloudcachelistener(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083364_cloudcachelistener_dtorcloudcachelistener(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083366_cloudcache_addcachefile_unsigned_long_unsigned_int_bool_cachepar(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_083367_cloudcache_opencachefile_unsigned_long_cachepartition(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083369_cloudcache_validatecache_rage_fistream_ptr_int_scacheheader_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_083370_cloudcache_getcachedata_int_void_ptr_unsigned_int(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_083493_ctunables_access_unsigned_int_bool_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083494_ctunables_access_unsigned_int_int_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_083496_ctunables_tryaccess_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083497_ctunables_tryaccess_unsigned_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083498_ctunables_tryaccess_unsigned_int_float(ph: *mut Phantom, a0: i64, a1: i32, a2: f32) -> f32; }
extern "C" { pub fn f_083668_networkdebug_getplayerfromupdaterateindex_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083669_networkdebug_addotherclonecreate(ph: *mut Phantom); }
extern "C" { pub fn f_083670_networkdebug_addothercloneremove(ph: *mut Phantom); }
extern "C" { pub fn f_083671_networkdebug_addcloneremoveduration_float(ph: *mut Phantom, a0: f32); }
extern "C" { pub fn f_083672_networkdebug_addpredictionpop_unsigned_short_unsigned_short_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_083674_networkutils_getnetworkblenderoverriddenreason_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083675_networkutils_getcanblenderrorstring_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083680_networkutils_getpedfromnetworkobject_rage_netobject_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083681_networkutils_getcobjectfromnetworkobject_rage_netobject_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083683_networkdebug_candebugbreakforfocus_edebugbreaktype_rage_netobjec(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_083770_networkcolours_getnetworkcolour_networkcolours_networkcolour(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083772_networkcolours_getteamcolour_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083773_networkcolours_getcustomteamcolour_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083774_cnetworkdamagetracker_cnetworkdamagetracker_rage_netobject_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083775_cnetworkdamagetracker_getdamagedealt_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> f32; }
extern "C" { pub fn f_083776_cnetworkdamagetracker_resetall(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083791_cnetworkkilltracker_registerkill_unsigned_int_unsigned_int_bool(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_083812_networkhasherutil_pushnewhashrequest_unsigned_char_rage_rlgameri(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32); }
extern "C" { pub fn f_083813_networkhasherutil_pushnewhashresult_unsigned_char_rage_rlgamerid(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_083820_cnetfxidentifier_isclone_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083823_cnetworkghostcollisions_cghostcollision_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083825_cnetworkghostcollisions_islocalplayeringhostcollision_cphysical(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083827_cnetworkghostcollisions_removeghostcollision_int_bool(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_083830_serialiseglobalflags_rage_csyncdatabase_ref_unsigned_int_ref_cha(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083831_serialiseanimation_rage_csyncdatabase_ref_rage_fwmvtemplateid_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_083834_serialisemodelhash_rage_csyncdatabase_ref_unsigned_int_ref_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_083835_serialisevelocity_rage_csyncdatabase_ref_int_ref_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64); }
extern "C" { pub fn f_083836_serialisetasktype_rage_csyncdatabase_ref_unsigned_int_ref_char_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_083837_serialisevehicletasktype_rage_csyncdatabase_ref_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_083839_serialisepedpoptype_rage_csyncdatabase_ref_unsigned_int_ref_char(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083840_serialiseobjectcreatedby_rage_csyncdatabase_ref_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083841_serialisevehiclepoptype_rage_csyncdatabase_ref_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083842_serialisemotiongroup_rage_csyncdatabase_ref_rage_fwmvtemplateid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_083843_serialiserelationshipgroup_rage_csyncdatabase_ref_unsigned_int_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_083844_logrelationshipgroup_rage_netlogginginterface_ref_unsigned_int_c(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_083845_serialisedecisionmaker_rage_csyncdatabase_ref_unsigned_int_ref_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083847_serialisedoorsystemstate_rage_csyncdatabase_ref_unsigned_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_083848_serialisemigrationtype_rage_csyncdatabase_ref_rage_emigrationtyp(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083849_serialisehash_rage_csyncdatabase_ref_unsigned_int_ref_char_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_083858_cnetworkstreamingrequestmanager_requestmodel_rage_fwmodelid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083863_cnetworksynchronisedscenes_scenedescription_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083864_cnetworksynchronisedscenes_stoplocalsynchronisedscene_cnetworksy(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_083868_cnetworksynchronisedscenes_update(ph: *mut Phantom); }
extern "C" { pub fn f_083871_cnetworksynchronisedscenes_issceneactive_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083872_cnetworksynchronisedscenes_getscenedescriptionfromnetworkid_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083873_cnetworksynchronisedscenes_getlocalsceneid_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083875_cnetworksynchronisedscenes_gettimescenestarted_int(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_083877_cnetworksynchronisedscenes_allocatenewscenedescription_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083878_cnetworksynchronisedscenes_logscenedescription_rage_netloggingin(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_083879_cnetworksynchronisedscenes_releasescene_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083886_cnetworksynchronisedscenes_setscenerate_int_float(ph: *mut Phantom, a0: i32, a1: f32); }
extern "C" { pub fn f_083888_cnetworksynchronisedscenes_startscenerunning_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_083889_cnetworksynchronisedscenes_stopscenerunning_int_char_const_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_083890_cnetworksynchronisedscenes_checksyncedscenestateforped_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083897_networktextfilter_networktextfilter(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083898_csyncedentity_csyncedentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083902_csyncedentity_getentity(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083908_csyncedvehicle_getvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083909_csyncedvehicle_getvehicleid(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083911_networkutils_getobjecttypename_unsigned_short_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i64; }
extern "C" { pub fn f_083912_networkutils_getpickupfromnetworkobject_rage_netobject_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083913_networkutils_getpickupplacementfromnetworkobject_rage_netobject(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083914_networkutils_getplayerpedfromnetworkobject_rage_netobject_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083915_networkutils_gettrainfromnetworkobject_rage_netobject_ptr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083916_networkutils_getclosestremoteplayersinscope_cnetobjproximitymigr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_083917_networkutils_setfontsettingsfornetworkohd_ctextlayout_ptr_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_083919_networkutils_getplayerfromdamageentity_centity_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_083921_networkutils_getdebugcolourforplayer_rage_netplayer_const_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_083922_networkutils_getcanpasscontrolerrorstring_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083923_networkutils_getcanacceptcontrolerrorstring_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083924_networkutils_getfixedbynetworkreason_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083925_networkutils_getscopereason_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083926_networkutils_getmigrationfailtrackingerrorstring_unsigned_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_083927_networkutils_getspecialalpharampingvalue_short_ref_bool_ref_shor(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_083928_networkutils_getspecialalpharampingfadeintime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083929_networkutils_getspecialalpharampingfadeouttime(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_083936_networkutils_loggamerhandle_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_083937_cnetworkvehiclemodeldoorlockedtablemanager_isvehiclemodellocked(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083938_cnetworkvehiclemodeldoorlockedtablemanager_getplayerlockindexbym(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083939_cnetworkvehiclemodeldoorlockedtablemanager_isvehicleinstanceregi(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083940_cnetworkvehiclemodeldoorlockedtablemanager_getplayerlockindexbyv(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083941_cnetworkvehiclemodeldoorlockedtablemanager_isvehicleinstancelock(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_083942_cnetworkvehiclemodeldoorlockedtablemanager_setmodelidplayerlock(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083943_cnetworkvehiclemodeldoorlockedtablemanager_setmodelinstanceplaye(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_083946_cnetworkscriptworldstatemgr_addtohostresponsependinglist_cnetwor(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_083957_networkscriptworldstatetypes_createworldstate_netscriptworldstat(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_084175_clivemanager_checkvoicecommunicationprivileges_int_bool(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_084319_cnetworktelemetry_appendmetricreportinvalidmodelcreated_rage_rlg(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_084353_cnetworktelemetry_appendcashcreated_unsigned_long_rage_rlgamerha(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_084362_cnetworktelemetry_appendpedattached_int_bool_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_084414_cprofilestatsreadsessionstats_endstatsread(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_084416_cprofilestatsreadsessionstats_readstats_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_084531_cplayercardxmldatamanager_playercard_getstathashbynetid_cplayerc(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_084629_cloudlistener_dtorcloudlistener_138661(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085293_rage_netbroadcastdataarrayidentifierbase_read_rage_datbitbuffer(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085295_rage_netbroadcastdataarrayidentifierbase_write_rage_datbitbuffer(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085297_rage_netbroadcastdataarrayidentifier_cgamescriptid_getlogname_co(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085570_cnetworksession_ishost_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085593_cnetworkvoice_ontextmessagereceived_char_const_ptr_rage_rlgamerh(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_085643_cnetworkscnewsstoryrequest_cancel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085720_networkbaseconfig_getmatchmakingregion(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_085726_networkgameconfig_getmatchingattributes_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_085728_networkgameconfig_debugprint_bool_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085777_void_networkgameconfig_reset_player_schema_matchingattributes_pl(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_085785_cnetworksession_updatecurrentuniquecrews_sessiontype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_085800_bool_rage_playerdatamsg_sermsgpayload_rage_datexportbuffer_rage(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085814_cnetworksession_gettimeinsession_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_085823_cnetworksession_updatematchmakinggroups(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_085919_cnetworksession_getsessionmembers_rage_rlgamerinfo_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_085963_cnetworksession_gettransitionnetendpointid_rage_rlgamerinfo_cons(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_085975_cnetworkvoice_buildmutelists_rage_rlgamerhandle_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_085976_cnetworkvoice_addpendinggamer_cnetworkvoice_pendingvoicegamer_co(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_085982_cnetworksession_onreceivedtransitionsessioninfo_cnetgameplayer_c(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086081_cnetworkreadleaderboards_updatewidgets(ph: *mut Phantom); }
extern "C" { pub fn f_086090_cnetworkwriteleaderboards_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086108_cnetworkcommunitystats_loadcommunitystatsdata(ph: *mut Phantom); }
extern "C" { pub fn f_086126_rage_atarray_rage_rlgamerid_0_unsigned_short_grow_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086130_cnetworkvoice_setoverridechatrestrictions_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_086131_cnetworkvoice_applyplayerrestrictions_unsigned_int_ref_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_086135_cnetworkvoice_isgamertalking_rage_rlgamerid_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086136_cnetworkvoice_isanygamertalking_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086139_cnetworkvoice_gamerhasheadset_rage_rlgamerhandle_const_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086140_cnetworkvoice_mutegamer_rage_rlgamerhandle_const_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_086147_cnetworkvoice_settalkerfocus_rage_rlgamerhandle_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086159_cnetworkvoice_getplayerloudness_int(ph: *mut Phantom, a0: i64) -> f32; }
extern "C" { pub fn f_086162_bool_rage_netconnectionmanager_send_cmsgvoicestatus_int_cmsgvoic(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_086177_cnetworkvoice_bank_getchosenplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086183_netcatalogpackedstatinventoryitem_setupstatsmapping(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086216_netcatalog_dtornetcatalog(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086220_netcatalogbaseitem_onsetstatvalue_statid_ref_long(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086221_netcatalogbaseitem_deserialize_rage_rsonreader_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086222_netcatalogbaseitem_loadhasharrayparam_rage_rsonreader_const_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_086224_packed_stats_getstatenuminfo_int_int_ref_bool_ref_bool_ref(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_086225_gametransactionsessionmgr_get(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_086226_packed_stats_getstatkey_statid_ref_int_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_086247_netcatalog_shutdown(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086248_netcatalog_cancelsaveworker(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086253_netcatalog_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086254_netcatalog_requestserverfile(ph: *mut Phantom); }
extern "C" { pub fn f_086255_netcatalog_additem_rage_atnonfinalhashstring_const_ref_netcatalo(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086257_netcatalog_calculatepackedexceptions(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086258_rage_atmapentry_int_spackedstatsscriptexceptions_const_ref_rage(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_086260_netcatalog_writetocache(ph: *mut Phantom); }
extern "C" { pub fn f_086262_gametransactionsessionmgr_bail_gametransactionsessionmgr_estate(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_086263_gametransactionsessionmgr_setstate_gametransactionsessionmgr_est(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086264_gametransactionsessionmgr_clearstartsession_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_086267_netcatalog_areitemslinked_rage_atnonfinalhashstring_ref_rage_atn(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086269_netcatalog_setlatestversion_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_086283_netinventorybaseitem_deserialize_rage_rsonreader_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086293_cnetworkshoppingmgr_shutdown_unsigned_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086294_pendingcashreductionshelper_finished_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086295_cnetshoptransactionbase_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086296_cnetshoptransactionbase_getrequestid_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086299_cnetshoptransactionbase_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086300_cnetshoptransactionbase_finishprocessingstart_networkgametransac(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086301_spendgametransactionhttptask_ptr_rage_rltaskmanager_createtask_s(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086302_networkgametransactions_dogametransactionevent_int_networkgametr(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_086303_earngametransactionhttptask_ptr_rage_rltaskmanager_createtask_ea(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086305_cnetshoptransactionbase_registerpendingcash_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086307_cnetshoptransactionbase_checkout(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086308_cnetshoptransactionbase_sendresulteventtoscript_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086310_cnetshoptransactionbase_applydatatostats_cnetshoptransactionserv(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_086311_gametransactionsessionmgr_havestatsfinishedloading_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086312_networkgametransactions_playerbalance_applydatatostats_int_netwo(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_086313_networkgametransactions_inventoryitemset_applydatatostats_networ(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086314_networkgametransactions_playerbalance_serialize_rage_rsonwriter(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086315_rage_atarray_networkgametransactions_inventoryitem_0_unsigned_sh(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_086316_networkgametransactions_sendnullevent_int_rage_netstatus_ptr(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_086317_cnetshoptransactionbase_getslotidforcatalogitem_unsigned_int_con(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_086318_cnetshoptransactionbase_grcdebugdraw(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086321_cnetshoptransactionbasket_gettotalprice_bool_ref_int_ref_bool_re(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_086335_cnetworkshoppingmgr_createfreespaces_bool(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086336_cnetworkshoppingmgr_appendnewnode_cnetshoptransactionbase_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086337_cnetworkshoppingmgr_createbasket_unsigned_int_ref_unsigned_int_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_086338_cnetworkshoppingmgr_deletebasket(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086339_cnetworkshoppingmgr_additem_cnetshopitem_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086342_cnetworkshoppingmgr_startcheckout_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_086374_networkgametransactions_inventoryitemset_deserialize_rage_rsonre(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086376_networkgametransactions_gametransactionbase_serialize_rage_rsonw(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086377_networkgametransactions_playerbalance_deserialize_rage_rsonreade(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086378_networkgametransactions_playerbalance_applydatatostats_networkga(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086379_networkgametransactions_applyuserinfohack_int_rage_rsonwriter_re(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_086403_networkgametransactions_gametransactionitems_serialize_rage_rson(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086404_networkgametransactions_deleteslot_int_int_bool_bool_int_network(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_086412_purchgametransactionhttptask_ptr_rage_rltaskmanager_createtask_p(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086415_gametransactionsessionmgr_sessionstart_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_086419_gametransactionsessionmgr_initmanager_int(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_086429_gametransactionbasehttptask_configure_int_unsigned_int_rage_rson(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_086430_getgameservererrorcodename_int(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_086435_getaccesstokenhelper_init_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086436_getaccesstokenhelper_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086437_gametransactionhttptask_configure_int_unsigned_int_rage_rsonwrit(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_086441_heartbeatgametransactionhttptask_checkfailure_rage_rltaskbase_fi(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086446_gametransactionsessionmgr_applydata(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086454_gametransactionsessionmgr_shutdownmanager(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086455_gametransactionsessionmgr_triggergetbalance_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_086456_bool_rage_rltaskbase_configure_getbalancegametransactionhttptask(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_086457_gametransactionsessionmgr_triggercatalogrefresh(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086460_forceerrorhelper_serialize_rage_rsonwriter_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086469_gametransactionbasehttptask_gettransactioneventerrorcode_rage_at(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086473_gametransactionhttptask_processsuccess_rage_rsonreader_const_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086512_cnetworknewsstorymgr_dtorcnetworknewsstorymgr_142531(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086513_cnetshoptransactionbase_dtorcnetshoptransactionbase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086524_socialclubeventmgr_dtorsocialclubeventmgr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086638_cachecatalogparsableinfo_dtorcachecatalogparsableinfo(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086650_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_bool_ptr_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_086651_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_cnetworkre(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_086730_cvehiclesynctree_cvehiclesynctree_unsigned_int(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086731_rage_netsyncdata_static_32u_16u_274u_5u_netsyncdata_static(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086732_rage_netsyncdata_static_32u_14u_229u_5u_netsyncdata_static(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086733_void_validatesyncdata_cnetobjvehicle_cvehiclesyncdata_char_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086734_cnetobjvehicle_getdummyvehiclegadget_evehiclegadgettype(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_086735_cnetobjvehicle_getsizeofvehiclegadgetdata_evehiclegadgettype(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_086740_cnetworkobjectmgr_handletelemetrymetrics_const(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086741_cnetworkobjectpopulationmgr_canregisterobjectsoftype_unsigned_sh(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_086742_cnetworkobjectmgr_getnumcloseplayervehiclesnotonourmachine(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086743_cnetworkobjectpopulationmgr_canregisterlocalobjectsoftype_unsign(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_086744_cnetgameplayer_getplayerped_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086745_cnetworkobjectmgr_getnetworkobjectfromplayer_unsigned_short_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_086747_cnetworkobjectpopulationmgr_canregisterobject_unsigned_short_boo(ph: *mut Phantom, a0: i32, a1: i32) -> i32; }
extern "C" { pub fn f_086748_cnetworkobjectmgr_trytomakespaceforobject_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086751_cnetworkobjectmgr_getsynctree_unsigned_short(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_086755_cnetworkobjectmgr_checknetworkobjectspawndistance_rage_netobject(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086756_cnetworkobjectmgr_isobjecttoocloseforcreationorremoval_rage_neto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_086762_shouldaddlatencysample_rage_netobject_const_ref_rage_netplayer_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_086763_cnetobjgame_getplayerowner_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086767_cnetobjtrain_cnetobjtrain_ctrain_ptr_unsigned_short_unsigned_sho(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_086768_cnetobjdoor_cnetobjdoor_cdoor_ptr_unsigned_short_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_086769_cnetobjvehicle_cnetobjvehicle_cvehicle_ptr_unsigned_short_unsign(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_086770_cprojectsynctree_initialiseinitialstatebuffer_rage_netsynctreeta(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_086771_cnetobjphysical_cnetobjphysical_cphysical_ptr_unsigned_short_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_086772_cnetobjgame_cnetobjgame_cpickupplacement_cpickupplacement_ptr_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_086782_cnetgameplayer_canacceptmigratingobjects_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086784_cnetworkobjectmgr_canacceptscriptobject_cnetobjgame_ref_cgamescr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_086785_cnetobjentity_flagfordeletion(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086788_cnetworkobjectpopulationmgr_getnumexternallyrequiredentities_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_086791_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_086797_cnetworkobjectpopulationmgr_getnumreservedobjects_unsigned_int_u(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_086798_cnetworkobjectpopulationmgr_getnumcreatedobjects_unsigned_int_un(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_086799_cnetworkobjectpopulationmgr_getmaxobjectsallowedtoownoftype_unsi(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_086802_cnetobjvehicle_updaterouteforclone(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086805_cnetobjautomobile_processcontrol(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086806_cnetobjvehicle_processcontrol(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086807_cnetobjdynamicentity_processcontrol(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086808_cnetobjautomobile_getautomobilecreatedata_cautomobilecreationdat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086810_cnetobjautomobile_setautomobilecreatedata_cautomobilecreationdat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086822_cnetblenderphysical_cnetblenderphysical_cphysical_ptr_cphysicalb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_086824_cnetobjvehicle_changeowner_rage_netplayer_const_ref_rage_emigrat(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_086825_cnetobjphysical_changeowner_rage_netplayer_const_ref_rage_emigra(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_086826_cvehiclesynctree_getvehiclegamestatenode(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086827_cnetobjvehicle_updatelastpedsinseats(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086831_cnetobjboat_setboatgamestate_cboatgamestatedatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086833_cnetobjobject_cnetobjobject_cobject_ptr_unsigned_short_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_086834_cnetobjdoor_validatedoorsystemdata_char_const_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_086835_cnetobjobject_isambientfence_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086836_cnetobjdoor_dtorcnetobjdoor(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086837_cnetobjobject_removeunassignedambientobject_cnetobjobject_ref_ch(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_086838_cnetobjobject_removeunassignedvehiclefragment_cnetobjobject_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086839_cnetobjphysical_dtorcnetobjphysical(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086855_cnetobjdoor_setdoorsystemdata_cdoorsystemdata_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086857_cnetobjobject_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086858_cnetobjproximitymigrateable_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086859_cnetobjobject_trytobreakobject(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086860_cnetobjentity_setoverridinglocalvisibility_bool_char_const_ptr_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_086861_cnetobjphysical_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086865_cnetobjdoor_getdoorscriptinfo_cdoorscriptinfodatanode_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086868_cnetobjdoor_getdoorscriptgamestate_cdoorscriptgamestatedatanode(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086870_cnetobjdoor_getdoormovementdata_cdoormovementdatanode_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086874_cnetobjdoor_setdoorscriptinfo_cdoorscriptinfodatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086876_cnetobjdoor_setdoorscriptgamestate_cdoorscriptgamestatedatanode(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086884_cnetobjobject_changeowner_rage_netplayer_const_ref_rage_emigrati(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_086888_cnetobjphysical_onunregistered(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086892_cnetobjphysical_onreassigned(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086894_cnetobjobject_hastobecloned_cobject_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_086895_cnetobjobject_canbenetworked_cobject_ptr_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_086896_cnetobjobject_assignambientobject_cobject_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086898_cnetobjgame_canacceptcontrol_rage_netplayer_const_ref_rage_emigr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_086904_cnetobjdynamicentity_postcreate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086906_cnetobjentity_changeowner_rage_netplayer_const_ref_rage_emigrati(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_086907_cnetobjdynamicentity_getdynamicentitygamestate_cdynamicentitygam(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086909_cnetobjdynamicentity_setdynamicentitygamestate_cdynamicentitygam(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086915_cnetobjentity_setoverridingremotevisibility_bool_bool_char_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_086916_cnetobjentity_setoverridingremotecollision_bool_bool_char_const(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_086917_cnetobjentity_setoverridinglocalcollision_bool_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_086918_cnetobjentity_clearoverridinglocalcollisionimmediately_char_cons(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086919_cnetobjentity_hideforcutscene(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086920_cnetobjentity_exposeaftercutscene(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086921_cnetobjentity_manageupdatelevel_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086922_cnetobjentity_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086926_cnetobjproximitymigrateable_isinscope_rage_netplayer_const_ref_u(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_086928_cnetobjproximitymigrateable_canpasscontrol_rage_netplayer_const(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_086929_cnetobjgame_canpasscontrol_rage_netplayer_const_ref_rage_emigrat(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_086931_cnetobjgame_changeowner_rage_netplayer_const_ref_rage_emigration(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_086932_cnetobjentity_updatebeforephysics(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086946_cnetobjentity_setisfixed_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086947_cnetobjentity_setisvisible_bool_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_086951_cnetobjentity_islocalentityvisibleovernetwork(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086957_cnetobjentity_displaynetworkinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086958_cnetgameplayer_getplayerinfo_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_086960_cnetobjentity_displayblenderinfo_rage_vector2_ref_rage_color32_f(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: f32, a4: f32); }
extern "C" { pub fn f_086964_cnetobjgame_setgameobject_void_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086966_cnetobjgame_postmigrate_rage_emigrationtype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086971_cnetobjgame_onconversiontoambientobject(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086972_cnetobjgame_islocallyrunningscriptobject_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086978_cnetobjvehicle_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_086979_cnetobjvehicle_applyremoteweaponimpactpointinformation(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086980_cnetobjvehicle_processremotedoors_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_086981_sendnetworkeventincrementawdcarbombs_cvehicle_ptr_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086983_cnetobjvehicle_validatedriverandpassengers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_086986_cnetobjheli_gethelihealthdata_chelihealthdatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086988_cnetobjheli_gethelicontroldata_chelicontroldatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086990_cnetobjheli_sethelihealthdata_chelihealthdatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086992_cnetobjheli_sethelicontroldata_chelicontroldatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_086996_cnetobjphysical_fixphysicswhennomaporinterior_unsigned_int_ptr_c(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_087019_cnetobjphysical_registercollision_cnetobjphysical_ptr_float(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_087023_cnetobjphysical_getentityattachedto_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087024_cnetobjobject_findassociatedambientobject_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_087025_cnetobjobject_addunassignedambientobject_cnetobjobject_ref_char(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_087026_cnetobjphysical_processpendingattachment(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087027_cnetobjphysical_applypendingscriptobjinfo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087028_cnetobjphysical_processalpha(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087029_cnetobjphysical_concealentity_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087030_cnetobjphysical_hideundermap(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087035_cnetobjphysical_calcreassignpriority_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087036_cnetobjproximitymigrateable_calcreassignpriority_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087039_cnetobjphysical_isinscope_rage_netplayer_const_ref_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_087040_cnetobjphysical_recursiveareanychildrenownedbyplayer_cphysical_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_087046_cnetobjphysical_canpasscontrol_rage_netplayer_const_ref_rage_emi(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_087047_cnetobjphysical_recursiveareallchildrencloned_cphysical_const_pt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_087051_cnetobjproximitymigrateable_postmigrate_rage_emigrationtype(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087060_cnetobjobject_settintindexonobjecthelper_unsigned_char(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087067_cnetobjobject_getfragobjectfromparent_cobject_ref_unsigned_int_b(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i64; }
extern "C" { pub fn f_087068_cnetobjphysical_updateblenderstate_bool_bool_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_087075_cnetobjphysical_attemptpendingattachment_cphysical_ptr_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_087081_cnetobjobject_getobjectcreatedata_cobjectcreationdatanode_ref_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087083_cnetobjobject_setobjectcreatedata_cobjectcreationdatanode_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087084_cnetblenderphysical_updatematrix_rage_matrix34_const_ref_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_087088_cnetobjobject_getobjectgamestatedata_cobjectgamestatedatanode_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087090_cnetobjobject_getobjectscriptgamestatedata_cobjectscriptgamestat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087098_cnetobjobject_setobjectgamestatedata_cobjectgamestatedatanode_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087100_cnetobjobject_setobjectscriptgamestatedata_cobjectscriptgamestat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087107_cnetobjped_cnetobjped_cped_ptr_unsigned_short_unsigned_short_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32) -> i64; }
extern "C" { pub fn f_087108_cnetobjped_dtorcnetobjped(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087121_cnetobjped_setpedoutofvehicle_unsigned_int(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087122_cnetobjped_setclonepedoutofvehicle_bool_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087123_cnetobjped_migrateswithvehicle_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087124_cnetobjped_setclonepedinvehicle_cvehicle_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_087127_cnetobjped_dumptaskinformation_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087128_cnetobjped_getweapondata_unsigned_int_ref_bool_ref_bool_ref_unsi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64); }
extern "C" { pub fn f_087129_cnetobjped_checkpedscurrenttasksformigratedata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087132_cnetobjped_extracttasksequencedata_unsigned_short_ctasksequencel(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_087133_cnetobjped_postsetupforrespawnped_cvehicle_ptr_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_087134_cnetobjped_setdeadpedrespawnremovaltimer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087135_cnetobjped_updatelocaltaskslotcache(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087138_cnetobjped_update(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087139_cnetobjped_updatependingmovementdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087140_cnetobjped_updatecachedweaponinfo_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087141_cnetobjped_updateweaponstatusfromped(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087142_cnetobjped_updatependingpropdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087143_cnetobjphysical_setalphafading_bool_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087144_cnetobjped_freeorphanedmovenetworktasksubnetwork(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087145_cnetobjped_fixupnavmeshposition_rage_vector3_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_087146_cnetobjped_updatependingscriptedtask(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087147_cnetobjped_cacheweaponinfo(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087148_cnetobjped_updateweaponslots_unsigned_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087149_cnetobjped_setweaponcomponentdata_unsigned_int_unsigned_int_cons(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_087150_cnetobjped_setweapondata_unsigned_int_bool_bool_unsigned_char_bo(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_087151_cnetobjped_updateweaponforped(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087152_cpedsynctreebase_getphysicalgamestatenode(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087153_cphysicalsynctreebase_getphysicalgamestatenode(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087154_cnetobjped_givepedscriptedtask_ctask_ptr_int_char_const_ptr_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_087157_cnetobjped_processcontrol(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087158_cnetobjped_handleunmanagedcloneragdoll(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087160_cnetobjped_forceresendofalltaskdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087161_cnetobjproximitymigrateable_startsynchronising(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087164_cnetobjped_cleanupgameobject_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087172_cnetobjped_networkblenderisoverridden_unsigned_int_ptr_const(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_087177_cnetobjped_isinscopevehiclechecks_rage_netplayer_const_ref_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_087178_cnetobjped_isinscopenovehiclechecks_rage_netplayer_const_ref_uns(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_087184_cnetobjped_setuppedaiasclone(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087186_cnetobjped_onunregistered(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087188_cnetobjped_postcreate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087190_cnetobjphysical_postsync(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087193_cnetobjphysical_converttoambientobject(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087196_cnetobjped_playerhasleft_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087200_cnetobjproximitymigrateable_trytopasscontrolproximity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087203_cnetobjproximitymigrateable_trytopasscontrolfromtutorial(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087204_cnetobjped_getpedcreatedata_cpedcreationdatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087208_cnetobjped_getpedgamestatedata_cpedgamestatedatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087209_cnetobjped_getweaponcomponentdata_unsigned_int_ptr_unsigned_char(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_087211_cnetobjped_getpedscriptgamestatedata_cpedscriptgamestatedatanode(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087212_csyncedpedvehicleentryscriptconfigdata_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087215_cnetobjped_getstandingonobjectdata_unsigned_short_ref_rage_vecto(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_087221_cnetobjped_getpedhealthdata_cpedhealthdatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087223_cnetobjped_getattachmentdata_cpedattachdatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087225_cnetobjped_getmotiongroup_cpedmovementgroupdatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087227_cnetobjped_getpedmovementdata_cpedmovementdatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087229_cnetobjped_getpedappearancedata_cpackedpedprops_ref_csyncedpedva(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i64, a13: i64, a14: i64, a15: i64, a16: i64, a17: i64, a18: i64, a19: i64); }
extern "C" { pub fn f_087233_cnetobjped_gettasktreedata_cpedtasktreedatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087235_cnetobjped_gettaskspecificdata_cpedtaskspecificdatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087237_cnetobjped_getpedinventorydata_cpedinventorydatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087240_cnetobjped_gettasksequencedata_bool_ref_unsigned_int_ref_unsigne(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64); }
extern "C" { pub fn f_087242_cnetobjped_setpedcreatedata_cpedcreationdatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087246_cnetobjped_setpedgamestatedata_cpedgamestatedatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087248_cnetobjped_setpedscriptgamestatedata_cpedscriptgamestatedatanode(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087251_cnetobjproximitymigrateable_setsectorposition_float_float_float(ph: *mut Phantom, a0: i64, a1: f32, a2: f32, a3: f32); }
extern "C" { pub fn f_087255_cnetobjped_setsectornavmeshposition_cpedsectorposnavmeshnode_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087257_cnetobjped_setpedheadings_cpedorientationdatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087259_cnetobjped_setpedhealthdata_cpedhealthdatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087262_cnetobjphysical_updatedamagetracker_cnetobjphysical_networkdamag(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_087265_cnetobjphysical_sethidewheninvisible_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087269_cnetobjped_setattachmentdata_cpedattachdatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087271_cnetobjped_setmotiongroup_cpedmovementgroupdatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087273_cnetobjped_setpedmovementdata_cpedmovementdatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087275_cnetobjped_setpedappearancedata_cpackedpedprops_csyncedpedvardat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32, a12: i32, a13: i32, a14: i32, a15: i32, a16: i32, a17: i32, a18: i32, a19: i32, a20: i32); }
extern "C" { pub fn f_087279_cnetobjped_settasktreedata_cpedtasktreedatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087281_cnetobjped_settaskspecificdata_cpedtaskspecificdatanode_const_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087283_cnetobjped_setpedinventorydata_cpedinventorydatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087285_cnetobjped_settasksequencedata_unsigned_int_ipednodedataaccessor(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_087287_cnetobjped_posttasksupdate(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087300_cnetobjphysical_canprocesspendingattachment_cphysical_ptr_ptr_in(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_087303_cnetobjphysical_attemptpendingdetachment_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087305_cnetobjphysical_updatependingattachment(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087312_cnetobjped_updatefixedbynetwork_bool_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_087313_cnetobjphysical_updatefixedbynetwork_bool_unsigned_int_unsigned(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_087315_cnetobjped_resurrectclone(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087316_cnetobjped_updateblenderdata(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087325_cnetobjped_displaynetworkinfoforobject_rage_color32_const_ref_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: f32); }
extern "C" { pub fn f_087326_cnetobjphysical_displaynetworkinfoforobject_rage_color32_const_r(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: f32); }
extern "C" { pub fn f_087327_cnetobjproximitymigrateable_displaynetworkinfoforobject_rage_col(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i64, a4: f32); }
extern "C" { pub fn f_087339_cnetobjphysical_getisattachedforsync_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087351_cnetobjphysical_setisvisible_bool_char_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32); }
extern "C" { pub fn f_087352_cnetobjphysical_stopalpharamping(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087354_cnetblenderphysical_getquaternionsfrommatrices_rage_matrix34_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_087355_cnetobjphysical_trytopasscontrolproximity(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087362_cnetobjphysical_hideforcutscene(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087367_cnetobjphysical_processghosting(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087368_cnetobjphysical_setpendingscriptobjinfo_cgamescriptobjinfo_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087371_cnetobjphysical_getphysicalscriptmigrationdata_cphysicalscriptmi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087375_cnetobjphysical_setphysicalscriptmigrationdata_cphysicalscriptmi(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087377_cnetobjphysical_getphysicalgamestatedata_cphysicalgamestatedatan(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087379_cnetobjphysical_getphysicalscriptgamestatedata_cphysicalscriptga(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087381_cnetobjphysical_getvelocity_int_ref_int_ref_int_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_087383_cnetobjphysical_getangularvelocity_cphysicalangvelocitydatanode(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087385_cnetobjphysical_getphysicalhealthdata_cphysicalhealthdatanode_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087387_cnetobjphysical_getphysicalattachmentdata_bool_ref_unsigned_shor(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i64, a13: i64); }
extern "C" { pub fn f_087389_cnetobjphysical_setphysicalgamestatedata_cphysicalgamestatedatan(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087390_cnetobjphysical_setalpharampinginternal_bool_bool_bool_bool_shor(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_087392_cnetobjphysical_setphysicalscriptgamestatedata_cphysicalscriptga(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087396_cnetobjphysical_setangularvelocity_cphysicalangvelocitydatanode(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087398_cnetobjphysical_updatedamagetrackers_cphysicalhealthdatanode_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087405_cnetobjphysical_setasghost_bool_unsigned_int_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_087410_cnetobjphysical_setfixedbynetwork_bool_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_087439_cnetobjproximitymigrateable_checkplayerhasauthorityoverobject_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_087442_cnetobjproximitymigrateable_trytopasscontroloutofscope(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087450_cnetobjpickup_getpickupcreatedata_cpickupcreationdatanode_ref_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087454_cnetobjpickup_setpickupcreatedata_cpickupcreationdatanode_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087456_cnetobjpickup_setpickupscriptgamestatedata_cpickupscriptgamestat(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087477_cnetobjpickupplacement_getpickupplacementcreatedata_cpickupplace(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087479_cnetobjpickupplacement_getpickupplacementstatedata_cpickupplacem(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087481_cnetobjpickupplacement_setpickupplacementcreatedata_cpickupplace(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087483_cnetobjpickupplacement_setpickupplacementstatedata_cpickupplacem(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087489_cnetobjplane_updatelockontarget(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087492_cnetobjplane_getplanegamestate_cplanegamestatedatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087494_cnetobjplane_getplanecontroldata_cplanecontroldatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087496_cnetobjplane_setplanegamestate_cplanegamestatedatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087498_cnetobjplane_setplanecontroldata_cplanecontroldatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087504_cnetobjsubmarine_getsubmarinecontroldata_csubmarinecontroldatano(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087506_cnetobjsubmarine_setsubmarinecontroldata_csubmarinecontroldatano(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087518_cnetobjplayer_cnetobjplayer_cped_ptr_unsigned_short_unsigned_sho(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i64; }
extern "C" { pub fn f_087519_cnetobjplayer_updatecachedtutorialsessionstate(ph: *mut Phantom); }
extern "C" { pub fn f_087520_cnetobjplayer_dtorcnetobjplayer(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087535_cnetobjplayer_setrespawninvincibilitytimer_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087538_cnetobjplayer_setuppendingtutorialchange(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087544_cnetobjplayer_shouldoverrideblenderforsecondaryanim_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087546_cnetobjplayer_updatetargettedentityposition(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087547_cnetobjplayer_calcstatvalue_cplayercardxmldatamanager_playercard(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> f32; }
extern "C" { pub fn f_087554_cnetobjplayer_clearcachedplayerheadblenddata_unsigned_char(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_087571_cnetobjplayer_applyheadblenddata_cped_ref_cpedheadblenddata_ref(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_087572_cnetobjplayer_getplayercreatedata_cplayercreationdatanode_ref_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087573_cnetobjplayer_getscarandblooddata_unsigned_int_ref_cpedbloodnetw(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_087575_cnetobjplayer_setplayercreatedata_cplayercreationdatanode_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087576_cnetobjplayer_setscarandblooddata_unsigned_int_cpedbloodnetworkd(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_087578_cnetobjplayer_getplayersectorposdata_cplayersectorposnode_ref_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087580_cnetobjplayer_getplayergamestatedata_cplayergamestatedatanode_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087581_cnetgameplayer_getvoiceproximity_rage_vector3_ref_const(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_087583_cnetobjplayer_getplayerappearancedata_cplayerappearancedatanode(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087585_cnetobjplayer_getcameradata_cplayercameradatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087586_cnetobjplayer_istargetentitysuitableforoffsetaiming_centity_cons(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_087587_cnetobjplayer_targetcloninginformation_setaimtarget_cped_const_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_087588_cnetobjplayer_targetcloninginformation_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087591_cnetobjplayer_getplayerpedgroupdata_cplayerpedgroupdatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087593_cnetobjplayer_getplayerwantedandlosdata_cplayerwantedandlosdatan(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087597_cnetobjplayer_getplayergamerdata_cplayergamerdatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087599_cnetobjplayer_getplayerextendedgamestatedata_cplayerextendedgame(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087601_cnetobjplayer_setpedhealthdata_cpedhealthdatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087607_cnetobjplayer_setplayersectorposdata_cplayersectorposnode_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087609_cnetobjplayer_setplayergamestatedata_cplayergamestatedatanode_co(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087613_cnetobjplayer_setplayerappearancedata_cplayerappearancedatanode(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087615_cnetobjplayer_setcameradata_cplayercameradatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087617_cnetobjplayer_setplayerpedgroupdata_cplayerpedgroupdatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087619_cnetobjplayer_setplayerwantedandlosdata_cplayerwantedandlosdatan(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087623_cnetobjplayer_setplayergamerdata_cplayergamerdatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087624_cnetgameplayer_setclanmembershipinfo_rage_rlclanmembershipdata_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087625_cnetgameplayer_settransitionsessioninfo_rage_rlsessioninfo_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087626_cnetgameplayer_setmutedata_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_087630_cnetobjplayer_setplayerextendedgamestatedata_cplayerextendedgame(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087643_cnetobjplayer_respawnplayerped_rage_vector3_const_ref_bool_unsig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_087675_cnetobjproximitymigrateable_getmigrationdata_unsigned_int_ref_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_087681_cnetobjvehicle_allowfixbynetwork_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087685_cnetobjvehicle_manageupdatelevel_rage_netplayer_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087687_cnetobjvehicle_isinscope_rage_netplayer_const_ref_unsigned_int_p(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_087689_cnetobjvehicle_canshowghostalpha_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087696_cnetobjvehicle_getphysicalattachmentdata_bool_ref_unsigned_short(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64, a10: i64, a11: i64, a12: i64, a13: i64); }
extern "C" { pub fn f_087702_cnetobjvehicle_attemptpendingattachment_cphysical_ptr_unsigned_i(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_087704_cnetobjvehicle_attemptpendingdetachment_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087709_cnetobjtrain_dtorcnetobjtrain(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087710_cnetobjvehicle_dtorcnetobjvehicle(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_087726_cnetobjtrain_updatependingengine(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087728_cnetblenderphysical_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087733_cnetobjvehicle_canpasscontrol_rage_netplayer_const_ref_rage_emig(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_087740_cnetobjtrain_gettraingamestate_ctraingamestatedatanode_ref_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087742_cnetobjtrain_settraingamestate_ctraingamestatedatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087755_cnetobjvehicle_evictallpeds(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087756_cnetobjvehicle_detonatephoneexplosive_unsigned_short(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087767_cnetobjvehicle_recursivedetachmentofchildren_cphysical_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087789_cnetobjvehicle_dirtyvehicledamagestatusdatasyncnode(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087790_cnetobjvehicle_getvehiclecreatedata_cvehiclecreationdatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087792_cnetobjvehicle_setvehiclecreatedata_cvehiclecreationdatanode_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087794_cnetobjvehicle_getvehiclemigrationdata_cvehicleproximitymigratio(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087796_cnetobjvehicle_setvehiclemigrationdata_cvehicleproximitymigratio(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087798_cnetobjvehicle_getvehicleangularvelocity_cvehicleangvelocitydata(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087801_cnetobjvehicle_getvehiclegamestate_cvehiclegamestatedatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087803_cnetobjvehicle_getvehiclescriptgamestate_cvehiclescriptgamestate(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087805_cnetobjvehicle_getvehiclehealth_cvehiclehealthdatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087809_cnetobjvehicle_getvehiclecontroldata_cvehiclecontroldatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087811_cnetobjvehicle_getvehicleappearancedata_cvehicleappearancedatano(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087813_cnetobjvehicle_getvehicledamagestatusdata_cvehicledamagestatusda(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087815_cnetobjvehicle_getvehicleaitask_cvehicletaskdatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087817_cnetobjvehicle_getcomponentreservations_cvehiclecomponentreserva(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087819_cnetobjvehicle_getvehiclegadgetdata_cvehiclegadgetdatanode_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087823_cnetobjvehicle_setvehiclegamestate_cvehiclegamestatedatanode_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087824_cnetobjvehicle_setextrafragsbreakable_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087826_cnetobjvehicle_processbadging_cvehicle_ptr_cvehiclebadgedesc_con(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_087827_cnetobjvehicle_setvehiclescriptgamestate_cvehiclescriptgamestate(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087829_cnetobjvehicle_setvehiclehealth_cvehiclehealthdatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087834_cnetobjvehicle_setvehiclecontroldata_cvehiclecontroldatanode_con(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087835_findclosestnodetodirection_rage_cnodeaddress_const_ref_rage_vect(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_087836_findroutebetweennodes_rage_cnodeaddress_const_ref_rage_cnodeaddr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_087838_cnetobjvehicle_setvehicleappearancedata_cvehicleappearancedatano(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087840_cnetobjvehicle_setvehicledamagestatusdata_cvehicledamagestatusda(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087842_cnetobjvehicle_setvehicleaitask_cvehicletaskdatanode_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087844_cnetobjvehicle_setvehiclegadgetdata_cvehiclegadgetdatanode_const(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087846_cnetobjvehicle_setcomponentreservations_cvehiclecomponentreserva(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087856_cnetobjvehicle_findanddetachattachedplayer_centity_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087871_cnetobjvehicle_getlowestallowedupdatelevel_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_087886_cnetblendervehicle_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087887_cnetblenderphysical_update(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087891_cnetblendervehicle_setpositiononobject_rage_vector3_const_ref_bo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_087892_cnetblenderphysical_setpositiononobject_rage_vector3_const_ref_b(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_087893_cnetblendervehicle_testformapcollision_rage_vector3_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_087898_cnetblenderped_checkforstuckonvehicle(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087899_cnetblenderped_updateheadingblending(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087900_cnetblenderped_setheadingonobject_float(ph: *mut Phantom, a0: i64, a1: f32); }
extern "C" { pub fn f_087901_cnetblenderped_attachragdoll(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087906_cnetblenderped_testformapcollision_rage_vector3_const_ref_rage_v(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_087926_cnetblenderphysical_onownerchange_rage_blenderresetflags(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_087927_cnetblenderphysical_updatenocollisiontimer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087929_cnetblenderphysical_setmatrixonobject_rage_matrix34_ref_bool_boo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_087930_cnetblenderphysical_updatepedsstandingonobjectafterwarp_rage_mat(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_087934_cnetblenderphysical_setvelocityonobject_rage_vector3_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087937_cnetblenderphysical_updateorientationblending(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_087962_cautomobilecreationdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087971_cprojectbasesyncdatanode_isreadytosendtoplayer_rage_netsynctreet(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_087972_cprojectbasesyncdatanode_getisattachedforplayers_rage_netsynctre(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_087976_cboatgamestatedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087982_rage_scriptobjinfobase_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087985_cdoorscriptgamestatedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_087994_cdynamicentitygamestatedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088004_chelihealthdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088007_chelicontroldatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088012_cobjectcreationdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088014_cobjectsectorposnode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088015_cobjectorientationnode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088016_cobjectgamestatedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088019_cobjectscriptgamestatedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088024_cpedcreationdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088027_cpedgamestatedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088031_cpedsectorposmapnode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088037_csyncedpedvehicleentryscriptconfig_hasnondefaultstate_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_088039_csyncedpedvehicleentryscriptconfig_getforcingseatindexforthisveh(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_088042_cpedscriptgamestatedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088044_cpedhealthdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088047_cpedattachdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088051_cpedmovementgroupdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088052_cpedmovementdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088055_cpedappearancedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088060_cpedtasktreedatanode_extractdataforserialising_ipednodedataacces(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088061_cpedtasktreedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088062_void_ipednodedataaccessor_ctaskdata_serialise_rage_csyncdatabase(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_088065_cpedinventorydatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088070_cpedtasksequencedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088072_cphysicalgamestatedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088075_cphysicalscriptgamestatedatanode_serialise_rage_csyncdatabase_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088080_cphysicalhealthdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088081_cphysicalattachdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088086_cpickupcreationdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088088_cpickupscriptgamestatenode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088091_cpickupplacementcreationdatanode_serialise_rage_csyncdatabase_re(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088092_cpickupplacementstatedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088095_cplayercreationdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088096_cplayersectorposnode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088097_cplayercameradatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088098_cplayergamestatedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088100_cplayerappearancedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088101_cplayerwantedandlosdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088104_cplayergamerdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088105_cplayerextendedgamestatenode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088110_ctraingamestatedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088113_cvehiclecreationdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088116_cvehiclegamestatedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088119_cvehiclescriptgamestatedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088123_cvehiclehealthdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088127_cvehiclecontroldatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088130_serialiseoptimizedsmallfloat_rage_csyncdatabase_ref_float_ref_fl(ph: *mut Phantom, a0: i64, a1: i64, a2: f32, a3: i32); }
extern "C" { pub fn f_088131_serialiseoptimized101float_rage_csyncdatabase_ref_float_ref_floa(ph: *mut Phantom, a0: i64, a1: i64, a2: f32); }
extern "C" { pub fn f_088132_cvehicleappearancedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088133_cvehicledamagestatusdatanode_extractdataforserialising_ivehiclen(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088134_cvehicledamagestatusdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088136_cvehicleproximitymigrationdatanode_serialise_rage_csyncdatabase(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088137_cvehiclegadgetdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088139_cplanegamestatedatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088142_cplanecontroldatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088150_cproximitymigrateablesynctreebase_cproximitymigrateablesynctreeb(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_088156_centitysynctreebase_centitysynctreebase_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_088157_cphysicalsynctreebase_cphysicalsynctreebase_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_088158_cpedsynctreebase_cpedsynctreebase_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_088182_cnetgameplayer_cnetgameplayer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_088187_cnetgameplayer_ischeatalreadynotified_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_088190_cnetgameplayer_setgamerinfo_rage_rlgamerinfo_const_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088194_cnetgameplayer_setplayerinfo_cplayerinfo_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088203_cnetworkplayermgr_dtorcnetworkplayermgr(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_088206_cnetworkplayermgr_tryloadtunables(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_088207_cnetworkplayermgr_sendplayedwithmetric_char_const_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088214_cnetworkplayermgr_removeplayedwith_rage_netplayer_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088227_croamingbubble_addplayer_cnetgameplayer_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_088228_croamingbubble_removeplayer_cnetgameplayer_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088232_croamingbubblemgr_allocateplayerinitialbubble_cnetgameplayer_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088233_croamingbubblemgr_addplayertobubble_cnetgameplayer_ptr_croamingb(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_088234_croamingbubblemgr_removeplayerfrombubble_cnetgameplayer_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088236_croamingjoinbubblemsg_export_void_ptr_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_088237_croamingbubblemgr_allocateremotebubble_cnetgameplayer_ptr(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_088238_croamingjoinbubbleackmsg_writetologfile_bool_unsigned_short_rage(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_088239_bool_croamingbubblememberinfo_serialisemsg_rage_datimportbuffer(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_088248_cnetworkparty_bank_refreshpartymembers(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_088249_cnetworkparty_canparty(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_088250_cnetworkparty_isinparty_const(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_088258_partyinvitemgr_removeinvite_unsigned_int(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_088417_cproximitymigrateablesynctreebase_dtorcproximitymigrateablesynct(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_088445_cphysicalsynctreebase_dtorcphysicalsynctreebase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_088466_cpedsynctree_dtorcpedsynctree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_088467_cpedsynctreebase_dtorcpedsynctreebase(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_088547_cplayersynctree_dtorcplayersynctree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_088576_cvehiclesynctree_dtorcvehiclesynctree(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_088670_cmigrationdatanode_serialise_rage_csyncdatabase_ref(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_089227_rage_netsyncdata_static_nobuffers_32u_25u_5u_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_089229_rage_netsyncdata_static_nobuffers_32u_25u_5u_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_089252_rage_netsyncdata_static_nobuffers_32u_29u_5u_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_089254_rage_netsyncdata_static_nobuffers_32u_29u_5u_reset(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_089320_bool_croamingbubblememberinfo_serialisemsg_rage_datexportbuffer(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_089334_cgivecontrolevent_trigger_rage_netplayer_const_ref_rage_netobjec(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_089335_cgivecontrolevent_addgivecontroldata_rage_netplayer_const_ref_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_089336_cgivecontrolevent_cgivecontrolevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_089340_cgivecontrolevent_dtorcgivecontrolevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_089356_cweapondamageevent_computewillkillplayer_cped_const_ref_cped_ref(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_089357_cweapondamageevent_dtorcweapondamageevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_089358_cweapondamageevent_processdamage_rage_netobject_ptr_rage_netobje(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_089389_crequestmappickupevent_getplacement_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_089404_cdebuggameclockevent_trigger_unsigned_int_rage_netplayer_const_p(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_089412_cdebuggameweatherevent_trigger_bool_unsigned_int_unsigned_int_ra(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_089430_cgiveweaponevent_trigger_cped_ptr_unsigned_int_int_bool(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_089438_cremoveweaponevent_trigger_cped_ptr_unsigned_int(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_089444_cremoveallweaponsevent_trigger_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_089450_cvehiclecomponentcontrolevent_trigger_unsigned_short_unsigned_sh(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i32, a4: i64); }
extern "C" { pub fn f_089462_cfireevent_getplayersinscopeforfiredata_cfireevent_sfiredata_con(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_089480_cstartprojectileevent_calculatemovingvehiclerelativefirepoint_ra(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_089487_cstartprojectileevent_makeweaponmatrix_rage_matrix34_ref_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_089577_cmarkasnolongerneededevent_trigger_cnetobjgame_ref_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_089584_cconverttoscriptentityevent_converttoscriptentity(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_089587_cconverttoscriptentityevent_trigger_cnetobjphysical_const_ref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_089597_cscriptworldstateevent_trigger_cnetworkworldstatedata_const_ref(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_089629_cstartnetworksyncedsceneevent_trigger_int_unsigned_int_rage_netp(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_089653_cgivepedscriptedtaskevent_cgivepedscriptedtaskevent_cped_ptr_cta(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_089654_cgivepedscriptedtaskevent_givepedscriptedtask(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_089657_cgivepedscriptedtaskevent_removeexistingevents_cped_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_089661_void_cgivepedscriptedtaskevent_serialiseevent_rage_csyncdatawrit(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_089662_cgivepedscriptedtaskevent_handle_rage_datbitbuffer_ref_rage_netp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_089664_cgivepedscriptedtaskevent_writeeventtologfile_bool_bool_rage_dat(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_089669_cgivepedsequencetaskevent_gettasksequencelist_const(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_089671_cgivepedsequencetaskevent_serialisetasksequencedata_rage_csyncda(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_089677_cclearpedtasksevent_trigger_cped_ptr_bool(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_089711_cgarageoccupiedstatusevent_trigger_int_unsigned_int_unsigned_cha(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_089733_cscriptentitystatechangeevent_csettingoftaskvehicletempaction_in(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_089749_cscriptentitystatechangeevent_trigger_rage_netobject_ptr_cscript(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_089752_cscriptentitystatechangeevent_createparameterclass_cscriptentity(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_089794_crequestdoorevent_dtorcrequestdoorevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_089817_cnetworktrainreportevent_trigger_int_bool_unsigned_char(ph: *mut Phantom, a0: i32, a1: i32, a2: i32); }
extern "C" { pub fn f_089837_cnetworkincrementstatevent_trigger_unsigned_int_int_rage_netplay(ph: *mut Phantom, a0: i32, a1: i32, a2: i64); }
extern "C" { pub fn f_089857_csendkickvotesevent_trigger_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_089878_cblowupvehicleevent_dtorcblowupvehicleevent(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_089929_cnetworkcheckexesizeevent_trigger_unsigned_char_unsigned_long_bo(ph: *mut Phantom, a0: i32, a1: i64, a2: i32); }
extern "C" { pub fn f_089982_cremovepedfrompedgroupevent_trytoremoveped_cped_ptr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_089992_cupdatefxnevent_trigger_unsigned_int_unsigned_int(ph: *mut Phantom, a0: i32, a1: i32); }
extern "C" { pub fn f_090014_cnetworkcheckcatalogcrc_trigger_bool(ph: *mut Phantom, a0: i32); }
extern "C" { pub fn f_090021_void_std_2_introsort_std_2_classicalgpolicy_bool_ptr_ref_rage_at(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32); }
extern "C" { pub fn f_090152_silk_bwexpander_32(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_090153_silk_lpc_inverse_pred_gain_c(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090154_silk_nlsf2a(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090155_silk_bwexpander(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_090156_silk_lpc_analysis_filter(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_090157_silk_sum_sqr_shift(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_090158_silk_plc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_090159_silk_reset_decoder(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090160_silk_resampler_private_up2_hq(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_090161_silk_resampler_private_iir_fir(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_090162_silk_resampler_private_down_fir(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_090165_ec_decode(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090166_ec_dec_update(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_090167_ec_dec_bit_logp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090168_ec_dec_icdf(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090169_ec_dec_uint(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090170_ec_dec_bits(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090173_silk_nlsf_unpack(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_090174_silk_decode_indices(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_090175_ec_encode(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_090176_ec_enc_uint(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_090177_ec_enc_bits(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_090178_silk_decode_pulses(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_090183_ec_tell_frac(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090185_exp_rotation(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32); }
extern "C" { pub fn f_090186_renormalise_vector(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_090187_isqrt32(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090190_deinterleave_hadamard(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_090191_quant_partition(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32, a7: f32, a8: i32) -> i32; }
extern "C" { pub fn f_090192_compute_theta(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32, a10: i64); }
extern "C" { pub fn f_090195_celt_pitch_xcorr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_090707_acosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090710_asinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090713_atan2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090716_cxa_atexit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090718_atoi(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090728_cos(ph: *mut Phantom, a0: f64) -> f64; }
extern "C" { pub fn f_090732_cosf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090748_exp(ph: *mut Phantom, a0: f64) -> f64; }
extern "C" { pub fn f_090814_memcmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090826_powf(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090901_qsort_r(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_090911_rand(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090914_roundf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090918_sinf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090920_snprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090921_siprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090922_small_sprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090923_sscanf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090930_strcmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090931_strcpy(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090939_strlen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090940_strncasecmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090942_strncmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_091002_emscripten_builtin_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
