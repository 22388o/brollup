#![allow(dead_code)]

use musig2::secp256k1::XOnlyPublicKey;

use super::{amount::ShortAmount, BitVec, ToBitVec};

type Key = XOnlyPublicKey;

pub struct Account {
    key: Key,
    index: Option<u32>,
}

impl Account {
    pub fn new(key: Key) -> Account {
        Account { key, index: None }
    }

    pub fn new_compact(key: Key, index: u32) -> Account {
        Account {
            key,
            index: Some(index),
        }
    }
}

impl ToBitVec for Account {
    fn to_bit_vec(&self) -> BitVec {
        let mut bit_vec = BitVec::new();

        match self.index {
            None => {
                // Non-compact form
                bit_vec.push(false);

                let key_array = self.key.serialize();
                let key_bits = BitVec::from_bytes(&key_array);

                bit_vec.extend(key_bits);
            }
            Some(index) => {
                // Compact form
                bit_vec.push(true);

                // ShortAmount represents compact integer forms
                let index_compact = ShortAmount(index);

                bit_vec.extend(index_compact.to_bit_vec());
            }
        }

        bit_vec
    }
}
