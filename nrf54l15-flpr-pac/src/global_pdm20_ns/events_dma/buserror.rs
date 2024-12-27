#[doc = "Register `BUSERROR` reader"]
pub type R = crate::R<BuserrorSpec>;
#[doc = "Register `BUSERROR` writer"]
pub type W = crate::W<BuserrorSpec>;
#[doc = "This event is generated if an error occurs during the bus transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Buserror {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<Buserror> for bool {
    #[inline(always)]
    fn from(variant: Buserror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSERROR` reader - This event is generated if an error occurs during the bus transfer."]
pub type BuserrorR = crate::BitReader<Buserror>;
impl BuserrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Buserror {
        match self.bits {
            false => Buserror::NotGenerated,
            true => Buserror::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == Buserror::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == Buserror::Generated
    }
}
#[doc = "Field `BUSERROR` writer - This event is generated if an error occurs during the bus transfer."]
pub type BuserrorW<'a, REG> = crate::BitWriter<'a, REG, Buserror>;
impl<'a, REG> BuserrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(Buserror::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(Buserror::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - This event is generated if an error occurs during the bus transfer."]
    #[inline(always)]
    pub fn buserror(&self) -> BuserrorR {
        BuserrorR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This event is generated if an error occurs during the bus transfer."]
    #[inline(always)]
    #[must_use]
    pub fn buserror(&mut self) -> BuserrorW<BuserrorSpec> {
        BuserrorW::new(self, 0)
    }
}
#[doc = "This event is generated if an error occurs during the bus transfer.\n\nYou can [`read`](crate::Reg::read) this register and get [`buserror::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buserror::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BuserrorSpec;
impl crate::RegisterSpec for BuserrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buserror::R`](R) reader structure"]
impl crate::Readable for BuserrorSpec {}
#[doc = "`write(|w| ..)` method takes [`buserror::W`](W) writer structure"]
impl crate::Writable for BuserrorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUSERROR to value 0"]
impl crate::Resettable for BuserrorSpec {
    const RESET_VALUE: u32 = 0;
}
