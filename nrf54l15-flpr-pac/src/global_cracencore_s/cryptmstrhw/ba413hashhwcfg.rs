#[doc = "Register `BA413HASHHWCFG` reader"]
pub type R = crate::R<Ba413hashhwcfgSpec>;
#[doc = "Register `BA413HASHHWCFG` writer"]
pub type W = crate::W<Ba413hashhwcfgSpec>;
#[doc = "Field `BA413HASHHWCFGMASK` reader - Generic g_HashMaskFunc value."]
pub type Ba413hashhwcfgmaskR = crate::FieldReader;
#[doc = "Field `BA413HASHHWCFGPADDING` reader - Generic g_HashPadding value."]
pub type Ba413hashhwcfgpaddingR = crate::BitReader;
#[doc = "Field `BA413HASHHWCFGHMAC` reader - Generic g_HMAC_enabled value."]
pub type Ba413hashhwcfghmacR = crate::BitReader;
#[doc = "Field `BA413HASHHWCFGVERIFYDIGEST` reader - Generic g_HashVerifyDigest value."]
pub type Ba413hashhwcfgverifydigestR = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - Generic g_HashMaskFunc value."]
    #[inline(always)]
    pub fn ba413hashhwcfgmask(&self) -> Ba413hashhwcfgmaskR {
        Ba413hashhwcfgmaskR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Generic g_HashPadding value."]
    #[inline(always)]
    pub fn ba413hashhwcfgpadding(&self) -> Ba413hashhwcfgpaddingR {
        Ba413hashhwcfgpaddingR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Generic g_HMAC_enabled value."]
    #[inline(always)]
    pub fn ba413hashhwcfghmac(&self) -> Ba413hashhwcfghmacR {
        Ba413hashhwcfghmacR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Generic g_HashVerifyDigest value."]
    #[inline(always)]
    pub fn ba413hashhwcfgverifydigest(&self) -> Ba413hashhwcfgverifydigestR {
        Ba413hashhwcfgverifydigestR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {}
#[doc = "Generic g_Hash value\n\nYou can [`read`](crate::Reg::read) this register and get [`ba413hashhwcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ba413hashhwcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ba413hashhwcfgSpec;
impl crate::RegisterSpec for Ba413hashhwcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ba413hashhwcfg::R`](R) reader structure"]
impl crate::Readable for Ba413hashhwcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ba413hashhwcfg::W`](W) writer structure"]
impl crate::Writable for Ba413hashhwcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BA413HASHHWCFG to value 0x0003_003f"]
impl crate::Resettable for Ba413hashhwcfgSpec {
    const RESET_VALUE: u32 = 0x0003_003f;
}
