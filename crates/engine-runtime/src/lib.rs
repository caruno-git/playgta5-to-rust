//! Deterministic fixed-timestep runtime for the headless GTA V server port.
//!
//! The recovered engine is single-threaded at the top level: one call to
//! `Main_OneLoopIteration` (func_idx 106, 86 bytes) is one frame, and that frame
//! is internally a debug guard plus exactly one `fwFsm::Update` step
//! (func_idx 8873). Everything below re-creates that contract on the server:
//!
//! * [`Clock`] — a virtual monotonic clock plus a frame counter with the same
//!   storage shape as `GetGameFrame` (func_idx 91) reads, which is a signed
//!   32-bit value at VA 12321252 (`0x00BC01E4`). `fwTimer::Update`
//!   (func_idx 8924) is the writer; this crate performs that same increment
//!   once per tick inside [`Clock::step`].
//! * [`Scheduler`] — accumulator loop at a configurable rate (default 60 Hz),
//!   with a catch-up cap and an explicit slow-tick policy.
//! * [`Host`] / [`run`] — the driver: owns clock + scheduler + job pool + stop
//!   flag and calls the user tick function once per frame.
//! * [`ThreadPool`] — a job-pool shim matching the engine's `JobManager`
//!   expectations (N workers, FIFO queue, deterministic completion), with a
//!   `single_threaded` switch so a server can force determinism.
//!
//! Determinism rules enforced by this crate (see `SERVER-TICK.md` for the full
//! policy): nothing inside a tick reads wall-clock time, the tick delta is a
//! constant derived from the tick rate, and the frame counter only ever
//! advances by one per executed tick.
//!
//! ```
//! use engine_runtime::{Clock, Scheduler, TickConfig};
//!
//! let cfg = TickConfig::default();               // 60 Hz, step = 16_666_667 ns
//! let mut sched = Scheduler::new(cfg).expect("valid rate");
//! let mut clock = Clock::new();
//! let step = sched.step_ns();
//!
//! // Sub-step pumps accumulate; the first pump that completes a step ticks.
//! let mut ran = 0;
//! sched.pump(step / 2, &mut clock, |_clock, _i| ran += 1);      // 0 ticks
//! sched.pump(step / 2 + 1, &mut clock, |_clock, _i| ran += 1);  // 1 tick
//! assert_eq!(ran, 1);
//! assert_eq!(clock.frame(), 1);
//! assert_eq!(clock.delta_ns(), step);
//! ```

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
#![warn(missing_docs)]

pub mod clock;
pub mod hash;
pub mod host;
pub mod scheduler;
pub mod threadpool;

pub use clock::{Clock, DeltaSource, FRAME_COUNTER_VA, FW_TIMER_UPDATE_FN, GAME_FRAME_FN};
pub use hash::{hash_bytes, StateHasher};
pub use host::{run, Host, OsTime, RunControl, RunStats, TickCtx, TimeSource, VirtualTime};
pub use scheduler::{
    PumpReport, Scheduler, SlowTickPolicy, TickConfig, TickConfigError, DEFAULT_MAX_BACKLOG_NS,
    DEFAULT_MAX_CATCH_UP_STEPS, DEFAULT_TICK_HZ, MAX_TICK_HZ, MIN_TICK_HZ,
};
pub use threadpool::{PoolError, ThreadPool};
