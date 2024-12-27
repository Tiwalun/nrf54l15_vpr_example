#[repr(C)]
#[doc = "Publish configuration for events"]
#[doc(alias = "PUBLISH_DMA")]
pub struct PublishDma {
    buserror: Buserror,
}
impl PublishDma {
    #[doc = "0x00 - Publish configuration for event DMA.BUSERROR"]
    #[inline(always)]
    pub const fn buserror(&self) -> &Buserror {
        &self.buserror
    }
}
#[doc = "BUSERROR (rw) register accessor: Publish configuration for event DMA.BUSERROR\n\nYou can [`read`](crate::Reg::read) this register and get [`buserror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buserror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buserror`]
module"]
#[doc(alias = "BUSERROR")]
pub type Buserror = crate::Reg<buserror::BuserrorSpec>;
#[doc = "Publish configuration for event DMA.BUSERROR"]
pub mod buserror;
