#[doc = "Register `INTEN0` reader"]
pub type R = crate::R<Inten0Spec>;
#[doc = "Register `INTEN0` writer"]
pub type W = crate::W<Inten0Spec>;
#[doc = "Enable or disable interrupt for event COMPARE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare0 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Compare0> for bool {
    #[inline(always)]
    fn from(variant: Compare0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE0` reader - Enable or disable interrupt for event COMPARE\\[0\\]"]
pub type Compare0R = crate::BitReader<Compare0>;
impl Compare0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare0 {
        match self.bits {
            false => Compare0::Disabled,
            true => Compare0::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare0::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare0::Enabled
    }
}
#[doc = "Field `COMPARE0` writer - Enable or disable interrupt for event COMPARE\\[0\\]"]
pub type Compare0W<'a, REG> = crate::BitWriter<'a, REG, Compare0>;
impl<'a, REG> Compare0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare0::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare0::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event COMPARE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare1 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Compare1> for bool {
    #[inline(always)]
    fn from(variant: Compare1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE1` reader - Enable or disable interrupt for event COMPARE\\[1\\]"]
pub type Compare1R = crate::BitReader<Compare1>;
impl Compare1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare1 {
        match self.bits {
            false => Compare1::Disabled,
            true => Compare1::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare1::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare1::Enabled
    }
}
#[doc = "Field `COMPARE1` writer - Enable or disable interrupt for event COMPARE\\[1\\]"]
pub type Compare1W<'a, REG> = crate::BitWriter<'a, REG, Compare1>;
impl<'a, REG> Compare1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare1::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare1::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event COMPARE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare2 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Compare2> for bool {
    #[inline(always)]
    fn from(variant: Compare2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE2` reader - Enable or disable interrupt for event COMPARE\\[2\\]"]
pub type Compare2R = crate::BitReader<Compare2>;
impl Compare2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare2 {
        match self.bits {
            false => Compare2::Disabled,
            true => Compare2::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare2::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare2::Enabled
    }
}
#[doc = "Field `COMPARE2` writer - Enable or disable interrupt for event COMPARE\\[2\\]"]
pub type Compare2W<'a, REG> = crate::BitWriter<'a, REG, Compare2>;
impl<'a, REG> Compare2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare2::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare2::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event COMPARE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare3 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Compare3> for bool {
    #[inline(always)]
    fn from(variant: Compare3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE3` reader - Enable or disable interrupt for event COMPARE\\[3\\]"]
pub type Compare3R = crate::BitReader<Compare3>;
impl Compare3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare3 {
        match self.bits {
            false => Compare3::Disabled,
            true => Compare3::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare3::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare3::Enabled
    }
}
#[doc = "Field `COMPARE3` writer - Enable or disable interrupt for event COMPARE\\[3\\]"]
pub type Compare3W<'a, REG> = crate::BitWriter<'a, REG, Compare3>;
impl<'a, REG> Compare3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare3::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare3::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event COMPARE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare4 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Compare4> for bool {
    #[inline(always)]
    fn from(variant: Compare4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE4` reader - Enable or disable interrupt for event COMPARE\\[4\\]"]
pub type Compare4R = crate::BitReader<Compare4>;
impl Compare4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare4 {
        match self.bits {
            false => Compare4::Disabled,
            true => Compare4::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare4::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare4::Enabled
    }
}
#[doc = "Field `COMPARE4` writer - Enable or disable interrupt for event COMPARE\\[4\\]"]
pub type Compare4W<'a, REG> = crate::BitWriter<'a, REG, Compare4>;
impl<'a, REG> Compare4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare4::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare4::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event COMPARE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare5 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Compare5> for bool {
    #[inline(always)]
    fn from(variant: Compare5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE5` reader - Enable or disable interrupt for event COMPARE\\[5\\]"]
pub type Compare5R = crate::BitReader<Compare5>;
impl Compare5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare5 {
        match self.bits {
            false => Compare5::Disabled,
            true => Compare5::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare5::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare5::Enabled
    }
}
#[doc = "Field `COMPARE5` writer - Enable or disable interrupt for event COMPARE\\[5\\]"]
pub type Compare5W<'a, REG> = crate::BitWriter<'a, REG, Compare5>;
impl<'a, REG> Compare5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare5::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare5::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event COMPARE\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare6 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Compare6> for bool {
    #[inline(always)]
    fn from(variant: Compare6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE6` reader - Enable or disable interrupt for event COMPARE\\[6\\]"]
pub type Compare6R = crate::BitReader<Compare6>;
impl Compare6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare6 {
        match self.bits {
            false => Compare6::Disabled,
            true => Compare6::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare6::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare6::Enabled
    }
}
#[doc = "Field `COMPARE6` writer - Enable or disable interrupt for event COMPARE\\[6\\]"]
pub type Compare6W<'a, REG> = crate::BitWriter<'a, REG, Compare6>;
impl<'a, REG> Compare6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare6::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare6::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event COMPARE\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare7 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Compare7> for bool {
    #[inline(always)]
    fn from(variant: Compare7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE7` reader - Enable or disable interrupt for event COMPARE\\[7\\]"]
pub type Compare7R = crate::BitReader<Compare7>;
impl Compare7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare7 {
        match self.bits {
            false => Compare7::Disabled,
            true => Compare7::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare7::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare7::Enabled
    }
}
#[doc = "Field `COMPARE7` writer - Enable or disable interrupt for event COMPARE\\[7\\]"]
pub type Compare7W<'a, REG> = crate::BitWriter<'a, REG, Compare7>;
impl<'a, REG> Compare7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare7::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare7::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event COMPARE\\[8\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare8 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Compare8> for bool {
    #[inline(always)]
    fn from(variant: Compare8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE8` reader - Enable or disable interrupt for event COMPARE\\[8\\]"]
pub type Compare8R = crate::BitReader<Compare8>;
impl Compare8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare8 {
        match self.bits {
            false => Compare8::Disabled,
            true => Compare8::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare8::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare8::Enabled
    }
}
#[doc = "Field `COMPARE8` writer - Enable or disable interrupt for event COMPARE\\[8\\]"]
pub type Compare8W<'a, REG> = crate::BitWriter<'a, REG, Compare8>;
impl<'a, REG> Compare8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare8::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare8::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event COMPARE\\[9\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare9 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Compare9> for bool {
    #[inline(always)]
    fn from(variant: Compare9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE9` reader - Enable or disable interrupt for event COMPARE\\[9\\]"]
pub type Compare9R = crate::BitReader<Compare9>;
impl Compare9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare9 {
        match self.bits {
            false => Compare9::Disabled,
            true => Compare9::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare9::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare9::Enabled
    }
}
#[doc = "Field `COMPARE9` writer - Enable or disable interrupt for event COMPARE\\[9\\]"]
pub type Compare9W<'a, REG> = crate::BitWriter<'a, REG, Compare9>;
impl<'a, REG> Compare9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare9::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare9::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event COMPARE\\[10\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare10 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Compare10> for bool {
    #[inline(always)]
    fn from(variant: Compare10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE10` reader - Enable or disable interrupt for event COMPARE\\[10\\]"]
pub type Compare10R = crate::BitReader<Compare10>;
impl Compare10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare10 {
        match self.bits {
            false => Compare10::Disabled,
            true => Compare10::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare10::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare10::Enabled
    }
}
#[doc = "Field `COMPARE10` writer - Enable or disable interrupt for event COMPARE\\[10\\]"]
pub type Compare10W<'a, REG> = crate::BitWriter<'a, REG, Compare10>;
impl<'a, REG> Compare10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare10::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare10::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event COMPARE\\[11\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare11 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Compare11> for bool {
    #[inline(always)]
    fn from(variant: Compare11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE11` reader - Enable or disable interrupt for event COMPARE\\[11\\]"]
pub type Compare11R = crate::BitReader<Compare11>;
impl Compare11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare11 {
        match self.bits {
            false => Compare11::Disabled,
            true => Compare11::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare11::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare11::Enabled
    }
}
#[doc = "Field `COMPARE11` writer - Enable or disable interrupt for event COMPARE\\[11\\]"]
pub type Compare11W<'a, REG> = crate::BitWriter<'a, REG, Compare11>;
impl<'a, REG> Compare11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare11::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare11::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RTCOMPARESYNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcomparesync {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Rtcomparesync> for bool {
    #[inline(always)]
    fn from(variant: Rtcomparesync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCOMPARESYNC` reader - Enable or disable interrupt for event RTCOMPARESYNC"]
pub type RtcomparesyncR = crate::BitReader<Rtcomparesync>;
impl RtcomparesyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcomparesync {
        match self.bits {
            false => Rtcomparesync::Disabled,
            true => Rtcomparesync::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rtcomparesync::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rtcomparesync::Enabled
    }
}
#[doc = "Field `RTCOMPARESYNC` writer - Enable or disable interrupt for event RTCOMPARESYNC"]
pub type RtcomparesyncW<'a, REG> = crate::BitWriter<'a, REG, Rtcomparesync>;
impl<'a, REG> RtcomparesyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcomparesync::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcomparesync::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event SYSCOUNTERVALID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syscountervalid {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Syscountervalid> for bool {
    #[inline(always)]
    fn from(variant: Syscountervalid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCOUNTERVALID` reader - Enable or disable interrupt for event SYSCOUNTERVALID"]
pub type SyscountervalidR = crate::BitReader<Syscountervalid>;
impl SyscountervalidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syscountervalid {
        match self.bits {
            false => Syscountervalid::Disabled,
            true => Syscountervalid::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Syscountervalid::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Syscountervalid::Enabled
    }
}
#[doc = "Field `SYSCOUNTERVALID` writer - Enable or disable interrupt for event SYSCOUNTERVALID"]
pub type SyscountervalidW<'a, REG> = crate::BitWriter<'a, REG, Syscountervalid>;
impl<'a, REG> SyscountervalidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Syscountervalid::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Syscountervalid::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event PWMPERIODEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmperiodend {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Pwmperiodend> for bool {
    #[inline(always)]
    fn from(variant: Pwmperiodend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMPERIODEND` reader - Enable or disable interrupt for event PWMPERIODEND"]
pub type PwmperiodendR = crate::BitReader<Pwmperiodend>;
impl PwmperiodendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmperiodend {
        match self.bits {
            false => Pwmperiodend::Disabled,
            true => Pwmperiodend::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmperiodend::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmperiodend::Enabled
    }
}
#[doc = "Field `PWMPERIODEND` writer - Enable or disable interrupt for event PWMPERIODEND"]
pub type PwmperiodendW<'a, REG> = crate::BitWriter<'a, REG, Pwmperiodend>;
impl<'a, REG> PwmperiodendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmperiodend::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmperiodend::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event COMPARE\\[0\\]"]
    #[inline(always)]
    pub fn compare0(&self) -> Compare0R {
        Compare0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event COMPARE\\[1\\]"]
    #[inline(always)]
    pub fn compare1(&self) -> Compare1R {
        Compare1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event COMPARE\\[2\\]"]
    #[inline(always)]
    pub fn compare2(&self) -> Compare2R {
        Compare2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event COMPARE\\[3\\]"]
    #[inline(always)]
    pub fn compare3(&self) -> Compare3R {
        Compare3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event COMPARE\\[4\\]"]
    #[inline(always)]
    pub fn compare4(&self) -> Compare4R {
        Compare4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event COMPARE\\[5\\]"]
    #[inline(always)]
    pub fn compare5(&self) -> Compare5R {
        Compare5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event COMPARE\\[6\\]"]
    #[inline(always)]
    pub fn compare6(&self) -> Compare6R {
        Compare6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event COMPARE\\[7\\]"]
    #[inline(always)]
    pub fn compare7(&self) -> Compare7R {
        Compare7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event COMPARE\\[8\\]"]
    #[inline(always)]
    pub fn compare8(&self) -> Compare8R {
        Compare8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event COMPARE\\[9\\]"]
    #[inline(always)]
    pub fn compare9(&self) -> Compare9R {
        Compare9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event COMPARE\\[10\\]"]
    #[inline(always)]
    pub fn compare10(&self) -> Compare10R {
        Compare10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for event COMPARE\\[11\\]"]
    #[inline(always)]
    pub fn compare11(&self) -> Compare11R {
        Compare11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable or disable interrupt for event RTCOMPARESYNC"]
    #[inline(always)]
    pub fn rtcomparesync(&self) -> RtcomparesyncR {
        RtcomparesyncR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable or disable interrupt for event SYSCOUNTERVALID"]
    #[inline(always)]
    pub fn syscountervalid(&self) -> SyscountervalidR {
        SyscountervalidR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable or disable interrupt for event PWMPERIODEND"]
    #[inline(always)]
    pub fn pwmperiodend(&self) -> PwmperiodendR {
        PwmperiodendR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event COMPARE\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare0(&mut self) -> Compare0W<Inten0Spec> {
        Compare0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event COMPARE\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare1(&mut self) -> Compare1W<Inten0Spec> {
        Compare1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event COMPARE\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare2(&mut self) -> Compare2W<Inten0Spec> {
        Compare2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event COMPARE\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare3(&mut self) -> Compare3W<Inten0Spec> {
        Compare3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event COMPARE\\[4\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare4(&mut self) -> Compare4W<Inten0Spec> {
        Compare4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event COMPARE\\[5\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare5(&mut self) -> Compare5W<Inten0Spec> {
        Compare5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event COMPARE\\[6\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare6(&mut self) -> Compare6W<Inten0Spec> {
        Compare6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event COMPARE\\[7\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare7(&mut self) -> Compare7W<Inten0Spec> {
        Compare7W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event COMPARE\\[8\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare8(&mut self) -> Compare8W<Inten0Spec> {
        Compare8W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event COMPARE\\[9\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare9(&mut self) -> Compare9W<Inten0Spec> {
        Compare9W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event COMPARE\\[10\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare10(&mut self) -> Compare10W<Inten0Spec> {
        Compare10W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for event COMPARE\\[11\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare11(&mut self) -> Compare11W<Inten0Spec> {
        Compare11W::new(self, 11)
    }
    #[doc = "Bit 25 - Enable or disable interrupt for event RTCOMPARESYNC"]
    #[inline(always)]
    #[must_use]
    pub fn rtcomparesync(&mut self) -> RtcomparesyncW<Inten0Spec> {
        RtcomparesyncW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable or disable interrupt for event SYSCOUNTERVALID"]
    #[inline(always)]
    #[must_use]
    pub fn syscountervalid(&mut self) -> SyscountervalidW<Inten0Spec> {
        SyscountervalidW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable or disable interrupt for event PWMPERIODEND"]
    #[inline(always)]
    #[must_use]
    pub fn pwmperiodend(&mut self) -> PwmperiodendW<Inten0Spec> {
        PwmperiodendW::new(self, 27)
    }
}
#[doc = "Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Inten0Spec;
impl crate::RegisterSpec for Inten0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten0::R`](R) reader structure"]
impl crate::Readable for Inten0Spec {}
#[doc = "`write(|w| ..)` method takes [`inten0::W`](W) writer structure"]
impl crate::Writable for Inten0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN0 to value 0"]
impl crate::Resettable for Inten0Spec {
    const RESET_VALUE: u32 = 0;
}
