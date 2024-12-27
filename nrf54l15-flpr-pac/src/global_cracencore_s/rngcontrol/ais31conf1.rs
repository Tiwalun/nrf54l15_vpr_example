#[doc = "Register `AIS31CONF1` reader"]
pub type R = crate::R<Ais31conf1Spec>;
#[doc = "Register `AIS31CONF1` writer"]
pub type W = crate::W<Ais31conf1Spec>;
#[doc = "Field `ONLINEREPTHRESHOLD` reader - Online repeat threshold."]
pub type OnlinerepthresholdR = crate::FieldReader<u16>;
#[doc = "Field `ONLINEREPTHRESHOLD` writer - Online repeat threshold."]
pub type OnlinerepthresholdW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `HEXPECTEDVALUE` reader - Expected history value."]
pub type HexpectedvalueR = crate::FieldReader<u16>;
#[doc = "Field `HEXPECTEDVALUE` writer - Expected history value."]
pub type HexpectedvalueW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Online repeat threshold."]
    #[inline(always)]
    pub fn onlinerepthreshold(&self) -> OnlinerepthresholdR {
        OnlinerepthresholdR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Expected history value."]
    #[inline(always)]
    pub fn hexpectedvalue(&self) -> HexpectedvalueR {
        HexpectedvalueR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Online repeat threshold."]
    #[inline(always)]
    #[must_use]
    pub fn onlinerepthreshold(&mut self) -> OnlinerepthresholdW<Ais31conf1Spec> {
        OnlinerepthresholdW::new(self, 0)
    }
    #[doc = "Bits 16:30 - Expected history value."]
    #[inline(always)]
    #[must_use]
    pub fn hexpectedvalue(&mut self) -> HexpectedvalueW<Ais31conf1Spec> {
        HexpectedvalueW::new(self, 16)
    }
}
#[doc = "AIS31 configuration register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`ais31conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ais31conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ais31conf1Spec;
impl crate::RegisterSpec for Ais31conf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ais31conf1::R`](R) reader structure"]
impl crate::Readable for Ais31conf1Spec {}
#[doc = "`write(|w| ..)` method takes [`ais31conf1::W`](W) writer structure"]
impl crate::Writable for Ais31conf1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIS31CONF1 to value 0x03c0_0680"]
impl crate::Resettable for Ais31conf1Spec {
    const RESET_VALUE: u32 = 0x03c0_0680;
}
