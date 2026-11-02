import pathlib
import shutil
import subprocess
import tempfile
import unittest
from w2r.emitter import Emitter, parse_body

__author__ = "carunodev @stfu"

class Semantics(unittest.TestCase):
    def test_unsigned_atomic_load_is_zero_extended(self):
        body = parse_body("(func $atomic (param i64) (result i64)\nlocal.get 0\ni64.atomic.load32_u\n)", {}, 0)
        emitter = Emitter(body, 0, {}, None, {}, None)
        emitter.fname = "atomic"
        emitter.type_sigs = {}
        self.assertIn("as u32 as i64", "\n".join(emitter.run()))

    def test_local_tee_captures_old_values_and_negative_receivers(self):
        cases = [
            ("capture", ["local.get 0", "local.get 0", "i64.const 8", "i64.add", "local.tee 0", "i64.add"], 10, 28),
            ("negative", ["i64.const -128", "local.get 0", "i64.sub"], 7, -135),
        ]
        generated = ["struct Phantom;"]
        assertions = []
        for name, instructions, value, expected in cases:
            wat = "(func $" + name + " (param i64) (result i64)\n" + "\n".join(instructions) + "\n)"
            body = parse_body(wat, {}, 0)
            emitter = Emitter(body, 0, {}, None, {}, None)
            emitter.fname = name
            emitter.type_sigs = {}
            generated += emitter.run()
            self.assertFalse(emitter.failed, emitter.fail_reason)
            self.assertFalse(emitter.untranslated)
            assertions.append(f"assert_eq!({name}(&mut ph, {value}), {expected});")
        generated += ["fn main() { let mut ph = Phantom; unsafe { " + " ".join(assertions) + " } }"]
        with tempfile.TemporaryDirectory(prefix="rust-emitter-") as directory:
            directory = pathlib.Path(directory)
            source = directory / "test.rs"
            executable = directory / "test.exe"
            source.write_text("\n".join(generated), encoding="utf-8")
            subprocess.run([shutil.which("rustc"), "--edition", "2021", str(source), "-o", str(executable)], check=True, capture_output=True, text=True)
            subprocess.run([str(executable)], check=True, timeout=10)

if __name__ == "__main__":
    unittest.main()
