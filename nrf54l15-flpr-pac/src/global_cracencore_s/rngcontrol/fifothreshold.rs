#[doc = "Register `FIFOTHRESHOLD` reader"]
pub type R = crate::R<FifothresholdSpec>;
#[doc = "Register `FIFOTHRESHOLD` writer"]
pub type W = crate::W<FifothresholdSpec>;
#[doc = "Field `FIFOTHRESHOLD` reader - FIFO level below which the module leaves the idle state to refill the FIFO, expressed in number of 128bit blocks."]
pub type FifothresholdR = crate::FieldReader;
#[doc = "Field `FIFOTHRESHOLD` writer - FIFO level below which the module leaves the idle state to refill the FIFO, expressed in number of 128bit blocks."]
pub type FifothresholdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - FIFO level below which the module leaves the idle state to refill the FIFO, expressed in number of 128bit blocks."]
    #[inline(always)]
    pub fn fifothreshold(&self) -> FifothresholdR {
        FifothresholdR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - FIFO level below which the module leaves the idle state to refill the FIFO, expressed in number of 128bit blocks."]
    #[inline(always)]
    pub fn fifothreshold(&mut self) -> FifothresholdW<FifothresholdSpec> {
        FifothresholdW::new(self, 0)
    }
}
#[doc = "FIFO threshold register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifothreshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifothreshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifothresholdSpec;
impl crate::RegisterSpec for FifothresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifothreshold::R`](R) reader structure"]
impl crate::Readable for FifothresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`fifothreshold::W`](W) writer structure"]
impl crate::Writable for FifothresholdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOTHRESHOLD to value 0x03"]
impl crate::Resettable for FifothresholdSpec {
    const RESET_VALUE: u32 = 0x03;
}
