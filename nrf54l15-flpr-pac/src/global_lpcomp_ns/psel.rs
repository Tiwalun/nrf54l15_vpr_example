#[doc = "Register `PSEL` reader"]
pub type R = crate::R<PselSpec>;
#[doc = "Register `PSEL` writer"]
pub type W = crate::W<PselSpec>;
#[doc = "Field `PIN` reader - Analog pin select"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - Analog pin select"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PORT` reader - GPIO Port selection"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - GPIO Port selection"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - Analog pin select"]
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
    #[doc = "Bits 0:4 - Analog pin select"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<PselSpec> {
        PinW::new(self, 0)
    }
    #[doc = "Bits 8:11 - GPIO Port selection"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<PselSpec> {
        PortW::new(self, 8)
    }
}
#[doc = "Input pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`psel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselSpec;
impl crate::RegisterSpec for PselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psel::R`](R) reader structure"]
impl crate::Readable for PselSpec {}
#[doc = "`write(|w| ..)` method takes [`psel::W`](W) writer structure"]
impl crate::Writable for PselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSEL to value 0"]
impl crate::Resettable for PselSpec {
    const RESET_VALUE: u32 = 0;
}
