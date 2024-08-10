use super::into::IntoPoint;
use super::into::IntoScalar;
use super::schnorr::SecpError;
use secp::MaybePoint;
use secp::MaybeScalar;
use secp::Point;
use secp::Scalar;

pub fn sum_scalars(scalars: Vec<Scalar>) -> Result<Scalar, SecpError> {
    if scalars.len() == 0 {
        return Err(SecpError::InvalidScalar);
    }

    let mut sum = scalars[0];

    for scalar in scalars.iter().skip(1) {
        sum = match sum + *scalar {
            MaybeScalar::Zero => return Err(SecpError::InvalidScalar),
            MaybeScalar::Valid(scalar) => scalar,
        };
    }

    Ok(sum)
}

pub fn sum_scalars_bytes(scalars_bytes: Vec<[u8; 32]>) -> Result<[u8; 32], SecpError> {
    let mut scalars = Vec::<Scalar>::with_capacity(scalars_bytes.len());

    for scalar_bytes in scalars_bytes {
        let scalar = scalar_bytes.into_scalar()?;
        scalars.push(scalar);
    }

    let sum = sum_scalars(scalars)?;

    Ok(sum.serialize())
}

pub fn sum_points(points: Vec<Point>) -> Result<Point, SecpError> {
    if points.len() == 0 {
        return Err(SecpError::InvalidPoint);
    }

    let mut sum = points[0];

    for point in points.iter().skip(1) {
        sum = match sum + *point {
            MaybePoint::Infinity => return Err(SecpError::InvalidPoint),
            MaybePoint::Valid(point) => point,
        };
    }

    Ok(sum)
}

pub fn sum_points_bytes(points_bytes: Vec<[u8; 32]>) -> Result<[u8; 33], SecpError> {
    let mut points = Vec::<Point>::with_capacity(points_bytes.len());

    for point_bytes in points_bytes {
        let point = point_bytes.into_point()?;
        points.push(point);
    }

    let sum = sum_points(points)?;

    Ok(sum.serialize())
}

pub fn sum_public_keys(public_keys: Vec<[u8; 32]>) -> Result<[u8; 33], SecpError> {
    sum_points_bytes(public_keys)
}

pub fn sum_public_nonces(public_nonces: Vec<[u8; 32]>) -> Result<[u8; 33], SecpError> {
    sum_points_bytes(public_nonces)
}

pub fn sum_commitments(commitments: Vec<[u8; 32]>) -> Result<[u8; 32], SecpError> {
    sum_scalars_bytes(commitments)
}

pub fn sum_challanges(challanges: Vec<[u8; 32]>) -> Result<[u8; 32], SecpError> {
    sum_scalars_bytes(challanges)
}

pub fn sum_signatures(signatures: Vec<[u8; 64]>) -> Result<[u8; 65], SecpError> {
    let mut public_nonces = Vec::<[u8; 32]>::with_capacity(signatures.len());
    let mut commitments = Vec::<[u8; 32]>::with_capacity(signatures.len());

    for signature in signatures {
        let public_nonce: [u8; 32] = (&signature[0..32])
            .try_into()
            .map_err(|_| SecpError::InvalidPoint)?;
        public_nonces.push(public_nonce);

        let commitment: [u8; 32] = (&signature[0..32])
            .try_into()
            .map_err(|_| SecpError::InvalidScalar)?;
        commitments.push(commitment);
    }

    let public_nonces_sum = sum_public_nonces(public_nonces)?;
    let commitments_sum = sum_commitments(commitments)?;

    let mut signature = [0u8; 65];

    signature[..33].copy_from_slice(&public_nonces_sum);
    signature[33..].copy_from_slice(&commitments_sum);

    Ok(signature)
}