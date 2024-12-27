#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "SYSCOUNTER")]
pub struct Syscounter {
    syscounterl: Syscounterl,
    syscounterh: Syscounterh,
    active: Active,
}
impl Syscounter {
    #[doc = "0x00 - Description cluster: The lower 32-bits of the SYSCOUNTER for index \\[n\\]"]
    #[inline(always)]
    pub const fn syscounterl(&self) -> &Syscounterl {
        &self.syscounterl
    }
    #[doc = "0x04 - Description cluster: The higher 20-bits of the SYSCOUNTER for index \\[n\\]"]
    #[inline(always)]
    pub const fn syscounterh(&self) -> &Syscounterh {
        &self.syscounterh
    }
    #[doc = "0x08 - Description cluster: Request to keep the SYSCOUNTER in the active state and prevent going to sleep for index \\[n\\]"]
    #[inline(always)]
    pub const fn active(&self) -> &Active {
        &self.active
    }
}
#[doc = "SYSCOUNTERL (r) register accessor: Description cluster: The lower 32-bits of the SYSCOUNTER for index \\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`syscounterl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscounterl`]
module"]
#[doc(alias = "SYSCOUNTERL")]
pub type Syscounterl = crate::Reg<syscounterl::SyscounterlSpec>;
#[doc = "Description cluster: The lower 32-bits of the SYSCOUNTER for index \\[n\\]"]
pub mod syscounterl;
#[doc = "SYSCOUNTERH (r) register accessor: Description cluster: The higher 20-bits of the SYSCOUNTER for index \\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`syscounterh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscounterh`]
module"]
#[doc(alias = "SYSCOUNTERH")]
pub type Syscounterh = crate::Reg<syscounterh::SyscounterhSpec>;
#[doc = "Description cluster: The higher 20-bits of the SYSCOUNTER for index \\[n\\]"]
pub mod syscounterh;
#[doc = "ACTIVE (rw) register accessor: Description cluster: Request to keep the SYSCOUNTER in the active state and prevent going to sleep for index \\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`active::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`active::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@active`]
module"]
#[doc(alias = "ACTIVE")]
pub type Active = crate::Reg<active::ActiveSpec>;
#[doc = "Description cluster: Request to keep the SYSCOUNTER in the active state and prevent going to sleep for index \\[n\\]"]
pub mod active;
