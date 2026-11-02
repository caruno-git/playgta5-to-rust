//! Runner: boot image → fixed number of ticks → digest.

use engine_runtime::{Clock, TickConfig};

use super::vm::{process_nonce, StateDigest, Vm};
use super::workloads;

/// Alias used in the report tables.
pub type Digest = StateDigest;

/// What a tick may observe. Mirrors the runtime's `TickCtx` without exposing
/// the thread pool (these workloads are single-threaded by design).
#[derive(Debug, Clone, Copy)]
pub struct TickInfo {
    /// 1-based tick index.
    pub index: u64,
    /// Engine frame counter, identical to `index`.
    pub frame: i32,
    /// Fixed step: never the measured elapsed time.
    pub delta_ns: u64,
    /// Virtual monotonic time at the start of the tick.
    pub now_ns: u64,
}

/// A deterministic unit of work driven once per tick.
pub trait Workload {
    /// Stable name used in the report.
    fn name(&self) -> &'static str;
    /// Recovered wasm function id, when this workload stands in for one.
    fn fn_id(&self) -> Option<u32>;
    /// Execute one tick.
    fn tick(&mut self, vm: &mut Vm, info: &TickInfo);
}

/// Closure-backed workload — the "supplied workload" path.
pub struct FnWorkload<F> {
    name: &'static str,
    fn_id: Option<u32>,
    body: F,
}

impl<F> FnWorkload<F>
where
    F: FnMut(&mut Vm, &TickInfo),
{
    /// Wrap a closure.
    pub fn new(name: &'static str, fn_id: Option<u32>, body: F) -> Self {
        Self { name, fn_id, body }
    }
}

impl<F> Workload for FnWorkload<F>
where
    F: FnMut(&mut Vm, &TickInfo),
{
    fn name(&self) -> &'static str {
        self.name
    }

    fn fn_id(&self) -> Option<u32> {
        self.fn_id
    }

    fn tick(&mut self, vm: &mut Vm, info: &TickInfo) {
        (self.body)(vm, info)
    }
}

/// The repeatable built-in workloads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmFn {
    /// Integer/float arithmetic, rotates, division, clz/ctz.
    Arithmetic = 0x101,
    /// Loads/stores, unaligned access, memcpy/memset, page crossing.
    Memory = 0x102,
    /// v128 helpers: add/mul/shuffle/dot/bitselect.
    Simd = 0x103,
    /// Cooperative futex wait/wake/requeue/expire.
    Futex = 0x104,
    /// All four of the above in one tick.
    Mixed = 0x105,
    /// KNOWN NONDETERMINISTIC control: reads the wall clock.
    WallClock = 0x106,
}

impl WasmFn {
    /// Recovered-style function id used in the report.
    pub const fn fn_id(self) -> u32 {
        self as u32
    }

    /// Stable name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Arithmetic => "arith",
            Self::Memory => "memory",
            Self::Simd => "simd",
            Self::Futex => "futex",
            Self::Mixed => "mixed",
            Self::WallClock => "wall_clock_control",
        }
    }

    /// Subsystem label for the verdict table.
    pub const fn subsystem(self) -> &'static str {
        match self {
            Self::Arithmetic => "integer/fp arithmetic",
            Self::Memory => "linear memory",
            Self::Simd => "v128 SIMD helpers",
            Self::Futex => "futex path",
            Self::Mixed => "all of the above",
            Self::WallClock => "wall clock (control)",
        }
    }

    /// True for workloads expected to reproduce.
    pub const fn expected_deterministic(self) -> bool {
        !matches!(self, Self::WallClock)
    }

    /// Run one tick of this built-in.
    pub fn tick(self, vm: &mut Vm, info: &TickInfo) {
        match self {
            Self::Arithmetic => workloads::arithmetic(vm, info),
            Self::Memory => workloads::memory(vm, info),
            Self::Simd => workloads::simd(vm, info),
            Self::Futex => workloads::futex(vm, info),
            Self::WallClock => workloads::wall_clock(vm, info),
            Self::Mixed => {
                workloads::arithmetic(vm, info);
                workloads::memory(vm, info);
                workloads::simd(vm, info);
                workloads::futex(vm, info);
            }
        }
    }
}

/// How a [`Spec`] produces ticks.
pub enum WorkloadKind {
    /// One of the repeatable built-ins, selected by wasm fn id.
    Builtin(WasmFn),
    /// Arbitrary caller closure.
    Closure(Box<dyn FnMut(&mut Vm, &TickInfo)>),
}

/// A named workload plus provenance.
pub struct Spec {
    /// Report name.
    pub name: &'static str,
    /// Function id, if any.
    pub fn_id: Option<u32>,
    /// Execution strategy.
    pub kind: WorkloadKind,
}

impl Spec {
    /// Built-in by function id.
    pub fn builtin(func: WasmFn) -> Self {
        Self {
            name: func.name(),
            fn_id: Some(func.fn_id()),
            kind: WorkloadKind::Builtin(func),
        }
    }

    /// Built-in by numeric id; `None` for an unknown id.
    pub fn from_fn_id(id: u32) -> Option<Self> {
        let func = match id {
            0x101 => WasmFn::Arithmetic,
            0x102 => WasmFn::Memory,
            0x103 => WasmFn::Simd,
            0x104 => WasmFn::Futex,
            0x105 => WasmFn::Mixed,
            0x106 => WasmFn::WallClock,
            _ => return None,
        };
        Some(Self::builtin(func))
    }

    /// Closure workload.
    pub fn closure<F>(name: &'static str, body: F) -> Self
    where
        F: FnMut(&mut Vm, &TickInfo) + 'static,
    {
        Self {
            name,
            fn_id: None,
            kind: WorkloadKind::Closure(Box::new(body)),
        }
    }

    /// Subsystem label for the report.
    pub fn subsystem(&self) -> &'static str {
        match &self.kind {
            WorkloadKind::Builtin(f) => f.subsystem(),
            WorkloadKind::Closure(_) => "custom closure",
        }
    }

    /// Expected reproducibility.
    pub fn expected_deterministic(&self) -> bool {
        match &self.kind {
            WorkloadKind::Builtin(f) => f.expected_deterministic(),
            WorkloadKind::Closure(_) => true,
        }
    }
}

impl Workload for Spec {
    fn name(&self) -> &'static str {
        self.name
    }

    fn fn_id(&self) -> Option<u32> {
        self.fn_id
    }

    fn tick(&mut self, vm: &mut Vm, info: &TickInfo) {
        match &mut self.kind {
            WorkloadKind::Builtin(f) => f.tick(vm, info),
            WorkloadKind::Closure(body) => body(vm, info),
        }
    }
}

/// Tick count and fixed step.
#[derive(Debug, Clone, Copy)]
pub struct RunConfig {
    /// Number of ticks to execute.
    pub ticks: u64,
    /// Fixed step in nanoseconds.
    pub step_ns: u64,
}

impl Default for RunConfig {
    fn default() -> Self {
        Self {
            ticks: 300,
            step_ns: TickConfig::default().step_ns(),
        }
    }
}

/// Result of one run.
#[derive(Debug, Clone)]
pub struct RunRecord {
    /// Workload name.
    pub name: &'static str,
    /// Function id, if any.
    pub fn_id: Option<u32>,
    /// Ticks executed.
    pub ticks: u64,
    /// Final engine frame.
    pub frame: i32,
    /// Allocated pages at the end.
    pub pages: usize,
    /// Futex wake/timeout events at the end.
    pub futex_events: usize,
    /// Final state digest.
    pub digest: Digest,
}

/// Boot a fresh image and execute `cfg.ticks` ticks of `workload`.
///
/// Each run gets a fresh [`Vm`] and a fresh [`Clock`]; the only per-run input
/// outside the workload is the nonce written to the excluded pages.
pub fn run_workload<W: Workload + ?Sized>(workload: &mut W, cfg: RunConfig) -> RunRecord {
    let mut vm = Vm::boot(process_nonce());
    let mut clock = Clock::new();
    for _ in 0..cfg.ticks {
        clock.step(cfg.step_ns);
        vm.frame = clock.frame();
        vm.tick = clock.ticks();
        let info = TickInfo {
            index: clock.ticks(),
            frame: clock.frame(),
            delta_ns: clock.delta_ns(),
            now_ns: clock.now_ns(),
        };
        workload.tick(&mut vm, &info);
    }
    RunRecord {
        name: workload.name(),
        fn_id: workload.fn_id(),
        ticks: clock.ticks(),
        frame: clock.frame(),
        pages: vm.allocated_pages(),
        futex_events: vm.futex.wake_events(),
        digest: vm.digest(),
    }
}

/// Convenience: run a [`Spec`].
pub fn run_spec(spec: &mut Spec, cfg: RunConfig) -> RunRecord {
    run_workload(spec, cfg)
}
