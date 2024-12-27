#[doc = "Register `RET2` reader"]
pub type R = crate::R<Ret2Spec>;
#[doc = "Register `RET2` writer"]
pub type W = crate::W<Ret2Spec>;
#[doc = "Keep the second bank in RAM block MEM\\[0\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem0 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem0> for bool {
    #[inline(always)]
    fn from(variant: Mem0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM0` reader - Keep the second bank in RAM block MEM\\[0\\]
retained when in System OFF mode."]
pub type Mem0R = crate::BitReader<Mem0>;
impl Mem0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem0 {
        match self.bits {
            false => Mem0::Off,
            true => Mem0::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem0::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem0::On
    }
}
#[doc = "Field `MEM0` writer - Keep the second bank in RAM block MEM\\[0\\]
retained when in System OFF mode."]
pub type Mem0W<'a, REG> = crate::BitWriter<'a, REG, Mem0>;
impl<'a, REG> Mem0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem0::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem0::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[1\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem1 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem1> for bool {
    #[inline(always)]
    fn from(variant: Mem1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM1` reader - Keep the second bank in RAM block MEM\\[1\\]
retained when in System OFF mode."]
pub type Mem1R = crate::BitReader<Mem1>;
impl Mem1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem1 {
        match self.bits {
            false => Mem1::Off,
            true => Mem1::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem1::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem1::On
    }
}
#[doc = "Field `MEM1` writer - Keep the second bank in RAM block MEM\\[1\\]
retained when in System OFF mode."]
pub type Mem1W<'a, REG> = crate::BitWriter<'a, REG, Mem1>;
impl<'a, REG> Mem1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem1::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem1::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[2\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem2 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem2> for bool {
    #[inline(always)]
    fn from(variant: Mem2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM2` reader - Keep the second bank in RAM block MEM\\[2\\]
retained when in System OFF mode."]
pub type Mem2R = crate::BitReader<Mem2>;
impl Mem2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem2 {
        match self.bits {
            false => Mem2::Off,
            true => Mem2::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem2::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem2::On
    }
}
#[doc = "Field `MEM2` writer - Keep the second bank in RAM block MEM\\[2\\]
retained when in System OFF mode."]
pub type Mem2W<'a, REG> = crate::BitWriter<'a, REG, Mem2>;
impl<'a, REG> Mem2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem2::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem2::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[3\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem3 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem3> for bool {
    #[inline(always)]
    fn from(variant: Mem3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM3` reader - Keep the second bank in RAM block MEM\\[3\\]
retained when in System OFF mode."]
pub type Mem3R = crate::BitReader<Mem3>;
impl Mem3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem3 {
        match self.bits {
            false => Mem3::Off,
            true => Mem3::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem3::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem3::On
    }
}
#[doc = "Field `MEM3` writer - Keep the second bank in RAM block MEM\\[3\\]
retained when in System OFF mode."]
pub type Mem3W<'a, REG> = crate::BitWriter<'a, REG, Mem3>;
impl<'a, REG> Mem3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem3::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem3::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[4\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem4 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem4> for bool {
    #[inline(always)]
    fn from(variant: Mem4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM4` reader - Keep the second bank in RAM block MEM\\[4\\]
retained when in System OFF mode."]
pub type Mem4R = crate::BitReader<Mem4>;
impl Mem4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem4 {
        match self.bits {
            false => Mem4::Off,
            true => Mem4::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem4::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem4::On
    }
}
#[doc = "Field `MEM4` writer - Keep the second bank in RAM block MEM\\[4\\]
retained when in System OFF mode."]
pub type Mem4W<'a, REG> = crate::BitWriter<'a, REG, Mem4>;
impl<'a, REG> Mem4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem4::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem4::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[5\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem5 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem5> for bool {
    #[inline(always)]
    fn from(variant: Mem5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM5` reader - Keep the second bank in RAM block MEM\\[5\\]
retained when in System OFF mode."]
pub type Mem5R = crate::BitReader<Mem5>;
impl Mem5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem5 {
        match self.bits {
            false => Mem5::Off,
            true => Mem5::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem5::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem5::On
    }
}
#[doc = "Field `MEM5` writer - Keep the second bank in RAM block MEM\\[5\\]
retained when in System OFF mode."]
pub type Mem5W<'a, REG> = crate::BitWriter<'a, REG, Mem5>;
impl<'a, REG> Mem5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem5::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem5::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[6\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem6 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem6> for bool {
    #[inline(always)]
    fn from(variant: Mem6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM6` reader - Keep the second bank in RAM block MEM\\[6\\]
retained when in System OFF mode."]
pub type Mem6R = crate::BitReader<Mem6>;
impl Mem6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem6 {
        match self.bits {
            false => Mem6::Off,
            true => Mem6::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem6::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem6::On
    }
}
#[doc = "Field `MEM6` writer - Keep the second bank in RAM block MEM\\[6\\]
retained when in System OFF mode."]
pub type Mem6W<'a, REG> = crate::BitWriter<'a, REG, Mem6>;
impl<'a, REG> Mem6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem6::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem6::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[7\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem7 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem7> for bool {
    #[inline(always)]
    fn from(variant: Mem7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM7` reader - Keep the second bank in RAM block MEM\\[7\\]
retained when in System OFF mode."]
pub type Mem7R = crate::BitReader<Mem7>;
impl Mem7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem7 {
        match self.bits {
            false => Mem7::Off,
            true => Mem7::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem7::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem7::On
    }
}
#[doc = "Field `MEM7` writer - Keep the second bank in RAM block MEM\\[7\\]
retained when in System OFF mode."]
pub type Mem7W<'a, REG> = crate::BitWriter<'a, REG, Mem7>;
impl<'a, REG> Mem7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem7::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem7::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[8\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem8 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem8> for bool {
    #[inline(always)]
    fn from(variant: Mem8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM8` reader - Keep the second bank in RAM block MEM\\[8\\]
retained when in System OFF mode."]
pub type Mem8R = crate::BitReader<Mem8>;
impl Mem8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem8 {
        match self.bits {
            false => Mem8::Off,
            true => Mem8::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem8::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem8::On
    }
}
#[doc = "Field `MEM8` writer - Keep the second bank in RAM block MEM\\[8\\]
retained when in System OFF mode."]
pub type Mem8W<'a, REG> = crate::BitWriter<'a, REG, Mem8>;
impl<'a, REG> Mem8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem8::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem8::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[9\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem9 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem9> for bool {
    #[inline(always)]
    fn from(variant: Mem9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM9` reader - Keep the second bank in RAM block MEM\\[9\\]
retained when in System OFF mode."]
pub type Mem9R = crate::BitReader<Mem9>;
impl Mem9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem9 {
        match self.bits {
            false => Mem9::Off,
            true => Mem9::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem9::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem9::On
    }
}
#[doc = "Field `MEM9` writer - Keep the second bank in RAM block MEM\\[9\\]
retained when in System OFF mode."]
pub type Mem9W<'a, REG> = crate::BitWriter<'a, REG, Mem9>;
impl<'a, REG> Mem9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem9::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem9::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[10\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem10 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem10> for bool {
    #[inline(always)]
    fn from(variant: Mem10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM10` reader - Keep the second bank in RAM block MEM\\[10\\]
retained when in System OFF mode."]
pub type Mem10R = crate::BitReader<Mem10>;
impl Mem10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem10 {
        match self.bits {
            false => Mem10::Off,
            true => Mem10::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem10::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem10::On
    }
}
#[doc = "Field `MEM10` writer - Keep the second bank in RAM block MEM\\[10\\]
retained when in System OFF mode."]
pub type Mem10W<'a, REG> = crate::BitWriter<'a, REG, Mem10>;
impl<'a, REG> Mem10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem10::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem10::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[11\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem11 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem11> for bool {
    #[inline(always)]
    fn from(variant: Mem11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM11` reader - Keep the second bank in RAM block MEM\\[11\\]
retained when in System OFF mode."]
pub type Mem11R = crate::BitReader<Mem11>;
impl Mem11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem11 {
        match self.bits {
            false => Mem11::Off,
            true => Mem11::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem11::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem11::On
    }
}
#[doc = "Field `MEM11` writer - Keep the second bank in RAM block MEM\\[11\\]
retained when in System OFF mode."]
pub type Mem11W<'a, REG> = crate::BitWriter<'a, REG, Mem11>;
impl<'a, REG> Mem11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem11::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem11::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[12\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem12 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem12> for bool {
    #[inline(always)]
    fn from(variant: Mem12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM12` reader - Keep the second bank in RAM block MEM\\[12\\]
retained when in System OFF mode."]
pub type Mem12R = crate::BitReader<Mem12>;
impl Mem12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem12 {
        match self.bits {
            false => Mem12::Off,
            true => Mem12::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem12::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem12::On
    }
}
#[doc = "Field `MEM12` writer - Keep the second bank in RAM block MEM\\[12\\]
retained when in System OFF mode."]
pub type Mem12W<'a, REG> = crate::BitWriter<'a, REG, Mem12>;
impl<'a, REG> Mem12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem12::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem12::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[13\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem13 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem13> for bool {
    #[inline(always)]
    fn from(variant: Mem13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM13` reader - Keep the second bank in RAM block MEM\\[13\\]
retained when in System OFF mode."]
pub type Mem13R = crate::BitReader<Mem13>;
impl Mem13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem13 {
        match self.bits {
            false => Mem13::Off,
            true => Mem13::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem13::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem13::On
    }
}
#[doc = "Field `MEM13` writer - Keep the second bank in RAM block MEM\\[13\\]
retained when in System OFF mode."]
pub type Mem13W<'a, REG> = crate::BitWriter<'a, REG, Mem13>;
impl<'a, REG> Mem13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem13::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem13::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[14\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem14 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem14> for bool {
    #[inline(always)]
    fn from(variant: Mem14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM14` reader - Keep the second bank in RAM block MEM\\[14\\]
retained when in System OFF mode."]
pub type Mem14R = crate::BitReader<Mem14>;
impl Mem14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem14 {
        match self.bits {
            false => Mem14::Off,
            true => Mem14::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem14::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem14::On
    }
}
#[doc = "Field `MEM14` writer - Keep the second bank in RAM block MEM\\[14\\]
retained when in System OFF mode."]
pub type Mem14W<'a, REG> = crate::BitWriter<'a, REG, Mem14>;
impl<'a, REG> Mem14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem14::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem14::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[15\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem15 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem15> for bool {
    #[inline(always)]
    fn from(variant: Mem15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM15` reader - Keep the second bank in RAM block MEM\\[15\\]
retained when in System OFF mode."]
pub type Mem15R = crate::BitReader<Mem15>;
impl Mem15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem15 {
        match self.bits {
            false => Mem15::Off,
            true => Mem15::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem15::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem15::On
    }
}
#[doc = "Field `MEM15` writer - Keep the second bank in RAM block MEM\\[15\\]
retained when in System OFF mode."]
pub type Mem15W<'a, REG> = crate::BitWriter<'a, REG, Mem15>;
impl<'a, REG> Mem15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem15::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem15::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[16\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem16 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem16> for bool {
    #[inline(always)]
    fn from(variant: Mem16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM16` reader - Keep the second bank in RAM block MEM\\[16\\]
retained when in System OFF mode."]
pub type Mem16R = crate::BitReader<Mem16>;
impl Mem16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem16 {
        match self.bits {
            false => Mem16::Off,
            true => Mem16::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem16::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem16::On
    }
}
#[doc = "Field `MEM16` writer - Keep the second bank in RAM block MEM\\[16\\]
retained when in System OFF mode."]
pub type Mem16W<'a, REG> = crate::BitWriter<'a, REG, Mem16>;
impl<'a, REG> Mem16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem16::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem16::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[17\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem17 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem17> for bool {
    #[inline(always)]
    fn from(variant: Mem17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM17` reader - Keep the second bank in RAM block MEM\\[17\\]
retained when in System OFF mode."]
pub type Mem17R = crate::BitReader<Mem17>;
impl Mem17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem17 {
        match self.bits {
            false => Mem17::Off,
            true => Mem17::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem17::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem17::On
    }
}
#[doc = "Field `MEM17` writer - Keep the second bank in RAM block MEM\\[17\\]
retained when in System OFF mode."]
pub type Mem17W<'a, REG> = crate::BitWriter<'a, REG, Mem17>;
impl<'a, REG> Mem17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem17::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem17::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[18\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem18 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem18> for bool {
    #[inline(always)]
    fn from(variant: Mem18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM18` reader - Keep the second bank in RAM block MEM\\[18\\]
retained when in System OFF mode."]
pub type Mem18R = crate::BitReader<Mem18>;
impl Mem18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem18 {
        match self.bits {
            false => Mem18::Off,
            true => Mem18::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem18::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem18::On
    }
}
#[doc = "Field `MEM18` writer - Keep the second bank in RAM block MEM\\[18\\]
retained when in System OFF mode."]
pub type Mem18W<'a, REG> = crate::BitWriter<'a, REG, Mem18>;
impl<'a, REG> Mem18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem18::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem18::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[19\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem19 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem19> for bool {
    #[inline(always)]
    fn from(variant: Mem19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM19` reader - Keep the second bank in RAM block MEM\\[19\\]
retained when in System OFF mode."]
pub type Mem19R = crate::BitReader<Mem19>;
impl Mem19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem19 {
        match self.bits {
            false => Mem19::Off,
            true => Mem19::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem19::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem19::On
    }
}
#[doc = "Field `MEM19` writer - Keep the second bank in RAM block MEM\\[19\\]
retained when in System OFF mode."]
pub type Mem19W<'a, REG> = crate::BitWriter<'a, REG, Mem19>;
impl<'a, REG> Mem19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem19::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem19::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[20\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem20 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem20> for bool {
    #[inline(always)]
    fn from(variant: Mem20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM20` reader - Keep the second bank in RAM block MEM\\[20\\]
retained when in System OFF mode."]
pub type Mem20R = crate::BitReader<Mem20>;
impl Mem20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem20 {
        match self.bits {
            false => Mem20::Off,
            true => Mem20::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem20::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem20::On
    }
}
#[doc = "Field `MEM20` writer - Keep the second bank in RAM block MEM\\[20\\]
retained when in System OFF mode."]
pub type Mem20W<'a, REG> = crate::BitWriter<'a, REG, Mem20>;
impl<'a, REG> Mem20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem20::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem20::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[21\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem21 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem21> for bool {
    #[inline(always)]
    fn from(variant: Mem21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM21` reader - Keep the second bank in RAM block MEM\\[21\\]
retained when in System OFF mode."]
pub type Mem21R = crate::BitReader<Mem21>;
impl Mem21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem21 {
        match self.bits {
            false => Mem21::Off,
            true => Mem21::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem21::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem21::On
    }
}
#[doc = "Field `MEM21` writer - Keep the second bank in RAM block MEM\\[21\\]
retained when in System OFF mode."]
pub type Mem21W<'a, REG> = crate::BitWriter<'a, REG, Mem21>;
impl<'a, REG> Mem21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem21::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem21::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[22\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem22 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem22> for bool {
    #[inline(always)]
    fn from(variant: Mem22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM22` reader - Keep the second bank in RAM block MEM\\[22\\]
retained when in System OFF mode."]
pub type Mem22R = crate::BitReader<Mem22>;
impl Mem22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem22 {
        match self.bits {
            false => Mem22::Off,
            true => Mem22::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem22::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem22::On
    }
}
#[doc = "Field `MEM22` writer - Keep the second bank in RAM block MEM\\[22\\]
retained when in System OFF mode."]
pub type Mem22W<'a, REG> = crate::BitWriter<'a, REG, Mem22>;
impl<'a, REG> Mem22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem22::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem22::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[23\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem23 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem23> for bool {
    #[inline(always)]
    fn from(variant: Mem23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM23` reader - Keep the second bank in RAM block MEM\\[23\\]
retained when in System OFF mode."]
pub type Mem23R = crate::BitReader<Mem23>;
impl Mem23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem23 {
        match self.bits {
            false => Mem23::Off,
            true => Mem23::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem23::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem23::On
    }
}
#[doc = "Field `MEM23` writer - Keep the second bank in RAM block MEM\\[23\\]
retained when in System OFF mode."]
pub type Mem23W<'a, REG> = crate::BitWriter<'a, REG, Mem23>;
impl<'a, REG> Mem23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem23::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem23::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[24\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem24 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem24> for bool {
    #[inline(always)]
    fn from(variant: Mem24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM24` reader - Keep the second bank in RAM block MEM\\[24\\]
retained when in System OFF mode."]
pub type Mem24R = crate::BitReader<Mem24>;
impl Mem24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem24 {
        match self.bits {
            false => Mem24::Off,
            true => Mem24::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem24::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem24::On
    }
}
#[doc = "Field `MEM24` writer - Keep the second bank in RAM block MEM\\[24\\]
retained when in System OFF mode."]
pub type Mem24W<'a, REG> = crate::BitWriter<'a, REG, Mem24>;
impl<'a, REG> Mem24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem24::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem24::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[25\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem25 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem25> for bool {
    #[inline(always)]
    fn from(variant: Mem25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM25` reader - Keep the second bank in RAM block MEM\\[25\\]
retained when in System OFF mode."]
pub type Mem25R = crate::BitReader<Mem25>;
impl Mem25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem25 {
        match self.bits {
            false => Mem25::Off,
            true => Mem25::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem25::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem25::On
    }
}
#[doc = "Field `MEM25` writer - Keep the second bank in RAM block MEM\\[25\\]
retained when in System OFF mode."]
pub type Mem25W<'a, REG> = crate::BitWriter<'a, REG, Mem25>;
impl<'a, REG> Mem25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem25::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem25::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[26\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem26 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem26> for bool {
    #[inline(always)]
    fn from(variant: Mem26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM26` reader - Keep the second bank in RAM block MEM\\[26\\]
retained when in System OFF mode."]
pub type Mem26R = crate::BitReader<Mem26>;
impl Mem26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem26 {
        match self.bits {
            false => Mem26::Off,
            true => Mem26::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem26::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem26::On
    }
}
#[doc = "Field `MEM26` writer - Keep the second bank in RAM block MEM\\[26\\]
retained when in System OFF mode."]
pub type Mem26W<'a, REG> = crate::BitWriter<'a, REG, Mem26>;
impl<'a, REG> Mem26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem26::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem26::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[27\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem27 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem27> for bool {
    #[inline(always)]
    fn from(variant: Mem27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM27` reader - Keep the second bank in RAM block MEM\\[27\\]
retained when in System OFF mode."]
pub type Mem27R = crate::BitReader<Mem27>;
impl Mem27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem27 {
        match self.bits {
            false => Mem27::Off,
            true => Mem27::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem27::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem27::On
    }
}
#[doc = "Field `MEM27` writer - Keep the second bank in RAM block MEM\\[27\\]
retained when in System OFF mode."]
pub type Mem27W<'a, REG> = crate::BitWriter<'a, REG, Mem27>;
impl<'a, REG> Mem27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem27::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem27::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[28\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem28 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem28> for bool {
    #[inline(always)]
    fn from(variant: Mem28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM28` reader - Keep the second bank in RAM block MEM\\[28\\]
retained when in System OFF mode."]
pub type Mem28R = crate::BitReader<Mem28>;
impl Mem28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem28 {
        match self.bits {
            false => Mem28::Off,
            true => Mem28::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem28::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem28::On
    }
}
#[doc = "Field `MEM28` writer - Keep the second bank in RAM block MEM\\[28\\]
retained when in System OFF mode."]
pub type Mem28W<'a, REG> = crate::BitWriter<'a, REG, Mem28>;
impl<'a, REG> Mem28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem28::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem28::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[29\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem29 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem29> for bool {
    #[inline(always)]
    fn from(variant: Mem29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM29` reader - Keep the second bank in RAM block MEM\\[29\\]
retained when in System OFF mode."]
pub type Mem29R = crate::BitReader<Mem29>;
impl Mem29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem29 {
        match self.bits {
            false => Mem29::Off,
            true => Mem29::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem29::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem29::On
    }
}
#[doc = "Field `MEM29` writer - Keep the second bank in RAM block MEM\\[29\\]
retained when in System OFF mode."]
pub type Mem29W<'a, REG> = crate::BitWriter<'a, REG, Mem29>;
impl<'a, REG> Mem29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem29::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem29::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[30\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem30 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem30> for bool {
    #[inline(always)]
    fn from(variant: Mem30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM30` reader - Keep the second bank in RAM block MEM\\[30\\]
retained when in System OFF mode."]
pub type Mem30R = crate::BitReader<Mem30>;
impl Mem30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem30 {
        match self.bits {
            false => Mem30::Off,
            true => Mem30::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem30::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem30::On
    }
}
#[doc = "Field `MEM30` writer - Keep the second bank in RAM block MEM\\[30\\]
retained when in System OFF mode."]
pub type Mem30W<'a, REG> = crate::BitWriter<'a, REG, Mem30>;
impl<'a, REG> Mem30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem30::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem30::On)
    }
}
#[doc = "Keep the second bank in RAM block MEM\\[31\\]
retained when in System OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem31 {
    #[doc = "0: Retention off"]
    Off = 0,
    #[doc = "1: Retention on"]
    On = 1,
}
impl From<Mem31> for bool {
    #[inline(always)]
    fn from(variant: Mem31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM31` reader - Keep the second bank in RAM block MEM\\[31\\]
retained when in System OFF mode."]
pub type Mem31R = crate::BitReader<Mem31>;
impl Mem31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem31 {
        match self.bits {
            false => Mem31::Off,
            true => Mem31::On,
        }
    }
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mem31::Off
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Mem31::On
    }
}
#[doc = "Field `MEM31` writer - Keep the second bank in RAM block MEM\\[31\\]
retained when in System OFF mode."]
pub type Mem31W<'a, REG> = crate::BitWriter<'a, REG, Mem31>;
impl<'a, REG> Mem31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Retention off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mem31::Off)
    }
    #[doc = "Retention on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Mem31::On)
    }
}
impl R {
    #[doc = "Bit 0 - Keep the second bank in RAM block MEM\\[0\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem0(&self) -> Mem0R {
        Mem0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Keep the second bank in RAM block MEM\\[1\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem1(&self) -> Mem1R {
        Mem1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Keep the second bank in RAM block MEM\\[2\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem2(&self) -> Mem2R {
        Mem2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Keep the second bank in RAM block MEM\\[3\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem3(&self) -> Mem3R {
        Mem3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Keep the second bank in RAM block MEM\\[4\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem4(&self) -> Mem4R {
        Mem4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Keep the second bank in RAM block MEM\\[5\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem5(&self) -> Mem5R {
        Mem5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Keep the second bank in RAM block MEM\\[6\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem6(&self) -> Mem6R {
        Mem6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Keep the second bank in RAM block MEM\\[7\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem7(&self) -> Mem7R {
        Mem7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Keep the second bank in RAM block MEM\\[8\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem8(&self) -> Mem8R {
        Mem8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Keep the second bank in RAM block MEM\\[9\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem9(&self) -> Mem9R {
        Mem9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Keep the second bank in RAM block MEM\\[10\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem10(&self) -> Mem10R {
        Mem10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Keep the second bank in RAM block MEM\\[11\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem11(&self) -> Mem11R {
        Mem11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Keep the second bank in RAM block MEM\\[12\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem12(&self) -> Mem12R {
        Mem12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Keep the second bank in RAM block MEM\\[13\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem13(&self) -> Mem13R {
        Mem13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Keep the second bank in RAM block MEM\\[14\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem14(&self) -> Mem14R {
        Mem14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Keep the second bank in RAM block MEM\\[15\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem15(&self) -> Mem15R {
        Mem15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Keep the second bank in RAM block MEM\\[16\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem16(&self) -> Mem16R {
        Mem16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Keep the second bank in RAM block MEM\\[17\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem17(&self) -> Mem17R {
        Mem17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Keep the second bank in RAM block MEM\\[18\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem18(&self) -> Mem18R {
        Mem18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Keep the second bank in RAM block MEM\\[19\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem19(&self) -> Mem19R {
        Mem19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Keep the second bank in RAM block MEM\\[20\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem20(&self) -> Mem20R {
        Mem20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Keep the second bank in RAM block MEM\\[21\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem21(&self) -> Mem21R {
        Mem21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Keep the second bank in RAM block MEM\\[22\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem22(&self) -> Mem22R {
        Mem22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Keep the second bank in RAM block MEM\\[23\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem23(&self) -> Mem23R {
        Mem23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Keep the second bank in RAM block MEM\\[24\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem24(&self) -> Mem24R {
        Mem24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Keep the second bank in RAM block MEM\\[25\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem25(&self) -> Mem25R {
        Mem25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Keep the second bank in RAM block MEM\\[26\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem26(&self) -> Mem26R {
        Mem26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Keep the second bank in RAM block MEM\\[27\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem27(&self) -> Mem27R {
        Mem27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Keep the second bank in RAM block MEM\\[28\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem28(&self) -> Mem28R {
        Mem28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Keep the second bank in RAM block MEM\\[29\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem29(&self) -> Mem29R {
        Mem29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Keep the second bank in RAM block MEM\\[30\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem30(&self) -> Mem30R {
        Mem30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Keep the second bank in RAM block MEM\\[31\\]
retained when in System OFF mode."]
    #[inline(always)]
    pub fn mem31(&self) -> Mem31R {
        Mem31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Keep the second bank in RAM block MEM\\[0\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem0(&mut self) -> Mem0W<Ret2Spec> {
        Mem0W::new(self, 0)
    }
    #[doc = "Bit 1 - Keep the second bank in RAM block MEM\\[1\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem1(&mut self) -> Mem1W<Ret2Spec> {
        Mem1W::new(self, 1)
    }
    #[doc = "Bit 2 - Keep the second bank in RAM block MEM\\[2\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem2(&mut self) -> Mem2W<Ret2Spec> {
        Mem2W::new(self, 2)
    }
    #[doc = "Bit 3 - Keep the second bank in RAM block MEM\\[3\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem3(&mut self) -> Mem3W<Ret2Spec> {
        Mem3W::new(self, 3)
    }
    #[doc = "Bit 4 - Keep the second bank in RAM block MEM\\[4\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem4(&mut self) -> Mem4W<Ret2Spec> {
        Mem4W::new(self, 4)
    }
    #[doc = "Bit 5 - Keep the second bank in RAM block MEM\\[5\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem5(&mut self) -> Mem5W<Ret2Spec> {
        Mem5W::new(self, 5)
    }
    #[doc = "Bit 6 - Keep the second bank in RAM block MEM\\[6\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem6(&mut self) -> Mem6W<Ret2Spec> {
        Mem6W::new(self, 6)
    }
    #[doc = "Bit 7 - Keep the second bank in RAM block MEM\\[7\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem7(&mut self) -> Mem7W<Ret2Spec> {
        Mem7W::new(self, 7)
    }
    #[doc = "Bit 8 - Keep the second bank in RAM block MEM\\[8\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem8(&mut self) -> Mem8W<Ret2Spec> {
        Mem8W::new(self, 8)
    }
    #[doc = "Bit 9 - Keep the second bank in RAM block MEM\\[9\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem9(&mut self) -> Mem9W<Ret2Spec> {
        Mem9W::new(self, 9)
    }
    #[doc = "Bit 10 - Keep the second bank in RAM block MEM\\[10\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem10(&mut self) -> Mem10W<Ret2Spec> {
        Mem10W::new(self, 10)
    }
    #[doc = "Bit 11 - Keep the second bank in RAM block MEM\\[11\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem11(&mut self) -> Mem11W<Ret2Spec> {
        Mem11W::new(self, 11)
    }
    #[doc = "Bit 12 - Keep the second bank in RAM block MEM\\[12\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem12(&mut self) -> Mem12W<Ret2Spec> {
        Mem12W::new(self, 12)
    }
    #[doc = "Bit 13 - Keep the second bank in RAM block MEM\\[13\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem13(&mut self) -> Mem13W<Ret2Spec> {
        Mem13W::new(self, 13)
    }
    #[doc = "Bit 14 - Keep the second bank in RAM block MEM\\[14\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem14(&mut self) -> Mem14W<Ret2Spec> {
        Mem14W::new(self, 14)
    }
    #[doc = "Bit 15 - Keep the second bank in RAM block MEM\\[15\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem15(&mut self) -> Mem15W<Ret2Spec> {
        Mem15W::new(self, 15)
    }
    #[doc = "Bit 16 - Keep the second bank in RAM block MEM\\[16\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem16(&mut self) -> Mem16W<Ret2Spec> {
        Mem16W::new(self, 16)
    }
    #[doc = "Bit 17 - Keep the second bank in RAM block MEM\\[17\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem17(&mut self) -> Mem17W<Ret2Spec> {
        Mem17W::new(self, 17)
    }
    #[doc = "Bit 18 - Keep the second bank in RAM block MEM\\[18\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem18(&mut self) -> Mem18W<Ret2Spec> {
        Mem18W::new(self, 18)
    }
    #[doc = "Bit 19 - Keep the second bank in RAM block MEM\\[19\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem19(&mut self) -> Mem19W<Ret2Spec> {
        Mem19W::new(self, 19)
    }
    #[doc = "Bit 20 - Keep the second bank in RAM block MEM\\[20\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem20(&mut self) -> Mem20W<Ret2Spec> {
        Mem20W::new(self, 20)
    }
    #[doc = "Bit 21 - Keep the second bank in RAM block MEM\\[21\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem21(&mut self) -> Mem21W<Ret2Spec> {
        Mem21W::new(self, 21)
    }
    #[doc = "Bit 22 - Keep the second bank in RAM block MEM\\[22\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem22(&mut self) -> Mem22W<Ret2Spec> {
        Mem22W::new(self, 22)
    }
    #[doc = "Bit 23 - Keep the second bank in RAM block MEM\\[23\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem23(&mut self) -> Mem23W<Ret2Spec> {
        Mem23W::new(self, 23)
    }
    #[doc = "Bit 24 - Keep the second bank in RAM block MEM\\[24\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem24(&mut self) -> Mem24W<Ret2Spec> {
        Mem24W::new(self, 24)
    }
    #[doc = "Bit 25 - Keep the second bank in RAM block MEM\\[25\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem25(&mut self) -> Mem25W<Ret2Spec> {
        Mem25W::new(self, 25)
    }
    #[doc = "Bit 26 - Keep the second bank in RAM block MEM\\[26\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem26(&mut self) -> Mem26W<Ret2Spec> {
        Mem26W::new(self, 26)
    }
    #[doc = "Bit 27 - Keep the second bank in RAM block MEM\\[27\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem27(&mut self) -> Mem27W<Ret2Spec> {
        Mem27W::new(self, 27)
    }
    #[doc = "Bit 28 - Keep the second bank in RAM block MEM\\[28\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem28(&mut self) -> Mem28W<Ret2Spec> {
        Mem28W::new(self, 28)
    }
    #[doc = "Bit 29 - Keep the second bank in RAM block MEM\\[29\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem29(&mut self) -> Mem29W<Ret2Spec> {
        Mem29W::new(self, 29)
    }
    #[doc = "Bit 30 - Keep the second bank in RAM block MEM\\[30\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem30(&mut self) -> Mem30W<Ret2Spec> {
        Mem30W::new(self, 30)
    }
    #[doc = "Bit 31 - Keep the second bank in RAM block MEM\\[31\\]
retained when in System OFF mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem31(&mut self) -> Mem31W<Ret2Spec> {
        Mem31W::new(self, 31)
    }
}
#[doc = "Description cluster: RAM retention for the second bank in the RAM block\n\nYou can [`read`](crate::Reg::read) this register and get [`ret2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ret2Spec;
impl crate::RegisterSpec for Ret2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ret2::R`](R) reader structure"]
impl crate::Readable for Ret2Spec {}
#[doc = "`write(|w| ..)` method takes [`ret2::W`](W) writer structure"]
impl crate::Writable for Ret2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RET2 to value 0"]
impl crate::Resettable for Ret2Spec {
    const RESET_VALUE: u32 = 0;
}
