#[doc = "Register `DISABLEOSC[%s]` reader"]
pub type R = crate::R<DisableoscSpec>;
#[doc = "Register `DISABLEOSC[%s]` writer"]
pub type W = crate::W<DisableoscSpec>;
#[doc = "Field `DISABLEOSC` reader - Disable oscillator rings #n*32 to #((n+1)*32)-1."]
pub type DisableoscR = crate::FieldReader<u32>;
#[doc = "Field `DISABLEOSC` writer - Disable oscillator rings #n*32 to #((n+1)*32)-1."]
pub type DisableoscW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Disable oscillator rings #n*32 to #((n+1)*32)-1."]
    #[inline(always)]
    pub fn disableosc(&self) -> DisableoscR {
        DisableoscR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Disable oscillator rings #n*32 to #((n+1)*32)-1."]
    #[inline(always)]
    #[must_use]
    pub fn disableosc(&mut self) -> DisableoscW<DisableoscSpec> {
        DisableoscW::new(self, 0)
    }
}
#[doc = "Description collection: Disable oscillator rings #n*32 to #((n+1)*32)-1.\n\nYou can [`read`](crate::Reg::read) this register and get [`disableosc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disableosc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DisableoscSpec;
impl crate::RegisterSpec for DisableoscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`disableosc::R`](R) reader structure"]
impl crate::Readable for DisableoscSpec {}
#[doc = "`write(|w| ..)` method takes [`disableosc::W`](W) writer structure"]
impl crate::Writable for DisableoscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DISABLEOSC[%s]
to value 0"]
impl crate::Resettable for DisableoscSpec {
    const RESET_VALUE: u32 = 0;
}
