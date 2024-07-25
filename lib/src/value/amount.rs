#![allow(dead_code)]

use bit_vec::BitVec;

pub struct ShortAmount(pub u32);

pub struct LongAmount(pub u64);

pub trait ToBitVec {
    fn to_bit_vec(&self) -> BitVec;
}

impl ShortAmount {
    pub fn new(value: u32) -> ShortAmount {
        ShortAmount(value)
    }
}

impl LongAmount {
    pub fn new(value: u64) -> LongAmount {
        LongAmount(value)
    }
}

impl ToBitVec for ShortAmount {
    fn to_bit_vec(&self) -> BitVec {
        let value = self.0;
        let mut bit_vec = BitVec::new();

        match value {
            0x00..=255 => {
                // b00 -> UInt 8 (1-byte)
                bit_vec.push(false);
                bit_vec.push(false);

                let val_byte = vec![value as u8];
                let val_bits = BitVec::from_bytes(&val_byte);
                bit_vec.extend(val_bits);
            }

            256..=65535 => {
                // b01 -> UInt 16 (2 bytes)
                bit_vec.push(false);
                bit_vec.push(true);

                let val_bytes = vec![(value & 0xFF) as u8, (value >> 8 & 0xFF) as u8];
                let val_bits = BitVec::from_bytes(&val_bytes);
                bit_vec.extend(val_bits);
            }

            65536..=16777215 => {
                // b10 -> UInt 24 (3 bytes)
                bit_vec.push(true);
                bit_vec.push(false);

                let val_bytes = vec![
                    (value & 0xFF) as u8,
                    ((value >> 8) & 0xFF) as u8,
                    ((value >> 16) & 0xFF) as u8,
                ];

                let val_bits = BitVec::from_bytes(&val_bytes);
                bit_vec.extend(val_bits);
            }

            16777216..=4294967295 => {
                // b11 -> UInt 32 (4 bytes)
                bit_vec.push(true);
                bit_vec.push(true);

                let val_bytes = vec![
                    (value & 0xFF) as u8,
                    ((value >> 8) & 0xFF) as u8,
                    ((value >> 16) & 0xFF) as u8,
                    ((value >> 24) & 0xFF) as u8,
                ];

                let val_bits = BitVec::from_bytes(&val_bytes);
                bit_vec.extend(val_bits);
            }
        }

        bit_vec
    }
}

impl ToBitVec for LongAmount {
    fn to_bit_vec(&self) -> BitVec {
        let value = self.0;
        let mut bit_vec = BitVec::new();

        match value {
            0..=4294967295 => {
                // Interpet as Short Amount and cast to Long Amount by appending a zero-bit prefix
                bit_vec.push(false);
                bit_vec.extend(ShortAmount(value as u32).to_bit_vec());
            }

            4294967296..=1099511627775 => {
                // b100 -> UInt 40 (5 bytes)
                bit_vec.push(true);
                bit_vec.push(false);
                bit_vec.push(false);

                let val_bytes = vec![
                    (value & 0xFF) as u8,
                    ((value >> 8) & 0xFF) as u8,
                    ((value >> 16) & 0xFF) as u8,
                    ((value >> 24) & 0xFF) as u8,
                    ((value >> 32) & 0xFF) as u8,
                ];

                let val_bits = BitVec::from_bytes(&val_bytes);
                bit_vec.extend(val_bits);
            }

            1099511627776..=281474976710655 => {
                // b101 -> UInt 48 (6 bytes)
                bit_vec.push(true);
                bit_vec.push(false);
                bit_vec.push(true);

                let val_bytes = vec![
                    (value & 0xFF) as u8,
                    ((value >> 8) & 0xFF) as u8,
                    ((value >> 16) & 0xFF) as u8,
                    ((value >> 24) & 0xFF) as u8,
                    ((value >> 32) & 0xFF) as u8,
                    ((value >> 40) & 0xFF) as u8,
                ];

                let val_bits = BitVec::from_bytes(&val_bytes);
                bit_vec.extend(val_bits);
            }

            281474976710656..=72057594037927935 => {
                // b110 -> UInt 56 (7 bytes)
                bit_vec.push(true);
                bit_vec.push(true);
                bit_vec.push(false);

                let val_bytes = vec![
                    (value & 0xFF) as u8,
                    ((value >> 8) & 0xFF) as u8,
                    ((value >> 16) & 0xFF) as u8,
                    ((value >> 24) & 0xFF) as u8,
                    ((value >> 32) & 0xFF) as u8,
                    ((value >> 40) & 0xFF) as u8,
                    ((value >> 48) & 0xFF) as u8,
                ];

                let val_bits = BitVec::from_bytes(&val_bytes);
                bit_vec.extend(val_bits);
            }

            72057594037927936..=18446744073709551615 => {
                // b111 -> UInt 64 (8 bytes)
                bit_vec.push(true);
                bit_vec.push(true);
                bit_vec.push(true);

                let val_bytes = vec![
                    (value & 0xFF) as u8,
                    ((value >> 8) & 0xFF) as u8,
                    ((value >> 16) & 0xFF) as u8,
                    ((value >> 24) & 0xFF) as u8,
                    ((value >> 32) & 0xFF) as u8,
                    ((value >> 40) & 0xFF) as u8,
                    ((value >> 48) & 0xFF) as u8,
                ];

                let val_bits = BitVec::from_bytes(&val_bytes);
                bit_vec.extend(val_bits);
            }
        }

        bit_vec
    }
}
