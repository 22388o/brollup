#![allow(dead_code)]

use bit_vec::BitVec;
use musig2::secp256k1::XOnlyPublicKey;

use crate::hash::hash_160;

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

    fn group_s_commitments_by_two(&self) -> Vec<([u8; 32], Option<[u8; 32]>)> {
        let s_commitments = self.s_commitments.clone();
        let mut tuples: Vec<([u8; 32], Option<[u8; 32]>)> = Vec::new();

        let iterations = match s_commitments.len() {
            0 => 0,
            1 => 1,
            _ => s_commitments.len() / 2 + s_commitments.len() % 2,
        };

        for i in 0..iterations {
            let is_last:bool = i + 1 == iterations;

            match is_last {
                false => tuples.push((s_commitments[i * 2], Some(s_commitments[i * 2 + 1]))),
                true => match s_commitments.len() % 2 {
                    0 => tuples.push((s_commitments[i * 2], Some(s_commitments[i * 2 + 1]))),
                    1 => tuples.push((s_commitments[i * 2], None)),
                    _ => (),
                },
            }

        }

        tuples
    }

    fn hashlocks(&self) -> Vec<[u8; 20]> {
        let s_commitments_grouped = self.group_s_commitments_by_two();
        let mut hashes = Vec::<[u8; 20]>::new();

        for group in s_commitments_grouped {
            let mut full = Vec::<u8>::new();

            full.extend(group.0);

            if let Some(s_com) = group.1 {
                full.extend(s_com);
            }
            hashes.push(hash_160(full));
        }
        hashes
    }

    fn data_to_be_pushed(&self) -> Bytes {
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
