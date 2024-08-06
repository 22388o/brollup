use bit_vec::BitVec;

pub trait CommonIndex {
    fn from_u8(common_index: &u8) -> BitVec;
    fn to_u8(&self) -> u8;
}

impl CommonIndex for BitVec {
    fn from_u8(common_index: &u8) -> BitVec {
        common_index_from_u8(common_index)
    }

    fn to_u8(&self) -> u8 {
        common_index_to_u8(self)
    }
}

fn common_index_from_u8(common_index: &u8) -> BitVec {
    let mut bit_vec = BitVec::new();

    // 3-bit common index encoding
    match common_index {
        0 => {
            // 0b000
            bit_vec.push(false);
            bit_vec.push(false);
            bit_vec.push(false);
        }
        1 => {
            // 0b001
            bit_vec.push(false);
            bit_vec.push(false);
            bit_vec.push(true);
        }
        2 => {
            // 0b010
            bit_vec.push(false);
            bit_vec.push(true);
            bit_vec.push(false);
        }
        3 => {
            // 0b011
            bit_vec.push(false);
            bit_vec.push(true);
            bit_vec.push(true);
        }
        4 => {
            // 0b100
            bit_vec.push(true);
            bit_vec.push(false);
            bit_vec.push(false);
        }
        5 => {
            // 0b101
            bit_vec.push(true);
            bit_vec.push(false);
            bit_vec.push(true);
        }
        6 => {
            // 0b110
            bit_vec.push(true);
            bit_vec.push(true);
            bit_vec.push(false);
        }
        7 => {
            // 0b111
            bit_vec.push(true);
            bit_vec.push(true);
            bit_vec.push(true);
        }
        _ => panic!("Common index is 3-bit-long."),
    }
    bit_vec
}

fn common_index_to_u8(common_index: &BitVec) -> u8 {
    let mut bit_vec = BitVec::new();
    bit_vec.extend(common_index);

    bit_vec.insert(0, false);
    bit_vec.insert(0, false);
    bit_vec.insert(0, false);
    bit_vec.insert(0, false);
    bit_vec.insert(0, false);

    bit_vec.to_bytes()[0]
}
