#[doc = "Register `CCEN` reader"]
pub type R = crate::R<CcenSpec>;
#[doc = "Register `CCEN` writer"]
pub type W = crate::W<CcenSpec>;
#[doc = "Configure the Capture/Compare register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Active {
    #[doc = "0: Capture/Compare register CC\\[n\\]
Disabled."]
    Disable = 0,
    #[doc = "1: Capture/Compare register CC\\[n\\]
enabled."]
    Enable = 1,
}
impl From<Active> for bool {
    #[inline(always)]
    fn from(variant: Active) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVE` reader - Configure the Capture/Compare register"]
pub type ActiveR = crate::BitReader<Active>;
impl ActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Active {
        match self.bits {
            false => Active::Disable,
            true => Active::Enable,
        }
    }
    #[doc = "Capture/Compare register CC\\[n\\]
Disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Active::Disable
    }
    #[doc = "Capture/Compare register CC\\[n\\]
enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Active::Enable
    }
}
#[doc = "Field `ACTIVE` writer - Configure the Capture/Compare register"]
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG, Active>;
impl<'a, REG> ActiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture/Compare register CC\\[n\\]
Disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Active::Disable)
    }
    #[doc = "Capture/Compare register CC\\[n\\]
enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Active::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Configure the Capture/Compare register"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure the Capture/Compare register"]
    #[inline(always)]
    pub fn active(&mut self) -> ActiveW<CcenSpec> {
        ActiveW::new(self, 0)
    }
}
#[doc = "Description cluster: Configure Capture/Compare register CC\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ccen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcenSpec;
impl crate::RegisterSpec for CcenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccen::R`](R) reader structure"]
impl crate::Readable for CcenSpec {}
#[doc = "`write(|w| ..)` method takes [`ccen::W`](W) writer structure"]
impl crate::Writable for CcenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCEN to value 0"]
impl crate::Resettable for CcenSpec {
    const RESET_VALUE: u32 = 0;
}
