import re

"""Opcode tables for the WAT -> Rust translator.

Everything that knows about wasm semantics lives here; `emitter.py` only knows
about the stack machine and the control-flow shape.

Three tables:

* ``NUM``   - scalar numeric ops   -> inline Rust expression templates
* ``LOADS`` / ``STORES`` - memory access -> `Phantom::read_*` / `write_*`
* ``SIMD``  - 128-bit ops -> `wasm_rt::simd::*` helper calls (the same table is
  used to generate ``wasm-rt/src/simd.rs``, so translator and runtime cannot
  drift apart)
"""

# wasm value type -> Rust type
RUST = {"i32": "i32", "i64": "i64", "f32": "f32", "f64": "f64", "v128": "u128"}

# default (zero) value per wasm type, used for block result bindings
ZERO = {"i32": "0", "i64": "0", "f32": "0.0", "f64": "0.0", "v128": "0"}

# ---------------------------------------------------------------------------
# scalar numeric ops: op -> (operand types, result type, template)
#   template placeholders: {a} {b}
# ---------------------------------------------------------------------------
NUM = {}


def _n(op, args, res, tmpl):
    NUM[op] = (args, res, tmpl)


# --- i32 -------------------------------------------------------------------
_n("i32.eqz", ("i32",), "i32", "(({a} == 0) as i32)")
_n("i32.eq", ("i32", "i32"), "i32", "(({a} == {b}) as i32)")
_n("i32.ne", ("i32", "i32"), "i32", "(({a} != {b}) as i32)")
_n("i32.lt_s", ("i32", "i32"), "i32", "((({a} as i32) < ({b} as i32)) as i32)")
_n("i32.lt_u", ("i32", "i32"), "i32", "((({a} as u32) < ({b} as u32)) as i32)")
_n("i32.gt_s", ("i32", "i32"), "i32", "((({a} as i32) > ({b} as i32)) as i32)")
_n("i32.gt_u", ("i32", "i32"), "i32", "((({a} as u32) > ({b} as u32)) as i32)")
_n("i32.le_s", ("i32", "i32"), "i32", "((({a} as i32) <= ({b} as i32)) as i32)")
_n("i32.le_u", ("i32", "i32"), "i32", "((({a} as u32) <= ({b} as u32)) as i32)")
_n("i32.ge_s", ("i32", "i32"), "i32", "((({a} as i32) >= ({b} as i32)) as i32)")
_n("i32.ge_u", ("i32", "i32"), "i32", "((({a} as u32) >= ({b} as u32)) as i32)")
_n("i32.clz", ("i32",), "i32", "(({a} as u32).leading_zeros() as i32)")
_n("i32.ctz", ("i32",), "i32", "(({a} as u32).trailing_zeros() as i32)")
_n("i32.popcnt", ("i32",), "i32", "(({a} as u32).count_ones() as i32)")
_n("i32.add", ("i32", "i32"), "i32", "({a}.wrapping_add({b}))")
_n("i32.sub", ("i32", "i32"), "i32", "({a}.wrapping_sub({b}))")
_n("i32.mul", ("i32", "i32"), "i32", "({a}.wrapping_mul({b}))")
_n("i32.div_s", ("i32", "i32"), "i32", "rt::i32_div_s({a}, {b})")
_n("i32.div_u", ("i32", "i32"), "i32", "rt::i32_div_u({a}, {b})")
_n("i32.rem_s", ("i32", "i32"), "i32", "rt::i32_rem_s({a}, {b})")
_n("i32.rem_u", ("i32", "i32"), "i32", "rt::i32_rem_u({a}, {b})")
_n("i32.and", ("i32", "i32"), "i32", "({a} & {b})")
_n("i32.or", ("i32", "i32"), "i32", "({a} | {b})")
_n("i32.xor", ("i32", "i32"), "i32", "({a} ^ {b})")
_n("i32.shl", ("i32", "i32"), "i32", "({a}.wrapping_shl(({b} as u32) & 31))")
_n("i32.shr_s", ("i32", "i32"), "i32", "(({a} as i32).wrapping_shr(({b} as u32) & 31))")
_n("i32.shr_u", ("i32", "i32"), "i32", "(({a} as u32).wrapping_shr(({b} as u32) & 31) as i32)")
_n("i32.rotl", ("i32", "i32"), "i32", "(({a} as u32).rotate_left(({b} as u32) & 31) as i32)")
_n("i32.rotr", ("i32", "i32"), "i32", "(({a} as u32).rotate_right(({b} as u32) & 31) as i32)")

# --- i64 -------------------------------------------------------------------
_n("i64.eqz", ("i64",), "i32", "(({a} == 0) as i32)")
_n("i64.eq", ("i64", "i64"), "i32", "(({a} == {b}) as i32)")
_n("i64.ne", ("i64", "i64"), "i32", "(({a} != {b}) as i32)")
_n("i64.lt_s", ("i64", "i64"), "i32", "((({a} as i64) < ({b} as i64)) as i32)")
_n("i64.lt_u", ("i64", "i64"), "i32", "((({a} as u64) < ({b} as u64)) as i32)")
_n("i64.gt_s", ("i64", "i64"), "i32", "((({a} as i64) > ({b} as i64)) as i32)")
_n("i64.gt_u", ("i64", "i64"), "i32", "((({a} as u64) > ({b} as u64)) as i32)")
_n("i64.le_s", ("i64", "i64"), "i32", "((({a} as i64) <= ({b} as i64)) as i32)")
_n("i64.le_u", ("i64", "i64"), "i32", "((({a} as u64) <= ({b} as u64)) as i32)")
_n("i64.ge_s", ("i64", "i64"), "i32", "((({a} as i64) >= ({b} as i64)) as i32)")
_n("i64.ge_u", ("i64", "i64"), "i32", "((({a} as u64) >= ({b} as u64)) as i32)")
_n("i64.clz", ("i64",), "i64", "(({a} as u64).leading_zeros() as i64)")
_n("i64.ctz", ("i64",), "i64", "(({a} as u64).trailing_zeros() as i64)")
_n("i64.popcnt", ("i64",), "i64", "(({a} as u64).count_ones() as i64)")
_n("i64.add", ("i64", "i64"), "i64", "({a}.wrapping_add({b}))")
_n("i64.sub", ("i64", "i64"), "i64", "({a}.wrapping_sub({b}))")
_n("i64.mul", ("i64", "i64"), "i64", "({a}.wrapping_mul({b}))")
_n("i64.div_s", ("i64", "i64"), "i64", "rt::i64_div_s({a}, {b})")
_n("i64.div_u", ("i64", "i64"), "i64", "rt::i64_div_u({a}, {b})")
_n("i64.rem_s", ("i64", "i64"), "i64", "rt::i64_rem_s({a}, {b})")
_n("i64.rem_u", ("i64", "i64"), "i64", "rt::i64_rem_u({a}, {b})")
_n("i64.and", ("i64", "i64"), "i64", "({a} & {b})")
_n("i64.or", ("i64", "i64"), "i64", "({a} | {b})")
_n("i64.xor", ("i64", "i64"), "i64", "({a} ^ {b})")
_n("i64.shl", ("i64", "i64"), "i64", "({a}.wrapping_shl(({b} as u32) & 63))")
_n("i64.shr_s", ("i64", "i64"), "i64", "(({a} as i64).wrapping_shr(({b} as u32) & 63))")
_n("i64.shr_u", ("i64", "i64"), "i64", "(({a} as u64).wrapping_shr(({b} as u32) & 63) as i64)")
_n("i64.rotl", ("i64", "i64"), "i64", "(({a} as u64).rotate_left(({b} as u32) & 63) as i64)")
_n("i64.rotr", ("i64", "i64"), "i64", "(({a} as u64).rotate_right(({b} as u32) & 63) as i64)")

# --- f32 -------------------------------------------------------------------
_n("f32.abs", ("f32",), "f32", "({a}.abs())")
_n("f32.neg", ("f32",), "f32", "(-{a})")
_n("f32.ceil", ("f32",), "f32", "({a}.ceil())")
_n("f32.floor", ("f32",), "f32", "({a}.floor())")
_n("f32.trunc", ("f32",), "f32", "({a}.trunc())")
_n("f32.nearest", ("f32",), "f32", "rt::f32_nearest({a})")
_n("f32.sqrt", ("f32",), "f32", "({a}.sqrt())")
_n("f32.add", ("f32", "f32"), "f32", "({a} + {b})")
_n("f32.sub", ("f32", "f32"), "f32", "({a} - {b})")
_n("f32.mul", ("f32", "f32"), "f32", "({a} * {b})")
_n("f32.div", ("f32", "f32"), "f32", "({a} / {b})")
_n("f32.min", ("f32", "f32"), "f32", "rt::f32_min({a}, {b})")
_n("f32.max", ("f32", "f32"), "f32", "rt::f32_max({a}, {b})")
_n("f32.copysign", ("f32", "f32"), "f32", "({a}.copysign({b}))")
_n("f32.eq", ("f32", "f32"), "i32", "(({a} == {b}) as i32)")
_n("f32.ne", ("f32", "f32"), "i32", "(({a} != {b}) as i32)")
_n("f32.lt", ("f32", "f32"), "i32", "(({a} < {b}) as i32)")
_n("f32.gt", ("f32", "f32"), "i32", "(({a} > {b}) as i32)")
_n("f32.le", ("f32", "f32"), "i32", "(({a} <= {b}) as i32)")
_n("f32.ge", ("f32", "f32"), "i32", "(({a} >= {b}) as i32)")

# --- f64 -------------------------------------------------------------------
_n("f64.abs", ("f64",), "f64", "({a}.abs())")
_n("f64.neg", ("f64",), "f64", "(-{a})")
_n("f64.ceil", ("f64",), "f64", "({a}.ceil())")
_n("f64.floor", ("f64",), "f64", "({a}.floor())")
_n("f64.trunc", ("f64",), "f64", "({a}.trunc())")
_n("f64.nearest", ("f64",), "f64", "rt::f64_nearest({a})")
_n("f64.sqrt", ("f64",), "f64", "({a}.sqrt())")
_n("f64.add", ("f64", "f64"), "f64", "({a} + {b})")
_n("f64.sub", ("f64", "f64"), "f64", "({a} - {b})")
_n("f64.mul", ("f64", "f64"), "f64", "({a} * {b})")
_n("f64.div", ("f64", "f64"), "f64", "({a} / {b})")
_n("f64.min", ("f64", "f64"), "f64", "rt::f64_min({a}, {b})")
_n("f64.max", ("f64", "f64"), "f64", "rt::f64_max({a}, {b})")
_n("f64.copysign", ("f64", "f64"), "f64", "({a}.copysign({b}))")
_n("f64.eq", ("f64", "f64"), "i32", "(({a} == {b}) as i32)")
_n("f64.ne", ("f64", "f64"), "i32", "(({a} != {b}) as i32)")
_n("f64.lt", ("f64", "f64"), "i32", "(({a} < {b}) as i32)")
_n("f64.gt", ("f64", "f64"), "i32", "(({a} > {b}) as i32)")
_n("f64.le", ("f64", "f64"), "i32", "(({a} <= {b}) as i32)")
_n("f64.ge", ("f64", "f64"), "i32", "(({a} >= {b}) as i32)")

# --- conversions -----------------------------------------------------------
_n("i32.wrap_i64", ("i64",), "i32", "({a} as i32)")
_n("i64.extend_i32_s", ("i32",), "i64", "(({a} as i32) as i64)")
_n("i64.extend_i32_u", ("i32",), "i64", "(({a} as u32) as i64)")
_n("i32.trunc_f32_s", ("f32",), "i32", "rt::i32_trunc_f32_s({a})")
_n("i32.trunc_f32_u", ("f32",), "i32", "rt::i32_trunc_f32_u({a})")
_n("i32.trunc_f64_s", ("f64",), "i32", "rt::i32_trunc_f64_s({a})")
_n("i32.trunc_f64_u", ("f64",), "i32", "rt::i32_trunc_f64_u({a})")
_n("i64.trunc_f32_s", ("f32",), "i64", "rt::i64_trunc_f32_s({a})")
_n("i64.trunc_f32_u", ("f32",), "i64", "rt::i64_trunc_f32_u({a})")
_n("i64.trunc_f64_s", ("f64",), "i64", "rt::i64_trunc_f64_s({a})")
_n("i64.trunc_f64_u", ("f64",), "i64", "rt::i64_trunc_f64_u({a})")
_n("i32.trunc_sat_f32_s", ("f32",), "i32", "rt::i32_trunc_sat_f32_s({a})")
_n("i32.trunc_sat_f32_u", ("f32",), "i32", "rt::i32_trunc_sat_f32_u({a})")
_n("i32.trunc_sat_f64_s", ("f64",), "i32", "rt::i32_trunc_sat_f64_s({a})")
_n("i32.trunc_sat_f64_u", ("f64",), "i32", "rt::i32_trunc_sat_f64_u({a})")
_n("i64.trunc_sat_f32_s", ("f32",), "i64", "rt::i64_trunc_sat_f32_s({a})")
_n("i64.trunc_sat_f32_u", ("f32",), "i64", "rt::i64_trunc_sat_f32_u({a})")
_n("i64.trunc_sat_f64_s", ("f64",), "i64", "rt::i64_trunc_sat_f64_s({a})")
_n("i64.trunc_sat_f64_u", ("f64",), "i64", "rt::i64_trunc_sat_f64_u({a})")
_n("f32.convert_i32_s", ("i32",), "f32", "(({a} as i32) as f32)")
_n("f32.convert_i32_u", ("i32",), "f32", "(({a} as u32) as f32)")
_n("f32.convert_i64_s", ("i64",), "f32", "(({a} as i64) as f32)")
_n("f32.convert_i64_u", ("i64",), "f32", "(({a} as u64) as f32)")
_n("f64.convert_i32_s", ("i32",), "f64", "(({a} as i32) as f64)")
_n("f64.convert_i32_u", ("i32",), "f64", "(({a} as u32) as f64)")
_n("f64.convert_i64_s", ("i64",), "f64", "(({a} as i64) as f64)")
_n("f64.convert_i64_u", ("i64",), "f64", "(({a} as u64) as f64)")
_n("f32.demote_f64", ("f64",), "f32", "({a} as f32)")
_n("f64.promote_f32", ("f32",), "f64", "({a} as f64)")
_n("i32.reinterpret_f32", ("f32",), "i32", "({a}.to_bits() as i32)")
_n("i64.reinterpret_f64", ("f64",), "i64", "({a}.to_bits() as i64)")
_n("f32.reinterpret_i32", ("i32",), "f32", "f32::from_bits({a} as u32)")
_n("f64.reinterpret_i64", ("i64",), "f64", "f64::from_bits({a} as u64)")
_n("i32.extend8_s", ("i32",), "i32", "((({a} as i8) as i32))")
_n("i32.extend16_s", ("i32",), "i32", "((({a} as i16) as i32))")
_n("i64.extend8_s", ("i64",), "i64", "((({a} as i8) as i64))")
_n("i64.extend16_s", ("i64",), "i64", "((({a} as i16) as i64))")
_n("i64.extend32_s", ("i64",), "i64", "((({a} as i32) as i64))")

# ---------------------------------------------------------------------------
# memory access: op -> (helper, load-cast, result wasm type)
#   emitted as: ph.<helper>(addr) <cast>
# ---------------------------------------------------------------------------
LOADS = {
    "i32.load": ("read_u32", " as i32", "i32"),
    "i32.load8_s": ("read_i8", " as i32", "i32"),
    "i32.load8_u": ("read_u8", " as i32", "i32"),
    "i32.load16_s": ("read_i16", " as i32", "i32"),
    "i32.load16_u": ("read_u16", " as i32", "i32"),
    "i64.load": ("read_u64", " as i64", "i64"),
    "i64.load8_s": ("read_i8", " as i64", "i64"),
    "i64.load8_u": ("read_u8", " as i64", "i64"),
    "i64.load16_s": ("read_i16", " as i64", "i64"),
    "i64.load16_u": ("read_u16", " as i64", "i64"),
    "i64.load32_s": ("read_i32", " as i64", "i64"),
    "i64.load32_u": ("read_u32", " as i64", "i64"),
    "f32.load": ("read_f32", "", "f32"),
    "f64.load": ("read_f64", "", "f64"),
    "v128.load": ("read_v128", "", "v128"),
    "v128.load8x8_s": ("read_8x8_s", "", "v128"),
    "v128.load8x8_u": ("read_8x8_u", "", "v128"),
    "v128.load16x4_s": ("read_16x4_s", "", "v128"),
    "v128.load16x4_u": ("read_16x4_u", "", "v128"),
    "v128.load32x2_s": ("read_32x2_s", "", "v128"),
    "v128.load32x2_u": ("read_32x2_u", "", "v128"),
}

# v128.loadN_splat / loadN_zero -> (width, zero_flag)
SPLAT_LOADS = {
    "v128.load8_splat": (8, False),
    "v128.load16_splat": (16, False),
    "v128.load32_splat": (32, False),
    "v128.load64_splat": (64, False),
    "v128.load32_zero": (32, True),
    "v128.load64_zero": (64, True),
}

# v128.loadN_lane / v128.storeN_lane -> width
LANE_LOADS = {
    "v128.load8_lane": 8,
    "v128.load16_lane": 16,
    "v128.load32_lane": 32,
    "v128.load64_lane": 64,
}
LANE_STORES = {
    "v128.store8_lane": 8,
    "v128.store16_lane": 16,
    "v128.store32_lane": 32,
    "v128.store64_lane": 64,
}

STORES = {
    "i32.store": ("write_u32", " as u32"),
    "i32.store8": ("write_u8", " as u8"),
    "i32.store16": ("write_u16", " as u16"),
    "i64.store": ("write_u64", " as u64"),
    "i64.store8": ("write_u8", " as u8"),
    "i64.store16": ("write_u16", " as u16"),
    "i64.store32": ("write_u32", " as u32"),
    "f32.store": ("write_f32", ""),
    "f64.store": ("write_f64", ""),
    "v128.store": ("write_v128", ""),
}

# atomic ops: op -> (kind, helper, operand types, result type)
#   kind "load"  : ph.<helper>(addr)                      -> i32/i64
#   kind "store" : ph.<helper>(addr, v)                   -> ()
#   kind "rmw"   : ph.<helper>(addr, v)                   -> i32/i64
ATOMIC = {
    "i32.atomic.load": ("load", "a_load_u32", (), "i32"),
    "i32.atomic.load8_u": ("load", "a_load_u8", (), "i32"),
    "i32.atomic.load16_u": ("load", "a_load_u16", (), "i32"),
    "i64.atomic.load": ("load", "a_load_u64", (), "i64"),
    "i64.atomic.load8_u": ("load", "a_load_u8", (), "i64"),
    "i64.atomic.load16_u": ("load", "a_load_u16", (), "i64"),
    "i64.atomic.load32_u": ("load", "a_load_u32", (), "i64"),
    "i32.atomic.store": ("store", "a_store_u32", ("i32",), ""),
    "i32.atomic.store8": ("store", "a_store_u8", ("i32",), ""),
    "i32.atomic.store16": ("store", "a_store_u16", ("i32",), ""),
    "i64.atomic.store": ("store", "a_store_u64", ("i64",), ""),
    "i64.atomic.store8": ("store", "a_store_u8", ("i64",), ""),
    "i64.atomic.store16": ("store", "a_store_u16", ("i64",), ""),
    "i64.atomic.store32": ("store", "a_store_u32", ("i64",), ""),
    "i32.atomic.rmw.add": ("rmw", "a_rmw_add_u32", ("i32",), "i32"),
    "i32.atomic.rmw.sub": ("rmw", "a_rmw_sub_u32", ("i32",), "i32"),
    "i32.atomic.rmw.and": ("rmw", "a_rmw_and_u32", ("i32",), "i32"),
    "i32.atomic.rmw.or": ("rmw", "a_rmw_or_u32", ("i32",), "i32"),
    "i32.atomic.rmw.xor": ("rmw", "a_rmw_xor_u32", ("i32",), "i32"),
    "i32.atomic.rmw.xchg": ("rmw", "a_rmw_xchg_u32", ("i32",), "i32"),
    "i32.atomic.rmw8.add_u": ("rmw", "a_rmw_add_u8", ("i32",), "i32"),
    "i32.atomic.rmw8.sub_u": ("rmw", "a_rmw_sub_u8", ("i32",), "i32"),
    "i32.atomic.rmw8.and_u": ("rmw", "a_rmw_and_u8", ("i32",), "i32"),
    "i32.atomic.rmw8.or_u": ("rmw", "a_rmw_or_u8", ("i32",), "i32"),
    "i32.atomic.rmw8.xor_u": ("rmw", "a_rmw_xor_u8", ("i32",), "i32"),
    "i32.atomic.rmw8.xchg_u": ("rmw", "a_rmw_xchg_u8", ("i32",), "i32"),
    "i32.atomic.rmw16.add_u": ("rmw", "a_rmw_add_u16", ("i32",), "i32"),
    "i32.atomic.rmw16.sub_u": ("rmw", "a_rmw_sub_u16", ("i32",), "i32"),
    "i32.atomic.rmw16.and_u": ("rmw", "a_rmw_and_u16", ("i32",), "i32"),
    "i32.atomic.rmw16.or_u": ("rmw", "a_rmw_or_u16", ("i32",), "i32"),
    "i32.atomic.rmw16.xor_u": ("rmw", "a_rmw_xor_u16", ("i32",), "i32"),
    "i32.atomic.rmw16.xchg_u": ("rmw", "a_rmw_xchg_u16", ("i32",), "i32"),
    "i64.atomic.rmw.add": ("rmw", "a_rmw_add_u64", ("i64",), "i64"),
    "i64.atomic.rmw.sub": ("rmw", "a_rmw_sub_u64", ("i64",), "i64"),
    "i64.atomic.rmw.and": ("rmw", "a_rmw_and_u64", ("i64",), "i64"),
    "i64.atomic.rmw.or": ("rmw", "a_rmw_or_u64", ("i64",), "i64"),
    "i64.atomic.rmw.xor": ("rmw", "a_rmw_xor_u64", ("i64",), "i64"),
    "i64.atomic.rmw.xchg": ("rmw", "a_rmw_xchg_u64", ("i64",), "i64"),
    "i64.atomic.rmw8.add_u": ("rmw", "a_rmw_add_u8", ("i64",), "i64"),
    "i64.atomic.rmw8.xchg_u": ("rmw", "a_rmw_xchg_u8", ("i64",), "i64"),
    "i64.atomic.rmw32.add_u": ("rmw", "a_rmw_add_u32", ("i64",), "i64"),
    "i64.atomic.rmw32.xchg_u": ("rmw", "a_rmw_xchg_u32", ("i64",), "i64"),
    # compare-exchange is special-cased in the emitter (2 operands)
    "i32.atomic.rmw8.cmpxchg_u": ("cmpxchg", "a_rmw_cmpxchg_u8", ("i32", "i32"), "i32"),
    "i32.atomic.rmw16.cmpxchg_u": ("cmpxchg", "a_rmw_cmpxchg_u16", ("i32", "i32"), "i32"),
    "i32.atomic.rmw.cmpxchg": ("cmpxchg", "a_rmw_cmpxchg_u32", ("i32", "i32"), "i32"),
    "i64.atomic.rmw8.cmpxchg_u": ("cmpxchg", "a_rmw_cmpxchg_u8", ("i64", "i64"), "i64"),
    "i64.atomic.rmw32.cmpxchg_u": ("cmpxchg", "a_rmw_cmpxchg_u32", ("i64", "i64"), "i64"),
    "i64.atomic.rmw.cmpxchg": ("cmpxchg", "a_rmw_cmpxchg_u64", ("i64", "i64"), "i64"),
    "memory.atomic.notify": ("notify", "a_notify", ("i32",), "i32"),
    "memory.atomic.wait32": ("wait", "a_wait32", ("i32", "i64"), "i32"),
}

# ---------------------------------------------------------------------------
# SIMD: op -> spec tuple, also used to generate wasm-rt/src/simd.rs
#
#   ("b2", ty, k, closure)        (V,V)->V   lane-wise binary op
#   ("c2", sty, uty, k, closure)  (V,V)->V   lane-wise compare -> all-ones mask
#   ("u1", ty, k, closure)        (V)->V     lane-wise unary op
#   ("sh", ty, k, closure)        (V,i32)->V lane-wise shift
#   ("spl", ty)                   (ty)->V
#   ("ex", ty, k)                 (V,u32)->ty
#   ("rep", ty, k)                (V,ty,u32)->V
#   ("tst", code)                 (V)->i32  any_true/all_true/bitmask
#   ("raw", body, sig)            hand-written
# ---------------------------------------------------------------------------
SIMD = {}


def _s(op, *spec):
    SIMD[op] = spec


# --- v128 bitwise ----------------------------------------------------------
_s("v128.not", "raw", "pub fn v128_not(a: V) -> V { !a }")
_s("v128.and", "raw", "pub fn v128_and(a: V, b: V) -> V { a & b }")
_s("v128.andnot", "raw", "pub fn v128_andnot(a: V, b: V) -> V { a & !b }")
_s("v128.or", "raw", "pub fn v128_or(a: V, b: V) -> V { a | b }")
_s("v128.xor", "raw", "pub fn v128_xor(a: V, b: V) -> V { a ^ b }")
_s(
    "v128.bitselect",
    "raw",
    "pub fn v128_bitselect(a: V, b: V, c: V) -> V { (a & c) | (b & !c) }",
)
_s(
    "v128.any_true",
    "raw",
    "pub fn v128_any_true(a: V) -> i32 { (a != 0) as i32 }",
)
_s(
    "i8x16.shuffle",
    "raw",
    """#[inline(always)]
pub fn i8x16_shuffle(a: V, b: V, imm: V) -> V {
    let x: [u8; 16] = unpack_u8(a);
    let y: [u8; 16] = unpack_u8(b);
    let m: [u8; 16] = unpack_u8(imm);
    let mut r = [0u8; 16];
    for i in 0..16 { let j = m[i] as usize; r[i] = if j < 16 { x[j] } else { y[j - 16] }; }
    pack_u8(r)
}""",
)
_s(
    "i8x16.swizzle",
    "raw",
    """pub fn i8x16_swizzle(a: V, b: V) -> V {
    let x: [u8; 16] = unpack_u8(a);
    let m: [u8; 16] = unpack_u8(b);
    let mut r = [0u8; 16];
    for i in 0..16 { let j = m[i] as usize; r[i] = if j < 16 { x[j] } else { 0 }; }
    pack_u8(r)
}""",
)

# --- i8x16 -----------------------------------------------------------------
_s("i8x16.splat", "spl", "i8")
_s("i8x16.extract_lane_s", "ex", "i8", 16)
_s("i8x16.extract_lane_u", "ex", "u8", 16)
_s("i8x16.replace_lane", "rep", "i8", 16)
_s("i8x16.add", "b2", "i8", 16, "|a, b| a.wrapping_add(b)")
_s("i8x16.sub", "b2", "i8", 16, "|a, b| a.wrapping_sub(b)")
_s("i8x16.min_s", "b2", "i8", 16, "|a, b| if a < b { a } else { b }")
_s("i8x16.min_u", "b2", "u8", 16, "|a, b| if a < b { a } else { b }")
_s("i8x16.max_s", "b2", "i8", 16, "|a, b| if a > b { a } else { b }")
_s("i8x16.max_u", "b2", "u8", 16, "|a, b| if a > b { a } else { b }")
_s("i8x16.eq", "c2", "i8", "u8", 16, "|a, b| a == b")
_s("i8x16.ne", "c2", "i8", "u8", 16, "|a, b| a != b")
_s("i8x16.lt_s", "c2", "i8", "u8", 16, "|a, b| a < b")
_s("i8x16.lt_u", "c2", "u8", "u8", 16, "|a, b| a < b")
_s("i8x16.gt_s", "c2", "i8", "u8", 16, "|a, b| a > b")
_s("i8x16.gt_u", "c2", "u8", "u8", 16, "|a, b| a > b")
_s("i8x16.le_s", "c2", "i8", "u8", 16, "|a, b| a <= b")
_s("i8x16.le_u", "c2", "u8", "u8", 16, "|a, b| a <= b")
_s("i8x16.ge_s", "c2", "i8", "u8", 16, "|a, b| a >= b")
_s("i8x16.ge_u", "c2", "u8", "u8", 16, "|a, b| a >= b")
_s("i8x16.abs", "u1", "i8", 16, "|a| a.wrapping_abs()")
_s("i8x16.neg", "u1", "i8", 16, "|a| a.wrapping_neg()")
_s("i8x16.popcnt", "u1", "u8", 16, "|a| a.count_ones() as u8")
_s("i8x16.shl", "sh", "i8", 16, "|a, s| a.wrapping_shl(s)")
_s("i8x16.shr_s", "sh", "i8", 16, "|a, s| a.wrapping_shr(s)")
_s("i8x16.shr_u", "sh", "u8", 16, "|a, s| a.wrapping_shr(s)")
_s("i8x16.add_sat_s", "b2", "i8", 16, "|a, b| a.saturating_add(b)")
_s("i8x16.add_sat_u", "b2", "u8", 16, "|a, b| a.saturating_add(b)")
_s("i8x16.sub_sat_s", "b2", "i8", 16, "|a, b| a.saturating_sub(b)")
_s("i8x16.sub_sat_u", "b2", "u8", 16, "|a, b| a.saturating_sub(b)")
_s(
    "i8x16.avgr_u",
    "b2",
    "u8",
    16,
    "|a, b| (((a as u16) + (b as u16) + 1) >> 1) as u8",
)
_s("i8x16.all_true", "tst", "all_u8")
_s("i8x16.bitmask", "tst", "mask_u8")
_s(
    "i8x16.narrow_i16x8_s",
    "raw",
    """pub fn i8x16_narrow_i16x8_s(a: V, b: V) -> V {
    let x: [i16; 8] = unpack_i16(a);
    let y: [i16; 8] = unpack_i16(b);
    let mut r = [0i8; 16];
    for i in 0..8 { r[i] = clamp_i8(x[i] as i32); r[8 + i] = clamp_i8(y[i] as i32); }
    pack_i8(r)
}""",
)
_s(
    "i8x16.narrow_i16x8_u",
    "raw",
    """pub fn i8x16_narrow_i16x8_u(a: V, b: V) -> V {
    let x: [i16; 8] = unpack_i16(a);
    let y: [i16; 8] = unpack_i16(b);
    let mut r = [0i8; 16];
    for i in 0..8 { r[i] = clamp_u8(x[i] as i32) as i8; r[8 + i] = clamp_u8(y[i] as i32) as i8; }
    pack_i8(r)
}""",
)

# --- i16x8 -----------------------------------------------------------------
_s("i16x8.splat", "spl", "i16")
_s("i16x8.extract_lane_s", "ex", "i16", 8)
_s("i16x8.extract_lane_u", "ex", "u16", 8)
_s("i16x8.replace_lane", "rep", "i16", 8)
_s("i16x8.add", "b2", "i16", 8, "|a, b| a.wrapping_add(b)")
_s("i16x8.sub", "b2", "i16", 8, "|a, b| a.wrapping_sub(b)")
_s("i16x8.mul", "b2", "i16", 8, "|a, b| a.wrapping_mul(b)")
_s("i16x8.min_s", "b2", "i16", 8, "|a, b| if a < b { a } else { b }")
_s("i16x8.min_u", "b2", "u16", 8, "|a, b| if a < b { a } else { b }")
_s("i16x8.max_s", "b2", "i16", 8, "|a, b| if a > b { a } else { b }")
_s("i16x8.max_u", "b2", "u16", 8, "|a, b| if a > b { a } else { b }")
_s("i16x8.eq", "c2", "i16", "u16", 8, "|a, b| a == b")
_s("i16x8.ne", "c2", "i16", "u16", 8, "|a, b| a != b")
_s("i16x8.lt_s", "c2", "i16", "u16", 8, "|a, b| a < b")
_s("i16x8.lt_u", "c2", "u16", "u16", 8, "|a, b| a < b")
_s("i16x8.gt_s", "c2", "i16", "u16", 8, "|a, b| a > b")
_s("i16x8.gt_u", "c2", "u16", "u16", 8, "|a, b| a > b")
_s("i16x8.le_s", "c2", "i16", "u16", 8, "|a, b| a <= b")
_s("i16x8.le_u", "c2", "u16", "u16", 8, "|a, b| a <= b")
_s("i16x8.ge_s", "c2", "i16", "u16", 8, "|a, b| a >= b")
_s("i16x8.ge_u", "c2", "u16", "u16", 8, "|a, b| a >= b")
_s("i16x8.abs", "u1", "i16", 8, "|a| a.wrapping_abs()")
_s("i16x8.neg", "u1", "i16", 8, "|a| a.wrapping_neg()")
_s("i16x8.shl", "sh", "i16", 8, "|a, s| a.wrapping_shl(s)")
_s("i16x8.shr_s", "sh", "i16", 8, "|a, s| a.wrapping_shr(s)")
_s("i16x8.shr_u", "sh", "u16", 8, "|a, s| a.wrapping_shr(s)")
_s("i16x8.add_sat_s", "b2", "i16", 8, "|a, b| a.saturating_add(b)")
_s("i16x8.add_sat_u", "b2", "u16", 8, "|a, b| a.saturating_add(b)")
_s("i16x8.sub_sat_s", "b2", "i16", 8, "|a, b| a.saturating_sub(b)")
_s("i16x8.sub_sat_u", "b2", "u16", 8, "|a, b| a.saturating_sub(b)")
_s(
    "i16x8.avgr_u",
    "b2",
    "u16",
    8,
    "|a, b| (((a as u32) + (b as u32) + 1) >> 1) as u16",
)
_s("i16x8.all_true", "tst", "all_u16")
_s("i16x8.bitmask", "tst", "mask_u16")
_s(
    "i16x8.q15mulr_sat_s",
    "b2",
    "i16",
    8,
    "|a, b| ((((a as i32) * (b as i32)) + 0x4000) >> 15).clamp(-32768, 32767) as i16",
)
_s(
    "i16x8.narrow_i32x4_s",
    "raw",
    """pub fn i16x8_narrow_i32x4_s(a: V, b: V) -> V {
    let x: [i32; 4] = unpack_i32(a);
    let y: [i32; 4] = unpack_i32(b);
    let mut r = [0i16; 8];
    for i in 0..4 { r[i] = clamp_i16(x[i]); r[4 + i] = clamp_i16(y[i]); }
    pack_i16(r)
}""",
)
_s(
    "i16x8.narrow_i32x4_u",
    "raw",
    """pub fn i16x8_narrow_i32x4_u(a: V, b: V) -> V {
    let x: [i32; 4] = unpack_i32(a);
    let y: [i32; 4] = unpack_i32(b);
    let mut r = [0i16; 8];
    for i in 0..4 { r[i] = clamp_u16(x[i]) as i16; r[4 + i] = clamp_u16(y[i]) as i16; }
    pack_i16(r)
}""",
)
for _half in ("low", "high"):
    _s(
        "i16x8.extend_%s_i8x16_s" % _half,
        "raw",
        """pub fn i16x8_extend_%s_i8x16_s(a: V) -> V {
    let x: [i8; 16] = unpack_i8(a);
    let o = %d;
    let mut r = [0i16; 8];
    for i in 0..8 { r[i] = x[o + i] as i16; }
    pack_i16(r)
}"""
        % (_half, 0 if _half == "low" else 8),
    )
    _s(
        "i16x8.extend_%s_i8x16_u" % _half,
        "raw",
        """pub fn i16x8_extend_%s_i8x16_u(a: V) -> V {
    let x: [u8; 16] = unpack_u8(a);
    let o = %d;
    let mut r = [0u16; 8];
    for i in 0..8 { r[i] = x[o + i] as u16; }
    pack_u16(r)
}"""
        % (_half, 0 if _half == "low" else 8),
    )
    _s(
        "i16x8.extmul_%s_i8x16_s" % _half,
        "raw",
        """pub fn i16x8_extmul_%s_i8x16_s(a: V, b: V) -> V {
    let x: [i8; 16] = unpack_i8(a);
    let y: [i8; 16] = unpack_i8(b);
    let o = %d;
    let mut r = [0i16; 8];
    for i in 0..8 { r[i] = (x[o + i] as i16).wrapping_mul(y[o + i] as i16); }
    pack_i16(r)
}"""
        % (_half, 0 if _half == "low" else 8),
    )
    _s(
        "i16x8.extmul_%s_i8x16_u" % _half,
        "raw",
        """pub fn i16x8_extmul_%s_i8x16_u(a: V, b: V) -> V {
    let x: [u8; 16] = unpack_u8(a);
    let y: [u8; 16] = unpack_u8(b);
    let o = %d;
    let mut r = [0u16; 8];
    for i in 0..8 { r[i] = (x[o + i] as u16).wrapping_mul(y[o + i] as u16); }
    pack_u16(r)
}"""
        % (_half, 0 if _half == "low" else 8),
    )

# --- i32x4 -----------------------------------------------------------------
_s("i32x4.splat", "spl", "i32")
_s("i32x4.extract_lane", "ex", "i32", 4)
_s("i32x4.replace_lane", "rep", "i32", 4)
_s("i32x4.add", "b2", "i32", 4, "|a, b| a.wrapping_add(b)")
_s("i32x4.sub", "b2", "i32", 4, "|a, b| a.wrapping_sub(b)")
_s("i32x4.mul", "b2", "i32", 4, "|a, b| a.wrapping_mul(b)")
_s("i32x4.min_s", "b2", "i32", 4, "|a, b| if a < b { a } else { b }")
_s("i32x4.min_u", "b2", "u32", 4, "|a, b| if a < b { a } else { b }")
_s("i32x4.max_s", "b2", "i32", 4, "|a, b| if a > b { a } else { b }")
_s("i32x4.max_u", "b2", "u32", 4, "|a, b| if a > b { a } else { b }")
_s("i32x4.eq", "c2", "i32", "u32", 4, "|a, b| a == b")
_s("i32x4.ne", "c2", "i32", "u32", 4, "|a, b| a != b")
_s("i32x4.lt_s", "c2", "i32", "u32", 4, "|a, b| a < b")
_s("i32x4.lt_u", "c2", "u32", "u32", 4, "|a, b| a < b")
_s("i32x4.gt_s", "c2", "i32", "u32", 4, "|a, b| a > b")
_s("i32x4.gt_u", "c2", "u32", "u32", 4, "|a, b| a > b")
_s("i32x4.le_s", "c2", "i32", "u32", 4, "|a, b| a <= b")
_s("i32x4.le_u", "c2", "u32", "u32", 4, "|a, b| a <= b")
_s("i32x4.ge_s", "c2", "i32", "u32", 4, "|a, b| a >= b")
_s("i32x4.ge_u", "c2", "u32", "u32", 4, "|a, b| a >= b")
_s("i32x4.abs", "u1", "i32", 4, "|a| a.wrapping_abs()")
_s("i32x4.neg", "u1", "i32", 4, "|a| a.wrapping_neg()")
_s("i32x4.shl", "sh", "i32", 4, "|a, s| a.wrapping_shl(s)")
_s("i32x4.shr_s", "sh", "i32", 4, "|a, s| a.wrapping_shr(s)")
_s("i32x4.shr_u", "sh", "u32", 4, "|a, s| a.wrapping_shr(s)")
_s("i32x4.all_true", "tst", "all_u32")
_s("i32x4.bitmask", "tst", "mask_u32")
_s(
    "i32x4.trunc_sat_f32x4_s",
    "raw",
    """pub fn i32x4_trunc_sat_f32x4_s(a: V) -> V {
    let x: [f32; 4] = unpack_f32(a);
    let mut r = [0i32; 4];
    for i in 0..4 {
        let f = x[i];
        r[i] = if f.is_nan() { 0 } else if f <= -2147483648.0 { i32::MIN }
               else if f >= 2147483648.0 { i32::MAX } else { f as i32 };
    }
    pack_i32(r)
}""",
)
_s(
    "i32x4.trunc_sat_f32x4_u",
    "raw",
    """pub fn i32x4_trunc_sat_f32x4_u(a: V) -> V {
    let x: [f32; 4] = unpack_f32(a);
    let mut r = [0i32; 4];
    for i in 0..4 {
        let f = x[i];
        r[i] = if f.is_nan() || f <= 0.0 { 0 }
               else if f >= 4294967296.0 { -1 } else { f as u32 as i32 };
    }
    pack_i32(r)
}""",
)
for _half in ("low", "high"):
    _s(
        "i32x4.extend_%s_i16x8_s" % _half,
        "raw",
        """pub fn i32x4_extend_%s_i16x8_s(a: V) -> V {
    let x: [i16; 8] = unpack_i16(a);
    let o = %d;
    let mut r = [0i32; 4];
    for i in 0..4 { r[i] = x[o + i] as i32; }
    pack_i32(r)
}"""
        % (_half, 0 if _half == "low" else 4),
    )
    _s(
        "i32x4.extend_%s_i16x8_u" % _half,
        "raw",
        """pub fn i32x4_extend_%s_i16x8_u(a: V) -> V {
    let x: [u16; 8] = unpack_u16(a);
    let o = %d;
    let mut r = [0u32; 4];
    for i in 0..4 { r[i] = x[o + i] as u32; }
    pack_u32(r)
}"""
        % (_half, 0 if _half == "low" else 4),
    )
    _s(
        "i32x4.extmul_%s_i16x8_s" % _half,
        "raw",
        """pub fn i32x4_extmul_%s_i16x8_s(a: V, b: V) -> V {
    let x: [i16; 8] = unpack_i16(a);
    let y: [i16; 8] = unpack_i16(b);
    let o = %d;
    let mut r = [0i32; 4];
    for i in 0..4 { r[i] = (x[o + i] as i32).wrapping_mul(y[o + i] as i32); }
    pack_i32(r)
}"""
        % (_half, 0 if _half == "low" else 4),
    )
    _s(
        "i32x4.extmul_%s_i16x8_u" % _half,
        "raw",
        """pub fn i32x4_extmul_%s_i16x8_u(a: V, b: V) -> V {
    let x: [u16; 8] = unpack_u16(a);
    let y: [u16; 8] = unpack_u16(b);
    let o = %d;
    let mut r = [0u32; 4];
    for i in 0..4 { r[i] = (x[o + i] as u32).wrapping_mul(y[o + i] as u32); }
    pack_u32(r)
}"""
        % (_half, 0 if _half == "low" else 4),
    )

# --- i64x2 -----------------------------------------------------------------
_s("i64x2.splat", "spl", "i64")
_s("i64x2.extract_lane", "ex", "i64", 2)
_s("i64x2.replace_lane", "rep", "i64", 2)
_s("i64x2.add", "b2", "i64", 2, "|a, b| a.wrapping_add(b)")
_s("i64x2.sub", "b2", "i64", 2, "|a, b| a.wrapping_sub(b)")
_s("i64x2.mul", "b2", "i64", 2, "|a, b| a.wrapping_mul(b)")
_s("i64x2.eq", "c2", "i64", "u64", 2, "|a, b| a == b")
_s("i64x2.ne", "c2", "i64", "u64", 2, "|a, b| a != b")
_s("i64x2.lt_s", "c2", "i64", "u64", 2, "|a, b| a < b")
_s("i64x2.gt_s", "c2", "i64", "u64", 2, "|a, b| a > b")
_s("i64x2.le_s", "c2", "i64", "u64", 2, "|a, b| a <= b")
_s("i64x2.ge_s", "c2", "i64", "u64", 2, "|a, b| a >= b")
_s("i64x2.abs", "u1", "i64", 2, "|a| a.wrapping_abs()")
_s("i64x2.neg", "u1", "i64", 2, "|a| a.wrapping_neg()")
_s("i64x2.shl", "sh", "i64", 2, "|a, s| a.wrapping_shl(s)")
_s("i64x2.shr_s", "sh", "i64", 2, "|a, s| a.wrapping_shr(s)")
_s("i64x2.shr_u", "sh", "u64", 2, "|a, s| a.wrapping_shr(s)")
_s("i64x2.all_true", "tst", "all_u64")
_s("i64x2.bitmask", "tst", "mask_u64")
for _half in ("low", "high"):
    _s(
        "i64x2.extend_%s_i32x4_s" % _half,
        "raw",
        """pub fn i64x2_extend_%s_i32x4_s(a: V) -> V {
    let x: [i32; 4] = unpack_i32(a);
    let o = %d;
    let mut r = [0i64; 2];
    for i in 0..2 { r[i] = x[o + i] as i64; }
    pack_i64(r)
}"""
        % (_half, 0 if _half == "low" else 2),
    )
    _s(
        "i64x2.extend_%s_i32x4_u" % _half,
        "raw",
        """pub fn i64x2_extend_%s_i32x4_u(a: V) -> V {
    let x: [u32; 4] = unpack_u32(a);
    let o = %d;
    let mut r = [0u64; 2];
    for i in 0..2 { r[i] = x[o + i] as u64; }
    pack_u64(r)
}"""
        % (_half, 0 if _half == "low" else 2),
    )
    _s(
        "i64x2.extmul_%s_i32x4_s" % _half,
        "raw",
        """pub fn i64x2_extmul_%s_i32x4_s(a: V, b: V) -> V {
    let x: [i32; 4] = unpack_i32(a);
    let y: [i32; 4] = unpack_i32(b);
    let o = %d;
    let mut r = [0i64; 2];
    for i in 0..2 { r[i] = (x[o + i] as i64).wrapping_mul(y[o + i] as i64); }
    pack_i64(r)
}"""
        % (_half, 0 if _half == "low" else 2),
    )
    _s(
        "i64x2.extmul_%s_i32x4_u" % _half,
        "raw",
        """pub fn i64x2_extmul_%s_i32x4_u(a: V, b: V) -> V {
    let x: [u32; 4] = unpack_u32(a);
    let y: [u32; 4] = unpack_u32(b);
    let o = %d;
    let mut r = [0u64; 2];
    for i in 0..2 { r[i] = (x[o + i] as u64).wrapping_mul(y[o + i] as u64); }
    pack_u64(r)
}"""
        % (_half, 0 if _half == "low" else 2),
    )

# --- f32x4 -----------------------------------------------------------------
_s("f32x4.splat", "spl", "f32")
_s("f32x4.extract_lane", "ex", "f32", 4)
_s("f32x4.replace_lane", "rep", "f32", 4)
_s("f32x4.add", "b2", "f32", 4, "|a, b| a + b")
_s("f32x4.sub", "b2", "f32", 4, "|a, b| a - b")
_s("f32x4.mul", "b2", "f32", 4, "|a, b| a * b")
_s("f32x4.div", "b2", "f32", 4, "|a, b| a / b")
_s("f32x4.min", "b2", "f32", 4, "|a: f32, b: f32| crate::f32_min(a, b)")
_s("f32x4.max", "b2", "f32", 4, "|a: f32, b: f32| crate::f32_max(a, b)")
_s("f32x4.pmin", "b2", "f32", 4, "|a: f32, b: f32| if b < a { b } else { a }")
_s("f32x4.pmax", "b2", "f32", 4, "|a: f32, b: f32| if a < b { b } else { a }")
_s("f32x4.abs", "u1", "f32", 4, "|a: f32| a.abs()")
_s("f32x4.neg", "u1", "f32", 4, "|a: f32| -a")
_s("f32x4.sqrt", "u1", "f32", 4, "|a: f32| a.sqrt()")
_s("f32x4.ceil", "u1", "f32", 4, "|a: f32| a.ceil()")
_s("f32x4.floor", "u1", "f32", 4, "|a: f32| a.floor()")
_s("f32x4.trunc", "u1", "f32", 4, "|a: f32| a.trunc()")
_s("f32x4.nearest", "u1", "f32", 4, "|a: f32| crate::f32_nearest(a)")
_s("f32x4.eq", "c2", "f32", "u32", 4, "|a: f32, b: f32| a == b")
_s("f32x4.ne", "c2", "f32", "u32", 4, "|a: f32, b: f32| a != b")
_s("f32x4.lt", "c2", "f32", "u32", 4, "|a: f32, b: f32| a < b")
_s("f32x4.gt", "c2", "f32", "u32", 4, "|a: f32, b: f32| a > b")
_s("f32x4.le", "c2", "f32", "u32", 4, "|a: f32, b: f32| a <= b")
_s("f32x4.ge", "c2", "f32", "u32", 4, "|a: f32, b: f32| a >= b")

# --- f64x2 -----------------------------------------------------------------
_s("f64x2.splat", "spl", "f64")
_s("f64x2.extract_lane", "ex", "f64", 2)
_s("f64x2.replace_lane", "rep", "f64", 2)
_s("f64x2.add", "b2", "f64", 2, "|a, b| a + b")
_s("f64x2.sub", "b2", "f64", 2, "|a, b| a - b")
_s("f64x2.mul", "b2", "f64", 2, "|a, b| a * b")
_s("f64x2.div", "b2", "f64", 2, "|a, b| a / b")
_s("f64x2.min", "b2", "f64", 2, "|a: f64, b: f64| crate::f64_min(a, b)")
_s("f64x2.max", "b2", "f64", 2, "|a: f64, b: f64| crate::f64_max(a, b)")
_s("f64x2.pmin", "b2", "f64", 2, "|a: f64, b: f64| if b < a { b } else { a }")
_s("f64x2.pmax", "b2", "f64", 2, "|a: f64, b: f64| if a < b { b } else { a }")
_s("f64x2.abs", "u1", "f64", 2, "|a: f64| a.abs()")
_s("f64x2.neg", "u1", "f64", 2, "|a: f64| -a")
_s("f64x2.sqrt", "u1", "f64", 2, "|a: f64| a.sqrt()")
_s("f64x2.ceil", "u1", "f64", 2, "|a: f64| a.ceil()")
_s("f64x2.floor", "u1", "f64", 2, "|a: f64| a.floor()")
_s("f64x2.trunc", "u1", "f64", 2, "|a: f64| a.trunc()")
_s("f64x2.nearest", "u1", "f64", 2, "|a: f64| crate::f64_nearest(a)")
_s("f64x2.eq", "c2", "f64", "u64", 2, "|a: f64, b: f64| a == b")
_s("f64x2.ne", "c2", "f64", "u64", 2, "|a: f64, b: f64| a != b")
_s("f64x2.lt", "c2", "f64", "u64", 2, "|a: f64, b: f64| a < b")
_s("f64x2.gt", "c2", "f64", "u64", 2, "|a: f64, b: f64| a > b")
_s("f64x2.le", "c2", "f64", "u64", 2, "|a: f64, b: f64| a <= b")
_s("f64x2.ge", "c2", "f64", "u64", 2, "|a: f64, b: f64| a >= b")
_s(
    "f64x2.convert_low_i32x4_s",
    "raw",
    """pub fn f64x2_convert_low_i32x4_s(a: V) -> V {
    let x: [i32; 4] = unpack_i32(a);
    pack_f64([x[0] as f64, x[1] as f64])
}""",
)
_s(
    "f64x2.convert_low_i32x4_u",
    "raw",
    """pub fn f64x2_convert_low_i32x4_u(a: V) -> V {
    let x: [u32; 4] = unpack_u32(a);
    pack_f64([x[0] as f64, x[1] as f64])
}""",
)
_s(
    "f64x2.promote_low_f32x4",
    "raw",
    """pub fn f64x2_promote_low_f32x4(a: V) -> V {
    let x: [f32; 4] = unpack_f32(a);
    pack_f64([x[0] as f64, x[1] as f64])
}""",
)
_s(
    "f32x4.demote_f64x2_zero",
    "raw",
    """pub fn f32x4_demote_f64x2_zero(a: V) -> V {
    let x: [f64; 2] = unpack_f64(a);
    pack_f32([x[0] as f32, x[1] as f32, 0.0, 0.0])
}""",
)
_s(
    "f32x4.convert_i32x4_s",
    "raw",
    """pub fn f32x4_convert_i32x4_s(a: V) -> V {
    let x: [i32; 4] = unpack_i32(a);
    pack_f32([x[0] as f32, x[1] as f32, x[2] as f32, x[3] as f32])
}""",
)
_s(
    "f32x4.convert_i32x4_u",
    "raw",
    """pub fn f32x4_convert_i32x4_u(a: V) -> V {
    let x: [u32; 4] = unpack_u32(a);
    pack_f32([x[0] as f32, x[1] as f32, x[2] as f32, x[3] as f32])
}""",
)
_s(
    "i32x4.trunc_sat_f64x2_s_zero",
    "raw",
    """pub fn i32x4_trunc_sat_f64x2_s_zero(a: V) -> V {
    let x: [f64; 2] = unpack_f64(a);
    let mut r = [0i32; 4];
    for i in 0..2 {
        let f = x[i];
        r[i] = if f.is_nan() { 0 } else if f <= -2147483648.0 { i32::MIN }
               else if f >= 2147483648.0 { i32::MAX } else { f as i32 };
    }
    pack_i32(r)
}""",
)
_s(
    "i32x4.trunc_sat_f64x2_u_zero",
    "raw",
    """pub fn i32x4_trunc_sat_f64x2_u_zero(a: V) -> V {
    let x: [f64; 2] = unpack_f64(a);
    let mut r = [0i32; 4];
    for i in 0..2 {
        let f = x[i];
        r[i] = if f.is_nan() || f <= 0.0 { 0 }
               else if f >= 4294967296.0 { -1 } else { f as u32 as i32 };
    }
    pack_i32(r)
}""",
)

# ---------------------------------------------------------------------------
# simd.rs generation
# ---------------------------------------------------------------------------

SIMD_HEADER = r'''//! 128-bit SIMD helpers for the translated module.
//!
//! GENERATED by `tools/wat2rust.py --emit-support` from the `SIMD` table in
//! `tools/w2r/tables.py` — edit the table, not this file.
//!
//! `V` is the translated representation of a wasm `v128`: a `u128` carrying the
//! 16 raw bytes in little-endian lane order, so `transmute::<V, [T; N]>` is the
//! lane view for every lane type.

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
            let sh = (s as u32) % (8 * core::mem::size_of::<$t>() as u32);
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
            let x: [$t; $k] = unsafe { core::mem::transmute(a) };
            x[(lane as usize) % $k]
        }
    };
}

macro_rules! replace {
    ($name:ident, $t:ty, $k:expr) => {
        #[inline]
        pub fn $name(a: V, lane: u32, v: $t) -> V {
            let mut x: [$t; $k] = unsafe { core::mem::transmute(a) };
            x[(lane as usize) % $k] = v;
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
'''

TST_BODIES = {
    "all_u8": "let x: [u8; 16] = unpack_u8(a); (x.iter().all(|&b| b != 0)) as i32",
    "all_u16": "let x: [u16; 8] = unpack_u16(a); (x.iter().all(|&b| b != 0)) as i32",
    "all_u32": "let x: [u32; 4] = unpack_u32(a); (x.iter().all(|&b| b != 0)) as i32",
    "all_u64": "let x: [u64; 2] = unpack_u64(a); (x.iter().all(|&b| b != 0)) as i32",
    "mask_u8": "let x: [u8; 16] = unpack_u8(a); let mut m = 0i32; for i in 0..16 { if x[i] & 0x80 != 0 { m |= 1 << i; } } m",
    "mask_u16": "let x: [u16; 8] = unpack_u16(a); let mut m = 0i32; for i in 0..8 { if x[i] & 0x8000 != 0 { m |= 1 << i; } } m",
    "mask_u32": "let x: [u32; 4] = unpack_u32(a); let mut m = 0i32; for i in 0..4 { if x[i] & 0x8000_0000 != 0 { m |= 1 << i; } } m",
    "mask_u64": "let x: [u64; 2] = unpack_u64(a); let mut m = 0i32; for i in 0..2 { if x[i] & 0x8000_0000_0000_0000 != 0 { m |= 1 << i; } } m",
}


def rust_name(op):
    return op.replace(".", "_")


def _norm_closure(body):
    """`|a: f32, b: f32| a + b` -> `|a, b| a + b` (macro supplies the types)."""
    m = re.match(r"^\|([^|]*)\|(.*)$", body, re.S)
    if not m:
        return body
    params = ",".join(p.split(":")[0].strip() for p in m.group(1).split(",") if p.strip())
    return "|%s|%s" % (params, m.group(2))


def gen_simd_rs():
    """Render wasm-rt/src/simd.rs from the SIMD table."""
    out = [SIMD_HEADER]
    for op in sorted(SIMD):
        spec = SIMD[op]
        kind = spec[0]
        name = rust_name(op)
        if kind == "raw":
            out.append(spec[1])
        elif kind == "b2":
            ty, k, body = spec[1], spec[2], _norm_closure(spec[3])
            out.append("b2!(%s, %s, %d, %s);" % (name, ty, k, body))
        elif kind == "c2":
            ty, uty, k, body = spec[1], spec[2], spec[3], _norm_closure(spec[4])
            out.append("c2!(%s, %s, %s, %d, %s);" % (name, ty, uty, k, body))
        elif kind == "u1":
            ty, k, body = spec[1], spec[2], _norm_closure(spec[3])
            out.append("u1!(%s, %s, %d, %s);" % (name, ty, k, body))
        elif kind == "sh":
            ty, k, body = spec[1], spec[2], _norm_closure(spec[3])
            out.append("sh!(%s, %s, %d, %s);" % (name, ty, k, body))
        elif kind == "spl":
            ty = spec[1]
            k = {"i8": 16, "u8": 16, "i16": 8, "u16": 8, "i32": 4, "u32": 4,
                 "i64": 2, "u64": 2, "f32": 4, "f64": 2}[ty]
            out.append("splat!(%s, %s, %d);" % (name, ty, k))
        elif kind == "ex":
            ty, k = spec[1], spec[2]
            out.append("extract!(%s, %s, %d);" % (name, ty, k))
        elif kind == "rep":
            ty, k = spec[1], spec[2]
            out.append("replace!(%s, %s, %d);" % (name, ty, k))
        elif kind == "tst":
            body = TST_BODIES[spec[1]]
            out.append("pub fn %s(a: V) -> i32 { %s }" % (name, body))
        else:
            raise ValueError("bad simd spec %s" % (op,))
    out.append("\n// op count: %d\n" % len(SIMD))
    return "\n".join(out)
