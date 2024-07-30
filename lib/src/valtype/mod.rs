pub mod account;
pub mod contract;
pub mod value;
pub mod maybe_common;

use bit_vec::BitVec;

// Compact Payload Encoding
pub trait CompactPayloadEncoding {
    fn to_cpe(&self) -> BitVec;
}