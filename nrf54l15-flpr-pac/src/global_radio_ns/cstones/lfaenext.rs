#[doc = "Register `LFAENEXT` reader"]
pub type R = crate::R<LfaenextSpec>;
#[doc = "Field `LFAENEXT` reader - Inphase"]
pub type LfaenextR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Inphase"]
    #[inline(always)]
    pub fn lfaenext(&self) -> LfaenextR {
        LfaenextR::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "Quantization error between ADPLL frequency and the desired value of FFO * RF Frequency. Values limited to \\[-64,63\\]
with units 7.6294 Hz.\n\nYou can [`read`](crate::Reg::read) this register and get [`lfaenext::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfaenextSpec;
impl crate::RegisterSpec for LfaenextSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfaenext::R`](R) reader structure"]
impl crate::Readable for LfaenextSpec {}
#[doc = "`reset()` method sets LFAENEXT to value 0"]
impl crate::Resettable for LfaenextSpec {
    const RESET_VALUE: u32 = 0;
}
