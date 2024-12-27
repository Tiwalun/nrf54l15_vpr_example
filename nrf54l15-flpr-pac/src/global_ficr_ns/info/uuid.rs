#[doc = "Register `UUID[%s]` reader"]
pub type R = crate::R<UuidSpec>;
#[doc = "Register `UUID[%s]` writer"]
pub type W = crate::W<UuidSpec>;
#[doc = "Field `UUID` reader - Device UUID \\[n\\]."]
pub type UuidR = crate::FieldReader<u32>;
#[doc = "Field `UUID` writer - Device UUID \\[n\\]."]
pub type UuidW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Device UUID \\[n\\]."]
    #[inline(always)]
    pub fn uuid(&self) -> UuidR {
        UuidR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Device UUID \\[n\\]."]
    #[inline(always)]
    pub fn uuid(&mut self) -> UuidW<UuidSpec> {
        UuidW::new(self, 0)
    }
}
#[doc = "Description collection: 128-bit Universally Unique IDentifier (UUID).\n\nYou can [`read`](crate::Reg::read) this register and get [`uuid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uuid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UuidSpec;
impl crate::RegisterSpec for UuidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uuid::R`](R) reader structure"]
impl crate::Readable for UuidSpec {}
#[doc = "`write(|w| ..)` method takes [`uuid::W`](W) writer structure"]
impl crate::Writable for UuidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UUID[%s]
to value 0xffff_ffff"]
impl crate::Resettable for UuidSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
