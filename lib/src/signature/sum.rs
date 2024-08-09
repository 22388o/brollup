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
