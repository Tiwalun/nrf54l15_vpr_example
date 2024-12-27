#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Hardware flow control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hwfc {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Hwfc> for bool {
    #[inline(always)]
    fn from(variant: Hwfc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWFC` reader - Hardware flow control"]
pub type HwfcR = crate::BitReader<Hwfc>;
impl HwfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hwfc {
        match self.bits {
            false => Hwfc::Disabled,
            true => Hwfc::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hwfc::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hwfc::Enabled
    }
}
#[doc = "Field `HWFC` writer - Hardware flow control"]
pub type HwfcW<'a, REG> = crate::BitWriter<'a, REG, Hwfc>;
impl<'a, REG> HwfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hwfc::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hwfc::Enabled)
    }
}
#[doc = "Parity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Parity {
    #[doc = "0: Exclude parity bit"]
    Excluded = 0,
    #[doc = "7: Include even parity bit"]
    Included = 7,
}
impl From<Parity> for u8 {
    #[inline(always)]
    fn from(variant: Parity) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Parity {
    type Ux = u8;
}
impl crate::IsEnum for Parity {}
#[doc = "Field `PARITY` reader - Parity"]
pub type ParityR = crate::FieldReader<Parity>;
impl ParityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Parity> {
        match self.bits {
            0 => Some(Parity::Excluded),
            7 => Some(Parity::Included),
            _ => None,
        }
    }
    #[doc = "Exclude parity bit"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Parity::Excluded
    }
    #[doc = "Include even parity bit"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Parity::Included
    }
}
#[doc = "Field `PARITY` writer - Parity"]
pub type ParityW<'a, REG> = crate::FieldWriter<'a, REG, 3, Parity>;
impl<'a, REG> ParityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Exclude parity bit"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Parity::Excluded)
    }
    #[doc = "Include even parity bit"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Parity::Included)
    }
}
#[doc = "Stop bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    #[doc = "0: One stop bit"]
    One = 0,
    #[doc = "1: Two stop bits"]
    Two = 1,
}
impl From<Stop> for bool {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Stop bits"]
pub type StopR = crate::BitReader<Stop>;
impl StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop {
        match self.bits {
            false => Stop::One,
            true => Stop::Two,
        }
    }
    #[doc = "One stop bit"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Stop::One
    }
    #[doc = "Two stop bits"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Stop::Two
    }
}
#[doc = "Field `STOP` writer - Stop bits"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One stop bit"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::One)
    }
    #[doc = "Two stop bits"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::Two)
    }
}
#[doc = "Even or odd parity type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Paritytype {
    #[doc = "0: Even parity"]
    Even = 0,
    #[doc = "1: Odd parity"]
    Odd = 1,
}
impl From<Paritytype> for bool {
    #[inline(always)]
    fn from(variant: Paritytype) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARITYTYPE` reader - Even or odd parity type"]
pub type ParitytypeR = crate::BitReader<Paritytype>;
impl ParitytypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Paritytype {
        match self.bits {
            false => Paritytype::Even,
            true => Paritytype::Odd,
        }
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Paritytype::Even
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Paritytype::Odd
    }
}
#[doc = "Field `PARITYTYPE` writer - Even or odd parity type"]
pub type ParitytypeW<'a, REG> = crate::BitWriter<'a, REG, Paritytype>;
impl<'a, REG> ParitytypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Paritytype::Even)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Paritytype::Odd)
    }
}
#[doc = "Set the data frame size\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Framesize {
    #[doc = "9: 9 bit data frame size. 9th bit is treated as address bit."]
    _9bit = 9,
    #[doc = "8: 8 bit data frame size."]
    _8bit = 8,
    #[doc = "7: 7 bit data frame size."]
    _7bit = 7,
    #[doc = "6: 6 bit data frame size."]
    _6bit = 6,
    #[doc = "5: 5 bit data frame size."]
    _5bit = 5,
    #[doc = "4: 4 bit data frame size."]
    _4bit = 4,
}
impl From<Framesize> for u8 {
    #[inline(always)]
    fn from(variant: Framesize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Framesize {
    type Ux = u8;
}
impl crate::IsEnum for Framesize {}
#[doc = "Field `FRAMESIZE` reader - Set the data frame size"]
pub type FramesizeR = crate::FieldReader<Framesize>;
impl FramesizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Framesize> {
        match self.bits {
            9 => Some(Framesize::_9bit),
            8 => Some(Framesize::_8bit),
            7 => Some(Framesize::_7bit),
            6 => Some(Framesize::_6bit),
            5 => Some(Framesize::_5bit),
            4 => Some(Framesize::_4bit),
            _ => None,
        }
    }
    #[doc = "9 bit data frame size. 9th bit is treated as address bit."]
    #[inline(always)]
    pub fn is_9bit(&self) -> bool {
        *self == Framesize::_9bit
    }
    #[doc = "8 bit data frame size."]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Framesize::_8bit
    }
    #[doc = "7 bit data frame size."]
    #[inline(always)]
    pub fn is_7bit(&self) -> bool {
        *self == Framesize::_7bit
    }
    #[doc = "6 bit data frame size."]
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == Framesize::_6bit
    }
    #[doc = "5 bit data frame size."]
    #[inline(always)]
    pub fn is_5bit(&self) -> bool {
        *self == Framesize::_5bit
    }
    #[doc = "4 bit data frame size."]
    #[inline(always)]
    pub fn is_4bit(&self) -> bool {
        *self == Framesize::_4bit
    }
}
#[doc = "Field `FRAMESIZE` writer - Set the data frame size"]
pub type FramesizeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Framesize>;
impl<'a, REG> FramesizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "9 bit data frame size. 9th bit is treated as address bit."]
    #[inline(always)]
    pub fn _9bit(self) -> &'a mut crate::W<REG> {
        self.variant(Framesize::_9bit)
    }
    #[doc = "8 bit data frame size."]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Framesize::_8bit)
    }
    #[doc = "7 bit data frame size."]
    #[inline(always)]
    pub fn _7bit(self) -> &'a mut crate::W<REG> {
        self.variant(Framesize::_7bit)
    }
    #[doc = "6 bit data frame size."]
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut crate::W<REG> {
        self.variant(Framesize::_6bit)
    }
    #[doc = "5 bit data frame size."]
    #[inline(always)]
    pub fn _5bit(self) -> &'a mut crate::W<REG> {
        self.variant(Framesize::_5bit)
    }
    #[doc = "4 bit data frame size."]
    #[inline(always)]
    pub fn _4bit(self) -> &'a mut crate::W<REG> {
        self.variant(Framesize::_4bit)
    }
}
#[doc = "Select if data is trimmed from MSB or LSB end when the data frame size is less than 8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endian {
    #[doc = "0: Data is trimmed from MSB end."]
    Msb = 0,
    #[doc = "1: Data is trimmed from LSB end."]
    Lsb = 1,
}
impl From<Endian> for bool {
    #[inline(always)]
    fn from(variant: Endian) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDIAN` reader - Select if data is trimmed from MSB or LSB end when the data frame size is less than 8."]
pub type EndianR = crate::BitReader<Endian>;
impl EndianR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endian {
        match self.bits {
            false => Endian::Msb,
            true => Endian::Lsb,
        }
    }
    #[doc = "Data is trimmed from MSB end."]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == Endian::Msb
    }
    #[doc = "Data is trimmed from LSB end."]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == Endian::Lsb
    }
}
#[doc = "Field `ENDIAN` writer - Select if data is trimmed from MSB or LSB end when the data frame size is less than 8."]
pub type EndianW<'a, REG> = crate::BitWriter<'a, REG, Endian>;
impl<'a, REG> EndianW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is trimmed from MSB end."]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::Msb)
    }
    #[doc = "Data is trimmed from LSB end."]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::Lsb)
    }
}
#[doc = "Enable packet timeout.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frametimeout {
    #[doc = "0: Packet timeout is disabled."]
    Disabled = 0,
    #[doc = "1: Packet timeout is enabled."]
    Enabled = 1,
}
impl From<Frametimeout> for bool {
    #[inline(always)]
    fn from(variant: Frametimeout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAMETIMEOUT` reader - Enable packet timeout."]
pub type FrametimeoutR = crate::BitReader<Frametimeout>;
impl FrametimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frametimeout {
        match self.bits {
            false => Frametimeout::Disabled,
            true => Frametimeout::Enabled,
        }
    }
    #[doc = "Packet timeout is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Frametimeout::Disabled
    }
    #[doc = "Packet timeout is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Frametimeout::Enabled
    }
}
#[doc = "Field `FRAMETIMEOUT` writer - Enable packet timeout."]
pub type FrametimeoutW<'a, REG> = crate::BitWriter<'a, REG, Frametimeout>;
impl<'a, REG> FrametimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Packet timeout is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Frametimeout::Disabled)
    }
    #[doc = "Packet timeout is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Frametimeout::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Hardware flow control"]
    #[inline(always)]
    pub fn hwfc(&self) -> HwfcR {
        HwfcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Parity"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Stop bits"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Even or odd parity type"]
    #[inline(always)]
    pub fn paritytype(&self) -> ParitytypeR {
        ParitytypeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:12 - Set the data frame size"]
    #[inline(always)]
    pub fn framesize(&self) -> FramesizeR {
        FramesizeR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Select if data is trimmed from MSB or LSB end when the data frame size is less than 8."]
    #[inline(always)]
    pub fn endian(&self) -> EndianR {
        EndianR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable packet timeout."]
    #[inline(always)]
    pub fn frametimeout(&self) -> FrametimeoutR {
        FrametimeoutR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware flow control"]
    #[inline(always)]
    #[must_use]
    pub fn hwfc(&mut self) -> HwfcW<ConfigSpec> {
        HwfcW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Parity"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> ParityW<ConfigSpec> {
        ParityW::new(self, 1)
    }
    #[doc = "Bit 4 - Stop bits"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<ConfigSpec> {
        StopW::new(self, 4)
    }
    #[doc = "Bit 8 - Even or odd parity type"]
    #[inline(always)]
    #[must_use]
    pub fn paritytype(&mut self) -> ParitytypeW<ConfigSpec> {
        ParitytypeW::new(self, 8)
    }
    #[doc = "Bits 9:12 - Set the data frame size"]
    #[inline(always)]
    #[must_use]
    pub fn framesize(&mut self) -> FramesizeW<ConfigSpec> {
        FramesizeW::new(self, 9)
    }
    #[doc = "Bit 13 - Select if data is trimmed from MSB or LSB end when the data frame size is less than 8."]
    #[inline(always)]
    #[must_use]
    pub fn endian(&mut self) -> EndianW<ConfigSpec> {
        EndianW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable packet timeout."]
    #[inline(always)]
    #[must_use]
    pub fn frametimeout(&mut self) -> FrametimeoutW<ConfigSpec> {
        FrametimeoutW::new(self, 14)
    }
}
#[doc = "Configuration of parity, hardware flow control, framesize, and packet timeout.\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0x1000"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x1000;
}
