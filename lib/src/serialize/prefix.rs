#![allow(dead_code)]
type Bytes = Vec<u8>;

enum PrefixFlag {
    // https://en.bitcoin.it/wiki/Script
    PrefixPushdata,
    // https://en.bitcoin.it/wiki/Protocol_documentation#Variable_length_integer
    PrefixCompactSize,
}

pub trait Prefix {
    fn with_prefix_pushdata(&self) -> Bytes;
    fn with_prefix_compact_size(&self) -> Bytes;
}

impl Prefix for Bytes {
    fn with_prefix_pushdata(&self) -> Self {
        let mut bytes = Vec::<u8>::new();
        let data_len = self.len();

        if data_len == 1 && &self[0] <= &0x10 {
            // Minimal push
            match &self[0] {
                0x00 => bytes.push(0x00), // OP_0
                0x01 => bytes.push(0x51), // OP_1
                0x02 => bytes.push(0x52), // OP_2
                0x03 => bytes.push(0x53), // OP_3
                0x04 => bytes.push(0x54), // OP_4
                0x05 => bytes.push(0x55), // OP_5
                0x06 => bytes.push(0x56), // OP_6
                0x07 => bytes.push(0x57), // OP_7
                0x08 => bytes.push(0x58), // OP_8
                0x09 => bytes.push(0x59), // OP_9
                0x0a => bytes.push(0x5a), // OP_10
                0x0b => bytes.push(0x5b), // OP_11
                0x0c => bytes.push(0x5c), // OP_12
                0x0d => bytes.push(0x5d), // OP_13
                0x0e => bytes.push(0x5e), // OP_14
                0x0f => bytes.push(0x5f), // OP_15
                0x10 => bytes.push(0x60), // OP_16
                _ => (),
            }
        } else {
            match data_len {
                0..=75 => bytes.extend(vec![data_len as u8]),
                76..=255 => {
                    bytes.extend([0x4c]);
                    bytes.extend([data_len as u8]);
                }
                256..=65535 => {
                    bytes.extend(vec![0x4d]);

                    let x_bytes: [u8; 2] = (data_len as u16).to_le_bytes();
                    bytes.extend(x_bytes);
                }
                65536..=4294967295 => {
                    bytes.extend([0x4e]);

                    let x_bytes: [u8; 4] = (data_len as u32).to_le_bytes();
                    bytes.extend(x_bytes);
                }
                _ => panic!("Out of range data to prefix."),
            }
            bytes.extend(self);
        }
        bytes
    }

    fn with_prefix_compact_size(&self) -> Self {
        let mut bytes = Vec::<u8>::new();
        let data_len = self.len();

        match data_len {
            0..=252 => bytes.extend(vec![data_len as u8]),
            253..=65535 => {
                bytes.extend([0xfd]);

                let data_len_bytes: [u8; 2] = (data_len as u16).to_le_bytes();
                bytes.extend(data_len_bytes);
            }
            65536..=4294967295 => {
                bytes.extend([0xfe]);

                let data_len_bytes: [u8; 4] = (data_len as u32).to_le_bytes();
                bytes.extend(data_len_bytes);
            }
            4294967296..=18446744073709551615 => {
                bytes.extend([0xff]);

                let data_len_bytes: [u8; 8] = (data_len as u64).to_le_bytes();
                bytes.extend(data_len_bytes);
            }
            _ => panic!("Out of range data to prefix."),
        }
        bytes.extend(self);
        bytes
    }
}
