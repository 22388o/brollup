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

    if data.len() == 1 && &data[0] <= &16 {
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
        return_vec
    } else {
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
}

#[derive(Clone)]
pub enum PushFlag {
    StandardWitnessPush,
    NonStandardWitnessPush,
    ScriptPush,
}

// Chunk bytes as evenly as possible
pub fn chunkify(data: &Bytes, flag: PushFlag) -> Vec<Bytes> {
    let mut chunks: Vec<Bytes> = Vec::<Bytes>::new();

    let chunk_size_max: usize = match flag {
        // https://github.com/bitcoin/bitcoin/blob/master/src/policy/policy.h#L45
        PushFlag::StandardWitnessPush => 80,
        // https://github.com/bitcoin/bitcoin/blob/master/src/script/script.h#L27
        PushFlag::NonStandardWitnessPush => 520,
        // https://github.com/bitcoin/bitcoin/blob/master/src/script/script.h#L27
        PushFlag::ScriptPush => 520,
    };

    let mut num_chunks = data.len() / chunk_size_max;

    if data.len() % chunk_size_max != 0 {
        num_chunks = num_chunks + 1;
    }

    if data.len() == 0 {
        num_chunks = 1;
    }

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

    // At the end, all bytes must have covered
    assert_eq!(data.len(), covered);

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

pub enum CSVFlag {
    CSVBlock,
    CSVHour,
    CSVDay,
    CSVWeek,
    CSVTwoWeeks,
    CSVMonth,
    CSVTwoMonths,
    CSVThreeMonths,
    CSVSixMonths,
    CSVYear,
    Days(u8),
}

pub fn days_to_bytes(days: u8, cscript_num: bool) -> Bytes {
    let blocks: u16 = days as u16 * 144;
    let mut vec = Vec::<u8>::new();

    if blocks <= 255 {
        // Single-byte
        vec.push(blocks as u8);
        if cscript_num == true && blocks > 127 {
            // CScriptNum
            vec.push(0x00);
        }
    } else {
        // Two-bytes
        vec.extend(vec![(blocks & 0xFF) as u8, (blocks >> 8 & 0xFF) as u8]);
        if cscript_num == true && blocks > 32767 {
            // CScriptNum
            vec.push(0x00);
        }
    }

    vec
}

fn pad_four(input: Bytes) -> Bytes {
    let input_len = input.len();
    let mut padded = input;

    match input_len {
        0 => padded.extend(vec![0x00, 0x00, 0x00, 0x00]),
        1 => padded.extend(vec![0x00, 0x00, 0x00]),
        2 => padded.extend(vec![0x00, 0x00]),
        3 => padded.extend(vec![0x00]),
        4 => (),
        _ => panic!(),
    }

    padded
}

pub fn to_n_sequence_encode(flag: CSVFlag) -> Bytes {
    let mut encoded = Vec::<u8>::new();

    match flag {
        CSVFlag::CSVBlock => encoded.extend(vec![0x01, 0x00, 0x00, 0x00]),
        CSVFlag::CSVHour => encoded.extend(vec![0x06, 0x00, 0x00, 0x00]),
        CSVFlag::CSVDay => encoded.extend(vec![0x90, 0x00, 0x00, 0x00]),
        CSVFlag::CSVWeek => encoded.extend(vec![0xf0, 0x03, 0x00, 0x00]),
        CSVFlag::CSVTwoWeeks => encoded.extend(vec![0xe0, 0x07, 0x00, 0x00]),
        CSVFlag::CSVMonth => encoded.extend(vec![0xe0, 0x10, 0x00, 0x00]),
        CSVFlag::CSVTwoMonths => encoded.extend(vec![0xc0, 0x21, 0x00, 0x00]),
        CSVFlag::CSVThreeMonths => encoded.extend(vec![0xa0, 0x32, 0x00, 0x00]),
        CSVFlag::CSVSixMonths => encoded.extend(vec![0x40, 0x65, 0x00, 0x00]),
        CSVFlag::CSVYear => encoded.extend(vec![0x50, 0xcd, 0x00, 0x00]),
        CSVFlag::Days(days) => encoded.extend(pad_four(days_to_bytes(days, false))),
    }

    encoded
}

pub fn to_csv_script_encode(flag: CSVFlag) -> Bytes {
    let mut encoded = Vec::<u8>::new();

    match flag {
        CSVFlag::CSVBlock => encoded.extend(vec![0x51]),
        CSVFlag::CSVHour => encoded.extend(vec![0x56]),
        CSVFlag::CSVDay => encoded.extend(vec![0x02, 0x90, 0x00]),
        CSVFlag::CSVWeek => encoded.extend(vec![0x02, 0xf0, 0x03]),
        CSVFlag::CSVTwoWeeks => encoded.extend(vec![0x02, 0xe0, 0x07]),
        CSVFlag::CSVMonth => encoded.extend(vec![0x02, 0xe0, 0x10]),
        CSVFlag::CSVTwoMonths => encoded.extend(vec![0x02, 0xc0, 0x21]),
        CSVFlag::CSVThreeMonths => encoded.extend(vec![0x02, 0xa0, 0x32]),
        CSVFlag::CSVSixMonths => encoded.extend(vec![0x02, 0x40, 0x65]),
        CSVFlag::CSVYear => encoded.extend(vec![0x03, 0x50, 0xcd, 0x00]),
        CSVFlag::Days(days) => encoded.extend(with_prefix_pushdata(&days_to_bytes(days, true))),
    }

    // OP_CHECKSEQUENCEVERIFY
    encoded.push(0xb2);

    // OP_DROP
    encoded.push(0x75);

    encoded
}
