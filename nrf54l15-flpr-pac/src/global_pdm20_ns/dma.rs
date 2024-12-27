#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "DMA")]
pub struct Dma {
    terminateonbuserror: Terminateonbuserror,
    buserroraddress: Buserroraddress,
}
impl Dma {
    #[doc = "0x00 - Terminate the transaction if a BUSERROR event is detected."]
    #[inline(always)]
    pub const fn terminateonbuserror(&self) -> &Terminateonbuserror {
        &self.terminateonbuserror
    }
    #[doc = "0x04 - Address of transaction that generated the last BUSERROR event."]
    #[inline(always)]
    pub const fn buserroraddress(&self) -> &Buserroraddress {
        &self.buserroraddress
    }
}
#[doc = "TERMINATEONBUSERROR (rw) register accessor: Terminate the transaction if a BUSERROR event is detected.\n\nYou can [`read`](crate::Reg::read) this register and get [`terminateonbuserror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`terminateonbuserror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@terminateonbuserror`]
module"]
#[doc(alias = "TERMINATEONBUSERROR")]
pub type Terminateonbuserror = crate::Reg<terminateonbuserror::TerminateonbuserrorSpec>;
#[doc = "Terminate the transaction if a BUSERROR event is detected."]
pub mod terminateonbuserror;
#[doc = "BUSERRORADDRESS (rw) register accessor: Address of transaction that generated the last BUSERROR event.\n\nYou can [`read`](crate::Reg::read) this register and get [`buserroraddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buserroraddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buserroraddress`]
module"]
#[doc(alias = "BUSERRORADDRESS")]
pub type Buserroraddress = crate::Reg<buserroraddress::BuserroraddressSpec>;
#[doc = "Address of transaction that generated the last BUSERROR event."]
pub mod buserroraddress;
