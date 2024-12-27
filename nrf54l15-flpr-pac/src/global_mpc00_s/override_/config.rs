#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `SLAVENUMBER` reader - Target slave number for override region n accesses. Slave number 0 is reserved for default slave"]
pub type SlavenumberR = crate::FieldReader;
#[doc = "Field `SLAVENUMBER` writer - Target slave number for override region n accesses. Slave number 0 is reserved for default slave"]
pub type SlavenumberW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Lock Override region n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: Override region n settings can be updated"]
    Unlocked = 0,
    #[doc = "1: Override region n settings can't be updated until next reset"]
    Locked = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Lock Override region n"]
pub type LockR = crate::BitReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            false => Lock::Unlocked,
            true => Lock::Locked,
        }
    }
    #[doc = "Override region n settings can be updated"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lock::Unlocked
    }
    #[doc = "Override region n settings can't be updated until next reset"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lock::Locked
    }
}
#[doc = "Field `LOCK` writer - Lock Override region n"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG, Lock>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override region n settings can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Unlocked)
    }
    #[doc = "Override region n settings can't be updated until next reset"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Locked)
    }
}
#[doc = "Enable Override region n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Override region n is not used"]
    Disabled = 0,
    #[doc = "1: Override region n is used"]
    Enabled = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable Override region n"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disabled,
            true => Enable::Enabled,
        }
    }
    #[doc = "Override region n is not used"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable::Disabled
    }
    #[doc = "Override region n is used"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable::Enabled
    }
}
#[doc = "Field `ENABLE` writer - Enable Override region n"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override region n is not used"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disabled)
    }
    #[doc = "Override region n is used"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enabled)
    }
}
#[doc = "Secure mask enable for Override region n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Securemask {
    #[doc = "0: Mask is disabled for override region n"]
    Disabled = 0,
    #[doc = "1: Mask is enabled for override region n"]
    Enabled = 1,
}
impl From<Securemask> for bool {
    #[inline(always)]
    fn from(variant: Securemask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECUREMASK` reader - Secure mask enable for Override region n"]
pub type SecuremaskR = crate::BitReader<Securemask>;
impl SecuremaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Securemask {
        match self.bits {
            false => Securemask::Disabled,
            true => Securemask::Enabled,
        }
    }
    #[doc = "Mask is disabled for override region n"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Securemask::Disabled
    }
    #[doc = "Mask is enabled for override region n"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Securemask::Enabled
    }
}
impl R {
    #[doc = "Bits 0:4 - Target slave number for override region n accesses. Slave number 0 is reserved for default slave"]
    #[inline(always)]
    pub fn slavenumber(&self) -> SlavenumberR {
        SlavenumberR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Lock Override region n"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Override region n"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Secure mask enable for Override region n"]
    #[inline(always)]
    pub fn securemask(&self) -> SecuremaskR {
        SecuremaskR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Target slave number for override region n accesses. Slave number 0 is reserved for default slave"]
    #[inline(always)]
    pub fn slavenumber(&mut self) -> SlavenumberW<ConfigSpec> {
        SlavenumberW::new(self, 0)
    }
    #[doc = "Bit 8 - Lock Override region n"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<ConfigSpec> {
        LockW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Override region n"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<ConfigSpec> {
        EnableW::new(self, 9)
    }
}
#[doc = "Description cluster: Override region n Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0;
}
