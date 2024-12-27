#[doc = "Register `RESEEDINTERVALMSB` reader"]
pub type R = crate::R<ReseedintervalmsbSpec>;
#[doc = "Register `RESEEDINTERVALMSB` writer"]
pub type W = crate::W<ReseedintervalmsbSpec>;
#[doc = "Field `RESEEDINTERVALMSB` reader - Reseed Interval MSB."]
pub type ReseedintervalmsbR = crate::FieldReader<u16>;
#[doc = "Field `RESEEDINTERVALMSB` writer - Reseed Interval MSB."]
pub type ReseedintervalmsbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Reseed Interval MSB."]
    #[inline(always)]
    pub fn reseedintervalmsb(&self) -> ReseedintervalmsbR {
        ReseedintervalmsbR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Reseed Interval MSB."]
    #[inline(always)]
    pub fn reseedintervalmsb(&mut self) -> ReseedintervalmsbW<ReseedintervalmsbSpec> {
        ReseedintervalmsbW::new(self, 0)
    }
}
#[doc = "Reseed Interval MSB register.\n\nYou can [`read`](crate::Reg::read) this register and get [`reseedintervalmsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reseedintervalmsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReseedintervalmsbSpec;
impl crate::RegisterSpec for ReseedintervalmsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reseedintervalmsb::R`](R) reader structure"]
impl crate::Readable for ReseedintervalmsbSpec {}
#[doc = "`write(|w| ..)` method takes [`reseedintervalmsb::W`](W) writer structure"]
impl crate::Writable for ReseedintervalmsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESEEDINTERVALMSB to value 0"]
impl crate::Resettable for ReseedintervalmsbSpec {
    const RESET_VALUE: u32 = 0;
}
