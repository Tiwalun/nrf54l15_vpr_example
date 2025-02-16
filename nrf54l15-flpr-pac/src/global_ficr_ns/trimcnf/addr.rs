#[doc = "Register `ADDR` reader"]
pub type R = crate::R<AddrSpec>;
#[doc = "Field `Address` reader - Address"]
pub type AddressR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
#[doc = "Description cluster: Address of the register which will be written\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrSpec;
impl crate::RegisterSpec for AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for AddrSpec {}
#[doc = "`reset()` method sets ADDR to value 0xffff_ffff"]
impl crate::Resettable for AddrSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
