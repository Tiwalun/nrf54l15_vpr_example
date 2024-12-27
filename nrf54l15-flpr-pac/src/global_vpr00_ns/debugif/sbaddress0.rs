#[doc = "Register `SBADDRESS0` reader"]
pub type R = crate::R<Sbaddress0Spec>;
#[doc = "Register `SBADDRESS0` writer"]
pub type W = crate::W<Sbaddress0Spec>;
#[doc = "Field `ADDRESS` reader - Accesses bits 31:0 of the physical address in sbaddress."]
pub type AddressR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Accesses bits 31:0 of the physical address in sbaddress."]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
impl W {}
#[doc = "System Bus Addres 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`sbaddress0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbaddress0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sbaddress0Spec;
impl crate::RegisterSpec for Sbaddress0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbaddress0::R`](R) reader structure"]
impl crate::Readable for Sbaddress0Spec {}
#[doc = "`write(|w| ..)` method takes [`sbaddress0::W`](W) writer structure"]
impl crate::Writable for Sbaddress0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SBADDRESS0 to value 0"]
impl crate::Resettable for Sbaddress0Spec {
    const RESET_VALUE: u32 = 0;
}
