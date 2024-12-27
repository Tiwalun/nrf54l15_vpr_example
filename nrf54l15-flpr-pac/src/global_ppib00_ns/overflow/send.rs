#[doc = "Register `SEND` reader"]
pub type R = crate::R<SendSpec>;
#[doc = "Register `SEND` writer"]
pub type W = crate::W<SendSpec>;
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send0 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send0> for bool {
    #[inline(always)]
    fn from(variant: Send0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_0` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[0\\]."]
pub type Send0R = crate::BitReader<Send0>;
impl Send0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send0 {
        match self.bits {
            true => Send0::Overflow,
            false => Send0::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send0::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send0::NoOverflow
    }
}
#[doc = "Field `SEND_0` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[0\\]."]
pub type Send0W<'a, REG> = crate::BitWriter<'a, REG, Send0>;
impl<'a, REG> Send0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send0::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send0::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send1 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send1> for bool {
    #[inline(always)]
    fn from(variant: Send1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_1` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[1\\]."]
pub type Send1R = crate::BitReader<Send1>;
impl Send1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send1 {
        match self.bits {
            true => Send1::Overflow,
            false => Send1::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send1::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send1::NoOverflow
    }
}
#[doc = "Field `SEND_1` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[1\\]."]
pub type Send1W<'a, REG> = crate::BitWriter<'a, REG, Send1>;
impl<'a, REG> Send1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send1::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send1::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[2\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send2 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send2> for bool {
    #[inline(always)]
    fn from(variant: Send2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_2` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[2\\]."]
pub type Send2R = crate::BitReader<Send2>;
impl Send2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send2 {
        match self.bits {
            true => Send2::Overflow,
            false => Send2::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send2::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send2::NoOverflow
    }
}
#[doc = "Field `SEND_2` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[2\\]."]
pub type Send2W<'a, REG> = crate::BitWriter<'a, REG, Send2>;
impl<'a, REG> Send2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send2::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send2::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[3\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send3 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send3> for bool {
    #[inline(always)]
    fn from(variant: Send3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_3` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[3\\]."]
pub type Send3R = crate::BitReader<Send3>;
impl Send3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send3 {
        match self.bits {
            true => Send3::Overflow,
            false => Send3::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send3::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send3::NoOverflow
    }
}
#[doc = "Field `SEND_3` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[3\\]."]
pub type Send3W<'a, REG> = crate::BitWriter<'a, REG, Send3>;
impl<'a, REG> Send3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send3::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send3::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[4\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send4 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send4> for bool {
    #[inline(always)]
    fn from(variant: Send4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_4` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[4\\]."]
pub type Send4R = crate::BitReader<Send4>;
impl Send4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send4 {
        match self.bits {
            true => Send4::Overflow,
            false => Send4::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send4::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send4::NoOverflow
    }
}
#[doc = "Field `SEND_4` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[4\\]."]
pub type Send4W<'a, REG> = crate::BitWriter<'a, REG, Send4>;
impl<'a, REG> Send4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send4::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send4::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[5\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send5 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send5> for bool {
    #[inline(always)]
    fn from(variant: Send5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_5` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[5\\]."]
pub type Send5R = crate::BitReader<Send5>;
impl Send5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send5 {
        match self.bits {
            true => Send5::Overflow,
            false => Send5::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send5::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send5::NoOverflow
    }
}
#[doc = "Field `SEND_5` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[5\\]."]
pub type Send5W<'a, REG> = crate::BitWriter<'a, REG, Send5>;
impl<'a, REG> Send5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send5::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send5::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[6\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send6 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send6> for bool {
    #[inline(always)]
    fn from(variant: Send6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_6` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[6\\]."]
pub type Send6R = crate::BitReader<Send6>;
impl Send6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send6 {
        match self.bits {
            true => Send6::Overflow,
            false => Send6::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send6::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send6::NoOverflow
    }
}
#[doc = "Field `SEND_6` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[6\\]."]
pub type Send6W<'a, REG> = crate::BitWriter<'a, REG, Send6>;
impl<'a, REG> Send6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send6::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send6::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[7\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send7 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send7> for bool {
    #[inline(always)]
    fn from(variant: Send7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_7` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[7\\]."]
pub type Send7R = crate::BitReader<Send7>;
impl Send7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send7 {
        match self.bits {
            true => Send7::Overflow,
            false => Send7::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send7::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send7::NoOverflow
    }
}
#[doc = "Field `SEND_7` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[7\\]."]
pub type Send7W<'a, REG> = crate::BitWriter<'a, REG, Send7>;
impl<'a, REG> Send7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send7::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send7::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[8\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send8 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send8> for bool {
    #[inline(always)]
    fn from(variant: Send8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_8` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[8\\]."]
pub type Send8R = crate::BitReader<Send8>;
impl Send8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send8 {
        match self.bits {
            true => Send8::Overflow,
            false => Send8::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send8::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send8::NoOverflow
    }
}
#[doc = "Field `SEND_8` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[8\\]."]
pub type Send8W<'a, REG> = crate::BitWriter<'a, REG, Send8>;
impl<'a, REG> Send8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send8::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send8::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[9\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send9 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send9> for bool {
    #[inline(always)]
    fn from(variant: Send9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_9` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[9\\]."]
pub type Send9R = crate::BitReader<Send9>;
impl Send9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send9 {
        match self.bits {
            true => Send9::Overflow,
            false => Send9::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send9::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send9::NoOverflow
    }
}
#[doc = "Field `SEND_9` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[9\\]."]
pub type Send9W<'a, REG> = crate::BitWriter<'a, REG, Send9>;
impl<'a, REG> Send9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send9::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send9::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[10\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send10 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send10> for bool {
    #[inline(always)]
    fn from(variant: Send10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_10` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[10\\]."]
pub type Send10R = crate::BitReader<Send10>;
impl Send10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send10 {
        match self.bits {
            true => Send10::Overflow,
            false => Send10::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send10::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send10::NoOverflow
    }
}
#[doc = "Field `SEND_10` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[10\\]."]
pub type Send10W<'a, REG> = crate::BitWriter<'a, REG, Send10>;
impl<'a, REG> Send10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send10::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send10::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[11\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send11 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send11> for bool {
    #[inline(always)]
    fn from(variant: Send11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_11` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[11\\]."]
pub type Send11R = crate::BitReader<Send11>;
impl Send11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send11 {
        match self.bits {
            true => Send11::Overflow,
            false => Send11::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send11::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send11::NoOverflow
    }
}
#[doc = "Field `SEND_11` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[11\\]."]
pub type Send11W<'a, REG> = crate::BitWriter<'a, REG, Send11>;
impl<'a, REG> Send11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send11::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send11::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[12\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send12 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send12> for bool {
    #[inline(always)]
    fn from(variant: Send12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_12` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[12\\]."]
pub type Send12R = crate::BitReader<Send12>;
impl Send12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send12 {
        match self.bits {
            true => Send12::Overflow,
            false => Send12::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send12::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send12::NoOverflow
    }
}
#[doc = "Field `SEND_12` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[12\\]."]
pub type Send12W<'a, REG> = crate::BitWriter<'a, REG, Send12>;
impl<'a, REG> Send12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send12::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send12::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[13\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send13 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send13> for bool {
    #[inline(always)]
    fn from(variant: Send13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_13` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[13\\]."]
pub type Send13R = crate::BitReader<Send13>;
impl Send13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send13 {
        match self.bits {
            true => Send13::Overflow,
            false => Send13::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send13::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send13::NoOverflow
    }
}
#[doc = "Field `SEND_13` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[13\\]."]
pub type Send13W<'a, REG> = crate::BitWriter<'a, REG, Send13>;
impl<'a, REG> Send13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send13::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send13::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[14\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send14 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send14> for bool {
    #[inline(always)]
    fn from(variant: Send14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_14` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[14\\]."]
pub type Send14R = crate::BitReader<Send14>;
impl Send14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send14 {
        match self.bits {
            true => Send14::Overflow,
            false => Send14::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send14::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send14::NoOverflow
    }
}
#[doc = "Field `SEND_14` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[14\\]."]
pub type Send14W<'a, REG> = crate::BitWriter<'a, REG, Send14>;
impl<'a, REG> Send14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send14::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send14::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[15\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send15 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send15> for bool {
    #[inline(always)]
    fn from(variant: Send15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_15` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[15\\]."]
pub type Send15R = crate::BitReader<Send15>;
impl Send15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send15 {
        match self.bits {
            true => Send15::Overflow,
            false => Send15::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send15::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send15::NoOverflow
    }
}
#[doc = "Field `SEND_15` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[15\\]."]
pub type Send15W<'a, REG> = crate::BitWriter<'a, REG, Send15>;
impl<'a, REG> Send15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send15::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send15::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[16\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send16 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send16> for bool {
    #[inline(always)]
    fn from(variant: Send16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_16` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[16\\]."]
pub type Send16R = crate::BitReader<Send16>;
impl Send16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send16 {
        match self.bits {
            true => Send16::Overflow,
            false => Send16::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send16::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send16::NoOverflow
    }
}
#[doc = "Field `SEND_16` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[16\\]."]
pub type Send16W<'a, REG> = crate::BitWriter<'a, REG, Send16>;
impl<'a, REG> Send16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send16::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send16::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[17\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send17 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send17> for bool {
    #[inline(always)]
    fn from(variant: Send17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_17` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[17\\]."]
pub type Send17R = crate::BitReader<Send17>;
impl Send17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send17 {
        match self.bits {
            true => Send17::Overflow,
            false => Send17::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send17::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send17::NoOverflow
    }
}
#[doc = "Field `SEND_17` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[17\\]."]
pub type Send17W<'a, REG> = crate::BitWriter<'a, REG, Send17>;
impl<'a, REG> Send17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send17::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send17::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[18\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send18 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send18> for bool {
    #[inline(always)]
    fn from(variant: Send18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_18` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[18\\]."]
pub type Send18R = crate::BitReader<Send18>;
impl Send18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send18 {
        match self.bits {
            true => Send18::Overflow,
            false => Send18::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send18::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send18::NoOverflow
    }
}
#[doc = "Field `SEND_18` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[18\\]."]
pub type Send18W<'a, REG> = crate::BitWriter<'a, REG, Send18>;
impl<'a, REG> Send18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send18::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send18::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[19\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send19 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send19> for bool {
    #[inline(always)]
    fn from(variant: Send19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_19` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[19\\]."]
pub type Send19R = crate::BitReader<Send19>;
impl Send19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send19 {
        match self.bits {
            true => Send19::Overflow,
            false => Send19::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send19::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send19::NoOverflow
    }
}
#[doc = "Field `SEND_19` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[19\\]."]
pub type Send19W<'a, REG> = crate::BitWriter<'a, REG, Send19>;
impl<'a, REG> Send19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send19::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send19::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[20\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send20 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send20> for bool {
    #[inline(always)]
    fn from(variant: Send20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_20` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[20\\]."]
pub type Send20R = crate::BitReader<Send20>;
impl Send20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send20 {
        match self.bits {
            true => Send20::Overflow,
            false => Send20::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send20::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send20::NoOverflow
    }
}
#[doc = "Field `SEND_20` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[20\\]."]
pub type Send20W<'a, REG> = crate::BitWriter<'a, REG, Send20>;
impl<'a, REG> Send20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send20::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send20::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[21\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send21 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send21> for bool {
    #[inline(always)]
    fn from(variant: Send21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_21` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[21\\]."]
pub type Send21R = crate::BitReader<Send21>;
impl Send21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send21 {
        match self.bits {
            true => Send21::Overflow,
            false => Send21::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send21::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send21::NoOverflow
    }
}
#[doc = "Field `SEND_21` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[21\\]."]
pub type Send21W<'a, REG> = crate::BitWriter<'a, REG, Send21>;
impl<'a, REG> Send21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send21::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send21::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[22\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send22 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send22> for bool {
    #[inline(always)]
    fn from(variant: Send22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_22` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[22\\]."]
pub type Send22R = crate::BitReader<Send22>;
impl Send22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send22 {
        match self.bits {
            true => Send22::Overflow,
            false => Send22::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send22::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send22::NoOverflow
    }
}
#[doc = "Field `SEND_22` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[22\\]."]
pub type Send22W<'a, REG> = crate::BitWriter<'a, REG, Send22>;
impl<'a, REG> Send22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send22::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send22::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[23\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send23 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send23> for bool {
    #[inline(always)]
    fn from(variant: Send23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_23` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[23\\]."]
pub type Send23R = crate::BitReader<Send23>;
impl Send23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send23 {
        match self.bits {
            true => Send23::Overflow,
            false => Send23::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send23::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send23::NoOverflow
    }
}
#[doc = "Field `SEND_23` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[23\\]."]
pub type Send23W<'a, REG> = crate::BitWriter<'a, REG, Send23>;
impl<'a, REG> Send23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send23::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send23::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[24\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send24 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send24> for bool {
    #[inline(always)]
    fn from(variant: Send24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_24` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[24\\]."]
pub type Send24R = crate::BitReader<Send24>;
impl Send24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send24 {
        match self.bits {
            true => Send24::Overflow,
            false => Send24::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send24::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send24::NoOverflow
    }
}
#[doc = "Field `SEND_24` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[24\\]."]
pub type Send24W<'a, REG> = crate::BitWriter<'a, REG, Send24>;
impl<'a, REG> Send24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send24::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send24::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[25\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send25 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send25> for bool {
    #[inline(always)]
    fn from(variant: Send25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_25` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[25\\]."]
pub type Send25R = crate::BitReader<Send25>;
impl Send25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send25 {
        match self.bits {
            true => Send25::Overflow,
            false => Send25::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send25::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send25::NoOverflow
    }
}
#[doc = "Field `SEND_25` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[25\\]."]
pub type Send25W<'a, REG> = crate::BitWriter<'a, REG, Send25>;
impl<'a, REG> Send25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send25::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send25::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[26\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send26 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send26> for bool {
    #[inline(always)]
    fn from(variant: Send26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_26` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[26\\]."]
pub type Send26R = crate::BitReader<Send26>;
impl Send26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send26 {
        match self.bits {
            true => Send26::Overflow,
            false => Send26::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send26::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send26::NoOverflow
    }
}
#[doc = "Field `SEND_26` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[26\\]."]
pub type Send26W<'a, REG> = crate::BitWriter<'a, REG, Send26>;
impl<'a, REG> Send26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send26::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send26::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[27\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send27 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send27> for bool {
    #[inline(always)]
    fn from(variant: Send27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_27` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[27\\]."]
pub type Send27R = crate::BitReader<Send27>;
impl Send27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send27 {
        match self.bits {
            true => Send27::Overflow,
            false => Send27::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send27::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send27::NoOverflow
    }
}
#[doc = "Field `SEND_27` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[27\\]."]
pub type Send27W<'a, REG> = crate::BitWriter<'a, REG, Send27>;
impl<'a, REG> Send27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send27::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send27::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[28\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send28 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send28> for bool {
    #[inline(always)]
    fn from(variant: Send28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_28` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[28\\]."]
pub type Send28R = crate::BitReader<Send28>;
impl Send28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send28 {
        match self.bits {
            true => Send28::Overflow,
            false => Send28::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send28::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send28::NoOverflow
    }
}
#[doc = "Field `SEND_28` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[28\\]."]
pub type Send28W<'a, REG> = crate::BitWriter<'a, REG, Send28>;
impl<'a, REG> Send28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send28::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send28::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[29\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send29 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send29> for bool {
    #[inline(always)]
    fn from(variant: Send29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_29` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[29\\]."]
pub type Send29R = crate::BitReader<Send29>;
impl Send29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send29 {
        match self.bits {
            true => Send29::Overflow,
            false => Send29::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send29::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send29::NoOverflow
    }
}
#[doc = "Field `SEND_29` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[29\\]."]
pub type Send29W<'a, REG> = crate::BitWriter<'a, REG, Send29>;
impl<'a, REG> Send29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send29::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send29::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[30\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send30 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send30> for bool {
    #[inline(always)]
    fn from(variant: Send30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_30` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[30\\]."]
pub type Send30R = crate::BitReader<Send30>;
impl Send30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send30 {
        match self.bits {
            true => Send30::Overflow,
            false => Send30::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send30::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send30::NoOverflow
    }
}
#[doc = "Field `SEND_30` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[30\\]."]
pub type Send30W<'a, REG> = crate::BitWriter<'a, REG, Send30>;
impl<'a, REG> Send30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send30::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send30::NoOverflow)
    }
}
#[doc = "The status for tasks overflow at SUBSCRIBE_SEND\\[31\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Send31 {
    #[doc = "1: Task overflow is happened."]
    Overflow = 1,
    #[doc = "0: Task overflow is not happened."]
    NoOverflow = 0,
}
impl From<Send31> for bool {
    #[inline(always)]
    fn from(variant: Send31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_31` reader - The status for tasks overflow at SUBSCRIBE_SEND\\[31\\]."]
pub type Send31R = crate::BitReader<Send31>;
impl Send31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Send31 {
        match self.bits {
            true => Send31::Overflow,
            false => Send31::NoOverflow,
        }
    }
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Send31::Overflow
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Send31::NoOverflow
    }
}
#[doc = "Field `SEND_31` writer - The status for tasks overflow at SUBSCRIBE_SEND\\[31\\]."]
pub type Send31W<'a, REG> = crate::BitWriter<'a, REG, Send31>;
impl<'a, REG> Send31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task overflow is happened."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send31::Overflow)
    }
    #[doc = "Task overflow is not happened."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Send31::NoOverflow)
    }
}
impl R {
    #[doc = "Bit 0 - The status for tasks overflow at SUBSCRIBE_SEND\\[0\\]."]
    #[inline(always)]
    pub fn send_0(&self) -> Send0R {
        Send0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The status for tasks overflow at SUBSCRIBE_SEND\\[1\\]."]
    #[inline(always)]
    pub fn send_1(&self) -> Send1R {
        Send1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The status for tasks overflow at SUBSCRIBE_SEND\\[2\\]."]
    #[inline(always)]
    pub fn send_2(&self) -> Send2R {
        Send2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The status for tasks overflow at SUBSCRIBE_SEND\\[3\\]."]
    #[inline(always)]
    pub fn send_3(&self) -> Send3R {
        Send3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The status for tasks overflow at SUBSCRIBE_SEND\\[4\\]."]
    #[inline(always)]
    pub fn send_4(&self) -> Send4R {
        Send4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The status for tasks overflow at SUBSCRIBE_SEND\\[5\\]."]
    #[inline(always)]
    pub fn send_5(&self) -> Send5R {
        Send5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The status for tasks overflow at SUBSCRIBE_SEND\\[6\\]."]
    #[inline(always)]
    pub fn send_6(&self) -> Send6R {
        Send6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The status for tasks overflow at SUBSCRIBE_SEND\\[7\\]."]
    #[inline(always)]
    pub fn send_7(&self) -> Send7R {
        Send7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The status for tasks overflow at SUBSCRIBE_SEND\\[8\\]."]
    #[inline(always)]
    pub fn send_8(&self) -> Send8R {
        Send8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The status for tasks overflow at SUBSCRIBE_SEND\\[9\\]."]
    #[inline(always)]
    pub fn send_9(&self) -> Send9R {
        Send9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The status for tasks overflow at SUBSCRIBE_SEND\\[10\\]."]
    #[inline(always)]
    pub fn send_10(&self) -> Send10R {
        Send10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The status for tasks overflow at SUBSCRIBE_SEND\\[11\\]."]
    #[inline(always)]
    pub fn send_11(&self) -> Send11R {
        Send11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The status for tasks overflow at SUBSCRIBE_SEND\\[12\\]."]
    #[inline(always)]
    pub fn send_12(&self) -> Send12R {
        Send12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The status for tasks overflow at SUBSCRIBE_SEND\\[13\\]."]
    #[inline(always)]
    pub fn send_13(&self) -> Send13R {
        Send13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The status for tasks overflow at SUBSCRIBE_SEND\\[14\\]."]
    #[inline(always)]
    pub fn send_14(&self) -> Send14R {
        Send14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The status for tasks overflow at SUBSCRIBE_SEND\\[15\\]."]
    #[inline(always)]
    pub fn send_15(&self) -> Send15R {
        Send15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The status for tasks overflow at SUBSCRIBE_SEND\\[16\\]."]
    #[inline(always)]
    pub fn send_16(&self) -> Send16R {
        Send16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The status for tasks overflow at SUBSCRIBE_SEND\\[17\\]."]
    #[inline(always)]
    pub fn send_17(&self) -> Send17R {
        Send17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The status for tasks overflow at SUBSCRIBE_SEND\\[18\\]."]
    #[inline(always)]
    pub fn send_18(&self) -> Send18R {
        Send18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The status for tasks overflow at SUBSCRIBE_SEND\\[19\\]."]
    #[inline(always)]
    pub fn send_19(&self) -> Send19R {
        Send19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The status for tasks overflow at SUBSCRIBE_SEND\\[20\\]."]
    #[inline(always)]
    pub fn send_20(&self) -> Send20R {
        Send20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The status for tasks overflow at SUBSCRIBE_SEND\\[21\\]."]
    #[inline(always)]
    pub fn send_21(&self) -> Send21R {
        Send21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The status for tasks overflow at SUBSCRIBE_SEND\\[22\\]."]
    #[inline(always)]
    pub fn send_22(&self) -> Send22R {
        Send22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The status for tasks overflow at SUBSCRIBE_SEND\\[23\\]."]
    #[inline(always)]
    pub fn send_23(&self) -> Send23R {
        Send23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The status for tasks overflow at SUBSCRIBE_SEND\\[24\\]."]
    #[inline(always)]
    pub fn send_24(&self) -> Send24R {
        Send24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The status for tasks overflow at SUBSCRIBE_SEND\\[25\\]."]
    #[inline(always)]
    pub fn send_25(&self) -> Send25R {
        Send25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The status for tasks overflow at SUBSCRIBE_SEND\\[26\\]."]
    #[inline(always)]
    pub fn send_26(&self) -> Send26R {
        Send26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The status for tasks overflow at SUBSCRIBE_SEND\\[27\\]."]
    #[inline(always)]
    pub fn send_27(&self) -> Send27R {
        Send27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The status for tasks overflow at SUBSCRIBE_SEND\\[28\\]."]
    #[inline(always)]
    pub fn send_28(&self) -> Send28R {
        Send28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The status for tasks overflow at SUBSCRIBE_SEND\\[29\\]."]
    #[inline(always)]
    pub fn send_29(&self) -> Send29R {
        Send29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - The status for tasks overflow at SUBSCRIBE_SEND\\[30\\]."]
    #[inline(always)]
    pub fn send_30(&self) -> Send30R {
        Send30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The status for tasks overflow at SUBSCRIBE_SEND\\[31\\]."]
    #[inline(always)]
    pub fn send_31(&self) -> Send31R {
        Send31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The status for tasks overflow at SUBSCRIBE_SEND\\[0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_0(&mut self) -> Send0W<SendSpec> {
        Send0W::new(self, 0)
    }
    #[doc = "Bit 1 - The status for tasks overflow at SUBSCRIBE_SEND\\[1\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_1(&mut self) -> Send1W<SendSpec> {
        Send1W::new(self, 1)
    }
    #[doc = "Bit 2 - The status for tasks overflow at SUBSCRIBE_SEND\\[2\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_2(&mut self) -> Send2W<SendSpec> {
        Send2W::new(self, 2)
    }
    #[doc = "Bit 3 - The status for tasks overflow at SUBSCRIBE_SEND\\[3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_3(&mut self) -> Send3W<SendSpec> {
        Send3W::new(self, 3)
    }
    #[doc = "Bit 4 - The status for tasks overflow at SUBSCRIBE_SEND\\[4\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_4(&mut self) -> Send4W<SendSpec> {
        Send4W::new(self, 4)
    }
    #[doc = "Bit 5 - The status for tasks overflow at SUBSCRIBE_SEND\\[5\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_5(&mut self) -> Send5W<SendSpec> {
        Send5W::new(self, 5)
    }
    #[doc = "Bit 6 - The status for tasks overflow at SUBSCRIBE_SEND\\[6\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_6(&mut self) -> Send6W<SendSpec> {
        Send6W::new(self, 6)
    }
    #[doc = "Bit 7 - The status for tasks overflow at SUBSCRIBE_SEND\\[7\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_7(&mut self) -> Send7W<SendSpec> {
        Send7W::new(self, 7)
    }
    #[doc = "Bit 8 - The status for tasks overflow at SUBSCRIBE_SEND\\[8\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_8(&mut self) -> Send8W<SendSpec> {
        Send8W::new(self, 8)
    }
    #[doc = "Bit 9 - The status for tasks overflow at SUBSCRIBE_SEND\\[9\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_9(&mut self) -> Send9W<SendSpec> {
        Send9W::new(self, 9)
    }
    #[doc = "Bit 10 - The status for tasks overflow at SUBSCRIBE_SEND\\[10\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_10(&mut self) -> Send10W<SendSpec> {
        Send10W::new(self, 10)
    }
    #[doc = "Bit 11 - The status for tasks overflow at SUBSCRIBE_SEND\\[11\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_11(&mut self) -> Send11W<SendSpec> {
        Send11W::new(self, 11)
    }
    #[doc = "Bit 12 - The status for tasks overflow at SUBSCRIBE_SEND\\[12\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_12(&mut self) -> Send12W<SendSpec> {
        Send12W::new(self, 12)
    }
    #[doc = "Bit 13 - The status for tasks overflow at SUBSCRIBE_SEND\\[13\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_13(&mut self) -> Send13W<SendSpec> {
        Send13W::new(self, 13)
    }
    #[doc = "Bit 14 - The status for tasks overflow at SUBSCRIBE_SEND\\[14\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_14(&mut self) -> Send14W<SendSpec> {
        Send14W::new(self, 14)
    }
    #[doc = "Bit 15 - The status for tasks overflow at SUBSCRIBE_SEND\\[15\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_15(&mut self) -> Send15W<SendSpec> {
        Send15W::new(self, 15)
    }
    #[doc = "Bit 16 - The status for tasks overflow at SUBSCRIBE_SEND\\[16\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_16(&mut self) -> Send16W<SendSpec> {
        Send16W::new(self, 16)
    }
    #[doc = "Bit 17 - The status for tasks overflow at SUBSCRIBE_SEND\\[17\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_17(&mut self) -> Send17W<SendSpec> {
        Send17W::new(self, 17)
    }
    #[doc = "Bit 18 - The status for tasks overflow at SUBSCRIBE_SEND\\[18\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_18(&mut self) -> Send18W<SendSpec> {
        Send18W::new(self, 18)
    }
    #[doc = "Bit 19 - The status for tasks overflow at SUBSCRIBE_SEND\\[19\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_19(&mut self) -> Send19W<SendSpec> {
        Send19W::new(self, 19)
    }
    #[doc = "Bit 20 - The status for tasks overflow at SUBSCRIBE_SEND\\[20\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_20(&mut self) -> Send20W<SendSpec> {
        Send20W::new(self, 20)
    }
    #[doc = "Bit 21 - The status for tasks overflow at SUBSCRIBE_SEND\\[21\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_21(&mut self) -> Send21W<SendSpec> {
        Send21W::new(self, 21)
    }
    #[doc = "Bit 22 - The status for tasks overflow at SUBSCRIBE_SEND\\[22\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_22(&mut self) -> Send22W<SendSpec> {
        Send22W::new(self, 22)
    }
    #[doc = "Bit 23 - The status for tasks overflow at SUBSCRIBE_SEND\\[23\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_23(&mut self) -> Send23W<SendSpec> {
        Send23W::new(self, 23)
    }
    #[doc = "Bit 24 - The status for tasks overflow at SUBSCRIBE_SEND\\[24\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_24(&mut self) -> Send24W<SendSpec> {
        Send24W::new(self, 24)
    }
    #[doc = "Bit 25 - The status for tasks overflow at SUBSCRIBE_SEND\\[25\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_25(&mut self) -> Send25W<SendSpec> {
        Send25W::new(self, 25)
    }
    #[doc = "Bit 26 - The status for tasks overflow at SUBSCRIBE_SEND\\[26\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_26(&mut self) -> Send26W<SendSpec> {
        Send26W::new(self, 26)
    }
    #[doc = "Bit 27 - The status for tasks overflow at SUBSCRIBE_SEND\\[27\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_27(&mut self) -> Send27W<SendSpec> {
        Send27W::new(self, 27)
    }
    #[doc = "Bit 28 - The status for tasks overflow at SUBSCRIBE_SEND\\[28\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_28(&mut self) -> Send28W<SendSpec> {
        Send28W::new(self, 28)
    }
    #[doc = "Bit 29 - The status for tasks overflow at SUBSCRIBE_SEND\\[29\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_29(&mut self) -> Send29W<SendSpec> {
        Send29W::new(self, 29)
    }
    #[doc = "Bit 30 - The status for tasks overflow at SUBSCRIBE_SEND\\[30\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_30(&mut self) -> Send30W<SendSpec> {
        Send30W::new(self, 30)
    }
    #[doc = "Bit 31 - The status for tasks overflow at SUBSCRIBE_SEND\\[31\\]."]
    #[inline(always)]
    #[must_use]
    pub fn send_31(&mut self) -> Send31W<SendSpec> {
        Send31W::new(self, 31)
    }
}
#[doc = "The task overflow for SEND tasks using SUBSCRIBE_SEND. Write 0 to clear.\n\nYou can [`read`](crate::Reg::read) this register and get [`send::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`send::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SendSpec;
impl crate::RegisterSpec for SendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`send::R`](R) reader structure"]
impl crate::Readable for SendSpec {}
#[doc = "`write(|w| ..)` method takes [`send::W`](W) writer structure"]
impl crate::Writable for SendSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEND to value 0"]
impl crate::Resettable for SendSpec {
    const RESET_VALUE: u32 = 0;
}
