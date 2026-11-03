import json
import pathlib
import re
import shutil
import sys
from shader_pass_fixes import apply_shader_pass_fixes
from task_upload_fixes import apply_task_upload_fixes
from minimap_mode_fixes import apply_minimap_mode_fixes, apply_minimap_mode_symbols
from resize_binding_fixes import apply_resize_binding_fixes, apply_resize_binding_symbols
from resource_readback_fixes import apply_resource_readback_fixes

__author__ = "carunodev @stfu"

root = pathlib.Path(__file__).resolve().parent.parent
out = root / "linked"
def static_shuffle(match):
    mask = int(match[2], 16)
    lanes = [(mask >> (8 * i)) & 255 for i in range(16)]
    if any(i >= 32 for i in lanes):
        raise ValueError("invalid shuffle mask")
    return "simd::i8x16_shuffle_static::<" + ",".join(map(str, lanes)) + ">(" + match[1] + ")"

shuffle_pattern = r"simd::i8x16_shuffle\(([^\n;]*), 0x([0-9a-fA-F]{32})u128\)"
conversion_barriers = {
    "f_021052_audnorthaudioengine_update.rs": ("s",),
    "f_035418_cminimap_renderthread_setupstateforsupertilerender_rage_vector2.rs": ("u",),
    "f_044343_centityselect_drawdebug.rs": ("u",),
}
if "--static-shuffles-only" in sys.argv:
    count = 0
    for source in (out / "generated").glob("gen*/src/f_*.rs"):
        text = source.read_text(encoding="utf-8")
        text, n = re.subn(shuffle_pattern, static_shuffle, text)
        if n:
            source.write_text(text, encoding="utf-8")
            count += n
    print(f"specialized {count} SIMD shuffles")
    sys.exit(0)
if "--fix-atomics-only" in sys.argv:
    count = 0
    for source in (out / "generated").glob("gen*/src/f_*.rs"):
        text = source.read_text(encoding="utf-8")
        text, n = re.subn(r"(\.a_load_u32\([^\n;]*\)) as i64", r"\1 as u32 as i64", text)
        if n:
            source.write_text(text, encoding="utf-8")
            count += n
    print(f"corrected {count} unsigned atomic loads")
    sys.exit(0)
meta = json.loads((root / "build/index.json").read_text(encoding="utf-8"))
types = meta["types"]
canonical = {}
for index, (params, results) in sorted(types.items(), key=lambda item: int(item[0])):
    canonical.setdefault((tuple(params), tuple(results)), int(index))
rust = {"i32": "i32", "i64": "i64", "f32": "f32", "f64": "f64", "v128": "u128"}
wasm = {v: k for k, v in rust.items()}

def signature_id(params, results):
    return canonical[(tuple(params), tuple(results))]

def return_type(results):
    if not results:
        return ""
    return " -> " + (rust[results[0]] if len(results) == 1 else "(" + ", ".join(rust[r] for r in results) + ")")

def import_name(index, imp):
    from w2r.emitter import sanitize
    return f"imp_{index}_" + sanitize(imp["name"], index)[9:]

(out / "imports/src").mkdir(parents=True, exist_ok=True)
(out / "imports/Cargo.toml").write_text('''[package]
name = "engine-imports"
version = "0.1.0"
edition = "2021"
authors = ["carunodev @stfu"]

[lib]
name = "wasm_imp"

[dependencies]
host-imports = { package = "wasm-imp", path = "../../wasm-imp" }
wasm-rt = { path = "../../wasm-rt" }

[dev-dependencies]
engine-abi = { path = "../../crates/engine-abi" }
''', encoding="utf-8")
lines = ["#![allow(unused_variables)]", "pub use host_imports::*;", "use wasm_rt::Phantom;",
         'extern "C" { fn f_091110_emscripten_thread_init(ph: &mut Phantom, a: i64, b: i32, c: i32, d: i32, e: i32, f: i32); fn f_090693_emscripten_tls_init(ph: &mut Phantom) -> i64; }']
invokes = []
for index, imp in enumerate(meta["imports"]):
    name = import_name(index, imp)
    params, results = imp["params"], imp["results"]
    declarations = ", ".join(f"a{i}: {rust[t]}" for i, t in enumerate(params))
    arguments = ", ".join(f"a{i}" for i in range(len(params)))
    if imp["field"].startswith("invoke_"):
        invokes.append(name)
        target_params = params[1:]
        tid = signature_id(target_params, results)
        function_type = 'unsafe extern "C" fn(*mut Phantom' + "".join(", " + rust[t] for t in target_params) + ")" + return_type(results)
        lines += [f"pub unsafe fn {name}(ph: &mut Phantom, {declarations}){return_type(results)} {{",
                  f"    let address = ph.table_entry_typed(a0, {tid});",
                  '    #[cfg(target_family = "wasm")]',
                  f'    {{ host_imports::{name}({arguments}) }}',
                  '    #[cfg(not(target_family = "wasm"))]',
                  '    {',
                  f"    let function: {function_type} = core::mem::transmute(address);",
                  "    function(ph as *mut Phantom" + "".join(f", a{i}" for i in range(1, len(params))) + ")", "}", "}"]
        call = f"{name}(ph, {arguments})"
    elif imp["field"] == "_emscripten_init_main_thread_js":
        invokes.append(name)
        lines += [f"pub unsafe fn {name}(ph: &mut Phantom, {declarations}) {{",
                  '    #[cfg(target_family = "wasm")]',
                  f'    {{ host_imports::{name}({arguments}); }}',
                  '    #[cfg(not(target_family = "wasm"))]',
                  "    { f_091110_emscripten_thread_init(ph, a0, 1, 1, 1, 1048576, 0);",
                  "      f_090693_emscripten_tls_init(ph); }", "}"]
        call = f"{name}(ph, {arguments})"
    else:
        call = f"host_imports::{name}({arguments})"
    lines += [f"pub unsafe extern \"C\" fn table_import_{index}(ph: &mut Phantom" + (", " + declarations if declarations else "") + ")" + return_type(results) + " {", f"    {call}", "}"]
(out / "imports/src/lib.rs").write_text("\n".join(lines) + "\n", encoding="utf-8")
import_source = (out / "imports/src/lib.rs").read_text(encoding="utf-8")
import_source = import_source.replace("ph: &mut Phantom", "ph: *mut Phantom").replace("ph.table_entry_typed", "(*ph).table_entry_typed")
(out / "imports/src/lib.rs").write_text(import_source, encoding="utf-8")
if "--imports-only" in sys.argv:
    sys.exit(0)

indirect = re.compile(r'(let \w+ = ph\.table_entry\()([^;\n]+)(\);\s*let \w+: unsafe extern "C" fn\(\*mut Phantom([^)]*)\)(?: -> (.*?))? = core::mem::transmute\(\w+\);)')
stats = {"functions": 0, "typed_indirect_calls": 0, "context_invokes": 0, "negative_receivers": 0}

def typed(match):
    params = [wasm[t.strip()] for t in match[4].strip(", ").split(",") if t.strip()]
    result = match[5]
    results = [] if not result else [wasm[t.strip()] for t in result.strip("()").split(",") if t.strip()]
    tid = signature_id(params, results)
    stats["typed_indirect_calls"] += 1
    return match[1].replace("table_entry(", "table_entry_typed(") + match[2] + f", {tid}" + match[3]

source_root = pathlib.Path(sys.argv[sys.argv.index("--source") + 1]) if "--source" in sys.argv else root / "generated"
for crate in sorted(source_root.glob("gen[0-9][0-9][0-9]")):
    destination = out / "generated" / crate.name
    (destination / "src").mkdir(parents=True, exist_ok=True)
    manifest = (crate / "Cargo.toml").read_text(encoding="utf-8")
    manifest = manifest.replace('path = "../../wasm-rt"', 'path = "../../../wasm-rt"')
    manifest = manifest.replace('wasm-imp = { path = "../../wasm-imp" }', 'wasm-imp = { package = "engine-imports", path = "../../imports" }')
    (destination / "Cargo.toml").write_text(manifest, encoding="utf-8")
    for source in (crate / "src").glob("*.rs"):
        text = source.read_text(encoding="utf-8")
        if source.name.startswith("f_"):
            stats["functions"] += 1
            text = re.sub(r"(\.a_load_u32\([^\n;]*\)) as i64", r"\1 as u32 as i64", text)
            text = re.sub(shuffle_pattern, static_shuffle, text)
            text, count = re.subn(r"(-\d+(?:i32|i64))(?=\.[a-zA-Z_]\w*\()", r"(\1)", text)
            stats["negative_receivers"] += count
            text = indirect.sub(typed, text)
            if "ph.table_entry(" in text:
                raise ValueError(f"unmatched indirect call in {source}")
            for name in invokes:
                text, count = re.subn(r"\bimp::" + re.escape(name) + r"\((?!ph,)", f"imp::{name}(ph, ", text)
                stats["context_invokes"] += count
        text = text.replace("ph: &mut Phantom", "ph: *mut Phantom")
        text = re.sub(r"\bph\.", "(*ph).", text)
        if source.name in conversion_barriers:
            prefix = ["use wasm_rt::Phantom;", "mod simd {", "    pub use wasm_rt::simd::*;"]
            for signed in conversion_barriers[source.name]:
                name = f"f32x4_convert_i32x4_{signed}"
                prefix += ["    #[inline(never)]", f"    pub fn {name}(value: u128) -> u128 {{", f"        wasm_rt::simd::{name}(value)", "    }"]
            prefix.append("}")
            text = text.replace("use wasm_rt::{simd, Phantom};", "\n".join(prefix))
        text = apply_shader_pass_fixes(source.name, text)
        text = apply_task_upload_fixes(source.name, text)
        text = apply_minimap_mode_fixes(source.name, text)
        text = apply_minimap_mode_symbols(crate.name, source.name, text)
        text = apply_resize_binding_fixes(source.name, text)
        text = apply_resize_binding_symbols(crate.name, source.name, text)
        text = apply_resource_readback_fixes(source.name, text)
        target = destination / "src" / source.name
        if not target.exists() or target.read_text(encoding="utf-8") != text:
            target.write_text(text, encoding="utf-8")
    print(crate.name, flush=True)
(out / "prepare.json").write_text(json.dumps(stats, indent=2), encoding="utf-8")
print(stats, flush=True)
