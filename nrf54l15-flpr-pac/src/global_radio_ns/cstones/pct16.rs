#[doc = "Register `PCT16` reader"]
pub type R = crate::R<Pct16Spec>;
#[doc = "Field `PCT16I` reader - Inphase"]
pub type Pct16iR = crate::FieldReader<u16>;
#[doc = "Field `PCT16Q` reader - Quadrature"]
pub type Pct16qR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Inphase"]
    #[inline(always)]
    pub fn pct16i(&self) -> Pct16iR {
        Pct16iR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Quadrature"]
    #[inline(always)]
    pub fn pct16q(&self) -> Pct16qR {
        Pct16qR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Mean magnitude and mean phase converted to IQ\n\nYou can [`read`](crate::Reg::read) this register and get [`pct16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pct16Spec;
impl crate::RegisterSpec for Pct16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pct16::R`](R) reader structure"]
impl crate::Readable for Pct16Spec {}
#[doc = "`reset()` method sets PCT16 to value 0"]
impl crate::Resettable for Pct16Spec {
    const RESET_VALUE: u32 = 0;
}
