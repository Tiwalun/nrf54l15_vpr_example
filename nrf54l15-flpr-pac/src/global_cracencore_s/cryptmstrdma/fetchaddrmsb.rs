#[doc = "Register `FETCHADDRMSB` reader"]
pub type R = crate::R<FetchaddrmsbSpec>;
#[doc = "Register `FETCHADDRMSB` writer"]
pub type W = crate::W<FetchaddrmsbSpec>;
#[doc = "Field `FETCHADDRMSB` reader - "]
pub type FetchaddrmsbR = crate::FieldReader<u32>;
#[doc = "Field `FETCHADDRMSB` writer - "]
pub type FetchaddrmsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fetchaddrmsb(&self) -> FetchaddrmsbR {
        FetchaddrmsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fetchaddrmsb(&mut self) -> FetchaddrmsbW<FetchaddrmsbSpec> {
        FetchaddrmsbW::new(self, 0)
    }
}
#[doc = "Fetch Address Most Significant Bit\n\nYou can [`read`](crate::Reg::read) this register and get [`fetchaddrmsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fetchaddrmsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FetchaddrmsbSpec;
impl crate::RegisterSpec for FetchaddrmsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fetchaddrmsb::R`](R) reader structure"]
impl crate::Readable for FetchaddrmsbSpec {}
#[doc = "`write(|w| ..)` method takes [`fetchaddrmsb::W`](W) writer structure"]
impl crate::Writable for FetchaddrmsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FETCHADDRMSB to value 0"]
impl crate::Resettable for FetchaddrmsbSpec {
    const RESET_VALUE: u32 = 0;
}
