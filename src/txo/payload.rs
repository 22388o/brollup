#![allow(dead_code)]

use bit_vec::BitVec;

use musig2::secp256k1::XOnlyPublicKey;

type Bytes = Vec<u8>;
type Key = XOnlyPublicKey;

pub struct Payload {
    msg_senders: Vec<Key>,
    operator_key_well_known: Key,
    s_commitments: Vec<[u8; 32]>,
    fresh_operator_key_dynamic: Key,
    vtxo_projector_msg_senders_agg_sig: [u8; 64],
    vtxo_projector_operator_s_commitment: [u8; 32],
    connector_projector_msg_senders_agg_sig: [u8; 64],
    connector_projector_operator_s_commitment: [u8; 32],
    entries: Vec<BitVec>,
}

impl Payload {
    pub fn new(
        msg_senders: Vec<Key>,
        operator_key_well_known: Key,
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
            operator_key_well_known,
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

        // Start with adding the fresh operator key
        data.extend(self.fresh_operator_key_dynamic.serialize().to_vec());

        // Add vtxo_projector_msg_senders_agg_sig (64 bytes)
        data.extend(self.vtxo_projector_msg_senders_agg_sig);

        // Add vtxo_projector_operator_s_commitment (32 bytes)
        data.extend(self.vtxo_projector_operator_s_commitment);

        // Add connector_projector_msg_senders_agg_sig (64 bytes)
        data.extend(self.connector_projector_msg_senders_agg_sig);

        // Add connector_projector_operator_s_commitment (32 bytes)
        data.extend(self.connector_projector_operator_s_commitment);

        let mut entries_whole = BitVec::new();

        for entry in self.entries.iter() {
            entries_whole.extend(entry);
        }

        let zero_bits_padded: u8 = 8 - (entries_whole.len() % 8) as u8;

        // Add the length of padded zero-bits
        data.push(zero_bits_padded);

        // Add entries
        data.extend(entries_whole.to_bytes());

        data
    }
}
