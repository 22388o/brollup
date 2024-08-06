#![allow(dead_code)]

use bit_vec::BitVec;

use crate::serialize::{cpe::CommonIndex, cpe::CompactPayloadEncoding};
use std::u8;

pub trait MaybeCommonValtype {}

impl MaybeCommonValtype for super::account::Account {}
impl MaybeCommonValtype for super::contract::Contract {}
impl MaybeCommonValtype for super::value::ShortVal {}
impl MaybeCommonValtype for super::value::LongVal {}

#[derive(Clone, Copy)]
pub enum MaybeCommon<T: MaybeCommonValtype + CompactPayloadEncoding> {
    Common(T, u8),
    Uncommon(T),
}

impl<T: MaybeCommonValtype + CompactPayloadEncoding> CompactPayloadEncoding for MaybeCommon<T> {
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
                bit_vec.extend(BitVec::from_u8_common_index(common_index));
                bit_vec
            }
        }
    }
}