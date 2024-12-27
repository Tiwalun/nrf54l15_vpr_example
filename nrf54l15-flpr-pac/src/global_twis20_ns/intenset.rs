#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to enable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopped {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Stopped> for bool {
    #[inline(always)]
    fn from(variant: Stopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` reader - Write '1' to enable interrupt for event STOPPED"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stopped::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stopped::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoppedWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<StoppedWO> for bool {
    #[inline(always)]
    fn from(variant: StoppedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` writer - Write '1' to enable interrupt for event STOPPED"]
pub type StoppedW<'a, REG> = crate::BitWriter<'a, REG, StoppedWO>;
impl<'a, REG> StoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(StoppedWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Error> for bool {
    #[inline(always)]
    fn from(variant: Error) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Write '1' to enable interrupt for event ERROR"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Error::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Error::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<ErrorWO> for bool {
    #[inline(always)]
    fn from(variant: ErrorWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` writer - Write '1' to enable interrupt for event ERROR"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG, ErrorWO>;
impl<'a, REG> ErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(ErrorWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event WRITE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Write {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Write> for bool {
    #[inline(always)]
    fn from(variant: Write) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE` reader - Write '1' to enable interrupt for event WRITE"]
pub type WriteR = crate::BitReader<Write>;
impl WriteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Write {
        match self.bits {
            false => Write::Disabled,
            true => Write::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Write::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Write::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event WRITE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<WriteWO> for bool {
    #[inline(always)]
    fn from(variant: WriteWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE` writer - Write '1' to enable interrupt for event WRITE"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG, WriteWO>;
impl<'a, REG> WriteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(WriteWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event READ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Read {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Read> for bool {
    #[inline(always)]
    fn from(variant: Read) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ` reader - Write '1' to enable interrupt for event READ"]
pub type ReadR = crate::BitReader<Read>;
impl ReadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Read {
        match self.bits {
            false => Read::Disabled,
            true => Read::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Read::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Read::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event READ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<ReadWO> for bool {
    #[inline(always)]
    fn from(variant: ReadWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ` writer - Write '1' to enable interrupt for event READ"]
pub type ReadW<'a, REG> = crate::BitWriter<'a, REG, ReadWO>;
impl<'a, REG> ReadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(ReadWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event DMARXEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarxend {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Dmarxend> for bool {
    #[inline(always)]
    fn from(variant: Dmarxend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXEND` reader - Write '1' to enable interrupt for event DMARXEND"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmarxend::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmarxend::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event DMARXEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmarxendWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<DmarxendWO> for bool {
    #[inline(always)]
    fn from(variant: DmarxendWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXEND` writer - Write '1' to enable interrupt for event DMARXEND"]
pub type DmarxendW<'a, REG> = crate::BitWriter<'a, REG, DmarxendWO>;
impl<'a, REG> DmarxendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DmarxendWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event DMARXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarxready {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Dmarxready> for bool {
    #[inline(always)]
    fn from(variant: Dmarxready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXREADY` reader - Write '1' to enable interrupt for event DMARXREADY"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmarxready::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmarxready::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event DMARXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmarxreadyWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<DmarxreadyWO> for bool {
    #[inline(always)]
    fn from(variant: DmarxreadyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXREADY` writer - Write '1' to enable interrupt for event DMARXREADY"]
pub type DmarxreadyW<'a, REG> = crate::BitWriter<'a, REG, DmarxreadyWO>;
impl<'a, REG> DmarxreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DmarxreadyWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event DMARXBUSERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarxbuserror {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Dmarxbuserror> for bool {
    #[inline(always)]
    fn from(variant: Dmarxbuserror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXBUSERROR` reader - Write '1' to enable interrupt for event DMARXBUSERROR"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmarxbuserror::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmarxbuserror::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event DMARXBUSERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmarxbuserrorWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<DmarxbuserrorWO> for bool {
    #[inline(always)]
    fn from(variant: DmarxbuserrorWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXBUSERROR` writer - Write '1' to enable interrupt for event DMARXBUSERROR"]
pub type DmarxbuserrorW<'a, REG> = crate::BitWriter<'a, REG, DmarxbuserrorWO>;
impl<'a, REG> DmarxbuserrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DmarxbuserrorWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event DMARXMATCH\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarxmatch0 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Dmarxmatch0> for bool {
    #[inline(always)]
    fn from(variant: Dmarxmatch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXMATCH0` reader - Write '1' to enable interrupt for event DMARXMATCH\\[0\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmarxmatch0::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmarxmatch0::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event DMARXMATCH\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarxmatch0WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Dmarxmatch0WO> for bool {
    #[inline(always)]
    fn from(variant: Dmarxmatch0WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXMATCH0` writer - Write '1' to enable interrupt for event DMARXMATCH\\[0\\]"]
pub type Dmarxmatch0W<'a, REG> = crate::BitWriter<'a, REG, Dmarxmatch0WO>;
impl<'a, REG> Dmarxmatch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarxmatch0WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event DMARXMATCH\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarxmatch1 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Dmarxmatch1> for bool {
    #[inline(always)]
    fn from(variant: Dmarxmatch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXMATCH1` reader - Write '1' to enable interrupt for event DMARXMATCH\\[1\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmarxmatch1::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmarxmatch1::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event DMARXMATCH\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarxmatch1WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Dmarxmatch1WO> for bool {
    #[inline(always)]
    fn from(variant: Dmarxmatch1WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXMATCH1` writer - Write '1' to enable interrupt for event DMARXMATCH\\[1\\]"]
pub type Dmarxmatch1W<'a, REG> = crate::BitWriter<'a, REG, Dmarxmatch1WO>;
impl<'a, REG> Dmarxmatch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarxmatch1WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event DMARXMATCH\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarxmatch2 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Dmarxmatch2> for bool {
    #[inline(always)]
    fn from(variant: Dmarxmatch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXMATCH2` reader - Write '1' to enable interrupt for event DMARXMATCH\\[2\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmarxmatch2::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmarxmatch2::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event DMARXMATCH\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarxmatch2WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Dmarxmatch2WO> for bool {
    #[inline(always)]
    fn from(variant: Dmarxmatch2WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXMATCH2` writer - Write '1' to enable interrupt for event DMARXMATCH\\[2\\]"]
pub type Dmarxmatch2W<'a, REG> = crate::BitWriter<'a, REG, Dmarxmatch2WO>;
impl<'a, REG> Dmarxmatch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarxmatch2WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event DMARXMATCH\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarxmatch3 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Dmarxmatch3> for bool {
    #[inline(always)]
    fn from(variant: Dmarxmatch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXMATCH3` reader - Write '1' to enable interrupt for event DMARXMATCH\\[3\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmarxmatch3::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmarxmatch3::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event DMARXMATCH\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarxmatch3WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Dmarxmatch3WO> for bool {
    #[inline(always)]
    fn from(variant: Dmarxmatch3WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARXMATCH3` writer - Write '1' to enable interrupt for event DMARXMATCH\\[3\\]"]
pub type Dmarxmatch3W<'a, REG> = crate::BitWriter<'a, REG, Dmarxmatch3WO>;
impl<'a, REG> Dmarxmatch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarxmatch3WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event DMATXEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmatxend {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Dmatxend> for bool {
    #[inline(always)]
    fn from(variant: Dmatxend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATXEND` reader - Write '1' to enable interrupt for event DMATXEND"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmatxend::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmatxend::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event DMATXEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmatxendWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<DmatxendWO> for bool {
    #[inline(always)]
    fn from(variant: DmatxendWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATXEND` writer - Write '1' to enable interrupt for event DMATXEND"]
pub type DmatxendW<'a, REG> = crate::BitWriter<'a, REG, DmatxendWO>;
impl<'a, REG> DmatxendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DmatxendWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event DMATXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmatxready {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Dmatxready> for bool {
    #[inline(always)]
    fn from(variant: Dmatxready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATXREADY` reader - Write '1' to enable interrupt for event DMATXREADY"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmatxready::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmatxready::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event DMATXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmatxreadyWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<DmatxreadyWO> for bool {
    #[inline(always)]
    fn from(variant: DmatxreadyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATXREADY` writer - Write '1' to enable interrupt for event DMATXREADY"]
pub type DmatxreadyW<'a, REG> = crate::BitWriter<'a, REG, DmatxreadyWO>;
impl<'a, REG> DmatxreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DmatxreadyWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event DMATXBUSERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmatxbuserror {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Dmatxbuserror> for bool {
    #[inline(always)]
    fn from(variant: Dmatxbuserror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATXBUSERROR` reader - Write '1' to enable interrupt for event DMATXBUSERROR"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmatxbuserror::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmatxbuserror::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event DMATXBUSERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmatxbuserrorWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<DmatxbuserrorWO> for bool {
    #[inline(always)]
    fn from(variant: DmatxbuserrorWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATXBUSERROR` writer - Write '1' to enable interrupt for event DMATXBUSERROR"]
pub type DmatxbuserrorW<'a, REG> = crate::BitWriter<'a, REG, DmatxbuserrorWO>;
impl<'a, REG> DmatxbuserrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DmatxbuserrorWO::Set)
    }
}
impl R {
    #[doc = "Bit 1 - Write '1' to enable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> StoppedR {
        StoppedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 15 - Write '1' to enable interrupt for event WRITE"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Write '1' to enable interrupt for event READ"]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - Write '1' to enable interrupt for event DMARXEND"]
    #[inline(always)]
    pub fn dmarxend(&self) -> DmarxendR {
        DmarxendR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write '1' to enable interrupt for event DMARXREADY"]
    #[inline(always)]
    pub fn dmarxready(&self) -> DmarxreadyR {
        DmarxreadyR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write '1' to enable interrupt for event DMARXBUSERROR"]
    #[inline(always)]
    pub fn dmarxbuserror(&self) -> DmarxbuserrorR {
        DmarxbuserrorR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Write '1' to enable interrupt for event DMARXMATCH\\[0\\]"]
    #[inline(always)]
    pub fn dmarxmatch0(&self) -> Dmarxmatch0R {
        Dmarxmatch0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write '1' to enable interrupt for event DMARXMATCH\\[1\\]"]
    #[inline(always)]
    pub fn dmarxmatch1(&self) -> Dmarxmatch1R {
        Dmarxmatch1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Write '1' to enable interrupt for event DMARXMATCH\\[2\\]"]
    #[inline(always)]
    pub fn dmarxmatch2(&self) -> Dmarxmatch2R {
        Dmarxmatch2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write '1' to enable interrupt for event DMARXMATCH\\[3\\]"]
    #[inline(always)]
    pub fn dmarxmatch3(&self) -> Dmarxmatch3R {
        Dmarxmatch3R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Write '1' to enable interrupt for event DMATXEND"]
    #[inline(always)]
    pub fn dmatxend(&self) -> DmatxendR {
        DmatxendR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Write '1' to enable interrupt for event DMATXREADY"]
    #[inline(always)]
    pub fn dmatxready(&self) -> DmatxreadyR {
        DmatxreadyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Write '1' to enable interrupt for event DMATXBUSERROR"]
    #[inline(always)]
    pub fn dmatxbuserror(&self) -> DmatxbuserrorR {
        DmatxbuserrorR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Write '1' to enable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&mut self) -> StoppedW<IntensetSpec> {
        StoppedW::new(self, 1)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<IntensetSpec> {
        ErrorW::new(self, 5)
    }
    #[doc = "Bit 15 - Write '1' to enable interrupt for event WRITE"]
    #[inline(always)]
    pub fn write(&mut self) -> WriteW<IntensetSpec> {
        WriteW::new(self, 15)
    }
    #[doc = "Bit 16 - Write '1' to enable interrupt for event READ"]
    #[inline(always)]
    pub fn read(&mut self) -> ReadW<IntensetSpec> {
        ReadW::new(self, 16)
    }
    #[doc = "Bit 19 - Write '1' to enable interrupt for event DMARXEND"]
    #[inline(always)]
    pub fn dmarxend(&mut self) -> DmarxendW<IntensetSpec> {
        DmarxendW::new(self, 19)
    }
    #[doc = "Bit 20 - Write '1' to enable interrupt for event DMARXREADY"]
    #[inline(always)]
    pub fn dmarxready(&mut self) -> DmarxreadyW<IntensetSpec> {
        DmarxreadyW::new(self, 20)
    }
    #[doc = "Bit 21 - Write '1' to enable interrupt for event DMARXBUSERROR"]
    #[inline(always)]
    pub fn dmarxbuserror(&mut self) -> DmarxbuserrorW<IntensetSpec> {
        DmarxbuserrorW::new(self, 21)
    }
    #[doc = "Bit 22 - Write '1' to enable interrupt for event DMARXMATCH\\[0\\]"]
    #[inline(always)]
    pub fn dmarxmatch0(&mut self) -> Dmarxmatch0W<IntensetSpec> {
        Dmarxmatch0W::new(self, 22)
    }
    #[doc = "Bit 23 - Write '1' to enable interrupt for event DMARXMATCH\\[1\\]"]
    #[inline(always)]
    pub fn dmarxmatch1(&mut self) -> Dmarxmatch1W<IntensetSpec> {
        Dmarxmatch1W::new(self, 23)
    }
    #[doc = "Bit 24 - Write '1' to enable interrupt for event DMARXMATCH\\[2\\]"]
    #[inline(always)]
    pub fn dmarxmatch2(&mut self) -> Dmarxmatch2W<IntensetSpec> {
        Dmarxmatch2W::new(self, 24)
    }
    #[doc = "Bit 25 - Write '1' to enable interrupt for event DMARXMATCH\\[3\\]"]
    #[inline(always)]
    pub fn dmarxmatch3(&mut self) -> Dmarxmatch3W<IntensetSpec> {
        Dmarxmatch3W::new(self, 25)
    }
    #[doc = "Bit 26 - Write '1' to enable interrupt for event DMATXEND"]
    #[inline(always)]
    pub fn dmatxend(&mut self) -> DmatxendW<IntensetSpec> {
        DmatxendW::new(self, 26)
    }
    #[doc = "Bit 27 - Write '1' to enable interrupt for event DMATXREADY"]
    #[inline(always)]
    pub fn dmatxready(&mut self) -> DmatxreadyW<IntensetSpec> {
        DmatxreadyW::new(self, 27)
    }
    #[doc = "Bit 28 - Write '1' to enable interrupt for event DMATXBUSERROR"]
    #[inline(always)]
    pub fn dmatxbuserror(&mut self) -> DmatxbuserrorW<IntensetSpec> {
        DmatxbuserrorW::new(self, 28)
    }
}
#[doc = "Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u32 = 0;
}
