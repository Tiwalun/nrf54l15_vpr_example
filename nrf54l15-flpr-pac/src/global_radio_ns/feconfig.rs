#[doc = "Register `FECONFIG` reader"]
pub type R = crate::R<FeconfigSpec>;
#[doc = "Register `FECONFIG` writer"]
pub type W = crate::W<FeconfigSpec>;
#[doc = "Mode for narrow scaling output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scalermode {
    #[doc = "0: Classic log based scaling mode."]
    Disabled = 0,
    #[doc = "1: LUT based scaling mode."]
    Enabled = 1,
}
impl From<Scalermode> for bool {
    #[inline(always)]
    fn from(variant: Scalermode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCALERMODE` reader - Mode for narrow scaling output."]
pub type ScalermodeR = crate::BitReader<Scalermode>;
impl ScalermodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scalermode {
        match self.bits {
            false => Scalermode::Disabled,
            true => Scalermode::Enabled,
        }
    }
    #[doc = "Classic log based scaling mode."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Scalermode::Disabled
    }
    #[doc = "LUT based scaling mode."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Scalermode::Enabled
    }
}
#[doc = "Field `SCALERMODE` writer - Mode for narrow scaling output."]
pub type ScalermodeW<'a, REG> = crate::BitWriter<'a, REG, Scalermode>;
impl<'a, REG> ScalermodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Classic log based scaling mode."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Scalermode::Disabled)
    }
    #[doc = "LUT based scaling mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Scalermode::Enabled)
    }
}
impl R {
    #[doc = "Bit 20 - Mode for narrow scaling output."]
    #[inline(always)]
    pub fn scalermode(&self) -> ScalermodeR {
        ScalermodeR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - Mode for narrow scaling output."]
    #[inline(always)]
    pub fn scalermode(&mut self) -> ScalermodeW<FeconfigSpec> {
        ScalermodeW::new(self, 20)
    }
}
#[doc = "Config register\n\nYou can [`read`](crate::Reg::read) this register and get [`feconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeconfigSpec;
impl crate::RegisterSpec for FeconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`feconfig::R`](R) reader structure"]
impl crate::Readable for FeconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`feconfig::W`](W) writer structure"]
impl crate::Writable for FeconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FECONFIG to value 0x1080_0005"]
impl crate::Resettable for FeconfigSpec {
    const RESET_VALUE: u32 = 0x1080_0005;
}
