use crate::entry::entry::Entry;

type Bytes = Vec<u8>;

pub trait Serialization {
    fn serialize(&self) -> Bytes;
    fn from_bytes(bytes: Bytes) -> Entry;
}
