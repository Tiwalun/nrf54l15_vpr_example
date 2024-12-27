#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PK")]
pub struct Pk {
    pointers: Pointers,
    command: Command,
    control: Control,
    status: Status,
    _reserved4: [u8; 0x04],
    timer: Timer,
    hwconfig: Hwconfig,
    opsize: Opsize,
    _reserved7: [u8; 0x20],
    ramerrorinject: Ramerrorinject,
    ramerrorstatus: Ramerrorstatus,
}
impl Pk {
    #[doc = "0x00 - Pointers register."]
    #[inline(always)]
    pub const fn pointers(&self) -> &Pointers {
        &self.pointers
    }
    #[doc = "0x04 - Command register."]
    #[inline(always)]
    pub const fn command(&self) -> &Command {
        &self.command
    }
    #[doc = "0x08 - Command register."]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x0c - Status register."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x14 - Timer register."]
    #[inline(always)]
    pub const fn timer(&self) -> &Timer {
        &self.timer
    }
    #[doc = "0x18 - Hardware configuration register."]
    #[inline(always)]
    pub const fn hwconfig(&self) -> &Hwconfig {
        &self.hwconfig
    }
    #[doc = "0x1c - Operand size register."]
    #[inline(always)]
    pub const fn opsize(&self) -> &Opsize {
        &self.opsize
    }
    #[doc = "0x40 - RAM error injection register."]
    #[inline(always)]
    pub const fn ramerrorinject(&self) -> &Ramerrorinject {
        &self.ramerrorinject
    }
    #[doc = "0x44 - RAM error status register."]
    #[inline(always)]
    pub const fn ramerrorstatus(&self) -> &Ramerrorstatus {
        &self.ramerrorstatus
    }
}
#[doc = "POINTERS (rw) register accessor: Pointers register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pointers::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pointers::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pointers`]
module"]
#[doc(alias = "POINTERS")]
pub type Pointers = crate::Reg<pointers::PointersSpec>;
#[doc = "Pointers register."]
pub mod pointers;
#[doc = "COMMAND (rw) register accessor: Command register.\n\nYou can [`read`](crate::Reg::read) this register and get [`command::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command`]
module"]
#[doc(alias = "COMMAND")]
pub type Command = crate::Reg<command::CommandSpec>;
#[doc = "Command register."]
pub mod command;
#[doc = "CONTROL (rw) register accessor: Command register.\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Command register."]
pub mod control;
#[doc = "STATUS (rw) register accessor: Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status register."]
pub mod status;
#[doc = "TIMER (rw) register accessor: Timer register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer`]
module"]
#[doc(alias = "TIMER")]
pub type Timer = crate::Reg<timer::TimerSpec>;
#[doc = "Timer register."]
pub mod timer;
#[doc = "HWCONFIG (rw) register accessor: Hardware configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hwconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwconfig`]
module"]
#[doc(alias = "HWCONFIG")]
pub type Hwconfig = crate::Reg<hwconfig::HwconfigSpec>;
#[doc = "Hardware configuration register."]
pub mod hwconfig;
#[doc = "OPSIZE (rw) register accessor: Operand size register.\n\nYou can [`read`](crate::Reg::read) this register and get [`opsize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opsize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opsize`]
module"]
#[doc(alias = "OPSIZE")]
pub type Opsize = crate::Reg<opsize::OpsizeSpec>;
#[doc = "Operand size register."]
pub mod opsize;
#[doc = "RAMERRORINJECT (rw) register accessor: RAM error injection register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ramerrorinject::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramerrorinject::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ramerrorinject`]
module"]
#[doc(alias = "RAMERRORINJECT")]
pub type Ramerrorinject = crate::Reg<ramerrorinject::RamerrorinjectSpec>;
#[doc = "RAM error injection register."]
pub mod ramerrorinject;
#[doc = "RAMERRORSTATUS (rw) register accessor: RAM error status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ramerrorstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramerrorstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ramerrorstatus`]
module"]
#[doc(alias = "RAMERRORSTATUS")]
pub type Ramerrorstatus = crate::Reg<ramerrorstatus::RamerrorstatusSpec>;
#[doc = "RAM error status register."]
pub mod ramerrorstatus;
