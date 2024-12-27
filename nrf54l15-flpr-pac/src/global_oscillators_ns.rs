#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0700],
    xosc32m: Xosc32m,
    _reserved1: [u8; 0xe0],
    pll: Pll,
    _reserved2: [u8; 0xf8],
    xosc32ki: Xosc32ki,
}
impl RegisterBlock {
    #[doc = "0x700..0x720 - 32 MHz oscillator control"]
    #[inline(always)]
    pub const fn xosc32m(&self) -> &Xosc32m {
        &self.xosc32m
    }
    #[doc = "0x800..0x808 - Oscillator control"]
    #[inline(always)]
    pub const fn pll(&self) -> &Pll {
        &self.pll
    }
    #[doc = "0x900..0x908 - 32.768 kHz oscillator control"]
    #[inline(always)]
    pub const fn xosc32ki(&self) -> &Xosc32ki {
        &self.xosc32ki
    }
}
#[doc = "32 MHz oscillator control"]
pub use self::xosc32m::Xosc32m;
#[doc = r"Cluster"]
#[doc = "32 MHz oscillator control"]
pub mod xosc32m;
#[doc = "Oscillator control"]
pub use self::pll::Pll;
#[doc = r"Cluster"]
#[doc = "Oscillator control"]
pub mod pll;
#[doc = "32.768 kHz oscillator control"]
pub use self::xosc32ki::Xosc32ki;
#[doc = r"Cluster"]
#[doc = "32.768 kHz oscillator control"]
pub mod xosc32ki;
