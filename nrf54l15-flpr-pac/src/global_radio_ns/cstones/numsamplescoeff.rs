#[doc = "Register `NUMSAMPLESCOEFF` reader"]
pub type R = crate::R<NumsamplescoeffSpec>;
#[doc = "Register `NUMSAMPLESCOEFF` writer"]
pub type W = crate::W<NumsamplescoeffSpec>;
#[doc = "Field `NUMSAMPLESCOEFF` reader - Coefficient 2**16/(numSamples/16) in Q1.15 format (Default numsamples value is 160)"]
pub type NumsamplescoeffR = crate::FieldReader<u16>;
#[doc = "Field `NUMSAMPLESCOEFF` writer - Coefficient 2**16/(numSamples/16) in Q1.15 format (Default numsamples value is 160)"]
pub type NumsamplescoeffW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Coefficient 2**16/(numSamples/16) in Q1.15 format (Default numsamples value is 160)"]
    #[inline(always)]
    pub fn numsamplescoeff(&self) -> NumsamplescoeffR {
        NumsamplescoeffR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Coefficient 2**16/(numSamples/16) in Q1.15 format (Default numsamples value is 160)"]
    #[inline(always)]
    #[must_use]
    pub fn numsamplescoeff(&mut self) -> NumsamplescoeffW<NumsamplescoeffSpec> {
        NumsamplescoeffW::new(self, 0)
    }
}
#[doc = "Parameter used in TPM, provided by software\n\nYou can [`read`](crate::Reg::read) this register and get [`numsamplescoeff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`numsamplescoeff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NumsamplescoeffSpec;
impl crate::RegisterSpec for NumsamplescoeffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`numsamplescoeff::R`](R) reader structure"]
impl crate::Readable for NumsamplescoeffSpec {}
#[doc = "`write(|w| ..)` method takes [`numsamplescoeff::W`](W) writer structure"]
impl crate::Writable for NumsamplescoeffSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NUMSAMPLESCOEFF to value 0x199a"]
impl crate::Resettable for NumsamplescoeffSpec {
    const RESET_VALUE: u32 = 0x199a;
}
