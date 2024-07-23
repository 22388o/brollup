use bit_vec::BitVec;

use crate::{
    serialize::{to_csv_script_encode, CSVFlag},
    taproot::{TapLeaf, TapRoot},
};
use musig2::secp256k1::{self, XOnlyPublicKey};

type Bytes = Vec<u8>;
type Key = XOnlyPublicKey;

pub struct Payload {
    // msg.sender keys
    msg_senders: Vec<Key>,
    // s commitments
    s_commitments: Vec<[u8; 32]>,
    // Fresh operator key 🔑
    fresh_operator_key_dynamic: Key,
    // VTXO projector signatures 🎥
    vtxo_projector_msg_senders_agg_sig: [u8; 64],
    vtxo_projector_operator_s_commitment: [u8; 32],
    // Connector projector signatures 🎥
    connector_projector_msg_senders_agg_sig: [u8; 64],
    connector_projector_operator_s_commitment: [u8; 32],
    // Entries
    entries: Vec<BitVec>,
}

impl Payload {
    pub fn new(
        msg_senders: Vec<Key>,
        s_commitments: Vec<[u8; 32]>,
        fresh_operator_key_dynamic: Key,
        vtxo_projector_msg_senders_agg_sig: [u8; 64],
        vtxo_projector_operator_s_commitment: [u8; 32],
        connector_projector_msg_senders_agg_sig: [u8; 64],
        connector_projector_operator_s_commitment: [u8; 32],
        entries: Vec<BitVec>,
    ) -> Payload {
        Payload {
            msg_senders,
            s_commitments,
            fresh_operator_key_dynamic,
            vtxo_projector_msg_senders_agg_sig,
            vtxo_projector_operator_s_commitment,
            connector_projector_msg_senders_agg_sig,
            connector_projector_operator_s_commitment,
            entries,
        }
    }

    pub fn data_to_be_pushed(&self) -> Bytes {
        let mut data = Vec::<u8>::new();

        // Start with adding the fresh operator key 🔑
        data.extend(self.fresh_operator_key_dynamic.serialize().to_vec());

        // Add vtxo_projector_msg_senders_agg_sig (64 bytes)
        data.extend(self.vtxo_projector_msg_senders_agg_sig);

        // Add vtxo_projector_operator_s_commitment (32 bytes)
        data.extend(self.vtxo_projector_operator_s_commitment);

        // Add connector_projector_msg_senders_agg_sig (64 bytes)
        data.extend(self.connector_projector_msg_senders_agg_sig);

        // Add connector_projector_operator_s_commitment (32 bytes)
        data.extend(self.connector_projector_operator_s_commitment);

        let mut entries = BitVec::new();

        for entry in self.entries.iter() {
            entries.extend(entry);
        }

        let zero_bits_padded: u8 = 8 - (entries.len() % 8) as u8;
        data.push(zero_bits_padded);

        data.extend(entries.to_bytes());

        data
    }
}
