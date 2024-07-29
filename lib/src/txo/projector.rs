#![allow(dead_code)]

use crate::{
    musig2::keys_to_key_agg_ctx,
    serialize::{to_csv_script_encode, CSVFlag},
    taproot::{TapLeaf, TapRoot},
    well_known::operator,
};
use musig2::{
    secp256k1::{self, PublicKey, XOnlyPublicKey},
    KeyAggContext,
};

type Bytes = Vec<u8>;
type Key = XOnlyPublicKey;

#[derive(Clone, Copy)]
pub enum ProjectorTag {
    VTXOProjector,
    ConnectorProjector,
}

#[derive(Clone)]
pub struct Projector {
    msg_sender_keys: Vec<Key>,
    operator_key_well_known: Key,
    tag: ProjectorTag,
}

impl Projector {
    pub fn new(msg_sender_keys: Vec<Key>, tag: ProjectorTag) -> Projector {
        let operator_key_well_known = Key::from_slice(&operator::OPERATOR_KEY_WELL_KNOWN).unwrap();

        Projector {
            msg_sender_keys,
            operator_key_well_known,
            tag,
        }
    }

    pub fn operator_key(&self) -> Key {
        self.operator_key_well_known
    }

    pub fn msg_sender_keys(&self) -> Vec<Key> {
        self.msg_sender_keys.clone()
    }

    pub fn key_agg_ctx(&self) -> Result<KeyAggContext, secp256k1::Error> {
        let mut keys = self.msg_sender_keys();
        keys.push(self.operator_key());
        keys_to_key_agg_ctx(&keys).map_err(|_| secp256k1::Error::InvalidPublicKey)
    }

    pub fn taproot(&self) -> Result<TapRoot, secp256k1::Error> {
        //// Inner Key: (Self + Operator)
        let key_agg_ctx = self.key_agg_ctx()?;
        let inner_key: PublicKey = key_agg_ctx.aggregated_pubkey();

        //// Sweep Path: (Operator after 3 months)
        let mut sweep_path_script = Vec::<u8>::new();
        sweep_path_script.extend(to_csv_script_encode(CSVFlag::CSVThreeMonths)); // Relative Timelock
        sweep_path_script.push(0x20); // OP_PUSHDATA_32
        sweep_path_script.extend(self.operator_key().serialize()); // Operator Key 32-bytes
        sweep_path_script.push(0xac); // OP_CHECKSIG
        let sweep_path = TapLeaf::new(sweep_path_script);

        Ok(TapRoot::key_and_script_path_single(inner_key, sweep_path))
    }

    pub fn spk(&self) -> Result<Bytes, secp256k1::Error> {
        self.taproot()?.spk()
    }

    pub fn tag(&self) -> ProjectorTag {
        self.tag
    }
}
