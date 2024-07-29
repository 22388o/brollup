pub mod account;
pub mod value;
pub mod contract;

use bit_vec::BitVec;

pub trait ToBitVec {
    fn to_bit_vec(&self) -> BitVec;
}