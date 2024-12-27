#[doc = "Register `CCL` reader"]
pub type R = crate::R<CclSpec>;
#[doc = "Register `CCL` writer"]
pub type W = crate::W<CclSpec>;
#[doc = "Field `CCL` reader - Capture/Compare low value in 1 us"]
pub type CclR = crate::FieldReader<u32>;
#[doc = "Field `CCL` writer - Capture/Compare low value in 1 us"]
pub type CclW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Capture/Compare low value in 1 us"]
    #[inline(always)]
    pub fn ccl(&self) -> CclR {
        CclR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Capture/Compare low value in 1 us"]
    #[inline(always)]
    pub fn ccl(&mut self) -> CclW<CclSpec> {
        CclW::new(self, 0)
    }
}
#[doc = "Description cluster: The lower 32-bits of Capture/Compare register CC\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ccl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CclSpec;
impl crate::RegisterSpec for CclSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccl::R`](R) reader structure"]
impl crate::Readable for CclSpec {}
#[doc = "`write(|w| ..)` method takes [`ccl::W`](W) writer structure"]
impl crate::Writable for CclSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCL to value 0"]
impl crate::Resettable for CclSpec {
    const RESET_VALUE: u32 = 0;
}
