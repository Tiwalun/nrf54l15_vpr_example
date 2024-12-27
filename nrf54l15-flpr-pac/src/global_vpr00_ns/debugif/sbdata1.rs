#[doc = "Register `SBDATA1` reader"]
pub type R = crate::R<Sbdata1Spec>;
#[doc = "Register `SBDATA1` writer"]
pub type W = crate::W<Sbdata1Spec>;
#[doc = "Field `DATA` reader - Accesses bits 63:32 of sbdata (if the system bus is that wide)."]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Accesses bits 63:32 of sbdata (if the system bus is that wide)."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {}
#[doc = "System Bus Data 63:32\n\nYou can [`read`](crate::Reg::read) this register and get [`sbdata1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbdata1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sbdata1Spec;
impl crate::RegisterSpec for Sbdata1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbdata1::R`](R) reader structure"]
impl crate::Readable for Sbdata1Spec {}
#[doc = "`write(|w| ..)` method takes [`sbdata1::W`](W) writer structure"]
impl crate::Writable for Sbdata1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SBDATA1 to value 0"]
impl crate::Resettable for Sbdata1Spec {
    const RESET_VALUE: u32 = 0;
}
