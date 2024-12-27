#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    vregm: Vregm,
    _reserved1: [u8; 0xfc],
    systemoff: Systemoff,
    _reserved2: [u8; 0x2c],
    pofcon: Pofcon,
    pofstat: Pofstat,
    _reserved4: [u8; 0xc8],
    vregmain: Vregmain,
}
impl RegisterBlock {
    #[doc = "0x400 - Register interface for the medium voltage regulator"]
    #[inline(always)]
    pub const fn vregm(&self) -> &Vregm {
        &self.vregm
    }
    #[doc = "0x500 - System OFF register"]
    #[inline(always)]
    pub const fn systemoff(&self) -> &Systemoff {
        &self.systemoff
    }
    #[doc = "0x530 - Power-fail comparator configuration"]
    #[inline(always)]
    pub const fn pofcon(&self) -> &Pofcon {
        &self.pofcon
    }
    #[doc = "0x534 - Power-fail comparator status register"]
    #[inline(always)]
    pub const fn pofstat(&self) -> &Pofstat {
        &self.pofstat
    }
    #[doc = "0x600..0x608 - Register interface for main voltage regulator."]
    #[inline(always)]
    pub const fn vregmain(&self) -> &Vregmain {
        &self.vregmain
    }
}
#[doc = "Register interface for the medium voltage regulator"]
pub use self::vregm::Vregm;
#[doc = r"Cluster"]
#[doc = "Register interface for the medium voltage regulator"]
pub mod vregm;
#[doc = "SYSTEMOFF (w) register accessor: System OFF register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systemoff::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systemoff`]
module"]
#[doc(alias = "SYSTEMOFF")]
pub type Systemoff = crate::Reg<systemoff::SystemoffSpec>;
#[doc = "System OFF register"]
pub mod systemoff;
#[doc = "POFCON (rw) register accessor: Power-fail comparator configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pofcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pofcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pofcon`]
module"]
#[doc(alias = "POFCON")]
pub type Pofcon = crate::Reg<pofcon::PofconSpec>;
#[doc = "Power-fail comparator configuration"]
pub mod pofcon;
#[doc = "POFSTAT (r) register accessor: Power-fail comparator status register\n\nYou can [`read`](crate::Reg::read) this register and get [`pofstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pofstat`]
module"]
#[doc(alias = "POFSTAT")]
pub type Pofstat = crate::Reg<pofstat::PofstatSpec>;
#[doc = "Power-fail comparator status register"]
pub mod pofstat;
#[doc = "Register interface for main voltage regulator."]
pub use self::vregmain::Vregmain;
#[doc = r"Cluster"]
#[doc = "Register interface for main voltage regulator."]
pub mod vregmain;
