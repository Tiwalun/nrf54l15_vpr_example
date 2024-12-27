#[doc = "Register `SEGMENT67` reader"]
pub type R = crate::R<Segment67Spec>;
#[doc = "Register `SEGMENT67` writer"]
pub type W = crate::W<Segment67Spec>;
#[doc = "Field `DATA` reader - Data Bits 127 - 96"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data Bits 127 - 96"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Bits 127 - 96"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Bits 127 - 96"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<Segment67Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "RTT segments 6 and 7\n\nYou can [`read`](crate::Reg::read) this register and get [`segment67::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segment67::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segment67Spec;
impl crate::RegisterSpec for Segment67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segment67::R`](R) reader structure"]
impl crate::Readable for Segment67Spec {}
#[doc = "`write(|w| ..)` method takes [`segment67::W`](W) writer structure"]
impl crate::Writable for Segment67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEGMENT67 to value 0"]
impl crate::Resettable for Segment67Spec {
    const RESET_VALUE: u32 = 0;
}
