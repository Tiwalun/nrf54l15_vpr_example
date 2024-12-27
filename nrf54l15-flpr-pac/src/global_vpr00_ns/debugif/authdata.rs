#[doc = "Register `AUTHDATA` reader"]
pub type R = crate::R<AuthdataSpec>;
#[doc = "Register `AUTHDATA` writer"]
pub type W = crate::W<AuthdataSpec>;
#[doc = "Field `DATA` reader - Data"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {}
#[doc = "Authentication Data\n\nYou can [`read`](crate::Reg::read) this register and get [`authdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`authdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuthdataSpec;
impl crate::RegisterSpec for AuthdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`authdata::R`](R) reader structure"]
impl crate::Readable for AuthdataSpec {}
#[doc = "`write(|w| ..)` method takes [`authdata::W`](W) writer structure"]
impl crate::Writable for AuthdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTHDATA to value 0"]
impl crate::Resettable for AuthdataSpec {
    const RESET_VALUE: u32 = 0;
}
