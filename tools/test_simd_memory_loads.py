import json
import os
import pathlib
import shutil
import subprocess
import tempfile
import unittest

from w2r.emitter import Emitter, parse_body
from w2r.fallback import StateMachine

__author__ = "carunodev @stfu"
ROOT = pathlib.Path(__file__).resolve().parent.parent
OPS = {"v128.load8_splat": 1, "v128.load16_splat": 2, "v128.load32_splat": 4, "v128.load64_splat": 8, "v128.load32_zero": 4, "v128.load64_zero": 8}


@unittest.skipUnless(all(shutil.which(name) for name in ("cargo", "rustc", "node")), "cargo, rustc and node required for native/WASM SIMD comparison")
class SimdMemoryLoads(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="simd-memory-semantics-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.directory = pathlib.Path(cls.temporary.name)
        cls.process_options = {"creationflags": subprocess.CREATE_NO_WINDOW | subprocess.BELOW_NORMAL_PRIORITY_CLASS} if os.name == "nt" else {}
        (cls.directory / "src").mkdir()
        manifest = "[package]\nname=\"simd-memory-semantics\"\nversion=\"0.1.0\"\nedition=\"2021\"\nauthors=[\"carunodev @stfu\"]\n[workspace]\n[dependencies]\nwasm-rt={path=" + json.dumps((ROOT / "wasm-rt").as_posix()) + "}\n"
        (cls.directory / "Cargo.toml").write_text(manifest, encoding="utf-8")
        generated = [
            "#![allow(warnings)]", "use wasm_rt::Phantom;",
            "enum Val { I64(i64), V128(u128) }",
            "trait PopVal { fn as_i64(self) -> i64; } impl PopVal for Option<Val> { fn as_i64(self) -> i64 { match self { Some(Val::I64(v)) => v, _ => panic!(\"fixture stack type\") } } }",
            "mod rt { pub use wasm_rt::*; pub struct Frame; pub fn ret_v128(stack: &mut Vec<super::Val>) -> u128 { match stack.pop() { Some(super::Val::V128(v)) => v, _ => panic!(\"fixture result type\") } } }",
        ]
        dispatch = []
        for backend, emitter_type in [("structured", Emitter), ("fallback", StateMachine)]:
            for op in OPS:
                for offset in [0, 3]:
                    identifier = backend + "_" + op.replace(".", "_") + "_" + str(offset)
                    wat = f"(func ${identifier} (param i64) (result v128)\nlocal.get 0\n{op} offset={offset} align=1\nreturn\n)"
                    emitter = emitter_type(parse_body(wat, {}, 0), 0, {}, None, {}, None)
                    emitter.fname = identifier
                    emitter.type_sigs = {}
                    generated.extend(emitter.run())
                    if emitter.failed or emitter.untranslated:
                        raise AssertionError((identifier, emitter.fail_reason, emitter.untranslated))
                    dispatch.append(f"({json.dumps(backend)}, {json.dumps(op)}, {offset}) => {identifier}(&mut ph, address),")
        generated.extend([
            "fn main() { let args: Vec<String> = std::env::args().collect(); let address: i64 = args[4].parse().unwrap(); let offset: u32 = args[3].parse().unwrap();",
            "let mut memory: Vec<u8> = (0..65536usize).map(|i| (i.wrapping_mul(37).wrapping_add(129) & 255) as u8).collect();",
            "let value = unsafe { let mut ph = Phantom::new(memory.as_mut_ptr(), memory.len() as u64); match (args[1].as_str(), args[2].as_str(), offset) {",
            *dispatch,
            "_ => panic!(\"Unknown fixture\") } }; for byte in value.to_le_bytes() { print!(\"{:02x}\", byte); } }",
        ])
        (cls.directory / "src/main.rs").write_text("\n".join(generated), encoding="utf-8")
        env = os.environ.copy()
        for name in ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTC_BOOTSTRAP"):
            env.pop(name, None)
        env["CARGO_BUILD_JOBS"] = "1"
        env["CARGO_TARGET_DIR"] = str(cls.directory / "target")
        compiled = subprocess.run([shutil.which("cargo"), "build", "--manifest-path", str(cls.directory / "Cargo.toml"), "--offline", "--quiet"], capture_output=True, text=True, timeout=60, env=env, **cls.process_options)
        if compiled.returncode:
            raise AssertionError(compiled.stderr)
        cls.executable = cls.directory / "target/debug" / ("simd-memory-semantics.exe" if os.name == "nt" else "simd-memory-semantics")

    @classmethod
    def tearDownClass(cls):
        cls.temporary.cleanup()

    def compare(self, cases):
        oracle = subprocess.run([shutil.which("node"), str(ROOT / "tools/testdata/simd_memory_oracle.cjs")], input=json.dumps(cases), check=True, capture_output=True, text=True, timeout=15, **self.process_options)
        results = json.loads(oracle.stdout)["results"]
        for expected in results:
            for backend in ["structured", "fallback"]:
                with self.subTest(backend=backend, **expected):
                    actual = subprocess.run([str(self.executable), backend, expected["op"], str(expected["offset"]), str(expected["address"])], capture_output=True, text=True, timeout=10, **self.process_options)
                    if expected["trap"]:
                        self.assertNotEqual(actual.returncode, 0)
                        self.assertIn("out of bounds", actual.stderr.lower())
                    else:
                        self.assertEqual(actual.returncode, 0, actual.stderr)
                        self.assertEqual(actual.stdout, expected["hex"])
                        if expected["op"].endswith("_zero"):
                            self.assertEqual(bytes.fromhex(actual.stdout)[OPS[expected["op"]]:], bytes(16 - OPS[expected["op"]]))

    def test_zero_loads_clear_upper_lanes_and_genuine_splats_stay_replicated(self):
        cases = [{"op": op, "offset": offset, "address": address} for op in OPS for offset in [0, 3] for address in [0, 1, 13]]
        self.compare(cases)

    def test_exact_end_reads_succeed_and_crossing_end_traps(self):
        cases = [{"op": op, "offset": offset, "address": 65536 - width - offset + extra} for op, width in OPS.items() for offset in [0, 3] for extra in [0, 1]]
        self.compare(cases)


if __name__ == "__main__":
    unittest.main()
