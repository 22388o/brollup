#![allow(dead_code)]

use bit_vec::BitVec;
use crate::serialize::cpe::CompactPayloadEncoding;
use super::value::ShortVal;

#[derive(Clone, Copy)]
pub struct Contract {
    id: [u8; 32],
    id_index: Option<u32>,
}

impl Contract {
    pub fn new(id: [u8; 32]) -> Contract {
        Contract { id, id_index: None }
    }

    pub fn new_compact(id: [u8; 32], id_index: u32) -> Contract {
        Contract {
            id,
            id_index: Some(id_index),
        }
    }
}

impl CompactPayloadEncoding for Contract {
    fn to_cpe(&self) -> BitVec {
        let mut bit_vec = BitVec::new();

        match self.id_index {
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
