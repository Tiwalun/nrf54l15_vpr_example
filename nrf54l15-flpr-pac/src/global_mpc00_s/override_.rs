#[repr(C)]
#[doc = "Special privilege tables"]
#[doc(alias = "OVERRIDE")]
pub struct Override {
    config: Config,
    startaddr: Startaddr,
    endaddr: Endaddr,
    _reserved3: [u8; 0x04],
    perm: Perm,
    permmask: Permmask,
    owner: Owner,
    masterport: Masterport,
}
impl Override {
    #[doc = "0x00 - Description cluster: Override region n Configuration register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x04 - Description cluster: Override region n Start Address"]
    #[inline(always)]
    pub const fn startaddr(&self) -> &Startaddr {
        &self.startaddr
    }
    #[doc = "0x08 - Description cluster: Override region n End Address"]
    #[inline(always)]
    pub const fn endaddr(&self) -> &Endaddr {
        &self.endaddr
    }
    #[doc = "0x10 - Description cluster: Permission settings for override region n"]
    #[inline(always)]
    pub const fn perm(&self) -> &Perm {
        &self.perm
    }
    #[doc = "0x14 - Description cluster: Masks permission setting fields from register OVERRIDE.PERM"]
    #[inline(always)]
    pub const fn permmask(&self) -> &Permmask {
        &self.permmask
    }
    #[doc = "0x18 - Description cluster: Owner for override region"]
    #[inline(always)]
    pub const fn owner(&self) -> &Owner {
        &self.owner
    }
    #[doc = "0x1c - Description cluster: Override region n local master enable"]
    #[inline(always)]
    pub const fn masterport(&self) -> &Masterport {
        &self.masterport
    }
}
#[doc = "CONFIG (rw) register accessor: Description cluster: Override region n Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Description cluster: Override region n Configuration register"]
pub mod config;
#[doc = "STARTADDR (rw) register accessor: Description cluster: Override region n Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`startaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`startaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@startaddr`]
module"]
#[doc(alias = "STARTADDR")]
pub type Startaddr = crate::Reg<startaddr::StartaddrSpec>;
#[doc = "Description cluster: Override region n Start Address"]
pub mod startaddr;
#[doc = "ENDADDR (rw) register accessor: Description cluster: Override region n End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`endaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endaddr`]
module"]
#[doc(alias = "ENDADDR")]
pub type Endaddr = crate::Reg<endaddr::EndaddrSpec>;
#[doc = "Description cluster: Override region n End Address"]
pub mod endaddr;
#[doc = "PERM (rw) register accessor: Description cluster: Permission settings for override region n\n\nYou can [`read`](crate::Reg::read) this register and get [`perm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perm`]
module"]
#[doc(alias = "PERM")]
pub type Perm = crate::Reg<perm::PermSpec>;
#[doc = "Description cluster: Permission settings for override region n"]
pub mod perm;
#[doc = "PERMMASK (rw) register accessor: Description cluster: Masks permission setting fields from register OVERRIDE.PERM\n\nYou can [`read`](crate::Reg::read) this register and get [`permmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`permmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@permmask`]
module"]
#[doc(alias = "PERMMASK")]
pub type Permmask = crate::Reg<permmask::PermmaskSpec>;
#[doc = "Description cluster: Masks permission setting fields from register OVERRIDE.PERM"]
pub mod permmask;
#[doc = "OWNER (rw) register accessor: Description cluster: Owner for override region\n\nYou can [`read`](crate::Reg::read) this register and get [`owner::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`owner::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owner`]
module"]
#[doc(alias = "OWNER")]
pub type Owner = crate::Reg<owner::OwnerSpec>;
#[doc = "Description cluster: Owner for override region"]
pub mod owner;
#[doc = "MASTERPORT (rw) register accessor: Description cluster: Override region n local master enable\n\nYou can [`read`](crate::Reg::read) this register and get [`masterport::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`masterport::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@masterport`]
module"]
#[doc(alias = "MASTERPORT")]
pub type Masterport = crate::Reg<masterport::MasterportSpec>;
#[doc = "Description cluster: Override region n local master enable"]
pub mod masterport;
