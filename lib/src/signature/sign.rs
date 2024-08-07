pub enum SignError {
    InvalidPrivateKey,
}
pub trait Sign {
    fn sign(&self, secret_key: [u8; 32], prev_state_hash: [u8; 32]) -> Result<[u8; 64], SignError>;
}
