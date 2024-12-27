#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PERIPHACCERR")]
pub struct Periphaccerr {
    address: Address,
}
impl Periphaccerr {
    #[doc = "0x00 - Address of the transaction that caused first error."]
    #[inline(always)]
    pub const fn address(&self) -> &Address {
        &self.address
    }
}
#[doc = "ADDRESS (r) register accessor: Address of the transaction that caused first error.\n\nYou can [`read`](crate::Reg::read) this register and get [`address::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address`]
module"]
#[doc(alias = "ADDRESS")]
pub type Address = crate::Reg<address::AddressSpec>;
#[doc = "Address of the transaction that caused first error."]
pub mod address;
