#![allow(dead_code)]

use crate::valtype::{
    account::Account, maybe_common::MaybeCommon, value::ShortVal, cpe::CompactPayloadEncoding,
};

use bit_vec::BitVec;

#[derive(Clone, Copy)]
pub struct Transfer {
    from: Account,
    to: MaybeCommon<Account>,
    amount: MaybeCommon<ShortVal>,
}

impl Transfer {
    pub fn new_uncommon(from: Account, to: Account, amount: u32) -> Transfer {
        Transfer {
            from,
            to: MaybeCommon::Uncommon(to),
            amount: MaybeCommon::Uncommon(ShortVal(amount)),
        }
    }

    pub fn new_common(from: Account, to: (Account, u8), amount: (u32, u8)) -> Transfer {
        Transfer {
            from,
            to: MaybeCommon::Common(to.0, to.1),
            amount: MaybeCommon::Common(ShortVal(amount.0), amount.1),
        }
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

        // Amount
        bit_vec.extend(self.amount.to_cpe());

        bit_vec
    }
}
