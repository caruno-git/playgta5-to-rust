//! Repeatable workloads. Each tick is a pure function of the tick index, the
//! linear image and (for the control) the wall clock.
//!
//! The tick index is the only varying input, so `300 ticks` always means the
//! same 300 operations in the same order. No workload reads the host clock
//! except [`wall_clock`], which exists precisely to prove the harness notices.

use std::time::{SystemTime, UNIX_EPOCH};

use super::runner::TickInfo;
use super::simd;
use super::vm::{Vm, DATA_BASE, FUTEX_BASE, SCRATCH_BASE, SEGMENT_BYTES, SEGMENT_COUNT};

/// Integer/FP arithmetic: wrapping mul, xorshift, rotates, division, clz/ctz,
/// popcount and a fixed-bit-pattern f32 path.
pub fn arithmetic(vm: &mut Vm, info: &TickInfo) {
    let t = info.index;
    let mut r = vm.regs;

    r[0] = r[0]
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407 ^ t);
    r[1] ^= r[1] << 13;
    r[1] ^= r[1] >> 7;
    r[1] ^= r[1] << 17;
    r[1] = r[1].rotate_left((t % 63 + 1) as u32) ^ r[0];
    r[2] = r[1]
        .wrapping_add(0x9E37_79B9_7F4A_7C15)
        .rotate_right((t % 31) as u32);
    r[3] = r[0]
        .wrapping_add(r[2] | 1)
        .wrapping_mul(0xD6E8_FEB8_6659_FD93);

    let d = r[4] | 1;
    r[4] = (r[0] / d) ^ (r[1] % d);
    let sd = (r[5] as i64) | 1;
    r[5] = (r[0] as i64).checked_div(sd).unwrap_or(i64::MAX) as u64;

    r[6] = r[2].leading_zeros() as u64
        + r[3].trailing_zeros() as u64
        + t.count_ones() as u64
        + r[0].count_ones() as u64;
    r[7] = (r[7] as i64)
        .wrapping_mul(0x5851_F42D_4C95_7F2D)
        .wrapping_add(t as i64) as u64;

    // Normal-range f32 arithmetic on a fixed exponent; bit-exact on one target.
    let f = f32::from_bits(0x3F80_0000 | ((t as u32) & 0x007F_FFFF));
    let g = f * 1.5 + 0.25;
    r[8] = (g.to_bits() as u64) ^ r[6];
    r[9] = r[3].count_ones() as u64;
    r[10] = r[0].swap_bytes() ^ r[1].rotate_left(9);
    r[11] = r[2].wrapping_sub(r[3]).wrapping_mul(t + 1);

    vm.globals[0] = vm.globals[0].wrapping_add(r[0] ^ r[6]);
    vm.globals[1] ^= r[3].rotate_left((t % 64) as u32);
    vm.globals[4] = r[8] ^ vm.globals[4];

    vm.regs = r;
    vm.pc = 0x101;
}

/// Linear-memory traffic: aligned/unaligned stores, read-modify-write, memcpy,
/// memset and a store that crosses a 64 KiB page boundary.
pub fn memory(vm: &mut Vm, info: &TickInfo) {
    let t = info.index;
    let slot = (t % 32) as u32;
    let base = DATA_BASE + 0x40_0000 + slot * 0x100;

    vm.write_u64(base, t.wrapping_mul(0x9E37_79B9_7F4A_7C15));
    vm.write(base + 1, &(t as u32).to_le_bytes()); // unaligned
    let old = vm.read_u32(base);
    vm.write_u32(base + 8, old ^ (t as u32).rotate_left(7));

    // Copy a 256-byte window out of the segment image into scratch.
    let src =
        DATA_BASE + 0x1000 + ((t as usize * 97) % (SEGMENT_COUNT * SEGMENT_BYTES - 256)) as u32;
    let block = vm.read(src, 256);
    vm.write(SCRATCH_BASE + 0x1000 + slot * 0x100, &block);

    // memset-style fill, then fold it back into a register.
    let fill = [((t & 0xFF) as u8); 64];
    vm.write(SCRATCH_BASE + 0x2000 + slot * 0x40, &fill);
    let mut fold = 0u64;
    for chunk in vm
        .read(SCRATCH_BASE + 0x2000 + slot * 0x40, 64)
        .chunks_exact(8)
    {
        let mut b = [0u8; 8];
        b.copy_from_slice(chunk);
        fold = fold.rotate_left(7) ^ u64::from_le_bytes(b);
    }

    // 16 bytes starting 8 bytes before a page boundary.
    let edge = DATA_BASE + 0x8_0000 - 8;
    vm.write(edge, &(t as u128).to_le_bytes());
    let across = vm.read(edge, 16);
    let mut acc = 0u64;
    for (i, byte) in across.iter().enumerate() {
        acc = acc.wrapping_add((*byte as u64) << (i % 8));
    }

    vm.regs[12] = old as u64 ^ fold;
    vm.regs[13] = acc ^ (vm.read_u64(base) >> 3);
    vm.pc = 0x102;
}

/// v128 helpers: load, add/mul, shuffle, shift, dot product, bitselect, store.
pub fn simd(vm: &mut Vm, info: &TickInfo) {
    let t = info.index;
    let off = ((t as usize * 16) % (SEGMENT_COUNT * SEGMENT_BYTES - 16)) as u32;
    let raw = vm.read(DATA_BASE + 0x1000 + off, 16);
    let mut a_bytes = [0u8; 16];
    a_bytes.copy_from_slice(&raw);
    let a = u128::from_le_bytes(a_bytes);
    let b = simd::splat_i32(t as i32);

    let mut c = simd::i32x4_add(a, b);
    c = simd::i32x4_mul(c, simd::splat_i32((t as i32).wrapping_mul(3)));
    c = simd::i16x8_mul(c, simd::i16x8_min_s(a, b));
    c = simd::i8x16_add_sat_s(c, a);

    let mask = [
        (t % 32) as u8,
        ((t + 1) % 32) as u8,
        ((t + 2) % 32) as u8,
        ((t + 3) % 32) as u8,
        ((t + 4) % 32) as u8,
        ((t + 5) % 32) as u8,
        ((t + 6) % 32) as u8,
        ((t + 7) % 32) as u8,
        ((t + 8) % 32) as u8,
        ((t + 9) % 32) as u8,
        ((t + 10) % 32) as u8,
        ((t + 11) % 32) as u8,
        ((t + 12) % 32) as u8,
        ((t + 13) % 32) as u8,
        ((t + 14) % 32) as u8,
        ((t + 15) % 32) as u8,
    ];
    let shuffled = simd::i8x16_shuffle(c, b, mask);
    let shifted = simd::i64x2_shl(shuffled, (t % 64) as u32);
    let dotted = simd::i32x4_dot_i16x8_s(c, shuffled);
    let selected = simd::v128_bitselect(dotted, shifted, c);

    let fa = simd::splat_i32((0x3F80_0000u32 | ((t as u32) & 0x007F_FFFF)) as i32);
    let fb = simd::splat_i32((0x4000_0000u32 | (((t as u32) * 3) & 0x003F_FFFF)) as i32);
    let fsum = simd::f32x4_add_bits(fa, fb);
    let fprod = simd::f32x4_mul_bits(fsum, fa);
    let mixed = simd::v128_xor(simd::v128_and(selected, fprod), simd::v128_or(c, shifted));

    vm.fregs[0] = mixed;
    vm.fregs[1] = simd::v128_bitselect(mixed, a, simd::splat_i32(-1));
    vm.fregs[2] = simd::set_lane_i32(mixed, (t % 4) as usize, simd::lane_i32(dotted, 0));
    vm.globals[3] ^= mixed as u64;

    let out = SCRATCH_BASE + 0x3000 + (t as u32 % 16) * 16;
    vm.write(out, &mixed.to_le_bytes());
    vm.regs[14] =
        simd::lane_i32(mixed, 0) as u64 ^ (simd::lane_i32(mixed, 3) as u64).rotate_left(32);
    vm.regs[15] = simd::lane_i32(dotted, (t % 4) as usize) as u64;
    vm.pc = 0x103;
}

/// Cooperative futex path: expire deadlines, wait with a 3-tick timeout, wake
/// one waiter every 16th tick, requeue one every 16th tick (phase 7).
pub fn futex(vm: &mut Vm, info: &TickInfo) {
    let now = info.index;
    let expired = vm.futex.expire(now);
    let slot = (now % 8) as u32;
    let addr = FUTEX_BASE + slot * 4;

    if now % 16 == 0 {
        vm.futex.set_word(addr, 1);
        vm.write_u32(addr, 1);
        let woken = vm.futex.wake(addr, 1);
        vm.regs[16] = vm.regs[16].wrapping_add(woken as u64);
    } else if now % 16 == 7 {
        vm.futex.set_word(addr, 1);
        vm.write_u32(addr, 1);
        let next = FUTEX_BASE + ((slot + 1) % 8) * 4;
        let moved = vm.futex.requeue(addr, 1, next);
        vm.regs[17] = vm.regs[17].wrapping_add(moved as u64);
    } else {
        let observed = vm.futex.word(addr);
        let outcome = vm.futex.wait(addr, observed, observed, 3, now);
        vm.regs[18] = vm.regs[18]
            .wrapping_mul(31)
            .wrapping_add(outcome.code() as u64)
            .wrapping_add(now);
    }

    vm.regs[19] = vm.regs[19]
        .wrapping_add(expired.len() as u64)
        .wrapping_add(vm.futex.waiters() as u64);
    vm.globals[2] = vm.globals[2].wrapping_add(vm.regs[18] ^ vm.regs[19]);
    vm.pc = 0x104;
}

/// KNOWN NONDETERMINISTIC control: mixes the wall clock into registers and
/// memory. The harness must flag this workload as divergent.
pub fn wall_clock(vm: &mut Vm, info: &TickInfo) {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    let pid = std::process::id() as u64;

    // A short dependency chain so consecutive reads cannot land in the same
    // clock tick, and so the divergence is not sensitive to a coarse timer.
    let mut x = nanos ^ (info.index.wrapping_mul(0x9E37_79B9_7F4A_7C15));
    for _ in 0..2_000 {
        x = x.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
    }

    vm.regs[0] = vm.regs[0].wrapping_add(nanos);
    vm.regs[1] ^= pid.rotate_left(17);
    vm.regs[2] = x;
    vm.write_u64(SCRATCH_BASE + 0x8000, nanos);
    vm.write_u64(SCRATCH_BASE + 0x8008, x);
    vm.pc = 0x106;
}
