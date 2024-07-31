#![allow(dead_code)]

use crate::valtype::{
    account::Account, maybe_common::MaybeCommon, value::ShortVal,
};

#[derive(Clone, Copy)]
pub struct Transfer {
    pub from: Account,
    pub to: MaybeCommon<Account>,
    pub amount: MaybeCommon<ShortVal>,
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

