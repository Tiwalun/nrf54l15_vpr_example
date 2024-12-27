#[repr(C)]
#[doc = "Peripheral events."]
#[doc(alias = "EVENTS_DMA")]
pub struct EventsDma {
    rx: Rx,
    tx: Tx,
}
impl EventsDma {
    #[doc = "0x00..0x1c - Peripheral events."]
    #[inline(always)]
    pub const fn rx(&self) -> &Rx {
        &self.rx
    }
    #[doc = "0x1c..0x28 - Peripheral events."]
    #[inline(always)]
    pub const fn tx(&self) -> &Tx {
        &self.tx
    }
}
#[doc = "Peripheral events."]
pub use self::rx::Rx;
#[doc = r"Cluster"]
#[doc = "Peripheral events."]
pub mod rx;
#[doc = "Peripheral events."]
pub use self::tx::Tx;
#[doc = r"Cluster"]
#[doc = "Peripheral events."]
pub mod tx;
