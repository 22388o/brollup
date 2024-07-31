use super::transfer::Transfer;
use crate::serialize::cpe::CompactPayloadEncoding;
use bit_vec::BitVec;

pub trait EntryType {}

impl EntryType for Transfer {}

pub struct Entry<T: EntryType>(T);

impl<T: EntryType + CompactPayloadEncoding> CompactPayloadEncoding for Entry<T> {
    fn to_cpe(&self) -> BitVec {
        self.0.to_cpe()
    }
}
