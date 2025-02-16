#[repr(C)]
#[doc = "IN EasyDMA channel"]
#[doc(alias = "IN")]
pub struct In {
    ptr: Ptr,
}
impl In {
    #[doc = "0x00 - Input pointer"]
    #[inline(always)]
    pub const fn ptr(&self) -> &Ptr {
        &self.ptr
    }
}
#[doc = "PTR (rw) register accessor: Input pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr`]
module"]
#[doc(alias = "PTR")]
pub type Ptr = crate::Reg<ptr::PtrSpec>;
#[doc = "Input pointer"]
pub mod ptr;
