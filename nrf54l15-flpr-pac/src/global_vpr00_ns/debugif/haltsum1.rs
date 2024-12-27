#[doc = "Register `HALTSUM1` reader"]
pub type R = crate::R<Haltsum1Spec>;
#[doc = "Register `HALTSUM1` writer"]
pub type W = crate::W<Haltsum1Spec>;
#[doc = "Field `HALTSUM1` reader - Halt Summary 1"]
pub type Haltsum1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Halt Summary 1"]
    #[inline(always)]
    pub fn haltsum1(&self) -> Haltsum1R {
        Haltsum1R::new(self.bits)
    }
}
impl W {}
#[doc = "Halt Summary 1\n\nYou can [`read`](crate::Reg::read) this register and get [`haltsum1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haltsum1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Haltsum1Spec;
impl crate::RegisterSpec for Haltsum1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`haltsum1::R`](R) reader structure"]
impl crate::Readable for Haltsum1Spec {}
#[doc = "`write(|w| ..)` method takes [`haltsum1::W`](W) writer structure"]
impl crate::Writable for Haltsum1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HALTSUM1 to value 0"]
impl crate::Resettable for Haltsum1Spec {
    const RESET_VALUE: u32 = 0;
}
