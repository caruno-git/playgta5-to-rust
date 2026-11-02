//! Monotonic time, in the two units the engine asks for.
//!
//! The engine calls `env.wasm_clock_micros` (frame pacing, streamer budget) and
//! `env.wasm_clock_nanos` (audio/video sync, profiler scopes). Both must be
//! driven by one monotonic origin so they never disagree, and both must be
//! cheap: the engine reads them thousands of times per frame.

use std::time::{Duration, Instant};

/// Microseconds since the host clock's origin (engine units: `u64`, wrapping
/// only after ~584 000 years).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Micros(pub u64);

impl Micros {
    pub const ZERO: Micros = Micros(0);

    pub fn from_duration(d: Duration) -> Self {
        // Saturating rather than wrapping: a >584k-year uptime is not a case
        // worth trading a panic for.
        Self(u64::try_from(d.as_micros()).unwrap_or(u64::MAX))
    }

    pub fn as_duration(self) -> Duration {
        Duration::from_micros(self.0)
    }

    pub fn saturating_sub(self, other: Micros) -> Micros {
        Micros(self.0.saturating_sub(other.0))
    }
}

/// Nanoseconds since the host clock's origin. Wraps only after ~584 years.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nanos(pub u64);

impl Nanos {
    pub const ZERO: Nanos = Nanos(0);

    pub fn from_duration(d: Duration) -> Self {
        Self(u64::try_from(d.as_nanos()).unwrap_or(u64::MAX))
    }

    pub fn as_duration(self) -> Duration {
        Duration::from_nanos(self.0)
    }
}

/// One monotonic origin shared by [`Micros`] and [`Nanos`].
///
/// Replaces `env.wasm_clock_micros` and `env.wasm_clock_nanos`
/// (`performance.now() * 1000` in `game.js`). `std::time::Instant` is
/// documented as monotonic, which is the property the engine's frame timing
/// asserts on; `SystemTime` is not used anywhere in the engine path.
#[derive(Clone, Debug)]
pub struct MonotonicClock {
    origin: Instant,
}

impl MonotonicClock {
    pub fn new() -> Self {
        Self {
            origin: Instant::now(),
        }
    }

    /// Microseconds since origin — `env.wasm_clock_micros`.
    pub fn micros(&self) -> u64 {
        Micros::from_duration(self.origin.elapsed()).0
    }

    /// Nanoseconds since origin — `env.wasm_clock_nanos`.
    pub fn nanos(&self) -> u64 {
        Nanos::from_duration(self.origin.elapsed()).0
    }

    pub fn elapsed(&self) -> Duration {
        self.origin.elapsed()
    }

    /// Convenience for frame deltas: time since an earlier micros reading.
    pub fn since(&self, earlier: Micros) -> Duration {
        self.origin
            .elapsed()
            .saturating_sub(earlier.as_duration())
    }
}

impl Default for MonotonicClock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micros_are_monotonic() {
        let clock = MonotonicClock::new();
        let mut previous = clock.micros();
        for _ in 0..512 {
            let now = clock.micros();
            assert!(
                now >= previous,
                "clock went backwards: {previous} -> {now}"
            );
            previous = now;
        }
    }

    #[test]
    fn nano_and_micro_readings_agree() {
        let clock = MonotonicClock::new();
        for _ in 0..64 {
            let micros = clock.micros();
            let nanos = clock.nanos();
            // Sampled in this order, so nanos >= micros*1000 up to one tick of
            // slack for the second read.
            assert!(nanos + 1_000 >= micros.saturating_mul(1_000));
            assert!(nanos <= micros.saturating_mul(1_000) + 1_000_000);
        }
    }

    #[test]
    fn sleep_advances_the_clock() {
        let clock = MonotonicClock::new();
        let before = clock.micros();
        std::thread::sleep(Duration::from_millis(5));
        let after = clock.micros();
        assert!(
            after.saturating_sub(before) >= 1_000,
            "5ms sleep advanced only {}us",
            after.saturating_sub(before)
        );
    }

    #[test]
    fn unit_conversions_round_trip() {
        let d = Duration::from_micros(1_234_567);
        assert_eq!(Micros::from_duration(d).as_duration(), d);
        assert_eq!(Nanos::from_duration(d).as_duration(), d);
        assert_eq!(Micros(10).saturating_sub(Micros(20)), Micros::ZERO);
    }
}