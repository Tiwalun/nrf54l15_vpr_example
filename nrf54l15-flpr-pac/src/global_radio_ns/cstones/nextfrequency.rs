#[doc = "Register `NEXTFREQUENCY` reader"]
pub type R = crate::R<NextfrequencySpec>;
#[doc = "Register `NEXTFREQUENCY` writer"]
pub type W = crate::W<NextfrequencySpec>;
#[doc = "Field `NEXTFREQUENCY` reader - Frequency = 2400 + FREQUENCY (MHz)"]
pub type NextfrequencyR = crate::FieldReader;
#[doc = "Field `NEXTFREQUENCY` writer - Frequency = 2400 + FREQUENCY (MHz)"]
pub type NextfrequencyW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Frequency = 2400 + FREQUENCY (MHz)"]
    #[inline(always)]
    pub fn nextfrequency(&self) -> NextfrequencyR {
        NextfrequencyR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Frequency = 2400 + FREQUENCY (MHz)"]
    #[inline(always)]
    pub fn nextfrequency(&mut self) -> NextfrequencyW<NextfrequencySpec> {
        NextfrequencyW::new(self, 0)
    }
}
#[doc = "The value of FREQUENCY that will be used in the next step\n\nYou can [`read`](crate::Reg::read) this register and get [`nextfrequency::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nextfrequency::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NextfrequencySpec;
impl crate::RegisterSpec for NextfrequencySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nextfrequency::R`](R) reader structure"]
impl crate::Readable for NextfrequencySpec {}
#[doc = "`write(|w| ..)` method takes [`nextfrequency::W`](W) writer structure"]
impl crate::Writable for NextfrequencySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NEXTFREQUENCY to value 0"]
impl crate::Resettable for NextfrequencySpec {
    const RESET_VALUE: u32 = 0;
}
