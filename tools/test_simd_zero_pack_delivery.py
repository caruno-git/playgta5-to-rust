import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

__author__ = "carunodev @stfu"
ROOT = Path(__file__).resolve().parent.parent
FIXTURES = ROOT / "tools/testdata/simd_zero_load_pack"
BEFORE = FIXTURES / "f_012912_before_zero_load_fix.rs"
DELIVERED = ROOT / "linked/generated/gen003/src/f_012912_rage_pack_core32_float_vector_4_ref_float_vector_4.rs"
SYMBOL = "f_012912_rage_pack_core32_float_vector_4_ref_float_vector_4"


class ZeroPackSourceTests(unittest.TestCase):
    def test_exact_pre_fix_source_has_portable_provenance(self):
        manifest = json.loads((FIXTURES / "provenance.json").read_text(encoding="utf-8"))
        self.assertEqual(manifest["author"], __author__)
        self.assertEqual(manifest["fixture"], BEFORE.name)
        self.assertEqual(manifest["sourceCommit"], "ea39fd0baaf7bab7d3a0857ec948585314c54fc8")
        self.assertEqual(manifest["sourcePath"], DELIVERED.relative_to(ROOT).as_posix())
        data = BEFORE.read_bytes()
        self.assertEqual(manifest["bytes"], len(data))
        self.assertEqual(manifest["sha256"], hashlib.sha256(data).hexdigest())
        historical = data.decode("utf-8").replace("\r\n", "\n").replace("\n", "\r\n").encode("utf-8")
        self.assertEqual(manifest["historicalCrlfSha256"], hashlib.sha256(historical).hexdigest())

    def test_delivered_pack_changes_only_the_three_zero_loads(self):
        before = BEFORE.read_text(encoding="utf-8")
        expected = before
        for address in (6706236, 6706240, 6706244):
            old = f"(*ph).read_splat(0i64 + {address}, 32)"
            new = f"(*ph).read_u32(0i64 + {address}) as u128"
            self.assertEqual(expected.count(old), 1)
            expected = expected.replace(old, new, 1)
        self.assertEqual(expected, DELIVERED.read_text(encoding="utf-8"))


@unittest.skipUnless(shutil.which("cargo") and shutil.which("rustc"), "cargo and rustc required for delivered native Pack_Core32 regression")
class ZeroPackExecutionTests(unittest.TestCase):
    def test_actual_delivered_function_preserves_xyz_and_packed_w_in_debug_and_release(self):
        before_hash = hashlib.sha256(DELIVERED.read_bytes()).hexdigest()
        options = {"creationflags": subprocess.CREATE_NO_WINDOW | subprocess.BELOW_NORMAL_PRIORITY_CLASS} if os.name == "nt" else {}
        results = []
        with tempfile.TemporaryDirectory(prefix="simd-pack-delivery-") as temporary:
            directory = Path(temporary)
            source = directory / "src"
            source.mkdir()
            manifest = "[package]\nname=\"simd-pack-delivery\"\nversion=\"0.1.0\"\nedition=\"2021\"\nauthors=[\"carunodev @stfu\"]\n[workspace]\n[dependencies]\nwasm-rt={path=" + json.dumps((ROOT / "wasm-rt").as_posix()) + "}\n"
            (directory / "Cargo.toml").write_text(manifest, encoding="utf-8")
            (source / "main.rs").write_bytes((FIXTURES / "harness.rs").read_bytes())
            (source / "delivered.rs").write_bytes(DELIVERED.read_bytes())
            old = BEFORE.read_text(encoding="utf-8")
            self.assertEqual(old.count("fn " + SYMBOL + "("), 1)
            (source / "before.rs").write_text(old.replace("fn " + SYMBOL + "(", "fn before_" + SYMBOL + "(", 1), encoding="utf-8")
            env = os.environ.copy()
            for name in ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTC_BOOTSTRAP"):
                env.pop(name, None)
            env["CARGO_BUILD_JOBS"] = "1"
            env["CARGO_TARGET_DIR"] = str(directory / "target")
            for release in (False, True):
                command = [shutil.which("cargo"), "run", "--manifest-path", str(directory / "Cargo.toml"), "--offline", "--quiet"]
                if release:
                    command.append("--release")
                run = subprocess.run(command, capture_output=True, text=True, timeout=120, env=env, **options)
                self.assertEqual(run.returncode, 0, run.stdout + run.stderr)
                report = json.loads(run.stdout)
                self.assertEqual(report["cases"], 2069)
                self.assertEqual(report["originalZeroVelocityZBits"], "60080200")
                self.assertTrue(report["xyzPreserved"] and report["packedWUnchanged"] and report["canariesPreserved"])
                results.append(report)
        self.assertEqual(results[0], results[1])
        self.assertEqual(hashlib.sha256(DELIVERED.read_bytes()).hexdigest(), before_hash)


if __name__ == "__main__":
    unittest.main()
