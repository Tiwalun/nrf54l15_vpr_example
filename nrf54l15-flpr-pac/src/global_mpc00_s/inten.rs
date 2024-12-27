#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event MEMACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memaccerr {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Memaccerr> for bool {
    #[inline(always)]
    fn from(variant: Memaccerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMACCERR` reader - Enable or disable interrupt for event MEMACCERR"]
pub type MemaccerrR = crate::BitReader<Memaccerr>;
impl MemaccerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memaccerr {
        match self.bits {
            false => Memaccerr::Disabled,
            true => Memaccerr::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Memaccerr::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Memaccerr::Enabled
    }
}
#[doc = "Field `MEMACCERR` writer - Enable or disable interrupt for event MEMACCERR"]
pub type MemaccerrW<'a, REG> = crate::BitWriter<'a, REG, Memaccerr>;
impl<'a, REG> MemaccerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Memaccerr::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Memaccerr::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event MEMACCERR"]
    #[inline(always)]
    pub fn memaccerr(&self) -> MemaccerrR {
        MemaccerrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event MEMACCERR"]
    #[inline(always)]
    #[must_use]
    pub fn memaccerr(&mut self) -> MemaccerrW<IntenSpec> {
        MemaccerrW::new(self, 0)
    }
}
#[doc = "Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
