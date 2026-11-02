//! `env.wgpu_start_worker` — the GPU worker handshake.

use crate::state;

/// `env.wgpu_start_worker` — wasm type `(func (param i64 i64 i64))`.
///
/// Call site (one in the module, from renderer init):
///
/// ```text
/// i64.const 8348872 ; gpu config
/// <local 0>         ; shared memory / ring base
/// i64.store         ; 8348872 <- ring base
/// i32.const 64      ; ring record count
/// i64.const 8348856 ; 8348856 <- ring base (atomic)
/// <local 0>, i64.const 33554432, i64.const 6583436
/// call $wgpu_start_worker   ; (ring_ptr, 32 MiB, input_state_ptr)
/// ```
///
/// So: `wgpu_start_worker(ring_base, ring_bytes, input_state)` spun up
/// `wgpu_worker.js` over a 32 MiB shared command ring and told it where the
/// input snapshot lives.
///
/// Deliberate no-op on the server. The headless host has no device and no
/// second thread reading a command ring; frame submission happens through
/// [`engine_host::Host::gpu`], whose null sink drains the ring in-process so the
/// producers never block. Starting a worker would hand 32 MiB to a thread that
/// could only discard it. The call is counted so a test can assert renderer init
/// reached this point.
#[no_mangle]
pub unsafe extern "C" fn wgpu_start_worker(
    _ring_base: *mut u8,
    _ring_bytes: u64,
    _input_state: *mut u8,
) {
    state::bump(state::Counter::WgpuStartWorker);
}