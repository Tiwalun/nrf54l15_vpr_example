#[repr(C)]
#[doc = "Peripheral events."]
#[doc(alias = "EVENTS_DMA")]
pub struct EventsDma {
    seq: [Seq; 2],
}
impl EventsDma {
    #[doc = "0x00..0x18 - Peripheral events."]
    #[inline(always)]
    pub const fn seq(&self, n: usize) -> &Seq {
        &self.seq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x18 - Peripheral events."]
    #[inline(always)]
    pub fn seq_iter(&self) -> impl Iterator<Item = &Seq> {
        self.seq.iter()
    }
}
#[doc = "Peripheral events."]
pub use self::seq::Seq;
#[doc = r"Cluster"]
#[doc = "Peripheral events."]
pub mod seq;
