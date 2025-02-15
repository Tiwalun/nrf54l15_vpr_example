#[doc = "Register `PART` reader"]
pub type R = crate::R<PartSpec>;
#[doc = "Part code\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Part {
    #[doc = "346901: nRF54L15"]
    N54l15 = 346901,
    #[doc = "346896: nRF54L10"]
    N54l10 = 346896,
    #[doc = "346885: nRF54L05"]
    N54l05 = 346885,
    #[doc = "4294967295: Unspecified"]
    Unspecified = 4294967295,
}
impl From<Part> for u32 {
    #[inline(always)]
    fn from(variant: Part) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Part {
    type Ux = u32;
}
impl crate::IsEnum for Part {}
#[doc = "Field `PART` reader - Part code"]
pub type PartR = crate::FieldReader<Part>;
impl PartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Part> {
        match self.bits {
            346901 => Some(Part::N54l15),
            346896 => Some(Part::N54l10),
            346885 => Some(Part::N54l05),
            4294967295 => Some(Part::Unspecified),
            _ => None,
        }
    }
    #[doc = "nRF54L15"]
    #[inline(always)]
    pub fn is_n54l15(&self) -> bool {
        *self == Part::N54l15
    }
    #[doc = "nRF54L10"]
    #[inline(always)]
    pub fn is_n54l10(&self) -> bool {
        *self == Part::N54l10
    }
    #[doc = "nRF54L05"]
    #[inline(always)]
    pub fn is_n54l05(&self) -> bool {
        *self == Part::N54l05
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == Part::Unspecified
    }
}
impl R {
    #[doc = "Bits 0:31 - Part code"]
    #[inline(always)]
    pub fn part(&self) -> PartR {
        PartR::new(self.bits)
    }
}
#[doc = "Part code\n\nYou can [`read`](crate::Reg::read) this register and get [`part::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PartSpec;
impl crate::RegisterSpec for PartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`part::R`](R) reader structure"]
impl crate::Readable for PartSpec {}
#[doc = "`reset()` method sets PART to value 0xffff_ffff"]
impl crate::Resettable for PartSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
