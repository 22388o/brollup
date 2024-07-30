pub mod account;
pub mod value;
pub mod contract;

use bit_vec::BitVec;

// Compact Payload Encoding
pub trait CompactPayloadEncoding {
    fn to_cpe(&self) -> BitVec;
}