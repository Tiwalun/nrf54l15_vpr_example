#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "IKG")]
pub struct Ikg {
    start: Start,
    status: Status,
    initdata: Initdata,
    nonce: Nonce,
    personalisationstring: Personalisationstring,
    reseedintervallsb: Reseedintervallsb,
    reseedintervalmsb: Reseedintervalmsb,
    pkecontrol: Pkecontrol,
    pkecommand: Pkecommand,
    pkestatus: Pkestatus,
    softrst: Softrst,
    hwconfig: Hwconfig,
}
impl Ikg {
    #[doc = "0x00 - Start register."]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x04 - Status register."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - InitData register."]
    #[inline(always)]
    pub const fn initdata(&self) -> &Initdata {
        &self.initdata
    }
    #[doc = "0x0c - Nonce register."]
    #[inline(always)]
    pub const fn nonce(&self) -> &Nonce {
        &self.nonce
    }
    #[doc = "0x10 - Personalisation String register."]
    #[inline(always)]
    pub const fn personalisationstring(&self) -> &Personalisationstring {
        &self.personalisationstring
    }
    #[doc = "0x14 - Reseed Interval LSB register."]
    #[inline(always)]
    pub const fn reseedintervallsb(&self) -> &Reseedintervallsb {
        &self.reseedintervallsb
    }
    #[doc = "0x18 - Reseed Interval MSB register."]
    #[inline(always)]
    pub const fn reseedintervalmsb(&self) -> &Reseedintervalmsb {
        &self.reseedintervalmsb
    }
    #[doc = "0x1c - PKE Control register."]
    #[inline(always)]
    pub const fn pkecontrol(&self) -> &Pkecontrol {
        &self.pkecontrol
    }
    #[doc = "0x20 - PKE Command register."]
    #[inline(always)]
    pub const fn pkecommand(&self) -> &Pkecommand {
        &self.pkecommand
    }
    #[doc = "0x24 - PKE Status register."]
    #[inline(always)]
    pub const fn pkestatus(&self) -> &Pkestatus {
        &self.pkestatus
    }
    #[doc = "0x28 - SoftRst register."]
    #[inline(always)]
    pub const fn softrst(&self) -> &Softrst {
        &self.softrst
    }
    #[doc = "0x2c - HwConfig register."]
    #[inline(always)]
    pub const fn hwconfig(&self) -> &Hwconfig {
        &self.hwconfig
    }
}
#[doc = "START (rw) register accessor: Start register.\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`]
module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = "Start register."]
pub mod start;
#[doc = "STATUS (rw) register accessor: Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status register."]
pub mod status;
#[doc = "INITDATA (rw) register accessor: InitData register.\n\nYou can [`read`](crate::Reg::read) this register and get [`initdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@initdata`]
module"]
#[doc(alias = "INITDATA")]
pub type Initdata = crate::Reg<initdata::InitdataSpec>;
#[doc = "InitData register."]
pub mod initdata;
#[doc = "NONCE (rw) register accessor: Nonce register.\n\nYou can [`read`](crate::Reg::read) this register and get [`nonce::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonce::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nonce`]
module"]
#[doc(alias = "NONCE")]
pub type Nonce = crate::Reg<nonce::NonceSpec>;
#[doc = "Nonce register."]
pub mod nonce;
#[doc = "PERSONALISATIONSTRING (rw) register accessor: Personalisation String register.\n\nYou can [`read`](crate::Reg::read) this register and get [`personalisationstring::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`personalisationstring::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@personalisationstring`]
module"]
#[doc(alias = "PERSONALISATIONSTRING")]
pub type Personalisationstring = crate::Reg<personalisationstring::PersonalisationstringSpec>;
#[doc = "Personalisation String register."]
pub mod personalisationstring;
#[doc = "RESEEDINTERVALLSB (rw) register accessor: Reseed Interval LSB register.\n\nYou can [`read`](crate::Reg::read) this register and get [`reseedintervallsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reseedintervallsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reseedintervallsb`]
module"]
#[doc(alias = "RESEEDINTERVALLSB")]
pub type Reseedintervallsb = crate::Reg<reseedintervallsb::ReseedintervallsbSpec>;
#[doc = "Reseed Interval LSB register."]
pub mod reseedintervallsb;
#[doc = "RESEEDINTERVALMSB (rw) register accessor: Reseed Interval MSB register.\n\nYou can [`read`](crate::Reg::read) this register and get [`reseedintervalmsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reseedintervalmsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reseedintervalmsb`]
module"]
#[doc(alias = "RESEEDINTERVALMSB")]
pub type Reseedintervalmsb = crate::Reg<reseedintervalmsb::ReseedintervalmsbSpec>;
#[doc = "Reseed Interval MSB register."]
pub mod reseedintervalmsb;
#[doc = "PKECONTROL (rw) register accessor: PKE Control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkecontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkecontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkecontrol`]
module"]
#[doc(alias = "PKECONTROL")]
pub type Pkecontrol = crate::Reg<pkecontrol::PkecontrolSpec>;
#[doc = "PKE Control register."]
pub mod pkecontrol;
#[doc = "PKECOMMAND (rw) register accessor: PKE Command register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkecommand::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkecommand::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkecommand`]
module"]
#[doc(alias = "PKECOMMAND")]
pub type Pkecommand = crate::Reg<pkecommand::PkecommandSpec>;
#[doc = "PKE Command register."]
pub mod pkecommand;
#[doc = "PKESTATUS (rw) register accessor: PKE Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkestatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkestatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkestatus`]
module"]
#[doc(alias = "PKESTATUS")]
pub type Pkestatus = crate::Reg<pkestatus::PkestatusSpec>;
#[doc = "PKE Status register."]
pub mod pkestatus;
#[doc = "SOFTRST (rw) register accessor: SoftRst register.\n\nYou can [`read`](crate::Reg::read) this register and get [`softrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`softrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst`]
module"]
#[doc(alias = "SOFTRST")]
pub type Softrst = crate::Reg<softrst::SoftrstSpec>;
#[doc = "SoftRst register."]
pub mod softrst;
#[doc = "HWCONFIG (rw) register accessor: HwConfig register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hwconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwconfig`]
module"]
#[doc(alias = "HWCONFIG")]
pub type Hwconfig = crate::Reg<hwconfig::HwconfigSpec>;
#[doc = "HwConfig register."]
pub mod hwconfig;
