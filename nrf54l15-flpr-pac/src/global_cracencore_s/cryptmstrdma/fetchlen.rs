#[doc = "Register `FETCHLEN` reader"]
pub type R = crate::R<FetchlenSpec>;
#[doc = "Register `FETCHLEN` writer"]
pub type W = crate::W<FetchlenSpec>;
#[doc = "Field `FETCHLEN` reader - "]
pub type FetchlenR = crate::FieldReader<u32>;
#[doc = "Field `FETCHLEN` writer - "]
pub type FetchlenW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `FETCHCSTADDR` reader - "]
pub type FetchcstaddrR = crate::BitReader;
#[doc = "Field `FETCHCSTADDR` writer - "]
pub type FetchcstaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETCHREALIGN` reader - "]
pub type FetchrealignR = crate::BitReader;
#[doc = "Field `FETCHREALIGN` writer - "]
pub type FetchrealignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETCHZPADDING` reader - "]
pub type FetchzpaddingR = crate::BitReader;
#[doc = "Field `FETCHZPADDING` writer - "]
pub type FetchzpaddingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn fetchlen(&self) -> FetchlenR {
        FetchlenR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn fetchcstaddr(&self) -> FetchcstaddrR {
        FetchcstaddrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn fetchrealign(&self) -> FetchrealignR {
        FetchrealignR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn fetchzpadding(&self) -> FetchzpaddingR {
        FetchzpaddingR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    #[must_use]
    pub fn fetchlen(&mut self) -> FetchlenW<FetchlenSpec> {
        FetchlenW::new(self, 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn fetchcstaddr(&mut self) -> FetchcstaddrW<FetchlenSpec> {
        FetchcstaddrW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn fetchrealign(&mut self) -> FetchrealignW<FetchlenSpec> {
        FetchrealignW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn fetchzpadding(&mut self) -> FetchzpaddingW<FetchlenSpec> {
        FetchzpaddingW::new(self, 30)
    }
}
#[doc = "Fetch Length\n\nYou can [`read`](crate::Reg::read) this register and get [`fetchlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fetchlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FetchlenSpec;
impl crate::RegisterSpec for FetchlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fetchlen::R`](R) reader structure"]
impl crate::Readable for FetchlenSpec {}
#[doc = "`write(|w| ..)` method takes [`fetchlen::W`](W) writer structure"]
impl crate::Writable for FetchlenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FETCHLEN to value 0"]
impl crate::Resettable for FetchlenSpec {
    const RESET_VALUE: u32 = 0;
}
