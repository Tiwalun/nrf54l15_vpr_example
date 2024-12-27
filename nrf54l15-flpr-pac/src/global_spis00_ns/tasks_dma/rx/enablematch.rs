#[doc = "Register `ENABLEMATCH[%s]` writer"]
pub type W = crate::W<EnablematchSpec>;
#[doc = "Enables the MATCH\\[n\\]
event by setting the ENABLE\\[n\\]
bit in the CONFIG register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enablematch {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<Enablematch> for bool {
    #[inline(always)]
    fn from(variant: Enablematch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLEMATCH` writer - Enables the MATCH\\[n\\]
event by setting the ENABLE\\[n\\]
bit in the CONFIG register."]
pub type EnablematchW<'a, REG> = crate::BitWriter<'a, REG, Enablematch>;
impl<'a, REG> EnablematchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Enablematch::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the MATCH\\[n\\]
event by setting the ENABLE\\[n\\]
bit in the CONFIG register."]
    #[inline(always)]
    #[must_use]
    pub fn enablematch(&mut self) -> EnablematchW<EnablematchSpec> {
        EnablematchW::new(self, 0)
    }
}
#[doc = "Description collection: Enables the MATCH\\[n\\]
event by setting the ENABLE\\[n\\]
bit in the CONFIG register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enablematch::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnablematchSpec;
impl crate::RegisterSpec for EnablematchSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enablematch::W`](W) writer structure"]
impl crate::Writable for EnablematchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENABLEMATCH[%s]
to value 0"]
impl crate::Resettable for EnablematchSpec {
    const RESET_VALUE: u32 = 0;
}
