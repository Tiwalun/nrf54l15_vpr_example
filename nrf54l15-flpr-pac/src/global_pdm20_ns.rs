#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    _reserved2: [u8; 0x78],
    subscribe_start: SubscribeStart,
    subscribe_stop: SubscribeStop,
    _reserved4: [u8; 0x78],
    events_started: EventsStarted,
    events_stopped: EventsStopped,
    events_end: EventsEnd,
    _reserved7: [u8; 0x04],
    events_dma: EventsDma,
    _reserved8: [u8; 0x6c],
    publish_started: PublishStarted,
    publish_stopped: PublishStopped,
    publish_end: PublishEnd,
    _reserved11: [u8; 0x04],
    publish_dma: PublishDma,
    _reserved12: [u8; 0x6c],
    shorts: Shorts,
    _reserved13: [u8; 0xfc],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    intpend: Intpend,
    _reserved17: [u8; 0x01f0],
    enable: Enable,
    _reserved18: [u8; 0x04],
    mode: Mode,
    _reserved19: [u8; 0x0c],
    gainl: Gainl,
    gainr: Gainr,
    ratio: Ratio,
    _reserved22: [u8; 0x1c],
    psel: Psel,
    _reserved23: [u8; 0x18],
    sample: Sample,
    _reserved24: [u8; 0x18],
    prescaler: Prescaler,
    _reserved25: [u8; 0x017c],
    dma: Dma,
}
impl RegisterBlock {
    #[doc = "0x00 - Starts continuous PDM transfer"]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x04 - Stops PDM transfer"]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
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
    #[doc = "0x100 - PDM transfer has started"]
    #[inline(always)]
    pub const fn events_started(&self) -> &EventsStarted {
        &self.events_started
    }
    #[doc = "0x104 - PDM transfer has finished"]
    #[inline(always)]
    pub const fn events_stopped(&self) -> &EventsStopped {
        &self.events_stopped
    }
    #[doc = "0x108 - The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM"]
    #[inline(always)]
    pub const fn events_end(&self) -> &EventsEnd {
        &self.events_end
    }
    #[doc = "0x110 - Peripheral events."]
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
    #[doc = "0x190 - Publish configuration for events"]
    #[inline(always)]
    pub const fn publish_dma(&self) -> &PublishDma {
        &self.publish_dma
    }
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(&self) -> &Shorts {
        &self.shorts
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
    #[doc = "0x500 - PDM module enable register"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x508 - Defines the routing of the connected PDM microphone signals"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x518 - Left output gain adjustment"]
    #[inline(always)]
    pub const fn gainl(&self) -> &Gainl {
        &self.gainl
    }
    #[doc = "0x51c - Right output gain adjustment"]
    #[inline(always)]
    pub const fn gainr(&self) -> &Gainr {
        &self.gainr
    }
    #[doc = "0x520 - Selects the decimation ratio between PDM_CLK and output sample rate. Change PRESCALER accordingly."]
    #[inline(always)]
    pub const fn ratio(&self) -> &Ratio {
        &self.ratio
    }
    #[doc = "0x540..0x548 - Unspecified"]
    #[inline(always)]
    pub const fn psel(&self) -> &Psel {
        &self.psel
    }
    #[doc = "0x560..0x568 - Unspecified"]
    #[inline(always)]
    pub const fn sample(&self) -> &Sample {
        &self.sample
    }
    #[doc = "0x580 - The prescaler is used to set the PDM frequency"]
    #[inline(always)]
    pub const fn prescaler(&self) -> &Prescaler {
        &self.prescaler
    }
    #[doc = "0x700..0x708 - Unspecified"]
    #[inline(always)]
    pub const fn dma(&self) -> &Dma {
        &self.dma
    }
}
#[doc = "TASKS_START (w) register accessor: Starts continuous PDM transfer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`]
module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Starts continuous PDM transfer"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Stops PDM transfer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`]
module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stops PDM transfer"]
pub mod tasks_stop;
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
#[doc = "EVENTS_STARTED (rw) register accessor: PDM transfer has started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_started::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_started::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_started`]
module"]
#[doc(alias = "EVENTS_STARTED")]
pub type EventsStarted = crate::Reg<events_started::EventsStartedSpec>;
#[doc = "PDM transfer has started"]
pub mod events_started;
#[doc = "EVENTS_STOPPED (rw) register accessor: PDM transfer has finished\n\nYou can [`read`](crate::Reg::read) this register and get [`events_stopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_stopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_stopped`]
module"]
#[doc(alias = "EVENTS_STOPPED")]
pub type EventsStopped = crate::Reg<events_stopped::EventsStoppedSpec>;
#[doc = "PDM transfer has finished"]
pub mod events_stopped;
#[doc = "EVENTS_END (rw) register accessor: The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM\n\nYou can [`read`](crate::Reg::read) this register and get [`events_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_end`]
module"]
#[doc(alias = "EVENTS_END")]
pub type EventsEnd = crate::Reg<events_end::EventsEndSpec>;
#[doc = "The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM"]
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
#[doc = "ENABLE (rw) register accessor: PDM module enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "PDM module enable register"]
pub mod enable;
#[doc = "MODE (rw) register accessor: Defines the routing of the connected PDM microphone signals\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`]
module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "Defines the routing of the connected PDM microphone signals"]
pub mod mode;
#[doc = "GAINL (rw) register accessor: Left output gain adjustment\n\nYou can [`read`](crate::Reg::read) this register and get [`gainl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gainl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gainl`]
module"]
#[doc(alias = "GAINL")]
pub type Gainl = crate::Reg<gainl::GainlSpec>;
#[doc = "Left output gain adjustment"]
pub mod gainl;
#[doc = "GAINR (rw) register accessor: Right output gain adjustment\n\nYou can [`read`](crate::Reg::read) this register and get [`gainr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gainr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gainr`]
module"]
#[doc(alias = "GAINR")]
pub type Gainr = crate::Reg<gainr::GainrSpec>;
#[doc = "Right output gain adjustment"]
pub mod gainr;
#[doc = "RATIO (rw) register accessor: Selects the decimation ratio between PDM_CLK and output sample rate. Change PRESCALER accordingly.\n\nYou can [`read`](crate::Reg::read) this register and get [`ratio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ratio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ratio`]
module"]
#[doc(alias = "RATIO")]
pub type Ratio = crate::Reg<ratio::RatioSpec>;
#[doc = "Selects the decimation ratio between PDM_CLK and output sample rate. Change PRESCALER accordingly."]
pub mod ratio;
#[doc = "Unspecified"]
pub use self::psel::Psel;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "Unspecified"]
pub use self::sample::Sample;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod sample;
#[doc = "PRESCALER (rw) register accessor: The prescaler is used to set the PDM frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`prescaler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prescaler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prescaler`]
module"]
#[doc(alias = "PRESCALER")]
pub type Prescaler = crate::Reg<prescaler::PrescalerSpec>;
#[doc = "The prescaler is used to set the PDM frequency"]
pub mod prescaler;
#[doc = "Unspecified"]
pub use self::dma::Dma;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod dma;
