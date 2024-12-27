#[doc = "Register `EVTENCLR` reader"]
pub type R = crate::R<EvtenclrSpec>;
#[doc = "Register `EVTENCLR` writer"]
pub type W = crate::W<EvtenclrSpec>;
#[doc = "Write '1' to disable event routing for event PWMPERIODEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmperiodend {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Pwmperiodend> for bool {
    #[inline(always)]
    fn from(variant: Pwmperiodend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMPERIODEND` reader - Write '1' to disable event routing for event PWMPERIODEND"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmperiodend::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmperiodend::Enabled
    }
}
#[doc = "Write '1' to disable event routing for event PWMPERIODEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwmperiodendWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<PwmperiodendWO> for bool {
    #[inline(always)]
    fn from(variant: PwmperiodendWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMPERIODEND` writer - Write '1' to disable event routing for event PWMPERIODEND"]
pub type PwmperiodendW<'a, REG> = crate::BitWriter<'a, REG, PwmperiodendWO>;
impl<'a, REG> PwmperiodendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PwmperiodendWO::Clear)
    }
}
impl R {
    #[doc = "Bit 27 - Write '1' to disable event routing for event PWMPERIODEND"]
    #[inline(always)]
    pub fn pwmperiodend(&self) -> PwmperiodendR {
        PwmperiodendR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - Write '1' to disable event routing for event PWMPERIODEND"]
    #[inline(always)]
    #[must_use]
    pub fn pwmperiodend(&mut self) -> PwmperiodendW<EvtenclrSpec> {
        PwmperiodendW::new(self, 27)
    }
}
#[doc = "Disable event routing\n\nYou can [`read`](crate::Reg::read) this register and get [`evtenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evtenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtenclrSpec;
impl crate::RegisterSpec for EvtenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evtenclr::R`](R) reader structure"]
impl crate::Readable for EvtenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`evtenclr::W`](W) writer structure"]
impl crate::Writable for EvtenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVTENCLR to value 0"]
impl crate::Resettable for EvtenclrSpec {
    const RESET_VALUE: u32 = 0;
}
