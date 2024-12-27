#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopped {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Stopped> for bool {
    #[inline(always)]
    fn from(variant: Stopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` reader - Enable or disable interrupt for event STOPPED"]
pub type StoppedR = crate::BitReader<Stopped>;
impl StoppedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopped {
        match self.bits {
            false => Stopped::Disabled,
            true => Stopped::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stopped::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stopped::Enabled
    }
}
#[doc = "Field `STOPPED` writer - Enable or disable interrupt for event STOPPED"]
pub type StoppedW<'a, REG> = crate::BitWriter<'a, REG, Stopped>;
impl<'a, REG> StoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stopped::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stopped::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Error> for bool {
    #[inline(always)]
    fn from(variant: Error) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Enable or disable interrupt for event ERROR"]
pub type ErrorR = crate::BitReader<Error>;
impl ErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Error {
        match self.bits {
            false => Error::Disabled,
            true => Error::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Error::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Error::Enabled
    }
}
#[doc = "Field `ERROR` writer - Enable or disable interrupt for event ERROR"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG, Error>;
impl<'a, REG> ErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Error::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Error::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event SUSPENDED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suspended {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Suspended> for bool {
    #[inline(always)]
    fn from(variant: Suspended) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPENDED` reader - Enable or disable interrupt for event SUSPENDED"]
pub type SuspendedR = crate::BitReader<Suspended>;
impl SuspendedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Suspended {
        match self.bits {
            false => Suspended::Disabled,
            true => Suspended::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Suspended::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Suspended::Enabled
    }
}
#[doc = "Field `SUSPENDED` writer - Enable or disable interrupt for event SUSPENDED"]
pub type SuspendedW<'a, REG> = crate::BitWriter<'a, REG, Suspended>;
impl<'a, REG> SuspendedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Suspended::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Suspended::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event LASTRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lastrx {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Lastrx> for bool {
    #[inline(always)]
    fn from(variant: Lastrx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTRX` reader - Enable or disable interrupt for event LASTRX"]
pub type LastrxR = crate::BitReader<Lastrx>;
impl LastrxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lastrx {
        match self.bits {
            false => Lastrx::Disabled,
            true => Lastrx::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lastrx::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lastrx::Enabled
    }
}
#[doc = "Field `LASTRX` writer - Enable or disable interrupt for event LASTRX"]
pub type LastrxW<'a, REG> = crate::BitWriter<'a, REG, Lastrx>;
impl<'a, REG> LastrxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lastrx::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lastrx::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event LASTTX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lasttx {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Lasttx> for bool {
    #[inline(always)]
    fn from(variant: Lasttx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTTX` reader - Enable or disable interrupt for event LASTTX"]
pub type LasttxR = crate::BitReader<Lasttx>;
impl LasttxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lasttx {
        match self.bits {
            false => Lasttx::Disabled,
            true => Lasttx::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lasttx::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lasttx::Enabled
    }
}
#[doc = "Field `LASTTX` writer - Enable or disable interrupt for event LASTTX"]
pub type LasttxW<'a, REG> = crate::BitWriter<'a, REG, Lasttx>;
impl<'a, REG> LasttxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lasttx::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lasttx::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DMARXEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarxend {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Dmarxend> for bool {
    #[inline(always)]
    fn from(variant: Dmarxend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXEND` reader - Enable or disable interrupt for event DMARXEND"]
pub type DmarxendR = crate::BitReader<Dmarxend>;
impl DmarxendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmarxend {
        match self.bits {
            false => Dmarxend::Disabled,
            true => Dmarxend::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmarxend::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmarxend::Enabled
    }
}
#[doc = "Field `DMARXEND` writer - Enable or disable interrupt for event DMARXEND"]
pub type DmarxendW<'a, REG> = crate::BitWriter<'a, REG, Dmarxend>;
impl<'a, REG> DmarxendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarxend::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarxend::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DMARXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarxready {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Dmarxready> for bool {
    #[inline(always)]
    fn from(variant: Dmarxready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXREADY` reader - Enable or disable interrupt for event DMARXREADY"]
pub type DmarxreadyR = crate::BitReader<Dmarxready>;
impl DmarxreadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmarxready {
        match self.bits {
            false => Dmarxready::Disabled,
            true => Dmarxready::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmarxready::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmarxready::Enabled
    }
}
#[doc = "Field `DMARXREADY` writer - Enable or disable interrupt for event DMARXREADY"]
pub type DmarxreadyW<'a, REG> = crate::BitWriter<'a, REG, Dmarxready>;
impl<'a, REG> DmarxreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarxready::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarxready::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DMARXBUSERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarxbuserror {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Dmarxbuserror> for bool {
    #[inline(always)]
    fn from(variant: Dmarxbuserror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXBUSERROR` reader - Enable or disable interrupt for event DMARXBUSERROR"]
pub type DmarxbuserrorR = crate::BitReader<Dmarxbuserror>;
impl DmarxbuserrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmarxbuserror {
        match self.bits {
            false => Dmarxbuserror::Disabled,
            true => Dmarxbuserror::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmarxbuserror::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmarxbuserror::Enabled
    }
}
#[doc = "Field `DMARXBUSERROR` writer - Enable or disable interrupt for event DMARXBUSERROR"]
pub type DmarxbuserrorW<'a, REG> = crate::BitWriter<'a, REG, Dmarxbuserror>;
impl<'a, REG> DmarxbuserrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarxbuserror::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarxbuserror::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DMARXMATCH\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarxmatch0 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Dmarxmatch0> for bool {
    #[inline(always)]
    fn from(variant: Dmarxmatch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXMATCH0` reader - Enable or disable interrupt for event DMARXMATCH\\[0\\]"]
pub type Dmarxmatch0R = crate::BitReader<Dmarxmatch0>;
impl Dmarxmatch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmarxmatch0 {
        match self.bits {
            false => Dmarxmatch0::Disabled,
            true => Dmarxmatch0::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmarxmatch0::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmarxmatch0::Enabled
    }
}
#[doc = "Field `DMARXMATCH0` writer - Enable or disable interrupt for event DMARXMATCH\\[0\\]"]
pub type Dmarxmatch0W<'a, REG> = crate::BitWriter<'a, REG, Dmarxmatch0>;
impl<'a, REG> Dmarxmatch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarxmatch0::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarxmatch0::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DMARXMATCH\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarxmatch1 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Dmarxmatch1> for bool {
    #[inline(always)]
    fn from(variant: Dmarxmatch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXMATCH1` reader - Enable or disable interrupt for event DMARXMATCH\\[1\\]"]
pub type Dmarxmatch1R = crate::BitReader<Dmarxmatch1>;
impl Dmarxmatch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmarxmatch1 {
        match self.bits {
            false => Dmarxmatch1::Disabled,
            true => Dmarxmatch1::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmarxmatch1::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmarxmatch1::Enabled
    }
}
#[doc = "Field `DMARXMATCH1` writer - Enable or disable interrupt for event DMARXMATCH\\[1\\]"]
pub type Dmarxmatch1W<'a, REG> = crate::BitWriter<'a, REG, Dmarxmatch1>;
impl<'a, REG> Dmarxmatch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarxmatch1::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarxmatch1::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DMARXMATCH\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarxmatch2 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Dmarxmatch2> for bool {
    #[inline(always)]
    fn from(variant: Dmarxmatch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXMATCH2` reader - Enable or disable interrupt for event DMARXMATCH\\[2\\]"]
pub type Dmarxmatch2R = crate::BitReader<Dmarxmatch2>;
impl Dmarxmatch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmarxmatch2 {
        match self.bits {
            false => Dmarxmatch2::Disabled,
            true => Dmarxmatch2::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmarxmatch2::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmarxmatch2::Enabled
    }
}
#[doc = "Field `DMARXMATCH2` writer - Enable or disable interrupt for event DMARXMATCH\\[2\\]"]
pub type Dmarxmatch2W<'a, REG> = crate::BitWriter<'a, REG, Dmarxmatch2>;
impl<'a, REG> Dmarxmatch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarxmatch2::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarxmatch2::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DMARXMATCH\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarxmatch3 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Dmarxmatch3> for bool {
    #[inline(always)]
    fn from(variant: Dmarxmatch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXMATCH3` reader - Enable or disable interrupt for event DMARXMATCH\\[3\\]"]
pub type Dmarxmatch3R = crate::BitReader<Dmarxmatch3>;
impl Dmarxmatch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmarxmatch3 {
        match self.bits {
            false => Dmarxmatch3::Disabled,
            true => Dmarxmatch3::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmarxmatch3::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmarxmatch3::Enabled
    }
}
#[doc = "Field `DMARXMATCH3` writer - Enable or disable interrupt for event DMARXMATCH\\[3\\]"]
pub type Dmarxmatch3W<'a, REG> = crate::BitWriter<'a, REG, Dmarxmatch3>;
impl<'a, REG> Dmarxmatch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarxmatch3::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarxmatch3::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DMATXEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmatxend {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Dmatxend> for bool {
    #[inline(always)]
    fn from(variant: Dmatxend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATXEND` reader - Enable or disable interrupt for event DMATXEND"]
pub type DmatxendR = crate::BitReader<Dmatxend>;
impl DmatxendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmatxend {
        match self.bits {
            false => Dmatxend::Disabled,
            true => Dmatxend::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmatxend::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmatxend::Enabled
    }
}
#[doc = "Field `DMATXEND` writer - Enable or disable interrupt for event DMATXEND"]
pub type DmatxendW<'a, REG> = crate::BitWriter<'a, REG, Dmatxend>;
impl<'a, REG> DmatxendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatxend::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatxend::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DMATXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmatxready {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Dmatxready> for bool {
    #[inline(always)]
    fn from(variant: Dmatxready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATXREADY` reader - Enable or disable interrupt for event DMATXREADY"]
pub type DmatxreadyR = crate::BitReader<Dmatxready>;
impl DmatxreadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmatxready {
        match self.bits {
            false => Dmatxready::Disabled,
            true => Dmatxready::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmatxready::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmatxready::Enabled
    }
}
#[doc = "Field `DMATXREADY` writer - Enable or disable interrupt for event DMATXREADY"]
pub type DmatxreadyW<'a, REG> = crate::BitWriter<'a, REG, Dmatxready>;
impl<'a, REG> DmatxreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatxready::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatxready::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DMATXBUSERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmatxbuserror {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Dmatxbuserror> for bool {
    #[inline(always)]
    fn from(variant: Dmatxbuserror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATXBUSERROR` reader - Enable or disable interrupt for event DMATXBUSERROR"]
pub type DmatxbuserrorR = crate::BitReader<Dmatxbuserror>;
impl DmatxbuserrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmatxbuserror {
        match self.bits {
            false => Dmatxbuserror::Disabled,
            true => Dmatxbuserror::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmatxbuserror::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmatxbuserror::Enabled
    }
}
#[doc = "Field `DMATXBUSERROR` writer - Enable or disable interrupt for event DMATXBUSERROR"]
pub type DmatxbuserrorW<'a, REG> = crate::BitWriter<'a, REG, Dmatxbuserror>;
impl<'a, REG> DmatxbuserrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatxbuserror::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatxbuserror::Enabled)
    }
}
impl R {
    #[doc = "Bit 1 - Enable or disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> StoppedR {
        StoppedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event SUSPENDED"]
    #[inline(always)]
    pub fn suspended(&self) -> SuspendedR {
        SuspendedR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for event LASTRX"]
    #[inline(always)]
    pub fn lastrx(&self) -> LastrxR {
        LastrxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event LASTTX"]
    #[inline(always)]
    pub fn lasttx(&self) -> LasttxR {
        LasttxR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable or disable interrupt for event DMARXEND"]
    #[inline(always)]
    pub fn dmarxend(&self) -> DmarxendR {
        DmarxendR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable or disable interrupt for event DMARXREADY"]
    #[inline(always)]
    pub fn dmarxready(&self) -> DmarxreadyR {
        DmarxreadyR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable or disable interrupt for event DMARXBUSERROR"]
    #[inline(always)]
    pub fn dmarxbuserror(&self) -> DmarxbuserrorR {
        DmarxbuserrorR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable or disable interrupt for event DMARXMATCH\\[0\\]"]
    #[inline(always)]
    pub fn dmarxmatch0(&self) -> Dmarxmatch0R {
        Dmarxmatch0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable or disable interrupt for event DMARXMATCH\\[1\\]"]
    #[inline(always)]
    pub fn dmarxmatch1(&self) -> Dmarxmatch1R {
        Dmarxmatch1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable or disable interrupt for event DMARXMATCH\\[2\\]"]
    #[inline(always)]
    pub fn dmarxmatch2(&self) -> Dmarxmatch2R {
        Dmarxmatch2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable or disable interrupt for event DMARXMATCH\\[3\\]"]
    #[inline(always)]
    pub fn dmarxmatch3(&self) -> Dmarxmatch3R {
        Dmarxmatch3R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable or disable interrupt for event DMATXEND"]
    #[inline(always)]
    pub fn dmatxend(&self) -> DmatxendR {
        DmatxendR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable or disable interrupt for event DMATXREADY"]
    #[inline(always)]
    pub fn dmatxready(&self) -> DmatxreadyR {
        DmatxreadyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable or disable interrupt for event DMATXBUSERROR"]
    #[inline(always)]
    pub fn dmatxbuserror(&self) -> DmatxbuserrorR {
        DmatxbuserrorR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable or disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&mut self) -> StoppedW<IntenSpec> {
        StoppedW::new(self, 1)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<IntenSpec> {
        ErrorW::new(self, 5)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event SUSPENDED"]
    #[inline(always)]
    pub fn suspended(&mut self) -> SuspendedW<IntenSpec> {
        SuspendedW::new(self, 10)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for event LASTRX"]
    #[inline(always)]
    pub fn lastrx(&mut self) -> LastrxW<IntenSpec> {
        LastrxW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event LASTTX"]
    #[inline(always)]
    pub fn lasttx(&mut self) -> LasttxW<IntenSpec> {
        LasttxW::new(self, 14)
    }
    #[doc = "Bit 19 - Enable or disable interrupt for event DMARXEND"]
    #[inline(always)]
    pub fn dmarxend(&mut self) -> DmarxendW<IntenSpec> {
        DmarxendW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable or disable interrupt for event DMARXREADY"]
    #[inline(always)]
    pub fn dmarxready(&mut self) -> DmarxreadyW<IntenSpec> {
        DmarxreadyW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable or disable interrupt for event DMARXBUSERROR"]
    #[inline(always)]
    pub fn dmarxbuserror(&mut self) -> DmarxbuserrorW<IntenSpec> {
        DmarxbuserrorW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable or disable interrupt for event DMARXMATCH\\[0\\]"]
    #[inline(always)]
    pub fn dmarxmatch0(&mut self) -> Dmarxmatch0W<IntenSpec> {
        Dmarxmatch0W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable or disable interrupt for event DMARXMATCH\\[1\\]"]
    #[inline(always)]
    pub fn dmarxmatch1(&mut self) -> Dmarxmatch1W<IntenSpec> {
        Dmarxmatch1W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable or disable interrupt for event DMARXMATCH\\[2\\]"]
    #[inline(always)]
    pub fn dmarxmatch2(&mut self) -> Dmarxmatch2W<IntenSpec> {
        Dmarxmatch2W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable or disable interrupt for event DMARXMATCH\\[3\\]"]
    #[inline(always)]
    pub fn dmarxmatch3(&mut self) -> Dmarxmatch3W<IntenSpec> {
        Dmarxmatch3W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable or disable interrupt for event DMATXEND"]
    #[inline(always)]
    pub fn dmatxend(&mut self) -> DmatxendW<IntenSpec> {
        DmatxendW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable or disable interrupt for event DMATXREADY"]
    #[inline(always)]
    pub fn dmatxready(&mut self) -> DmatxreadyW<IntenSpec> {
        DmatxreadyW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable or disable interrupt for event DMATXBUSERROR"]
    #[inline(always)]
    pub fn dmatxbuserror(&mut self) -> DmatxbuserrorW<IntenSpec> {
        DmatxbuserrorW::new(self, 28)
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
