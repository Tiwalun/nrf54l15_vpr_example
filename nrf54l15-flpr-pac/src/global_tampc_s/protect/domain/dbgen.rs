#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "DBGEN")]
pub struct Dbgen {
    ctrl: Ctrl,
    status: Status,
}
impl Dbgen {
    #[doc = "0x00 - Description cluster: Control register for invasive (halting) debug enable for the local debug components within domain n."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Description cluster: Status register for invasive (halting) debug enable for domain n."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
}
#[doc = "CTRL (rw) register accessor: Description cluster: Control register for invasive (halting) debug enable for the local debug components within domain n.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Description cluster: Control register for invasive (halting) debug enable for the local debug components within domain n."]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: Description cluster: Status register for invasive (halting) debug enable for domain n.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Description cluster: Status register for invasive (halting) debug enable for domain n."]
pub mod status;
