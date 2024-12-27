#[doc = "Register `MAGPHASEMEAN` reader"]
pub type R = crate::R<MagphasemeanSpec>;
#[doc = "Field `PHASE` reader - Mean phase"]
pub type PhaseR = crate::FieldReader<u16>;
#[doc = "Field `MAG` reader - Mean magnitude"]
pub type MagR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Mean phase"]
    #[inline(always)]
    pub fn phase(&self) -> PhaseR {
        PhaseR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Mean magnitude"]
    #[inline(always)]
    pub fn mag(&self) -> MagR {
        MagR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Mean magnitude and phase of the signal before it is converted to PCT16\n\nYou can [`read`](crate::Reg::read) this register and get [`magphasemean::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MagphasemeanSpec;
impl crate::RegisterSpec for MagphasemeanSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`magphasemean::R`](R) reader structure"]
impl crate::Readable for MagphasemeanSpec {}
#[doc = "`reset()` method sets MAGPHASEMEAN to value 0"]
impl crate::Resettable for MagphasemeanSpec {
    const RESET_VALUE: u32 = 0;
}
