#[doc = "Register `FRAMETIMEOUT` reader"]
pub type R = crate::R<FrametimeoutSpec>;
#[doc = "Register `FRAMETIMEOUT` writer"]
pub type W = crate::W<FrametimeoutSpec>;
#[doc = "Field `COUNTERTOP` reader - Number of UARTE bits before timeout."]
pub type CountertopR = crate::FieldReader<u16>;
#[doc = "Field `COUNTERTOP` writer - Number of UARTE bits before timeout."]
pub type CountertopW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Number of UARTE bits before timeout."]
    #[inline(always)]
    pub fn countertop(&self) -> CountertopR {
        CountertopR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Number of UARTE bits before timeout."]
    #[inline(always)]
    #[must_use]
    pub fn countertop(&mut self) -> CountertopW<FrametimeoutSpec> {
        CountertopW::new(self, 0)
    }
}
#[doc = "Set the number of UARTE bits to count before triggering packet timeout.\n\nYou can [`read`](crate::Reg::read) this register and get [`frametimeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frametimeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrametimeoutSpec;
impl crate::RegisterSpec for FrametimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frametimeout::R`](R) reader structure"]
impl crate::Readable for FrametimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`frametimeout::W`](W) writer structure"]
impl crate::Writable for FrametimeoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRAMETIMEOUT to value 0x10"]
impl crate::Resettable for FrametimeoutSpec {
    const RESET_VALUE: u32 = 0x10;
}
