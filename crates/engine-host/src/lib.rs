//! # `engine-host`
//!
//! The browser-free side of the engine ABI.
//!
//! The reconstructed engine (`generated/gen000..gen022`) was lifted from a
//! wasm64 module whose 86 imports were satisfied by JavaScript
//! (`game.js`, `wgpu_worker.js`, `io_worker.js`, `audio-worklet.js`). To run
//! the same machine code in a server process those imports must be backed by
//! Rust. This crate is that boundary: [`Host`] names every dependency the
//! engine actually has, [`HeadlessHost`] implements it with `std` only, and
//! [`browser::BrowserHost`] is the (feature-gated) browser implementation that
//! server builds compile out.
//!
//! ## Import → method map
//!
//! | wasm import (module `env`)      | [`Host`] method            |
//! |---------------------------------|----------------------------|
//! | `wasm_clock_micros`             | [`Host::now_micros`]       |
//! | `wasm_clock_nanos`              | [`Host::now_nanos`]        |
//! | `wasm_module_int`               | [`Host::module_int`]       |
//! | `wasm_httpfs_open` / `_size`    | [`Host::assets`]           |
//! | `wasm_httpfs_pread` (remote)    | [`Host::http`]             |
//! | `wasm_userdata_put_js` etc.     | [`Host::userdata`]         |
//! | `wasm_audio_publish`            | [`Host::audio`]            |
//! | `wasm_input_poll`               | [`Host::input`]            |
//! | `wasm_gpu_ring_submit`          | [`Host::gpu`]              |
//! | `wasm_thread_spawn/_join/_aff`  | [`Host::spawn`] / [`Host::join`] / [`Host::set_affinity`] |
//! | `wasm_sleep` / `wasm_yield`     | [`Host::sleep`] / [`Host::yield_now`] |
//! | `wasm_log`                      | [`Host::log`]              |
//!
//! ## Layout
//!
//! * [`host`] — the trait, thread handles, errors, module-int table.
//! * [`time`] — the monotonic clock that backs the µs/ns imports.
//! * [`fs`] — asset ranges ([`fs::AssetStore`]) and persistent userdata
//!   ([`fs::userdata`]).
//! * [`net`] — HTTP range reads for streamed assets.
//! * [`gpu_sink`] — the shared command ring and the sink that drains it.
//! * [`audio`] — PCM blocks and the audio sink.
//! * [`input`] — input events and the channel-fed source.
//!
//! No module contains global mutable state: every piece of state lives in an
//! owned struct, and the `Host` implementation is passed to the engine rather
//! than reached through a static.

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

pub mod audio;
#[cfg(feature = "browser")]
pub mod browser;
pub mod fs;
pub mod gpu_sink;
pub mod host;
pub mod input;
pub mod net;
pub mod time;

pub use audio::{AudioBlock, AudioSink, NullAudioSink};
pub use fs::userdata::{UserdataEntry, UserdataPage, UserdataStore};
pub use fs::{AssetEntry, AssetId, AssetManifest, AssetStore, DirAssetStore};
pub use gpu_sink::{GpuCommand, GpuRing, GpuSink, NullGpuSink};
pub use host::{
    HeadlessConfig, HeadlessHost, Host, HostError, HostResult, LogLevel, ThreadHandle,
};
pub use input::{ChannelInputSource, InputEvent, InputKind, InputSender, InputSource};
pub use net::{HttpRangeClient, HttpUrl, TcpHttpRangeClient};
pub use time::{Micros, MonotonicClock, Nanos};

/// Test-only helpers: a self-cleaning temp directory. Kept in the crate root
/// so the individual module tests do not each re-implement it.
#[cfg(test)]
pub(crate) mod testing {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    /// A directory under the OS temp dir that deletes itself on drop.
    pub struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        pub fn new(tag: &str) -> std::io::Result<Self> {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0);
            let path = std::env::temp_dir().join(format!(
                "engine-host-{tag}-{}-{nanos}",
                std::process::id()
            ));
            fs::create_dir_all(&path)?;
            Ok(Self { path })
        }

        pub fn path(&self) -> &Path {
            &self.path
        }

        pub fn join(&self, name: &str) -> PathBuf {
            self.path.join(name)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}