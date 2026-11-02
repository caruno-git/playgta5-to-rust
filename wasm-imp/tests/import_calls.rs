#![cfg(not(target_family = "wasm"))]

#[test]
fn imports_call_the_host_using_the_original_wasm_signatures() {
    let mut memory = vec![0u8; 512];
    unsafe {
        engine_abi::set_memory(memory.as_mut_ptr(), memory.len());
    }
    unsafe {
        assert_eq!(
            wasm_imp::imp_0_wasm_now_ms(),
            engine_abi::time::wasm_now_ms()
        );
        assert_eq!(
            wasm_imp::imp_12_emscripten_get_now(),
            engine_abi::time::emscripten_get_now()
        );
        assert_eq!(wasm_imp::imp_17_emscripten_get_heap_max(), 512);
        assert_eq!(wasm_imp::imp_34_wasi_clock_time_get(1, 0, 8), 0);
        assert_eq!(&memory[8..16], &0u64.to_le_bytes());
    }
    engine_abi::state::clear_memory();
}
