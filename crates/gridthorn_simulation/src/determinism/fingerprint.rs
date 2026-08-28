const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// An incremental FNV-1a fingerprint for explicitly encoded authoritative state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateFingerprint {
    value: u64,
}

impl Default for StateFingerprint {
    fn default() -> Self {
        Self::new()
    }
}

impl StateFingerprint {
    /// Creates an empty fingerprint.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            value: FNV_OFFSET_BASIS,
        }
    }

    /// Appends bytes in their supplied order.
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.value ^= u64::from(*byte);
            self.value = self.value.wrapping_mul(FNV_PRIME);
        }
    }

    /// Appends an unsigned integer using little-endian encoding.
    pub fn write_u64(&mut self, value: u64) {
        self.write_bytes(&value.to_le_bytes());
    }

    /// Appends a signed integer using little-endian two's-complement encoding.
    pub fn write_i64(&mut self, value: i64) {
        self.write_bytes(&value.to_le_bytes());
    }

    /// Returns the completed fingerprint value.
    #[must_use]
    pub const fn finish(&self) -> u64 {
        self.value
    }
}
