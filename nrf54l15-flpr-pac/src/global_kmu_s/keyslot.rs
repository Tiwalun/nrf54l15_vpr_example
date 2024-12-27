#[doc = "Register `KEYSLOT` reader"]
pub type R = crate::R<KeyslotSpec>;
#[doc = "Register `KEYSLOT` writer"]
pub type W = crate::W<KeyslotSpec>;
#[doc = "Field `ID` reader - Select key slot ID to provision, push, read METADATA, revoke or block when the corresponding task is triggered."]
pub type IdR = crate::FieldReader;
#[doc = "Field `ID` writer - Select key slot ID to provision, push, read METADATA, revoke or block when the corresponding task is triggered."]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Select key slot ID to provision, push, read METADATA, revoke or block when the corresponding task is triggered."]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Select key slot ID to provision, push, read METADATA, revoke or block when the corresponding task is triggered."]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<KeyslotSpec> {
        IdW::new(self, 0)
    }
}
#[doc = "Select key slot to operate on\n\nYou can [`read`](crate::Reg::read) this register and get [`keyslot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyslot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyslotSpec;
impl crate::RegisterSpec for KeyslotSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyslot::R`](R) reader structure"]
impl crate::Readable for KeyslotSpec {}
#[doc = "`write(|w| ..)` method takes [`keyslot::W`](W) writer structure"]
impl crate::Writable for KeyslotSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYSLOT to value 0"]
impl crate::Resettable for KeyslotSpec {
    const RESET_VALUE: u32 = 0;
}
