use std::panic::{catch_unwind, AssertUnwindSafe};
use wasm_rt::{Memory, Phantom};

#[no_mangle]
unsafe extern "C" fn f_091110_emscripten_thread_init(ph: &mut Phantom, a: i64, b: i32, c: i32, d: i32, e: i32, f: i32) {
    ph.g[1] = a as u64;
    ph.g[2] = (b + c + d + e + f) as u64;
}

#[no_mangle]
unsafe extern "C" fn f_090693_emscripten_tls_init(ph: &mut Phantom) -> i64 { ph.g[3] = 99; 99 }

#[test]
fn invoke_forwards_the_context_and_checks_the_target_type() {
    std::hint::black_box(engine_abi::time::wasm_now_ms());
    unsafe extern "C" fn target(ph: *mut Phantom, a: i64, b: i64) { (*ph).g[63] = a.wrapping_add(b) as u64; }
    let mut ph = Phantom::with_memory(Memory::new(1, 1).unwrap());
    let table = [target as *const ()];
    let mut types = [3u32];
    ph.table = table.as_ptr();
    ph.table_types = types.as_ptr();
    ph.tlen = 1;
    unsafe { wasm_imp::imp_19_invoke_vjj(&mut ph, 0, 7, 9); }
    assert_eq!(ph.g[63], 16);
    types[0] = 4;
    std::hint::black_box(&types);
    assert!(catch_unwind(AssertUnwindSafe(|| unsafe { wasm_imp::imp_19_invoke_vjj(&mut ph, 0, 7, 9) })).is_err());
    unsafe { wasm_imp::imp_52_emscripten_init_main_thread_js(&mut ph, 128); }
    assert_eq!(ph.g[1], 128);
    assert_eq!(ph.g[2], 1_048_579);
    assert_eq!(ph.g[3], 99);
}
