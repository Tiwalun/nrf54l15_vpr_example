#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "TRIMCNF")]
pub struct Trimcnf {
    addr: Addr,
    data: Data,
}
impl Trimcnf {
    #[doc = "0x00 - Description cluster: Address of the register which will be written"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x04 - Description cluster: Data to be written into the register"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
}
#[doc = "ADDR (r) register accessor: Description cluster: Address of the register which will be written\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Description cluster: Address of the register which will be written"]
pub mod addr;
#[doc = "DATA (r) register accessor: Description cluster: Data to be written into the register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Description cluster: Data to be written into the register"]
pub mod data;
