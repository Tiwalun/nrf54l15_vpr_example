#[doc = "Register `INTPEND` reader"]
pub type R = crate::R<IntpendSpec>;
#[doc = "Read pending status of interrupt for event TAMPER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamper {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Tamper> for bool {
    #[inline(always)]
    fn from(variant: Tamper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPER` reader - Read pending status of interrupt for event TAMPER"]
pub type TamperR = crate::BitReader<Tamper>;
impl TamperR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamper {
        match self.bits {
            false => Tamper::NotPending,
            true => Tamper::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Tamper::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Tamper::Pending
    }
}
#[doc = "Read pending status of interrupt for event WRITEERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Writeerror {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Writeerror> for bool {
    #[inline(always)]
    fn from(variant: Writeerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITEERROR` reader - Read pending status of interrupt for event WRITEERROR"]
pub type WriteerrorR = crate::BitReader<Writeerror>;
impl WriteerrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Writeerror {
        match self.bits {
            false => Writeerror::NotPending,
            true => Writeerror::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Writeerror::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Writeerror::Pending
    }
}
impl R {
    #[doc = "Bit 0 - Read pending status of interrupt for event TAMPER"]
    #[inline(always)]
    pub fn tamper(&self) -> TamperR {
        TamperR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event WRITEERROR"]
    #[inline(always)]
    pub fn writeerror(&self) -> WriteerrorR {
        WriteerrorR::new(((self.bits >> 1) & 1) != 0)
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
