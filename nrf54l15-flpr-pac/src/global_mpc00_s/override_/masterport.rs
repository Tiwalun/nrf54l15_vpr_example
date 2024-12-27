#[doc = "Register `MASTERPORT` reader"]
pub type R = crate::R<MasterportSpec>;
#[doc = "Register `MASTERPORT` writer"]
pub type W = crate::W<MasterportSpec>;
#[doc = "Enable override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable0 {
    #[doc = "0: Override region n is disabled for master port 0"]
    Disable = 0,
    #[doc = "1: Override region n is enabled for master port 0"]
    Enable = 1,
}
impl From<Enable0> for bool {
    #[inline(always)]
    fn from(variant: Enable0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE0` reader - Enable override"]
pub type Enable0R = crate::BitReader<Enable0>;
impl Enable0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable0 {
        match self.bits {
            false => Enable0::Disable,
            true => Enable0::Enable,
        }
    }
    #[doc = "Override region n is disabled for master port 0"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable0::Disable
    }
    #[doc = "Override region n is enabled for master port 0"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable0::Enable
    }
}
#[doc = "Field `ENABLE0` writer - Enable override"]
pub type Enable0W<'a, REG> = crate::BitWriter<'a, REG, Enable0>;
impl<'a, REG> Enable0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override region n is disabled for master port 0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable0::Disable)
    }
    #[doc = "Override region n is enabled for master port 0"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable0::Enable)
    }
}
#[doc = "Enable override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable1 {
    #[doc = "0: Override region n is disabled for master port 1"]
    Disable = 0,
    #[doc = "1: Override region n is enabled for master port 1"]
    Enable = 1,
}
impl From<Enable1> for bool {
    #[inline(always)]
    fn from(variant: Enable1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE1` reader - Enable override"]
pub type Enable1R = crate::BitReader<Enable1>;
impl Enable1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable1 {
        match self.bits {
            false => Enable1::Disable,
            true => Enable1::Enable,
        }
    }
    #[doc = "Override region n is disabled for master port 1"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable1::Disable
    }
    #[doc = "Override region n is enabled for master port 1"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable1::Enable
    }
}
#[doc = "Field `ENABLE1` writer - Enable override"]
pub type Enable1W<'a, REG> = crate::BitWriter<'a, REG, Enable1>;
impl<'a, REG> Enable1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override region n is disabled for master port 1"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable1::Disable)
    }
    #[doc = "Override region n is enabled for master port 1"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable1::Enable)
    }
}
#[doc = "Enable override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable2 {
    #[doc = "0: Override region n is disabled for master port 2"]
    Disable = 0,
    #[doc = "1: Override region n is enabled for master port 2"]
    Enable = 1,
}
impl From<Enable2> for bool {
    #[inline(always)]
    fn from(variant: Enable2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE2` reader - Enable override"]
pub type Enable2R = crate::BitReader<Enable2>;
impl Enable2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable2 {
        match self.bits {
            false => Enable2::Disable,
            true => Enable2::Enable,
        }
    }
    #[doc = "Override region n is disabled for master port 2"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable2::Disable
    }
    #[doc = "Override region n is enabled for master port 2"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable2::Enable
    }
}
#[doc = "Field `ENABLE2` writer - Enable override"]
pub type Enable2W<'a, REG> = crate::BitWriter<'a, REG, Enable2>;
impl<'a, REG> Enable2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override region n is disabled for master port 2"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable2::Disable)
    }
    #[doc = "Override region n is enabled for master port 2"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable2::Enable)
    }
}
#[doc = "Enable override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable3 {
    #[doc = "0: Override region n is disabled for master port 3"]
    Disable = 0,
    #[doc = "1: Override region n is enabled for master port 3"]
    Enable = 1,
}
impl From<Enable3> for bool {
    #[inline(always)]
    fn from(variant: Enable3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE3` reader - Enable override"]
pub type Enable3R = crate::BitReader<Enable3>;
impl Enable3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable3 {
        match self.bits {
            false => Enable3::Disable,
            true => Enable3::Enable,
        }
    }
    #[doc = "Override region n is disabled for master port 3"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable3::Disable
    }
    #[doc = "Override region n is enabled for master port 3"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable3::Enable
    }
}
#[doc = "Field `ENABLE3` writer - Enable override"]
pub type Enable3W<'a, REG> = crate::BitWriter<'a, REG, Enable3>;
impl<'a, REG> Enable3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override region n is disabled for master port 3"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable3::Disable)
    }
    #[doc = "Override region n is enabled for master port 3"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable3::Enable)
    }
}
#[doc = "Enable override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable4 {
    #[doc = "0: Override region n is disabled for master port 4"]
    Disable = 0,
    #[doc = "1: Override region n is enabled for master port 4"]
    Enable = 1,
}
impl From<Enable4> for bool {
    #[inline(always)]
    fn from(variant: Enable4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE4` reader - Enable override"]
pub type Enable4R = crate::BitReader<Enable4>;
impl Enable4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable4 {
        match self.bits {
            false => Enable4::Disable,
            true => Enable4::Enable,
        }
    }
    #[doc = "Override region n is disabled for master port 4"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable4::Disable
    }
    #[doc = "Override region n is enabled for master port 4"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable4::Enable
    }
}
#[doc = "Field `ENABLE4` writer - Enable override"]
pub type Enable4W<'a, REG> = crate::BitWriter<'a, REG, Enable4>;
impl<'a, REG> Enable4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override region n is disabled for master port 4"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable4::Disable)
    }
    #[doc = "Override region n is enabled for master port 4"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable4::Enable)
    }
}
#[doc = "Enable override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable5 {
    #[doc = "0: Override region n is disabled for master port 5"]
    Disable = 0,
    #[doc = "1: Override region n is enabled for master port 5"]
    Enable = 1,
}
impl From<Enable5> for bool {
    #[inline(always)]
    fn from(variant: Enable5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE5` reader - Enable override"]
pub type Enable5R = crate::BitReader<Enable5>;
impl Enable5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable5 {
        match self.bits {
            false => Enable5::Disable,
            true => Enable5::Enable,
        }
    }
    #[doc = "Override region n is disabled for master port 5"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable5::Disable
    }
    #[doc = "Override region n is enabled for master port 5"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable5::Enable
    }
}
#[doc = "Field `ENABLE5` writer - Enable override"]
pub type Enable5W<'a, REG> = crate::BitWriter<'a, REG, Enable5>;
impl<'a, REG> Enable5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override region n is disabled for master port 5"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable5::Disable)
    }
    #[doc = "Override region n is enabled for master port 5"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable5::Enable)
    }
}
#[doc = "Enable override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable6 {
    #[doc = "0: Override region n is disabled for master port 6"]
    Disable = 0,
    #[doc = "1: Override region n is enabled for master port 6"]
    Enable = 1,
}
impl From<Enable6> for bool {
    #[inline(always)]
    fn from(variant: Enable6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE6` reader - Enable override"]
pub type Enable6R = crate::BitReader<Enable6>;
impl Enable6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable6 {
        match self.bits {
            false => Enable6::Disable,
            true => Enable6::Enable,
        }
    }
    #[doc = "Override region n is disabled for master port 6"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable6::Disable
    }
    #[doc = "Override region n is enabled for master port 6"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable6::Enable
    }
}
#[doc = "Field `ENABLE6` writer - Enable override"]
pub type Enable6W<'a, REG> = crate::BitWriter<'a, REG, Enable6>;
impl<'a, REG> Enable6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override region n is disabled for master port 6"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable6::Disable)
    }
    #[doc = "Override region n is enabled for master port 6"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable6::Enable)
    }
}
#[doc = "Enable override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable7 {
    #[doc = "0: Override region n is disabled for master port 7"]
    Disable = 0,
    #[doc = "1: Override region n is enabled for master port 7"]
    Enable = 1,
}
impl From<Enable7> for bool {
    #[inline(always)]
    fn from(variant: Enable7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE7` reader - Enable override"]
pub type Enable7R = crate::BitReader<Enable7>;
impl Enable7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable7 {
        match self.bits {
            false => Enable7::Disable,
            true => Enable7::Enable,
        }
    }
    #[doc = "Override region n is disabled for master port 7"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable7::Disable
    }
    #[doc = "Override region n is enabled for master port 7"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable7::Enable
    }
}
#[doc = "Field `ENABLE7` writer - Enable override"]
pub type Enable7W<'a, REG> = crate::BitWriter<'a, REG, Enable7>;
impl<'a, REG> Enable7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override region n is disabled for master port 7"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable7::Disable)
    }
    #[doc = "Override region n is enabled for master port 7"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable7::Enable)
    }
}
#[doc = "Enable override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable8 {
    #[doc = "0: Override region n is disabled for master port 8"]
    Disable = 0,
    #[doc = "1: Override region n is enabled for master port 8"]
    Enable = 1,
}
impl From<Enable8> for bool {
    #[inline(always)]
    fn from(variant: Enable8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE8` reader - Enable override"]
pub type Enable8R = crate::BitReader<Enable8>;
impl Enable8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable8 {
        match self.bits {
            false => Enable8::Disable,
            true => Enable8::Enable,
        }
    }
    #[doc = "Override region n is disabled for master port 8"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable8::Disable
    }
    #[doc = "Override region n is enabled for master port 8"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable8::Enable
    }
}
#[doc = "Field `ENABLE8` writer - Enable override"]
pub type Enable8W<'a, REG> = crate::BitWriter<'a, REG, Enable8>;
impl<'a, REG> Enable8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override region n is disabled for master port 8"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable8::Disable)
    }
    #[doc = "Override region n is enabled for master port 8"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable8::Enable)
    }
}
#[doc = "Enable override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable9 {
    #[doc = "0: Override region n is disabled for master port 9"]
    Disable = 0,
    #[doc = "1: Override region n is enabled for master port 9"]
    Enable = 1,
}
impl From<Enable9> for bool {
    #[inline(always)]
    fn from(variant: Enable9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE9` reader - Enable override"]
pub type Enable9R = crate::BitReader<Enable9>;
impl Enable9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable9 {
        match self.bits {
            false => Enable9::Disable,
            true => Enable9::Enable,
        }
    }
    #[doc = "Override region n is disabled for master port 9"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable9::Disable
    }
    #[doc = "Override region n is enabled for master port 9"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable9::Enable
    }
}
#[doc = "Field `ENABLE9` writer - Enable override"]
pub type Enable9W<'a, REG> = crate::BitWriter<'a, REG, Enable9>;
impl<'a, REG> Enable9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override region n is disabled for master port 9"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable9::Disable)
    }
    #[doc = "Override region n is enabled for master port 9"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable9::Enable)
    }
}
#[doc = "Enable override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable10 {
    #[doc = "0: Override region n is disabled for master port 10"]
    Disable = 0,
    #[doc = "1: Override region n is enabled for master port 10"]
    Enable = 1,
}
impl From<Enable10> for bool {
    #[inline(always)]
    fn from(variant: Enable10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE10` reader - Enable override"]
pub type Enable10R = crate::BitReader<Enable10>;
impl Enable10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable10 {
        match self.bits {
            false => Enable10::Disable,
            true => Enable10::Enable,
        }
    }
    #[doc = "Override region n is disabled for master port 10"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable10::Disable
    }
    #[doc = "Override region n is enabled for master port 10"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable10::Enable
    }
}
#[doc = "Field `ENABLE10` writer - Enable override"]
pub type Enable10W<'a, REG> = crate::BitWriter<'a, REG, Enable10>;
impl<'a, REG> Enable10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override region n is disabled for master port 10"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable10::Disable)
    }
    #[doc = "Override region n is enabled for master port 10"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable10::Enable)
    }
}
#[doc = "Enable override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable11 {
    #[doc = "0: Override region n is disabled for master port 11"]
    Disable = 0,
    #[doc = "1: Override region n is enabled for master port 11"]
    Enable = 1,
}
impl From<Enable11> for bool {
    #[inline(always)]
    fn from(variant: Enable11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE11` reader - Enable override"]
pub type Enable11R = crate::BitReader<Enable11>;
impl Enable11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable11 {
        match self.bits {
            false => Enable11::Disable,
            true => Enable11::Enable,
        }
    }
    #[doc = "Override region n is disabled for master port 11"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable11::Disable
    }
    #[doc = "Override region n is enabled for master port 11"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable11::Enable
    }
}
#[doc = "Field `ENABLE11` writer - Enable override"]
pub type Enable11W<'a, REG> = crate::BitWriter<'a, REG, Enable11>;
impl<'a, REG> Enable11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override region n is disabled for master port 11"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable11::Disable)
    }
    #[doc = "Override region n is enabled for master port 11"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable11::Enable)
    }
}
#[doc = "Enable override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable12 {
    #[doc = "0: Override region n is disabled for master port 12"]
    Disable = 0,
    #[doc = "1: Override region n is enabled for master port 12"]
    Enable = 1,
}
impl From<Enable12> for bool {
    #[inline(always)]
    fn from(variant: Enable12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE12` reader - Enable override"]
pub type Enable12R = crate::BitReader<Enable12>;
impl Enable12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable12 {
        match self.bits {
            false => Enable12::Disable,
            true => Enable12::Enable,
        }
    }
    #[doc = "Override region n is disabled for master port 12"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable12::Disable
    }
    #[doc = "Override region n is enabled for master port 12"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable12::Enable
    }
}
#[doc = "Field `ENABLE12` writer - Enable override"]
pub type Enable12W<'a, REG> = crate::BitWriter<'a, REG, Enable12>;
impl<'a, REG> Enable12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override region n is disabled for master port 12"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable12::Disable)
    }
    #[doc = "Override region n is enabled for master port 12"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable12::Enable)
    }
}
#[doc = "Enable override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable13 {
    #[doc = "0: Override region n is disabled for master port 13"]
    Disable = 0,
    #[doc = "1: Override region n is enabled for master port 13"]
    Enable = 1,
}
impl From<Enable13> for bool {
    #[inline(always)]
    fn from(variant: Enable13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE13` reader - Enable override"]
pub type Enable13R = crate::BitReader<Enable13>;
impl Enable13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable13 {
        match self.bits {
            false => Enable13::Disable,
            true => Enable13::Enable,
        }
    }
    #[doc = "Override region n is disabled for master port 13"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable13::Disable
    }
    #[doc = "Override region n is enabled for master port 13"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable13::Enable
    }
}
#[doc = "Field `ENABLE13` writer - Enable override"]
pub type Enable13W<'a, REG> = crate::BitWriter<'a, REG, Enable13>;
impl<'a, REG> Enable13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override region n is disabled for master port 13"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable13::Disable)
    }
    #[doc = "Override region n is enabled for master port 13"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable13::Enable)
    }
}
#[doc = "Enable override\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable14 {
    #[doc = "0: Override region n is disabled for master port 14"]
    Disable = 0,
    #[doc = "1: Override region n is enabled for master port 14"]
    Enable = 1,
}
impl From<Enable14> for bool {
    #[inline(always)]
    fn from(variant: Enable14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE14` reader - Enable override"]
pub type Enable14R = crate::BitReader<Enable14>;
impl Enable14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable14 {
        match self.bits {
            false => Enable14::Disable,
            true => Enable14::Enable,
        }
    }
    #[doc = "Override region n is disabled for master port 14"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable14::Disable
    }
    #[doc = "Override region n is enabled for master port 14"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable14::Enable
    }
}
#[doc = "Field `ENABLE14` writer - Enable override"]
pub type Enable14W<'a, REG> = crate::BitWriter<'a, REG, Enable14>;
impl<'a, REG> Enable14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override region n is disabled for master port 14"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable14::Disable)
    }
    #[doc = "Override region n is enabled for master port 14"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable14::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable override"]
    #[inline(always)]
    pub fn enable0(&self) -> Enable0R {
        Enable0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable override"]
    #[inline(always)]
    pub fn enable1(&self) -> Enable1R {
        Enable1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable override"]
    #[inline(always)]
    pub fn enable2(&self) -> Enable2R {
        Enable2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable override"]
    #[inline(always)]
    pub fn enable3(&self) -> Enable3R {
        Enable3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable override"]
    #[inline(always)]
    pub fn enable4(&self) -> Enable4R {
        Enable4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable override"]
    #[inline(always)]
    pub fn enable5(&self) -> Enable5R {
        Enable5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable override"]
    #[inline(always)]
    pub fn enable6(&self) -> Enable6R {
        Enable6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable override"]
    #[inline(always)]
    pub fn enable7(&self) -> Enable7R {
        Enable7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable override"]
    #[inline(always)]
    pub fn enable8(&self) -> Enable8R {
        Enable8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable override"]
    #[inline(always)]
    pub fn enable9(&self) -> Enable9R {
        Enable9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable override"]
    #[inline(always)]
    pub fn enable10(&self) -> Enable10R {
        Enable10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable override"]
    #[inline(always)]
    pub fn enable11(&self) -> Enable11R {
        Enable11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable override"]
    #[inline(always)]
    pub fn enable12(&self) -> Enable12R {
        Enable12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable override"]
    #[inline(always)]
    pub fn enable13(&self) -> Enable13R {
        Enable13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable override"]
    #[inline(always)]
    pub fn enable14(&self) -> Enable14R {
        Enable14R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable override"]
    #[inline(always)]
    pub fn enable0(&mut self) -> Enable0W<MasterportSpec> {
        Enable0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable override"]
    #[inline(always)]
    pub fn enable1(&mut self) -> Enable1W<MasterportSpec> {
        Enable1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable override"]
    #[inline(always)]
    pub fn enable2(&mut self) -> Enable2W<MasterportSpec> {
        Enable2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable override"]
    #[inline(always)]
    pub fn enable3(&mut self) -> Enable3W<MasterportSpec> {
        Enable3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable override"]
    #[inline(always)]
    pub fn enable4(&mut self) -> Enable4W<MasterportSpec> {
        Enable4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable override"]
    #[inline(always)]
    pub fn enable5(&mut self) -> Enable5W<MasterportSpec> {
        Enable5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable override"]
    #[inline(always)]
    pub fn enable6(&mut self) -> Enable6W<MasterportSpec> {
        Enable6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable override"]
    #[inline(always)]
    pub fn enable7(&mut self) -> Enable7W<MasterportSpec> {
        Enable7W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable override"]
    #[inline(always)]
    pub fn enable8(&mut self) -> Enable8W<MasterportSpec> {
        Enable8W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable override"]
    #[inline(always)]
    pub fn enable9(&mut self) -> Enable9W<MasterportSpec> {
        Enable9W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable override"]
    #[inline(always)]
    pub fn enable10(&mut self) -> Enable10W<MasterportSpec> {
        Enable10W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable override"]
    #[inline(always)]
    pub fn enable11(&mut self) -> Enable11W<MasterportSpec> {
        Enable11W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable override"]
    #[inline(always)]
    pub fn enable12(&mut self) -> Enable12W<MasterportSpec> {
        Enable12W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable override"]
    #[inline(always)]
    pub fn enable13(&mut self) -> Enable13W<MasterportSpec> {
        Enable13W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable override"]
    #[inline(always)]
    pub fn enable14(&mut self) -> Enable14W<MasterportSpec> {
        Enable14W::new(self, 14)
    }
}
#[doc = "Description cluster: Override region n local master enable\n\nYou can [`read`](crate::Reg::read) this register and get [`masterport::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`masterport::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
