#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Error detection status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    #[doc = "0: No error detected."]
    NoError = 0,
    #[doc = "1: Error detected."]
    Error = 1,
}
impl From<Error> for bool {
    #[inline(always)]
    fn from(variant: Error) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Error detection status."]
pub type ErrorR = crate::BitReader<Error>;
impl ErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Error {
        match self.bits {
            false => Error::NoError,
            true => Error::Error,
        }
    }
    #[doc = "No error detected."]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Error::NoError
    }
    #[doc = "Error detected."]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Error::Error
    }
}
#[doc = "Field `ERROR` writer - Error detection status."]
pub type ErrorW<'a, REG> = crate::BitWriter1C<'a, REG, Error>;
impl<'a, REG> ErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error detected."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(Error::NoError)
    }
    #[doc = "Error detected."]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Error::Error)
    }
}
impl R {
    #[doc = "Bit 0 - Error detection status."]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error detection status."]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<StatusSpec> {
        ErrorW::new(self, 0)
    }
}
#[doc = "Status register for slow domain glitch detectors enable signal.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
