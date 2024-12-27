#[repr(C)]
#[doc = "Peripheral tasks."]
#[doc(alias = "TASKS_DMA")]
pub struct TasksDma {
    rx: Rx,
    tx: Tx,
}
impl TasksDma {
    #[doc = "0x00..0x28 - Peripheral tasks."]
    #[inline(always)]
    pub const fn rx(&self) -> &Rx {
        &self.rx
    }
    #[doc = "0x28..0x30 - Peripheral tasks."]
    #[inline(always)]
    pub const fn tx(&self) -> &Tx {
        &self.tx
    }
}
#[doc = "Peripheral tasks."]
pub use self::rx::Rx;
#[doc = r"Cluster"]
#[doc = "Peripheral tasks."]
pub mod rx;
#[doc = "Peripheral tasks."]
pub use self::tx::Tx;
#[doc = r"Cluster"]
#[doc = "Peripheral tasks."]
pub mod tx;
