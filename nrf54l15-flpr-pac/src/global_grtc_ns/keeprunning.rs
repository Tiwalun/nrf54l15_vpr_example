#[doc = "Register `KEEPRUNNING` reader"]
pub type R = crate::R<KeeprunningSpec>;
#[doc = "Register `KEEPRUNNING` writer"]
pub type W = crate::W<KeeprunningSpec>;
#[doc = "Request from index \\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Request0 {
    #[doc = "0: Allow SYSCOUNTER to go to sleep"]
    NotActive = 0,
    #[doc = "1: Keep SYSCOUNTER active"]
    Active = 1,
}
impl From<Request0> for bool {
    #[inline(always)]
    fn from(variant: Request0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQUEST_0` reader - Request from index \\[0\\]"]
pub type Request0R = crate::BitReader<Request0>;
impl Request0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Request0 {
        match self.bits {
            false => Request0::NotActive,
            true => Request0::Active,
        }
    }
    #[doc = "Allow SYSCOUNTER to go to sleep"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Request0::NotActive
    }
    #[doc = "Keep SYSCOUNTER active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Request0::Active
    }
}
#[doc = "Field `REQUEST_0` writer - Request from index \\[0\\]"]
pub type Request0W<'a, REG> = crate::BitWriter<'a, REG, Request0>;
impl<'a, REG> Request0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Allow SYSCOUNTER to go to sleep"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(Request0::NotActive)
    }
    #[doc = "Keep SYSCOUNTER active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Request0::Active)
    }
}
#[doc = "Request from index \\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Request1 {
    #[doc = "0: Allow SYSCOUNTER to go to sleep"]
    NotActive = 0,
    #[doc = "1: Keep SYSCOUNTER active"]
    Active = 1,
}
impl From<Request1> for bool {
    #[inline(always)]
    fn from(variant: Request1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQUEST_1` reader - Request from index \\[1\\]"]
pub type Request1R = crate::BitReader<Request1>;
impl Request1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Request1 {
        match self.bits {
            false => Request1::NotActive,
            true => Request1::Active,
        }
    }
    #[doc = "Allow SYSCOUNTER to go to sleep"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Request1::NotActive
    }
    #[doc = "Keep SYSCOUNTER active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Request1::Active
    }
}
#[doc = "Field `REQUEST_1` writer - Request from index \\[1\\]"]
pub type Request1W<'a, REG> = crate::BitWriter<'a, REG, Request1>;
impl<'a, REG> Request1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Allow SYSCOUNTER to go to sleep"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(Request1::NotActive)
    }
    #[doc = "Keep SYSCOUNTER active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Request1::Active)
    }
}
#[doc = "Request from index \\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Request2 {
    #[doc = "0: Allow SYSCOUNTER to go to sleep"]
    NotActive = 0,
    #[doc = "1: Keep SYSCOUNTER active"]
    Active = 1,
}
impl From<Request2> for bool {
    #[inline(always)]
    fn from(variant: Request2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQUEST_2` reader - Request from index \\[2\\]"]
pub type Request2R = crate::BitReader<Request2>;
impl Request2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Request2 {
        match self.bits {
            false => Request2::NotActive,
            true => Request2::Active,
        }
    }
    #[doc = "Allow SYSCOUNTER to go to sleep"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Request2::NotActive
    }
    #[doc = "Keep SYSCOUNTER active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Request2::Active
    }
}
#[doc = "Field `REQUEST_2` writer - Request from index \\[2\\]"]
pub type Request2W<'a, REG> = crate::BitWriter<'a, REG, Request2>;
impl<'a, REG> Request2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Allow SYSCOUNTER to go to sleep"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(Request2::NotActive)
    }
    #[doc = "Keep SYSCOUNTER active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Request2::Active)
    }
}
#[doc = "Request from index \\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Request3 {
    #[doc = "0: Allow SYSCOUNTER to go to sleep"]
    NotActive = 0,
    #[doc = "1: Keep SYSCOUNTER active"]
    Active = 1,
}
impl From<Request3> for bool {
    #[inline(always)]
    fn from(variant: Request3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQUEST_3` reader - Request from index \\[3\\]"]
pub type Request3R = crate::BitReader<Request3>;
impl Request3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Request3 {
        match self.bits {
            false => Request3::NotActive,
            true => Request3::Active,
        }
    }
    #[doc = "Allow SYSCOUNTER to go to sleep"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Request3::NotActive
    }
    #[doc = "Keep SYSCOUNTER active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Request3::Active
    }
}
#[doc = "Field `REQUEST_3` writer - Request from index \\[3\\]"]
pub type Request3W<'a, REG> = crate::BitWriter<'a, REG, Request3>;
impl<'a, REG> Request3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Allow SYSCOUNTER to go to sleep"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(Request3::NotActive)
    }
    #[doc = "Keep SYSCOUNTER active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Request3::Active)
    }
}
impl R {
    #[doc = "Bit 0 - Request from index \\[0\\]"]
    #[inline(always)]
    pub fn request_0(&self) -> Request0R {
        Request0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Request from index \\[1\\]"]
    #[inline(always)]
    pub fn request_1(&self) -> Request1R {
        Request1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Request from index \\[2\\]"]
    #[inline(always)]
    pub fn request_2(&self) -> Request2R {
        Request2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Request from index \\[3\\]"]
    #[inline(always)]
    pub fn request_3(&self) -> Request3R {
        Request3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Request from index \\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn request_0(&mut self) -> Request0W<KeeprunningSpec> {
        Request0W::new(self, 0)
    }
    #[doc = "Bit 1 - Request from index \\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn request_1(&mut self) -> Request1W<KeeprunningSpec> {
        Request1W::new(self, 1)
    }
    #[doc = "Bit 2 - Request from index \\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn request_2(&mut self) -> Request2W<KeeprunningSpec> {
        Request2W::new(self, 2)
    }
    #[doc = "Bit 3 - Request from index \\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn request_3(&mut self) -> Request3W<KeeprunningSpec> {
        Request3W::new(self, 3)
    }
}
#[doc = "Request to keep the SYSCOUNTER in the active state and prevent going to sleep\n\nYou can [`read`](crate::Reg::read) this register and get [`keeprunning::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keeprunning::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeeprunningSpec;
impl crate::RegisterSpec for KeeprunningSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keeprunning::R`](R) reader structure"]
impl crate::Readable for KeeprunningSpec {}
#[doc = "`write(|w| ..)` method takes [`keeprunning::W`](W) writer structure"]
impl crate::Writable for KeeprunningSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEEPRUNNING to value 0"]
impl crate::Resettable for KeeprunningSpec {
    const RESET_VALUE: u32 = 0;
}
