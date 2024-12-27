#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Enable or disable TPM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpm {
    #[doc = "0: TPM is disabled"]
    Disabled = 0,
    #[doc = "1: TPM is enabled"]
    Enabled = 1,
}
impl From<Tpm> for bool {
    #[inline(always)]
    fn from(variant: Tpm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPM` reader - Enable or disable TPM"]
pub type TpmR = crate::BitReader<Tpm>;
impl TpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpm {
        match self.bits {
            false => Tpm::Disabled,
            true => Tpm::Enabled,
        }
    }
    #[doc = "TPM is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tpm::Disabled
    }
    #[doc = "TPM is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tpm::Enabled
    }
}
#[doc = "Field `TPM` writer - Enable or disable TPM"]
pub type TpmW<'a, REG> = crate::BitWriter<'a, REG, Tpm>;
impl<'a, REG> TpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TPM is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm::Disabled)
    }
    #[doc = "TPM is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm::Enabled)
    }
}
#[doc = "Enable or disable TFM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfm {
    #[doc = "0: TFM is disabled"]
    Disabled = 0,
    #[doc = "1: TFM is enabled"]
    Enabled = 1,
}
impl From<Tfm> for bool {
    #[inline(always)]
    fn from(variant: Tfm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFM` reader - Enable or disable TFM"]
pub type TfmR = crate::BitReader<Tfm>;
impl TfmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfm {
        match self.bits {
            false => Tfm::Disabled,
            true => Tfm::Enabled,
        }
    }
    #[doc = "TFM is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tfm::Disabled
    }
    #[doc = "TFM is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tfm::Enabled
    }
}
#[doc = "Field `TFM` writer - Enable or disable TFM"]
pub type TfmW<'a, REG> = crate::BitWriter<'a, REG, Tfm>;
impl<'a, REG> TfmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TFM is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tfm::Disabled)
    }
    #[doc = "TFM is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tfm::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable TPM"]
    #[inline(always)]
    pub fn tpm(&self) -> TpmR {
        TpmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable TFM"]
    #[inline(always)]
    pub fn tfm(&self) -> TfmR {
        TfmR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable TPM"]
    #[inline(always)]
    #[must_use]
    pub fn tpm(&mut self) -> TpmW<ModeSpec> {
        TpmW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable TFM"]
    #[inline(always)]
    #[must_use]
    pub fn tfm(&mut self) -> TfmW<ModeSpec> {
        TfmW::new(self, 1)
    }
}
#[doc = "Selects the mode(s) that are activated on the start signal\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE to value 0x03"]
impl crate::Resettable for ModeSpec {
    const RESET_VALUE: u32 = 0x03;
}
