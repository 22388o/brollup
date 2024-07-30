#![allow(dead_code)]

use crate::valtype::{account::Account, value::ShortVal, CompactPayloadEncoding};

use bit_vec::BitVec;

pub enum Fallback {
    Bare,
    Virtual,
}

pub struct Transfer {
    from: Account,
    to: Account,
    amount: ShortVal,
    fallback: Option<Fallback>,
}

impl Transfer {
    pub fn new(from: Account, to: Account, amount: u32) -> Transfer {
        Transfer {
            from,
            to,
            amount: ShortVal(amount),
            fallback: None,
        }
    }

    pub fn set_fallback(&mut self, fallback: Fallback) {
        self.fallback = Some(fallback);
    }
}

impl CompactPayloadEncoding for Transfer {
    fn to_cpe(&self) -> BitVec {
        let mut bit_vec = BitVec::new();

        // Transfer or call
        bit_vec.push(false);

        // Transfer
        bit_vec.push(false);

        // From
        bit_vec.extend(self.from.to_cpe());

        // To
        bit_vec.extend(self.to.to_cpe());

        // Value
        bit_vec.extend(self.amount.to_cpe());

        bit_vec
    }
}
