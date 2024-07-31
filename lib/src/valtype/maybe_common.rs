#![allow(dead_code)]

use bit_vec::BitVec;

use super::ValType;
use crate::serialize::{common_index::common_index_from_u8, cpe::CompactPayloadEncoding};
use std::u8;

#[derive(Clone, Copy)]
pub enum MaybeCommon<T: ValType + CompactPayloadEncoding> {
    Common(T, u8),
    Uncommon(T),
}

impl<T: ValType + CompactPayloadEncoding> CompactPayloadEncoding for MaybeCommon<T> {
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
            MaybeCommon::Common(_, common_index) => {
                // Common bit: true
                bit_vec.push(true);
                // 3-bit common index encoding
                bit_vec.extend(common_index_from_u8(common_index));
                bit_vec
            }
        }
    }
}