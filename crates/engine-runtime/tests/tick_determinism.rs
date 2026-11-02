//! End-to-end determinism tests for the fixed-timestep runtime.
//!
//! These are the acceptance tests for the server loop:
//!
//! 1. 1000 virtual-clock ticks produce exactly 1000 frames and a constant delta.
//! 2. A slow consumer never drifts the frame counter.
//! 3. `single_threaded` mode gives byte-identical state hashes across two runs.
//! 4. `run(host, tick_fn)` honours the stop flag at a frame boundary.

use engine_runtime::{
    run, Clock, Host, Scheduler, SlowTickPolicy, StateHasher, ThreadPool, TickConfig, TickCtx,
    VirtualTime,
};

const STEP: u64 = 16_666_667; // 60 Hz step, TickConfig::at(60).step_ns()

fn cfg_catch_up() -> TickConfig {
    let mut cfg = TickConfig::at(60);
    cfg.max_catch_up_steps = 4;
    cfg.slow_tick = SlowTickPolicy::CatchUp;
    cfg
}

/// Trivial but order-sensitive tick state: the kind of thing a real subsystem
/// would hash at the end of a run.
#[derive(Default, Clone)]
struct State {
    position: u64,
    velocity: u64,
    counter: u32,
}

impl State {
    fn hash(&self, clock: &Clock) -> u64 {
        let mut h = StateHasher::new();
        h.write_u64(self.position);
        h.write_u64(self.velocity);
        h.write_u32(self.counter);
        h.write_u64(clock.ticks());
        h.write_i32(clock.frame());
        h.write_u64(clock.delta_ns());
        h.finish()
    }
}

/// Deterministic "physics": integer only, no floats, no wall clock, no map
/// iteration order.
fn step_state(state: &mut State, delta_ns: u64) {
    state.velocity = state.velocity.wrapping_add(delta_ns / 1_000);
    state.position = state.position.wrapping_add(state.velocity);
    state.counter = state.counter.wrapping_add(1);
}

#[test]
fn thousand_ticks_virtual_clock_gives_exactly_1000_frames_and_stable_delta() {
    let mut sched = Scheduler::new(TickConfig::at(60)).expect("scheduler");
    let mut clock = Clock::new();
    assert_eq!(sched.step_ns(), STEP);

    let mut observed_frames = Vec::with_capacity(1000);
    let mut observed_deltas = Vec::with_capacity(1000);
    let total = STEP * 1000;
    let mut fed = 0u64;
    let mut i = 0u64;

    // Deliberately irregular pump granularity (4..=8 chunks per step).
    while fed < total {
        let mut chunk = STEP / 4 + (i % 5) * (STEP / 8);
        if fed + chunk > total {
            chunk = total - fed;
        }
        fed += chunk;
        i += 1;
        let report = sched.pump(chunk, &mut clock, |c, index| {
            assert_eq!(c.frame() as u64, index, "frame and tick index must agree");
            observed_frames.push(index);
            observed_deltas.push(c.delta_ns());
        });
        assert!(!report.capped, "1000 steps at cap 8 must never cap");
        assert_eq!(report.dropped_ns, 0);
    }

    assert_eq!(sched.ticks(), 1000);
    assert_eq!(clock.frame(), 1000);
    assert_eq!(clock.ticks(), 1000);
    assert_eq!(clock.now_ns(), total);
    assert_eq!(observed_frames, (1..=1000).collect::<Vec<u64>>());
    assert!(
        observed_deltas.iter().all(|d| *d == STEP),
        "every tick must see the fixed step"
    );
    // delta is constant and exactly 1/60 s (within the documented 1 ns rounding)
    assert!((clock.delta_secs() - 1.0 / 60.0).abs() < 1e-7);
}

#[test]
fn slow_consumer_never_drifts_the_frame_count() {
    // Catch-up policy, cap 4 ticks per pump. Every 10th pump offers 6 steps
    // (more than the cap can pay for), the rest offer 2. Backlog is retained,
    // so the total tick count is exactly floor(total_elapsed / step) — the
    // definition of "no drift".
    let mut sched = Scheduler::new(cfg_catch_up()).expect("scheduler");
    let mut clock = Clock::new();
    let mut total = 0u64;

    for pump in 0..1000u64 {
        let elapsed = if pump % 10 == 0 { STEP * 6 } else { STEP * 2 };
        total += elapsed;
        let report = sched.pump(elapsed, &mut clock, |c, index| {
            assert_eq!(c.frame() as u64, index);
            assert_eq!(c.delta_ns(), STEP);
        });
        assert_eq!(
            report.dropped_ns, 0,
            "catch-up must not drop within ceiling"
        );
        assert_eq!(clock.frame() as u64, sched.ticks());
    }

    assert_eq!(sched.ticks(), total / STEP);
    assert_eq!(clock.frame() as u64, total / STEP);
    assert_eq!(clock.now_ns(), sched.ticks() * STEP);
    assert!(sched.overruns() > 0, "cap must have been exercised");
    assert_eq!(sched.backlog_ns(), 0, "the burst cycle drains fully");

    // Slow tick body: after the burst, single-step pumps keep the counter
    // contiguous — no phantom frames, no skipped frames.
    let before = clock.frame() as u64;
    for _ in 0..50 {
        sched.pump(STEP, &mut clock, |c, index| {
            assert_eq!(c.frame() as u64, index);
        });
    }
    assert_eq!(clock.frame() as u64, before + 50);
    assert_eq!(clock.frame() as u64, sched.ticks());
}

#[test]
fn drop_excess_bounds_the_backlog_without_losing_frame_continuity() {
    let mut cfg = TickConfig::at(60);
    cfg.max_catch_up_steps = 2;
    cfg.slow_tick = SlowTickPolicy::DropExcess;
    let mut sched = Scheduler::new(cfg).expect("scheduler");
    let mut clock = Clock::new();

    // Each pump offers 10 steps; only 2 may run, 8 are dropped.
    for _ in 0..100 {
        let report = sched.pump(STEP * 10, &mut clock, |_, _| {});
        assert_eq!(report.ticks_run, 2);
        assert_eq!(report.dropped_ns, STEP * 8);
        assert_eq!(clock.frame() as u64, sched.ticks());
    }
    assert_eq!(sched.ticks(), 200);
    assert_eq!(sched.dropped_ns(), STEP * 800);

    // Frame numbers stay contiguous across the dilation: a 3-step pump with a
    // cap of 2 runs 2 frames (200 -> 202), then drops the remaining step to
    // reset phase (that is what "drop excess" means).
    let mut seen = Vec::new();
    sched.pump(STEP * 3, &mut clock, |c, _| seen.push(c.frame()));
    assert_eq!(seen, vec![201, 202]);
    assert_eq!(clock.frame(), 202);
    assert_eq!(sched.backlog_ns(), 0);
    assert_eq!(sched.dropped_ns(), STEP * 801);
}

#[test]
fn single_threaded_mode_is_byte_identical_across_runs() {
    let run_once = || {
        let mut host = Host::with_virtual_time(
            TickConfig::at(60),
            ThreadPool::new(8, true).expect("pool"),
            1_000_000, // 1 ms virtual pump
        )
        .expect("host");
        assert_eq!(host.pool.spawned_workers(), 0);
        assert!(host.pool.is_deterministic());

        let mut state = State::default();
        let stats = run(&mut host, |clock, ctx: &TickCtx<'_>| {
            assert_eq!(clock.delta_ns(), STEP);
            step_state(&mut state, clock.delta_ns());
            if ctx.tick_index >= 500 {
                ctx.control.stop();
            }
        });
        assert_eq!(stats.ticks, 500);
        assert_eq!(stats.frame, 500);
        assert_eq!(stats.dropped_ns, 0, "1 ms pumps must never cap");
        (state.hash(&host.clock), stats, state.counter)
    };

    let (hash_a, stats_a, count_a) = run_once();
    let (hash_b, stats_b, count_b) = run_once();
    assert_eq!(count_a, 500);
    assert_eq!(count_b, 500);
    assert_eq!(hash_a, hash_b, "state hash must be identical across runs");
    assert_eq!(stats_a, stats_b);
    assert_ne!(hash_a, 0);
}

#[test]
fn parallel_pool_results_are_observationally_identical_to_single_threaded() {
    // Same tick, same input batch, 4 real workers vs inline execution: the
    // batch API restores submission order, so the state hash matches.
    let run_with = |single: bool| {
        let mut host = Host::with_virtual_time(
            TickConfig::at(60),
            ThreadPool::new(4, single).expect("pool"),
            1_000_000,
        )
        .expect("host");
        let mut state = State::default();
        run(&mut host, |clock, ctx: &TickCtx<'_>| {
            let batch: Vec<u64> = (0..64).map(|i| i * 7 + ctx.tick_index).collect();
            let results = ctx.pool.map(batch, |v| v.wrapping_mul(3)).expect("map");
            let acc = results.iter().fold(0u64, |a, v| a.wrapping_add(*v));
            state.velocity = state.velocity.wrapping_add(acc);
            step_state(&mut state, clock.delta_ns());
            if ctx.tick_index >= 200 {
                ctx.control.stop();
            }
        });
        state.hash(&host.clock)
    };

    assert_eq!(run_with(true), run_with(false));
}

#[test]
fn run_driver_reports_frames_equal_to_ticks() {
    let mut host = Host::with_virtual_time(
        TickConfig::at(60),
        ThreadPool::new(2, true).expect("pool"),
        250_000,
    )
    .expect("host");
    let stats = run(&mut host, |clock, ctx| {
        assert_eq!(clock.frame(), ctx.frame);
        if ctx.tick_index >= 1000 {
            ctx.control.stop();
        }
    });
    assert_eq!(stats.ticks, 1000);
    assert_eq!(stats.frame, 1000);
    assert_eq!(host.clock.now_ns(), STEP * 1000);
    assert!(!stats.halted);
}

#[test]
fn configurable_rate_changes_step_only() {
    for hz in [30u32, 60, 120, 240] {
        let mut sched = Scheduler::new(TickConfig::at(hz)).expect("scheduler");
        let mut clock = Clock::new();
        let step = sched.step_ns();
        for _ in 0..hz {
            sched.pump(step, &mut clock, |_, _| {});
        }
        assert_eq!(sched.ticks(), hz as u64);
        assert_eq!(clock.frame(), hz as i32);
        assert_eq!(clock.now_ns(), step * hz as u64);
    }
}

#[test]
fn virtual_time_pump_is_the_only_thing_that_moves_time() {
    let mut vt = VirtualTime::new(1_000_000);
    use engine_runtime::TimeSource;
    assert_eq!(vt.now_ns(), 0);
    vt.idle();
    assert_eq!(vt.now_ns(), 1_000_000);
    vt.advance(5);
    assert_eq!(vt.now_ns(), 1_000_005);
}
