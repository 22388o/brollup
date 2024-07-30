#![allow(dead_code)]

use super::{value::ShortVal, BitVec, CompactPayloadEncoding};

pub struct Contract {
    id: [u8; 32],
    index: Option<u32>,
}

impl Contract {
    pub fn new(id: [u8; 32]) -> Contract {
        Contract { id, index: None }
    }

    pub fn new_compact(id: [u8; 32], index: u32) -> Contract {
        Contract {
            id,
            index: Some(index),
        }
    }
}

impl CompactPayloadEncoding for Contract {
    fn to_cpe(&self) -> BitVec {
        let mut bit_vec = BitVec::new();

        match self.index {
            None => {
                // Non-compact form
                bit_vec.push(false);

                let id_array = self.id;
                let id_bits = BitVec::from_bytes(&id_array);

                bit_vec.extend(id_bits);
            }
            Some(index) => {
                // Compact form
                bit_vec.push(true);

                // ShortAmount represents compact integer forms
                let index_compact = ShortVal(index);

                bit_vec.extend(index_compact.to_cpe());
            }
        }

        bit_vec
    }
}
