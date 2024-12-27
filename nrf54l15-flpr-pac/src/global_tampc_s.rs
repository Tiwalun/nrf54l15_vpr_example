#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    events_tamper: EventsTamper,
    events_writeerror: EventsWriteerror,
    _reserved2: [u8; 0x01f8],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    intpend: Intpend,
    _reserved6: [u8; 0xf0],
    status: Status,
    activeshield: Activeshield,
    _reserved8: [u8; 0xf8],
    protect: Protect,
}
impl RegisterBlock {
    #[doc = "0x100 - Tamper controller detected an error."]
    #[inline(always)]
    pub const fn events_tamper(&self) -> &EventsTamper {
        &self.events_tamper
    }
    #[doc = "0x104 - Attempt to write a VALUE in PROTECT registers without clearing the WRITEPROTECT."]
    #[inline(always)]
    pub const fn events_writeerror(&self) -> &EventsWriteerror {
        &self.events_writeerror
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
    #[doc = "0x400 - The tamper controller status."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x404 - Unspecified"]
    #[inline(always)]
    pub const fn activeshield(&self) -> &Activeshield {
        &self.activeshield
    }
    #[doc = "0x500..0x988 - Unspecified"]
    #[inline(always)]
    pub const fn protect(&self) -> &Protect {
        &self.protect
    }
}
#[doc = "EVENTS_TAMPER (rw) register accessor: Tamper controller detected an error.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_tamper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_tamper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_tamper`]
module"]
#[doc(alias = "EVENTS_TAMPER")]
pub type EventsTamper = crate::Reg<events_tamper::EventsTamperSpec>;
#[doc = "Tamper controller detected an error."]
pub mod events_tamper;
#[doc = "EVENTS_WRITEERROR (rw) register accessor: Attempt to write a VALUE in PROTECT registers without clearing the WRITEPROTECT.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_writeerror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_writeerror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_writeerror`]
module"]
#[doc(alias = "EVENTS_WRITEERROR")]
pub type EventsWriteerror = crate::Reg<events_writeerror::EventsWriteerrorSpec>;
#[doc = "Attempt to write a VALUE in PROTECT registers without clearing the WRITEPROTECT."]
pub mod events_writeerror;
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
#[doc = "STATUS (rw) register accessor: The tamper controller status.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "The tamper controller status."]
pub mod status;
#[doc = "Unspecified"]
pub use self::activeshield::Activeshield;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod activeshield;
#[doc = "Unspecified"]
pub use self::protect::Protect;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod protect;
