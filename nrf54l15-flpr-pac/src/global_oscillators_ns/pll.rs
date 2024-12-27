#[repr(C)]
#[doc = "Oscillator control"]
#[doc(alias = "PLL")]
pub struct Pll {
    freq: Freq,
    currentfreq: Currentfreq,
}
impl Pll {
    #[doc = "0x00 - Set speed of MCU power domain, including CPU"]
    #[inline(always)]
    pub const fn freq(&self) -> &Freq {
        &self.freq
    }
    #[doc = "0x04 - Current speed of MCU power domain, including CPU"]
    #[inline(always)]
    pub const fn currentfreq(&self) -> &Currentfreq {
        &self.currentfreq
    }
}
#[doc = "FREQ (rw) register accessor: Set speed of MCU power domain, including CPU\n\nYou can [`read`](crate::Reg::read) this register and get [`freq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freq`]
module"]
#[doc(alias = "FREQ")]
pub type Freq = crate::Reg<freq::FreqSpec>;
#[doc = "Set speed of MCU power domain, including CPU"]
pub mod freq;
#[doc = "CURRENTFREQ (r) register accessor: Current speed of MCU power domain, including CPU\n\nYou can [`read`](crate::Reg::read) this register and get [`currentfreq::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@currentfreq`]
module"]
#[doc(alias = "CURRENTFREQ")]
pub type Currentfreq = crate::Reg<currentfreq::CurrentfreqSpec>;
#[doc = "Current speed of MCU power domain, including CPU"]
pub mod currentfreq;
