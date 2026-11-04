use wasm_rt::{simd, Phantom};

mod imp {}
mod syms {
    use wasm_rt::Phantom;
    pub unsafe extern "C" fn f_091076_cxa_guard_acquire(_: *mut Phantom, _: i64) -> i32 { 1 }
    pub unsafe extern "C" fn f_091077_cxa_guard_release(ph: *mut Phantom, address: i64) { (*ph).write_u8(address, 1); }
}

mod delivered;
mod before;

const POINT: i64 = 4096;
const MASKS: [u32; 3] = [0x3ff, 0xffc00, 0x3ff00000];
const GLOBALS: [i64; 3] = [13070784, 13070816, 13070848];

struct Context {
    bytes: Vec<u8>,
    ph: Box<Phantom>,
    before: bool,
    restricted: bool,
}

impl Context {
    fn new(before: bool, restricted: bool) -> Self {
        let mut bytes = vec![0; 14 * 1024 * 1024];
        let ph = Box::new(unsafe { Phantom::new(bytes.as_mut_ptr(), bytes.len() as u64) });
        unsafe {
            for i in 0..3 {
                ph.write_u32(6706236 + i as i64 * 4, MASKS[i]);
                if restricted {
                    ph.write_v128(GLOBALS[i], simd::pack_u32([0, 0, 0, MASKS[i]]));
                    ph.write_u8(GLOBALS[i] + 16, 1);
                }
            }
        }
        Self { bytes, ph, before, restricted }
    }

    fn pack(&mut self, position: [u32; 4], velocity: [u32; 4]) -> [u32; 4] {
        self.bytes[POINT as usize - 32..POINT as usize + 48].fill(0xa7);
        unsafe {
            self.ph.write_v128(POINT, simd::pack_u32(position));
            let function = if self.before {
                before::before_f_012912_rage_pack_core32_float_vector_4_ref_float_vector_4
            } else {
                delivered::f_012912_rage_pack_core32_float_vector_4_ref_float_vector_4
            };
            let value = function(&mut *self.ph, POINT, simd::pack_u32(velocity));
            assert_eq!(value, self.ph.read_v128(POINT));
            for i in 0..3 {
                let old_z_mask = if self.before && !self.restricted { MASKS[i] } else { 0 };
                assert_eq!(simd::unpack_u32(self.ph.read_v128(GLOBALS[i])), [0, 0, old_z_mask, MASKS[i]]);
            }
            assert!(self.bytes[POINT as usize - 32..POINT as usize].iter().all(|v| *v == 0xa7));
            assert!(self.bytes[POINT as usize + 16..POINT as usize + 48].iter().all(|v| *v == 0xa7));
            simd::unpack_u32(value)
        }
    }
}

fn next(seed: &mut u32) -> u32 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    *seed
}

fn check(actual: &mut Context, control: &mut Context, old: &mut Context, position: [u32; 4], velocity: [u32; 4]) {
    let fixed = actual.pack(position, velocity);
    let expected = control.pack(position, velocity);
    let original = old.pack(position, velocity);
    assert_eq!(&fixed[..3], &position[..3]);
    assert_eq!(fixed, expected);
    assert_eq!(fixed[3], original[3]);
}

fn main() {
    let mut actual = Context::new(false, false);
    let mut control = Context::new(true, true);
    let mut original = Context::new(true, false);
    let position = [0.125f32.to_bits(), (-0.25f32).to_bits(), 2.0f32.to_bits(), 0x20080200];
    let regression = original.pack(position, [0; 4]);
    assert_eq!(regression[2], 0x60080200);
    assert!(f32::from_bits(regression[2]) > 1e19);
    check(&mut actual, &mut control, &mut original, position, [0; 4]);
    let mut cases = 1;
    let mut seed = 0x7e5329acu32;
    for _ in 0..512 {
        let position = core::array::from_fn(|lane| {
            if lane == 3 { next(&mut seed) } else { ((next(&mut seed) as i32) as f32 / 1048576.0).to_bits() }
        });
        let x = ((next(&mut seed) as i32) as f32 / 1073741824.0).to_bits();
        let y = ((next(&mut seed) as i32) as f32 / 1073741824.0).to_bits();
        let z = ((next(&mut seed) as i32) as f32 / 1073741824.0).to_bits();
        let arbitrary_w = next(&mut seed);
        for velocity in [[0; 4], [x, y, z, 0], [x, y, z, arbitrary_w], [x, y, z, 0x7fc001fd]] {
            check(&mut actual, &mut control, &mut original, position, velocity);
            cases += 1;
        }
    }
    for value in [0, 0x80000000, 1, 0x80000001, 0x3f000000, 0x3fffffff, 0x40000000, 0xc0000000, 0x7f7fffff, 0xff7fffff] {
        for velocity in [[0; 4], [0, 0, 0, 150.0f32.to_bits()]] {
            check(&mut actual, &mut control, &mut original, [value, value, value, 0xffffffff], velocity);
            cases += 1;
        }
    }
    assert_eq!(cases, 2069);
    println!("{{\"author\":\"carunodev @stfu\",\"cases\":{cases},\"originalZeroVelocityZBits\":\"60080200\",\"xyzPreserved\":true,\"packedWUnchanged\":true,\"canariesPreserved\":true}}");
}
