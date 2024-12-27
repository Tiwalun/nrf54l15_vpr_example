#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to enable interrupt for event CTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cts {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Cts> for bool {
    #[inline(always)]
    fn from(variant: Cts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` reader - Write '1' to enable interrupt for event CTS"]
pub type CtsR = crate::BitReader<Cts>;
impl CtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cts {
        match self.bits {
            false => Cts::Disabled,
            true => Cts::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cts::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cts::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtsWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<CtsWO> for bool {
    #[inline(always)]
    fn from(variant: CtsWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` writer - Write '1' to enable interrupt for event CTS"]
pub type CtsW<'a, REG> = crate::BitWriter<'a, REG, CtsWO>;
impl<'a, REG> CtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CtsWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event NCTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncts {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ncts> for bool {
    #[inline(always)]
    fn from(variant: Ncts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCTS` reader - Write '1' to enable interrupt for event NCTS"]
pub type NctsR = crate::BitReader<Ncts>;
impl NctsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncts {
        match self.bits {
            false => Ncts::Disabled,
            true => Ncts::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ncts::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ncts::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event NCTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NctsWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<NctsWO> for bool {
    #[inline(always)]
    fn from(variant: NctsWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCTS` writer - Write '1' to enable interrupt for event NCTS"]
pub type NctsW<'a, REG> = crate::BitWriter<'a, REG, NctsWO>;
impl<'a, REG> NctsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(NctsWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event TXDRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdrdy {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Txdrdy> for bool {
    #[inline(always)]
    fn from(variant: Txdrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDRDY` reader - Write '1' to enable interrupt for event TXDRDY"]
pub type TxdrdyR = crate::BitReader<Txdrdy>;
impl TxdrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdrdy {
        match self.bits {
            false => Txdrdy::Disabled,
            true => Txdrdy::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txdrdy::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txdrdy::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event TXDRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxdrdyWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<TxdrdyWO> for bool {
    #[inline(always)]
    fn from(variant: TxdrdyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDRDY` writer - Write '1' to enable interrupt for event TXDRDY"]
pub type TxdrdyW<'a, REG> = crate::BitWriter<'a, REG, TxdrdyWO>;
impl<'a, REG> TxdrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TxdrdyWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RXDRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdrdy {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Rxdrdy> for bool {
    #[inline(always)]
    fn from(variant: Rxdrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDRDY` reader - Write '1' to enable interrupt for event RXDRDY"]
pub type RxdrdyR = crate::BitReader<Rxdrdy>;
impl RxdrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdrdy {
        match self.bits {
            false => Rxdrdy::Disabled,
            true => Rxdrdy::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxdrdy::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxdrdy::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RXDRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxdrdyWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<RxdrdyWO> for bool {
    #[inline(always)]
    fn from(variant: RxdrdyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDRDY` writer - Write '1' to enable interrupt for event RXDRDY"]
pub type RxdrdyW<'a, REG> = crate::BitWriter<'a, REG, RxdrdyWO>;
impl<'a, REG> RxdrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(RxdrdyWO::Set)
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
#[doc = "Write '1' to enable interrupt for event RXTO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxto {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Rxto> for bool {
    #[inline(always)]
    fn from(variant: Rxto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTO` reader - Write '1' to enable interrupt for event RXTO"]
pub type RxtoR = crate::BitReader<Rxto>;
impl RxtoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxto {
        match self.bits {
            false => Rxto::Disabled,
            true => Rxto::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxto::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxto::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RXTO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxtoWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<RxtoWO> for bool {
    #[inline(always)]
    fn from(variant: RxtoWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTO` writer - Write '1' to enable interrupt for event RXTO"]
pub type RxtoW<'a, REG> = crate::BitWriter<'a, REG, RxtoWO>;
impl<'a, REG> RxtoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(RxtoWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event TXSTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txstopped {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Txstopped> for bool {
    #[inline(always)]
    fn from(variant: Txstopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSTOPPED` reader - Write '1' to enable interrupt for event TXSTOPPED"]
pub type TxstoppedR = crate::BitReader<Txstopped>;
impl TxstoppedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txstopped {
        match self.bits {
            false => Txstopped::Disabled,
            true => Txstopped::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txstopped::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txstopped::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event TXSTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxstoppedWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<TxstoppedWO> for bool {
    #[inline(always)]
    fn from(variant: TxstoppedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSTOPPED` writer - Write '1' to enable interrupt for event TXSTOPPED"]
pub type TxstoppedW<'a, REG> = crate::BitWriter<'a, REG, TxstoppedWO>;
impl<'a, REG> TxstoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TxstoppedWO::Set)
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
#[doc = "Write '1' to enable interrupt for event FRAMETIMEOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frametimeout {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Frametimeout> for bool {
    #[inline(always)]
    fn from(variant: Frametimeout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAMETIMEOUT` reader - Write '1' to enable interrupt for event FRAMETIMEOUT"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Frametimeout::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Frametimeout::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event FRAMETIMEOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrametimeoutWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<FrametimeoutWO> for bool {
    #[inline(always)]
    fn from(variant: FrametimeoutWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAMETIMEOUT` writer - Write '1' to enable interrupt for event FRAMETIMEOUT"]
pub type FrametimeoutW<'a, REG> = crate::BitWriter<'a, REG, FrametimeoutWO>;
impl<'a, REG> FrametimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(FrametimeoutWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event CTS"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event NCTS"]
    #[inline(always)]
    pub fn ncts(&self) -> NctsR {
        NctsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event TXDRDY"]
    #[inline(always)]
    pub fn txdrdy(&self) -> TxdrdyR {
        TxdrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event RXDRDY"]
    #[inline(always)]
    pub fn rxdrdy(&self) -> RxdrdyR {
        RxdrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for event RXTO"]
    #[inline(always)]
    pub fn rxto(&self) -> RxtoR {
        RxtoR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Write '1' to enable interrupt for event TXSTOPPED"]
    #[inline(always)]
    pub fn txstopped(&self) -> TxstoppedR {
        TxstoppedR::new(((self.bits >> 12) & 1) != 0)
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
    #[doc = "Bit 29 - Write '1' to enable interrupt for event FRAMETIMEOUT"]
    #[inline(always)]
    pub fn frametimeout(&self) -> FrametimeoutR {
        FrametimeoutR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event CTS"]
    #[inline(always)]
    #[must_use]
    pub fn cts(&mut self) -> CtsW<IntensetSpec> {
        CtsW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event NCTS"]
    #[inline(always)]
    #[must_use]
    pub fn ncts(&mut self) -> NctsW<IntensetSpec> {
        NctsW::new(self, 1)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event TXDRDY"]
    #[inline(always)]
    #[must_use]
    pub fn txdrdy(&mut self) -> TxdrdyW<IntensetSpec> {
        TxdrdyW::new(self, 3)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event RXDRDY"]
    #[inline(always)]
    #[must_use]
    pub fn rxdrdy(&mut self) -> RxdrdyW<IntensetSpec> {
        RxdrdyW::new(self, 4)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<IntensetSpec> {
        ErrorW::new(self, 5)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for event RXTO"]
    #[inline(always)]
    #[must_use]
    pub fn rxto(&mut self) -> RxtoW<IntensetSpec> {
        RxtoW::new(self, 9)
    }
    #[doc = "Bit 12 - Write '1' to enable interrupt for event TXSTOPPED"]
    #[inline(always)]
    #[must_use]
    pub fn txstopped(&mut self) -> TxstoppedW<IntensetSpec> {
        TxstoppedW::new(self, 12)
    }
    #[doc = "Bit 19 - Write '1' to enable interrupt for event DMARXEND"]
    #[inline(always)]
    #[must_use]
    pub fn dmarxend(&mut self) -> DmarxendW<IntensetSpec> {
        DmarxendW::new(self, 19)
    }
    #[doc = "Bit 20 - Write '1' to enable interrupt for event DMARXREADY"]
    #[inline(always)]
    #[must_use]
    pub fn dmarxready(&mut self) -> DmarxreadyW<IntensetSpec> {
        DmarxreadyW::new(self, 20)
    }
    #[doc = "Bit 21 - Write '1' to enable interrupt for event DMARXBUSERROR"]
    #[inline(always)]
    #[must_use]
    pub fn dmarxbuserror(&mut self) -> DmarxbuserrorW<IntensetSpec> {
        DmarxbuserrorW::new(self, 21)
    }
    #[doc = "Bit 22 - Write '1' to enable interrupt for event DMARXMATCH\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dmarxmatch0(&mut self) -> Dmarxmatch0W<IntensetSpec> {
        Dmarxmatch0W::new(self, 22)
    }
    #[doc = "Bit 23 - Write '1' to enable interrupt for event DMARXMATCH\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dmarxmatch1(&mut self) -> Dmarxmatch1W<IntensetSpec> {
        Dmarxmatch1W::new(self, 23)
    }
    #[doc = "Bit 24 - Write '1' to enable interrupt for event DMARXMATCH\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dmarxmatch2(&mut self) -> Dmarxmatch2W<IntensetSpec> {
        Dmarxmatch2W::new(self, 24)
    }
    #[doc = "Bit 25 - Write '1' to enable interrupt for event DMARXMATCH\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dmarxmatch3(&mut self) -> Dmarxmatch3W<IntensetSpec> {
        Dmarxmatch3W::new(self, 25)
    }
    #[doc = "Bit 26 - Write '1' to enable interrupt for event DMATXEND"]
    #[inline(always)]
    #[must_use]
    pub fn dmatxend(&mut self) -> DmatxendW<IntensetSpec> {
        DmatxendW::new(self, 26)
    }
    #[doc = "Bit 27 - Write '1' to enable interrupt for event DMATXREADY"]
    #[inline(always)]
    #[must_use]
    pub fn dmatxready(&mut self) -> DmatxreadyW<IntensetSpec> {
        DmatxreadyW::new(self, 27)
    }
    #[doc = "Bit 28 - Write '1' to enable interrupt for event DMATXBUSERROR"]
    #[inline(always)]
    #[must_use]
    pub fn dmatxbuserror(&mut self) -> DmatxbuserrorW<IntensetSpec> {
        DmatxbuserrorW::new(self, 28)
    }
    #[doc = "Bit 29 - Write '1' to enable interrupt for event FRAMETIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn frametimeout(&mut self) -> FrametimeoutW<IntensetSpec> {
        FrametimeoutW::new(self, 29)
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
