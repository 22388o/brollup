use super::transfer::Transfer;
use crate::{
    hash::{tagged_hash, HashTag},
    serialization::{cpe::CompactPayloadEncoding, serialize::Serialize, sighash::Sighash},
    signature::sign::{random_scalar, Sign, SignError},
};
use bit_vec::BitVec;
use secp::MaybeScalar;

pub enum Entry {
    Transfer(Transfer),
}

impl CompactPayloadEncoding for Entry {
    fn to_cpe(&self) -> BitVec {
        match self {
            Entry::Transfer(transfer) => transfer.to_cpe(),
        }
    }
}

impl Sighash for Entry {
    fn sighash(&self, prev_state_hash: [u8; 32]) -> [u8; 32] {
        let mut sighash_preimage = Vec::<u8>::new();

        sighash_preimage.extend(prev_state_hash);

        let (serialized_entry, sighash_flag) = match self {
            Entry::Transfer(transfer) => (transfer.serialize(), HashTag::SighashTransfer),
        };

        sighash_preimage.extend(serialized_entry);

        tagged_hash(sighash_preimage, sighash_flag)
    }
}

impl Sign for Entry {
    fn sign(&self, secret_key: [u8; 32], prev_state_hash: [u8; 32]) -> Result<[u8; 64], SignError> {
        let secret_key_scalar = MaybeScalar::reduce_from(&secret_key);

        let nonce = random_scalar();
        let nonce_point = nonce.base_point_mul();

        let e = self.sighash(prev_state_hash);
        let e_scalar = MaybeScalar::reduce_from(&e);

        let s = nonce + secret_key_scalar * e_scalar;

        let mut signature = Vec::<u8>::new();

        signature.extend(nonce_point.serialize_xonly());
        signature.extend(s.serialize());

        signature
            .try_into()
            .map_err(|_| SignError::InvalidPrivateKey)
    }
}
