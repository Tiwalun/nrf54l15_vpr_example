#[doc = "Register `SEEDLOCK` reader"]
pub type R = crate::R<SeedlockSpec>;
#[doc = "Register `SEEDLOCK` writer"]
pub type W = crate::W<SeedlockSpec>;
#[doc = "Enable the lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Lock disabled."]
    Disabled = 0,
    #[doc = "1: Lock enabled."]
    Enabled = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable the lock"]
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
    #[doc = "Lock disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable::Disabled
    }
    #[doc = "Lock enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable::Enabled
    }
}
#[doc = "Field `ENABLE` writer - Enable the lock"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lock disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disabled)
    }
    #[doc = "Lock enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the lock"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the lock"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<SeedlockSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Lock the access to the SEED register.\n\nYou can [`read`](crate::Reg::read) this register and get [`seedlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seedlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeedlockSpec;
impl crate::RegisterSpec for SeedlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seedlock::R`](R) reader structure"]
impl crate::Readable for SeedlockSpec {}
#[doc = "`write(|w| ..)` method takes [`seedlock::W`](W) writer structure"]
impl crate::Writable for SeedlockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEEDLOCK to value 0"]
impl crate::Resettable for SeedlockSpec {
    const RESET_VALUE: u32 = 0;
}
