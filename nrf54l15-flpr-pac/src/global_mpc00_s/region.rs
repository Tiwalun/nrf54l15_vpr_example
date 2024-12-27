#[repr(C)]
#[doc = "Memory region to slave decoding table"]
#[doc(alias = "REGION")]
pub struct Region {
    config: Config,
    startaddr: Startaddr,
    addrmask: Addrmask,
    masterport: Masterport,
}
impl Region {
    #[doc = "0x00 - Description cluster: Slave region n Configuration register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x04 - Description cluster: Region n start address"]
    #[inline(always)]
    pub const fn startaddr(&self) -> &Startaddr {
        &self.startaddr
    }
    #[doc = "0x08 - Description cluster: Select which bits of the incoming address are compared against the STARTADDR"]
    #[inline(always)]
    pub const fn addrmask(&self) -> &Addrmask {
        &self.addrmask
    }
    #[doc = "0x0c - Description cluster: Region n local master enable"]
    #[inline(always)]
    pub const fn masterport(&self) -> &Masterport {
        &self.masterport
    }
}
#[doc = "CONFIG (rw) register accessor: Description cluster: Slave region n Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Description cluster: Slave region n Configuration register"]
pub mod config;
#[doc = "STARTADDR (rw) register accessor: Description cluster: Region n start address\n\nYou can [`read`](crate::Reg::read) this register and get [`startaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`startaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@startaddr`]
module"]
#[doc(alias = "STARTADDR")]
pub type Startaddr = crate::Reg<startaddr::StartaddrSpec>;
#[doc = "Description cluster: Region n start address"]
pub mod startaddr;
#[doc = "ADDRMASK (rw) register accessor: Description cluster: Select which bits of the incoming address are compared against the STARTADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`addrmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrmask`]
module"]
#[doc(alias = "ADDRMASK")]
pub type Addrmask = crate::Reg<addrmask::AddrmaskSpec>;
#[doc = "Description cluster: Select which bits of the incoming address are compared against the STARTADDR"]
pub mod addrmask;
#[doc = "MASTERPORT (rw) register accessor: Description cluster: Region n local master enable\n\nYou can [`read`](crate::Reg::read) this register and get [`masterport::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`masterport::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@masterport`]
module"]
#[doc(alias = "MASTERPORT")]
pub type Masterport = crate::Reg<masterport::MasterportSpec>;
#[doc = "Description cluster: Region n local master enable"]
pub mod masterport;
