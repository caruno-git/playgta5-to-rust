//! 128-bit SIMD helpers for the translated module.
//!
//! GENERATED from the `SIMD` table in `tools/w2r/tables.py` — edit the table,
//! not the generated block. The `ADDITIONS` section at the bottom is hand
//! written and lists the corrections that still have to move into the table.
//!
//! `V` is the translated representation of a wasm `v128`: a `u128` carrying the
//! 16 raw bytes in little-endian lane order, so `transmute::<V, [T; N]>` is the
//! lane view for every lane type.
//!
//! Semantics follow the wasm SIMD spec: integer lane arithmetic is wrapping,
//! `*_sat_*` ops saturate, compares produce all-ones/all-zero masks, shifts take
//! the count modulo the lane width, and lane immediates are validated (`lane <
//! lanes`) rather than masked.

#![allow(clippy::all, dead_code, unknown_lints, unnecessary_transmutes)]
#![allow(clippy::all, dead_code, unknown_lints, unnecessary_transmutes)]

pub type V = u128;
pub use crate::simd_shuffle::i8x16_shuffle_static;


#[inline]
pub fn pack_u8(x: [u8; 16]) -> V { unsafe { core::mem::transmute(x) } }
#[inline]
pub fn pack_i8(x: [i8; 16]) -> V { unsafe { core::mem::transmute(x) } }
#[inline]
pub fn pack_u16(x: [u16; 8]) -> V { unsafe { core::mem::transmute(x) } }
#[inline]
pub fn pack_i16(x: [i16; 8]) -> V { unsafe { core::mem::transmute(x) } }
#[inline]
pub fn pack_u32(x: [u32; 4]) -> V { unsafe { core::mem::transmute(x) } }
#[inline]
pub fn pack_i32(x: [i32; 4]) -> V { unsafe { core::mem::transmute(x) } }
#[inline]
pub fn pack_u64(x: [u64; 2]) -> V { unsafe { core::mem::transmute(x) } }
#[inline]
pub fn pack_i64(x: [i64; 2]) -> V { unsafe { core::mem::transmute(x) } }
#[inline]
pub fn pack_f32(x: [f32; 4]) -> V { unsafe { core::mem::transmute(x) } }
#[inline]
pub fn pack_f64(x: [f64; 2]) -> V { unsafe { core::mem::transmute(x) } }
#[inline]
pub fn pack_u16x8(x: [u16; 8]) -> V { pack_u16(x) }
#[inline]
pub fn pack_i16x8(x: [i16; 8]) -> V { pack_i16(x) }
#[inline]
pub fn pack_u32x4(x: [u32; 4]) -> V { pack_u32(x) }
#[inline]
pub fn pack_i32x4(x: [i32; 4]) -> V { pack_i32(x) }
#[inline]
pub fn pack_u64x2(x: [u64; 2]) -> V { pack_u64(x) }
#[inline]
pub fn pack_i64x2(x: [i64; 2]) -> V { pack_i64(x) }

#[inline]
pub fn unpack_u8(v: V) -> [u8; 16] { unsafe { core::mem::transmute(v) } }
#[inline]
pub fn unpack_i8(v: V) -> [i8; 16] { unsafe { core::mem::transmute(v) } }
#[inline]
pub fn unpack_u16(v: V) -> [u16; 8] { unsafe { core::mem::transmute(v) } }
#[inline]
pub fn unpack_i16(v: V) -> [i16; 8] { unsafe { core::mem::transmute(v) } }
#[inline]
pub fn unpack_u32(v: V) -> [u32; 4] { unsafe { core::mem::transmute(v) } }
#[inline]
pub fn unpack_i32(v: V) -> [i32; 4] { unsafe { core::mem::transmute(v) } }
#[inline]
pub fn unpack_u64(v: V) -> [u64; 2] { unsafe { core::mem::transmute(v) } }
#[inline]
pub fn unpack_i64(v: V) -> [i64; 2] { unsafe { core::mem::transmute(v) } }
#[inline]
pub fn unpack_f32(v: V) -> [f32; 4] { unsafe { core::mem::transmute(v) } }
#[inline]
pub fn unpack_f64(v: V) -> [f64; 2] { unsafe { core::mem::transmute(v) } }

macro_rules! b2 {
    ($name:ident, $t:ty, $k:expr, |$x:ident, $y:ident| $body:expr) => {
        #[inline]
        pub fn $name(a: V, b: V) -> V {
            let va: [$t; $k] = unsafe { core::mem::transmute(a) };
            let vb: [$t; $k] = unsafe { core::mem::transmute(b) };
            let mut r: [$t; $k] = [0 as $t; $k];
            for i in 0..$k {
                let $x: $t = va[i];
                let $y: $t = vb[i];
                r[i] = $body;
            }
            unsafe { core::mem::transmute(r) }
        }
    };
}

macro_rules! c2 {
    ($name:ident, $t:ty, $u:ty, $k:expr, |$x:ident, $y:ident| $body:expr) => {
        #[inline]
        pub fn $name(a: V, b: V) -> V {
            let va: [$t; $k] = unsafe { core::mem::transmute(a) };
            let vb: [$t; $k] = unsafe { core::mem::transmute(b) };
            let mut r: [$u; $k] = [0 as $u; $k];
            for i in 0..$k {
                let $x: $t = va[i];
                let $y: $t = vb[i];
                r[i] = if $body { !0 as $u } else { 0 as $u };
            }
            unsafe { core::mem::transmute(r) }
        }
    };
}

macro_rules! u1 {
    ($name:ident, $t:ty, $k:expr, |$x:ident| $body:expr) => {
        #[inline]
        pub fn $name(a: V) -> V {
            let va: [$t; $k] = unsafe { core::mem::transmute(a) };
            let mut r: [$t; $k] = [0 as $t; $k];
            for i in 0..$k {
                let $x: $t = va[i];
                r[i] = $body;
            }
            unsafe { core::mem::transmute(r) }
        }
    };
}

macro_rules! sh {
    ($name:ident, $t:ty, $k:expr, |$x:ident, $y:ident| $body:expr) => {
        #[inline]
        pub fn $name(a: V, s: i32) -> V {
            let va: [$t; $k] = unsafe { core::mem::transmute(a) };
            let mut r: [$t; $k] = [0 as $t; $k];
            // wasm: the shift count is taken modulo the lane width (signed
            // immediates therefore behave as if unsigned).
            let lane_bits = (core::mem::size_of::<$t>() * 8) as u32;
            let sh = (s as u32) % lane_bits;
            for i in 0..$k {
                let $x: $t = va[i];
                let $y: u32 = sh;
                r[i] = $body;
            }
            unsafe { core::mem::transmute(r) }
        }
    };
}

macro_rules! splat {
    ($name:ident, $t:ty, $k:expr) => {
        #[inline]
        pub fn $name(a: $t) -> V {
            let r: [$t; $k] = [a; $k];
            unsafe { core::mem::transmute(r) }
        }
    };
}

macro_rules! extract {
    ($name:ident, $t:ty, $k:expr) => {
        #[inline]
        pub fn $name(a: V, lane: u32) -> $t {
            assert!(
                (lane as usize) < $k,
                "extract_lane: lane {lane} out of range for a {}-lane op",
                $k
            );
            let x: [$t; $k] = unsafe { core::mem::transmute(a) };
            x[lane as usize]
        }
    };
}

macro_rules! replace {
    ($name:ident, $t:ty, $k:expr) => {
        #[inline]
        pub fn $name(a: V, lane: u32, v: $t) -> V {
            assert!(
                (lane as usize) < $k,
                "replace_lane: lane {lane} out of range for a {}-lane op",
                $k
            );
            let mut x: [$t; $k] = unsafe { core::mem::transmute(a) };
            x[lane as usize] = v;
            unsafe { core::mem::transmute(x) }
        }
    };
}

extract!(extract_u8, u8, 16);
extract!(extract_i8, i8, 16);
extract!(extract_u16, u16, 8);
extract!(extract_i16, i16, 8);
extract!(extract_u32, u32, 4);
extract!(extract_i32, i32, 4);
extract!(extract_u64, u64, 2);
extract!(extract_i64, i64, 2);
extract!(extract_f32, f32, 4);
extract!(extract_f64, f64, 2);

replace!(replace_u8, u8, 16);
replace!(replace_i8, i8, 16);
replace!(replace_u16, u16, 8);
replace!(replace_i16, i16, 8);
replace!(replace_u32, u32, 4);
replace!(replace_i32, i32, 4);
replace!(replace_u64, u64, 2);
replace!(replace_i64, i64, 2);
replace!(replace_f32, f32, 4);
replace!(replace_f64, f64, 2);

#[inline]
pub fn clamp_i8(x: i32) -> i8 { x.clamp(-128, 127) as i8 }
#[inline]
pub fn clamp_u8(x: i32) -> u8 { x.clamp(0, 255) as u8 }
#[inline]
pub fn clamp_i16(x: i32) -> i16 { x.clamp(-32768, 32767) as i16 }
#[inline]
pub fn clamp_u16(x: i32) -> u16 { x.clamp(0, 65535) as u16 }

u1!(f32x4_abs, f32, 4, |a| a.abs());
b2!(f32x4_add, f32, 4, |a,b| a + b);
u1!(f32x4_ceil, f32, 4, |a| a.ceil());
pub fn f32x4_convert_i32x4_s(a: V) -> V {
    let x: [i32; 4] = unpack_i32(a);
    pack_f32([x[0] as f32, x[1] as f32, x[2] as f32, x[3] as f32])
}
pub fn f32x4_convert_i32x4_u(a: V) -> V {
    let x: [u32; 4] = unpack_u32(a);
    pack_f32([x[0] as f32, x[1] as f32, x[2] as f32, x[3] as f32])
}
pub fn f32x4_demote_f64x2_zero(a: V) -> V {
    let x: [f64; 2] = unpack_f64(a);
    pack_f32([x[0] as f32, x[1] as f32, 0.0, 0.0])
}
b2!(f32x4_div, f32, 4, |a,b| a / b);
c2!(f32x4_eq, f32, u32, 4, |a,b| a == b);
extract!(f32x4_extract_lane, f32, 4);
u1!(f32x4_floor, f32, 4, |a| a.floor());
c2!(f32x4_ge, f32, u32, 4, |a,b| a >= b);
c2!(f32x4_gt, f32, u32, 4, |a,b| a > b);
c2!(f32x4_le, f32, u32, 4, |a,b| a <= b);
c2!(f32x4_lt, f32, u32, 4, |a,b| a < b);
b2!(f32x4_max, f32, 4, |a,b| crate::f32_max(a, b));
b2!(f32x4_min, f32, 4, |a,b| crate::f32_min(a, b));
b2!(f32x4_mul, f32, 4, |a,b| a * b);
c2!(f32x4_ne, f32, u32, 4, |a,b| a != b);
u1!(f32x4_nearest, f32, 4, |a| crate::f32_nearest(a));
u1!(f32x4_neg, f32, 4, |a| -a);
b2!(f32x4_pmax, f32, 4, |a,b| if a < b { b } else { a });
b2!(f32x4_pmin, f32, 4, |a,b| if b < a { b } else { a });
replace!(f32x4_replace_lane, f32, 4);
splat!(f32x4_splat, f32, 4);
u1!(f32x4_sqrt, f32, 4, |a| a.sqrt());
b2!(f32x4_sub, f32, 4, |a,b| a - b);
u1!(f32x4_trunc, f32, 4, |a| a.trunc());
u1!(f64x2_abs, f64, 2, |a| a.abs());
b2!(f64x2_add, f64, 2, |a,b| a + b);
u1!(f64x2_ceil, f64, 2, |a| a.ceil());
pub fn f64x2_convert_low_i32x4_s(a: V) -> V {
    let x: [i32; 4] = unpack_i32(a);
    pack_f64([x[0] as f64, x[1] as f64])
}
pub fn f64x2_convert_low_i32x4_u(a: V) -> V {
    let x: [u32; 4] = unpack_u32(a);
    pack_f64([x[0] as f64, x[1] as f64])
}
b2!(f64x2_div, f64, 2, |a,b| a / b);
c2!(f64x2_eq, f64, u64, 2, |a,b| a == b);
extract!(f64x2_extract_lane, f64, 2);
u1!(f64x2_floor, f64, 2, |a| a.floor());
c2!(f64x2_ge, f64, u64, 2, |a,b| a >= b);
c2!(f64x2_gt, f64, u64, 2, |a,b| a > b);
c2!(f64x2_le, f64, u64, 2, |a,b| a <= b);
c2!(f64x2_lt, f64, u64, 2, |a,b| a < b);
b2!(f64x2_max, f64, 2, |a,b| crate::f64_max(a, b));
b2!(f64x2_min, f64, 2, |a,b| crate::f64_min(a, b));
b2!(f64x2_mul, f64, 2, |a,b| a * b);
c2!(f64x2_ne, f64, u64, 2, |a,b| a != b);
u1!(f64x2_nearest, f64, 2, |a| crate::f64_nearest(a));
u1!(f64x2_neg, f64, 2, |a| -a);
b2!(f64x2_pmax, f64, 2, |a,b| if a < b { b } else { a });
b2!(f64x2_pmin, f64, 2, |a,b| if b < a { b } else { a });
pub fn f64x2_promote_low_f32x4(a: V) -> V {
    let x: [f32; 4] = unpack_f32(a);
    pack_f64([x[0] as f64, x[1] as f64])
}
replace!(f64x2_replace_lane, f64, 2);
splat!(f64x2_splat, f64, 2);
u1!(f64x2_sqrt, f64, 2, |a| a.sqrt());
b2!(f64x2_sub, f64, 2, |a,b| a - b);
u1!(f64x2_trunc, f64, 2, |a| a.trunc());
u1!(i16x8_abs, i16, 8, |a| a.wrapping_abs());
b2!(i16x8_add, i16, 8, |a,b| a.wrapping_add(b));
b2!(i16x8_add_sat_s, i16, 8, |a,b| a.saturating_add(b));
b2!(i16x8_add_sat_u, u16, 8, |a,b| a.saturating_add(b));
pub fn i16x8_all_true(a: V) -> i32 { let x: [u16; 8] = unpack_u16(a); (x.iter().all(|&b| b != 0)) as i32 }
b2!(i16x8_avgr_u, u16, 8, |a,b| (((a as u32) + (b as u32) + 1) >> 1) as u16);
pub fn i16x8_bitmask(a: V) -> i32 { let x: [u16; 8] = unpack_u16(a); let mut m = 0i32; for i in 0..8 { if x[i] & 0x8000 != 0 { m |= 1 << i; } } m }
c2!(i16x8_eq, i16, u16, 8, |a,b| a == b);
pub fn i16x8_extend_high_i8x16_s(a: V) -> V {
    let x: [i8; 16] = unpack_i8(a);
    let o = 8;
    let mut r = [0i16; 8];
    for i in 0..8 { r[i] = x[o + i] as i16; }
    pack_i16(r)
}
pub fn i16x8_extend_high_i8x16_u(a: V) -> V {
    let x: [u8; 16] = unpack_u8(a);
    let o = 8;
    let mut r = [0u16; 8];
    for i in 0..8 { r[i] = x[o + i] as u16; }
    pack_u16(r)
}
pub fn i16x8_extend_low_i8x16_s(a: V) -> V {
    let x: [i8; 16] = unpack_i8(a);
    let o = 0;
    let mut r = [0i16; 8];
    for i in 0..8 { r[i] = x[o + i] as i16; }
    pack_i16(r)
}
pub fn i16x8_extend_low_i8x16_u(a: V) -> V {
    let x: [u8; 16] = unpack_u8(a);
    let o = 0;
    let mut r = [0u16; 8];
    for i in 0..8 { r[i] = x[o + i] as u16; }
    pack_u16(r)
}
pub fn i16x8_extmul_high_i8x16_s(a: V, b: V) -> V {
    let x: [i8; 16] = unpack_i8(a);
    let y: [i8; 16] = unpack_i8(b);
    let o = 8;
    let mut r = [0i16; 8];
    for i in 0..8 { r[i] = (x[o + i] as i16).wrapping_mul(y[o + i] as i16); }
    pack_i16(r)
}
pub fn i16x8_extmul_high_i8x16_u(a: V, b: V) -> V {
    let x: [u8; 16] = unpack_u8(a);
    let y: [u8; 16] = unpack_u8(b);
    let o = 8;
    let mut r = [0u16; 8];
    for i in 0..8 { r[i] = (x[o + i] as u16).wrapping_mul(y[o + i] as u16); }
    pack_u16(r)
}
pub fn i16x8_extmul_low_i8x16_s(a: V, b: V) -> V {
    let x: [i8; 16] = unpack_i8(a);
    let y: [i8; 16] = unpack_i8(b);
    let o = 0;
    let mut r = [0i16; 8];
    for i in 0..8 { r[i] = (x[o + i] as i16).wrapping_mul(y[o + i] as i16); }
    pack_i16(r)
}
pub fn i16x8_extmul_low_i8x16_u(a: V, b: V) -> V {
    let x: [u8; 16] = unpack_u8(a);
    let y: [u8; 16] = unpack_u8(b);
    let o = 0;
    let mut r = [0u16; 8];
    for i in 0..8 { r[i] = (x[o + i] as u16).wrapping_mul(y[o + i] as u16); }
    pack_u16(r)
}
extract!(i16x8_extract_lane_s, i16, 8);
extract!(i16x8_extract_lane_u, u16, 8);
c2!(i16x8_ge_s, i16, u16, 8, |a,b| a >= b);
c2!(i16x8_ge_u, u16, u16, 8, |a,b| a >= b);
c2!(i16x8_gt_s, i16, u16, 8, |a,b| a > b);
c2!(i16x8_gt_u, u16, u16, 8, |a,b| a > b);
c2!(i16x8_le_s, i16, u16, 8, |a,b| a <= b);
c2!(i16x8_le_u, u16, u16, 8, |a,b| a <= b);
c2!(i16x8_lt_s, i16, u16, 8, |a,b| a < b);
c2!(i16x8_lt_u, u16, u16, 8, |a,b| a < b);
b2!(i16x8_max_s, i16, 8, |a,b| if a > b { a } else { b });
b2!(i16x8_max_u, u16, 8, |a,b| if a > b { a } else { b });
b2!(i16x8_min_s, i16, 8, |a,b| if a < b { a } else { b });
b2!(i16x8_min_u, u16, 8, |a,b| if a < b { a } else { b });
b2!(i16x8_mul, i16, 8, |a,b| a.wrapping_mul(b));
pub fn i16x8_narrow_i32x4_s(a: V, b: V) -> V {
    let x: [i32; 4] = unpack_i32(a);
    let y: [i32; 4] = unpack_i32(b);
    let mut r = [0i16; 8];
    for i in 0..4 { r[i] = clamp_i16(x[i]); r[4 + i] = clamp_i16(y[i]); }
    pack_i16(r)
}
pub fn i16x8_narrow_i32x4_u(a: V, b: V) -> V {
    let x: [i32; 4] = unpack_i32(a);
    let y: [i32; 4] = unpack_i32(b);
    let mut r = [0i16; 8];
    for i in 0..4 { r[i] = clamp_u16(x[i]) as i16; r[4 + i] = clamp_u16(y[i]) as i16; }
    pack_i16(r)
}
c2!(i16x8_ne, i16, u16, 8, |a,b| a != b);
u1!(i16x8_neg, i16, 8, |a| a.wrapping_neg());
b2!(i16x8_q15mulr_sat_s, i16, 8, |a,b| ((((a as i32) * (b as i32)) + 0x4000) >> 15).clamp(-32768, 32767) as i16);
replace!(i16x8_replace_lane, i16, 8);
sh!(i16x8_shl, i16, 8, |a,s| a.wrapping_shl(s));
sh!(i16x8_shr_s, i16, 8, |a,s| a.wrapping_shr(s));
sh!(i16x8_shr_u, u16, 8, |a,s| a.wrapping_shr(s));
splat!(i16x8_splat, i16, 8);
b2!(i16x8_sub, i16, 8, |a,b| a.wrapping_sub(b));
b2!(i16x8_sub_sat_s, i16, 8, |a,b| a.saturating_sub(b));
b2!(i16x8_sub_sat_u, u16, 8, |a,b| a.saturating_sub(b));
u1!(i32x4_abs, i32, 4, |a| a.wrapping_abs());
b2!(i32x4_add, i32, 4, |a,b| a.wrapping_add(b));
pub fn i32x4_all_true(a: V) -> i32 { let x: [u32; 4] = unpack_u32(a); (x.iter().all(|&b| b != 0)) as i32 }
pub fn i32x4_bitmask(a: V) -> i32 { let x: [u32; 4] = unpack_u32(a); let mut m = 0i32; for i in 0..4 { if x[i] & 0x8000_0000 != 0 { m |= 1 << i; } } m }
c2!(i32x4_eq, i32, u32, 4, |a,b| a == b);
pub fn i32x4_extend_high_i16x8_s(a: V) -> V {
    let x: [i16; 8] = unpack_i16(a);
    let o = 4;
    let mut r = [0i32; 4];
    for i in 0..4 { r[i] = x[o + i] as i32; }
    pack_i32(r)
}
pub fn i32x4_extend_high_i16x8_u(a: V) -> V {
    let x: [u16; 8] = unpack_u16(a);
    let o = 4;
    let mut r = [0u32; 4];
    for i in 0..4 { r[i] = x[o + i] as u32; }
    pack_u32(r)
}
pub fn i32x4_extend_low_i16x8_s(a: V) -> V {
    let x: [i16; 8] = unpack_i16(a);
    let o = 0;
    let mut r = [0i32; 4];
    for i in 0..4 { r[i] = x[o + i] as i32; }
    pack_i32(r)
}
pub fn i32x4_extend_low_i16x8_u(a: V) -> V {
    let x: [u16; 8] = unpack_u16(a);
    let o = 0;
    let mut r = [0u32; 4];
    for i in 0..4 { r[i] = x[o + i] as u32; }
    pack_u32(r)
}
pub fn i32x4_extmul_high_i16x8_s(a: V, b: V) -> V {
    let x: [i16; 8] = unpack_i16(a);
    let y: [i16; 8] = unpack_i16(b);
    let o = 4;
    let mut r = [0i32; 4];
    for i in 0..4 { r[i] = (x[o + i] as i32).wrapping_mul(y[o + i] as i32); }
    pack_i32(r)
}
pub fn i32x4_extmul_high_i16x8_u(a: V, b: V) -> V {
    let x: [u16; 8] = unpack_u16(a);
    let y: [u16; 8] = unpack_u16(b);
    let o = 4;
    let mut r = [0u32; 4];
    for i in 0..4 { r[i] = (x[o + i] as u32).wrapping_mul(y[o + i] as u32); }
    pack_u32(r)
}
pub fn i32x4_extmul_low_i16x8_s(a: V, b: V) -> V {
    let x: [i16; 8] = unpack_i16(a);
    let y: [i16; 8] = unpack_i16(b);
    let o = 0;
    let mut r = [0i32; 4];
    for i in 0..4 { r[i] = (x[o + i] as i32).wrapping_mul(y[o + i] as i32); }
    pack_i32(r)
}
pub fn i32x4_extmul_low_i16x8_u(a: V, b: V) -> V {
    let x: [u16; 8] = unpack_u16(a);
    let y: [u16; 8] = unpack_u16(b);
    let o = 0;
    let mut r = [0u32; 4];
    for i in 0..4 { r[i] = (x[o + i] as u32).wrapping_mul(y[o + i] as u32); }
    pack_u32(r)
}
extract!(i32x4_extract_lane, i32, 4);
c2!(i32x4_ge_s, i32, u32, 4, |a,b| a >= b);
c2!(i32x4_ge_u, u32, u32, 4, |a,b| a >= b);
c2!(i32x4_gt_s, i32, u32, 4, |a,b| a > b);
c2!(i32x4_gt_u, u32, u32, 4, |a,b| a > b);
c2!(i32x4_le_s, i32, u32, 4, |a,b| a <= b);
c2!(i32x4_le_u, u32, u32, 4, |a,b| a <= b);
c2!(i32x4_lt_s, i32, u32, 4, |a,b| a < b);
c2!(i32x4_lt_u, u32, u32, 4, |a,b| a < b);
b2!(i32x4_max_s, i32, 4, |a,b| if a > b { a } else { b });
b2!(i32x4_max_u, u32, 4, |a,b| if a > b { a } else { b });
b2!(i32x4_min_s, i32, 4, |a,b| if a < b { a } else { b });
b2!(i32x4_min_u, u32, 4, |a,b| if a < b { a } else { b });
b2!(i32x4_mul, i32, 4, |a,b| a.wrapping_mul(b));
c2!(i32x4_ne, i32, u32, 4, |a,b| a != b);
u1!(i32x4_neg, i32, 4, |a| a.wrapping_neg());
replace!(i32x4_replace_lane, i32, 4);
sh!(i32x4_shl, i32, 4, |a,s| a.wrapping_shl(s));
sh!(i32x4_shr_s, i32, 4, |a,s| a.wrapping_shr(s));
sh!(i32x4_shr_u, u32, 4, |a,s| a.wrapping_shr(s));
splat!(i32x4_splat, i32, 4);
b2!(i32x4_sub, i32, 4, |a,b| a.wrapping_sub(b));
pub fn i32x4_trunc_sat_f32x4_s(a: V) -> V {
    let x: [f32; 4] = unpack_f32(a);
    let mut r = [0i32; 4];
    for i in 0..4 {
        let f = x[i];
        r[i] = if f.is_nan() { 0 } else if f <= -2147483648.0 { i32::MIN }
               else if f >= 2147483648.0 { i32::MAX } else { f as i32 };
    }
    pack_i32(r)
}
pub fn i32x4_trunc_sat_f32x4_u(a: V) -> V {
    let x: [f32; 4] = unpack_f32(a);
    let mut r = [0i32; 4];
    for i in 0..4 {
        let f = x[i];
        r[i] = if f.is_nan() || f <= 0.0 { 0 }
               else if f >= 4294967296.0 { -1 } else { f as u32 as i32 };
    }
    pack_i32(r)
}
pub fn i32x4_trunc_sat_f64x2_s_zero(a: V) -> V {
    let x: [f64; 2] = unpack_f64(a);
    let mut r = [0i32; 4];
    for i in 0..2 {
        let f = x[i];
        r[i] = if f.is_nan() { 0 } else if f <= -2147483648.0 { i32::MIN }
               else if f >= 2147483648.0 { i32::MAX } else { f as i32 };
    }
    pack_i32(r)
}
pub fn i32x4_trunc_sat_f64x2_u_zero(a: V) -> V {
    let x: [f64; 2] = unpack_f64(a);
    let mut r = [0i32; 4];
    for i in 0..2 {
        let f = x[i];
        r[i] = if f.is_nan() || f <= 0.0 { 0 }
               else if f >= 4294967296.0 { -1 } else { f as u32 as i32 };
    }
    pack_i32(r)
}
u1!(i64x2_abs, i64, 2, |a| a.wrapping_abs());
b2!(i64x2_add, i64, 2, |a,b| a.wrapping_add(b));
pub fn i64x2_all_true(a: V) -> i32 { let x: [u64; 2] = unpack_u64(a); (x.iter().all(|&b| b != 0)) as i32 }
pub fn i64x2_bitmask(a: V) -> i32 { let x: [u64; 2] = unpack_u64(a); let mut m = 0i32; for i in 0..2 { if x[i] & 0x8000_0000_0000_0000 != 0 { m |= 1 << i; } } m }
c2!(i64x2_eq, i64, u64, 2, |a,b| a == b);
pub fn i64x2_extend_high_i32x4_s(a: V) -> V {
    let x: [i32; 4] = unpack_i32(a);
    let o = 2;
    let mut r = [0i64; 2];
    for i in 0..2 { r[i] = x[o + i] as i64; }
    pack_i64(r)
}
pub fn i64x2_extend_high_i32x4_u(a: V) -> V {
    let x: [u32; 4] = unpack_u32(a);
    let o = 2;
    let mut r = [0u64; 2];
    for i in 0..2 { r[i] = x[o + i] as u64; }
    pack_u64(r)
}
pub fn i64x2_extend_low_i32x4_s(a: V) -> V {
    let x: [i32; 4] = unpack_i32(a);
    let o = 0;
    let mut r = [0i64; 2];
    for i in 0..2 { r[i] = x[o + i] as i64; }
    pack_i64(r)
}
pub fn i64x2_extend_low_i32x4_u(a: V) -> V {
    let x: [u32; 4] = unpack_u32(a);
    let o = 0;
    let mut r = [0u64; 2];
    for i in 0..2 { r[i] = x[o + i] as u64; }
    pack_u64(r)
}
pub fn i64x2_extmul_high_i32x4_s(a: V, b: V) -> V {
    let x: [i32; 4] = unpack_i32(a);
    let y: [i32; 4] = unpack_i32(b);
    let o = 2;
    let mut r = [0i64; 2];
    for i in 0..2 { r[i] = (x[o + i] as i64).wrapping_mul(y[o + i] as i64); }
    pack_i64(r)
}
pub fn i64x2_extmul_high_i32x4_u(a: V, b: V) -> V {
    let x: [u32; 4] = unpack_u32(a);
    let y: [u32; 4] = unpack_u32(b);
    let o = 2;
    let mut r = [0u64; 2];
    for i in 0..2 { r[i] = (x[o + i] as u64).wrapping_mul(y[o + i] as u64); }
    pack_u64(r)
}
pub fn i64x2_extmul_low_i32x4_s(a: V, b: V) -> V {
    let x: [i32; 4] = unpack_i32(a);
    let y: [i32; 4] = unpack_i32(b);
    let o = 0;
    let mut r = [0i64; 2];
    for i in 0..2 { r[i] = (x[o + i] as i64).wrapping_mul(y[o + i] as i64); }
    pack_i64(r)
}
pub fn i64x2_extmul_low_i32x4_u(a: V, b: V) -> V {
    let x: [u32; 4] = unpack_u32(a);
    let y: [u32; 4] = unpack_u32(b);
    let o = 0;
    let mut r = [0u64; 2];
    for i in 0..2 { r[i] = (x[o + i] as u64).wrapping_mul(y[o + i] as u64); }
    pack_u64(r)
}
extract!(i64x2_extract_lane, i64, 2);
c2!(i64x2_ge_s, i64, u64, 2, |a,b| a >= b);
c2!(i64x2_gt_s, i64, u64, 2, |a,b| a > b);
c2!(i64x2_le_s, i64, u64, 2, |a,b| a <= b);
c2!(i64x2_lt_s, i64, u64, 2, |a,b| a < b);
b2!(i64x2_mul, i64, 2, |a,b| a.wrapping_mul(b));
c2!(i64x2_ne, i64, u64, 2, |a,b| a != b);
u1!(i64x2_neg, i64, 2, |a| a.wrapping_neg());
replace!(i64x2_replace_lane, i64, 2);
sh!(i64x2_shl, i64, 2, |a,s| a.wrapping_shl(s));
sh!(i64x2_shr_s, i64, 2, |a,s| a.wrapping_shr(s));
sh!(i64x2_shr_u, u64, 2, |a,s| a.wrapping_shr(s));
splat!(i64x2_splat, i64, 2);
b2!(i64x2_sub, i64, 2, |a,b| a.wrapping_sub(b));
u1!(i8x16_abs, i8, 16, |a| a.wrapping_abs());
b2!(i8x16_add, i8, 16, |a,b| a.wrapping_add(b));
b2!(i8x16_add_sat_s, i8, 16, |a,b| a.saturating_add(b));
b2!(i8x16_add_sat_u, u8, 16, |a,b| a.saturating_add(b));
pub fn i8x16_all_true(a: V) -> i32 { let x: [u8; 16] = unpack_u8(a); (x.iter().all(|&b| b != 0)) as i32 }
b2!(i8x16_avgr_u, u8, 16, |a,b| (((a as u16) + (b as u16) + 1) >> 1) as u8);
pub fn i8x16_bitmask(a: V) -> i32 { let x: [u8; 16] = unpack_u8(a); let mut m = 0i32; for i in 0..16 { if x[i] & 0x80 != 0 { m |= 1 << i; } } m }
c2!(i8x16_eq, i8, u8, 16, |a,b| a == b);
extract!(i8x16_extract_lane_s, i8, 16);
extract!(i8x16_extract_lane_u, u8, 16);
c2!(i8x16_ge_s, i8, u8, 16, |a,b| a >= b);
c2!(i8x16_ge_u, u8, u8, 16, |a,b| a >= b);
c2!(i8x16_gt_s, i8, u8, 16, |a,b| a > b);
c2!(i8x16_gt_u, u8, u8, 16, |a,b| a > b);
c2!(i8x16_le_s, i8, u8, 16, |a,b| a <= b);
c2!(i8x16_le_u, u8, u8, 16, |a,b| a <= b);
c2!(i8x16_lt_s, i8, u8, 16, |a,b| a < b);
c2!(i8x16_lt_u, u8, u8, 16, |a,b| a < b);
b2!(i8x16_max_s, i8, 16, |a,b| if a > b { a } else { b });
b2!(i8x16_max_u, u8, 16, |a,b| if a > b { a } else { b });
b2!(i8x16_min_s, i8, 16, |a,b| if a < b { a } else { b });
b2!(i8x16_min_u, u8, 16, |a,b| if a < b { a } else { b });
pub fn i8x16_narrow_i16x8_s(a: V, b: V) -> V {
    let x: [i16; 8] = unpack_i16(a);
    let y: [i16; 8] = unpack_i16(b);
    let mut r = [0i8; 16];
    for i in 0..8 { r[i] = clamp_i8(x[i] as i32); r[8 + i] = clamp_i8(y[i] as i32); }
    pack_i8(r)
}
pub fn i8x16_narrow_i16x8_u(a: V, b: V) -> V {
    let x: [i16; 8] = unpack_i16(a);
    let y: [i16; 8] = unpack_i16(b);
    let mut r = [0i8; 16];
    for i in 0..8 { r[i] = clamp_u8(x[i] as i32) as i8; r[8 + i] = clamp_u8(y[i] as i32) as i8; }
    pack_i8(r)
}
c2!(i8x16_ne, i8, u8, 16, |a,b| a != b);
u1!(i8x16_neg, i8, 16, |a| a.wrapping_neg());
u1!(i8x16_popcnt, u8, 16, |a| a.count_ones() as u8);
replace!(i8x16_replace_lane, i8, 16);
sh!(i8x16_shl, i8, 16, |a,s| a.wrapping_shl(s));
sh!(i8x16_shr_s, i8, 16, |a,s| a.wrapping_shr(s));
sh!(i8x16_shr_u, u8, 16, |a,s| a.wrapping_shr(s));
#[inline(always)]
pub fn i8x16_shuffle(a: V, b: V, imm: V) -> V {
    let x: [u8; 16] = unpack_u8(a);
    let y: [u8; 16] = unpack_u8(b);
    let m: [u8; 16] = unpack_u8(imm);
    let mut r = [0u8; 16];
    for i in 0..16 {
        let j = m[i] as usize;
        assert!(j < 32, "i8x16.shuffle: immediate {} out of range 0..32", j);
        r[i] = if j < 16 { x[j] } else { y[j - 16] };
    }
    pack_u8(r)
}
splat!(i8x16_splat, i8, 16);
b2!(i8x16_sub, i8, 16, |a,b| a.wrapping_sub(b));
b2!(i8x16_sub_sat_s, i8, 16, |a,b| a.saturating_sub(b));
b2!(i8x16_sub_sat_u, u8, 16, |a,b| a.saturating_sub(b));
pub fn i8x16_swizzle(a: V, b: V) -> V {
    let x: [u8; 16] = unpack_u8(a);
    let m: [u8; 16] = unpack_u8(b);
    let mut r = [0u8; 16];
    for i in 0..16 { let j = m[i] as usize; r[i] = if j < 16 { x[j] } else { 0 }; }
    pack_u8(r)
}
pub fn v128_and(a: V, b: V) -> V { a & b }
pub fn v128_andnot(a: V, b: V) -> V { a & !b }
pub fn v128_any_true(a: V) -> i32 { (a != 0) as i32 }
pub fn v128_bitselect(a: V, b: V, c: V) -> V { (a & c) | (b & !c) }
pub fn v128_not(a: V) -> V { !a }
pub fn v128_or(a: V, b: V) -> V { a | b }
pub fn v128_xor(a: V, b: V) -> V { a ^ b }

// generated ops: 208 (+5 in ADDITIONS)

// ===========================================================================
// ADDITIONS — not in tools/w2r/tables.py yet (see report/simd-audit.md)
//
// Five core wasm SIMD ops are absent from the table, so the generated block
// cannot emit them. They are defined here so the module still links; each one
// needs the following table entry to move into the generator:
//
//   _s("i16x8.extadd_pairwise_i8x16_s", "raw", <body below>)
//   _s("i16x8.extadd_pairwise_i8x16_u", "raw", <body below>)
//   _s("i32x4.extadd_pairwise_i16x8_s", "raw", <body below>)
//   _s("i32x4.extadd_pairwise_i16x8_u", "raw", <body below>)
//   _s("i32x4.dot_i16x8_s",             "raw", <body below>)
// ===========================================================================

/// `i32x4.dot_i16x8_s`: adjacent i16 products summed pairwise into i32 lanes.
///
/// Each product fits i32 exactly; the sum of the pair is modular, so a pair of
/// `-32768 * -32768` products wraps to `i32::MIN` instead of saturating.
pub fn i32x4_dot_i16x8_s(a: V, b: V) -> V {
    let x: [i16; 8] = unpack_i16(a);
    let y: [i16; 8] = unpack_i16(b);
    let mut r = [0i32; 4];
    for i in 0..4 {
        let lo = (x[2 * i] as i32).wrapping_mul(y[2 * i] as i32);
        let hi = (x[2 * i + 1] as i32).wrapping_mul(y[2 * i + 1] as i32);
        r[i] = lo.wrapping_add(hi);
    }
    pack_i32(r)
}

/// `i16x8.extadd_pairwise_i8x16_s`: adjacent i8 pairs summed into i16 lanes.
pub fn i16x8_extadd_pairwise_i8x16_s(a: V) -> V {
    let x: [i8; 16] = unpack_i8(a);
    let mut r = [0i16; 8];
    for i in 0..8 {
        r[i] = (x[2 * i] as i16).wrapping_add(x[2 * i + 1] as i16);
    }
    pack_i16(r)
}

/// `i16x8.extadd_pairwise_i8x16_u`: adjacent u8 pairs summed into u16 lanes.
pub fn i16x8_extadd_pairwise_i8x16_u(a: V) -> V {
    let x: [u8; 16] = unpack_u8(a);
    let mut r = [0u16; 8];
    for i in 0..8 {
        r[i] = (x[2 * i] as u16).wrapping_add(x[2 * i + 1] as u16);
    }
    pack_u16(r)
}

/// `i32x4.extadd_pairwise_i16x8_s`: adjacent i16 pairs summed into i32 lanes.
pub fn i32x4_extadd_pairwise_i16x8_s(a: V) -> V {
    let x: [i16; 8] = unpack_i16(a);
    let mut r = [0i32; 4];
    for i in 0..4 {
        r[i] = (x[2 * i] as i32).wrapping_add(x[2 * i + 1] as i32);
    }
    pack_i32(r)
}

/// `i32x4.extadd_pairwise_i16x8_u`: adjacent u16 pairs summed into u32 lanes.
pub fn i32x4_extadd_pairwise_i16x8_u(a: V) -> V {
    let x: [u16; 8] = unpack_u16(a);
    let mut r = [0u32; 4];
    for i in 0..4 {
        r[i] = (x[2 * i] as u32).wrapping_add(x[2 * i + 1] as u32);
    }
    pack_u32(r)
}
