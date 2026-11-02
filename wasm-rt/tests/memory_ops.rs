//! `memory.init` / `data.drop` / `memory.copy` / `memory.fill` semantics.
//!
//! The cases that matter for a translated module are the trapping corners:
//! a `memory.init` whose source range runs past the segment, one whose
//! destination runs past memory, and the same instruction after `data.drop`
//! (where the segment length is zero, so only a zero-length init succeeds).

use wasm_rt::segments::{segment, SEG_BLOB, SEG_COUNT, SEG_LENS, SEG_OFFSETS};
use wasm_rt::{
    data_drop, memory_copy, memory_fill, memory_init, Access, Memory, SegmentState, Trap, PAGE_SIZE,
};

fn mem(pages: u64) -> Memory {
    Memory::new(pages, pages).expect("memory allocation")
}

// ---------------------------------------------------------------------------
// segment blob self-consistency
// ---------------------------------------------------------------------------

#[test]
fn segment_table_matches_the_blob() {
    assert_eq!(SEG_COUNT, 4_077);
    assert_eq!(SEG_LENS.len(), SEG_COUNT);
    assert_eq!(SEG_OFFSETS.len(), SEG_COUNT);

    let mut total = 0u64;
    for i in 0..SEG_COUNT {
        assert_eq!(
            SEG_OFFSETS[i], total as u32,
            "offset {i} is not a prefix sum"
        );
        total += SEG_LENS[i] as u64;
    }
    assert_eq!(
        total,
        SEG_BLOB.len() as u64,
        "blob length disagrees with SEG_LENS"
    );
}

#[test]
fn every_segment_is_a_slice_of_the_original_blob() {
    for i in 0..SEG_COUNT {
        let bytes = segment(i as u32).expect("segment in range");
        assert_eq!(bytes.len(), SEG_LENS[i] as usize);
        let off = SEG_OFFSETS[i] as usize;
        assert_eq!(bytes, &SEG_BLOB[off..off + bytes.len()]);
    }
}

#[test]
fn segment_out_of_range_is_none() {
    assert!(segment(SEG_COUNT as u32).is_none());
    assert!(segment(u32::MAX).is_none());
}

// ---------------------------------------------------------------------------
// memory.init, happy path
// ---------------------------------------------------------------------------

#[test]
fn init_copies_the_whole_segment() {
    let mut m = mem(128);
    let mut segs = SegmentState::new();
    let len = SEG_LENS[0] as u64;
    assert!(len > 0);
    memory_init(&mut m, &segs, 0, 0x100, 0, len).expect("init");
    assert_eq!(m.read(0x100, len).expect("read"), segment(0).expect("seg"));

    // partial copies, including the last byte
    memory_init(&mut m, &segs, 3, 0x500000, 0, 1).expect("init 1 byte");
    assert_eq!(
        m.read(0x500000, 1).expect("read"),
        &segment(3).expect("seg")[..1]
    );
    let last = len - 1;
    memory_init(&mut m, &segs, 0, 0x500100, last, 1).expect("init tail");
    assert_eq!(
        m.read(0x500100, 1).expect("read"),
        &segment(0).expect("seg")[last as usize..]
    );

    // dropping the state does not affect what was already copied
    data_drop(&mut segs, 0).expect("drop");
    assert_eq!(m.read(0x100, len).expect("read"), segment(0).expect("seg"));
}

#[test]
fn init_zero_length_is_allowed_at_the_end_of_a_segment_and_of_memory() {
    let mut m = mem(1);
    let segs = SegmentState::new();
    let seglen = SEG_LENS[3] as u64;
    assert!(seglen > 0);
    // src == seglen, len == 0: nothing to read, no trap
    memory_init(&mut m, &segs, 3, 0, seglen, 0).expect("zero-length init at segment end");
    // dst == size, len == 0: no bytes to write, no trap
    memory_init(&mut m, &segs, 3, PAGE_SIZE, 0, 0).expect("zero-length init at memory end");
    // ... but one byte past either boundary traps
    assert!(matches!(
        memory_init(&mut m, &segs, 3, 0, seglen, 1),
        Err(Trap::SegmentOutOfRange { .. })
    ));
    assert!(matches!(
        memory_init(&mut m, &segs, 3, PAGE_SIZE, 0, 1),
        Err(Trap::OutOfBounds { .. })
    ));
}

// ---------------------------------------------------------------------------
// memory.init, bounds traps
// ---------------------------------------------------------------------------

#[test]
fn init_past_the_segment_traps_and_writes_nothing() {
    let mut m = mem(1);
    let segs = SegmentState::new();
    let len = SEG_LENS[0] as u64;
    let before = m.read(0, 64).expect("read").to_vec();

    let err = memory_init(&mut m, &segs, 0, 0, 1, len).expect_err("must trap");
    match err {
        Trap::SegmentOutOfRange {
            segment,
            src,
            len: l,
            seglen,
        } => {
            assert_eq!(segment, 0);
            assert_eq!(src, 1);
            assert_eq!(l, len);
            assert_eq!(seglen, len);
        }
        other => panic!("wrong trap: {other:?}"),
    }
    assert_eq!(
        m.read(0, 64).expect("read"),
        &before[..],
        "trapping init wrote bytes"
    );
}

#[test]
fn init_past_memory_traps_and_writes_nothing() {
    let mut m = mem(1);
    let segs = SegmentState::new();
    let len = SEG_LENS[0] as u64;
    let dst = PAGE_SIZE - 4; // room for 4 of `len` bytes
    let before = m.read(dst, 4).expect("read").to_vec();

    let err = memory_init(&mut m, &segs, 0, dst, 0, len).expect_err("must trap");
    match err {
        Trap::OutOfBounds {
            access,
            addr,
            len: l,
            size,
        } => {
            assert_eq!(access, Access::Init);
            assert_eq!(addr, dst);
            assert_eq!(l, len);
            assert_eq!(size, PAGE_SIZE);
        }
        other => panic!("wrong trap: {other:?}"),
    }
    assert_eq!(
        m.read(dst, 4).expect("read"),
        &before[..],
        "trapping init wrote bytes"
    );
}

#[test]
fn init_with_huge_len_traps_instead_of_overflowing() {
    let mut m = mem(1);
    let segs = SegmentState::new();
    assert!(matches!(
        memory_init(&mut m, &segs, 0, 0, u64::MAX, 2),
        Err(Trap::SegmentOutOfRange { .. })
    ));
    assert!(matches!(
        memory_init(&mut m, &segs, 0, u64::MAX, 0, 2),
        Err(Trap::OutOfBounds { .. })
    ));
}

#[test]
fn init_with_unknown_segment_traps() {
    let mut m = mem(1);
    let mut segs = SegmentState::new();
    assert!(matches!(
        memory_init(&mut m, &segs, SEG_COUNT as u32, 0, 0, 0),
        Err(Trap::NoSuchSegment { .. })
    ));
    assert!(matches!(
        data_drop(&mut segs, SEG_COUNT as u32),
        Err(Trap::NoSuchSegment { .. })
    ));
}

// ---------------------------------------------------------------------------
// data.drop
// ---------------------------------------------------------------------------

#[test]
fn drop_makes_later_non_empty_init_trap() {
    let mut m = mem(128);
    let mut segs = SegmentState::new();
    let len = SEG_LENS[0] as u64;

    memory_init(&mut m, &segs, 0, 0, 0, len).expect("init before drop");
    assert!(!segs.is_dropped(0));
    data_drop(&mut segs, 0).expect("drop");
    assert!(segs.is_dropped(0));
    assert_eq!(segs.len_of(0), Some(0), "a dropped segment has length 0");
    assert_eq!(segs.bytes(0).expect("bytes").len(), 0);

    // zero-length init from a dropped segment still succeeds (src 0, len 0)
    memory_init(&mut m, &segs, 0, 0, 0, 0).expect("zero-length init after drop");

    let err = memory_init(&mut m, &segs, 0, 0, 0, 1).expect_err("must trap");
    match err {
        Trap::SegmentOutOfRange { seglen, .. } => assert_eq!(seglen, 0),
        other => panic!("wrong trap: {other:?}"),
    }
}

#[test]
fn drop_is_idempotent_and_touches_only_its_segment() {
    let mut segs = SegmentState::new();
    data_drop(&mut segs, 7).expect("drop");
    data_drop(&mut segs, 7).expect("drop again");
    assert!(segs.is_dropped(7));
    assert!(!segs.is_dropped(8));
    assert_eq!(segs.len_of(8), Some(SEG_LENS[8]));
}

#[test]
fn dropped_segment_is_not_revived_by_a_later_init() {
    let mut m = mem(1);
    let mut segs = SegmentState::new();
    data_drop(&mut segs, 5).expect("drop");
    assert!(memory_init(&mut m, &segs, 5, 0, 0, 1).is_err());
    // the failure must not resurrect the segment
    assert_eq!(segs.len_of(5), Some(0));
    assert!(memory_init(&mut m, &segs, 5, 0, 0, 1).is_err());
}

// ---------------------------------------------------------------------------
// memory.copy
// ---------------------------------------------------------------------------

#[test]
fn copy_matches_memmove_in_both_overlap_directions() {
    for (dst, src, len) in [(0u64, 8u64, 64u64), (8, 0, 64), (0, 0, 32), (100, 90, 20)] {
        let mut m = mem(1);
        let pattern: Vec<u8> = (0..128u32).map(|i| (i * 7 + 3) as u8).collect();
        m.write(0, &pattern).expect("seed");
        memory_copy(&mut m, dst, src, len).expect("copy");

        let mut want = pattern.clone();
        want.copy_within(src as usize..(src + len) as usize, dst as usize);
        assert_eq!(
            m.read(0, 128).expect("read"),
            &want[..],
            "copy dst={dst} src={src} len={len}"
        );
    }
}

#[test]
fn copy_bounds_are_checked_for_both_ranges() {
    let mut m = mem(1);
    let before = m.read(0, 32).expect("read").to_vec();
    assert!(matches!(
        memory_copy(&mut m, PAGE_SIZE - 8, 0, 16),
        Err(Trap::OutOfBounds {
            access: Access::CopyDst,
            ..
        })
    ));
    assert!(matches!(
        memory_copy(&mut m, 0, PAGE_SIZE - 8, 16),
        Err(Trap::OutOfBounds {
            access: Access::CopySrc,
            ..
        })
    ));
    assert_eq!(m.read(0, 32).expect("read"), &before[..]);
    // zero-length copy at the very end is fine
    memory_copy(&mut m, PAGE_SIZE, PAGE_SIZE, 0).expect("zero-length copy");
}

// ---------------------------------------------------------------------------
// memory.fill
// ---------------------------------------------------------------------------

#[test]
fn fill_repeats_the_low_byte_and_checks_bounds() {
    let mut m = mem(1);
    memory_fill(&mut m, 16, -1, 8).expect("fill"); // -1 -> 0xFF
    assert_eq!(m.read(16, 8).expect("read"), &[0xFFu8; 8]);
    memory_fill(&mut m, 32, 0x1234, 4).expect("fill");
    assert_eq!(m.read(32, 4).expect("read"), &[0x34u8; 4]);

    let before = m.read(0, 16).expect("read").to_vec();
    assert!(matches!(
        memory_fill(&mut m, PAGE_SIZE - 4, 0xAA, 8),
        Err(Trap::OutOfBounds {
            access: Access::Fill,
            ..
        })
    ));
    assert_eq!(m.read(0, 16).expect("read"), &before[..]);
    memory_fill(&mut m, PAGE_SIZE, 0xAA, 0).expect("zero-length fill");
}

// ---------------------------------------------------------------------------
// growth and plain accessors
// ---------------------------------------------------------------------------

#[test]
fn grow_extends_zeroed_memory_and_respects_the_maximum() {
    let mut m = Memory::new(1, 2).expect("memory");
    assert_eq!(m.pages(), 1);
    assert_eq!(m.grow(1).expect("grow"), 1);
    assert_eq!(m.size(), 2 * PAGE_SIZE);
    assert_eq!(m.read(PAGE_SIZE, 8).expect("read"), &[0u8; 8]);
    assert!(matches!(
        m.grow(1),
        Err(Trap::GrowLimit { want: 3, max: 2 })
    ));
    // still usable at the old size
    m.write(PAGE_SIZE * 2 - 1, &[7]).expect("last byte");
    assert_eq!(m.read(PAGE_SIZE * 2 - 1, 1).expect("read"), &[7]);
}

#[test]
fn plain_accessors_trap_instead_of_panicking() {
    let mut m = mem(1);
    assert!(matches!(
        m.read(PAGE_SIZE - 2, 4),
        Err(Trap::OutOfBounds {
            access: Access::Load,
            ..
        })
    ));
    assert!(matches!(
        m.write(PAGE_SIZE - 2, &[1, 2, 3, 4]),
        Err(Trap::OutOfBounds {
            access: Access::Store,
            ..
        })
    ));
}

#[test]
fn later_init_overwrites_earlier_bytes() {
    // overlapping destinations: the last init wins for the overlap
    let mut m = mem(1);
    let segs = SegmentState::new();
    memory_init(&mut m, &segs, 0, 0, 0, 32).expect("first");
    memory_init(&mut m, &segs, 3, 16, 0, 16).expect("second");
    assert_eq!(
        m.read(0, 16).expect("read"),
        &segment(0).expect("seg")[..16]
    );
    assert_eq!(
        m.read(16, 16).expect("read"),
        &segment(3).expect("seg")[..16]
    );
}
