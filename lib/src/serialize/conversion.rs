use std::u8;

use uintx::{u24, u40, u48, u56};

type Bytes = Vec<u8>;

pub fn u64_to_array(value: u64) -> [u8; 8] {
    value.to_le_bytes()
}


pub fn u56_to_array(value: u56) -> [u8; 7] {
    value.to_le_bytes()
}

pub fn u48_to_array(value: u48) -> [u8; 6] {
    value.to_le_bytes()
}

pub fn u40_to_array(value: u40) -> [u8; 5] {
    value.to_le_bytes()
}

pub fn u32_to_array(value: u32) -> [u8; 4] {
    value.to_le_bytes()
}

pub fn u24_to_array(value: u24) -> [u8; 3] {
    value.to_le_bytes()
}

pub fn u16_to_array(value: u16) -> [u8; 2] {
    [(value & 0xFF) as u8, ((value >> 8) & 0xFF) as u8]
}
