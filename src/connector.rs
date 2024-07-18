#![allow(dead_code)]

use crate::{
    operator,
    taproot::{TapLeaf, TapRoot},
};
use musig2::secp256k1::{self, XOnlyPublicKey};

type Bytes = Vec<u8>;

pub struct Connector {
    operator_key: XOnlyPublicKey,
    self_key: XOnlyPublicKey,
}

impl Connector {
    pub fn new(self_key: XOnlyPublicKey) -> Connector {
        let operator_key = XOnlyPublicKey::from_slice(&operator::OPERATOR_KEY_WELL_KNOWN).unwrap();
        Connector {
            operator_key,
            self_key,
        }
    }

    pub fn new_with_operator(self_key: XOnlyPublicKey, operator_key: XOnlyPublicKey) -> Connector {
        Connector {
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

        let mut connector_script = Vec::<u8>::new();

        // Push self key
        connector_script.push(0x20);
        connector_script.extend(self.self_key().serialize().to_vec());

        // OP_CHECKSIGVERIFY
        connector_script.push(0xad);

        // Push operator key
        connector_script.push(0x20);
        connector_script.extend(self.operator_key().serialize().to_vec());

        // OP_CHECKSIG
        connector_script.push(0xac);

        TapRoot::script_path_only_single(TapLeaf::new(connector_script))
    }

    pub fn spk(&self) -> Result<Bytes, secp256k1::Error> {
        self.taproot().spk()
    }
}
