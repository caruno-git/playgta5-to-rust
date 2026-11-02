//! `game-server` — headless server library for the machine-translated engine.
//!
//! Two layers live here:
//!
//! * [`mem`] / [`segments`] / [`assets`] — the engine's linear memory (reserved, lazily
//!   committed, with a real `mprotect`-backed guard where the OS allows it), the 4 077
//!   recovered data segments that build the image, and the asset tree that the streaming
//!   layer reads from disk.
//! * [`engine`] / [`stats`] / [`sys`] / [`cli`] / [`log`] — boot, the deterministic tick
//!   loop, the observable counters, the process-level plumbing and the flag surface.
//!
//! The binary (`src/main.rs`) is a thin driver over this library; the tests in
//! `src/*.rs` and `tests/` exercise the library directly.
//!
//! # Scope honesty
//!
//! This crate installs the engine's memory image and drives its frame counter. It does
//! **not** execute the engine's own `__wasm_call_ctors` / `main`: those need the 85 wasm
//! imports bound to real implementations (`engine-abi`) and the game assets.

pub mod assets;
pub mod cli;
pub mod engine;
pub mod log;
pub mod mem;
pub mod segments;
pub mod stats;
pub mod sys;

use std::net::SocketAddr;
use std::path::PathBuf;

use log::Level;

/// Runtime configuration, filled by [`cli::parse`].
#[derive(Debug, Clone)]
pub struct Config {
    /// Asset and userdata root.
    pub data_dir: PathBuf,
    /// Fixed simulation rate; the loop never drifts from it.
    pub tick_hz: u32,
    /// Worker threads for host dispatch.
    pub threads: usize,
    /// Control listener address.
    pub listen: SocketAddr,
    /// Concurrent control clients.
    pub max_clients: usize,
    /// Verbosity floor.
    pub log_level: Level,
    /// No renderer. Always true on a server; kept explicit so the flag surface is honest.
    pub headless: bool,
    /// No worker pool: tick thread plus control thread only.
    pub single_threaded: bool,
    /// Exit after this many ticks (test hook).
    pub max_ticks: Option<u64>,
    /// Linear-memory reservation, in bytes. The engine's image needs ~8 MiB; its heap
    /// reservation asks for ~2.17 GiB, so the default reserves 3 GiB like the browser build.
    pub memory_bytes: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("./userdata"),
            tick_hz: 30,
            threads: 1,
            listen: SocketAddr::from(([127, 0, 0, 1], 7777)),
            max_clients: 8,
            log_level: Level::Info,
            headless: true,
            single_threaded: false,
            max_ticks: None,
            memory_bytes: 3 * 1024 * 1024 * 1024,
        }
    }
}

/// Unified server error: one type at the crate root so `?` works across modules.
#[derive(Debug)]
pub enum ServerError {
    /// Linear memory could not be reserved, committed or accessed.
    Memory(mem::MemError),
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::Memory(e) => write!(f, "memory: {e}"),
        }
    }
}

impl std::error::Error for ServerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ServerError::Memory(e) => Some(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_reserve_three_gib_and_listen_locally() {
        let cfg = Config::default();
        assert_eq!(cfg.memory_bytes, 3 * 1024 * 1024 * 1024);
        assert_eq!(cfg.listen.port(), 7777);
        assert!(cfg.listen.ip().is_loopback(), "a server must not listen publicly by default");
    }

    #[test]
    fn cli_round_trips_into_this_config() {
        let parsed = cli::parse(["--tick-hz=45".to_string(), "--data-dir".to_string(), "d".to_string()])
            .expect("parse");
        match parsed {
            cli::Command::Run(cfg) => {
                assert_eq!(cfg.tick_hz, 45);
                assert_eq!(cfg.data_dir, PathBuf::from("d"));
            }
            _ => panic!("expected a run command"),
        }
    }
}