#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to disable interrupt for event XOSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xostarted {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Xostarted> for bool {
    #[inline(always)]
    fn from(variant: Xostarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XOSTARTED` reader - Write '1' to disable interrupt for event XOSTARTED"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Xostarted::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Xostarted::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event XOSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XostartedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<XostartedWO> for bool {
    #[inline(always)]
    fn from(variant: XostartedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XOSTARTED` writer - Write '1' to disable interrupt for event XOSTARTED"]
pub type XostartedW<'a, REG> = crate::BitWriter<'a, REG, XostartedWO>;
impl<'a, REG> XostartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(XostartedWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event PLLSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllstarted {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Pllstarted> for bool {
    #[inline(always)]
    fn from(variant: Pllstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTARTED` reader - Write '1' to disable interrupt for event PLLSTARTED"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pllstarted::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pllstarted::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event PLLSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PllstartedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<PllstartedWO> for bool {
    #[inline(always)]
    fn from(variant: PllstartedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTARTED` writer - Write '1' to disable interrupt for event PLLSTARTED"]
pub type PllstartedW<'a, REG> = crate::BitWriter<'a, REG, PllstartedWO>;
impl<'a, REG> PllstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PllstartedWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event LFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfclkstarted {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Lfclkstarted> for bool {
    #[inline(always)]
    fn from(variant: Lfclkstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFCLKSTARTED` reader - Write '1' to disable interrupt for event LFCLKSTARTED"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lfclkstarted::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lfclkstarted::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event LFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LfclkstartedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<LfclkstartedWO> for bool {
    #[inline(always)]
    fn from(variant: LfclkstartedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFCLKSTARTED` writer - Write '1' to disable interrupt for event LFCLKSTARTED"]
pub type LfclkstartedW<'a, REG> = crate::BitWriter<'a, REG, LfclkstartedWO>;
impl<'a, REG> LfclkstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LfclkstartedWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event DONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Write '1' to disable interrupt for event DONE"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Done::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Done::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event DONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DoneWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<DoneWO> for bool {
    #[inline(always)]
    fn from(variant: DoneWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` writer - Write '1' to disable interrupt for event DONE"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG, DoneWO>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DoneWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event XOTUNED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xotuned {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Xotuned> for bool {
    #[inline(always)]
    fn from(variant: Xotuned) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XOTUNED` reader - Write '1' to disable interrupt for event XOTUNED"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Xotuned::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Xotuned::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event XOTUNED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XotunedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<XotunedWO> for bool {
    #[inline(always)]
    fn from(variant: XotunedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XOTUNED` writer - Write '1' to disable interrupt for event XOTUNED"]
pub type XotunedW<'a, REG> = crate::BitWriter<'a, REG, XotunedWO>;
impl<'a, REG> XotunedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(XotunedWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event XOTUNEERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xotuneerror {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Xotuneerror> for bool {
    #[inline(always)]
    fn from(variant: Xotuneerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XOTUNEERROR` reader - Write '1' to disable interrupt for event XOTUNEERROR"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Xotuneerror::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Xotuneerror::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event XOTUNEERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XotuneerrorWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<XotuneerrorWO> for bool {
    #[inline(always)]
    fn from(variant: XotuneerrorWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XOTUNEERROR` writer - Write '1' to disable interrupt for event XOTUNEERROR"]
pub type XotuneerrorW<'a, REG> = crate::BitWriter<'a, REG, XotuneerrorWO>;
impl<'a, REG> XotuneerrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(XotuneerrorWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event XOTUNEFAILED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xotunefailed {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Xotunefailed> for bool {
    #[inline(always)]
    fn from(variant: Xotunefailed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XOTUNEFAILED` reader - Write '1' to disable interrupt for event XOTUNEFAILED"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Xotunefailed::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Xotunefailed::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event XOTUNEFAILED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XotunefailedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<XotunefailedWO> for bool {
    #[inline(always)]
    fn from(variant: XotunefailedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XOTUNEFAILED` writer - Write '1' to disable interrupt for event XOTUNEFAILED"]
pub type XotunefailedW<'a, REG> = crate::BitWriter<'a, REG, XotunefailedWO>;
impl<'a, REG> XotunefailedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(XotunefailedWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event XOSTARTED"]
    #[inline(always)]
    pub fn xostarted(&self) -> XostartedR {
        XostartedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event PLLSTARTED"]
    #[inline(always)]
    pub fn pllstarted(&self) -> PllstartedR {
        PllstartedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&self) -> LfclkstartedR {
        LfclkstartedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event XOTUNED"]
    #[inline(always)]
    pub fn xotuned(&self) -> XotunedR {
        XotunedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event XOTUNEERROR"]
    #[inline(always)]
    pub fn xotuneerror(&self) -> XotuneerrorR {
        XotuneerrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event XOTUNEFAILED"]
    #[inline(always)]
    pub fn xotunefailed(&self) -> XotunefailedR {
        XotunefailedR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event XOSTARTED"]
    #[inline(always)]
    pub fn xostarted(&mut self) -> XostartedW<IntenclrSpec> {
        XostartedW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event PLLSTARTED"]
    #[inline(always)]
    pub fn pllstarted(&mut self) -> PllstartedW<IntenclrSpec> {
        PllstartedW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&mut self) -> LfclkstartedW<IntenclrSpec> {
        LfclkstartedW::new(self, 2)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<IntenclrSpec> {
        DoneW::new(self, 3)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event XOTUNED"]
    #[inline(always)]
    pub fn xotuned(&mut self) -> XotunedW<IntenclrSpec> {
        XotunedW::new(self, 4)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event XOTUNEERROR"]
    #[inline(always)]
    pub fn xotuneerror(&mut self) -> XotuneerrorW<IntenclrSpec> {
        XotuneerrorW::new(self, 5)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event XOTUNEFAILED"]
    #[inline(always)]
    pub fn xotunefailed(&mut self) -> XotunefailedW<IntenclrSpec> {
        XotunefailedW::new(self, 6)
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
