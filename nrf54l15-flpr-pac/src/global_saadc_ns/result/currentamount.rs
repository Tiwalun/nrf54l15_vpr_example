#[doc = "Register `CURRENTAMOUNT` reader"]
pub type R = crate::R<CurrentamountSpec>;
#[doc = "Field `AMOUNT` reader - Number of buffer bytes transferred since last START, continuously updated."]
pub type AmountR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - Number of buffer bytes transferred since last START, continuously updated."]
    #[inline(always)]
    pub fn amount(&self) -> AmountR {
        AmountR::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "Number of buffer bytes transferred since last START, continuously updated\n\nYou can [`read`](crate::Reg::read) this register and get [`currentamount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
