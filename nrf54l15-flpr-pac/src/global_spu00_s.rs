#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    events_periphaccerr: EventsPeriphaccerr,
    _reserved1: [u8; 0x01fc],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    intpend: Intpend,
    _reserved5: [u8; 0xf4],
    periphaccerr: Periphaccerr,
    _reserved6: [u8; 0xf8],
    periph: [Periph; 64],
    feature: Feature,
}
impl RegisterBlock {
    #[doc = "0x100 - A security violation has been detected on one or several peripherals"]
    #[inline(always)]
    pub const fn events_periphaccerr(&self) -> &EventsPeriphaccerr {
        &self.events_periphaccerr
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
    #[doc = "0x404 - Unspecified"]
    #[inline(always)]
    pub const fn periphaccerr(&self) -> &Periphaccerr {
        &self.periphaccerr
    }
    #[doc = "0x500..0x600 - Unspecified"]
    #[inline(always)]
    pub const fn periph(&self, n: usize) -> &Periph {
        &self.periph[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0x600 - Unspecified"]
    #[inline(always)]
    pub fn periph_iter(&self) -> impl Iterator<Item = &Periph> {
        self.periph.iter()
    }
    #[doc = "0x600..0xdc0 - Unspecified"]
    #[inline(always)]
    pub const fn feature(&self) -> &Feature {
        &self.feature
    }
}
#[doc = "EVENTS_PERIPHACCERR (rw) register accessor: A security violation has been detected on one or several peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`events_periphaccerr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_periphaccerr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_periphaccerr`]
module"]
#[doc(alias = "EVENTS_PERIPHACCERR")]
pub type EventsPeriphaccerr = crate::Reg<events_periphaccerr::EventsPeriphaccerrSpec>;
#[doc = "A security violation has been detected on one or several peripherals"]
pub mod events_periphaccerr;
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
pub use self::periphaccerr::Periphaccerr;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod periphaccerr;
#[doc = "Unspecified"]
pub use self::periph::Periph;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod periph;
#[doc = "Unspecified"]
pub use self::feature::Feature;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod feature;
