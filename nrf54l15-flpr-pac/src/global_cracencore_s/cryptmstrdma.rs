#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "CRYPTMSTRDMA")]
pub struct Cryptmstrdma {
    fetchaddrlsb: Fetchaddrlsb,
    fetchaddrmsb: Fetchaddrmsb,
    fetchlen: Fetchlen,
    fetchtag: Fetchtag,
    pushaddrlsb: Pushaddrlsb,
    pushaddrmsb: Pushaddrmsb,
    pushlen: Pushlen,
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    intstatraw: Intstatraw,
    intstat: Intstat,
    intstatclr: Intstatclr,
    config: Config,
    start: Start,
    status: Status,
}
impl Cryptmstrdma {
    #[doc = "0x00 - Fetch Address Least Significant Bit"]
    #[inline(always)]
    pub const fn fetchaddrlsb(&self) -> &Fetchaddrlsb {
        &self.fetchaddrlsb
    }
    #[doc = "0x04 - Fetch Address Most Significant Bit"]
    #[inline(always)]
    pub const fn fetchaddrmsb(&self) -> &Fetchaddrmsb {
        &self.fetchaddrmsb
    }
    #[doc = "0x08 - Fetch Length"]
    #[inline(always)]
    pub const fn fetchlen(&self) -> &Fetchlen {
        &self.fetchlen
    }
    #[doc = "0x0c - Fetch Tag"]
    #[inline(always)]
    pub const fn fetchtag(&self) -> &Fetchtag {
        &self.fetchtag
    }
    #[doc = "0x10 - Push Address Least Significant Bit"]
    #[inline(always)]
    pub const fn pushaddrlsb(&self) -> &Pushaddrlsb {
        &self.pushaddrlsb
    }
    #[doc = "0x14 - Push Address Most Significant Bit"]
    #[inline(always)]
    pub const fn pushaddrmsb(&self) -> &Pushaddrmsb {
        &self.pushaddrmsb
    }
    #[doc = "0x18 - Push Length"]
    #[inline(always)]
    pub const fn pushlen(&self) -> &Pushlen {
        &self.pushlen
    }
    #[doc = "0x1c - Interrupt Enable"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x20 - Interrupt Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x24 - Interrupt Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x28 - Interrupt Status Raw"]
    #[inline(always)]
    pub const fn intstatraw(&self) -> &Intstatraw {
        &self.intstatraw
    }
    #[doc = "0x2c - Interrupt Status"]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x30 - Interrupt Status Clear"]
    #[inline(always)]
    pub const fn intstatclr(&self) -> &Intstatclr {
        &self.intstatclr
    }
    #[doc = "0x34 - Configuration"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x38 - Start"]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x3c - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
}
#[doc = "FETCHADDRLSB (rw) register accessor: Fetch Address Least Significant Bit\n\nYou can [`read`](crate::Reg::read) this register and get [`fetchaddrlsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fetchaddrlsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fetchaddrlsb`]
module"]
#[doc(alias = "FETCHADDRLSB")]
pub type Fetchaddrlsb = crate::Reg<fetchaddrlsb::FetchaddrlsbSpec>;
#[doc = "Fetch Address Least Significant Bit"]
pub mod fetchaddrlsb;
#[doc = "FETCHADDRMSB (rw) register accessor: Fetch Address Most Significant Bit\n\nYou can [`read`](crate::Reg::read) this register and get [`fetchaddrmsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fetchaddrmsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fetchaddrmsb`]
module"]
#[doc(alias = "FETCHADDRMSB")]
pub type Fetchaddrmsb = crate::Reg<fetchaddrmsb::FetchaddrmsbSpec>;
#[doc = "Fetch Address Most Significant Bit"]
pub mod fetchaddrmsb;
#[doc = "FETCHLEN (rw) register accessor: Fetch Length\n\nYou can [`read`](crate::Reg::read) this register and get [`fetchlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fetchlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fetchlen`]
module"]
#[doc(alias = "FETCHLEN")]
pub type Fetchlen = crate::Reg<fetchlen::FetchlenSpec>;
#[doc = "Fetch Length"]
pub mod fetchlen;
#[doc = "FETCHTAG (rw) register accessor: Fetch Tag\n\nYou can [`read`](crate::Reg::read) this register and get [`fetchtag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fetchtag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fetchtag`]
module"]
#[doc(alias = "FETCHTAG")]
pub type Fetchtag = crate::Reg<fetchtag::FetchtagSpec>;
#[doc = "Fetch Tag"]
pub mod fetchtag;
#[doc = "PUSHADDRLSB (rw) register accessor: Push Address Least Significant Bit\n\nYou can [`read`](crate::Reg::read) this register and get [`pushaddrlsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pushaddrlsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pushaddrlsb`]
module"]
#[doc(alias = "PUSHADDRLSB")]
pub type Pushaddrlsb = crate::Reg<pushaddrlsb::PushaddrlsbSpec>;
#[doc = "Push Address Least Significant Bit"]
pub mod pushaddrlsb;
#[doc = "PUSHADDRMSB (rw) register accessor: Push Address Most Significant Bit\n\nYou can [`read`](crate::Reg::read) this register and get [`pushaddrmsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pushaddrmsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pushaddrmsb`]
module"]
#[doc(alias = "PUSHADDRMSB")]
pub type Pushaddrmsb = crate::Reg<pushaddrmsb::PushaddrmsbSpec>;
#[doc = "Push Address Most Significant Bit"]
pub mod pushaddrmsb;
#[doc = "PUSHLEN (rw) register accessor: Push Length\n\nYou can [`read`](crate::Reg::read) this register and get [`pushlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pushlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pushlen`]
module"]
#[doc(alias = "PUSHLEN")]
pub type Pushlen = crate::Reg<pushlen::PushlenSpec>;
#[doc = "Push Length"]
pub mod pushlen;
#[doc = "INTEN (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt Enable"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: Interrupt Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Set"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Interrupt Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Clear"]
pub mod intenclr;
#[doc = "INTSTATRAW (rw) register accessor: Interrupt Status Raw\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatraw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstatraw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatraw`]
module"]
#[doc(alias = "INTSTATRAW")]
pub type Intstatraw = crate::Reg<intstatraw::IntstatrawSpec>;
#[doc = "Interrupt Status Raw"]
pub mod intstatraw;
#[doc = "INTSTAT (rw) register accessor: Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`]
module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "Interrupt Status"]
pub mod intstat;
#[doc = "INTSTATCLR (rw) register accessor: Interrupt Status Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstatclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatclr`]
module"]
#[doc(alias = "INTSTATCLR")]
pub type Intstatclr = crate::Reg<intstatclr::IntstatclrSpec>;
#[doc = "Interrupt Status Clear"]
pub mod intstatclr;
#[doc = "CONFIG (rw) register accessor: Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration"]
pub mod config;
#[doc = "START (rw) register accessor: Start\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`]
module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = "Start"]
pub mod start;
#[doc = "STATUS (rw) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
