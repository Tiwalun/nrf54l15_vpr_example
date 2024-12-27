#[repr(C)]
#[doc = "RESULT EasyDMA channel"]
#[doc(alias = "RESULT")]
pub struct Result {
    _reserved0: [u8; 0x04],
    ptr: Ptr,
    maxcnt: Maxcnt,
    amount: Amount,
    currentamount: Currentamount,
}
impl Result {
    #[doc = "0x04 - Data pointer"]
    #[inline(always)]
    pub const fn ptr(&self) -> &Ptr {
        &self.ptr
    }
    #[doc = "0x08 - Maximum number of buffer bytes to transfer"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> &Maxcnt {
        &self.maxcnt
    }
    #[doc = "0x0c - Number of buffer bytes transferred since last START, updated after the END or STOPPED events"]
    #[inline(always)]
    pub const fn amount(&self) -> &Amount {
        &self.amount
    }
    #[doc = "0x10 - Number of buffer bytes transferred since last START, continuously updated"]
    #[inline(always)]
    pub const fn currentamount(&self) -> &Currentamount {
        &self.currentamount
    }
}
#[doc = "PTR (rw) register accessor: Data pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr`]
module"]
#[doc(alias = "PTR")]
pub type Ptr = crate::Reg<ptr::PtrSpec>;
#[doc = "Data pointer"]
pub mod ptr;
#[doc = "MAXCNT (rw) register accessor: Maximum number of buffer bytes to transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`maxcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxcnt`]
module"]
#[doc(alias = "MAXCNT")]
pub type Maxcnt = crate::Reg<maxcnt::MaxcntSpec>;
#[doc = "Maximum number of buffer bytes to transfer"]
pub mod maxcnt;
#[doc = "AMOUNT (r) register accessor: Number of buffer bytes transferred since last START, updated after the END or STOPPED events\n\nYou can [`read`](crate::Reg::read) this register and get [`amount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amount`]
module"]
#[doc(alias = "AMOUNT")]
pub type Amount = crate::Reg<amount::AmountSpec>;
#[doc = "Number of buffer bytes transferred since last START, updated after the END or STOPPED events"]
pub mod amount;
#[doc = "CURRENTAMOUNT (r) register accessor: Number of buffer bytes transferred since last START, continuously updated\n\nYou can [`read`](crate::Reg::read) this register and get [`currentamount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@currentamount`]
module"]
#[doc(alias = "CURRENTAMOUNT")]
pub type Currentamount = crate::Reg<currentamount::CurrentamountSpec>;
#[doc = "Number of buffer bytes transferred since last START, continuously updated"]
pub mod currentamount;
