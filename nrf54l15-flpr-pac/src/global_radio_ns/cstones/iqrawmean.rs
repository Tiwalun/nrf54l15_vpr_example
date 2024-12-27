#[doc = "Register `IQRAWMEAN` reader"]
pub type R = crate::R<IqrawmeanSpec>;
#[doc = "Field `IQRAWMEANI` reader - Inphase"]
pub type IqrawmeaniR = crate::FieldReader<u16>;
#[doc = "Field `IQRAWMEANQ` reader - Quadrature"]
pub type IqrawmeanqR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Inphase"]
    #[inline(always)]
    pub fn iqrawmeani(&self) -> IqrawmeaniR {
        IqrawmeaniR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Quadrature"]
    #[inline(always)]
    pub fn iqrawmeanq(&self) -> IqrawmeanqR {
        IqrawmeanqR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Mean of IQ values\n\nYou can [`read`](crate::Reg::read) this register and get [`iqrawmean::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IqrawmeanSpec;
impl crate::RegisterSpec for IqrawmeanSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iqrawmean::R`](R) reader structure"]
impl crate::Readable for IqrawmeanSpec {}
#[doc = "`reset()` method sets IQRAWMEAN to value 0"]
impl crate::Resettable for IqrawmeanSpec {
    const RESET_VALUE: u32 = 0;
}
