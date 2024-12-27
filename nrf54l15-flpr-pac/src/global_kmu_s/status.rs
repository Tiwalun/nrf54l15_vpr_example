#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "KMU status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: KMU is ready for new operation"]
    Ready = 0,
    #[doc = "1: KMU is busy, an operation is in progress"]
    Busy = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - KMU status"]
pub type StatusR = crate::BitReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status {
        match self.bits {
            false => Status::Ready,
            true => Status::Busy,
        }
    }
    #[doc = "KMU is ready for new operation"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Status::Ready
    }
    #[doc = "KMU is busy, an operation is in progress"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Status::Busy
    }
}
impl R {
    #[doc = "Bit 0 - KMU status"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "KMU status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
