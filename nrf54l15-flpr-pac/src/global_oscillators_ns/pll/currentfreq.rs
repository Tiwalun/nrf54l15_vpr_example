#[doc = "Register `CURRENTFREQ` reader"]
pub type R = crate::R<CurrentfreqSpec>;
#[doc = "Active CPU speed\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Currentfreq {
    #[doc = "1: 128 MHz"]
    Ck128m = 1,
    #[doc = "3: 64 MHz"]
    Ck64m = 3,
}
impl From<Currentfreq> for u8 {
    #[inline(always)]
    fn from(variant: Currentfreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Currentfreq {
    type Ux = u8;
}
impl crate::IsEnum for Currentfreq {}
#[doc = "Field `CURRENTFREQ` reader - Active CPU speed"]
pub type CurrentfreqR = crate::FieldReader<Currentfreq>;
impl CurrentfreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Currentfreq> {
        match self.bits {
            1 => Some(Currentfreq::Ck128m),
            3 => Some(Currentfreq::Ck64m),
            _ => None,
        }
    }
    #[doc = "128 MHz"]
    #[inline(always)]
    pub fn is_ck128m(&self) -> bool {
        *self == Currentfreq::Ck128m
    }
    #[doc = "64 MHz"]
    #[inline(always)]
    pub fn is_ck64m(&self) -> bool {
        *self == Currentfreq::Ck64m
    }
}
impl R {
    #[doc = "Bits 0:1 - Active CPU speed"]
    #[inline(always)]
    pub fn currentfreq(&self) -> CurrentfreqR {
        CurrentfreqR::new((self.bits & 3) as u8)
    }
}
#[doc = "Current speed of MCU power domain, including CPU\n\nYou can [`read`](crate::Reg::read) this register and get [`currentfreq::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrentfreqSpec;
impl crate::RegisterSpec for CurrentfreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`currentfreq::R`](R) reader structure"]
impl crate::Readable for CurrentfreqSpec {}
#[doc = "`reset()` method sets CURRENTFREQ to value 0x03"]
impl crate::Resettable for CurrentfreqSpec {
    const RESET_VALUE: u32 = 0x03;
}
