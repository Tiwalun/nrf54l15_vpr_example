#[doc = "Register `INDUCTORDET` reader"]
pub type R = crate::R<InductordetSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Detected {
    #[doc = "0: VREGMAIN inductor not detected"]
    InductorNotDetected = 0,
    #[doc = "1: VREGMAIN inductor detected"]
    InductorDetected = 1,
}
impl From<Detected> for bool {
    #[inline(always)]
    fn from(variant: Detected) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DETECTED` reader - "]
pub type DetectedR = crate::BitReader<Detected>;
impl DetectedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Detected {
        match self.bits {
            false => Detected::InductorNotDetected,
            true => Detected::InductorDetected,
        }
    }
    #[doc = "VREGMAIN inductor not detected"]
    #[inline(always)]
    pub fn is_inductor_not_detected(&self) -> bool {
        *self == Detected::InductorNotDetected
    }
    #[doc = "VREGMAIN inductor detected"]
    #[inline(always)]
    pub fn is_inductor_detected(&self) -> bool {
        *self == Detected::InductorDetected
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn detected(&self) -> DetectedR {
        DetectedR::new((self.bits & 1) != 0)
    }
}
#[doc = "VREGMAIN inductor detection\n\nYou can [`read`](crate::Reg::read) this register and get [`inductordet::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InductordetSpec;
impl crate::RegisterSpec for InductordetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inductordet::R`](R) reader structure"]
impl crate::Readable for InductordetSpec {}
#[doc = "`reset()` method sets INDUCTORDET to value 0"]
impl crate::Resettable for InductordetSpec {
    const RESET_VALUE: u32 = 0;
}
