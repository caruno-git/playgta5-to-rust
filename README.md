# FivePlay-to-rust

Current source snapshot of the PlayGTA5 WASM64 engine port to Rust.

## Included

- All 91,026 translated function bodies in `linked/generated/gen000` through
  `gen022`, not the earlier signature-only skeletons.
- Linear-memory runtime, atomics, passive-segment handling, SIMD helpers and tests.
- Context-aware imports, checked indirect calls, browser export adapters and the
  memory-isolating WASM packager.
- Native host, ABI, fixed-timestep runtime and headless-server sources.
- WAT translation, segment extraction and browser adapter generation tools.

## Verification Status

A prior Rust release loaded the real world in Chrome, rendered through WebGPU,
accepted movement, and completed two minutes after world load without a WASM
crash or GPU validation error. It was slower than the original in the comparison
scene: approximately 32-42 FPS versus approximately 60 FPS.

This snapshot also includes the subsequent optimization: 86,035 shuffle sites
use constant lane indices and the standard WASM SIMD intrinsic, with short memory
operations inlined. A compiled fixture emits `i8x16.shuffle` and passes 256
reference vectors. The latest complete SIMD-optimized build and its gameplay/FPS
retest are not yet verified. This is a development snapshot, not a finished
performance release. See `linked/STATUS.md`.

## External Inputs

Game binaries, original JavaScript glue, the 21 GB game assets, segment blobs,
save files, compiled outputs and local caches are deliberately not included.
Supply the original build locally. The required original `game.wasm` SHA-256 is:

```text
11ca8d2c04c5e843d18ff4aea4899d72c86973c6b031df334e67c446b2ae83e0
```

The original module is used for passive bytes and module metadata at build time;
translated Rust function bodies provide the engine code. It is not used as a
runtime fallback. Game assets and the remaining JS host/loader are separate.

## Build

Tested toolchain: Rust 1.99.1, its matching `rust-src` component, Python 3.12,
Node.js and PowerShell 7. WASM64 uses `-Z build-std` with `RUSTC_BOOTSTRAP=1`.

```powershell
rustup component add rust-src
./Prepare-Inputs.ps1 -Wasm /path/to/original/game.wasm -JavaScript /path/to/original/game.js
npm ci --prefix tools
./linked/Build-Wasm.ps1
```

Output: `linked/browser/game.wasm`, matching `game.js` and memory metadata.
Keep those generated files together. They are intended for the existing
PlayGTA5 loader and assets, not as a native Windows executable.

For Linux, the commands and required flags are also shown in `BUILD-LINUX.md`.

## Tests

Run input preparation first so `wasm-rt/src/segments.bin` exists.

```powershell
cargo test --workspace -- --test-threads=1
cargo test --manifest-path wasm-imp/Cargo.toml -- --test-threads=1
cargo test --manifest-path linked/Cargo.toml -p engine-imports --test context
cargo test --manifest-path tools/wasm-package/Cargo.toml --bin split_memory
python tools/test_emitter_semantics.py
```

The independent boot-image comparison additionally requires `inputs/va.bin`.
Provide it with `-ReferenceImage` during preparation, then run:

```powershell
cargo test -p wasm-rt --test original_engine -- --ignored
cargo run -p wasm-rt --example init_check
```

Regenerating translated bodies is optional and needs the matching WAT in
`inputs/game.wat` (or the `WAT` environment variable):

```powershell
python tools/retranslate_linked.py
```

Script author: carunodev @stfu. No repository-wide license grant is implied for
the recovered game-derived code; upstream rights remain with their owners.
