#![allow(dead_code)]

use crate::{
    serialize::{to_csv_script_encode, CSVFlag},
    taproot::{TapLeaf, TapRoot},
    well_known::operator,
};
use musig2::secp256k1::{self, XOnlyPublicKey};

type Bytes = Vec<u8>;
type Key = XOnlyPublicKey;

pub struct Payload {
    fresh_operator_key_dynamic: Key,
    vtxo_projector_msg_senders_agg_sig: Bytes,
    vtxo_projector_operator_s_commitment: Bytes,
    connector_projector_msg_senders_agg_sig: Bytes,
    connector_projector_operator_s_commitment: Bytes,
    msg_senders: Vec<Key>,
    s_commitments: Vec<Bytes>,
    entries: Vec<Bytes>,
    entries_zero_bits_padding: u8,
}
