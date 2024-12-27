#[doc = "Register `KEY[%s]` reader"]
pub type R = crate::R<KeySpec>;
#[doc = "Register `KEY[%s]` writer"]
pub type W = crate::W<KeySpec>;
#[doc = "Field `KEY` reader - Key register."]
pub type KeyR = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - Key register."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key register."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key register."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<KeySpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "Description collection: Key register.\n\nYou can [`read`](crate::Reg::read) this register and get [`key::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeySpec;
impl crate::RegisterSpec for KeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key::R`](R) reader structure"]
impl crate::Readable for KeySpec {}
#[doc = "`write(|w| ..)` method takes [`key::W`](W) writer structure"]
impl crate::Writable for KeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY[%s]
to value 0"]
impl crate::Resettable for KeySpec {
    const RESET_VALUE: u32 = 0;
}
