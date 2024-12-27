#[doc = "Register `FETCHADDRLSB` reader"]
pub type R = crate::R<FetchaddrlsbSpec>;
#[doc = "Register `FETCHADDRLSB` writer"]
pub type W = crate::W<FetchaddrlsbSpec>;
#[doc = "Field `FETCHADDRLSB` reader - "]
pub type FetchaddrlsbR = crate::FieldReader<u32>;
#[doc = "Field `FETCHADDRLSB` writer - "]
pub type FetchaddrlsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fetchaddrlsb(&self) -> FetchaddrlsbR {
        FetchaddrlsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fetchaddrlsb(&mut self) -> FetchaddrlsbW<FetchaddrlsbSpec> {
        FetchaddrlsbW::new(self, 0)
    }
}
#[doc = "Fetch Address Least Significant Bit\n\nYou can [`read`](crate::Reg::read) this register and get [`fetchaddrlsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fetchaddrlsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FetchaddrlsbSpec;
impl crate::RegisterSpec for FetchaddrlsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fetchaddrlsb::R`](R) reader structure"]
impl crate::Readable for FetchaddrlsbSpec {}
#[doc = "`write(|w| ..)` method takes [`fetchaddrlsb::W`](W) writer structure"]
impl crate::Writable for FetchaddrlsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FETCHADDRLSB to value 0"]
impl crate::Resettable for FetchaddrlsbSpec {
    const RESET_VALUE: u32 = 0;
}
