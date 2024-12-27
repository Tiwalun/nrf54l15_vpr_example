#[doc = "Register `DCDCEN` reader"]
pub type R = crate::R<DcdcenSpec>;
#[doc = "Register `DCDCEN` writer"]
pub type W = crate::W<DcdcenSpec>;
#[doc = "Enable DC/DC buck converter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Val {
    #[doc = "0: Disable DC/DC converter and use LDO"]
    Disabled = 0,
    #[doc = "1: Enable DC/DC converter"]
    Enabled = 1,
}
impl From<Val> for bool {
    #[inline(always)]
    fn from(variant: Val) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VAL` reader - Enable DC/DC buck converter"]
pub type ValR = crate::BitReader<Val>;
impl ValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Val {
        match self.bits {
            false => Val::Disabled,
            true => Val::Enabled,
        }
    }
    #[doc = "Disable DC/DC converter and use LDO"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Val::Disabled
    }
    #[doc = "Enable DC/DC converter"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Val::Enabled
    }
}
#[doc = "Field `VAL` writer - Enable DC/DC buck converter"]
pub type ValW<'a, REG> = crate::BitWriter<'a, REG, Val>;
impl<'a, REG> ValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable DC/DC converter and use LDO"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Disabled)
    }
    #[doc = "Enable DC/DC converter"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable DC/DC buck converter"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DC/DC buck converter"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<DcdcenSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Enable DC/DC converter for better power efficiency\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcdcenSpec;
impl crate::RegisterSpec for DcdcenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdcen::R`](R) reader structure"]
impl crate::Readable for DcdcenSpec {}
#[doc = "`write(|w| ..)` method takes [`dcdcen::W`](W) writer structure"]
impl crate::Writable for DcdcenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDCEN to value 0"]
impl crate::Resettable for DcdcenSpec {
    const RESET_VALUE: u32 = 0;
}
