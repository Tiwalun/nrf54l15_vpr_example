#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "REGION")]
pub struct Region {
    address: Address,
    config: Config,
}
impl Region {
    #[doc = "0x00 - Description cluster: Region address"]
    #[inline(always)]
    pub const fn address(&self) -> &Address {
        &self.address
    }
    #[doc = "0x04 - Description cluster: Region configuration"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
}
#[doc = "ADDRESS (rw) register accessor: Description cluster: Region address\n\nYou can [`read`](crate::Reg::read) this register and get [`address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address`]
module"]
#[doc(alias = "ADDRESS")]
pub type Address = crate::Reg<address::AddressSpec>;
#[doc = "Description cluster: Region address"]
pub mod address;
#[doc = "CONFIG (rw) register accessor: Description cluster: Region configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Description cluster: Region configuration"]
pub mod config;
