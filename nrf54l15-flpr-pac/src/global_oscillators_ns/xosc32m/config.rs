#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "CONFIG")]
pub struct Config {
    _reserved0: [u8; 0x08],
    intcap: Intcap,
}
impl Config {
    #[doc = "0x08 - Crystal load capacitor as seen by the crystal across its terminals, including pin capacitance but excluding PCB stray capacitance."]
    #[inline(always)]
    pub const fn intcap(&self) -> &Intcap {
        &self.intcap
    }
}
#[doc = "INTCAP (rw) register accessor: Crystal load capacitor as seen by the crystal across its terminals, including pin capacitance but excluding PCB stray capacitance.\n\nYou can [`read`](crate::Reg::read) this register and get [`intcap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intcap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intcap`]
module"]
#[doc(alias = "INTCAP")]
pub type Intcap = crate::Reg<intcap::IntcapSpec>;
#[doc = "Crystal load capacitor as seen by the crystal across its terminals, including pin capacitance but excluding PCB stray capacitance."]
pub mod intcap;
