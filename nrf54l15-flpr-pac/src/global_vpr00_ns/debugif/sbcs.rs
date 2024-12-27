#[doc = "Register `SBCS` reader"]
pub type R = crate::R<SbcsSpec>;
#[doc = "Register `SBCS` writer"]
pub type W = crate::W<SbcsSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbaccess8 {
    #[doc = "1: 8-bit system bus accesses are supported."]
    Sbaccess8 = 1,
}
impl From<Sbaccess8> for bool {
    #[inline(always)]
    fn from(variant: Sbaccess8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBACCESS8` reader - "]
pub type Sbaccess8R = crate::BitReader<Sbaccess8>;
impl Sbaccess8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sbaccess8> {
        match self.bits {
            true => Some(Sbaccess8::Sbaccess8),
            _ => None,
        }
    }
    #[doc = "8-bit system bus accesses are supported."]
    #[inline(always)]
    pub fn is_sbaccess8(&self) -> bool {
        *self == Sbaccess8::Sbaccess8
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbaccess16 {
    #[doc = "1: 16-bit system bus accesses are supported."]
    Sbaccess16 = 1,
}
impl From<Sbaccess16> for bool {
    #[inline(always)]
    fn from(variant: Sbaccess16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBACCESS16` reader - "]
pub type Sbaccess16R = crate::BitReader<Sbaccess16>;
impl Sbaccess16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sbaccess16> {
        match self.bits {
            true => Some(Sbaccess16::Sbaccess16),
            _ => None,
        }
    }
    #[doc = "16-bit system bus accesses are supported."]
    #[inline(always)]
    pub fn is_sbaccess16(&self) -> bool {
        *self == Sbaccess16::Sbaccess16
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbaccess32 {
    #[doc = "1: 32-bit system bus accesses are supported."]
    Sbaccess32 = 1,
}
impl From<Sbaccess32> for bool {
    #[inline(always)]
    fn from(variant: Sbaccess32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBACCESS32` reader - "]
pub type Sbaccess32R = crate::BitReader<Sbaccess32>;
impl Sbaccess32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sbaccess32> {
        match self.bits {
            true => Some(Sbaccess32::Sbaccess32),
            _ => None,
        }
    }
    #[doc = "32-bit system bus accesses are supported."]
    #[inline(always)]
    pub fn is_sbaccess32(&self) -> bool {
        *self == Sbaccess32::Sbaccess32
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbaccess64 {
    #[doc = "1: 64-bit system bus accesses are supported."]
    Sbaccess64 = 1,
}
impl From<Sbaccess64> for bool {
    #[inline(always)]
    fn from(variant: Sbaccess64) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBACCESS64` reader - "]
pub type Sbaccess64R = crate::BitReader<Sbaccess64>;
impl Sbaccess64R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sbaccess64> {
        match self.bits {
            true => Some(Sbaccess64::Sbaccess64),
            _ => None,
        }
    }
    #[doc = "64-bit system bus accesses are supported."]
    #[inline(always)]
    pub fn is_sbaccess64(&self) -> bool {
        *self == Sbaccess64::Sbaccess64
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbaccess128 {
    #[doc = "1: 128-bit system bus accesses are supported."]
    Sbaccess128 = 1,
}
impl From<Sbaccess128> for bool {
    #[inline(always)]
    fn from(variant: Sbaccess128) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBACCESS128` reader - "]
pub type Sbaccess128R = crate::BitReader<Sbaccess128>;
impl Sbaccess128R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sbaccess128> {
        match self.bits {
            true => Some(Sbaccess128::Sbaccess128),
            _ => None,
        }
    }
    #[doc = "128-bit system bus accesses are supported."]
    #[inline(always)]
    pub fn is_sbaccess128(&self) -> bool {
        *self == Sbaccess128::Sbaccess128
    }
}
#[doc = "Field `SBASIZE` reader - Width of system bus addresses in bits. (0 indicates there is no bus access support.)"]
pub type SbasizeR = crate::FieldReader;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sberror {
    #[doc = "0: There was no bus error."]
    Normal = 0,
    #[doc = "1: There was a timeout."]
    Timeout = 1,
    #[doc = "2: A bad address was accessed."]
    Address = 2,
    #[doc = "3: There was an alignment error."]
    Alignment = 3,
    #[doc = "4: An access of unsupported size was requested."]
    Size = 4,
    #[doc = "7: Other."]
    Other = 7,
}
impl From<Sberror> for u8 {
    #[inline(always)]
    fn from(variant: Sberror) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sberror {
    type Ux = u8;
}
impl crate::IsEnum for Sberror {}
#[doc = "Field `SBERROR` reader - "]
pub type SberrorR = crate::FieldReader<Sberror>;
impl SberrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sberror> {
        match self.bits {
            0 => Some(Sberror::Normal),
            1 => Some(Sberror::Timeout),
            2 => Some(Sberror::Address),
            3 => Some(Sberror::Alignment),
            4 => Some(Sberror::Size),
            7 => Some(Sberror::Other),
            _ => None,
        }
    }
    #[doc = "There was no bus error."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Sberror::Normal
    }
    #[doc = "There was a timeout."]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == Sberror::Timeout
    }
    #[doc = "A bad address was accessed."]
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == Sberror::Address
    }
    #[doc = "There was an alignment error."]
    #[inline(always)]
    pub fn is_alignment(&self) -> bool {
        *self == Sberror::Alignment
    }
    #[doc = "An access of unsupported size was requested."]
    #[inline(always)]
    pub fn is_size(&self) -> bool {
        *self == Sberror::Size
    }
    #[doc = "Other."]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == Sberror::Other
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbreadondata {
    #[doc = "1: Every read from sbdata0 automatically triggers a system bus read at the (possibly autoincremented) address."]
    Sbreadondata = 1,
}
impl From<Sbreadondata> for bool {
    #[inline(always)]
    fn from(variant: Sbreadondata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBREADONDATA` reader - "]
pub type SbreadondataR = crate::BitReader<Sbreadondata>;
impl SbreadondataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sbreadondata> {
        match self.bits {
            true => Some(Sbreadondata::Sbreadondata),
            _ => None,
        }
    }
    #[doc = "Every read from sbdata0 automatically triggers a system bus read at the (possibly autoincremented) address."]
    #[inline(always)]
    pub fn is_sbreadondata(&self) -> bool {
        *self == Sbreadondata::Sbreadondata
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbautoincrement {
    #[doc = "1: sbaddress is incremented by the access size (in bytes) selected in sbaccess after every system bus access."]
    Sbautoincrement = 1,
}
impl From<Sbautoincrement> for bool {
    #[inline(always)]
    fn from(variant: Sbautoincrement) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBAUTOINCREMENT` reader - "]
pub type SbautoincrementR = crate::BitReader<Sbautoincrement>;
impl SbautoincrementR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sbautoincrement> {
        match self.bits {
            true => Some(Sbautoincrement::Sbautoincrement),
            _ => None,
        }
    }
    #[doc = "sbaddress is incremented by the access size (in bytes) selected in sbaccess after every system bus access."]
    #[inline(always)]
    pub fn is_sbautoincrement(&self) -> bool {
        *self == Sbautoincrement::Sbautoincrement
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sbaccess {
    #[doc = "0: 8-bit."]
    Size8 = 0,
    #[doc = "1: 16-bit."]
    Size16 = 1,
    #[doc = "2: 32-bit."]
    Size32 = 2,
    #[doc = "3: 64-bit."]
    Size64 = 3,
    #[doc = "4: 128-bit."]
    Size128 = 4,
}
impl From<Sbaccess> for u8 {
    #[inline(always)]
    fn from(variant: Sbaccess) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sbaccess {
    type Ux = u8;
}
impl crate::IsEnum for Sbaccess {}
#[doc = "Field `SBACCESS` reader - "]
pub type SbaccessR = crate::FieldReader<Sbaccess>;
impl SbaccessR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sbaccess> {
        match self.bits {
            0 => Some(Sbaccess::Size8),
            1 => Some(Sbaccess::Size16),
            2 => Some(Sbaccess::Size32),
            3 => Some(Sbaccess::Size64),
            4 => Some(Sbaccess::Size128),
            _ => None,
        }
    }
    #[doc = "8-bit."]
    #[inline(always)]
    pub fn is_size8(&self) -> bool {
        *self == Sbaccess::Size8
    }
    #[doc = "16-bit."]
    #[inline(always)]
    pub fn is_size16(&self) -> bool {
        *self == Sbaccess::Size16
    }
    #[doc = "32-bit."]
    #[inline(always)]
    pub fn is_size32(&self) -> bool {
        *self == Sbaccess::Size32
    }
    #[doc = "64-bit."]
    #[inline(always)]
    pub fn is_size64(&self) -> bool {
        *self == Sbaccess::Size64
    }
    #[doc = "128-bit."]
    #[inline(always)]
    pub fn is_size128(&self) -> bool {
        *self == Sbaccess::Size128
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbreadonaddr {
    #[doc = "1: Every write to sbaddress0 automatically triggers a system bus read at the new address."]
    Sbreadonaddr = 1,
}
impl From<Sbreadonaddr> for bool {
    #[inline(always)]
    fn from(variant: Sbreadonaddr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBREADONADDR` reader - "]
pub type SbreadonaddrR = crate::BitReader<Sbreadonaddr>;
impl SbreadonaddrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sbreadonaddr> {
        match self.bits {
            true => Some(Sbreadonaddr::Sbreadonaddr),
            _ => None,
        }
    }
    #[doc = "Every write to sbaddress0 automatically triggers a system bus read at the new address."]
    #[inline(always)]
    pub fn is_sbreadonaddr(&self) -> bool {
        *self == Sbreadonaddr::Sbreadonaddr
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbbusy {
    #[doc = "0: System bus master is not busy."]
    Notbusy = 0,
    #[doc = "1: System bus master is busy."]
    Busy = 1,
}
impl From<Sbbusy> for bool {
    #[inline(always)]
    fn from(variant: Sbbusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBBUSY` reader - "]
pub type SbbusyR = crate::BitReader<Sbbusy>;
impl SbbusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbbusy {
        match self.bits {
            false => Sbbusy::Notbusy,
            true => Sbbusy::Busy,
        }
    }
    #[doc = "System bus master is not busy."]
    #[inline(always)]
    pub fn is_notbusy(&self) -> bool {
        *self == Sbbusy::Notbusy
    }
    #[doc = "System bus master is busy."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Sbbusy::Busy
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbbusyerror {
    #[doc = "0: No error."]
    Noerror = 0,
    #[doc = "1: Debugger access attempted while one in progress."]
    Error = 1,
}
impl From<Sbbusyerror> for bool {
    #[inline(always)]
    fn from(variant: Sbbusyerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBBUSYERROR` reader - "]
pub type SbbusyerrorR = crate::BitReader<Sbbusyerror>;
impl SbbusyerrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbbusyerror {
        match self.bits {
            false => Sbbusyerror::Noerror,
            true => Sbbusyerror::Error,
        }
    }
    #[doc = "No error."]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Sbbusyerror::Noerror
    }
    #[doc = "Debugger access attempted while one in progress."]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Sbbusyerror::Error
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sbversion {
    #[doc = "0: The System Bus interface conforms to mainline drafts of thia RISC-V External Debug Support spec older than 1 January, 2018."]
    Version0 = 0,
    #[doc = "1: The System Bus interface conforms to RISC-V External Debug Support version 0.14.0-DRAFT. Other values are reserved for future versions."]
    Version1 = 1,
}
impl From<Sbversion> for u8 {
    #[inline(always)]
    fn from(variant: Sbversion) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sbversion {
    type Ux = u8;
}
impl crate::IsEnum for Sbversion {}
#[doc = "Field `SBVERSION` reader - "]
pub type SbversionR = crate::FieldReader<Sbversion>;
impl SbversionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sbversion> {
        match self.bits {
            0 => Some(Sbversion::Version0),
            1 => Some(Sbversion::Version1),
            _ => None,
        }
    }
    #[doc = "The System Bus interface conforms to mainline drafts of thia RISC-V External Debug Support spec older than 1 January, 2018."]
    #[inline(always)]
    pub fn is_version0(&self) -> bool {
        *self == Sbversion::Version0
    }
    #[doc = "The System Bus interface conforms to RISC-V External Debug Support version 0.14.0-DRAFT. Other values are reserved for future versions."]
    #[inline(always)]
    pub fn is_version1(&self) -> bool {
        *self == Sbversion::Version1
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sbaccess8(&self) -> Sbaccess8R {
        Sbaccess8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sbaccess16(&self) -> Sbaccess16R {
        Sbaccess16R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sbaccess32(&self) -> Sbaccess32R {
        Sbaccess32R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sbaccess64(&self) -> Sbaccess64R {
        Sbaccess64R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sbaccess128(&self) -> Sbaccess128R {
        Sbaccess128R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:11 - Width of system bus addresses in bits. (0 indicates there is no bus access support.)"]
    #[inline(always)]
    pub fn sbasize(&self) -> SbasizeR {
        SbasizeR::new(((self.bits >> 5) & 0x7f) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn sberror(&self) -> SberrorR {
        SberrorR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn sbreadondata(&self) -> SbreadondataR {
        SbreadondataR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sbautoincrement(&self) -> SbautoincrementR {
        SbautoincrementR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn sbaccess(&self) -> SbaccessR {
        SbaccessR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sbreadonaddr(&self) -> SbreadonaddrR {
        SbreadonaddrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn sbbusy(&self) -> SbbusyR {
        SbbusyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sbbusyerror(&self) -> SbbusyerrorR {
        SbbusyerrorR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn sbversion(&self) -> SbversionR {
        SbversionR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {}
#[doc = "System Bus Access Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`sbcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SbcsSpec;
impl crate::RegisterSpec for SbcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbcs::R`](R) reader structure"]
impl crate::Readable for SbcsSpec {}
#[doc = "`write(|w| ..)` method takes [`sbcs::W`](W) writer structure"]
impl crate::Writable for SbcsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SBCS to value 0x2000_0000"]
impl crate::Resettable for SbcsSpec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
