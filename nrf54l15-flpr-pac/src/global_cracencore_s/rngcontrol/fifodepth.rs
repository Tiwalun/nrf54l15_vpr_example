#[doc = "Register `FIFODEPTH` reader"]
pub type R = crate::R<FifodepthSpec>;
#[doc = "Register `FIFODEPTH` writer"]
pub type W = crate::W<FifodepthSpec>;
#[doc = "Field `FIFODEPTH` reader - Maximum number of 32 bits words that can be stored in the FIFO: 2**g_fifodepth."]
pub type FifodepthR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Maximum number of 32 bits words that can be stored in the FIFO: 2**g_fifodepth."]
    #[inline(always)]
    pub fn fifodepth(&self) -> FifodepthR {
        FifodepthR::new(self.bits)
    }
}
impl W {}
#[doc = "FIFO depth register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifodepth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifodepth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifodepthSpec;
impl crate::RegisterSpec for FifodepthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifodepth::R`](R) reader structure"]
impl crate::Readable for FifodepthSpec {}
#[doc = "`write(|w| ..)` method takes [`fifodepth::W`](W) writer structure"]
impl crate::Writable for FifodepthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFODEPTH to value 0x10"]
impl crate::Resettable for FifodepthSpec {
    const RESET_VALUE: u32 = 0x10;
}
