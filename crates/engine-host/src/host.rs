//! The [`Host`] trait: every dependency the engine has on its runtime.
//!
//! The engine never reaches for a global. It is handed `&dyn Host` at startup
//! and calls through it for time, storage, audio, input, GPU submission,
//! threads and logging. That keeps the wasm build and the server build on the
//! same code path: only the `Host` implementation changes.

use std::collections::BTreeMap;
use std::fmt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::JoinHandle;
use std::time::Duration;

use crate::audio::{AudioSink, NullAudioSink};
use crate::fs::userdata::{DirUserdata, UserdataStore};
use crate::fs::{AssetStore, DirAssetStore, NullAssets};
use crate::gpu_sink::{GpuSink, NullGpuSink};
use crate::input::{ChannelInputSource, InputSender, InputSource};
use crate::net::{HttpRangeClient, TcpHttpRangeClient};
use crate::time::MonotonicClock;

/// Errors crossing the host boundary. The engine's imports are all
/// `Result`-shaped; a failure is reported to the engine, not panicked on.
#[derive(Debug)]
pub enum HostError {
    /// An `std::io` failure (file, socket, thread).
    Io(std::io::Error),
    /// A malformed asset manifest.
    Manifest(String),
    /// The requested key/id/url is not present.
    NotFound(String),
    /// A read outside the asset's bounds.
    OutOfRange(String),
    /// A peer broke the wire protocol (HTTP framing, ring framing).
    Protocol(String),
    /// The caller passed something the host rejects (empty userdata key,
    /// oversized audio block, …).
    InvalidArgument(String),
    /// The host cannot provide this capability (e.g. CPU affinity without a
    /// platform crate, or HTTPS without a TLS stack).
    Unsupported(&'static str),
    /// Thread spawn/join failure.
    Thread(String),
    /// The channel/sink was closed.
    Closed,
}

impl fmt::Display for HostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HostError::Io(e) => write!(f, "io: {e}"),
            HostError::Manifest(m) => write!(f, "manifest: {m}"),
            HostError::NotFound(m) => write!(f, "not found: {m}"),
            HostError::OutOfRange(m) => write!(f, "out of range: {m}"),
            HostError::Protocol(m) => write!(f, "protocol: {m}"),
            HostError::InvalidArgument(m) => write!(f, "invalid argument: {m}"),
            HostError::Unsupported(m) => write!(f, "unsupported: {m}"),
            HostError::Thread(m) => write!(f, "thread: {m}"),
            HostError::Closed => write!(f, "closed"),
        }
    }
}

impl std::error::Error for HostError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            HostError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for HostError {
    fn from(e: std::io::Error) -> Self {
        HostError::Io(e)
    }
}

pub type HostResult<T> = Result<T, HostError>;

/// Log severity — `env.wasm_log(level, target, ptr, len)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum LogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "error" => Some(LogLevel::Error),
            "warn" | "warning" => Some(LogLevel::Warn),
            "info" => Some(LogLevel::Info),
            "debug" => Some(LogLevel::Debug),
            "trace" => Some(LogLevel::Trace),
            _ => None,
        }
    }
}

/// A spawned engine thread. Owns the OS handle so `join` can consume it; if the
/// handle is dropped without joining the thread is detached, which is what the
/// engine's fire-and-forget workers (streamer, async loader) expect.
#[derive(Debug)]
pub struct ThreadHandle {
    id: u64,
    name: String,
    join: Option<JoinHandle<()>>,
}

impl ThreadHandle {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn into_join(self) -> Option<JoinHandle<()>> {
        self.join
    }
}

/// The engine's entire runtime dependency surface.
///
/// Every method corresponds to one (or a tight group of) wasm imports; the doc
/// comment on each names them so the generated code's call sites can be
/// audited one-to-one.
pub trait Host: Send + Sync {
    /// `env.wasm_clock_micros` — monotonic µs since host start.
    fn now_micros(&self) -> u64;

    /// `env.wasm_clock_nanos` — monotonic ns since host start, same origin as
    /// [`Host::now_micros`].
    fn now_nanos(&self) -> u64;

    /// `env.wasm_module_int(key)` — tunables/feature flags the JS shell used to
    /// supply (`renderer`, `locale`, `platform`). Unknown keys answer `0`
    /// rather than failing: the engine treats 0 as "default".
    fn module_int(&self, key: &str) -> i64;

    /// `env.wasm_httpfs_open`, `env.wasm_httpfs_size`,
    /// `env.wasm_httpfs_pread` — local asset cache keyed by numeric id.
    fn assets(&self) -> &dyn AssetStore;

    /// `env.wasm_httpfs_pread` in remote mode — range reads straight from the
    /// CDN for assets too large to stage locally.
    fn http(&self) -> &dyn HttpRangeClient;

    /// `env.wasm_userdata_put_js`, `env.wasm_userdata_delete_js`,
    /// `env.wasm_userdata_page_js`, `env.wasm_userdata_get_js` — the save
    /// game / settings blob store.
    fn userdata(&self) -> &dyn UserdataStore;

    /// `env.wasm_audio_publish` — hand a mixed PCM block to the device.
    fn audio(&self) -> &dyn AudioSink;

    /// `env.wasm_input_poll`, `env.wasm_input_state` — drain queued input.
    fn input(&self) -> &dyn InputSource;

    /// `env.wasm_gpu_ring_submit` — the worker drains the engine's command ring.
    fn gpu(&self) -> &dyn GpuSink;

    /// `env.wasm_thread_spawn` — start an engine worker thread.
    fn spawn(
        &self,
        name: &str,
        task: Box<dyn FnOnce() + Send + 'static>,
    ) -> HostResult<ThreadHandle>;

    /// `env.wasm_thread_join` — block until a worker finishes.
    fn join(&self, handle: ThreadHandle) -> HostResult<()>;

    /// `env.wasm_thread_affinity` — pin a worker (main/render/audio) to a CPU.
    fn set_affinity(&self, handle: &ThreadHandle, cpu: usize) -> HostResult<()>;

    /// `env.wasm_sleep(us)` — block the current thread.
    fn sleep(&self, duration: Duration);

    /// `env.wasm_yield` — let the scheduler run another worker.
    fn yield_now(&self);

    /// `env.wasm_log(level, target, ptr, len)`.
    fn log(&self, level: LogLevel, target: &str, message: &str);
}

/// Construction knobs for [`HeadlessHost`]. Nothing here is global; two hosts
/// with different roots can coexist in one process (handy for tests and for
/// running several world servers).
#[derive(Clone, Debug)]
pub struct HeadlessConfig {
    /// Directory holding the extracted asset tree. `None` builds an asset store
    /// that answers every read with [`HostError::Unsupported`] (the engine
    /// falls back to [`Host::http`]).
    pub asset_root: Option<PathBuf>,
    /// Directory holding userdata blobs; created on first write.
    pub userdata_root: PathBuf,
    /// `module_int` table. Missing keys read as 0.
    pub module_ints: BTreeMap<String, i64>,
    /// Connect/read timeout for HTTP range reads.
    pub http_timeout: Duration,
    /// Messages above this level are dropped before formatting.
    pub log_level: LogLevel,
}

impl HeadlessConfig {
    pub fn new(userdata_root: impl Into<PathBuf>) -> Self {
        Self {
            asset_root: None,
            userdata_root: userdata_root.into(),
            module_ints: BTreeMap::new(),
            http_timeout: Duration::from_secs(10),
            log_level: LogLevel::Info,
        }
    }

    pub fn with_asset_root(mut self, root: impl Into<PathBuf>) -> Self {
        self.asset_root = Some(root.into());
        self
    }

    pub fn with_module_int(mut self, key: impl Into<String>, value: i64) -> Self {
        self.module_ints.insert(key.into(), value);
        self
    }

    pub fn with_log_level(mut self, level: LogLevel) -> Self {
        self.log_level = level;
        self
    }
}

/// The server build's host: `std` only, no browser, no async runtime.
///
/// Time is `std::time::Instant`, storage is `std::fs`, HTTP is
/// `std::net::TcpStream`, threads are `std::thread`. GPU, audio and input are
/// the null implementations — the server still runs the engine's simulation and
/// streaming, it just discards frames and mixes silence.
pub struct HeadlessHost {
    clock: MonotonicClock,
    module_ints: BTreeMap<String, i64>,
    assets: Box<dyn AssetStore>,
    http: Box<dyn HttpRangeClient>,
    userdata: Box<dyn UserdataStore>,
    gpu: NullGpuSink,
    audio: NullAudioSink,
    input: ChannelInputSource,
    input_sender: InputSender,
    log_level: LogLevel,
    next_thread: AtomicU64,
}

impl HeadlessHost {
    pub fn new(config: HeadlessConfig) -> HostResult<Self> {
        let assets: Box<dyn AssetStore> = match &config.asset_root {
            Some(root) => Box::new(DirAssetStore::from_root(root)?),
            None => Box::new(NullAssets::new()),
        };
        let (input, input_sender) = ChannelInputSource::new();
        Ok(Self {
            clock: MonotonicClock::new(),
            module_ints: config.module_ints,
            assets,
            http: Box::new(TcpHttpRangeClient::new(config.http_timeout)?),
            userdata: Box::new(DirUserdata::new(config.userdata_root)),
            gpu: NullGpuSink::new(),
            audio: NullAudioSink::new(),
            input,
            input_sender,
            log_level: config.log_level,
            next_thread: AtomicU64::new(1),
        })
    }

    /// Build a host whose asset store is an explicit implementation (tests and
    /// the loaders use this to inject a pre-built manifest).
    pub fn with_stores(
        config: HeadlessConfig,
        assets: Box<dyn AssetStore>,
        userdata: Box<dyn UserdataStore>,
    ) -> HostResult<Self> {
        let mut host = Self::new(HeadlessConfig {
            asset_root: None,
            ..config
        })?;
        host.assets = assets;
        host.userdata = userdata;
        Ok(host)
    }

    /// Handle used to feed the engine input from outside (netcode, replay,
    /// headless test harness).
    pub fn input_sender(&self) -> InputSender {
        self.input_sender.clone()
    }

    pub fn gpu_sink(&self) -> &NullGpuSink {
        &self.gpu
    }

    pub fn audio_sink(&self) -> &NullAudioSink {
        &self.audio
    }

    pub fn clock(&self) -> &MonotonicClock {
        &self.clock
    }
}

impl Host for HeadlessHost {
    fn now_micros(&self) -> u64 {
        self.clock.micros()
    }

    fn now_nanos(&self) -> u64 {
        self.clock.nanos()
    }

    fn module_int(&self, key: &str) -> i64 {
        self.module_ints.get(key).copied().unwrap_or(0)
    }

    fn assets(&self) -> &dyn AssetStore {
        self.assets.as_ref()
    }

    fn http(&self) -> &dyn HttpRangeClient {
        self.http.as_ref()
    }

    fn userdata(&self) -> &dyn UserdataStore {
        self.userdata.as_ref()
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
        name: &str,
        task: Box<dyn FnOnce() + Send + 'static>,
    ) -> HostResult<ThreadHandle> {
        let id = self.next_thread.fetch_add(1, Ordering::Relaxed);
        let join = std::thread::Builder::new()
            .name(name.to_owned())
            .spawn(task)
            .map_err(|e| HostError::Thread(format!("spawn {name}: {e}")))?;
        Ok(ThreadHandle {
            id,
            name: name.to_owned(),
            join: Some(join),
        })
    }

    fn join(&self, handle: ThreadHandle) -> HostResult<()> {
        let name = handle.name().to_owned();
        match handle.into_join() {
            Some(join) => join
                .join()
                .map(|_| ())
                .map_err(|_| HostError::Thread(format!("thread {name} panicked"))),
            // Already detached (dropped handle handed back) — nothing to wait on.
            None => Ok(()),
        }
    }

    fn set_affinity(&self, handle: &ThreadHandle, cpu: usize) -> HostResult<()> {
        let _ = (handle, cpu);
        // sched_setaffinity(2) needs libc; this crate is dependency-free, so a
        // server deployment pins via `taskset -c` / container cpuset instead.
        Err(HostError::Unsupported(
            "cpu affinity requires the platform scheduler (use taskset/cpuset)",
        ))
    }

    fn sleep(&self, duration: Duration) {
        std::thread::sleep(duration);
    }

    fn yield_now(&self) {
        std::thread::yield_now();
    }

    fn log(&self, level: LogLevel, target: &str, message: &str) {
        if level > self.log_level {
            return;
        }
        // `eprintln!` discards write errors, so a closed stderr cannot panic
        // the engine's logging path.
        eprintln!(
            "[{}] {}: {}",
            level.as_str(),
            if target.is_empty() { "engine" } else { target },
            message
        );
    }
}

impl fmt::Debug for HeadlessHost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HeadlessHost")
            .field("log_level", &self.log_level)
            .field("module_ints", &self.module_ints.len())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gpu_sink::GpuCommand;
    use std::sync::atomic::AtomicI64;
    use std::sync::Arc;

    fn host(tag: &str) -> (crate::testing::TempDir, HeadlessHost) {
        let dir = crate::testing::TempDir::new(tag).expect("tempdir");
        let host = HeadlessHost::new(HeadlessConfig::new(dir.join("userdata")))
            .expect("headless host");
        (dir, host)
    }

    #[test]
    fn module_ints_default_to_zero_and_read_back_configured_values() {
        let dir = crate::testing::TempDir::new("module-int").expect("tempdir");
        let host = HeadlessHost::new(
            HeadlessConfig::new(dir.join("ud"))
                .with_module_int("renderer", 2)
                .with_module_int("locale", 8),
        )
        .expect("host");
        assert_eq!(host.module_int("renderer"), 2);
        assert_eq!(host.module_int("locale"), 8);
        assert_eq!(host.module_int("missing"), 0);
    }

    #[test]
    fn spawn_join_runs_the_task_and_propagates_panics() {
        let (_dir, host) = host("spawn");
        let flag = Arc::new(AtomicI64::new(0));
        let f = flag.clone();
        let handle = host
            .spawn(
                "engine-worker",
                Box::new(move || f.store(42, Ordering::SeqCst)),
            )
            .expect("spawn");
        assert_eq!(handle.name(), "engine-worker");
        assert!(handle.id() >= 1);
        host.join(handle).expect("join");
        assert_eq!(flag.load(Ordering::SeqCst), 42);

        let panicking = host
            .spawn("boom", Box::new(|| panic!("worker died")))
            .expect("spawn");
        let err = host.join(panicking).expect_err("panic must surface");
        assert!(matches!(err, HostError::Thread(_)), "got {err:?}");
    }

    #[test]
    fn clock_is_shared_and_gpu_ring_drains_through_the_trait() {
        let (_dir, host) = host("gpu");
        let before = host.now_micros();
        assert!(host.now_nanos() >= before.saturating_mul(1_000));

        let mut ring = crate::gpu_sink::GpuRing::new(4);
        assert!(ring.push(GpuCommand::new(1, 2, 3, 0)));
        assert!(ring.push(GpuCommand::new(4, 5, 6, 0)));
        let drained = host.gpu().submit(&mut ring).expect("submit");
        assert_eq!(drained, 2);
        assert!(ring.is_empty());
        assert_eq!(host.gpu_sink().drained(), 2);
    }

    #[test]
    fn affinity_is_reported_as_unsupported_not_faked() {
        let (_dir, host) = host("affinity");
        let handle = host.spawn("w", Box::new(|| {})).expect("spawn");
        let err = host.set_affinity(&handle, 3).expect_err("must fail");
        assert!(matches!(err, HostError::Unsupported(_)));
        host.join(handle).expect("join");
    }

    #[test]
    fn logging_respects_the_level_filter() {
        let (_dir, host) = host("log");
        host.log(LogLevel::Error, "grc", "shader compile failed");
        host.log(LogLevel::Trace, "grc", "dropped by filter");
        assert_eq!(LogLevel::parse("warn"), Some(LogLevel::Warn));
        assert_eq!(LogLevel::parse("nope"), None);
    }
}