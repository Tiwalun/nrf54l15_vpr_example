#[doc = "Register `SEED[%s]` writer"]
pub type W = crate::W<SeedSpec>;
#[doc = "Field `VAL` writer - Seed value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Seed value"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<SeedSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Description collection: Seed word \\[n\\]
for symmetric and asymmetric key generation. This register is only writable from KMU.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seed::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeedSpec;
impl crate::RegisterSpec for SeedSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`seed::W`](W) writer structure"]
impl crate::Writable for SeedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEED[%s]
to value 0"]
impl crate::Resettable for SeedSpec {
    const RESET_VALUE: u32 = 0;
}
