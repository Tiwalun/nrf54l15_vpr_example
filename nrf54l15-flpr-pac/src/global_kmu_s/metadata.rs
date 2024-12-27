#[doc = "Register `METADATA` reader"]
pub type R = crate::R<MetadataSpec>;
#[doc = "Register `METADATA` writer"]
pub type W = crate::W<MetadataSpec>;
#[doc = "Field `METADATA` reader - Read metadata."]
pub type MetadataR = crate::FieldReader<u32>;
#[doc = "Field `METADATA` writer - Read metadata."]
pub type MetadataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read metadata."]
    #[inline(always)]
    pub fn metadata(&self) -> MetadataR {
        MetadataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read metadata."]
    #[inline(always)]
    #[must_use]
    pub fn metadata(&mut self) -> MetadataW<MetadataSpec> {
        MetadataW::new(self, 0)
    }
}
#[doc = "Key slot metadata as read by TASKS_READMETADATA.\n\nYou can [`read`](crate::Reg::read) this register and get [`metadata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`metadata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MetadataSpec;
impl crate::RegisterSpec for MetadataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`metadata::R`](R) reader structure"]
impl crate::Readable for MetadataSpec {}
#[doc = "`write(|w| ..)` method takes [`metadata::W`](W) writer structure"]
impl crate::Writable for MetadataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets METADATA to value 0"]
impl crate::Resettable for MetadataSpec {
    const RESET_VALUE: u32 = 0;
}
