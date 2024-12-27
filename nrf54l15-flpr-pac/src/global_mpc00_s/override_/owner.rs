#[doc = "Register `OWNER` reader"]
pub type R = crate::R<OwnerSpec>;
#[doc = "Register `OWNER` writer"]
pub type W = crate::W<OwnerSpec>;
#[doc = "Field `OWNERID` reader - owner identifier for override region n"]
pub type OwneridR = crate::FieldReader;
#[doc = "Field `OWNERID` writer - owner identifier for override region n"]
pub type OwneridW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - owner identifier for override region n"]
    #[inline(always)]
    pub fn ownerid(&self) -> OwneridR {
        OwneridR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - owner identifier for override region n"]
    #[inline(always)]
    #[must_use]
    pub fn ownerid(&mut self) -> OwneridW<OwnerSpec> {
        OwneridW::new(self, 0)
    }
}
#[doc = "Description cluster: Owner for override region\n\nYou can [`read`](crate::Reg::read) this register and get [`owner::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`owner::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OwnerSpec;
impl crate::RegisterSpec for OwnerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`owner::R`](R) reader structure"]
impl crate::Readable for OwnerSpec {}
#[doc = "`write(|w| ..)` method takes [`owner::W`](W) writer structure"]
impl crate::Writable for OwnerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OWNER to value 0"]
impl crate::Resettable for OwnerSpec {
    const RESET_VALUE: u32 = 0;
}
