#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Value of LFCLK.SRCCOPY register when LFCLKSTARTED event was triggered\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "0: 32.768 kHz RC oscillator"]
    Lfrc = 0,
    #[doc = "1: 32.768 kHz crystal oscillator"]
    Lfxo = 1,
    #[doc = "2: 32.768 kHz synthesized from HFCLK"]
    Lfsynt = 2,
}
impl From<Src> for u8 {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src {
    type Ux = u8;
}
impl crate::IsEnum for Src {}
#[doc = "Field `SRC` reader - Value of LFCLK.SRCCOPY register when LFCLKSTARTED event was triggered"]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src> {
        match self.bits {
            0 => Some(Src::Lfrc),
            1 => Some(Src::Lfxo),
            2 => Some(Src::Lfsynt),
            _ => None,
        }
    }
    #[doc = "32.768 kHz RC oscillator"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == Src::Lfrc
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Src::Lfxo
    }
    #[doc = "32.768 kHz synthesized from HFCLK"]
    #[inline(always)]
    pub fn is_lfsynt(&self) -> bool {
        *self == Src::Lfsynt
    }
}
#[doc = "ALWAYSRUN activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alwaysrunning {
    #[doc = "0: Automatic clock control enabled"]
    NotRunning = 0,
    #[doc = "1: Oscillator is always running"]
    Running = 1,
}
impl From<Alwaysrunning> for bool {
    #[inline(always)]
    fn from(variant: Alwaysrunning) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALWAYSRUNNING` reader - ALWAYSRUN activated"]
pub type AlwaysrunningR = crate::BitReader<Alwaysrunning>;
impl AlwaysrunningR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alwaysrunning {
        match self.bits {
            false => Alwaysrunning::NotRunning,
            true => Alwaysrunning::Running,
        }
    }
    #[doc = "Automatic clock control enabled"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == Alwaysrunning::NotRunning
    }
    #[doc = "Oscillator is always running"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == Alwaysrunning::Running
    }
}
#[doc = "LFCLK state (Running between START task and STOPPED event)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    #[doc = "0: LFCLK not running"]
    NotRunning = 0,
    #[doc = "1: LFCLK running"]
    Running = 1,
}
impl From<State> for bool {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATE` reader - LFCLK state (Running between START task and STOPPED event)"]
pub type StateR = crate::BitReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> State {
        match self.bits {
            false => State::NotRunning,
            true => State::Running,
        }
    }
    #[doc = "LFCLK not running"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == State::NotRunning
    }
    #[doc = "LFCLK running"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == State::Running
    }
}
impl R {
    #[doc = "Bits 0:1 - Value of LFCLK.SRCCOPY register when LFCLKSTARTED event was triggered"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - ALWAYSRUN activated"]
    #[inline(always)]
    pub fn alwaysrunning(&self) -> AlwaysrunningR {
        AlwaysrunningR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - LFCLK state (Running between START task and STOPPED event)"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Copy of LFCLK.SRCCOPY register, set when LFCLKSTARTED event is triggered.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
