#[doc = "Register `CLEARPATTERN` writer"]
pub type W = crate::W<ClearpatternSpec>;
#[doc = "Field `CLEARPATTERN` writer - Clear the GPIO pattern array for antenna control Behaves as a task register, but does not have PPI nor IRQ"]
pub type ClearpatternW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear the GPIO pattern array for antenna control Behaves as a task register, but does not have PPI nor IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn clearpattern(&mut self) -> ClearpatternW<ClearpatternSpec> {
        ClearpatternW::new(self, 0)
    }
}
#[doc = "Clear the GPIO pattern array for antenna control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearpattern::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearpatternSpec;
impl crate::RegisterSpec for ClearpatternSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clearpattern::W`](W) writer structure"]
impl crate::Writable for ClearpatternSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLEARPATTERN to value 0"]
impl crate::Resettable for ClearpatternSpec {
    const RESET_VALUE: u32 = 0;
}
