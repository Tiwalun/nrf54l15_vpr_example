#[doc = "Register `INTPEND` reader"]
pub type R = crate::R<IntpendSpec>;
#[doc = "Read pending status of interrupt for event CRYPTOMASTER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cryptomaster {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Cryptomaster> for bool {
    #[inline(always)]
    fn from(variant: Cryptomaster) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRYPTOMASTER` reader - Read pending status of interrupt for event CRYPTOMASTER"]
pub type CryptomasterR = crate::BitReader<Cryptomaster>;
impl CryptomasterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cryptomaster {
        match self.bits {
            false => Cryptomaster::NotPending,
            true => Cryptomaster::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Cryptomaster::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Cryptomaster::Pending
    }
}
#[doc = "Read pending status of interrupt for event RNG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rng {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Rng> for bool {
    #[inline(always)]
    fn from(variant: Rng) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG` reader - Read pending status of interrupt for event RNG"]
pub type RngR = crate::BitReader<Rng>;
impl RngR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rng {
        match self.bits {
            false => Rng::NotPending,
            true => Rng::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Rng::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Rng::Pending
    }
}
#[doc = "Read pending status of interrupt for event PKEIKG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pkeikg {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Pkeikg> for bool {
    #[inline(always)]
    fn from(variant: Pkeikg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PKEIKG` reader - Read pending status of interrupt for event PKEIKG"]
pub type PkeikgR = crate::BitReader<Pkeikg>;
impl PkeikgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pkeikg {
        match self.bits {
            false => Pkeikg::NotPending,
            true => Pkeikg::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Pkeikg::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Pkeikg::Pending
    }
}
impl R {
    #[doc = "Bit 0 - Read pending status of interrupt for event CRYPTOMASTER"]
    #[inline(always)]
    pub fn cryptomaster(&self) -> CryptomasterR {
        CryptomasterR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event RNG"]
    #[inline(always)]
    pub fn rng(&self) -> RngR {
        RngR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read pending status of interrupt for event PKEIKG"]
    #[inline(always)]
    pub fn pkeikg(&self) -> PkeikgR {
        PkeikgR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Pending interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`intpend::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntpendSpec;
impl crate::RegisterSpec for IntpendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intpend::R`](R) reader structure"]
impl crate::Readable for IntpendSpec {}
#[doc = "`reset()` method sets INTPEND to value 0"]
impl crate::Resettable for IntpendSpec {
    const RESET_VALUE: u32 = 0;
}
