#[doc = "Register `RESET` writer"]
pub type W = crate::W<ResetSpec>;
#[doc = "Reset request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reset {
    #[doc = "0: No reset is generated"]
    NoReset = 0,
    #[doc = "1: Perform a device soft reset"]
    SoftReset = 1,
    #[doc = "2: Perform a device hard reset"]
    HardReset = 2,
    #[doc = "4: Perform a device pin reset"]
    PinReset = 4,
}
impl From<Reset> for u8 {
    #[inline(always)]
    fn from(variant: Reset) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reset {
    type Ux = u8;
}
impl crate::IsEnum for Reset {}
#[doc = "Field `RESET` writer - Reset request"]
pub type ResetW<'a, REG> = crate::FieldWriter<'a, REG, 3, Reset>;
impl<'a, REG> ResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No reset is generated"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::NoReset)
    }
    #[doc = "Perform a device soft reset"]
    #[inline(always)]
    pub fn soft_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::SoftReset)
    }
    #[doc = "Perform a device hard reset"]
    #[inline(always)]
    pub fn hard_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::HardReset)
    }
    #[doc = "Perform a device pin reset"]
    #[inline(always)]
    pub fn pin_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::PinReset)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reset request"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<ResetSpec> {
        ResetW::new(self, 0)
    }
}
#[doc = "System reset request.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetSpec;
impl crate::RegisterSpec for ResetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`reset::W`](W) writer structure"]
impl crate::Writable for ResetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESET to value 0"]
impl crate::Resettable for ResetSpec {
    const RESET_VALUE: u32 = 0;
}
