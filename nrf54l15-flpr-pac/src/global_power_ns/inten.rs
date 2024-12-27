#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event POFWARN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pofwarn {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Pofwarn> for bool {
    #[inline(always)]
    fn from(variant: Pofwarn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFWARN` reader - Enable or disable interrupt for event POFWARN"]
pub type PofwarnR = crate::BitReader<Pofwarn>;
impl PofwarnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pofwarn {
        match self.bits {
            false => Pofwarn::Disabled,
            true => Pofwarn::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pofwarn::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pofwarn::Enabled
    }
}
#[doc = "Field `POFWARN` writer - Enable or disable interrupt for event POFWARN"]
pub type PofwarnW<'a, REG> = crate::BitWriter<'a, REG, Pofwarn>;
impl<'a, REG> PofwarnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pofwarn::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pofwarn::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event SLEEPENTER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleepenter {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Sleepenter> for bool {
    #[inline(always)]
    fn from(variant: Sleepenter) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPENTER` reader - Enable or disable interrupt for event SLEEPENTER"]
pub type SleepenterR = crate::BitReader<Sleepenter>;
impl SleepenterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sleepenter {
        match self.bits {
            false => Sleepenter::Disabled,
            true => Sleepenter::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sleepenter::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sleepenter::Enabled
    }
}
#[doc = "Field `SLEEPENTER` writer - Enable or disable interrupt for event SLEEPENTER"]
pub type SleepenterW<'a, REG> = crate::BitWriter<'a, REG, Sleepenter>;
impl<'a, REG> SleepenterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepenter::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepenter::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event SLEEPEXIT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleepexit {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Sleepexit> for bool {
    #[inline(always)]
    fn from(variant: Sleepexit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPEXIT` reader - Enable or disable interrupt for event SLEEPEXIT"]
pub type SleepexitR = crate::BitReader<Sleepexit>;
impl SleepexitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sleepexit {
        match self.bits {
            false => Sleepexit::Disabled,
            true => Sleepexit::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sleepexit::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sleepexit::Enabled
    }
}
#[doc = "Field `SLEEPEXIT` writer - Enable or disable interrupt for event SLEEPEXIT"]
pub type SleepexitW<'a, REG> = crate::BitWriter<'a, REG, Sleepexit>;
impl<'a, REG> SleepexitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepexit::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepexit::Enabled)
    }
}
impl R {
    #[doc = "Bit 12 - Enable or disable interrupt for event POFWARN"]
    #[inline(always)]
    pub fn pofwarn(&self) -> PofwarnR {
        PofwarnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for event SLEEPENTER"]
    #[inline(always)]
    pub fn sleepenter(&self) -> SleepenterR {
        SleepenterR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event SLEEPEXIT"]
    #[inline(always)]
    pub fn sleepexit(&self) -> SleepexitR {
        SleepexitR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Enable or disable interrupt for event POFWARN"]
    #[inline(always)]
    #[must_use]
    pub fn pofwarn(&mut self) -> PofwarnW<IntenSpec> {
        PofwarnW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for event SLEEPENTER"]
    #[inline(always)]
    #[must_use]
    pub fn sleepenter(&mut self) -> SleepenterW<IntenSpec> {
        SleepenterW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event SLEEPEXIT"]
    #[inline(always)]
    #[must_use]
    pub fn sleepexit(&mut self) -> SleepexitW<IntenSpec> {
        SleepexitW::new(self, 14)
    }
}
#[doc = "Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
