#[doc = "Register `PCT11` reader"]
pub type R = crate::R<Pct11Spec>;
#[doc = "Field `PCT11I` reader - Inphase"]
pub type Pct11iR = crate::FieldReader<u16>;
#[doc = "Field `PCT11Q` reader - Quadrature"]
pub type Pct11qR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Inphase"]
    #[inline(always)]
    pub fn pct11i(&self) -> Pct11iR {
        Pct11iR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - Quadrature"]
    #[inline(always)]
    pub fn pct11q(&self) -> Pct11qR {
        Pct11qR::new(((self.bits >> 11) & 0x07ff) as u16)
    }
}
#[doc = "Mean magnitude and mean phase converted to IQ. IQ values limited to \\[-1024,1023\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`pct11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pct11Spec;
impl crate::RegisterSpec for Pct11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pct11::R`](R) reader structure"]
impl crate::Readable for Pct11Spec {}
#[doc = "`reset()` method sets PCT11 to value 0"]
impl crate::Resettable for Pct11Spec {
    const RESET_VALUE: u32 = 0;
}
