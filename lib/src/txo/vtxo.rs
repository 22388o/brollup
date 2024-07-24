#![allow(dead_code)]

use crate::{
    serialize::{to_csv_script_encode, CSVFlag},
    taproot::{TapLeaf, TapRoot},
    well_known::operator,
};
use musig2::secp256k1::{self, XOnlyPublicKey};

type Bytes = Vec<u8>;
type Key = XOnlyPublicKey;

pub struct VTXO {
    self_key: Key,
    operator_key_well_known: Key,
}

impl VTXO {
    pub fn new(self_key: Key) -> VTXO {
        let operator_key_well_known = Key::from_slice(&operator::OPERATOR_KEY_WELL_KNOWN).unwrap();
        VTXO {
            self_key,
            operator_key_well_known,
        }
    }

    pub fn new_with_operator(self_key: Key, operator_key_well_known: Key) -> VTXO {
        VTXO {
            operator_key_well_known,
            self_key,
        }
    }

    pub fn self_key(&self) -> Key {
        self.self_key
    }

    pub fn operator_key(&self) -> Key {
        self.operator_key_well_known
    }

    pub fn taproot(&self) -> TapRoot {
        //// Channel path
        let mut channel_path = Vec::<u8>::new();

        // Push self key
        channel_path.push(0x20);
        channel_path.extend(self.self_key().serialize());

        // OP_CHECKSIGVERIFY
        channel_path.push(0xad);

        // Push operator key
        channel_path.push(0x20);
        channel_path.extend(self.operator_key().serialize());

        // OP_CHECKSIG
        channel_path.push(0xac);

        //// Exit path
        let mut exit_path = Vec::<u8>::new();

        // Relative timelock - VTXO is like Lift, but lives for three months instead
        exit_path.extend(to_csv_script_encode(CSVFlag::CSVThreeMonths));

        // Push self key
        exit_path.push(0x20);
        exit_path.extend(self.self_key().serialize());

        // OP_CHECKSIG
        exit_path.push(0xac);

        let leaves = vec![TapLeaf::new(channel_path), TapLeaf::new(exit_path)];
        TapRoot::script_path_only_multi(leaves)
    }

    pub fn spk(&self) -> Result<Bytes, secp256k1::Error> {
        self.taproot().spk()
    }
}
