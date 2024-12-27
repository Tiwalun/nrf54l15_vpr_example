#[doc = "Register `FFOSOURCE` reader"]
pub type R = crate::R<FfosourceSpec>;
#[doc = "Register `FFOSOURCE` writer"]
pub type W = crate::W<FfosourceSpec>;
#[doc = "Use external or internal FFOSOURCE\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ffosource {
    #[doc = "0: Use FFOIN"]
    External = 0,
    #[doc = "1: Calc FFO from CnAcc"]
    Internal = 1,
}
impl From<Ffosource> for bool {
    #[inline(always)]
    fn from(variant: Ffosource) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFOSOURCE` reader - Use external or internal FFOSOURCE"]
pub type FfosourceR = crate::BitReader<Ffosource>;
impl FfosourceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ffosource {
        match self.bits {
            false => Ffosource::External,
            true => Ffosource::Internal,
        }
    }
    #[doc = "Use FFOIN"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == Ffosource::External
    }
    #[doc = "Calc FFO from CnAcc"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == Ffosource::Internal
    }
}
#[doc = "Field `FFOSOURCE` writer - Use external or internal FFOSOURCE"]
pub type FfosourceW<'a, REG> = crate::BitWriter<'a, REG, Ffosource>;
impl<'a, REG> FfosourceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use FFOIN"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(Ffosource::External)
    }
    #[doc = "Calc FFO from CnAcc"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(Ffosource::Internal)
    }
}
impl R {
    #[doc = "Bit 0 - Use external or internal FFOSOURCE"]
    #[inline(always)]
    pub fn ffosource(&self) -> FfosourceR {
        FfosourceR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Use external or internal FFOSOURCE"]
    #[inline(always)]
    pub fn ffosource(&mut self) -> FfosourceW<FfosourceSpec> {
        FfosourceW::new(self, 0)
    }
}
#[doc = "Source of FFO\n\nYou can [`read`](crate::Reg::read) this register and get [`ffosource::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffosource::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfosourceSpec;
impl crate::RegisterSpec for FfosourceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffosource::R`](R) reader structure"]
impl crate::Readable for FfosourceSpec {}
#[doc = "`write(|w| ..)` method takes [`ffosource::W`](W) writer structure"]
impl crate::Writable for FfosourceSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFOSOURCE to value 0x01"]
impl crate::Resettable for FfosourceSpec {
    const RESET_VALUE: u32 = 0x01;
}
