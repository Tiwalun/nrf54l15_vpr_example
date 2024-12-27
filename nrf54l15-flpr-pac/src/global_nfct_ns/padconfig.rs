#[doc = "Register `PADCONFIG` reader"]
pub type R = crate::R<PadconfigSpec>;
#[doc = "Register `PADCONFIG` writer"]
pub type W = crate::W<PadconfigSpec>;
#[doc = "Enable NFC pads\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: NFC pads are used as GPIO pins"]
    Disabled = 0,
    #[doc = "1: The NFC pads are configured as NFC antenna pins"]
    Enabled = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable NFC pads"]
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
    #[doc = "NFC pads are used as GPIO pins"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable::Disabled
    }
    #[doc = "The NFC pads are configured as NFC antenna pins"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable::Enabled
    }
}
#[doc = "Field `ENABLE` writer - Enable NFC pads"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NFC pads are used as GPIO pins"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disabled)
    }
    #[doc = "The NFC pads are configured as NFC antenna pins"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable NFC pads"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable NFC pads"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<PadconfigSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "NFC pad configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`padconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadconfigSpec;
impl crate::RegisterSpec for PadconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padconfig::R`](R) reader structure"]
impl crate::Readable for PadconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`padconfig::W`](W) writer structure"]
impl crate::Writable for PadconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADCONFIG to value 0x01"]
impl crate::Resettable for PadconfigSpec {
    const RESET_VALUE: u32 = 0x01;
}
