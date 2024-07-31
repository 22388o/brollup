pub mod account;
pub mod contract;
pub mod maybe_common;
pub mod value;

pub trait ValType {}

impl ValType for account::Account {}
impl ValType for contract::Contract {}
impl ValType for value::ShortVal {}
impl ValType for value::LongVal {}