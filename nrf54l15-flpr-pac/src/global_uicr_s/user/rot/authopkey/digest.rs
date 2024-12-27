#[doc = "Register `DIGEST[%s]` reader"]
pub type R = crate::R<DigestSpec>;
#[doc = "Register `DIGEST[%s]` writer"]
pub type W = crate::W<DigestSpec>;
#[doc = "Field `VALUE` reader - Value for word \\[o\\]
in the key digest \\[n\\]."]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Value for word \\[o\\]
in the key digest \\[n\\]."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value for word \\[o\\]
in the key digest \\[n\\]."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value for word \\[o\\]
in the key digest \\[n\\]."]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<DigestSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Description collection: First 256 bits of SHA2-512 digest over RoT authenticated operation public key generation \\[n\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`digest::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`digest::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DigestSpec;
impl crate::RegisterSpec for DigestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`digest::R`](R) reader structure"]
impl crate::Readable for DigestSpec {}
#[doc = "`write(|w| ..)` method takes [`digest::W`](W) writer structure"]
impl crate::Writable for DigestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIGEST[%s]
to value 0xffff_ffff"]
impl crate::Resettable for DigestSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
