use musig2::secp256k1;

use crate::taproot::{TapRoot, P2TR};
type Bytes = Vec<u8>;

pub trait TXOType {}

impl TXOType for super::channel::Channel {}
impl TXOType for super::connector::Connector {}
impl TXOType for super::lift::Lift {}
impl TXOType for super::projector::Projector {}
impl TXOType for super::vtxo::VTXO {}
impl TXOType for super::payload::Payload {}

pub struct TXO<T: TXOType>(pub T);

impl<T: TXOType + P2TR> P2TR for TXO<T> {
    fn taproot(&self) -> Result<TapRoot, secp256k1::Error> {
        self.0.taproot()
    }

    fn spk(&self) -> Result<Bytes, secp256k1::Error> {
        self.taproot()?.spk()
    }
}
