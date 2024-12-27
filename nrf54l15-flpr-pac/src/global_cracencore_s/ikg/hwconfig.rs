#[doc = "Register `HWCONFIG` reader"]
pub type R = crate::R<HwconfigSpec>;
#[doc = "Register `HWCONFIG` writer"]
pub type W = crate::W<HwconfigSpec>;
#[doc = "Field `NBSYMKEYS` reader - Number of Symmetric Keys generated."]
pub type NbsymkeysR = crate::FieldReader;
#[doc = "Field `NBPRIVKEYS` reader - Number of Private Keys generated."]
pub type NbprivkeysR = crate::FieldReader;
#[doc = "Field `IKGCM` reader - Countermeasures for IKG operations are implemented when 1."]
pub type IkgcmR = crate::BitReader;
#[doc = "Field `HWHEALTHTEST` reader - CTR_DRBG health test is implemented when 1."]
pub type HwhealthtestR = crate::BitReader;
#[doc = "ECC curve for IKG (input).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Curve {
    #[doc = "0: P256."]
    P256 = 0,
    #[doc = "1: P384."]
    P384 = 1,
    #[doc = "2: P521."]
    P521 = 2,
}
impl From<Curve> for u8 {
    #[inline(always)]
    fn from(variant: Curve) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Curve {
    type Ux = u8;
}
impl crate::IsEnum for Curve {}
#[doc = "Field `CURVE` reader - ECC curve for IKG (input)."]
pub type CurveR = crate::FieldReader<Curve>;
impl CurveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Curve> {
        match self.bits {
            0 => Some(Curve::P256),
            1 => Some(Curve::P384),
            2 => Some(Curve::P521),
            _ => None,
        }
    }
    #[doc = "P256."]
    #[inline(always)]
    pub fn is_p256(&self) -> bool {
        *self == Curve::P256
    }
    #[doc = "P384."]
    #[inline(always)]
    pub fn is_p384(&self) -> bool {
        *self == Curve::P384
    }
    #[doc = "P521."]
    #[inline(always)]
    pub fn is_p521(&self) -> bool {
        *self == Curve::P521
    }
}
#[doc = "Field `DF` reader - Derivation function is implemented in the CTR_DRBG when 1."]
pub type DfR = crate::BitReader;
#[doc = "AES Key Size support for the AES Core embedded in the CTR_DRBG.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keysize {
    #[doc = "1: supports AES128"]
    Aes128 = 1,
    #[doc = "2: supports AES192"]
    Aes192 = 2,
    #[doc = "4: supports AES256"]
    Aes256 = 4,
}
impl From<Keysize> for u8 {
    #[inline(always)]
    fn from(variant: Keysize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Keysize {
    type Ux = u8;
}
impl crate::IsEnum for Keysize {}
#[doc = "Field `KEYSIZE` reader - AES Key Size support for the AES Core embedded in the CTR_DRBG."]
pub type KeysizeR = crate::FieldReader<Keysize>;
impl KeysizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Keysize> {
        match self.bits {
            1 => Some(Keysize::Aes128),
            2 => Some(Keysize::Aes192),
            4 => Some(Keysize::Aes256),
            _ => None,
        }
    }
    #[doc = "supports AES128"]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == Keysize::Aes128
    }
    #[doc = "supports AES192"]
    #[inline(always)]
    pub fn is_aes192(&self) -> bool {
        *self == Keysize::Aes192
    }
    #[doc = "supports AES256"]
    #[inline(always)]
    pub fn is_aes256(&self) -> bool {
        *self == Keysize::Aes256
    }
}
#[doc = "Field `ENTROPYINPUTLENGTH` reader - Value of g_entropy_input_length/32."]
pub type EntropyinputlengthR = crate::FieldReader;
#[doc = "Field `NONCELENGTH` reader - Value of g_nonce_length/32."]
pub type NoncelengthR = crate::FieldReader;
#[doc = "Field `PERSONALIZATIONSTRINGLENGTH` reader - Value of g_personalization_string_length/32."]
pub type PersonalizationstringlengthR = crate::FieldReader;
#[doc = "Field `ADDITIONALINPUTLENGTH` reader - Value of g_additional_input_length/32."]
pub type AdditionalinputlengthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Number of Symmetric Keys generated."]
    #[inline(always)]
    pub fn nbsymkeys(&self) -> NbsymkeysR {
        NbsymkeysR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Number of Private Keys generated."]
    #[inline(always)]
    pub fn nbprivkeys(&self) -> NbprivkeysR {
        NbprivkeysR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Countermeasures for IKG operations are implemented when 1."]
    #[inline(always)]
    pub fn ikgcm(&self) -> IkgcmR {
        IkgcmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTR_DRBG health test is implemented when 1."]
    #[inline(always)]
    pub fn hwhealthtest(&self) -> HwhealthtestR {
        HwhealthtestR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - ECC curve for IKG (input)."]
    #[inline(always)]
    pub fn curve(&self) -> CurveR {
        CurveR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Derivation function is implemented in the CTR_DRBG when 1."]
    #[inline(always)]
    pub fn df(&self) -> DfR {
        DfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - AES Key Size support for the AES Core embedded in the CTR_DRBG."]
    #[inline(always)]
    pub fn keysize(&self) -> KeysizeR {
        KeysizeR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Value of g_entropy_input_length/32."]
    #[inline(always)]
    pub fn entropyinputlength(&self) -> EntropyinputlengthR {
        EntropyinputlengthR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Value of g_nonce_length/32."]
    #[inline(always)]
    pub fn noncelength(&self) -> NoncelengthR {
        NoncelengthR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Value of g_personalization_string_length/32."]
    #[inline(always)]
    pub fn personalizationstringlength(&self) -> PersonalizationstringlengthR {
        PersonalizationstringlengthR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Value of g_additional_input_length/32."]
    #[inline(always)]
    pub fn additionalinputlength(&self) -> AdditionalinputlengthR {
        AdditionalinputlengthR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "HwConfig register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hwconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwconfigSpec;
impl crate::RegisterSpec for HwconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwconfig::R`](R) reader structure"]
impl crate::Readable for HwconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`hwconfig::W`](W) writer structure"]
impl crate::Writable for HwconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWCONFIG to value 0xcc4c_8312"]
impl crate::Resettable for HwconfigSpec {
    const RESET_VALUE: u32 = 0xcc4c_8312;
}
