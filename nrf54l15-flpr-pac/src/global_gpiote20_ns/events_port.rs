#[repr(C)]
#[doc = "Peripheral events."]
#[doc(alias = "EVENTS_PORT")]
pub struct EventsPort {
    nonsecure: Nonsecure,
    secure: Secure,
}
impl EventsPort {
    #[doc = "0x00 - Description cluster: Non-secure port event from owner n"]
    #[inline(always)]
    pub const fn nonsecure(&self) -> &Nonsecure {
        &self.nonsecure
    }
    #[doc = "0x04 - Description cluster: Secure port event from owner n"]
    #[inline(always)]
    pub const fn secure(&self) -> &Secure {
        &self.secure
    }
}
#[doc = "NONSECURE (rw) register accessor: Description cluster: Non-secure port event from owner n\n\nYou can [`read`](crate::Reg::read) this register and get [`nonsecure::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonsecure::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nonsecure`]
module"]
#[doc(alias = "NONSECURE")]
pub type Nonsecure = crate::Reg<nonsecure::NonsecureSpec>;
#[doc = "Description cluster: Non-secure port event from owner n"]
pub mod nonsecure;
#[doc = "SECURE (rw) register accessor: Description cluster: Secure port event from owner n\n\nYou can [`read`](crate::Reg::read) this register and get [`secure::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure`]
module"]
#[doc(alias = "SECURE")]
pub type Secure = crate::Reg<secure::SecureSpec>;
#[doc = "Description cluster: Secure port event from owner n"]
pub mod secure;
