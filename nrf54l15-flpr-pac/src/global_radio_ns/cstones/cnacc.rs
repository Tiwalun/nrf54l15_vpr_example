#[doc = "Register `CNACC` reader"]
pub type R = crate::R<CnaccSpec>;
#[doc = "Field `CNACCI` reader - "]
pub type CnacciR = crate::FieldReader<u16>;
#[doc = "Field `CNACCQ` reader - "]
pub type CnaccqR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cnacci(&self) -> CnacciR {
        CnacciR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cnaccq(&self) -> CnaccqR {
        CnaccqR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Output of the autocorrelation of the accumulated IQ signal\n\nYou can [`read`](crate::Reg::read) this register and get [`cnacc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CnaccSpec;
impl crate::RegisterSpec for CnaccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnacc::R`](R) reader structure"]
impl crate::Readable for CnaccSpec {}
#[doc = "`reset()` method sets CNACC to value 0"]
impl crate::Resettable for CnaccSpec {
    const RESET_VALUE: u32 = 0;
}
