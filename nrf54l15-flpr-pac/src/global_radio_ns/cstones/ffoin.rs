#[doc = "Register `FFOIN` reader"]
pub type R = crate::R<FfoinSpec>;
#[doc = "Register `FFOIN` writer"]
pub type W = crate::W<FfoinSpec>;
#[doc = "Field `FFFIN` reader - Units 62.5 ppb. Max range +/-100 ppm plus margin."]
pub type FffinR = crate::FieldReader<u16>;
#[doc = "Field `FFFIN` writer - Units 62.5 ppb. Max range +/-100 ppm plus margin."]
pub type FffinW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Units 62.5 ppb. Max range +/-100 ppm plus margin."]
    #[inline(always)]
    pub fn fffin(&self) -> FffinR {
        FffinR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Units 62.5 ppb. Max range +/-100 ppm plus margin."]
    #[inline(always)]
    pub fn fffin(&mut self) -> FffinW<FfoinSpec> {
        FffinW::new(self, 0)
    }
}
#[doc = "Override value of FFO (Fractional Frequency Offset) if not to be based on the frequency estimate derived from CnAcc (autocorrelation of the scaled input signal) value\n\nYou can [`read`](crate::Reg::read) this register and get [`ffoin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffoin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfoinSpec;
impl crate::RegisterSpec for FfoinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffoin::R`](R) reader structure"]
impl crate::Readable for FfoinSpec {}
#[doc = "`write(|w| ..)` method takes [`ffoin::W`](W) writer structure"]
impl crate::Writable for FfoinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFOIN to value 0"]
impl crate::Resettable for FfoinSpec {
    const RESET_VALUE: u32 = 0;
}
