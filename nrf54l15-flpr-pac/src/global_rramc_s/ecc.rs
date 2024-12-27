#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "ECC")]
pub struct Ecc {
    erroraddr: Erroraddr,
}
impl Ecc {
    #[doc = "0x00 - Address of the first ECC error that could not be corrected"]
    #[inline(always)]
    pub const fn erroraddr(&self) -> &Erroraddr {
        &self.erroraddr
    }
}
#[doc = "ERRORADDR (r) register accessor: Address of the first ECC error that could not be corrected\n\nYou can [`read`](crate::Reg::read) this register and get [`erroraddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erroraddr`]
module"]
#[doc(alias = "ERRORADDR")]
pub type Erroraddr = crate::Reg<erroraddr::ErroraddrSpec>;
#[doc = "Address of the first ECC error that could not be corrected"]
pub mod erroraddr;
