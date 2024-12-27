#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "GRTC")]
pub struct Grtc {
    cc: [Cc; 24],
    _reserved1: [u8; 0x14],
    pwmconfig: Pwmconfig,
    clk: Clk,
    syscounter: Syscounter,
    interrupt: [Interrupt; 16],
}
impl Grtc {
    #[doc = "0x00..0x60 - Description collection: Configuration of features for CC n of GRTC"]
    #[inline(always)]
    pub const fn cc(&self, n: usize) -> &Cc {
        &self.cc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x60 - Description collection: Configuration of features for CC n of GRTC"]
    #[inline(always)]
    pub fn cc_iter(&self) -> impl Iterator<Item = &Cc> {
        self.cc.iter()
    }
    #[doc = "0x74 - Configuration of feature for PWMCONFIG of GRTC"]
    #[inline(always)]
    pub const fn pwmconfig(&self) -> &Pwmconfig {
        &self.pwmconfig
    }
    #[doc = "0x78 - Configuration of features for CLKOUT/CLKCFG of GRTC"]
    #[inline(always)]
    pub const fn clk(&self) -> &Clk {
        &self.clk
    }
    #[doc = "0x7c - Configuration of features for SYSCOUNTERL/SYSCOUNTERH of GRTC"]
    #[inline(always)]
    pub const fn syscounter(&self) -> &Syscounter {
        &self.syscounter
    }
    #[doc = "0x80..0xc0 - Description collection: Configuration of features for interrupt n of GRTC"]
    #[inline(always)]
    pub const fn interrupt(&self, n: usize) -> &Interrupt {
        &self.interrupt[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xc0 - Description collection: Configuration of features for interrupt n of GRTC"]
    #[inline(always)]
    pub fn interrupt_iter(&self) -> impl Iterator<Item = &Interrupt> {
        self.interrupt.iter()
    }
}
#[doc = "CC (rw) register accessor: Description collection: Configuration of features for CC n of GRTC\n\nYou can [`read`](crate::Reg::read) this register and get [`cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc`]
module"]
#[doc(alias = "CC")]
pub type Cc = crate::Reg<cc::CcSpec>;
#[doc = "Description collection: Configuration of features for CC n of GRTC"]
pub mod cc;
#[doc = "PWMCONFIG (rw) register accessor: Configuration of feature for PWMCONFIG of GRTC\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmconfig`]
module"]
#[doc(alias = "PWMCONFIG")]
pub type Pwmconfig = crate::Reg<pwmconfig::PwmconfigSpec>;
#[doc = "Configuration of feature for PWMCONFIG of GRTC"]
pub mod pwmconfig;
#[doc = "CLK (rw) register accessor: Configuration of features for CLKOUT/CLKCFG of GRTC\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`]
module"]
#[doc(alias = "CLK")]
pub type Clk = crate::Reg<clk::ClkSpec>;
#[doc = "Configuration of features for CLKOUT/CLKCFG of GRTC"]
pub mod clk;
#[doc = "SYSCOUNTER (rw) register accessor: Configuration of features for SYSCOUNTERL/SYSCOUNTERH of GRTC\n\nYou can [`read`](crate::Reg::read) this register and get [`syscounter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscounter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscounter`]
module"]
#[doc(alias = "SYSCOUNTER")]
pub type Syscounter = crate::Reg<syscounter::SyscounterSpec>;
#[doc = "Configuration of features for SYSCOUNTERL/SYSCOUNTERH of GRTC"]
pub mod syscounter;
#[doc = "INTERRUPT (rw) register accessor: Description collection: Configuration of features for interrupt n of GRTC\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt`]
module"]
#[doc(alias = "INTERRUPT")]
pub type Interrupt = crate::Reg<interrupt::InterruptSpec>;
#[doc = "Description collection: Configuration of features for interrupt n of GRTC"]
pub mod interrupt;
