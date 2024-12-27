#[doc = "Register `PRESCALER` reader"]
pub type R = crate::R<PrescalerSpec>;
#[doc = "Register `PRESCALER` writer"]
pub type W = crate::W<PrescalerSpec>;
#[doc = "Field `DIVISOR` reader - Core clock to SCK divisor"]
pub type DivisorR = crate::FieldReader;
#[doc = "Field `DIVISOR` writer - Core clock to SCK divisor"]
pub type DivisorW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Core clock to SCK divisor"]
    #[inline(always)]
    pub fn divisor(&self) -> DivisorR {
        DivisorR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Core clock to SCK divisor"]
    #[inline(always)]
    #[must_use]
    pub fn divisor(&mut self) -> DivisorW<PrescalerSpec> {
        DivisorW::new(self, 0)
    }
}
#[doc = "The prescaler is used to set the SPI frequency.\n\nYou can [`read`](crate::Reg::read) this register and get [`prescaler::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prescaler::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrescalerSpec;
impl crate::RegisterSpec for PrescalerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prescaler::R`](R) reader structure"]
impl crate::Readable for PrescalerSpec {}
#[doc = "`write(|w| ..)` method takes [`prescaler::W`](W) writer structure"]
impl crate::Writable for PrescalerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRESCALER to value 0x40"]
impl crate::Resettable for PrescalerSpec {
    const RESET_VALUE: u32 = 0x40;
}
