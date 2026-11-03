# Active Port Status

Updated: 2026-11-03. Production has not been modified.

## Current Artifact

The current Rust-compiled browser engine is `browser/game.wasm`, 95,927,000 bytes,
SHA-256 `642304fc7a0ad542b512e4624194db7561b54fc90ab1dee5c4a585d57ca59159`.
It contains all 91,026 translated engine bodies and has executed constructors,
main, scripts, streaming, the game loop, keyboard input and WebGPU rendering.
It remains browser WASM, not a native Windows executable.

The asset-complete isolated test site is:
https://gta.caruno.wtf:2087/?mode=sandbox

The lab shares all 5814 original game files read-only from the VPS. No files
are missing; the manifest totals 20,944,285,552 bytes. Complete sampled archive
hashes and all 32 startup batch-cache payloads match. The production service,
original nginx master and original code hashes remain unchanged.

Four inherited FOW pass indices were corrected against the actual normal/LQ
`im.fxc` contracts. The old pass used a clip-space vertex shader for world-space
minimap geometry. The source generator preserves the fix and all three LLVM
conversion barriers. Current screenshots show textured GTA streets and minimap
roads, with keyboard movement and no captured engine/GPU validation errors.
Short minimap acceptance: task workspace `work/vps-lab/browser-fow-first-frame`.

The cloth-instancing task now installs the existing upload/cancel callbacks and
temporary allocator, as the sibling cloth-update task already does. Original
task plus real Unlock_Internal reproduces changed CPU bytes with no queued
upload; the fixed task queues exact bytes, offset and length. Four delivery
regressions and ten transform/task tests pass. The lab-only live comparison
reduced the corresponding warning from 87 occurrences to zero. It did not fix
the separate stretched-hair defect.

The lab IO worker fixes complete OPFS writes, journal retry, independent slice
completion, demand promotion and startup-prefetch scheduling. The lab GPU worker
caps free staging-buffer retention at 32 MiB without changing active resources.

Long asset-complete gameplay tests reach the Windows harness's 512 MiB free-RAM
floor, which closes only the owned test browser. These are protective stops,
not captured engine crashes. Long-session stability remains unestablished.
Tests used `cores=2` or `cores=4` and `-randomseed=1`. Recent 390x844 diagnostic
captures use a test-browser-only baseline WASM compiler with one compilation
task to fit the host. These runs passed loading and movement, but their startup
times/FPS are not default-compiler benchmarks. No site graphics-quality,
resolution, compiler or memory-limit defaults were changed.
Other inherited `im` pass users, including optional UI3D composition, were not
globally shifted by the four-case FOW correction and need separate validation.

A stretched player-model geometry defect is also under active investigation.
It is visible after movement on both older and current artifacts; live draw
isolation removes the distortion together with ped-family draws and restores
the original renderer afterward. This is not yet a fully correct visual
acceptance result, and the engine goal remains active until it is addressed.

Reproduction and deployment evidence are in task workspace `work/vps-lab/README.md`.
The latest incremental build used `work/vps-lab/Build-FowEngine.ps1` with
`-CandidateName engine-cloth-upload-candidate` and log
`work/engine-cloth-upload-build.log`. The prior canonical artifact is preserved
under `verification/pre-cloth-upload-16661fd93655`; the earlier pre-FOW backup
remains under `verification/pre-fow-d442bc27b56b`.

No extra Binaryen optimization was applied. A conservative audit found only
326 unreferenced functions totaling 22,916 body bytes. The removed 91,026 exports
were actual translated functions, not 91,026 duplicated wrapper bodies. A shared
memory64 probe found no resident-memory benefit from reducing untouched capacity
alone, so the 3 GiB start and 16 GiB growth maximum remain unchanged.

## Historical Notes

The following older runs used partial cached assets. Their longer loop results
do not establish current asset-complete visual or stability acceptance.

The first Rust release has executed the original initialization, constructors,
main, scripts, streaming, game loop and WebGPU rendering. Two browser runs loaded
the world, accepted movement and recorded no WASM crash or GPU validation errors.
The longer run completed two minutes after world load. Screenshots and results
are in `verification/first-pass` and `verification/stability`.

That first release was not the final optimized artifact: a same-scene reference
run showed about 60 FPS for the original and 32-42 for the Rust candidate. The
historical working but slower candidate had SHA-256
`1863448c26911ca1dfe043e10ce9975df5c0fa971c84f6c0421d01a0d626e5b4`.

CPU profiling (`verification/profile/engine.cpuprofile`) identified scalar
`i8x16_shuffle` as a hot path and animation/job waits on the main thread. The
current Rust sources specialize 86,035 shuffle sites with const lane indices and
use the standard WASM SIMD intrinsic. A small compiled WASM fixture now contains
the actual `i8x16.shuffle` instruction, and 256 executed vectors match the
independent reference. SIMD tests pass. Short float/vector loads, bulk-memory
operations and typed table lookups have also been marked for inlining.

The earlier SIMD build log is `work/engine-simd-release.log` in the task
workspace. The current release is identified above. To rebuild, package
with `tools/wasm-package` binary `split_memory`, publish its memory report as
`browser/memory-config.json`, regenerate `browser/game.js` with
`tools/browser_adapter.cjs`, then repeat gameplay and FPS comparison. Do not
declare the optimization complete merely from compilation or the earlier pass.

The optional Binaryen optimization experiment against the older release was
stopped to leave RAM for the current Rust compilation. It is not an approved
candidate and is not required for the build pipeline.

Local servers: Rust on port 8940 and reference on 8941. These are isolated staging
servers. Their assets come from the existing verified cache/snapshot and public
release mirror, not from production writes.
