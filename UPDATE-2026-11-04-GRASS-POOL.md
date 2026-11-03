# Grass append-buffer reuse

Author: carunodev @stfu

The grass renderer's pool returns eligible buffers to the front of its free list
but previously allocated from the back. This rotates draws through all 3000
preallocated buffers even when a much smaller set has completed the existing
three-frame retirement period, increasing the variety of GPU bindings.

The allocator now prefers the recently returned end of the already-free list.
Recorded frames 0–2 retain the original selection, including the u32 wrap window.
Retirement eligibility, intrusive list updates, pool capacity, allocation
fallback, resource flags, GPU counter resets and command order are unchanged.
The existing readback lifetime checks remain in place.

`tools/grass_pool_fixes.py` verifies the complete pre-transform function before
applying this single-expression change. Its linker hook is idempotent and rejects
source changes outside the reviewed selection. Function signatures and guest
layouts stay unchanged.

Run the portable focused checks with Python 3.9+ and `rustc` on PATH:

```text
python -m unittest discover -s tools -p "test_grass_pool*.py" -v
git diff --check
```

Six tests pass. The actual translated allocator fixture covers steady reuse,
skipped frames, pool exhaustion and frame-counter wrap, checking links, counts,
canaries and simulated deferred GPU completion. With 80 allocations per frame
for 60 frames, distinct resource use falls from 3000 to 240 without new
allocations; both variants preserve the same logical completion checksum.
The exact original Rust fixture and portable harness have SHA-256 provenance
under `tools/testdata/grass_pool`.

The complete Rust engine built as WASM SHA-256
`a8cf2692f0009499a584d98b655a04d1865b77fcf6ded6734337682b825909f9`
(95,927,243 bytes), preserving its browser interface. A second isolated run with
the normal WASM compiler and 16 reported cores completed 35 seconds of streaming
movement and all six fullscreen cycles: 18 verified canvas size changes across
2560x1600, 2560x1398 and 1638x1096. It continued presenting frames and moving the
player, with no panic, GPU validation error or HTTP error.

The first, longer run completed 60 seconds of movement but stopped during its
fourth resize cycle after 11 size changes because free host memory fell below
the test harness's 512 MiB guard. It is not a completed resize pass. The second
run retained at least 742,932,480 bytes of free host memory. Continued memory
growth remains a follow-up; the successful bounded run does not establish long
session stability. Both runs used the separately checked stencil-channel and
child-texture lifetime corrections in their GPU worker.

Fixture results and these separate sessions are not a controlled measurement of
game FPS gain. This update does not establish that all rendering or performance
issues are resolved. It includes no compiled engine, GPU worker candidate,
shaders, assets or graphics-setting changes.
