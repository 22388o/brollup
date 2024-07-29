#![allow(dead_code)]

use crate::{musig2::keys_to_key_agg_ctx, taproot::TapRoot, well_known::operator};
use musig2::{secp256k1::{self, PublicKey, XOnlyPublicKey}, KeyAggContext};

type Bytes = Vec<u8>;
type Key = XOnlyPublicKey;

pub struct Connector {
    self_key: Key,
    operator_key_well_known: Key,
}

impl Connector {
    pub fn new(self_key: Key) -> Connector {
        let operator_key_well_known = Key::from_slice(&operator::OPERATOR_KEY_WELL_KNOWN).unwrap();
        Connector {
            self_key,
            operator_key_well_known,
        }
    }

    pub fn new_with_operator(self_key: Key, operator_key_well_known: Key) -> Connector {
        Connector {
            self_key,
            operator_key_well_known,
        }
    }

    pub fn self_key(&self) -> Key {
        self.self_key
    }

    pub fn operator_key(&self) -> Key {
        self.operator_key_well_known
    }

    pub fn key_agg_ctx(&self) -> Result<KeyAggContext, secp256k1::Error> {
        let keys = vec![self.self_key(), self.operator_key()];
        keys_to_key_agg_ctx(&keys).map_err(|_| secp256k1::Error::InvalidPublicKey)
    }

    pub fn taproot(&self) -> Result<TapRoot, secp256k1::Error> {
        //// Inner Key: (Self + Operator)
        let key_agg_ctx = self.key_agg_ctx()?;
        let inner_key: PublicKey = key_agg_ctx.aggregated_pubkey();

        Ok(TapRoot::key_path_only(inner_key))
    }

    pub fn spk(&self) -> Result<Bytes, secp256k1::Error> {
        self.taproot()?.spk()
    }
}
