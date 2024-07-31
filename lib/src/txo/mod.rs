pub mod channel;
pub mod connector;
pub mod lift;
pub mod projector;
pub mod vtxo;
pub mod payload;

pub trait TXOType {}

impl TXOType for channel::Channel {}
impl TXOType for connector::Connector {}
impl TXOType for lift::Lift {}
impl TXOType for projector::Projector {}
impl TXOType for vtxo::VTXO {}
impl TXOType for payload::Payload {}
