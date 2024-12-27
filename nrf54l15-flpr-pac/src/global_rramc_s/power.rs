#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "POWER")]
pub struct Power {
    config: Config,
    _reserved1: [u8; 0x04],
    lowpowerconfig: Lowpowerconfig,
}
impl Power {
    #[doc = "0x00 - Power configuration"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x08 - Low power mode configuration"]
    #[inline(always)]
    pub const fn lowpowerconfig(&self) -> &Lowpowerconfig {
        &self.lowpowerconfig
    }
}
#[doc = "CONFIG (rw) register accessor: Power configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Power configuration"]
pub mod config;
#[doc = "LOWPOWERCONFIG (rw) register accessor: Low power mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`lowpowerconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lowpowerconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lowpowerconfig`]
module"]
#[doc(alias = "LOWPOWERCONFIG")]
pub type Lowpowerconfig = crate::Reg<lowpowerconfig::LowpowerconfigSpec>;
#[doc = "Low power mode configuration"]
pub mod lowpowerconfig;
