//! Severity model shared by the logger and the profiler's reporting path.
//!
//! The engine's `rage::diagChannel` family ranks a message by severity and
//! lets a per-channel bit decide whether that channel is emitted at all. The
//! host keeps exactly that shape: a total order over six severities, and a
//! numeric rank that can be stored in one byte so filtering costs a compare
//! and a load (no locking on the emit path).
//!
//! | rank | [`Severity`] | engine usage                                       |
//! |------|--------------|----------------------------------------------------|
//! | 0    | `Trace`      | per-tick diagnostic detail (`diagDebug` at its most verbose) |
//! | 1    | `Debug`      | development detail, per-system state dumps         |
//! | 2    | `Info`       | lifecycle events: streamer opened a pack, map loaded |
//! | 3    | `Warn`       | recovered condition, degraded but continuing       |
//! | 4    | `Error`      | failed operation; engine carries on                |
//! | 5    | `Fatal`      | invariant violation; the tick loop cannot continue  |
//!
//! Assertions recovered from the module (the `diagChannel` assert sites) are
//! emitted at `Error` with an explicit `kind=assert` marker rather than as a
//! seventh severity, so a stream can be filtered by rank alone and asserts
//! still sort above warnings.

use std::fmt;

/// Ordered severity, as used by `rage::diagChannel`.
///
/// `Trace` is the most verbose and `Fatal` the most severe; `>=` on the enum
/// therefore means "at least as severe", which is the comparison every filter
/// in this module performs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Severity {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Fatal = 5,
}

impl Severity {
    /// Number of severities; the length of every per-severity counter array.
    pub const COUNT: usize = 6;

    /// All severities, least severe first.
    pub const ALL: [Severity; Severity::COUNT] = [
        Severity::Trace,
        Severity::Debug,
        Severity::Info,
        Severity::Warn,
        Severity::Error,
        Severity::Fatal,
    ];

    /// Lower-case name, as it appears in the `level=` field and in metric
    /// labels (`engine_log_records_total{level="warn"}`).
    pub const fn as_str(self) -> &'static str {
        match self {
            Severity::Trace => "trace",
            Severity::Debug => "debug",
            Severity::Info => "info",
            Severity::Warn => "warn",
            Severity::Error => "error",
            Severity::Fatal => "fatal",
        }
    }

    /// One-character badge, for compact streams where column width matters.
    pub const fn short(self) -> char {
        match self {
            Severity::Trace => 'T',
            Severity::Debug => 'D',
            Severity::Info => 'I',
            Severity::Warn => 'W',
            Severity::Error => 'E',
            Severity::Fatal => 'F',
        }
    }

    /// Rank as a `u8`; also the index into per-severity counters.
    pub const fn rank(self) -> u8 {
        self as u8
    }

    /// Inverse of [`Severity::rank`]. `None` for values outside `0..=5`, which
    /// is what a corrupted `--log-level` argument looks like.
    pub const fn from_rank(rank: u8) -> Option<Self> {
        match rank {
            0 => Some(Severity::Trace),
            1 => Some(Severity::Debug),
            2 => Some(Severity::Info),
            3 => Some(Severity::Warn),
            4 => Some(Severity::Error),
            5 => Some(Severity::Fatal),
            _ => None,
        }
    }

    /// Parse a severity name. Accepts the canonical lower-case spelling and
    /// the usual aliases seen in server CLI flags (`warning`, `err`, `critical`).
    pub fn parse(name: &str) -> Option<Self> {
        // Case-insensitive without allocating: compare ASCII case-insensitively.
        let eq = |want: &str| name.eq_ignore_ascii_case(want);
        if eq("trace") || eq("t") {
            Some(Severity::Trace)
        } else if eq("debug") || eq("d") {
            Some(Severity::Debug)
        } else if eq("info") || eq("i") || eq("notice") {
            Some(Severity::Info)
        } else if eq("warn") || eq("warning") || eq("w") {
            Some(Severity::Warn)
        } else if eq("error") || eq("err") || eq("e") {
            Some(Severity::Error)
        } else if eq("fatal") || eq("critical") || eq("crit") || eq("f") {
            Some(Severity::Fatal)
        } else {
            None
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for Severity {
    type Err = ParseSeverityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Severity::parse(s).ok_or(ParseSeverityError)
    }
}

/// Returned by [`Severity`]'s [`FromStr`](std::str::FromStr) impl.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseSeverityError;

impl fmt::Display for ParseSeverityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            "unknown severity (expected one of trace, debug, info, warn, error, fatal)",
        )
    }
}

impl std::error::Error for ParseSeverityError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_is_trace_to_fatal() {
        for pair in Severity::ALL.windows(2) {
            assert!(pair[0] < pair[1], "{} !< {}", pair[0], pair[1]);
        }
        assert!(Severity::Fatal >= Severity::Warn);
        assert!(!(Severity::Info >= Severity::Warn));
    }

    #[test]
    fn rank_round_trips_and_matches_order() {
        for (i, sev) in Severity::ALL.iter().copied().enumerate() {
            assert_eq!(sev.rank() as usize, i);
            assert_eq!(Severity::from_rank(sev.rank()), Some(sev));
        }
        assert_eq!(Severity::from_rank(6), None);
        assert_eq!(Severity::from_rank(u8::MAX), None);
    }

    #[test]
    fn names_round_trip() {
        for sev in Severity::ALL {
            assert_eq!(Severity::parse(sev.as_str()), Some(sev));
            assert_eq!(sev.as_str().parse::<Severity>(), Ok(sev));
            assert_eq!(sev.short().to_ascii_lowercase().to_string(), sev.as_str()[..1]);
        }
        assert_eq!(Severity::parse("WARNING"), Some(Severity::Warn));
        assert_eq!(Severity::parse("err"), Some(Severity::Error));
        assert_eq!(Severity::parse("bogus"), None);
        assert!("bogus".parse::<Severity>().is_err());
    }

    #[test]
    fn display_matches_field_encoding() {
        assert_eq!(Severity::Warn.to_string(), "warn");
        assert_eq!(format!("{}", Severity::Fatal), "fatal");
    }
}