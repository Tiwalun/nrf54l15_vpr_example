#[doc = "Register `VALUE[%s]` writer"]
pub type W = crate::W<ValueSpec>;
#[doc = "Field `VALUE` writer - AES 128-bit key value, bits (32*(i+1))-1 : (32*i)"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - AES 128-bit key value, bits (32*(i+1))-1 : (32*i)"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<ValueSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Description collection: 128-bit AES key\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
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
