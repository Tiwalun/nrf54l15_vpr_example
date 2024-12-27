#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_trigger: [TasksTrigger; 7],
    _reserved1: [u8; 0x64],
    subscribe_trigger: [SubscribeTrigger; 4],
    _reserved2: [u8; 0x70],
    events_triggered: [EventsTriggered; 7],
    _reserved3: [u8; 0x64],
    publish_triggered: [PublishTriggered; 4],
    _reserved4: [u8; 0x0170],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    intpend: Intpend,
    _reserved8: [u8; 0xf0],
    debugif: Debugif,
    _reserved9: [u8; 0x02fc],
    cpurun: Cpurun,
    _reserved10: [u8; 0x04],
    initpc: Initpc,
}
impl RegisterBlock {
    #[doc = "0x00..0x1c - Description collection: VPR task \\[n\\]
register"]
    #[inline(always)]
    pub const fn tasks_trigger(&self, n: usize) -> &TasksTrigger {
        &self.tasks_trigger[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x1c - Description collection: VPR task \\[n\\]
register"]
    #[inline(always)]
    pub fn tasks_trigger_iter(&self) -> impl Iterator<Item = &TasksTrigger> {
        self.tasks_trigger.iter()
    }
    #[doc = "0x80..0x90 - Description collection: Subscribe configuration for task TASKS_TRIGGER\\[n\\]"]
    #[inline(always)]
    pub const fn subscribe_trigger(&self, n: usize) -> &SubscribeTrigger {
        &self.subscribe_trigger[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x90 - Description collection: Subscribe configuration for task TASKS_TRIGGER\\[n\\]"]
    #[inline(always)]
    pub fn subscribe_trigger_iter(&self) -> impl Iterator<Item = &SubscribeTrigger> {
        self.subscribe_trigger.iter()
    }
    #[doc = "0x100..0x11c - Description collection: VPR event \\[n\\]
register"]
    #[inline(always)]
    pub const fn events_triggered(&self, n: usize) -> &EventsTriggered {
        &self.events_triggered[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x11c - Description collection: VPR event \\[n\\]
register"]
    #[inline(always)]
    pub fn events_triggered_iter(&self) -> impl Iterator<Item = &EventsTriggered> {
        self.events_triggered.iter()
    }
    #[doc = "0x180..0x190 - Description collection: Publish configuration for event EVENTS_TRIGGERED\\[n\\]"]
    #[inline(always)]
    pub const fn publish_triggered(&self, n: usize) -> &PublishTriggered {
        &self.publish_triggered[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x190 - Description collection: Publish configuration for event EVENTS_TRIGGERED\\[n\\]"]
    #[inline(always)]
    pub fn publish_triggered_iter(&self) -> impl Iterator<Item = &PublishTriggered> {
        self.publish_triggered.iter()
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
    #[doc = "0x400..0x504 - Unspecified"]
    #[inline(always)]
    pub const fn debugif(&self) -> &Debugif {
        &self.debugif
    }
    #[doc = "0x800 - State of the CPU after a core reset"]
    #[inline(always)]
    pub const fn cpurun(&self) -> &Cpurun {
        &self.cpurun
    }
    #[doc = "0x808 - Initial value of the PC at CPU start."]
    #[inline(always)]
    pub const fn initpc(&self) -> &Initpc {
        &self.initpc
    }
}
#[doc = "TASKS_TRIGGER (w) register accessor: Description collection: VPR task \\[n\\]
register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_trigger`]
module"]
#[doc(alias = "TASKS_TRIGGER")]
pub type TasksTrigger = crate::Reg<tasks_trigger::TasksTriggerSpec>;
#[doc = "Description collection: VPR task \\[n\\]
register"]
pub mod tasks_trigger;
#[doc = "SUBSCRIBE_TRIGGER (rw) register accessor: Description collection: Subscribe configuration for task TASKS_TRIGGER\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_trigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_trigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_trigger`]
module"]
#[doc(alias = "SUBSCRIBE_TRIGGER")]
pub type SubscribeTrigger = crate::Reg<subscribe_trigger::SubscribeTriggerSpec>;
#[doc = "Description collection: Subscribe configuration for task TASKS_TRIGGER\\[n\\]"]
pub mod subscribe_trigger;
#[doc = "EVENTS_TRIGGERED (rw) register accessor: Description collection: VPR event \\[n\\]
register\n\nYou can [`read`](crate::Reg::read) this register and get [`events_triggered::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_triggered::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_triggered`]
module"]
#[doc(alias = "EVENTS_TRIGGERED")]
pub type EventsTriggered = crate::Reg<events_triggered::EventsTriggeredSpec>;
#[doc = "Description collection: VPR event \\[n\\]
register"]
pub mod events_triggered;
#[doc = "PUBLISH_TRIGGERED (rw) register accessor: Description collection: Publish configuration for event EVENTS_TRIGGERED\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_triggered::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_triggered::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_triggered`]
module"]
#[doc(alias = "PUBLISH_TRIGGERED")]
pub type PublishTriggered = crate::Reg<publish_triggered::PublishTriggeredSpec>;
#[doc = "Description collection: Publish configuration for event EVENTS_TRIGGERED\\[n\\]"]
pub mod publish_triggered;
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
#[doc = "Unspecified"]
pub use self::debugif::Debugif;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod debugif;
#[doc = "CPURUN (rw) register accessor: State of the CPU after a core reset\n\nYou can [`read`](crate::Reg::read) this register and get [`cpurun::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpurun::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpurun`]
module"]
#[doc(alias = "CPURUN")]
pub type Cpurun = crate::Reg<cpurun::CpurunSpec>;
#[doc = "State of the CPU after a core reset"]
pub mod cpurun;
#[doc = "INITPC (rw) register accessor: Initial value of the PC at CPU start.\n\nYou can [`read`](crate::Reg::read) this register and get [`initpc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initpc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@initpc`]
module"]
#[doc(alias = "INITPC")]
pub type Initpc = crate::Reg<initpc::InitpcSpec>;
#[doc = "Initial value of the PC at CPU start."]
pub mod initpc;
