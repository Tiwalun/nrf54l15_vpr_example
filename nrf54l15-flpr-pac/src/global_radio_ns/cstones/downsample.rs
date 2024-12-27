#[doc = "Register `DOWNSAMPLE` reader"]
pub type R = crate::R<DownsampleSpec>;
#[doc = "Register `DOWNSAMPLE` writer"]
pub type W = crate::W<DownsampleSpec>;
#[doc = "Turn on/off down sample of input IQ-signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enablefilter {
    #[doc = "0: Disable filter"]
    Off = 0,
    #[doc = "1: Enable filter"]
    On = 1,
}
impl From<Enablefilter> for bool {
    #[inline(always)]
    fn from(variant: Enablefilter) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLEFILTER` reader - Turn on/off down sample of input IQ-signals"]
pub type EnablefilterR = crate::BitReader<Enablefilter>;
impl EnablefilterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enablefilter {
        match self.bits {
            false => Enablefilter::Off,
            true => Enablefilter::On,
        }
    }
    #[doc = "Disable filter"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Enablefilter::Off
    }
    #[doc = "Enable filter"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Enablefilter::On
    }
}
#[doc = "Field `ENABLEFILTER` writer - Turn on/off down sample of input IQ-signals"]
pub type EnablefilterW<'a, REG> = crate::BitWriter<'a, REG, Enablefilter>;
impl<'a, REG> EnablefilterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable filter"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Enablefilter::Off)
    }
    #[doc = "Enable filter"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Enablefilter::On)
    }
}
#[doc = "Indicating if BLE1M or BLE2M is used\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rate {
    #[doc = "0: Radio mode BLE1M is used"]
    Ble1m = 0,
    #[doc = "1: Radio mode BLE2M is used"]
    Ble2m = 1,
}
impl From<Rate> for bool {
    #[inline(always)]
    fn from(variant: Rate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RATE` reader - Indicating if BLE1M or BLE2M is used"]
pub type RateR = crate::BitReader<Rate>;
impl RateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rate {
        match self.bits {
            false => Rate::Ble1m,
            true => Rate::Ble2m,
        }
    }
    #[doc = "Radio mode BLE1M is used"]
    #[inline(always)]
    pub fn is_ble1m(&self) -> bool {
        *self == Rate::Ble1m
    }
    #[doc = "Radio mode BLE2M is used"]
    #[inline(always)]
    pub fn is_ble2m(&self) -> bool {
        *self == Rate::Ble2m
    }
}
#[doc = "Field `RATE` writer - Indicating if BLE1M or BLE2M is used"]
pub type RateW<'a, REG> = crate::BitWriter<'a, REG, Rate>;
impl<'a, REG> RateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Radio mode BLE1M is used"]
    #[inline(always)]
    pub fn ble1m(self) -> &'a mut crate::W<REG> {
        self.variant(Rate::Ble1m)
    }
    #[doc = "Radio mode BLE2M is used"]
    #[inline(always)]
    pub fn ble2m(self) -> &'a mut crate::W<REG> {
        self.variant(Rate::Ble2m)
    }
}
impl R {
    #[doc = "Bit 0 - Turn on/off down sample of input IQ-signals"]
    #[inline(always)]
    pub fn enablefilter(&self) -> EnablefilterR {
        EnablefilterR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicating if BLE1M or BLE2M is used"]
    #[inline(always)]
    pub fn rate(&self) -> RateR {
        RateR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Turn on/off down sample of input IQ-signals"]
    #[inline(always)]
    #[must_use]
    pub fn enablefilter(&mut self) -> EnablefilterW<DownsampleSpec> {
        EnablefilterW::new(self, 0)
    }
    #[doc = "Bit 1 - Indicating if BLE1M or BLE2M is used"]
    #[inline(always)]
    #[must_use]
    pub fn rate(&mut self) -> RateW<DownsampleSpec> {
        RateW::new(self, 1)
    }
}
#[doc = "Turn on/off down sample of input IQ-signals\n\nYou can [`read`](crate::Reg::read) this register and get [`downsample::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`downsample::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DownsampleSpec;
impl crate::RegisterSpec for DownsampleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`downsample::R`](R) reader structure"]
impl crate::Readable for DownsampleSpec {}
#[doc = "`write(|w| ..)` method takes [`downsample::W`](W) writer structure"]
impl crate::Writable for DownsampleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOWNSAMPLE to value 0"]
impl crate::Resettable for DownsampleSpec {
    const RESET_VALUE: u32 = 0;
}
