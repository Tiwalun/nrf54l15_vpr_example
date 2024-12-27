#[doc = "Register `CURRENTAMOUNT` reader"]
pub type R = crate::R<CurrentamountSpec>;
#[doc = "Field `AMOUNT` reader - Number of bytes transferred in the current transaction. Continuously updated."]
pub type AmountR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Number of bytes transferred in the current transaction. Continuously updated."]
    #[inline(always)]
    pub fn amount(&self) -> AmountR {
        AmountR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Number of bytes transferred in the current transaction\n\nYou can [`read`](crate::Reg::read) this register and get [`currentamount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrentamountSpec;
impl crate::RegisterSpec for CurrentamountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`currentamount::R`](R) reader structure"]
impl crate::Readable for CurrentamountSpec {}
#[doc = "`reset()` method sets CURRENTAMOUNT to value 0"]
impl crate::Resettable for CurrentamountSpec {
    const RESET_VALUE: u32 = 0;
}
