use ripemd::Ripemd160;
use sha2::Digest as _;
use sha2::Sha256;
use sha2::Sha512;

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
