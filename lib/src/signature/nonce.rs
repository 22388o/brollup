use crate::hash::{tagged_hash, HashTag};

pub fn deterministic_nonce(secret_key: [u8; 32], message: [u8; 32]) -> [u8; 32] {
    let mut preimage = Vec::<u8>::new();

    preimage.extend(secret_key);
    preimage.extend(message);
    
    tagged_hash(preimage, HashTag::DeterministicNonce)
}