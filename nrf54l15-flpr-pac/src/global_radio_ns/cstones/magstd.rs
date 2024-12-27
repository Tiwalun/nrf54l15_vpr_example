#[doc = "Register `MAGSTD` reader"]
pub type R = crate::R<MagstdSpec>;
#[doc = "Field `MAGSTD` reader - Magnitude standard deviation approximation"]
pub type MagstdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Magnitude standard deviation approximation"]
    #[inline(always)]
    pub fn magstd(&self) -> MagstdR {
        MagstdR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Magnitude standard deviation approximation\n\nYou can [`read`](crate::Reg::read) this register and get [`magstd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MagstdSpec;
impl crate::RegisterSpec for MagstdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`magstd::R`](R) reader structure"]
impl crate::Readable for MagstdSpec {}
#[doc = "`reset()` method sets MAGSTD to value 0"]
impl crate::Resettable for MagstdSpec {
    const RESET_VALUE: u32 = 0;
}
