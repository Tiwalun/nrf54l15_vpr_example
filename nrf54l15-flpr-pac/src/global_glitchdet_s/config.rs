#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Enable glitch detector\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Disable glitch detector"]
    Disable = 0,
    #[doc = "1: Enable glitch detector"]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable glitch detector"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disable,
            true => Enable::Enable,
        }
    }
    #[doc = "Disable glitch detector"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable::Disable
    }
    #[doc = "Enable glitch detector"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable::Enable
    }
}
#[doc = "Field `ENABLE` writer - Enable glitch detector"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable glitch detector"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disable)
    }
    #[doc = "Enable glitch detector"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
#[doc = "Glitch detector mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: High pass filter mode"]
    HighPassFilter = 0,
    #[doc = "1: Cap divider mode"]
    CapDiv = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Glitch detector mode"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::HighPassFilter,
            true => Mode::CapDiv,
        }
    }
    #[doc = "High pass filter mode"]
    #[inline(always)]
    pub fn is_high_pass_filter(&self) -> bool {
        *self == Mode::HighPassFilter
    }
    #[doc = "Cap divider mode"]
    #[inline(always)]
    pub fn is_cap_div(&self) -> bool {
        *self == Mode::CapDiv
    }
}
#[doc = "Field `MODE` writer - Glitch detector mode"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High pass filter mode"]
    #[inline(always)]
    pub fn high_pass_filter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::HighPassFilter)
    }
    #[doc = "Cap divider mode"]
    #[inline(always)]
    pub fn cap_div(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::CapDiv)
    }
}
impl R {
    #[doc = "Bit 0 - Enable glitch detector"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Glitch detector mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable glitch detector"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<ConfigSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 4 - Glitch detector mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<ConfigSpec> {
        ModeW::new(self, 4)
    }
}
#[doc = "Configuration for glitch detector\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0x01"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x01;
}
