//! The driver: `run(host, tick_fn)`.
//!
//! [`Host`] bundles everything a tick is allowed to touch — [`Clock`],
//! [`Scheduler`], [`ThreadPool`] and the stop flag — and [`run`] is the loop
//! that owns them. This is the server-side replacement for the shipped loop
//! (`Main_OneLoopIteration`, func_idx 106 + the `CSystem::EndRender` pacer).
//!
//! The one place wall-clock time is allowed: [`TimeSource`]. The OS-backed
//! implementation ([`OsTime`]) reads `Instant` *between* ticks only, to decide
//! how much elapsed time to feed the accumulator. Nothing inside a tick can see
//! it, and tests use [`VirtualTime`] so the driver itself is fully
//! reproducible.

use std::time::{Duration, Instant};

use crate::clock::Clock;
use crate::scheduler::{Scheduler, TickConfig, TickConfigError};
use crate::threadpool::ThreadPool;

/// Where elapsed time comes from. Implementations are consulted only between
/// ticks.
pub trait TimeSource {
    /// Monotonic nanoseconds since an arbitrary epoch.
    fn now_ns(&mut self) -> u64;

    /// Called by the driver on a pump that produced no tick, so the loop does
    /// not spin. For a virtual source this advances the simulated time.
    fn idle(&mut self);
}

/// Fully deterministic time source: time moves only when [`VirtualTime::idle`]
/// is called (or explicitly via [`VirtualTime::advance`]).
#[derive(Debug, Clone)]
pub struct VirtualTime {
    now_ns: u64,
    pump_ns: u64,
}

impl VirtualTime {
    /// A virtual clock that advances by `pump_ns` each time the driver idles.
    ///
    /// Typical choice for tests is 1 ms, so the driver pumps ~60 times per
    /// simulated second.
    pub const fn new(pump_ns: u64) -> Self {
        Self { now_ns: 0, pump_ns }
    }

    /// Jump the virtual clock forward.
    pub fn advance(&mut self, ns: u64) {
        self.now_ns = self.now_ns.saturating_add(ns);
    }

    /// Step per idle.
    pub const fn pump_ns(&self) -> u64 {
        self.pump_ns
    }
}

impl TimeSource for VirtualTime {
    fn now_ns(&mut self) -> u64 {
        self.now_ns
    }

    fn idle(&mut self) {
        self.now_ns = self.now_ns.saturating_add(self.pump_ns);
    }
}

/// `std::time::Instant`-backed source — the only wall-clock dependency in the
/// crate, and it lives strictly outside the tick boundary.
#[derive(Debug)]
pub struct OsTime {
    start: Instant,
}

impl Default for OsTime {
    fn default() -> Self {
        Self::new()
    }
}

impl OsTime {
    /// Start measuring from now.
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl TimeSource for OsTime {
    fn now_ns(&mut self) -> u64 {
        self.start.elapsed().as_nanos() as u64
    }

    fn idle(&mut self) {
        // 1 ms is below the 16.67 ms step, so this loses no precision; it stops
        // a busy spin when the loop is waiting for the next due tick.
        std::thread::sleep(Duration::from_millis(1));
    }
}

/// Stop flag shared with the tick body and with external control threads.
#[derive(Debug, Clone, Default)]
pub struct RunControl {
    flag: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl RunControl {
    /// A fresh flag, not stopped.
    pub fn new() -> Self {
        Self::default()
    }

    /// Request the driver to stop after the current tick.
    pub fn stop(&self) {
        self.flag.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    /// True once [`RunControl::stop`] has been called.
    pub fn is_stopped(&self) -> bool {
        self.flag.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// What a tick may observe besides the clock.
#[derive(Debug)]
pub struct TickCtx<'a> {
    /// Worker pool, already in the mode chosen at construction.
    pub pool: &'a ThreadPool,
    /// Stop flag; a tick may call [`RunControl::stop`] for a bounded run.
    pub control: &'a RunControl,
    /// 1-based tick index, identical to `clock.frame()`.
    pub tick_index: u64,
    /// Fixed step in nanoseconds — the only legal delta.
    pub step_ns: u64,
    /// Frame number at tick start (same as `clock.frame()`).
    pub frame: i32,
}

/// Outcome of a [`run`] call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RunStats {
    /// Ticks (frames) executed.
    pub ticks: u64,
    /// Final frame counter, as `GetGameFrame` would report it.
    pub frame: i32,
    /// Time discarded by the slow-tick policy.
    pub dropped_ns: u64,
    /// Pumps that hit the catch-up cap.
    pub overruns: u64,
    /// True when [`crate::SlowTickPolicy::Halt`] stopped the loop.
    pub halted: bool,
}

/// Clock + scheduler + job pool + stop flag, i.e. one engine process.
pub struct Host {
    /// Virtual frame clock.
    pub clock: Clock,
    /// Fixed-timestep accumulator.
    pub scheduler: Scheduler,
    /// Job pool shim.
    pub pool: ThreadPool,
    time: Box<dyn TimeSource + Send>,
    control: RunControl,
}

impl Host {
    /// Assemble a host.
    ///
    /// # Errors
    /// Propagates [`TickConfigError`] for an invalid rate or cap.
    pub fn new(
        cfg: TickConfig,
        pool: ThreadPool,
        time: Box<dyn TimeSource + Send>,
    ) -> Result<Self, TickConfigError> {
        Ok(Self {
            clock: Clock::new(),
            scheduler: Scheduler::new(cfg)?,
            pool,
            time,
            control: RunControl::new(),
        })
    }

    /// Host driven by a [`VirtualTime`] source — the deterministic default.
    ///
    /// # Errors
    /// Propagates [`TickConfigError`].
    pub fn with_virtual_time(
        cfg: TickConfig,
        pool: ThreadPool,
        pump_ns: u64,
    ) -> Result<Self, TickConfigError> {
        Self::new(cfg, pool, Box::new(VirtualTime::new(pump_ns)))
    }

    /// Host driven by the OS clock (production).
    ///
    /// # Errors
    /// Propagates [`TickConfigError`].
    pub fn with_os_time(cfg: TickConfig, pool: ThreadPool) -> Result<Self, TickConfigError> {
        Self::new(cfg, pool, Box::new(OsTime::new()))
    }

    /// The stop flag.
    pub const fn control(&self) -> &RunControl {
        &self.control
    }

    /// Ask the driver to stop.
    pub fn stop(&self) {
        self.control.stop();
    }
}

/// Drive `host` until its stop flag is set (or a `Halt` overrun fires).
///
/// `tick_fn` is called exactly once per simulated frame with the clock already
/// advanced into that frame, so `clock.frame()` is 1-based and
/// `clock.delta_ns()` equals the fixed step on every single call.
pub fn run<F>(host: &mut Host, mut tick_fn: F) -> RunStats
where
    F: FnMut(&mut Clock, &TickCtx<'_>),
{
    let step_ns = host.scheduler.step_ns();
    let mut last = host.time.now_ns();

    loop {
        if host.control.is_stopped() {
            break;
        }

        let now = host.time.now_ns();
        let elapsed = now.saturating_sub(last);
        last = now;

        let report = {
            let Host {
                clock,
                scheduler,
                pool,
                control,
                ..
            } = host;
            let pool: &ThreadPool = pool;
            let control: &RunControl = control;
            scheduler.pump(elapsed, clock, |c, index| {
                let ctx = TickCtx {
                    pool,
                    control,
                    tick_index: index,
                    step_ns,
                    frame: c.frame(),
                };
                tick_fn(c, &ctx);
            })
        };

        if report.halted {
            return RunStats {
                ticks: host.scheduler.ticks(),
                frame: host.clock.frame(),
                dropped_ns: host.scheduler.dropped_ns(),
                overruns: host.scheduler.overruns(),
                halted: true,
            };
        }

        if report.ticks_run == 0 {
            host.time.idle();
        }
    }

    RunStats {
        ticks: host.scheduler.ticks(),
        frame: host.clock.frame(),
        dropped_ns: host.scheduler.dropped_ns(),
        overruns: host.scheduler.overruns(),
        halted: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::SlowTickPolicy;

    fn host(pump_ns: u64, single: bool) -> Host {
        let pool = ThreadPool::new(4, single).expect("pool");
        Host::with_virtual_time(TickConfig::default(), pool, pump_ns).expect("host")
    }

    #[test]
    fn stop_flag_ends_the_loop_at_a_frame_boundary() {
        let mut h = host(1_000_000, true);
        let stats = run(&mut h, |_clock, ctx| {
            if ctx.tick_index >= 100 {
                ctx.control.stop();
            }
        });
        assert_eq!(stats.ticks, 100);
        assert_eq!(stats.frame, 100);
        assert!(!stats.halted);
    }

    #[test]
    fn external_stop_works_before_first_tick() {
        let mut h = host(1_000_000, true);
        h.stop();
        let stats = run(&mut h, |_clock, _ctx| {});
        assert_eq!(stats.ticks, 0);
    }

    #[test]
    fn halt_policy_stops_the_driver() {
        let cfg = TickConfig {
            max_catch_up_steps: 1,
            slow_tick: SlowTickPolicy::Halt,
            ..TickConfig::default()
        };
        // 100 ms between idles = 6 steps at 60 Hz, one more than the cap.
        let mut h = Host::new(
            cfg,
            ThreadPool::new(1, true).expect("pool"),
            Box::new(VirtualTime::new(100_000_000)),
        )
        .expect("host");
        let stats = run(&mut h, |_clock, _ctx| {});
        assert!(stats.halted);
        assert_eq!(stats.overruns, 1);
        assert_eq!(stats.ticks, 1);
        assert_eq!(stats.frame, 1);
    }
}
