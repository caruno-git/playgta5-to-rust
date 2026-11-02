//! `env.emscripten_*`, `env._*_js`, `env.invoke_*` — the emscripten runtime glue.
//!
//! Three groups live here because they share one property: they existed only to
//! bridge C++ to JavaScript, and most of them disappear when the runtime is the
//! process itself.
//!
//! * **Logging / abort** (`wasm_print_line_js`, `wasm_hang_line_js`, `_abort_js`,
//!   `_emscripten_throw_longjmp`) — real, routed to [`engine_host::Host::log`].
//! * **Stack & heap** (`emscripten_stack_snapshot`, `emscripten_get_heap_max`,
//!   `emscripten_resize_heap`) — the heap answers are real (they describe the
//!   mapping published with [`crate::state::set_memory`]); the stack snapshot has
//!   no native equivalent and is an explicit no-op.
//! * **Threads & mailboxes** (`__pthread_create_js`, `_emscripten_thread_*`,
//!   `_emscripten_*mailbox*`) — threads are real `std::thread`s through
//!   [`engine_host::Host::spawn`]; the proxying-queue machinery is a no-op because
//!   native threads call each other directly instead of posting to a JS mailbox.
//! * **Indirect calls** (`invoke_*`) — real: the 11 trampolines dispatch to a
//!   native function pointer (or a registered table slot).
//! * **Time zone** (`_tzset_js`, `_mktime_js`, `_localtime_js`, `_gmtime_js`) —
//!   real UTC arithmetic; see the note on each function about local time.

use std::collections::BTreeMap;
use std::sync::Mutex;

use engine_host::LogLevel;

use crate::mem;
use crate::state;

// ---------------------------------------------------------------------------
// Logging
// ---------------------------------------------------------------------------

/// `env.wasm_print_line_js` — wasm type `(func (param i64))`.
///
/// `(line)` where `line` is a NUL-terminated string in linear memory. Three call
/// sites in the module, all on the engine's diagnostic path (`printf`-style
/// output that the browser wrote to `console.log`). Routed to
/// [`engine_host::Host::log`] at `INFO` with the `engine` target, so a server's
/// stdout carries the same lines the browser console did.
#[no_mangle]
pub unsafe extern "C" fn wasm_print_line_js(line: *const u8) {
    let text = match mem::cstr(line) {
        Some(text) => text,
        None => return,
    };
    if let Some(host) = state::host() {
        host.log(LogLevel::Info, "engine", text);
    }
}

/// `env.wasm_hang_line_js` — wasm type `(func (param i64))`.
///
/// `(line)`: the engine's "I detected an inconsistency and am about to hang"
/// report — five call sites, all on assertion paths that in the browser logged to
/// the console and then blocked. On the server it is logged at `ERROR` and the
/// caller then blocks *inside the engine* (that behaviour is the engine's, not
/// the import's: the import only reported).
///
/// The call is counted so an operator can alert on "the engine hit a hang line".
#[no_mangle]
pub unsafe extern "C" fn wasm_hang_line_js(line: *const u8) {
    state::bump(state::Counter::HangLine);
    let text = mem::cstr(line).unwrap_or("<unreadable hang line>");
    if let Some(host) = state::host() {
        host.log(LogLevel::Error, "engine", text);
    }
}

// ---------------------------------------------------------------------------
// Stack snapshots
// ---------------------------------------------------------------------------

/// `env.emscripten_stack_snapshot` — wasm type `(func (result i64))`.
///
/// Explicit no-op answering `0` (the "no snapshot" handle). The browser build
/// captured the wasm shadow-stack pointer so `emscripten_stack_unwind_buffer`
/// could walk `__stack_high` frames afterwards; the native runtime has real
/// unwinding (and the translated code's frames are Rust frames, not a shadow
/// stack), so a captured wasm stack pointer would be meaningless. Error paths
/// that use the snapshot only need "no snapshot available", which `0` is.
#[no_mangle]
pub extern "C" fn emscripten_stack_snapshot() -> u64 {
    state::bump(state::Counter::StackSnapshot);
    0
}

/// `env.emscripten_stack_unwind_buffer` — wasm type
/// `(func (param i64 i64 i32) (result i32))`.
///
/// `(snapshot, buffer, depth)` → number of frames written. Always `0`: see
/// [`emscripten_stack_snapshot`]. Returning "zero frames" is the documented
/// contract for an unusable snapshot, so the engine's crash reporter prints an
/// empty backtrace instead of garbage.
#[no_mangle]
pub unsafe extern "C" fn emscripten_stack_unwind_buffer(
    _snapshot: u64,
    _buffer: *mut u8,
    _depth: u32,
) -> i32 {
    0
}

// ---------------------------------------------------------------------------
// Inline-asm constants
// ---------------------------------------------------------------------------

/// `env.emscripten_asm_const_int` — wasm type
/// `(func (param i64 i64 i64) (result i32))`.
///
/// `(code, signature, args)` — emscripten's `EM_ASM` bridge: `code` indexes a
/// table of JavaScript snippets baked into `game.js`, `args` points at the packed
/// arguments. Every snippet in this build was UI plumbing (canvas size, cursor
/// visibility, clipboard), so the native port has the equivalent logic in Rust and
/// there is no table to dispatch to.
///
/// Explicit no-op answering `0` (the JS snippets that returned a value all
/// returned a success code). The call is counted, and a warning is logged once per
/// distinct code so a server operator can spot a path that still expects JS.
#[no_mangle]
pub unsafe extern "C" fn emscripten_asm_const_int(code: u64, _signature: *const u8, _args: *mut u8) -> i32 {
    warn_once(code);
    state::bump(state::Counter::AsmConst);
    0
}

/// `env.emscripten_asm_const_int_sync_on_main_thread` — wasm type
/// `(func (param i64 i64 i64) (result i32))`.
///
/// Same bridge as [`emscripten_asm_const_int`], called from a worker thread with
/// an implicit proxy to the main thread. There is no main thread to proxy to on a
/// server, and the snippets are UI-only, so: explicit no-op answering `0`.
#[no_mangle]
pub unsafe extern "C" fn emscripten_asm_const_int_sync_on_main_thread(
    code: u64,
    _signature: *const u8,
    _args: *mut u8,
) -> i32 {
    warn_once(code);
    state::bump(state::Counter::AsmConstSync);
    0
}

/// Codes already warned about, so a per-frame `EM_ASM` cannot flood the log.
static WARNED: Mutex<Option<BTreeMap<u64, ()>>> = Mutex::new(None);

/// Log the first use of an inline-asm code.
fn warn_once(code: u64) {
    let mut guard = match WARNED.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let seen = guard.get_or_insert_with(BTreeMap::new);
    if seen.insert(code, ()).is_none() {
        if let Some(host) = state::host() {
            host.log(
                LogLevel::Warn,
                "emscripten",
                &format!("EM_ASM code {code} has no native implementation; ignored"),
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Heap
// ---------------------------------------------------------------------------

/// `env.emscripten_get_heap_max` — wasm type `(func (result i64))`.
///
/// Highest address the engine's linear memory can reach. Real: answers the size
/// of the mapping published with [`crate::state::set_memory`], which is the
/// server's equivalent of the wasm memory maximum the browser reported.
#[no_mangle]
pub extern "C" fn emscripten_get_heap_max() -> u64 {
    state::memory_len() as u64
}

/// `env.emscripten_resize_heap` — wasm type `(func (param i64) (result i32))`.
///
/// `(requested_size)` → `1` if memory now extends to at least that size, else `0`.
///
/// The server's linear memory is one reservation sized at startup (the engine's
/// own heap allocator carves it up), so the only honest answers are "already big
/// enough" (`1`) and "cannot grow" (`0`). A request beyond the reservation is
/// refused *without* aborting, which is what the engine's `sbrk` caller expects
/// when it probes the limit.
#[no_mangle]
pub extern "C" fn emscripten_resize_heap(requested_size: u64) -> i32 {
    let capacity = state::memory_len() as u64;
    if requested_size <= capacity {
        1
    } else {
        if let Some(host) = state::host() {
            host.log(
                LogLevel::Error,
                "emscripten",
                &format!(
                    "resize_heap({requested_size}) exceeds the {capacity}-byte reservation"
                ),
            );
        }
        0
    }
}

// ---------------------------------------------------------------------------
// Abort / longjmp
// ---------------------------------------------------------------------------

/// `env._abort_js` — wasm type `(func)`.
///
/// `abort()` from libc. Real: `std::process::abort()` — an uncatchable abnormal
/// termination, which is exactly what the wasm build did after logging
/// (`RuntimeError: unreachable`). Not routed through the exit hook: a hook is for
/// orderly exits, and masking an abort would hide the engine's invariant failure.
#[no_mangle]
pub extern "C" fn _abort_js() {
    if let Some(host) = state::host() {
        host.log(LogLevel::Error, "engine", "abort()");
    }
    std::process::abort();
}

/// `env._emscripten_throw_longjmp` — wasm type `(func)`.
///
/// `longjmp` out of a JS frame: emscripten re-entered the wasm module with an
/// exception. The native translation has no `setjmp` buffer to jump to — the
/// engine's `try`/`catch`-style paths were C++ destructors plus a flag — so the
/// only faithful answer is to fail loudly rather than continue with an
/// inconsistent stack. Logs and calls `std::process::abort()`, which is the
/// native spelling of the original `unreachable` trap.
#[no_mangle]
pub extern "C" fn _emscripten_throw_longjmp() {
    if let Some(host) = state::host() {
        host.log(
            LogLevel::Error,
            "engine",
            "_emscripten_throw_longjmp: no native longjmp target (wasm trap equivalent)",
        );
    }
    std::process::abort();
}

// ---------------------------------------------------------------------------
// Threads and mailboxes
// ---------------------------------------------------------------------------

/// `env.__pthread_create_js` — wasm type
/// `(func (param i64 i64 i64 i64) (result i32))`.
///
/// `(pthread *, attr *, start_routine, arg)` → `0` on success, `-EAGAIN`
/// (`11`) on failure, matching `pthread_create`.
///
/// Real: the start routine is a native `extern "C" fn(*mut u8) -> *mut u8` in the
/// translated module, and the thread is a real `std::thread` created through
/// [`engine_host::Host::spawn`]. `pthread`/`attr` are written the way libc expects
/// (`pthread` receives the host handle id, which is the only value it ever reads
/// back) — the engine's `pthread_join` counterpart is libc-side bookkeeping.
#[no_mangle]
pub unsafe extern "C" fn __pthread_create_js(
    pthread_out: *mut u8,
    _attr: *const u8,
    start_routine: u64,
    arg: u64,
) -> i32 {
    let host = match state::host() {
        Some(host) => host,
        None => return -11, // EAGAIN: nothing to spawn on
    };
    let target = match resolve_indirect(start_routine) {
        Some(target) => target,
        None => return -22, // EINVAL: not a callable pointer
    };
    // SAFETY: `target` came from the engine as the address of a start routine with
    // the standard `pthread` signature; both arguments are opaque engine words.
    let entry: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(target) };
    let handle = match host.spawn(
        "engine-pthread",
        Box::new(move || {
            entry(arg);
        }),
    ) {
        Ok(handle) => handle,
        Err(_) => return -11,
    };
    if !mem::write_u64(pthread_out, handle.id()) {
        return -22;
    }
    0
}

/// `env._emscripten_thread_set_strongref` — wasm type `(func (param i64))`.
///
/// Explicit no-op. The import told the JS worker pool "keep this worker alive"
/// (a `Worker` reference count). A `std::thread` is kept alive by its handle,
/// which [`engine_host::Host::spawn`] already owns.
#[no_mangle]
pub unsafe extern "C" fn _emscripten_thread_set_strongref(_thread: *mut u8) {}

/// `env._emscripten_init_main_thread_js` — wasm type `(func (param i64))`.
///
/// Explicit no-op. It copied the main thread's TLS block pointer into the JS
/// thread registry so a newly spawned worker could find it. Each native thread has
/// its own TLS (the Rust runtime's), initialised by the platform, so there is
/// nothing to publish.
#[no_mangle]
pub unsafe extern "C" fn _emscripten_init_main_thread_js(_tls_block: *mut u8) {}

/// `env._emscripten_thread_mailbox_await` — wasm type `(func (param i64))`.
///
/// Explicit no-op. It blocked a worker until its JS mailbox had a message, i.e. it
/// was the proxy-then-block primitive. Native threads hand work to each other
/// directly (a channel or a join), so there is no mailbox to await; blocking here
/// would deadlock a server.
#[no_mangle]
pub unsafe extern "C" fn _emscripten_thread_mailbox_await(_mailbox: *mut u8) {
    state::bump(state::Counter::MailboxAwait);
}

/// `env._emscripten_notify_mailbox_postmessage` — wasm type
/// `(func (param i64 i64))`.
///
/// `(target_thread, queue_ptr)` — "post this queue to that worker". Explicit no-op:
/// same reason as [`_emscripten_thread_mailbox_await`], the native runtime has no
/// message queue between threads. Counted so a test can see the engine tried.
#[no_mangle]
pub unsafe extern "C" fn _emscripten_notify_mailbox_postmessage(
    _target_thread: *mut u8,
    _queue: *mut u8,
) {
    state::bump(state::Counter::MailboxPost);
}

/// `env._emscripten_receive_on_main_thread_js` — wasm type
/// `(func (param i32 i64 i64 i32 i64 i64 i64) (result f64))`.
///
/// `(index, args, arg_types, arg_count, _, _, _)` → the proxied call's result.
/// Explicit no-op answering `0.0`: it re-entered the JS snippet table on the main
/// thread, and on a server the calling thread *is* the executor, so there is no
/// proxy to perform and no snippet table to index (see
/// [`emscripten_asm_const_int`]). Counted.
#[no_mangle]
pub unsafe extern "C" fn _emscripten_receive_on_main_thread_js(
    _index: u32,
    _args: *mut u8,
    _arg_types: *const u8,
    _arg_count: u32,
    _arg0: u64,
    _arg1: u64,
    _arg2: u64,
) -> f64 {
    state::bump(state::Counter::MainThreadCall);
    0.0
}

/// `env._emscripten_thread_cleanup` — wasm type `(func (param i64))`.
///
/// Explicit no-op. It told the JS pool "this worker is exiting, recycle it"; the
/// corresponding native action is the thread function returning, which the
/// `std::thread` handle already handles.
#[no_mangle]
pub unsafe extern "C" fn _emscripten_thread_cleanup(_thread: *mut u8) {}

/// `env.emscripten_check_blocking_allowed` — wasm type `(func)`.
///
/// Explicit no-op. In the browser it asserted that blocking was legal (i.e. the
/// call was not on the main thread without `ALLOW_BLOCKING_ON_MAIN_THREAD`).
/// Every server thread may block, so the check always passes; a real
/// implementation would have to abort, and aborting on the engine's blocking
/// paths would be wrong here.
#[no_mangle]
pub extern "C" fn emscripten_check_blocking_allowed() {}

/// `env.emscripten_exit_with_live_runtime` — wasm type `(func)`.
///
/// Explicit no-op. It ended `emscripten_set_main_loop` while keeping the runtime
/// alive so JS could still call in. The server has no event loop to hand control
/// back to: the caller's frame returning *is* the mechanism.
#[no_mangle]
pub extern "C" fn emscripten_exit_with_live_runtime() {}

/// `env.emscripten_num_logical_cores` — wasm type `(func (result i32))`.
///
/// Real: `std::thread::available_parallelism()`, which is what
/// `navigator.hardwareConcurrency` reported to the browser build. Falls back to
/// `1` when the platform refuses to say.
#[no_mangle]
pub extern "C" fn emscripten_num_logical_cores() -> i32 {
    std::thread::available_parallelism()
        .map(|n| i32::try_from(n.get()).unwrap_or(i32::MAX))
        .unwrap_or(1)
}

/// `env._emscripten_system` — wasm type `(func (param i64) (result i32))`.
///
/// `(command_line)` → the shell's exit status, or a negative errno.
///
/// Real: runs the command through `sh -c` (or `cmd /C` on Windows). The engine
/// uses it for launcher-adjacent helpers that were shell scripts in the browser
/// package; a server has a real shell, so the command runs rather than being
/// faked. A missing interpreter or a non-zero status is reported, never panicked.
#[no_mangle]
pub unsafe extern "C" fn _emscripten_system(command: *const u8) -> i32 {
    let command = match mem::cstr(command) {
        Some(command) => command,
        None => return -crate::fd::EFAULT,
    };
    let mut invocation = if cfg!(windows) {
        let mut cmd = std::process::Command::new("cmd");
        cmd.arg("/C").arg(command);
        cmd
    } else {
        let mut cmd = std::process::Command::new("sh");
        cmd.arg("-c").arg(command);
        cmd
    };
    match invocation.status() {
        Ok(status) => status.code().unwrap_or(0),
        Err(error) => -crate::fd::errno_of(&error),
    }
}

// ---------------------------------------------------------------------------
// Time zone and calendar
// ---------------------------------------------------------------------------

/// Size of emscripten's wasm64 `struct tm`: nine `int`s, then
/// `long tm_gmtoff`(8) and `char *tm_zone`(8).
const TM_LEN: usize = 56;

/// Write a broken-down time into linear memory at `ptr`.
///
/// Layout: `sec@0, min@4, hour@8, mday@12, mon@16, year@20, wday@24, yday@28,
/// isdst@32` (all `i32`), `gmtoff@40` (`i64`, seconds east of UTC), `zone@48`
/// (`i64`, linear-memory offset of a NUL-terminated zone name or `0`).
unsafe fn write_tm(ptr: *mut u8, parts: &TmParts) -> bool {
    if !mem::zero(ptr, TM_LEN) {
        return false;
    }
    let fields: [(usize, i32); 9] = [
        (0, parts.sec),
        (4, parts.min),
        (8, parts.hour),
        (12, parts.mday),
        (16, parts.mon),
        (20, parts.year),
        (24, parts.wday),
        (28, parts.yday),
        (32, parts.isdst),
    ];
    for (offset, value) in fields {
        if !mem::write_i32(ptr.wrapping_add(offset), value) {
            return false;
        }
    }
    mem::write_i64(ptr.wrapping_add(40), parts.gmtoff)
}

/// A broken-down time.
#[derive(Clone, Copy, Debug)]
struct TmParts {
    sec: i32,
    min: i32,
    hour: i32,
    mday: i32,
    mon: i32,
    year: i32,
    wday: i32,
    yday: i32,
    isdst: i32,
    gmtoff: i64,
}

/// Days since 1970-01-01 for a proleptic Gregorian date (Howard Hinnant's
/// `days_from_civil`).
fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400; // [0, 399]
    let mp = (i64::from(month) + 9) % 12; // [0, 11]
    let doy = (153 * mp + 2) / 5 + i64::from(day) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// `(year, month, day)` for a day count since 1970-01-01 (`civil_from_days`).
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    (year + i64::from(month <= 2), month as u32, day as u32)
}

/// Split a Unix timestamp into broken-down UTC.
fn utc_parts(seconds: i64) -> TmParts {
    let days = seconds.div_euclid(86_400);
    let rem = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    // 1970-01-01 was a Thursday (4).
    let wday = (days + 4).rem_euclid(7) as i32;
    let yday = (days - days_from_civil(year, 1, 1)) as i32;
    TmParts {
        sec: (rem % 60) as i32,
        min: ((rem / 60) % 60) as i32,
        hour: (rem / 3600) as i32,
        mday: day as i32,
        mon: month as i32 - 1,
        year: (year - 1900) as i32,
        wday,
        yday,
        isdst: 0,
        gmtoff: 0,
    }
}

/// `env._localtime_js` — wasm type `(func (param i64 i64) (result i32))`.
///
/// `(time_t *, struct tm *)` → `0` on success. Real conversion, but **UTC only**:
/// the server has no `TZ` database dependency and the engine's gameplay code
/// (`GET_CLOCK_*` natives, save timestamps) only ever compared epoch values. A
/// deployment that needs true local time sets `TZ` and links a tz-aware host;
/// `tm_gmtoff` is reported as `0` so callers can detect the simplification.
#[no_mangle]
pub unsafe extern "C" fn _localtime_js(time: *const u8, tm: *mut u8) -> i32 {
    let seconds = match mem::read_i64(time) {
        Some(seconds) => seconds,
        None => return -1,
    };
    let parts = utc_parts(seconds);
    // SAFETY: `tm` is the caller's `struct tm *`; `write_tm` range-checks it
    // against the installed linear memory before writing.
    if unsafe { write_tm(tm, &parts) } {
        0
    } else {
        -1
    }
}

/// `env._gmtime_js` — wasm type `(func (param i64 i64) (result i32))`.
///
/// `(time_t *, struct tm *)` → `0` on success. Real UTC conversion; identical to
/// [`_localtime_js`] because that one is UTC too, kept separate because libc's
/// `gmtime` and `localtime` were separate imports.
#[no_mangle]
pub unsafe extern "C" fn _gmtime_js(time: *const u8, tm: *mut u8) -> i32 {
    // SAFETY: forwards the caller's pointers; `_localtime_js` validates them.
    unsafe { _localtime_js(time, tm) }
}

/// `env._mktime_js` — wasm type `(func (param i64) (result i64))`.
///
/// `(struct tm *)` → Unix seconds, or `-1`. Inverse of [`_gmtime_js`] over UTC:
/// normalises out-of-range fields the way `mktime` does (`mon`/`mday` may
/// overflow, `isdst` is ignored), so the engine's "add one day" arithmetic keeps
/// working.
#[no_mangle]
pub unsafe extern "C" fn _mktime_js(tm: *const u8) -> i64 {
    let read = |offset: usize| mem::read_u32(tm.wrapping_add(offset)).map(|v| v as i32);
    let (sec, min, hour, mday, mon, year) = match (
        read(0),
        read(4),
        read(8),
        read(12),
        read(16),
        read(20),
    ) {
        (Some(sec), Some(min), Some(hour), Some(mday), Some(mon), Some(year)) => {
            (sec, min, hour, mday, mon, year)
        }
        _ => return -1,
    };
    let total_year = i64::from(year) + 1900;
    let total_month = i64::from(mon);
    // Normalise the month so `mon` may be negative or above 11.
    let normal_year = total_year + total_month.div_euclid(12);
    let normal_month = total_month.rem_euclid(12) as u32 + 1;
    let days = days_from_civil(normal_year, normal_month, 1) + i64::from(mday) - 1;
    i64::from(hour) * 3600 + i64::from(min) * 60 + i64::from(sec) + days * 86_400
}

/// `env._tzset_js` — wasm type `(func (param i64 i64 i64 i64))`.
///
/// `(tzname_ptr, daylight_ptr, timezone_ptr, tz_ptr)`: fills libc's `tzname[2]`,
/// `daylight`, `timezone` and the `TZ` string. The server reports a fixed UTC zone
/// (`"UTC"` for both names, `daylight = 0`, `timezone = 0`), matching the UTC-only
/// [`_localtime_js`]. The `TZ` buffer is left untouched: it is an input.
#[no_mangle]
pub unsafe extern "C" fn _tzset_js(
    tzname: *mut u8,
    daylight: *mut u8,
    timezone: *mut u8,
    _tz: *const u8,
) {
    // `tzname` is `char *tzname[2]`; the two pointers are written as offsets so
    // the engine's `strdup`-free access still works. `0` means "no name", which
    // libc treats as empty.
    let _ = mem::write_u64(tzname, 0);
    let _ = mem::write_u64(tzname.wrapping_add(8), 0);
    let _ = mem::write_u32(daylight, 0);
    let _ = mem::write_u32(timezone, 0);
}

// ---------------------------------------------------------------------------
// Indirect calls
// ---------------------------------------------------------------------------

/// Registry for indirect-call targets that arrive as wasm table indices rather
/// than native addresses.
static INDIRECT: Mutex<Option<BTreeMap<u64, usize>>> = Mutex::new(None);

/// Register `target` (a native code address) under wasm table slot `index`.
///
/// The translated code passes real function addresses in the `invoke_*` first
/// argument, which is what the native port needs. A translation that kept the wasm
/// table index instead can publish its table here and the trampolines will resolve
/// through it.
pub fn register_indirect(index: u64, target: usize) {
    let mut guard = match INDIRECT.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    guard.get_or_insert_with(BTreeMap::new).insert(index, target);
}

/// Anything below this looks like a wasm table index, not a native address.
const TABLE_INDEX_LIMIT: u64 = 0x1_0000;

/// Turn the trampoline's first argument into a callable address.
fn resolve_indirect(target: u64) -> Option<usize> {
    if target >= TABLE_INDEX_LIMIT {
        return Some(target as usize);
    }
    let guard = match INDIRECT.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    guard.as_ref().and_then(|map| map.get(&target).copied())
}

/// Report an unresolvable trampoline call.
fn trampoline_miss(name: &str, target: u64) {
    if let Some(host) = state::host() {
        host.log(
            LogLevel::Error,
            "invoke",
            &format!("{name}: cannot resolve indirect target {target:#x}"),
        );
    }
}

/// `env.invoke_vjj` — wasm type `(func (param i64 i64 i64))`.
///
/// The `invoke_*` family is emscripten's indirect-call trampoline: JavaScript
/// could only call a wasm function through a table index, so every
/// `call_indirect` that a host callback could reach was wrapped in
/// `invoke_<result><args>(index, args…)`. On the native runtime the caller holds a
/// real function pointer, so the trampoline transmutes it and calls it directly —
/// no table, no allocation, no JS.
///
/// `invoke_vjj(f, a, b)` calls `extern "C" fn(u64, u64)`.
#[no_mangle]
pub unsafe extern "C" fn invoke_vjj(target: u64, a: u64, b: u64) {
    match resolve_indirect(target) {
        // SAFETY: the engine passes the address of a function with this exact
        // signature (it is the trampoline's whole purpose).
        Some(address) => {
            let function: extern "C" fn(u64, u64) = unsafe { std::mem::transmute(address) };
            function(a, b);
        }
        None => trampoline_miss("invoke_vjj", target),
    }
}

/// `env.invoke_ij` — wasm type `(func (param i64 i64) (result i32))`.
///
/// `invoke_ij(f, a)` calls `extern "C" fn(u64) -> u32`.
#[no_mangle]
pub unsafe extern "C" fn invoke_ij(target: u64, a: u64) -> u32 {
    match resolve_indirect(target) {
        // SAFETY: as `invoke_vjj`.
        Some(address) => {
            let function: extern "C" fn(u64) -> u32 = unsafe { std::mem::transmute(address) };
            function(a)
        }
        None => {
            trampoline_miss("invoke_ij", target);
            0
        }
    }
}

/// `env.invoke_vj` — wasm type `(func (param i64 i64))`.
///
/// `invoke_vj(f, a)` calls `extern "C" fn(u64)`.
#[no_mangle]
pub unsafe extern "C" fn invoke_vj(target: u64, a: u64) {
    match resolve_indirect(target) {
        // SAFETY: as `invoke_vjj`.
        Some(address) => {
            let function: extern "C" fn(u64) = unsafe { std::mem::transmute(address) };
            function(a);
        }
        None => trampoline_miss("invoke_vj", target),
    }
}

/// `env.invoke_vjij` — wasm type `(func (param i64 i64 i32 i64))`.
///
/// `invoke_vjij(f, a, b, c)` calls `extern "C" fn(u64, u32, u64)`.
#[no_mangle]
pub unsafe extern "C" fn invoke_vjij(target: u64, a: u64, b: u32, c: u64) {
    match resolve_indirect(target) {
        // SAFETY: as `invoke_vjj`.
        Some(address) => {
            let function: extern "C" fn(u64, u32, u64) = unsafe { std::mem::transmute(address) };
            function(a, b, c);
        }
        None => trampoline_miss("invoke_vjij", target),
    }
}

/// `env.invoke_jjij` — wasm type `(func (param i64 i64 i32 i64) (result i64))`.
///
/// `invoke_jjij(f, a, b, c)` calls `extern "C" fn(u64, u32, u64) -> u64`.
#[no_mangle]
pub unsafe extern "C" fn invoke_jjij(target: u64, a: u64, b: u32, c: u64) -> u64 {
    match resolve_indirect(target) {
        // SAFETY: as `invoke_vjj`.
        Some(address) => {
            let function: extern "C" fn(u64, u32, u64) -> u64 =
                unsafe { std::mem::transmute(address) };
            function(a, b, c)
        }
        None => {
            trampoline_miss("invoke_jjij", target);
            0
        }
    }
}

/// `env.invoke_vjii` — wasm type `(func (param i64 i64 i32 i32))`.
///
/// `invoke_vjii(f, a, b, c)` calls `extern "C" fn(u64, u32, u32)`.
#[no_mangle]
pub unsafe extern "C" fn invoke_vjii(target: u64, a: u64, b: u32, c: u32) {
    match resolve_indirect(target) {
        // SAFETY: as `invoke_vjj`.
        Some(address) => {
            let function: extern "C" fn(u64, u32, u32) = unsafe { std::mem::transmute(address) };
            function(a, b, c);
        }
        None => trampoline_miss("invoke_vjii", target),
    }
}

/// `env.invoke_iji` — wasm type `(func (param i64 i64 i32) (result i32))`.
///
/// `invoke_iji(f, a, b)` calls `extern "C" fn(u64, u32) -> u32`.
#[no_mangle]
pub unsafe extern "C" fn invoke_iji(target: u64, a: u64, b: u32) -> u32 {
    match resolve_indirect(target) {
        // SAFETY: as `invoke_vjj`.
        Some(address) => {
            let function: extern "C" fn(u64, u32) -> u32 = unsafe { std::mem::transmute(address) };
            function(a, b)
        }
        None => {
            trampoline_miss("invoke_iji", target);
            0
        }
    }
}

/// `env.invoke_vjjii` — wasm type `(func (param i64 i64 i64 i32 i32))`.
///
/// `invoke_vjjii(f, a, b, c, d)` calls `extern "C" fn(u64, u64, u32, u32)`.
#[no_mangle]
pub unsafe extern "C" fn invoke_vjjii(target: u64, a: u64, b: u64, c: u32, d: u32) {
    match resolve_indirect(target) {
        // SAFETY: as `invoke_vjj`.
        Some(address) => {
            let function: extern "C" fn(u64, u64, u32, u32) =
                unsafe { std::mem::transmute(address) };
            function(a, b, c, d);
        }
        None => trampoline_miss("invoke_vjjii", target),
    }
}

/// `env.invoke_jjiiijij` — wasm type
/// `(func (param i64 i64 i32 i32 i32 i64 i32 i64) (result i64))`.
///
/// `invoke_jjiiijij(f, a, b, c, d, e, g, h)` calls
/// `extern "C" fn(u64, u32, u32, u32, u64, u32, u64) -> u64`.
#[no_mangle]
pub unsafe extern "C" fn invoke_jjiiijij(
    target: u64,
    a: u64,
    b: u32,
    c: u32,
    d: u32,
    e: u64,
    g: u32,
    h: u64,
) -> u64 {
    match resolve_indirect(target) {
        // SAFETY: as `invoke_vjj`.
        Some(address) => {
            let function: extern "C" fn(u64, u32, u32, u32, u64, u32, u64) -> u64 =
                unsafe { std::mem::transmute(address) };
            function(a, b, c, d, e, g, h)
        }
        None => {
            trampoline_miss("invoke_jjiiijij", target);
            0
        }
    }
}

/// `env.invoke_ijji` — wasm type `(func (param i64 i64 i64 i32) (result i32))`.
///
/// `invoke_ijji(f, a, b, c)` calls `extern "C" fn(u64, u64, u32) -> u32`.
#[no_mangle]
pub unsafe extern "C" fn invoke_ijji(target: u64, a: u64, b: u64, c: u32) -> u32 {
    match resolve_indirect(target) {
        // SAFETY: as `invoke_vjj`.
        Some(address) => {
            let function: extern "C" fn(u64, u64, u32) -> u32 =
                unsafe { std::mem::transmute(address) };
            function(a, b, c)
        }
        None => {
            trampoline_miss("invoke_ijji", target);
            0
        }
    }
}

/// `env.invoke_ijjiii` — wasm type
/// `(func (param i64 i64 i64 i32 i32 i32) (result i32))`.
///
/// `invoke_ijjiii(f, a, b, c, d, e)` calls
/// `extern "C" fn(u64, u64, u32, u32, u32) -> u32`.
#[no_mangle]
pub unsafe extern "C" fn invoke_ijjiii(
    target: u64,
    a: u64,
    b: u64,
    c: u32,
    d: u32,
    e: u32,
) -> u32 {
    match resolve_indirect(target) {
        // SAFETY: as `invoke_vjj`.
        Some(address) => {
            let function: extern "C" fn(u64, u64, u32, u32, u32) -> u32 =
                unsafe { std::mem::transmute(address) };
            function(a, b, c, d, e)
        }
        None => {
            trampoline_miss("invoke_ijjiii", target);
            0
        }
    }
}