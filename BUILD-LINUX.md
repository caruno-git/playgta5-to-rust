# Building the browser engine on Linux

This guide describes compiling the Rust engine into browser WebAssembly from
source. To run an existing build, download the
[prebuilt release](https://github.com/caruno-git/playgta5-to-rust/releases/latest)
and follow its bundled README. The prebuilt package needs Python and the
original game data; it does not require Rust or this build process.

The commands below mirror [linked/Build-Wasm.ps1](linked/Build-Wasm.ps1).
Rust 1.99.1 with its matching standard-library sources was used for the verified
Windows-hosted WASM64 build. A complete Linux-hosted build has not yet been
verified. The output target is `wasm64-unknown-unknown`, regardless of the host
operating system.

## Requirements

- Rust 1.99.1, its `rust-src` component, and a working host C/C++ linker.
- Python 3.12, Node.js and npm.
- The matching original `game.wasm` and `game.js` files.
- A checkout of this repository and enough memory and disk space for the
  generated Rust crates. The recipe uses one build job to limit peak memory.

The original engine files are external inputs and are not included in Git.
The WASM file must have this SHA-256:

```text
11ca8d2c04c5e843d18ff4aea4899d72c86973c6b031df334e67c446b2ae83e0
```

The build extracts passive data and module metadata from that file. Translated
Rust functions supply the executable engine code. The matching JavaScript is
used to generate the browser adapter. Original game data is needed at runtime,
but is not needed to compile the engine.

## Build steps

Run the following commands from the repository root. Replace the two
`/path/to/original/` paths with the matching local files. The generated outputs
are written under `linked/`; use a separate checkout when retaining an earlier
build.

```bash
(
set -eu
mkdir -p inputs
cp /path/to/original/game.wasm inputs/game.wasm
cp /path/to/original/game.js inputs/game.js
printf '%s  %s\n' 11ca8d2c04c5e843d18ff4aea4899d72c86973c6b031df334e67c446b2ae83e0 inputs/game.wasm | sha256sum -c -
python3 tools/w2r/gen_segments.py inputs/game.wasm wasm-rt/src
rustup toolchain install 1.99.1 --component rust-src
npm ci --prefix tools

unset PLAYGTA5_WASM PLAYGTA5_JS
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR="$PWD/linked/target-wasm64"
export RUSTC_BOOTSTRAP=1
export RUSTFLAGS='-Aexplicit_builtin_cfgs_in_flags --cfg target_feature="atomics" -C target-feature=+atomics,+bulk-memory,+mutable-globals,+simd128 -C link-arg=--shared-memory -C link-arg=--import-memory -C link-arg=--export-table -C link-arg=--export=__stack_pointer -C link-arg=-zstack-size=16777216'
cargo +1.99.1 build --release --manifest-path linked/Cargo.toml -p engine-linked --lib --target wasm64-unknown-unknown -Z build-std=std,panic_abort

rust_engine_artifact="$CARGO_TARGET_DIR/wasm64-unknown-unknown/release/engine_linked.wasm"
unset RUSTFLAGS
export CARGO_TARGET_DIR="$PWD/linked/target-tools"
mkdir -p linked/browser
cargo +1.99.1 run --release --manifest-path tools/wasm-package/Cargo.toml --bin split_memory -- "$rust_engine_artifact" linked/browser/game.wasm
cp linked/browser/game.memory.json linked/browser/memory-config.json
node tools/browser_adapter.cjs
)
```

`RUSTC_BOOTSTRAP=1` enables the `build-std` option required by this pipeline.
The commands run in a subshell so their environment overrides do not persist
in the calling shell.

## Generated files

| File | Purpose |
| --- | --- |
| `linked/browser/game.wasm` | Packaged Rust engine with separate guest and Rust memory. |
| `linked/browser/game.memory.json` | Packager report, including memory dimensions and the module hash. |
| `linked/browser/memory-config.json` | Copy of the memory report consumed by the adapter generator. |
| `linked/browser/game.js` | Browser adapter generated for those memory dimensions. |
| `linked/browser/adapter.json` | Adapter-generation report. |

The generated engine and adapter must be deployed together with the compatible
loader, workers, shaders and original game data. A successful compilation alone
does not verify world loading, rendering or gameplay.

The browser host must serve WASM as `application/wasm` and provide
`Cross-Origin-Opener-Policy: same-origin` and
`Cross-Origin-Embedder-Policy: require-corp`. HTTPS or localhost is required for
the browser GPU APIs. See [release status](linked/STATUS.md) for current runtime
verification and [streaming documentation](docs/streaming.md) for asset-serving
requirements.
