//! Userdata imports — the save/settings blob store.
//!
//! In the browser build these three imports were IndexedDB transactions driven
//! from `game.js`. The server keeps the same triples
//! (`put`/`delete`/`page`) over [`engine_host::UserdataStore`], which the
//! headless host backs with one file per key (temp file + rename, so a crash
//! mid-save cannot tear a blob).

use crate::mem;
use crate::state;

/// Rows reported by [`wasm_userdata_page_js`] per call. The import carries no
/// cursor, so the server reports the whole store rather than a window.
const PAGE_WINDOW: usize = 4096;

/// `env.wasm_userdata_put_js` — wasm type
/// `(func (param i64 i64 i64 f64))`.
///
/// Call site (one in the module, in the save writer):
///
/// ```text
/// <key data pointer>      ; arg 1: NUL-terminated key
/// <buffer pointer>        ; arg 2: payload
/// <end - start>           ; arg 3: payload length
/// <seconds as f64>        ; arg 4: engine-side timestamp
/// call $wasm_userdata_put_js
/// ```
///
/// The key is a C string because the import carries no key length. The payload
/// is copied into the store before returning: the engine acknowledges the save
/// to the UI on the next instruction, so a deferred write would lose it.
///
/// The `f64` timestamp is advisory (the browser stored it as the IndexedDB
/// record's `ts`); [`engine_host::UserdataStore::put`] stamps the file itself, so
/// it is accepted and dropped. A key or payload pointer outside the memory
/// mapping, an empty key, or a store error is dropped silently — the engine's
/// own save-state machine retries and reports the failure.
#[no_mangle]
pub unsafe extern "C" fn wasm_userdata_put_js(key: *const u8, data: *const u8, len: u64, _ts: f64) {
    let host = match state::host() {
        Some(host) => host,
        None => return,
    };
    let key = match mem::cstr(key) {
        Some(key) => key,
        None => return,
    };
    let len = match usize::try_from(len) {
        Ok(len) => len,
        Err(_) => return,
    };
    let payload = match mem::bytes(data, len) {
        Some(payload) => payload,
        None => return,
    };
    let _ = host.userdata().put(key, payload);
}

/// `env.wasm_userdata_delete_js` — wasm type `(func (param i64))`.
///
/// `key` is the same NUL-terminated key [`wasm_userdata_put_js`] wrote. Deleting
/// a key that is not present is not an error (the browser build's
/// `objectStore.delete()` behaved the same way).
#[no_mangle]
pub unsafe extern "C" fn wasm_userdata_delete_js(key: *const u8) {
    let host = match state::host() {
        Some(host) => host,
        None => return,
    };
    if let Some(key) = mem::cstr(key) {
        let _ = host.userdata().delete(key);
    }
}

/// `env.wasm_userdata_page_js` — wasm type `(func (result i32))`.
///
/// The engine probes "how many userdata rows are there?" once at startup and
/// caches the answer in a global, treating a negative result as "storage
/// unavailable, use in-memory defaults". The import therefore takes no cursor
/// and answers a count: rows currently in the store, or `-1` when the store
/// cannot be read.
#[no_mangle]
pub extern "C" fn wasm_userdata_page_js() -> i32 {
    let host = match state::host() {
        Some(host) => host,
        None => return -1,
    };
    match host.userdata().page(None, PAGE_WINDOW) {
        Ok(page) => i32::try_from(page.entries.len()).unwrap_or(i32::MAX),
        Err(_) => -1,
    }
}

/// `env.wasm_has_page_js` — wasm type `(func (result i32))`.
///
/// "Is persistent storage available?" — the engine gates its save UI (and the
/// auto-save path) on a non-zero answer. The server answers `1` whenever the
/// userdata store accepts a listing call, `0` when no host is installed or the
/// listing fails.
#[no_mangle]
pub extern "C" fn wasm_has_page_js() -> i32 {
    let host = match state::host() {
        Some(host) => host,
        None => return 0,
    };
    match host.userdata().page(None, 1) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}