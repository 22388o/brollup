#![allow(dead_code)]

use musig2::secp256k1::XOnlyPublicKey;

type Account = XOnlyPublicKey;

pub enum Fallback {
    Bare,
    Virtual,
}

pub struct Transfer {
    from: Account,
    to: Account,
    fallback: Fallback,
}

impl Transfer {
    pub fn new_with_bare_fallback(from: Account, to: Account) -> Transfer {
        Transfer {
            from,
            to,
            fallback: Fallback::Bare,
        }
    }

    pub fn new_with_virtual_fallback(from: Account, to: Account) -> Transfer {
        Transfer {
            from,
            to,
            fallback: Fallback::Virtual,
        }
    }
}
