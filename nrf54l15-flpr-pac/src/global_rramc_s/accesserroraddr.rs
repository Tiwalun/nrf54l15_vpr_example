#[doc = "Register `ACCESSERRORADDR` reader"]
pub type R = crate::R<AccesserroraddrSpec>;
#[doc = "Field `ADDRESS` reader - Access error address"]
pub type AddressR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Access error address"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
#[doc = "Address of the first access error\n\nYou can [`read`](crate::Reg::read) this register and get [`accesserroraddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AccesserroraddrSpec;
impl crate::RegisterSpec for AccesserroraddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`accesserroraddr::R`](R) reader structure"]
impl crate::Readable for AccesserroraddrSpec {}
#[doc = "`reset()` method sets ACCESSERRORADDR to value 0x00ff_ffff"]
impl crate::Resettable for AccesserroraddrSpec {
    const RESET_VALUE: u32 = 0x00ff_ffff;
}
