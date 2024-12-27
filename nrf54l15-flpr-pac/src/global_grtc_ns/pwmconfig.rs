#[doc = "Register `PWMCONFIG` reader"]
pub type R = crate::R<PwmconfigSpec>;
#[doc = "Register `PWMCONFIG` writer"]
pub type W = crate::W<PwmconfigSpec>;
#[doc = "Field `COMPAREVALUE` reader - The PWM compare value"]
pub type ComparevalueR = crate::FieldReader;
#[doc = "Field `COMPAREVALUE` writer - The PWM compare value"]
pub type ComparevalueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The PWM compare value"]
    #[inline(always)]
    pub fn comparevalue(&self) -> ComparevalueR {
        ComparevalueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The PWM compare value"]
    #[inline(always)]
    pub fn comparevalue(&mut self) -> ComparevalueW<PwmconfigSpec> {
        ComparevalueW::new(self, 0)
    }
}
#[doc = "PWM configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmconfigSpec;
impl crate::RegisterSpec for PwmconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmconfig::R`](R) reader structure"]
impl crate::Readable for PwmconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmconfig::W`](W) writer structure"]
impl crate::Writable for PwmconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWMCONFIG to value 0"]
impl crate::Resettable for PwmconfigSpec {
    const RESET_VALUE: u32 = 0;
}
