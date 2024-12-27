#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    syspwrupreq: Syspwrupreq,
    dbgpwrupreq: Dbgpwrupreq,
    _reserved2: [u8; 0xf8],
    enable: Enable,
    _reserved3: [u8; 0x14],
    traceportspeed: Traceportspeed,
    _reserved4: [u8; 0x04],
    tinstance: Tinstance,
}
impl RegisterBlock {
    #[doc = "0x400 - System power-up request"]
    #[inline(always)]
    pub const fn syspwrupreq(&self) -> &Syspwrupreq {
        &self.syspwrupreq
    }
    #[doc = "0x404 - Debug power-up request"]
    #[inline(always)]
    pub const fn dbgpwrupreq(&self) -> &Dbgpwrupreq {
        &self.dbgpwrupreq
    }
    #[doc = "0x500 - Enable debug domain and aquire selected GPIOs"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x518 - Trace port speed"]
    #[inline(always)]
    pub const fn traceportspeed(&self) -> &Traceportspeed {
        &self.traceportspeed
    }
    #[doc = "0x520 - SW-DP Target instance"]
    #[inline(always)]
    pub const fn tinstance(&self) -> &Tinstance {
        &self.tinstance
    }
}
#[doc = "SYSPWRUPREQ (rw) register accessor: System power-up request\n\nYou can [`read`](crate::Reg::read) this register and get [`syspwrupreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspwrupreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspwrupreq`]
module"]
#[doc(alias = "SYSPWRUPREQ")]
pub type Syspwrupreq = crate::Reg<syspwrupreq::SyspwrupreqSpec>;
#[doc = "System power-up request"]
pub mod syspwrupreq;
#[doc = "DBGPWRUPREQ (rw) register accessor: Debug power-up request\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgpwrupreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgpwrupreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgpwrupreq`]
module"]
#[doc(alias = "DBGPWRUPREQ")]
pub type Dbgpwrupreq = crate::Reg<dbgpwrupreq::DbgpwrupreqSpec>;
#[doc = "Debug power-up request"]
pub mod dbgpwrupreq;
#[doc = "ENABLE (rw) register accessor: Enable debug domain and aquire selected GPIOs\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable debug domain and aquire selected GPIOs"]
pub mod enable;
#[doc = "TRACEPORTSPEED (rw) register accessor: Trace port speed\n\nYou can [`read`](crate::Reg::read) this register and get [`traceportspeed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`traceportspeed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@traceportspeed`]
module"]
#[doc(alias = "TRACEPORTSPEED")]
pub type Traceportspeed = crate::Reg<traceportspeed::TraceportspeedSpec>;
#[doc = "Trace port speed"]
pub mod traceportspeed;
#[doc = "TINSTANCE (rw) register accessor: SW-DP Target instance\n\nYou can [`read`](crate::Reg::read) this register and get [`tinstance::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tinstance::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tinstance`]
module"]
#[doc(alias = "TINSTANCE")]
pub type Tinstance = crate::Reg<tinstance::TinstanceSpec>;
#[doc = "SW-DP Target instance"]
pub mod tinstance;
