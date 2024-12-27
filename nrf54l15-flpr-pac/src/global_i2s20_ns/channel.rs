#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "CHANNEL")]
pub struct Channel {
    terminateonbuserror: Terminateonbuserror,
    buserroraddress: Buserroraddress,
}
impl Channel {
    #[doc = "0x00 - Description cluster: Terminate the transaction if a BUSERROR event is detected."]
    #[inline(always)]
    pub const fn terminateonbuserror(&self) -> &Terminateonbuserror {
        &self.terminateonbuserror
    }
    #[doc = "0x04 - Description cluster: Address of transaction that generated the last BUSERROR event."]
    #[inline(always)]
    pub const fn buserroraddress(&self) -> &Buserroraddress {
        &self.buserroraddress
    }
}
#[doc = "TERMINATEONBUSERROR (rw) register accessor: Description cluster: Terminate the transaction if a BUSERROR event is detected.\n\nYou can [`read`](crate::Reg::read) this register and get [`terminateonbuserror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`terminateonbuserror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@terminateonbuserror`]
module"]
#[doc(alias = "TERMINATEONBUSERROR")]
pub type Terminateonbuserror = crate::Reg<terminateonbuserror::TerminateonbuserrorSpec>;
#[doc = "Description cluster: Terminate the transaction if a BUSERROR event is detected."]
pub mod terminateonbuserror;
#[doc = "BUSERRORADDRESS (r) register accessor: Description cluster: Address of transaction that generated the last BUSERROR event.\n\nYou can [`read`](crate::Reg::read) this register and get [`buserroraddress::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buserroraddress`]
module"]
#[doc(alias = "BUSERRORADDRESS")]
pub type Buserroraddress = crate::Reg<buserroraddress::BuserroraddressSpec>;
#[doc = "Description cluster: Address of transaction that generated the last BUSERROR event."]
pub mod buserroraddress;
