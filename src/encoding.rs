use std::{usize, vec};

use sha2::digest::consts::True;

// https://en.bitcoin.it/wiki/Protocol_documentation#Variable_length_integer
pub fn prefix_compact_size(data: &Vec<u8>) -> Vec<u8> {
    let mut return_vec: Vec<u8> = Vec::<u8>::new();

    match data.len() {
        x if x < 0xFD => return_vec.extend(vec![x as u8]),
        x if x <= 0xFFFF => {
            return_vec.extend(vec![0xfd]);
            let vec_u8: Vec<u8> = vec![(x & 0xFF) as u8, (x >> 8 & 0xFF) as u8];
            return_vec.extend(vec_u8)
        }
        x if x <= 0xFFFFFFFF => {
            return_vec.extend(vec![0xfe]);
            let vec_u8: Vec<u8> = vec![
                (x & 0xFF) as u8,
                ((x >> 8) & 0xFF) as u8,
                ((x >> 16) & 0xFF) as u8,
                ((x >> 24) & 0xFF) as u8,
            ];
            return_vec.extend(vec_u8)
        }
        x if x < 0xFFFFFFFFFFFFFFFF => {
            return_vec.extend(vec![0xff]);
            let vec_u8: Vec<u8> = vec![
                (x & 0xFF) as u8,
                ((x >> 8) & 0xFF) as u8,
                ((x >> 16) & 0xFF) as u8,
                ((x >> 24) & 0xFF) as u8,
                ((x >> 32) & 0xFF) as u8,
                ((x >> 40) & 0xFF) as u8,
                ((x >> 48) & 0xFF) as u8,
                ((x >> 56) & 0xFF) as u8,
            ];
            return_vec.extend(vec_u8)
        }
        _ => panic!(),
    }
    return_vec.extend(data);
    return_vec
}

// https://en.bitcoin.it/wiki/Script
pub fn prefix_pushdata(data: &Vec<u8>) -> Vec<u8> {
    let mut return_vec: Vec<u8> = Vec::<u8>::new();

    match data.len() {
        x if x <= 75 => return_vec.extend(vec![x as u8]),
        x if x <= 0xFF => {
            return_vec.extend(vec![0x4c]);
            return_vec.extend(vec![x as u8])
        }
        x if x <= 0xFFFF => {
            return_vec.extend(vec![0x4d]);

            let vec_u8: Vec<u8> = vec![(x & 0xFF) as u8, (x >> 8 & 0xFF) as u8];

            return_vec.extend(vec_u8)
        }
        x if x <= 0xFFFFFFFF => {
            return_vec.extend(vec![0x4e]);

            // In little endian order
            let vec_u8: Vec<u8> = vec![
                (x & 0xFF) as u8,
                ((x >> 8) & 0xFF) as u8,
                ((x >> 16) & 0xFF) as u8,
                ((x >> 24) & 0xFF) as u8,
            ];

            return_vec.extend(vec_u8)
        }
        _ => panic!(),
    }
    return_vec.extend(data);
    return_vec
}

pub enum PushFlag {
    WitnessStandardPush,
    WitnessNonStandardPush,
    ScriptPush,
}

pub fn multi_push_encode(data: &Vec<u8>, flag: PushFlag) {
    let mut return_vec: Vec<u8> = Vec::<u8>::new();

    let chunk_size_max: u16 = match flag {
        // https://github.com/bitcoin/bitcoin/blob/master/src/policy/policy.h#L45
        PushFlag::WitnessStandardPush => 80,
        // https://github.com/bitcoin/bitcoin/blob/master/src/script/script.h#L27
        PushFlag::WitnessNonStandardPush => 520,
        // https://github.com/bitcoin/bitcoin/blob/master/src/script/script.h#L27
        PushFlag::ScriptPush => 520,
    };

    let num_chunks = 1 + (data.len() / chunk_size_max as usize);

    let (chunk_size, mut chunk_leftover) = (data.len() / num_chunks, data.len() % num_chunks);

    for i in 0..num_chunks {
        let start = i * chunk_size;
        let mut end: usize = start + chunk_size;

        // Distribute leftovers by one
        if chunk_leftover > 0 {
            end = end + 1;
            chunk_leftover = chunk_leftover - 1;
        }

        let chunk = data.clone()[start..end].to_vec();

        match flag {
            // Use OP_PUSHDATA encoding for in-script witness pushes
            PushFlag::ScriptPush => return_vec.extend(prefix_pushdata(&chunk)),
            // Use varint encoding for out-script witness pushes
            _ => return_vec.extend(prefix_compact_size(&chunk)),
        }
    }
}
