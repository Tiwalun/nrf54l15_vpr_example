#[repr(C)]
#[doc = "32 MHz oscillator control"]
#[doc(alias = "XOSC32M")]
pub struct Xosc32m {
    _reserved0: [u8; 0x14],
    config: Config,
}
impl Xosc32m {
    #[doc = "0x14..0x20 - Unspecified"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
}
#[doc = "Unspecified"]
pub use self::config::Config;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod config;
