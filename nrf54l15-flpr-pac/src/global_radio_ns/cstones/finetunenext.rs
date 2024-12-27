#[doc = "Register `FINETUNENEXT` reader"]
pub type R = crate::R<FinetunenextSpec>;
#[doc = "Field `FINETUNENEXT` reader - Units of 488.28125 Hz"]
pub type FinetunenextR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - Units of 488.28125 Hz"]
    #[inline(always)]
    pub fn finetunenext(&self) -> FinetunenextR {
        FinetunenextR::new((self.bits & 0x1fff) as u16)
    }
}
#[doc = "Number of full ADPLL finetune steps\n\nYou can [`read`](crate::Reg::read) this register and get [`finetunenext::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FinetunenextSpec;
impl crate::RegisterSpec for FinetunenextSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`finetunenext::R`](R) reader structure"]
impl crate::Readable for FinetunenextSpec {}
#[doc = "`reset()` method sets FINETUNENEXT to value 0"]
impl crate::Resettable for FinetunenextSpec {
    const RESET_VALUE: u32 = 0;
}
