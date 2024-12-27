#[doc = "Register `SEGMENT01` reader"]
pub type R = crate::R<Segment01Spec>;
#[doc = "Register `SEGMENT01` writer"]
pub type W = crate::W<Segment01Spec>;
#[doc = "Field `DATA` reader - Data Bits 31 - 0"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data Bits 31 - 0"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Bits 31 - 0"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Bits 31 - 0"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Segment01Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "RTT segments 0 and 1\n\nYou can [`read`](crate::Reg::read) this register and get [`segment01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segment01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segment01Spec;
impl crate::RegisterSpec for Segment01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segment01::R`](R) reader structure"]
impl crate::Readable for Segment01Spec {}
#[doc = "`write(|w| ..)` method takes [`segment01::W`](W) writer structure"]
impl crate::Writable for Segment01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEGMENT01 to value 0"]
impl crate::Resettable for Segment01Spec {
    const RESET_VALUE: u32 = 0;
}
