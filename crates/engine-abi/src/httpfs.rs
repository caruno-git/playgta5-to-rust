//! `env.wasm_httpfs_io_start_js` and `env.wasm_httpfs_manifest_js` — the asset
//! manifest handshake and the io-worker wake-up.

use crate::mem;
use crate::state;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use wasm_rt::Phantom;

struct IoWorker {
    stop: Arc<AtomicBool>,
    thread: JoinHandle<()>,
}

static IO_WORKER: Mutex<Option<IoWorker>> = Mutex::new(None);

pub struct IoGuard;

impl Drop for IoGuard {
    fn drop(&mut self) { stop_io_worker(); }
}

pub fn stop_io_worker() {
    let worker = IO_WORKER.lock().unwrap_or_else(|p| p.into_inner()).take();
    if let Some(worker) = worker {
        worker.stop.store(true, Ordering::Release);
        let _ = worker.thread.join();
    }
}

unsafe fn serve(ph: &Phantom, host: &dyn engine_host::Host, slot: i64) {
    let id = unsafe { ph.read_u32(slot + 4) };
    let offset = unsafe { ph.read_u64(slot + 8) };
    let dst = unsafe { ph.read_u64(slot + 16) };
    let len = unsafe { ph.read_u32(slot + 24) };
    let valid = len <= i32::MAX as u32 && dst.checked_add(len as u64).is_some_and(|end| end <= ph.msize);
    let result = if valid {
        let destination = unsafe { std::slice::from_raw_parts_mut(ph.base().add(dst as usize), len as usize) };
        host.assets().pread(id, offset, destination).map(|n| n as i32).unwrap_or(-1)
    } else { -1 };
    unsafe {
        ph.a_store_u32(slot + 28, result);
        ph.a_store_u32(slot, 2);
    }
    ph.a_notify(slot, 1);
}

unsafe fn drain_hints(ph: &Phantom, table: i64) {
    let ring = unsafe { ph.a_load_u32(table + 8) } as u32 as u64;
    let capacity = unsafe { ph.a_load_u32(table + 12) } as u32;
    if ring == 0 || capacity == 0 { return; }
    let start = table as u64 + ring * 4;
    if !start.checked_add(capacity as u64 * 16).is_some_and(|end| end <= ph.msize) { return; }
    let head = unsafe { ph.a_load_u32(table + 16) } as u32;
    let mut tail = unsafe { ph.a_load_u32(table + 20) } as u32;
    if head.wrapping_sub(tail) > capacity { tail = head.wrapping_sub(capacity); }
    for _ in 0..head.wrapping_sub(tail).min(capacity) {
        let entry = start + (tail % capacity) as u64 * 16;
        if unsafe { ph.a_load_u32(entry as i64 + 12) } == 0 { break; }
        unsafe { ph.a_store_u32(entry as i64 + 12, 0); }
        tail = tail.wrapping_add(1);
    }
    unsafe { ph.a_store_u32(table + 20, tail as i32); }
}

/// `env.wasm_httpfs_manifest_js` — wasm type
/// `(func (param i64 i32) (result i32))`.
///
/// Query-then-fill, exactly as the module drives it:
///
/// ```text
/// i64.const 0            ; ptr = 0
/// i32.const 0            ; cap = 0
/// call $wasm_httpfs_manifest_js     ; -> required byte length, or negative
/// …
/// i64.extend_i32_u                  ; needed + 1
/// call $emscripten_builtin_malloc   ; buffer for the text
/// <buf>, needed + 1
/// call $wasm_httpfs_manifest_js     ; -> bytes written
/// ```
///
/// * `ptr == 0` → returns the number of bytes the manifest needs, **excluding**
///   the terminating NUL.
/// * `ptr != 0` → writes the manifest text (NUL terminated) when `cap` is large
///   enough, and returns the number of bytes written. `cap` too small or an
///   invalid pointer answers `-1`; the engine treats any negative result as
///   "no manifest" and falls back to on-demand path lookups.
///
/// The text is the tab-separated manifest the io worker used
/// (`id<TAB>size<TAB>path`, `#` comments) — see `engine_host::AssetManifest`.
#[no_mangle]
pub unsafe extern "C" fn wasm_httpfs_manifest_js(out: *mut u8, cap: u32) -> i32 {
    let host = match state::host() {
        Some(host) => host,
        None => return -1,
    };
    let text = match host.assets().manifest().to_engine_json() {
        Ok(text) => text,
        Err(_) => return -1,
    };
    let needed = text.len();
    if needed > i32::MAX as usize {
        return -1;
    }
    if out.is_null() {
        return needed as i32;
    }
    let cap = cap as usize;
    if cap <= needed { return needed as i32; }
    if !mem::write_bytes(out, text.as_bytes()) {
        return -1;
    }
    // NUL terminator: the engine hands the buffer to its own string parser.
    if !mem::write_bytes(out.wrapping_add(needed), &[0]) {
        return -1;
    }
    needed as i32
}

/// `env.wasm_httpfs_io_start_js` — wasm type `(func (param i64))`.
///
#[no_mangle]
pub unsafe extern "C" fn wasm_httpfs_io_start_js(request: *mut u8) {
    state::bump(state::Counter::IoStart);
    let mut worker = IO_WORKER.lock().unwrap_or_else(|p| p.into_inner());
    if worker.is_some() || request as usize % 4 != 0 || !mem::is_valid(request, 64) { return; }
    let Some(host) = state::host_arc() else { return; };
    let base = state::memory_base();
    let size = state::memory_len();
    let table = request as usize as i64;
    let stop = Arc::new(AtomicBool::new(false));
    let stopping = Arc::clone(&stop);
    let spawned = std::thread::Builder::new().name("engine-io".into()).spawn(move || {
        let ph = unsafe { Phantom::new(base as *mut u8, size as u64) };
        while !stopping.load(Ordering::Acquire) {
            let seen = unsafe { ph.a_load_u32(table) };
            let count = unsafe { ph.a_load_u32(table + 4) } as u32 as usize;
            if count > 4096 || !(table as u64).checked_add(64 + count as u64 * 64).is_some_and(|end| end <= ph.msize) { break; }
            for i in 0..count {
                let slot = table + 64 + i as i64 * 64;
                if unsafe { ph.a_rmw_cmpxchg_u32(slot, 1, 3) } == 1 {
                    unsafe { serve(&ph, host.as_ref(), slot); }
                }
            }
            unsafe {
                drain_hints(&ph, table);
                ph.a_wait32(table, seen, 20_000_000);
            }
        }
    });
    if let Ok(thread) = spawned { *worker = Some(IoWorker { stop, thread }); }
}
