#[doc = "Register `ERRORADDR` reader"]
pub type R = crate::R<ErroraddrSpec>;
#[doc = "Field `ADDRESS` reader - ECC error address"]
pub type AddressR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC error address"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
#[doc = "Address of the first ECC error that could not be corrected\n\nYou can [`read`](crate::Reg::read) this register and get [`erroraddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErroraddrSpec;
impl crate::RegisterSpec for ErroraddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`erroraddr::R`](R) reader structure"]
impl crate::Readable for ErroraddrSpec {}
#[doc = "`reset()` method sets ERRORADDR to value 0x00ff_ffff"]
impl crate::Resettable for ErroraddrSpec {
    const RESET_VALUE: u32 = 0x00ff_ffff;
}
