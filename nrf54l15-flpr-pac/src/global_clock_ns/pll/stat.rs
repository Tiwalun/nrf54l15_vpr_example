#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "PLL state (Running between START task and STOPPED event)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    #[doc = "0: PLL is not running"]
    NotRunning = 0,
    #[doc = "1: PLL is running"]
    Running = 1,
}
impl From<State> for bool {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATE` reader - PLL state (Running between START task and STOPPED event)"]
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
    #[doc = "PLL is not running"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == State::NotRunning
    }
    #[doc = "PLL is running"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == State::Running
    }
}
impl R {
    #[doc = "Bit 16 - PLL state (Running between START task and STOPPED event)"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Which PLL settings were selected when triggering START task\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
