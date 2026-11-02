use std::{path::PathBuf, sync::Arc, time::Instant};
use engine_host::{HeadlessConfig, HeadlessHost};
use game_server::mem::LinearMemory;
use wasm_rt::Phantom;

include!(concat!(env!("OUT_DIR"), "/module.rs"));

fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut stage = String::from("ctors");
    let mut data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("userdata");
    let mut ticks = 1u64;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--stage" => stage = args.next().ok_or("missing stage")?,
            "--data-dir" => data = args.next().ok_or("missing data directory")?.into(),
            "--ticks" => ticks = args.next().ok_or("missing ticks")?.parse()?,
            _ => return Err(format!("unknown argument {arg}").into()),
        }
    }
    if !["init", "ctors", "main", "frames"].contains(&stage.as_str()) { return Err("invalid stage".into()); }
    let start = Instant::now();
    let host = HeadlessHost::new(HeadlessConfig::new(data))?;
    let _guard = engine_abi::install(Arc::new(host));
    let memory = LinearMemory::reserve(INITIAL_PAGES * 65536)?;
    memory.commit(0, memory.len())?;
    let table = make_table();
    let mut ph = unsafe { Phantom::new(memory.base(), memory.len()) };
    ph.g[..GLOBAL_INIT.len()].copy_from_slice(GLOBAL_INIT);
    ph.sp = GLOBAL_INIT[0] as i64;
    ph.table = table.as_ptr();
    ph.tlen = table.len();
    ph.table_types = TABLE_TYPES.as_ptr();
    unsafe { engine_abi::set_memory(ph.base(), ph.msize as usize); }
    let _io_guard = engine_abi::httpfs::IoGuard;
    let context_address = &ph as *const Phantom as usize;
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        let ph = unsafe { &*(context_address as *const Phantom) };
        let word = |offset: u64| -> u64 {
            if offset.checked_add(8).is_some_and(|end| end <= ph.msize) {
                unsafe { core::ptr::read_unaligned(ph.base().wrapping_add(offset as usize) as *const u64) }
            } else { u64::MAX }
        };
        let master = word(ph.g[1] + 840);
        let current = word(ph.g[1] + 832);
        eprintln!("[linked] panic context: tls={:#x}, sp={:#x}, master={master:#x}, current={current:#x}", ph.g[1], ph.sp);
        for object in [master, current] {
            let vtable = word(object);
            eprintln!("[linked] allocator={object:#x}, vtable={vtable:#x}, slot72={}, slot112={}", word(vtable.wrapping_add(72)), word(vtable.wrapping_add(112)));
        }
        previous_hook(panic);
    }));
    eprintln!("[linked] {} table slots, {} MiB memory, stack {:#x}", table.len(), ph.msize / 1048576, ph.sp);
    unsafe {
        gen000::f_000088_wasm_init_memory::f_000088_wasm_init_memory(&mut ph);
        eprintln!("[linked] init PASS in {:?}", start.elapsed());
        if stage != "init" {
            gen000::f_000085_wasm_call_ctors::f_000085_wasm_call_ctors(&mut ph);
            eprintln!("[linked] ctors PASS in {:?}", start.elapsed());
        }
        if stage == "main" || stage == "frames" {
            let result = gen000::f_000103_main::f_000103_main(&mut ph, 0, 0);
            eprintln!("[linked] main returned {result} in {:?}", start.elapsed());
        }
        if stage == "frames" {
            gen000::f_000105_main_prologue::f_000105_main_prologue(&mut ph);
            for _ in 0..ticks { gen000::f_000106_main_oneloopiteration::f_000106_main_oneloopiteration(&mut ph); }
            eprintln!("[linked] completed {ticks} game iterations in {:?}", start.elapsed());
        }
    }
    engine_abi::state::clear_memory();
    Ok(())
}

fn main() {
    let result = std::thread::Builder::new().name("engine-main".into()).stack_size(64 * 1024 * 1024).spawn(run).expect("engine thread").join();
    match result {
        Ok(Ok(())) => {}
        other => { eprintln!("[linked] failed: {other:?}"); std::process::exit(1); }
    }
}
