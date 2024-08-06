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
    fn with_prefix_pushdata(&self) -> Bytes {
        with_prefix(self, PrefixFlag::PrefixPushdata)
    }

    fn with_prefix_compact_size(&self) -> Bytes {
        with_prefix(self, PrefixFlag::PrefixCompactSize)
    }
}

fn with_prefix(data: &Bytes, flag: PrefixFlag) -> Bytes {
    let mut return_vec = Vec::<u8>::new();
    let data_len = data.len();

    match flag {
        PrefixFlag::PrefixCompactSize => {
            match data_len {
                0..=252 => return_vec.extend(vec![data_len as u8]),
                253..=65535 => {
                    return_vec.extend([0xfd]);

                    let data_len_bytes: [u8; 2] = (data_len as u16).to_le_bytes();
                    return_vec.extend(data_len_bytes);
                }
                65536..=4294967295 => {
                    return_vec.extend([0xfe]);

                    let data_len_bytes: [u8; 4] = (data_len as u32).to_le_bytes();
                    return_vec.extend(data_len_bytes);
                }
                4294967296..=18446744073709551615 => {
                    return_vec.extend([0xff]);

                    let data_len_bytes: [u8; 8] = (data_len as u64).to_le_bytes();
                    return_vec.extend(data_len_bytes);
                }
                _ => panic!(),
            }
            return_vec.extend(data);
            return_vec
        }
        PrefixFlag::PrefixPushdata => {
            if data_len == 1 && &data[0] <= &16 {
                // Minimal push
                match &data[0] {
                    0x00 => return_vec.push(0x00), // OP_0
                    0x01 => return_vec.push(0x51), // OP_1
                    0x02 => return_vec.push(0x52), // OP_2
                    0x03 => return_vec.push(0x53), // OP_3
                    0x04 => return_vec.push(0x54), // OP_4
                    0x05 => return_vec.push(0x55), // OP_5
                    0x06 => return_vec.push(0x56), // OP_6
                    0x07 => return_vec.push(0x57), // OP_7
                    0x08 => return_vec.push(0x58), // OP_8
                    0x09 => return_vec.push(0x59), // OP_9
                    0x0a => return_vec.push(0x5a), // OP_10
                    0x0b => return_vec.push(0x5b), // OP_11
                    0x0c => return_vec.push(0x5c), // OP_12
                    0x0d => return_vec.push(0x5d), // OP_13
                    0x0e => return_vec.push(0x5e), // OP_14
                    0x0f => return_vec.push(0x5f), // OP_15
                    0x10 => return_vec.push(0x60), // OP_16
                    _ => panic!(),
                }
            } else {
                match data_len {
                    0..=75 => return_vec.extend(vec![data_len as u8]),
                    76..=255 => {
                        return_vec.extend([0x4c]);
                        return_vec.extend([data_len as u8]);
                    }
                    256..=65535 => {
                        return_vec.extend(vec![0x4d]);

                        let x_bytes: [u8; 2] = (data_len as u16).to_le_bytes();
                        return_vec.extend(x_bytes);
                    }
                    65536..=4294967295 => {
                        return_vec.extend([0x4e]);

                        let x_bytes: [u8; 4] = (data_len as u32).to_le_bytes();
                        return_vec.extend(x_bytes);
                    }
                    _ => panic!(),
                }
                return_vec.extend(data);
            }
            return_vec
        }
    }
}
