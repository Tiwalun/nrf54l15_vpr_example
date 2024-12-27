#[doc = "Register `BA419SM4HWCFG` reader"]
pub type R = crate::R<Ba419sm4hwcfgSpec>;
#[doc = "Register `BA419SM4HWCFG` writer"]
pub type W = crate::W<Ba419sm4hwcfgSpec>;
#[doc = "Field `BA419SM4HWCFG` reader - Generic g_SM4ModesPoss value."]
pub type Ba419sm4hwcfgR = crate::FieldReader<u16>;
#[doc = "Field `USEMASKING` reader - Generic g_sm4UseMasking value."]
pub type UsemaskingR = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - Generic g_SM4ModesPoss value."]
    #[inline(always)]
    pub fn ba419sm4hwcfg(&self) -> Ba419sm4hwcfgR {
        Ba419sm4hwcfgR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 17 - Generic g_sm4UseMasking value."]
    #[inline(always)]
    pub fn usemasking(&self) -> UsemaskingR {
        UsemaskingR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {}
#[doc = "Generic g_SM4ModesPoss value.\n\nYou can [`read`](crate::Reg::read) this register and get [`ba419sm4hwcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ba419sm4hwcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ba419sm4hwcfgSpec;
impl crate::RegisterSpec for Ba419sm4hwcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ba419sm4hwcfg::R`](R) reader structure"]
impl crate::Readable for Ba419sm4hwcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ba419sm4hwcfg::W`](W) writer structure"]
impl crate::Writable for Ba419sm4hwcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BA419SM4HWCFG to value 0x0002_01ff"]
impl crate::Resettable for Ba419sm4hwcfgSpec {
    const RESET_VALUE: u32 = 0x0002_01ff;
}
