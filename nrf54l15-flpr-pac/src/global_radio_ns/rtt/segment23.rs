#[doc = "Register `SEGMENT23` reader"]
pub type R = crate::R<Segment23Spec>;
#[doc = "Register `SEGMENT23` writer"]
pub type W = crate::W<Segment23Spec>;
#[doc = "Field `DATA` reader - Data Bits 63 - 32"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data Bits 63 - 32"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Bits 63 - 32"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Bits 63 - 32"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Segment23Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "RTT segments 2 and 3\n\nYou can [`read`](crate::Reg::read) this register and get [`segment23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segment23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segment23Spec;
impl crate::RegisterSpec for Segment23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segment23::R`](R) reader structure"]
impl crate::Readable for Segment23Spec {}
#[doc = "`write(|w| ..)` method takes [`segment23::W`](W) writer structure"]
impl crate::Writable for Segment23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEGMENT23 to value 0"]
impl crate::Resettable for Segment23Spec {
    const RESET_VALUE: u32 = 0;
}
