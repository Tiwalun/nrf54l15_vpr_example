#[doc = "Register `INTPEND1` reader"]
pub type R = crate::R<Intpend1Spec>;
#[doc = "Read pending status of interrupt for event COMPARE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare0 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Compare0> for bool {
    #[inline(always)]
    fn from(variant: Compare0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE0` reader - Read pending status of interrupt for event COMPARE\\[0\\]"]
pub type Compare0R = crate::BitReader<Compare0>;
impl Compare0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare0 {
        match self.bits {
            false => Compare0::NotPending,
            true => Compare0::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Compare0::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Compare0::Pending
    }
}
#[doc = "Read pending status of interrupt for event COMPARE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare1 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Compare1> for bool {
    #[inline(always)]
    fn from(variant: Compare1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE1` reader - Read pending status of interrupt for event COMPARE\\[1\\]"]
pub type Compare1R = crate::BitReader<Compare1>;
impl Compare1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare1 {
        match self.bits {
            false => Compare1::NotPending,
            true => Compare1::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Compare1::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Compare1::Pending
    }
}
#[doc = "Read pending status of interrupt for event COMPARE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare2 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Compare2> for bool {
    #[inline(always)]
    fn from(variant: Compare2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE2` reader - Read pending status of interrupt for event COMPARE\\[2\\]"]
pub type Compare2R = crate::BitReader<Compare2>;
impl Compare2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare2 {
        match self.bits {
            false => Compare2::NotPending,
            true => Compare2::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Compare2::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Compare2::Pending
    }
}
#[doc = "Read pending status of interrupt for event COMPARE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare3 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Compare3> for bool {
    #[inline(always)]
    fn from(variant: Compare3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE3` reader - Read pending status of interrupt for event COMPARE\\[3\\]"]
pub type Compare3R = crate::BitReader<Compare3>;
impl Compare3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare3 {
        match self.bits {
            false => Compare3::NotPending,
            true => Compare3::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Compare3::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Compare3::Pending
    }
}
#[doc = "Read pending status of interrupt for event COMPARE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare4 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Compare4> for bool {
    #[inline(always)]
    fn from(variant: Compare4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE4` reader - Read pending status of interrupt for event COMPARE\\[4\\]"]
pub type Compare4R = crate::BitReader<Compare4>;
impl Compare4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare4 {
        match self.bits {
            false => Compare4::NotPending,
            true => Compare4::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Compare4::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Compare4::Pending
    }
}
#[doc = "Read pending status of interrupt for event COMPARE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare5 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Compare5> for bool {
    #[inline(always)]
    fn from(variant: Compare5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE5` reader - Read pending status of interrupt for event COMPARE\\[5\\]"]
pub type Compare5R = crate::BitReader<Compare5>;
impl Compare5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare5 {
        match self.bits {
            false => Compare5::NotPending,
            true => Compare5::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Compare5::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Compare5::Pending
    }
}
#[doc = "Read pending status of interrupt for event COMPARE\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare6 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Compare6> for bool {
    #[inline(always)]
    fn from(variant: Compare6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE6` reader - Read pending status of interrupt for event COMPARE\\[6\\]"]
pub type Compare6R = crate::BitReader<Compare6>;
impl Compare6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare6 {
        match self.bits {
            false => Compare6::NotPending,
            true => Compare6::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Compare6::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Compare6::Pending
    }
}
#[doc = "Read pending status of interrupt for event COMPARE\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare7 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Compare7> for bool {
    #[inline(always)]
    fn from(variant: Compare7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE7` reader - Read pending status of interrupt for event COMPARE\\[7\\]"]
pub type Compare7R = crate::BitReader<Compare7>;
impl Compare7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare7 {
        match self.bits {
            false => Compare7::NotPending,
            true => Compare7::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Compare7::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Compare7::Pending
    }
}
#[doc = "Read pending status of interrupt for event COMPARE\\[8\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare8 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Compare8> for bool {
    #[inline(always)]
    fn from(variant: Compare8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE8` reader - Read pending status of interrupt for event COMPARE\\[8\\]"]
pub type Compare8R = crate::BitReader<Compare8>;
impl Compare8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare8 {
        match self.bits {
            false => Compare8::NotPending,
            true => Compare8::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Compare8::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Compare8::Pending
    }
}
#[doc = "Read pending status of interrupt for event COMPARE\\[9\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare9 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Compare9> for bool {
    #[inline(always)]
    fn from(variant: Compare9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE9` reader - Read pending status of interrupt for event COMPARE\\[9\\]"]
pub type Compare9R = crate::BitReader<Compare9>;
impl Compare9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare9 {
        match self.bits {
            false => Compare9::NotPending,
            true => Compare9::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Compare9::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Compare9::Pending
    }
}
#[doc = "Read pending status of interrupt for event COMPARE\\[10\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare10 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Compare10> for bool {
    #[inline(always)]
    fn from(variant: Compare10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE10` reader - Read pending status of interrupt for event COMPARE\\[10\\]"]
pub type Compare10R = crate::BitReader<Compare10>;
impl Compare10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare10 {
        match self.bits {
            false => Compare10::NotPending,
            true => Compare10::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Compare10::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Compare10::Pending
    }
}
#[doc = "Read pending status of interrupt for event COMPARE\\[11\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare11 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Compare11> for bool {
    #[inline(always)]
    fn from(variant: Compare11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE11` reader - Read pending status of interrupt for event COMPARE\\[11\\]"]
pub type Compare11R = crate::BitReader<Compare11>;
impl Compare11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare11 {
        match self.bits {
            false => Compare11::NotPending,
            true => Compare11::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Compare11::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Compare11::Pending
    }
}
#[doc = "Read pending status of interrupt for event RTCOMPARESYNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcomparesync {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Rtcomparesync> for bool {
    #[inline(always)]
    fn from(variant: Rtcomparesync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCOMPARESYNC` reader - Read pending status of interrupt for event RTCOMPARESYNC"]
pub type RtcomparesyncR = crate::BitReader<Rtcomparesync>;
impl RtcomparesyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcomparesync {
        match self.bits {
            false => Rtcomparesync::NotPending,
            true => Rtcomparesync::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Rtcomparesync::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Rtcomparesync::Pending
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
impl R {
    #[doc = "Bit 0 - Read pending status of interrupt for event COMPARE\\[0\\]"]
    #[inline(always)]
    pub fn compare0(&self) -> Compare0R {
        Compare0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event COMPARE\\[1\\]"]
    #[inline(always)]
    pub fn compare1(&self) -> Compare1R {
        Compare1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read pending status of interrupt for event COMPARE\\[2\\]"]
    #[inline(always)]
    pub fn compare2(&self) -> Compare2R {
        Compare2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read pending status of interrupt for event COMPARE\\[3\\]"]
    #[inline(always)]
    pub fn compare3(&self) -> Compare3R {
        Compare3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read pending status of interrupt for event COMPARE\\[4\\]"]
    #[inline(always)]
    pub fn compare4(&self) -> Compare4R {
        Compare4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read pending status of interrupt for event COMPARE\\[5\\]"]
    #[inline(always)]
    pub fn compare5(&self) -> Compare5R {
        Compare5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read pending status of interrupt for event COMPARE\\[6\\]"]
    #[inline(always)]
    pub fn compare6(&self) -> Compare6R {
        Compare6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Read pending status of interrupt for event COMPARE\\[7\\]"]
    #[inline(always)]
    pub fn compare7(&self) -> Compare7R {
        Compare7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Read pending status of interrupt for event COMPARE\\[8\\]"]
    #[inline(always)]
    pub fn compare8(&self) -> Compare8R {
        Compare8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read pending status of interrupt for event COMPARE\\[9\\]"]
    #[inline(always)]
    pub fn compare9(&self) -> Compare9R {
        Compare9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Read pending status of interrupt for event COMPARE\\[10\\]"]
    #[inline(always)]
    pub fn compare10(&self) -> Compare10R {
        Compare10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Read pending status of interrupt for event COMPARE\\[11\\]"]
    #[inline(always)]
    pub fn compare11(&self) -> Compare11R {
        Compare11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 25 - Read pending status of interrupt for event RTCOMPARESYNC"]
    #[inline(always)]
    pub fn rtcomparesync(&self) -> RtcomparesyncR {
        RtcomparesyncR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Read pending status of interrupt for event PWMPERIODEND"]
    #[inline(always)]
    pub fn pwmperiodend(&self) -> PwmperiodendR {
        PwmperiodendR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Pending interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`intpend1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intpend1Spec;
impl crate::RegisterSpec for Intpend1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intpend1::R`](R) reader structure"]
impl crate::Readable for Intpend1Spec {}
#[doc = "`reset()` method sets INTPEND1 to value 0"]
impl crate::Resettable for Intpend1Spec {
    const RESET_VALUE: u32 = 0;
}
