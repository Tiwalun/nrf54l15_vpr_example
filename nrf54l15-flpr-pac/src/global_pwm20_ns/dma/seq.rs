#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "SEQ")]
pub struct Seq {
    _reserved0: [u8; 0x04],
    ptr: Ptr,
    maxcnt: Maxcnt,
    amount: Amount,
    currentamount: Currentamount,
    _reserved4: [u8; 0x08],
    terminateonbuserror: Terminateonbuserror,
    buserroraddress: Buserroraddress,
}
impl Seq {
    #[doc = "0x04 - Description cluster: RAM buffer start address"]
    #[inline(always)]
    pub const fn ptr(&self) -> &Ptr {
        &self.ptr
    }
    #[doc = "0x08 - Description cluster: Maximum number of bytes in channel buffer"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> &Maxcnt {
        &self.maxcnt
    }
    #[doc = "0x0c - Description cluster: Number of bytes transferred in the last transaction, updated after the END event."]
    #[inline(always)]
    pub const fn amount(&self) -> &Amount {
        &self.amount
    }
    #[doc = "0x10 - Description cluster: Number of bytes transferred in the current transaction"]
    #[inline(always)]
    pub const fn currentamount(&self) -> &Currentamount {
        &self.currentamount
    }
    #[doc = "0x1c - Description cluster: Terminate the transaction if a BUSERROR event is detected."]
    #[inline(always)]
    pub const fn terminateonbuserror(&self) -> &Terminateonbuserror {
        &self.terminateonbuserror
    }
    #[doc = "0x20 - Description cluster: Address of transaction that generated the last BUSERROR event."]
    #[inline(always)]
    pub const fn buserroraddress(&self) -> &Buserroraddress {
        &self.buserroraddress
    }
}
#[doc = "PTR (rw) register accessor: Description cluster: RAM buffer start address\n\nYou can [`read`](crate::Reg::read) this register and get [`ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr`]
module"]
#[doc(alias = "PTR")]
pub type Ptr = crate::Reg<ptr::PtrSpec>;
#[doc = "Description cluster: RAM buffer start address"]
pub mod ptr;
#[doc = "MAXCNT (rw) register accessor: Description cluster: Maximum number of bytes in channel buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`maxcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxcnt`]
module"]
#[doc(alias = "MAXCNT")]
pub type Maxcnt = crate::Reg<maxcnt::MaxcntSpec>;
#[doc = "Description cluster: Maximum number of bytes in channel buffer"]
pub mod maxcnt;
#[doc = "AMOUNT (r) register accessor: Description cluster: Number of bytes transferred in the last transaction, updated after the END event.\n\nYou can [`read`](crate::Reg::read) this register and get [`amount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amount`]
module"]
#[doc(alias = "AMOUNT")]
pub type Amount = crate::Reg<amount::AmountSpec>;
#[doc = "Description cluster: Number of bytes transferred in the last transaction, updated after the END event."]
pub mod amount;
#[doc = "CURRENTAMOUNT (r) register accessor: Description cluster: Number of bytes transferred in the current transaction\n\nYou can [`read`](crate::Reg::read) this register and get [`currentamount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@currentamount`]
module"]
#[doc(alias = "CURRENTAMOUNT")]
pub type Currentamount = crate::Reg<currentamount::CurrentamountSpec>;
#[doc = "Description cluster: Number of bytes transferred in the current transaction"]
pub mod currentamount;
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
