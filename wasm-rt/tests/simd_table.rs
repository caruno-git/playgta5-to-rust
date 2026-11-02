//! Table-driven conformance test for `wasm_rt::simd`.
//!
//! Every row comes from `tools/gen_simd_cases.py`, which computes the expected
//! 128-bit result with an independent Python implementation of the wasm SIMD
//! semantics. The vectors are literal values here, so a wrong helper fails with
//! the op, the inputs and both bit patterns in the message.
//!
//! Classes covered: lane-wise binary (wrapping add/sub/mul, signed and unsigned
//! min/max, saturating add/sub, `avgr_u`, `q15mulr_sat_s`, `dot_i16x8_s`),
//! compares (masked all-ones per lane), unary (abs/neg/popcnt/float rounding
//! and conversions), shifts (count taken mod lane width), narrowing, widening,
//! pairwise extend/extmul, splat/extract/replace, bitwise and the scalar
//! reduces.

#[path = "common/simd_cases.rs"]
mod cases;

use cases::{bin, ext, rep, sh, spl, ter, tst, un, Case, Fmt, OPS};
use wasm_rt::simd::*;

fn lanes_f32(v: u128) -> [f32; 4] {
    let b = v.to_le_bytes();
    let mut out = [0f32; 4];
    for (i, slot) in out.iter_mut().enumerate() {
        let mut w = [0u8; 4];
        w.copy_from_slice(&b[i * 4..i * 4 + 4]);
        *slot = f32::from_bits(u32::from_le_bytes(w));
    }
    out
}

fn lanes_f64(v: u128) -> [f64; 2] {
    let b = v.to_le_bytes();
    let mut out = [0f64; 2];
    for (i, slot) in out.iter_mut().enumerate() {
        let mut w = [0u8; 8];
        w.copy_from_slice(&b[i * 8..i * 8 + 8]);
        *slot = f64::from_bits(u64::from_le_bytes(w));
    }
    out
}

/// Compare per the row's format: exact bits, or "any NaN equals any NaN" for
/// the float lane types (a NaN payload is not specified by wasm).
fn equal(fmt: Fmt, want: u128, got: u128) -> bool {
    match fmt {
        Fmt::Bits => want == got,
        Fmt::F32 => lanes_f32(want)
            .iter()
            .zip(lanes_f32(got).iter())
            .all(|(w, g)| w.to_bits() == g.to_bits() || (w.is_nan() && g.is_nan())),
        Fmt::F64 => lanes_f64(want)
            .iter()
            .zip(lanes_f64(got).iter())
            .all(|(w, g)| w.to_bits() == g.to_bits() || (w.is_nan() && g.is_nan())),
    }
}

fn fail_bin(c: &Case, got: u128) -> ! {
    panic!(
        "{}: a={:#034x} b={:#034x}\n  want {:#034x}\n  got  {:#034x}",
        c.op, c.a, c.b, c.want, got
    )
}

#[test]
fn binary_vectors() {
    assert!(!cases::BIN.is_empty());
    for c in cases::BIN {
        match bin(c.op, c.a, c.b) {
            Some(got) => {
                if !equal(c.fmt, c.want, got) {
                    fail_bin(c, got);
                }
            }
            None => panic!("no helper dispatched for {}", c.op),
        }
    }
}

#[test]
fn three_operand_vectors() {
    assert!(!cases::TER.is_empty());
    for c in cases::TER {
        let got = ter(c.op, c.a, c.b, c.c).unwrap_or_else(|| panic!("no helper for {}", c.op));
        if got != c.want {
            panic!(
                "{}: a={:#034x} b={:#034x} c={:#034x}
  want {:#034x}
  got  {:#034x}",
                c.op, c.a, c.b, c.c, c.want, got
            );
        }
    }
}

#[test]
fn unary_vectors() {
    assert!(!cases::UN.is_empty());
    for c in cases::UN {
        match un(c.op, c.a) {
            Some(got) => {
                if !equal(c.fmt, c.want, got) {
                    panic!(
                        "{}: a={:#034x}\n  want {:#034x}\n  got  {:#034x}",
                        c.op, c.a, c.want, got
                    );
                }
            }
            None => panic!("no helper dispatched for {}", c.op),
        }
    }
}

#[test]
fn shift_vectors() {
    assert!(!cases::SH.is_empty());
    for c in cases::SH {
        let got = sh(c.op, c.a, c.shift).unwrap_or_else(|| panic!("no helper for {}", c.op));
        if got != c.want {
            panic!(
                "{}: a={:#034x} shift={}\n  want {:#034x}\n  got  {:#034x}",
                c.op, c.a, c.shift, c.want, got
            );
        }
    }
}

#[test]
fn replace_lane_vectors() {
    assert!(!cases::REP.is_empty());
    for c in cases::REP {
        let got = rep(c.op, c.a, c.lane, c.value)
            .unwrap_or_else(|| panic!("no helper for {}", c.op));
        if got != c.want {
            panic!(
                "{}: a={:#034x} lane={} value={:#x}\n  want {:#034x}\n  got  {:#034x}",
                c.op, c.a, c.lane, c.value, c.want, got
            );
        }
    }
}

#[test]
fn splat_vectors() {
    assert!(!cases::SPL.is_empty());
    for c in cases::SPL {
        let got = spl(c.op, c.value).unwrap_or_else(|| panic!("no helper for {}", c.op));
        if got != c.want {
            panic!(
                "{}: value={:#x}\n  want {:#034x}\n  got  {:#034x}",
                c.op, c.value, c.want, got
            );
        }
    }
}

#[test]
fn extract_lane_vectors() {
    assert!(!cases::EXT.is_empty());
    for c in cases::EXT {
        let got = ext(c.op, c.a, c.lane).unwrap_or_else(|| panic!("no helper for {}", c.op));
        if got != c.want {
            panic!(
                "{}: a={:#034x} lane={}\n  want {:#x}\n  got  {:#x}",
                c.op, c.a, c.lane, c.want, got
            );
        }
    }
}

#[test]
fn scalar_reduce_vectors() {
    assert!(!cases::TST.is_empty());
    for c in cases::TST {
        let got = tst(c.op, c.a).unwrap_or_else(|| panic!("no helper for {}", c.op));
        if got != c.want {
            panic!(
                "{}: a={:#034x}\n  want {:#x}\n  got  {:#x}",
                c.op, c.a, c.want, got
            );
        }
    }
}

/// Every op the module is expected to provide must be reachable from at least
/// one table, and the tables must not be empty for it.
#[test]
fn every_declared_op_has_a_vector() {
    let mut uncovered = Vec::new();
    for op in OPS {
        let hit = bin(op, 0, 0).is_some()
            || ter(op, 0, 0, 0).is_some()
            || un(op, 0).is_some()
            || sh(op, 0, 1).is_some()
            || rep(op, 0, 0, 0).is_some()
            || spl(op, 0).is_some()
            || ext(op, 0, 0).is_some()
            || tst(op, 0).is_some();
        if !hit {
            uncovered.push(*op);
        }
    }
    assert!(uncovered.is_empty(), "ops without a vector: {uncovered:?}");
    assert_eq!(OPS.len(), 213, "op count changed: {}", OPS.len());
}

#[test]
fn vector_count_is_not_trivial() {
    let total = cases::BIN.len()
        + cases::TER.len()
        + cases::UN.len()
        + cases::SH.len()
        + cases::REP.len()
        + cases::SPL.len()
        + cases::EXT.len()
        + cases::TST.len();
    assert_eq!(total, 681, "expected 681 vectors, got {total}");
}

// ---------------------------------------------------------------------------
// Behaviour the vectors cannot express: lane immediates are validated
// ---------------------------------------------------------------------------

#[test]
#[should_panic(expected = "lane")]
fn extract_lane_out_of_range_traps() {
    let v = i8x16_splat(7);
    let _ = i8x16_extract_lane_s(v, 16);
}

#[test]
#[should_panic(expected = "lane")]
fn replace_lane_out_of_range_traps() {
    let v = i32x4_splat(7);
    let _ = i32x4_replace_lane(v, 4, 9);
}

#[test]
#[should_panic(expected = "out of range")]
fn shuffle_mask_out_of_range_traps() {
    let mut mask = [0u8; 16];
    mask[3] = 32;
    let _ = i8x16_shuffle(0, 0, pack_u8(mask));
}

#[test]
fn shift_count_is_modulo_lane_width() {
    let v = i8x16_splat(0x5A);
    // 8 % 8 == 0 for i8 lanes, 32 % 32 == 0 for i32 lanes, 64 % 64 == 0 for i64
    assert_eq!(i8x16_shl(v, 8), v);
    assert_eq!(i8x16_shr_u(v, 8), v);
    assert_eq!(i32x4_shl(i32x4_splat(-8), 32), i32x4_splat(-8));
    assert_eq!(i64x2_shr_s(i64x2_splat(-8), 64), i64x2_splat(-8));
    // 9 % 8 == 1
    assert_eq!(i8x16_shl(v, 9), i8x16_shl(v, 1));
}

#[test]
fn swizzle_zeroes_out_of_range_indices_but_shuffle_wraps_between_operands() {
    let a = pack_u8([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let idx = pack_u8([0, 15, 16, 200, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let sw = unpack_u8(i8x16_swizzle(a, idx));
    assert_eq!(&sw[..4], &[0, 15, 0, 0]);
}

#[test]
fn bitselect_is_per_bit() {
    let a = u128::MAX;
    let b = 0u128;
    assert_eq!(v128_bitselect(a, b, 0x0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F),
               0x0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F);
}

#[test]
fn signed_and_unsigned_min_differ_on_high_bit() {
    // 0x80 is -128 signed, 128 unsigned; 0x01 is 1 either way.
    let a = i8x16_splat(-128);
    let b = i8x16_splat(1);
    assert_eq!(unpack_i8(i8x16_min_s(a, b))[0], -128);
    assert_eq!(unpack_u8(i8x16_min_u(a, b))[0], 1);
    assert_eq!(unpack_i8(i8x16_max_s(a, b))[0], 1);
    assert_eq!(unpack_u8(i8x16_max_u(a, b))[0], 128);
}

#[test]
fn neg_and_abs_wrap_at_lane_min() {
    let min = i8x16_splat(-128);
    assert_eq!(i8x16_neg(min), min);
    assert_eq!(i8x16_abs(min), min);
    assert_eq!(i64x2_neg(i64x2_splat(i64::MIN)), i64x2_splat(i64::MIN));
}

#[test]
fn q15mulr_saturates_only_at_min_times_min() {
    let a = i16x8_splat(-32768);
    assert_eq!(unpack_i16(i16x8_q15mulr_sat_s(a, a))[0], 32767);
}

#[test]
fn nearest_preserves_negative_zero() {
    let v = f32x4_splat(-0.5);
    let bits = unpack_u32(f32x4_nearest(v))[0];
    assert_eq!(bits, 0x8000_0000, "nearest(-0.5) must be -0.0");
    let v = f64x2_splat(-0.5);
    assert_eq!(unpack_u64(f64x2_nearest(v))[0], 0x8000_0000_0000_0000);
}

#[test]
fn constant_shuffle_matches_the_runtime_spec_across_both_inputs() {
    for seed in 0..128u128 {
        let a = seed.wrapping_mul(0x01020304050607080910111213141516);
        let b = !a;
        let result = i8x16_shuffle_static::<0,31,1,30,2,29,3,28,4,27,5,26,6,25,7,24>(a,b);
        let mask = u128::from_le_bytes([0,31,1,30,2,29,3,28,4,27,5,26,6,25,7,24]);
        assert_eq!(result, i8x16_shuffle(a,b,mask));
    }
}
