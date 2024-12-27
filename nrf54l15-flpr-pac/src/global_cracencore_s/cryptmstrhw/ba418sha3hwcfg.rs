#[doc = "Register `BA418SHA3HWCFG` reader"]
pub type R = crate::R<Ba418sha3hwcfgSpec>;
#[doc = "Register `BA418SHA3HWCFG` writer"]
pub type W = crate::W<Ba418sha3hwcfgSpec>;
#[doc = "Field `BA418SHA3HWCFG` reader - Generic g_Sha3CtxtEn value."]
pub type Ba418sha3hwcfgR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Generic g_Sha3CtxtEn value."]
    #[inline(always)]
    pub fn ba418sha3hwcfg(&self) -> Ba418sha3hwcfgR {
        Ba418sha3hwcfgR::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "Generic g_Sha3CtxtEn value.\n\nYou can [`read`](crate::Reg::read) this register and get [`ba418sha3hwcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ba418sha3hwcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ba418sha3hwcfgSpec;
impl crate::RegisterSpec for Ba418sha3hwcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ba418sha3hwcfg::R`](R) reader structure"]
impl crate::Readable for Ba418sha3hwcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ba418sha3hwcfg::W`](W) writer structure"]
impl crate::Writable for Ba418sha3hwcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BA418SHA3HWCFG to value 0x01"]
impl crate::Resettable for Ba418sha3hwcfgSpec {
    const RESET_VALUE: u32 = 0x01;
}
