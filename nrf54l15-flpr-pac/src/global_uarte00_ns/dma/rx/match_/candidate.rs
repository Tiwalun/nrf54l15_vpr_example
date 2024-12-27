#[doc = "Register `CANDIDATE[%s]` reader"]
pub type R = crate::R<CandidateSpec>;
#[doc = "Register `CANDIDATE[%s]` writer"]
pub type W = crate::W<CandidateSpec>;
#[doc = "Field `DATA` reader - Data to look for"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data to look for"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data to look for"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data to look for"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<CandidateSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Description collection: The data to look for - any match will trigger the MATCH\\[n\\]
event, if enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`candidate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`candidate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CandidateSpec;
impl crate::RegisterSpec for CandidateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`candidate::R`](R) reader structure"]
impl crate::Readable for CandidateSpec {}
#[doc = "`write(|w| ..)` method takes [`candidate::W`](W) writer structure"]
impl crate::Writable for CandidateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CANDIDATE[%s]
to value 0"]
impl crate::Resettable for CandidateSpec {
    const RESET_VALUE: u32 = 0;
}
