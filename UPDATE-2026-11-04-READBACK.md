# GPU readback lifetime correction — November 4, 2026

Author: carunodev @stfu

This change prevents asynchronous GPU readbacks from writing into storage that
has already been freed during resource destruction. It is included in subsequent
releases; the [README](README.md) identifies the current downloadable build.

## Cause and correction

A texture or buffer destructor could free its readback destination before
waiting for the GPU worker's write to finish. A late write could corrupt the
freed storage and the allocator's free-list links.

All 24 affected destructor entrypoints now perform their existing wait before
the first storage free, including entrypoints with adjusted interface pointers.
The shared wait routine also checks its atomic completion flag after the
existing 12,000-attempt limit. A still-pending readback raises a diagnostic trap
before the destination or its owner can be freed. Completion on the final
attempt returns normally.

The trap prevents unsafe reclamation. It does not recover a hung GPU mapping or
define a wall-clock timeout. Function signatures, guest layouts and the normal
GPU command sequence remain unchanged.

The reproducible transformation is implemented in
[resource_readback_fixes.py](tools/resource_readback_fixes.py) and applied by the
linker preparation hook.

## Regression coverage

Python 3.9 or newer and `rustc` on PATH are required for the focused checks:

```text
python -m unittest discover -s tools -p "test_*readback*.py" -v
```

The [portable fixtures](tools/testdata/readback) contain 25 exact pre-transform
Rust functions and SHA-256 provenance. They require no original binary, shaders
or game assets. Native tests execute the translated readback, wait and destructor
functions with simulated asynchronous completion. Cases cover the first and
final wait attempts, permanently pending readbacks, storage lifetime, completion
ownership, surrounding-memory canaries and normal deletion.

## Runtime validation

The intermediate engine used for these checks was 95,927,240 bytes, SHA-256:

```text
3eef35f93081868c514fbdfff1f4afad49eea5c11416215a76c648fb4563ef1c
```

Two isolated gameplay runs used the normal WASM compiler and 16 reported cores.
They completed six and twelve fullscreen cycles, totalling 54 canvas-size
changes across 2560×1600, 2560×1398 and 1638×1096. Frames continued to render and
the player continued moving without a recorded panic, GPU validation error or
HTTP error. One run included 60 seconds of movement with asset streaming.

The tested builds also contained stencil-channel and child-texture lifetime
corrections in the GPU worker. These are combined-build observations, not a
claim that this single change fixes every fullscreen crash or rendering issue.
Long-session stability and recovery from a permanently hung GPU remain outside
the scope of these checks.
