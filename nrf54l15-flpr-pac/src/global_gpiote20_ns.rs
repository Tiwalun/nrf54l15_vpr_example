#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_out: [TasksOut; 8],
    _reserved1: [u8; 0x10],
    tasks_set: [TasksSet; 8],
    _reserved2: [u8; 0x10],
    tasks_clr: [TasksClr; 8],
    subscribe_out: [SubscribeOut; 8],
    _reserved4: [u8; 0x10],
    subscribe_set: [SubscribeSet; 8],
    _reserved5: [u8; 0x10],
    subscribe_clr: [SubscribeClr; 8],
    events_in: [EventsIn; 8],
    _reserved7: [u8; 0x20],
    events_port: [EventsPort; 1],
    _reserved8: [u8; 0x38],
    publish_in: [PublishIn; 8],
    _reserved9: [u8; 0x20],
    publish_port: [PublishPort; 1],
    _reserved10: [u8; 0x013c],
    intenset0: Intenset0,
    intenclr0: Intenclr0,
    _reserved12: [u8; 0x08],
    intenset1: Intenset1,
    intenclr1: Intenclr1,
    _reserved14: [u8; 0x01f4],
    config: [Config; 8],
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY."]
    #[inline(always)]
    pub const fn tasks_out(&self, n: usize) -> &TasksOut {
        &self.tasks_out[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY."]
    #[inline(always)]
    pub fn tasks_out_iter(&self) -> impl Iterator<Item = &TasksOut> {
        self.tasks_out.iter()
    }
    #[doc = "0x30..0x50 - Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it high."]
    #[inline(always)]
    pub const fn tasks_set(&self, n: usize) -> &TasksSet {
        &self.tasks_set[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x50 - Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it high."]
    #[inline(always)]
    pub fn tasks_set_iter(&self) -> impl Iterator<Item = &TasksSet> {
        self.tasks_set.iter()
    }
    #[doc = "0x60..0x80 - Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it low."]
    #[inline(always)]
    pub const fn tasks_clr(&self, n: usize) -> &TasksClr {
        &self.tasks_clr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x80 - Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it low."]
    #[inline(always)]
    pub fn tasks_clr_iter(&self) -> impl Iterator<Item = &TasksClr> {
        self.tasks_clr.iter()
    }
    #[doc = "0x80..0xa0 - Description collection: Subscribe configuration for task OUT\\[n\\]"]
    #[inline(always)]
    pub const fn subscribe_out(&self, n: usize) -> &SubscribeOut {
        &self.subscribe_out[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xa0 - Description collection: Subscribe configuration for task OUT\\[n\\]"]
    #[inline(always)]
    pub fn subscribe_out_iter(&self) -> impl Iterator<Item = &SubscribeOut> {
        self.subscribe_out.iter()
    }
    #[doc = "0xb0..0xd0 - Description collection: Subscribe configuration for task SET\\[n\\]"]
    #[inline(always)]
    pub const fn subscribe_set(&self, n: usize) -> &SubscribeSet {
        &self.subscribe_set[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xb0..0xd0 - Description collection: Subscribe configuration for task SET\\[n\\]"]
    #[inline(always)]
    pub fn subscribe_set_iter(&self) -> impl Iterator<Item = &SubscribeSet> {
        self.subscribe_set.iter()
    }
    #[doc = "0xe0..0x100 - Description collection: Subscribe configuration for task CLR\\[n\\]"]
    #[inline(always)]
    pub const fn subscribe_clr(&self, n: usize) -> &SubscribeClr {
        &self.subscribe_clr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe0..0x100 - Description collection: Subscribe configuration for task CLR\\[n\\]"]
    #[inline(always)]
    pub fn subscribe_clr_iter(&self) -> impl Iterator<Item = &SubscribeClr> {
        self.subscribe_clr.iter()
    }
    #[doc = "0x100..0x120 - Description collection: Event from pin specified in CONFIG\\[n\\].PSEL"]
    #[inline(always)]
    pub const fn events_in(&self, n: usize) -> &EventsIn {
        &self.events_in[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x120 - Description collection: Event from pin specified in CONFIG\\[n\\].PSEL"]
    #[inline(always)]
    pub fn events_in_iter(&self) -> impl Iterator<Item = &EventsIn> {
        self.events_in.iter()
    }
    #[doc = "0x140..0x148 - Peripheral events."]
    #[inline(always)]
    pub const fn events_port(&self, n: usize) -> &EventsPort {
        &self.events_port[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x148 - Peripheral events."]
    #[inline(always)]
    pub fn events_port_iter(&self) -> impl Iterator<Item = &EventsPort> {
        self.events_port.iter()
    }
    #[doc = "0x180..0x1a0 - Description collection: Publish configuration for event IN\\[n\\]"]
    #[inline(always)]
    pub const fn publish_in(&self, n: usize) -> &PublishIn {
        &self.publish_in[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x1a0 - Description collection: Publish configuration for event IN\\[n\\]"]
    #[inline(always)]
    pub fn publish_in_iter(&self) -> impl Iterator<Item = &PublishIn> {
        self.publish_in.iter()
    }
    #[doc = "0x1c0..0x1c8 - Publish configuration for events"]
    #[inline(always)]
    pub const fn publish_port(&self, n: usize) -> &PublishPort {
        &self.publish_port[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0..0x1c8 - Publish configuration for events"]
    #[inline(always)]
    pub fn publish_port_iter(&self) -> impl Iterator<Item = &PublishPort> {
        self.publish_port.iter()
    }
    #[doc = "0x304 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset0(&self) -> &Intenset0 {
        &self.intenset0
    }
    #[doc = "0x308 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr0(&self) -> &Intenclr0 {
        &self.intenclr0
    }
    #[doc = "0x314 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset1(&self) -> &Intenset1 {
        &self.intenset1
    }
    #[doc = "0x318 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr1(&self) -> &Intenclr1 {
        &self.intenclr1
    }
    #[doc = "0x510..0x530 - Description collection: Configuration for OUT\\[n\\], SET\\[n\\], and CLR\\[n\\]
tasks and IN\\[n\\]
event"]
    #[inline(always)]
    pub const fn config(&self, n: usize) -> &Config {
        &self.config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x510..0x530 - Description collection: Configuration for OUT\\[n\\], SET\\[n\\], and CLR\\[n\\]
tasks and IN\\[n\\]
event"]
    #[inline(always)]
    pub fn config_iter(&self) -> impl Iterator<Item = &Config> {
        self.config.iter()
    }
}
#[doc = "TASKS_OUT (w) register accessor: Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_out::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_out`]
module"]
#[doc(alias = "TASKS_OUT")]
pub type TasksOut = crate::Reg<tasks_out::TasksOutSpec>;
#[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY."]
pub mod tasks_out;
#[doc = "TASKS_SET (w) register accessor: Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it high.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_set`]
module"]
#[doc(alias = "TASKS_SET")]
pub type TasksSet = crate::Reg<tasks_set::TasksSetSpec>;
#[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it high."]
pub mod tasks_set;
#[doc = "TASKS_CLR (w) register accessor: Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it low.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_clr`]
module"]
#[doc(alias = "TASKS_CLR")]
pub type TasksClr = crate::Reg<tasks_clr::TasksClrSpec>;
#[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it low."]
pub mod tasks_clr;
#[doc = "SUBSCRIBE_OUT (rw) register accessor: Description collection: Subscribe configuration for task OUT\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_out`]
module"]
#[doc(alias = "SUBSCRIBE_OUT")]
pub type SubscribeOut = crate::Reg<subscribe_out::SubscribeOutSpec>;
#[doc = "Description collection: Subscribe configuration for task OUT\\[n\\]"]
pub mod subscribe_out;
#[doc = "SUBSCRIBE_SET (rw) register accessor: Description collection: Subscribe configuration for task SET\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_set`]
module"]
#[doc(alias = "SUBSCRIBE_SET")]
pub type SubscribeSet = crate::Reg<subscribe_set::SubscribeSetSpec>;
#[doc = "Description collection: Subscribe configuration for task SET\\[n\\]"]
pub mod subscribe_set;
#[doc = "SUBSCRIBE_CLR (rw) register accessor: Description collection: Subscribe configuration for task CLR\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_clr`]
module"]
#[doc(alias = "SUBSCRIBE_CLR")]
pub type SubscribeClr = crate::Reg<subscribe_clr::SubscribeClrSpec>;
#[doc = "Description collection: Subscribe configuration for task CLR\\[n\\]"]
pub mod subscribe_clr;
#[doc = "EVENTS_IN (rw) register accessor: Description collection: Event from pin specified in CONFIG\\[n\\].PSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`events_in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_in`]
module"]
#[doc(alias = "EVENTS_IN")]
pub type EventsIn = crate::Reg<events_in::EventsInSpec>;
#[doc = "Description collection: Event from pin specified in CONFIG\\[n\\].PSEL"]
pub mod events_in;
#[doc = "Peripheral events."]
pub use self::events_port::EventsPort;
#[doc = r"Cluster"]
#[doc = "Peripheral events."]
pub mod events_port;
#[doc = "PUBLISH_IN (rw) register accessor: Description collection: Publish configuration for event IN\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_in`]
module"]
#[doc(alias = "PUBLISH_IN")]
pub type PublishIn = crate::Reg<publish_in::PublishInSpec>;
#[doc = "Description collection: Publish configuration for event IN\\[n\\]"]
pub mod publish_in;
#[doc = "Publish configuration for events"]
pub use self::publish_port::PublishPort;
#[doc = r"Cluster"]
#[doc = "Publish configuration for events"]
pub mod publish_port;
#[doc = "INTENSET0 (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset0`]
module"]
#[doc(alias = "INTENSET0")]
pub type Intenset0 = crate::Reg<intenset0::Intenset0Spec>;
#[doc = "Enable interrupt"]
pub mod intenset0;
#[doc = "INTENCLR0 (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr0`]
module"]
#[doc(alias = "INTENCLR0")]
pub type Intenclr0 = crate::Reg<intenclr0::Intenclr0Spec>;
#[doc = "Disable interrupt"]
pub mod intenclr0;
#[doc = "INTENSET1 (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset1`]
module"]
#[doc(alias = "INTENSET1")]
pub type Intenset1 = crate::Reg<intenset1::Intenset1Spec>;
#[doc = "Enable interrupt"]
pub mod intenset1;
#[doc = "INTENCLR1 (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr1`]
module"]
#[doc(alias = "INTENCLR1")]
pub type Intenclr1 = crate::Reg<intenclr1::Intenclr1Spec>;
#[doc = "Disable interrupt"]
pub mod intenclr1;
#[doc = "CONFIG (rw) register accessor: Description collection: Configuration for OUT\\[n\\], SET\\[n\\], and CLR\\[n\\]
tasks and IN\\[n\\]
event\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Description collection: Configuration for OUT\\[n\\], SET\\[n\\], and CLR\\[n\\]
tasks and IN\\[n\\]
event"]
pub mod config;
