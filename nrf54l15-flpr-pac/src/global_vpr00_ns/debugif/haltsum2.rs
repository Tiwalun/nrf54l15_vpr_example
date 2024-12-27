#[doc = "Register `HALTSUM2` reader"]
pub type R = crate::R<Haltsum2Spec>;
#[doc = "Register `HALTSUM2` writer"]
pub type W = crate::W<Haltsum2Spec>;
#[doc = "Field `HALTSUM2` reader - Halt Summary 2"]
pub type Haltsum2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Halt Summary 2"]
    #[inline(always)]
    pub fn haltsum2(&self) -> Haltsum2R {
        Haltsum2R::new(self.bits)
    }
}
impl W {}
#[doc = "Halt Summary 2\n\nYou can [`read`](crate::Reg::read) this register and get [`haltsum2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haltsum2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Haltsum2Spec;
impl crate::RegisterSpec for Haltsum2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`haltsum2::R`](R) reader structure"]
impl crate::Readable for Haltsum2Spec {}
#[doc = "`write(|w| ..)` method takes [`haltsum2::W`](W) writer structure"]
impl crate::Writable for Haltsum2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HALTSUM2 to value 0"]
impl crate::Resettable for Haltsum2Spec {
    const RESET_VALUE: u32 = 0;
}
