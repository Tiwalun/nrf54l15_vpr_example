#[doc = "Register `INTPEND` reader"]
pub type R = crate::R<IntpendSpec>;
#[doc = "Read pending status of interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopped {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Stopped> for bool {
    #[inline(always)]
    fn from(variant: Stopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` reader - Read pending status of interrupt for event STOPPED"]
pub type StoppedR = crate::BitReader<Stopped>;
impl StoppedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopped {
        match self.bits {
            false => Stopped::NotPending,
            true => Stopped::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Stopped::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Stopped::Pending
    }
}
#[doc = "Read pending status of interrupt for event SEQSTARTED\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqstarted0 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Seqstarted0> for bool {
    #[inline(always)]
    fn from(variant: Seqstarted0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQSTARTED0` reader - Read pending status of interrupt for event SEQSTARTED\\[0\\]"]
pub type Seqstarted0R = crate::BitReader<Seqstarted0>;
impl Seqstarted0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seqstarted0 {
        match self.bits {
            false => Seqstarted0::NotPending,
            true => Seqstarted0::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Seqstarted0::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Seqstarted0::Pending
    }
}
#[doc = "Read pending status of interrupt for event SEQSTARTED\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqstarted1 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Seqstarted1> for bool {
    #[inline(always)]
    fn from(variant: Seqstarted1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQSTARTED1` reader - Read pending status of interrupt for event SEQSTARTED\\[1\\]"]
pub type Seqstarted1R = crate::BitReader<Seqstarted1>;
impl Seqstarted1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seqstarted1 {
        match self.bits {
            false => Seqstarted1::NotPending,
            true => Seqstarted1::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Seqstarted1::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Seqstarted1::Pending
    }
}
#[doc = "Read pending status of interrupt for event SEQEND\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqend0 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Seqend0> for bool {
    #[inline(always)]
    fn from(variant: Seqend0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND0` reader - Read pending status of interrupt for event SEQEND\\[0\\]"]
pub type Seqend0R = crate::BitReader<Seqend0>;
impl Seqend0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seqend0 {
        match self.bits {
            false => Seqend0::NotPending,
            true => Seqend0::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Seqend0::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Seqend0::Pending
    }
}
#[doc = "Read pending status of interrupt for event SEQEND\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqend1 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Seqend1> for bool {
    #[inline(always)]
    fn from(variant: Seqend1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND1` reader - Read pending status of interrupt for event SEQEND\\[1\\]"]
pub type Seqend1R = crate::BitReader<Seqend1>;
impl Seqend1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seqend1 {
        match self.bits {
            false => Seqend1::NotPending,
            true => Seqend1::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Seqend1::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Seqend1::Pending
    }
}
#[doc = "Read pending status of interrupt for event PWMPERIODEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmperiodend {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Pwmperiodend> for bool {
    #[inline(always)]
    fn from(variant: Pwmperiodend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMPERIODEND` reader - Read pending status of interrupt for event PWMPERIODEND"]
pub type PwmperiodendR = crate::BitReader<Pwmperiodend>;
impl PwmperiodendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmperiodend {
        match self.bits {
            false => Pwmperiodend::NotPending,
            true => Pwmperiodend::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Pwmperiodend::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Pwmperiodend::Pending
    }
}
#[doc = "Read pending status of interrupt for event LOOPSDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Loopsdone {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Loopsdone> for bool {
    #[inline(always)]
    fn from(variant: Loopsdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPSDONE` reader - Read pending status of interrupt for event LOOPSDONE"]
pub type LoopsdoneR = crate::BitReader<Loopsdone>;
impl LoopsdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Loopsdone {
        match self.bits {
            false => Loopsdone::NotPending,
            true => Loopsdone::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Loopsdone::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Loopsdone::Pending
    }
}
#[doc = "Read pending status of interrupt for event RAMUNDERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramunderflow {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Ramunderflow> for bool {
    #[inline(always)]
    fn from(variant: Ramunderflow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMUNDERFLOW` reader - Read pending status of interrupt for event RAMUNDERFLOW"]
pub type RamunderflowR = crate::BitReader<Ramunderflow>;
impl RamunderflowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramunderflow {
        match self.bits {
            false => Ramunderflow::NotPending,
            true => Ramunderflow::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Ramunderflow::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Ramunderflow::Pending
    }
}
#[doc = "Read pending status of interrupt for event DMASEQ0END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaseq0end {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Dmaseq0end> for bool {
    #[inline(always)]
    fn from(variant: Dmaseq0end) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMASEQ0END` reader - Read pending status of interrupt for event DMASEQ0END"]
pub type Dmaseq0endR = crate::BitReader<Dmaseq0end>;
impl Dmaseq0endR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaseq0end {
        match self.bits {
            false => Dmaseq0end::NotPending,
            true => Dmaseq0end::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Dmaseq0end::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Dmaseq0end::Pending
    }
}
#[doc = "Read pending status of interrupt for event DMASEQ0READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaseq0ready {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Dmaseq0ready> for bool {
    #[inline(always)]
    fn from(variant: Dmaseq0ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMASEQ0READY` reader - Read pending status of interrupt for event DMASEQ0READY"]
pub type Dmaseq0readyR = crate::BitReader<Dmaseq0ready>;
impl Dmaseq0readyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaseq0ready {
        match self.bits {
            false => Dmaseq0ready::NotPending,
            true => Dmaseq0ready::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Dmaseq0ready::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Dmaseq0ready::Pending
    }
}
#[doc = "Read pending status of interrupt for event DMASEQ0BUSERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaseq0buserror {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Dmaseq0buserror> for bool {
    #[inline(always)]
    fn from(variant: Dmaseq0buserror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMASEQ0BUSERROR` reader - Read pending status of interrupt for event DMASEQ0BUSERROR"]
pub type Dmaseq0buserrorR = crate::BitReader<Dmaseq0buserror>;
impl Dmaseq0buserrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaseq0buserror {
        match self.bits {
            false => Dmaseq0buserror::NotPending,
            true => Dmaseq0buserror::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Dmaseq0buserror::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Dmaseq0buserror::Pending
    }
}
#[doc = "Read pending status of interrupt for event DMASEQ1END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaseq1end {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Dmaseq1end> for bool {
    #[inline(always)]
    fn from(variant: Dmaseq1end) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMASEQ1END` reader - Read pending status of interrupt for event DMASEQ1END"]
pub type Dmaseq1endR = crate::BitReader<Dmaseq1end>;
impl Dmaseq1endR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaseq1end {
        match self.bits {
            false => Dmaseq1end::NotPending,
            true => Dmaseq1end::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Dmaseq1end::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Dmaseq1end::Pending
    }
}
#[doc = "Read pending status of interrupt for event DMASEQ1READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaseq1ready {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Dmaseq1ready> for bool {
    #[inline(always)]
    fn from(variant: Dmaseq1ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMASEQ1READY` reader - Read pending status of interrupt for event DMASEQ1READY"]
pub type Dmaseq1readyR = crate::BitReader<Dmaseq1ready>;
impl Dmaseq1readyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaseq1ready {
        match self.bits {
            false => Dmaseq1ready::NotPending,
            true => Dmaseq1ready::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Dmaseq1ready::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Dmaseq1ready::Pending
    }
}
#[doc = "Read pending status of interrupt for event DMASEQ1BUSERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaseq1buserror {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Dmaseq1buserror> for bool {
    #[inline(always)]
    fn from(variant: Dmaseq1buserror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMASEQ1BUSERROR` reader - Read pending status of interrupt for event DMASEQ1BUSERROR"]
pub type Dmaseq1buserrorR = crate::BitReader<Dmaseq1buserror>;
impl Dmaseq1buserrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaseq1buserror {
        match self.bits {
            false => Dmaseq1buserror::NotPending,
            true => Dmaseq1buserror::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Dmaseq1buserror::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Dmaseq1buserror::Pending
    }
}
#[doc = "Read pending status of interrupt for event COMPAREMATCH\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparematch0 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Comparematch0> for bool {
    #[inline(always)]
    fn from(variant: Comparematch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREMATCH0` reader - Read pending status of interrupt for event COMPAREMATCH\\[0\\]"]
pub type Comparematch0R = crate::BitReader<Comparematch0>;
impl Comparematch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comparematch0 {
        match self.bits {
            false => Comparematch0::NotPending,
            true => Comparematch0::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Comparematch0::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Comparematch0::Pending
    }
}
#[doc = "Read pending status of interrupt for event COMPAREMATCH\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparematch1 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Comparematch1> for bool {
    #[inline(always)]
    fn from(variant: Comparematch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREMATCH1` reader - Read pending status of interrupt for event COMPAREMATCH\\[1\\]"]
pub type Comparematch1R = crate::BitReader<Comparematch1>;
impl Comparematch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comparematch1 {
        match self.bits {
            false => Comparematch1::NotPending,
            true => Comparematch1::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Comparematch1::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Comparematch1::Pending
    }
}
#[doc = "Read pending status of interrupt for event COMPAREMATCH\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparematch2 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Comparematch2> for bool {
    #[inline(always)]
    fn from(variant: Comparematch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREMATCH2` reader - Read pending status of interrupt for event COMPAREMATCH\\[2\\]"]
pub type Comparematch2R = crate::BitReader<Comparematch2>;
impl Comparematch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comparematch2 {
        match self.bits {
            false => Comparematch2::NotPending,
            true => Comparematch2::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Comparematch2::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Comparematch2::Pending
    }
}
#[doc = "Read pending status of interrupt for event COMPAREMATCH\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparematch3 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Comparematch3> for bool {
    #[inline(always)]
    fn from(variant: Comparematch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREMATCH3` reader - Read pending status of interrupt for event COMPAREMATCH\\[3\\]"]
pub type Comparematch3R = crate::BitReader<Comparematch3>;
impl Comparematch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comparematch3 {
        match self.bits {
            false => Comparematch3::NotPending,
            true => Comparematch3::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Comparematch3::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Comparematch3::Pending
    }
}
impl R {
    #[doc = "Bit 1 - Read pending status of interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> StoppedR {
        StoppedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read pending status of interrupt for event SEQSTARTED\\[0\\]"]
    #[inline(always)]
    pub fn seqstarted0(&self) -> Seqstarted0R {
        Seqstarted0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read pending status of interrupt for event SEQSTARTED\\[1\\]"]
    #[inline(always)]
    pub fn seqstarted1(&self) -> Seqstarted1R {
        Seqstarted1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read pending status of interrupt for event SEQEND\\[0\\]"]
    #[inline(always)]
    pub fn seqend0(&self) -> Seqend0R {
        Seqend0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read pending status of interrupt for event SEQEND\\[1\\]"]
    #[inline(always)]
    pub fn seqend1(&self) -> Seqend1R {
        Seqend1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read pending status of interrupt for event PWMPERIODEND"]
    #[inline(always)]
    pub fn pwmperiodend(&self) -> PwmperiodendR {
        PwmperiodendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Read pending status of interrupt for event LOOPSDONE"]
    #[inline(always)]
    pub fn loopsdone(&self) -> LoopsdoneR {
        LoopsdoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Read pending status of interrupt for event RAMUNDERFLOW"]
    #[inline(always)]
    pub fn ramunderflow(&self) -> RamunderflowR {
        RamunderflowR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read pending status of interrupt for event DMASEQ0END"]
    #[inline(always)]
    pub fn dmaseq0end(&self) -> Dmaseq0endR {
        Dmaseq0endR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Read pending status of interrupt for event DMASEQ0READY"]
    #[inline(always)]
    pub fn dmaseq0ready(&self) -> Dmaseq0readyR {
        Dmaseq0readyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Read pending status of interrupt for event DMASEQ0BUSERROR"]
    #[inline(always)]
    pub fn dmaseq0buserror(&self) -> Dmaseq0buserrorR {
        Dmaseq0buserrorR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Read pending status of interrupt for event DMASEQ1END"]
    #[inline(always)]
    pub fn dmaseq1end(&self) -> Dmaseq1endR {
        Dmaseq1endR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Read pending status of interrupt for event DMASEQ1READY"]
    #[inline(always)]
    pub fn dmaseq1ready(&self) -> Dmaseq1readyR {
        Dmaseq1readyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Read pending status of interrupt for event DMASEQ1BUSERROR"]
    #[inline(always)]
    pub fn dmaseq1buserror(&self) -> Dmaseq1buserrorR {
        Dmaseq1buserrorR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Read pending status of interrupt for event COMPAREMATCH\\[0\\]"]
    #[inline(always)]
    pub fn comparematch0(&self) -> Comparematch0R {
        Comparematch0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Read pending status of interrupt for event COMPAREMATCH\\[1\\]"]
    #[inline(always)]
    pub fn comparematch1(&self) -> Comparematch1R {
        Comparematch1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Read pending status of interrupt for event COMPAREMATCH\\[2\\]"]
    #[inline(always)]
    pub fn comparematch2(&self) -> Comparematch2R {
        Comparematch2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Read pending status of interrupt for event COMPAREMATCH\\[3\\]"]
    #[inline(always)]
    pub fn comparematch3(&self) -> Comparematch3R {
        Comparematch3R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "Pending interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`intpend::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntpendSpec;
impl crate::RegisterSpec for IntpendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intpend::R`](R) reader structure"]
impl crate::Readable for IntpendSpec {}
#[doc = "`reset()` method sets INTPEND to value 0"]
impl crate::Resettable for IntpendSpec {
    const RESET_VALUE: u32 = 0;
}
