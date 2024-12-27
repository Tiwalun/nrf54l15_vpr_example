#[doc = "Register `T1` reader"]
pub type R = crate::R<T1Spec>;
#[doc = "Register `T1` writer"]
pub type W = crate::W<T1Spec>;
#[doc = "Field `T1` reader - End point of 2nd piece wise linear function"]
pub type T1R = crate::FieldReader;
#[doc = "Field `T1` writer - End point of 2nd piece wise linear function"]
pub type T1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - End point of 2nd piece wise linear function"]
    #[inline(always)]
    pub fn t1(&self) -> T1R {
        T1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End point of 2nd piece wise linear function"]
    #[inline(always)]
    #[must_use]
    pub fn t1(&mut self) -> T1W<T1Spec> {
        T1W::new(self, 0)
    }
}
#[doc = "End point of 2nd piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`t1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T1Spec;
impl crate::RegisterSpec for T1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t1::R`](R) reader structure"]
impl crate::Readable for T1Spec {}
#[doc = "`write(|w| ..)` method takes [`t1::W`](W) writer structure"]
impl crate::Writable for T1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T1 to value 0x02"]
impl crate::Resettable for T1Spec {
    const RESET_VALUE: u32 = 0x02;
}
