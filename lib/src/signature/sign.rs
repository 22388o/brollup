use secp::MaybeScalar;

use crate::hash::{tagged_hash, HashTag};

use super::nonce::deterministic_nonce;

pub enum SignFlag {
    BIP340Sign,
    EntrySign,
    ProtocolMessageSign,
    CustomMessageSign,
}

pub enum SignError {
    SignatureParseError,
    InvalidPrivateKey,
    InvalidScalar,
    InvalidPoint,
}
pub trait Sign {
    fn sign(&self, secret_key: [u8; 32], prev_state_hash: [u8; 32]) -> Result<[u8; 64], SignError>;
}

pub fn schnorr_sign(
    secret: [u8; 32],
    message: [u8; 32],
    flag: SignFlag,
) -> Result<[u8; 64], SignError> {
    // Check if the message is a valid scalar.
    if let MaybeScalar::Zero = MaybeScalar::reduce_from(&message) {
        return Err(SignError::InvalidScalar);
    }

    // Check if the secret key is a valid scalar.
    let secret_key = match MaybeScalar::reduce_from(&secret) {
        MaybeScalar::Zero => return Err(SignError::InvalidScalar),
        MaybeScalar::Valid(scalar) => scalar,
    };

    // Nonce generation is deterministic.
    // Secret nonce is = H(sk||m).
    let private_nonce_bytes = deterministic_nonce(secret, message);
    let private_nonce = match MaybeScalar::reduce_from(&private_nonce_bytes) {
        MaybeScalar::Zero => return Err(SignError::InvalidScalar),
        MaybeScalar::Valid(scalar) => scalar,
    };
    let public_nonce = private_nonce.base_point_mul();

    // Compute the challenge e bytes based on whether it is a BIP-340 or a Brollup-native signing method.
    let challange_e_bytes: [u8; 32] = match flag {
        SignFlag::BIP340Sign => {
            let public_key = secret_key.base_point_mul();
            // Follow BIP-340 for computing challenge e.
            // Challenge e is = H(R||P||m).
            let mut challenge_preimage = Vec::<u8>::with_capacity(96);
            challenge_preimage.extend(public_nonce.serialize_xonly());
            challenge_preimage.extend(public_key.serialize_xonly());
            challenge_preimage.extend(message);
            tagged_hash(challenge_preimage.to_vec(), HashTag::BIP0340Challenge)
        }
        SignFlag::EntrySign => {
            // Do not follow BIP-340 for computing challange e.
            // Challange e is = H(m) instead of H(R||P||m).
            tagged_hash(message.to_vec(), HashTag::EntryChallenge)
        }
        SignFlag::ProtocolMessageSign => {
            // Do not follow BIP-340 for computing challange e.
            // Challange e is = H(m) instead of H(R||P||m).
            tagged_hash(message.to_vec(), HashTag::ProtocolMessageChallenge)
        }
        SignFlag::CustomMessageSign => {
            // Do not follow BIP-340 for computing challange e.
            // Challange e is = H(m) instead of H(R||P||m).
            tagged_hash(message.to_vec(), HashTag::CustomMessageChallenge)
        }
    };

    // challange e is = int(challange_e_bytes) mod n.
    let challange_e = match MaybeScalar::reduce_from(&challange_e_bytes) {
        MaybeScalar::Zero => return Err(SignError::InvalidScalar),
        MaybeScalar::Valid(scalar) => scalar,
    };

    // s commitment is = k + ed mod n.
    let s_commitment = match private_nonce + challange_e * secret_key {
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
