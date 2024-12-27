#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "CRACEN")]
pub struct Cracen {
    _reserved0: [u8; 0x0180],
    seed: Seed,
}
impl Cracen {
    #[doc = "0x180 - Configuration for CRACEN SEED"]
    #[inline(always)]
    pub const fn seed(&self) -> &Seed {
        &self.seed
    }
}
#[doc = "SEED (rw) register accessor: Configuration for CRACEN SEED\n\nYou can [`read`](crate::Reg::read) this register and get [`seed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seed`]
module"]
#[doc(alias = "SEED")]
pub type Seed = crate::Reg<seed::SeedSpec>;
#[doc = "Configuration for CRACEN SEED"]
pub mod seed;
