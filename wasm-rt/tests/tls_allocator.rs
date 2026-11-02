use wasm_rt::{Memory, Phantom};
mod imp {}
mod syms {}
#[path = "../../linked/generated/gen000/src/f_002929_rage_sysmemstartdebug.rs"]
mod start;
#[path = "../../linked/generated/gen000/src/f_002930_rage_sysmemenddebug.rs"]
mod end;

#[test]
fn translated_allocator_scope_preserves_tls_and_restores_the_previous_allocator() {
    unsafe extern "C" fn get_allocator(ph: *mut Phantom, object: i64, kind: i32) -> i64 {
        unsafe { (*ph).g[63] = (object as u64) << 32 | kind as u32 as u64; }
        0x5000
    }
    let mut ph = Phantom::with_memory(Memory::new(1, 1).unwrap());
    let table = [core::ptr::null(), get_allocator as *const ()];
    let types = [u32::MAX, 8];
    ph.g[1] = 0x1000;
    ph.table = table.as_ptr();
    ph.tlen = table.len();
    ph.table_types = types.as_ptr();
    unsafe {
        ph.write_u64(0x1000, 0x1234);
        ph.write_u64(0x1000 + 832, 0x2000);
        ph.write_u64(0x1000 + 840, 0x3000);
        ph.write_u64(0x3000, 0x4000);
        ph.write_u64(0x4000 + 72, 1);
        start::f_002929_rage_sysmemstartdebug(&mut ph);
        assert_eq!(ph.read_u64(0x1000), 0x1234);
        assert_eq!(ph.read_u64(0x1000 + 880), 0x2000);
        assert_eq!(ph.read_u64(0x1000 + 832), 0x5000);
        assert_eq!(ph.g[63], (0x3000u64 << 32) | 4);
        end::f_002930_rage_sysmemenddebug(&mut ph);
        assert_eq!(ph.read_u64(0x1000 + 832), 0x2000);
        assert_eq!(ph.read_u64(0x1000 + 880), 0);
        assert_eq!(ph.read_u64(0x1000), 0x1234);
    }
}
