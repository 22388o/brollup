pub enum ParseError {
    ParseError32,
    ParseError64,
}

pub trait IntoByteArray {
    fn into_byte_array_32(&self) -> Result<[u8; 32], ParseError>;
    fn into_byte_array_64(&self) -> Result<[u8; 64], ParseError>;
}

impl IntoByteArray for Vec<u8> {
    fn into_byte_array_32(&self) -> Result<[u8; 32], ParseError> {
        let mut vec = Vec::<u8>::with_capacity(32);
        vec.extend(self);
        let bytes_32: [u8; 32] = vec.try_into().map_err(|_| ParseError::ParseError32)?;

        Ok(bytes_32)
    }

    fn into_byte_array_64(&self) -> Result<[u8; 64], ParseError> {
        let mut vec = Vec::<u8>::with_capacity(64);
        vec.extend(self);
        let bytes_64: [u8; 64] = vec.try_into().map_err(|_| ParseError::ParseError64)?;

        Ok(bytes_64)
    }
}
