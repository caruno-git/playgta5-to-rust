//! Fixed-timestep scheduler: accumulator loop with a catch-up cap.
//!
//! Contract, in one paragraph: real (or virtual) elapsed time is added to an
//! accumulator; while the accumulator holds at least one step, one step is
//! removed and exactly one tick runs; the tick's delta is the fixed step, never
//! the observed elapsed time. Therefore `frames == ticks executed` always, and
//! the simulation advances in identical increments no matter how the pumps are
//! spaced.
//!
//! Rate default: 60 Hz ([`DEFAULT_TICK_HZ`]). That is a *server* choice, not a
//! value from the binary — the engine's own pacing hint is the hardcoded
//! `sysIpcSleep(33)` at the `CSystem::EndRender` sites, i.e. ~30.3 Hz. 60 Hz is
//! the default here because a server wants a finer simulation step; set
//! `hz: 30` if you are replaying the shipping frame rate.

use crate::clock::Clock;

/// Default tick rate. Server-side choice; the binary's own pacing is
/// `sysIpcSleep(33)` (≈30.3 Hz), documented in `SERVER-TICK.md`.
pub const DEFAULT_TICK_HZ: u32 = 60;

/// Highest accepted tick rate (guards against a step of 0 ns).
pub const MAX_TICK_HZ: u32 = 1_000;

/// Lowest accepted tick rate.
pub const MIN_TICK_HZ: u32 = 1;

/// Ticks a single pump may run before the slow-tick policy applies.
pub const DEFAULT_MAX_CATCH_UP_STEPS: u32 = 8;

/// Hard ceiling on retained backlog time (250 ms).
pub const DEFAULT_MAX_BACKLOG_NS: u64 = 250_000_000;

/// Nanoseconds in one second (local copy so this module stays standalone).
const NANOS_PER_SEC: u64 = 1_000_000_000;

/// What to do when the accumulated work exceeds [`TickConfig::max_catch_up_steps`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SlowTickPolicy {
    /// Default. Run up to the cap, then throw away the remaining backlog and
    /// record it as dropped time. The simulation dilates (game time falls
    /// behind wall time) but never spirals, and the frame counter stays exactly
    /// in step with executed ticks.
    #[default]
    DropExcess,
    /// Run up to the cap and keep the backlog for the next pump, bounded by
    /// [`TickConfig::max_backlog_ns`]. Total ticks then equal
    /// `floor(total_elapsed / step)` — no drift, ever, at the cost of latency.
    CatchUp,
    /// Run up to the cap, then stop the driver (report an overrun). Strictest
    /// fidelity: a tick that cannot keep up is a server fault, not something to
    /// paper over.
    Halt,
}

/// Scheduler configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TickConfig {
    /// Ticks per simulated second.
    pub hz: u32,
    /// Maximum ticks executed by one [`Scheduler::pump`] call.
    pub max_catch_up_steps: u32,
    /// Backlog ceiling in nanoseconds (only enforced for
    /// [`SlowTickPolicy::CatchUp`]).
    pub max_backlog_ns: u64,
    /// Policy used when the cap is hit.
    pub slow_tick: SlowTickPolicy,
}

impl Default for TickConfig {
    fn default() -> Self {
        Self {
            hz: DEFAULT_TICK_HZ,
            max_catch_up_steps: DEFAULT_MAX_CATCH_UP_STEPS,
            max_backlog_ns: DEFAULT_MAX_BACKLOG_NS,
            slow_tick: SlowTickPolicy::default(),
        }
    }
}

impl TickConfig {
    /// Config at `hz` with default caps.
    pub const fn at(hz: u32) -> Self {
        Self {
            hz,
            max_catch_up_steps: DEFAULT_MAX_CATCH_UP_STEPS,
            max_backlog_ns: DEFAULT_MAX_BACKLOG_NS,
            slow_tick: SlowTickPolicy::DropExcess,
        }
    }

    /// Step length in nanoseconds, rounded to nearest (60 Hz => 16_666_667 ns).
    ///
    /// Rounded up at 60 Hz so the step is never *shorter* than the nominal
    /// frame; the residual is 20 ns per second and is irrelevant because the
    /// frame count is derived from executed ticks, not from time.
    pub const fn step_ns(&self) -> u64 {
        let hz = self.hz as u64;
        (NANOS_PER_SEC + hz / 2) / hz
    }
}

/// Invalid [`TickConfig`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TickConfigError {
    /// `hz` outside `MIN_TICK_HZ..=MAX_TICK_HZ`.
    RateOutOfRange {
        /// Requested rate.
        hz: u32,
    },
    /// `max_catch_up_steps == 0` would never run a tick.
    NoCatchUpSteps,
}

impl core::fmt::Display for TickConfigError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RateOutOfRange { hz } => write!(
                f,
                "tick rate {hz} Hz outside supported range {MIN_TICK_HZ}..={MAX_TICK_HZ}"
            ),
            Self::NoCatchUpSteps => write!(f, "max_catch_up_steps must be >= 1"),
        }
    }
}

impl std::error::Error for TickConfigError {}

/// Result of one [`Scheduler::pump`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PumpReport {
    /// Ticks executed by this pump.
    pub ticks_run: u32,
    /// Total ticks executed since construction.
    pub ticks_total: u64,
    /// Backlog discarded by this pump, in nanoseconds.
    pub dropped_ns: u64,
    /// True when [`SlowTickPolicy::Halt`] fired.
    pub halted: bool,
    /// True when the catch-up cap was reached.
    pub capped: bool,
}

/// The accumulator.
#[derive(Debug, Clone)]
pub struct Scheduler {
    cfg: TickConfig,
    step_ns: u64,
    acc_ns: u64,
    ticks: u64,
    dropped_ns: u64,
    overruns: u64,
}

impl Scheduler {
    /// Build a scheduler, validating the rate and caps.
    pub fn new(cfg: TickConfig) -> Result<Self, TickConfigError> {
        if cfg.hz < MIN_TICK_HZ || cfg.hz > MAX_TICK_HZ {
            return Err(TickConfigError::RateOutOfRange { hz: cfg.hz });
        }
        if cfg.max_catch_up_steps == 0 {
            return Err(TickConfigError::NoCatchUpSteps);
        }
        Ok(Self {
            step_ns: cfg.step_ns(),
            cfg,
            acc_ns: 0,
            ticks: 0,
            dropped_ns: 0,
            overruns: 0,
        })
    }

    /// The configuration in force.
    pub const fn config(&self) -> &TickConfig {
        &self.cfg
    }

    /// Fixed step, nanoseconds.
    pub const fn step_ns(&self) -> u64 {
        self.step_ns
    }

    /// Ticks executed so far.
    pub const fn ticks(&self) -> u64 {
        self.ticks
    }

    /// Backlog currently held, nanoseconds (< one step between pumps).
    pub const fn backlog_ns(&self) -> u64 {
        self.acc_ns
    }

    /// Total backlog discarded under [`SlowTickPolicy::DropExcess`].
    pub const fn dropped_ns(&self) -> u64 {
        self.dropped_ns
    }

    /// Number of pumps that hit the catch-up cap.
    pub const fn overruns(&self) -> u64 {
        self.overruns
    }

    /// How many ticks the current backlog can pay for, capped.
    pub fn due(&self) -> u32 {
        let affordable = self.acc_ns / self.step_ns;
        affordable.min(self.cfg.max_catch_up_steps as u64) as u32
    }

    /// Feed elapsed time and run however many ticks it pays for.
    ///
    /// The tick closure receives the clock (already advanced into the frame it
    /// must simulate) and the 1-based tick index.
    pub fn pump<F>(&mut self, elapsed_ns: u64, clock: &mut Clock, mut tick: F) -> PumpReport
    where
        F: FnMut(&mut Clock, u64),
    {
        let mut report = PumpReport::default();
        self.acc_ns = self.acc_ns.saturating_add(elapsed_ns);

        while self.acc_ns >= self.step_ns {
            if report.ticks_run >= self.cfg.max_catch_up_steps {
                report.capped = true;
                self.overruns = self.overruns.saturating_add(1);
                match self.cfg.slow_tick {
                    SlowTickPolicy::Halt => {
                        report.halted = true;
                    }
                    SlowTickPolicy::DropExcess => {
                        report.dropped_ns = self.acc_ns;
                        self.acc_ns = 0;
                    }
                    SlowTickPolicy::CatchUp => {
                        // Keep the backlog, but never beyond the ceiling: a
                        // 10 s stall must not turn into 600 queued ticks.
                        if self.acc_ns > self.cfg.max_backlog_ns {
                            report.dropped_ns = self.acc_ns - self.cfg.max_backlog_ns;
                            self.acc_ns = self.cfg.max_backlog_ns;
                        }
                    }
                }
                break;
            }

            self.acc_ns -= self.step_ns;
            let index = clock.step(self.step_ns);
            self.ticks = index;
            report.ticks_run += 1;
            tick(clock, index);
        }

        self.dropped_ns = self.dropped_ns.saturating_add(report.dropped_ns);
        report.ticks_total = self.ticks;
        report
    }

    /// Drop any sub-step remainder (used when resuming from a snapshot so the
    /// first frame is not a partial one).
    pub fn clear_backlog(&mut self) {
        self.acc_ns = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_rate_is_60hz() {
        let cfg = TickConfig::default();
        assert_eq!(cfg.hz, 60);
        assert_eq!(cfg.step_ns(), 16_666_667);
    }

    #[test]
    fn rejects_bad_rates() {
        assert_eq!(
            Scheduler::new(TickConfig::at(0)).unwrap_err(),
            TickConfigError::RateOutOfRange { hz: 0 }
        );
        assert_eq!(
            Scheduler::new(TickConfig::at(MAX_TICK_HZ + 1)).unwrap_err(),
            TickConfigError::RateOutOfRange {
                hz: MAX_TICK_HZ + 1
            }
        );
        let cfg = TickConfig {
            max_catch_up_steps: 0,
            ..TickConfig::default()
        };
        assert_eq!(
            Scheduler::new(cfg).unwrap_err(),
            TickConfigError::NoCatchUpSteps
        );
    }

    #[test]
    fn sub_step_time_accumulates_across_pumps() {
        let mut sched = Scheduler::new(TickConfig::default()).unwrap();
        let mut clock = Clock::new();
        let step = sched.step_ns();
        // three pumps of a third of a step => exactly one tick
        for _ in 0..3 {
            let r = sched.pump(step / 3, &mut clock, |_, _| {});
            assert_eq!(r.ticks_run, 0);
        }
        let r = sched.pump(step / 3 + 1, &mut clock, |_, _| {});
        assert_eq!(r.ticks_run, 1);
        assert_eq!(clock.frame(), 1);
    }

    #[test]
    fn drop_excess_reports_dropped_time() {
        let cfg = TickConfig {
            max_catch_up_steps: 2,
            ..TickConfig::default()
        };
        let mut sched = Scheduler::new(cfg).unwrap();
        let mut clock = Clock::new();
        let step = sched.step_ns();
        let r = sched.pump(step * 10, &mut clock, |_, _| {});
        assert_eq!(r.ticks_run, 2);
        assert!(r.capped);
        assert_eq!(r.dropped_ns, step * 8);
        assert_eq!(clock.frame(), 2);
        assert_eq!(sched.backlog_ns(), 0);
    }

    #[test]
    fn halt_stops_and_keeps_backlog() {
        let cfg = TickConfig {
            max_catch_up_steps: 2,
            slow_tick: SlowTickPolicy::Halt,
            ..TickConfig::default()
        };
        let mut sched = Scheduler::new(cfg).unwrap();
        let mut clock = Clock::new();
        let step = sched.step_ns();
        let r = sched.pump(step * 10, &mut clock, |_, _| {});
        assert!(r.halted);
        assert_eq!(r.ticks_run, 2);
        assert_eq!(r.dropped_ns, 0);
        assert_eq!(sched.backlog_ns(), step * 8);
    }

    #[test]
    fn catch_up_preserves_total_tick_count() {
        let cfg = TickConfig {
            max_catch_up_steps: 3,
            slow_tick: SlowTickPolicy::CatchUp,
            ..TickConfig::default()
        };
        let mut sched = Scheduler::new(cfg).unwrap();
        let mut clock = Clock::new();
        let step = sched.step_ns();
        let total = step * 320 + step / 2;
        let mut fed = 0;
        while fed < total {
            let chunk = (step / 4).max(1);
            let r = sched.pump(chunk, &mut clock, |_, _| {});
            fed += chunk;
            if r.halted {
                break;
            }
        }
        assert_eq!(sched.ticks(), 320);
        assert_eq!(clock.frame(), 320);
    }
}
