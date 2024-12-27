#[doc = "Register `INTPEND` reader"]
pub type R = crate::R<IntpendSpec>;
#[doc = "Read pending status of interrupt for event TRIGGERED\\[16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered16 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Triggered16> for bool {
    #[inline(always)]
    fn from(variant: Triggered16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED16` reader - Read pending status of interrupt for event TRIGGERED\\[16\\]"]
pub type Triggered16R = crate::BitReader<Triggered16>;
impl Triggered16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered16 {
        match self.bits {
            false => Triggered16::NotPending,
            true => Triggered16::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Triggered16::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Triggered16::Pending
    }
}
#[doc = "Read pending status of interrupt for event TRIGGERED\\[17\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered17 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Triggered17> for bool {
    #[inline(always)]
    fn from(variant: Triggered17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED17` reader - Read pending status of interrupt for event TRIGGERED\\[17\\]"]
pub type Triggered17R = crate::BitReader<Triggered17>;
impl Triggered17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered17 {
        match self.bits {
            false => Triggered17::NotPending,
            true => Triggered17::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Triggered17::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Triggered17::Pending
    }
}
#[doc = "Read pending status of interrupt for event TRIGGERED\\[18\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered18 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Triggered18> for bool {
    #[inline(always)]
    fn from(variant: Triggered18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED18` reader - Read pending status of interrupt for event TRIGGERED\\[18\\]"]
pub type Triggered18R = crate::BitReader<Triggered18>;
impl Triggered18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered18 {
        match self.bits {
            false => Triggered18::NotPending,
            true => Triggered18::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Triggered18::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Triggered18::Pending
    }
}
#[doc = "Read pending status of interrupt for event TRIGGERED\\[19\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered19 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Triggered19> for bool {
    #[inline(always)]
    fn from(variant: Triggered19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED19` reader - Read pending status of interrupt for event TRIGGERED\\[19\\]"]
pub type Triggered19R = crate::BitReader<Triggered19>;
impl Triggered19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered19 {
        match self.bits {
            false => Triggered19::NotPending,
            true => Triggered19::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Triggered19::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Triggered19::Pending
    }
}
#[doc = "Read pending status of interrupt for event TRIGGERED\\[20\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered20 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Triggered20> for bool {
    #[inline(always)]
    fn from(variant: Triggered20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED20` reader - Read pending status of interrupt for event TRIGGERED\\[20\\]"]
pub type Triggered20R = crate::BitReader<Triggered20>;
impl Triggered20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered20 {
        match self.bits {
            false => Triggered20::NotPending,
            true => Triggered20::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Triggered20::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Triggered20::Pending
    }
}
#[doc = "Read pending status of interrupt for event TRIGGERED\\[21\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered21 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Triggered21> for bool {
    #[inline(always)]
    fn from(variant: Triggered21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED21` reader - Read pending status of interrupt for event TRIGGERED\\[21\\]"]
pub type Triggered21R = crate::BitReader<Triggered21>;
impl Triggered21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered21 {
        match self.bits {
            false => Triggered21::NotPending,
            true => Triggered21::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Triggered21::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Triggered21::Pending
    }
}
#[doc = "Read pending status of interrupt for event TRIGGERED\\[22\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered22 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Triggered22> for bool {
    #[inline(always)]
    fn from(variant: Triggered22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED22` reader - Read pending status of interrupt for event TRIGGERED\\[22\\]"]
pub type Triggered22R = crate::BitReader<Triggered22>;
impl Triggered22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered22 {
        match self.bits {
            false => Triggered22::NotPending,
            true => Triggered22::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Triggered22::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Triggered22::Pending
    }
}
impl R {
    #[doc = "Bit 16 - Read pending status of interrupt for event TRIGGERED\\[16\\]"]
    #[inline(always)]
    pub fn triggered16(&self) -> Triggered16R {
        Triggered16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Read pending status of interrupt for event TRIGGERED\\[17\\]"]
    #[inline(always)]
    pub fn triggered17(&self) -> Triggered17R {
        Triggered17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Read pending status of interrupt for event TRIGGERED\\[18\\]"]
    #[inline(always)]
    pub fn triggered18(&self) -> Triggered18R {
        Triggered18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Read pending status of interrupt for event TRIGGERED\\[19\\]"]
    #[inline(always)]
    pub fn triggered19(&self) -> Triggered19R {
        Triggered19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Read pending status of interrupt for event TRIGGERED\\[20\\]"]
    #[inline(always)]
    pub fn triggered20(&self) -> Triggered20R {
        Triggered20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Read pending status of interrupt for event TRIGGERED\\[21\\]"]
    #[inline(always)]
    pub fn triggered21(&self) -> Triggered21R {
        Triggered21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Read pending status of interrupt for event TRIGGERED\\[22\\]"]
    #[inline(always)]
    pub fn triggered22(&self) -> Triggered22R {
        Triggered22R::new(((self.bits >> 22) & 1) != 0)
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
