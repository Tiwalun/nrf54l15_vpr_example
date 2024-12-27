#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x05a0],
    config: Config,
}
impl RegisterBlock {
    #[doc = "0x5a0 - Configuration for glitch detector"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
}
#[doc = "CONFIG (rw) register accessor: Configuration for glitch detector\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration for glitch detector"]
pub mod config;
