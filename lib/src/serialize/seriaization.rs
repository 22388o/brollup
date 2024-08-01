type Bytes = Vec<u8>;

pub enum SerializationError {
    KeyParseError
}

pub trait Serialization {
    fn serialize(&self) -> Bytes;
    fn from_bytes(bytes: Bytes) -> Result<Self, SerializationError> where Self: Sized;
}
