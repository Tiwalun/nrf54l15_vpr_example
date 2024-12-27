#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    _reserved2: [u8; 0x78],
    subscribe_start: SubscribeStart,
    subscribe_stop: SubscribeStop,
    _reserved4: [u8; 0x78],
    events_end: EventsEnd,
    events_resolved: EventsResolved,
    events_notresolved: EventsNotresolved,
    events_error: EventsError,
    _reserved8: [u8; 0x70],
    publish_end: PublishEnd,
    publish_resolved: PublishResolved,
    publish_notresolved: PublishNotresolved,
    publish_error: PublishError,
    _reserved12: [u8; 0x0174],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved14: [u8; 0xf8],
    errorstatus: Errorstatus,
    _reserved15: [u8; 0xf8],
    enable: Enable,
    _reserved16: [u8; 0x04],
    maxresolved: Maxresolved,
    _reserved17: [u8; 0x24],
    in_: In,
    _reserved18: [u8; 0x04],
    out: Out,
}
impl RegisterBlock {
    #[doc = "0x00 - Start resolving addresses based on IRKs specified in the IRK data structure"]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x04 - Stop resolving addresses"]
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
    #[doc = "0x100 - Address resolution procedure complete or ended due to an error"]
    #[inline(always)]
    pub const fn events_end(&self) -> &EventsEnd {
        &self.events_end
    }
    #[doc = "0x104 - Address resolved"]
    #[inline(always)]
    pub const fn events_resolved(&self) -> &EventsResolved {
        &self.events_resolved
    }
    #[doc = "0x108 - Address not resolved"]
    #[inline(always)]
    pub const fn events_notresolved(&self) -> &EventsNotresolved {
        &self.events_notresolved
    }
    #[doc = "0x10c - Operation aborted because of a STOP task or due to an error"]
    #[inline(always)]
    pub const fn events_error(&self) -> &EventsError {
        &self.events_error
    }
    #[doc = "0x180 - Publish configuration for event END"]
    #[inline(always)]
    pub const fn publish_end(&self) -> &PublishEnd {
        &self.publish_end
    }
    #[doc = "0x184 - Publish configuration for event RESOLVED"]
    #[inline(always)]
    pub const fn publish_resolved(&self) -> &PublishResolved {
        &self.publish_resolved
    }
    #[doc = "0x188 - Publish configuration for event NOTRESOLVED"]
    #[inline(always)]
    pub const fn publish_notresolved(&self) -> &PublishNotresolved {
        &self.publish_notresolved
    }
    #[doc = "0x18c - Publish configuration for event ERROR"]
    #[inline(always)]
    pub const fn publish_error(&self) -> &PublishError {
        &self.publish_error
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
    #[doc = "0x404 - Error status"]
    #[inline(always)]
    pub const fn errorstatus(&self) -> &Errorstatus {
        &self.errorstatus
    }
    #[doc = "0x500 - Enable AAR"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x508 - Maximum number of IRKs to resolve"]
    #[inline(always)]
    pub const fn maxresolved(&self) -> &Maxresolved {
        &self.maxresolved
    }
    #[doc = "0x530 - IN EasyDMA channel"]
    #[inline(always)]
    pub const fn in_(&self) -> &In {
        &self.in_
    }
    #[doc = "0x538..0x540 - OUT EasyDMA channel"]
    #[inline(always)]
    pub const fn out(&self) -> &Out {
        &self.out
    }
}
#[doc = "TASKS_START (w) register accessor: Start resolving addresses based on IRKs specified in the IRK data structure\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`]
module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Start resolving addresses based on IRKs specified in the IRK data structure"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Stop resolving addresses\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`]
module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop resolving addresses"]
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
#[doc = "EVENTS_END (rw) register accessor: Address resolution procedure complete or ended due to an error\n\nYou can [`read`](crate::Reg::read) this register and get [`events_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_end`]
module"]
#[doc(alias = "EVENTS_END")]
pub type EventsEnd = crate::Reg<events_end::EventsEndSpec>;
#[doc = "Address resolution procedure complete or ended due to an error"]
pub mod events_end;
#[doc = "EVENTS_RESOLVED (rw) register accessor: Address resolved\n\nYou can [`read`](crate::Reg::read) this register and get [`events_resolved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_resolved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_resolved`]
module"]
#[doc(alias = "EVENTS_RESOLVED")]
pub type EventsResolved = crate::Reg<events_resolved::EventsResolvedSpec>;
#[doc = "Address resolved"]
pub mod events_resolved;
#[doc = "EVENTS_NOTRESOLVED (rw) register accessor: Address not resolved\n\nYou can [`read`](crate::Reg::read) this register and get [`events_notresolved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_notresolved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_notresolved`]
module"]
#[doc(alias = "EVENTS_NOTRESOLVED")]
pub type EventsNotresolved = crate::Reg<events_notresolved::EventsNotresolvedSpec>;
#[doc = "Address not resolved"]
pub mod events_notresolved;
#[doc = "EVENTS_ERROR (rw) register accessor: Operation aborted because of a STOP task or due to an error\n\nYou can [`read`](crate::Reg::read) this register and get [`events_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_error`]
module"]
#[doc(alias = "EVENTS_ERROR")]
pub type EventsError = crate::Reg<events_error::EventsErrorSpec>;
#[doc = "Operation aborted because of a STOP task or due to an error"]
pub mod events_error;
#[doc = "PUBLISH_END (rw) register accessor: Publish configuration for event END\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_end`]
module"]
#[doc(alias = "PUBLISH_END")]
pub type PublishEnd = crate::Reg<publish_end::PublishEndSpec>;
#[doc = "Publish configuration for event END"]
pub mod publish_end;
#[doc = "PUBLISH_RESOLVED (rw) register accessor: Publish configuration for event RESOLVED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_resolved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_resolved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_resolved`]
module"]
#[doc(alias = "PUBLISH_RESOLVED")]
pub type PublishResolved = crate::Reg<publish_resolved::PublishResolvedSpec>;
#[doc = "Publish configuration for event RESOLVED"]
pub mod publish_resolved;
#[doc = "PUBLISH_NOTRESOLVED (rw) register accessor: Publish configuration for event NOTRESOLVED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_notresolved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_notresolved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_notresolved`]
module"]
#[doc(alias = "PUBLISH_NOTRESOLVED")]
pub type PublishNotresolved = crate::Reg<publish_notresolved::PublishNotresolvedSpec>;
#[doc = "Publish configuration for event NOTRESOLVED"]
pub mod publish_notresolved;
#[doc = "PUBLISH_ERROR (rw) register accessor: Publish configuration for event ERROR\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_error`]
module"]
#[doc(alias = "PUBLISH_ERROR")]
pub type PublishError = crate::Reg<publish_error::PublishErrorSpec>;
#[doc = "Publish configuration for event ERROR"]
pub mod publish_error;
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
#[doc = "ERRORSTATUS (r) register accessor: Error status\n\nYou can [`read`](crate::Reg::read) this register and get [`errorstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errorstatus`]
module"]
#[doc(alias = "ERRORSTATUS")]
pub type Errorstatus = crate::Reg<errorstatus::ErrorstatusSpec>;
#[doc = "Error status"]
pub mod errorstatus;
#[doc = "ENABLE (rw) register accessor: Enable AAR\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable AAR"]
pub mod enable;
#[doc = "MAXRESOLVED (rw) register accessor: Maximum number of IRKs to resolve\n\nYou can [`read`](crate::Reg::read) this register and get [`maxresolved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxresolved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxresolved`]
module"]
#[doc(alias = "MAXRESOLVED")]
pub type Maxresolved = crate::Reg<maxresolved::MaxresolvedSpec>;
#[doc = "Maximum number of IRKs to resolve"]
pub mod maxresolved;
#[doc = "IN EasyDMA channel"]
pub use self::in_::In;
#[doc = r"Cluster"]
#[doc = "IN EasyDMA channel"]
pub mod in_;
#[doc = "OUT EasyDMA channel"]
pub use self::out::Out;
#[doc = r"Cluster"]
#[doc = "OUT EasyDMA channel"]
pub mod out;
