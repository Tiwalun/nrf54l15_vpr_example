#[doc = "Register `BUSERRORADDRESS` reader"]
pub type R = crate::R<BuserroraddressSpec>;
#[doc = "Field `ADDRESS` reader - "]
pub type AddressR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
#[doc = "Description cluster: Address of transaction that generated the last BUSERROR event.\n\nYou can [`read`](crate::Reg::read) this register and get [`buserroraddress::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BuserroraddressSpec;
impl crate::RegisterSpec for BuserroraddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buserroraddress::R`](R) reader structure"]
impl crate::Readable for BuserroraddressSpec {}
#[doc = "`reset()` method sets BUSERRORADDRESS to value 0"]
impl crate::Resettable for BuserroraddressSpec {
    const RESET_VALUE: u32 = 0;
}
