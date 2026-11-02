#![allow(improper_ctypes)]
use core::arch::wasm64::v128;

#[link(wasm_import_module = "engine_intrinsics")]
extern "C" {
    pub fn read_u8(addr: i64) -> u8;
    pub fn read_i8(addr: i64) -> i8;
    pub fn read_u16(addr: i64) -> u16;
    pub fn read_i16(addr: i64) -> i16;
    pub fn read_u32(addr: i64) -> u32;
    pub fn read_i32(addr: i64) -> i32;
    pub fn read_u64(addr: i64) -> u64;
    pub fn read_f32(addr: i64) -> f32;
    pub fn read_f64(addr: i64) -> f64;
    pub fn read_v128(addr: i64) -> v128;
    pub fn write_u8(addr: i64, value: u8);
    pub fn write_i8(addr: i64, value: i8);
    pub fn write_u16(addr: i64, value: u16);
    pub fn write_i16(addr: i64, value: i16);
    pub fn write_u32(addr: i64, value: u32);
    pub fn write_i32(addr: i64, value: i32);
    pub fn write_u64(addr: i64, value: u64);
    pub fn write_f32(addr: i64, value: f32);
    pub fn write_f64(addr: i64, value: f64);
    pub fn write_v128(addr: i64, value: v128);
    pub fn memory_size() -> i64;
    pub fn memory_grow(delta: i64) -> i64;
    pub fn memory_copy(dst: i64, src: i64, len: i64);
    pub fn memory_fill(dst: i64, value: i32, len: i64);
    pub fn memory_init_copy(dst: i64, src: i64, len: i64);
    pub fn a_wait32(addr: i64, expected: i32, timeout: i64) -> i32;
    pub fn a_notify(addr: i64, count: i32) -> i32;
}

macro_rules! atomics {
    ($abi:ty, $load:ident, $store:ident, $add:ident, $sub:ident, $and:ident, $or:ident, $xor:ident, $xchg:ident, $cmp:ident) => {
        #[link(wasm_import_module = "engine_intrinsics")]
        extern "C" {
            pub fn $load(addr: i64) -> $abi;
            pub fn $store(addr: i64, value: $abi);
            pub fn $add(addr: i64, value: $abi) -> $abi;
            pub fn $sub(addr: i64, value: $abi) -> $abi;
            pub fn $and(addr: i64, value: $abi) -> $abi;
            pub fn $or(addr: i64, value: $abi) -> $abi;
            pub fn $xor(addr: i64, value: $abi) -> $abi;
            pub fn $xchg(addr: i64, value: $abi) -> $abi;
            pub fn $cmp(addr: i64, old: $abi, new: $abi) -> $abi;
        }
    };
}
atomics!(u32, a_load_u8, a_store_u8, a_rmw_add_u8, a_rmw_sub_u8, a_rmw_and_u8, a_rmw_or_u8, a_rmw_xor_u8, a_rmw_xchg_u8, a_rmw_cmpxchg_u8);
atomics!(u32, a_load_u16, a_store_u16, a_rmw_add_u16, a_rmw_sub_u16, a_rmw_and_u16, a_rmw_or_u16, a_rmw_xor_u16, a_rmw_xchg_u16, a_rmw_cmpxchg_u16);
atomics!(u32, a_load_u32, a_store_u32, a_rmw_add_u32, a_rmw_sub_u32, a_rmw_and_u32, a_rmw_or_u32, a_rmw_xor_u32, a_rmw_xchg_u32, a_rmw_cmpxchg_u32);
atomics!(u64, a_load_u64, a_store_u64, a_rmw_add_u64, a_rmw_sub_u64, a_rmw_and_u64, a_rmw_or_u64, a_rmw_xor_u64, a_rmw_xchg_u64, a_rmw_cmpxchg_u64);
