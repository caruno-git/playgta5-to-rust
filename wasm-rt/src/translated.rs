use crate::{futex, segments, trap, Memory, PAGE_SIZE};
use core::sync::atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8, Ordering};

#[repr(C)]
pub struct Phantom {
    pub mem: *mut u8,
    pub msize: u64,
    pub sp: i64,
    pub g: [u64; 64],
    pub table: *const *const (),
    pub tlen: usize,
    pub table_types: *const u32,
    pub seglen: [usize; 4096],
    pub segoff: [u32; 4096],
    owned: Option<Memory>,
}

impl Phantom {
    pub const fn empty() -> Self {
        Self {
            mem: core::ptr::null_mut(),
            msize: 0,
            sp: 0,
            g: [0; 64],
            table: core::ptr::null(),
            tlen: 0,
            table_types: core::ptr::null(),
            seglen: [0; 4096],
            segoff: [0; 4096],
            owned: None,
        }
    }

    #[doc = "Safety: memory must remain allocated for msize bytes while this context is used. Concurrent accesses must obey Rust atomic and aliasing rules."]
    pub unsafe fn new(mem: *mut u8, msize: u64) -> Self {
        assert!(msize <= isize::MAX as u64);
        assert!(cfg!(target_family = "wasm") || !mem.is_null() || msize == 0);
        let mut p = Self::empty();
        p.mem = mem;
        p.msize = msize;
        for (i, &(offset, len)) in segments::SEG_TAB.iter().enumerate() {
            p.segoff[i] = offset;
            p.seglen[i] = len as usize;
        }
        p
    }

    pub fn with_memory(mut memory: Memory) -> Self {
        let mut p = unsafe { Self::new(memory.buf.as_mut_ptr(), memory.size()) };
        p.owned = Some(memory);
        p
    }

    pub fn base(&self) -> *mut u8 {
        self.mem
    }

    #[inline]
    fn range(&self, addr: i64, len: u64) -> usize {
        #[cfg(target_arch = "wasm64")]
        { let _ = len; return addr as usize; }
        #[cfg(not(target_arch = "wasm64"))]
        {
        let addr = addr as u64;
        if addr > self.msize || len > self.msize - addr {
            trap("out of bounds linear memory access")
        }
        usize::try_from(addr).unwrap_or_else(|_| trap("address does not fit host"))
        }
    }

    #[inline]
    fn atomic_addr(&self, addr: i64, width: usize) -> *mut u8 {
        #[cfg(target_arch = "wasm64")]
        { let _ = width; return addr as *mut u8; }
        #[cfg(not(target_arch = "wasm64"))]
        {
        let offset = self.range(addr, width as u64);
        let ptr = self.mem.wrapping_add(offset);
        if offset % width != 0 || ptr as usize % width != 0 {
            trap("unaligned atomic")
        }
        ptr
        }
    }

    #[inline(always)]
    pub fn memory_size(&self) -> i64 {
        #[cfg(target_arch = "wasm64")]
        { return unsafe { crate::guest_ops::memory_size() }; }
        #[cfg(not(target_arch = "wasm64"))]
        { (self.msize / PAGE_SIZE) as i64 }
    }

    #[inline]
    pub fn memory_grow(&mut self, delta: i64) -> i64 {
        #[cfg(target_arch = "wasm64")]
        { return unsafe { crate::guest_ops::memory_grow(delta) }; }
        #[cfg(not(target_arch = "wasm64"))]
        {
        if delta == 0 {
            return self.memory_size();
        }
        let Some(memory) = &mut self.owned else {
            return -1;
        };
        match memory.grow(delta as u64) {
            Ok(old) => {
                self.mem = memory.buf.as_mut_ptr();
                self.msize = memory.size();
                old as i64
            }
            Err(_) => -1,
        }
        }
    }

    #[inline(always)]
    pub unsafe fn memory_copy(&mut self, dst: i64, src: i64, len: i64) {
        #[cfg(target_arch = "wasm64")]
        { return crate::guest_ops::memory_copy(dst, src, len); }
        #[cfg(not(target_arch = "wasm64"))]
        {
        let dst = self.range(dst, len as u64);
        let src = self.range(src, len as u64);
        if len != 0 {
            core::ptr::copy(self.mem.wrapping_add(src), self.mem.wrapping_add(dst), len as usize);
        }
        }
    }

    #[inline(always)]
    pub unsafe fn memory_fill(&mut self, dst: i64, value: i32, len: i64) {
        #[cfg(target_arch = "wasm64")]
        { return crate::guest_ops::memory_fill(dst, value, len); }
        #[cfg(not(target_arch = "wasm64"))]
        {
        let dst = self.range(dst, len as u64);
        if len != 0 {
            core::ptr::write_bytes(self.mem.wrapping_add(dst), value as u8, len as usize);
        }
        }
    }

    pub unsafe fn memory_init(&mut self, segment: usize, dst: i64, src: i32, len: i32) {
        if segment >= segments::SEG_COUNT {
            trap("unknown data segment")
        }
        let src = src as u32 as usize;
        let len = len as u32 as usize;
        let limit = self.seglen[segment];
        if src > limit || len > limit - src {
            trap("data segment out of bounds")
        }
        let dst = self.range(dst, len as u64);
        let offset = self.segoff[segment] as usize;
        let bytes = segments::SEG_BLOB
            .get(offset + src..offset + src + len)
            .unwrap_or_else(|| trap("data segment out of bounds"));
        if len != 0 {
            #[cfg(target_arch = "wasm64")]
            crate::guest_ops::memory_init_copy(dst as i64, bytes.as_ptr() as i64, len as i64);
            #[cfg(not(target_arch = "wasm64"))]
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), self.mem.wrapping_add(dst), len);
        }
    }

    pub fn data_drop(&mut self, segment: usize) {
        if segment >= segments::SEG_COUNT {
            trap("unknown data segment")
        }
        self.seglen[segment] = 0;
    }

    #[inline(always)]
    pub unsafe fn table_entry(&self, index: i64) -> *const () {
        let i = usize::try_from(index as u64).unwrap_or_else(|_| trap("undefined table element"));
        if i >= self.tlen || self.table.is_null() {
            trap("undefined table element")
        }
        let entry = *self.table.add(i);
        if entry.is_null() {
            trap("uninitialized table element")
        }
        entry
    }

    #[inline(always)]
    pub unsafe fn table_entry_typed(&self, index: i64, signature: u32) -> *const () {
        let entry = self.table_entry(index);
        if self.table_types.is_null() || *self.table_types.add(index as usize) != signature {
            trap("indirect call type mismatch")
        }
        entry
    }

    pub unsafe fn a_wait32(&self, addr: i64, expected: i32, timeout: i64) -> i32 {
        #[cfg(target_arch = "wasm64")]
        { return crate::guest_ops::a_wait32(addr, expected, timeout); }
        #[cfg(not(target_arch = "wasm64"))]
        {
        let word = &*(self.atomic_addr(addr, 4) as *const AtomicU32);
        futex::wait(
            word as *const _ as usize,
            || word.load(Ordering::SeqCst) == expected as u32,
            timeout,
        )
        }
    }

    pub fn a_notify(&self, addr: i64, count: i32) -> i32 {
        #[cfg(target_arch = "wasm64")]
        { return unsafe { crate::guest_ops::a_notify(addr, count) }; }
        #[cfg(not(target_arch = "wasm64"))]
        { futex::notify(self.atomic_addr(addr, 4) as usize, count as u32) as i32 }
    }

    pub fn waiting(&self, addr: i64) -> u32 {
        futex::waiting(self.atomic_addr(addr, 4) as usize)
    }
}

macro_rules! integer_memory {
    ($read:ident, $write:ident, $ty:ty) => {
        impl Phantom {
            #[inline]
            pub unsafe fn $read(&self, addr: i64) -> $ty {
                #[cfg(target_arch = "wasm64")]
                { return crate::guest_ops::$read(addr); }
                #[cfg(not(target_arch = "wasm64"))]
                {
                let offset = self.range(addr, core::mem::size_of::<$ty>() as u64);
                <$ty>::from_le(core::ptr::read_unaligned(self.mem.wrapping_add(offset) as *const $ty))
                }
            }
            #[inline]
            pub unsafe fn $write(&self, addr: i64, value: $ty) {
                #[cfg(target_arch = "wasm64")]
                { return crate::guest_ops::$write(addr, value); }
                #[cfg(not(target_arch = "wasm64"))]
                {
                let offset = self.range(addr, core::mem::size_of::<$ty>() as u64);
                core::ptr::write_unaligned(self.mem.wrapping_add(offset) as *mut $ty, value.to_le());
                }
            }
        }
    };
}
integer_memory!(read_u8, write_u8, u8);
integer_memory!(read_u16, write_u16, u16);
integer_memory!(read_u32, write_u32, u32);
integer_memory!(read_u64, write_u64, u64);
integer_memory!(read_i8, write_i8, i8);
integer_memory!(read_i16, write_i16, i16);
integer_memory!(read_i32, write_i32, i32);

impl Phantom {
    #[inline(always)]
    pub unsafe fn read_v128(&self, addr: i64) -> u128 {
        #[cfg(target_arch = "wasm64")]
        { return core::mem::transmute(crate::guest_ops::read_v128(addr)); }
        #[cfg(not(target_arch = "wasm64"))]
        { core::ptr::read_unaligned(self.mem.wrapping_add(self.range(addr, 16)) as *const u128).to_le() }
    }
    #[inline(always)]
    pub unsafe fn write_v128(&self, addr: i64, value: u128) {
        #[cfg(target_arch = "wasm64")]
        { return crate::guest_ops::write_v128(addr, core::mem::transmute(value)); }
        #[cfg(not(target_arch = "wasm64"))]
        { core::ptr::write_unaligned(self.mem.wrapping_add(self.range(addr, 16)) as *mut u128, value.to_le()); }
    }
    #[inline(always)]
    pub unsafe fn read_f32(&self, addr: i64) -> f32 {
        #[cfg(target_arch = "wasm64")]
        { return crate::guest_ops::read_f32(addr); }
        #[cfg(not(target_arch = "wasm64"))]
        { f32::from_bits(self.read_u32(addr)) }
    }
    #[inline(always)]
    pub unsafe fn read_f64(&self, addr: i64) -> f64 {
        #[cfg(target_arch = "wasm64")]
        { return crate::guest_ops::read_f64(addr); }
        #[cfg(not(target_arch = "wasm64"))]
        { f64::from_bits(self.read_u64(addr)) }
    }
    #[inline(always)]
    pub unsafe fn write_f32(&self, addr: i64, v: f32) {
        #[cfg(target_arch = "wasm64")]
        { return crate::guest_ops::write_f32(addr, v); }
        #[cfg(not(target_arch = "wasm64"))]
        self.write_u32(addr, v.to_bits());
    }
    #[inline(always)]
    pub unsafe fn write_f64(&self, addr: i64, v: f64) {
        #[cfg(target_arch = "wasm64")]
        { return crate::guest_ops::write_f64(addr, v); }
        #[cfg(not(target_arch = "wasm64"))]
        self.write_u64(addr, v.to_bits());
    }

    #[inline(always)]
    pub unsafe fn read_splat(&self, addr: i64, width: u32) -> u128 {
        let v = match width {
            8 => self.read_u8(addr) as u128,
            16 => self.read_u16(addr) as u128,
            32 => self.read_u32(addr) as u128,
            64 => self.read_u64(addr) as u128,
            _ => trap("invalid SIMD width"),
        };
        let mut out = 0;
        for i in 0..128 / width {
            out |= v << (i * width);
        }
        out
    }

    #[inline(always)]
    pub unsafe fn load_lane(&self, addr: i64, value: u128, lane: u32, width: u32) -> u128 {
        if !matches!(width, 8 | 16 | 32 | 64) || lane >= 128 / width {
            trap("invalid SIMD lane")
        }
        let v = match width {
            8 => self.read_u8(addr) as u128,
            16 => self.read_u16(addr) as u128,
            32 => self.read_u32(addr) as u128,
            _ => self.read_u64(addr) as u128,
        };
        let shift = lane * width;
        (value & !(((1u128 << width) - 1) << shift)) | (v << shift)
    }

    #[inline(always)]
    pub unsafe fn store_lane(&self, addr: i64, value: u128, lane: u32, width: u32) {
        if !matches!(width, 8 | 16 | 32 | 64) || lane >= 128 / width {
            trap("invalid SIMD lane")
        }
        let v = value >> (lane * width);
        match width {
            8 => self.write_u8(addr, v as u8),
            16 => self.write_u16(addr, v as u16),
            32 => self.write_u32(addr, v as u32),
            _ => self.write_u64(addr, v as u64),
        }
    }
}

macro_rules! widening_load {
    ($name:ident, $narrow:ty, $wide:ty, $lanes:expr) => {
        impl Phantom {
            #[inline(always)]
            pub unsafe fn $name(&self, addr: i64) -> u128 {
                let mut out = [0u8; 16];
                let bits = self.read_u64(addr).to_le_bytes();
                let n = core::mem::size_of::<$narrow>();
                let w = core::mem::size_of::<$wide>();
                for i in 0..$lanes {
                    let mut b = [0u8; core::mem::size_of::<$narrow>()];
                    b.copy_from_slice(&bits[i * n..(i + 1) * n]);
                    let v = <$narrow>::from_le_bytes(b) as $wide;
                    out[i * w..(i + 1) * w].copy_from_slice(&v.to_le_bytes());
                }
                u128::from_le_bytes(out)
            }
        }
    };
}
widening_load!(read_8x8_s, i8, i16, 8);
widening_load!(read_8x8_u, u8, u16, 8);
widening_load!(read_16x4_s, i16, i32, 4);
widening_load!(read_16x4_u, u16, u32, 4);
widening_load!(read_32x2_s, i32, i64, 2);
widening_load!(read_32x2_u, u32, u64, 2);

pub trait AtomicValue: Copy {
    fn bits(self) -> u64;
    fn from_bits(bits: u64) -> Self;
}
impl AtomicValue for i32 {
    fn bits(self) -> u64 {
        self as u32 as u64
    }
    fn from_bits(bits: u64) -> Self {
        bits as i32
    }
}
impl AtomicValue for i64 {
    fn bits(self) -> u64 {
        self as u64
    }
    fn from_bits(bits: u64) -> Self {
        bits as i64
    }
}

macro_rules! atomic_memory {
    ($ty:ty, $atom:ty, $result:ty, $load:ident, $store:ident, $add:ident, $sub:ident,
     $and:ident, $or:ident, $xor:ident, $xchg:ident, $cmp:ident) => {
        impl Phantom {
            #[inline]
            pub unsafe fn $load(&self, addr: i64) -> $result {
                #[cfg(target_arch = "wasm64")]
                { return crate::guest_ops::$load(addr) as $result; }
                #[cfg(not(target_arch = "wasm64"))]
                {
                (&*(self.atomic_addr(addr, core::mem::size_of::<$ty>()) as *const $atom))
                    .load(Ordering::SeqCst) as $result
                }
            }
            #[inline]
            pub unsafe fn $store<T: AtomicValue>(&self, addr: i64, value: T) {
                #[cfg(target_arch = "wasm64")]
                { return crate::guest_ops::$store(addr, value.bits() as _); }
                #[cfg(not(target_arch = "wasm64"))]
                (&*(self.atomic_addr(addr, core::mem::size_of::<$ty>()) as *const $atom))
                    .store(value.bits() as $ty, Ordering::SeqCst);
            }
            #[inline]
            pub unsafe fn $add<T: AtomicValue>(&self, addr: i64, value: T) -> T {
                #[cfg(target_arch = "wasm64")]
                { return T::from_bits(crate::guest_ops::$add(addr, value.bits() as _) as u64); }
                #[cfg(not(target_arch = "wasm64"))]
                {
                T::from_bits(
                    (&*(self.atomic_addr(addr, core::mem::size_of::<$ty>()) as *const $atom))
                        .fetch_add(value.bits() as $ty, Ordering::SeqCst) as u64,
                )
                }
            }
            #[inline]
            pub unsafe fn $sub<T: AtomicValue>(&self, addr: i64, value: T) -> T {
                #[cfg(target_arch = "wasm64")]
                { return T::from_bits(crate::guest_ops::$sub(addr, value.bits() as _) as u64); }
                #[cfg(not(target_arch = "wasm64"))]
                {
                T::from_bits(
                    (&*(self.atomic_addr(addr, core::mem::size_of::<$ty>()) as *const $atom))
                        .fetch_sub(value.bits() as $ty, Ordering::SeqCst) as u64,
                )
                }
            }
            #[inline]
            pub unsafe fn $and<T: AtomicValue>(&self, addr: i64, value: T) -> T {
                #[cfg(target_arch = "wasm64")]
                { return T::from_bits(crate::guest_ops::$and(addr, value.bits() as _) as u64); }
                #[cfg(not(target_arch = "wasm64"))]
                {
                T::from_bits(
                    (&*(self.atomic_addr(addr, core::mem::size_of::<$ty>()) as *const $atom))
                        .fetch_and(value.bits() as $ty, Ordering::SeqCst) as u64,
                )
                }
            }
            #[inline]
            pub unsafe fn $or<T: AtomicValue>(&self, addr: i64, value: T) -> T {
                #[cfg(target_arch = "wasm64")]
                { return T::from_bits(crate::guest_ops::$or(addr, value.bits() as _) as u64); }
                #[cfg(not(target_arch = "wasm64"))]
                {
                T::from_bits(
                    (&*(self.atomic_addr(addr, core::mem::size_of::<$ty>()) as *const $atom))
                        .fetch_or(value.bits() as $ty, Ordering::SeqCst) as u64,
                )
                }
            }
            #[inline]
            pub unsafe fn $xor<T: AtomicValue>(&self, addr: i64, value: T) -> T {
                #[cfg(target_arch = "wasm64")]
                { return T::from_bits(crate::guest_ops::$xor(addr, value.bits() as _) as u64); }
                #[cfg(not(target_arch = "wasm64"))]
                {
                T::from_bits(
                    (&*(self.atomic_addr(addr, core::mem::size_of::<$ty>()) as *const $atom))
                        .fetch_xor(value.bits() as $ty, Ordering::SeqCst) as u64,
                )
                }
            }
            #[inline]
            pub unsafe fn $xchg<T: AtomicValue>(&self, addr: i64, value: T) -> T {
                #[cfg(target_arch = "wasm64")]
                { return T::from_bits(crate::guest_ops::$xchg(addr, value.bits() as _) as u64); }
                #[cfg(not(target_arch = "wasm64"))]
                {
                T::from_bits(
                    (&*(self.atomic_addr(addr, core::mem::size_of::<$ty>()) as *const $atom))
                        .swap(value.bits() as $ty, Ordering::SeqCst) as u64,
                )
                }
            }
            #[inline]
            pub unsafe fn $cmp<T: AtomicValue>(&self, addr: i64, old: T, new: T) -> T {
                #[cfg(target_arch = "wasm64")]
                { return T::from_bits(crate::guest_ops::$cmp(addr, old.bits() as _, new.bits() as _) as u64); }
                #[cfg(not(target_arch = "wasm64"))]
                {
                let atom = &*(self.atomic_addr(addr, core::mem::size_of::<$ty>()) as *const $atom);
                let prev = match atom.compare_exchange(
                    old.bits() as $ty,
                    new.bits() as $ty,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                ) {
                    Ok(v) | Err(v) => v,
                };
                T::from_bits(prev as u64)
                }
            }
        }
    };
}
atomic_memory!(
    u8,
    AtomicU8,
    i32,
    a_load_u8,
    a_store_u8,
    a_rmw_add_u8,
    a_rmw_sub_u8,
    a_rmw_and_u8,
    a_rmw_or_u8,
    a_rmw_xor_u8,
    a_rmw_xchg_u8,
    a_rmw_cmpxchg_u8
);
atomic_memory!(
    u16,
    AtomicU16,
    i32,
    a_load_u16,
    a_store_u16,
    a_rmw_add_u16,
    a_rmw_sub_u16,
    a_rmw_and_u16,
    a_rmw_or_u16,
    a_rmw_xor_u16,
    a_rmw_xchg_u16,
    a_rmw_cmpxchg_u16
);
atomic_memory!(
    u32,
    AtomicU32,
    i32,
    a_load_u32,
    a_store_u32,
    a_rmw_add_u32,
    a_rmw_sub_u32,
    a_rmw_and_u32,
    a_rmw_or_u32,
    a_rmw_xor_u32,
    a_rmw_xchg_u32,
    a_rmw_cmpxchg_u32
);
atomic_memory!(
    u64,
    AtomicU64,
    i64,
    a_load_u64,
    a_store_u64,
    a_rmw_add_u64,
    a_rmw_sub_u64,
    a_rmw_and_u64,
    a_rmw_or_u64,
    a_rmw_xor_u64,
    a_rmw_xchg_u64,
    a_rmw_cmpxchg_u64
);
