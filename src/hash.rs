use ripemd::Ripemd160;
use sha2::Digest as _;
use sha2::Sha256;
use sha2::Sha512;

type Bytes = Vec<u8>;

pub fn sha_256(data: impl AsRef<[u8]>) -> [u8; 32] {
    let result: [u8; 32] = Sha256::new().chain_update(&data).finalize().into();
    result
}

pub fn hash_256(data: impl AsRef<[u8]>) -> [u8; 32] {
    let result: [u8; 32] = sha_256(sha_256(data));
    result
}

pub fn sha_512(data: impl AsRef<[u8]>) -> [u8; 64] {
    let result: [u8; 64] = Sha512::new().chain_update(&data).finalize().into();
    result
}

pub fn hash_512(data: impl AsRef<[u8]>) -> [u8; 64] {
    let result: [u8; 64] = sha_512(sha_512(data));
    result
}

pub fn ripemd_160(data: impl AsRef<[u8]>) -> [u8; 20] {
    let mut ripemd_160_hash = Ripemd160::new();
    ripemd_160_hash.update(data);

    let result: [u8; 20] = ripemd_160_hash.finalize().into();
    result
}

pub fn hash_160(data: impl AsRef<[u8]>) -> [u8; 20] {
    let result = ripemd_160(sha_256(data));
    result
}

pub enum HashTag {
    TapLeafTag,
    TapBranchTag,
    TapTweakTag,
    CustomTag(String),
}

pub fn tagged_hash(data: Bytes, tag: HashTag) -> [u8; 32] {
    let mut full = Vec::<u8>::new();

    let tag_digest = match tag {
        HashTag::TapLeafTag => Sha256::digest("TapLeaf"),
        HashTag::TapBranchTag => Sha256::digest("TapBranch"),
        HashTag::TapTweakTag => Sha256::digest("TapTweak"),
        HashTag::CustomTag(tag) => Sha256::digest(tag),
    };

    full.extend(tag_digest);
    full.extend(tag_digest);
    full.extend(data);

    sha_256(full)
}
