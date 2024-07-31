#![allow(dead_code)]

use bit_vec::BitVec;
use musig2::secp256k1::{self, XOnlyPublicKey};

use crate::musig2::keys_to_key_agg_ctx;
use crate::serialize::{push::encode_multi_push, csv::CSVFlag};
use crate::taproot::TapLeaf;
use crate::{hash::hash_160, serialize::csv::to_csv_script_encode, taproot::TapRoot};
use crate::serialize::push::PushFlag;

type Bytes = Vec<u8>;
type Key = XOnlyPublicKey;

pub struct Payload {
    msg_senders: Vec<Key>,
    operator_key_well_known: Key,
    s_commitments: Vec<[u8; 32]>,
    sats_per_vbyte: u8,
    liquidity_basis_points: u8,
    fresh_operator_key_dynamic: Key,
    vtxo_projector_agg_sig: [u8; 64],
    connector_projector_agg_sig: [u8; 64],
    entries: Vec<BitVec>,
}

impl Payload {
    pub fn new(
        msg_senders: Vec<Key>,
        operator_key_well_known: Key,
        s_commitments: Vec<[u8; 32]>,
        sats_per_vbyte: u8,
        liquidity_basis_points: u8,
        fresh_operator_key_dynamic: Key,
        vtxo_projector_agg_sig: [u8; 64],
        connector_projector_agg_sig: [u8; 64],
        entries: Vec<BitVec>,
    ) -> Payload {
        Payload {
            msg_senders,
            operator_key_well_known,
            s_commitments,
            sats_per_vbyte,
            liquidity_basis_points,
            fresh_operator_key_dynamic,
            vtxo_projector_agg_sig,
            connector_projector_agg_sig,
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
            let is_last: bool = i + 1 == iterations;

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

    fn payload(&self) -> Bytes {
        let mut data = Vec::<u8>::new();

        // Start with feerate
        data.push(self.sats_per_vbyte);

        // Add basis points
        data.push(self.liquidity_basis_points);

        // Add the fresh operator key
        data.extend(self.fresh_operator_key_dynamic.serialize().to_vec());

        // Add vtxo_projector_agg_sig (64 bytes)
        data.extend(self.vtxo_projector_agg_sig);

        // Add connector_projector_agg_sig (64 bytes)
        data.extend(self.connector_projector_agg_sig);

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

    fn msg_senders_aggregate_key(&self) -> XOnlyPublicKey {

        let key_agg_ctx = keys_to_key_agg_ctx(&self.msg_senders);

        // consider removing unwrap here
        key_agg_ctx.unwrap().aggregated_pubkey()
    }

    pub fn taproot(&self) -> TapRoot {
        let mut tap_script = Vec::<u8>::new();

        // OP_IF
        tap_script.push(0x63);

        // Haslocks
        let hashlocks = self.hashlocks();
        for hashlock in hashlocks {
            // OP_HASH160
            tap_script.push(0xa9);

            // Push hash into stack
            tap_script.push(0x14);
            tap_script.extend(hashlock);

            // OP_EQUALVERIFY
            tap_script.push(0x88);
        }

        // Push operator key into stack
        tap_script.push(0x20);
        tap_script.extend(self.operator_key_well_known.serialize());

        // OP_CHECKSIG
        tap_script.push(0xac);

        // OP_ELSE
        tap_script.push(0x67);

        tap_script.extend(to_csv_script_encode(CSVFlag::CSVWeek));

        // Push msg.senders aggregate key into stack
        tap_script.push(0x20);
        tap_script.extend(self.msg_senders_aggregate_key().serialize());

        // OP_CHECKSIG
        tap_script.push(0xac);

        // OP_ENDIF
        tap_script.push(0x68);

        // Push payload
        tap_script.extend(encode_multi_push(&self.payload(), PushFlag::ScriptPush));

        let tap_leaf = TapLeaf::new(tap_script);
        let tap_root = TapRoot::script_path_only_single(tap_leaf);

        tap_root
    }

    pub fn spk(&self) -> Result<Bytes, secp256k1::Error> {
        self.taproot().spk()
    }
}
