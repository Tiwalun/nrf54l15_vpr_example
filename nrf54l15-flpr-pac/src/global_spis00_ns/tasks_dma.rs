#[repr(C)]
#[doc = "Peripheral tasks."]
#[doc(alias = "TASKS_DMA")]
pub struct TasksDma {
    _reserved0: [u8; 0x08],
    rx: Rx,
}
impl TasksDma {
    #[doc = "0x08..0x28 - Peripheral tasks."]
    #[inline(always)]
    pub const fn rx(&self) -> &Rx {
        &self.rx
    }
}
#[doc = "Peripheral tasks."]
pub use self::rx::Rx;
#[doc = r"Cluster"]
#[doc = "Peripheral tasks."]
pub mod rx;
