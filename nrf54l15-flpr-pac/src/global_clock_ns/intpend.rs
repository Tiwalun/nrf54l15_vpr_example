#[doc = "Register `INTPEND` reader"]
pub type R = crate::R<IntpendSpec>;
#[doc = "Read pending status of interrupt for event XOSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xostarted {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Xostarted> for bool {
    #[inline(always)]
    fn from(variant: Xostarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XOSTARTED` reader - Read pending status of interrupt for event XOSTARTED"]
pub type XostartedR = crate::BitReader<Xostarted>;
impl XostartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xostarted {
        match self.bits {
            false => Xostarted::NotPending,
            true => Xostarted::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Xostarted::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Xostarted::Pending
    }
}
#[doc = "Read pending status of interrupt for event PLLSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllstarted {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Pllstarted> for bool {
    #[inline(always)]
    fn from(variant: Pllstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTARTED` reader - Read pending status of interrupt for event PLLSTARTED"]
pub type PllstartedR = crate::BitReader<Pllstarted>;
impl PllstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllstarted {
        match self.bits {
            false => Pllstarted::NotPending,
            true => Pllstarted::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Pllstarted::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Pllstarted::Pending
    }
}
#[doc = "Read pending status of interrupt for event LFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfclkstarted {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Lfclkstarted> for bool {
    #[inline(always)]
    fn from(variant: Lfclkstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFCLKSTARTED` reader - Read pending status of interrupt for event LFCLKSTARTED"]
pub type LfclkstartedR = crate::BitReader<Lfclkstarted>;
impl LfclkstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfclkstarted {
        match self.bits {
            false => Lfclkstarted::NotPending,
            true => Lfclkstarted::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Lfclkstarted::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Lfclkstarted::Pending
    }
}
#[doc = "Read pending status of interrupt for event DONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Read pending status of interrupt for event DONE"]
pub type DoneR = crate::BitReader<Done>;
impl DoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Done {
        match self.bits {
            false => Done::NotPending,
            true => Done::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Done::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Done::Pending
    }
}
#[doc = "Read pending status of interrupt for event XOTUNED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xotuned {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Xotuned> for bool {
    #[inline(always)]
    fn from(variant: Xotuned) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XOTUNED` reader - Read pending status of interrupt for event XOTUNED"]
pub type XotunedR = crate::BitReader<Xotuned>;
impl XotunedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xotuned {
        match self.bits {
            false => Xotuned::NotPending,
            true => Xotuned::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Xotuned::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Xotuned::Pending
    }
}
#[doc = "Read pending status of interrupt for event XOTUNEERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xotuneerror {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Xotuneerror> for bool {
    #[inline(always)]
    fn from(variant: Xotuneerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XOTUNEERROR` reader - Read pending status of interrupt for event XOTUNEERROR"]
pub type XotuneerrorR = crate::BitReader<Xotuneerror>;
impl XotuneerrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xotuneerror {
        match self.bits {
            false => Xotuneerror::NotPending,
            true => Xotuneerror::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Xotuneerror::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Xotuneerror::Pending
    }
}
#[doc = "Read pending status of interrupt for event XOTUNEFAILED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xotunefailed {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Xotunefailed> for bool {
    #[inline(always)]
    fn from(variant: Xotunefailed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XOTUNEFAILED` reader - Read pending status of interrupt for event XOTUNEFAILED"]
pub type XotunefailedR = crate::BitReader<Xotunefailed>;
impl XotunefailedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xotunefailed {
        match self.bits {
            false => Xotunefailed::NotPending,
            true => Xotunefailed::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Xotunefailed::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Xotunefailed::Pending
    }
}
impl R {
    #[doc = "Bit 0 - Read pending status of interrupt for event XOSTARTED"]
    #[inline(always)]
    pub fn xostarted(&self) -> XostartedR {
        XostartedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event PLLSTARTED"]
    #[inline(always)]
    pub fn pllstarted(&self) -> PllstartedR {
        PllstartedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read pending status of interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&self) -> LfclkstartedR {
        LfclkstartedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read pending status of interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read pending status of interrupt for event XOTUNED"]
    #[inline(always)]
    pub fn xotuned(&self) -> XotunedR {
        XotunedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read pending status of interrupt for event XOTUNEERROR"]
    #[inline(always)]
    pub fn xotuneerror(&self) -> XotuneerrorR {
        XotuneerrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read pending status of interrupt for event XOTUNEFAILED"]
    #[inline(always)]
    pub fn xotunefailed(&self) -> XotunefailedR {
        XotunefailedR::new(((self.bits >> 6) & 1) != 0)
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
