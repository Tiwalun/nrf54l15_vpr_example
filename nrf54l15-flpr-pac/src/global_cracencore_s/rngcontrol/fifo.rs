#[doc = "Register `FIFO[%s]` reader"]
pub type R = crate::R<FifoSpec>;
#[doc = "Field `DATA` reader - FIFO data"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Description collection: FIFO data\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoSpec;
impl crate::RegisterSpec for FifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo::R`](R) reader structure"]
impl crate::Readable for FifoSpec {}
#[doc = "`reset()` method sets FIFO[%s]
to value 0"]
impl crate::Resettable for FifoSpec {
    const RESET_VALUE: u32 = 0;
}
