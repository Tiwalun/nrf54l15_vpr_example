#[repr(C)]
#[doc = "OUT EasyDMA channel"]
#[doc(alias = "OUT")]
pub struct Out {
    ptr: Ptr,
    amount: Amount,
}
impl Out {
    #[doc = "0x00 - Output pointer"]
    #[inline(always)]
    pub const fn ptr(&self) -> &Ptr {
        &self.ptr
    }
    #[doc = "0x04 - Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub const fn amount(&self) -> &Amount {
        &self.amount
    }
}
#[doc = "PTR (rw) register accessor: Output pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr`]
module"]
#[doc(alias = "PTR")]
pub type Ptr = crate::Reg<ptr::PtrSpec>;
#[doc = "Output pointer"]
pub mod ptr;
#[doc = "AMOUNT (r) register accessor: Number of bytes transferred in the last transaction\n\nYou can [`read`](crate::Reg::read) this register and get [`amount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amount`]
module"]
#[doc(alias = "AMOUNT")]
pub type Amount = crate::Reg<amount::AmountSpec>;
#[doc = "Number of bytes transferred in the last transaction"]
pub mod amount;
