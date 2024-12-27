#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "POWER")]
pub struct Power {
    control: Control,
    _reserved1: [u8; 0x04],
    ret: Ret,
    ret2: Ret2,
}
impl Power {
    #[doc = "0x00 - Description cluster: Control memory block power."]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x08 - Description cluster: RAM retention for RAM \\[n\\]."]
    #[inline(always)]
    pub const fn ret(&self) -> &Ret {
        &self.ret
    }
    #[doc = "0x0c - Description cluster: RAM retention for the second bank in the RAM block"]
    #[inline(always)]
    pub const fn ret2(&self) -> &Ret2 {
        &self.ret2
    }
}
#[doc = "CONTROL (rw) register accessor: Description cluster: Control memory block power.\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Description cluster: Control memory block power."]
pub mod control;
#[doc = "RET (rw) register accessor: Description cluster: RAM retention for RAM \\[n\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`ret::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret`]
module"]
#[doc(alias = "RET")]
pub type Ret = crate::Reg<ret::RetSpec>;
#[doc = "Description cluster: RAM retention for RAM \\[n\\]."]
pub mod ret;
#[doc = "RET2 (rw) register accessor: Description cluster: RAM retention for the second bank in the RAM block\n\nYou can [`read`](crate::Reg::read) this register and get [`ret2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret2`]
module"]
#[doc(alias = "RET2")]
pub type Ret2 = crate::Reg<ret2::Ret2Spec>;
#[doc = "Description cluster: RAM retention for the second bank in the RAM block"]
pub mod ret2;
