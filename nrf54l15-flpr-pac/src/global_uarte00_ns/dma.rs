#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "DMA")]
pub struct Dma {
    rx: Rx,
    tx: Tx,
}
impl Dma {
    #[doc = "0x00..0x38 - Unspecified"]
    #[inline(always)]
    pub const fn rx(&self) -> &Rx {
        &self.rx
    }
    #[doc = "0x38..0x5c - Unspecified"]
    #[inline(always)]
    pub const fn tx(&self) -> &Tx {
        &self.tx
    }
}
#[doc = "Unspecified"]
pub use self::rx::Rx;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod rx;
#[doc = "Unspecified"]
pub use self::tx::Tx;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod tx;
