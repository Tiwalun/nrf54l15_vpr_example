#[doc = "Register `INCLIPSHWCFG` reader"]
pub type R = crate::R<InclipshwcfgSpec>;
#[doc = "Register `INCLIPSHWCFG` writer"]
pub type W = crate::W<InclipshwcfgSpec>;
#[doc = "Field `BA411AESINCLUDED` reader - Generic g_IncludeAES value."]
pub type Ba411aesincludedR = crate::BitReader;
#[doc = "Field `BA415HPAESGCMINCLUDED` reader - Generic g_IncludeAESGCM value."]
pub type Ba415hpaesgcmincludedR = crate::BitReader;
#[doc = "Field `BA416HPAESXTSINCLUDED` reader - Generic g_IncludeAESXTS value."]
pub type Ba416hpaesxtsincludedR = crate::BitReader;
#[doc = "Field `BA412DESINCLUDED` reader - Generic g_IncludeDES value."]
pub type Ba412desincludedR = crate::BitReader;
#[doc = "Field `BA413HASHINCLUDED` reader - Generic g_IncludeHASH value."]
pub type Ba413hashincludedR = crate::BitReader;
#[doc = "Field `BA417CHACHAPOLYINCLUDED` reader - Generic g_IncludeChachaPoly value."]
pub type Ba417chachapolyincludedR = crate::BitReader;
#[doc = "Field `BA418SHA3INCLUDED` reader - Generic g_IncludeSHA3 value."]
pub type Ba418sha3includedR = crate::BitReader;
#[doc = "Field `BA421ZUCINCLUDED` reader - Generic g_IncludeZUC value."]
pub type Ba421zucincludedR = crate::BitReader;
#[doc = "Field `BA419SM4INCLUDED` reader - Generic g_IncludeSM4 value."]
pub type Ba419sm4includedR = crate::BitReader;
#[doc = "Field `BA414EPPKEINCLUDED` reader - Generic g_IncludePKE value."]
pub type Ba414eppkeincludedR = crate::BitReader;
#[doc = "Field `BA431NDRNGINCLUDED` reader - Generic g_IncludeNDRNG value."]
pub type Ba431ndrngincludedR = crate::BitReader;
#[doc = "Field `BA420HPCHACHAPOLYINCLUDED` reader - Generic g_IncludeHPChachaPoly value."]
pub type Ba420hpchachapolyincludedR = crate::BitReader;
#[doc = "Field `BA423SNOW3GINCLUDED` reader - Generic g_IncludeSnow3G value."]
pub type Ba423snow3gincludedR = crate::BitReader;
#[doc = "Field `BA422KASUMIINCLUDED` reader - Generic g_IncludeKasumi value."]
pub type Ba422kasumiincludedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Generic g_IncludeAES value."]
    #[inline(always)]
    pub fn ba411aesincluded(&self) -> Ba411aesincludedR {
        Ba411aesincludedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generic g_IncludeAESGCM value."]
    #[inline(always)]
    pub fn ba415hpaesgcmincluded(&self) -> Ba415hpaesgcmincludedR {
        Ba415hpaesgcmincludedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generic g_IncludeAESXTS value."]
    #[inline(always)]
    pub fn ba416hpaesxtsincluded(&self) -> Ba416hpaesxtsincludedR {
        Ba416hpaesxtsincludedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Generic g_IncludeDES value."]
    #[inline(always)]
    pub fn ba412desincluded(&self) -> Ba412desincludedR {
        Ba412desincludedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Generic g_IncludeHASH value."]
    #[inline(always)]
    pub fn ba413hashincluded(&self) -> Ba413hashincludedR {
        Ba413hashincludedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generic g_IncludeChachaPoly value."]
    #[inline(always)]
    pub fn ba417chachapolyincluded(&self) -> Ba417chachapolyincludedR {
        Ba417chachapolyincludedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Generic g_IncludeSHA3 value."]
    #[inline(always)]
    pub fn ba418sha3included(&self) -> Ba418sha3includedR {
        Ba418sha3includedR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generic g_IncludeZUC value."]
    #[inline(always)]
    pub fn ba421zucincluded(&self) -> Ba421zucincludedR {
        Ba421zucincludedR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Generic g_IncludeSM4 value."]
    #[inline(always)]
    pub fn ba419sm4included(&self) -> Ba419sm4includedR {
        Ba419sm4includedR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generic g_IncludePKE value."]
    #[inline(always)]
    pub fn ba414eppkeincluded(&self) -> Ba414eppkeincludedR {
        Ba414eppkeincludedR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Generic g_IncludeNDRNG value."]
    #[inline(always)]
    pub fn ba431ndrngincluded(&self) -> Ba431ndrngincludedR {
        Ba431ndrngincludedR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Generic g_IncludeHPChachaPoly value."]
    #[inline(always)]
    pub fn ba420hpchachapolyincluded(&self) -> Ba420hpchachapolyincludedR {
        Ba420hpchachapolyincludedR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Generic g_IncludeSnow3G value."]
    #[inline(always)]
    pub fn ba423snow3gincluded(&self) -> Ba423snow3gincludedR {
        Ba423snow3gincludedR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Generic g_IncludeKasumi value."]
    #[inline(always)]
    pub fn ba422kasumiincluded(&self) -> Ba422kasumiincludedR {
        Ba422kasumiincludedR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {}
#[doc = "Incuded IPs Hardware configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`inclipshwcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inclipshwcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InclipshwcfgSpec;
impl crate::RegisterSpec for InclipshwcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inclipshwcfg::R`](R) reader structure"]
impl crate::Readable for InclipshwcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`inclipshwcfg::W`](W) writer structure"]
impl crate::Writable for InclipshwcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INCLIPSHWCFG to value 0x0771"]
impl crate::Resettable for InclipshwcfgSpec {
    const RESET_VALUE: u32 = 0x0771;
}
