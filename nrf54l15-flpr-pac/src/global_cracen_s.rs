#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    events_cryptomaster: EventsCryptomaster,
    events_rng: EventsRng,
    events_pkeikg: EventsPkeikg,
    _reserved3: [u8; 0x01f4],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    intpend: Intpend,
    _reserved7: [u8; 0xf0],
    enable: Enable,
    seedvalid: Seedvalid,
    _reserved9: [u8; 0x08],
    seed: [Seed; 12],
    seedlock: Seedlock,
    protectedramlock: Protectedramlock,
}
impl RegisterBlock {
    #[doc = "0x100 - Event indicating that interrupt triggered at Cryptomaster"]
    #[inline(always)]
    pub const fn events_cryptomaster(&self) -> &EventsCryptomaster {
        &self.events_cryptomaster
    }
    #[doc = "0x104 - Event indicating that interrupt triggered at RNG"]
    #[inline(always)]
    pub const fn events_rng(&self) -> &EventsRng {
        &self.events_rng
    }
    #[doc = "0x108 - Event indicating that interrupt triggered at PKE or IKG"]
    #[inline(always)]
    pub const fn events_pkeikg(&self) -> &EventsPkeikg {
        &self.events_pkeikg
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
    #[doc = "0x400 - Enable CRACEN peripheral modules."]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x404 - Marks the SEED register as valid"]
    #[inline(always)]
    pub const fn seedvalid(&self) -> &Seedvalid {
        &self.seedvalid
    }
    #[doc = "0x410..0x440 - Description collection: Seed word \\[n\\]
for symmetric and asymmetric key generation. This register is only writable from KMU."]
    #[inline(always)]
    pub const fn seed(&self, n: usize) -> &Seed {
        &self.seed[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x410..0x440 - Description collection: Seed word \\[n\\]
for symmetric and asymmetric key generation. This register is only writable from KMU."]
    #[inline(always)]
    pub fn seed_iter(&self) -> impl Iterator<Item = &Seed> {
        self.seed.iter()
    }
    #[doc = "0x440 - Lock the access to the SEED register."]
    #[inline(always)]
    pub const fn seedlock(&self) -> &Seedlock {
        &self.seedlock
    }
    #[doc = "0x444 - Lock the access to the protected RAM."]
    #[inline(always)]
    pub const fn protectedramlock(&self) -> &Protectedramlock {
        &self.protectedramlock
    }
}
#[doc = "EVENTS_CRYPTOMASTER (rw) register accessor: Event indicating that interrupt triggered at Cryptomaster\n\nYou can [`read`](crate::Reg::read) this register and get [`events_cryptomaster::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_cryptomaster::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_cryptomaster`]
module"]
#[doc(alias = "EVENTS_CRYPTOMASTER")]
pub type EventsCryptomaster = crate::Reg<events_cryptomaster::EventsCryptomasterSpec>;
#[doc = "Event indicating that interrupt triggered at Cryptomaster"]
pub mod events_cryptomaster;
#[doc = "EVENTS_RNG (rw) register accessor: Event indicating that interrupt triggered at RNG\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rng`]
module"]
#[doc(alias = "EVENTS_RNG")]
pub type EventsRng = crate::Reg<events_rng::EventsRngSpec>;
#[doc = "Event indicating that interrupt triggered at RNG"]
pub mod events_rng;
#[doc = "EVENTS_PKEIKG (rw) register accessor: Event indicating that interrupt triggered at PKE or IKG\n\nYou can [`read`](crate::Reg::read) this register and get [`events_pkeikg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_pkeikg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_pkeikg`]
module"]
#[doc(alias = "EVENTS_PKEIKG")]
pub type EventsPkeikg = crate::Reg<events_pkeikg::EventsPkeikgSpec>;
#[doc = "Event indicating that interrupt triggered at PKE or IKG"]
pub mod events_pkeikg;
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
#[doc = "ENABLE (rw) register accessor: Enable CRACEN peripheral modules.\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable CRACEN peripheral modules."]
pub mod enable;
#[doc = "SEEDVALID (rw) register accessor: Marks the SEED register as valid\n\nYou can [`read`](crate::Reg::read) this register and get [`seedvalid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seedvalid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seedvalid`]
module"]
#[doc(alias = "SEEDVALID")]
pub type Seedvalid = crate::Reg<seedvalid::SeedvalidSpec>;
#[doc = "Marks the SEED register as valid"]
pub mod seedvalid;
#[doc = "SEED (w) register accessor: Description collection: Seed word \\[n\\]
for symmetric and asymmetric key generation. This register is only writable from KMU.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seed`]
module"]
#[doc(alias = "SEED")]
pub type Seed = crate::Reg<seed::SeedSpec>;
#[doc = "Description collection: Seed word \\[n\\]
for symmetric and asymmetric key generation. This register is only writable from KMU."]
pub mod seed;
#[doc = "SEEDLOCK (rw) register accessor: Lock the access to the SEED register.\n\nYou can [`read`](crate::Reg::read) this register and get [`seedlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seedlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seedlock`]
module"]
#[doc(alias = "SEEDLOCK")]
pub type Seedlock = crate::Reg<seedlock::SeedlockSpec>;
#[doc = "Lock the access to the SEED register."]
pub mod seedlock;
#[doc = "PROTECTEDRAMLOCK (rw) register accessor: Lock the access to the protected RAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`protectedramlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`protectedramlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@protectedramlock`]
module"]
#[doc(alias = "PROTECTEDRAMLOCK")]
pub type Protectedramlock = crate::Reg<protectedramlock::ProtectedramlockSpec>;
#[doc = "Lock the access to the protected RAM."]
pub mod protectedramlock;
