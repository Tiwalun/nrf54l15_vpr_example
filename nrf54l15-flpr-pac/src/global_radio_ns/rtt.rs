#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "RTT")]
pub struct Rtt {
    config: Config,
    segment01: Segment01,
    segment23: Segment23,
    segment45: Segment45,
    segment67: Segment67,
}
impl Rtt {
    #[doc = "0x00 - RTT Config."]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x04 - RTT segments 0 and 1"]
    #[inline(always)]
    pub const fn segment01(&self) -> &Segment01 {
        &self.segment01
    }
    #[doc = "0x08 - RTT segments 2 and 3"]
    #[inline(always)]
    pub const fn segment23(&self) -> &Segment23 {
        &self.segment23
    }
    #[doc = "0x0c - RTT segments 4 and 5"]
    #[inline(always)]
    pub const fn segment45(&self) -> &Segment45 {
        &self.segment45
    }
    #[doc = "0x10 - RTT segments 6 and 7"]
    #[inline(always)]
    pub const fn segment67(&self) -> &Segment67 {
        &self.segment67
    }
}
#[doc = "CONFIG (rw) register accessor: RTT Config.\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "RTT Config."]
pub mod config;
#[doc = "SEGMENT01 (rw) register accessor: RTT segments 0 and 1\n\nYou can [`read`](crate::Reg::read) this register and get [`segment01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segment01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segment01`]
module"]
#[doc(alias = "SEGMENT01")]
pub type Segment01 = crate::Reg<segment01::Segment01Spec>;
#[doc = "RTT segments 0 and 1"]
pub mod segment01;
#[doc = "SEGMENT23 (rw) register accessor: RTT segments 2 and 3\n\nYou can [`read`](crate::Reg::read) this register and get [`segment23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segment23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segment23`]
module"]
#[doc(alias = "SEGMENT23")]
pub type Segment23 = crate::Reg<segment23::Segment23Spec>;
#[doc = "RTT segments 2 and 3"]
pub mod segment23;
#[doc = "SEGMENT45 (rw) register accessor: RTT segments 4 and 5\n\nYou can [`read`](crate::Reg::read) this register and get [`segment45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segment45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segment45`]
module"]
#[doc(alias = "SEGMENT45")]
pub type Segment45 = crate::Reg<segment45::Segment45Spec>;
#[doc = "RTT segments 4 and 5"]
pub mod segment45;
#[doc = "SEGMENT67 (rw) register accessor: RTT segments 6 and 7\n\nYou can [`read`](crate::Reg::read) this register and get [`segment67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segment67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segment67`]
module"]
#[doc(alias = "SEGMENT67")]
pub type Segment67 = crate::Reg<segment67::Segment67Spec>;
#[doc = "RTT segments 6 and 7"]
pub mod segment67;
