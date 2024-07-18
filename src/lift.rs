#![allow(dead_code)]

use crate::{
    operator,
    serialize::{to_csv_script_encode, CSVFlag},
    taproot::{TapLeaf, TapRoot},
};
use musig2::secp256k1::{self, XOnlyPublicKey};

type Bytes = Vec<u8>;

pub struct Lift {
    operator_key: XOnlyPublicKey,
    self_key: XOnlyPublicKey,
}

impl Lift {
    pub fn new(self_key: XOnlyPublicKey) -> Lift {
        let operator_key = XOnlyPublicKey::from_slice(&operator::OPERATOR_KEY_WELL_KNOWN).unwrap();
        Lift {
            operator_key,
            self_key,
        }
    }

    pub fn new_with_operator(self_key: XOnlyPublicKey, operator_key: XOnlyPublicKey) -> Lift {
        Lift {
            operator_key,
            self_key,
        }
    }

    pub fn self_key(&self) -> XOnlyPublicKey {
        self.self_key
    }

    pub fn operator_key(&self) -> XOnlyPublicKey {
        self.operator_key
    }

    pub fn taproot(&self) -> TapRoot {
        //// Collab path
        let mut collab_path = Vec::<u8>::new();

        // Push self key
        collab_path.push(0x20);
        collab_path.extend(self.self_key().serialize().to_vec());

        // OP_CHECKSIGVERIFY
        collab_path.push(0xad);

        // Push operator key
        collab_path.push(0x20);
        collab_path.extend(self.operator_key().serialize().to_vec());

        // OP_CHECKSIG
        collab_path.push(0xac);

        //// Exit path
        let mut exit_path = Vec::<u8>::new();

        // Relative timelock
        exit_path.extend(to_csv_script_encode(CSVFlag::CSVMonth));

        // Push self key
        exit_path.push(0x20);
        exit_path.extend(self.self_key().serialize().to_vec());

        // OP_CHECKSIG
        exit_path.push(0xac);

        let leaves = vec![TapLeaf::new(collab_path), TapLeaf::new(exit_path)];
        TapRoot::script_path_only_multi(leaves)
    }

    pub fn spk(&self) -> Result<Bytes, secp256k1::Error> {
        self.taproot().spk()
    }
}
