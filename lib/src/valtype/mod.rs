pub mod account;
pub mod amount;

use bit_vec::BitVec;

pub trait ToBitVec {
    fn to_bit_vec(&self) -> BitVec;
}