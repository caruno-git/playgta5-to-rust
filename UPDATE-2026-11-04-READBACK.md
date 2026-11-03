# GPU readback lifetime correction

Author: carunodev @stfu

Resource destruction previously freed a readback destination before waiting for
the GPU worker to finish writing it. A late readback could then overwrite freed
storage and the allocator's free-list links. All 24 texture/buffer destructor
entrypoints now run their existing wait before the first storage free, including
the entrypoints with adjusted interface pointers.

The shared wait routine also checks its atomic completion flag after its existing
12,000-attempt limit. If a readback is still pending, it raises a diagnostic trap
before the caller can free the destination or its owner. Completion on the final
attempt returns normally. This intentionally stops an unresolved readback; it
does not recover a hung GPU mapping or guarantee a particular wall-clock timeout.

`tools/resource_readback_fixes.py` and the existing linker preparation hook make
the correction reproducible. The function signatures, guest layouts and normal
GPU command sequence stay unchanged.

Run the focused regressions from this source checkout with Python 3.9+ and `rustc`
on PATH:

```text
python -m unittest discover -s tools -p "test_*readback*.py" -v
git diff --check
```

The portable fixtures in `tools/testdata/readback` contain 25 exact pre-transform
Rust functions, with relative source paths and SHA-256 provenance. No unpublished
raw-generated tree, original binary, shaders or game assets are required for
these tests. Native fixtures execute the actual translated readback, wait and
destructor functions with a simulated asynchronous GPU completion, including
the first/final wait attempts and a permanently pending readback. They check
storage lifetime, completion ownership, canaries and normal deletion behavior.

The complete Rust engine built as WASM SHA-256
`3eef35f93081868c514fbdfff1f4afad49eea5c11416215a76c648fb4563ef1c`
(95,927,240 bytes), preserving its browser interface. Two isolated gameplay runs
with the normal WASM compiler and 16 reported cores completed 6 and 12 fullscreen
cycles: 54 actual canvas size changes across 2560x1600, 2560x1398 and 1638x1096.
Both continued presenting frames and moving the player, with no panic, GPU
validation error or HTTP error. One run included 60 seconds of streaming movement.
The private renderer also included separately checked stencil-channel and child
texture lifetime corrections, so these are combined-build runtime observations.

These bounded checks do not establish that every fullscreen crash, rendering
artifact or performance issue is fixed. This source update includes no
experimental GPU worker/cache changes or original game assets. Production was
unchanged; the combined build was exercised through additional lab-only URLs.
