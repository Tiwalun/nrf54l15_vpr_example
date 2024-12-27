#[doc = "Register `EXTREFSEL` reader"]
pub type R = crate::R<ExtrefselSpec>;
#[doc = "Register `EXTREFSEL` writer"]
pub type W = crate::W<ExtrefselSpec>;
#[doc = "Field `PIN` reader - External analog reference pin select"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - External analog reference pin select"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PORT` reader - GPIO Port selection"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - GPIO Port selection"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - External analog reference pin select"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - GPIO Port selection"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - External analog reference pin select"]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self) -> PinW<ExtrefselSpec> {
        PinW::new(self, 0)
    }
    #[doc = "Bits 8:11 - GPIO Port selection"]
    #[inline(always)]
    #[must_use]
    pub fn port(&mut self) -> PortW<ExtrefselSpec> {
        PortW::new(self, 8)
    }
}
#[doc = "External reference select\n\nYou can [`read`](crate::Reg::read) this register and get [`extrefsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extrefsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtrefselSpec;
impl crate::RegisterSpec for ExtrefselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extrefsel::R`](R) reader structure"]
impl crate::Readable for ExtrefselSpec {}
#[doc = "`write(|w| ..)` method takes [`extrefsel::W`](W) writer structure"]
impl crate::Writable for ExtrefselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTREFSEL to value 0"]
impl crate::Resettable for ExtrefselSpec {
    const RESET_VALUE: u32 = 0;
}
