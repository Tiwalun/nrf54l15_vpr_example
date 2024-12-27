#[doc = "Register `FREQOFFSET` reader"]
pub type R = crate::R<FreqoffsetSpec>;
#[doc = "Field `FREQOFFSET` reader - "]
pub type FreqoffsetR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn freqoffset(&self) -> FreqoffsetR {
        FreqoffsetR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "Frequency offset estimate\n\nYou can [`read`](crate::Reg::read) this register and get [`freqoffset::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FreqoffsetSpec;
impl crate::RegisterSpec for FreqoffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`freqoffset::R`](R) reader structure"]
impl crate::Readable for FreqoffsetSpec {}
#[doc = "`reset()` method sets FREQOFFSET to value 0"]
impl crate::Resettable for FreqoffsetSpec {
    const RESET_VALUE: u32 = 0;
}
