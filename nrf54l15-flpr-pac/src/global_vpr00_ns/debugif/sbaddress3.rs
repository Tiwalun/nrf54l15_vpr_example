#[doc = "Register `SBADDRESS3` reader"]
pub type R = crate::R<Sbaddress3Spec>;
#[doc = "Register `SBADDRESS3` writer"]
pub type W = crate::W<Sbaddress3Spec>;
#[doc = "Field `ADDRESS` reader - Accesses bits 127:96 of the physical address in sbaddress (if the system address bus is that wide)."]
pub type AddressR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Accesses bits 127:96 of the physical address in sbaddress (if the system address bus is that wide)."]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
impl W {}
#[doc = "System Bus Addres 127:96\n\nYou can [`read`](crate::Reg::read) this register and get [`sbaddress3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbaddress3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sbaddress3Spec;
impl crate::RegisterSpec for Sbaddress3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbaddress3::R`](R) reader structure"]
impl crate::Readable for Sbaddress3Spec {}
#[doc = "`write(|w| ..)` method takes [`sbaddress3::W`](W) writer structure"]
impl crate::Writable for Sbaddress3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SBADDRESS3 to value 0"]
impl crate::Resettable for Sbaddress3Spec {
    const RESET_VALUE: u32 = 0;
}
