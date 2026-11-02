//! `env.wasm_audio_publish_js` — hand a finished PCM block to the device.

use engine_host::AudioBlock;

use crate::mem;
use crate::state;

/// Descriptor the engine fills before publishing.
///
/// Observed call site (one in the module):
///
/// ```text
/// i64.const 8153472            ; descriptor
/// i32.const 16384              ; capacity in bytes
/// i32.store offset=8           ; descriptor.byte_len = 16384
/// i64.const 8153472
/// i32.const 16384
/// call $wasm_audio_publish_js
/// ```
///
/// The engine therefore publishes a *descriptor*: `{ data_ptr: u64 @0,
/// byte_len: u32 @8, … }` plus the capacity of that descriptor in bytes. The
/// shim reads the descriptor; if it does not look like one (no plausible
/// in-memory `data_ptr` at offset 0, which is what a plain buffer looks like),
/// it falls back to treating `block` itself as the PCM payload of `capacity`
/// bytes. Both shapes are accepted so a translation that collapses the
/// descriptor into a direct buffer keeps working.
///
/// Channel layout is not carried by the import. The engine's mixer worklet was
/// always 48 kHz stereo, so `frames` is derived as `samples / 2`; a payload that
/// is not an even number of `f32`s is treated as mono.
#[no_mangle]
pub unsafe extern "C" fn wasm_audio_publish_js(block: *mut u8, capacity: u32) {
    let host = match state::host() {
        Some(host) => host,
        None => return,
    };

    let capacity = capacity as usize;
    let (data_off, byte_len) = match (
        mem::read_u64(block as *const u8),
        mem::read_u32((block as *const u8).wrapping_add(8)),
    ) {
        (Some(data_off), Some(byte_len))
            if mem::is_valid(data_off as usize as *const u8, byte_len as usize)
                && byte_len as usize <= capacity =>
        {
            (data_off, byte_len as usize)
        }
        _ => (block as usize as u64, capacity),
    };

    if byte_len == 0 || byte_len % 4 != 0 {
        return;
    }
    let raw = match mem::bytes(data_off as usize as *const u8, byte_len) {
        Some(raw) => raw,
        None => return,
    };
    let mut samples = Vec::with_capacity(raw.len() / 4);
    for chunk in raw.chunks_exact(4) {
        samples.push(f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
    }
    let channels: u16 = if samples.len() % 2 == 0 { 2 } else { 1 };

    // `AudioBlock::new` validates channels/rate/frames; a malformed block is the
    // engine's bug, and the documented behaviour is to drop it rather than to
    // unwind across the extern "C" boundary.
    if let Ok(block) = AudioBlock::new(0, 48_000, channels, samples) {
        let _ = host.audio().publish(block);
    }
}