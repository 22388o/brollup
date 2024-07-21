#![allow(dead_code)]

use crate::{
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
    operator_key: Key,
    msg_senders: Vec<Key>,
    msg_senders_key_agg_ctx: KeyAggContext,
    tag: ProjectorTag,
}

impl Projector {
    pub fn new(msg_senders: Vec<Key>, tag: ProjectorTag) -> Projector {
        let operator_key = Key::from_slice(&operator::OPERATOR_KEY_WELL_KNOWN).unwrap();

        // Lift msg.sender from their XOnly keys
        let mut msg_senders_lifted = Vec::<PublicKey>::new();

        for sender in &msg_senders {
            msg_senders_lifted.push(sender.public_key(secp256k1::Parity::Even));
        }

        // Sort the keys
        msg_senders_lifted.sort();

        let msg_senders_iter = msg_senders_lifted.into_iter();

        // Create Key Aggregation Context from the the msg.sender keys
        let msg_senders_key_agg_ctx: KeyAggContext = KeyAggContext::new(msg_senders_iter).unwrap();

        Projector {
            operator_key,
            msg_senders,
            msg_senders_key_agg_ctx,
            tag,
        }
    }

    pub fn msg_senders_aggregate_key(&self) -> Key {
        self.msg_senders_key_agg_ctx.aggregated_pubkey()
    }

    pub fn operator_key(&self) -> Key {
        self.operator_key
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
