use super::prefix::Prefix;

type Bytes = Vec<u8>;

#[derive(Clone)]
pub enum PushFlag {
    StandardWitnessPush,
    NonStandardWitnessPush,
    ScriptPush,
}

// Put data in chunks 520/80 byte-long each
pub fn chunkify(data: &Bytes, flag: PushFlag) -> Vec<Bytes> {
    let mut chunks: Vec<Bytes> = Vec::<Bytes>::new();

    let data_len = data.len();

    let chunk_size: usize = match flag {
        // https://github.com/bitcoin/bitcoin/blob/master/src/policy/policy.h#L45
        PushFlag::StandardWitnessPush => 80,
        // https://github.com/bitcoin/bitcoin/blob/master/src/script/script.h#L27
        PushFlag::NonStandardWitnessPush => 520,
        // https://github.com/bitcoin/bitcoin/blob/master/src/script/script.h#L27
        PushFlag::ScriptPush => 520,
    };

    let num_chunks = match data_len % chunk_size {
        x if x == 0 => data_len / chunk_size,
        x if x != 0 => data_len / chunk_size + 1,
        _ => panic!(),
    };

    let mut covered = 0;

    for i in 0..num_chunks {
        let is_last: bool = i + 1 == num_chunks;

        let to_cover = match is_last {
            false => chunk_size,
            true => {
                if data_len % chunk_size == 0 {
                    chunk_size
                } else {
                    data_len % chunk_size
                }
            }
        };

        let chunk: Bytes = data[covered..(covered + to_cover)].to_vec();
        chunks.push(chunk);

        covered = covered + to_cover;
    }

    // At the end, all bytes must have covered
    assert_eq!(data_len, covered);

    chunks
}

pub fn encode_multi_push(data: &Bytes, flag: PushFlag) -> Bytes {
    let mut encoded: Bytes = Vec::<u8>::new();
    let chunks: Vec<Bytes> = chunkify(data, flag.clone());

    for chunk in chunks {
        match flag {
            // Use OP_PUSHDATA encoding for in-script witness pushes
            PushFlag::ScriptPush => encoded.extend(&chunk.prefix_pushdata()),
            // Use varint encoding for out-script witness pushes
            _ => encoded.extend(&chunk.prefix_compact_size()),
        }
    }

    encoded
}
