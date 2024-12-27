#[doc = "Register `INTPEND` reader"]
pub type R = crate::R<IntpendSpec>;
#[doc = "Read pending status of interrupt for event WOKENUP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wokenup {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Wokenup> for bool {
    #[inline(always)]
    fn from(variant: Wokenup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WOKENUP` reader - Read pending status of interrupt for event WOKENUP"]
pub type WokenupR = crate::BitReader<Wokenup>;
impl WokenupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wokenup {
        match self.bits {
            false => Wokenup::NotPending,
            true => Wokenup::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Wokenup::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Wokenup::Pending
    }
}
#[doc = "Read pending status of interrupt for event READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ready {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Ready> for bool {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - Read pending status of interrupt for event READY"]
pub type ReadyR = crate::BitReader<Ready>;
impl ReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ready {
        match self.bits {
            false => Ready::NotPending,
            true => Ready::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Ready::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Ready::Pending
    }
}
#[doc = "Read pending status of interrupt for event READYNEXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Readynext {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Readynext> for bool {
    #[inline(always)]
    fn from(variant: Readynext) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READYNEXT` reader - Read pending status of interrupt for event READYNEXT"]
pub type ReadynextR = crate::BitReader<Readynext>;
impl ReadynextR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Readynext {
        match self.bits {
            false => Readynext::NotPending,
            true => Readynext::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Readynext::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Readynext::Pending
    }
}
#[doc = "Read pending status of interrupt for event ACCESSERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accesserror {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Accesserror> for bool {
    #[inline(always)]
    fn from(variant: Accesserror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCESSERROR` reader - Read pending status of interrupt for event ACCESSERROR"]
pub type AccesserrorR = crate::BitReader<Accesserror>;
impl AccesserrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Accesserror {
        match self.bits {
            false => Accesserror::NotPending,
            true => Accesserror::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Accesserror::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Accesserror::Pending
    }
}
impl R {
    #[doc = "Bit 0 - Read pending status of interrupt for event WOKENUP"]
    #[inline(always)]
    pub fn wokenup(&self) -> WokenupR {
        WokenupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read pending status of interrupt for event READYNEXT"]
    #[inline(always)]
    pub fn readynext(&self) -> ReadynextR {
        ReadynextR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read pending status of interrupt for event ACCESSERROR"]
    #[inline(always)]
    pub fn accesserror(&self) -> AccesserrorR {
        AccesserrorR::new(((self.bits >> 3) & 1) != 0)
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
