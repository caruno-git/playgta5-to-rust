//! Process-global state for the ABI shims.
//!
//! The engine's wasm imports are free functions: the generated code calls
//! `env.wasm_now_ms()` with no context argument, so *something* has to hold the
//! [`Host`] and the linear-memory base. That something is this module, and it is
//! the only mutable global in the crate.
//!
//! Design constraints:
//!
//! * A missing host is never a panic. Every shim answers its documented
//!   "no host" default, because the engine may run its init sequence before the
//!   server installs a host.
//! * Two hosts can be installed in a nested (stack) fashion — [`install`]
//!   returns a guard that restores the previous host on drop, which is what the
//!   integration tests rely on.
//! * The hot path (`wasm_now_ms`, 51 static call sites in the engine) reads a
//!   single atomic and never touches a lock.

use std::path::PathBuf;
use std::ptr;
use std::sync::atomic::{AtomicPtr, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use engine_host::Host;

/// The installed host, as a raw pointer to a leaked `Arc`.
///
/// A fat pointer (`Arc<dyn Host>`) cannot live in an `AtomicPtr<()>` without
/// splitting the vtable out, so the pointer stored here is to a *boxed* `Arc`.
/// The box is owned by whichever [`InstallGuard`] installed it.
static HOST: AtomicPtr<Arc<dyn Host>> = AtomicPtr::new(ptr::null_mut());

/// Base address of the engine's linear memory, `0` when not installed.
static MEM_BASE: AtomicUsize = AtomicUsize::new(0);
/// Length of that memory reservation, `0` when not installed.
static MEM_LEN: AtomicUsize = AtomicUsize::new(0);
/// Whether [`set_memory`] has been called (a base of `0` is legal).
static MEM_SET: AtomicUsize = AtomicUsize::new(0);

/// Root that `/`-absolute engine paths resolve against (`/assets/x` →
/// `<root>/assets/x`). `None` keeps the process working directory.
static FS_ROOT: OnceLock<Mutex<PathBuf>> = OnceLock::new();

/// Callback installed to observe `env.exit` / `wasi.proc_exit`.
pub type ExitHook = Box<dyn Fn(u32) + Send + Sync>;

/// Interceptor for `env.exit` / `wasi.proc_exit`, so a test process does not
/// have to die to observe an exit request.
static EXIT_HOOK: OnceLock<Mutex<Option<ExitHook>>> = OnceLock::new();

/// Counters for imports whose effect is intentionally dropped, so tests and
/// operators can see that the engine did call them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Counter {
    /// `env.wgpu_start_worker`
    WgpuStartWorker = 0,
    /// `env.wasm_input_publish_js`
    InputPublish,
    /// `env.emscripten_asm_const_int`
    AsmConst,
    /// `env.emscripten_asm_const_int_sync_on_main_thread`
    AsmConstSync,
    /// `env._emscripten_notify_mailbox_postmessage`
    MailboxPost,
    /// `env._emscripten_receive_on_main_thread_js`
    MainThreadCall,
    /// `env.emscripten_stack_snapshot`
    StackSnapshot,
    /// `env.wasm_hang_line_js`
    HangLine,
    /// `env.wasm_httpfs_io_start_js`
    IoStart,
    /// `env._emscripten_thread_mailbox_await`
    MailboxAwait,
    /// Number of counters; keep last.
    Count,
}

/// Number of [`Counter`] variants.
pub const COUNTER_COUNT: usize = Counter::Count as usize;

static COUNTERS: [AtomicU64; COUNTER_COUNT] = [
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
];

/// Increment a counter and return its new value.
pub fn bump(counter: Counter) -> u64 {
    COUNTERS[counter as usize].fetch_add(1, Ordering::Relaxed) + 1
}

/// Current value of a counter.
pub fn count(counter: Counter) -> u64 {
    COUNTERS[counter as usize].load(Ordering::Relaxed)
}

/// Guard returned by [`install`]; restores the previous host when dropped.
#[derive(Debug)]
pub struct InstallGuard {
    own: *mut Arc<dyn Host>,
    prev: *mut Arc<dyn Host>,
}

impl InstallGuard {
    /// The host this guard installed.
    pub fn host(&self) -> &Arc<dyn Host> {
        // SAFETY: `own` was created by `Box::into_raw` in `install` and is only
        // freed in this guard's `Drop`, so it is valid for the guard's lifetime.
        unsafe { &*self.own }
    }
}

impl Drop for InstallGuard {
    fn drop(&mut self) {
        // Only uninstall if nobody installed over us; otherwise leave the newer
        // host in place and just release our own reference.
        if HOST.compare_exchange(
            self.own,
            self.prev,
            Ordering::AcqRel,
            Ordering::Acquire,
        ).is_ok() { crate::httpfs::stop_io_worker(); }
        // SAFETY: `own` came from `Box::into_raw` and is dropped exactly once.
        unsafe {
            drop(Box::from_raw(self.own));
        }
    }
}

/// Install `host` as the process host, returning a guard that restores the
/// previous host (usually `None`) on drop.
pub fn install(host: Arc<dyn Host>) -> InstallGuard {
    let boxed = Box::into_raw(Box::new(host));
    let prev = HOST.swap(boxed, Ordering::AcqRel);
    InstallGuard { own: boxed, prev }
}

/// The installed host, if any.
///
/// The returned reference is valid until the installing [`InstallGuard`] is
/// dropped; every ABI shim only uses it for the duration of one call.
pub fn host() -> Option<&'static dyn Host> {
    let raw = HOST.load(Ordering::Acquire);
    if raw.is_null() {
        return None;
    }
    // SAFETY: non-null implies a live `Arc` owned by an `InstallGuard`.
    Some(unsafe { &**raw })
}

/// Owned handle to the installed host (cheap: one atomic increment).
pub fn host_arc() -> Option<Arc<dyn Host>> {
    let raw = HOST.load(Ordering::Acquire);
    if raw.is_null() {
        return None;
    }
    // SAFETY: as above — cloning is safe because the `Arc` is alive.
    Some(unsafe { (*raw).clone() })
}

/// Whether a host is installed.
pub fn is_installed() -> bool {
    !HOST.load(Ordering::Acquire).is_null()
}

/// Publish the engine's linear memory so pointers can be resolved.
///
/// # Safety
///
/// `base` must point to `len` readable and writable bytes that stay alive (and
/// at a fixed address) for as long as the engine runs, exactly like a wasm
/// memory reservation. The engine is free to alias those bytes from any thread,
/// which is why the accessors in [`crate::mem`] are `unsafe`-adjacent by nature.
pub unsafe fn set_memory(base: *mut u8, len: usize) {
    crate::httpfs::stop_io_worker();
    MEM_BASE.store(base as usize, Ordering::Release);
    MEM_LEN.store(len, Ordering::Release);
    MEM_SET.store(1, Ordering::Release);
}

/// Total size of the installed linear memory, `0` when none is installed.
pub fn memory_len() -> usize {
    if MEM_SET.load(Ordering::Acquire) == 0 {
        0
    } else {
        MEM_LEN.load(Ordering::Acquire)
    }
}

/// Base address of the installed linear memory, `0` when none is installed.
pub fn memory_base() -> usize {
    if MEM_SET.load(Ordering::Acquire) == 0 {
        0
    } else {
        MEM_BASE.load(Ordering::Acquire)
    }
}

/// Whether [`set_memory`] has been called.
pub fn memory_installed() -> bool {
    MEM_SET.load(Ordering::Acquire) != 0
}

/// Drop the linear-memory mapping (the engine has exited).
pub fn clear_memory() {
    crate::httpfs::stop_io_worker();
    MEM_BASE.store(0, Ordering::Release);
    MEM_LEN.store(0, Ordering::Release);
    MEM_SET.store(0, Ordering::Release);
}

/// Root that absolute engine paths resolve against.
pub fn fs_root() -> PathBuf {
    match FS_ROOT.get_or_init(|| Mutex::new(PathBuf::from("."))).lock() {
        Ok(p) => p.clone(),
        Err(poisoned) => poisoned.into_inner().clone(),
    }
}

/// Set the root that absolute engine paths resolve against.
pub fn set_fs_root(root: impl Into<PathBuf>) {
    let slot = FS_ROOT.get_or_init(|| Mutex::new(PathBuf::from(".")));
    match slot.lock() {
        Ok(mut p) => *p = root.into(),
        Err(poisoned) => *poisoned.into_inner() = root.into(),
    }
}

/// Install a hook that receives `env.exit` / `wasi.proc_exit` codes instead of
/// the process exiting. Passing `None` restores the default (`std::process::exit`).
pub fn set_exit_hook(hook: Option<ExitHook>) {
    let slot = EXIT_HOOK.get_or_init(|| Mutex::new(None));
    match slot.lock() {
        Ok(mut h) => *h = hook,
        Err(poisoned) => *poisoned.into_inner() = hook,
    }
}

/// Whether an exit hook is installed.
pub fn has_exit_hook() -> bool {
    match EXIT_HOOK.get_or_init(|| Mutex::new(None)).lock() {
        Ok(h) => h.is_some(),
        Err(poisoned) => poisoned.into_inner().is_some(),
    }
}

/// Deliver an exit code to the hook; returns `false` when there is no hook and
/// the caller must exit the process itself.
pub fn dispatch_exit(code: u32) -> bool {
    let slot = EXIT_HOOK.get_or_init(|| Mutex::new(None));
    match slot.lock() {
        Ok(h) => match h.as_ref() {
            Some(f) => {
                f(code);
                true
            }
            None => false,
        },
        Err(poisoned) => match poisoned.into_inner().as_ref() {
            Some(f) => {
                f(code);
                true
            }
            None => false,
        },
    }
}
