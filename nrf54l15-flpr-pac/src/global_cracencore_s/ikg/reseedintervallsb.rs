#[doc = "Register `RESEEDINTERVALLSB` reader"]
pub type R = crate::R<ReseedintervallsbSpec>;
#[doc = "Register `RESEEDINTERVALLSB` writer"]
pub type W = crate::W<ReseedintervallsbSpec>;
#[doc = "Field `RESEEDINTERVALLSB` reader - Reseed Interval LSB."]
pub type ReseedintervallsbR = crate::FieldReader<u32>;
#[doc = "Field `RESEEDINTERVALLSB` writer - Reseed Interval LSB."]
pub type ReseedintervallsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reseed Interval LSB."]
    #[inline(always)]
    pub fn reseedintervallsb(&self) -> ReseedintervallsbR {
        ReseedintervallsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reseed Interval LSB."]
    #[inline(always)]
    #[must_use]
    pub fn reseedintervallsb(&mut self) -> ReseedintervallsbW<ReseedintervallsbSpec> {
        ReseedintervallsbW::new(self, 0)
    }
}
#[doc = "Reseed Interval LSB register.\n\nYou can [`read`](crate::Reg::read) this register and get [`reseedintervallsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reseedintervallsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReseedintervallsbSpec;
impl crate::RegisterSpec for ReseedintervallsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reseedintervallsb::R`](R) reader structure"]
impl crate::Readable for ReseedintervallsbSpec {}
#[doc = "`write(|w| ..)` method takes [`reseedintervallsb::W`](W) writer structure"]
impl crate::Writable for ReseedintervallsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESEEDINTERVALLSB to value 0x8000_0000"]
impl crate::Resettable for ReseedintervallsbSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
