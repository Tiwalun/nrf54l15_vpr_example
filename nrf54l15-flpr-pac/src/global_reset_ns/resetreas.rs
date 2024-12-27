#[doc = "Register `RESETREAS` reader"]
pub type R = crate::R<ResetreasSpec>;
#[doc = "Register `RESETREAS` writer"]
pub type W = crate::W<ResetreasSpec>;
#[doc = "Reset from pin reset detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resetpin {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Resetpin> for bool {
    #[inline(always)]
    fn from(variant: Resetpin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETPIN` reader - Reset from pin reset detected"]
pub type ResetpinR = crate::BitReader<Resetpin>;
impl ResetpinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resetpin {
        match self.bits {
            false => Resetpin::NotDetected,
            true => Resetpin::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Resetpin::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Resetpin::Detected
    }
}
#[doc = "Field `RESETPIN` writer - Reset from pin reset detected"]
pub type ResetpinW<'a, REG> = crate::BitWriter<'a, REG, Resetpin>;
impl<'a, REG> ResetpinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Resetpin::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Resetpin::Detected)
    }
}
#[doc = "Reset from watchdog timer 0 detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dog0 {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Dog0> for bool {
    #[inline(always)]
    fn from(variant: Dog0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOG0` reader - Reset from watchdog timer 0 detected"]
pub type Dog0R = crate::BitReader<Dog0>;
impl Dog0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dog0 {
        match self.bits {
            false => Dog0::NotDetected,
            true => Dog0::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Dog0::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Dog0::Detected
    }
}
#[doc = "Field `DOG0` writer - Reset from watchdog timer 0 detected"]
pub type Dog0W<'a, REG> = crate::BitWriter<'a, REG, Dog0>;
impl<'a, REG> Dog0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Dog0::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Dog0::Detected)
    }
}
#[doc = "Reset from watchdog timer 1 detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dog1 {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Dog1> for bool {
    #[inline(always)]
    fn from(variant: Dog1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOG1` reader - Reset from watchdog timer 1 detected"]
pub type Dog1R = crate::BitReader<Dog1>;
impl Dog1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dog1 {
        match self.bits {
            false => Dog1::NotDetected,
            true => Dog1::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Dog1::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Dog1::Detected
    }
}
#[doc = "Field `DOG1` writer - Reset from watchdog timer 1 detected"]
pub type Dog1W<'a, REG> = crate::BitWriter<'a, REG, Dog1>;
impl<'a, REG> Dog1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Dog1::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Dog1::Detected)
    }
}
#[doc = "Soft reset from CTRL-AP detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrlapsoft {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Ctrlapsoft> for bool {
    #[inline(always)]
    fn from(variant: Ctrlapsoft) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRLAPSOFT` reader - Soft reset from CTRL-AP detected"]
pub type CtrlapsoftR = crate::BitReader<Ctrlapsoft>;
impl CtrlapsoftR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctrlapsoft {
        match self.bits {
            false => Ctrlapsoft::NotDetected,
            true => Ctrlapsoft::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Ctrlapsoft::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Ctrlapsoft::Detected
    }
}
#[doc = "Field `CTRLAPSOFT` writer - Soft reset from CTRL-AP detected"]
pub type CtrlapsoftW<'a, REG> = crate::BitWriter<'a, REG, Ctrlapsoft>;
impl<'a, REG> CtrlapsoftW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrlapsoft::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrlapsoft::Detected)
    }
}
#[doc = "Reset due to CTRL-AP hard reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrlaphard {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Ctrlaphard> for bool {
    #[inline(always)]
    fn from(variant: Ctrlaphard) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRLAPHARD` reader - Reset due to CTRL-AP hard reset"]
pub type CtrlaphardR = crate::BitReader<Ctrlaphard>;
impl CtrlaphardR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctrlaphard {
        match self.bits {
            false => Ctrlaphard::NotDetected,
            true => Ctrlaphard::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Ctrlaphard::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Ctrlaphard::Detected
    }
}
#[doc = "Field `CTRLAPHARD` writer - Reset due to CTRL-AP hard reset"]
pub type CtrlaphardW<'a, REG> = crate::BitWriter<'a, REG, Ctrlaphard>;
impl<'a, REG> CtrlaphardW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrlaphard::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrlaphard::Detected)
    }
}
#[doc = "Reset due to CTRL-AP pin reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrlappin {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Ctrlappin> for bool {
    #[inline(always)]
    fn from(variant: Ctrlappin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRLAPPIN` reader - Reset due to CTRL-AP pin reset"]
pub type CtrlappinR = crate::BitReader<Ctrlappin>;
impl CtrlappinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctrlappin {
        match self.bits {
            false => Ctrlappin::NotDetected,
            true => Ctrlappin::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Ctrlappin::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Ctrlappin::Detected
    }
}
#[doc = "Field `CTRLAPPIN` writer - Reset due to CTRL-AP pin reset"]
pub type CtrlappinW<'a, REG> = crate::BitWriter<'a, REG, Ctrlappin>;
impl<'a, REG> CtrlappinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrlappin::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrlappin::Detected)
    }
}
#[doc = "Reset from soft reset detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sreq {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Sreq> for bool {
    #[inline(always)]
    fn from(variant: Sreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SREQ` reader - Reset from soft reset detected"]
pub type SreqR = crate::BitReader<Sreq>;
impl SreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sreq {
        match self.bits {
            false => Sreq::NotDetected,
            true => Sreq::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Sreq::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Sreq::Detected
    }
}
#[doc = "Field `SREQ` writer - Reset from soft reset detected"]
pub type SreqW<'a, REG> = crate::BitWriter<'a, REG, Sreq>;
impl<'a, REG> SreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Sreq::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Sreq::Detected)
    }
}
#[doc = "Reset from CPU lockup detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lockup {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Lockup> for bool {
    #[inline(always)]
    fn from(variant: Lockup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKUP` reader - Reset from CPU lockup detected"]
pub type LockupR = crate::BitReader<Lockup>;
impl LockupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lockup {
        match self.bits {
            false => Lockup::NotDetected,
            true => Lockup::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Lockup::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Lockup::Detected
    }
}
#[doc = "Field `LOCKUP` writer - Reset from CPU lockup detected"]
pub type LockupW<'a, REG> = crate::BitWriter<'a, REG, Lockup>;
impl<'a, REG> LockupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Lockup::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Lockup::Detected)
    }
}
#[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Off {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Off> for bool {
    #[inline(always)]
    fn from(variant: Off) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFF` reader - Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
pub type OffR = crate::BitReader<Off>;
impl OffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Off {
        match self.bits {
            false => Off::NotDetected,
            true => Off::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Off::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Off::Detected
    }
}
#[doc = "Field `OFF` writer - Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
pub type OffW<'a, REG> = crate::BitWriter<'a, REG, Off>;
impl<'a, REG> OffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Off::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Off::Detected)
    }
}
#[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpcomp {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Lpcomp> for bool {
    #[inline(always)]
    fn from(variant: Lpcomp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPCOMP` reader - Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
pub type LpcompR = crate::BitReader<Lpcomp>;
impl LpcompR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpcomp {
        match self.bits {
            false => Lpcomp::NotDetected,
            true => Lpcomp::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Lpcomp::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Lpcomp::Detected
    }
}
#[doc = "Field `LPCOMP` writer - Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
pub type LpcompW<'a, REG> = crate::BitWriter<'a, REG, Lpcomp>;
impl<'a, REG> LpcompW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Lpcomp::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Lpcomp::Detected)
    }
}
#[doc = "Reset triggered by Debug Interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dif {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Dif> for bool {
    #[inline(always)]
    fn from(variant: Dif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIF` reader - Reset triggered by Debug Interface"]
pub type DifR = crate::BitReader<Dif>;
impl DifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dif {
        match self.bits {
            false => Dif::NotDetected,
            true => Dif::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Dif::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Dif::Detected
    }
}
#[doc = "Field `DIF` writer - Reset triggered by Debug Interface"]
pub type DifW<'a, REG> = crate::BitWriter<'a, REG, Dif>;
impl<'a, REG> DifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Dif::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Dif::Detected)
    }
}
#[doc = "Reset due to wakeup from GRTC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Grtc {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Grtc> for bool {
    #[inline(always)]
    fn from(variant: Grtc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GRTC` reader - Reset due to wakeup from GRTC"]
pub type GrtcR = crate::BitReader<Grtc>;
impl GrtcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Grtc {
        match self.bits {
            false => Grtc::NotDetected,
            true => Grtc::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Grtc::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Grtc::Detected
    }
}
#[doc = "Field `GRTC` writer - Reset due to wakeup from GRTC"]
pub type GrtcW<'a, REG> = crate::BitWriter<'a, REG, Grtc>;
impl<'a, REG> GrtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Grtc::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Grtc::Detected)
    }
}
#[doc = "Reset after wakeup from System OFF mode due to NFC field being detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nfc {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Nfc> for bool {
    #[inline(always)]
    fn from(variant: Nfc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NFC` reader - Reset after wakeup from System OFF mode due to NFC field being detected"]
pub type NfcR = crate::BitReader<Nfc>;
impl NfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nfc {
        match self.bits {
            false => Nfc::NotDetected,
            true => Nfc::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Nfc::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Nfc::Detected
    }
}
#[doc = "Field `NFC` writer - Reset after wakeup from System OFF mode due to NFC field being detected"]
pub type NfcW<'a, REG> = crate::BitWriter<'a, REG, Nfc>;
impl<'a, REG> NfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Nfc::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Nfc::Detected)
    }
}
#[doc = "Reset due to illegal tampering of the device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sectamper {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Sectamper> for bool {
    #[inline(always)]
    fn from(variant: Sectamper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECTAMPER` reader - Reset due to illegal tampering of the device"]
pub type SectamperR = crate::BitReader<Sectamper>;
impl SectamperR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sectamper {
        match self.bits {
            false => Sectamper::NotDetected,
            true => Sectamper::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Sectamper::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Sectamper::Detected
    }
}
#[doc = "Field `SECTAMPER` writer - Reset due to illegal tampering of the device"]
pub type SectamperW<'a, REG> = crate::BitWriter<'a, REG, Sectamper>;
impl<'a, REG> SectamperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Sectamper::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Sectamper::Detected)
    }
}
impl R {
    #[doc = "Bit 0 - Reset from pin reset detected"]
    #[inline(always)]
    pub fn resetpin(&self) -> ResetpinR {
        ResetpinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset from watchdog timer 0 detected"]
    #[inline(always)]
    pub fn dog0(&self) -> Dog0R {
        Dog0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset from watchdog timer 1 detected"]
    #[inline(always)]
    pub fn dog1(&self) -> Dog1R {
        Dog1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Soft reset from CTRL-AP detected"]
    #[inline(always)]
    pub fn ctrlapsoft(&self) -> CtrlapsoftR {
        CtrlapsoftR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset due to CTRL-AP hard reset"]
    #[inline(always)]
    pub fn ctrlaphard(&self) -> CtrlaphardR {
        CtrlaphardR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset due to CTRL-AP pin reset"]
    #[inline(always)]
    pub fn ctrlappin(&self) -> CtrlappinR {
        CtrlappinR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset from soft reset detected"]
    #[inline(always)]
    pub fn sreq(&self) -> SreqR {
        SreqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset from CPU lockup detected"]
    #[inline(always)]
    pub fn lockup(&self) -> LockupR {
        LockupR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
    #[inline(always)]
    pub fn off(&self) -> OffR {
        OffR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
    #[inline(always)]
    pub fn lpcomp(&self) -> LpcompR {
        LpcompR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset triggered by Debug Interface"]
    #[inline(always)]
    pub fn dif(&self) -> DifR {
        DifR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset due to wakeup from GRTC"]
    #[inline(always)]
    pub fn grtc(&self) -> GrtcR {
        GrtcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset after wakeup from System OFF mode due to NFC field being detected"]
    #[inline(always)]
    pub fn nfc(&self) -> NfcR {
        NfcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reset due to illegal tampering of the device"]
    #[inline(always)]
    pub fn sectamper(&self) -> SectamperR {
        SectamperR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset from pin reset detected"]
    #[inline(always)]
    #[must_use]
    pub fn resetpin(&mut self) -> ResetpinW<ResetreasSpec> {
        ResetpinW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset from watchdog timer 0 detected"]
    #[inline(always)]
    #[must_use]
    pub fn dog0(&mut self) -> Dog0W<ResetreasSpec> {
        Dog0W::new(self, 1)
    }
    #[doc = "Bit 2 - Reset from watchdog timer 1 detected"]
    #[inline(always)]
    #[must_use]
    pub fn dog1(&mut self) -> Dog1W<ResetreasSpec> {
        Dog1W::new(self, 2)
    }
    #[doc = "Bit 3 - Soft reset from CTRL-AP detected"]
    #[inline(always)]
    #[must_use]
    pub fn ctrlapsoft(&mut self) -> CtrlapsoftW<ResetreasSpec> {
        CtrlapsoftW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset due to CTRL-AP hard reset"]
    #[inline(always)]
    #[must_use]
    pub fn ctrlaphard(&mut self) -> CtrlaphardW<ResetreasSpec> {
        CtrlaphardW::new(self, 4)
    }
    #[doc = "Bit 5 - Reset due to CTRL-AP pin reset"]
    #[inline(always)]
    #[must_use]
    pub fn ctrlappin(&mut self) -> CtrlappinW<ResetreasSpec> {
        CtrlappinW::new(self, 5)
    }
    #[doc = "Bit 6 - Reset from soft reset detected"]
    #[inline(always)]
    #[must_use]
    pub fn sreq(&mut self) -> SreqW<ResetreasSpec> {
        SreqW::new(self, 6)
    }
    #[doc = "Bit 7 - Reset from CPU lockup detected"]
    #[inline(always)]
    #[must_use]
    pub fn lockup(&mut self) -> LockupW<ResetreasSpec> {
        LockupW::new(self, 7)
    }
    #[doc = "Bit 8 - Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
    #[inline(always)]
    #[must_use]
    pub fn off(&mut self) -> OffW<ResetreasSpec> {
        OffW::new(self, 8)
    }
    #[doc = "Bit 9 - Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
    #[inline(always)]
    #[must_use]
    pub fn lpcomp(&mut self) -> LpcompW<ResetreasSpec> {
        LpcompW::new(self, 9)
    }
    #[doc = "Bit 10 - Reset triggered by Debug Interface"]
    #[inline(always)]
    #[must_use]
    pub fn dif(&mut self) -> DifW<ResetreasSpec> {
        DifW::new(self, 10)
    }
    #[doc = "Bit 11 - Reset due to wakeup from GRTC"]
    #[inline(always)]
    #[must_use]
    pub fn grtc(&mut self) -> GrtcW<ResetreasSpec> {
        GrtcW::new(self, 11)
    }
    #[doc = "Bit 12 - Reset after wakeup from System OFF mode due to NFC field being detected"]
    #[inline(always)]
    #[must_use]
    pub fn nfc(&mut self) -> NfcW<ResetreasSpec> {
        NfcW::new(self, 12)
    }
    #[doc = "Bit 13 - Reset due to illegal tampering of the device"]
    #[inline(always)]
    #[must_use]
    pub fn sectamper(&mut self) -> SectamperW<ResetreasSpec> {
        SectamperW::new(self, 13)
    }
}
#[doc = "Reset reason\n\nYou can [`read`](crate::Reg::read) this register and get [`resetreas::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetreas::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetreasSpec;
impl crate::RegisterSpec for ResetreasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resetreas::R`](R) reader structure"]
impl crate::Readable for ResetreasSpec {}
#[doc = "`write(|w| ..)` method takes [`resetreas::W`](W) writer structure"]
impl crate::Writable for ResetreasSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESETREAS to value 0"]
impl crate::Resettable for ResetreasSpec {
    const RESET_VALUE: u32 = 0;
}
