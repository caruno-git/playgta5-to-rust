//! Bit-exact v128 helpers used by the SIMD workload.
//!
//! The recovered module's SIMD helpers are ordinary wasm `v128` ops. They are
//! reimplemented here over `u128` so the harness does not depend on a host
//! `wasmtime`/`wasmer` build. Wrapping integer arithmetic is exactly wasm's;
//! the two float helpers operate on fixed bit patterns and are deterministic on
//! one target (cross-target float behaviour is called out in DETERMINISM.md).

/// `i32x4.splat`.
pub fn splat_i32(x: i32) -> u128 {
    let mut out = [0u8; 16];
    for chunk in out.chunks_exact_mut(4) {
        chunk.copy_from_slice(&x.to_le_bytes());
    }
    u128::from_le_bytes(out)
}

/// Replace lane `lane` (0..4) with `value`.
pub fn set_lane_i32(v: u128, lane: usize, value: i32) -> u128 {
    let mut b = v.to_le_bytes();
    let at = (lane & 3) * 4;
    b[at..at + 4].copy_from_slice(&value.to_le_bytes());
    u128::from_le_bytes(b)
}

/// Extract `i32` lane `lane` (0..3).
pub fn lane_i32(v: u128, lane: usize) -> i32 {
    let b = v.to_le_bytes();
    let at = (lane & 3) * 4;
    i32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

/// `i32x4.add`.
pub fn i32x4_add(a: u128, b: u128) -> u128 {
    let mut out = [0u8; 16];
    for i in 0..4 {
        let at = i * 4;
        out[at..at + 4].copy_from_slice(&lane_i32(a, i).wrapping_add(lane_i32(b, i)).to_le_bytes());
    }
    u128::from_le_bytes(out)
}

/// `i32x4.mul`.
pub fn i32x4_mul(a: u128, b: u128) -> u128 {
    let mut out = [0u8; 16];
    for i in 0..4 {
        let at = i * 4;
        out[at..at + 4].copy_from_slice(&lane_i32(a, i).wrapping_mul(lane_i32(b, i)).to_le_bytes());
    }
    u128::from_le_bytes(out)
}

/// `i16x8.mul` (wrapping).
pub fn i16x8_mul(a: u128, b: u128) -> u128 {
    let ab = a.to_le_bytes();
    let bb = b.to_le_bytes();
    let mut out = [0u8; 16];
    for i in 0..8 {
        let at = i * 2;
        let x = i16::from_le_bytes([ab[at], ab[at + 1]]);
        let y = i16::from_le_bytes([bb[at], bb[at + 1]]);
        out[at..at + 2].copy_from_slice(&x.wrapping_mul(y).to_le_bytes());
    }
    u128::from_le_bytes(out)
}

/// `i16x8.min_s`.
pub fn i16x8_min_s(a: u128, b: u128) -> u128 {
    let ab = a.to_le_bytes();
    let bb = b.to_le_bytes();
    let mut out = [0u8; 16];
    for i in 0..8 {
        let at = i * 2;
        let x = i16::from_le_bytes([ab[at], ab[at + 1]]);
        let y = i16::from_le_bytes([bb[at], bb[at + 1]]);
        out[at..at + 2].copy_from_slice(&x.min(y).to_le_bytes());
    }
    u128::from_le_bytes(out)
}

/// `i8x16.add_sat_s`.
pub fn i8x16_add_sat_s(a: u128, b: u128) -> u128 {
    let ab = a.to_le_bytes();
    let bb = b.to_le_bytes();
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = (ab[i] as i8).saturating_add(bb[i] as i8) as u8;
    }
    u128::from_le_bytes(out)
}

/// `i8x16.shuffle` with a 16-byte lane index table (indices are taken mod 32).
pub fn i8x16_shuffle(a: u128, b: u128, mask: [u8; 16]) -> u128 {
    let mut joined = [0u8; 32];
    joined[..16].copy_from_slice(&a.to_le_bytes());
    joined[16..].copy_from_slice(&b.to_le_bytes());
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = joined[(mask[i] & 31) as usize];
    }
    u128::from_le_bytes(out)
}

/// `i64x2.shl`.
pub fn i64x2_shl(a: u128, shift: u32) -> u128 {
    let b = a.to_le_bytes();
    let mut out = [0u8; 16];
    for i in 0..2 {
        let at = i * 8;
        let x = u64::from_le_bytes([
            b[at],
            b[at + 1],
            b[at + 2],
            b[at + 3],
            b[at + 4],
            b[at + 5],
            b[at + 6],
            b[at + 7],
        ]);
        out[at..at + 8].copy_from_slice(&x.wrapping_shl(shift & 63).to_le_bytes());
    }
    u128::from_le_bytes(out)
}

/// `v128.and`.
pub const fn v128_and(a: u128, b: u128) -> u128 {
    a & b
}

/// `v128.or`.
pub const fn v128_or(a: u128, b: u128) -> u128 {
    a | b
}

/// `v128.xor`.
pub const fn v128_xor(a: u128, b: u128) -> u128 {
    a ^ b
}

/// `v128.bitselect`.
pub const fn v128_bitselect(a: u128, b: u128, mask: u128) -> u128 {
    (a & mask) | (b & !mask)
}

/// `f32x4.add` over fixed bit patterns.
pub fn f32x4_add_bits(a: u128, b: u128) -> u128 {
    let ab = a.to_le_bytes();
    let bb = b.to_le_bytes();
    let mut out = [0u8; 16];
    for i in 0..4 {
        let at = i * 4;
        let x = f32::from_bits(u32::from_le_bytes([
            ab[at],
            ab[at + 1],
            ab[at + 2],
            ab[at + 3],
        ]));
        let y = f32::from_bits(u32::from_le_bytes([
            bb[at],
            bb[at + 1],
            bb[at + 2],
            bb[at + 3],
        ]));
        out[at..at + 4].copy_from_slice(&(x + y).to_bits().to_le_bytes());
    }
    u128::from_le_bytes(out)
}

/// `f32x4.mul` over fixed bit patterns.
pub fn f32x4_mul_bits(a: u128, b: u128) -> u128 {
    let ab = a.to_le_bytes();
    let bb = b.to_le_bytes();
    let mut out = [0u8; 16];
    for i in 0..4 {
        let at = i * 4;
        let x = f32::from_bits(u32::from_le_bytes([
            ab[at],
            ab[at + 1],
            ab[at + 2],
            ab[at + 3],
        ]));
        let y = f32::from_bits(u32::from_le_bytes([
            bb[at],
            bb[at + 1],
            bb[at + 2],
            bb[at + 3],
        ]));
        out[at..at + 4].copy_from_slice(&(x * y).to_bits().to_le_bytes());
    }
    u128::from_le_bytes(out)
}

/// `i32x4.dot_i16x8_s`: adjacent i16 products summed into four i32 lanes.
pub fn i32x4_dot_i16x8_s(a: u128, b: u128) -> u128 {
    let ab = a.to_le_bytes();
    let bb = b.to_le_bytes();
    let mut out = [0u8; 16];
    for lane in 0..4 {
        let mut sum = 0i32;
        for k in 0..2 {
            let at = lane * 4 + k * 2;
            let x = i16::from_le_bytes([ab[at], ab[at + 1]]) as i32;
            let y = i16::from_le_bytes([bb[at], bb[at + 1]]) as i32;
            sum = sum.wrapping_add(x.wrapping_mul(y));
        }
        out[lane * 4..lane * 4 + 4].copy_from_slice(&sum.to_le_bytes());
    }
    u128::from_le_bytes(out)
}
