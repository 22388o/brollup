#![allow(dead_code)]

use bit_vec::BitVec;

use crate::{
    serialize::cpe::CompactPayloadEncoding,
    valtype::{account::Account, maybe_common::MaybeCommon, value::ShortVal},
};

#[derive(Clone, Copy)]
pub struct Transfer {
    from: Account,
    to: MaybeCommon<Account>,
    amount: MaybeCommon<ShortVal>,
}

impl Transfer {
    pub fn new(from: Account, to: MaybeCommon<Account>, amount: MaybeCommon<ShortVal>) -> Transfer {
        Transfer { from, to, amount }
    }

    pub fn new_uncommon(from: Account, to: Account, amount: ShortVal) -> Transfer {
        Transfer {
            from,
            to: MaybeCommon::Uncommon(to),
            amount: MaybeCommon::Uncommon(amount),
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
