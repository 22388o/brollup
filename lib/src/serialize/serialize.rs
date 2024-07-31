use crate::entry::EntryType;

type Bytes = Vec<u8>;

pub trait Serialization {
    fn serialize(&self) -> Bytes;
    fn from_bytes(bytes: Bytes) -> impl EntryType;
}
