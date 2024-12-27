#[repr(C)]
#[doc = "Enable slow domain glitch detectors."]
#[doc(alias = "GLITCHSLOWDOMAIN")]
pub struct Glitchslowdomain {
    ctrl: Ctrl,
    status: Status,
}
impl Glitchslowdomain {
    #[doc = "0x00 - Control register for slow domain glitch detectors enable signal."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Status register for slow domain glitch detectors enable signal."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
}
#[doc = "CTRL (rw) register accessor: Control register for slow domain glitch detectors enable signal.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register for slow domain glitch detectors enable signal."]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: Status register for slow domain glitch detectors enable signal.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status register for slow domain glitch detectors enable signal."]
pub mod status;
