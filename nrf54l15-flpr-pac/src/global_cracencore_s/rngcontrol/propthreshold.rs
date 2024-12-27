#[doc = "Register `PROPTHRESHOLD` reader"]
pub type R = crate::R<PropthresholdSpec>;
#[doc = "Register `PROPTHRESHOLD` writer"]
pub type W = crate::W<PropthresholdSpec>;
#[doc = "Field `PROPTHRESHOLD` reader - Adaptive Proportion Test (1024-sample window) Cut-Off value."]
pub type PropthresholdR = crate::FieldReader<u16>;
#[doc = "Field `PROPTHRESHOLD` writer - Adaptive Proportion Test (1024-sample window) Cut-Off value."]
pub type PropthresholdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Adaptive Proportion Test (1024-sample window) Cut-Off value."]
    #[inline(always)]
    pub fn propthreshold(&self) -> PropthresholdR {
        PropthresholdR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Adaptive Proportion Test (1024-sample window) Cut-Off value."]
    #[inline(always)]
    pub fn propthreshold(&mut self) -> PropthresholdW<PropthresholdSpec> {
        PropthresholdW::new(self, 0)
    }
}
#[doc = "Adaptive Proportion Test (1024-sample window) Cut-Off value.\n\nYou can [`read`](crate::Reg::read) this register and get [`propthreshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`propthreshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PropthresholdSpec;
impl crate::RegisterSpec for PropthresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`propthreshold::R`](R) reader structure"]
impl crate::Readable for PropthresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`propthreshold::W`](W) writer structure"]
impl crate::Writable for PropthresholdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROPTHRESHOLD to value 0x0319"]
impl crate::Resettable for PropthresholdSpec {
    const RESET_VALUE: u32 = 0x0319;
}
