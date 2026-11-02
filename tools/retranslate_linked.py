import json
import os
import pathlib
import shutil
import subprocess
import sys
from types import SimpleNamespace

__author__ = "carunodev @stfu"
ROOT = pathlib.Path(__file__).resolve().parent.parent
RAW = ROOT / "linked/raw-generated"
BUILD = ROOT / "linked/raw-build"
os.environ["WAT2RUST_GEN"] = str(RAW)
os.environ["WAT2RUST_BUILD"] = str(BUILD)
import wat2rust

def main():
    BUILD.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(ROOT / "build/index.json", BUILD / "index.json")
    wat2rust.cmd_run(SimpleNamespace(jobs=4, batch=200, limit=0))
    results = json.loads((BUILD / "results.json").read_text(encoding="utf-8"))
    failed = [r for r in results if r["failed"] or r["untranslated"]]
    if failed:
        raise RuntimeError(f"{len(failed)} translation failures: {failed[:3]}")
    subprocess.run([sys.executable, str(ROOT / "tools/link_engine.py"), "--source", str(RAW)], check=True)

if __name__ == "__main__":
    main()
