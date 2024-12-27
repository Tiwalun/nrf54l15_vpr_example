#[repr(C)]
#[doc = "Peripheral events."]
#[doc(alias = "EVENTS_DMA")]
pub struct EventsDma {
    buserror: Buserror,
}
impl EventsDma {
    #[doc = "0x00 - This event is generated if an error occurs during the bus transfer."]
    #[inline(always)]
    pub const fn buserror(&self) -> &Buserror {
        &self.buserror
    }
}
#[doc = "BUSERROR (rw) register accessor: This event is generated if an error occurs during the bus transfer.\n\nYou can [`read`](crate::Reg::read) this register and get [`buserror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buserror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buserror`]
module"]
#[doc(alias = "BUSERROR")]
pub type Buserror = crate::Reg<buserror::BuserrorSpec>;
#[doc = "This event is generated if an error occurs during the bus transfer."]
pub mod buserror;
