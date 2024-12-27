#[doc = "Register `RAMERRORSTATUS` reader"]
pub type R = crate::R<RamerrorstatusSpec>;
#[doc = "Register `RAMERRORSTATUS` writer"]
pub type W = crate::W<RamerrorstatusSpec>;
#[doc = "Field `RAMCORRECTION` reader - This bit indicates that a 1-bit error has been detected and corrected on RAM interface"]
pub type RamcorrectionR = crate::BitReader;
#[doc = "Field `RAMFAILURE` reader - This bit indicates that an uncorrectable error has been detected on the data RAM interface"]
pub type RamfailureR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit indicates that a 1-bit error has been detected and corrected on RAM interface"]
    #[inline(always)]
    pub fn ramcorrection(&self) -> RamcorrectionR {
        RamcorrectionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit indicates that an uncorrectable error has been detected on the data RAM interface"]
    #[inline(always)]
    pub fn ramfailure(&self) -> RamfailureR {
        RamfailureR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {}
#[doc = "RAM error status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ramerrorstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramerrorstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamerrorstatusSpec;
impl crate::RegisterSpec for RamerrorstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ramerrorstatus::R`](R) reader structure"]
impl crate::Readable for RamerrorstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ramerrorstatus::W`](W) writer structure"]
impl crate::Writable for RamerrorstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAMERRORSTATUS to value 0"]
impl crate::Resettable for RamerrorstatusSpec {
    const RESET_VALUE: u32 = 0;
}
