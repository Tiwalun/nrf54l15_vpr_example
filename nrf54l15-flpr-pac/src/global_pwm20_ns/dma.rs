#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "DMA")]
pub struct Dma {
    seq: [Seq; 2],
}
impl Dma {
    #[doc = "0x00..0x48 - Unspecified"]
    #[inline(always)]
    pub const fn seq(&self, n: usize) -> &Seq {
        &self.seq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x48 - Unspecified"]
    #[inline(always)]
    pub fn seq_iter(&self) -> impl Iterator<Item = &Seq> {
        self.seq.iter()
    }
}
#[doc = "Unspecified"]
pub use self::seq::Seq;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod seq;
