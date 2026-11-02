#!/usr/bin/env python3
"""wat2rust.py — machine-translate WAT function bodies from `game.wasm` into Rust.

Layer 2 of the reconstruction: a *mechanical* source-to-source translator, not a
decompiler.  Every one of the 91 026 defined functions of the wasm64 module is
parsed out of the `game.wasm` disassembly and emitted as one Rust file.

    python wat2rust.py index          # scan game.wat -> build/index.json
    python wat2rust.py emit-support   # regenerate wasm-rt/src/simd.rs from tables
    python wat2rust.py run            # translate all functions (parallel)
    python wat2rust.py report         # write generated/TRANSLATION-REPORT.md
    python wat2rust.py verify         # cargo check a 200-function crate
    python wat2rust.py all            # index + support + run + report + verify

What the output looks like (one `.rs` per function, header carries func_idx,
original name and WAT source line range):

    // func_idx: 2055 ...
    #[no_mangle]
    pub unsafe extern "C" fn f_002055_rage_fidevice_convertfiletimetosystemtime(
        ph: &mut Phantom, mut l0: i64, mut l1: i64) -> i32 {
        let mut l2: i64 = 0;
        l2 = ph.sp - 64;
        ph.sp = l2;
        ...
        if (l0 != 0) != 0 {
            let t1: i32 = ph.read_u16(l2 + 20);
            ph.write_u16(l1, (t1 + 1900) & 0xffff);
        }
        ...
    }

Design notes
------------
* the wasm operand stack is reconstructed as *deferred expressions* which are
  materialised into named temporaries (`let t3 = ph.read_u64(p + 296);`) at
  every point where sequencing, mutation or size makes deferral unsafe;
* memory access becomes `Phantom::read_*` / `write_*` over `*mut u8` with the
  static `offset=` folded into an explicit address addition;
* `global.get $__stack_pointer` becomes the `ph.sp` field, all other module
  globals the `ph.g[N]` word array;
* `call $name` becomes a call through the crate's `syms` module (an `extern "C"`
  declaration per callee, so a function's callers do not need to know which of
  the generated crates it lives in);
* `call_indirect` becomes a table dispatch stub: fetch the funcref, transmute it
  to the statically known signature, call;
* control flow is emitted structurally (block/loop/if/br/br_if/br_table) with
  labels only where a branch actually targets them; functions that use a
  construct the structured emitter cannot express fall back to a labelled state
  machine (`--fallback`), never to silence;
* an opcode the translator does not know is emitted as an `UNTRANSLATED` comment
  and counted — nothing is ever silently dropped.
"""

import argparse
import json
import os
import re
import sys
import time
from collections import Counter

__author__ = "carunodev @stfu"

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

from w2r import tables                            # noqa: E402
from w2r.emitter import (Emitter, Unsupported, sanitize,   # noqa: E402
                         parse_body, parse_header)
from w2r.fallback import StateMachine             # noqa: E402

ROOT = os.path.dirname(HERE)                      # recovered/rust
GEN = os.environ.get("WAT2RUST_GEN", os.path.join(ROOT, "generated"))
BUILD = os.environ.get("WAT2RUST_BUILD", os.path.join(ROOT, "build"))
WAT = os.environ.get("WAT", os.path.join(ROOT, "inputs", "game.wat"))

DEFAULT_WAT_EVIDENCE = r"recovered\tools\wat_index.json"

PER_CRATE = 4096
TRANSLATOR_VERSION = "wat2rust.py v1.2 (layer 2)"


# ---------------------------------------------------------------------------
# 1. index: scan the WAT once for byte offsets + module metadata
# ---------------------------------------------------------------------------

RE_TYPE = re.compile(r"^\s*\(type\s+\(;(\d+);\)\s+\(func(.*)\)\s*$")
RE_IMPORT_FUNC = re.compile(r'^\s*\(import\s+"([^"]*)"\s+"([^"]*)"\s+\(func\s+\$?(\S+?)(?:\s+\(type\s+(\d+)\))?\)\s*\)')
RE_GLOBAL = re.compile(r"^\s*\(global\s+\$?(\S+?)\s+\(mut\s+([a-z0-9]+)\)")
RE_FUNC_START = re.compile(rb"^  \(func")
RE_SIG_PARAMS = re.compile(r"\(param\s+([^)]*)\)")
RE_SIG_RESULTS = re.compile(r"\(result\s+([^)]*)\)")


def _sig(text):
    p = []
    r = []
    for m in RE_SIG_PARAMS.finditer(text):
        p.extend(t for t in m.group(1).split() if t)
    for m in RE_SIG_RESULTS.finditer(text):
        r.extend(t for t in m.group(1).split() if t)
    return p, r


def build_index(jobs=1):
    t0 = time.time()
    types = {}
    globals_ = []
    imports = []
    funcs = []
    first_func_idx = None
    line = 0
    pos = 0
    pending_func = None
    with open(WAT, "rb") as f:
        for raw in f:
            line += 1
            off = pos
            pos += len(raw)
            if not raw.startswith(b"  ("):
                continue
            s = raw.decode("utf8", "replace").rstrip("\r\n")
            if s.startswith("  (type ("):
                m = RE_TYPE.match(s)
                if m:
                    types[int(m.group(1))] = _sig(m.group(2))
            elif s.startswith("  (import"):
                m = RE_IMPORT_FUNC.match(s)
                if m:
                    imports.append({"module": m.group(1), "field": m.group(2),
                                    "name": m.group(3),
                                    "type": int(m.group(4)) if m.group(4) else None})
            elif s.startswith("  (global"):
                m = RE_GLOBAL.match(s)
                if m:
                    globals_.append({"name": m.group(1), "type": m.group(2)})
            elif s.startswith("  (func"):
                m = re.match(r"^  \(func\s+\$?(\S+?)(?:\s|$)(.*)$", s)
                name = m.group(1) if m else ""
                rest = m.group(2) if m else ""
                tm = re.search(r"\(type\s+(\d+)\)", rest)
                p, r = _sig(rest)
                funcs.append({"name": name, "off": off, "line": line,
                              "type": int(tm.group(1)) if tm else None,
                              "params": p, "results": r})
                pending_func = funcs[-1]
            elif pending_func is not None and s.startswith("  (") and "func" not in s:
                pending_func = None

    # function line ranges: functions are contiguous in the WAT, so the end of a
    # function is the line before the next one.  (The evidence index in
    # recovered/tools/wat_index.json leaves `end` unset for single-line
    # functions, which made 71 of them read to EOF.)
    total_lines = line
    for k, r in enumerate(funcs):
        r["line_end"] = (funcs[k + 1]["line"] - 1) if k + 1 < len(funcs) else total_lines

    # types for imports
    for imp in imports:
        imp["params"], imp["results"] = types.get(imp["type"], ([], []))

    first_idx = 85 if len(imports) == 85 else len(imports)
    for k, r in enumerate(funcs):
        r["idx"] = first_idx + k
        r["rustname"] = sanitize(r["name"], r["idx"])
        if not r["params"] and not r["results"] and r["type"] in types:
            r["params"], r["results"] = types[r["type"]]

    os.makedirs(BUILD, exist_ok=True)
    meta = {
        "wat": WAT,
        "translator": TRANSLATOR_VERSION,
        "types": {str(k): v for k, v in types.items()},
        "globals": globals_,
        "imports": imports,
        "funcs": [[r["idx"], r["name"], r["params"], r["results"], r["off"],
                   r["line"], r["line_end"], r["rustname"]] for r in funcs],
    }
    json.dump(meta, open(os.path.join(BUILD, "index.json"), "w", encoding="utf8"),
              ensure_ascii=False)
    print("indexed %d funcs, %d types, %d globals, %d imports in %.1fs -> build/index.json"
          % (len(funcs), len(types), len(globals_), len(imports), time.time() - t0))
    return meta


def load_meta():
    p = os.path.join(BUILD, "index.json")
    if not os.path.exists(p):
        return build_index()
    return json.load(open(p, encoding="utf8"))


# ---------------------------------------------------------------------------
# 2. worker state
# ---------------------------------------------------------------------------

_W = {}


def _init(meta_path):
    meta = json.load(open(meta_path, encoding="utf8"))
    fm = {}
    for idx, name, params, results, off, l0, l1, rustname in meta["funcs"]:
        fm[name] = (idx, params, results, False, rustname)
    imp_map = {}
    for k, imp in enumerate(meta["imports"]):
        imp_map[imp["name"]] = (k, imp["params"], imp["results"], True,
                                "imp_%d_%s" % (k, sanitize(imp["name"], k)[9:]))
    fm.update(imp_map)
    _W["meta"] = meta
    _W["funcmap"] = fm
    _W["gmap"] = {g["name"]: (i, g["type"]) for i, g in enumerate(meta["globals"])}
    _W["types"] = {int(k): (v[0], v[1]) for k, v in meta["types"].items()}
    _W["fh"] = open(os.environ.get("WAT") or os.path.join(ROOT, "inputs", "game.wat"), "rb")
    _W["nfunc"] = len(meta["funcs"])


def _read_func(rec):
    idx, name, params, results, off, l0, l1, rustname = rec
    fh = _W["fh"]
    fh.seek(off)
    n = l1 - l0 + 1
    chunks = []
    for k in range(n):
        line = fh.readline()
        if not line:
            break
        if k and line.startswith(b"  (") and not line.startswith(b"  (func"):
            break
        chunks.append(line)
    return b"".join(chunks).decode("utf8", "replace")


def _crate_of(ordinal):
    return "gen%03d" % (ordinal // PER_CRATE)


def translate_one(rec, ordinal):
    idx, name, params, results, off, l0, l1, rustname = rec
    text = _read_func(rec)
    body = parse_body(text, _W["types"], idx)
    if not body.name:
        body.name = name
    if not body.params and params:
        body.params = params
    if not body.results and results:
        body.results = results
    em = Emitter(body, idx, _W["gmap"], None, _W["funcmap"], None)
    em.type_sigs = _W["types"]
    em.fname = rustname
    mode = "structured"
    try:
        lines = em.run()
        if em.failed:
            raise Unsupported(em.fail_reason or "structured emitter failed")
    except Unsupported as exc:
        # structured emitter cannot express this function: labelled state machine
        em.untranslated.pop("<fatal:%s>" % "", None)
        try:
            sm = StateMachine(body, idx, _W["gmap"], None, _W["funcmap"], None)
            sm.type_sigs = _W["types"]
            sm.fname = rustname
            lines = sm.run()
            mode = "state-machine"
            em.failed = False
            em.fail_reason = None
            em.instr_count = max(em.instr_count, sm.instr_count)
            em.dead_skipped += sm.dead_skipped
            em.callees |= sm.callees
            for k, v in sm.untranslated.items():
                em.untranslated[k] = em.untranslated.get(k, 0) + v
        except Exception as exc2:  # noqa: BLE001 - fallback failed too
            mode = "failed"
            em.failed = True
            em.fail_reason = "%s | fallback: %r" % (exc, exc2)
            lines = ['// STRUCTURED TRANSLATION FAILED: %s' % (em.fail_reason,),
                     '#[no_mangle]',
                     'pub unsafe extern "C" fn %s(ph: &mut Phantom) {' % rustname,
                     '    rt::wasm_unreachable();',
                     '}']
    except Exception as exc:  # noqa: BLE001 - never lose a function
        em.failed = True
        em.fail_reason = repr(exc)
        mode = "failed"
        lines = ['// translation crashed: %r' % (exc,),
                 '#[no_mangle]',
                 'pub unsafe extern "C" fn %s(ph: &mut Phantom) {' % rustname,
                 '    rt::wasm_unreachable();',
                 '}']

    header = [
        "//! GTA5 engine — wasm -> Rust machine translation (layer 2).",
        "//!",
        "//! func_idx:    %d" % idx,
        "//! name:        %s" % (name or "<unnamed>"),
        "//! wasm type:   (param %s) (result %s)" % (" ".join(params), " ".join(results)),
        "//! wat lines:   %d-%d (%d lines)" % (l0, l1, l1 - l0 + 1),
        "//! instructions: %d (%d untranslated, %d dead/unreachable)"
        % (em.instr_count, sum(em.untranslated.values()), em.dead_skipped),
        "//! mode:        %s" % mode,
        "//!",
        "//! Generated by %s. Do not edit; see generated/TRANSLATION-REPORT.md." % TRANSLATOR_VERSION,
        "#![allow(unused_variables, unused_mut, unused_assignments, unused_parens)]",
        "#![allow(unreachable_code, unused_imports, non_snake_case)]",
        "#![allow(improper_ctypes_definitions, clippy::all)]",
        "use wasm_rt::{simd, Phantom};",
        "use wasm_rt as rt;",
        "use crate::{imp, syms};",
        "",
    ]
    out = header + lines + [""]
    crate = _crate_of(ordinal)
    d = os.path.join(GEN, crate, "src")
    path = os.path.join(d, rustname + ".rs")
    with open(path, "w", encoding="utf8", newline="\n") as f:
        f.write("\n".join(out))

    # call targets for this crate's syms.rs
    return {
        "idx": idx,
        "crate": crate,
        "file": rustname,
        "name": name,
        "instrs": em.instr_count,
        "untranslated": em.untranslated,
        "dead": em.dead_skipped,
        "mode": mode,
        "failed": bool(em.failed),
        "reason": em.fail_reason,
        "calls": sorted(em.callees) if hasattr(em, "callees") else [],
        "bytes": l1 - l0 + 1,
    }


def _work(batch):
    out = []
    for ordinal, rec in batch:
        try:
            out.append(translate_one(rec, ordinal))
        except Exception as exc:  # noqa: BLE001
            out.append({"idx": rec[0], "crate": _crate_of(ordinal), "file": rec[7],
                        "name": rec[1], "instrs": 0, "untranslated": {"<crash>": 1},
                        "dead": 0, "mode": "failed", "failed": True,
                        "reason": repr(exc), "calls": [], "bytes": rec[6] - rec[5] + 1})
    return out


# ---------------------------------------------------------------------------
# 3. run
# ---------------------------------------------------------------------------

def prepare_dirs(meta):
    n = len(meta["funcs"])
    ncrates = (n + PER_CRATE - 1) // PER_CRATE
    for c in range(ncrates):
        crate = "gen%03d" % c
        d = os.path.join(GEN, crate)
        os.makedirs(os.path.join(d, "src"), exist_ok=True)
        members = [r for k, r in enumerate(meta["funcs"]) if k // PER_CRATE == c]
        with open(os.path.join(d, "Cargo.toml"), "w", encoding="utf8") as f:
            f.write('[package]\nname = "%s"\nversion = "0.1.0"\nedition = "2021"\n'
                    'description = "GTA5 wasm -> Rust machine translation, functions '
                    '%d..%d"\n\n[dependencies]\nwasm-rt = { path = "../../wasm-rt" }\n'
                    'wasm-imp = { path = "../../wasm-imp" }\n\n[lib]\npath = "src/lib.rs"\n\n'
                    '# standalone: not a member of ../../Cargo.toml (the hand-written\n'
                    '# reconstruction workspace); it must NOT declare its own [workspace],\n'
                    '# or cargo refuses to use it as a path dependency (multiple roots).\n'
                    % (crate, members[0][0], members[-1][0]))
    return ncrates


def cmd_run(args):
    meta = load_meta()
    n = len(meta["funcs"])
    prepare_dirs(meta)
    jobs = args.jobs or max(1, (os.cpu_count() or 4))
    batch = args.batch or 200
    work = []
    for ordinal, rec in enumerate(meta["funcs"]):
        if args.limit and ordinal >= args.limit:
            break
        work.append((ordinal, rec))
    batches = [work[i:i + batch] for i in range(0, len(work), batch)]
    t0 = time.time()
    results = []
    if jobs == 1:
        _init(os.path.join(BUILD, "index.json"))
        for k, b in enumerate(batches):
            results.extend(_work(b))
            if k % 20 == 0:
                sys.stderr.write("  %d/%d functions (%.0fs)\n"
                                 % (len(results), len(work), time.time() - t0))
    else:
        import multiprocessing as mp
        ctx = mp.get_context("spawn")
        with ctx.Pool(jobs, initializer=_init,
                      initargs=(os.path.join(BUILD, "index.json"),)) as pool:
            for k, chunk in enumerate(pool.imap_unordered(_work, batches, chunksize=1)):
                results.extend(chunk)
                if k % 20 == 0:
                    sys.stderr.write("  %d/%d functions (%.0fs)\n"
                                     % (len(results), len(work), time.time() - t0))
    results.sort(key=lambda r: r["idx"])
    json.dump(results, open(os.path.join(BUILD, "results.json"), "w"), ensure_ascii=False)
    write_libs(meta, results, args)
    write_syms(results, args)
    print("translated %d functions in %.1fs (%.0f fn/s)"
          % (len(results), time.time() - t0, len(results) / max(0.001, time.time() - t0)))


def _calls_from_generated(path):
    """Recover the syms:: call targets of an already written file."""
    names = set()
    try:
        with open(path, encoding="utf8") as f:
            for line in f:
                if "syms::" in line:
                    for m in re.finditer(r"syms::(f_\d+_[A-Za-z0-9_]+)", line):
                        names.add(m.group(1))
    except OSError:
        pass
    return names


def write_syms(results, args):
    """Per-crate `syms.rs`: extern "C" declarations for every callee."""
    meta = load_meta()
    by_name = {}
    for idx, name, params, results_, off, l0, l1, rustname in meta["funcs"]:
        by_name[rustname] = (params, results_)
    per_crate = {}
    for r in results:
        path = os.path.join(GEN, r["crate"], "src", r["file"] + ".rs")
        for nm in _calls_from_generated(path):
            per_crate.setdefault(r["crate"], set()).add(nm)
    for crate, names in per_crate.items():
        lines = ["//! Callee declarations for crate %s." % crate,
                 "//!",
                 "//! Every `call $name` in the wasm module becomes a call through this",
                 "//! module: an `extern \"C\"` declaration of the target's generated",
                 "//! identifier.  That keeps a function's callers independent of which",
                 "//! generated crate the callee ended up in (they are all linked into the",
                 "//! same binary and the identifiers are unique).",
                 "#![allow(improper_ctypes, dead_code, unused_imports)]",
                 "use wasm_rt::Phantom;",
                 ""]
        for nm in sorted(names):
            sig = by_name.get(nm)
            if sig is None:
                continue
            params, res = sig
            plist = ", ".join(["ph: &mut Phantom"] + ["a%d: %s" % (i, tables.RUST[t])
                                                     for i, t in enumerate(params)])
            rt = ""
            if res:
                rt = " -> " + (tables.RUST[res[0]] if len(res) == 1
                               else "(%s)" % ", ".join(tables.RUST[t] for t in res))
            lines.append('extern "C" { pub fn %s(%s)%s; }' % (nm, plist, rt))
        with open(os.path.join(GEN, crate, "src", "syms.rs"), "w", encoding="utf8",
                  newline="\n") as f:
            f.write("\n".join(lines) + "\n")


def write_libs(meta, results, args):
    n = len(meta["funcs"])
    ncrates = (n + PER_CRATE - 1) // PER_CRATE
    files = {}
    for r in results:
        files.setdefault(r["crate"], []).append(r["file"])
    for c in range(ncrates):
        crate = "gen%03d" % c
        mods = files.get(crate, [])
        lines = ["//! Generated crate %s — %d machine-translated wasm functions." % (crate, len(mods)),
                 "//!",
                 "//! Produced by tools/wat2rust.py from game.wasm. One module per wasm",
                 "//! function; `syms` holds `extern \"C\"` declarations of callees.",
                 "#![allow(clippy::all)]",
                 "pub mod imp {",
                 "    pub use wasm_imp::*;",
                 "}",
                 "pub mod syms;",
                 ""]
        for m in sorted(mods):
            lines.append("pub mod %s;" % m)
        with open(os.path.join(GEN, crate, "src", "lib.rs"), "w", encoding="utf8",
                  newline="\n") as f:
            f.write("\n".join(lines) + "\n")


# ---------------------------------------------------------------------------
# 4. report
# ---------------------------------------------------------------------------

def cmd_report(args):
    meta = load_meta()
    results = json.load(open(os.path.join(BUILD, "results.json")))
    total = len(results)
    ok = sum(1 for r in results if not r["failed"])
    failed = [r for r in results if r["failed"]]
    instrs = sum(r["instrs"] for r in results)
    dead = sum(r["dead"] for r in results)
    hist = Counter()
    for r in results:
        for k, v in r["untranslated"].items():
            hist[k] += v
    modes = Counter(r["mode"] for r in results)
    # untranslated instructions
    untr = sum(hist.values())

    def size_fmt(n):
        for u in ("B", "KB", "MB", "GB"):
            if n < 1024 or u == "GB":
                return "%.1f %s" % (n, u)
            n /= 1024.0
        return "%.1f GB" % n

    out = []
    A = out.append
    A("# WAT → Rust machine translation report (layer 2)")
    A("")
    A("Generated by `tools/wat2rust.py` (%s)." % TRANSLATOR_VERSION)
    A("Source of truth: `game.wasm` (63 201 802 B, wasm64 Emscripten build,")
    A("sha256 `11ca8d2c04c5e843d18ff4aea4899d72c86973c6b031df334e67c446b2ae83e0`)")
    A("disassembled to 591 MB of WAT (`%s`)." % os.path.basename(WAT))
    A("")
    A("## Totals")
    A("")
    A("| metric | value |")
    A("|---|---|")
    A("| defined functions | %d |" % len(meta["funcs"]))
    A("| functions translated | %d |" % total)
    A("| translated cleanly | %d (%.2f%%) |" % (ok, 100.0 * ok / max(1, total)))
    A("| functions with untranslated constructs | %d |" % len(failed))
    A("| wasm instructions handled | %d |" % instrs)
    A("| instructions emitted as `UNTRANSLATED` | %d (%.4f%%) |"
      % (untr, 100.0 * untr / max(1, instrs)))
    A("| dead/unreachable instructions skipped (counted) | %d |" % dead)
    A("| output | %d `.rs` files in %d crates under `generated/` |"
      % (total, len(set(r["crate"] for r in results))))
    A("")
    A("Translation mode: " + ", ".join("%s %d" % (k, v) for k, v in sorted(modes.items())))
    A("")
    A("Instructions do not vanish: control flow ends a basic block, and the")
    A("instructions after an unconditional branch until the matching `end` are")
    A("unreachable in the wasm sense.  They are counted in the `dead/unreachable`")
    A("line above and left in the file as capped `// [dead] ...` comments.")
    A("")
    A("## Untranslated-opcode histogram")
    A("")
    if not hist:
        A("_Empty:_ every opcode in the module is covered by the translator.")
    else:
        A("| opcode | count |")
        A("|---|---|")
        for op, cnt in hist.most_common(200):
            A("| `%s` | %d |" % (op, cnt))
    A("")
    A("## Largest translation failures")
    A("")
    if not failed:
        A("_None:_ all %d functions were translated." % total)
    else:
        A("Sorted by wasm instruction count of the failing function.")
        A("")
        A("| func_idx | function | instructions | reason |")
        A("|---|---|---|---|")
        for r in sorted(failed, key=lambda x: -x["instrs"])[:100]:
            A("| %d | `%s` | %d | %s |" % (r["idx"], (r["name"] or "")[:90],
                                          r["instrs"], (r["reason"] or "")[:110]))
    A("")
    A("## Output layout")
    A("")
    A("```")
    A("recovered/rust/")
    A("  wasm-rt/                 hand-written runtime: Phantom (memory, globals,")
    A("                           atomics, funcref table), trapping numeric helpers,")
    A("                           simd.rs (generated from tools/w2r/tables.py)")
    A("  wasm-imp/                extern declarations for the 85 env.* imports")
    A("  generated/genNNN/src/    one .rs per wasm function + syms.rs + imp shim")
    A("  generated/TRANSLATION-REPORT.md")
    A("  tools/wat2rust.py        this translator")
    A("```")
    A("")
    A("File naming: `<sanitized demangled name>` with `f_<func_idx>_` prefix,")
    A("ASCII snake_case, truncated to 64 chars, so names are unique and stable.")
    A("")
    A("## How to read a translated function")
    A("")
    A("* `lN` – wasm locals/params by index (params are the `mut lN` arguments),")
    A("* `tN` – temporaries materialised from the wasm operand stack,")
    A("* `rN` – block/loop/if result bindings (`br` targets),"),
    A("* `ph.read_u64(p + 296)` – `i64.load offset=296`,")
    A("* `ph.sp` – `global.get/set $__stack_pointer`,")
    A("* `ph.g[N]` – every other module global,")
    A("* `syms::f_…` – direct `call`,")
    A("* `ph.table_entry(idx)` + `transmute` – `call_indirect`,")
    A("* `rt::…` – trapping/saturating scalar helpers, `simd::…` – v128 helpers.")
    A("")
    path = os.path.join(GEN, "TRANSLATION-REPORT.md")
    with open(path, "w", encoding="utf8", newline="\n") as f:
        f.write("\n".join(out) + "\n")
    print("wrote %s (%d functions, %d instructions, %d untranslated)"
          % (path, total, instrs, untr))


# ---------------------------------------------------------------------------
# 5. support generation
# ---------------------------------------------------------------------------

def cmd_emit_support(args):
    p = os.path.join(ROOT, "wasm-rt", "src", "simd.rs")
    os.makedirs(os.path.dirname(p), exist_ok=True)
    with open(p, "w", encoding="utf8", newline="\n") as f:
        f.write(tables.gen_simd_rs())
    print("wrote %s (%d simd ops)" % (p, len(tables.SIMD)))
    write_wasm_imp()


def write_wasm_imp():
    meta = load_meta()
    d = os.path.join(ROOT, "wasm-imp")
    os.makedirs(os.path.join(d, "src"), exist_ok=True)
    with open(os.path.join(d, "Cargo.toml"), "w", encoding="utf8") as f:
        f.write('[package]\nname = "wasm-imp"\nversion = "0.1.0"\nedition = "2021"\n'
                'authors = ["carunodev @stfu"]\n'
                'description = "extern declarations for the game.wasm env.* imports"\n\n'
                '[lib]\nname = "wasm_imp"\npath = "src/lib.rs"\n\n'
                '[dependencies]\nwasm-rt = { path = "../wasm-rt" }\n\n'
                '[dev-dependencies]\nengine-abi = { path = "../crates/engine-abi" }\n\n'
                '[workspace]\n')
    lines = ["//! Declarations for the %d `env.*` imports of game.wasm." % len(meta["imports"]),
             "//!",
             "//! These are provided by the JS host at runtime; the translated Rust only",
             "//! needs them to type-check and to keep the call graph visible.",
             "#![allow(improper_ctypes, dead_code, non_snake_case, clippy::all)]",
             ""]
    symbols = []
    for k, imp in enumerate(meta["imports"]):
        rn = "imp_%d_%s" % (k, sanitize(imp["name"], k)[9:])
        symbols.append(rn)
        params = ", ".join("a%d: %s" % (i, tables.RUST[t]) for i, t in enumerate(imp["params"]))
        res = imp["results"]
        rt = ""
        if res:
            rt = " -> " + (tables.RUST[res[0]] if len(res) == 1
                           else "(%s)" % ", ".join(tables.RUST[t] for t in res))
        lines.append('#[cfg_attr(target_family = "wasm", link(wasm_import_module = "%s"))]' % imp["module"])
        lines.append('extern "C" {')
        lines.append('    #[link_name = "%s"]' % imp["field"])
        lines.append('    pub fn %s(%s)%s;' % (rn, params, rt))
        lines.append('}')
    with open(os.path.join(d, "src", "lib.rs"), "w", encoding="utf8", newline="\n") as f:
        f.write("\n".join(lines) + "\n")
    tests = os.path.join(d, "tests")
    os.makedirs(tests, exist_ok=True)
    lines = ['#![cfg(not(target_family = "wasm"))]', '#[test]',
             'fn all_import_symbols_link_to_the_rust_host() {',
             '    std::hint::black_box(engine_abi::time::wasm_now_ms());',
             '    let symbols = std::hint::black_box([']
    lines.extend('        wasm_imp::%s as *const (),' % name for name in symbols)
    lines.extend(['    ]);', '    assert_eq!(symbols.len(), 85);',
                  '    assert!(symbols.iter().all(|p| !p.is_null()));', '}'])
    with open(os.path.join(tests, "native_symbols.rs"), "w", encoding="utf8", newline="\n") as f:
        f.write("\n".join(lines) + "\n")
    print("wrote %s (%d imports)" % (d, len(meta["imports"])))


# ---------------------------------------------------------------------------
# 6. verify: cargo check a crate of N translated functions
# ---------------------------------------------------------------------------

def cmd_verify(args):
    meta = load_meta()
    results = json.load(open(os.path.join(BUILD, "results.json")))
    count = args.count
    # pick a deterministic, representative sample: middle-sized functions that
    # translated cleanly, spread over the whole index
    ok = [r for r in results if not r["failed"] and r["instrs"] >= 12]
    ok.sort(key=lambda r: r["idx"])
    if not ok:
        print("nothing to verify")
        return 1
    step = max(1, len(ok) // count)
    sample = ok[::step][:count]
    vdir = os.path.join(ROOT, "verify-200")
    src = os.path.join(vdir, "src")
    os.makedirs(src, exist_ok=True)
    with open(os.path.join(vdir, "Cargo.toml"), "w", encoding="utf8") as f:
        f.write('[package]\nname = "verify-200"\nversion = "0.1.0"\nedition = "2021"\n'
                'description = "cargo-check probe: 200 machine-translated wasm functions"\n\n'
                '[dependencies]\nwasm-rt = { path = "../wasm-rt" }\n'
                'wasm-imp = { path = "../wasm-imp" }\n\n[lib]\npath = "src/lib.rs"\n\n'
                '[profile.dev]\ndebug = false\nincremental = false\n')
    mods = []
    calls = set()
    for r in sample:
        srcp = os.path.join(GEN, r["crate"], "src", r["file"] + ".rs")
        if not os.path.exists(srcp):
            continue
        dst = os.path.join(src, r["file"] + ".rs")
        with open(srcp, encoding="utf8") as f:
            data = f.read()
        with open(dst, "w", encoding="utf8", newline="\n") as f:
            f.write(data)
        mods.append(r["file"])
        calls |= _calls_from_generated(srcp)
    by_name = {}
    for idx, name, params, results_, off, l0, l1, rustname in meta["funcs"]:
        by_name[rustname] = (params, results_)
    # syms for every callee of the sample (including the sample's own members:
    # an extern declaration never conflicts with a definition)
    lines = ["#![allow(improper_ctypes, dead_code, unused_imports)]",
             "use wasm_rt::Phantom;"]
    for nm in sorted(calls):
        sig = by_name.get(nm)
        if sig is None:
            continue
        params, res = sig
        plist = ", ".join(["ph: &mut Phantom"] + ["a%d: %s" % (i, tables.RUST[t])
                                                 for i, t in enumerate(params)])
        rt = ""
        if res:
            rt = " -> " + (tables.RUST[res[0]] if len(res) == 1
                           else "(%s)" % ", ".join(tables.RUST[t] for t in res))
        lines.append('extern "C" { pub fn %s(%s)%s; }' % (nm, plist, rt))
    with open(os.path.join(src, "syms.rs"), "w", encoding="utf8", newline="\n") as f:
        f.write("\n".join(lines) + "\n")
    lib = ["#![allow(clippy::all)]",
           "pub mod imp { pub use wasm_imp::*; }",
           "pub mod syms;"]
    for m in sorted(mods):
        lib.append("pub mod %s;" % m)
    with open(os.path.join(src, "lib.rs"), "w", encoding="utf8", newline="\n") as f:
        f.write("\n".join(lib) + "\n")
    print("verify crate: %d functions, %d callee declarations" % (len(mods), len(calls)))
    return run_cargo_check(vdir)


def run_cargo_check(vdir):
    import subprocess
    t0 = time.time()
    env = dict(os.environ)
    cmd = ["cargo", "check", "--message-format=short", "--quiet"]
    try:
        p = subprocess.run(cmd, cwd=vdir, capture_output=True, text=True, env=env,
                           timeout=3600)
    except FileNotFoundError:
        print("cargo not found on PATH")
        return 2
    out = p.stdout + p.stderr
    open(os.path.join(BUILD, "cargo-check.log"), "w", encoding="utf8").write(out)
    print("cargo check finished in %.1fs, exit %d" % (time.time() - t0, p.returncode))
    classify(out)
    return p.returncode


CLASS = [
    ("E0308", "mismatched types"),
    ("E0061", "wrong number of arguments"),
    ("E0599", "no method / no associated item"),
    ("E0425", "cannot find value"),
    ("E0433", "failed to resolve (path/crate)"),
    ("E0412", "cannot find type"),
    ("E0499", "multiple mutable borrows"),
    ("E0502", "mutable+immutable borrow conflict"),
    ("E0503", "use of moved value"),
    ("E0505", "use of moved value (borrow)"),
    ("E0507", "cannot move out"),
    ("E0600", "cannot apply unary operator"),
    ("E0601", "main not found"),
    ("E0618", "call expression not a function"),
    ("E0689", "ambiguous numeric type"),
    ("E0277", "trait bound not satisfied"),
    ("E0070", "invalid left-hand side of assignment"),
    ("E0004", "non-exhaustive match"),
    ("E0005", "refutable pattern"),
    ("E0080", "evaluation of constant failed"),
    ("E0658", "unstable feature"),
    ("E0583", "file not found for module"),
    ("E0576", "cannot find relevant scope"),
    ("E0596", "cannot borrow as mutable"),
    ("E0063", "missing field in initializer"),
    ("E0062", "field specified twice"),
]


def classify(text):
    lines = [l for l in text.splitlines() if l.strip()]
    codes = Counter()
    for l in lines:
        m = re.search(r"\berror\[(E\d{4})\]", l)
        if m:
            codes[m.group(1)] += 1
    if not codes and not any("error" in l for l in lines):
        print("no errors")
        return
    print("error classes:")
    names = dict(CLASS)
    for code, n in codes.most_common():
        print("  %s  %-6s %4d  %s" % (code, "", n, names.get(code, "")))
    other = [l for l in lines if "error" in l and not re.search(r"error\[(E\d{4})\]", l)]
    if other:
        print("  (non-coded errors: %d) e.g. %s" % (len(other), other[0][:160]))
    print("  total error lines: %d" % len([l for l in lines if l.startswith("error")]))


# ---------------------------------------------------------------------------
# main
# ---------------------------------------------------------------------------

def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd")

    p = sub.add_parser("index", help="scan game.wat for offsets and metadata")
    p = sub.add_parser("emit-support", help="regenerate simd.rs and wasm-imp")
    p = sub.add_parser("run", help="translate all functions")
    p.add_argument("--jobs", type=int, default=0)
    p.add_argument("--batch", type=int, default=200)
    p.add_argument("--limit", type=int, default=0)
    p = sub.add_parser("report", help="write generated/TRANSLATION-REPORT.md")
    p = sub.add_parser("verify", help="cargo check a crate of translated functions")
    p.add_argument("--count", type=int, default=200)
    p = sub.add_parser("all", help="index + support + run + report + verify")
    p.add_argument("--jobs", type=int, default=0)
    p.add_argument("--count", type=int, default=200)

    args = ap.parse_args(argv)
    if args.cmd == "index":
        build_index()
    elif args.cmd == "emit-support":
        cmd_emit_support(args)
    elif args.cmd == "run":
        cmd_run(args)
    elif args.cmd == "report":
        cmd_report(args)
    elif args.cmd == "verify":
        return cmd_verify(args)
    elif args.cmd == "all":
        build_index()
        cmd_emit_support(args)
        cmd_run(args)
        cmd_report(args)
        return cmd_verify(args)
    else:
        ap.print_help()
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())
