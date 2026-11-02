//! `env.wasm_module_int_js` (imp 11) — the engine's two numeric switches.
//!
//! Recovered behaviour: the import is called with a key and returns an integer.
//! Exactly two keys exist: `gameResourceMB` (default 1024, amount of memory the
//! growable resource allocator may use) and `keepThreads` (default 0, a bit mask where
//! bit 1 = replay, 2 = net, 4 = bink).

/// The two keys the engine asks about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleKey {
    GameResourceMb,
    KeepThreads,
}

impl ModuleKey {
    pub fn from_id(id: i64) -> Option<Self> {
        match id {
            0 => Some(Self::GameResourceMb),
            1 => Some(Self::KeepThreads),
            _ => None,
        }
    }
}

/// Server-side answers to those questions.
#[derive(Debug, Clone, Copy)]
pub struct ModuleInts {
    pub game_resource_mb: i64,
    pub keep_threads: i64,
}

impl Default for ModuleInts {
    fn default() -> Self {
        Self { game_resource_mb: 1024, keep_threads: 0 }
    }
}

impl ModuleInts {
    pub fn get(&self, key: ModuleKey) -> i64 {
        match key {
            ModuleKey::GameResourceMb => self.game_resource_mb,
            ModuleKey::KeepThreads => self.keep_threads,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_the_browser_glue() {
        let m = ModuleInts::default();
        assert_eq!(m.get(ModuleKey::GameResourceMb), 1024);
        assert_eq!(m.get(ModuleKey::KeepThreads), 0);
        assert_eq!(ModuleKey::from_id(2), None);
    }
}
