#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `TESTDATABUSY` reader - High when data written to TestData register is being processed."]
pub type TestdatabusyR = crate::BitReader;
#[doc = "State of the control FSM:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    #[doc = "0: Reset"]
    Reset = 0,
    #[doc = "1: Startup"]
    Startup = 1,
    #[doc = "2: Idle (Rings On)"]
    Idleron = 2,
    #[doc = "3: Idle (Rings Off)"]
    Idleroff = 3,
    #[doc = "4: Fill FIFO"]
    Fillfifo = 4,
    #[doc = "5: Error"]
    Error = 5,
}
impl From<State> for u8 {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for State {
    type Ux = u8;
}
impl crate::IsEnum for State {}
#[doc = "Field `STATE` reader - State of the control FSM:"]
pub type StateR = crate::FieldReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<State> {
        match self.bits {
            0 => Some(State::Reset),
            1 => Some(State::Startup),
            2 => Some(State::Idleron),
            3 => Some(State::Idleroff),
            4 => Some(State::Fillfifo),
            5 => Some(State::Error),
            _ => None,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == State::Reset
    }
    #[doc = "Startup"]
    #[inline(always)]
    pub fn is_startup(&self) -> bool {
        *self == State::Startup
    }
    #[doc = "Idle (Rings On)"]
    #[inline(always)]
    pub fn is_idleron(&self) -> bool {
        *self == State::Idleron
    }
    #[doc = "Idle (Rings Off)"]
    #[inline(always)]
    pub fn is_idleroff(&self) -> bool {
        *self == State::Idleroff
    }
    #[doc = "Fill FIFO"]
    #[inline(always)]
    pub fn is_fillfifo(&self) -> bool {
        *self == State::Fillfifo
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == State::Error
    }
}
#[doc = "Field `REPFAIL` reader - NIST-800-90B repetition Count Test interrupt status."]
pub type RepfailR = crate::BitReader;
#[doc = "Field `REPFAIL` writer - NIST-800-90B repetition Count Test interrupt status."]
pub type RepfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROPFAIL` reader - NIST-800-90B adaptive Proportion Test (1024-sample window) interrupt status."]
pub type PropfailR = crate::BitReader;
#[doc = "Field `PROPFAIL` writer - NIST-800-90B adaptive Proportion Test (1024-sample window) interrupt status."]
pub type PropfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULLINT` reader - FIFO full status."]
pub type FullintR = crate::BitReader;
#[doc = "Field `FULLINT` writer - FIFO full status."]
pub type FullintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREINT` reader - AIS31 preliminary noise alarm interrupt status."]
pub type PreintR = crate::BitReader;
#[doc = "Field `PREINT` writer - AIS31 preliminary noise alarm interrupt status."]
pub type PreintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALMINT` reader - AIS31 noise alarm interrupt status."]
pub type AlmintR = crate::BitReader;
#[doc = "Field `ALMINT` writer - AIS31 noise alarm interrupt status."]
pub type AlmintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTUPFAIL` reader - Start-up test failure."]
pub type StartupfailR = crate::BitReader;
#[doc = "Field `FIFOACCFAIL` reader - Set when a FIFO data read is performed while the NDRNG is disabled AND has its FIFO empty (FIFOLevel = 0)."]
pub type FifoaccfailR = crate::BitReader;
#[doc = "Field `FIFOACCFAIL` writer - Set when a FIFO data read is performed while the NDRNG is disabled AND has its FIFO empty (FIFOLevel = 0)."]
pub type FifoaccfailW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - High when data written to TestData register is being processed."]
    #[inline(always)]
    pub fn testdatabusy(&self) -> TestdatabusyR {
        TestdatabusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - State of the control FSM:"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - NIST-800-90B repetition Count Test interrupt status."]
    #[inline(always)]
    pub fn repfail(&self) -> RepfailR {
        RepfailR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NIST-800-90B adaptive Proportion Test (1024-sample window) interrupt status."]
    #[inline(always)]
    pub fn propfail(&self) -> PropfailR {
        PropfailR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - FIFO full status."]
    #[inline(always)]
    pub fn fullint(&self) -> FullintR {
        FullintR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AIS31 preliminary noise alarm interrupt status."]
    #[inline(always)]
    pub fn preint(&self) -> PreintR {
        PreintR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AIS31 noise alarm interrupt status."]
    #[inline(always)]
    pub fn almint(&self) -> AlmintR {
        AlmintR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Start-up test failure."]
    #[inline(always)]
    pub fn startupfail(&self) -> StartupfailR {
        StartupfailR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set when a FIFO data read is performed while the NDRNG is disabled AND has its FIFO empty (FIFOLevel = 0)."]
    #[inline(always)]
    pub fn fifoaccfail(&self) -> FifoaccfailR {
        FifoaccfailR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - NIST-800-90B repetition Count Test interrupt status."]
    #[inline(always)]
    pub fn repfail(&mut self) -> RepfailW<StatusSpec> {
        RepfailW::new(self, 4)
    }
    #[doc = "Bit 5 - NIST-800-90B adaptive Proportion Test (1024-sample window) interrupt status."]
    #[inline(always)]
    pub fn propfail(&mut self) -> PropfailW<StatusSpec> {
        PropfailW::new(self, 5)
    }
    #[doc = "Bit 7 - FIFO full status."]
    #[inline(always)]
    pub fn fullint(&mut self) -> FullintW<StatusSpec> {
        FullintW::new(self, 7)
    }
    #[doc = "Bit 8 - AIS31 preliminary noise alarm interrupt status."]
    #[inline(always)]
    pub fn preint(&mut self) -> PreintW<StatusSpec> {
        PreintW::new(self, 8)
    }
    #[doc = "Bit 9 - AIS31 noise alarm interrupt status."]
    #[inline(always)]
    pub fn almint(&mut self) -> AlmintW<StatusSpec> {
        AlmintW::new(self, 9)
    }
    #[doc = "Bit 11 - Set when a FIFO data read is performed while the NDRNG is disabled AND has its FIFO empty (FIFOLevel = 0)."]
    #[inline(always)]
    pub fn fifoaccfail(&mut self) -> FifoaccfailW<StatusSpec> {
        FifoaccfailW::new(self, 11)
    }
}
#[doc = "Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
