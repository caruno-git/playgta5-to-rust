//! `memory.atomic.wait32` / `notify` — wake, timeout and not-equal.
//!
//! Unlike `tests/harness/futex.rs` in the determinism harness (a cooperative
//! model with virtual deadlines), this exercises the real thing: one OS thread
//! parks inside `a_wait32` and another releases it with `a_notify`. The three
//! documented outcomes are covered:
//!
//! * `1` — the word did not hold `expected`, so no wait happened
//! * `2` — the timeout elapsed with the word unchanged
//! * `0` — a matching `notify` released the waiter

use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use wasm_rt::{Memory, Trap, PAGE_SIZE};

const WORD: u64 = 0x100;

fn mem() -> Memory {
    Memory::new(1, 1).expect("memory")
}

/// Spin until `n` threads are parked on `addr` (or fail after ~2 s).
fn wait_until_parked(m: &Memory, addr: u64, n: u32) {
    let start = Instant::now();
    while m.waiting(addr) < n {
        assert!(
            start.elapsed() < Duration::from_secs(2),
            "waiter never parked: waiting={} want={n}",
            m.waiting(addr)
        );
        thread::sleep(Duration::from_millis(1));
    }
}

#[test]
fn wait_returns_not_equal_when_the_word_differs() {
    let mut m = mem();
    m.write_u32(WORD, 5).expect("store");
    assert_eq!(m.a_wait32(WORD, 7, -1).expect("wait"), 1);
    assert_eq!(
        m.waiting(WORD),
        0,
        "a failed wait must not register a waiter"
    );
}

#[test]
fn wait_times_out_and_unregisters() {
    let mut m = mem();
    m.write_u32(WORD, 5).expect("store");
    let t0 = Instant::now();
    let rc = m.a_wait32(WORD, 5, 20_000_000).expect("wait"); // 20 ms
    let elapsed = t0.elapsed();
    assert_eq!(rc, 2, "expected ETIMEDOUT code 2");
    assert!(
        elapsed >= Duration::from_millis(15),
        "timed out too early: {elapsed:?}"
    );
    assert_eq!(m.waiting(WORD), 0, "timed-out waiter must unregister");
}

#[test]
fn zero_timeout_returns_immediately_timed_out() {
    let mut m = mem();
    m.write_u32(WORD, 1).expect("store");
    let t0 = Instant::now();
    assert_eq!(m.a_wait32(WORD, 1, 0).expect("wait"), 2);
    assert!(t0.elapsed() < Duration::from_millis(200));
    assert_eq!(m.waiting(WORD), 0);
}

#[test]
fn notify_wakes_a_parked_waiter() {
    let mut owned = mem();
    owned.write_u32(WORD, 1).expect("store");
    let m = Arc::new(owned);

    let waiter = {
        let m = Arc::clone(&m);
        thread::spawn(move || m.a_wait32(WORD, 1, -1).expect("wait"))
    };

    wait_until_parked(&m, WORD, 1);
    assert_eq!(m.waiting(WORD), 1);

    assert_eq!(
        m.a_notify(WORD, 1),
        1,
        "notify reports the waiters it released"
    );
    assert_eq!(waiter.join().expect("join"), 0, "woken waits return 0");
    assert_eq!(m.waiting(WORD), 0);
}

#[test]
fn notify_wakes_all_waiters_when_count_is_negative() {
    let mut owned = mem();
    owned.write_u32(WORD, 3).expect("store");
    let m = Arc::new(owned);

    let mut handles = Vec::new();
    for _ in 0..3 {
        let m = Arc::clone(&m);
        handles.push(thread::spawn(move || {
            m.a_wait32(WORD, 3, -1).expect("wait")
        }));
    }
    wait_until_parked(&m, WORD, 3);
    assert_eq!(m.a_notify(WORD, -1), 3);
    for h in handles {
        assert_eq!(h.join().expect("join"), 0);
    }
    assert_eq!(m.waiting(WORD), 0);
}

#[test]
fn notify_without_waiters_returns_zero() {
    let m = mem();
    assert_eq!(m.a_notify(WORD, 1), 0);
    assert!(matches!(
        m.try_notify(0xDEAD_BEEF, -1),
        Err(Trap::OutOfBounds { .. })
    ));
    assert_eq!(m.a_notify(WORD, 0), 0);
}

#[test]
fn wait_on_out_of_bounds_address_traps() {
    let m = mem();
    assert!(matches!(
        m.a_wait32(PAGE_SIZE - 2, 0, -1),
        Err(Trap::OutOfBounds { .. })
    ));
}

#[test]
fn a_woken_waiter_that_is_notified_before_parking_sees_the_new_word() {
    // The wait protocol re-reads the word: notifying first then waiting with the
    // *old* expected value must fail fast instead of parking for the timeout.
    let mut m = mem();
    m.write_u32(WORD, 1).expect("store");
    m.write_u32(WORD, 2).expect("store");
    assert_eq!(m.a_wait32(WORD, 1, 50_000_000).expect("wait"), 1);
}

#[test]
fn partial_notify_does_not_turn_other_timeouts_into_wakes() {
    let m = Arc::new(mem());
    let mut handles = Vec::new();
    for _ in 0..3 {
        let m = Arc::clone(&m);
        handles.push(thread::spawn(move || {
            m.a_wait32(WORD, 0, 300_000_000).unwrap()
        }));
    }
    wait_until_parked(&m, WORD, 3);
    assert_eq!(m.a_notify(WORD, 1), 1);
    let mut results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    results.sort();
    assert_eq!(results, [0, 2, 2]);
}

#[test]
fn notify_counts_each_waiter_only_once() {
    let m = Arc::new(mem());
    let mut handles = Vec::new();
    for _ in 0..3 {
        let m = Arc::clone(&m);
        handles.push(thread::spawn(move || {
            m.a_wait32(WORD, 0, 1_000_000_000).unwrap()
        }));
    }
    wait_until_parked(&m, WORD, 3);
    for _ in 0..3 {
        assert_eq!(m.a_notify(WORD, 1), 1);
    }
    assert_eq!(m.a_notify(WORD, 1), 0);
    for h in handles {
        assert_eq!(h.join().unwrap(), 0);
    }
}

#[test]
fn separate_memories_never_share_waiters_at_the_same_offset() {
    let first = Arc::new(mem());
    let second = mem();
    let m = Arc::clone(&first);
    let handle = thread::spawn(move || m.a_wait32(WORD, 0, 1_000_000_000).unwrap());
    wait_until_parked(&first, WORD, 1);
    assert_eq!(second.a_notify(WORD, 1), 0);
    assert_eq!(first.a_notify(WORD, 1), 1);
    assert_eq!(handle.join().unwrap(), 0);
}

#[test]
fn atomic_alignment_and_notify_bounds_are_checked_even_with_zero_count() {
    let m = mem();
    assert!(matches!(
        m.a_wait32(WORD + 1, 0, 0),
        Err(Trap::UnalignedAtomic { .. })
    ));
    assert!(matches!(
        m.try_notify(WORD + 1, 0),
        Err(Trap::UnalignedAtomic { .. })
    ));
    assert!(matches!(
        m.try_notify(PAGE_SIZE, 0),
        Err(Trap::OutOfBounds { .. })
    ));
}
