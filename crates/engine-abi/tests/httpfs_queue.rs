use std::{fs, sync::Arc, thread, time::{Duration, Instant, SystemTime, UNIX_EPOCH}};
use engine_host::{HeadlessConfig, HeadlessHost};
use wasm_rt::Phantom;

fn until(mut ready: impl FnMut() -> bool) {
    let start = Instant::now();
    while !ready() {
        assert!(start.elapsed() < Duration::from_secs(2), "I/O worker did not complete");
        thread::sleep(Duration::from_millis(1));
    }
}

unsafe fn request(ph: &Phantom, id: i32, offset: u64, dst: u64, len: u32) -> i32 {
    unsafe {
    let slot = 0x140;
    ph.write_u32(slot + 4, id as u32);
    ph.write_u64(slot + 8, offset);
    ph.write_u64(slot + 16, dst);
    ph.write_u32(slot + 24, len);
    ph.a_store_u32(slot, 1);
    ph.a_rmw_add_u32(0x100, 1i32);
    ph.a_notify(0x100, 1);
    until(|| ph.a_load_u32(slot) == 2);
    ph.a_load_u32(slot + 28)
    }
}

#[test]
fn queued_reads_complete_with_data_eof_errors_and_hint_ordering() {
    let root = std::env::temp_dir().join(format!("engine-queue-{}-{}", std::process::id(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("a.bin"), b"hello world!").unwrap();
    fs::write(root.join("manifest.tsv"), "0\t12\ta.bin\n").unwrap();
    let host = HeadlessHost::new(HeadlessConfig::new(root.join("userdata")).with_asset_root(&root)).unwrap();
    let _host = engine_abi::install(Arc::new(host));
    let mut memory = vec![0u8; 16384];
    unsafe { engine_abi::set_memory(memory.as_mut_ptr(), memory.len()); }
    let ph = unsafe { Phantom::new(memory.as_mut_ptr(), memory.len() as u64) };
    let io_guard = engine_abi::httpfs::IoGuard;
    unsafe {
        ph.a_store_u32(0x104, 1);
        ph.a_store_u32(0x108, 128);
        ph.a_store_u32(0x10c, 2);
        ph.a_store_u32(0x110, 1);
        engine_abi::httpfs::wasm_httpfs_io_start_js(0x100usize as *mut u8);
        assert_eq!(request(&ph, 0, 7, 2048, 10), 5);
        assert_eq!(&memory[2048..2053], b"orld!");
        assert_eq!(request(&ph, 0, 100, 2100, 4), 0);
        assert_eq!(request(&ph, 99, 0, 2100, 4), -1);
        assert_eq!(request(&ph, 0, 0, 16383, 4), -1);
        assert_eq!(ph.a_load_u32(0x114), 0);
        ph.a_store_u32(0x30c, 4096);
        ph.a_rmw_add_u32(0x100, 1i32);
        ph.a_notify(0x100, 1);
        until(|| ph.a_load_u32(0x114) == 1);
        assert_eq!(ph.a_load_u32(0x30c), 0);
    }
    drop(io_guard);
    engine_abi::state::clear_memory();
    fs::remove_dir_all(root).unwrap();
}
