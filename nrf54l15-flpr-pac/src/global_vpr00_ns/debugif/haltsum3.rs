#[doc = "Register `HALTSUM3` reader"]
pub type R = crate::R<Haltsum3Spec>;
#[doc = "Register `HALTSUM3` writer"]
pub type W = crate::W<Haltsum3Spec>;
#[doc = "Field `HALTSUM3` reader - Halt Summary 3"]
pub type Haltsum3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Halt Summary 3"]
    #[inline(always)]
    pub fn haltsum3(&self) -> Haltsum3R {
        Haltsum3R::new(self.bits)
    }
}
impl W {}
#[doc = "Halt Summary 3\n\nYou can [`read`](crate::Reg::read) this register and get [`haltsum3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haltsum3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Haltsum3Spec;
impl crate::RegisterSpec for Haltsum3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`haltsum3::R`](R) reader structure"]
impl crate::Readable for Haltsum3Spec {}
#[doc = "`write(|w| ..)` method takes [`haltsum3::W`](W) writer structure"]
impl crate::Writable for Haltsum3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HALTSUM3 to value 0"]
impl crate::Resettable for Haltsum3Spec {
    const RESET_VALUE: u32 = 0;
}
