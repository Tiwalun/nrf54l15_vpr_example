#[doc = "Register `ADDRESS` reader"]
pub type R = crate::R<AddressSpec>;
#[doc = "Field `ADDRESS` reader - Address"]
pub type AddressR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Address"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Address of the transaction that caused first error.\n\nYou can [`read`](crate::Reg::read) this register and get [`address::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddressSpec;
impl crate::RegisterSpec for AddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`address::R`](R) reader structure"]
impl crate::Readable for AddressSpec {}
#[doc = "`reset()` method sets ADDRESS to value 0"]
impl crate::Resettable for AddressSpec {
    const RESET_VALUE: u32 = 0;
}
