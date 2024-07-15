use std::vec;

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

            // In little endian order
            let vec_u8: Vec<u8> = vec![(x >> 8 & 0xFF) as u8, (x & 0xFF) as u8];

            return_vec.extend(vec_u8)
        }
        x if x <= 0xFFFFFFFF => {
            return_vec.extend(vec![0x4e]);

            // In little endian order
            let vec_u8: Vec<u8> = vec![
                ((x >> 24) & 0xFF) as u8,
                ((x >> 16) & 0xFF) as u8,
                ((x >> 8) & 0xFF) as u8,
                (x & 0xFF) as u8,
            ];

            return_vec.extend(vec_u8)
        }
        _ => panic!(),
    }
    return_vec.extend(data);
    return_vec
}
