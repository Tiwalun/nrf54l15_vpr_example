#[repr(C)]
#[doc = "Publish configuration for events"]
#[doc(alias = "PUBLISH_DMA")]
pub struct PublishDma {
    seq: [Seq; 2],
}
impl PublishDma {
    #[doc = "0x00..0x18 - Publish configuration for events"]
    #[inline(always)]
    pub const fn seq(&self, n: usize) -> &Seq {
        &self.seq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x18 - Publish configuration for events"]
    #[inline(always)]
    pub fn seq_iter(&self) -> impl Iterator<Item = &Seq> {
        self.seq.iter()
    }
}
#[doc = "Publish configuration for events"]
pub use self::seq::Seq;
#[doc = r"Cluster"]
#[doc = "Publish configuration for events"]
pub mod seq;
