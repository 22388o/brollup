#![allow(dead_code)]

use bit_vec::BitVec;

use crate::serialize::{common_index::common_index_from_u8, cpe::CompactPayloadEncoding};
use std::u8;

pub trait Commonable {}

impl Commonable for super::account::Account {}
impl Commonable for super::contract::Contract {}
impl Commonable for super::value::ShortVal {}
impl Commonable for super::value::LongVal {}

#[derive(Clone, Copy)]
pub enum MaybeCommon<T: Commonable + CompactPayloadEncoding> {
    Common(T, u8),
    Uncommon(T),
}

impl<T: Commonable + CompactPayloadEncoding> CompactPayloadEncoding for MaybeCommon<T> {
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