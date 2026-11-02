# Active Port Status

This is local work. Production has not been modified.

The first Rust release has executed the original initialization, constructors,
main, scripts, streaming, game loop and WebGPU rendering. Two browser runs loaded
the world, accepted movement and recorded no WASM crash or GPU validation errors.
The longer run completed two minutes after world load. Screenshots and results
are in `verification/first-pass` and `verification/stability`.

That first release is NOT the final optimized artifact: a same-scene reference
run showed about 60 FPS for the original and 32-42 for the Rust candidate. The
current `browser/game.wasm` is that working but slower candidate, SHA-256
`1863448c26911ca1dfe043e10ce9975df5c0fa971c84f6c0421d01a0d626e5b4`.

CPU profiling (`verification/profile/engine.cpuprofile`) identified scalar
`i8x16_shuffle` as a hot path and animation/job waits on the main thread. The
current Rust sources specialize 86,035 shuffle sites with const lane indices and
use the standard WASM SIMD intrinsic. A small compiled WASM fixture now contains
the actual `i8x16.shuffle` instruction, and 256 executed vectors match the
independent reference. SIMD tests pass. Short float/vector loads, bulk-memory
operations and typed table lookups have also been marked for inlining.

The updated full release attempt stopped in LLVM while compiling `gen005`:
`Cannot select: v4f32 = WebAssemblyISD::CONVERT_LOW_S`. This is a code-generation
failure, not a failed runtime test. Log: `work/engine-simd-release.log` in the task
workspace. After resolving it and completing the build, package
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
