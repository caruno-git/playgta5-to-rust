"""WAT function body -> Rust: parser, structured emitter, state-machine fallback.

The translator is a straight-line walk over the wasm operand stack.  Values are
kept as *deferred expressions* (so `local.get p; i64.const 296; i64.add;
i64.load` becomes `ph.read_u64(p + 296)`), and are materialised into named
temporaries (`let t3 = ...;`) at every point where sequencing or mutation makes
the expression no longer safe to defer.  Nothing is ever dropped: any opcode the
translator does not understand is emitted as an `UNTRANSLATED` comment and
counted.
"""

import re

try:
    from .tables import (RUST, ZERO, NUM, LOADS, STORES, SIMD, ATOMIC,
                         SPLAT_LOADS, LANE_LOADS, LANE_STORES, rust_name)
except ImportError:  # running as a plain script directory
    from tables import (RUST, ZERO, NUM, LOADS, STORES, SIMD, ATOMIC,
                        SPLAT_LOADS, LANE_LOADS, LANE_STORES, rust_name)

COMMENT_RE = re.compile(r"\(;.*?;\)")


class Unsupported(Exception):
    """Construct the structured emitter cannot express."""


# ---------------------------------------------------------------------------
# names
# ---------------------------------------------------------------------------

_RUST_KEYWORDS = {
    "as", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
    "move", "mut", "pub", "ref", "return", "self", "static", "struct", "super",
    "trait", "true", "type", "unsafe", "use", "where", "while", "async", "await",
    "dyn", "abstract", "become", "box", "do", "final", "macro", "override",
    "priv", "typeof", "unsized", "virtual", "yield", "try", "union",
}


def sanitize(name, idx, maxlen=64):
    """demangled C++ name -> ASCII snake_case Rust identifier (unique via idx)."""
    s = name
    if s.startswith("$"):
        s = s[1:]
    # common demangler artefacts -> word separators
    s = s.replace("::", "_").replace("~", "dtor").replace("<", "_").replace(">", "_")
    s = s.replace("*", "_ptr").replace("&", "_ref").replace(",", "_")
    s = s.replace("operator", "op").replace("-", "_").replace(".", "_")
    s = s.replace(":", "_").replace(" ", "_")
    out = []
    for ch in s:
        c = ch.lower()
        if ("a" <= c <= "z") or ("0" <= c <= "9") or c == "_":
            out.append(c)
        else:
            out.append("_")
    body = re.sub(r"_+", "_", "".join(out)).strip("_")
    if not body:
        body = "unnamed"
    if body[0].isdigit():
        body = "f" + body
    body = body[:maxlen].strip("_") or "unnamed"
    if body in _RUST_KEYWORDS:
        body += "_fn"
    return "f_%06d_%s" % (idx, body)


# ---------------------------------------------------------------------------
# parsing
# ---------------------------------------------------------------------------

RE_FUNC_HDR = re.compile(r"^\s*\(func\s+(?:\$(\S+))?(.*)$")
RE_TYPE_IDX = re.compile(r"\(type\s+(\d+)\)")
RE_PARAMS = re.compile(r"\(param\s+([^)]*)\)")
RE_RESULTS = re.compile(r"\(result\s+([^)]*)\)")
RE_LOCAL = re.compile(r"^\s*\(local\s+(.*?)\)\s*$")
RE_BLOCKT = re.compile(r"^(?:\(type\s+(\d+)\))?\s*((?:\(param\s+[^)]*\))*)\s*((?:\(result\s+[^)]*\))*)\s*$")


def _valtypes(s):
    out = []
    for tok in s.split():
        tok = tok.strip()
        if not tok or tok.startswith("$"):
            continue
        out.append(tok)
    return out


def parse_header(text, type_sigs):
    """-> (name, params, results, n_header_lines)"""
    name = ""
    params = []
    results = []
    typ = None
    for line in text.splitlines():
        m = RE_FUNC_HDR.match(line)
        if not m:
            continue
        if m.group(1):
            name = m.group(1)
        rest = m.group(2)
        tm = RE_TYPE_IDX.search(rest)
        if tm:
            typ = int(tm.group(1))
        for pm in RE_PARAMS.finditer(rest):
            params.extend(_valtypes(pm.group(1)))
        for rm in RE_RESULTS.finditer(rest):
            results.extend(_valtypes(rm.group(1)))
        if ")" in rest and not rest.rstrip().endswith(("(", ",")):
            pass
    if typ is not None and type_sigs and typ in type_sigs:
        tp, tr = type_sigs[typ]
        if not params:
            params = list(tp)
        if not results:
            results = list(tr)
    return name, params, results


class Body(object):
    """A parsed function body: instruction list with pre-resolved branch targets."""

    __slots__ = ("name", "params", "results", "locals", "instrs", "targeted",
                 "type_idx")


def parse_body(text, type_sigs, idx):
    b = Body()
    lines = text.splitlines()
    b.name, b.params, b.results = parse_header(text, type_sigs)
    b.type_idx = None
    locs = []
    instrs = []
    started = False
    for line in lines:
        raw = line.strip()
        if not raw:
            continue
        if raw.startswith("(func"):
            continue
        if not started and raw.startswith("(local"):
            m = RE_LOCAL.match(line)
            if m:
                inner = m.group(1)
                # `(local i32 i32)` — unnamed; `(local $x i32)` — named
                toks = inner.split()
                if toks and toks[0].startswith("$"):
                    for t in toks:
                        if not t.startswith("$"):
                            locs.append(t)
                else:
                    locs.extend(_valtypes(inner))
            continue
        if not started and raw.startswith("(") and not raw.startswith("(local"):
            # `(type N) (param ..)` continued on its own line
            for pm in RE_PARAMS.finditer(raw):
                b.params.extend(_valtypes(pm.group(1)))
            for rm in RE_RESULTS.finditer(raw):
                b.results.extend(_valtypes(rm.group(1)))
            continue
        if raw.startswith("(func"):
            continue
        started = True
        # strip block comments (;...;) then line comment
        if "(;" in raw:
            s = COMMENT_RE.sub("", raw)
        else:
            s = raw
        if ";;" in s:
            s = s.split(";;", 1)[0]
        while s.endswith(")") and s.count("(") != s.count(")"):
            s = s[:-1]
        s = s.strip()
        if not s:
            continue
        op, _, rest = s.partition(" ")
        instrs.append((op, rest.strip()))
    b.locals = locs
    b.instrs = instrs
    b.targeted = _prescan_targets(instrs)
    return b


def _prescan_targets(instrs):
    """For every block/loop/if instruction index, is it the target of a branch?

    Returns dict: opener_instr_index -> True.  Also resolves which label a
    `br`-family instruction addresses so the emitter never has to guess.
    """
    stack = []
    targeted = {}
    for i, (op, rest) in enumerate(instrs):
        if op in ("block", "loop", "if"):
            stack.append(i)
        elif op == "end":
            if stack:
                stack.pop()
        elif op == "else":
            pass
        elif op == "br" or op == "br_if":
            try:
                d = int(rest.split()[0])
            except (ValueError, IndexError):
                continue
            if 0 <= d < len(stack):
                targeted[stack[-1 - d]] = True
        elif op == "br_table":
            for tok in rest.split():
                if tok.startswith("("):
                    continue
                try:
                    d = int(tok)
                except ValueError:
                    continue
                if 0 <= d < len(stack):
                    targeted[stack[-1 - d]] = True
    return targeted


# ---------------------------------------------------------------------------
# literal formatting
# ---------------------------------------------------------------------------

def fmt_int(val, ty):
    """wasm integer literal -> Rust literal of the same bit pattern."""
    bits = 32 if ty == "i32" else 64
    val &= (1 << bits) - 1
    if val >= 1 << (bits - 1):
        val -= 1 << bits
    # Suffix the literal with its wasm type: an unsuffixed `126` makes
    # `126.wrapping_sub(x)` ambiguous (`{integer}`) and `-58 as u64` negates a
    # u64 (E0600).  `-58i64` is a legal literal expression, and `(-58i64) as u64`
    # is the intended bit pattern.
    return "(%d%s)" % (val, ty) if val < 0 else "%d%s" % (val, ty)


def _f32_lit(x):
    import struct
    packed = struct.pack("<f", x)
    bits = struct.unpack("<I", packed)[0]
    for cand in ("%.9g" % x, "%.7g" % x, "%.1f" % x):
        try:
            if struct.pack("<f", float(cand)) == packed:
                s = cand
                break
        except (ValueError, OverflowError):
            continue
    else:
        s = None
    if s is None or "nan" in s or "inf" in s:
        return "f32::from_bits(0x%08xu32)" % bits
    if "e" not in s and "." not in s:
        s += ".0"
    return "%sf32" % s


def _f64_lit(x):
    import struct
    packed = struct.pack("<d", x)
    for cand in ("%.17g" % x, "%.15g" % x):
        try:
            if struct.pack("<d", float(cand)) == packed:
                s = cand
                break
        except (ValueError, OverflowError):
            continue
    else:
        s = None
    if s is None or "nan" in s or "inf" in s:
        return "f64::from_bits(0x%016xu64)" % struct.unpack("<Q", packed)[0]
    if "e" not in s and "." not in s:
        s += ".0"
    return "%sf64" % s


def fmt_float(txt, ty):
    import struct
    t = txt.strip()
    neg = False
    if t.startswith("-"):
        neg = True
        t = t[1:]
    try:
        if t.startswith(("nan", "inf")):
            if t.startswith("nan"):
                if ":" in t:
                    payload = int(t.split(":", 1)[1], 0)
                else:
                    payload = 0x400000
                if ty == "f32":
                    v = struct.unpack("<f", struct.pack("<I", (payload & 0x7FFFFF) | 0x7F800000))[0]
                else:
                    v = struct.unpack("<d", struct.pack("<Q", (payload & 0xF_FFFF_FFFF_FFFF) | 0x7FF0_0000_0000_0000))[0]
            else:
                v = float("inf")
            x = -v if neg else v
        else:
            x = float.fromhex(t) if ("x" in t or "X" in t) else float(t)
            if neg:
                x = -x
    except ValueError:
        x = 0.0
    return _f32_lit(x) if ty == "f32" else _f64_lit(x)


def fmt_v128(rest):
    """`i32x4 0x0 0x0 0x0 0x0` -> u128 literal."""
    toks = rest.split()
    if not toks:
        return "0u128"
    kind = toks[0]
    lanes = toks[1:]
    widths = {"i8x16": 8, "i16x8": 16, "i32x4": 32, "i64x2": 64,
              "f32x4": 32, "f64x2": 64}
    w = widths.get(kind, 32)
    val = 0
    for i, t in enumerate(lanes):
        if kind.startswith("f"):
            import struct
            f = float.fromhex(t) if "p" in t or "x" in t else float(t)
            bits = struct.unpack("<I", struct.pack("<f", f))[0] if w == 32 else struct.unpack("<Q", struct.pack("<d", f))[0]
        else:
            bits = int(t, 0) & ((1 << w) - 1)
        val |= bits << (i * w)
    return "0x%032xu128" % val


def parse_memargs(rest):
    """`offset=56 align=4` -> (offset, align, other tokens)"""
    off = 0
    align = None
    other = []
    for tok in rest.split():
        if tok.startswith("offset="):
            off = int(tok[7:])
        elif tok.startswith("align="):
            try:
                align = int(tok[6:])
            except ValueError:
                align = None
        elif tok.startswith("("):
            continue
        else:
            other.append(tok)
    return off, align, other


# ---------------------------------------------------------------------------
# structured emitter
# ---------------------------------------------------------------------------

class Frame(object):
    __slots__ = ("kind", "label", "res", "resvars", "base", "wrapped", "opener",
                 "else_open", "dead", "name")

    def __init__(self, kind, label, res, resvars, base, wrapped, opener):
        self.kind = kind
        self.label = label
        self.res = res
        self.resvars = resvars
        self.base = base
        self.wrapped = wrapped
        self.opener = opener
        self.else_open = False
        self.dead = False
        self.name = label


_SEG_NAMES = None


def _seg_names():
    """name -> index for the module's passive data segments (built once)."""
    global _SEG_NAMES
    if _SEG_NAMES is None:
        import json, pathlib
        p = pathlib.Path(__file__).with_name('seg_names.json')
        _SEG_NAMES = json.loads(p.read_text(encoding='utf8')) if p.exists() else {}
    return _SEG_NAMES


class Emitter(object):
    """Structured (block/loop/if) translation of one function body."""

    def __init__(self, body, idx, gmap, gtypes, funcmap, imports):
        self.b = body
        self.idx = idx
        self.gmap = gmap            # global name -> (index, wasm type)
        self.gtypes = gtypes
        self.funcmap = funcmap      # func name -> (idx, params, results, is_import)
        self.imports = imports
        self.stack = []
        self.frames = []
        self.out = []
        self.depth = 1
        self.ntemp = 0
        self.nlabel = 0
        self.instr_count = 0
        self.untranslated = {}
        self.dead_skipped = 0
        self.warnings = []
        self.failed = False
        self.fail_reason = None
        self.callees = set()
        # locals
        self.vars = []              # wasm types
        self.vars.extend(body.params)
        self.vars.extend(body.locals)

    # -- output ---------------------------------------------------------
    def emit(self, text):
        self.out.append("    " * self.depth + text)

    def comment(self, text):
        self.emit("// " + text)

    def newtemp(self):
        self.ntemp += 1
        return "t%d" % self.ntemp

    def newlabel(self):
        self.nlabel += 1
        return "L%d" % self.nlabel

    # -- stack ----------------------------------------------------------
    def push(self, ty, code, refs=(), atom=False):
        self.stack.append((ty, code, refs, atom))

    def pop(self):
        if not self.stack:
            raise Unsupported("stack underflow")
        return self.stack.pop()

    def popv(self):
        return self.pop()[1]

    def peek(self, i):
        return self.stack[-1 - i]

    def seal(self, i):
        ty, code, refs, atom = self.stack[i]
        if atom:
            return code
        t = self.newtemp()
        self.emit("let %s: %s = %s;" % (t, RUST[ty], code))
        self.stack[i] = (ty, t, (), True)
        return t

    def seal_all(self):
        for i in range(len(self.stack)):
            self.seal(i)

    def seal_top(self, n):
        for i in range(len(self.stack) - n, len(self.stack)):
            if i >= 0:
                self.seal(i)

    def seal_refs(self, names):
        if not names:
            return
        for i in range(len(self.stack)):
            ty, code, refs, atom = self.stack[i]
            if atom or not refs:
                continue
            for r in refs:
                if r in names:
                    self.seal(i)
                    break

    # -- statements -----------------------------------------------------
    def stmt(self, text, seals=True):
        if seals:
            self.seal_all()
        self.emit(text)

    # -- debugging ------------------------------------------------------
    def note_untranslated(self, op, raw):
        self.untranslated[op] = self.untranslated.get(op, 0) + 1
        self.emit("// UNTRANSLATED (%s): %s" % (op, raw))

    # -- expressions ----------------------------------------------------
    def _fold(self, ty, code, refs, siza, sizeb):
        return (ty, code, refs, False)

    def binop(self, ty, tmpl, otys=None):
        b = self.pop()
        a = self.pop()
        refs = a[2] + b[2] if (a[2] or b[2]) else ()
        code = tmpl.format(a=a[1], b=b[1])
        if len(code) > 180 or (len(a[2]) + len(b[2])) > 8:
            # keep the output readable: materialise both sides
            ia = len(self.stack)
            self.stack.append(a)
            ta = self.seal(ia)
            self.stack.pop()
            self.stack.append(b)
            tb = self.seal(len(self.stack) - 1)
            self.stack.pop()
            code = tmpl.format(a=ta, b=tb)
            refs = ()
        self.push(ty, code, refs, False)

    def unop(self, ty, tmpl):
        a = self.pop()
        code = tmpl.format(a=a[1])
        self.push(ty, code, a[2], False)

    # -- control helpers -------------------------------------------------
    def cur(self):
        return self.frames[-1]

    def target(self, depth):
        """frame addressed by `depth` (0 = innermost), or None for function."""
        k = len(self.frames) - 1 - depth
        if k < 0:
            return None
        return self.frames[k]

    def assign_results(self, frame):
        """Move the current stack top onto the frame's result variables."""
        for i in range(len(frame.res) - 1, -1, -1):
            ty, code, refs, atom = self.pop()
            if frame.resvars:
                self.emit("%s = %s;" % (frame.resvars[i], code))

    def close_frame(self, frame):
        self.depth -= 1
        if frame.wrapped:
            self.emit("break '%s;" % frame.label)
        self.emit("}")
        for i, rv in enumerate(frame.resvars):
            self.push(frame.res[i], rv, (), True)

    # -- entry ----------------------------------------------------------
    def run(self):
        b = self.b
        # signature
        rty = [RUST[t] for t in b.results]
        if len(rty) == 0:
            ret = ""
        elif len(rty) == 1:
            ret = " -> %s" % rty[0]
        else:
            ret = " -> (%s)" % ", ".join(rty)
        args = ["ph: &mut Phantom"]
        for i, t in enumerate(b.params):
            args.append("mut l%d: %s" % (i, RUST[t]))
        self.emit('#[no_mangle]\n    pub unsafe extern "C" fn %s(%s)%s {' % (self.fname, ", ".join(args), ret))
        self.depth = 2
        for i in range(len(b.params), len(self.vars)):
            self.emit("let mut l%d: %s = %s;" % (i, RUST[self.vars[i]], ZERO[self.vars[i]]))

        fnframe = Frame("func", "fn", b.results, [], 0, False, -1)
        self.frames.append(fnframe)
        self.walk()
        # implicit return of the fall-through stack
        if not fnframe.dead:
            n = len(b.results)
            if n and len(self.stack) >= n:
                vals = [self.stack[-n + k][1] for k in range(n)]
                if n == 1:
                    self.emit("return %s;" % vals[0])
                else:
                    self.emit("return (%s);" % ", ".join(vals))
            elif n:
                self.emit("// no value on the operand stack at the function end")
                self.emit("rt::wasm_unreachable()")
        self.depth = 1
        self.emit("}")
        return self.out

    # -- main walk -------------------------------------------------------
    def walk(self):
        instrs = self.b.instrs
        i = 0
        n = len(instrs)
        dead_nest = 0
        dead_emitted = 0
        while i < n:
            op, rest = instrs[i]
            cur = self.frames[-1]
            if cur.dead:
                if op in ("block", "loop", "if", "try", "try_table"):
                    dead_nest += 1
                    self.instr_count += 1
                    self.dead_skipped += 1
                    i += 1
                    continue
                if op == "end":
                    if dead_nest:
                        dead_nest -= 1
                        self.instr_count += 1
                        self.dead_skipped += 1
                        i += 1
                        continue
                    self.instr_count += 1
                    i = self.step(i, op, rest)
                    continue
                if op == "else":
                    if dead_nest:
                        self.instr_count += 1
                        self.dead_skipped += 1
                        i += 1
                        continue
                    self.instr_count += 1
                    i = self.step(i, op, rest)
                    continue
                # ordinary unreachable instruction
                self.instr_count += 1
                self.dead_skipped += 1
                if dead_emitted < 6:
                    self.emit("// [dead] %s" % (op + " " + rest).strip())
                    dead_emitted += 1
                elif dead_emitted == 6:
                    self.emit("// [dead] ... (further dead instructions elided, see WAT)")
                    dead_emitted += 1
                i += 1
                continue
            dead_emitted = 0
            self.instr_count += 1
            try:
                i = self.step(i, op, rest)
            except Unsupported as exc:
                self.failed = True
                self.fail_reason = "%s at instr %d (%s)" % (exc, i, op)
                self.emit("// UNTRANSLATED FUNCTION: %s" % exc)
                self.untranslated["<fatal:%s>" % op] = self.untranslated.get("<fatal:%s>" % op, 0) + 1
                return

    def step(self, i, op, rest):
        h = self.HANDLERS.get(op)
        if h is not None:
            return h(self, i, op, rest)
        # table driven numeric ops
        spec = NUM.get(op)
        if spec is not None:
            argtys, res, tmpl = spec
            n = len(argtys)
            if n == 1:
                self.unop(res, tmpl)
            else:
                self.binop(res, tmpl)
            return i + 1
        spec = LOADS.get(op)
        if spec is not None:
            self.do_load(op, rest, spec)
            return i + 1
        spec = STORES.get(op)
        if spec is not None:
            self.do_store(op, rest, spec)
            return i + 1
        spec = SIMD.get(op)
        if spec is not None:
            self.do_simd(op, rest, spec)
            return i + 1
        spec = ATOMIC.get(op)
        if spec is not None:
            self.do_atomic(op, rest, spec)
            return i + 1
        self.note_untranslated(op, (op + " " + rest).strip())
        return i + 1

    # -- memory ----------------------------------------------------------
    def addr_expr(self, off):
        a = self.pop()[1]
        if off:
            if a.replace("_", "").replace(".", "").replace("-", "").isalnum() or re.match(r"^[a-zA-Z_][\w.]*$", a):
                return "%s + %d" % (a, off)
            return "(%s) + %d" % (a, off)
        return a

    def do_load(self, op, rest, spec):
        helper, cast, ty = spec
        off, _align, _other = parse_memargs(rest)
        a = self.addr_expr(off)
        t = self.newtemp()
        self.emit("let %s: %s = ph.%s(%s)%s;" % (t, RUST[ty], helper, a, cast))
        self.push(ty, t, (), True)

    def do_store(self, op, rest, spec):
        helper, cast = spec
        off, _align, _other = parse_memargs(rest)
        v = self.pop()
        a = self.addr_expr(off)
        self.seal_all()
        self.emit("ph.%s(%s, %s%s);" % (helper, a, v[1], cast))

    def do_atomic(self, op, rest, spec):
        kind, helper, otys, rty = spec
        off, _align, _other = parse_memargs(rest)
        vals = [self.pop() for _ in otys]
        a = self.addr_expr(off)
        self.seal_all()
        if kind == "load":
            t = self.newtemp()
            cast = " as i32" if rty == "i32" else " as i64"
            if rty == "i64" and helper == "a_load_u32":
                cast = " as u32 as i64"
            self.emit("let %s: %s = ph.%s(%s)%s;" % (t, RUST[rty], helper, a, cast))
            self.push(rty, t, (), True)
        elif kind == "store":
            self.emit("ph.%s(%s, %s);" % (helper, a, vals[0][1]))
        elif kind == "rmw":
            t = self.newtemp()
            self.emit("let %s: %s = ph.%s(%s, %s);" % (t, RUST[rty], helper, a, vals[0][1]))
            self.push(rty, t, (), True)
        elif kind == "cmpxchg":
            t = self.newtemp()
            self.emit("let %s: %s = ph.%s(%s, %s, %s);" % (t, RUST[rty], helper, a, vals[1][1], vals[0][1]))
            self.push(rty, t, (), True)
        elif kind == "notify":
            t = self.newtemp()
            self.emit("let %s: i32 = ph.%s(%s, %s);" % (t, helper, a, vals[0][1]))
            self.push("i32", t, (), True)
        elif kind == "wait":
            t = self.newtemp()
            self.emit("let %s: i32 = ph.%s(%s, %s, %s);" % (t, helper, a, vals[1][1], vals[0][1]))
            self.push("i32", t, (), True)

    # -- simd ------------------------------------------------------------
    def do_simd(self, op, rest, spec):
        kind = spec[0]
        name = "simd::" + rust_name(op)
        off, _align, other = parse_memargs(rest)
        if kind == "raw":
            if op == "i8x16.shuffle":
                # [v1 v2] + 16 immediate lane indices
                lanes = [int(t) for t in rest.split() if not t.startswith(("offset", "align"))]
                imm = 0
                for k, l in enumerate(lanes[:16]):
                    imm |= (l & 0xFF) << (8 * k)
                b = self.pop()[1]
                a = self.pop()[1]
                t = self.newtemp()
                if len(lanes) != 16 or any(lane < 0 or lane >= 32 for lane in lanes):
                    raise Unsupported("invalid shuffle immediate")
                self.emit("let %s: u128 = simd::i8x16_shuffle_static::<%s>(%s, %s);" % (t, ",".join(map(str, lanes)), a, b))
                self.push("v128", t, (), True)
                return
            # infer operand count from the spec body's signature
            sig = spec[1]
            nargs = sig.count(",") - sig.count("->") + 1 if "(" in sig else 1
            m = re.search(r"fn\s+\w+\s*\(([^)]*)\)", sig)
            params = [p for p in m.group(1).split(",") if p.strip()] if m else []
            tys = []
            for p in params:
                t = p.split(":")[-1].strip()
                tys.append(t)
            if len(tys) == 0:
                args = ""
            else:
                vals = [self.pop()[1] for _ in tys][::-1]
                # bitselect operand order in wasm: (a, b, c) = (v1, v2, mask)
                args = ", ".join(vals)
            # The spec body declares the real result type (`-> i32` for
            # v128.any_true / all_true / bitmask). Assuming V here put an i32
            # call into a u128 temporary (E0308).
            mret = re.search(r'->\s*([^\{]+)\{', sig)
            ret = mret.group(1).strip() if mret else "V"
            rty = {"V": "u128", "i32": "i32", "i64": "i64", "f32": "f32", "f64": "f64",
                   "u32": "u32", "u64": "u64"}.get(ret, ret)
            wsp = {"u128": "v128", "i32": "i32", "i64": "i64", "f32": "f32", "f64": "f64",
                   "u32": "i32", "u64": "i64"}.get(rty, "v128")
            t = self.newtemp()
            self.emit("let %s: %s = %s(%s);" % (t, rty, name, args))
            self.push(wsp, t, (), True)
            return
        if kind == "b2":
            b = self.pop()[1]
            a = self.pop()[1]
            t = self.newtemp()
            self.emit("let %s: u128 = %s(%s, %s);" % (t, name, a, b))
            self.push("v128", t, (), True)
        elif kind == "c2":
            b = self.pop()[1]
            a = self.pop()[1]
            t = self.newtemp()
            self.emit("let %s: u128 = %s(%s, %s);" % (t, name, a, b))
            self.push("v128", t, (), True)
        elif kind == "u1":
            a = self.pop()[1]
            t = self.newtemp()
            self.emit("let %s: u128 = %s(%s);" % (t, name, a))
            self.push("v128", t, (), True)
        elif kind == "sh":
            s = self.pop()[1]
            a = self.pop()[1]
            t = self.newtemp()
            self.emit("let %s: u128 = %s(%s, %s);" % (t, name, a, s))
            self.push("v128", t, (), True)
        elif kind == "spl":
            a = self.pop()
            t = self.newtemp()
            ty = spec[1]
            # wasm takes an i32/i64/f32/f64 lane value; the helper is typed by lane
            self.emit("let %s: u128 = %s(%s as %s);" % (t, name, a[1], ty))
            self.push("v128", t, (), True)
        elif kind == "ex":
            lane = other[-1] if other else "0"
            a = self.pop()[1]
            ty = spec[1]
            t = self.newtemp()
            wty = {"i8": "i32", "u8": "i32", "i16": "i32", "u16": "i32",
                   "i32": "i32", "u32": "i32", "i64": "i64", "u64": "i64",
                   "f32": "f32", "f64": "f64"}[ty]
            # the helper returns the narrow lane type; wasm pushes the wide one
            self.emit("let %s: %s = %s(%s, %s) as %s;" % (t, wty, name, a, lane, wty))
            self.push(wty, t, (), True)
        elif kind == "rep":
            lane = other[-1] if other else "0"
            v = self.pop()[1]
            a = self.pop()[1]
            t = self.newtemp()
            cast = ""
            if spec[1] in ("i8", "u8", "i16", "u16"):
                cast = " as %s" % spec[1]
            self.emit("let %s: u128 = %s(%s, %s, %s%s);" % (t, name, a, lane, v, cast))
            self.push("v128", t, (), True)
        elif kind == "tst":
            a = self.pop()[1]
            t = self.newtemp()
            self.emit("let %s: i32 = %s(%s);" % (t, name, a))
            self.push("i32", t, (), True)

    # -- control flow ----------------------------------------------------
    def h_block(self, i, op, rest):
        res, params = self._blocktype(rest)
        if params:
            raise Unsupported("block with params")
        self.seal_all()
        label = self.newlabel()
        resvars = ["r%d" % (self.newtemp_num()) for _ in res]
        for rv, ty in zip(resvars, res):
            self.emit("let mut %s: %s = %s;" % (rv, RUST[ty], ZERO[ty]))
        wrapped = self.b.targeted.get(i, False)
        if wrapped:
            self.emit("'%s: loop {" % label)
        else:
            self.emit("{")
        self.depth += 1
        self.frames.append(Frame("block", label, res, resvars, len(self.stack), wrapped, i))
        return i + 1

    def h_loop(self, i, op, rest):
        res, params = self._blocktype(rest)
        if params:
            raise Unsupported("loop with params")
        self.seal_all()
        label = self.newlabel()
        resvars = ["r%d" % (self.newtemp_num()) for _ in res]
        for rv, ty in zip(resvars, res):
            self.emit("let mut %s: %s = %s;" % (rv, RUST[ty], ZERO[ty]))
        self.emit("'%s: loop {" % label)
        self.depth += 1
        self.frames.append(Frame("loop", label, res, resvars, len(self.stack), True, i))
        return i + 1

    def h_if(self, i, op, rest):
        res, params = self._blocktype(rest)
        if params:
            raise Unsupported("if with params")
        cond = self.pop()
        self.seal_all()
        base_label = self.newlabel()
        resvars = ["r%d" % (self.newtemp_num()) for _ in res]
        for rv, ty in zip(resvars, res):
            self.emit("let mut %s: %s = %s;" % (rv, RUST[ty], ZERO[ty]))
        wrapped = self.b.targeted.get(i, False)
        self.emit("if %s != 0 {" % cond[1])
        self.depth += 1
        tl = base_label + "a"
        if wrapped:
            self.emit("'%s: loop {" % tl)
            self.depth += 1
        self.frames.append(Frame("if", tl, res, resvars, len(self.stack), wrapped, i))
        return i + 1

    def h_else(self, i, op, rest):
        f = self.cur()
        if f.kind != "if":
            raise Unsupported("else outside if")
        if not f.dead:
            self.assign_results(f)
        if f.wrapped:
            self.depth -= 1
            self.emit("break '%s;" % f.label)
            self.emit("}")
        self.depth -= 1
        self.emit("} else {")
        self.depth += 1
        el = f.label[:-1] + "b" if f.label.endswith("a") else f.label + "b"
        if f.wrapped:
            self.emit("'%s: loop {" % el)
            self.depth += 1
        f.label = el
        f.else_open = True
        f.dead = False
        return i + 1

    def h_end(self, i, op, rest):
        f = self.frames.pop()
        if f.kind == "func":
            self.frames.append(f)
            return i + 1
        if f.kind == "if":
            if not f.dead:
                self.assign_results(f)
            # an `if` opens exactly one brace (`} else {` already closed the
            # then-arm), plus one more if the arm was wrapped in a labelled loop
            if f.wrapped:
                self.depth -= 1
                self.emit("break '%s;" % f.label)
                self.depth -= 1
                self.emit("}")
            self.depth -= 1
            self.emit("}")
            for i2, rv in enumerate(f.resvars):
                self.push(f.res[i2], rv, (), True)
            return i + 1
        if not f.dead:
            self.assign_results(f)
        self.close_frame(f)
        return i + 1

    def newtemp_num(self):
        self.ntemp += 1
        return self.ntemp

    def _blocktype(self, rest):
        m = RE_BLOCKT.match(rest.strip())
        res = []
        params = []
        if m:
            if m.group(2):
                for pm in RE_PARAMS.finditer(m.group(2)):
                    params.extend(_valtypes(pm.group(1)))
            if m.group(3):
                for rm in RE_RESULTS.finditer(m.group(3)):
                    res.extend(_valtypes(rm.group(1)))
        return res, params

    # -- branches --------------------------------------------------------
    def _kill(self, base):
        """Truncate the operand stack to `base` and mark the current frame dead."""
        if len(self.stack) > base:
            del self.stack[base:]
        self.cur().dead = True

    @staticmethod
    def branch_arity(f):
        """Values a branch to `f` must carry.

        wasm label types: a `loop` label takes the loop's *parameters* (this
        module has none), a `block`/`if` label takes the block's results.
        """
        if f is None or f.kind == "func":
            return None
        if f.kind == "loop":
            return 0
        return len(f.res)

    def _br_to(self, f, vals):
        """Emit code that transfers control to frame `f` (None = return)."""
        if f is None or f.kind == "func":
            if self.b.results:
                e = vals[0] if len(vals) == 1 else "(%s)" % ", ".join(vals)
                return "return %s;" % e
            return "return;"
        if f.kind == "loop":
            return "continue '%s;" % f.label
        parts = []
        for rv, v in zip(f.resvars, vals):
            parts.append("%s = %s;" % (rv, v))
        parts.append("break '%s;" % f.label)
        return " ".join(parts)

    def h_br(self, i, op, rest):
        d = int(rest.split()[0])
        f = self.target(d)
        n = self.branch_arity(f) or 0
        vals = [self.pop()[1] for _ in range(n)][::-1]
        self.seal_all()
        self.emit(self._br_to(f, vals))
        base = self.frames[0].base if f is None else f.base
        self._kill(self.cur().base)
        return i + 1

    def h_br_if(self, i, op, rest):
        d = int(rest.split()[0])
        cond = self.pop()
        f = self.target(d)
        n = self.branch_arity(f) or 0
        self.seal_top(n)
        vals = [self.peek(n - 1 - k)[1] for k in range(n)]
        self.seal_all()
        self.emit("if %s != 0 {" % cond[1])
        self.depth += 1
        self.emit(self._br_to(f, vals))
        self.depth -= 1
        self.emit("}")
        return i + 1

    def h_br_table(self, i, op, rest):
        tgts = [int(t) for t in rest.split() if not t.startswith("(")]
        if not tgts:
            raise Unsupported("empty br_table")
        idx = self.pop()
        self.seal_all()
        default = tgts[-1]
        cases = tgts[:-1]
        arities = set()
        for d in tgts:
            arities.add(self.branch_arity(self.target(d)) or 0)
        if len(arities) != 1:
            raise Unsupported("br_table with mixed arities")
        n = arities.pop()
        vals = [self.pop()[1] for _ in range(n)][::-1]
        if n:
            temps = []
            for v in vals:
                t = self.newtemp()
                self.emit("let %s = %s;" % (t, v))
                temps.append(t)
            vals = temps
        self.emit("match %s {" % idx[1])
        self.depth += 1
        for k, d in enumerate(cases):
            self.emit("%d => { %s }" % (k, self._br_to(self.target(d), vals)))
        self.emit("_ => { %s }" % self._br_to(self.target(default), vals))
        self.depth -= 1
        self.emit("}")
        self.cur().dead = True
        return i + 1

    def h_return(self, i, op, rest):
        n = len(self.b.results)
        vals = [self.pop()[1] for _ in range(n)][::-1]
        self.seal_all()
        if vals:
            self.emit("return %s;" % (vals[0] if len(vals) == 1 else "(%s)" % ", ".join(vals)))
        else:
            self.emit("return;")
        self._kill(self.cur().base)
        return i + 1

    def h_unreachable(self, i, op, rest):
        self.emit("rt::wasm_unreachable();")
        self._kill(self.cur().base)
        return i + 1

    # -- variables -------------------------------------------------------
    def h_local_get(self, i, op, rest):
        k = int(rest)
        ty = self.vars[k]
        self.push(ty, "l%d" % k, ("l%d" % k,), False)
        return i + 1

    def h_local_set(self, i, op, rest):
        k = int(rest)
        v = self.pop()
        self.seal_refs(("l%d" % k,))
        self.seal_all()
        self.emit("l%d = %s;" % (k, v[1]))
        return i + 1

    def h_local_tee(self, i, op, rest):
        k = int(rest)
        v = self.pop()
        self.seal_refs(("l%d" % k,))
        self.seal_all()
        self.emit("l%d = %s;" % (k, v[1]))
        self.push(self.vars[k], "l%d" % k, ("l%d" % k,), False)
        return i + 1

    def _global_ref(self, name):
        if name.startswith("$"):
            name = name[1:]
        got = self.gmap.get(name)
        if got is None:
            return None
        return got

    def h_global_get(self, i, op, rest):
        g = self._global_ref(rest.strip())
        if g is None:
            self.note_untranslated(op, "global.get " + rest)
            return i + 1
        gidx, ty = g
        if gidx == 0:
            self.push("i64", "ph.sp", (), False)
            return i + 1
        if ty == "i32":
            self.push("i32", "(ph.g[%d] as u32 as i32)" % gidx, (), False)
        elif ty == "i64":
            self.push("i64", "(ph.g[%d] as i64)" % gidx, (), False)
        elif ty == "f32":
            self.push("f32", "f32::from_bits(ph.g[%d] as u32)" % gidx, (), False)
        elif ty == "f64":
            self.push("f64", "f64::from_bits(ph.g[%d])" % gidx, (), False)
        elif ty == "v128":
            self.push("v128", "(ph.g[%d] as u128)" % gidx, (), False)
        else:
            self.note_untranslated(op, "global.get " + rest)
        return i + 1

    def h_global_set(self, i, op, rest):
        g = self._global_ref(rest.strip())
        v = self.pop()
        self.seal_all()
        if g is None:
            self.note_untranslated(op, "global.set " + rest)
            return i + 1
        gidx, ty = g
        if gidx == 0:
            self.emit("ph.sp = %s;" % v[1])
        elif ty == "i32":
            self.emit("ph.g[%d] = (%s as u32) as u64;" % (gidx, v[1]))
        elif ty == "i64":
            self.emit("ph.g[%d] = %s as u64;" % (gidx, v[1]))
        elif ty == "f32":
            self.emit("ph.g[%d] = %s.to_bits() as u64;" % (gidx, v[1]))
        elif ty == "f64":
            self.emit("ph.g[%d] = %s.to_bits();" % (gidx, v[1]))
        else:
            self.emit("ph.g[%d] = %s as u64;" % (gidx, v[1]))
        return i + 1

    # -- calls -----------------------------------------------------------
    def h_call(self, i, op, rest):
        name = rest.strip()
        target = self.funcmap.get(name[1:] if name.startswith("$") else name)
        if target is None:
            self.note_untranslated(op, "call " + rest)
            return i + 1
        tidx, ptypes, rtypes, is_import = target[:4]
        args = [self.pop()[1] for _ in ptypes][::-1]
        self.seal_all()
        tname = target[4]
        self.callees.add(tname)
        if is_import:
            argstr = ", ".join(args)
            call = "imp::%s(%s)" % (tname, argstr)
        else:
            argstr = ", ".join(["ph"] + args)
            call = "syms::%s(%s)" % (tname, argstr)
        if rtypes:
            t = self.newtemp()
            if len(rtypes) == 1:
                self.emit("let %s: %s = %s;" % (t, RUST[rtypes[0]], call))
                self.push(rtypes[0], t, (), True)
            else:
                rs = [self.newtemp() for _ in rtypes]
                self.emit("let (%s): (%s) = %s;" % (", ".join(rs),
                                                    ", ".join(RUST[r] for r in rtypes), call))
                for r_ty, r_n in zip(rtypes, rs):
                    self.push(r_ty, r_n, (), True)
        else:
            self.emit("%s;" % call)
        return i + 1

    def h_call_indirect(self, i, op, rest):
        m = re.search(r"\(type\s+(\d+)\)", rest)
        if not m:
            self.note_untranslated(op, "call_indirect " + rest)
            return i + 1
        tidx = int(m.group(1))
        sig = self.type_sigs.get(tidx, ([], []))
        ptypes, rtypes = sig
        elemidx = self.pop()[1]
        args = [self.pop()[1] for _ in ptypes][::-1]
        self.seal_all()
        fp = self.newtemp()
        self.emit("let %s = ph.table_entry(%s);" % (fp, elemidx))
        fnty = "unsafe extern \"C\" fn(*mut Phantom%s)%s" % (
            "".join(", " + RUST[t] for t in ptypes),
            ("" if not rtypes else " -> " + (RUST[rtypes[0]] if len(rtypes) == 1 else "(" + ", ".join(RUST[r] for r in rtypes) + ")")))
        fnv = self.newtemp()
        self.emit("let %s: %s = core::mem::transmute(%s);" % (fnv, fnty, fp))
        argstr = ", ".join(["ph as *mut Phantom"] + args)
        if rtypes:
            t = self.newtemp()
            if len(rtypes) == 1:
                self.emit("let %s: %s = %s(%s);" % (t, RUST[rtypes[0]], fnv, argstr))
                self.push(rtypes[0], t, (), True)
            else:
                rs = [self.newtemp() for _ in rtypes]
                self.emit("let (%s): (%s) = %s(%s);" % (", ".join(rs),
                                                        ", ".join(RUST[r] for r in rtypes), fnv, argstr))
                for r_ty, r_n in zip(rtypes, rs):
                    self.push(r_ty, r_n, (), True)
        else:
            self.emit("%s(%s);" % (fnv, argstr))
        return i + 1

    # -- stack ops -------------------------------------------------------
    def h_drop(self, i, op, rest):
        self.pop()
        return i + 1

    def h_select(self, i, op, rest):
        c = self.pop()
        b = self.pop()
        a = self.pop()
        ty = a[0]
        t = self.newtemp()
        f = {"i32": "select_i32", "i64": "select_i64", "f32": "select_f32",
             "f64": "select_f64", "v128": "select_v128"}[ty]
        self.emit("let %s: %s = rt::%s(%s, %s, %s);" % (t, RUST[ty], f, c[1], b[1], a[1]))
        self.push(ty, t, (), True)
        return i + 1

    def h_nop(self, i, op, rest):
        return i + 1

    # -- constants -------------------------------------------------------
    def h_i32_const(self, i, op, rest):
        self.push("i32", fmt_int(int(rest, 0), "i32"), (), True)
        return i + 1

    def h_i64_const(self, i, op, rest):
        self.push("i64", fmt_int(int(rest, 0), "i64"), (), True)
        return i + 1

    def h_f32_const(self, i, op, rest):
        self.push("f32", fmt_float(rest, "f32"), (), True)
        return i + 1

    def h_f64_const(self, i, op, rest):
        self.push("f64", fmt_float(rest, "f64"), (), True)
        return i + 1

    def h_v128_const(self, i, op, rest):
        self.push("v128", fmt_v128(rest), (), True)
        return i + 1

    # -- bulk memory -----------------------------------------------------
    def h_memory_copy(self, i, op, rest):
        n = self.pop()[1]
        s = self.pop()[1]
        d = self.pop()[1]
        self.seal_all()
        self.emit("ph.memory_copy(%s, %s, %s);" % (d, s, n))
        return i + 1

    def h_memory_fill(self, i, op, rest):
        n = self.pop()[1]
        v = self.pop()[1]
        d = self.pop()[1]
        self.seal_all()
        self.emit("ph.memory_fill(%s, %s, %s);" % (d, v, n))
        return i + 1

    def h_memory_init(self, i, op, rest):
        segname = rest.strip()
        seg = self.seg_index(segname)
        n = self.pop()[1]
        s = self.pop()[1]
        d = self.pop()[1]
        self.seal_all()
        self.emit("ph.memory_init(%d, %s, %s, %s);" % (seg, d, s, n))
        return i + 1

    def h_data_drop(self, i, op, rest):
        seg = self.seg_index(rest.strip())
        self.seal_all()
        self.emit("ph.data_drop(%d);" % seg)
        return i + 1

    def h_memory_size(self, i, op, rest):
        t = self.newtemp()
        self.emit("let %s: i64 = ph.memory_size();" % t)
        self.push("i64", t, (), True)
        return i + 1

    def h_memory_grow(self, i, op, rest):
        d = self.pop()[1]
        self.seal_all()
        t = self.newtemp()
        self.emit("let %s: i64 = ph.memory_grow(%s);" % (t, d))
        self.push("i64", t, (), True)
        return i + 1

    def seg_index(self, name):
        """Data-segment operand -> real segment index.

        wabt names passive segments in the WAT (`memory.init $.rodata`), so the
        numeric index is not in the text: the order of the `(data $…)` forms in
        the module *is* the index order, captured once in seg_names.json.
        """
        name = (name or '').strip()
        if not name:
            raise ValueError('memory.init/data.drop without a segment operand')
        if name.startswith('$'):
            nm = name[1:]
            if nm.isdigit():
                return int(nm)
            mp = _seg_names()
            if nm not in mp:
                raise KeyError('unknown data segment %r' % nm)
            return mp[nm]
        return int(name)

    # -- simd memory -----------------------------------------------------
    def h_splat_load(self, i, op, rest):
        width, zero = SPLAT_LOADS[op]
        off, _a, _o = parse_memargs(rest)
        a = self.addr_expr(off)
        t = self.newtemp()
        self.emit("let %s: u128 = ph.read_splat(%s, %d);" % (t, a, width))
        self.push("v128", t, (), True)
        return i + 1

    def h_lane_load(self, i, op, rest):
        width = LANE_LOADS[op]
        off, _a, other = parse_memargs(rest)
        lane = other[-1] if other else "0"
        v = self.pop()[1]
        a = self.addr_expr(off)
        t = self.newtemp()
        self.emit("let %s: u128 = ph.load_lane(%s, %s, %s, %d);" % (t, a, v, lane, width))
        self.push("v128", t, (), True)
        return i + 1

    def h_lane_store(self, i, op, rest):
        width = LANE_STORES[op]
        off, _a, other = parse_memargs(rest)
        lane = other[-1] if other else "0"
        v = self.pop()[1]
        a = self.addr_expr(off)
        self.seal_all()
        self.emit("ph.store_lane(%s, %s, %s, %d);" % (a, v, lane, width))
        return i + 1


HANDLERS = {
    "block": Emitter.h_block,
    "loop": Emitter.h_loop,
    "if": Emitter.h_if,
    "else": Emitter.h_else,
    "end": Emitter.h_end,
    "br": Emitter.h_br,
    "br_if": Emitter.h_br_if,
    "br_table": Emitter.h_br_table,
    "return": Emitter.h_return,
    "unreachable": Emitter.h_unreachable,
    "local.get": Emitter.h_local_get,
    "local.set": Emitter.h_local_set,
    "local.tee": Emitter.h_local_tee,
    "global.get": Emitter.h_global_get,
    "global.set": Emitter.h_global_set,
    "call": Emitter.h_call,
    "call_indirect": Emitter.h_call_indirect,
    "drop": Emitter.h_drop,
    "select": Emitter.h_select,
    "nop": Emitter.h_nop,
    "i32.const": Emitter.h_i32_const,
    "i64.const": Emitter.h_i64_const,
    "f32.const": Emitter.h_f32_const,
    "f64.const": Emitter.h_f64_const,
    "v128.const": Emitter.h_v128_const,
    "memory.copy": Emitter.h_memory_copy,
    "memory.fill": Emitter.h_memory_fill,
    "memory.init": Emitter.h_memory_init,
    "data.drop": Emitter.h_data_drop,
    "memory.size": Emitter.h_memory_size,
    "memory.grow": Emitter.h_memory_grow,
}
for _op, (_w, _z) in SPLAT_LOADS.items():
    HANDLERS[_op] = Emitter.h_splat_load
for _op in LANE_LOADS:
    HANDLERS[_op] = Emitter.h_lane_load
for _op in LANE_STORES:
    HANDLERS[_op] = Emitter.h_lane_store

Emitter.HANDLERS = HANDLERS
Emitter.type_sigs = {}
