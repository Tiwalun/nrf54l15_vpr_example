#[doc = "Register `FETCHTAG` reader"]
pub type R = crate::R<FetchtagSpec>;
#[doc = "Register `FETCHTAG` writer"]
pub type W = crate::W<FetchtagSpec>;
#[doc = "Field `FETCHTAG` reader - "]
pub type FetchtagR = crate::FieldReader<u32>;
#[doc = "Field `FETCHTAG` writer - "]
pub type FetchtagW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fetchtag(&self) -> FetchtagR {
        FetchtagR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fetchtag(&mut self) -> FetchtagW<FetchtagSpec> {
        FetchtagW::new(self, 0)
    }
}
#[doc = "Fetch Tag\n\nYou can [`read`](crate::Reg::read) this register and get [`fetchtag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fetchtag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FetchtagSpec;
impl crate::RegisterSpec for FetchtagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fetchtag::R`](R) reader structure"]
impl crate::Readable for FetchtagSpec {}
#[doc = "`write(|w| ..)` method takes [`fetchtag::W`](W) writer structure"]
impl crate::Writable for FetchtagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FETCHTAG to value 0"]
impl crate::Resettable for FetchtagSpec {
    const RESET_VALUE: u32 = 0;
}
