//! Engine backend seam.
//!
//! Boot order recovered from the shipped binary:
//!
//! ```text
//! wasm start -> __wasm_init_memory   (func 88, 4,077 memory.init ops)
//!            -> __wasm_call_ctors    (func 85, 953 KB inlined static init)
//!            -> main                 (func 103)
//!            -> per-frame loop via table slots 11/12
//!                 Main_Prologue        (func 105)
//!                 Main_OneLoopIteration(func 106)
//! ```
//!
//! [`Backend`] is the single place where "execute translated guest code" plugs
//! in. [`ReferenceBackend`] walks the same stages host-side and does the real
//! work this binary owns — memory reservation, segment replay, fixed-tick
//! accounting, status reporting — so the server is runnable and observable
//! before the generated bodies are linked. When `engine-runtime` lands its
//! `Runtime`, it implements this trait and `--engine` picks it; nothing else in
//! the binary changes.

use std::fmt;

use crate::assets::AssetReport;
use crate::log_info;
use crate::mem::LinearMemory;
use crate::segments::{self, SegmentBlob};

#[derive(Debug)]
pub enum EngineError {
    Segment(String),
    InvalidState(&'static str),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::Segment(e) => write!(f, "segment replay failed: {e}"),
            EngineError::InvalidState(s) => write!(f, "engine in unexpected state: {s}"),
        }
    }
}

impl std::error::Error for EngineError {}

#[derive(Debug, Clone)]
pub struct BootInfo {
    pub segments: usize,
    pub segment_bytes: u64,
    pub ctors_bytes: u64,
    pub stage: &'static str,
}

pub trait Backend {
    fn name(&self) -> &'static str;
    /// `__wasm_init_memory` -> `__wasm_call_ctors` -> `main`.
    fn boot(
        &mut self,
        mem: &LinearMemory,
        blob: Option<&SegmentBlob>,
    ) -> Result<BootInfo, EngineError>;
    /// One `Main_OneLoopIteration` (func 106) with fixed timestep accounting.
    fn tick(&mut self, frame: u64) -> Result<(), EngineError>;
    fn shutdown(&mut self) {}
}

/// Host-side reference loop. Executes every stage except the translated guest
/// bodies, which are not linked into this binary yet.
pub struct ReferenceBackend {
    assets: AssetReport,
    booted: bool,
    ticks: u64,
    warned_missing_assets: bool,
}

impl ReferenceBackend {
    pub fn new(assets: AssetReport) -> Self {
        ReferenceBackend {
            assets,
            booted: false,
            ticks: 0,
            warned_missing_assets: false,
        }
    }
}

impl Backend for ReferenceBackend {
    fn name(&self) -> &'static str {
        "reference"
    }

    fn boot(
        &mut self,
        mem: &LinearMemory,
        blob: Option<&SegmentBlob>,
    ) -> Result<BootInfo, EngineError> {
        // Stage 1: __wasm_init_memory (func 88).
        let report = match blob {
            Some(blob) => {
                let installed =
                    segments::install(mem, blob).map_err(|e| EngineError::Segment(e.to_string()))?;
                log_info!(
                    "boot",
                    "__wasm_init_memory: replayed {} segments ({} passive), {} bytes",
                    installed.segments,
                    installed.passive,
                    installed.bytes
                );
                installed
            }
            None => {
                log_info!(
                    "boot",
                    "__wasm_init_memory: no segment blob; linear memory left zeroed"
                );
                Default::default()
            }
        };

        // Stage 2: __wasm_call_ctors (func 85). The 953 KB static-init region
        // lives inside the segment blob; with no blob there is nothing to
        // replay, and the guest image would fault if it were linked. Say so
        // once rather than pretending.
        let ctors_bytes = if blob.is_some() { report.bytes } else { 0 };
        if blob.is_some() {
            log_info!("boot", "__wasm_call_ctors: static init {} bytes", ctors_bytes);
        } else {
            log_info!("boot", "__wasm_call_ctors: skipped (no linked guest image)");
        }

        // Stage 3: main (func 103) -> engine loop registration.
        log_info!(
            "boot",
            "main: entered; table slots 11/12 -> Main_Prologue(105) / Main_OneLoopIteration(106)"
        );

        if !self.assets.is_present() {
            log_info!("boot", "{}", self.assets.summary());
            log_info!(
                "boot",
                "continuing without assets: ticks advance the clock only, gameplay systems stay idle"
            );
        } else {
            log_info!("boot", "{}", self.assets.summary());
        }

        self.booted = true;
        Ok(BootInfo {
            segments: report.segments,
            segment_bytes: report.bytes,
            ctors_bytes,
            stage: "running",
        })
    }

    fn tick(&mut self, frame: u64) -> Result<(), EngineError> {
        if !self.booted {
            return Err(EngineError::InvalidState("tick before boot"));
        }
        self.ticks = self.ticks.saturating_add(1);
        if !self.assets.is_present() && !self.warned_missing_assets && frame >= 60 {
            self.warned_missing_assets = true;
            log_info!(
                "engine",
                "still ticking with no assets after {frame} frames (expected: clock-only loop)"
            );
        }
        Ok(())
    }

    fn shutdown(&mut self) {
        log_info!("engine", "shutdown after {} ticks", self.ticks);
        self.booted = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assets::probe;
    use std::path::Path;

    fn empty_assets() -> AssetReport {
        probe(Path::new("./definitely-not-a-real-data-dir"))
    }

    #[test]
    fn boot_without_blob_succeeds() {
        let mem = LinearMemory::reserve(4 * 1024 * 1024).unwrap();
        let mut backend = ReferenceBackend::new(empty_assets());
        let info = backend.boot(&mem, None).unwrap();
        assert_eq!(info.segments, 0);
        assert_eq!(info.ctors_bytes, 0);
        assert_eq!(info.stage, "running");
        backend.tick(0).unwrap();
        backend.shutdown();
    }

    #[test]
    fn tick_before_boot_errors() {
        let mut backend = ReferenceBackend::new(empty_assets());
        assert!(matches!(
            backend.tick(0),
            Err(EngineError::InvalidState("tick before boot"))
        ));
    }
}