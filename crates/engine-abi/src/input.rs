//! `env.wasm_input_publish_js` — the engine's input snapshot.

use crate::state;

/// `env.wasm_input_publish_js` — wasm type `(func (param i64))`.
///
/// Call site (one in the module, from the per-frame input update):
///
/// ```text
/// i64.const 6583436          ; pointer to the engine's input state block
/// call $wasm_input_publish_js
/// ```
///
/// *Direction matters.* This import runs engine → browser: the engine pushed the
/// keyboard/pad state it observed into the input worklet and the debug overlay,
/// which read a fixed layout out of linear memory. `env.wasm_input_poll`
/// (host → engine) is the opposite direction and lives on
/// [`engine_host::Host::input`].
///
/// On the server there is no observer: input arrives from netcode or a replay
/// through [`engine_host::InputSender`] and is drained by the engine itself. The
/// pointer is a stable global owned by the engine, so publishing it has no side
/// effect worth reproducing, and copying the block would only allocate. The call
/// is counted (see [`state::count`]) so a test can prove the input path ran.
#[no_mangle]
pub unsafe extern "C" fn wasm_input_publish_js(_state_block: *mut u8) {
    state::bump(state::Counter::InputPublish);
}