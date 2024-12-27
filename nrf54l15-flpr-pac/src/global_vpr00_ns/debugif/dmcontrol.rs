#[doc = "Register `DMCONTROL` reader"]
pub type R = crate::R<DmcontrolSpec>;
#[doc = "Register `DMCONTROL` writer"]
pub type W = crate::W<DmcontrolSpec>;
#[doc = "Reset signal for the debug module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmactive {
    #[doc = "0: Reset the debug module itself"]
    Disabled = 0,
    #[doc = "1: Normal operation"]
    Enabled = 1,
}
impl From<Dmactive> for bool {
    #[inline(always)]
    fn from(variant: Dmactive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACTIVE` reader - Reset signal for the debug module."]
pub type DmactiveR = crate::BitReader<Dmactive>;
impl DmactiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmactive {
        match self.bits {
            false => Dmactive::Disabled,
            true => Dmactive::Enabled,
        }
    }
    #[doc = "Reset the debug module itself"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmactive::Disabled
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmactive::Enabled
    }
}
#[doc = "Field `DMACTIVE` writer - Reset signal for the debug module."]
pub type DmactiveW<'a, REG> = crate::BitWriter<'a, REG, Dmactive>;
impl<'a, REG> DmactiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the debug module itself"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmactive::Disabled)
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmactive::Enabled)
    }
}
#[doc = "Reset signal output from the debug module to the system.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ndmreset {
    #[doc = "0: Reset inactive"]
    Inactive = 0,
    #[doc = "1: Reset active"]
    Active = 1,
}
impl From<Ndmreset> for bool {
    #[inline(always)]
    fn from(variant: Ndmreset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NDMRESET` reader - Reset signal output from the debug module to the system."]
pub type NdmresetR = crate::BitReader<Ndmreset>;
impl NdmresetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ndmreset {
        match self.bits {
            false => Ndmreset::Inactive,
            true => Ndmreset::Active,
        }
    }
    #[doc = "Reset inactive"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ndmreset::Inactive
    }
    #[doc = "Reset active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ndmreset::Active
    }
}
#[doc = "Field `NDMRESET` writer - Reset signal output from the debug module to the system."]
pub type NdmresetW<'a, REG> = crate::BitWriter<'a, REG, Ndmreset>;
impl<'a, REG> NdmresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset inactive"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ndmreset::Inactive)
    }
    #[doc = "Reset active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ndmreset::Active)
    }
}
#[doc = "Clear the halt on reset request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clrresethaltreq {
    #[doc = "0: No operation when written 0."]
    NoOperation = 0,
    #[doc = "1: Clears the halt on reset request"]
    Clear = 1,
}
impl From<Clrresethaltreq> for bool {
    #[inline(always)]
    fn from(variant: Clrresethaltreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRRESETHALTREQ` writer - Clear the halt on reset request."]
pub type ClrresethaltreqW<'a, REG> = crate::BitWriter<'a, REG, Clrresethaltreq>;
impl<'a, REG> ClrresethaltreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation when written 0."]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut crate::W<REG> {
        self.variant(Clrresethaltreq::NoOperation)
    }
    #[doc = "Clears the halt on reset request"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Clrresethaltreq::Clear)
    }
}
#[doc = "Set the halt on reset request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Setresethaltreq {
    #[doc = "0: No operation when written 0."]
    NoOperation = 0,
    #[doc = "1: Sets the halt on reset request"]
    Clear = 1,
}
impl From<Setresethaltreq> for bool {
    #[inline(always)]
    fn from(variant: Setresethaltreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETRESETHALTREQ` writer - Set the halt on reset request."]
pub type SetresethaltreqW<'a, REG> = crate::BitWriter<'a, REG, Setresethaltreq>;
impl<'a, REG> SetresethaltreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation when written 0."]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut crate::W<REG> {
        self.variant(Setresethaltreq::NoOperation)
    }
    #[doc = "Sets the halt on reset request"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Setresethaltreq::Clear)
    }
}
#[doc = "Field `HARTSELHI` writer - The high 10 bits of hartsel."]
pub type HartselhiW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `HARTSELLO` writer - The low 10 bits of hartsel."]
pub type HartselloW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Definition of currently selected harts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hasel {
    #[doc = "0: Single hart selected."]
    Single = 0,
    #[doc = "1: Multiple harts selected"]
    Multiple = 1,
}
impl From<Hasel> for bool {
    #[inline(always)]
    fn from(variant: Hasel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASEL` writer - Definition of currently selected harts."]
pub type HaselW<'a, REG> = crate::BitWriter<'a, REG, Hasel>;
impl<'a, REG> HaselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single hart selected."]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Hasel::Single)
    }
    #[doc = "Multiple harts selected"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut crate::W<REG> {
        self.variant(Hasel::Multiple)
    }
}
#[doc = "Clear the havereset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackhavereset {
    #[doc = "0: No operation when written 0."]
    NoOperation = 0,
    #[doc = "1: Clears the havereset for selected harts."]
    Clear = 1,
}
impl From<Ackhavereset> for bool {
    #[inline(always)]
    fn from(variant: Ackhavereset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKHAVERESET` writer - Clear the havereset."]
pub type AckhaveresetW<'a, REG> = crate::BitWriter<'a, REG, Ackhavereset>;
impl<'a, REG> AckhaveresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation when written 0."]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut crate::W<REG> {
        self.variant(Ackhavereset::NoOperation)
    }
    #[doc = "Clears the havereset for selected harts."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ackhavereset::Clear)
    }
}
#[doc = "Reset harts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hartreset {
    #[doc = "0: Reset de-asserted."]
    Deasserted = 0,
    #[doc = "1: Reset asserted."]
    Asserted = 1,
}
impl From<Hartreset> for bool {
    #[inline(always)]
    fn from(variant: Hartreset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HARTRESET` reader - Reset harts."]
pub type HartresetR = crate::BitReader<Hartreset>;
impl HartresetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hartreset {
        match self.bits {
            false => Hartreset::Deasserted,
            true => Hartreset::Asserted,
        }
    }
    #[doc = "Reset de-asserted."]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        *self == Hartreset::Deasserted
    }
    #[doc = "Reset asserted."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Hartreset::Asserted
    }
}
#[doc = "Field `HARTRESET` writer - Reset harts."]
pub type HartresetW<'a, REG> = crate::BitWriter<'a, REG, Hartreset>;
impl<'a, REG> HartresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset de-asserted."]
    #[inline(always)]
    pub fn deasserted(self) -> &'a mut crate::W<REG> {
        self.variant(Hartreset::Deasserted)
    }
    #[doc = "Reset asserted."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Hartreset::Asserted)
    }
}
#[doc = "Resume currently selected harts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resumereq {
    #[doc = "0: No operation when written 0."]
    NoOperation = 0,
    #[doc = "1: Currently selected harts resumed."]
    Resumed = 1,
}
impl From<Resumereq> for bool {
    #[inline(always)]
    fn from(variant: Resumereq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESUMEREQ` writer - Resume currently selected harts."]
pub type ResumereqW<'a, REG> = crate::BitWriter<'a, REG, Resumereq>;
impl<'a, REG> ResumereqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation when written 0."]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut crate::W<REG> {
        self.variant(Resumereq::NoOperation)
    }
    #[doc = "Currently selected harts resumed."]
    #[inline(always)]
    pub fn resumed(self) -> &'a mut crate::W<REG> {
        self.variant(Resumereq::Resumed)
    }
}
#[doc = "Halt currently selected harts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Haltreq {
    #[doc = "0: Clears halt request bit for all currently selected harts."]
    Clear = 0,
    #[doc = "1: Currently selected harts halted."]
    Halt = 1,
}
impl From<Haltreq> for bool {
    #[inline(always)]
    fn from(variant: Haltreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALTREQ` writer - Halt currently selected harts."]
pub type HaltreqW<'a, REG> = crate::BitWriter<'a, REG, Haltreq>;
impl<'a, REG> HaltreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears halt request bit for all currently selected harts."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Haltreq::Clear)
    }
    #[doc = "Currently selected harts halted."]
    #[inline(always)]
    pub fn halt(self) -> &'a mut crate::W<REG> {
        self.variant(Haltreq::Halt)
    }
}
impl R {
    #[doc = "Bit 0 - Reset signal for the debug module."]
    #[inline(always)]
    pub fn dmactive(&self) -> DmactiveR {
        DmactiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset signal output from the debug module to the system."]
    #[inline(always)]
    pub fn ndmreset(&self) -> NdmresetR {
        NdmresetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 29 - Reset harts."]
    #[inline(always)]
    pub fn hartreset(&self) -> HartresetR {
        HartresetR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset signal for the debug module."]
    #[inline(always)]
    #[must_use]
    pub fn dmactive(&mut self) -> DmactiveW<DmcontrolSpec> {
        DmactiveW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset signal output from the debug module to the system."]
    #[inline(always)]
    #[must_use]
    pub fn ndmreset(&mut self) -> NdmresetW<DmcontrolSpec> {
        NdmresetW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear the halt on reset request."]
    #[inline(always)]
    #[must_use]
    pub fn clrresethaltreq(&mut self) -> ClrresethaltreqW<DmcontrolSpec> {
        ClrresethaltreqW::new(self, 2)
    }
    #[doc = "Bit 3 - Set the halt on reset request."]
    #[inline(always)]
    #[must_use]
    pub fn setresethaltreq(&mut self) -> SetresethaltreqW<DmcontrolSpec> {
        SetresethaltreqW::new(self, 3)
    }
    #[doc = "Bits 6:15 - The high 10 bits of hartsel."]
    #[inline(always)]
    #[must_use]
    pub fn hartselhi(&mut self) -> HartselhiW<DmcontrolSpec> {
        HartselhiW::new(self, 6)
    }
    #[doc = "Bits 16:25 - The low 10 bits of hartsel."]
    #[inline(always)]
    #[must_use]
    pub fn hartsello(&mut self) -> HartselloW<DmcontrolSpec> {
        HartselloW::new(self, 16)
    }
    #[doc = "Bit 26 - Definition of currently selected harts."]
    #[inline(always)]
    #[must_use]
    pub fn hasel(&mut self) -> HaselW<DmcontrolSpec> {
        HaselW::new(self, 26)
    }
    #[doc = "Bit 28 - Clear the havereset."]
    #[inline(always)]
    #[must_use]
    pub fn ackhavereset(&mut self) -> AckhaveresetW<DmcontrolSpec> {
        AckhaveresetW::new(self, 28)
    }
    #[doc = "Bit 29 - Reset harts."]
    #[inline(always)]
    #[must_use]
    pub fn hartreset(&mut self) -> HartresetW<DmcontrolSpec> {
        HartresetW::new(self, 29)
    }
    #[doc = "Bit 30 - Resume currently selected harts."]
    #[inline(always)]
    #[must_use]
    pub fn resumereq(&mut self) -> ResumereqW<DmcontrolSpec> {
        ResumereqW::new(self, 30)
    }
    #[doc = "Bit 31 - Halt currently selected harts."]
    #[inline(always)]
    #[must_use]
    pub fn haltreq(&mut self) -> HaltreqW<DmcontrolSpec> {
        HaltreqW::new(self, 31)
    }
}
#[doc = "Debug Module Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dmcontrol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcontrol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmcontrolSpec;
impl crate::RegisterSpec for DmcontrolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmcontrol::R`](R) reader structure"]
impl crate::Readable for DmcontrolSpec {}
#[doc = "`write(|w| ..)` method takes [`dmcontrol::W`](W) writer structure"]
impl crate::Writable for DmcontrolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMCONTROL to value 0"]
impl crate::Resettable for DmcontrolSpec {
    const RESET_VALUE: u32 = 0;
}
