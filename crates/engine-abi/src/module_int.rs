//! `env.wasm_module_int_js` — the JS shell's integer tunables.

use crate::mem;
use crate::state;

/// `env.wasm_module_int_js` — wasm type `(func (param i64 i32) (result i32))`.
///
/// Call sites in the module (`0x2DA6CC` region, two of them):
///
/// ```text
/// i64.const <key>      ; pointer to a NUL-terminated key
/// i32.const 1024       ; fallback value used when the shell had no answer
/// call $wasm_module_int_js
/// ```
///
/// So the contract is `module_int(key: *const c_char, fallback: i32) -> i32`:
/// look the key up in the host's tunable table, and answer `fallback` when the
/// key is absent. The browser shell sourced these from the launcher
/// (`renderer`, `locale`, `platform`, buffer sizes); the server reads them from
/// [`engine_host::HeadlessConfig::module_ints`].
///
/// A key that does not resolve to memory, or that the host answers with `0`
/// (its "unset" value), yields `fallback` — the engine treats both the same way.
#[no_mangle]
pub unsafe extern "C" fn wasm_module_int_js(key: *const u8, fallback: u32) -> i32 {
    let fallback = fallback as i32;
    let host = match state::host() {
        Some(host) => host,
        None => return fallback,
    };
    let text = match mem::cstr(key) {
        Some(text) => text,
        None => return fallback,
    };
    let value = host.module_int(text);
    if value == 0 {
        fallback
    } else {
        value as i32
    }
}