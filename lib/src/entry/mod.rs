pub mod transfer;

pub trait EntryType {}

impl EntryType for transfer::Transfer {}