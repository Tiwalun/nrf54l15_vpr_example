#[doc = "Register `RRAM` reader"]
pub type R = crate::R<RramSpec>;
#[doc = "RRAM variant\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Rram {
    #[doc = "1524: 1524 KByte RRAM"]
    K1524 = 1524,
    #[doc = "4294967295: Unspecified"]
    Unspecified = 4294967295,
}
impl From<Rram> for u32 {
    #[inline(always)]
    fn from(variant: Rram) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rram {
    type Ux = u32;
}
impl crate::IsEnum for Rram {}
#[doc = "Field `RRAM` reader - RRAM variant"]
pub type RramR = crate::FieldReader<Rram>;
impl RramR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rram> {
        match self.bits {
            1524 => Some(Rram::K1524),
            4294967295 => Some(Rram::Unspecified),
            _ => None,
        }
    }
    #[doc = "1524 KByte RRAM"]
    #[inline(always)]
    pub fn is_k1524(&self) -> bool {
        *self == Rram::K1524
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == Rram::Unspecified
    }
}
impl R {
    #[doc = "Bits 0:31 - RRAM variant"]
    #[inline(always)]
    pub fn rram(&self) -> RramR {
        RramR::new(self.bits)
    }
}
#[doc = "RRAM variant\n\nYou can [`read`](crate::Reg::read) this register and get [`rram::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RramSpec;
impl crate::RegisterSpec for RramSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rram::R`](R) reader structure"]
impl crate::Readable for RramSpec {}
#[doc = "`reset()` method sets RRAM to value 0xffff_ffff"]
impl crate::Resettable for RramSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
