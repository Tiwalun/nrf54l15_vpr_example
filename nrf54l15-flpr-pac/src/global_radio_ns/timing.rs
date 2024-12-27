#[doc = "Register `TIMING` reader"]
pub type R = crate::R<TimingSpec>;
#[doc = "Register `TIMING` writer"]
pub type W = crate::W<TimingSpec>;
#[doc = "Ramp-up time\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ru {
    #[doc = "0: Legacy ramp-up time"]
    Legacy = 0,
    #[doc = "1: Fast ramp-up (default)"]
    Fast = 1,
}
impl From<Ru> for bool {
    #[inline(always)]
    fn from(variant: Ru) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RU` reader - Ramp-up time"]
pub type RuR = crate::BitReader<Ru>;
impl RuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ru {
        match self.bits {
            false => Ru::Legacy,
            true => Ru::Fast,
        }
    }
    #[doc = "Legacy ramp-up time"]
    #[inline(always)]
    pub fn is_legacy(&self) -> bool {
        *self == Ru::Legacy
    }
    #[doc = "Fast ramp-up (default)"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == Ru::Fast
    }
}
#[doc = "Field `RU` writer - Ramp-up time"]
pub type RuW<'a, REG> = crate::BitWriter<'a, REG, Ru>;
impl<'a, REG> RuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Legacy ramp-up time"]
    #[inline(always)]
    pub fn legacy(self) -> &'a mut crate::W<REG> {
        self.variant(Ru::Legacy)
    }
    #[doc = "Fast ramp-up (default)"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(Ru::Fast)
    }
}
impl R {
    #[doc = "Bit 0 - Ramp-up time"]
    #[inline(always)]
    pub fn ru(&self) -> RuR {
        RuR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ramp-up time"]
    #[inline(always)]
    #[must_use]
    pub fn ru(&mut self) -> RuW<TimingSpec> {
        RuW::new(self, 0)
    }
}
#[doc = "Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`timing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimingSpec;
impl crate::RegisterSpec for TimingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timing::R`](R) reader structure"]
impl crate::Readable for TimingSpec {}
#[doc = "`write(|w| ..)` method takes [`timing::W`](W) writer structure"]
impl crate::Writable for TimingSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMING to value 0x01"]
impl crate::Resettable for TimingSpec {
    const RESET_VALUE: u32 = 0x01;
}
