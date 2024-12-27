#[repr(C)]
#[doc = "Peripheral tasks."]
#[doc(alias = "TASKS_DMA")]
pub struct TasksDma {
    seq: [Seq; 2],
}
impl TasksDma {
    #[doc = "0x00..0x10 - Peripheral tasks."]
    #[inline(always)]
    pub const fn seq(&self, n: usize) -> &Seq {
        &self.seq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - Peripheral tasks."]
    #[inline(always)]
    pub fn seq_iter(&self) -> impl Iterator<Item = &Seq> {
        self.seq.iter()
    }
}
#[doc = "Peripheral tasks."]
pub use self::seq::Seq;
#[doc = r"Cluster"]
#[doc = "Peripheral tasks."]
pub mod seq;
