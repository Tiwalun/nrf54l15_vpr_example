#[doc = "Register `BA424ARIAHWCFG` reader"]
pub type R = crate::R<Ba424ariahwcfgSpec>;
#[doc = "Register `BA424ARIAHWCFG` writer"]
pub type W = crate::W<Ba424ariahwcfgSpec>;
#[doc = "Field `BA424ARIAHWCFG` reader - Generic g_aria_modePoss value."]
pub type Ba424ariahwcfgR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Generic g_aria_modePoss value."]
    #[inline(always)]
    pub fn ba424ariahwcfg(&self) -> Ba424ariahwcfgR {
        Ba424ariahwcfgR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {}
#[doc = "Generic g_aria_modePoss value.\n\nYou can [`read`](crate::Reg::read) this register and get [`ba424ariahwcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ba424ariahwcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ba424ariahwcfgSpec;
impl crate::RegisterSpec for Ba424ariahwcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ba424ariahwcfg::R`](R) reader structure"]
impl crate::Readable for Ba424ariahwcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ba424ariahwcfg::W`](W) writer structure"]
impl crate::Writable for Ba424ariahwcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BA424ARIAHWCFG to value 0x017f"]
impl crate::Resettable for Ba424ariahwcfgSpec {
    const RESET_VALUE: u32 = 0x017f;
}
