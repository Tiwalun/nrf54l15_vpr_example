#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `FETCHBUSY` reader - "]
pub type FetchbusyR = crate::BitReader;
#[doc = "Field `PUSHBUSY` reader - "]
pub type PushbusyR = crate::BitReader;
#[doc = "Field `FETCHNOTEMPTY` reader - "]
pub type FetchnotemptyR = crate::BitReader;
#[doc = "Field `PUSHWAITINGFIFO` reader - "]
pub type PushwaitingfifoR = crate::BitReader;
#[doc = "Field `SOFTRSTBUSY` reader - "]
pub type SoftrstbusyR = crate::BitReader;
#[doc = "Field `PUSHNBDATA` reader - "]
pub type PushnbdataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fetchbusy(&self) -> FetchbusyR {
        FetchbusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pushbusy(&self) -> PushbusyR {
        PushbusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fetchnotempty(&self) -> FetchnotemptyR {
        FetchnotemptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pushwaitingfifo(&self) -> PushwaitingfifoR {
        PushwaitingfifoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn softrstbusy(&self) -> SoftrstbusyR {
        SoftrstbusyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn pushnbdata(&self) -> PushnbdataR {
        PushnbdataR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
