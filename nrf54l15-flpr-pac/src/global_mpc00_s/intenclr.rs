#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to disable interrupt for event MEMACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memaccerr {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Memaccerr> for bool {
    #[inline(always)]
    fn from(variant: Memaccerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMACCERR` reader - Write '1' to disable interrupt for event MEMACCERR"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Memaccerr::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Memaccerr::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event MEMACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemaccerrWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<MemaccerrWO> for bool {
    #[inline(always)]
    fn from(variant: MemaccerrWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMACCERR` writer - Write '1' to disable interrupt for event MEMACCERR"]
pub type MemaccerrW<'a, REG> = crate::BitWriter<'a, REG, MemaccerrWO>;
impl<'a, REG> MemaccerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MemaccerrWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event MEMACCERR"]
    #[inline(always)]
    pub fn memaccerr(&self) -> MemaccerrR {
        MemaccerrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event MEMACCERR"]
    #[inline(always)]
    pub fn memaccerr(&mut self) -> MemaccerrW<IntenclrSpec> {
        MemaccerrW::new(self, 0)
    }
}
#[doc = "Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u32 = 0;
}
