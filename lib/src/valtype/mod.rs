pub mod account;
pub mod contract;
pub mod value;

use std::u8;

use account::Account;
use bit_vec::BitVec;
use contract::Contract;
use value::ShortVal;

// Compact Payload Encoding
pub trait CompactPayloadEncoding {
    fn to_cpe(&self) -> BitVec;
}

pub trait MaybeCommonType {}

impl MaybeCommonType for Account {}
impl MaybeCommonType for Contract {}
impl MaybeCommonType for ShortVal {}

#[derive(Clone, Copy)]
pub enum MaybeCommon<T: MaybeCommonType + CompactPayloadEncoding> {
    Common(T, u8),
    Uncommon(T),
}

impl<T: MaybeCommonType + CompactPayloadEncoding> CompactPayloadEncoding for MaybeCommon<T> {
    fn to_cpe(&self) -> BitVec {
        let mut bit_vec = BitVec::new();

        match self {
            MaybeCommon::Uncommon(uncommon) => {
                // Common bit: false
                bit_vec.push(false);
                // Bit-encoding
                bit_vec.extend(uncommon.to_cpe());
                bit_vec
            }
            MaybeCommon::Common(_, _) => {
                // Common bit: true
                bit_vec.push(true);
                panic!("Future extension.")
            }
        }
    }
}
