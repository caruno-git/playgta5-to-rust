//! Stable state hashing for determinism checks.
//!
//! FNV-1a/64 — trivially portable, no std hashing randomness (unlike
//! `std::collections::hash_map::DefaultHasher`, which is seeded per process and
//! therefore useless for comparing runs). The server writes the same hasher
//! over the same serialized tick state every frame.

/// FNV-1a 64-bit offset basis.
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
/// FNV-1a 64-bit prime.
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// Incremental FNV-1a/64 hasher over tick state.
#[derive(Debug, Clone)]
pub struct StateHasher {
    state: u64,
}

impl Default for StateHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl StateHasher {
    /// Start a fresh hash.
    pub const fn new() -> Self {
        Self { state: FNV_OFFSET }
    }

    /// Mix in raw bytes.
    pub fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.state ^= b as u64;
            self.state = self.state.wrapping_mul(FNV_PRIME);
        }
    }

    /// Mix in a little-endian u64.
    pub fn write_u64(&mut self, v: u64) {
        self.write(&v.to_le_bytes());
    }

    /// Mix in a little-endian u32.
    pub fn write_u32(&mut self, v: u32) {
        self.write(&v.to_le_bytes());
    }

    /// Mix in a little-endian i32 (the engine's frame counter storage).
    pub fn write_i32(&mut self, v: i32) {
        self.write(&v.to_le_bytes());
    }

    /// Finalize. Feeding the same byte sequence always yields the same digest.
    pub const fn finish(&self) -> u64 {
        self.state
    }
}

/// One-shot FNV-1a/64 over a byte slice.
pub fn hash_bytes(bytes: &[u8]) -> u64 {
    let mut h = StateHasher::new();
    h.write(bytes);
    h.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_vectors() {
        // FNV-1a/64 reference vectors.
        assert_eq!(hash_bytes(b""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(hash_bytes(b"a"), 0xaf63_dc4c_8601_ec8c);
        assert_eq!(hash_bytes(b"foobar"), 0x85944171f73967e8);
    }

    #[test]
    fn order_sensitive() {
        let mut a = StateHasher::new();
        a.write_u32(1);
        a.write_u32(2);
        let mut b = StateHasher::new();
        b.write_u32(2);
        b.write_u32(1);
        assert_ne!(a.finish(), b.finish());
    }
}
