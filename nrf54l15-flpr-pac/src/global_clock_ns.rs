#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_xostart: TasksXostart,
    tasks_xostop: TasksXostop,
    tasks_pllstart: TasksPllstart,
    tasks_pllstop: TasksPllstop,
    tasks_lfclkstart: TasksLfclkstart,
    tasks_lfclkstop: TasksLfclkstop,
    tasks_cal: TasksCal,
    tasks_xotune: TasksXotune,
    tasks_xotuneabort: TasksXotuneabort,
    _reserved9: [u8; 0x5c],
    subscribe_xostart: SubscribeXostart,
    subscribe_xostop: SubscribeXostop,
    subscribe_pllstart: SubscribePllstart,
    subscribe_pllstop: SubscribePllstop,
    subscribe_lfclkstart: SubscribeLfclkstart,
    subscribe_lfclkstop: SubscribeLfclkstop,
    subscribe_cal: SubscribeCal,
    subscribe_xotune: SubscribeXotune,
    subscribe_xotuneabort: SubscribeXotuneabort,
    _reserved18: [u8; 0x5c],
    events_xostarted: EventsXostarted,
    events_pllstarted: EventsPllstarted,
    events_lfclkstarted: EventsLfclkstarted,
    events_done: EventsDone,
    events_xotuned: EventsXotuned,
    events_xotuneerror: EventsXotuneerror,
    events_xotunefailed: EventsXotunefailed,
    _reserved25: [u8; 0x64],
    publish_xostarted: PublishXostarted,
    publish_pllstarted: PublishPllstarted,
    publish_lfclkstarted: PublishLfclkstarted,
    publish_done: PublishDone,
    publish_xotuned: PublishXotuned,
    publish_xotuneerror: PublishXotuneerror,
    publish_xotunefailed: PublishXotunefailed,
    _reserved32: [u8; 0x64],
    shorts: Shorts,
    _reserved33: [u8; 0xfc],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    intpend: Intpend,
    _reserved37: [u8; 0xf0],
    xo: Xo,
    _reserved38: [u8; 0x10],
    pll: Pll,
    _reserved39: [u8; 0x10],
    lfclk: Lfclk,
}
impl RegisterBlock {
    #[doc = "0x00 - Start crystal oscillator (HFXO)"]
    #[inline(always)]
    pub const fn tasks_xostart(&self) -> &TasksXostart {
        &self.tasks_xostart
    }
    #[doc = "0x04 - Stop crystal oscillator (HFXO)"]
    #[inline(always)]
    pub const fn tasks_xostop(&self) -> &TasksXostop {
        &self.tasks_xostop
    }
    #[doc = "0x08 - Start PLL and keep it running, regardless of the automatic clock requests"]
    #[inline(always)]
    pub const fn tasks_pllstart(&self) -> &TasksPllstart {
        &self.tasks_pllstart
    }
    #[doc = "0x0c - Stop PLL"]
    #[inline(always)]
    pub const fn tasks_pllstop(&self) -> &TasksPllstop {
        &self.tasks_pllstop
    }
    #[doc = "0x10 - Start LFCLK source as selected in LFCLK.SRC"]
    #[inline(always)]
    pub const fn tasks_lfclkstart(&self) -> &TasksLfclkstart {
        &self.tasks_lfclkstart
    }
    #[doc = "0x14 - Stop LFCLK source"]
    #[inline(always)]
    pub const fn tasks_lfclkstop(&self) -> &TasksLfclkstop {
        &self.tasks_lfclkstop
    }
    #[doc = "0x18 - Start calibration of LFRC oscillator"]
    #[inline(always)]
    pub const fn tasks_cal(&self) -> &TasksCal {
        &self.tasks_cal
    }
    #[doc = "0x1c - Request tuning for HFXO"]
    #[inline(always)]
    pub const fn tasks_xotune(&self) -> &TasksXotune {
        &self.tasks_xotune
    }
    #[doc = "0x20 - Abort tuning for HFXO"]
    #[inline(always)]
    pub const fn tasks_xotuneabort(&self) -> &TasksXotuneabort {
        &self.tasks_xotuneabort
    }
    #[doc = "0x80 - Subscribe configuration for task XOSTART"]
    #[inline(always)]
    pub const fn subscribe_xostart(&self) -> &SubscribeXostart {
        &self.subscribe_xostart
    }
    #[doc = "0x84 - Subscribe configuration for task XOSTOP"]
    #[inline(always)]
    pub const fn subscribe_xostop(&self) -> &SubscribeXostop {
        &self.subscribe_xostop
    }
    #[doc = "0x88 - Subscribe configuration for task PLLSTART"]
    #[inline(always)]
    pub const fn subscribe_pllstart(&self) -> &SubscribePllstart {
        &self.subscribe_pllstart
    }
    #[doc = "0x8c - Subscribe configuration for task PLLSTOP"]
    #[inline(always)]
    pub const fn subscribe_pllstop(&self) -> &SubscribePllstop {
        &self.subscribe_pllstop
    }
    #[doc = "0x90 - Subscribe configuration for task LFCLKSTART"]
    #[inline(always)]
    pub const fn subscribe_lfclkstart(&self) -> &SubscribeLfclkstart {
        &self.subscribe_lfclkstart
    }
    #[doc = "0x94 - Subscribe configuration for task LFCLKSTOP"]
    #[inline(always)]
    pub const fn subscribe_lfclkstop(&self) -> &SubscribeLfclkstop {
        &self.subscribe_lfclkstop
    }
    #[doc = "0x98 - Subscribe configuration for task CAL"]
    #[inline(always)]
    pub const fn subscribe_cal(&self) -> &SubscribeCal {
        &self.subscribe_cal
    }
    #[doc = "0x9c - Subscribe configuration for task XOTUNE"]
    #[inline(always)]
    pub const fn subscribe_xotune(&self) -> &SubscribeXotune {
        &self.subscribe_xotune
    }
    #[doc = "0xa0 - Subscribe configuration for task XOTUNEABORT"]
    #[inline(always)]
    pub const fn subscribe_xotuneabort(&self) -> &SubscribeXotuneabort {
        &self.subscribe_xotuneabort
    }
    #[doc = "0x100 - Crystal oscillator has started"]
    #[inline(always)]
    pub const fn events_xostarted(&self) -> &EventsXostarted {
        &self.events_xostarted
    }
    #[doc = "0x104 - PLL started"]
    #[inline(always)]
    pub const fn events_pllstarted(&self) -> &EventsPllstarted {
        &self.events_pllstarted
    }
    #[doc = "0x108 - LFCLK source started"]
    #[inline(always)]
    pub const fn events_lfclkstarted(&self) -> &EventsLfclkstarted {
        &self.events_lfclkstarted
    }
    #[doc = "0x10c - Calibration of LFRC oscillator complete event"]
    #[inline(always)]
    pub const fn events_done(&self) -> &EventsDone {
        &self.events_done
    }
    #[doc = "0x110 - HFXO tuning is done. XOTUNED is generated after TASKS_XOSTART or after TASKS_XOTUNE has completed"]
    #[inline(always)]
    pub const fn events_xotuned(&self) -> &EventsXotuned {
        &self.events_xotuned
    }
    #[doc = "0x114 - HFXO quality issue detected, XOTUNE is needed"]
    #[inline(always)]
    pub const fn events_xotuneerror(&self) -> &EventsXotuneerror {
        &self.events_xotuneerror
    }
    #[doc = "0x118 - HFXO tuning could not be completed"]
    #[inline(always)]
    pub const fn events_xotunefailed(&self) -> &EventsXotunefailed {
        &self.events_xotunefailed
    }
    #[doc = "0x180 - Publish configuration for event XOSTARTED"]
    #[inline(always)]
    pub const fn publish_xostarted(&self) -> &PublishXostarted {
        &self.publish_xostarted
    }
    #[doc = "0x184 - Publish configuration for event PLLSTARTED"]
    #[inline(always)]
    pub const fn publish_pllstarted(&self) -> &PublishPllstarted {
        &self.publish_pllstarted
    }
    #[doc = "0x188 - Publish configuration for event LFCLKSTARTED"]
    #[inline(always)]
    pub const fn publish_lfclkstarted(&self) -> &PublishLfclkstarted {
        &self.publish_lfclkstarted
    }
    #[doc = "0x18c - Publish configuration for event DONE"]
    #[inline(always)]
    pub const fn publish_done(&self) -> &PublishDone {
        &self.publish_done
    }
    #[doc = "0x190 - Publish configuration for event XOTUNED"]
    #[inline(always)]
    pub const fn publish_xotuned(&self) -> &PublishXotuned {
        &self.publish_xotuned
    }
    #[doc = "0x194 - Publish configuration for event XOTUNEERROR"]
    #[inline(always)]
    pub const fn publish_xotuneerror(&self) -> &PublishXotuneerror {
        &self.publish_xotuneerror
    }
    #[doc = "0x198 - Publish configuration for event XOTUNEFAILED"]
    #[inline(always)]
    pub const fn publish_xotunefailed(&self) -> &PublishXotunefailed {
        &self.publish_xotunefailed
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
    #[doc = "0x400..0x410 - Unspecified"]
    #[inline(always)]
    pub const fn xo(&self) -> &Xo {
        &self.xo
    }
    #[doc = "0x420..0x430 - Unspecified"]
    #[inline(always)]
    pub const fn pll(&self) -> &Pll {
        &self.pll
    }
    #[doc = "0x440..0x454 - Unspecified"]
    #[inline(always)]
    pub const fn lfclk(&self) -> &Lfclk {
        &self.lfclk
    }
}
#[doc = "TASKS_XOSTART (w) register accessor: Start crystal oscillator (HFXO)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_xostart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_xostart`]
module"]
#[doc(alias = "TASKS_XOSTART")]
pub type TasksXostart = crate::Reg<tasks_xostart::TasksXostartSpec>;
#[doc = "Start crystal oscillator (HFXO)"]
pub mod tasks_xostart;
#[doc = "TASKS_XOSTOP (w) register accessor: Stop crystal oscillator (HFXO)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_xostop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_xostop`]
module"]
#[doc(alias = "TASKS_XOSTOP")]
pub type TasksXostop = crate::Reg<tasks_xostop::TasksXostopSpec>;
#[doc = "Stop crystal oscillator (HFXO)"]
pub mod tasks_xostop;
#[doc = "TASKS_PLLSTART (w) register accessor: Start PLL and keep it running, regardless of the automatic clock requests\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_pllstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_pllstart`]
module"]
#[doc(alias = "TASKS_PLLSTART")]
pub type TasksPllstart = crate::Reg<tasks_pllstart::TasksPllstartSpec>;
#[doc = "Start PLL and keep it running, regardless of the automatic clock requests"]
pub mod tasks_pllstart;
#[doc = "TASKS_PLLSTOP (w) register accessor: Stop PLL\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_pllstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_pllstop`]
module"]
#[doc(alias = "TASKS_PLLSTOP")]
pub type TasksPllstop = crate::Reg<tasks_pllstop::TasksPllstopSpec>;
#[doc = "Stop PLL"]
pub mod tasks_pllstop;
#[doc = "TASKS_LFCLKSTART (w) register accessor: Start LFCLK source as selected in LFCLK.SRC\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_lfclkstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_lfclkstart`]
module"]
#[doc(alias = "TASKS_LFCLKSTART")]
pub type TasksLfclkstart = crate::Reg<tasks_lfclkstart::TasksLfclkstartSpec>;
#[doc = "Start LFCLK source as selected in LFCLK.SRC"]
pub mod tasks_lfclkstart;
#[doc = "TASKS_LFCLKSTOP (w) register accessor: Stop LFCLK source\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_lfclkstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_lfclkstop`]
module"]
#[doc(alias = "TASKS_LFCLKSTOP")]
pub type TasksLfclkstop = crate::Reg<tasks_lfclkstop::TasksLfclkstopSpec>;
#[doc = "Stop LFCLK source"]
pub mod tasks_lfclkstop;
#[doc = "TASKS_CAL (w) register accessor: Start calibration of LFRC oscillator\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_cal::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_cal`]
module"]
#[doc(alias = "TASKS_CAL")]
pub type TasksCal = crate::Reg<tasks_cal::TasksCalSpec>;
#[doc = "Start calibration of LFRC oscillator"]
pub mod tasks_cal;
#[doc = "TASKS_XOTUNE (w) register accessor: Request tuning for HFXO\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_xotune::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_xotune`]
module"]
#[doc(alias = "TASKS_XOTUNE")]
pub type TasksXotune = crate::Reg<tasks_xotune::TasksXotuneSpec>;
#[doc = "Request tuning for HFXO"]
pub mod tasks_xotune;
#[doc = "TASKS_XOTUNEABORT (w) register accessor: Abort tuning for HFXO\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_xotuneabort::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_xotuneabort`]
module"]
#[doc(alias = "TASKS_XOTUNEABORT")]
pub type TasksXotuneabort = crate::Reg<tasks_xotuneabort::TasksXotuneabortSpec>;
#[doc = "Abort tuning for HFXO"]
pub mod tasks_xotuneabort;
#[doc = "SUBSCRIBE_XOSTART (rw) register accessor: Subscribe configuration for task XOSTART\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_xostart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_xostart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_xostart`]
module"]
#[doc(alias = "SUBSCRIBE_XOSTART")]
pub type SubscribeXostart = crate::Reg<subscribe_xostart::SubscribeXostartSpec>;
#[doc = "Subscribe configuration for task XOSTART"]
pub mod subscribe_xostart;
#[doc = "SUBSCRIBE_XOSTOP (rw) register accessor: Subscribe configuration for task XOSTOP\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_xostop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_xostop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_xostop`]
module"]
#[doc(alias = "SUBSCRIBE_XOSTOP")]
pub type SubscribeXostop = crate::Reg<subscribe_xostop::SubscribeXostopSpec>;
#[doc = "Subscribe configuration for task XOSTOP"]
pub mod subscribe_xostop;
#[doc = "SUBSCRIBE_PLLSTART (rw) register accessor: Subscribe configuration for task PLLSTART\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_pllstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_pllstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_pllstart`]
module"]
#[doc(alias = "SUBSCRIBE_PLLSTART")]
pub type SubscribePllstart = crate::Reg<subscribe_pllstart::SubscribePllstartSpec>;
#[doc = "Subscribe configuration for task PLLSTART"]
pub mod subscribe_pllstart;
#[doc = "SUBSCRIBE_PLLSTOP (rw) register accessor: Subscribe configuration for task PLLSTOP\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_pllstop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_pllstop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_pllstop`]
module"]
#[doc(alias = "SUBSCRIBE_PLLSTOP")]
pub type SubscribePllstop = crate::Reg<subscribe_pllstop::SubscribePllstopSpec>;
#[doc = "Subscribe configuration for task PLLSTOP"]
pub mod subscribe_pllstop;
#[doc = "SUBSCRIBE_LFCLKSTART (rw) register accessor: Subscribe configuration for task LFCLKSTART\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_lfclkstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_lfclkstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_lfclkstart`]
module"]
#[doc(alias = "SUBSCRIBE_LFCLKSTART")]
pub type SubscribeLfclkstart = crate::Reg<subscribe_lfclkstart::SubscribeLfclkstartSpec>;
#[doc = "Subscribe configuration for task LFCLKSTART"]
pub mod subscribe_lfclkstart;
#[doc = "SUBSCRIBE_LFCLKSTOP (rw) register accessor: Subscribe configuration for task LFCLKSTOP\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_lfclkstop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_lfclkstop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_lfclkstop`]
module"]
#[doc(alias = "SUBSCRIBE_LFCLKSTOP")]
pub type SubscribeLfclkstop = crate::Reg<subscribe_lfclkstop::SubscribeLfclkstopSpec>;
#[doc = "Subscribe configuration for task LFCLKSTOP"]
pub mod subscribe_lfclkstop;
#[doc = "SUBSCRIBE_CAL (rw) register accessor: Subscribe configuration for task CAL\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_cal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_cal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_cal`]
module"]
#[doc(alias = "SUBSCRIBE_CAL")]
pub type SubscribeCal = crate::Reg<subscribe_cal::SubscribeCalSpec>;
#[doc = "Subscribe configuration for task CAL"]
pub mod subscribe_cal;
#[doc = "SUBSCRIBE_XOTUNE (rw) register accessor: Subscribe configuration for task XOTUNE\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_xotune::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_xotune::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_xotune`]
module"]
#[doc(alias = "SUBSCRIBE_XOTUNE")]
pub type SubscribeXotune = crate::Reg<subscribe_xotune::SubscribeXotuneSpec>;
#[doc = "Subscribe configuration for task XOTUNE"]
pub mod subscribe_xotune;
#[doc = "SUBSCRIBE_XOTUNEABORT (rw) register accessor: Subscribe configuration for task XOTUNEABORT\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_xotuneabort::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_xotuneabort::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_xotuneabort`]
module"]
#[doc(alias = "SUBSCRIBE_XOTUNEABORT")]
pub type SubscribeXotuneabort = crate::Reg<subscribe_xotuneabort::SubscribeXotuneabortSpec>;
#[doc = "Subscribe configuration for task XOTUNEABORT"]
pub mod subscribe_xotuneabort;
#[doc = "EVENTS_XOSTARTED (rw) register accessor: Crystal oscillator has started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_xostarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_xostarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_xostarted`]
module"]
#[doc(alias = "EVENTS_XOSTARTED")]
pub type EventsXostarted = crate::Reg<events_xostarted::EventsXostartedSpec>;
#[doc = "Crystal oscillator has started"]
pub mod events_xostarted;
#[doc = "EVENTS_PLLSTARTED (rw) register accessor: PLL started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_pllstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_pllstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_pllstarted`]
module"]
#[doc(alias = "EVENTS_PLLSTARTED")]
pub type EventsPllstarted = crate::Reg<events_pllstarted::EventsPllstartedSpec>;
#[doc = "PLL started"]
pub mod events_pllstarted;
#[doc = "EVENTS_LFCLKSTARTED (rw) register accessor: LFCLK source started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_lfclkstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_lfclkstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_lfclkstarted`]
module"]
#[doc(alias = "EVENTS_LFCLKSTARTED")]
pub type EventsLfclkstarted = crate::Reg<events_lfclkstarted::EventsLfclkstartedSpec>;
#[doc = "LFCLK source started"]
pub mod events_lfclkstarted;
#[doc = "EVENTS_DONE (rw) register accessor: Calibration of LFRC oscillator complete event\n\nYou can [`read`](crate::Reg::read) this register and get [`events_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_done`]
module"]
#[doc(alias = "EVENTS_DONE")]
pub type EventsDone = crate::Reg<events_done::EventsDoneSpec>;
#[doc = "Calibration of LFRC oscillator complete event"]
pub mod events_done;
#[doc = "EVENTS_XOTUNED (rw) register accessor: HFXO tuning is done. XOTUNED is generated after TASKS_XOSTART or after TASKS_XOTUNE has completed\n\nYou can [`read`](crate::Reg::read) this register and get [`events_xotuned::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_xotuned::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_xotuned`]
module"]
#[doc(alias = "EVENTS_XOTUNED")]
pub type EventsXotuned = crate::Reg<events_xotuned::EventsXotunedSpec>;
#[doc = "HFXO tuning is done. XOTUNED is generated after TASKS_XOSTART or after TASKS_XOTUNE has completed"]
pub mod events_xotuned;
#[doc = "EVENTS_XOTUNEERROR (rw) register accessor: HFXO quality issue detected, XOTUNE is needed\n\nYou can [`read`](crate::Reg::read) this register and get [`events_xotuneerror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_xotuneerror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_xotuneerror`]
module"]
#[doc(alias = "EVENTS_XOTUNEERROR")]
pub type EventsXotuneerror = crate::Reg<events_xotuneerror::EventsXotuneerrorSpec>;
#[doc = "HFXO quality issue detected, XOTUNE is needed"]
pub mod events_xotuneerror;
#[doc = "EVENTS_XOTUNEFAILED (rw) register accessor: HFXO tuning could not be completed\n\nYou can [`read`](crate::Reg::read) this register and get [`events_xotunefailed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_xotunefailed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_xotunefailed`]
module"]
#[doc(alias = "EVENTS_XOTUNEFAILED")]
pub type EventsXotunefailed = crate::Reg<events_xotunefailed::EventsXotunefailedSpec>;
#[doc = "HFXO tuning could not be completed"]
pub mod events_xotunefailed;
#[doc = "PUBLISH_XOSTARTED (rw) register accessor: Publish configuration for event XOSTARTED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_xostarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_xostarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_xostarted`]
module"]
#[doc(alias = "PUBLISH_XOSTARTED")]
pub type PublishXostarted = crate::Reg<publish_xostarted::PublishXostartedSpec>;
#[doc = "Publish configuration for event XOSTARTED"]
pub mod publish_xostarted;
#[doc = "PUBLISH_PLLSTARTED (rw) register accessor: Publish configuration for event PLLSTARTED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_pllstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_pllstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_pllstarted`]
module"]
#[doc(alias = "PUBLISH_PLLSTARTED")]
pub type PublishPllstarted = crate::Reg<publish_pllstarted::PublishPllstartedSpec>;
#[doc = "Publish configuration for event PLLSTARTED"]
pub mod publish_pllstarted;
#[doc = "PUBLISH_LFCLKSTARTED (rw) register accessor: Publish configuration for event LFCLKSTARTED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_lfclkstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_lfclkstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_lfclkstarted`]
module"]
#[doc(alias = "PUBLISH_LFCLKSTARTED")]
pub type PublishLfclkstarted = crate::Reg<publish_lfclkstarted::PublishLfclkstartedSpec>;
#[doc = "Publish configuration for event LFCLKSTARTED"]
pub mod publish_lfclkstarted;
#[doc = "PUBLISH_DONE (rw) register accessor: Publish configuration for event DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_done`]
module"]
#[doc(alias = "PUBLISH_DONE")]
pub type PublishDone = crate::Reg<publish_done::PublishDoneSpec>;
#[doc = "Publish configuration for event DONE"]
pub mod publish_done;
#[doc = "PUBLISH_XOTUNED (rw) register accessor: Publish configuration for event XOTUNED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_xotuned::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_xotuned::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_xotuned`]
module"]
#[doc(alias = "PUBLISH_XOTUNED")]
pub type PublishXotuned = crate::Reg<publish_xotuned::PublishXotunedSpec>;
#[doc = "Publish configuration for event XOTUNED"]
pub mod publish_xotuned;
#[doc = "PUBLISH_XOTUNEERROR (rw) register accessor: Publish configuration for event XOTUNEERROR\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_xotuneerror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_xotuneerror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_xotuneerror`]
module"]
#[doc(alias = "PUBLISH_XOTUNEERROR")]
pub type PublishXotuneerror = crate::Reg<publish_xotuneerror::PublishXotuneerrorSpec>;
#[doc = "Publish configuration for event XOTUNEERROR"]
pub mod publish_xotuneerror;
#[doc = "PUBLISH_XOTUNEFAILED (rw) register accessor: Publish configuration for event XOTUNEFAILED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_xotunefailed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_xotunefailed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_xotunefailed`]
module"]
#[doc(alias = "PUBLISH_XOTUNEFAILED")]
pub type PublishXotunefailed = crate::Reg<publish_xotunefailed::PublishXotunefailedSpec>;
#[doc = "Publish configuration for event XOTUNEFAILED"]
pub mod publish_xotunefailed;
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
#[doc = "Unspecified"]
pub use self::xo::Xo;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod xo;
#[doc = "Unspecified"]
pub use self::pll::Pll;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod pll;
#[doc = "Unspecified"]
pub use self::lfclk::Lfclk;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod lfclk;
