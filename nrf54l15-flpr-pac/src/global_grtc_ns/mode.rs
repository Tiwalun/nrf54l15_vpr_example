#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Automatic enable to keep the SYSCOUNTER active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autoen {
    #[doc = "0: Default configuration to keep the SYSCOUNTER active."]
    Default = 0,
    #[doc = "1: In addition to the above mode, any local CPU that is not sleeping keep the SYSCOUNTER active."]
    CpuActive = 1,
}
impl From<Autoen> for bool {
    #[inline(always)]
    fn from(variant: Autoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOEN` reader - Automatic enable to keep the SYSCOUNTER active."]
pub type AutoenR = crate::BitReader<Autoen>;
impl AutoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autoen {
        match self.bits {
            false => Autoen::Default,
            true => Autoen::CpuActive,
        }
    }
    #[doc = "Default configuration to keep the SYSCOUNTER active."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Autoen::Default
    }
    #[doc = "In addition to the above mode, any local CPU that is not sleeping keep the SYSCOUNTER active."]
    #[inline(always)]
    pub fn is_cpu_active(&self) -> bool {
        *self == Autoen::CpuActive
    }
}
#[doc = "Field `AUTOEN` writer - Automatic enable to keep the SYSCOUNTER active."]
pub type AutoenW<'a, REG> = crate::BitWriter<'a, REG, Autoen>;
impl<'a, REG> AutoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default configuration to keep the SYSCOUNTER active."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Autoen::Default)
    }
    #[doc = "In addition to the above mode, any local CPU that is not sleeping keep the SYSCOUNTER active."]
    #[inline(always)]
    pub fn cpu_active(self) -> &'a mut crate::W<REG> {
        self.variant(Autoen::CpuActive)
    }
}
#[doc = "Enable the SYSCOUNTER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syscounteren {
    #[doc = "0: SYSCOUNTER disabled"]
    Disabled = 0,
    #[doc = "1: SYSCOUNTER enabled"]
    Enabled = 1,
}
impl From<Syscounteren> for bool {
    #[inline(always)]
    fn from(variant: Syscounteren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCOUNTEREN` reader - Enable the SYSCOUNTER"]
pub type SyscounterenR = crate::BitReader<Syscounteren>;
impl SyscounterenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syscounteren {
        match self.bits {
            false => Syscounteren::Disabled,
            true => Syscounteren::Enabled,
        }
    }
    #[doc = "SYSCOUNTER disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Syscounteren::Disabled
    }
    #[doc = "SYSCOUNTER enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Syscounteren::Enabled
    }
}
#[doc = "Field `SYSCOUNTEREN` writer - Enable the SYSCOUNTER"]
pub type SyscounterenW<'a, REG> = crate::BitWriter<'a, REG, Syscounteren>;
impl<'a, REG> SyscounterenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYSCOUNTER disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Syscounteren::Disabled)
    }
    #[doc = "SYSCOUNTER enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Syscounteren::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Automatic enable to keep the SYSCOUNTER active."]
    #[inline(always)]
    pub fn autoen(&self) -> AutoenR {
        AutoenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the SYSCOUNTER"]
    #[inline(always)]
    pub fn syscounteren(&self) -> SyscounterenR {
        SyscounterenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Automatic enable to keep the SYSCOUNTER active."]
    #[inline(always)]
    #[must_use]
    pub fn autoen(&mut self) -> AutoenW<ModeSpec> {
        AutoenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable the SYSCOUNTER"]
    #[inline(always)]
    #[must_use]
    pub fn syscounteren(&mut self) -> SyscounterenW<ModeSpec> {
        SyscounterenW::new(self, 1)
    }
}
#[doc = "Counter mode selection\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for ModeSpec {
    const RESET_VALUE: u32 = 0;
}
