#[repr(C)]
#[doc = "32.768 kHz oscillator control"]
#[doc(alias = "XOSC32KI")]
pub struct Xosc32ki {
    _reserved0: [u8; 0x04],
    intcap: Intcap,
}
impl Xosc32ki {
    #[doc = "0x04 - Programmable capacitance of XL1 and XL2"]
    #[inline(always)]
    pub const fn intcap(&self) -> &Intcap {
        &self.intcap
    }
}
#[doc = "INTCAP (rw) register accessor: Programmable capacitance of XL1 and XL2\n\nYou can [`read`](crate::Reg::read) this register and get [`intcap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intcap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intcap`]
module"]
#[doc(alias = "INTCAP")]
pub type Intcap = crate::Reg<intcap::IntcapSpec>;
#[doc = "Programmable capacitance of XL1 and XL2"]
pub mod intcap;
