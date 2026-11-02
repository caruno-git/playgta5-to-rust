use std::{fs, path::Path};
use wasm_rt::{segments, Memory, Phantom};
use wasmparser::{DataKind, Parser, Payload};

mod imp {}
mod syms {}
#[path = "../../linked/generated/gen000/src/f_000091_getgameframe.rs"]
mod original_frame;
#[path = "../../linked/generated/gen000/src/f_000088_wasm_init_memory.rs"]
mod original_init;

#[test]
fn segment_bytes_match_the_original_binary() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../inputs/game.wasm");
    let bytes = fs::read(path).expect("original game.wasm");
    let mut count = 0;
    for payload in Parser::new(0).parse_all(&bytes) {
        if let Payload::DataSection(reader) = payload.expect("valid wasm") {
            for data in reader {
                let data = data.expect("valid data segment");
                assert!(matches!(data.kind, DataKind::Passive));
                assert_eq!(
                    segments::segment(count).expect("runtime segment"),
                    data.data,
                    "segment {count}"
                );
                count += 1;
            }
        }
    }
    assert_eq!(count as usize, segments::SEG_COUNT);
    assert_eq!(segments::SEG_BLOB.len(), 7_818_183);
}

#[test]
#[ignore = "Requires independent inputs/va.bin reference image"]
fn translated_init_executes_and_matches_the_original_image() {
    let baseline = fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("../inputs/va.bin"))
        .expect("original image");
    let memory = Memory::new(512, 512).expect("32 MiB memory");
    let mut ph = Phantom::with_memory(memory);
    unsafe {
        original_init::f_000088_wasm_init_memory(&mut ph);
        let image = std::slice::from_raw_parts(ph.base(), baseline.len());
        assert_eq!(image.iter().zip(&baseline).position(|(a, b)| a != b), None);
        assert_eq!(ph.a_load_u32(29_604_488), 2);
        assert_eq!(ph.seglen[0], 0);
        assert_eq!(ph.seglen[4075], 0);
        assert_eq!(ph.seglen[4076], segments::SEG_LENS[4076] as usize);
        original_init::f_000088_wasm_init_memory(&mut ph);
        ph.write_u32(12_321_252, 12345);
        assert_eq!(original_frame::f_000091_getgameframe(&mut ph), 12345);
    }
}
