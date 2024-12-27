#[doc = "Register `SYSPWRUPREQ` reader"]
pub type R = crate::R<SyspwrupreqSpec>;
#[doc = "Register `SYSPWRUPREQ` writer"]
pub type W = crate::W<SyspwrupreqSpec>;
#[doc = "Activate power-up request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Active {
    #[doc = "0: Power-up request not active"]
    NotActive = 0,
    #[doc = "1: Power-up request active"]
    Active = 1,
}
impl From<Active> for bool {
    #[inline(always)]
    fn from(variant: Active) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVE` reader - Activate power-up request"]
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
    #[doc = "Power-up request not active"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Active::NotActive
    }
    #[doc = "Power-up request active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Active::Active
    }
}
#[doc = "Field `ACTIVE` writer - Activate power-up request"]
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG, Active>;
impl<'a, REG> ActiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power-up request not active"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(Active::NotActive)
    }
    #[doc = "Power-up request active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Active::Active)
    }
}
impl R {
    #[doc = "Bit 0 - Activate power-up request"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Activate power-up request"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ActiveW<SyspwrupreqSpec> {
        ActiveW::new(self, 0)
    }
}
#[doc = "System power-up request\n\nYou can [`read`](crate::Reg::read) this register and get [`syspwrupreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspwrupreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyspwrupreqSpec;
impl crate::RegisterSpec for SyspwrupreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspwrupreq::R`](R) reader structure"]
impl crate::Readable for SyspwrupreqSpec {}
#[doc = "`write(|w| ..)` method takes [`syspwrupreq::W`](W) writer structure"]
impl crate::Writable for SyspwrupreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSPWRUPREQ to value 0"]
impl crate::Resettable for SyspwrupreqSpec {
    const RESET_VALUE: u32 = 0;
}
