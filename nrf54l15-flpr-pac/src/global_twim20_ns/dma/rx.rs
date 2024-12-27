#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "RX")]
pub struct Rx {
    _reserved0: [u8; 0x04],
    ptr: Ptr,
    maxcnt: Maxcnt,
    amount: Amount,
    _reserved3: [u8; 0x0c],
    terminateonbuserror: Terminateonbuserror,
    buserroraddress: Buserroraddress,
    match_: Match,
}
impl Rx {
    #[doc = "0x04 - RAM buffer start address"]
    #[inline(always)]
    pub const fn ptr(&self) -> &Ptr {
        &self.ptr
    }
    #[doc = "0x08 - Maximum number of bytes in channel buffer"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> &Maxcnt {
        &self.maxcnt
    }
    #[doc = "0x0c - Number of bytes transferred in the last transaction, updated after the END event. Also updated after each MATCH event."]
    #[inline(always)]
    pub const fn amount(&self) -> &Amount {
        &self.amount
    }
    #[doc = "0x1c - Terminate the transaction if a BUSERROR event is detected."]
    #[inline(always)]
    pub const fn terminateonbuserror(&self) -> &Terminateonbuserror {
        &self.terminateonbuserror
    }
    #[doc = "0x20 - Address of transaction that generated the last BUSERROR event."]
    #[inline(always)]
    pub const fn buserroraddress(&self) -> &Buserroraddress {
        &self.buserroraddress
    }
    #[doc = "0x24..0x38 - Registers to control the behavior of the pattern matcher engine"]
    #[inline(always)]
    pub const fn match_(&self) -> &Match {
        &self.match_
    }
}
#[doc = "PTR (rw) register accessor: RAM buffer start address\n\nYou can [`read`](crate::Reg::read) this register and get [`ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr`]
module"]
#[doc(alias = "PTR")]
pub type Ptr = crate::Reg<ptr::PtrSpec>;
#[doc = "RAM buffer start address"]
pub mod ptr;
#[doc = "MAXCNT (rw) register accessor: Maximum number of bytes in channel buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`maxcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxcnt`]
module"]
#[doc(alias = "MAXCNT")]
pub type Maxcnt = crate::Reg<maxcnt::MaxcntSpec>;
#[doc = "Maximum number of bytes in channel buffer"]
pub mod maxcnt;
#[doc = "AMOUNT (r) register accessor: Number of bytes transferred in the last transaction, updated after the END event. Also updated after each MATCH event.\n\nYou can [`read`](crate::Reg::read) this register and get [`amount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amount`]
module"]
#[doc(alias = "AMOUNT")]
pub type Amount = crate::Reg<amount::AmountSpec>;
#[doc = "Number of bytes transferred in the last transaction, updated after the END event. Also updated after each MATCH event."]
pub mod amount;
#[doc = "TERMINATEONBUSERROR (rw) register accessor: Terminate the transaction if a BUSERROR event is detected.\n\nYou can [`read`](crate::Reg::read) this register and get [`terminateonbuserror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`terminateonbuserror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@terminateonbuserror`]
module"]
#[doc(alias = "TERMINATEONBUSERROR")]
pub type Terminateonbuserror = crate::Reg<terminateonbuserror::TerminateonbuserrorSpec>;
#[doc = "Terminate the transaction if a BUSERROR event is detected."]
pub mod terminateonbuserror;
#[doc = "BUSERRORADDRESS (r) register accessor: Address of transaction that generated the last BUSERROR event.\n\nYou can [`read`](crate::Reg::read) this register and get [`buserroraddress::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buserroraddress`]
module"]
#[doc(alias = "BUSERRORADDRESS")]
pub type Buserroraddress = crate::Reg<buserroraddress::BuserroraddressSpec>;
#[doc = "Address of transaction that generated the last BUSERROR event."]
pub mod buserroraddress;
#[doc = "Registers to control the behavior of the pattern matcher engine"]
pub use self::match_::Match;
#[doc = r"Cluster"]
#[doc = "Registers to control the behavior of the pattern matcher engine"]
pub mod match_;
