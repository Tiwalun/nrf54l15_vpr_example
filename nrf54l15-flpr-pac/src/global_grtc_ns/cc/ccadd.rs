#[doc = "Register `CCADD` reader"]
pub type R = crate::R<CcaddSpec>;
#[doc = "Register `CCADD` writer"]
pub type W = crate::W<CcaddSpec>;
#[doc = "Field `VALUE` reader - Count to add to CC\\[n\\]"]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Count to add to CC\\[n\\]"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Configure the Capture/Compare register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reference {
    #[doc = "0: Adds SYSCOUNTER value."]
    Syscounter = 0,
    #[doc = "1: Adds CC value."]
    Cc = 1,
}
impl From<Reference> for bool {
    #[inline(always)]
    fn from(variant: Reference) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFERENCE` reader - Configure the Capture/Compare register"]
pub type ReferenceR = crate::BitReader<Reference>;
impl ReferenceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reference {
        match self.bits {
            false => Reference::Syscounter,
            true => Reference::Cc,
        }
    }
    #[doc = "Adds SYSCOUNTER value."]
    #[inline(always)]
    pub fn is_syscounter(&self) -> bool {
        *self == Reference::Syscounter
    }
    #[doc = "Adds CC value."]
    #[inline(always)]
    pub fn is_cc(&self) -> bool {
        *self == Reference::Cc
    }
}
#[doc = "Field `REFERENCE` writer - Configure the Capture/Compare register"]
pub type ReferenceW<'a, REG> = crate::BitWriter<'a, REG, Reference>;
impl<'a, REG> ReferenceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Adds SYSCOUNTER value."]
    #[inline(always)]
    pub fn syscounter(self) -> &'a mut crate::W<REG> {
        self.variant(Reference::Syscounter)
    }
    #[doc = "Adds CC value."]
    #[inline(always)]
    pub fn cc(self) -> &'a mut crate::W<REG> {
        self.variant(Reference::Cc)
    }
}
impl R {
    #[doc = "Bits 0:30 - Count to add to CC\\[n\\]"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Configure the Capture/Compare register"]
    #[inline(always)]
    pub fn reference(&self) -> ReferenceR {
        ReferenceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Count to add to CC\\[n\\]"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<CcaddSpec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bit 31 - Configure the Capture/Compare register"]
    #[inline(always)]
    pub fn reference(&mut self) -> ReferenceW<CcaddSpec> {
        ReferenceW::new(self, 31)
    }
}
#[doc = "Description cluster: Count to add to CC\\[n\\]
when this register is written.\n\nYou can [`read`](crate::Reg::read) this register and get [`ccadd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccadd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcaddSpec;
impl crate::RegisterSpec for CcaddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccadd::R`](R) reader structure"]
impl crate::Readable for CcaddSpec {}
#[doc = "`write(|w| ..)` method takes [`ccadd::W`](W) writer structure"]
impl crate::Writable for CcaddSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCADD to value 0"]
impl crate::Resettable for CcaddSpec {
    const RESET_VALUE: u32 = 0;
}
