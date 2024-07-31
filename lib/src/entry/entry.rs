use super::transfer::Transfer;
use crate::serialize::cpe::CompactPayloadEncoding;
use bit_vec::BitVec;

pub enum Entry {
    Transfer(Transfer),
}

impl CompactPayloadEncoding for Entry {
    fn to_cpe(&self) -> BitVec {
        match self {
            Entry::Transfer(transfer) => transfer.to_cpe(),
        }
    }
}
