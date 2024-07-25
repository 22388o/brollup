#![allow(dead_code)]

use bit_vec::BitVec;
use musig2::secp256k1::XOnlyPublicKey;

use crate::value::amount::{ShortAmount, ToBitVec};

type Account = XOnlyPublicKey;

pub enum Fallback {
    Bare,
    Virtual,
}

pub struct Transfer {
    from: Account,
    to: Account,
    amount: ShortAmount,
    fallback: Fallback,
}

impl Transfer {
    pub fn new_with_bare_fallback(from: Account, to: Account, amount: u32) -> Transfer {
        Transfer {
            from,
            to,
            amount: ShortAmount(amount),
            fallback: Fallback::Bare,
        }
    }

    pub fn new_with_virtual_fallback(from: Account, to: Account, amount: u32) -> Transfer {
        Transfer {
            from,
            to,
            amount: ShortAmount(amount),
            fallback: Fallback::Virtual,
        }
    }

    pub fn to_non_compact_bit_vec(&self) -> BitVec {
        let mut bit_vec = BitVec::new();

        // Transfer or call
        bit_vec.push(false);

        // Transfer
        bit_vec.push(false);

        // Fallback type
        match self.fallback {
            Fallback::Bare => bit_vec.push(false),
            Fallback::Virtual => bit_vec.push(true),
        }

        // Non-compact from
        bit_vec.push(false);
        let from_key_bytes = self.from.serialize().to_vec();
        let from_key_bits = BitVec::from_bytes(&from_key_bytes);
        bit_vec.extend(from_key_bits);

        // Non-compact to
        bit_vec.push(false);
        let to_key_bytes = self.to.serialize().to_vec();
        let to_key_bits = BitVec::from_bytes(&to_key_bytes);
        bit_vec.extend(to_key_bits);

        // Value
        bit_vec.extend(self.amount.to_bit_vec());

        bit_vec
    }
}
