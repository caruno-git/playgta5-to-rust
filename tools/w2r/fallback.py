"""Labelled-state fallback backend.

Used when the structured emitter cannot express a function (multi-value
block/loop parameters, or any construct the structured walk rejects).  Instead
of dropping the function the body becomes a labelled state machine: the wasm
operand stack and control-frame stack become runtime data (`Vec<Val>`,
`Vec<rt::Frame>`), every basic block becomes a `state` number, and a branch is
`rt::br()` + `continue 'm`.

This always works, but it is much less readable than the structured output, so
it is a fallback and its use is counted in the translation report.
"""

import re

try:
    from .emitter import (Emitter, fmt_int, fmt_float, fmt_v128, parse_memargs,
                          ZERO, RUST, NUM, LOADS, STORES, SIMD, ATOMIC,
                          SPLAT_LOADS, LANE_LOADS, LANE_STORES)
    from .tables import rust_name
except ImportError:
    from emitter import (Emitter, fmt_int, fmt_float, fmt_v128, parse_memargs,
                         ZERO, RUST, NUM, LOADS, STORES, SIMD, ATOMIC,
                         SPLAT_LOADS, LANE_LOADS, LANE_STORES)
    from tables import rust_name

VALCTOR = {"i32": "I32", "i64": "I64", "f32": "F32", "f64": "F64", "v128": "V128"}


class State(object):
    """One basic block of the fallback translation."""

    __slots__ = ("uid", "lines")

    def __init__(self, uid):
        self.uid = uid
        self.lines = []


class StateMachine(Emitter):
    def __init__(self, body, idx, gmap, gtypes, funcmap, imports):
        Emitter.__init__(self, body, idx, gmap, gtypes, funcmap, imports)
        self.states = []
        self.cur_state = None
        self.uid = 0
        self.dead = False
        self.dead_nest = 0
        self.frames_s = []
        self._pending_if = None

    # -- state plumbing --------------------------------------------------
    def new_state(self):
        self.uid += 1
        s = State(self.uid)
        self.states.append(s)
        return s

    def goto(self, st):
        self.emit("state = %s; continue 'm;" % self.ref(st))

    @staticmethod
    def ref(st):
        return "@S%d@" % st.uid

    def emit(self, text):
        self.cur_state.lines.append("        " + text)

    # -- dynamic stack ---------------------------------------------------
    def pushv(self, ty, code):
        self.emit("stk.push(Val::%s(%s));" % (VALCTOR[ty], code))

    def pop_as(self, ty):
        t = self.newtemp()
        self.emit("let %s: %s = stk.pop().as_%s();" % (t, RUST[ty], ty))
        return t

    # -- entry -----------------------------------------------------------
    def run(self):
        b = self.b
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
        head = ['pub unsafe extern "C" fn %s(%s)%s {' % (self.fname, ", ".join(args), ret)]
        for i in range(len(b.params), len(self.vars)):
            head.append("    let mut l%d: %s = %s;"
                        % (i, RUST[self.vars[i]], ZERO[self.vars[i]]))
        head.append("    let mut stk: Vec<Val> = Vec::new();")
        head.append("    let mut fs: Vec<rt::Frame> = Vec::new();")
        head.append("    let mut state: u32 = 0;")
        head.append("    'm: loop {")
        head.append("        match state {")

        entry = self.new_state()
        self.cur_state = entry
        self.walk()
        tail_state = self.new_state()
        self.goto(tail_state)

        ids = {}
        for k, s in enumerate(self.states):
            ids[s.uid] = k
        body = []
        for st in self.states:
            body.append("            %d => {" % ids[st.uid])
            body.extend("    " + self._subst(l, ids) for l in st.lines)
            body.append("            }")
        body.append("            _ => break 'm,")
        tail = ["        }", "    }"]
        if b.results:
            tail.append("    rt::ret_%s(&mut stk)" % b.results[0])
        else:
            tail.append("    let _ = &mut stk;")
        return head + body + tail + ["}"]

    @staticmethod
    def _subst(line, ids):
        return re.sub(r"@S(\d+)@", lambda m: str(ids[int(m.group(1))]), line)

    # -- walk ------------------------------------------------------------
    def walk(self):
        instrs = self.b.instrs
        i = 0
        n = len(instrs)
        while i < n:
            op, rest = instrs[i]
            self.instr_count += 1
            if self.dead:
                if op in ("block", "loop", "if"):
                    self.dead_nest += 1
                    self.dead_skipped += 1
                elif op == "end":
                    if self.dead_nest:
                        self.dead_nest -= 1
                        self.dead_skipped += 1
                    else:
                        self.dead = False
                        self.close(op, rest)
                elif op == "else":
                    if self.dead_nest:
                        self.dead_skipped += 1
                    else:
                        self.dead = False
                        self.open_else()
                else:
                    self.dead_skipped += 1
                i += 1
                continue
            try:
                if op == "block":
                    self.open_block(i, rest, loop=False)
                elif op == "loop":
                    self.open_block(i, rest, loop=True)
                elif op == "if":
                    self.open_if(i, rest)
                elif op == "else":
                    self.open_else()
                elif op == "end":
                    self.close(op, rest)
                elif op == "br":
                    self.do_br(int(rest.split()[0]))
                elif op == "br_if":
                    self.do_br_if(int(rest.split()[0]))
                elif op == "br_table":
                    self.do_br_table(rest)
                elif op == "return":
                    self.do_return()
                elif op == "unreachable":
                    self.emit("rt::wasm_unreachable();")
                    self.dead = True
                else:
                    self.dyn_op(op, rest)
            except Exception as exc:  # noqa: BLE001
                self.note_untranslated(op, (op + " " + rest).strip())
                self.emit("// [state-machine] %r" % (exc,))
            i += 1

    # -- control ---------------------------------------------------------
    def open_block(self, i, rest, loop):
        res, params = self._blocktype(rest)
        if params:
            self.note_untranslated("block-params", rest)
        after = self.new_state()
        body = self.new_state()
        kind = 1 if loop else 0
        target = body if loop else after
        self.frames_s.append((kind, target, 0 if loop else len(res)))
        self.emit("fs.push(rt::Frame { kind: %d, base: stk.len(), arity: %d, target: %s });"
                  % (kind, len(res), self.ref(target)))
        self.goto(body)
        self.cur_state = body

    def open_if(self, i, rest):
        res, params = self._blocktype(rest)
        cond = self.pop_as("i32")
        after = self.new_state()
        then_s = self.new_state()
        else_s = self.new_state()
        self.frames_s.append((0, after, len(res)))
        self.emit("fs.push(rt::Frame { kind: 0, base: stk.len(), arity: %d, target: %s });"
                  % (len(res), self.ref(after)))
        self.emit("if %s != 0 { state = %s; } else { state = %s; } continue 'm;"
                  % (cond, self.ref(then_s), self.ref(else_s)))
        self.cur_state = then_s
        self._pending_if = (after, else_s)

    def open_else(self):
        after, else_s = self._pending_if
        self.emit("fs.pop();")
        self.emit("state = %s; continue 'm;" % self.ref(after))
        self.cur_state = else_s

    def close(self, op, rest):
        if not self.frames_s:
            self.dead = True
            return
        kind, target, arity = self.frames_s.pop()
        self.emit("fs.pop();")
        self.emit("state = %s; continue 'm;" % self.ref(target))
        self.cur_state = target
        self.dead = False

    def arity(self, d):
        k = len(self.frames_s) - 1 - d
        if k < 0:
            return len(self.b.results)
        return self.frames_s[k][2]

    def do_br(self, d):
        self.emit("state = rt::br(&mut stk, &mut fs, %d, %d); continue 'm;" % (d, self.arity(d)))
        self.dead = True

    def do_br_if(self, d):
        cond = self.pop_as("i32")
        self.emit("if %s != 0 { state = rt::br(&mut stk, &mut fs, %d, %d); continue 'm; }"
                  % (cond, d, self.arity(d)))

    def do_br_table(self, rest):
        tgts = [int(t) for t in rest.split() if not t.startswith("(")]
        idx = self.pop_as("i32")
        arms = ["%d => rt::br(&mut stk, &mut fs, %d, %d)" % (k, d, self.arity(d))
                for k, d in enumerate(tgts[:-1])]
        arms.append("_ => rt::br(&mut stk, &mut fs, %d, %d)"
                    % (tgts[-1], self.arity(tgts[-1])))
        self.emit("state = match %s { %s }; continue 'm;" % (idx, ", ".join(arms)))
        self.dead = True

    def do_return(self):
        if self.b.results:
            self.emit("return rt::ret_%s(&mut stk);" % self.b.results[0])
        else:
            self.emit("return;")
        self.dead = True

    # -- straight-line ops -----------------------------------------------
    def dyn_op(self, op, rest):
        if op in ("i32.const", "i64.const", "f32.const", "f64.const", "v128.const"):
            if op == "i32.const":
                self.pushv("i32", fmt_int(int(rest, 0), "i32"))
            elif op == "i64.const":
                self.pushv("i64", fmt_int(int(rest, 0), "i64"))
            elif op == "f32.const":
                self.pushv("f32", fmt_float(rest, "f32"))
            elif op == "f64.const":
                self.pushv("f64", fmt_float(rest, "f64"))
            else:
                self.pushv("v128", fmt_v128(rest))
            return
        if op == "local.get":
            k = int(rest)
            self.pushv(self.vars[k], "l%d" % k)
            return
        if op in ("local.set", "local.tee"):
            k = int(rest)
            v = self.pop_as(self.vars[k])
            self.emit("l%d = %s;" % (k, v))
            if op == "local.tee":
                self.pushv(self.vars[k], "l%d" % k)
            return
        if op == "drop":
            self.emit("stk.pop();")
            return
        if op == "select":
            c = self.pop_as("i32")
            b = self.newtemp()
            self.emit("let %s = stk.pop();" % b)
            a = self.newtemp()
            self.emit("let %s = stk.pop();" % a)
            self.emit("stk.push(if %s != 0 { %s } else { %s });" % (c, a, b))
            return
        if op == "global.get":
            g = self._global_ref(rest.strip())
            if g is None:
                self.note_untranslated(op, rest)
                return
            gidx, ty = g
            if gidx == 0:
                self.pushv("i64", "ph.sp")
            else:
                self.pushv(ty, {"i32": "(ph.g[%d] as u32 as i32)" % gidx,
                                "i64": "(ph.g[%d] as i64)" % gidx,
                                "f32": "f32::from_bits(ph.g[%d] as u32)" % gidx,
                                "f64": "f64::from_bits(ph.g[%d])" % gidx}.get(
                                    ty, "(ph.g[%d] as u128)" % gidx))
            return
        if op == "global.set":
            g = self._global_ref(rest.strip())
            if g is None:
                self.note_untranslated(op, rest)
                return
            gidx, ty = g
            v = self.pop_as(ty)
            if gidx == 0:
                self.emit("ph.sp = %s;" % v)
            elif ty == "i32":
                self.emit("ph.g[%d] = (%s as u32) as u64;" % (gidx, v))
            elif ty == "i64":
                self.emit("ph.g[%d] = %s as u64;" % (gidx, v))
            elif ty == "f32":
                self.emit("ph.g[%d] = %s.to_bits() as u64;" % (gidx, v))
            else:
                self.emit("ph.g[%d] = %s.to_bits();" % (gidx, v))
            return
        spec = NUM.get(op)
        if spec is not None:
            argtys, res, tmpl = spec
            vals = [self.pop_as(t) for t in reversed(argtys)][::-1]
            if len(vals) == 1:
                self.pushv(res, tmpl.format(a=vals[0]))
            else:
                self.pushv(res, tmpl.format(a=vals[0], b=vals[1]))
            return
        spec = LOADS.get(op)
        if spec is not None:
            helper, cast, ty = spec
            off, _a, _o = parse_memargs(rest)
            a = self.pop_as("i64")
            e = ("%s + %d" % (a, off)) if off else a
            self.pushv(ty, "ph.%s(%s)%s" % (helper, e, cast))
            return
        spec = STORES.get(op)
        if spec is not None:
            helper, cast = spec
            off, _a, _o = parse_memargs(rest)
            vty = op.split(".")[0]
            v = self.pop_as(vty)
            a = self.pop_as("i64")
            e = ("%s + %d" % (a, off)) if off else a
            self.emit("ph.%s(%s, %s%s);" % (helper, e, v, cast))
            return
        if op == "call":
            tgt = self.funcmap.get(rest.strip().lstrip("$"))
            if tgt is None:
                self.note_untranslated(op, rest)
                return
            tidx, ptypes, rtypes, is_imp, tname = tgt[:5]
            vals = [self.pop_as(t) for t in reversed(ptypes)][::-1]
            if is_imp:
                call = "imp::%s(%s)" % (tname, ", ".join(vals))
            else:
                call = "syms::%s(%s)" % (tname, ", ".join(["ph"] + vals))
            if len(rtypes) == 1:
                self.pushv(rtypes[0], call)
            elif not rtypes:
                self.emit("%s;" % call)
            return
        if op in SPLAT_LOADS:
            width, zero = SPLAT_LOADS[op]
            off, _a, _o = parse_memargs(rest)
            a = self.pop_as("i64")
            e = ("%s + %d" % (a, off)) if off else a
            expression = ("ph.read_u%d(%s) as u128" % (width, e)) if zero else ("ph.read_splat(%s, %d)" % (e, width))
            self.pushv("v128", expression)
            return
        if op in LANE_LOADS:
            width = LANE_LOADS[op]
            off, _a, other = parse_memargs(rest)
            lane = other[-1] if other else "0"
            v = self.pop_as("v128")
            a = self.pop_as("i64")
            e = ("%s + %d" % (a, off)) if off else a
            self.pushv("v128", "ph.load_lane(%s, %s, %s, %d)" % (e, v, lane, width))
            return
        if op in LANE_STORES:
            width = LANE_STORES[op]
            off, _a, other = parse_memargs(rest)
            lane = other[-1] if other else "0"
            v = self.pop_as("v128")
            a = self.pop_as("i64")
            e = ("%s + %d" % (a, off)) if off else a
            self.emit("ph.store_lane(%s, %s, %s, %d);" % (e, v, lane, width))
            return
        spec = SIMD.get(op)
        if spec is not None:
            self.dyn_simd(op, rest, spec)
            return
        spec = ATOMIC.get(op)
        if spec is not None:
            self.dyn_atomic(op, rest, spec)
            return
        if op == "memory.copy":
            n = self.pop_as("i64")
            s = self.pop_as("i64")
            d = self.pop_as("i64")
            self.emit("ph.memory_copy(%s, %s, %s);" % (d, s, n))
            return
        if op == "memory.fill":
            n = self.pop_as("i64")
            v = self.pop_as("i32")
            d = self.pop_as("i64")
            self.emit("ph.memory_fill(%s, %s, %s);" % (d, v, n))
            return
        if op == "memory.size":
            self.pushv("i64", "ph.memory_size()")
            return
        if op == "memory.grow":
            d = self.pop_as("i64")
            self.pushv("i64", "ph.memory_grow(%s)" % d)
            return
        self.note_untranslated(op, (op + " " + rest).strip())

    def dyn_atomic(self, op, rest, spec):
        kind, helper, otys, rty = spec
        off, _a, _o = parse_memargs(rest)
        vals = [self.pop_as(t) for t in reversed(otys)][::-1]
        a = self.pop_as("i64")
        e = ("%s + %d" % (a, off)) if off else a
        if kind == "load":
            self.pushv(rty, "ph.%s(%s)" % (helper, e))
        elif kind == "store":
            self.emit("ph.%s(%s, %s);" % (helper, e, vals[0]))
        elif kind == "rmw":
            self.pushv(rty, "ph.%s(%s, %s)" % (helper, e, vals[0]))
        elif kind == "cmpxchg":
            self.pushv(rty, "ph.%s(%s, %s, %s)" % (helper, e, vals[1], vals[0]))
        elif kind == "notify":
            self.pushv("i32", "ph.%s(%s, %s)" % (helper, e, vals[0]))
        elif kind == "wait":
            self.pushv("i32", "ph.%s(%s, %s, %s)" % (helper, e, vals[1], vals[0]))

    def dyn_simd(self, op, rest, spec):
        kind = spec[0]
        name = "simd::" + rust_name(op)
        if kind in ("b2", "c2"):
            b = self.pop_as("v128")
            a = self.pop_as("v128")
            self.pushv("v128", "%s(%s, %s)" % (name, a, b))
        elif kind == "u1":
            a = self.pop_as("v128")
            self.pushv("v128", "%s(%s)" % (name, a))
        elif kind == "sh":
            s = self.pop_as("i32")
            a = self.pop_as("v128")
            self.pushv("v128", "%s(%s, %s)" % (name, a, s))
        elif kind == "spl":
            a = self.pop_as(spec[1])
            self.pushv("v128", "%s(%s)" % (name, a))
        elif kind == "ex":
            _o, _a, other = parse_memargs(rest)
            lane = other[-1] if other else "0"
            a = self.pop_as("v128")
            ty = {"i8": "i32", "u8": "i32", "i16": "i32", "u16": "i32",
                  "i32": "i32", "u32": "i32", "i64": "i64", "u64": "i64",
                  "f32": "f32", "f64": "f64"}[spec[1]]
            self.pushv(ty, "%s(%s, %s)" % (name, a, lane))
        elif kind == "rep":
            _o, _a, other = parse_memargs(rest)
            lane = other[-1] if other else "0"
            v = self.pop_as(spec[1])
            a = self.pop_as("v128")
            self.pushv("v128", "%s(%s, %s, %s)" % (name, a, lane, v))
        elif kind == "tst":
            a = self.pop_as("v128")
            self.pushv("i32", "%s(%s)" % (name, a))
        elif kind == "raw":
            m = re.search(r"fn\s+\w+\s*\(([^)]*)\)", spec[1])
            params = [p for p in m.group(1).split(",") if p.strip()] if m else []
            tys = [p.split(":")[-1].strip() for p in params]
            vals = [self.pop_as("v128" if t == "V" else "i32") for t in reversed(tys)][::-1]
            self.pushv("v128" if "-> V" in spec[1] else "i32",
                       "%s(%s)" % (name, ", ".join(vals)))
        else:
            self.note_untranslated(op, (op + " " + rest).strip())
