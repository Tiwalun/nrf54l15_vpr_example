#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PLL")]
pub struct Pll {
    _reserved0: [u8; 0x08],
    run: Run,
    stat: Stat,
}
impl Pll {
    #[doc = "0x08 - Indicates that PLLSTART task was triggered"]
    #[inline(always)]
    pub const fn run(&self) -> &Run {
        &self.run
    }
    #[doc = "0x0c - Which PLL settings were selected when triggering START task"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
}
#[doc = "RUN (r) register accessor: Indicates that PLLSTART task was triggered\n\nYou can [`read`](crate::Reg::read) this register and get [`run::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@run`]
module"]
#[doc(alias = "RUN")]
pub type Run = crate::Reg<run::RunSpec>;
#[doc = "Indicates that PLLSTART task was triggered"]
pub mod run;
#[doc = "STAT (r) register accessor: Which PLL settings were selected when triggering START task\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Which PLL settings were selected when triggering START task"]
pub mod stat;
