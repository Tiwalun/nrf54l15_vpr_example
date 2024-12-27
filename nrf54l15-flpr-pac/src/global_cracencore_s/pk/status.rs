#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `ERRORFLAGS` reader - These bits indicate an error condition."]
pub type ErrorflagsR = crate::FieldReader<u16>;
#[doc = "Field `PKBUSY` reader - This bit reflects the BUSY output value."]
pub type PkbusyR = crate::BitReader;
#[doc = "Field `INTRPTSTATUS` reader - This bit reflects the IRQ output value."]
pub type IntrptstatusR = crate::BitReader;
#[doc = "Field `FAILPTR` reader - These bits indicate which data location generated the error flag."]
pub type FailptrR = crate::FieldReader;
impl R {
    #[doc = "Bits 4:15 - These bits indicate an error condition."]
    #[inline(always)]
    pub fn errorflags(&self) -> ErrorflagsR {
        ErrorflagsR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - This bit reflects the BUSY output value."]
    #[inline(always)]
    pub fn pkbusy(&self) -> PkbusyR {
        PkbusyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit reflects the IRQ output value."]
    #[inline(always)]
    pub fn intrptstatus(&self) -> IntrptstatusR {
        IntrptstatusR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:28 - These bits indicate which data location generated the error flag."]
    #[inline(always)]
    pub fn failptr(&self) -> FailptrR {
        FailptrR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {}
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
