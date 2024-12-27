#[doc = "Register `SBADDRESS2` reader"]
pub type R = crate::R<Sbaddress2Spec>;
#[doc = "Register `SBADDRESS2` writer"]
pub type W = crate::W<Sbaddress2Spec>;
#[doc = "Field `ADDRESS` reader - Accesses bits 95:64 of the physical address in sbaddress (if the system address bus is that wide)."]
pub type AddressR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Accesses bits 95:64 of the physical address in sbaddress (if the system address bus is that wide)."]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
impl W {}
#[doc = "System Bus Addres 95:64\n\nYou can [`read`](crate::Reg::read) this register and get [`sbaddress2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbaddress2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sbaddress2Spec;
impl crate::RegisterSpec for Sbaddress2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbaddress2::R`](R) reader structure"]
impl crate::Readable for Sbaddress2Spec {}
#[doc = "`write(|w| ..)` method takes [`sbaddress2::W`](W) writer structure"]
impl crate::Writable for Sbaddress2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SBADDRESS2 to value 0"]
impl crate::Resettable for Sbaddress2Spec {
    const RESET_VALUE: u32 = 0;
}
