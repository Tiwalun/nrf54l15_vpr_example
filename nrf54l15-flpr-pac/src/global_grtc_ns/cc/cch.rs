#[doc = "Register `CCH` reader"]
pub type R = crate::R<CchSpec>;
#[doc = "Register `CCH` writer"]
pub type W = crate::W<CchSpec>;
#[doc = "Field `CCH` reader - Capture/Compare high value in 1 us"]
pub type CchR = crate::FieldReader<u32>;
#[doc = "Field `CCH` writer - Capture/Compare high value in 1 us"]
pub type CchW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Capture/Compare high value in 1 us"]
    #[inline(always)]
    pub fn cch(&self) -> CchR {
        CchR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Capture/Compare high value in 1 us"]
    #[inline(always)]
    pub fn cch(&mut self) -> CchW<CchSpec> {
        CchW::new(self, 0)
    }
}
#[doc = "Description cluster: The higher 32-bits of Capture/Compare register CC\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`cch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CchSpec;
impl crate::RegisterSpec for CchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cch::R`](R) reader structure"]
impl crate::Readable for CchSpec {}
#[doc = "`write(|w| ..)` method takes [`cch::W`](W) writer structure"]
impl crate::Writable for CchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCH to value 0"]
impl crate::Resettable for CchSpec {
    const RESET_VALUE: u32 = 0;
}
