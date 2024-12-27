#[doc = "Register `CFOPHASE` reader"]
pub type R = crate::R<CfophaseSpec>;
#[doc = "Field `CFOPHASE` reader - "]
pub type CfophaseR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cfophase(&self) -> CfophaseR {
        CfophaseR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Cordic output of CnAcc\n\nYou can [`read`](crate::Reg::read) this register and get [`cfophase::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfophaseSpec;
impl crate::RegisterSpec for CfophaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfophase::R`](R) reader structure"]
impl crate::Readable for CfophaseSpec {}
#[doc = "`reset()` method sets CFOPHASE to value 0"]
impl crate::Resettable for CfophaseSpec {
    const RESET_VALUE: u32 = 0;
}
