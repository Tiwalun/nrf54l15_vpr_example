#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    tasks_stop: TasksStop,
    _reserved1: [u8; 0x04],
    tasks_suspend: TasksSuspend,
    tasks_resume: TasksResume,
    _reserved3: [u8; 0x14],
    tasks_dma: TasksDma,
    _reserved4: [u8; 0x2c],
    subscribe_stop: SubscribeStop,
    _reserved5: [u8; 0x04],
    subscribe_suspend: SubscribeSuspend,
    subscribe_resume: SubscribeResume,
    _reserved7: [u8; 0x14],
    subscribe_dma: SubscribeDma,
    _reserved8: [u8; 0x2c],
    events_stopped: EventsStopped,
    _reserved9: [u8; 0x0c],
    events_error: EventsError,
    _reserved10: [u8; 0x10],
    events_suspended: EventsSuspended,
    _reserved11: [u8; 0x08],
    events_lastrx: EventsLastrx,
    events_lasttx: EventsLasttx,
    _reserved13: [u8; 0x10],
    events_dma: EventsDma,
    _reserved14: [u8; 0x10],
    publish_stopped: PublishStopped,
    _reserved15: [u8; 0x0c],
    publish_error: PublishError,
    _reserved16: [u8; 0x10],
    publish_suspended: PublishSuspended,
    _reserved17: [u8; 0x08],
    publish_lastrx: PublishLastrx,
    publish_lasttx: PublishLasttx,
    _reserved19: [u8; 0x10],
    publish_dma: PublishDma,
    _reserved20: [u8; 0x0c],
    shorts: Shorts,
    _reserved21: [u8; 0xfc],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved24: [u8; 0x01b8],
    errorsrc: Errorsrc,
    _reserved25: [u8; 0x38],
    enable: Enable,
    _reserved26: [u8; 0x20],
    frequency: Frequency,
    _reserved27: [u8; 0x60],
    address: Address,
    _reserved28: [u8; 0x74],
    psel: Psel,
    _reserved29: [u8; 0xf8],
    dma: Dma,
}
impl RegisterBlock {
    #[doc = "0x04 - Stop TWI transaction. Must be issued while the TWI master is not suspended."]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x0c - Suspend TWI transaction"]
    #[inline(always)]
    pub const fn tasks_suspend(&self) -> &TasksSuspend {
        &self.tasks_suspend
    }
    #[doc = "0x10 - Resume TWI transaction"]
    #[inline(always)]
    pub const fn tasks_resume(&self) -> &TasksResume {
        &self.tasks_resume
    }
    #[doc = "0x28..0x58 - Peripheral tasks."]
    #[inline(always)]
    pub const fn tasks_dma(&self) -> &TasksDma {
        &self.tasks_dma
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
    #[doc = "0xa8..0xd8 - Subscribe configuration for tasks"]
    #[inline(always)]
    pub const fn subscribe_dma(&self) -> &SubscribeDma {
        &self.subscribe_dma
    }
    #[doc = "0x104 - TWI stopped"]
    #[inline(always)]
    pub const fn events_stopped(&self) -> &EventsStopped {
        &self.events_stopped
    }
    #[doc = "0x114 - TWI error"]
    #[inline(always)]
    pub const fn events_error(&self) -> &EventsError {
        &self.events_error
    }
    #[doc = "0x128 - SUSPEND task has been issued, TWI traffic is now suspended."]
    #[inline(always)]
    pub const fn events_suspended(&self) -> &EventsSuspended {
        &self.events_suspended
    }
    #[doc = "0x134 - Byte boundary, starting to receive the last byte"]
    #[inline(always)]
    pub const fn events_lastrx(&self) -> &EventsLastrx {
        &self.events_lastrx
    }
    #[doc = "0x138 - Byte boundary, starting to transmit the last byte"]
    #[inline(always)]
    pub const fn events_lasttx(&self) -> &EventsLasttx {
        &self.events_lasttx
    }
    #[doc = "0x14c..0x174 - Peripheral events."]
    #[inline(always)]
    pub const fn events_dma(&self) -> &EventsDma {
        &self.events_dma
    }
    #[doc = "0x184 - Publish configuration for event STOPPED"]
    #[inline(always)]
    pub const fn publish_stopped(&self) -> &PublishStopped {
        &self.publish_stopped
    }
    #[doc = "0x194 - Publish configuration for event ERROR"]
    #[inline(always)]
    pub const fn publish_error(&self) -> &PublishError {
        &self.publish_error
    }
    #[doc = "0x1a8 - Publish configuration for event SUSPENDED"]
    #[inline(always)]
    pub const fn publish_suspended(&self) -> &PublishSuspended {
        &self.publish_suspended
    }
    #[doc = "0x1b4 - Publish configuration for event LASTRX"]
    #[inline(always)]
    pub const fn publish_lastrx(&self) -> &PublishLastrx {
        &self.publish_lastrx
    }
    #[doc = "0x1b8 - Publish configuration for event LASTTX"]
    #[inline(always)]
    pub const fn publish_lasttx(&self) -> &PublishLasttx {
        &self.publish_lasttx
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
    #[doc = "0x4c4 - Error source"]
    #[inline(always)]
    pub const fn errorsrc(&self) -> &Errorsrc {
        &self.errorsrc
    }
    #[doc = "0x500 - Enable TWIM"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x524 - TWI frequency. Accuracy depends on the HFCLK source selected."]
    #[inline(always)]
    pub const fn frequency(&self) -> &Frequency {
        &self.frequency
    }
    #[doc = "0x588 - Address used in the TWI transfer"]
    #[inline(always)]
    pub const fn address(&self) -> &Address {
        &self.address
    }
    #[doc = "0x600..0x608 - Unspecified"]
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
#[doc = "TASKS_STOP (w) register accessor: Stop TWI transaction. Must be issued while the TWI master is not suspended.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`]
module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop TWI transaction. Must be issued while the TWI master is not suspended."]
pub mod tasks_stop;
#[doc = "TASKS_SUSPEND (w) register accessor: Suspend TWI transaction\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_suspend::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_suspend`]
module"]
#[doc(alias = "TASKS_SUSPEND")]
pub type TasksSuspend = crate::Reg<tasks_suspend::TasksSuspendSpec>;
#[doc = "Suspend TWI transaction"]
pub mod tasks_suspend;
#[doc = "TASKS_RESUME (w) register accessor: Resume TWI transaction\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_resume::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_resume`]
module"]
#[doc(alias = "TASKS_RESUME")]
pub type TasksResume = crate::Reg<tasks_resume::TasksResumeSpec>;
#[doc = "Resume TWI transaction"]
pub mod tasks_resume;
#[doc = "Peripheral tasks."]
pub use self::tasks_dma::TasksDma;
#[doc = r"Cluster"]
#[doc = "Peripheral tasks."]
pub mod tasks_dma;
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
#[doc = "EVENTS_STOPPED (rw) register accessor: TWI stopped\n\nYou can [`read`](crate::Reg::read) this register and get [`events_stopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_stopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_stopped`]
module"]
#[doc(alias = "EVENTS_STOPPED")]
pub type EventsStopped = crate::Reg<events_stopped::EventsStoppedSpec>;
#[doc = "TWI stopped"]
pub mod events_stopped;
#[doc = "EVENTS_ERROR (rw) register accessor: TWI error\n\nYou can [`read`](crate::Reg::read) this register and get [`events_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_error`]
module"]
#[doc(alias = "EVENTS_ERROR")]
pub type EventsError = crate::Reg<events_error::EventsErrorSpec>;
#[doc = "TWI error"]
pub mod events_error;
#[doc = "EVENTS_SUSPENDED (rw) register accessor: SUSPEND task has been issued, TWI traffic is now suspended.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_suspended::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_suspended::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_suspended`]
module"]
#[doc(alias = "EVENTS_SUSPENDED")]
pub type EventsSuspended = crate::Reg<events_suspended::EventsSuspendedSpec>;
#[doc = "SUSPEND task has been issued, TWI traffic is now suspended."]
pub mod events_suspended;
#[doc = "EVENTS_LASTRX (rw) register accessor: Byte boundary, starting to receive the last byte\n\nYou can [`read`](crate::Reg::read) this register and get [`events_lastrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_lastrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_lastrx`]
module"]
#[doc(alias = "EVENTS_LASTRX")]
pub type EventsLastrx = crate::Reg<events_lastrx::EventsLastrxSpec>;
#[doc = "Byte boundary, starting to receive the last byte"]
pub mod events_lastrx;
#[doc = "EVENTS_LASTTX (rw) register accessor: Byte boundary, starting to transmit the last byte\n\nYou can [`read`](crate::Reg::read) this register and get [`events_lasttx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_lasttx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_lasttx`]
module"]
#[doc(alias = "EVENTS_LASTTX")]
pub type EventsLasttx = crate::Reg<events_lasttx::EventsLasttxSpec>;
#[doc = "Byte boundary, starting to transmit the last byte"]
pub mod events_lasttx;
#[doc = "Peripheral events."]
pub use self::events_dma::EventsDma;
#[doc = r"Cluster"]
#[doc = "Peripheral events."]
pub mod events_dma;
#[doc = "PUBLISH_STOPPED (rw) register accessor: Publish configuration for event STOPPED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_stopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_stopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_stopped`]
module"]
#[doc(alias = "PUBLISH_STOPPED")]
pub type PublishStopped = crate::Reg<publish_stopped::PublishStoppedSpec>;
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "PUBLISH_ERROR (rw) register accessor: Publish configuration for event ERROR\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_error`]
module"]
#[doc(alias = "PUBLISH_ERROR")]
pub type PublishError = crate::Reg<publish_error::PublishErrorSpec>;
#[doc = "Publish configuration for event ERROR"]
pub mod publish_error;
#[doc = "PUBLISH_SUSPENDED (rw) register accessor: Publish configuration for event SUSPENDED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_suspended::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_suspended::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_suspended`]
module"]
#[doc(alias = "PUBLISH_SUSPENDED")]
pub type PublishSuspended = crate::Reg<publish_suspended::PublishSuspendedSpec>;
#[doc = "Publish configuration for event SUSPENDED"]
pub mod publish_suspended;
#[doc = "PUBLISH_LASTRX (rw) register accessor: Publish configuration for event LASTRX\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_lastrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_lastrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_lastrx`]
module"]
#[doc(alias = "PUBLISH_LASTRX")]
pub type PublishLastrx = crate::Reg<publish_lastrx::PublishLastrxSpec>;
#[doc = "Publish configuration for event LASTRX"]
pub mod publish_lastrx;
#[doc = "PUBLISH_LASTTX (rw) register accessor: Publish configuration for event LASTTX\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_lasttx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_lasttx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_lasttx`]
module"]
#[doc(alias = "PUBLISH_LASTTX")]
pub type PublishLasttx = crate::Reg<publish_lasttx::PublishLasttxSpec>;
#[doc = "Publish configuration for event LASTTX"]
pub mod publish_lasttx;
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
#[doc = "ERRORSRC (rw) register accessor: Error source\n\nYou can [`read`](crate::Reg::read) this register and get [`errorsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errorsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errorsrc`]
module"]
#[doc(alias = "ERRORSRC")]
pub type Errorsrc = crate::Reg<errorsrc::ErrorsrcSpec>;
#[doc = "Error source"]
pub mod errorsrc;
#[doc = "ENABLE (rw) register accessor: Enable TWIM\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable TWIM"]
pub mod enable;
#[doc = "FREQUENCY (rw) register accessor: TWI frequency. Accuracy depends on the HFCLK source selected.\n\nYou can [`read`](crate::Reg::read) this register and get [`frequency::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frequency::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frequency`]
module"]
#[doc(alias = "FREQUENCY")]
pub type Frequency = crate::Reg<frequency::FrequencySpec>;
#[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
pub mod frequency;
#[doc = "ADDRESS (rw) register accessor: Address used in the TWI transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address`]
module"]
#[doc(alias = "ADDRESS")]
pub type Address = crate::Reg<address::AddressSpec>;
#[doc = "Address used in the TWI transfer"]
pub mod address;
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
