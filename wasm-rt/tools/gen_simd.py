#!/usr/bin/env python3
"""Render `wasm-rt/src/simd.rs` from the project's SIMD table, then apply the
local corrections that the table cannot express yet.

The generated half is verbatim `tables.gen_simd_rs()` output (208 ops) — the
same bytes as `benches/src/simd_wasm_rt.rs`, which is what makes the audit
reproducible:

    python -c "import sys;sys.path.insert(0,'benches/tools/wat2rust');\
        from w2r import tables;print(tables.gen_simd_rs())"  # == generated half

The `ADDITIONS` block holds five core wasm SIMD ops the table is missing
(see `report/simd-audit.md` for the table entries to add) plus the
lane-index checks that turn a validation-time immediate violation into a
trap instead of a silently masked lane.

Run from `recovered/rust/wasm-rt`:
    python tools/gen_simd.py
"""
from __future__ import annotations

import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CRATE = os.path.dirname(HERE)
RUST_ROOT = os.path.dirname(CRATE)
TABLE_DIR = os.path.join(RUST_ROOT, "benches", "tools", "wat2rust")

HEADER = '''//! 128-bit SIMD helpers for the translated module.
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
'''

EXTRACT_OLD = '''        #[inline]
        pub fn $name(a: V, lane: u32) -> $t {
            let x: [$t; $k] = unsafe { core::mem::transmute(a) };
            x[(lane as usize) % $k]
        }'''

EXTRACT_NEW = '''        #[inline]
        pub fn $name(a: V, lane: u32) -> $t {
            assert!(
                (lane as usize) < $k,
                "extract_lane: lane {lane} out of range for a {}-lane op",
                $k
            );
            let x: [$t; $k] = unsafe { core::mem::transmute(a) };
            x[lane as usize]
        }'''

REPLACE_OLD = '''        #[inline]
        pub fn $name(a: V, lane: u32, v: $t) -> V {
            let mut x: [$t; $k] = unsafe { core::mem::transmute(a) };
            x[(lane as usize) % $k] = v;
            unsafe { core::mem::transmute(x) }
        }'''

REPLACE_NEW = '''        #[inline]
        pub fn $name(a: V, lane: u32, v: $t) -> V {
            assert!(
                (lane as usize) < $k,
                "replace_lane: lane {lane} out of range for a {}-lane op",
                $k
            );
            let mut x: [$t; $k] = unsafe { core::mem::transmute(a) };
            x[lane as usize] = v;
            unsafe { core::mem::transmute(x) }
        }'''

SHUFFLE_OLD = '''    for i in 0..16 { let j = m[i] as usize; r[i] = if j < 16 { x[j] } else { y[j - 16] }; }'''

SHUFFLE_NEW = '''    for i in 0..16 {
        let j = m[i] as usize;
        assert!(j < 32, "i8x16.shuffle: immediate {} out of range 0..32", j);
        r[i] = if j < 16 { x[j] } else { y[j - 16] };
    }'''

SHIFT_OLD = '''            let sh = (s as u32) % (8 * core::mem::size_of::<$t>() as u32);'''

SHIFT_NEW = '''            // wasm: the shift count is taken modulo the lane width (signed
            // immediates therefore behave as if unsigned).
            let lane_bits = (core::mem::size_of::<$t>() * 8) as u32;
            let sh = (s as u32) % lane_bits;'''

ADDITIONS = '''
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
'''


def main() -> int:
    sys.path.insert(0, TABLE_DIR)
    from w2r import tables  # noqa: PLC0415  (path is set up above)

    body = tables.gen_simd_rs()

    # Drop the generated file's own header (everything up to `pub type V`).
    marker = "pub type V = u128;"
    if marker not in body:
        raise SystemExit("unexpected generator output: no `pub type V` line")
    rest = body[body.index(marker) + len(marker):]

    # The generator's header carries the crate/module attributes; keep exactly
    # one copy, after the `V` alias this script re-emits.
    rest = rest.replace(
        "#![allow(clippy::all, dead_code, unknown_lints, unnecessary_transmutes)]",
        "",
    )
    rest = "#![allow(clippy::all, dead_code, unknown_lints, unnecessary_transmutes)]\n\npub type V = u128;\n" + rest

    for old, new in (
        (EXTRACT_OLD, EXTRACT_NEW),
        (REPLACE_OLD, REPLACE_NEW),
        (SHUFFLE_OLD, SHUFFLE_NEW),
        (SHIFT_OLD, SHIFT_NEW),
    ):
        if old not in rest:
            raise SystemExit("correction target not found:\n%s" % old)
        rest = rest.replace(old, new)

    # `// op count: N` from the generator is now the generated subset only.
    rest = rest.replace(
        "// op count: %d" % len(tables.SIMD),
        "// generated ops: %d (+5 in ADDITIONS)" % len(tables.SIMD),
    )

    out = HEADER + rest + ADDITIONS
    path = os.path.join(CRATE, "src", "simd.rs")
    with open(path, "w", encoding="utf-8", newline="\n") as fh:
        fh.write(out)
    print("wrote %s: %d generated ops + 5 additions" % (path, len(tables.SIMD)))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())