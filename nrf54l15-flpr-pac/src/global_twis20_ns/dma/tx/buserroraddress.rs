#[doc = "Register `BUSERRORADDRESS` reader"]
pub type R = crate::R<BuserroraddressSpec>;
#[doc = "Register `BUSERRORADDRESS` writer"]
pub type W = crate::W<BuserroraddressSpec>;
#[doc = "Field `ADDRESS` reader - "]
pub type AddressR = crate::FieldReader<u32>;
#[doc = "Field `ADDRESS` writer - "]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn address(&mut self) -> AddressW<BuserroraddressSpec> {
        AddressW::new(self, 0)
    }
}
#[doc = "Address of transaction that generated the last BUSERROR event.\n\nYou can [`read`](crate::Reg::read) this register and get [`buserroraddress::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buserroraddress::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BuserroraddressSpec;
impl crate::RegisterSpec for BuserroraddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buserroraddress::R`](R) reader structure"]
impl crate::Readable for BuserroraddressSpec {}
#[doc = "`write(|w| ..)` method takes [`buserroraddress::W`](W) writer structure"]
impl crate::Writable for BuserroraddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUSERRORADDRESS to value 0"]
impl crate::Resettable for BuserroraddressSpec {
    const RESET_VALUE: u32 = 0;
}
