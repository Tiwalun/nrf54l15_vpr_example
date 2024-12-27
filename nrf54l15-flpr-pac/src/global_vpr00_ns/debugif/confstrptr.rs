#[doc = "Register `CONFSTRPTR[%s]` reader"]
pub type R = crate::R<ConfstrptrSpec>;
#[doc = "Register `CONFSTRPTR[%s]` writer"]
pub type W = crate::W<ConfstrptrSpec>;
#[doc = "Field `ADDR` reader - Address"]
pub type AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {}
#[doc = "Description collection: Configuration String Pointer \\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`confstrptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confstrptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfstrptrSpec;
impl crate::RegisterSpec for ConfstrptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`confstrptr::R`](R) reader structure"]
impl crate::Readable for ConfstrptrSpec {}
#[doc = "`write(|w| ..)` method takes [`confstrptr::W`](W) writer structure"]
impl crate::Writable for ConfstrptrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFSTRPTR[%s]
to value 0"]
impl crate::Resettable for ConfstrptrSpec {
    const RESET_VALUE: u32 = 0;
}
