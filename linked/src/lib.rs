#![cfg(target_arch = "wasm64")]
#![allow(improper_ctypes_definitions)]

use std::sync::OnceLock;
use wasm_rt::Phantom;

include!(concat!(env!("OUT_DIR"), "/module.rs"));

#[link(wasm_import_module = "env")]
extern "C" {
    fn engine_context() -> i64;
    fn engine_debug(ptr: i64, len: i32);
}

unsafe fn context() -> *mut Phantom { engine_context() as *mut Phantom }

#[no_mangle]
pub unsafe extern "C" fn engine_new_context() -> i64 {
    static TABLE: OnceLock<(usize, usize)> = OnceLock::new();
    let &(table, len) = TABLE.get_or_init(|| {
        let table = make_table().into_boxed_slice();
        let len = table.len();
        (Box::into_raw(table) as *mut *const () as usize, len)
    });
    let mut ph = Box::new(Phantom::new(core::ptr::null_mut(), INITIAL_PAGES * 65536));
    ph.g[..GLOBAL_INIT.len()].copy_from_slice(GLOBAL_INIT);
    ph.sp = GLOBAL_INIT[0] as i64;
    ph.table = table as *const *const ();
    ph.tlen = len;
    ph.table_types = TABLE_TYPES.as_ptr();
    gen000::f_000088_wasm_init_memory::f_000088_wasm_init_memory(&mut *ph);
    std::panic::set_hook(Box::new(|panic| {
        let text = format!("Rust engine: {panic}");
        unsafe { engine_debug(text.as_ptr() as i64, text.len() as i32); }
    }));
    Box::into_raw(ph) as i64
}

#[no_mangle]
pub extern "C" fn engine_allocate_stack(requested: u32) -> i64 {
    let size = (requested as usize).clamp(2 * 1024 * 1024, 16 * 1024 * 1024);
    let memory = Vec::<u8>::with_capacity(size);
    let top = memory.as_ptr() as usize + memory.capacity();
    core::mem::forget(memory);
    (top & !15) as i64
}

#[no_mangle]
pub extern "C" fn engine_function_type(slot: i64) -> u32 {
    TABLE_TYPES.get(slot as usize).copied().unwrap_or(u32::MAX)
}

include!(concat!(env!("OUT_DIR"), "/adapters.rs"));
