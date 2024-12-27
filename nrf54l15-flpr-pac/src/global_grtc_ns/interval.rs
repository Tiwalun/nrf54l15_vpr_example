#[doc = "Register `INTERVAL` reader"]
pub type R = crate::R<IntervalSpec>;
#[doc = "Register `INTERVAL` writer"]
pub type W = crate::W<IntervalSpec>;
#[doc = "Field `VALUE` reader - Count to add to CC\\[0\\]"]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - Count to add to CC\\[0\\]"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Count to add to CC\\[0\\]"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count to add to CC\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<IntervalSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Count to add to CC\\[0\\]
when the event EVENTS_COMPARE\\[0\\]
triggers.\n\nYou can [`read`](crate::Reg::read) this register and get [`interval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntervalSpec;
impl crate::RegisterSpec for IntervalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interval::R`](R) reader structure"]
impl crate::Readable for IntervalSpec {}
#[doc = "`write(|w| ..)` method takes [`interval::W`](W) writer structure"]
impl crate::Writable for IntervalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERVAL to value 0"]
impl crate::Resettable for IntervalSpec {
    const RESET_VALUE: u32 = 0;
}
