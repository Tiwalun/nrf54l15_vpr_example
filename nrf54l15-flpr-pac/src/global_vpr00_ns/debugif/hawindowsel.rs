#[doc = "Register `HAWINDOWSEL` reader"]
pub type R = crate::R<HawindowselSpec>;
#[doc = "Register `HAWINDOWSEL` writer"]
pub type W = crate::W<HawindowselSpec>;
#[doc = "Field `HAWINDOWSEL` reader - The high bits of this field may be tied to 0, depending on how large the array mask register is. E.g. on a system with 48 harts only bit 0 of this field may actually be writable."]
pub type HawindowselR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - The high bits of this field may be tied to 0, depending on how large the array mask register is. E.g. on a system with 48 harts only bit 0 of this field may actually be writable."]
    #[inline(always)]
    pub fn hawindowsel(&self) -> HawindowselR {
        HawindowselR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {}
#[doc = "Hart Array Window Select\n\nYou can [`read`](crate::Reg::read) this register and get [`hawindowsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hawindowsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HawindowselSpec;
impl crate::RegisterSpec for HawindowselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hawindowsel::R`](R) reader structure"]
impl crate::Readable for HawindowselSpec {}
#[doc = "`write(|w| ..)` method takes [`hawindowsel::W`](W) writer structure"]
impl crate::Writable for HawindowselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HAWINDOWSEL to value 0"]
impl crate::Resettable for HawindowselSpec {
    const RESET_VALUE: u32 = 0;
}
