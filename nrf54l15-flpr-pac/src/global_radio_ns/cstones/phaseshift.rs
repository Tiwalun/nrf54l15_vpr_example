#[doc = "Register `PHASESHIFT` reader"]
pub type R = crate::R<PhaseshiftSpec>;
#[doc = "Register `PHASESHIFT` writer"]
pub type W = crate::W<PhaseshiftSpec>;
#[doc = "Field `PHASESHIFT` reader - Phase shift used in TPM calculation"]
pub type PhaseshiftR = crate::FieldReader<u16>;
#[doc = "Field `PHASESHIFT` writer - Phase shift used in TPM calculation"]
pub type PhaseshiftW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Phase shift used in TPM calculation"]
    #[inline(always)]
    pub fn phaseshift(&self) -> PhaseshiftR {
        PhaseshiftR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Phase shift used in TPM calculation"]
    #[inline(always)]
    pub fn phaseshift(&mut self) -> PhaseshiftW<PhaseshiftSpec> {
        PhaseshiftW::new(self, 0)
    }
}
#[doc = "Parameter used in TPM, provided by software\n\nYou can [`read`](crate::Reg::read) this register and get [`phaseshift::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phaseshift::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhaseshiftSpec;
impl crate::RegisterSpec for PhaseshiftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phaseshift::R`](R) reader structure"]
impl crate::Readable for PhaseshiftSpec {}
#[doc = "`write(|w| ..)` method takes [`phaseshift::W`](W) writer structure"]
impl crate::Writable for PhaseshiftSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHASESHIFT to value 0"]
impl crate::Resettable for PhaseshiftSpec {
    const RESET_VALUE: u32 = 0;
}
