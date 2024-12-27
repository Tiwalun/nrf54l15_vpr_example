#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "RNGCONTROL")]
pub struct Rngcontrol {
    control: Control,
    fifolevel: Fifolevel,
    fifothreshold: Fifothreshold,
    fifodepth: Fifodepth,
    key: [Key; 4],
    testdata: Testdata,
    repeatthreshold: Repeatthreshold,
    propthreshold: Propthreshold,
    _reserved8: [u8; 0x04],
    status: Status,
    initwaitval: Initwaitval,
    disableosc: [Disableosc; 2],
    swofftmrval: Swofftmrval,
    clkdiv: Clkdiv,
    ais31conf0: Ais31conf0,
    ais31conf1: Ais31conf1,
    ais31conf2: Ais31conf2,
    ais31status: Ais31status,
    hwconfig: Hwconfig,
    _reserved18: [u8; 0x24],
    fifo: [Fifo; 16],
}
impl Rngcontrol {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x04 - FIFO level register."]
    #[inline(always)]
    pub const fn fifolevel(&self) -> &Fifolevel {
        &self.fifolevel
    }
    #[doc = "0x08 - FIFO threshold register."]
    #[inline(always)]
    pub const fn fifothreshold(&self) -> &Fifothreshold {
        &self.fifothreshold
    }
    #[doc = "0x0c - FIFO depth register."]
    #[inline(always)]
    pub const fn fifodepth(&self) -> &Fifodepth {
        &self.fifodepth
    }
    #[doc = "0x10..0x20 - Description collection: Key register."]
    #[inline(always)]
    pub const fn key(&self, n: usize) -> &Key {
        &self.key[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - Description collection: Key register."]
    #[inline(always)]
    pub fn key_iter(&self) -> impl Iterator<Item = &Key> {
        self.key.iter()
    }
    #[doc = "0x20 - Test data register."]
    #[inline(always)]
    pub const fn testdata(&self) -> &Testdata {
        &self.testdata
    }
    #[doc = "0x24 - Repetition Test Count Cut-Off value."]
    #[inline(always)]
    pub const fn repeatthreshold(&self) -> &Repeatthreshold {
        &self.repeatthreshold
    }
    #[doc = "0x28 - Adaptive Proportion Test (1024-sample window) Cut-Off value."]
    #[inline(always)]
    pub const fn propthreshold(&self) -> &Propthreshold {
        &self.propthreshold
    }
    #[doc = "0x30 - Status register."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x34 - Initial wait counter value."]
    #[inline(always)]
    pub const fn initwaitval(&self) -> &Initwaitval {
        &self.initwaitval
    }
    #[doc = "0x38..0x40 - Description collection: Disable oscillator rings #n*32 to #((n+1)*32)-1."]
    #[inline(always)]
    pub const fn disableosc(&self, n: usize) -> &Disableosc {
        &self.disableosc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x38..0x40 - Description collection: Disable oscillator rings #n*32 to #((n+1)*32)-1."]
    #[inline(always)]
    pub fn disableosc_iter(&self) -> impl Iterator<Item = &Disableosc> {
        self.disableosc.iter()
    }
    #[doc = "0x40 - Switch off timer value."]
    #[inline(always)]
    pub const fn swofftmrval(&self) -> &Swofftmrval {
        &self.swofftmrval
    }
    #[doc = "0x44 - Sample clock divider."]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x48 - AIS31 configuration register 0."]
    #[inline(always)]
    pub const fn ais31conf0(&self) -> &Ais31conf0 {
        &self.ais31conf0
    }
    #[doc = "0x4c - AIS31 configuration register 1."]
    #[inline(always)]
    pub const fn ais31conf1(&self) -> &Ais31conf1 {
        &self.ais31conf1
    }
    #[doc = "0x50 - AIS31 configuration register 2."]
    #[inline(always)]
    pub const fn ais31conf2(&self) -> &Ais31conf2 {
        &self.ais31conf2
    }
    #[doc = "0x54 - AIS31 status register."]
    #[inline(always)]
    pub const fn ais31status(&self) -> &Ais31status {
        &self.ais31status
    }
    #[doc = "0x58 - Hardware configuration register."]
    #[inline(always)]
    pub const fn hwconfig(&self) -> &Hwconfig {
        &self.hwconfig
    }
    #[doc = "0x80..0xc0 - Description collection: FIFO data"]
    #[inline(always)]
    pub const fn fifo(&self, n: usize) -> &Fifo {
        &self.fifo[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xc0 - Description collection: FIFO data"]
    #[inline(always)]
    pub fn fifo_iter(&self) -> impl Iterator<Item = &Fifo> {
        self.fifo.iter()
    }
}
#[doc = "CONTROL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Control register"]
pub mod control;
#[doc = "FIFOLEVEL (rw) register accessor: FIFO level register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifolevel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifolevel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifolevel`]
module"]
#[doc(alias = "FIFOLEVEL")]
pub type Fifolevel = crate::Reg<fifolevel::FifolevelSpec>;
#[doc = "FIFO level register."]
pub mod fifolevel;
#[doc = "FIFOTHRESHOLD (rw) register accessor: FIFO threshold register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifothreshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifothreshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifothreshold`]
module"]
#[doc(alias = "FIFOTHRESHOLD")]
pub type Fifothreshold = crate::Reg<fifothreshold::FifothresholdSpec>;
#[doc = "FIFO threshold register."]
pub mod fifothreshold;
#[doc = "FIFODEPTH (rw) register accessor: FIFO depth register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifodepth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifodepth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifodepth`]
module"]
#[doc(alias = "FIFODEPTH")]
pub type Fifodepth = crate::Reg<fifodepth::FifodepthSpec>;
#[doc = "FIFO depth register."]
pub mod fifodepth;
#[doc = "KEY (rw) register accessor: Description collection: Key register.\n\nYou can [`read`](crate::Reg::read) this register and get [`key::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`]
module"]
#[doc(alias = "KEY")]
pub type Key = crate::Reg<key::KeySpec>;
#[doc = "Description collection: Key register."]
pub mod key;
#[doc = "TESTDATA (rw) register accessor: Test data register.\n\nYou can [`read`](crate::Reg::read) this register and get [`testdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testdata`]
module"]
#[doc(alias = "TESTDATA")]
pub type Testdata = crate::Reg<testdata::TestdataSpec>;
#[doc = "Test data register."]
pub mod testdata;
#[doc = "REPEATTHRESHOLD (rw) register accessor: Repetition Test Count Cut-Off value.\n\nYou can [`read`](crate::Reg::read) this register and get [`repeatthreshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repeatthreshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@repeatthreshold`]
module"]
#[doc(alias = "REPEATTHRESHOLD")]
pub type Repeatthreshold = crate::Reg<repeatthreshold::RepeatthresholdSpec>;
#[doc = "Repetition Test Count Cut-Off value."]
pub mod repeatthreshold;
#[doc = "PROPTHRESHOLD (rw) register accessor: Adaptive Proportion Test (1024-sample window) Cut-Off value.\n\nYou can [`read`](crate::Reg::read) this register and get [`propthreshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`propthreshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@propthreshold`]
module"]
#[doc(alias = "PROPTHRESHOLD")]
pub type Propthreshold = crate::Reg<propthreshold::PropthresholdSpec>;
#[doc = "Adaptive Proportion Test (1024-sample window) Cut-Off value."]
pub mod propthreshold;
#[doc = "STATUS (rw) register accessor: Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status register."]
pub mod status;
#[doc = "INITWAITVAL (rw) register accessor: Initial wait counter value.\n\nYou can [`read`](crate::Reg::read) this register and get [`initwaitval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initwaitval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@initwaitval`]
module"]
#[doc(alias = "INITWAITVAL")]
pub type Initwaitval = crate::Reg<initwaitval::InitwaitvalSpec>;
#[doc = "Initial wait counter value."]
pub mod initwaitval;
#[doc = "DISABLEOSC (rw) register accessor: Description collection: Disable oscillator rings #n*32 to #((n+1)*32)-1.\n\nYou can [`read`](crate::Reg::read) this register and get [`disableosc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disableosc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disableosc`]
module"]
#[doc(alias = "DISABLEOSC")]
pub type Disableosc = crate::Reg<disableosc::DisableoscSpec>;
#[doc = "Description collection: Disable oscillator rings #n*32 to #((n+1)*32)-1."]
pub mod disableosc;
#[doc = "SWOFFTMRVAL (rw) register accessor: Switch off timer value.\n\nYou can [`read`](crate::Reg::read) this register and get [`swofftmrval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swofftmrval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swofftmrval`]
module"]
#[doc(alias = "SWOFFTMRVAL")]
pub type Swofftmrval = crate::Reg<swofftmrval::SwofftmrvalSpec>;
#[doc = "Switch off timer value."]
pub mod swofftmrval;
#[doc = "CLKDIV (rw) register accessor: Sample clock divider.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`]
module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "Sample clock divider."]
pub mod clkdiv;
#[doc = "AIS31CONF0 (rw) register accessor: AIS31 configuration register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ais31conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ais31conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ais31conf0`]
module"]
#[doc(alias = "AIS31CONF0")]
pub type Ais31conf0 = crate::Reg<ais31conf0::Ais31conf0Spec>;
#[doc = "AIS31 configuration register 0."]
pub mod ais31conf0;
#[doc = "AIS31CONF1 (rw) register accessor: AIS31 configuration register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`ais31conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ais31conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ais31conf1`]
module"]
#[doc(alias = "AIS31CONF1")]
pub type Ais31conf1 = crate::Reg<ais31conf1::Ais31conf1Spec>;
#[doc = "AIS31 configuration register 1."]
pub mod ais31conf1;
#[doc = "AIS31CONF2 (rw) register accessor: AIS31 configuration register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`ais31conf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ais31conf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ais31conf2`]
module"]
#[doc(alias = "AIS31CONF2")]
pub type Ais31conf2 = crate::Reg<ais31conf2::Ais31conf2Spec>;
#[doc = "AIS31 configuration register 2."]
pub mod ais31conf2;
#[doc = "AIS31STATUS (rw) register accessor: AIS31 status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ais31status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ais31status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ais31status`]
module"]
#[doc(alias = "AIS31STATUS")]
pub type Ais31status = crate::Reg<ais31status::Ais31statusSpec>;
#[doc = "AIS31 status register."]
pub mod ais31status;
#[doc = "HWCONFIG (rw) register accessor: Hardware configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hwconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwconfig`]
module"]
#[doc(alias = "HWCONFIG")]
pub type Hwconfig = crate::Reg<hwconfig::HwconfigSpec>;
#[doc = "Hardware configuration register."]
pub mod hwconfig;
#[doc = "FIFO (r) register accessor: Description collection: FIFO data\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`]
module"]
#[doc(alias = "FIFO")]
pub type Fifo = crate::Reg<fifo::FifoSpec>;
#[doc = "Description collection: FIFO data"]
pub mod fifo;
