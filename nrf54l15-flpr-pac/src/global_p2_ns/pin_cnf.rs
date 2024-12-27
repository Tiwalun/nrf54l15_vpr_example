#[doc = "Register `PIN_CNF[%s]` reader"]
pub type R = crate::R<PinCnfSpec>;
#[doc = "Register `PIN_CNF[%s]` writer"]
pub type W = crate::W<PinCnfSpec>;
#[doc = "Pin direction. Same physical register as DIR register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: Configure pin as an input pin"]
    Input = 0,
    #[doc = "1: Configure pin as an output pin"]
    Output = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Pin direction. Same physical register as DIR register"]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::Input,
            true => Dir::Output,
        }
    }
    #[doc = "Configure pin as an input pin"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Dir::Input
    }
    #[doc = "Configure pin as an output pin"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Dir::Output
    }
}
#[doc = "Field `DIR` writer - Pin direction. Same physical register as DIR register"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG, Dir>;
impl<'a, REG> DirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configure pin as an input pin"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::Input)
    }
    #[doc = "Configure pin as an output pin"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::Output)
    }
}
#[doc = "Connect or disconnect input buffer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Input {
    #[doc = "0: Connect input buffer"]
    Connect = 0,
    #[doc = "1: Disconnect input buffer"]
    Disconnect = 1,
}
impl From<Input> for bool {
    #[inline(always)]
    fn from(variant: Input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPUT` reader - Connect or disconnect input buffer"]
pub type InputR = crate::BitReader<Input>;
impl InputR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Input {
        match self.bits {
            false => Input::Connect,
            true => Input::Disconnect,
        }
    }
    #[doc = "Connect input buffer"]
    #[inline(always)]
    pub fn is_connect(&self) -> bool {
        *self == Input::Connect
    }
    #[doc = "Disconnect input buffer"]
    #[inline(always)]
    pub fn is_disconnect(&self) -> bool {
        *self == Input::Disconnect
    }
}
#[doc = "Field `INPUT` writer - Connect or disconnect input buffer"]
pub type InputW<'a, REG> = crate::BitWriter<'a, REG, Input>;
impl<'a, REG> InputW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect input buffer"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut crate::W<REG> {
        self.variant(Input::Connect)
    }
    #[doc = "Disconnect input buffer"]
    #[inline(always)]
    pub fn disconnect(self) -> &'a mut crate::W<REG> {
        self.variant(Input::Disconnect)
    }
}
#[doc = "Pull configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pull {
    #[doc = "0: No pull"]
    Disabled = 0,
    #[doc = "1: Pull down on pin"]
    Pulldown = 1,
    #[doc = "3: Pull up on pin"]
    Pullup = 3,
}
impl From<Pull> for u8 {
    #[inline(always)]
    fn from(variant: Pull) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pull {
    type Ux = u8;
}
impl crate::IsEnum for Pull {}
#[doc = "Field `PULL` reader - Pull configuration"]
pub type PullR = crate::FieldReader<Pull>;
impl PullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pull> {
        match self.bits {
            0 => Some(Pull::Disabled),
            1 => Some(Pull::Pulldown),
            3 => Some(Pull::Pullup),
            _ => None,
        }
    }
    #[doc = "No pull"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pull::Disabled
    }
    #[doc = "Pull down on pin"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == Pull::Pulldown
    }
    #[doc = "Pull up on pin"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == Pull::Pullup
    }
}
#[doc = "Field `PULL` writer - Pull configuration"]
pub type PullW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pull>;
impl<'a, REG> PullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pull::Disabled)
    }
    #[doc = "Pull down on pin"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Pull::Pulldown)
    }
    #[doc = "Pull up on pin"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut crate::W<REG> {
        self.variant(Pull::Pullup)
    }
}
#[doc = "Drive configuration for '0'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Drive0 {
    #[doc = "0: Standard '0'"]
    S0 = 0,
    #[doc = "1: High drive '0'"]
    H0 = 1,
    #[doc = "2: Disconnect '0'(normally used for wired-or connections)"]
    D0 = 2,
    #[doc = "3: Extra high drive '0'"]
    E0 = 3,
}
impl From<Drive0> for u8 {
    #[inline(always)]
    fn from(variant: Drive0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Drive0 {
    type Ux = u8;
}
impl crate::IsEnum for Drive0 {}
#[doc = "Field `DRIVE0` reader - Drive configuration for '0'"]
pub type Drive0R = crate::FieldReader<Drive0>;
impl Drive0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drive0 {
        match self.bits {
            0 => Drive0::S0,
            1 => Drive0::H0,
            2 => Drive0::D0,
            3 => Drive0::E0,
            _ => unreachable!(),
        }
    }
    #[doc = "Standard '0'"]
    #[inline(always)]
    pub fn is_s0(&self) -> bool {
        *self == Drive0::S0
    }
    #[doc = "High drive '0'"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == Drive0::H0
    }
    #[doc = "Disconnect '0'(normally used for wired-or connections)"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Drive0::D0
    }
    #[doc = "Extra high drive '0'"]
    #[inline(always)]
    pub fn is_e0(&self) -> bool {
        *self == Drive0::E0
    }
}
#[doc = "Field `DRIVE0` writer - Drive configuration for '0'"]
pub type Drive0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Drive0, crate::Safe>;
impl<'a, REG> Drive0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard '0'"]
    #[inline(always)]
    pub fn s0(self) -> &'a mut crate::W<REG> {
        self.variant(Drive0::S0)
    }
    #[doc = "High drive '0'"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(Drive0::H0)
    }
    #[doc = "Disconnect '0'(normally used for wired-or connections)"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(Drive0::D0)
    }
    #[doc = "Extra high drive '0'"]
    #[inline(always)]
    pub fn e0(self) -> &'a mut crate::W<REG> {
        self.variant(Drive0::E0)
    }
}
#[doc = "Drive configuration for '1'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Drive1 {
    #[doc = "0: Standard '1'"]
    S1 = 0,
    #[doc = "1: High drive '1'"]
    H1 = 1,
    #[doc = "2: Disconnect '1'(normally used for wired-or connections)"]
    D1 = 2,
    #[doc = "3: Extra high drive '1'"]
    E1 = 3,
}
impl From<Drive1> for u8 {
    #[inline(always)]
    fn from(variant: Drive1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Drive1 {
    type Ux = u8;
}
impl crate::IsEnum for Drive1 {}
#[doc = "Field `DRIVE1` reader - Drive configuration for '1'"]
pub type Drive1R = crate::FieldReader<Drive1>;
impl Drive1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drive1 {
        match self.bits {
            0 => Drive1::S1,
            1 => Drive1::H1,
            2 => Drive1::D1,
            3 => Drive1::E1,
            _ => unreachable!(),
        }
    }
    #[doc = "Standard '1'"]
    #[inline(always)]
    pub fn is_s1(&self) -> bool {
        *self == Drive1::S1
    }
    #[doc = "High drive '1'"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == Drive1::H1
    }
    #[doc = "Disconnect '1'(normally used for wired-or connections)"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Drive1::D1
    }
    #[doc = "Extra high drive '1'"]
    #[inline(always)]
    pub fn is_e1(&self) -> bool {
        *self == Drive1::E1
    }
}
#[doc = "Field `DRIVE1` writer - Drive configuration for '1'"]
pub type Drive1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Drive1, crate::Safe>;
impl<'a, REG> Drive1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard '1'"]
    #[inline(always)]
    pub fn s1(self) -> &'a mut crate::W<REG> {
        self.variant(Drive1::S1)
    }
    #[doc = "High drive '1'"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(Drive1::H1)
    }
    #[doc = "Disconnect '1'(normally used for wired-or connections)"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(Drive1::D1)
    }
    #[doc = "Extra high drive '1'"]
    #[inline(always)]
    pub fn e1(self) -> &'a mut crate::W<REG> {
        self.variant(Drive1::E1)
    }
}
#[doc = "Pin sensing mechanism\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sense {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "2: Sense for high level"]
    High = 2,
    #[doc = "3: Sense for low level"]
    Low = 3,
}
impl From<Sense> for u8 {
    #[inline(always)]
    fn from(variant: Sense) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sense {
    type Ux = u8;
}
impl crate::IsEnum for Sense {}
#[doc = "Field `SENSE` reader - Pin sensing mechanism"]
pub type SenseR = crate::FieldReader<Sense>;
impl SenseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sense> {
        match self.bits {
            0 => Some(Sense::Disabled),
            2 => Some(Sense::High),
            3 => Some(Sense::Low),
            _ => None,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sense::Disabled
    }
    #[doc = "Sense for high level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sense::High
    }
    #[doc = "Sense for low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sense::Low
    }
}
#[doc = "Field `SENSE` writer - Pin sensing mechanism"]
pub type SenseW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sense>;
impl<'a, REG> SenseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sense::Disabled)
    }
    #[doc = "Sense for high level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sense::High)
    }
    #[doc = "Sense for low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sense::Low)
    }
}
#[doc = "Select which module has direct control over this pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctrlsel {
    #[doc = "0: GPIO or peripherals with PSEL registers"]
    Gpio = 0,
    #[doc = "1: VPR processor"]
    Vpr = 1,
    #[doc = "4: GRTC peripheral"]
    Grtc = 4,
}
impl From<Ctrlsel> for u8 {
    #[inline(always)]
    fn from(variant: Ctrlsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctrlsel {
    type Ux = u8;
}
impl crate::IsEnum for Ctrlsel {}
#[doc = "Field `CTRLSEL` reader - Select which module has direct control over this pin"]
pub type CtrlselR = crate::FieldReader<Ctrlsel>;
impl CtrlselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctrlsel> {
        match self.bits {
            0 => Some(Ctrlsel::Gpio),
            1 => Some(Ctrlsel::Vpr),
            4 => Some(Ctrlsel::Grtc),
            _ => None,
        }
    }
    #[doc = "GPIO or peripherals with PSEL registers"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Ctrlsel::Gpio
    }
    #[doc = "VPR processor"]
    #[inline(always)]
    pub fn is_vpr(&self) -> bool {
        *self == Ctrlsel::Vpr
    }
    #[doc = "GRTC peripheral"]
    #[inline(always)]
    pub fn is_grtc(&self) -> bool {
        *self == Ctrlsel::Grtc
    }
}
#[doc = "Field `CTRLSEL` writer - Select which module has direct control over this pin"]
pub type CtrlselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ctrlsel>;
impl<'a, REG> CtrlselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO or peripherals with PSEL registers"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrlsel::Gpio)
    }
    #[doc = "VPR processor"]
    #[inline(always)]
    pub fn vpr(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrlsel::Vpr)
    }
    #[doc = "GRTC peripheral"]
    #[inline(always)]
    pub fn grtc(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrlsel::Grtc)
    }
}
impl R {
    #[doc = "Bit 0 - Pin direction. Same physical register as DIR register"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Connect or disconnect input buffer"]
    #[inline(always)]
    pub fn input(&self) -> InputR {
        InputR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Pull configuration"]
    #[inline(always)]
    pub fn pull(&self) -> PullR {
        PullR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Drive configuration for '0'"]
    #[inline(always)]
    pub fn drive0(&self) -> Drive0R {
        Drive0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive configuration for '1'"]
    #[inline(always)]
    pub fn drive1(&self) -> Drive1R {
        Drive1R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Pin sensing mechanism"]
    #[inline(always)]
    pub fn sense(&self) -> SenseR {
        SenseR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 28:30 - Select which module has direct control over this pin"]
    #[inline(always)]
    pub fn ctrlsel(&self) -> CtrlselR {
        CtrlselR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Pin direction. Same physical register as DIR register"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<PinCnfSpec> {
        DirW::new(self, 0)
    }
    #[doc = "Bit 1 - Connect or disconnect input buffer"]
    #[inline(always)]
    pub fn input(&mut self) -> InputW<PinCnfSpec> {
        InputW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Pull configuration"]
    #[inline(always)]
    pub fn pull(&mut self) -> PullW<PinCnfSpec> {
        PullW::new(self, 2)
    }
    #[doc = "Bits 8:9 - Drive configuration for '0'"]
    #[inline(always)]
    pub fn drive0(&mut self) -> Drive0W<PinCnfSpec> {
        Drive0W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive configuration for '1'"]
    #[inline(always)]
    pub fn drive1(&mut self) -> Drive1W<PinCnfSpec> {
        Drive1W::new(self, 10)
    }
    #[doc = "Bits 16:17 - Pin sensing mechanism"]
    #[inline(always)]
    pub fn sense(&mut self) -> SenseW<PinCnfSpec> {
        SenseW::new(self, 16)
    }
    #[doc = "Bits 28:30 - Select which module has direct control over this pin"]
    #[inline(always)]
    pub fn ctrlsel(&mut self) -> CtrlselW<PinCnfSpec> {
        CtrlselW::new(self, 28)
    }
}
#[doc = "Description collection: Pin n configuration of GPIO pin\n\nYou can [`read`](crate::Reg::read) this register and get [`pin_cnf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin_cnf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PinCnfSpec;
impl crate::RegisterSpec for PinCnfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin_cnf::R`](R) reader structure"]
impl crate::Readable for PinCnfSpec {}
#[doc = "`write(|w| ..)` method takes [`pin_cnf::W`](W) writer structure"]
impl crate::Writable for PinCnfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIN_CNF[%s]
to value 0x02"]
impl crate::Resettable for PinCnfSpec {
    const RESET_VALUE: u32 = 0x02;
}
