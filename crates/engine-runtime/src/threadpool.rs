//! Job-pool shim matching what the engine's `JobManager` expects.
//!
//! The recovered engine calls into a pthread-backed job manager from the
//! single-threaded top level (`Main_OneLoopIteration`, func_idx 106) and then
//! waits for the batch before the frame continues — that is why the pool here
//! offers *batch* semantics with a blocking completion point rather than a
//! fire-and-forget executor.
//!
//! Two guarantees matter for a server:
//!
//! 1. **Deterministic completion.** [`ThreadPool::map`] returns results in
//!    submission order no matter which worker finished first, because results
//!    are written into per-index slots and read back by index. Scheduling order
//!    is never observable.
//! 2. **`single_threaded` mode.** Jobs execute inline, in submission order, on
//!    the calling thread; no worker threads are spawned at all. A server sets
//!    this to make the whole process single-threaded and therefore bit-for-bit
//!    reproducible.
//!
//! Jobs are `FnOnce() + Send + 'static`. Panics are caught and reported as
//! [`PoolError::JobPanicked`] instead of killing a worker.

use std::cell::Cell;
use std::collections::VecDeque;
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

thread_local! {
    /// Depth of nested job execution on this thread; `> 0` means we are inside
    /// a job and must not block on the pool.
    static IN_JOB: Cell<u32> = const { Cell::new(0) };
}

fn in_job() -> bool {
    IN_JOB.with(|c| c.get() > 0)
}

fn enter_job() {
    IN_JOB.with(|c| c.set(c.get() + 1));
}

fn leave_job() {
    IN_JOB.with(|c| c.set(c.get().saturating_sub(1)));
}

/// Pool failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoolError {
    /// The OS refused to create a worker thread.
    Spawn(String),
    /// A job panicked; the payload is the panic message when it was a string.
    JobPanicked(String),
    /// `wait_idle` was called from inside a job — that would deadlock, so it is
    /// reported instead.
    NestedWait,
    /// The pool has been shut down; new jobs are rejected.
    Shutdown,
}

impl fmt::Display for PoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Spawn(msg) => write!(f, "failed to spawn worker: {msg}"),
            Self::JobPanicked(msg) => write!(f, "job panicked: {msg}"),
            Self::NestedWait => write!(f, "wait_idle called from inside a job"),
            Self::Shutdown => write!(f, "thread pool is shut down"),
        }
    }
}

impl std::error::Error for PoolError {}

struct State {
    jobs: VecDeque<Job>,
    pending: usize,
    shutdown: bool,
    panic: Option<String>,
}

struct Shared {
    state: Mutex<State>,
    ready: Condvar,
    done: Condvar,
}

fn lock(shared: &Shared) -> MutexGuard<'_, State> {
    // Poisoning only happens if a worker died mid-lock; the queue is still
    // structurally sound, so we recover instead of panicking.
    match shared.state.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn wait<'a>(cv: &Condvar, guard: MutexGuard<'a, State>) -> MutexGuard<'a, State> {
    match cv.wait(guard) {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<&'static str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "non-string panic payload".to_string()
    }
}

fn run_job(shared: &Shared, job: Job) {
    enter_job();
    let outcome = catch_unwind(AssertUnwindSafe(job));
    leave_job();
    if let Err(payload) = outcome {
        let msg = panic_message(&*payload);
        let mut st = lock(shared);
        if st.panic.is_none() {
            st.panic = Some(msg);
        }
    }
}

fn worker_loop(shared: Arc<Shared>) {
    loop {
        let job = {
            let mut st = lock(&shared);
            while st.jobs.is_empty() && !st.shutdown {
                st = wait(&shared.ready, st);
            }
            match st.jobs.pop_front() {
                Some(job) => job,
                // shutdown with an empty queue
                None => return,
            }
        };
        run_job(&shared, job);
        let mut st = lock(&shared);
        st.pending = st.pending.saturating_sub(1);
        if st.pending == 0 {
            shared.done.notify_all();
        }
    }
}

/// A fixed-size worker pool with a FIFO job queue and a deterministic batch API.
pub struct ThreadPool {
    requested_workers: usize,
    single_threaded: bool,
    shared: Arc<Shared>,
    handles: Vec<JoinHandle<()>>,
}

impl fmt::Debug for ThreadPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ThreadPool")
            .field("requested_workers", &self.requested_workers)
            .field("single_threaded", &self.single_threaded)
            .field("spawned_workers", &self.handles.len())
            .finish()
    }
}

impl ThreadPool {
    /// Create a pool.
    ///
    /// With `single_threaded == true` no threads are created and every job runs
    /// inline on the submitting thread, in submission order. Otherwise
    /// `workers.max(1)` worker threads are spawned.
    pub fn new(requested_workers: usize, single_threaded: bool) -> Result<Self, PoolError> {
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                jobs: VecDeque::new(),
                pending: 0,
                shutdown: false,
                panic: None,
            }),
            ready: Condvar::new(),
            done: Condvar::new(),
        });

        let mut handles = Vec::new();
        if !single_threaded {
            let n = requested_workers.max(1);
            for i in 0..n {
                let shared = Arc::clone(&shared);
                let handle = thread::Builder::new()
                    .name(format!("engine-job-{i}"))
                    .spawn(move || worker_loop(shared))
                    .map_err(|e| PoolError::Spawn(e.to_string()))?;
                handles.push(handle);
            }
        }

        Ok(Self {
            requested_workers,
            single_threaded,
            shared,
            handles,
        })
    }

    /// Worker count the caller asked for.
    pub const fn requested_workers(&self) -> usize {
        self.requested_workers
    }

    /// Worker count actually available to run jobs in parallel (1 in
    /// single-threaded mode).
    pub fn workers(&self) -> usize {
        if self.single_threaded {
            1
        } else {
            self.requested_workers.max(1)
        }
    }

    /// Threads actually spawned (0 in single-threaded mode).
    pub fn spawned_workers(&self) -> usize {
        self.handles.len()
    }

    /// True when the pool is in deterministic single-threaded mode.
    pub const fn is_single_threaded(&self) -> bool {
        self.single_threaded
    }

    /// True when completion order is deterministic by construction.
    ///
    /// Only single-threaded mode guarantees *scheduling* determinism; parallel
    /// mode guarantees *observable* determinism through [`Self::map`].
    pub const fn is_deterministic(&self) -> bool {
        self.single_threaded
    }

    /// Submit one job. In single-threaded mode it runs before this returns.
    ///
    /// # Errors
    /// Propagates a panic from an inline job, or [`PoolError::Shutdown`].
    pub fn spawn<F>(&self, job: F) -> Result<(), PoolError>
    where
        F: FnOnce() + Send + 'static,
    {
        if self.single_threaded {
            run_job(&self.shared, Box::new(job));
            return self.take_panic();
        }

        let mut st = lock(&self.shared);
        if st.shutdown {
            return Err(PoolError::Shutdown);
        }
        st.pending += 1;
        st.jobs.push_back(Box::new(job));
        drop(st);
        self.shared.ready.notify_one();
        Ok(())
    }

    /// Submit a batch and block until every job in it has finished.
    ///
    /// # Errors
    /// [`PoolError::NestedWait`] if called from inside a job.
    pub fn spawn_batch<I>(&self, jobs: I) -> Result<(), PoolError>
    where
        I: IntoIterator<Item = Job>,
    {
        if in_job() {
            return Err(PoolError::NestedWait);
        }
        for job in jobs {
            self.spawn(job)?;
        }
        self.wait_idle()
    }

    /// Block until the queue is empty and no job is running.
    ///
    /// # Errors
    /// [`PoolError::NestedWait`] when called from inside a job (that would
    /// deadlock when the pool has a single worker), or
    /// [`PoolError::JobPanicked`] if a job panicked.
    pub fn wait_idle(&self) -> Result<(), PoolError> {
        if in_job() {
            return Err(PoolError::NestedWait);
        }
        if self.single_threaded {
            // Inline execution: nothing can be outstanding.
            return self.take_panic();
        }
        let mut st = lock(&self.shared);
        while st.pending > 0 {
            st = wait(&self.shared.done, st);
        }
        drop(st);
        self.take_panic()
    }

    /// Apply `f` to every item and return the results **in input order**.
    ///
    /// This is the determinism-critical API: a tick may use the pool freely,
    /// and as long as it consumes the results through this function the
    /// observable behaviour does not depend on which worker won a race.
    ///
    /// # Errors
    /// [`PoolError::NestedWait`] when called from inside a job,
    /// [`PoolError::JobPanicked`] when a job panicked.
    pub fn map<T, R, F>(&self, items: Vec<T>, f: F) -> Result<Vec<R>, PoolError>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: Fn(T) -> R + Send + Sync + 'static,
    {
        if in_job() {
            return Err(PoolError::NestedWait);
        }
        let slots: Arc<Mutex<Vec<Option<R>>>> =
            Arc::new(Mutex::new((0..items.len()).map(|_| None).collect()));
        let f = Arc::new(f);

        if self.single_threaded {
            for (index, item) in items.into_iter().enumerate() {
                let func = Arc::clone(&f);
                let slots = Arc::clone(&slots);
                run_job(
                    &self.shared,
                    Box::new(move || store(&slots, index, func(item))),
                );
            }
        } else {
            for (index, item) in items.into_iter().enumerate() {
                let func = Arc::clone(&f);
                let slots = Arc::clone(&slots);
                self.spawn(move || store(&slots, index, func(item)))?;
            }
            self.wait_idle()?;
        }

        drain(&slots)
    }
}

fn store<R>(slots: &Mutex<Vec<Option<R>>>, index: usize, value: R) {
    match slots.lock() {
        Ok(mut guard) => {
            if let Some(slot) = guard.get_mut(index) {
                *slot = Some(value);
            }
        }
        Err(poisoned) => {
            let mut guard = poisoned.into_inner();
            if let Some(slot) = guard.get_mut(index) {
                *slot = Some(value);
            }
        }
    }
}

fn drain<R>(slots: &Mutex<Vec<Option<R>>>) -> Result<Vec<R>, PoolError> {
    let mut guard = match slots.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let mut out = Vec::with_capacity(guard.len());
    for (index, slot) in guard.iter_mut().enumerate() {
        match slot.take() {
            Some(value) => out.push(value),
            None => {
                return Err(PoolError::JobPanicked(format!(
                    "job {index} produced no result"
                )))
            }
        }
    }
    Ok(out)
}

impl ThreadPool {
    fn take_panic(&self) -> Result<(), PoolError> {
        let mut st = lock(&self.shared);
        match st.panic.take() {
            Some(msg) => Err(PoolError::JobPanicked(msg)),
            None => Ok(()),
        }
    }

    /// Stop accepting jobs and join every worker.
    pub fn shutdown(&mut self) {
        {
            let mut st = lock(&self.shared);
            st.shutdown = true;
        }
        self.shared.ready.notify_all();
        for handle in std::mem::take(&mut self.handles) {
            let _ = handle.join();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex as StdMutex;

    #[test]
    fn single_threaded_spawns_no_threads_and_runs_inline() {
        let pool = ThreadPool::new(8, true).expect("pool");
        assert_eq!(pool.spawned_workers(), 0);
        assert_eq!(pool.workers(), 1);
        assert!(pool.is_deterministic());

        let order = Arc::new(StdMutex::new(Vec::new()));
        for i in 0..8usize {
            let order = Arc::clone(&order);
            pool.spawn(move || {
                let mut guard = match order.lock() {
                    Ok(g) => g,
                    Err(p) => p.into_inner(),
                };
                guard.push(i);
            })
            .expect("spawn");
        }
        let seen = match order.lock() {
            Ok(g) => g.clone(),
            Err(p) => p.into_inner().clone(),
        };
        assert_eq!(seen, (0..8).collect::<Vec<_>>());
    }

    #[test]
    fn parallel_pool_uses_all_workers() {
        let pool = ThreadPool::new(4, false).expect("pool");
        assert_eq!(pool.spawned_workers(), 4);
        let counter = Arc::new(AtomicUsize::new(0));
        for _ in 0..64 {
            let counter = Arc::clone(&counter);
            pool.spawn(move || {
                counter.fetch_add(1, Ordering::SeqCst);
            })
            .expect("spawn");
        }
        pool.wait_idle().expect("idle");
        assert_eq!(counter.load(Ordering::SeqCst), 64);
    }

    #[test]
    fn map_returns_input_order_in_both_modes() {
        let inputs: Vec<u64> = (0..257).collect();
        for single in [true, false] {
            let pool = ThreadPool::new(4, single).expect("pool");
            let out = pool
                .map(inputs.clone(), |v| v * 3 + 1)
                .expect("map must succeed");
            assert_eq!(out, inputs.iter().map(|v| v * 3 + 1).collect::<Vec<_>>());
        }
    }

    #[test]
    fn nested_wait_is_reported_not_deadlocked() {
        let pool = Arc::new(ThreadPool::new(1, false).expect("pool"));
        let inner = Arc::clone(&pool);
        let slot: Arc<StdMutex<Option<Result<(), PoolError>>>> = Arc::new(StdMutex::new(None));
        let sink = Arc::clone(&slot);

        pool.spawn(move || {
            // A job asking the pool to drain would deadlock on this worker;
            // the thread-local guard turns it into an error instead.
            let err = inner.wait_idle();
            if let Ok(mut guard) = sink.lock() {
                *guard = Some(err);
            }
        })
        .expect("spawn");

        pool.wait_idle().expect("outer wait must succeed");
        let observed = match slot.lock() {
            Ok(guard) => guard.clone(),
            Err(poisoned) => poisoned.into_inner().clone(),
        };
        assert_eq!(observed, Some(Err(PoolError::NestedWait)));
    }

    #[test]
    fn panicking_job_is_reported() {
        let pool = ThreadPool::new(2, false).expect("pool");
        pool.spawn(|| panic!("boom")).expect("submit");
        let err = pool.wait_idle().expect_err("must report panic");
        assert!(matches!(err, PoolError::JobPanicked(_)));
    }
}
