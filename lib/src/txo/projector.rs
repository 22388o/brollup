#![allow(dead_code)]

use crate::{
    musig2::keys_to_key_agg_ctx, serialize::{to_csv_script_encode, CSVFlag}, taproot::{TapLeaf, TapRoot}, well_known::operator
};
use musig2::{
    secp256k1::{self, XOnlyPublicKey},
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
    msg_senders_key_agg_ctx: KeyAggContext,
    tag: ProjectorTag,
}

impl Projector {
    pub fn new(msg_sender_keys: Vec<Key>, tag: ProjectorTag) -> Projector {
        
        let operator_key_well_known = Key::from_slice(&operator::OPERATOR_KEY_WELL_KNOWN).unwrap();

        // consider removing unwrap here
        let msg_senders_key_agg_ctx = keys_to_key_agg_ctx(&msg_sender_keys).unwrap();

        Projector {
            msg_sender_keys,
            operator_key_well_known,
            msg_senders_key_agg_ctx,
            tag,
        }
    }

    pub fn msg_senders_aggregate_key(&self) -> Key {
        self.msg_senders_key_agg_ctx.aggregated_pubkey()
    }

    pub fn operator_key(&self) -> Key {
        self.operator_key_well_known
    }

    pub fn taproot(&self) -> TapRoot {
        // Reveal path
        let mut reveal_path = Vec::<u8>::new();

        // Push aggregate key
        reveal_path.push(0x20);
        reveal_path.extend(self.msg_senders_aggregate_key().serialize());

        // OP_CHECKSIGVERIFY
        reveal_path.push(0xad);

        // Push operator key
        reveal_path.push(0x20);
        reveal_path.extend(self.operator_key().serialize());

        // OP_CHECKSIG
        reveal_path.push(0xac);

        //// Reclaim path
        let mut reclaim_path = Vec::<u8>::new();

        // Relative timelock to sweep funds back to the operator
        reclaim_path.extend(to_csv_script_encode(CSVFlag::CSVThreeMonths));

        // Push operator key
        reclaim_path.push(0x20);
        reclaim_path.extend(self.operator_key().serialize());

        // OP_CHECKSIG
        reclaim_path.push(0xac);

        let leaves = vec![TapLeaf::new(reveal_path), TapLeaf::new(reclaim_path)];
        TapRoot::script_path_only_multi(leaves)
    }

    pub fn spk(&self) -> Result<Bytes, secp256k1::Error> {
        self.taproot().spk()
    }

    pub fn tag(&self) -> ProjectorTag {
        self.tag
    }
}
