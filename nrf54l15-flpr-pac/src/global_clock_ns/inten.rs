#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event XOSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xostarted {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Xostarted> for bool {
    #[inline(always)]
    fn from(variant: Xostarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XOSTARTED` reader - Enable or disable interrupt for event XOSTARTED"]
pub type XostartedR = crate::BitReader<Xostarted>;
impl XostartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xostarted {
        match self.bits {
            false => Xostarted::Disabled,
            true => Xostarted::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Xostarted::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Xostarted::Enabled
    }
}
#[doc = "Field `XOSTARTED` writer - Enable or disable interrupt for event XOSTARTED"]
pub type XostartedW<'a, REG> = crate::BitWriter<'a, REG, Xostarted>;
impl<'a, REG> XostartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Xostarted::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Xostarted::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event PLLSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllstarted {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Pllstarted> for bool {
    #[inline(always)]
    fn from(variant: Pllstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTARTED` reader - Enable or disable interrupt for event PLLSTARTED"]
pub type PllstartedR = crate::BitReader<Pllstarted>;
impl PllstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllstarted {
        match self.bits {
            false => Pllstarted::Disabled,
            true => Pllstarted::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pllstarted::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pllstarted::Enabled
    }
}
#[doc = "Field `PLLSTARTED` writer - Enable or disable interrupt for event PLLSTARTED"]
pub type PllstartedW<'a, REG> = crate::BitWriter<'a, REG, Pllstarted>;
impl<'a, REG> PllstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pllstarted::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pllstarted::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event LFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfclkstarted {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Lfclkstarted> for bool {
    #[inline(always)]
    fn from(variant: Lfclkstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFCLKSTARTED` reader - Enable or disable interrupt for event LFCLKSTARTED"]
pub type LfclkstartedR = crate::BitReader<Lfclkstarted>;
impl LfclkstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfclkstarted {
        match self.bits {
            false => Lfclkstarted::Disabled,
            true => Lfclkstarted::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lfclkstarted::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lfclkstarted::Enabled
    }
}
#[doc = "Field `LFCLKSTARTED` writer - Enable or disable interrupt for event LFCLKSTARTED"]
pub type LfclkstartedW<'a, REG> = crate::BitWriter<'a, REG, Lfclkstarted>;
impl<'a, REG> LfclkstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lfclkstarted::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lfclkstarted::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Enable or disable interrupt for event DONE"]
pub type DoneR = crate::BitReader<Done>;
impl DoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Done {
        match self.bits {
            false => Done::Disabled,
            true => Done::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Done::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Done::Enabled
    }
}
#[doc = "Field `DONE` writer - Enable or disable interrupt for event DONE"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG, Done>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Done::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Done::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event XOTUNED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xotuned {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Xotuned> for bool {
    #[inline(always)]
    fn from(variant: Xotuned) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XOTUNED` reader - Enable or disable interrupt for event XOTUNED"]
pub type XotunedR = crate::BitReader<Xotuned>;
impl XotunedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xotuned {
        match self.bits {
            false => Xotuned::Disabled,
            true => Xotuned::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Xotuned::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Xotuned::Enabled
    }
}
#[doc = "Field `XOTUNED` writer - Enable or disable interrupt for event XOTUNED"]
pub type XotunedW<'a, REG> = crate::BitWriter<'a, REG, Xotuned>;
impl<'a, REG> XotunedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Xotuned::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Xotuned::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event XOTUNEERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xotuneerror {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Xotuneerror> for bool {
    #[inline(always)]
    fn from(variant: Xotuneerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XOTUNEERROR` reader - Enable or disable interrupt for event XOTUNEERROR"]
pub type XotuneerrorR = crate::BitReader<Xotuneerror>;
impl XotuneerrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xotuneerror {
        match self.bits {
            false => Xotuneerror::Disabled,
            true => Xotuneerror::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Xotuneerror::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Xotuneerror::Enabled
    }
}
#[doc = "Field `XOTUNEERROR` writer - Enable or disable interrupt for event XOTUNEERROR"]
pub type XotuneerrorW<'a, REG> = crate::BitWriter<'a, REG, Xotuneerror>;
impl<'a, REG> XotuneerrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Xotuneerror::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Xotuneerror::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event XOTUNEFAILED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xotunefailed {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Xotunefailed> for bool {
    #[inline(always)]
    fn from(variant: Xotunefailed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XOTUNEFAILED` reader - Enable or disable interrupt for event XOTUNEFAILED"]
pub type XotunefailedR = crate::BitReader<Xotunefailed>;
impl XotunefailedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xotunefailed {
        match self.bits {
            false => Xotunefailed::Disabled,
            true => Xotunefailed::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Xotunefailed::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Xotunefailed::Enabled
    }
}
#[doc = "Field `XOTUNEFAILED` writer - Enable or disable interrupt for event XOTUNEFAILED"]
pub type XotunefailedW<'a, REG> = crate::BitWriter<'a, REG, Xotunefailed>;
impl<'a, REG> XotunefailedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Xotunefailed::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Xotunefailed::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event XOSTARTED"]
    #[inline(always)]
    pub fn xostarted(&self) -> XostartedR {
        XostartedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event PLLSTARTED"]
    #[inline(always)]
    pub fn pllstarted(&self) -> PllstartedR {
        PllstartedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&self) -> LfclkstartedR {
        LfclkstartedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event XOTUNED"]
    #[inline(always)]
    pub fn xotuned(&self) -> XotunedR {
        XotunedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event XOTUNEERROR"]
    #[inline(always)]
    pub fn xotuneerror(&self) -> XotuneerrorR {
        XotuneerrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event XOTUNEFAILED"]
    #[inline(always)]
    pub fn xotunefailed(&self) -> XotunefailedR {
        XotunefailedR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event XOSTARTED"]
    #[inline(always)]
    pub fn xostarted(&mut self) -> XostartedW<IntenSpec> {
        XostartedW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event PLLSTARTED"]
    #[inline(always)]
    pub fn pllstarted(&mut self) -> PllstartedW<IntenSpec> {
        PllstartedW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&mut self) -> LfclkstartedW<IntenSpec> {
        LfclkstartedW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<IntenSpec> {
        DoneW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event XOTUNED"]
    #[inline(always)]
    pub fn xotuned(&mut self) -> XotunedW<IntenSpec> {
        XotunedW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event XOTUNEERROR"]
    #[inline(always)]
    pub fn xotuneerror(&mut self) -> XotuneerrorW<IntenSpec> {
        XotuneerrorW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event XOTUNEFAILED"]
    #[inline(always)]
    pub fn xotunefailed(&mut self) -> XotunefailedW<IntenSpec> {
        XotunefailedW::new(self, 6)
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
