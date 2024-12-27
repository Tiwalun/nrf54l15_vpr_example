#[doc = "Register `SYSCOUNTERH` reader"]
pub type R = crate::R<SyscounterhSpec>;
#[doc = "Field `VALUE` reader - The higher 20-bits of the SYSCOUNTER value."]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "SYSCOUNTER busy status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: SYSCOUNTER is ready for read"]
    Ready = 0,
    #[doc = "1: SYSCOUNTER is busy, so not ready for read (value returned in the VALUE field of this register is not valid)"]
    Busy = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - SYSCOUNTER busy status"]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Ready,
            true => Busy::Busy,
        }
    }
    #[doc = "SYSCOUNTER is ready for read"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Busy::Ready
    }
    #[doc = "SYSCOUNTER is busy, so not ready for read (value returned in the VALUE field of this register is not valid)"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Busy::Busy
    }
}
#[doc = "The SYSCOUNTERL overflow indication after reading it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overflow {
    #[doc = "0: SYSCOUNTERL is not overflown"]
    NoOverflow = 0,
    #[doc = "1: SYSCOUNTERL overflown"]
    Overflow = 1,
}
impl From<Overflow> for bool {
    #[inline(always)]
    fn from(variant: Overflow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERFLOW` reader - The SYSCOUNTERL overflow indication after reading it."]
pub type OverflowR = crate::BitReader<Overflow>;
impl OverflowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Overflow {
        match self.bits {
            false => Overflow::NoOverflow,
            true => Overflow::Overflow,
        }
    }
    #[doc = "SYSCOUNTERL is not overflown"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Overflow::NoOverflow
    }
    #[doc = "SYSCOUNTERL overflown"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Overflow::Overflow
    }
}
impl R {
    #[doc = "Bits 0:19 - The higher 20-bits of the SYSCOUNTER value."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 30 - SYSCOUNTER busy status"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The SYSCOUNTERL overflow indication after reading it."]
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Description cluster: The higher 20-bits of the SYSCOUNTER for index \\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`syscounterh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscounterhSpec;
impl crate::RegisterSpec for SyscounterhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscounterh::R`](R) reader structure"]
impl crate::Readable for SyscounterhSpec {}
#[doc = "`reset()` method sets SYSCOUNTERH to value 0x4000_0000"]
impl crate::Resettable for SyscounterhSpec {
    const RESET_VALUE: u32 = 0x4000_0000;
}
