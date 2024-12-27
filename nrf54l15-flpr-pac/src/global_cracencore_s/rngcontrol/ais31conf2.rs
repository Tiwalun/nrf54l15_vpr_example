#[doc = "Register `AIS31CONF2` reader"]
pub type R = crate::R<Ais31conf2Spec>;
#[doc = "Register `AIS31CONF2` writer"]
pub type W = crate::W<Ais31conf2Spec>;
#[doc = "Field `HMIN` reader - Minimum allowed history value."]
pub type HminR = crate::FieldReader<u16>;
#[doc = "Field `HMIN` writer - Minimum allowed history value."]
pub type HminW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `HMAX` reader - Maximum allowed history value."]
pub type HmaxR = crate::FieldReader<u16>;
#[doc = "Field `HMAX` writer - Maximum allowed history value."]
pub type HmaxW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Minimum allowed history value."]
    #[inline(always)]
    pub fn hmin(&self) -> HminR {
        HminR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Maximum allowed history value."]
    #[inline(always)]
    pub fn hmax(&self) -> HmaxR {
        HmaxR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Minimum allowed history value."]
    #[inline(always)]
    #[must_use]
    pub fn hmin(&mut self) -> HminW<Ais31conf2Spec> {
        HminW::new(self, 0)
    }
    #[doc = "Bits 16:30 - Maximum allowed history value."]
    #[inline(always)]
    #[must_use]
    pub fn hmax(&mut self) -> HmaxW<Ais31conf2Spec> {
        HmaxW::new(self, 16)
    }
}
#[doc = "AIS31 configuration register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`ais31conf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ais31conf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ais31conf2Spec;
impl crate::RegisterSpec for Ais31conf2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ais31conf2::R`](R) reader structure"]
impl crate::Readable for Ais31conf2Spec {}
#[doc = "`write(|w| ..)` method takes [`ais31conf2::W`](W) writer structure"]
impl crate::Writable for Ais31conf2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIS31CONF2 to value 0x0440_0340"]
impl crate::Resettable for Ais31conf2Spec {
    const RESET_VALUE: u32 = 0x0440_0340;
}
