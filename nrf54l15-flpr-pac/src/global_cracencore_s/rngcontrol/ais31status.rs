#[doc = "Register `AIS31STATUS` reader"]
pub type R = crate::R<Ais31statusSpec>;
#[doc = "Register `AIS31STATUS` writer"]
pub type W = crate::W<Ais31statusSpec>;
#[doc = "Field `NUMPRELIMALARMS` reader - Number of preliminary noise alarms since counter was last cleared."]
pub type NumprelimalarmsR = crate::FieldReader<u16>;
#[doc = "Field `NUMPRELIMALARMS` writer - Number of preliminary noise alarms since counter was last cleared."]
pub type NumprelimalarmsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PRELIMNOISEALARMRNG` reader - Last preliminary noise alarm occurred due to history value out of range."]
pub type PrelimnoisealarmrngR = crate::BitReader;
#[doc = "Field `PRELIMNOISEALARMRNG` writer - Last preliminary noise alarm occurred due to history value out of range."]
pub type PrelimnoisealarmrngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRELIMNOISEALARMREP` reader - Last preliminary noise alarm occurred due to consecutive high Χ**2."]
pub type PrelimnoisealarmrepR = crate::BitReader;
#[doc = "Field `PRELIMNOISEALARMREP` writer - Last preliminary noise alarm occurred due to consecutive high Χ**2."]
pub type PrelimnoisealarmrepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Number of preliminary noise alarms since counter was last cleared."]
    #[inline(always)]
    pub fn numprelimalarms(&self) -> NumprelimalarmsR {
        NumprelimalarmsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Last preliminary noise alarm occurred due to history value out of range."]
    #[inline(always)]
    pub fn prelimnoisealarmrng(&self) -> PrelimnoisealarmrngR {
        PrelimnoisealarmrngR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Last preliminary noise alarm occurred due to consecutive high Χ**2."]
    #[inline(always)]
    pub fn prelimnoisealarmrep(&self) -> PrelimnoisealarmrepR {
        PrelimnoisealarmrepR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of preliminary noise alarms since counter was last cleared."]
    #[inline(always)]
    pub fn numprelimalarms(&mut self) -> NumprelimalarmsW<Ais31statusSpec> {
        NumprelimalarmsW::new(self, 0)
    }
    #[doc = "Bit 16 - Last preliminary noise alarm occurred due to history value out of range."]
    #[inline(always)]
    pub fn prelimnoisealarmrng(&mut self) -> PrelimnoisealarmrngW<Ais31statusSpec> {
        PrelimnoisealarmrngW::new(self, 16)
    }
    #[doc = "Bit 17 - Last preliminary noise alarm occurred due to consecutive high Χ**2."]
    #[inline(always)]
    pub fn prelimnoisealarmrep(&mut self) -> PrelimnoisealarmrepW<Ais31statusSpec> {
        PrelimnoisealarmrepW::new(self, 17)
    }
}
#[doc = "AIS31 status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ais31status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ais31status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ais31statusSpec;
impl crate::RegisterSpec for Ais31statusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ais31status::R`](R) reader structure"]
impl crate::Readable for Ais31statusSpec {}
#[doc = "`write(|w| ..)` method takes [`ais31status::W`](W) writer structure"]
impl crate::Writable for Ais31statusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIS31STATUS to value 0"]
impl crate::Resettable for Ais31statusSpec {
    const RESET_VALUE: u32 = 0;
}
