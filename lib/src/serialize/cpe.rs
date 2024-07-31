use bit_vec::BitVec;

pub trait CompactPayloadEncoding {
    fn to_cpe(&self) -> BitVec;
}
