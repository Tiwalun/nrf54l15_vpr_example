#[doc = "Register `SBADDRESS1` reader"]
pub type R = crate::R<Sbaddress1Spec>;
#[doc = "Register `SBADDRESS1` writer"]
pub type W = crate::W<Sbaddress1Spec>;
#[doc = "Field `ADDRESS` reader - Accesses bits 63:32 of the physical address in sbaddress (if the system address bus is that wide)."]
pub type AddressR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Accesses bits 63:32 of the physical address in sbaddress (if the system address bus is that wide)."]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
impl W {}
#[doc = "System Bus Addres 63:32\n\nYou can [`read`](crate::Reg::read) this register and get [`sbaddress1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbaddress1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sbaddress1Spec;
impl crate::RegisterSpec for Sbaddress1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbaddress1::R`](R) reader structure"]
impl crate::Readable for Sbaddress1Spec {}
#[doc = "`write(|w| ..)` method takes [`sbaddress1::W`](W) writer structure"]
impl crate::Writable for Sbaddress1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SBADDRESS1 to value 0"]
impl crate::Resettable for Sbaddress1Spec {
    const RESET_VALUE: u32 = 0;
}
