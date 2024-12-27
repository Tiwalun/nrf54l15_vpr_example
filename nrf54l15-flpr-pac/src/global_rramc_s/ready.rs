#[doc = "Register `READY` reader"]
pub type R = crate::R<ReadySpec>;
#[doc = "RRAMC is ready or busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ready {
    #[doc = "0: RRAMC is busy"]
    Busy = 0,
    #[doc = "1: The current RRAMC operation is completed and RRAMC is ready"]
    Ready = 1,
}
impl From<Ready> for bool {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - RRAMC is ready or busy"]
pub type ReadyR = crate::BitReader<Ready>;
impl ReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ready {
        match self.bits {
            false => Ready::Busy,
            true => Ready::Ready,
        }
    }
    #[doc = "RRAMC is busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Ready::Busy
    }
    #[doc = "The current RRAMC operation is completed and RRAMC is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Ready::Ready
    }
}
impl R {
    #[doc = "Bit 0 - RRAMC is ready or busy"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
}
#[doc = "RRAMC ready status\n\nYou can [`read`](crate::Reg::read) this register and get [`ready::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadySpec;
impl crate::RegisterSpec for ReadySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ready::R`](R) reader structure"]
impl crate::Readable for ReadySpec {}
#[doc = "`reset()` method sets READY to value 0"]
impl crate::Resettable for ReadySpec {
    const RESET_VALUE: u32 = 0;
}
