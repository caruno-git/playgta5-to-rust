# Linux Build

Supply the original inputs described in `README.md`. They are not distributed
with this source snapshot. Rust 1.99.1 and its matching sources were used for
the verified Windows-hosted WASM64 build; a Linux-hosted build is not yet tested.

```bash
mkdir -p inputs
cp /path/to/original/game.wasm inputs/game.wasm
cp /path/to/original/game.js inputs/game.js
printf '%s  %s\n' 11ca8d2c04c5e843d18ff4aea4899d72c86973c6b031df334e67c446b2ae83e0 inputs/game.wasm | sha256sum -c -
python3 tools/w2r/gen_segments.py inputs/game.wasm wasm-rt/src
rustup component add rust-src
npm ci --prefix tools

export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR="$PWD/linked/target-wasm64"
export RUSTC_BOOTSTRAP=1
export RUSTFLAGS='-Aexplicit_builtin_cfgs_in_flags --cfg target_feature="atomics" -C target-feature=+atomics,+bulk-memory,+mutable-globals,+simd128 -C link-arg=--shared-memory -C link-arg=--import-memory -C link-arg=--export-table -C link-arg=--export=__stack_pointer -C link-arg=-zstack-size=16777216'
cargo build --release --manifest-path linked/Cargo.toml -p engine-linked --lib --target wasm64-unknown-unknown -Z build-std=std,panic_abort

artifact="$CARGO_TARGET_DIR/wasm64-unknown-unknown/release/engine_linked.wasm"
unset RUSTFLAGS
export CARGO_TARGET_DIR="$PWD/linked/target-tools"
mkdir -p linked/browser
cargo run --release --manifest-path tools/wasm-package/Cargo.toml --bin split_memory -- "$artifact" linked/browser/game.wasm
cp linked/browser/game.memory.json linked/browser/memory-config.json
node tools/browser_adapter.cjs
```

Use the generated `game.wasm` and `game.js` together with the existing JS loader,
workers, shaders and game assets. Shared WASM64 memory requires the original
cross-origin-isolation HTTP headers and a capable browser. This does not produce
a native ELF game server.
