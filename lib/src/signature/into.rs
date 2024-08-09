use super::schnorr::SecpError;
use secp::{MaybePoint, MaybeScalar, Point, Scalar};

pub trait IntoPoint {
    fn into_point(&self) -> Result<Point, SecpError>;
}

pub trait IntoScalar {
    fn into_scalar(&self) -> Result<Scalar, SecpError>;
}

pub trait IntoByteArray {
    fn into_secret_key_byte_array(&self) -> Result<[u8; 32], SecpError>;
    fn into_message_byte_array(&self) -> Result<[u8; 32], SecpError>;
    fn into_public_key_byte_array(&self) -> Result<[u8; 32], SecpError>;
    fn into_signature_byte_array(&self) -> Result<[u8; 64], SecpError>;
}

impl IntoByteArray for Vec<u8> {
    fn into_secret_key_byte_array(&self) -> Result<[u8; 32], SecpError> {
        let mut vec = Vec::<u8>::with_capacity(32);
        vec.extend(self);
        let bytes_32: [u8; 32] = vec.try_into().map_err(|_| SecpError::SecretKeyParseError)?;

        Ok(bytes_32)
    }

    fn into_message_byte_array(&self) -> Result<[u8; 32], SecpError> {
        let mut vec = Vec::<u8>::with_capacity(32);
        vec.extend(self);
        let bytes_32: [u8; 32] = vec.try_into().map_err(|_| SecpError::SecretKeyParseError)?;

        Ok(bytes_32)
    }

    fn into_public_key_byte_array(&self) -> Result<[u8; 32], SecpError> {
        let mut vec = Vec::<u8>::with_capacity(32);
        vec.extend(self);
        let bytes_32: [u8; 32] = vec.try_into().map_err(|_| SecpError::SecretKeyParseError)?;

        Ok(bytes_32)
    }

    fn into_signature_byte_array(&self) -> Result<[u8; 64], SecpError> {
        let mut vec = Vec::<u8>::with_capacity(64);
        vec.extend(self);
        let bytes_64: [u8; 64] = vec.try_into().map_err(|_| SecpError::SecretKeyParseError)?;

        Ok(bytes_64)
    }
}

impl IntoPoint for [u8; 32] {
    fn into_point(&self) -> Result<Point, SecpError> {
        let mut point_bytes = Vec::with_capacity(33);
        point_bytes.push(0x02);
        point_bytes.extend(self);

        let point = match MaybePoint::from_slice(&point_bytes) {
            Ok(maybe_point) => match maybe_point {
                MaybePoint::Infinity => {
                    return Err(SecpError::InvalidPoint);
                }
                MaybePoint::Valid(point) => point,
            },
            Err(_) => return Err(SecpError::InvalidPoint),
        };

        Ok(point)
    }
}

impl IntoScalar for [u8; 32] {
    fn into_scalar(&self) -> Result<Scalar, SecpError> {
        let mut scalar_bytes = Vec::with_capacity(32);
        scalar_bytes.extend(self);

        let scalar = match MaybeScalar::from_slice(&scalar_bytes) {
            Ok(maybe_scalar) => match maybe_scalar {
                MaybeScalar::Zero => {
                    return Err(SecpError::InvalidScalar);
                }
                MaybeScalar::Valid(point) => point,
            },
            Err(_) => return Err(SecpError::InvalidScalar),
        };

        Ok(scalar)
    }
}
