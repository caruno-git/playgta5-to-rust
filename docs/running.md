# Running PlayGTA5 to Rust

PlayGTA5 to Rust runs in the browser using WebAssembly and WebGPU. The
[online version](https://gta.caruno.wtf:8443/?mode=sandbox) needs no local server.
The downloadable build includes a Python launcher for local play.

## Requirements

- A recent desktop Chrome or Edge with hardware acceleration and WebGPU support.
- Python 3.10 or newer for the local launcher.
- The compiled runtime archive from
  [Releases](https://github.com/caruno-git/playgta5-to-rust/releases/latest).
- A compatible PlayGTA5 `data/` directory, approximately 21 GB, with
  `manifest.json`, `common/` and `x64/`. The current release expects manifest
  version `1791169379-5814`.

The runtime archive includes the compiled engine, browser scripts, shaders and
loading-screen assets. Original game data is distributed separately through the
[upstream PlayGTA5 project](https://github.com/alihammoud21/playgta5).
Source builds are documented in the [repository README](../README.md#building-from-source).

This is a memory-intensive development build. A minimum supported RAM/GPU
configuration has not yet been established. Startup speed and sustained driving
remain active areas of work; see [Project status](../linked/STATUS.md).

## Linux

Download `PlayGTA5-Rust-Web-2026.11.05.tar.gz` and its adjacent `.sha256` file
into the same directory. Verify and extract the archive:

```bash
sha256sum -c PlayGTA5-Rust-Web-2026.11.05.tar.gz.sha256
tar -xzf PlayGTA5-Rust-Web-2026.11.05.tar.gz
cd PlayGTA5-Rust-Web-2026.11.05
```

Connect the existing game data. Replace the source path below with the directory
that contains `manifest.json`; the destination is inside the extracted package.

```bash
ln -s /absolute/path/to/existing/playgta5/data mirror/playgta5.com/data
python3 verify_release.py
python3 serve_local.py --port 8000
```

Open [localhost:8000](http://localhost:8000/?mode=sandbox) in the browser.
Keep the launcher running while playing. `Ctrl+C` stops it.

## Windows

In PowerShell, extract the downloaded archive and enter its directory:

```powershell
tar -xf .\PlayGTA5-Rust-Web-2026.11.05.tar.gz
Set-Location .\PlayGTA5-Rust-Web-2026.11.05
```

Create a directory junction to the existing game data. Replace the target with
the actual `data` directory:

```powershell
New-Item -ItemType Junction -Path .\mirror\playgta5.com\data -Target 'D:\PlayGTA5\data'
python verify_release.py
python serve_local.py --port 8000
```

Open [localhost:8000](http://localhost:8000/?mode=sandbox). The junction reuses the
existing files without copying the game data. An existing destination named
`data` must be checked before creating another link.

## Verification and common problems

`verify_release.py` verifies the packaged files and checks the external manifest
version when game data is linked. A successful result with
`external data not linked` verifies only the runtime package; game data is still
required before play.

If port 8000 is occupied, start the launcher with `--port 8001` and use the
matching browser address. The supplied server listens on `127.0.0.1`.

Opening `index.html` directly from disk does not provide the HTTP headers needed
by shared WebAssembly memory. Use the launcher. For loading or graphics issues,
record the browser version, loading stage, first console error and failed network
requests when opening an [issue](https://github.com/caruno-git/playgta5-to-rust/issues).
Those details help distinguish missing files from engine, network and GPU problems.

## Hosting on a server

Public hosting requires HTTPS, WebAssembly MIME type `application/wasm` and these
cross-origin-isolation headers:

```http
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
Cross-Origin-Resource-Policy: same-origin
```

Keep each versioned `b/<build>/` directory together with its matching gzip
files. Preserve raw byte-range offsets and HTTP 206/416 responses. A web server
such as Nginx can serve static assets directly while forwarding batch requests
to the existing backend. The [streaming guide](streaming.md#server-configuration)
describes compression and priority behavior.
