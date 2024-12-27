#[doc = "Register `MASTERPORT` reader"]
pub type R = crate::R<MasterportSpec>;
#[doc = "Register `MASTERPORT` writer"]
pub type W = crate::W<MasterportSpec>;
#[doc = "Global slave connection information for master port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connection0 {
    #[doc = "0: Master port 0 connection to global slave is disabled"]
    Disabled = 0,
    #[doc = "1: Master port 0 connection to global slave is enabled"]
    Enabled = 1,
}
impl From<Connection0> for bool {
    #[inline(always)]
    fn from(variant: Connection0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECTION_0` reader - Global slave connection information for master port"]
pub type Connection0R = crate::BitReader<Connection0>;
impl Connection0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connection0 {
        match self.bits {
            false => Connection0::Disabled,
            true => Connection0::Enabled,
        }
    }
    #[doc = "Master port 0 connection to global slave is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Connection0::Disabled
    }
    #[doc = "Master port 0 connection to global slave is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Connection0::Enabled
    }
}
#[doc = "Field `CONNECTION_0` writer - Global slave connection information for master port"]
pub type Connection0W<'a, REG> = crate::BitWriter<'a, REG, Connection0>;
impl<'a, REG> Connection0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master port 0 connection to global slave is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection0::Disabled)
    }
    #[doc = "Master port 0 connection to global slave is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection0::Enabled)
    }
}
#[doc = "Global slave connection information for master port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connection1 {
    #[doc = "0: Master port 1 connection to global slave is disabled"]
    Disabled = 0,
    #[doc = "1: Master port 1 connection to global slave is enabled"]
    Enabled = 1,
}
impl From<Connection1> for bool {
    #[inline(always)]
    fn from(variant: Connection1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECTION_1` reader - Global slave connection information for master port"]
pub type Connection1R = crate::BitReader<Connection1>;
impl Connection1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connection1 {
        match self.bits {
            false => Connection1::Disabled,
            true => Connection1::Enabled,
        }
    }
    #[doc = "Master port 1 connection to global slave is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Connection1::Disabled
    }
    #[doc = "Master port 1 connection to global slave is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Connection1::Enabled
    }
}
#[doc = "Field `CONNECTION_1` writer - Global slave connection information for master port"]
pub type Connection1W<'a, REG> = crate::BitWriter<'a, REG, Connection1>;
impl<'a, REG> Connection1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master port 1 connection to global slave is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection1::Disabled)
    }
    #[doc = "Master port 1 connection to global slave is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection1::Enabled)
    }
}
#[doc = "Global slave connection information for master port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connection2 {
    #[doc = "0: Master port 2 connection to global slave is disabled"]
    Disabled = 0,
    #[doc = "1: Master port 2 connection to global slave is enabled"]
    Enabled = 1,
}
impl From<Connection2> for bool {
    #[inline(always)]
    fn from(variant: Connection2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECTION_2` reader - Global slave connection information for master port"]
pub type Connection2R = crate::BitReader<Connection2>;
impl Connection2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connection2 {
        match self.bits {
            false => Connection2::Disabled,
            true => Connection2::Enabled,
        }
    }
    #[doc = "Master port 2 connection to global slave is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Connection2::Disabled
    }
    #[doc = "Master port 2 connection to global slave is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Connection2::Enabled
    }
}
#[doc = "Field `CONNECTION_2` writer - Global slave connection information for master port"]
pub type Connection2W<'a, REG> = crate::BitWriter<'a, REG, Connection2>;
impl<'a, REG> Connection2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master port 2 connection to global slave is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection2::Disabled)
    }
    #[doc = "Master port 2 connection to global slave is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection2::Enabled)
    }
}
#[doc = "Global slave connection information for master port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connection3 {
    #[doc = "0: Master port 3 connection to global slave is disabled"]
    Disabled = 0,
    #[doc = "1: Master port 3 connection to global slave is enabled"]
    Enabled = 1,
}
impl From<Connection3> for bool {
    #[inline(always)]
    fn from(variant: Connection3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECTION_3` reader - Global slave connection information for master port"]
pub type Connection3R = crate::BitReader<Connection3>;
impl Connection3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connection3 {
        match self.bits {
            false => Connection3::Disabled,
            true => Connection3::Enabled,
        }
    }
    #[doc = "Master port 3 connection to global slave is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Connection3::Disabled
    }
    #[doc = "Master port 3 connection to global slave is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Connection3::Enabled
    }
}
#[doc = "Field `CONNECTION_3` writer - Global slave connection information for master port"]
pub type Connection3W<'a, REG> = crate::BitWriter<'a, REG, Connection3>;
impl<'a, REG> Connection3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master port 3 connection to global slave is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection3::Disabled)
    }
    #[doc = "Master port 3 connection to global slave is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection3::Enabled)
    }
}
#[doc = "Global slave connection information for master port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connection4 {
    #[doc = "0: Master port 4 connection to global slave is disabled"]
    Disabled = 0,
    #[doc = "1: Master port 4 connection to global slave is enabled"]
    Enabled = 1,
}
impl From<Connection4> for bool {
    #[inline(always)]
    fn from(variant: Connection4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECTION_4` reader - Global slave connection information for master port"]
pub type Connection4R = crate::BitReader<Connection4>;
impl Connection4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connection4 {
        match self.bits {
            false => Connection4::Disabled,
            true => Connection4::Enabled,
        }
    }
    #[doc = "Master port 4 connection to global slave is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Connection4::Disabled
    }
    #[doc = "Master port 4 connection to global slave is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Connection4::Enabled
    }
}
#[doc = "Field `CONNECTION_4` writer - Global slave connection information for master port"]
pub type Connection4W<'a, REG> = crate::BitWriter<'a, REG, Connection4>;
impl<'a, REG> Connection4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master port 4 connection to global slave is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection4::Disabled)
    }
    #[doc = "Master port 4 connection to global slave is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection4::Enabled)
    }
}
#[doc = "Global slave connection information for master port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connection5 {
    #[doc = "0: Master port 5 connection to global slave is disabled"]
    Disabled = 0,
    #[doc = "1: Master port 5 connection to global slave is enabled"]
    Enabled = 1,
}
impl From<Connection5> for bool {
    #[inline(always)]
    fn from(variant: Connection5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECTION_5` reader - Global slave connection information for master port"]
pub type Connection5R = crate::BitReader<Connection5>;
impl Connection5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connection5 {
        match self.bits {
            false => Connection5::Disabled,
            true => Connection5::Enabled,
        }
    }
    #[doc = "Master port 5 connection to global slave is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Connection5::Disabled
    }
    #[doc = "Master port 5 connection to global slave is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Connection5::Enabled
    }
}
#[doc = "Field `CONNECTION_5` writer - Global slave connection information for master port"]
pub type Connection5W<'a, REG> = crate::BitWriter<'a, REG, Connection5>;
impl<'a, REG> Connection5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master port 5 connection to global slave is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection5::Disabled)
    }
    #[doc = "Master port 5 connection to global slave is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection5::Enabled)
    }
}
#[doc = "Global slave connection information for master port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connection6 {
    #[doc = "0: Master port 6 connection to global slave is disabled"]
    Disabled = 0,
    #[doc = "1: Master port 6 connection to global slave is enabled"]
    Enabled = 1,
}
impl From<Connection6> for bool {
    #[inline(always)]
    fn from(variant: Connection6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECTION_6` reader - Global slave connection information for master port"]
pub type Connection6R = crate::BitReader<Connection6>;
impl Connection6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connection6 {
        match self.bits {
            false => Connection6::Disabled,
            true => Connection6::Enabled,
        }
    }
    #[doc = "Master port 6 connection to global slave is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Connection6::Disabled
    }
    #[doc = "Master port 6 connection to global slave is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Connection6::Enabled
    }
}
#[doc = "Field `CONNECTION_6` writer - Global slave connection information for master port"]
pub type Connection6W<'a, REG> = crate::BitWriter<'a, REG, Connection6>;
impl<'a, REG> Connection6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master port 6 connection to global slave is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection6::Disabled)
    }
    #[doc = "Master port 6 connection to global slave is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection6::Enabled)
    }
}
#[doc = "Global slave connection information for master port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connection7 {
    #[doc = "0: Master port 7 connection to global slave is disabled"]
    Disabled = 0,
    #[doc = "1: Master port 7 connection to global slave is enabled"]
    Enabled = 1,
}
impl From<Connection7> for bool {
    #[inline(always)]
    fn from(variant: Connection7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECTION_7` reader - Global slave connection information for master port"]
pub type Connection7R = crate::BitReader<Connection7>;
impl Connection7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connection7 {
        match self.bits {
            false => Connection7::Disabled,
            true => Connection7::Enabled,
        }
    }
    #[doc = "Master port 7 connection to global slave is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Connection7::Disabled
    }
    #[doc = "Master port 7 connection to global slave is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Connection7::Enabled
    }
}
#[doc = "Field `CONNECTION_7` writer - Global slave connection information for master port"]
pub type Connection7W<'a, REG> = crate::BitWriter<'a, REG, Connection7>;
impl<'a, REG> Connection7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master port 7 connection to global slave is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection7::Disabled)
    }
    #[doc = "Master port 7 connection to global slave is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection7::Enabled)
    }
}
#[doc = "Global slave connection information for master port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connection8 {
    #[doc = "0: Master port 8 connection to global slave is disabled"]
    Disabled = 0,
    #[doc = "1: Master port 8 connection to global slave is enabled"]
    Enabled = 1,
}
impl From<Connection8> for bool {
    #[inline(always)]
    fn from(variant: Connection8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECTION_8` reader - Global slave connection information for master port"]
pub type Connection8R = crate::BitReader<Connection8>;
impl Connection8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connection8 {
        match self.bits {
            false => Connection8::Disabled,
            true => Connection8::Enabled,
        }
    }
    #[doc = "Master port 8 connection to global slave is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Connection8::Disabled
    }
    #[doc = "Master port 8 connection to global slave is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Connection8::Enabled
    }
}
#[doc = "Field `CONNECTION_8` writer - Global slave connection information for master port"]
pub type Connection8W<'a, REG> = crate::BitWriter<'a, REG, Connection8>;
impl<'a, REG> Connection8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master port 8 connection to global slave is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection8::Disabled)
    }
    #[doc = "Master port 8 connection to global slave is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection8::Enabled)
    }
}
#[doc = "Global slave connection information for master port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connection9 {
    #[doc = "0: Master port 9 connection to global slave is disabled"]
    Disabled = 0,
    #[doc = "1: Master port 9 connection to global slave is enabled"]
    Enabled = 1,
}
impl From<Connection9> for bool {
    #[inline(always)]
    fn from(variant: Connection9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECTION_9` reader - Global slave connection information for master port"]
pub type Connection9R = crate::BitReader<Connection9>;
impl Connection9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connection9 {
        match self.bits {
            false => Connection9::Disabled,
            true => Connection9::Enabled,
        }
    }
    #[doc = "Master port 9 connection to global slave is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Connection9::Disabled
    }
    #[doc = "Master port 9 connection to global slave is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Connection9::Enabled
    }
}
#[doc = "Field `CONNECTION_9` writer - Global slave connection information for master port"]
pub type Connection9W<'a, REG> = crate::BitWriter<'a, REG, Connection9>;
impl<'a, REG> Connection9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master port 9 connection to global slave is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection9::Disabled)
    }
    #[doc = "Master port 9 connection to global slave is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection9::Enabled)
    }
}
#[doc = "Global slave connection information for master port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connection10 {
    #[doc = "0: Master port 10 connection to global slave is disabled"]
    Disabled = 0,
    #[doc = "1: Master port 10 connection to global slave is enabled"]
    Enabled = 1,
}
impl From<Connection10> for bool {
    #[inline(always)]
    fn from(variant: Connection10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECTION_10` reader - Global slave connection information for master port"]
pub type Connection10R = crate::BitReader<Connection10>;
impl Connection10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connection10 {
        match self.bits {
            false => Connection10::Disabled,
            true => Connection10::Enabled,
        }
    }
    #[doc = "Master port 10 connection to global slave is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Connection10::Disabled
    }
    #[doc = "Master port 10 connection to global slave is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Connection10::Enabled
    }
}
#[doc = "Field `CONNECTION_10` writer - Global slave connection information for master port"]
pub type Connection10W<'a, REG> = crate::BitWriter<'a, REG, Connection10>;
impl<'a, REG> Connection10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master port 10 connection to global slave is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection10::Disabled)
    }
    #[doc = "Master port 10 connection to global slave is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection10::Enabled)
    }
}
#[doc = "Global slave connection information for master port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connection11 {
    #[doc = "0: Master port 11 connection to global slave is disabled"]
    Disabled = 0,
    #[doc = "1: Master port 11 connection to global slave is enabled"]
    Enabled = 1,
}
impl From<Connection11> for bool {
    #[inline(always)]
    fn from(variant: Connection11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECTION_11` reader - Global slave connection information for master port"]
pub type Connection11R = crate::BitReader<Connection11>;
impl Connection11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connection11 {
        match self.bits {
            false => Connection11::Disabled,
            true => Connection11::Enabled,
        }
    }
    #[doc = "Master port 11 connection to global slave is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Connection11::Disabled
    }
    #[doc = "Master port 11 connection to global slave is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Connection11::Enabled
    }
}
#[doc = "Field `CONNECTION_11` writer - Global slave connection information for master port"]
pub type Connection11W<'a, REG> = crate::BitWriter<'a, REG, Connection11>;
impl<'a, REG> Connection11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master port 11 connection to global slave is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection11::Disabled)
    }
    #[doc = "Master port 11 connection to global slave is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection11::Enabled)
    }
}
#[doc = "Global slave connection information for master port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connection12 {
    #[doc = "0: Master port 12 connection to global slave is disabled"]
    Disabled = 0,
    #[doc = "1: Master port 12 connection to global slave is enabled"]
    Enabled = 1,
}
impl From<Connection12> for bool {
    #[inline(always)]
    fn from(variant: Connection12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECTION_12` reader - Global slave connection information for master port"]
pub type Connection12R = crate::BitReader<Connection12>;
impl Connection12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connection12 {
        match self.bits {
            false => Connection12::Disabled,
            true => Connection12::Enabled,
        }
    }
    #[doc = "Master port 12 connection to global slave is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Connection12::Disabled
    }
    #[doc = "Master port 12 connection to global slave is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Connection12::Enabled
    }
}
#[doc = "Field `CONNECTION_12` writer - Global slave connection information for master port"]
pub type Connection12W<'a, REG> = crate::BitWriter<'a, REG, Connection12>;
impl<'a, REG> Connection12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master port 12 connection to global slave is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection12::Disabled)
    }
    #[doc = "Master port 12 connection to global slave is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection12::Enabled)
    }
}
#[doc = "Global slave connection information for master port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connection13 {
    #[doc = "0: Master port 13 connection to global slave is disabled"]
    Disabled = 0,
    #[doc = "1: Master port 13 connection to global slave is enabled"]
    Enabled = 1,
}
impl From<Connection13> for bool {
    #[inline(always)]
    fn from(variant: Connection13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECTION_13` reader - Global slave connection information for master port"]
pub type Connection13R = crate::BitReader<Connection13>;
impl Connection13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connection13 {
        match self.bits {
            false => Connection13::Disabled,
            true => Connection13::Enabled,
        }
    }
    #[doc = "Master port 13 connection to global slave is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Connection13::Disabled
    }
    #[doc = "Master port 13 connection to global slave is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Connection13::Enabled
    }
}
#[doc = "Field `CONNECTION_13` writer - Global slave connection information for master port"]
pub type Connection13W<'a, REG> = crate::BitWriter<'a, REG, Connection13>;
impl<'a, REG> Connection13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master port 13 connection to global slave is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection13::Disabled)
    }
    #[doc = "Master port 13 connection to global slave is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection13::Enabled)
    }
}
#[doc = "Global slave connection information for master port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connection14 {
    #[doc = "0: Master port 14 connection to global slave is disabled"]
    Disabled = 0,
    #[doc = "1: Master port 14 connection to global slave is enabled"]
    Enabled = 1,
}
impl From<Connection14> for bool {
    #[inline(always)]
    fn from(variant: Connection14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECTION_14` reader - Global slave connection information for master port"]
pub type Connection14R = crate::BitReader<Connection14>;
impl Connection14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connection14 {
        match self.bits {
            false => Connection14::Disabled,
            true => Connection14::Enabled,
        }
    }
    #[doc = "Master port 14 connection to global slave is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Connection14::Disabled
    }
    #[doc = "Master port 14 connection to global slave is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Connection14::Enabled
    }
}
#[doc = "Field `CONNECTION_14` writer - Global slave connection information for master port"]
pub type Connection14W<'a, REG> = crate::BitWriter<'a, REG, Connection14>;
impl<'a, REG> Connection14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master port 14 connection to global slave is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection14::Disabled)
    }
    #[doc = "Master port 14 connection to global slave is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connection14::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_0(&self) -> Connection0R {
        Connection0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_1(&self) -> Connection1R {
        Connection1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_2(&self) -> Connection2R {
        Connection2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_3(&self) -> Connection3R {
        Connection3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_4(&self) -> Connection4R {
        Connection4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_5(&self) -> Connection5R {
        Connection5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_6(&self) -> Connection6R {
        Connection6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_7(&self) -> Connection7R {
        Connection7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_8(&self) -> Connection8R {
        Connection8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_9(&self) -> Connection9R {
        Connection9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_10(&self) -> Connection10R {
        Connection10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_11(&self) -> Connection11R {
        Connection11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_12(&self) -> Connection12R {
        Connection12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_13(&self) -> Connection13R {
        Connection13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_14(&self) -> Connection14R {
        Connection14R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_0(&mut self) -> Connection0W<MasterportSpec> {
        Connection0W::new(self, 0)
    }
    #[doc = "Bit 1 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_1(&mut self) -> Connection1W<MasterportSpec> {
        Connection1W::new(self, 1)
    }
    #[doc = "Bit 2 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_2(&mut self) -> Connection2W<MasterportSpec> {
        Connection2W::new(self, 2)
    }
    #[doc = "Bit 3 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_3(&mut self) -> Connection3W<MasterportSpec> {
        Connection3W::new(self, 3)
    }
    #[doc = "Bit 4 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_4(&mut self) -> Connection4W<MasterportSpec> {
        Connection4W::new(self, 4)
    }
    #[doc = "Bit 5 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_5(&mut self) -> Connection5W<MasterportSpec> {
        Connection5W::new(self, 5)
    }
    #[doc = "Bit 6 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_6(&mut self) -> Connection6W<MasterportSpec> {
        Connection6W::new(self, 6)
    }
    #[doc = "Bit 7 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_7(&mut self) -> Connection7W<MasterportSpec> {
        Connection7W::new(self, 7)
    }
    #[doc = "Bit 8 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_8(&mut self) -> Connection8W<MasterportSpec> {
        Connection8W::new(self, 8)
    }
    #[doc = "Bit 9 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_9(&mut self) -> Connection9W<MasterportSpec> {
        Connection9W::new(self, 9)
    }
    #[doc = "Bit 10 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_10(&mut self) -> Connection10W<MasterportSpec> {
        Connection10W::new(self, 10)
    }
    #[doc = "Bit 11 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_11(&mut self) -> Connection11W<MasterportSpec> {
        Connection11W::new(self, 11)
    }
    #[doc = "Bit 12 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_12(&mut self) -> Connection12W<MasterportSpec> {
        Connection12W::new(self, 12)
    }
    #[doc = "Bit 13 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_13(&mut self) -> Connection13W<MasterportSpec> {
        Connection13W::new(self, 13)
    }
    #[doc = "Bit 14 - Global slave connection information for master port"]
    #[inline(always)]
    pub fn connection_14(&mut self) -> Connection14W<MasterportSpec> {
        Connection14W::new(self, 14)
    }
}
#[doc = "Global slave connection information for master port\n\nYou can [`read`](crate::Reg::read) this register and get [`masterport::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`masterport::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MasterportSpec;
impl crate::RegisterSpec for MasterportSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`masterport::R`](R) reader structure"]
impl crate::Readable for MasterportSpec {}
#[doc = "`write(|w| ..)` method takes [`masterport::W`](W) writer structure"]
impl crate::Writable for MasterportSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASTERPORT to value 0"]
impl crate::Resettable for MasterportSpec {
    const RESET_VALUE: u32 = 0;
}
