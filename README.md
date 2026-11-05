# PlayGTA5 to Rust

A Rust port of the PlayGTA5 browser engine. The game runs as WebAssembly in a
WebGPU-capable browser. This repository contains the translated engine, runtime,
browser streaming worker and build tools.

[Play online](https://gta.caruno.wtf:8443/?mode=sandbox) ·
[Download a build](https://github.com/caruno-git/playgta5-to-rust/releases/latest) ·
[Setup guide](docs/running.md) · [Project status](linked/STATUS.md)

## Getting started

The online version runs without a local installation. Use a recent desktop
Chrome or Edge with hardware acceleration enabled. Loading time and performance
depend on the browser, available memory and network connection.

For local play, download **PlayGTA5-Rust-Web-2026.11.05.tar.gz** from
[Releases](https://github.com/caruno-git/playgta5-to-rust/releases/latest).
It contains the compiled `game.wasm`, browser adapter, GPU and IO workers,
shaders, loading screen and Python launcher. Running this package requires
Python 3.10 or newer; a Rust compiler is only needed to build from source.
GitHub's automatically generated **Source code** archives contain the repository
sources instead of the compiled build.

The original game data is separate. Local play requires a compatible PlayGTA5
`data/` directory of about 21 GB, with manifest version `1791169379-5814`.
The approximately 98 MB release archive contains the browser runtime, not those
game data files. A symbolic link or directory junction can reuse an existing
installation. See the [Linux and Windows setup instructions](docs/running.md).

## Release status

The current development release is
[2026.11.05](https://github.com/caruno-git/playgta5-to-rust/releases/tag/v2026.11.05-rust-web).
It includes fixes for SIMD loads, character cloth geometry, GPU readback lifetime,
map rendering and resource reuse, together with demand-prioritized streaming.

The engine has loaded the game world, rendered through WebGPU and accepted player
movement in browser tests. Map, cloth and fullscreen-transition checks have also
passed. Cold loading can still be slow, and sustained driving and long-session
stability are not fully verified. Test conditions and remaining limitations are
listed in [Project status](linked/STATUS.md).

## Repository contents

| Path | Purpose |
| --- | --- |
| `linked/` | Translated engine, import bridge and browser build entry point |
| `wasm-rt/` | Linear-memory runtime, atomics and SIMD support |
| `wasm-imp/` | Supporting import implementations and tests |
| `browser/` | Browser streaming worker |
| `crates/` | Host, ABI, runtime and server components |
| `tools/` | Translation, packaging, adapter generation and regression tests |
| `docs/` | Installation and streaming documentation |

## Building from source

The verified build environment used Rust 1.99.1 with its matching `rust-src`,
Python 3.12, Node.js and PowerShell 7. The target is `wasm64-unknown-unknown` with
shared memory and SIMD. The build uses `-Z build-std` through `RUSTC_BOOTSTRAP=1`.

Source builds require the matching original `game.wasm` and `game.js` as external
inputs. The original WASM supplies passive data and module metadata during the
build; translated Rust function bodies provide the executable engine code.
The expected original WASM SHA-256 is:

```text
11ca8d2c04c5e843d18ff4aea4899d72c86973c6b031df334e67c446b2ae83e0
```

From the repository root in PowerShell:

```powershell
rustup component add rust-src
./Prepare-Inputs.ps1 -Wasm /path/to/original/game.wasm -JavaScript /path/to/original/game.js
npm ci --prefix tools
./linked/Build-Wasm.ps1
```

The output is `linked/browser/game.wasm`, its matching `game.js` and
`memory-config.json`. These files form one build and must be used together with
the browser host, workers, shaders and game data. Linux commands and their
verification status are documented in [BUILD-LINUX.md](BUILD-LINUX.md).

## Tests and technical notes

The browser streaming tests run with Node.js and require no game files:

```sh
node --test tools/test_streaming_worker.cjs
```

After preparing the external inputs, the runtime and build-tool tests include:

```sh
cargo test --workspace -- --test-threads=1
cargo test --manifest-path wasm-imp/Cargo.toml -- --test-threads=1
cargo test --manifest-path linked/Cargo.toml -p engine-imports --test context
cargo test --manifest-path tools/wasm-package/Cargo.toml --bin split_memory
python tools/test_emitter_semantics.py
```

The optional boot-image comparison requires the matching `inputs/va.bin`;
`Prepare-Inputs.ps1 -ReferenceImage` accepts that file. Regenerating translated
bodies with `python tools/retranslate_linked.py` additionally requires the matching
`inputs/game.wat` or a `WAT` environment variable pointing to it.

- [Streaming behavior and measurements](docs/streaming.md)
- [Engine ABI reference](crates/engine-abi/ABI-MAP.md)
- [Map and task-upload changes](UPDATE-2026-11-04.md)
- [GPU readback lifetime](UPDATE-2026-11-04-READBACK.md)
- [Grass resource reuse](UPDATE-2026-11-04-GRASS-POOL.md)
- [SIMD zero-load correction](UPDATE-2026-11-04-SIMD-ZERO-LOADS.md)

Project author: carunodev @stfu. License: [MIT](LICENSE).
