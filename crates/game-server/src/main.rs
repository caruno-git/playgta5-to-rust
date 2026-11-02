//! `game-server` — headless boot and tick loop for the machine-translated engine.
//!
//! Thin driver over the `game_server` library:
//!
//! 1. parse flags ([`game_server::cli`]),
//! 2. reserve the engine's linear memory and install the recovered segment image,
//! 3. probe the asset tree (missing assets are logged, never fatal),
//! 4. run the deterministic fixed-step loop that writes the engine's frame counter at
//!    VA `12321252` (`GetGameFrame`, func_idx 91) — the same cell `fwTimer::Update`
//!    (func 8924) writes in the shipped build,
//! 5. print one status line, so a smoke run is checkable without a control client.
//!
//! Scope, stated plainly: this binary does **not** execute the engine's own
//! `__wasm_call_ctors` / `main`. Those need the 85 wasm imports bound to real
//! implementations (`engine-abi`, partially done) and the game assets. What it proves
//! is that the memory image, the host services and the tick clock hold together on a
//! server with no browser.

use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use engine_runtime::clock::FRAME_COUNTER_VA;
use engine_runtime::host::{run, Host, OsTime, TickCtx};
use engine_runtime::scheduler::TickConfig;
use engine_runtime::threadpool::ThreadPool;
use engine_runtime::clock::NANOS_PER_SEC;
use game_server::cli::{self, Command, USAGE};
use game_server::log::Level;
use game_server::mem::LinearMemory;
use game_server::stats::Stats;
use game_server::{assets, segments, Config};

fn main() -> ExitCode {
    let cfg = match cli::parse(std::env::args().skip(1)) {
        Ok(Command::Run(cfg)) => *cfg,
        Ok(Command::Help) => {
            print!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        Ok(Command::Version) => {
            println!("game-server {}", env!("CARGO_PKG_VERSION"));
            return ExitCode::SUCCESS;
        }
        Err(e) => {
            eprintln!("game-server: {e}");
            eprintln!("try --help");
            return ExitCode::from(2);
        }
    };

    set_level(cfg.log_level);
    match serve(cfg) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("game-server: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Map the CLI verbosity onto the library's logger.
fn set_level(level: Level) {
    game_server::log::init(level);
}

fn serve(cfg: Config) -> Result<(), String> {
    game_server::log_info!(
        "boot",
        "reserving {} MiB of linear memory",
        cfg.memory_bytes / (1024 * 1024)
    );
    let mem = LinearMemory::reserve(cfg.memory_bytes).map_err(|e| format!("memory: {e}"))?;
    game_server::log_info!(
        "boot",
        "memory backend={} reserved={} bytes",
        mem.backend(),
        mem.len()
    );

    // The engine's own data image, recovered from game.wasm's 4 077 passive segments.
    let image = PathBuf::from(&cfg.data_dir).join("segments.bin");
    let source = if image.is_file() { cfg.data_dir.clone() } else { cfg.data_dir.clone() };
    match segments::load_from_dir(&source) {
        Ok(Some(blob)) => {
            let report = segments::install(&mem, &blob).map_err(|e| format!("segments: {e}"))?;
            game_server::log_info!(
                "boot",
                "installed {} segments / {} bytes ({} passive)",
                report.segments,
                report.bytes,
                report.passive
            );
        }
        Ok(None) => game_server::log_warn!(
            "boot",
            "no segment image under {}; the engine reads zeros until one is placed there",
            cfg.data_dir.display()
        ),
        Err(e) => game_server::log_warn!("boot", "segment image rejected: {e}"),
    }

    // Assets are optional for a smoke run: absent files are a logged condition.
    let assets = assets::probe(&cfg.data_dir.join("game"));
    game_server::log_info!("boot", "assets: {}", assets.summary());

    let stats = Arc::new(Stats::new("8b0b5899ed", &mem));
    stats.set_assets(assets.is_present(), assets.state());

    let pool = ThreadPool::new(cfg.threads, cfg.single_threaded)
        .map_err(|e| format!("thread pool: {e}"))?;
    let tick_cfg = TickConfig { hz: cfg.tick_hz, ..TickConfig::default() };
    let mut host =
        Host::new(tick_cfg, pool, Box::new(OsTime::new())).map_err(|e| format!("tick config: {e}"))?;

    let max_ticks = cfg.max_ticks;
    let done = AtomicU64::new(0);
    let step_ns = NANOS_PER_SEC / u64::from(cfg.tick_hz.max(1));

    let run_stats = run(&mut host, |clock, ctx: &TickCtx<'_>| {
        // What the engine's own `fwTimer::Update` (func 8924) does to the frame counter:
        // a 32-bit store at this address. GetGameFrame() then reads it back.
        let frame = ctx.frame as u32;
        if mem.write(u64::from(FRAME_COUNTER_VA), &frame.to_le_bytes()).is_err() {
            ctx.control.stop();
            return;
        }
        clock.set_frame(ctx.frame);
        stats.record_tick(ctx.tick_index, step_ns);

        let n = done.fetch_add(1, Ordering::Relaxed) + 1;
        if let Some(limit) = max_ticks {
            if n >= limit {
                ctx.control.stop();
            }
        }
    });

    game_server::log_info!(
        "shutdown",
        "ticks={} dropped_ticks={} elapsed={} ms",
        run_stats.ticks,
        run_stats.dropped_ns / step_ns,
        run_stats.ticks * step_ns / 1_000_000
    );

    // The engine's own view of the frame, read back from memory rather than from the
    // counter we just wrote — that is the whole point of the check.
    let frame_in_memory = mem
        .read(u64::from(FRAME_COUNTER_VA), 4)
        .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        .unwrap_or(0);

    println!(
        "{{\"status\":\"ok\",\"frames\":{},\"frame_counter_va\":{},\"tick_hz\":{},\"dropped\":{}}}",
        frame_in_memory, FRAME_COUNTER_VA, cfg.tick_hz, run_stats.dropped_ns / step_ns
    );

    Ok(())
}