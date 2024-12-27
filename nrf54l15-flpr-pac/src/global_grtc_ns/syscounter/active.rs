#[doc = "Register `ACTIVE` reader"]
pub type R = crate::R<ActiveSpec>;
#[doc = "Register `ACTIVE` writer"]
pub type W = crate::W<ActiveSpec>;
#[doc = "Keep SYSCOUNTER in active state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Active {
    #[doc = "0: Allow SYSCOUNTER to go to sleep"]
    NotActive = 0,
    #[doc = "1: Keep SYSCOUNTER active"]
    Active = 1,
}
impl From<Active> for bool {
    #[inline(always)]
    fn from(variant: Active) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVE` reader - Keep SYSCOUNTER in active state"]
pub type ActiveR = crate::BitReader<Active>;
impl ActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Active {
        match self.bits {
            false => Active::NotActive,
            true => Active::Active,
        }
    }
    #[doc = "Allow SYSCOUNTER to go to sleep"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Active::NotActive
    }
    #[doc = "Keep SYSCOUNTER active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Active::Active
    }
}
#[doc = "Field `ACTIVE` writer - Keep SYSCOUNTER in active state"]
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG, Active>;
impl<'a, REG> ActiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Allow SYSCOUNTER to go to sleep"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(Active::NotActive)
    }
    #[doc = "Keep SYSCOUNTER active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Active::Active)
    }
}
impl R {
    #[doc = "Bit 0 - Keep SYSCOUNTER in active state"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Keep SYSCOUNTER in active state"]
    #[inline(always)]
    pub fn active(&mut self) -> ActiveW<ActiveSpec> {
        ActiveW::new(self, 0)
    }
}
#[doc = "Description cluster: Request to keep the SYSCOUNTER in the active state and prevent going to sleep for index \\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`active::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`active::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActiveSpec;
impl crate::RegisterSpec for ActiveSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`active::R`](R) reader structure"]
impl crate::Readable for ActiveSpec {}
#[doc = "`write(|w| ..)` method takes [`active::W`](W) writer structure"]
impl crate::Writable for ActiveSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACTIVE to value 0"]
impl crate::Resettable for ActiveSpec {
    const RESET_VALUE: u32 = 0;
}
