# Grass append-buffer reuse — November 4, 2026

Author: carunodev @stfu

This change reduces unnecessary rotation through the grass renderer's GPU
buffers. It preserves graphics settings, pool capacity and the existing
retirement rules. The [README](README.md) covers the current release.

## Cause and correction

Eligible buffers returned to the front of the free list, but allocation took
buffers from the back. Draws therefore rotated through all 3,000 preallocated
buffers even when a smaller set had already completed the three-frame retirement
period. This increased the variety of GPU bindings.

Allocation now prefers the recently returned end of the already-free list.
Recorded frames 0–2 retain the original selection, including the `u32` wrap
window. Retirement eligibility, intrusive-list updates, allocation fallback,
resource flags, GPU counter resets and command ordering are unchanged.
The [readback lifetime checks](UPDATE-2026-11-04-READBACK.md) remain in place.

[grass_pool_fixes.py](tools/grass_pool_fixes.py) checks the full pre-transform
function before changing the selection expression. Its linker hook is idempotent
and rejects unexpected source changes. Function signatures and guest layouts
remain compatible.

## Regression coverage

The focused suite requires Python 3.9 or newer and `rustc` on PATH:

```text
python -m unittest discover -s tools -p "test_grass_pool*.py" -v
```

All six tests passed. The actual translated allocator fixture covers steady
reuse, skipped frames, pool exhaustion and frame-counter wrap. It checks list
links, counts, surrounding-memory canaries and simulated deferred GPU completion.

With 80 allocations per frame over 60 frames, the fixture's distinct resource
count fell from 3,000 to 240 without adding allocations. Both variants produced
the same logical completion checksum. The original Rust fixture and portable
harness have SHA-256 provenance in [tools/testdata/grass_pool](tools/testdata/grass_pool).

## Runtime validation

The intermediate engine used for these checks was 95,927,243 bytes, SHA-256:

```text
a8cf2692f0009499a584d98b655a04d1865b77fcf6ded6734337682b825909f9
```

A run with the normal WASM compiler and 16 reported cores completed 35 seconds
of movement with streaming and six fullscreen cycles. All 18 canvas-size
changes across 2560×1600, 2560×1398 and 1638×1096 were verified. Frames continued
to render and the player remained controllable without a recorded panic, GPU
validation error or HTTP error.

An earlier, longer run completed 60 seconds of movement but stopped during its
fourth resize cycle, after 11 size changes, when host free memory fell below
the test harness's 512 MiB guard. That run is an incomplete resize test. The
successful run retained at least 742,932,480 bytes of free host memory.
Both builds included the separately checked stencil-channel and child-texture
lifetime corrections in their GPU worker.

These fixture results and separate gameplay sessions do not establish an FPS
gain. Continued memory growth and long-session stability remain unresolved.
