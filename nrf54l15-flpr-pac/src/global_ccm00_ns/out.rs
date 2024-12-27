#[repr(C)]
#[doc = "OUT EasyDMA channel"]
#[doc(alias = "OUT")]
pub struct Out {
    ptr: Ptr,
}
impl Out {
    #[doc = "0x00 - Output pointer Points to a job list containing encrypted CCM data structure in Encryption mode Points to a job list containing decrypted CCM data structure in Decryption mode"]
    #[inline(always)]
    pub const fn ptr(&self) -> &Ptr {
        &self.ptr
    }
}
#[doc = "PTR (rw) register accessor: Output pointer Points to a job list containing encrypted CCM data structure in Encryption mode Points to a job list containing decrypted CCM data structure in Decryption mode\n\nYou can [`read`](crate::Reg::read) this register and get [`ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr`]
module"]
#[doc(alias = "PTR")]
pub type Ptr = crate::Reg<ptr::PtrSpec>;
#[doc = "Output pointer Points to a job list containing encrypted CCM data structure in Encryption mode Points to a job list containing decrypted CCM data structure in Decryption mode"]
pub mod ptr;
