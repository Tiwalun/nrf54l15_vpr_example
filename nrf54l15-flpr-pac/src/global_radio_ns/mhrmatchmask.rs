#[doc = "Register `MHRMATCHMASK` reader"]
pub type R = crate::R<MhrmatchmaskSpec>;
#[doc = "Register `MHRMATCHMASK` writer"]
pub type W = crate::W<MhrmatchmaskSpec>;
#[doc = "Field `MHRMATCHMASK` reader - Pattern mask"]
pub type MhrmatchmaskR = crate::FieldReader<u32>;
#[doc = "Field `MHRMATCHMASK` writer - Pattern mask"]
pub type MhrmatchmaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Pattern mask"]
    #[inline(always)]
    pub fn mhrmatchmask(&self) -> MhrmatchmaskR {
        MhrmatchmaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pattern mask"]
    #[inline(always)]
    pub fn mhrmatchmask(&mut self) -> MhrmatchmaskW<MhrmatchmaskSpec> {
        MhrmatchmaskW::new(self, 0)
    }
}
#[doc = "Pattern mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mhrmatchmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mhrmatchmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MhrmatchmaskSpec;
impl crate::RegisterSpec for MhrmatchmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhrmatchmask::R`](R) reader structure"]
impl crate::Readable for MhrmatchmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`mhrmatchmask::W`](W) writer structure"]
impl crate::Writable for MhrmatchmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MHRMATCHMASK to value 0"]
impl crate::Resettable for MhrmatchmaskSpec {
    const RESET_VALUE: u32 = 0;
}
