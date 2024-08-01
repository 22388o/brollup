type Bytes = Vec<u8>;

pub fn u32_to_bytes(value: u32) -> Bytes {
    let vec_u8: Bytes = vec![
        (value & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        ((value >> 16) & 0xFF) as u8,
        ((value >> 24) & 0xFF) as u8,
    ];
    vec_u8
}

// Not tested.
pub fn bytes_to_u32(bytes: Bytes) -> u32 {
    let vec_u32: Vec<u32> = bytes
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect();
    vec_u32[0]
}