#![allow(dead_code)]

use ripemd::Ripemd160;
use sha2::Digest as _;
use sha2::Sha256;
use sha2::Sha512;

type Bytes = Vec<u8>;

pub fn sha_256(data: impl AsRef<[u8]>) -> [u8; 32] {
    let hash: [u8; 32] = Sha256::new().chain_update(&data).finalize().into();
    hash
}

pub fn hash_256(data: impl AsRef<[u8]>) -> [u8; 32] {
    let hash: [u8; 32] = sha_256(sha_256(data));
    hash
}

pub fn sha_512(data: impl AsRef<[u8]>) -> [u8; 64] {
    let hash: [u8; 64] = Sha512::new().chain_update(&data).finalize().into();
    hash
}

pub fn ripemd_160(data: impl AsRef<[u8]>) -> [u8; 20] {
    let mut ripemd_160_hash = Ripemd160::new();
    ripemd_160_hash.update(data);

    let hash: [u8; 20] = ripemd_160_hash.finalize().into();
    hash
}

pub fn hash_160(data: impl AsRef<[u8]>) -> [u8; 20] {
    let hash: [u8; 20] = ripemd_160(sha_256(data));
    hash
}

pub enum HashTag {
    TapLeaf,
    TapBranch,
    TapTweak,
    SighashTransfer,
    SighashCall,
    SighashLiftup,
    SighashLiftdown,
    SighashRecharge,
    SighashReserved,
    CustomTag(String),
}

pub fn tagged_hash(data: Bytes, tag: HashTag) -> [u8; 32] {
    let mut preimage = Vec::<u8>::new();

    let tag_digest = match tag {
        HashTag::TapLeaf => Sha256::digest("TapLeaf"),
        HashTag::TapBranch => Sha256::digest("TapBranch"),
        HashTag::TapTweak => Sha256::digest("TapTweak"),
        HashTag::SighashTransfer => Sha256::digest("SighashTransfer"),
        HashTag::SighashCall => Sha256::digest("SighashCall"),
        HashTag::SighashLiftup => Sha256::digest("SighashLiftup"),
        HashTag::SighashLiftdown => Sha256::digest("SighashLiftdown"),
        HashTag::SighashRecharge => Sha256::digest("SighashRecharge"),
        HashTag::SighashReserved => Sha256::digest("SighashReserved"),
        HashTag::CustomTag(tag) => Sha256::digest(tag),
    };

    preimage.extend(tag_digest);
    preimage.extend(tag_digest);
    preimage.extend(data);

    sha_256(preimage)
}
