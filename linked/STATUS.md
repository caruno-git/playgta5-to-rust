# Release status

Updated: 5 November 2026.

PlayGTA5-to-Rust is a browser engine port with a published development build.
The engine runs as WebAssembly, with WebGPU rendering and browser workers for
file streaming. It has reached playable scenes with the original game data;
loading performance and long-session stability still have known limitations.

[Play online](https://gta.caruno.wtf:8443/?mode=sandbox) ·
[Download the build](https://github.com/caruno-git/playgta5-to-rust/releases/tag/v2026.11.05-rust-web) ·
[Setup and source build instructions](../README.md)

## Current release

| Item | Version |
| --- | --- |
| Release | `v2026.11.05-rust-web` |
| Runtime source commit | `55f0736a7555977c79421c426cf890b5317b7351` |
| Browser build directory | `b/55f0736a75/` |
| Compiled engine | `game.wasm`, 95,868,954 bytes |
| Download archive | `PlayGTA5-Rust-Web-2026.11.05.tar.gz`, 97,789,262 bytes |

Engine SHA-256:

```text
a4922b4b6e45fb4ace72f5ab0154fe2c611b8051bca8f08f6c3baf9575138dbe
```

The archive includes the compiled engine, its matching browser adapter, GPU and
IO workers, loader, shaders, loading-screen assets and a Python local launcher.
Its `release.json` identifies every packaged file. The included verifier and
published SHA-256 file check the package contents and download respectively.

The original game data is a separate requirement: approximately 21 GB, manifest
version `1791169379-5814`. The package can reuse an existing `data/` directory.
The compiled runtime is available from Releases; the Git source checkout
contains the translation, runtime and build tools.

## Verified behavior

| Area | Evidence and scope |
| --- | --- |
| Engine execution | Constructors, main, scripts, the game loop, keyboard movement and WebGPU world rendering have executed with the original assets. |
| Cloth geometry | The diagnosed Michael shirt mesh has finite native and GPU cloth positions after the SIMD zero-load correction. Visual checks show the complete shirt. This does not establish identical appearance for every outfit. |
| Map | The native full map and zoom passed a visual check at 2560×1600. |
| Window resizing | The current engine completed six fullscreen cycles and 18 canvas-size changes while continuing to render and move the player, with no recorded engine panic, GPU validation error or HTTP error. |
| Streaming transport | Exact byte-range reads, persistent-cache indexing and demand prioritization passed worker tests and real HTTP/2 transfer checks. See [streaming behavior](../docs/streaming.md). |
| Release package | All 8,486 packaged files passed integrity checks. The supplied Python launcher passed HTTP checks for the entry page, runtime files, shaders, isolation headers, WASM MIME type and byte ranges. |
| Hosted files | The main site serves the release under its new build directory. Public checks passed for code hashes, gzip, metadata and raw byte-range responses. |

These checks cover specific workloads. Package and HTTP checks establish file
integrity and delivery; gameplay checks establish the behavior observed during
those sessions.

## Known limitations

- **Cold loading can be slow.** The first check after deployment to the main
  site did not reach gameplay within five minutes. It remained at 66%,
  “Loading audio metadata,” without a recorded engine crash or HTTP error.
  Earlier runs of the same engine reached gameplay with cached resources.
- **Driving and long-session stability are not fully validated.** The streaming
  change improves priority for blocking reads, but does not establish that all
  driving freezes have been resolved.
- **Character switching remains inconsistent.** One tested transition to Trevor
  did not finish within 90 seconds. Franklin and Michael reached gameplay in
  that sequence.
- **Memory use is substantial.** The guest memory starts at 3 GiB, with additional
  memory required by the browser, translated runtime and GPU. Some longer tests
  stopped when host free memory reached the test harness's 512 MiB guard.
- **Performance varies with the browser, hardware and cache state.** Some
  diagnostic sessions used fewer reported CPU cores or the baseline WASM
  compiler. Their startup times and FPS do not represent default browser
  performance. No general FPS improvement is claimed for the current release.

## Changes included in this release

- [SIMD zero-extending loads and cloth geometry](../UPDATE-2026-11-04-SIMD-ZERO-LOADS.md)
- [GPU readback lifetime during resource destruction](../UPDATE-2026-11-04-READBACK.md)
- [Reuse of retired grass buffers](../UPDATE-2026-11-04-GRASS-POOL.md)
- [Map, resize and renderer corrections](../UPDATE-2026-11-04.md)
- [Browser streaming priorities and metadata compression](../docs/streaming.md)

The dated change notes describe the artifacts and checks associated with each
update. The version table above identifies the current downloadable runtime.
