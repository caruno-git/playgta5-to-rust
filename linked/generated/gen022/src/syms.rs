//! Callee declarations for crate gen022.
//!
//! Every `call $name` in the wasm module becomes a call through this
//! module: an `extern "C"` declaration of the target's generated
//! identifier.  That keeps a function's callers independent of which
//! generated crate the callee ended up in (they are all linked into the
//! same binary and the identifiers are unique).
#![allow(improper_ctypes, dead_code, unused_imports)]
use wasm_rt::Phantom;

extern "C" { pub fn f_000086_wasm_init_tls(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000100_op_new_unsigned_long(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_000102_op_delete_void_ptr(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000193_void_std_2_call_once_proxy_abi_nqn220108_std_2_tuple_void_ref_vo(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_000197_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom); }
extern "C" { pub fn f_000609_rage_voicechat_nativepackchat_int_void_ptr_unsigned_int_ptr(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_013054_std_2_vector_rage_bgedgeref_rage_stlallocator_rage_bgedgeref_thr(ph: *mut Phantom); }
extern "C" { pub fn f_014046_xmalloc(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_014047_xfree(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_037071_void_rage_atplaceptrcreator_void_ptr_cvideoeditormenuoption_void(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090159_silk_reset_decoder(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090163_silk_resampler(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090164_ec_dec_init(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090166_ec_dec_update(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_090167_ec_dec_bit_logp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090168_ec_dec_icdf(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090169_ec_dec_uint(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090170_ec_dec_bits(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090171_silk_stereo_decode_pred(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090172_silk_stereo_decode_mid_only(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090174_silk_decode_indices(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_090177_ec_enc_bits(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_090178_silk_decode_pulses(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_090179_silk_decode_frame(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_090181_silk_resetdecoder(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090182_comb_filter(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: f32, a6: f32, a7: i32, a8: i32, a9: i64, a10: i32); }
extern "C" { pub fn f_090183_ec_tell_frac(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090184_unquant_energy_finalise(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i64, a5: i64, a6: i32, a7: i64, a8: i32); }
extern "C" { pub fn f_090186_renormalise_vector(ph: *mut Phantom, a0: i64, a1: i32, a2: f32); }
extern "C" { pub fn f_090188_denormalise_bands(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32); }
extern "C" { pub fn f_090189_quant_band(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i32, a7: i64, a8: f32, a9: i64, a10: i32) -> i32; }
extern "C" { pub fn f_090192_compute_theta(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32, a7: i32, a8: i32, a9: i32, a10: i64); }
extern "C" { pub fn f_090193_celt_lpc(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090194_celt_autocorr(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_090195_celt_pitch_xcorr_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_090196_clt_mdct_backward_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_090198_opus_custom_decoder_ctl(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090199_celt_decode_with_ec_dred(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_090200_deemphasis(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i64, a6: i64, a7: i32); }
extern "C" { pub fn f_090201_prefilter_and_fold(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_090202_celt_synthesis(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: i32, a11: i32); }
extern "C" { pub fn f_090203_celt_decode_with_ec(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_090205_opus_decode_frame(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_090206_smooth_fade(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i64, a6: i32); }
extern "C" { pub fn f_090208_wc_aessetkey(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_090209_wc_aessetkeylocal(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_090210_wc_aesencrypt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090211_wc_aescbcencrypt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090212_wc_aescbcdecrypt(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090213_wc_aesgcmsetkey(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090214_ghash(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32); }
extern "C" { pub fn f_090215_wc_aesgcmsetiv(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090216_wc_aesinit(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090217_getlength(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090218_getint(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090219_getasnint(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090220_checkbitstring(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_090221_oidfromid(ph: *mut Phantom, a0: i32, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_090222_getobjectid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_090223_getalgoid(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_090224_totraditionalinline_ex(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_090225_setalgoid(ph: *mut Phantom, a0: i32, a1: i64) -> i32; }
extern "C" { pub fn f_090226_wc_rsapublickeydecode(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090227_setasnintmp(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090228_initdecodedcert(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_090229_freedecodedcert(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090230_freesignaturectx(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090231_getname(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_090232_gettime(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090233_getdate(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_090234_decodetokey(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090235_wc_encodesignature(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_090236_parsecertrelative(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_090237_decodecertextensions(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090238_decodeecc_dsa_sig(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090239_wc_eccpublickeydecode(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090240_matchbasename(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_090241_freesigner(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090242_allocder(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_090243_freeder(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090244_pemtoder(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_090245_base64_skipnewline(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090246_wc_curve25519_make_key(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090247_wc_curve25519_shared_secret_ex(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090248_wc_curve25519_export_public_ex(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090249_wc_curve25519_import_public_ex(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090250_wc_curve25519_check_public(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090251_wc_curve25519_init_ex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090252_wc_curve25519_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090253_wc_initdhkey_ex(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090254_wc_freedhkey(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090255_ffc_validate_public_key(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090256_wc_dhcheckpubvalue(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090257_wc_dhgeneratekeypair(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_090258_wc_dhagree(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64, a6: i32) -> i32; }
extern "C" { pub fn f_090259_wc_dhsetcheckkey(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_090260_dhsetkey(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32, a6: i64) -> i32; }
extern "C" { pub fn f_090261_wc_dhsetkey(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_090262_ecc_projective_dbl_point(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090263_ecc_map_ex(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_090264_wc_ecc_new_point_ex(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090265_wc_ecc_gen_z(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_090266_ecc_projective_dbl_point_safe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090267_ecc_projective_add_point_safe(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i64) -> i32; }
extern "C" { pub fn f_090268_wc_ecc_del_point_ex(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090269_wc_ecc_mulmod_ex2(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_090270_wc_ecc_shared_secret(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090271_wc_ecc_curve_load(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090272_wc_ecc_curve_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090273_wc_ecc_gen_k(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090274_ecc_make_pub_ex(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090275_ecc_make_key_ex(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090276_wc_ecc_init_ex(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090277_wc_ecc_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090278_wc_ecc_sign_hash(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_090279_wc_ecc_verify_hash(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_090280_wc_ecc_export_x963(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090281_wc_ecc_import_x963_ex(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090282_wc_ecc_size(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090283_wc_ecc_set_rng(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090284_wc_ecc_get_oid(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090285_wc_ed25519_sign_msg_ex(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_090286_wc_ed25519_verify_msg_ex(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_090287_wc_ed25519_init_ex(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090288_wc_ed25519_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090289_wc_ed25519_import_public(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090290_load_3(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090291_fe_1(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090292_curve25519(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090293_fe_frombytes(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090294_fe_mul(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090295_fe_sq(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090296_fe_invert(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090297_fe_tobytes(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090298_fe_copy(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090299_fe_sub(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090300_fe_add(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090301_fe_sq2(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090302_fe_neg(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090303_fe_isnonzero(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090304_fe_isnegative(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090305_fe_cmov(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090306_sc_reduce(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090307_ge_compress_key(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090308_ge_p3_tobytes(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090309_ge_select(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_090310_ge_p3_dbl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090311_slide(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090312_wc_hashgetdigestsize(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090313_wc_hash(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i32) -> i32; }
extern "C" { pub fn f_090314_wc_shahash(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090315_wc_md5hash(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090316_wc_hashupdate(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090317_wc_hmacsetkey(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090318_wc_hmacfree(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090319_wc_hmacupdate(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090320_hmackeyinnerhash(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090321_wc_hmacfinal(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090322_wc_hmacinit(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090323_wc_initmd5_ex(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090324_wc_md5update(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090325_transform(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090326_wc_md5final(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090327_wc_initmd5(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090328_wc_md5gethash(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090329_wc_md5copy(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090330_hash_drbg_reseed(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090331_hash_df(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_090332_initrng(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090333_wc_freerng(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090335_wc_rng_generateblock(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090336_hash_drbg_generate(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090337_wc_rng_healthtest_ex(ph: *mut Phantom, a0: i32, a1: i64, a2: i32, a3: i64, a4: i32, a5: i64, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_090338_wc_initrsakey_ex(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090339_wc_freersakey(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090340_rsapublicencryptex(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_090341_rsaprivatedecryptex(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64, a6: i32, a7: i32, a8: i64) -> i32; }
extern "C" { pub fn f_090342_wc_rsafunction(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64, a4: i32, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_090343_wc_rsassl_verifyinline(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090344_wc_rsassl_sign(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_090345_wc_rsaencryptsize(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090346_wc_initsha_ex(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090347_wc_shaupdate(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090348_transform_148397(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090349_wc_shafinal(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090350_wc_initsha(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090351_wc_shagethash(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090352_wc_initsha256_ex(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090353_wc_sha256update(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090354_transform_sha256(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090355_wc_sha256final(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090356_wc_initsha256(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090357_wc_sha256gethash(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090358_wc_initsha512_ex(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090359_wc_sha512update(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090360_transform_sha512(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090361_wc_sha512final(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090362_sha512final(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090363_wc_initsha512(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090364_wc_sha384final(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090365_wc_initsha384_ex(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090366_wc_initsha384(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090367_wc_sha512_224final(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090368_wc_sha512_256final(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090369_wc_sha384gethash(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090370_sp_2048_from_mp(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_090371_sp_2048_sqr_72(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090372_sp_2048_mod_72(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090373_sp_2048_mul_72(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090374_sp_2048_mont_norm_72(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090375_sp_2048_mont_reduce_72(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090376_sp_2048_cmp_72(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090377_sp_2048_cond_sub_72(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_090378_sp_2048_to_bin_72(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090379_sp_2048_add_36(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090380_sp_2048_norm_36(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090381_sp_2048_sqr_36(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090382_sp_2048_sub_72(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090383_sp_2048_add_72(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090384_sp_2048_norm_144(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090385_sp_2048_mul_d_72(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090386_sp_2048_norm_71(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090387_sp_2048_mul_36(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090388_sp_2048_norm_72(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090389_sp_2048_mul_add_72(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090390_sp_2048_mod_exp_36(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_090391_sp_2048_sub_36(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090392_sp_2048_cond_add_36(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_090393_sp_2048_mod_36(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090394_sp_2048_mont_reduce_36(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090395_sp_2048_cond_sub_36(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_090396_sp_2048_add_12(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090397_sp_2048_mul_12(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090398_sp_2048_sub_24(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090399_sp_2048_add_24(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090400_sp_2048_mul_d_36(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090401_sp_2048_mod_exp_72(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_090402_sp_dhexp_2048(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_090403_sp_3072_from_mp(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_090404_sp_3072_sqr_112(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090405_sp_3072_mod_112(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090406_sp_3072_mul_112(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090407_sp_3072_mont_norm_112(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090408_sp_3072_mont_reduce_112(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090409_sp_3072_cmp_112(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090410_sp_3072_cond_sub_112(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_090411_sp_3072_to_bin_112(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090412_sp_3072_add_56(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090413_sp_3072_norm_56(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090414_sp_3072_sqr_56(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090415_sp_3072_sub_112(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090416_sp_3072_add_112(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090417_sp_3072_norm_224(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090418_sp_3072_mul_d_112(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090419_sp_3072_norm_110(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090420_sp_3072_mul_56(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090421_sp_3072_norm_112(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090422_sp_3072_mul_add_112(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090423_sp_3072_mod_exp_56(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32) -> i32; }
extern "C" { pub fn f_090424_sp_3072_sub_56(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090425_sp_3072_norm_55(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090426_sp_3072_cond_add_56(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_090427_sp_3072_mod_56(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090428_sp_3072_mont_reduce_56(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090429_sp_3072_cond_sub_56(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_090430_sp_3072_add_28(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090431_sp_3072_norm_28(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090432_sp_3072_mul_28(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090433_sp_3072_mul_d_56(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090434_sp_3072_mod_exp_112(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_090435_sp_dhexp_3072(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_090436_sp_4096_from_mp(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_090437_sp_4096_sqr_162(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090438_sp_4096_mod_162(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090439_sp_4096_mul_162(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090440_sp_4096_sub_162(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090441_sp_4096_mont_reduce_162(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090442_sp_4096_cmp_162(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090443_sp_4096_cond_sub_162(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_090444_sp_4096_to_bin_162(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090445_sp_4096_add_81(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090446_sp_4096_sqr_81(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090447_sp_4096_add_162(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090448_sp_4096_norm_324(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090449_sp_4096_mul_d_162(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090450_sp_4096_norm_158(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090451_sp_4096_mul_81(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090452_sp_4096_norm_162(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090453_sp_4096_mul_add_162(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090454_sp_4096_mod_exp_81(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090455_sp_4096_sub_81(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090456_sp_4096_norm_79(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090457_sp_4096_cond_add_81(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_090458_sp_4096_mod_81(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090459_sp_4096_mont_reduce_81(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090460_sp_4096_cond_sub_81(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_090461_sp_4096_add_27(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090462_sp_4096_norm_27(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090463_sp_4096_mul_27(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090464_sp_4096_sub_54(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090465_sp_4096_norm_54(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090466_sp_4096_add_54(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090467_sp_4096_mul_d_81(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090468_sp_4096_mod_exp_162(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_090469_sp_dhexp_4096(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_090470_sp_256_from_mp(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090471_sp_256_ecc_mulmod_9(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_090472_sp_256_to_mp(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090473_sp_256_mod_mul_norm_9(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090474_sp_256_mont_dbl_9(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090475_sp_256_sqr_9(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090476_sp_256_mont_reduce_9(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090477_sp_256_mont_sub_9(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090478_sp_256_mont_tpl_9(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090479_sp_256_mul_9(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090480_sp_256_rshift1_9(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090481_sp_256_proj_point_add_9(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_090482_sp_256_proj_point_dbl_9(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090483_sp_256_proj_point_add_sub_9(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_090484_sp_256_get_point_33_9(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090485_sp_256_sub_9(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090486_sp_256_map_9(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090487_sp_256_ecc_mulmod_base_9(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_090488_sp_256_get_entry_256_9(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_090489_sp_256_ecc_gen_k_9(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090490_sp_256_mod_9(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090491_sp_256_mont_reduce_order_9(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090492_sp_256_add_9(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090493_sp_256_mont_add_9(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090494_sp_384_from_mp(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090495_sp_384_ecc_mulmod_15(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_090496_sp_384_to_mp(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090497_sp_384_mod_mul_norm_15(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090498_sp_384_mont_dbl_15(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090499_sp_384_sqr_15(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090500_sp_384_mont_reduce_15(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090501_sp_384_mont_sub_15(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090502_sp_384_mont_tpl_15(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090503_sp_384_mul_15(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090504_sp_384_div2_15(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090505_sp_384_proj_point_add_15(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_090506_sp_384_proj_point_dbl_15(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090507_sp_384_proj_point_add_sub_15(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_090508_sp_384_get_point_33_15(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090509_sp_384_sub_15(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090510_sp_384_map_15(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090511_sp_384_ecc_mulmod_base_15(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_090512_sp_384_get_entry_256_15(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_090513_sp_384_ecc_gen_k_15(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090514_sp_384_mul_15_specialized_1(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090515_sp_384_mod_15(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090516_sp_384_mont_reduce_order_15(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090517_sp_384_add_15(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090518_sp_384_rshift1_15(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090519_sp_384_mont_add_15(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090520_sp_2048_sqr_12(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090521_sp_2048_mul_add_36(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090522_sp_3072_sqr_28(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090523_sp_3072_add_14(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090524_sp_3072_sqr_14(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090525_sp_3072_sub_28(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090526_sp_3072_mul_add_56(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090527_sp_3072_mul_14(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090528_sp_4096_sqr_27(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090529_sp_4096_sqr_9(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090530_sp_4096_sub_18(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090531_sp_4096_add_18(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090532_sp_4096_mul_add_81(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090533_sp_4096_mul_9(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090534_sp_256_mul_d_9(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090535_sp_256_mul_add_9(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_090536_sp_384_mul_d_15(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090537_sp_384_mul_add_15(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_090538_fp_add(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090539_s_fp_add(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090540_s_fp_sub(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090541_fp_cmp_mag(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090542_fp_sub(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090543_fp_mul(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090544_fp_mul_comba(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090545_fp_mul_d(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090546_fp_mul_2d(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090547_fp_div_2d(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_090548_fp_cmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090549_fp_rshb(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_090550_fp_mod(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090551_fp_invmod(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090552_fp_sqr_comba(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090553_fp_montgomery_reduce_ex(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090554_fp_sqr(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090555_fp_sub_d(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090556_fp_exptmod(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090557_fp_exptmod_ct(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_090558_fp_montgomery_calc_normalization(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090559_fp_exptmod_nct(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090560_mp_init(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090561_mp_forcezero(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090562_mp_clear(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090563_mp_init_multi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_090564_mp_mulmod(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090565_mp_submod_ct(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090566_mp_addmod_ct(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090567_mp_cmp_d(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090568_mp_unsigned_bin_size(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090569_mp_to_unsigned_bin(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090570_mp_read_unsigned_bin(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090571_mp_copy(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090572_mp_iszero(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090573_mp_count_bits(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090574_mp_leading_bit(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090575_fp_prime_miller_rabin_ex(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i32; }
extern "C" { pub fn f_090576_mp_cond_swap_ct(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_090577_fp_add_d(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090578_mp_read_radix(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090579_mp_montgomery_setup(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090580_mp_div_2_mod_ct(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090581_mp_set(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090582_wc_initmutex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090583_wc_freemutex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090584_wc_lockmutex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090585_wc_unlockmutex(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090586_mystrnstr(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_090587_get_digit_count(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090588_mp_cond_copy(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090589_setcipherspecs(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090590_setkeysside(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090591_storekeys(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090592_derivekeys(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090593_makemastersecret(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090594_istls(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090595_isatleasttlsv1_2(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090596_isatleasttlsv1_3(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090597_isdtlsnotsctpmode(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090598_initssl_method(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090599_ssl_ctxresourcefree(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090600_freessl_ctx(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090601_initsuites(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32); }
extern "C" { pub fn f_090602_verifyrsasign(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_090603_ed25519checkpubkey(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090604_inithandshakehashes(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090605_freehandshakehashes(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090607_writeseq(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_090608_freekey(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_090609_allockey(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090610_freekeyexchange(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090611_shrinkinputbuffer(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090612_freehandshakeresources(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090613_freessl(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090614_dtlstxmsglistclean(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090615_dtlsmsgset(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_090616_dtlsmsgstore(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32, a8: i64); }
extern "C" { pub fn f_090617_dtlsmsgpoolsave(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_090618_dtlsmsgpoolsend(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090619_checkavailablesize(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090620_buildmessage(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i32, a5: i32, a6: i32, a7: i32) -> i32; }
extern "C" { pub fn f_090621_sendbuffered(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090622_hashraw(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090623_lowrestimer(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090624_hashoutput(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090625_hashinput(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090626_matchdomainname(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090627_sendalert(ph: *mut Phantom, a0: i64, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_090628_doverifycallback(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i32; }
extern "C" { pub fn f_090629_processpeercertparse(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_090630_reusekey(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090631_processreplyex(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090632_getinputdata(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090633_dohandshakemsgtype(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32, a5: i32) -> i32; }
extern "C" { pub fn f_090634_sanitycheckmsgreceived(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090635_buildfinished(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090636_sendchangecipher(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090637_buildcerthashes(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090638_sendfinished(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090639_sendcertificate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090640_addfragheaders(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32, a5: i64); }
extern "C" { pub fn f_090641_addheaders(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i64); }
extern "C" { pub fn f_090642_pickhashsigalgo(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090643_decodeprivatekey(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090644_sendclienthello(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090645_cipherrequires(ph: *mut Phantom, a0: i32, a1: i32, a2: i32) -> i32; }
extern "C" { pub fn f_090646_sendhandshakemsg(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_090647_matchsuite(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090648_comparesuites(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_090650_wolfssl_init(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090651_wolfssl_cleanup(ph: *mut Phantom); }
extern "C" { pub fn f_090652_wolfssl_ctx_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090655_wolfssl_negotiate(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090656_wolfssl_connect(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090662_wolfssl_getdtlsmacsecret(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090663_wolfssl_getmacsecret(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090664_wolfssl_certmanagerfree(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090666_getca(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090667_addca(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32) -> i32; }
extern "C" { pub fn f_090670_wolfssl_dtls_get_using_nonblock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090673_wolfssl_unloadcertskeys(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090674_wc_prf(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32, a6: i32, a7: i64) -> i32; }
extern "C" { pub fn f_090675_wc_prf_tls(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i32, a4: i64, a5: i32, a6: i64, a7: i32, a8: i32, a9: i32, a10: i64) -> i32; }
extern "C" { pub fn f_090676_buildtlshandshakehash(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090677_derivetlskeys(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090679_tlsx_usesupportedcurve(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090680_tlsx_push(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090681_tlsx_usepointformat(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090682_tlsx_freeall(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090683_tlsx_supportextensions(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090684_tlsx_populateextensions(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090685_tlsx_getsize(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64); }
extern "C" { pub fn f_090686_tlsx_write(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_090687_tlsx_parse(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i64) -> i32; }
extern "C" { pub fn f_090695_errno_location(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_090696_lock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090697_futexwait(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_090698_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090699_wake(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090700_tzset(ph: *mut Phantom); }
extern "C" { pub fn f_090701_do_tzset(ph: *mut Phantom); }
extern "C" { pub fn f_090702_wait(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090703_a_inc(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090704_a_dec(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090705_abort(ph: *mut Phantom); }
extern "C" { pub fn f_090708_r_148942(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090711_atan(ph: *mut Phantom, a0: f64) -> f64; }
extern "C" { pub fn f_090714_atanf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090719_isspace(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090722_close(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090724_cos(ph: *mut Phantom, a0: f64, a1: f64) -> f64; }
extern "C" { pub fn f_090725_rem_pio2_large(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32, a4: i32) -> i32; }
extern "C" { pub fn f_090726_rem_pio2(ph: *mut Phantom, a0: f64, a1: i64) -> i32; }
extern "C" { pub fn f_090727_sin(ph: *mut Phantom, a0: f64, a1: f64, a2: i32) -> f64; }
extern "C" { pub fn f_090729_cosdf(ph: *mut Phantom, a0: f64) -> f32; }
extern "C" { pub fn f_090730_sindf(ph: *mut Phantom, a0: f64) -> f32; }
extern "C" { pub fn f_090731_rem_pio2f(ph: *mut Phantom, a0: f32, a1: i64) -> i32; }
extern "C" { pub fn f_090733_expo2f(ph: *mut Phantom, a0: f32, a1: f32) -> f32; }
extern "C" { pub fn f_090735_do_futex_wait(ph: *mut Phantom, a0: i64, a1: i32, a2: f64) -> i32; }
extern "C" { pub fn f_090736_a_cas_p(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090737_emscripten_futex_wake(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_090738_emscripten_thread_notify(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090739_emscripten_get_heap_size(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_090740_set_thread_state(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32); }
extern "C" { pub fn f_090741_time(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090743_emscripten_yield(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090744_math_xflow(ph: *mut Phantom, a0: i32, a1: f64) -> f64; }
extern "C" { pub fn f_090745_fp_barrier(ph: *mut Phantom, a0: f64) -> f64; }
extern "C" { pub fn f_090746_math_uflow(ph: *mut Phantom, a0: i32) -> f64; }
extern "C" { pub fn f_090747_math_oflow(ph: *mut Phantom, a0: i32) -> f64; }
extern "C" { pub fn f_090748_exp(ph: *mut Phantom, a0: f64) -> f64; }
extern "C" { pub fn f_090749_top12(ph: *mut Phantom, a0: f64) -> i32; }
extern "C" { pub fn f_090750_fp_force_eval(ph: *mut Phantom, a0: f64); }
extern "C" { pub fn f_090751_math_xflowf(ph: *mut Phantom, a0: i32, a1: f32) -> f32; }
extern "C" { pub fn f_090752_fp_barrierf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090753_math_oflowf(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_090754_math_uflowf(ph: *mut Phantom, a0: i32) -> f32; }
extern "C" { pub fn f_090756_top12_149013(ph: *mut Phantom, a0: f32) -> i32; }
extern "C" { pub fn f_090757_expf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090758_expm1f(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090759_lockfile(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090760_unlockfile(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090761_a_swap(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090764_fflush(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090765_toread(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090766_uflow(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090770_memset(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_090778_towrite(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090779_overflow(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090780_fputs(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090781_memcpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090783_fseeko_unlocked(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_090786_fstatat(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32) -> i32; }
extern "C" { pub fn f_090787_ftello_unlocked(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090789_fwritex(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090790_fwrite(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_090794_bswap_16(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090800_emscripten_thread_is_valid(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090802_math_invalid(ph: *mut Phantom, a0: f64) -> f64; }
extern "C" { pub fn f_090803_log(ph: *mut Phantom, a0: f64) -> f64; }
extern "C" { pub fn f_090806_math_invalidf(ph: *mut Phantom, a0: f32) -> f32; }
extern "C" { pub fn f_090810_lseek(ph: *mut Phantom, a0: i32, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_090811_mbrtowc(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090813_memchr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_090814_memcmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090816_localtime_r(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090817_gmtime_r(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090820_ofl_lock(ph: *mut Phantom) -> i64; }
extern "C" { pub fn f_090821_open(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i32; }
extern "C" { pub fn f_090823_pow(ph: *mut Phantom, a0: f64, a1: f64) -> f64; }
extern "C" { pub fn f_090824_zeroinfnan(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090825_checkint(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090827_zeroinfnan_149149(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090828_checkint_149150(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090831_em_task_queue_create(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090832_em_task_queue_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090833_em_task_queue_execute(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090834_em_task_queue_is_empty(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090835_em_task_queue_dequeue(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090836_em_task_queue_cancel(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090837_em_task_queue_enqueue(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090840_emscripten_proxy_execute_queue(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090841_get_tasks_for_thread(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090842_do_proxy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090843_get_or_add_tasks_for_thread(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090844_emscripten_proxy_finish(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090845_remove_active_ctx(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090847_free_ctx(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090848_em_proxying_ctx_deinit(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090849_emscripten_proxy_sync_with_ctx(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090851_cancel_ctx(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090852_init_active_ctxs(ph: *mut Phantom); }
extern "C" { pub fn f_090862_pthread_cond_broadcast(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090863_wake_149203(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090864_pthread_cond_destroy(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090865_pthread_cond_signal(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090866_timedwait_cp(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090867_timedwait(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090868_pthread_cond_timedwait(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090869_lock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090870_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090871_a_fetch_add_149218(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090872_a_store(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090873_private_cond_signal(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_090874_vm_lock(ph: *mut Phantom); }
extern "C" { pub fn f_090875_vm_unlock(ph: *mut Phantom); }
extern "C" { pub fn f_090876_tl_lock(ph: *mut Phantom); }
extern "C" { pub fn f_090877_tl_unlock(ph: *mut Phantom); }
extern "C" { pub fn f_090878_a_barrier(ph: *mut Phantom); }
extern "C" { pub fn f_090880_init_file_lock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090883_pthread_getspecific(ph: *mut Phantom, a0: i32) -> i64; }
extern "C" { pub fn f_090885_pthread_mutex_destroy(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090886_pthread_mutex_init(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090887_pthread_mutex_lock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090888_pthread_mutex_trylock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090889_pthread_mutex_unlock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090892_pthread_rwlock_timedrdlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090893_a_cas_149288(ph: *mut Phantom, a0: i64, a1: i32, a2: i32); }
extern "C" { pub fn f_090894_pthread_rwlock_tryrdlock(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090895_pthread_rwlock_unlock(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090896_pthread_rwlock_trywrlock(ph: *mut Phantom) -> i32; }
extern "C" { pub fn f_090898_pthread_setcancelstate(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_090899_pthread_setspecific(ph: *mut Phantom, a0: i32, a1: i64); }
extern "C" { pub fn f_090902_sift(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i64); }
extern "C" { pub fn f_090903_shr(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_090904_trinkle(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32, a6: i64); }
extern "C" { pub fn f_090905_shl(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_090906_pntz(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090907_cycle(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_090908_builtin_ctz(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090909_wrapper_cmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090912_read(ph: *mut Phantom, a0: i32, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090915_scalbn(ph: *mut Phantom, a0: f64, a1: i32) -> f64; }
extern "C" { pub fn f_090920_snprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090928_strchr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090929_strchrnul(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090930_strcmp(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_090931_strcpy(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090932_strcspn(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090934_nl_langinfo_l(ph: *mut Phantom, a0: i32, a1: i64) -> i64; }
extern "C" { pub fn f_090935_week_num(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090936_strftime_l(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64; }
extern "C" { pub fn f_090937_is_leap(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090939_strlen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090942_strncmp(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090943_strncpy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_090944_strnlen(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090946_strspn(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090948_shlim(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_090949_shgetc(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090950_copysignl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_090951_scalbnl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_090952_fmodl(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_090953_floatscan(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i32); }
extern "C" { pub fn f_090954_scanexp(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090955_strtod(ph: *mut Phantom, a0: i64, a1: i64) -> f64; }
extern "C" { pub fn f_090957_strtoull(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i64; }
extern "C" { pub fn f_090958_strtox_149401(ph: *mut Phantom, a0: i64, a1: i64, a2: i32, a3: i64) -> i64; }
extern "C" { pub fn f_090960_syscall_ret(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090964_tandf(ph: *mut Phantom, a0: f64, a1: i32) -> f32; }
extern "C" { pub fn f_090967_emscripten_thread_mailbox_unref(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090968_emscripten_thread_mailbox_init(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_090970_tolower(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090972_frexp(ph: *mut Phantom, a0: f64, a1: i64) -> f64; }
extern "C" { pub fn f_090973_vfprintf_internal(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32; }
extern "C" { pub fn f_090974_printf_core(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i32; }
extern "C" { pub fn f_090975_out(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_090976_getint(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_090977_pop_arg(ph: *mut Phantom, a0: i64, a1: i32, a2: i64, a3: i64); }
extern "C" { pub fn f_090978_fmt_u(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_090979_pad(ph: *mut Phantom, a0: i64, a1: i32, a2: i32, a3: i32, a4: i32); }
extern "C" { pub fn f_090980_vfprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090983_vfiprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090984_small_vfprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_090985_vsnprintf(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_090988_store_int(ph: *mut Phantom, a0: i64, a1: i32, a2: i64); }
extern "C" { pub fn f_090990_wasi_syscall_ret(ph: *mut Phantom, a0: i32) -> i32; }
extern "C" { pub fn f_090991_wcslen(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_090993_wcschr(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090995_wcrtomb(ph: *mut Phantom, a0: i64, a1: i32) -> i64; }
extern "C" { pub fn f_090997_wctomb(ph: *mut Phantom, a0: i64, a1: i32) -> i32; }
extern "C" { pub fn f_090998_wmemchr(ph: *mut Phantom, a0: i64, a1: i32, a2: i64) -> i64; }
extern "C" { pub fn f_091000_emscripten_builtin_malloc(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_091001_init_mparams(ph: *mut Phantom); }
extern "C" { pub fn f_091002_emscripten_builtin_free(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091003_emscripten_builtin_calloc(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_091004_emscripten_builtin_realloc(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_091005_internal_memalign(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_091007_dispose_chunk(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_091008_sbrk(ph: *mut Phantom, a0: i64) -> i64; }
extern "C" { pub fn f_091009_addtf3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_091010_ashlti3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_091011_letf2(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i32; }
extern "C" { pub fn f_091012_getf2(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i32; }
extern "C" { pub fn f_091013_divtf3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_091014_setthrew(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_091018_extenddftf2(ph: *mut Phantom, a0: i64, a1: f64); }
extern "C" { pub fn f_091019_floatsitf(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_091020_floatunsitf(ph: *mut Phantom, a0: i64, a1: i32); }
extern "C" { pub fn f_091021_lshrti3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_091022_multf3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_091023_multi3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_091027_subtf3(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_091028_trunctfdf2(ph: *mut Phantom, a0: i64, a1: i64) -> f64; }
extern "C" { pub fn f_091030_void_std_2_introsort_std_2_classicalgpolicy_std_2_ranges_less_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_091031_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_std_2_rang(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_091032_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_std_2_rang(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_091033_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_std_2_rang(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_091034_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_091036_void_std_2_introsort_std_2_classicalgpolicy_std_2_ranges_less_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_091037_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_std_2_rang(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_091038_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_std_2_rang(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_091039_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_std_2_rang(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_091040_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_091041_void_std_2_introsort_std_2_classicalgpolicy_std_2_ranges_less_un(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_091042_bool_std_2_sort3_abi_nqn220108_std_2_classicalgpolicy_std_2_rang(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_091043_void_std_2_sort4_abi_nqn220108_std_2_classicalgpolicy_std_2_rang(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_091044_void_std_2_sort5_abi_nqn220108_std_2_classicalgpolicy_std_2_rang(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64); }
extern "C" { pub fn f_091045_bool_std_2_insertion_sort_incomplete_abi_nqn220108_std_2_classic(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_091046_bool_std_2_partially_sorted_swap_abi_nqn220108_std_2_ranges_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_091047_void_std_2_swap_bitmap_pos_abi_nqn220108_std_2_classicalgpolicy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_091048_void_std_2_sift_up_abi_nqn220108_std_2_classicalgpolicy_std_2_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_091049_bool_std_2_partially_sorted_swap_abi_nqn220108_std_2_ranges_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_091050_void_std_2_swap_bitmap_pos_abi_nqn220108_std_2_classicalgpolicy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_091051_void_std_2_sift_up_abi_nqn220108_std_2_classicalgpolicy_std_2_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_091052_bool_std_2_partially_sorted_swap_abi_nqn220108_std_2_ranges_less(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_091053_void_std_2_swap_bitmap_pos_abi_nqn220108_std_2_classicalgpolicy(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_091054_void_std_2_sift_up_abi_nqn220108_std_2_classicalgpolicy_std_2_ra(ph: *mut Phantom, a0: i64, a1: i64, a2: i64); }
extern "C" { pub fn f_091057_std_2_murmur2_or_cityhash_unsigned_long_64ul_hash_len_16_abi_nqn(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_091058_std_2_murmur2_or_cityhash_unsigned_long_64ul_weak_hash_len_32_wi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64); }
extern "C" { pub fn f_091060_unsigned_int_const_ptr_std_2_lower_bound_abi_nqn220108_unsigned(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_091067_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_091068_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom, a0: i64, a1: i64); }
extern "C" { pub fn f_091070_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64) -> i64; }
extern "C" { pub fn f_091071_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64, a7: i64); }
extern "C" { pub fn f_091072_std_2_basic_string_char_std_2_char_traits_char_std_2_allocator_c(ph: *mut Phantom, a0: i64, a1: i64) -> i64; }
extern "C" { pub fn f_091076_cxa_guard_acquire(ph: *mut Phantom, a0: i64) -> i32; }
extern "C" { pub fn f_091077_cxa_guard_release(ph: *mut Phantom, a0: i64); }
extern "C" { pub fn f_091080_is_equal_std_type_info_const_ptr_std_type_info_const_ptr_bool(ph: *mut Phantom, a0: i64, a1: i64, a2: i32) -> i32; }
extern "C" { pub fn f_091082_dynamic_cast(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64) -> i64; }
extern "C" { pub fn f_091083_cxxabiv1_class_type_info_process_found_base_class_cxxabiv1_dynam(ph: *mut Phantom, a0: i64, a1: i64, a2: i32); }
extern "C" { pub fn f_091086_cxxabiv1_base_class_type_info_has_unambiguous_public_base_cxxabi(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_091089_cxxabiv1_pointer_to_member_type_info_can_catch_nested_cxxabiv1_s(ph: *mut Phantom, a0: i64, a1: i64) -> i32; }
extern "C" { pub fn f_091090_cxxabiv1_class_type_info_process_static_type_above_dst_cxxabiv1(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32); }
extern "C" { pub fn f_091092_cxxabiv1_base_class_type_info_search_above_dst_cxxabiv1_dynamic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i64, a4: i32, a5: i32); }
extern "C" { pub fn f_091093_cxxabiv1_base_class_type_info_search_below_dst_cxxabiv1_dynamic(ph: *mut Phantom, a0: i64, a1: i64, a2: i64, a3: i32, a4: i32); }
extern "C" { pub fn f_091100_recv(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_091101_recvfrom(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i64) -> i64; }
extern "C" { pub fn f_091102_send(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32) -> i64; }
extern "C" { pub fn f_091103_sendto(ph: *mut Phantom, a0: i32, a1: i64, a2: i64, a3: i32, a4: i64, a5: i32) -> i64; }
extern "C" { pub fn f_091104_setsockopt(ph: *mut Phantom, a0: i32, a1: i32, a2: i32, a3: i64, a4: i32) -> i32; }
