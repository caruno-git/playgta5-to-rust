use crate::lock;
use std::collections::{BTreeMap, VecDeque};
use std::sync::{Arc, Condvar, Mutex, OnceLock, Weak};
use std::time::{Duration, Instant};

#[derive(Default)]
struct Waiter {
    notified: Mutex<bool>,
    cv: Condvar,
}

#[derive(Default)]
struct Queue {
    waiters: Mutex<VecDeque<Arc<Waiter>>>,
}

fn queues() -> &'static Mutex<BTreeMap<usize, Weak<Queue>>> {
    static QUEUES: OnceLock<Mutex<BTreeMap<usize, Weak<Queue>>>> = OnceLock::new();
    QUEUES.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn queue(address: usize) -> Arc<Queue> {
    let mut slots = lock(queues());
    if let Some(q) = slots.get(&address).and_then(Weak::upgrade) {
        return q;
    }
    slots.retain(|_, q| q.strong_count() != 0);
    let q = Arc::new(Queue::default());
    slots.insert(address, Arc::downgrade(&q));
    q
}

pub(crate) fn wait(address: usize, matches: impl FnOnce() -> bool, timeout: i64) -> i32 {
    let q = queue(address);
    let mut waiters = lock(&q.waiters);
    if !matches() {
        return 1;
    }
    if timeout == 0 {
        return 2;
    }
    let waiter = Arc::new(Waiter::default());
    waiters.push_back(Arc::clone(&waiter));
    drop(waiters);
    let start = Instant::now();
    let mut notified = lock(&waiter.notified);
    while !*notified {
        if timeout < 0 {
            notified = waiter.cv.wait(notified).unwrap_or_else(|p| p.into_inner());
        } else {
            let remaining = Duration::from_nanos(timeout as u64).saturating_sub(start.elapsed());
            if remaining.is_zero() {
                break;
            }
            let (guard, _) = waiter
                .cv
                .wait_timeout(notified, remaining)
                .unwrap_or_else(|p| p.into_inner());
            notified = guard;
        }
    }
    if *notified {
        return 0;
    }
    drop(notified);
    let mut waiters = lock(&q.waiters);
    if let Some(i) = waiters.iter().position(|w| Arc::ptr_eq(w, &waiter)) {
        waiters.remove(i);
        2
    } else {
        0
    }
}

pub(crate) fn notify(address: usize, count: u32) -> u32 {
    let q = lock(queues()).get(&address).and_then(Weak::upgrade);
    let Some(q) = q else {
        return 0;
    };
    let mut waiters = lock(&q.waiters);
    let n = (count as usize).min(waiters.len());
    for _ in 0..n {
        if let Some(w) = waiters.pop_front() {
            *lock(&w.notified) = true;
            w.cv.notify_one();
        }
    }
    n as u32
}

pub(crate) fn waiting(address: usize) -> u32 {
    let q = lock(queues()).get(&address).and_then(Weak::upgrade);
    q.map(|q| lock(&q.waiters).len() as u32).unwrap_or(0)
}
