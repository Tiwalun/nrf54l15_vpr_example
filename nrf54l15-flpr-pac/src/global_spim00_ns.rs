#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    _reserved2: [u8; 0x04],
    tasks_suspend: TasksSuspend,
    tasks_resume: TasksResume,
    _reserved4: [u8; 0x14],
    tasks_dma: TasksDma,
    _reserved5: [u8; 0x30],
    subscribe_start: SubscribeStart,
    subscribe_stop: SubscribeStop,
    _reserved7: [u8; 0x04],
    subscribe_suspend: SubscribeSuspend,
    subscribe_resume: SubscribeResume,
    _reserved9: [u8; 0x14],
    subscribe_dma: SubscribeDma,
    _reserved10: [u8; 0x30],
    events_started: EventsStarted,
    events_stopped: EventsStopped,
    events_end: EventsEnd,
    _reserved13: [u8; 0x40],
    events_dma: EventsDma,
    _reserved14: [u8; 0x0c],
    publish_started: PublishStarted,
    publish_stopped: PublishStopped,
    publish_end: PublishEnd,
    _reserved17: [u8; 0x40],
    publish_dma: PublishDma,
    _reserved18: [u8; 0x0c],
    shorts: Shorts,
    _reserved19: [u8; 0x0100],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved21: [u8; 0x01f4],
    enable: Enable,
    _reserved22: [u8; 0x28],
    prescaler: Prescaler,
    _reserved23: [u8; 0x24],
    config: Config,
    _reserved24: [u8; 0x54],
    iftiming: Iftiming,
    dcxcnt: Dcxcnt,
    csnpol: Csnpol,
    _reserved27: [u8; 0x04],
    orc: Orc,
    _reserved28: [u8; 0x3c],
    psel: Psel,
    _reserved29: [u8; 0xec],
    dma: Dma,
}
impl RegisterBlock {
    #[doc = "0x00 - Start SPI transaction"]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x04 - Stop SPI transaction"]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x0c - Suspend SPI transaction"]
    #[inline(always)]
    pub const fn tasks_suspend(&self) -> &TasksSuspend {
        &self.tasks_suspend
    }
    #[doc = "0x10 - Resume SPI transaction"]
    #[inline(always)]
    pub const fn tasks_resume(&self) -> &TasksResume {
        &self.tasks_resume
    }
    #[doc = "0x28..0x50 - Peripheral tasks."]
    #[inline(always)]
    pub const fn tasks_dma(&self) -> &TasksDma {
        &self.tasks_dma
    }
    #[doc = "0x80 - Subscribe configuration for task START"]
    #[inline(always)]
    pub const fn subscribe_start(&self) -> &SubscribeStart {
        &self.subscribe_start
    }
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    #[inline(always)]
    pub const fn subscribe_stop(&self) -> &SubscribeStop {
        &self.subscribe_stop
    }
    #[doc = "0x8c - Subscribe configuration for task SUSPEND"]
    #[inline(always)]
    pub const fn subscribe_suspend(&self) -> &SubscribeSuspend {
        &self.subscribe_suspend
    }
    #[doc = "0x90 - Subscribe configuration for task RESUME"]
    #[inline(always)]
    pub const fn subscribe_resume(&self) -> &SubscribeResume {
        &self.subscribe_resume
    }
    #[doc = "0xa8..0xd0 - Subscribe configuration for tasks"]
    #[inline(always)]
    pub const fn subscribe_dma(&self) -> &SubscribeDma {
        &self.subscribe_dma
    }
    #[doc = "0x100 - SPI transaction has started"]
    #[inline(always)]
    pub const fn events_started(&self) -> &EventsStarted {
        &self.events_started
    }
    #[doc = "0x104 - SPI transaction has stopped"]
    #[inline(always)]
    pub const fn events_stopped(&self) -> &EventsStopped {
        &self.events_stopped
    }
    #[doc = "0x108 - End of RXD buffer and TXD buffer reached"]
    #[inline(always)]
    pub const fn events_end(&self) -> &EventsEnd {
        &self.events_end
    }
    #[doc = "0x14c..0x174 - Peripheral events."]
    #[inline(always)]
    pub const fn events_dma(&self) -> &EventsDma {
        &self.events_dma
    }
    #[doc = "0x180 - Publish configuration for event STARTED"]
    #[inline(always)]
    pub const fn publish_started(&self) -> &PublishStarted {
        &self.publish_started
    }
    #[doc = "0x184 - Publish configuration for event STOPPED"]
    #[inline(always)]
    pub const fn publish_stopped(&self) -> &PublishStopped {
        &self.publish_stopped
    }
    #[doc = "0x188 - Publish configuration for event END"]
    #[inline(always)]
    pub const fn publish_end(&self) -> &PublishEnd {
        &self.publish_end
    }
    #[doc = "0x1cc..0x1f4 - Publish configuration for events"]
    #[inline(always)]
    pub const fn publish_dma(&self) -> &PublishDma {
        &self.publish_dma
    }
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(&self) -> &Shorts {
        &self.shorts
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
    #[doc = "0x500 - Enable SPIM"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x52c - The prescaler is used to set the SPI frequency."]
    #[inline(always)]
    pub const fn prescaler(&self) -> &Prescaler {
        &self.prescaler
    }
    #[doc = "0x554 - Configuration register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x5ac..0x5b4 - Unspecified"]
    #[inline(always)]
    pub const fn iftiming(&self) -> &Iftiming {
        &self.iftiming
    }
    #[doc = "0x5b4 - DCX configuration"]
    #[inline(always)]
    pub const fn dcxcnt(&self) -> &Dcxcnt {
        &self.dcxcnt
    }
    #[doc = "0x5b8 - Polarity of CSN output"]
    #[inline(always)]
    pub const fn csnpol(&self) -> &Csnpol {
        &self.csnpol
    }
    #[doc = "0x5c0 - Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT"]
    #[inline(always)]
    pub const fn orc(&self) -> &Orc {
        &self.orc
    }
    #[doc = "0x600..0x614 - Unspecified"]
    #[inline(always)]
    pub const fn psel(&self) -> &Psel {
        &self.psel
    }
    #[doc = "0x700..0x75c - Unspecified"]
    #[inline(always)]
    pub const fn dma(&self) -> &Dma {
        &self.dma
    }
}
#[doc = "TASKS_START (w) register accessor: Start SPI transaction\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`]
module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Start SPI transaction"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Stop SPI transaction\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`]
module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop SPI transaction"]
pub mod tasks_stop;
#[doc = "TASKS_SUSPEND (w) register accessor: Suspend SPI transaction\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_suspend::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_suspend`]
module"]
#[doc(alias = "TASKS_SUSPEND")]
pub type TasksSuspend = crate::Reg<tasks_suspend::TasksSuspendSpec>;
#[doc = "Suspend SPI transaction"]
pub mod tasks_suspend;
#[doc = "TASKS_RESUME (w) register accessor: Resume SPI transaction\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_resume::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_resume`]
module"]
#[doc(alias = "TASKS_RESUME")]
pub type TasksResume = crate::Reg<tasks_resume::TasksResumeSpec>;
#[doc = "Resume SPI transaction"]
pub mod tasks_resume;
#[doc = "Peripheral tasks."]
pub use self::tasks_dma::TasksDma;
#[doc = r"Cluster"]
#[doc = "Peripheral tasks."]
pub mod tasks_dma;
#[doc = "SUBSCRIBE_START (rw) register accessor: Subscribe configuration for task START\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_start`]
module"]
#[doc(alias = "SUBSCRIBE_START")]
pub type SubscribeStart = crate::Reg<subscribe_start::SubscribeStartSpec>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "SUBSCRIBE_STOP (rw) register accessor: Subscribe configuration for task STOP\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_stop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_stop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_stop`]
module"]
#[doc(alias = "SUBSCRIBE_STOP")]
pub type SubscribeStop = crate::Reg<subscribe_stop::SubscribeStopSpec>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "SUBSCRIBE_SUSPEND (rw) register accessor: Subscribe configuration for task SUSPEND\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_suspend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_suspend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_suspend`]
module"]
#[doc(alias = "SUBSCRIBE_SUSPEND")]
pub type SubscribeSuspend = crate::Reg<subscribe_suspend::SubscribeSuspendSpec>;
#[doc = "Subscribe configuration for task SUSPEND"]
pub mod subscribe_suspend;
#[doc = "SUBSCRIBE_RESUME (rw) register accessor: Subscribe configuration for task RESUME\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_resume::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_resume::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_resume`]
module"]
#[doc(alias = "SUBSCRIBE_RESUME")]
pub type SubscribeResume = crate::Reg<subscribe_resume::SubscribeResumeSpec>;
#[doc = "Subscribe configuration for task RESUME"]
pub mod subscribe_resume;
#[doc = "Subscribe configuration for tasks"]
pub use self::subscribe_dma::SubscribeDma;
#[doc = r"Cluster"]
#[doc = "Subscribe configuration for tasks"]
pub mod subscribe_dma;
#[doc = "EVENTS_STARTED (rw) register accessor: SPI transaction has started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_started::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_started::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_started`]
module"]
#[doc(alias = "EVENTS_STARTED")]
pub type EventsStarted = crate::Reg<events_started::EventsStartedSpec>;
#[doc = "SPI transaction has started"]
pub mod events_started;
#[doc = "EVENTS_STOPPED (rw) register accessor: SPI transaction has stopped\n\nYou can [`read`](crate::Reg::read) this register and get [`events_stopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_stopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_stopped`]
module"]
#[doc(alias = "EVENTS_STOPPED")]
pub type EventsStopped = crate::Reg<events_stopped::EventsStoppedSpec>;
#[doc = "SPI transaction has stopped"]
pub mod events_stopped;
#[doc = "EVENTS_END (rw) register accessor: End of RXD buffer and TXD buffer reached\n\nYou can [`read`](crate::Reg::read) this register and get [`events_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_end`]
module"]
#[doc(alias = "EVENTS_END")]
pub type EventsEnd = crate::Reg<events_end::EventsEndSpec>;
#[doc = "End of RXD buffer and TXD buffer reached"]
pub mod events_end;
#[doc = "Peripheral events."]
pub use self::events_dma::EventsDma;
#[doc = r"Cluster"]
#[doc = "Peripheral events."]
pub mod events_dma;
#[doc = "PUBLISH_STARTED (rw) register accessor: Publish configuration for event STARTED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_started::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_started::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_started`]
module"]
#[doc(alias = "PUBLISH_STARTED")]
pub type PublishStarted = crate::Reg<publish_started::PublishStartedSpec>;
#[doc = "Publish configuration for event STARTED"]
pub mod publish_started;
#[doc = "PUBLISH_STOPPED (rw) register accessor: Publish configuration for event STOPPED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_stopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_stopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_stopped`]
module"]
#[doc(alias = "PUBLISH_STOPPED")]
pub type PublishStopped = crate::Reg<publish_stopped::PublishStoppedSpec>;
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "PUBLISH_END (rw) register accessor: Publish configuration for event END\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_end`]
module"]
#[doc(alias = "PUBLISH_END")]
pub type PublishEnd = crate::Reg<publish_end::PublishEndSpec>;
#[doc = "Publish configuration for event END"]
pub mod publish_end;
#[doc = "Publish configuration for events"]
pub use self::publish_dma::PublishDma;
#[doc = r"Cluster"]
#[doc = "Publish configuration for events"]
pub mod publish_dma;
#[doc = "SHORTS (rw) register accessor: Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`]
module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
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
#[doc = "ENABLE (rw) register accessor: Enable SPIM\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable SPIM"]
pub mod enable;
#[doc = "PRESCALER (rw) register accessor: The prescaler is used to set the SPI frequency.\n\nYou can [`read`](crate::Reg::read) this register and get [`prescaler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prescaler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prescaler`]
module"]
#[doc(alias = "PRESCALER")]
pub type Prescaler = crate::Reg<prescaler::PrescalerSpec>;
#[doc = "The prescaler is used to set the SPI frequency."]
pub mod prescaler;
#[doc = "CONFIG (rw) register accessor: Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "Unspecified"]
pub use self::iftiming::Iftiming;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod iftiming;
#[doc = "DCXCNT (rw) register accessor: DCX configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dcxcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcxcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcxcnt`]
module"]
#[doc(alias = "DCXCNT")]
pub type Dcxcnt = crate::Reg<dcxcnt::DcxcntSpec>;
#[doc = "DCX configuration"]
pub mod dcxcnt;
#[doc = "CSNPOL (rw) register accessor: Polarity of CSN output\n\nYou can [`read`](crate::Reg::read) this register and get [`csnpol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csnpol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csnpol`]
module"]
#[doc(alias = "CSNPOL")]
pub type Csnpol = crate::Reg<csnpol::CsnpolSpec>;
#[doc = "Polarity of CSN output"]
pub mod csnpol;
#[doc = "ORC (rw) register accessor: Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT\n\nYou can [`read`](crate::Reg::read) this register and get [`orc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`orc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@orc`]
module"]
#[doc(alias = "ORC")]
pub type Orc = crate::Reg<orc::OrcSpec>;
#[doc = "Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT"]
pub mod orc;
#[doc = "Unspecified"]
pub use self::psel::Psel;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "Unspecified"]
pub use self::dma::Dma;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod dma;
