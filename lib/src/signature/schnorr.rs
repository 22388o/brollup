use super::nonce::deterministic_nonce;
use crate::hash::{tagged_hash, HashTag};
use secp::{MaybePoint, MaybeScalar};

pub enum SignFlag {
    BIP340Sign,
    EntrySign,
    ProtocolMessageSign,
    CustomMessageSign,
}

#[derive(Debug)]
pub enum SignError {
    SignatureParseError,
    InvalidScalar,
    InvalidPoint,
    InvalidSecretKey,
}
pub trait Sign {
    fn sign(&self, secret_key: [u8; 32], prev_state_hash: [u8; 32]) -> Result<[u8; 64], SignError>;
}

pub fn schnorr_sign(
    secret: [u8; 32],
    message: [u8; 32],
    flag: SignFlag,
) -> Result<[u8; 64], SignError> {
    // Check if the secret key is a valid scalar.
    let secret_key = match MaybeScalar::reduce_from(&secret) {
        MaybeScalar::Zero => return Err(SignError::InvalidScalar),
        MaybeScalar::Valid(scalar) => scalar,
    };

    let public_key = secret_key.base_point_mul();

    // In this scope we assume supplied 'secret' parameter has_even_y(P).
    // We are not interested in negating the secret key otherwise: we simply return an InvalidSecretKey error.
    if bool::from(public_key.parity()) == true {
        return Err(SignError::InvalidSecretKey);
    }

    // Nonce generation is deterministic.
    // Secret nonce is = H(sk||m).
    let secret_nonce_bytes = deterministic_nonce(secret, message);
    let mut secret_nonce = match MaybeScalar::reduce_from(&secret_nonce_bytes) {
        MaybeScalar::Zero => return Err(SignError::InvalidScalar),
        MaybeScalar::Valid(scalar) => scalar,
    };
    let mut public_nonce = secret_nonce.base_point_mul();

    // Negate the nonce if it has_odd_y(R).
    secret_nonce = secret_nonce.negate_if(public_nonce.parity());
    public_nonce = public_nonce.negate_if(public_nonce.parity());

    // Compute the challenge e bytes based on whether it is a BIP-340 or a Brollup-native signing method.
    let challange_e_bytes: [u8; 32] = match flag {
        SignFlag::BIP340Sign => {
            // Follow BIP-340 for computing challenge e.
            // Challenge e is = H(R||P||m).
            let mut challenge_preimage = Vec::<u8>::with_capacity(96);
            challenge_preimage.extend(public_nonce.serialize_xonly());
            challenge_preimage.extend(public_key.serialize_xonly());
            challenge_preimage.extend(message);
            tagged_hash(challenge_preimage, HashTag::BIP0340Challenge)
        }
        SignFlag::EntrySign => {
            // Do not follow BIP-340 for computing challange e.
            // Challange e is = H(m) instead of H(R||P||m).
            tagged_hash(message, HashTag::EntryChallenge)
        }
        SignFlag::ProtocolMessageSign => {
            // Do not follow BIP-340 for computing challange e.
            // Challange e is = H(m) instead of H(R||P||m).
            tagged_hash(message, HashTag::ProtocolMessageChallenge)
        }
        SignFlag::CustomMessageSign => {
            // Do not follow BIP-340 for computing challange e.
            // Challange e is = H(m) instead of H(R||P||m).
            tagged_hash(message, HashTag::CustomMessageChallenge)
        }
    };

    // Challange e is = int(challange_e_bytes) mod n.
    let challange_e = match MaybeScalar::reduce_from(&challange_e_bytes) {
        MaybeScalar::Zero => return Err(SignError::InvalidScalar),
        MaybeScalar::Valid(scalar) => scalar,
    };

    // s commitment is = k + ed mod n.
    let s_commitment = match secret_nonce + challange_e * secret_key {
        MaybeScalar::Zero => return Err(SignError::InvalidScalar),
        MaybeScalar::Valid(scalar) => scalar,
    };

    // Initialize the signature vector with a 64 bytes capacity
    let mut signature = Vec::<u8>::with_capacity(64);

    // Add public nonce: R (32 bytes)
    signature.extend(public_nonce.serialize_xonly());

    // Add s commitment: s (32 bytes)
    signature.extend(s_commitment.serialize());

    // Signature is = bytes(R) || bytes((k + ed) mod n).
    signature
        .try_into()
        .map_err(|_| SignError::SignatureParseError)
}

pub fn schnorr_verify(
    public_key: [u8; 32],
    message: [u8; 32],
    signature: [u8; 64],
    flag: SignFlag,
) -> bool {
    // Parse public key bytes
    let mut public_key_bytes = vec![0x02];
    public_key_bytes.extend(&public_key);

    // Public key
    let public_key = match MaybePoint::from_slice(&public_key_bytes) {
        Ok(maybe_point) => match maybe_point {
            MaybePoint::Infinity => {
                return false;
            }
            MaybePoint::Valid(point) => point,
        },
        Err(_) => return false,
    };

    // Parse public nonce bytes
    let mut public_nonce_bytes = vec![0x02];
    public_nonce_bytes.extend(&signature[0..32]);

    // Public nonce
    let public_nonce = match MaybePoint::from_slice(&public_nonce_bytes) {
        Ok(maybe_point) => match maybe_point {
            MaybePoint::Infinity => {
                return false;
            }
            MaybePoint::Valid(point) => point,
        },
        Err(_) => return false,
    };

    // Compute the challenge e bytes based on whether it is a BIP-340 or a Brollup-native signing method.
    let challange_e_bytes: [u8; 32] = match flag {
        SignFlag::BIP340Sign => {
            // Follow BIP-340 for computing challenge e.
            // Challenge e is = H(R||P||m).
            let mut challenge_preimage = Vec::<u8>::with_capacity(96);
            challenge_preimage.extend(public_nonce.serialize_xonly());
            challenge_preimage.extend(public_key.serialize_xonly());
            challenge_preimage.extend(message);
            tagged_hash(challenge_preimage, HashTag::BIP0340Challenge)
        }
        SignFlag::EntrySign => {
            // Do not follow BIP-340 for computing challange e.
            // Challange e is = H(m) instead of H(R||P||m).
            tagged_hash(message, HashTag::EntryChallenge)
        }
        SignFlag::ProtocolMessageSign => {
            // Do not follow BIP-340 for computing challange e.
            // Challange e is = H(m) instead of H(R||P||m).
            tagged_hash(message, HashTag::ProtocolMessageChallenge)
        }
        SignFlag::CustomMessageSign => {
            // Do not follow BIP-340 for computing challange e.
            // Challange e is = H(m) instead of H(R||P||m).
            tagged_hash(message, HashTag::CustomMessageChallenge)
        }
    };

    // Challange e is = int(challange_e_bytes) mod n.
    let challange_e = match MaybeScalar::reduce_from(&challange_e_bytes) {
        MaybeScalar::Zero => return false,
        MaybeScalar::Valid(scalar) => scalar,
    };

    // Parse s commitment bytes
    let s_commitment_bytes = &signature[32..64];

    // S commitment
    let s_commitment = match MaybeScalar::from_slice(s_commitment_bytes) {
        Result::Ok(maybe_scalar) => match maybe_scalar {
            MaybeScalar::Valid(scalar) => scalar,
            MaybeScalar::Zero => return false,
        },
        Result::Err(_) => return false,
    };

    let equation = match public_nonce + challange_e * public_key {
        MaybePoint::Infinity => {
            return false;
        }
        MaybePoint::Valid(point) => point,
    };

    s_commitment.base_point_mul() == equation
}
