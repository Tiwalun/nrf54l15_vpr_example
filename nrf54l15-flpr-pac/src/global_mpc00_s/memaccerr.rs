#[repr(C)]
#[doc = "Memory Access Error status registers"]
#[doc(alias = "MEMACCERR")]
pub struct Memaccerr {
    address: Address,
    info: Info,
}
impl Memaccerr {
    #[doc = "0x00 - Target Address of Memory Access Error. Register content won't be changed as long as MEMACCERR event is active."]
    #[inline(always)]
    pub const fn address(&self) -> &Address {
        &self.address
    }
    #[doc = "0x04 - Access information for the transaction that triggered a memory access error. Register content won't be changed as long as MEMACCERR event is active."]
    #[inline(always)]
    pub const fn info(&self) -> &Info {
        &self.info
    }
}
#[doc = "ADDRESS (r) register accessor: Target Address of Memory Access Error. Register content won't be changed as long as MEMACCERR event is active.\n\nYou can [`read`](crate::Reg::read) this register and get [`address::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address`]
module"]
#[doc(alias = "ADDRESS")]
pub type Address = crate::Reg<address::AddressSpec>;
#[doc = "Target Address of Memory Access Error. Register content won't be changed as long as MEMACCERR event is active."]
pub mod address;
#[doc = "INFO (r) register accessor: Access information for the transaction that triggered a memory access error. Register content won't be changed as long as MEMACCERR event is active.\n\nYou can [`read`](crate::Reg::read) this register and get [`info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@info`]
module"]
#[doc(alias = "INFO")]
pub type Info = crate::Reg<info::InfoSpec>;
#[doc = "Access information for the transaction that triggered a memory access error. Register content won't be changed as long as MEMACCERR event is active."]
pub mod info;
