#[doc = "Register `T5` reader"]
pub type R = crate::R<T5Spec>;
#[doc = "Register `T5` writer"]
pub type W = crate::W<T5Spec>;
#[doc = "Field `T5` reader - End point of 6th piece wise linear function"]
pub type T5R = crate::FieldReader;
#[doc = "Field `T5` writer - End point of 6th piece wise linear function"]
pub type T5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - End point of 6th piece wise linear function"]
    #[inline(always)]
    pub fn t5(&self) -> T5R {
        T5R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End point of 6th piece wise linear function"]
    #[inline(always)]
    pub fn t5(&mut self) -> T5W<T5Spec> {
        T5W::new(self, 0)
    }
}
#[doc = "End point of 6th piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`t5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T5Spec;
impl crate::RegisterSpec for T5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t5::R`](R) reader structure"]
impl crate::Readable for T5Spec {}
#[doc = "`write(|w| ..)` method takes [`t5::W`](W) writer structure"]
impl crate::Writable for T5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T5 to value 0x66"]
impl crate::Resettable for T5Spec {
    const RESET_VALUE: u32 = 0x66;
}
