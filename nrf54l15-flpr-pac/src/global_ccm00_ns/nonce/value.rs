#[doc = "Register `VALUE[%s]` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Register `VALUE[%s]` writer"]
pub type W = crate::W<ValueSpec>;
#[doc = "Field `VALUE` reader - NONCE value, bits (32*(n+1))-1 : (32*n)"]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - NONCE value, bits (32*(n+1))-1 : (32*n)"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NONCE value, bits (32*(n+1))-1 : (32*n)"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - NONCE value, bits (32*(n+1))-1 : (32*n)"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<ValueSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Description collection: 13-byte NONCE vector Only the lower 13 bytes are used\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for ValueSpec {}
#[doc = "`write(|w| ..)` method takes [`value::W`](W) writer structure"]
impl crate::Writable for ValueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VALUE[%s]
to value 0"]
impl crate::Resettable for ValueSpec {
    const RESET_VALUE: u32 = 0;
}
