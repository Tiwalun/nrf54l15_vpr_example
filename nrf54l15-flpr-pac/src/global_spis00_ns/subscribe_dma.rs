#[repr(C)]
#[doc = "Subscribe configuration for tasks"]
#[doc(alias = "SUBSCRIBE_DMA")]
pub struct SubscribeDma {
    _reserved0: [u8; 0x08],
    rx: Rx,
}
impl SubscribeDma {
    #[doc = "0x08..0x28 - Subscribe configuration for tasks"]
    #[inline(always)]
    pub const fn rx(&self) -> &Rx {
        &self.rx
    }
}
#[doc = "Subscribe configuration for tasks"]
pub use self::rx::Rx;
#[doc = r"Cluster"]
#[doc = "Subscribe configuration for tasks"]
pub mod rx;
