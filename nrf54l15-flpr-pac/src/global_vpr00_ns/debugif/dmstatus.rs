#[doc = "Register `DMSTATUS` reader"]
pub type R = crate::R<DmstatusSpec>;
#[doc = "Version of the debug module.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Version {
    #[doc = "0: Debug module not present."]
    NotPresent = 0,
    #[doc = "1: There is a Debug Module and it conforms to version 0.11 of this specifcation."]
    V011 = 1,
    #[doc = "2: There is a Debug Module and it conforms to version 0.13 of this specifcation."]
    V013 = 2,
    #[doc = "15: There is a Debug Module but it does not conform to any available version of the spec."]
    NonConform = 15,
}
impl From<Version> for u8 {
    #[inline(always)]
    fn from(variant: Version) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Version {
    type Ux = u8;
}
impl crate::IsEnum for Version {}
#[doc = "Field `VERSION` reader - Version of the debug module."]
pub type VersionR = crate::FieldReader<Version>;
impl VersionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Version> {
        match self.bits {
            0 => Some(Version::NotPresent),
            1 => Some(Version::V011),
            2 => Some(Version::V013),
            15 => Some(Version::NonConform),
            _ => None,
        }
    }
    #[doc = "Debug module not present."]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Version::NotPresent
    }
    #[doc = "There is a Debug Module and it conforms to version 0.11 of this specifcation."]
    #[inline(always)]
    pub fn is_v011(&self) -> bool {
        *self == Version::V011
    }
    #[doc = "There is a Debug Module and it conforms to version 0.13 of this specifcation."]
    #[inline(always)]
    pub fn is_v013(&self) -> bool {
        *self == Version::V013
    }
    #[doc = "There is a Debug Module but it does not conform to any available version of the spec."]
    #[inline(always)]
    pub fn is_non_conform(&self) -> bool {
        *self == Version::NonConform
    }
}
#[doc = "Configuration string.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Confstrptrvalid {
    #[doc = "0: The confstrptr0..confstrptr3 holds information which is not relevant to the configuration string."]
    NotRelevant = 0,
    #[doc = "1: The confstrptr0..confstrptr3 holds the address of the configuration string."]
    Address = 1,
}
impl From<Confstrptrvalid> for bool {
    #[inline(always)]
    fn from(variant: Confstrptrvalid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONFSTRPTRVALID` reader - Configuration string."]
pub type ConfstrptrvalidR = crate::BitReader<Confstrptrvalid>;
impl ConfstrptrvalidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Confstrptrvalid {
        match self.bits {
            false => Confstrptrvalid::NotRelevant,
            true => Confstrptrvalid::Address,
        }
    }
    #[doc = "The confstrptr0..confstrptr3 holds information which is not relevant to the configuration string."]
    #[inline(always)]
    pub fn is_not_relevant(&self) -> bool {
        *self == Confstrptrvalid::NotRelevant
    }
    #[doc = "The confstrptr0..confstrptr3 holds the address of the configuration string."]
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == Confstrptrvalid::Address
    }
}
#[doc = "Halt-on-reset support status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hasresethaltreq {
    #[doc = "0: Halt-on-reset is supported."]
    No = 0,
    #[doc = "1: Halt-on-reset is not supported."]
    Yes = 1,
}
impl From<Hasresethaltreq> for bool {
    #[inline(always)]
    fn from(variant: Hasresethaltreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASRESETHALTREQ` reader - Halt-on-reset support status."]
pub type HasresethaltreqR = crate::BitReader<Hasresethaltreq>;
impl HasresethaltreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hasresethaltreq {
        match self.bits {
            false => Hasresethaltreq::No,
            true => Hasresethaltreq::Yes,
        }
    }
    #[doc = "Halt-on-reset is supported."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Hasresethaltreq::No
    }
    #[doc = "Halt-on-reset is not supported."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Hasresethaltreq::Yes
    }
}
#[doc = "Authentication busy status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Authbusy {
    #[doc = "0: The authentication module is ready."]
    No = 0,
    #[doc = "1: The authentication module is busy."]
    Yes = 1,
}
impl From<Authbusy> for bool {
    #[inline(always)]
    fn from(variant: Authbusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTHBUSY` reader - Authentication busy status."]
pub type AuthbusyR = crate::BitReader<Authbusy>;
impl AuthbusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Authbusy {
        match self.bits {
            false => Authbusy::No,
            true => Authbusy::Yes,
        }
    }
    #[doc = "The authentication module is ready."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Authbusy::No
    }
    #[doc = "The authentication module is busy."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Authbusy::Yes
    }
}
#[doc = "Authentication status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Authenticated {
    #[doc = "0: Authentication required before using the debug module."]
    No = 0,
    #[doc = "1: Authentication passed."]
    Yes = 1,
}
impl From<Authenticated> for bool {
    #[inline(always)]
    fn from(variant: Authenticated) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTHENTICATED` reader - Authentication status."]
pub type AuthenticatedR = crate::BitReader<Authenticated>;
impl AuthenticatedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Authenticated {
        match self.bits {
            false => Authenticated::No,
            true => Authenticated::Yes,
        }
    }
    #[doc = "Authentication required before using the debug module."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Authenticated::No
    }
    #[doc = "Authentication passed."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Authenticated::Yes
    }
}
#[doc = "Any currently selected harts halted status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anyhalted {
    #[doc = "0: None of the currently selected harts halted."]
    No = 0,
    #[doc = "1: Any of the currently selected harts halted."]
    Yes = 1,
}
impl From<Anyhalted> for bool {
    #[inline(always)]
    fn from(variant: Anyhalted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANYHALTED` reader - Any currently selected harts halted status."]
pub type AnyhaltedR = crate::BitReader<Anyhalted>;
impl AnyhaltedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anyhalted {
        match self.bits {
            false => Anyhalted::No,
            true => Anyhalted::Yes,
        }
    }
    #[doc = "None of the currently selected harts halted."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Anyhalted::No
    }
    #[doc = "Any of the currently selected harts halted."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Anyhalted::Yes
    }
}
#[doc = "All currently selected harts halted status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Allhalted {
    #[doc = "0: Not all of the currently selected harts halted."]
    No = 0,
    #[doc = "1: All of the currently selected harts halted."]
    Yes = 1,
}
impl From<Allhalted> for bool {
    #[inline(always)]
    fn from(variant: Allhalted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALLHALTED` reader - All currently selected harts halted status."]
pub type AllhaltedR = crate::BitReader<Allhalted>;
impl AllhaltedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Allhalted {
        match self.bits {
            false => Allhalted::No,
            true => Allhalted::Yes,
        }
    }
    #[doc = "Not all of the currently selected harts halted."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Allhalted::No
    }
    #[doc = "All of the currently selected harts halted."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Allhalted::Yes
    }
}
#[doc = "Any currently selected harts running status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anyrunning {
    #[doc = "0: None of the currently selected harts running."]
    No = 0,
    #[doc = "1: Any of the currently selected harts running."]
    Yes = 1,
}
impl From<Anyrunning> for bool {
    #[inline(always)]
    fn from(variant: Anyrunning) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANYRUNNING` reader - Any currently selected harts running status."]
pub type AnyrunningR = crate::BitReader<Anyrunning>;
impl AnyrunningR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anyrunning {
        match self.bits {
            false => Anyrunning::No,
            true => Anyrunning::Yes,
        }
    }
    #[doc = "None of the currently selected harts running."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Anyrunning::No
    }
    #[doc = "Any of the currently selected harts running."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Anyrunning::Yes
    }
}
#[doc = "All currently selected harts running status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Allrunning {
    #[doc = "0: Not all of the currently selected harts running."]
    No = 0,
    #[doc = "1: All of the currently selected harts running."]
    Yes = 1,
}
impl From<Allrunning> for bool {
    #[inline(always)]
    fn from(variant: Allrunning) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALLRUNNING` reader - All currently selected harts running status."]
pub type AllrunningR = crate::BitReader<Allrunning>;
impl AllrunningR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Allrunning {
        match self.bits {
            false => Allrunning::No,
            true => Allrunning::Yes,
        }
    }
    #[doc = "Not all of the currently selected harts running."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Allrunning::No
    }
    #[doc = "All of the currently selected harts running."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Allrunning::Yes
    }
}
#[doc = "Any currently selected harts unavailable status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anyunavail {
    #[doc = "0: None of the currently selected harts unavailable."]
    No = 0,
    #[doc = "1: Any of the currently selected harts unavailable."]
    Yes = 1,
}
impl From<Anyunavail> for bool {
    #[inline(always)]
    fn from(variant: Anyunavail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANYUNAVAIL` reader - Any currently selected harts unavailable status."]
pub type AnyunavailR = crate::BitReader<Anyunavail>;
impl AnyunavailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anyunavail {
        match self.bits {
            false => Anyunavail::No,
            true => Anyunavail::Yes,
        }
    }
    #[doc = "None of the currently selected harts unavailable."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Anyunavail::No
    }
    #[doc = "Any of the currently selected harts unavailable."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Anyunavail::Yes
    }
}
#[doc = "All currently selected harts unavailable status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Allunavail {
    #[doc = "0: Not all of the currently selected harts unavailable."]
    No = 0,
    #[doc = "1: All of the currently selected harts unavailable."]
    Yes = 1,
}
impl From<Allunavail> for bool {
    #[inline(always)]
    fn from(variant: Allunavail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALLUNAVAIL` reader - All currently selected harts unavailable status."]
pub type AllunavailR = crate::BitReader<Allunavail>;
impl AllunavailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Allunavail {
        match self.bits {
            false => Allunavail::No,
            true => Allunavail::Yes,
        }
    }
    #[doc = "Not all of the currently selected harts unavailable."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Allunavail::No
    }
    #[doc = "All of the currently selected harts unavailable."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Allunavail::Yes
    }
}
#[doc = "Any currently selected harts nonexistent status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anynonexistent {
    #[doc = "0: None of the currently selected harts nonexistent."]
    No = 0,
    #[doc = "1: Any of the currently selected harts nonexistent."]
    Yes = 1,
}
impl From<Anynonexistent> for bool {
    #[inline(always)]
    fn from(variant: Anynonexistent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANYNONEXISTENT` reader - Any currently selected harts nonexistent status."]
pub type AnynonexistentR = crate::BitReader<Anynonexistent>;
impl AnynonexistentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anynonexistent {
        match self.bits {
            false => Anynonexistent::No,
            true => Anynonexistent::Yes,
        }
    }
    #[doc = "None of the currently selected harts nonexistent."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Anynonexistent::No
    }
    #[doc = "Any of the currently selected harts nonexistent."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Anynonexistent::Yes
    }
}
#[doc = "All currently selected harts nonexistent status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Allnonexistent {
    #[doc = "0: Not all of the currently selected harts nonexistent."]
    No = 0,
    #[doc = "1: All of the currently selected harts nonexistent."]
    Yes = 1,
}
impl From<Allnonexistent> for bool {
    #[inline(always)]
    fn from(variant: Allnonexistent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALLNONEXISTENT` reader - All currently selected harts nonexistent status."]
pub type AllnonexistentR = crate::BitReader<Allnonexistent>;
impl AllnonexistentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Allnonexistent {
        match self.bits {
            false => Allnonexistent::No,
            true => Allnonexistent::Yes,
        }
    }
    #[doc = "Not all of the currently selected harts nonexistent."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Allnonexistent::No
    }
    #[doc = "All of the currently selected harts nonexistent."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Allnonexistent::Yes
    }
}
#[doc = "Any currently selected harts acknowledged last resume request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anyresumeack {
    #[doc = "0: None of the currently selected harts acknowledged last resume request."]
    No = 0,
    #[doc = "1: Any of the currently selected harts acknowledged last resume request."]
    Yes = 1,
}
impl From<Anyresumeack> for bool {
    #[inline(always)]
    fn from(variant: Anyresumeack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANYRESUMEACK` reader - Any currently selected harts acknowledged last resume request."]
pub type AnyresumeackR = crate::BitReader<Anyresumeack>;
impl AnyresumeackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anyresumeack {
        match self.bits {
            false => Anyresumeack::No,
            true => Anyresumeack::Yes,
        }
    }
    #[doc = "None of the currently selected harts acknowledged last resume request."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Anyresumeack::No
    }
    #[doc = "Any of the currently selected harts acknowledged last resume request."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Anyresumeack::Yes
    }
}
#[doc = "All currently selected harts acknowledged last resume\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Allresumeack {
    #[doc = "0: Not all of the currently selected harts acknowledged last resume request."]
    No = 0,
    #[doc = "1: All of the currently selected harts acknowledged last resume request."]
    Yes = 1,
}
impl From<Allresumeack> for bool {
    #[inline(always)]
    fn from(variant: Allresumeack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALLRESUMEACK` reader - All currently selected harts acknowledged last resume"]
pub type AllresumeackR = crate::BitReader<Allresumeack>;
impl AllresumeackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Allresumeack {
        match self.bits {
            false => Allresumeack::No,
            true => Allresumeack::Yes,
        }
    }
    #[doc = "Not all of the currently selected harts acknowledged last resume request."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Allresumeack::No
    }
    #[doc = "All of the currently selected harts acknowledged last resume request."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Allresumeack::Yes
    }
}
#[doc = "Any currently selected harts have been reset and reset is not acknowledged.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anyhavereset {
    #[doc = "0: None of the currently selected harts have been reset and reset is not acknowledget."]
    No = 0,
    #[doc = "1: Any of the currently selected harts have been reset and reset is not acknowledge."]
    Yes = 1,
}
impl From<Anyhavereset> for bool {
    #[inline(always)]
    fn from(variant: Anyhavereset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANYHAVERESET` reader - Any currently selected harts have been reset and reset is not acknowledged."]
pub type AnyhaveresetR = crate::BitReader<Anyhavereset>;
impl AnyhaveresetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anyhavereset {
        match self.bits {
            false => Anyhavereset::No,
            true => Anyhavereset::Yes,
        }
    }
    #[doc = "None of the currently selected harts have been reset and reset is not acknowledget."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Anyhavereset::No
    }
    #[doc = "Any of the currently selected harts have been reset and reset is not acknowledge."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Anyhavereset::Yes
    }
}
#[doc = "All currently selected harts have been reset and reset is not acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Allhavereset {
    #[doc = "0: Not all of the currently selected harts have been reset and reset is not acknowledge."]
    No = 0,
    #[doc = "1: All of the currently selected harts have been reset and reset is not acknowledge."]
    Yes = 1,
}
impl From<Allhavereset> for bool {
    #[inline(always)]
    fn from(variant: Allhavereset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALLHAVERESET` reader - All currently selected harts have been reset and reset is not acknowledge"]
pub type AllhaveresetR = crate::BitReader<Allhavereset>;
impl AllhaveresetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Allhavereset {
        match self.bits {
            false => Allhavereset::No,
            true => Allhavereset::Yes,
        }
    }
    #[doc = "Not all of the currently selected harts have been reset and reset is not acknowledge."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Allhavereset::No
    }
    #[doc = "All of the currently selected harts have been reset and reset is not acknowledge."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Allhavereset::Yes
    }
}
#[doc = "Implicit ebreak instruction at the non-existent word immediately after the Program Buffer.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Impebreak {
    #[doc = "0: No implicit ebreak instruction."]
    No = 0,
    #[doc = "1: Implicit ebreak instruction."]
    Yes = 1,
}
impl From<Impebreak> for bool {
    #[inline(always)]
    fn from(variant: Impebreak) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMPEBREAK` reader - Implicit ebreak instruction at the non-existent word immediately after the Program Buffer."]
pub type ImpebreakR = crate::BitReader<Impebreak>;
impl ImpebreakR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Impebreak {
        match self.bits {
            false => Impebreak::No,
            true => Impebreak::Yes,
        }
    }
    #[doc = "No implicit ebreak instruction."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Impebreak::No
    }
    #[doc = "Implicit ebreak instruction."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Impebreak::Yes
    }
}
impl R {
    #[doc = "Bits 0:3 - Version of the debug module."]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Configuration string."]
    #[inline(always)]
    pub fn confstrptrvalid(&self) -> ConfstrptrvalidR {
        ConfstrptrvalidR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Halt-on-reset support status."]
    #[inline(always)]
    pub fn hasresethaltreq(&self) -> HasresethaltreqR {
        HasresethaltreqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Authentication busy status."]
    #[inline(always)]
    pub fn authbusy(&self) -> AuthbusyR {
        AuthbusyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Authentication status."]
    #[inline(always)]
    pub fn authenticated(&self) -> AuthenticatedR {
        AuthenticatedR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Any currently selected harts halted status."]
    #[inline(always)]
    pub fn anyhalted(&self) -> AnyhaltedR {
        AnyhaltedR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - All currently selected harts halted status."]
    #[inline(always)]
    pub fn allhalted(&self) -> AllhaltedR {
        AllhaltedR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Any currently selected harts running status."]
    #[inline(always)]
    pub fn anyrunning(&self) -> AnyrunningR {
        AnyrunningR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - All currently selected harts running status."]
    #[inline(always)]
    pub fn allrunning(&self) -> AllrunningR {
        AllrunningR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Any currently selected harts unavailable status."]
    #[inline(always)]
    pub fn anyunavail(&self) -> AnyunavailR {
        AnyunavailR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - All currently selected harts unavailable status."]
    #[inline(always)]
    pub fn allunavail(&self) -> AllunavailR {
        AllunavailR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Any currently selected harts nonexistent status."]
    #[inline(always)]
    pub fn anynonexistent(&self) -> AnynonexistentR {
        AnynonexistentR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - All currently selected harts nonexistent status."]
    #[inline(always)]
    pub fn allnonexistent(&self) -> AllnonexistentR {
        AllnonexistentR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Any currently selected harts acknowledged last resume request."]
    #[inline(always)]
    pub fn anyresumeack(&self) -> AnyresumeackR {
        AnyresumeackR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - All currently selected harts acknowledged last resume"]
    #[inline(always)]
    pub fn allresumeack(&self) -> AllresumeackR {
        AllresumeackR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Any currently selected harts have been reset and reset is not acknowledged."]
    #[inline(always)]
    pub fn anyhavereset(&self) -> AnyhaveresetR {
        AnyhaveresetR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - All currently selected harts have been reset and reset is not acknowledge"]
    #[inline(always)]
    pub fn allhavereset(&self) -> AllhaveresetR {
        AllhaveresetR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - Implicit ebreak instruction at the non-existent word immediately after the Program Buffer."]
    #[inline(always)]
    pub fn impebreak(&self) -> ImpebreakR {
        ImpebreakR::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "Debug Module Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dmstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmstatusSpec;
impl crate::RegisterSpec for DmstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmstatus::R`](R) reader structure"]
impl crate::Readable for DmstatusSpec {}
#[doc = "`reset()` method sets DMSTATUS to value 0x0040_0082"]
impl crate::Resettable for DmstatusSpec {
    const RESET_VALUE: u32 = 0x0040_0082;
}
