#[doc = "Register `EVTEN` reader"]
pub type R = crate::R<EvtenSpec>;
#[doc = "Register `EVTEN` writer"]
pub type W = crate::W<EvtenSpec>;
#[doc = "Enable or disable event routing for event PWMPERIODEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmperiodend {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Pwmperiodend> for bool {
    #[inline(always)]
    fn from(variant: Pwmperiodend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMPERIODEND` reader - Enable or disable event routing for event PWMPERIODEND"]
pub type PwmperiodendR = crate::BitReader<Pwmperiodend>;
impl PwmperiodendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmperiodend {
        match self.bits {
            false => Pwmperiodend::Disabled,
            true => Pwmperiodend::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmperiodend::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmperiodend::Enabled
    }
}
#[doc = "Field `PWMPERIODEND` writer - Enable or disable event routing for event PWMPERIODEND"]
pub type PwmperiodendW<'a, REG> = crate::BitWriter<'a, REG, Pwmperiodend>;
impl<'a, REG> PwmperiodendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmperiodend::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmperiodend::Enabled)
    }
}
impl R {
    #[doc = "Bit 27 - Enable or disable event routing for event PWMPERIODEND"]
    #[inline(always)]
    pub fn pwmperiodend(&self) -> PwmperiodendR {
        PwmperiodendR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - Enable or disable event routing for event PWMPERIODEND"]
    #[inline(always)]
    pub fn pwmperiodend(&mut self) -> PwmperiodendW<EvtenSpec> {
        PwmperiodendW::new(self, 27)
    }
}
#[doc = "Enable or disable event routing\n\nYou can [`read`](crate::Reg::read) this register and get [`evten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtenSpec;
impl crate::RegisterSpec for EvtenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evten::R`](R) reader structure"]
impl crate::Readable for EvtenSpec {}
#[doc = "`write(|w| ..)` method takes [`evten::W`](W) writer structure"]
impl crate::Writable for EvtenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVTEN to value 0"]
impl crate::Resettable for EvtenSpec {
    const RESET_VALUE: u32 = 0;
}
