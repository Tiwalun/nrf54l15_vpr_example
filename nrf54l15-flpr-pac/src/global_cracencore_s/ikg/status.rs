#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `SEEDERROR` reader - Seed Error during Isolated Key Generation."]
pub type SeederrorR = crate::BitReader;
#[doc = "Field `ENTROPYERROR` reader - Entropy Error during Isolated Key Generation."]
pub type EntropyerrorR = crate::BitReader;
#[doc = "Field `OKAY` reader - Isolated Key Generation is okay."]
pub type OkayR = crate::BitReader;
#[doc = "Field `CTRDRBGBUSY` reader - CTR_DRBG health test is busy (only when g_hw_health_test = true)."]
pub type CtrdrbgbusyR = crate::BitReader;
#[doc = "Field `CATASTROPHICERROR` reader - Catastrophic error during CTR_DRBG health test (only when g_hw_health_test = true)."]
pub type CatastrophicerrorR = crate::BitReader;
#[doc = "Field `SYMKEYSTORED` reader - Symmetric Keys are stored."]
pub type SymkeystoredR = crate::BitReader;
#[doc = "Field `PRIVKEYSTORED` reader - Private Keys are stored."]
pub type PrivkeystoredR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Seed Error during Isolated Key Generation."]
    #[inline(always)]
    pub fn seederror(&self) -> SeederrorR {
        SeederrorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Entropy Error during Isolated Key Generation."]
    #[inline(always)]
    pub fn entropyerror(&self) -> EntropyerrorR {
        EntropyerrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Isolated Key Generation is okay."]
    #[inline(always)]
    pub fn okay(&self) -> OkayR {
        OkayR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CTR_DRBG health test is busy (only when g_hw_health_test = true)."]
    #[inline(always)]
    pub fn ctrdrbgbusy(&self) -> CtrdrbgbusyR {
        CtrdrbgbusyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Catastrophic error during CTR_DRBG health test (only when g_hw_health_test = true)."]
    #[inline(always)]
    pub fn catastrophicerror(&self) -> CatastrophicerrorR {
        CatastrophicerrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Symmetric Keys are stored."]
    #[inline(always)]
    pub fn symkeystored(&self) -> SymkeystoredR {
        SymkeystoredR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Private Keys are stored."]
    #[inline(always)]
    pub fn privkeystored(&self) -> PrivkeystoredR {
        PrivkeystoredR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {}
#[doc = "Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
