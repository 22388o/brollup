use super::nonce::deterministic_nonce;
use crate::hash::{tagged_hash, HashTag};
use secp::{MaybePoint, MaybeScalar, Point, Scalar};

pub enum SignFlag {
    BIP340Sign,
    EntrySign,
    ProtocolMessageSign,
    CustomMessageSign,
}

#[derive(Debug)]
pub enum SecpError {
    SignatureParseError,
    InvalidSignature,
    InvalidScalar,
    InvalidPoint,
}

pub trait SignEntry {
    fn sign(
        &self,
        secret_key: [u8; 32],
        prev_state_hash: [u8; 32],
    ) -> Result<[u8; 64], SecpError>;
}

pub trait IntoPoint {
    fn into_point(&self) -> Result<Point, SecpError>;
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

pub trait IntoScalar {
    fn into_scalar(&self) -> Result<Scalar, SecpError>;
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

pub fn schnorr_sign(
    secret_key_bytes: [u8; 32],
    message_bytes: [u8; 32],
    flag: SignFlag,
) -> Result<[u8; 64], SecpError> {
    // Check if the secret key (d) is a valid scalar.
    let mut secret_key = secret_key_bytes.into_scalar()?;

    // Public key (P) is = d * G.
    let public_key = secret_key.base_point_mul();

    // Negate the secret key (d) if it has odd public key.
    secret_key = secret_key.negate_if(public_key.parity());

    // Nonce generation is deterministic. Secret nonce (k) is = H(sk||m).
    let secret_nonce_bytes = deterministic_nonce(secret_key_bytes, message_bytes);

    // Check if the secret nonce (k) is a valid scalar.
    let mut secret_nonce = secret_nonce_bytes.into_scalar()?;

    // Public nonce (R) is = k * G.
    let public_nonce = secret_nonce.base_point_mul();

    // Negate the secret nonce (k) if it has odd public key.
    secret_nonce = secret_nonce.negate_if(public_nonce.parity());

    // Compute the challenge (e) bytes depending on the signing method.
    let challange_bytes: [u8; 32] = match flag {
        SignFlag::BIP340Sign => {
            // Follow BIP-340. Challenge e bytes is = H(R||P||m).
            let mut challenge_preimage = Vec::<u8>::with_capacity(96);
            challenge_preimage.extend(public_nonce.serialize_xonly());
            challenge_preimage.extend(public_key.serialize_xonly());
            challenge_preimage.extend(message_bytes);
            tagged_hash(challenge_preimage, HashTag::BIP0340Challenge)
        }
        SignFlag::EntrySign => {
            // Do not follow BIP-340. Challange (e) bytes is = H(m).
            tagged_hash(message_bytes, HashTag::EntryChallenge)
        }
        SignFlag::ProtocolMessageSign => {
            // Do not follow BIP-340. Challange (e) bytes is = H(m).
            tagged_hash(message_bytes, HashTag::ProtocolMessageChallenge)
        }
        SignFlag::CustomMessageSign => {
            // Do not follow BIP-340. Challange (e) bytes is = H(m).
            tagged_hash(message_bytes, HashTag::CustomMessageChallenge)
        }
    };

    // Challange (e) is = int(challange_e_bytes) mod n.
    let challange = challange_bytes.into_scalar()?;

    // Commitment (s) is = k + ed mod n.
    let commitment = match secret_nonce + challange * secret_key {
        MaybeScalar::Zero => return Err(SecpError::InvalidScalar),
        MaybeScalar::Valid(scalar) => scalar,
    };

    // Initialize the signature with a capacity of 64 bytes.
    let mut signature = Vec::<u8>::with_capacity(64);

    // Add public nonce (R) 32 bytes.
    signature.extend(public_nonce.serialize_xonly());

    // Add commitment (s) 32 bytes.
    signature.extend(commitment.serialize());

    // Signature is = bytes(R) || bytes((k + ed) mod n).
    signature
        .try_into()
        .map_err(|_| SecpError::SignatureParseError)
}

pub fn schnorr_verify(
    public_key_bytes: [u8; 32],
    message_bytes: [u8; 32],
    signature_bytes: [u8; 64],
    flag: SignFlag,
) -> Result<(), SecpError> {
    // Public key
    let public_key = public_key_bytes.into_point()?;

    // Parse public nonce bytes
    let public_nonce_bytes: [u8; 32] = signature_bytes[0..32]
        .try_into()
        .map_err(|_| SecpError::SignatureParseError)?;

    // Public nonce
    let public_nonce = public_nonce_bytes.into_point()?;

    // Compute the challenge e bytes based on whether it is a BIP-340 or a Brollup-native signing method.
    let challange_e_bytes: [u8; 32] = match flag {
        SignFlag::BIP340Sign => {
            // Follow BIP-340 for computing challenge e.
            // Challenge e is = H(R||P||m).
            let mut challenge_preimage = Vec::<u8>::with_capacity(96);
            challenge_preimage.extend(public_nonce.serialize_xonly());
            challenge_preimage.extend(public_key.serialize_xonly());
            challenge_preimage.extend(message_bytes);
            tagged_hash(challenge_preimage, HashTag::BIP0340Challenge)
        }
        SignFlag::EntrySign => {
            // Do not follow BIP-340 for computing challange e.
            // Challange e is = H(m) instead of H(R||P||m).
            tagged_hash(message_bytes, HashTag::EntryChallenge)
        }
        SignFlag::ProtocolMessageSign => {
            // Do not follow BIP-340 for computing challange e.
            // Challange e is = H(m) instead of H(R||P||m).
            tagged_hash(message_bytes, HashTag::ProtocolMessageChallenge)
        }
        SignFlag::CustomMessageSign => {
            // Do not follow BIP-340 for computing challange e.
            // Challange e is = H(m) instead of H(R||P||m).
            tagged_hash(message_bytes, HashTag::CustomMessageChallenge)
        }
    };

    // Challange e is = int(challange_e_bytes) mod n.
    let challange_e = challange_e_bytes.into_scalar()?;

    // Parse s commitment bytes
    let s_commitment_bytes: [u8; 32] = signature_bytes[32..64]
        .try_into()
        .map_err(|_| SecpError::SignatureParseError)?;

    // S commitment
    let s_commitment = s_commitment_bytes.into_scalar()?;

    let equation = match public_nonce + challange_e * public_key {
        MaybePoint::Infinity => {
            return Err(SecpError::InvalidPoint);
        }
        MaybePoint::Valid(point) => point,
    };

    match s_commitment.base_point_mul() == equation {
        false => return Err(SecpError::InvalidSignature),
        true => return Ok(()),
    }
}
