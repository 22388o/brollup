#![allow(dead_code)]

use musig2::secp256k1::XOnlyPublicKey;

type Key = XOnlyPublicKey;

#[derive(Clone, Copy)]
pub struct Account {
    pub key: Key,
    pub key_index: Option<u32>,
}

impl Account {
    pub fn new(key: Key) -> Account {
        Account { key, key_index: None }
    }

    pub fn new_compact(key: Key, index: u32) -> Account {
        Account {
            key,
            key_index: Some(index),
        }
    }
}
