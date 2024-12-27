#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between event RTCOMPARE and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcompareClear {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<RtcompareClear> for bool {
    #[inline(always)]
    fn from(variant: RtcompareClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCOMPARE_CLEAR` reader - Shortcut between event RTCOMPARE and task CLEAR"]
pub type RtcompareClearR = crate::BitReader<RtcompareClear>;
impl RtcompareClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcompareClear {
        match self.bits {
            false => RtcompareClear::Disabled,
            true => RtcompareClear::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RtcompareClear::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RtcompareClear::Enabled
    }
}
#[doc = "Field `RTCOMPARE_CLEAR` writer - Shortcut between event RTCOMPARE and task CLEAR"]
pub type RtcompareClearW<'a, REG> = crate::BitWriter<'a, REG, RtcompareClear>;
impl<'a, REG> RtcompareClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RtcompareClear::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RtcompareClear::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event RTCOMPARE and task CLEAR"]
    #[inline(always)]
    pub fn rtcompare_clear(&self) -> RtcompareClearR {
        RtcompareClearR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event RTCOMPARE and task CLEAR"]
    #[inline(always)]
    pub fn rtcompare_clear(&mut self) -> RtcompareClearW<ShortsSpec> {
        RtcompareClearW::new(self, 0)
    }
}
#[doc = "Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShortsSpec;
impl crate::RegisterSpec for ShortsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shorts::R`](R) reader structure"]
impl crate::Readable for ShortsSpec {}
#[doc = "`write(|w| ..)` method takes [`shorts::W`](W) writer structure"]
impl crate::Writable for ShortsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for ShortsSpec {
    const RESET_VALUE: u32 = 0;
}
