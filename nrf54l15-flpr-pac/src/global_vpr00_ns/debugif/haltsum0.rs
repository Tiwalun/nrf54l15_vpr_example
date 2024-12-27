#[doc = "Register `HALTSUM0` reader"]
pub type R = crate::R<Haltsum0Spec>;
#[doc = "Register `HALTSUM0` writer"]
pub type W = crate::W<Haltsum0Spec>;
#[doc = "Field `HALTSUM0` reader - Halt summary 0"]
pub type Haltsum0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Halt summary 0"]
    #[inline(always)]
    pub fn haltsum0(&self) -> Haltsum0R {
        Haltsum0R::new(self.bits)
    }
}
impl W {}
#[doc = "Halt summary 0\n\nYou can [`read`](crate::Reg::read) this register and get [`haltsum0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haltsum0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Haltsum0Spec;
impl crate::RegisterSpec for Haltsum0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`haltsum0::R`](R) reader structure"]
impl crate::Readable for Haltsum0Spec {}
#[doc = "`write(|w| ..)` method takes [`haltsum0::W`](W) writer structure"]
impl crate::Writable for Haltsum0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HALTSUM0 to value 0"]
impl crate::Resettable for Haltsum0Spec {
    const RESET_VALUE: u32 = 0;
}
