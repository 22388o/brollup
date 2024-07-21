#![allow(dead_code)]

use crate::{
    serialize::{to_csv_script_encode, CSVFlag},
    taproot::{TapLeaf, TapRoot},
    well_known::operator,
};
use musig2::secp256k1::{self, XOnlyPublicKey};

type Bytes = Vec<u8>;
type Key = XOnlyPublicKey;

const DEGRADING_PERIOD_START_AT: u8 = 141;

pub struct Channel {
    operator_key: Key,
    self_key: Key,
}

impl Channel {
    pub fn new(self_key: Key) -> Channel {
        let operator_key = Key::from_slice(&operator::OPERATOR_KEY_WELL_KNOWN).unwrap();
        Channel {
            operator_key,
            self_key,
        }
    }

    pub fn new_with_operator(self_key: Key, operator_key: Key) -> Channel {
        Channel {
            operator_key,
            self_key,
        }
    }

    pub fn self_key(&self) -> Key {
        self.self_key
    }

    pub fn operator_key(&self) -> Key {
        self.operator_key
    }

    pub fn taproot(&self) -> TapRoot {
        let mut leaves = Vec::<TapLeaf>::new();

        for i in 0..128 {
            let mut tap_script = Vec::<u8>::new();

            // Degrading timelock period
            let days: u8 = DEGRADING_PERIOD_START_AT - i;
            tap_script.extend(to_csv_script_encode(CSVFlag::Days(days)));

            // Push self key
            tap_script.push(0x20);
            tap_script.extend(self.self_key().serialize());

            // OP_CHECKSIGVERIFY
            tap_script.push(0xad);

            // Push operator key
            tap_script.push(0x20);
            tap_script.extend(self.operator_key().serialize());

            // OP_CHECKSIG
            tap_script.push(0xac);

            leaves.push(TapLeaf::new(tap_script));
        }

        TapRoot::script_path_only_multi(leaves)
    }

    pub fn spk(&self) -> Result<Bytes, secp256k1::Error> {
        self.taproot().spk()
    }
}
