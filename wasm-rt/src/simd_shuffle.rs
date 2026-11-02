#[inline(always)]
pub fn i8x16_shuffle_static<
    const I0: usize, const I1: usize, const I2: usize, const I3: usize,
    const I4: usize, const I5: usize, const I6: usize, const I7: usize,
    const I8: usize, const I9: usize, const I10: usize, const I11: usize,
    const I12: usize, const I13: usize, const I14: usize, const I15: usize,
>(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "wasm64")]
    unsafe {
        core::mem::transmute(core::arch::wasm64::i8x16_shuffle::<
            I0,I1,I2,I3,I4,I5,I6,I7,I8,I9,I10,I11,I12,I13,I14,I15,
        >(core::mem::transmute(a), core::mem::transmute(b)))
    }
    #[cfg(not(target_arch = "wasm64"))]
    {
        let lanes = [I0,I1,I2,I3,I4,I5,I6,I7,I8,I9,I10,I11,I12,I13,I14,I15];
        assert!(lanes.iter().all(|&i| i < 32));
        let mask = u128::from_le_bytes(lanes.map(|i| i as u8));
        crate::simd::i8x16_shuffle(a, b, mask)
    }
}
