use std::{usize, vec};

use sha2::digest::consts::True;

type Bytes = Vec<u8>;

// https://en.bitcoin.it/wiki/Protocol_documentation#Variable_length_integer
pub fn with_prefix_compact_size(data: &Bytes) -> Bytes {
    let mut return_vec: Bytes = Vec::<u8>::new();

    match data.len() {
        x if x < 0xFD => return_vec.extend(vec![x as u8]),
        x if x <= 0xFFFF => {
            return_vec.extend(vec![0xfd]);
            let vec_u8: Bytes = vec![(x & 0xFF) as u8, (x >> 8 & 0xFF) as u8];
            return_vec.extend(vec_u8)
        }
        x if x <= 0xFFFFFFFF => {
            return_vec.extend(vec![0xfe]);
            let vec_u8: Bytes = vec![
                (x & 0xFF) as u8,
                ((x >> 8) & 0xFF) as u8,
                ((x >> 16) & 0xFF) as u8,
                ((x >> 24) & 0xFF) as u8,
            ];
            return_vec.extend(vec_u8)
        }
        x if x < 0xFFFFFFFFFFFFFFFF => {
            return_vec.extend(vec![0xff]);
            let vec_u8: Bytes = vec![
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
pub fn with_prefix_pushdata(data: &Bytes) -> Bytes {
    let mut return_vec: Bytes = Vec::<u8>::new();

    match data.len() {
        x if x <= 75 => return_vec.extend(vec![x as u8]),
        x if x <= 0xFF => {
            return_vec.extend(vec![0x4c]);
            return_vec.extend(vec![x as u8])
        }
        x if x <= 0xFFFF => {
            return_vec.extend(vec![0x4d]);

            let vec_u8: Bytes = vec![(x & 0xFF) as u8, (x >> 8 & 0xFF) as u8];

            return_vec.extend(vec_u8)
        }
        x if x <= 0xFFFFFFFF => {
            return_vec.extend(vec![0x4e]);

            // In little endian order
            let vec_u8: Bytes = vec![
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

#[derive(Clone)]
pub enum PushFlag {
    WitnessStandardPush,
    WitnessNonStandardPush,
    ScriptPush,
}

pub fn chunkify(data: &Bytes, flag: PushFlag) -> Vec<Bytes> {
    let mut chunks: Vec<Bytes> = Vec::<Bytes>::new();

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

    let mut covered = 0;

    for _ in 0..num_chunks {
        let mut to_cover = chunk_size;

        // Distribute leftovers by one
        if chunk_leftover > 0 {
            to_cover = to_cover + 1;
            chunk_leftover = chunk_leftover - 1;
        }

        let chunk: Bytes = data[covered..(covered + to_cover)].to_vec();
        covered = covered + to_cover;

        chunks.push(chunk);
    }
    chunks
}

pub fn encode_multi_push(data: &Bytes, flag: PushFlag) -> Bytes {
    let mut encoded: Bytes = Vec::<u8>::new();
    let chunks: Vec<Bytes> = chunkify(data, flag.clone());

    for chunk in chunks {
        match flag {
            // Use OP_PUSHDATA encoding for in-script witness pushes
            PushFlag::ScriptPush => encoded.extend(with_prefix_pushdata(&chunk)),
            // Use varint encoding for out-script witness pushes
            _ => encoded.extend(with_prefix_compact_size(&chunk)),
        }
    }

    encoded
}
