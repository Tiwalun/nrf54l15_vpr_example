#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_capture: [TasksCapture; 12],
    _reserved1: [u8; 0x30],
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    tasks_clear: TasksClear,
    tasks_pwmstart: TasksPwmstart,
    tasks_pwmstop: TasksPwmstop,
    _reserved6: [u8; 0x0c],
    subscribe_capture: [SubscribeCapture; 12],
    _reserved7: [u8; 0x50],
    events_compare: [EventsCompare; 12],
    _reserved8: [u8; 0x34],
    events_rtcomparesync: EventsRtcomparesync,
    events_syscountervalid: EventsSyscountervalid,
    events_pwmperiodend: EventsPwmperiodend,
    _reserved11: [u8; 0x10],
    publish_compare: [PublishCompare; 12],
    _reserved12: [u8; 0x50],
    shorts: Shorts,
    _reserved13: [u8; 0xfc],
    inten0: Inten0,
    intenset0: Intenset0,
    intenclr0: Intenclr0,
    intpend0: Intpend0,
    inten1: Inten1,
    intenset1: Intenset1,
    intenclr1: Intenclr1,
    intpend1: Intpend1,
    inten2: Inten2,
    intenset2: Intenset2,
    intenclr2: Intenclr2,
    intpend2: Intpend2,
    inten3: Inten3,
    intenset3: Intenset3,
    intenclr3: Intenclr3,
    intpend3: Intpend3,
    _reserved29: [u8; 0xc0],
    evten: Evten,
    evtenset: Evtenset,
    evtenclr: Evtenclr,
    _reserved32: [u8; 0x0104],
    mode: Mode,
    _reserved33: [u8; 0x0c],
    cc: [Cc; 12],
    _reserved34: [u8; 0xc0],
    keeprunning: Keeprunning,
    timeout: Timeout,
    interval: Interval,
    waketime: Waketime,
    _reserved38: [u8; 0x60],
    pwmconfig: Pwmconfig,
    clkout: Clkout,
    clkcfg: Clkcfg,
    _reserved41: [u8; 0x04],
    syscounter: (),
}
impl RegisterBlock {
    #[doc = "0x00..0x30 - Description collection: Capture the counter value to CC\\[n\\]
register"]
    #[inline(always)]
    pub const fn tasks_capture(&self, n: usize) -> &TasksCapture {
        &self.tasks_capture[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x30 - Description collection: Capture the counter value to CC\\[n\\]
register"]
    #[inline(always)]
    pub fn tasks_capture_iter(&self) -> impl Iterator<Item = &TasksCapture> {
        self.tasks_capture.iter()
    }
    #[doc = "0x60 - Start the counter"]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x64 - Stop the counter"]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x68 - Clear the counter"]
    #[inline(always)]
    pub const fn tasks_clear(&self) -> &TasksClear {
        &self.tasks_clear
    }
    #[doc = "0x6c - Start the PWM"]
    #[inline(always)]
    pub const fn tasks_pwmstart(&self) -> &TasksPwmstart {
        &self.tasks_pwmstart
    }
    #[doc = "0x70 - Stop the PWM"]
    #[inline(always)]
    pub const fn tasks_pwmstop(&self) -> &TasksPwmstop {
        &self.tasks_pwmstop
    }
    #[doc = "0x80..0xb0 - Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
    #[inline(always)]
    pub const fn subscribe_capture(&self, n: usize) -> &SubscribeCapture {
        &self.subscribe_capture[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xb0 - Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
    #[inline(always)]
    pub fn subscribe_capture_iter(&self) -> impl Iterator<Item = &SubscribeCapture> {
        self.subscribe_capture.iter()
    }
    #[doc = "0x100..0x130 - Description collection: Compare event on CC\\[n\\]
match"]
    #[inline(always)]
    pub const fn events_compare(&self, n: usize) -> &EventsCompare {
        &self.events_compare[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x130 - Description collection: Compare event on CC\\[n\\]
match"]
    #[inline(always)]
    pub fn events_compare_iter(&self) -> impl Iterator<Item = &EventsCompare> {
        self.events_compare.iter()
    }
    #[doc = "0x164 - Synchronize always-on LFCLK clock domain"]
    #[inline(always)]
    pub const fn events_rtcomparesync(&self) -> &EventsRtcomparesync {
        &self.events_rtcomparesync
    }
    #[doc = "0x168 - The SYSCOUNTER is in active state and value is valid"]
    #[inline(always)]
    pub const fn events_syscountervalid(&self) -> &EventsSyscountervalid {
        &self.events_syscountervalid
    }
    #[doc = "0x16c - Event on end of each PWM period"]
    #[inline(always)]
    pub const fn events_pwmperiodend(&self) -> &EventsPwmperiodend {
        &self.events_pwmperiodend
    }
    #[doc = "0x180..0x1b0 - Description collection: Publish configuration for event COMPARE\\[n\\]"]
    #[inline(always)]
    pub const fn publish_compare(&self, n: usize) -> &PublishCompare {
        &self.publish_compare[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x1b0 - Description collection: Publish configuration for event COMPARE\\[n\\]"]
    #[inline(always)]
    pub fn publish_compare_iter(&self) -> impl Iterator<Item = &PublishCompare> {
        self.publish_compare.iter()
    }
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(&self) -> &Shorts {
        &self.shorts
    }
    #[doc = "0x300 - Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten0(&self) -> &Inten0 {
        &self.inten0
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
    #[doc = "0x30c - Pending interrupts"]
    #[inline(always)]
    pub const fn intpend0(&self) -> &Intpend0 {
        &self.intpend0
    }
    #[doc = "0x310 - Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten1(&self) -> &Inten1 {
        &self.inten1
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
    #[doc = "0x31c - Pending interrupts"]
    #[inline(always)]
    pub const fn intpend1(&self) -> &Intpend1 {
        &self.intpend1
    }
    #[doc = "0x320 - Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten2(&self) -> &Inten2 {
        &self.inten2
    }
    #[doc = "0x324 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset2(&self) -> &Intenset2 {
        &self.intenset2
    }
    #[doc = "0x328 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr2(&self) -> &Intenclr2 {
        &self.intenclr2
    }
    #[doc = "0x32c - Pending interrupts"]
    #[inline(always)]
    pub const fn intpend2(&self) -> &Intpend2 {
        &self.intpend2
    }
    #[doc = "0x330 - Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten3(&self) -> &Inten3 {
        &self.inten3
    }
    #[doc = "0x334 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset3(&self) -> &Intenset3 {
        &self.intenset3
    }
    #[doc = "0x338 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr3(&self) -> &Intenclr3 {
        &self.intenclr3
    }
    #[doc = "0x33c - Pending interrupts"]
    #[inline(always)]
    pub const fn intpend3(&self) -> &Intpend3 {
        &self.intpend3
    }
    #[doc = "0x400 - Enable or disable event routing"]
    #[inline(always)]
    pub const fn evten(&self) -> &Evten {
        &self.evten
    }
    #[doc = "0x404 - Enable event routing"]
    #[inline(always)]
    pub const fn evtenset(&self) -> &Evtenset {
        &self.evtenset
    }
    #[doc = "0x408 - Disable event routing"]
    #[inline(always)]
    pub const fn evtenclr(&self) -> &Evtenclr {
        &self.evtenclr
    }
    #[doc = "0x510 - Counter mode selection"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x520..0x5e0 - Unspecified"]
    #[inline(always)]
    pub const fn cc(&self, n: usize) -> &Cc {
        &self.cc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x520..0x5e0 - Unspecified"]
    #[inline(always)]
    pub fn cc_iter(&self) -> impl Iterator<Item = &Cc> {
        self.cc.iter()
    }
    #[doc = "0x6a0 - Request to keep the SYSCOUNTER in the active state and prevent going to sleep"]
    #[inline(always)]
    pub const fn keeprunning(&self) -> &Keeprunning {
        &self.keeprunning
    }
    #[doc = "0x6a4 - Timeout after all CPUs gone into sleep state to stop the SYSCOUNTER"]
    #[inline(always)]
    pub const fn timeout(&self) -> &Timeout {
        &self.timeout
    }
    #[doc = "0x6a8 - Count to add to CC\\[0\\]
when the event EVENTS_COMPARE\\[0\\]
triggers."]
    #[inline(always)]
    pub const fn interval(&self) -> &Interval {
        &self.interval
    }
    #[doc = "0x6ac - GRTC wake up time."]
    #[inline(always)]
    pub const fn waketime(&self) -> &Waketime {
        &self.waketime
    }
    #[doc = "0x710 - PWM configuration."]
    #[inline(always)]
    pub const fn pwmconfig(&self) -> &Pwmconfig {
        &self.pwmconfig
    }
    #[doc = "0x714 - Configuration of clock output"]
    #[inline(always)]
    pub const fn clkout(&self) -> &Clkout {
        &self.clkout
    }
    #[doc = "0x718 - Clock Configuration"]
    #[inline(always)]
    pub const fn clkcfg(&self) -> &Clkcfg {
        &self.clkcfg
    }
    #[doc = "0x720..0x750 - Unspecified"]
    #[inline(always)]
    pub const fn syscounter(&self, n: usize) -> &Syscounter {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1824)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x720..0x750 - Unspecified"]
    #[inline(always)]
    pub fn syscounter_iter(&self) -> impl Iterator<Item = &Syscounter> {
        (0..4).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1824)
                .add(16 * n)
                .cast()
        })
    }
}
#[doc = "TASKS_CAPTURE (w) register accessor: Description collection: Capture the counter value to CC\\[n\\]
register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_capture::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_capture`]
module"]
#[doc(alias = "TASKS_CAPTURE")]
pub type TasksCapture = crate::Reg<tasks_capture::TasksCaptureSpec>;
#[doc = "Description collection: Capture the counter value to CC\\[n\\]
register"]
pub mod tasks_capture;
#[doc = "TASKS_START (w) register accessor: Start the counter\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`]
module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Start the counter"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Stop the counter\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`]
module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop the counter"]
pub mod tasks_stop;
#[doc = "TASKS_CLEAR (w) register accessor: Clear the counter\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_clear`]
module"]
#[doc(alias = "TASKS_CLEAR")]
pub type TasksClear = crate::Reg<tasks_clear::TasksClearSpec>;
#[doc = "Clear the counter"]
pub mod tasks_clear;
#[doc = "TASKS_PWMSTART (w) register accessor: Start the PWM\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_pwmstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_pwmstart`]
module"]
#[doc(alias = "TASKS_PWMSTART")]
pub type TasksPwmstart = crate::Reg<tasks_pwmstart::TasksPwmstartSpec>;
#[doc = "Start the PWM"]
pub mod tasks_pwmstart;
#[doc = "TASKS_PWMSTOP (w) register accessor: Stop the PWM\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_pwmstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_pwmstop`]
module"]
#[doc(alias = "TASKS_PWMSTOP")]
pub type TasksPwmstop = crate::Reg<tasks_pwmstop::TasksPwmstopSpec>;
#[doc = "Stop the PWM"]
pub mod tasks_pwmstop;
#[doc = "SUBSCRIBE_CAPTURE (rw) register accessor: Description collection: Subscribe configuration for task CAPTURE\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_capture::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_capture::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_capture`]
module"]
#[doc(alias = "SUBSCRIBE_CAPTURE")]
pub type SubscribeCapture = crate::Reg<subscribe_capture::SubscribeCaptureSpec>;
#[doc = "Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
pub mod subscribe_capture;
#[doc = "EVENTS_COMPARE (rw) register accessor: Description collection: Compare event on CC\\[n\\]
match\n\nYou can [`read`](crate::Reg::read) this register and get [`events_compare::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_compare::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_compare`]
module"]
#[doc(alias = "EVENTS_COMPARE")]
pub type EventsCompare = crate::Reg<events_compare::EventsCompareSpec>;
#[doc = "Description collection: Compare event on CC\\[n\\]
match"]
pub mod events_compare;
#[doc = "EVENTS_RTCOMPARESYNC (rw) register accessor: Synchronize always-on LFCLK clock domain\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rtcomparesync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rtcomparesync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rtcomparesync`]
module"]
#[doc(alias = "EVENTS_RTCOMPARESYNC")]
pub type EventsRtcomparesync = crate::Reg<events_rtcomparesync::EventsRtcomparesyncSpec>;
#[doc = "Synchronize always-on LFCLK clock domain"]
pub mod events_rtcomparesync;
#[doc = "EVENTS_SYSCOUNTERVALID (rw) register accessor: The SYSCOUNTER is in active state and value is valid\n\nYou can [`read`](crate::Reg::read) this register and get [`events_syscountervalid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_syscountervalid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_syscountervalid`]
module"]
#[doc(alias = "EVENTS_SYSCOUNTERVALID")]
pub type EventsSyscountervalid = crate::Reg<events_syscountervalid::EventsSyscountervalidSpec>;
#[doc = "The SYSCOUNTER is in active state and value is valid"]
pub mod events_syscountervalid;
#[doc = "EVENTS_PWMPERIODEND (rw) register accessor: Event on end of each PWM period\n\nYou can [`read`](crate::Reg::read) this register and get [`events_pwmperiodend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_pwmperiodend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_pwmperiodend`]
module"]
#[doc(alias = "EVENTS_PWMPERIODEND")]
pub type EventsPwmperiodend = crate::Reg<events_pwmperiodend::EventsPwmperiodendSpec>;
#[doc = "Event on end of each PWM period"]
pub mod events_pwmperiodend;
#[doc = "PUBLISH_COMPARE (rw) register accessor: Description collection: Publish configuration for event COMPARE\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_compare::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_compare::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_compare`]
module"]
#[doc(alias = "PUBLISH_COMPARE")]
pub type PublishCompare = crate::Reg<publish_compare::PublishCompareSpec>;
#[doc = "Description collection: Publish configuration for event COMPARE\\[n\\]"]
pub mod publish_compare;
#[doc = "SHORTS (rw) register accessor: Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`]
module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTEN0 (rw) register accessor: Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten0`]
module"]
#[doc(alias = "INTEN0")]
pub type Inten0 = crate::Reg<inten0::Inten0Spec>;
#[doc = "Enable or disable interrupt"]
pub mod inten0;
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
#[doc = "INTPEND0 (r) register accessor: Pending interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`intpend0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpend0`]
module"]
#[doc(alias = "INTPEND0")]
pub type Intpend0 = crate::Reg<intpend0::Intpend0Spec>;
#[doc = "Pending interrupts"]
pub mod intpend0;
#[doc = "INTEN1 (rw) register accessor: Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten1`]
module"]
#[doc(alias = "INTEN1")]
pub type Inten1 = crate::Reg<inten1::Inten1Spec>;
#[doc = "Enable or disable interrupt"]
pub mod inten1;
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
#[doc = "INTPEND1 (r) register accessor: Pending interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`intpend1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpend1`]
module"]
#[doc(alias = "INTPEND1")]
pub type Intpend1 = crate::Reg<intpend1::Intpend1Spec>;
#[doc = "Pending interrupts"]
pub mod intpend1;
#[doc = "INTEN2 (rw) register accessor: Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten2`]
module"]
#[doc(alias = "INTEN2")]
pub type Inten2 = crate::Reg<inten2::Inten2Spec>;
#[doc = "Enable or disable interrupt"]
pub mod inten2;
#[doc = "INTENSET2 (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset2`]
module"]
#[doc(alias = "INTENSET2")]
pub type Intenset2 = crate::Reg<intenset2::Intenset2Spec>;
#[doc = "Enable interrupt"]
pub mod intenset2;
#[doc = "INTENCLR2 (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr2`]
module"]
#[doc(alias = "INTENCLR2")]
pub type Intenclr2 = crate::Reg<intenclr2::Intenclr2Spec>;
#[doc = "Disable interrupt"]
pub mod intenclr2;
#[doc = "INTPEND2 (r) register accessor: Pending interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`intpend2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpend2`]
module"]
#[doc(alias = "INTPEND2")]
pub type Intpend2 = crate::Reg<intpend2::Intpend2Spec>;
#[doc = "Pending interrupts"]
pub mod intpend2;
#[doc = "INTEN3 (rw) register accessor: Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten3`]
module"]
#[doc(alias = "INTEN3")]
pub type Inten3 = crate::Reg<inten3::Inten3Spec>;
#[doc = "Enable or disable interrupt"]
pub mod inten3;
#[doc = "INTENSET3 (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset3`]
module"]
#[doc(alias = "INTENSET3")]
pub type Intenset3 = crate::Reg<intenset3::Intenset3Spec>;
#[doc = "Enable interrupt"]
pub mod intenset3;
#[doc = "INTENCLR3 (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr3`]
module"]
#[doc(alias = "INTENCLR3")]
pub type Intenclr3 = crate::Reg<intenclr3::Intenclr3Spec>;
#[doc = "Disable interrupt"]
pub mod intenclr3;
#[doc = "INTPEND3 (r) register accessor: Pending interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`intpend3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpend3`]
module"]
#[doc(alias = "INTPEND3")]
pub type Intpend3 = crate::Reg<intpend3::Intpend3Spec>;
#[doc = "Pending interrupts"]
pub mod intpend3;
#[doc = "EVTEN (rw) register accessor: Enable or disable event routing\n\nYou can [`read`](crate::Reg::read) this register and get [`evten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evten`]
module"]
#[doc(alias = "EVTEN")]
pub type Evten = crate::Reg<evten::EvtenSpec>;
#[doc = "Enable or disable event routing"]
pub mod evten;
#[doc = "EVTENSET (rw) register accessor: Enable event routing\n\nYou can [`read`](crate::Reg::read) this register and get [`evtenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evtenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtenset`]
module"]
#[doc(alias = "EVTENSET")]
pub type Evtenset = crate::Reg<evtenset::EvtensetSpec>;
#[doc = "Enable event routing"]
pub mod evtenset;
#[doc = "EVTENCLR (rw) register accessor: Disable event routing\n\nYou can [`read`](crate::Reg::read) this register and get [`evtenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evtenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtenclr`]
module"]
#[doc(alias = "EVTENCLR")]
pub type Evtenclr = crate::Reg<evtenclr::EvtenclrSpec>;
#[doc = "Disable event routing"]
pub mod evtenclr;
#[doc = "MODE (rw) register accessor: Counter mode selection\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`]
module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "Counter mode selection"]
pub mod mode;
#[doc = "Unspecified"]
pub use self::cc::Cc;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod cc;
#[doc = "KEEPRUNNING (rw) register accessor: Request to keep the SYSCOUNTER in the active state and prevent going to sleep\n\nYou can [`read`](crate::Reg::read) this register and get [`keeprunning::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keeprunning::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keeprunning`]
module"]
#[doc(alias = "KEEPRUNNING")]
pub type Keeprunning = crate::Reg<keeprunning::KeeprunningSpec>;
#[doc = "Request to keep the SYSCOUNTER in the active state and prevent going to sleep"]
pub mod keeprunning;
#[doc = "TIMEOUT (rw) register accessor: Timeout after all CPUs gone into sleep state to stop the SYSCOUNTER\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout`]
module"]
#[doc(alias = "TIMEOUT")]
pub type Timeout = crate::Reg<timeout::TimeoutSpec>;
#[doc = "Timeout after all CPUs gone into sleep state to stop the SYSCOUNTER"]
pub mod timeout;
#[doc = "INTERVAL (rw) register accessor: Count to add to CC\\[0\\]
when the event EVENTS_COMPARE\\[0\\]
triggers.\n\nYou can [`read`](crate::Reg::read) this register and get [`interval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interval`]
module"]
#[doc(alias = "INTERVAL")]
pub type Interval = crate::Reg<interval::IntervalSpec>;
#[doc = "Count to add to CC\\[0\\]
when the event EVENTS_COMPARE\\[0\\]
triggers."]
pub mod interval;
#[doc = "WAKETIME (rw) register accessor: GRTC wake up time.\n\nYou can [`read`](crate::Reg::read) this register and get [`waketime::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`waketime::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@waketime`]
module"]
#[doc(alias = "WAKETIME")]
pub type Waketime = crate::Reg<waketime::WaketimeSpec>;
#[doc = "GRTC wake up time."]
pub mod waketime;
#[doc = "PWMCONFIG (rw) register accessor: PWM configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmconfig`]
module"]
#[doc(alias = "PWMCONFIG")]
pub type Pwmconfig = crate::Reg<pwmconfig::PwmconfigSpec>;
#[doc = "PWM configuration."]
pub mod pwmconfig;
#[doc = "CLKOUT (rw) register accessor: Configuration of clock output\n\nYou can [`read`](crate::Reg::read) this register and get [`clkout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkout`]
module"]
#[doc(alias = "CLKOUT")]
pub type Clkout = crate::Reg<clkout::ClkoutSpec>;
#[doc = "Configuration of clock output"]
pub mod clkout;
#[doc = "CLKCFG (rw) register accessor: Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcfg`]
module"]
#[doc(alias = "CLKCFG")]
pub type Clkcfg = crate::Reg<clkcfg::ClkcfgSpec>;
#[doc = "Clock Configuration"]
pub mod clkcfg;
#[doc = "Unspecified"]
pub use self::syscounter::Syscounter;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod syscounter;
