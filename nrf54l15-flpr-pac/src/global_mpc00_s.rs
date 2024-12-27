#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    events_memaccerr: EventsMemaccerr,
    _reserved1: [u8; 0x01fc],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved4: [u8; 0xf4],
    memaccerr: Memaccerr,
    _reserved5: [u8; 0x08],
    globalslave: Globalslave,
    _reserved6: [u8; 0x01e8],
    region: [Region; 8],
    _reserved7: [u8; 0x0180],
    override_: [Override; 7],
}
impl RegisterBlock {
    #[doc = "0x100 - Memory Access Error event"]
    #[inline(always)]
    pub const fn events_memaccerr(&self) -> &EventsMemaccerr {
        &self.events_memaccerr
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
    #[doc = "0x400..0x408 - Memory Access Error status registers"]
    #[inline(always)]
    pub const fn memaccerr(&self) -> &Memaccerr {
        &self.memaccerr
    }
    #[doc = "0x410..0x418 - Global slave master port connection information"]
    #[inline(always)]
    pub const fn globalslave(&self) -> &Globalslave {
        &self.globalslave
    }
    #[doc = "0x600..0x680 - Memory region to slave decoding table"]
    #[inline(always)]
    pub const fn region(&self, n: usize) -> &Region {
        &self.region[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x600..0x680 - Memory region to slave decoding table"]
    #[inline(always)]
    pub fn region_iter(&self) -> impl Iterator<Item = &Region> {
        self.region.iter()
    }
    #[doc = "0x800..0x8e0 - Special privilege tables"]
    #[inline(always)]
    pub const fn override_(&self, n: usize) -> &Override {
        &self.override_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x8e0 - Special privilege tables"]
    #[inline(always)]
    pub fn override__iter(&self) -> impl Iterator<Item = &Override> {
        self.override_.iter()
    }
}
#[doc = "EVENTS_MEMACCERR (rw) register accessor: Memory Access Error event\n\nYou can [`read`](crate::Reg::read) this register and get [`events_memaccerr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_memaccerr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_memaccerr`]
module"]
#[doc(alias = "EVENTS_MEMACCERR")]
pub type EventsMemaccerr = crate::Reg<events_memaccerr::EventsMemaccerrSpec>;
#[doc = "Memory Access Error event"]
pub mod events_memaccerr;
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
#[doc = "Memory Access Error status registers"]
pub use self::memaccerr::Memaccerr;
#[doc = r"Cluster"]
#[doc = "Memory Access Error status registers"]
pub mod memaccerr;
#[doc = "Global slave master port connection information"]
pub use self::globalslave::Globalslave;
#[doc = r"Cluster"]
#[doc = "Global slave master port connection information"]
pub mod globalslave;
#[doc = "Memory region to slave decoding table"]
pub use self::region::Region;
#[doc = r"Cluster"]
#[doc = "Memory region to slave decoding table"]
pub mod region;
#[doc = "Special privilege tables"]
pub use self::override_::Override;
#[doc = r"Cluster"]
#[doc = "Special privilege tables"]
pub mod override_;
