#[doc = "Register `AIS31CONF0` reader"]
pub type R = crate::R<Ais31conf0Spec>;
#[doc = "Register `AIS31CONF0` writer"]
pub type W = crate::W<Ais31conf0Spec>;
#[doc = "Field `STARTUPTHRESHOLD` reader - Start-up test threshold."]
pub type StartupthresholdR = crate::FieldReader<u16>;
#[doc = "Field `STARTUPTHRESHOLD` writer - Start-up test threshold."]
pub type StartupthresholdW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `ONLINETHRESHOLD` reader - Online threshold."]
pub type OnlinethresholdR = crate::FieldReader<u16>;
#[doc = "Field `ONLINETHRESHOLD` writer - Online threshold."]
pub type OnlinethresholdW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Start-up test threshold."]
    #[inline(always)]
    pub fn startupthreshold(&self) -> StartupthresholdR {
        StartupthresholdR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Online threshold."]
    #[inline(always)]
    pub fn onlinethreshold(&self) -> OnlinethresholdR {
        OnlinethresholdR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Start-up test threshold."]
    #[inline(always)]
    pub fn startupthreshold(&mut self) -> StartupthresholdW<Ais31conf0Spec> {
        StartupthresholdW::new(self, 0)
    }
    #[doc = "Bits 16:30 - Online threshold."]
    #[inline(always)]
    pub fn onlinethreshold(&mut self) -> OnlinethresholdW<Ais31conf0Spec> {
        OnlinethresholdW::new(self, 16)
    }
}
#[doc = "AIS31 configuration register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ais31conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ais31conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ais31conf0Spec;
impl crate::RegisterSpec for Ais31conf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ais31conf0::R`](R) reader structure"]
impl crate::Readable for Ais31conf0Spec {}
#[doc = "`write(|w| ..)` method takes [`ais31conf0::W`](W) writer structure"]
impl crate::Writable for Ais31conf0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIS31CONF0 to value 0x4340_1040"]
impl crate::Resettable for Ais31conf0Spec {
    const RESET_VALUE: u32 = 0x4340_1040;
}
