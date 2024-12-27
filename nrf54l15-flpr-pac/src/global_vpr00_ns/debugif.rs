#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "DEBUGIF")]
pub struct Debugif {
    _reserved0: [u8; 0x10],
    data0: Data0,
    data1: Data1,
    _reserved2: [u8; 0x28],
    dmcontrol: Dmcontrol,
    dmstatus: Dmstatus,
    hartinfo: Hartinfo,
    haltsum1: Haltsum1,
    hawindowsel: Hawindowsel,
    hawindow: Hawindow,
    abstractcs: Abstractcs,
    abstractcmd: Abstractcmd,
    abstractauto: Abstractauto,
    confstrptr: [Confstrptr; 4],
    nextdm: Nextdm,
    _reserved13: [u8; 0x08],
    progbuf: [Progbuf; 16],
    authdata: Authdata,
    _reserved15: [u8; 0x0c],
    haltsum2: Haltsum2,
    haltsum3: Haltsum3,
    _reserved17: [u8; 0x04],
    sbaddress3: Sbaddress3,
    sbcs: Sbcs,
    sbaddress0: Sbaddress0,
    sbaddress1: Sbaddress1,
    sbaddress2: Sbaddress2,
    sbdata0: Sbdata0,
    sbdata1: Sbdata1,
    sbdata2: Sbdata2,
    sbdata3: Sbdata3,
    haltsum0: Haltsum0,
}
impl Debugif {
    #[doc = "0x10 - Abstract Data 0. Read/write data for argument 0"]
    #[inline(always)]
    pub const fn data0(&self) -> &Data0 {
        &self.data0
    }
    #[doc = "0x14 - Abstract Data 1. Read/write data for argument 1"]
    #[inline(always)]
    pub const fn data1(&self) -> &Data1 {
        &self.data1
    }
    #[doc = "0x40 - Debug Module Control"]
    #[inline(always)]
    pub const fn dmcontrol(&self) -> &Dmcontrol {
        &self.dmcontrol
    }
    #[doc = "0x44 - Debug Module Status"]
    #[inline(always)]
    pub const fn dmstatus(&self) -> &Dmstatus {
        &self.dmstatus
    }
    #[doc = "0x48 - Hart Information"]
    #[inline(always)]
    pub const fn hartinfo(&self) -> &Hartinfo {
        &self.hartinfo
    }
    #[doc = "0x4c - Halt Summary 1"]
    #[inline(always)]
    pub const fn haltsum1(&self) -> &Haltsum1 {
        &self.haltsum1
    }
    #[doc = "0x50 - Hart Array Window Select"]
    #[inline(always)]
    pub const fn hawindowsel(&self) -> &Hawindowsel {
        &self.hawindowsel
    }
    #[doc = "0x54 - Hart Array Window"]
    #[inline(always)]
    pub const fn hawindow(&self) -> &Hawindow {
        &self.hawindow
    }
    #[doc = "0x58 - Abstract Control and Status"]
    #[inline(always)]
    pub const fn abstractcs(&self) -> &Abstractcs {
        &self.abstractcs
    }
    #[doc = "0x5c - Abstract command"]
    #[inline(always)]
    pub const fn abstractcmd(&self) -> &Abstractcmd {
        &self.abstractcmd
    }
    #[doc = "0x60 - Abstract Command Autoexec"]
    #[inline(always)]
    pub const fn abstractauto(&self) -> &Abstractauto {
        &self.abstractauto
    }
    #[doc = "0x64..0x74 - Description collection: Configuration String Pointer \\[n\\]"]
    #[inline(always)]
    pub const fn confstrptr(&self, n: usize) -> &Confstrptr {
        &self.confstrptr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x64..0x74 - Description collection: Configuration String Pointer \\[n\\]"]
    #[inline(always)]
    pub fn confstrptr_iter(&self) -> impl Iterator<Item = &Confstrptr> {
        self.confstrptr.iter()
    }
    #[doc = "0x74 - Next Debug Module"]
    #[inline(always)]
    pub const fn nextdm(&self) -> &Nextdm {
        &self.nextdm
    }
    #[doc = "0x80..0xc0 - Description collection: Program Buffer \\[n\\]"]
    #[inline(always)]
    pub const fn progbuf(&self, n: usize) -> &Progbuf {
        &self.progbuf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xc0 - Description collection: Program Buffer \\[n\\]"]
    #[inline(always)]
    pub fn progbuf_iter(&self) -> impl Iterator<Item = &Progbuf> {
        self.progbuf.iter()
    }
    #[doc = "0xc0 - Authentication Data"]
    #[inline(always)]
    pub const fn authdata(&self) -> &Authdata {
        &self.authdata
    }
    #[doc = "0xd0 - Halt Summary 2"]
    #[inline(always)]
    pub const fn haltsum2(&self) -> &Haltsum2 {
        &self.haltsum2
    }
    #[doc = "0xd4 - Halt Summary 3"]
    #[inline(always)]
    pub const fn haltsum3(&self) -> &Haltsum3 {
        &self.haltsum3
    }
    #[doc = "0xdc - System Bus Addres 127:96"]
    #[inline(always)]
    pub const fn sbaddress3(&self) -> &Sbaddress3 {
        &self.sbaddress3
    }
    #[doc = "0xe0 - System Bus Access Control and Status"]
    #[inline(always)]
    pub const fn sbcs(&self) -> &Sbcs {
        &self.sbcs
    }
    #[doc = "0xe4 - System Bus Addres 31:0"]
    #[inline(always)]
    pub const fn sbaddress0(&self) -> &Sbaddress0 {
        &self.sbaddress0
    }
    #[doc = "0xe8 - System Bus Addres 63:32"]
    #[inline(always)]
    pub const fn sbaddress1(&self) -> &Sbaddress1 {
        &self.sbaddress1
    }
    #[doc = "0xec - System Bus Addres 95:64"]
    #[inline(always)]
    pub const fn sbaddress2(&self) -> &Sbaddress2 {
        &self.sbaddress2
    }
    #[doc = "0xf0 - System Bus Data 31:0"]
    #[inline(always)]
    pub const fn sbdata0(&self) -> &Sbdata0 {
        &self.sbdata0
    }
    #[doc = "0xf4 - System Bus Data 63:32"]
    #[inline(always)]
    pub const fn sbdata1(&self) -> &Sbdata1 {
        &self.sbdata1
    }
    #[doc = "0xf8 - System Bus Data 95:64"]
    #[inline(always)]
    pub const fn sbdata2(&self) -> &Sbdata2 {
        &self.sbdata2
    }
    #[doc = "0xfc - System Bus Data 127:96"]
    #[inline(always)]
    pub const fn sbdata3(&self) -> &Sbdata3 {
        &self.sbdata3
    }
    #[doc = "0x100 - Halt summary 0"]
    #[inline(always)]
    pub const fn haltsum0(&self) -> &Haltsum0 {
        &self.haltsum0
    }
}
#[doc = "DATA0 (rw) register accessor: Abstract Data 0. Read/write data for argument 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0`]
module"]
#[doc(alias = "DATA0")]
pub type Data0 = crate::Reg<data0::Data0Spec>;
#[doc = "Abstract Data 0. Read/write data for argument 0"]
pub mod data0;
#[doc = "DATA1 (rw) register accessor: Abstract Data 1. Read/write data for argument 1\n\nYou can [`read`](crate::Reg::read) this register and get [`data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1`]
module"]
#[doc(alias = "DATA1")]
pub type Data1 = crate::Reg<data1::Data1Spec>;
#[doc = "Abstract Data 1. Read/write data for argument 1"]
pub mod data1;
#[doc = "DMCONTROL (rw) register accessor: Debug Module Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dmcontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmcontrol`]
module"]
#[doc(alias = "DMCONTROL")]
pub type Dmcontrol = crate::Reg<dmcontrol::DmcontrolSpec>;
#[doc = "Debug Module Control"]
pub mod dmcontrol;
#[doc = "DMSTATUS (r) register accessor: Debug Module Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dmstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmstatus`]
module"]
#[doc(alias = "DMSTATUS")]
pub type Dmstatus = crate::Reg<dmstatus::DmstatusSpec>;
#[doc = "Debug Module Status"]
pub mod dmstatus;
#[doc = "HARTINFO (rw) register accessor: Hart Information\n\nYou can [`read`](crate::Reg::read) this register and get [`hartinfo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hartinfo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hartinfo`]
module"]
#[doc(alias = "HARTINFO")]
pub type Hartinfo = crate::Reg<hartinfo::HartinfoSpec>;
#[doc = "Hart Information"]
pub mod hartinfo;
#[doc = "HALTSUM1 (rw) register accessor: Halt Summary 1\n\nYou can [`read`](crate::Reg::read) this register and get [`haltsum1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haltsum1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haltsum1`]
module"]
#[doc(alias = "HALTSUM1")]
pub type Haltsum1 = crate::Reg<haltsum1::Haltsum1Spec>;
#[doc = "Halt Summary 1"]
pub mod haltsum1;
#[doc = "HAWINDOWSEL (rw) register accessor: Hart Array Window Select\n\nYou can [`read`](crate::Reg::read) this register and get [`hawindowsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hawindowsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hawindowsel`]
module"]
#[doc(alias = "HAWINDOWSEL")]
pub type Hawindowsel = crate::Reg<hawindowsel::HawindowselSpec>;
#[doc = "Hart Array Window Select"]
pub mod hawindowsel;
#[doc = "HAWINDOW (rw) register accessor: Hart Array Window\n\nYou can [`read`](crate::Reg::read) this register and get [`hawindow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hawindow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hawindow`]
module"]
#[doc(alias = "HAWINDOW")]
pub type Hawindow = crate::Reg<hawindow::HawindowSpec>;
#[doc = "Hart Array Window"]
pub mod hawindow;
#[doc = "ABSTRACTCS (rw) register accessor: Abstract Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`abstractcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abstractcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@abstractcs`]
module"]
#[doc(alias = "ABSTRACTCS")]
pub type Abstractcs = crate::Reg<abstractcs::AbstractcsSpec>;
#[doc = "Abstract Control and Status"]
pub mod abstractcs;
#[doc = "ABSTRACTCMD (w) register accessor: Abstract command\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abstractcmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@abstractcmd`]
module"]
#[doc(alias = "ABSTRACTCMD")]
pub type Abstractcmd = crate::Reg<abstractcmd::AbstractcmdSpec>;
#[doc = "Abstract command"]
pub mod abstractcmd;
#[doc = "ABSTRACTAUTO (rw) register accessor: Abstract Command Autoexec\n\nYou can [`read`](crate::Reg::read) this register and get [`abstractauto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abstractauto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@abstractauto`]
module"]
#[doc(alias = "ABSTRACTAUTO")]
pub type Abstractauto = crate::Reg<abstractauto::AbstractautoSpec>;
#[doc = "Abstract Command Autoexec"]
pub mod abstractauto;
#[doc = "CONFSTRPTR (rw) register accessor: Description collection: Configuration String Pointer \\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`confstrptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confstrptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@confstrptr`]
module"]
#[doc(alias = "CONFSTRPTR")]
pub type Confstrptr = crate::Reg<confstrptr::ConfstrptrSpec>;
#[doc = "Description collection: Configuration String Pointer \\[n\\]"]
pub mod confstrptr;
#[doc = "NEXTDM (rw) register accessor: Next Debug Module\n\nYou can [`read`](crate::Reg::read) this register and get [`nextdm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nextdm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nextdm`]
module"]
#[doc(alias = "NEXTDM")]
pub type Nextdm = crate::Reg<nextdm::NextdmSpec>;
#[doc = "Next Debug Module"]
pub mod nextdm;
#[doc = "PROGBUF (rw) register accessor: Description collection: Program Buffer \\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`progbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`progbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@progbuf`]
module"]
#[doc(alias = "PROGBUF")]
pub type Progbuf = crate::Reg<progbuf::ProgbufSpec>;
#[doc = "Description collection: Program Buffer \\[n\\]"]
pub mod progbuf;
#[doc = "AUTHDATA (rw) register accessor: Authentication Data\n\nYou can [`read`](crate::Reg::read) this register and get [`authdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`authdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@authdata`]
module"]
#[doc(alias = "AUTHDATA")]
pub type Authdata = crate::Reg<authdata::AuthdataSpec>;
#[doc = "Authentication Data"]
pub mod authdata;
#[doc = "HALTSUM2 (rw) register accessor: Halt Summary 2\n\nYou can [`read`](crate::Reg::read) this register and get [`haltsum2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haltsum2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haltsum2`]
module"]
#[doc(alias = "HALTSUM2")]
pub type Haltsum2 = crate::Reg<haltsum2::Haltsum2Spec>;
#[doc = "Halt Summary 2"]
pub mod haltsum2;
#[doc = "HALTSUM3 (rw) register accessor: Halt Summary 3\n\nYou can [`read`](crate::Reg::read) this register and get [`haltsum3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haltsum3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haltsum3`]
module"]
#[doc(alias = "HALTSUM3")]
pub type Haltsum3 = crate::Reg<haltsum3::Haltsum3Spec>;
#[doc = "Halt Summary 3"]
pub mod haltsum3;
#[doc = "SBADDRESS3 (rw) register accessor: System Bus Addres 127:96\n\nYou can [`read`](crate::Reg::read) this register and get [`sbaddress3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbaddress3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbaddress3`]
module"]
#[doc(alias = "SBADDRESS3")]
pub type Sbaddress3 = crate::Reg<sbaddress3::Sbaddress3Spec>;
#[doc = "System Bus Addres 127:96"]
pub mod sbaddress3;
#[doc = "SBCS (rw) register accessor: System Bus Access Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`sbcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbcs`]
module"]
#[doc(alias = "SBCS")]
pub type Sbcs = crate::Reg<sbcs::SbcsSpec>;
#[doc = "System Bus Access Control and Status"]
pub mod sbcs;
#[doc = "SBADDRESS0 (rw) register accessor: System Bus Addres 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`sbaddress0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbaddress0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbaddress0`]
module"]
#[doc(alias = "SBADDRESS0")]
pub type Sbaddress0 = crate::Reg<sbaddress0::Sbaddress0Spec>;
#[doc = "System Bus Addres 31:0"]
pub mod sbaddress0;
#[doc = "SBADDRESS1 (rw) register accessor: System Bus Addres 63:32\n\nYou can [`read`](crate::Reg::read) this register and get [`sbaddress1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbaddress1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbaddress1`]
module"]
#[doc(alias = "SBADDRESS1")]
pub type Sbaddress1 = crate::Reg<sbaddress1::Sbaddress1Spec>;
#[doc = "System Bus Addres 63:32"]
pub mod sbaddress1;
#[doc = "SBADDRESS2 (rw) register accessor: System Bus Addres 95:64\n\nYou can [`read`](crate::Reg::read) this register and get [`sbaddress2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbaddress2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbaddress2`]
module"]
#[doc(alias = "SBADDRESS2")]
pub type Sbaddress2 = crate::Reg<sbaddress2::Sbaddress2Spec>;
#[doc = "System Bus Addres 95:64"]
pub mod sbaddress2;
#[doc = "SBDATA0 (rw) register accessor: System Bus Data 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`sbdata0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbdata0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbdata0`]
module"]
#[doc(alias = "SBDATA0")]
pub type Sbdata0 = crate::Reg<sbdata0::Sbdata0Spec>;
#[doc = "System Bus Data 31:0"]
pub mod sbdata0;
#[doc = "SBDATA1 (rw) register accessor: System Bus Data 63:32\n\nYou can [`read`](crate::Reg::read) this register and get [`sbdata1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbdata1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbdata1`]
module"]
#[doc(alias = "SBDATA1")]
pub type Sbdata1 = crate::Reg<sbdata1::Sbdata1Spec>;
#[doc = "System Bus Data 63:32"]
pub mod sbdata1;
#[doc = "SBDATA2 (rw) register accessor: System Bus Data 95:64\n\nYou can [`read`](crate::Reg::read) this register and get [`sbdata2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbdata2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbdata2`]
module"]
#[doc(alias = "SBDATA2")]
pub type Sbdata2 = crate::Reg<sbdata2::Sbdata2Spec>;
#[doc = "System Bus Data 95:64"]
pub mod sbdata2;
#[doc = "SBDATA3 (rw) register accessor: System Bus Data 127:96\n\nYou can [`read`](crate::Reg::read) this register and get [`sbdata3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbdata3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbdata3`]
module"]
#[doc(alias = "SBDATA3")]
pub type Sbdata3 = crate::Reg<sbdata3::Sbdata3Spec>;
#[doc = "System Bus Data 127:96"]
pub mod sbdata3;
#[doc = "HALTSUM0 (rw) register accessor: Halt summary 0\n\nYou can [`read`](crate::Reg::read) this register and get [`haltsum0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haltsum0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haltsum0`]
module"]
#[doc(alias = "HALTSUM0")]
pub type Haltsum0 = crate::Reg<haltsum0::Haltsum0Spec>;
#[doc = "Halt summary 0"]
pub mod haltsum0;
