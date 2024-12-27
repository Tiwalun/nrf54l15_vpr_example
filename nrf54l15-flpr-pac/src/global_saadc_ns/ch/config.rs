#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Gain control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gain {
    #[doc = "0: 2"]
    Gain2 = 0,
    #[doc = "1: 1"]
    Gain1 = 1,
    #[doc = "2: 2/3"]
    Gain2_3 = 2,
    #[doc = "3: 2/4"]
    Gain2_4 = 3,
    #[doc = "4: 2/5"]
    Gain2_5 = 4,
    #[doc = "5: 2/6"]
    Gain2_6 = 5,
    #[doc = "6: 2/7"]
    Gain2_7 = 6,
    #[doc = "7: 2/8"]
    Gain2_8 = 7,
}
impl From<Gain> for u8 {
    #[inline(always)]
    fn from(variant: Gain) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gain {
    type Ux = u8;
}
impl crate::IsEnum for Gain {}
#[doc = "Field `GAIN` reader - Gain control"]
pub type GainR = crate::FieldReader<Gain>;
impl GainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gain {
        match self.bits {
            0 => Gain::Gain2,
            1 => Gain::Gain1,
            2 => Gain::Gain2_3,
            3 => Gain::Gain2_4,
            4 => Gain::Gain2_5,
            5 => Gain::Gain2_6,
            6 => Gain::Gain2_7,
            7 => Gain::Gain2_8,
            _ => unreachable!(),
        }
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn is_gain2(&self) -> bool {
        *self == Gain::Gain2
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn is_gain1(&self) -> bool {
        *self == Gain::Gain1
    }
    #[doc = "2/3"]
    #[inline(always)]
    pub fn is_gain2_3(&self) -> bool {
        *self == Gain::Gain2_3
    }
    #[doc = "2/4"]
    #[inline(always)]
    pub fn is_gain2_4(&self) -> bool {
        *self == Gain::Gain2_4
    }
    #[doc = "2/5"]
    #[inline(always)]
    pub fn is_gain2_5(&self) -> bool {
        *self == Gain::Gain2_5
    }
    #[doc = "2/6"]
    #[inline(always)]
    pub fn is_gain2_6(&self) -> bool {
        *self == Gain::Gain2_6
    }
    #[doc = "2/7"]
    #[inline(always)]
    pub fn is_gain2_7(&self) -> bool {
        *self == Gain::Gain2_7
    }
    #[doc = "2/8"]
    #[inline(always)]
    pub fn is_gain2_8(&self) -> bool {
        *self == Gain::Gain2_8
    }
}
#[doc = "Field `GAIN` writer - Gain control"]
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 3, Gain, crate::Safe>;
impl<'a, REG> GainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2"]
    #[inline(always)]
    pub fn gain2(self) -> &'a mut crate::W<REG> {
        self.variant(Gain::Gain2)
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn gain1(self) -> &'a mut crate::W<REG> {
        self.variant(Gain::Gain1)
    }
    #[doc = "2/3"]
    #[inline(always)]
    pub fn gain2_3(self) -> &'a mut crate::W<REG> {
        self.variant(Gain::Gain2_3)
    }
    #[doc = "2/4"]
    #[inline(always)]
    pub fn gain2_4(self) -> &'a mut crate::W<REG> {
        self.variant(Gain::Gain2_4)
    }
    #[doc = "2/5"]
    #[inline(always)]
    pub fn gain2_5(self) -> &'a mut crate::W<REG> {
        self.variant(Gain::Gain2_5)
    }
    #[doc = "2/6"]
    #[inline(always)]
    pub fn gain2_6(self) -> &'a mut crate::W<REG> {
        self.variant(Gain::Gain2_6)
    }
    #[doc = "2/7"]
    #[inline(always)]
    pub fn gain2_7(self) -> &'a mut crate::W<REG> {
        self.variant(Gain::Gain2_7)
    }
    #[doc = "2/8"]
    #[inline(always)]
    pub fn gain2_8(self) -> &'a mut crate::W<REG> {
        self.variant(Gain::Gain2_8)
    }
}
#[doc = "Enable burst mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Burst {
    #[doc = "0: Burst mode is disabled (normal operation)"]
    Disabled = 0,
    #[doc = "1: Burst mode is enabled. SAADC takes 2^OVERSAMPLE number of samples as fast as it can, and sends the average to Data RAM."]
    Enabled = 1,
}
impl From<Burst> for bool {
    #[inline(always)]
    fn from(variant: Burst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BURST` reader - Enable burst mode"]
pub type BurstR = crate::BitReader<Burst>;
impl BurstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Burst {
        match self.bits {
            false => Burst::Disabled,
            true => Burst::Enabled,
        }
    }
    #[doc = "Burst mode is disabled (normal operation)"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Burst::Disabled
    }
    #[doc = "Burst mode is enabled. SAADC takes 2^OVERSAMPLE number of samples as fast as it can, and sends the average to Data RAM."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Burst::Enabled
    }
}
#[doc = "Field `BURST` writer - Enable burst mode"]
pub type BurstW<'a, REG> = crate::BitWriter<'a, REG, Burst>;
impl<'a, REG> BurstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Burst mode is disabled (normal operation)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Burst::Disabled)
    }
    #[doc = "Burst mode is enabled. SAADC takes 2^OVERSAMPLE number of samples as fast as it can, and sends the average to Data RAM."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Burst::Enabled)
    }
}
#[doc = "Reference control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refsel {
    #[doc = "0: Internal reference (1.024 V)"]
    Internal = 0,
    #[doc = "1: External reference given at PADC_EXT_REF_1V2"]
    External = 1,
}
impl From<Refsel> for bool {
    #[inline(always)]
    fn from(variant: Refsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFSEL` reader - Reference control"]
pub type RefselR = crate::BitReader<Refsel>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refsel {
        match self.bits {
            false => Refsel::Internal,
            true => Refsel::External,
        }
    }
    #[doc = "Internal reference (1.024 V)"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == Refsel::Internal
    }
    #[doc = "External reference given at PADC_EXT_REF_1V2"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == Refsel::External
    }
}
#[doc = "Field `REFSEL` writer - Reference control"]
pub type RefselW<'a, REG> = crate::BitWriter<'a, REG, Refsel>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal reference (1.024 V)"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Internal)
    }
    #[doc = "External reference given at PADC_EXT_REF_1V2"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::External)
    }
}
#[doc = "Enable differential mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Single ended, PSELN will be ignored, negative input to ADC shorted to GND"]
    Se = 0,
    #[doc = "1: Differential"]
    Diff = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Enable differential mode"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Se,
            true => Mode::Diff,
        }
    }
    #[doc = "Single ended, PSELN will be ignored, negative input to ADC shorted to GND"]
    #[inline(always)]
    pub fn is_se(&self) -> bool {
        *self == Mode::Se
    }
    #[doc = "Differential"]
    #[inline(always)]
    pub fn is_diff(&self) -> bool {
        *self == Mode::Diff
    }
}
#[doc = "Field `MODE` writer - Enable differential mode"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single ended, PSELN will be ignored, negative input to ADC shorted to GND"]
    #[inline(always)]
    pub fn se(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Se)
    }
    #[doc = "Differential"]
    #[inline(always)]
    pub fn diff(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Diff)
    }
}
#[doc = "Field `TACQ` reader - Acquisition time, the time the ADC uses to sample the input voltage. Resulting acquistion time is ((TACQ+1) x 125 ns)"]
pub type TacqR = crate::FieldReader<u16>;
#[doc = "Field `TACQ` writer - Acquisition time, the time the ADC uses to sample the input voltage. Resulting acquistion time is ((TACQ+1) x 125 ns)"]
pub type TacqW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TCONV` reader - Conversion time. Resulting conversion time is ((TCONV+1) x 250 ns)"]
pub type TconvR = crate::FieldReader;
#[doc = "Field `TCONV` writer - Conversion time. Resulting conversion time is ((TCONV+1) x 250 ns)"]
pub type TconvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 8:10 - Gain control"]
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Enable burst mode"]
    #[inline(always)]
    pub fn burst(&self) -> BurstR {
        BurstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reference control"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable differential mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:24 - Acquisition time, the time the ADC uses to sample the input voltage. Resulting acquistion time is ((TACQ+1) x 125 ns)"]
    #[inline(always)]
    pub fn tacq(&self) -> TacqR {
        TacqR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 28:30 - Conversion time. Resulting conversion time is ((TCONV+1) x 250 ns)"]
    #[inline(always)]
    pub fn tconv(&self) -> TconvR {
        TconvR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Gain control"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GainW<ConfigSpec> {
        GainW::new(self, 8)
    }
    #[doc = "Bit 11 - Enable burst mode"]
    #[inline(always)]
    #[must_use]
    pub fn burst(&mut self) -> BurstW<ConfigSpec> {
        BurstW::new(self, 11)
    }
    #[doc = "Bit 12 - Reference control"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> RefselW<ConfigSpec> {
        RefselW::new(self, 12)
    }
    #[doc = "Bit 15 - Enable differential mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<ConfigSpec> {
        ModeW::new(self, 15)
    }
    #[doc = "Bits 16:24 - Acquisition time, the time the ADC uses to sample the input voltage. Resulting acquistion time is ((TACQ+1) x 125 ns)"]
    #[inline(always)]
    #[must_use]
    pub fn tacq(&mut self) -> TacqW<ConfigSpec> {
        TacqW::new(self, 16)
    }
    #[doc = "Bits 28:30 - Conversion time. Resulting conversion time is ((TCONV+1) x 250 ns)"]
    #[inline(always)]
    #[must_use]
    pub fn tconv(&mut self) -> TconvW<ConfigSpec> {
        TconvW::new(self, 28)
    }
}
#[doc = "Description cluster: Input configuration for CH\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CONFIG to value 0x0002_0000"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x0002_0000;
}
