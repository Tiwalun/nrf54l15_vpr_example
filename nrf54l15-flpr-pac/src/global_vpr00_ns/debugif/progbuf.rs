#[doc = "Register `PROGBUF[%s]` reader"]
pub type R = crate::R<ProgbufSpec>;
#[doc = "Register `PROGBUF[%s]` writer"]
pub type W = crate::W<ProgbufSpec>;
#[doc = "Field `DATA` reader - Data"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {}
#[doc = "Description collection: Program Buffer \\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`progbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`progbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProgbufSpec;
impl crate::RegisterSpec for ProgbufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`progbuf::R`](R) reader structure"]
impl crate::Readable for ProgbufSpec {}
#[doc = "`write(|w| ..)` method takes [`progbuf::W`](W) writer structure"]
impl crate::Writable for ProgbufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROGBUF[%s]
to value 0"]
impl crate::Resettable for ProgbufSpec {
    const RESET_VALUE: u32 = 0;
}
