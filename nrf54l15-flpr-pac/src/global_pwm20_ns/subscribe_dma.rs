#[repr(C)]
#[doc = "Subscribe configuration for tasks"]
#[doc(alias = "SUBSCRIBE_DMA")]
pub struct SubscribeDma {
    seq: [Seq; 2],
}
impl SubscribeDma {
    #[doc = "0x00..0x10 - Subscribe configuration for tasks"]
    #[inline(always)]
    pub const fn seq(&self, n: usize) -> &Seq {
        &self.seq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - Subscribe configuration for tasks"]
    #[inline(always)]
    pub fn seq_iter(&self) -> impl Iterator<Item = &Seq> {
        self.seq.iter()
    }
}
#[doc = "Subscribe configuration for tasks"]
pub use self::seq::Seq;
#[doc = r"Cluster"]
#[doc = "Subscribe configuration for tasks"]
pub mod seq;
