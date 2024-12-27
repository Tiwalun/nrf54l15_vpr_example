#[doc = "Register `OTP[%s]` reader"]
pub type R = crate::R<OtpSpec>;
#[doc = "Register `OTP[%s]` writer"]
pub type W = crate::W<OtpSpec>;
#[doc = "Field `OTP` reader - OTP word"]
pub type OtpR = crate::FieldReader<u32>;
#[doc = "Field `OTP` writer - OTP word"]
pub type OtpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP word"]
    #[inline(always)]
    pub fn otp(&self) -> OtpR {
        OtpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP word"]
    #[inline(always)]
    pub fn otp(&mut self) -> OtpW<OtpSpec> {
        OtpW::new(self, 0)
    }
}
#[doc = "Description collection: One time programmable memory\n\nYou can [`read`](crate::Reg::read) this register and get [`otp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpSpec;
impl crate::RegisterSpec for OtpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp::R`](R) reader structure"]
impl crate::Readable for OtpSpec {}
#[doc = "`write(|w| ..)` method takes [`otp::W`](W) writer structure"]
impl crate::Writable for OtpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTP[%s]
to value 0xffff_ffff"]
impl crate::Resettable for OtpSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
