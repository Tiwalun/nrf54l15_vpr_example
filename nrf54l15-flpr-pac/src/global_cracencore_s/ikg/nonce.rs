#[doc = "Register `NONCE` reader"]
pub type R = crate::R<NonceSpec>;
#[doc = "Register `NONCE` writer"]
pub type W = crate::W<NonceSpec>;
#[doc = "Field `NONCE` reader - Nonce (write/read value 32-bit by 32-bit)."]
pub type NonceR = crate::FieldReader<u32>;
#[doc = "Field `NONCE` writer - Nonce (write/read value 32-bit by 32-bit)."]
pub type NonceW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Nonce (write/read value 32-bit by 32-bit)."]
    #[inline(always)]
    pub fn nonce(&self) -> NonceR {
        NonceR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Nonce (write/read value 32-bit by 32-bit)."]
    #[inline(always)]
    pub fn nonce(&mut self) -> NonceW<NonceSpec> {
        NonceW::new(self, 0)
    }
}
#[doc = "Nonce register.\n\nYou can [`read`](crate::Reg::read) this register and get [`nonce::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonce::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NonceSpec;
impl crate::RegisterSpec for NonceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nonce::R`](R) reader structure"]
impl crate::Readable for NonceSpec {}
#[doc = "`write(|w| ..)` method takes [`nonce::W`](W) writer structure"]
impl crate::Writable for NonceSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NONCE to value 0"]
impl crate::Resettable for NonceSpec {
    const RESET_VALUE: u32 = 0;
}
