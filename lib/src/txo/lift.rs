#![allow(dead_code)]

use crate::{
    musig2::keys_to_key_agg_ctx,
    serialize::csv::{to_csv_script_encode, CSVFlag},
    taproot::{TapLeaf, TapRoot},
    well_known::operator,
};
use musig2::{
    secp256k1::{self, PublicKey, XOnlyPublicKey},
    KeyAggContext,
};

type Bytes = Vec<u8>;
type Key = XOnlyPublicKey;

pub struct Lift {
    self_key: Key,
    operator_key_well_known: Key,
}

impl Lift {
    pub fn new(self_key: Key) -> Lift {
        let operator_key_well_known = Key::from_slice(&operator::OPERATOR_KEY_WELL_KNOWN).unwrap();
        Lift {
            self_key,
            operator_key_well_known,
        }
    }

    pub fn new_with_operator(self_key: Key, operator_key_well_known: Key) -> Lift {
        Lift {
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

        //// Exit Path: (Self after 3 months)
        let mut exit_path_script = Vec::<u8>::new();
        exit_path_script.extend(to_csv_script_encode(CSVFlag::CSVYear)); // Relative Timelock
        exit_path_script.push(0x20); // OP_PUSHDATA_32
        exit_path_script.extend(self.self_key().serialize()); // Self Key 32-bytes
        exit_path_script.push(0xac); // OP_CHECKSIG
        let exit_path = TapLeaf::new(exit_path_script);

        Ok(TapRoot::key_and_script_path_single(inner_key, exit_path))
    }

    pub fn spk(&self) -> Result<Bytes, secp256k1::Error> {
        self.taproot()?.spk()
    }
}
