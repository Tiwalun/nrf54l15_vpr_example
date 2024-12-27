#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_send: [TasksSend; 32],
    subscribe_send: [SubscribeSend; 32],
    events_receive: [EventsReceive; 32],
    publish_receive: [PublishReceive; 32],
    _reserved4: [u8; 0x0200],
    overflow: Overflow,
}
impl RegisterBlock {
    #[doc = "0x00..0x80 - Description collection: This task is unused, but the PPIB provides the SUBSCRIBE task to connect SEND \\[n\\]
task."]
    #[inline(always)]
    pub const fn tasks_send(&self, n: usize) -> &TasksSend {
        &self.tasks_send[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x80 - Description collection: This task is unused, but the PPIB provides the SUBSCRIBE task to connect SEND \\[n\\]
task."]
    #[inline(always)]
    pub fn tasks_send_iter(&self) -> impl Iterator<Item = &TasksSend> {
        self.tasks_send.iter()
    }
    #[doc = "0x80..0x100 - Description collection: Subscribe configuration for task SEND\\[n\\]"]
    #[inline(always)]
    pub const fn subscribe_send(&self, n: usize) -> &SubscribeSend {
        &self.subscribe_send[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - Description collection: Subscribe configuration for task SEND\\[n\\]"]
    #[inline(always)]
    pub fn subscribe_send_iter(&self) -> impl Iterator<Item = &SubscribeSend> {
        self.subscribe_send.iter()
    }
    #[doc = "0x100..0x180 - Description collection: This event is unused, but the PPIB provides the PUBLISH event to connect RECEIVE \\[n\\]
event."]
    #[inline(always)]
    pub const fn events_receive(&self, n: usize) -> &EventsReceive {
        &self.events_receive[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x180 - Description collection: This event is unused, but the PPIB provides the PUBLISH event to connect RECEIVE \\[n\\]
event."]
    #[inline(always)]
    pub fn events_receive_iter(&self) -> impl Iterator<Item = &EventsReceive> {
        self.events_receive.iter()
    }
    #[doc = "0x180..0x200 - Description collection: Publish configuration for event RECEIVE\\[n\\]"]
    #[inline(always)]
    pub const fn publish_receive(&self, n: usize) -> &PublishReceive {
        &self.publish_receive[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x200 - Description collection: Publish configuration for event RECEIVE\\[n\\]"]
    #[inline(always)]
    pub fn publish_receive_iter(&self) -> impl Iterator<Item = &PublishReceive> {
        self.publish_receive.iter()
    }
    #[doc = "0x400 - Unspecified"]
    #[inline(always)]
    pub const fn overflow(&self) -> &Overflow {
        &self.overflow
    }
}
#[doc = "TASKS_SEND (w) register accessor: Description collection: This task is unused, but the PPIB provides the SUBSCRIBE task to connect SEND \\[n\\]
task.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_send::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_send`]
module"]
#[doc(alias = "TASKS_SEND")]
pub type TasksSend = crate::Reg<tasks_send::TasksSendSpec>;
#[doc = "Description collection: This task is unused, but the PPIB provides the SUBSCRIBE task to connect SEND \\[n\\]
task."]
pub mod tasks_send;
#[doc = "SUBSCRIBE_SEND (rw) register accessor: Description collection: Subscribe configuration for task SEND\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_send::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_send::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_send`]
module"]
#[doc(alias = "SUBSCRIBE_SEND")]
pub type SubscribeSend = crate::Reg<subscribe_send::SubscribeSendSpec>;
#[doc = "Description collection: Subscribe configuration for task SEND\\[n\\]"]
pub mod subscribe_send;
#[doc = "EVENTS_RECEIVE (rw) register accessor: Description collection: This event is unused, but the PPIB provides the PUBLISH event to connect RECEIVE \\[n\\]
event.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_receive::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_receive::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_receive`]
module"]
#[doc(alias = "EVENTS_RECEIVE")]
pub type EventsReceive = crate::Reg<events_receive::EventsReceiveSpec>;
#[doc = "Description collection: This event is unused, but the PPIB provides the PUBLISH event to connect RECEIVE \\[n\\]
event."]
pub mod events_receive;
#[doc = "PUBLISH_RECEIVE (rw) register accessor: Description collection: Publish configuration for event RECEIVE\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_receive::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_receive::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_receive`]
module"]
#[doc(alias = "PUBLISH_RECEIVE")]
pub type PublishReceive = crate::Reg<publish_receive::PublishReceiveSpec>;
#[doc = "Description collection: Publish configuration for event RECEIVE\\[n\\]"]
pub mod publish_receive;
#[doc = "Unspecified"]
pub use self::overflow::Overflow;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod overflow;
