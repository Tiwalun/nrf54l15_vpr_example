#[doc = "Register `NUMSAMPLES` reader"]
pub type R = crate::R<NumsamplesSpec>;
#[doc = "Register `NUMSAMPLES` writer"]
pub type W = crate::W<NumsamplesSpec>;
#[doc = "Field `NUMSAMPLES` reader - Maximum value supported is 160"]
pub type NumsamplesR = crate::FieldReader;
#[doc = "Field `NUMSAMPLES` writer - Maximum value supported is 160"]
pub type NumsamplesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Maximum value supported is 160"]
    #[inline(always)]
    pub fn numsamples(&self) -> NumsamplesR {
        NumsamplesR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum value supported is 160"]
    #[inline(always)]
    pub fn numsamples(&mut self) -> NumsamplesW<NumsamplesSpec> {
        NumsamplesW::new(self, 0)
    }
}
#[doc = "Number of input samples at 2MHz sample rate\n\nYou can [`read`](crate::Reg::read) this register and get [`numsamples::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`numsamples::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NumsamplesSpec;
impl crate::RegisterSpec for NumsamplesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`numsamples::R`](R) reader structure"]
impl crate::Readable for NumsamplesSpec {}
#[doc = "`write(|w| ..)` method takes [`numsamples::W`](W) writer structure"]
impl crate::Writable for NumsamplesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NUMSAMPLES to value 0xa0"]
impl crate::Resettable for NumsamplesSpec {
    const RESET_VALUE: u32 = 0xa0;
}
