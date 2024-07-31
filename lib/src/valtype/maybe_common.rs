#![allow(dead_code)]

use std::u8;

use crate::serialize::cpe::CompactPayloadEncoding;

use super::ValType;

#[derive(Clone, Copy)]
pub enum MaybeCommon<T: ValType + CompactPayloadEncoding> {
    Common(T, u8),
    Uncommon(T),
}
