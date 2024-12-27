#[doc = "Register `CONSTLATSTAT` reader"]
pub type R = crate::R<ConstlatstatSpec>;
#[doc = "Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: Constant latency disabled"]
    Disable = 0,
    #[doc = "1: Constant latency enabled"]
    Enable = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - Status"]
pub type StatusR = crate::BitReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status {
        match self.bits {
            false => Status::Disable,
            true => Status::Enable,
        }
    }
    #[doc = "Constant latency disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Status::Disable
    }
    #[doc = "Constant latency enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Status::Enable
    }
}
impl R {
    #[doc = "Bit 0 - Status"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status of constant latency\n\nYou can [`read`](crate::Reg::read) this register and get [`constlatstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConstlatstatSpec;
impl crate::RegisterSpec for ConstlatstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`constlatstat::R`](R) reader structure"]
impl crate::Readable for ConstlatstatSpec {}
#[doc = "`reset()` method sets CONSTLATSTAT to value 0"]
impl crate::Resettable for ConstlatstatSpec {
    const RESET_VALUE: u32 = 0;
}
