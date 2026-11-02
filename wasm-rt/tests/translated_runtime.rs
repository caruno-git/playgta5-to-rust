use std::panic::{catch_unwind, AssertUnwindSafe};
use wasm_rt::*;

fn context() -> Phantom {
    Phantom::with_memory(Memory::new(1, 2).unwrap())
}

#[test]
fn loads_stores_and_bulk_ops_trap_before_writing() {
    let mut ph = context();
    unsafe {
        ph.write_u32(3, 0x78563412);
        assert_eq!(ph.read_u8(3), 0x12);
        assert_eq!(ph.read_u32(3), 0x78563412);
        assert!(catch_unwind(AssertUnwindSafe(|| ph.write_u32(65534, 9))).is_err());
        assert!(catch_unwind(AssertUnwindSafe(|| ph.memory_copy(0, 65534, 4))).is_err());
        assert!(catch_unwind(AssertUnwindSafe(|| ph.read_u8(-1))).is_err());
        assert_eq!(ph.read_u32(3), 0x78563412);
        ph.memory_copy(4, 3, 4);
        assert_eq!(ph.read_u32(4), 0x78563412);
        ph.memory_fill(16, 0x1ab, 8);
        assert_eq!(ph.read_u64(16), 0xabababababababab);
    }
}

#[test]
fn dropped_segments_and_overflowing_ranges_trap() {
    let mut ph = context();
    unsafe {
        ph.memory_init(1, 0, 0, 16);
        let before = ph.read_v128(0);
        ph.data_drop(1);
        ph.memory_init(1, 65536, 0, 0);
        assert!(catch_unwind(AssertUnwindSafe(|| ph.memory_init(1, 0, 0, 1))).is_err());
        assert!(catch_unwind(AssertUnwindSafe(|| ph.memory_init(0, 0, -1, 1))).is_err());
        assert!(catch_unwind(AssertUnwindSafe(|| ph.memory_init(0, -1, 0, 1))).is_err());
        assert_eq!(ph.read_v128(0), before);
    }
}

#[test]
fn owned_memory_grows_and_refreshes_the_base_pointer() {
    let mut ph = context();
    unsafe {
        ph.write_u32(0, 42);
    }
    assert_eq!(ph.memory_grow(0), 1);
    assert_eq!(ph.memory_grow(1), 1);
    assert_eq!(ph.memory_size(), 2);
    unsafe {
        assert_eq!(ph.read_u32(0), 42);
        assert_eq!(ph.read_u64(65536), 0);
    }
    assert_eq!(ph.memory_grow(1), -1);
    assert_eq!(ph.memory_grow(-1), -1);
}

#[test]
fn external_memory_is_not_reallocated_by_the_runtime() {
    let mut bytes = vec![0; PAGE_SIZE as usize];
    let mut ph = unsafe { Phantom::new(bytes.as_mut_ptr(), PAGE_SIZE) };
    assert_eq!(ph.memory_grow(0), 1);
    assert_eq!(ph.memory_grow(1), -1);
    assert_eq!(ph.base(), bytes.as_mut_ptr());
}

#[test]
fn atomic_ops_preserve_lane_width_and_check_alignment() {
    let ph = context();
    unsafe {
        ph.a_store_u8(0, 255);
        assert_eq!(ph.a_rmw_add_u8(0, 1i64), 255i64);
        assert_eq!(ph.a_load_u8(0), 0);
        ph.a_store_u32(4, -1i32);
        assert_eq!(ph.a_rmw_cmpxchg_u32(4, -1, 3), -1);
        assert_eq!(ph.a_rmw_cmpxchg_u32(4, -1, 7), 3);
        assert_eq!(ph.a_load_u32(4), 3);
        assert!(catch_unwind(AssertUnwindSafe(|| ph.a_load_u32(1))).is_err());
        assert!(catch_unwind(AssertUnwindSafe(|| ph.a_load_u64(65536))).is_err());
    }
}

#[test]
fn indirect_table_checks_64_bit_indices_without_truncation() {
    unsafe extern "C" fn target(_: &mut Phantom) {}
    let entries = [target as *const (), core::ptr::null()];
    let mut ph = context();
    ph.table = entries.as_ptr();
    ph.tlen = entries.len();
    unsafe {
        assert_eq!(ph.table_entry(0), entries[0]);
        for i in [1, 2, -1, 1i64 << 32] {
            assert!(catch_unwind(AssertUnwindSafe(|| ph.table_entry(i))).is_err());
        }
    }
}

#[test]
fn simd_loads_extend_sign_and_touch_only_the_selected_lane() {
    let ph = context();
    unsafe {
        ph.write_u64(0, 0x807f00ff807f00ff);
        assert_eq!(ph.read_8x8_s(0) & 0xffff, 0xffff);
        assert_eq!(ph.read_8x8_u(0) & 0xffff, 0xff);
        assert_eq!(ph.read_splat(0, 8), u128::MAX);
        let v = ph.load_lane(0, 0, 2, 32);
        assert_eq!(v, 0x807f00ffu128 << 64);
        ph.store_lane(16, v, 2, 32);
        assert_eq!(ph.read_u32(16), 0x807f00ff);
    }
}

#[test]
fn numeric_conversion_boundaries_follow_wasm_truncation() {
    assert_eq!(i32_trunc_f64_u(-0.9), 0);
    assert_eq!(i32_trunc_f64_s(-2147483648.75), i32::MIN);
    assert_eq!(i32_trunc_f64_u(4294967295.75), -1);
    assert_eq!(i64_trunc_f64_s(-9223372036854775808.0), i64::MIN);
    for value in [f64::NAN, f64::INFINITY, -1.0, 4294967296.0] {
        assert!(catch_unwind(|| i32_trunc_f64_u(value)).is_err());
    }
    assert!(catch_unwind(|| i64_trunc_f64_s(9223372036854775808.0)).is_err());
    assert!(catch_unwind(|| i64_trunc_f64_u(18446744073709551616.0)).is_err());
    assert_eq!(i32_trunc_sat_f64_u(f64::NAN), 0);
    assert_eq!(i64_trunc_sat_f64_u(f64::INFINITY), -1);
}

#[test]
fn division_and_remainder_have_distinct_overflow_rules() {
    assert!(catch_unwind(|| i32_div_s(i32::MIN, -1)).is_err());
    assert!(catch_unwind(|| i64_div_s(1, 0)).is_err());
    assert_eq!(i32_rem_s(i32::MIN, -1), 0);
    assert_eq!(i64_rem_s(i64::MIN, -1), 0);
    assert_eq!(i32_div_u(-1, 2), i32::MAX);
    assert_eq!(i64_rem_u(-1, 2), 1);
    assert_eq!(select_i32(0, 5, 9), 5);
    assert_eq!(select_i32(-1, 5, 9), 9);
}
