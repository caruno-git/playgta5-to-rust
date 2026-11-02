//! Single-threaded model of the wasm futex path.
//!
//! A real server futex blocks an OS thread. The fixed-tick server cannot block
//! a tick, so the port's futex import is modelled as a *cooperative* wait:
//! `FUTEX_WAIT` enqueues a waiter with a virtual deadline and returns
//! `Waiting`; the tick loop calls [`FutexTable::expire`] at the start of each
//! tick and treats expired waiters as timed out. `FUTEX_WAKE` and
//! `FUTEX_REQUEUE` move tokens exactly like the kernel does for a single word.
//!
//! Every decision here is a pure function of the tick number and the words in
//! linear memory — no OS primitive, no wall clock, no thread scheduling.

use std::collections::BTreeMap;

use engine_runtime::hash::StateHasher;

/// `EAGAIN` — the futex word did not hold the expected value.
pub const EAGAIN: i32 = 11;
/// `ETIMEDOUT` — the virtual deadline elapsed.
pub const ETIMEDOUT: i32 = 110;

/// Result of a `FUTEX_WAIT`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitOutcome {
    /// Waiter enqueued; it will be reported by `expire` or `wake`.
    Waiting,
    /// The observed word differed from `expected`.
    ValueMismatch,
    /// `timeout_ticks == 0`, so the wait completed immediately.
    TimedOut,
}

impl WaitOutcome {
    /// Numeric result the wasm import returns.
    pub const fn code(self) -> i32 {
        match self {
            Self::Waiting => 0,
            Self::ValueMismatch => EAGAIN,
            Self::TimedOut => ETIMEDOUT,
        }
    }
}

/// A parked waiter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Waiter {
    /// Linear-memory address of the futex word.
    pub addr: u32,
    /// Tick at which the waiter times out.
    pub deadline: u64,
    /// Monotonic identity used by wake/requeue.
    pub token: u64,
}

/// Futex words plus waiters, wake log and token source.
#[derive(Debug, Clone)]
pub struct FutexTable {
    words: BTreeMap<u32, u32>,
    waiters: Vec<Waiter>,
    wake_log: Vec<u64>,
    next_token: u64,
}

impl FutexTable {
    /// Empty table.
    pub const fn new() -> Self {
        Self {
            words: BTreeMap::new(),
            waiters: Vec::new(),
            wake_log: Vec::new(),
            next_token: 1,
        }
    }

    /// Number of parked waiters.
    pub fn waiters(&self) -> usize {
        self.waiters.len()
    }

    /// Number of wake/timeout events logged.
    pub fn wake_events(&self) -> usize {
        self.wake_log.len()
    }

    /// Value of a futex word as this table sees it (kept separate from linear
    /// memory so the workload controls publication, as with `FUTEX_WAKE`).
    pub fn word(&self, addr: u32) -> u32 {
        self.words.get(&addr).copied().unwrap_or(0)
    }

    /// Publish a value, as a lock release would before `FUTEX_WAKE`.
    pub fn set_word(&mut self, addr: u32, value: u32) {
        self.words.insert(addr, value);
    }

    /// `FUTEX_WAIT(addr, expected, timeout_ticks)` evaluated at tick `now`.
    pub fn wait(
        &mut self,
        addr: u32,
        expected: u32,
        observed: u32,
        timeout_ticks: u64,
        now: u64,
    ) -> WaitOutcome {
        if observed != expected {
            return WaitOutcome::ValueMismatch;
        }
        if timeout_ticks == 0 {
            return WaitOutcome::TimedOut;
        }
        let token = self.next_token;
        self.next_token += 1;
        self.waiters.push(Waiter {
            addr,
            deadline: now.saturating_add(timeout_ticks),
            token,
        });
        WaitOutcome::Waiting
    }

    /// `FUTEX_WAKE(addr, count)`: returns how many waiters were released.
    pub fn wake(&mut self, addr: u32, count: u32) -> u32 {
        let mut released = 0u32;
        let mut kept = Vec::with_capacity(self.waiters.len());
        for waiter in self.waiters.drain(..) {
            if waiter.addr == addr && released < count {
                self.wake_log.push(waiter.token);
                released += 1;
            } else {
                kept.push(waiter);
            }
        }
        self.waiters = kept;
        released
    }

    /// `FUTEX_REQUEUE(addr, count, new_addr)`.
    pub fn requeue(&mut self, addr: u32, count: u32, new_addr: u32) -> u32 {
        let mut moved = 0u32;
        for waiter in &mut self.waiters {
            if waiter.addr == addr && moved < count {
                waiter.addr = new_addr;
                moved += 1;
            }
        }
        moved
    }

    /// Release waiters whose virtual deadline is at or before `now`.
    /// Returns their tokens in FIFO order.
    pub fn expire(&mut self, now: u64) -> Vec<u64> {
        let mut expired = Vec::new();
        let mut kept = Vec::with_capacity(self.waiters.len());
        for waiter in self.waiters.drain(..) {
            if waiter.deadline <= now {
                expired.push(waiter.token);
                self.wake_log.push(waiter.token);
            } else {
                kept.push(waiter);
            }
        }
        self.waiters = kept;
        expired
    }

    /// Stable digest of the whole table.
    pub fn digest(&self) -> u64 {
        let mut h = StateHasher::new();
        h.write_u64(self.words.len() as u64);
        for (addr, value) in &self.words {
            h.write_u32(*addr);
            h.write_u32(*value);
        }
        h.write_u64(self.waiters.len() as u64);
        for w in &self.waiters {
            h.write_u32(w.addr);
            h.write_u64(w.deadline);
            h.write_u64(w.token);
        }
        h.write_u64(self.wake_log.len() as u64);
        for token in &self.wake_log {
            h.write_u64(*token);
        }
        h.write_u64(self.next_token);
        h.finish()
    }
}

impl Default for FutexTable {
    fn default() -> Self {
        Self::new()
    }
}
