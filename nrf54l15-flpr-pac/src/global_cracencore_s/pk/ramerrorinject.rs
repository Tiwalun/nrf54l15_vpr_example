#[doc = "Register `RAMERRORINJECT` reader"]
pub type R = crate::R<RamerrorinjectSpec>;
#[doc = "Register `RAMERRORINJECT` writer"]
pub type W = crate::W<RamerrorinjectSpec>;
#[doc = "Field `BITERROR1` reader - Bit position of first error"]
pub type Biterror1R = crate::FieldReader<u16>;
#[doc = "Field `BITERROR1` writer - Bit position of first error"]
pub type Biterror1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `BITERROR2` reader - Bit position of second error"]
pub type Biterror2R = crate::FieldReader<u16>;
#[doc = "Field `BITERROR2` writer - Bit position of second error"]
pub type Biterror2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Bit position of first error"]
    #[inline(always)]
    pub fn biterror1(&self) -> Biterror1R {
        Biterror1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Bit position of second error"]
    #[inline(always)]
    pub fn biterror2(&self) -> Biterror2R {
        Biterror2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Bit position of first error"]
    #[inline(always)]
    #[must_use]
    pub fn biterror1(&mut self) -> Biterror1W<RamerrorinjectSpec> {
        Biterror1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Bit position of second error"]
    #[inline(always)]
    #[must_use]
    pub fn biterror2(&mut self) -> Biterror2W<RamerrorinjectSpec> {
        Biterror2W::new(self, 16)
    }
}
#[doc = "RAM error injection register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ramerrorinject::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramerrorinject::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamerrorinjectSpec;
impl crate::RegisterSpec for RamerrorinjectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ramerrorinject::R`](R) reader structure"]
impl crate::Readable for RamerrorinjectSpec {}
#[doc = "`write(|w| ..)` method takes [`ramerrorinject::W`](W) writer structure"]
impl crate::Writable for RamerrorinjectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAMERRORINJECT to value 0x03ff_03ff"]
impl crate::Resettable for RamerrorinjectSpec {
    const RESET_VALUE: u32 = 0x03ff_03ff;
}
