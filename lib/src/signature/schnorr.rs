use crate::hash::{tagged_hash, HashTag};
use secp::{MaybePoint, MaybeScalar, Point};

use super::into::{IntoArray, IntoPoint, IntoScalar};

pub enum SignFlag {
    BIP340Sign,
    EntrySign,
    ProtocolMessageSign,
    CustomMessageSign,
}

#[derive(Debug)]
pub enum SecpError {
    InvalidSignature,
    InvalidScalar,
    InvalidPoint,
    SignatureParseError,
    MessageParseError,
    SecretKeyParseError,
    PublicKeyParseError,
}

pub trait SignEntry {
    fn sign(&self, secret_key: [u8; 32], prev_state_hash: [u8; 32]) -> Result<[u8; 64], SecpError>;
}

fn compute_challenge_bytes(
    public_nonce: Point,
    public_key: Point,
    message_bytes: [u8; 32],
    flag: SignFlag,
) -> [u8; 32] {
    match flag {
        SignFlag::BIP340Sign => {
            // Follow BIP-340. Challenge e bytes is = H(R||P||m).
            let mut challenge_preimage = Vec::<u8>::with_capacity(96);
            challenge_preimage.extend(public_nonce.serialize_xonly());
            challenge_preimage.extend(public_key.serialize_xonly());
            challenge_preimage.extend(message_bytes);
            return tagged_hash(challenge_preimage, HashTag::BIP0340Challenge);
        }
        SignFlag::EntrySign => {
            // Do not follow BIP-340. Challange (e) bytes is = H(m).
            return tagged_hash(message_bytes, HashTag::EntryChallenge);
        }
        SignFlag::ProtocolMessageSign => {
            // Do not follow BIP-340. Challange (e) bytes is = H(m).
            return tagged_hash(message_bytes, HashTag::ProtocolMessageChallenge);
        }
        SignFlag::CustomMessageSign => {
            // Do not follow BIP-340. Challange (e) bytes is = H(m).
            return tagged_hash(message_bytes, HashTag::CustomMessageChallenge);
        }
    };
}

fn deterministic_nonce(secret_key: [u8; 32], message: [u8; 32]) -> [u8; 32] {
    let mut preimage = Vec::<u8>::new();

    preimage.extend(secret_key);
    preimage.extend(message);

    tagged_hash(preimage, HashTag::DeterministicNonce)
}

pub fn schnorr_sign(
    secret_key_bytes: [u8; 32],
    message_bytes: [u8; 32],
    flag: SignFlag,
) -> Result<[u8; 64], SecpError> {
    // Check if the secret key (d) is a valid scalar.
    let mut secret_key = secret_key_bytes.into_scalar()?;

    // Public key (P) is = dG.
    let public_key = secret_key.base_point_mul();

    // Negate the secret key (d) if it has odd public key.
    secret_key = secret_key.negate_if(public_key.parity());

    // Nonce generation is deterministic. Secret nonce (k) is = H(sk||m).
    let secret_nonce_bytes = deterministic_nonce(secret_key_bytes, message_bytes);

    // Check if the secret nonce (k) is a valid scalar.
    let mut secret_nonce = secret_nonce_bytes.into_scalar()?;

    // Public nonce (R) is = kG.
    let public_nonce = secret_nonce.base_point_mul();

    // Negate the secret nonce (k) if it has odd public key.
    secret_nonce = secret_nonce.negate_if(public_nonce.parity());

    // Compute the challenge (e) bytes depending on the signing method.
    let challange_bytes: [u8; 32] =
        compute_challenge_bytes(public_nonce, public_key, message_bytes, flag);

    // Challange (e) is = int(challange_bytes) mod n.
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
    signature.into_signature_array()
}

pub fn schnorr_verify(
    public_key_bytes: [u8; 32],
    message_bytes: [u8; 32],
    signature_bytes: [u8; 64],
    flag: SignFlag,
) -> Result<(), SecpError> {
    // Check if the public key (P) is a valid point.
    let public_key = public_key_bytes.into_point()?;

    // Parse public nonce (R) bytes.
    let public_nonce_bytes: [u8; 32] = (&signature_bytes[0..32])
        .try_into()
        .map_err(|_| SecpError::SignatureParseError)?;

    // Check if the public nonce (R) is a valid point.
    let public_nonce = public_nonce_bytes.into_point()?;

    // Compute the challenge (e) bytes depending on the signing method.
    let challange_bytes: [u8; 32] =
        compute_challenge_bytes(public_nonce, public_key, message_bytes, flag);

    // Challange (e) is = int(challange_bytes) mod n.
    let challange = challange_bytes.into_scalar()?;

    // Parse commitment (s) bytes.
    let commitment_bytes: [u8; 32] = (&signature_bytes[32..64])
        .try_into()
        .map_err(|_| SecpError::SignatureParseError)?;

    // Check if commitment (s) is a valid scalar.
    let commitment = commitment_bytes.into_scalar()?;

    // Check if the equation (R + eP) is a valid point.
    let equation = match public_nonce + challange * public_key {
        MaybePoint::Infinity => {
            return Err(SecpError::InvalidPoint);
        }
        MaybePoint::Valid(point) => point,
    };

    // Check if the equation (R + eP) point equals to sG point.
    match commitment.base_point_mul() == equation {
        false => return Err(SecpError::InvalidSignature),
        true => return Ok(()),
    }
}
