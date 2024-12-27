#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    events_rxready: EventsRxready,
    events_txdone: EventsTxdone,
    _reserved2: [u8; 0x01f8],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    intpend: Intpend,
    _reserved6: [u8; 0xf0],
    mailbox: Mailbox,
    _reserved7: [u8; 0x78],
    eraseprotect: Eraseprotect,
    _reserved8: [u8; 0x18],
    reset: Reset,
}
impl RegisterBlock {
    #[doc = "0x100 - RXSTATUS is changed to DataPending."]
    #[inline(always)]
    pub const fn events_rxready(&self) -> &EventsRxready {
        &self.events_rxready
    }
    #[doc = "0x104 - TXSTATUS is changed to NoDataPending."]
    #[inline(always)]
    pub const fn events_txdone(&self) -> &EventsTxdone {
        &self.events_txdone
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
    #[doc = "0x400..0x488 - Unspecified"]
    #[inline(always)]
    pub const fn mailbox(&self) -> &Mailbox {
        &self.mailbox
    }
    #[doc = "0x500..0x508 - Unspecified"]
    #[inline(always)]
    pub const fn eraseprotect(&self) -> &Eraseprotect {
        &self.eraseprotect
    }
    #[doc = "0x520 - System reset request."]
    #[inline(always)]
    pub const fn reset(&self) -> &Reset {
        &self.reset
    }
}
#[doc = "EVENTS_RXREADY (rw) register accessor: RXSTATUS is changed to DataPending.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rxready`]
module"]
#[doc(alias = "EVENTS_RXREADY")]
pub type EventsRxready = crate::Reg<events_rxready::EventsRxreadySpec>;
#[doc = "RXSTATUS is changed to DataPending."]
pub mod events_rxready;
#[doc = "EVENTS_TXDONE (rw) register accessor: TXSTATUS is changed to NoDataPending.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txdone::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txdone::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_txdone`]
module"]
#[doc(alias = "EVENTS_TXDONE")]
pub type EventsTxdone = crate::Reg<events_txdone::EventsTxdoneSpec>;
#[doc = "TXSTATUS is changed to NoDataPending."]
pub mod events_txdone;
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
pub use self::mailbox::Mailbox;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod mailbox;
#[doc = "Unspecified"]
pub use self::eraseprotect::Eraseprotect;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod eraseprotect;
#[doc = "RESET (w) register accessor: System reset request.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset`]
module"]
#[doc(alias = "RESET")]
pub type Reset = crate::Reg<reset::ResetSpec>;
#[doc = "System reset request."]
pub mod reset;
