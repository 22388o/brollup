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

#[derive(Clone, Copy)]
pub enum ProjectorTag {
    VTXOProjector,
    ConnectorProjector,
}

#[derive(Clone)]
pub struct Projector {
    operator_key: XOnlyPublicKey,
    msg_senders: Vec<XOnlyPublicKey>,
    msg_senders_aggregate_key: XOnlyPublicKey,
    msg_senders_key_agg_ctx: KeyAggContext,
    tag: ProjectorTag,
}

impl Projector {
    pub fn new(msg_senders: Vec<XOnlyPublicKey>, tag: ProjectorTag) -> Projector {
        let operator_key = XOnlyPublicKey::from_slice(&operator::OPERATOR_KEY_WELL_KNOWN).unwrap();

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

        let msg_senders_aggregate_key: PublicKey = msg_senders_key_agg_ctx.aggregated_pubkey();

        Projector {
            operator_key,
            msg_senders,
            msg_senders_aggregate_key: msg_senders_aggregate_key.x_only_public_key().0,
            msg_senders_key_agg_ctx,
            tag
        }
    }

    pub fn new_with_operator(msg_senders: Vec<XOnlyPublicKey>, operator_key: XOnlyPublicKey, tag: ProjectorTag) -> Projector {

        let mut msg_senders_lifted = Vec::<PublicKey>::new();

        for sender in &msg_senders {
            msg_senders_lifted.push(sender.public_key(secp256k1::Parity::Even));
        }

        // Sort the keys
        msg_senders_lifted.sort();

        let msg_senders_iter = msg_senders_lifted.into_iter();

        let msg_senders_key_agg_ctx: KeyAggContext = KeyAggContext::new(msg_senders_iter).unwrap();

        let msg_senders_aggregate_key: PublicKey = msg_senders_key_agg_ctx.aggregated_pubkey();

        Projector {
            operator_key,
            msg_senders,
            msg_senders_aggregate_key: msg_senders_aggregate_key.x_only_public_key().0,
            msg_senders_key_agg_ctx,
            tag
        }
    }

    pub fn msg_senders_aggregate_key(&self) -> XOnlyPublicKey {
        self.msg_senders_aggregate_key
    }

    pub fn operator_key(&self) -> XOnlyPublicKey {
        self.operator_key
    }

    pub fn taproot(&self) -> TapRoot {

        // Reveal path
        let mut reveal_path = Vec::<u8>::new();

        // Push aggregate key
        reveal_path.push(0x20);
        reveal_path.extend(self.msg_senders_aggregate_key().serialize().to_vec());

        // OP_CHECKSIGVERIFY
        reveal_path.push(0xad);

        // Push operator key
        reveal_path.push(0x20);
        reveal_path.extend(self.operator_key().serialize().to_vec());

        // OP_CHECKSIG
        reveal_path.push(0xac);

        //// Reclaim path
        let mut reclaim_path = Vec::<u8>::new();

        // Relative timelock - VTXO is like Lift, but lives for three months instead
        reclaim_path.extend(to_csv_script_encode(CSVFlag::CSVThreeMonths));

        // Push 32-bytes
        reclaim_path.push(0x20);
        reclaim_path.extend(self.operator_key().serialize().to_vec());

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
