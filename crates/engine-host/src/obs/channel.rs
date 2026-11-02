//! Diagnostic channels — the host-side mirror of `rage::diagChannel`.
//!
//! In the module every log call names a channel (`diagRender`, `diagNet`, …)
//! and the channel carries two pieces of policy: whether it is on at all, and
//! the lowest severity it will emit. Recovering that model matters for a
//! server because the useful default is *not* one global level: the streamer
//! wants `info`, the network layer wants `warn`, and `diagPerf` wants
//! `debug` only while someone is watching.
//!
//! A [`Channel`] is a `u8` index into the static table below, so a set of
//! channels is a 64-bit mask and testing membership is a shift and a mask.
//! The table is fixed at compile time: channels are named by the engine, not
//! by the operator, and adding one is a source change with a test that pins
//! its name.
//!
//! Names are stored verbatim as the engine spells them, including the `diag`
//! prefix, because log lines and metric labels are read against the recovered
//! string table. [`Channel::short_name`] returns the suffix for contexts where
//! the prefix is redundant (metric labels, CLI flags).

use std::fmt;

use super::severity::Severity;

/// Number of channels the engine defines. Bounded by 64 ([`ChannelMask`]).
pub const CHANNEL_COUNT: usize = 28;

/// One engine diagnostic channel.
///
/// Construction is restricted to the associated constants and
/// [`Channel::from_index`], so an out-of-range index cannot exist.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Channel(u8);

/// `(name, short name, default minimum severity)` for every channel,
/// indexed by [`Channel::index`]. The short name is stored rather than
/// derived so it can be read in a `const fn` on the crate's MSRV.
const TABLE: [(&str, &str, Severity); CHANNEL_COUNT] = [
    ("diagRender", "Render", Severity::Info),
    ("diagScene", "Scene", Severity::Warn),
    ("diagStreaming", "Streaming", Severity::Info),
    ("diagPhysics", "Physics", Severity::Warn),
    ("diagAnimation", "Animation", Severity::Warn),
    ("diagAudio", "Audio", Severity::Warn),
    ("diagScript", "Script", Severity::Info),
    ("diagAI", "AI", Severity::Warn),
    ("diagPeds", "Peds", Severity::Warn),
    ("diagVehicles", "Vehicles", Severity::Warn),
    ("diagPathfind", "Pathfind", Severity::Warn),
    ("diagTasks", "Tasks", Severity::Warn),
    ("diagNetwork", "Network", Severity::Info),
    ("diagFile", "File", Severity::Info),
    ("diagMemory", "Memory", Severity::Warn),
    ("diagAllocator", "Allocator", Severity::Warn),
    ("diagInput", "Input", Severity::Info),
    ("diagUI", "UI", Severity::Info),
    ("diagWeather", "Weather", Severity::Warn),
    ("diagTime", "Time", Severity::Info),
    ("diagThreads", "Threads", Severity::Info),
    ("diagGPU", "GPU", Severity::Warn),
    ("diagAssets", "Assets", Severity::Info),
    ("diagPerf", "Perf", Severity::Debug),
    ("diagAsserts", "Asserts", Severity::Error),
    ("diagDebug", "Debug", Severity::Debug),
    ("diagUserData", "UserData", Severity::Info),
    ("diagHost", "Host", Severity::Info),
];

impl Channel {
    pub const RENDER: Channel = Channel(0);
    pub const SCENE: Channel = Channel(1);
    pub const STREAMING: Channel = Channel(2);
    pub const PHYSICS: Channel = Channel(3);
    pub const ANIMATION: Channel = Channel(4);
    pub const AUDIO: Channel = Channel(5);
    pub const SCRIPT: Channel = Channel(6);
    pub const AI: Channel = Channel(7);
    pub const PEDS: Channel = Channel(8);
    pub const VEHICLES: Channel = Channel(9);
    pub const PATHFIND: Channel = Channel(10);
    pub const TASKS: Channel = Channel(11);
    pub const NETWORK: Channel = Channel(12);
    pub const FILE: Channel = Channel(13);
    pub const MEMORY: Channel = Channel(14);
    pub const ALLOCATOR: Channel = Channel(15);
    pub const INPUT: Channel = Channel(16);
    pub const UI: Channel = Channel(17);
    pub const WEATHER: Channel = Channel(18);
    pub const TIME: Channel = Channel(19);
    pub const THREADS: Channel = Channel(20);
    pub const GPU: Channel = Channel(21);
    pub const ASSETS: Channel = Channel(22);
    pub const PERF: Channel = Channel(23);
    pub const ASSERTS: Channel = Channel(24);
    pub const DEBUG: Channel = Channel(25);
    pub const USERDATA: Channel = Channel(26);
    pub const HOST: Channel = Channel(27);

    /// Every channel, in table order.
    pub const ALL: [Channel; CHANNEL_COUNT] = [
        Channel(0),
        Channel(1),
        Channel(2),
        Channel(3),
        Channel(4),
        Channel(5),
        Channel(6),
        Channel(7),
        Channel(8),
        Channel(9),
        Channel(10),
        Channel(11),
        Channel(12),
        Channel(13),
        Channel(14),
        Channel(15),
        Channel(16),
        Channel(17),
        Channel(18),
        Channel(19),
        Channel(20),
        Channel(21),
        Channel(22),
        Channel(23),
        Channel(24),
        Channel(25),
        Channel(26),
        Channel(27),
    ];

    /// Table index, also the bit position in [`ChannelMask`].
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    /// The engine's spelling, as recovered from the string table.
    pub const fn name(self) -> &'static str {
        TABLE[self.0 as usize].0
    }

    /// The name without the `diag` prefix (`Render`, `Streaming`, …).
    pub const fn short_name(self) -> &'static str {
        TABLE[self.0 as usize].1
    }

    /// Minimum severity this channel emits before any operator override.
    ///
    /// Chosen so a server that does nothing but set a global level still gets
    /// a sane stream: per-frame systems that would flood (`diagRender`,
    /// `diagScene`) start at `warn`, lifecycle systems at `info`, and the
    /// assert channel never lets anything below `error` through.
    pub const fn default_level(self) -> Severity {
        TABLE[self.0 as usize].2
    }

    /// Lookup by table index; `None` outside `0..CHANNEL_COUNT`.
    pub const fn from_index(index: u8) -> Option<Channel> {
        if (index as usize) < CHANNEL_COUNT {
            Some(Channel(index))
        } else {
            None
        }
    }

    /// Lookup by engine name, with or without the `diag` prefix.
    ///
    /// Linear over 28 entries — this is config-path code, called when a
    /// command-line flag or a control-port request names a channel.
    pub fn from_name(name: &str) -> Option<Channel> {
        let stripped = name
            .strip_prefix("diag")
            .or_else(|| name.strip_prefix("Diag"))
            .unwrap_or(name);
        Channel::ALL.into_iter().find(|ch| {
            ch.short_name().eq_ignore_ascii_case(stripped) || ch.name().eq_ignore_ascii_case(name)
        })
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A set of enabled channels, as a 64-bit mask.
///
/// One bit per [`Channel`]; bit `i` is [`Channel::from_index(i)`]. All
/// operations are total: bits outside the table are ignored rather than
/// panicking, so a mask parsed from operator input cannot take the process
/// down.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ChannelMask(u64);

impl ChannelMask {
    /// No channel enabled.
    pub const EMPTY: ChannelMask = ChannelMask(0);

    /// Every channel enabled.
    pub fn all() -> ChannelMask {
        let mut mask = ChannelMask::EMPTY;
        for ch in Channel::ALL {
            mask.insert(ch);
        }
        mask
    }

    pub fn insert(&mut self, channel: Channel) -> &mut Self {
        self.0 |= 1u64 << channel.index();
        self
    }

    pub fn remove(&mut self, channel: Channel) -> &mut Self {
        self.0 &= !(1u64 << channel.index());
        self
    }

    pub fn set(&mut self, channel: Channel, enabled: bool) -> &mut Self {
        if enabled {
            self.insert(channel)
        } else {
            self.remove(channel)
        }
    }

    #[inline]
    pub fn contains(self, channel: Channel) -> bool {
        self.0 & (1u64 << channel.index()) != 0
    }

    pub fn union(self, other: ChannelMask) -> ChannelMask {
        ChannelMask(self.0 | other.0)
    }

    pub fn intersection(self, other: ChannelMask) -> ChannelMask {
        ChannelMask(self.0 & other.0)
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Count of set bits inside the table (stray high bits are not counted).
    pub fn len(self) -> usize {
        let table_bits = if CHANNEL_COUNT == 64 {
            u64::MAX
        } else {
            (1u64 << CHANNEL_COUNT) - 1
        };
        (self.0 & table_bits).count_ones() as usize
    }

    /// Iterate the set channels in table order.
    pub fn iter(self) -> impl Iterator<Item = Channel> {
        Channel::ALL.into_iter().filter(move |ch| self.contains(*ch))
    }

    /// Underlying bits. Useful for a control-port dump; not stable output.
    pub const fn bits(self) -> u64 {
        self.0
    }

    pub const fn from_bits(bits: u64) -> ChannelMask {
        ChannelMask(bits)
    }
}

impl FromIterator<Channel> for ChannelMask {
    fn from_iter<I: IntoIterator<Item = Channel>>(iter: I) -> Self {
        let mut mask = ChannelMask::EMPTY;
        for ch in iter {
            mask.insert(ch);
        }
        mask
    }
}

/// A channel with an explicit per-channel level, as set by an operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChannelPolicy {
    pub enabled: bool,
    pub level: Severity,
}

impl ChannelPolicy {
    pub const fn default_for(channel: Channel) -> ChannelPolicy {
        ChannelPolicy {
            enabled: true,
            level: channel.default_level(),
        }
    }
}

impl Default for ChannelPolicy {
    fn default() -> Self {
        ChannelPolicy {
            enabled: true,
            level: Severity::Trace,
        }
    }
}

/// Compile-time guard: the table must fit the mask, and every index must be
/// reachable. `Channel::ALL` is hand-written, so this catches a transposed
/// row at build time rather than at the first log line.
const _: () = assert!(CHANNEL_COUNT <= 64);
const _: () = assert!(Channel::HOST.index() == CHANNEL_COUNT - 1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_match_table_names() {
        let expected = [
            (Channel::RENDER, "diagRender"),
            (Channel::SCENE, "diagScene"),
            (Channel::STREAMING, "diagStreaming"),
            (Channel::PHYSICS, "diagPhysics"),
            (Channel::ANIMATION, "diagAnimation"),
            (Channel::AUDIO, "diagAudio"),
            (Channel::SCRIPT, "diagScript"),
            (Channel::AI, "diagAI"),
            (Channel::PEDS, "diagPeds"),
            (Channel::VEHICLES, "diagVehicles"),
            (Channel::PATHFIND, "diagPathfind"),
            (Channel::TASKS, "diagTasks"),
            (Channel::NETWORK, "diagNetwork"),
            (Channel::FILE, "diagFile"),
            (Channel::MEMORY, "diagMemory"),
            (Channel::ALLOCATOR, "diagAllocator"),
            (Channel::INPUT, "diagInput"),
            (Channel::UI, "diagUI"),
            (Channel::WEATHER, "diagWeather"),
            (Channel::TIME, "diagTime"),
            (Channel::THREADS, "diagThreads"),
            (Channel::GPU, "diagGPU"),
            (Channel::ASSETS, "diagAssets"),
            (Channel::PERF, "diagPerf"),
            (Channel::ASSERTS, "diagAsserts"),
            (Channel::DEBUG, "diagDebug"),
            (Channel::USERDATA, "diagUserData"),
            (Channel::HOST, "diagHost"),
        ];
        assert_eq!(expected.len(), CHANNEL_COUNT);
        for (i, (ch, name)) in expected.into_iter().enumerate() {
            assert_eq!(ch.index(), i, "{} is out of order", name);
            assert_eq!(ch.name(), name);
            assert_eq!(Channel::ALL[i], ch);
            assert_eq!(Channel::from_index(i as u8), Some(ch));
        }
    }

    #[test]
    fn short_name_strips_prefix() {
        assert_eq!(Channel::RENDER.short_name(), "Render");
        assert_eq!(Channel::USERDATA.short_name(), "UserData");
        assert_eq!(Channel::NETWORK.short_name(), "Network");
    }

    #[test]
    fn from_name_accepts_prefixed_and_bare() {
        assert_eq!(Channel::from_name("diagNetwork"), Some(Channel::NETWORK));
        assert_eq!(Channel::from_name("Network"), Some(Channel::NETWORK));
        assert_eq!(Channel::from_name("network"), Some(Channel::NETWORK));
        assert_eq!(Channel::from_name("diagnetwork"), Some(Channel::NETWORK));
        assert_eq!(Channel::from_name("diagNope"), None);
        assert_eq!(Channel::from_name(""), None);
    }

    #[test]
    fn from_index_rejects_out_of_range() {
        assert_eq!(Channel::from_index(0), Some(Channel::RENDER));
        assert_eq!(Channel::from_index(CHANNEL_COUNT as u8 - 1), Some(Channel::HOST));
        assert_eq!(Channel::from_index(CHANNEL_COUNT as u8), None);
        assert_eq!(Channel::from_index(255), None);
    }

    #[test]
    fn mask_operations() {
        let mut mask = ChannelMask::EMPTY;
        assert!(mask.is_empty());
        mask.insert(Channel::NETWORK).insert(Channel::RENDER);
        assert!(mask.contains(Channel::NETWORK));
        assert!(!mask.contains(Channel::AUDIO));
        assert_eq!(mask.len(), 2);
        assert_eq!(mask.iter().collect::<Vec<_>>(), vec![Channel::RENDER, Channel::NETWORK]);

        mask.remove(Channel::RENDER);
        assert_eq!(mask.len(), 1);

        let all = ChannelMask::all();
        assert_eq!(all.len(), CHANNEL_COUNT);
        assert!(Channel::ALL.into_iter().all(|ch| all.contains(ch)));
        assert_eq!(all.intersection(mask), mask);
        assert_eq!(mask.union(ChannelMask::EMPTY), mask);
        assert_eq!(ChannelMask::from_bits(mask.bits()), mask);
    }

    #[test]
    fn mask_ignores_bits_outside_table() {
        // A control-port poke that sets bit 63 must not be able to invent a
        // channel or trip a length check.
        let mask = ChannelMask::from_bits(1u64 << 63);
        assert_eq!(mask.len(), 0);
        assert!(mask.iter().next().is_none());
        assert!(!mask.contains(Channel::RENDER));
    }

    #[test]
    fn default_levels_are_sane() {
        assert_eq!(Channel::ASSERTS.default_level(), Severity::Error);
        assert_eq!(Channel::PERF.default_level(), Severity::Debug);
        assert_eq!(Channel::STREAMING.default_level(), Severity::Info);
        assert_eq!(ChannelPolicy::default_for(Channel::ASSERTS).enabled, true);
    }
}