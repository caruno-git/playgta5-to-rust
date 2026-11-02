use std::{env, fs, path::PathBuf, process::ExitCode};
use wasm_rt::{init_events::INIT, memory_init, Memory, SegmentState, PAGE_SIZE};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let reference = match args.next().as_deref() {
        None => PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../inputs/va.bin"),
        Some("--va-bin") => PathBuf::from(args.next().ok_or("--va-bin requires a path")?),
        Some(_) => return Err("expected --va-bin <path>".into()),
    };
    if args.next().is_some() {
        return Err("unexpected argument".into());
    }
    let extent = INIT
        .iter()
        .map(|&(_, dst, _, len)| dst + len as u64)
        .max()
        .ok_or("no init events")?;
    let pages = extent.div_ceil(PAGE_SIZE);
    let mut memory = Memory::new(pages, pages)?;
    let state = SegmentState::new();
    for &(segment, dst, src, len) in &INIT {
        memory_init(&mut memory, &state, segment, dst, src as u64, len as u64)?;
    }
    let baseline = fs::read(&reference)?;
    if baseline.len() as u64 != extent {
        return Err("reference length differs from image extent".into());
    }
    if let Some(offset) = memory.as_slice()[..baseline.len()]
        .iter()
        .zip(&baseline)
        .position(|(a, b)| a != b)
    {
        return Err(format!("image mismatch at {offset:#x}").into());
    }
    println!(
        "PASS: {} original init events, {} segment bytes, image {} bytes identical to {}",
        INIT.len(),
        wasm_rt::segments::SEG_BLOB.len(),
        extent,
        reference.display()
    );
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("init_check: FAIL: {e}");
            ExitCode::FAILURE
        }
    }
}
