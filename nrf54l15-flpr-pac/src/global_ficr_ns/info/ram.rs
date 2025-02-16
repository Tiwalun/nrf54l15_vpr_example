#[doc = "Register `RAM` reader"]
pub type R = crate::R<RamSpec>;
#[doc = "RAM size (KB)\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Ram {
    #[doc = "256: 256 kByte RAM"]
    K256 = 256,
    #[doc = "192: 192 kByte RAM"]
    K192 = 192,
    #[doc = "96: 96 kByte RAM"]
    K96 = 96,
    #[doc = "4294967295: Unspecified"]
    Unspecified = 4294967295,
}
impl From<Ram> for u32 {
    #[inline(always)]
    fn from(variant: Ram) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ram {
    type Ux = u32;
}
impl crate::IsEnum for Ram {}
#[doc = "Field `RAM` reader - RAM size (KB)"]
pub type RamR = crate::FieldReader<Ram>;
impl RamR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ram> {
        match self.bits {
            256 => Some(Ram::K256),
            192 => Some(Ram::K192),
            96 => Some(Ram::K96),
            4294967295 => Some(Ram::Unspecified),
            _ => None,
        }
    }
    #[doc = "256 kByte RAM"]
    #[inline(always)]
    pub fn is_k256(&self) -> bool {
        *self == Ram::K256
    }
    #[doc = "192 kByte RAM"]
    #[inline(always)]
    pub fn is_k192(&self) -> bool {
        *self == Ram::K192
    }
    #[doc = "96 kByte RAM"]
    #[inline(always)]
    pub fn is_k96(&self) -> bool {
        *self == Ram::K96
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == Ram::Unspecified
    }
}
impl R {
    #[doc = "Bits 0:31 - RAM size (KB)"]
    #[inline(always)]
    pub fn ram(&self) -> RamR {
        RamR::new(self.bits)
    }
}
#[doc = "RAM size (KB)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamSpec;
impl crate::RegisterSpec for RamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram::R`](R) reader structure"]
impl crate::Readable for RamSpec {}
#[doc = "`reset()` method sets RAM to value 0xffff_ffff"]
impl crate::Resettable for RamSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
