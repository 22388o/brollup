#![allow(dead_code)]

#[derive(Clone, Copy)]
pub struct Contract {
    pub id: [u8; 32],
    pub id_index: Option<u32>,
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
