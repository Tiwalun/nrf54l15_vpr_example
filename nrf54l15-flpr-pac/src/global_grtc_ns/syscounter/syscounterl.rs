#[doc = "Register `SYSCOUNTERL` reader"]
pub type R = crate::R<SyscounterlSpec>;
#[doc = "Field `VALUE` reader - The lower 32-bits of the SYSCOUNTER value."]
pub type ValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The lower 32-bits of the SYSCOUNTER value."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
#[doc = "Description cluster: The lower 32-bits of the SYSCOUNTER for index \\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`syscounterl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscounterlSpec;
impl crate::RegisterSpec for SyscounterlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscounterl::R`](R) reader structure"]
impl crate::Readable for SyscounterlSpec {}
#[doc = "`reset()` method sets SYSCOUNTERL to value 0"]
impl crate::Resettable for SyscounterlSpec {
    const RESET_VALUE: u32 = 0;
}
