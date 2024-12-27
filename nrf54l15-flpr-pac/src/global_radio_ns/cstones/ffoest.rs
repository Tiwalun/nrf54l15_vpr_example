#[doc = "Register `FFOEST` reader"]
pub type R = crate::R<FfoestSpec>;
#[doc = "Field `FFOEST` reader - Units 62.5 ppb. Max range +/-100 ppm plus margin."]
pub type FfoestR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Units 62.5 ppb. Max range +/-100 ppm plus margin."]
    #[inline(always)]
    pub fn ffoest(&self) -> FfoestR {
        FfoestR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "FFO estimate\n\nYou can [`read`](crate::Reg::read) this register and get [`ffoest::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfoestSpec;
impl crate::RegisterSpec for FfoestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffoest::R`](R) reader structure"]
impl crate::Readable for FfoestSpec {}
#[doc = "`reset()` method sets FFOEST to value 0"]
impl crate::Resettable for FfoestSpec {
    const RESET_VALUE: u32 = 0;
}
