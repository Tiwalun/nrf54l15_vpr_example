#[doc = "Register `SBDATA0` reader"]
pub type R = crate::R<Sbdata0Spec>;
#[doc = "Register `SBDATA0` writer"]
pub type W = crate::W<Sbdata0Spec>;
#[doc = "Field `DATA` reader - Accesses bits 31:0 of sbdata"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Accesses bits 31:0 of sbdata"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {}
#[doc = "System Bus Data 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`sbdata0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbdata0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sbdata0Spec;
impl crate::RegisterSpec for Sbdata0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbdata0::R`](R) reader structure"]
impl crate::Readable for Sbdata0Spec {}
#[doc = "`write(|w| ..)` method takes [`sbdata0::W`](W) writer structure"]
impl crate::Writable for Sbdata0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SBDATA0 to value 0"]
impl crate::Resettable for Sbdata0Spec {
    const RESET_VALUE: u32 = 0;
}
