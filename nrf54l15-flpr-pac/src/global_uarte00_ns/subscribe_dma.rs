#[repr(C)]
#[doc = "Subscribe configuration for tasks"]
#[doc(alias = "SUBSCRIBE_DMA")]
pub struct SubscribeDma {
    rx: Rx,
    tx: Tx,
}
impl SubscribeDma {
    #[doc = "0x00..0x28 - Subscribe configuration for tasks"]
    #[inline(always)]
    pub const fn rx(&self) -> &Rx {
        &self.rx
    }
    #[doc = "0x28..0x30 - Subscribe configuration for tasks"]
    #[inline(always)]
    pub const fn tx(&self) -> &Tx {
        &self.tx
    }
}
#[doc = "Subscribe configuration for tasks"]
pub use self::rx::Rx;
#[doc = r"Cluster"]
#[doc = "Subscribe configuration for tasks"]
pub mod rx;
#[doc = "Subscribe configuration for tasks"]
pub use self::tx::Tx;
#[doc = r"Cluster"]
#[doc = "Subscribe configuration for tasks"]
pub mod tx;
