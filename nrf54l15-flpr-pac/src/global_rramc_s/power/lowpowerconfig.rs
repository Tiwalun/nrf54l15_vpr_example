#[doc = "Register `LOWPOWERCONFIG` reader"]
pub type R = crate::R<LowpowerconfigSpec>;
#[doc = "Register `LOWPOWERCONFIG` writer"]
pub type W = crate::W<LowpowerconfigSpec>;
#[doc = "RRAM low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: The RRAM goes into power down mode"]
    PowerDown = 0,
    #[doc = "1: The RRAM automatically goes into standby mode while the RRAM is not being accessed"]
    Standby = 1,
    #[doc = "2: The RRAM goes into NAP mode"]
    Nap = 2,
    #[doc = "3: The RRAM is powered Off"]
    PowerOff = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - RRAM low power mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::PowerDown,
            1 => Mode::Standby,
            2 => Mode::Nap,
            3 => Mode::PowerOff,
            _ => unreachable!(),
        }
    }
    #[doc = "The RRAM goes into power down mode"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == Mode::PowerDown
    }
    #[doc = "The RRAM automatically goes into standby mode while the RRAM is not being accessed"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == Mode::Standby
    }
    #[doc = "The RRAM goes into NAP mode"]
    #[inline(always)]
    pub fn is_nap(&self) -> bool {
        *self == Mode::Nap
    }
    #[doc = "The RRAM is powered Off"]
    #[inline(always)]
    pub fn is_power_off(&self) -> bool {
        *self == Mode::PowerOff
    }
}
#[doc = "Field `MODE` writer - RRAM low power mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The RRAM goes into power down mode"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::PowerDown)
    }
    #[doc = "The RRAM automatically goes into standby mode while the RRAM is not being accessed"]
    #[inline(always)]
    pub fn standby(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Standby)
    }
    #[doc = "The RRAM goes into NAP mode"]
    #[inline(always)]
    pub fn nap(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Nap)
    }
    #[doc = "The RRAM is powered Off"]
    #[inline(always)]
    pub fn power_off(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::PowerOff)
    }
}
impl R {
    #[doc = "Bits 0:1 - RRAM low power mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RRAM low power mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<LowpowerconfigSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "Low power mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`lowpowerconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lowpowerconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LowpowerconfigSpec;
impl crate::RegisterSpec for LowpowerconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lowpowerconfig::R`](R) reader structure"]
impl crate::Readable for LowpowerconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`lowpowerconfig::W`](W) writer structure"]
impl crate::Writable for LowpowerconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOWPOWERCONFIG to value 0"]
impl crate::Resettable for LowpowerconfigSpec {
    const RESET_VALUE: u32 = 0;
}
