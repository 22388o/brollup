pub mod transfer;
use bit_vec::BitVec;

pub fn value_to_compact_bit_vec(value: u32) -> BitVec {
    let mut bit_vec = BitVec::new();
    match value {
        0..=255 => {
            // 00 is 1-byte
            bit_vec.push(false);
            bit_vec.push(false);

            let val_byte = vec![value as u8];
            let val_bits = BitVec::from_bytes(&val_byte);
            bit_vec.extend(val_bits);
        }
        256..=65535 => {
            // 01 is 2-bytes
            bit_vec.push(false);
            bit_vec.push(true);

            let val_bytes = vec![(value & 0xFF) as u8, (value >> 8 & 0xFF) as u8];
            let val_bits = BitVec::from_bytes(&val_bytes);
            bit_vec.extend(val_bits);
        }
        65536..=16777215 => {
            // 10 is 3-bytes
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
            // 11 is 4-bytes
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
