#[doc = "Register `BA411EAESHWCFG1` reader"]
pub type R = crate::R<Ba411eaeshwcfg1Spec>;
#[doc = "Register `BA411EAESHWCFG1` writer"]
pub type W = crate::W<Ba411eaeshwcfg1Spec>;
#[doc = "Field `BA411EAESHWCFGMODE` reader - Generic g_AesModesPoss value."]
pub type Ba411eaeshwcfgmodeR = crate::FieldReader<u16>;
#[doc = "Field `BA411EAESHWCFGCS` reader - Generic g_CS value."]
pub type Ba411eaeshwcfgcsR = crate::BitReader;
#[doc = "Field `BA411EAESHWCFGMASKING` reader - Generic g_UseMasking value."]
pub type Ba411eaeshwcfgmaskingR = crate::BitReader;
#[doc = "Field `BA411EAESHWCFGKEYSIZE` reader - Generic g_Keysize value."]
pub type Ba411eaeshwcfgkeysizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:8 - Generic g_AesModesPoss value."]
    #[inline(always)]
    pub fn ba411eaeshwcfgmode(&self) -> Ba411eaeshwcfgmodeR {
        Ba411eaeshwcfgmodeR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - Generic g_CS value."]
    #[inline(always)]
    pub fn ba411eaeshwcfgcs(&self) -> Ba411eaeshwcfgcsR {
        Ba411eaeshwcfgcsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Generic g_UseMasking value."]
    #[inline(always)]
    pub fn ba411eaeshwcfgmasking(&self) -> Ba411eaeshwcfgmaskingR {
        Ba411eaeshwcfgmaskingR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Generic g_Keysize value."]
    #[inline(always)]
    pub fn ba411eaeshwcfgkeysize(&self) -> Ba411eaeshwcfgkeysizeR {
        Ba411eaeshwcfgkeysizeR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {}
#[doc = "Generic g_AesModesPoss value.\n\nYou can [`read`](crate::Reg::read) this register and get [`ba411eaeshwcfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ba411eaeshwcfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ba411eaeshwcfg1Spec;
impl crate::RegisterSpec for Ba411eaeshwcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ba411eaeshwcfg1::R`](R) reader structure"]
impl crate::Readable for Ba411eaeshwcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`ba411eaeshwcfg1::W`](W) writer structure"]
impl crate::Writable for Ba411eaeshwcfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BA411EAESHWCFG1 to value 0x0703_01ff"]
impl crate::Resettable for Ba411eaeshwcfg1Spec {
    const RESET_VALUE: u32 = 0x0703_01ff;
}
