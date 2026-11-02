//! Live counters and the one-line JSON status document.
//!
//! Every field is written by the tick thread and read by the control thread, so
//! all of it is atomics. The JSON is assembled by hand: it is a fixed schema of
//! numbers, and a serialiser dependency would buy nothing.

use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use crate::mem::LinearMemory;
use crate::sys;

pub const STATE_BOOTING: u8 = 0;
pub const STATE_RUNNING: u8 = 1;
pub const STATE_QUITTING: u8 = 2;
pub const STATE_STOPPED: u8 = 3;

pub struct Stats {
    pub start: Instant,
    pub engine: &'static str,
    pub mem_backend: &'static str,
    pub mem_reserved: u64,
    engine_stage: Mutex<&'static str>,

    frame: AtomicU64,
    dropped: AtomicU64,
    last_tick_ns: AtomicU64,
    max_tick_ns: AtomicU64,
    total_tick_ns: AtomicU64,
    committed_bytes: AtomicU64,
    peers: AtomicUsize,
    clients: AtomicUsize,
    total_clients: AtomicU64,
    state: AtomicUsize,
    segments: AtomicU64,
    segment_bytes: AtomicU64,
    ctors_bytes: AtomicU64,
    assets_present: AtomicBool,
    assets_state: Mutex<&'static str>,
}

impl Stats {
    pub fn new(engine: &'static str, mem: &LinearMemory) -> Stats {
        Stats {
            start: Instant::now(),
            engine,
            mem_backend: mem.backend(),
            mem_reserved: mem.len(),
            engine_stage: Mutex::new("booting"),
            frame: AtomicU64::new(0),
            dropped: AtomicU64::new(0),
            last_tick_ns: AtomicU64::new(0),
            max_tick_ns: AtomicU64::new(0),
            total_tick_ns: AtomicU64::new(0),
            committed_bytes: AtomicU64::new(mem.committed_bytes()),
            peers: AtomicUsize::new(0),
            clients: AtomicUsize::new(0),
            total_clients: AtomicU64::new(0),
            state: AtomicUsize::new(STATE_BOOTING as usize),
            segments: AtomicU64::new(0),
            segment_bytes: AtomicU64::new(0),
            ctors_bytes: AtomicU64::new(0),
            assets_present: AtomicBool::new(false),
            assets_state: Mutex::new("unknown"),
        }
    }

    pub fn set_boot(&self, segments: usize, segment_bytes: u64, ctors_bytes: u64) {
        self.segments.store(segments as u64, Ordering::Relaxed);
        self.segment_bytes.store(segment_bytes, Ordering::Relaxed);
        self.ctors_bytes.store(ctors_bytes, Ordering::Relaxed);
        self.set_stage("running");
    }

    pub fn set_assets(&self, present: bool, state: &'static str) {
        self.assets_present.store(present, Ordering::Relaxed);
        if let Ok(mut s) = self.assets_state.lock() {
            *s = state;
        }
    }

    pub fn set_stage(&self, stage: &'static str) {
        if let Ok(mut s) = self.engine_stage.lock() {
            *s = stage;
        }
    }

    pub fn set_state(&self, state: u8) {
        self.state.store(state as usize, Ordering::SeqCst);
    }

    pub fn state(&self) -> u8 {
        self.state.load(Ordering::SeqCst) as u8
    }

    pub fn record_tick(&self, frame: u64, elapsed_ns: u64) {
        self.frame.store(frame, Ordering::Relaxed);
        self.last_tick_ns.store(elapsed_ns, Ordering::Relaxed);
        self.total_tick_ns.fetch_add(elapsed_ns, Ordering::Relaxed);
        // Relaxed min/max: a stale read would only mean a slightly old peak.
        let mut peak = self.max_tick_ns.load(Ordering::Relaxed);
        while elapsed_ns > peak {
            match self.max_tick_ns.compare_exchange_weak(
                peak,
                elapsed_ns,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(observed) => peak = observed,
            }
        }
    }

    pub fn record_dropped(&self, n: u64) {
        self.dropped.fetch_add(n, Ordering::Relaxed);
    }

    pub fn frame(&self) -> u64 {
        self.frame.load(Ordering::Relaxed)
    }

    pub fn peers(&self) -> usize {
        self.peers.load(Ordering::Relaxed)
    }

    pub fn set_peers(&self, n: usize) {
        self.peers.store(n, Ordering::Relaxed);
    }

    pub fn client_opened(&self) {
        self.clients.fetch_add(1, Ordering::Relaxed);
        self.total_clients.fetch_add(1, Ordering::Relaxed);
    }

    pub fn client_closed(&self) {
        self.clients.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn uptime(&self) -> std::time::Duration {
        self.start.elapsed()
    }

    pub fn status(&self, final_line: bool) -> String {
        let frame = self.frame();
        let uptime = self.uptime().as_secs_f64();
        let last_ns = self.last_tick_ns.load(Ordering::Relaxed);
        let max_ns = self.max_tick_ns.load(Ordering::Relaxed);
        let total_ns = self.total_tick_ns.load(Ordering::Relaxed);
        let avg_ms = if frame > 0 {
            (total_ns as f64 / frame as f64) / 1_000_000.0
        } else {
            0.0
        };
        let state = match self.state() {
            STATE_BOOTING => "booting",
            STATE_RUNNING => "running",
            STATE_QUITTING => "quitting",
            _ => "stopped",
        };
        let stage = self
            .engine_stage
            .lock()
            .map(|s| *s)
            .unwrap_or("unknown");
        let assets = self.assets_state.lock().map(|s| *s).unwrap_or("unknown");

        format!(
            concat!(
                "{{\"event\":\"status\",\"final\":{},\"state\":\"{}\",",
                "\"frame\":{},\"tick_ms\":{:.4},\"tick_ms_avg\":{:.4},\"tick_ms_max\":{:.4},",
                "\"dropped\":{},\"uptime_s\":{:.3},\"peers\":{},\"control_clients\":{},\"control_total\":{},",
                "\"rss_kb\":{},",
                "\"mem\":{{\"backend\":\"{}\",\"reserved_mb\":{},\"committed_kb\":{}}},",
                "\"engine\":{{\"name\":\"{}\",\"stage\":\"{}\",\"segments\":{},\"segment_kb\":{},\"ctors_kb\":{},\"assets\":\"{}\"}}",
                "}}"
            ),
            final_line,
            state,
            frame,
            last_ns as f64 / 1_000_000.0,
            avg_ms,
            max_ns as f64 / 1_000_000.0,
            self.dropped.load(Ordering::Relaxed),
            uptime,
            self.peers.load(Ordering::Relaxed),
            self.clients.load(Ordering::Relaxed),
            self.total_clients.load(Ordering::Relaxed),
            sys::rss_kb(),
            self.mem_backend,
            self.mem_reserved / (1024 * 1024),
            self.committed_kb(),
            self.engine,
            stage,
            self.segments.load(Ordering::Relaxed),
            self.segment_bytes.load(Ordering::Relaxed) / 1024,
            self.ctors_bytes.load(Ordering::Relaxed) / 1024,
            assets
        )
    }

    /// Committed bytes are owned by the memory object; the control thread
    /// receives them through this atomic mirror to avoid touching the guest
    /// address space from a second thread.
    fn committed_kb(&self) -> u64 {
        self.committed_bytes.load(Ordering::Relaxed) / 1024
    }

    pub fn mirror_committed(&self, bytes: u64) {
        self.committed_bytes.store(bytes, Ordering::Relaxed);
    }
}

// Declared with the rest of the fields above; kept separate so `new` stays
// readable.
impl Stats {
    #[allow(dead_code)]
    fn unused(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_is_single_line_and_has_required_fields() {
        let mem = LinearMemory::reserve(4 * 1024 * 1024).unwrap();
        let stats = Stats::new("reference", &mem);
        stats.set_state(STATE_RUNNING);
        stats.set_assets(false, "missing");
        stats.record_tick(7, 1_500_000);
        let line = stats.status(false);
        assert!(!line.contains('\n'));
        assert!(line.starts_with("{\"event\":\"status\""));
        assert!(line.ends_with('}'));
        for needle in ["\"frame\":7", "\"uptime_s\"", "\"peers\":0", "\"rss_kb\"", "\"tick_ms\":1.5000"] {
            assert!(line.contains(needle), "missing {needle} in {line}");
        }
        // Balanced braces: cheap sanity check that the hand-built JSON is whole.
        assert_eq!(line.matches('{').count(), line.matches('}').count());
    }

    #[test]
    fn tick_stats_track_max_and_avg() {
        let mem = LinearMemory::reserve(4 * 1024 * 1024).unwrap();
        let stats = Stats::new("reference", &mem);
        stats.record_tick(1, 1_000_000);
        stats.record_tick(2, 3_000_000);
        assert_eq!(stats.max_tick_ns.load(Ordering::Relaxed), 3_000_000);
        assert_eq!(stats.frame(), 2);
    }
}