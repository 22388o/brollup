use bit_vec::BitVec;

use crate::{
    entry::transfer::Transfer,
    valtype::{
        account::Account, contract::Contract, maybe_common::MaybeCommon, value::{LongVal, ShortVal}, ValType,
    },
};

use super::common_index::common_index_from_u8;

pub trait CompactPayloadEncoding {
    fn to_cpe(&self) -> BitVec;
}

impl CompactPayloadEncoding for Account {
    fn to_cpe(&self) -> BitVec {
        let mut bit_vec = BitVec::new();

        match self.key_index {
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

                // ShortVal represents compact integer forms
                let index_compact = ShortVal(index);

                bit_vec.extend(index_compact.to_cpe());
            }
        }

        bit_vec
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

impl<T: ValType + CompactPayloadEncoding> CompactPayloadEncoding for MaybeCommon<T> {
    fn to_cpe(&self) -> BitVec {
        let mut bit_vec = BitVec::new();

        match self {
            MaybeCommon::Uncommon(uncommon) => {
                // Common bit: false
                bit_vec.push(false);
                // Bit-encoding
                bit_vec.extend(uncommon.to_cpe());
                bit_vec
            }
            MaybeCommon::Common(_, common_index) => {
                // Common bit: true
                bit_vec.push(true);
                // 3-bit common index encoding
                bit_vec.extend(common_index_from_u8(common_index));
                bit_vec
            }
        }
    }
}


impl CompactPayloadEncoding for ShortVal {
    fn to_cpe(&self) -> BitVec {
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

impl CompactPayloadEncoding for LongVal {
    fn to_cpe(&self) -> BitVec {
        let value = self.0;
        let mut bit_vec = BitVec::new();

        match value {
            0..=4294967295 => {
                // Interpet as Short Val and cast to Long Val by appending a zero-bit prefix
                bit_vec.push(false);
                bit_vec.extend(ShortVal(value as u32).to_cpe());
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

impl CompactPayloadEncoding for Transfer {
    fn to_cpe(&self) -> BitVec {
        let mut bit_vec = BitVec::new();

        // Transfer or call
        bit_vec.push(false);

        // Transfer
        bit_vec.push(false);

        // From
        bit_vec.extend(self.from.to_cpe());

        // To
        bit_vec.extend(self.to.to_cpe());

        // Amount
        bit_vec.extend(self.amount.to_cpe());

        bit_vec
    }
}
