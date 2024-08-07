#![allow(dead_code)]

use musig2::{
    errors::KeyAggError,
    secp256k1::{self, PublicKey, XOnlyPublicKey},
    KeyAggContext,
};

pub fn keys_to_key_agg_ctx(keys: &Vec<XOnlyPublicKey>) -> Result<KeyAggContext, KeyAggError> {
    // Lift keys
    let mut keys_lifted = Vec::<PublicKey>::new();

    for key in keys {
        keys_lifted.push(key.public_key(secp256k1::Parity::Even));
    }

    // Sort the keys
    keys_lifted.sort();

    let keys_iter = keys_lifted.into_iter();

    // Create Key Aggregation Context
    let key_agg_ctx: KeyAggContext = KeyAggContext::new(keys_iter)?;

    Ok(key_agg_ctx)
}
