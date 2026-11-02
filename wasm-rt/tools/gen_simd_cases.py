#!/usr/bin/env python3
"""Generate `tests/common/simd_cases.rs`: hand-computable vectors for every SIMD op.

Expected values come from a *second*, independent implementation of the wasm
SIMD semantics written here from the specification text: integer lanes widened
to arbitrary precision with explicit wrap/clamp, IEEE-754 single-precision
rounding via `struct`. It deliberately does not import the table's Rust
closures, so a bug in `src/simd.rs` shows up as a mismatch against a literal
instead of as agreement between two copies of the same mistake.

Run from `recovered/rust/wasm-rt`:
    python tools/gen_simd_cases.py
"""
from __future__ import annotations

import math
import os
import struct
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CRATE = os.path.dirname(HERE)
RUST_ROOT = os.path.dirname(CRATE)
TABLE_DIR = os.path.join(RUST_ROOT, "benches", "tools", "wat2rust")

INF = float("inf")
QNAN = struct.unpack("<f", b"\x00\x00\xc0\x7f")[0]
DQNAN = struct.unpack("<d", b"\x00\x00\x00\x00\x00\x00\xf8\x7f")[0]

MASK64 = (1 << 64) - 1

INT_W = {"i8x16": 1, "i16x8": 2, "i32x4": 4, "i64x2": 8}
FLOAT_W = {"f32x4": 4, "f64x2": 8}


# ---------------------------------------------------------------------------
# lane plumbing
# ---------------------------------------------------------------------------


def lanes(v, w, signed=True):
    b = v.to_bytes(16, "little")
    return [int.from_bytes(b[i * w:(i + 1) * w], "little", signed=signed)
            for i in range(16 // w)]


def pack(ls, w):
    bits = (1 << (8 * w)) - 1
    out = 0
    for i, x in enumerate(ls):
        out |= (x & bits) << (8 * w * i)
    return out


def f32_lanes(v):
    b = v.to_bytes(16, "little")
    return [struct.unpack("<f", b[i * 4:i * 4 + 4])[0] for i in range(4)]


def f64_lanes(v):
    b = v.to_bytes(16, "little")
    return [struct.unpack("<d", b[i * 8:i * 8 + 8])[0] for i in range(2)]


def f32_bits(x):
    try:
        return struct.unpack("<I", struct.pack("<f", x))[0]
    except OverflowError:
        # wasm demote/promote overflow to infinity rather than trapping
        return 0x7F80_0000 if x > 0 else 0xFF80_0000


def f64_bits(x):
    return struct.unpack("<Q", struct.pack("<d", x))[0]


def pack_f32(ls):
    return pack([f32_bits(x) for x in ls], 4)


def pack_f64(ls):
    return pack([f64_bits(x) for x in ls], 8)


def nan(x):
    return x != x


def neg_zero(x):
    return struct.pack("<d", x)[7] & 0x80 != 0


def wrap(x, w):
    bits = 8 * w
    x &= (1 << bits) - 1
    if x >> (bits - 1):
        x -= 1 << bits
    return x


def sat(x, lo, hi):
    return lo if x < lo else hi if x > hi else x


def nearest(x):
    """wasm f*nearest: round half to even, sign of a zero result preserved."""
    if nan(x) or x in (INF, -INF):
        return x
    if x == 0.0:
        return x
    r = float(round(x))  # python rounds half to even
    if r == 0.0:
        return -0.0 if (neg_zero(x) or x < 0) else 0.0
    return r


def fmin(a, b):
    if nan(a) or nan(b):
        return QNAN
    if a == b:
        return a if neg_zero(a) else b
    return a if a < b else b


def fmax(a, b):
    if nan(a) or nan(b):
        return QNAN
    if a == b:
        return b if neg_zero(a) else a
    return a if a > b else b


def dmin(a, b):
    if nan(a) or nan(b):
        return DQNAN
    if a == b:
        return a if neg_zero(a) else b
    return a if a < b else b


def dmax(a, b):
    if nan(a) or nan(b):
        return DQNAN
    if a == b:
        return b if neg_zero(a) else a
    return a if a > b else b


def fdiv(a, b):
    """IEEE division, including the zero-denominator cases Python raises on."""
    if b == 0.0:
        if a == 0.0 or nan(a):
            return float("nan")
        return -INF if (neg_zero(a) != neg_zero(b)) else INF
    return a / b


# ---------------------------------------------------------------------------
# reference: (V,V) -> V
# ---------------------------------------------------------------------------


def ref_bin(op, a, b):
    prefix, suffix = op.split(".", 1)

    if prefix == "v128":
        if suffix == "bitselect":
            # caller passes mask in `b`; a is data -> handled by caller
            raise KeyError(op)
        return {"and": a & b,
                "andnot": a & (~b & ((1 << 128) - 1)),
                "or": a | b,
                "xor": a ^ b}[suffix]

    if prefix in FLOAT_W:
        w = FLOAT_W[prefix]
        read, write = (f32_lanes, pack_f32) if w == 4 else (f64_lanes, pack_f64)
        mn, mx = (fmin, fmax) if w == 4 else (dmin, dmax)
        xs, ys = read(a), read(b)
        mask = (1 << (8 * w)) - 1
        if suffix in ("eq", "ne", "lt", "gt", "le", "ge"):
            cmp = {"eq": lambda x, y: x == y, "ne": lambda x, y: x != y,
                   "lt": lambda x, y: x < y, "gt": lambda x, y: x > y,
                   "le": lambda x, y: x <= y, "ge": lambda x, y: x >= y}[suffix]
            return pack([mask if cmp(x, y) else 0 for x, y in zip(xs, ys)], w)
        out = []
        for x, y in zip(xs, ys):
            if suffix == "add":
                out.append(x + y)
            elif suffix == "sub":
                out.append(x - y)
            elif suffix == "mul":
                out.append(x * y)
            elif suffix == "div":
                out.append(fdiv(x, y))
            elif suffix == "min":
                out.append(mn(x, y))
            elif suffix == "max":
                out.append(mx(x, y))
            elif suffix == "pmin":
                out.append(y if y < x else x)
            elif suffix == "pmax":
                out.append(y if x < y else x)
            else:
                raise KeyError(op)
        return write(out)

    if prefix in INT_W:
        w = INT_W[prefix]

        if suffix == "swizzle":
            x = a.to_bytes(16, "little")
            m = b.to_bytes(16, "little")
            return pack([x[i] if i < 16 else 0 for i in m], 1)

        if suffix.startswith("narrow_"):
            sw = INT_W[suffix.split("_")[1]]
            signed = suffix.endswith("_s")
            if signed:
                lo, hi = -(1 << (8 * w - 1)), (1 << (8 * w - 1)) - 1
            else:
                lo, hi = 0, (1 << (8 * w)) - 1
            # both narrow_s and narrow_u read the SOURCE lanes as signed and only
            # differ in the destination range (spec: sat_u takes a signed integer)
            xs = lanes(a, sw, True)
            ys = lanes(b, sw, True)
            return pack([sat(v, lo, hi) for v in xs + ys], w)

        if suffix.startswith("extmul_"):
            parts = suffix.split("_")
            half, src = parts[1], parts[2]
            sw = INT_W[src]
            signed = suffix.endswith("_s")
            xs, ys = lanes(a, sw, signed), lanes(b, sw, signed)
            k = 16 // w
            off = 0 if half == "low" else len(xs) - k
            return pack([xs[off + i] * ys[off + i] for i in range(k)], w)

        if suffix.startswith("extadd_pairwise_"):
            sw = INT_W[suffix.split("_")[2]]
            signed = suffix.endswith("_s")
            xs = lanes(a, sw, signed)
            return pack([xs[2 * i] + xs[2 * i + 1] for i in range(16 // w)], w)

        if suffix == "dot_i16x8_s":
            xs, ys = lanes(a, 2, True), lanes(b, 2, True)
            return pack([wrap(xs[2 * i] * ys[2 * i] + xs[2 * i + 1] * ys[2 * i + 1], 4)
                         for i in range(4)], 4)

        if suffix == "q15mulr_sat_s":
            xs, ys = lanes(a, 2, True), lanes(b, 2, True)
            return pack([sat((x * y + 0x4000) >> 15, -32768, 32767)
                         for x, y in zip(xs, ys)], 2)

        if suffix == "avgr_u":
            xs, ys = lanes(a, w, False), lanes(b, w, False)
            return pack([(x + y + 1) >> 1 for x, y in zip(xs, ys)], w)

        unsigned = suffix.endswith("_u") and suffix.split("_")[0] in (
            "min", "max", "lt", "gt", "le", "ge")
        xs = lanes(a, w, not unsigned)
        ys = lanes(b, w, not unsigned)
        lo, hi = -(1 << (8 * w - 1)), (1 << (8 * w - 1)) - 1
        uhi = (1 << (8 * w)) - 1
        mask = uhi

        if suffix == "add":
            return pack([wrap(x + y, w) for x, y in zip(xs, ys)], w)
        if suffix == "sub":
            return pack([wrap(x - y, w) for x, y in zip(xs, ys)], w)
        if suffix == "mul":
            return pack([wrap(x * y, w) for x, y in zip(xs, ys)], w)
        if suffix == "add_sat_s":
            return pack([sat(x + y, lo, hi) for x, y in zip(xs, ys)], w)
        if suffix == "sub_sat_s":
            return pack([sat(x - y, lo, hi) for x, y in zip(xs, ys)], w)
        if suffix == "add_sat_u":
            ux, uy = lanes(a, w, False), lanes(b, w, False)
            return pack([sat(x + y, 0, uhi) for x, y in zip(ux, uy)], w)
        if suffix == "sub_sat_u":
            ux, uy = lanes(a, w, False), lanes(b, w, False)
            return pack([sat(x - y, 0, uhi) for x, y in zip(ux, uy)], w)
        if suffix in ("min_s", "min_u"):
            return pack([min(x, y) for x, y in zip(xs, ys)], w)
        if suffix in ("max_s", "max_u"):
            return pack([max(x, y) for x, y in zip(xs, ys)], w)
        if suffix in ("eq", "ne", "lt_s", "gt_s", "le_s", "ge_s",
                      "lt_u", "gt_u", "le_u", "ge_u"):
            op2 = suffix[:2]
            f = {"eq": lambda x, y: x == y, "ne": lambda x, y: x != y,
                 "lt": lambda x, y: x < y, "gt": lambda x, y: x > y,
                 "le": lambda x, y: x <= y, "ge": lambda x, y: x >= y}[op2]
            return pack([mask if f(x, y) else 0 for x, y in zip(xs, ys)], w)
        raise KeyError(op)

    raise KeyError(op)


# ---------------------------------------------------------------------------
# reference: (V) -> V
# ---------------------------------------------------------------------------


def ref_un(op, a):
    prefix, suffix = op.split(".", 1)

    if op == "v128.not":
        return ~a & ((1 << 128) - 1)

    # conversions: names start with i32x4/f32x4/f64x2 but are not lane-wise
    if op == "i32x4.trunc_sat_f32x4_s":
        out = []
        for x in f32_lanes(a):
            if nan(x):
                out.append(0)
            elif x <= -(1 << 31):
                out.append(-(1 << 31))
            elif x >= (1 << 31):
                out.append((1 << 31) - 1)
            else:
                out.append(int(x))
        return pack(out, 4)
    if op == "i32x4.trunc_sat_f32x4_u":
        out = []
        for x in f32_lanes(a):
            if nan(x) or x <= 0:
                out.append(0)
            elif x >= (1 << 32):
                out.append((1 << 32) - 1)
            else:
                out.append(int(x))
        return pack(out, 4)
    if op == "i32x4.trunc_sat_f64x2_s_zero":
        out = []
        for x in f64_lanes(a):
            if nan(x):
                out.append(0)
            elif x <= -(1 << 31):
                out.append(-(1 << 31))
            elif x >= (1 << 31):
                out.append((1 << 31) - 1)
            else:
                out.append(int(x))
        return pack(out + [0, 0], 4)
    if op == "i32x4.trunc_sat_f64x2_u_zero":
        out = []
        for x in f64_lanes(a):
            if nan(x) or x <= 0:
                out.append(0)
            elif x >= (1 << 32):
                out.append((1 << 32) - 1)
            else:
                out.append(int(x))
        return pack(out + [0, 0], 4)
    if op == "f32x4.convert_i32x4_s":
        return pack_f32([float(x) for x in lanes(a, 4, True)])
    if op == "f32x4.convert_i32x4_u":
        return pack_f32([float(x) for x in lanes(a, 4, False)])
    if op == "f64x2.convert_low_i32x4_s":
        return pack_f64([float(x) for x in lanes(a, 4, True)[:2]])
    if op == "f64x2.convert_low_i32x4_u":
        return pack_f64([float(x) for x in lanes(a, 4, False)[:2]])
    if op == "f32x4.demote_f64x2_zero":
        return pack_f32(list(f64_lanes(a)) + [0.0, 0.0])
    if op == "f64x2.promote_low_f32x4":
        return pack_f64([float(x) for x in f32_lanes(a)[:2]])

    if prefix in FLOAT_W:
        w = FLOAT_W[prefix]
        read, write = (f32_lanes, pack_f32) if w == 4 else (f64_lanes, pack_f64)
        out = []
        for x in read(a):
            if suffix == "abs":
                out.append(abs(x))
            elif suffix == "neg":
                out.append(-x)
            elif suffix == "sqrt":
                out.append(x if x == 0.0 else (x ** 0.5 if x > 0 else float("nan")))
            elif suffix in ("ceil", "floor", "trunc"):
                if nan(x) or x in (INF, -INF) or x == 0.0:
                    out.append(x)
                else:
                    f = {"ceil": math.ceil, "floor": math.floor, "trunc": math.trunc}[suffix]
                    r = float(f(x))
                    # ceil(-0.5) is -0.0: a zero result keeps the sign of x
                    out.append(-0.0 if (r == 0.0 and (neg_zero(x) or x < 0)) else r)
            elif suffix == "nearest":
                out.append(nearest(x))
            else:
                raise KeyError(op)
        return write(out)

    if prefix in INT_W:
        w = INT_W[prefix]
        if suffix.startswith("extend_"):
            parts = suffix.split("_")
            half, src = parts[1], parts[2]
            sw = INT_W[src]
            signed = suffix.endswith("_s")
            xs = lanes(a, sw, signed)
            k = 16 // w
            off = 0 if half == "low" else len(xs) - k
            return pack(xs[off:off + k], w)
        if suffix.startswith("extadd_pairwise_"):
            sw = INT_W[suffix.split("_")[2]]
            signed = suffix.endswith("_s")
            xs = lanes(a, sw, signed)
            return pack([xs[2 * i] + xs[2 * i + 1] for i in range(16 // w)], w)
        if suffix == "abs":
            lo = -(1 << (8 * w - 1))
            return pack([lo if x == lo else abs(x) for x in lanes(a, w, True)], w)
        if suffix == "neg":
            return pack([wrap(-x, w) for x in lanes(a, w, True)], w)
        if suffix == "popcnt":
            return pack([bin(x).count("1") for x in lanes(a, 1, False)], 1)
        raise KeyError(op)

    raise KeyError(op)


# ---------------------------------------------------------------------------
# vectors
# ---------------------------------------------------------------------------

A0 = [0x80, 0x7F, 0xFF, 0x00, 0x01, 0x7E, 0x81, 0x40,
      0xC0, 0x02, 0xFD, 0x7F, 0x80, 0x00, 0xFF, 0x10]
B0 = [0x01, 0xFF, 0x80, 0x7F, 0x00, 0x02, 0xFE, 0x41,
      0x3F, 0xFF, 0x01, 0x80, 0x7F, 0x00, 0x02, 0x80]


def vec(b):
    assert len(b) == 16
    return int.from_bytes(bytes(b), "little")


IVEC = [
    vec(A0),                                    # high-bit mix
    vec(B0),
    vec([0x80] * 16),                           # every lane = signed MIN
    vec([0x7F] * 16),                           # every lane = signed MAX
    vec([0xFF] * 16),                           # -1 / unsigned MAX
    vec([0x00] * 16),                           # zeroes
    vec(list(reversed(A0))),
    vec([(i * 37 + 11) & 0xFF for i in range(16)]),
]

FV = [
    pack_f32([1.5, -2.25, 0.0, -0.0]),
    pack_f32([-4.0, 2.25, 0.5, 1e30]),
    pack_f32([0.5, -0.5, INF, -INF]),
    pack_f32([2.5, 3.5, -0.0, 3.5]),            # exact halfway cases
    pack_f32([1e-40, -1e38, 16777217.0, -16777216.0]),
    pack_f64([1.5, -2.25]),
    pack_f64([0.5, -0.5]),
    pack_f64([2.5, 3.5]),
    pack_f64([1e300, -0.0]),
]

INT_INPUTS = [(IVEC[0], IVEC[1]), (IVEC[2], IVEC[3]), (IVEC[6], IVEC[7])]
FLOAT_INPUTS = [(FV[0], FV[1]), (FV[2], FV[3]), (FV[4], FV[0])]
FLOAT_UNARY = [FV[0], FV[2], FV[3], FV[4]]

SPL_VALUES = {
    "i8x16": [0x80, 0x7F],
    "i16x8": [0x8000, 0x7FFF],
    "i32x4": [0x8000_0000, 0xFFFF_FFFF],
    "i64x2": [0x8000_0000_0000_0000, 0xFFFF_FFFF_FFFF_FFFF],
    "f32x4": [f32_bits(-0.0), f32_bits(1.5)],
    "f64x2": [f64_bits(-0.0), f64_bits(-2.5)],
}

# ops that benches/tools/wat2rust/w2r/tables.py does not declare yet
MISSING_KIND = {
    "i16x8.extadd_pairwise_i8x16_s": "u1",
    "i16x8.extadd_pairwise_i8x16_u": "u1",
    "i32x4.extadd_pairwise_i16x8_s": "u1",
    "i32x4.extadd_pairwise_i16x8_u": "u1",
    "i32x4.dot_i16x8_s": "b2",
}


def kind_of(op, spec):
    if spec is not None and spec[0] != "raw":
        # `c2` is still a (V,V)->V op; only the result lanes are masks
        return "b2" if spec[0] == "c2" else spec[0]
    if op == "v128.not":
        return "u1"
    if op in ("v128.and", "v128.andnot", "v128.or", "v128.xor"):
        return "b2"
    if op == "v128.bitselect":
        return "b3"
    if op == "v128.any_true":
        return "tst"
    if op in ("i8x16.shuffle", "i8x16.swizzle"):
        return "b2"
    if ".narrow_" in op or ".extmul_" in op or op == "i32x4.dot_i16x8_s":
        return "b2"
    return "u1"


def fmt_of(op):
    if op.startswith("f32x4."):
        return "F32"
    if op.startswith("f64x2."):
        return "F64"
    return "Bits"


def main():
    sys.path.insert(0, TABLE_DIR)
    from w2r import tables  # noqa: PLC0415

    ops = sorted(set(tables.SIMD) | set(MISSING_KIND))

    BIN, UN, SH, REP, SPL, EXT, TST, TER = [], [], [], [], [], [], [], []

    for op in ops:
        spec = tables.SIMD.get(op)
        kind = kind_of(op, spec)
        prefix = op.split(".")[0]
        w = INT_W.get(prefix) or FLOAT_W.get(prefix)
        is_float = prefix in FLOAT_W
        k = 16 // w if w else 0

        if kind == "b3":  # v128.bitselect(data, data, mask)
            for x, y, m in ((IVEC[0], IVEC[1], IVEC[2]), (IVEC[3], IVEC[4], IVEC[5]),
                            (IVEC[1], IVEC[6], IVEC[7])):
                TER.append((op, x, y, m, (x & m) | (y & ~m & ((1 << 128) - 1))))
            continue

        if op == "i8x16.shuffle":
            x = IVEC[0].to_bytes(16, "little")
            y = IVEC[1].to_bytes(16, "little")
            joined = x + y
            for m in ([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                      [31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16],
                      [15, 0, 31, 1, 14, 2, 30, 3, 13, 4, 29, 5, 12, 6, 28, 7]):
                TER.append((op, IVEC[0], IVEC[1], vec(m), vec([joined[i] for i in m])))
            continue

        if kind == "b2":
            for a, b in (FLOAT_INPUTS if is_float else INT_INPUTS):
                BIN.append((op, a, b, ref_bin(op, a, b), fmt_of(op)))
            continue

        if kind == "u1":
            for a in (FLOAT_UNARY if is_float else [IVEC[0], IVEC[2], IVEC[3], IVEC[7]]):
                UN.append((op, a, ref_un(op, a), fmt_of(op)))
            continue

        if kind == "sh":
            for a, s in ((IVEC[0], 0), (IVEC[0], 1), (IVEC[0], 8), (IVEC[2], 255)):
                bits = 8 * w
                sh = s % bits
                if op.endswith(".shl"):
                    ls = [wrap(x << sh, w) for x in lanes(a, w, True)]
                elif op.endswith(".shr_s"):
                    ls = [wrap(x >> sh, w) for x in lanes(a, w, True)]
                else:
                    ls = [(x & ((1 << bits) - 1)) >> sh for x in lanes(a, w, False)]
                SH.append((op, a, s, pack(ls, w)))
            continue

        if kind == "ex":
            a = FV[0] if is_float else IVEC[0]
            for lane in (0, k - 1):
                if is_float:
                    raw = f32_bits(f32_lanes(a)[lane]) if w == 4 else f64_bits(f64_lanes(a)[lane])
                elif op.endswith("_s"):
                    # wasm: extract_lane_s sign-extends the lane to i32
                    v = lanes(a, w, True)[lane]
                    raw = v & MASK64
                else:
                    raw = lanes(a, w, False)[lane]
                EXT.append((op, a, lane, raw))
            continue

        if kind == "rep":
            a = FV[1] if is_float else IVEC[1]
            for lane, v in ((0, SPL_VALUES[prefix][0]), (k - 1, SPL_VALUES[prefix][1])):
                raw = bytearray(a.to_bytes(16, "little"))
                raw[lane * w:(lane + 1) * w] = (v & ((1 << (8 * w)) - 1)).to_bytes(w, "little")
                REP.append((op, a, lane, v, int.from_bytes(bytes(raw), "little")))
            continue

        if kind == "spl":
            for v in SPL_VALUES[prefix]:
                bits = (1 << (8 * w)) - 1
                SPL.append((op, v, pack([v & bits] * k, w)))
            continue

        if kind == "tst":
            for a in (IVEC[5], IVEC[2], IVEC[0]):
                if op.endswith("any_true"):
                    want = int(a != 0)
                elif op.endswith("all_true"):
                    want = int(all(x != 0 for x in lanes(a, w, False)))
                else:
                    xs = lanes(a, w, False)
                    want = sum(1 << i for i, x in enumerate(xs) if x >> (8 * w - 1))
                TST.append((op, a, want))
            continue

        raise SystemExit("unhandled kind %s for %s" % (kind, op))

    covered = ({r[0] for r in BIN} | {r[0] for r in TER} | {r[0] for r in UN} | {r[0] for r in SH}
               | {r[0] for r in REP} | {r[0] for r in SPL} | {r[0] for r in EXT}
               | {r[0] for r in TST})
    missing = sorted(set(ops) - covered)
    if missing:
        raise SystemExit("ops with no vector: %s" % missing)

    L = []
    L.append("//! Hand-computed SIMD vectors. GENERATED by `tools/gen_simd_cases.py`.")
    L.append("//!")
    L.append("//! `want` is the exact v128 the wasm semantics require, produced by an")
    L.append("//! independent implementation of the spec (Python, widened integers), never")
    L.append("//! by the Rust helper under test.")
    L.append("")
    L.append("")
    L.append("use wasm_rt::simd::*;")
    L.append("")
    L.append("/// Result-lane comparison mode.")
    L.append("#[derive(Clone, Copy, PartialEq, Eq)]")
    L.append("pub enum Fmt {")
    L.append("    /// Compare all 128 bits exactly.")
    L.append("    Bits,")
    L.append("    /// Compare four f32 lanes; any NaN equals any NaN.")
    L.append("    F32,")
    L.append("    /// Compare two f64 lanes; any NaN equals any NaN.")
    L.append("    F64,")
    L.append("}")
    L.append("")
    L.append("/// One `(V,V) -> V` or `(V) -> V` row.")
    L.append("pub struct Case {")
    L.append("    pub op: &'static str,")
    L.append("    pub a: u128,")
    L.append("    pub b: u128,")
    L.append("    pub want: u128,")
    L.append("    pub fmt: Fmt,")
    L.append("}")
    L.append("")
    L.append("/// Shift rows carry the immediate as an i32.")
    L.append("pub struct ShCase { pub op: &'static str, pub a: u128, pub shift: i32, pub want: u128 }")
    L.append("")
    L.append("/// `replace_lane` / `splat` rows carry the raw lane bits in `value`.")
    L.append("pub struct RepCase { pub op: &'static str, pub a: u128, pub lane: u32, pub value: u64, pub want: u128 }")
    L.append("")
    L.append("/// Scalar lane reads; `want` is the raw bits of the returned lane.")
    L.append("pub struct ExtCase { pub op: &'static str, pub a: u128, pub lane: u32, pub want: u64 }")
    L.append("")
    L.append("/// Scalar reduces: `all_true` / `any_true` / `bitmask`.")
    L.append("pub struct TstCase { pub op: &'static str, pub a: u128, pub want: i32 }")
    L.append("")

    def case_rows(name, rows):
        L.append("pub const %s: &[Case] = &[" % name)
        for op, a, b, want, fmt in rows:
            L.append('    Case { op: "%s", a: %s, b: %s, want: %s, fmt: Fmt::%s },'
                     % (op, hex(a), hex(b), hex(want), fmt))
        L.append("];")
        L.append("")

    case_rows("BIN", BIN)
    case_rows("UN", [(op, a, 0, want, fmt) for op, a, want, fmt in UN])

    L.append("/// Three-operand rows: `i8x16.shuffle` and `v128.bitselect` take a")
    L.append("/// second data operand plus an immediate/mask in `c`.")
    L.append("#[derive(Clone, Copy)]")
    L.append("pub struct TerCase { pub op: &'static str, pub a: u128, pub b: u128, pub c: u128, pub want: u128 }")
    L.append("")
    L.append("pub const TER: &[TerCase] = &[")
    for op, a, b, c, want in TER:
        L.append('    TerCase { op: "%s", a: %s, b: %s, c: %s, want: %s },'
                 % (op, hex(a), hex(b), hex(c), hex(want)))
    L.append("];")
    L.append("")

    L.append("pub const SH: &[ShCase] = &[")
    for op, a, s, want in SH:
        L.append('    ShCase { op: "%s", a: %s, shift: %d, want: %s },' % (op, hex(a), s, hex(want)))
    L.append("];")
    L.append("")
    L.append("pub const REP: &[RepCase] = &[")
    for op, a, lane, v, want in REP:
        L.append('    RepCase { op: "%s", a: %s, lane: %d, value: %s, want: %s },'
                 % (op, hex(a), lane, hex(v), hex(want)))
    L.append("];")
    L.append("")
    L.append("pub const SPL: &[RepCase] = &[")
    for op, v, want in SPL:
        L.append('    RepCase { op: "%s", a: 0, lane: 0, value: %s, want: %s },'
                 % (op, hex(v), hex(want)))
    L.append("];")
    L.append("")
    L.append("pub const EXT: &[ExtCase] = &[")
    for op, a, lane, want in EXT:
        L.append('    ExtCase { op: "%s", a: %s, lane: %d, want: %s },'
                 % (op, hex(a), lane, hex(want)))
    L.append("];")
    L.append("")
    L.append("pub const TST: &[TstCase] = &[")
    for op, a, want in TST:
        L.append('    TstCase { op: "%s", a: %s, want: %d },' % (op, hex(a), want))
    L.append("];")
    L.append("")

    L.append("/// Every op the runtime is expected to provide (table ops + additions).")
    L.append("pub const OPS: &[&str] = &[")
    for op in ops:
        L.append('    "%s",' % op)
    L.append("];")
    L.append("")

    def name_of(op):
        return op.replace(".", "_")

    def dispatch(fn, rows, args, body_for):
        L.append("pub fn %s(%s) -> Option<u128> {" % (fn, args))
        L.append("    Some(match op {")
        seen = set()
        for r in rows:
            op = r[0]
            if op in seen:
                continue
            seen.add(op)
            L.append('        "%s" => %s,' % (op, body_for(op)))
        L.append("        _ => return None,")
        L.append("    })")
        L.append("}")
        L.append("")

    dispatch("bin", BIN, "op: &str, a: u128, b: u128", lambda op: "%s(a, b)" % name_of(op))
    dispatch("ter", TER, "op: &str, a: u128, b: u128, c: u128",
             lambda op: "%s(a, b, c)" % name_of(op))
    dispatch("un", UN, "op: &str, a: u128", lambda op: "%s(a)" % name_of(op))
    dispatch("sh", SH, "op: &str, a: u128, s: i32", lambda op: "%s(a, s)" % name_of(op))

    def lane_value(op, expr):
        prefix = op.split(".")[0]
        if prefix == "f32x4":
            return "f32::from_bits(%s as u32)" % expr
        if prefix == "f64x2":
            return "f64::from_bits(%s)" % expr
        return "%s as %s" % (expr, {"i8x16": "i8", "i16x8": "i16",
                                    "i32x4": "i32", "i64x2": "i64"}[prefix])

    L.append("/// `replace_lane`: `value` is the raw bits of the lane-typed operand.")
    L.append("pub fn rep(op: &str, a: u128, lane: u32, value: u64) -> Option<u128> {")
    L.append("    Some(match op {")
    for op in sorted({r[0] for r in REP}):
        L.append('        "%s" => %s(a, lane, %s),' % (op, name_of(op), lane_value(op, "value")))
    L.append("        _ => return None,")
    L.append("    })")
    L.append("}")
    L.append("")
    L.append("/// `splat`: `value` is the raw bits of the lane-typed operand.")
    L.append("pub fn spl(op: &str, value: u64) -> Option<u128> {")
    L.append("    Some(match op {")
    for op in sorted({r[0] for r in SPL}):
        L.append('        "%s" => %s(%s),' % (op, name_of(op), lane_value(op, "value")))
    L.append("        _ => return None,")
    L.append("    })")
    L.append("}")
    L.append("")
    L.append("/// Scalar lane read, widened to raw `u64` bits.")
    L.append("pub fn ext(op: &str, a: u128, lane: u32) -> Option<u64> {")
    L.append("    Some(match op {")
    for op in sorted({r[0] for r in EXT}):
        prefix = op.split(".")[0]
        call = "%s(a, lane)" % name_of(op)
        if prefix == "f32x4":
            expr = "f32::to_bits(%s) as u64" % call
        elif prefix == "f64x2":
            expr = "f64::to_bits(%s)" % call
        elif op.endswith("_u"):
            expr = "%s as u64" % call
        else:
            expr = "%s as i64 as u64" % call
        L.append('        "%s" => %s,' % (op, expr))
    L.append("        _ => return None,")
    L.append("    })")
    L.append("}")
    L.append("")
    L.append("pub fn tst(op: &str, a: u128) -> Option<i32> {")
    L.append("    Some(match op {")
    for op in sorted({r[0] for r in TST}):
        L.append('        "%s" => %s(a),' % (op, name_of(op)))
    L.append("        _ => return None,")
    L.append("    })")
    L.append("}")
    L.append("")

    path = os.path.join(CRATE, "tests", "common", "simd_cases.rs")
    with open(path, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("\n".join(L))
    print("wrote %s" % path)
    print("ops=%d bin=%d ter=%d un=%d sh=%d rep=%d spl=%d ext=%d tst=%d"
          % (len(ops), len(BIN), len(TER), len(UN), len(SH), len(REP), len(SPL),
             len(EXT), len(TST)))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())