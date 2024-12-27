#[doc = "Register `HAWINDOW` reader"]
pub type R = crate::R<HawindowSpec>;
#[doc = "Register `HAWINDOW` writer"]
pub type W = crate::W<HawindowSpec>;
#[doc = "Field `MASKDATA` reader - Mask data."]
pub type MaskdataR = crate::FieldReader<u32>;
#[doc = "Field `MASKDATA` writer - Mask data."]
pub type MaskdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask data."]
    #[inline(always)]
    pub fn maskdata(&self) -> MaskdataR {
        MaskdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask data."]
    #[inline(always)]
    pub fn maskdata(&mut self) -> MaskdataW<HawindowSpec> {
        MaskdataW::new(self, 0)
    }
}
#[doc = "Hart Array Window\n\nYou can [`read`](crate::Reg::read) this register and get [`hawindow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hawindow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HawindowSpec;
impl crate::RegisterSpec for HawindowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hawindow::R`](R) reader structure"]
impl crate::Readable for HawindowSpec {}
#[doc = "`write(|w| ..)` method takes [`hawindow::W`](W) writer structure"]
impl crate::Writable for HawindowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HAWINDOW to value 0"]
impl crate::Resettable for HawindowSpec {
    const RESET_VALUE: u32 = 0;
}
