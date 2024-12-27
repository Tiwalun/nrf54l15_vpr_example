#[doc = "Register `CLICINT[%s]` reader"]
pub type R = crate::R<ClicintSpec>;
#[doc = "Register `CLICINT[%s]` writer"]
pub type W = crate::W<ClicintSpec>;
#[doc = "Interrupt Pending bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ip {
    #[doc = "0: Interrupt not pending"]
    NotPending = 0,
    #[doc = "1: Interrupt pending"]
    Pending = 1,
}
impl From<Ip> for bool {
    #[inline(always)]
    fn from(variant: Ip) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IP` reader - Interrupt Pending bit."]
pub type IpR = crate::BitReader<Ip>;
impl IpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ip {
        match self.bits {
            false => Ip::NotPending,
            true => Ip::Pending,
        }
    }
    #[doc = "Interrupt not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Ip::NotPending
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Ip::Pending
    }
}
#[doc = "Field `IP` writer - Interrupt Pending bit."]
pub type IpW<'a, REG> = crate::BitWriter<'a, REG, Ip>;
impl<'a, REG> IpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt not pending"]
    #[inline(always)]
    pub fn not_pending(self) -> &'a mut crate::W<REG> {
        self.variant(Ip::NotPending)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(Ip::Pending)
    }
}
#[doc = "Field `READ1` reader - Read as 0, write ignored."]
pub type Read1R = crate::FieldReader;
#[doc = "Interrupt enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ie {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<Ie> for bool {
    #[inline(always)]
    fn from(variant: Ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE` reader - Interrupt enable bit."]
pub type IeR = crate::BitReader<Ie>;
impl IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ie {
        match self.bits {
            false => Ie::Disabled,
            true => Ie::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ie::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ie::Enabled
    }
}
#[doc = "Field `IE` writer - Interrupt enable bit."]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG, Ie>;
impl<'a, REG> IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ie::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ie::Enabled)
    }
}
#[doc = "Field `READ2` reader - Read as 0, write ignored."]
pub type Read2R = crate::FieldReader;
#[doc = "Selective Hardware Vectoring.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shv {
    #[doc = "1: Hardware vectored"]
    Vectored = 1,
}
impl From<Shv> for bool {
    #[inline(always)]
    fn from(variant: Shv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHV` reader - Selective Hardware Vectoring."]
pub type ShvR = crate::BitReader<Shv>;
impl ShvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Shv> {
        match self.bits {
            true => Some(Shv::Vectored),
            _ => None,
        }
    }
    #[doc = "Hardware vectored"]
    #[inline(always)]
    pub fn is_vectored(&self) -> bool {
        *self == Shv::Vectored
    }
}
#[doc = "Trigger type and polarity for each interrupt input.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trig {
    #[doc = "1: Interrupts are edge-triggered"]
    EdgeTriggered = 1,
}
impl From<Trig> for u8 {
    #[inline(always)]
    fn from(variant: Trig) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trig {
    type Ux = u8;
}
impl crate::IsEnum for Trig {}
#[doc = "Field `TRIG` reader - Trigger type and polarity for each interrupt input."]
pub type TrigR = crate::FieldReader<Trig>;
impl TrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trig> {
        match self.bits {
            1 => Some(Trig::EdgeTriggered),
            _ => None,
        }
    }
    #[doc = "Interrupts are edge-triggered"]
    #[inline(always)]
    pub fn is_edge_triggered(&self) -> bool {
        *self == Trig::EdgeTriggered
    }
}
#[doc = "Privilege mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "3: Machine mode"]
    MachineMode = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Privilege mode."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            3 => Some(Mode::MachineMode),
            _ => None,
        }
    }
    #[doc = "Machine mode"]
    #[inline(always)]
    pub fn is_machine_mode(&self) -> bool {
        *self == Mode::MachineMode
    }
}
#[doc = "Interrupt priority level\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Priority {
    #[doc = "63: Priority level 0"]
    Priolevel0 = 63,
    #[doc = "127: Priority level 1"]
    Priolevel1 = 127,
    #[doc = "191: Priority level 2"]
    Priolevel2 = 191,
    #[doc = "255: Priority level 3"]
    Priolevel3 = 255,
}
impl From<Priority> for u8 {
    #[inline(always)]
    fn from(variant: Priority) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Priority {
    type Ux = u8;
}
impl crate::IsEnum for Priority {}
#[doc = "Field `PRIORITY` reader - Interrupt priority level"]
pub type PriorityR = crate::FieldReader<Priority>;
impl PriorityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Priority> {
        match self.bits {
            63 => Some(Priority::Priolevel0),
            127 => Some(Priority::Priolevel1),
            191 => Some(Priority::Priolevel2),
            255 => Some(Priority::Priolevel3),
            _ => None,
        }
    }
    #[doc = "Priority level 0"]
    #[inline(always)]
    pub fn is_priolevel0(&self) -> bool {
        *self == Priority::Priolevel0
    }
    #[doc = "Priority level 1"]
    #[inline(always)]
    pub fn is_priolevel1(&self) -> bool {
        *self == Priority::Priolevel1
    }
    #[doc = "Priority level 2"]
    #[inline(always)]
    pub fn is_priolevel2(&self) -> bool {
        *self == Priority::Priolevel2
    }
    #[doc = "Priority level 3"]
    #[inline(always)]
    pub fn is_priolevel3(&self) -> bool {
        *self == Priority::Priolevel3
    }
}
#[doc = "Field `PRIORITY` writer - Interrupt priority level"]
pub type PriorityW<'a, REG> = crate::FieldWriter<'a, REG, 8, Priority>;
impl<'a, REG> PriorityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Priority level 0"]
    #[inline(always)]
    pub fn priolevel0(self) -> &'a mut crate::W<REG> {
        self.variant(Priority::Priolevel0)
    }
    #[doc = "Priority level 1"]
    #[inline(always)]
    pub fn priolevel1(self) -> &'a mut crate::W<REG> {
        self.variant(Priority::Priolevel1)
    }
    #[doc = "Priority level 2"]
    #[inline(always)]
    pub fn priolevel2(self) -> &'a mut crate::W<REG> {
        self.variant(Priority::Priolevel2)
    }
    #[doc = "Priority level 3"]
    #[inline(always)]
    pub fn priolevel3(self) -> &'a mut crate::W<REG> {
        self.variant(Priority::Priolevel3)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Pending bit."]
    #[inline(always)]
    pub fn ip(&self) -> IpR {
        IpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Read as 0, write ignored."]
    #[inline(always)]
    pub fn read1(&self) -> Read1R {
        Read1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Interrupt enable bit."]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Read as 0, write ignored."]
    #[inline(always)]
    pub fn read2(&self) -> Read2R {
        Read2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Selective Hardware Vectoring."]
    #[inline(always)]
    pub fn shv(&self) -> ShvR {
        ShvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Trigger type and polarity for each interrupt input."]
    #[inline(always)]
    pub fn trig(&self) -> TrigR {
        TrigR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Privilege mode."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt priority level"]
    #[inline(always)]
    pub fn priority(&self) -> PriorityR {
        PriorityR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn ip(&mut self) -> IpW<ClicintSpec> {
        IpW::new(self, 0)
    }
    #[doc = "Bit 8 - Interrupt enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IeW<ClicintSpec> {
        IeW::new(self, 8)
    }
    #[doc = "Bits 24:31 - Interrupt priority level"]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PriorityW<ClicintSpec> {
        PriorityW::new(self, 24)
    }
}
#[doc = "Description collection: Interrupt control register for IRQ number \\[n\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`clicint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clicint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClicintSpec;
impl crate::RegisterSpec for ClicintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clicint::R`](R) reader structure"]
impl crate::Readable for ClicintSpec {}
#[doc = "`write(|w| ..)` method takes [`clicint::W`](W) writer structure"]
impl crate::Writable for ClicintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLICINT[%s]
to value 0x3fc3_0000"]
impl crate::Resettable for ClicintSpec {
    const RESET_VALUE: u32 = 0x3fc3_0000;
}
