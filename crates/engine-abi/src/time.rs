//! Clock imports (`env.wasm_now_ms`, `env.emscripten_get_now`,
//! `env.emscripten_date_now`, `wasi_snapshot_preview1.clock_time_get`).

use crate::state;

/// `env.wasm_now_ms` — wasm type `(func (result f64))`.
///
/// The engine's primary clock: milliseconds on the **monotonic** timeline
/// (`performance.now()` in the browser build). Every 51 call sites in the
/// module use it for frame timing, wait budgets and profiling, so it must never
/// go backwards and must not read the wall clock (a server would otherwise be
/// thrown off by NTP steps).
///
/// Backed by [`engine_host::Host::now_micros`] / 1000. With no host installed it
/// answers `0.0`, i.e. "time has not advanced"; the engine's waits then reduce to
/// zero-length spins instead of panicking.
#[no_mangle]
pub extern "C" fn wasm_now_ms() -> f64 {
    match state::host() {
        Some(host) => host.now_micros() as f64 / 1000.0,
        None => 0.0,
    }
}

/// `env.emscripten_get_now` — wasm type `(func (result f64))`.
///
/// Emscripten's `emscripten_get_now()`: same monotonic millisecond timeline as
/// [`wasm_now_ms`], kept as a separate import because libc's `clock_gettime`
/// and `emscripten_get_now` were distinct JS functions. Both now read the one
/// host clock, which is what the engine expects (they were never allowed to
/// disagree in the browser build either).
#[no_mangle]
pub extern "C" fn emscripten_get_now() -> f64 {
    wasm_now_ms()
}

/// `env.emscripten_date_now` — wasm type `(func (result f64))`.
///
/// `Date.now()`: **wall-clock** milliseconds since the Unix epoch. Distinct from
/// [`wasm_now_ms`] on purpose — the engine uses it for timestamps written into
/// saves and logs, where the absolute time matters and a backwards step is
/// acceptable. Reads `std::time::SystemTime`; a pre-epoch clock answers `0.0`.
#[no_mangle]
pub extern "C" fn emscripten_date_now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as f64)
        .unwrap_or(0.0)
}

/// `wasi_snapshot_preview1.clock_time_get` — wasm type
/// `(func (param i32 i64 i64) (result i32))`.
///
/// `args[0]` = clock id (`0` = realtime, `1` = monotonic, …), `args[1]` =
/// precision hint (ignored), `args[2]` = out pointer for a `u64` nanosecond
/// timestamp. Returns a wasi errno (`0` = success, `28` = `EINVAL`).
///
/// Realtime comes from `SystemTime`, monotonic from [`engine_host::Host::now_nanos`],
/// so `clock_gettime(CLOCK_MONOTONIC)` and `wasm_now_ms` share one origin — the
/// engine mixes both in its timer paths.
#[no_mangle]
pub unsafe extern "C" fn __wasi_clock_time_get(clock_id: u32, _precision: u64, out: *mut u8) -> i32 {
    const WASI_ESUCCESS: i32 = 0;
    const WASI_EINVAL: i32 = 28;

    let nanos: u64 = match clock_id {
        // CLOCK_REALTIME
        0 => std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0),
        // CLOCK_MONOTONIC, CLOCK_PROCESS_CPUTIME_ID, CLOCK_THREAD_CPUTIME_ID —
        // the server has one monotonic timeline; CPU-time variants report it too
        // rather than failing, because libc's `clock()` only needs monotonicity.
        1..=3 => match state::host() {
            Some(host) => host.now_nanos(),
            None => 0,
        },
        _ => return -WASI_EINVAL,
    };
    if !crate::mem::write_u64(out, nanos) {
        return -WASI_EINVAL;
    }
    WASI_ESUCCESS
}

/// `wasi_snapshot_preview1.clock_time_get` under its import field name.
///
/// The `.wat` binds it to `$__wasi_clock_time_get`; the linker resolves whichever
/// name the translation emits, so both exist.
#[no_mangle]
pub unsafe extern "C" fn clock_time_get(clock_id: u32, precision: u64, out: *mut u8) -> i32 {
    // SAFETY: forwards the caller's out pointer; the callee range-checks it.
    unsafe { __wasi_clock_time_get(clock_id, precision, out) }
}