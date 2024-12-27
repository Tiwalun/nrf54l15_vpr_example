#[doc = "Register `READYNEXT` reader"]
pub type R = crate::R<ReadynextSpec>;
#[doc = "RRAMC can accept a new write operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Readynext {
    #[doc = "0: RRAMC cannot accept any write operation now"]
    Busy = 0,
    #[doc = "1: RRAMC is ready to accept a new write operation"]
    Ready = 1,
}
impl From<Readynext> for bool {
    #[inline(always)]
    fn from(variant: Readynext) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READYNEXT` reader - RRAMC can accept a new write operation"]
pub type ReadynextR = crate::BitReader<Readynext>;
impl ReadynextR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Readynext {
        match self.bits {
            false => Readynext::Busy,
            true => Readynext::Ready,
        }
    }
    #[doc = "RRAMC cannot accept any write operation now"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Readynext::Busy
    }
    #[doc = "RRAMC is ready to accept a new write operation"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Readynext::Ready
    }
}
impl R {
    #[doc = "Bit 0 - RRAMC can accept a new write operation"]
    #[inline(always)]
    pub fn readynext(&self) -> ReadynextR {
        ReadynextR::new((self.bits & 1) != 0)
    }
}
#[doc = "Ready next flag\n\nYou can [`read`](crate::Reg::read) this register and get [`readynext::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadynextSpec;
impl crate::RegisterSpec for ReadynextSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`readynext::R`](R) reader structure"]
impl crate::Readable for ReadynextSpec {}
#[doc = "`reset()` method sets READYNEXT to value 0"]
impl crate::Resettable for ReadynextSpec {
    const RESET_VALUE: u32 = 0;
}
