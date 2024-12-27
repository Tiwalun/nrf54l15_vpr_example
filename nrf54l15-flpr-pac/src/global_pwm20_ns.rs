#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    tasks_stop: TasksStop,
    tasks_nextstep: TasksNextstep,
    _reserved2: [u8; 0x04],
    tasks_dma: TasksDma,
    _reserved3: [u8; 0x64],
    subscribe_stop: SubscribeStop,
    subscribe_nextstep: SubscribeNextstep,
    _reserved5: [u8; 0x04],
    subscribe_dma: SubscribeDma,
    _reserved6: [u8; 0x64],
    events_stopped: EventsStopped,
    events_seqstarted: [EventsSeqstarted; 2],
    events_seqend: [EventsSeqend; 2],
    events_pwmperiodend: EventsPwmperiodend,
    events_loopsdone: EventsLoopsdone,
    events_ramunderflow: EventsRamunderflow,
    events_dma: EventsDma,
    events_comparematch: [EventsComparematch; 4],
    _reserved14: [u8; 0x38],
    publish_stopped: PublishStopped,
    publish_seqstarted: [PublishSeqstarted; 2],
    publish_seqend: [PublishSeqend; 2],
    publish_pwmperiodend: PublishPwmperiodend,
    publish_loopsdone: PublishLoopsdone,
    publish_ramunderflow: PublishRamunderflow,
    publish_dma: PublishDma,
    publish_comparematch: [PublishComparematch; 4],
    _reserved22: [u8; 0x34],
    shorts: Shorts,
    _reserved23: [u8; 0xfc],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    intpend: Intpend,
    _reserved27: [u8; 0x01f0],
    enable: Enable,
    mode: Mode,
    countertop: Countertop,
    prescaler: Prescaler,
    decoder: Decoder,
    loop_: Loop,
    idleout: Idleout,
    _reserved34: [u8; 0x04],
    seq: (),
    _reserved35: [u8; 0x40],
    psel: Psel,
    _reserved36: [u8; 0x0190],
    dma: Dma,
}
impl RegisterBlock {
    #[doc = "0x04 - Stops PWM pulse generation on all channels at the end of current PWM period, and stops sequence playback"]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x08 - Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running."]
    #[inline(always)]
    pub const fn tasks_nextstep(&self) -> &TasksNextstep {
        &self.tasks_nextstep
    }
    #[doc = "0x10..0x20 - Peripheral tasks."]
    #[inline(always)]
    pub const fn tasks_dma(&self) -> &TasksDma {
        &self.tasks_dma
    }
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    #[inline(always)]
    pub const fn subscribe_stop(&self) -> &SubscribeStop {
        &self.subscribe_stop
    }
    #[doc = "0x88 - Subscribe configuration for task NEXTSTEP"]
    #[inline(always)]
    pub const fn subscribe_nextstep(&self) -> &SubscribeNextstep {
        &self.subscribe_nextstep
    }
    #[doc = "0x90..0xa0 - Subscribe configuration for tasks"]
    #[inline(always)]
    pub const fn subscribe_dma(&self) -> &SubscribeDma {
        &self.subscribe_dma
    }
    #[doc = "0x104 - Response to STOP task, emitted when PWM pulses are no longer generated"]
    #[inline(always)]
    pub const fn events_stopped(&self) -> &EventsStopped {
        &self.events_stopped
    }
    #[doc = "0x108..0x110 - Description collection: First PWM period started on sequence n"]
    #[inline(always)]
    pub const fn events_seqstarted(&self, n: usize) -> &EventsSeqstarted {
        &self.events_seqstarted[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x108..0x110 - Description collection: First PWM period started on sequence n"]
    #[inline(always)]
    pub fn events_seqstarted_iter(&self) -> impl Iterator<Item = &EventsSeqstarted> {
        self.events_seqstarted.iter()
    }
    #[doc = "0x110..0x118 - Description collection: Emitted at end of every sequence n, when last value from RAM has been applied to wave counter"]
    #[inline(always)]
    pub const fn events_seqend(&self, n: usize) -> &EventsSeqend {
        &self.events_seqend[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x110..0x118 - Description collection: Emitted at end of every sequence n, when last value from RAM has been applied to wave counter"]
    #[inline(always)]
    pub fn events_seqend_iter(&self) -> impl Iterator<Item = &EventsSeqend> {
        self.events_seqend.iter()
    }
    #[doc = "0x118 - Emitted at the end of each PWM period"]
    #[inline(always)]
    pub const fn events_pwmperiodend(&self) -> &EventsPwmperiodend {
        &self.events_pwmperiodend
    }
    #[doc = "0x11c - Concatenated sequences have been played the amount of times defined in LOOP.CNT"]
    #[inline(always)]
    pub const fn events_loopsdone(&self) -> &EventsLoopsdone {
        &self.events_loopsdone
    }
    #[doc = "0x120 - Emitted when retrieving from RAM does not complete in time for the PWM module"]
    #[inline(always)]
    pub const fn events_ramunderflow(&self) -> &EventsRamunderflow {
        &self.events_ramunderflow
    }
    #[doc = "0x124..0x13c - Peripheral events."]
    #[inline(always)]
    pub const fn events_dma(&self) -> &EventsDma {
        &self.events_dma
    }
    #[doc = "0x13c..0x14c - Description collection: This event is generated when the compare matches for the compare channel \\[n\\]."]
    #[inline(always)]
    pub const fn events_comparematch(&self, n: usize) -> &EventsComparematch {
        &self.events_comparematch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x13c..0x14c - Description collection: This event is generated when the compare matches for the compare channel \\[n\\]."]
    #[inline(always)]
    pub fn events_comparematch_iter(&self) -> impl Iterator<Item = &EventsComparematch> {
        self.events_comparematch.iter()
    }
    #[doc = "0x184 - Publish configuration for event STOPPED"]
    #[inline(always)]
    pub const fn publish_stopped(&self) -> &PublishStopped {
        &self.publish_stopped
    }
    #[doc = "0x188..0x190 - Description collection: Publish configuration for event SEQSTARTED\\[n\\]"]
    #[inline(always)]
    pub const fn publish_seqstarted(&self, n: usize) -> &PublishSeqstarted {
        &self.publish_seqstarted[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x188..0x190 - Description collection: Publish configuration for event SEQSTARTED\\[n\\]"]
    #[inline(always)]
    pub fn publish_seqstarted_iter(&self) -> impl Iterator<Item = &PublishSeqstarted> {
        self.publish_seqstarted.iter()
    }
    #[doc = "0x190..0x198 - Description collection: Publish configuration for event SEQEND\\[n\\]"]
    #[inline(always)]
    pub const fn publish_seqend(&self, n: usize) -> &PublishSeqend {
        &self.publish_seqend[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x190..0x198 - Description collection: Publish configuration for event SEQEND\\[n\\]"]
    #[inline(always)]
    pub fn publish_seqend_iter(&self) -> impl Iterator<Item = &PublishSeqend> {
        self.publish_seqend.iter()
    }
    #[doc = "0x198 - Publish configuration for event PWMPERIODEND"]
    #[inline(always)]
    pub const fn publish_pwmperiodend(&self) -> &PublishPwmperiodend {
        &self.publish_pwmperiodend
    }
    #[doc = "0x19c - Publish configuration for event LOOPSDONE"]
    #[inline(always)]
    pub const fn publish_loopsdone(&self) -> &PublishLoopsdone {
        &self.publish_loopsdone
    }
    #[doc = "0x1a0 - Publish configuration for event RAMUNDERFLOW"]
    #[inline(always)]
    pub const fn publish_ramunderflow(&self) -> &PublishRamunderflow {
        &self.publish_ramunderflow
    }
    #[doc = "0x1a4..0x1bc - Publish configuration for events"]
    #[inline(always)]
    pub const fn publish_dma(&self) -> &PublishDma {
        &self.publish_dma
    }
    #[doc = "0x1bc..0x1cc - Description collection: Publish configuration for event COMPAREMATCH\\[n\\]"]
    #[inline(always)]
    pub const fn publish_comparematch(&self, n: usize) -> &PublishComparematch {
        &self.publish_comparematch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1bc..0x1cc - Description collection: Publish configuration for event COMPAREMATCH\\[n\\]"]
    #[inline(always)]
    pub fn publish_comparematch_iter(&self) -> impl Iterator<Item = &PublishComparematch> {
        self.publish_comparematch.iter()
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
    #[doc = "0x500 - PWM module enable register"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x504 - Selects operating mode of the wave counter"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x508 - Value up to which the pulse generator counter counts"]
    #[inline(always)]
    pub const fn countertop(&self) -> &Countertop {
        &self.countertop
    }
    #[doc = "0x50c - Configuration for PWM_CLK"]
    #[inline(always)]
    pub const fn prescaler(&self) -> &Prescaler {
        &self.prescaler
    }
    #[doc = "0x510 - Configuration of the decoder"]
    #[inline(always)]
    pub const fn decoder(&self) -> &Decoder {
        &self.decoder
    }
    #[doc = "0x514 - Number of playbacks of a loop"]
    #[inline(always)]
    pub const fn loop_(&self) -> &Loop {
        &self.loop_
    }
    #[doc = "0x518 - Configure the output value on the PWM channel during idle"]
    #[inline(always)]
    pub const fn idleout(&self) -> &Idleout {
        &self.idleout
    }
    #[doc = "0x520..0x540 - Unspecified"]
    #[inline(always)]
    pub const fn seq(&self, n: usize) -> &Seq {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1312)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x520..0x540 - Unspecified"]
    #[inline(always)]
    pub fn seq_iter(&self) -> impl Iterator<Item = &Seq> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1312)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x560..0x570 - Unspecified"]
    #[inline(always)]
    pub const fn psel(&self) -> &Psel {
        &self.psel
    }
    #[doc = "0x700..0x748 - Unspecified"]
    #[inline(always)]
    pub const fn dma(&self) -> &Dma {
        &self.dma
    }
}
#[doc = "TASKS_STOP (w) register accessor: Stops PWM pulse generation on all channels at the end of current PWM period, and stops sequence playback\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`]
module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stops PWM pulse generation on all channels at the end of current PWM period, and stops sequence playback"]
pub mod tasks_stop;
#[doc = "TASKS_NEXTSTEP (w) register accessor: Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_nextstep::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_nextstep`]
module"]
#[doc(alias = "TASKS_NEXTSTEP")]
pub type TasksNextstep = crate::Reg<tasks_nextstep::TasksNextstepSpec>;
#[doc = "Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running."]
pub mod tasks_nextstep;
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
#[doc = "SUBSCRIBE_NEXTSTEP (rw) register accessor: Subscribe configuration for task NEXTSTEP\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_nextstep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_nextstep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_nextstep`]
module"]
#[doc(alias = "SUBSCRIBE_NEXTSTEP")]
pub type SubscribeNextstep = crate::Reg<subscribe_nextstep::SubscribeNextstepSpec>;
#[doc = "Subscribe configuration for task NEXTSTEP"]
pub mod subscribe_nextstep;
#[doc = "Subscribe configuration for tasks"]
pub use self::subscribe_dma::SubscribeDma;
#[doc = r"Cluster"]
#[doc = "Subscribe configuration for tasks"]
pub mod subscribe_dma;
#[doc = "EVENTS_STOPPED (rw) register accessor: Response to STOP task, emitted when PWM pulses are no longer generated\n\nYou can [`read`](crate::Reg::read) this register and get [`events_stopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_stopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_stopped`]
module"]
#[doc(alias = "EVENTS_STOPPED")]
pub type EventsStopped = crate::Reg<events_stopped::EventsStoppedSpec>;
#[doc = "Response to STOP task, emitted when PWM pulses are no longer generated"]
pub mod events_stopped;
#[doc = "EVENTS_SEQSTARTED (rw) register accessor: Description collection: First PWM period started on sequence n\n\nYou can [`read`](crate::Reg::read) this register and get [`events_seqstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_seqstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_seqstarted`]
module"]
#[doc(alias = "EVENTS_SEQSTARTED")]
pub type EventsSeqstarted = crate::Reg<events_seqstarted::EventsSeqstartedSpec>;
#[doc = "Description collection: First PWM period started on sequence n"]
pub mod events_seqstarted;
#[doc = "EVENTS_SEQEND (rw) register accessor: Description collection: Emitted at end of every sequence n, when last value from RAM has been applied to wave counter\n\nYou can [`read`](crate::Reg::read) this register and get [`events_seqend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_seqend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_seqend`]
module"]
#[doc(alias = "EVENTS_SEQEND")]
pub type EventsSeqend = crate::Reg<events_seqend::EventsSeqendSpec>;
#[doc = "Description collection: Emitted at end of every sequence n, when last value from RAM has been applied to wave counter"]
pub mod events_seqend;
#[doc = "EVENTS_PWMPERIODEND (rw) register accessor: Emitted at the end of each PWM period\n\nYou can [`read`](crate::Reg::read) this register and get [`events_pwmperiodend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_pwmperiodend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_pwmperiodend`]
module"]
#[doc(alias = "EVENTS_PWMPERIODEND")]
pub type EventsPwmperiodend = crate::Reg<events_pwmperiodend::EventsPwmperiodendSpec>;
#[doc = "Emitted at the end of each PWM period"]
pub mod events_pwmperiodend;
#[doc = "EVENTS_LOOPSDONE (rw) register accessor: Concatenated sequences have been played the amount of times defined in LOOP.CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`events_loopsdone::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_loopsdone::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_loopsdone`]
module"]
#[doc(alias = "EVENTS_LOOPSDONE")]
pub type EventsLoopsdone = crate::Reg<events_loopsdone::EventsLoopsdoneSpec>;
#[doc = "Concatenated sequences have been played the amount of times defined in LOOP.CNT"]
pub mod events_loopsdone;
#[doc = "EVENTS_RAMUNDERFLOW (rw) register accessor: Emitted when retrieving from RAM does not complete in time for the PWM module\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ramunderflow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ramunderflow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ramunderflow`]
module"]
#[doc(alias = "EVENTS_RAMUNDERFLOW")]
pub type EventsRamunderflow = crate::Reg<events_ramunderflow::EventsRamunderflowSpec>;
#[doc = "Emitted when retrieving from RAM does not complete in time for the PWM module"]
pub mod events_ramunderflow;
#[doc = "Peripheral events."]
pub use self::events_dma::EventsDma;
#[doc = r"Cluster"]
#[doc = "Peripheral events."]
pub mod events_dma;
#[doc = "EVENTS_COMPAREMATCH (rw) register accessor: Description collection: This event is generated when the compare matches for the compare channel \\[n\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`events_comparematch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_comparematch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_comparematch`]
module"]
#[doc(alias = "EVENTS_COMPAREMATCH")]
pub type EventsComparematch = crate::Reg<events_comparematch::EventsComparematchSpec>;
#[doc = "Description collection: This event is generated when the compare matches for the compare channel \\[n\\]."]
pub mod events_comparematch;
#[doc = "PUBLISH_STOPPED (rw) register accessor: Publish configuration for event STOPPED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_stopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_stopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_stopped`]
module"]
#[doc(alias = "PUBLISH_STOPPED")]
pub type PublishStopped = crate::Reg<publish_stopped::PublishStoppedSpec>;
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "PUBLISH_SEQSTARTED (rw) register accessor: Description collection: Publish configuration for event SEQSTARTED\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_seqstarted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_seqstarted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_seqstarted`]
module"]
#[doc(alias = "PUBLISH_SEQSTARTED")]
pub type PublishSeqstarted = crate::Reg<publish_seqstarted::PublishSeqstartedSpec>;
#[doc = "Description collection: Publish configuration for event SEQSTARTED\\[n\\]"]
pub mod publish_seqstarted;
#[doc = "PUBLISH_SEQEND (rw) register accessor: Description collection: Publish configuration for event SEQEND\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_seqend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_seqend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_seqend`]
module"]
#[doc(alias = "PUBLISH_SEQEND")]
pub type PublishSeqend = crate::Reg<publish_seqend::PublishSeqendSpec>;
#[doc = "Description collection: Publish configuration for event SEQEND\\[n\\]"]
pub mod publish_seqend;
#[doc = "PUBLISH_PWMPERIODEND (rw) register accessor: Publish configuration for event PWMPERIODEND\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_pwmperiodend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_pwmperiodend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_pwmperiodend`]
module"]
#[doc(alias = "PUBLISH_PWMPERIODEND")]
pub type PublishPwmperiodend = crate::Reg<publish_pwmperiodend::PublishPwmperiodendSpec>;
#[doc = "Publish configuration for event PWMPERIODEND"]
pub mod publish_pwmperiodend;
#[doc = "PUBLISH_LOOPSDONE (rw) register accessor: Publish configuration for event LOOPSDONE\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_loopsdone::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_loopsdone::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_loopsdone`]
module"]
#[doc(alias = "PUBLISH_LOOPSDONE")]
pub type PublishLoopsdone = crate::Reg<publish_loopsdone::PublishLoopsdoneSpec>;
#[doc = "Publish configuration for event LOOPSDONE"]
pub mod publish_loopsdone;
#[doc = "PUBLISH_RAMUNDERFLOW (rw) register accessor: Publish configuration for event RAMUNDERFLOW\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_ramunderflow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_ramunderflow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_ramunderflow`]
module"]
#[doc(alias = "PUBLISH_RAMUNDERFLOW")]
pub type PublishRamunderflow = crate::Reg<publish_ramunderflow::PublishRamunderflowSpec>;
#[doc = "Publish configuration for event RAMUNDERFLOW"]
pub mod publish_ramunderflow;
#[doc = "Publish configuration for events"]
pub use self::publish_dma::PublishDma;
#[doc = r"Cluster"]
#[doc = "Publish configuration for events"]
pub mod publish_dma;
#[doc = "PUBLISH_COMPAREMATCH (rw) register accessor: Description collection: Publish configuration for event COMPAREMATCH\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_comparematch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_comparematch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_comparematch`]
module"]
#[doc(alias = "PUBLISH_COMPAREMATCH")]
pub type PublishComparematch = crate::Reg<publish_comparematch::PublishComparematchSpec>;
#[doc = "Description collection: Publish configuration for event COMPAREMATCH\\[n\\]"]
pub mod publish_comparematch;
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
#[doc = "ENABLE (rw) register accessor: PWM module enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "PWM module enable register"]
pub mod enable;
#[doc = "MODE (rw) register accessor: Selects operating mode of the wave counter\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`]
module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "Selects operating mode of the wave counter"]
pub mod mode;
#[doc = "COUNTERTOP (rw) register accessor: Value up to which the pulse generator counter counts\n\nYou can [`read`](crate::Reg::read) this register and get [`countertop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`countertop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@countertop`]
module"]
#[doc(alias = "COUNTERTOP")]
pub type Countertop = crate::Reg<countertop::CountertopSpec>;
#[doc = "Value up to which the pulse generator counter counts"]
pub mod countertop;
#[doc = "PRESCALER (rw) register accessor: Configuration for PWM_CLK\n\nYou can [`read`](crate::Reg::read) this register and get [`prescaler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prescaler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prescaler`]
module"]
#[doc(alias = "PRESCALER")]
pub type Prescaler = crate::Reg<prescaler::PrescalerSpec>;
#[doc = "Configuration for PWM_CLK"]
pub mod prescaler;
#[doc = "DECODER (rw) register accessor: Configuration of the decoder\n\nYou can [`read`](crate::Reg::read) this register and get [`decoder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decoder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@decoder`]
module"]
#[doc(alias = "DECODER")]
pub type Decoder = crate::Reg<decoder::DecoderSpec>;
#[doc = "Configuration of the decoder"]
pub mod decoder;
#[doc = "LOOP (rw) register accessor: Number of playbacks of a loop\n\nYou can [`read`](crate::Reg::read) this register and get [`loop_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loop_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@loop_`]
module"]
#[doc(alias = "LOOP")]
pub type Loop = crate::Reg<loop_::LoopSpec>;
#[doc = "Number of playbacks of a loop"]
pub mod loop_;
#[doc = "IDLEOUT (rw) register accessor: Configure the output value on the PWM channel during idle\n\nYou can [`read`](crate::Reg::read) this register and get [`idleout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idleout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idleout`]
module"]
#[doc(alias = "IDLEOUT")]
pub type Idleout = crate::Reg<idleout::IdleoutSpec>;
#[doc = "Configure the output value on the PWM channel during idle"]
pub mod idleout;
#[doc = "Unspecified"]
pub use self::seq::Seq;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod seq;
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
