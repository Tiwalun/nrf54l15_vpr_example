#[doc = "Register `MAXRESOLVED` reader"]
pub type R = crate::R<MaxresolvedSpec>;
#[doc = "Register `MAXRESOLVED` writer"]
pub type W = crate::W<MaxresolvedSpec>;
#[doc = "Field `MAXRESOLVED` reader - The maximum number of IRKs to resolve"]
pub type MaxresolvedR = crate::FieldReader<u16>;
#[doc = "Field `MAXRESOLVED` writer - The maximum number of IRKs to resolve"]
pub type MaxresolvedW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - The maximum number of IRKs to resolve"]
    #[inline(always)]
    pub fn maxresolved(&self) -> MaxresolvedR {
        MaxresolvedR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - The maximum number of IRKs to resolve"]
    #[inline(always)]
    #[must_use]
    pub fn maxresolved(&mut self) -> MaxresolvedW<MaxresolvedSpec> {
        MaxresolvedW::new(self, 0)
    }
}
#[doc = "Maximum number of IRKs to resolve\n\nYou can [`read`](crate::Reg::read) this register and get [`maxresolved::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxresolved::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxresolvedSpec;
impl crate::RegisterSpec for MaxresolvedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxresolved::R`](R) reader structure"]
impl crate::Readable for MaxresolvedSpec {}
#[doc = "`write(|w| ..)` method takes [`maxresolved::W`](W) writer structure"]
impl crate::Writable for MaxresolvedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAXRESOLVED to value 0x01"]
impl crate::Resettable for MaxresolvedSpec {
    const RESET_VALUE: u32 = 0x01;
}
