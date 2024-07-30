use bit_vec::BitVec;

// Compact Payload Encoding
pub trait CompactPayloadEncoding {
    fn to_cpe(&self) -> BitVec;
}