//! The GPU command ring and the sink that drains it.
//!
//! The engine does not own a graphics API. It encodes draw state into a byte
//! ring in linear memory and calls `env.wasm_gpu_ring_submit`; the JS side
//! (`wgpu_worker.js`) walks the ring on the render thread. The types here are
//! the on-the-wire contract, shared by both sides, plus a sink that consumes
//! the ring and throws the work away so the simulation can run headless.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::host::HostResult;

/// One ring record, 4 × u32 = 16 bytes, little-endian, no padding.
///
/// `op` is a `GpuOp` discriminant from `generated/`; `a`, `b` are op-specific
/// operands (binding index, vertex count, buffer offset); `flags` carries the
/// per-frame bits the renderer needs (depth state, pass id).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct GpuCommand {
    pub op: u32,
    pub a: u32,
    pub b: u32,
    pub flags: u32,
}

impl GpuCommand {
    /// Encoded size in bytes; also the ring's slot stride.
    pub const ENCODED_LEN: usize = 16;

    pub const fn new(op: u32, a: u32, b: u32, flags: u32) -> Self {
        Self { op, a, b, flags }
    }

    pub fn encode(&self, out: &mut [u8; Self::ENCODED_LEN]) {
        out[0..4].copy_from_slice(&self.op.to_le_bytes());
        out[4..8].copy_from_slice(&self.a.to_le_bytes());
        out[8..12].copy_from_slice(&self.b.to_le_bytes());
        out[12..16].copy_from_slice(&self.flags.to_le_bytes());
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::ENCODED_LEN {
            return None;
        }
        let word = |i: usize| {
            let mut buf = [0u8; 4];
            buf.copy_from_slice(&bytes[i * 4..i * 4 + 4]);
            u32::from_le_bytes(buf)
        };
        Some(Self {
            op: word(0),
            a: word(1),
            b: word(2),
            flags: word(3),
        })
    }
}

/// Fixed-capacity record ring, engine producer side.
///
/// Mirrors the shared-memory layout `wgpu_worker.js` reads: a linear buffer
/// with a read cursor, a length and a dropped counter. A full ring drops the
/// newest record and bumps `dropped_records` — the renderer would rather miss a
/// frame's tail than block the simulation thread on a stalled GPU.
#[derive(Debug)]
pub struct GpuRing {
    slot: usize,
    capacity: usize,
    buf: Vec<u8>,
    head: usize,
    len: usize,
    dropped: u64,
}

impl GpuRing {
    pub fn new(capacity_records: usize) -> Self {
        let capacity = capacity_records.max(1);
        Self {
            slot: GpuCommand::ENCODED_LEN,
            capacity,
            buf: vec![0u8; capacity * GpuCommand::ENCODED_LEN],
            head: 0,
            len: 0,
            dropped: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn dropped_records(&self) -> u64 {
        self.dropped
    }

    /// Append a command. `false` means the ring was full and the command was
    /// dropped (counted).
    pub fn push(&mut self, command: GpuCommand) -> bool {
        if self.len == self.capacity {
            self.dropped = self.dropped.saturating_add(1);
            return false;
        }
        let index = (self.head + self.len) % self.capacity;
        let start = index * self.slot;
        let mut encoded = [0u8; GpuCommand::ENCODED_LEN];
        command.encode(&mut encoded);
        self.buf[start..start + self.slot].copy_from_slice(&encoded);
        self.len += 1;
        true
    }

    /// Pop the oldest command.
    pub fn pop(&mut self) -> Option<GpuCommand> {
        if self.len == 0 {
            return None;
        }
        let start = self.head * self.slot;
        let command = GpuCommand::decode(&self.buf[start..start + self.slot]);
        self.head = (self.head + 1) % self.capacity;
        self.len -= 1;
        command
    }

    /// Drop everything without decoding (frame abort / device reset).
    pub fn clear(&mut self) {
        self.head = 0;
        self.len = 0;
    }
}

/// `env.wasm_gpu_ring_submit` — one call per engine frame, discards or forwards
/// the ring's contents.
pub trait GpuSink: Send + Sync {
    /// Drain `ring`; returns how many commands were consumed.
    fn submit(&self, ring: &mut GpuRing) -> HostResult<usize>;
}

/// Sink for builds without a renderer: drains the ring so producers never block
/// and keeps counters for diagnostics.
#[derive(Debug, Default)]
pub struct NullGpuSink {
    drained: AtomicU64,
    frames: AtomicU64,
}

impl NullGpuSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn drained(&self) -> u64 {
        self.drained.load(Ordering::Relaxed)
    }

    pub fn frames(&self) -> u64 {
        self.frames.load(Ordering::Relaxed)
    }
}

impl GpuSink for NullGpuSink {
    fn submit(&self, ring: &mut GpuRing) -> HostResult<usize> {
        let mut count = 0;
        while ring.pop().is_some() {
            count += 1;
        }
        self.drained
            .fetch_add(count as u64, Ordering::Relaxed);
        self.frames.fetch_add(1, Ordering::Relaxed);
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_is_fifo_and_wraps() {
        let mut ring = GpuRing::new(3);
        assert!(ring.is_empty());
        for i in 0..3 {
            assert!(ring.push(GpuCommand::new(i, i + 10, 0, 0)));
        }
        assert_eq!(ring.len(), 3);
        assert!(!ring.push(GpuCommand::new(99, 0, 0, 0)));
        assert_eq!(ring.dropped_records(), 1);

        assert_eq!(ring.pop(), Some(GpuCommand::new(0, 10, 0, 0)));
        assert!(ring.push(GpuCommand::new(3, 13, 0, 0)));
        assert_eq!(ring.len(), 3);
        let order: Vec<u32> = std::iter::from_fn(|| ring.pop()).map(|c| c.op).collect();
        assert_eq!(order, vec![1, 2, 3]);
        assert_eq!(ring.pop(), None);
    }

    #[test]
    fn record_encoding_is_little_endian_and_reversible() {
        let command = GpuCommand::new(0x0102_0304, 7, 8, 0xFFFF_0000);
        let mut buf = [0u8; GpuCommand::ENCODED_LEN];
        command.encode(&mut buf);
        assert_eq!(&buf[0..4], &[0x04, 0x03, 0x02, 0x01]);
        assert_eq!(GpuCommand::decode(&buf), Some(command));
        assert_eq!(GpuCommand::decode(&buf[..15]), None);
    }

    #[test]
    fn null_sink_drains_and_counts() {
        let sink = NullGpuSink::new();
        let mut ring = GpuRing::new(8);
        ring.push(GpuCommand::new(1, 0, 0, 0));
        ring.push(GpuCommand::new(2, 0, 0, 0));
        assert_eq!(sink.submit(&mut ring).expect("submit"), 2);
        assert!(ring.is_empty());
        assert_eq!(sink.drained(), 2);
        assert_eq!(sink.frames(), 1);
        assert_eq!(sink.submit(&mut ring).expect("empty"), 0);
        assert_eq!(sink.frames(), 2);
    }

    #[test]
    fn clear_abandons_pending_work() {
        let mut ring = GpuRing::new(2);
        ring.push(GpuCommand::new(1, 0, 0, 0));
        ring.clear();
        assert!(ring.is_empty());
        assert_eq!(ring.pop(), None);
        assert!(ring.push(GpuCommand::new(5, 0, 0, 0)));
        assert_eq!(ring.pop().map(|c| c.op), Some(5));
    }
}