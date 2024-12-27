#[repr(C)]
#[doc = "Publish configuration for events"]
#[doc(alias = "PUBLISH_DMA")]
pub struct PublishDma {
    rx: Rx,
    tx: Tx,
}
impl PublishDma {
    #[doc = "0x00..0x1c - Publish configuration for events"]
    #[inline(always)]
    pub const fn rx(&self) -> &Rx {
        &self.rx
    }
    #[doc = "0x1c..0x28 - Publish configuration for events"]
    #[inline(always)]
    pub const fn tx(&self) -> &Tx {
        &self.tx
    }
}
#[doc = "Publish configuration for events"]
pub use self::rx::Rx;
#[doc = r"Cluster"]
#[doc = "Publish configuration for events"]
pub mod rx;
#[doc = "Publish configuration for events"]
pub use self::tx::Tx;
#[doc = r"Cluster"]
#[doc = "Publish configuration for events"]
pub mod tx;
