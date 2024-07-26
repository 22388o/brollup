#![allow(dead_code)]

use crate::valtype::{account::Account, amount::ShortAmount, ToBitVec};

use bit_vec::BitVec;

pub enum Fallback {
    Bare,
    Virtual,
}

pub struct Transfer {
    from: Account,
    to: Account,
    amount: ShortAmount,
    fallback: Option<Fallback>,
}

impl Transfer {
    pub fn new(from: Account, to: Account, amount: u32) -> Transfer {
        Transfer {
            from,
            to,
            amount: ShortAmount(amount),
            fallback: None,
        }
    }

    pub fn set_fallback(&mut self, fallback: Fallback) {
        self.fallback = Some(fallback);
    }
}

impl ToBitVec for Transfer {
    fn to_bit_vec(&self) -> BitVec {
        let mut bit_vec = BitVec::new();

        // Transfer or call
        bit_vec.push(false);

        // Transfer
        bit_vec.push(false);

        // From
        bit_vec.extend(self.from.to_bit_vec());

        // To
        bit_vec.extend(self.to.to_bit_vec());

        // Value
        bit_vec.extend(self.amount.to_bit_vec());

        bit_vec
    }
}
