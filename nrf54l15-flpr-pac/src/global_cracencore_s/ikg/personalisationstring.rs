#[doc = "Register `PERSONALISATIONSTRING` reader"]
pub type R = crate::R<PersonalisationstringSpec>;
#[doc = "Register `PERSONALISATIONSTRING` writer"]
pub type W = crate::W<PersonalisationstringSpec>;
#[doc = "Field `PERSONALISATIONSTRING` reader - Personalisation String (write/read value 32-bit by 32-bit)."]
pub type PersonalisationstringR = crate::FieldReader<u32>;
#[doc = "Field `PERSONALISATIONSTRING` writer - Personalisation String (write/read value 32-bit by 32-bit)."]
pub type PersonalisationstringW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Personalisation String (write/read value 32-bit by 32-bit)."]
    #[inline(always)]
    pub fn personalisationstring(&self) -> PersonalisationstringR {
        PersonalisationstringR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Personalisation String (write/read value 32-bit by 32-bit)."]
    #[inline(always)]
    #[must_use]
    pub fn personalisationstring(&mut self) -> PersonalisationstringW<PersonalisationstringSpec> {
        PersonalisationstringW::new(self, 0)
    }
}
#[doc = "Personalisation String register.\n\nYou can [`read`](crate::Reg::read) this register and get [`personalisationstring::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`personalisationstring::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PersonalisationstringSpec;
impl crate::RegisterSpec for PersonalisationstringSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`personalisationstring::R`](R) reader structure"]
impl crate::Readable for PersonalisationstringSpec {}
#[doc = "`write(|w| ..)` method takes [`personalisationstring::W`](W) writer structure"]
impl crate::Writable for PersonalisationstringSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERSONALISATIONSTRING to value 0"]
impl crate::Resettable for PersonalisationstringSpec {
    const RESET_VALUE: u32 = 0;
}
