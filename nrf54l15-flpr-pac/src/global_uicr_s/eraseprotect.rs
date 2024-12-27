#[repr(C)]
#[doc = "Erase Protection Registers"]
#[doc(alias = "ERASEPROTECT")]
pub struct Eraseprotect {
    protect0: Protect0,
    _reserved1: [u8; 0x18],
    protect1: Protect1,
}
impl Eraseprotect {
    #[doc = "0x00 - Description cluster: Erase protection"]
    #[inline(always)]
    pub const fn protect0(&self) -> &Protect0 {
        &self.protect0
    }
    #[doc = "0x1c - Description cluster: Erase protection"]
    #[inline(always)]
    pub const fn protect1(&self) -> &Protect1 {
        &self.protect1
    }
}
#[doc = "PROTECT0 (rw) register accessor: Description cluster: Erase protection\n\nYou can [`read`](crate::Reg::read) this register and get [`protect0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`protect0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@protect0`]
module"]
#[doc(alias = "PROTECT0")]
pub type Protect0 = crate::Reg<protect0::Protect0Spec>;
#[doc = "Description cluster: Erase protection"]
pub mod protect0;
#[doc = "PROTECT1 (rw) register accessor: Description cluster: Erase protection\n\nYou can [`read`](crate::Reg::read) this register and get [`protect1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`protect1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@protect1`]
module"]
#[doc(alias = "PROTECT1")]
pub type Protect1 = crate::Reg<protect1::Protect1Spec>;
#[doc = "Description cluster: Erase protection"]
pub mod protect1;
