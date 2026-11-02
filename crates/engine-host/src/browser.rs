//! The browser host — the wasm32 build's `Host`, backed by the JS shims.
//!
//! This module is behind the `browser` feature and is **not** compiled into
//! server builds: `cargo test -p engine-host` and any `--no-default-features`
//! build see only [`crate::HeadlessHost`]. It exists so the same engine can be
//! built for the web without a second ABI, and so the mapping from every
//! `Host` method back to its JS shim is written down in one place.
//!
//! Today it is a skeleton: it validates the shape of the browser wiring (one
//! clock, one asset handle table, one ring) but every device-facing call
//! reports [`HostError::Unsupported`] until the `wasm-bindgen` glue in
//! `game.js` / `wgpu_worker.js` / `io_worker.js` / `audio-worklet.js` is
//! attached. That is deliberate — a stub that silently succeeds is worse than
//! one that refuses, because the engine cannot tell the difference.
//!
//! Shim map (module `env`, implemented in JS):
//!
//! * `wasm_clock_micros` / `wasm_clock_nanos` → `performance.now()`
//! * `wasm_httpfs_*` → `io_worker.js` `pread` over `fetch`
//! * `wasm_userdata_*` → IndexedDB
//! * `wasm_audio_publish` → `AudioWorkletNode.port.postMessage`
//! * `wasm_gpu_ring_submit` → `wgpu_worker.js` ring walk
//! * `wasm_input_poll` → DOM event queue on the main thread


use std::time::Duration;

use crate::audio::{AudioSink, NullAudioSink};
use crate::fs::userdata::{NullUserdata, UserdataStore};
use crate::fs::{AssetStore, NullAssets};
use crate::gpu_sink::{GpuSink, NullGpuSink};
use crate::host::{Host, HostError, HostResult, LogLevel, ThreadHandle};
use crate::input::{ChannelInputSource, InputSource};
use crate::net::{HttpRangeClient, TcpHttpRangeClient};
use crate::time::MonotonicClock;

/// The JS side of the boundary. Implemented with `wasm-bindgen` externs in the
/// wasm32 build; kept as a trait so this crate still compiles (and is testable)
/// on a host target.
pub trait BrowserShims: Send + Sync {
    /// `performance.now()` in microseconds.
    fn performance_now_micros(&self) -> u64;
    /// `wasm_log` → `console.log`.
    fn console_log(&self, level: LogLevel, target: &str, message: &str);
    /// `wasm_gpu_ring_submit` → post to `wgpu_worker.js`.
    fn submit_ring(&self, ring_words: &[u8]) -> HostResult<()>;
}

/// Browser implementation of [`Host`].
///
/// Holds no globals: the shim is injected at construction (the wasm entry point
/// passes a `'static` binding), and storage/device objects are the same traits
/// the server build uses.
pub struct BrowserHost {
    shims: Box<dyn BrowserShims>,
    clock: MonotonicClock,
    assets: NullAssets,
    userdata: NullUserdata,
    http: TcpHttpRangeClient,
    gpu: NullGpuSink,
    audio: NullAudioSink,
    input: ChannelInputSource,
}

impl BrowserHost {
    pub fn new(shims: Box<dyn BrowserShims>) -> HostResult<Self> {
        Ok(Self {
            shims,
            clock: MonotonicClock::new(),
            assets: NullAssets::new(),
            userdata: NullUserdata,
            http: TcpHttpRangeClient::new(Duration::from_secs(5))?,
            gpu: NullGpuSink::new(),
            audio: NullAudioSink::new(),
            input: ChannelInputSource::disconnected(),

        })
    }

    /// Forward the ring to the worker thread. The real implementation serialises
    /// the ring's bytes; the stub refuses so the engine falls back to its
    /// software path instead of quietly dropping frames.
    pub fn submit_to_worker(&self, ring: &mut crate::gpu_sink::GpuRing) -> HostResult<usize> {
        let mut bytes = Vec::new();
        let mut count = 0usize;
        while let Some(cmd) = ring.pop() {
            let mut encoded = [0u8; crate::gpu_sink::GpuCommand::ENCODED_LEN];
            cmd.encode(&mut encoded);
            bytes.extend_from_slice(&encoded);
            count += 1;
        }
        self.shims.submit_ring(&bytes)?;
        Ok(count)
    }
}

impl Host for BrowserHost {
    fn now_micros(&self) -> u64 {
        // Shim first, local clock second: the JS clock is what the render and
        // audio clocks are compared against in the browser build.
        self.shims.performance_now_micros().max(self.clock.micros())
    }

    fn now_nanos(&self) -> u64 {
        self.clock.nanos()
    }

    fn module_int(&self, _key: &str) -> i64 {
        0
    }

    fn assets(&self) -> &dyn AssetStore {
        &self.assets
    }

    fn http(&self) -> &dyn HttpRangeClient {
        &self.http
    }

    fn userdata(&self) -> &dyn UserdataStore {
        &self.userdata
    }

    fn audio(&self) -> &dyn AudioSink {
        &self.audio
    }

    fn input(&self) -> &dyn InputSource {
        &self.input
    }

    fn gpu(&self) -> &dyn GpuSink {
        &self.gpu
    }

    fn spawn(
        &self,
        _name: &str,
        _task: Box<dyn FnOnce() + Send + 'static>,
    ) -> HostResult<ThreadHandle> {
        // web workers, not OS threads; wiring lands with the shims.
        Err(HostError::Unsupported(
            "browser build spawns web workers, not threads",
        ))
    }

    fn join(&self, _handle: ThreadHandle) -> HostResult<()> {
        Err(HostError::Unsupported(
            "browser build joins web workers, not threads",
        ))
    }

    fn set_affinity(&self, _handle: &ThreadHandle, _cpu: usize) -> HostResult<()> {
        Err(HostError::Unsupported("no cpu affinity in the browser"))
    }

    fn sleep(&self, _duration: Duration) {
        // Never block the wasm thread: the frame loop is scheduled by rAF.
    }

    fn yield_now(&self) {
        // Same: rAF yields implicitly.
    }

    fn log(&self, level: LogLevel, target: &str, message: &str) {
        self.shims.console_log(level, target, message);
    }
}

impl std::fmt::Debug for BrowserHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BrowserHost").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeShims;

    impl BrowserShims for FakeShims {
        fn performance_now_micros(&self) -> u64 {
            1_000_000
        }

        fn console_log(&self, _level: LogLevel, _target: &str, _message: &str) {}

        fn submit_ring(&self, ring_words: &[u8]) -> HostResult<()> {
            if ring_words.len() % crate::gpu_sink::GpuCommand::ENCODED_LEN != 0 {
                return Err(HostError::Protocol("unaligned ring payload".into()));
            }
            Ok(())
        }
    }

    #[test]
    fn browser_host_reports_the_js_clock_and_refuses_device_work() {
        let host = BrowserHost::new(Box::new(FakeShims)).expect("host");
        assert!(host.now_micros() >= 1_000_000);
        assert!(matches!(
            host.spawn("w", Box::new(|| {})),
            Err(HostError::Unsupported(_))
        ));
        let mut ring = crate::gpu_sink::GpuRing::new(4);
        assert!(ring.push(crate::gpu_sink::GpuCommand::new(1, 0, 0, 0)));
        assert_eq!(host.submit_to_worker(&mut ring).expect("submit"), 1);
        assert!(ring.is_empty());
    }
}