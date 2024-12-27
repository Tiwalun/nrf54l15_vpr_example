#[doc = "Register `SBDATA2` reader"]
pub type R = crate::R<Sbdata2Spec>;
#[doc = "Register `SBDATA2` writer"]
pub type W = crate::W<Sbdata2Spec>;
#[doc = "Field `DATA` reader - Accesses bits 95:64 of sbdata (if the system bus is that wide)."]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Accesses bits 95:64 of sbdata (if the system bus is that wide)."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {}
#[doc = "System Bus Data 95:64\n\nYou can [`read`](crate::Reg::read) this register and get [`sbdata2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbdata2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sbdata2Spec;
impl crate::RegisterSpec for Sbdata2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbdata2::R`](R) reader structure"]
impl crate::Readable for Sbdata2Spec {}
#[doc = "`write(|w| ..)` method takes [`sbdata2::W`](W) writer structure"]
impl crate::Writable for Sbdata2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SBDATA2 to value 0"]
impl crate::Resettable for Sbdata2Spec {
    const RESET_VALUE: u32 = 0;
}
