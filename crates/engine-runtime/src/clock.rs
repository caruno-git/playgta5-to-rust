//! Virtual frame clock.
//!
//! Binary facts used in this module (all taken from the recovered wasm64 module):
//!
//! | fact | source in binary |
//! |------|------------------|
//! | `GetGameFrame` is func_idx 91 and *reads* a signed i32 (never writes) | func_idx 91 |
//! | that frame counter lives at VA 12321252 (`0x00BC01E4`) | func_idx 91's single load |
//! | `fwTimer::Update` (func_idx 8924) is the incrementing writer | func_idx 8924 |
//! | one frame == one `Main_OneLoopIteration` (func_idx 106) | func_idx 106 |
//!
//! Nothing here touches `std::time`. Time only moves when the driver calls
//! [`Clock::step`], which is what makes a tick reproducible.

/// Virtual address of the engine's frame counter (signed i32).
///
/// `GetGameFrame` (func_idx 91) loads from this address; `fwTimer::Update`
/// (func_idx 8924) stores to it. Reproduced here as [`Clock::frame`].
pub const FRAME_COUNTER_VA: u32 = 12_321_252; // 0x00BC01E4

/// func_idx of `GetGameFrame`, the reader of [`FRAME_COUNTER_VA`].
pub const GAME_FRAME_FN: u32 = 91;

/// func_idx of `fwTimer::Update`, the writer of [`FRAME_COUNTER_VA`].
pub const FW_TIMER_UPDATE_FN: u32 = 8924;

/// Nanoseconds per second, for conversions.
pub const NANOS_PER_SEC: u64 = 1_000_000_000;

/// Where a tick's `delta` (frame time) comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeltaSource {
    /// The delta is the value the driver hands to [`Clock::step`] — i.e. the
    /// scheduler's fixed step. Time and delta always agree. This is the
    /// server default and the only mode that is deterministic by construction.
    Driven,
    /// The delta (and the advance) is forced to a constant, ignoring whatever
    /// the driver passes. Use when the engine code being replayed expects a
    /// hardcoded frame time (e.g. a 30 Hz title screen).
    Fixed(u64),
}

/// A virtual, monotonically increasing clock with the engine's frame counter.
///
/// `now` advances only through [`Clock::step`]; there is no syscall and no
/// wall-clock read anywhere in this type, so two runs fed the same step
/// sequence observe byte-identical times.
#[derive(Debug, Clone)]
pub struct Clock {
    now_ns: u64,
    delta_ns: u64,
    /// Mirrors the i32 at [`FRAME_COUNTER_VA`]; 1-based inside a tick because
    /// `fwTimer::Update` bumps it before the frame body runs.
    frame: i32,
    ticks: u64,
    source: DeltaSource,
}

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock {
    /// A clock at t=0, frame 0, driven delta.
    pub const fn new() -> Self {
        Self {
            now_ns: 0,
            delta_ns: 0,
            frame: 0,
            ticks: 0,
            source: DeltaSource::Driven,
        }
    }

    /// A clock with an explicit delta policy.
    pub const fn with_delta_source(source: DeltaSource) -> Self {
        Self {
            source,
            ..Self::new()
        }
    }

    /// Virtual monotonic time in nanoseconds.
    pub const fn now_ns(&self) -> u64 {
        self.now_ns
    }

    /// Virtual monotonic time in seconds (f64; presentation only).
    pub fn now_secs(&self) -> f64 {
        self.now_ns as f64 / NANOS_PER_SEC as f64
    }

    /// The delta used by the tick that is currently running, in nanoseconds.
    pub const fn delta_ns(&self) -> u64 {
        self.delta_ns
    }

    /// The delta in seconds — the value engine code would pass around as
    /// `Timestep`.
    pub fn delta_secs(&self) -> f32 {
        self.delta_ns as f32 / NANOS_PER_SEC as f32
    }

    /// Current frame number, with the same signed-i32 storage as
    /// `GetGameFrame` (func_idx 91). 1-based: the first tick sees `1`.
    pub const fn frame(&self) -> i32 {
        self.frame
    }

    /// Frame number as an unsigned index (i32 bits reinterpreted, matching how
    /// `GetGameFrame`'s result is zero-extended into the caller's u64).
    pub const fn frame_index(&self) -> u32 {
        self.frame as u32
    }

    /// Number of ticks executed since construction.
    pub const fn ticks(&self) -> u64 {
        self.ticks
    }

    /// The delta policy in force.
    pub const fn delta_source(&self) -> DeltaSource {
        self.source
    }

    /// Advance the clock by one engine frame and bump the frame counter.
    ///
    /// This is `Main_OneLoopIteration`'s time bookkeeping: the `now`/`delta`
    /// write plus the `fwTimer::Update` (func_idx 8924) increment of the i32 at
    /// VA 12321252. Wrapping is intentional — the binary's counter is a plain
    /// i32 and wraps after 2^31 frames (~414 days at 60 Hz).
    ///
    /// Returns the tick index (1-based) that the caller should pass to the tick
    /// body.
    pub fn step(&mut self, dt_ns: u64) -> u64 {
        let effective = match self.source {
            DeltaSource::Driven => dt_ns,
            DeltaSource::Fixed(fixed) => fixed,
        };
        self.now_ns = self.now_ns.saturating_add(effective);
        self.delta_ns = effective;
        self.frame = self.frame.wrapping_add(1);
        self.ticks = self.ticks.saturating_add(1);
        self.ticks
    }

    /// Overwrite the frame counter, e.g. to resume from a snapshot. The value
    /// is stored in the same wrapping i32 the engine uses.
    pub fn set_frame(&mut self, frame: i32) {
        self.frame = frame;
    }

    /// Reset time and counters to the initial state, keeping the delta policy.
    pub fn reset(&mut self) {
        self.now_ns = 0;
        self.delta_ns = 0;
        self.frame = 0;
        self.ticks = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_counter_matches_getgameframe_shape() {
        let mut clock = Clock::new();
        assert_eq!(clock.frame(), 0);
        assert_eq!(clock.step(16_666_667), 1);
        assert_eq!(clock.frame(), 1);
        assert_eq!(clock.frame_index(), 1);
        assert_eq!(clock.now_ns(), 16_666_667);
        assert_eq!(clock.delta_ns(), 16_666_667);
    }

    #[test]
    fn fixed_delta_ignores_driver_dt() {
        let mut clock = Clock::with_delta_source(DeltaSource::Fixed(33_333_333));
        clock.step(1_000);
        assert_eq!(clock.delta_ns(), 33_333_333);
        assert_eq!(clock.now_ns(), 33_333_333);
    }

    #[test]
    fn frame_wraps_like_i32() {
        let mut clock = Clock::new();
        clock.set_frame(i32::MAX);
        clock.step(1);
        assert_eq!(clock.frame(), i32::MIN);
    }

    #[test]
    fn address_is_the_one_getgameframe_reads() {
        assert_eq!(FRAME_COUNTER_VA, 12_321_252);
        assert_eq!(format!("{:#010X}", FRAME_COUNTER_VA), "0x00BC01E4");
        assert_eq!(GAME_FRAME_FN, 91);
        assert_eq!(FW_TIMER_UPDATE_FN, 8924);
    }
}
