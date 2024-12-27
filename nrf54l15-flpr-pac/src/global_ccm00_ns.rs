#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    tasks_rateoverride: TasksRateoverride,
    _reserved3: [u8; 0x74],
    subscribe_start: SubscribeStart,
    subscribe_stop: SubscribeStop,
    subscribe_rateoverride: SubscribeRateoverride,
    _reserved6: [u8; 0x78],
    events_end: EventsEnd,
    events_error: EventsError,
    _reserved8: [u8; 0x78],
    publish_end: PublishEnd,
    publish_error: PublishError,
    _reserved10: [u8; 0x0178],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved12: [u8; 0xf4],
    macstatus: Macstatus,
    errorstatus: Errorstatus,
    _reserved14: [u8; 0xf8],
    enable: Enable,
    mode: Mode,
    _reserved16: [u8; 0x08],
    key: Key,
    nonce: Nonce,
    in_: In,
    _reserved19: [u8; 0x04],
    out: Out,
    _reserved20: [u8; 0x08],
    rateoverride: Rateoverride,
    adatamask: Adatamask,
}
impl RegisterBlock {
    #[doc = "0x00 - Start encryption/decryption. This operation will stop by itself when completed."]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x04 - Stop encryption/decryption"]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x08 - Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption"]
    #[inline(always)]
    pub const fn tasks_rateoverride(&self) -> &TasksRateoverride {
        &self.tasks_rateoverride
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
    #[doc = "0x88 - Subscribe configuration for task RATEOVERRIDE"]
    #[inline(always)]
    pub const fn subscribe_rateoverride(&self) -> &SubscribeRateoverride {
        &self.subscribe_rateoverride
    }
    #[doc = "0x104 - Encrypt/decrypt complete or ended because of an error"]
    #[inline(always)]
    pub const fn events_end(&self) -> &EventsEnd {
        &self.events_end
    }
    #[doc = "0x108 - CCM error event"]
    #[inline(always)]
    pub const fn events_error(&self) -> &EventsError {
        &self.events_error
    }
    #[doc = "0x184 - Publish configuration for event END"]
    #[inline(always)]
    pub const fn publish_end(&self) -> &PublishEnd {
        &self.publish_end
    }
    #[doc = "0x188 - Publish configuration for event ERROR"]
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
    #[doc = "0x400 - MAC check result"]
    #[inline(always)]
    pub const fn macstatus(&self) -> &Macstatus {
        &self.macstatus
    }
    #[doc = "0x404 - Error status"]
    #[inline(always)]
    pub const fn errorstatus(&self) -> &Errorstatus {
        &self.errorstatus
    }
    #[doc = "0x500 - Enable"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x504 - Operation mode"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x510..0x520 - Unspecified"]
    #[inline(always)]
    pub const fn key(&self) -> &Key {
        &self.key
    }
    #[doc = "0x520..0x530 - Unspecified"]
    #[inline(always)]
    pub const fn nonce(&self) -> &Nonce {
        &self.nonce
    }
    #[doc = "0x530 - IN EasyDMA channel"]
    #[inline(always)]
    pub const fn in_(&self) -> &In {
        &self.in_
    }
    #[doc = "0x538 - OUT EasyDMA channel"]
    #[inline(always)]
    pub const fn out(&self) -> &Out {
        &self.out
    }
    #[doc = "0x544 - Data rate override setting."]
    #[inline(always)]
    pub const fn rateoverride(&self) -> &Rateoverride {
        &self.rateoverride
    }
    #[doc = "0x548 - CCM adata mask."]
    #[inline(always)]
    pub const fn adatamask(&self) -> &Adatamask {
        &self.adatamask
    }
}
#[doc = "TASKS_START (w) register accessor: Start encryption/decryption. This operation will stop by itself when completed.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`]
module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Start encryption/decryption. This operation will stop by itself when completed."]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Stop encryption/decryption\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`]
module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop encryption/decryption"]
pub mod tasks_stop;
#[doc = "TASKS_RATEOVERRIDE (w) register accessor: Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rateoverride::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_rateoverride`]
module"]
#[doc(alias = "TASKS_RATEOVERRIDE")]
pub type TasksRateoverride = crate::Reg<tasks_rateoverride::TasksRateoverrideSpec>;
#[doc = "Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption"]
pub mod tasks_rateoverride;
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
#[doc = "SUBSCRIBE_RATEOVERRIDE (rw) register accessor: Subscribe configuration for task RATEOVERRIDE\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_rateoverride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_rateoverride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_rateoverride`]
module"]
#[doc(alias = "SUBSCRIBE_RATEOVERRIDE")]
pub type SubscribeRateoverride = crate::Reg<subscribe_rateoverride::SubscribeRateoverrideSpec>;
#[doc = "Subscribe configuration for task RATEOVERRIDE"]
pub mod subscribe_rateoverride;
#[doc = "EVENTS_END (rw) register accessor: Encrypt/decrypt complete or ended because of an error\n\nYou can [`read`](crate::Reg::read) this register and get [`events_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_end`]
module"]
#[doc(alias = "EVENTS_END")]
pub type EventsEnd = crate::Reg<events_end::EventsEndSpec>;
#[doc = "Encrypt/decrypt complete or ended because of an error"]
pub mod events_end;
#[doc = "EVENTS_ERROR (rw) register accessor: CCM error event\n\nYou can [`read`](crate::Reg::read) this register and get [`events_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_error`]
module"]
#[doc(alias = "EVENTS_ERROR")]
pub type EventsError = crate::Reg<events_error::EventsErrorSpec>;
#[doc = "CCM error event"]
pub mod events_error;
#[doc = "PUBLISH_END (rw) register accessor: Publish configuration for event END\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_end`]
module"]
#[doc(alias = "PUBLISH_END")]
pub type PublishEnd = crate::Reg<publish_end::PublishEndSpec>;
#[doc = "Publish configuration for event END"]
pub mod publish_end;
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
#[doc = "MACSTATUS (r) register accessor: MAC check result\n\nYou can [`read`](crate::Reg::read) this register and get [`macstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macstatus`]
module"]
#[doc(alias = "MACSTATUS")]
pub type Macstatus = crate::Reg<macstatus::MacstatusSpec>;
#[doc = "MAC check result"]
pub mod macstatus;
#[doc = "ERRORSTATUS (r) register accessor: Error status\n\nYou can [`read`](crate::Reg::read) this register and get [`errorstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errorstatus`]
module"]
#[doc(alias = "ERRORSTATUS")]
pub type Errorstatus = crate::Reg<errorstatus::ErrorstatusSpec>;
#[doc = "Error status"]
pub mod errorstatus;
#[doc = "ENABLE (rw) register accessor: Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable"]
pub mod enable;
#[doc = "MODE (rw) register accessor: Operation mode\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`]
module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "Operation mode"]
pub mod mode;
#[doc = "Unspecified"]
pub use self::key::Key;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod key;
#[doc = "Unspecified"]
pub use self::nonce::Nonce;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod nonce;
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
#[doc = "RATEOVERRIDE (rw) register accessor: Data rate override setting.\n\nYou can [`read`](crate::Reg::read) this register and get [`rateoverride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rateoverride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rateoverride`]
module"]
#[doc(alias = "RATEOVERRIDE")]
pub type Rateoverride = crate::Reg<rateoverride::RateoverrideSpec>;
#[doc = "Data rate override setting."]
pub mod rateoverride;
#[doc = "ADATAMASK (rw) register accessor: CCM adata mask.\n\nYou can [`read`](crate::Reg::read) this register and get [`adatamask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adatamask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adatamask`]
module"]
#[doc(alias = "ADATAMASK")]
pub type Adatamask = crate::Reg<adatamask::AdatamaskSpec>;
#[doc = "CCM adata mask."]
pub mod adatamask;
