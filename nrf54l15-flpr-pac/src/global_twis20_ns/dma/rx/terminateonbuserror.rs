#[doc = "Register `TERMINATEONBUSERROR` reader"]
pub type R = crate::R<TerminateonbuserrorSpec>;
#[doc = "Register `TERMINATEONBUSERROR` writer"]
pub type W = crate::W<TerminateonbuserrorSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - "]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disabled,
            true => Enable::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable::Enabled
    }
}
#[doc = "Field `ENABLE` writer - "]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enabled)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<TerminateonbuserrorSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Terminate the transaction if a BUSERROR event is detected.\n\nYou can [`read`](crate::Reg::read) this register and get [`terminateonbuserror::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`terminateonbuserror::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TerminateonbuserrorSpec;
impl crate::RegisterSpec for TerminateonbuserrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`terminateonbuserror::R`](R) reader structure"]
impl crate::Readable for TerminateonbuserrorSpec {}
#[doc = "`write(|w| ..)` method takes [`terminateonbuserror::W`](W) writer structure"]
impl crate::Writable for TerminateonbuserrorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TERMINATEONBUSERROR to value 0"]
impl crate::Resettable for TerminateonbuserrorSpec {
    const RESET_VALUE: u32 = 0;
}
