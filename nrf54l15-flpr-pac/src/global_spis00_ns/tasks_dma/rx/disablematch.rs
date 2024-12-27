#[doc = "Register `DISABLEMATCH[%s]` writer"]
pub type W = crate::W<DisablematchSpec>;
#[doc = "Disables the MATCH\\[n\\]
event by clearing the ENABLE\\[n\\]
bit in the CONFIG register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disablematch {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<Disablematch> for bool {
    #[inline(always)]
    fn from(variant: Disablematch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLEMATCH` writer - Disables the MATCH\\[n\\]
event by clearing the ENABLE\\[n\\]
bit in the CONFIG register."]
pub type DisablematchW<'a, REG> = crate::BitWriter<'a, REG, Disablematch>;
impl<'a, REG> DisablematchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Disablematch::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Disables the MATCH\\[n\\]
event by clearing the ENABLE\\[n\\]
bit in the CONFIG register."]
    #[inline(always)]
    pub fn disablematch(&mut self) -> DisablematchW<DisablematchSpec> {
        DisablematchW::new(self, 0)
    }
}
#[doc = "Description collection: Disables the MATCH\\[n\\]
event by clearing the ENABLE\\[n\\]
bit in the CONFIG register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disablematch::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DisablematchSpec;
impl crate::RegisterSpec for DisablematchSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`disablematch::W`](W) writer structure"]
impl crate::Writable for DisablematchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DISABLEMATCH[%s]
to value 0"]
impl crate::Resettable for DisablematchSpec {
    const RESET_VALUE: u32 = 0;
}
