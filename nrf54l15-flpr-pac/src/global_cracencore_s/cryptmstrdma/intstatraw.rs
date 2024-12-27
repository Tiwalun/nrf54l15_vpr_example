#[doc = "Register `INTSTATRAW` reader"]
pub type R = crate::R<IntstatrawSpec>;
#[doc = "Register `INTSTATRAW` writer"]
pub type W = crate::W<IntstatrawSpec>;
#[doc = "Field `INTSTATRAW` reader - "]
pub type IntstatrawR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn intstatraw(&self) -> IntstatrawR {
        IntstatrawR::new(self.bits)
    }
}
impl W {}
#[doc = "Interrupt Status Raw\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatraw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstatraw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatrawSpec;
impl crate::RegisterSpec for IntstatrawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstatraw::R`](R) reader structure"]
impl crate::Readable for IntstatrawSpec {}
#[doc = "`write(|w| ..)` method takes [`intstatraw::W`](W) writer structure"]
impl crate::Writable for IntstatrawSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTSTATRAW to value 0"]
impl crate::Resettable for IntstatrawSpec {
    const RESET_VALUE: u32 = 0;
}
