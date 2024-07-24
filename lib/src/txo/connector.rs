#![allow(dead_code)]

use crate::{
    taproot::{TapLeaf, TapRoot},
    well_known::operator,
};
use musig2::secp256k1::{self, XOnlyPublicKey};

type Bytes = Vec<u8>;
type Key = XOnlyPublicKey;

#[derive(Clone, Copy)]
pub enum ConnectorTag {
    Bare,
    Virtual,
}

pub struct Connector {
    msg_sender_key: Option<Key>,
    operator_key_well_known: Key,
    tag: ConnectorTag,
}

impl Connector {
    pub fn new_bare() -> Connector {
        let operator_key_well_known = Key::from_slice(&operator::OPERATOR_KEY_WELL_KNOWN).unwrap();
        Connector {
            msg_sender_key: None,
            operator_key_well_known,
            tag: ConnectorTag::Bare,
        }
    }

    pub fn new_bare_with_operator(operator_key_well_known: Key) -> Connector {
        Connector {
            msg_sender_key: None,
            operator_key_well_known,
            tag: ConnectorTag::Bare,
        }
    }

    pub fn new_virtual(msg_sender_key: Key) -> Connector {
        let operator_key_well_known = Key::from_slice(&operator::OPERATOR_KEY_WELL_KNOWN).unwrap();
        Connector {
            msg_sender_key: Some(msg_sender_key),
            operator_key_well_known,
            tag: ConnectorTag::Virtual,
        }
    }

    pub fn new_virtual_operator(msg_sender_key: Key, operator_key_well_known: Key) -> Connector {
        Connector {
            msg_sender_key: Some(msg_sender_key),
            operator_key_well_known,
            tag: ConnectorTag::Virtual,
        }
    }

    pub fn msg_sender_key(&self) -> Option<Key> {
        self.msg_sender_key
    }

    pub fn operator_key(&self) -> Key {
        self.operator_key_well_known
    }

    pub fn tag(&self) -> ConnectorTag {
        self.tag
    }

    pub fn taproot(&self) -> TapRoot {
        let mut connector_script = Vec::<u8>::new();

        match self.tag {
            ConnectorTag::Bare => {
                // Push operator key
                connector_script.push(0x20);
                connector_script.extend(self.operator_key().serialize());

                // OP_CHECKSIG
                connector_script.push(0xac);
            }
            ConnectorTag::Virtual => {
                // Push msg.sender key
                connector_script.push(0x20);
                connector_script.extend(self.msg_sender_key().unwrap().serialize());

                // OP_CHECKSIGVERIFY
                connector_script.push(0xad);

                // Push operator key
                connector_script.push(0x20);
                connector_script.extend(self.operator_key().serialize());

                // OP_CHECKSIG
                connector_script.push(0xac);
            }
        }

        TapRoot::script_path_only_single(TapLeaf::new(connector_script))
    }

    pub fn spk(&self) -> Result<Bytes, secp256k1::Error> {
        self.taproot().spk()
    }
}
