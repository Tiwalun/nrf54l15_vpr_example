#[doc = "Register `NEXTDM` reader"]
pub type R = crate::R<NextdmSpec>;
#[doc = "Register `NEXTDM` writer"]
pub type W = crate::W<NextdmSpec>;
#[doc = "Field `ADDR` reader - Address"]
pub type AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {}
#[doc = "Next Debug Module\n\nYou can [`read`](crate::Reg::read) this register and get [`nextdm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nextdm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NextdmSpec;
impl crate::RegisterSpec for NextdmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nextdm::R`](R) reader structure"]
impl crate::Readable for NextdmSpec {}
#[doc = "`write(|w| ..)` method takes [`nextdm::W`](W) writer structure"]
impl crate::Writable for NextdmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NEXTDM to value 0"]
impl crate::Resettable for NextdmSpec {
    const RESET_VALUE: u32 = 0;
}
