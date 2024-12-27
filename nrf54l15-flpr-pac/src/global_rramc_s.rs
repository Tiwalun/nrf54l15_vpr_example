#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_wakeup: TasksWakeup,
    _reserved1: [u8; 0x04],
    tasks_commitwritebuf: TasksCommitwritebuf,
    _reserved2: [u8; 0x74],
    subscribe_wakeup: SubscribeWakeup,
    _reserved3: [u8; 0x04],
    subscribe_commitwritebuf: SubscribeCommitwritebuf,
    _reserved4: [u8; 0x74],
    events_wokenup: EventsWokenup,
    events_ready: EventsReady,
    events_readynext: EventsReadynext,
    events_accesserror: EventsAccesserror,
    _reserved8: [u8; 0x70],
    publish_wokenup: PublishWokenup,
    _reserved9: [u8; 0x017c],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    intpend: Intpend,
    _reserved13: [u8; 0xf0],
    ready: Ready,
    readynext: Readynext,
    accesserroraddr: Accesserroraddr,
    _reserved16: [u8; 0x04],
    bufstatus: Bufstatus,
    _reserved17: [u8; 0xe4],
    config: Config,
    _reserved18: [u8; 0x08],
    readynexttimeout: Readynexttimeout,
    power: Power,
    _reserved20: [u8; 0x2c],
    erase: Erase,
    _reserved21: [u8; 0x0c],
    region: [Region; 5],
}
impl RegisterBlock {
    #[doc = "0x00 - Wakeup the RRAM from low power mode"]
    #[inline(always)]
    pub const fn tasks_wakeup(&self) -> &TasksWakeup {
        &self.tasks_wakeup
    }
    #[doc = "0x08 - Commits the data stored in internal write-buffer to RRAM"]
    #[inline(always)]
    pub const fn tasks_commitwritebuf(&self) -> &TasksCommitwritebuf {
        &self.tasks_commitwritebuf
    }
    #[doc = "0x80 - Subscribe configuration for task WAKEUP"]
    #[inline(always)]
    pub const fn subscribe_wakeup(&self) -> &SubscribeWakeup {
        &self.subscribe_wakeup
    }
    #[doc = "0x88 - Subscribe configuration for task COMMITWRITEBUF"]
    #[inline(always)]
    pub const fn subscribe_commitwritebuf(&self) -> &SubscribeCommitwritebuf {
        &self.subscribe_commitwritebuf
    }
    #[doc = "0x100 - RRAMC is woken up from low power mode"]
    #[inline(always)]
    pub const fn events_wokenup(&self) -> &EventsWokenup {
        &self.events_wokenup
    }
    #[doc = "0x104 - RRAMC is ready"]
    #[inline(always)]
    pub const fn events_ready(&self) -> &EventsReady {
        &self.events_ready
    }
    #[doc = "0x108 - Ready to accept a new write operation"]
    #[inline(always)]
    pub const fn events_readynext(&self) -> &EventsReadynext {
        &self.events_readynext
    }
    #[doc = "0x10c - RRAM access error"]
    #[inline(always)]
    pub const fn events_accesserror(&self) -> &EventsAccesserror {
        &self.events_accesserror
    }
    #[doc = "0x180 - Publish configuration for event WOKENUP"]
    #[inline(always)]
    pub const fn publish_wokenup(&self) -> &PublishWokenup {
        &self.publish_wokenup
    }
    #[doc = "0x300 - Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x304 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x308 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x30c - Pending interrupts"]
    #[inline(always)]
    pub const fn intpend(&self) -> &Intpend {
        &self.intpend
    }
    #[doc = "0x400 - RRAMC ready status"]
    #[inline(always)]
    pub const fn ready(&self) -> &Ready {
        &self.ready
    }
    #[doc = "0x404 - Ready next flag"]
    #[inline(always)]
    pub const fn readynext(&self) -> &Readynext {
        &self.readynext
    }
    #[doc = "0x408 - Address of the first access error"]
    #[inline(always)]
    pub const fn accesserroraddr(&self) -> &Accesserroraddr {
        &self.accesserroraddr
    }
    #[doc = "0x410..0x41c - Unspecified"]
    #[inline(always)]
    pub const fn bufstatus(&self) -> &Bufstatus {
        &self.bufstatus
    }
    #[doc = "0x500 - Configuration register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x50c - Configuration for ready next timeout counter, in units of AXI clock frequency"]
    #[inline(always)]
    pub const fn readynexttimeout(&self) -> &Readynexttimeout {
        &self.readynexttimeout
    }
    #[doc = "0x510 - Unspecified"]
    #[inline(always)]
    pub const fn power(&self) -> &Power {
        &self.power
    }
    #[doc = "0x540 - Unspecified"]
    #[inline(always)]
    pub const fn erase(&self) -> &Erase {
        &self.erase
    }
    #[doc = "0x550..0x578 - Unspecified"]
    #[inline(always)]
    pub const fn region(&self, n: usize) -> &Region {
        &self.region[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x550..0x578 - Unspecified"]
    #[inline(always)]
    pub fn region_iter(&self) -> impl Iterator<Item = &Region> {
        self.region.iter()
    }
}
#[doc = "TASKS_WAKEUP (w) register accessor: Wakeup the RRAM from low power mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_wakeup::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_wakeup`]
module"]
#[doc(alias = "TASKS_WAKEUP")]
pub type TasksWakeup = crate::Reg<tasks_wakeup::TasksWakeupSpec>;
#[doc = "Wakeup the RRAM from low power mode"]
pub mod tasks_wakeup;
#[doc = "TASKS_COMMITWRITEBUF (w) register accessor: Commits the data stored in internal write-buffer to RRAM\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_commitwritebuf::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_commitwritebuf`]
module"]
#[doc(alias = "TASKS_COMMITWRITEBUF")]
pub type TasksCommitwritebuf = crate::Reg<tasks_commitwritebuf::TasksCommitwritebufSpec>;
#[doc = "Commits the data stored in internal write-buffer to RRAM"]
pub mod tasks_commitwritebuf;
#[doc = "SUBSCRIBE_WAKEUP (rw) register accessor: Subscribe configuration for task WAKEUP\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_wakeup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_wakeup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_wakeup`]
module"]
#[doc(alias = "SUBSCRIBE_WAKEUP")]
pub type SubscribeWakeup = crate::Reg<subscribe_wakeup::SubscribeWakeupSpec>;
#[doc = "Subscribe configuration for task WAKEUP"]
pub mod subscribe_wakeup;
#[doc = "SUBSCRIBE_COMMITWRITEBUF (rw) register accessor: Subscribe configuration for task COMMITWRITEBUF\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_commitwritebuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_commitwritebuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_commitwritebuf`]
module"]
#[doc(alias = "SUBSCRIBE_COMMITWRITEBUF")]
pub type SubscribeCommitwritebuf =
    crate::Reg<subscribe_commitwritebuf::SubscribeCommitwritebufSpec>;
#[doc = "Subscribe configuration for task COMMITWRITEBUF"]
pub mod subscribe_commitwritebuf;
#[doc = "EVENTS_WOKENUP (rw) register accessor: RRAMC is woken up from low power mode\n\nYou can [`read`](crate::Reg::read) this register and get [`events_wokenup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_wokenup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_wokenup`]
module"]
#[doc(alias = "EVENTS_WOKENUP")]
pub type EventsWokenup = crate::Reg<events_wokenup::EventsWokenupSpec>;
#[doc = "RRAMC is woken up from low power mode"]
pub mod events_wokenup;
#[doc = "EVENTS_READY (rw) register accessor: RRAMC is ready\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ready`]
module"]
#[doc(alias = "EVENTS_READY")]
pub type EventsReady = crate::Reg<events_ready::EventsReadySpec>;
#[doc = "RRAMC is ready"]
pub mod events_ready;
#[doc = "EVENTS_READYNEXT (rw) register accessor: Ready to accept a new write operation\n\nYou can [`read`](crate::Reg::read) this register and get [`events_readynext::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_readynext::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_readynext`]
module"]
#[doc(alias = "EVENTS_READYNEXT")]
pub type EventsReadynext = crate::Reg<events_readynext::EventsReadynextSpec>;
#[doc = "Ready to accept a new write operation"]
pub mod events_readynext;
#[doc = "EVENTS_ACCESSERROR (rw) register accessor: RRAM access error\n\nYou can [`read`](crate::Reg::read) this register and get [`events_accesserror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_accesserror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_accesserror`]
module"]
#[doc(alias = "EVENTS_ACCESSERROR")]
pub type EventsAccesserror = crate::Reg<events_accesserror::EventsAccesserrorSpec>;
#[doc = "RRAM access error"]
pub mod events_accesserror;
#[doc = "PUBLISH_WOKENUP (rw) register accessor: Publish configuration for event WOKENUP\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_wokenup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_wokenup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_wokenup`]
module"]
#[doc(alias = "PUBLISH_WOKENUP")]
pub type PublishWokenup = crate::Reg<publish_wokenup::PublishWokenupSpec>;
#[doc = "Publish configuration for event WOKENUP"]
pub mod publish_wokenup;
#[doc = "INTEN (rw) register accessor: Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "INTPEND (r) register accessor: Pending interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`intpend::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpend`]
module"]
#[doc(alias = "INTPEND")]
pub type Intpend = crate::Reg<intpend::IntpendSpec>;
#[doc = "Pending interrupts"]
pub mod intpend;
#[doc = "READY (r) register accessor: RRAMC ready status\n\nYou can [`read`](crate::Reg::read) this register and get [`ready::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ready`]
module"]
#[doc(alias = "READY")]
pub type Ready = crate::Reg<ready::ReadySpec>;
#[doc = "RRAMC ready status"]
pub mod ready;
#[doc = "READYNEXT (r) register accessor: Ready next flag\n\nYou can [`read`](crate::Reg::read) this register and get [`readynext::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readynext`]
module"]
#[doc(alias = "READYNEXT")]
pub type Readynext = crate::Reg<readynext::ReadynextSpec>;
#[doc = "Ready next flag"]
pub mod readynext;
#[doc = "ACCESSERRORADDR (r) register accessor: Address of the first access error\n\nYou can [`read`](crate::Reg::read) this register and get [`accesserroraddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@accesserroraddr`]
module"]
#[doc(alias = "ACCESSERRORADDR")]
pub type Accesserroraddr = crate::Reg<accesserroraddr::AccesserroraddrSpec>;
#[doc = "Address of the first access error"]
pub mod accesserroraddr;
#[doc = "Unspecified"]
pub use self::bufstatus::Bufstatus;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod bufstatus;
#[doc = "CONFIG (rw) register accessor: Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "READYNEXTTIMEOUT (rw) register accessor: Configuration for ready next timeout counter, in units of AXI clock frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`readynexttimeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readynexttimeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readynexttimeout`]
module"]
#[doc(alias = "READYNEXTTIMEOUT")]
pub type Readynexttimeout = crate::Reg<readynexttimeout::ReadynexttimeoutSpec>;
#[doc = "Configuration for ready next timeout counter, in units of AXI clock frequency"]
pub mod readynexttimeout;
#[doc = "Unspecified"]
pub use self::power::Power;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod power;
#[doc = "Unspecified"]
pub use self::erase::Erase;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod erase;
#[doc = "Unspecified"]
pub use self::region::Region;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod region;
